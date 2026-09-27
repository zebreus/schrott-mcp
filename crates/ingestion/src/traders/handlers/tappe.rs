//! Tappe Rohstoffhandel (Essen): price block embedded in the homepage,
//! with the price date next to the heading ("Aktuelle Schrottpreise /
//! 24.09.2026"). Labels and prices arrive as alternating text nodes;
//! multi-line labels ("Kupferschrott 1" + "ECU/Milb.") accumulate until
//! a price node closes the pair. Daily schedule: they refresh mornings.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-essen-vogelheim-tappe-rohstoffhandel";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.tappe-recycling.de/impressum";

pub const URL: &str = "https://www.tappe-recycling.de/";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::DailyAt { times: vec![(7, 30)] },
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (published_at, rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit,
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label,
            }),
            None => skipped_labels.push(label),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        prices,
        acceptances: vec![],
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at,
    })
}

/// Explicit mapping; generic page labels land on the closest grade and
/// keep the raw label in `notes` for traceability.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("schredder") || l.contains("shredder") {
        Some(("stahlschrott-shredder", ""))
    } else if l.contains("kabel") {
        Some(("kabel-kupfer", "40%"))
    } else if l.contains("kupferschrott 1") || l.contains("ecu") || l.contains("milb") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kupferschrott 2") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("kupfer") {
        Some(("kupfer-berry", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("alu") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("edelstahl") || l.contains("va ") || l == "va" {
        Some(("edelstahl-gemischt", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else {
        None
    }
}

fn parse(
    html: &str,
) -> Result<(Option<String>, Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    // Only the price box: from its heading to the contact link run-out.
    let start = html.find("Aktuelle Schrottpreise").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisbox fehlt".to_owned(),
    })?;
    let window = &html[start..(start + html[start..].len().min(12_000))];
    let published_at = date_in(window);
    // Collect <p> texts in order.
    let mut texts = Vec::new();
    let mut rest = window;
    while let Some(a) = rest.find("<p") {
        let after = &rest[a..];
        let Some(b) = after.find('>') else { break };
        let after = &after[b + 1..];
        let Some(c) = after.find("</p>") else { break };
        let mut t = after[..c].to_owned();
        // strip nested tags, decode the entities we care about
        loop {
            let Some(x) = t.find('<') else { break };
            let Some(y) = t[x..].find('>') else { break };
            t.replace_range(x..x + y + 1, " ");
        }
        let t = t
            .replace("&nbsp;", " ")
            .replace("&#160;", " ")
            .replace(['\u{a0}'], " ");
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if !t.is_empty() {
            texts.push(t);
        }
        rest = &after[c + 4..];
    }
    let mut rows = Vec::new();
    let mut unit_skips = Vec::new();
    let mut pending: Vec<String> = Vec::new();
    // Page-global unit (the "€ pro kg" header): rows carry no unit of
    // their own here. No detectable header unit means no rows at all —
    // a silent EUR/kg default would risk 1000x errors.
    let mut page_unit: Option<&'static str> = None;
    for t in texts {
        if is_price(&t) {
            if let Some(price) = parse_eur(&t) {
                let label = pending.join(" ").trim().to_owned();
                pending.clear();
                if label.is_empty() {
                    continue;
                }
                let unit = unit_of(&t).or(if t.contains('/') || t.to_lowercase().contains("pro") {
                    // Explicit but unknown unit ("pro Sack"): skip loudly
                    // instead of inheriting the page default.
                    None
                } else {
                    page_unit
                });
                let Some(unit) = unit else {
                    unit_skips.push(format!("{label} (Einheit unverständlich: {t})"));
                    continue;
                };
                rows.push((label, price, unit));
            }
        } else if is_header(&t) {
            if page_unit.is_none() {
                page_unit = unit_of(&t);
            }
            pending.clear();
            // The price box ends at the "... auf Anfrage" terminator:
            // anything after it is footer/nav, never prices.
            if t.contains("Anfrage") {
                break;
            }
        } else {
            pending.push(t);
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "keine Preispaare".to_owned() });
    }
    Ok((published_at, rows, unit_skips))
}

fn is_price(t: &str) -> bool {
    t.contains('€') && parse_eur(t).is_some()
}

/// Bespoke unit matcher for THIS page (live: "€ pro kg" header, bare
/// "0,170 €" rows). Only kg/t exist here — anything else returns None
/// and the caller decides (page default vs. loud skip).
fn unit_of(t: &str) -> Option<&'static str> {
    let lower = t.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower.split(|c: char| !c.is_alphanumeric()).any(|w| w == "t" || w == "to") {
        Some("EUR/t")
    } else {
        None
    }
}

fn is_header(t: &str) -> bool {
    // Any euro text without digits ("€ pro kg", "€/kg") is a header, not a
    // label: otherwise it would glue itself onto the next material.
    if t.contains('€') && parse_eur(t).is_none() {
        return true;
    }
    matches!(t, "Schrottsorte" | "Aktuelle Schrottpreise")
        || t.contains("weitere")
        || t.contains("Anfrage")
        || t.len() > 120
}

