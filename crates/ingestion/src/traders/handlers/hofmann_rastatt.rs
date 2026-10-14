//! Rastatt remuneration table; never the linked waste-disposal fee PDF.
//! NE prices are EUR/t too. Unknown sorts are loud skips.
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
    let snapshot = scrape_dated(client).await?;
    legacy_observation(snapshot)
}

/// Keeps row publication dates bound to the parsed price order. Fields are
/// private so callers cannot edit prices independently of their dates.
#[derive(Debug)]
pub struct DatedSnapshot {
    outcome: HandlerOutcome,
    row_dates: Vec<String>,
}

impl DatedSnapshot {
    pub fn observation(&self) -> &HandlerOutcome {
        &self.outcome
    }

    pub(crate) fn row_dates(&self) -> &[String] {
        &self.row_dates
    }
}

pub(crate) async fn scrape_dated(client: &reqwest::Client) -> Result<DatedSnapshot, IngestError> {
    // A 429 is a real failed attempt, not a cached successful price scrape.
    let (status, html) = fetch_text(client, URL).await?;
    let mut snapshot = parse_dated(&html)?;
    let out = &mut snapshot.outcome;
    out.status_code = status;
    out.byte_len = html.len();
    out.fetch_url = URL.to_owned();
    out.website_alive = true;
    Ok(snapshot)
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
        "Zinn alt" => ("zinn", "alt"),
        "Bleibatterien / Starterbatterien" => ("batterien-blei", "Starterbatterien"),
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

#[cfg(test)]
fn parse(html: &str) -> Result<HandlerOutcome, IngestError> {
    legacy_observation(parse_dated(html)?)
}

fn legacy_observation(snapshot: DatedSnapshot) -> Result<HandlerOutcome, IngestError> {
    if snapshot.outcome.published_at.is_none() {
        return Err(error(
            "mixed row publication dates require typed dated snapshot",
        ));
    }
    Ok(snapshot.outcome)
}

fn parse_dated(html: &str) -> Result<DatedSnapshot, IngestError> {
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
    let mut out = HandlerOutcome {
        fetch_url: URL.to_owned(),
        ..HandlerOutcome::default()
    };
    let mut row_dates = Vec::new();
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
            variant: variant.into(),
            price,
            currency: "EUR",
            unit: "EUR/t",
            price_kind: "exact",
            price_min: None,
            price_max: None,
            confidence: Some(1.0),
            label,
        });
        row_dates.push(date);
    }
    if out.prices.is_empty() {
        return Err(error("no mapped remuneration prices"));
    }
    // Preserve the uniform legacy summary, but never choose one row's date
    // for a page with mixed publication days.
    if row_dates.iter().all(|date| date == &row_dates[0]) {
        out.published_at = Some(row_dates[0].clone());
    }
    Ok(DatedSnapshot {
        outcome: out,
        row_dates,
    })
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
    parse_eur(number).filter(|p| p.is_finite() && *p > 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("fixtures/hofmann_rastatt_20260930.html");
    #[test]
    fn complete_live_table_preserves_grades_units_and_date() {
        let out = parse(FIXTURE).unwrap();
        assert_eq!(out.prices.len(), 23);
        assert!(out.skipped_labels.is_empty());
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
            assert!(keys.insert((price.material, price.variant.as_ref())));
        }
        assert_eq!(
            grade_for("Bleibatterien / Starterbatterien"),
            Some(("batterien-blei", "Starterbatterien"))
        );
        assert_eq!(grade_for("Kupfer neue Sorte"), None);
        assert_eq!(
            grade_for("Kupfer Draht"),
            Some(("kupfer-gemischt", "Draht"))
        );
    }
    #[test]
    fn zinn_alt_keeps_the_operators_grade() {
        let changed = FIXTURE.replace("<td>Zinn</td>", "<td>Zinn alt</td>");
        let out = parse(&changed).unwrap();
        assert_eq!(out.prices.len(), 23);
        let zinn = out.prices.iter().find(|p| p.material == "zinn").unwrap();
        assert_eq!(zinn.variant, "alt");
        assert_eq!(zinn.label, "Zinn alt");
        assert_eq!(zinn.price, 11500.0);
        assert!(out.skipped_labels.is_empty());
    }
    #[test]
    fn mixed_dates_can_be_transportable_without_relabelling_prices() {
        let changed = FIXTURE.replacen("30.09.2026", "14.10.2026", 1);
        let snapshot =
            parse_dated(&changed).expect("independently dated rows must be transportable");
        let out = snapshot.observation();
        assert_eq!(out.prices.len(), 23);
        assert!(
            out.published_at.is_none(),
            "no single page date for mixed rows"
        );
        assert_eq!(snapshot.row_dates[0], "2026-10-14T00:00:00+00:00");
        assert!(snapshot.row_dates[1..]
            .iter()
            .all(|date| date == "2026-09-30T00:00:00+00:00"));
    }
    #[test]
    fn skipped_rows_do_not_shift_publication_dates() {
        let changed = FIXTURE
            .replacen("30.09.2026", "14.10.2026", 3)
            .replacen("40 €/t", "auf Anfrage", 1)
            .replace("Mischschrott leicht", "Neue Sorte");
        let snapshot = parse_dated(&changed).unwrap();
        assert_eq!(snapshot.outcome.prices.len(), 21);
        assert_eq!(snapshot.row_dates.len(), 21);
        assert_eq!(snapshot.outcome.prices[0].label, "Mischschrott schwer");
        assert_eq!(snapshot.row_dates[0], "2026-10-14T00:00:00+00:00");
        assert!(snapshot.row_dates[1..]
            .iter()
            .all(|date| date == "2026-09-30T00:00:00+00:00"));
        assert!(snapshot
            .outcome
            .skipped_labels
            .contains(&"Altblech: auf Anfrage".to_owned()));
        assert!(snapshot
            .outcome
            .skipped_labels
            .contains(&"Neue Sorte".to_owned()));
    }
    #[test]
    fn mixed_publication_dates_remain_fail_closed() {
        let context = "<h2>Auszahlung von Vergütungen</h2><h2>Preise Schrott &amp; Metalle</h2><h3>Für Privatkunden</h3>";
        let old = "<tr><td>V2A</td><td>850 €/t<span class='hmp-stand'>Stand: 30.09.2026, 16:22 Uhr</span></td></tr>";
        let new = "<tr><td>Altblech</td><td>35 €/t<span class='hmp-stand'>Stand: 14.10.2026, 07:22 Uhr</span></td></tr>";
        let page = |rows: &str| {
            format!("{context}<table class='hmp-table'><thead><tr><th>Sorte</th><th>Preis pro t</th></tr></thead><tbody>{rows}</tbody></table>")
        };
        assert!(parse(&page(old)).is_ok());
        assert!(parse(&page(new)).is_ok());
        let err = parse(&page(&format!("{old}{new}"))).unwrap_err();
        assert!(err.to_string().contains("mixed row publication dates"));
    }
    #[tokio::test]
    async fn typed_recording_keeps_each_date_and_rejects_lossy_transport() {
        use crate::traders::{record, record_outcome, RecordOutcome};
        use schrott_mcp_store::{InternalDb, PublicDb};

        let dir = crate::test_support::TempDbDir::new("hofmann-row-dates");
        let public = PublicDb::open(dir.path()).unwrap();
        let internal = InternalDb::open(dir.path()).unwrap();
        crate::pipeline::seed_metadata(&public).unwrap();
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-20T22:30:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let snapshot = parse_dated(FIXTURE).unwrap();
        let legacy = snapshot.observation().clone();
        assert!(record(&public, &internal, SLUG, &legacy, &now)
            .await
            .is_err());
        let typed = RecordOutcome::HofmannDated(snapshot);
        assert!(
            record_outcome(&public, &internal, "wrong-trader", &typed, &now)
                .await
                .is_err()
        );
        assert_eq!(
            public
                .query_sql("SELECT COUNT(*) FROM current_prices")
                .unwrap()
                .rows[0][0],
            0
        );
        assert_eq!(
            record_outcome(&public, &internal, SLUG, &typed, &now)
                .await
                .unwrap()
                .recorded,
            23
        );

        // The first quote changes and is newly dated; all other prices keep
        // the older stated date even when a (conflicting) page date exists.
        let changed = FIXTURE
            .replacen("30.09.2026", "14.10.2026", 1)
            .replacen("40 €/t", "45 €/t", 1)
            .replacen("850 €/t", "900 €/t", 1);
        let mut snapshot = parse_dated(&changed).unwrap();
        snapshot.outcome.published_at = Some("2026-10-19T00:00:00+00:00".into());
        let typed = RecordOutcome::HofmannDated(snapshot);
        assert_eq!(
            record_outcome(&public, &internal, SLUG, &typed, &now)
                .await
                .unwrap()
                .recorded,
            23
        );
        let trader = public.find_trader_id(SLUG).unwrap().unwrap();
        for (index, p) in typed.observation().prices.iter().enumerate() {
            let material = public.find_material_id(p.material).unwrap().unwrap();
            let current = public
                .current_price_for(trader, material, p.variant.as_ref())
                .unwrap()
                .unwrap();
            assert_eq!(
                current.published_at.as_deref(),
                typed.price_published_at(index)
            );
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&current.extra_json).unwrap()
                    ["published_at_basis"],
                "row_stated"
            );
            assert_eq!(current.source_url, URL);
        }
        let material = public
            .find_material_id("stahlschrott-shredder")
            .unwrap()
            .unwrap();
        let history = public
            .price_history(trader, material, "Altblech", 10)
            .unwrap();
        assert_eq!(history.len(), 2);
        assert!(history
            .iter()
            .any(|p| p.published_at.as_deref() == Some("2026-09-30T00:00:00+00:00")));
        assert!(history
            .iter()
            .any(|p| p.published_at.as_deref() == Some("2026-10-14T00:00:00+00:00")));

        // Date-only updates must work at unchanged prices too. Removing one
        // date or corrupting it rejects the entire batch before any write.
        let all_new = changed.replace("30.09.2026", "15.10.2026");
        let mut invalid = parse_dated(&all_new).unwrap();
        invalid.row_dates.pop();
        assert!(record_outcome(
            &public,
            &internal,
            SLUG,
            &RecordOutcome::HofmannDated(invalid),
            &now
        )
        .await
        .is_err());
        let mut invalid = parse_dated(&all_new).unwrap();
        invalid.row_dates[22] = "invalid".into();
        assert!(record_outcome(
            &public,
            &internal,
            SLUG,
            &RecordOutcome::HofmannDated(invalid),
            &now
        )
        .await
        .is_err());
        let v2a = public.find_material_id("edelstahl-v2a").unwrap().unwrap();
        assert_eq!(
            public
                .current_price_for(trader, v2a, "")
                .unwrap()
                .unwrap()
                .published_at
                .as_deref(),
            Some("2026-09-30T00:00:00+00:00")
        );
        let typed = RecordOutcome::HofmannDated(parse_dated(&all_new).unwrap());
        record_outcome(&public, &internal, SLUG, &typed, &now)
            .await
            .unwrap();
        assert_eq!(
            public
                .current_price_for(trader, v2a, "")
                .unwrap()
                .unwrap()
                .published_at
                .as_deref(),
            Some("2026-10-15T00:00:00+00:00")
        );
        assert_eq!(
            public
                .query_sql("SELECT COUNT(*) FROM current_prices")
                .unwrap()
                .rows[0][0],
            23
        );
    }
    #[test]
    fn rejects_unit_direction_date_and_structure_changes() {
        for changed in [
            FIXTURE.replacen("40 €/t", "40 €/kg", 1),
            FIXTURE.replacen("40 €/t", "-40 €/t", 1),
            FIXTURE.replacen("40 €/t", "0 €/t", 1),
            FIXTURE.replacen("40 €/t", "bis zu 40 €/t", 1),
            FIXTURE.replacen("30.09.2026", "31.09.2026", 1),
            FIXTURE.replace("Für Privatkunden", "Entsorgungskosten"),
            FIXTURE.replace("Auszahlung von Vergütungen", "Annahmekosten"),
            FIXTURE.replace("hmp-table", "changed-table"),
            FIXTURE.replace("Preis pro t", "Preis pro kg"),
            FIXTURE.replace("Kupfer Berry", "Kupfer Millberry"),
        ] {
            assert!(parse(&changed).is_err());
            assert!(parse_dated(&changed).is_err());
        }
    }
    #[test]
    fn unknown_and_unpriced_rows_are_loudly_skipped() {
        let changed = FIXTURE
            .replacen("40 €/t", "auf Anfrage", 1)
            .replace("Handelsguss", "Neue Sorte");
        let out = parse(&changed).unwrap();
        assert_eq!(out.prices.len(), 21);
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
