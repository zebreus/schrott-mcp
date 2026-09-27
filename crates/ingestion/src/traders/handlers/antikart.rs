//! Antik&ART (Berlin An- und Verkauf, Reinickendorf): exact
//! Privatkunden prices in the nine Elementor price-table cards under
//! "Preise Privatkunden" ("Kupfer", "Messing", "Aluminium", "Blei",
//! "Kupferkabel", "Kfz - Batterien", "Zink", "Elektromotoren",
//! "Edelstahl"). Each card's price span holds "Sauber … / Unsauber …"
//! lines; the "ab 100 kg Preis auf Anfrage" line lives in the features
//! list and is never a price (segments without '€' are prose, not
//! prices). Kfz-Batterien have no catalog material and are skipped
//! loudly. The same prices exist as a linked PDF, but the HTML cards are
//! parsed (preferred). No page date, so `published_at` stays `None`.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "be-reinickendorf-antik-art-berlin-an-und-verkauf";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.berlin-an-und-verkauf.de/impressum/";

pub const URL: &str =
    "https://www.berlin-an-und-verkauf.de/buntmetall-zu-guten-preisen-verkaufen-recycling-in-berlin/";

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

/// Explicit label → (material, variant) mapping, "Kupfer Sauber" style.
/// Anything unlisted is skipped. Kabel before Kupfer: "Kupferkabel"
/// would otherwise fall into the generic copper arm. Generic page words
/// map to generic materials only ("Edelstahl" → edelstahl-gemischt, never
/// V2A — the page never names a grade).
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    let cond: &'static str = if l.ends_with("unsauber") {
        "Unsauber"
    } else if l.ends_with("sauber") {
        "Sauber"
    } else {
        ""
    };
    if l.contains("batterie") {
        None
    } else if l.contains("kabel") {
        Some(("kabel-kupfer", cond))
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", cond))
    } else if l.contains("messing") {
        Some(("messing", cond))
    } else if l.contains("aluminium") {
        Some(("aluminium-gemischt", cond))
    } else if l.contains("edelstahl") {
        Some(("edelstahl-gemischt", cond))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("elektromotor") {
        Some(("elektromotoren", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after
/// the "Angaben gemäß § 5 TMG" heading holds firm + street + PLZ city
/// (street glued to its number live: "Holländerstr.116"), and the `<p>`
/// after the "Kontakt" heading carries the Telefon/E-Mail lines.
/// Missing anchors → loud error, never a fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let h3 = Selector::parse("h3").expect("valid selector");
    let anchor = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Angaben gemäß § 5 TMG");
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    let addr_p = anchor
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let mut lines = Vec::new();
    if let Some(p) = addr_p {
        for part in p.inner_html().split("<br") {
            let t = strip_fragment(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if lines.len() >= 2 {
        let last = lines.last().expect("len checked");
        let mut it = last.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| a + " " + b);
                street = unglue_street(&lines[lines.len() - 2]);
            }
        }
    }
    let kontakt = doc
        .select(&h3)
        .find(|h| h.text().collect::<String>().trim() == "Kontakt");
    let Some(kontakt) = kontakt else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let contact_p = kontakt
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let mut phone = String::new();
    let mut email = String::new();
    if let Some(p) = contact_p {
        for part in p.inner_html().split("<br") {
            let t = strip_fragment(part);
            if phone.is_empty() {
                if let Some(rest) = t.strip_prefix("Telefon:") {
                    phone = rest.trim().to_owned();
                }
            }
            if email.is_empty() {
                if let Some(rest) = t.strip_prefix("E-Mail:") {
                    email = rest
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
                        .to_owned();
                }
            }
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

/// The page glues street and number ("Holländerstr.116"): split at the
/// first digit so the street parses as "Holländerstr. 116".
fn unglue_street(s: &str) -> String {
    match s.find(|c: char| c.is_ascii_digit()) {
        Some(i) if i > 0 && !s[..i].ends_with(' ') => {
            format!("{} {}", s[..i].trim_end(), s[i..].trim_start())
        }
        _ => s.to_owned(),
    }
}

/// Strip tags from a `<br`-split fragment (drop everything up to the
/// first '>' first, or tag attributes parse as text).
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

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    // Window: the Privatkunden cards only, from their heading to the
    // page-title block that follows the last card.
    let start = html
        .find("Preise Privatkunden")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbereich fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Buntmetall zu guten Preisen verkaufen")
        .unwrap_or(tail.len());
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    let card = Selector::parse("div.elementor-price-table").expect("valid selector");
    let heading = Selector::parse("h3.elementor-price-table__heading").expect("valid selector");
    let price =
        Selector::parse("span.elementor-price-table__integer-part").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips: Vec<String> = Vec::new();
    let mut cards = 0;
    for c in doc.select(&card) {
        let Some(h) = c.select(&heading).next() else {
            continue;
        };
        let head: String = h
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let Some(p) = c.select(&price).next() else {
            continue;
        };
        cards += 1;
        for part in p.inner_html().split("<br") {
            let seg = strip_fragment(part);
            if seg.is_empty() {
                continue;
            }
            // Prose guard: only segments carrying '€' are prices (the
            // "ab 100 kg Preis auf Anfrage" line has none — and lives in
            // the features list anyway).
            if !seg.contains('€') {
                continue;
            }
            let Some(value) = parse_eur(&seg) else {
                skips.push(format!("{head} {seg} (Preis unverständlich)"));
                continue;
            };
            // An unparseable unit is a loud skip, never a silent default.
            let Some(unit) = unit_of(&seg) else {
                skips.push(format!("{head} {seg} (Einheit unverständlich)"));
                continue;
            };
            rows.push((condition_label(&head, &seg), value, unit));
        }
    }
    if cards == 0 {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preiskarten".to_owned(),
        });
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preiskarten leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// "Sauber 5,91 €/kg" → "Kupfer Sauber"; "0,77 €/kg" → "Blei": the
/// condition is the leading word only when it holds no digits.
fn condition_label(head: &str, seg: &str) -> String {
    let first = seg.split_whitespace().next().unwrap_or_default();
    if !first.is_empty() && !first.chars().any(|c| c.is_ascii_digit()) {
        format!("{head} {first}")
    } else {
        head.to_owned()
    }
}

/// Bespoke unit matcher for THIS page's price lines (live: "€/kg" —
/// once spaced as "€ /kg"). Only kg/t exist here — anything else skips
/// loudly at the call site.
fn unit_of(seg: &str) -> Option<&'static str> {
    let lower = seg.to_lowercase().replace(' ', "");
    if lower.contains("/kg") {
        Some("EUR/kg")
    } else if lower.contains("/t") || lower.contains("/to") {
        Some("EUR/t")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{condition_label, extract_info, grade_for, parse, unglue_street};

    const FIXTURE: &str = "<h2>Preise Privatkunden</h2>\
        <div class=\"elementor-price-table\"><div class=\"elementor-price-table__header\">\
        <h3 class=\"elementor-price-table__heading\">Kupfer</h3></div>\
        <div class=\"elementor-price-table__price\">\
        <span class=\"elementor-price-table__integer-part\">Sauber 5,91 €/kg<br><br>Unsauber 4,57 €/kg</span>\
        </div></div>\
        <div class=\"elementor-price-table\"><div class=\"elementor-price-table__header\">\
        <h3 class=\"elementor-price-table__heading\">Aluminium</h3></div>\
        <div class=\"elementor-price-table__price\">\
        <span class=\"elementor-price-table__integer-part\">Sauber 0,99 €/kg<br><br>Unsauber 0,77 € /kg</span>\
        </div></div>\
        <div class=\"elementor-price-table\"><div class=\"elementor-price-table__header\">\
        <h3 class=\"elementor-price-table__heading\">Kfz - Batterien</h3></div>\
        <div class=\"elementor-price-table__price\">\
        <span class=\"elementor-price-table__integer-part\">0,15 €/kg</span>\
        </div></div>\
        <h1>Buntmetall zu guten Preisen verkaufen – Recycling in Berlin</h1>";

    #[test]
    fn cards_parse_with_conditions() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 5);
        assert!(skips.is_empty());
        assert_eq!(rows[0], ("Kupfer Sauber".to_owned(), 5.91, "EUR/kg"));
        assert_eq!(rows[1], ("Kupfer Unsauber".to_owned(), 4.57, "EUR/kg"));
        // Spaced unit variant from the live page.
        assert_eq!(rows[3], ("Aluminium Unsauber".to_owned(), 0.77, "EUR/kg"));
        assert_eq!(rows[4], ("Kfz - Batterien".to_owned(), 0.15, "EUR/kg"));
        assert_eq!(condition_label("Blei", "0,77 €/kg"), "Blei");
    }

    #[test]
    fn missing_anchors_fail_loudly() {
        let no_start = FIXTURE.replace("Preise Privatkunden", "Preise");
        assert!(parse(&no_start).is_err());
        let no_cards = "<h2>Preise Privatkunden</h2><p>neu gestaltet</p>\
            <h1>Buntmetall zu guten Preisen verkaufen</h1>";
        let err = parse(no_cards).expect_err("no cards errors");
        assert!(err.to_string().contains("keine Preiskarten"));
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h2>Angaben gemäß § 5 TMG </h2>\
            <p>Antik&ART (Einzelunternehmen)<br />Holländerstr.116<br />13407 Berlin</p>\
            <h3>Kontakt</h3><p>Telefon: +49 (0) 30 40 39 65 51<br />\
            E-Mail: info@berlin-an-und-verkauf.de</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Holländerstr. 116");
        assert_eq!(info.postcode, "13407");
        assert_eq!(info.city, "Berlin");
        assert_eq!(info.phone, "+49 (0) 30 40 39 65 51");
        assert_eq!(info.email, "info@berlin-an-und-verkauf.de");
        assert_eq!(unglue_street("Wallenroder Str. 7"), "Wallenroder Str. 7");
        assert!(extract_info("<h2>Anderes</h2>").is_err());
    }

    #[test]
    fn mapping_splits_conditions_and_skips_batteries() {
        assert_eq!(
            grade_for("Kupfer Sauber"),
            Some(("kupfer-gemischt", "Sauber"))
        );
        assert_eq!(
            grade_for("Kupferkabel Unsauber"),
            Some(("kabel-kupfer", "Unsauber"))
        );
        assert_eq!(
            grade_for("Edelstahl Sauber"),
            Some(("edelstahl-gemischt", "Sauber"))
        );
        assert_eq!(grade_for("Elektromotoren"), Some(("elektromotoren", "")));
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(grade_for("Kfz - Batterien"), None, "no battery material");
    }
}
