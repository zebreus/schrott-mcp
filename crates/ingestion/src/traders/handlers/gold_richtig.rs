//! GOLD richtig (Peter Zytelewski), Sulzbach: JS case — no static prices.
//! The "Unsere aktuellen Ankaufspreise" block ships placeholders only
//! (`<strong data-home-rate="gold-750">–</strong>`, `aria-busy="true"`;
//! the Gold-/Silber-Ankaufrechner pages likewise show "Rechner wird
//! geladen …" and "Kursgrundlage Feingold: –"). Prices appear only after
//! JS execution, so nothing is calculated or faked here: this handler is
//! contact + acceptance only (esh.rs pattern).
//!
//! Static acceptance evidence comes from the alloy chips
//! (`span.home-rate-alloy`: "750 Gold", "585 Gold", "333 Gold", "999
//! Silber") and the condition categories (`div.home-rates-percent-item`:
//! "Goldmünzen & Barren", "Tragbarer Goldschmuck", "Altgold, Zahngold &
//! Bruchgold"). Chips map to fineness variants, the mixed category fans
//! out to gold + zahngold with the raw label as conditions.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "sl-sulzbach-66280-gold-richtig-peter-zytelewski";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.sulzbach-goldankauf.de/impressum.html";

pub const URL: &str = "https://www.sulzbach-goldankauf.de/";
/// History: the rates block used to live at goldpit.de/gold-ankaufrechner.html
/// (30.09.2026: domain repurposed as marketplace, rechner 404). The trader's
/// own homepage carries the identical static block, live-verified.

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
    let (labels, mut skipped_labels) = parse(&html)?;
    let mut acceptances = Vec::with_capacity(labels.len());
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
            // No catalog material for precious metal: loud skip, kept
            // for traceability.
            None => skipped_labels.push(format!("{label} (kein Katalogmaterial: Edelmetall)")),
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

/// Explicit label → acceptances. Alloy chips map to fineness variants
/// ("750 Gold" → `gold`/`750`, "999 Silber" → `silber`/`999`); the mixed
/// "Altgold, Zahngold & Bruchgold" category fans out to gold + zahngold
/// with the raw label as conditions. Anything else skips loudly.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    // Mixed category first: it names both metals, so it fans out.
    // (The raw label rides in the acceptance record itself.)
    if l.contains("altgold") {
        return Some(vec![
            ("gold", "Altgold & Bruchgold"),
            ("zahngold", "Dentalgold"),
        ]);
    }
    if l.contains("zahngold") {
        return Some(vec![("zahngold", "")]);
    }
    if l.contains("goldmünzen") || l.contains("barren") {
        return Some(vec![("gold", "Münzen & Barren")]);
    }
    if l.contains("schmuck") {
        return Some(vec![("gold", "Schmuck")]);
    }
    if l.contains("silber") {
        return Some(vec![("silber", fineness(&l))]);
    }
    if l.contains("gold") {
        return Some(vec![("gold", fineness(&l))]);
    }
    None
}

/// First fineness run in the label ("750 Gold" → "750").
fn fineness(l: &str) -> &'static str {
    for fin in ["999", "916", "900", "875", "750", "585", "375", "333"] {
        if l.contains(fin) {
            return fin;
        }
    }
    ""
}

/// Parse the static acceptance evidence in the rates window between
/// "Unsere aktuellen Ankaufspreise" and "Kursgrundlage". Returns
/// (labels, js_notes): the JS placeholder cells carry no prices, so the
/// JS-only state is reported as one loud note, never a faked row.
fn parse(html: &str) -> Result<(Vec<String>, Vec<String>), IngestError> {
    let start = html
        .find("Unsere aktuellen Ankaufspreise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankaufsblock fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Kursgrundlage")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankaufsblock unvollständig".to_owned(),
        })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let alloy_sel = Selector::parse("span.home-rate-alloy").expect("valid selector");
    let cat_sel = Selector::parse("div.home-rates-percent-item > span").expect("valid selector");
    let rate_sel = Selector::parse("strong[data-home-rate]").expect("valid selector");
    let mut labels = Vec::new();
    for el in frag.select(&alloy_sel).chain(frag.select(&cat_sel)) {
        let t = el
            .text()
            .collect::<String>()
            .replace(['\u{a0}'], " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if !t.is_empty() && t.len() <= 120 && !labels.contains(&t) {
            labels.push(t);
        }
    }
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    // JS evidence: rate cells hold "–" until the calculator script runs.
    let mut js_notes = Vec::new();
    let values: Vec<String> = frag
        .select(&rate_sel)
        .map(|el| el.text().collect::<String>().trim().to_owned())
        .collect();
    if !values.is_empty() && values.iter().all(|v| v == "–" || v == "-" || v.is_empty()) {
        js_notes.push(
            "Ankaufspreise nur per JS-Rechner (data-home-rate ohne statischen Wert), keine Grammpreise übernommen"
                .to_owned(),
        );
    }
    Ok((labels, js_notes))
}

