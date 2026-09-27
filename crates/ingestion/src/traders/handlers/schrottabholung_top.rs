//! schrottabholung-top (Bochum-Wiemelhausen): acceptance list without
//! prices. The "Schrottpreise" page quotes no fixed euro amounts anywhere —
//! it says prices depend on metal type and market value and that scrap is
//! checked individually for an offer ("prüfen wir Ihren Schrott vorab").
//! What the page does give is a fixed acceptance list under "Welche Arten
//! von Schrott nehmen wir als Schrotthändler an?" (5 `<li>` items). This
//! handler fills `trader_materials` plus contact enrichment. Zero prices
//! with resolved acceptances is normal operation, not a canary trip.

use scraper::{ElementRef, Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "nw-bochum-wiemelhausen-44799-schrottabholung-top";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://schrottabholung-top.de/impressum/";

pub const URL: &str = "https://schrottabholung-top.de/schrottpreise/";

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
        // "Edelstahl und Kabelschrott" maps partially: the steel half is a
        // real acceptance, the cable half cannot be attributed (Cu vs. Al
        // share unknown) and is logged loudly below instead of guessed.
        if let Some(remainder) = unmapped_remainder(&label) {
            skipped_labels.push(remainder);
        }
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

/// Explicit label → acceptances. "Buntmetalle" fans out to the four named
/// metals. Catch-all generics ("Metallschrott: Von Eisen, Stahl bis hin zu
/// Kupfer und Aluminium") span iron and non-iron and are skipped: a wrong
/// acceptance is worse than a logged gap. "Autoteile" (vehicles, bodies,
/// batteries) has no catalog material and is skipped with a proposal in
/// the step detail.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("buntmetalle") {
        Some(vec![
            ("kupfer-gemischt", ""),
            ("messing", ""),
            ("aluminium-gemischt", ""),
            ("zink", ""),
        ])
    } else if l.contains("metallschrott") {
        // Catch-all across Eisen/Stahl/Kupfer/Alu — unattributable.
        None
    } else if l.contains("autoteile") {
        // Schrottfahrzeuge/Karosserien/Batterien: no catalog material.
        None
    } else if l.contains("eisenschrott") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("aluminium") || l.contains("alu") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("edelstahl") {
        // V2A/V4A split unknown → mixed grade.
        Some(vec![("edelstahl-gemischt", "")])
    } else {
        None
    }
}

/// The unattributable remainder of a partially mapped label, if any.
/// Currently only "Kabelschrott" (Cu/Al share unknown — neither
/// `kabel-kupfer` nor `kabel-alu` may be guessed).
fn unmapped_remainder(label: &str) -> Option<String> {
    if label.to_lowercase().contains("kabelschrott") {
        Some(format!("{label} [Kabelschrott-Anteil: Cu/Al unklar]"))
    } else {
        None
    }
}

fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    // Acceptance items live between the list heading and the "Warum uns"
    // block that follows it — never the whole page (footer/city links
    // must not become acceptances).
    let start = html
        .find("Welche Arten von Schrott nehmen wir")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Warum uns als Schrotthändler")
        .unwrap_or(tail.len());
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

