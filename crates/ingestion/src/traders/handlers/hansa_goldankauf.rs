//! Hansa-Goldankauf (Hamburg-Lokstedt): exact per-gram list prices in a
//! server-rendered Goldrechner (`div.gold-calculator__row`: name in
//! `.gold-calculator__name`, quoted price + `<small>€/g</small>` in
//! `.gold-calculator__price`), plus a Zahngold row and the quote date in
//! `div.gold-calculator__update` ("Letzte Aktualisierung: …").
//!
//! Quoted unit is honestly EUR/g (catalog unit for `gold`/`zahngold`/
//! `silber` is EUR/g — no conversion anywhere). Fineness rides in the
//! variant ("999", "585"), Zahngold maps to `zahngold` (dental alloy,
//! never a gold alias).

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "hh-lokstedt-hansa-goldankauf";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://hansa-goldankauf.de/impressum/";

pub const URL: &str = "https://hansa-goldankauf.de/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (published_at, rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::new();
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
            // No catalog material for precious metal: keep the quoted
            // price as evidence in the skip, never drop it silently.
            None => skipped_labels.push(format!(
                "{label} ({}, {unit}, kein Katalogmaterial: Edelmetall)",
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
        published_at,
    })
}

fn fmt_eur(price: f64) -> String {
    format!("{price:.2}").replace('.', ",")
}

/// Explicit label → (material, variant) mapping. Fineness rides in the
/// variant ("999er Gold" → `gold`/`999`); Zahngold is dental alloy, never
/// a gold alias. Anything without fineness or metal word stays `None`.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    if l.contains("zahngold") {
        return Some(("zahngold", ""));
    }
    let fin = fineness(&l);
    if l.contains("silber") {
        return Some(("silber", fin));
    }
    if l.contains("gold") {
        return Some(("gold", fin));
    }
    None
}

/// First 3-digit run in the label ("999er Gold" → "999"). All grades on
/// this page carry one; labels without stay variant-less (""), never guessed.
fn fineness(l: &str) -> &'static str {
    let b = l.as_bytes();
    let mut i = 0;
    while i + 3 <= b.len() {
        if b[i].is_ascii_digit() && b[i + 1].is_ascii_digit() && b[i + 2].is_ascii_digit() {
            return match &l[i..i + 3] {
                "999" => "999",
                "916" => "916",
                "900" => "900",
                "585" => "585",
                "333" => "333",
                _ => "",
            };
        }
        i += 1;
    }
    ""
}

