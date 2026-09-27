//! philoro Hamburg (Neuer Wall 77): acceptance list without per-gram
//! prices. The central Preisliste (`URL` below) is a shop table — every
//! product row carries a TOTAL buy/sell price ("3.928,57 €" for 31.10 g),
//! never a per-gram quote, so dividing by the Feingewicht would be a
//! forbidden unit conversion. This handler therefore only fills
//! `trader_materials` (each listed product proves philoro buys that metal)
//! plus contact enrichment from the branch page. Zero prices with resolved
//! acceptances is normal operation, not a canary trip.
//!
//! Live shape (27.09.2026): six `<thead>` tables ("Artikel … Name …
//! Feingewicht … Verkaufspreis … Kaufpreis") between the filter line "Nur
//! verfügbare Produkte anzeigen" and the SEO heading "Die philoro
//! Edelmetall-Preisliste". Product rows are `<tbody><tr>`s with two
//! `/produkt/` links (picture + name); section/header rows have none.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "hh-neustadt-philoro-filiale-hamburg";
/// Central price list, shared by all philoro branches by design.
/// Duplicated per handler file on purpose — never a shared constant, so a
/// URL move fails each step loudly instead of hiding behind an import.
pub const URL: &str = "https://philoro.de/preisliste/alle";
/// Bespoke, live-verified branch contact page (Hamburg: Neuer Wall 77,
/// 20354 Hamburg — reached via the Filialen overview, never guessed).
/// A move fails the step loudly.
pub const IMPRESSUM_URL: &str = "https://philoro.de/filialen/hamburg";
/// Street this branch must show in its contact block. Guards against the
/// shared template silently serving a different branch.
const BRANCH_STREET: &str = "Neuer Wall 77";

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
            None => skipped_labels.push(label),
        }
    }
    // Contact failure fails the whole step on purpose: a moved branch page
    // means the site changed and needs eyeballs before we trust anything
    // from it again.
    let (_, branch_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&branch_html)?;
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

/// Explicit label → acceptances. Every live product name carries its metal
/// word, so the section heading is never needed. Order matters: "Silber
/// Golden Eagle" and "Silber Australia's First Gold Rush" contain "gold" —
/// silber must win. Platin/palladium names never contain another metal
/// word. Anything without a metal word stays `None`.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("silber") {
        Some(vec![("silber", "")])
    } else if l.contains("platin") {
        Some(vec![("platin", "")])
    } else if l.contains("palladium") {
        Some(vec![("palladium", "")])
    } else if l.contains("gold") {
        Some(vec![("gold", "")])
    } else {
        None
    }
}

/// Product names from the six Preisliste tables inside the window between
/// the filter line and the SEO heading. The header words prove the right
/// tables are in the window; rows are picked by their `/produkt/` name
/// link (header/section rows have none). Returns (labels, shape_skips):
/// rows with zero product text are headers (silent), rows with several
/// distinct link texts or prose-length names skip loudly. Zero product
/// rows → `Err` (a redesign must never look like success).
fn parse(html: &str) -> Result<(Vec<String>, Vec<String>), IngestError> {
    let start = html
        .find("Nur verfügbare Produkte anzeigen")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste: Filter-Anker fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Die philoro Edelmetall-Preisliste")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste: Ende-Anker fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    for head in ["Feingewicht", "Verkaufspreis", "Kaufpreis"] {
        if !window.contains(head) {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: format!("Preisliste: Tabellenkopf fehlt ({head})"),
            });
        }
    }
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let tr_sel = Selector::parse("tbody tr").expect("valid selector");
    let link_sel = Selector::parse("a[href*=\"/produkt/\"]").expect("valid selector");
    let mut labels = Vec::new();
    let mut skipped = Vec::new();
    for row in frag.select(&tr_sel) {
        // One row links its product twice (picture + name); only the name
        // link carries text. Anything else is an unexpected shape.
        let mut texts: Vec<String> = row
            .select(&link_sel)
            .map(|el| el.text().collect::<String>())
            .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
            .filter(|t| !t.is_empty())
            .collect();
        texts.dedup();
        match texts.len() {
            0 => continue, // thead/section rows: no product link, no label.
            1 => {
                let label = texts.pop().expect("one text");
                if label.len() > 120 {
                    skipped.push(format!("{label} (Prosa, kein Produkt)"));
                } else {
                    labels.push(label);
                }
            }
            _ => skipped.push(format!("unerwartete Links: {}", texts.join(" | "))),
        }
    }
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste: keine Produktzeilen".to_owned(),
        });
    }
    Ok((labels, skipped))
}

