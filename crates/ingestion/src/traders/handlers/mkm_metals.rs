//! MKM Metals (Ludwigshafen): daily purchase prices for the Kupfer group
//! on the /metallpreise overview (live "13 Sorten · bis 11,24 €/kg").
//! Window = the `id="kupfer"` section up to `id="messing"`, table picked
//! by its caption "Ankaufpreise für Kupfer" (columns Sorte /
//! Artikelnummer / Höchstpreis / Geändert / Menge).
//!
//! "Höchstpreis" + the "bis … €/kg" group header are up-to prices →
//! `price = price_max` = advertised value, confidence 0.5, kind "upto"
//! (kupferhelden pattern). Every row quotes "/kg" plus a uniform "ab
//! 250 kg" minimum quantity (documented here — no per-row field carries
//! it). Page date: "Preisliste · Stand 27.09.2026" → `published_at`.
//!
//! Scope is the Kupfer table only: Messing/Rotguss, Aluminium, Edelstahl,
//! NE-Metalle, Blei/Zink/E-Motoren, Hartmetall and Schrott groups share
//! the shape but are separate sections — a later step can extend the
//! window per group. Kupfergranulat and Cu/Ms-Kühler have no catalog
//! material and skip loudly.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "rp-ludwigshafen-mkm-metals";
/// Bespoke, live-verified impressum URL (site footer's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "https://mkm-metals.de/impressum";

/// Price overview; the Kupfer section is parsed, other metal groups are
/// out of scope for this handler (see module docs).
pub const URL: &str = "https://mkm-metals.de/metallpreise";

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
    let (published_at, rows, mut skipped_labels) = parse(&html)?;
    // Höchstpreise are upper bounds: price = price_max = advertised.
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit,
                price_kind: "upto",
                price_min: None,
                price_max: Some(price),
                confidence: Some(0.5),
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

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific-before-generic: "Kupfer Kerze" must hit berry
/// before any copper fallback, "Cu/Ms Kühler" must die on the kühler
/// arm before cable/copper arms, and cable percentages stay apart via
/// distinct variants (70% vs 60% vs 38-40% vs Stecker 30%).
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kerze") {
        Some(("kupfer-berry", "Kerze"))
    } else if l.contains("berry") {
        Some(("kupfer-berry", "Berry 98%"))
    } else if l.contains("granulat") {
        // Kupfergranulat ohne Katalogmaterial (Vorschlag: kupfer-granulat).
        None
    } else if l.contains("kühler") || l.contains("kuehler") {
        // Cu/Ms-Mischprodukt ohne Katalogmaterial (Vorschlag: kühler).
        None
    } else if l.contains("sammelschienen") {
        Some(("kupfer-gemischt", "Sammelschienen"))
    } else if l.contains("schwer") {
        Some(("kupfer-gemischt", "schwer"))
    } else if l.contains("raff") {
        Some(("kupfer-gemischt", "Raff"))
    } else if l.contains("leicht") {
        Some(("kupfer-gemischt", "leicht"))
    } else if l.contains("kabel") || l.contains("steck") {
        if l.contains("70%") {
            Some(("kabel-kupfer", "70%"))
        } else if l.contains("60%") {
            Some(("kabel-kupfer", "60%"))
        } else if l.contains("38") {
            Some(("kabel-kupfer", "38-40%"))
        } else if l.contains("steck") {
            Some(("kabel-mit-stecker", "Stecker 30%"))
        } else {
            None
        }
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after
/// "Angaben gemäß § 5 DDG" carries firm + "Inhaber: …" + street + PLZ
/// city inline (no `<br>`), and the `<p>` after "Kontakt" holds
/// "Telefon:" / "E-Mail:" lines (address also via mailto-href). Missing
/// anchors mean the page changed shape → loud error, never a fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let a = Selector::parse("a").expect("valid selector");
    let anchor = |title: &str| {
        doc.select(&h2)
            .find(|h| h.text().collect::<String>().trim() == title)
    };
    let Some(addr_h) = anchor("Angaben gemäß § 5 DDG") else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    let addr_p = addr_h
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    // Inline address: strip tags (source already spaces tokens — no
    // glued neighbours here) then read street/PLZ/city off the tokens.
    let addr_text = addr_p
        .map(|p| strip_tags(&p.inner_html()))
        .unwrap_or_default();
    let toks: Vec<&str> = addr_text.split_whitespace().collect();
    let mut street = String::new();
    for (k, t) in toks.iter().enumerate() {
        if *t == "Industriestraße" {
            if let Some(n) = toks.get(k + 1) {
                street = format!("Industriestraße {n}");
                break;
            }
        }
    }
    let (mut postcode, mut city) = (String::new(), String::new());
    for (k, t) in toks.iter().enumerate() {
        if t.len() == 5 && t.chars().all(|c| c.is_ascii_digit()) {
            if let Some(ci) = toks.get(k + 1) {
                if ci.chars().next().is_some_and(|c| c.is_uppercase()) {
                    postcode = (*t).to_owned();
                    city = (*ci).to_owned();
                    break;
                }
            }
        }
    }
    let Some(kontakt_h) = anchor("Kontakt") else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let mut phone = String::new();
    let mut email = String::new();
    if let Some(p) = kontakt_h
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p")
    {
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
                if phone.is_empty() {
                    phone = v.trim().to_owned();
                }
            } else if email.is_empty() {
                if let Some(v) = t.strip_prefix("E-Mail:") {
                    email = v.split_whitespace().next().unwrap_or_default().to_owned();
                }
            }
        }
    }
    if street.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    }
    if phone.is_empty() && email.is_empty() {
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

/// Strip tags from a fragment (html5ever already decoded entities).
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

/// Strip tags from a `<br>`-split fragment. Only fragments that START
/// with a tag remnant drop everything up to the first '>' — a fragment
/// starting with text ("Telefon: <a …>…") keeps its label, or the
/// prefix would be cut off with the tag.
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
    strip_tags(s)
}

