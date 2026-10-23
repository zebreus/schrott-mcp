//! Wahl & Co. Inh. Thomas Madeiski (Zella-Mehlis): per-grade guide prices
//! come from the CMS Sortenpreisliste and are `approx` (the page calls them
//! unverbindliche, tagesabhängige Richtwerte); the overview contributes
//! explicitly ranged categories as `range`. The source publishes a machine-
//! readable price timestamp. Assortment acceptances come from metal cards.
//! `0,00`/empty cells fail closed, never as price 0. Composite or battery
//! categories that do not fit the catalog stay unmapped.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "th-zella-mehlis-wahl-co-inh-thomas-madeiski";
/// Bespoke, live-verified price URL (homepage carries the price FAQs).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://wahlundco.de/";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://wahlundco.de/impressum.html";

/// Both current price tables state their values in euros per kilogram.
const UNIT: &str = "EUR/kg";
const DETAIL_ROW_COUNT: usize = 31;
const OVERVIEW_RANGE_COUNT: usize = 14;
pub(crate) const SNAPSHOT_PRICE_COUNT: usize = 40;
pub(crate) const SNAPSHOT_OWNER: &str = "wahl-cms-price-v1";

/// Stable CMS row identity paired with its currently reviewed visible label.
/// A row ID alone is not enough to attribute a changed label to the same
/// catalog grade.
const DETAIL_ROW_IDENTITIES: [(&str, &str); DETAIL_ROW_COUNT] = [
    ("kupfer-draht-millberry", "Kupfer-Draht „Millberry“"),
    ("kupfer-draht-berry", "Kupfer-Draht „Berry“"),
    ("kupfer-draht-kanal", "Kupfer-Draht „Kanal“"),
    ("kupfer-neublech", "Kupfer-Neublech"),
    ("schwerkupfer-lotfrei", "Schwerkupfer, lotfrei"),
    ("leichtkupfer", "Leichtkupfer"),
    (
        "messing-ohne-hulsen-schlauche",
        "Messing ohne Hülsen/Schläuche",
    ),
    ("messing-spane-gemischt", "Messing-Späne, gemischt"),
    ("messing-hulsen-sauber", "Messing-Hülsen, sauber"),
    ("ms-erodierdraht", "MS-Erodierdraht"),
    ("messing-58-schrott", "Messing 58 Schrott"),
    ("messing-58-spane", "Messing 58 Späne"),
    ("messing-63-blech-neu-blank", "Messing 63 Blech, neu, blank"),
    ("kupfer-messing-kuhler", "Kupfer-Messing-Kühler"),
    ("rotguss-schrott", "Rotguss-Schrott"),
    ("rotguss-spane", "Rotguss-Späne"),
    ("walzbronze", "Walzbronze"),
    ("alu-profile-blank", "Alu-Profile, blank"),
    ("alu-profile-gestegt", "Alu-Profile, gestegt"),
    ("alu-profile-lackiert", "Alu-Profile, lackiert"),
    ("ral-draht-eisenfrei", "RAL-Draht, eisenfrei"),
    ("alu-offset-ohne-papier", "Alu-Offset ohne Papier"),
    (
        "almg-blech-teilweise-foliert",
        "AlMg-Blech, teilweise foliert",
    ),
    (
        "alu-guss-felgen-ohne-anhaftung",
        "Alu-Guss-Felgen ohne Anhaftung",
    ),
    ("alu-geschirr-ohne-fe", "Alu-Geschirr ohne Fe"),
    ("alu-geschirr-mit-fe", "Alu-Geschirr, mit Fe"),
    ("alu-guss-ohne-fe", "Alu-Guss ohne Fe"),
    ("alu-guss-mit-fe", "Alu-Guss, mit Fe"),
    ("zinkblech-neu", "Zinkblech, neu"),
    ("zinkblech-alt", "Zinkblech, alt"),
    ("altblei", "Altblei"),
];

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
    let (rows, skipped_labels) = parse_prices(&html)?;
    let range_rows = parse_ranges(&html)?;
    let published_at = parse_published_at(&html)?;
    let (prices, mut skipped_labels) = map_price_snapshot(rows, skipped_labels, range_rows)?;

    // Assortment from the metal-card headings (bonus, never fails the step;
    // unmapped cards skip loudly via skipped_labels).
    let mut acceptances = Vec::new();
    for name in parse_cards(&html) {
        match grade_for_card(&name) {
            Some(materials) => {
                for (material, conditions) in materials {
                    acceptances.push(ScrapedAcceptance {
                        material,
                        conditions: conditions.to_owned(),
                        label: name.clone(),
                    });
                }
            }
            None => skipped_labels.push(format!("Karte: {name}")),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    let outcome = HandlerOutcome {
        skip_price_acceptances: false,
        prices,
        acceptances,
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: Some(published_at),
    };
    validate_snapshot(&outcome)?;
    Ok(outcome)
}

/// Map the two parsed price tables to catalog identities and enforce the
/// complete expected snapshot. Kept pure so source-label drift can be tested
/// through the same mapping/count guard that production uses.
fn map_price_snapshot(
    rows: Vec<(String, f64)>,
    mut skipped_labels: Vec<String>,
    range_rows: Vec<(String, f64, f64)>,
) -> Result<(Vec<ScrapedPrice>, Vec<String>), IngestError> {
    let mut prices = Vec::with_capacity(rows.len() + range_rows.len());
    for (label, price) in rows {
        if !price.is_finite() || price <= 0.0 {
            return Err(parse_error(&format!("ungültiger Sortenpreis für {label}")));
        }
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant: variant.into(),
                price,
                currency: "EUR",
                unit: UNIT,
                price_kind: "approx",
                price_min: None,
                price_max: None,
                confidence: Some(0.5),
                label,
            }),
            None if matches!(
                label.to_lowercase().as_str(),
                "kupfer-messing-kühler" | "kupfer-messing-kuehler" | "blei-akkus"
            ) =>
            {
                skipped_labels.push(format!("{label} (keine eindeutige Katalogsorte)"));
            }
            None => return Err(parse_error(&format!("unbekannte bepreiste Sorte: {label}"))),
        }
    }
    // The overview uses explicitly stated low/high bounds. Keep a true
    // range as a range; skip unlisted/composite materials rather than
    // attributing their bound to a similar-looking catalog slug.
    for (label, min, max) in range_rows {
        if !min.is_finite() || !max.is_finite() || min <= 0.0 || max <= 0.0 {
            return Err(parse_error(&format!("ungültiger Preisbereich für {label}")));
        }
        match grade_for_range(&label) {
            Some(material) => prices.push(ScrapedPrice {
                material,
                variant: "".into(),
                price: (min + max) / 2.0,
                currency: "EUR",
                unit: UNIT,
                price_kind: "range",
                price_min: Some(min),
                price_max: Some(max),
                confidence: Some(0.5),
                label,
            }),
            None if matches!(
                label.to_lowercase().as_str(),
                "rotguss (rg)" | "zinn (sn)" | "blei-akkus" | "kabel & e-schrott"
            ) =>
            {
                skipped_labels.push(format!("{label} (Bereich nicht eindeutig zuordenbar)"));
            }
            None => return Err(parse_error(&format!("unbekannter Preisbereich: {label}"))),
        }
    }
    if prices.len() != SNAPSHOT_PRICE_COUNT {
        return Err(parse_error(&format!(
            "unvollständiger Preis-Snapshot: {} statt {SNAPSHOT_PRICE_COUNT} Katalogpreise",
            prices.len()
        )));
    }
    Ok((prices, skipped_labels))
}

