//! AMR Schrottplatz (Treptow-Köpenick, Berlin): four exact per-kg price
//! cards on the homepage — an Elementor `h2` "Schrottpreise" section where
//! each card pairs an `h4` label ("Kupfer Millberry") with the following
//! `h3` price ("7,00€/ kg"). The window ends at the next section heading
//! ("Ankauf bei AMR:"), so the address/contact `h3`s and the category
//! list below stay out. A dangling `h4` without a price skips loudly;
//! zero pairs is a loud error, never a silent success. No visible page
//! date (only meta modified_time), so `published_at` stays `None`.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "be-treptow-kopenick-amr-metalle-und-rohstoffe-amr-schrottpla";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "http://www.amr-schrottplatz.de/impressum-und-datenschutz/";

pub const URL: &str = "http://www.amr-schrottplatz.de";

/// Price-card window anchors on the live homepage.
const CARDS_START: &str = ">Schrottpreise</h2>";
const CARDS_END: &str = "Ankauf bei AMR:";

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
    let (rows, mut skipped_labels) = parse(&html)?;
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
        published_at: None,
    })
}

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. "Raff" without further grading is the standard mixed copper.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("raff") && !l.contains("kabel") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("kabel") {
        Some(("kabel-kupfer", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after the
/// "Impressum" heading must hold "AMR Metalle und Rohstoffe GmbH" plus
/// `<br>` lines for street, PLZ city and "Tel:"; the e-mail comes from
/// the `mailto:` href (never token-split — scraper glues neighbours).
/// Missing anchors mean the page changed shape → loud error, never a
/// guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let anchor = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Impressum");
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    };
    let firm_p = anchor
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .filter(|e| e.value().name() == "p")
        .find(|p| {
            p.text()
                .collect::<String>()
                .contains("AMR Metalle und Rohstoffe GmbH")
        });
    let Some(firm_p) = firm_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmen-Block fehlt".to_owned(),
        });
    };
    let mut lines = Vec::new();
    for part in firm_p.inner_html().split("<br") {
        let t = strip_tags(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    // "AMR Metalle und Rohstoffe GmbH" / "Schnellerstr. 20E" /
    // "12439 Berlin" / "Tel: 030 53 013 220" / "Fax: …".
    let mut street = String::new();
    let mut postcode = String::new();
    let mut city = String::new();
    let mut phone = String::new();
    for (k, line) in lines.iter().enumerate() {
        if line.starts_with("Tel:") {
            phone = line["Tel:".len()..]
                .split_whitespace()
                .take_while(|t| {
                    t.chars()
                        .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ");
        }
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = ci.to_owned();
                if k > 0 {
                    street = lines[k - 1].clone();
                }
            }
        }
    }
    let a = Selector::parse("a").expect("valid selector");
    let mut email = String::new();
    for link in firm_p.select(&a) {
        if let Some(addr) = link
            .value()
            .attr("href")
            .and_then(|h| h.strip_prefix("mailto:"))
        {
            email = addr.to_owned();
            break;
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

/// Strip tags from a fragment (entities are already decoded by html5ever).
fn strip_tags(s: &str) -> String {
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

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    // Window, never the whole page: the homepage also carries address
    // h3s ("Schnellerstr. 20E, 12439 Berlin"), a category list and phone
    // h3s that must never pair with a card label.
    let start = html.find(CARDS_START).ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Schrottpreise-Block fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find(CARDS_END).ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Schrottpreise-Block fehlt".to_owned(),
    })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    let heads = Selector::parse("h4, h3").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    let mut pending: Option<String> = None;
    for el in doc.select(&heads) {
        let text: String = el.text().collect();
        let text: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if text.is_empty() {
            continue;
        }
        if el.value().name() == "h4" {
            if let Some(label) = pending.take() {
                skips.push(format!("{label} (ohne Preis)"));
            }
            if text.len() > 120 {
                continue; // Prosa, kein Label.
            }
            pending = Some(text);
        } else if let Some(label) = pending.take() {
            let Some(price) = parse_eur(&text) else {
                skips.push(format!("{label} (Preis unverständlich: {text})"));
                continue;
            };
            // An unparseable unit is a loud skip, never a silent default.
            let Some(unit) = unit_of(&text) else {
                skips.push(format!("{label} (Einheit unverständlich: {text})"));
                continue;
            };
            rows.push((label, price, unit));
        } else {
            skips.push(format!("Preis ohne Label: {text}"));
        }
    }
    if let Some(label) = pending.take() {
        skips.push(format!("{label} (ohne Preis)"));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Schrottpreise leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS card row (live: "7,00€/ kg"). Only kg/t
/// exist here — anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase().replace([' ', '\u{a0}'], "");
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|t| t == "t")
    {
        Some("EUR/t")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    const FIXTURE: &str =
        "<h2 class=\"elementor-heading-title elementor-size-default\">Schrottpreise</h2>\
        <div><h4 class=\"elementor-heading-title elementor-size-default\">Kupfer Millberry</h4>\
        <h3 class=\"elementor-heading-title elementor-size-default\">7,00€/ kg</h3></div>\
        <div><h4 class=\"elementor-heading-title elementor-size-default\">Kupfer Kabel </h4>\
        <h3 class=\"elementor-heading-title elementor-size-default\">2,19€/ kg</h3></div>\
        <div><h4 class=\"elementor-heading-title elementor-size-default\">Messing</h4>\
        <h3 class=\"elementor-heading-title elementor-size-default\">3,97€/ kg</h3></div>\
        <div><h4 class=\"elementor-heading-title elementor-size-default\">Kupfer Raff</h4>\
        <h3 class=\"elementor-heading-title elementor-size-default\">6,49€/ kg</h3></div>\
        <h3 class=\"elementor-heading-title elementor-size-default\">Ankauf bei AMR:</h3>\
        <h3>Schnellerstr. 20E, 12439 Berlin</h3>";

    #[test]
    fn cards_parse_and_window_holds() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 4, "{rows:?}");
        assert!(skips.is_empty(), "{skips:?}");
        assert_eq!(rows[0], ("Kupfer Millberry".to_owned(), 7.0, "EUR/kg"));
        assert_eq!(rows[1], ("Kupfer Kabel".to_owned(), 2.19, "EUR/kg"));
        assert_eq!(rows[2], ("Messing".to_owned(), 3.97, "EUR/kg"));
        assert_eq!(rows[3], ("Kupfer Raff".to_owned(), 6.49, "EUR/kg"));
    }

    #[test]
    fn anchors_dangles_and_units_are_rejected_loudly() {
        // Missing window anchors: loud error, not silent success.
        assert!(parse("<html><body><p>Neu hier</p></body></html>").is_err());
        let no_end = FIXTURE.replace("Ankauf bei AMR:", "Ankauf X");
        assert!(parse(&no_end).is_err());
        // Dangling label without price skips loudly, valid cards survive.
        let html = FIXTURE.replacen("2,19€/ kg", "auf Anfrage", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Kupfer Kabel"), "{skips:?}");
        // Unknown unit: skipped loudly.
        let html = FIXTURE.replacen("€/ kg", "pro Sack", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 3);
        assert!(skips[0].contains("Einheit unverständlich"), "{skips:?}");
        // Every card unparseable: loud error.
        let html = FIXTURE.replace("€/ kg", "pro Sack");
        assert!(parse(&html).is_err());
        assert_eq!(unit_of("7,00€/ kg"), Some("EUR/kg"));
        assert_eq!(unit_of("pro Sack"), None);
    }

    #[test]
    fn mapping_covers_live_labels() {
        assert_eq!(
            grade_for("Kupfer Millberry"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(grade_for("Kupfer Kabel"), Some(("kabel-kupfer", "")));
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Kupfer Raff"), Some(("kupfer-gemischt", "")));
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h2>Impressum</h2><p>AMR Metalle und Rohstoffe GmbH<br>\
            Schnellerstr. 20E<br>12439 Berlin<br>Tel: 030 53 013 220<br>Fax: 030 53 013 221<br>\
            <a href=\"mailto:kontakt@amr-schrottplatz.de\">kontakt@amr-schrottplatz.de</a></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Schnellerstr. 20E");
        assert_eq!(info.postcode, "12439");
        assert_eq!(info.city, "Berlin");
        assert_eq!(info.phone, "030 53 013 220");
        assert_eq!(info.email, "kontakt@amr-schrottplatz.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(
            extract_info("<h2>Impressum</h2><p>Fremde Firma<br>Woanders 1<br>00000 Weitweg</p>")
                .is_err(),
            "missing firm anchor errors"
        );
    }
}
