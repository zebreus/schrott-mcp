//! Sperling's material Ankauf table only. Original variants and units survive.
//! The block's publication timestamp is not an individual repricing claim for fixed
//! Zinn/Versilbertes. No address enrichment: 22087/22089 remains an owner issue.
use scraper::{ElementRef, Html, Selector};

use super::super::{parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice};
use crate::IngestError;

pub const SLUG: &str = "hh-hohenfelde-trauschmuck-sperling";
pub const URL: &str = "https://www.goldankauf-ge.de/preise";
pub const IMPRESSUM_URL: &str = "https://www.goldankauf-ge.de/impressum";
const MAX_BYTES: usize = 512 * 1024;
const DIRECTION: &str = "Im Folgenden unser aktueller Goldpreis Ankauf für Goldmünzen, Feingoldbarren sowie Silber, Versilbertes, Platin, Palladium und Zinn. Wichtig: Wir zahlen den folgenden Gold Ankaufspreis ohne Abzüge wie Schmelzkosten oder Prüfkosten aus und alle Kurse bis auf Versilbertes und Zinn werden automatisch alle 5 Minuten aktualisiert (dazu Browserfenster aktualisieren).";
// Snapshot contract, including unsupported originals: a missing reviewed grade
// must not leave a silently stale Current quote behind. Additive new grades may
// skip, but removal/changed conditions require a fresh owner-reviewed contract.
const REVIEWED_LABELS: [&str; 47] = [
    "Goldbarren 1-5g",
    "Goldbarren 6-1000g",
    "999er Altgold",
    "986er Altgold",
    "965er Altgold",
    "950er Altgold",
    "917er Altgold",
    "900er Altgold",
    "875er Altgold",
    "833er Altgold",
    "750er Altgold",
    "585er Altgold",
    "417er Altgold",
    "375er Altgold",
    "333er Altgold",
    "Zahngold Gelb gereinigt",
    "Zahngold Gelb mit Keramik",
    "Zahngold Weiß gereinigt",
    "Zahngold Weiß mit Keramik",
    "999er Silber Barren Münzen gestempelt",
    "999er Altsilber",
    "925er Silber",
    "900er Silber",
    "835er Silber",
    "830er Silber",
    "800er Silber",
    "750er Silber",
    "625er Silber",
    "333er Silber",
    "Ab 90er Besteck",
    "Ab 90er Besteck 18/10 Edelstahl",
    "Ab 90er Messer",
    "Ab 90er Messer 18/10 Edelstahl",
    "Unter 90er Besteck",
    "Unter 90er Messer",
    "Korpusware versilbert",
    "vergoldetes Silber",
    "999er Platin",
    "950er Platin",
    "900er Platin",
    "800er Platin",
    "600er Platin",
    "999er Palladium",
    "950er Palladium",
    "900er Palladium",
    "Zinn",
    "Zinn unter 90%",
];

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

fn error(url: &str, detail: &str) -> IngestError {
    IngestError::Parse {
        url: url.into(),
        detail: detail.into(),
    }
}

/// Bound the body before HTML parsing; reject redirects to unrelated sources.
async fn fetch(client: &reqwest::Client, url: &str) -> Result<(u16, String), IngestError> {
    let mut res = client
        .get(url)
        .send()
        .await
        .map_err(|source| IngestError::Fetch {
            url: url.into(),
            source,
        })?;
    let status = res.status().as_u16();
    if !res.status().is_success() || res.url().as_str() != url {
        return Err(error(url, "HTTP status or final source URL mismatch"));
    }
    if res.content_length().is_some_and(|n| n > MAX_BYTES as u64) {
        return Err(error(url, "Response exceeds size limit"));
    }
    let mut body = Vec::new();
    while let Some(chunk) = res.chunk().await.map_err(|source| IngestError::Body {
        url: url.into(),
        source,
    })? {
        if body.len() + chunk.len() > MAX_BYTES {
            return Err(error(url, "Response exceeds size limit"));
        }
        body.extend_from_slice(&chunk);
    }
    let html = String::from_utf8(body).map_err(|_| error(url, "Invalid UTF-8"))?;
    Ok((status, html))
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch(client, URL).await?;
    let (_, identity) = fetch(client, IMPRESSUM_URL).await?;
    check_identity(&identity)?;
    let (prices, skipped_labels, date) = parse(&html)?;
    Ok(HandlerOutcome {
        prices,
        skipped_labels,
        published_at: Some(date),
        website_alive: true,
        fetch_url: URL.into(),
        status_code: status,
        byte_len: html.len(),
        ..Default::default()
    })
}

