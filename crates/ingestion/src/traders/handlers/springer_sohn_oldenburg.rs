//! Springer & Sohn (Oldenburg): acceptance list without prices.
//!
//! The Wertstoffannahme page carries NO static euro prices in its HTML
//! (verified live 28.09.2026: zero `€` signs, zero `EUR` tokens, zero
//! price-like numbers — the only price references are two links to PDF
//! downloads: `Kundenpreisliste_08.08.2026.pdf` and the
//! Zuzahlung-Abfälle-PDF). PDF-only lists are rejected as a price source
//! (nordkat precedent), so this handler records acceptances for the
//! proven categories under "Materialien für die wir zahlen" (YOOtheme
//! card grid, `<h3 class="el-title">` titles, desktop + mobile grid
//! duplicated → deduped) plus contact enrichment. Zero prices with
//! resolved acceptances is normal operation, not a canary trip.

use scraper::{ElementRef, Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "ni-oldenburg-springer-sohn-schrott-metallhandel-conta";
/// Bespoke, live-verified acceptance page. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://www.springer-und-sohn.de/wertstoffannahme/";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.springer-und-sohn.de/impressum/";

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
            None => skipped_labels.push(format!("{label}{}", skip_reason(&label))),
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

/// Explicit label → acceptances. Generic labels map to generic materials
/// ("Aluminium" → aluminium-gemischt, never a specific alloy); unqualified
/// "Guss" is the scrap trade's shorthand for Eisenguss (the catalog groups
/// it as "Eisenschrott / Gussbruch"). Anything without a catalog fit is
/// skipped loudly — a wrong acceptance is worse than a logged gap.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("edelstahl") || l.trim() == "va" {
        Some(vec![("edelstahl-gemischt", "")])
    } else if l.contains("eisen") && l.contains("stahl") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("bremsscheibe") {
        // Grauguss-Bauteil → Gussbruch, Eigenbezeichnung als conditions.
        Some(vec![("eisenschrott-gussbruch", "Bremsscheiben")])
    } else if l.trim() == "guss" {
        Some(vec![("eisenschrott-gussbruch", "")])
    } else if l.contains("rotguss") {
        Some(vec![("bronze-rotguss", "")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("aluminium") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("zinn") {
        Some(vec![("zinn", "")])
    } else if l.contains("blei") && !l.contains("batterie") {
        Some(vec![("blei", "")])
    } else if l.contains("motor") {
        Some(vec![("elektromotoren", "")])
    } else {
        None
    }
}

/// Loud skip reasons (nordkat pattern): proposals, not guesses.
fn skip_reason(label: &str) -> &'static str {
    let l = label.to_lowercase();
    if l.contains("batterie") {
        " (kein Batterie-Material im Katalog)"
    } else if l.contains("altauto") {
        " (kein Auto-Material im Katalog)"
    } else if l.contains("blech") || l.contains("schmelz") {
        " (generisch, kein Sortenmaterial)"
    } else if l.contains("späne") || l.contains("spaene") {
        " (kein Stahlspäne-Material im Katalog)"
    } else if l.contains("kabel") {
        " (mehrdeutig: Kupfer vs. Alu)"
    } else {
        " (kein Katalogmaterial)"
    }
}

fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    // Card titles between the acceptance heading and the charged-waste
    // section. Windowed, never whole-page: footer/nav repeat the words.
    let start = html
        .find("Materialien f&uuml;r die wir zahlen")
        .or_else(|| html.find("Materialien für die wir zahlen"))
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("in Rechnung stellen").unwrap_or(tail.len());
    let window = &tail[..end];
    let doc = Html::parse_fragment(window);
    let h3 = Selector::parse("h3").expect("valid selector");
    let mut out: Vec<String> = Vec::new();
    for el in doc.select(&h3) {
        let t = el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if t.is_empty() || t.len() > 60 {
            continue;
        }
        // Desktop + mobile grids repeat every card: dedupe, keep order.
        if !out.contains(&t) {
            out.push(t);
        }
    }
    if out.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    Ok(out)
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after the
/// "Angaben gemäß § 5 TMG" heading holds firm + street + PLZ city
/// (`<br>`-separated), and the `<p>` after the "Kontakt" heading holds the
/// labeled "Telefon:" / "E-Mail:" lines. Missing headings mean the page
/// changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let anchor = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().contains("Angaben gem"));
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
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
            let t = strip_fragment(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
    // "Viktoriastraße 10" / "26135 Oldenburg" (last two lines).
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if lines.len() >= 2 {
        let last = lines.last().expect("len checked");
        let mut it = last.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                let rest: Vec<&str> = it.collect();
                city = if rest.is_empty() {
                    ci.to_owned()
                } else {
                    format!("{ci} {}", rest.join(" "))
                };
                street = lines[lines.len() - 2].clone();
            }
        }
    }
    // Labeled contact lines after the "Kontakt" heading.
    let kontakt = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Kontakt");
    let Some(kontakt) = kontakt else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let kontakt_p = kontakt
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let mut phone = String::new();
    let mut email = String::new();
    if let Some(p) = kontakt_p {
        for part in p.inner_html().split("<br") {
            let t = strip_fragment(part);
            if let Some(v) = t.strip_prefix("Telefon:") {
                phone = v.trim().to_owned();
            } else if let Some(v) = t.strip_prefix("E-Mail:") {
                // Own rule: one token (phone-style filters stop at letters).
                email = v.split_whitespace().next().unwrap_or_default().to_owned();
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

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant (` class="…"`) — drop everything up to the first '>' first, or
/// the attributes parse as text.
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
    use super::{extract_info, grade_for, parse, skip_reason};

    // Real excerpt shape of the live page (28.09.2026): YOOtheme card
    // titles, desktop + mobile grids duplicated, windowed by the two
    // section headings. Entities kept verbatim (`&uuml;`).
    const FIXTURE: &str = "<h2><a>Materialien f&uuml;r die wir zahlen</a></h2>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Aluminium</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Edelstahl / VA</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Eisen &amp; Stahl</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Kupfer</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Guss</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Bleibatterien</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Elektro-Motore</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Bremsscheiben</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Rotguss</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Stahlsp&auml;ne &amp; Metallsp&auml;ne</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Kabel</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Aluminium</h3>\
        <h3 class=\"el-title uk-h3 uk-margin-top uk-margin-remove-bottom\">Kupfer</h3>\
        <h2>Materialien, die wir in Rechnung stellen</h2>";

    #[test]
    fn list_parses_windowed_and_deduped() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 11, "{labels:?}");
        assert_eq!(labels[0], "Aluminium");
        assert!(labels.contains(&"Eisen & Stahl".to_owned()));
        assert!(labels.contains(&"Stahlspäne & Metallspäne".to_owned()));
        // No heading, no charged-waste leakage.
        assert!(!labels.iter().any(|l| l.contains("Rechnung")));
        // Missing anchor fails loudly, never empty success.
        assert!(parse("<html><body><p>Neu hier</p></body></html>").is_err());
    }

    #[test]
    fn mapping_covers_live_categories() {
        assert_eq!(
            grade_for("Aluminium"),
            Some(vec![("aluminium-gemischt", "")])
        );
        assert_eq!(
            grade_for("Edelstahl / VA"),
            Some(vec![("edelstahl-gemischt", "")])
        );
        assert_eq!(grade_for("VA"), Some(vec![("edelstahl-gemischt", "")]));
        assert_eq!(grade_for("Eisen & Stahl"), Some(vec![("mischschrott", "")]));
        assert_eq!(grade_for("Kupfer"), Some(vec![("kupfer-gemischt", "")]));
        assert_eq!(grade_for("Messing"), Some(vec![("messing", "")]));
        assert_eq!(
            grade_for("Guss"),
            Some(vec![("eisenschrott-gussbruch", "")])
        );
        assert_eq!(
            grade_for("Bremsscheiben"),
            Some(vec![("eisenschrott-gussbruch", "Bremsscheiben")])
        );
        assert_eq!(grade_for("Rotguss"), Some(vec![("bronze-rotguss", "")]));
        assert_eq!(grade_for("Blei"), Some(vec![("blei", "")]));
        assert_eq!(grade_for("Zink"), Some(vec![("zink", "")]));
        assert_eq!(grade_for("Zinn"), Some(vec![("zinn", "")]));
        assert_eq!(
            grade_for("Elektro-Motore"),
            Some(vec![("elektromotoren", "")])
        );
        // Loud skips with proposals, never guesses.
        assert_eq!(grade_for("Bleibatterien"), None);
        assert!(skip_reason("Bleibatterien").contains("Batterie"));
        assert_eq!(grade_for("Altautos"), None);
        assert_eq!(grade_for("Bleche / Schmelz"), None);
        assert_eq!(grade_for("Stahlspäne & Metallspäne"), None);
        assert_eq!(grade_for("Kabel"), None, "Cu vs. Alu mehrdeutig");
    }

    #[test]
    fn impressum_tmg_and_kontakt_blocks() {
        // Real fragment shape of the live impressum (28.09.2026).
        let imp = "<h2 class=\"wp-block-heading\">Angaben gemäß § 5 TMG</h2>\
            <p class=\"wp-block-paragraph\">Springer &amp; Sohn GmbH &amp; Co. KG<br>Viktoriastraße 10<br>26135 Oldenburg</p>\
            <h2 class=\"wp-block-heading\">Kontakt</h2>\
            <p class=\"wp-block-paragraph\">Telefon: +49 (0) 4 41 / 92074 – 0<br>E-Mail: springer@springer-und-sohn.de</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Viktoriastraße 10");
        assert_eq!(info.postcode, "26135");
        assert_eq!(info.city, "Oldenburg");
        assert_eq!(info.phone, "+49 (0) 4 41 / 92074 – 0");
        assert_eq!(info.email, "springer@springer-und-sohn.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(extract_info("<h2>Angaben gemäß § 5 TMG</h2><p>X</p>").is_err());
    }
}
