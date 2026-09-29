//! GOLDhandelshaus Bremen (Edelmetall-Ankauf, Sögestraße 1):
//! statische Ankauf-Kurse in EUR/g auf dem Altgoldrechner.
//!
//! Preisquelle (`PRICE_URL`): die Sidebar "Aktuelle Ankauf-Kurse" mit
//! `p.small` ("Kurs vom 28. September 2026 um 19:42 Uhr.") plus sieben
//! `div.kurs-row`-Zeilen — drei Barren ("999.9 Gold = € 111,87 / g",
//! "999.9 Silber = € 1.400,00 / kg", "999.9 Platin = € 47,31 / g") und
//! vier Goldlegierungen ("900/000", "750/000", "585/000", "333/000" je
//! €/g). Der Feingehalt fährt in der Variante (`gold`/`900` …,
//! `silber`/`999`, `platin`/`999`); der Silber-Kurs steht in €/kg und
//! wird durch 1000 auf die Katalogeinheit EUR/g normiert — belegt durch
//! die seilteneigene "/ kg"-Angabe derselben Zeile (reine
//! Metrik-Umrechnung, kein Raten). Der Rechner-Ergebnisbetrag
//! (`#kursrechner-ergebnis`, Default "0,00") ist JS-only und liegt
//! außerhalb des Fensters → WALLED. `0,00 €`-Kurszeilen (kein Ankauf)
//! skippen laut. Null Zeilen → `Err` (Redesign darf nie wie Erfolg
//! aussehen). Das Seitendatum → Outcome-`published_at`; ohne Datum →
//! `None`.
//!
//! Kontakt (`INFO_URL`, live-verifiziert 28.09.2026): Filialseite
//! `/unsere-standorte/goldankauf-bremen/` mit `div.contact-widget`
//! ("Adresse: Sögestraße 1, 28195 Bremen", "Telefon" mit
//! `tel:`-Link). Kein `mailto:` auf der Seite → E-Mail bleibt leer
//! (normal, kein Fehler). Fehlender Widget-Anker oder falsche Straße →
//! lauter Fehler, nie raten.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "hb-bremen-28195-goldhandelshaus-bremen";

/// Altgoldrechner mit der statischen Kurs-Sidebar, live-verifiziert
/// 28.09.2026 (sieben Kurszeilen + "Kurs vom …"). Ein Umzug lässt den
/// Step laut scheitern.
pub const PRICE_URL: &str = "https://goldhandelshaus.de/altgoldrechner/";
/// Bespoke, live-verifizierte Filial-Kontaktseite (eigene URL + eigener
/// Block, kein Crawler).
pub const INFO_URL: &str = "https://goldhandelshaus.de/unsere-standorte/goldankauf-bremen/";
/// Straßen-Anker: schützt davor, dass das Shared-Template still eine
/// andere Filiale ausliefert.
const STREET_GUARD: &str = "Sögestraße 1";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: PRICE_URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, PRICE_URL).await?;
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
            // Preis steht, aber kein Katalogmaterial: als Beleg im Skip,
            // nie still fallenlassen, nie in ein falsches Material pressen.
            None => skipped_labels.push(format!(
                "{label} ({}, {unit}, kein Katalogmaterial: Edelmetall)",
                fmt_eur(price)
            )),
        }
    }
    // Kontaktseiten-Fehler scheitern laut per Design: eine gezogene
    // Filialseite braucht Augen, bevor ihr wieder vertraut wird.
    let (_, info_html) = fetch_text(client, INFO_URL).await?;
    let trader_info = extract_info(&info_html)?;
    Ok(HandlerOutcome {
        prices,
        acceptances: vec![],
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: PRICE_URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: published_at_of(&html),
    })
}

fn fmt_eur(price: f64) -> String {
    format!("{price:.2}").replace('.', ",")
}

