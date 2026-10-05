//! Sommer GmbH (Hanau): the operator's timestamped purchase-price page.
//! Gold and dental-gold rows are quoted per gram; the marketable 1 oz
//! coin is quoted per coin. The silver rows have no explicit unit on the
//! page and are therefore reported as skips rather than guessed.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "he-hanau-63450-sommer";
pub const URL: &str = "https://goldankauf-hanau.de/preise/";

const LABELS: [&str; 11] = [
    "333er Gold",
    "585er Gold",
    "750er Gold",
    "900er Gold",
    "986er Gold",
    "999er Gold Schmelzware",
    "Zahngold (Sofortankauf nur bis 20 g)",
    "Goldmünzen 1 oz 999 (handelsfähig)",
    "Ag 800",
    "Ag 925",
    "Ag 999 Schmelzware",
];

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |client| Box::pin(scrape(client)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (published_at, rows, skipped_labels) = parse(&html)?;
    let prices = rows
        .into_iter()
        .map(|(label, material, variant, price, unit)| ScrapedPrice {
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
        })
        .collect();

    Ok(HandlerOutcome {
        prices,
        acceptances: vec![],
        trader_info: TraderInfo::default(),
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at,
    })
}

type ParsedRow = (String, &'static str, &'static str, f64, &'static str);

fn parse(html: &str) -> Result<(Option<String>, Vec<ParsedRow>, Vec<String>), IngestError> {
    let anchor = html
        .find("Aktuelle Ankaufpreise für Gold")
        .ok_or_else(|| parse_error("Ankaufpreisliste fehlt"))?;
    let start = html[..anchor]
        .rfind("<h2")
        .ok_or_else(|| parse_error("Überschrift der Ankaufpreisliste fehlt"))?;
    let after_heading = anchor + "Aktuelle Ankaufpreise für Gold".len();
    let end = html[after_heading..]
        .find("<h2")
        .map(|offset| after_heading + offset)
        .ok_or_else(|| parse_error("Ende der Ankaufpreisliste fehlt"))?;
    let section = Html::parse_fragment(&html[start..end]);
    let paragraph_selector = Selector::parse("p").expect("valid selector");

    let mut labels = Vec::new();
    let mut quotes = Vec::new();
    for paragraph in section.select(&paragraph_selector) {
        for line in lines_in(paragraph) {
            if LABELS.contains(&line.as_str()) {
                labels.push(line);
            } else if line.contains('€') {
                quotes.push(line);
            }
        }
    }
    if labels.iter().map(String::as_str).collect::<Vec<_>>() != LABELS {
        return Err(parse_error(&format!(
            "Preisetiketten unerwartet (gefunden: {})",
            labels.join(" | ")
        )));
    }
    if quotes.len() != LABELS.len() {
        return Err(parse_error(&format!(
            "Preisanzeige unerwartet: {} statt {} Beträge",
            quotes.len(),
            LABELS.len()
        )));
    }

    let published_at = published_at(html);
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for (index, (label, quote)) in LABELS.into_iter().zip(quotes).enumerate() {
        let Some(price) = parse_eur(&quote) else {
            return Err(parse_error(&format!(
                "Preis nicht lesbar für {label}: {quote}"
            )));
        };
        if !price.is_finite() || price <= 0.0 {
            skipped.push(format!("{label} ({quote}: kein Ankaufspreis)"));
            continue;
        }
        match index {
            0..=5 => rows.push((
                label.to_owned(),
                "gold",
                gold_variant(index),
                price,
                "EUR/g",
            )),
            6 => rows.push((label.to_owned(), "zahngold", "", price, "EUR/g")),
            7 => rows.push((
                label.to_owned(),
                "gold",
                "1 oz 999 Goldmünze handelsfähig",
                price,
                "EUR/Stk",
            )),
            // The page labels these as silver (Ag) but does not state a
            // unit for them; do not inherit the preceding gold heading.
            _ => skipped.push(format!(
                "{label} ({quote}: Einheit auf der Seite nicht ausgewiesen)"
            )),
        }
    }
    if rows.is_empty() {
        return Err(parse_error("keine eindeutig zuordenbaren Ankaufspreise"));
    }
    Ok((published_at, rows, skipped))
}

fn gold_variant(index: usize) -> &'static str {
    ["333", "585", "750", "900", "986", "999"][index]
}