pub(crate) fn validate_snapshot(outcome: &HandlerOutcome) -> Result<(), IngestError> {
    let timestamp_ok = outcome
        .published_at
        .as_deref()
        .is_some_and(|value| chrono::DateTime::parse_from_rfc3339(value).is_ok());
    let mut keys = std::collections::HashSet::new();
    let rows_ok = outcome.prices.iter().all(|price| {
        let range_ok = match (price.price_kind, price.price_min, price.price_max) {
            ("approx", None, None) => true,
            ("range", Some(min), Some(max)) => {
                min.is_finite()
                    && max.is_finite()
                    && min > 0.0
                    && min <= max
                    && price.price == (min + max) / 2.0
            }
            _ => false,
        };
        price.currency == "EUR"
            && price.unit == UNIT
            && price.confidence == Some(0.5)
            && price.price.is_finite()
            && price.price > 0.0
            && range_ok
            && keys.insert((price.material, price.variant.to_string()))
    });
    if outcome.fetch_url != URL
        || !(200..300).contains(&outcome.status_code)
        || outcome.skip_price_acceptances
        || outcome.prices.len() != SNAPSHOT_PRICE_COUNT
        || !timestamp_ok
        || !rows_ok
    {
        return Err(parse_error(
            "unvollständiger oder ungültiger Wahl-Preissnapshot",
        ));
    }
    Ok(())
}

/// Explicit label → (material, variant). Continuation labels carry their
/// group base ("Alu-Profile, blank: / gestegt:"), so specific-before-generic
/// arms resolve them: "lackiert" beats "gestegt" beats "blank", "mit Fe"
/// beats "ohne Fe" (the base always contains the first grade too).
/// Anything unlisted (composites, off-catalog) is skipped.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if (l.contains("blei") && l.contains("akku")) || l.contains("batterie") {
        // Batteries are lead-bearing composites, not clean/soft lead.
        None
    } else if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("berry") {
        Some(("kupfer-berry", ""))
    } else if l.contains("kanal") {
        Some(("kupfer-gemischt", "kanal"))
    } else if l.contains("neublech") {
        Some(("kupfer-gemischt", "neublech"))
    } else if l.contains("schwerkupfer") {
        Some(("kupfer-gemischt", "schwerkupfer-lotfrei"))
    } else if l.contains("leichtkupfer") {
        Some(("kupfer-gemischt", "leichtkupfer"))
    } else if l.contains("kühler") || l.contains("kuehler") {
        // "Kupfer-Messing-Kühler": composite, `kuehler-verbund` was never
        // crammed into the catalog → loud skip, never a wrong row.
        None
    } else if l.contains("58") && l.contains("späne") {
        Some(("messing", "ms58-spaene"))
    } else if l.contains("58") {
        Some(("messing", "ms58-schrott"))
    } else if l.contains("63") {
        Some(("messing", "ms63-blech"))
    } else if l.contains("ohne hülsen") || l.contains("ohne-hülsen") {
        // "Messing ohne Hülsen/Schläuche" — vor der hülsen-Arm, die sonst
        // auch dieses Label fängt.
        Some(("messing", "ohne-huelsen"))
    } else if l.contains("hülsen") || l.contains("huelsen") {
        Some(("messing", "huelsen"))
    } else if l.contains("erodierdraht") {
        Some(("messing", "erodierdraht"))
    } else if l.contains("späne") && l.contains("messing") {
        Some(("messing", "spaene-gemischt"))
    } else if l.contains("messing") {
        Some(("messing", "ohne-huelsen"))
    } else if l.contains("walzbronze") {
        Some(("bronze-rotguss", "walzbronze"))
    } else if l.contains("rotguss") && l.contains("späne") {
        Some(("bronze-rotguss", "spaene"))
    } else if l.contains("rotguss") {
        Some(("bronze-rotguss", ""))
    } else if l.contains("ral-draht") {
        Some(("aluminium-gemischt", "ral-draht"))
    } else if l.contains("profile") && l.contains("lackiert") {
        Some(("aluminium-profile", "lackiert"))
    } else if l.contains("profile") && l.contains("gestegt") {
        Some(("aluminium-profile", "gestegt"))
    } else if l.contains("profile") {
        Some(("aluminium-profile", ""))
    } else if l.contains("offset") {
        Some(("aluminium-blech", "offset"))
    } else if l.contains("almg") {
        Some(("aluminium-blech", "almg"))
    } else if l.contains("felgen") {
        Some(("aluminium-guss", "felgen"))
    } else if l.contains("geschirr") && l.contains("mit fe") {
        Some(("aluminium-gemischt", "geschirr-mit-fe"))
    } else if l.contains("geschirr") {
        Some(("aluminium-gemischt", "geschirr-ohne-fe"))
    } else if l.contains("guss") && l.contains("mit fe") {
        Some(("aluminium-guss", "mit-fe"))
    } else if l.contains("guss") {
        Some(("aluminium-guss", "ohne-fe"))
    } else if l.contains("zink") && l.contains("alt") {
        Some(("zink", "alt"))
    } else if l.contains("zink") {
        Some(("zink", "neu"))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else {
        None
    }
}

