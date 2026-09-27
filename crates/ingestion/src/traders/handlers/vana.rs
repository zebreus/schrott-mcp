//! VANA Schrotthandel & Entrümpelung (Freudenstadt): daily indicative
//! purchase ranges in the homepage "Tagespreise für Altmetall" board
//! (`<dl class="vana-prices">` with dt/dd pairs like "Kupfer" → "8 bis
//! 8,50 € / kg"). The page itself hedges ("Preise schwanken täglich …
//! Der Endpreis richtet sich nach Sortenreinheit und Gewicht"), so ranges
//! are recorded as `price_kind: "range"` at confidence 0.5 with both
//! bounds — never exact. No page date → `published_at` is None. Bare
//! "Kabel" rows cannot choose between `kabel-kupfer` and `kabel-alu` and
//! skip loudly; "auf Anfrage" rows carry no price and skip loudly too.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bw-freudenstadt-72250-vana-schrotthandel-entrumpelung";
/// Bespoke, live-verified impressum URL (site footer's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "https://vana-service.de/impressum/";

/// The Tagespreise board lives on the homepage (single-page site) —
/// this IS the price page here, not a fallback.
pub const URL: &str = "https://vana-service.de";

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
    for (label, min, max, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => {
                // Von-bis ranges mirror the upto pattern: the bounds are
                // honest data with their own kind, never a silent exact.
                let (price_kind, confidence) = if (min - max).abs() < f64::EPSILON {
                    ("exact", Some(1.0))
                } else {
                    ("range", Some(0.5))
                };
                let (price_min, price_max) = if price_kind == "range" {
                    (Some(min), Some(max))
                } else {
                    (None, None)
                };
                prices.push(ScrapedPrice {
                    material,
                    variant,
                    price: max,
                    currency: "EUR",
                    unit,
                    price_kind,
                    price_min,
                    price_max,
                    confidence,
                    label,
                });
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
/// skipped. "Kabel" / "Kabel mit Stärke" name no metal: copper and
/// aluminium cable price worlds apart (the page itself lists Erdkabel,
/// which exists in both) → loud skip, never a guessed `kabel-kupfer`.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("kabel") {
        None
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("aluminium") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("edelstahl") {
        Some(("edelstahl-gemischt", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after
/// the "Angaben gemäß § 5 DDG" heading holds firm lines + street + PLZ
/// city, and the `<p>` after the "Kontakt" heading holds "Telefon:" /
/// "E-Mail:" lines (mail address also via mailto-href). Missing
/// anchors mean the page changed shape → loud error, never a fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let a = Selector::parse("a").expect("valid selector");
    let heading = |title: &str| {
        doc.select(&h2)
            .find(|h| h.text().collect::<String>().trim() == title)
    };
    let Some(addr_h) = heading("Angaben gemäß § 5 DDG") else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    let addr_p = addr_h
        .next_siblings()
        .filter_map(scraper::ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let mut lines = Vec::new();
    if let Some(p) = addr_p {
        for part in p.inner_html().split("<br") {
            let t = strip_fragment(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
    // "Hermann-Hesse-Straße 16" / "72250 Freudenstadt" (last two lines).
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if lines.len() >= 2 {
        let last = lines.last().expect("len checked");
        let mut it = last.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = ci.to_owned();
                street = lines[lines.len() - 2].clone();
            }
        }
    }
    let Some(kontakt_h) = heading("Kontakt") else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let mut phone = String::new();
    let mut email = String::new();
    let kontakt_p = kontakt_h
        .next_siblings()
        .filter_map(scraper::ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    if let Some(p) = kontakt_p {
        // E-mail per mailto-href, never per token split (the phone-style
        // take_while would stop at the first letter).
        for el in p.select(&a) {
            if email.is_empty() {
                if let Some(href) = el.value().attr("href") {
                    if let Some(addr) = href.strip_prefix("mailto:") {
                        email = addr.trim().to_owned();
                    }
                }
            }
        }
        for part in p.inner_html().split("<br") {
            let t = strip_fragment(part);
            if let Some(v) = t.strip_prefix("Telefon:") {
                phone = v.trim().to_owned();
            } else if let Some(v) = t.strip_prefix("E-Mail:") {
                if email.is_empty() {
                    email = v.split_whitespace().next().unwrap_or_default().to_owned();
                }
            }
        }
    }
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    // The address lines are as much an anchor as the headings: without
    // them the page changed shape → loud error, never a guessed fallback.
    if street.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
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

/// Strip tags from a `<br>`-split fragment (html5ever already decoded
/// entities). Only fragments that START with a tag remnant (` />…` after
/// the `<br` split point) drop everything up to the first '>' — a
/// fragment starting with text ("Telefon: <a …>…") keeps its label,
/// or the "Telefon:" prefix would be cut off with the tag.
fn strip_fragment(s: &str) -> String {
    let s = s.trim_start();
    let s = if s.starts_with('<') {
        match s.find('>') {
            Some(i) => &s[i + 1..],
            None => s,
        }
    } else {
        s
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

/// Bespoke range splitter for THIS board's dd cells (live: "8 bis 8,50
/// € / kg", single prices like "1 € / kg"). Returns (min, max).
fn split_range(dd: &str) -> Option<(f64, f64)> {
    let t = dd.replace(['\u{a0}'], " ");
    if let Some((a, b)) = t.split_once("bis") {
        Some((parse_eur(a)?, parse_eur(b)?))
    } else {
        let p = parse_eur(&t)?;
        Some((p, p))
    }
}

fn parse(html: &str) -> Result<(Vec<(String, f64, f64, &'static str)>, Vec<String>), IngestError> {
    // Window: the price board only — never the whole page (service prose
    // and footer numbers must not pair with labels into phantom prices).
    let start = html
        .find("<dl class=\"vana-prices\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Tagespreis-Board fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("</dl>").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Tagespreis-Board offen".to_owned(),
    })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(window);
    let div = Selector::parse("div").expect("valid selector");
    let dt = Selector::parse("dt").expect("valid selector");
    let dd = Selector::parse("dd").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for box_el in doc.select(&div) {
        let Some(dt_el) = box_el.select(&dt).next() else {
            continue;
        };
        let Some(dd_el) = box_el.select(&dd).next() else {
            continue;
        };
        let label: String = dt_el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let dd_text: String = dd_el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if label.is_empty() {
            continue;
        }
        // "auf Anfrage" rows name no price — loud skip, never a zero row.
        let Some((min, max)) = split_range(&dd_text) else {
            skips.push(format!("{label} (auf Anfrage, kein Preis)"));
            continue;
        };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error. The
        // board note ("Angaben in Euro pro Kilogramm") documents kg as the
        // page default; explicitly foreign units still skip.
        let Some(unit) = unit_of(&dd_text) else {
            skips.push(format!("{label} (Einheit unverständlich: {dd_text})"));
            continue;
        };
        rows.push((label, min, max, unit));
    }
    if rows.is_empty() && skips.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisboard leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS board's dd cells (live: "/ kg" small
/// print in every row). Only kg exists here — anything else skips
/// loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    if cell.to_lowercase().contains("kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{grade_for, parse, split_range};

    // Real live markup of the board (dt label + dd range, full set
    // incl. the four "auf Anfrage" rows).
    const FIXTURE: &str = "<div><h2>Tagespreise für Altmetall</h2>\
        <p class=\"vana-board__note\">Angaben in Euro pro Kilogramm.</p></div>\
        <div><dl class=\"vana-prices\">\
        <div><dt>Kupfer</dt><span class=\"vana-dots\"></span><dd>8 bis 8,50 €<small>/ kg</small></dd></div>\
        <div><dt>Messing</dt><span class=\"vana-dots\"></span><dd>4 bis 4,50 €<small>/ kg</small></dd></div>\
        <div><dt>Kabel</dt><span class=\"vana-dots\"></span><dd>2,3 bis 2,5 €<small>/ kg</small></dd></div>\
        <div><dt>Kabel mit Stärke</dt><span class=\"vana-dots\"></span><dd>1 €<small>/ kg</small></dd></div>\
        <div><dt>Aluminium</dt><span class=\"vana-dots\"></span><dd>0,8 bis 1,4 €<small>/ kg</small></dd></div>\
        <div><dt>Edelstahl</dt><span class=\"vana-dots\"></span><dd>0,30 bis 0,40 €<small>/ kg</small></dd></div>\
        <div><dt>Alle anderen Schrottmetalle</dt><span class=\"vana-dots\"></span><dd>auf Anfrage<small>/ kg</small></dd></div>\
        <div><dt>Elektroschrott</dt><span class=\"vana-dots\"></span><dd>auf Anfrage<small>/ kg</small></dd></div>\
        <div><dt>Abholung</dt><span class=\"vana-dots\"></span><dd>auf Anfrage<small>/ kg</small></dd></div>\
        <div><dt>Entsorgung</dt><span class=\"vana-dots\"></span><dd>auf Anfrage<small>/ kg</small></dd></div>\
        </dl></div>";

    #[test]
    fn board_ranges_and_anfrage_skips() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 6);
        assert_eq!(skips.len(), 4);
        assert_eq!(rows[0].0, "Kupfer");
        assert_eq!((rows[0].1, rows[0].2), (8.0, 8.5));
        assert_eq!(rows[0].3, "EUR/kg");
        assert_eq!((rows[2].1, rows[2].2), (2.3, 2.5));
        assert_eq!((rows[3].1, rows[3].2), (1.0, 1.0));
        assert!(skips.iter().any(|s| s.contains("Elektroschrott")));
        assert!(skips.iter().all(|s| s.contains("auf Anfrage")));
    }

    #[test]
    fn range_splitter() {
        assert_eq!(split_range("8 bis 8,50 € / kg"), Some((8.0, 8.5)));
        assert_eq!(split_range("0,30 bis 0,40 € / kg"), Some((0.3, 0.4)));
        assert_eq!(split_range("1 € / kg"), Some((1.0, 1.0)));
        assert_eq!(split_range("auf Anfrage / kg"), None);
    }

    #[test]
    fn missing_board_and_unit_fail_loudly() {
        assert!(parse("<html><body><p>Neu hier</p></body></html>").is_err());
        let html = FIXTURE.replacen(
            "/ kg</small></dd></div><div><dt>Messing",
            "/ Sack</small></dd></div><div><dt>Messing",
            1,
        );
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 5);
        assert!(skips
            .iter()
            .any(|s| s.contains("Kupfer") && s.contains("Einheit")));
    }

    #[test]
    fn mapping_keeps_generics_and_skips_cable() {
        assert_eq!(grade_for("Kupfer"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Aluminium"), Some(("aluminium-gemischt", "")));
        assert_eq!(grade_for("Edelstahl"), Some(("edelstahl-gemischt", "")));
        // No metal named: copper vs aluminium cable is a different world.
        assert_eq!(grade_for("Kabel"), None);
        assert_eq!(grade_for("Kabel mit Stärke"), None);
        assert_eq!(grade_for("Alle anderen Schrottmetalle"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h1>Impressum</h1><h2>Angaben gemäß § 5 DDG</h2>\
            <p>Vasil Radev<br />VANA Services<br />Hermann-Hesse-Straße 16<br />72250 Freudenstadt</p>\
            <h2>Kontakt</h2><p>Telefon: <a href=\"tel:+491733078005\">+49 173 3078005</a><br />\
            E-Mail: <a href=\"mailto:info@vana-service.de\">info@vana-service.de</a></p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Hermann-Hesse-Straße 16");
        assert_eq!(info.postcode, "72250");
        assert_eq!(info.city, "Freudenstadt");
        assert_eq!(info.phone, "+49 173 3078005");
        assert_eq!(info.email, "info@vana-service.de");
        // Redesign without anchors fails loudly.
        assert!(super::extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(
            super::extract_info("<h2>Angaben gemäß § 5 DDG</h2><p>Ohne Adresszeilen</p>").is_err()
        );
    }
}
