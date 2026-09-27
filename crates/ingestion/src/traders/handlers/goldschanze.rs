//! Goldschanze GmbH (Hamburg-Sternschanze): exact per-gram purchase
//! prices in two Elementor tables (`table.uael-table` with "Reinheit" /
//! "Preis" headers) under the "Goldpreis …€/gr" and "Silberpreis …€/Gr"
//! section headings — 9 gold rows ("999 (24K)" → 115,95€/gr) + 6 silver
//! rows ("999" → 1,37€/gr). The metal comes from the section heading
//! above each table (never table order alone); the spot `<h2>`s
//! themselves never pair into rows. Date in "Ankaufpreise 26.09.2026 /
//! 10:00". Quoted unit honestly EUR/g (catalog units match).

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "hh-sternschanze-goldschanze";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://goldschanze.de/impressum/";

pub const URL: &str = "https://goldschanze.de/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (published_at, rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (metal, label, price, unit) in rows {
        match grade_for(&metal, &label) {
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
            None => skipped_labels.push(format!("{label} ({price:.2} {unit}, kein Katalogmaterial")),
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
        published_at,
    })
}

/// Explicit (section metal, label) → (material, variant). Fineness rides
/// in the variant ("999 (24K)" → `gold`/`999`). "750" exists in both
/// tables, so the section metal (not the digits) decides gold vs silver.
fn grade_for(metal: &str, label: &str) -> Option<(&'static str, &'static str)> {
    let variant = fineness(label);
    match metal {
        "gold" => Some(("gold", variant)),
        "silber" => Some(("silber", variant)),
        _ => None,
    }
}

/// First fineness run in the label ("999 (24K)" → "999").
fn fineness(label: &str) -> &'static str {
    let l = label.to_lowercase();
    for fin in ["999", "986", "925", "916", "900", "875", "835", "800", "750", "625", "585", "375", "333"] {
        if l.contains(fin) {
            return fin;
        }
    }
    ""
}

/// Bespoke unit rule for THESE tables: every price cell quotes "€/gr"
/// (case-insensitive). Anything else skips loudly, never a default.
fn unit_of(cell: &str) -> Option<&'static str> {
    if cell.to_lowercase().contains("€/gr") {
        Some("EUR/g")
    } else {
        None
    }
}

