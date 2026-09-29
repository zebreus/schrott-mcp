//! Uwe Hanusa Schrott- und Metallhandel (Vechelde): Tagespreisliste auf
//! der Homepage — Label/Preis-`<p>`-Paare unter der Überschrift "Aktuelle
//! Preise" ("Kupfer Milberry" → "EUR 9,00-9,60"). Alle Zeilen sind
//! Von-bis-Spannen ("Der Preis richtet sich nach der angelieferten
//! Menge"), daher `price_kind: "range"` mit beiden Bounds zu
//! confidence 0.5 (VANA-Muster) — nie ein stilles exact. Seiten-Default
//! ist kg ("Alle Preise in EUR pro kg"); nur Mischschrott trägt
//! explizit `€/t`. Seitendatum ("Letzte Aktualisierung am 18.09.2026")
//! → Outcome-`published_at`.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "ni-vechelde-uwe-hanusa-schrott-und-metallhandel";
/// Bespoke, live-verified impressum URL (Footer-Navigation "Impressum /
/// Datenschutz", 200 am 28.09.2026). A move fails the step loudly (fix
/// the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://hanusa-schrott.de/impressum-datenschutz/";

/// The Tagespreise board lives on the homepage (single-page price list)
/// — this IS the price page here, not a fallback.
pub const URL: &str = "https://hanusa-schrott.de/";

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
    for (label, min, max, unit) in rows {
        // A zero row is a missing price, never a 0-EUR observation.
        if max == 0.0 {
            skipped_labels.push(format!("{label} (Preis 0,00)"));
            continue;
        }
        match grade_for(&label) {
            Some((material, variant)) => {
                // Von-bis ranges mirror the upto pattern: the bounds are
                // honest data with their own kind, never a silent exact.
                let (price_kind, confidence) = if (min - max).abs() < f64::EPSILON {
                    ("exact", Some(1.0))
                } else {
                    ("range", Some(0.5))
                };
                let (price_min, price_max) = if price_kind == "range" {
                    (Some(min), Some(max))
                } else {
                    (None, None)
                };
                prices.push(ScrapedPrice {
                    material,
                    variant,
                    price: max,
                    currency: "EUR",
                    unit,
                    price_kind,
                    price_min,
                    price_max,
                    confidence,
                    label,
                });
            }
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

/// Explicit label → (material, variant) mapping, specific before generic
/// ("kupfer" would otherwise catch "Kupfer Milberry" and
/// "Kupfer Haushaltskabel"). Generics land on generic materials (bare
/// "Kupfer" → `kupfer-gemischt`, bare "Aluminium" →
/// `aluminium-gemischt`), never on specific grades. One cable row only →
/// variant `''`; the raw label stays in `notes` via `label`.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("milberry") || l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kabel") {
        Some(("kabel-kupfer", ""))
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("v2a") || l.contains("edelstahl") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("alu") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else {
        None
    }
}

fn parse(
    html: &str,
) -> Result<
    (
        Option<String>,
        Vec<(String, f64, f64, &'static str)>,
        Vec<String>,
    ),
    IngestError,
> {
    // Window: the price board only — from its heading to the page-stated
    // update date. Footer/service prose must never pair with labels into
    // phantom prices.
    let start = html
        .find("Aktuelle Preise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbox fehlt".to_owned(),
        })?;
    let date_anchor = html[start..].find("Letzte Aktualisierung am").ok_or_else(|| {
        IngestError::Parse {
            url: URL.to_owned(),
            detail: "Datumsanker fehlt".to_owned(),
        }
    })?;
    let window = &html[start..start + date_anchor + 600.min(html.len() - start - date_anchor)];
    let published_at = date_in(window);
    // Collect <p> texts in order (Jimdo: label and price are sibling
    // grid columns, each a <p>).
    let mut texts = Vec::new();
    let mut rest = window;
    while let Some(a) = rest.find("<p") {
        let after = &rest[a..];
        let Some(b) = after.find('>') else { break };
        let after = &after[b + 1..];
        let Some(c) = after.find("</p>") else { break };
        let mut t = after[..c].to_owned();
        loop {
            let Some(x) = t.find('<') else { break };
            let Some(y) = t[x..].find('>') else { break };
            t.replace_range(x..x + y + 1, " ");
        }
        let t = t
            .replace("&nbsp;", " ")
            .replace("&#160;", " ")
            .replace(['\u{a0}'], " ");
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if !t.is_empty() {
            texts.push(t);
        }
        rest = &after[c + 4..];
    }
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    let mut pending: Vec<String> = Vec::new();
    for t in texts {
        if is_header(&t) {
            pending.clear();
            // The board ends at the bulk-conditions sentence: anything
            // after it is contact/date prose, never prices.
            if t.contains("Konditionen") || t.contains("Anfrage") {
                break;
            }
        } else if is_price(&t) {
            let label = pending.join(" ").trim().to_owned();
            pending.clear();
            if label.is_empty() {
                continue;
            }
            let Some((min, max)) = split_range(&t) else {
                continue;
            };
            let Some(unit) = unit_of(&t) else {
                skips.push(format!("{label} (Einheit unverständlich: {t})"));
                continue;
            };
            rows.push((label, min, max, unit));
        } else {
            pending.push(t);
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preispaare".to_owned(),
        });
    }
    Ok((published_at, rows, skips))
}

