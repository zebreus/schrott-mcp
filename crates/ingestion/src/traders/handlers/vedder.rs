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
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (published_at, rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
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

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. The variant keeps the trader's own grade wording.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kerze") {
        Some(("kupfer-berry", "Kerze"))
    } else if l.contains("raff") && !l.contains("kabel") {
        Some(("kupfer-berry", "Alt"))
    } else if l.contains("kabel") && l.contains("kupfer") {
        Some(("kabel-kupfer", "38%"))
    } else if l.contains("rotguss") || l.contains("bronze") {
        Some(("bronze-rotguss", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", ""))
    } else if l.contains("v2a") || l.contains("edelstahl") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("profile") {
        Some(("aluminium-profile", ""))
    } else if l.contains("blech") {
        Some(("aluminium-blech", ""))
    } else if l.contains("guss") && l.contains("alu") {
        Some(("aluminium-guss", ""))
    } else if l.contains("zinn") {
        // Grades: the range IS the grade ("Zinn 80% - 98% (Geschirr)" …).
        if l.contains("80%") {
            Some(("zinn", "80-98%"))
        } else if l.contains("70%") {
            Some(("zinn", "70-79%"))
        } else if l.contains("60%") {
            Some(("zinn", "60-69%"))
        } else if l.contains("50%") {
            Some(("zinn", "50-59%"))
        } else {
            Some(("zinn", ""))
        }
    } else if l.contains("geschirr") {
        Some(("aluminium-blech", "Geschirr"))
    } else {
        None
    }
}

fn parse(
    html: &str,
) -> Result<(Option<String>, Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tbody tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
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
    let mut unit_skips = Vec::new();
    // Never trust page order: take the table carrying the price header,
    // not just the first <table> on the page.
    let table = doc.select(&table).find(|t| {
        t.select(&head).any(|h| {
            h.text()
                .collect::<String>()
                .to_lowercase()
                .contains("material")
        })
    });
    let Some(table) = table else {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "keine Preistabelle".to_owned() });
    };
    for tr in table.select(&row) {
        let cells: Vec<String> = tr.select(&cell).map(|c| c.text().collect()).collect();
        if cells.len() < 3 {
            continue;
        }
        let label = cells[0].trim().replace(['\u{a0}'], " ");
        let Some(price) = parse_eur(&cells[1]) else { continue };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let Some(unit) = eur_unit(&cells[2]) else {
            unit_skips.push(format!("{label} (Einheit unverständlich: {})", cells[2].trim()));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Preistabelle leer".to_owned() });
    }
    Ok((published_at, rows, unit_skips))
}

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    const FIXTURE: &str = "<h2>Unverbindliche Ankaufspreise27.09.2026</h2>\
        <table><thead><tr><th>Materialbezeichnung</th><th>Preis</th><th>Einheit</th></tr></thead>\
        <tbody><tr><td>Kupfer-Kabel blank (Millberry)</td><td>9,80</td><td>EUR / KG</td></tr>\
        <tr><td>Messing gemischt</td><td>4,90</td><td>EUR / KG</td></tr>\
        <tr><td>Hartmetall Widia Platten und Bohrer</td><td>30,00</td><td>EUR / KG</td></tr>\
        </tbody></table>";

    #[test]
    fn table_and_date_parse() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-27T00:00:00+00:00"));
        assert_eq!(rows.len(), 3);
        assert!(skips.is_empty());
        assert_eq!(rows[0].0, "Kupfer-Kabel blank (Millberry)");
        assert_eq!(rows[0].1, 9.8);
        assert_eq!(rows[0].2, "EUR/kg");
    }

    #[test]
    fn wrong_table_and_unit_are_rejected_loudly() {
        // A layout table before the price table must not win.
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (_, rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 3);
        // Unknown unit: skipped loudly, valid rows survive.
        let html = FIXTURE.replacen("EUR / KG", "pro Sack", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Millberry"));
        // Every row unparseable: loud error, not silent success.
        let html = FIXTURE.replace("EUR / KG", "pro Sack");
        let err = parse(&html).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn mapping_skips_catalog_gaps() {
        assert_eq!(
            grade_for("Kupfer-Kabel blank (Millberry)"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(grade_for("Kupfer blank (Kerze)"), Some(("kupfer-berry", "Kerze")));
        assert_eq!(grade_for("Rotguss Stücke sauber"), Some(("bronze-rotguss", "")));
        assert_eq!(grade_for("Edelstahlabfälle V4A"), Some(("edelstahl-v4a", "")));
        assert_eq!(grade_for("Zinn 80% - 98% (Geschirr)"), Some(("zinn", "80-98%")));
        assert_eq!(grade_for("Zinn 50% - 59%"), Some(("zinn", "50-59%")));
        assert_eq!(grade_for("Hartmetall Widia Platten und Bohrer"), None);
        assert_eq!(grade_for("Versilberte Messer"), None);
    }
}
