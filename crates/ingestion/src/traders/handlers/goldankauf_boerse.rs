//! Goldankauf Börse, Filiale Erfurt: exact per-gram prices in the static
//! live Kurstabelle (`div.GoldXML_Line_*`: label in
//! `span.GoldXML_left_*`, quote in `span.GoldXML_right_*` as "118,62
//! €/g") across the Gold/Silber/Goldbarren/Platin/Palladium sections,
//! with the quote time in `div.GoldXML_Zeit` ("Stand: …"). The table is
//! rendered twice (desktop + mobile block) — identical (label, price,
//! unit) rows dedupe after parsing.
//!
//! Quoted unit is honestly EUR/g (catalog units for gold/zahngold/
//! silber/platin/palladium are EUR/g — no conversion anywhere). Fineness
//! rides in the variant ("Zahngold 750" → `zahngold`/`750`); "500er
//! (Zahnpalladium*)" is dental palladium → `palladium`.
//!
//! Note: the interactive Ankaufsrechner above the table is JS-driven
//! (weight inputs, computed results); only the static Kurstabelle below
//! it is parsed — never a simulated calculation.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "th-erfurt-goldankauf-boerse-filiale-erfurt";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.goldankauf-boerse.de/unternehmen/impressum/";

pub const URL: &str = "https://www.goldankauf-boerse.de/ankaufsrechner/";

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
    let mut prices = Vec::new();
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(super::super::ScrapedPrice {
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
            // No catalog material for precious metal: keep the quoted
            // price as evidence in the skip, never drop it silently.
            None => skipped_labels.push(format!(
                "{label} ({}, {unit}, kein Katalogmaterial: Edelmetall)",
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
        published_at,
    })
}

fn fmt_eur(price: f64) -> String {
    format!("{price:.2}").replace('.', ",")
}

/// Explicit label → (material, variant) mapping. Fineness rides in the
/// variant ("Zahngold 750" → `zahngold`/`750`); dental palladium is
/// palladium, never a gold alias. Labels without metal word stay `None`.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    if l.contains("zahngold") {
        return Some(("zahngold", fineness(&l)));
    }
    if l.contains("zahnpalladium") {
        return Some(("palladium", fineness(&l)));
    }
    if l.contains("silber") {
        return Some(("silber", fineness(&l)));
    }
    if l.contains("palladium") {
        return Some(("palladium", fineness(&l)));
    }
    if l.contains("platin") {
        return Some(("platin", fineness(&l)));
    }
    if l.contains("gold")
        || l.contains("feingold")
        || l.contains("barren")
        || l.contains("gestempelt")
    {
        // Chain Sonderpreis ("999er gestempelt im Neuzustand bis 2g")
        // must not collide with plain "999er Feingold": it gets its own
        // variant (feedback: Import-Kollision 30.09.2026).
        if l.contains("gestempelt") {
            return Some(("gold", "999-gestempelt-bis-2g"));
        }
        return Some(("gold", fineness(&l)));
    }
    None
}

/// First 3-digit run in the label ("999er Feingold" → "999"). Unknown
/// fineness stays "" — never guessed.
fn fineness(l: &str) -> &'static str {
    let b = l.as_bytes();
    let mut i = 0;
    while i + 3 <= b.len() {
        if b[i].is_ascii_digit() && b[i + 1].is_ascii_digit() && b[i + 2].is_ascii_digit() {
            return match &l[i..i + 3] {
                "999" => "999",
                "986" => "986",
                "959" => "959",
                "950" => "950",
                "925" => "925",
                "916" => "916",
                "900" => "900",
                "835" => "835",
                "800" => "800",
                "750" => "750",
                "625" => "625",
                "600" => "600",
                "585" => "585",
                "500" => "500",
                "375" => "375",
                "333" => "333",
                _ => "",
            };
        }
        i += 1;
    }
    ""
}

/// Parse the Kurstabelle window between the `id="kurse"` icon block and
/// the `id="kurse-div"` block that follows the table. Returns
/// (published_at, deduped rows, unit_skips).
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
    let start = html
        .find("id=\"kurse\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Kurstabelle fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("id=\"kurse-div\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Kurstabelle unvollständig".to_owned(),
        })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let line_sel = Selector::parse("div[class*=GoldXML_Line]").expect("valid selector");
    let left_sel = Selector::parse("span[class*=GoldXML_left]").expect("valid selector");
    let right_sel = Selector::parse("span[class*=GoldXML_right]").expect("valid selector");
    let mut rows = Vec::new();
    let mut unit_skips = Vec::new();
    for line in frag.select(&line_sel) {
        let label = line
            .select(&left_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .replace(['\u{a0}'], " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if label.is_empty() || label.len() > 120 {
            continue;
        }
        let right = line
            .select(&right_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();
        let Some(price) = parse_eur(&right) else {
            unit_skips.push(format!("{label} (Preis unverständlich: {})", right.trim()));
            continue;
        };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-kilo price recorded as per-gram would be a 1000x error.
        let Some(unit) = unit_of(&right) else {
            unit_skips.push(format!(
                "{label} (Einheit unverständlich: {})",
                right.trim()
            ));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Kurstabelle leer".to_owned(),
        });
    }
    // Desktop + mobile blocks repeat the table: dedupe identical
    // (label, price, unit) rows after parsing.
    let mut seen = std::collections::HashSet::new();
    rows.retain(|(l, p, u)| seen.insert((l.clone(), p.to_bits(), u.to_string())));
    let published_at = find_date(window);
    Ok((published_at, rows, unit_skips))
}