fn date_in(window: &str) -> Option<String> {
    let bytes = window.as_bytes();
    let mut i = 0;
    while i + 10 <= bytes.len() {
        if bytes[i].is_ascii_digit()
            && bytes[i + 2] == b'.'
            && bytes[i + 5] == b'.'
            && bytes[i + 6..].iter().take(4).all(|c| c.is_ascii_digit())
        {
            let (d, m, y) = (&window[i..i + 2], &window[i + 3..i + 5], &window[i + 6..i + 10]);
            if let Some(rfc) = parse_de_date(d, m, y) {
                return Some(rfc);
            }
        }
        i += 1;
    }
    None
}


/// Bespoke contact extraction for THIS impressum only: the `<dl>` carries
/// labeled rows ("Anschrift:" → "Am Stadthafen 18, 45356 Essen").
/// Missing anchors mean the page changed shape → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let dt = Selector::parse("dt").expect("valid selector");
    let dd = Selector::parse("dd").expect("valid selector");
    let dts: Vec<_> = doc.select(&dt).collect();
    let dds: Vec<_> = doc.select(&dd).collect();
    if dts.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-dl fehlt".to_owned(),
        });
    }
    let value_of = |want: &str| {
        dts.iter()
            .zip(dds.iter())
            .find(|(t, _)| t.text().collect::<String>().trim() == want)
            .map(|(_, d)| {
                d.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ")
            })
    };
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if let Some(addr) = value_of("Anschrift:") {
        // "Am Stadthafen 18, 45356 Essen"
        if let Some((left, right)) = addr.split_once(',') {
            street = left.trim().to_owned();
            let mut it = right.split_whitespace();
            if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    city = ci.to_owned();
                }
            }
        }
    }
    let phone = value_of("Telefon:").unwrap_or_default();
    let mut email = value_of("E-Mail:").unwrap_or_default();
    email = email.replace('\u{2202}', "@");
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
    use super::{date_in, grade_for, parse};

    const FIXTURE: &str = "<p><strong>Aktuelle Schrottpreise </strong></p><p>24.09.2026</p>\
        <p>Schrottsorte</p><p>€ pro kg</p>\
        <p>Mischschrott</p><p>0,170 €</p>\
        <p>Kupferschrott 1</p><p>ECU/Milb.</p><p>11,20 €</p>\
        <p>... weitere auf Anfrage</p>";

    #[test]
    fn impressum_definition_list() {
        let imp = "<dl><dt>Firmenname:</dt><dd>Tappe Rohstoffhandel GmbH</dd>            <dt>Anschrift:</dt><dd>Am Stadthafen 18, 45356 Essen</dd>            <dt>Telefon:</dt><dd>0201 / 61 44 122</dd>            <dt>Telefax:</dt><dd>0201 / 61 44 129</dd>            <dt>E-Mail:</dt><dd>tappe-recycling\u{2202}t-online.de</dd></dl>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Am Stadthafen 18");
        assert_eq!(info.postcode, "45356");
        assert_eq!(info.city, "Essen");
        assert_eq!(info.phone, "0201 / 61 44 122");
        assert_eq!(info.email, "tappe-recycling@t-online.de");
        assert!(super::extract_info("<dl><dt>Nix</dt><dd>da</dd></dl>").is_err());
    }

    #[test]
    fn pairs_date_and_mapping() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-24T00:00:00+00:00"));
        assert_eq!(rows.len(), 2);
        assert!(skips.is_empty());
        assert_eq!(rows[0].0, "Mischschrott");
        assert_eq!(rows[0].1, 0.17);
        assert_eq!(rows[1].0, "Kupferschrott 1 ECU/Milb.");
        assert_eq!(rows[1].1, 11.2);
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Kupferschrott 1 ECU/Milb."), Some(("kupfer-millberry", "")));
        assert_eq!(grade_for("Kabelschrott (Basis 40% Kupfer)"), Some(("kabel-kupfer", "40%")));
        assert_eq!(grade_for("Schredderschrott"), Some(("stahlschrott-shredder", "")));
        assert_eq!(grade_for("Aluminium"), Some(("aluminium-gemischt", "")));
        assert_eq!(grade_for("Edelstahl"), Some(("edelstahl-gemischt", "")));
        assert_eq!(grade_for("Kupferschrott 2"), Some(("kupfer-gemischt", "")));
    }

    #[test]
    fn terminator_and_unit_safety() {
        // Euro junk after the terminator never pairs up.
        let html = FIXTURE.to_owned() + "<p>Container ab 49 €</p><p>Anfahrt 10 €</p>";
        let (_, rows, _) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 2);
        // "€/kg"-style headers glue onto nothing; unknown units skip loudly.
        let html = FIXTURE
            .replace("€ pro kg", "€/kg")
            .replace("0,170 €", "0,170 € pro Sack");
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "Kupferschrott 1 ECU/Milb.");
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Mischschrott"));
    }

    #[test]
    fn date_scan() {
        assert_eq!(
            date_in("Preise 24.09.2026 Liste").as_deref(),
            Some("2026-09-24T00:00:00+00:00")
        );
        assert_eq!(date_in("kein Datum"), None);
    }
}
