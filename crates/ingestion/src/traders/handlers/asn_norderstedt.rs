//! Altmetall- und Schrotthandel Norderstedt (ASN): acceptance list
//! without machine-readable prices. The "Ankaufliste" page shows the
//! prices as PHOTOS (no text layer, no OCR in the pipeline) plus a
//! product gallery ("Kupfer", "Messing", "Elektrokabel", "Aluminium",
//! "Blei", "Zink", "VA", "Metallspäne", "Elektromotoren",
//! "Sondersorten auf Anfrage"). Those nine become acceptances; the
//! exclusion box ("Elektroschrott", "Bleibatterien",
//! "Umweltgefährdende Stoffe") is deliberately NOT ingested. Zero
//! prices with resolved acceptances is normal operation.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "sh-norderstedt-altmetall-und-schrotthandel-norderstedt";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://altmetall-asn.de/impressum/";

pub const URL: &str = "https://altmetall-asn.de/ankaufliste/";

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

/// Explicit label → acceptances. "Elektrokabel" is copper installation
/// cable → `kabel-kupfer` (plum precedent); "VA" without grade is the
/// generic stainless; "Metallspäne" without metal is unattributable
/// (Cu vs Alu vs Messing); "Sondersorten auf Anfrage" names nothing.
/// Exclusion-box items never reach this function (outside the window).
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("elektrokabel") {
        Some(vec![("kabel-kupfer", "Elektrokabel")])
    } else if l.contains("aluminium") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("blei") {
        Some(vec![("blei", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l == "va" {
        Some(vec![("edelstahl-gemischt", "VA")])
    } else if l.contains("elektromotoren") {
        Some(vec![("elektromotoren", "")])
    } else {
        None
    }
}

/// The gallery headings between "Unsere Ankaufliste" and the exclusion
/// box ("Sorten, die wir nicht übernehmen"). Both anchors mandatory.
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let start = html
        .find("Unsere Ankaufliste")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankaufliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    // End BEFORE the exclusion heading opens: ending mid-heading would
    // leave an unclosed <h3> whose partial text ("Es gibt auch Sorten,
    // die wir nicht") parses as a phantom label.
    let end = tail
        .find("Es gibt auch Sorten")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankaufliste unvollständig".to_owned(),
        })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    // Gallery captions live in image-overlay <p>s
    // (`div.av-image-caption-overlay-center > p`); bare prose <p>s must
    // never leak in as labels.
    let sel = Selector::parse("div.av-image-caption-overlay-center p").expect("valid selector");
    let mut labels = Vec::new();
    for el in doc.select(&sel) {
        let t: String = el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if !t.is_empty() && t.len() <= 40 && t != "Unsere Ankaufliste" && !labels.contains(&t) {
            labels.push(t);
        }
    }
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankaufliste leer".to_owned(),
        });
    }
    Ok(labels)
}

/// Bespoke contact extraction for THIS impressum only: the lines after
/// "Diensteanbieter" ("Altmetall- und Schrotthandel Norderstedt GmbH" /
/// "Schützenwall 30" / "22844 Norderstedt") plus "Telefon:"/"E-Mail-
/// Adresse:" lines. Missing firm anchor → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    if !doc
        .root_element()
        .text()
        .collect::<String>()
        .contains("Altmetall- und Schrotthandel Norderstedt GmbH")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmen-Block fehlt".to_owned(),
        });
    }
    let p_sel = Selector::parse("p").expect("valid selector");
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let (mut phone, mut email) = (String::new(), String::new());
    // Previous line across <p> boundaries: firm name, street and PLZ
    // often live in separate <p> elements (street is the line right
    // above the PLZ line — but only if it carries a house number,
    // otherwise it is the firm line and stays out).
    let mut prev_line = String::new();
    for el in doc.select(&p_sel) {
        let lines: Vec<String> = el
            .inner_html()
            .split("<br")
            .map(|s| strip_fragment(s))
            .filter(|s| !s.is_empty())
            .collect();
        for line in &lines {
            let mut it = line.split_whitespace();
            if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                if postcode.is_empty() && pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    city = ci.to_owned();
                    if prev_line.chars().any(|c| c.is_ascii_digit()) {
                        street = prev_line.clone();
                    }
                    continue;
                }
            }
            if phone.is_empty() {
                if let Some(v) = line.strip_prefix("Telefon:") {
                    phone = v.trim().to_owned();
                }
            }
            if email.is_empty() {
                if let Some(v) = line.strip_prefix("E-Mail-Adresse:") {
                    email = v.trim().to_owned();
                }
            }
            prev_line = line.clone();
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

