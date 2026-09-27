//! MKR Rothenbücher GmbH (Köln-Niehl): exact "Preise für Privatkunden" in a
//! Beaver-Builder table (Material / Rohstoff × Preis 0-49 kg × Preis ab
//! 50 kg), terminated by the "freibleibend" footnote. Live: 12 material
//! rows, 22 filled price cells, 2 empty ab-50-kg cells (Mischschrott,
//! Kufperkabel) → loud tier skips. No page date (the only 2026 timestamp
//! sits inside a plugin `<script>`, which is never read) —
//! `published_at` is None.
//!
//! Quantity tiers become `variant` values ("0-49 kg" / "ab 50 kg"),
//! combined with the grade detail where one material has several grades
//! (three kabel-kupfer grades must never collapse). Units are quoted per
//! kg with a DECIMAL POINT ("11.30 € / kg" = 11.3, not 1130 — pinned by
//! tests); record() normalizes into catalog units (kg↔t) downstream.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-koln-niehl-mkr-rothenbucher";
/// Bespoke, live-verified impressum URL (the site's own footer link).
/// A move fails the step loudly — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.kabel-recycling.de/kontakt/impressum/";

pub const URL: &str = "https://www.kabel-recycling.de/preise/";

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
    for (label, tier, price, unit) in rows {
        // Tiers are page-driven strings; only the two live headers may pass.
        // A renamed column skips loudly instead of minting variants.
        let tier: &'static str = match tier.as_str() {
            "0-49 kg" => "0-49 kg",
            "ab 50 kg" => "ab 50 kg",
            _ => {
                skipped_labels.push(format!("{label} (Preisstaffel unverständlich: {tier})"));
                continue;
            }
        };
        match grade_for(&label, tier) {
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
        published_at: None,
    })
}

/// Explicit (label, tier) → (material, variant) mapping. Anything unlisted
/// is skipped. Arms are specific-before-generic ("schälkabel" before
/// "kabel"); the variant always carries the tier plus the grade detail so
/// same-material grades at different prices never collapse.
fn grade_for(label: &str, tier: &'static str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Grade detail first; the tier joins it in the variant table below.
    let (material, detail): (&'static str, &'static str) = if l.contains("geschirr") {
        ("aluminium-blech", "Geschirr 10%")
    } else if l.contains("v2a") {
        ("edelstahl-v2a", "")
    } else if l.contains("v4a") {
        ("edelstahl-v4a", "")
    } else if l.contains("zink") {
        ("zink", "")
    } else if l.contains("mischschrott") {
        ("mischschrott", "")
    } else if l.contains("millberry") {
        ("kupfer-millberry", "")
    } else if l.contains("schälkabel") {
        ("kabel-kupfer", "Schälkabel 50%")
    } else if l.contains("litzen") {
        ("kabel-kupfer", "Litzenkabel 50%")
    } else if l.contains("kabel") && l.contains("38") {
        // Live typo "Kufperkabel" included: "kabel"+"38" is the anchor.
        ("kabel-kupfer", "38% ohne Stecker")
    } else if l.contains("schwer") {
        ("kupfer-gemischt", "schwer")
    } else if l.contains("messing") {
        ("messing", "")
    } else if l.contains("profil") {
        // "Alu Iso Profile": explicit product form wins over the "Iso"
        // prefix (kept as detail); flagged as uncertain in review.
        ("aluminium-profile", "Iso")
    } else {
        return None;
    };
    let variant: &'static str = match (detail, tier) {
        ("", "0-49 kg") => "0-49 kg",
        ("", "ab 50 kg") => "ab 50 kg",
        ("Geschirr 10%", "0-49 kg") => "Geschirr 10%, 0-49 kg",
        ("Geschirr 10%", "ab 50 kg") => "Geschirr 10%, ab 50 kg",
        ("schwer", "0-49 kg") => "schwer, 0-49 kg",
        ("schwer", "ab 50 kg") => "schwer, ab 50 kg",
        ("38% ohne Stecker", "0-49 kg") => "38% ohne Stecker, 0-49 kg",
        ("38% ohne Stecker", "ab 50 kg") => "38% ohne Stecker, ab 50 kg",
        ("Schälkabel 50%", "0-49 kg") => "Schälkabel 50%, 0-49 kg",
        ("Schälkabel 50%", "ab 50 kg") => "Schälkabel 50%, ab 50 kg",
        ("Litzenkabel 50%", "0-49 kg") => "Litzenkabel 50%, 0-49 kg",
        ("Litzenkabel 50%", "ab 50 kg") => "Litzenkabel 50%, ab 50 kg",
        ("Iso", "0-49 kg") => "Iso, 0-49 kg",
        ("Iso", "ab 50 kg") => "Iso, ab 50 kg",
        _ => return None,
    };
    Some((material, variant))
}

