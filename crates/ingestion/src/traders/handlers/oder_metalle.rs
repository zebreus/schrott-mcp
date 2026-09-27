//! Oder-Metalle (Neulewin): exact Tagespreise in a single WordPress
//! table ("Schrott-Art" / "Preis *" — the unit is quoted per row:
//! iron rows in EUR/t, non-ferrous in EUR/kg, see the "* kg =
//! Kilogramm // t = Tonne" footnote; record() normalizes into the
//! catalog unit). Empty spacer rows are layout, not labels. No page
//! date ("Preise werden regelmässig aktualisiert" prose only), so
//! `published_at` stays `None`. Altpapier has no catalog material and
//! skips loudly (proposal: none — paper is out of scope).
//!
//! Impressum quirk (with test): the E-Mail is an image (`email2.png`),
//! so no text address exists — `email` stays empty while street,
//! postcode, city and phone fill normally.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bb-neulewin-oder-metalle-schrotthandel-oderbruch";
/// Bespoke, live-verified impressum URL (the site's own nav link).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "http://oder-metalle.de/impressum/";

pub const URL: &str = "http://oder-metalle.de/schrott-preise/";

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

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific-before-generic: "milberry" wins over bare copper,
/// "Misch" over "Guss" ("Alu Misch ohne Guss" contains both), "Felgen"
/// over bare "Guss".
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("milberry") || l.contains("millberry") {
        return Some(("kupfer-millberry", ""));
    }
    if l.contains("raff") {
        return Some(("kupfer-gemischt", "Raff b95"));
    }
    if l.contains("kabel") {
        return Some(("kabel-kupfer", "40%"));
    }
    if l.contains("kupfer") {
        return None;
    }
    if l.contains("felgen") {
        return Some(("aluminium-guss", "Felgen"));
    }
    if l.contains("misch") {
        if l.contains("alu") {
            return Some(("aluminium-gemischt", "ohne Guss"));
        }
        return Some(("mischschrott", ""));
    }
    if l.contains("langschrott") {
        // Heavy long scrap: generic iron bucket with the trader's grade
        // as variant (closest bucket, flagged as uncertain).
        return Some(("mischschrott", "schwerer Langschrott"));
    }
    if l.contains("sorte 3") {
        return Some(("stahlschrott-scheren", "S3"));
    }
    if l.contains("guss") || l.contains("guß") {
        if l.contains("alu") {
            return Some(("aluminium-guss", "bis 2% Anhaftung"));
        }
        return Some(("eisenschrott-gussbruch", ""));
    }
    if l.contains("altpapier") || l.contains("papier") {
        return None;
    }
    if l.contains("v4a") {
        return Some(("edelstahl-v4a", ""));
    }
    if l.contains("v2a") {
        return Some(("edelstahl-v2a", ""));
    }
    if l.contains("messing") {
        return Some(("messing", ""));
    }
    if l.contains("zink") {
        return Some(("zink", ""));
    }
    if l.contains("blei") {
        return Some(("blei", ""));
    }
    if l.contains("elektromotor") {
        return Some(("elektromotoren", ""));
    }
    None
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after
/// "Angaben gemäß § 5 TMG:" holding firm lines + street + PLZ city as
/// `<br>`-separated lines, and the first "Telefon:" number (phone-token
/// rule — the E-Mail is an image and stays empty). Missing anchors mean
/// the page changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p_sel = Selector::parse("p").expect("valid selector");
    let paras: Vec<_> = doc.select(&p_sel).collect();
    let anchor = paras.iter().position(|p| {
        p.text()
            .collect::<String>()
            .contains("Angaben gemäß § 5 TMG")
    });
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "TMG-Block fehlt".to_owned(),
        });
    };
    // Firm block: the <p> after the anchor naming Oder-Metalle.
    // ["O.M.N. Oder-Metalle GmbH", "Geschäftsführer: Sascha Lesner",
    //  "Neulewin 47a", "16259 Neulewin"]: street is the line before the
    // PLZ line. scraper text() would glue "47a"+"16259", so split the
    // <p> on <br> into real lines instead.
    let firm = paras[anchor..]
        .iter()
        .find(|p| p.text().collect::<String>().contains("Oder-Metalle"))
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        })?;
    let lines: Vec<String> = firm
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
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
    // First "Telefon:" line; phone-style tokens only (digits +/().-).
    // Line-wise (not whole-<p> text): the second number glues onto the
    // first across <br> ("…37 30"+"Telefon:") and would cut it short.
    let mut phone = String::new();
    if let Some(p) = paras
        .iter()
        .find(|p| p.text().collect::<String>().contains("Telefon:"))
    {
        let first_line = p
            .inner_html()
            .split("<br")
            .map(|s| strip_fragment(s))
            .find(|l| l.contains("Telefon:"));
        if let Some(line) = first_line {
            if let Some(i) = line.find("Telefon:") {
                phone = line[i + "Telefon:".len()..]
                    .split_whitespace()
                    .take_while(|t| {
                        t.chars()
                            .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
            }
        }
    }
    if street.is_empty() && phone.is_empty() {
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
        email: String::new(),
    })
}

