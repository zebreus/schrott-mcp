//! Schiefer & Co. Scheideanstalt (Hamburg-St. Georg): indicative industrial
//! purchase rates from the dedicated `Ankaufspreis` rows in
//! `table.tablekursdaten`. The page's header ticker is explicitly a
//! `Verkaufspreis` and must never be recorded as what the trader pays.
//!
//! Keep the last non-empty quote per metal across Eröffnung/Fixing/
//! Nachfixing. The table says Silber €/kg while the catalog uses EUR/g, so
//! convert that one quote exactly. These industrial rates are only a basis
//! for retail counter offers; the operator calls the published figures
//! nonbinding, so they are `approx`, not exact customer payouts.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "hh-st-georg-schiefer-co-edelmetall-scheideanstalt";
/// Bespoke, live-verified impressum URL (site footer links `/impressum`).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://schieferco.de/impressum";

pub const URL: &str = "https://schieferco.de/aktuelle-preise";

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
    let (published_at, rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (symbol, label, price, unit) in rows {
        match grade_for(&symbol) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit,
                price_kind: "approx",
                price_min: None,
                price_max: None,
                confidence: Some(0.5),
                label,
            }),
            // Unknown metal symbol: keep the quote as evidence in the skip.
            None => skipped_labels.push(format!(
                "{label} {symbol} ({}, {unit}, kein Katalogmaterial)",
                fmt_eur(price)
            )),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        prices,
        acceptances: vec![],
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at,
    })
}

fn fmt_eur(price: f64) -> String {
    format!("{price:.3}").replace('.', ",")
}

/// Explicit metal symbol → (material, variant). No fineness is stated, so
/// the variant stays standard ("").
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    match label.trim() {
        "Au" => Some(("gold", "")),
        "Ag" => Some(("silber", "")),
        "Pt" => Some(("platin", "")),
        "Pd" => Some(("palladium", "")),
        _ => None,
    }
}

