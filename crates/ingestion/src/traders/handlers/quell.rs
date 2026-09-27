//! Quell Recycling (Lorsch): product lists without prices (Wix homepage:
//! "Buntmetalle aller Art" line items, then an "Eisen aller Art" block
//! collapsed into one acceptance). No prices, no page date — fills
//! `trader_materials` plus contact enrichment.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "he-lorsch-quell-recycling";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.quellrecycling.de/impressum";

pub const URL: &str = "https://www.quellrecycling.de/";

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

/// Explicit label → acceptances. The Eisen block arrives pre-collapsed
/// (see parse); unknown future items are skipped loudly.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase().replace("spähne", "späne");
    let l = l.as_str();
    if l.contains("millberry") {
        Some(vec![("kupfer-millberry", "")])
    } else if l.contains("elektrolyt") {
        Some(vec![("kupfer-millberry", "Elektrolyt")])
    } else if l.contains("kupfer-rohre") || l.contains("kupferrohre") {
        Some(vec![("kupfer-gemischt", "Rohre")])
    } else if l.contains("kupfer") && l.contains("kabel") {
        Some(vec![("kabel-kupfer", "")])
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("messing") && l.contains("späne") {
        Some(vec![("messing", "Späne")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("rotguss") {
        Some(vec![("bronze-rotguss", "")])
    } else if l.contains("v2a") && l.contains("späne") {
        Some(vec![("edelstahl-gemischt", "Späne")])
    } else if l.contains("v2a") {
        Some(vec![("edelstahl-v2a", "")])
    } else if l.contains("alu") && l.contains("späne") {
        Some(vec![("aluminium-gemischt", "Späne")])
    } else if l.contains("alu") && l.contains("profile") {
        Some(vec![("aluminium-profile", "")])
    } else if l.contains("alu") && l.contains("guss") {
        Some(vec![("aluminium-guss", "")])
    } else if l.contains("alu") && l.contains("kabel") {
        Some(vec![("kabel-alu", "")])
    } else if l.contains("aluminium") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("blei") && l.contains("auswucht") {
        Some(vec![("blei", "Auswucht")])
    } else if l.contains("blei") {
        Some(vec![("blei", "")])
    } else if l.contains("e-motoren") || l.contains("motoren") {
        Some(vec![("elektromotoren", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("zinn") {
        Some(vec![("zinn", "")])
    } else if l.contains("eisen aller art") {
        Some(vec![("mischschrott", "Eisen aller Art")])
    } else {
        None
    }
}

fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    // Wix rich-text line items starting at "Aluminium" (single occurrence
    // on the page). DOM order is jumbled (the "Buntmetalle" heading sits
    // AFTER its list), so window generously and filter.
    let start = html.find("Aluminium").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Buntmetall-Liste fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail
        .find("Folgende Rohstoffe nehmen wir an")
        .unwrap_or(tail.len().min(30_000));
    let window = &tail[..end];
    let mut text = String::with_capacity(window.len() / 4);
    let mut in_tag = false;
    for c in window.chars() {
        if c == '<' {
            in_tag = true;
            text.push('\n');
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            text.push(c);
        }
    }
    let mut out: Vec<String> = Vec::new();
    for raw in text.split('\n') {
        let t = raw
            .replace("&auml;", "ä")
            .replace("&ouml;", "ö")
            .replace("&uuml;", "ü")
            .replace("&Auml;", "Ä")
            .replace("&Ouml;", "Ö")
            .replace("&Uuml;", "Ü")
            .replace("&szlig;", "ß")
            .replace("&nbsp;", " ")
            .replace(['\u{a0}', '\u{200b}'], " ");
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if t.is_empty() || t.starts_with("und vieles mehr") || t.len() > 60 {
            continue;
        }
        out.push(t);
    }
    // The Eisen block ("Stahlträger", "Blech", …) lists examples, not
    // grades: collapse everything from the "Buntmetalle aller Art" heading
    // on into one acceptance instead of N guesses.
    if let Some(pos) = out.iter().position(|l| l == "Buntmetalle aller Art") {
        out.truncate(pos);
        out.push("Eisen aller Art".to_owned());
    }
    if out.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    Ok(out)
}


/// Bespoke contact extraction for THIS impressum only: the address lines
/// after the "Angaben" heading ("Quell Recycling GmbH" /
/// "Ludwig-Erhard-Straße 30" / "64653 Lorsch") plus the labeled
/// "Telefon:" / "E-Mail:" lines. Missing heading → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let anchor = doc.select(&h1).find(|h| {
        h.text().collect::<String>().contains("Angaben")
    });
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    // Address lines: document-wide <p> scan (Wix nests the address
    // paragraphs unpredictably — sibling walks find nothing). First <p>
    // with a PLZ line wins; street is the previous line in the same <p>.
    let p = Selector::parse("p").expect("valid selector");
    let h1b = Selector::parse("h1").expect("valid selector");
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    // Address lines live inside the <h1> block itself on this page.
    for el in doc.select(&h1b).chain(doc.select(&p)) {
        let lines: Vec<String> = el
            .inner_html()
            .split("<br")
            .map(|s| strip_fragment(s))
            .filter(|s| !s.is_empty())
            .collect();
        let mut done = false;
        for (k, line) in lines.iter().enumerate() {
            let mut it = line.split_whitespace();
            if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                if pc.len() == 5
                    && pc.chars().all(|c| c.is_ascii_digit())
                    && ci.chars().next().is_some_and(|c| c.is_uppercase())
                {
                    postcode = pc.to_owned();
                    city = ci.to_owned();
                    if k > 0 {
                        street = lines[k - 1].clone();
                    }
                    done = true;
                    break;
                }
            }
        }
        if done {
            break;
        }
    }
    // Labeled contact lines anywhere in the impressum.
    let body: String = doc
        .root_element()
        .text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let after_label = |marker: &str| {
        body.find(marker).map(|i| {
            body[i + marker.len()..]
                .split_whitespace()
                .take_while(|t| {
                    t.chars()
                        .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
    };
    let phone = after_label("Telefon:").unwrap_or_default();
    let email = body
        .find("E-Mail:")
        .map(|i| {
            body[i + 7..]
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

/// Strip tags from a fragment (html5ever already decoded entities).
/// Fragments from splitting on "<br" start with a tag remnant
/// (` class="…"`) — drop everything up to the first '>' first, or the
/// attributes parse as text.
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

    const FIXTURE: &str = "<div>Aluminium<br>Alu-Sp&auml;hne<br>Alu-Profile<br>Kupfer-Millberry<br>\
        Kupfer-Kabel<br>Messing-Sp&auml;ne<br>V2a<br>Zink<br>Unbekanntes Zeug<br>\
        Buntmetalle aller Art<br>Stahlträger<br>Blech<br>Eisen aller Art</div>\
        <p>Folgende Rohstoffe nehmen wir an</p>";

    #[test]
    fn impressum_angaben_block() {
        let imp = "<h1>Angaben gem. 5 DDG</h1>            <p>Quell Recycling GmbH<br>Ludwig-Erhard-Strasse 30<br>            64653 Lorsch<br>Deutschland</p>            <p>Kontakt Telefon: +49 (0) 6251 / 52385 E-Mail: info@quellrecycling.de</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Ludwig-Erhard-Strasse 30");
        assert_eq!(info.postcode, "64653");
        assert_eq!(info.city, "Lorsch");
        assert_eq!(info.phone, "+49 (0) 6251 / 52385");
        assert_eq!(info.email, "info@quellrecycling.de");
        assert!(super::extract_info("<h1>Anderes</h1>").is_err());
    }

    #[test]
    fn list_maps_collapses_and_skips() {
        let labels = parse(FIXTURE).expect("parses");
        // Eisen items collapse into one row; unknown items survive parsing
        // (they skip at mapping, loudly).
        assert!(labels.contains(&"Eisen aller Art".to_owned()));
        assert!(!labels.iter().any(|l| l == "Stahlträger" || l == "Blech"));
        assert!(labels.contains(&"Unbekanntes Zeug".to_owned()));
        assert_eq!(
            grade_for("Kupfer-Millberry"),
            Some(vec![("kupfer-millberry", "")])
        );
        assert_eq!(
            grade_for("Messing-Späne"),
            Some(vec![("messing", "Späne")])
        );
        assert_eq!(
            grade_for("Alu-Späne"),
            Some(vec![("aluminium-gemischt", "Späne")])
        );
        // Site typo, varies between fetches: "Spähne" must work too.
        assert_eq!(
            grade_for("Alu-Spähne"),
            Some(vec![("aluminium-gemischt", "Späne")])
        );
        assert_eq!(
            grade_for("Messing-Spähne"),
            Some(vec![("messing", "Späne")])
        );
        assert_eq!(
            grade_for("Eisen aller Art"),
            Some(vec![("mischschrott", "Eisen aller Art")])
        );
        assert_eq!(grade_for("Unbekanntes Zeug"), None);
    }
}