/// Bespoke contact extraction for THIS impressum only: the
/// `data-impressum-module="anbieter"` card (heading "Angaben gemäß § 5
/// Digitale-Dienste-Gesetz (DDG)") holds the owner line, the
/// `data-contact-address-html` span ("Bahnhofstr. 9 / 66280 Sulzbach /
/// Saar") and the `data-setting-text="firma.email|firma.festnetz"` contact
/// lines. Missing card → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let card_sel =
        Selector::parse("div[data-impressum-module=\"anbieter\"]").expect("valid selector");
    let card = doc
        .select(&card_sel)
        .next()
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Anbieter-Block fehlt".to_owned(),
        })?;
    let addr_sel = Selector::parse("span[data-contact-address-html]").expect("valid selector");
    let addr_html = card.select(&addr_sel).next().map(|el| el.inner_html());
    let Some(addr_html) = addr_html else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let addr_lines: Vec<String> = addr_html
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if addr_lines.len() >= 2 {
        street = addr_lines[0].clone();
        let mut it = addr_lines[1].split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| format!("{a} {b}"));
            }
        }
    }
    // Contact lines carry data-setting-text markers (firma.email,
    // firma.festnetz, firma.mobil) — select them directly.
    let any_sel = Selector::parse("[data-setting-text]").expect("valid selector");
    let mut phone = String::new();
    let mut email = String::new();
    for el in card.select(&any_sel) {
        let key = el.value().attr("data-setting-text").unwrap_or_default();
        let value = el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        match key {
            "firma.email" if email.is_empty() => email = value,
            "firma.festnetz" if phone.is_empty() => phone = value,
            _ => {}
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

/// Strip tags from a `<br>`-split fragment (drop up to the first '>'
/// first so attributes never parse as text).
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

    // Real shape of the live rates block (data-home-rate placeholders
    // with "–", alloy chips, percent categories), trimmed.
    const FIXTURE: &str = "<h2>Unsere aktuellen Ankaufspreise</h2>\
        <div class=\"home-rates-grid\" role=\"list\">\
        <div class=\"home-rate-item\"><span class=\"home-rate-alloy\">750 Gold</span>\
        <span class=\"home-rate-value\">bis zu <strong data-home-rate=\"gold-750\">–</strong></span></div>\
        <div class=\"home-rate-item\"><span class=\"home-rate-alloy\">999 Silber</span>\
        <span class=\"home-rate-value\">bis zu <strong data-home-rate=\"silver-999\">–</strong></span></div>\
        </div><div class=\"home-rates-percentages-grid\" role=\"list\">\
        <div class=\"home-rates-percent-item\"><span>Goldmünzen &amp; Barren</span>\
        <strong data-home-gold-percent=\"anlagegold\">–</strong></div>\
        <div class=\"home-rates-percent-item\"><span>Altgold, Zahngold &amp; Bruchgold</span>\
        <strong data-home-gold-percent=\"altgold\">–</strong></div>\
        </div><div class=\"home-rates-stand\">Kursgrundlage</div>";

    #[test]
    fn acceptance_labels_and_js_note() {
        let (labels, notes) = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 4);
        assert!(labels.contains(&"750 Gold".to_owned()));
        assert!(labels.contains(&"999 Silber".to_owned()));
        assert!(labels.contains(&"Goldmünzen & Barren".to_owned()));
        assert!(labels.contains(&"Altgold, Zahngold & Bruchgold".to_owned()));
        assert_eq!(notes.len(), 1);
        assert!(notes[0].contains("JS-Rechner"));
        assert!(parse("<div>Redesign ohne Block</div>").is_err());
        assert!(parse("<div>Unsere aktuellen Ankaufspreise ohne Ende").is_err());
    }

    #[test]
    fn fineness_variants_and_fanout() {
        assert_eq!(grade_for("750 Gold"), Some(vec![("gold", "750")]));
        assert_eq!(grade_for("585 Gold"), Some(vec![("gold", "585")]));
        assert_eq!(grade_for("333 Gold"), Some(vec![("gold", "333")]));
        assert_eq!(grade_for("999 Silber"), Some(vec![("silber", "999")]));
        assert_eq!(
            grade_for("Goldmünzen & Barren"),
            Some(vec![("gold", "Münzen & Barren")])
        );
        assert_eq!(
            grade_for("Tragbarer Goldschmuck"),
            Some(vec![("gold", "Schmuck")])
        );
        assert_eq!(
            grade_for("Altgold, Zahngold & Bruchgold"),
            Some(vec![
                ("gold", "Altgold & Bruchgold"),
                ("zahngold", "Dentalgold"),
            ])
        );
        assert_eq!(grade_for("Ankaufsbedingungen"), None);
    }

    #[test]
    fn impressum_anbieter_card() {
        let imp = "<div class=\"card\" data-impressum-module=\"anbieter\">\
            <h3>Angaben gemäß § 5 Digitale-Dienste-Gesetz (DDG)</h3><p>\
            <span>Peter Zytelewski</span><br>\
            <span data-contact-address-html=\"\">Bahnhofstr. 9<br>66280 Sulzbach / Saar</span>\
            <br><br>E-Mail: <a data-setting-href=\"firma.emailHref\" \
            data-setting-text=\"firma.email\">mail@sulzbach-goldankauf.de</a>\
            <br>Festnetz: <a data-setting-text=\"firma.festnetz\">06897-9388920</a></p></div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Bahnhofstr. 9");
        assert_eq!(info.postcode, "66280");
        assert_eq!(info.city, "Sulzbach / Saar");
        assert_eq!(info.phone, "06897-9388920");
        assert_eq!(info.email, "mail@sulzbach-goldankauf.de");
        assert!(super::extract_info("<div>Neu hier</div>").is_err());
    }
}
