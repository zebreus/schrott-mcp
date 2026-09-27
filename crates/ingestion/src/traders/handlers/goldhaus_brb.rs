//! Goldhaus BRB (Brandenburg an der Havel + Stendal): acceptance list
//! without prices ("Wir kaufen alle Gegenstände an, die edelmetallhaltig
//! sind, insbesondere:" followed by a 15-item `<ul>`, terminated by
//! `</ul>`). No prices, no page date — this handler only fills
//! `trader_materials` plus contact enrichment. Zero prices with resolved
//! acceptances is normal operation, not a canary trip.
//!
//! Why the homepage and not /goldrechner/: the Goldrechner page renders
//! an empty `<main>` in static HTML (its calculator is client-side; the
//! WP API returns empty page content), the Zahngold/Altgold pages carry
//! prose but no concrete Ankaufspreise, and the homepage "Tageskurse"
//! widget is cookie-walled ("Der Inhalt ist nicht verfügbar. Bitte
//! erlaube Cookies"). Verified live: no machine-readable prices anywhere
//! on this site. The homepage "insbesondere" list is the only
//! machine-readable acceptance source.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bb-brandenburg-a-d-h-goldhaus-brb";
/// Bespoke, live-verified impressum URL (the site's own footer link).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://goldhaus-brb.de/edelmetall/impressum/";

pub const URL: &str = "https://goldhaus-brb.de/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| {
        Box::pin(scrape(c))
    } }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let labels = parse(&html)?;
    let mut acceptances = Vec::new();
    let mut skipped_labels = Vec::new();
    for label in labels {
        match grade_for(&label) {
            Some(materials) => {
                for (material, conditions) in materials {
                    acceptances.push(ScrapedAcceptance {
                        material,
                        conditions: conditions.to_owned(),
                        label: label.clone(),
                    });
                }
            }
            None => skipped_labels.push(format!("{label} (kein Katalogmaterial: Edelmetall)")),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        prices: vec![],
        acceptances,
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

/// Explicit label → acceptances. The EUR/g `gold`/`zahngold`/`silber`/
/// `platin`/`palladium` materials cover the solid-metal rows; plated ware
/// ("versilbertes Besteck", "vergoldetes …") would fail any fineness bar
/// and stays out. A wrong acceptance is worse than a logged gap.
/// Explicit label → acceptances. Plated ware ("versilbert",
/// "vergoldet") never resolves to a fineness material; "Alte D-Mark"
/// stays out (banknotes are paper, coins are silver — unattributable);
/// diamonds have no material. Everything else maps; the raw label rides
/// as conditions for traceability.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    if l.contains("versilbert") || l.contains("vergoldet") {
        return None;
    }
    if l.contains("d-mark") || l.contains("dmark") {
        return None;
    }
    if l.contains("diamant") || l.contains("brillant") {
        return None;
    }
    if l.contains("zahngold") {
        return Some(vec![("zahngold", "")]);
    }
    if l.contains("silberbesteck") {
        return Some(vec![("silber", "Besteck")]);
    }
    if l.contains("silber") {
        return Some(vec![("silber", "")]);
    }
    if l.contains("platin") {
        return Some(vec![("platin", "")]);
    }
    if l.contains("palladium") {
        return Some(vec![("palladium", "")]);
    }
    if l.contains("goldschmuck") {
        return Some(vec![("gold", "Schmuck")]);
    }
    if l.contains("bruchgold") {
        return Some(vec![("gold", "Bruchgold")]);
    }
    if l.contains("golduhr") || l.contains("taschenuhr") {
        return Some(vec![("gold", "Uhren")]);
    }
    if l.contains("münzen") || l.contains("medaillen") {
        return Some(vec![("gold", "Münzen & Medaillen")]);
    }
    if l.contains("altgold") {
        return Some(vec![("gold", "")]);
    }
    if l == "zinn" {
        return Some(vec![("zinn", "")]);
    }
    None
}

/// The acceptance list between the "insbesondere:" offer line and the
/// closing `</ul>`. 0 items → Err (a silent empty success would hide a
/// redesign).
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let start = html.find("insbesondere:").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Annahmeliste fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("</ul>").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Annahmeliste unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<ul>{window}</ul>"));
    let li = Selector::parse("li").expect("valid selector");
    let labels: Vec<String> = doc
        .select(&li)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|t| !t.is_empty())
        .collect();
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    Ok(labels)
}

