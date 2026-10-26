//! Limekon's dedicated Richtpreisliste; never merge the conflicting homepage.
//! The Impressum verifies operator/address identity, but contact values are
//! intentionally not projected until independently corroborated.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-doerentrup-limekon";
pub const URL: &str = "https://limekon.de/pages/preisliste.php";
pub const IMPRESSUM_URL: &str = "https://limekon.de/pages/impressum1.php";

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
    let mut out = parse(&html)?;
    let (_, impressum) = fetch_text(client, IMPRESSUM_URL).await?;
    out.trader_info = extract_info(&impressum)?;
    out.website_alive = true;
    out.status_code = status;
    out.byte_len = html.len();
    Ok(out)
}

fn error(detail: &str) -> IngestError {
    IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.to_owned(),
    }
}

fn text(el: ElementRef<'_>) -> String {
    el.text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Exact page-specific labels AND source units: no substring/fineness guessing.
fn grade(label: &str) -> Option<(&'static str, &'static str, &'static str, f64)> {
    Some(match label {
        "Zinn fein / 99er" => ("zinn", "99er", "€/kg", 1.0),
        "Geschirrzinn 85-97%" => ("zinn-geschirr", "85-97%", "€/kg", 1.0),
        "Zinnlegierungen ca. 50%" => ("zinn-legierungen", "ca. 50%", "€/kg", 1.0),
        "Silber fein / 999" => ("silber", "999", "€/kg", 0.001),
        "Silber 800er" => ("silber", "800", "€/g", 1.0),
        "Silber 925er" => ("silber", "925", "€/g", 1.0),
        "Gold 999 / fein" => ("gold", "999", "€/g", 1.0),
        "Gold 750er" => ("gold", "750", "€/g", 1.0),
        "Gold 585er" => ("gold", "585", "€/g", 1.0),
        "Gold 333er" => ("gold", "333", "€/g", 1.0),
        "Zahngold gelb" => ("zahngold", "gelb", "€/g", 1.0),
        "Zahngold weiß" => ("zahngold", "weiß", "€/g", 1.0),
        "Eisen" => ("mischschrott", "Eisen", "€/kg", 1000.0),
        // Existing catalog explicitly includes Raff in kupfer-gemischt.
        "Kupfer raff" => ("kupfer-gemischt", "raff", "€/kg", 1.0),
        "Messing schwer" => ("messing", "schwer", "€/kg", 1.0),
        _ => return None,
    })
}

fn skip_reason(label: &str) -> &'static str {
    match label {
        "Besteck versilbert >=90"
        | "Besteck versilbert <90"
        | "Messer >=90"
        | "Messer <90"
        | "Korpus versilbert" => "versilberte Objektklasse, kein massives Silber",
        "5 DEM Silber" => "Stückpreis, kein Gewichtspreis",
        "andere Legierungen" => "Formel ohne numerische Quote/Feingehalt",
        "Münzen / Barren (auf fein)" => {
            "Feingewichtsbasis; kein eigener Basis-Typ, nicht als Bruttogewicht übernehmen"
        }
        "Double" => "vergoldete Ware, keine massive Goldlegierung",
        "Modeschmuck" | "Weltmünzen" => "unspezifizierte Objekt-/Mischfraktion",
        "Blei/Zink" => "kombinierter Eintrag, nicht in Einzelmaterialpreise aufteilen",
        _ => "unbekanntes Label/Grad, nicht in Allowlist",
    }
}