fn text(el: ElementRef<'_>) -> String {
    el.text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Conservative HTML visibility gate, not a browser/CSS engine. Inspect the
/// consumed node and every ancestor; reject inline hiding even if a later rule
/// could override it. External stylesheet visibility is not inferred.
fn visible(el: ElementRef<'_>) -> bool {
    std::iter::once(el)
        .chain(el.ancestors().filter_map(ElementRef::wrap))
        .all(|node| {
            if node.value().attr("hidden").is_some()
                || node
                    .value()
                    .attr("aria-hidden")
                    .is_some_and(|v| v.trim().eq_ignore_ascii_case("true"))
            {
                return false;
            }
            let style = node
                .value()
                .attr("style")
                .unwrap_or("")
                .to_ascii_lowercase();
            !style.split(';').any(|rule| {
                rule.split_once(':').is_some_and(|(property, value)| {
                    let value = value.split('!').next().unwrap_or("").trim();
                    matches!(
                        (property.trim(), value),
                        ("display", "none")
                            | ("visibility", "hidden" | "collapse")
                            | ("content-visibility", "hidden")
                    )
                })
            })
        })
}

fn visible_tree(el: ElementRef<'_>) -> bool {
    visible(el) && el.descendants().filter_map(ElementRef::wrap).all(visible)
}

fn check_identity(html: &str) -> Result<(), IngestError> {
    if html.len() > MAX_BYTES {
        return Err(error(IMPRESSUM_URL, "Oversized identity document"));
    }
    let doc = Html::parse_document(html);
    let sel = Selector::parse("#content .container.content .row-fluid").unwrap();
    let blocks: Vec<_> = doc.select(&sel).collect();
    if blocks.len() != 1 {
        return Err(error(IMPRESSUM_URL, "Identity block missing/ambiguous"));
    }
    let block_el = blocks[0];
    if !visible_tree(block_el) {
        return Err(error(IMPRESSUM_URL, "Hidden operator identity"));
    }
    let block = text(block_el);
    let lower = block.to_lowercase();
    // One visible, contiguous Anbieter+register block, ending at Kontakt.
    // Reject a second provider/register even within the same row-fluid node.
    let provider_sel = Selector::parse("h2").unwrap();
    let providers: Vec<_> = block_el
        .select(&provider_sel)
        .filter(|el| text(*el) == "Anbieter:")
        .collect();
    let register_sel = Selector::parse("p").unwrap();
    let registers: Vec<_> = block_el
        .select(&register_sel)
        .filter(|el| text(*el).contains("Handelsregister:"))
        .collect();
    const REGISTER: &str = "Geschäftsführer: Patrick Marcel Bächle Handelsregister: Registergericht Hamburg, HRB 119447 Umsatzsteuer-Identifikationsnummer(n): DE278456955";
    let expected = format!("Anbieter: Trauschmuck Sperling GmbH Lübecker Straße 95 22087 Hamburg Deutschland {REGISTER}");
    if providers.len() != 1
        || registers.len() != 1
        || text(registers[0]) != REGISTER
        || block
            .split_once("Kontakt:")
            .map(|(before, _)| before.trim())
            != Some(expected.as_str())
        || lower.matches("anbieter").count() != 1
        || lower.matches("handelsregister").count() != 1
        || lower.matches("hrb").count() != 1
        || lower.matches("gmbh").count() != 1
    {
        return Err(error(
            IMPRESSUM_URL,
            "Operator/provider/register identity ambiguous or changed",
        ));
    }
    Ok(())
}

/// Explicit source whitelist; no substring-derived catalogue guesses.
/// The variant remains the exact normalized label, including all conditions.
fn mapping(label: &str) -> Option<(&'static str, &'static str)> {
    Some(match label {
        "Goldbarren 1-5g" | "Goldbarren 6-1000g" | "999er Altgold" | "986er Altgold"
        | "965er Altgold" | "950er Altgold" | "917er Altgold" | "900er Altgold"
        | "875er Altgold" | "833er Altgold" | "750er Altgold" | "585er Altgold"
        | "417er Altgold" | "375er Altgold" | "333er Altgold" => ("gold", "EUR/g"),
        "Zahngold Gelb gereinigt"
        | "Zahngold Gelb mit Keramik"
        | "Zahngold Weiß gereinigt"
        | "Zahngold Weiß mit Keramik" => ("zahngold", "EUR/g"),
        "999er Silber Barren Münzen gestempelt"
        | "999er Altsilber"
        | "925er Silber"
        | "900er Silber"
        | "835er Silber"
        | "830er Silber"
        | "800er Silber"
        | "750er Silber"
        | "625er Silber"
        | "333er Silber"
        | "vergoldetes Silber" => ("silber", "EUR/g"),
        "999er Platin" | "950er Platin" | "900er Platin" | "800er Platin" | "600er Platin" => {
            ("platin", "EUR/g")
        }
        "999er Palladium" | "950er Palladium" | "900er Palladium" => ("palladium", "EUR/g"),
        "Zinn" | "Zinn unter 90%" => ("zinn", "EUR/kg"),
        _ => return None,
    })
}

fn quote(raw: &str) -> Option<(f64, &'static str)> {
    let (number, unit) = if let Some(n) = raw.strip_suffix(" €/kg") {
        (n, "EUR/kg")
    } else {
        (raw.strip_suffix(" €/g")?, "EUR/g")
    };
    let (whole, fraction) = number.split_once(',')?;
    if whole.is_empty()
        || whole.len() > 6
        || !whole.chars().all(|c| c.is_ascii_digit())
        || fraction.len() != 2
        || !fraction.chars().all(|c| c.is_ascii_digit())
    {
        return None;
    }
    let p = parse_eur(number).filter(|p| p.is_finite() && *p > 0.0)?;
    Some((p, unit))
}

fn block_date(raw: &str) -> Option<String> {
    let raw = raw.strip_prefix("(Aktualisiert: ")?.strip_suffix(')')?;
    let (date, time) = raw.split_once(' ')?;
    let parts: Vec<_> = date.split('.').collect();
    if parts.len() != 3
        || parts[0].len() != 2
        || parts[1].len() != 2
        || parts[2].len() != 4
        || !parts
            .iter()
            .all(|part| part.chars().all(|c| c.is_ascii_digit()))
        || time.len() != 5
        || time.as_bytes()[2] != b':'
    {
        return None;
    }
    let (hour, minute) = time.split_once(':')?;
    if !hour
        .chars()
        .chain(minute.chars())
        .all(|c| c.is_ascii_digit())
        || hour.parse::<u8>().ok()? > 23
        || minute.parse::<u8>().ok()? > 59
    {
        return None;
    }
    use chrono::TimeZone;
    let date = chrono::DateTime::parse_from_rfc3339(&parse_de_date(parts[0], parts[1], parts[2])?)
        .ok()?
        .date_naive();
    let local = date.and_hms_opt(hour.parse().ok()?, minute.parse().ok()?, 0)?;
    chrono_tz::Europe::Berlin
        .from_local_datetime(&local)
        .single()
        .map(|dt| dt.with_timezone(&chrono::Utc).to_rfc3339())
}

fn parse(html: &str) -> Result<(Vec<ScrapedPrice>, Vec<String>, String), IngestError> {
    if html.len() > MAX_BYTES {
        return Err(error(URL, "Oversized price document"));
    }
    let doc = Html::parse_document(html);
    let heading_sel = Selector::parse("#content h2[id='1_0']").unwrap();
    let headings: Vec<_> = doc.select(&heading_sel).collect();
    if headings.len() != 1
        || text(headings[0]) != "1. Goldpreis Ankauf aktuell"
        || !visible_tree(headings[0])
    {
        return Err(error(URL, "Material section missing/ambiguous"));
    }
    let section: Vec<_> = headings[0]
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .take_while(|el| el.value().name() != "h2")
        .collect();
    let paragraphs: Vec<_> = section
        .iter()
        .filter(|el| el.value().name() == "p")
        .collect();
    if !section.iter().all(|el| visible(*el))
        || !paragraphs.iter().all(|el| visible_tree(**el))
        || paragraphs
            .iter()
            .filter(|el| text(***el) == DIRECTION)
            .count()
            != 1
        || section.iter().any(|el| {
            let t = text(*el).to_lowercase();
            t.contains("verkauf") || t.contains("nicht")
        })
    {
        return Err(error(
            URL,
            "Ankauf direction hidden, missing or contradicted in material section",
        ));
    }
    let table_sel = Selector::parse("table.price-table[aria-label='Preise']").unwrap();
    let head = Selector::parse("thead th").unwrap();
    let tables: Vec<_> = section
        .iter()
        .flat_map(|el| el.select(&table_sel))
        .filter(|t| t.select(&head).map(text).collect::<Vec<_>>() == ["Edelmetall", "Preis"])
        .collect();
    if tables.len() != 1 {
        return Err(error(URL, "Material price table missing/ambiguous"));
    }
    let table = tables[0];
    if !visible(table) || !table.select(&head).all(visible_tree) {
        return Err(error(URL, "Hidden material table/header"));
    }
    let panel = table
        .parent()
        .and_then(ElementRef::wrap)
        .ok_or_else(|| error(URL, "Price panel missing"))?;
    let date_sel = Selector::parse(".panel-heading .price_time").unwrap();
    let dates: Vec<_> = panel.select(&date_sel).collect();
    if dates.len() != 1 || !visible_tree(dates[0]) {
        return Err(error(URL, "Price date missing/ambiguous"));
    }
    let date = block_date(&text(dates[0])).ok_or_else(|| error(URL, "Invalid price block date"))?;
    let title_sel = Selector::parse(".panel-heading .panel-title").unwrap();
    let titles: Vec<_> = panel.select(&title_sel).collect();
    if titles.len() != 1 || text(titles[0]) != "Preise" || !visible_tree(titles[0]) {
        return Err(error(URL, "Local material panel title changed or hidden"));
    }
    let row_sel = Selector::parse("tbody tr").unwrap();
    let cells = Selector::parse("td").unwrap();
    let mut prices = Vec::new();
    let mut skipped = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut count = 0;
    for row in table.select(&row_sel) {
        let values: Vec<_> = row.select(&cells).map(text).collect();
        if values.len() != 2 {
            return Err(error(URL, "Malformed material row"));
        }
        if row.value().attr("role") == Some("presentation")
            && row.value().attr("aria-hidden") == Some("true")
            && values[1] == "Preis"
            && [
                "Altgold",
                "Zahngold",
                "Silber",
                "Platin",
                "Palladium",
                "Zinn",
            ]
            .contains(&values[0].as_str())
        {
            continue;
        }
        if !visible_tree(row) {
            return Err(error(URL, "Hidden material row/cell"));
        }
        count += 1;
        if count > 100 || !seen.insert(values[0].clone()) {
            return Err(error(URL, "Too many/duplicate material rows"));
        }
        if !REVIEWED_LABELS.contains(&values[0].as_str())
            && REVIEWED_LABELS
                .iter()
                .any(|known| values[0].contains(known))
        {
            return Err(error(URL, "Known label has unreviewed semantic conditions"));
        }
        // Even unsupported rows must have genuine positive prices with explicit units.
        let (price, unit) =
            quote(&values[1]).ok_or_else(|| error(URL, "Invalid material price/unit"))?;
        if [
            "Ab 90er Besteck",
            "Ab 90er Besteck 18/10 Edelstahl",
            "Ab 90er Messer",
            "Ab 90er Messer 18/10 Edelstahl",
            "Unter 90er Besteck",
            "Unter 90er Messer",
            "Korpusware versilbert",
        ]
        .contains(&values[0].as_str())
            && unit != "EUR/kg"
        {
            return Err(error(URL, "Unsupported plated-ware source unit changed"));
        }
        let Some((material, expected_unit)) = mapping(&values[0]) else {
            skipped.push(values[0].clone());
            continue;
        };
        if unit != expected_unit {
            return Err(error(URL, "Source unit changed"));
        }
        prices.push(ScrapedPrice {
            material,
            variant: values[0].clone().into(),
            price,
            currency: "EUR",
            unit,
            price_kind: "exact",
            price_min: None,
            price_max: None,
            confidence: Some(1.0),
            label: values[0].clone(),
        });
    }
    if !REVIEWED_LABELS.iter().all(|label| seen.contains(*label)) || prices.is_empty() {
        return Err(error(URL, "Incomplete material list"));
    }
    Ok((prices, skipped, date))
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("fixtures/sperling-rates.html");
    const IDENTITY: &str = include_str!("fixtures/sperling-impressum.html");

    #[test]
    fn b1_visibility_and_local_direction_contract() {
        let mutations = [
            ("hidden table", FIXTURE.replace("class=\"table table-striped price-table\"", "hidden style=\"display:none\" class=\"table table-striped price-table\"")),
            ("hidden row", FIXTURE.replace("<tr><td>999er Altgold", "<tr hidden aria-hidden=\"true\"><td>999er Altgold")),
            ("hidden ancestor", FIXTURE.replace("class=\"panel panel-grey margin-bottom-40\"", "class=\"panel panel-grey margin-bottom-40\" style=\"DISPLAY : none !important\"")),
            ("hidden price cell", FIXTURE.replace("118,09 €/g", "<span hidden>118,09 €/g</span>")),
            ("hidden heading", FIXTURE.replace("id=\"1_0\"", "id=\"1_0\" aria-hidden=\"true\"")),
            ("hidden direction", FIXTURE.replace("<p>Im Folgenden", "<p hidden>Im Folgenden")),
            ("sale panel", FIXTURE.replace("<span class=\"panel-title\">Preise</span>", "<span class=\"panel-title\">Verkaufspreise</span>")),
            ("not paying", FIXTURE.replace("Wir zahlen den folgenden Gold Ankaufspreis", "Wir zahlen den folgenden Gold Ankaufspreis nicht; diese Tabelle enthält Verkaufspreise")),
            ("second contradictory paragraph", FIXTURE.replace("<div class=\"panel panel-grey", "<p>Diese Tabelle enthält Verkaufspreise.</p><div class=\"panel panel-grey")),
        ];
        for (name, html) in &mutations {
            assert_ne!(html, FIXTURE, "B1 mutation ineffective: {name}");
        }
        let accepted: Vec<_> = mutations
            .iter()
            .filter(|(_, html)| parse(html).is_ok())
            .map(|(name, _)| *name)
            .collect();
        assert!(
            accepted.is_empty(),
            "B1 unexpectedly accepted: {accepted:?}"
        );
        for hiding in [
            "hidden",
            "aria-hidden=\"true\"",
            "style=\"display:none\"",
            "style=\" DISPLAY : none !important \"",
            "style=\"visibility:hidden\"",
        ] {
            for node in [
                "<div id=\"content\"",
                "<table class=\"table table-striped price-table\"",
                "<tbody",
                "<tr><td>999er Altgold",
                "<td>999er Altgold",
                "<th scope=\"col\"",
                "<span class=\"price_time\"",
            ] {
                let replacement = if node.ends_with("999er Altgold") {
                    node.replacen('>', &format!(" {hiding}>"), 1)
                } else {
                    format!("{node} {hiding}")
                };
                let hidden = FIXTURE.replace(node, &replacement);
                assert_ne!(hidden, FIXTURE);
                assert!(
                    parse(&hidden).is_err(),
                    "Hidden consumption accepted: {node} {hiding}"
                );
            }
        }
    }

    #[test]
    fn b2_visible_unambiguous_provider_register_contract() {
        let mutations = [
            ("hidden provider block", IDENTITY.replace("class=\"row-fluid\"", "class=\"row-fluid\" hidden style=\"display:none\"")),
            ("hidden provider ancestor", IDENTITY.replace("id=\"content\"", "id=\"content\" aria-hidden=\"true\"")),
            ("hidden register", IDENTITY.replace("<p>Geschäftsführer:", "<p hidden>Geschäftsführer:")),
            ("extra provider and register", IDENTITY.replace("</div></div></div>", "<h2>Anbieter:</h2><p>Andere GmbH; Handelsregister: HRB 999999</p></div></div></div>")),
            ("inline extra provider", IDENTITY.replace("Deutschland", "Deutschland Anbieter: Andere GmbH; Handelsregister: HRB 999999")),
            ("extra register only", IDENTITY.replace("DE278456955", "DE278456955 Handelsregister: HRB 999999")),
            ("duplicate correct register", IDENTITY.replace("</div></div></div>", "<p>Handelsregister: Registergericht Hamburg, HRB 119447</p></div></div></div>")),
        ];
        for (name, html) in &mutations {
            assert_ne!(html, IDENTITY, "B2 mutation ineffective: {name}");
        }
        let accepted: Vec<_> = mutations
            .iter()
            .filter(|(_, html)| check_identity(html).is_ok())
            .map(|(name, _)| *name)
            .collect();
        assert!(
            accepted.is_empty(),
            "B2 unexpectedly accepted: {accepted:?}"
        );
        for field in ["Trauschmuck Sperling GmbH", "HRB 119447", "Anbieter:"] {
            let hidden = IDENTITY.replace(
                field,
                &format!("<span style=\"display:none\">{field}</span>"),
            );
            assert!(
                check_identity(&hidden).is_err(),
                "Hidden identity field: {field}"
            );
        }
    }

    #[test]
    fn b3_semantic_tails_and_complete_reviewed_coverage_contract() {
        let unknown_row = "<tr><td>998er Altgold</td><td>117,00 €/g</td></tr>";
        let mutations = [
            (
                "known label sale tail",
                FIXTURE.replace("999er Altgold", "999er Altgold Verkauf nur ab 100kg"),
            ),
            (
                "known dental condition tail",
                FIXTURE.replace(
                    "Zahngold Weiß gereinigt",
                    "Zahngold Weiß gereinigt nur ab 100kg",
                ),
            ),
            (
                "masked missing Zinn",
                FIXTURE.replace(
                    "<tr><td>Zinn unter 90%</td><td class=\"text-right\">7,00 €/kg</td></tr>",
                    unknown_row,
                ),
            ),
            (
                "masked missing unsupported grade",
                FIXTURE.replace("Korpusware versilbert", "998er Altgold"),
            ),
        ];
        for (name, html) in &mutations {
            assert_ne!(html, FIXTURE, "B3 mutation ineffective: {name}");
        }
        let accepted: Vec<_> = mutations
            .iter()
            .filter(|(_, html)| parse(html).is_ok())
            .map(|(name, _)| *name)
            .collect();
        assert!(
            accepted.is_empty(),
            "B3 unexpectedly accepted: {accepted:?}"
        );
        // Keep the count constant while masking each reviewed row with unknown.
        for label in REVIEWED_LABELS {
            let cell = if label == "Goldbarren 6-1000g" {
                format!("<td>{label} </td>")
            } else {
                format!("<td>{label}</td>")
            };
            let masked = FIXTURE.replace(&cell, "<td>998er Altgold</td>");
            assert_ne!(masked, FIXTURE, "Ineffective coverage mutation: {label}");
            assert!(
                parse(&masked).is_err(),
                "Missing reviewed grade masked by unknown: {label}"
            );
        }
        // Tails also fail with all 47 originals still present.
        let added_tail = FIXTURE.replace("<tr><td>Zinn unter 90%</td><td class=\"text-right\">7,00 €/kg</td></tr>", "<tr><td>Zinn unter 90%</td><td class=\"text-right\">7,00 €/kg</td></tr><tr><td>Zahngold Weiß gereinigt nur ab 100kg</td><td>21,26 €/g</td></tr>");
        assert!(parse(&added_tail).is_err());
    }

    #[test]
    fn source_rows_preserve_all_variants_units_and_block_date() {
        let (prices, skipped, date) = parse(FIXTURE).unwrap();
        assert_eq!(prices.len(), 40);
        assert_eq!(
            skipped,
            [
                "Ab 90er Besteck",
                "Ab 90er Besteck 18/10 Edelstahl",
                "Ab 90er Messer",
                "Ab 90er Messer 18/10 Edelstahl",
                "Unter 90er Besteck",
                "Unter 90er Messer",
                "Korpusware versilbert"
            ]
        );
        assert_eq!(date, "2026-10-11T11:05:00+00:00");
        assert_eq!(prices[0].price, 118.41);
        assert_eq!(prices[1].price, 118.21);
        assert_eq!(prices[15].variant, "Zahngold Gelb gereinigt");
        assert_eq!(prices[18].variant, "Zahngold Weiß mit Keramik");
        assert_eq!(prices[19].variant, "999er Silber Barren Münzen gestempelt");
        assert_eq!(prices[20].variant, "999er Altsilber");
        assert!(prices
            .iter()
            .filter(|p| p.material == "zinn")
            .all(|p| p.unit == "EUR/kg"));
        assert!(prices
            .iter()
            .filter(|p| p.material != "zinn")
            .all(|p| p.unit == "EUR/g"));
        let variants: std::collections::HashSet<_> =
            prices.iter().map(|p| (&p.material, &p.variant)).collect();
        assert_eq!(variants.len(), 40);
        check_identity(IDENTITY).unwrap();
    }

    #[test]
    fn source_date_identity_unit_fail_closed() {
        for html in [
            String::new(),
            FIXTURE.replace("11.10.2026 13:05", "31.02.2026 13:05"),
            FIXTURE.replace("11.10.2026 13:05", "11.10.2026 24:00"),
            FIXTURE.replace("Aktualisiert:", "Zuletzt aktualisiert:"),
            FIXTURE.replace("118,41 €/g", "118,41 €/kg"),
            FIXTURE.replace("Wir zahlen den folgenden Gold Ankaufspreis", "Shop Verkauf"),
            FIXTURE.replace("999er Altgold", "986er Altgold"),
            FIXTURE.replace("118,41", "bis zu 118,41"),
            FIXTURE.replace("118,41", "-118,41"),
            FIXTURE.replace("118,41", "0,00"),
        ] {
            assert!(parse(&html).is_err());
        }
        for html in [
            IDENTITY.replace("119447", "1194470"),
            IDENTITY.replace("Sperling GmbH", "Andere GmbH"),
            IDENTITY.replace("22087", "22089"),
            String::new(),
        ] {
            assert!(check_identity(&html).is_err());
        }
        assert!(parse(&"x".repeat(MAX_BYTES + 1)).is_err());
    }

    #[test]
    fn unknown_labels_reported_without_catalog_guess() {
        let extra = FIXTURE.replace("<tr><td>Zinn unter 90%</td><td class=\"text-right\">7,00 €/kg</td></tr>", "<tr><td>Zinn unter 90%</td><td class=\"text-right\">7,00 €/kg</td></tr><tr><td>998er Altgold</td><td>117,00 €/g</td></tr>");
        let (prices, skipped, _) = parse(&extra).unwrap();
        assert_eq!(prices.len(), 40);
        assert!(skipped.contains(&"998er Altgold".into()));
        assert_eq!(mapping("Ab 90er Besteck 18/10 Edelstahl"), None);
        assert_eq!(mapping("Shop Gold 999"), None);
        assert!(parse(&format!("{FIXTURE}{FIXTURE}")).is_err());
        let h = super::super::all();
        assert_eq!(
            h.iter().filter(|h| h.slug == SLUG && h.url == URL).count(),
            1
        );
    }

    #[test]
    fn block_scope_calendar_and_quote_grammar_regressions() {
        for raw in [
            "118,41 €",
            "118.41 €/g",
            "118,41-120,00 €/g",
            "118,41,5 €/g",
            "NaN €/g",
            "1.118,41 €/g",
            "118,41 €/t",
        ] {
            assert!(quote(raw).is_none(), "{raw}");
        }
        for raw in [
            "(Aktualisiert: 29.02.2025 13:05)",
            "(Aktualisiert: 25.10.2026 02:30)",
            "(Aktualisiert: 29.03.2026 02:30)",
            "(Aktualisiert: 11.10.2026 13:5)",
        ] {
            assert!(block_date(raw).is_none(), "{raw}");
        }
        assert_eq!(
            block_date("(Aktualisiert: 11.01.2026 13:05)").as_deref(),
            Some("2026-01-11T12:05:00+00:00")
        );
        let row = "<tr><td>Zinn unter 90%</td><td class=\"text-right\">7,00 €/kg</td></tr>";
        assert!(parse(&FIXTURE.replace(row, "")).is_err());
        assert!(parse(&FIXTURE.replace("32,00 €/kg", "32,00 €/g")).is_err());
        assert!(parse(&FIXTURE.replace("id=\"1_0\"", "id=\"other\"")).is_err());
        assert!(parse(&FIXTURE.replace(
            "<span class=\"panel-title\">Preise</span>",
            "<span class=\"price_time\">(Aktualisiert: 11.10.2026 13:05)</span>"
        ))
        .is_err());
    }

    #[tokio::test]
    async fn mapped_rows_record_to_real_local_catalog_without_canaries() {
        use schrott_mcp_store::{InternalDb, PublicDb};
        let dir = crate::test_support::TempDbDir::new("sperling");
        let public = PublicDb::open(dir.path()).unwrap();
        let internal = InternalDb::open(dir.path()).unwrap();
        crate::seed_metadata(&public).unwrap();
        let (prices, skipped_labels, date) = parse(FIXTURE).unwrap();
        let out = HandlerOutcome {
            prices,
            skipped_labels,
            published_at: Some(date.clone()),
            fetch_url: URL.into(),
            ..Default::default()
        };
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-11T12:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let recorded = super::super::super::record(&public, &internal, SLUG, &out, &now)
            .await
            .unwrap();
        assert_eq!(recorded.recorded, 40);
        assert_eq!(recorded.skipped.len(), 7);
        assert!(recorded.canaries.is_empty(), "{:?}", recorded.canaries);
        let trader = public.find_trader_id(SLUG).unwrap().unwrap();
        for price in &out.prices {
            let material = public.find_material_id(price.material).unwrap().unwrap();
            let current = public
                .current_price_for(trader, material, price.variant.as_ref())
                .unwrap()
                .unwrap();
            assert_eq!(current.price, price.price);
            assert_eq!(current.published_at.as_deref(), Some(date.as_str()));
            assert_eq!(current.unit, price.unit);
        }
    }

    #[tokio::test]
    #[ignore = "explicit network-only check: no database, no ingestion"]
    async fn live_db_free_handler() {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(45))
            .build()
            .unwrap();
        let out = (handler().scrape)(&client).await.unwrap();
        assert_eq!(out.status_code, 200);
        assert_eq!(out.fetch_url, URL);
        assert_eq!(out.prices.len(), 40);
        assert_eq!(out.skipped_labels.len(), 7);
        assert!(out.trader_info.is_empty());
        println!("Sperling LIVE DB-FREE: status={} bytes={} prices={} skipped={:?} price_block_published_at={:?}", out.status_code, out.byte_len, out.prices.len(), out.skipped_labels, out.published_at);
    }
}
