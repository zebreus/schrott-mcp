//! ON Schrott & Metallhandel (Mannheim): acceptance list without
//! prices. The Ankauf page names eleven material cards (`<h4>` headings
//! under "Geld für Schrott") and pays "nach tagesaktuellen Preisen,
//! sofort und in Bar" — daily prices, cash on the scale, but no fixed
//! figures published. This handler turns the cards into
//! `trader_materials` acceptances plus contact enrichment. Zero prices
//! with resolved acceptances is normal operation, not a canary trip.

use scraper::{ElementRef, Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "bw-mannheim-on-schrott-metallhandel";
/// Bespoke, live-verified impressum URL (site footer links
/// "/impressum/"). A move fails the step loudly (fix the URL) — never
/// guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://on-schrott.de/impressum/";

pub const URL: &str = "https://on-schrott.de/ankauf/";

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

/// Explicit card → acceptances. Generic cards stay generic ("Edelstahl"
/// names no grade → `edelstahl-gemischt`, never V2A/V4A). "Batterien"
/// (Pb vs. Li ambiguous), "Bremsscheiben", "Nickel" and the bare "SPÄNE"
/// (metal unnamed) have no catalog material and are skipped loudly (see
/// proposals in the step report).
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("aluminium") || l == "alu" {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("edelstahl") {
        Some(vec![("edelstahl-gemischt", "")])
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("mischschrott") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("blei") {
        Some(vec![("blei", "")])
    } else {
        None
    }
}

/// The eleven material cards: `<h4>` headings between the "Geld für
/// Schrott" intro and the "Haben Sie Fragen?" contact block. Missing
/// anchors or zero cards fail loudly — a silent success would hide a
/// redesign.
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let start = html
        .find("Geld für Schrott")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankauf-Übersicht fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("Haben Sie Fragen?").unwrap_or(tail.len());
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    let h4 = Selector::parse("h4").expect("valid selector");
    let labels: Vec<String> = doc
        .select(&h4)
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

/// Bespoke contact extraction for THIS impressum only: the `<p>` after
/// the "Angaben gemäß § 5 TMG" heading holds firm + street + PLZ city
/// over `<br>` lines, and the `<p>` after the "Kontakt" heading holds
/// the labeled Telefon/E-Mail lines. Missing anchors → loud error, never
/// a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h4 = Selector::parse("h4").expect("valid selector");
    let anchors: Vec<ElementRef> = doc.select(&h4).collect();
    let addr_h = anchors.iter().find(|h| {
        h.text()
            .collect::<String>()
            .contains("Angaben gemäß § 5 TMG")
    });
    let Some(addr_h) = addr_h else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    // Firm block: first <p> sibling after the Angaben heading
    // ("ON-Schrott und Metallhandels GmbH<br />Inselstr. 6<br />68169
    // Mannheim").
    let firm_p = addr_h
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let Some(firm_p) = firm_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmen-Block fehlt".to_owned(),
        });
    };
    let lines: Vec<String> = firm_p
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    for line in &lines {
        let low = line.to_lowercase();
        if low.contains("straße") || low.contains("strasse") || low.contains("str.") {
            street = line.clone();
        }
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(_)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = line[pc.len()..].trim().to_owned();
            }
        }
    }
    // Contact block: first <p> sibling after the "Kontakt" heading.
    let contact_h = anchors
        .iter()
        .find(|h| h.text().collect::<String>().trim() == "Kontakt");
    let Some(contact_h) = contact_h else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let contact_p = contact_h
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let mut phone = String::new();
    let mut email = String::new();
    if let Some(p) = contact_p {
        for part in p.inner_html().split("<br") {
            let t = strip_fragment(part);
            if let Some(rest) = t.strip_prefix("Telefon:") {
                phone = rest
                    .split_whitespace()
                    .take_while(|tok| {
                        tok.chars()
                            .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
            } else if let Some(rest) = t.strip_prefix("E-Mail:") {
                // E-mail needs its own rule: the phone-style take_while
                // would stop at the first letter.
                email = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .to_owned();
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

/// Strip tags from a `<br`-split fragment (drop everything up to the
/// first '>' first, or tag attributes parse as text).
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

    // Real shapes from the live Ankauf page (Elementor cards).
    const FIXTURE: &str = "<h2>Geld für Schrott</h2>\
        <div class=\"elementor-text-editor\"><h4><strong>Aluminium</strong></h4></div>\
        <div class=\"elementor-text-editor\"><h4><strong>Batterien</strong></h4></div>\
        <div class=\"elementor-text-editor\"><h4><strong>Kupfer</strong></h4></div>\
        <div class=\"elementor-text-editor\"><h4><strong>SPÄNE</strong></h4></div>\
        <h2>Haben Sie Fragen?</h2>";

    #[test]
    fn impressum_angaben_block() {
        let imp = "<div><h4>Angaben gemäß § 5 TMG</h4>\
            <p>ON-Schrott und Metallhandels GmbH<br />Inselstr. 6<br />68169 Mannheim</p>\
            <h4>Kontakt</h4><p>Telefon: +49 (0) 621 76 22 31 35<br />E-Mail: info@on-schrott.de</p></div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Inselstr. 6");
        assert_eq!(info.postcode, "68169");
        assert_eq!(info.city, "Mannheim");
        assert_eq!(info.phone, "+49 (0) 621 76 22 31 35");
        assert_eq!(info.email, "info@on-schrott.de");
        assert!(super::extract_info("<h4>Anderes</h4>").is_err());
    }

    #[test]
    fn cards_map_and_skip() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels, vec!["Aluminium", "Batterien", "Kupfer", "SPÄNE"]);
        assert_eq!(
            grade_for("Aluminium"),
            Some(vec![("aluminium-gemischt", "")])
        );
        assert_eq!(
            grade_for("Edelstahl"),
            Some(vec![("edelstahl-gemischt", "")]),
            "generic stays generic, never V2A/V4A"
        );
        assert_eq!(grade_for("Kupfer"), Some(vec![("kupfer-gemischt", "")]));
        assert_eq!(grade_for("Messing"), Some(vec![("messing", "")]));
        assert_eq!(grade_for("Mischschrott"), Some(vec![("mischschrott", "")]));
        assert_eq!(grade_for("Blei"), Some(vec![("blei", "")]));
        assert_eq!(grade_for("Zink"), Some(vec![("zink", "")]));
        assert_eq!(grade_for("Batterien"), None, "Pb vs. Li ambiguous");
        assert_eq!(grade_for("Bremsscheiben"), None, "no cast-iron material");
        assert_eq!(grade_for("Nickel"), None, "no nickel material");
        assert_eq!(grade_for("SPÄNE"), None, "metal unnamed");
        // Missing intro / zero cards fail loudly.
        assert!(parse("<html><body>Neu hier</body></html>").is_err());
        assert!(parse("<h2>Geld für Schrott</h2><p>keine Karten</p>").is_err());
    }
}