/// Date is a page-stated calendar day encoded using the shared date convention,
/// not a claim that the page was updated at UTC midnight. No valid_to inferred.
fn parse(html: &str) -> Result<HandlerOutcome, IngestError> {
    let doc = Html::parse_document(html);
    let main_sel = Selector::parse("#beginContent").unwrap();
    let main = doc
        .select(&main_sel)
        .next()
        .ok_or_else(|| error("Preisbereich fehlt"))?;
    let p_sel = Selector::parse("p").unwrap();
    let dates: Vec<_> = main
        .select(&p_sel)
        .map(text)
        .filter_map(|s| s.strip_prefix("Preisliste ab ").map(str::to_owned))
        .collect();
    if dates.len() != 1 || !main.select(&p_sel).any(|p| text(p) == "Richtpreise") {
        return Err(error(
            "Preisliste-Datum oder Richtpreise-Kennzeichnung fehlt/mehrdeutig",
        ));
    }
    let parts: Vec<_> = dates[0].split('.').collect();
    let published_at =
        if parts.len() == 3 && parts[0].len() == 2 && parts[1].len() == 2 && parts[2].len() == 4 {
            parse_de_date(parts[0], parts[1], parts[2])
        } else {
            None
        }
        .ok_or_else(|| error("ungültiges Preisdatum"))?;
    let table_sel = Selector::parse("table").unwrap();
    let tables: Vec<_> = main.select(&table_sel).collect();
    if tables.len() != 1 {
        return Err(error("Preistabelle fehlt/mehrdeutig"));
    }
    let row_sel = Selector::parse("tr").unwrap();
    let cell_sel = Selector::parse("td").unwrap();
    let mut out = HandlerOutcome {
        fetch_url: URL.to_owned(),
        published_at: Some(published_at),
        ..Default::default()
    };
    for row in tables[0].select(&row_sel) {
        let cells: Vec<_> = row.select(&cell_sel).map(text).collect();
        if cells.iter().all(String::is_empty) {
            continue;
        }
        if cells.len() != 2 {
            out.skipped_labels
                .push(format!("{} (unerwartete Tabellenstruktur)", text(row)));
            continue;
        }
        let (label, value) = (&cells[0], &cells[1]);
        let raw = format!("{label}: {value}");
        let Some((material, variant, source_unit, factor)) = grade(label) else {
            out.skipped_labels
                .push(format!("{raw} ({})", skip_reason(label)));
            continue;
        };
        let Some(price) = strict_price(value, source_unit).map(|v| v * factor) else {
            out.skipped_labels
                .push(format!("{raw} (unbekannte Einheit/ungültiger Preis)"));
            continue;
        };
        if out
            .prices
            .iter()
            .any(|p| p.material == material && p.variant == variant)
        {
            return Err(error(
                "doppelte Preisvariante; Konflikt nicht automatisch mergen",
            ));
        }
        out.prices.push(ScrapedPrice {
            material,
            variant: variant.into(),
            price,
            currency: "EUR",
            unit: if factor == 1000.0 {
                "EUR/t"
            } else if source_unit == "€/g" || factor == 0.001 {
                "EUR/g"
            } else {
                "EUR/kg"
            },
            price_kind: "approx",
            price_min: None,
            price_max: None,
            confidence: Some(0.5),
            label: raw,
        });
    }
    if out.prices.is_empty() {
        return Err(error("keine zuordenbaren Richtpreise"));
    }
    Ok(out)
}

/// Whole-cell validation precedes permissive shared number parsing.
fn strict_price(value: &str, unit: &str) -> Option<f64> {
    let number = value.strip_suffix(unit)?.trim();
    let (whole, decimal) = number.split_once(',')?;
    if decimal.len() != 2 || !decimal.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let groups: Vec<_> = whole.split('.').collect();
    if groups[0].is_empty()
        || !groups.iter().all(|g| g.bytes().all(|b| b.is_ascii_digit()))
        || (groups.len() > 1 && (groups[0].len() > 3 || groups[1..].iter().any(|g| g.len() != 3)))
    {
        return None;
    }
    parse_eur(number).filter(|p| p.is_finite() && *p > 0.0)
}

