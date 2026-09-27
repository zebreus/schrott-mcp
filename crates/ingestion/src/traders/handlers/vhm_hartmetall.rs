//! VHM Hartmetall Ankauf (Remscheid): four fixed €/kg grades in photo price
//! cards (`article.price-card-new` between the "Aktuelle Hartmetall
//! Ankaufspreise" heading and the "Preisfaktoren" explainer), plus a
//! Hartmetallschlamm card without a fixed price
//! ("Nach Analyse" → loud skip). The hero ticker above repeats the same
//! four prices twice (aria-hidden copy) and the FAQ below repeats them in
//! prose — both stay outside the window on purpose. The page date is only
//! relative ("Aktualisiert: heute"), so `published_at` is None.
//!
//! Mapping: the four fixed-price grades map to the `hartmetall` catalog
//! material ("Hartmetall / VHM / Widia", nichteisen, EUR/kg — page quotes
//! per kg, no conversion), each with its own `variant` so the three
//! 65 €/kg grades never collapse. NOTE: `hartmetall` exists in the working
//! tree (a concurrent catalog extension); if it is ever absent, record()
//! loud-skips these rows as unknown materials — never a wrong material.
//! Cramming tungsten carbide into Kupfer/Messing would corrupt those
//! histories, hence a dedicated material or nothing.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-remscheid-vhm-hartmetall-ankauf";
/// Bespoke, live-verified impressum URL (the site's own `/impressum` link).
/// A move fails the step loudly — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.vhm-hartmetall.de/impressum";

pub const URL: &str = "https://www.vhm-hartmetall.de/aktueller-hartmetall-preis";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit,
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label,
            }),
            None => skipped_labels.push(label),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        prices,
        acceptances: vec![],
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

