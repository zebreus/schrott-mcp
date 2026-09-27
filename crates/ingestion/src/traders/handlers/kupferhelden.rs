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
    Handler { slug: SLUG, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let rows = parse(&html)?;
    let prices = rows
        .into_iter()
        .map(|(label, price, unit)| ScrapedPrice {
            material: "kabel-kupfer",
            price,
            currency: "EUR",
            unit,
            price_min: None,
            price_max: Some(price),
            confidence: Some(0.5),
            label,
        })
        .collect();
    Ok(HandlerOutcome {
        prices,
        skipped_labels: vec![],
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

fn parse(html: &str) -> Result<Vec<(String, f64, &'static str)>, IngestError> {
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
    // Text-node walk; a "bis zu" price closes the pair with the previous
    // text as the grade label.
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
    let mut pending: Option<String> = None;
    for t in texts {
        let t = t.replace("&nbsp;", " ").replace(['\u{a0}'], " ");
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if t.is_empty() {
            continue;
        }
        if t.to_lowercase().contains("bis zu") {
            if let (Some(price), Some(label)) = (parse_eur(&t), pending.take()) {
                rows.push((label, price, eur_unit(&t).unwrap_or("EUR/kg")));
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
    Ok(rows)
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
    use super::parse;

    const FIXTURE: &str = "<h2>TAGESPREISE</h2>\
        <h4>Kabel mit Stecker, bis 37%</h4><div>bis zu 0,45 €/KG</div>\
        <h4>Kupfer ohne Stecker, min 38%,cu</h4><div>bis zu* 1,80 €/KG</div>\
        <footer>Quick Links</footer>";

    #[test]
    fn bis_zu_pairs() {
        let rows = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "Kabel mit Stecker, bis 37%");
        assert_eq!(rows[0].1, 0.45);
        assert_eq!(rows[1].0, "Kupfer ohne Stecker, min 38%,cu");
        assert_eq!(rows[1].1, 1.8);
    }
}
