//! Metallhandel Jens Jacob (Leipzig): acceptance list WITHOUT prices.
//! The only price publication is a PDF
//! (`media/files/preisliste-standard_2026-08-17.pdf`, live, ~140 kB) —
//! the workspace has no PDF-text dependency and none is added, so prices
//! stay out of reach and this handler fills `trader_materials` plus
//! contact enrichment from the "Dienstleistungen" page ("Unser Angebot
//! umfasst folgende Leistungen:" followed by a 5-item `<ul>`, terminated
//! by the Absetzcontainer sentence). Zero prices with resolved
//! acceptances is normal operation, not a canary trip. Should an HTML
//! price table ever appear, this handler wants a price parse on top.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "sn-leipzig-metallhandel-jens-jacob";
/// Bespoke, live-verified impressum URL (site nav's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "http://www.metallhandel-jacob.de/impressum.html";

pub const URL: &str = "http://www.metallhandel-jacob.de/dienstleistungen.html";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
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

/// Explicit label → acceptances. Generic iron scrap lands on generic
/// `mischschrott` (esh precedent); bare "Buntmetall", "Kabelschrott"
/// (copper vs. aluminium?) and "Elektroschrott" (no generic e-scrap
/// material in the catalog) stay `None`: a wrong acceptance is worse
/// than a logged gap. Proposals: `buntmetall-gemischt`, `kabel-gemischt`
/// (or a Kupfer-default policy), `elektroschrott-gemischt`.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("eisenmetall") || l.contains("eisenschrott") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("katalysator") {
        Some(vec![("katalysatoren", "")])
    } else {
        None
    }
}

/// The offer list between its heading sentence and the Absetzcontainer
/// terminator. Missing anchors → `Err`, never an empty success.
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let start = html.find("Unser Angebot umfasst folgende Leistungen:").ok_or_else(|| {
        IngestError::Parse { url: URL.to_owned(), detail: "Angebotsliste fehlt".to_owned() }
    })?;
    let tail = &html[start..];
    let end = tail.find("Absetzcontainer").unwrap_or(tail.len());
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<ul>{window}</ul>"));
    let li = Selector::parse("li").expect("valid selector");
    let labels: Vec<String> = doc
        .select(&li)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|t| !t.is_empty() && t.len() <= 120)
        .collect();
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Angebotsliste leer".to_owned(),
        });
    }
    Ok(labels)
}

/// Bespoke contact extraction for THIS impressum only: `<h1>Impressum</h1>`
/// followed by the address `<p>` ("Dortmunder Str.12<br>04357 Leipzig<br>
/// Telefon: …<br>Email: mailto-link<br>Internet: …"). The street line
/// glues the house number ("Str.12") — deglued to "Str. 12" with a test.
/// Missing anchors → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let a = Selector::parse("a").expect("valid selector");
    if !doc.select(&h1).any(|h| h.text().collect::<String>().trim() == "Impressum") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let addr_p = doc.select(&p).find(|el| {
        let t: String = el.text().collect();
        t.contains("Dortmunder Str.")
    });
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let addr_lines: Vec<String> = addr_p
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in addr_lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| format!("{a} {b}"));
                if k > 0 {
                    street = deglue_number(&addr_lines[k - 1]);
                }
                break;
            }
        }
    }
    let mut phone = String::new();
    for part in addr_p.inner_html().split("<br") {
        let t = strip_fragment(part);
        if let Some(v) = t.strip_prefix("Telefon:") {
            phone = v.trim().to_owned();
            break;
        }
    }
    // E-mail per mailto-href, never per token split (neighbour nodes glue
    // together without whitespace in scraper text output).
    let email = addr_p
        .select(&a)
        .filter_map(|el| el.value().attr("href"))
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
    Ok(TraderInfo { street, postcode, city, phone, email })
}

/// "Dortmunder Str.12" → "Dortmunder Str. 12": split a house number glued
/// onto the street token ("Str.12" → "Str." + "12"). Anything else passes
/// through untouched.
fn deglue_number(s: &str) -> String {
    let mut toks: Vec<String> = s.split_whitespace().map(str::to_owned).collect();
    if let Some(last) = toks.last() {
        let cut = last
            .char_indices()
            .rev()
            .take_while(|(_, c)| c.is_ascii_digit() || *c == '-')
            .last()
            .map(|(i, _)| i);
        if let Some(i) = cut {
            let head_ends = last[..i].ends_with(' ') || last[..i].ends_with('-');
            if i > 0 && !head_ends {
                let head = last[..i].to_owned();
                let tail = last[i..].to_owned();
                if head.ends_with('.') || head.len() > 2 {
                    toks.pop();
                    toks.push(head);
                    toks.push(tail);
                }
            }
        }
    }
    toks.join(" ")
}

/// Strip tags from a `<br>`-split fragment. Fragments start with a tag
/// remnant — drop everything up to the first '>' first, or attributes
/// parse as text.
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
    use super::{deglue_number, grade_for, parse};

    // Real shape of the live Dienstleistungen page: the offer sentence,
    // the 5-item <ul>, the Absetzcontainer terminator.
    const FIXTURE: &str = "<h2>Dienstleistungen</h2>\
        <p>Unser Angebot umfasst folgende Leistungen:</p>\
        <ul><li>Eisenmetall</li><li>Buntmetall</li><li>Kabelschrott</li>\
        <li>Elektroschrott</li><li>Katalysatoren</li></ul>\
        <p>Zusätzlich sind wir in der Lage Ihnen <a href=\"transportbehaelter.html\">Absetzcontainer</a>\
        oder Gitterboxen zu stellen.</p>";

    #[test]
    fn list_window_and_mapping() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(
            labels.iter().map(String::as_str).collect::<Vec<_>>(),
            vec![
                "Eisenmetall",
                "Buntmetall",
                "Kabelschrott",
                "Elektroschrott",
                "Katalysatoren"
            ]
        );
        assert_eq!(grade_for("Eisenmetall"), Some(vec![("mischschrott", "")]));
        assert_eq!(grade_for("Katalysatoren"), Some(vec![("katalysatoren", "")]));
        // No generic bunt/cable/e-scrap material: loud gaps, not guesses.
        assert_eq!(grade_for("Buntmetall"), None);
        assert_eq!(grade_for("Kabelschrott"), None);
        assert_eq!(grade_for("Elektroschrott"), None);
        assert!(parse("<div>Redesign ohne Liste</div>").is_err());
    }

    #[test]
    fn impressum_glued_street_and_mailto() {
        let imp = "<h1>Impressum</h1>\
            <p>Dortmunder Str.12<br>04357 Leipzig<br>Telefon: +49 341 58573 0<br>\
            Telefax: +49 341 58573 33<br>Email: \
            <a href=\"mailto:info@metallhandel-jacob.de\">info@metallhandel-jacob.de</a><br>\
            Internet: <a href=\"http://www.metallhandel-jacob.de\">www.metallhandel-jacob.de</a></p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Dortmunder Str. 12");
        assert_eq!(info.postcode, "04357");
        assert_eq!(info.city, "Leipzig");
        assert_eq!(info.phone, "+49 341 58573 0");
        assert_eq!(info.email, "info@metallhandel-jacob.de");
        assert_eq!(deglue_number("Dortmunder Str.12"), "Dortmunder Str. 12");
        assert_eq!(deglue_number("Mannheimer Str. 65-67"), "Mannheimer Str. 65-67");
        assert!(super::extract_info("<h1>Neu hier</h1>").is_err());
    }
}
