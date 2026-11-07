//! pro aurum GmbH: the national Onlineshop price list, static product tables.
//!
//! The price list (`/shop/preisliste-onlineshop/`) ships its product tables
//! server-side: one table per product group, every row
//! `Bezeichnung | Gewicht | Verkaufen | Kaufen`. The page's own footnote
//! fixes the reading: "** Ankaufspreis = netto; Kaufpreis = brutto." — the
//! column **"Verkaufen"** therefore carries the price pro aurum **pays**
//! (net), "Kaufen" is the gross retail price the customer pays. Only
//! "Verkaufen" is recorded; the retail price is not an Ankaufpreis.
//!
//! Only the **bar** tables are read. A bar's stated weight is its pure
//! metal weight, so price / weight is a valid EUR/g. A coin's weight is
//! **not** its fine-metal content (a 900er coin weighs more than its
//! gold), so the coin tables are skipped loudly instead of converted.
//!
//! The base-rate table at the top ("Basiskurse ab …") quotes the metal
//! rates in US dollars **without a unit** — a loud skip, never a guess.
//!
//! The price list is national, so all eleven branches share it. The
//! branches carry their own phone and e-mail (the chain's impressum has
//! only the headquarters contact), so this handler records **prices only**
//! and never writes the chain's contact data into a branch dossier.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, TraderInfo,
};
use crate::IngestError;

/// The eleven pro aurum branches — one national price list.
pub const SLUGS: &[&str] = &[
    "be-charlottenburg-wilmersdorf-pro-aurum-filiale-berlin",
    "bw-stuttgart-pro-aurum-filiale",
    "bw-ulm-pro-aurum-filiale",
    "by-muenchen-riem-pro-aurum-goldhaus",
    "by-muenchen-schwabing-pro-aurum-filiale",
    "he-bad-homburg-pro-aurum-filiale",
    "hh-altstadt-pro-aurum-vertrieb-filiale-hamburg",
    "nw-bielefeld-pro-aurum-filiale",
    "nw-duesseldorf-pro-aurum-filiale",
    "sl-saarbruecken-pro-aurum-filiale",
    "sn-dresden-pro-aurum-filiale",
];

pub const URL: &str = "https://www.proaurum.de/shop/preisliste-onlineshop/";

/// Product groups whose rows are pure metal: heading → catalog material.
/// The coin groups are deliberately absent (see the module doc).
const BAR_TABLES: &[(&str, &str)] = &[
    ("Goldbarren", "gold"),
    ("Silberbarren", "silber"),
    ("Palladiumbarren", "palladium"),
    ("Platinbarren", "platin"),
];

