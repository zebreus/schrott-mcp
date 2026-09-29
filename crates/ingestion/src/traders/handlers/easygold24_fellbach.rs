//! easygold24 Fellbach (Hartmann & Benz GmbH, Gutenbergstraße 40,
//! 70736 Fellbach): tagesaktuelle Leiterplatten-Scheidgut-Vergütungen.
//!
//! Live shape (28.09.2026, 433 KB): the "Tagesaktuelle Konditionen" card
//! carries `table.eg-scheidgut__table` (thead: Feinmetall / Börsenkurs /
//! Abschlag / Vergütung / Quote) with exactly four body rows (Gold, Silber,
//! Platin, Palladium). Each row shows Börsenkurs, Abschlag and the payout
//! Vergütung (= Börsenkurs − Abschlag, formula stated above the table);
//! machine values ride in `data-sg-eur` (dot decimals) plus
//! `data-sg-unit` ("g"/"kg"), display texts are German-rounded
//! ("116,71 €/g", "1.730 €/kg").
//!
//! Modeling: `price` is the Vergütung column — the actual tagesaktuelle
//! payout, exact at observation time (`price_kind: "exact"`,
//! `confidence: 1.0`). The Abschlag is provenance, kept verbatim in the
//! label (notes) — never a `price_max` without kind. Machine values are
//! per-gram throughout (the silver machine figure 1.729749 equals the
//! site's own €/g live header "1,73" although its display unit is kg);
//! only the display-text fallback converts a per-kg quote via /1000. A "0,00 €" Vergütung (calculator null) skips loudly, never as a
//! zero price. The "Alle Ankaufskurse nach Reinheit" modal (Altgold
//! purity rows like "999 Feingold") is a different product and
//! deliberately outside the window — this handler covers Scheidgut only.
//! No page-stated price validity date (live Tageskurse), so
//! `published_at` stays `None` (`observed_at` = age).

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bw-fellbach-easygold24-hartmann-benz";
/// Bespoke, live-verified Scheidgut price page (Leiterplatten table with
/// the Konditionen card). A move fails the step loudly — never guessed,
/// never shared.
pub const URL: &str = "https://easygold24.com/scheidgut/leiterplatten/";
/// Bespoke, live-verified impressum URL (site footer's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "https://easygold24.com/rechtliches/impressum/";

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
    for (metal, price, unit, label) in rows {
        match grade_for(&metal) {
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
            // New Konditionen row without catalog mapping: keep the quoted
            // payout as evidence in the skip, never drop it silently.
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
        published_at: None,
    })
}

fn fmt_eur(price: f64) -> String {
    format!("{price:.2}").replace('.', ",")
}

/// Explicit metal → (material, variant). Row headers are exactly the four
/// fein metals; anything else (e.g. a future Kupfer row) stays `None` —
/// Scheidgut copper shares have no catalog grade and must not pose as one.
fn grade_for(metal: &str) -> Option<(&'static str, &'static str)> {
    match metal.trim().to_lowercase().as_str() {
        "gold" => Some(("gold", "")),
        "silber" => Some(("silber", "")),
        "platin" => Some(("platin", "")),
        "palladium" => Some(("palladium", "")),
        _ => None,
    }
}

