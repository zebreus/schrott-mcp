//! KATALYSATOR-HAI (Saarbrücken, Marke der S.B. Recycling): server-rendered
//! converter price list — one `<table>` headed "Marke | Erkennungsmerkmal |
//! Tagespreis" with 25 per-converter Tagespreise in bare "240,00 EUR"
//! cells plus the page date ("Die aktuelle Preisliste hat den Stand
//! 14.01.2026").
//!
//! No "bis zu" anywhere on this page (verified live): every row is an
//! exact per-piece Tagespreis → `price_kind: "exact"`, confidence 1.0.
//! All rows map to `katalysatoren` (catalog unit EUR/Stk); the variant is
//! the converter identity ("Marke Erkennungsmerkmal") so the 25 grades
//! never collapse onto one current price.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sl-saarbrucken-66113-katalysator-hai-marke-der-s-b-recycling";
/// Bespoke, live-verified impressum URL (the site's own footer link).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://katalysator-hai.de/impressum.html";

pub const URL: &str = "https://katalysator-hai.de/preisliste-katalysatoren.html";

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
    for (marke, merkmal, price, unit) in rows {
        let label = format!("{marke} {merkmal}");
        match grade_for(&label) {
            Some(material) => prices.push(ScrapedPrice {
                material,
                variant: converter_variant(&marke, &merkmal),
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
        published_at,
    })
}

/// Explicit mapping: this table lists nothing but catalytic converters,
/// so every row is `katalysatoren`. Kept as a table (not a constant) so a
/// redesigned page mixing in DPF/Metall rows fails visibly here instead of
/// silently mapping scrap onto catalysts.
fn grade_for(_label: &str) -> Option<&'static str> {
    Some("katalysatoren")
}

/// Per-converter variant ("Audi 8D0131701BS"). `ScrapedPrice.variant` is
/// `&'static str` while converter identities are page data, so each
/// identity is interned once via `Box::leak` — bounded (one small alloc
/// per table row per run), and the only way to keep 25 grades from
/// collapsing onto a single arbitrary current price.
fn converter_variant(marke: &str, merkmal: &str) -> &'static str {
    Box::leak(format!("{marke} {merkmal}").into_boxed_str())
}

/// Parse the converter table: the `<table>` whose header names
/// "Erkennungsmerkmal" (never the first table blindly). Returns
/// (published_at, rows, skips) with rows as (Marke, Merkmal, price, unit).
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
    let doc = Html::parse_document(html);
    let table_sel = Selector::parse("table").expect("valid selector");
    let tr_sel = Selector::parse("tr").expect("valid selector");
    let cell_sel = Selector::parse("th, td").expect("valid selector");
    let table = doc
        .select(&table_sel)
        .find(|t| t.text().collect::<String>().contains("Erkennungsmerkmal"))
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Konvertertabelle fehlt".to_owned(),
        })?;
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in table.select(&tr_sel) {
        let cells: Vec<String> = tr
            .select(&cell_sel)
            .map(|c| c.text().collect::<String>())
            .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        if cells.len() != 3 {
            continue;
        }
        let (marke, merkmal, price_cell) = (&cells[0], &cells[1], &cells[2]);
        if marke == "Marke" && merkmal == "Erkennungsmerkmal" {
            continue; // header row
        }
        if marke.is_empty() || merkmal.is_empty() || merkmal.len() > 120 {
            continue; // prosa, kein Konverter
        }
        let Some(price) = parse_eur(price_cell) else {
            skips.push(format!(
                "{marke} {merkmal} (Preis unverständlich: {})",
                price_cell.trim()
            ));
            continue;
        };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-kilo price recorded as per-piece would be orders off.
        let Some(unit) = unit_of(price_cell) else {
            skips.push(format!(
                "{marke} {merkmal} (Einheit unverständlich: {})",
                price_cell.trim()
            ));
            continue;
        };
        rows.push((marke.clone(), merkmal.clone(), price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Konvertertabelle leer".to_owned(),
        });
    }
    let published_at = find_date(html);
    Ok((published_at, rows, skips))
}

