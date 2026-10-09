//! AGH Altgoldhandel's dated scrap-gold purchase table.
//! Bullion buy/sell prices and the separate calculator are deliberately excluded.
use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
};
use crate::IngestError;
use scraper::{Html, Selector};

pub const SLUG: &str = "he-erlensee-agh-altgoldhandel";
pub const URL: &str = "https://www.agh-goldankauf.de/wir-kaufen/altgoldankauf/";
const IMPRESSUM_URL: &str = "https://www.agh-goldankauf.de/impressum/";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |client| Box::pin(scrape(client)),
    }
}

fn error(url: &str, detail: impl Into<String>) -> IngestError {
    IngestError::Parse {
        url: url.to_owned(),
        detail: detail.into(),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status_code, html) = fetch_text(client, URL).await?;
    let mut out = parse(&html)?;
    let (_, impressum) = fetch_text(client, IMPRESSUM_URL).await?;
    verify_operator(&impressum)?;
    out.status_code = status_code;
    out.byte_len = html.len();
    out.website_alive = true;
    Ok(out)
}

fn verify_operator(html: &str) -> Result<(), IngestError> {
    let doc = Html::parse_document(html);
    let body = doc.root_element().text().collect::<String>();
    if [
        "AGH Altgoldhandel",
        "Raimund Pyrka",
        "Langendiebacherstr. 45",
        "63526 Erlensee",
        "DE 270343605",
    ]
    .iter()
    .all(|fact| body.contains(fact))
    {
        Ok(())
    } else {
        Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Betreiberidentität oder Standort hat sich geändert".to_owned(),
        })
    }
}

fn grade(label: &str) -> Option<&'static str> {
    match label {
        "Feingold 999 (24kt)" => Some("999"),
        "Gold 986 (23,6kt)" => Some("986"),
        "Gold 916 (22kt)" => Some("916"),
        "Gold 900 (21,6kt)" => Some("900"),
        "Gold 750 (18kt)" => Some("750"),
        "Gold 585 (14kt)" => Some("585"),
        "Gold 375 (9kt)" => Some("375"),
        "Gold 333 (8kt)" => Some("333"),
        _ => None,
    }
}

