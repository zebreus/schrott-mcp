//! Wertstoff-Bauer, Zschopau (Inh. Micha Bauer): exact prices in
//! twelve two-column `<table class="Table_texttable__2GrpI">` blocks
//! ("↳ label" | "8,80 €"), windowed between the bold 48px
//! "Preisübersicht" content heading (last occurrence — nav links precede
//! it) and the "Weitere nicht gelistete Metalle auf Anfrage." terminator.
//! The "Preis/Kg" header tables document EUR/kg; "€ / Tonne" cells are
//! EUR/t. The page date is "Stand: 21.09.2026". Mixed or catalog-less
//! rows (Altpapier, Blech/Guss-Mischzeilen, Al-Cu Kühler, Bleiakkus,
//! Smartphones, 0,00-€-Nullzeile) skip loudly.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sn-zschopau-wertstoff-bauer-inh-micha-bauer";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://wertstoff-bauer.de/startseite/impressum.html";

pub const URL: &str = "https://wertstoff-bauer.de/startseite/preise.html";

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

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Order matters throughout: "Batterieblei / Bleiakkus" is not
/// the catalog's Weichblei (batteries skip, like antikart/gutzmann),
/// "Wuchtblei / Kabelblei" holds "kabel" but is lead, and the
/// "Shredderkabel" rows hold "shredder" but are cables, not steel.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("batterie") || l.contains("akku") {
        None
    } else if l.contains("kühler") || l.contains("kuehler") {
        // Al-Cu-Mischprodukt: kein Katalogmaterial.
        None
    } else if l.contains("smartphone") || l.contains("handy") {
        // Ganze Geräte: keine Platinen, kein Stückpreis-Katalog.
        None
    } else if l.contains("altpapier") {
        None
    } else if l.contains("zinkblech") || (l.contains("zink") && !l.contains("blech /")) {
        Some(("zink", ""))
    } else if l.contains("blech /") || l.contains("blech/") {
        // "Al Blech / Guß …" mischt zwei Katalogmaterialien.
        None
    } else if l.contains("altblei") {
        Some(("blei", "Alt"))
    } else if l.contains("wuchtblei") || l.contains("kabelblei") {
        Some(("blei", "Wucht-/Kabelblei"))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("kabel") {
        if l.contains("cu") || l.contains("kupfer") {
            if l.contains("shredder") {
                Some(("kabel-kupfer", "Shredderkabel"))
            } else if l.contains("60%") {
                Some(("kabel-kupfer", "ab 60%"))
            } else {
                Some(("kabel-kupfer", ""))
            }
        } else if l.contains("al") {
            if l.contains("shredder") {
                Some(("kabel-alu", "Shredderkabel"))
            } else if l.contains("50%") {
                Some(("kabel-alu", "ab 50%"))
            } else {
                Some(("kabel-alu", ""))
            }
        } else {
            None
        }
    } else if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("lackdraht") || l.contains("berry") {
        Some(("kupfer-berry", "Lackdraht"))
    } else if l.contains("cu-raff") {
        Some(("kupfer-gemischt", "Raff"))
    } else if l.contains("cu-schwer") {
        Some(("kupfer-gemischt", "Schwer"))
    } else if l.contains("ms-raff") {
        Some(("messing", "Raff"))
    } else if l.contains("ms-schwer") {
        Some(("messing", "Schwer"))
    } else if l.contains("rotgu") {
        Some(("bronze-rotguss", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", ""))
    } else if l.contains("getriebemotor") {
        Some(("elektromotoren", "mit Getriebe"))
    } else if l.contains("motor") {
        Some(("elektromotoren", ""))
    } else if l.contains("zinn") {
        Some(("zinn", "Geschirr 88-95%"))
    } else if l.contains("hartmetall") {
        Some(("hartmetall", ""))
    } else if l.contains("felgen") {
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("draht") {
        Some(("aluminium-gemischt", "Draht blank"))
    } else if l.contains("leitschienen") {
        Some(("aluminium-gemischt", "Leitschienen"))
    } else if l.contains("mischschrott leicht") {
        Some(("mischschrott", "leicht"))
    } else if l.contains("mischschrott schwer") {
        Some(("mischschrott", "schwer"))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("shredder") {
        Some(("stahlschrott-shredder", ""))
    } else if l.contains("guss") || l.contains("guß") {
        Some(("eisenschrott-gussbruch", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: centered `<p>`
/// lines ("WERTSTOFF-BAUER", "Inh. Micha Bauer", "Gerbergasse 13",
/// "09405 Zschopau", "Telefon: …", "E-Mail: …"). Anchored on
/// "Inh. Micha Bauer" — without it the page changed shape → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p = Selector::parse("p").expect("valid selector");
    let paras: Vec<String> = doc
        .select(&p)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.replace('\u{a0}', " "))
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect();
    if !paras.iter().any(|t| t.contains("Inh. Micha Bauer")) {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Inhaber-Block fehlt".to_owned(),
        });
    }
    // This trader's street, verified live (esh-style: exact match).
    let street = if paras.iter().any(|t| t == "Gerbergasse 13") {
        "Gerbergasse 13".to_owned()
    } else {
        String::new()
    };
    let (mut postcode, mut city) = (String::new(), String::new());
    for t in &paras {
        let toks: Vec<&str> = t.split_whitespace().collect();
        if toks.len() >= 2
            && toks[0].len() == 5
            && toks[0].chars().all(|c| c.is_ascii_digit())
            && toks[1].chars().next().is_some_and(|c| c.is_uppercase())
        {
            postcode = toks[0].to_owned();
            city = toks[1..].join(" ");
            break;
        }
    }
    let phone = paras
        .iter()
        .find(|t| t.starts_with("Telefon:"))
        .map(|t| {
            t["Telefon:".len()..]
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();
    // E-Mail needs its own rule (phone-style take_while would stop at
    // the first letter).
    let email = paras
        .iter()
        .find(|t| t.starts_with("E-Mail:"))
        .map(|t| {
            t["E-Mail:".len()..]
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_owned()
        })
        .unwrap_or_default();
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
    // Window, never the whole page: last "Preisübersicht" is the 48px
    // content heading (nav links precede it); the list ends at the
    // "auf Anfrage" terminator, before footer contact + disclaimer.
    let start = html
        .rfind("Preisübersicht")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Weitere nicht gelistete Metalle auf Anfrage.")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preislisten-Ende fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    let mut published_at = None;
    if let Some(pos) = tail.find("Stand:") {
        // Raw HTML: stop at the next tag ("21.09.2026</span>").
        let tok: String = tail[pos + 6..]
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == ' ')
            .collect();
        let rest: Vec<&str> = tok.trim().split('.').collect();
        if rest.len() == 3 {
            published_at = parse_de_date(rest[0], rest[1], rest[2]);
        }
    }
    let doc = Html::parse_fragment(window);
    let table = Selector::parse("table").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for t in doc.select(&table) {
        let cells: Vec<String> = t
            .select(&cell)
            .map(|c| c.text().collect::<String>())
            .map(|x| x.replace('\u{a0}', " "))
            .map(|x| x.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        // Strict pairs: label cell + price cell per row.
        for pair in cells.chunks(2) {
            if pair.len() < 2 {
                continue;
            }
            let label = pair[0]
                .trim()
                .trim_start_matches(['↳', ' '])
                .trim()
                .to_owned();
            let price_text = pair[1].trim().to_owned();
            if label.is_empty() {
                continue;
            }
            if label.len() > 120 {
                continue;
            }
            // Header tables ("Buntmetall" | "Preis/Kg"): price cell
            // without digits is a header, not a label.
            let Some(price) = parse_eur(&price_text) else {
                if !price_text.chars().any(|c| c.is_ascii_digit()) {
                    continue;
                }
                skips.push(format!("{label} (Preis unverständlich: {price_text})"));
                continue;
            };
            if price == 0.0 {
                // Live: "Al-Shredderkabel 0,00 €" — no purchase.
                skips.push(format!("{label} (kein Ankauf: {price_text})"));
                continue;
            }
            // An unparseable unit is a loud skip, never a silent
            // default: a per-tonne price recorded as per-kg would be a
            // 1000x error.
            let Some(unit) = unit_of(&price_text) else {
                skips.push(format!("{label} (Einheit unverständlich: {price_text})"));
                continue;
            };
            rows.push((label, price, unit));
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste leer".to_owned(),
        });
    }
    // Same label twice (e.g. a repeated block) must not double-count.
    rows.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.partial_cmp(&b.1).unwrap()));
    rows.dedup();
    Ok((published_at, rows, skips))
}

/// Bespoke unit matcher for THIS page's price cells (live: "8,80 €" =
/// kg per the "Preis/Kg" headers, "50,00 € / Tonne" on Schrott rows).
/// Only kg/t exist here — anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("tonne") {
        Some("EUR/t")
    } else if lower.contains('€') {
        Some("EUR/kg")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    // Real markup shape: table.Table_texttable__2GrpI, ↳ labels,
    // &nbsp;-padded prices, terminator + Stand paragraphs.
    const FIXTURE: &str = "<p><span style=\"font-size: 48px;\">\
        <span style=\"font-weight: bold;\">Preisübersicht</span></span></p>\
        <table class=\"Table_texttable__2GrpI\"><tbody>\
        <tr><td><div><p><span>↳ Cu-Millberry (nicht oxidiert)</span><br></p></div></td>\
        <td><div><p><span>&nbsp; 9,60 €</span><br></p></div></td></tr>\
        <tr><td><div><p><span>↳ Al Blech / Guß sauber</span><br></p></div></td>\
        <td><div><p><span>&nbsp; 0,90 €</span><br></p></div></td></tr>\
        </tbody></table>\
        <table class=\"Table_texttable__2GrpI\"><tbody>\
        <tr><td><span>Buntmetall</span></td><td><span>Preis/Kg</span></td></tr>\
        </tbody></table>\
        <table class=\"Table_texttable__2GrpI\"><tbody>\
        <tr><td><span>↳ Al-Shredderkabel</span></td><td><span>&nbsp; 0,00 €</span></td></tr>\
        <tr><td><span>↳ Mischschrott schwer</span></td>\
        <td><span>&nbsp;120,00 € / Tonne</span></td></tr>\
        </tbody></table>\
        <p><span>Weitere nicht gelistete Metalle auf Anfrage.</span></p>\
        <p><span>Stand: 21.09.2026</span></p>";

    #[test]
    fn tables_date_and_zero_row_parse() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-21T00:00:00+00:00"));
        assert_eq!(rows.len(), 3);
        assert!(rows
            .iter()
            .any(|(l, p, u)| l == "Cu-Millberry (nicht oxidiert)" && *p == 9.6 && *u == "EUR/kg"));
        assert!(rows
            .iter()
            .any(|(l, p, u)| l == "Mischschrott schwer" && *p == 120.0 && *u == "EUR/t"));
        // Header pair skipped silently; zero-price row skipped loudly.
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Al-Shredderkabel"));
    }

    #[test]
    fn missing_anchors_and_empty_list_error() {
        let no_start = FIXTURE.replace("Preisübersicht", "Xyz");
        assert!(parse(&no_start).is_err());
        let no_end = FIXTURE.replace("Weitere nicht gelistete Metalle auf Anfrage.", "Xyz");
        assert!(parse(&no_end).is_err());
        // Longest first: "0,00 €" is a substring of "120,00 € / Tonne".
        let empty = FIXTURE
            .replace("120,00 € / Tonne", "pro Sack")
            .replace("9,60 €", "pro Sack")
            .replace("0,90 €", "pro Sack")
            .replace("0,00 €", "pro Sack");
        let err = parse(&empty).expect_err("empty list errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn mapping_covers_every_live_row() {
        assert_eq!(
            grade_for("Cu-raff (max. 5% Anhaftung)"),
            Some(("kupfer-gemischt", "Raff"))
        );
        assert_eq!(
            grade_for("Cu-schwer (ohne Anhaftung)"),
            Some(("kupfer-gemischt", "Schwer"))
        );
        assert_eq!(
            grade_for("Cu-Lackdraht / Berry"),
            Some(("kupfer-berry", "Lackdraht"))
        );
        assert_eq!(
            grade_for("Cu-Millberry (nicht oxidiert)"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Ms-raff (max. 5% Anhaftung)"),
            Some(("messing", "Raff"))
        );
        assert_eq!(grade_for("Ms-schwer"), Some(("messing", "Schwer")));
        assert_eq!(grade_for("Rotguß"), Some(("bronze-rotguss", "")));
        assert_eq!(
            grade_for("Al Draht blank"),
            Some(("aluminium-gemischt", "Draht blank"))
        );
        assert_eq!(
            grade_for("Al Felgen sauber"),
            Some(("aluminium-guss", "Felgen"))
        );
        assert_eq!(
            grade_for("Al Leitschienen"),
            Some(("aluminium-gemischt", "Leitschienen"))
        );
        assert_eq!(grade_for("Zinkblech"), Some(("zink", "")));
        assert_eq!(grade_for("Altblei"), Some(("blei", "Alt")));
        assert_eq!(
            grade_for("Wuchtblei / Kabelblei"),
            Some(("blei", "Wucht-/Kabelblei"))
        );
        assert_eq!(grade_for("V2A"), Some(("edelstahl-v2a", "")));
        assert_eq!(grade_for("V4A"), Some(("edelstahl-v4a", "")));
        assert_eq!(grade_for("E-Motoren"), Some(("elektromotoren", "")));
        assert_eq!(
            grade_for("Getriebemotoren"),
            Some(("elektromotoren", "mit Getriebe"))
        );
        assert_eq!(
            grade_for("Zinngeschirr 88 - 95 %"),
            Some(("zinn", "Geschirr 88-95%"))
        );
        assert_eq!(grade_for("Hartmetall"), Some(("hartmetall", "")));
        assert_eq!(
            grade_for("Cu-Shredderkabel"),
            Some(("kabel-kupfer", "Shredderkabel"))
        );
        assert_eq!(
            grade_for("Cu-Kabel ab 60%"),
            Some(("kabel-kupfer", "ab 60%"))
        );
        assert_eq!(grade_for("Al-Kabel ab 50%"), Some(("kabel-alu", "ab 50%")));
        assert_eq!(
            grade_for("Al-Shredderkabel"),
            Some(("kabel-alu", "Shredderkabel"))
        );
        assert_eq!(grade_for("Shredder"), Some(("stahlschrott-shredder", "")));
        assert_eq!(
            grade_for("Mischschrott leicht"),
            Some(("mischschrott", "leicht"))
        );
        assert_eq!(
            grade_for("Mischschrott schwer"),
            Some(("mischschrott", "schwer"))
        );
        assert_eq!(
            grade_for("Gußschrott"),
            Some(("eisenschrott-gussbruch", ""))
        );
        // Loud skips: mixed, catalog-less, batteries, whole devices.
        assert_eq!(grade_for("Al Blech / Guß sauber"), None);
        assert_eq!(grade_for("Al Blech / Guß > 20% / Al Getriebe"), None);
        assert_eq!(grade_for("Al-Cu Kühler"), None);
        assert_eq!(grade_for("Batterieblei / Bleiakkus"), None);
        assert_eq!(grade_for("Smartphones / Handys (ohne Akkus)"), None);
        assert_eq!(grade_for("Altpapier"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<p><span style=\"font-weight: bold;\">WERTSTOFF-BAUER</span></p>\
            <p><span>Inh. Micha Bauer </span></p>\
            <p><span>Gerbergasse 13 </span></p>\
            <p><span>09405 Zschopau</span></p>\
            <p><span>Telefon: 0173 - 38 13 559 </span></p>\
            <p><span>E-Mail: info@wertstoff-bauer.de</span></p>\
            <p><span>Inhaber: Micha Bauer</span></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Gerbergasse 13");
        assert_eq!(info.postcode, "09405");
        assert_eq!(info.city, "Zschopau");
        assert_eq!(info.phone, "0173 - 38 13 559");
        assert_eq!(info.email, "info@wertstoff-bauer.de");
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
