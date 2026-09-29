//! Schrott Buntmetallankauf / Buntmetallhandel Altmittweida: exact
//! prices in a clean `<table class="jw-table">` headed "Produkt" |
//! "Preis" (live rows: "Kupfer Milbery" [sic] 10,50 €, "V4" [sic for
//! V4A] 1,50 €, "Mischschrott" 0.10 € with a dot decimal). Bare-€
//! cells are EUR/kg — documented page default, market-plausibilized
//! (Cu 8–10, steel 0.10–0.18 = 100–180 €/t). No page date, no caveats:
//! exact at 1.0, `published_at` None. Bleiakku and Altpapier have no
//! catalog material and skip loudly. The site has no impressum page;
//! IMPRESSUM_URL is its own live /kontakt link (phone + Standort).

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sn-altmittweida-schrott-buntmetallankauf-buntmetallhande";
/// Bespoke, live-verified contact URL (this Webador site exposes no
/// impressum page — /kontakt is its own nav link). A move fails the
/// step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://buntmetallhandel-altmittweida.de/kontakt";

pub const URL: &str = "https://buntmetallhandel-altmittweida.de/ankaufspreisliste";

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
    // Kontakt-page failure fails the whole step on purpose: a moved
    // contact page means the site changed and needs eyeballs before we
    // trust anything from it again.
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

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific before generic: "Kupfer Kabel" holds "kupfer",
/// "Blei Akku" holds "blei" (batteries skip — antikart/gutzmann
/// precedent), "Alu Blech"/"Alugus"/"Alu Profil" share the "alu" word.
/// Live typos handled and documented: "Milbery" (Millberry), "V4"
/// (V4A), "Elektomotore" (missing the r in "elektro" as well as the
/// trailing n — matched on both spellings).
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("milbery") || l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("berry") {
        Some(("kupfer-berry", ""))
    } else if l.contains("verzinnt") {
        // Tin-coated copper: coated, like Berry (beschichtet).
        Some(("kupfer-berry", "verzinnt"))
    } else if l.contains("kabel") {
        Some(("kabel-kupfer", ""))
    } else if l.contains("kupfer") {
        if l.contains("raff") {
            Some(("kupfer-gemischt", "Raff"))
        } else if l.contains("schwer") {
            Some(("kupfer-gemischt", "Schwer"))
        } else {
            Some(("kupfer-gemischt", ""))
        }
    } else if l.contains("messing") {
        if l.contains("raff") {
            Some(("messing", "Raff"))
        } else if l.contains("schwer") {
            Some(("messing", "Schwer"))
        } else {
            Some(("messing", ""))
        }
    } else if l.contains("akku") || l.contains("batterie") {
        None
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("elektromotor") || l.contains("elektomotore") {
        if l.contains("getriebe") {
            Some(("elektromotoren", "mit Getriebe"))
        } else {
            Some(("elektromotoren", ""))
        }
    } else if l.contains("v4a") || l.trim() == "v4" {
        Some(("edelstahl-v4a", ""))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("alufelgen") {
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("prof") {
        // "Alu Profil blank/lackiert", live "Alu Iso-profi …": profile
        // shapes; the Iso line's 10 % attachment rides in the variant.
        if l.contains("iso") {
            Some(("aluminium-profile", "Iso max. 10% Anhaftung"))
        } else if l.contains("lackiert") {
            Some(("aluminium-profile", "lackiert"))
        } else {
            Some(("aluminium-profile", "blank"))
        }
    } else if l.contains("alu blech") {
        if l.contains("ohne") {
            Some(("aluminium-blech", "ohne Anhaftung"))
        } else {
            Some(("aluminium-blech", "mit Anhaftung"))
        }
    } else if l.contains("alugus") {
        if l.contains("sauber") {
            Some(("aluminium-guss", "sauber"))
        } else {
            Some(("aluminium-guss", "mit Anhaftung"))
        }
    } else if l.contains("bremsscheiben") {
        Some(("eisenschrott-gussbruch", "Bremsscheiben"))
    } else if l.contains("guss") {
        Some(("eisenschrott-gussbruch", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("altpapier") {
        None
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS /kontakt page only: the
/// `<h2>Kontaktiere uns</h2>` block holds "&nbsp;Tel. 017641192544"
/// (anchor — missing → loud error) and the `<h2>Standort</h2>` block
/// holds "Schrott Buntmetallankauf<br>Altmittweida, Deutschland".
/// Street, postcode and e-mail are not listed → empty (documented).
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let anchor = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Kontaktiere uns");
    if anchor.is_none() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let paras: Vec<String> = doc
        .select(&p)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.replace('\u{a0}', " "))
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect();
    let mut phone = String::new();
    for t in &paras {
        if let Some(pos) = t.find("Tel.") {
            let digits: String = t[pos + 4..]
                .split_whitespace()
                .take_while(|w| {
                    w.chars()
                        .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ");
            if !digits.is_empty() {
                phone = digits;
                break;
            }
        }
    }
    // "Altmittweida, Deutschland" under the Standort heading.
    let mut city = String::new();
    for t in &paras {
        if t.contains("Altmittweida") {
            city = "Altmittweida".to_owned();
            break;
        }
    }
    if phone.is_empty() && city.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo {
        street: String::new(),
        postcode: String::new(),
        city,
        phone,
        email: String::new(),
    })
}

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tbody tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
    // Never trust page order: take the table carrying the Produkt
    // header, not just the first <table> on the page.
    let table = doc.select(&table).find(|t| {
        t.select(&head)
            .any(|h| h.text().collect::<String>().trim() == "Produkt")
    });
    let Some(table) = table else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    };
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in table.select(&row) {
        let cells: Vec<String> = tr.select(&cell).map(|c| c.text().collect()).collect();
        if cells.len() < 2 {
            continue;
        }
        let label = cells[0]
            .replace('\u{a0}', " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_owned();
        if label.is_empty() || label.len() > 120 {
            continue;
        }
        let price_text = cells[1].trim().to_owned();
        let Some(price) = parse_eur(&price_text) else {
            if price_text.chars().any(|c| c.is_ascii_digit()) {
                skips.push(format!("{label} (Preis unverständlich: {price_text})"));
            }
            continue;
        };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let Some(unit) = unit_of(&price_text) else {
            skips.push(format!("{label} (Einheit unverständlich: {price_text})"));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    // Dedupe repeated blocks after mapping-relevant fields.
    rows.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.partial_cmp(&b.1).unwrap()));
    rows.dedup();
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS list: every live price is a bare-€
/// cell ("8,60€", "0.10€") and kg magnitudes check out against the
/// market (Cu 8–10, steel 0.10–0.18 = 100–180 €/t), so bare € is the
/// documented EUR/kg default. Explicitly foreign units ("/", "pro",
/// "Stück") skip loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("tonne") || lower.contains("/t") {
        Some("EUR/t")
    } else if lower.contains("pro ") || lower.contains("stück") || lower.contains("stk") {
        None
    } else if lower.contains('€') || lower.contains("eur") {
        Some("EUR/kg")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    // Real markup shape: table.jw-table with thead Produkt/Preis,
    // live typos ("Milbery", "V4") and the dot-decimal "0.10€".
    const FIXTURE: &str = "<table><tr><td>Nav</td></tr></table>\
        <table width=\"100%\" class=\"jw-table jw-table--header jw-table--style-striped\">\
        <thead><tr><th width=\"50%\">Produkt</th><th width=\"50%\">Preis</th></tr></thead>\
        <tbody>\
        <tr><td width=\"50%\">Kupfer Milbery</td><td width=\"50%\">10,50€</td></tr>\
        <tr><td>V4</td><td>1,50€</td></tr>\
        <tr><td>Mischschrott</td><td>0.10€</td></tr>\
        <tr><td>Blei Akku</td><td>0,20€</td></tr>\
        <tr><td></td><td></td></tr>\
        </tbody></table>";

    #[test]
    fn table_parses_with_typos_and_dot_decimal() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // Decoy table ignored via Produkt header; empty row skipped.
        assert_eq!(rows.len(), 4);
        assert!(skips.is_empty());
        let mil = rows
            .iter()
            .find(|(l, _, _)| l == "Kupfer Milbery")
            .expect("milbery");
        assert_eq!(mil.1, 10.5);
        assert_eq!(mil.2, "EUR/kg");
        let ms = rows
            .iter()
            .find(|(l, _, _)| l == "Mischschrott")
            .expect("ms");
        assert_eq!(ms.1, 0.1);
    }

    #[test]
    fn missing_table_and_empty_table_error() {
        let err = parse("<table><tr><td>Nav</td></tr></table>").expect_err("no table");
        assert!(err.to_string().contains("keine Preistabelle"));
        let empty = FIXTURE
            .replace("10,50€", "pro Sack")
            .replace("1,50€", "pro Sack")
            .replace("0.10€", "pro Sack")
            .replace("0,20€", "pro Sack");
        let err = parse(&empty).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn mapping_covers_every_live_row() {
        assert_eq!(
            grade_for("Kupfer Raff , 94,% Cu."),
            Some(("kupfer-gemischt", "Raff"))
        );
        assert_eq!(
            grade_for("Kupfer Schwer"),
            Some(("kupfer-gemischt", "Schwer"))
        );
        assert_eq!(
            grade_for("Kupfer verzinnt"),
            Some(("kupfer-berry", "verzinnt"))
        );
        assert_eq!(grade_for("Kupfer Berry"), Some(("kupfer-berry", "")));
        assert_eq!(grade_for("Kupfer Milbery"), Some(("kupfer-millberry", "")));
        assert_eq!(grade_for("Kupfer Kabel"), Some(("kabel-kupfer", "")));
        assert_eq!(
            grade_for("Messing Raff max 5%Anh."),
            Some(("messing", "Raff"))
        );
        assert_eq!(
            grade_for("Messing Schwer max1%Anh."),
            Some(("messing", "Schwer"))
        );
        assert_eq!(grade_for("Elektomotore"), Some(("elektromotoren", "")));
        assert_eq!(
            grade_for("Elektomotore mit Getriebe"),
            Some(("elektromotoren", "mit Getriebe"))
        );
        assert_eq!(grade_for("V2A"), Some(("edelstahl-v2a", "")));
        assert_eq!(grade_for("V4"), Some(("edelstahl-v4a", "")));
        assert_eq!(grade_for("V4A"), Some(("edelstahl-v4a", "")));
        assert_eq!(grade_for("Zinkblech"), Some(("zink", "")));
        assert_eq!(
            grade_for("Alu Blech ohne Anhaftung"),
            Some(("aluminium-blech", "ohne Anhaftung"))
        );
        assert_eq!(
            grade_for("Alu Blech mit Anhaftung"),
            Some(("aluminium-blech", "mit Anhaftung"))
        );
        assert_eq!(
            grade_for("Alugus Sauber"),
            Some(("aluminium-guss", "sauber"))
        );
        assert_eq!(
            grade_for("Alugus mit Anhaftung"),
            Some(("aluminium-guss", "mit Anhaftung"))
        );
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(
            grade_for("Gussschrott"),
            Some(("eisenschrott-gussbruch", ""))
        );
        assert_eq!(
            grade_for("Bremsscheiben"),
            Some(("eisenschrott-gussbruch", "Bremsscheiben"))
        );
        assert_eq!(
            grade_for("Alu Profil blank"),
            Some(("aluminium-profile", "blank"))
        );
        assert_eq!(
            grade_for("Alu Profil lackiert"),
            Some(("aluminium-profile", "lackiert"))
        );
        assert_eq!(
            grade_for("Alu Iso-profi max. 10%Anh."),
            Some(("aluminium-profile", "Iso max. 10% Anhaftung"))
        );
        assert_eq!(
            grade_for("Alufelgen ohne Anh."),
            Some(("aluminium-guss", "Felgen"))
        );
        assert_eq!(grade_for("Blei Akku"), None);
        assert_eq!(grade_for("Altpapier"), None);
    }

    #[test]
    fn kontakt_extracts_phone_and_city() {
        let imp = "<h2 class=\"jw-heading-100\">Kontaktiere uns</h2>\
            <p>Kontaktieren Sie uns f&uuml;r unseren Service.</p>\
            <p>&nbsp;Tel. 017641192544</p>\
            <h2 class=\"jw-heading-85\">Standort</h2>\
            <p>Schrott Buntmetallankauf<br />Altmittweida, Deutschland</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.phone, "017641192544");
        assert_eq!(info.city, "Altmittweida");
        assert!(info.street.is_empty());
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