/// Split a paragraph at `<br>` while retaining text nested in `<strong>`.
fn lines_in(paragraph: ElementRef<'_>) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for child in paragraph.children() {
        if let Some(element) = ElementRef::wrap(child) {
            if element.value().name() == "br" {
                push_line(&mut lines, &mut current);
            } else {
                current.push_str(&element.text().collect::<String>());
            }
        } else if let Some(text) = child.value().as_text() {
            current.push_str(text);
        }
    }
    push_line(&mut lines, &mut current);
    lines
}

fn push_line(lines: &mut Vec<String>, current: &mut String) {
    let normalized = current.split_whitespace().collect::<Vec<_>>().join(" ");
    if !normalized.is_empty() {
        lines.push(normalized);
    }
    current.clear();
}

fn published_at(html: &str) -> Option<String> {
    let anchor = html.find("Stand:")? + "Stand:".len();
    let tail = html[anchor..].trim_start();
    let date: String = tail
        .chars()
        .take_while(|character| character.is_ascii_digit() || *character == '.')
        .collect();
    let mut parts = date.split('.');
    parse_de_date(parts.next()?, parts.next()?, parts.next()?)
}

fn parse_error(detail: &str) -> IngestError {
    IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, URL};

    const FIXTURE: &str = r#"
        <h2>Aktuelle Ankaufpreise für Gold</h2>
        <p>Stand: 05.10.2026&nbsp; &nbsp;12:50 Uhr</p>
        <p><strong>Gold je Gramm</strong></p>
        <p><strong>333er Gold</strong><br />
        <strong>585er Gold</strong><br />
        <strong>750er Gold</strong><br />
        <strong>900er Gold</strong><br />
        <strong>986er Gold</strong><br />
        <strong>999er Gold Schmelzware</strong></p>
        <p><strong>Zahngold (Sofortankauf nur bis 20 g)</strong></p>
        <p>&nbsp;</p>
        <p><strong>Goldmünzen 1 oz 999 (handelsfähig)</strong></p>
        <p>&nbsp;</p>
        <p><strong>Ag 800</strong></p>
        <p><strong>Ag 925</strong></p>
        <p><strong>Ag 999 Schmelzware</strong></p>
        <p><strong>&nbsp; 30,75 €</strong><br />
        <strong>&nbsp; 55,40 €</strong><br />
        <strong>&nbsp; 71,15 €</strong><br />
        <strong>&nbsp; 95,60 €</strong><br />
        <strong>104,80</strong><strong>&nbsp;€</strong><br />
        <strong>108,30 €</strong></p>
        <p><strong>&nbsp;55,10 €</strong></p>
        <p>&nbsp;</p>
        <p><strong>3472,00 €</strong></p>
        <p>&nbsp;</p>
        <p><strong>0,80 €</strong></p>
        <p><strong>1,05 €</strong></p>
        <p><strong>1,30 €</strong></p>
        <h2>Service &amp; Beratung</h2>
    "#;

    #[test]
    fn parses_timestamped_buy_quotes_and_skips_unclear_silver_units() {
        let (published_at, rows, skipped) = parse(FIXTURE).expect("parses live shape");
        assert_eq!(published_at.as_deref(), Some("2026-10-05T00:00:00+00:00"));
        assert_eq!(rows.len(), 8);
        assert_eq!(rows[0].1, "gold");
        assert_eq!(rows[0].2, "333");
        assert_eq!(rows[0].3, 30.75);
        assert_eq!(rows[0].4, "EUR/g");
        assert_eq!(rows[4].2, "986");
        assert_eq!(rows[5].2, "999");
        assert_eq!(rows[6].1, "zahngold");
        assert_eq!(rows[6].3, 55.10);
        assert_eq!(rows[7].2, "1 oz 999 Goldmünze handelsfähig");
        assert_eq!(rows[7].3, 3472.0);
        assert_eq!(rows[7].4, "EUR/Stk");
        assert_eq!(skipped.len(), 3);
        assert!(skipped
            .iter()
            .all(|row| row.contains("Einheit auf der Seite nicht ausgewiesen")));
    }

    #[test]
    fn missing_or_reordered_price_list_fails_loudly() {
        assert!(parse("<h2>Umbau</h2>").is_err());
        assert!(parse(&FIXTURE.replace("<strong>585er Gold</strong><br />", "")).is_err());
        assert!(parse(&FIXTURE.replace("30,75 €", "Preis später")).is_err());
        assert!(URL.contains("goldankauf-hanau.de/preise"));
    }
}
