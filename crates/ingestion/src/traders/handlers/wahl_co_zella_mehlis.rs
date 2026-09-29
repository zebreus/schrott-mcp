//! Wahl & Co. Inh. Thomas Madeiski (Zella-Mehlis): exact per-grade prices
//! live in the static FAQ "Sortenpreisliste" block (heading states
//! "Tagesaktuelle Sortenpreise in €/kg (Richtwerte)"), plus two "bis zu"
//! maxima from the FAQ above it (Edelstahl, Kabel → `upto`, confidence
//! 0.5) and assortment acceptances from the 15 metal-card headings.
//! No page date anywhere (the `price-update-date` span is JS-filled) →
//! `published_at` stays `None`. `0,00`/empty cells skip loudly, never as
//! price 0. "Kupfer-Messing-Kühler" (composite, no catalog material),
//! "Blei-Akkus" (batteries ≠ Weichblei), "Wolfram" (no catalog material)
//! and "Kabel & E-Schrott" (composite) skip loudly.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "th-zella-mehlis-wahl-co-inh-thomas-madeiski";
/// Bespoke, live-verified price URL (homepage carries the price FAQs).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://wahlundco.de/";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://wahlundco.de/impressum.html";

/// Page-stated unit for the whole Sortenpreisliste (heading:
/// "Tagesaktuelle Sortenpreise in €/kg (Richtwerte)"). The upto lines
/// state €/kg inline too — no per-row unit parsing needed.
const UNIT: &str = "EUR/kg";

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
    let (rows, mut skipped_labels) = parse_prices(&html)?;
    let mut prices = Vec::with_capacity(rows.len() + 2);
    for (label, price) in rows {
        if price == 0.0 {
            skipped_labels.push(format!("{label} (Nullpreis)"));
            continue;
        }
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit: UNIT,
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label,
            }),
            None => skipped_labels.push(label),
        }
    }
    // "bis zu" maxima from the FAQ above the table — best effort: the
    // marketing line may change shape while the table stays the contract.
    for (label, price, material) in parse_upto(&html) {
        if price == 0.0 {
            continue;
        }
        prices.push(ScrapedPrice {
            material,
            variant: "",
            price,
            currency: "EUR",
            unit: UNIT,
            price_kind: "upto",
            price_min: None,
            price_max: Some(price),
            confidence: Some(0.5),
            label,
        });
    }
    // Assortment from the metal-card headings (bonus, never fails the step;
    // unmapped cards skip loudly via skipped_labels).
    let mut acceptances = Vec::new();
    for name in parse_cards(&html) {
        match grade_for_card(&name) {
            Some(materials) => {
                for (material, conditions) in materials {
                    acceptances.push(ScrapedAcceptance {
                        material,
                        conditions: conditions.to_owned(),
                        label: name.clone(),
                    });
                }
            }
            None => skipped_labels.push(format!("Karte: {name}")),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        prices,
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

/// Explicit label → (material, variant). Continuation labels carry their
/// group base ("Alu-Profile, blank: / gestegt:"), so specific-before-generic
/// arms resolve them: "lackiert" beats "gestegt" beats "blank", "mit Fe"
/// beats "ohne Fe" (the base always contains the first grade too).
/// Anything unlisted (composites, off-catalog) is skipped.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("berry") {
        Some(("kupfer-berry", ""))
    } else if l.contains("kanal") {
        Some(("kupfer-gemischt", "kanal"))
    } else if l.contains("neublech") {
        Some(("kupfer-gemischt", "neublech"))
    } else if l.contains("schwerkupfer") {
        Some(("kupfer-gemischt", "schwerkupfer-lotfrei"))
    } else if l.contains("leichtkupfer") {
        Some(("kupfer-gemischt", "leichtkupfer"))
    } else if l.contains("kühler") || l.contains("kuehler") {
        // "Kupfer-Messing-Kühler": composite, `kuehler-verbund` was never
        // crammed into the catalog → loud skip, never a wrong row.
        None
    } else if l.contains("58") && l.contains("späne") {
        Some(("messing", "ms58-spaene"))
    } else if l.contains("58") {
        Some(("messing", "ms58-schrott"))
    } else if l.contains("63") {
        Some(("messing", "ms63-blech"))
    } else if l.contains("ohne hülsen") || l.contains("ohne-hülsen") {
        // "Messing ohne Hülsen/Schläuche" — vor der hülsen-Arm, die sonst
        // auch dieses Label fängt.
        Some(("messing", "ohne-huelsen"))
    } else if l.contains("hülsen") || l.contains("huelsen") {
        Some(("messing", "huelsen"))
    } else if l.contains("erodierdraht") {
        Some(("messing", "erodierdraht"))
    } else if l.contains("späne") && l.contains("messing") {
        Some(("messing", "spaene-gemischt"))
    } else if l.contains("messing") {
        Some(("messing", "ohne-huelsen"))
    } else if l.contains("walzbronze") {
        Some(("bronze-rotguss", "walzbronze"))
    } else if l.contains("rotguss") && l.contains("späne") {
        Some(("bronze-rotguss", "spaene"))
    } else if l.contains("rotguss") {
        Some(("bronze-rotguss", ""))
    } else if l.contains("ral-draht") {
        Some(("aluminium-gemischt", "ral-draht"))
    } else if l.contains("profile") && l.contains("lackiert") {
        Some(("aluminium-profile", "lackiert"))
    } else if l.contains("profile") && l.contains("gestegt") {
        Some(("aluminium-profile", "gestegt"))
    } else if l.contains("profile") {
        Some(("aluminium-profile", ""))
    } else if l.contains("offset") {
        Some(("aluminium-blech", "offset"))
    } else if l.contains("almg") {
        Some(("aluminium-blech", "almg"))
    } else if l.contains("felgen") {
        Some(("aluminium-guss", "felgen"))
    } else if l.contains("geschirr") && l.contains("mit fe") {
        Some(("aluminium-gemischt", "geschirr-mit-fe"))
    } else if l.contains("geschirr") {
        Some(("aluminium-gemischt", "geschirr-ohne-fe"))
    } else if l.contains("guss") && l.contains("mit fe") {
        Some(("aluminium-guss", "mit-fe"))
    } else if l.contains("guss") {
        Some(("aluminium-guss", "ohne-fe"))
    } else if l.contains("zink") && l.contains("/ alt") {
        Some(("zink", "alt"))
    } else if l.contains("zink") {
        Some(("zink", "neu"))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else {
        None
    }
}

/// Metal-card heading → acceptances. Edelstahl fans out to V2A+V4A (the
/// card reads "Edelstahl (V2A/V4A)"). Batteries, wolfram and the
/// Kabel&E-Schrott composite have no catalog fit → loud skip.
fn grade_for_card(name: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = name.to_lowercase();
    let l = l.as_str();
    if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("armaturenmessing") || l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("rotguss") {
        Some(vec![("bronze-rotguss", "")])
    } else if l.contains("aluminium") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("edelstahl") {
        Some(vec![("edelstahl-v2a", ""), ("edelstahl-v4a", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("zinn") {
        Some(vec![("zinn", "")])
    } else if l.contains("motor") {
        Some(vec![("elektromotoren", "")])
    } else if l.contains("hss") || l.contains("hochleistungsstahl") {
        Some(vec![("hss-werkzeuge", "")])
    } else if l.contains("hartmetall") || l.contains("vhm") {
        Some(vec![("hartmetall", "")])
    } else if l.contains("mischschrott") {
        Some(vec![("mischschrott", "")])
    } else {
        // "Blei-Akkus" (batteries ≠ Weichblei), "Wolfram (W)" and
        // "Kabel & E-Schrott" (composite): no catalog material.
        None
    }
}

/// Price rows from the FAQ "Sortenpreisliste" block: window between the
/// heading and the "Richtwerte" disclaimer, bullets split on "•", each
/// `<strong>X,YY €</strong>` paired with its preceding label text.
/// Continuation prices ("/ Späne: …") inherit their group base.
fn parse_prices(html: &str) -> Result<(Vec<(String, f64)>, Vec<String>), IngestError> {
    let start = html
        .find("Tagesaktuelle Sortenpreise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Sortenpreisliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("Preise sind Richtwerte").unwrap_or(tail.len());
    let window = &tail[..end];
    let doc = Html::parse_fragment(window);
    let p = Selector::parse("p").expect("valid selector");
    let mut rows: Vec<(String, f64)> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    for para in doc.select(&p) {
        let inner = para.inner_html();
        for bullet in inner.split('•').skip(1) {
            let mut segs = bullet.split("<strong>");
            let head = clean(&segs.next().unwrap_or(""));
            if head.is_empty() {
                continue;
            }
            let mut current = head.clone();
            for seg in segs {
                let Some((price_html, after)) = seg.split_once("</strong>") else {
                    continue;
                };
                let after_text = clean(after);
                match parse_eur(price_html) {
                    Some(price) => {
                        rows.push((current.clone(), price));
                        // Continuation label for the next price in this
                        // bullet ("/ Späne: …") keeps the group base.
                        if !after_text.is_empty() {
                            current = format!("{head} {after_text}");
                        }
                    }
                    None => {
                        skipped.push(format!("{current} (Preis unverständlich)"));
                    }
                }
            }
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Sortenpreisliste leer".to_owned(),
        });
    }
    Ok((rows, skipped))
}

/// "bis zu" maxima from the "was zahlt der Schrotthändler?" FAQ:
/// Edelstahl → edelstahl-gemischt, Kabel (je nach Kupferanteil) →
/// kabel-kupfer. Best effort — absent lines are ignored, never an error.
fn parse_upto(html: &str) -> Vec<(String, f64, &'static str)> {
    let mut out = Vec::new();
    for (marker, material) in [
        ("Edelstahlpreis", "edelstahl-gemischt"),
        ("Kabelpreis", "kabel-kupfer"),
    ] {
        let Some(mpos) = html.find(marker) else {
            continue;
        };
        let tail = &html[mpos..];
        let Some(bpos) = tail.find("bis zu") else {
            continue;
        };
        let after = &tail[bpos + "bis zu".len()..];
        // Only same-line numbers count (marketing line, not the table).
        let line_end = after.find('<').unwrap_or(after.len());
        let Some(price) = parse_eur(&after[..line_end]) else {
            continue;
        };
        let num = after[..line_end].trim().to_owned();
        out.push((format!("{marker}: bis zu {num}"), price, material));
    }
    out
}

/// Metal-card headings (`<div class="metal-card">…<h3>…`) as assortment
/// proof. Bonus only: an empty card list is not an error.
fn parse_cards(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let sel = Selector::parse(".metal-card h3").expect("valid selector");
    doc.select(&sel)
        .map(|el| clean(&el.inner_html()))
        .filter(|t| !t.is_empty())
        .collect()
}

/// Bespoke contact extraction for THIS impressum only: address lines come
/// from the `<br>` lines of the `<p>` carrying the street anchor
/// ("Heinrich-Ehrhardt-Str. 47b / 98544 Zella-Mehlis"), phone/mail from
/// the `<br>` lines of the Kontakt `<p>` ("Telefon: …" / "E-Mail: …").
/// Whole-doc `text()` glues neighbouring nodes without spaces
/// ("e.K.Heinrich…", "47b98544") — so `<br>` lines it is, first match wins
/// (authority phone numbers further down must not overwrite the trader's).
/// Missing anchors → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p_sel = Selector::parse("p").expect("valid selector");
    let (mut street, mut postcode, mut city, mut phone, mut email) = (
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    );
    for el in doc.select(&p_sel) {
        let lines: Vec<String> = el
            .inner_html()
            .split("<br>")
            .map(clean)
            .filter(|l| !l.is_empty())
            .collect();
        if street.is_empty() {
            if let Some(line) = lines.iter().find(|l| l.contains("Heinrich-Ehrhardt-Str.")) {
                street = line.clone();
            }
            // PLZ/Ort stehen auf der Folgezeile ("98544 Zella-Mehlis").
            // First match wins (Behördenadressen weiter unten zählen nicht).
            if postcode.is_empty() {
                for line in &lines {
                    let toks: Vec<&str> = line.split_whitespace().collect();
                    for (k, t) in toks.iter().enumerate() {
                        if t.len() == 5 && t.chars().all(|c| c.is_ascii_digit()) {
                            if let Some(ci) = toks.get(k + 1) {
                                if ci.chars().next().is_some_and(|c| c.is_uppercase()) {
                                    postcode = (*t).to_owned();
                                    city = (*ci).to_owned();
                                }
                            }
                        }
                    }
                }
            }
        }
        if phone.is_empty() {
            if let Some(line) = lines.iter().find(|l| l.starts_with("Telefon:")) {
                phone = line["Telefon:".len()..].trim().to_owned();
            }
        }
        if email.is_empty() {
            if let Some(line) = lines.iter().find(|l| l.starts_with("E-Mail:")) {
                email = line["E-Mail:".len()..]
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .to_owned();
            }
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

/// Strip tags from an HTML snippet, collapse whitespace.
fn clean(raw: &str) -> String {
    Html::parse_fragment(raw)
        .root_element()
        .text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, grade_for_card, parse_cards, parse_prices, parse_upto};

    /// Realer HTML-Ausschnitt der live Sortenpreisliste (FAQ HOOK 4),
    /// Stand 28.09.2026 — vereinfachte Nachbauten geben falsche Sicherheit.
    const FIXTURE_TABLE: &str = "<p><strong>Tagesaktuelle Sortenpreise</strong> in \u{20ac}/kg (Richtwerte):</p>\
        <p><strong>Kupfer:</strong><br>\
        \u{2022} Kupfer-Draht \u{201e}Millberry\": <strong>10,10 \u{20ac}</strong><br>\
        \u{2022} Kupfer-Draht \u{201e}Berry\": <strong>9,70 \u{20ac}</strong><br>\
        \u{2022} Kupfer-Draht \u{201e}Kanal\": <strong>9,20 \u{20ac}</strong><br>\
        \u{2022} Kupfer-Neublech: <strong>9,95 \u{20ac}</strong><br>\
        \u{2022} Schwerkupfer, lotfrei: <strong>9,55 \u{20ac}</strong><br>\
        \u{2022} Leichtkupfer: <strong>9,20 \u{20ac}</strong></p>\
        <p><strong>Messing:</strong><br>\
        \u{2022} Messing ohne H\u{00fc}lsen/Schl\u{00e4}uche: <strong>5,45 \u{20ac}</strong><br>\
        \u{2022} Messing-Sp\u{00e4}ne, gemischt: <strong>5,75 \u{20ac}</strong><br>\
        \u{2022} Messing-H\u{00fc}lsen, sauber: <strong>6,50 \u{20ac}</strong><br>\
        \u{2022} MS-Erodierdraht: <strong>6,20 \u{20ac}</strong><br>\
        \u{2022} Messing 58 Schrott: <strong>6,15 \u{20ac}</strong> / Sp\u{00e4}ne: <strong>5,80 \u{20ac}</strong><br>\
        \u{2022} Messing 63 Blech, neu, blank: <strong>6,50 \u{20ac}</strong><br>\
        \u{2022} Kupfer-Messing-K\u{00fc}hler: <strong>5,80 \u{20ac}</strong></p>\
        <p><strong>Rotguss / Bronze:</strong><br>\
        \u{2022} Rotguss-Schrott: <strong>8,85 \u{20ac}</strong> / Sp\u{00e4}ne: <strong>8,60 \u{20ac}</strong><br>\
        \u{2022} Walzbronze: <strong>10,95 \u{20ac}</strong></p>\
        <p><strong>Aluminium:</strong><br>\
        \u{2022} Alu-Profile, blank: <strong>2,69 \u{20ac}</strong> / gestegt: <strong>1,80 \u{20ac}</strong> / lackiert: <strong>2,39 \u{20ac}</strong><br>\
        \u{2022} RAL-Draht, eisenfrei: <strong>2,51 \u{20ac}</strong><br>\
        \u{2022} Alu-Offset ohne Papier: <strong>2,45 \u{20ac}</strong><br>\
        \u{2022} AlMg-Blech, teilweise foliert: <strong>2,30 \u{20ac}</strong><br>\
        \u{2022} Alu-Guss-Felgen ohne Anhaftung: <strong>2,20 \u{20ac}</strong><br>\
        \u{2022} Alu-Geschirr ohne Fe: <strong>1,65 \u{20ac}</strong> / mit Fe: <strong>1,20 \u{20ac}</strong><br>\
        \u{2022} Alu-Guss ohne Fe: <strong>1,62 \u{20ac}</strong> / mit Fe: <strong>0,95 \u{20ac}</strong></p>\
        <p><strong>Zink / Blei:</strong><br>\
        \u{2022} Zinkblech, neu: <strong>1,97 \u{20ac}</strong> / alt: <strong>1,77 \u{20ac}</strong><br>\
        \u{2022} Altblei: <strong>0,93 \u{20ac}</strong></p>\
        <p><strong>Hinweis:</strong> Preise sind Richtwerte, tagesabh\u{00e4}ngig.";

    const FIXTURE_UPTO: &str = "<p><strong>Unsere Ankaufspreise pro kg</strong> (Richtwerte):<br>\
        \u{1f947} <strong>Kupferpreis</strong>: oft \u{00fc}ber 10 \u{20ac}/kg<br>\
        \u{1f948} <strong>Messingpreis</strong>: oft \u{00fc}ber 5 \u{20ac}/kg<br>\
        \u{1f949} <strong>Kabelpreis</strong>: bis zu 5 \u{20ac}/kg \u{2013} je nach Kupferanteil<br>\
        4. <strong>Edelstahlpreis</strong>: bis zu 1,20 \u{20ac}/kg \u{2013} Sp\u{00fc}len, T\u{00f6}pfe<br>\
        5. <strong>Alupreis</strong>: oft \u{00fc}ber 2,50 \u{20ac}/kg</p>";

    const FIXTURE_CARDS: &str = "<div class=\"metal-card\"><div class=\"metal-content\">\
        <h3 itemprop=\"name\">Kupfer (Cu)</h3></div></div>\
        <div class=\"metal-card\"><div class=\"metal-content\">\
        <h3 itemprop=\"name\">Edelstahl (V2A/V4A)</h3></div></div>\
        <div class=\"metal-card\"><div class=\"metal-content\">\
        <h3 itemprop=\"name\">Blei-Akkus</h3></div></div>\
        <div class=\"metal-card\"><div class=\"metal-content\">\
        <h3 itemprop=\"name\">Wolfram (W)</h3></div></div>\
        <div class=\"metal-card\"><div class=\"metal-content\">\
        <h3 itemprop=\"name\">Kabel & E-Schrott</h3></div></div>";

    fn live_rows() -> Vec<(String, f64)> {
        let (rows, skipped) = parse_prices(FIXTURE_TABLE).expect("parses");
        assert!(skipped.is_empty(), "{skipped:?}");
        rows
    }

    #[test]
    fn table_yields_31_rows() {
        let rows = live_rows();
        assert_eq!(rows.len(), 31, "{rows:?}");
    }

    #[test]
    fn anchor_missing_is_error() {
        assert!(parse_prices("<p>Kein Preis hier</p>").is_err());
        assert!(parse_prices("<p>Tagesaktuelle Sortenpreise, aber nichts dahinter").is_err());
    }

    #[test]
    fn mapping_resolves_grades_and_skips_composite() {
        let rows = live_rows();
        let get = |want: &str| {
            rows.iter()
                .find(|(l, _)| l.contains(want))
                .unwrap_or_else(|| panic!("missing {want}"))
                .clone()
        };
        // Brief-Richtwerte zuerst.
        let (l, p) = get("Millberry");
        assert_eq!((grade_for(&l), p), (Some(("kupfer-millberry", "")), 10.10));
        let (l, p) = get("H\u{00fc}lsen, sauber");
        assert_eq!((grade_for(&l), p), (Some(("messing", "huelsen")), 6.50));
        // Sorten trennen sich per Variante, sonst kollabieren Current-Preise.
        let (l, _) = get("\u{201e}Berry");
        assert!(!l.contains("Millberry"));
        assert_eq!(grade_for(&l), Some(("kupfer-berry", "")));
        assert_eq!(
            grade_for(&get("Kanal").0),
            Some(("kupfer-gemischt", "kanal"))
        );
        assert_eq!(
            grade_for(&get("Neublech").0),
            Some(("kupfer-gemischt", "neublech"))
        );
        assert_eq!(
            grade_for(&get("Schwerkupfer").0),
            Some(("kupfer-gemischt", "schwerkupfer-lotfrei"))
        );
        assert_eq!(
            grade_for(&get("Leichtkupfer").0),
            Some(("kupfer-gemischt", "leichtkupfer"))
        );
        // Messing 58: Schrott vs. Späne sind zwei Varianten.
        let ms58: Vec<_> = rows.iter().filter(|(l, _)| l.contains("58")).collect();
        assert_eq!(ms58.len(), 2);
        assert_eq!(grade_for(&ms58[0].0), Some(("messing", "ms58-schrott")));
        assert_eq!(ms58[0].1, 6.15);
        assert_eq!(grade_for(&ms58[1].0), Some(("messing", "ms58-spaene")));
        assert_eq!(ms58[1].1, 5.80);
        assert_eq!(
            grade_for(&get("63 Blech").0),
            Some(("messing", "ms63-blech"))
        );
        assert_eq!(
            grade_for(&get("Erodierdraht").0),
            Some(("messing", "erodierdraht"))
        );
        assert_eq!(
            grade_for(&get("Sp\u{00e4}ne, gemischt").0),
            Some(("messing", "spaene-gemischt"))
        );
        assert_eq!(
            grade_for(&get("ohne H\u{00fc}lsen").0),
            Some(("messing", "ohne-huelsen"))
        );
        // Composite ohne Katalogmaterial → laut skippen, nie raten.
        assert_eq!(grade_for(&get("K\u{00fc}hler").0), None);
        // Rotguss/Bronze.
        assert_eq!(
            grade_for(&get("Rotguss-Schrott:").0),
            Some(("bronze-rotguss", ""))
        );
        assert_eq!(
            grade_for(&get("Rotguss-Schrott: / Sp\u{00e4}ne").0),
            Some(("bronze-rotguss", "spaene"))
        );
        assert_eq!(
            grade_for(&get("Walzbronze").0),
            Some(("bronze-rotguss", "walzbronze"))
        );
        // Alu-Profile: blank/gestegt/lackiert trotz gemeinsamem Base-Label.
        let prof: Vec<_> = rows.iter().filter(|(l, _)| l.contains("Profile")).collect();
        assert_eq!(prof.len(), 3);
        assert_eq!(grade_for(&prof[0].0), Some(("aluminium-profile", "")));
        assert_eq!(prof[0].1, 2.69);
        assert_eq!(
            grade_for(&prof[1].0),
            Some(("aluminium-profile", "gestegt"))
        );
        assert_eq!(
            grade_for(&prof[2].0),
            Some(("aluminium-profile", "lackiert"))
        );
        assert_eq!(
            grade_for(&get("RAL-Draht").0),
            Some(("aluminium-gemischt", "ral-draht"))
        );
        assert_eq!(
            grade_for(&get("Offset").0),
            Some(("aluminium-blech", "offset"))
        );
        assert_eq!(grade_for(&get("AlMg").0), Some(("aluminium-blech", "almg")));
        assert_eq!(
            grade_for(&get("Felgen").0),
            Some(("aluminium-guss", "felgen"))
        );
        // Geschirr/Guss mit/ohne Fe: "mit" schlägt "ohne" (Base enthält beides).
        assert_eq!(
            grade_for(&get("Geschirr ohne").0),
            Some(("aluminium-gemischt", "geschirr-ohne-fe"))
        );
        let mit_fe: Vec<_> = rows
            .iter()
            .filter(|(l, _)| l.contains("/ mit Fe"))
            .collect();
        assert_eq!(mit_fe.len(), 2);
        assert_eq!(
            grade_for(&mit_fe[0].0),
            Some(("aluminium-gemischt", "geschirr-mit-fe"))
        );
        assert_eq!(grade_for(&mit_fe[1].0), Some(("aluminium-guss", "mit-fe")));
        assert_eq!(
            grade_for(&get("Alu-Guss ohne").0),
            Some(("aluminium-guss", "ohne-fe"))
        );
        // Zink neu/alt, Blei.
        let zink: Vec<_> = rows
            .iter()
            .filter(|(l, _)| l.contains("Zinkblech"))
            .collect();
        assert_eq!(zink.len(), 2);
        assert_eq!(grade_for(&zink[0].0), Some(("zink", "neu")));
        assert_eq!(grade_for(&zink[1].0), Some(("zink", "alt")));
        assert_eq!(grade_for(&get("Altblei").0), Some(("blei", "")));
    }

    #[test]
    fn upto_only_bis_zu_not_oft_ueber() {
        let upto = parse_upto(FIXTURE_UPTO);
        assert_eq!(upto.len(), 2);
        assert_eq!(upto[0].2, "edelstahl-gemischt");
        assert_eq!(upto[0].1, 1.20);
        assert_eq!(upto[1].2, "kabel-kupfer");
        assert_eq!(upto[1].1, 5.0);
        // "oft über" ist kein Preisversprechen → keine Zeilen dafür.
        assert!(!upto.iter().any(|(l, _, _)| l.contains("Kupferpreis")));
        assert!(!upto.iter().any(|(l, _, _)| l.contains("Alupreis")));
        // Fehlende Marketingzeile ist kein Fehler.
        assert!(parse_upto("<p>leer</p>").is_empty());
    }

    #[test]
    fn cards_map_and_skip_off_catalog() {
        let cards = parse_cards(FIXTURE_CARDS);
        assert_eq!(cards.len(), 5);
        assert_eq!(
            grade_for_card("Edelstahl (V2A/V4A)"),
            Some(vec![("edelstahl-v2a", ""), ("edelstahl-v4a", "")])
        );
        assert_eq!(
            grade_for_card("Kupfer (Cu)"),
            Some(vec![("kupfer-gemischt", "")])
        );
        assert_eq!(grade_for_card("Blei-Akkus"), None, "Batterien ≠ Weichblei");
        assert_eq!(grade_for_card("Wolfram (W)"), None, "kein Katalogmaterial");
        assert_eq!(
            grade_for_card("Kabel & E-Schrott"),
            None,
            "Composite ohne exakten Fit"
        );
        assert_eq!(
            grade_for_card("Hochleistungsstahl (HSS)"),
            Some(vec![("hss-werkzeuge", "")])
        );
        assert_eq!(
            grade_for_card("Hartmetall (VHM)"),
            Some(vec![("hartmetall", "")])
        );
        assert_eq!(
            grade_for_card("Mischschrott & Eisen"),
            Some(vec![("mischschrott", "")])
        );
        assert_eq!(grade_for_card("Zinn (Sn)"), Some(vec![("zinn", "")]));
    }

    #[test]
    fn impressum_resists_node_glue() {
        // Glue-Check: ohne Whitespace zwischen Blockelementen klebt
        // scraper-text() alles zusammen ("e.K.Heinrich…47b98544").
        let imp = "<h2>1. Anbieter und Verantwortlicher</h2><p>Wahl & Co.</p><p>Heinrich-Ehrhardt-Str. 47b<br>98544 Zella-Mehlis<br>Deutschland</p><h3>Kontakt</h3><p><strong>Telefon:</strong> 03682 483449<br><strong>E-Mail:</strong> wahl-@t-online.de<br></p><h2>2. Registereintrag</h2>";
        let info = super::extract_info(imp).expect("parses trotz Glue");
        assert_eq!(info.street, "Heinrich-Ehrhardt-Str. 47b");
        assert_eq!(info.postcode, "98544");
        assert_eq!(info.city, "Zella-Mehlis");
        assert_eq!(info.phone, "03682 483449");
        assert_eq!(info.email, "wahl-@t-online.de");
    }
    #[test]
    fn impressum_provider_block() {
        let imp = "<h2>1. Anbieter und Verantwortlicher</h2>\
            <div class=\"info-box\"><p><strong>Wahl & Co. Inhaber Thomas Madeiski e.K.</strong></p>\
            <p>Heinrich-Ehrhardt-Str. 47b<br>98544 Zella-Mehlis<br>Deutschland</p></div>\
            <h3>Kontakt</h3><p><strong>Telefon:</strong> 03682 483449<br>\
            <strong>E-Mail:</strong> wahl-@t-online.de<br></p>\
            <h2>2. Registereintrag</h2>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Heinrich-Ehrhardt-Str. 47b");
        assert_eq!(info.postcode, "98544");
        assert_eq!(info.city, "Zella-Mehlis");
        assert_eq!(info.phone, "03682 483449");
        assert_eq!(info.email, "wahl-@t-online.de");
        assert!(extract_info("<p>ohne Anbieter</p>").is_err());
    }
}