/// Bespoke range splitter for THIS board's price cells (live:
/// "EUR 9,00-9,60", "EUR 130,00 -180,00€/t"). Returns (min, max).
fn split_range(cell: &str) -> Option<(f64, f64)> {
    let t = cell.replace(['\u{a0}'], " ");
    if let Some((a, b)) = t.split_once('-') {
        Some((parse_eur(a)?, parse_eur(b)?))
    } else {
        let p = parse_eur(&t)?;
        Some((p, p))
    }
}

/// Bespoke unit matcher for THIS board's price cells. Only kg/t exist
/// here: an explicit `/t` (Mischschrott) is per-tonne, everything else
/// follows the documented page default ("Alle Preise in EUR pro kg") —
/// except explicitly foreign units (`/`, `"pro"` + unknown word), which
/// skip loudly at the call site instead of inheriting the default.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("/t") {
        Some("EUR/t")
    } else if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower.contains('/')
        || lower
            .split(|c: char| !c.is_alphanumeric())
            .any(|w| w == "pro")
    {
        None
    } else {
        // Documented page default: "Alle Preise in EUR pro kg".
        Some("EUR/kg")
    }
}

fn is_price(t: &str) -> bool {
    // Price cells on THIS page always carry the currency ("EUR …",
    // "…€/t") — labels never do ("Edelstahl V2A" parses as 2.0 via
    // parse_eur without this anchor and would eat its own pair).
    (t.contains('€') || t.contains("EUR")) && split_range(t).is_some()
}

fn is_header(t: &str) -> bool {
    // Any euro text without digits is a header, not a label; long prose
    // is never a label either.
    if (t.contains('€') || t.contains("EUR")) && split_range(t).is_none() {
        return true;
    }
    matches!(t, "Aktuelle Preise")
        || t.contains("tagesaktuellen Preise")
        || t.contains("Konditionen")
        || t.contains("Anfrage")
        || t.contains("Aktualisierung")
        || t.len() > 120
}

fn date_in(window: &str) -> Option<String> {
    let bytes = window.as_bytes();
    let mut i = 0;
    while i + 10 <= bytes.len() {
        if bytes[i].is_ascii_digit()
            && bytes[i + 2] == b'.'
            && bytes[i + 5] == b'.'
            && bytes[i + 6..].iter().take(4).all(|c| c.is_ascii_digit())
        {
            let (d, m, y) = (
                &window[i..i + 2],
                &window[i + 3..i + 5],
                &window[i + 6..i + 10],
            );
            if let Some(rfc) = parse_de_date(d, m, y) {
                return Some(rfc);
            }
        }
        i += 1;
    }
    None
}

