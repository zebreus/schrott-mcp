//! Oschatz Recycling (Oschatzer Recycling- und Umwelt-Technik GmbH,
//! Oschatz): exact and ranged Fe-buy prices in the homepage price ticker
//! (`div.price-ticker`: "Baustahl: 0,13€/kg", "Elektromotoren: von 0,30 bis
//! 0,40€/kg", "Katalysatoren: 20,00€/Stück"). One-sided rows ("von X"
//! without "bis") would be a guess and skip loudly; two-sided rows become
//! `price_kind: "range"` with the midpoint as `price` at confidence 0.5
//! (ms_recycling_frankfurt precedent) — never a silent exact.
//!
//! Mapping (all deliberate, precedents in other handlers): "Bremsscheiben
//! und Trommeln" → `eisenschrott-gussbruch` (Grauguss; altmittweida,
//! albus_leipzig, doering, neuwert agree), "Guss" → `eisenschrott-gussbruch`,
//! "Scherenvormaterial schwer" → `stahlschrott-scheren` ("schwer";
//! db_recycling/koppe precedent), "Schredderschrott" → `stahlschrott-shredder`
//! (neuwert precedent), "Mischschrott"/"Elektromotoren"/"Katalysatoren" →
//! their exact catalog materials. "Baustahl" has no catalog entry (neither
//! Sorte 1 nor Scheren is provable), "Brennmatten" is not provably
//! Brennerschrott, and "Industrieschrott Sorte 3" has no catalog entry
//! (neuwert precedent) — all three skip loudly.
//!
//! Units are this ticker's own spellings only ("€/kg" → EUR/kg,
//! "€/Stück" → EUR/Stk); anything else skips loudly. The page states no
//! price date (only a Saturday-opening note), so `published_at` is None.
//! Contact lives in the homepage footer: the site has NO separate impressum
//! route (live 404 on /impressum, /impressum.html, /impressum/,
//! /impressum.php, /datenschutz, /datenschutz.html, /kontakt and
//! /impressum-datenschutz; no "Impressum" string anywhere on the page), so
//! IMPRESSUM_URL is the homepage itself and `extract_info` anchors on the
//! `footer.footer` "Kontakt" block. Missing anchors fail loudly.

use std::collections::HashSet;

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sn-oschatz-oschatz-recycling-oschatzer-recycling-un";
/// Bespoke, live-verified impressum URL. This single-page site exposes no
/// separate impressum route (all probed variants 404 — see module docs),
/// so the homepage footer is the contact source. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://oschatzrecycling.de";