/// Strip tags from a `<br>`-split fragment (discard everything up to the
/// first `>` so no `class="…"` rest parses as text).
fn strip_fragment(s: &str) -> String {
    let mut plain = String::new();
    let mut tag = false;
    for c in s.chars() {
        if c == '<' {
            tag = true;
        } else if c == '>' {
            tag = false;
        } else if !tag {
            plain.push(c);
        }
    }
    plain.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let table_sel = Selector::parse("table").expect("valid selector");
    let row_sel = Selector::parse("tr").expect("valid selector");
    let cell_sel = Selector::parse("td").expect("valid selector");
    let head_sel = Selector::parse("th").expect("valid selector");
    // Never trust page order: take the table carrying the Schrott-Art
    // header, not just the first <table> on the page.
    let table = doc.select(&table_sel).find(|t| {
        t.select(&head_sel).any(|h| {
            h.text()
                .collect::<String>()
                .to_lowercase()
                .contains("schrott-art")
        })
    });
    let Some(table) = table else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    };
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in table.select(&row_sel) {
        let cells: Vec<String> = tr.select(&cell_sel).map(|c| c.text().collect()).collect();
        if cells.len() < 2 {
            continue;
        }
        let label = cells[0].replace(['\u{a0}'], " ");
        let label = label.split_whitespace().collect::<Vec<_>>().join(" ");
        if label.is_empty() {
            // Spacer rows between the groups — layout, not labels.
            continue;
        }
        let price_raw = cells[1].replace(['\u{a0}'], " ");
        let Some(price) = parse_eur(&price_raw) else {
            skips.push(format!(
                "{label} (Preis unverständlich: {})",
                price_raw.trim()
            ));
            continue;
        };
        // Per-row unit, quoted honestly (record() normalizes kg↔t into
        // the catalog unit — never convert here).
        let Some(unit) = unit_of(&price_raw) else {
            skips.push(format!(
                "{label} (Einheit unverständlich: {})",
                price_raw.trim()
            ));
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
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS table's Preis cells (live: "110,00 €
/// / t", "10,40€ / kg"). Only kg/t exist here — anything else skips
/// loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|t| t == "t")
    {
        Some("EUR/t")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    // Real shape (live 27.09.2026): header row in <th>, empty spacer
    // rows, <strong>-wrapped rows, trailing spaces and &nbsp; cells.
    const FIXTURE: &str = "<table class=\"has-black-color has-text-color has-link-color\">\
        <tbody><tr><th>Schrott-Art</th><th>Preis *</th></tr>\
        <tr><td>Mischschrott</td><td>110,00 € / t</td></tr>\
        <tr><td>Guss</td><td>110,00 € / t</td></tr>\
        <tr><td>schwerer Langschrott</td><td>150,00 € / t&nbsp;</td></tr>\
        <tr><td>Sorte 3</td><td>170,00 € / t</td></tr>\
        <tr><td></td><td></td></tr>\
        <tr><td><strong>Altpapier</strong></td><td><strong>0,07€ / kg</strong></td></tr>\
        <tr><td></td><td></td></tr>\
        <tr><td>Kupferdraht blank (milberry) </td><td>10,40€ / kg </td></tr>\
        <tr><td>Kupfer Raff b95 </td><td>9,70€  / kg</td></tr>\
        <tr><td>&nbsp;</td><td>&nbsp;</td></tr>\
        <tr><td>Messing </td><td>5,60€ / kg</td></tr>\
        <tr><td>Edelstahl V2A </td><td>0,50 € / kg</td></tr>\
        </tbody></table>";

    #[test]
    fn rows_units_and_spacers_parse() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 9, "{rows:?}");
        assert_eq!(rows[0], ("Mischschrott".to_owned(), 110.0, "EUR/t"));
        assert_eq!(rows[1], ("Guss".to_owned(), 110.0, "EUR/t"));
        assert_eq!(rows[2], ("schwerer Langschrott".to_owned(), 150.0, "EUR/t"));
        assert_eq!(rows[3], ("Sorte 3".to_owned(), 170.0, "EUR/t"));
        assert_eq!(rows[4], ("Altpapier".to_owned(), 0.07, "EUR/kg"));
        assert_eq!(
            rows[5],
            ("Kupferdraht blank (milberry)".to_owned(), 10.4, "EUR/kg")
        );
        assert_eq!(rows[7], ("Messing".to_owned(), 5.6, "EUR/kg"));
        assert_eq!(rows[8], ("Edelstahl V2A".to_owned(), 0.5, "EUR/kg"));
        assert!(skips.is_empty(), "{skips:?}");
        assert_eq!(unit_of("110,00 € / t"), Some("EUR/t"));
        assert_eq!(unit_of("10,40€ / kg"), Some("EUR/kg"));
        assert_eq!(unit_of("pro Sack"), None);
    }

    #[test]
    fn header_and_empty_guards_fail_loudly() {
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 9);
        assert!(parse("<html><body><p>Neu</p></body></html>").is_err());
        // Unknown unit: skipped loudly, valid rows survive.
        let html = FIXTURE.replacen("110,00 € / t", "110,00 € pro Sack", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 8);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Mischschrott"));
        // Every row unparseable: loud error, not silent success.
        let html = FIXTURE
            .replace("/ kg", "pro Sack")
            .replace("/ t", "pro Sack");
        let err = parse(&html).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn impressum_tmg_block_with_image_email() {
        let imp = "<div class=\"entry-content clearfix\">\
            <p>Angaben gemäß § 5 TMG:</p>\
            <p>O.M.N. Oder-Metalle GmbH<br>Geschäftsführer: Sascha Lesner<br>\
            Neulewin 47a<br>16259 Neulewin</p><p>Kontakt:</p>\
            <p>Telefon: +49 (0) 33452 49 37 30<br>Telefon:&nbsp;+49 (0) 1523 696 3758<br>\
            Telefax:&nbsp;+49 (0) 33452 49 37 31<br>E-Mail:&nbsp; \
            <sub><img src=\"http://oder-metalle.de/wp-content/uploads/2024/05/email2.png\" alt=\"\"></sub></p>\
            </div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Neulewin 47a");
        assert_eq!(info.postcode, "16259");
        assert_eq!(info.city, "Neulewin");
        assert_eq!(info.phone, "+49 (0) 33452 49 37 30");
        assert_eq!(info.email, "", "email is an image — no text address exists");
        assert!(extract_info("<p>Neu hier</p>").is_err());
    }

    #[test]
    fn mapping_covers_every_live_label() {
        let cases: &[(&str, Option<(&str, &str)>)] = &[
            ("Mischschrott", Some(("mischschrott", ""))),
            ("Guss", Some(("eisenschrott-gussbruch", ""))),
            (
                "schwerer Langschrott",
                Some(("mischschrott", "schwerer Langschrott")),
            ),
            ("Sorte 3", Some(("stahlschrott-scheren", "S3"))),
            ("Altpapier", None),
            (
                "Kupferdraht blank (milberry)",
                Some(("kupfer-millberry", "")),
            ),
            ("Kupfer Raff b95", Some(("kupfer-gemischt", "Raff b95"))),
            ("Kupfer Kabel 40%", Some(("kabel-kupfer", "40%"))),
            (
                "Alu Misch ohne Guss",
                Some(("aluminium-gemischt", "ohne Guss")),
            ),
            ("Alufelgen", Some(("aluminium-guss", "Felgen"))),
            (
                "Alu Guß bis 2% anh.",
                Some(("aluminium-guss", "bis 2% Anhaftung")),
            ),
            ("Edelstahl V2A", Some(("edelstahl-v2a", ""))),
            ("Edelstahl V4A", Some(("edelstahl-v4a", ""))),
            ("Messing", Some(("messing", ""))),
            ("Zink", Some(("zink", ""))),
            ("Blei", Some(("blei", ""))),
            ("Elektromotore", Some(("elektromotoren", ""))),
        ];
        for (label, want) in cases {
            assert_eq!(&grade_for(label), want, "{label}");
        }
    }
}
