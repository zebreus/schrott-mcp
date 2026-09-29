//! Huth Schrott-Metall GmbH (Aschaffenburg): graduated purchase prices in
//! `div.purchaseprices` tables (live 28.09.2026: 5 price tables with
//! per-quantity columns, plus 1 opening-hours table without a Quantity
//! header). Each material row carries one price per quantity tier
//! ("up to 25 kg" / "from 25 kg" / "from 250 kg" / "from 1000 kg" on the
//! Cu/Ms block; other blocks use their own thresholds), so every
//! (grade × tier) pair becomes its own `variant` (`"candle, from 25 kg"`):
//! same-material grades must never collapse onto one current price.
//! Page date ("Metal purchase prices as of 28.09.2026") → `published_at`.
//!
//! Quirks, all pinned by tests:
//! - Overview labels are English ("Copper, bare I, Millberry") while the
//!   detail-page slugs are German — mapping keys off the visible labels.
//! - No per-row unit is quoted anywhere; every live value is market-
//!   plausible per kg (Cu ~10, tin ~30-40, Fe scrap 0.05-0.21 = 50-210 €/t
//!   after record() normalizes into the EUR/t catalog units), so the page
//!   default is a documented EUR/kg constant. A foreign unit would have to
//!   appear as text in a price cell — anything unparseable skips loudly.
//! - Zinc block header typo: third tier reads "to 250 kg" (values rise
//!   monotonically, same position as "from 250 kg" elsewhere) → mapped to
//!   the canonical "from 250 kg" tier. A fixed header keeps working.
//! - "Heavy scrap" quotes "021" in the last tier (missing comma, would read
//!   21 €/kg): the row has no catalog entry (Schwerschrott ≠ Scheren-
//!   schrott) and skips loudly anyway — never a silent 100× price.
//! - "Silver contacts" (3-5 €/kg) must NOT map to `silber`: the catalog
//!   unit is EUR/g, so that mapping would be a 1000× error. Loud skip.
//! - Mixed-material coolers (Cu-brass, Alu, Alu-Cu) and batteries/CPUs/
//!   phones/disks/ram have own catalog entries (handys/festplatten/ram) or skip loudly, never guesses.

