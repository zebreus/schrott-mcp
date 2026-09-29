//! EAS-Recycling Solution GmbH (Herdecke): three static price pages in
//! identical YAML-GRID layout (`div.ym-g70 > div.ym-gbox` left blocks with
//! `<p><strong>Label</strong>…</p>` title + one `<p><strong>… € …</strong></p>`
//! price paragraph each) — no tables, no dates, all per-kilogram.
//!
//! Pages (all live-verified HTTP 200 on 28.09.2026):
//! - `/ankaufspreise/elektronische-bauteile/` (40 blocks: Leiterplatten
//!   3,00–28,50 €/kg, RAM Goldkante 85, Keramikprozessoren 220, Slot-CPU 80)
//! - `/ankaufspreise/elektronische-hardware/` (7 blocks: Computer 1,20,
//!   Laptop 2,20, Server 1,40 €/kg — whole devices, no catalog material)
//! - `/ankaufspreise/altmetalle/` (15 blocks: Millberry 7,40, Raff 6,20,
//!   Messing 3,60, Zinn 27,80, Kabel 2,20 €/kg)
//!
//! Deliberate mapping (Frisch precedent: CPUs/RAM/whole devices have no
//! catalog material and skip loudly — no `cpu`/`ram`/`e-schrott-geraete`
//! invention; vedder precedent: silver-plated cutlery is not `silber`,
//! mixed Al-Cu coolers are not a catalog alloy):
//! - Leiterplatten/Steckkarten/Festplatten-/Laufwerk-/Handy-Platinen and
//!   Rückwände → `platinen` (trader grade in `variant`). Unknown future
//!   Leiterplatten grades skip loudly (no silent `''` collapse).
//! - Arbeitsspeicher (Gold-/Silberkante, mit/ohne Alu), Slot-/Kunststoff-/
//!   Keramik-Prozessoren, ICs/Eprom, Handys/Smartphones, Computer/Laptops/
//!   Server, Netzteile, Laufwerke, Festplatten (whole), IDE-Kabel/Stecker,
//!   Ablenkeinheiten, Tastaturen, Drucker → `None` (loud skip).
//! - Millberry → `kupfer-millberry`; Raff → `kupfer-gemischt`/`Raff`
//!   (hendrichs precedent: grade in variant); Messing → `messing`;
//!   Zinn 99% / Zinngeschirr → `zinn` with grade variant; Kupferkabel +
//!   PC-Netzteil-Kabel → `kabel-kupfer` (mit/ohne Stecker variant);
//!   Aluminium Profile/gemischt → catalog alu materials.
//! - Mischschrott 0,00, Tastaturen 0,00, Drucker 0,00 → loud 0.00 skips,
//!   never price-0 rows. "Preis auf Anfrage!" (iCore) → loud skip.
//! - Rückwände "4,00 € - 40,00 €" → `range`/0.5 with both bounds, price =
//!   max (VANA pattern); everything else is `exact`/1.0.
//! - No validity date anywhere on the three pages → `published_at` None.
//!
//! Impressum quirks: the address lives in the `<p>` holding
//! "Betriebsstätte:" (Betriebsstätte Ostender Weg 12A, 58313 Herdecke —
//! the Wienbergweg 2 seat address in the same page is NOT used); phone
//! comes from the `tel:` link text; the E-Mail is JS-obfuscated with
//! random `…@nospam` placeholders in the static HTML (both href and text),
//! so `email` stays empty on purpose — never a placeholder address.
//! Missing anchors fail loudly (redesign → eyeballs, no guessing).

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-herdecke-eas-recycling-solution";
/// Bespoke, live-verified price pages (all HTTP 200 on 28.09.2026).
/// A move fails the step loudly (fix the URLs) — never guessed.
pub const URL_BAUTEILE: &str = "https://www.eas-recycling.de/ankaufspreise/elektronische-bauteile/";
pub const URL_HARDWARE: &str = "https://www.eas-recycling.de/ankaufspreise/elektronische-hardware/";
pub const URL_ALTMETALLE: &str = "https://www.eas-recycling.de/ankaufspreise/altmetalle/";
/// Bespoke, live-verified impressum URL (HTTP 200 on 28.09.2026).
pub const IMPRESSUM_URL: &str = "https://www.eas-recycling.de/impressum/";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL_BAUTEILE,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let pages = [
        (URL_BAUTEILE, fetch_text(client, URL_BAUTEILE).await?),
        (URL_HARDWARE, fetch_text(client, URL_HARDWARE).await?),
        (URL_ALTMETALLE, fetch_text(client, URL_ALTMETALLE).await?),
    ];
    let status = pages[0].1.0;
    let mut byte_len = 0;
    // (label, price, min, max, unit) across the three pages.
    let mut rows: Vec<(String, f64, Option<f64>, Option<f64>, &'static str)> = Vec::new();
    let mut skipped_labels: Vec<String> = Vec::new();
    for (url, (_, html)) in &pages {
        byte_len += html.len();
        let (page_rows, page_skips) = parse_blocks(html, url)?;
        rows.extend(page_rows);
        skipped_labels.extend(page_skips);
    }
    let mut prices = Vec::with_capacity(rows.len());
    // Doppelblock-Guard: same (material, variant, price) only once.
    let mut seen = std::collections::HashSet::new();
    for (label, price, min, max, unit) in rows {
        let Some((material, variant)) = grade_for(&label) else {
            skipped_labels.push(label);
            continue;
        };
        if !seen.insert((material, variant, price.to_bits())) {
            continue;
        }
        // Von-bis ranges mirror the upto pattern: honest bounds with
        // their own kind, never a silent exact (VANA-Muster).
        let (price_kind, confidence, price_min, price_max) = match (min, max) {
            (Some(lo), Some(hi)) if (hi - lo).abs() > f64::EPSILON => {
                ("range", Some(0.5), Some(lo), Some(hi))
            }
            _ => ("exact", Some(1.0), None, None),
        };
        prices.push(ScrapedPrice {
            material,
            variant,
            price,
            currency: "EUR",
            unit,
            price_kind,
            price_min,
            price_max,
            confidence,
            label,
        });
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
        fetch_url: URL_BAUTEILE.to_owned(),
        status_code: status,
        byte_len,
        published_at: None,
    })
}

