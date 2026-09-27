//! NEUWERT Schrott & Altmetallhandel OHG (Braunschweig): exact weekly
//! list prices in two Elementor shapes inside one window
//! ("Altmetall-Preisliste für diese Woche" … "Häufig gestellte Fragen"):
//! headline cards (`h2.elementor-heading-title` + three `ab … kg` tier
//! spans) and swiper slides (`elementor-slide-heading` + `<li>` tiers).
//! Every material quotes three quantity tiers (ab 1000/100/1 kg, steel
//! cards ab 1000/500/1 kg) per kg — the tier rides in the variant, so no
//! two sorts collapse. The pipeline normalizes EUR/kg into the iron
//! catalog units (EUR/t) itself; this handler always records the honest
//! page unit. No page date ("für diese Woche" only) → `published_at`
//! stays `None`. Ambiguous grades (Erdkabel mit Stahl, Alu-Kupfer-Kühler,
//! Alu-Leitung mit Stahl, Sorte 3) skip loudly instead of being crammed.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "ni-braunschweig-neuwert-schrott-altmetallhandel";
/// Bespoke, live-verified impressum URL: the site nav labels
/// `https://neu-wert.de/privacy-policy/` as "Impressum". A move fails
/// the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://neu-wert.de/privacy-policy/";

pub const URL: &str = "https://neu-wert.de/unsere-aktuelle-preisliste-altmetalle/";