/// Parse the Konditionen window between the card badge and the next
/// section heading. Returns (rows, skips); rows carry
/// (metal, payout price in catalog unit, unit, label).
fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str, String)>, Vec<String>), IngestError> {
    let start = html
        .find("Tagesaktuelle Konditionen")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Konditionen-Tabelle fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Der Feinmetallgehalt entscheidet")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Konditionen-Tabelle unvollständig".to_owned(),
        })?;
    let window = &tail[..end];
    // Head-anchored table choice (guide: Kopfinhalt, nie die erste).
    for head in ["Feinmetall", "Börsenkurs", "Abschlag", "Vergütung", "Quote"] {
        if !window.contains(head) {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: format!("Tabellenkopf fehlt ({head})"),
            });
        }
    }
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let row_sel = Selector::parse("table.eg-scheidgut__table tbody tr").expect("valid selector");
    let th_sel = Selector::parse("th").expect("valid selector");
    let td_sel = Selector::parse("td").expect("valid selector");
    let verg_sel = Selector::parse("td.eg-scheidgut__verg").expect("valid selector");
    let ab_sel = Selector::parse("td.eg-scheidgut__abschlag").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for row in frag.select(&row_sel) {
        let metal = row
            .select(&th_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if metal.is_empty() || metal.len() > 120 {
            continue;
        }
        let kurs = row
            .select(&td_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let abschlag = row
            .select(&ab_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let Some(verg_el) = row.select(&verg_sel).next() else {
            skips.push(format!("{metal} (Vergütungsspalte fehlt)"));
            continue;
        };
        let verg_text = verg_el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        // Machine attribute wins (exact dot decimals); display text is the
        // fallback — never locale-guessed.
        let attr = verg_el.value().attr("data-sg-eur").unwrap_or_default();
        // A per-kilo payout recorded as per-gram would be a 1000x error —
        // unparseable units skip loudly, never default.
        let unit_attr = verg_el.value().attr("data-sg-unit").unwrap_or_default();
        let Some(quoted) = unit_of(unit_attr, &verg_text) else {
            skips.push(format!("{metal} (Einheit unverständlich: {verg_text})"));
            continue;
        };
        // Machine `data-sg-eur` is always per-gram (live-corroborated: the
        // silver machine value 1.729749 equals the €/g live header "1,73"
        // although its display unit is kg). Only the display-text fallback
        // converts a per-kg quote (cederbaum pattern, exact kg→g).
        let Some(payout) = price_of(attr).map(|m| (m, "EUR/g")).or_else(|| {
            parse_eur(&verg_text).map(|v| {
                if quoted == "EUR/kg" {
                    (v / 1000.0, "EUR/g")
                } else {
                    (v, "EUR/g")
                }
            })
        }) else {
            skips.push(format!("{metal} (Preis unverständlich: {verg_text})"));
            continue;
        };
        let (price, unit) = payout;
        if price <= 0.0 {
            skips.push(format!("{metal} (Vergütung 0,00 €: kein Kurs)"));
            continue;
        }
        let label = format!("{metal} (Börsenkurs {kurs}, Abschlag {abschlag}, Vergütung {verg_text})");
        rows.push((metal, price, unit, label));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Konditionen-Tabelle leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke price reader for THIS table's `data-sg-eur` attributes (live:
/// "116.206542", always plain dot decimals). Anything else falls back to
/// display-text parsing at the call site.
fn price_of(attr: &str) -> Option<f64> {
    let s = attr.trim();
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit() || b == b'.') {
        return None;
    }
    let v: f64 = s.parse().ok()?;
    if v.is_finite() && v >= 0.0 { Some(v) } else { None }
}

/// Bespoke unit matcher for THIS table's Vergütung cells: the machine
/// `data-sg-unit` ("g"/"kg") wins, the display suffix ("€/g", "€/kg")
/// corroborates. Explicit-but-foreign skips loudly at the call site.
fn unit_of(attr: &str, text: &str) -> Option<&'static str> {
    match attr.trim() {
        "g" => Some("EUR/g"),
        "kg" => Some("EUR/kg"),
        _ => {
            let l = text.to_lowercase();
            if l.contains("€/kg") || l.contains("eur/kg") {
                Some("EUR/kg")
            } else if l.contains("€/g") || l.contains("/g") || l.contains("pro gramm") {
                Some("EUR/g")
            } else {
                None
            }
        }
    }
}

/// Bespoke contact extraction for THIS impressum only: anchored on the
/// "Verantwortlich i.S.d." heading, the address `<p>` carries
/// "Hartmann & Benz GmbH<br>Gutenbergstraße 40<br>70736 Fellbach" and the
/// next `<p>` the "Telefon:" display line. Mail comes from the Cloudflare
/// `data-cfemail` attribute (hex-xor, decoded below — the page shows only
/// "[email protected]"). Missing anchors → loud error, never guessed.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let start = imp.find("Verantwortlich i.S.d.").ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Verantwortlich-Block fehlt".to_owned(),
    })?;
    let tail = &imp[start..];
    let end = tail.find("USt-IdNr").ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Verantwortlich-Block unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    if !window.contains("Hartmann") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "falsche Firma (kein Hartmann)".to_owned(),
        });
    }
    let lines: Vec<String> = window
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let mut prev = String::new();
    for line in &lines {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(_)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = line[pc.len()..].trim().to_owned();
                street = prev.clone();
                break;
            }
        }
        prev = line.clone();
    }
    // "Telefon:" rides glued to the previous </p> ("DeutschlandTelefon:",
    // the guide's glued-nodes gotcha) — split_once, never prefix match.
    let phone = lines
        .iter()
        .find_map(|line| line.split_once("Telefon:"))
        .map(|(_, v)| v.trim().to_owned())
        .unwrap_or_default();
    let email = window
        .split("data-cfemail=\"")
        .nth(1)
        .and_then(|v| v.split('"').next())
        .and_then(cf_decode)
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

