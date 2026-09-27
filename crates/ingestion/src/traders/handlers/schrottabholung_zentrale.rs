//! Schrottabholung Zentrale (Bochum-Gerthe): indicative per-kg purchase
//! rates from the "Buntmetall Rechner" on the price page. The rates live in
//! the calculator's inline `const preise = {...}` table (live: Kupfer 4.50,
//! Messing 3.00, Kabel 1.20, Blei/Zink/Aluminium/Edelstahl 0.80, Zinn 12.00,
//! Bleibatterien 0.30) with `<option>` labels joined on the same keys, and
//! the page computes "Preis pro kg" from them — so EUR/kg is belegt, not
//! guessed (a per-tonne reading would be absurd: 4.50 EUR/t copper).
//!
//! The page itself calls these numbers "lediglich Richtwerte" ("Die
//! tatsächlichen Ankaufspreise … können täglich schwanken"), so they are
//! recorded as `price_kind: "approx"` at confidence 0.5 — never `exact`.
//! If that caveat disappears the parse fails loudly instead of silently
//! upgrading the numbers to exact prices.
//!
//! Identity note (Verbund check): the page is self-consistent branding for
//! this trader — "Schrottabholung Zentrale", Dieselstraße 88, 44805 Bochum
//! (matches the seed city), mail `info@schrottabholung-zentrale.de` on its
//! own domain, phone 0152-59084206. The seed's Verbund suspicion pointed at
//! `info@schrott-zentrale.de` / schrotthaendler-dortmund.com, a different
//! domain that appears nowhere on this site.
//!
//! The only price source on this page is the calculator's inline rate
//! table, so `parse` windows that `<script>` block strictly between
//! "Buntmetall Rechner" and "Kreislaufwirtschaftsgesetz" and reads the
//! `const preise = {…};` literal — no whole-page script/text walk (which
//! would glue unrelated numbers onto labels).

