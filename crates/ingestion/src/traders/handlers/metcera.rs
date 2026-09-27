//! Metcera (Großpösna): exact "Vergütung" purchase prices from the
//! homepage sidebar (`aside#block-20`: one `<p>` with `<br>` rows like
//! "Mischschrott 0,02 € / kg", windowed between "Vergütung für" and
//! "Tagespreise"). Quoted unit is honestly EUR/kg throughout (the
//! pipeline's `normalize_unit` folds kg↔t into the catalog unit —
//! `mischschrott` lands on EUR/t there, no handler-side math).
//! "Kupferpreise ab 4,00 € / kg" is a from-price: price = price_min = quoted value,
//! kind `approx` at 0.5 (scheideanstalt precedent), never a silent exact.
//! The sidebar slider ("Annahme von … ab … €") quotes disposal FEES
//! (wrong price direction, no catalog material either) and is skipped
//! loudly with its quoted fee as evidence. Papier and Bleibatterien have
//! no catalog material and are skipped loudly. "Tagespreise" without a
//! date → `published_at` stays `None`.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sn-groposna-metcera";
/// Bespoke, live-verified impressum URL (site footer's own
/// "Impressum / Datenschutzerklärung" link). A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.metcera-recycling.de/impressum/";

/// Price page: the Tagespreise live in the homepage sidebar, so the
/// homepage IS the source URL — never the annahmeliste (no prices there).
pub const URL: &str = "https://www.metcera-recycling.de/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit, ab) in rows {
        match grade_for(&label) {
            Some((material, variant)) => {
                // From-prices ("ab") mirror the upto pattern: the bound is
                // honest data with its own kind, never a silent exact.
                // (kg↔t catalog normalization happens centrally in
                // `record()` — the handler keeps the honest quote.)
                let (price_kind, price_min, confidence) = if ab {
                    ("approx", Some(price), Some(0.5))
                } else {
                    ("exact", None, Some(1.0))
                };
                prices.push(ScrapedPrice {
                    material,
                    variant,
                    price,
                    currency: "EUR",
                    unit,
                    price_kind,
                    price_min,
                    price_max: None,
                    confidence,
                    label,
                });
            }
            // No catalog material (Papier, Bleibatterien): keep the quoted
            // price as evidence in the skip, never drop it silently.
            None => skipped_labels.push(format!(
                "{label} ({}, {unit}, kein Katalogmaterial)",
                fmt_eur(price)
            )),
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

fn fmt_eur(price: f64) -> String {
    format!("{price:.2}").replace('.', ",")
}

/// Explicit label → (material, variant) mapping. Kupferkabel before
/// bare Kupfer (the specific arm must not be shadowed); a bare "Kabel"
/// without a metal word is ambiguous (kupfer vs. alu) and stays `None`.
/// Anything unlisted (Papier, Bleibatterien, …) is skipped.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("kabel") {
        if l.contains("kupfer") {
            Some(("kabel-kupfer", ""))
        } else {
            None
        }
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        // Grade wording kept: a future "Messing schwer" row must not
        // collapse onto this one.
        Some(("messing", "leicht"))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("v2a") || (l.contains("edelstahl") && !l.contains("v4a")) {
        Some(("edelstahl-v2a", ""))
    } else {
        None
    }
}

/// Parse the "Vergütung für" sidebar box plus the "Annahme von" fee
/// slider. Returns (rows, skips) with rows as (label, price, unit, ab).
fn parse(
    html: &str,
) -> Result<(Vec<(String, f64, &'static str, bool)>, Vec<String>), IngestError> {
    let start = html.find("Vergütung für").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Vergütungs-Block fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("Tagespreise").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Vergütungs-Block unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let p_sel = Selector::parse("p").expect("valid selector");
    let box_p = frag.select(&p_sel).find(|el| {
        let t: String = el.text().collect();
        t.contains("Mischschrott") && t.contains('€')
    });
    let Some(box_p) = box_p else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preiszeilen fehlen".to_owned(),
        });
    };
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for part in box_p.inner_html().split("<br") {
        let line = strip_fragment(part);
        if line.is_empty() || line.len() > 120 {
            continue;
        }
        if !line.contains('€') {
            skips.push(format!("{line} (kein Preis in der Preisbox)"));
            continue;
        }
        let (label, ab) = split_label_price(&line);
        if label.is_empty() {
            skips.push(format!("{line} (Bezeichnung unverständlich)"));
            continue;
        }
        // Price from the remainder AFTER the label: labels like "V2A"
        // start with a number that would win as a phantom price.
        let Some(price) = parse_eur(&line[label.len()..]) else {
            skips.push(format!("{line} (Preis unverständlich)"));
            continue;
        };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-piece fee recorded as per-kg would corrupt the series.
        let Some(unit) = unit_of(&line) else {
            skips.push(format!("{line} (Einheit unverständlich)"));
            continue;
        };
        rows.push((label, price, unit, ab));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbox leer".to_owned(),
        });
    }
    // The fee slider is supplementary evidence, not a second price box:
    // absent today it stays silent; present its rows skip loudly.
    skips.extend(parse_fee_slider(html));
    Ok((rows, skips))
}

