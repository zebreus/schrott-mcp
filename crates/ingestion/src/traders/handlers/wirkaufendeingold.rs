//! WirKaufenDeinGold.de (Rohat Erdem, Bremen-Findorff, Edelmetall-Ankauf):
//! zwei statische Tagespreise in EUR/g auf der Homepage.
//!
//! Preisquelle (`PRICE_URL`): die Homepage trägt drei server-renderte
//! Tagespreis-Elemente — `span#calc-spot` ("122,00 €/g", Label
//! `span#calc-spot-lbl` "Tageskurs Feingold"), `p.bars-note` ("… Gold
//! 122,00 €/g, Silber 1,80 €/g …") und `p#hero-gold-price` ("3.563,90 €",
//! `p#hero-metal-sub` "pro Unze · 999er Feingold"). Daraus: `gold`/`999`
//! = Feingold-Tageskurs, `silber`/`999` = Silber-Tagespreis (Feinsilber-
//! Basis, einziger Silberpreis der Seite; Hauskonvention wie Degussa
//! "Silber Feinsilber 999"). Der Unzen-Hero (`pro Unze`) wird laut
//! geskippt: Troy-Unze → Gramm wäre eine unbelegte Umrechnung (keine
//! kg↔t-Normierung, Oz→g ist nicht belegt). Der Rechner-Ergebnisbetrag
//! (`#calc-result-low`, Auszahlungs-Schätzung nach Payout-Faktor),
//! die Barren-Produktpreise (`#bars-grid`, JS-gefüllt) und die
//! Detailseite `/goldpreis` (kein einziges statisches €) sind JS-only →
//! WALLED, keine Fakes. Die Seite nennt kein Stand-Datum (nur "alle 60
//! Sekunden aktualisiert") → `published_at = None`.
//!
//! Kontakt (`INFO_URL`, live-verifiziert 28.09.2026): Impressum mit
//! `<address>` ("Rohat Erdem … Admiralstraße 111, 28215
//! Bremen-Findorff") plus `tel:`-/`mailto:`-Links. Fehlt der
//! Diensteanbieter-Anker oder die Straße → lauter Fehler, nie raten.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "hb-bremen-findorff-28215-wirkaufendeingold-de-rohat-erdem";