pub const URL: &str = "https://oschatzrecycling.de";

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
    let mut seen = HashSet::new();
    for row in rows {
        match grade_for(&row.label) {
            Some((material, variant)) => {
                // Ticker repeats would collapse onto one current price —
                // dedupe after mapping, not on raw labels.
                if seen.insert((material, variant, row.price.to_bits())) {
                    prices.push(ScrapedPrice {
                        material,
                        variant,
                        price: row.price,
                        currency: "EUR",
                        unit: row.unit,
                        price_kind: row.price_kind,
                        price_min: row.price_min,
                        price_max: row.price_max,
                        confidence: row.confidence,
                        label: row.label,
                    });
                }
            }
            None => skipped_labels.push(format!("{} ({})", row.label, skip_note(&row.label))),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // block means the site changed and needs eyeballs before we trust
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

/// One parsed ticker row before material mapping.
struct Row {
    label: String,
    price: f64,
    price_min: Option<f64>,
    price_max: Option<f64>,
    price_kind: &'static str,
    confidence: Option<f64>,
    unit: &'static str,
}

/// Explicit label → (material, variant) mapping, specific before generic.
/// Anything unlisted is skipped loudly at the call site; the variant keeps
/// the trader's own grade wording so rows never collapse.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("elektromotor") {
        Some(("elektromotoren", ""))
    } else if l.contains("katalysator") {
        Some(("katalysatoren", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("bremsscheiben") {
        // Grey-cast discs and drums → Gussbruch, own variant.
        Some(("eisenschrott-gussbruch", "Bremsscheiben und Trommeln"))
    } else if l.contains("scherenvormaterial") || l.contains("scherenschrott") {
        Some(("stahlschrott-scheren", "schwer"))
    } else if l.contains("schredder") || l.contains("shredder") {
        Some(("stahlschrott-shredder", ""))
    } else if l.contains("guss") {
        Some(("eisenschrott-gussbruch", ""))
    } else {
        None
    }
}

/// Loud reason for ticker labels without catalog material.
fn skip_note(label: &str) -> &'static str {
    let l = label.to_lowercase();
    if l.contains("sorte 3") {
        "kein Katalogmaterial (Vorschlag: stahlschrott-sorte-3)"
    } else if l.contains("baustahl") {
        "kein Katalogmaterial (Baustahl: Sorte unbelegbar)"
    } else if l.contains("brennmatten") {
        "kein Katalogmaterial (Brennmatten uneindeutig)"
    } else {
        "kein Katalogmaterial"
    }
}

/// Bespoke contact extraction for THIS footer only: inside `footer.footer`
/// the `<h3>Kontakt</h3>` block holds firm/street/"PLZ Ort" `<p>` lines and
/// the `<h3>Telefon</h3>` block the `tel:`/`mailto:` links. Missing anchors
/// mean the page changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let footer = Selector::parse("footer.footer").expect("valid selector");
    let h3 = Selector::parse("h3").expect("valid selector");
    let tel = Selector::parse("a[href^=\"tel:\"]").expect("valid selector");
    let mail = Selector::parse("a[href^=\"mailto:\"]").expect("valid selector");
    let foot = doc
        .select(&footer)
        .next()
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Footer-Block fehlt".to_owned(),
        })?;
    let has_kontakt = foot
        .select(&h3)
        .any(|h| h.text().collect::<String>().trim() == "Kontakt");
    if !has_kontakt {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    }
    // Address lines: <p> siblings after the Kontakt heading up to the next
    // heading ("…Recycling- und Umwelt-Technik GmbH" / street / "PLZ Ort").
    let mut lines: Vec<String> = Vec::new();
    let mut in_kontakt = false;
    for node in foot.descendants().filter_map(ElementRef::wrap) {
        let name = node.value().name();
        if name == "h3" {
            let title: String = node.text().collect();
            in_kontakt = title.trim() == "Kontakt";
        } else if in_kontakt && name == "p" {
            let t: String = node.text().collect();
            let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
    if lines.is_empty() || !lines[0].contains("Recycling") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmenzeile fehlt".to_owned(),
        });
    }
    let street = lines.get(1).cloned().unwrap_or_default();
    let (mut postcode, mut city) = (String::new(), String::new());
    if let Some(plz) = lines.get(2) {
        let mut it = plz.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = ci.to_owned();
            }
        }
    }
    let phone = foot
        .select(&tel)
        .next()
        .map(|a| {
            a.text()
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();
    let email = foot
        .select(&mail)
        .next()
        .and_then(|a| a.value().attr("href"))
        .and_then(|h| h.strip_prefix("mailto:"))
        .unwrap_or("")
        .trim()
        .to_owned();
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

/// Cut the ticker window (start AND end anchored) and parse its
/// `span.price-item` rows. Missing anchors or 0 rows → loud `Err`, never
/// an empty success (a redesign must be heard).
fn parse(html: &str) -> Result<(Vec<Row>, Vec<String>), IngestError> {
    // Markup-anchors (with `<div`), never bare class names: those also
    // occur in the page's own <style> block and would cut a CSS window.
    let start = html
        .find("<div class=\"price-ticker\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preis-Ticker fehlt".to_owned(),
        })?;
    let end_rel = html[start..]
        .find("<div class=\"services-section\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ticker-Ende fehlt".to_owned(),
        })?;
    let window = &html[start..start + end_rel];
    let doc = Html::parse_document(window);
    let item = Selector::parse("span.price-item").expect("valid selector");
    let value = Selector::parse("span.price-value").expect("valid selector");
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for el in doc.select(&item) {
        let value_text = el
            .select(&value)
            .next()
            .map(|v| v.text().collect::<String>())
            .unwrap_or_default();
        let full: String = el.text().collect();
        let label = full
            .replace(value_text.trim(), "")
            .trim()
            .trim_end_matches(':')
            .trim()
            .replace(['\u{a0}'], " ");
        if label.is_empty() || label.len() > 120 {
            continue;
        }
        if value_text.trim().is_empty() {
            skipped.push(format!("{label} (kein Preis)"));
            continue;
        }
        let Some(unit) = unit_of(&value_text) else {
            skipped.push(format!(
                "{label} (Einheit unverständlich: {})",
                value_text.trim()
            ));
            continue;
        };
        let Some((price, price_min, price_max, price_kind, confidence)) = parse_value(&value_text)
        else {
            skipped.push(format!(
                "{label} (Preis unverständlich: {})",
                value_text.trim()
            ));
            continue;
        };
        // Never record an exact 0.00 price — that is "no buy price".
        if price == 0.0 {
            skipped.push(format!("{label} (Preis 0,00 — kein Ankaufspreis)"));
            continue;
        }
        rows.push(Row {
            label,
            price,
            price_min,
            price_max,
            price_kind,
            confidence,
            unit,
        });
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preis-Ticker leer".to_owned(),
        });
    }
    Ok((rows, skipped))
}