/// Bespoke contact extraction for THIS impressum only (live Elementor
/// markup): `<h2>Impressum</h2>` followed by one `<p>` with firm line +
/// name + "street, PLZ city" + a "Mob. …" span. The contact e-mail lives
/// in the page header (`hallo@schrottabholung-top.de`) and is picked up
/// best-effort from the page text. Missing heading/address block → loud
/// error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let anchor = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Impressum");
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    };
    let addr_p = anchor
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let Some(p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let mut lines = Vec::new();
    for part in p.inner_html().split("<br") {
        let t = strip_fragment(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    // "Girondelle 90, 44799 Bochum": street is everything before the
    // 5-digit PLZ token, city the token after it.
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for line in &lines {
        let toks: Vec<&str> = line.split_whitespace().collect();
        for (k, t) in toks.iter().enumerate() {
            if t.len() == 5 && t.chars().all(|c| c.is_ascii_digit()) {
                if let Some(ci) = toks.get(k + 1) {
                    if ci.chars().next().is_some_and(|c| c.is_uppercase()) {
                        postcode = (*t).to_owned();
                        city = (*ci).to_owned();
                        street = toks[..k].join(" ").trim_end_matches(',').to_owned();
                        break;
                    }
                }
            }
        }
        if !postcode.is_empty() {
            break;
        }
    }
    // "Mob. 0176 62683328" line: phone-style tokens only.
    let mut phone = String::new();
    for line in &lines {
        if let Some(rest) = line.strip_prefix("Mob.") {
            phone = rest
                .split_whitespace()
                .take_while(|t| {
                    t.chars()
                        .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ");
            if !phone.is_empty() {
                break;
            }
        }
    }
    // Email: prefer a mailto: href; else scan per-element own text.
    // Whole-doc text glues block boundaries ("Haftung für
    // Inhalte"+"hallo@…"), but the <div> holding the address has it as
    // its own direct text — ancestors only hold whitespace.
    let mailto_sel = Selector::parse("a[href^=\"mailto:\"]").expect("valid selector");
    let mut email = doc
        .select(&mailto_sel)
        .filter_map(|a| a.value().attr("href"))
        .map(|h| h.trim_start_matches("mailto:").trim().to_owned())
        .next()
        .unwrap_or_default();
    if email.is_empty() {
        let any_sel = Selector::parse("*").expect("valid selector");
        'scan: for el in doc.select(&any_sel) {
            let own: String = el
                .children()
                .filter_map(|n| n.value().as_text().map(|t| t.to_string()))
                .collect::<Vec<_>>()
                .join(" ");
            for tok in own.split_whitespace() {
                let tok = tok.trim_matches(|c: char| "()<>;,".contains(c));
                if tok.contains('@') && tok.split('@').nth(1).is_some_and(|d| d.contains('.')) {
                    email = tok.to_owned();
                    break 'scan;
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

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant (` />`, `class="…"`) — drop everything up to the first '>'
/// first, or the attributes parse as text.
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
    use super::{grade_for, parse, unmapped_remainder};

    // Real live markup (trimmed to the anchored list window).
    const FIXTURE: &str = "<h4>Welche Arten von Schrott nehmen wir als Schrotthändler an?</h4>\
        <ul>\
        <li><b>Eisenschrott</b>: Von alten Geräten bis hin zu Stahlträgern – wir kaufen Ihren Eisenschrott zu Top-Preisen.</li>\
        <li><strong>Metallschrott</strong>: Von Eisen, Stahl bis hin zu Kupfer und Aluminium – wir kaufen alle Arten von Schrott und Altmetallen.</li>\
        <li><b>Buntmetalle</b><span>: Darunter fallen Kupfer, Messing, Aluminium und Zink.</span></li>\
        <li><b>Edelstahl und Kabelschrott</b><span>: Auch für diese Schrottarten bieten wir attraktive Preise in NRW.</span></li>\
        <li><strong>Autoteile</strong>: Schrottfahrzeuge und ihre Einzelteile wie Motoren, Karosserien und Batterien werden von uns fachgerecht entsorgt.</li>\
        </ul>\
        <h3>Warum uns als Schrotthändler in NRW. wählen?</h3>";

    #[test]
    fn list_window_parses_five_items() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 5);
        assert!(labels[0].starts_with("Eisenschrott"));
        assert!(labels[2].starts_with("Buntmetalle"));
        // Missing anchors fail loudly, never silently empty.
        assert!(parse("<html><body><p>Neu hier</p></body></html>").is_err());
    }

    #[test]
    fn mapping_fans_out_and_skips_loudly() {
        assert_eq!(
            grade_for("Eisenschrott: Von alten Geräten"),
            Some(vec![("mischschrott", "")])
        );
        // Catch-all across iron + non-iron: unattributable.
        assert_eq!(
            grade_for("Metallschrott: Von Eisen, Stahl bis hin zu Kupfer und Aluminium"),
            None
        );
        // Fan-out: one label, four materials.
        assert_eq!(
            grade_for("Buntmetalle: Darunter fallen Kupfer, Messing, Aluminium und Zink."),
            Some(vec![
                ("kupfer-gemischt", ""),
                ("messing", ""),
                ("aluminium-gemischt", ""),
                ("zink", ""),
            ])
        );
        assert_eq!(
            grade_for("Edelstahl und Kabelschrott: Auch für diese Schrottarten"),
            Some(vec![("edelstahl-gemischt", "")])
        );
        // Vehicles/batteries: no catalog material.
        assert_eq!(
            grade_for("Autoteile: Schrottfahrzeuge und Karosserien"),
            None
        );
    }

    #[test]
    fn kabel_remainder_is_logged_not_guessed() {
        assert!(unmapped_remainder("Edelstahl und Kabelschrott: Auch für diese").is_some());
        assert_eq!(unmapped_remainder("Eisenschrott: Von alten Geräten"), None);
        assert_eq!(
            unmapped_remainder("Buntmetalle: Darunter fallen Kupfer"),
            None
        );
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp =
            "<h2>Impressum</h2><p><strong>Schrottabholung-top</strong><br />Nader Kaisar<br />\
            Girondelle 90, 44799 Bochum<br />\
            <span aria-label=\"0176 62683328 anrufen\">Mob. 0176 62683328</span></p>\
            <h4>Haftung für Inhalte</h4><div>hallo@schrottabholung-top.de</div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Girondelle 90");
        assert_eq!(info.postcode, "44799");
        assert_eq!(info.city, "Bochum");
        assert_eq!(info.phone, "0176 62683328");
        assert_eq!(info.email, "hallo@schrottabholung-top.de");
        // Redesign without anchors fails loudly.
        assert!(super::extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
