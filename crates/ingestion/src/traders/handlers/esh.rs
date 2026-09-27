//! ESH Darmstadt: acceptance list without prices ("Wir bieten Ankauf …"
//! followed by <li> grades, terminated by the "Von:" customer list).
//! No prices, no page date — this handler only fills `trader_materials`
//! plus contact enrichment. Zero prices with resolved acceptances is
//! normal operation, not a canary trip.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "he-darmstadt-nord-esh-darmstadt";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.esh-darmstadt.de/impressum";

pub const URL: &str = "https://www.esh-darmstadt.de/services/schrotthandel-demontage/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| {
        Box::pin(scrape(c))
    } }
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

/// Explicit label → acceptances. One label may fan out to several
/// materials ("Edelstahl (V2A und V4A)"). Unattributable generics
/// ("Bleche", "Metallspäne", "Stahlschrott", "Nickel") are skipped:
/// a wrong acceptance is worse than a logged gap.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("v2a") && l.contains("v4a") {
        Some(vec![("edelstahl-v2a", ""), ("edelstahl-v4a", "")])
    } else if l.contains("edelstahlspäne") || l.contains("edelstahlsp") {
        Some(vec![("edelstahl-gemischt", "Späne")])
    } else if l.contains("alu") && !l.contains("stahl") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("eisenschrott") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("zinn") && l.contains("blei") {
        // Sloppy single line for two grades ("Zinn Blei/ / Alt Blei").
        Some(vec![("zinn", ""), ("blei", "")])
    } else if l.contains("zinn") {
        Some(vec![("zinn", "")])
    } else if l.contains("blei") {
        Some(vec![("blei", "")])
    } else {
        None
    }
}

fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    // List items between the offer heading and the customer list.
    let start = html.find("Wir bieten Ankauf").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Annahmeliste fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("Von:").unwrap_or(tail.len());
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


/// Bespoke contact extraction for THIS impressum only: the labeled inline
/// block ("ESH Darmstadt Inhaber: Rocky Truber Akazienweg 15b 64293
/// Darmstadt Mobil: … Tel.: … E-mail: …"). Anchored on "Inhaber:" —
/// without it the page changed shape → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let all = doc
        .root_element()
        .text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let (_, block) = all.split_once("Inhaber:").ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Inhaber-Block fehlt".to_owned(),
    })?;
    // "Rocky Truber Akazienweg 15b 64293 Darmstadt Mobil: …"
    let toks: Vec<&str> = block.split_whitespace().collect();
    // street: "Akazienweg 15b" (this trader's street, verified live).
    let mut street = String::new();
    for (k, t) in toks.iter().enumerate() {
        if *t == "Akazienweg" {
            if let Some(n) = toks.get(k + 1) {
                street = format!("Akazienweg {n}");
                break;
            }
        }
    }
    // postcode + city right after.
    let (mut postcode, mut city) = (String::new(), String::new());
    for (k, t) in toks.iter().enumerate() {
        if t.len() == 5 && t.chars().all(|c| c.is_ascii_digit()) {
            if let Some(ci) = toks.get(k + 1) {
                if ci.chars().next().is_some_and(|c| c.is_uppercase()) {
                    postcode = (*t).to_owned();
                    city = (*ci).to_owned();
                    break;
                }
            }
        }
    }
    let after = |marker: &str| {
        block.find(marker).map(|i| {
            block[i + marker.len()..]
                .split_whitespace()
                .take_while(|t| {
                    t.chars()
                        .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
    };
    let phone = after("Tel.:").unwrap_or_default();
    // Email is one token (the phone-style take_while above would stop at
    // the first letter — emails need their own rule).
    let email = block
        .to_lowercase()
        .find("e-mail:")
        .map(|i| {
            block[i + 7..]
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_owned()
        })
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

    const FIXTURE: &str = "<p>Wir bieten Ankauf von Altmetallen / Metallschrott:</p>\
        <ul><li>Aluminium</li><li>Edelstahl (V2A und V4A)</li>\
        <li>Bleche und Blechabschnitte</li><li>Edelstahlspäne</li>\
        <li>Eisenschrott</li><li>Nickel</li><li>Zinn Blei/ / Alt Blei</li></ul>\
        <p>Von:</p><ul><li>Privat</li></ul>";

    #[test]
    fn impressum_inhaber_block() {
        let imp = "<div>ESH Darmstadt Inhaber: Rocky Truber Akazienweg 15b             64293 Darmstadt Mobil: 0 151 / 547 747 18 Tel.: 0 6151 / 789 99 59             E-mail: info@esh-darmstadt.de Steuernummer: 00787564391</div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Akazienweg 15b");
        assert_eq!(info.postcode, "64293");
        assert_eq!(info.city, "Darmstadt");
        assert_eq!(info.phone, "0 6151 / 789 99 59");
        assert_eq!(info.email, "info@esh-darmstadt.de");
        assert!(super::extract_info("<div>Ohne Inhaber</div>").is_err());
    }

    #[test]
    fn list_maps_and_skips() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 7);
        // Fan-out: one label, two materials.
        assert_eq!(
            grade_for("Edelstahl (V2A und V4A)"),
            Some(vec![("edelstahl-v2a", ""), ("edelstahl-v4a", "")])
        );
        assert_eq!(
            grade_for("Edelstahlspäne"),
            Some(vec![("edelstahl-gemischt", "Späne")])
        );
        assert_eq!(grade_for("Eisenschrott"), Some(vec![("mischschrott", "")]));
        assert_eq!(
            grade_for("Zinn Blei/ / Alt Blei"),
            Some(vec![("zinn", ""), ("blei", "")])
        );
        assert_eq!(grade_for("Bleche und Blechabschnitte"), None);
        assert_eq!(grade_for("Nickel"), None, "no nickel material");
        assert_eq!(grade_for("Kupfer"), Some(vec![("kupfer-gemischt", "")]));
    }
}
