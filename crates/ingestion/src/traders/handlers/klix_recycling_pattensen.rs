//! Klix Recycling (Pattensen): price table embedded in the homepage below
//! the "Aktuelle Preise" heading (`Mischschrott / Ab 150 € To` … `Edelstahl
//! V2A / Ab 0,70 € Kg`, live-verified 28.09.2026, 12 rows). The table
//! carries no header row, so the parse window runs from the heading anchor
//! to the "auf Anfrage" terminator and the table inside must yield rows —
//! a missing table or 0 rows is a loud error, never a silent success.
//! Every live row is "Ab …" (a floor, not a fixed quote), so each becomes
//! `price_kind: "approx"` with the bound as `price_min` at confidence 0.5
//! (ms_recycling_frankfurt "ab" precedent) — never a silent exact.
//! The page states no calendar date ("immer aktuelle Tagespreise"), so
//! `published_at` stays `None`. "Schwer Schrott" rides as
//! `stahlschrott-scheren`/`schwer` (koppe_strausberg "Schwere Schere"
//! precedent: heavy steel grade in the steel block at a steel price); the
//! arm sits below all copper arms so "Kupfer Schwer" still lands on
//! `kupfer-gemischt`. "Alu Blank/Neu" names neither profile, sheet nor
//! cast, so it lands on generic `aluminium-gemischt` with the trader's own
//! wording as variant — never a guessed profile grade.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "ni-pattensen-klix-recycling";
/// Bespoke, live-verified price URL (the price table lives on the
/// homepage). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const URL: &str = "https://klix-recycling.de/";
/// Bespoke, live-verified impressum URL (the site footer's own
/// "Impressum" link, canonical without trailing slash). A move fails the
/// step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://klix-recycling.de/impressum";

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
    for (label, price, price_min, price_max, price_kind, confidence, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit,
                price_kind,
                price_min,
                price_max,
                confidence,
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

/// Explicit label → (material, variant) mapping, specific before generic
/// ("kupfer" would otherwise catch "Kupfer Schwer" and "Kupfer Kabel").
/// Anything unlisted is skipped. The variant keeps the trader's own grade
/// wording so the two copper and the two aluminium rows never collapse.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kupfer") && l.contains("schwer") {
        Some(("kupfer-gemischt", "Schwer"))
    } else if l.contains("kabel") {
        Some(("kabel-kupfer", ""))
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("alu") && l.contains("geschirr") {
        Some(("aluminium-blech", "Geschirr 10%"))
    } else if l.contains("alu") {
        Some(("aluminium-gemischt", "Blank/Neu"))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("v2a") || l.contains("edelstahl") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("schredder") || l.contains("shredder") {
        Some(("stahlschrott-shredder", ""))
    } else if l.contains("schwer") {
        // Below all copper arms, so only the steel grade lands here:
        // "Schwer Schrott" sits in the steel block next to Mischschrott /
        // Schredder at a steel price, i.e. heavy shear-class scrap.
        Some(("stahlschrott-scheren", "schwer"))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after the
/// `<h1>Impressum</h1>` heading holds firm lines + street + PLZ city, and
/// the `<p>` after the `<h2>Kontakt</h2>` heading carries labeled
/// `Telefon:` / `E-Mail:` lines. Missing anchors mean the page changed
/// shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let h2 = Selector::parse("h2").expect("valid selector");
    let anchor = doc
        .select(&h1)
        .find(|h| h.text().collect::<String>().trim() == "Impressum");
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    };
    // Address lines: first <p> sibling after the heading.
    let addr_p = anchor
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let mut lines = Vec::new();
    if let Some(p) = addr_p {
        for part in p.inner_html().split("<br") {
            let t = strip_tags(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
    // "Ludwig-Erhard-Str. 15" / "30982 Pattensen" (last two lines).
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if lines.len() >= 2 {
        let last = lines.last().expect("len checked");
        let mut it = last.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it
                    .fold(ci.to_owned(), |mut a, w| {
                        a.push(' ');
                        a.push_str(w);
                        a
                    })
                    .trim()
                    .to_owned();
                let candidate = lines[lines.len() - 2].clone();
                if candidate.chars().any(|c| c.is_ascii_digit()) {
                    street = candidate;
                }
            }
        }
    }
    // Contact lines: first <p> sibling after the "Kontakt" heading.
    let contact = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Kontakt");
    let Some(contact) = contact else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let contact_p = contact
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let (mut phone, mut email) = (String::new(), String::new());
    if let Some(p) = contact_p {
        for part in p.inner_html().split("<br") {
            let t = strip_tags(part);
            let low = t.to_lowercase();
            if low.starts_with("telefon:") || low.starts_with("tel.:") || low.starts_with("tel:") {
                if let Some((_, v)) = t.split_once(':') {
                    phone = v.trim().to_owned();
                }
            } else if low.starts_with("e-mail:") || low.starts_with("email:") {
                if let Some((_, v)) = t.split_once(':') {
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
    Ok(TraderInfo {
        street,
        postcode,
        city,
        phone,
        email,
    })
}

/// Strip tags from a fragment (entities are already decoded by html5ever).
fn strip_tags(s: &str) -> String {
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

#[allow(clippy::type_complexity)]
fn parse(
    html: &str,
) -> Result<
    (
        Option<String>,
        Vec<(
            String,
            f64,
            Option<f64>,
            Option<f64>,
            &'static str,
            Option<f64>,
            &'static str,
        )>,
        Vec<String>,
    ),
    IngestError,
> {
    // Only the price box: from its heading to the terminator run-out.
    // Anything after "auf Anfrage" is footer/service prose, never prices.
    let start = html
        .find("Aktuelle Preise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbox fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let term = tail.find("auf Anfrage").map(|i| i + "auf Anfrage".len());
    let window = &tail[..term.unwrap_or(tail.len()).min(12_000)];
    // The table carries no header row (bare label/price pairs), so the
    // heading window above is the selector — never page order.
    let frag = Html::parse_fragment(window);
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let Some(table) = frag.select(&table).next() else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    };
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for tr in table.select(&row) {
        let cells: Vec<String> = tr
            .select(&cell)
            .map(|c| {
                c.text()
                    .collect::<String>()
                    .replace(['\u{a0}'], " ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();
        if cells.len() < 2 {
            continue;
        }
        let label = cells[0].clone();
        if label.is_empty() {
            continue;
        }
        // Texts >120 chars are prose, never a grade label.
        if label.len() > 120 {
            skipped.push(format!("{label} (kein Preislabel)"));
            continue;
        }
        let price_text = cells[1].clone();
        // Empty price cell: loud skip, never a silent zero.
        let Some(price) = parse_eur(&price_text) else {
            skipped.push(format!("{label} (kein Preis: {})", price_text.trim()));
            continue;
        };
        // A "0,00" row is "no quote", not a free gift: loud skip.
        if price == 0.0 {
            skipped.push(format!("{label} (Preis 0,00)"));
            continue;
        }
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let Some(unit) = unit_of(&price_text) else {
            skipped.push(format!(
                "{label} (Einheit unverständlich: {})",
                price_text.trim()
            ));
            continue;
        };
        // "Ab …" quotes a floor, "bis zu …" a ceiling — both get their
        // own kind deliberately per row, never a silent exact and never
        // price_min/max without kind.
        let (price_min, price_max, price_kind, confidence) =
            if price_text.to_lowercase().contains("bis zu") {
                (None, Some(price), "upto", Some(0.5))
            } else if price_text
                .to_lowercase()
                .split(|c: char| !c.is_alphanumeric())
                .any(|w| w == "ab")
            {
                (Some(price), None, "approx", Some(0.5))
            } else {
                (None, None, "exact", Some(1.0))
            };
        rows.push((
            label, price, price_min, price_max, price_kind, confidence, unit,
        ));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    // No calendar date anywhere on the page ("immer aktuelle
    // Tagespreise"): published_at stays None, observed_at is the age.
    Ok((None, rows, skipped))
}

/// Bespoke unit matcher for THIS table's price column (live: "Ab 150 € To"
/// for steel, "Ab 9,60 € Kg" for non-ferrous). Only kg/t exist here —
/// anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|t| t == "to" || t == "t")
    {
        Some("EUR/t")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    // Real excerpt of the live homepage (28.09.2026): heading, the full
    // 12-row table verbatim (incl. the `&nbsp;` entity), terminator line.
    const FIXTURE: &str =
        "<h2 class=\"elementor-heading-title elementor-size-default\">Aktuelle Preise</h2>\
        <table><tbody>\
        <tr><td>Mischschrott</td><td>Ab 150 € To</td></tr>\
        <tr><td>Schwer Schrott</td><td>Ab 160 € To</td></tr>\
        <tr><td>Schredder Schrott</td><td>Ab 100 € To</td></tr>\
        <tr><td>Kupfer Millberry</td><td>Ab 9,60 € Kg</td></tr>\
        <tr><td>Kupfer Schwer</td><td>Ab 9,00 € Kg</td></tr>\
        <tr><td>Kupfer Kabel</td><td>Ab 3,10 € Kg</td></tr>\
        <tr><td>Messing</td><td>Ab 5,30 € Kg</td></tr>\
        <tr><td>Alu Geschirr 10%</td><td>Ab 1,10 € Kg</td></tr>\
        <tr><td>Alu Blank/Neu</td><td>Ab 1,80 € Kg&nbsp;</td></tr>\
        <tr><td>Zink</td><td>Ab 1,60 € Kg</td></tr>\
        <tr><td>Blei</td><td>Ab 1,00 € Kg</td></tr>\
        <tr><td>Edelstahl V2A</td><td>Ab 0,70 € Kg</td></tr>\
        </tbody></table>\
        <p>weitere Sorten auf Anfrage,<br>Alle Preise angeliefert, frei Lager.</p>";

    #[test]
    fn table_parses_twelve_approx_rows() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at, None, "no page date");
        assert!(skips.is_empty(), "{skips:?}");
        assert_eq!(rows.len(), 12);
        assert_eq!(rows[0].0, "Mischschrott");
        assert_eq!(rows[0].1, 150.0);
        assert_eq!(
            (rows[0].2, rows[0].3, rows[0].4, rows[0].5),
            (Some(150.0), None, "approx", Some(0.5))
        );
        assert_eq!(rows[0].6, "EUR/t");
        assert_eq!(rows[1].0, "Schwer Schrott");
        assert_eq!(rows[1].6, "EUR/t");
        assert_eq!(rows[2].6, "EUR/t");
        assert_eq!(rows[3].1, 9.6);
        assert_eq!(rows[3].6, "EUR/kg");
        assert_eq!(rows[8].0, "Alu Blank/Neu");
        assert_eq!(rows[8].1, 1.8);
        assert_eq!(rows[11].0, "Edelstahl V2A");
        assert_eq!(rows[11].1, 0.7);
        for r in &rows {
            assert_eq!(r.6, if r.1 >= 100.0 { "EUR/t" } else { "EUR/kg" });
        }
    }

    #[test]
    fn empty_zero_and_foreign_unit_skip_loudly() {
        let html = FIXTURE
            .replacen("<td>Ab 9,60 € Kg</td>", "<td></td>", 1)
            .replacen("<td>Ab 9,00 € Kg</td>", "<td>0,00 € Kg</td>", 1)
            .replacen("<td>Ab 3,10 € Kg</td>", "<td>Ab 3,10 € pro Sack</td>", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 9);
        assert_eq!(skips.len(), 3);
        assert!(skips[0].contains("Millberry") && skips[0].contains("kein Preis"));
        assert!(skips[1].contains("Kupfer Schwer") && skips[1].contains("0,00"));
        assert!(skips[2].contains("Kupfer Kabel") && skips[2].contains("Einheit"));
    }

    #[test]
    fn missing_table_and_empty_table_error_loudly() {
        let err = parse("<h2>Aktuelle Preise</h2><p>keine Tabelle</p>").expect_err("no table");
        assert!(err.to_string().contains("keine Preistabelle"));
        let err = parse("<html><body><p>Redesign</p></body></html>").expect_err("no anchor");
        assert!(err.to_string().contains("Preisbox"));
        // Every row unparseable: loud error, not silent success.
        let html = FIXTURE
            .replace("€ To", "€ pro Sack")
            .replace("€ Kg", "€ pro Sack");
        let err = parse(&html).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn upto_and_exact_kinds_stay_honest() {
        let html = FIXTURE
            .replacen("<td>Ab 9,60 € Kg</td>", "<td>bis zu 9,60 € Kg</td>", 1)
            .replacen("<td>Ab 9,00 € Kg</td>", "<td>9,00 € Kg</td>", 1);
        let (_, rows, _) = parse(&html).expect("parses");
        let mill = rows
            .iter()
            .find(|r| r.0 == "Kupfer Millberry")
            .expect("row");
        assert_eq!(
            (mill.2, mill.3, mill.4, mill.5),
            (None, Some(9.6), "upto", Some(0.5))
        );
        let schwer = rows.iter().find(|r| r.0 == "Kupfer Schwer").expect("row");
        assert_eq!(
            (schwer.2, schwer.3, schwer.4, schwer.5),
            (None, None, "exact", Some(1.0))
        );
    }

    #[test]
    fn units_cover_live_spellings() {
        assert_eq!(unit_of("Ab 150 € To"), Some("EUR/t"));
        assert_eq!(unit_of("Ab 9,60 € Kg"), Some("EUR/kg"));
        assert_eq!(unit_of("Ab 1,80 € Kg "), Some("EUR/kg"));
        assert_eq!(unit_of("Ab 3,10 € pro Sack"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        // Real fragment shape of the live impressum (28.09.2026).
        let imp = "<div style=\"word-wrap: break-word;\"><h1>Impressum</h1>\
            <p>Thomas Klix<br />Klix-Recycling<br />Ludwig-Erhard-Str. 15<br />30982 Pattensen</p>\
            <h2>Kontakt</h2><p>Telefon: 05101-855757<br />Telefax: 05101-855 758<br />\
            E-Mail: info@klix-recycling.de</p></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Ludwig-Erhard-Str. 15");
        assert_eq!(info.postcode, "30982");
        assert_eq!(info.city, "Pattensen");
        assert_eq!(info.phone, "05101-855757");
        assert_eq!(info.email, "info@klix-recycling.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(extract_info("<h1>Impressum</h1><p>Thomas Klix<br />30982 Pattensen</p>").is_err());
    }

    #[test]
    fn mapping_covers_live_table() {
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(
            grade_for("Schwer Schrott"),
            Some(("stahlschrott-scheren", "schwer"))
        );
        assert_eq!(
            grade_for("Schredder Schrott"),
            Some(("stahlschrott-shredder", ""))
        );
        assert_eq!(
            grade_for("Kupfer Millberry"),
            Some(("kupfer-millberry", ""))
        );
        // Specific-before-generic: heavy copper is copper, not steel.
        assert_eq!(
            grade_for("Kupfer Schwer"),
            Some(("kupfer-gemischt", "Schwer"))
        );
        assert_eq!(grade_for("Kupfer Kabel"), Some(("kabel-kupfer", "")));
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(
            grade_for("Alu Geschirr 10%"),
            Some(("aluminium-blech", "Geschirr 10%"))
        );
        assert_eq!(
            grade_for("Alu Blank/Neu"),
            Some(("aluminium-gemischt", "Blank/Neu"))
        );
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(grade_for("Edelstahl V2A"), Some(("edelstahl-v2a", "")));
        assert_eq!(grade_for("Katalysatoren"), None);
    }
}
