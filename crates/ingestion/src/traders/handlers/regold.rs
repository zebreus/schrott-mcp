//! reGOLD Spandau: server-rendered dynamic Ankaufkurse, not calculator output.
//! All twenty fineness rows are EUR/g indicative prices ("Richtwert").
//! No publication date is stated; never substitute the HTTP/observation date.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "be-spandau-regold-edelmetallhandel";
pub const URL: &str = "https://www.regold.de/vor-ort-ankauf.html";
pub const IMPRESSUM_URL: &str = "https://www.regold.de/impressum.html";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (prices, skipped_labels) = parse(&html)?;
    let (_, imp) = fetch_text(client, IMPRESSUM_URL).await?;
    Ok(HandlerOutcome {
        prices,
        skipped_labels,
        trader_info: extract_info(&imp)?,
        website_alive: true,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
        ..Default::default()
    })
}

fn error(url: &str, detail: &str) -> IngestError {
    IngestError::Parse {
        url: url.to_owned(),
        detail: detail.to_owned(),
    }
}

fn text(el: ElementRef<'_>) -> String {
    el.text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Exhaustive live label table: Zahngold stays separate even at equal prices.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    Some(match label {
        "Gold 999" => ("gold", "999"),
        "Gold 585" => ("gold", "585"),
        "Gold 900" => ("gold", "900"),
        "Gold 333" => ("gold", "333"),
        "Gold 750" => ("gold", "750"),
        "Zahngold 750" => ("zahngold", "750"),
        "Zahngold 600" => ("zahngold", "600"),
        "Silber 999" => ("silber", "999"),
        "Silber 835" => ("silber", "835"),
        "Silber 700" => ("silber", "700"),
        "Silber 925" => ("silber", "925"),
        "Silber 800" => ("silber", "800"),
        "Silber 625" => ("silber", "625"),
        "Silber 900" => ("silber", "900"),
        "Platin 999" => ("platin", "999"),
        "Platin 950" => ("platin", "950"),
        "Platin 750" => ("platin", "750"),
        "Palladium 999" => ("palladium", "999"),
        "Palladium 950" => ("palladium", "950"),
        "Palladium 500" => ("palladium", "500"),
        _ => return None,
    })
}

fn parse(html: &str) -> Result<(Vec<ScrapedPrice>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let rows = Selector::parse("#kurse .k-value").expect("selector");
    let label_sel = Selector::parse(".k-label").expect("selector");
    let price_sel = Selector::parse(".k-price").expect("selector");
    let heading = Selector::parse("h2").expect("selector");
    if !doc
        .select(&heading)
        .any(|el| text(el) == "Börsen Ankaufkurse")
    {
        return Err(error(URL, "Ankaufkurse-Überschrift fehlt"));
    }
    let mut prices = Vec::new();
    let mut skipped = Vec::new();
    for row in doc.select(&rows) {
        let label = row
            .select(&label_sel)
            .next()
            .map(text)
            .ok_or_else(|| error(URL, "Kurslabel fehlt"))?;
        let Some((material, variant)) = grade_for(&label) else {
            skipped.push(label);
            continue;
        };
        let raw = row
            .select(&price_sel)
            .next()
            .map(text)
            .ok_or_else(|| error(URL, "Kurspreis fehlt"))?;
        // Strict suffix/numeric validation: kg, placeholders, negative values,
        // ranges and prose must not pass through parse_eur's permissive scan.
        let number = raw
            .strip_suffix("€ / g")
            .map(str::trim)
            .ok_or_else(|| error(URL, "Kurs nicht EUR/g"))?;
        let valid_number = number.split_once(',').is_some_and(|(whole, cents)| {
            !whole.is_empty()
                && whole.chars().all(|c| c.is_ascii_digit())
                && cents.len() == 2
                && cents.chars().all(|c| c.is_ascii_digit())
        });
        if !valid_number {
            return Err(error(URL, "Ungültiger Grammpreis"));
        }
        let price = parse_eur(number)
            .filter(|p| p.is_finite() && *p > 0.0)
            .ok_or_else(|| error(URL, "Ungültiger Grammpreis"))?;
        if prices
            .iter()
            .any(|p: &ScrapedPrice| p.material == material && p.variant == variant)
        {
            return Err(error(URL, "Doppelter Feingehalt"));
        }
        prices.push(ScrapedPrice {
            material,
            variant,
            price,
            currency: "EUR",
            unit: "EUR/g",
            price_kind: "approx",
            price_min: None,
            price_max: None,
            confidence: Some(0.8),
            label,
        });
    }
    if prices.is_empty() {
        return Err(error(URL, "Keine bekannten Ankaufkurse"));
    }
    Ok((prices, skipped))
}

