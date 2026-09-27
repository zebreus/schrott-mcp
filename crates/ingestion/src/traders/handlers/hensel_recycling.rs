//! Hensel Recycling GmbH (Aschaffenburg-Obernau): acceptance-only
//! handler. REVIEW DECISION: the "Edelmetallpreise" chart/table widget
//! is JS-driven (Highcharts + wp-admin/admin-ajax.php data source with
//! date-range picker) — no static per-gram quotes exist, so nothing is
//! scraped from it and no ajax-internals are chased (fragile, against
//! the guide). What the Ankauf pages statically prove is the material
//! list under "Was wir alles für Sie recyceln" (Autokatalysatoren,
//! Elektronikschrott, LKW-Katalysatoren, Industriekatalysatoren,
//! Brennstoffzellen, Weitere Materialien). Only ceramic auto/truck
//! catalysts map (`katalysatoren` + conditions); industry catalysts
//! are an unknown substrate (NORDKAT precedent), fuel cells and generic
//! e-scrap have no material, "Weitere Materialien" names nothing — all
//! skipped loudly, never guessed. Contact via the clean impressum
//! block (Mühlweg 10, 63743 Aschaffenburg).

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "by-aschaffenburg-obernau-63743-hensel-recycling";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://hensel-recycling.com/impressum/";

pub const URL: &str = "https://hensel-recycling.com/leistung/ankauf-autokatalysatoren/";

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

/// Explicit material → acceptances. Auto and LKW catalysts are ceramic
/// converter scrap (`katalysatoren` + conditions). Industry catalysts
/// are an unknown substrate (metal/chemical possible — NORDKAT
/// precedent: never hide under ceramic); fuel cells, generic e-scrap
/// and "Weitere Materialien" have no catalog material — all skipped
/// loudly with their reason in the skip label.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    if l.contains("auto") && l.contains("katalysator") {
        Some(vec![("katalysatoren", "Autokatalysatoren")])
    } else if l.contains("lkw") && l.contains("katalysator") {
        Some(vec![("katalysatoren", "LKW-Katalysatoren")])
    } else {
        None
    }
}

/// The material link list between "Was wir alles für Sie recyceln" and
/// the "Broschüren Download" section. Both anchors mandatory.
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let start = html.find("Was wir alles für Sie recyceln").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Materialliste fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("Broschüren Download").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Materialliste unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    let a_sel = Selector::parse("a[href*=\"/material/\"]").expect("valid selector");
    let mut labels = Vec::new();
    for el in doc.select(&a_sel) {
        let t: String = el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if !t.is_empty() && t.len() <= 60 && !labels.contains(&t) {
            labels.push(t);
        }
    }
    if labels.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Materialliste leer".to_owned() });
    }
    Ok(labels)
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` lines
/// ("Hensel Recycling GmbH" / "Mühlweg 10" / "63743 Aschaffenburg" /
/// "Telefon:" / "E-Mail:"). Missing firm anchor → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    if !doc
        .root_element()
        .text()
        .collect::<String>()
        .contains("Hensel Recycling GmbH")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmen-Block fehlt".to_owned(),
        });
    }
    let p_sel = Selector::parse("p").expect("valid selector");
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let (mut phone, mut email) = (String::new(), String::new());
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
                if let Some(v) = line.strip_prefix("E-Mail:") {
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
    Ok(TraderInfo { street, postcode, city, phone, email })
}

/// Strip tags from a `<br`-split fragment. Fragments start either with
/// real text ("Telefon: …") or with tag/attribute remnants (" />",
/// `class="…" />`): remnants carry '=' before any '<' (or start with
/// '/'), real label text doesn't — dropping blindly up to the first
/// '>' would eat the label itself ("Telefon: <a …>" → label gone).
fn strip_fragment(s: &str) -> String {
    let mut s = s;
    if let Some(i) = s.find('<') {
        if s[..i].contains('=') {
            s = &s[s.find('>').map(|j| j + 1).unwrap_or(s.len())..];
        }
    }
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
    out.replace("&nbsp;", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_start_matches(|c| c == '/' || c == '>')
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    // Real material-link shape, trimmed to 3 links.
    const FIXTURE: &str = "<h2>Was wir alles für Sie recyceln</h2>\
        <a href=\"https://hensel-recycling.com/material/autokatalysatoren/\">Autokatalysatoren</a>\
        <a href=\"https://hensel-recycling.com/material/elektronikschrott/\">Elektronikschrott</a>\
        <a href=\"https://hensel-recycling.com/material/lkw-katalysatoren/\">LKW-Katalysatoren</a>\
        <h2>Broschüren Download</h2>";

    #[test]
    fn material_links_parse() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels, vec!["Autokatalysatoren", "Elektronikschrott", "LKW-Katalysatoren"]);
        assert!(parse("<div>Kein Ankauf hier</div>").is_err());
        assert!(parse("<h2>Was wir alles für Sie recyceln ohne Ende").is_err());
    }

    #[test]
    fn ceramic_only_maps() {
        assert_eq!(
            grade_for("Autokatalysatoren"),
            Some(vec![("katalysatoren", "Autokatalysatoren")])
        );
        assert_eq!(
            grade_for("LKW-Katalysatoren"),
            Some(vec![("katalysatoren", "LKW-Katalysatoren")])
        );
        assert_eq!(grade_for("Industriekatalysatoren"), None, "unknown substrate");
        assert_eq!(grade_for("Elektronikschrott"), None, "generic, not Platinen");
        assert_eq!(grade_for("Brennstoffzellen"), None, "no material");
        assert_eq!(grade_for("Weitere Materialien"), None, "names nothing");
    }

    #[test]
    fn impressum_firm_block() {
        let imp = "<p><strong>Hensel Recycling GmbH</strong><br />Mühlweg 10<br />\
            63743 Aschaffenburg<br />Deutschland</p>\
            <p>Telefon: <a href=\"tel:+49602812090\">+49 6028 1209-0</a><br />\
            E-Mail: <a href=\"mailto:info@hensel-recycling.com\">info@hensel-recycling.com</a></p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Mühlweg 10");
        assert_eq!(info.postcode, "63743");
        assert_eq!(info.city, "Aschaffenburg");
        assert_eq!(info.phone, "+49 6028 1209-0");
        assert_eq!(info.email, "info@hensel-recycling.com");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}