/// Parse the explicit purchase rows in the metals table. The ticker is a
/// sell-price display and its timestamp does not date the purchase table.
/// Returns (published_at, (symbol, label, price, unit) rows, skips).
fn parse(
    html: &str,
) -> Result<
    (
        Option<String>,
        Vec<(String, String, f64, &'static str)>,
        Vec<String>,
    ),
    IngestError,
> {
    let missing = |detail: &str| IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.to_owned(),
    };
    let doc = Html::parse_document(html);
    let table_sel = Selector::parse("table.tablekursdaten").expect("valid selector");
    let row_sel = Selector::parse("tr").expect("valid selector");
    let cell_sel = Selector::parse("td").expect("valid selector");
    let table = doc
        .select(&table_sel)
        .next()
        .ok_or_else(|| missing("Kursdaten-Tabelle fehlt"))?;
    let mut columns: Option<Vec<Option<(&'static str, &'static str)>>> = None;
    let mut section = String::new();
    // symbol -> latest non-empty (label, normalized price, unit). A dash in
    // a later fixing does not erase that metal's last actually quoted rate.
    let mut latest: std::collections::BTreeMap<&'static str, (String, f64, &'static str)> =
        std::collections::BTreeMap::new();
    let mut skips = Vec::new();

    for row in table.select(&row_sel) {
        let cells: Vec<String> = row
            .select(&cell_sel)
            .map(|cell| {
                cell.text()
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();
        let Some(label) = cells.first().map(|s| s.trim()) else {
            continue;
        };

        let header_columns: Vec<_> = cells.iter().skip(1).map(|s| table_column(s)).collect();
        if header_columns.iter().filter(|c| c.is_some()).count() >= 4 {
            let symbols: std::collections::HashSet<_> = header_columns
                .iter()
                .filter_map(|c| c.map(|(symbol, _)| symbol))
                .collect();
            if symbols.len() != 4 {
                return Err(missing("Kursdaten-Kopfzeile hat doppelte Metallspalten"));
            }
            columns = Some(header_columns);
            continue;
        }

        if let Some(name) = ["Eröffnungspreise", "Fixingpreise", "Nachfixingpreise"]
            .into_iter()
            .find(|name| label.eq_ignore_ascii_case(name))
        {
            section = name.to_owned();
            continue;
        }
        if !label.eq_ignore_ascii_case("Ankaufspreis") {
            continue;
        }
        if section.is_empty() {
            return Err(missing("Ankaufspreis ohne Preisabschnitt"));
        }
        let columns = columns
            .as_ref()
            .ok_or_else(|| missing("Kursdaten-Metallspalten fehlen"))?;
        for (index, column) in columns.iter().enumerate() {
            let Some((symbol, source_unit)) = column else {
                continue;
            };
            let value = cells.get(index + 1).map(String::as_str).unwrap_or("");
            let Some(parsed) = parse_tick(value) else {
                if !value.is_empty() {
                    skips.push(format!(
                        "Ankaufspreis {section} {symbol} (kein Kurs: {value})"
                    ));
                }
                continue;
            };
            if !parsed.is_finite() || parsed <= 0.0 {
                skips.push(format!(
                    "Ankaufspreis {section} {symbol} (ungültiger Kurs: {value})"
                ));
                continue;
            }
            let (price, unit) = match *source_unit {
                "EUR/kg" => (parsed / 1000.0, "EUR/g"),
                "EUR/g" => (parsed, "EUR/g"),
                _ => {
                    skips.push(format!(
                        "Ankaufspreis {section} {symbol} (Einheit unbekannt: {source_unit})"
                    ));
                    continue;
                }
            };
            latest.insert(symbol, (format!("Ankaufspreis {section}"), price, unit));
        }
    }

    let columns = columns.ok_or_else(|| missing("Kursdaten-Metallspalten fehlen"))?;
    let symbols: std::collections::HashSet<_> = columns
        .iter()
        .filter_map(|c| c.map(|(symbol, _)| symbol))
        .collect();
    if symbols.len() != 4 {
        return Err(missing(
            "Kursdaten-Tabelle unvollständig: Gold/Silber/Platin/Palladium",
        ));
    }
    if latest.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine verwertbaren Ankaufspreise".to_owned(),
        });
    }
    let rows = latest
        .into_iter()
        .map(|(symbol, (label, price, unit))| (symbol.to_owned(), label, price, unit))
        .collect();
    // The timestamp beside the sell-side ticker is not asserted to date the
    // table's industrial purchase rates.
    Ok((None, rows, skips))
}

/// Only the table's four named fine-metal columns are recognized. The unit
/// is taken from the heading, never borrowed from the unrelated ticker.
fn table_column(header: &str) -> Option<(&'static str, &'static str)> {
    let header = header.to_lowercase().replace(' ', "");
    let unit = if header.contains("€/kg") {
        "EUR/kg"
    } else if header.contains("€/g") {
        "EUR/g"
    } else {
        return None;
    };
    if header.contains("gold") && !header.contains("palladium") {
        Some(("Au", unit))
    } else if header.contains("silber") {
        Some(("Ag", unit))
    } else if header.contains("platin") {
        Some(("Pt", unit))
    } else if header.contains("palladium") {
        Some(("Pd", unit))
    } else {
        None
    }
}