/// Split "Kupferpreise ab 4,00 € / kg" into ("Kupferpreise", ab=true) by
/// dropping trailing numeric/"ab" tokens; "Messing leicht 1,90 € / kg"
/// into ("Messing leicht", ab=false).
fn split_label_price(line: &str) -> (String, bool) {
    let before_euro = line.split('€').next().unwrap_or("").trim();
    let toks: Vec<&str> = before_euro.split_whitespace().collect();
    let mut k = toks.len();
    while k > 0 && (toks[k - 1] == "ab" || toks[k - 1].chars().any(|c| c.is_ascii_digit())) {
        k -= 1;
    }
    let ab = toks[k..].contains(&"ab");
    (toks[..k].join(" "), ab)
}

/// Bespoke unit matcher for THIS sidebar (live: "0,02 € / kg" rows,
/// "ab 40,00 € / Stück" fees). Only kg/Stk exist here — anything else
/// skips loudly at the call site.
fn unit_of(line: &str) -> Option<&'static str> {
    let lower = line.to_lowercase();
    if lower.contains("€ / kg") || lower.contains("€/kg") {
        Some("EUR/kg")
    } else if lower.contains("stück") || lower.contains("stuck") {
        Some("EUR/Stk")
    } else {
        None
    }
}

/// The "Aktuelle Informationen" slider: `<strong>Annahme von …</strong>`
/// labels with "ab … €" disposal fees. Every row is a loud skip (wrong
/// price direction, no catalog material either) with the quoted fee kept
/// as evidence. A missing slider stays silent — it is marketing chrome,
/// not the price box. Anchored on the slider heading (the
/// "slick-slider-item" class name also matches plugin CSS URLs and other
/// sliders — never anchor on it).
fn parse_fee_slider(html: &str) -> Vec<String> {
    let Some(start) = html.find("Aktuelle Informationen") else {
        return vec![];
    };
    let tail = &html[start..];
    let end = tail.find("</section>").unwrap_or(tail.len());
    let window = &tail[..end];
    let mut skips = Vec::new();
    for chunk in window.split("<strong>").skip(1) {
        let Some((lab, rest)) = chunk.split_once("</strong>") else {
            continue;
        };
        let label = strip_fragment(lab);
        if label.is_empty() {
            continue;
        }
        let fee = parse_eur(rest)
            .map(fmt_eur)
            .unwrap_or_else(|| "unlesbar".to_owned());
        let unit = unit_of(rest).unwrap_or("?");
        skips.push(format!(
            "{label} ({fee} {unit}, Annahmegebühr statt Ankaufspreis: falsche Preisrichtung)"
        ));
    }
    skips
}

/// Bespoke contact extraction for THIS impressum only: `h1.entry-title`
/// "Impressum" (must), the address `<p>` with "Sestewitzer Str. 7" +
/// "04463 Großpösna OT Störmthal", and the contact `<p>` with the
/// mailto link plus a "Tel:" line. Missing anchors → loud error, never
/// a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let a = Selector::parse("a").expect("valid selector");
    if !doc.select(&h1).any(|h| h.text().collect::<String>().trim() == "Impressum") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let addr_p = doc.select(&p).find(|el| {
        let t: String = el.text().collect();
        t.contains("Sestewitzer Str.")
    });
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let addr_lines: Vec<String> = addr_p
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in addr_lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| format!("{a} {b}"));
                if k > 0 {
                    street = addr_lines[k - 1].clone();
                }
                break;
            }
        }
    }
    let cont_p = doc.select(&p).find(|el| {
        el.inner_html().contains("mailto:")
    });
    let mut email = String::new();
    let mut phone = String::new();
    if let Some(el) = cont_p {
        email = el
            .select(&a)
            .filter_map(|l| l.value().attr("href"))
            .find_map(|h| h.strip_prefix("mailto:"))
            .unwrap_or_default()
            .trim()
            .to_owned();
        for part in el.inner_html().split("<br") {
            let t = strip_fragment(part);
            if let Some(v) = t.strip_prefix("Tel:") {
                phone = v.trim().to_owned();
                break;
            }
        }
    }
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo { street, postcode, city, phone, email })
}