/// Bespoke value parser for THIS ticker's spellings: "0,13€/kg" → exact,
/// "von 0,14 bis 0,20€/kg" → range (midpoint, min/max, 0.5). A one-sided
/// "von/ab …" without "bis" is not a price we may quote → None (loud skip
/// at the call site, never a silent exact).
fn parse_value(raw: &str) -> Option<(f64, Option<f64>, Option<f64>, &'static str, Option<f64>)> {
    let lower = raw.to_lowercase();
    if lower.contains("bis") {
        let mut parts = lower.splitn(2, "bis");
        let lo = parse_eur(parts.next().unwrap_or(""))?;
        let hi = parse_eur(parts.next().unwrap_or(""))?;
        if lo <= 0.0 || hi <= 0.0 {
            return None;
        }
        Some(((lo + hi) / 2.0, Some(lo), Some(hi), "range", Some(0.5)))
    } else {
        let t = lower.trim_start();
        if t.starts_with("von ") || t.starts_with("ab ") || t.starts_with("bis zu") {
            return None;
        }
        let v = parse_eur(raw)?;
        Some((v, None, None, "exact", Some(1.0)))
    }
}

/// Bespoke unit matcher for THIS ticker's value spellings (live: "€/kg",
/// "€/Stück"). Only kg/Stk exist here — anything else skips loudly at the
/// call site (a per-tonne price recorded as per-kg would be a 1000x error).
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower.contains("stück") || lower.contains("stuck") || lower.contains("stk") {
        Some("EUR/Stk")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, parse_value, skip_note, unit_of};

    // Real excerpt of the live ticker (28.09.2026), trimmed to 5 items.
    const FIXTURE: &str = "<div class=\"price-ticker\"><div class=\"ticker-content\">\
        <span class=\"price-item\">Baustahl: <span class=\"price-value\">0,13€/kg</span></span>\
        <span class=\"price-item\">Bremsscheiben und Trommeln: \
        <span class=\"price-value\">von 0,14 bis 0,20€/kg</span></span>\
        <span class=\"price-item\">Elektromotoren: \
        <span class=\"price-value\">von 0,30 bis 0,40€/kg</span></span>\
        <span class=\"price-item\">Katalysatoren: \
        <span class=\"price-value\">20,00€/Stück</span></span>\
        <span class=\"price-item\">Mischschrott: \
        <span class=\"price-value\">von 0,10 bis 0,13€/kg</span></span>\
        </div></div><div class=\"services-section\">";

    #[test]
    fn ticker_rows_parse() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 5);
        assert!(skips.is_empty());
        let exact = &rows[0];
        assert_eq!(exact.label, "Baustahl");
        assert_eq!(exact.price, 0.13);
        assert_eq!(exact.price_kind, "exact");
        assert_eq!(exact.confidence, Some(1.0));
        assert_eq!(exact.unit, "EUR/kg");
        let range = &rows[1];
        assert_eq!(range.label, "Bremsscheiben und Trommeln");
        assert!((range.price - 0.17).abs() < 1e-9);
        assert_eq!((range.price_min, range.price_max), (Some(0.14), Some(0.20)));
        assert_eq!(range.price_kind, "range");
        assert_eq!(range.confidence, Some(0.5));
        let kat = &rows[3];
        assert_eq!(kat.price, 20.0);
        assert_eq!(kat.unit, "EUR/Stk");
    }

    #[test]
    fn ticker_anchors_and_units_fail_loudly() {
        assert!(parse("<div>kein Ticker hier</div>").is_err());
        // End anchor missing: loud error, not a silent window.
        let html = FIXTURE.replace("services-section", "anders");
        assert!(parse(&html).is_err());
        // Unknown unit: row skipped loudly, valid rows survive.
        let html = FIXTURE.replacen("0,13€/kg", "0,13 pro Sack", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 4);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Baustahl") && skips[0].contains("Einheit"));
        // Every row unparseable: loud error, not silent success.
        let html = FIXTURE
            .replace("€/kg", "pro Sack")
            .replace("€/Stück", "pro Sack");
        assert!(parse(&html).is_err());
        // One-sided "von" without "bis" is no quotable price.
        assert!(parse_value("von 0,30€/kg").is_none());
        assert!(parse_value("ab 0,30€/kg").is_none());
        assert_eq!(unit_of("20,00€/Stück"), Some("EUR/Stk"));
        assert_eq!(unit_of("0,13€/kg"), Some("EUR/kg"));
        assert_eq!(unit_of("5 Euro pro Sack"), None);
    }

    #[test]
    fn mapping_covers_every_live_label() {
        assert_eq!(grade_for("Baustahl"), None);
        assert_eq!(
            grade_for("Bremsscheiben und Trommeln"),
            Some(("eisenschrott-gussbruch", "Bremsscheiben und Trommeln"))
        );
        assert_eq!(grade_for("Brennmatten"), None);
        assert_eq!(grade_for("Elektromotoren"), Some(("elektromotoren", "")));
        assert_eq!(grade_for("Guss"), Some(("eisenschrott-gussbruch", "")));
        assert_eq!(grade_for("Industrieschrott Sorte 3"), None);
        assert_eq!(grade_for("Katalysatoren"), Some(("katalysatoren", "")));
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(
            grade_for("Scherenvormaterial schwer"),
            Some(("stahlschrott-scheren", "schwer"))
        );
        assert_eq!(
            grade_for("Schredderschrott"),
            Some(("stahlschrott-shredder", ""))
        );
        assert!(skip_note("Industrieschrott Sorte 3").contains("stahlschrott-sorte-3"));
        assert!(skip_note("Baustahl").contains("Baustahl"));
        assert!(skip_note("Brennmatten").contains("uneindeutig"));
    }

    #[test]
    fn impressum_footer_extracts_contact() {
        let imp = "<footer class=\"footer\"><div class=\"footer-content\">\
            <div class=\"footer-section\"><h3>Kontakt</h3>\
            <p>Oschatzer Recycling- und Umwelt-Technik GmbH</p>\
            <p>Schlachthofstraße 2</p><p>04758 Oschatz</p>\
            <p>Geschäftsführer: Immo Schwarz</p></div>\
            <div class=\"footer-section\"><h3>Telefon</h3>\
            <p><a href=\"tel:03435622728\">03435 | 62 27 28</a></p>\
            <p><a href=\"mailto:max@oschatzrecycling.de\">max@oschatzrecycling.de</a></p>\
            </div></div></footer>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Schlachthofstraße 2");
        assert_eq!(info.postcode, "04758");
        assert_eq!(info.city, "Oschatz");
        assert_eq!(info.phone, "03435 | 62 27 28");
        assert_eq!(info.email, "max@oschatzrecycling.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<footer><p>Neu hier</p></footer>").is_err());
        assert!(extract_info("<div>kein Footer</div>").is_err());
    }
}