fn parse(
    html: &str,
) -> Result<(Vec<(String, String, f64, &'static str)>, Vec<String>), IngestError> {
    // Window: the private-customer table between its heading and the
    // "freibleibend" footnote. The hero banner above quotes its own
    // "3,80€ / kg Kupferkabel 38%" presentation — a different number that
    // must stay out of the table parse.
    let start = html
        .find("Preise für Privatkunden")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("freibleibend").unwrap_or(tail.len());
    let window = &tail[..end];
    let doc = Html::parse_fragment(window);
    let table = Selector::parse("table").expect("valid selector");
    let th = Selector::parse("th").expect("valid selector");
    let tr = Selector::parse("tbody tr").expect("valid selector");
    let td = Selector::parse("td").expect("valid selector");
    // Never trust page order: take the table carrying the material header,
    // not just the first <table> on the page.
    let table = doc.select(&table).find(|t| {
        t.select(&th)
            .any(|h| h.text().collect::<String>().contains("Material / Rohstoff"))
    });
    let Some(table) = table else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle fehlt".to_owned(),
        });
    };
    // Tier names come from the live headers ("Preis 0-49 kg" → "0-49 kg"):
    // a renamed column breaks loudly instead of minting mystery variants.
    let headers: Vec<String> = table
        .select(&th)
        .map(|h| h.text().collect::<String>())
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect();
    if headers.len() != 3 {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisspalten geändert".to_owned(),
        });
    }
    let tiers: Vec<String> = headers[1..]
        .iter()
        .map(|h| h.strip_prefix("Preis ").unwrap_or(h).to_owned())
        .collect();
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for row in table.select(&tr) {
        if row.select(&th).next().is_some() {
            continue; // header row, not data
        }
        let cells: Vec<String> = row
            .select(&td)
            .map(|c| c.text().collect::<String>())
            .map(|t| t.replace(['\u{a0}'], " "))
            .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        if cells.len() < tiers.len() + 1 {
            skips.push(format!(
                "Tabellenzeile unverständlich: {}",
                cells.join(" / ")
            ));
            continue;
        }
        let label = cells[0].clone();
        for (tier, cell) in tiers.iter().zip(cells[1..].iter()) {
            if cell.is_empty() {
                skips.push(format!("{label} ({tier}: kein Preis)"));
                continue;
            }
            let Some(price) = parse_eur(cell) else {
                skips.push(format!("{label} ({tier}: Preis unverständlich: {cell})"));
                continue;
            };
            // An unparseable unit is a loud skip, never a silent default: a
            // per-tonne price recorded as per-kg would be a 1000x error.
            let Some(unit) = unit_of(cell) else {
                skips.push(format!("{label} (Einheit unverständlich: {cell})"));
                continue;
            };
            rows.push((label.clone(), tier.clone(), price, unit));
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS table's price cells (live: "1.35 € / kg",
/// "1.55€ / kg" — always per kg). Only kg is evidenced — anything else
/// skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    if cell.to_lowercase().contains("kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the address `<p>`
/// naming the firm ("MKR Rothenbücher GmbH / Geestemünder Straße 34 /
/// D-50735 Köln" — footer lookalikes lack the firm line in the same `<p>`)
/// plus the `<p>` after the "Kontakt" heading ("Telefon:" / "E-Mail:"
/// lines). Anchored on the `h1` "Impressum" heading and the "Kontakt"
/// heading — missing anchors → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let h2 = Selector::parse("h2").expect("valid selector");
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
    for el in doc.select(&p) {
        let lines = block_lines(&el.inner_html());
        if !lines.iter().any(|l| l.contains("MKR Rothenbücher GmbH")) {
            continue;
        }
        for (k, line) in lines.iter().enumerate() {
            let mut it = line.split_whitespace();
            // "D-50735 Köln": optional D- prefix, then PLZ + city.
            if let (Some(pc_raw), Some(ci)) = (it.next(), it.next()) {
                let pc = pc_raw.trim_start_matches("D-");
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
        if !postcode.is_empty() {
            break;
        }
    }
    let mut phone = String::new();
    let mut email = String::new();
    let mut found_kontakt = false;
    for el in doc.select(&h2) {
        if el.text().collect::<String>().trim() != "Kontakt" {
            continue;
        }
        found_kontakt = true;
        let sib = el
            .next_siblings()
            .filter_map(ElementRef::wrap)
            .find(|e| e.value().name() == "p");
        if let Some(p) = sib {
            for line in block_lines(&p.inner_html()) {
                if let Some(v) = line.strip_prefix("Telefon:") {
                    phone = v
                        .split_whitespace()
                        .take_while(|t| {
                            t.chars()
                                .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                        })
                        .collect::<Vec<_>>()
                        .join(" ");
                } else if let Some(v) = line.strip_prefix("E-Mail:") {
                    // Email needs its own rule: the phone-style take_while
                    // above would stop at the first letter.
                    email = v.split_whitespace().next().unwrap_or_default().to_owned();
                }
            }
        }
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

/// Split an inner-HTML block on `<br` into decoded text lines. Newlines are
/// planted BEFORE stripping tags so "Telefon:" labels survive (a
/// drop-to-first-'>' strip would eat them with the `<a ...>` opener), and
/// this page's live entities (`&uuml;`, `&szlig;`, `&shy;`, …) are decoded
/// per line.
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
            decode_entities(&out)
        })
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| !s.is_empty())
        .collect()
}

/// The entities this page actually emits (verified live in the impressum
/// block). Anything else stays verbatim — never a silent wrong character.
fn decode_entities(s: &str) -> String {
    s.replace("&uuml;", "ü")
        .replace("&Uuml;", "Ü")
        .replace("&ouml;", "ö")
        .replace("&Ouml;", "Ö")
        .replace("&auml;", "ä")
        .replace("&Auml;", "Ä")
        .replace("&szlig;", "ß")
        .replace("&shy;", "")
        .replace("&nbsp;", " ")
        .replace("&#160;", " ")
        .replace("&amp;", "&")
        .replace(['\u{ad}'], "")
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    // Verbatim structure of the live table (2026-09-27): real Beaver
    // Builder classes, real headers, real labels incl. the "Kufperkabel"
    // typo and the two empty ab-50-kg cells, real decimal-point prices.
    const FIXTURE: &str = "<h2>Preise für Privatkunden</h2>\
        <table class=\"uabb-table-inner-wrap\"><thead class=\"uabb-table-header\">\
        <tr class=\"table-header-tr\">\
        <th class=\"table-heading-0 table-header-th\"><span class=\"head-inner-text\">\
        <span> Material / Rohstoff </span></span></th>\
        <th class=\"table-heading-1 table-header-th\"><label class=\"head-style-1 th-style\">\
        <label class=\"head-inner-text\"> Preis 0-49 kg </label></label></th>\
        <th class=\"table-heading-2 table-header-th\"><label class=\"head-style-2 th-style\">\
        <label class=\"head-inner-text\"> Preis ab 50 kg </label></label></th>\
        </thead><tbody class=\"uabb-table-features\">\
        <tr class=\"tbody-row\">\
        <td class=\"table-body-td table-body-0\"><span class=\"content-text\"> Alu Geschirr 10% </span></td>\
        <td class=\"table-body-td table-body-bg-highlight table-body-1\"><span class=\"td-style\">\
        <span class=\"content-text\"> 1.35 &euro; / kg </span></span></td>\
        <td class=\"table-body-td table-body-2\"><span class=\"content-text\"> 1.55&euro; / kg </span></td>\
        <tr class=\"tbody-row\">\
        <td class=\"table-body-td table-body-3\"><span class=\"content-text\"> V2A </span></td>\
        <td class=\"table-body-td table-body-bg-highlight table-body-4\"><span class=\"td-style\">\
        <span class=\"content-text\"> 0.80 &euro; / kg </span></span></td>\
        <td class=\"table-body-td table-body-bg-highlight table-body-5\"><span class=\"td-style\">\
        <span class=\"content-text\"> 1.00 &euro; / kg </span></span></td>\
        <tr class=\"tbody-row\">\
        <td class=\"table-body-td table-body-12\"><span class=\"content-text\"> Mischschrott </span></td>\
        <td class=\"table-body-td table-body-bg-highlight table-body-13\"><span class=\"td-style\">\
        <span class=\"content-text\"> 0.18&euro; / kg </span></span></td>\
        <td class=\"table-body-td table-body-bg-highlight table-body-14\"><span class=\"td-style\">\
        <span class=\"content-text\">  </span></span></td>\
        <tr class=\"tbody-row\">\
        <td class=\"table-body-td table-body-15\"><span class=\"content-text\"> Millberry </span></td>\
        <td class=\"table-body-td table-body-bg-highlight table-body-16\"><span class=\"td-style\">\
        <span class=\"content-text\"> 11.30 &euro; / kg </span></span></td>\
        <td class=\"table-body-td table-body-bg-highlight table-body-17\"><span class=\"td-style\">\
        <span class=\"content-text\"> 11.70&euro; / kg </span></span></td>\
        <tr class=\"tbody-row\">\
        <td class=\"table-body-td table-body-21\"><span class=\"content-text\"> Kufperkabel 38% ohne Stecker </span></td>\
        <td class=\"table-body-td table-body-bg-highlight table-body-22\"><span class=\"td-style\">\
        <span class=\"content-text\"> 4.20 &euro; / kg </span></span></td>\
        <td class=\"table-body-td table-body-bg-highlight table-body-23\"><span class=\"td-style\">\
        <span class=\"content-text\">  </span></span></td>\
        <tr class=\"tbody-row\">\
        <td class=\"table-body-td table-body-24\"><span class=\"content-text\"> Kupfersch&auml;lkabel, 50% </span></td>\
        <td class=\"table-body-td table-body-bg-highlight table-body-25\"><span class=\"td-style\">\
        <span class=\"content-text\"> 5.00 &euro; / kg </span></span></td>\
        <td class=\"table-body-td table-body-bg-highlight table-body-26\"><span class=\"td-style\">\
        <span class=\"content-text\"> 5.15 &euro; / kg </span></span></td>\
        <tr class=\"tbody-row\">\
        <td class=\"table-body-td table-body-33\"><span class=\"content-text\"> Alu Iso Profile </span></td>\
        <td class=\"table-body-td table-body-34\"><span class=\"content-text\"> 1.60 &euro; / kg </span></td>\
        <td class=\"table-body-td table-body-35\"><span class=\"content-text\"> 1.70 &euro; / kg </span></td>\
        </tbody></table>\
        <p>Die angegebenen Preise sind freibleibend, frei geliefert zu unserem Lager in K&ouml;ln.</p>";

    #[test]
    fn table_tiers_and_empty_cells() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // 7 fixture rows: 5 × 2 tiers + 2 × 1 tier (empty ab-50-kg cells).
        assert_eq!(rows.len(), 12);
        assert_eq!(skips.len(), 2);
        assert!(skips
            .iter()
            .any(|s| s.contains("Mischschrott") && s.contains("ab 50 kg")));
        assert!(skips
            .iter()
            .any(|s| s.contains("Kufperkabel") && s.contains("ab 50 kg")));
        // Decimal points are decimals here, not thousands separators.
        let alu = rows
            .iter()
            .find(|(l, t, _, _)| l == "Alu Geschirr 10%" && t == "0-49 kg")
            .expect("alu tier1");
        assert_eq!(alu.2, 1.35);
        let mill = rows
            .iter()
            .find(|(l, t, _, _)| l == "Millberry" && t == "ab 50 kg")
            .expect("mill tier2");
        assert_eq!(mill.2, 11.7);
        assert!(rows.iter().all(|(_, _, _, u)| *u == "EUR/kg"));
    }

    #[test]
    fn wrong_table_and_columns_reject_loudly() {
        // A layout table before the price table must not win.
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 12);
        // No material header at all: loud error.
        let html = FIXTURE.replace("Material / Rohstoff", "Sorte");
        assert!(parse(&html).is_err());
        // A third price column (redesign) breaks loudly.
        let html = FIXTURE.replacen(
            "</label></th>",
            "</label></th><th><span> Preis ab 500 kg </span></th>",
            1,
        );
        let err = parse(&html).expect_err("column change errors");
        assert!(err.to_string().contains("Preisspalten"));
        // Every cell empty: loud error, not silent success.
        let html = FIXTURE
            .replace("content-text\"> ", "content-text\">")
            .replace("1.35", "")
            .replace("1.55", "")
            .replace("0.80", "")
            .replace("1.00", "")
            .replace("0.18", "")
            .replace("11.30", "")
            .replace("11.70", "")
            .replace("4.20", "")
            .replace("5.00", "")
            .replace("5.15", "")
            .replace("1.60", "")
            .replace("1.70", "");
        assert!(parse(&html).is_err());
        assert_eq!(unit_of("1.35 € / kg"), Some("EUR/kg"));
        assert_eq!(unit_of("pro Sack"), None);
    }

    #[test]
    fn impressum_extracts_contact_with_entities() {
        let imp = "<h1 class=\"fl-heading\"><span class=\"fl-heading-text\">Impressum</span></h1>\
            <p>MKR Rothenb&uuml;cher GmbH<br />Geestem&uuml;nder Stra&szlig;e 34<br />D-50735 K&ouml;ln</p>\
            <h2>Kontakt</h2><p>Telefon: 0221/712 81 52<br />Telefax: 0221/712 59 37<br />\
            E-Mail: info@kabel-recycling.de</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Geestemünder Straße 34");
        assert_eq!(info.postcode, "50735");
        assert_eq!(info.city, "Köln");
        assert_eq!(info.phone, "0221/712 81 52");
        assert_eq!(info.email, "info@kabel-recycling.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<h1>Neu</h1><p>x</p>").is_err());
        assert!(extract_info("<h1>Impressum</h1><p>x</p>").is_err());
    }

    #[test]
    fn mapping_covers_every_live_label() {
        // Every live label maps, with tier- and grade-distinct variants so
        // same-material rows never collapse onto one current price.
        assert_eq!(
            grade_for("Alu Geschirr 10%", "0-49 kg"),
            Some(("aluminium-blech", "Geschirr 10%, 0-49 kg"))
        );
        assert_eq!(
            grade_for("V2A", "0-49 kg"),
            Some(("edelstahl-v2a", "0-49 kg"))
        );
        assert_eq!(
            grade_for("V2A", "ab 50 kg"),
            Some(("edelstahl-v2a", "ab 50 kg"))
        );
        assert_eq!(
            grade_for("V4A", "ab 50 kg"),
            Some(("edelstahl-v4a", "ab 50 kg"))
        );
        assert_eq!(grade_for("Zink", "0-49 kg"), Some(("zink", "0-49 kg")));
        assert_eq!(
            grade_for("Mischschrott", "0-49 kg"),
            Some(("mischschrott", "0-49 kg"))
        );
        assert_eq!(
            grade_for("Millberry", "ab 50 kg"),
            Some(("kupfer-millberry", "ab 50 kg"))
        );
        assert_eq!(
            grade_for("Kupferschwer", "0-49 kg"),
            Some(("kupfer-gemischt", "schwer, 0-49 kg"))
        );
        // Live typo, pinned: "Kufperkabel".
        assert_eq!(
            grade_for("Kufperkabel 38% ohne Stecker", "0-49 kg"),
            Some(("kabel-kupfer", "38% ohne Stecker, 0-49 kg"))
        );
        assert_eq!(
            grade_for("Kupferschälkabel, 50%", "0-49 kg"),
            Some(("kabel-kupfer", "Schälkabel 50%, 0-49 kg"))
        );
        assert_eq!(
            grade_for("Kupferschälkabel, 50%", "ab 50 kg"),
            Some(("kabel-kupfer", "Schälkabel 50%, ab 50 kg"))
        );
        assert_eq!(
            grade_for("Kupferlitzenkabel, 50%", "0-49 kg"),
            Some(("kabel-kupfer", "Litzenkabel 50%, 0-49 kg"))
        );
        assert_eq!(
            grade_for("Messing", "ab 50 kg"),
            Some(("messing", "ab 50 kg"))
        );
        assert_eq!(
            grade_for("Alu Iso Profile", "0-49 kg"),
            Some(("aluminium-profile", "Iso, 0-49 kg"))
        );
        assert_eq!(grade_for("Goldbarren", "0-49 kg"), None);
    }
}
