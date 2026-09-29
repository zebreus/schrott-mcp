//! Böhner Altmetalle GmbH (Düsseldorf, Königsberger Str. 234a): exact
//! daily prices in the static Mengenstaffel table `table#preisliste`
//! (never the first table — selected by id and header content). Each
//! priced row carries three quantity tiers — Kleinmenge bis 50 kg /
//! Standard ab 50 kg / Container ab 500 kg — so one sort yields three
//! rows and the tier rides in the variant ("bis 50 kg", "ab 50 kg",
//! "ab 500 kg"). Group headers (Aluminium/Edelstahl/Kupfer/Stahlschrott
//! with empty price cells) carry no price and are skipped as structure,
//! never as labels. Live 27.09.2026: 17 priced rows × 3 tiers = 51
//! points, €/kg throughout except the three iron rows (€/t). No price
//! date on the page (only a closure notice) → `published_at` stays
//! `None` (`observed_at` = age).

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-dusseldorf-bohner-altmetalle";
/// Bespoke, live-verified impressum URL (site nav link). A move fails
/// the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://boehner-altmetalle.de/impressum-agb";

pub const URL: &str = "https://boehner-altmetalle.de/ankauf-preise";

/// Quantity tiers in column order (`td.plc-2/3/4`), the trader's own
/// header words. Every tier becomes a variant so sorts never collapse
/// onto one current price.
const TIERS: [&str; 3] = ["bis 50 kg", "ab 50 kg", "ab 500 kg"];

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
    for (label, tier, price, unit) in rows {
        match grade_for(&label, tier) {
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

/// Explicit (sort, tier) → (material, variant) mapping. Arms are ordered
/// specific-before-generic: "Bleche alt" before "Blei" ("ble…"), "Rotguss"
/// before "Guss", "Mischschrott schwer" before "Mischschrott",
/// "Kupferkabel" before any bare "Kupfer". The tier is the variant; the
/// three colliding sorts additionally carry their sort word
/// ("Bleche alt, …", "Offset, …", "schwer/Abbruch, …") so two sorts on
/// one material never collapse. Anything unlisted skips loudly.
fn grade_for(label: &str, tier: &'static str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("ausbauprofile") {
        Some(("aluminium-profile", tier))
    } else if l.contains("bleche alt") {
        Some(("aluminium-blech", prefixed("Bleche alt", tier)))
    } else if l.contains("felgen") {
        Some(("aluminium-gemischt", prefixed("Felgen", tier)))
    } else if l.contains("offset") {
        Some(("aluminium-blech", prefixed("Offset", tier)))
    } else if l.contains("blei") {
        Some(("blei", tier))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", tier))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", tier))
    } else if l.contains("millberry") {
        Some(("kupfer-millberry", tier))
    } else if l.contains("kerze") {
        Some(("kupfer-berry", tier))
    } else if l.contains("raff") {
        Some(("kupfer-gemischt", tier))
    } else if l.contains("kupferkabel") {
        Some(("kabel-kupfer", tier))
    } else if l.contains("messing") {
        Some(("messing", tier))
    } else if l.contains("rotguss") {
        Some(("bronze-rotguss", tier))
    } else if l.contains("mischschrott schwer") {
        Some(("mischschrott", prefixed("schwer/Abbruch", tier)))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", tier))
    } else if l.contains("guss") {
        Some(("eisenschrott-gussbruch", tier))
    } else if l.contains("zink") {
        Some(("zink", tier))
    } else {
        None
    }
}