/// Only the site's own Impressum table; footer/privacy prose cannot fill fields.
fn extract_info(html: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(html);
    let row_sel = Selector::parse("#impressum .ce_text table tr").expect("selector");
    let cell_sel = Selector::parse("td").expect("selector");
    let mut info = TraderInfo::default();
    let mut owner = false;
    for row in doc.select(&row_sel) {
        let cells: Vec<_> = row.select(&cell_sel).map(text).collect();
        if cells.len() != 2 {
            continue;
        }
        match cells[0].as_str() {
            "reGOLD.de:" => owner = cells[1] == "reGOLD Edelmetallhandel UG (haftungsbeschränkt)",
            "Straße:" => info.street = cells[1].clone(),
            "E-Mail:" => info.email = cells[1].clone(),
            "Ort:" => {
                if let Some((postcode, city)) = cells[1].split_once(' ') {
                    if postcode.len() == 5 && postcode.chars().all(|c| c.is_ascii_digit()) {
                        info.postcode = postcode.to_owned();
                        info.city = city.to_owned();
                    }
                }
            }
            _ => {}
        }
    }
    if !owner
        || info.street.is_empty()
        || info.postcode.is_empty()
        || info.city.is_empty()
        || !info.email.contains('@')
    {
        return Err(error(
            IMPRESSUM_URL,
            "Betreiber-/Kontaktblock unvollständig",
        ));
    }
    let phone_sel =
        Selector::parse("#header .header-kontakt a.number_1[href^='tel:']").expect("selector");
    info.phone = doc.select(&phone_sel).next().map(text).unwrap_or_default();
    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("fixtures/regold-rates.html");

    #[test]
    fn full_live_list_preserves_twenty_grades_and_indicative_units() {
        let (prices, skipped) = parse(FIXTURE).unwrap();
        assert_eq!(prices.len(), 20);
        assert!(skipped.is_empty());
        assert_eq!(prices[0].price, 118.38);
        assert_eq!(prices[2].material, "zahngold");
        assert_eq!(prices[2].variant, "750");
        assert_eq!(prices[5].variant, "600");
        assert_eq!(prices[7].price, 1.68);
        assert_eq!(prices[19].material, "palladium");
        assert_eq!(prices[19].variant, "500");
        assert_eq!(prices[19].price, 15.41);
        let keys: std::collections::HashSet<_> =
            prices.iter().map(|p| (p.material, p.variant)).collect();
        assert_eq!(keys.len(), 20);
        assert!(prices.iter().all(|p| p.unit == "EUR/g"
            && p.currency == "EUR"
            && p.price_kind == "approx"
            && p.confidence == Some(0.8)));
    }

    #[test]
    fn redesigned_empty_or_invalid_rates_fail_loudly() {
        for html in ["", "<h2>Börsen Ankaufkurse</h2><div id='kurse'></div>"] {
            assert!(parse(html).is_err());
        }
        for value in [
            "-118,38",
            "0,00",
            "–",
            "bis zu 118,38",
            "118,38-120,00",
            "118,38,5",
            "118.38",
        ] {
            assert!(parse(&FIXTURE.replace("118,38", value)).is_err(), "{value}");
        }
        assert!(parse(&FIXTURE.replace("&euro; / g", "&euro; / kg")).is_err());
        assert!(parse(&FIXTURE.replace("Gold 585", "Gold 999")).is_err());
    }

    #[test]
    fn unknown_grades_skip_without_guessing_and_outside_rows_are_ignored() {
        let (prices, skipped) = parse(&FIXTURE.replace("Gold 585", "Gold 375")).unwrap();
        assert_eq!(prices.len(), 19);
        assert_eq!(skipped, ["Gold 375"]);
        assert_eq!(grade_for("vergoldet 999"), None);
        let html = format!("{FIXTURE}<div class='k-value'><b class='k-label'>Gold 999</b><span class='k-price'>1,00 € / g</span></div>");
        assert_eq!(parse(&html).unwrap().0.len(), 20);
    }

    #[test]
    fn actual_impressum_shape_decodes_email_and_checks_operator() {
        let html = "<header id='header'><div class='header-kontakt'><a class='number_1' href='tel:03060922615'>030 60 92 26 15</a></div></header><div id='impressum'><div class='ce_text'><table><tr><td>reGOLD.de:</td><td>reGOLD Edelmetallhandel UG (haftungsbeschränkt)</td></tr><tr><td>E-Mail:</td><td>&#109;&#x61;&#105;&#108;&#64;regold.de</td></tr><tr><td>Straße:</td><td>Klosterstrasse 6-7</td></tr><tr><td>Ort:</td><td>13581 Berlin</td></tr></table></div></div>";
        let info = extract_info(html).unwrap();
        assert_eq!(info.email, "mail@regold.de");
        assert_eq!(info.phone, "030 60 92 26 15");
        assert_eq!(info.street, "Klosterstrasse 6-7");
        assert_eq!(info.postcode, "13581");
        assert_eq!(info.city, "Berlin");
        assert!(extract_info(&html.replace("Edelmetallhandel UG", "Andere GmbH")).is_err());
        assert!(extract_info(&html.replace("13581", "1358")).is_err());
        assert!(extract_info("<p>mail@regold.de 13581 Berlin</p>").is_err());
    }

    #[test]
    fn handler_registered_once_and_has_correct_provenance() {
        let handlers = super::super::all();
        let entries: Vec<_> = handlers.iter().filter(|h| h.slug == SLUG).collect();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].url, URL);
    }

    #[tokio::test]
    async fn all_rates_record_to_real_catalog_and_keep_history() {
        use schrott_mcp_store::{InternalDb, PublicDb};
        let dir = crate::test_support::TempDbDir::new("regold");
        let public = PublicDb::open(dir.path()).unwrap();
        let internal = InternalDb::open(dir.path()).unwrap();
        crate::seed_metadata(&public).unwrap();
        let (prices, skipped_labels) = parse(FIXTURE).unwrap();
        let mut outcome = HandlerOutcome {
            prices,
            skipped_labels,
            fetch_url: URL.to_owned(),
            ..Default::default()
        };
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-08T20:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let first = super::super::super::record(&public, &internal, SLUG, &outcome, &now)
            .await
            .unwrap();
        assert_eq!(first.recorded, 20);
        assert!(first.skipped.is_empty());
        assert!(first.canaries.is_empty());
        let trader = public.find_trader_id(SLUG).unwrap().unwrap();
        for price in &outcome.prices {
            let material = public.find_material_id(price.material).unwrap().unwrap();
            let current = public
                .current_price_for(trader, material, price.variant)
                .unwrap()
                .unwrap();
            assert_eq!(current.published_at, None);
        }
        // Only a changed quote appends a history row; unchanged quotes refresh
        // their observation time without duplicating the price-change history.
        outcome.prices[0].price += 0.01;
        let later = now + chrono::Duration::hours(6);
        let second = super::super::super::record(&public, &internal, SLUG, &outcome, &later)
            .await
            .unwrap();
        assert_eq!(second.recorded, 20);
        assert!(second.skipped.is_empty());
        assert!(second.canaries.is_empty());
        let rows = public
            .query_sql(&format!(
                "SELECT count(*) AS n FROM prices WHERE trader_id={trader}"
            ))
            .unwrap();
        assert_eq!(rows.rows[0][0], serde_json::json!(21));
        for (index, price) in outcome.prices.iter().enumerate() {
            let material = public.find_material_id(price.material).unwrap().unwrap();
            let current = public
                .current_price_for(trader, material, price.variant)
                .unwrap()
                .unwrap();
            assert_eq!(current.price, price.price);
            let history = public
                .query_sql(&format!(
                    "SELECT count(*) FROM prices WHERE trader_id={trader} AND material_id={material} AND variant='{}'",
                    price.variant.replace('\'', "''")
                ))
                .unwrap();
            assert_eq!(
                history.rows[0][0],
                serde_json::json!(if index == 0 { 2 } else { 1 })
            );
        }
    }
}