/// Explizites Label → (Material, Variante). Der Feingehalt fährt in der
/// Variante (Metallwort + Gehalt müssen beide stimmen); ohne Metallwort
/// oder ohne belegten Gehalt → `None` (lauter Skip). Silber/Platin vor
/// Gold: "999.9 …" fängt sonst alles.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    if l.contains("silber") {
        if l.contains("999") {
            return Some(("silber", "999"));
        }
        return None;
    }
    if l.contains("platin") && !l.contains("palladium") {
        if l.contains("999") {
            return Some(("platin", "999"));
        }
        return None;
    }
    if l.contains("palladium") {
        return None;
    }
    if l.contains("gold") {
        if l.contains("999") {
            return Some(("gold", "999"));
        }
        if l.contains("900") {
            return Some(("gold", "900"));
        }
        if l.contains("750") {
            return Some(("gold", "750"));
        }
        if l.contains("585") {
            return Some(("gold", "585"));
        }
        if l.contains("333") {
            return Some(("gold", "333"));
        }
        return None;
    }
    // Gehalt ohne Metallwort ("900/000") = Goldlegierung: Die Seite
    // gruppiert diese Zeilen unter "Goldlegierungen" — der Beleg steht
    // im Fenster, nicht im Label. Enthält die Zeile nur einen der
    // belegten Goldgehalte, gilt sie als Goldlegierung.
    for (fin, variant) in [
        ("900", "900"),
        ("750", "750"),
        ("585", "585"),
        ("333", "333"),
    ] {
        if l.contains(fin) {
            return Some(("gold", variant));
        }
    }
    None
}

/// Sieben Kurszeilen im Fenster zwischen der Listenüberschrift und dem
/// Button-Block. Gibt (Zeilen, Skips) mit (Label, Preis, Einheit);
/// `0,00 €` (kein Ankauf) skippt laut; `€/kg` wird belegt (/1000) auf
/// die Katalogeinheit EUR/g normiert. Null Zeilen → `Err`.
fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let missing = |detail: &str| IngestError::Parse {
        url: PRICE_URL.to_owned(),
        detail: detail.to_owned(),
    };
    let start = html
        .find("Aktuelle Ankauf-Kurse")
        .ok_or_else(|| missing("Ankauf-Kurse: Listen-Anker fehlt"))?;
    let tail = &html[start..];
    let end = tail
        .find("button-wrapper")
        .ok_or_else(|| missing("Ankauf-Kurse: Ende-Anker fehlt"))?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let row_sel = Selector::parse("div.kurs-row").expect("valid selector");
    let label_sel = Selector::parse("span.label").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for row in frag.select(&row_sel) {
        let label = row
            .select(&label_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if label.is_empty() || label.len() > 120 {
            skips.push("(Kurszeile ohne Label)".to_owned());
            continue;
        }
        let full: String = row.text().collect();
        let rest = full.replacen(&label, "", 1);
        let Some(price) = parse_eur(&rest) else {
            skips.push(format!("{label} (Preis unverständlich: {})", rest.trim()));
            continue;
        };
        if !price.is_finite() || price <= 0.0 {
            skips.push(format!("{label} ({}: kein Ankaufspreis)", fmt_eur(price)));
            continue;
        }
        // Laut skippen statt Default: ein Kilo-Preis als Gramm verbucht
        // wäre ein 1000×-Fehler.
        let Some(quoted) = unit_of(&rest) else {
            skips.push(format!("{label} (Einheit unverständlich: {})", rest.trim()));
            continue;
        };
        // Belegte Normierung auf die Katalogeinheit EUR/g: Die Zeile
        // selbst nennt "/ kg" — reine Metrik-Umrechnung (/1000), kein
        // Raten. Alles andere als €/g oder €/kg skippt oben bereits.
        let (price, unit) = if quoted == "EUR/kg" {
            (price / 1000.0, "EUR/g")
        } else {
            (price, quoted)
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(missing("Ankauf-Kurse: keine Kurszeilen"));
    }
    Ok((rows, skips))
}

/// Bespoke Unit-Matcher DIESER Kurszeilen (live: "€ 111,87 / g",
/// "€ 1.400,00 / kg"). Nur g/kg existieren hier — alles andere skippt
/// laut am Call-Site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("/ kg") || lower.contains("/kg") {
        Some("EUR/kg")
    } else if lower.contains("/ g") || lower.contains("/g") {
        Some("EUR/g")
    } else {
        None
    }
}