/// Strip tags from a `<br`-split fragment (drop up to the first '>').
/// `inner_html` re-encodes nbsp as "&nbsp;" (no whitespace to split on),
/// so decode it first or addresses glue onto emails.
fn strip_fragment(s: &str) -> String {
    let s = match s.find('>') {
        Some(i) => &s[i + 1..],
        None => s,
    };
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.replace("&nbsp;", " ").chars() {
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

    // Real gallery shape (image-overlay captions + exclusion box).
    const FIXTURE: &str = "<h2>Unsere Ankaufliste</h2>\
        <p>Nachfolgend finden Sie einige Beispiele der Metalle, die wir ankaufen.</p>\
        <div class=\"av-image-caption-overlay-center\"><p>Kupfer</p></div>\
        <div class=\"av-image-caption-overlay-center\"><p>Messing</p></div>\
        <div class=\"av-image-caption-overlay-center\"><p>Elektrokabel</p></div>\
        <div class=\"av-image-caption-overlay-center\"><p>Aluminium</p></div>\
        <div class=\"av-image-caption-overlay-center\"><p>Blei</p></div>\
        <div class=\"av-image-caption-overlay-center\"><p>Zink</p></div>\
        <div class=\"av-image-caption-overlay-center\"><p>VA</p></div>\
        <div class=\"av-image-caption-overlay-center\"><p>Metallspäne</p></div>\
        <div class=\"av-image-caption-overlay-center\"><p>Elektromotoren</p></div>\
        <div class=\"av-image-caption-overlay-center\"><p>Sondersorten auf Anfrage</p></div>\
        <h3>Es gibt auch Sorten, die wir nicht übernehmen. Diese gehören unbedingt dazu!</h3>\
        <div class=\"av-image-caption-overlay-center\"><p>Elektroschrott</p></div>\
        <div class=\"av-image-caption-overlay-center\"><p>Bleibatterien</p></div>";

    #[test]
    fn gallery_parses_exclusions_stay_out() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 10);
        assert_eq!(labels[0], "Kupfer");
        assert!(labels.contains(&"Elektromotoren".to_owned()));
        assert!(!labels
            .iter()
            .any(|l| l.contains("Elektroschrott") || l.contains("Bleibatterien")));
        assert!(parse("<div>Kein Ankauf hier</div>").is_err());
        assert!(parse("<h2>Unsere Ankaufliste ohne Ende").is_err());
    }

    #[test]
    fn mapping_maps_and_skips_loudly() {
        assert_eq!(grade_for("Kupfer"), Some(vec![("kupfer-gemischt", "")]));
        assert_eq!(grade_for("Messing"), Some(vec![("messing", "")]));
        assert_eq!(
            grade_for("Elektrokabel"),
            Some(vec![("kabel-kupfer", "Elektrokabel")])
        );
        assert_eq!(
            grade_for("Aluminium"),
            Some(vec![("aluminium-gemischt", "")])
        );
        assert_eq!(grade_for("Blei"), Some(vec![("blei", "")]));
        assert_eq!(grade_for("Zink"), Some(vec![("zink", "")]));
        assert_eq!(grade_for("VA"), Some(vec![("edelstahl-gemischt", "VA")]));
        assert_eq!(
            grade_for("Elektromotoren"),
            Some(vec![("elektromotoren", "")])
        );
        assert_eq!(grade_for("Metallspäne"), None, "metal unattributable");
        assert_eq!(grade_for("Sondersorten auf Anfrage"), None, "names nothing");
    }

    #[test]
    fn impressum_diensteanbieter_block() {
        let imp = "<p>Diensteanbieter</p>\
            <p>Altmetall- und Schrotthandel Norderstedt GmbH<br />Schützenwall 30<br />\
            22844 Norderstedt<br />Deutschland</p>\
            <p>E-Mail-Adresse:\u{a0}asn@wtnet.de<br />Telefon: 040 525 61 41</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Schützenwall 30");
        assert_eq!(info.postcode, "22844");
        assert_eq!(info.city, "Norderstedt");
        assert_eq!(info.phone, "040 525 61 41");
        assert_eq!(info.email, "asn@wtnet.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}
