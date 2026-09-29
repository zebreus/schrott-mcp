//! Manuel Triebsch Autoverwertung / Altmetall- & Schrottankauf
//! (Wangerland-Hooksiel): exact purchase prices in the homepage table
//! under "Wir zahlen Tageshöchstpreise für Schrott & Altmetalle !"
//! with explicit headers ("Preis je Tonne" / "Preis je Kilo") — 6
//! tonne rows (Stahlschrott, Guss, Motorenschrott, Bleibatterien,
//! SV/Schmelz, abgeholt) + 11 kilo rows (Edelstahl, Kupfer, Messing,
//! Mill-Berry, Zink, Blei, E-Motoren, Kabel, Alu, Felgen, Altfahrzeuge).
//! Empty cells stay empty (no cross-column defaults); each column's
//! unit is explicit, recorded honestly, normalized centrally.
//! "Altfahrzeuge (mit Kat) ab 120 Euro" is a per-vehicle quote → loud
//! skip (Stückpreis, never per-kg). "Schrott abgeholt" is a pickup
//! service rate, not a material price → loud skip.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "ni-wangerland-hooksiel-26434-manuel-triebsch-autoverwertung-altmetall";
/// Bespoke, live-verified contact page. Jimdo serves no dedicated
/// impressum URL (only a footer contact block on every page), so the
/// homepage doubles as the contact source — loudly documented, with
/// anchors on the "Firmengelände in Hooksiel" footer block. A move
/// fails the step loudly (fix the URL).
pub const IMPRESSUM_URL: &str = "https://www.schrott-triebsch.de/";

pub const URL: &str = "https://www.schrott-triebsch.de/";

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
    let (rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit,
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label,
            }),
            None => skipped_labels.push(label),
        }
    }
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
        published_at: None,
    })
}

/// Explicit label → (material, variant) mapping. "SV / Schmelz" names
/// no provable grade (not scheren without proof); "Motorenschrott" is
/// an unattributable engine mix; "Bleibatterien" are batteries, never
/// Weichblei — all skipped loudly, never guessed.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("stahlschrott") || l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("edelstahl") || l == "va" {
        Some(("edelstahl-gemischt", ""))
    } else if l.contains("mill-berry") || l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("blei") && !l.contains("batterie") {
        Some(("blei", ""))
    } else if l.contains("elektromotor") {
        Some(("elektromotoren", ""))
    } else if l.contains("kabel") {
        Some(("kabel-kupfer", "o. Stecker"))
    } else if l.contains("alu") && l.contains("felgen") {
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("alu") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("guss") {
        Some(("eisenschrott-gussbruch", ""))
    } else {
        None
    }
}