/// Bespoke Impressum identity/address parser; phone and email remain empty
/// because the Dossier source audit found no independent corroboration for
/// those operator-only contact fields. The Impressum is still required to
/// confirm the current entity and address before accepting this price page.
fn extract_info(html: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(html);
    let main_sel = Selector::parse("#beginContent").unwrap();
    let heading = Selector::parse("h1").unwrap();
    let p_sel = Selector::parse("p").unwrap();
    let failure = || IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Limekon-Impressum/Kontaktblock fehlt oder ist unvollständig".to_owned(),
    };
    let main = doc.select(&main_sel).next().ok_or_else(failure)?;
    if !main.select(&heading).any(|h| text(h) == "Impressum") {
        return Err(failure());
    }
    let mut info = TraderInfo::default();
    for p in main.select(&p_sel) {
        let inner = p.inner_html().replace("<br>", "\n").replace("<br />", "\n");
        let lines: Vec<_> = inner
            .split('\n')
            .map(|s| {
                Html::parse_fragment(s)
                    .root_element()
                    .text()
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .filter(|s| !s.is_empty())
            .collect();
        if lines.first().is_some_and(|s| s == "Limekon GmbH") && lines.len() == 3 {
            if let Some((postcode, city)) = lines[2].split_once(' ') {
                if postcode.len() == 5 && postcode.bytes().all(|b| b.is_ascii_digit()) {
                    info.street = lines[1].clone();
                    info.postcode = postcode.to_owned();
                    info.city = city.to_owned();
                }
            }
        }
    }
    if info.street.is_empty() || info.postcode.is_empty() || info.city.is_empty() {
        return Err(failure());
    }
    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRICES: &str = include_str!("fixtures/limekon-preisliste-20261014.html");

    #[test]
    fn dedicated_richtpreise_keep_grades_date_and_catalog_units() {
        let out = parse(PRICES).unwrap();
        let expected = [
            ("zinn", "99er", 37.0, "EUR/kg"),
            ("zinn-geschirr", "85-97%", 28.50, "EUR/kg"),
            ("zinn-legierungen", "ca. 50%", 17.50, "EUR/kg"),
            ("silber", "999", 1.0, "EUR/g"),
            ("silber", "800", 0.80, "EUR/g"),
            ("silber", "925", 0.93, "EUR/g"),
            ("gold", "999", 105.0, "EUR/g"),
            ("gold", "750", 78.75, "EUR/g"),
            ("gold", "585", 61.42, "EUR/g"),
            ("gold", "333", 34.65, "EUR/g"),
            ("zahngold", "gelb", 69.0, "EUR/g"),
            ("zahngold", "weiß", 25.0, "EUR/g"),
            ("mischschrott", "Eisen", 100.0, "EUR/t"),
            ("kupfer-gemischt", "raff", 9.0, "EUR/kg"),
            ("messing", "schwer", 5.0, "EUR/kg"),
        ];
        assert_eq!(out.prices.len(), expected.len());
        for (p, (material, variant, price, unit)) in out.prices.iter().zip(expected) {
            assert_eq!(
                (p.material, p.variant.as_ref(), p.price, p.unit),
                (material, variant, price, unit)
            );
            assert_eq!(p.price_kind, "approx");
            assert_eq!(p.confidence, Some(0.5));
            assert_eq!(p.currency, "EUR");
            assert_eq!((p.price_min, p.price_max), (None, None));
        }
        assert_eq!(
            out.published_at.as_deref(),
            Some("2026-10-14T00:00:00+00:00")
        );
        assert_eq!(out.skipped_labels.len(), 12);
        assert!(out.prices[3].label.contains("1.000,00 €/kg"));
        assert!(out.prices[12].label.contains("0,10 €/kg"));
    }

    #[test]
    fn unknown_labels_units_and_invalid_values_fail_closed() {
        for (old, new) in [
            ("Geschirrzinn 85-97%", "Geschirrzinn 80-99%"),
            ("Gold 333er", "Gold 334er"),
            ("37,00 &euro;/kg", "37,00 &euro;/g"),
            ("37,00 &euro;/kg", "37,00 &euro;/Stück"),
            ("37,00 &euro;/kg", "37,00 &euro;/kg netto"),
            ("105,00 &euro;/g", "105,00 &euro;/g fein"),
            ("37,00 &euro;/kg", "0,00 &euro;/kg"),
            ("37,00 &euro;/kg", "-37,00 &euro;/kg"),
            ("37,00 &euro;/kg", "37,00-40,00 &euro;/kg"),
            ("37,00 &euro;/kg", "Preis auf Anfrage"),
            ("1.000,00 &euro;/kg", "1.000,00 &euro;/g"),
        ] {
            let out = parse(&PRICES.replace(old, new)).unwrap();
            assert_eq!(out.prices.len(), 14, "{new}");
            assert_eq!(out.skipped_labels.len(), 13, "{new}");
        }
        for excluded in [
            "Besteck versilbert >=90",
            "Besteck versilbert <90",
            "Messer >=90",
            "Messer <90",
            "Korpus versilbert",
            "5 DEM Silber",
            "andere Legierungen",
            "Münzen / Barren (auf fein)",
            "Double",
            "Modeschmuck",
            "Weltmünzen",
            "Blei/Zink",
        ] {
            let out = parse(PRICES).unwrap();
            assert!(
                out.skipped_labels.iter().any(|s| s.starts_with(excluded)),
                "{excluded}"
            );
            assert!(grade(excluded).is_none());
        }
    }

    #[test]
    fn missing_anchors_empty_prices_and_duplicate_grade_are_errors() {
        for html in [
            PRICES.replace("beginContent", "other"),
            PRICES.replace("Preisliste ab", "Stand"),
            PRICES.replace("14.10.2026", "31.02.2026"),
            PRICES.replace("Richtpreise", "Preise"),
            PRICES.replace("Gold 750er", "Gold 585er"),
            PRICES.replace("</div>", "<table></table></div>"),
            "<div id='beginContent'><p>Preisliste ab 14.10.2026</p><p>Richtpreise</p><table><tr><td>Unbekannt</td><td>1,00 €/kg</td></tr></table></div>".to_owned(),
        ] { assert!(parse(&html).is_err()); }
        let out = parse(&format!(
            "{PRICES}<table><tr><td>Gold 835er</td><td>999,00 €/g</td></tr></table>"
        ))
        .unwrap();
        assert_eq!(out.prices.len(), 15, "outside page scope is not merged");
    }

    #[test]
    fn registered_handler_keeps_identity_and_address_but_withholds_unverified_contacts() {
        let h = super::super::all()
            .into_iter()
            .filter(|h| h.slug == SLUG)
            .collect::<Vec<_>>();
        assert_eq!(h.len(), 1);
        assert_eq!(h[0].url, URL);
        let info = extract_info(include_str!("fixtures/limekon-impressum-20261014.html")).unwrap();
        assert_eq!(info.street, "Bahnhofstr. 23");
        assert_eq!(info.postcode, "32694");
        assert_eq!(info.city, "Dörentrup-Bega");
        assert_eq!(info.phone, "");
        assert_eq!(info.email, "");
        let no_contacts = include_str!("fixtures/limekon-impressum-20261014.html")
            .replace("Telefon: +49 5263 95 61 906<br />", "")
            .replace("E-Mail: info(at)limekon(dot)de", "");
        let info = extract_info(&no_contacts).expect("identity/address do not depend on contacts");
        assert_eq!(info.street, "Bahnhofstr. 23");
        assert_eq!(info.phone, "");
        assert_eq!(info.email, "");
        assert!(extract_info("<h1>Impressum</h1>").is_err());
    }
}