/// The source sometimes writes decimal points with three places (e.g.
/// "1.712"), which the shared German parser treats as a thousands group.
/// This table-local parser recognizes that shape as a decimal.
fn parse_tick(raw: &str) -> Option<f64> {
    let tok: String = raw
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
        .collect();
    let tok = tok.trim_matches(['.', ',']);
    if tok.contains(',') {
        return parse_eur(raw);
    }
    let mut parts = tok.split('.');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(_), Some(dec), None) if dec.len() == 3 && dec.chars().all(|c| c.is_ascii_digit()) => {
            tok.parse().ok()
        }
        _ => parse_eur(raw),
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` holding
/// "Ellmenreichstraße 24 | … | St. Georg" plus "D – 20099 Hamburg | Tel:
/// …", and the `mailto:` link. Missing anchors → loud error, never a
/// guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p = Selector::parse("p").expect("valid selector");
    let addr_html = doc
        .select(&p)
        .map(|el| el.inner_html())
        .find(|h| h.contains("Ellmenreichstra"))
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Ellmenreichstraßen-Block fehlt".to_owned(),
        })?;
    let lines: Vec<String> = addr_html
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let mut street = String::new();
    let mut phone = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    for line in &lines {
        for seg in line.split('|') {
            let seg = seg.trim();
            if seg.contains("Ellmenreichstra") && street.is_empty() {
                // "Ellmenreichstraße 24 | Am Hamburger Hauptbahnhof | St. Georg"
                street = seg.split(" |").next().unwrap_or(seg).trim().to_owned();
            }
            if seg.starts_with("Tel:") && phone.is_empty() {
                phone = seg
                    .trim_start_matches("Tel:")
                    .split('|')
                    .next()
                    .unwrap_or("")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
            }
            let toks: Vec<&str> = seg.split_whitespace().collect();
            for (k, t) in toks.iter().enumerate() {
                let digits: String = t.chars().filter(|c| c.is_ascii_digit()).collect();
                if digits.len() == 5 && toks.len() > k + 1 {
                    postcode = digits;
                    city = toks[k + 1..]
                        .join(" ")
                        .trim_matches([',', '|'])
                        .trim()
                        .to_owned();
                    break;
                }
            }
        }
    }
    // Email rides on the page's own mailto link (never a token split —
    // scraper text() would glue neighbour nodes).
    let mut email = String::new();
    if let Some(i) = imp.find("mailto:") {
        email = imp[i + 7..]
            .chars()
            .take_while(|c| *c != '"' && *c != '\'' && *c != '>' && !c.is_whitespace())
            .collect();
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

/// Strip tags from a `<br>`-split fragment. Fragments start with a tag
/// remnant — drop everything up to the first '>' first, or attributes
/// parse as text.
fn strip_fragment(s: &str) -> String {
    let s = match s.find('>') {
        Some(i) => &s[i + 1..],
        None => s,
    };
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            out.push(c);
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, parse_tick};

    // Real shape of the live ticker (div ids, b/span split, the
    // "Notierungen, €/g" line with quote time), trimmed to two metals.
    const FIXTURE: &str = "<div class=\"kursen-block\"><div id=\"kursen\">\
        <div id=\"pd\"><b>Pd</b><br/><span>38.800</span></div>\
        <div id=\"au\"><b>Au</b><br/><span>127.730</span></div>\
        <div id=\"partner\"><div><b>Notierungen, €/g</b>27.09.2026 21:00</div></div>\
        </div><div id=\"unverbindlich\"><small>Verkaufspreise für unverarbeitete Feinmetalle. Unverbindliche Angaben.</small></div></div>";

    #[test]
    fn sell_only_ticker_cannot_become_a_purchase_price() {
        assert!(parse(FIXTURE).is_err());
        assert!(parse("<div>Redesign ohne Preis-Tabelle</div>").is_err());
    }

    #[test]
    fn uses_only_latest_purchase_quotes_and_normalizes_silver() {
        // Live page shape: the header ticker is explicitly a sell price;
        // the table has separate purchase/sale rows and several quote times.
        let html = r#"
            <div id="kursen">
              <div id="au"><b>Au</b><span>126.740</span></div>
              <div id="ag"><b>Ag</b><span>1.911</span></div>
              <div id="pt"><b>Pt</b><span>52.800</span></div>
              <div id="pd"><b>Pd</b><span>37.250</span></div>
              <div id="partner"><b>Notierungen, €/g</b>04.10.2026 13:46</div>
            </div>
            <div id="unverbindlich">Verkaufspreise für unverarbeitete Feinmetalle.</div>
            <table class="tablekursdaten">
              <tr><td></td><td>Gold €/g</td><td>Silber €/kg</td><td>Platin €/g</td><td>Palladium €/g</td></tr>
              <tr><td><b>Eröffnungspreise</b></td><td></td><td></td><td></td><td></td></tr>
              <tr><td>Ankaufspreis</td><td>117.32</td><td>1719.80</td><td>47.70</td><td>33.00</td></tr>
              <tr><td>Verkaufspreis unverarbeitet</td><td>126.74</td><td>1911.30</td><td>-</td><td>-</td></tr>
              <tr><td>Verkaufspreis verarbeitet</td><td>132.78</td><td>2052.90</td><td>52.80</td><td>37.25</td></tr>
              <tr><td><b>Fixingpreise</b></td><td></td><td></td><td></td><td></td></tr>
              <tr><td>Ankaufspreis</td><td>117.64</td><td>1720.40</td><td>-</td><td>-</td></tr>
              <tr><td>Verkaufspreis unverarbeitet</td><td>126.19</td><td>1891.60</td><td>-</td><td>-</td></tr>
              <tr><td>Verkaufspreis verarbeitet</td><td>132.20</td><td>2031.80</td><td>-</td><td>-</td></tr>
              <tr><td><b>Nachfixingpreise</b></td><td></td><td></td><td></td><td></td></tr>
              <tr><td>Ankaufspreis</td><td>117.22</td><td>1712.00</td><td>-</td><td>-</td></tr>
              <tr><td>Verkaufspreis unverarbeitet</td><td>126.64</td><td>1900.90</td><td>-</td><td>-</td></tr>
              <tr><td>Verkaufspreis verarbeitet</td><td>132.67</td><td>2041.80</td><td>-</td><td>-</td></tr>
            </table>
        "#;

        let (published_at, rows, _) = parse(html).expect("purchase rows parse");
        let by_material: std::collections::BTreeMap<_, _> = rows
            .into_iter()
            .map(|(symbol, _, price, unit)| (symbol, (price, unit)))
            .collect();
        assert_eq!(
            published_at, None,
            "sell ticker timestamp is not a table date"
        );
        assert_eq!(by_material.get("Au"), Some(&(117.22, "EUR/g")));
        assert_eq!(by_material.get("Ag"), Some(&(1.712, "EUR/g")));
        assert_eq!(by_material.get("Pt"), Some(&(47.7, "EUR/g")));
        assert_eq!(by_material.get("Pd"), Some(&(33.0, "EUR/g")));
        assert_eq!(by_material.len(), 4);
    }

    #[test]
    fn dot_with_three_decimals_is_not_thousands() {
        // Shared parse_eur reads these as thousands groups (the 1000x
        // trap this bespoke parser exists to avoid).
        assert_eq!(super::parse_eur("127.730"), Some(127730.0));
        assert_eq!(parse_tick("127.730"), Some(127.73));
        assert_eq!(parse_tick("1.961"), Some(1.961));
        assert_eq!(parse_tick("38.800"), Some(38.8));
        // Two decimals stay shared behaviour.
        assert_eq!(parse_tick("118.25"), Some(118.25));
        assert_eq!(parse_tick("1.765,10"), Some(1765.1));
    }

    #[test]
    fn symbols_map_to_fine_metals() {
        assert_eq!(grade_for("Au"), Some(("gold", "")));
        assert_eq!(grade_for("Ag"), Some(("silber", "")));
        assert_eq!(grade_for("Pt"), Some(("platin", "")));
        assert_eq!(grade_for("Pd"), Some(("palladium", "")));
        assert_eq!(grade_for("Rh"), None);
    }

    #[test]
    fn impressum_address_block() {
        let imp = "<h1>IMPRESSUM</h1><p>Schiefer &amp; Co. (GmbH &amp; Co.)&nbsp;Edelmetall-Scheideanstalt seit 1923<br/>\
            Ellmenreichstraße 24 | Am Hamburger Hauptbahnhof | St. Georg<br/>\
            D – 20099 Hamburg | Tel: +49 (0)40 28 40 92 – 0<br/>\
            Fax: +49 (0)40 28 40 92 – 20 | Web: www.schieferco.de<br/>\
            Mail: <a href=\"mailto:info@schiefer.co\">info@schiefer.co</a></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Ellmenreichstraße 24");
        assert_eq!(info.postcode, "20099");
        assert_eq!(info.city, "Hamburg");
        assert!(info.phone.contains("28 40 92"));
        assert_eq!(info.email, "info@schiefer.co");
        assert!(extract_info("<p>Neu hier</p>").is_err());
    }
}