/// Bespoke page-date reader: "Preisliste · Stand 27.09.2026" at the top
/// of the page (one date per page — no per-material dates exist). Later
/// "Stand …" decoys (JSON-LD FAQ: "mit dem Stand des jeweiligen Tages")
/// parse to no date and are skipped — the first parseable dd.mm.yyyy
/// after a "Stand " wins.
fn stand_date(html: &str) -> Option<String> {
    let mut rest = html;
    while let Some(i) = rest.find("Stand ") {
        rest = &rest[i + "Stand ".len()..];
        let mut parts = rest.trim_start().split('.');
        if let (Some(d), Some(m), Some(y)) = (parts.next(), parts.next(), parts.next()) {
            let year: String = y.chars().take_while(|c| c.is_ascii_digit()).collect();
            if year.len() == 4 {
                if let Some(date) = parse_de_date(d.trim(), m.trim(), &year) {
                    return Some(date);
                }
            }
        }
    }
    None
}

fn parse(
    html: &str,
) -> Result<
    (
        Option<String>,
        Vec<(String, f64, &'static str)>,
        Vec<String>,
    ),
    IngestError,
> {
    // Window: the Kupfer section only — from its opening tag to the
    // Messing section. Other metal groups are out of scope; a whole-page
    // walk would silently absorb their sorts into copper mapping.
    let kupfer = html
        .find("id=\"kupfer\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Kupfer-Block fehlt".to_owned(),
        })?;
    let sec_start = html[..kupfer].rfind('<').unwrap_or(0);
    let after = &html[kupfer..];
    let messing = after
        .find("id=\"messing\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Kupfer-Block offen".to_owned(),
        })?;
    let sec_end_rel = after[..messing].rfind('<').unwrap_or(messing);
    let window = &html[sec_start..kupfer + sec_end_rel];
    // Head-content check (never the first table): the copper price
    // caption must be in the window, or the page changed shape.
    if !window.contains("Ankaufpreise für Kupfer") {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Kupfer-Preistabelle fehlt".to_owned(),
        });
    }
    let doc = Html::parse_fragment(window);
    let row = Selector::parse("tbody tr").expect("valid selector");
    let th_link = Selector::parse("th a").expect("valid selector");
    let price_cell = Selector::parse("td.num").expect("valid selector");
    let strong = Selector::parse("strong").expect("valid selector");
    let mut rows = Vec::new();
    let mut unit_skips = Vec::new();
    for tr in doc.select(&row) {
        let Some(link) = tr.select(&th_link).next() else {
            continue;
        };
        let label: String = link
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if label.is_empty() {
            continue;
        }
        // Two td.num cells per row (Höchstpreis + Geändert): the price
        // cell is the one quoting "/kg".
        let cell = tr
            .select(&price_cell)
            .find(|c| c.text().collect::<String>().contains("/kg"));
        let Some(cell) = cell else {
            unit_skips.push(format!("{label} (Einheit unverständlich)"));
            continue;
        };
        let cell_text: String = cell.text().collect();
        let num_src: String = cell
            .select(&strong)
            .next()
            .map(|s| s.text().collect())
            .unwrap_or_else(|| cell_text.clone());
        // parse_eur reads the FIRST number — the "11,24" up front, never
        // the "250" of the "ab 250 kg" minimum quantity behind it.
        let Some(price) = parse_eur(&num_src) else {
            continue;
        };
        // An unparseable unit is a loud skip, never a silent default.
        let Some(unit) = unit_of(&cell_text) else {
            unit_skips.push(format!(
                "{label} (Einheit unverständlich: {})",
                cell_text.trim()
            ));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Kupfer-Tabelle leer".to_owned(),
        });
    }
    Ok((stand_date(html), rows, unit_skips))
}

