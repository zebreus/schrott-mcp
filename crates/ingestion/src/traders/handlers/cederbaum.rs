//! Cederbaum Container GmbH (Braunschweig, Privatkunden-Annahme): exact
//! per-tonne remunerations ("Vergütung pro Tonne X €") as Elementor cards —
//! one `<h4>` label plus one price `<p>` per card — on the Privatkunden page,
//! windowed between "Tagesaktuelle Höchstpreise" and "Dein Rohstoff war
//! nicht dabei?".
//!
//! Deliberately NOT a second price source: `/schrott-altmetalle/` carries no
//! prices (verified live 28.09.2026: zero `€` amounts, zero "Vergütung" rows;
//! its button "Zu den aktuellen Preisen für Schrott & Altmetall" links to
//! `/privatkunden/?scrollto=leistungen`). One price page, one block.
//!
//! Page quirks, all live-verified:
//! - No page date exists (the only "gültig" is the ID note "gültigen
//!   Ausweis"; digit triples are image srcset dimensions) → `published_at`
//!   is None. "Tageshöchstpreise" is a market-top promise in the heading,
//!   while each card states a concrete "Vergütung" — so `exact` / 1.0, not
//!   `upto` (no "bis zu" anywhere near the cards).
//! - Every card quotes "pro Tonne" (€/t), but the catalog prices NE metals,
//!   stainless, cable, motors and zinc per kg: those rows are divided by
//!   1000 into EUR/kg (the documented kg↔t conversion), iron rows
//!   (Mischschrott, Guss, Shredder) stay EUR/t. 10.700 €/t → 10.70 €/kg is
//!   the plausibility anchor (kg 1–2-stellig, t 3–5-stellig).
//! - "0,00" rows would mean "no quote", never a free gift → loud skip.
//! - Kühler/Felgen follow precedent, not instinct: Felgen → aluminium-guss
//!   (db_recycling), Alu-Cu-Kühler → None as Cu/Alu-Mischprodukt without a
//!   catalog material (doering). Batterieblei → None (db_recycling:
//!   batteries have no catalog material). Bremsscheiben →
//!   eisenschrott-gussbruch "Bremsscheiben" (doering). Alu-Geschirr →
//!   aluminium-blech "Geschirr" (vedder/doering).
//! - Plain "Edelstahl" names no grade → edelstahl-gemischt, never a
//!   specific V2A/V4A sort.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "ni-braunschweig-cederbaum-privatkunden-annahme";
/// Bespoke, live-verified price URL (the category page links here for
/// prices). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const URL: &str = "https://www.cederbaum.de/privatkunden/";
/// Bespoke, live-verified impressum URL (site footer "Impressum"). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.cederbaum.de/impressum/";

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
    let mut seen = std::collections::HashSet::new();
    for (label, price_t, _unit_t) in rows {
        match grade_for(&label) {
            Some((material, variant, unit)) => {
                let price = to_catalog(price_t, unit);
                // Single card block live, but dedup guards a repeated
                // "gültig ab"-style double block after mapping, not on
                // raw labels.
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
        published_at,
    })
}

