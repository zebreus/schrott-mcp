//! Lausitz Recycling (Lauchhammer): label/price pairs scattered over
//! nested card tables, with the whole block repeated under "Gültig ab".
//! No page date. Strategy: walk all text nodes, pair each price text
//! with the pending label, then dedupe identical pairs. "KEIN ANKAUF …"
//! and paper have no mappable material and are skipped loudly.

use super::super::{
    eur_unit, fetch_text, has_unit_markers, parse_eur, Handler, HandlerOutcome, Schedule,
    ScrapedPrice,
};
use crate::IngestError;

pub const SLUG: &str = "bb-lauchhammer-ost-lausitz-recycling";
pub const URL: &str = "https://www.lausitz-recycling.de/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
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
                        price_kind: "exact",
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
    } else if norm.contains("schwer") || norm.contains("berry") {
        // Same grade, two spellings across page sections ("Cu – schwer"
        // vs "Cu –Berry", same price): one variant keeps them together.
        Some(("kupfer-gemischt", "schwer"))
    } else if norm.contains("raff") {
        Some(("kupfer-gemischt", "Raff"))
    } else if norm.contains("kabel") {
        Some(("kabel-kupfer", ""))
    } else if norm.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else {
        None
    }
}

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    // Only the price area: everything between the price heading and the
    // company section. A stray "5 €" in the footer must never pair with
    // some random pending label into a phantom price.
    let start = html.find("Unsere Preise").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisbereich fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("Über uns").unwrap_or(tail.len());
    let html = &tail[..end];
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
    let mut unit_skips = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut pending: Option<String> = None;
    // Page-global unit: most rows quote bare "€ X" with the unit ("x pro
    // kg") in a sibling node or not at all. EUR/kg is the page default;
    // an explicit different unit always wins, an explicit unknown one
    // skips loudly.
    const PAGE_UNIT: &str = "EUR/kg";
    let nodes: Vec<&str> = text.split('\n').collect();
    let mut i = 0;
    while i < nodes.len() {
        let t = nodes[i]
            .replace("&nbsp;", " ")
            .replace(['\u{a0}', '\u{200b}'], " ")
            .trim()
            .to_owned();
        i += 1;
        if t.is_empty() {
            continue;
        }
        if t.contains('€') {
            if let (Some(price), Some(label)) = (parse_eur(&t), pending.take()) {
                let mut unit = eur_unit(&t);
                // Unit split across nodes ("€ 8,59" + "x pro kg")?
                if unit.is_none() {
                    if let Some(next) = nodes.get(i) {
                        let n = next
                            .replace("&nbsp;", " ")
                            .replace(['\u{a0}', '\u{200b}'], " ")
                            .trim()
                            .to_owned();
                        if let Some(u) = eur_unit(&n) {
                            unit = Some(u);
                            i += 1; // consumed as unit, not a label
                        }
                    }
                }
                let unit = unit.or(if has_unit_markers(&t) { None } else { Some(PAGE_UNIT) });
                let Some(unit) = unit else {
                    unit_skips.push(format!("{label} (Einheit unverständlich: {t})"));
                    continue;
                };
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
    Ok((rows, unit_skips))
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
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 3, "dedupe kills the Gültig-ab repeat: {rows:?}");
        assert!(skips.is_empty());
        assert_eq!(rows[0].0, "Cu - Millberry");
        assert_eq!(rows[0].1, 9.19);
        assert_eq!(rows[1].2, "EUR/t");
        assert_eq!(grade_for("Cu - Millberry"), Some(("kupfer-millberry", "")));
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Cu – schwer"), Some(("kupfer-gemischt", "schwer")));
        assert_eq!(grade_for("Cu –Berry"), Some(("kupfer-gemischt", "schwer")));
        assert_eq!(grade_for("Cu-Raff."), Some(("kupfer-gemischt", "Raff")));
        assert_eq!(grade_for("Altpapier"), None);
        assert!(is_junk("KEIN ANKAUF MEHR VON"));
    }

    #[test]
    fn split_unit_nodes_resolve() {
        // "€ 8,59" + sibling "x pro kg": the unit lives next door.
        let html = "<h2>Unsere Preise</h2>\
            <table><tr><td>Cu - Millberry</td></tr>\
            <tr><td>€ 9,19</td></tr><tr><td>x pro kg</td></tr></table>\
            <h2>Über uns</h2>";
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].2, "EUR/kg");
        assert!(skips.is_empty());
    }

    #[test]
    fn window_and_unit_safety() {
        // A stray euro amount outside the price area must not pair up.
        let html = "<p>Container ab 49 €</p>".to_owned() + FIXTURE
            + "<h2>Über uns</h2><p>Anfahrt pauschal 10 €</p>";
        let (rows, _) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 3);
        // Unknown unit: skipped loudly, valid rows survive.
        let html = FIXTURE.replace("pro kg", "pro Sack");
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "Mischschrott");
        assert_eq!(skips.len(), 3);
    }
}