/// Explicit label → (material, variant) mapping. All four fixed-price
/// grades land on `hartmetall` with the trader's own grade wording as
/// variant (the three 65 €/kg grades must not collapse). Anything
/// unlisted — including future "Nach Analyse" grades with real prices —
/// is skipped loudly, never guessed.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("fräser") || l.contains("fraeser") || l.contains("bohrer") {
        Some(("hartmetall", "VHM-Fräser & Bohrer"))
    } else if l.contains("wende") || l.contains("wsp") {
        Some(("hartmetall", "Wendeschneidplatten"))
    } else if l.contains("widia") {
        Some(("hartmetall", "Widia"))
    } else if l.contains("gemischt") {
        Some(("hartmetall", "gemischt"))
    } else {
        None
    }
}

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    // Window: the card grid between its heading and the "Preisfaktoren"
    // explainer. The ticker (same prices twice) and the FAQ (prices in
    // prose) stay out — article selection alone would already ignore the
    // ticker spans, but the FAQ uses bare <article> tags too, so the end
    // anchor is load-bearing.
    let start = html
        .find("Aktuelle Hartmetall Ankaufspreise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preiskarten fehlen".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("Preisfaktoren").unwrap_or(tail.len());
    let window = &tail[..end];
    // Only content elements: the JSON-LD OfferCatalog mirrors these prices
    // but scripts are never read as text (they glue addresses elsewhere).
    let doc = Html::parse_fragment(window);
    let card = Selector::parse("article.price-card-new").expect("valid selector");
    let h3 = Selector::parse("h3").expect("valid selector");
    let value = Selector::parse("div.price-main-value").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for art in doc.select(&card) {
        let label = art
            .select(&h3)
            .next()
            .map(|h| h.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if label.is_empty() {
            skips.push("(Preiskarte ohne Bezeichnung)".to_owned());
            continue;
        }
        let price_text = art
            .select(&value)
            .next()
            .map(|v| v.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        // "Nach Analyse" (Hartmetallschlamm) is an analysis promise, not a
        // price — the "... auf Anfrage" terminator pattern.
        let Some(price) = parse_eur(&price_text) else {
            skips.push(format!("{label} (kein Festpreis: {price_text})"));
            continue;
        };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let Some(unit) = unit_of(&price_text) else {
            skips.push(format!("{label} (Einheit unverständlich: {price_text})"));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preiskarten leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS page (live: "65,00 €/kg" on every card,
/// page note "alle Preise pro kg"). Only kg is evidenced — anything else
/// skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    if cell.to_lowercase().contains("kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after the
/// "Angaben gemäß § 5 DDG" heading holds firm / name / street / PLZ city /
/// country lines, and the `<p>` after the "Kontakt" heading holds the
/// "Telefon:" / "E-Mail:" lines (numbers inside tel:/mailto: links).
/// Missing headings → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let headings: Vec<ElementRef> = doc.select(&h2).collect();
    let block_after = |title: &str| -> Option<Vec<String>> {
        headings
            .iter()
            .find(|h| h.text().collect::<String>().trim() == title)
            .and_then(|h| {
                h.next_siblings()
                    .filter_map(ElementRef::wrap)
                    .find(|e| e.value().name() == "p")
                    .map(|p| block_lines(&p.inner_html()))
            })
    };
    let Some(addr) = block_after("Angaben gemäß § 5 DDG") else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    let Some(contact) = block_after("Kontakt") else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    // "... Grünenplatzstraße 1a / 42899 Remscheid / Deutschland": the PLZ
    // line carries postcode + city, street is the line right before it.
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in addr.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, w| a + " " + w);
                if k > 0 {
                    street = addr[k - 1].clone();
                }
                break;
            }
        }
    }
    let mut phone = String::new();
    let mut email = String::new();
    for line in &contact {
        if let Some(v) = line.strip_prefix("Telefon:") {
            phone = v
                .split_whitespace()
                .take_while(|t| {
                    t.chars()
                        .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ");
        } else if let Some(v) = line.strip_prefix("E-Mail:") {
            // Email needs its own rule: the phone-style take_while above
            // would stop at the first letter.
            email = v.split_whitespace().next().unwrap_or_default().to_owned();
        }
    }
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo {
        street,
        postcode,
        city,
        phone,
        email,
    })
}

/// Split an inner-HTML block on `<br` into plain-text lines. Newlines are
/// planted BEFORE stripping tags so the "Telefon:"/"E-Mail:" labels survive
/// (a drop-to-first-'>' strip would eat them together with the `<a ...>`
/// opener), and `<br/>` tag remnants (`/>`) are trimmed per line.
fn block_lines(inner: &str) -> Vec<String> {
    inner
        .replace("<br", "\n")
        .split('\n')
        .map(|part| {
            let part = part.trim_start_matches("/>").trim_start_matches('>').trim();
            let mut out = String::new();
            let mut in_tag = false;
            for c in part.chars() {
                if c == '<' {
                    in_tag = true;
                } else if c == '>' {
                    in_tag = false;
                } else if !in_tag {
                    out.push(c);
                }
            }
            out.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    // Verbatim structure of the live cards (2026-09-27): real class names,
    // real data-price attrs, real `&amp;` entity, real "Nach Analyse" card.
    const FIXTURE: &str = "<div class=\"kicker\">Preise nach Material</div>\
        <h2>Aktuelle Hartmetall Ankaufspreise</h2>\
        <div class=\"price-card-grid category-photo-price-grid\">\
        <article class=\"price-card-new price-card-with-photo category-photo-card\">\
        <div class=\"price-card-content\">\
        <div class=\"price-card-topline\">Beispiel: VHM-Werkzeuge</div>\
        <h3>VHM-Fr&auml;ser &amp; VHM-Bohrer</h3>\
        <div class=\"price-main-value\" data-price=\"vhmFraeserBohrer\">65,00 &euro;/kg</div>\
        </div></article>\
        <article class=\"price-card-new price-card-with-photo category-photo-card\">\
        <div class=\"price-card-content\">\
        <div class=\"price-card-topline\">Beispiel: WSP / Inserts</div>\
        <h3>Wendeschneidplatten</h3>\
        <div class=\"price-main-value\" data-price=\"wendeschneidplatten\">65,00 &euro;/kg</div>\
        </div></article>\
        <article class=\"price-card-new price-card-with-photo category-photo-card\">\
        <div class=\"price-card-content\">\
        <div class=\"price-card-topline\">Beispiel: Widia / St&uuml;cke</div>\
        <h3>Widia</h3>\
        <div class=\"price-main-value\" data-price=\"widia\">65,00 &euro;/kg</div>\
        </div></article>\
        <article class=\"price-card-new price-card-with-photo category-photo-card\">\
        <div class=\"price-card-content\">\
        <div class=\"price-card-topline\">Beispiel: gemischtes Material</div>\
        <h3>Hartmetall gemischt</h3>\
        <div class=\"price-main-value\" data-price=\"hartmetallGemischt\">63,00 &euro;/kg</div>\
        </div></article>\
        <article class=\"price-card-new price-card-with-photo category-photo-card\">\
        <div class=\"price-card-content\">\
        <div class=\"price-card-topline\">Beispiel: Schlamm / R&uuml;ckst&auml;nde</div>\
        <h3>Hartmetallschlamm</h3>\
        <div class=\"price-main-value price-analysis\" data-price=\"hartmetallschlamm\">Nach Analyse</div>\
        </div></article>\
        </div><div class=\"kicker\">Preisfaktoren</div>";

    #[test]
    fn cards_parse_and_schlamm_skips() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 4);
        assert_eq!(
            rows[0],
            ("VHM-Fräser & VHM-Bohrer".to_owned(), 65.0, "EUR/kg")
        );
        assert_eq!(rows[1], ("Wendeschneidplatten".to_owned(), 65.0, "EUR/kg"));
        assert_eq!(rows[2], ("Widia".to_owned(), 65.0, "EUR/kg"));
        assert_eq!(rows[3], ("Hartmetall gemischt".to_owned(), 63.0, "EUR/kg"));
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Hartmetallschlamm"), "got {skips:?}");
        assert!(skips[0].contains("Nach Analyse"), "got {skips:?}");
    }

    #[test]
    fn windows_and_units_reject_loudly() {
        // Ticker duplicates (spans, no article cards) must not leak in, and
        // the FAQ prose after the end anchor must not either.
        let html = "<div aria-label=\"Aktuelle Hartmetall Ankaufspreise\">\
            <span>Wendeschneidplatten: <strong>65,00 €/kg</strong></span></div>"
            .to_owned()
            + FIXTURE
            + "<article><h3>Was kostet 1 kg?</h3><p>65,00 €/kg</p></article>";
        let (rows, _) = parse(&html).expect("window holds");
        assert_eq!(rows.len(), 4);
        // Unknown unit: loud skip, valid rows survive.
        let html = FIXTURE.replacen("&euro;/kg", "pro Sack", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!(skips.len(), 2);
        assert!(skips.iter().any(|s| s.contains("Einheit unverständlich")));
        // Every card unparseable: loud error, not silent success.
        let html = FIXTURE.replace("price-main-value", "price-other");
        let err = parse(&html).expect_err("empty cards error");
        assert!(err.to_string().contains("leer"));
        // Missing heading: loud error.
        assert!(parse("<p>Neu hier</p>").is_err());
        assert_eq!(unit_of("65,00 €/kg"), Some("EUR/kg"));
        assert_eq!(unit_of("pro Sack"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h2>Angaben gemäß § 5 DDG</h2>\
            <p><strong>VHM Hartmetall Ankauf</strong><br/>Yehya Jadouh<br/>\
            Grünenplatzstraße 1a<br/>42899 Remscheid<br/>Deutschland</p>\
            <h2>Kontakt</h2><p>Telefon: <a href=\"tel:+4917670524959\">017670524959</a><br/>\
            E-Mail: <a href=\"mailto:info@vhm-hartmetall.de\">info@vhm-hartmetall.de</a></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Grünenplatzstraße 1a");
        assert_eq!(info.postcode, "42899");
        assert_eq!(info.city, "Remscheid");
        assert_eq!(info.phone, "017670524959");
        assert_eq!(info.email, "info@vhm-hartmetall.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<h2>Neu</h2><p>x</p>").is_err());
    }

    #[test]
    fn mapping_resolves_grades_and_skips_schlamm() {
        // One material, four variants: the three 65 €/kg grades must not
        // collapse onto a single current price.
        assert_eq!(
            grade_for("VHM-Fräser & VHM-Bohrer"),
            Some(("hartmetall", "VHM-Fräser & Bohrer"))
        );
        assert_eq!(
            grade_for("Wendeschneidplatten"),
            Some(("hartmetall", "Wendeschneidplatten"))
        );
        assert_eq!(grade_for("Widia"), Some(("hartmetall", "Widia")));
        assert_eq!(
            grade_for("Hartmetall gemischt"),
            Some(("hartmetall", "gemischt"))
        );
        assert_eq!(grade_for("Hartmetallschlamm"), None);
        assert_eq!(grade_for("Ankaufspreise gelten bei passender Sorte"), None);
    }
}
