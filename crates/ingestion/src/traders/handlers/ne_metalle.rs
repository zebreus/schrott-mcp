//! NE-Metalle GmbH (Spremberg, Filiale Doberlug-Kirchhain): acceptance
//! list WITHOUT prices. The `/preisliste` table carries 14 material labels
//! in its middle column with EMPTY price cells — live 27.09.2026 the page
//! states "Alle Preise sind Tagespreise und können derzeit nur telefonisch
//! angefragt werden!". So this handler only fills `trader_materials` plus
//! contact enrichment, like `esh.rs`. Zero prices with resolved acceptances
//! is normal operation, not a canary trip.
//!
//! Quoted prices from older research (Messing 5,70, Cu 9,70–10,80, Cu-Kabel
//! 3,35, Papier 0,14 €/kg) are NOT on the page anymore and are deliberately
//! not hardcoded.

use scraper::{ElementRef, Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "bb-spremberg-ne-metalle";
/// Bespoke, live-verified impressum URL (site footer "IMPRESSUM" link).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.ne-metalle-gmbh.de/impressum";

/// Price/acceptance page; gilt laut Navigation für Spremberg UND
/// Doberlug-Kirchhain, geschrieben wird auf den Spremberger Stammsitz
/// (die Filiale ist ein separater Seed-Eintrag im Prüfstatus).
pub const URL: &str = "https://www.ne-metalle-gmbh.de/preisliste";

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
    let labels = parse(&html)?;
    let mut acceptances = Vec::with_capacity(labels.len());
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
            None => skipped_labels.push(label),
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

/// Explicit label → acceptances. Specific-before-generic ("Messing RAFF"
/// before "Messing", "Wuchtblei" before "Blei", "Kabel (Kupfer)" before
/// "Kupfer"). Unproven grades are skipped, never crammed:
///
/// Catalog-gap proposals (do NOT cram):
/// - "Papier" → new `papier` material.
/// - "Schwarzmetall S3" → new `stahlschrott-sorte-3` material? "S3" reads
///   like a BDSV steel grade, but the page never defines it — mapping it to
///   `stahlschrott-scheren` or `mischschrott` would be a guess.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("mischschrott") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("messing raff") {
        Some(vec![("messing", "RAFF")])
    } else if l.contains("messing") {
        // "Messing (schwer)": heavy brass, standard grade, noted condition.
        Some(vec![("messing", "schwer")])
    } else if l.contains("kabel") {
        Some(vec![("kabel-kupfer", "ohne Stecker ohne Klemmen")])
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("alu draht") {
        Some(vec![("aluminium-gemischt", "Draht blank")])
    } else if l.contains("alu") {
        Some(vec![("aluminium-gemischt", "ohne Eisen")])
    } else if l.contains("e-motor") || l.contains("emotor") {
        Some(vec![("elektromotoren", "ohne Anbauten")])
    } else if l.contains("wuchtblei") {
        Some(vec![("blei", "Wucht")])
    } else if l.contains("blei") {
        Some(vec![("blei", "")])
    } else if l.contains("va ") || l.contains("(edelstahl)") || l == "va" {
        // "VA (Edelstahl)": VA could abbreviate V2A, but the page never
        // says so — generic `edelstahl-gemischt`, never a guessed V2A.
        Some(vec![("edelstahl-gemischt", "VA")])
    } else if l.contains("ve edelstahl") || l.contains("edelstahl") {
        // Same for "VE Edelstahl": no proven V4A link → generic grade.
        Some(vec![("edelstahl-gemischt", "VE")])
    } else {
        None
    }
}

fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    // Middle-column labels of the price table, windowed between the first
    // label and the "nur telefonisch" note — the Tel/E-Mail header table
    // and the logo table sit before the window, the footer after it.
    let start = html
        .find("Mischschrott")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste fehlt".to_owned(),
        })?;
    // Back up to the row start: slicing at the label would cut its own
    // <td> tag and drop the first label as stray text.
    let tag = html[..start].rfind("<tr").unwrap_or(0);
    let tail = &html[tag..];
    let end = tail.find("Tagespreise").unwrap_or(tail.len());
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<table>{window}</table>"));
    let td = Selector::parse("td").expect("valid selector");
    let labels: Vec<String> = doc
        .select(&td)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.replace(['\u{a0}'], " "))
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

/// Bespoke contact extraction for THIS impressum only: the `<p>` after the
/// "Angaben gemäß § 5 TMG" heading holds firm + street + PLZ city, and the
/// `<p>` after the "Kontakt" heading holds Telefon/Telefax plus a cloaked
/// mailto link (read from the href — scraper `text()` would glue it).
/// Missing anchors mean the page changed shape → loud error, never a
/// guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let mailto = Selector::parse("a[href^=\"mailto:\"]").expect("valid selector");
    let anchor = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().contains("Angaben gemäß"));
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "TMG-Block fehlt".to_owned(),
        });
    };
    let addr_p = anchor
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    // "NE-Metalle GmbH / Klaus-Gutschke-Straße 7 / 02999 Lohsa".
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if let Some(p) = addr_p {
        let mut lines = Vec::new();
        for part in p.inner_html().split("<br") {
            // Drop tag remnants first ("<br />" → "/>"), then strip.
            let frag = part.split('>').nth(1).unwrap_or(part);
            let t = strip_tags(frag);
            if !t.is_empty() {
                lines.push(t);
            }
        }
        if let Some(last) = lines.last() {
            let mut it = last.split_whitespace();
            if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    city = ci.to_owned();
                    if lines.len() >= 2 {
                        street = lines[lines.len() - 2].clone();
                    }
                }
            }
        }
    }
    // Phone from the Kontakt paragraph, email from the mailto href.
    let contact_h = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Kontakt");
    let mut phone = String::new();
    if let Some(h) = contact_h {
        if let Some(p) = h
            .next_siblings()
            .filter_map(ElementRef::wrap)
            .find(|e| e.value().name() == "p")
        {
            let text: String = p.text().collect();
            // Cut at "Telefax": scraper text() glues "…279910"+"Telefax:"
            // into one token and the digit filter below would stop cold.
            let text = text.split("Telefax").next().unwrap_or(&text).to_owned();
            phone = phone_after(&text, "Telefon:");
        }
    }
    let email = doc
        .select(&mailto)
        .filter_map(|a| a.value().attr("href"))
        .find_map(|h| h.strip_prefix("mailto:"))
        .unwrap_or_default()
        .trim()
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

/// Phone-ish token run after a marker ("Telefon: 035751-279910").
fn phone_after(text: &str, marker: &str) -> String {
    text.find(marker).map_or_else(String::new, |i| {
        text[i + marker.len()..]
            .split_whitespace()
            .take_while(|t| {
                t.chars()
                    .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
            })
            .collect::<Vec<_>>()
            .join(" ")
    })
}