/// Strip tags from a `<br>`-split fragment. Fragments start with a tag
/// remnant — drop everything up to the first '>' first, or attributes
/// parse as text.
fn strip_fragment(s: &str) -> String {
    let s = match s.find('>') {
        Some(i) => &s[i + 1..],
        None => s,
    };
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            out.push(c);
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::{grade_for, parse, split_label_price, unit_of};

    // Real shape of the live homepage sidebar: the "Vergütung für" box
    // with all 8 rows plus the fee slider section, trimmed.
    const FIXTURE: &str = "<p><strong>Vergütung für</strong>:</p>\
        <p>Mischschrott 0,02 € / kg<br>Papier 0,01 € / kg<br>\
        Kupferkabel o. Stecker 1,10 € / kg<br>Kupferpreise ab 4,00 € / kg<br>\
        Messing leicht 1,90 € / kg<br>Zink 0,70 € / kg<br>\
        V2A (Edelstahl) 0,30 € / kg<br>Bleibatterien 0,17 € / kg</p>\
        <p>Preise sind Tagespreise. Änderungen und Irrtümer vorbehalten.</p>\
        <h5>Aktuelle Informationen:</h5>\
        <section><div class=\"wp-block-gb-for-slick-slider-slick-slider-item\">\
        <p><strong>Annahme von Kristallinen Photovoltaik/PV-Modulen</strong>\
        <br>ab 20,00 € / Stück (Berechnung nach Größe)</p></div>\
        <div class=\"wp-block-gb-for-slick-slider-slick-slider-item\">\
        <p><strong>Annahme von Elektroschrott und gefährlichem Elektroabfall</strong>\
        <br>ab 0,35 € / kg</p></div></section>";

    #[test]
    fn box_rows_units_and_ab_flag() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 8);
        assert_eq!(rows[0], ("Mischschrott".to_owned(), 0.02, "EUR/kg", false));
        assert_eq!(rows[2], ("Kupferkabel o. Stecker".to_owned(), 1.1, "EUR/kg", false));
        assert_eq!(rows[3], ("Kupferpreise".to_owned(), 4.0, "EUR/kg", true));
        assert_eq!(rows[4], ("Messing leicht".to_owned(), 1.9, "EUR/kg", false));
        // "V2A … 0,30": the price comes from behind the label — the
        // leading "2" must not win as a phantom price.
        assert_eq!(rows[6], ("V2A (Edelstahl)".to_owned(), 0.3, "EUR/kg", false));
        // Slider fees skip loudly, never as prices.
        assert_eq!(skips.len(), 2);
        assert!(skips[0].contains("Photovoltaik"));
        assert!(skips[0].contains("Annahmegebühr"));
        assert!(skips[1].contains("0,35"));
        assert_eq!(split_label_price("Kupferpreise ab 4,00"), ("Kupferpreise".to_owned(), true));
        assert_eq!(
            split_label_price("Messing leicht 1,90"),
            ("Messing leicht".to_owned(), false)
        );
        assert_eq!(unit_of("0,02 € / kg"), Some("EUR/kg"));
        assert_eq!(unit_of("ab 20,00 € / Stück"), Some("EUR/Stk"));
        assert_eq!(unit_of("pro Sack"), None);
        assert!(parse("<div>Redesign ohne Box</div>").is_err());
    }

    #[test]
    fn mapping_resolves_and_skips_catalog_gaps() {
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Kupferkabel o. Stecker"), Some(("kabel-kupfer", "")));
        assert_eq!(grade_for("Kupferpreise"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Messing leicht"), Some(("messing", "leicht")));
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("V2A (Edelstahl)"), Some(("edelstahl-v2a", "")));
        // Bare cable grade is ambiguous (kupfer vs. alu) — no guess.
        assert_eq!(grade_for("Kabel"), None);
        // No paper or battery material in the catalog.
        assert_eq!(grade_for("Papier"), None);
        assert_eq!(grade_for("Bleibatterien"), None);
    }

    #[test]
    fn impressum_blocks() {
        let imp = "<h1 class=\"entry-title\">Impressum</h1>\
            <p>Metcera GmbH<br>Sestewitzer Str. 7<br>04463 Großpösna OT Störmthal</p>\
            <p><a href=\"mailto:info@metcera-recycling.de\">info@metcera-recycling.de</a>\
            <br>Tel: <a href=\"tel:+4934297778066\">034297 – 778066</a></p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Sestewitzer Str. 7");
        assert_eq!(info.postcode, "04463");
        assert_eq!(info.city, "Großpösna OT Störmthal");
        assert_eq!(info.phone, "034297 – 778066");
        assert_eq!(info.email, "info@metcera-recycling.de");
        assert!(super::extract_info("<h1>Neu hier</h1>").is_err());
    }
}