/// Parse the Goldrechner window between the rows block and the update
/// line. Returns (published_at, rows, unit_skips).
fn parse(
    html: &str,
) -> Result<(Option<String>, Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html.find("gold-calculator__rows").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Goldrechner fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("gold-calculator__update").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Goldrechner unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let row_sel = Selector::parse("div.gold-calculator__row").expect("valid selector");
    let name_sel = Selector::parse(".gold-calculator__name").expect("valid selector");
    let price_sel = Selector::parse(".gold-calculator__price").expect("valid selector");
    let mut rows = Vec::new();
    let mut unit_skips = Vec::new();
    for row in frag.select(&row_sel) {
        let label = row
            .select(&name_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if label.is_empty() || label.len() > 120 {
            continue;
        }
        let price_text = row
            .select(&price_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();
        let Some(price) = parse_eur(&price_text) else {
            continue;
        };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-kilo price recorded as per-gram would be a 1000x error.
        let Some(unit) = unit_of(&price_text) else {
            unit_skips.push(format!("{label} (Einheit unverständlich: {})", price_text.trim()));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Goldrechner leer".to_owned() });
    }
    let published_at = find_date(&html[start..]);
    Ok((published_at, rows, unit_skips))
}

/// Bespoke unit matcher for THIS calculator's price cells (live: "117,11
/// €/g" with `<small>€/g</small>`). Only g/kg/t exist here — anything
/// else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("/g") || lower.contains("€/g") || lower.contains("pro gramm") {
        Some("EUR/g")
    } else if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower.split(|c: char| !c.is_alphanumeric()).any(|t| t == "t") {
        Some("EUR/t")
    } else {
        None
    }
}

/// Bespoke date finder for THIS page: "Letzte Aktualisierung: 27.09.2026
/// 19:11 Uhr" in `div.gold-calculator__update`. No anchor → None (the
/// observation age stays the provenance).
fn find_date(window: &str) -> Option<String> {
    let (_, after) = window.split_once("Letzte Aktualisierung:")?;
    let date = after.split_whitespace().next()?;
    let parts: Vec<&str> = date.split('.').collect();
    if parts.len() == 3 {
        parse_de_date(parts[0], parts[1], parts[2])
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the impressum
/// `<table>` carries an "Anbieter:" row (firm + street + PLZ city) and a
/// "Kontakt:" row ("Telefonnumer:" — the page's own typo — plus
/// "Email:"). Missing rows → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let td = Selector::parse("td").expect("valid selector");
    let mut address_cell = None;
    let mut contact_cell = None;
    for el in doc.select(&td) {
        let text: String = el.text().collect();
        if text.contains("Anbieter:") && address_cell.is_none() {
            // The address lives in the sibling cell of the same row.
            let sibs: Vec<ElementRef> = el
                .parent()
                .map(|p| {
                    p.children()
                        .filter_map(ElementRef::wrap)
                        .filter(|e| e.value().name() == "td")
                        .collect()
                })
                .unwrap_or_default();
            address_cell = sibs.into_iter().nth(1).map(|c| c.inner_html());
        } else if text.contains("Kontakt:") && contact_cell.is_none() {
            let sibs: Vec<ElementRef> = el
                .parent()
                .map(|p| {
                    p.children()
                        .filter_map(ElementRef::wrap)
                        .filter(|e| e.value().name() == "td")
                        .collect()
                })
                .unwrap_or_default();
            contact_cell = sibs.into_iter().nth(1).map(|c| c.inner_html());
        }
    }
    let (Some(addr_html), Some(cont_html)) = (address_cell, contact_cell) else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Anbieter/Kontakt-Zeilen fehlen".to_owned(),
        });
    };
    let addr_lines: Vec<String> = addr_html
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
    let mut phone = String::new();
    let mut email = String::new();
    for part in cont_html.split("<br") {
        let t = strip_fragment(part);
        // "Telefonnumer:" is the page's own spelling — anchor on it.
        if let Some(v) = t.strip_prefix("Telefonnumer:") {
            phone = v.trim().to_owned();
        } else if let Some(v) = t.strip_prefix("Telefonnummer:") {
            phone = v.trim().to_owned();
        } else if let Some(v) = t.strip_prefix("Email:") {
            email = v.trim().to_owned();
        } else if let Some(v) = t.strip_prefix("E-Mail:") {
            email = v.trim().to_owned();
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
    use super::{find_date, grade_for, parse, unit_of};

    // Real shape of the live calculator (class names, data attrs, the
    // Zahngold row and the update line), trimmed to three rows.
    const FIXTURE: &str = "<div class=\"gold-calculator__rows\">\
        <div class=\"gold-calculator__row\" data-price=\"117.10637049996\" data-unit=\"€/g\">\
        <div class=\"gold-calculator__input\"><input type=\"number\" value=\"0.00\"/></div>\
        <div class=\"gold-calculator__name\"><strong>999er Gold</strong></div>\
        <div class=\"gold-calculator__price\">117,11 <small>€/g</small></div></div>\
        <div class=\"gold-calculator__row\" data-price=\"102.235\" data-unit=\"€/g\">\
        <div class=\"gold-calculator__name\"><strong>900er Gold</strong></div>\
        <div class=\"gold-calculator__price\">102,24 <small>€/g</small></div></div>\
        <div class=\"gold-calculator__row\" data-price=\"64.90\" data-unit=\"€/g\">\
        <div class=\"gold-calculator__name\"><strong>Zahngold Gelb gereinigt</strong></div>\
        <div class=\"gold-calculator__price\">64,90 <small>€/g</small></div></div>\
        </div><div class=\"gold-calculator__total\"><span>Summe</span></div>\
        <div class=\"gold-calculator__update\">Letzte Aktualisierung: 27.09.2026 19:11 Uhr</div>";

    #[test]
    fn calculator_rows_units_and_date() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-27T00:00:00+00:00"));
        assert_eq!(rows.len(), 3);
        assert!(skips.is_empty());
        // The displayed quote wins, not the data-price attribute.
        assert_eq!(rows[0], ("999er Gold".to_owned(), 117.11, "EUR/g"));
        assert_eq!(rows[1], ("900er Gold".to_owned(), 102.24, "EUR/g"));
        assert_eq!(rows[2], ("Zahngold Gelb gereinigt".to_owned(), 64.9, "EUR/g"));
        assert_eq!(unit_of("117,11 €/g"), Some("EUR/g"));
        assert_eq!(unit_of("pro Sack"), None);
        assert_eq!(find_date("ohne Datum"), None);
        assert!(parse("<div>Redesign ohne Rechner</div>").is_err());
    }

    #[test]
    fn fineness_rides_in_variant() {
        assert_eq!(grade_for("999er Gold"), Some(("gold", "999")));
        assert_eq!(grade_for("916er Gold"), Some(("gold", "916")));
        assert_eq!(grade_for("900er Gold"), Some(("gold", "900")));
        assert_eq!(grade_for("585er Gold"), Some(("gold", "585")));
        assert_eq!(grade_for("333er Gold"), Some(("gold", "333")));
        // Dental alloy, never a gold alias.
        assert_eq!(
            grade_for("Zahngold Gelb gereinigt"),
            Some(("zahngold", ""))
        );
        assert_eq!(
            grade_for("Silberschmuck und Silbermünzen"),
            Some(("silber", ""))
        );
        assert_eq!(grade_for("Ankaufbedingungen"), None);
    }

    #[test]
    fn impressum_table_rows() {
        let imp = "<table><tbody>\
            <tr><td><strong>Anbieter:</strong></td><td><strong>Hansa-Goldankauf</strong><br>\
            Inhaber: Kumar Chhabra<br><br>Siemersplatz 1<br>22529 Hamburg<br>Deutschland</td></tr>\
            <tr><td><strong>Kontakt:</strong></td><td>Telefonnumer: 040-23 82 520 4<br>\
            Email: info@hansa-goldankauf.de<br>Web: www.hansa-goldankauf.de</td></tr>\
            </tbody></table>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Siemersplatz 1");
        assert_eq!(info.postcode, "22529");
        assert_eq!(info.city, "Hamburg");
        assert_eq!(info.phone, "040-23 82 520 4");
        assert_eq!(info.email, "info@hansa-goldankauf.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}
