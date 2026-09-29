//! A-Z Recycling (Udo Salzsieder, Münster): staffel prices in Elementor
//! heading cards on the homepage (`class="elementor-heading-title"`:
//! material name + "Ankaufspreis: €/kg" + three `(price, "ab … kg")`
//! pairs for ab 5 / 100 / 500 kg). Four cards are visible, four more sit
//! in the "Weitere Schrottpreise" accordion — same static markup, so one
//! window covers all eight materials. Each material's three tier prices
//! are recorded as ONE `price_kind: "range"` row (price = best tier
//! ab 500 kg, min/max = staffel extremes, confidence 0.5); a collapsed
//! staffel (min == max) falls back to `exact` like VANA. The tier rides
//! implicitly in the span — only genuine grades get a `variant`
//! ("Haushaltskabel 38%" → kabel-kupfer, Neuwert grade-gluing), so no two
//! sorts collapse. Page facts without price rows ("Sofort bar
//! ausgezahlt") have no handler field and stay out. No page-stated price
//! date ("tagesaktuell" only) → `published_at` stays `None`. A bare
//! "Kabel" row without a copper share skips loudly (Cu vs Alu cable are
//! different worlds); 0.00 tiers skip as "kein Ankaufpreis".

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-munster-a-z-recycling-udo-salzsieder";
/// Bespoke, live-verified price URL (Punycode for schrottplatz-münster.de;
/// the seed URL `https://https://schrottplatz-münster.de.de` is malformed
/// and dead). The price cards live on the homepage — this IS the price
/// page here, not a fallback.
pub const URL: &str = "https://xn--schrottplatz-mnster-jbc.de/";
/// Bespoke, live-verified impressum URL (site footer's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "https://xn--schrottplatz-mnster-jbc.de/impressum/";

/// Price window: the cards only — never the whole page (reviews, FAQ and
/// footer numbers must not pair with labels into phantom prices).
const START_ANCHOR: &str = "Unser aktueller Schrottpreis";
const END_ANCHOR: &str = "So funktioniert unser Schrottankauf";

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
    for (label, min, max, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => {
                // One staffel = one range row: both bounds are page-quoted
                // prices with their own kind, never a silent exact.
                let (price_kind, confidence) = kind_for(min, max);
                let (price_min, price_max) = if price_kind == "range" {
                    (Some(min), Some(max))
                } else {
                    (None, None)
                };
                prices.push(ScrapedPrice {
                    material,
                    variant,
                    price: max,
                    currency: "EUR",
                    unit,
                    price_kind,
                    price_min,
                    price_max,
                    confidence,
                    label,
                });
            }
            None => skipped_labels.push(format!("{label} (kein Katalogmaterial)")),
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

/// Collapsed staffels (all tiers equal) are exact; real spans are ranges.
fn kind_for(min: f64, max: f64) -> (&'static str, Option<f64>) {
    if (min - max).abs() < f64::EPSILON {
        ("exact", Some(1.0))
    } else {
        ("range", Some(0.5))
    }
}

/// Explicit label → (material, variant) mapping, specific before generic
/// ("Edelstahl V2A" must not fall into a bare-edelstahl arm). Anything
/// unlisted is skipped loudly at the call site. Bare "Kabel" names no
/// metal share (Cu vs Alu cable price worlds apart) → loud skip; only an
/// explicit share ("38%") maps to `kabel-kupfer`.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("kabel") {
        if l.contains("38") {
            Some(("kabel-kupfer", "38% ohne Stecker"))
        } else {
            None
        }
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("edelstahl") {
        Some(("edelstahl-gemischt", ""))
    } else if l.contains("aluminium") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the
/// `elementor-widget-text-editor` block carrying "Geschäftsleitung" holds
/// the address `<p>` (firm lines + street + PLZ city over `<br>`), the
/// "Tel …" `<p>` (Festnetz first, Mobil ignored — one phone field) and
/// the `mailto:` contact. Scoping to that block keeps header/footer
/// look-alikes (Info@…, top-bar phone) out. Missing anchors mean the page
/// changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let div = Selector::parse("div.elementor-widget-text-editor").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
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
    let block = doc
        .select(&div)
        .find(|d| d.text().collect::<String>().contains("Geschäftsleitung"))
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        })?;
    let addr_p = block
        .select(&p)
        .find(|el| el.text().collect::<String>().contains("Geschäftsleitung"));
    let mut lines = Vec::new();
    if let Some(p) = addr_p {
        for part in p.inner_html().split("<br") {
            let t = strip_fragment(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
    // "Coermühle 4a" / "48157 Münster" (street = line before the PLZ).
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
    // "Tel & Fax: 0251 / 27 70 98" — the Festnetz line wins (matches the
    // site header); "Mobil:" has no field of its own and stays out.
    let mut phone = String::new();
    if let Some(tp) = block
        .select(&p)
        .find(|el| el.text().collect::<String>().contains("Tel"))
    {
        for part in tp.inner_html().split("<br") {
            let t = strip_fragment(part);
            if let Some((k, v)) = t.split_once(':') {
                if k.trim().starts_with("Tel") && phone.is_empty() {
                    phone = v.trim().to_owned();
                }
            }
        }
    }
    let email = block
        .select(&a)
        .filter_map(|el| el.value().attr("href"))
        .filter_map(|h| h.strip_prefix("mailto:"))
        .map(|s| s.split('?').next().unwrap_or_default().trim().to_owned())
        .find(|s| !s.is_empty())
        .unwrap_or_default();
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    // The street line is as much an anchor as the headings: without it
    // the page changed shape → loud error, never a guessed fallback.
    if street.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
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

/// Strip tags from a `<br`-split fragment (fragments start with a tag
/// remnant — drop everything up to the first '>' first, or attributes
/// parse as text).
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

/// The page's three quantity tiers. Identity only feeds skip messages —
/// the range collapses min/max over whatever tiers parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tier {
    T5,
    T100,
    T500,
}

fn tier_str(tier: Tier) -> &'static str {
    match tier {
        Tier::T5 => "ab 5 kg",
        Tier::T100 => "ab 100 kg",
        Tier::T500 => "ab 500 kg",
    }
}

/// "ab 500 kg" → tier. Anything without kg (a per-sack price recorded as
/// per-kg would be a 1000x-class error) returns None → loud skip.
fn tier_of(text: &str) -> Option<Tier> {
    let l = text.to_lowercase();
    if !l.contains("kg") {
        return None;
    }
    if l.contains("500") {
        Some(Tier::T500)
    } else if l.contains("100") {
        Some(Tier::T100)
    } else if l.contains('5') {
        Some(Tier::T5)
    } else {
        None
    }
}

/// Bare numbers only ("1.55"); tier labels ("ab 5 kg") and prose never
/// qualify, so prices can't pair with the wrong neighbour.
fn is_price(s: &str) -> bool {
    let t = s.trim();
    !t.is_empty()
        && t.chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == ',')
        && parse_eur(t).is_some()
}