/// Metal-card heading → acceptances. Edelstahl fans out to V2A+V4A (the
/// card reads "Edelstahl (V2A/V4A)"). Batteries and the Kabel&E-Schrott
/// composite have no safe catalog fit → loud skip.
fn grade_for_card(name: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = name.to_lowercase();
    let l = l.as_str();
    if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("armaturenmessing") || l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("rotguss") {
        Some(vec![("bronze-rotguss", "")])
    } else if l.contains("aluminium") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("edelstahl") {
        Some(vec![("edelstahl-v2a", ""), ("edelstahl-v4a", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("zinn") {
        Some(vec![("zinn", "")])
    } else if l.contains("wolfram") {
        Some(vec![("wolfram", "")])
    } else if l.contains("motor") {
        Some(vec![("elektromotoren", "")])
    } else if l.contains("hss") || l.contains("hochleistungsstahl") {
        Some(vec![("hss-werkzeuge", "")])
    } else if l.contains("hartmetall") || l.contains("vhm") {
        Some(vec![("hartmetall", "")])
    } else if l.contains("mischschrott") {
        Some(vec![("mischschrott", "")])
    } else {
        // "Blei-Akkus" (batteries ≠ Weichblei) and
        // "Kabel & E-Schrott" (composite) have no safe catalog mapping.
        None
    }
}

/// Per-grade Richtwerte from the CMS Sortenpreisliste. These are explicitly
/// nonbinding on the page and are recorded as approximate prices.
fn parse_prices(html: &str) -> Result<(Vec<(String, f64)>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let table_sel = Selector::parse("table.cms-price-table").expect("valid selector");
    let tables: Vec<_> = doc.select(&table_sel).collect();
    if tables.len() != 1 {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Sortenpreistabelle fehlt oder mehrfach vorhanden".to_owned(),
        });
    }
    let caption_sel = Selector::parse("caption").expect("valid selector");
    let caption = tables[0]
        .select(&caption_sel)
        .next()
        .map(|el| clean(&el.inner_html()))
        .unwrap_or_default();
    if caption != "Sortenpreise in Euro pro Kilogramm" {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: format!("unerwartete Sortenpreistabelle: {caption:?}"),
        });
    }
    let p = Selector::parse("tbody tr[data-price-id]").expect("valid selector");
    let label_sel = Selector::parse("th[scope=row]").expect("valid selector");
    let value_sel = Selector::parse("td").expect("valid selector");
    let price_rows: Vec<_> = tables[0].select(&p).collect();
    if price_rows.len() != DETAIL_ROW_COUNT {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: format!(
                "unerwartete Anzahl Sortenpreiszeilen: {} statt {DETAIL_ROW_COUNT}",
                price_rows.len()
            ),
        });
    }
    let mut rows: Vec<(String, f64)> = Vec::new();
    let mut ids = std::collections::HashSet::new();
    for row in price_rows {
        let id = row.value().attr("data-price-id").unwrap_or_default();
        if id.is_empty() || !ids.insert(id.to_owned()) {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: "Sortenpreistabelle enthält eine fehlende oder doppelte Preis-ID"
                    .to_owned(),
            });
        }
        let label = row
            .select(&label_sel)
            .next()
            .map(|el| clean(&el.inner_html()))
            .unwrap_or_default();
        if label.is_empty() {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: "Sortenpreiszeile ohne Bezeichnung".to_owned(),
            });
        }
        let expected_label = DETAIL_ROW_IDENTITIES
            .iter()
            .find_map(|(expected_id, expected_label)| {
                (*expected_id == id).then_some(*expected_label)
            })
            .ok_or_else(|| IngestError::Parse {
                url: URL.to_owned(),
                detail: format!("unbekannte Sortenpreis-ID: {id}"),
            })?;
        if label != expected_label {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: format!(
                    "Sortenpreis-ID {id} gehört zu {expected_label:?}, nicht {label:?}"
                ),
            });
        }
        let values: Vec<_> = row.select(&value_sel).collect();
        if values.len() != 1 {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: format!("{label}: unerwartete Anzahl Preiszellen"),
            });
        }
        let raw = clean(&values[0].inner_html());
        match parse_eur(&raw) {
            Some(price) => rows.push((label, price)),
            None => {
                return Err(IngestError::Parse {
                    url: URL.to_owned(),
                    detail: format!("{label}: Preis unverständlich: {raw}"),
                });
            }
        }
    }
    if rows.len() != DETAIL_ROW_COUNT {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: format!("unvollständige Sortenpreistabelle: {} Zeilen", rows.len()),
        });
    }
    Ok((rows, Vec::new()))
}

/// Explicit ranges in the price overview. Single-value rows (e.g. Kupfer
/// Kanal) are handled by the more specific per-grade table instead.
fn parse_ranges(html: &str) -> Result<Vec<(String, f64, f64)>, IngestError> {
    let doc = Html::parse_document(html);
    let table_sel = Selector::parse("table.finder-table").expect("valid selector");
    let tables: Vec<_> = doc.select(&table_sel).collect();
    if tables.len() != 1 {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisübersicht fehlt oder mehrfach vorhanden".to_owned(),
        });
    }
    let caption_sel = Selector::parse("caption").expect("valid selector");
    let caption = tables[0]
        .select(&caption_sel)
        .next()
        .map(|el| clean(&el.inner_html()))
        .unwrap_or_default();
    if caption != "Materialpreise im Überblick, Euro pro Kilogramm" {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: format!("unerwartete Preisübersicht: {caption:?}"),
        });
    }
    let row_sel = Selector::parse("tbody tr").expect("valid selector");
    let label_sel = Selector::parse("th[scope=row]").expect("valid selector");
    let value_sel = Selector::parse("td").expect("valid selector");
    let mut ranges = Vec::new();
    for row in tables[0].select(&row_sel) {
        let label = row
            .select(&label_sel)
            .next()
            .map(|el| clean(&el.inner_html()))
            .unwrap_or_default();
        let Some(value) = row.select(&value_sel).next() else {
            continue;
        };
        let raw = clean(&value.inner_html());
        if !raw.contains('–') {
            continue;
        }
        let Some((min, max)) = parse_range(&raw) else {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: format!("ungültiger Preisbereich für {label}: {raw}"),
            });
        };
        ranges.push((label, min, max));
    }
    if ranges.len() != OVERVIEW_RANGE_COUNT {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: format!(
                "unerwartete Anzahl Preisbereiche: {} statt {OVERVIEW_RANGE_COUNT}",
                ranges.len()
            ),
        });
    }
    Ok(ranges)
}

fn parse_range(raw: &str) -> Option<(f64, f64)> {
    let (min, max) = raw.split_once('–')?;
    let min = parse_eur(min)?;
    let max = parse_eur(max)?;
    (min <= max).then_some((min, max))
}

/// The generic Rotguss range would collide with its separately priced
/// Rotguss-Schrott grade. The generic Zinn range includes solder and
/// tableware, outside the high-purity catalog entry. Battery and cable/e-scrap
/// composites likewise have no safe generic catalog mapping.
fn grade_for_range(label: &str) -> Option<&'static str> {
    match label.trim().to_lowercase().as_str() {
        "kupfer (cu)" => Some("kupfer-gemischt"),
        "armaturenmessing" => Some("messing"),
        "aluminium (al)" => Some("aluminium-gemischt"),
        "edelstahl (v2a/v4a)" => Some("edelstahl-gemischt"),
        "zinkblech" => Some("zink"),
        "zinn (sn)" => None,
        "elektro-motoren" => Some("elektromotoren"),
        "hochleistungsstahl (hss)" => Some("hss-werkzeuge"),
        "hartmetall (vhm)" => Some("hartmetall"),
        "wolfram (w)" => Some("wolfram"),
        "mischschrott & eisen" => Some("mischschrott"),
        _ => None,
    }
}

