//! ALBUS Leipzig (Leipzig): exact daily prices ("Tagespreise") in a
//! single `<table class="table">`, one row per material with two price
//! legs — "mit Kundenkarte" and "ohne Kundenkarte" — kept apart via
//! `variant` (same material, two prices must not collapse). The header
//! states "alle Preise sind Kilo-Preise", so EUR/kg is belegt. The page
//! date hides in the last table row ("Stand: 21.09.2026"). Altpapier,
//! Bücher and PC-Teile have no catalog material and are skipped loudly.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sn-leipzig-albus-leipzig";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.albus-leipzig.de/impressum";

pub const URL: &str = "https://www.albus-leipzig.de/preise";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

/// One table row: the material label plus the optional price of each
/// Kundenkarte leg (the "ohne" cell may be empty, live: "PC-Teile").
#[derive(Debug)]
struct PriceRow {
    label: String,
    mit: Option<f64>,
    ohne: Option<f64>,
    unit: &'static str,
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (published_at, rows, mut skipped_labels) = parse(&html)?;
    let prices = prices_from_rows(rows, &mut skipped_labels);
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

fn prices_from_rows(rows: Vec<PriceRow>, skipped_labels: &mut Vec<String>) -> Vec<ScrapedPrice> {
    let mut prices = Vec::with_capacity(rows.len() * 2);
    for row in rows {
        let Some((material, grade)) = grade_for(&row.label) else {
            skipped_labels.push(row.label);
            continue;
        };
        // Keep both grade and Kundenkarte leg in the current-price key.
        // Empty grades retain the existing standard variants.
        let (mit_variant, ohne_variant) = match grade {
            "" => ("mit Kundenkarte", "ohne Kundenkarte"),
            "Bremsscheiben" => (
                "Bremsscheiben, mit Kundenkarte",
                "Bremsscheiben, ohne Kundenkarte",
            ),
            "Felgen" => ("Felgen, mit Kundenkarte", "Felgen, ohne Kundenkarte"),
            _ => unreachable!("grade_for returned an unsupported ALBUS grade"),
        };
        if let Some(price) = row.mit {
            prices.push(ScrapedPrice {
                material,
                variant: mit_variant,
                price,
                currency: "EUR",
                unit: row.unit,
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label: row.label.clone(),
            });
        }
        if let Some(price) = row.ohne {
            prices.push(ScrapedPrice {
                material,
                variant: ohne_variant,
                price,
                currency: "EUR",
                unit: row.unit,
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label: row.label.clone(),
            });
        }
    }
    prices
}

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific arms first: "Kupfer-Kabel" and "Alufelgen" contain
/// the generic "Kupfer"/"Alu" words, "Eisenguss" contains "Eisen".
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kabel") {
        Some(("kabel-kupfer", ""))
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("alufelgen") {
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("alu-profile") {
        Some(("aluminium-profile", ""))
    } else if l.contains("alu") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("edelstahl") {
        Some(("edelstahl-gemischt", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("bremsscheiben") {
        Some(("eisenschrott-gussbruch", "Bremsscheiben"))
    } else if l.contains("eisenguss") {
        Some(("eisenschrott-gussbruch", ""))
    } else if l.contains("eisen") {
        Some(("mischschrott", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("motoren") {
        Some(("elektromotoren", ""))
    } else if l.contains("zinn") {
        Some(("zinn", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the
/// "Kontakt: Albus Leipzig e.K." blockquote carries the firm line and
/// nested `<div>` address/phone lines ("Eisenacher Straße 88",
/// "04155 Leipzig", "Tel: 0163- 87 47 214"); the mail address
/// (buero@albus-leipzig.de) is plain header text, picked by its own
/// @-rule. Missing contact anchor → loud error, never a fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let quote = Selector::parse("blockquote").expect("valid selector");
    let div = Selector::parse("div").expect("valid selector");
    let anchor = doc.select(&quote).find(|q| {
        q.text()
            .collect::<String>()
            .contains("Kontakt: Albus Leipzig")
    });
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    // Address/phone lines are sibling blockquotes under the same
    // heading, not descendants of the firm line — scope to the parent.
    let scope = anchor
        .parent()
        .and_then(ElementRef::wrap)
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        })?;
    let mut lines: Vec<String> = scope
        .select(&div)
        .map(|d| d.text().collect::<String>())
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|t| !t.is_empty())
        .collect();
    // The firm line itself is direct blockquote text, not a div.
    let own: String = anchor
        .text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    lines.insert(0, own);
    let mut street = String::new();
    let mut postcode = String::new();
    let mut city = String::new();
    let mut phone = String::new();
    for line in &lines {
        if line.starts_with("Tel") {
            let digits: String = line
                .trim_start_matches("Tel:")
                .trim()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            if !digits.is_empty() {
                phone = digits;
            }
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() >= 2
            && toks[0].len() == 5
            && toks[0].chars().all(|c| c.is_ascii_digit())
            && toks[1].chars().next().is_some_and(|c| c.is_uppercase())
        {
            postcode = toks[0].to_owned();
            city = toks[1..].join(" ");
            continue;
        }
        if street.is_empty()
            && !line.contains("Kontakt:")
            && !line.contains("Verantwortlich:")
            && line.chars().any(|c| c.is_alphabetic())
        {
            street = line.clone();
        }
    }
    // E-Mail needs its own rule: the phone-style token filter would stop
    // at the first letter, so take the whitespace token holding '@'.
    // Strict shape required: the page's inline CSS holds "@media", which
    // a bare contains('@') would mistake for a mail address.
    let email = doc
        .root_element()
        .text()
        .flat_map(|t| t.split_whitespace().map(str::to_owned).collect::<Vec<_>>())
        .find(|t| is_email(t))
        .unwrap_or_default();
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

/// Bespoke mail check for THIS impressum: exactly one '@', dotted
/// domain, no CSS/URL characters (the page's `<style>` holds "@media").
fn is_email(tok: &str) -> bool {
    let tok = tok.trim_matches([',', ';', '.', ':', '!', '"', '\'', '(', ')']);
    let mut parts = tok.split('@');
    let (Some(local), Some(domain), None) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    if local.is_empty()
        || !local
            .chars()
            .all(|c| c.is_alphanumeric() || "._%+-".contains(c))
    {
        return false;
    }
    let dot = domain.rfind('.');
    match dot {
        Some(i)
            if i > 0
                && domain.len() - i - 1 >= 2
                && domain[..i]
                    .chars()
                    .all(|c| c.is_alphanumeric() || "-.".contains(c))
                && domain[i + 1..].chars().all(|c| c.is_alphabetic()) =>
        {
            true
        }
        _ => false,
    }
}

fn parse(html: &str) -> Result<(Option<String>, Vec<PriceRow>, Vec<String>), IngestError> {
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tr").expect("valid selector");
    let cell = Selector::parse("td, th").expect("valid selector");
    // Never trust page order: take the table carrying the Kundenkarte
    // header, not just the first <table> on the page.
    let doc = Html::parse_document(html);
    let table = doc.select(&table).find(|t| {
        t.select(&cell)
            .any(|c| c.text().collect::<String>().contains("mit Kundenkarte"))
    });
    let Some(table) = table else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    };
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    let mut published_at = None;
    for tr in table.select(&row) {
        let cells: Vec<String> = tr
            .select(&cell)
            .map(|c| c.text().collect::<String>())
            .map(|t| t.replace('\u{a0}', " "))
            .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        if cells.len() < 3 {
            continue;
        }
        // The page date hides in the last table row ("Stand:
        // 21.09.2026" in an otherwise empty row).
        for c in &cells {
            if let Some(pos) = c.find("Stand:") {
                let rest: Vec<&str> = c[pos + 6..].trim().split('.').collect();
                if rest.len() == 3 {
                    published_at = parse_de_date(
                        rest[0].trim(),
                        rest[1].trim(),
                        rest[2].trim().split_whitespace().next().unwrap_or(""),
                    );
                }
            }
        }
        let label = cells[0].trim().to_owned();
        // Header row ("Material" / "mit Kundenkarte" / …) and visual
        // spacer rows carry no product.
        if label.is_empty() || label == "Material" || cells[1].contains("Kundenkarte") {
            continue;
        }
        if label.len() > 120 {
            continue;
        }
        let mit = if cells[1].trim().is_empty() {
            None
        } else {
            parse_eur(&cells[1])
        };
        let ohne = if cells[2].trim().is_empty() {
            None
        } else {
            parse_eur(&cells[2])
        };
        if mit.is_none() && !cells[1].trim().is_empty() {
            skips.push(format!(
                "{} (Preis unverständlich: {})",
                label,
                cells[1].trim()
            ));
        }
        if ohne.is_none() && !cells[2].trim().is_empty() {
            skips.push(format!(
                "{} (Preis unverständlich: {})",
                label,
                cells[2].trim()
            ));
        }
        if mit.is_none() && ohne.is_none() {
            continue;
        }
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let probe = if !cells[1].trim().is_empty() {
            &cells[1]
        } else {
            &cells[2]
        };
        let Some(unit) = unit_of(probe) else {
            skips.push(format!(
                "{label} (Einheit unverständlich: {})",
                probe.trim()
            ));
            continue;
        };
        rows.push(PriceRow {
            label,
            mit,
            ohne,
            unit,
        });
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    Ok((published_at, rows, skips))
}

/// Bespoke unit matcher for THIS table: the header states "alle Preise
/// sind Kilo-Preise", so a bare "EUR" cell is EUR/kg — documented here,
/// not guessed. Anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("tonne") || lower.contains(" / t") || lower.contains("/t") {
        Some("EUR/t")
    } else if lower.contains("eur") || lower.contains('€') {
        Some("EUR/kg")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    const DECOY: &str = "<table><tr><td>Nav</td></tr></table>";
    const FIXTURE: &str = "<table class=\"table\"> <tbody> \
        <tr class=\"row\"> <th class=\"header-cell\"></th> \
        <th class=\"header-cell\">alle Preise sind Kilo-</th> \
        <th class=\"header-cell\">Preise</th> </tr> \
        <tr class=\"row\"> <td class=\"cell\">Material</td> \
        <td class=\"cell\">mit Kundenkarte</td> \
        <td class=\"cell\">ohne Kundenkarte</td> </tr> \
        <tr class=\"row\"> <td class=\"cell\">Kupfer</td> \
        <td class=\"cell\">9,00 EUR</td> <td class=\"cell\">8,00 EUR</td> </tr> \
        <tr class=\"row\"> <td class=\"cell\">Millberry</td> \
        <td class=\"cell\">10,00 EUR</td> <td class=\"cell\">9,00 EUR</td> </tr> \
        <tr class=\"row\"> <td class=\"cell\">B&uuml;cher</td> \
        <td class=\"cell\">0,04 EUR</td> <td class=\"cell\">0,04 EUR</td> </tr> \
        <tr class=\"row\"> <td class=\"cell\"></td> \
        <td class=\"cell\"></td> <td class=\"cell\"></td> </tr> \
        <tr class=\"row\"> <td class=\"cell\">Bremsscheiben</td> \
        <td class=\"cell\">0,19 EUR</td> <td class=\"cell\">0,16 EUR</td> </tr> \
        <tr class=\"row\"> <td class=\"cell\">PC-Teile</td> \
        <td class=\"cell\">ab 0,10 EUR</td> <td class=\"cell\"></td> </tr> \
        <tr class=\"row\"> <td class=\"cell\"></td> \
        <td class=\"cell\"></td> <td class=\"cell\">Stand: 21.09.2026</td> </tr> \
        </tbody> </table>";

    #[test]
    fn feedback_4782_grades_and_card_legs_have_distinct_keys() {
        let html =
            "<table><tr><td>Material</td><td>mit Kundenkarte</td><td>ohne Kundenkarte</td></tr>\
            <tr><td>Alu</td><td>0,80 EUR</td><td>0,50 EUR</td></tr>\
            <tr><td>Alu-Profile</td><td>1,65 EUR</td><td>1,15 EUR</td></tr>\
            <tr><td>Eisenguss</td><td>0,15 EUR</td><td>0,12 EUR</td></tr>\
            <tr><td>Bremsscheiben</td><td>0,19 EUR</td><td>0,16 EUR</td></tr>\
            <tr><td>Alufelgen sauber</td><td>1,65 EUR</td><td>1,00 EUR</td></tr></table>";
        let (_, rows, mut skips) = parse(html).expect("live table shape");
        let prices = super::prices_from_rows(rows, &mut skips);
        assert!(skips.is_empty());
        let keys: std::collections::HashSet<_> =
            prices.iter().map(|p| (p.material, p.variant)).collect();
        assert_eq!(
            keys.len(),
            prices.len(),
            "no grade may overwrite another: {prices:?}"
        );
        let actual: Vec<_> = prices
            .iter()
            .map(|p| (p.material, p.variant, p.price, p.unit))
            .collect();
        assert_eq!(
            actual,
            vec![
                ("aluminium-gemischt", "mit Kundenkarte", 0.80, "EUR/kg"),
                ("aluminium-gemischt", "ohne Kundenkarte", 0.50, "EUR/kg"),
                ("aluminium-profile", "mit Kundenkarte", 1.65, "EUR/kg"),
                ("aluminium-profile", "ohne Kundenkarte", 1.15, "EUR/kg"),
                ("eisenschrott-gussbruch", "mit Kundenkarte", 0.15, "EUR/kg"),
                ("eisenschrott-gussbruch", "ohne Kundenkarte", 0.12, "EUR/kg"),
                (
                    "eisenschrott-gussbruch",
                    "Bremsscheiben, mit Kundenkarte",
                    0.19,
                    "EUR/kg"
                ),
                (
                    "eisenschrott-gussbruch",
                    "Bremsscheiben, ohne Kundenkarte",
                    0.16,
                    "EUR/kg"
                ),
                ("aluminium-guss", "Felgen, mit Kundenkarte", 1.65, "EUR/kg"),
                ("aluminium-guss", "Felgen, ohne Kundenkarte", 1.00, "EUR/kg"),
            ]
        );
    }

    #[test]
    fn table_date_and_legs_parse() {
        let html = DECOY.to_owned() + FIXTURE;
        let (published_at, rows, skips) = parse(&html).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-21T00:00:00+00:00"));
        assert_eq!(rows.len(), 5);
        assert!(skips.is_empty());
        assert_eq!(rows[0].label, "Kupfer");
        assert_eq!(rows[0].mit, Some(9.0));
        assert_eq!(rows[0].ohne, Some(8.0));
        assert_eq!(rows[0].unit, "EUR/kg");
        // "ab 0,10 EUR" parses as a number; single empty leg stays None.
        let pc = rows.iter().find(|r| r.label == "PC-Teile").expect("pc row");
        assert_eq!(pc.mit, Some(0.1));
        assert_eq!(pc.ohne, None);
    }

    #[test]
    fn missing_table_and_empty_table_error() {
        let err = parse(DECOY).expect_err("no price table");
        assert!(err.to_string().contains("keine Preistabelle"));
        let empty = FIXTURE
            .replace("9,00 EUR", "pro Sack")
            .replace("8,00 EUR", "pro Sack")
            .replace("10,00 EUR", "pro Sack")
            .replace("9,00 EUR", "pro Sack")
            .replace("0,04 EUR", "pro Sack")
            .replace("0,19 EUR", "pro Sack")
            .replace("0,16 EUR", "pro Sack")
            .replace("ab 0,10 EUR", "pro Sack");
        let err = parse(&empty).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn mapping_covers_every_live_row() {
        assert_eq!(grade_for("Kupfer"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Millberry"), Some(("kupfer-millberry", "")));
        assert_eq!(grade_for("Kupfer-Kabel"), Some(("kabel-kupfer", "")));
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Alu"), Some(("aluminium-gemischt", "")));
        assert_eq!(
            grade_for("Alufelgen sauber"),
            Some(("aluminium-guss", "Felgen"))
        );
        assert_eq!(grade_for("Edelstahl"), Some(("edelstahl-gemischt", "")));
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(grade_for("Motoren"), Some(("elektromotoren", "")));
        assert_eq!(grade_for("Zinn"), Some(("zinn", "")));
        assert_eq!(grade_for("Eisen"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Eisenguss"), Some(("eisenschrott-gussbruch", "")));
        assert_eq!(
            grade_for("Bremsscheiben"),
            Some(("eisenschrott-gussbruch", "Bremsscheiben"))
        );
        assert_eq!(grade_for("Altpapier"), None);
        assert_eq!(grade_for("Bücher"), None);
        assert_eq!(grade_for("PC-Teile"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<style>video.videobgframe{margin:0}@media only screen{}</style>\
            <h3>Verantwortlich: Katrin K&uuml;rschner</h3>\
            <h3><blockquote>&nbsp;Kontakt: Albus Leipzig e.K.</blockquote>\
            <blockquote><blockquote><blockquote><blockquote>\
            <div><span>Eisenacher Stra&szlig;e 88</span></div> \
            <div><span>04155 Leipzig</span></div> \
            <div><span>Tel: 0163- 87 47 214</span></div> \
            </blockquote></blockquote></blockquote></blockquote></h3>\
            <span>buero@albus-leipzig.de</span>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Eisenacher Straße 88");
        assert_eq!(info.postcode, "04155");
        assert_eq!(info.city, "Leipzig");
        assert_eq!(info.phone, "0163- 87 47 214");
        assert_eq!(info.email, "buero@albus-leipzig.de");
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