/// Bespoke unit matcher for THIS table's quote cells (live: "118,62
/// €/g"). Only g/kg/t exist here — anything else skips loudly at the
/// call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("/g") || lower.contains("pro gramm") {
        Some("EUR/g")
    } else if lower.contains("kg") {
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

/// Bespoke date finder for THIS table: `div.GoldXML_Zeit` holds "Stand:
/// 27.09.2026 19:11:30". Missing → None (observation age stays the
/// provenance).
fn find_date(window: &str) -> Option<String> {
    let (_, after) = window.split_once("Stand:")?;
    let date = after.split_whitespace().next()?;
    let parts: Vec<&str> = date.split('.').collect();
    if parts.len() == 3 {
        parse_de_date(parts[0], parts[1], parts[2])
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the
/// `id="angaben-gemass-5-tmg"` heading ("Angaben gemäß § 5 TMG:") is
/// followed by the address `<p>` ("Herr Ilhan Kör", street, "D-99084
/// Erfurt"), and the `id="kontakt"` heading by the contact `<p>`
/// ("Telefon:" / "E-Mail:" lines). Missing headings → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let addr_p = doc
        .select(&h2)
        .find(|h| {
            h.value().attr("id") == Some("angaben-gemass-5-tmg")
                || h.text()
                    .collect::<String>()
                    .contains("Angaben gemäß § 5 TMG")
        })
        .and_then(next_p);
    let cont_p = doc
        .select(&h2)
        .find(|h| {
            h.value().attr("id") == Some("kontakt")
                || h.text().collect::<String>().trim() == "Kontakt:"
        })
        .and_then(next_p);
    let (Some(addr_html), Some(cont_html)) = (addr_p, cont_p) else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben/Kontakt-Block fehlt".to_owned(),
        });
    };
    let addr_lines: Vec<String> = addr_html
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in addr_lines.iter().enumerate() {
        // "D-99084 Erfurt": strip the country prefix, keep PLZ + city.
        let bare = line.strip_prefix("D-").unwrap_or(line).to_owned();
        let mut it = bare.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| format!("{a} {b}"));
                if k > 0 {
                    street = addr_lines[k - 1].clone();
                }
                break;
            }
        }
    }
    let mut phone = String::new();
    let mut email = String::new();
    for part in cont_html.split("<br") {
        let t = strip_fragment(part);
        if let Some(v) = t.strip_prefix("Telefon:") {
            phone = v.trim().to_owned();
        } else if let Some(v) = t.strip_prefix("E-Mail:") {
            email = v.trim().to_owned();
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

/// First `<p>` among the heading's following siblings (wild nesting
/// defeats a pure sibling walk, so headings stay the required anchors).
fn next_p(h: ElementRef<'_>) -> Option<String> {
    h.next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p")
        .map(|p| p.inner_html())
}

/// Strip tags from a `<br>`-split fragment (see hansa handler: drop up to
/// the first '>' first so attributes never parse as text).
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
    use super::{find_date, grade_for, parse, unit_of};

    // Real shape of the live Kurstabelle (GoldXML ids, nested
    // Zahnpalladium span, Zeit div), trimmed to two sections.
    const FIXTURE: &str = "<div id=\"kurse\"></div>\
        <div id=\"GoldXML_fieldset\"><div id=\"GoldXML_spalte_1\">\
        <div id=\"GoldXML_field_Gold\" class=\"GoldXML_field GoldXML_field_Gold\"><h3>Gold</h3>\
        <div id=\"GoldXML_Line_Gold_1\" class=\"GoldXML_Line GoldXML_Line_Gold_1 clearfix\">\
        <span id=\"GoldXML_left_Gold_1\" class=\"GoldXML_left GoldXML_left_Gold_1\">999er Feingold</span>\
        <span id=\"GoldXML_right_Gold_1\" class=\"GoldXML_right GoldXML_right_Gold_1\">118,62 €/g</span></div>\
        <div id=\"GoldXML_Line_Gold_9\" class=\"GoldXML_Line GoldXML_Line_Gold_9 clearfix\">\
        <span id=\"GoldXML_left_Gold_9\" class=\"GoldXML_left GoldXML_left_Gold_9\">Zahngold 750</span>\
        <span id=\"GoldXML_right_Gold_9\" class=\"GoldXML_right GoldXML_right_Gold_9\">88,42 €/g</span></div></div>\
        <div id=\"GoldXML_field_Palladium\" class=\"GoldXML_field GoldXML_field_Palladium\"><h3>Palladium</h3>\
        <div id=\"GoldXML_Line_Palladium_4\" class=\"GoldXML_Line GoldXML_Line_Palladium_4 clearfix\">\
        <span id=\"GoldXML_left_Palladium_4\" class=\"GoldXML_left GoldXML_left_Palladium_4\">500er \
        <span id=\"GoldXML_500er_spezial\">(Zahnpalladium*)</span></span>\
        <span id=\"GoldXML_right_Palladium_4\" class=\"GoldXML_right GoldXML_right_Palladium_4\">15,54 €/g</span></div></div>\
        </div><div id=\"GoldXML_Zeit\" class=\"GoldXML_Zeit\">Stand: 27.09.2026 19:11:30</div></div>\
        <div id=\"kurse-div\"></div>";

    #[test]
    fn kurse_lines_units_and_stand() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-27T00:00:00+00:00"));
        assert_eq!(rows.len(), 3);
        assert!(skips.is_empty());
        assert_eq!(rows[0], ("999er Feingold".to_owned(), 118.62, "EUR/g"));
        assert_eq!(rows[1], ("Zahngold 750".to_owned(), 88.42, "EUR/g"));
        assert_eq!(rows[2].0, "500er (Zahnpalladium*)");
        assert_eq!(rows[2].1, 15.54);
        assert_eq!(unit_of("118,62 €/g"), Some("EUR/g"));
        assert_eq!(unit_of("pro Sack"), None);
        assert_eq!(find_date("ohne Stand"), None);
    }

    #[test]
    fn double_block_dedupes() {
        // Desktop + mobile repeat the table: identical rows collapse.
        let start = FIXTURE
            .find("<div id=\"GoldXML_fieldset\"")
            .expect("anchor");
        let end = FIXTURE.find("<div id=\"kurse-div\"").expect("anchor");
        let block = &FIXTURE[start..end];
        let doubled = FIXTURE.replacen(
            "<div id=\"kurse-div\">",
            &format!("{block}<div id=\"kurse-div\">"),
            1,
        );
        let (_, rows, _) = parse(&doubled).expect("parses");
        assert_eq!(rows.len(), 3, "doubled block dedupes");
    }

    #[test]
    fn gaps_error_loudly() {
        let html = FIXTURE.replacen("118,62 €/g", "118,62 pro Sack", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Feingold"));
        assert!(parse("<div>Redesign ohne Kurse</div>").is_err());
        assert!(parse("<div id=\"kurse\">ohne Ende").is_err());
    }

    #[test]
    fn fineness_rides_in_variant() {
        assert_eq!(grade_for("625er Silber"), Some(("silber", "625")));
        assert_eq!(grade_for("999er Feingold"), Some(("gold", "999")));
        assert_eq!(grade_for("916er Gold"), Some(("gold", "916")));
        assert_eq!(grade_for("585er Gold"), Some(("gold", "585")));
        assert_eq!(grade_for("333er Gold"), Some(("gold", "333")));
        assert_eq!(grade_for("Zahngold 750"), Some(("zahngold", "750")));
        assert_eq!(grade_for("Zahngold 600"), Some(("zahngold", "600")));
        assert_eq!(grade_for("999er Feinsilber"), Some(("silber", "999")));
        assert_eq!(
            grade_for("999er gestempelt im Neuzustand bis 2g"),
            Some(("gold", "999-gestempelt-bis-2g"))
        );
        assert_eq!(grade_for("999er Platin"), Some(("platin", "999")));
        assert_eq!(grade_for("999er Palladium"), Some(("palladium", "999")));
        assert_eq!(
            grade_for("500er (Zahnpalladium*)"),
            Some(("palladium", "500"))
        );
        assert_eq!(grade_for("Ankaufsbedingungen"), None);
    }

    #[test]
    fn impressum_headings() {
        let imp = "<h2 id=\"angaben-gemass-5-tmg\">Angaben gemäß § 5 TMG:</h2>\
            <p>Herr Ilhan Kör<br><br>Bahnhofstr. 38<br>D-99084 Erfurt</p>\
            <h2 id=\"kontakt\">Kontakt:</h2>\
            <p>Telefon: +49 (0)361 / 65 78 24 70<br>Telefax: +49 (0)361 / 65 78 24 71<br>\
            E-Mail: info@goldankauf-boerse.de</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Bahnhofstr. 38");
        assert_eq!(info.postcode, "99084");
        assert_eq!(info.city, "Erfurt");
        assert_eq!(info.phone, "+49 (0)361 / 65 78 24 70");
        assert_eq!(info.email, "info@goldankauf-boerse.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}
