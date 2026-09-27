//! Schrott-Recycle Meikel (Frankfurt): six exact day prices in the
//! homepage price table (`table.jw-table`) whose thead IS the first price
//! row ("Milberry" / "7,00 € kg"). The "Ankaufspreise sind
//! Tages-/mengenabhängig" row carries no price and skips loudly — it is a
//! mid-table disclaimer, not a terminator, so the Messing row below it
//! still parses (its "3,95 kg" cell has no € sign, which is fine: the
//! bespoke unit matcher only needs the "kg"). Day/quantity dependence is a
//! disclaimer like Vedder's "Unverbindliche Ankaufspreise", so rows are
//! exact at 1.0. The homepage is the price page here (tappe pattern), so
//! URL doubles as source_url by necessity.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "he-frankfurt-am-main-schrott-recycle-meikel";
/// Bespoke, live-verified impressum URL (the site menu's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "https://www.schrott-recycling-meikel.de/impressum";

pub const URL: &str = "https://www.schrott-recycling-meikel.de/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
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
        published_at: None,
    })
}

/// Explicit label → (material, variant) mapping, specific before generic
/// ("kupfer" must not catch "Milberry" — it doesn't contain it, but the
/// order documents the intent). Generic page labels land on the generic
/// material, never a specific grade. Anything unlisted is skipped.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("milberry") || l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("kabel") {
        // Bare "Kabel" at a copper-cable price level (2,80 €/kg; alu cable
        // trades far lower) — mapped deliberately, flagged as uncertain.
        Some(("kabel-kupfer", ""))
    } else if l.contains("alu") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<h2>` "Angaben
/// gemäß § 5 TMG" heads the firm `<h1>` plus address `<p>`s
/// ("… Vatterstraße 23" / "60386 Frankfurt"), and the `<h2>Kontakt</h2>`
/// heads the `<p>` with "Telefon:"/"E-Mail:" lines. Missing anchors mean
/// the page changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let headings: Vec<String> =
        doc.select(&h2).map(|h| h.text().collect::<String>().trim().to_owned()).collect();
    if !headings.iter().any(|h| h.contains("Angaben gemäß")) {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    }
    if !headings.iter().any(|h| h == "Kontakt") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let (mut phone, mut email) = (String::new(), String::new());
    for el in doc.select(&p) {
        let lines: Vec<String> =
            el.inner_html().split("<br").map(strip_fragment).filter(|s| !s.is_empty()).collect();
        for line in &lines {
            if let Some(v) = line.strip_prefix("Telefon:") {
                if phone.is_empty() {
                    phone = v.trim().to_owned();
                }
            } else if let Some(v) = line.strip_prefix("E-Mail:") {
                if email.is_empty() {
                    email = v.trim().to_owned();
                }
            } else if street.is_empty()
                && (line.to_lowercase().contains("straße")
                    || line.contains("Strasse")
                    || line.contains("Str."))
            {
                street = line.clone();
            } else if postcode.is_empty() {
                let mut it = line.split_whitespace();
                if let (Some(pc), Some(_)) = (it.next(), it.next()) {
                    if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                        postcode = pc.to_owned();
                        city = line[pc.len()..].trim().to_owned();
                    }
                }
            }
        }
    }
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo { street, postcode, city, phone, email })
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

/// Parse the homepage price table. The thead row is the Milberry price
/// row, not a header — it parses like every tbody row. Returns
/// (rows, skips); the disclaimer row has no price and skips loudly, 0
/// priced rows is an error.
fn parse(
    html: &str,
) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let table = Selector::parse("table.jw-table").expect("valid selector");
    let row = Selector::parse("tr").expect("valid selector");
    let cell = Selector::parse("th, td").expect("valid selector");
    // Head-content selection (guide: Kopfinhalt, nie die erste): the price
    // table is the one whose header carries the Milberry price in €.
    let table = doc.select(&table).find(|t| {
        t.select(&cell).any(|c| {
            c.value().name() == "th"
                && c.text().collect::<String>().contains('€')
                && parse_eur(&c.text().collect::<String>()).is_some()
        })
    });
    let Some(table) = table else {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "keine Preistabelle".to_owned() });
    };
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in table.select(&row) {
        let cells: Vec<String> = tr
            .select(&cell)
            .map(|c| {
                c.text().collect::<String>().replace(['\u{a0}'], " ").split_whitespace().collect::<Vec<_>>().join(" ")
            })
            .collect();
        if cells.len() < 2 {
            continue;
        }
        let label = cells[0].trim().to_owned();
        let price_cell = cells[1].trim().to_owned();
        if label.len() > 120 {
            continue;
        }
        let Some(price) = parse_eur(&price_cell) else {
            if !label.is_empty() {
                skips.push(format!("{label} (kein Preis)"));
            }
            continue;
        };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let Some(unit) = unit_of(&price_cell) else {
            skips.push(format!("{label} (Einheit unverständlich: {price_cell})"));
            continue;
        };
        if label.is_empty() {
            continue;
        }
        rows.push((label, price, unit));
    }
    // Dedup identical (material-blind) repeats after mapping: the table is
    // rendered once, but a repeated block must never double-count.
    let mut seen = std::collections::HashSet::new();
    rows.retain(|(l, p, _)| seen.insert((l.clone(), p.to_bits())));
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Preistabelle leer".to_owned() });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS table's price cells (live: "7,00 € kg",
/// "3,95 kg"). Only kg exists here — anything else skips loudly at the
/// call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower.split(|c: char| !c.is_alphanumeric()).any(|w| w == "t" || w == "to") {
        Some("EUR/t")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    // Real table shape (thead as Milberry price row, disclaimer row
    // without price, Messing row without € sign), trimmed like live.
    const FIXTURE: &str = "<table width=\"100%\" class=\"jw-table jw-table--header\">\
        <thead><tr><th width=\"50%\">Milberry</th><th width=\"50%\">7,00 € kg</th></tr></thead>\
        <tbody>\
        <tr><td width=\"50%\">Kupfer</td><td width=\"50%\">6,20 € kg</td></tr>\
        <tr><td>Ankaufspreise sind Tages-/mengenabhängig</td><td></td></tr>\
        <tr><td>Messing</td><td>3,95 kg</td></tr>\
        </tbody></table>";

    #[test]
    fn thead_row_disclaimer_and_euroless_cell() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], ("Milberry".to_owned(), 7.0, "EUR/kg"));
        assert_eq!(rows[1], ("Kupfer".to_owned(), 6.2, "EUR/kg"));
        // No € sign, still kg: parses (bespoke unit rule).
        assert_eq!(rows[2], ("Messing".to_owned(), 3.95, "EUR/kg"));
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Tages"), "{skips:?}");
        assert_eq!(unit_of("7,00 € kg"), Some("EUR/kg"));
        assert_eq!(unit_of("3,95 kg"), Some("EUR/kg"));
        assert_eq!(unit_of("4 € pro Sack"), None);
        // Wrong table (no € header) and missing table: loud errors.
        assert!(parse("<table class=\"jw-table\"><tr><td>Nav</td></tr></table>").is_err());
        assert!(parse("<div>Redesign ohne Tabelle</div>").is_err());
    }

    #[test]
    fn mapping_covers_all_live_labels() {
        assert_eq!(grade_for("Milberry"), Some(("kupfer-millberry", "")));
        assert_eq!(grade_for("Kupfer"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Kabel"), Some(("kabel-kupfer", "")));
        assert_eq!(grade_for("Alu"), Some(("aluminium-gemischt", "")));
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Edelstahl"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h2 class=\"jw-heading-200\">Angaben gemäß § 5 TMG</h2>\
            <h1 class=\"jw-heading-130\">Schrott-Recycle Meikel&nbsp;</h1>\
            <p>Schrottabholung und Containerdienst&nbsp;<br />Timo Warth<br />Vatterstra&szlig;e 23</p>\
            <p>60386 Frankfurt<br />Deutschland</p>\
            <h2 class=\"jw-heading-100\">Kontakt</h2>\
            <p>Telefon: 01776310559<br />E-Mail: schrottfrankfurt1@gmail.com<br />\
            Website: www.schrott-recycling-meikel.de</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Vatterstraße 23");
        assert_eq!(info.postcode, "60386");
        assert_eq!(info.city, "Frankfurt");
        assert_eq!(info.phone, "01776310559");
        assert_eq!(info.email, "schrottfrankfurt1@gmail.com");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<p>Neu hier</p>").is_err());
        assert!(extract_info("<h2>Kontakt</h2><p>Telefon: 1</p>").is_err());
    }
}