/// Cloudflare `/cdn-cgi/l/email-protection` decoder for THIS impressum:
/// first byte is the xor key, the rest is the address. Anything
/// non-hex → `None` (mail stays empty, never guessed).
fn cf_decode(hex: &str) -> Option<String> {
    if hex.len() < 4 || hex.len() % 2 != 0 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let key = u8::from_str_radix(&hex[..2], 16).ok()?;
    let mut bytes = Vec::with_capacity(hex.len() / 2 - 1);
    for i in (2..hex.len()).step_by(2) {
        bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).ok().map(|b| b ^ key)?);
    }
    String::from_utf8(bytes).ok()
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
    // Live quirk: "Telefon:&nbsp; …" — the entity survives tag stripping,
    // so normalize it before whitespace folding (covered by test).
    out.replace("&nbsp;", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::{cf_decode, extract_info, grade_for, parse, price_of, unit_of};

    // Real shape of the live Konditionen card (badge anchor, formula,
    // thead heads, four body rows with machine data-sg-eur/data-sg-unit
    // plus German display texts, Quote column), terminated by the next
    // section heading exactly like live.
    const FIXTURE: &str = "<span class=\"eg-badge eg-badge--success\">\
        <span class=\"eg-badge__dot eg-badge__dot--live\"></span> Tagesaktuelle Konditionen</span>\
        <span>Vergütung = Börsenkurs − Abschlag</span>\
        <table class=\"eg-table eg-scheidgut__table\"><thead><tr>\
        <th scope=\"col\">Feinmetall</th>\
        <th scope=\"col\" class=\"eg-scheidgut__num\">Börsenkurs</th>\
        <th scope=\"col\" class=\"eg-scheidgut__num\">Abschlag</th>\
        <th scope=\"col\" class=\"eg-scheidgut__num eg-scheidgut__verg-col\">Vergütung</th>\
        <th scope=\"col\" class=\"eg-scheidgut__num\">Quote</th></tr></thead><tbody>\
        <tr><th scope=\"row\"><span class=\"eg-scheidgut__metal\">\
        <span class=\"eg-scheidgut__metal-dot\" data-metal=\"gold\"></span>Gold</span></th>\
        <td class=\"eg-scheidgut__num\" data-sg-eur=\"116.706542\" data-sg-unit=\"g\">116,71 €/g</td>\
        <td class=\"eg-scheidgut__num eg-scheidgut__abschlag\" data-sg-eur=\"0.5\" data-sg-unit=\"g\" data-sg-neg=\"1\">−0,50 €/g</td>\
        <td class=\"eg-scheidgut__num eg-scheidgut__verg eg-scheidgut__verg-col\" data-sg-eur=\"116.206542\" data-sg-unit=\"g\">116,21 €/g</td>\
        <td class=\"eg-scheidgut__num\"><span class=\"eg-scheidgut__quote\">99,57 %</span></td></tr>\
        <tr><th scope=\"row\"><span class=\"eg-scheidgut__metal\">\
        <span class=\"eg-scheidgut__metal-dot\" data-metal=\"silver\"></span>Silber</span></th>\
        <td class=\"eg-scheidgut__num\" data-sg-eur=\"1.729749\" data-sg-unit=\"kg\">1.730 €/kg</td>\
        <td class=\"eg-scheidgut__num eg-scheidgut__abschlag\" data-sg-eur=\"0.1\" data-sg-unit=\"kg\" data-sg-neg=\"1\">−100 €/kg</td>\
        <td class=\"eg-scheidgut__num eg-scheidgut__verg eg-scheidgut__verg-col\" data-sg-eur=\"1.629749\" data-sg-unit=\"kg\">1.630 €/kg</td>\
        <td class=\"eg-scheidgut__num\"><span class=\"eg-scheidgut__quote\">94,22 %</span></td></tr>\
        <tr><th scope=\"row\"><span class=\"eg-scheidgut__metal\">\
        <span class=\"eg-scheidgut__metal-dot\" data-metal=\"platinum\"></span>Platin</span></th>\
        <td class=\"eg-scheidgut__num\" data-sg-eur=\"48.779764\" data-sg-unit=\"g\">48,78 €/g</td>\
        <td class=\"eg-scheidgut__num eg-scheidgut__abschlag\" data-sg-eur=\"2\" data-sg-unit=\"g\" data-sg-neg=\"1\">−2,00 €/g</td>\
        <td class=\"eg-scheidgut__num eg-scheidgut__verg eg-scheidgut__verg-col\" data-sg-eur=\"46.779764\" data-sg-unit=\"g\">46,78 €/g</td>\
        <td class=\"eg-scheidgut__num\"><span class=\"eg-scheidgut__quote\">95,90 %</span></td></tr>\
        <tr><th scope=\"row\"><span class=\"eg-scheidgut__metal\">\
        <span class=\"eg-scheidgut__metal-dot\" data-metal=\"palladium\"></span>Palladium</span></th>\
        <td class=\"eg-scheidgut__num\" data-sg-eur=\"34.46483\" data-sg-unit=\"g\">34,46 €/g</td>\
        <td class=\"eg-scheidgut__num eg-scheidgut__abschlag\" data-sg-eur=\"2\" data-sg-unit=\"g\" data-sg-neg=\"1\">−2,00 €/g</td>\
        <td class=\"eg-scheidgut__num eg-scheidgut__verg eg-scheidgut__verg-col\" data-sg-eur=\"32.46483\" data-sg-unit=\"g\">32,46 €/g</td>\
        <td class=\"eg-scheidgut__num\"><span class=\"eg-scheidgut__quote\">94,20 %</span></td></tr>\
        </tbody></table><h2>Der Feinmetallgehalt entscheidet – ermittelt per Schmelzanalyse.</h2>";

    #[test]
    fn konditionen_rows_payouts_and_units() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 4);
        assert!(skips.is_empty(), "{skips:?}");
        // Machine Vergütung wins over rounded display text.
        assert_eq!(rows[0].0, "Gold");
        assert!((rows[0].1 - 116.206542).abs() < 1e-9);
        assert_eq!(rows[0].2, "EUR/g");
        assert!(rows[0].3.contains("116,71"), "{}", rows[0].3);
        assert!(rows[0].3.contains("−0,50"), "{}", rows[0].3);
        assert!(rows[0].3.contains("116,21"), "{}", rows[0].3);
        // Silver: per-kg quote divided by 1000 into the catalog unit.
        assert_eq!(rows[1].0, "Silber");
        assert!((rows[1].1 - 1.629749).abs() < 1e-9);
        assert_eq!(rows[1].2, "EUR/g");
        assert_eq!(rows[2].0, "Platin");
        assert!((rows[2].1 - 46.779764).abs() < 1e-9);
        assert_eq!(rows[2].2, "EUR/g");
        assert_eq!(rows[3].0, "Palladium");
        assert!((rows[3].1 - 32.46483).abs() < 1e-9);
        assert_eq!(rows[3].2, "EUR/g");
        // Shape guards: anchors and head words are mandatory.
        assert!(parse("<div>Redesign ohne Tabelle</div>").is_err());
        assert!(parse("Tagesaktuelle Konditionen ohne Ende").is_err());
        let no_head = FIXTURE.replace("Börsenkurs", "Kurs");
        assert!(parse(&no_head).is_err());
        assert_eq!(unit_of("g", ""), Some("EUR/g"));
        assert_eq!(unit_of("kg", ""), Some("EUR/kg"));
        assert_eq!(unit_of("", "1.630 €/kg"), Some("EUR/kg"));
        assert_eq!(unit_of("", "10 Euro pro Sack"), None);
        assert_eq!(price_of("116.206542"), Some(116.206542));
        assert_eq!(price_of(""), None);
        assert_eq!(price_of("116,21 €"), None);
        // Display-text fallback converts a per-kg quote into the catalog unit.
        let fallback = FIXTURE
            .replace("data-sg-eur=\"116.206542\" data-sg-unit=\"g\"", "data-sg-eur=\"\" data-sg-unit=\"\"");
        let (rows, _) = parse(&fallback).expect("parses");
        assert!((rows[0].1 - 116.21).abs() < 1e-9);
        assert_eq!(rows[0].2, "EUR/g");
        let fallback_kg = FIXTURE
            .replace("data-sg-eur=\"1.629749\" data-sg-unit=\"kg\"", "data-sg-eur=\"\" data-sg-unit=\"\"");
        let (rows, _) = parse(&fallback_kg).expect("parses");
        assert!((rows[1].1 - 1.63).abs() < 1e-9);
        assert_eq!(rows[1].2, "EUR/g");
    }

    #[test]
    fn zero_payout_skips_loudly() {
        // Calculator null ("0,00 €") is a missing quote, never a price.
        let zero = FIXTURE.replace(
            "data-sg-eur=\"32.46483\" data-sg-unit=\"g\">32,46 €/g",
            "data-sg-eur=\"0\" data-sg-unit=\"g\">0,00 €",
        );
        let (rows, skips) = parse(&zero).expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Palladium"), "{}", skips[0]);
        assert!(skips[0].contains("0,00"), "{}", skips[0]);
    }

    #[test]
    fn metals_map_to_catalog() {
        assert_eq!(grade_for("Gold"), Some(("gold", "")));
        assert_eq!(grade_for("Silber"), Some(("silber", "")));
        assert_eq!(grade_for("Platin"), Some(("platin", "")));
        assert_eq!(grade_for("Palladium"), Some(("palladium", "")));
        // No copper grade in the catalog for Scheidgut shares.
        assert_eq!(grade_for("Kupfer"), None);
        assert_eq!(grade_for("Gold + Silber Mix"), None);
        assert_eq!(grade_for(""), None);
    }

    #[test]
    fn cf_mail_decodes() {
        // Live data-cfemail from the impressum page.
        assert_eq!(
            cf_decode("5d2e282d2d322f291d383c2e243a3231396f69733e3230").as_deref(),
            Some("support@easygold24.com")
        );
        assert_eq!(cf_decode("zz"), None);
        assert_eq!(cf_decode("abc"), None);
    }

    #[test]
    fn impressum_verantwortlich_block() {
        // Real block shape of the live impressum page (entity, <br> lines,
        // tel: href, Cloudflare mail, USt-IdNr terminator).
        let imp = "<h5><strong>Verantwortlich i.S.d. Digitale-Dienste-Gesetz (DDG) und des § 18 Abs. 2 MStV:</strong></h5>\
            <p>Hartmann &amp; Benz GmbH<br>Gutenbergstraße 40<br>70736 Fellbach<br>Deutschland</p>\
            <p>Telefon:&nbsp; (+49) 711 967 431 65<br>E-Mail: \
            <a href=\"/cdn-cgi/l/email-protection\" class=\"__cf_email__\" \
            data-cfemail=\"5d2e282d2d322f291d383c2e243a3231396f69733e3230\">[email protected]</a></p>\
            <p>USt-IdNr.: DE325244140<br>eingetragen im Handelsregister des Amtsgerichtes Stuttgart</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Gutenbergstraße 40");
        assert_eq!(info.postcode, "70736");
        assert_eq!(info.city, "Fellbach");
        assert_eq!(info.phone, "(+49) 711 967 431 65");
        assert_eq!(info.email, "support@easygold24.com");
        assert!(extract_info("<div>ohne Verantwortlich-Block</div>").is_err());
        // Another firm's page must not pass as Hartmann-Benz.
        let other = imp.replace("Hartmann &amp; Benz GmbH", "Muster GmbH");
        assert!(extract_info(&other).is_err());
    }
}
