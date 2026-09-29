//! Buntmetallhandel MIS GmbH (Mannheim-Neckarstadt, Innstr. 10): acceptance
//! list without prices. The single-page site names four service cards
//! (`h4`: Schrottankauf / Buntmetalle / Containerdienst / Demontage, each
//! with a one-line `p`) and buys "zu fairen Tagespreisen" — daily prices
//! on request, no fixed quotes anywhere (verified live 27.09.2026: the
//! only numbers on the page are the address, hours and the closure
//! notice). No prices, no page date — this handler only fills
//! `trader_materials` plus contact enrichment. Zero prices with resolved
//! acceptances is normal operation, not a canary trip.
//!
//! The site has no separate impressum page (verified: single-page site,
//! `wp-json` lists only the homepage plus a boilerplate privacy page,
//! the footer links nowhere, the lone "impressum" string is a
//! Complianz placeholder with `href="#"`). `IMPRESSUM_URL` is therefore
//! the homepage itself and `extract_info` reads its Kontakt block —
//! documented here so a future impressum page is a visible improvement,
//! not a silent gap.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "bw-mannheim-68199-buntmetallhandel-mis";
/// No separate impressum page exists on this single-page site (only a
/// Complianz `href="#"` placeholder). Contact comes from the homepage
/// Kontakt block; a future impressum page should replace this URL.
pub const IMPRESSUM_URL: &str = "https://mis-buntmetall.de/";

pub const URL: &str = "https://mis-buntmetall.de/";

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
            // Services without a material (Containerdienst, Demontage)
            // are evidence, never silent drops.
            None => skipped_labels.push(format!("{label} (Service, kein Material)")),
        }
    }
    // Contact lives on the same page (no impressum exists); a second fetch
    // keeps the failure semantics identical to other handlers.
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

/// Explicit card → acceptances. "Buntmetalle (Kupfer, Aluminium, Messing
/// und mehr)" fans out to the three named metals; the "und mehr" stays
/// uncovered (no guessing). Services without a catalog material
/// (Containerdienst, Demontage) return `None` and are skipped loudly.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("schrottankauf") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("buntmetalle") {
        Some(vec![
            ("kupfer-gemischt", ""),
            ("aluminium-gemischt", ""),
            ("messing", ""),
        ])
    } else {
        None
    }
}

/// Service cards between the first card heading and the Kontakt block.
/// Returns "Name — description" labels; 0 cards = `Err` (a redesign
/// without cards must fail loudly, not succeed silently).
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    // Start at the card heading's opening tag (the "Schrottankauf" hit
    // sits INSIDE the first h4 — starting there would cut the tag and
    // lose the first card).
    let hit = html
        .find("Schrottankauf")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Leistungskarten fehlen".to_owned(),
        })?;
    let start = html[..hit].rfind("<h4").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Leistungskarten fehlen".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail
        .find("Kontaktieren Sie uns jetzt")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Kontaktblock fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let h4 = Selector::parse("h4").expect("valid selector");
    let mut labels = Vec::new();
    for head in frag.select(&h4) {
        let name: String = head.text().collect();
        let name = name.split_whitespace().collect::<Vec<_>>().join(" ");
        if name.is_empty() {
            continue;
        }
        // Description: first <p> among the heading's following siblings.
        let mut desc = String::new();
        let mut sib = head.next_siblings();
        while let Some(n) = sib.next() {
            if let Some(el) = scraper::ElementRef::wrap(n) {
                if el.value().name() == "p" {
                    let t: String = el.text().collect();
                    let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
                    if !t.is_empty() {
                        desc = t;
                        break;
                    }
                }
            }
        }
        if desc.is_empty() || desc.len() > 200 {
            continue;
        }
        labels.push(format!("{name} — {desc}"));
    }
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Leistungskarten leer".to_owned(),
        });
    }
    // The page shows exactly four cards; any change surfaces via grade
    // skips and the card-count-sensitive review of this handler.
    Ok(labels)
}

/// Bespoke contact extraction for THIS homepage only: anchored on the
/// "Haben Sie Schrott oder Metalle zu verkaufen?" Kontakt heading —
/// without it the page changed shape → loud error. The address `<p>`
/// holds the northdata-linked "Innstr. 10, D-68199 Mannheim" line, the
/// mail `<p>` the bare "kontakt@mis-buntmetall.de" address (own `@`
/// rule — a phone-style token filter would stop at the first letter).
/// No phone is published on the site (verified live) → empty.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    if !imp.contains("Haben Sie Schrott oder Metalle zu verkaufen?") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    }
    let doc = Html::parse_document(imp);
    let p = Selector::parse("p").expect("valid selector");
    let mut street = String::new();
    let mut postcode = String::new();
    let mut city = String::new();
    let mut email = String::new();
    for el in doc.select(&p) {
        let t: String = el.text().collect();
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if t.contains("68199") {
            // "Innstr. 10, D-68199 Mannheim": strip the country prefix.
            let bare = t.replace("D-", "");
            let toks: Vec<&str> = bare.split_whitespace().collect();
            for (k, tok) in toks.iter().enumerate() {
                if tok.len() == 5 && tok.chars().all(|c| c.is_ascii_digit()) {
                    postcode = (*tok).to_owned();
                    city = toks[k + 1..].join(" ");
                    street = toks[..k].join(" ").trim_matches([',', ' ']).to_owned();
                    break;
                }
            }
        } else if t.contains('@') && email.is_empty() {
            if let Some(tok) = t.split_whitespace().find(|w| w.contains('@')) {
                email = tok.trim_matches([',', ';']).to_owned();
            }
        }
    }
    if street.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo {
        street,
        postcode,
        city,
        phone: String::new(),
        email,
    })
}

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    // Real shape of the live cards (emoji h4 + one-line p), trimmed to
    // two cards plus the Kontakt terminator.
    const FIXTURE: &str = "<h4>🔩 Schrottankauf</h4>\
        <p>Ankauf von Schrott aller Art zu fairen Tagespreisen</p>\
        <h4>🏗️ Demontage</h4>\
        <p>Fachgerechte Demontage und Entsorgung</p>\
        <h4>Haben Sie Schrott oder Metalle zu verkaufen?<br>Kontaktieren Sie uns jetzt</h4>";

    #[test]
    fn cards_map_and_skip() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 2);
        assert!(labels[0].contains("Schrottankauf"));
        assert_eq!(grade_for(&labels[0]), Some(vec![("mischschrott", "")]));
        assert_eq!(
            grade_for("🔶 Buntmetalle — Kupfer, Aluminium, Messing und mehr"),
            Some(vec![
                ("kupfer-gemischt", ""),
                ("aluminium-gemischt", ""),
                ("messing", "")
            ])
        );
        assert_eq!(grade_for(&labels[1]), None, "Demontage is a service");
        assert!(parse("<div>Redesign ohne Karten</div>").is_err());
        assert!(parse("<h4>🔩 Schrottankauf</h4><p>ohne Kontakt danach").is_err());
    }

    #[test]
    fn kontakt_block_extracts() {
        let imp =
            "<h4>Haben Sie Schrott oder Metalle zu verkaufen?<br>Kontaktieren Sie uns jetzt</h4>\
            <p><a href=\"https://www.northdata.de/x\">Innstr. 10, D-68199 Mannheim</a></p>\
            <p>kontakt@mis-buntmetall.de</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Innstr. 10");
        assert_eq!(info.postcode, "68199");
        assert_eq!(info.city, "Mannheim");
        assert_eq!(info.phone, "");
        assert_eq!(info.email, "kontakt@mis-buntmetall.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}