/// Parse one EAS price page: every `div.ym-g70 > div.ym-gbox` block holds
/// a title `<p><strong>Label</strong>…</p>` plus price `<p>`s carrying `€`.
/// Returns (label, price, min, max, unit); `price` is the max (== price
/// for exact rows). Zero parsed rows mean a redesign → loud error.
fn parse_blocks(
    html: &str,
    url: &str,
) -> Result<
    (
        Vec<(String, f64, Option<f64>, Option<f64>, &'static str)>,
        Vec<String>,
    ),
    IngestError,
> {
    let doc = Html::parse_document(html);
    let gbox = Selector::parse("div.ym-g70 div.ym-gbox").expect("valid selector");
    let para = Selector::parse("p").expect("valid selector");
    let strong = Selector::parse("strong").expect("valid selector");
    let boxes: Vec<ElementRef> = doc.select(&gbox).collect();
    if boxes.is_empty() {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: "keine Preisblöcke (ym-g70/ym-gbox)".to_owned(),
        });
    }
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for b in boxes {
        let ps: Vec<ElementRef> = b.select(&para).collect();
        if ps.is_empty() {
            continue;
        }
        let title = ps[0]
            .select(&strong)
            .next()
            .map(|s| norm(&s.text().collect::<String>()))
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| "[Block ohne Titel]".to_owned());
        let mut block_rows = 0;
        for p in &ps {
            let t = norm(&p.text().collect::<String>());
            // "Preis auf Anfrage!" carries no € — check before the price gate.
            if t.to_lowercase().contains("auf anfrage") {
                skips.push(format!("{title} (Preis auf Anfrage)"));
                continue;
            }
            if !t.contains('€') {
                continue;
            }
            let (price, min, max) = match split_range(&t) {
                Some((lo, hi)) => (hi, Some(lo), Some(hi)),
                None => {
                    let Some(v) = parse_eur(&t) else {
                        skips.push(format!("{title} (kein Preis: {t})"));
                        continue;
                    };
                    (v, None, None)
                }
            };
            // A "0,00" row is "no quote", not a free gift: loud skip.
            if price == 0.0 {
                skips.push(format!("{title} (Preis 0,00)"));
                continue;
            }
            // An unparseable unit is a loud skip, never a silent default.
            let Some(unit) = unit_of(&t) else {
                skips.push(format!("{title} (Einheit unverständlich: {t})"));
                continue;
            };
            // Own label only when the price paragraph names one beyond
            // price + unit ("… mit Aluminium 25,00€ …"); a bare "(Beraubt)"
            // parenthesis belongs to the block title instead.
            let label = inline_label(p, &strong).unwrap_or_else(|| title.clone());
            rows.push((label, price, min, max, unit));
            block_rows += 1;
        }
        // A block with € but no usable row already logged its skips above;
        // nothing further to do here.
        let _ = block_rows;
    }
    if rows.is_empty() && skips.is_empty() {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: "Preisblöcke ohne Preise".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// "4,00 € - 40,00 € / Kilogramm" → (4.0, 40.0); anything else → None
/// (single prices go through `parse_eur` at the call site).
fn split_range(t: &str) -> Option<(f64, f64)> {
    let (a, b) = t.split_once('-')?;
    if !b.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }
    let lo = parse_eur(a)?;
    let hi = parse_eur(b)?;
    if lo <= 0.0 || hi <= 0.0 || hi < lo {
        return None;
    }
    Some((lo, hi))
}

