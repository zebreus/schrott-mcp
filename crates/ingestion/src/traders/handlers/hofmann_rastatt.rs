//! Rastatt remuneration table; never the linked waste-disposal fee PDF.
//! NE prices are EUR/t too. Unknown sorts (including batteries) are loud skips.
use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
};
use crate::IngestError;
use scraper::{Html, Selector};

pub const SLUG: &str = "bw-rastatt-hofmann";
pub const URL: &str = "https://hofmann-entsorgung.de/preise-und-verguetungen/";
pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}
async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    // A 429 is a real failed attempt, not a cached successful price scrape.
    let (status, html) = fetch_text(client, URL).await?;
    let mut out = parse(&html)?;
    out.status_code = status;
    out.byte_len = html.len();
    out.fetch_url = URL.to_owned();
    out.website_alive = true;
    Ok(out)
}
fn error(detail: impl Into<String>) -> IngestError {
    IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.into(),
    }
}
fn text(el: scraper::ElementRef<'_>) -> String {
    el.text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    Some(match label {
        "Altblech" => ("stahlschrott-shredder", "Altblech"),
        "Mischschrott leicht" => ("mischschrott", "leicht"),
        "Mischschrott schwer" => ("stahlschrott-scheren", "Mischschrott schwer"),
        "Handelsguss" => ("eisenschrott-gussbruch", "Handelsguss"),
        "Blei alt" => ("blei", "alt"),
        "Zinkblech alt" => ("zink", "Zinkblech alt"),
        "Zinn" => ("zinn", ""),
        "Alu-Geschirr" => ("aluminium-gemischt", "Geschirr"),
        "Alu, bunt, eisenfrei" => ("aluminium-gemischt", "bunt, eisenfrei"),
        "Alu-Felgen, sauber – ohne Anhaftungen" => {
            ("aluminium-felgen", "sauber – ohne Anhaftungen")
        }
        "V2A" => ("edelstahl-v2a", ""),
        "Messing raff" => ("messing", "raff"),
        "Messing schwer" => ("messing", "schwer"),
        "Bronze" => ("bronze-rotguss", "Bronze"),
        "Elektromotoren" => ("elektromotoren", ""),
        "Kupfer Kabel mit Stecker" => ("kabel-mit-stecker", ""),
        "Kupfer Kabel ohne Stecker" => ("kabel-kupfer", "ohne Stecker"),
        "Kupfer leicht" => ("kupfer-leicht", ""),
        "Kupfer schwer" => ("kupfer-schwer", ""),
        // Unqualified Draht is not evidence for blank Berry or Millberry.
        "Kupfer Draht" => ("kupfer-gemischt", "Draht"),
        "Kupfer Berry" => ("kupfer-berry", ""),
        "Kupfer Millberry" => ("kupfer-millberry", ""),
        _ => return None,
    })
}

fn parse(html: &str) -> Result<HandlerOutcome, IngestError> {
    let doc = Html::parse_document(html);
    let heading = Selector::parse("h2, h3").unwrap();
    let headings: Vec<_> = doc.select(&heading).map(text).collect();
    for required in [
        "Preise Schrott & Metalle",
        "Für Privatkunden",
        "Auszahlung von Vergütungen",
    ] {
        if !headings.iter().any(|h| h == required) {
            return Err(error(format!("remuneration context missing: {required}")));
        }
    }
    let table_sel = Selector::parse("table.hmp-table").unwrap();
    let tables: Vec<_> = doc.select(&table_sel).collect();
    if tables.len() != 1 {
        return Err(error("expected one remuneration table"));
    }
    let table = tables[0];
    let th = Selector::parse("thead th").unwrap();
    let headers: Vec<_> = table.select(&th).map(text).collect();
    if headers != ["Sorte", "Preis pro t"] {
        return Err(error("unexpected remuneration headers/unit"));
    }
    let tr = Selector::parse("tbody tr").unwrap();
    let td = Selector::parse("td").unwrap();
    let stand = Selector::parse("span.hmp-stand").unwrap();
    let mut out = HandlerOutcome::default();
    let mut keys = std::collections::HashSet::new();
    for row in table.select(&tr) {
        let cells: Vec<_> = row.select(&td).collect();
        if cells.len() != 2 {
            return Err(error("malformed remuneration row"));
        }
        let label = text(cells[0]);
        let Some((material, variant)) = grade_for(&label) else {
            out.skipped_labels.push(label);
            continue;
        };
        let dates: Vec<_> = cells[1].select(&stand).collect();
        if dates.len() != 1 {
            return Err(error(format!("missing/ambiguous date: {label}")));
        }
        let raw_date = text(dates[0]);
        let date = row_date(&raw_date).ok_or_else(|| error(format!("invalid date: {label}")))?;
        // HandlerOutcome currently has only one date. Fail closed on mixed
        // publication days rather than silently assigning another row's date.
        if out
            .published_at
            .as_ref()
            .is_some_and(|previous| previous != &date)
        {
            return Err(error(
                "mixed row publication dates: row-date support required",
            ));
        }
        out.published_at = Some(date);
        let raw = text(cells[1]);
        let quote = raw
            .strip_suffix(&raw_date)
            .ok_or_else(|| error("row date is not suffix"))?
            .trim();
        let Some(price) = quote_price(quote) else {
            if quote == "auf Anfrage" || quote == "Preis auf Anfrage" {
                out.skipped_labels.push(format!("{label}: auf Anfrage"));
                continue;
            }
            return Err(error(format!(
                "unsupported price/unit/direction: {label}: {quote}"
            )));
        };
        if !keys.insert((material, variant)) {
            return Err(error("duplicate material/variant"));
        }
        out.prices.push(ScrapedPrice {
            material,
            variant,
            price,
            currency: "EUR",
            unit: "EUR/t",
            price_kind: "exact",
            price_min: None,
            price_max: None,
            confidence: Some(1.0),
            label,
        });
    }
    if out.prices.is_empty() {
        return Err(error("no mapped remuneration prices"));
    }
    Ok(out)
}