/// Bespoke contact extraction for THIS impressum only: Jimdo nests the
/// `<h3>Impressum</h3>` heading and the address `<p>` blocks in separate
/// `<div>`s, so no sibling walk can reach them — search document-wide
/// instead, with the heading as a mandatory anchor. The street `<p>`
/// ("Uwe Hanusa, Inh. Silke Hanusa" / "Brackestraße 9") is the first
/// `<p>` mentioning "Brackestraße" after the heading; the next `<p>`
/// carries "PLZ Ort", mail and "Fon". The mail address hides behind
/// Cloudflare protection in the link text, so it comes from the
/// `title="mailto:…"` (or `href="mailto:…"`) attribute, never from token
/// split. Missing anchors mean the page changed shape → loud error,
/// never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h3 = Selector::parse("h3").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let a = Selector::parse("a").expect("valid selector");
    let missing = |detail: &str| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: detail.to_owned(),
    };
    if doc
        .select(&h3)
        .find(|h| h.text().collect::<String>().trim() == "Impressum")
        .is_none()
    {
        return Err(missing("Impressum-Block fehlt"));
    }
    let paras: Vec<_> = doc.select(&p).collect();
    // First street paragraph at/after the heading.
    let street_idx = paras
        .iter()
        .position(|e| e.text().collect::<String>().contains("Brackestraße"))
        .ok_or_else(|| missing("Adress-Block fehlt"))?;
    let lines_of = |e: scraper::ElementRef| {
        let mut lines = Vec::new();
        for part in e.inner_html().split("<br") {
            let t = strip_fragment(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
        lines
    };
    // p1: "Uwe Hanusa, Inh. Silke Hanusa" / "Brackestraße 9".
    let street = lines_of(paras[street_idx])
        .last()
        .cloned()
        .unwrap_or_default();
    // p2: "38159 Vechelde" / "Mail: …" / "Fon …" (next <p> with a PLZ).
    let city_para = paras[street_idx..]
        .iter()
        .find(|e| {
            e.text()
                .collect::<String>()
                .split_whitespace()
                .any(|w| w.len() == 5 && w.chars().all(|c| c.is_ascii_digit()))
        })
        .ok_or_else(|| missing("Adress-Block fehlt"))?;
    let (mut postcode, mut city, mut phone) = (String::new(), String::new(), String::new());
    for line in lines_of(*city_para) {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) && postcode.is_empty() {
                postcode = pc.to_owned();
                city = ci.to_owned();
                continue;
            }
        }
        if let Some(v) = line.strip_prefix("Fon") {
            phone = v.trim().to_owned();
        }
    }
    // Mail via mailto attribute (Cloudflare masks the link text).
    let mut email = String::new();
    for el in city_para.select(&a) {
        for attr in ["title", "href"] {
            if email.is_empty() {
                if let Some(v) = el.value().attr(attr) {
                    if let Some(addr) = v.strip_prefix("mailto:") {
                        email = addr.trim().to_owned();
                    }
                }
            }
        }
    }
    if street.is_empty() || postcode.is_empty() {
        return Err(missing("Adress-Block fehlt"));
    }
    Ok(TraderInfo {
        street,
        postcode,
        city,
        phone,
        email,
    })
}