/// Sort-word-prefixed tier variant for sorts sharing one material.
/// Exhaustive over the three prefixed sorts × three tiers (tiers are
/// constants from `parse`, so the fallback is unreachable by
/// construction — but loud, never a guessed string).
fn prefixed(sort: &str, tier: &'static str) -> &'static str {
    match (sort, tier) {
        ("Bleche alt", "bis 50 kg") => "Bleche alt, bis 50 kg",
        ("Bleche alt", "ab 50 kg") => "Bleche alt, ab 50 kg",
        ("Bleche alt", "ab 500 kg") => "Bleche alt, ab 500 kg",
        ("Felgen", "bis 50 kg") => "Felgen, bis 50 kg",
        ("Felgen", "ab 50 kg") => "Felgen, ab 50 kg",
        ("Felgen", "ab 500 kg") => "Felgen, ab 500 kg",
        ("Offset", "bis 50 kg") => "Offset, bis 50 kg",
        ("Offset", "ab 50 kg") => "Offset, ab 50 kg",
        ("Offset", "ab 500 kg") => "Offset, ab 500 kg",
        ("schwer/Abbruch", "bis 50 kg") => "schwer/Abbruch, bis 50 kg",
        ("schwer/Abbruch", "ab 50 kg") => "schwer/Abbruch, ab 50 kg",
        ("schwer/Abbruch", "ab 500 kg") => "schwer/Abbruch, ab 500 kg",
        _ => tier,
    }
}

/// Parse the `table#preisliste` window (header-content selected, never
/// the first table). Returns rows of (sort label, tier, price, unit)
/// plus loud skips. Group headers with empty price cells are structure,
/// not labels — skipped silently. 0 priced rows = `Err`.
fn parse(
    html: &str,
) -> Result<(Vec<(String, &'static str, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html
        .find("<table id=\"preisliste\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("</table>").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisliste unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    // Tier headers must read exactly as verified live — a relabeled
    // column would silently mistier every row.
    if !(window.contains(">Sorte<") && window.contains("bis 50 kg") && window.contains("ab 500 kg"))
    {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preislisten-Kopf unbekannt".to_owned(),
        });
    }
    let frag = Html::parse_fragment(&format!("{window}</table>"));
    let row_sel = Selector::parse("tr").expect("valid selector");
    let cell_sel = Selector::parse("td").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in frag.select(&row_sel) {
        let cells: Vec<String> = tr
            .select(&cell_sel)
            .map(|c| {
                c.text()
                    .collect::<String>()
                    .replace(['\u{a0}'], " ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();
        if cells.is_empty() {
            continue;
        }
        let label = cells[0].clone();
        if label.is_empty() || label.len() > 120 {
            continue;
        }
        // Price columns plc-2/3/4 in tier order; rows without any price
        // are group headers (Aluminium/Edelstahl/…), not labels.
        if cells.len() < 4 || cells[1..4].iter().all(|c| c.is_empty()) {
            continue;
        }
        for (cell, tier) in cells[1..4].iter().zip(TIERS) {
            if cell.is_empty() {
                continue;
            }
            let Some(price) = parse_eur(cell) else {
                skips.push(format!("{label} ({tier}: Preis unverständlich: {cell})"));
                continue;
            };
            // A "0,00" cell is "no quote", not a free gift: loud skip.
            if price == 0.0 {
                skips.push(format!("{label} ({tier}: Preis 0,00)"));
                continue;
            };
            // An unparseable unit is a loud skip, never a silent
            // default: a per-tonne price recorded as per-kg would be a
            // 1000x error.
            let Some(unit) = unit_of(cell) else {
                skips.push(format!("{label} ({tier}: Einheit unverständlich: {cell})"));
                continue;
            };
            rows.push((label.clone(), tier, price, unit));
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS table's price cells (live: "1,50 €/kg",
/// iron rows "100,00 €/t"). Only kg/t exist here — anything else skips
/// loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
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

/// Bespoke contact extraction for THIS impressum only: anchored on the
/// `h1` ("Impressum der Böhner Altmetalle GmbH") and the "Herausgeber der
/// Website:" `h2` — without both the page changed shape → loud error.
/// The following `<p>` holds firm + street + PLZ city over `<br />`;
/// the "Kontakt:" `<p>` holds "Telefon:" (page uses an en-dash:
/// "0211 – 213356") and "E-Mail:" lines. E-mail needs its own rule
/// (a phone-token filter stops at the first letter).
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    if !doc.select(&h1).any(|h| {
        h.text()
            .collect::<String>()
            .contains("Impressum der Böhner")
    }) {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let h2 = Selector::parse("h2").expect("valid selector");
    let anchor = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Herausgeber der Website:");
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Herausgeber-Block fehlt".to_owned(),
        });
    };
    let addr_p = anchor
        .next_siblings()
        .filter_map(scraper::ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let p_sel = Selector::parse("p").expect("valid selector");
    let contact_p = doc
        .select(&p_sel)
        .find(|el| el.inner_html().contains("Kontakt:"));
    let (Some(addr_p), Some(contact_p)) = (addr_p, contact_p) else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-/Kontakt-Block fehlt".to_owned(),
        });
    };
    let addr_lines: Vec<String> = addr_p
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in addr_lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| format!("{a} {b}"));
                if k > 0 {
                    street = addr_lines[k - 1].clone();
                }
                break;
            }
        }
    }
    let mut phone = String::new();
    let mut email = String::new();
    for part in contact_p.inner_html().split("<br") {
        let t = strip_fragment(part);
        if t.contains("Kontakt:") && phone.is_empty() && email.is_empty() {
            continue;
        }
        if let Some(v) = t.strip_prefix("Telefon:") {
            phone = v.trim().to_owned();
        } else if let Some(v) = t.strip_prefix("Telefax:") {
            let _ = v;
        } else if let Some(v) = t.strip_prefix("E-Mail:") {
            email = v.trim().to_owned();
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

/// Strip tags from a `<br>`-split fragment. Fragments start with a tag
/// remnant — drop everything up to the first '>' first, or attributes
/// parse as text.
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
    use super::{grade_for, parse, prefixed, unit_of};

    // Real shape of the live table (id, classes, sub/bem spans, unit
    // spans, €/t iron rows), trimmed to a group header + four sorts.
    const FIXTURE: &str = "<table id=\"preisliste\"><th class=\"plhc-1\">Sorte</th>\
        <th class=\"plhc-2\"><b>Kleinmenge</b><br>bis 50 kg</th>\
        <th class=\"plhc-3\"><b>Standard</b><br>ab 50 kg</th>\
        <th class=\"plhc-4\"><b>Container</b><br>ab 500 kg</th></th>\
        <tr><td class=\"plc-1\"><a href=\"/info#Kupfer\"><span class=\"head\">Kupfer<span class=\"bem\"> </span></span></a></td>\
        <td></td><td></td><td></td></tr>\
        <tr><td class=\"plc-1\"><a href=\"/info#Kupfer\"><span class=\"sub\">Millberry<span class=\"bem\"> </span></span></a></td>\
        <td class=\"plc-2\">10,50<span class=\"unit\"> €/kg</span></td>\
        <td class=\"plc-3\">10,75<span class=\"unit\"> €/kg</span></td>\
        <td class=\"plc-4\">11,00<span class=\"unit\"> €/kg</span></td></tr>\
        <tr><td class=\"plc-1\"><a href=\"/info#Alu\"><span class=\"sub\">Bleche alt<span class=\"bem\"> mit max. 2% FE</span></span></a></td>\
        <td class=\"plc-2\">1,25<span class=\"unit\"> €/kg</span></td>\
        <td class=\"plc-3\">1,35<span class=\"unit\"> €/kg</span></td>\
        <td class=\"plc-4\">1,45<span class=\"unit\"> €/kg</span></td></tr>\
        <tr><td class=\"plc-1\"><span class=\"sub\">Mischschrott schwer / Abbruch</span></td>\
        <td class=\"plc-2\">100,00<span class=\"unit\"> €/t</span></td>\
        <td class=\"plc-3\">140,00<span class=\"unit\"> €/t</span></td>\
        <td class=\"plc-4\">170,00<span class=\"unit\"> €/t</span></td></tr>\
        <tr><td class=\"plc-1\"><span class=\"sub\">V4A</span></td>\
        <td class=\"plc-2\">1,20<span class=\"unit\"> €/kg</span></td>\
        <td class=\"plc-3\">1,30<span class=\"unit\"> €/kg</span></td>\
        <td class=\"plc-4\">1,40<span class=\"unit\"> €/kg</span></td></tr>\
        </table>";

    #[test]
    fn tiers_units_and_header_check() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // Group header yields no rows; four sorts × three tiers.
        assert_eq!(rows.len(), 12);
        assert!(skips.is_empty());
        assert_eq!(
            rows[0],
            ("Millberry".to_owned(), "bis 50 kg", 10.5, "EUR/kg")
        );
        assert_eq!(
            rows[2],
            ("Millberry".to_owned(), "ab 500 kg", 11.0, "EUR/kg")
        );
        assert_eq!(
            rows[3],
            (
                "Bleche alt mit max. 2% FE".to_owned(),
                "bis 50 kg",
                1.25,
                "EUR/kg"
            )
        );
        assert_eq!(
            rows[6],
            (
                "Mischschrott schwer / Abbruch".to_owned(),
                "bis 50 kg",
                100.0,
                "EUR/t"
            )
        );
        assert_eq!(unit_of("1,50 €/kg"), Some("EUR/kg"));
        assert_eq!(unit_of("100,00 €/t"), Some("EUR/t"));
        assert_eq!(unit_of("pro Sack"), None);
        // Unknown header or missing table fails loudly.
        assert!(parse("<table id=\"preisliste\"><th>Sonst was</th></table>").is_err());
        assert!(parse("<div>Redesign ohne Tabelle</div>").is_err());
    }

    #[test]
    fn zero_price_cells_skip_loudly() {
        // One tier quoted "0,00" is "no quote", never a price-0 row.
        let html = FIXTURE.replacen("10,50<span", "0,00<span", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 11);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Millberry") && skips[0].contains("0,00"));
        assert!(rows.iter().all(|(_, _, p, _)| *p > 0.0));
    }

    #[test]
    fn mapping_keeps_sorts_apart() {
        assert_eq!(
            grade_for("Millberry", "bis 50 kg"),
            Some(("kupfer-millberry", "bis 50 kg"))
        );
        assert_eq!(
            grade_for("Bleche alt mit max. 2% FE", "ab 50 kg"),
            Some(("aluminium-blech", "Bleche alt, ab 50 kg"))
        );
        assert_eq!(
            grade_for("Offset", "ab 500 kg"),
            Some(("aluminium-blech", "Offset, ab 500 kg"))
        );
        assert_eq!(
            grade_for("Felgen", "bis 50 kg"),
            Some(("aluminium-gemischt", "Felgen, bis 50 kg"))
        );
        assert_eq!(grade_for("Blei", "bis 50 kg"), Some(("blei", "bis 50 kg")));
        assert_eq!(
            grade_for("V2A", "ab 50 kg"),
            Some(("edelstahl-v2a", "ab 50 kg"))
        );
        assert_eq!(
            grade_for("V4A", "ab 50 kg"),
            Some(("edelstahl-v4a", "ab 50 kg"))
        );
        assert_eq!(
            grade_for("Mischschrott", "ab 500 kg"),
            Some(("mischschrott", "ab 500 kg"))
        );
        assert_eq!(
            grade_for("Mischschrott schwer / Abbruch", "ab 500 kg"),
            Some(("mischschrott", "schwer/Abbruch, ab 500 kg"))
        );
        assert_eq!(
            grade_for("Rotguss", "bis 50 kg"),
            Some(("bronze-rotguss", "bis 50 kg"))
        );
        assert_eq!(
            grade_for("Kupferkabel", "bis 50 kg"),
            Some(("kabel-kupfer", "bis 50 kg"))
        );
        assert_eq!(prefixed("Offset", "ab 50 kg"), "Offset, ab 50 kg");
        assert_eq!(grade_for("E-Motoren", "bis 50 kg"), None);
    }

    #[test]
    fn impressum_blocks() {
        let imp = "<h1>Impressum der Böhner Altmetalle GmbH</h1>\
            <h2>Herausgeber der Website:</h2>\
            <p>Böhner Altmetalle GmbH<br />Königsberger Str. 234a<br />40231 Düsseldorf</p>\
            <p><b>Kontakt:</b><br />Telefon: 0211 – 213356<br />Telefax: 0211 – 219881<br />\
            E-Mail: info@boehner-altmetalle.de</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Königsberger Str. 234a");
        assert_eq!(info.postcode, "40231");
        assert_eq!(info.city, "Düsseldorf");
        assert_eq!(info.phone, "0211 – 213356");
        assert_eq!(info.email, "info@boehner-altmetalle.de");
        assert!(super::extract_info("<h1>Sonst was</h1>").is_err());
        assert!(super::extract_info("<h1>Impressum der Böhner Altmetalle GmbH</h1>").is_err());
    }
}
