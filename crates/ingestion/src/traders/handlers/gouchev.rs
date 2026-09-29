//! Efrem Gouchev Schrottankauf (Berlin-Marzahn, Bitterfelder Str. 23):
//! exact daily prices per grade (`div#price > div.cms-article.<sorte>`:
//! title in `p.h5`, base price behind "ab 1 kg ➜"). The banner renews
//! every morning by 9:00 ("BIS 9:00 UHR PASSEN WIR DIE PREISE AN, DIE
//! DANN DEN GESAMTEN TAG ÜBER GELTEN"), so the schedule is `DailyAt`
//! 9:30 Berlin — the fresh Tagespreis, once. No per-row date on the page,
//! so `published_at` stays `None` (`observed_at` = age).
//!
//! Quantity/payment staffel per block (live 28.09.2026: Kupfer 10,45 /
//! Millberry 11,15 / Schwer 10,75 / Kerze 10,95 / Messing 6,30 /
//! Kupferkabel 3,45 / mit Stecker 1,00 / Zinn Teller 10,00–12,00 /
//! Lötzinn 5,00–7,00 / Zink 1,75): "ab 1 kg" is the Bar base, then
//! "ab 200 kg" and "ab 1000 kg" quotes read "<Überweisung>€ / <base>€
//! Überweisung / Bar" — the higher staffel price applies ONLY on
//! Überweisung. Every (grade × tier) pair becomes its own `variant`
//! ("… , ab 200 kg Überweisung"), so same-material sorts never collapse
//! onto one current price; tier labels carry the condition into `notes`
//! (record() stores the label as notes). Base rows alone would silently
//! underquote the staffel — hence one row per tier, never just the base.
//!
//! Mapping notes: "Scherenschrott / Gussschrott" names two iron grades at
//! one price and is skipped loudly (ambiguous, never crammed). The
//! "Aluminiumkabel" block carries a second sort ("ALUKABEL DICK 0,35€",
//! own `<p>`) with its own variant and no staffel of its own.
//!
//! Units: the page quotes no per-row unit, but the site's own homepage
//! prices "0,10 Euro pro kg für viele Buntmetalle", and every live value
//! is kg-plausible (Cu ~10–11, Fe ~0,1 — as €/t both would be absurd).
//! `unit_of` therefore defaults to EUR/kg (documented, plausibilized)
//! but still rejects any explicit foreign unit — a per-tonne quote
//! recorded as per-kg would be a 1000x error.
//!
//! Duplicate: `bb-altlandsberg-fa-efrem-gouchev-schrottankauf` (seed, no
//! website) shares the firm name but has no price page — intentionally no
//! handler for it, documented here only, seed untouched.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "be-marzahn-hellersdorf-efrem-gouchev-schrottankauf";
/// Bespoke, live-verified impressum URL (site nav links `/impressum`).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.schrottankauf-bitterfelderstr23.de/impressum";

pub const URL: &str = "https://www.schrottankauf-bitterfelderstr23.de/schrottpreise";

