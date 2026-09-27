//! Schrott Anton (München): acceptance list without prices. The price
//! page ("aktuelle-schrottpreise-münchen") publishes no fixed prices —
//! only a Preis-Anfrageformular whose material `<select>` carries the
//! bought grades as `<option>` values. This handler turns those options
//! into `trader_materials` acceptances plus contact enrichment. Zero
//! prices with resolved acceptances is normal operation, not a canary
//! trip.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "by-munchen-schrott-anton";
/// Bespoke, live-verified impressum URL (linked in the site footer as
/// "Impressum"). A move fails the step loudly (fix the URL) — never
/// guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.schrott-anton.de/impressum-schrott/";

pub const URL: &str = "https://www.schrott-anton.de/aktuelle-schrottpreise-muenchen/";

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

/// Explicit option → acceptances. Unattributable kabel grades ("mit/ohne
/// Stecker", "Litzen-", "Schlitzkabel" name no metal — neither
/// `kabel-kupfer` nor `kabel-alu` may be guessed), "Bremsscheiben",
/// "Schwerschrott" and "Träger" have no catalog material and are skipped
/// loudly (see proposals in the step report).
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") || l.contains("milberry") {
        // Live typo on this page: "Kupfer rein (Milberry)".
        Some(vec![("kupfer-millberry", "")])
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("unrein") {
        Some(vec![("aluminium-gemischt", "unrein")])
    } else if l.contains("aluminium") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("e-motor") || l.contains("e-motoren") || l.contains("elektromotor") {
        Some(vec![("elektromotoren", "")])
    } else if l.contains("mischschrott") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("v2a") || l.contains("edelstahl") {
        Some(vec![("edelstahl-v2a", "")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("zinn") {
        Some(vec![("zinn", "")])
    } else if l.contains("blei") {
        Some(vec![("blei", "")])
    } else {
        None
    }
}

/// The bought grades are the `<option>` values of the request form's
/// material `<select>` (anchored on `<select` … `</select>`). The
/// placeholder option ("Material auswählen", empty value) is dropped;
/// a missing select or zero options is a loud error, never an empty
/// success (a redesign would otherwise look like "trader buys nothing").
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let start = html.find("<select").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Anfrage-Formular fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("</select>").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Anfrage-Formular ohne Ende".to_owned(),
    })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<select>{window}</select>"));
    let option = Selector::parse("option").expect("valid selector");
    let labels: Vec<String> = doc
        .select(&option)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|t| !t.is_empty() && t != "Material auswählen")
        .collect();
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    Ok(labels)
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` holding
/// "Schrott-Sam GmbH" right after the "Angaben gemäß § 5 TMG:" heading
/// carries firm + street ("Lerchenstr" + house number in a spam-guard
/// span) + PLZ city over `<br>` lines, and a second `<p>` carries the
/// labeled "Telefon:" line. The e-mail is CleanTalk-masked on purpose
/// and stays empty. Missing anchors → loud error, never a guess.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h3 = Selector::parse("h3").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let anchor = doc.select(&h3).find(|h| {
        h.text().collect::<String>().contains("Angaben gemäß § 5 TMG")
    });
    let Some(_) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    let firm_p = doc.select(&p).find(|e| {
        e.text().collect::<String>().contains("Schrott-Sam GmbH")
    });
    let Some(firm_p) = firm_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmen-Block fehlt".to_owned(),
        });
    };
    // "<p>Schrott-Sam GmbH<br />Lerchenstr&nbsp;<span>19</span><br
    // />80959&nbsp;München</p>" — split on "<br", drop tag remnants first
    // (attributes would parse as text), entities already decoded.
    let lines: Vec<String> = firm_p
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    for line in &lines {
        if line.contains("Lerchenstr") {
            street = line.clone();
        }
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| a + " " + b);
            }
        }
    }
    // Phone: the "<p>…Telefon:&nbsp;<span>+49 89 …</span></p>" block.
    let mut phone = String::new();
    if let Some(tel_p) = doc.select(&p).find(|e| {
        e.text().collect::<String>().contains("Telefon:")
    }) {
        let body: String =
            tel_p.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ");
        if let Some(i) = body.find("Telefon:") {
            phone = body[i + "Telefon:".len()..]
                .split_whitespace()
                .take_while(|t| {
                    t.chars().all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ");
        }
    }
    if street.is_empty() && phone.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo { street, postcode, city, phone, email: String::new() })
}

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant (` / class="…"`) — drop everything up to the first '>' first,
/// or attributes parse as text.
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
    out.replace("&nbsp;", " ").split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    // Real shape of the live form (Forminator select, attributes trimmed,
    // options verbatim).
    const FIXTURE: &str = "<form><select name=\"select-1\">\
        <option value=\"\">Material auswählen</option>\
        <option value=\"Aluminium rein\">Aluminium rein</option>\
        <option value=\"Kabel ohne Stecker\">Kabel ohne Stecker</option>\
        <option value=\"Kupfer rein (Milberry)\">Kupfer rein (Milberry)</option>\
        <option value=\"Mischschrott\">Mischschrott</option>\
        <option value=\"V2A Edelstahl\">V2A Edelstahl</option>\
        <option value=\"Bremsscheiben\">Bremsscheiben</option>\
        </select></form>";

    #[test]
    fn impressum_angaben_block() {
        let imp = "<h2>Impressum</h2><h3>Angaben gemäß § 5 TMG:</h3>\
            <p>Schrott-Sam GmbH<br />Lerchenstr&nbsp;<span>19</span><br />80959&nbsp;München</p>\
            <p><strong>Kontakt:</strong><br />Telefon:&nbsp;<span>+49 89 48 956 853</span></p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Lerchenstr 19");
        assert_eq!(info.postcode, "80959");
        assert_eq!(info.city, "München");
        assert_eq!(info.phone, "+49 89 48 956 853");
        assert!(info.email.is_empty(), "e-mail is spam-masked live");
        assert!(super::extract_info("<h3>Anderes</h3>").is_err());
        assert!(super::extract_info(
            "<h3>Angaben gemäß § 5 TMG:</h3><p>Leer</p>"
        )
        .is_err());
    }

    #[test]
    fn options_map_and_skip() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 6, "placeholder dropped: {labels:?}");
        assert_eq!(
            grade_for("Kupfer rein (Milberry)"),
            Some(vec![("kupfer-millberry", "")])
        );
        assert_eq!(
            grade_for("Kupfer rein (Millberry)"),
            Some(vec![("kupfer-millberry", "")])
        );
        assert_eq!(
            grade_for("Aluminium rein"),
            Some(vec![("aluminium-gemischt", "")])
        );
        assert_eq!(
            grade_for("Aluminium unrein"),
            Some(vec![("aluminium-gemischt", "unrein")])
        );
        assert_eq!(
            grade_for("V2A Edelstahl"),
            Some(vec![("edelstahl-v2a", "")])
        );
        assert_eq!(
            grade_for("Mischschrott"),
            Some(vec![("mischschrott", "")])
        );
        assert_eq!(grade_for("Kabel ohne Stecker"), None, "metal unnamed");
        assert_eq!(grade_for("Litzenkabel"), None);
        assert_eq!(grade_for("Bremsscheiben"), None, "no cast-iron material");
        assert_eq!(grade_for("Schwerschrott"), None);
        assert_eq!(grade_for("Träger"), None);
        // Missing form / empty options fail loudly.
        assert!(parse("<html><body>Kein Formular</body></html>").is_err());
        assert!(parse("<select><option value=\"\">Material auswählen</option></select>").is_err());
    }
}