use std::collections::HashSet;

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "by-aschaffenburg-huth-schrott-metall-huth";
/// Bespoke, live-verified price URL (footer nav "Einkaufspreise"). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://www.huth-recycling.de/einkaufspreise";
/// Bespoke, live-verified impressum URL (the site's own footer link).
/// A move fails the step loudly — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.huth-recycling.de/impressum";

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
    let mut prices = Vec::with_capacity(rows.len());
    let mut seen = HashSet::new();
    for (label, tier, price, unit) in rows {
        match grade_for(&label, tier) {
            Some((material, variant)) => {
                // Repeated "Gültig ab" blocks would re-emit the same triple:
                // dedupe after mapping, on (material, variant, price).
                let key = (material, variant, price.to_bits());
                if seen.insert(key) {
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
            None => skipped_labels.push(format!("{label} ({tier})")),
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

/// Explicit (label, tier) → (material, variant) mapping. Labels are the
/// page's own English wording, matched specific-before-generic ("plug"
/// before the bare "copper" fallthrough, "ram" before the "silver"
/// trap). Anything unlisted is skipped loudly at the call site.
fn grade_for(label: &str, tier: &'static str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Mixed-material coolers/radiators have no catalog entry.
    if l.contains("cooler") || l.contains("radiator") {
        return None;
    }
    // "Buried cable muff" is a surcharge note, not a price row.
    if l.contains("muff") {
        return None;
    }
    // Lead-acid batteries are devices, not lead scrap.
    if l.contains("batter") {
        return None;
    }
    // CPUs are chips, not boards — no catalog entry.
    if l.contains("cpu") {
        return None;
    }
    let (material, detail): (&'static str, &'static str) = if l.contains("millberry") {
        ("kupfer-millberry", "")
    } else if l.contains("candle") {
        ("kupfer-berry", "candle")
    } else if l.contains("raff") || l.contains("berry") {
        ("kupfer-gemischt", "Raff, old 95 %")
    } else if l.contains("copper") && l.contains("plug") {
        ("kabel-mit-stecker", "with plug, 25 % Cu")
    } else if l.contains("copper") && l.contains("buried") {
        ("kabel-kupfer", "buried cable, 25 % Cu")
    } else if l.contains("copper") && l.contains("70") {
        ("kabel-kupfer", "70 % Cu")
    } else if l.contains("copper") && l.contains("38") {
        ("kabel-kupfer", "38 % Cu")
    } else if l.contains("copper") {
        // "Copper, light": generic grade → generic material.
        ("kupfer-gemischt", "light")
    } else if l.contains("brass") {
        if l.contains("ms58") {
            ("messing", "Ms58, new, piece")
        } else if l.contains("chip") {
            ("messing", "chips, mixed")
        } else if l.contains("heavy") {
            ("messing", "heavy")
        } else {
            // "Brass, light and sleeves (separate), brass with hoses".
            ("messing", "light, sleeves")
        }
    } else if l.contains("gunmetal") {
        if l.contains("lump") {
            ("bronze-rotguss", "lumpy")
        } else {
            ("bronze-rotguss", "chips")
        }
    } else if l.contains("aluminium") {
        if l.contains("tableware") {
            ("aluminium-blech", "tableware")
        } else if l.contains("sheet") {
            if l.contains("5000") {
                ("aluminium-blech", "sheets, bright, new, 5000/6000")
            } else if l.contains("sn") {
                ("aluminium-blech", "sheets, bright, new, alloyed")
            } else {
                ("aluminium-blech", "sheets, old, varnished")
            }
        } else if l.contains("profil") {
            if l.contains("blank") {
                ("aluminium-profile", "blank, new")
            } else if l.contains("old") {
                ("aluminium-profile", "old")
            } else {
                ("aluminium-profile", "lacquered")
            }
        } else if l.contains("cast") {
            if l.contains("new") {
                ("aluminium-guss", "new")
            } else {
                ("aluminium-guss", "old")
            }
        } else if l.contains("rim") {
            if l.contains("clean") {
                ("aluminium-guss", "rims, clean")
            } else {
                ("aluminium-guss", "rims, affixed")
            }
        } else if l.contains("offset") {
            ("aluminium-blech", "offset plates")
        } else if l.contains("shav") {
            // Aluminium shavings: no catalog entry.
            return None;
        } else if l.contains("cable") {
            if l.contains("buried") {
                ("kabel-alu", "buried cable")
            } else if l.contains("40") {
                ("kabel-alu", "40 % Al")
            } else {
                return None;
            }
        } else {
            return None;
        }
    } else if l.contains("zinc") {
        if l.contains("new") {
            ("zink", "new")
        } else {
            ("zink", "old")
        }
    } else if l.contains("lead") {
        if l.contains("balanc") {
            ("blei", "balancing weights")
        } else if l.contains("soft") {
            ("blei", "soft")
        } else if l.contains("mixed") {
            ("blei", "mixed")
        } else {
            return None;
        }
    } else if l.contains("shredder") {
        ("stahlschrott-shredder", "")
    } else if l.contains("mixed scrap") {
        ("mischschrott", "")
    } else if l.contains("new scrap") || l.contains("casting") {
        // Before the tin section: "castings" contains "tin".
        ("stahlschrott-sorte-1", "new")
    } else if l.contains("tin") {
        if l.contains("pure") {
            ("zinn", "pure, 99 %")
        } else if l.contains("solder") {
            ("zinn", "solder")
        } else if l.contains("harness") {
            ("zinn", "harness tin")
        } else {
            return None;
        }
    } else if l.contains("ram") {
        // RAM sticks are their own catalog material (gold/silver contacts
        // as variants); the fallback table records platinen acceptance.
        // Checked before the "silver contacts" trap below: that row quotes
        // €/kg while the `silber` catalog unit is EUR/g.
        if l.contains("gold") {
            ("ram", "RAM, gold contacts")
        } else {
            ("ram", "RAM, silver contacts")
        }
    } else if l.contains("board") {
        if l.contains("phone") {
            ("handys", "phone boards")
        } else if l.contains("1a") {
            ("platinen", "class 1A")
        } else if l.contains('2') {
            ("platinen", "class 2 / 1B")
        } else if l.contains('3') {
            ("platinen", "class 3")
        } else {
            return None;
        }
    } else if l.contains("motor") {
        ("elektromotoren", "")
    } else if l.contains("carbide") || l.contains("widia") || l.contains("tungsten") {
        if l.contains("widia") || l.contains("indexable") {
            ("hartmetall", "Widia inserts")
        } else if l.contains("drill") || l.contains("cutter") {
            ("hartmetall", "drills and cutters")
        } else if l.contains("saw") {
            ("hartmetall", "saw blades")
        } else {
            ("hartmetall", "mixed")
        }
    } else if l.contains("hss") {
        ("hss-werkzeuge", "")
    } else if l.contains("v4a") {
        if l.contains("chip") {
            ("edelstahl-v4a", "chips")
        } else {
            ("edelstahl-v4a", "")
        }
    } else if l.contains("v2a") {
        if l.contains("chip") {
            ("edelstahl-v2a", "chips")
        } else {
            ("edelstahl-v2a", "")
        }
    } else if l.contains("chrome") {
        ("edelstahl-gemischt", "chrome steel")
    } else if l.contains("phone") || l.contains("smartphone") || l.contains("handy") {
        // Whole devices → handys (boards handled above).
        return Some(("handys", ""));
    } else if l.contains("hard disk") || l.contains("drive") {
        return None;
    } else if l.contains("power") {
        // "Power supplies".
        return None;
    } else if l.contains("pcs") {
        // "PCs, unrobbed". ("piece" in "Ms58, new, piece" is routed above
        // and contains no "pcs".)
        return None;
    } else if l.contains("transformer") || l.contains("trafo") {
        return None;
    } else if l.contains("electronic scrap") {
        return None;
    } else if l.contains("silver") {
        // "Silver contacts": catalog `silber` is EUR/g, page quotes €/kg.
        return None;
    } else {
        return None;
    };
    // The variant always carries the tier plus the grade detail, so
    // same-material grades at different prices never collapse. Only live
    // (detail, tier) pairs exist — anything else is a redesign and skips
    // loudly instead of minting mystery variants.
    let variant: &'static str = match (detail, tier) {
        ("", "up to 25 kg") => "up to 25 kg",
        ("", "from 25 kg") => "from 25 kg",
        ("", "from 250 kg") => "from 250 kg",
        ("", "from 1000 kg") => "from 1000 kg",
        ("", "from 10 kg") => "from 10 kg",
        ("", "from 100 kg") => "from 100 kg",
        ("", "from 500 kg") => "from 500 kg",
        ("", "from 5 kg") => "from 5 kg",
        ("", "from 5000 kg") => "from 5000 kg",
        ("new", "from 100 kg") => "new, from 100 kg",
        ("new", "from 5000 kg") => "new, from 5000 kg",
        ("candle", "up to 25 kg") => "candle, up to 25 kg",
        ("candle", "from 25 kg") => "candle, from 25 kg",
        ("candle", "from 250 kg") => "candle, from 250 kg",
        ("candle", "from 1000 kg") => "candle, from 1000 kg",
        ("Raff, old 95 %", "up to 25 kg") => "Raff, old 95 %, up to 25 kg",
        ("Raff, old 95 %", "from 25 kg") => "Raff, old 95 %, from 25 kg",
        ("Raff, old 95 %", "from 250 kg") => "Raff, old 95 %, from 250 kg",
        ("Raff, old 95 %", "from 1000 kg") => "Raff, old 95 %, from 1000 kg",
        ("light", "up to 25 kg") => "light, up to 25 kg",
        ("light", "from 25 kg") => "light, from 25 kg",
        ("light", "from 250 kg") => "light, from 250 kg",
        ("light", "from 1000 kg") => "light, from 1000 kg",
        ("with plug, 25 % Cu", "up to 25 kg") => "with plug, 25 % Cu, up to 25 kg",
        ("with plug, 25 % Cu", "from 25 kg") => "with plug, 25 % Cu, from 25 kg",
        ("with plug, 25 % Cu", "from 250 kg") => "with plug, 25 % Cu, from 250 kg",
        ("with plug, 25 % Cu", "from 1000 kg") => "with plug, 25 % Cu, from 1000 kg",
        ("buried cable, 25 % Cu", "up to 25 kg") => "buried cable, 25 % Cu, up to 25 kg",
        ("buried cable, 25 % Cu", "from 25 kg") => "buried cable, 25 % Cu, from 25 kg",
        ("buried cable, 25 % Cu", "from 250 kg") => "buried cable, 25 % Cu, from 250 kg",
        ("buried cable, 25 % Cu", "from 1000 kg") => "buried cable, 25 % Cu, from 1000 kg",
        ("38 % Cu", "up to 25 kg") => "38 % Cu, up to 25 kg",
        ("38 % Cu", "from 25 kg") => "38 % Cu, from 25 kg",
        ("38 % Cu", "from 250 kg") => "38 % Cu, from 250 kg",
        ("38 % Cu", "from 1000 kg") => "38 % Cu, from 1000 kg",
        ("70 % Cu", "up to 25 kg") => "70 % Cu, up to 25 kg",
        ("70 % Cu", "from 25 kg") => "70 % Cu, from 25 kg",
        ("70 % Cu", "from 250 kg") => "70 % Cu, from 250 kg",
        ("70 % Cu", "from 1000 kg") => "70 % Cu, from 1000 kg",
        ("Ms58, new, piece", "up to 25 kg") => "Ms58, new, piece, up to 25 kg",
        ("Ms58, new, piece", "from 25 kg") => "Ms58, new, piece, from 25 kg",
        ("Ms58, new, piece", "from 250 kg") => "Ms58, new, piece, from 250 kg",
        ("Ms58, new, piece", "from 1000 kg") => "Ms58, new, piece, from 1000 kg",
        ("chips, mixed", "up to 25 kg") => "chips, mixed, up to 25 kg",
        ("chips, mixed", "from 25 kg") => "chips, mixed, from 25 kg",
        ("chips, mixed", "from 250 kg") => "chips, mixed, from 250 kg",
        ("chips, mixed", "from 1000 kg") => "chips, mixed, from 1000 kg",
        ("heavy", "up to 25 kg") => "heavy, up to 25 kg",
        ("heavy", "from 25 kg") => "heavy, from 25 kg",
        ("heavy", "from 250 kg") => "heavy, from 250 kg",
        ("heavy", "from 1000 kg") => "heavy, from 1000 kg",
        ("light, sleeves", "up to 25 kg") => "light, sleeves, up to 25 kg",
        ("light, sleeves", "from 25 kg") => "light, sleeves, from 25 kg",
        ("light, sleeves", "from 250 kg") => "light, sleeves, from 250 kg",
        ("light, sleeves", "from 1000 kg") => "light, sleeves, from 1000 kg",
        ("lumpy", "up to 25 kg") => "lumpy, up to 25 kg",
        ("lumpy", "from 25 kg") => "lumpy, from 25 kg",
        ("lumpy", "from 250 kg") => "lumpy, from 250 kg",
        ("lumpy", "from 1000 kg") => "lumpy, from 1000 kg",
        ("chips", "up to 25 kg") => "chips, up to 25 kg",
        ("chips", "from 25 kg") => "chips, from 25 kg",
        ("chips", "from 250 kg") => "chips, from 250 kg",
        ("chips", "from 1000 kg") => "chips, from 1000 kg",
        ("tableware", "up to 25 kg") => "tableware, up to 25 kg",
        ("tableware", "from 25 kg") => "tableware, from 25 kg",
        ("tableware", "from 250 kg") => "tableware, from 250 kg",
        ("tableware", "from 1000 kg") => "tableware, from 1000 kg",
        ("sheets, bright, new, 5000/6000", "up to 25 kg") => {
            "sheets, bright, new, 5000/6000, up to 25 kg"
        }
        ("sheets, bright, new, 5000/6000", "from 25 kg") => {
            "sheets, bright, new, 5000/6000, from 25 kg"
        }
        ("sheets, bright, new, 5000/6000", "from 250 kg") => {
            "sheets, bright, new, 5000/6000, from 250 kg"
        }
        ("sheets, bright, new, 5000/6000", "from 1000 kg") => {
            "sheets, bright, new, 5000/6000, from 1000 kg"
        }
        ("sheets, bright, new, alloyed", "up to 25 kg") => {
            "sheets, bright, new, alloyed, up to 25 kg"
        }
        ("sheets, bright, new, alloyed", "from 25 kg") => {
            "sheets, bright, new, alloyed, from 25 kg"
        }
        ("sheets, bright, new, alloyed", "from 250 kg") => {
            "sheets, bright, new, alloyed, from 250 kg"
        }
        ("sheets, bright, new, alloyed", "from 1000 kg") => {
            "sheets, bright, new, alloyed, from 1000 kg"
        }
        ("sheets, old, varnished", "up to 25 kg") => "sheets, old, varnished, up to 25 kg",
        ("sheets, old, varnished", "from 25 kg") => "sheets, old, varnished, from 25 kg",
        ("sheets, old, varnished", "from 250 kg") => "sheets, old, varnished, from 250 kg",
        ("sheets, old, varnished", "from 1000 kg") => "sheets, old, varnished, from 1000 kg",
        ("blank, new", "up to 25 kg") => "blank, new, up to 25 kg",
        ("blank, new", "from 25 kg") => "blank, new, from 25 kg",
        ("blank, new", "from 250 kg") => "blank, new, from 250 kg",
        ("blank, new", "from 1000 kg") => "blank, new, from 1000 kg",
        ("old", "up to 25 kg") => "old, up to 25 kg",
        ("old", "from 25 kg") => "old, from 25 kg",
        ("old", "from 250 kg") => "old, from 250 kg",
        ("old", "from 1000 kg") => "old, from 1000 kg",
        ("lacquered", "up to 25 kg") => "lacquered, up to 25 kg",
        ("lacquered", "from 25 kg") => "lacquered, from 25 kg",
        ("lacquered", "from 250 kg") => "lacquered, from 250 kg",
        ("lacquered", "from 1000 kg") => "lacquered, from 1000 kg",
        ("new", "up to 25 kg") => "new, up to 25 kg",
        ("new", "from 25 kg") => "new, from 25 kg",
        ("new", "from 250 kg") => "new, from 250 kg",
        ("new", "from 1000 kg") => "new, from 1000 kg",
        ("rims, clean", "up to 25 kg") => "rims, clean, up to 25 kg",
        ("rims, clean", "from 25 kg") => "rims, clean, from 25 kg",
        ("rims, clean", "from 250 kg") => "rims, clean, from 250 kg",
        ("rims, clean", "from 1000 kg") => "rims, clean, from 1000 kg",
        ("rims, affixed", "up to 25 kg") => "rims, affixed, up to 25 kg",
        ("rims, affixed", "from 25 kg") => "rims, affixed, from 25 kg",
        ("rims, affixed", "from 250 kg") => "rims, affixed, from 250 kg",
        ("rims, affixed", "from 1000 kg") => "rims, affixed, from 1000 kg",
        ("offset plates", "up to 25 kg") => "offset plates, up to 25 kg",
        ("offset plates", "from 25 kg") => "offset plates, from 25 kg",
        ("offset plates", "from 250 kg") => "offset plates, from 250 kg",
        ("offset plates", "from 1000 kg") => "offset plates, from 1000 kg",
        ("40 % Al", "up to 25 kg") => "40 % Al, up to 25 kg",
        ("40 % Al", "from 25 kg") => "40 % Al, from 25 kg",
        ("40 % Al", "from 250 kg") => "40 % Al, from 250 kg",
        ("40 % Al", "from 1000 kg") => "40 % Al, from 1000 kg",
        ("buried cable", "up to 25 kg") => "buried cable, up to 25 kg",
        ("buried cable", "from 25 kg") => "buried cable, from 25 kg",
        ("buried cable", "from 250 kg") => "buried cable, from 250 kg",
        ("buried cable", "from 1000 kg") => "buried cable, from 1000 kg",
        ("soft", "up to 10 kg") => "soft, up to 10 kg",
        ("soft", "from 10 kg") => "soft, from 10 kg",
        ("soft", "from 100 kg") => "soft, from 100 kg",
        ("soft", "from 500 kg") => "soft, from 500 kg",
        ("mixed", "up to 10 kg") => "mixed, up to 10 kg",
        ("mixed", "from 10 kg") => "mixed, from 10 kg",
        ("mixed", "from 100 kg") => "mixed, from 100 kg",
        ("mixed", "from 500 kg") => "mixed, from 500 kg",
        ("balancing weights", "up to 50 kg") => "balancing weights, up to 50 kg",
        ("balancing weights", "from 50 kg") => "balancing weights, from 50 kg",
        ("balancing weights", "from 300 kg") => "balancing weights, from 300 kg",
        ("balancing weights", "from 1000 kg") => "balancing weights, from 1000 kg",
        ("pure, 99 %", "up to 25 kg") => "pure, 99 %, up to 25 kg",
        ("pure, 99 %", "from 25 kg") => "pure, 99 %, from 25 kg",
        ("pure, 99 %", "from 100 kg") => "pure, 99 %, from 100 kg",
        ("pure, 99 %", "from 250 kg") => "pure, 99 %, from 250 kg",
        ("solder", "up to 25 kg") => "solder, up to 25 kg",
        ("solder", "from 25 kg") => "solder, from 25 kg",
        ("solder", "from 100 kg") => "solder, from 100 kg",
        ("solder", "from 250 kg") => "solder, from 250 kg",
        ("harness tin", "up to 25 kg") => "harness tin, up to 25 kg",
        ("harness tin", "from 25 kg") => "harness tin, from 25 kg",
        ("harness tin", "from 100 kg") => "harness tin, from 100 kg",
        ("harness tin", "from 250 kg") => "harness tin, from 250 kg",
        ("class 1A", "from 5 kg") => "class 1A, from 5 kg",
        ("class 1A", "from 30 kg") => "class 1A, from 30 kg",
        ("class 1A", "from 100 kg") => "class 1A, from 100 kg",
        ("class 2 / 1B", "from 5 kg") => "class 2 / 1B, from 5 kg",
        ("class 2 / 1B", "from 30 kg") => "class 2 / 1B, from 30 kg",
        ("class 2 / 1B", "from 100 kg") => "class 2 / 1B, from 100 kg",
        ("class 3", "from 15 kg") => "class 3, from 15 kg",
        ("class 3", "from 30 kg") => "class 3, from 30 kg",
        ("class 3", "from 100 kg") => "class 3, from 100 kg",
        ("RAM, gold contacts", "from 5 kg") => "RAM, gold contacts, from 5 kg",
        ("RAM, gold contacts", "from 30 kg") => "RAM, gold contacts, from 30 kg",
        ("RAM, gold contacts", "from 100 kg") => "RAM, gold contacts, from 100 kg",
        ("RAM, silver contacts", "from 5 kg") => "RAM, silver contacts, from 5 kg",
        ("RAM, silver contacts", "from 30 kg") => "RAM, silver contacts, from 30 kg",
        ("RAM, silver contacts", "from 100 kg") => "RAM, silver contacts, from 100 kg",
        ("phone boards", "from 5 kg") => "phone boards, from 5 kg",
        ("phone boards", "from 50 kg") => "phone boards, from 50 kg",
        ("phone boards", "from 200 kg") => "phone boards, from 200 kg",
        ("drills and cutters", "from 5 kg") => "drills and cutters, from 5 kg",
        ("drills and cutters", "from 100 kg") => "drills and cutters, from 100 kg",
        ("drills and cutters", "from 350 kg") => "drills and cutters, from 350 kg",
        ("Widia inserts", "from 5 kg") => "Widia inserts, from 5 kg",
        ("Widia inserts", "from 100 kg") => "Widia inserts, from 100 kg",
        ("Widia inserts", "from 350 kg") => "Widia inserts, from 350 kg",
        ("mixed", "from 5 kg") => "mixed, from 5 kg",
        ("mixed", "from 350 kg") => "mixed, from 350 kg",
        ("saw blades", "from 5 kg") => "saw blades, from 5 kg",
        ("saw blades", "from 100 kg") => "saw blades, from 100 kg",
        ("saw blades", "from 350 kg") => "saw blades, from 350 kg",
        ("chrome steel", "up to 25 kg") => "chrome steel, up to 25 kg",
        ("chrome steel", "from 25 kg") => "chrome steel, from 25 kg",
        ("chrome steel", "from 100 kg") => "chrome steel, from 100 kg",
        ("chrome steel", "from 250 kg") => "chrome steel, from 250 kg",
        ("chrome steel", "from 1000 kg") => "chrome steel, from 1000 kg",
        ("chips", "from 100 kg") => "chips, from 100 kg",
        _ => return None,
    };
    Some((material, variant))
}

/// Canonical tier key for a live quantity header. Only the headers in this
/// table may pass — a renamed column skips its cells loudly at the call
/// site instead of minting mystery variants.
fn tier_key(raw: &str) -> Option<&'static str> {
    match raw.trim().to_lowercase().as_str() {
        "up to 25 kg" => Some("up to 25 kg"),
        "up to 10 kg" => Some("up to 10 kg"),
        "up to 50 kg" => Some("up to 50 kg"),
        "from 3 kg" => Some("from 3 kg"),
        "from 5 kg" => Some("from 5 kg"),
        "from 10 kg" => Some("from 10 kg"),
        "from 15 kg" => Some("from 15 kg"),
        "from 20 kg" => Some("from 20 kg"),
        "from 25 kg" => Some("from 25 kg"),
        "from 30 kg" => Some("from 30 kg"),
        "from 50 kg" => Some("from 50 kg"),
        "from 100 kg" => Some("from 100 kg"),
        "from 200 kg" => Some("from 200 kg"),
        "from 250 kg" => Some("from 250 kg"),
        "from 300 kg" => Some("from 300 kg"),
        "from 350 kg" => Some("from 350 kg"),
        "from 500 kg" => Some("from 500 kg"),
        "from 1000 kg" => Some("from 1000 kg"),
        "from 5000 kg" => Some("from 5000 kg"),
        // Live typo in the zinc block ("to 250 kg" between "from 25 kg"
        // and "from 1000 kg", values rising monotonically): same tier as
        // the "from 250 kg" columns elsewhere, so variants stay stable
        // when the typo is fixed.
        "to 250 kg" => Some("from 250 kg"),
        _ => None,
    }
}

fn parse(
    html: &str,
) -> Result<
    (
        Option<String>,
        Vec<(String, &'static str, f64, &'static str)>,
        Vec<String>,
    ),
    IngestError,
> {
    // Window: the price blocks between the purchaseprices div and </main>.
    // The footer below holds its own contact + opening-hours table that
    // must never pair with prices.
    let start = html
        .find("purchaseprices")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Einkaufspreis-Block fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("</main>").unwrap_or(tail.len());
    let window = &tail[..end];
    let published_at = page_date(window);
    let doc = Html::parse_fragment(window);
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
    // Never trust page order: take the tables carrying the quantity
    // header, not just the first <table> on the page (the footer hours
    // table has empty headers and stays out).
    let tables: Vec<_> = doc
        .select(&table)
        .filter(|t| {
            t.select(&head).any(|h| {
                h.text()
                    .collect::<String>()
                    .to_lowercase()
                    .contains("quantity")
            })
        })
        .collect();
    if tables.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    }
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for table in tables {
        // Column tiers come from the thead Quantity row; further
        // Quantity rows inside the body (zinc → lead → …) switch them.
        let mut tiers: Vec<(String, Option<&'static str>)> = Vec::new();
        if let Some(head_row) = table
            .select(&row)
            .find(|r| r.select(&head).next().is_some())
        {
            let heads: Vec<String> = head_row
                .select(&head)
                .map(|h| clean(&h.text().collect::<String>()))
                .collect();
            if heads
                .first()
                .is_some_and(|h| h.to_lowercase().starts_with("quantity"))
            {
                tiers = heads[1..]
                    .iter()
                    .map(|h| (h.clone(), tier_key(h)))
                    .collect();
            }
        }
        for tr in table.select(&row) {
            if tr.select(&head).next().is_some() {
                continue; // header row, not data
            }
            let cells: Vec<String> = tr
                .select(&cell)
                .map(|c| clean(&c.text().collect::<String>()))
                .collect();
            if cells.iter().all(|c| c.is_empty()) {
                continue; // separator row
            }
            if cells[0].to_lowercase().starts_with("quantity") {
                tiers = cells[1..]
                    .iter()
                    .map(|h| (h.clone(), tier_key(h)))
                    .collect();
                continue;
            }
            if cells[0].is_empty() {
                skipped.push(format!(
                    "Tabellenzeile ohne Bezeichnung: {}",
                    cells.join(" / ")
                ));
                continue;
            }
            if tiers.is_empty() {
                skipped.push(format!("{} (Preisstaffel fehlt)", cells[0]));
                continue;
            }
            let label = cells[0].clone();
            // Short rows (e.g. the "Buried cable muff" surcharge note with
            // 3 cells under 4 tiers) only pair what the page shows.
            for ((tier_raw, tier), cell) in tiers.iter().zip(cells[1..].iter()) {
                if cell.is_empty() {
                    skipped.push(format!("{label} ({tier_raw}: kein Preis)"));
                    continue;
                }
                let Some(price) = parse_eur(cell) else {
                    skipped.push(format!(
                        "{label} ({tier_raw}: Preis unverständlich: {cell})"
                    ));
                    continue;
                };
                // A "0,00" row is "no quote", not a free gift: loud skip.
                if price == 0.0 {
                    skipped.push(format!("{label} ({tier_raw}: Preis 0,00)"));
                    continue;
                }
                let Some(tier) = tier else {
                    skipped.push(format!("{label} (Preisstaffel unverständlich: {tier_raw})"));
                    continue;
                };
                // No per-row unit is quoted on this page; every live value
                // is market-plausible per kg (see module docs), so EUR/kg
                // is the documented page default. record() normalizes into
                // the EUR/t catalog units for steel grades downstream.
                rows.push((label.clone(), *tier, price, "EUR/kg"));
            }
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    Ok((published_at, rows, skipped))
}

/// Page-stated validity date ("Metal purchase prices as of 28.09.2026").
/// No date → None (`observed_at` carries the age instead).
fn page_date(window: &str) -> Option<String> {
    let anchor = "purchase prices as of";
    let i = window.to_lowercase().find(anchor)?;
    let tail = window[i + anchor.len()..].trim_start().to_string();
    let tok = tail.split_whitespace().next()?;
    let tok = tok.split('<').next().unwrap_or(tok);
    let mut parts = tok.split('.');
    parse_de_date(parts.next()?, parts.next()?, parts.next()?)
}

/// Collapse HTML text to single-spaced words (scraper `text()` keeps the
/// figure/link nodes' empty strings; `&nbsp;` arrives decoded as \u{a0}).
fn clean(raw: &str) -> String {
    raw.replace(['\u{a0}'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` naming
/// the firm ("Huth Schrott – Metall GmbH / Hafenkopfstraße 7 /
/// 63741 Aschaffenburg") plus the `<p>` after the "Kontakt:" line
/// ("Telefon:" row, bare `info@…` address). Anchored on the `h1`
/// "Impressum" heading — missing anchors → loud error, never a guessed
/// fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    if !doc
        .select(&h1)
        .any(|h| h.text().collect::<String>().contains("Impressum"))
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let mut found_firm = false;
    for el in doc.select(&p) {
        let lines = block_lines(&el.inner_html());
        if !lines.iter().any(|l| l.contains("Huth Schrott")) {
            continue;
        }
        found_firm = true;
        for (k, line) in lines.iter().enumerate() {
            let mut it = line.split_whitespace();
            if let (Some(pc), Some(_)) = (it.next(), it.next()) {
                if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    city = line[pc.len()..].trim().to_owned();
                    if k > 0 {
                        street = lines[k - 1].clone();
                    }
                    break;
                }
            }
        }
        break;
    }
    if !found_firm {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmen-Block fehlt".to_owned(),
        });
    }
    let mut phone = String::new();
    let mut email = String::new();
    let mut found_kontakt = false;
    for el in doc.select(&p) {
        let lines = block_lines(&el.inner_html());
        if !lines.iter().any(|l| l.starts_with("Kontakt:")) {
            continue;
        }
        found_kontakt = true;
        for line in &lines {
            if let Some(v) = line.strip_prefix("Telefon:") {
                // Phone needs token filtering; the e-mail rule below is
                // separate (a phone-style filter stops at the first
                // letter, swallowing nothing of an address — and vice
                // versa a letter-blind split would keep fax noise).
                phone = v
                    .split_whitespace()
                    .take_while(|t| {
                        t.chars()
                            .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
            }
            if email.is_empty() {
                if let Some(tok) = line.split_whitespace().find(|t| t.contains('@')) {
                    email = tok.trim_matches([',', ';', '.']).to_owned();
                }
            }
        }
        break;
    }
    if !found_kontakt {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
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

/// Split an inner-HTML block on `<br` into decoded text lines (entities on
/// this page arrive as UTF-8; only `&nbsp;`/`&#160;` need normalizing).
fn block_lines(inner: &str) -> Vec<String> {
    inner
        .replace("<br", "\n")
        .split('\n')
        .map(|part| {
            let part = part.trim_start_matches("/>").trim_start_matches('>').trim();
            let mut out = String::new();
            let mut in_tag = false;
            for c in part.chars() {
                if c == '<' {
                    in_tag = true;
                } else if c == '>' {
                    in_tag = false;
                } else if !in_tag {
                    out.push(c);
                }
            }
            out.replace("&nbsp;", " ")
                .replace("&#160;", " ")
                .replace("&amp;", "&")
        })
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, tier_key};

    // Real excerpts of the live page (28.09.2026): verbatim tags, headers
    // (incl. the zinc "to 250 kg" typo), labels, hrefs and prices; trimmed
    // to representative rows across all five tier shapes plus the surcharge
    // note, an empty separator row and the footer hours table.
    const FIXTURE: &str = "<div class=\"purchaseprices\">\
        <p><strong>Metal purchase prices as of 28.09.2026</strong><br />all information is subject to change</p>\
        <h3>Cu / Ms</h3>\
        <table><thead><tr><th>Quantity:</th><th>up to 25 kg</th><th>from 25 kg</th><th>from 250 kg</th><th>from 1000 kg</th></tr></thead>\
        <tbody>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/kupfer-blank-i-millberry\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Copper, bare I, Millberry</td><td>9,91</td><td>11,16</td><td>11,31</td><td>11,86</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/kupfer-blank-ii-kerze\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Copper, bare II, candle</td><td>9,56</td><td>10,76</td><td>11,06</td><td>11,36</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/kupfer-gemischt-raff-alt-95-berry\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Copper, mixed, Raff, old 95 %, Berry</td><td>9,36</td><td>10,51</td><td>10,86</td><td>11,16</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/kupfer-leicht\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Copper, light</td><td>8,86</td><td>9,41</td><td>9,86</td><td>10,36</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/kupfer-messing-kuhler-mit-max-5-anh\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Copper-brass cooler, max. 5 % adhesions</td><td>2,31</td><td>2,56</td><td>3,01</td><td>3,65</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/kupferkabel-mit-stecker-und-kupferkabel-25\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Copper cable with plug and copper cable, min. 25 % Cu</td><td>1,02</td><td>1,22</td><td>1,47</td><td>1,79</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/ms-schwer\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Brass, heavy</td><td>5,20</td><td>5,40</td><td>5,70</td><td>6,30</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/rotguss-spane\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Gunmetal chips</td><td>8,00</td><td>8,80</td><td>9,00</td><td>9,20</td></tr>\
        <tr><td></td><td></td><td></td><td></td><td></td></tr>\
        </tbody></table>\
        <table><thead><tr><th>Quantity:</th><th>up to 25 kg</th><th>from 25 kg</th><th>to 250 kg</th><th>from 1000 kg</th></tr></thead>\
        <tbody>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/zink-neu\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Zinc, new</td><td>2,10</td><td>2,20</td><td>2,30</td><td>2,50</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/zink-alt\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Zinc, old</td><td>1,65</td><td>1,95</td><td>2,05</td><td>2,20</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/Erdkabel-Muffern\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Buried cable muff</td><td>additional payment</td><td>100 €</td><td>/ to</td></tr>\
        <tr><td>Quantity:</td><td>up to 10 kg</td><td>from 10 kg</td><td>from 100 kg</td><td>from 500 kg</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/altblei-weich-weichblei-o-anhaftung\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Old lead, soft, soft lead without adhesions</td><td>0,60</td><td>0,75</td><td>0,83</td><td>1,00</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/altblei-gemischt\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Old lead, mixed</td><td>0,52</td><td>0,67</td><td>0,75</td><td>0,91</td></tr>\
        </tbody></table>\
        <table><thead><tr><th>Quantity:</th><th>up to 25 kg</th><th>from 25 kg</th><th>from 100 kg</th><th>from 250 kg</th><th>from 1000 kg</th></tr></thead>\
        <tbody>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/edelstahl-v2a-nirosta-18-8\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Stainless steel, V2A, Nirosta, 18/8</td><td>0,50</td><td>0,70</td><td>0,78</td><td>0,85</td><td>1,03</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/chromstahl\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Chrome steel</td><td>0,03</td><td>0,08</td><td>0,13</td><td>0,16</td><td>0,18</td></tr>\
        </tbody></table>\
        <table><thead><tr><th>Quantity:</th><th>up to 25 kg</th><th>from 25 kg</th><th>from 100 kg</th><th>from 1000 kg</th><th>from 5000 kg</th></tr></thead>\
        <tbody>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/shreddervormaterial\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Shredder input material / Adhesions &gt; 10 % to be charged as waste</td><td>0,05</td><td>0,06</td><td>0,08</td><td>0,09</td><td>0,12</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/schwerschrott\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Heavy scrap / all adhesions to be accounted for as waste</td><td>0,07</td><td>0,09</td><td>0,15</td><td>0,19</td><td>021</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/neuschrott-handelsguss\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>New scrap / commercial castings / all adhesions to be accounted for as waste</td><td>0,06</td><td>0,09</td><td>0,17</td><td>0,19</td><td>0,21</td></tr>\
        </tbody></table>\
        <table><thead><tr><th>Quantity:</th><th>from 5 kg</th><th>from 30 kg</th><th>from 100 kg</th></tr></thead>\
        <tbody>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/leiterplatten-klasse-1-a\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Printed circuit boards, class 1A</td><td>4,50</td><td>5,40</td><td>5,60</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/cpu-keramik-gemischt\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>CPU, ceramic, mixed</td><td>50,00</td><td>55,00</td><td>66,00</td></tr>\
        <tr><td><figure><a href=\"https://www.huth-recycling.de/einkaufspreise/silberkontakte\"><img alt=\"\" src=\"https://www.huth-recycling.de/media/pages/einkaufspreise/fc882253aa-1717406504/info.svg\"></a></figure>Silver contacts</td><td>3,00</td><td>4,00</td><td>5,00</td></tr>\
        </tbody></table>\
        </div></main>";

    #[test]
    fn tiers_date_and_loud_skips() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-28T00:00:00+00:00"));
        // 8×4 (Cu/Ms) + zinc 8 + muff 1 + lead 8 + VA 10 + Fe 15 + boards 9.
        assert_eq!(rows.len(), 83);
        // The muff surcharge note has no parseable prices in two cells.
        assert_eq!(skips.len(), 2);
        assert!(skips.iter().any(|s| s.contains("Buried cable muff")
            && s.contains("Preis unverständlich")
            && s.contains("additional payment")));
        assert!(skips.iter().any(|s| s.contains("Buried cable muff")
            && s.contains("Preis unverständlich")
            && s.contains("/ to")));
        let mill: Vec<_> = rows
            .iter()
            .filter(|(l, _, _, _)| l == "Copper, bare I, Millberry")
            .collect();
        assert_eq!(mill.len(), 4);
        assert_eq!((mill[0].1, mill[0].2), ("up to 25 kg", 9.91));
        assert_eq!((mill[3].1, mill[3].2), ("from 1000 kg", 11.86));
        assert!(rows.iter().all(|(_, _, _, u)| *u == "EUR/kg"));
        // The zinc "to 250 kg" typo resolves to the canonical tier.
        let zn: Vec<_> = rows
            .iter()
            .filter(|(l, _, _, _)| l == "Zinc, new")
            .collect();
        assert_eq!(zn.len(), 4);
        assert_eq!((zn[2].1, zn[2].2), ("from 250 kg", 2.3));
        // Tier switch mid-table: lead rows use the second Quantity header.
        let pb: Vec<_> = rows
            .iter()
            .filter(|(l, _, _, _)| l == "Old lead, mixed")
            .collect();
        assert_eq!(pb.len(), 4);
        assert_eq!((pb[0].1, pb[0].2), ("up to 10 kg", 0.52));
        assert_eq!((pb[3].1, pb[3].2), ("from 500 kg", 0.91));
        // Five-column VA and Fe blocks pair all tiers.
        let v2a: Vec<_> = rows
            .iter()
            .filter(|(l, _, _, _)| l == "Stainless steel, V2A, Nirosta, 18/8")
            .collect();
        assert_eq!(v2a.len(), 5);
        assert_eq!((v2a[4].1, v2a[4].2), ("from 1000 kg", 1.03));
        // No two fixture rows collapse onto one (material, variant): the
        // tier in every variant keeps grades distinct.
        let mut seen = std::collections::HashSet::new();
        let mut mapped = 0;
        for (label, tier, _, _) in &rows {
            if let Some((material, variant)) = grade_for(label, tier) {
                mapped += 1;
                assert!(seen.insert((material, variant)), "{material} {variant}");
            }
        }
        assert_eq!(mapped, 67);
        // The "021" typo cell parses as a number but its row has no
        // catalog entry, so it can only ever become a loud grade skip.
        let heavy: Vec<_> = rows
            .iter()
            .filter(|(l, _, _, _)| l.starts_with("Heavy scrap"))
            .collect();
        assert_eq!(heavy.len(), 5);
        assert_eq!(grade_for(&heavy[4].0, heavy[4].1), None);
    }

    #[test]
    fn empty_and_zero_price_cells_skip_loudly() {
        let html = FIXTURE.replacen("<td>9,91</td>", "<td></td>", 1).replacen(
            "<td>9,56</td>",
            "<td>0,00</td>",
            1,
        );
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 81);
        assert_eq!(skips.len(), 4);
        assert!(skips
            .iter()
            .any(|s| s.contains("Millberry") && s.contains("kein Preis")));
        assert!(skips
            .iter()
            .any(|s| s.contains("candle") && s.contains("0,00")));
    }

    #[test]
    fn wrong_tables_and_columns_reject_loudly() {
        // A layout table before the price tables must not win.
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (_, rows, _) = parse(&html).expect("finds the price tables");
        assert_eq!(rows.len(), 83);
        // Footer hours table (empty headers) is excluded by selection.
        assert!(!rows.iter().any(|(l, _, _, _)| l.contains("Montag")));
        // No quantity header at all: loud error.
        let html = FIXTURE.replace("Quantity:", "Menge:");
        assert!(parse(&html).is_err());
        // Missing price window: loud error, not silent success.
        assert!(parse("<html><body><p>Neu hier</p></body></html>").is_err());
        // Every cell empty: loud error, not silent success.
        let html = FIXTURE
            .replace("<td>9,91</td>", "<td></td>")
            .replace("<td>11,16</td>", "<td></td>")
            .replace("<td>11,31</td>", "<td></td>")
            .replace("<td>11,86</td>", "<td></td>");
        // "11,16" appears twice (Millberry ab 25 kg, Raff ab 1000 kg), so
        // emptying the four Millberry cells removes five pairings.
        let (_, rows, _) = parse(&html).expect("other rows still parse");
        assert_eq!(rows.len(), 78);
        // Unknown tier header skips its cells loudly instead of minting
        // mystery variants.
        let html = FIXTURE.replacen("from 1000 kg", "from 2000 kg", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert!(skips
            .iter()
            .any(|s| s.contains("Preisstaffel unverständlich") && s.contains("from 2000 kg")));
        let _ = rows;
        assert_eq!(tier_key("to 250 kg"), Some("from 250 kg"));
        assert_eq!(tier_key("from 200 kg"), Some("from 200 kg"));
        assert_eq!(tier_key("from 2000 kg"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        // Real fragment shape: h1 anchor, TMG firm block, Kontakt block
        // with bare (non-mailto) address.
        let imp = "<h1>Impressum</h1><p>Angaben gemäß § 5 TMG:</p>\
            <p>Huth Schrott – Metall GmbH<br />Hafenkopfstraße 7<br />63741 Aschaffenburg</p>\
            <p>Vertreten durch:<br />Viktoria Westarp</p>\
            <p>Kontakt:<br />Telefon: +49 (0)6021/412323<br />Telefax: +49 (0)6021/412300<br />\
            info@huth-recycling.de<br />Eintragung im Handelsregister<br />\
            Registergericht: Amtsgericht Aschaffenburg<br />HRB Nr. 5758</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Hafenkopfstraße 7");
        assert_eq!(info.postcode, "63741");
        assert_eq!(info.city, "Aschaffenburg");
        assert_eq!(info.phone, "+49 (0)6021/412323");
        assert_eq!(info.email, "info@huth-recycling.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<h1>Neu</h1><p>x</p>").is_err());
        assert!(extract_info("<h1>Impressum</h1><p>Neu hier</p>").is_err());
    }

    #[test]
    fn mapping_covers_every_live_label() {
        // Every live label maps (or loudly skips), with tier- and
        // grade-distinct variants so same-material rows never collapse.
        assert_eq!(
            grade_for("Copper, bare I, Millberry", "up to 25 kg"),
            Some(("kupfer-millberry", "up to 25 kg"))
        );
        assert_eq!(
            grade_for("Copper, bare I, Millberry", "from 1000 kg"),
            Some(("kupfer-millberry", "from 1000 kg"))
        );
        assert_eq!(
            grade_for("Copper, bare II, candle", "from 25 kg"),
            Some(("kupfer-berry", "candle, from 25 kg"))
        );
        assert_eq!(
            grade_for("Copper, mixed, Raff, old 95 %, Berry", "from 250 kg"),
            Some(("kupfer-gemischt", "Raff, old 95 %, from 250 kg"))
        );
        assert_eq!(
            grade_for("Copper, light", "up to 25 kg"),
            Some(("kupfer-gemischt", "light, up to 25 kg"))
        );
        assert_eq!(
            grade_for(
                "Copper cable with plug and copper cable, min. 25 % Cu",
                "from 25 kg"
            ),
            Some(("kabel-mit-stecker", "with plug, 25 % Cu, from 25 kg"))
        );
        assert_eq!(
            grade_for("Copper buried cable, at least 25 % Cu", "from 250 kg"),
            Some(("kabel-kupfer", "buried cable, 25 % Cu, from 250 kg"))
        );
        assert_eq!(
            grade_for("Copper cable, at least 38 % Cu", "from 25 kg"),
            Some(("kabel-kupfer", "38 % Cu, from 25 kg"))
        );
        assert_eq!(
            grade_for("Copper cable, at least 70 % Cu", "from 1000 kg"),
            Some(("kabel-kupfer", "70 % Cu, from 1000 kg"))
        );
        assert_eq!(
            grade_for("Brass Ms58, new, piece", "up to 25 kg"),
            Some(("messing", "Ms58, new, piece, up to 25 kg"))
        );
        assert_eq!(
            grade_for("Brass chips, mixed, max. 5 % impurities", "from 25 kg"),
            Some(("messing", "chips, mixed, from 25 kg"))
        );
        assert_eq!(
            grade_for("Brass, heavy", "from 1000 kg"),
            Some(("messing", "heavy, from 1000 kg"))
        );
        assert_eq!(
            grade_for(
                "Brass, light and sleeves (separate), brass with hoses",
                "from 250 kg"
            ),
            Some(("messing", "light, sleeves, from 250 kg"))
        );
        assert_eq!(
            grade_for("Gunmetal, lumpy", "up to 25 kg"),
            Some(("bronze-rotguss", "lumpy, up to 25 kg"))
        );
        assert_eq!(
            grade_for("Gunmetal chips", "from 1000 kg"),
            Some(("bronze-rotguss", "chips, from 1000 kg"))
        );
        assert_eq!(
            grade_for("Aluminium tableware, max. 5 % adhesions", "from 25 kg"),
            Some(("aluminium-blech", "tableware, from 25 kg"))
        );
        assert_eq!(
            grade_for("Aluminium sheets, bright, new, 5000 or 6000", "up to 25 kg"),
            Some((
                "aluminium-blech",
                "sheets, bright, new, 5000/6000, up to 25 kg"
            ))
        );
        assert_eq!(
            grade_for(
                "Aluminium sheets, bright, new, with Sn, Zn, Cu, Mn etc.",
                "from 250 kg"
            ),
            Some((
                "aluminium-blech",
                "sheets, bright, new, alloyed, from 250 kg"
            ))
        );
        assert_eq!(
            grade_for("Aluminium sheets, old, varnished", "from 1000 kg"),
            Some(("aluminium-blech", "sheets, old, varnished, from 1000 kg"))
        );
        assert_eq!(
            grade_for("Aluminium profiles, blank, new", "from 25 kg"),
            Some(("aluminium-profile", "blank, new, from 25 kg"))
        );
        assert_eq!(
            grade_for(
                "Aluminium profiles, old / finishing profiles minus adhesions",
                "up to 25 kg"
            ),
            Some(("aluminium-profile", "old, up to 25 kg"))
        );
        assert_eq!(
            grade_for("Aluminium profiles, lacquered", "from 1000 kg"),
            Some(("aluminium-profile", "lacquered, from 1000 kg"))
        );
        assert_eq!(
            grade_for("Cast aluminium, new, without adhesions", "up to 25 kg"),
            Some(("aluminium-guss", "new, up to 25 kg"))
        );
        assert_eq!(
            grade_for("Cast aluminium, old, max. 5 % adherence", "from 250 kg"),
            Some(("aluminium-guss", "old, from 250 kg"))
        );
        assert_eq!(
            grade_for("Aluminium rims, clean", "from 25 kg"),
            Some(("aluminium-guss", "rims, clean, from 25 kg"))
        );
        assert_eq!(
            grade_for("Aluminium rims, affixed", "from 1000 kg"),
            Some(("aluminium-guss", "rims, affixed, from 1000 kg"))
        );
        assert_eq!(
            grade_for("Aluminium offset plates", "from 250 kg"),
            Some(("aluminium-blech", "offset plates, from 250 kg"))
        );
        assert_eq!(
            grade_for("Aluminium cable, 40 % Al", "up to 25 kg"),
            Some(("kabel-alu", "40 % Al, up to 25 kg"))
        );
        assert_eq!(
            grade_for("Aluminium buried cable", "from 1000 kg"),
            Some(("kabel-alu", "buried cable, from 1000 kg"))
        );
        assert_eq!(
            grade_for("Zinc, new", "from 250 kg"),
            Some(("zink", "new, from 250 kg"))
        );
        assert_eq!(
            grade_for("Zinc, old", "up to 25 kg"),
            Some(("zink", "old, up to 25 kg"))
        );
        assert_eq!(
            grade_for("Old lead, soft, soft lead without adhesions", "up to 10 kg"),
            Some(("blei", "soft, up to 10 kg"))
        );
        assert_eq!(
            grade_for("Old lead, mixed", "from 500 kg"),
            Some(("blei", "mixed, from 500 kg"))
        );
        assert_eq!(
            grade_for("Balancing lead, balancing weights Pb", "from 300 kg"),
            Some(("blei", "balancing weights, from 300 kg"))
        );
        assert_eq!(
            grade_for("Tin, pure, at least 99 %", "from 250 kg"),
            Some(("zinn", "pure, 99 %, from 250 kg"))
        );
        assert_eq!(
            grade_for("Tin scrap, solder", "up to 25 kg"),
            Some(("zinn", "solder, up to 25 kg"))
        );
        assert_eq!(
            grade_for("Harness tin", "from 100 kg"),
            Some(("zinn", "harness tin, from 100 kg"))
        );
        assert_eq!(
            grade_for("Printed circuit boards, class 1A", "from 5 kg"),
            Some(("platinen", "class 1A, from 5 kg"))
        );
        assert_eq!(
            grade_for("Printed circuit boards, class 2 and 1B", "from 30 kg"),
            Some(("platinen", "class 2 / 1B, from 30 kg"))
        );
        assert_eq!(
            grade_for("Printed circuit boards, class 3", "from 100 kg"),
            Some(("platinen", "class 3, from 100 kg"))
        );
        assert_eq!(
            grade_for("RAM, with gold contacts", "from 30 kg"),
            Some(("ram", "RAM, gold contacts, from 30 kg"))
        );
        assert_eq!(
            grade_for("RAM, with silver contacts", "from 100 kg"),
            Some(("ram", "RAM, silver contacts, from 100 kg"))
        );
        assert_eq!(
            grade_for("Mobile phone, smartphone boards", "from 200 kg"),
            Some(("handys", "phone boards, from 200 kg"))
        );
        assert_eq!(
            grade_for("Electronic motors, without gears etc.", "from 100 kg"),
            Some(("elektromotoren", "from 100 kg"))
        );
        assert_eq!(
            grade_for("Carbide, drills and cutters", "from 100 kg"),
            Some(("hartmetall", "drills and cutters, from 100 kg"))
        );
        assert_eq!(
            grade_for("Carbide, Widia indexable inserts", "from 5 kg"),
            Some(("hartmetall", "Widia inserts, from 5 kg"))
        );
        assert_eq!(
            grade_for("Tungsten carbide, mixed", "from 350 kg"),
            Some(("hartmetall", "mixed, from 350 kg"))
        );
        assert_eq!(
            grade_for("Carbide, saw blades", "from 350 kg"),
            Some(("hartmetall", "saw blades, from 350 kg"))
        );
        assert_eq!(
            grade_for("HSS, mixed (high-speed steel)", "from 500 kg"),
            Some(("hss-werkzeuge", "from 500 kg"))
        );
        assert_eq!(
            grade_for("Stainless steel, V2A, Nirosta, 18/8", "from 250 kg"),
            Some(("edelstahl-v2a", "from 250 kg"))
        );
        assert_eq!(
            grade_for("Stainless steel, V4A, scrap, 20/10/2", "from 100 kg"),
            Some(("edelstahl-v4a", "from 100 kg"))
        );
        assert_eq!(
            grade_for("Chrome steel", "up to 25 kg"),
            Some(("edelstahl-gemischt", "chrome steel, up to 25 kg"))
        );
        assert_eq!(
            grade_for("V2A-Chips", "from 1000 kg"),
            Some(("edelstahl-v2a", "chips, from 1000 kg"))
        );
        assert_eq!(
            grade_for("V4A-Chips", "up to 25 kg"),
            Some(("edelstahl-v4a", "chips, up to 25 kg"))
        );
        assert_eq!(
            grade_for(
                "Shredder input material / Adhesions > 10 % to be charged as waste",
                "from 5000 kg"
            ),
            Some(("stahlschrott-shredder", "from 5000 kg"))
        );
        assert_eq!(
            grade_for(
                "Mixed scrap / account for all adhesions as waste",
                "from 100 kg"
            ),
            Some(("mischschrott", "from 100 kg"))
        );
        assert_eq!(
            grade_for(
                "New scrap / commercial castings / all adhesions to be accounted for as waste",
                "from 25 kg"
            ),
            Some(("stahlschrott-sorte-1", "new, from 25 kg"))
        );
        // A foreign tier never mints a variant (redesign → loud skip).
        assert_eq!(grade_for("Copper, bare I, Millberry", "from 2000 kg"), None);
        // Loud skips: mixed coolers, surcharge note, batteries, chips and
        // devices without a catalog entry — plus the EUR/g trap.
        for label in [
            "Copper-brass cooler, max. 5 % adhesions",
            "Copper brass radiator, pure",
            "Aluminium cooler, pure",
            "Aluminium cooler, adhesive, minus adhesions",
            "Aluminium-copper cooler, pure",
            "Aluminium-copper cooler, adhesive, minus adhesions",
            "Buried cable muff",
            "Aluminium shavings, max. 5 % adherence",
            "Lead acid batteries",
            "CPU, plastic, adhesive",
            "CPU, ceramic, mixed",
            "CPU, plastic, mixed",
            "Slot CPU",
            "Mobile phones, smartphones without battery",
            "Hard disks",
            "Drives",
            "Power supplies",
            "PCs, unrobbed",
            "Small transformers",
            "Electronic scrap, mixed",
            "Silver contacts",
            "Heavy scrap / all adhesions to be accounted for as waste",
            "Chips, clean / all build-up to be accounted for as waste",
            "Goldbarren",
        ] {
            assert_eq!(grade_for(label, "up to 25 kg"), None, "{label}");
        }
    }
}
