//! Rheinische Scheidestätte GmbH — Filialen Hamburg (Steinstraße 27,
//! 20095 Hamburg), Frankfurt (Hochstr. 29, 60313 Frankfurt) und Wiesbaden
//! (Bahnhofstraße 15-17, 65185 Wiesbaden): acceptance-only lists from each
//! branch page's `ul.store-servicelist` ("Unsere Services vor Ort" …
//! `id="nav-map"`), plus branch contact from the `h3.h4` / `div.address` /
//! `div.phone` block on the same page (that block IS the branch's
//! impressum-equivalent — the central `/impressum/` only covers the
//! Düsseldorf HQ, so it is never used here).
//!
//! ONE file for three slugs on purpose: all three branch pages share the
//! exact form of `rheinische_paderborn.rs` (verified live 29.09.2026 —
//! same list markup, same map-pane terminator, same contact block, same
//! footer spot widget), so the parsing below is bespoke for this form,
//! parameterized only by branch URL + city anchor. No code is shared with
//! any other handler file (only `fetch_text` from `super`).
//!
//! Slugs verified live in seed files: `hh-altstadt-rheinische-
//! scheidestatte-filiale-hamburg` (`dossiers/hh/`, seed website is
//! exactly the Hamburg branch page), `he-frankfurt-rheinische-
//! scheidestatte-filialen-frankf` and `he-wiesbaden-rheinische-
//! scheidestatte-filialen-frankf` (`dossiers/he/`, seed website is
//! the bare homepage — the live price/acceptance carriers are the
//! per-city branch pages below, both HTTP 200 live 29.09.2026).
//!
//! The central Kurse page below carries NO per-gram buying prices — only a
//! shop spot widget (`span.metal` + `span.price`, "Preise in EUR pro
//! Feinunze", refreshed every 10 minutes) plus prose/FAQ links, verified
//! live 29.09.2026 (Gold 3.636,30 € et al. in static HTML — numbers ARE
//! static, but per-ounce). A per-ounce quote recorded as per-gram would be
//! a ~31x error and oz→g is no proven normalization (only kg↔t is), so
//! every spot row is reported as a loud skip, never a price. Same for each
//! branch page itself (only the footer `ul.kursentwicklung` per-ounce
//! widget — Hamburg/Frankfurt 4 € hits, Wiesbaden 5 incl. a "2€/Stunde"
//! parking note, all outside the service-list window) and the
//! `goldankauf-hamburg` / `goldankauf-frankfurt` city pages (same widget,
//! fineness prose, no quotes).

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG_HAMBURG: &str = "hh-altstadt-rheinische-scheidestatte-filiale-hamburg";
/// Bespoke, live-verified branch page (Hamburg, Steinstraße).
/// Carries the service list AND the branch contact block; a move fails
/// the step loudly (fix the URL).
pub const URL_HAMBURG: &str = "https://rheinische-scheidestaette.de/unternehmen/filialen/hamburg/";
/// Bespoke, live-verified branch contact anchor page (same page — the
/// branch block is the contact source of truth, never the HQ impressum).
pub const IMPRESSUM_URL_HAMBURG: &str =
    "https://rheinische-scheidestaette.de/unternehmen/filialen/hamburg/";

pub const SLUG_FRANKFURT: &str = "he-frankfurt-rheinische-scheidestatte-filialen-frankf";
/// Bespoke, live-verified branch page (Frankfurt, Hochstraße).
pub const URL_FRANKFURT: &str =
    "https://rheinische-scheidestaette.de/unternehmen/filialen/frankfurt/";
/// Bespoke, live-verified branch contact anchor page (same page).
pub const IMPRESSUM_URL_FRANKFURT: &str =
    "https://rheinische-scheidestaette.de/unternehmen/filialen/frankfurt/";

pub const SLUG_WIESBADEN: &str = "he-wiesbaden-rheinische-scheidestatte-filialen-frankf";
/// Bespoke, live-verified branch page (Wiesbaden, Bahnhofstraße).
pub const URL_WIESBADEN: &str =
    "https://rheinische-scheidestaette.de/unternehmen/filialen/wiesbaden/";
