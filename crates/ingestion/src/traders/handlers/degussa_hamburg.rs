//! Degussa Goldhandel, Hamburg-Altstadt (Ballindamm 5, 20095): exact
//! per-gram Ankauf prices from the shared Goldrechner
//! (`form#goldCalculatorForm`: `div.metalType` rows with a machine
//! `data-price` attribute in EUR/g, the grade in `<label>`, the unit in a
//! `<span>/g</span>`), plus the branch contact block.
//!
//! Quoted unit is honestly EUR/g (catalog unit for `gold`/`zahngold`/
//! `silber`/`platin`/`palladium` is EUR/g — no conversion anywhere).
//! Fineness rides in the variant ("Silber 925" → `silber`/`925`),
//! Dentalgold maps to `zahngold` (dental alloy, never a gold alias).
//! "Keine Angabe" rows carry no fineness and are skipped loudly (a fixed
//! price for unknown alloy must not collapse into a generic variant).
//!
//! The Hamburg page names Dentalgold/Zahngold explicitly in its selling
//! range ("Brücken, Implantate, Kronen, Dentalplättchen" via the
//! Verkaufen assortment; branch prose buys "Edelmetalle … in egal
//! welcher Form" an). The shop Preisliste
//! (/de-de/header_navigation/preise/preisliste/) is deliberately NOT the
//! source: it lists investment bars/coins as per-piece retail prices,
//! which would need weight division (unit conversion — forbidden).
//! The page carries no validity date (only the calculator result), so
//! `published_at` stays `None` (`observed_at` = age).

use scraper::{ElementRef, Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "hh-altstadt-degussa-niederlassung-hamburg";
/// Bespoke, live-verified branch page (address + phone + mail live
/// 27.09.2026: Ballindamm 5, 20095 Hamburg). A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const BRANCH_URL: &str =
    "https://degussa.com/de-de/header_navigation/niederlassungen/hamburg/";

/// The working per-gram price source, hardcoded (intentional duplication
/// across the Degussa branch handlers — one file per branch on purpose).
pub const URL: &str = "https://degussa.com/de-de/header_navigation/preise/goldrechner/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::new();
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
            // No fineness, no mapping: keep the quoted price as evidence
            // in the skip, never drop it silently.
            None => skipped_labels.push(format!(
                "{label} ({}, {unit}, keine Feingehaltsangabe: Edelmetall)",
                fmt_eur(price)
            )),
        }
    }
    // Branch-page failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, branch_html) = fetch_text(client, BRANCH_URL).await?;
    let trader_info = extract_info(&branch_html)?;
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

/// Explicit label → (material, variant) mapping. Labels arrive as
/// "{Category} {grade}" ("Gold Feingold 999", "Silber 925", "Gold
/// Dentalgold"). Dental alloy maps to `zahngold`, never a gold alias.
/// Grades without fineness ("Gold Keine Angabe") stay `None` — a fixed
/// price for unknown alloy must not pose as a generic sort.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    if l.contains("dental") || l.contains("zahngold") {
        return Some(("zahngold", ""));
    }
    let fin = fineness(&l);
    if fin.is_empty() {
        return None;
    }
    if l.contains("silber") {
        return Some(("silber", fin));
    }
    if l.contains("platin") && !l.contains("palladium") {
        return Some(("platin", fin));
    }
    if l.contains("palladium") {
        return Some(("palladium", fin));
    }
    if l.contains("gold") {
        return Some(("gold", fin));
    }
    None
}

/// First 3-digit run in the label that names a traded fineness ("Silber
/// 925" → "925", "Palladium 500" → "500"). Unknown runs and digit-less
/// labels ("Keine Angabe") stay variant-less (""), never guessed.
fn fineness(l: &str) -> &'static str {
    let b = l.as_bytes();
    let mut i = 0;
    while i + 3 <= b.len() {
        // The digit guard keeps the byte slice on char boundaries.
        if b[i].is_ascii_digit() && b[i + 1].is_ascii_digit() && b[i + 2].is_ascii_digit() {
            match &l[i..i + 3] {
                "999" => return "999",
                "950" => return "950",
                "925" => return "925",
                "900" => return "900",
                "830" => return "830",
                "800" => return "800",
                "750" => return "750",
                "585" => return "585",
                "500" => return "500",
                "333" => return "333",
                _ => {}
            }
        }
        i += 1;
    }
    ""
}