/// Explicit label → (material, variant, catalog unit) mapping. Anything
/// unlisted is skipped. Specific-before-generic throughout: "Alu-Cu Kühler"
/// must die on the kühler arm before any alu/copper arm claims it, and
/// "Batterieblei" must die before the bare "blei" arm claims it.
fn grade_for(label: &str) -> Option<(&'static str, &'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // No-catalog guards first (mixed product / batteries, never crammed).
    if l.contains("kühler") || l.contains("kuehler") {
        // Alu-Cu-Mischprodukt ohne Katalogmaterial (vgl. doering).
        return None;
    }
    if l.contains("batterie") || l.contains("bleiakku") {
        // Akkus haben kein Katalogmaterial (vgl. db_recycling).
        return None;
    }
    if l.contains("millberry") {
        Some(("kupfer-millberry", "", "EUR/kg"))
    } else if l.contains("kabel") {
        // Live only "Kupfer Kabelschrott" (no yield stated) → generic
        // copper cable, standard grade.
        Some(("kabel-kupfer", "", "EUR/kg"))
    } else if l.contains("kupfer") && l.contains("schwer") {
        Some(("kupfer-gemischt", "schwer", "EUR/kg"))
    } else if l.contains("messing") {
        // Live only "Messing schwer".
        if l.contains("schwer") {
            Some(("messing", "schwer", "EUR/kg"))
        } else {
            Some(("messing", "", "EUR/kg"))
        }
    } else if l.contains("mischschrott") {
        // Two sorts, one material, two prices → variants, or they
        // collapse onto one arbitrary current price.
        if l.contains("leicht") {
            Some(("mischschrott", "leicht", "EUR/t"))
        } else if l.contains("schwer") {
            Some(("mischschrott", "schwer", "EUR/t"))
        } else {
            Some(("mischschrott", "", "EUR/t"))
        }
    } else if l.contains("bremsscheiben") || l.contains("bremsscheibe") {
        // Grauguss-Bremsscheiben → Gussbruch, eigene Variante (vgl. doering).
        Some(("eisenschrott-gussbruch", "Bremsscheiben", "EUR/t"))
    } else if l.contains("guss") || l.contains("ofen") || l.contains("handel") {
        // Live "Ofen- und Handelsguss".
        Some(("eisenschrott-gussbruch", "", "EUR/t"))
    } else if l.contains("felgen") {
        // Alufelgen → Aluguss (vgl. db_recycling).
        Some(("aluminium-guss", "Felgen", "EUR/kg"))
    } else if l.contains("geschirr") {
        Some(("aluminium-blech", "Geschirr", "EUR/kg"))
    } else if l.contains("altblei") || l.trim() == "blei" {
        Some(("blei", "", "EUR/kg"))
    } else if l.contains("e-motor") || l.contains("emotor") || l.contains("e motor") {
        Some(("elektromotoren", "", "EUR/kg"))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", "", "EUR/kg"))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", "", "EUR/kg"))
    } else if l.contains("edelstahl") || l.trim() == "va" {
        // No grade stated → generic, never a specific sort.
        Some(("edelstahl-gemischt", "", "EUR/kg"))
    } else if l.contains("shredder") {
        Some(("stahlschrott-shredder", "", "EUR/t"))
    } else if l.contains("zink") {
        Some(("zink", "", "EUR/kg"))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` holding
/// `<strong>Cederbaum Container GmbH</strong>` carries street + PLZ city
/// (plus a Postfach line that must NOT win), and the `<p>` holding
/// `<strong>Kontakt</strong>` carries "Telefon:" / "E-Mail:" lines (the
/// mail address is decimal+hex entity-encoded and arrives decoded via
/// `text()`). Missing anchors mean the page changed shape → loud error,
/// never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p = Selector::parse("p").expect("valid selector");
    let strong = Selector::parse("strong").expect("valid selector");
    let has_strong = |el: &scraper::ElementRef, title: &str| {
        el.select(&strong)
            .any(|s| s.text().collect::<String>().trim() == title)
    };
    let addr_p = doc
        .select(&p)
        .find(|el| has_strong(el, "Cederbaum Container GmbH"));
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Cederbaum-Adressblock fehlt".to_owned(),
        });
    };
    let mut lines = Vec::new();
    for part in addr_p.inner_html().split("<br") {
        let t = strip_tags(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    // "Hannoversche Straße 65" / "38116 Braunschweig"; the "Postfach 16 19,
    // 38006 Braunschweig" line must never win either slot.
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for line in &lines {
        if line.starts_with("Postfach") || line.contains("Cederbaum") {
            continue;
        }
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(first)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                if postcode.is_empty() {
                    postcode = pc.to_owned();
                    city = format!("{first} {}", it.collect::<Vec<_>>().join(" "))
                        .trim_end()
                        .to_owned();
                }
                continue;
            }
        }
        if street.is_empty() && line.chars().any(|c| c.is_ascii_digit()) {
            street = line.clone();
        }
    }
    let kontakt_p = doc.select(&p).find(|el| has_strong(el, "Kontakt"));
    let Some(kontakt_p) = kontakt_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let (mut phone, mut email) = (String::new(), String::new());
    for part in kontakt_p.inner_html().split("<br") {
        let t = strip_tags(part);
        if let Some(v) = t.strip_prefix("Telefon:") {
            if phone.is_empty() {
                phone = v.trim().to_owned();
            }
        } else if let Some(v) = t.strip_prefix("E-Mail:") {
            // Own rule for mail (a phone-style take_while would stop at
            // the first letter): take the @ token.
            email = v
                .split_whitespace()
                .find(|tok| tok.contains('@'))
                .unwrap_or_default()
                .trim_matches([',', ';', '.'])
                .to_owned();
        }
    }
    if street.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    }
    if phone.is_empty() && email.is_empty() {
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

/// Strip tags from a `<br>`-split fragment (html5ever already decoded
/// entities). Fragments starting with a tag remnant (after the `<br` split
/// point, e.g. ` class="…">`) drop everything up to the first '>' first —
/// otherwise attribute text parses as an address line.
fn strip_tags(s: &str) -> String {
    let s = s.trim_start();
    let s = if s.starts_with('<') {
        match s.find('>') {
            Some(i) => &s[i + 1..],
            None => s,
        }
    } else {
        s
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

/// Per-tonne page price into the catalog unit: kg-catalog materials are
/// divided by 1000 (the documented kg↔t conversion), tonne materials pass
/// through untouched.
fn to_catalog(price_per_tonne: f64, unit: &str) -> f64 {
    if unit == "EUR/kg" {
        price_per_tonne / 1000.0
    } else {
        price_per_tonne
    }
}

fn parse(
    html: &str,
) -> Result<
    (
        Option<String>,
        Vec<(String, f64, &'static str)>,
        Vec<String>,
    ),
    IngestError,
> {
    // Window: the price cards only — start AND end anchored. The footer
    // carries its own "0,00 €" cart teaser; a whole-page walk would glue it
    // onto a label as a phantom zero price.
    let start = html
        .find("Tagesaktuelle Höchstpreise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Dein Rohstoff war nicht dabei?")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock offen".to_owned(),
        })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(window);
    let h4 = Selector::parse("h4").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let labels: Vec<String> = doc
        .select(&h4)
        .map(|h| {
            h.text()
                .collect::<String>()
                .replace(['\u{a0}'], " ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|t| !t.is_empty())
        .collect();
    // Only content elements: price cards, never script/style text (the
    // page embeds JSON-LD and Elementor settings nearby).
    let price_ps: Vec<String> = doc
        .select(&p)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.replace(['\u{a0}'], " "))
        .filter(|t| t.contains("Vergütung"))
        .collect();
    if labels.is_empty() || price_ps.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock leer".to_owned(),
        });
    }
    // Labels and price rows are paired by document order (one card each);
    // a mismatch means the page changed shape → loud error, never a
    // guessed pairing.
    if labels.len() != price_ps.len() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: format!(
                "Label/Preis-Paare ungleich ({} Labels, {} Preise)",
                labels.len(),
                price_ps.len()
            ),
        });
    }
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for (label, ptext) in labels.into_iter().zip(price_ps.iter()) {
        if label.len() > 120 {
            // Prose heading, not a material card.
            continue;
        }
        let Some(price) = parse_eur(ptext) else {
            skipped.push(format!("{label} (kein Preis: {})", ptext.trim()));
            continue;
        };
        // A "0,00" row is "no quote", not a free gift: loud skip.
        if price == 0.0 {
            skipped.push(format!("{label} (Preis 0,00)"));
            continue;
        }
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let Some(unit) = unit_of(ptext) else {
            skipped.push(format!(
                "{label} (Einheit unverständlich: {})",
                ptext.trim()
            ));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock leer".to_owned(),
        });
    }
    // No page-stated date anywhere (see module docs) → None; observed_at
    // dates the run.
    Ok((None, rows, skipped))
}

/// Bespoke unit matcher for THIS card row (live: always "Vergütung pro
/// Tonne"). Only t exists here — anything else skips loudly at the call
/// site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("tonne") || lower.contains("pro t") || lower.contains("/t") {
        Some("EUR/t")
    } else if lower.contains("kg") || lower.contains("kilo") {
        Some("EUR/kg")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, to_catalog};

    // Real card shape of the live page (28.09.2026), condensed to
    // representative rows: window anchors, h4 + "Vergütung pro Tonne" pairs,
    // the terminator heading, and the footer cart teaser that must stay out
    // of the window.
    const FIXTURE: &str =
        "<h3>Entdecke den Wert deiner Altmetalle - <br>Tagesaktuelle Höchstpreise garantiert</h3>\
        <h4 class=\"elementor-heading-title elementor-size-default\">Altblei</h4>\
        <p>Vergütung pro Tonne<strong> 900,00 €</strong></p>\
        <h4 class=\"elementor-heading-title elementor-size-default\">Alu-Cu Kühler</h4>\
        <p>Vergütung pro Tonne <strong>2.600,00 €</strong></p>\
        <h4 class=\"elementor-heading-title elementor-size-default\">Alu-Felgen</h4>\
        <p>Vergütung pro Tonne <strong>1.600,00 €</strong></p>\
        <h4 class=\"elementor-heading-title elementor-size-default\">Kupfer Millberry</h4>\
        <p>Vergütung pro Tonne <strong>10.700,00 €</strong></p>\
        <h4 class=\"elementor-heading-title elementor-size-default\">Mischschrott leicht</h4>\
        <p>Vergütung pro Tonne <strong>150,00 €</strong></p>\
        <h4 class=\"elementor-heading-title elementor-size-default\">Mischschrott schwer</h4>\
        <p>Vergütung pro Tonne <strong>170,00 €</strong></p>\
        <h4 class=\"elementor-heading-title elementor-size-default\">Ofen- und Handelsguss</h4>\
        <p>Vergütung pro Tonne <strong>140,00 €</strong></p>\
        <h4 class=\"elementor-heading-title elementor-size-default\">Zink</h4>\
        <p>Vergütung pro Tonne <strong>1.700,00 €</strong></p>\
        <h3>Dein Rohstoff war nicht dabei?</h3>\
        <p>0,00&nbsp; &euro; 0 Warenkorb</p>";

    #[test]
    fn cards_parse_windowed_with_tonne_unit() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at, None);
        assert!(skips.is_empty());
        assert_eq!(rows.len(), 8);
        assert_eq!(rows[0], ("Altblei".to_owned(), 900.0, "EUR/t"));
        assert_eq!(rows[3], ("Kupfer Millberry".to_owned(), 10_700.0, "EUR/t"));
        assert_eq!(rows[7], ("Zink".to_owned(), 1700.0, "EUR/t"));
    }

    #[test]
    fn tonne_to_catalog_unit_converts() {
        // Beweisbare Umrechnung: NE per kg, Eisen per t.
        assert_eq!(to_catalog(10_700.0, "EUR/kg"), 10.7);
        assert_eq!(to_catalog(900.0, "EUR/kg"), 0.9);
        assert_eq!(to_catalog(150.0, "EUR/t"), 150.0);
    }

    #[test]
    fn zero_and_unit_skip_loudly() {
        let html = FIXTURE
            .replacen("<strong>10.700,00 €</strong>", "<strong>0,00 €</strong>", 1)
            .replacen(
                "Vergütung pro Tonne <strong>1.700,00 €</strong>",
                "Vergütung pro Sack <strong>50,00 €</strong>",
                1,
            );
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 6);
        assert_eq!(skips.len(), 2);
        assert!(skips[0].contains("Millberry") && skips[0].contains("0,00"));
        assert!(skips[1].contains("Zink") && skips[1].contains("unverständlich"));
    }

    #[test]
    fn window_and_pairing_fail_loudly() {
        // Missing start anchor: loud error, not silent success.
        assert!(parse("<html><body><p>Neu hier</p></body></html>").is_err());
        // Missing terminator: loud error.
        let html = FIXTURE.replace("Dein Rohstoff war nicht dabei?", "Anfahrt");
        assert!(parse(&html).is_err());
        // Label/price mismatch: loud error, never guessed pairing.
        let html = FIXTURE.replacen("<p>Vergütung pro Tonne", "<p>Anlieferung", 1);
        assert!(parse(&html).is_err());
        // Every row unparseable: loud error, not silent success.
        let html = FIXTURE.replace("Vergütung pro Tonne", "Vergütung pro Sack");
        let err = parse(&html).expect_err("empty block errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn mapping_covers_every_live_card() {
        assert_eq!(grade_for("Altblei"), Some(("blei", "", "EUR/kg")));
        assert_eq!(grade_for("Alu-Cu Kühler"), None);
        assert_eq!(
            grade_for("Alu-Felgen"),
            Some(("aluminium-guss", "Felgen", "EUR/kg"))
        );
        assert_eq!(
            grade_for("Alu-Geschirr"),
            Some(("aluminium-blech", "Geschirr", "EUR/kg"))
        );
        assert_eq!(grade_for("Batterieblei"), None);
        assert_eq!(
            grade_for("Bremsscheiben"),
            Some(("eisenschrott-gussbruch", "Bremsscheiben", "EUR/t"))
        );
        assert_eq!(
            grade_for("Edelstahl"),
            Some(("edelstahl-gemischt", "", "EUR/kg"))
        );
        assert_eq!(
            grade_for("E-Motoren"),
            Some(("elektromotoren", "", "EUR/kg"))
        );
        assert_eq!(
            grade_for("Kupfer Kabelschrott"),
            Some(("kabel-kupfer", "", "EUR/kg"))
        );
        assert_eq!(
            grade_for("Kupfer Millberry"),
            Some(("kupfer-millberry", "", "EUR/kg"))
        );
        assert_eq!(
            grade_for("Kupfer schwer"),
            Some(("kupfer-gemischt", "schwer", "EUR/kg"))
        );
        assert_eq!(
            grade_for("Messing schwer"),
            Some(("messing", "schwer", "EUR/kg"))
        );
        assert_eq!(
            grade_for("Mischschrott leicht"),
            Some(("mischschrott", "leicht", "EUR/t"))
        );
        assert_eq!(
            grade_for("Mischschrott schwer"),
            Some(("mischschrott", "schwer", "EUR/t"))
        );
        assert_eq!(
            grade_for("Ofen- und Handelsguss"),
            Some(("eisenschrott-gussbruch", "", "EUR/t"))
        );
        assert_eq!(
            grade_for("Shreddervormaterial"),
            Some(("stahlschrott-shredder", "", "EUR/t"))
        );
        assert_eq!(grade_for("Zink"), Some(("zink", "", "EUR/kg")));
    }

    #[test]
    fn mapping_orders_specific_before_generic() {
        // The kühler guard wins over any alu/copper arm; the battery guard
        // wins over bare blei.
        assert_eq!(grade_for("Alu-Cu Kühler o. Fe"), None);
        assert_eq!(grade_for("Batterieblei/Bleiakkus"), None);
        // Bremsscheiben keeps its variant apart from plain Gussbruch.
        assert_ne!(
            grade_for("Bremsscheiben"),
            grade_for("Ofen- und Handelsguss")
        );
    }

    #[test]
    fn impressum_extracts_contact_with_postfach_guard() {
        // Real fragment shape: firm block with Postfach line, Kontakt block
        // with decimal+hex entity mail.
        let imp = "<p><strong>Cederbaum Container GmbH</strong><br />Hannoversche Straße 65<br />\
            38116 Braunschweig<br />Postfach 16 19, 38006 Braunschweig</p>\
            <p><strong>Kontakt</strong><br />Telefon: 0531 58005-0<br />Telefax: 0531 58005-55<br />\
            E-Mail: &#105;&#x6e;f&#x6f;&#64;&#99;&#x65;&#100;&#x65;r&#98;&#x61;&#117;&#x6d;&#46;&#x64;&#x65;</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Hannoversche Straße 65");
        assert_eq!(info.postcode, "38116");
        assert_eq!(info.city, "Braunschweig");
        assert_eq!(info.phone, "0531 58005-0");
        assert_eq!(info.email, "info@cederbaum.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