fn text(el: scraper::ElementRef<'_>) -> String {
    el.text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn date_from_heading(heading: &str) -> Option<String> {
    let (date, time) = heading
        .strip_prefix("Ankauf- und Verkauf-Preise vom ")?
        .split_once(" um ")?;
    let time = time.strip_suffix(" Uhr")?;
    let (hour, minute) = time.split_once(':')?;
    if hour.len() != 2
        || minute.len() != 2
        || hour.parse::<u8>().ok()? > 23
        || minute.parse::<u8>().ok()? > 59
    {
        return None;
    }
    let parts: Vec<_> = date.split('.').collect();
    if parts.len() != 3 || parts[0].len() != 2 || parts[1].len() != 2 || parts[2].len() != 4 {
        return None;
    }
    parse_de_date(parts[0], parts[1], parts[2])
}

fn parse(html: &str) -> Result<HandlerOutcome, IngestError> {
    let doc = Html::parse_document(html);
    let block_selector = Selector::parse(".goldrechner").unwrap();
    let blocks: Vec<_> = doc.select(&block_selector).collect();
    if blocks.len() != 1 {
        return Err(error(URL, "Goldrechner fehlt oder mehrfach vorhanden"));
    }
    let block = blocks[0];
    let heading = Selector::parse("h2").unwrap();
    let heading = block
        .select(&heading)
        .next()
        .map(text)
        .ok_or_else(|| error(URL, "Preisstand fehlt"))?;
    let published_at =
        date_from_heading(&heading).ok_or_else(|| error(URL, "Preisstand ungültig"))?;

    let h3 = Selector::parse("h3").unwrap();
    if !block.select(&h3).any(|item| text(item) == "Goldankauf") {
        return Err(error(URL, "Goldankauf-Überschrift fehlt"));
    }
    let rows = Selector::parse(".rechner.goldankauf > .rechner__row").unwrap();
    let columns = Selector::parse(".rechner__column").unwrap();
    let mut out = HandlerOutcome {
        fetch_url: URL.to_owned(),
        published_at: Some(published_at),
        ..HandlerOutcome::default()
    };
    let mut variants = std::collections::HashSet::new();
    for row in block.select(&rows) {
        let cells: Vec<_> = row.select(&columns).map(text).collect();
        if cells.len() != 2 {
            return Err(error(URL, "Goldankauf-Spalten verändert"));
        }
        let Some(variant) = grade(&cells[0]) else {
            out.skipped_labels.push(cells[0].clone());
            continue;
        };
        if !variants.insert(variant) {
            return Err(error(URL, "doppelter Feingehalt"));
        }
        let amount = cells[1]
            .strip_suffix(" €/g")
            .ok_or_else(|| error(URL, "Ankaufspreis nicht in EUR/g"))?;
        if !amount
            .chars()
            .all(|ch| ch.is_ascii_digit() || ch == ',' || ch == '.')
        {
            return Err(error(URL, "kein exakter Ankaufspreis"));
        }
        let price = parse_eur(amount)
            .filter(|price| price.is_finite() && *price > 0.0)
            .ok_or_else(|| error(URL, "ungültiger Ankaufspreis"))?;
        out.prices.push(ScrapedPrice {
            material: "gold",
            variant,
            price,
            currency: "EUR",
            unit: "EUR/g",
            price_kind: "exact",
            price_min: None,
            price_max: None,
            confidence: Some(1.0),
            label: cells[0].clone(),
        });
    }
    if out.prices.len() != 8 {
        return Err(error(URL, "unvollständige Goldankauf-Feingehalte"));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("fixtures/agh_altgoldhandel.html");

    #[test]
    fn extracts_only_dated_gold_purchase_grades() {
        let out = parse(FIXTURE).unwrap();
        assert_eq!(
            out.published_at.as_deref(),
            Some("2026-10-09T00:00:00+00:00")
        );
        let expected = [
            ("999", 113.18),
            ("986", 105.83),
            ("916", 98.32),
            ("900", 96.60),
            ("750", 76.02),
            ("585", 59.30),
            ("375", 37.63),
            ("333", 33.41),
        ];
        assert_eq!(out.prices.len(), expected.len());
        for (price, (variant, value)) in out.prices.iter().zip(expected) {
            assert_eq!(price.material, "gold");
            assert_eq!(price.variant, variant);
            assert_eq!(price.unit, "EUR/g");
            assert!((price.price - value).abs() < 1e-9);
        }
        assert!(out.skipped_labels.is_empty());
    }

    #[test]
    fn fails_closed_on_changed_price_context_date_unit_or_value() {
        for changed in [
            FIXTURE.replace("Goldankauf</h3>", "Goldverkauf</h3>"),
            FIXTURE.replace("113,18 €/g", "113,18 €/kg"),
            FIXTURE.replace("113,18 €/g", "0,00 €/g"),
            FIXTURE.replace("09.10.2026", "31.02.2026"),
            FIXTURE.replace("09.10.2026", "2026-10-09"),
            FIXTURE.replace("21:10 Uhr", "25:10 Uhr"),
            FIXTURE.replace("Gold 333 (8kt)", "Gold 334 (8kt)"),
        ] {
            assert!(parse(&changed).is_err());
        }
    }

    #[test]
    fn unknown_grade_is_reported_without_guessing() {
        let extra = r#"<div class="rechner__row"><div class="rechner__column">Gold 875</div><div class="rechner__column">90,00 €/g</div></div>"#;
        let changed = FIXTURE.replace(
            "<div class=\"rechner goldankauf\">",
            &format!("<div class=\"rechner goldankauf\">{extra}"),
        );
        let out = parse(&changed).unwrap();
        assert_eq!(out.prices.len(), 8);
        assert_eq!(out.skipped_labels, ["Gold 875"]);
    }

    #[test]
    fn operator_identity_is_checked_separately() {
        let impressum =
            "AGH Altgoldhandel Raimund Pyrka Langendiebacherstr. 45 63526 Erlensee DE 270343605";
        assert!(verify_operator(impressum).is_ok());
        assert!(verify_operator(&impressum.replace("Raimund Pyrka", "anderer Betreiber")).is_err());
    }
}