/// Bespoke contact extraction for THIS branch page only: the single
/// `<address>` block carries `<h4>philoro EDELMETALLE GmbH</h4>`, an address
/// `<span>` ("Neuer Wall 77, 20354 Hamburg") and `tel:`/`mailto:` hrefs
/// (hrefs, never glued text nodes). Missing block, firm heading or branch
/// street → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let start = imp.find("<address").ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Adress-Block fehlt".to_owned(),
    })?;
    let tail = &imp[start..];
    let end = tail.find("</address>").ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Adress-Block unvollständig".to_owned(),
    })?;
    let block = &tail[..end];
    if !block.contains("<h4>philoro EDELMETALLE GmbH</h4>") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmen-Heading fehlt".to_owned(),
        });
    }
    if !block.contains(BRANCH_STREET) {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: format!("falsche Filiale (kein {BRANCH_STREET})"),
        });
    }
    let doc = Html::parse_fragment(&format!("<div>{block}</div>"));
    let span_sel = Selector::parse("span").expect("valid selector");
    let addr_text: String = doc
        .select(&span_sel)
        .next()
        .map(|el| el.text().collect())
        .unwrap_or_default();
    let addr_text = addr_text.split_whitespace().collect::<Vec<_>>().join(" ");
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if let Some((left, right)) = addr_text.split_once(',') {
        street = left.trim().to_owned();
        let mut it = right.split_whitespace();
        if let Some(pc) = it.next() {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.collect::<Vec<_>>().join(" ");
            }
        }
    }
    let href_val = |prefix: &str| {
        let mut out = String::new();
        let mut rest = block;
        while let Some(i) = rest.find(prefix) {
            let v = &rest[i + prefix.len()..];
            if let Some(q) = v.find('"') {
                let cand = v[..q].trim().to_owned();
                if !cand.is_empty() {
                    out = cand;
                    break;
                }
            }
            rest = &rest[i + prefix.len()..];
            if rest.is_empty() {
                break;
            }
        }
        out
    };
    let phone = href_val("href=\"tel:");
    let email = href_val("href=\"mailto:");
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
    use super::{extract_info, grade_for, parse};

    // Real shape of the live Preisliste (window anchors, thead heads,
    // section row without product link, product row with picture + name
    // links, weight + total-price cells), trimmed to two products.
    const FIXTURE: &str = "Nur verfügbare Produkte anzeigen</span></div>\
        <table><thead><tr><th>Artikel</th><td></td><th>Name</th><td></td>\
        <th>Feingewicht</th><th>Verkaufspreis</th><th>Kaufpreis</th></tr></thead>\
        <tbody><tr><td colspan=\"7\"><h4>Goldmünzen</h4></td></tr>\
        <tr><td><span>1991</span></td>\
        <td><a href=\"/produkt/gold-philharmoniker-1-oz-2026-1991v\"><picture></picture></a></td>\
        <td><a href=\"/produkt/gold-philharmoniker-1-oz-2026-1991v\">Gold Philharmoniker 1 oz - 2026</a></td>\
        <td><span></span></td><td><span>31.10 g</span></td>\
        <td aria-label=\"Verkaufen\"><span>3.743,39 €</span></td>\
        <td aria-label=\"Kaufen\"><span>3.928,57 €</span></td></tr>\
        <tr><td><span>3380</span></td>\
        <td><a href=\"/produkt/silber-golden-eagle-1-oz-2025-3380v\"><picture></picture></a></td>\
        <td><a href=\"/produkt/silber-golden-eagle-1-oz-2025-3380v\">Silber Golden Eagle 1 oz - 2025</a></td>\
        <td><span></span></td><td><span>31.10 g</span></td>\
        <td aria-label=\"Verkaufen\"><span>62,00 €</span></td>\
        <td aria-label=\"Kaufen\"><span>86,80 €</span></td></tr>\
        </tbody></table>Die philoro Edelmetall-Preisliste";

    #[test]
    fn table_rows_become_labels() {
        let (labels, shape_skips) = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0], "Gold Philharmoniker 1 oz - 2026");
        assert_eq!(labels[1], "Silber Golden Eagle 1 oz - 2025");
        assert!(shape_skips.is_empty());
        assert!(parse("<div>Redesign ohne Liste</div>").is_err());
        assert!(parse("Nur verfügbare Produkte anzeigen ohne Tabellen").is_err());
    }

    #[test]
    fn metals_map_silber_before_gold() {
        assert_eq!(
            grade_for("Gold Philharmoniker 1 oz - 2026"),
            Some(vec![("gold", "")])
        );
        assert_eq!(
            grade_for("Silber Maple Leaf 1 oz - 2026"),
            Some(vec![("silber", "")])
        );
        // Silver coins with "gold"/"Golden" in the name stay silver.
        assert_eq!(
            grade_for("Silber Golden Eagle 1 oz - 2025"),
            Some(vec![("silber", "")])
        );
        assert_eq!(
            grade_for("Silber Australia's First Gold Rush 1 oz PP - 2026"),
            Some(vec![("silber", "")])
        );
        assert_eq!(
            grade_for("Platin Lunar III 1/10 oz - Pferd 2026"),
            Some(vec![("platin", "")])
        );
        assert_eq!(
            grade_for("Palladiumbarren 100 g diverse Hersteller"),
            Some(vec![("palladium", "")])
        );
        assert_eq!(grade_for("Zubehör"), None);
    }

    #[test]
    fn branch_address_block() {
        let imp = "<address class=\"mb-8 flex flex-col gap-4 not-italic\">\
            <h4>philoro EDELMETALLE GmbH</h4>\
            <span class=\"body-l lg:body-l-medium\">Neuer Wall 77, 20354 Hamburg</span>\
            <div><a href=\"tel:+49 40 181000300\">+49 40 181000300</a></div>\
            <div><a href=\"mailto:hamburg@philoro.de\">hamburg@philoro.de</a></div></address>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Neuer Wall 77");
        assert_eq!(info.postcode, "20354");
        assert_eq!(info.city, "Hamburg");
        assert_eq!(info.phone, "+49 40 181000300");
        assert_eq!(info.email, "hamburg@philoro.de");
        assert!(extract_info("<div>ohne Adressblock</div>").is_err());
        // Another branch's page must not pass as Hamburg.
        let other = imp.replace(
            "Neuer Wall 77, 20354 Hamburg",
            "Wachtstraße 20, 28195 Bremen",
        );
        assert!(extract_info(&other).is_err());
    }
}
