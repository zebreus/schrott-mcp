//! Kupferhelden (Hattersheim): cable/copper specialist with "bis zu"
//! (up-to) Tagespreise in Elementor cards — no table, no date. The upper
//! bound is honest data for our uncertainty model: price = price_max =
//! advertised value, confidence 0.5. All four grades map to `kabel-kupfer`
//! with the raw grade in the label.

use super::super::{
    eur_unit, fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
};
use crate::IngestError;

pub const SLUG: &str = "he-hattersheim-kupferhelden";
pub const URL: &str = "https://kupferhelden.de/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
    // Exact list prices (no "bis zu") are first-class rows at full
    // confidence; "bis zu" rows carry the bound as price_max at 0.5.
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit, upto) in rows {
        let (price_max, confidence) = if upto { (Some(price), Some(0.5)) } else { (None, Some(1.0)) };
        prices.push(ScrapedPrice {
            material: "kabel-kupfer",
            variant: grade_variant(&label),
            price,
            currency: "EUR",
            unit,
            price_kind: "upto",
            price_min: None,
            price_max,
            confidence,
            label,
            published_at: None,
            valid_from: None,
            valid_to: None,
        });
    }
    // Grades the variant extractor does not know land here, not in the DB.
    prices.retain(|p| {
        if p.variant.is_empty() && p.label.len() > 4 {
            skipped_labels.push(format!("{} (Sorte unverständlich)", p.label));
            false
        } else {
            true
        }
    });
    Ok(HandlerOutcome {
        prices,
        skipped_labels: vec![],
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str, bool)>, Vec<String>), IngestError> {
    // Window: price cards live between TAGESPREISE and the footer links.
    let start = html.find("TAGESPREISE").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "TAGESPREISE fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail
        .find("Quick Links")
        .or_else(|| tail.find("Impressum"))
        .unwrap_or(tail.len());
    let window = &tail[..end];
    // Text-node walk; an € price closes the pair with the previous text
    // as the grade label. Returns (label, price, unit, upto).
    let mut texts = Vec::new();
    let mut in_tag = false;
    let mut cur = String::new();
    for c in window.chars() {
        if c == '<' {
            if !cur.trim().is_empty() {
                texts.push(cur.trim().to_owned());
            }
            cur.clear();
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            cur.push(c);
        }
    }
    let mut rows = Vec::new();
    let mut unit_skips = Vec::new();
    let mut pending: Option<String> = None;
    for t in texts {
        let t = t.replace("&nbsp;", " ").replace(['\u{a0}'], " ");
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if t.is_empty() {
            continue;
        }
        if t.contains('€') && parse_eur(&t).is_some() {
            if let Some(label) = pending.take() {
                let Some(unit) = eur_unit(&t) else {
                    unit_skips.push(format!("{label} (Einheit unverständlich: {t})"));
                    continue;
                };
                rows.push((label, parse_eur(&t).expect("checked"), unit, is_upto(&t)));
            }
        } else if is_junk(&t) {
            pending = None;
        } else {
            pending = Some(match pending {
                Some(p) => format!("{p} / {t}"),
                None => t,
            });
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "keine Preispaare".to_owned() });
    }
    Ok((rows, unit_skips))
}

/// "bis zu" (and variants) mark upper bounds; anything else with € and
/// digits is an exact list price.
fn is_upto(t: &str) -> bool {
    let l = t.to_lowercase();
    l.contains("bis zu") || l.contains("biszu") || l.contains("max.")
}

fn grade_variant(label: &str) -> &'static str {
    let l = label.to_lowercase();
    if l.contains("37%") {
        "bis 37%"
    } else if l.contains("38%") {
        "min 38%"
    } else if l.contains("60%") {
        "min 60%"
    } else if l.contains("70%") {
        "min 70%"
    } else {
        ""
    }
}

fn is_junk(t: &str) -> bool {
    let l = t.to_lowercase();
    ["willkommen", "kontakt", "impressum", "datenschutz", "tagespreise"]
        .iter()
        .any(|j| l.contains(j))
        || l.len() > 120
}

#[cfg(test)]
mod tests {
    use super::{grade_variant, parse};

    const FIXTURE: &str = "<h2>TAGESPREISE</h2>\
        <h4>Kabel mit Stecker, bis 37%</h4><div>bis zu 0,45 €/KG</div>\
        <h4>Kupfer ohne Stecker, min 38%,cu</h4><div>bis zu* 1,80 €/KG</div>\
        <h4>Alukabel sortiert</h4><div>bis zu 0,90 €/KG</div>\
        <h4>Messing Armaturen</h4><div>4,20 €/KG</div>\
        <footer>Quick Links</footer>";

    #[test]
    fn bis_zu_pairs() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 4);
        assert!(skips.is_empty());
        assert_eq!(rows[0].0, "Kabel mit Stecker, bis 37%");
        assert_eq!(rows[0].1, 0.45);
        assert!(rows[0].3, "bis zu flag");
        assert_eq!(rows[1].0, "Kupfer ohne Stecker, min 38%,cu");
        assert_eq!(rows[1].1, 1.8);
        assert!(!rows[3].3, "exact price has no upto flag");
        assert_eq!(rows[3].1, 4.2);
        // Unknown grades never reach the DB as copper cable.
        assert_eq!(grade_variant("Alukabel sortiert"), "");
        assert_eq!(grade_variant("Kupferkabel, min 60%,cu"), "min 60%");
    }
}
