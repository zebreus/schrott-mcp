//! Lausitz Recycling (Lauchhammer): label/price pairs scattered over
//! nested card tables, with the whole block repeated under "Gültig ab".
//! No page date. Strategy: walk all text nodes, pair each price text
//! with the pending label, then dedupe identical pairs. "KEIN ANKAUF …"
//! and paper have no mappable material and are skipped loudly.

use super::super::{
    eur_unit, fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
};
use crate::IngestError;

pub const SLUG: &str = "bb-lauchhammer-ost-lausitz-recycling";
pub const URL: &str = "https://www.lausitz-recycling.de/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let rows = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    let mut skipped_labels = Vec::new();
    // The "Gültig ab" repeat uses slightly different spellings for the
    // same grades: collapse identical (material, price) pairs.
    let mut seen = std::collections::HashSet::new();
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => {
                if seen.insert((material, variant, price.to_bits())) {
                    prices.push(ScrapedPrice {
                        material,
                        variant,
                        price,
                        currency: "EUR",
                        unit,
                        price_min: None,
                        price_max: None,
                        confidence: Some(1.0),
                        label,
                        published_at: None,
                        valid_from: None,
                        valid_to: None,
                    });
                }
            }
            None => skipped_labels.push(label),
        }
    }
    Ok(HandlerOutcome {
        prices,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let norm: String = label
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ')
        .collect();
    let norm = norm.as_str();
    if norm.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if norm.contains("messing") {
        Some(("messing", ""))
    } else if norm.contains("schwer") || norm.contains("berry") || norm.contains("raff") {
        Some(("kupfer-berry", ""))
    } else if norm.contains("kabel") {
        Some(("kabel-kupfer", ""))
    } else if norm.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else {
        None
    }
}

fn parse(html: &str) -> Result<Vec<(String, f64, &'static str)>, IngestError> {
    // Text-node walk: strip tags, split on the remnants.
    let mut text = String::with_capacity(html.len() / 2);
    let mut in_tag = false;
    for c in html.chars() {
        if c == '<' {
            in_tag = true;
            text.push('\n');
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            text.push(c);
        }
    }
    let mut rows: Vec<(String, f64, &'static str)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut pending: Option<String> = None;
    for raw in text.split('\n') {
        let t = raw
            .replace("&nbsp;", " ")
            .replace(['\u{a0}', '\u{200b}'], " ")
            .trim()
            .to_owned();
        if t.is_empty() {
            continue;
        }
        if t.contains('€') {
            if let (Some(price), Some(label)) = (parse_eur(&t), pending.take()) {
                let unit = eur_unit(&t).unwrap_or("EUR/kg");
                // The page repeats the block under "Gültig ab": dedupe.
                if seen.insert((label.clone(), price.to_bits())) {
                    rows.push((label, price, unit));
                }
            }
        } else if is_junk(&t) {
            pending = None;
        } else {
            pending = Some(t);
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "keine Preispaare".to_owned() });
    }
    Ok(rows)
}

/// Section headings and prose — never a material label.
fn is_junk(t: &str) -> bool {
    let l = t.to_lowercase();
    ["unsere preise", "gültig ab", "über uns", "leistungen", "kontakt", "impressum",
     "datenschutz", "kein ankauf", "cookies", "anfrage", "rufen sie", "folgen sie"]
        .iter()
        .any(|j| l.contains(j))
        || l.len() > 120
}

#[cfg(test)]
mod tests {
    use super::{is_junk, grade_for, parse};

    const FIXTURE: &str = "<h2>Unsere Preise</h2>\
        <table><tr><td><strong>Cu - Millberry</strong></td></tr>\
        <tr><td><strong>€ 9,19 x pro kg</strong></td></tr></table>\
        <table><tr><td><strong>Mischschrott</strong></td></tr>\
        <tr><td><strong>€ 100 x pro to</strong></td></tr></table>\
        <h2>Gültig ab</h2>\
        <table><tr><td><strong>Cu - Millberry</strong></td></tr>\
        <tr><td><strong>€ 9,19 X € pro kg</strong></td></tr></table>\
        <table><tr><td><strong>Altpapier</strong></td></tr>\
        <tr><td><strong>€ 0,08 X € pro kg</strong></td></tr></table>";

    #[test]
    fn pairs_dedupe_and_map() {
        let rows = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 3, "dedupe kills the Gültig-ab repeat: {rows:?}");
        assert_eq!(rows[0].0, "Cu - Millberry");
        assert_eq!(rows[0].1, 9.19);
        assert_eq!(rows[1].2, "EUR/t");
        assert_eq!(grade_for("Cu - Millberry"), Some(("kupfer-millberry", "")));
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Altpapier"), None);
        assert!(is_junk("KEIN ANKAUF MEHR VON"));
    }
}
