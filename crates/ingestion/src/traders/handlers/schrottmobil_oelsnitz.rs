//! Schrottmobil Oelsnitz (Inh. Ralf Weßel): one-pager (`One.com Web
//! Editor`) with the September price list as six `<h2>` lines inside one
//! block (`"Derzeitige Ankaufpreise Monat September"` … `"Wir stellen uns
//! vor:"`), each line carrying bilingual label + price + `€/kg` in a
//! single heading (`"E3(6mm Stärke)/E3 (tloušťka6 mm) 0,14 €/ kg"`).
//! The price MUST be read as the number directly before `€`: the E3 label
//! itself starts with a digit, so `parse_eur` on the whole line would
//! return 3.0 instead of 0.14. The Impressum lives inline on the same
//! page (separate `/impressum*` URLs 404, live-verified 28.09.2026), so
//! both URLs point at the homepage and `extract_info` works on the
//! `"Impressum"` … `"Datenschutz"` heading window. Papier and Weiße Ware
//! have no catalog material and skip loudly; iron grades map to
//! `mischschrott` / `eisenschrott-gussbruch` (Grauguss vs. Bremsguss via
//! `variant`) / `stahlschrott-scheren` (E3 = schwerer Altschrott ≥6 mm).
//! Quoted unit is always `EUR/kg`; the pipeline normalizes into the
//! catalog `EUR/t`. The page names only the month ("September") plus
//! "Preise Tagesaktuell" — no calendar date, so `published_at` is `None`.

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sn-oelsnitz-vogtl-schrottmobil-inh-ralf-weel";
/// Bespoke, live-verified price URL (one-pager homepage). A move fails
/// the step loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://schrottmobil.info";
/// Bespoke, live-verified impressum URL. The Impressum is an inline block
/// on the one-pager (`/impressum*` 404s, verified 28.09.2026), so this is
/// deliberately the same page — `extract_info` anchors on the
/// `"Impressum"` heading, never on a guessed sub-path.
pub const IMPRESSUM_URL: &str = "https://schrottmobil.info";

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
    let mut seen: Vec<(&'static str, &'static str, u64)> = Vec::new();
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => {
                // Single-block page, but dedupe on (material, variant,
                // price) after mapping — a repeated block must never
                // collapse two grades into one current price nor double
                // the row count.
                let key = (material, variant, price.to_bits());
                if seen.contains(&key) {
                    continue;
                }
                seen.push(key);
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
                })
            }
            None => skipped_labels.push(label),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // block means the site changed and needs eyeballs before we trust
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

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific grades first (`"bremsguss"`/`"grauguss"` before bare
/// `"guss"`); the variant keeps the trader's own grade wording so the two
/// Gussbruch sorts never collapse into one current price. Papier and
/// Weiße Ware have no catalog material → `None` (loud skip at call site).
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("bremsguss") || l.contains("bremsscheib") {
        Some(("eisenschrott-gussbruch", "Bremsguss"))
    } else if l.contains("grauguss") {
        Some(("eisenschrott-gussbruch", "Grauguss"))
    } else if l.contains("guss") {
        Some(("eisenschrott-gussbruch", ""))
    } else if l.contains("e3") {
        // E3 = schwerer Altschrott (hier "6mm Stärke"): scherengerechtes
        // Altschrott-Material, kein Neuschrott (Sorte 1).
        Some(("stahlschrott-scheren", "E3"))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS inline impressum only: the `<h2>`
/// `"Impressum"` heading opens the block, `"Datenschutzerklärung"` closes
/// it. The address line reads `"Talsperrenstrasse 4-08606 Ölsnitz"`
/// (street and postcode glued with `-`), phone/email come from their
/// labeled `"Rufnummer:"` / `"E-Mail:"` lines. Missing anchors mean the
/// page changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let texts = h2_texts(imp);
    let start = texts
        .iter()
        .position(|t| t == "Impressum")
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        })?;
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let (mut phone, mut email) = (String::new(), String::new());
    for t in &texts[start + 1..] {
        let l = t.to_lowercase();
        if l.starts_with("datenschutz") {
            break;
        }
        if street.is_empty() && l.contains("talsperrenstrasse") {
            // "Talsperrenstrasse 4-08606 Ölsnitz": PLZ = first run of 5
            // digits; street = everything before (trailing "-" trimmed).
            if let Some(i) = digit_run_5(t) {
                postcode = t[i..i + 5].to_owned();
                street = t[..i].trim().trim_end_matches('-').trim().to_owned();
                city = t[i + 5..].trim().to_owned();
            }
        } else if phone.is_empty() && l.starts_with("rufnummer:") {
            phone = t
                .split_once(':')
                .map(|(_, v)| v.trim().to_owned())
                .unwrap_or_default();
        } else if email.is_empty() && l.starts_with("e-mail:") {
            let addr = t
                .split_once(':')
                .map(|(_, v)| v.trim().to_owned())
                .unwrap_or_default();
            if addr.contains('@') {
                email = addr;
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

/// Byte index of the first run of 5 ASCII digits (German PLZ), if any.
fn digit_run_5(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    b.windows(5)
        .position(|w| w.iter().all(|c| c.is_ascii_digit()))
}

/// All `<h2>` heading texts in order, tags stripped, entities/spaces
/// normalized. `<script>`/`<style>` never surface here — only content
/// headings, so CSS color values (`rgba(30,10,175,1)`) can't pair up with
/// labels into phantom prices.
fn h2_texts(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(a) = rest.find("<h2") {
        let after = &rest[a..];
        let Some(b) = after.find('>') else { break };
        let after = &after[b + 1..];
        let Some(c) = after.find("</h2>") else { break };
        let t = clean_text(&after[..c]);
        if !t.is_empty() {
            out.push(t);
        }
        rest = &after[c + 5..];
    }
    out
}

fn clean_text(raw: &str) -> String {
    let mut t = String::with_capacity(raw.len());
    let mut in_tag = false;
    for c in raw.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            t.push(c);
        }
    }
    t.replace("&nbsp;", " ")
        .replace("&#160;", " ")
        .replace(['\u{a0}'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
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
    // Only the price box: from its heading to the about-section run-out.
    // Both anchors are required — a redesign must fail loudly, never
    // silently harvest footer prose as prices.
    let start = html
        .find("Derzeitige Ankaufpreise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbox fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Wir stellen uns vor")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbox-Ende fehlt".to_owned(),
        })?;
    // No calendar date on the page (only month "September" + "Preise
    // Tagesaktuell"), so published_at stays None (observed_at = age).
    let published_at = None;
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for text in h2_texts(&tail[..end]) {
        if !text.contains('€') {
            continue;
        }
        if text.len() > 120 {
            continue;
        }
        // The price is the number directly before `€` — never the first
        // number on the line ("E3(6mm …)" would parse as 3.0).
        let Some((label, token, suffix)) = split_price(&text) else {
            if text.chars().any(|c| c.is_ascii_digit()) {
                skipped.push(format!("{text} (kein Preis)"));
            }
            continue;
        };
        let Some(price) = parse_eur(&token) else {
            skipped.push(format!("{text} (kein Preis: {token})"));
            continue;
        };
        // A "0,00" row is "no quote", not a free gift: loud skip.
        if price == 0.0 {
            skipped.push(format!("{label} (Preis 0,00)"));
            continue;
        }
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error. The
        // unit check reads only the suffix after the price — the bilingual
        // labels themselves contain "/" ("Papier/papír").
        let Some(unit) = unit_of(&suffix) else {
            skipped.push(format!("{label} (Einheit unverständlich: {suffix})"));
            continue;
        };
        if label.is_empty() {
            skipped.push(format!("{text} (ohne Bezeichnung)"));
            continue;
        }
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preiszeilen".to_owned(),
        });
    }
    Ok((published_at, rows, skipped))
}