fn row_date(raw: &str) -> Option<String> {
    let (date, time) = raw.strip_prefix("Stand: ")?.split_once(", ")?;
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

fn quote_price(raw: &str) -> Option<f64> {
    let number = raw.strip_suffix("€/t")?.trim();
    let parts: Vec<_> = number.split(',').collect();
    if parts.len() > 2 || parts.iter().any(|p| p.is_empty()) {
        return None;
    }
    if parts.len() == 2 && !parts[1].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let groups: Vec<_> = parts[0].split('.').collect();
    if groups
        .iter()
        .any(|g| g.is_empty() || !g.chars().all(|c| c.is_ascii_digit()))
        || (groups.len() > 1 && (groups[0].len() > 3 || groups[1..].iter().any(|g| g.len() != 3)))
    {
        return None;
    }
    parse_eur(number).filter(|p| p.is_finite())
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("fixtures/hofmann_rastatt_20260930.html");
    #[test]
    fn complete_live_table_preserves_grades_units_and_date() {
        let out = parse(FIXTURE).unwrap();
        assert_eq!(out.prices.len(), 22);
        assert_eq!(out.skipped_labels, ["Bleibatterien / Starterbatterien"]);
        assert_eq!(
            out.published_at.as_deref(),
            Some("2026-09-30T00:00:00+00:00")
        );
        assert!(out.prices.iter().all(|p| p.unit == "EUR/t"));
        assert_eq!(
            out.prices
                .iter()
                .find(|p| p.material == "kupfer-millberry")
                .unwrap()
                .price,
            10550.0
        );
        assert_eq!(
            out.prices
                .iter()
                .find(|p| p.material == "zinn")
                .unwrap()
                .price,
            11500.0
        );
        let mut keys = std::collections::HashSet::new();
        for price in &out.prices {
            assert!(keys.insert((price.material, price.variant)));
        }
        assert_eq!(grade_for("Bleibatterien / Starterbatterien"), None);
        assert_eq!(grade_for("Kupfer neue Sorte"), None);
        assert_eq!(
            grade_for("Kupfer Draht"),
            Some(("kupfer-gemischt", "Draht"))
        );
    }
    #[test]
    fn rejects_unit_direction_date_and_structure_changes() {
        for changed in [
            FIXTURE.replacen("40 €/t", "40 €/kg", 1),
            FIXTURE.replacen("40 €/t", "-40 €/t", 1),
            FIXTURE.replacen("40 €/t", "bis zu 40 €/t", 1),
            FIXTURE.replacen("30.09.2026", "31.09.2026", 1),
            FIXTURE.replacen("30.09.2026", "29.09.2026", 1),
            FIXTURE.replace("Für Privatkunden", "Entsorgungskosten"),
            FIXTURE.replace("Auszahlung von Vergütungen", "Annahmekosten"),
            FIXTURE.replace("hmp-table", "changed-table"),
            FIXTURE.replace("Preis pro t", "Preis pro kg"),
            FIXTURE.replace("Kupfer Berry", "Kupfer Millberry"),
        ] {
            assert!(parse(&changed).is_err());
        }
    }
    #[test]
    fn unknown_and_unpriced_rows_are_loudly_skipped() {
        let changed = FIXTURE
            .replacen("40 €/t", "auf Anfrage", 1)
            .replace("Handelsguss", "Neue Sorte");
        let out = parse(&changed).unwrap();
        assert_eq!(out.prices.len(), 20);
        assert!(out.skipped_labels.contains(&"Neue Sorte".to_owned()));
        assert!(out
            .skipped_labels
            .contains(&"Altblech: auf Anfrage".to_owned()));
    }
    #[test]
    fn no_prices_or_duplicate_tables_fail() {
        assert!(parse(&FIXTURE.replace("<td>", "<td>Unbekannt ")).is_err());
        assert!(parse(&format!("{FIXTURE}{FIXTURE}")).is_err());
    }
}
