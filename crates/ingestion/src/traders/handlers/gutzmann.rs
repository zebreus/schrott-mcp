//! SDM Gutzmann (Gotha): exact purchase prices in the TablePress table
//! "Schrott | Preis €/t" under "Aktuelle Preise für Schrott". The page
//! carries two more tables — "Entsorgung" (disposal FEES charged, not
//! paid) and "Baustoffe" (building-material SALES) — which are
//! deliberately outside the parse window: recording them as purchase
//! prices would invert the economics. Kernschrott S3 and Batterien have
//! no catalog material and are skipped loudly. No page-stated price date
//! (the meta modified-time is stale page metadata), so `published_at`
//! stays `None`.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "th-gotha-sdm-gutzmann";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.sdm-gutzmann.de/impressum/";

pub const URL: &str = "https://www.sdm-gutzmann.de/preislisten/";

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

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific-before-generic: "Millberry" must win over "Berry",
/// V4A over V2A, "Alu Kabel" over bare "Kabel".
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("berry") {
        Some(("kupfer-berry", ""))
    } else if l.contains("raff") {
        Some(("kupfer-gemischt", "Raff"))
    } else if l.contains("alu") && l.contains("kabel") {
        Some(("kabel-alu", ""))
    } else if l.contains("kupferkabel") || (l.contains("kupfer") && l.contains("kabel")) {
        Some(("kabel-kupfer", ""))
    } else if l.contains("motore") || l.contains("motoren") {
        Some(("elektromotoren", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("geschirr") {
        Some(("aluminium-blech", "Geschirr"))
    } else if l.contains("blech") {
        Some(("aluminium-blech", ""))
    } else if l.contains("profile") {
        Some(("aluminium-profile", ""))
    } else if l.contains("felgen") {
        Some(("aluminium-guss", ""))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", ""))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("shredder") {
        Some(("stahlschrott-shredder", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("scherenschrott") {
        Some(("stahlschrott-scheren", ""))
    } else if l.contains("guss") {
        Some(("eisenschrott-gussbruch", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after
/// the "Angaben gemäß § 5 TMG:" heading holds firm + street + PLZ city,
/// and the paragraphs after the "Kontakt:" heading carry labeled
/// Telefon/E-Mail lines. Missing anchors → loud error, never a fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h3 = Selector::parse("h3").expect("valid selector");
    let h2 = Selector::parse("h2").expect("valid selector");
    let anchor = doc
        .select(&h3)
        .find(|h| h.text().collect::<String>().trim() == "Angaben gemäß § 5 TMG:");
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    // Address lines: first <p> sibling after the heading.
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
                street = lines[lines.len() - 2].clone();
            }
        }
    }
    // Labeled contact lines after the "Kontakt:" heading.
    let kontakt = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Kontakt:");
    let Some(kontakt) = kontakt else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let mut phone = String::new();
    let mut email = String::new();
    let mut el = kontakt.next_sibling();
    while let Some(node) = el {
        if let Some(e) = ElementRef::wrap(node) {
            let name = e.value().name();
            if name == "h1" || name == "h2" || name == "h3" {
                break;
            }
            if name == "p" {
                // Telefon and Telefax share one <p> live — split on
                // <br> so the prefixes match per line, not per blob.
                for part in e.inner_html().split("<br") {
                    let text = strip_fragment(part);
                    if phone.is_empty() {
                        if let Some(rest) = text.strip_prefix("Telefon:") {
                            phone = rest.trim().to_owned();
                        }
                    }
                    if email.is_empty() {
                        if let Some(rest) = text.strip_prefix("E-Mail:") {
                            email = rest
                                .split_whitespace()
                                .next()
                                .unwrap_or_default()
                                .to_owned();
                        }
                    }
                }
            }
        }
        el = node.next_sibling();
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
    // Window: the Schrott section only. The Entsorgung table below lists
    // disposal fees (charged per tonne, not paid) and the Baustoffe table
    // lists building-material sales — both must never become prices.
    let start = html
        .find("Aktuelle Preise für Schrott</")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Schrott-Preisbereich fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Preise für Entsorgung von Wertstoffen")
        .unwrap_or(tail.len());
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tbody tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
    // Never trust page order: take the table carrying the Schrott price
    // header, not just the first <table> in the window.
    let mut found: Option<(ElementRef, String)> = None;
    for t in doc.select(&table) {
        let heads: Vec<String> = t
            .select(&head)
            .map(|h| h.text().collect::<String>())
            .collect();
        if heads.iter().any(|h| h.to_lowercase().contains("schrott")) {
            let price_head = heads.get(1).cloned().unwrap_or_default();
            found = Some((t, price_head));
            break;
        }
    }
    let Some((table, price_head)) = found else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    };
    // The unit lives in the Preis header alone ("Preis €/t") — unknown
    // means no row can be trusted → loud error, not a default.
    let Some(unit) = unit_of(&price_head) else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: format!("Einheit unverständlich: {}", price_head.trim()),
        });
    };
    let mut rows = Vec::new();
    for tr in table.select(&row) {
        let cells: Vec<String> = tr
            .select(&cell)
            .map(|c| {
                c.text()
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();
        if cells.len() < 2 || cells[0].is_empty() || cells[0].len() > 120 {
            continue;
        }
        let Some(price) = parse_eur(&cells[1]) else {
            continue;
        };
        rows.push((cells[0].clone(), price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    Ok((rows, Vec::new()))
}

/// Bespoke unit matcher for THIS table's Preis header (live:
/// "Preis €/t"). Only kg/t exist here — anything else fails loudly at
/// the call site.
fn unit_of(header: &str) -> Option<&'static str> {
    let lower = header.to_lowercase();
    if lower.contains("/t") || lower.contains("€/to") || lower.contains("tonne") {
        Some("EUR/t")
    } else if lower.contains("/kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    const FIXTURE: &str =
        "<h2 id=\"aktuelle-preise-fuer-schrott\">Aktuelle Preise für Schrott</h2>\
        <table id=\"tablepress-10\"><thead><tr><th class=\"column-1\">Schrott</th>\
        <th class=\"column-2\">Preis €/t</th></tr></thead><tbody>\
        <tr><td class=\"column-1\">Shreddervormaterial</td><td class=\"column-2\">60,00 €</td></tr>\
        <tr><td class=\"column-1\">Kupfer Millberry</td><td class=\"column-2\">6.400,00 €</td></tr>\
        <tr><td class=\"column-1\">Kernschrott S3</td><td class=\"column-2\">200,00 €</td></tr>\
        <tr><td class=\"column-1\">Batterien</td><td class=\"column-2\">100,00 €</td></tr>\
        <tr><td class=\"column-1\">V4A</td><td class=\"column-2\">800,00 €</td></tr>\
        </tbody></table><h2>Preise für Entsorgung von Wertstoffen</h2>\
        <table><thead><tr><th>Entsorgung</th><th>Preis €/t (Netto)</th></tr></thead>\
        <tbody><tr><td>Altholz</td><td>70,00 €</td></tr></tbody></table>";

    #[test]
    fn section_table_and_unit_parse() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // The Entsorgung table (Altholz) stays outside the window.
        assert_eq!(rows.len(), 5);
        assert!(skips.is_empty());
        assert!(!rows.iter().any(|r| r.0 == "Altholz"));
        assert_eq!(rows[0], ("Shreddervormaterial".to_owned(), 60.0, "EUR/t"));
        // Tausenderpunkt: 6.400 €/t are sixty-four hundred, not 6.4.
        assert_eq!(rows[1].1, 6400.0);
        assert_eq!(unit_of("Preis €/t"), Some("EUR/t"));
        assert_eq!(unit_of("Preis pro Sack"), None);
    }

    #[test]
    fn missing_anchors_fail_loudly() {
        let no_section = FIXTURE.replace("Aktuelle Preise für Schrott</h2>", "Preise</h2>");
        assert!(parse(&no_section).is_err());
        let no_table = "<h2>Aktuelle Preise für Schrott</h2><p>nichts hier</p>";
        assert!(parse(no_table).is_err());
        let bad_unit = FIXTURE.replace("Preis €/t</th>", "Preis pro Sack</th>");
        assert!(parse(&bad_unit).is_err());
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h3>Angaben gemäß § 5 TMG:</h3>\
            <p>SDM Gutzmann GmbH & Co.KG<br />Hauptstraße 46<br />99947 Tottleben</p>\
            <h2>Vertreten durch:</h2><p>Christoph Gutzmann</p>\
            <h2>Kontakt:</h2><p>Telefon: +49(0)03621 733 4660<br />Telefax: +49(0)3603 39 84 315</p>\
            <p>E-Mail: info@sdm-gutzmann.de</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Hauptstraße 46");
        assert_eq!(info.postcode, "99947");
        assert_eq!(info.city, "Tottleben");
        assert_eq!(info.phone, "+49(0)03621 733 4660");
        assert_eq!(info.email, "info@sdm-gutzmann.de");
        assert!(extract_info("<h3>Anderes</h3>").is_err());
        assert!(extract_info("<h3>Angaben gemäß § 5 TMG:</h3><p>x</p>").is_err());
    }

    #[test]
    fn mapping_splits_grades_and_skips_catalog_gaps() {
        assert_eq!(
            grade_for("Kupfer Millberry"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(grade_for("Kupfer Raff"), Some(("kupfer-gemischt", "Raff")));
        assert_eq!(grade_for("Alu Kabel"), Some(("kabel-alu", "")));
        assert_eq!(grade_for("Kupferkabel"), Some(("kabel-kupfer", "")));
        assert_eq!(
            grade_for("Alu Geschirr"),
            Some(("aluminium-blech", "Geschirr"))
        );
        assert_eq!(
            grade_for("Scherenschrott schwer"),
            Some(("stahlschrott-scheren", ""))
        );
        assert_eq!(grade_for("V4A"), Some(("edelstahl-v4a", "")));
        assert_eq!(grade_for("Kernschrott S3"), None, "no Sorte-3 material");
        assert_eq!(grade_for("Batterien"), None, "no battery material");
    }
}