/// Parse the calculator window between the form and the result block.
/// Returns (rows, skips); rows carry (label, price, unit).
fn parse(
    html: &str,
) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html.find("id=\"goldCalculatorForm\"").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Goldrechner fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("goldCalculator__result").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Goldrechner unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let row_sel = Selector::parse("div.metalType").expect("valid selector");
    let label_sel = Selector::parse("label").expect("valid selector");
    let span_sel = Selector::parse("span").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for row in frag.select(&row_sel) {
        let category = row
            .ancestors()
            .filter_map(ElementRef::wrap)
            .find_map(|a| a.value().attr("data-category"))
            .unwrap_or_default();
        let grade = row
            .select(&label_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if grade.is_empty() || grade.len() > 120 {
            continue;
        }
        let label =
            if category.is_empty() { grade.clone() } else { format!("{category} {grade}") };
        // Machine attribute, dot decimals ("110.8100") — parsed as-is by
        // the bespoke helper below, never through locale guessing.
        let price_attr = row.value().attr("data-price").unwrap_or_default();
        let Some(price) = price_of(price_attr) else {
            skips.push(format!("{label} (Preis unverständlich: {price_attr})"));
            continue;
        };
        let unit_text = row
            .select(&span_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();
        // An unparseable unit is a loud skip, never a silent default: a
        // per-kilo price recorded as per-gram would be a 1000x error.
        let Some(unit) = unit_of(&unit_text) else {
            skips.push(format!("{label} (Einheit unverständlich: {})", unit_text.trim()));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Goldrechner leer".to_owned() });
    }
    Ok((rows, skips))
}

/// Bespoke price reader for THIS calculator's `data-price` attributes
/// (live: "110.8100", always plain dot decimals). Anything else skips
/// loudly at the call site.
fn price_of(attr: &str) -> Option<f64> {
    let s = attr.trim();
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit() || b == b'.') {
        return None;
    }
    let v: f64 = s.parse().ok()?;
    if v.is_finite() && v > 0.0 { Some(v) } else { None }
}

/// Bespoke unit matcher for THIS calculator's unit spans (live: "/g").
/// Only g/kg/t exist here — anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("/g") || lower.contains("pro gramm") {
        Some("EUR/g")
    } else if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower.split(|c: char| !c.is_alphanumeric()).any(|t| t == "t") {
        Some("EUR/t")
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS branch page only: the Kontakt
/// `div.textcontent` under `<h2 class="h6">Kontakt:</h2>` carries the
/// firm line, street, "PLZ city", a tel: link and a mailto: link.
/// Missing anchor or closing div → loud error, never a guessed fallback.
fn extract_info(branch: &str) -> Result<TraderInfo, IngestError> {
    let start = branch.find(">Kontakt:</h2>").ok_or_else(|| IngestError::Parse {
        url: BRANCH_URL.to_owned(),
        detail: "Kontakt-Block fehlt".to_owned(),
    })?;
    let tail = &branch[start..];
    let end = tail.find("</div>").ok_or_else(|| IngestError::Parse {
        url: BRANCH_URL.to_owned(),
        detail: "Kontakt-Block unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    let lines: Vec<String> =
        window.split("<br").map(strip_fragment).filter(|s| !s.is_empty()).collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let (mut phone, mut email) = (String::new(), String::new());
    let mut prev = String::new();
    for line in &lines {
        if line == "Kontakt:" {
            continue;
        }
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(_)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = line[pc.len()..].trim().to_owned();
                street = prev.clone();
                continue;
            }
        }
        if line.contains('@') {
            email = line.clone();
        } else if line.chars().filter(|c| c.is_ascii_digit()).count() >= 6 {
            phone = line.clone();
        } else {
            prev = line.clone();
        }
    }
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: BRANCH_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo { street, postcode, city, phone, email })
}