/// Seitendatum aus `<p class="small">Kurs vom 28. September 2026 um
/// …</p>` → RFC 3339 (Mitternacht UTC) via `parse_de_date`. Kein Datum
/// → `None` (Alter = `observed_at`); ein Datum scheitert hier nie den
/// Step.
fn published_at_of(html: &str) -> Option<String> {
    let at = html.find("Kurs vom")?;
    let tail = &html[at + "Kurs vom".len()..];
    let mut it = tail.split_whitespace();
    let day = it.next()?.trim_end_matches('.');
    let month = month_de(it.next()?)?;
    let year = it.next()?.trim_end_matches(|c: char| !c.is_ascii_digit());
    parse_de_date(day, month, year)
}

/// Deutscher Monatsname → zweistellige Monatsnummer für `parse_de_date`.
fn month_de(name: &str) -> Option<&'static str> {
    match name {
        "Januar" => Some("01"),
        "Februar" => Some("02"),
        "März" | "Maerz" => Some("03"),
        "April" => Some("04"),
        "Mai" => Some("05"),
        "Juni" => Some("06"),
        "Juli" => Some("07"),
        "August" => Some("08"),
        "September" => Some("09"),
        "Oktober" => Some("10"),
        "November" => Some("11"),
        "Dezember" => Some("12"),
        _ => None,
    }
}