use scraper::{ElementRef, Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "nw-bochum-gerthe-44805-schrottabholung-zentrale";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://schrottabholung-zentrale.de/impressum/";

pub const URL: &str =
    "https://schrottabholung-zentrale.de/faire-schrottpreise-bei-der-schrott-abholung-zentrale/";

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
    // Indicative rates only ("lediglich Richtwerte"): approx at 0.5.
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit,
                price_kind: "approx",
                price_min: None,
                price_max: None,
                confidence: Some(0.5),
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
/// skipped. "Kabel" cannot choose between `kabel-kupfer` and `kabel-alu`
/// (the page's own text lists copper, antenna, phone, earth cables), and
/// "Bleibatterien" is not the catalog's Weichblei (`blei`) — both skip
/// loudly with new-material proposals in the step detail.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Specific before generic: "Bleibatterien" contains "blei".
    if l.contains("bleibatterien") || l.contains("batterie") {
        None
    } else if l.contains("kabel") {
        None
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("zinn") {
        Some(("zinn", ""))
    } else if l.contains("alu") {
        Some(("aluminium-gemischt", ""))
    } else if l.contains("edelstahl") || l.contains("stahl") {
        // V2A/V4A split unknown → mixed grade.
        Some(("edelstahl-gemischt", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only (live markup):
/// `<h2>Angaben gemäß § 5 TMG</h2>` + `<p>` with name, glued street
/// ("Dieselstr.88"), "Gewerbegebiet" context line and "PLZ city", then
/// `<h2>Kontakt</h2>` + labeled Telefon/E-Mail `<p>`. Missing anchors →
/// loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let anchors: Vec<_> = doc.select(&h2).collect();
    let addr_h = anchors.iter().find(|h| {
        h.text()
            .collect::<String>()
            .contains("Angaben gemäß § 5 TMG")
    });
    let Some(addr_h) = addr_h else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    let addr_p = addr_h
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let mut lines = Vec::new();
    for part in addr_p.inner_html().split("<br") {
        let t = strip_fragment(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for line in &lines {
        // Tolerated page quirk (with test): glued "Dieselstr.88".
        let norm = line.replace("str.", "str. ").replace("  ", " ");
        if norm.contains("Dieselstr") && street.is_empty() {
            street = norm.split_whitespace().collect::<Vec<_>>().join(" ");
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        for (k, t) in toks.iter().enumerate() {
            if t.len() == 5 && t.chars().all(|c| c.is_ascii_digit()) {
                if let Some(ci) = toks.get(k + 1) {
                    if ci.chars().next().is_some_and(|c| c.is_uppercase()) {
                        postcode = (*t).to_owned();
                        city = (*ci).to_owned();
                        break;
                    }
                }
            }
        }
    }
    // Kontakt heading → next <p> holds the labeled lines.
    let mut phone = String::new();
    let mut email = String::new();
    let mut found_kontakt = false;
    for h in anchors {
        if h.text().collect::<String>().trim() == "Kontakt" {
            found_kontakt = true;
            let sib = h
                .next_siblings()
                .filter_map(ElementRef::wrap)
                .find(|e| e.value().name() == "p");
            if let Some(p) = sib {
                for part in p.inner_html().split("<br") {
                    let t = strip_fragment(part);
                    if let Some(v) = t.strip_prefix("Telefon:") {
                        phone = v
                            .split_whitespace()
                            .take_while(|tok| {
                                tok.chars()
                                    .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                            })
                            .collect::<Vec<_>>()
                            .join(" ");
                    } else if let Some(v) = t.strip_prefix("E-Mail:") {
                        // E-mail needs its own rule: the phone-style
                        // take_while above stops at the first letter.
                        email = v.split_whitespace().next().unwrap_or_default().to_owned();
                    }
                }
            }
        }
    }
    if !found_kontakt || (street.is_empty() && phone.is_empty() && email.is_empty()) {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
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

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant — drop everything up to the first '>' first, or the
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

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    // Window: the calculator lives between its heading and the KrWG
    // widget after it — the page's tag cloud and article prose must not
    // contribute labels or numbers.
    let start = html
        .find("Buntmetall Rechner")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Buntmetall-Rechner fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Kreislaufwirtschaftsgesetz")
        .unwrap_or(tail.len());
    let window = &tail[..end];
    // The approx character is load-bearing for our uncertainty model: if
    // the page stops calling these numbers Richtwerte, fail loudly
    // instead of silently keeping approx (or worse, implying exact).
    if !window.contains("lediglich Richtwerte") {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Richtwert-Hinweis fehlt".to_owned(),
        });
    }
    // Unit proof for THIS page: the calculator renders "Preis pro kg".
    // Anything else (or nothing) skips loudly at the call site — a
    // per-tonne price recorded as per-kg would be a 1000x error.
    let unit_source = if window.contains("Preis pro kg") {
        "Preis pro kg"
    } else {
        ""
    };
    let Some(unit) = unit_of(unit_source) else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Einheit unbelegt".to_owned(),
        });
    };
    // Display labels from the real <option> elements, keyed by value.
    let mut options = Vec::new();
    let mut rest = window;
    while let Some(i) = rest.find("<option value=\"") {
        let after = &rest[i + "<option value=\"".len()..];
        let (Some(q), Some(gt), Some(lt)) =
            (after.find('"'), after.find('>'), after.find("</option>"))
        else {
            break;
        };
        if q < gt && gt < lt {
            options.push((after[..q].to_owned(), after[gt + 1..lt].trim().to_owned()));
        }
        rest = &after[lt..];
    }
    if options.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Rechner-Optionen fehlen".to_owned(),
        });
    }
    // Rates from the calculator's inline table literal.
    const TABLE_OPEN: &str = "const preise = {";
    let table = window.find(TABLE_OPEN).ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preis-Tabelle fehlt".to_owned(),
    })?;
    let table_tail = &window[table + TABLE_OPEN.len()..];
    let table_end = table_tail.find("};").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preis-Tabelle fehlt".to_owned(),
    })?;
    let literal = &table_tail[..table_end];
    let mut rates = std::collections::HashMap::new();
    for part in literal.split(',') {
        let mut kv = part.splitn(2, ':');
        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
            let key = k.trim().trim_start_matches('{').trim().to_lowercase();
            // JS dot-decimals only ("4.50"), parsed bespoke: the shared
            // German `parse_eur` would read "4.50" as 450 (Tausenderpunkt).
            if let Some(price) = parse_js_decimal(v.trim()) {
                rates.insert(key, price);
            }
        }
    }
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for (key, label) in options {
        match rates.get(&key) {
            Some(price) => rows.push((label, *price, unit)),
            None => skips.push(format!("{label} (kein Richtwert hinterlegt)")),
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Rechner-Tabelle leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS calculator: it renders "Preis pro kg"
/// and takes "Gewicht in kg" — per-kg only. Anything else (per tonne,
/// per piece, per sack, or a missing unit proof) skips loudly.
fn unit_of(phrase: &str) -> Option<&'static str> {
    let lower = phrase.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

/// Bespoke decimal parser for the calculator's JS literal (`4.50`,
/// `12.00`, `0.30`). Dot-decimals only — a German-comma value means the
/// page changed notation and must not parse silently.
fn parse_js_decimal(s: &str) -> Option<f64> {
    let s = s.trim();
    let (int, frac) = s.split_once('.')?;
    if int.is_empty()
        || frac.is_empty()
        || !int.bytes().all(|b| b.is_ascii_digit())
        || !frac.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    s.parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use super::{grade_for, parse, parse_js_decimal, unit_of};

    // Real live markup (calculator window, function body trimmed but
    // verbatim — the unit proof line is inside berechnePreis).
    const FIXTURE: &str = "<h3>Buntmetall Rechner</h3>\
        <label for=\"metall\">Wähle ein Metall:</label>\
        <select id=\"metall\">\
        <option value=\"kupfer\">Kupfer</option>\
        <option value=\"messing\">Messing</option>\
        <option value=\"kabel\">Kabel</option>\
        <option value=\"blei\">Blei</option>\
        <option value=\"zink\">Zink</option>\
        <option value=\"zinn\">Zinn</option>\
        <option value=\"aluminium\">Aluminium</option>\
        <option value=\"edelstahl\">Edelstahl</option>\
        <option value=\"bleibatterien\">Bleibatterien</option>\
        </select>\
        <label for=\"gewicht\">Gewicht in kg:</label>\
        <input type=\"number\" id=\"gewicht\" step=\"0.01\" min=\"0\">\
        <button onclick=\"berechnePreis()\">Berechnen</button>\
        <div id=\"ergebnis\"></div>\
        <div class=\"hinweis\">\
        <p><strong>Hinweis:</strong> Die hier berechneten Preise sind lediglich Richtwerte. \
        Die tatsächlichen Ankaufspreise für Buntmetall und Schrott können täglich schwanken.</p>\
        </div>\
        <script>\
        const preise = {\
        kupfer: 4.50,\
        messing: 3.00,\
        kabel: 1.20,\
        blei: 0.80,\
        zink: 0.80,\
        zinn: 12.00,\
        aluminium: 0.80,\
        edelstahl: 0.80,\
        bleibatterien: 0.30\
        };\
        function berechnePreis() {\
        const metall = document.getElementById(\"metall\").value;\
        const preisProKg = preise[metall];\
        `<p>Preis pro kg: <strong>${preisProKg.toFixed(2)} €</strong></p>`;\
        }\
        </script>\
        <p>Gesetz zur Förderung der Kreislaufwirtschaft und Sicherung</p>";

    #[test]
    fn rechner_table_parses() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 9);
        assert!(skips.is_empty());
        assert_eq!(rows[0].0, "Kupfer");
        assert_eq!(rows[0].1, 4.5);
        assert_eq!(rows[0].2, "EUR/kg");
        assert_eq!(rows[5].0, "Zinn");
        assert_eq!(rows[5].1, 12.0);
        assert_eq!(rows[8].0, "Bleibatterien");
        assert_eq!(rows[8].1, 0.3);
    }

    #[test]
    fn changed_page_character_fails_loudly() {
        // Caveat gone → no silent approx.
        let html = FIXTURE.replace("lediglich Richtwerte", "verbindliche Festpreise");
        assert!(parse(&html).is_err());
        // Unit proof gone → loud error, not a silent default.
        let html = FIXTURE.replace("Preis pro kg", "Preis pro Sack");
        assert!(parse(&html).is_err());
        // Calculator heading gone → loud error.
        let html = FIXTURE.replacen("Buntmetall Rechner", "Altmetall Rechner", 1);
        assert!(parse(&html).is_err());
        // Rate table emptied → loud error, not silent success.
        let html = FIXTURE
            .replace("kupfer: 4.50,", "")
            .replace("messing: 3.00,", "")
            .replace("kabel: 1.20,", "")
            .replace("blei: 0.80,", "")
            .replace("zink: 0.80,", "")
            .replace("zinn: 12.00,", "")
            .replace("aluminium: 0.80,", "")
            .replace("edelstahl: 0.80,", "")
            .replace("bleibatterien: 0.30", "");
        assert!(parse(&html).is_err());
    }

    #[test]
    fn js_decimals_not_german_thousands() {
        assert_eq!(parse_js_decimal("4.50"), Some(4.5));
        assert_eq!(parse_js_decimal("12.00"), Some(12.0));
        assert_eq!(parse_js_decimal("0.30"), Some(0.3));
        assert_eq!(parse_js_decimal("4,50"), None);
        assert_eq!(parse_js_decimal("1.100,50"), None);
        assert_eq!(parse_js_decimal("frei"), None);
    }

    #[test]
    fn unit_only_knows_this_pages_kg() {
        assert_eq!(unit_of("Preis pro kg"), Some("EUR/kg"));
        assert_eq!(unit_of("Gewicht in KG"), Some("EUR/kg"));
        assert_eq!(unit_of(""), None);
        assert_eq!(unit_of("pro Sack"), None);
        assert_eq!(unit_of("pro t"), None);
    }

    #[test]
    fn mapping_skips_unattributable() {
        assert_eq!(grade_for("Kupfer"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Zinn"), Some(("zinn", "")));
        assert_eq!(grade_for("Aluminium"), Some(("aluminium-gemischt", "")));
        assert_eq!(grade_for("Edelstahl"), Some(("edelstahl-gemischt", "")));
        // Specific before generic: batteries are not Weichblei.
        assert_eq!(grade_for("Bleibatterien"), None);
        // Cu/Al share unknown.
        assert_eq!(grade_for("Kabel"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h2><span id=\"Angaben_gemaess\"></span>Angaben gemäß § 5 TMG</h2>\
            <p>Rameh Schakif<br>(Einzelunternehmer)<br>Dieselstr.88<br>Gewerbegebiet<br>44805 Bochum</p>\
            <h2><span id=\"Kontakt\"></span>Kontakt</h2>\
            <p>Telefon: +49 (0) 15259084206<br>Telefax: +49 (0) 234 95294917<br>\
            E-Mail: info@schrottabholung-zentrale.de</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Dieselstr. 88");
        assert_eq!(info.postcode, "44805");
        assert_eq!(info.city, "Bochum");
        assert_eq!(info.phone, "+49 (0) 15259084206");
        assert_eq!(info.email, "info@schrottabholung-zentrale.de");
        // Redesign without anchors fails loudly.
        assert!(super::extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