/// Canonical staffel tiers, the trader's own threshold words. The tier
/// rides in every variant so (grade × tier) pairs never collapse onto one
/// current price; the Überweisung condition is part of the tier because
/// the higher price exists ONLY on Überweisung (Bar stays at base).
const TIER_BASE: &str = "ab 1 kg";
const TIER_200: &str = "ab 200 kg Überweisung";
const TIER_1000: &str = "ab 1000 kg Überweisung";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::DailyAt {
            times: vec![(9, 30)],
        },
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (title, tier, price, unit) in rows {
        // The tier condition rides in the label (= notes in record()) as
        // well as in the variant: provenance for "why is this higher".
        // Base rows keep the raw title (legacy notes continuity).
        let label = match tier {
            TIER_200 => format!("{title} (ab 200 kg, Überweisung)"),
            TIER_1000 => format!("{title} (ab 1000 kg, Überweisung)"),
            _ => title.clone(),
        };
        match grade_for(&title, tier) {
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
            // No (or ambiguous) catalog material: keep the quoted price as
            // evidence in the skip, never drop it silently.
            None => skipped_labels.push(format!(
                "{label} ({}, {unit}, kein Katalogmaterial: Sorte)",
                fmt_eur(price)
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

fn fmt_eur(price: f64) -> String {
    format!("{price:.2}").replace('.', ",")
}

/// Explicit (block title, tier) → (material, variant) mapping. Anything
/// unlisted is skipped. Arms are specific-before-generic: "Millberry"/
/// "Kerze"/"Schwer" before plain copper, "Aluminiumkabel" before plain
/// aluminium, lead cable grades before "Altblei". The split "… dick" row
/// maps to the dick detail only (see parse).
///
/// Variant rule: the base tier keeps the legacy grade wording untouched
/// (`""`, `"Kerze"`, …) so existing current prices keep updating; staffel
/// tiers append the payment condition (`"Kerze, ab 200 kg Überweisung"`).
/// Every (detail × tier) pair is enumerated — an unknown combination is a
/// redesign and skips loudly via the fallthrough, never a minted variant.
fn grade_for(label: &str, tier: &'static str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Grade detail first; the tier joins it in the variant table below.
    let (material, detail): (&'static str, &'static str) = if l.contains("millberry") {
        ("kupfer-millberry", "")
    } else if l.contains("kerze") && l.contains("kupfer") {
        ("kupfer-berry", "Kerze")
    } else if l.contains("schwer") && l.contains("kupfer") {
        ("kupfer-berry", "Schwer")
    } else if l.contains("kupferkabelschrott") && l.contains("stecker") {
        ("kabel-kupfer", "mit Stecker")
    } else if l.contains("kupferkabel") || l.contains("kupfer-kabel") {
        ("kabel-kupfer", "")
    } else if l.contains("aluminiumkabel") || l.contains("alukabel") {
        // Two sorts, one block ("ab 1 kg ➜ 0,10€ / ALUKABEL DICK 0,35€"):
        // the parse step splits them into two rows ("… dick" suffix), so
        // each row maps to exactly one detail — never both, or the two
        // prices would collapse onto one arbitrary current price.
        if l.contains("dick") {
            ("kabel-alu", "dick")
        } else {
            ("kabel-alu", "")
        }
    } else if l.contains("kupfer") {
        // Generic copper ("Kupfer ohne Eisen- oder Messinganhaftungen"):
        // placed before lead/brass — the label names what is EXCLUDED,
        // not what it is. All kupfer-* sorts above already matched.
        ("kupfer-gemischt", "")
    } else if l.contains("elektromotor") {
        ("elektromotoren", "")
    } else if l.contains("lötzinn") || l.contains("loetzinn") {
        ("loetzinn", "Lötzinn")
    } else if l.contains("zinn") {
        // Grades: the range IS the grade ("Zinnschrott 90-95 % (Teller)").
        if l.contains("90") {
            ("zinn-geschirr", "90-95%")
        } else {
            ("zinn", "")
        }
    } else if l.contains("auswuchtblei") {
        ("blei-auswucht", "Auswuchtblei")
    } else if l.contains("schälblei") || l.contains("schaelblei") {
        ("blei-auswucht", "Kabelschälblei")
    } else if l.contains("altblei") || (l.contains("blei") && !l.contains("kabel")) {
        ("blei", "")
    } else if l.contains("messing") {
        ("messing", "")
    } else if l.contains("mischschrott") {
        ("mischschrott", "")
    } else if l.contains("scherenschrott") || l.contains("gussschrott") {
        // One price for two iron grades — ambiguous, skip loudly.
        return None;
    } else if l.contains("v2a") || l.contains("edelstahl") {
        ("edelstahl-v2a", "")
    } else if l.contains("zink") {
        ("zink", "")
    } else if l.contains("aluminium") {
        if l.contains("5%") || l.contains("anhaftung") && l.contains("max") {
            ("aluminium-gemischt", "5% Anhaftung")
        } else {
            ("aluminium-gemischt", "")
        }
    } else {
        return None;
    };
    let variant: &'static str = match (detail, tier) {
        ("", TIER_BASE) => "",
        ("", TIER_200) => "ab 200 kg Überweisung",
        ("", TIER_1000) => "ab 1000 kg Überweisung",
        ("Kerze", TIER_BASE) => "Kerze",
        ("Kerze", TIER_200) => "Kerze, ab 200 kg Überweisung",
        ("Kerze", TIER_1000) => "Kerze, ab 1000 kg Überweisung",
        ("Schwer", TIER_BASE) => "Schwer",
        ("Schwer", TIER_200) => "Schwer, ab 200 kg Überweisung",
        ("Schwer", TIER_1000) => "Schwer, ab 1000 kg Überweisung",
        ("mit Stecker", TIER_BASE) => "mit Stecker",
        ("mit Stecker", TIER_200) => "mit Stecker, ab 200 kg Überweisung",
        ("mit Stecker", TIER_1000) => "mit Stecker, ab 1000 kg Überweisung",
        ("dick", TIER_BASE) => "dick",
        ("Lötzinn", TIER_BASE) => "Lötzinn",
        ("Lötzinn", TIER_200) => "Lötzinn, ab 200 kg Überweisung",
        ("Lötzinn", TIER_1000) => "Lötzinn, ab 1000 kg Überweisung",
        ("90-95%", TIER_BASE) => "90-95%",
        ("90-95%", TIER_200) => "90-95%, ab 200 kg Überweisung",
        ("90-95%", TIER_1000) => "90-95%, ab 1000 kg Überweisung",
        ("Auswuchtblei", TIER_BASE) => "Auswuchtblei",
        ("Auswuchtblei", TIER_200) => "Auswuchtblei, ab 200 kg Überweisung",
        ("Auswuchtblei", TIER_1000) => "Auswuchtblei, ab 1000 kg Überweisung",
        ("Kabelschälblei", TIER_BASE) => "Kabelschälblei",
        ("Kabelschälblei", TIER_200) => "Kabelschälblei, ab 200 kg Überweisung",
        ("Kabelschälblei", TIER_1000) => "Kabelschälblei, ab 1000 kg Überweisung",
        ("5% Anhaftung", TIER_BASE) => "5% Anhaftung",
        ("5% Anhaftung", TIER_200) => "5% Anhaftung, ab 200 kg Überweisung",
        ("5% Anhaftung", TIER_1000) => "5% Anhaftung, ab 1000 kg Überweisung",
        _ => return None,
    };
    Some((material, variant))
}

/// Parse the `div#price` blocks. Returns (rows, skips) with one row per
/// (block × tier): the "ab 1 kg" Bar base plus the "ab 200 kg" and
/// "ab 1000 kg" Überweisung quotes (first number after the tier marker —
/// the pair reads "<Überweisung>€ / <base>€ Überweisung / Bar"). The
/// Aluminiumkabel block has no staffel and yields base + DICK instead.
/// An empty listing — or a page whose staffel vanished entirely — is a
/// loud error, never a silent success.
fn parse(
    html: &str,
) -> Result<
    (
        Vec<(String, &'static str, f64, &'static str)>,
        Vec<String>,
    ),
    IngestError,
> {
    let start = html
        .find("id=\"price\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Tagespreis-Block fehlt".to_owned(),
        })?;
    // Back up to the opening '<': slicing at `id=` would cut the div tag
    // itself and the `div#price` selector below would never match.
    let tag = html[..start].rfind('<').unwrap_or(0);
    let frag = Html::parse_fragment(&format!("<div>{}</div>", &html[tag..]));
    let price_sel = Selector::parse("div#price").expect("valid selector");
    let block_sel = Selector::parse("div.cms-article").expect("valid selector");
    let h5_sel = Selector::parse("p.h5").expect("valid selector");
    let p_sel = Selector::parse("p").expect("valid selector");
    let price_box = frag
        .select(&price_sel)
        .next()
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Tagespreis-Block fehlt".to_owned(),
        })?;
    let mut rows: Vec<(String, &'static str, f64, &'static str)> = Vec::new();
    let mut skips = Vec::new();
    // Redesign guard: exactly one block (Aluminiumkabel) ships without a
    // staffel today — if NO block carries one, the tiers moved shape.
    let mut saw_staffel = false;
    for block in price_box.select(&block_sel) {
        let title = block
            .select(&h5_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if title.is_empty() || title.len() > 120 {
            skips.push("(Preisblock ohne Titel, übersprungen)".to_owned());
            continue;
        }
        let body: String = block
            .select(&p_sel)
            .skip(1)
            .map(|el| el.text().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ");
        let body = body.split_whitespace().collect::<Vec<_>>().join(" ");
        // An explicit foreign unit anywhere in the block rejects the
        // site-default loudly instead of silently mis-scaling.
        let Some(unit) = unit_of(&body) else {
            skips.push(format!("{title} (Einheit unverständlich: {body})"));
            continue;
        };
        // Base price = the "ab 1 kg" Bar quote. The marker is required:
        // without it a prose block's stray number (banner "9:00", footer
        // "12681") would become a phantom price.
        let Some((_, after_base)) = body.split_once("ab 1 kg") else {
            skips.push(format!("{title} (kein ab-1-kg-Preis)"));
            continue;
        };
        let Some(base) = parse_eur(after_base) else {
            skips.push(format!("{title} (kein ab-1-kg-Preis)"));
            continue;
        };
        // A "0,00" quote is "no quote", not a free gift: loud skip.
        if base == 0.0 {
            skips.push(format!("{title} (Preis 0,00)"));
            continue;
        }
        push_row(&mut rows, title.clone(), TIER_BASE, base, unit);
        // Staffel tiers: first number after the marker is the Überweisung
        // price (the pair's second number is the Bar base, ignored here).
        if let Some((_, after_200)) = body.split_once("ab 200") {
            saw_staffel = true;
            // Cut the 200-segment before the 1000-marker so its quote can
            // never leak into this tier (both spellings: "1000"/"1.000").
            let seg = after_200
                .split("ab 1000")
                .next()
                .unwrap_or(after_200)
                .split("ab 1.000")
                .next()
                .unwrap_or(after_200);
            match parse_eur(seg) {
                Some(tier_price) if tier_price != 0.0 => {
                    push_row(&mut rows, title.clone(), TIER_200, tier_price, unit)
                }
                Some(_) => skips.push(format!("{title} (ab 200 kg: Preis 0,00)")),
                None => skips.push(format!("{title} (ab 200 kg: Preis unverständlich)")),
            }
            match tier_1000_price(&body) {
                Some(tier_price) if tier_price != 0.0 => {
                    push_row(&mut rows, title.clone(), TIER_1000, tier_price, unit)
                }
                Some(_) => skips.push(format!("{title} (ab 1000 kg: Preis 0,00)")),
                None => skips.push(format!(
                    "{title} (ab 1000 kg: Staffel fehlt oder Preis unverständlich)"
                )),
            }
        }
        // Second sort inside the Aluminiumkabel block ("ALUKABEL DICK
        // 0,35€", own `<p>`): base tier only, no staffel of its own.
        if let Some((_, dick_text)) = body.split_once("DICK") {
            if let Some(dick) = parse_eur(dick_text) {
                if dick != 0.0
                    && !rows.iter().any(
                        |(t, _, p, _): &(String, &'static str, f64, &'static str)| {
                            *t == title && (*p - dick).abs() <= f64::EPSILON
                        },
                    )
                {
                    push_row(&mut rows, format!("{title} dick"), TIER_BASE, dick, unit);
                }
            }
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Tagespreise leer".to_owned(),
        });
    }
    if !saw_staffel {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisstaffel fehlt".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Dedupe against double blocks: same (title, tier, price) twice counts
/// once — after the mapping, on cooked rows, not on raw labels.
fn push_row(
    rows: &mut Vec<(String, &'static str, f64, &'static str)>,
    title: String,
    tier: &'static str,
    price: f64,
    unit: &'static str,
) {
    if !rows
        .iter()
        .any(|(t, ti, p, _)| *t == title && *ti == tier && (*p - price).abs() <= f64::EPSILON)
    {
        rows.push((title, tier, price, unit));
    }
}

/// First number after the "ab 1000 kg" marker (both spellings), i.e. the
/// Überweisung quote of the top tier. `None` = marker missing or number
/// unparseable (loud skip at the call site).
fn tier_1000_price(body: &str) -> Option<f64> {
    let after = body
        .split_once("ab 1000")
        .map(|(_, rest)| rest)
        .or_else(|| body.split_once("ab 1.000").map(|(_, rest)| rest))?;
    parse_eur(after)
}

/// Bespoke unit gate for THESE blocks: the site quotes "Euro pro kg"
/// (homepage banner above the listing) and nothing else — but any explicit
/// foreign unit still rejects the block loudly.
fn unit_of(block_text: &str) -> Option<&'static str> {
    let lower = block_text.to_lowercase();
    if lower.contains("/t")
        || lower.contains("pro tonne")
        || lower.contains("pro stück")
        || lower.contains("/stk")
        || lower.contains("pauschal")
    {
        None
    } else {
        Some("EUR/kg")
    }
}

/// Bespoke contact extraction for THIS impressum only: `dl.imprint-list`
/// carries labeled dt/dd rows (Adresse, Stadt, PLZ, E-Mail with the
/// `∂`-glyph + data-email JSON, Telefonnummer). Missing list → loud
/// error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let list = Selector::parse("dl.imprint-list").expect("valid selector");
    let dt = Selector::parse("dt").expect("valid selector");
    let dd = Selector::parse("dd").expect("valid selector");
    let Some(first) = doc.select(&list).next() else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressums-Liste fehlt".to_owned(),
        });
    };
    // Pair dt/dd by document order inside the first list.
    let labels: Vec<String> = first.select(&dt).map(|el| el.text().collect()).collect();
    let values: Vec<ElementRef> = first.select(&dd).collect();
    let text_of = |i: usize| {
        values
            .get(i)
            .map(|el| {
                el.text()
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default()
    };
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    let (mut phone, mut email) = (String::new(), String::new());
    for (k, label) in labels.iter().enumerate() {
        match label.trim() {
            "Adresse" => street = text_of(k),
            "Stadt" => city = text_of(k),
            "PLZ" => {
                postcode = text_of(k)
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .to_owned()
            }
            "E-Mail" => {
                // "info ∂ schrottankauf-bitterfelderstr23.de" — the ∂
                // glyph (also in data-email JSON) joins the halves.
                email = text_of(k)
                    .replace('∂', "@")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join("");
            }
            "Telefonnummer" if phone.is_empty() => phone = text_of(k),
            _ => {}
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

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of, TIER_1000, TIER_200, TIER_BASE};

    // Real shape of the live listing (cms-article classes, h5 title spans,
    // "ab 1 kg ➜" quotes, "…€ / …€ Überweisung / Bar" staffel pairs, the
    // ALUKABEL-DICK second sort in its own <p>), trimmed to three blocks.
    const FIXTURE: &str = "<div id=\"price\" class=\"cms-container-el price-container\">\
        <div class=\"cms-article millberry lazy-bg\">\
        <p class=\"h5\"><span style=\"font-size: 36px;\">Kupfer Millberry</span> <span>nicht angelaufen, nicht lackiert</span></p>\
        <p><span style=\"font-size: 36px;\">ab 1 kg ➜ <strong>11,15</strong><strong>€</strong></span><br/>\
        <span style=\"font-size: 24px;\">ab 200kg<span>: </span><strong><em>11,35€</em>/ </span>11,15€ </strong>\
        <strong><em>Überweisung</em>/ </span>Bar</strong>&nbsp; ab&nbsp;1000kg:<strong><em>11,45€</em>/ </span>11,15</strong>\
        <strong>€&nbsp; &nbsp; <em>Überweisung</em>/ Bar</strong></span></p></div>\
        <div class=\"cms-article aluminiumkabel lazy-bg\">\
        <p class=\"h5\">Aluminiumkabel</p>\
        <p><span style=\"font-size: 36px;\">ab 1 kg ➜ <strong>0,10€</strong></span></p>\
        <p><span style=\"font-size: 36px;\"><strong>ALUKABEL DICK</strong> <em>0,35€</em></span></p></div>\
        <div class=\"cms-article guss lazy-bg\">\
        <p class=\"h5\">Scherenschrott / Gussschrott</p>\
        <p><span style=\"font-size: 36px;\">ab 1 kg ➜ <strong>0,12€</strong></span></p>\
        <p><span style=\"font-size: 18px;\">ab 200 kg: <strong><em>0,12€</em>/0,12€</strong> <em>Überweisung</em>/Bar&nbsp;<br/>\
        ab 1000 kg: <strong><em>0,13€</em>/0,12€</strong> <em>Überweisung</em>/Bar</span></p></div>\
        </div>";

    #[test]
    fn tiers_parse_with_base_and_dick() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty());
        // Millberry × 3 tiers + Alukabel base + Alukabel dick + Scheren × 3.
        assert_eq!(rows.len(), 8);
        assert_eq!(
            rows[0],
            (
                "Kupfer Millberry nicht angelaufen, nicht lackiert".to_owned(),
                TIER_BASE,
                11.15,
                "EUR/kg"
            )
        );
        assert_eq!(rows[1].1, TIER_200);
        assert_eq!(rows[1].2, 11.35);
        assert_eq!(rows[2].1, TIER_1000);
        assert_eq!(rows[2].2, 11.45);
        assert_eq!(rows[3], ("Aluminiumkabel".to_owned(), TIER_BASE, 0.1, "EUR/kg"));
        assert_eq!(
            rows[4],
            ("Aluminiumkabel dick".to_owned(), TIER_BASE, 0.35, "EUR/kg")
        );
        assert_eq!(
            rows[5],
            (
                "Scherenschrott / Gussschrott".to_owned(),
                TIER_BASE,
                0.12,
                "EUR/kg"
            )
        );
        assert_eq!(rows[6].1, TIER_200);
        assert_eq!(rows[6].2, 0.12);
        assert_eq!(rows[7].1, TIER_1000);
        assert_eq!(rows[7].2, 0.13);
        assert_eq!(unit_of("ab 1 kg 10,40 €"), Some("EUR/kg"));
        assert_eq!(unit_of("pauschal 5 €"), None);
        assert!(parse("<div>Redesign ohne Preise</div>").is_err());
    }

    #[test]
    fn prose_without_marker_never_becomes_a_price() {
        // A titled block WITHOUT "ab 1 kg" (banner "9:00", footer "12681"
        // shapes): loud skip, never a phantom 9,00/12681,00 price.
        let html = "<div id=\"price\">\
            <div class=\"cms-article\"><p class=\"h5\">Hinweis</p>\
            <p>Jeden Morgen bis 9:00 Uhr passen wir die Preise an.</p></div>\
            <div class=\"cms-article millberry\"><p class=\"h5\">Kupfer Millberry</p>\
            <p>ab 1 kg ➜ 11,15€ ab 200kg: 11,35€ / 11,15€ Überweisung / Bar \
            ab 1000kg: 11,45€ / 11,15€ Überweisung / Bar</p></div></div>";
        let (rows, skips) = parse(html).expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("kein ab-1-kg-Preis"));
        // No staffel anywhere on the page: loud redesign error, not a
        // silent base-only success.
        let html = "<div id=\"price\">\
            <div class=\"cms-article aluminiumkabel\"><p class=\"h5\">Aluminiumkabel</p>\
            <p>ab 1 kg ➜ 0,10€</p><p>ALUKABEL DICK 0,35€</p></div></div>";
        let err = parse(html).expect_err("staffel missing errors");
        assert!(err.to_string().contains("Preisstaffel"));
    }

    #[test]
    fn zero_and_garbled_tier_quotes_skip_loudly() {
        // 0,00 tier quote: loud skip, sibling tiers survive.
        let html = FIXTURE.replacen("11,35€", "0,00€", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 7);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("ab 200 kg") && skips[0].contains("0,00"));
        // 200-marker without 1000-marker: the missing top tier skips
        // loudly instead of silently dropping a price.
        let html = FIXTURE.replacen("ab&nbsp;1000kg:", "ab heute:", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 7);
        assert!(skips.iter().any(|s| s.contains("ab 1000 kg")));
    }

    #[test]
    fn mapping_splits_grades_and_tiers() {
        // Base tier keeps the legacy grade wording (current-price
        // continuity); staffel tiers append the payment condition so no
        // two (grade × tier) pairs ever share a variant.
        assert_eq!(
            grade_for("Kupfer Millberry nicht angelaufen, nicht lackiert", TIER_BASE),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Kupfer Millberry nicht angelaufen, nicht lackiert", TIER_200),
            Some(("kupfer-millberry", "ab 200 kg Überweisung"))
        );
        assert_eq!(
            grade_for("Kupfer Millberry nicht angelaufen, nicht lackiert", TIER_1000),
            Some(("kupfer-millberry", "ab 1000 kg Überweisung"))
        );
        assert_eq!(
            grade_for("Kupfer ohne Eisen- oder Messinganhaftungen", TIER_BASE),
            Some(("kupfer-gemischt", ""))
        );
        assert_eq!(
            grade_for("Kupfer Schwer (ohne Lötstellen und Farbe)", TIER_BASE),
            Some(("kupfer-berry", "Schwer"))
        );
        assert_eq!(
            grade_for("Kupfer Schwer (ohne Lötstellen und Farbe)", TIER_1000),
            Some(("kupfer-berry", "Schwer, ab 1000 kg Überweisung"))
        );
        assert_eq!(
            grade_for("Kupfer Kerze (neu, ohne Anhaftung, nicht angelaufen)", TIER_200),
            Some(("kupfer-berry", "Kerze, ab 200 kg Überweisung"))
        );
        assert_eq!(
            grade_for("Kupferkabel kein Antennen-, Fett-, ALCU-, Eisenkabel", TIER_200),
            Some(("kabel-kupfer", "ab 200 kg Überweisung"))
        );
        assert_eq!(
            grade_for("Kupferkabelschrott mit Stecker", TIER_BASE),
            Some(("kabel-kupfer", "mit Stecker"))
        );
        assert_eq!(
            grade_for("Kupferkabelschrott mit Stecker", TIER_1000),
            Some(("kabel-kupfer", "mit Stecker, ab 1000 kg Überweisung"))
        );
        assert_eq!(
            grade_for("Aluminiumkabel", TIER_BASE),
            Some(("kabel-alu", ""))
        );
        assert_eq!(
            grade_for("Aluminiumkabel dick", TIER_BASE),
            Some(("kabel-alu", "dick"))
        );
        // A staffel on the dick sort was never quoted: loud skip, never a
        // minted variant.
        assert_eq!(grade_for("Aluminiumkabel dick", TIER_200), None);
        assert_eq!(
            grade_for("Edelstahlschrott (V2A)", TIER_1000),
            Some(("edelstahl-v2a", "ab 1000 kg Überweisung"))
        );
        assert_eq!(
            grade_for("Zinnschrott 90-95 % (Teller)", TIER_BASE),
            Some(("zinn-geschirr", "90-95%"))
        );
        assert_eq!(
            grade_for("Zinnschrott 90-95 % (Teller)", TIER_200),
            Some(("zinn-geschirr", "90-95%, ab 200 kg Überweisung"))
        );
        assert_eq!(
            grade_for("Zinnschrott Lötzinn", TIER_1000),
            Some(("loetzinn", "Lötzinn, ab 1000 kg Überweisung"))
        );
        assert_eq!(
            grade_for("Auswuchtblei", TIER_200),
            Some(("blei-auswucht", "Auswuchtblei, ab 200 kg Überweisung"))
        );
        assert_eq!(
            grade_for("Kabelschälblei Alt", TIER_BASE),
            Some(("blei-auswucht", "Kabelschälblei"))
        );
        assert_eq!(
            grade_for("Aluminiumschrott mit max. 5% Anhaftung", TIER_1000),
            Some(("aluminium-gemischt", "5% Anhaftung, ab 1000 kg Überweisung"))
        );
        assert_eq!(grade_for("Messing ohne Schläuche", TIER_200), Some(("messing", "ab 200 kg Überweisung")));
        assert_eq!(grade_for("Altblei", TIER_BASE), Some(("blei", "")));
        assert_eq!(grade_for("Zink", TIER_1000), Some(("zink", "ab 1000 kg Überweisung")));
        assert_eq!(
            grade_for("Elektromotoren", TIER_200),
            Some(("elektromotoren", "ab 200 kg Überweisung"))
        );
        assert_eq!(
            grade_for("Mischschrott", TIER_BASE),
            Some(("mischschrott", ""))
        );
        // Two iron grades, one price: ambiguous, skipped in every tier.
        assert_eq!(grade_for("Scherenschrott / Gussschrott", TIER_BASE), None);
        assert_eq!(grade_for("Scherenschrott / Gussschrott", TIER_200), None);
        assert_eq!(grade_for("Scherenschrott / Gussschrott", TIER_1000), None);
    }

    #[test]
    fn impressum_dl_rows() {
        let imp = "<dl class=\"imprint-list\">\
            <dt>Vollständiger Firmenname</dt><dd>Fa. Efrem Gouchev Schrottankauf</dd>\
            <dt>Adresse</dt><dd>Bitterfelder Str. 23</dd>\
            <dt>Stadt</dt><dd>Berlin</dd><dt>PLZ</dt><dd>12681 </dd>\
            <dt>E-Mail</dt><dd><a data-email='{\"name\":\"info\"}'>info<span>∂</span>schrottankauf-bitterfelderstr23.de</a></dd>\
            <dt>Telefonnummer</dt><dd><a href=\"tel:+493099272366\">030 99 272 366</a></dd>\
            </dl>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Bitterfelder Str. 23");
        assert_eq!(info.postcode, "12681");
        assert_eq!(info.city, "Berlin");
        assert_eq!(info.phone, "030 99 272 366");
        assert_eq!(info.email, "info@schrottankauf-bitterfelderstr23.de");
        assert!(extract_info("<p>Neu hier</p>").is_err());
    }
}