/// Homepage mit den statischen Tagespreis-Elementen, live-verifiziert
/// 28.09.2026 (`#calc-spot` 122,00 €/g, `#bars-spot-silver` 1,80 €/g).
/// Ein Umzug lässt den Step laut scheitern.
pub const PRICE_URL: &str = "https://wirkaufendeingold.de";
/// Bespoke, live-verifizierte Impressum-Seite (eigene URL + eigener
/// Block, kein Crawler).
pub const INFO_URL: &str = "https://wirkaufendeingold.de/impressum";
/// Straßen-Anker: schützt davor, dass die Seite still ein anderes
/// Unternehmen ausliefert.
const STREET_GUARD: &str = "Admiralstraße 111";

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
    for (material, variant, label, price, unit) in rows {
        prices.push(ScrapedPrice {
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
        });
    }
    // Kontaktseiten-Fehler scheitern laut per Design: ein gezogenes
    // Impressum braucht Augen, bevor ihm wieder vertraut wird.
    let (_, info_html) = fetch_text(client, INFO_URL).await?;
    let trader_info = extract_info(&info_html)?;
    // Der Unzen-Hero ist ein sichtbarer Preis ohne Katalogeinheit —
    // laut skippen, nie umrechnen, nie still fallenlassen.
    skipped_labels.extend(hero_unze_skip(&html));
    Ok(HandlerOutcome {
        prices,
        acceptances: vec![],
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: PRICE_URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

/// Die zwei statischen Tagespreise der Homepage als
/// (Material, Variante, Label, Preis, Einheit). Der Gold-Tageskurs ist
/// Pflicht (`Err` bei Redesign); fehlt der Silber-Hinweis, skippt er
/// laut. `0,00 €` (kein Kurs) skippt laut. Null Zeilen → `Err`.
fn parse(
    html: &str,
) -> Result<
    (
        Vec<(&'static str, &'static str, String, f64, &'static str)>,
        Vec<String>,
    ),
    IngestError,
> {
    let missing = |detail: &str| IngestError::Parse {
        url: PRICE_URL.to_owned(),
        detail: detail.to_owned(),
    };
    let doc = Html::parse_document(html);
    let sel = |s: &str| Selector::parse(s).expect("valid selector");
    let text_of = |css: &str| {
        doc.select(&sel(css))
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    // Gold: "Tageskurs Feingold" + "122,00 €/g" — zwei Anker, ein Wert.
    let gold_lbl = text_of("span#calc-spot-lbl");
    if !gold_lbl.contains("Feingold") {
        return Err(missing("Tageskurs-Label (Feingold) fehlt"));
    }
    let gold_txt = text_of("span#calc-spot");
    if gold_txt.is_empty() {
        return Err(missing("Tageskurs-Element (#calc-spot) fehlt"));
    }
    match parse_eur(&gold_txt) {
        Some(p) if p.is_finite() && p > 0.0 => match unit_of(&gold_txt) {
            // Laut skippen statt Default: ein Kilo-Preis als Gramm
            // verbucht wäre ein 1000×-Fehler.
            Some(u) => rows.push(("gold", "999", "Tageskurs Feingold".to_owned(), p, u)),
            None => skips.push(format!(
                "Tageskurs Feingold (Einheit unverständlich: {gold_txt})"
            )),
        },
        Some(p) => skips.push(format!(
            "Tageskurs Feingold ({}: kein Ankaufspreis)",
            fmt_eur(p)
        )),
        None => skips.push(format!(
            "Tageskurs Feingold (Preis unverständlich: {gold_txt})"
        )),
    }
    // Silber: "… Silber 1,80 €/g …" in p.bars-note — Zahl steht NACH
    // dem Metallwort, also erst ab "Silber" lesen (parse_eur nähme
    // sonst den Goldwert davor).
    let note = text_of("p.bars-note");
    match note.split_once("Silber") {
        Some((_, tail)) => match parse_eur(tail) {
            Some(p) if p.is_finite() && p > 0.0 => match unit_of(tail) {
                Some(u) => rows.push(("silber", "999", "Silber-Tagespreis".to_owned(), p, u)),
                None => skips.push(format!(
                    "Silber-Tagespreis (Einheit unverständlich: {})",
                    tail.trim()
                )),
            },
            Some(p) => skips.push(format!(
                "Silber-Tagespreis ({}: kein Ankaufspreis)",
                fmt_eur(p)
            )),
            None => skips.push(format!(
                "Silber-Tagespreis (Preis unverständlich: {})",
                tail.trim()
            )),
        },
        None => skips.push("(Silber-Tagespreis: Hinweis fehlt)".to_owned()),
    }
    if rows.is_empty() {
        return Err(missing("keine Tagespreise gefunden"));
    }
    Ok((rows, skips))
}

/// Der Hero-Unzenpreis als lauter Skip (sichtbar, aber keine
/// Katalogeinheit). Fehlt er, gibt es nichts zu dokumentieren.
fn hero_unze_skip(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let sel = |s: &str| Selector::parse(s).expect("valid selector");
    let text_of = |css: &str| {
        doc.select(&sel(css))
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };
    let amount = text_of("p#hero-gold-price");
    let sub = text_of("p#hero-metal-sub");
    if amount.is_empty() || !sub.to_lowercase().contains("unze") {
        return vec![];
    }
    vec![format!(
        "Tagespreis Gold {amount} {sub} (Einheit Unze: keine Katalogeinheit, keine Umrechnung)"
    )]
}

/// Bespoke Unit-Matcher DIESER Elemente (live: "122,00 €/g").
/// Nur g existiert hier — alles andere skippt laut am Call-Site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("/g") || lower.contains("pro gramm") {
        Some("EUR/g")
    } else {
        None
    }
}

fn fmt_eur(price: f64) -> String {
    format!("{price:.2}").replace('.', ",")
}

/// Bespoke Kontakt-Extraktion NUR für diese Impressum-Bauart:
/// `<address>` ("Rohat Erdem … Admiralstraße 111 / 28215
/// Bremen-Findorff") plus `tel:`-/`mailto:`-Links (Anzeige-Text
/// gewinnt). Fehlender Diensteanbieter-Anker oder falsche Straße →
/// lauter Fehler, nie geraten.
fn extract_info(html: &str) -> Result<TraderInfo, IngestError> {
    let missing = |detail: &str| IngestError::Parse {
        url: INFO_URL.to_owned(),
        detail: detail.to_owned(),
    };
    if !html.contains("Diensteanbieter") {
        return Err(missing("Diensteanbieter-Anker fehlt"));
    }
    if !html.contains(STREET_GUARD) {
        return Err(missing(&format!("falsche Seite (kein {STREET_GUARD})")));
    }
    let a_start = html
        .find("<address")
        .ok_or_else(|| missing("Adress-Block fehlt"))?;
    let a_tail = &html[a_start..];
    let a_end = a_tail
        .find("</address>")
        .ok_or_else(|| missing("Adress-Block unvollständig"))?;
    let block = a_tail[..a_end]
        .replace("<br />", "\n")
        .replace("<br/>", "\n")
        .replace("<br>", "\n");
    let lines: Vec<String> = block
        .split('\n')
        .map(|l| {
            // `<br`-Split hinterlässt Tag-Reste → erst alles bis zum
            // ersten `>` verwerfen, dann whitespace-normieren.
            let no_tags: String = l
                .split('>')
                .next_back()
                .unwrap_or("")
                .split('<')
                .next()
                .unwrap_or("")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            no_tags
        })
        .filter(|l| !l.is_empty())
        .collect();
    let street = lines
        .iter()
        .find(|l| l.contains(STREET_GUARD))
        .cloned()
        .unwrap_or_default();
    let (mut postcode, mut city) = (String::new(), String::new());
    for l in &lines {
        let mut it = l.split_whitespace();
        let Some(pc) = it.next() else { continue };
        if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
            postcode = pc.to_owned();
            city = it.collect::<Vec<_>>().join(" ");
            break;
        }
    }
    let doc = Html::parse_document(html);
    let link_text = |prefix: &str| {
        let sel = Selector::parse(&format!("a[href^=\"{prefix}\"]")).expect("valid selector");
        doc.select(&sel)
            .next()
            .map(|el| {
                el.text()
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default()
    };
    let phone = link_text("tel:");
    let email = link_text("mailto:");
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(missing("keine Kontaktdaten gefunden"));
    }
    Ok(TraderInfo {
        street,
        postcode,
        city,
        phone,
        email,
    })
}

#[cfg(test)]
mod tests {
    use super::{extract_info, hero_unze_skip, parse, unit_of};

    // Reale Live-Ausschnitte der Homepage (Element-Bauart span#calc-spot
    // + p.bars-note + Hero-Medaillon), Preise = Stand 28.09.2026.
    const FIXTURE: &str = "<p class=\"micro-label medallion-lbl\" id=\"hero-metal-lbl\">Tagespreis Gold</p>\
        <p class=\"medallion-price\" id=\"hero-gold-price\">3.563,90&nbsp;€</p>\
        <p class=\"medallion-sub\" id=\"hero-metal-sub\">pro Unze · 999er Feingold</p>\
        <div class=\"calc-meta-row\"><span class=\"k\" id=\"calc-spot-lbl\">Tageskurs Feingold</span>\
        <span class=\"v\" id=\"calc-spot\">122,00&nbsp;€/g</span></div>\
        <p class=\"bars-note\">* Richtwerte auf Basis des aktuellen Tagespreises (Gold \
        <span id=\"bars-spot\">122,00&nbsp;€</span>/g, Silber \
        <span id=\"bars-spot-silver\">1,80&nbsp;€</span>/g). Ein verbindliches Angebot \
        erhalten Sie nach persönlicher Beratung.</p>";

    #[test]
    fn two_spot_prices_become_rows() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty());
        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows[0],
            (
                "gold",
                "999",
                "Tageskurs Feingold".to_owned(),
                122.0,
                "EUR/g"
            )
        );
        assert_eq!(
            rows[1],
            (
                "silber",
                "999",
                "Silber-Tagespreis".to_owned(),
                1.8,
                "EUR/g"
            )
        );
        assert_eq!(unit_of("122,00 €/g"), Some("EUR/g"));
        assert_eq!(unit_of("pro Sack"), None);
        assert!(parse("<div>Redesign ohne Kurse</div>").is_err());
        assert!(parse("Tageskurs Feingold ohne Element").is_err());
    }

    #[test]
    fn hero_ounce_skips_loudly() {
        let skips = hero_unze_skip(FIXTURE);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Unze"));
        assert!(skips[0].contains("keine Umrechnung"));
        assert!(hero_unze_skip("<div>ohne Hero</div>").is_empty());
    }

    #[test]
    fn zero_spot_skips_loudly() {
        let html = FIXTURE.replace("122,00&nbsp;€/g", "0,00&nbsp;€/g");
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "silber");
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("kein Ankaufspreis"));
    }

    #[test]
    fn missing_silver_note_skips_loudly() {
        let html = FIXTURE.replace("Silber", "Platin");
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "gold");
        assert!(skips.iter().any(|s| s.contains("Silber")));
    }

    #[test]
    fn impressum_contact_block() {
        // Realer Ausschnitt der Live-Impressumseite /impressum.
        let imp = "<h2>Diensteanbieter</h2><address><strong>Rohat Erdem</strong><br />\
            Einzelunternehmen, handelnd unter wirkaufendeingold.de<br />\
            Admiralstraße 111<br />28215 Bremen-Findorff<br />Deutschland</address>\
            <h2>Kontakt</h2><p>Telefon: <a href=\"tel:015150247474\">0151 5024 7474</a><br />\
            E-Mail: <a href=\"mailto:info@wirkaufendeingold.de\">info@wirkaufendeingold.de</a><br />\
            Internet: <a href=\"https://wirkaufendeingold.de\">wirkaufendeingold.de</a></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Admiralstraße 111");
        assert_eq!(info.postcode, "28215");
        assert_eq!(info.city, "Bremen-Findorff");
        assert_eq!(info.phone, "0151 5024 7474");
        assert_eq!(info.email, "info@wirkaufendeingold.de");
        assert!(extract_info("<div>ohne Anker</div>").is_err());
        // Fremde Seite scheitert am Straßen-Anker.
        let other = imp.replace("Admiralstraße 111", "Sögestraße 1");
        assert!(extract_info(&other).is_err());
    }
}