/// Split `"E3(6mm Stärke)/E3 (tloušťka6 mm) 0,14 €/ kg"` into label
/// (`"E3(6mm Stärke)/E3 (tloušťka6 mm)"`), price token (`"0,14"`) and unit
/// suffix (`"/ kg"`), anchored on the last `€` carrying a number.
fn split_price(text: &str) -> Option<(String, String, String)> {
    let euro = text.char_indices().filter(|(_, c)| *c == '€').last()?.0;
    let before = &text[..euro];
    // Walk back over spaces, then over the number itself.
    let num_end = before.trim_end().len();
    let num_start = before[..num_end]
        .char_indices()
        .rev()
        .take_while(|(_, c)| c.is_ascii_digit() || *c == '.' || *c == ',')
        .last()
        .map(|(i, _)| i)?;
    let token = before[num_start..num_end]
        .trim_matches(['.', ','])
        .to_owned();
    if token.is_empty() || !token.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }
    Some((
        before[..num_start].trim().to_owned(),
        token,
        text[euro + '€'.len_utf8()..].trim().to_owned(),
    ))
}

/// Bespoke unit matcher for THIS price box's suffix (live: `"€/kg"`,
/// `"€ /kg"`, `"€/ kg"`). Only kg/t exist here — anything else skips
/// loudly at the call site.
fn unit_of(suffix: &str) -> Option<&'static str> {
    let lower = suffix.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|t| t == "t" || t == "to")
    {
        Some("EUR/t")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, split_price};

    // Real excerpts of the live one-pager (28.09.2026): entities and nested
    // spans kept verbatim, filler lines trimmed. The block holds exactly
    // six price headings between the month heading and the about section.
    const FIXTURE: &str = "<h2><span>&nbsp;&nbsp; &nbsp;&nbsp;<span>Derzeitige Ankaufpreise Monat September/zari</span></span></h2>\
        <h2><span>&nbsp;&nbsp; &nbsp; Papier/papír&nbsp; &nbsp;<span>&nbsp;0,11</span>&nbsp;<span>€/kg&nbsp; <span>Preise Tagesaktuell/</span></span>&nbsp;</span></h2>\
        <h2><span>&nbsp;&nbsp; &nbsp; Mischschrott/ smíšený odpad&nbsp; &nbsp;<span>0,10 € /kg&nbsp; &nbsp;</span></span></h2>\
        <h2><span>&nbsp;&nbsp; &nbsp; Grauguss/litina&nbsp; &nbsp;<span>&nbsp; 0,13 €/ kg</span></span></h2>\
        <h2><span>&nbsp;&nbsp; &nbsp; Bremsguss/brzda cast&nbsp; &nbsp;<span>0,14 €/ kg</span><span>&nbsp; &nbsp;&nbsp;</span></span></h2>\
        <h2><span>&nbsp;&nbsp; &nbsp; E3(6mm Stärke)/E3 (tloušťka6 mm)&nbsp; <span>0,14 €/ kg</span></span></h2>\
        <h2><span>&nbsp;&nbsp; &nbsp; Weisse Ware/Bílé zboží&nbsp; &nbsp;<span>0,05 €/ kg</span>&nbsp;</span></h2>\
        <h2><span>&nbsp;&nbsp; &nbsp; <span>Wir stellen uns vor:</span></span></h2>";

    #[test]
    fn window_parses_all_six_live_rows() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at, None, "nur Monatsname, kein Datum");
        assert!(skips.is_empty(), "{skips:?}");
        assert_eq!(rows.len(), 6);
        assert_eq!(rows[0], ("Papier/papír".to_owned(), 0.11, "EUR/kg"));
        assert_eq!(rows[1].0, "Mischschrott/ smíšený odpad");
        assert_eq!(rows[1].1, 0.10);
        assert_eq!(rows[2], ("Grauguss/litina".to_owned(), 0.13, "EUR/kg"));
        assert_eq!(rows[3].0, "Bremsguss/brzda cast");
        assert_eq!(rows[3].1, 0.14);
        assert_eq!(rows[4].0, "E3(6mm Stärke)/E3 (tloušťka6 mm)");
        assert_eq!(rows[4].1, 0.14);
        assert_eq!(rows[5].0, "Weisse Ware/Bílé zboží");
        assert_eq!(rows[5].1, 0.05);
        assert!(rows.iter().all(|r| r.2 == "EUR/kg"));
    }

    #[test]
    fn e3_label_yields_cents_not_first_number() {
        // parse_eur on the whole line would return 3.0 ("E3…"); the price
        // is the number directly before €.
        let (label, token, suffix) =
            split_price("E3(6mm Stärke)/E3 (tloušťka6 mm) 0,14 €/ kg").expect("splits");
        assert_eq!(label, "E3(6mm Stärke)/E3 (tloušťka6 mm)");
        assert_eq!(token, "0,14");
        assert_eq!(suffix, "/ kg");
        assert_eq!(
            super::parse_eur("E3(6mm Stärke)/E3 (tloušťka6 mm) 0,14 €/ kg"),
            Some(3.0)
        );
    }

    #[test]
    fn zero_price_skips_loudly() {
        let html = FIXTURE.replacen("0,10 € /kg", "0,00 € /kg", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 5);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Mischschrott") && skips[0].contains("0,00"));
    }

    #[test]
    fn anchors_and_units_fail_loudly() {
        let no_start = FIXTURE.replacen("Derzeitige Ankaufpreise", "Tagespreise", 1);
        assert!(parse(&no_start).is_err());
        let no_end = FIXTURE.replacen("Wir stellen uns vor", "Über uns", 1);
        assert!(parse(&no_end).is_err());
        // Explicit-but-foreign unit ("pro Sack") skips loudly; valid rows
        // survive. The bilingual "/" in labels must not count as a unit.
        let sack = FIXTURE.replacen("0,10 € /kg", "0,10 € pro Sack", 1);
        let (_, rows, skips) = parse(&sack).expect("parses");
        assert_eq!(rows.len(), 5);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Mischschrott") && skips[0].contains("Einheit"));
        let all_sack = FIXTURE
            .replace("€/kg", "€ pro Sack")
            .replace("€ /kg", "€ pro Sack")
            .replace("€/ kg", "€ pro Sack");
        assert!(parse(&all_sack).is_err());
    }

    #[test]
    fn mapping_covers_live_labels() {
        assert_eq!(
            grade_for("Mischschrott/ smíšený odpad"),
            Some(("mischschrott", ""))
        );
        assert_eq!(
            grade_for("Grauguss/litina"),
            Some(("eisenschrott-gussbruch", "Grauguss"))
        );
        assert_eq!(
            grade_for("Bremsguss/brzda cast"),
            Some(("eisenschrott-gussbruch", "Bremsguss"))
        );
        assert_eq!(
            grade_for("E3(6mm Stärke)/E3 (tloušťka6 mm)"),
            Some(("stahlschrott-scheren", "E3"))
        );
        // No catalog material for paper or white goods: loud skips.
        assert_eq!(grade_for("Papier/papír"), None);
        assert_eq!(grade_for("Weisse Ware/Bílé zboží"), None);
        assert_eq!(grade_for("Weiße Ware"), None);
    }

    #[test]
    fn impressum_extracts_inline_contact() {
        // Real fragment shapes: glued "4-08606 Ölsnitz", labeled rows.
        let imp = "<h2><span>Impressum</span></h2>\
            <h2><span> Betreiber der Seite: Fa. Schrottmobil,Inh. Ralf Weßel</span></h2>\
            <h2><span> Talsperrenstrasse 4-08606 Ölsnitz</span></h2>\
            <h2><span> Rufnummer:&nbsp; &nbsp; 017621793134</span></h2>\
            <h2><span> E-Mail:&nbsp; &nbsp; eisenschrotti@web.de</span></h2>\
            <h2><span>Datenschutzerklärung</span></h2>\
            <h2><span>Rufnummer: 00000 nach Datenschutz</span></h2>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Talsperrenstrasse 4");
        assert_eq!(info.postcode, "08606");
        assert_eq!(info.city, "Ölsnitz");
        assert_eq!(info.phone, "017621793134");
        assert_eq!(info.email, "eisenschrotti@web.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<h2><span>Neu hier</span></h2>").is_err());
    }
}
