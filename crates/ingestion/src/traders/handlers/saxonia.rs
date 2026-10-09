//! SAXONIA: current server-rendered metal buttons, not historical chart data.
//! Only explicit Ankauf is money paid by the operator. Verarbeitet and
//! unverarbeitet are NOT purchase grades and are reported as excluded.
//! The page quotes dot-decimal EUR/kg; the catalog uses EUR/g for all four.
use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sn-halsbrucke-09633-saxonia-edelmetalle";
pub const URL: &str = "https://saxonia.de/edelmetallhandel/tageskurse/";
pub const IMPRESSUM_URL: &str = "https://saxonia.de/impressum/";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

fn error(detail: &str) -> IngestError {
    IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.to_owned(),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status_code, html) = fetch_text(client, URL).await?;
    let mut out = parse(&html)?;
    let (_, imprint) = fetch_text(client, IMPRESSUM_URL).await?;
    out.trader_info = extract_info(&imprint)?;
    out.status_code = status_code;
    out.byte_len = html.len();
    out.website_alive = true;
    out.fetch_url = URL.to_owned();
    Ok(out)
}

fn extract_info(html: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(html);
    let sel = Selector::parse(".entry-content").unwrap();
    let body = doc
        .select(&sel)
        .next()
        .map(text)
        .ok_or_else(|| error("Impressumsinhalt fehlt"))?;
    if !body.contains("SAXONIA Edelmetalle GmbH") || !body.contains("HRB 31481") {
        return Err(error("SAXONIA Betreiberidentität im Impressum fehlt"));
    }
    let address = body
        .split("SAXONIA Edelmetalle GmbH")
        .nth(1)
        .unwrap()
        .trim();
    let (street, rest) = address
        .split_once("09633 ")
        .ok_or_else(|| error("Halsbrücker Adresse fehlt"))?;
    if street.trim() != "Erzstraße 9" || !rest.starts_with("Halsbrücke/Sachsen") {
        return Err(error("SAXONIA Standort geändert: manuelle Prüfung nötig"));
    }
    let phone = body
        .split_once("Telefon:")
        .and_then(|(_, t)| t.split_once("Telefax:"))
        .map(|(t, _)| t.trim().to_owned())
        .ok_or_else(|| error("Telefon fehlt"))?;
    let email = body
        .split_once("E-Mail:")
        .and_then(|(_, t)| t.split_whitespace().next())
        .filter(|t| t.ends_with("@saxonia.de"))
        .ok_or_else(|| error("Betreiber-E-Mail fehlt"))?;
    Ok(TraderInfo {
        street: street.trim().to_owned(),
        postcode: "09633".to_owned(),
        city: "Halsbrücke".to_owned(),
        phone,
        email: email.to_owned(),
    })
}

