//! Schrott Frankfurt e.K. (Scholz, Frankfurt): seven exact day prices as
//! photo-caption `<span>`s inside a one-cell-per-row table ("Mischrott
//! 0,12 € / kg Tagespreis abhängig" …). No table header, no date —
//! "Tagespreis abhängig" is a day-price disclaimer like Vedder's
//! "Unverbindliche Ankaufspreise", so rows are exact at 1.0. The window
//! runs from the "Schrottplatz Frankfurt und Schrottpreise" heading to the
//! "Tageshöchstpreise" box so footer/phone lines never pair up.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "he-frankfurt-am-main-schrott-frankfurt-scholz";
/// Bespoke, live-verified impressum URL (the site footer's own
/// "Impressum" link). A move fails the step loudly (fix the URL) —
/// never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://schrott-frankfurt.de/Impressum/";

pub const URL: &str = "https://schrott-frankfurt.de/Schrottpreise-Schrottplatz/";

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

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. "Alu 5% Anhaftung" needs its own variant or it would collapse
/// onto the clean "Alu-Reinschrott" row of the same material.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("anhaften") || l.contains("5%") {
        Some(("aluminium-gemischt", "5% Anhaftung"))
    } else if l.contains("alu") || l.contains("reinschrott") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("mischrott") || l.contains("mischschrott") {
        // Live spells it "Mischrott" (one s) — match the typo, not the norm.
        Some(("mischschrott", ""))
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("guss") {
        Some(("eisenschrott-gussbruch", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<h2>`
/// "Impressum von Schrott Frankfurt" (glued from two spans) heads labeled
/// `<p>` lines ("Firma :", street, "PLZ Ort", "Telefon :", "Mobil :") and
/// the e-mail rides in a `mailto:` link. Missing anchors mean the page
/// changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let a = Selector::parse("a[href^=\"mailto:\"]").expect("valid selector");
    let anchor = doc.select(&h2).find(|h| {
        h.text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .contains("Impressum von Schrott Frankfurt")
    });
    let Some(_) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    };
    // Labeled content lines after the anchor; stop at the
    // "Geschäftsstelle" branch block so it never mixes in (first wins
    // anyway, but the window stays disciplined).
    let mut lines = Vec::new();
    for el in doc.select(&p) {
        let t = el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if t.contains("Impressum von Schrott Frankfurt") {
            continue;
        }
        lines.push(t);
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let (mut phone, mut mobile) = (String::new(), String::new());
    for line in &lines {
        if line.starts_with("Telefon") {
            if let Some((_, v)) = line.split_once(':') {
                if phone.is_empty() {
                    phone = v.trim().to_owned();
                }
            }
        } else if line.starts_with("Mobil") {
            if let Some((_, v)) = line.split_once(':') {
                if mobile.is_empty() {
                    mobile = v.trim().to_owned();
                }
            }
        } else if street.is_empty()
            && (line.to_lowercase().contains("straße") || line.contains("Str."))
            && !line.starts_with("Firma")
            && !line.starts_with("Inhaber")
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
        // The branch-office block ("Geschäftsstelle : …") below the
        // verantwortlich-line must never mix in — stop there.
        if line.contains("Geschäftsstelle") {
            break;
        }
    }
    if phone.is_empty() {
        phone = mobile;
    }
    // E-mail via the mailto href (never token-split glued text).
    let mut email = String::new();
    if let Some(link) = doc.select(&a).next() {
        if let Some(href) = link.value().attr("href") {
            let addr = href.strip_prefix("mailto:").unwrap_or(href);
            let addr = addr.split('?').next().unwrap_or(addr);
            email = addr.replace("%40", "@").trim().to_owned();
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

/// Parse the caption window between the price heading and the
/// "Tageshöchstpreise" box. Returns (rows, skips); 0 rows is an error.
fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html
        .find("Schrottplatz Frankfurt und Schrottpreise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbox fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Tageshöchstpreise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbox unvollständig".to_owned(),
        })?;
    let window = &tail[..end];
    // Content spans only (never script/style): each caption carries one
    // "Label 0,12 € / kg Tagespreis abhängig" line.
    let frag = Html::parse_fragment(window);
    let span = Selector::parse("span").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for el in frag.select(&span) {
        let t = el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if !t.contains('€') {
            continue;
        }
        if t.len() > 120 {
            continue;
        }
        let Some(unit) = unit_of(&t) else {
            skips.push(format!("{t} (Einheit unverständlich: {t})"));
            continue;
        };
        // Label = everything before the trailing price token: strip the
        // disclaimer, split at €, the price is the last token on the left.
        let left = t.split('€').next().unwrap_or("").trim();
        let Some((label, num)) = left.rsplit_once(char::is_whitespace) else {
            skips.push(format!("{t} (Preis unverständlich)"));
            continue;
        };
        let Some(price) = parse_eur(num) else {
            skips.push(format!("{t} (Preis unverständlich: {num})"));
            continue;
        };
        let label = label.trim().to_owned();
        if label.is_empty() {
            continue;
        }
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preispaare".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS page's captions (live: "0,12 € / kg").
/// Only kg exists here — anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|t| t == "t")
    {
        Some("EUR/t")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    // Real caption shape (photo widget + caption span per table row),
    // trimmed to three rows, terminated by the Tageshöchstpreise box
    // exactly like live.
    const FIXTURE: &str = "<h1><span>Schrottplatz Frankfurt und Schrottpreise</span><br></h1>\
        <table><tbody>\
        <tr><td><div><picture><img src=\"/m.jpg\"></picture>\
        <span>Mischrott 0,12 € / kg Tagespreis abhängig</span></div></td></tr>\
        <tr><td><div><picture><img src=\"/a.jpg\"></picture>\
        <span>Alu 5% Anhaftung   0,60 € / kg Tagespreis abhängig</span></div></td></tr>\
        <tr><td><div><picture><img src=\"/k.jpg\"></picture>\
        <span>Kupfer CU 6,10 € / kg Tagespreis abhängig</span></div></td></tr>\
        </tbody></table>\
        <table><tbody><tr><td><p>Tageshöchstpreise täglich telefonisch neu unter :</p>\
        <p>Mobil :  0170 2725 486</p></td></tr></tbody></table>";

    #[test]
    fn captions_labels_and_units() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 3);
        assert!(skips.is_empty(), "{skips:?}");
        assert_eq!(rows[0], ("Mischrott".to_owned(), 0.12, "EUR/kg"));
        // The "5%" grade stays in the label (traceability); the variant
        // split happens in grade_for, not here.
        assert_eq!(rows[1], ("Alu 5% Anhaftung".to_owned(), 0.6, "EUR/kg"));
        assert_eq!(rows[2], ("Kupfer CU".to_owned(), 6.1, "EUR/kg"));
        assert_eq!(unit_of("0,12 € / kg Tagespreis abhängig"), Some("EUR/kg"));
        assert_eq!(unit_of("0,12 € pro Sack"), None);
        // Window gone / captions gone: loud error, not silent success.
        assert!(parse("<h1>Neu hier</h1>").is_err());
        assert!(parse("<h1>Schrottplatz Frankfurt und Schrottpreise</h1><p>leer</p>").is_err());
    }

    #[test]
    fn mapping_covers_all_live_labels() {
        assert_eq!(grade_for("Mischrott"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Kupfer CU"), Some(("kupfer-gemischt", "")));
        assert_eq!(
            grade_for("Alu-Reinschrott"),
            Some(("aluminium-gemischt", ""))
        );
        assert_eq!(
            grade_for("Alu 5% Anhaftung"),
            Some(("aluminium-gemischt", "5% Anhaftung"))
        );
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(
            grade_for("Guss-Schrott"),
            Some(("eisenschrott-gussbruch", ""))
        );
        assert_eq!(grade_for("Kabelschrott"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h2><span>Imp</span><span>ressum von Schrott Frankfurt </span></h2>\
            <p><span>Firma :   Schrott Frankfurt </span></p>\
            <p><span>Inhaber : Heinz-Jürgen Scholz </span></p>\
            <p><span>Sigmund-Freud-Straße 97</span></p>\
            <p><span>60435 Frankfurt am Main <br></span></p>\
            <p><span>Telefon : 069 - 95 868 123<br></span></p>\
            <p><span>Mobil    : 0170 2725486</span></p>\
            <p><span>E-Mail  </span>: <a href=\"mailto:service%40schrott-frankfurt.de?subject=X\">service@schrott-frankfurt.de</a><br></p>\
            <h3><span>Für die Inhalte dieser Webseite verantwortlich : Heinz-Jürgen Scholz</span></h3>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Sigmund-Freud-Straße 97");
        assert_eq!(info.postcode, "60435");
        assert_eq!(info.city, "Frankfurt am Main");
        assert_eq!(info.phone, "069 - 95 868 123");
        assert_eq!(info.email, "service@schrott-frankfurt.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