/// Bespoke contact extraction for THIS impressum only: the `<h1>`
/// "Impressum" heading is the must-anchor; the `<p>` blocks below hold
/// "Goldhaus BRB<br>Ingenieurbüro be David" / "Antonio be David" plus
/// maps links ("Steinstraße 12, 14776 Brandenburg an der Havel",
/// "Breite str. 17, 39576 Stendal") and a "Telefon:" / mailto "E-Mail:"
/// line. Missing heading → loud error, never a guessed fallback. The
/// Brandenburg address wins (seed city is Brandenburg a.d.H.).
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let found = doc.select(&h1).any(|el| el.text().collect::<String>().trim() == "Impressum");
    if !found {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    // Text nodes glue without separators ("David"+"Steinstraße"), so join
    // nodes with spaces BEFORE whitespace normalization.
    let all = doc
        .root_element()
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let (_, after) = all.split_once("Impressum").ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Impressum-Block fehlt".to_owned(),
    })?;
    // Street: "Steinstraße 12" (this trader's street, verified live).
    let toks: Vec<&str> = after.split_whitespace().collect();
    let mut street = String::new();
    for (k, t) in toks.iter().enumerate() {
        if *t == "Steinstraße" {
            if let Some(n) = toks.get(k + 1) {
                street = format!("Steinstraße {}", n.trim_matches(','));
                break;
            }
        }
    }
    // Postcode + multi-word city ("14776 Brandenburg an der Havel"):
    // everything after the PLZ up to the next anchor (Stendal address or
    // "Telefon:").
    let (mut postcode, mut city) = (String::new(), String::new());
    for (k, t) in toks.iter().enumerate() {
        if *t == "14776" {
            postcode = (*t).to_owned();
            let mut parts = Vec::new();
            for u in toks.iter().skip(k + 1) {
                if *u == "Breite" || *u == "Telefon:" {
                    break;
                }
                parts.push(*u);
            }
            city = parts.join(" ");
            break;
        }
    }
    let phone = after
        .find("Telefon:")
        .map(|i| {
            after[i + "Telefon:".len()..]
                .split_whitespace()
                .take_while(|t| t.chars().all(|c| c.is_ascii_digit() || "+/().-".contains(c)))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();
    // Email needs its own rule (the phone-style take_while above would
    // stop at the first letter — emails are one token).
    let email = after
        .find("E-Mail:")
        .map(|i| after[i + "E-Mail:".len()..].split_whitespace().next().unwrap_or_default().to_owned())
        .unwrap_or_default();
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo { street, postcode, city, phone, email })
}

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    // Real shape of the live homepage list (offer line + full <ul>),
    // untrimmed: all 15 live labels.
    const FIXTURE: &str = "<p>Alle Preise sind bei uns unverbindlich und die Preisbestimmung kostenlos.\
        Wir kaufen alle Gegenstände an, die edelmetallhaltig sind, insbesondere:</p><ul>\
        <li>Goldschmuck jeglicher Art</li><li>Altgold</li>\
        <li>Bruchgold (kaputter Schmuck, Goldreste u.v.m.)</li>\
        <li>Münzen und Medaillen (z.B. Krügerrand, Dukaten, Philharmoniker, Deutsche Mark)</li>\
        <li>Golduhren, Taschenuhren</li><li>Zahngold, egal ob mit oder ohne Zahnresten</li>\
        <li>Silber</li><li>Silberbesteck</li><li>versilbertes Besteck</li>\
        <li>vergoldetes (Besteck, Münzen, Ketten etc.)</li><li>Platin</li><li>Palladium</li>\
        <li>Diamanten, Brillanten</li><li>Alte D-Mark</li><li>Zinn</li></ul>";

    #[test]
    fn list_parses_and_edelmetalle_map() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 15);
        assert_eq!(labels[0], "Goldschmuck jeglicher Art");
        assert_eq!(labels[5], "Zahngold, egal ob mit oder ohne Zahnresten");
        assert_eq!(labels[14], "Zinn");
        assert_eq!(grade_for("Zinn"), Some(vec![("zinn", "")]));
        assert_eq!(
            grade_for("Goldschmuck jeglicher Art"),
            Some(vec![("gold", "Schmuck")])
        );
        assert_eq!(grade_for("Altgold"), Some(vec![("gold", "")]));
        assert_eq!(
            grade_for("Bruchgold (kaputter Schmuck, Goldreste u.v.m.)"),
            Some(vec![("gold", "Bruchgold")])
        );
        assert_eq!(
            grade_for("Münzen und Medaillen (z.B. Krügerrand, Dukaten, Philharmoniker, Deutsche Mark)"),
            Some(vec![("gold", "Münzen & Medaillen")])
        );
        assert_eq!(
            grade_for("Golduhren, Taschenuhren"),
            Some(vec![("gold", "Uhren")])
        );
        assert_eq!(
            grade_for("Zahngold, egal ob mit oder ohne Zahnresten"),
            Some(vec![("zahngold", "")])
        );
        assert_eq!(grade_for("Silber"), Some(vec![("silber", "")]));
        assert_eq!(
            grade_for("Silberbesteck"),
            Some(vec![("silber", "Besteck")])
        );
        assert_eq!(grade_for("Platin"), Some(vec![("platin", "")]));
        assert_eq!(grade_for("Palladium"), Some(vec![("palladium", "")]));
        // Plated ware must never resolve to a fineness material.
        assert_eq!(grade_for("versilbertes Besteck"), None);
        assert_eq!(grade_for("vergoldetes (Besteck, Münzen, Ketten etc.)"), None);
        // Banknotes are paper, coins are silver — unattributable.
        assert_eq!(grade_for("Alte D-Mark"), None);
        // Diamonds have no material.
        assert_eq!(grade_for("Diamanten, Brillanten"), None);
        assert!(parse("<p>Redesign ohne Liste</p>").is_err());
        assert!(parse("<p>insbesondere:</p><ul></ul>").is_err());
    }

    #[test]
    fn impressum_heading_block() {
        let imp = "<h1>Impressum</h1><p>Goldhaus BRB<br>Ingenieurbüro be David</p>\
            <p>Antonio be David<br>\
            <a href=\"https://maps.app.goo.gl/dptu3qkeErtEoQWz8\">Steinstraße 12, 14776 Brandenburg an der Havel</a><br>\
            <a href=\"https://maps.app.goo.gl/SyjmJ9CuD8WDVSAN9\">Breite str. 17, 39576 Stendal</a></p>\
            <p>Telefon: 03381 / 306 98 85<br>E-Mail:<a href=\"mailto:info@goldhaus-brb.de\">info@goldhaus-brb.de</a></p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Steinstraße 12");
        assert_eq!(info.postcode, "14776");
        assert_eq!(info.city, "Brandenburg an der Havel");
        assert_eq!(info.phone, "03381 / 306 98 85");
        assert_eq!(info.email, "info@goldhaus-brb.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}

