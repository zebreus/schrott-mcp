//! Willi Kalkmann GbR (Neuss): acceptance list without prices (the
//! `<ul>` after "Metalle, die von uns angekauft werden, sind
//! beispielsweise:", plus the "Sie haben alte Kühler, …" follow-up
//! sentence). Prices are day-prices by phone only ("Anhand
//! tagesaktueller Preise ermitteln wir den Ankaufspreis"), so this
//! handler only fills `trader_materials` plus contact enrichment.
//! Zero prices with resolved acceptances is normal operation.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "nw-neuss-willi-kalkmann";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.kalkmann-metalle.de/impressum/";

pub const URL: &str = "https://kalkmann-metalle.de/#ankauf";

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

/// Explicit label → acceptances. "Aluminium (gemischt, rein, Späne,
/// Offset)" and "Messing (auch Späne)" fan out per variant; "rein"
/// has no catalog material and rides along silently inside the mapped
/// rows (reported here, not crammed into a wrong material).
/// "Verhüttung" is a process, "Hartmetall" has no catalog material
/// (same as vedder), "Träger- und Kernschrott" is heavy steel scrap
/// with no exact catalog fit, and bare "Kabel"/"Kühler"/
/// "Durchlauferhitzer" are unattributable (Cu vs Alu, device vs
/// metal) — all skipped loudly, never guessed.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("alu") {
        Some(vec![
            ("aluminium-gemischt", ""),
            ("aluminium-gemischt", "Späne"),
            ("aluminium-blech", "Offset"),
        ])
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("messing") && l.contains("späne") {
        Some(vec![("messing", ""), ("messing", "Späne")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("wasserh") {
        // Taps are brass fittings (catalog: "Messing aus Armaturen").
        Some(vec![("messing", "Wasserhähne")])
    } else if l.contains("rotguss") {
        Some(vec![("bronze-rotguss", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("blei") {
        Some(vec![("blei", "")])
    } else if l.contains("zinn") {
        Some(vec![("zinn", "")])
    } else if l.contains("edelstahl") || l.contains("(va)") {
        // Bare "VA" with no V2A/V4A grade → the generic material.
        Some(vec![("edelstahl-gemischt", "")])
    } else if l.contains("guß") || l.contains("guss") {
        Some(vec![("eisenschrott-gussbruch", "")])
    } else if l.contains("mischschrott") {
        Some(vec![("mischschrott", "")])
    } else {
        None
    }
}

fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    // List items between the offer heading and the "Wie funktioniert"
    // accordion: the <ul> grades plus the "Sie haben alte …" product
    // sentence. The "Was kaufen wir nicht an?" exclusion box sits AFTER
    // the end anchor and must never leak into acceptances.
    let start = html
        .find("Metalle, die von uns angekauft werden")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Wie funktioniert der Metallankauf?")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeblock-Ende fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    let li = Selector::parse("li").expect("valid selector");
    let mut labels: Vec<String> = doc
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
    // "Sie haben alte Kühler, Kabel, Durchlauferhitzer, Wasserhähne
    // oder andere Metallprodukte?" — extra bought products outside the
    // <ul>, same bespoke window.
    let s = window
        .find("Sie haben alte")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Produkt-Satz fehlt".to_owned(),
        })?;
    let rest = &window[s + "Sie haben alte".len()..];
    let e = rest
        .find(" oder andere Metallprodukte?")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Produkt-Satz-Ende fehlt".to_owned(),
        })?;
    for part in rest[..e].split(',') {
        let t = part.split_whitespace().collect::<Vec<_>>().join(" ");
        if !t.is_empty() {
            labels.push(t);
        }
    }
    Ok(labels)
}

/// Bespoke contact extraction for THIS impressum only: labeled
/// `<p>` rows ("Firmenname:", "Telefon:" … "Telefax", "Adresse:
/// Osterather Straße 7, 41460 Neuss (Deutschland)") and the
/// `mailto:` link. Missing anchors mean the page changed shape →
/// loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    if !imp.contains("Firmenname:") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmenname-Block fehlt".to_owned(),
        });
    }
    let doc = Html::parse_document(imp);
    let all = doc
        .root_element()
        .text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    // "Adresse: Osterather Straße 7, 41460 Neuss (Deutschland)".
    let (_, addr) = all
        .split_once("Adresse:")
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adresse-Block fehlt".to_owned(),
        })?;
    let addr_end = addr.find("Geschäftsführung:").unwrap_or(addr.len());
    let addr = addr[..addr_end].trim();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if let Some((s, rest)) = addr.split_once(',') {
        street = s.trim().to_owned();
        let toks: Vec<&str> = rest.split_whitespace().collect();
        if toks.len() >= 2
            && toks[0].len() == 5
            && toks[0].chars().all(|c| c.is_ascii_digit())
            && toks[1].chars().next().is_some_and(|c| c.is_uppercase())
        {
            postcode = toks[0].to_owned();
            city = toks[1].trim_matches(|c| c == '(' || c == ')').to_owned();
        }
    }
    // Phone between its own label and "Telefax" (the en dash in
    // "02131 – 541561" is not a phone-token char, so marker-slicing
    // beats token-filtering here).
    let phone = all
        .split_once("Telefon:")
        .and_then(|(_, r)| r.split_once("Telefax").map(|(p, _)| p.trim().to_owned()))
        .unwrap_or_default();
    // Email lives in the mailto: link; fall back to the token after
    // the "E-Mail:" label.
    let mailto = Selector::parse("a[href^=\"mailto:\"]").expect("valid selector");
    let mut email = doc
        .select(&mailto)
        .filter_map(|a| a.value().attr("href"))
        .map(|h| h.trim_start_matches("mailto:").trim().to_owned())
        .next()
        .unwrap_or_default();
    if email.is_empty() {
        email = all
            .split_once("E-Mail:")
            .map(|(_, r)| {
                r.split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .trim()
                    .to_owned()
            })
            .unwrap_or_default();
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

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    const FIXTURE: &str = "<p><strong>Metalle, die von uns angekauft werden, sind beispielsweise:</strong></p>\
        <ul><li>Aluminium (gemischt, rein, Späne, Offset)</li>\
        <li>Kupfer (auch Installations-Kupfer)</li>\
        <li>Messing (auch Späne)</li><li>Rotguss</li><li>Zink</li><li>Blei</li>\
        <li>Edelstahl (VA)</li><li>Zinn</li><li>Verhüttung</li><li>Hartmetall</li>\
        <li>Gußeisen</li><li>Mischschrott</li><li>Träger- und Kernschrott</li>\
        <li>und viele weitere</li></ul>\
        <p>Sie haben alte Kühler, Kabel, Durchlauferhitzer, Wasserhähne oder andere Metallprodukte? Selbstverständlich kaufen wir auch diese an.</p>\
        <h3><span class=\"inner\">Wie funktioniert der Metallankauf?</span></h3>";

    #[test]
    fn list_and_sentence_parse() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 18);
        assert_eq!(labels[0], "Aluminium (gemischt, rein, Späne, Offset)");
        assert_eq!(labels[13], "und viele weitere");
        assert_eq!(
            &labels[14..],
            &["Kühler", "Kabel", "Durchlauferhitzer", "Wasserhähne"]
        );
        // Anchors are mandatory: redesign fails loudly.
        assert!(parse("<p>Neu hier</p>").is_err());
        assert!(parse("<p>Metalle, die von uns angekauft werden</p>").is_err());
    }

    #[test]
    fn impressum_labeled_rows() {
        let imp = "<h3>Angaben gemäß Digitale-Dienste-Gesetz (DDG)</h3>\
            <p><strong>Firmenname:</strong> Willi Kalkmann GbR</p>\
            <p><strong>Telefon:</strong> 02131 – 541561<br />\
            <strong>Telefax:</strong> 02131 – 2032440<br />\
            <strong>E-Mail:</strong> <a href=\"mailto:info@kalkmann-metalle.de\">info@kalkmann-metalle.de</a></p>\
            <p><strong>Adresse:</strong> Osterather Straße 7, 41460 Neuss (Deutschland)</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Osterather Straße 7");
        assert_eq!(info.postcode, "41460");
        assert_eq!(info.city, "Neuss");
        assert_eq!(info.phone, "02131 – 541561");
        assert_eq!(info.email, "info@kalkmann-metalle.de");
        assert!(super::extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }

    #[test]
    fn mapping_fans_out_and_skips_loudly() {
        assert_eq!(
            grade_for("Aluminium (gemischt, rein, Späne, Offset)"),
            Some(vec![
                ("aluminium-gemischt", ""),
                ("aluminium-gemischt", "Späne"),
                ("aluminium-blech", "Offset"),
            ])
        );
        assert_eq!(
            grade_for("Kupfer (auch Installations-Kupfer)"),
            Some(vec![("kupfer-gemischt", "")])
        );
        assert_eq!(
            grade_for("Messing (auch Späne)"),
            Some(vec![("messing", ""), ("messing", "Späne")])
        );
        assert_eq!(grade_for("Rotguss"), Some(vec![("bronze-rotguss", "")]));
        assert_eq!(
            grade_for("Edelstahl (VA)"),
            Some(vec![("edelstahl-gemischt", "")])
        );
        assert_eq!(
            grade_for("Gußeisen"),
            Some(vec![("eisenschrott-gussbruch", "")])
        );
        assert_eq!(grade_for("Mischschrott"), Some(vec![("mischschrott", "")]));
        assert_eq!(
            grade_for("Wasserhähne"),
            Some(vec![("messing", "Wasserhähne")])
        );
        assert_eq!(grade_for("Verhüttung"), None, "process, not a material");
        assert_eq!(grade_for("Hartmetall Widia"), None, "no catalog material");
        assert_eq!(
            grade_for("Träger- und Kernschrott"),
            None,
            "no exact catalog fit"
        );
        assert_eq!(grade_for("Kühler"), None, "Cu vs Alu unattributable");
        assert_eq!(grade_for("Kabel"), None, "Cu vs Alu unattributable");
        assert_eq!(
            grade_for("Durchlauferhitzer"),
            None,
            "device, not a material"
        );
        assert_eq!(grade_for("und viele weitere"), None, "filler prose");
    }
}