/// The price table between "Tageshöchstpreise für Schrott" and the page
/// footer. Columns are Materialart | Preis je Tonne | Preis je Kilo —
/// each cell carries its column's explicit unit; empty cells stay
/// empty (no cross-column defaults, no silent EUR/kg).
fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html
        .find("Tageshöchstpreise für Schrott")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    // Table end: the footer nav after the price table closes it.
    let end = tail.find("</table>").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preistabelle unvollständig".to_owned(),
    })?;
    let window = &tail[..end + "</table>".len()];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let table_sel = Selector::parse("table").expect("valid selector");
    let row_sel = Selector::parse("tr").expect("valid selector");
    let cell_sel = Selector::parse("td").expect("valid selector");
    let table = frag
        .select(&table_sel)
        .next()
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle fehlt".to_owned(),
        })?;
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in table.select(&row_sel) {
        let cells: Vec<String> = tr
            .select(&cell_sel)
            .map(|c| c.text().collect::<String>())
            .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        if cells.len() < 3 {
            continue;
        }
        let label = cells[0].clone();
        if label.is_empty() || label.to_lowercase().contains("materialart") {
            continue;
        }
        // Column 1 = per-tonne, column 2 = per-kilo. A row prices in at
        // most one column; a row with neither skips loudly.
        let mut priced = false;
        for (cell, unit) in [(&cells[1], "EUR/t"), (&cells[2], "EUR/kg")] {
            let Some(price) = parse_eur(cell) else {
                continue;
            };
            if price <= 0.0 {
                continue;
            }
            // "ab 120 Euro" (Altfahrzeuge) is a per-vehicle quote, not a
            // per-kg/t price — loud skip, never a scaled row.
            if cell.to_lowercase().contains("ab ") {
                skips.push(format!(
                    "{label} (Stückpreis, kein kg/t-Preis: {})",
                    cell.trim()
                ));
                priced = true;
                break;
            }
            rows.push((label.clone(), price, unit));
            priced = true;
            break;
        }
        if !priced {
            skips.push(format!("{label} (kein Preis)"));
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke contact extraction for THIS footer block only: "Firmengelände
/// in Hooksiel" + firm name + "Berghamm 1a" / "26434 Wangerland / OT
/// Hooksiel" + "Tel:"/"Fax:"/"Mob:" lines. Missing firm anchor → loud
/// error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let all: String = doc.root_element().text().collect();
    if !all.contains("Firmengelände in Hooksiel") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    }
    // Street: "Berghamm 1a" (this trader's street, verified live).
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if let Some(i) = all.find("Berghamm 1a") {
        street = "Berghamm 1a".to_owned();
        let rest = &all[i + "Berghamm 1a".len()..];
        let toks: Vec<&str> = rest.split_whitespace().collect();
        for (k, t) in toks.iter().enumerate().take(12) {
            if t.len() == 5 && t.chars().all(|c| c.is_ascii_digit()) {
                postcode = (*t).to_owned();
                if let Some(ci) = toks.get(k + 1) {
                    city = (*ci).to_owned();
                }
                break;
            }
        }
    }
    let after = |marker: &str| {
        all.find(marker).map(|i| {
            all[i + marker.len()..]
                .split_whitespace()
                .take_while(|t| {
                    t.chars()
                        .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
    };
    let phone = after("Tel:").unwrap_or_default();
    let email = all
        .split_whitespace()
        .find(|t| t.contains('@') && t.contains('.'))
        .unwrap_or_default()
        .trim_matches(|c: char| "()<>;,".contains(c))
        .to_owned();
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

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    // Real table shape (3 columns, mixed tonne/kilo/empty cells).
    const FIXTURE: &str = "<h2>Wir zahlen Tageshöchstpreise für Schrott & Altmetalle !</h2>\
        <table><tbody>\
        <tr><td>Materialart</td><td>Preis je Tonne</td><td>Preis je Kilo</td></tr>\
        <tr><td>Stahlschrott / Mischschrott</td><td>160 Euro</td><td></td></tr>\
        <tr><td>Kupfer</td><td></td><td>9,50 Euro</td></tr>\
        <tr><td>Bleibatterien</td><td>300 Euro</td><td>0,30 Euro</td></tr>\
        <tr><td>Altfahrzeuge (mit Kat)</td><td></td><td>ab 120 Euro</td></tr>\
        <tr><td>Motorenschrott</td><td>320 Euro</td><td></td></tr>\
        <tr><td>SV / Schmelz</td><td></td><td></td></tr>\
        </tbody></table><footer>nav</footer>";

    #[test]
    fn columns_carry_their_unit() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // Stahlschrott (t) + Kupfer (kg) + Bleibatterien (t-cell wins;
        // grade_for skips batteries later) + Motorenschrott (t).
        assert_eq!(rows.len(), 4);
        assert_eq!(
            rows[0],
            ("Stahlschrott / Mischschrott".to_owned(), 160.0, "EUR/t")
        );
        assert_eq!(rows[1], ("Kupfer".to_owned(), 9.50, "EUR/kg"));
        assert_eq!(rows[2].2, "EUR/t");
        assert_eq!(rows[3], ("Motorenschrott".to_owned(), 320.0, "EUR/t"));
        assert!(rows.iter().any(|r| r.0 == "Bleibatterien"), "{rows:?}");
        assert!(
            skips
                .iter()
                .any(|s| s.contains("Altfahrzeuge") && s.contains("Stückpreis")),
            "{skips:?}"
        );
        assert!(
            skips.iter().any(|s| s.contains("SV / Schmelz")),
            "{skips:?}"
        );
        assert!(parse("<div>Kein Ankauf hier</div>").is_err());
    }

    #[test]
    fn mapping_maps_and_skips_loudly() {
        assert_eq!(
            grade_for("Stahlschrott / Mischschrott"),
            Some(("mischschrott", ""))
        );
        assert_eq!(grade_for("Kupfer"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("SV / Schmelz"), None, "unproven grade");
        assert_eq!(grade_for("Bleibatterien"), None, "batteries, not Weichblei");
        assert_eq!(grade_for("Motorenschrott"), None, "engine mix");
        assert_eq!(grade_for("Altfahrzeuge (mit Kat)"), None, "per vehicle");
        assert_eq!(grade_for("Schrott abgeholt"), None, "service rate");
    }

    #[test]
    fn impressum_footer_block() {
        let imp = "<div>Firmengelände in Hooksiel Manuel Triebsch Berghamm 1a \
            26434 Wangerland / OT Hooksiel Tel: 04425 / 990 100 Fax: 04425 / 990 102</div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Berghamm 1a");
        assert_eq!(info.postcode, "26434");
        assert_eq!(info.city, "Wangerland");
        assert_eq!(info.phone, "04425 / 990 100");
        assert!(super::extract_info("<div>Neu hier</div>").is_err());
    }
}
