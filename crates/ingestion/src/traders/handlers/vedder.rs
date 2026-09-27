//! Vedder & Stockrahm (Bremen): exact daily prices in a clean HTML table,
//! plus the price date in the heading ("Unverbindliche Ankaufspreise
//! 27.09.2026"). Hartmetall and silverware have no catalog material and
//! are skipped loudly.

use scraper::{Html, Selector};

use super::super::{
    eur_unit, fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule,
    ScrapedPrice,
};
use crate::IngestError;

pub const SLUG: &str = "hb-woltmershausen-vedder-stockrahm";
pub const URL: &str = "https://www.vedder-stockrahm.de/ankauf/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (published_at, rows) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    let mut skipped_labels = Vec::new();
    for (label, price, unit) in rows {
        match material_for(&label) {
            Some(material) => prices.push(ScrapedPrice {
                material,
                price,
                currency: "EUR",
                unit,
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label,
            }),
            None => skipped_labels.push(label),
        }
    }
    Ok(HandlerOutcome {
        prices,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at,
    })
}

/// Explicit label → material mapping. Anything unlisted is skipped.
fn material_for(label: &str) -> Option<&'static str> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some("kupfer-millberry")
    } else if l.contains("kerze") || (l.contains("raff") && !l.contains("kabel")) {
        Some("kupfer-berry")
    } else if l.contains("kabel") && l.contains("kupfer") {
        Some("kabel-kupfer")
    } else if l.contains("rotguss") || l.contains("bronze") {
        Some("bronze-rotguss")
    } else if l.contains("messing") {
        Some("messing")
    } else if l.contains("blei") {
        Some("blei")
    } else if l.contains("zink") {
        Some("zink")
    } else if l.contains("v4a") {
        Some("edelstahl-v4a")
    } else if l.contains("v2a") || l.contains("edelstahl") {
        Some("edelstahl-v2a")
    } else if l.contains("profile") {
        Some("aluminium-profile")
    } else if l.contains("blech") {
        Some("aluminium-blech")
    } else if l.contains("guss") && l.contains("alu") {
        Some("aluminium-guss")
    } else if l.contains("zinn") {
        Some("zinn")
    } else if l.contains("geschirr") {
        Some("aluminium-blech")
    } else {
        None
    }
}

fn parse(html: &str) -> Result<(Option<String>, Vec<(String, f64, &'static str)>), IngestError> {
    let doc = Html::parse_document(html);
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tbody tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let h2 = Selector::parse("h2").expect("valid selector");
    let mut published_at = None;
    for el in doc.select(&h2) {
        let text: String = el.text().collect();
        // "Unverbindliche Ankaufspreise27.09.2026"
        if let Some(date) = text.split("Ankaufspreise").nth(1) {
            let parts: Vec<&str> = date.trim().split('.').collect();
            if parts.len() == 3 {
                published_at = parse_de_date(parts[0], parts[1], parts[2]);
            }
        }
    }
    let mut rows = Vec::new();
    let Some(table) = doc.select(&table).next() else {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "keine Preistabelle".to_owned() });
    };
    for tr in table.select(&row) {
        let cells: Vec<String> = tr.select(&cell).map(|c| c.text().collect()).collect();
        if cells.len() < 3 {
            continue;
        }
        let (Some(price), Some(unit)) =
            (parse_eur(&cells[1]), eur_unit(&cells[2]).or(Some("EUR/kg")))
        else {
            continue;
        };
        rows.push((cells[0].trim().replace(['\u{a0}'], " "), price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Preistabelle leer".to_owned() });
    }
    Ok((published_at, rows))
}

#[cfg(test)]
mod tests {
    use super::{material_for, parse};

    const FIXTURE: &str = "<h2>Unverbindliche Ankaufspreise27.09.2026</h2>\
        <table><thead><tr><th>Materialbezeichnung</th><th>Preis</th><th>Einheit</th></tr></thead>\
        <tbody><tr><td>Kupfer-Kabel blank (Millberry)</td><td>9,80</td><td>EUR / KG</td></tr>\
        <tr><td>Messing gemischt</td><td>4,90</td><td>EUR / KG</td></tr>\
        <tr><td>Hartmetall Widia Platten und Bohrer</td><td>30,00</td><td>EUR / KG</td></tr>\
        </tbody></table>";

    #[test]
    fn table_and_date_parse() {
        let (published_at, rows) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-27T00:00:00+00:00"));
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].0, "Kupfer-Kabel blank (Millberry)");
        assert_eq!(rows[0].1, 9.8);
        assert_eq!(rows[0].2, "EUR/kg");
    }

    #[test]
    fn mapping_skips_catalog_gaps() {
        assert_eq!(material_for("Kupfer-Kabel blank (Millberry)"), Some("kupfer-millberry"));
        assert_eq!(material_for("Rotguss Stücke sauber"), Some("bronze-rotguss"));
        assert_eq!(material_for("Edelstahlabfälle V4A"), Some("edelstahl-v4a"));
        assert_eq!(material_for("Hartmetall Widia Platten und Bohrer"), None);
        assert_eq!(material_for("Versilberte Messer"), None);
    }
}