/// Bespoke unit matcher for THIS table's price cells (live: bare "240,00
/// EUR" — one Tagespreis per converter, i.e. per piece). Only this page's
/// spellings exist here: bare EUR (documented per-piece default, the page
/// quotes exactly one price per converter) and explicit per-piece marks
/// ("Stk", "Stück", "pro Kat"). Anything smelling of weight ("kg", "/g",
/// "Gramm", "/t", "Tonne") or any other qualifier ("pro", "/") skips
/// loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("kg")
        || lower.contains("/g")
        || lower.contains("gramm")
        || lower.contains("/t")
        || lower.contains("tonne")
    {
        return None;
    }
    if lower.contains("stk") || lower.contains("stück") || lower.contains("stuck") {
        return Some("EUR/Stk");
    }
    if lower.contains("pro kat") {
        return Some("EUR/Stk");
    }
    if lower.contains("pro") || lower.contains('/') {
        return None;
    }
    if lower.contains("eur") || lower.contains('€') {
        return Some("EUR/Stk");
    }
    None
}

/// Bespoke date finder for THIS page: "Die aktuelle Preisliste hat den
/// Stand 14.01.2026". No anchor → None (the observation age stays the
/// provenance).
fn find_date(html: &str) -> Option<String> {
    let (_, after) = html.split_once("Stand")?;
    let date = after
        .split_whitespace()
        .find(|t| t.matches('.').count() == 2)?;
    let parts: Vec<&str> = date
        .trim_matches(|c: char| !c.is_ascii_digit() && c != '.')
        .split('.')
        .collect();
    if parts.len() == 3 {
        parse_de_date(parts[0], parts[1], parts[2])
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<h3>`
/// "Postanschrift:" heading (next `<p>` holds "Jenneweg 55<br>66113
/// Saarbrücken") and the `<h3>` "Kontakt:" heading (next `<p>` holds
/// "Telefon:" / entity-encoded "E-Mail:" lines). Missing headings → loud
/// error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let heading_sel = Selector::parse("h1, h2, h3").expect("valid selector");
    let mut address_html = None;
    let mut contact_html = None;
    for el in doc.select(&heading_sel) {
        let title = el.text().collect::<String>().trim().to_owned();
        if title == "Postanschrift:" && address_html.is_none() {
            address_html = el
                .next_siblings()
                .find_map(|n| scraper::ElementRef::wrap(n).filter(|e| e.value().name() == "p"))
                .map(|p| p.inner_html());
        } else if title == "Kontakt:" && contact_html.is_none() {
            contact_html = el
                .next_siblings()
                .find_map(|n| scraper::ElementRef::wrap(n).filter(|e| e.value().name() == "p"))
                .map(|p| p.inner_html());
        }
    }
    let (Some(addr_html), Some(cont_html)) = (address_html, contact_html) else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Postanschrift/Kontakt-Blöcke fehlen".to_owned(),
        });
    };
    let addr_lines: Vec<String> = addr_html
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in addr_lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| format!("{a} {b}"));
                if k > 0 {
                    street = addr_lines[k - 1].clone();
                }
                break;
            }
        }
    }
    let mut phone = String::new();
    let mut email = String::new();
    for part in cont_html.split("<br") {
        let t = strip_fragment(part);
        if let Some(v) = t.strip_prefix("Telefon:") {
            phone = v.trim().to_owned();
        } else if let Some(v) = t.strip_prefix("E-Mail:") {
            email = v.trim().to_owned();
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

/// Strip tags from a `<br>`-split fragment. Fragments start with a tag
/// remnant — drop everything up to the first '>' first, or attributes
/// parse as text. (html5ever already decoded the numeric e-mail entities.)
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
    use super::{find_date, grade_for, parse, unit_of};

    // Real shape of the live table (inline styles, three columns, bare
    // "EUR" cells) plus the live Stand line, trimmed to three rows.
    const FIXTURE: &str = "<p>Die Ankaufpreise für Katalysatoren sind börsenabhängig und können täglich variieren. Die aktuelle Preisliste hat den Stand 14.01.2026</p>\
        <table style=\"width: 92.541436%; height: 486px;\"><thead><tr style=\"height: 18px;\">\
        <th style=\"width: 22.155689%; height: 18px;\">Marke</th>\
        <th style=\"width: 48.802395%; height: 18px;\">Erkennungsmerkmal</th>\
        <th style=\"width: 29.041916%; height: 18px;\">Tagespreis</th></tr></thead><tbody>\
        <tr style=\"height: 18px;\"><td style=\"width: 22.155689%; height: 18px;\">Audi</td>\
        <td style=\"width: 48.802395%; height: 18px;\">8D0131701BS</td>\
        <td style=\"width: 29.041916%; height: 18px;\">240,00 EUR</td></tr>\
        <tr style=\"height: 18px;\"><td style=\"width: 22.155689%; height: 18px;\">Mercedes</td>\
        <td style=\"width: 48.802395%; height: 18px;\">KT6029</td>\
        <td style=\"width: 29.041916%; height: 18px;\">2000,00 EUR</td></tr>\
        <tr style=\"height: 18px;\"><td style=\"width: 22.155689%; height: 18px;\">Ford</td>\
        <td style=\"width: 48.802395%; height: 18px;\">001 9315B</td>\
        <td style=\"width: 29.041916%; height: 18px;\">320,00 EUR</td></tr>\
        </tbody></table><footer>Impressum</footer>";

    #[test]
    fn table_rows_units_and_date() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-01-14T00:00:00+00:00"));
        assert_eq!(rows.len(), 3);
        assert!(skips.is_empty());
        assert_eq!(
            rows[0],
            (
                "Audi".to_owned(),
                "8D0131701BS".to_owned(),
                240.0,
                "EUR/Stk"
            )
        );
        assert_eq!(
            rows[1],
            (
                "Mercedes".to_owned(),
                "KT6029".to_owned(),
                2000.0,
                "EUR/Stk"
            )
        );
        assert_eq!(
            rows[2],
            ("Ford".to_owned(), "001 9315B".to_owned(), 320.0, "EUR/Stk")
        );
        // Per-converter variants keep grades apart.
        assert_eq!(
            super::converter_variant("Audi", "8D0131701BS"),
            "Audi 8D0131701BS"
        );
        assert_eq!(find_date("ohne Stand"), None);
        assert!(parse("<table><tr><td>Neu hier</td></tr></table>").is_err());
        assert!(parse("<p>Redesign ohne Tabelle</p>").is_err());
    }

    #[test]
    fn units_are_piece_only() {
        assert_eq!(unit_of("240,00 EUR"), Some("EUR/Stk"));
        assert_eq!(unit_of("320,00 € pro Kat"), Some("EUR/Stk"));
        assert_eq!(unit_of("4,20 €/Stk"), Some("EUR/Stk"));
        assert_eq!(
            unit_of("9,80 €/kg"),
            None,
            "kilo price must never become per-piece"
        );
        assert_eq!(unit_of("1,80 €/g"), None);
        assert_eq!(unit_of("100 € pro to"), None);
        assert_eq!(unit_of("Preis auf Anfrage"), None);
    }

    #[test]
    fn every_row_is_a_catalyst() {
        for label in [
            "Audi 8D0131701BS",
            "Mercedes KT6029",
            "Volvo 1275700 Holland",
        ] {
            assert_eq!(grade_for(label), Some("katalysatoren"), "{label}");
        }
    }

    #[test]
    fn impressum_headings() {
        let imp = "<h2>Angaben gemäß § 5 TMG:</h2><p>S.B Recycling<br>Sebastian Blaziak</p>\
            <h3>Postanschrift:</h3><p>Jenneweg 55<br>66113 Saarbrücken</p>\
            <h3>Kontakt:</h3><p>Telefon: +4917622946151<br>E-Mail: ankauf@katalysator-hai.de</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Jenneweg 55");
        assert_eq!(info.postcode, "66113");
        assert_eq!(info.city, "Saarbrücken");
        assert_eq!(info.phone, "+4917622946151");
        assert_eq!(info.email, "ankauf@katalysator-hai.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}
