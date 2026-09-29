//! Lausitz Recycling (Lauchhammer): label/price pairs scattered over
//! nested card tables, with the whole block repeated under "Gültig ab"
//! (partly different spellings: "Cu – schwer" vs "Cu –Berry",
//! "Cu-Kabel o. Stecker" vs "Cu – Kabel ohne Stecker").
//! No page date. Strategy: walk all text nodes, pair each price text
//! with the pending label, then dedupe identical pairs. "KEIN ANKAUF …"
//! and paper ("Altpapier" — no paper in the catalog) have no mappable
//! material and are skipped loudly. Exact 0.00 prices mean "no buy
//! price" and are skipped loudly too.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bb-lauchhammer-ost-lausitz-recycling";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.lausitz-recycling.de/impressum";

pub const URL: &str = "https://www.lausitz-recycling.de/";

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
    // The "Gültig ab" repeat uses slightly different spellings for the
    // same grades: collapse identical (material, price) pairs.
    let mut seen = std::collections::HashSet::new();
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => {
                if seen.insert((material, variant, price.to_bits())) {
                    prices.push(ScrapedPrice {
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

fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let norm: String = label
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ')
        .collect();
    let norm = norm.as_str();
    if norm.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if norm.contains("messing") {
        // BEFORE schwer/berry: "Schwermessing" starts with "schwer".
        Some(("messing", ""))
    } else if norm.contains("schwer") || norm.contains("berry") {
        // Same grade, two spellings across page sections ("Cu – schwer"
        // vs "Cu –Berry", same price): one variant keeps them together.
        Some(("kupfer-gemischt", "schwer"))
    } else if norm.contains("raff") {
        Some(("kupfer-gemischt", "Raff"))
    } else if norm.contains("kabel") {
        Some(("kabel-kupfer", ""))
    } else if norm.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: semantic
/// `data-bind="customer.*"` spans plus the `mailto:` link. Missing spans
/// mean the page changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let text_of = |sel: &str| {
        doc.select(&Selector::parse(sel).expect("valid selector"))
            .next()
            .map(|e| e.text().collect::<String>())
            .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
    };
    let street = text_of(r#"span[data-bind="customer.street"]"#).unwrap_or_default();
    let postcode = text_of(r#"span[data-bind="customer.zip"]"#).unwrap_or_default();
    let city = text_of(r#"span[data-bind="customer.city"]"#).unwrap_or_default();
    let phone = text_of(r#"span[data-bind="customer.phone"]"#).unwrap_or_default();
    let email = doc
        .select(&Selector::parse(r#"a[href^="mailto:"]"#).expect("valid selector"))
        .next()
        .map(|e| {
            e.value()
                .attr("href")
                .unwrap_or_default()
                .strip_prefix("mailto:")
                .unwrap_or_default()
                .to_owned()
        })
        .unwrap_or_default();
    if street.is_empty() && postcode.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Spans fehlen".to_owned(),
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

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    // Only the price area: everything between the price heading and the
    // company section. A stray "5 €" in the footer must never pair with
    // some random pending label into a phantom price.
    let start = html
        .find("Unsere Preise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbereich fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("Über uns").unwrap_or(tail.len());
    let html = &tail[..end];
    // Text-node walk: strip tags, split on the remnants.
    let mut text = String::with_capacity(html.len() / 2);
    let mut in_tag = false;
    for c in html.chars() {
        if c == '<' {
            in_tag = true;
            text.push('\n');
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            text.push(c);
        }
    }
    let mut rows: Vec<(String, f64, &'static str)> = Vec::new();
    let mut skips = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut pending: Option<String> = None;
    // Page-global unit: most rows quote bare "€ X" with the unit ("x pro
    // kg") in a sibling node or not at all. EUR/kg is the page default;
    // an explicit different unit always wins, an explicit unknown one
    // skips loudly.
    const PAGE_UNIT: &str = "EUR/kg";
    let nodes: Vec<&str> = text.split('\n').collect();
    let mut i = 0;
    while i < nodes.len() {
        let t = nodes[i]
            .replace("&nbsp;", " ")
            .replace(['\u{a0}', '\u{200b}'], " ")
            .trim()
            .to_owned();
        i += 1;
        if t.is_empty() {
            continue;
        }
        if t.contains('€') {
            if let (Some(price), Some(label)) = (parse_eur(&t), pending.take()) {
                let mut unit = unit_of(&t);
                // Unit split across nodes ("€ 8,59" + "x pro kg")?
                // Tag-stripping leaves empty nodes between siblings, so
                // scan past them to the first non-empty sibling. A
                // sibling that names a unit ("pro …"/"…/…") but no known
                // one ("pro Sack") is explicitly foreign → skip, never
                // the page default. Anything else (next label, heading)
                // is left for the main walk.
                let mut foreign_unit = false;
                if unit.is_none() {
                    let mut j = i;
                    while j < nodes.len() {
                        let n = nodes[j]
                            .replace("&nbsp;", " ")
                            .replace(['\u{a0}', '\u{200b}'], " ")
                            .trim()
                            .to_owned();
                        if n.is_empty() {
                            j += 1;
                            continue;
                        }
                        if let Some(u) = unit_of(&n) {
                            unit = Some(u);
                            i = j + 1; // consumed as unit, not a label
                        } else if n.contains('/') || n.to_lowercase().contains("pro") {
                            foreign_unit = true;
                            i = j + 1; // consumed as (unusable) unit
                        }
                        break;
                    }
                }
                // Bare "€ X" inherits the page default; an explicit but
                // unknown unit ("pro Sack") skips loudly instead.
                let lower = t.to_lowercase();
                let unit = unit.or(
                    if foreign_unit || t.contains('/') || lower.contains("pro") {
                        None
                    } else {
                        Some(PAGE_UNIT)
                    },
                );
                let Some(unit) = unit else {
                    skips.push(format!("{label} (Einheit unverständlich: {t})"));
                    continue;
                };
                // Never record an exact 0.00 price — that is "no buy price".
                if price == 0.0 {
                    skips.push(format!("{label} (Preis 0,00 — kein Ankaufspreis)"));
                    continue;
                }
                // The page repeats the block under "Gültig ab": dedupe.
                if seen.insert((label.clone(), price.to_bits())) {
                    rows.push((label, price, unit));
                }
            }
        } else if is_junk(&t) {
            pending = None;
        } else {
            pending = Some(t);
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preispaare".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS page (live: "€ 9,19 x pro kg", "€ 100
/// x pro to"). Only kg/t exist here — anything else returns None and the
/// caller decides (page default vs. loud skip).
fn unit_of(t: &str) -> Option<&'static str> {
    let lower = t.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|w| w == "t" || w == "to")
    {
        Some("EUR/t")
    } else {
        None
    }
}

/// Section headings and prose — never a material label.
fn is_junk(t: &str) -> bool {
    let l = t.to_lowercase();
    [
        "unsere preise",
        "gültig ab",
        "über uns",
        "leistungen",
        "kontakt",
        "impressum",
        "datenschutz",
        "kein ankauf",
        "cookies",
        "anfrage",
        "rufen sie",
        "folgen sie",
    ]
    .iter()
    .any(|j| l.contains(j))
        || l.len() > 120
}

#[cfg(test)]
mod tests {
    use super::{grade_for, is_junk, parse};

    // Real shape, trimmed: nested ed-text cards, unit glued into split
    // <strong> nodes, zero-width chars, "Gültig ab" repeat with
    // different spellings, Altpapier (no catalog material).
    const FIXTURE: &str = "<div class=\"ed-element ed-text custom-theme wv-light-edit\" id=\"ed-72845756\">\
        <h2>Unsere Preise</h2>\
        <table style=\"width: 100%;\"><tbody>\
        <tr><td><div style=\"text-align: center;\"><strong>Cu - Millberry</strong></div></td></tr>\
        <tr><td><div style=\"text-align: center;\"><strong>€ 9,19 x pro kg</strong></div></td></tr>\
        </tbody></table></div>\
        <div class=\"ed-element ed-text custom-theme wv-light-edit\" id=\"ed-72845759\">\
        <table style=\"width: 100%;\"><tbody>\
        <tr><td><div style=\"text-align: center;\"><strong>Cu – schwer</strong></div></td></tr>\
        <tr><td><p style=\"text-align: center;\"><strong>€ 8,59&nbsp;</strong><strong>x pro kg</strong></p></td></tr>\
        </tbody></table></div>\
        <div class=\"ed-element ed-text custom-theme wv-light-edit\" id=\"ed-72846127\">\u{200b}\
        <table style=\"width: 100%;\"><tbody>\
        <tr><td><div style=\"text-align: center;\"><strong>Cu-Kabel o. Stecker</strong></div></td></tr>\
        <tr><td><div style=\"text-align: center;\"><strong>€ 2,74 x pro kg</strong></div></td></tr>\
        </tbody></table></div>\
        <div class=\"ed-element ed-text custom-theme wv-light-edit\">\
        <table style=\"width: 100%;\"><tbody>\
        <tr><td><div style=\"text-align: center;\"><strong>Schwermessing</strong></div></td></tr>\
        <tr><td><div style=\"text-align: center;\"><strong>€ 4,03 x pro kg</strong></div></td></tr>\
        </tbody></table></div>\
        <div class=\"ed-element ed-text custom-theme wv-light-edit\">\
        <table style=\"width: 100%;\"><tbody>\
        <tr><td><div style=\"text-align: center;\"><strong>Mischschrott</strong></div></td></tr>\
        <tr><td><div style=\"text-align: center;\"><strong>€ 100 x pro to</strong></div></td></tr>\
        </tbody></table>\
        <h2>Gültig ab</h2>\
        <table style=\"width: 100%;\"><tbody>\
        <tr><td><div style=\"text-align: center;\"><strong>Cu - Millberry</strong></div></td></tr>\
        <tr><td><div style=\"text-align: center;\"><strong>€ 9,19 X € pro kg</strong></div></td></tr>\
        </tbody></table>\
        <table style=\"width: 100%;\"><tbody>\
        <tr><td><div style=\"text-align: center;\"><strong>Cu – Kabel ohne Stecker</strong></div></td></tr>\
        <tr><td><div style=\"text-align: center;\"><strong>€ 2,74 X € pro kg</strong></div></td></tr>\
        </tbody></table>\
        <table style=\"width: 100%;\"><tbody>\
        <tr><td><div style=\"text-align: center;\"><strong>Altpapier</strong></div></td></tr>\
        <tr><td><div style=\"text-align: center;\"><strong>€ 0,08 X € pro kg</strong></div></td></tr>\
        </tbody></table></div>";

    #[test]
    fn impressum_databind_spans() {
        let imp = "<address><span data-bind=\"customer.company\">Lausitz Recycling</span><br>            <span data-bind=\"customer.street\">Eisenwerkstr. 29</span><br>            <span data-bind=\"customer.zip\">01979</span>             <span data-bind=\"customer.city\">Lauchhammer-Ost</span></address>            <p>Telefon: <span data-bind=\"customer.phone\">03574/467452</span></p>            <p><a href=\"mailto:info@lausitz-recycling.de\">info</a></p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Eisenwerkstr. 29");
        assert_eq!(info.postcode, "01979");
        assert_eq!(info.city, "Lauchhammer-Ost");
        assert_eq!(info.phone, "03574/467452");
        assert_eq!(info.email, "info@lausitz-recycling.de");
        assert!(super::extract_info("<p>leer</p>").is_err());
    }

    #[test]
    fn pairs_dedupe_and_map() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // 7 price pairs: "Gültig ab" repeats Millberry identically
        // (deduped); "Kabel ohne Stecker" is a different spelling of the
        // same grade — parse keeps it, scrape collapses it.
        assert_eq!(rows.len(), 7, "rows: {rows:?}");
        assert!(skips.is_empty());
        assert_eq!(rows[0].0, "Cu - Millberry");
        assert_eq!(rows[0].1, 9.19);
        assert_eq!(rows[1].0, "Cu – schwer");
        assert_eq!(rows[1].1, 8.59);
        assert_eq!(rows[4].0, "Mischschrott");
        assert_eq!(rows[4].2, "EUR/t");
        // Every live label maps — or loudly doesn't.
        assert_eq!(grade_for("Cu - Millberry"), Some(("kupfer-millberry", "")));
        assert_eq!(
            grade_for("Cu – schwer"),
            Some(("kupfer-gemischt", "schwer"))
        );
        assert_eq!(grade_for("Cu –Berry"), Some(("kupfer-gemischt", "schwer")));
        assert_eq!(grade_for("Cu-Raff."), Some(("kupfer-gemischt", "Raff")));
        assert_eq!(grade_for("Cu – Raff"), Some(("kupfer-gemischt", "Raff")));
        assert_eq!(grade_for("Cu-Kabel o. Stecker"), Some(("kabel-kupfer", "")));
        assert_eq!(
            grade_for("Cu – Kabel ohne Stecker"),
            Some(("kabel-kupfer", ""))
        );
        assert_eq!(grade_for("Schwermessing"), Some(("messing", "")));
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Altpapier"), None);
        assert!(is_junk("KEIN ANKAUF MEHR VON"));
    }

    #[test]
    fn zero_price_skips_loudly() {
        let html = FIXTURE.replace("€ 4,03 x pro kg", "€ 0,00 x pro kg");
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 6);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Schwermessing"), "{skips:?}");
        assert!(skips[0].contains("0,00"), "{skips:?}");
    }

    #[test]
    fn split_unit_nodes_resolve() {
        // "€ 8,59" + sibling "x pro kg": the unit lives next door.
        let html = "<h2>Unsere Preise</h2>\
            <table><tr><td>Cu - Millberry</td></tr>\
            <tr><td>€ 9,19</td></tr><tr><td>x pro kg</td></tr></table>\
            <h2>Über uns</h2>";
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].2, "EUR/kg");
        assert!(skips.is_empty());
    }

    #[test]
    fn window_and_unit_safety() {
        // A stray euro amount outside the price area must not pair up.
        let html = "<p>Container ab 49 €</p>".to_owned()
            + FIXTURE
            + "<h2>Über uns</h2><p>Anfahrt pauschal 10 €</p>";
        let (rows, _) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 7);
        // Unknown unit: skipped loudly, valid rows survive.
        let html = FIXTURE.replace("pro kg", "pro Sack");
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "Mischschrott");
        assert_eq!(skips.len(), 7);
    }
}
