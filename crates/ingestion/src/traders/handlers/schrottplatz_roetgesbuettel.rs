//! Schrottplatz Rötgesbüttel: Divi one-pager, the whole price list lives on
//! the homepage inside `#preisliste` as Blurb cards — no table, no iframe,
//! no calendar date ("Tagespreis" only). Each card pair is a label module
//! (`<h2>LABEL</h2> <p>Tagespreis</p>`) immediately followed by a price
//! module (`<h2>PRICE</h2>`), so pairing is positional: a price-h2 (digits
//! plus `€`/`Euro`) closes the pair with the previous label-h2. Exact
//! prices run at full confidence; `ab …` prices are floors ("from X
//! upwards depending on quality/quantity"), modelled as `approx` at 0.5 —
//! never `upto`, which would claim a ceiling the page does not state. The
//! footer footnote ("Preise nicht für kleine Mengen") confirms the vagueness.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "ni-rotgesbuttel-schrottplatz-rotgesbuttel";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://schrottplatz-roetgesbuettel.de/impressum/";

pub const URL: &str = "https://schrottplatz-roetgesbuettel.de/";

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
    for (label, price, unit, from_price) in rows {
        // Zero is not a price (placeholder card) — loud skip, never a row.
        if price <= 0.0 {
            skipped_labels.push(format!("{label} (kein Ankaufpreis)"));
            continue;
        }
        let Some((material, variant)) = grade_for(&label) else {
            skipped_labels.push(label);
            continue;
        };
        // "ab …" is a floor, not a ceiling: approx at 0.5 with no bound
        // fields (never price_max without an upto kind).
        let (price_kind, confidence) = if from_price {
            ("approx", Some(0.5))
        } else {
            ("exact", Some(1.0))
        };
        prices.push(ScrapedPrice {
            material,
            variant,
            price,
            currency: "EUR",
            unit,
            price_kind,
            price_min: None,
            price_max: None,
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
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

/// Parse the `#preisliste` card window into (label, price, unit, ab-flag).
/// Labels pair positionally: a price-h2 closes the pair with the previous
/// label-h2. Returns loud skips for unit-less or orphan prices.
fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str, bool)>, Vec<String>), IngestError> {
    // Window: cards live between the preisliste section anchor and the
    // small-quantities footnote (or the footer, whichever comes first).
    let start = html
        .find("id=\"preisliste\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "preisliste-Anker fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Bitte beachten Sie")
        .or_else(|| tail.find("main-footer"))
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preislisten-Ende fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    // Raw h2 scan in document order (Divi nests them too wildly for a
    // sibling walk; classes are layout noise, not anchors).
    let mut heads = Vec::new();
    let mut rest = window;
    while let Some(o) = rest.find("<h2") {
        let after = &rest[o..];
        let Some(gt) = after.find('>') else { break };
        let inner = &after[gt + 1..];
        let Some(close) = inner.find("</h2>") else {
            break;
        };
        heads.push(strip_fragment(&inner[..close]));
        rest = &inner[close + 5..];
    }
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    let mut pending: Option<String> = None;
    for h in heads {
        if is_price(&h) {
            if let Some(label) = pending.take() {
                let Some(price) = parse_eur(&h) else {
                    skips.push(format!("{label} (Preis unverständlich: {h})"));
                    continue;
                };
                let Some(unit) = unit_of(&h) else {
                    skips.push(format!("{label} (Einheit unverständlich: {h})"));
                    continue;
                };
                rows.push((label, price, unit, is_from_price(&h)));
            } else {
                skips.push(format!("(Preis ohne Label: {h})"));
            }
        } else if h.len() > 120 {
            // Prose heading, not a grade label.
            if let Some(label) = pending.take() {
                skips.push(format!("{label} (kein Preis)"));
            }
        } else if let Some(prev) = pending.take() {
            // Two labels in a row: the first never got a price card.
            skips.push(format!("{prev} (kein Preis)"));
            pending = Some(h);
        } else {
            pending = Some(h);
        }
    }
    if let Some(label) = pending {
        skips.push(format!("{label} (kein Preis)"));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preispaare".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// A price-h2 carries digits plus a currency marker. Grade labels like
/// "Haushaltskabel 38%" have digits but no currency — never prices.
fn is_price(h: &str) -> bool {
    let l = h.to_lowercase();
    (l.contains('€') || l.contains("euro")) && parse_eur(h).is_some()
}

/// "ab 1,30 € / kg" is a from-price (floor); anything else is exact.
fn is_from_price(h: &str) -> bool {
    h.trim_start().to_lowercase().starts_with("ab ")
}

/// Bespoke units for this page only: "Euro/kg" and "€ / kg" per kilo,
/// "€ / t*" per tonne. Anything else skips loudly — a tonne price filed
/// as per-kilo would be three orders off.
fn unit_of(price: &str) -> Option<&'static str> {
    let flat: String = price
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    if flat.contains("kg") {
        Some("EUR/kg")
    } else if flat.contains("/t") {
        Some("EUR/t")
    } else {
        None
    }
}

/// Explicit label → (material, variant) table, specific arms first.
/// Two copper grades share `kupfer-gemischt` (neither is blank Millberry
/// wire) and split by variant; cable grades split by copper share; brake
/// discs ride with cast breakage but keep their own variant.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("haushaltskabel") || (l.contains("kabel") && l.contains("38")) {
        Some(("kabel-kupfer", "38%"))
    } else if l.contains("kabel") && l.contains("50") {
        Some(("kabel-kupfer", "50%"))
    } else if l.contains("kabel") && l.contains("60") {
        Some(("kabel-kupfer", "60%"))
    } else if l.contains("kabel") && l.contains("75") {
        Some(("kabel-kupfer", "75%"))
    } else if l.contains("kerze") {
        // Second-best copper grade after Millberry, not blank wire:
        // generic mixed copper, raw grade in the variant.
        Some(("kupfer-gemischt", "Kerze"))
    } else if l.contains("kupfer") && l.contains("schwer") {
        Some(("kupfer-gemischt", "schwer"))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("bremsscheiben") {
        // Brake discs are cast iron: breakage grade, own variant so the
        // row never collapses with furnace/handelsguss.
        Some(("eisenschrott-gussbruch", "Bremsscheiben"))
    } else if l.contains("ofen") || l.contains("handelsguss") {
        Some(("eisenschrott-gussbruch", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("aluprofile") {
        Some(("aluminium-profile", ""))
    } else if l.contains("alufelgen") {
        // Alloy wheels are cast AlSi: guss grade with their own variant.
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("alugeschirr") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("zinn") {
        Some(("zinn", ""))
    } else if l.contains("widia") || l.contains("plättchen") {
        Some(("hartmetall", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` anchored
/// by "Inh." holds firm line + street + PLZ city on `<br>` lines, and the
/// `<p>` with "Telefon"/"Mobil" labels holds the numbers. The site shows
/// no e-mail address (no mailto, no @ in content), so email stays empty.
/// Missing anchors → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p = Selector::parse("p").expect("valid selector");
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let (mut phone, mut mobil) = (String::new(), String::new());
    for el in doc.select(&p) {
        let lines: Vec<String> = el
            .inner_html()
            .split("<br")
            .map(strip_fragment)
            .filter(|s| !s.is_empty())
            .collect();
        if lines.iter().any(|s| s.contains("Inh.")) {
            for (k, line) in lines.iter().enumerate() {
                let mut it = line.split_whitespace();
                // PLZ city: first token 5 digits, second capitalized.
                if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                    if pc.len() == 5
                        && pc.chars().all(|c| c.is_ascii_digit())
                        && ci.chars().next().is_some_and(|c| c.is_uppercase())
                    {
                        postcode = pc.to_owned();
                        city = ci.to_owned();
                        if k > 0 {
                            street = lines[k - 1].clone();
                        }
                        break;
                    }
                }
            }
        }
        for line in &lines {
            if let Some(v) = line.strip_prefix("Telefon") {
                phone = v.trim().to_owned();
            } else if let Some(v) = line.strip_prefix("Mobil") {
                mobil = v.trim().to_owned();
            }
        }
    }
    if street.is_empty() || postcode.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    }
    if phone.is_empty() {
        phone = mobil;
    }
    if phone.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
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

/// Strip tags from a fragment (html5ever already decoded entities).
/// Fragments from splitting on "<br" start with a tag remnant
/// (` class="…"`) — drop everything up to the first '>' first, or the
/// attributes parse as text.
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
    out.replace("&nbsp;", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, is_from_price, parse, unit_of};

    /// Real card markup from 28.09.2026 (whitespace compressed, tags and
    /// classes verbatim): label module + price module per grade, framed by
    /// the live window anchors.
    const FIXTURE: &str = "<div id=\"preisliste\" class=\"et_pb_section\">\
        <div class=\"et_pb_text_inner\"><h2>Kupfer Millberry</h2> <p>Tagespreis</p></div>\
        <div class=\"et_pb_text_inner\"><h2>11,15 Euro/kg</h2></div>\
        <div class=\"et_pb_text_inner\"><h2>Kupfer Kerze</h2> <p>Tagespreis</p></div>\
        <div class=\"et_pb_text_inner\"><h2>10,70 Euro/ kg</h2></div>\
        <div class=\"et_pb_text_inner\"><h2>Haushaltskabel 38%</h2> <p>Tagespreis</p></div>\
        <div class=\"et_pb_text_inner\"><h2>3,35 € / kg</h2></div>\
        <div class=\"et_pb_text_inner\"><h2>Kabel 75%</h2> <p>Tagespreis</p></div>\
        <div class=\"et_pb_text_inner\"><h2>4,80 € / kg</h2></div>\
        <div class=\"et_pb_text_inner\"><h2>Ofen/Handelsguss</h2> <p>Tagespreis</p></div>\
        <div class=\"et_pb_text_inner\"><h2>180 € / t*</h2></div>\
        <div class=\"et_pb_text_inner\"><h2>Mischschrott</h2> <p>Tagespreis<strong></strong></p></div>\
        <div class=\"et_pb_text_inner\"><h2>150 € / t*</h2> <p>&nbsp;</p></div>\
        <div class=\"et_pb_text_inner\"><h2>Alugeschirr</h2> <p>Tagespreis</p></div>\
        <div class=\"et_pb_text_inner\"><h2>ab 1,30 € / kg</h2></div>\
        <div class=\"et_pb_text_inner\"><h2>V2A</h2> <p>Tagespreis</p></div>\
        <div class=\"et_pb_text_inner\"><h2>ab 0,75 € / kg</h2></div>\
        <div class=\"et_pb_text_inner\"><h2>Zinn</h2> <p>Tagespreis</p></div>\
        <div class=\"et_pb_text_inner\"><h2>ab 15,00 € / kg</h2></div>\
        <div class=\"et_pb_text_inner\"><h2>Widia -Plättchen</h2> <p>Tagespreis</p></div>\
        <div class=\"et_pb_text_inner\"><h2>ab 24,00 € / kg</h2></div>\
        <p><span>* Bitte beachten Sie, dass die Preise nicht für kleine Mengen wie z.B. 2 oder 3 kg gelten! </span></p>";

    #[test]
    fn card_pairs() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 10, "rows: {rows:?} skips: {skips:?}");
        assert!(skips.is_empty());
        assert_eq!(rows[0].0, "Kupfer Millberry");
        assert_eq!(rows[0].1, 11.15);
        assert_eq!(rows[0].2, "EUR/kg");
        assert!(!rows[0].3, "exact price has no from flag");
        assert_eq!(rows[1].0, "Kupfer Kerze");
        assert_eq!(rows[1].1, 10.70);
        assert_eq!(rows[2].0, "Haushaltskabel 38%");
        assert_eq!(rows[2].1, 3.35);
        assert_eq!(rows[4].0, "Ofen/Handelsguss");
        assert_eq!(rows[4].1, 180.0);
        assert_eq!(rows[4].2, "EUR/t");
        assert_eq!(rows[5].0, "Mischschrott");
        assert_eq!(rows[5].2, "EUR/t");
        assert_eq!(rows[6].0, "Alugeschirr");
        assert!(rows[6].3, "ab flag");
        assert_eq!(rows[9].0, "Widia -Plättchen");
        assert_eq!(rows[9].1, 24.0);
        assert!(rows[9].3, "ab flag");
    }

    #[test]
    fn every_live_label_maps() {
        // All 19 grade labels seen live on 28.09.2026, specific before
        // generic: copper grades must not collapse into Millberry.
        let cases = [
            ("Kupfer Millberry", "kupfer-millberry", ""),
            ("Kupfer Kerze", "kupfer-gemischt", "Kerze"),
            ("Kupfer Schwer", "kupfer-gemischt", "schwer"),
            ("Messing Schwer", "messing", ""),
            ("Haushaltskabel 38%", "kabel-kupfer", "38%"),
            ("Kabel 50%", "kabel-kupfer", "50%"),
            ("Kabel 60%", "kabel-kupfer", "60%"),
            ("Kabel 75%", "kabel-kupfer", "75%"),
            ("Ofen/Handelsguss", "eisenschrott-gussbruch", ""),
            ("Bremsscheiben", "eisenschrott-gussbruch", "Bremsscheiben"),
            ("Mischschrott", "mischschrott", ""),
            ("Alugeschirr", "aluminium-gemischt", ""),
            ("Aluprofile blank", "aluminium-profile", ""),
            ("Alufelgen", "aluminium-guss", "Felgen"),
            ("Altzink", "zink", ""),
            ("Altblei", "blei", ""),
            ("V2A", "edelstahl-v2a", ""),
            ("Zinn", "zinn", ""),
            ("Widia -Plättchen", "hartmetall", ""),
        ];
        for (label, material, variant) in cases {
            assert_eq!(grade_for(label), Some((material, variant)), "{label}");
        }
        // Shared-material grades stay apart via variant; zinc never mints.
        assert_ne!(grade_for("Kupfer Kerze"), grade_for("Kupfer Schwer"));
        assert_ne!(grade_for("Kabel 50%"), grade_for("Kabel 60%"));
        assert_ne!(grade_for("Ofen/Handelsguss"), grade_for("Bremsscheiben"));
        assert_eq!(grade_for("Altzink"), Some(("zink", "")));
        assert!(grade_for("Schrott").is_none());
        assert!(grade_for("Kupfer").is_none());
    }

    #[test]
    fn units_and_flags() {
        assert_eq!(unit_of("11,15 Euro/kg"), Some("EUR/kg"));
        assert_eq!(unit_of("10,70 Euro/ kg"), Some("EUR/kg"));
        assert_eq!(unit_of("6,35Euro/kg"), Some("EUR/kg"));
        assert_eq!(unit_of("180 € / t*"), Some("EUR/t"));
        assert_eq!(unit_of("ab 0,75 € / kg"), Some("EUR/kg"));
        assert_eq!(unit_of("120 pro Sack"), None);
        assert!(is_from_price("ab 1,30 € / kg"));
        assert!(!is_from_price("1,10 € / kg"));
    }

    #[test]
    fn loud_skips_and_errors() {
        // Unknown unit skips loudly with its label (next to a valid row,
        // so the page still parses).
        let html = "<div id=\"preisliste\"><div class=\"et_pb_text_inner\">\
            <h2>Altblei</h2></div><div class=\"et_pb_text_inner\">\
            <h2>1,10 € / kg</h2></div><div class=\"et_pb_text_inner\">\
            <h2>Zinn</h2></div><div class=\"et_pb_text_inner\">\
            <h2>15 Euro pro Sack</h2></div><p>* Bitte beachten Sie</p></div>";
        let (rows, skips) = parse(html).expect("parses with skip");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "Altblei");
        assert!(skips.iter().any(|s| s.contains("Zinn")), "{skips:?}");
        // No pairs at all is an error, never a silent success.
        let empty = "<div id=\"preisliste\"><p>* Bitte beachten Sie</p></div>";
        assert!(parse(empty).is_err());
        assert!(parse("<p>kein Anker hier</p>").is_err());
    }

    #[test]
    fn impressum_blocks() {
        // Real impressum excerpt (28.09.2026): no e-mail on the whole site.
        let imp = "<p>Inh. M. Malitowska<br />Hauptstraße 4<br />\
            38531 Rötgesbüttel</p> <p><strong>Telefon</strong> 05304  5098294<br />\
            <strong>Mobil</strong> 0160 98045718</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Hauptstraße 4");
        assert_eq!(info.postcode, "38531");
        assert_eq!(info.city, "Rötgesbüttel");
        assert_eq!(info.phone, "05304 5098294");
        assert!(info.email.is_empty());
        assert!(extract_info("<p>Neu hier</p>").is_err());
    }

    /// Manual live check (needs network): runs the full scrape — price
    /// page plus impressum — against the real site. Ignored by default;
    /// run with `cargo test -p schrott-mcp-ingestion
    /// schrottplatz_roetgesbuettel -- --ignored --nocapture`.
    #[tokio::test]
    #[ignore]
    async fn live_scrape() {
        let client = reqwest::Client::builder()
            .user_agent("schrott-mcp-ingestion/0.1")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("client");
        let out = super::scrape(&client).await.expect("live scrape works");
        assert!(out.website_alive);
        assert_eq!(out.prices.len(), 19, "all cards map live");
        assert!(out.skipped_labels.is_empty(), "{:?}", out.skipped_labels);
        assert_eq!(out.trader_info.postcode, "38531");
        assert!(out.published_at.is_none());
    }
}