pub fn handler(slug: &'static str) -> Handler {
    Handler {
        slug,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (published_at, rows, skipped_labels) = parse(&html)?;
    let mut prices = Vec::new();
    for row in rows {
        prices.push(super::super::ScrapedPrice {
            material: row.material,
            variant: row.variant.into(),
            price: row.price_per_gram,
            currency: "EUR",
            unit: "EUR/g",
            price_kind: "exact",
            price_min: None,
            price_max: None,
            confidence: Some(1.0),
            label: row.label,
        });
    }
    Ok(HandlerOutcome {
        skip_price_acceptances: false,
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

/// One price row of a bar table.
#[derive(Debug)]
struct Row {
    material: &'static str,
    variant: String,
    /// EUR per gram (piece price divided by the stated weight).
    price_per_gram: f64,
    label: String,
}

/// Parse the bar tables.
///
/// Returns (published_at, rows, skips).
fn parse(html: &str) -> Result<(Option<String>, Vec<Row>, Vec<String>), IngestError> {
    let published_at = find_published_at(html);
    let doc = Html::parse_document(html);
    let heading_sel = Selector::parse("h2").expect("valid selector");
    let table_sel = Selector::parse("table").expect("valid selector");
    let tile_sel = Selector::parse("tr.cs-product-tile").expect("valid selector");
    let name_sel = Selector::parse("a.cs-product-tile__name-link").expect("valid selector");
    let weight_sel = Selector::parse("td.cs-product-tile__weight").expect("valid selector");
    let sell_sel =
        Selector::parse("td.cs-product-tile__transaction--sell").expect("valid selector");
    let amount_sel = Selector::parse("[data-price-amount]").expect("valid selector");

    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for heading in doc.select(&heading_sel) {
        let title = text_of(&heading);
        let Some((_, material)) = BAR_TABLES.iter().find(|(h, _)| *h == title) else {
            // A coin or numismatic group: not convertible to a fine-metal
            // price, so it is skipped loudly rather than guessed.
            if title.to_lowercase().contains("münz") {
                skips.push(format!(
                    "{title} (Münzgruppe: Gewicht ist nicht der Feinmetallgehalt)"
                ));
            }
            continue;
        };
        let Some(table) = next_table(&heading) else {
            skips.push(format!("{title} (Tabelle fehlt)"));
            continue;
        };
        for tile in table.select(&tile_sel) {
            let label = tile
                .select(&name_sel)
                .next()
                .map(|el| text_of(&el))
                .unwrap_or_default();
            if label.is_empty() || label.len() > 120 {
                continue;
            }
            let weight_cell = tile
                .select(&weight_sel)
                .next()
                .map(|el| text_of(&el))
                .unwrap_or_default();
            let sell_cell = match tile.select(&sell_sel).next() {
                Some(cell) => cell,
                None => {
                    skips.push(format!("{label} (keine Ankaufspreis-Spalte)"));
                    continue;
                }
            };
            // The sell column carries the machine-readable amount; its text
            // says "Produkt aktuell nicht verfügbar" when there is none.
            let amount = sell_cell
                .select(&amount_sel)
                .next()
                .and_then(|el| el.value().attr("data-price-amount"))
                .and_then(|raw| raw.trim().parse::<f64>().ok());
            let Some(piece) = amount else {
                skips.push(format!(
                    "{label} (nicht verfügbar: {})",
                    text_of(&sell_cell)
                ));
                continue;
            };
            let Some(grams) = grams_of(&weight_cell) else {
                skips.push(format!("{label} (Gewicht unverständlich: {weight_cell})"));
                continue;
            };
            if piece <= 0.0 {
                skips.push(format!("{label} (Preis nicht positiv)"));
                continue;
            }
            rows.push(Row {
                material,
                variant: label.clone(),
                price_per_gram: piece / grams,
                label,
            });
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Barrenzeile mit Preis gefunden".to_owned(),
        });
    }
    Ok((published_at, rows, skips))
}

/// The text of an element, whitespace collapsed.
fn text_of(el: &ElementRef) -> String {
    el.text()
        .collect::<String>()
        .replace(['\u{a0}'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// The next table after a heading element.
fn next_table<'a>(heading: &ElementRef<'a>) -> Option<ElementRef<'a>> {
    for el in heading.next_siblings().filter_map(ElementRef::wrap) {
        match el.value().name() {
            "table" => return Some(el),
            // The next product group starts: no table for this heading.
            "h2" => return None,
            _ => {}
        }
    }
    None
}

/// The weight of a bar in grams: "1,00 g" → 1.0, "1,00 kg" → 1000.0.
/// An unknown unit is a loud skip, never a default.
fn grams_of(cell: &str) -> Option<f64> {
    let lower = cell.to_lowercase();
    let value = parse_eur(cell)?;
    if value <= 0.0 {
        return None;
    }
    if lower.contains("kg") {
        Some(value * 1000.0)
    } else if lower.contains('g') {
        Some(value)
    } else {
        None
    }
}

/// The page's own timestamp: "Basiskurse ab 2026-11-07 10:36:01".
///
/// Only the date is kept (like the other price pages): the guard needs the
/// calendar day, and a local clock time without a stated zone would be a
/// guess.
fn find_published_at(html: &str) -> Option<String> {
    let idx = html.find("Basiskurse ab ")? + "Basiskurse ab ".len();
    let tail = &html[idx..];
    let head: String = tail.chars().take(32).collect();
    // ISO form: 2026-11-07
    if let Some(date) = head.get(0..10) {
        let mut parts = date.split('-');
        let (y, m, d) = (parts.next()?, parts.next()?, parts.next()?);
        if y.len() == 4 && m.len() == 2 && d.len() == 2 {
            return parse_de_date(d, m, y);
        }
    }
    // German form: 07.11.2026
    if let Some(date) = head.get(0..10) {
        let mut parts = date.split('.');
        let (d, m, y) = (parts.next()?, parts.next()?, parts.next()?);
        if d.len() == 2 && m.len() == 2 && y.len() == 4 {
            return parse_de_date(d, m, y);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The live page of 07.11.2026 (10:50 UTC), trimmed to the base-rate
    /// table and the first four product tiles per bar group.
    const FIXTURE: &str = include_str!("fixtures/proaurum-preisliste.html");

    #[test]
    fn barren_preise_je_gramm() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-11-07T00:00:00+00:00"));
        // four gold bars + four silver bars (the coin group is not read)
        let gold: Vec<&Row> = rows.iter().filter(|r| r.material == "gold").collect();
        let silver: Vec<&Row> = rows.iter().filter(|r| r.material == "silber").collect();
        assert_eq!(gold.len(), 4);
        assert_eq!(silver.len(), 4);
        assert!(rows
            .iter()
            .all(|r| r.material != "platin" || r.price_per_gram > 0.0));
        // "Goldbarren 1 Gramm Umicore": 115,00 € for 1,00 g
        let first = gold
            .iter()
            .find(|r| r.label == "Goldbarren 1 Gramm Umicore")
            .expect("the 1 g bar is parsed");
        assert!((first.price_per_gram - 115.0).abs() < 1e-9);
        assert_eq!(first.variant, "Goldbarren 1 Gramm Umicore");
        // the coin group is a loud skip, not a converted price
        assert!(skips.iter().any(|s| s.contains("Münzgruppe")));
    }

    #[test]
    fn gewicht_und_datum() {
        assert_eq!(grams_of("1,00 g"), Some(1.0));
        assert_eq!(grams_of("31,10 g"), Some(31.10));
        assert_eq!(grams_of("1,00 kg"), Some(1000.0));
        assert_eq!(grams_of("1 Unze"), None);
        assert_eq!(grams_of("pro Stück"), None);
        assert_eq!(
            find_published_at("Basiskurse ab 2026-11-07 10:36:01"),
            Some("2026-11-07T00:00:00+00:00".to_owned())
        );
        assert_eq!(find_published_at("ohne Zeitangabe"), None);
    }

    #[test]
    fn ohne_preise_ist_ein_fehler() {
        let err = parse("<h2>Goldbarren</h2>").expect_err("no rows");
        assert!(format!("{err:?}").contains("Barrenzeile"));
    }
}