/// Bespoke Kontakt-Extraktion NUR für diese Filialseiten-Bauart:
/// `div.contact-widget` mit `contact-widget-single`-Kacheln
/// ("Adresse: Sögestraße 1 / 28195 Bremen", "Telefon" mit `tel:`-Link,
/// Anzeige-Text gewinnt). `<br`-Zeilen werden aus dem Kachel-HTML
/// gelesen (scraper-`text()` klebte sonst "Adresse"+"Straße"+"PLZ"
/// zusammen). Fehlender Widget-Anker oder falsche Straße → lauter
/// Fehler, nie geraten. Kein `mailto:` auf der Seite → E-Mail bleibt
/// leer (normal, kein Fehler).
fn extract_info(html: &str) -> Result<TraderInfo, IngestError> {
    let missing = |detail: &str| IngestError::Parse {
        url: INFO_URL.to_owned(),
        detail: detail.to_owned(),
    };
    let w_start = html
        .find("contact-widget")
        .ok_or_else(|| missing("Kontakt-Widget fehlt"))?;
    let w_tail = &html[w_start..];
    // Nächste Sektion ("img-widget") beendet das Widget-Fenster.
    let w_end = w_tail.find("img-widget").unwrap_or(w_tail.len());
    let window = &w_tail[..w_end];
    if !window.contains(STREET_GUARD) {
        return Err(missing(&format!("falsche Filiale (kein {STREET_GUARD})")));
    }
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let tile_sel = Selector::parse("div.contact-widget-single").expect("valid selector");
    let p_sel = Selector::parse("p").expect("valid selector");
    let strong_sel = Selector::parse("strong").expect("valid selector");
    let (mut street, mut postcode, mut city, mut phone) =
        (String::new(), String::new(), String::new(), String::new());
    for tile in frag.select(&tile_sel) {
        // Kachel-Sorte aus dem <p>-Kopf ("Adresse"/"Telefon"/…) — das
        // <img>-alt ist unzuverlässig (Copy-Paste auf der Seite).
        let p = tile.select(&p_sel).next();
        let head = p
            .as_ref()
            .and_then(|p| p.select(&strong_sel).next())
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();
        if head.contains("Adresse") {
            let inner = p.map(|p| p.inner_html()).unwrap_or_default();
            // `<br`-Zeilen lesen: alles bis zum ersten `>` pro Segment
            // verwerfen (Tag-Reste parsen sonst als Text).
            let lines: Vec<String> = inner
                .replace("<br/>", "\n")
                .replace("<br />", "\n")
                .replace("<br>", "\n")
                .split('\n')
                .map(|l| {
                    l.split('>')
                        .next_back()
                        .unwrap_or("")
                        .split('<')
                        .next()
                        .unwrap_or("")
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .filter(|l| !l.is_empty() && l != "Adresse")
                .collect();
            for l in lines {
                if l.contains(STREET_GUARD) {
                    street = l.clone();
                    continue;
                }
                let mut it = l.split_whitespace();
                let Some(pc) = it.next() else { continue };
                if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    city = it.collect::<Vec<_>>().join(" ");
                }
            }
        } else if head.contains("Telefon") {
            let a_sel = Selector::parse("a[href^=\"tel:\"]").expect("valid selector");
            phone = tile
                .select(&a_sel)
                .next()
                .map(|el| {
                    el.text()
                        .collect::<String>()
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .unwrap_or_default();
        }
    }
    if street.is_empty() && phone.is_empty() {
        return Err(missing("keine Kontaktdaten gefunden"));
    }
    Ok(TraderInfo {
        street,
        postcode,
        city,
        phone,
        email: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, month_de, parse, published_at_of, unit_of};

    // Realer Live-Ausschnitt der Altgoldrechner-Seite (Sidebar-Bauart
    // p.small + kurs-container + button-wrapper-Terminator), Kurse =
    // Stand 28.09.2026 19:42 Uhr.
    const FIXTURE: &str = "<h3>Aktuelle Ankauf-Kurse</h3><div class=\"textwidget\"><div class=\"sidebar-content\">\
        <p class=\"small\">Kurs vom 28. September 2026 um <span class=\"nowrap\">19:42 Uhr</span>.</p>\
        <p><h5>Barren</h5><div class=\"kurs-container\">\
        <div class=\"kurs-row\"><span class=\"label\">999.9 Gold</span><span> = </span> € 111,87 / g</div>\
        <div class=\"kurs-row\"><span class=\"label\">999.9 Silber</span><span> = </span> € 1.400,00 / kg</div>\
        <div class=\"kurs-row\"><span class=\"label\">999.9 Platin</span><span> = </span> € 47,31 / g</div></div>\
        <br /><h5>Goldlegierungen</h5><div class=\"kurs-container\">\
        <div class=\"kurs-row\"><span class=\"label\">900/000</span><span> = </span> € 91,69 / g</div>\
        <div class=\"kurs-row\"><span class=\"label\">750/000</span><span> = </span> € 76,41 / g</div>\
        <div class=\"kurs-row\"><span class=\"label\">585/000</span><span> = </span> € 59,59 / g</div>\
        <div class=\"kurs-row\"><span class=\"label\">333/000</span><span> = </span> € 33,92 / g</div></div>\
        <br /><div class=\"button-wrapper center no-margin-bottom\">";

    #[test]
    fn seven_rows_become_prices() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty());
        assert_eq!(rows.len(), 7);
        assert_eq!(rows[0], ("999.9 Gold".to_owned(), 111.87, "EUR/g"));
        // €/kg → €/g ist belegt durch die Zeile selbst (/1000).
        assert_eq!(rows[1], ("999.9 Silber".to_owned(), 1.4, "EUR/g"));
        assert_eq!(rows[2], ("999.9 Platin".to_owned(), 47.31, "EUR/g"));
        assert_eq!(rows[3], ("900/000".to_owned(), 91.69, "EUR/g"));
        assert_eq!(rows[4], ("750/000".to_owned(), 76.41, "EUR/g"));
        assert_eq!(rows[5], ("585/000".to_owned(), 59.59, "EUR/g"));
        assert_eq!(rows[6], ("333/000".to_owned(), 33.92, "EUR/g"));
        assert_eq!(unit_of("€ 111,87 / g"), Some("EUR/g"));
        assert_eq!(unit_of("€ 1.400,00 / kg"), Some("EUR/kg"));
        assert_eq!(unit_of("pro Sack"), None);
        assert!(parse("<div>Redesign ohne Liste</div>").is_err());
        assert!(parse("Aktuelle Ankauf-Kurse ohne Zeilen").is_err());
    }

    #[test]
    fn zero_price_skips_loudly() {
        let html = FIXTURE.replace("€ 111,87 / g", "€ 0,00 / g");
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 6);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("999.9 Gold"));
        assert!(skips[0].contains("kein Ankaufspreis"));
    }

    #[test]
    fn fineness_rides_in_variant() {
        assert_eq!(grade_for("999.9 Gold"), Some(("gold", "999")));
        assert_eq!(grade_for("900/000"), Some(("gold", "900")));
        assert_eq!(grade_for("750/000"), Some(("gold", "750")));
        assert_eq!(grade_for("585/000"), Some(("gold", "585")));
        assert_eq!(grade_for("333/000"), Some(("gold", "333")));
        assert_eq!(grade_for("999.9 Silber"), Some(("silber", "999")));
        assert_eq!(grade_for("999.9 Platin"), Some(("platin", "999")));
        // Metallose Fremdgehalte und Palladium ohne Zeile: kein Raten.
        assert_eq!(grade_for("925 Silber"), None);
        assert_eq!(grade_for("999.9 Palladium"), None);
        assert_eq!(grade_for("Altgold-Ankauf"), None);
        assert_eq!(month_de("September"), Some("09"));
        assert_eq!(month_de("Unfug"), None);
    }

    #[test]
    fn kurs_date_becomes_published_at() {
        assert_eq!(
            published_at_of(FIXTURE),
            Some("2026-09-28T00:00:00+00:00".to_owned())
        );
        assert_eq!(published_at_of("<div>ohne Datum</div>"), None);
    }

    #[test]
    fn bremen_contact_block() {
        // Realer Ausschnitt der Live-Filialseite goldankauf-bremen
        // (contact-widget-Bauart, kein mailto auf der Seite).
        let imp = "<div class=\"row contact-widget\">\
            <div class=\"col-xs-6 col-md-4 contact-widget-single\">\
            <img src=\"icon-map.png\" alt=\"Adresse\" />\
            <p><strong>Adresse</strong><br/>Sögestraße 1<br/>28195 Bremen</p></div>\
            <div class=\"col-xs-6 col-md-4 contact-widget-single\">\
            <img src=\"icon-phone.png\" alt=\"Telefon\" />\
            <p><strong>Telefon</strong><br/>\
            <a href=\"tel:0421/38028024\">0421 / 38028024</a></p></div>\
            <div class=\"col-xs-6 col-md-4 contact-widget-single\">\
            <p><strong>Ankaufszeiten</strong><br/>Montag bis Samstag<br/>11.00 - 19.00 Uhr</p></div>\
            </div><div class=\"row img-widget\">";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Sögestraße 1");
        assert_eq!(info.postcode, "28195");
        assert_eq!(info.city, "Bremen");
        assert_eq!(info.phone, "0421 / 38028024");
        assert_eq!(info.email, "");
        assert!(extract_info("<div>ohne Widget</div>").is_err());
        // Fremde Filiale scheitert am Straßen-Anker.
        let other = imp.replace("Sögestraße 1", "Hohe Straße 1");
        assert!(extract_info(&other).is_err());
    }
}