/// Bespoke, live-verified branch contact anchor page (same page).
pub const IMPRESSUM_URL_WIESBADEN: &str =
    "https://rheinische-scheidestaette.de/unternehmen/filialen/wiesbaden/";

/// Central Kurse URL, hardcoded per file on purpose (intentional
/// duplication, never a shared constant). Live-checked per scrape: spot
/// rows become loud skips, never prices.
pub const KURSE_URL: &str =
    "https://rheinische-scheidestaette.de/infothek/aktuelle-edelmetallkurse/";
/// Branch headings each handler's contact block must carry. Any other
/// heading → the page changed shape → loud error. Note Wiesbaden uses an
/// en dash (–, U+2013), Hamburg/Frankfurt a hyphen (-).
const CITY_ANCHOR_HAMBURG: &str = "Rheinische Scheidestätte GmbH - Hamburg";
const CITY_ANCHOR_FRANKFURT: &str = "Rheinische Scheidestätte GmbH - Frankfurt";
const CITY_ANCHOR_WIESBADEN: &str = "Rheinische Scheidestätte GmbH – Wiesbaden";

pub fn handler_hamburg() -> Handler {
    Handler {
        slug: SLUG_HAMBURG,
        url: URL_HAMBURG,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_hamburg(c)),
    }
}

pub fn handler_frankfurt() -> Handler {
    Handler {
        slug: SLUG_FRANKFURT,
        url: URL_FRANKFURT,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_frankfurt(c)),
    }
}

pub fn handler_wiesbaden() -> Handler {
    Handler {
        slug: SLUG_WIESBADEN,
        url: URL_WIESBADEN,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_wiesbaden(c)),
    }
}

async fn scrape_hamburg(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape_impl(
        client,
        URL_HAMBURG,
        IMPRESSUM_URL_HAMBURG,
        CITY_ANCHOR_HAMBURG,
    )
    .await
}

async fn scrape_frankfurt(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape_impl(
        client,
        URL_FRANKFURT,
        IMPRESSUM_URL_FRANKFURT,
        CITY_ANCHOR_FRANKFURT,
    )
    .await
}

async fn scrape_wiesbaden(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape_impl(
        client,
        URL_WIESBADEN,
        IMPRESSUM_URL_WIESBADEN,
        CITY_ANCHOR_WIESBADEN,
    )
    .await
}