/// Strip tags from a `<br>`-split fragment. Fragments start with a tag
/// remnant — drop everything up to the first '>' first, or attributes
/// parse as text.
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

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, price_of, unit_of};

    // Real shape of the live calculator (form id, data-category groups,
    // data-id + machine data-price, label text, /g span, result block),
    // trimmed to eleven of eighteen rows (inputs shortened).
    const FIXTURE: &str = "<form id=\"goldCalculatorForm\" data-locale=\"de-DE\" data-currency=\"EUR\">\
        <div class=\"slidecontent\"><div class=\"slidecontentItem gold\" data-category=\"Gold\">\
        <div class=\"slidecontentItem__content\"><fieldset>\
        <div class=\"metalType\" data-id=\"1S999\" data-price=\"110.8100\">\
        <label for=\"1S999\">Feingold 999</label>\
        <div><input id=\"1S999\" type=\"number\"/><span>/g</span></div></div>\
        <div class=\"metalType\" data-id=\"1S585\" data-price=\"59.9600\">\
        <label for=\"1S585\">585</label>\
        <div><input id=\"1S585\" type=\"number\"/><span>/g</span></div></div>\
        <div class=\"metalType\" data-id=\"1S500\" data-price=\"51.2500\">\
        <label for=\"1S500\">Keine Angabe</label>\
        <div><input id=\"1S500\" type=\"number\"/><span>/g</span></div></div>\
        <div class=\"metalType\" data-id=\"1SDENT\" data-price=\"83.3100\">\
        <label for=\"1SDENT\">Dentalgold</label>\
        <div><input id=\"1SDENT\" type=\"number\"/><span>/g</span></div></div>\
        </fieldset></div></div>\
        <div class=\"slidecontentItem silver\" data-category=\"Silber\">\
        <div class=\"slidecontentItem__content\"><fieldset>\
        <div class=\"metalType\" data-id=\"2S999\" data-price=\"1.3800\">\
        <label for=\"2S999\">Feinsilber 999</label>\
        <div><input id=\"2S999\" type=\"number\"/><span>/g</span></div></div>\
        <div class=\"metalType\" data-id=\"2S500\" data-price=\"0.6200\">\
        <label for=\"2S500\">Keine Angabe</label>\
        <div><input id=\"2S500\" type=\"number\"/><span>/g</span></div></div>\
        </fieldset></div></div>\
        <div class=\"slidecontentItem platin\" data-category=\"Platin\">\
        <div class=\"slidecontentItem__content\"><fieldset>\
        <div class=\"metalType\" data-id=\"3S999\" data-price=\"35.9600\">\
        <label for=\"3S999\">Feinplatin 999</label>\
        <div><input id=\"3S999\" type=\"number\"/><span>/g</span></div></div>\
        <div class=\"metalType\" data-id=\"3S500\" data-price=\"16.5000\">\
        <label for=\"3S500\">Keine Angabe</label>\
        <div><input id=\"3S500\" type=\"number\"/><span>/g</span></div></div>\
        </fieldset></div></div>\
        <div class=\"slidecontentItem palladium\" data-category=\"Palladium\">\
        <div class=\"slidecontentItem__content\"><fieldset>\
        <div class=\"metalType\" data-id=\"4S999\" data-price=\"26.4800\">\
        <label for=\"4S999\">Feinpalladium 999</label>\
        <div><input id=\"4S999\" type=\"number\"/><span>/g</span></div></div>\
        <div class=\"metalType\" data-id=\"4S950\" data-price=\"23.4900\">\
        <label for=\"4S950\">950</label>\
        <div><input id=\"4S950\" type=\"number\"/><span>/g</span></div></div>\
        <div class=\"metalType\" data-id=\"4S500\" data-price=\"12.3600\">\
        <label for=\"4S500\">500</label>\
        <div><input id=\"4S500\" type=\"number\"/><span>/g</span></div></div>\
        </fieldset></div></div></div>\
        <div class=\"goldCalculator__result\">\
        <div class=\"goldCalculator__result--headline\">Ihr Edelmetallwert</div></div></form>";

    #[test]
    fn calculator_rows_and_units() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 11);
        assert!(skips.is_empty());
        // Displayed machine price wins, parsed as dot decimals.
        assert_eq!(rows[0], ("Gold Feingold 999".to_owned(), 110.81, "EUR/g"));
        assert_eq!(rows[1], ("Gold 585".to_owned(), 59.96, "EUR/g"));
        assert_eq!(rows[3], ("Gold Dentalgold".to_owned(), 83.31, "EUR/g"));
        assert_eq!(rows[4], ("Silber Feinsilber 999".to_owned(), 1.38, "EUR/g"));
        assert_eq!(rows[6], ("Platin Feinplatin 999".to_owned(), 35.96, "EUR/g"));
        assert_eq!(rows[10], ("Palladium 500".to_owned(), 12.36, "EUR/g"));
        assert_eq!(unit_of("/g"), Some("EUR/g"));
        assert_eq!(unit_of("pro Sack"), None);
        assert_eq!(price_of("110.8100"), Some(110.81));
        assert_eq!(price_of("0.6200"), Some(0.62));
        assert_eq!(price_of(""), None);
        assert_eq!(price_of("11,60 €"), None);
        assert!(parse("<div>Redesign ohne Rechner</div>").is_err());
        assert!(parse("<form id=\"goldCalculatorForm\">leer</form>").is_err());
    }

    #[test]
    fn fineness_rides_in_variant() {
        assert_eq!(grade_for("Gold Feingold 999"), Some(("gold", "999")));
        assert_eq!(grade_for("Gold 900"), Some(("gold", "900")));
        assert_eq!(grade_for("Gold 750"), Some(("gold", "750")));
        assert_eq!(grade_for("Gold 585"), Some(("gold", "585")));
        assert_eq!(grade_for("Gold 333"), Some(("gold", "333")));
        // Dental alloy, never a gold alias.
        assert_eq!(grade_for("Gold Dentalgold"), Some(("zahngold", "")));
        assert_eq!(grade_for("Silber Feinsilber 999"), Some(("silber", "999")));
        assert_eq!(grade_for("Silber 925"), Some(("silber", "925")));
        assert_eq!(grade_for("Silber 830"), Some(("silber", "830")));
        assert_eq!(grade_for("Silber 800"), Some(("silber", "800")));
        assert_eq!(grade_for("Platin Feinplatin 999"), Some(("platin", "999")));
        assert_eq!(grade_for("Platin 950"), Some(("platin", "950")));
        assert_eq!(grade_for("Palladium Feinpalladium 999"), Some(("palladium", "999")));
        assert_eq!(grade_for("Palladium 950"), Some(("palladium", "950")));
        assert_eq!(grade_for("Palladium 500"), Some(("palladium", "500")));
        // Unknown alloy: loud skip, never a generic variant.
        assert_eq!(grade_for("Gold Keine Angabe"), None);
        assert_eq!(grade_for("Silber Keine Angabe"), None);
        assert_eq!(grade_for("Platin Keine Angabe"), None);
        assert_eq!(grade_for("Online verkaufen"), None);
    }

    #[test]
    fn branch_kontakt_block() {
        // Real Kontakt block of the live Hamburg branch page (gallery
        // and scripts omitted).
        let branch = "<div class=\"textcontent noBackground\" >\
            <h2 class=\"h6\">Kontakt:</h2><br />\
            Degussa Sonne/Mond Goldhandel GmbH<br />\
            Ballindamm 5<br />20095 Hamburg<br /><br />\
            <a class=\"linkWithIcon\" href=\"https://maps.app.goo.gl/gjmrsQrnJazkwKXq6\" target=\"_blank\">\
            <i class=\"icon icon-map\"></i><span></span><span>Route zur Niederlassung</span>\
            <span></span></a><br /><br />\
            <a class=\"linkWithIcon\" href=\"tel:+4904032908720\">\
            <i class=\"icon icon-phone\"></i><span>+49 (0)40 3290872-0</span></a><br /><br />\
            <a class=\"linkWithIcon\" href=\"mailto:hamburg@degussa.com\">\
            <i class=\"icon icon-mail\"></i><span>hamburg@degussa.com</span></a></div>";
        let info = extract_info(branch).expect("parses");
        assert_eq!(info.street, "Ballindamm 5");
        assert_eq!(info.postcode, "20095");
        assert_eq!(info.city, "Hamburg");
        assert_eq!(info.phone, "+49 (0)40 3290872-0");
        assert_eq!(info.email, "hamburg@degussa.com");
        assert!(extract_info("<p>Neu hier</p>").is_err());
    }
}