/// Strip tags from a fragment (entities already decoded by html5ever).
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

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    // Real live-table excerpts (verbatim rows + the telefonisch note).
    const FIXTURE: &str = concat!(
        "<table><tbody>",
        "<tr><td>&nbsp;</td><td>Mischschrott</td><td>&nbsp;</td></tr>",
        "<tr><td>&nbsp;</td>",
        "<td style=\"background-color: #cccccc;\">Kupfer&nbsp;</td>",
        "<td>&nbsp;</td></tr>",
        "<tr><td>&nbsp;</td><td>Messing RAFF (unrein mit Anhaftungen)</td><td>&nbsp;</td></tr>",
        "<tr><td>&nbsp;</td>",
        "<td style=\"background-color: #cccccc;\">E-Motor ohne Anbauten</td>",
        "<td>&nbsp;</td></tr>",
        "<tr><td>&nbsp;</td><td>Papier</td><td>&nbsp;</td></tr>",
        "<tr><td>&nbsp;</td>",
        "<td style=\"background-color: #cccccc;\">Schwarzmetall S3</td>",
        "<td>&nbsp;</td></tr>",
        "<tr><td>&nbsp;</td><td>VA (Edelstahl)</td><td>&nbsp;</td></tr>",
        "</tbody></table>",
        "<p>Alle&nbsp; Preise sind Tagespreise und können derzeit nur telefonisch ",
        "angefragt werden !&nbsp;</p>",
    );

    #[test]
    fn list_parses_between_anchors() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 7, "{labels:?}");
        assert_eq!(labels[0], "Mischschrott");
        assert_eq!(labels[1], "Kupfer");
        assert_eq!(labels[2], "Messing RAFF (unrein mit Anhaftungen)");
        // Missing anchor fails loudly, never silently empty.
        assert!(parse("<table><tr><td>Neu hier</td></tr></table>").is_err());
        assert!(parse("Mischschrott <p>leer</p> Tagespreise").is_err());
    }

    #[test]
    fn impressum_tmg_block() {
        let imp = "<h2>Angaben gemäß § 5 TMG</h2>\
            <p>NE-Metalle GmbH<br />Klaus-Gutschke-Straße 7<br />02999 Lohsa</p>\
            <h2>Kontakt</h2>\
            <p>Telefon: 035751-279910<br />Telefax: 035751-279940<br />E-Mail:&nbsp;\
            <a href=\"mailto:info@ne-metalle-gmbh.de\">info@ne-metalle-gmbh.de</a></p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Klaus-Gutschke-Straße 7");
        assert_eq!(info.postcode, "02999");
        assert_eq!(info.city, "Lohsa");
        assert_eq!(info.phone, "035751-279910");
        assert_eq!(info.email, "info@ne-metalle-gmbh.de");
        assert!(super::extract_info("<h2>Neu</h2><p>x</p>").is_err());
    }

    #[test]
    fn mapping_covers_every_live_label() {
        // Full live label set 27.09.2026 (empty price cells, all of them).
        assert_eq!(grade_for("Mischschrott"), Some(vec![("mischschrott", "")]));
        assert_eq!(grade_for("Kupfer"), Some(vec![("kupfer-gemischt", "")]));
        assert_eq!(
            grade_for("Messing (schwer)"),
            Some(vec![("messing", "schwer")])
        );
        assert_eq!(
            grade_for("Messing RAFF (unrein mit Anhaftungen)"),
            Some(vec![("messing", "RAFF")])
        );
        assert_eq!(
            grade_for("Alu ohne Eisen"),
            Some(vec![("aluminium-gemischt", "ohne Eisen")])
        );
        assert_eq!(
            grade_for("Alu Draht blank"),
            Some(vec![("aluminium-gemischt", "Draht blank")])
        );
        assert_eq!(
            grade_for("Kabel (Kupfer) ohne Stecker ohne Klemmen"),
            Some(vec![("kabel-kupfer", "ohne Stecker ohne Klemmen")])
        );
        assert_eq!(
            grade_for("E-Motor ohne Anbauten"),
            Some(vec![("elektromotoren", "ohne Anbauten")])
        );
        assert_eq!(grade_for("Blei"), Some(vec![("blei", "")]));
        assert_eq!(grade_for("Wuchtblei"), Some(vec![("blei", "Wucht")]));
        assert_eq!(
            grade_for("VA (Edelstahl)"),
            Some(vec![("edelstahl-gemischt", "VA")])
        );
        assert_eq!(
            grade_for("VE Edelstahl"),
            Some(vec![("edelstahl-gemischt", "VE")])
        );
        // Loud skips with new-material proposals (see `grade_for` docs).
        assert_eq!(grade_for("Papier"), None);
        assert_eq!(grade_for("Schwarzmetall S3"), None);
    }
}