/// Parse the machine-readable timestamp shown beside the current price list.
fn parse_published_at(html: &str) -> Result<String, IngestError> {
    let doc = Html::parse_document(html);
    let date_sel = Selector::parse("time#price-update-date").expect("valid selector");
    let dates: Vec<_> = doc.select(&date_sel).collect();
    if dates.len() != 1 {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisstand fehlt oder mehrfach vorhanden".to_owned(),
        });
    }
    let raw = dates[0].value().attr("datetime").unwrap_or_default();
    let date = chrono::DateTime::parse_from_rfc3339(raw).map_err(|_| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisstand ist kein gültiger RFC3339-Zeitstempel".to_owned(),
    })?;
    Ok(date.to_rfc3339())
}

/// Metal-card headings (`<div class="metal-card">…<h3>…`) as assortment
/// proof. Bonus only: an empty card list is not an error.
fn parse_cards(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let sel = Selector::parse(".metal-card h3").expect("valid selector");
    doc.select(&sel)
        .map(|el| clean(&el.inner_html()))
        .filter(|t| !t.is_empty())
        .collect()
}

/// Bespoke contact extraction for THIS impressum only: address lines come
/// from the `<br>` lines of the provider `<p>` or `<address>` block; phone
/// and mail come from labeled lines in those blocks. The current provider
/// block uses `<address>` and spells out "Straße" instead of "Str.".
/// Whole-doc `text()` glues neighbouring nodes without spaces
/// ("e.K.Heinrich…", "47b98544") — so `<br>` lines it is, first match wins
/// (authority phone numbers further down must not overwrite the trader's).
/// Missing anchors → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p_sel = Selector::parse("p, address").expect("valid selector");
    let (mut street, mut postcode, mut city, mut phone, mut email) = (
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    );
    for el in doc.select(&p_sel) {
        let inner = el
            .inner_html()
            .replace("<br/>", "<br>")
            .replace("<br />", "<br>");
        let lines: Vec<String> = inner
            .split("<br>")
            .map(clean)
            .filter(|l| !l.is_empty())
            .collect();
        if street.is_empty() {
            if let Some(line) = lines
                .iter()
                .find(|l| l.to_lowercase().starts_with("heinrich-ehrhardt-str"))
            {
                street = line.clone();
            }
            // PLZ/Ort stehen auf der Folgezeile ("98544 Zella-Mehlis").
            // First match wins (Behördenadressen weiter unten zählen nicht).
            if postcode.is_empty() {
                for line in &lines {
                    let toks: Vec<&str> = line.split_whitespace().collect();
                    for (k, t) in toks.iter().enumerate() {
                        if t.len() == 5 && t.chars().all(|c| c.is_ascii_digit()) {
                            if let Some(ci) = toks.get(k + 1) {
                                if ci.chars().next().is_some_and(|c| c.is_uppercase()) {
                                    postcode = (*t).to_owned();
                                    city = (*ci).to_owned();
                                }
                            }
                        }
                    }
                }
            }
        }
        if phone.is_empty() {
            if let Some(line) = lines.iter().find(|l| l.starts_with("Telefon:")) {
                phone = line["Telefon:".len()..].trim().to_owned();
            }
        }
        if email.is_empty() {
            if let Some(line) = lines.iter().find(|l| l.starts_with("E-Mail:")) {
                email = line["E-Mail:".len()..]
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .to_owned();
            }
        }
    }
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo {
        street,
        postcode,
        city,
        phone,
        email,
    })
}

/// Strip tags from an HTML snippet, collapse whitespace.
fn clean(raw: &str) -> String {
    Html::parse_fragment(raw)
        .root_element()
        .text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_owned()
}

