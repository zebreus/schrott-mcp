//! Efrem Gouchev Schrottankauf (Berlin-Marzahn): exact daily prices per
//! grade (`div#price > div.cms-article.<sorte>`: title in `p.h5`, base
//! price behind "ab 1 kg ➜"). The page renews every morning by 9:00 and
//! carries no per-row date, so `published_at` stays `None` (the
//! observation age is the provenance).
//!
//! One row per block: the ab-1-kg Bar-Sockel (quantity tiers and
//! Überweisung rows are dropped on purpose — one more variant dimension
//! would burst the catalog). The "Aluminiumkabel" block carries a second
//! sort ("ALUKABEL DICK 0,35€") and yields a second row with its own
//! variant. "Scherenschrott / Gussschrott" names two iron grades at one
//! price and is skipped loudly (ambiguous, never crammed).
//!
//! Units: the page quotes no per-row unit; the site's own banner prices
//! "0,10 Euro pro kg für Buntmetalle", and every value is kg-plausible.
//! `unit_of` therefore defaults to EUR/kg but still rejects any explicit
//! foreign unit — a per-tonne quote recorded as per-kg would be a 1000x
//! error.

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

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some(variants) => {
                for (material, variant) in variants {
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
                        label: label.clone(),
                    });
                }
            }
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

