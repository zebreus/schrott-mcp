//! Moroder Scheideanstalt (Essen): acceptance-only handler. REVIEW DECISION
//! (27.09.2026): the page's "Kurse" JSON feed (`/moroder-price-bar-data`:
//! GOLD/oz, SILBER/oz …) is a world-market SPOT ticker, not Moroder's own
//! Ankaufspreise — recording it as `haendler_angabe` would fake provenance,
//! so it is deliberately NOT parsed (an earlier revision did; removed in
//! review). Likewise the visible price tables are empty `<tbody>` shells
//! for sell-side investment products, and the Zahngold/Altgold category
//! pages carry prose but no concrete buy prices.
//!
//! What the price page does prove: the "Zahngold verkaufen" / "Altgold
//! verkaufen" buying categories. Those two anchors become acceptances
//! (`zahngold`, `gold`); missing anchors fail loudly. Contact enrichment
//! via the RDFa impressum block.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-essen-moroder-scheideanstalt";
/// Bespoke, live-verified impressum URL (the site's own footer link).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.moroder-scheideanstalt.de/rechtliches/impressum/";

pub const URL: &str = "https://www.moroder-scheideanstalt.de/aktuelle-preisliste/";

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

/// Explicit label → acceptances. Only the two buying categories the
/// price page itself names. Anything else skips loudly.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    if l.contains("zahngold") {
        Some(vec![("zahngold", "")])
    } else if l.contains("altgold") {
        Some(vec![("gold", "")])
    } else {
        None
    }
}

/// The buying categories on the price page ("Zahngold verkaufen" /
/// "Altgold verkaufen"). Both anchors are mandatory — without them the
/// page proves no take-back claim and the step fails loudly.
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let mut labels = Vec::new();
    for anchor in ["Zahngold verkaufen", "Altgold verkaufen"] {
        if html.contains(anchor) {
            labels.push(anchor.to_owned());
        }
    }
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankaufskategorien fehlen".to_owned(),
        });
    }
    Ok(labels)
}

/// Bespoke contact extraction for THIS impressum only: the RDFa block
/// `div.impressum-rdfa` with `property="streetAddress|postalCode|
/// addressLocality|telephone|email"` spans ("Kaninenberghöhe 2", "45136",
/// "Essen", "Telefon: +49 201 74 74 790", "E-Mail:
/// info@moroder-scheideanstalt.de"). Missing block or properties → loud
/// error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let block_sel = Selector::parse("div.impressum-rdfa").expect("valid selector");
    let prop_sel = Selector::parse("[property]").expect("valid selector");
    let block = doc.select(&block_sel).next().ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Impressum-Block fehlt".to_owned(),
    })?;
    let mut prop = std::collections::HashMap::new();
    for el in block.select(&prop_sel) {
        if let Some(name) = el.value().attr("property") {
            let text = el.text().collect::<String>();
            prop.entry(name.to_owned()).or_insert(text);
        }
    }
    if prop.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let street = prop.get("streetAddress").map(|s| s.trim().to_owned()).unwrap_or_default();
    let postcode = prop.get("postalCode").map(|s| s.trim().to_owned()).unwrap_or_default();
    let city = prop.get("addressLocality").map(|s| s.trim().to_owned()).unwrap_or_default();
    let phone = prop
        .get("telephone")
        .map(|s| s.strip_prefix("Telefon:").unwrap_or(s).trim().to_owned())
        .unwrap_or_default();
    let email = prop
        .get("email")
        .map(|s| s.strip_prefix("E-Mail:").unwrap_or(s).trim().to_owned())
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

    #[test]
    fn categories_parse_and_map() {
        let html = "<h1>Aktuelle Preisliste</h1><p>Sie möchten Zahngold verkaufen\
            oder Altgold verkaufen? <a>Kurse</a></p>";
        assert_eq!(
            parse(html).expect("parses"),
            vec!["Zahngold verkaufen", "Altgold verkaufen"]
        );
        assert_eq!(
            grade_for("Zahngold verkaufen"),
            Some(vec![("zahngold", "")])
        );
        assert_eq!(grade_for("Altgold verkaufen"), Some(vec![("gold", "")]));
        assert_eq!(grade_for("Silber verkaufen"), None);
        assert!(parse("<p>Nur Kurse, kein Ankauf</p>").is_err());
    }

    #[test]
    fn impressum_rdfa_block() {
        let imp = "<div class=\"impressum-rdfa\" vocab=\"http://schema.org/\" typeof=\"Organization\">\
            <span property=\"name\"><strong>Moroder Scheideanstalt GmbH</strong></span><br>\
            <div property=\"address\" typeof=\"PostalAddress\">\
            <span property=\"streetAddress\">Kaninenberghöhe 2</span><br>\
            <span property=\"postalCode\">45136</span> \
            <span property=\"addressLocality\">Essen</span>, \
            <span property=\"addressCountry\">Deutschland</span></div><br>\
            <span property=\"telephone\"><strong>Telefon:</strong> +49 201 74 74 790</span><br>\
            <a property=\"email\" href=\"mailto:info@moroder-scheideanstalt.de\">\
            <strong>E-Mail:</strong> info@moroder-scheideanstalt.de</a><br></div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Kaninenberghöhe 2");
        assert_eq!(info.postcode, "45136");
        assert_eq!(info.city, "Essen");
        assert_eq!(info.phone, "+49 201 74 74 790");
        assert_eq!(info.email, "info@moroder-scheideanstalt.de");
        assert!(super::extract_info("<div>Neu hier</div>").is_err());
    }
}