fn is_tier_like(s: &str) -> bool {
    s.trim_start().to_lowercase().starts_with("ab ")
}

fn is_unit_header(s: &str) -> bool {
    s.trim_start().starts_with("Ankaufspreis")
}

/// Bespoke unit matcher for THIS page's card headers (live:
/// "Ankaufspreis: €/kg" per card). Only kg exists here — anything else
/// skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    if cell.to_lowercase().contains("kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

/// Parse the card window into (raw label, staffel-min, staffel-max, unit)
/// rows. Cards are found structurally: a heading that is NOT a unit
/// header, price or tier AND is followed by an "Ankaufspreis" header —
/// accordion titles ("Weitere Schrottpreise") and CTA headings ("Jetzt
/// Schrott verkaufen!") never satisfy both and stay out without noise
/// skips.
fn parse(html: &str) -> Result<(Vec<(String, f64, f64, &'static str)>, Vec<String>), IngestError> {
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
    let doc = Html::parse_fragment(window);
    let sel = Selector::parse(".elementor-heading-title").expect("valid selector");
    let heads: Vec<String> = doc
        .select(&sel)
        .map(|el| {
            el.text()
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|t| !t.is_empty())
        .collect();
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    let mut i = 0;
    while i < heads.len() {
        let h = heads[i].clone();
        let is_candidate =
            !is_unit_header(&h) && !is_price(&h) && !is_tier_like(&h) && h.len() <= 120;
        if is_candidate && i + 1 < heads.len() && is_unit_header(&heads[i + 1]) {
            let label = h;
            let unit_head = heads[i + 1].clone();
            let Some(unit) = unit_of(&unit_head) else {
                skips.push(format!("{label} (Einheit unverständlich: {unit_head})"));
                i += 2;
                continue;
            };
            i += 2;
            let mut tiers: Vec<(Tier, f64)> = Vec::new();
            while i + 1 < heads.len() && is_price(&heads[i]) && is_tier_like(&heads[i + 1]) {
                let price = parse_eur(&heads[i]).unwrap_or(0.0);
                match (price, tier_of(&heads[i + 1])) {
                    (p, Some(t)) if p > 0.0 => tiers.push((t, p)),
                    (p, _) if p <= 0.0 => skips.push(format!(
                        "{label} (kein Ankaufpreis: {} / {})",
                        heads[i],
                        heads[i + 1]
                    )),
                    (_, _) => skips.push(format!(
                        "{label} (Staffel unverständlich: {} / {})",
                        heads[i],
                        heads[i + 1]
                    )),
                }
                i += 2;
            }
            // A dangling price without its tier line is a loud skip, never
            // silently the next card's label.
            if i < heads.len() && is_price(&heads[i]) {
                skips.push(format!("{} (Staffel fehlt: {})", label, heads[i]));
                i += 1;
            }
            if tiers.is_empty() {
                skips.push(format!("{label} (keine Staffelpreise)"));
            } else {
                let min = tiers.iter().map(|(_, p)| *p).fold(f64::INFINITY, f64::min);
                let max = tiers
                    .iter()
                    .map(|(_, p)| *p)
                    .fold(f64::NEG_INFINITY, f64::max);
                rows.push((label, min, max, unit));
            }
        } else {
            // A tier line without its price is a loud skip too (redesign
            // smell) — tier-like headings occur only inside cards here.
            if is_tier_like(&h) {
                skips.push(format!("(Preis fehlt: {h})"));
            }
            i += 1;
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

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, is_price, is_tier_like, kind_for, parse, tier_of};

    // Real live markup, trimmed to two visible cards + the accordion title
    // + one accordion card + CTA tail + anchors (verbatim tags/classes).
    const FIXTURE: &str = concat!(
        "<h2 class=\"elementor-heading-title elementor-size-default\">Unser aktueller Schrottpreis –<br> das zahlen wir für Ihr Altmetall</h2>",
        "<h3 class=\"elementor-heading-title elementor-size-default\">Aluminium gemischt</h3>",
        "<h3 class=\"elementor-heading-title elementor-size-default\">Ankaufspreis: €/kg</h3>",
        "<p class=\"elementor-heading-title elementor-size-default\">1.55</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">ab 5 kg</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">1.78</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">ab 100 kg</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">2.22</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">ab 500 kg</p>",
        "<h2 class=\"elementor-heading-title elementor-size-default\">Mischschrott</h2>",
        "<h2 class=\"elementor-heading-title elementor-size-default\">Ankaufspreis: €/kg</h2>",
        "<p class=\"elementor-heading-title elementor-size-default\">1.23</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">ab 5 kg</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">1.24</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">ab 100 kg</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">1.25</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">ab 500 kg</p>",
        "<span class='e-n-accordion-item-title-header'><div class=\"e-n-accordion-item-title-text\"> Weitere Schrottpreise </div></span>",
        "<h3 class=\"elementor-heading-title elementor-size-default\">Kupfer</h3>",
        "<h3 class=\"elementor-heading-title elementor-size-default\">Ankaufspreis: €/kg</h3>",
        "<p class=\"elementor-heading-title elementor-size-default\">1.32</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">ab 5 kg</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">1.33</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">ab 100 kg</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">1.34</p>",
        "<p class=\"elementor-heading-title elementor-size-default\">ab 500 kg</p>",
        "<h3 class=\"elementor-heading-title elementor-size-default\">Jetzt Schrott verkaufen!</h3>",
        "<h5 class=\"elementor-heading-title elementor-size-default\">Von der Anfrage bis zur Auszahlung</h5>",
        "<h2 class=\"elementor-heading-title elementor-size-default\">So funktioniert unser Schrottankauf in Münster</h2>",
    );

    #[test]
    fn cards_collapse_to_ranges() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty(), "{skips:?}");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].0, "Aluminium gemischt");
        assert_eq!((rows[0].1, rows[0].2), (1.55, 2.22));
        assert_eq!(rows[0].3, "EUR/kg");
        assert_eq!(rows[1].0, "Mischschrott");
        assert_eq!((rows[1].1, rows[1].2), (1.23, 1.25));
        assert_eq!(rows[2].0, "Kupfer");
        assert_eq!((rows[2].1, rows[2].2), (1.32, 1.34));
    }

    #[test]
    fn anchors_and_empty_fail_loudly() {
        assert!(parse("<html><body><p>Neu hier</p></body></html>").is_err());
        let no_end = FIXTURE.replacen("So funktioniert unser Schrottankauf", "Sonst was", 1);
        assert!(parse(&no_end).is_err(), "end anchor missing");
        // CTA tail alone is no price list, not a silent success.
        let tail_only = concat!(
            "<h2 class=\"elementor-heading-title elementor-size-default\">Unser aktueller Schrottpreis</h2>",
            "<h3 class=\"elementor-heading-title elementor-size-default\">Jetzt Schrott verkaufen!</h3>",
            "<h2 class=\"elementor-heading-title elementor-size-default\">So funktioniert unser Schrottankauf</h2>",
        );
        assert!(parse(tail_only).is_err(), "no cards error");
    }

    #[test]
    fn zero_and_foreign_tiers_skip_loudly() {
        let zero = FIXTURE.replacen(
            "<p class=\"elementor-heading-title elementor-size-default\">1.78</p>",
            "<p class=\"elementor-heading-title elementor-size-default\">0.00</p>",
            1,
        );
        let (rows, skips) = parse(&zero).expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!((rows[0].1, rows[0].2), (1.55, 2.22));
        assert!(
            skips.iter().any(|s| s.contains("Aluminium") && s.contains("kein Ankaufpreis")),
            "{skips:?}"
        );
        let bad_unit = FIXTURE.replacen("ab 100 kg</p>", "ab Palette</p>", 1);
        let (rows, skips) = parse(&bad_unit).expect("parses");
        assert_eq!(rows.len(), 3);
        assert!(
            skips
                .iter()
                .any(|s| s.contains("Aluminium") && s.contains("Staffel unverständlich")),
            "{skips:?}"
        );
    }

    #[test]
    fn orphan_tier_skips_loudly() {
        let orphan = FIXTURE.replacen(
            "<p class=\"elementor-heading-title elementor-size-default\">1.78</p>",
            "",
            1,
        );
        let (rows, skips) = parse(&orphan).expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].0, "Aluminium gemischt");
        assert!(skips.iter().any(|s| s.contains("Preis fehlt")), "{skips:?}");
    }

    #[test]
    fn collapsed_staffel_is_exact() {
        assert_eq!(kind_for(1.25, 1.25), ("exact", Some(1.0)));
        assert_eq!(kind_for(1.55, 2.22), ("range", Some(0.5)));
    }

    #[test]
    fn guards_catch_non_rows() {
        assert!(is_price("1.55"));
        assert!(!is_price("ab 5 kg"));
        assert!(!is_price("Aluminium gemischt"));
        assert!(is_tier_like("ab 500 kg"));
        assert!(is_tier_like("ab Palette"));
        assert!(!is_tier_like("1.55"));
        assert!(tier_of("ab 5 kg").is_some());
        assert!(tier_of("ab Palette").is_none());
    }

    #[test]
    fn mapping_orders_specific_first() {
        assert_eq!(grade_for("Aluminium gemischt"), Some(("aluminium-gemischt", "")));
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(grade_for("Kupfer"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(
            grade_for("Haushaltskabel 38% (Ohne Stecker)"),
            Some(("kabel-kupfer", "38% ohne Stecker"))
        );
        assert_eq!(grade_for("Edelstahl V2A"), Some(("edelstahl-v2a", "")));
        // Bare cable names no share: Cu vs Alu is a different world.
        assert_eq!(grade_for("Kabel"), None);
        assert_eq!(grade_for("Irgendwas"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        // Real live markup of the impressum content block (trimmed).
        let imp = "<h1 class=\"elementor-heading-title elementor-size-default\">Impressum</h1>\
            <div class=\"elementor-element elementor-widget-text-editor\"><div class=\"elementor-widget-container\">\
            <p><strong>A-Z Recycling</strong></p><p>Geschäftsleitung:<br />Udo Salzsieder<br />Coermühle 4a<br />48157 Münster</p>\
            <p>Tel &amp; Fax: 0251 / 27 70 98<br />Mobil: 0172 / 52 196 49</p>\
            <p><a href=\"http://www.schrottplatz-münster.de\">www.schrottplatz-münster.de</a><br />\
            <a href=\"mailto:udo-salzsieder@web.de\">udo-salzsieder@web.de</a></p>\
            </div></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Coermühle 4a");
        assert_eq!(info.postcode, "48157");
        assert_eq!(info.city, "Münster");
        assert_eq!(info.phone, "0251 / 27 70 98");
        assert_eq!(info.email, "udo-salzsieder@web.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(
            extract_info("<h1>Impressum</h1><div class=\"elementor-widget-text-editor\"><p>Neu hier</p></div>")
                .is_err()
        );
    }
}