/// Explicit block title → material rows. Specific before generic:
/// "Millberry"/"Kerze"/"Schwer" must not fall into plain copper,
/// "Aluminiumkabel" must not fall into plain aluminium, lead cable grades
/// must not fall into "Altblei" unwatched. One label may fan out
/// ("Aluminiumkabel" + "ALUKABEL DICK" → two kabel-alu variants).
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(vec![("kupfer-millberry", "")])
    } else if l.contains("kerze") && l.contains("kupfer") {
        Some(vec![("kupfer-berry", "Kerze")])
    } else if l.contains("schwer") && l.contains("kupfer") {
        Some(vec![("kupfer-berry", "Schwer")])
    } else if l.contains("kupferkabelschrott") && l.contains("stecker") {
        Some(vec![("kabel-kupfer", "mit Stecker")])
    } else if l.contains("kupferkabel") || l.contains("kupfer-kabel") {
        Some(vec![("kabel-kupfer", "")])
    } else if l.contains("aluminiumkabel") || l.contains("alukabel") {
        // Two sorts, one block ("ab 1 kg ➜ 0,10€ / ALUKABEL DICK 0,35€"):
        // two variants, or they collapse onto one arbitrary price.
        Some(vec![("kabel-alu", ""), ("kabel-alu", "dick")])
    } else if l.contains("elektromotor") {
        Some(vec![("elektromotoren", "")])
    } else if l.contains("lötzinn") || l.contains("loetzinn") {
        Some(vec![("zinn", "Lötzinn")])
    } else if l.contains("zinn") {
        // Grades: the range IS the grade ("Zinnschrott 90-95 % (Teller)").
        if l.contains("90") {
            Some(vec![("zinn", "90-95%")])
        } else {
            Some(vec![("zinn", "")])
        }
    } else if l.contains("auswuchtblei") {
        Some(vec![("blei", "Auswuchtblei")])
    } else if l.contains("schälblei") || l.contains("schaelblei") {
        Some(vec![("blei", "Kabelschälblei")])
    } else if l.contains("altblei") || (l.contains("blei") && !l.contains("kabel")) {
        Some(vec![("blei", "")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("mischschrott") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("scherenschrott") || l.contains("gussschrott") {
        // One price for two iron grades — ambiguous, skip loudly.
        None
    } else if l.contains("v2a") || l.contains("edelstahl") {
        Some(vec![("edelstahl-v2a", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("aluminium") {
        if l.contains("5%") || l.contains("anhaftung") && l.contains("max") {
            Some(vec![("aluminium-gemischt", "5% Anhaftung")])
        } else {
            Some(vec![("aluminium-gemischt", "")])
        }
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else {
        None
    }
}

/// Parse the `div#price` blocks. Returns (rows, skips); the Aluminiumkabel
/// block yields two rows (base + DICK). An empty listing is a loud error,
/// never a silent success.
fn parse(
    html: &str,
) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html.find("id=\"price\"").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Tagespreis-Block fehlt".to_owned(),
    })?;
    let frag = Html::parse_fragment(&format!("<div>{}</div>", &html[start..]));
    let price_sel = Selector::parse("div#price").expect("valid selector");
    let block_sel = Selector::parse("div.cms-article").expect("valid selector");
    let h5_sel = Selector::parse("p.h5").expect("valid selector");
    let p_sel = Selector::parse("p").expect("valid selector");
    let price_box = frag.select(&price_sel).next().ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Tagespreis-Block fehlt".to_owned(),
    })?;
    let mut rows = Vec::new();
    let mut skips = Vec::new();
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
        let body: String = block.select(&p_sel).skip(1).map(|el| el.text().collect::<String>()).collect::<Vec<_>>().join(" ");
        let body = body.split_whitespace().collect::<Vec<_>>().join(" ");
        // An explicit foreign unit anywhere in the block rejects the
        // site-default loudly instead of silently mis-scaling.
        let Some(unit) = unit_of(&body) else {
            skips.push(format!("{title} (Einheit unverständlich: {body})"));
            continue;
        };
        // Base price = the "ab 1 kg" quote (Bar-Sockel).
        let base_text = body.split_once("ab 1 kg").map(|(_, rest)| rest).unwrap_or(&body);
        let Some(price) = parse_eur(base_text) else {
            skips.push(format!("{title} (kein ab-1-kg-Preis)"));
            continue;
        };
        // Dedupe against double blocks: same (title, price) twice counts once.
        if !rows.iter().any(|(t, p, _): &(String, f64, &'static str)| *t == title && *p == price) {
            rows.push((title.clone(), price, unit));
        }
        // Second sort inside the Aluminiumkabel block ("ALUKABEL DICK 0,35€").
        if let Some((_, dick_text)) = body.split_once("DICK") {
            if let Some(dick) = parse_eur(dick_text) {
                if (dick - price).abs() > f64::EPSILON
                    && !rows.iter().any(|(t, p, _): &(String, f64, &'static str)| {
                        *t == title && (*p - dick).abs() <= f64::EPSILON
                    })
                {
                    rows.push((format!("{title} dick"), dick, unit));
                }
            }
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Tagespreise leer".to_owned() });
    }
    // Fan-out helper: the DICK row maps through the same label table
    // (its "dick" suffix keeps the kabel-alu/dick variant reachable even
    // if the base row is skipped).
    Ok((rows, skips))
}

/// Bespoke unit gate for THESE blocks: the site quotes "Euro pro kg"
/// (banner above the listing) and nothing else — but any explicit foreign
/// unit still rejects the block loudly.
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
            .map(|el| el.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" "))
            .unwrap_or_default()
    };
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    let (mut phone, mut email) = (String::new(), String::new());
    for (k, label) in labels.iter().enumerate() {
        match label.trim() {
            "Adresse" => street = text_of(k),
            "Stadt" => city = text_of(k),
            "PLZ" => postcode = text_of(k).split_whitespace().next().unwrap_or_default().to_owned(),
            "E-Mail" => {
                // "info ∂ schrottankauf-bitterfelderstr23.de" — the ∂
                // glyph (also in data-email JSON) joins the halves.
                email = text_of(k).replace('∂', "@").split_whitespace().collect::<Vec<_>>().join("");
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
    Ok(TraderInfo { street, postcode, city, phone, email })
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    // Real shape of the live listing (cms-article classes, h5 title spans,
    // "ab 1 kg ➜" quotes, the ALUKABEL-DICK second sort), trimmed to four
    // blocks.
    const FIXTURE: &str = "<div id=\"price\" class=\"cms-container-el price-container\">\
        <div class=\"cms-article millberry lazy-bg\">\
        <p class=\"h5\"><span style=\"font-size: 36px;\">Kupfer Millberry</span> <span>nicht angelaufen, nicht lackiert</span></p>\
        <p><span style=\"font-size: 36px;\">ab 1 kg ➜ <strong>11,15</strong><strong>€</strong></span><br/>\
        <span style=\"font-size: 24px;\">ab 200kg: <em>11,35€</em>/ 11,15€ Überweisung/ Bar</span></p></div>\
        <div class=\"cms-article aluminiumkabel lazy-bg\">\
        <p class=\"h5\">Aluminiumkabel</p>\
        <p><span style=\"font-size: 36px;\">ab 1 kg ➜ <strong>0,10€</strong></span><br/>ALUKABEL DICK 0,35€</p></div>\
        <div class=\"cms-article guss lazy-bg\">\
        <p class=\"h5\">Scherenschrott / Gussschrott</p>\
        <p><span style=\"font-size: 36px;\">ab 1 kg ➜ <strong>0,12€</strong></span></p></div>\
        <div class=\"cms-article zinnschrott lazy-bg\">\
        <p class=\"h5\">Zinnschrott Lötzinn</p>\
        <p><span style=\"font-size: 36px;\">ab 1 kg ➜ <strong>5,00€</strong></span></p></div>\
        </div>";

    #[test]
    fn blocks_parse_and_dick_fans_out() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty());
        // Millberry + Alukabel base + Alukabel dick + Scheren/Guss + Lötzinn.
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0], ("Kupfer Millberry nicht angelaufen, nicht lackiert".to_owned(), 11.15, "EUR/kg"));
        assert_eq!(rows[1], ("Aluminiumkabel".to_owned(), 0.1, "EUR/kg"));
        assert_eq!(rows[2], ("Aluminiumkabel dick".to_owned(), 0.35, "EUR/kg"));
        assert_eq!(unit_of("ab 1 kg 10,40 €"), Some("EUR/kg"));
        assert_eq!(unit_of("pauschal 5 €"), None);
        assert!(parse("<div>Redesign ohne Preise</div>").is_err());
    }

    #[test]
    fn mapping_splits_and_skips_ambiguity() {
        assert_eq!(grade_for("Kupfer Millberry nicht angelaufen, nicht lackiert"), Some(vec![("kupfer-millberry", "")]));
        assert_eq!(grade_for("Kupfer ohne Eisen- oder Messinganhaftungen"), Some(vec![("kupfer-gemischt", "")]));
        assert_eq!(grade_for("Kupfer Schwer (ohne Lötstellen und Farbe)"), Some(vec![("kupfer-berry", "Schwer")]));
        assert_eq!(grade_for("Kupfer Kerze (neu, ohne Anhaftung, nicht angelaufen)"), Some(vec![("kupfer-berry", "Kerze")]));
        assert_eq!(grade_for("Kupferkabel kein Antennen-, Fett-, ALCU-, Eisenkabel"), Some(vec![("kabel-kupfer", "")]));
        assert_eq!(grade_for("Kupferkabelschrott mit Stecker"), Some(vec![("kabel-kupfer", "mit Stecker")]));
        assert_eq!(grade_for("Aluminiumkabel dick"), Some(vec![("kabel-alu", ""), ("kabel-alu", "dick")]));
        assert_eq!(grade_for("Edelstahlschrott (V2A)"), Some(vec![("edelstahl-v2a", "")]));
        assert_eq!(grade_for("Zinnschrott 90-95 % (Teller)"), Some(vec![("zinn", "90-95%")]));
        assert_eq!(grade_for("Zinnschrott Lötzinn"), Some(vec![("zinn", "Lötzinn")]));
        assert_eq!(grade_for("Auswuchtblei"), Some(vec![("blei", "Auswuchtblei")]));
        assert_eq!(grade_for("Scherenschrott / Gussschrott"), None);
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