fn text(el: scraper::ElementRef<'_>) -> String {
    el.text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse(html: &str) -> Result<HandlerOutcome, IngestError> {
    let doc = Html::parse_document(html);
    let dates = Selector::parse(".oberschrift").unwrap();
    let mut published_at = None;
    for el in doc.select(&dates) {
        let t = text(el);
        if let Some(date) = t.strip_prefix("Aktueller Tageskurs vom ") {
            let parts: Vec<_> = date.split('.').collect();
            let parsed = (parts.len() == 3)
                .then(|| parse_de_date(parts[0], parts[1], parts[2]))
                .flatten()
                .ok_or_else(|| error("ungültiges Tageskursdatum"))?;
            if published_at.as_ref().is_some_and(|d| d != &parsed) {
                return Err(error("widersprüchliche Tageskursdaten"));
            }
            published_at = Some(parsed);
        }
    }
    if published_at.is_none() {
        return Err(error("Tageskursdatum fehlt"));
    }
    let blocks = Selector::parse(".metal-buttons .metal-flex").unwrap();
    // Button CSS classes are misleading: silver/platinum rows also use palladium.
    let headings = Selector::parse(".chart-metall, .chart-metall2").unwrap();
    let buttons = Selector::parse("button.metal-button").unwrap();
    let mut out = HandlerOutcome {
        published_at,
        ..Default::default()
    };
    for block in doc.select(&blocks) {
        let name = block
            .select(&headings)
            .next()
            .map(text)
            .ok_or_else(|| error("Metallüberschrift fehlt"))?;
        let material = match name.as_str() {
            "GOLD" => Some("gold"),
            "SILBER" => Some("silber"),
            "PLATIN" => Some("platin"),
            "PALLADIUM" => Some("palladium"),
            _ => None,
        };
        for button in block.select(&buttons) {
            let raw = text(button);
            let label = format!("{name} {raw}");
            let Some(value) = raw.strip_prefix("Ankauf:") else {
                out.skipped_labels
                    .push(format!("{label} (kein expliziter Ankaufskurs)"));
                continue;
            };
            let Some(material) = material else {
                out.skipped_labels
                    .push(format!("{label} (kein Katalogmaterial)"));
                continue;
            };
            let value = value
                .trim()
                .strip_suffix("€/kg")
                .ok_or_else(|| error("Ankaufseinheit ist nicht EUR/kg"))?
                .trim();
            // The live widget uses an ungrouped decimal point, NOT German thousands.
            if value.is_empty() || !value.chars().all(|c| c.is_ascii_digit() || c == '.') {
                return Err(error("ungültiger Ankaufskurs"));
            }
            let per_kg: f64 = value.parse().map_err(|_| error("ungültiger Ankaufskurs"))?;
            if !per_kg.is_finite() || per_kg <= 0.0 {
                return Err(error("Ankaufskurs muss positiv sein"));
            }
            let price = per_kg / 1000.0;
            if let Some(previous) = out.prices.iter().find(|p| p.material == material) {
                if previous.price != price {
                    return Err(error("widersprüchliche Ankaufskurse"));
                }
                continue;
            }
            out.prices.push(ScrapedPrice {
                material,
                variant: "",
                price,
                currency: "EUR",
                unit: "EUR/g",
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label,
            });
        }
    }
    if out.prices.len() != 4 {
        return Err(error("vier aktuelle Ankaufskurse erwartet"));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    const PAGE: &str = include_str!("fixtures/saxonia-20261008.html");
    #[test]
    fn current_buying_prices_units_date_and_direction() {
        let out = parse(PAGE).unwrap();
        assert_eq!(
            out.published_at.as_deref(),
            Some("2026-10-08T00:00:00+00:00")
        );
        assert_eq!(out.prices.len(), 4);
        assert_eq!(out.skipped_labels.len(), 6);
        for (p, (material, price)) in out.prices.iter().zip([
            ("gold", 117.3879),
            ("silber", 1.68499),
            ("platin", 46.41538),
            ("palladium", 30.56969),
        ]) {
            assert_eq!(p.material, material);
            assert!((p.price - price).abs() < 1e-10);
            assert_eq!(p.unit, "EUR/g");
            assert_eq!(p.variant, "");
            assert_eq!(p.confidence, Some(1.0));
        }
    }
    #[test]
    fn missing_or_invalid_date_is_not_inferred_from_chart() {
        for date in ["31.02.2026", "", "2026-10-08"] {
            assert!(parse(&PAGE.replace("08.10.2026", date)).is_err());
        }
    }
    #[test]
    fn malformed_or_incomplete_buying_widget_fails_closed() {
        for value in ["0", "-123", "NaN", "1,234.56", "1.2.3"] {
            assert!(parse(&PAGE.replace("117387.9", value)).is_err());
        }
        assert!(parse(&PAGE.replace("€/kg", "€/g")).is_err());
        assert!(parse(&PAGE.replace("Ankauf: 117387.9", "Verkauf: 117387.9")).is_err());
        assert!(parse("<script>GOLD Ankauf: 117387.9 €/kg</script>").is_err());
    }
    #[test]
    fn dynamic_values_and_unknown_metals() {
        let html = PAGE
            .replace("117387.9", "123456.78")
            .replace("08.10.2026", "09.10.2026");
        assert!((parse(&html).unwrap().prices[0].price - 123.45678).abs() < 1e-10);
        let html = format!("{PAGE}<div class='metal-buttons'><div class='metal-flex'><div class='chart-metall'>RHODIUM</div><button class='metal-button'>Ankauf: 100000 €/kg</button></div></div>");
        let out = parse(&html).unwrap();
        assert_eq!(out.prices.len(), 4);
        assert!(out.skipped_labels.last().unwrap().contains("RHODIUM"));
        assert!(parse(&format!("{PAGE}{PAGE}")).is_ok());
        assert!(parse(&format!("{PAGE}{}", PAGE.replace("117387.9", "120000"))).is_err());
    }
    #[test]
    fn operator_guard_and_live_contact_fields() {
        let imprint = "<div class='entry-content'><p>SAXONIA Edelmetalle GmbH<br>Erzstraße 9<br>09633 Halsbrücke/Sachsen</p><p>Registernummer: HRB 31481</p><p>Telefon: +49 3731 20890<br>Telefax: +49 3731 2089 100<br>E-Mail: info@saxonia.de</p></div>";
        let info = extract_info(imprint).unwrap();
        assert_eq!(info.phone, "+49 3731 20890");
        assert_eq!(info.email, "info@saxonia.de");
        assert_eq!(info.street, "Erzstraße 9");
        assert!(extract_info(&imprint.replace("31481", "99999")).is_err());
        assert!(extract_info(&imprint.replace("Erzstraße 9", "Erzstraße 10")).is_err());
    }
}