fn parse_error(detail: &str) -> IngestError {
    IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        extract_info, grade_for, grade_for_card, grade_for_range, map_price_snapshot, parse_cards,
        parse_prices, parse_published_at, parse_range, parse_ranges, SNAPSHOT_PRICE_COUNT,
    };

    /// Historical FAQ layout, retained only to assert that stale markup is
    /// rejected instead of yielding old price semantics.
    const FIXTURE_TABLE: &str = "<p><strong>Tagesaktuelle Sortenpreise</strong> in \u{20ac}/kg (Richtwerte):</p>\
        <p><strong>Kupfer:</strong><br>\
        \u{2022} Kupfer-Draht \u{201e}Millberry\": <strong>10,10 \u{20ac}</strong><br>\
        \u{2022} Kupfer-Draht \u{201e}Berry\": <strong>9,70 \u{20ac}</strong><br>\
        \u{2022} Kupfer-Draht \u{201e}Kanal\": <strong>9,20 \u{20ac}</strong><br>\
        \u{2022} Kupfer-Neublech: <strong>9,95 \u{20ac}</strong><br>\
        \u{2022} Schwerkupfer, lotfrei: <strong>9,55 \u{20ac}</strong><br>\
        \u{2022} Leichtkupfer: <strong>9,20 \u{20ac}</strong></p>\
        <p><strong>Messing:</strong><br>\
        \u{2022} Messing ohne H\u{00fc}lsen/Schl\u{00e4}uche: <strong>5,45 \u{20ac}</strong><br>\
        \u{2022} Messing-Sp\u{00e4}ne, gemischt: <strong>5,75 \u{20ac}</strong><br>\
        \u{2022} Messing-H\u{00fc}lsen, sauber: <strong>6,50 \u{20ac}</strong><br>\
        \u{2022} MS-Erodierdraht: <strong>6,20 \u{20ac}</strong><br>\
        \u{2022} Messing 58 Schrott: <strong>6,15 \u{20ac}</strong> / Sp\u{00e4}ne: <strong>5,80 \u{20ac}</strong><br>\
        \u{2022} Messing 63 Blech, neu, blank: <strong>6,50 \u{20ac}</strong><br>\
        \u{2022} Kupfer-Messing-K\u{00fc}hler: <strong>5,80 \u{20ac}</strong></p>\
        <p><strong>Rotguss / Bronze:</strong><br>\
        \u{2022} Rotguss-Schrott: <strong>8,85 \u{20ac}</strong> / Sp\u{00e4}ne: <strong>8,60 \u{20ac}</strong><br>\
        \u{2022} Walzbronze: <strong>10,95 \u{20ac}</strong></p>\
        <p><strong>Aluminium:</strong><br>\
        \u{2022} Alu-Profile, blank: <strong>2,69 \u{20ac}</strong> / gestegt: <strong>1,80 \u{20ac}</strong> / lackiert: <strong>2,39 \u{20ac}</strong><br>\
        \u{2022} RAL-Draht, eisenfrei: <strong>2,51 \u{20ac}</strong><br>\
        \u{2022} Alu-Offset ohne Papier: <strong>2,45 \u{20ac}</strong><br>\
        \u{2022} AlMg-Blech, teilweise foliert: <strong>2,30 \u{20ac}</strong><br>\
        \u{2022} Alu-Guss-Felgen ohne Anhaftung: <strong>2,20 \u{20ac}</strong><br>\
        \u{2022} Alu-Geschirr ohne Fe: <strong>1,65 \u{20ac}</strong> / mit Fe: <strong>1,20 \u{20ac}</strong><br>\
        \u{2022} Alu-Guss ohne Fe: <strong>1,62 \u{20ac}</strong> / mit Fe: <strong>0,95 \u{20ac}</strong></p>\
        <p><strong>Zink / Blei:</strong><br>\
        \u{2022} Zinkblech, neu: <strong>1,97 \u{20ac}</strong> / alt: <strong>1,77 \u{20ac}</strong><br>\
        \u{2022} Altblei: <strong>0,93 \u{20ac}</strong></p>\
        <p><strong>Hinweis:</strong> Preise sind Richtwerte, tagesabh\u{00e4}ngig.";

    const FIXTURE_UPTO: &str = "<p><strong>Unsere Ankaufspreise pro kg</strong> (Richtwerte):<br>\
        \u{1f947} <strong>Kupferpreis</strong>: oft \u{00fc}ber 10 \u{20ac}/kg<br>\
        \u{1f948} <strong>Messingpreis</strong>: oft \u{00fc}ber 5 \u{20ac}/kg<br>\
        \u{1f949} <strong>Kabelpreis</strong>: bis zu 5 \u{20ac}/kg \u{2013} je nach Kupferanteil<br>\
        4. <strong>Edelstahlpreis</strong>: bis zu 1,20 \u{20ac}/kg \u{2013} Sp\u{00fc}len, T\u{00f6}pfe<br>\
        5. <strong>Alupreis</strong>: oft \u{00fc}ber 2,50 \u{20ac}/kg</p>";

    const FIXTURE_CARDS: &str = "<div class=\"metal-card\"><div class=\"metal-content\">\
        <h3 itemprop=\"name\">Kupfer (Cu)</h3></div></div>\
        <div class=\"metal-card\"><div class=\"metal-content\">\
        <h3 itemprop=\"name\">Edelstahl (V2A/V4A)</h3></div></div>\
        <div class=\"metal-card\"><div class=\"metal-content\">\
        <h3 itemprop=\"name\">Blei-Akkus</h3></div></div>\
        <div class=\"metal-card\"><div class=\"metal-content\">\
        <h3 itemprop=\"name\">Wolfram (W)</h3></div></div>\
        <div class=\"metal-card\"><div class=\"metal-content\">\
        <h3 itemprop=\"name\">Kabel & E-Schrott</h3></div></div>";

    const FIXTURE_CMS_TABLE: &str = r#"
        <table class="cms-price-table">
            <caption>Sortenpreise in Euro pro Kilogramm</caption>
            <thead><tr><th>Material / Sorte</th><th>Richtpreis €/kg</th></tr></thead>
            <tbody>
                <tr class="cms-price-group"><th colspan="2">Kupfer</th></tr>
                <tr data-price-id="kupfer-draht-millberry"><th scope="row">Kupfer-Draht „Millberry“</th><td>10,50 €</td></tr>
                <tr data-price-id="zinkblech-alt"><th scope="row">Zinkblech, alt</th><td>1,77 €</td></tr>
            </tbody>
        </table>
        <time id="price-update-date" datetime="2026-10-21T14:44:01Z">21.10.2026, 16:44 Uhr</time>
    "#;

    /// Live CMS table snapshot, including its data-row structure and labels.
    const FIXTURE_LIVE_TABLE: &str = r#"
        <table class="cms-price-table">
          <caption>Sortenpreise in Euro pro Kilogramm</caption>
          <thead><tr><th scope="col">Material / Sorte</th><th scope="col">Richtpreis €/kg</th></tr></thead>
          <tbody>
            <tr class="cms-price-group"><th colspan="2" scope="colgroup">Kupfer</th></tr>
            <tr data-price-id="kupfer-draht-millberry"><th scope="row">Kupfer-Draht „Millberry“</th><td>10,50 €</td></tr>
            <tr data-price-id="kupfer-draht-berry"><th scope="row">Kupfer-Draht „Berry“</th><td>9,70 €</td></tr>
            <tr data-price-id="kupfer-draht-kanal"><th scope="row">Kupfer-Draht „Kanal“</th><td>9,20 €</td></tr>
            <tr data-price-id="kupfer-neublech"><th scope="row">Kupfer-Neublech</th><td>9,95 €</td></tr>
            <tr data-price-id="schwerkupfer-lotfrei"><th scope="row">Schwerkupfer, lotfrei</th><td>9,55 €</td></tr>
            <tr data-price-id="leichtkupfer"><th scope="row">Leichtkupfer</th><td>9,20 €</td></tr>
            <tr class="cms-price-group"><th colspan="2" scope="colgroup">Messing</th></tr>
            <tr data-price-id="messing-ohne-hulsen-schlauche"><th scope="row">Messing ohne Hülsen/Schläuche</th><td>5,45 €</td></tr>
            <tr data-price-id="messing-spane-gemischt"><th scope="row">Messing-Späne, gemischt</th><td>5,75 €</td></tr>
            <tr data-price-id="messing-hulsen-sauber"><th scope="row">Messing-Hülsen, sauber</th><td>6,50 €</td></tr>
            <tr data-price-id="ms-erodierdraht"><th scope="row">MS-Erodierdraht</th><td>6,20 €</td></tr>
            <tr data-price-id="messing-58-schrott"><th scope="row">Messing 58 Schrott</th><td>6,15 €</td></tr>
            <tr data-price-id="messing-58-spane"><th scope="row">Messing 58 Späne</th><td>5,80 €</td></tr>
            <tr data-price-id="messing-63-blech-neu-blank"><th scope="row">Messing 63 Blech, neu, blank</th><td>6,50 €</td></tr>
            <tr data-price-id="kupfer-messing-kuhler"><th scope="row">Kupfer-Messing-Kühler</th><td>5,80 €</td></tr>
            <tr class="cms-price-group"><th colspan="2" scope="colgroup">Rotguss / Bronze</th></tr>
            <tr data-price-id="rotguss-schrott"><th scope="row">Rotguss-Schrott</th><td>8,85 €</td></tr>
            <tr data-price-id="rotguss-spane"><th scope="row">Rotguss-Späne</th><td>8,60 €</td></tr>
            <tr data-price-id="walzbronze"><th scope="row">Walzbronze</th><td>10,95 €</td></tr>
            <tr class="cms-price-group"><th colspan="2" scope="colgroup">Aluminium</th></tr>
            <tr data-price-id="alu-profile-blank"><th scope="row">Alu-Profile, blank</th><td>2,69 €</td></tr>
            <tr data-price-id="alu-profile-gestegt"><th scope="row">Alu-Profile, gestegt</th><td>1,80 €</td></tr>
            <tr data-price-id="alu-profile-lackiert"><th scope="row">Alu-Profile, lackiert</th><td>2,39 €</td></tr>
            <tr data-price-id="ral-draht-eisenfrei"><th scope="row">RAL-Draht, eisenfrei</th><td>2,51 €</td></tr>
            <tr data-price-id="alu-offset-ohne-papier"><th scope="row">Alu-Offset ohne Papier</th><td>2,45 €</td></tr>
            <tr data-price-id="almg-blech-teilweise-foliert"><th scope="row">AlMg-Blech, teilweise foliert</th><td>2,30 €</td></tr>
            <tr data-price-id="alu-guss-felgen-ohne-anhaftung"><th scope="row">Alu-Guss-Felgen ohne Anhaftung</th><td>2,20 €</td></tr>
            <tr data-price-id="alu-geschirr-ohne-fe"><th scope="row">Alu-Geschirr ohne Fe</th><td>1,65 €</td></tr>
            <tr data-price-id="alu-geschirr-mit-fe"><th scope="row">Alu-Geschirr, mit Fe</th><td>1,20 €</td></tr>
            <tr data-price-id="alu-guss-ohne-fe"><th scope="row">Alu-Guss ohne Fe</th><td>1,62 €</td></tr>
            <tr data-price-id="alu-guss-mit-fe"><th scope="row">Alu-Guss, mit Fe</th><td>0,95 €</td></tr>
            <tr class="cms-price-group"><th colspan="2" scope="colgroup">Zink / Blei</th></tr>
            <tr data-price-id="zinkblech-neu"><th scope="row">Zinkblech, neu</th><td>1,97 €</td></tr>
            <tr data-price-id="zinkblech-alt"><th scope="row">Zinkblech, alt</th><td>1,77 €</td></tr>
            <tr data-price-id="altblei"><th scope="row">Altblei</th><td>0,93 €</td></tr>
          </tbody>
        </table>
    "#;

    const FIXTURE_RANGES: &str = r#"
        <table class="finder-table">
          <caption class="sr-only">Materialpreise im Überblick, Euro pro Kilogramm</caption>
          <tbody>
            <tr><th scope="row">Kupfer (Cu)</th><td>9,20–10,50 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Kupfer Kanal</th><td>9,20 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Armaturenmessing</th><td>5,45–6,50 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Rotguss (RG)</th><td>8,60–10,95 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Aluminium (Al)</th><td>0,95–2,69 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Edelstahl (V2A/V4A)</th><td>0,80–1,50 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Zinkblech</th><td>1,77–1,97 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Zinn (Sn)</th><td>13,00–21,00 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Blei-Akkus</th><td>0,85–1,00 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Elektro-Motoren</th><td>0,50–0,80 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Hochleistungsstahl (HSS)</th><td>12,00–20,00 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Hartmetall (VHM)</th><td>15,00–25,00 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Wolfram (W)</th><td>15,00–25,00 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Kabel &amp; E-Schrott</th><td>1,00–5,00 €/kg</td><td>Anfragen</td></tr>
            <tr><th scope="row">Mischschrott &amp; Eisen</th><td>0,15–0,30 €/kg</td><td>Anfragen</td></tr>
          </tbody>
        </table>
    "#;

    fn live_rows() -> Vec<(String, f64)> {
        let (rows, skipped) = parse_prices(FIXTURE_LIVE_TABLE).expect("parses");
        assert!(skipped.is_empty(), "{skipped:?}");
        rows
    }

    #[test]
    fn table_yields_31_rows() {
        let rows = live_rows();
        assert_eq!(rows.len(), 31, "{rows:?}");
    }

    #[test]
    fn current_cms_table_yields_named_prices() {
        let (rows, skipped) = parse_prices(FIXTURE_LIVE_TABLE).expect("current table parses");
        assert!(skipped.is_empty(), "{skipped:?}");
        assert_eq!(rows.len(), 31);
        assert_eq!(rows[0], ("Kupfer-Draht „Millberry“".to_owned(), 10.5));
        assert_eq!(rows[30], ("Altblei".to_owned(), 0.93));
    }

    #[test]
    fn overview_preserves_ranges_and_ignores_single_price_rows() {
        let ranges = parse_ranges(FIXTURE_RANGES).expect("overview parses");
        assert_eq!(ranges.len(), 14, "single-price Kupfer Kanal is not a range");
        assert_eq!(ranges[0], ("Kupfer (Cu)".to_owned(), 9.2, 10.5));
        assert_eq!(ranges[4], ("Edelstahl (V2A/V4A)".to_owned(), 0.8, 1.5));
        assert_eq!(ranges[13], ("Mischschrott & Eisen".to_owned(), 0.15, 0.3));

        let mapped: Vec<_> = ranges
            .iter()
            .filter_map(|(label, min, max)| {
                grade_for_range(label).map(|material| (material, min, max))
            })
            .collect();
        assert_eq!(mapped.len(), 10);
        assert!(mapped.iter().any(|(material, min, max)| {
            *material == "edelstahl-gemischt" && **min == 0.8 && **max == 1.5
        }));
        assert_eq!(grade_for_range("Rotguss (RG)"), None);
        assert_eq!(grade_for_range("Blei-Akkus"), None);
        assert_eq!(grade_for_range("Kabel & E-Schrott"), None);
        assert_eq!(grade_for_range("Wolfram (W)"), Some("wolfram"));
    }

    #[test]
    fn generic_tin_range_is_not_attributed_to_high_purity_catalog_tin() {
        assert_eq!(grade_for_range("Zinn (Sn)"), None);
    }

    #[test]
    fn current_snapshot_skips_ambiguous_tin_and_keeps_other_prices() {
        let (rows, skipped) = parse_prices(FIXTURE_LIVE_TABLE).expect("detail table parses");
        let ranges = parse_ranges(FIXTURE_RANGES).expect("overview parses");
        let (prices, skipped) = map_price_snapshot(rows, skipped, ranges).expect("full snapshot");

        assert_eq!(prices.len(), SNAPSHOT_PRICE_COUNT);
        assert!(skipped.iter().any(|label| label.starts_with("Zinn (Sn)")));
        assert!(prices
            .iter()
            .any(|price| price.material == "edelstahl-gemischt"));
        assert!(!prices
            .iter()
            .any(|price| price.material == "zinn" && price.label == "Zinn (Sn)"));
    }

    #[test]
    fn page_price_timestamp_is_validated_and_preserved() {
        assert_eq!(
            parse_published_at(FIXTURE_CMS_TABLE).expect("valid timestamp"),
            "2026-10-21T14:44:01+00:00"
        );
        assert!(parse_published_at("<time id='price-update-date'>today</time>").is_err());
    }

    #[test]
    fn historical_faq_markup_is_rejected() {
        assert!(parse_prices(FIXTURE_TABLE).is_err());
        assert!(parse_ranges(FIXTURE_UPTO).is_err());
    }

    #[test]
    fn changed_or_duplicated_table_contract_fails_closed() {
        let duplicate = format!("{FIXTURE_CMS_TABLE}{FIXTURE_CMS_TABLE}");
        assert!(parse_prices(&duplicate).is_err());
        let wrong_caption =
            FIXTURE_CMS_TABLE.replace("Sortenpreise in Euro pro Kilogramm", "Andere Tabelle");
        assert!(parse_prices(&wrong_caption).is_err());
        assert_eq!(parse_range("9,20–10,50 €/kg"), Some((9.2, 10.5)));
        assert_eq!(parse_range("10,50–9,20 €/kg"), None);
        let reversed = FIXTURE_RANGES.replace("9,20–10,50", "10,50–9,20");
        assert!(parse_ranges(&reversed).is_err());
        let changed_separator = FIXTURE_RANGES.replace("9,20–10,50", "9,20-10,50");
        assert!(parse_ranges(&changed_separator).is_err());
        assert!(parse_prices(FIXTURE_CMS_TABLE).is_err(), "partial snapshot");
    }

    #[test]
    fn anchor_missing_is_error() {
        assert!(parse_prices("<p>Kein Preis hier</p>").is_err());
        assert!(parse_prices("<p>Tagesaktuelle Sortenpreise, aber nichts dahinter").is_err());
    }

    #[test]
    fn mapping_resolves_grades_and_skips_composite() {
        let rows = live_rows();
        let get = |want: &str| {
            rows.iter()
                .find(|(l, _)| l.contains(want))
                .unwrap_or_else(|| panic!("missing {want}"))
                .clone()
        };
        // Brief-Richtwerte zuerst.
        let (l, p) = get("Millberry");
        assert_eq!((grade_for(&l), p), (Some(("kupfer-millberry", "")), 10.50));
        let (l, p) = get("H\u{00fc}lsen, sauber");
        assert_eq!((grade_for(&l), p), (Some(("messing", "huelsen")), 6.50));
        // Sorten trennen sich per Variante, sonst kollabieren Current-Preise.
        let (l, _) = get("\u{201e}Berry");
        assert!(!l.contains("Millberry"));
        assert_eq!(grade_for(&l), Some(("kupfer-berry", "")));
        assert_eq!(
            grade_for(&get("Kanal").0),
            Some(("kupfer-gemischt", "kanal"))
        );
        assert_eq!(
            grade_for(&get("Neublech").0),
            Some(("kupfer-gemischt", "neublech"))
        );
        assert_eq!(
            grade_for(&get("Schwerkupfer").0),
            Some(("kupfer-gemischt", "schwerkupfer-lotfrei"))
        );
        assert_eq!(
            grade_for(&get("Leichtkupfer").0),
            Some(("kupfer-gemischt", "leichtkupfer"))
        );
        // Messing 58: Schrott vs. Späne sind zwei Varianten.
        let ms58: Vec<_> = rows.iter().filter(|(l, _)| l.contains("58")).collect();
        assert_eq!(ms58.len(), 2);
        assert_eq!(grade_for(&ms58[0].0), Some(("messing", "ms58-schrott")));
        assert_eq!(ms58[0].1, 6.15);
        assert_eq!(grade_for(&ms58[1].0), Some(("messing", "ms58-spaene")));
        assert_eq!(ms58[1].1, 5.80);
        assert_eq!(
            grade_for(&get("63 Blech").0),
            Some(("messing", "ms63-blech"))
        );
        assert_eq!(
            grade_for(&get("Erodierdraht").0),
            Some(("messing", "erodierdraht"))
        );
        assert_eq!(
            grade_for(&get("Sp\u{00e4}ne, gemischt").0),
            Some(("messing", "spaene-gemischt"))
        );
        assert_eq!(
            grade_for(&get("ohne H\u{00fc}lsen").0),
            Some(("messing", "ohne-huelsen"))
        );
        // Composite ohne Katalogmaterial → laut skippen, nie raten.
        assert_eq!(grade_for(&get("K\u{00fc}hler").0), None);
        // Rotguss/Bronze.
        assert_eq!(
            grade_for(&get("Rotguss-Schrott").0),
            Some(("bronze-rotguss", ""))
        );
        assert_eq!(
            grade_for(&get("Rotguss-Sp\u{00e4}ne").0),
            Some(("bronze-rotguss", "spaene"))
        );
        assert_eq!(
            grade_for(&get("Walzbronze").0),
            Some(("bronze-rotguss", "walzbronze"))
        );
        // Alu-Profile: blank/gestegt/lackiert trotz gemeinsamem Base-Label.
        let prof: Vec<_> = rows.iter().filter(|(l, _)| l.contains("Profile")).collect();
        assert_eq!(prof.len(), 3);
        assert_eq!(grade_for(&prof[0].0), Some(("aluminium-profile", "")));
        assert_eq!(prof[0].1, 2.69);
        assert_eq!(
            grade_for(&prof[1].0),
            Some(("aluminium-profile", "gestegt"))
        );
        assert_eq!(
            grade_for(&prof[2].0),
            Some(("aluminium-profile", "lackiert"))
        );
        assert_eq!(
            grade_for(&get("RAL-Draht").0),
            Some(("aluminium-gemischt", "ral-draht"))
        );
        assert_eq!(
            grade_for(&get("Offset").0),
            Some(("aluminium-blech", "offset"))
        );
        assert_eq!(grade_for(&get("AlMg").0), Some(("aluminium-blech", "almg")));
        assert_eq!(
            grade_for(&get("Felgen").0),
            Some(("aluminium-guss", "felgen"))
        );
        // Geschirr/Guss mit/ohne Fe: "mit" schlägt "ohne" (Base enthält beides).
        assert_eq!(
            grade_for(&get("Geschirr ohne").0),
            Some(("aluminium-gemischt", "geschirr-ohne-fe"))
        );
        let mit_fe: Vec<_> = rows.iter().filter(|(l, _)| l.contains("mit Fe")).collect();
        assert_eq!(mit_fe.len(), 2);
        assert_eq!(
            grade_for(&mit_fe[0].0),
            Some(("aluminium-gemischt", "geschirr-mit-fe"))
        );
        assert_eq!(grade_for(&mit_fe[1].0), Some(("aluminium-guss", "mit-fe")));
        assert_eq!(
            grade_for(&get("Alu-Guss ohne").0),
            Some(("aluminium-guss", "ohne-fe"))
        );
        // Zink neu/alt, Blei.
        let zink: Vec<_> = rows
            .iter()
            .filter(|(l, _)| l.contains("Zinkblech"))
            .collect();
        assert_eq!(zink.len(), 2);
        assert_eq!(grade_for(&zink[0].0), Some(("zink", "neu")));
        assert_eq!(grade_for(&zink[1].0), Some(("zink", "alt")));
        assert_eq!(grade_for(&get("Altblei").0), Some(("blei", "")));
    }

    #[test]
    fn battery_price_labels_are_not_mapped_to_soft_lead() {
        assert_eq!(grade_for("Blei-Akkus"), None);
        let (mut rows, skipped) = parse_prices(FIXTURE_LIVE_TABLE).expect("current table parses");
        let lead_row = rows
            .iter_mut()
            .find(|(label, _)| label == "Altblei")
            .expect("lead row exists");
        lead_row.0 = "Blei-Akkus".to_owned();
        let ranges = parse_ranges(FIXTURE_RANGES).expect("overview parses");
        let error = map_price_snapshot(rows, skipped, ranges).unwrap_err();
        assert!(error.to_string().contains("39 statt 40"), "{error}");
    }

    #[test]
    fn cms_price_id_rejects_same_count_visible_label_drift() {
        let changed_label = FIXTURE_LIVE_TABLE.replace(
            r#"<tr data-price-id="altblei"><th scope="row">Altblei</th>"#,
            r#"<tr data-price-id="altblei"><th scope="row">Blei-Akkus</th>"#,
        );
        assert_ne!(
            changed_label, FIXTURE_LIVE_TABLE,
            "mutation must reach fixture"
        );
        assert!(
            parse_prices(&changed_label).is_err(),
            "row count and data-price-id must not hide a changed grade"
        );
    }

    #[test]
    fn cards_map_and_skip_off_catalog() {
        let cards = parse_cards(FIXTURE_CARDS);
        assert_eq!(cards.len(), 5);
        assert_eq!(
            grade_for_card("Edelstahl (V2A/V4A)"),
            Some(vec![("edelstahl-v2a", ""), ("edelstahl-v4a", "")])
        );
        assert_eq!(
            grade_for_card("Kupfer (Cu)"),
            Some(vec![("kupfer-gemischt", "")])
        );
        assert_eq!(grade_for_card("Blei-Akkus"), None, "Batterien ≠ Weichblei");
        assert_eq!(grade_for_card("Wolfram (W)"), Some(vec![("wolfram", "")]));
        assert_eq!(
            grade_for_card("Kabel & E-Schrott"),
            None,
            "Composite ohne exakten Fit"
        );
        assert_eq!(
            grade_for_card("Hochleistungsstahl (HSS)"),
            Some(vec![("hss-werkzeuge", "")])
        );
        assert_eq!(
            grade_for_card("Hartmetall (VHM)"),
            Some(vec![("hartmetall", "")])
        );
        assert_eq!(
            grade_for_card("Mischschrott & Eisen"),
            Some(vec![("mischschrott", "")])
        );
        assert_eq!(grade_for_card("Zinn (Sn)"), Some(vec![("zinn", "")]));
    }

    #[test]
    fn impressum_resists_node_glue() {
        // Glue-Check: ohne Whitespace zwischen Blockelementen klebt
        // scraper-text() alles zusammen ("e.K.Heinrich…47b98544").
        let imp = "<h2>1. Anbieter und Verantwortlicher</h2><p>Wahl & Co.</p><p>Heinrich-Ehrhardt-Str. 47b<br>98544 Zella-Mehlis<br>Deutschland</p><h3>Kontakt</h3><p><strong>Telefon:</strong> 03682 483449<br><strong>E-Mail:</strong> wahl-@t-online.de<br></p><h2>2. Registereintrag</h2>";
        let info = super::extract_info(imp).expect("parses trotz Glue");
        assert_eq!(info.street, "Heinrich-Ehrhardt-Str. 47b");
        assert_eq!(info.postcode, "98544");
        assert_eq!(info.city, "Zella-Mehlis");
        assert_eq!(info.phone, "03682 483449");
        assert_eq!(info.email, "wahl-@t-online.de");
    }
    #[test]
    fn impressum_provider_block() {
        let imp = "<h2>1. Anbieter und Verantwortlicher</h2>\
            <div class=\"info-box\"><p><strong>Wahl & Co. Inhaber Thomas Madeiski e.K.</strong></p>\
            <p>Heinrich-Ehrhardt-Str. 47b<br>98544 Zella-Mehlis<br>Deutschland</p></div>\
            <h3>Kontakt</h3><p><strong>Telefon:</strong> 03682 483449<br>\
            <strong>E-Mail:</strong> wahl-@t-online.de<br></p>\
            <h2>2. Registereintrag</h2>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Heinrich-Ehrhardt-Str. 47b");
        assert_eq!(info.postcode, "98544");
        assert_eq!(info.city, "Zella-Mehlis");
        assert_eq!(info.phone, "03682 483449");
        assert_eq!(info.email, "wahl-@t-online.de");
        assert!(extract_info("<p>ohne Anbieter</p>").is_err());
    }

    #[test]
    fn current_impressum_address_block_parses() {
        let imp = "<main><address>Wahl &amp; Co Inhaber Thomas Madeiski e.K.<br>\
            Heinrich-Ehrhardt-Straße 47b<br>98544 Zella-Mehlis<br>\
            Telefon: <a href=\"tel:+493682483449\">03682 483449</a><br>\
            E-Mail: <a href=\"mailto:wahl-@t-online.de\">wahl-@t-online.de</a>\
            </address></main>";
        let info = extract_info(imp).expect("current address block parses");
        assert_eq!(info.street, "Heinrich-Ehrhardt-Straße 47b");
        assert_eq!(info.postcode, "98544");
        assert_eq!(info.city, "Zella-Mehlis");
        assert_eq!(info.phone, "03682 483449");
        assert_eq!(info.email, "wahl-@t-online.de");
    }

    #[tokio::test]
    #[ignore = "read-only live source check; parses the page and Impressum, never records data"]
    async fn live_handler_parses_the_complete_current_snapshot() {
        let client = reqwest::Client::builder()
            .user_agent("schrott-mcp-ingestion/0.1")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        let outcome = super::scrape(&client)
            .await
            .expect("current Wahl price page, complete snapshot and Impressum");

        assert_eq!(outcome.prices.len(), SNAPSHOT_PRICE_COUNT);
    }
}