/// Strip tags from a `<br>`-split fragment. Only fragments that START
/// with a tag remnant drop everything up to the first '>' — a fragment
/// starting with text ("Fon <a …>…") keeps its label.
fn strip_fragment(s: &str) -> String {
    let s = s.trim_start();
    let s = if s.starts_with('<') {
        match s.find('>') {
            Some(i) => &s[i + 1..],
            None => s,
        }
    } else {
        s
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
    use super::{date_in, grade_for, parse, split_range, unit_of};

    // Verbatim live texts of the board (28.09.2026: Stand 18.09.2026),
    // wrappers trimmed — the parser only sees heading-anchored <p> order.
    const FIXTURE: &str = "<h2>Aktuelle Preise</h2>\
        <p>An dieser Stelle finden Sie immer die tagesaktuellen Preise für Metalle (Alle Preise in EUR pro kg). Der Preis richtet sich nach der angelieferten Menge.</p>\
        <p>Kupfer Milberry</p><p>EUR 9,00-9,60</p>\
        <p>Kupfer</p><p>EUR 8,20-8,80</p>\
        <p>Messing</p><p>EUR 5,00-5,70</p>\
        <p>Kupfer Haushaltskabel</p><p>EUR 2,50-3,10</p>\
        <p>Blei</p><p>EUR 0,70-1,10</p>\
        <p>Zink</p><p>EUR 1,30-1,70</p>\
        <p>Edelstahl V2A</p><p>EUR 0,30-0,70</p>\
        <p>Aluminium</p><p>EUR 0,90-1,30</p>\
        <p>Mischschrott</p><p>EUR 130,00 -180,00€/t</p>\
        <p>Bitte sprechen Sie uns an, wenn Sie für größere Mengen individuelle Konditionen benötigen: info@hanusa-schrott.de</p>\
        <p>Letzte Aktualisierung am</p><p><strong>18.09.2026, 10:30 Uhr</strong></p>";

    #[test]
    fn board_nine_ranges_and_date() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-18T00:00:00+00:00"));
        assert!(skips.is_empty());
        assert_eq!(rows.len(), 9);
        assert_eq!(rows[0], ("Kupfer Milberry".to_owned(), 9.0, 9.6, "EUR/kg"));
        assert_eq!(rows[1], ("Kupfer".to_owned(), 8.2, 8.8, "EUR/kg"));
        assert_eq!(rows[2], ("Messing".to_owned(), 5.0, 5.7, "EUR/kg"));
        assert_eq!(
            rows[3],
            ("Kupfer Haushaltskabel".to_owned(), 2.5, 3.1, "EUR/kg")
        );
        assert_eq!(rows[8], ("Mischschrott".to_owned(), 130.0, 180.0, "EUR/t"));
    }

    #[test]
    fn range_splitter_and_units() {
        assert_eq!(split_range("EUR 9,00-9,60"), Some((9.0, 9.6)));
        assert_eq!(split_range("EUR 130,00 -180,00€/t"), Some((130.0, 180.0)));
        assert_eq!(split_range("EUR 5,00"), Some((5.0, 5.0)));
        assert_eq!(split_range("Preis auf Anfrage"), None);
        assert_eq!(unit_of("EUR 9,00-9,60"), Some("EUR/kg"));
        assert_eq!(unit_of("EUR 130,00 -180,00€/t"), Some("EUR/t"));
        // Explicitly foreign units skip loudly instead of inheriting kg.
        assert_eq!(unit_of("EUR 5,00 pro Sack"), None);
        assert_eq!(unit_of("EUR 5,00 / Stk"), None);
    }

    #[test]
    fn missing_anchors_and_zero_fail_loudly() {
        assert!(parse("<html><body><p>Neu hier</p></body></html>").is_err());
        // Date anchor gone → loud error, not a dateless success.
        let nodate = FIXTURE.replace("Letzte Aktualisierung am", "Stand");
        assert!(parse(&nodate).is_err());
        // Unknown unit skips loudly, row count drops by one.
        let sack = FIXTURE.replace("EUR 5,00-5,70", "EUR 5,00 pro Sack");
        let (_, rows, skips) = parse(&sack).expect("parses");
        assert_eq!(rows.len(), 8);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Messing") && skips[0].contains("Einheit"));
    }

    #[test]
    fn mapping_specific_before_generic() {
        assert_eq!(grade_for("Kupfer Milberry"), Some(("kupfer-millberry", "")));
        assert_eq!(
            grade_for("Kupfer Haushaltskabel"),
            Some(("kabel-kupfer", ""))
        );
        assert_eq!(grade_for("Kupfer"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Edelstahl V2A"), Some(("edelstahl-v2a", "")));
        assert_eq!(grade_for("Aluminium"), Some(("aluminium-gemischt", "")));
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
    }

    #[test]
    fn digit_labels_are_no_prices() {
        // "Edelstahl V2A" parses as 2.0 via parse_eur: without the
        // currency anchor it would eat its own pair (live failure).
        assert!(!super::is_price("Edelstahl V2A"));
        assert!(super::is_price("EUR 0,30-0,70"));
    }

    #[test]
    fn impressum_extracts_contact() {
        // Live nesting: heading and address <p>s sit in separate <div>s,
        // so only a document-wide search reaches them.
        let imp = "<div class=\"j-module n j-header \"><h3 class=\"\" id=\"cc-m-header-7717218011\">Impressum</h3></div>\
            <div class=\"j-module n j-text \"><p>Uwe Hanusa, Inh. Silke Hanusa<br/>Brackestraße 9</p>\
            <p>38159 Vechelde<br/><br/>Mail: <a href=\"/cdn-cgi/l/email-protection#xyz\" title=\"mailto:info@hanusa-schrott.de\"><span>[email protected]</span></a><br/>Fon 0 53 02 . 9 34 87 11</p>\
            <p> <br/>USt.-ID: DE 179365814</p></div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Brackestraße 9");
        assert_eq!(info.postcode, "38159");
        assert_eq!(info.city, "Vechelde");
        assert_eq!(info.phone, "0 53 02 . 9 34 87 11");
        assert_eq!(info.email, "info@hanusa-schrott.de");
        // Redesign without anchors fails loudly.
        assert!(super::extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(super::extract_info("<h3>Impressum</h3><p>Nur ein Absatz</p>").is_err());
    }

    #[test]
    fn date_scan() {
        assert_eq!(
            date_in("Letzte Aktualisierung am 18.09.2026, 10:30").as_deref(),
            Some("2026-09-18T00:00:00+00:00")
        );
        assert_eq!(date_in("kein Datum"), None);
    }
}
