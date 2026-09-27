//! Schiefer & Co. Scheideanstalt (Hamburg-St. Georg): exact per-gram
//! sell-side fine-metal notations in the header ticker (`div#kursen`:
//! `div#au`/`#ag`/`#pt`/`#pd` carry the symbol in `<b>` and the quote in
//! `<span>`; `div#partner` carries "Notierungen, €/g" plus the quote time).
//!
//! One row per metal (gold/silber/platin/palladium, EUR/g — the catalog
//! unit, so nothing is ever converted). The big kursdaten table quotes
//! silver per kg and mixes buy/sell/fixing rows, so it stays unread on
//! purpose: the ticker IS the live €/g notation the page advertises
//! ("Verkaufspreise für unverarbeitete Feinmetalle").
//!
//! Ticker trap: quotes use a dot decimal with exactly three places
//! ("127.730" = 127.73 €/g — the table's Verkaufspreis unverarbeitet
//! reads 127.73). Shared `parse_eur` would take that for a thousands
//! group (127730, a 1000x error), so this handler re-marks the
//! ticker shape to a comma before delegating — bespoke, tested, and
//! confined to this file.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
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
            // Unknown ticker symbol: keep the quote as evidence in the
            // skip, never drop it silently.
            None => skipped_labels.push(format!(
                "{label} ({}, {unit}, kein Katalogmaterial: Tickersymbol)",
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

/// Explicit ticker symbol → (material, variant). Fine-metal trade prices,
/// no fineness on the page, so the variant stays standard ("").
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    match label.trim() {
        "Au" => Some(("gold", "")),
        "Ag" => Some(("silber", "")),
        "Pt" => Some(("platin", "")),
        "Pd" => Some(("palladium", "")),
        _ => None,
    }
}

/// Parse the ticker window between `div#kursen` and the `div#partner`
/// unit/date line. Returns (published_at, rows, unit_skips).
fn parse(
    html: &str,
) -> Result<
    (
        Option<String>,
        Vec<(String, f64, &'static str)>,
        Vec<String>,
    ),
    IngestError,
> {
    let start = html
        .find("id=\"kursen\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Notierungs-Ticker fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("id=\"unverbindlich\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Notierungs-Ticker unvollständig".to_owned(),
        })?;
    let window = &tail[..end];
    // The unit anchor must read €/g: a per-kilo quote recorded as
    // per-gram would be a 1000x error, and silver's table column is €/kg.
    let unit = unit_of(window).ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Notierungs-Einheit fehlt".to_owned(),
    })?;
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let sym_sel = Selector::parse("div#au, div#ag, div#pt, div#pd").expect("valid selector");
    let b_sel = Selector::parse("b").expect("valid selector");
    let span_sel = Selector::parse("span").expect("valid selector");
    let mut rows = Vec::new();
    let mut unit_skips = Vec::new();
    for cell in frag.select(&sym_sel) {
        let symbol = cell
            .select(&b_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let quote = cell
            .select(&span_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();
        if symbol.is_empty() || symbol.len() > 120 {
            continue;
        }
        let Some(price) = parse_tick(&quote) else {
            continue;
        };
        // Re-check the unit per quote: anything but €/g skips loudly at
        // the call site instead of silently defaulting.
        if unit_of(&quote).is_some_and(|u| u != unit) {
            unit_skips.push(format!(
                "{symbol} (Einheit unverständlich: {})",
                quote.trim()
            ));
            continue;
        }
        rows.push((symbol, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Notierungen leer".to_owned(),
        });
    }
    let published_at = find_date(window);
    Ok((published_at, rows, unit_skips))
}

/// Bespoke unit matcher for THIS ticker: the `div#partner` line reads
/// "Notierungen, €/g". Only €/g exists here — anything else skips loudly
/// at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase().replace(' ', "");
    if lower.contains("€/g") {
        Some("EUR/g")
    } else {
        None
    }
}

/// Bespoke quote parser for THIS ticker: three decimal places behind a
/// dot ("127.730", "1.961") — a decimal point, never a thousands group
/// (cross-checked against the table's Verkaufspreis unverarbeitet:
/// 127.73). Anything else delegates to shared `parse_eur`.
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

/// Bespoke date finder for THIS ticker: "Notierungen, €/g" followed by
/// "27.09.2026 21:00" in `div#partner` (raw markup glues the date to the
/// closing tag, so this scans bytes for a dd.mm.yyyy shape instead of
/// splitting whitespace). No anchor → None (the observation age stays the
/// provenance).
fn find_date(window: &str) -> Option<String> {
    let (_, after) = window.split_once("Notierungen,")?;
    let b = after.as_bytes();
    let mut i = 0;
    while i + 10 <= b.len() {
        // Byte-safe: non-boundary slices are skipped, never panicked on.
        if let Some(t) = after.get(i..i + 10) {
            let is_date = t.as_bytes().iter().enumerate().all(|(k, c)| match k {
                2 | 5 => *c == b'.',
                _ => c.is_ascii_digit(),
            });
            if is_date {
                return parse_de_date(&t[0..2], &t[3..5], &t[6..10]);
            }
        }
        i += 1;
    }
    None
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
    use super::{extract_info, find_date, grade_for, parse, parse_tick, unit_of};

    // Real shape of the live ticker (div ids, b/span split, the
    // "Notierungen, €/g" line with quote time), trimmed to two metals.
    const FIXTURE: &str = "<div class=\"kursen-block\"><div id=\"kursen\">\
        <div id=\"pd\"><b>Pd</b><br/><span>38.800</span></div>\
        <div id=\"au\"><b>Au</b><br/><span>127.730</span></div>\
        <div id=\"partner\"><div><b>Notierungen, €/g</b>27.09.2026 21:00</div></div>\
        </div><div id=\"unverbindlich\"><small>Verkaufspreise für unverarbeitete Feinmetalle. Unverbindliche Angaben.</small></div></div>";

    #[test]
    fn ticker_rows_unit_and_date() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-27T00:00:00+00:00"));
        assert_eq!(rows.len(), 2);
        assert!(skips.is_empty());
        // Dot-decimal, never thousands: 127.730 → 127.73, not 127730.
        assert_eq!(rows[0], ("Pd".to_owned(), 38.8, "EUR/g"));
        assert_eq!(rows[1], ("Au".to_owned(), 127.73, "EUR/g"));
        assert_eq!(unit_of("Notierungen, €/g"), Some("EUR/g"));
        assert_eq!(unit_of("pro Sack"), None);
        assert_eq!(find_date("ohne Datum"), None);
        assert!(parse("<div>Redesign ohne Ticker</div>").is_err());
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