/// Parse the two Reinheit/Preis tables. Each table's metal comes from
/// the nearest preceding "Goldpreis"/"Silberpreis" heading — never from
/// table order. Returns (published_at, rows, skips) with rows as
/// (metal, label, price, unit).
fn parse(
    html: &str,
) -> Result<(Option<String>, Vec<(String, String, f64, &'static str)>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let table_sel = Selector::parse("table.uael-table").expect("valid selector");
    let tr_sel = Selector::parse("tbody tr").expect("valid selector");
    let td_sel = Selector::parse("td").expect("valid selector");
    let tables: Vec<_> = doc
        .select(&table_sel)
        .filter(|t| t.text().collect::<String>().contains("Reinheit"))
        .collect();
    if tables.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Reinheit-Tabellen fehlen".to_owned() });
    }
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    // Tables in document order; the cursor walks the raw HTML forward so
    // each table is located by its own first row (a bare "999" must not
    // match the gold table's "999 (24K)" again).
    let mut cursor = 0;
    for table in tables {
        let first: String = table
            .select(&Selector::parse("tbody tr td").expect("valid selector"))
            .next()
            .map(|c| c.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let rel = html[cursor..].find(&first);
        let Some(rel) = rel else {
            skips.push("Tabelle ohne Metall-Abschnitt".to_owned());
            continue;
        };
        let pos = cursor + rel;
        cursor = pos + first.len();
        let head = &html[..pos];
        let gold = head.rfind("Goldpreis").unwrap_or(0);
        let silber = head.rfind("Silberpreis").unwrap_or(0);
        if gold == 0 && silber == 0 {
            skips.push("Tabelle ohne Metall-Abschnitt".to_owned());
            continue;
        }
        let metal = if gold > silber { "gold" } else { "silber" };
        for tr in table.select(&tr_sel) {
            let cells: Vec<String> = tr
                .select(&td_sel)
                .map(|c| c.text().collect::<String>())
                .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
                .collect();
            if cells.len() < 2 || cells[0].is_empty() {
                continue;
            }
            let (label, price_cell) = (cells[0].clone(), cells[1].clone());
            let Some(price) = parse_eur(&price_cell) else { continue };
            let Some(unit) = unit_of(&price_cell) else {
                skips.push(format!("{label} (Einheit unverständlich: {price_cell})"));
                continue;
            };
            rows.push((metal.to_owned(), label, price, unit));
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Reinheit-Tabellen leer".to_owned() });
    }
    Ok((date_in(html), rows, skips))
}

/// "Ankaufpreise 26.09.2026 / 10:00" → RFC 3339 UTC midnight.
fn date_in(html: &str) -> Option<String> {
    let i = html.find("Ankaufpreise")?;
    let w = &html[i..(i + 60).min(html.len())];
    let bytes = w.as_bytes();
    let mut k = 0;
    while k + 10 <= bytes.len() {
        if bytes[k].is_ascii_digit()
            && bytes[k + 2] == b'.'
            && bytes[k + 5] == b'.'
            && bytes[k + 6..].iter().take(4).all(|c| c.is_ascii_digit())
        {
            let (d, m, y) = (&w[k..k + 2], &w[k + 3..k + 5], &w[k + 6..k + 10]);
            if let Some(rfc) = parse_de_date(d, m, y) {
                return Some(rfc);
            }
        }
        k += 1;
    }
    None
}

/// Bespoke contact extraction for THIS impressum only: firm name,
/// street, PLZ/city, phone and email live in SEPARATE `<p>` elements
/// ("Goldschanze GmbH" / "Geschäftsführer: …" / "Schanzenstraße 115<br>
/// "20357 Hamburg<br>Tel:" / "E-Mail: …"). The street `<p>` is the one
/// holding a PLZ line; street is its previous line IN THE SAME `<p>`.
/// Missing firm anchor → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    if !doc
        .root_element()
        .text()
        .collect::<String>()
        .contains("Goldschanze GmbH")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmen-Block fehlt".to_owned(),
        });
    }
    let p_sel = Selector::parse("p").expect("valid selector");
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let (mut phone, mut email) = (String::new(), String::new());
    for el in doc.select(&p_sel) {
        let lines: Vec<String> = el
            .inner_html()
            .split("<br")
            .map(|s| strip_fragment(s))
            .filter(|s| !s.is_empty())
            .collect();
        for (k, line) in lines.iter().enumerate() {
            let mut it = line.split_whitespace();
            if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                if postcode.is_empty()
                    && pc.len() == 5
                    && pc.chars().all(|c| c.is_ascii_digit())
                {
                    postcode = pc.to_owned();
                    city = ci.to_owned();
                    if k > 0 {
                        street = lines[k - 1].clone();
                    }
                    continue;
                }
            }
            if phone.is_empty() {
                if let Some(v) = line.strip_prefix("Tel:").or_else(|| line.strip_prefix("Telefon:")) {
                    phone = v.trim().to_owned();
                }
            }
            if email.is_empty() {
                if let Some(v) = line.strip_prefix("E-Mail:") {
                    email = v.trim().to_owned();
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

/// Strip tags from a `<br`-split fragment (drop up to the first '>').
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
    use super::{grade_for, parse};

    // Real Elementor table shape (Reinheit/Preis, €/gr cells), trimmed.
    const FIXTURE: &str = "<h2>Goldpreis 120,95€/gr</h2>\
        <table class=\"uael-table\"><thead><tr><th>Reinheit</th><th>Preis</th></tr></thead><tbody>\
        <tr><td>999 (24K)</td><td>115,95€/gr</td></tr>\
        <tr><td>750 (18K)</td><td>83,77€/gr</td></tr></tbody></table>\
        <h2>Silberpreis 1,81€/Gr</h2>\
        <table class=\"uael-table\"><thead><tr><th>Reinheit</th><th>Preis</th></tr></thead><tbody>\
        <tr><td>999</td><td>1,37€/gr</td></tr>\
        <tr><td>750</td><td>0,95€/gr</td></tr></tbody></table>\
        <h2>Ankaufpreise 26.09.2026 / 10:00</h2>";

    #[test]
    fn tables_parse_with_section_metal() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 4);
        assert!(skips.is_empty());
        assert_eq!(rows[0], ("gold".to_owned(), "999 (24K)".to_owned(), 115.95, "EUR/g"));
        assert_eq!(rows[1].0, "gold");
        // Same digits, other table: section decides, not digits.
        assert_eq!(rows[2], ("silber".to_owned(), "999".to_owned(), 1.37, "EUR/g"));
        assert_eq!(rows[3], ("silber".to_owned(), "750".to_owned(), 0.95, "EUR/g"));
        assert_eq!(
            published_at.as_deref(),
            Some("2026-09-26T00:00:00+00:00")
        );
        assert!(parse("<div>Kein Gold hier</div>").is_err());
    }

    #[test]
    fn metal_decides_fineness_variant() {
        assert_eq!(grade_for("gold", "999 (24K)"), Some(("gold", "999")));
        assert_eq!(grade_for("gold", "750 (18K)"), Some(("gold", "750")));
        assert_eq!(grade_for("silber", "999"), Some(("silber", "999")));
        assert_eq!(grade_for("silber", "750"), Some(("silber", "750")));
        assert_eq!(grade_for("platin", "999"), None);
    }

    #[test]
    fn impressum_separate_paragraphs() {
        // Real live shape: firm, manager, street block and email live in
        // SEPARATE <p> elements (a merged single-<p> fixture would pass
        // here and fail live — exactly this bug).
        let imp = "<p>Goldschanze GmbH</p><p>Geschäftsführer: Herr Semih Dülger</p>\
            <p>Schanzenstraße 115<br />20357 Hamburg<br />Tel: 040 27806668</p>\
            <p>E-Mail: info@goldschanze.de<br />Registergericht: Amtsgericht Hamburg</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Schanzenstraße 115");
        assert_eq!(info.postcode, "20357");
        assert_eq!(info.city, "Hamburg");
        assert_eq!(info.phone, "040 27806668");
        assert_eq!(info.email, "info@goldschanze.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}