/// Bespoke unit matcher for THESE pages (live: "€ / Kilogramm",
/// once "€ / KG"). Only kg exists here — anything else skips loudly.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("kilogramm") || lower.contains("kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

/// Inline label of a price paragraph: its `<strong>` text minus price and
/// unit tokens. Parentheses-only remainders ("(Beraubt)") are not labels.
/// Returns None when the block title owns the row.
fn inline_label(p: &ElementRef, strong: &Selector) -> Option<String> {
    let s: String = p
        .select(strong)
        .map(|e| e.text().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ");
    let mut words: Vec<&str> = Vec::new();
    let cleaned = norm(&s);
    for w in cleaned.split_whitespace() {
        let wl = w.to_lowercase();
        if w.chars().any(|c| c.is_ascii_digit()) || w.contains('€') || w == "/" {
            continue;
        }
        if wl == "kilogramm" || wl == "kg" || wl == "pro" {
            continue;
        }
        words.push(w);
    }
    let cand = words.join(" ");
    // Strip "(…)" groups: a remainder of only "(Beraubt)" means the real
    // label is the block title ("Laptop-Schrott (Beraubt)").
    let mut bare = cand.clone();
    while let Some(a) = bare.find('(') {
        if let Some(b) = bare[a..].find(')') {
            bare.replace_range(a..a + b + 1, " ");
        } else {
            break;
        }
    }
    if bare.split_whitespace().count() >= 2 {
        Some(cand.trim().to_owned())
    } else {
        None
    }
}

/// Explicit label → (material, variant) mapping, specific-before-generic:
/// every platinen phrase precedes the no-catalog skip arms ("Festplatten
/// Platinen" vs. whole "Festplatten", "Handy-Leiterplatten" vs. "Handy´s",
/// "Steckkarten mit Slotblende" vs. "Slot Prozessoren"). Anything unlisted
/// returns None (loud skip at the call site). The variant keeps the
/// trader's own grade wording so price grades never collapse.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Platinen family — the only E-Schrott with catalog material.
    if l.contains("festplatten platinen") || l.contains("festplatte platine") {
        Some(("festplatten", "Festplatte"))
    } else if l.contains("laufwerk platinen") || l.contains("laufwerk-platine") {
        Some(("platinen", "Laufwerk"))
    } else if l.contains("handy") && (l.contains("platine") || l.contains("platte")) {
        Some(("handys", "Handy"))
    } else if l.contains("steckkarte") {
        if l.contains("slotblende") {
            Some(("platinen", "Steckkarte mit Slotblende"))
        } else if l.contains("ohne") {
            Some(("platinen", "Steckkarte ohne Anhaftungen"))
        } else {
            Some(("platinen", "Steckkarte mit Anhaftungen"))
        }
    } else if l.contains("rückw") || l.contains("rueckw") {
        Some(("platinen", "Rückwände"))
    } else if l.contains("leiterplatte") || l.contains("leiterplatine") {
        // "1A ++" contains "1A +" as a substring — longest first.
        if l.contains("1a ++") {
            Some(("platinen", "Klasse 1A ++"))
        } else if l.contains("1a +") {
            Some(("platinen", "Klasse 1A +"))
        } else if l.contains("alt") {
            Some(("platinen", "Klasse 1A alt"))
        } else if l.contains("1b+") {
            Some(("platinen", "Klasse 1B+ neu"))
        } else if l.contains("1b-") {
            Some(("platinen", "Klasse 1B- neu"))
        } else if l.contains("laptop") {
            Some(("platinen", "Laptop"))
        } else if l.contains("klasse 2a") {
            Some(("platinen", "Klasse 2A"))
        } else if l.contains("klasse 2b") {
            Some(("platinen", "Klasse 2B"))
        } else if l.contains("klasse 3") {
            Some(("platinen", "Klasse 3"))
        } else {
            // Unknown Leiterplatten grade: loud skip, never a silent
            // '' collapse onto another grade's current price.
            None
        }
    // Catalog metals (altmetalle page).
    } else if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("raff") {
        Some(("kupfer-gemischt", "Raff"))
    } else if l.contains("zinngeschirr") {
        Some(("zinn", "Geschirr 85-98%"))
    } else if l.contains("zinn") {
        Some(("zinn", "99%"))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("pc-netzteil kabel") {
        if l.contains("ohne") {
            Some(("kabel-kupfer", "ohne Stecker"))
        } else {
            Some(("kabel-kupfer", "mit Stecker"))
        }
    } else if l.contains("netzteil") {
        // Whole PSUs ("Netzteile mit/ohne Kabel", "Externe Netzteile"),
        // not cable scrap — no catalog material.
        None
    } else if l.contains("kabel") {
        if l.contains("ide") {
            Some(("kabel-kupfer", "IDE"))
        } else if l.contains("ohne") {
            Some(("kabel-kupfer", "ohne Stecker"))
        } else if l.contains("mit") {
            Some(("kabel-kupfer", "mit Stecker"))
        } else {
            Some(("kabel-kupfer", ""))
        }
    } else if l.contains("aluminium profile") {
        Some(("aluminium-profile", ""))
    } else if l.contains("aluminium gemischt") {
        Some(("aluminium-gemischt", ""))
    } else {
        // No catalog material (Frisch precedent — proposals, not guesses):
        // - CPUs ("Slot/Kunststoff/Keramik Prozessoren", "iCore") → `cpu`
        // - Arbeitsspeicher Gold-/Silberkante (mit/ohne Alu) → `ram`
        // - ICs/Eprom → chip material; Handys/Smartphones, Computer/
        //   Laptops/Server, Netzteile, Laufwerke, whole Festplatten,
        //   IDE-/Kabel/Stecker, Ablenkeinheiten, Tastaturen, Drucker →
        //   `e-schrott-geraete`; versilbertes Besteck/Messer is not
        //   `silber` (vedder precedent); Alu-Cu-Kühler is a mixed alloy.
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` holding
/// "Betriebsstätte:" lists firm + street + PLZ city over `<br>` lines
/// (the Wienbergweg seat address elsewhere on the page is NOT used);
/// the phone is the `tel:` link text under the "Kontakt:" heading. The
/// E-Mail is JS-obfuscated (`…@nospam` random placeholders in static
/// HTML), so `email` stays empty — never a placeholder. Missing anchors
/// mean the page changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let para = Selector::parse("p").expect("valid selector");
    let link = Selector::parse("a").expect("valid selector");
    let addr_p = doc.select(&para).find(|p| {
        p.text()
            .collect::<String>()
            .contains("Betriebsstätte")
    });
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Betriebsstätte-Block fehlt".to_owned(),
        });
    };
    let mut lines = Vec::new();
    for part in addr_p.inner_html().split("<br") {
        let t = strip_tags(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    // "… / Ostender Weg 12A / 58313 Herdecke" (last two lines).
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if lines.len() >= 2 {
        let last = lines.last().expect("len checked");
        let mut it = last.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it
                    .fold(ci.to_owned(), |a, w| a + " " + w)
                    .trim()
                    .to_owned();
                street = lines[lines.len() - 2].clone();
            }
        }
    }
    // Phone: visible text of the tel: link (contact paragraph).
    let mut phone = String::new();
    for a in doc.select(&link) {
        if let Some(href) = a.value().attr("href") {
            if href.starts_with("tel:") {
                let t: String = a.text().collect();
                let t = norm(&t);
                if !t.is_empty() {
                    phone = t;
                    break;
                }
            }
        }
    }
    if street.is_empty() && phone.is_empty() {
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
        email: String::new(),
    })
}

/// Strip tags from a fragment (entities are already decoded by html5ever).
fn strip_tags(s: &str) -> String {
    // Drop the tag remnant before the first '>' (`<br />` splits leave
    // `/>` heads that would parse as text, e.g. `class="…"`).
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
    norm(&out)
}

fn norm(s: &str) -> String {
    s.replace(['\u{a0}'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, inline_label, parse_blocks, split_range, unit_of};
    use scraper::{Html, Selector};

    // Real excerpts of the live pages (28.09.2026), entities verbatim.
    const BAUTEILE_EXCERPT: &str = "\
        <div class=\"ym-grid equal-grid linearize-level-2\">\
        <div class=\"ym-g70 ym-gl\"><div class=\"ym-gbox\">\
        <p><strong>Leiterplatten Klasse 1A ++</strong></p>\
        <p>Leiterplatten aus Gro&szlig;rechneranlagen mit hoher IC Best&uuml;ckung. Mit integriertem CPU der sichtbar Gold enth&auml;lt.</p>\
        <p><strong>28,50 &euro; / Kilogramm</strong></p>\
        </div></div></div>\
        <div class=\"ym-grid equal-grid linearize-level-2\">\
        <div class=\"ym-g70 ym-gl\"><div class=\"ym-gbox\">\
        <p><strong>Arbeitsspeicher Goldkante</strong><br />Ram-Module aus Computern. Ohne Aluminium und Eisenanhaftung.</p>\
        <p><strong>85,00 &euro; / Kilogramm</strong></p>\
        <p><strong>Arbeitsspeicher Goldkante mit Aluminium 25,00&euro; / Kilogramm</strong></p>\
        </div></div></div>\
        <div class=\"ym-grid equal-grid linearize-level-2\">\
        <div class=\"ym-g70 ym-gl\"><div class=\"ym-gbox\">\
        <p><strong>R&uuml;ckw&auml;nde</strong><br />Leiterplatten R&uuml;ckw&auml;nde aus Gro&szlig;rechneranlagen.</p>\
        <p><strong>4,00 &euro; - 40,00 &euro; / Kilogramm</strong></p>\
        </div></div></div>\
        <div class=\"ym-grid equal-grid linearize-level-2\">\
        <div class=\"ym-g70 ym-gl\"><div class=\"ym-gbox\">\
        <p><strong>Prozessoren iCore Serie</strong><br />Prozessoren ab der dritten Generation ohne Besch&auml;digung.</p>\
        <p><strong>Preis auf Anfrage!</strong></p>\
        </div></div></div>";

    const HARDWARE_EXCERPT: &str = "\
        <div class=\"ym-grid equal-grid linearize-level-2\">\
        <div class=\"ym-g70 ym-gl\"><div class=\"ym-gbox\">\
        <p><strong>Computer (Unberaubt)</strong><br />Gebrauchte unberaubte Computer.</p>\
        <p><strong>1,20 &euro; / Kilogramm</strong></p>\
        </div></div></div>\
        <div class=\"ym-grid equal-grid linearize-level-2\">\
        <div class=\"ym-g70 ym-gl\"><div class=\"ym-gbox\">\
        <p><strong>Laptop-Schrott (Beraubt)</strong><br />Gebrauchte Laptops beraubt ohne Akku.</p>\
        <p><strong>1,00 &euro; / KG (Beraubt)</strong></p>\
        </div></div></div>\
        <div class=\"ym-grid equal-grid linearize-level-2\">\
        <div class=\"ym-g70 ym-gl\"><div class=\"ym-gbox\">\
        <p><strong>Tastaturen</strong><br />Das Kabel darf nicht abgetrennt sein.</p>\
        <p><strong>0,00 &euro; / Kilogramm</strong></p>\
        </div></div></div>";

    const ALTMETALLE_EXCERPT: &str = "\
        <div class=\"ym-grid equal-grid linearize-level-2\">\
        <div class=\"ym-g70 ym-gl\"><div class=\"ym-gbox\">\
        <p><strong>Kupfer Millberry</strong></p>\
        <p><strong>7,40 &euro; / Kilogramm</strong></p>\
        </div></div></div>\
        <div class=\"ym-grid equal-grid linearize-level-2\">\
        <div class=\"ym-g70 ym-gl\"><div class=\"ym-gbox\">\
        <p><strong>Mischschrott</strong></p>\
        <p><strong>0,00 &euro; / Kilogramm</strong></p>\
        </div></div></div>\
        <div class=\"ym-grid equal-grid linearize-level-2\">\
        <div class=\"ym-g70 ym-gl\"><div class=\"ym-gbox\">\
        <p><strong>Besteck ab 90er Auflage</strong></p>\
        <p><strong>30,00 &euro; / Kilogramm</strong></p>\
        </div></div></div>";

    #[test]
    fn blocks_prices_range_and_skips_parse() {
        let (rows, skips) = parse_blocks(BAUTEILE_EXCERPT, "u").expect("parses");
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].0, "Leiterplatten Klasse 1A ++");
        assert_eq!(rows[0].1, 28.5);
        assert_eq!(rows[0].4, "EUR/kg");
        assert_eq!(rows[1].0, "Arbeitsspeicher Goldkante");
        assert_eq!(rows[1].1, 85.0);
        // Second price paragraph carries its own inline label.
        assert_eq!(rows[2].0, "Arbeitsspeicher Goldkante mit Aluminium");
        assert_eq!(rows[2].1, 25.0);
        // Range: price = max, bounds attached.
        assert_eq!(rows[3].0, "Rückwände");
        assert_eq!(rows[3].1, 40.0);
        assert_eq!(rows[3].2, Some(4.0));
        assert_eq!(rows[3].3, Some(40.0));
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("iCore") && skips[0].contains("Anfrage"));
    }

    #[test]
    fn hardware_parses_with_zero_skip_and_paren_title() {
        let (rows, skips) = parse_blocks(HARDWARE_EXCERPT, "u").expect("parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "Computer (Unberaubt)");
        assert_eq!(rows[0].1, 1.2);
        // "(Beraubt)" alone is no label — the block title owns the row.
        assert_eq!(rows[1].0, "Laptop-Schrott (Beraubt)");
        assert_eq!(rows[1].1, 1.0);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Tastaturen") && skips[0].contains("0,00"));
    }

    #[test]
    fn altmetalle_parses_with_zero_skip() {
        let (rows, skips) = parse_blocks(ALTMETALLE_EXCERPT, "u").expect("parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "Kupfer Millberry");
        assert_eq!(rows[0].1, 7.4);
        assert_eq!(rows[1].0, "Besteck ab 90er Auflage");
        assert_eq!(rows[1].1, 30.0);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Mischschrott") && skips[0].contains("0,00"));
    }

    #[test]
    fn redesign_and_unit_fail_loudly() {
        let err = parse_blocks("<html><body><p>Neu hier</p></body></html>", "u")
            .expect_err("no boxes errors");
        assert!(err.to_string().contains("Preisblöcke"));
        // Unknown unit: skipped loudly, valid rows survive.
        let html = BAUTEILE_EXCERPT.replacen("Kilogramm", "Sack", 1);
        let (rows, skips) = parse_blocks(&html, "u").expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!(skips.len(), 2);
        assert!(skips[0].contains("Einheit"));
        assert!(unit_of("1,00 € / KG (Beraubt)").is_some());
        assert!(unit_of("5 € pro Sack").is_none());
        assert_eq!(split_range("4,00 € - 40,00 € / Kilogramm"), Some((4.0, 40.0)));
        assert_eq!(split_range("28,50 € / Kilogramm"), None);
    }

    #[test]
    fn inline_label_rules() {
        let doc = Html::parse_document(
            "<p><strong>Arbeitsspeicher Goldkante mit Aluminium 25,00€ / Kilogramm</strong></p>",
        );
        let sel = Selector::parse("p").expect("valid selector");
        let strong = Selector::parse("strong").expect("valid selector");
        let p = doc.select(&sel).next().expect("p");
        assert_eq!(
            inline_label(&p, &strong).as_deref(),
            Some("Arbeitsspeicher Goldkante mit Aluminium")
        );
        let doc = Html::parse_document("<p><strong>1,00 € / KG (Beraubt)</strong></p>");
        let p = doc.select(&sel).next().expect("p");
        assert_eq!(inline_label(&p, &strong), None);
    }

    #[test]
    fn impressum_extracts_address_and_tel_without_email() {
        // Real fragment shape: Betriebsstätte <p>, tel: link, JS-obfuscated
        // @nospam mailto placeholders (random tokens, no real address).
        let imp = "<h1>Impressum</h1>\
            <p>EAS-Recycling Solution GmbH<br />Betriebsst&auml;tte: <br />Ostender Weg 12A <br />58313 Herdecke</p>\
            <p><strong>Unternehmenssitz:</strong> <br />Wienbergweg 2 <br />58313 Herdecke</p>\
            <h3>Kontakt:</h3><p>Telefon: <a href=\"tel:+4923309107023\">+49 (0) 2330 / 910 7023</a> \
            <br />E-Mail: <a href=\"mailto:yqOkrKWKr6u557ivqbOppqOkreSurw@nospam\">SiMkLCUKLys5ZzgvKTMpJiMkLWQuLw@nospam</a></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Ostender Weg 12A");
        assert_eq!(info.postcode, "58313");
        assert_eq!(info.city, "Herdecke");
        assert_eq!(info.phone, "+49 (0) 2330 / 910 7023");
        assert!(info.email.is_empty());
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }

    #[test]
    fn mapping_covers_live_labels() {
        // Platinen family → platinen with trader grade.
        assert_eq!(
            grade_for("Leiterplatten Klasse 1A ++"),
            Some(("platinen", "Klasse 1A ++"))
        );
        assert_eq!(
            grade_for("Leiterplatten Klasse 1A +"),
            Some(("platinen", "Klasse 1A +"))
        );
        assert_eq!(
            grade_for("Leiterplatten Klasse 1A (alte Generation)"),
            Some(("platinen", "Klasse 1A alt"))
        );
        assert_eq!(
            grade_for("Leiterplatten Klasse 1B+ (neue Generation)"),
            Some(("platinen", "Klasse 1B+ neu"))
        );
        assert_eq!(
            grade_for("Leiterplatten Klasse 1B- (neue Generation)"),
            Some(("platinen", "Klasse 1B- neu"))
        );
        assert_eq!(
            grade_for("Computer und Server Steckkarten ohne Anhaftungen"),
            Some(("platinen", "Steckkarte ohne Anhaftungen"))
        );
        assert_eq!(
            grade_for("Computer und Server Steckkarten mit Slotblende"),
            Some(("platinen", "Steckkarte mit Slotblende"))
        );
        assert_eq!(
            grade_for("Computer und Server Steckkarten mit Anhaftungen"),
            Some(("platinen", "Steckkarte mit Anhaftungen"))
        );
        assert_eq!(
            grade_for("Laptop Leiterplatten"),
            Some(("platinen", "Laptop"))
        );
        assert_eq!(
            grade_for("Rückwände"),
            Some(("platinen", "Rückwände"))
        );
        assert_eq!(
            grade_for("Leiterplatten Klasse 2A"),
            Some(("platinen", "Klasse 2A"))
        );
        assert_eq!(
            grade_for("Leiterplatten Klasse 2B"),
            Some(("platinen", "Klasse 2B"))
        );
        assert_eq!(
            grade_for("Leiterplatten Klasse 3"),
            Some(("platinen", "Klasse 3"))
        );
        assert_eq!(
            grade_for("Festplatten Platinen"),
            Some(("festplatten", "Festplatte"))
        );
        assert_eq!(
            grade_for("Laufwerk Platinen"),
            Some(("platinen", "Laufwerk"))
        );
        assert_eq!(
            grade_for("Handy-Leiterplatten"),
            Some(("handys", "Handy"))
        );
        // No catalog material → loud skip (Frisch precedent).
        assert_eq!(grade_for("Arbeitsspeicher Goldkante"), None);
        assert_eq!(
            grade_for("Arbeitsspeicher Goldkante mit Aluminium"),
            None
        );
        assert_eq!(grade_for("Arbeitsspeicher Silberkante"), None);
        assert_eq!(grade_for("Slot Prozessoren"), None);
        assert_eq!(
            grade_for("Kunststoffprozessoren mit Kupferkühler"),
            None
        );
        assert_eq!(grade_for("Kunststoffprozessoren Schwarz"), None);
        assert_eq!(
            grade_for("Keramikprozessoren mit Aluminiumkühler"),
            None
        );
        assert_eq!(
            grade_for("Keramikprozessoren Pentium und AMD"),
            None
        );
        assert_eq!(grade_for("Keramikprozessoren Goldcap"), None);
        assert_eq!(grade_for("Keramik und Kunststoff ICs / Eprom"), None);
        assert_eq!(grade_for("Prozessoren iCore Serie"), None);
        assert_eq!(grade_for("Handy´s"), None);
        assert_eq!(grade_for("Smartphone 8,00 € / Kilogramm"), None);
        assert_eq!(grade_for("Computer (Unberaubt)"), None);
        assert_eq!(grade_for("Laptop-Schrott (Unberaubt)"), None);
        assert_eq!(grade_for("Servereinschübe und Switches"), None);
        assert_eq!(grade_for("Netzteile mit Kabel"), None);
        assert_eq!(grade_for("Netzteile ohne Kabel"), None);
        assert_eq!(grade_for("Externe Netzteile mit Kabel"), None);
        assert_eq!(grade_for("Computer und Server Laufwerke"), None);
        assert_eq!(grade_for("Festplatten"), None);
        assert_eq!(grade_for("IDE-Kabel"), Some(("kabel-kupfer", "IDE")));
        assert_eq!(grade_for("Computer Stecker"), None);
        assert_eq!(grade_for("Ablenkeinheiten"), None);
        assert_eq!(grade_for("Tastaturen"), None);
        assert_eq!(grade_for("Drucker"), None);
        // Catalog metals map normally.
        assert_eq!(
            grade_for("Kupfer Millberry"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Kupfer Raff"),
            Some(("kupfer-gemischt", "Raff"))
        );
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Zinn 99%"), Some(("zinn", "99%")));
        assert_eq!(
            grade_for("Zinngeschirr 85% - 98%"),
            Some(("zinn", "Geschirr 85-98%"))
        );
        assert_eq!(
            grade_for("Kupferkabel"),
            Some(("kabel-kupfer", ""))
        );
        assert_eq!(
            grade_for("PC-Netzteil Kabel mit Stecker"),
            Some(("kabel-kupfer", "mit Stecker"))
        );
        assert_eq!(
            grade_for("PC-Netzteil Kabel ohne Stecker"),
            Some(("kabel-kupfer", "ohne Stecker"))
        );
        assert_eq!(
            grade_for("Aluminium Profile"),
            Some(("aluminium-profile", ""))
        );
        assert_eq!(
            grade_for("Aluminium gemischt max 5% Fremdstoffe."),
            Some(("aluminium-gemischt", ""))
        );
        // Mixed alloy + silver plate → loud skip.
        assert_eq!(grade_for("Aluminium Kupfer Kühler"), None);
        assert_eq!(grade_for("Besteck ab 90er Auflage"), None);
        assert_eq!(grade_for("Messer ab 90er Auflage"), None);
        assert_eq!(grade_for("Mischschrott"), None);
    }
}
