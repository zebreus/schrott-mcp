//! Rheinische Scheidestätte GmbH — Filiale Trier (Konstantinstraße 8-10,
//! 54290 Trier): acceptance-only list from the branch page's
//! `ul.store-servicelist` ("Unsere Services vor Ort" … `id="nav-map"`), plus
//! branch contact from the `h3.h4` / `div.address` / `div.phone` block on the
//! same page (that block IS the branch's impressum-equivalent — the central
//! `/impressum/` only covers the Düsseldorf HQ, so it is never used here).
//!
//! Slug verified live in `dossiers/rp/`
//! (`rp-trier-54290-rheinische-scheidestatte-filiale-trier`). The seed
//! address (Konstantinstraße 8-10) matches exactly this `/filialen/trier/`
//! page — the separate `/filialen/trier-sued/` page is a different branch
//! and is not covered here.
//!
//! The central Kurse page below carries NO per-gram buying prices — only a
//! shop spot widget (`span.metal` + `span.price`, "Preise in EUR pro
//! Feinunze", refreshed every 10 minutes) plus prose/FAQ links, verified
//! live 27.09.2026. A per-ounce quote recorded as per-gram would be a ~31x
//! error and oz→g is no proven normalization (only kg↔t is), so every spot
//! row is reported as a loud skip, never a price. Same for the
//! `goldpreis-ankauf-trier` city page (fineness prose, no quotes).

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "rp-trier-54290-rheinische-scheidestatte-filiale-trier";
/// Bespoke, live-verified branch page (Trier, Konstantinstraße). Carries
/// the service list AND the branch contact block; a move fails the step
/// loudly (fix the URL).
pub const URL: &str = "https://rheinische-scheidestaette.de/unternehmen/filialen/trier/";
/// Bespoke, live-verified branch contact anchor page (same page — the
/// branch block is the contact source of truth, never the HQ impressum).
pub const IMPRESSUM_URL: &str = "https://rheinische-scheidestaette.de/unternehmen/filialen/trier/";
/// Central Kurse URL, hardcoded per file on purpose (intentional
/// duplication, never a shared constant). Live-checked per scrape: spot
/// rows become loud skips, never prices.
pub const KURSE_URL: &str =
    "https://rheinische-scheidestaette.de/infothek/aktuelle-edelmetallkurse/";
/// Branch heading this handler's contact block must carry. Any other
/// heading → the page changed shape → loud error.
const CITY_ANCHOR: &str = "Rheinische Scheidestätte GmbH - Trier";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    // Central Kurse page first: evidence-only. Its spot quotes are loud
    // skips (per-ounce shop prices, never per-gram buying prices); a dead
    // or redesigned Kurse page must not fail the branch acceptances.
    let mut skipped_labels = match fetch_text(client, KURSE_URL).await {
        Ok((_, kurse_html)) => kurse_skips(&kurse_html),
        Err(e) => vec![format!("Kurse-Seite unerreichbar ({KURSE_URL}: {e})")],
    };
    let (status, html) = fetch_text(client, URL).await?;
    let labels = parse(&html)?;
    let mut acceptances = Vec::new();
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
    // Contact failure fails the whole step on purpose: a moved branch
    // block means the site changed and needs eyeballs before we trust
    // anything from it again.
    let trader_info = extract_info(&html)?;
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

/// Explicit label → acceptances. The "Ankauf von Edelmetallen (…)" row fans
/// out to the five catalogued precious-metal materials (raw forms ride as
/// conditions); brand jewelry follows the gold_richtig precedent
/// ("schmuck" → `gold`). Watches, diamonds, the SELL side ("Verkauf von
/// Münzen & Barren" — selling is not acceptance) and pure services
/// (Wertermittlung, Tafelgeschäft, Investmentberatung) skip loudly: a wrong
/// acceptance is worse than a logged gap.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    if l.contains("ankauf von edelmetallen") {
        return Some(vec![
            ("gold", "Alt- und Bruchgold, Barren & Münzen"),
            ("zahngold", "Dentalgold"),
            ("silber", "Barren & Münzen"),
            ("platin", ""),
            ("palladium", ""),
        ]);
    }
    if l.contains("markenschmuck") {
        return Some(vec![("gold", "Markenschmuck")]);
    }
    None
}

/// Service rows between the "Unsere Services vor Ort" list head and the
/// map pane. Missing anchors → `Err`, never an empty success; 0 rows =
/// `Err` (silent success would hide a redesign).
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let start = html
        .find("store-servicelist")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("id=\"nav-map\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste unvollständig".to_owned(),
        })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let li = Selector::parse("li").expect("valid selector");
    let labels: Vec<String> = frag
        .select(&li)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|t| !t.is_empty())
        .collect();
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    Ok(labels)
}