async fn scrape_impl(
    client: &reqwest::Client,
    url: &'static str,
    impressum_url: &'static str,
    city_anchor: &'static str,
) -> Result<HandlerOutcome, IngestError> {
    // Central Kurse page first: evidence-only. Its spot quotes are loud
    // skips (per-ounce shop prices, never per-gram buying prices); a dead
    // or redesigned Kurse page must not fail the branch acceptances.
    let mut skipped_labels = match fetch_text(client, KURSE_URL).await {
        Ok((_, kurse_html)) => kurse_skips(&kurse_html),
        Err(e) => vec![format!("Kurse-Seite unerreichbar ({KURSE_URL}: {e})")],
    };
    let (status, html) = fetch_text(client, url).await?;
    let labels = parse(&html, url)?;
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
    let trader_info = extract_info(&html, impressum_url, city_anchor)?;
    Ok(HandlerOutcome {
        prices: vec![],
        acceptances,
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: url.to_owned(),
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
fn parse(html: &str, url: &str) -> Result<Vec<String>, IngestError> {
    let start = html
        .find("store-servicelist")
        .ok_or_else(|| IngestError::Parse {
            url: url.to_owned(),
            detail: "Annahmeliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("id=\"nav-map\"")
        .ok_or_else(|| IngestError::Parse {
            url: url.to_owned(),
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
            url: url.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    Ok(labels)
}

/// Spot rows of the central Kurse page as loud skips ("Gold 3.636,30 €" is
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

/// Bespoke contact extraction for THESE branch pages only: the `h3.h4`
/// branch heading (must contain the city's anchor), then `div.address`
/// ("Steinstraße 27<br>20095 Hamburg" etc.) and `div.phone` (`tel:` +
/// `mailto:` anchors). Missing anchors → loud error, never a guessed
/// fallback.
fn extract_info(
    imp: &str,
    impressum_url: &str,
    city_anchor: &str,
) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h3 = Selector::parse("h3").expect("valid selector");
    if !doc
        .select(&h3)
        .any(|el| el.text().collect::<String>().contains(city_anchor))
    {
        return Err(IngestError::Parse {
            url: impressum_url.to_owned(),
            detail: "Filial-Block fehlt".to_owned(),
        });
    }
    let div = Selector::parse("div.address").expect("valid selector");
    let addr_html = doc
        .select(&div)
        .next()
        .map(|el| el.inner_html())
        .ok_or_else(|| IngestError::Parse {
            url: impressum_url.to_owned(),
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
            url: impressum_url.to_owned(),
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
    use super::{
        extract_info, grade_for, kurse_skips, parse, CITY_ANCHOR_FRANKFURT, CITY_ANCHOR_HAMBURG,
        CITY_ANCHOR_WIESBADEN, IMPRESSUM_URL_FRANKFURT, IMPRESSUM_URL_HAMBURG,
        IMPRESSUM_URL_WIESBADEN, URL_FRANKFURT, URL_HAMBURG, URL_WIESBADEN,
    };

    // Verbatim excerpts of the live branch pages (29.09.2026): service
    // list + map-pane terminator. Per-city quirks are real: Hamburg uses
    // `diamantankauf-hamburg/` (singular) and `tafelgeschaeft-hamburg`
    // (no `anonymes-` prefix); Frankfurt uses `diamantenankauf-frankfurt/`
    // (plural) and the `goldbmuenzen-kaufen-frankfurt/` typo link target;
    // Wiesbaden lists Silberbarren/-münzen before Goldbarren/-münzen and
    // uses `diamantankauf-wiesbaden/` (singular).
    const FIXTURE_HAMBURG: &str = "<ul class=\"store-servicelist\">\
        <li><p>Ankauf von Edelmetallen (<a href=\"https://rheinische-scheidestaette.de/goldankauf-hamburg\">Gold</a>, <a href=\"https://rheinische-scheidestaette.de/silberankauf-hamburg/\">Silber</a>, <a href=\"https://rheinische-scheidestaette.de/platin-verkaufen-hamburg\">Platin</a>, <a href=\"https://rheinische-scheidestaette.de/palladium-verkaufen-hamburg\">Palladium</a>, <a href=\"https://rheinische-scheidestaette.de/zahngoldankauf-hamburg\">Zahngold</a>, <a href=\"https://rheinische-scheidestaette.de/altgoldankauf-hamburg\">Alt- und Bruchgold</a>, <a href=\"https://rheinische-scheidestaette.de/goldbarren-verkaufen-hamburg\">Goldbarren</a>, <a href=\"https://rheinische-scheidestaette.de/goldmuenzen-verkaufen-hamburg\">Goldmünzen</a>, <a href=\"https://rheinische-scheidestaette.de/silberbarren-verkaufen-hamburg\">Silberbarren</a>, <a href=\"https://rheinische-scheidestaette.de/silbermuenzen-verkaufen-hamburg\">Silbermünzen</a>)<br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/uhrenankauf-hamburg\">Ankauf von Markenuhren </a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/markenschmuck-verkaufen-hamburg\">Ankauf von Markenschmuck</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/diamantankauf-hamburg/\">Ankauf von Diamanten</a><br /></li>\
        <li>Verkauf von <a href=\"https://rheinische-scheidestaette.de/goldmuenzen-kaufen-hamburg\">Münzen</a> &amp; <a href=\"https://rheinische-scheidestaette.de/goldbarren-kaufen-hamburg\">Barren</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/leistungen/edelmetalle-verkaufen/\">Kostenlose Wertermittlung</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/tafelgeschaeft-hamburg\">Anonymes Tafelgeschäft</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/leistungen/edelmetalle-kaufen/\">Investmentberatung</a></p></li><li></li>\
        </ul><div class=\"tab-pane fade\" id=\"nav-map\" role=\"tabpanel\">Karte</div>";

    const FIXTURE_FRANKFURT: &str = "<ul class=\"store-servicelist\">\
        <li><p>Ankauf von Edelmetallen (<a href=\"https://rheinische-scheidestaette.de/goldankauf-frankfurt/\">Gold</a>, <a href=\"https://rheinische-scheidestaette.de/silberankauf-frankfurt/\">Silber</a>, <a href=\"https://rheinische-scheidestaette.de/platin-verkaufen-frankfurt/\">Platin</a>, <a href=\"https://rheinische-scheidestaette.de/palladium-verkaufen-frankfurt/\">Palladium</a>, <a href=\"https://rheinische-scheidestaette.de/zahngoldankauf-frankfurt/\">Zahngold</a>, <a href=\"https://rheinische-scheidestaette.de/altgoldankauf-frankfurt/\">Alt- und Bruchgold</a>, <a href=\"https://rheinische-scheidestaette.de/goldbarren-verkaufen-frankfurt/\">Goldbarren</a>, <a href=\"https://rheinische-scheidestaette.de/goldmuenzen-verkaufen-frankfurt/\">Goldmünzen</a>, <a href=\"https://rheinische-scheidestaette.de/silberbarren-verkaufen-frankfurt/\">Silberbarren</a>, <a href=\"https://rheinische-scheidestaette.de/silbermuenzen-verkaufen-frankfurt/\">Silbermünzen</a>)<br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/uhrenankauf-frankfurt/\">Ankauf von Markenuhren</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/markenschmuck-verkaufen-frankfurt/\">Ankauf von Markenschmuck</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/diamantenankauf-frankfurt/\">Ankauf von Diamanten</a><br /></li>\
        <li>Verkauf von <a href=\"https://rheinische-scheidestaette.de/goldbmuenzen-kaufen-frankfurt/\">Münzen</a> &amp; <a href=\"https://rheinische-scheidestaette.de/goldbarren-kaufen-frankfurt/\">Barren</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/leistungen/edelmetalle-verkaufen/\">Kostenlose Wertermittlung</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/tafelgeschaeft-frankfurt/\">Anonymes Tafelgeschäft</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/leistungen/edelmetalle-kaufen/\">Investmentberatung</a></p></li><li></li>\
        </ul><div class=\"tab-pane fade\" id=\"nav-map\" role=\"tabpanel\">Karte</div>";

    const FIXTURE_WIESBADEN: &str = "<ul class=\"store-servicelist\">\
        <li><p>Ankauf von Edelmetallen (<a href=\"https://rheinische-scheidestaette.de/goldankauf-wiesbaden/\">Gold</a>, <a href=\"https://rheinische-scheidestaette.de/silberankauf-wiesbaden/\">Silber</a>, <a href=\"https://rheinische-scheidestaette.de/platin-verkaufen-wiesbaden/\">Platin</a>, <a href=\"https://rheinische-scheidestaette.de/palladium-verkaufen-wiesbaden/\">Palladium</a>, <a href=\"https://rheinische-scheidestaette.de/zahngoldankauf-wiesbaden/\">Zahngold</a>, <a href=\"https://rheinische-scheidestaette.de/altgoldankauf-wiesbaden/\">Alt- und Bruchgold</a>, <a href=\"https://rheinische-scheidestaette.de/silberbarren-verkaufen-wiesbaden/\">Silberbarren</a>, <a href=\"https://rheinische-scheidestaette.de/silbermuenzen-verkaufen-wiesbaden/\">Silbermünzen</a>, <a href=\"https://rheinische-scheidestaette.de/goldbarren-verkaufen-wiesbaden/\">Goldbarren</a>, <a href=\"https://rheinische-scheidestaette.de/goldmuenzen-verkaufen-wiesbaden/\">Goldmünzen</a>)<br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/uhrenankauf-wiesbaden/\">Ankauf von Markenuhren</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/markenschmuck-verkaufen-wiesbaden/\">Ankauf von Markenschmuck</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/diamantankauf-wiesbaden/\">Ankauf von Diamanten</a><br /></li>\
        <li>Verkauf von <a href=\"https://rheinische-scheidestaette.de/goldmuenzen-kaufen-wiesbaden\">Münzen</a> &amp; <a href=\"https://rheinische-scheidestaette.de/goldbarren-kaufen-wiesbaden/\">Barren</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/leistungen/edelmetalle-verkaufen/\">Kostenlose Wertermittlung</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/tafelgeschaeft-wiesbaden/\">Anonymes Tafelgeschäft</a><br /></li>\
        <li><a href=\"https://rheinische-scheidestaette.de/leistungen/edelmetalle-kaufen/\">Investmentberatung</a></p></li><li></li>\
        </ul><div class=\"tab-pane fade\" id=\"nav-map\" role=\"tabpanel\">Karte</div>";

    // Verbatim shape of the live central Kurse page (29.09.2026): per-ounce
    // spot widget, "Preise in EUR pro Feinunze".
    const FIXTURE_KURSE: &str = "<h2><span class=\"headline_dash\">Aktuelle Börsenkurse</span></h2>\
        <div class=\"tmetal\"><a href=\"https://rheinische-scheidestaette.de/wissenswertes/aktuelle-edelmetallkurse/goldkurs/\"><span class=\"metal h4\">Gold</span></a><br><span id=\"tp_gold\" class=\"price\">3.636,30 €</span></div>\
        <div class=\"tmetal\"><a href=\"https://rheinische-scheidestaette.de/wissenswertes/aktuelle-edelmetallkurse/silberkurs/\"><span class=\"metal h4\">Silber</span></a><br><span id=\"tp_silber\" class=\"price\">53,28 €</span></div>\
        <div class=\"tmetal\"><a href=\"https://rheinische-scheidestaette.de/wissenswertes/aktuelle-edelmetallkurse/palladiumkurs/\"><span class=\"metal h4\">Palladium</span></a><br><span id=\"tp_palladium\" class=\"price\">1.062,40 €</span></div>\
        <div class=\"tmetal\"><a href=\"https://rheinische-scheidestaette.de/wissenswertes/aktuelle-edelmetallkurse/platinkurs/\"><span class=\"metal h4\">Platin</span></a><br><span id=\"tp_platin\" class=\"price\">1.488,72 €</span></div>\
        <small>Preise in EUR pro Feinunze<br>Unsere Edelmetallpreise unterliegen u.a. dem Börsenkurs.</small>";

    #[test]
    fn services_map_and_skip_all_branches() {
        for (fixture, url) in [
            (FIXTURE_HAMBURG, URL_HAMBURG),
            (FIXTURE_FRANKFURT, URL_FRANKFURT),
            (FIXTURE_WIESBADEN, URL_WIESBADEN),
        ] {
            let labels = parse(fixture, url).expect("parses");
            assert_eq!(labels.len(), 8, "{url}");
            let first = &labels[0];
            assert!(first.contains("Ankauf von Edelmetallen"), "{first}");
            let fan = grade_for(first).expect("fans out");
            let mats: Vec<&str> = fan.iter().map(|(m, _)| *m).collect();
            assert_eq!(
                mats,
                vec!["gold", "zahngold", "silber", "platin", "palladium"],
                "{url}"
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
        }
        assert!(parse("<div>Redesign ohne Liste</div>", URL_HAMBURG).is_err());
        assert!(parse("store-servicelist ohne Ende", URL_FRANKFURT).is_err());
        assert!(parse("<div>Redesign ohne Liste</div>", URL_WIESBADEN).is_err());
    }

    #[test]
    fn kurse_spots_skip_loudly() {
        let skips = kurse_skips(FIXTURE_KURSE);
        assert_eq!(skips.len(), 4);
        assert!(
            skips[0].contains("Gold") && skips[0].contains("Feinunze"),
            "{skips:?}"
        );
        assert!(skips.iter().any(|s| s.contains("Platin")), "{skips:?}");
        assert!(skips.iter().any(|s| s.contains("Palladium")), "{skips:?}");
        let note = kurse_skips("<div>Redesign</div>");
        assert_eq!(note.len(), 1);
        assert!(note[0].contains("Kurse-Block fehlt"), "{note:?}");
    }

    #[test]
    fn branch_contact_blocks() {
        // Verbatim contact blocks of the live branch pages (29.09.2026).
        let imp_hh = "<h3 class=\"h4\">Rheinische Scheidestätte GmbH - Hamburg</h3>\
            <div class=\"address\">Steinstraße 27<br>20095 Hamburg</div>\
            <div class=\"phone\"><a href=\"tel:+4940248278787\">040-248278787</a>\
            <a class=\"d-block\" href=\"mailto:info-hamburg@rheinische-scheidestaette.de\">info-hamburg@rheinische-scheidestaette.de</a></div>";
        let info =
            extract_info(imp_hh, IMPRESSUM_URL_HAMBURG, CITY_ANCHOR_HAMBURG).expect("parses");
        assert_eq!(info.street, "Steinstraße 27");
        assert_eq!(info.postcode, "20095");
        assert_eq!(info.city, "Hamburg");
        assert_eq!(info.phone, "040-248278787");
        assert_eq!(info.email, "info-hamburg@rheinische-scheidestaette.de");

        let imp_ffm = "<h3 class=\"h4\">Rheinische Scheidestätte GmbH - Frankfurt</h3>\
            <div class=\"address\">Hochstr. 29<br>60313 Frankfurt</div>\
            <div class=\"phone\"><a href=\"tel:+496977011759\">069-77011759</a>\
            <a class=\"d-block\" href=\"mailto:info-frankfurt@rheinische-scheidestaette.de\">info-frankfurt@rheinische-scheidestaette.de</a></div>";
        let info =
            extract_info(imp_ffm, IMPRESSUM_URL_FRANKFURT, CITY_ANCHOR_FRANKFURT).expect("parses");
        assert_eq!(info.street, "Hochstr. 29");
        assert_eq!(info.postcode, "60313");
        assert_eq!(info.city, "Frankfurt");
        assert_eq!(info.phone, "069-77011759");
        assert_eq!(info.email, "info-frankfurt@rheinische-scheidestaette.de");

        // Wiesbaden heading uses an en dash (U+2013), not a hyphen.
        let imp_wi = "<h3 class=\"h4\">Rheinische Scheidestätte GmbH – Wiesbaden</h3>\
            <div class=\"address\">Bahnhofstraße 15-17<br>65185 Wiesbaden</div>\
            <div class=\"phone\"><a href=\"tel:+4961198874968\">0611-98874968</a>\
            <a class=\"d-block\" href=\"mailto:info-wiesbaden@rheinische-scheidestaette.de\">info-wiesbaden@rheinische-scheidestaette.de</a></div>";
        let info =
            extract_info(imp_wi, IMPRESSUM_URL_WIESBADEN, CITY_ANCHOR_WIESBADEN).expect("parses");
        assert_eq!(info.street, "Bahnhofstraße 15-17");
        assert_eq!(info.postcode, "65185");
        assert_eq!(info.city, "Wiesbaden");
        assert_eq!(info.phone, "0611-98874968");
        assert_eq!(info.email, "info-wiesbaden@rheinische-scheidestaette.de");

        // Wrong-city anchor and anchorless pages fail loudly.
        assert!(extract_info(imp_hh, IMPRESSUM_URL_HAMBURG, CITY_ANCHOR_FRANKFURT).is_err());
        assert!(
            extract_info(
                "<h3 class=\"h4\">Rheinische Scheidestätte GmbH - Wiesbaden</h3>",
                IMPRESSUM_URL_WIESBADEN,
                CITY_ANCHOR_WIESBADEN
            )
            .is_err(),
            "hyphen must not match the en-dash anchor"
        );
        assert!(extract_info(
            "<p>Neu hier</p>",
            IMPRESSUM_URL_WIESBADEN,
            CITY_ANCHOR_WIESBADEN
        )
        .is_err());
    }
}