/// Bespoke unit matcher for THIS table's Höchstpreis cells (live: "/kg"
/// on every copper row). Only kg exists here — anything else skips
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
    use super::{grade_for, parse, stand_date};

    // Real live markup of the Kupfer table (caption + 4 of 13 rows: the
    // mapping-critical shapes Millberry/Kerze/Kabel-70/Kühler).
    const FIXTURE: &str = "<section id=\"kupfer\"><header><h2>Kupfer</h2>\
        <p>13 Sorten · bis <strong>11,24 €/kg</strong></p></header>\
        <div class=\"table-wrap\"><table class=\"table\">\
        <caption class=\"visually-hidden\">Ankaufpreise für Kupfer</caption>\
        <thead><tr><th scope=\"col\">Sorte</th><th scope=\"col\">Artikelnummer</th>\
        <th scope=\"col\" class=\"num\">Höchstpreis</th><th scope=\"col\" class=\"num\">Geändert</th>\
        <th scope=\"col\">Menge</th></tr></thead><tbody>\
        <tr><th scope=\"row\"><a href=\"/metallankauf/produkt/kupfer-millberry\">Kupfer Millberry</a></th>\
        <td class=\"muted small\">MET-KU-MILL</td>\
        <td class=\"num\"><strong>11,24 €</strong><span class=\"muted small\">/kg</span>\
        <span class=\"muted small\">ab 250 kg</span></td>\
        <td class=\"num muted small\"><time datetime=\"2026-09-26\">gestern</time></td><td></td></tr>\
        <tr><th scope=\"row\"><a href=\"/metallankauf/produkt/kupfer-kerze\">Kupfer Kerze</a></th>\
        <td class=\"muted small\">MKM-CU-01</td>\
        <td class=\"num\"><strong>11,03 €</strong><span class=\"muted small\">/kg</span>\
        <span class=\"muted small\">ab 250 kg</span></td>\
        <td class=\"num muted small\"><time datetime=\"2026-09-26\">gestern</time></td><td></td></tr>\
        <tr><th scope=\"row\"><a href=\"/metallankauf/produkt/kupferkabel-ca-70\">Kupferkabel ca. 70%</a></th>\
        <td class=\"muted small\">MET-KA-70</td>\
        <td class=\"num\"><strong>5,99 €</strong><span class=\"muted small\">/kg</span>\
        <span class=\"muted small\">ab 250 kg</span></td>\
        <td class=\"num muted small\">vor zwei Wochen</td><td></td></tr>\
        <tr><th scope=\"row\"><a href=\"/metallankauf/produkt/cu-ms-kuehler-ohne-fe\">Cu/Ms Kühler ohne Fe</a></th>\
        <td class=\"muted small\">SW10017</td>\
        <td class=\"num\"><strong>5,10 €</strong><span class=\"muted small\">/kg</span>\
        <span class=\"muted small\">ab 250 kg</span></td>\
        <td class=\"num muted small\">am 27.07.2026</td><td></td></tr>\
        </tbody></table></div></section><section id=\"messing\">";

    #[test]
    fn copper_table_and_stand_date_parse() {
        let html = "<p>Preisliste · Stand 27.09.2026</p>".to_owned() + FIXTURE;
        let (published_at, rows, skips) = parse(&html).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-27T00:00:00+00:00"));
        assert_eq!(rows.len(), 4);
        assert!(skips.is_empty());
        assert_eq!(rows[0].0, "Kupfer Millberry");
        assert_eq!(rows[0].1, 11.24);
        assert_eq!(rows[0].2, "EUR/kg");
        // The 250 of "ab 250 kg" must never win over the Höchstpreis.
        assert_eq!(rows[2].0, "Kupferkabel ca. 70%");
        assert_eq!(rows[2].1, 5.99);
    }

    #[test]
    fn stand_date_reader() {
        assert_eq!(
            stand_date("Preisliste · Stand 27.09.2026"),
            Some("2026-09-27T00:00:00+00:00".to_owned())
        );
        // JSON-LD FAQ decoy first ("mit dem Stand des jeweiligen Tages"):
        // skipped, the real Stand date behind it still wins.
        assert_eq!(
            stand_date("mit dem Stand des jeweiligen Tages. Preisliste · Stand 27.09.2026"),
            Some("2026-09-27T00:00:00+00:00".to_owned())
        );
        assert_eq!(stand_date("Preisliste ohne Datum"), None);
    }

    #[test]
    fn wrong_section_and_unit_are_rejected_loudly() {
        // Messing table before the copper section must not win.
        let html = FIXTURE.replace("id=\"kupfer\"", "id=\"edelstahl\"");
        assert!(parse(&html).is_err());
        // Unknown unit: skipped loudly, valid rows survive.
        let html = FIXTURE.replacen("/kg</span>", "/Stk</span>", 1);
        let (_, rows, skips) = parse(&("Stand 27.09.2026".to_owned() + &html)).expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Millberry"));
    }

    #[test]
    fn mapping_separates_grades_and_skips_gaps() {
        assert_eq!(
            grade_for("Kupfer Millberry"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(grade_for("Kupfer Kerze"), Some(("kupfer-berry", "Kerze")));
        assert_eq!(
            grade_for("Cu Berry / 98%"),
            Some(("kupfer-berry", "Berry 98%"))
        );
        assert_eq!(
            grade_for("Kupfer Sammelschienen"),
            Some(("kupfer-gemischt", "Sammelschienen"))
        );
        assert_eq!(
            grade_for("Cu schwer Basis 95% Kupfer-Inhalt"),
            Some(("kupfer-gemischt", "schwer"))
        );
        assert_eq!(grade_for("Kupfer Raff"), Some(("kupfer-gemischt", "Raff")));
        assert_eq!(
            grade_for("Kupfer leicht"),
            Some(("kupfer-gemischt", "leicht"))
        );
        assert_eq!(
            grade_for("Kupferkabel ca. 70%"),
            Some(("kabel-kupfer", "70%"))
        );
        assert_eq!(
            grade_for("Kupferkabel ca. 60%"),
            Some(("kabel-kupfer", "60%"))
        );
        assert_eq!(
            grade_for("Cu Kabel gemischt (38-40%Cu)"),
            Some(("kabel-kupfer", "38-40%"))
        );
        assert_eq!(
            grade_for("Cu Steckerkabel (30%Cu)"),
            Some(("kabel-mit-stecker", "Stecker 30%"))
        );
        assert_eq!(grade_for("Kupfergranulat"), None, "kein Granulat-Material");
        assert_eq!(
            grade_for("Cu/Ms Kühler ohne Fe"),
            None,
            "kein Kühler-Material"
        );
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h2 id=\"angaben-gemaess-5-ddg\">Angaben gemäß § 5 DDG</h2>\
            <p><strong>MKM Metals e.K.</strong> Inhaber: Mubarik Ahmad Industriestraße 4 67063 Ludwigshafen am Rhein Deutschland</p>\
            <h2 id=\"kontakt\">Kontakt</h2>\
            <p>Telefon: <a href=\"tel:+4917622016840\">0176 220 168 40</a><br>\
            E-Mail: <a href=\"mailto:contact@mkm-metals.de\">contact@mkm-metals.de</a><br></p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Industriestraße 4");
        assert_eq!(info.postcode, "67063");
        assert_eq!(info.city, "Ludwigshafen");
        assert_eq!(info.phone, "0176 220 168 40");
        assert_eq!(info.email, "contact@mkm-metals.de");
        // Redesign without anchors fails loudly.
        assert!(super::extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
