//! Koppe Schrottplatz Strausberg: exact per-kg list prices in plain
//! `<p><strong>* Label … €</strong></>` lines (no table), split into three
//! headed sections (Altpapier / Metall / EDV — all "Preise je Kilogramm").
//! Live 27.09.2026: 1 paper + 26 metal + 23 EDV rows, no page date.
//! Leiterplatten map to `platinen` (with Sorte variants); CPUs, RAM, ICs,
//! Festplatten/Laufwerke/Netzteile/Laptops, Altpapier, KFZ-Karosserie,
//! Elo-Leitschienen and Alt-Batterien have no catalog material and are
//! skipped loudly (see proposals in `grade_for`).

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bb-strausberg-koppe-andreas-schrottplatz-strausberg";
/// Bespoke, live-verified impressum URL (site nav "Impressum" link). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.schrottplatz-strausberg.de/impressum/";

pub const URL: &str = "https://www.schrottplatz-strausberg.de/preise/";

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
    for (label, price, unit) in rows {
        match grade_for(&label) {
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

/// Explicit label → (material, variant) mapping, specific-before-generic.
/// Anything unlisted returns None (loud skip at the call site).
///
/// Catalog-gap proposals (do NOT cram):
/// - "Altpapier gemischt" → new `papier` material.
/// - "KFZ (Karosserie)" → no auto-body material (not `mischschrott`: bodies
///   are a distinct grade with coatings/interior residue).
/// - "Elo-Leitschienen" → unproven: lacquered copper rails would be
///   `kupfer-berry`, tinned ones something else; the page does not say.
/// - "Alt Batterien" → new `batterien` material (Starterbatterien).
/// - "CPU …" → new `cpu` material (ceramic/goldcap/plastic are own grades).
/// - "RAM Gold/Silber" → new `ram` material.
/// - "ICs Keramik/Kunststoff" → new `ic` material (or fold into `cpu`?).
/// - "Festplatten", "Laufwerke", "Netzteile mit Kabel",
///   "Laptop mit/ohne Display" → new `e-schrott-geraete` material?
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("serverrückwand") || l.contains("serverrueckwand") {
        Some(("platinen", "Serverrückwande"))
    } else if l.contains("leiterplatte") {
        // Eight priced grades, one material: variants keep them apart.
        if l.contains("1 a") {
            Some(("platinen", "Sorte 1 A"))
        } else if l.contains("1 b") {
            Some(("platinen", "Sorte 1 B"))
        } else if l.contains("2 a") {
            Some(("platinen", "Sorte 2 A"))
        } else if l.contains("2 b") {
            Some(("platinen", "Sorte 2 B"))
        } else if l.contains("laptop") {
            Some(("platinen", "Laptop"))
        } else if l.contains("servereinschub") {
            Some(("platinen", "Servereinschub"))
        } else if l.contains("sorte 3") {
            Some(("platinen", "Sorte 3"))
        } else {
            Some(("platinen", ""))
        }
    } else if l.contains("steckkarte") {
        Some(("platinen", "Steckkarte ohne Blende"))
    } else if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kupfer raff") {
        Some(("kupfer-gemischt", "Raff"))
    } else if l.contains("kupferschrott") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("cu kabel") || l.contains("haushaltskabel") {
        if l.contains("70%") {
            Some(("kabel-kupfer", "70%+"))
        } else if l.contains("50%") {
            Some(("kabel-kupfer", "50%+"))
        } else {
            Some(("kabel-kupfer", "38-40%"))
        }
    } else if l.contains("kabel mit stecker") {
        Some(("kabel-kupfer", "mit Stecker"))
    } else if l.contains("hülsen") || l.contains("huelsen") {
        Some(("messing", "Hülsen"))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("zinn") {
        Some(("zinn", ""))
    } else if l.contains("auswuchtblei") {
        Some(("blei-auswucht", "Auswucht"))
    } else if l.contains("altblei") {
        Some(("blei", ""))
    } else if l.contains("elektromotor") || l.contains("elektormotore") {
        // Live typo: "Elektormotore" (missing r) — tolerated with test.
        if l.contains("getriebe") {
            Some(("elektromotoren", "mit Getriebe"))
        } else {
            Some(("elektromotoren", ""))
        }
    } else if l.contains("edelstahl") {
        Some(("edelstahl-gemischt", ""))
    } else if l.contains("alu profile") {
        Some(("aluminium-profile", ""))
    } else if l.contains("aluminium") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("schwere schere") {
        // Deliberation: "Schere"/"Schwere Schere" sit in the steel list next
        // to Mischschrott/Schreddervormaterial at shear-scrap prices, so
        // `stahlschrott-scheren` ("Scherenschrott") is proven; the variant
        // keeps the heavy grade from collapsing into the standard one.
        Some(("stahlschrott-scheren", "schwer"))
    } else if l.contains("schreddervormaterial") {
        Some(("stahlschrott-shredder", ""))
    } else if l.contains("schere") {
        Some(("stahlschrott-scheren", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("guß") || l.contains("guss") {
        Some(("eisenschrott-gussbruch", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only, anchored on the
/// verbatim "Firmen Haupsitz:" heading (page typo, kept as-is), the
/// "Verantwortlich:" block and the "Kontakt:" block. Missing anchors mean
/// the page changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let anchor = "Firmen Haupsitz:";
    let i = imp.find(anchor).ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Haupsitz-Block fehlt".to_owned(),
    })?;
    let tail = &imp[i..];
    let end = tail.find("Registereintrag").unwrap_or(tail.len());
    // Lines per <p>, split on <br> inside each: raw splitting glues
    // separate <p> elements without <br> between them, and scraper
    // text() would glue "Chaussee 15" and "15344" into one token —
    // so work <p>-by-<p>, line-wise.
    let frag = Html::parse_fragment(&format!("<div>{}</div>", &tail[..end]));
    let p_sel = Selector::parse("p").expect("valid selector");
    let mut lines: Vec<String> = Vec::new();
    for el in frag.select(&p_sel) {
        for part in el.inner_html().split("<br") {
            let t = strip_fragment(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
    // "Klosterdorfer Chaussee 15" is the line before the PLZ line
    // ("15344 Strausberg").
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5
                && pc.chars().all(|c| c.is_ascii_digit())
                && ci.chars().next().is_some_and(|c| c.is_uppercase())
            {
                postcode = pc.to_owned();
                city = ci.trim_matches(',').to_owned();
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                break;
            }
        }
    }
    // Phone: token run after "Telefon:"; email: '@'-expansion (glued
    // neighbours like "…deRegistereintrag" defeat token splitting).
    let flat = lines.join(" ");
    let phone = phone_after(&flat, "Telefon:");
    let email = email_token(&flat);
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

/// Phone-ish token run after a marker ("Telefon: +49 1606775140").
fn phone_after(text: &str, marker: &str) -> String {
    text.find(marker).map_or_else(String::new, |i| {
        text[i + marker.len()..]
            .split_whitespace()
            .take_while(|t| {
                t.chars()
                    .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
            })
            .collect::<Vec<_>>()
            .join(" ")
    })
}

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant (`/>`) — drop everything up to the first '>' first, or the
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
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// First email address in the text: expand from the '@' over email
/// characters (glued neighbours defeat token splitting). Cut at the
/// domain end so trailing prose ("…deRegistereintrag") never sticks.
fn email_token(r: &str) -> String {
    let Some(at) = r.find('@') else {
        return String::new();
    };
    let b = r.as_bytes();
    let is_email = |c: u8| c.is_ascii_alphanumeric() || b".-_+@".contains(&c);
    let mut s = at;
    while s > 0 && is_email(b[s - 1]) {
        s -= 1;
    }
    let mut e = at + 1;
    while e < b.len() && is_email(b[e]) {
        e += 1;
    }
    let cand = &r[s..e];
    for suffix in [".de", ".com", ".net", ".org", ".eu", ".info", ".biz"] {
        if let Some(p) = cand.rfind(suffix) {
            let cut = cand[..p + suffix.len()].to_owned();
            if cut.contains('@') && !cut.starts_with('@') {
                return cut;
            }
        }
    }
    String::new()
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
    // Window: price sections only — sidebar address/phone and footer must
    // never pair a stray € with a label into a phantom price.
    let start = html
        .find("Ankaufspreise für Altpapier")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankaufspreise-Block fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("Mitteilungen").unwrap_or(tail.len());
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    // Only content elements, never scripts/styles: rows are <p>/<li>.
    let sel = Selector::parse("p, li").expect("valid selector");
    let mut rows = Vec::new();
    let mut unit_skips = Vec::new();
    for el in doc.select(&sel) {
        // join("") on purpose: live markup splits prices across <strong>
        // nodes ("7,0"+"0 €", "0,15"+" €") which belong together.
        let raw: String = el.text().collect::<Vec<_>>().join("");
        let text = raw.replace(['\u{a0}'], " ");
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if !text.contains('€') {
            continue; // headers ("Ankaufpreise für Metall:"), note lines.
        }
        let Some((label, price)) = split_price(&text) else {
            continue;
        };
        if label.is_empty() || label.len() > 120 {
            continue; // €-prose, never a label.
        }
        let Some(unit) = unit_of(&text) else {
            unit_skips.push(format!("{label} (Einheit unverständlich: {text})"));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste leer".to_owned(),
        });
    }
    // No page date anywhere (live checked) → None (observed_at = age).
    Ok((None, rows, unit_skips))
}

/// Split "… * Label 190,00 €" into label + price. The price is the LAST
/// whitespace token before the € — labels carry their own numbers ("Intel
/// 286/386/486", "CU Kabel 70%+", "Sorte 3"), so `parse_eur` on the whole
/// tail would return 286 instead of 190. Split `<strong>` nodes glue
/// without separator in scraper `text()` ("3,0"+"0 €" → "3,00 €"), so the
/// last token is always the full price on this page.
fn split_price(text: &str) -> Option<(String, f64)> {
    let euro = text.find('€')?;
    let left = text[..euro].trim();
    if left.is_empty() {
        return None;
    }
    let cut = left.rfind(char::is_whitespace)?;
    let price = parse_eur(left[cut..].trim())?;
    let label = left[..cut].trim().trim_start_matches('*').trim().to_owned();
    if label.is_empty() {
        return None;
    }
    Some((label, price))
}

/// Bespoke unit rule for THIS page: every section header states "Preise je
/// Kilogramm" (metal section with typo "Kilogrmm"), so EUR/kg is the
/// documented page default. An explicitly foreign unit still skips loudly —
/// a per-tonne price recorded as per-kg would be a 1000x error.
fn unit_of(row: &str) -> Option<&'static str> {
    let lower = row.to_lowercase();
    if lower.contains("pro tonne")
        || lower.contains("pro to")
        || lower.contains("/t")
        || lower.contains("pro sack")
    {
        None
    } else {
        Some("EUR/kg")
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, split_price};

    // Real live-HTML excerpts (only whitespace shortened): split <strong>
    // prices, the "Kilogrmm" header, the auf-Anfrage terminator.
    const FIXTURE: &str = concat!(
        "<h1><span> Preise</span></h1>",
        "<p><strong>Ankaufspreise für Altpapier,</strong> <strong>Preise je Kilogramm.</strong></p>",
        "<ul><li><strong>Altpapier gemischt \u{a0}\u{a0} 0,07 €</strong></li></ul>",
        "<p><strong>Ankaufpreise für Metall:</strong></p>",
        "<p><strong>Preise je Kilogrmm:</strong></p>",
        "<p><strong>* Schreddervormaterial \u{a0} 0,04 €</strong></p>",
        "<p style=\"margin-bottom: 0cm;\"><strong>* Schwere Schere 0,15</strong> €</p>",
        "<p><strong>* Millberry \u{a0} 7,00 €</strong></p>",
        "<p><strong>*\u{a0} Steckkarte ohne Blende</strong>\u{a0} <strong>7,0</strong><strong>0 €</strong></p>",
        "<p><strong>* CU Kabel 70%+ 3,0</strong><strong>0 €</strong></p>",
        "<p><strong>* \u{a0}CPU Keramik, Intel 286/386/486\u{a0} 190,00 €</strong></p>",
        "<p><span>Andere Schrottsorten</span> <span>auf</span> <span>Anfrage</span></p>",
        "<p><strong>Ankaufspreise für EDV</strong></p>",
        "<p><strong>Preise je Kilogramm</strong></p>",
        "<p><strong>* Leiterplatte Sorte 1 A \u{a0} 4,00 €</strong></p>",
        "<p><strong>* Leiterplatte Sorte 3 \u{a0} 0,20 €</strong></p>",
        "<p><strong>Mitteilungen</strong></p>",
    );

    #[test]
    fn rows_split_and_priced() {
        let (_, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 9, "{rows:?}");
        assert!(skips.is_empty());
        let get = |l: &str| {
            rows.iter()
                .find(|(x, _, _)| x == l)
                .map(|(_, p, u)| (*p, *u))
        };
        assert_eq!(get("Schreddervormaterial"), Some((0.04, "EUR/kg")));
        assert_eq!(get("Schwere Schere"), Some((0.15, "EUR/kg")));
        assert_eq!(get("Millberry"), Some((7.0, "EUR/kg")));
        assert_eq!(get("Steckkarte ohne Blende"), Some((7.0, "EUR/kg")));
        assert_eq!(get("CU Kabel 70%+"), Some((3.0, "EUR/kg")));
        assert_eq!(get("Leiterplatte Sorte 1 A"), Some((4.0, "EUR/kg")));
        // "Sorte 3 … 0,20": the price is the last token, not "3 0,20".
        assert_eq!(get("Leiterplatte Sorte 3"), Some((0.2, "EUR/kg")));
        assert_eq!(get("Altpapier gemischt"), Some((0.07, "EUR/kg")));
        // The CPU row parses (price = last number, not 286).
        assert_eq!(
            get("CPU Keramik, Intel 286/386/486"),
            Some((190.0, "EUR/kg"))
        );
    }

    #[test]
    fn last_number_wins_and_garbage_skipped() {
        assert_eq!(
            split_price("* CU Kabel 70%+ 3,00 €"),
            Some(("CU Kabel 70%+".to_owned(), 3.0))
        );
        // Header without price: no row.
        let html = "<p>Ankaufspreise für Altpapier, Preise je Kilogramm.</p><p>Mitteilungen</p>";
        assert!(parse(html).is_err());
        // Foreign unit skips loudly, valid rows survive.
        let html = FIXTURE.replacen("7,00 €", "7,00 € pro Sack", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 8);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Millberry"));
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<p><strong>Firmen Haupsitz:</strong></p>\
            <p>Strausberg,Klosterdorfer Chaussee 15</p>\
            <p><strong>Verantwortlich:</strong></p>\
            <p>Immobilien-aGenthen UG (haftungsbeschränkt)</p>\
            <p>Klosterdorfer Chaussee 15<br/>15344 Strausberg</p>\
            <p><strong>Kontakt:</strong><br/>Telefon: +49 1606775140<br/>\
            E-Mail: info@schrottplatz-strausberg.de</p>\
            <p><strong>Registereintrag</strong></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Klosterdorfer Chaussee 15");
        assert_eq!(info.postcode, "15344");
        assert_eq!(info.city, "Strausberg");
        assert_eq!(info.phone, "+49 1606775140");
        assert_eq!(info.email, "info@schrottplatz-strausberg.de");
        assert!(extract_info("<p>Neu hier</p>").is_err());
    }

    #[test]
    fn mapping_covers_every_live_label() {
        // Full live label set 27.09.2026: mapped…
        assert_eq!(grade_for("Millberry"), Some(("kupfer-millberry", "")));
        assert_eq!(grade_for("Kupferschrott"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Kupfer Raff"), Some(("kupfer-gemischt", "Raff")));
        assert_eq!(grade_for("CU Kabel 70%+"), Some(("kabel-kupfer", "70%+")));
        assert_eq!(grade_for("CU Kabel 50%+"), Some(("kabel-kupfer", "50%+")));
        assert_eq!(
            grade_for("Haushaltskabel 38%-40%"),
            Some(("kabel-kupfer", "38-40%"))
        );
        assert_eq!(
            grade_for("Kabel mit Stecker"),
            Some(("kabel-kupfer", "mit Stecker"))
        );
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Messing Hülsen"), Some(("messing", "Hülsen")));
        assert_eq!(grade_for("Altblei"), Some(("blei", "")));
        assert_eq!(
            grade_for("Auswuchtblei"),
            Some(("blei-auswucht", "Auswucht"))
        );
        assert_eq!(grade_for("Elektormotore"), Some(("elektromotoren", "")));
        assert_eq!(
            grade_for("Elektromotore mit Getriebe"),
            Some(("elektromotoren", "mit Getriebe"))
        );
        assert_eq!(
            grade_for("Edelstahlschrott"),
            Some(("edelstahl-gemischt", ""))
        );
        assert_eq!(grade_for("Alu Profile"), Some(("aluminium-profile", "")));
        assert_eq!(grade_for("Aluminium"), Some(("aluminium-gemischt", "")));
        assert_eq!(grade_for("Schere"), Some(("stahlschrott-scheren", "")));
        assert_eq!(
            grade_for("Schwere Schere"),
            Some(("stahlschrott-scheren", "schwer"))
        );
        assert_eq!(
            grade_for("Schreddervormaterial"),
            Some(("stahlschrott-shredder", ""))
        );
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Guß"), Some(("eisenschrott-gussbruch", "")));
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Zinn"), Some(("zinn", "")));
        assert_eq!(
            grade_for("Serverrückwande"),
            Some(("platinen", "Serverrückwande"))
        );
        assert_eq!(
            grade_for("Leiterplatte Sorte 1 A"),
            Some(("platinen", "Sorte 1 A"))
        );
        assert_eq!(
            grade_for("Leiterplatte Sorte 1 B"),
            Some(("platinen", "Sorte 1 B"))
        );
        assert_eq!(
            grade_for("Leiterplatte Sorte 2 A"),
            Some(("platinen", "Sorte 2 A"))
        );
        assert_eq!(
            grade_for("Leiterplatte Sorte 2 B"),
            Some(("platinen", "Sorte 2 B"))
        );
        assert_eq!(
            grade_for("Leiterplatte Sorte 3"),
            Some(("platinen", "Sorte 3"))
        );
        assert_eq!(
            grade_for("Leiterplatte Laptop"),
            Some(("platinen", "Laptop"))
        );
        assert_eq!(
            grade_for("Leiterplatte Servereinschub"),
            Some(("platinen", "Servereinschub"))
        );
        assert_eq!(
            grade_for("Steckkarte ohne Blende"),
            Some(("platinen", "Steckkarte ohne Blende"))
        );
        // …and loudly skipped (no catalog material, see proposals above).
        for l in [
            "Altpapier gemischt",
            "KFZ (Karosserie)",
            "Elo-Leitschienen",
            "Alt Batterien",
            "CPU Keramik, Intel 286/386/486",
            "CPU Keramik, mit Goldcap",
            "CPU Kunststoff, grün-braun",
            "CPU Kunststoff mit Kühlkörper",
            "CPU Slot",
            "RAM Gold",
            "RAM Silber",
            "ICs Keramik",
            "ICs Kunststoff",
            "Festplatten",
            "Laufwerke",
            "Netzteile mit Kabel",
            "Laptop mit Display ohne Akku",
            "Laptop ohne Display, ohne Akku",
        ] {
            assert_eq!(grade_for(l), None, "{l}");
        }
    }
}