/// Category headline cards carry no tiers and are ignored (never labels).
const START_ANCHOR: &str = "Altmetall-Preisliste für diese Woche";
const END_ANCHOR: &str = "Häufig gestellte Fragen";
const H_MARKER: &str = "<h2 class=\"elementor-heading-title elementor-size-default\">";
const TIER_MARKER: &str = "elementor-icon-list-text\">";
const SLIDE_HEAD_MARKER: &str = "elementor-slide-heading\">";
const SLIDE_DESC_MARKER: &str = "elementor-slide-description\">";

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
        match grade_for(&label) {
            Some((material, grade)) => match tier_variant(grade, tier) {
                Some(variant) => prices.push(ScrapedPrice {
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
                None => {
                    skipped_labels.push(format!("{label} (Staffel unbekannt: {})", tier_str(tier)))
                }
            },
            None => skipped_labels.push(format!(
                "{label} [{}] ({})",
                tier_str(tier),
                skip_note(&label)
            )),
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

/// Tier glued to the grade: two grades on one material stay apart AND two
/// tiers of one grade never collapse onto one current price. Unknown
/// combinations (site added a tier) return `None` → loud skip, never a
/// made-up variant.
fn tier_variant(grade: &'static str, tier: Tier) -> Option<&'static str> {
    match (grade, tier) {
        ("Millberry", Tier::T1000) => Some("Millberry / ab 1000 kg"),
        ("Millberry", Tier::T100) => Some("Millberry / ab 100 kg"),
        ("Millberry", Tier::T1) => Some("Millberry / ab 1 kg"),
        ("Candy", Tier::T1000) => Some("Candy / ab 1000 kg"),
        ("Candy", Tier::T100) => Some("Candy / ab 100 kg"),
        ("Candy", Tier::T1) => Some("Candy / ab 1 kg"),
        ("Kerze", Tier::T1000) => Some("Kerze / ab 1000 kg"),
        ("Kerze", Tier::T100) => Some("Kerze / ab 100 kg"),
        ("Kerze", Tier::T1) => Some("Kerze / ab 1 kg"),
        ("verzinnt", Tier::T1000) => Some("verzinnt / ab 1000 kg"),
        ("verzinnt", Tier::T100) => Some("verzinnt / ab 100 kg"),
        ("verzinnt", Tier::T1) => Some("verzinnt / ab 1 kg"),
        ("Berry", Tier::T1000) => Some("Berry / ab 1000 kg"),
        ("Berry", Tier::T100) => Some("Berry / ab 100 kg"),
        ("Berry", Tier::T1) => Some("Berry / ab 1 kg"),
        ("schwer", Tier::T1000) => Some("schwer / ab 1000 kg"),
        ("schwer", Tier::T100) => Some("schwer / ab 100 kg"),
        ("schwer", Tier::T1) => Some("schwer / ab 1 kg"),
        ("leicht", Tier::T1000) => Some("leicht / ab 1000 kg"),
        ("leicht", Tier::T100) => Some("leicht / ab 100 kg"),
        ("leicht", Tier::T1) => Some("leicht / ab 1 kg"),
        ("Späne", Tier::T1000) => Some("Späne / ab 1000 kg"),
        ("Späne", Tier::T100) => Some("Späne / ab 100 kg"),
        ("Späne", Tier::T1) => Some("Späne / ab 1 kg"),
        ("WiCu-Rohre", Tier::T1000) => Some("WiCu-Rohre / ab 1000 kg"),
        ("WiCu-Rohre", Tier::T100) => Some("WiCu-Rohre / ab 100 kg"),
        ("WiCu-Rohre", Tier::T1) => Some("WiCu-Rohre / ab 1 kg"),
        ("Ms-58", Tier::T1000) => Some("Ms-58 / ab 1000 kg"),
        ("Ms-58", Tier::T100) => Some("Ms-58 / ab 100 kg"),
        ("Ms-58", Tier::T1) => Some("Ms-58 / ab 1 kg"),
        ("Ms-58 Späne", Tier::T1000) => Some("Ms-58 Späne / ab 1000 kg"),
        ("Ms-58 Späne", Tier::T100) => Some("Ms-58 Späne / ab 100 kg"),
        ("Ms-58 Späne", Tier::T1) => Some("Ms-58 Späne / ab 1 kg"),
        ("Hülsen", Tier::T1000) => Some("Hülsen / ab 1000 kg"),
        ("Hülsen", Tier::T100) => Some("Hülsen / ab 100 kg"),
        ("Hülsen", Tier::T1) => Some("Hülsen / ab 1 kg"),
        ("75%", Tier::T1000) => Some("75% / ab 1000 kg"),
        ("75%", Tier::T100) => Some("75% / ab 100 kg"),
        ("75%", Tier::T1) => Some("75% / ab 1 kg"),
        ("60%", Tier::T100) => Some("60% / ab 100 kg"),
        ("60%", Tier::T1000) => Some("60% / ab 1000 kg"),
        ("60%", Tier::T1) => Some("60% / ab 1 kg"),
        ("50%", Tier::T1000) => Some("50% / ab 1000 kg"),
        ("50%", Tier::T100) => Some("50% / ab 100 kg"),
        ("50%", Tier::T1) => Some("50% / ab 1 kg"),
        ("38%", Tier::T1000) => Some("38% / ab 1000 kg"),
        ("38%", Tier::T100) => Some("38% / ab 100 kg"),
        ("38%", Tier::T1) => Some("38% / ab 1 kg"),
        ("Litzen 75%", Tier::T1000) => Some("Litzen 75% / ab 1000 kg"),
        ("Litzen 75%", Tier::T100) => Some("Litzen 75% / ab 100 kg"),
        ("Litzen 75%", Tier::T1) => Some("Litzen 75% / ab 1 kg"),
        ("Litzen 60%", Tier::T1000) => Some("Litzen 60% / ab 1000 kg"),
        ("Litzen 60%", Tier::T100) => Some("Litzen 60% / ab 100 kg"),
        ("Litzen 60%", Tier::T1) => Some("Litzen 60% / ab 1 kg"),
        ("mit Stecker", Tier::T1000) => Some("mit Stecker / ab 1000 kg"),
        ("mit Stecker", Tier::T100) => Some("mit Stecker / ab 100 kg"),
        ("mit Stecker", Tier::T1) => Some("mit Stecker / ab 1 kg"),
        ("Farbe", Tier::T1000) => Some("Farbe / ab 1000 kg"),
        ("Farbe", Tier::T100) => Some("Farbe / ab 100 kg"),
        ("Farbe", Tier::T1) => Some("Farbe / ab 1 kg"),
        ("Offset", Tier::T1000) => Some("Offset / ab 1000 kg"),
        ("Offset", Tier::T100) => Some("Offset / ab 100 kg"),
        ("Offset", Tier::T1) => Some("Offset / ab 1 kg"),
        ("Konstruktal", Tier::T1000) => Some("Konstruktal / ab 1000 kg"),
        ("Konstruktal", Tier::T100) => Some("Konstruktal / ab 100 kg"),
        ("Konstruktal", Tier::T1) => Some("Konstruktal / ab 1 kg"),
        ("Felgen", Tier::T1000) => Some("Felgen / ab 1000 kg"),
        ("Felgen", Tier::T100) => Some("Felgen / ab 100 kg"),
        ("Felgen", Tier::T1) => Some("Felgen / ab 1 kg"),
        ("unsauber", Tier::T1000) => Some("unsauber / ab 1000 kg"),
        ("unsauber", Tier::T100) => Some("unsauber / ab 100 kg"),
        ("unsauber", Tier::T1) => Some("unsauber / ab 1 kg"),
        ("Geschirr", Tier::T1000) => Some("Geschirr / ab 1000 kg"),
        ("Geschirr", Tier::T100) => Some("Geschirr / ab 100 kg"),
        ("Geschirr", Tier::T1) => Some("Geschirr / ab 1 kg"),
        ("Bremsscheiben", Tier::T1000) => Some("Bremsscheiben / ab 1000 kg"),
        ("Bremsscheiben", Tier::T500) => Some("Bremsscheiben / ab 500 kg"),
        ("Bremsscheiben", Tier::T1) => Some("Bremsscheiben / ab 1 kg"),
        ("V4A", Tier::T1000) => Some("V4A / ab 1000 kg"),
        ("V4A", Tier::T100) => Some("V4A / ab 100 kg"),
        ("V4A", Tier::T1) => Some("V4A / ab 1 kg"),
        ("Schwerschrott", Tier::T1000) => Some("Schwerschrott / ab 1000 kg"),
        ("Schwerschrott", Tier::T500) => Some("Schwerschrott / ab 500 kg"),
        ("Schwerschrott", Tier::T1) => Some("Schwerschrott / ab 1 kg"),
        ("Brennerschrott", Tier::T1000) => Some("Brennerschrott / ab 1000 kg"),
        ("Brennerschrott", Tier::T100) => Some("Brennerschrott / ab 100 kg"),
        ("Brennerschrott", Tier::T1) => Some("Brennerschrott / ab 1 kg"),
        ("Stahlspäne", Tier::T1000) => Some("Stahlspäne / ab 1000 kg"),
        ("Stahlspäne", Tier::T100) => Some("Stahlspäne / ab 100 kg"),
        ("Stahlspäne", Tier::T1) => Some("Stahlspäne / ab 1 kg"),
        ("", Tier::T1000) => Some("ab 1000 kg"),
        ("", Tier::T100) => Some("ab 100 kg"),
        ("", Tier::T500) => Some("ab 500 kg"),
        ("", Tier::T1) => Some("ab 1 kg"),
        _ => None,
    }
}

/// Tier text for skip messages and labels.
fn tier_str(tier: Tier) -> &'static str {
    match tier {
        Tier::T1000 => "ab 1000 kg",
        Tier::T100 => "ab 100 kg",
        Tier::T500 => "ab 500 kg",
        Tier::T1 => "ab 1 kg",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tier {
    T1000,
    T100,
    T500,
    T1,
}

/// Explicit label → (material, grade) mapping, specific before generic
/// ("Kupferkabel …" must not fall into a bare-copper arm; "Ms-58 Späne"
/// before "Ms-58"). Anything unlisted is skipped loudly at the call site.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", "Millberry"))
    } else if l.contains("kerze") {
        Some(("kupfer-berry", "Kerze"))
    } else if l.contains("verzinnt") {
        Some(("kupfer-berry", "verzinnt"))
    } else if l.contains("berry") {
        Some(("kupfer-berry", "Berry"))
    } else if l.contains("candy") {
        Some(("kupfer-gemischt", "Candy"))
    } else if l.contains("litzenkabel 75") {
        Some(("kabel-kupfer", "Litzen 75%"))
    } else if l.contains("litzenkabel 60") {
        Some(("kabel-kupfer", "Litzen 60%"))
    } else if l.contains("kupferkabel 75") {
        Some(("kabel-kupfer", "75%"))
    } else if l.contains("kupferkabel 60") {
        Some(("kabel-kupfer", "60%"))
    } else if l.contains("kupferkabel 50") {
        Some(("kabel-kupfer", "50%"))
    } else if l.contains("kupferkabel 38") {
        Some(("kabel-kupfer", "38%"))
    } else if l.contains("kabel mit stecker") {
        Some(("kabel-kupfer", "mit Stecker"))
    } else if l.contains("aluminium kabel") {
        Some(("kabel-alu", ""))
    } else if l.contains("ms-58 späne") || l.contains("ms-58 sp") {
        Some(("messing", "Ms-58 Späne"))
    } else if l.contains("ms-58") {
        Some(("messing", "Ms-58"))
    } else if l.contains("messing hülsen") {
        Some(("messing", "Hülsen"))
    } else if l.contains("messing schwer") {
        Some(("messing", "schwer"))
    } else if l.contains("messing leicht") {
        Some(("messing", "leicht"))
    } else if l.contains("rotgu") {
        Some(("bronze-rotguss", ""))
    } else if l.contains("wicu") {
        Some(("kupfer-gemischt", "WiCu-Rohre"))
    } else if l.contains("kupfer schwer") {
        Some(("kupfer-gemischt", "schwer"))
    } else if l.contains("kupfer leicht") {
        Some(("kupfer-gemischt", "leicht"))
    } else if l.contains("kupfer späne") || l.contains("kupferspäne") {
        Some(("kupfer-gemischt", "Späne"))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", "V4A"))
    } else if l.contains("edelstahl") {
        Some(("edelstahl-gemischt", ""))
    } else if l.contains("aluminium profile farbe") || l.contains("profile farbe") {
        Some(("aluminium-profile", "Farbe"))
    } else if l.contains("aluminium profile") {
        Some(("aluminium-profile", ""))
    } else if l.contains("offset") {
        Some(("aluminium-blech", "Offset"))
    } else if l.contains("aluminium blech farbe") || l.contains("blech farbe") {
        Some(("aluminium-blech", "Farbe"))
    } else if l.contains("aluminium bleche") {
        Some(("aluminium-blech", ""))
    } else if l.contains("konstruktal") {
        Some(("aluminium-gemischt", "Konstruktal"))
    } else if l.contains("aluminium felgen") {
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("guß unsauber") || l.contains("guss unsauber") {
        Some(("aluminium-guss", "unsauber"))
    } else if l.contains("aluminium späne") {
        Some(("aluminium-gemischt", "Späne"))
    } else if l.contains("aluminium gemischt") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("bremsscheiben") {
        Some(("eisenschrott-gussbruch", "Bremsscheiben"))
    } else if l.contains("gußeisen") || l.contains("gusseisen") {
        Some(("eisenschrott-gussbruch", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", "leicht"))
    } else if l.contains("schredderschrott") {
        Some(("stahlschrott-shredder", ""))
    } else if l.contains("schwerschrott") {
        Some(("mischschrott", "Schwerschrott"))
    } else if l.contains("brennerschrott") {
        Some(("mischschrott", "Brennerschrott"))
    } else if l.contains("stahlspäne") || l.contains("stahlsp") {
        Some(("mischschrott", "Stahlspäne"))
    } else if l.contains("widia") || l.contains("vhm") {
        Some(("hartmetall", ""))
    } else if l.contains("zinngeschirr") {
        Some(("zinn", "Geschirr"))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("elektromotoren") || l.contains("e-motoren") {
        Some(("elektromotoren", ""))
    } else {
        None
    }
}

/// Loud reason for labels without catalog material: bimetallic or
/// steel-armored grades would corrupt single-metal series, and Sorte 3
/// has no catalog entry (extension = separate step).
fn skip_note(label: &str) -> &'static str {
    let l = label.to_lowercase();
    if l.contains("erdkabel") {
        "Stahlmantel-Erdkabel: Leiter uneindeutig"
    } else if l.contains("alu-kupfer") {
        "Bimetall-Kühler: Cu/Alu-Anteil uneindeutig"
    } else if l.contains("alu-leitung") {
        "Stahlseelen-Leitung: kein reines Alukabel"
    } else if l.contains("sorte 3") {
        "kein Katalogmaterial (Vorschlag: stahlschrott-sorte-3)"
    } else {
        "kein Katalogmaterial"
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` carrying
/// "Altmetallhandel OHG" holds firm lines + street + PLZ city over `<br>`,
/// the phone is the bare digits-only sibling `<div>`, the mail is the
/// `mailto:` link. Missing anchors mean the page changed shape → loud
/// error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let div = Selector::parse("div").expect("valid selector");
    let a = Selector::parse("a[href^=\"mailto:\"]").expect("valid selector");
    if !doc
        .select(&h1)
        .any(|h| h.text().collect::<String>().trim() == "Impressum")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let addr_p = doc.select(&p).find(|el| {
        el.text()
            .collect::<String>()
            .contains("Altmetallhandel OHG")
    });
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let mut lines = Vec::new();
    for part in addr_p.inner_html().split("<br") {
        let t = strip_fragment(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, w| a + " " + w);
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                break;
            }
        }
    }
    // Bare digits-only sibling <div> ("0531 / 28 76 60 08"); the mailto
    // sibling carries the address with '@' and never matches.
    let mut phone = String::new();
    let mut after_addr = false;
    for el in doc.select(&div) {
        if !after_addr {
            if el.inner_html().contains("Altmetallhandel OHG") {
                after_addr = true;
            }
            continue;
        }
        let t: String = el.text().collect();
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if !t.is_empty()
            && t.chars()
                .all(|c| c.is_ascii_digit() || " +/().-".contains(c))
            && t.chars().filter(|c| c.is_ascii_digit()).count() >= 7
        {
            phone = t;
            break;
        }
        if el.text().collect::<String>().contains("ffnungszeiten") {
            break;
        }
    }
    let email = doc
        .select(&a)
        .filter_map(|el| el.value().attr("href"))
        .next()
        .and_then(|h| h.strip_prefix("mailto:"))
        .map(|s| s.to_owned())
        .unwrap_or_default();
    // Scraper glues the mailto anchor text without separator; the href is
    // authoritative — but only the trader's own domain counts.
    let email = if email.contains("neu-wert.de") {
        email
    } else {
        String::new()
    };
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

/// Strip tags from a `<br`-split fragment (html5ever already decoded
/// entities). Fragments start with a tag remnant — drop everything up to
/// the first '>' first, or attributes parse as text.
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

fn parse(html: &str) -> Result<(Vec<(String, Tier, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html.find(START_ANCHOR).ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisliste fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find(END_ANCHOR).ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preislisten-Ende fehlt".to_owned(),
    })?;
    let window = &tail[..end];
    let mut rows: Vec<(String, Tier, f64, &'static str)> = Vec::new();
    let mut skips: Vec<String> = Vec::new();
    // Pass 1: headline cards — ordered h2/tier events; a tier before the
    // first headline means the markup moved → loud error.
    let mut events: Vec<(usize, bool, String)> = Vec::new();
    let mut i = 0;
    while let Some(h) = window[i..].find(H_MARKER) {
        let s = i + h + H_MARKER.len();
        if let Some(e) = window[s..].find("</h2>") {
            events.push((s, true, window[s..s + e].trim().to_owned()));
            i = s + e;
        } else {
            break;
        }
    }
    let mut j = 0;
    while let Some(p) = window[j..].find(TIER_MARKER) {
        let s = j + p + TIER_MARKER.len();
        if let Some(e) = window[s..].find("</span>") {
            let raw = window[s..s + e].replace("<b>", " ").replace("</b>", " ");
            events.push((
                s,
                false,
                raw.split_whitespace().collect::<Vec<_>>().join(" "),
            ));
            j = s + e;
        } else {
            break;
        }
    }
    events.sort_by_key(|e| e.0);
    let mut current: Option<String> = None;
    for (_, is_head, text) in events {
        if is_head {
            current = Some(text);
        } else if let Some(name) = current.clone() {
            match split_tier(&text) {
                Some((tier, price)) => rows.push((name, tier, price, "EUR/kg")),
                None => skips.push(format!("{name} (Staffel unverständlich: {text})")),
            }
        } else {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: "Preis ohne Material".to_owned(),
            });
        }
    }
    // Pass 2: swiper slides — self-delimiting heading/description pairs;
    // a slide without tier lines skips loudly, never silently.
    let mut k = 0;
    let mut slide_rows = 0;
    while let Some(h) = window[k..].find(SLIDE_HEAD_MARKER) {
        let hs = k + h + SLIDE_HEAD_MARKER.len();
        let Some(he) = window[hs..].find("</div>") else {
            break;
        };
        let head = window[hs..hs + he].trim().to_owned();
        k = hs + he;
        let Some(d) = window[k..].find(SLIDE_DESC_MARKER) else {
            skips.push(format!("{head} (keine Staffelpreise)"));
            continue;
        };
        let ds = k + d + SLIDE_DESC_MARKER.len();
        let Some(de) = window[ds..].find("</div>") else {
            skips.push(format!("{head} (keine Staffelpreise)"));
            continue;
        };
        let desc = &window[ds..ds + de];
        k = ds + de;
        let mut tiers = 0;
        let mut pos = 0;
        while let Some(li) = desc[pos..].find("<li>") {
            let ls = pos + li + 4;
            if let Some(le) = desc[ls..].find("</li>") {
                let text = desc[ls..ls + le]
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                if text.starts_with("ab ") {
                    match split_tier(&text) {
                        Some((tier, price)) => {
                            rows.push((head.clone(), tier, price, "EUR/kg"));
                            tiers += 1;
                        }
                        None => skips.push(format!("{head} (Staffel unverständlich: {text})")),
                    }
                }
                pos = ls + le;
            } else {
                break;
            }
        }
        if tiers == 0 {
            skips.push(format!("{head} (keine Staffelpreise)"));
        } else {
            slide_rows += tiers;
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preispaare".to_owned(),
        });
    }
    if slide_rows == 0 {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Slider-Preise".to_owned(),
        });
    }
    // Doppelblöcke ("Aluminium Kabel" steht in zwei Karussells) per
    // (Material, Staffel, Preis) dedupen — nach dem Rohlabel, vor Mapping.
    let mut seen = std::collections::HashSet::new();
    rows.retain(|(l, t, p, _)| seen.insert((l.clone(), *t, p.to_bits())));
    Ok((rows, skips))
}

/// "ab 1000 kg: 11,40 €" → tier + price. The tier text must name kg —
/// anything else skips loudly at the call site (a per-tonne price
/// recorded as per-kg would be a 1000x error).
fn split_tier(text: &str) -> Option<(Tier, f64)> {
    let (left, right) = text.split_once(':')?;
    let tier = if left.contains("1000") {
        Tier::T1000
    } else if left.contains("500") {
        Tier::T500
    } else if left.contains("100") {
        Tier::T100
    } else if left.contains('1') {
        Tier::T1
    } else {
        return None;
    };
    if !left.to_lowercase().contains("kg") {
        return None;
    }
    parse_eur(right).map(|p| (tier, p))
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, skip_note, Tier};

    /// Real live markup, trimmed to two cards + two slides + anchors.
    const FIXTURE: &str = "<h2 class=\"elementor-heading-title elementor-size-default\">Altmetall-Preisliste für diese Woche</h2>\
        <h2 class=\"elementor-heading-title elementor-size-default\">Kupfer</h2>\
        <h2 class=\"elementor-heading-title elementor-size-default\">Kupfer Millberry</h2>\
        <span class=\"elementor-icon-list-text\">ab 1000 kg: <b>11,40 €</b></span>\
        <span class=\"elementor-icon-list-text\">ab 100 kg: <b>11,10 €</b></span>\
        <span class=\"elementor-icon-list-text\">ab 1 kg: <b>10,80 €</b></span>\
        <div class=\"elementor-toggle-title\">Infos zu Kupfer Millberry</div>\
        <h2 class=\"elementor-heading-title elementor-size-default\">Bremsscheiben</h2>\
        <span class=\"elementor-icon-list-text\">ab 1000 kg: <b>0,23 €</b></span>\
        <span class=\"elementor-icon-list-text\">ab 500 kg: <b>0,20 €</b></span>\
        <span class=\"elementor-icon-list-text\">ab 1 kg: <b>0,17 €</b></span>\
        <div class=\"elementor-slide-heading\">Erdkabel mit Stahl</div><div class=\"elementor-slide-description\"><l> \n<li> ab 1000 kg: 0,90 € </li>\n<li> ab 100 kg: 0,70 € </li>\n<li> ab 1 kg: 0,50 € </li>\n</l></div>\
        <div class=\"elementor-slide-heading\">V4A-Edelstahl</div><div class=\"elementor-slide-description\"><l> \n<li> ab 1000 kg: 1,60 € </li>\n<li> ab 100 kg: 1,40 € </li>\n<li> ab 1 kg: 1,20 € </li>\n</l></div>\
        <h2 class=\"elementor-heading-title elementor-size-default\">Häufig gestellte Fragen</h2>";

    #[test]
    fn cards_and_slides_parse() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 12);
        assert!(skips.is_empty());
        assert_eq!(
            rows[0],
            ("Kupfer Millberry".to_owned(), Tier::T1000, 11.4, "EUR/kg")
        );
        assert_eq!(
            rows[3],
            ("Bremsscheiben".to_owned(), Tier::T1000, 0.23, "EUR/kg")
        );
        assert_eq!(rows[4].1, Tier::T500);
        assert_eq!(rows[6].0, "Erdkabel mit Stahl");
        assert_eq!(rows[9].0, "V4A-Edelstahl");
        assert_eq!(rows[11].2, 1.2);
    }

    #[test]
    fn anchors_and_slides_fail_loudly() {
        assert!(parse("<h2>Sonst was</h2>").is_err(), "start anchor missing");
        let no_end = FIXTURE.replacen("Häufig gestellte Fragen", "Sonst was", 1);
        assert!(parse(&no_end).is_err(), "end anchor missing");
        // Cards without any slider block: redesign, not success.
        let no_slides = FIXTURE
            .replace("<div class=\"elementor-slide-heading\">Erdkabel mit Stahl</div><div class=\"elementor-slide-description\"><l> \n<li> ab 1000 kg: 0,90 € </li>\n<li> ab 100 kg: 0,70 € </li>\n<li> ab 1 kg: 0,50 € </li>\n</l></div>", "")
            .replace("<div class=\"elementor-slide-heading\">V4A-Edelstahl</div><div class=\"elementor-slide-description\"><l> \n<li> ab 1000 kg: 1,60 € </li>\n<li> ab 100 kg: 1,40 € </li>\n<li> ab 1 kg: 1,20 € </li>\n</l></div>", "");
        assert!(parse(&no_slides).is_err(), "missing slides error");
        // Foreign tier unit never defaults to kg.
        let bad_unit = FIXTURE.replacen(
            "ab 1000 kg: <b>11,40 €</b>",
            "ab Palette: <b>11,40 €</b>",
            1,
        );
        let (rows, skips) = parse(&bad_unit).expect("parses");
        assert_eq!(rows.len(), 11);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Millberry"));
    }

    #[test]
    fn duplicate_slides_dedup() {
        let dup = FIXTURE.replacen(
            "<h2 class=\"elementor-heading-title elementor-size-default\">Häufig gestellte Fragen</h2>",
            "<div class=\"elementor-slide-heading\">V4A-Edelstahl</div><div class=\"elementor-slide-description\"><l> \n<li> ab 1000 kg: 1,60 € </li>\n<li> ab 100 kg: 1,40 € </li>\n<li> ab 1 kg: 1,20 € </li>\n</l></div>\
            <h2 class=\"elementor-heading-title elementor-size-default\">Häufig gestellte Fragen</h2>",
            1,
        );
        let (rows, _) = parse(&dup).expect("parses");
        assert_eq!(rows.len(), 12, "repeat slide dedups");
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h1 class=\"elementor-heading-title elementor-size-default\">Impressum</h1>\
            <div><p><b>NEUWERT</b> <b>&#8211;</b><strong> Schrott &amp; Altmetallhandel OHG</strong><br>Benzstraße 2<br>38112 Braunschweig</p></div>\
            <div><a title=\"Öffnet ein Fenster\" href=\"mailto:info@neu-wert.de\" rel=\"nofollow\">info@neu-wert.de</a></div>\
            <div>0531 / 28 76 60 08</div><div>&nbsp;</div><div><b>Öffnungszeiten</b></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Benzstraße 2");
        assert_eq!(info.postcode, "38112");
        assert_eq!(info.city, "Braunschweig");
        assert_eq!(info.phone, "0531 / 28 76 60 08");
        assert_eq!(info.email, "info@neu-wert.de");
        assert!(extract_info("<h1>Impressum</h1><p>Neu hier</p>").is_err());
        assert!(extract_info("<p>Ohne Titel</p>").is_err());
    }

    #[test]
    fn mapping_orders_specific_first_and_skips_loudly() {
        assert_eq!(
            grade_for("Kupfer Millberry"),
            Some(("kupfer-millberry", "Millberry"))
        );
        assert_eq!(grade_for("Kupfer Kerze"), Some(("kupfer-berry", "Kerze")));
        assert_eq!(grade_for("Kupfer Berry"), Some(("kupfer-berry", "Berry")));
        assert_eq!(
            grade_for("Kupfer verzinnt"),
            Some(("kupfer-berry", "verzinnt"))
        );
        assert_eq!(
            grade_for("Kupfer Candy"),
            Some(("kupfer-gemischt", "Candy"))
        );
        assert_eq!(
            grade_for("Kupfer schwer"),
            Some(("kupfer-gemischt", "schwer"))
        );
        assert_eq!(grade_for("Kupferkabel 75%"), Some(("kabel-kupfer", "75%")));
        assert_eq!(grade_for("Kupferkabel 38%"), Some(("kabel-kupfer", "38%")));
        assert_eq!(
            grade_for("Litzenkabel 60% (flexibel)"),
            Some(("kabel-kupfer", "Litzen 60%"))
        );
        assert_eq!(
            grade_for("Kabel mit Stecker"),
            Some(("kabel-kupfer", "mit Stecker"))
        );
        assert_eq!(grade_for("Aluminium Kabel"), Some(("kabel-alu", "")));
        assert_eq!(
            grade_for("Ms-58 Späne (sauber)"),
            Some(("messing", "Ms-58 Späne"))
        );
        assert_eq!(grade_for("Ms-58"), Some(("messing", "Ms-58")));
        assert_eq!(grade_for("Messing Hülsen"), Some(("messing", "Hülsen")));
        assert_eq!(grade_for("Rotguß"), Some(("bronze-rotguss", "")));
        assert_eq!(
            grade_for("Aluminium Profile Farbe"),
            Some(("aluminium-profile", "Farbe"))
        );
        assert_eq!(
            grade_for("Aluminium Profile"),
            Some(("aluminium-profile", ""))
        );
        assert_eq!(
            grade_for("Aluminium Offset-Blech"),
            Some(("aluminium-blech", "Offset"))
        );
        assert_eq!(
            grade_for("Aluminium Felgen"),
            Some(("aluminium-guss", "Felgen"))
        );
        assert_eq!(
            grade_for("Aluminium Konstruktal"),
            Some(("aluminium-gemischt", "Konstruktal"))
        );
        assert_eq!(
            grade_for("Aluminium gemischt"),
            Some(("aluminium-gemischt", ""))
        );
        assert_eq!(
            grade_for("Bremsscheiben"),
            Some(("eisenschrott-gussbruch", "Bremsscheiben"))
        );
        assert_eq!(grade_for("Gußeisen"), Some(("eisenschrott-gussbruch", "")));
        assert_eq!(
            grade_for("Mischschrott leicht"),
            Some(("mischschrott", "leicht"))
        );
        assert_eq!(
            grade_for("Schredderschrott"),
            Some(("stahlschrott-shredder", ""))
        );
        assert_eq!(grade_for("Widia / VHM"), Some(("hartmetall", "")));
        assert_eq!(grade_for("Zinngeschirr"), Some(("zinn", "Geschirr")));
        assert_eq!(grade_for("V4A-Edelstahl"), Some(("edelstahl-v4a", "V4A")));
        assert_eq!(grade_for("Edelstahl"), Some(("edelstahl-gemischt", "")));
        assert_eq!(grade_for("Elektromotoren"), Some(("elektromotoren", "")));
        assert_eq!(grade_for("Erdkabel mit Stahl"), None);
        assert_eq!(grade_for("Alu-Kupfer-Kühler"), None);
        assert_eq!(grade_for("Alu-Leitung mit Stahl"), None);
        assert_eq!(grade_for("Sorte 3"), None);
        assert!(skip_note("Erdkabel mit Stahl").contains("uneindeutig"));
        assert!(skip_note("Sorte 3").contains("stahlschrott-sorte-3"));
    }
}