/// Spot rows of the central Kurse page as loud skips ("Gold 3.761,75 €" is
/// a per-ounce shop price — never a per-gram buying price). Evidence-only:
/// a missing block yields a redesign note, never an error.
fn kurse_skips(html: &str) -> Vec<String> {
    let Some(start) = html.find("Aktuelle Börsenkurse") else {
        return vec![format!("Kurse-Block fehlt ({KURSE_URL}: Redesign?)")];
    };
    let tail = &html[start..];
    let Some(end) = tail.find("Preise in EUR pro Feinunze") else {
        return vec![format!(
            "Kurse-Block unvollständig ({KURSE_URL}: Redesign?)"
        )];
    };
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let metal = Selector::parse("span.metal").expect("valid selector");
    let price = Selector::parse("span.price").expect("valid selector");
    let metals: Vec<String> = frag
        .select(&metal)
        .map(|el| {
            el.text()
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();
    let prices: Vec<String> = frag
        .select(&price)
        .map(|el| {
            el.text()
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();
    if metals.is_empty() {
        return vec![format!("Kurse-Block leer ({KURSE_URL}: Redesign?)")];
    }
    metals
        .into_iter()
        .zip(prices.into_iter().chain(std::iter::repeat(String::new())))
        .map(|(m, p)| {
            format!(
                "{m} {} (Börsenkurs je Feinunze, kein Ankaufspreis pro Gramm)",
                p.trim()
            )
        })
        .collect()
}

/// Bespoke contact extraction for THIS branch page only: the `h3.h4`
/// branch heading (must contain the Trier anchor), then `div.address`
/// ("Konstantinstraße 8-10<br>54290 Trier") and `div.phone` (`tel:` +
/// `mailto:` anchors). Missing anchors → loud error, never a guessed
/// fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h3 = Selector::parse("h3").expect("valid selector");
    if !doc
        .select(&h3)
        .any(|el| el.text().collect::<String>().contains(CITY_ANCHOR))
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Filial-Block fehlt".to_owned(),
        });
    }
    let div = Selector::parse("div.address").expect("valid selector");
    let addr_html = doc
        .select(&div)
        .next()
        .map(|el| el.inner_html())
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        })?;
    let addr_lines: Vec<String> = addr_html
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let street = addr_lines.first().cloned().unwrap_or_default();
    let (mut postcode, mut city) = (String::new(), String::new());
    for line in &addr_lines {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| format!("{a} {b}"));
                break;
            }
        }
    }
    if street.is_empty() || postcode.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Adressdaten gefunden".to_owned(),
        });
    }
    let phone_div = Selector::parse("div.phone").expect("valid selector");
    let a = Selector::parse("a").expect("valid selector");
    let (mut phone, mut email) = (String::new(), String::new());
    if let Some(pdiv) = doc.select(&phone_div).next() {
        for link in pdiv.select(&a) {
            let href = link.value().attr("href").unwrap_or_default();
            let text: String = link
                .text()
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            if href.starts_with("tel:") && phone.is_empty() {
                phone = text;
            } else if href.starts_with("mailto:") && email.is_empty() {
                email = text;
            }
        }
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
    use super::{extract_info, grade_for, kurse_skips, parse};

    // Real shape of the live branch page (anchors, class names, link
    // targets), trimmed to the service list + map-pane terminator.
    const FIXTURE: &str = "<h4 class=\"d-block d-lg-none\">Unsere Services vor Ort</h4>\
        <ul class=\"store-servicelist\">\
        <li><p>Ankauf von Edelmetallen (<a href=\"https://rheinische-scheidestaette.de/goldankauf-trier/\">Gold</a>, <a href=\"https://rheinische-scheidestaette.de/silberankauf-trier/\">Silber</a>, <a href=\"https://rheinische-scheidestaette.de/platin-verkaufen-trier/\">Platin</a>, <a href=\"https://rheinische-scheidestaette.de/palladium-verkaufen-trier/\">Palladium</a>, <a href=\"https://rheinische-scheidestaette.de/zahngoldankauf-trier/\">Zahngold</a>, <a href=\"https://rheinische-scheidestaette.de/altgoldankauf-trier/\">Alt- und Bruchgold</a>, <a href=\"https://rheinische-scheidestaette.de/goldbarren-verkaufen-trier/\">Goldbarren</a>, <a href=\"https://rheinische-scheidestaette.de/goldmuenzen-verkaufen-trier/\">Goldmünzen</a>, <a href=\"https://rheinische-scheidestaette.de/silberbarren-verkaufen-trier/\">Silberbarren</a>, <a href=\"https://rheinische-scheidestaette.de/silbermuenzen-verkaufen-trier/\">Silbermünzen</a>)<br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/uhrenankauf-trier/\">Ankauf von Markenuhren</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/schmuckankauf-trier/\">Ankauf von Markenschmuck</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/diamantankauf-trier/\">Ankauf von Diamanten</a><br /></li>\
        <li>Verkauf von <a href=\"https://rheinische-scheidestaette.de/goldmuenzen-kaufen-trier/\">Münzen</a> &amp; <a href=\"https://rheinische-scheidestaette.de/goldbarren-kaufen-trier/\">Barren</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/leistungen/edelmetalle-verkaufen/\">Kostenlose Wertermittlung</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/tafelgeschaeft-trier/\">Anonymes Tafelgeschäft</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/leistungen/edelmetalle-kaufen/\">Investmentberatung</a></p></li><li></li>\
        </ul><div class=\"tab-pane fade\" id=\"nav-map\" role=\"tabpanel\">Karte</div>";

    const FIXTURE_KURSE: &str = "<h2>Aktuelle Börsenkurse</h2>\
        <div class=\"tmetal\"><a href=\"https://rheinische-scheidestaette.de/wissenswertes/aktuelle-edelmetallkurse/goldkurs/\"><span class=\"metal h4\">Gold</span></a><br><span id=\"tp_gold\" class=\"price\">3.761,75 €</span></div>\
        <div class=\"tmetal\"><a href=\"https://rheinische-scheidestaette.de/wissenswertes/aktuelle-edelmetallkurse/silberkurs/\"><span class=\"metal h4\">Silber</span></a><br><span id=\"tp_silber\" class=\"price\">56,44 €</span></div>\
        <small>Preise in EUR pro Feinunze<br>aktualisieren wir diese alle 10 Minuten.</small>";

    #[test]
    fn services_map_and_skip() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 8);
        let first = &labels[0];
        assert!(first.contains("Ankauf von Edelmetallen"), "{first}");
        let fan = grade_for(first).expect("fans out");
        let mats: Vec<&str> = fan.iter().map(|(m, _)| *m).collect();
        assert_eq!(
            mats,
            vec!["gold", "zahngold", "silber", "platin", "palladium"]
        );
        assert_eq!(
            grade_for("Ankauf von Markenschmuck"),
            Some(vec![("gold", "Markenschmuck")])
        );
        // Watches, diamonds, the sell side and pure services skip loudly.
        assert_eq!(grade_for("Ankauf von Markenuhren"), None);
        assert_eq!(grade_for("Ankauf von Diamanten"), None);
        assert_eq!(grade_for("Verkauf von Münzen & Barren"), None);
        assert_eq!(grade_for("Kostenlose Wertermittlung"), None);
        assert_eq!(grade_for("Anonymes Tafelgeschäft"), None);
        assert_eq!(grade_for("Investmentberatung"), None);
        assert!(parse("<div>Redesign ohne Liste</div>").is_err());
        assert!(parse("store-servicelist ohne Ende").is_err());
    }

    #[test]
    fn kurse_spots_skip_loudly() {
        let skips = kurse_skips(FIXTURE_KURSE);
        assert_eq!(skips.len(), 2);
        assert!(
            skips[0].contains("Gold") && skips[0].contains("Feinunze"),
            "{skips:?}"
        );
        assert!(
            skips[1].contains("Silber") && skips[1].contains("Feinunze"),
            "{skips:?}"
        );
        let note = kurse_skips("<div>Redesign</div>");
        assert_eq!(note.len(), 1);
        assert!(note[0].contains("Kurse-Block fehlt"), "{note:?}");
    }

    #[test]
    fn branch_contact_block() {
        let imp = "<h3 class=\"h4\">Rheinische Scheidestätte GmbH - Trier</h3>\
            <div class=\"address\">Konstantinstraße 8-10<br>54290 Trier</div>\
            <div class=\"phone\"><a href=\"tel:+4965191897765\">0651-91897765</a>\
            <a class=\"d-block\" href=\"mailto:info-trier@rheinische-scheidestaette.de\">info-trier@rheinische-scheidestaette.de</a></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Konstantinstraße 8-10");
        assert_eq!(info.postcode, "54290");
        assert_eq!(info.city, "Trier");
        assert_eq!(info.phone, "0651-91897765");
        assert_eq!(info.email, "info-trier@rheinische-scheidestaette.de");
        assert!(
            extract_info("<h3 class=\"h4\">Rheinische Scheidestätte GmbH - Bremen</h3>").is_err()
        );
        assert!(extract_info("<p>Neu hier</p>").is_err());
    }
}
