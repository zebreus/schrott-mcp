//! OPHIRUM GmbH (Edelmetall-Ankauf, zentrale Legierungspreise in EUR/g):
//! eine Datei für drei Slugs — gleiche Seitenform (eine zentrale
//! Ankaufseite, Kontaktblöcke gleicher Bauart).
//!
//! Preisquelle für alle drei Filialen ist die zentrale Goldankaufseite
//! (`PRICE_URL`): sechs `div.price-box-wrapper`-Karten ("333
//! Goldlegierung" … "999 Goldlegierung" mit `h4.realtime-price` wie
//! "32,41 € pro Gramm") zwischen der Überschrift "Unsere aktuellen
//! Legierungspreise" und dem `further-services`-Teaserblock, plus
//! `<div class="date">Stand: 28.09.2026 …</div>` als Seitendatum. Die
//! Filialseiten (`/filialen/…`) zeigen Shop-Verkaufspreise je Stück
//! (Division durchs Gewicht wäre verbotene Umrechnung) und tragen daher
//! keine Ankaufpreise bei — sie liefern nur den Kontakt je Filiale.
//!
//! Katalogeinheit für `gold` ist EUR/g (keine Umrechnung irgendwo);
//! der Feingehalt fährt in der Variante ("333 Goldlegierung" →
//! `gold`/`333`). `0,00 €`-Karten (kein Ankauf) skippen laut.
//! Das Seitendatum → Outcome-`published_at`; ohne Datum → `None`.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG_FRANKFURT: &str = "he-frankfurt-60311-ophirum";
pub const SLUG_BREMEN: &str = "hb-bremen-28203-ophirum-bremen-by-goldfuxx";
pub const SLUG_HANAU: &str = "he-hanau-63450-goldfuxx-hanau-ophirum";

/// Zentrale Ankaufpreisliste, live-verifiziert 28.09.2026 (sechs
/// Legierungskarten + Stand-Datum, server-rendered). Gilt für alle
/// Filialen per Design — ein Umzug lässt alle drei Steps laut scheitern.
pub const PRICE_URL: &str = "https://www.ophirum.de/goldankauf";
/// Bespoke, live-verifizierte Filial-Kontaktseiten (je eigene URL +eigener
/// Block, kein Crawler). Hanau: live-kanonisch `/filialen/hanau`
/// (JSON-LD + `store-address` "Hirschstr. 11, 63450 Hanau"); die
/// Seed-URL `…/hanau-goldfuxx` fällt live auf die Filialübersicht zurück
/// und ist daher NICHT verdrahtet. Ein Umzug scheitert laut.
pub const INFO_URL_FRANKFURT: &str = "https://www.ophirum.de/filialen/frankfurt";
pub const INFO_URL_BREMEN: &str = "https://www.ophirum.de/filialen/bremen-goldfuxx";
pub const INFO_URL_HANAU: &str = "https://www.ophirum.de/filialen/hanau";
/// Straßen-Anker je Filiale: schützt davor, dass das Shared-Template
/// still eine andere Filiale ausliefert.
const STREET_FRANKFURT: &str = "Friedensstraße 6-10";
const STREET_BREMEN: &str = "Fedelhören 12";
const STREET_HANAU: &str = "Hirschstr. 11";

pub fn handler_frankfurt() -> Handler {
    Handler {
        slug: SLUG_FRANKFURT,
        url: PRICE_URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_frankfurt(c)),
    }
}

pub fn handler_bremen() -> Handler {
    Handler {
        slug: SLUG_BREMEN,
        url: PRICE_URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_bremen(c)),
    }
}

pub fn handler_hanau() -> Handler {
    Handler {
        slug: SLUG_HANAU,
        url: PRICE_URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_hanau(c)),
    }
}

async fn scrape_frankfurt(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape_branch(client, INFO_URL_FRANKFURT, STREET_FRANKFURT).await
}

async fn scrape_bremen(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape_branch(client, INFO_URL_BREMEN, STREET_BREMEN).await
}

async fn scrape_hanau(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape_branch(client, INFO_URL_HANAU, STREET_HANAU).await
}

async fn scrape_branch(
    client: &reqwest::Client,
    info_url: &str,
    street_guard: &str,
) -> Result<HandlerOutcome, IngestError> {
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
    let (_, info_html) = fetch_text(client, info_url).await?;
    let trader_info = extract_info(&info_html, street_guard, info_url)?;
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
/// Variante (gold/silber/platin/palladium je Feingehalt); ohne
/// Feingehalt oder ohne Metallwort → `None` (lauter Skip). Silber vor
/// Gold: Silberlabels könnten sonst je nach Wortlaut falsch landen.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
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

/// Erster gehandelter Feingehalt im Label ("333 Goldlegierung" → "333").
/// Nur belegte Legierungen; alles andere bleibt variantenlos (""),
/// nie geraten.
fn fineness(l: &str) -> &'static str {
    let b = l.as_bytes();
    let mut i = 0;
    while i + 3 <= b.len() {
        if b[i].is_ascii_digit() && b[i + 1].is_ascii_digit() && b[i + 2].is_ascii_digit() {
            match &l[i..i + 3] {
                "999" => return "999",
                "916" => return "916",
                "900" => return "900",
                "750" => return "750",
                "585" => return "585",
                "333" => return "333",
                _ => {}
            }
        }
        i += 1;
    }
    ""
}

/// Sechs Legierungskarten im Fenster zwischen der Listenüberschrift und
/// dem Teaserblock. Gibt (Zeilen, Skips) mit (Label, Preis, Einheit);
/// `0,00 €` (kein Ankauf) skippt laut. Null Karten → `Err` (ein
/// Redesign darf nie wie Erfolg aussehen).
fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html
        .find("Unsere aktuellen Legierungspreise")
        .ok_or_else(|| IngestError::Parse {
            url: PRICE_URL.to_owned(),
            detail: "Legierungspreise: Listen-Anker fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("further-services").ok_or_else(|| IngestError::Parse {
        url: PRICE_URL.to_owned(),
        detail: "Legierungspreise: Ende-Anker fehlt".to_owned(),
    })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let box_sel = Selector::parse("div.price-box-wrapper").expect("valid selector");
    let title_sel = Selector::parse("h3").expect("valid selector");
    let price_sel = Selector::parse("h4.realtime-price").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for card in frag.select(&box_sel) {
        let label = card
            .select(&title_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if label.is_empty() || label.len() > 120 {
            skips.push("(Preiskarte ohne Titel)".to_owned());
            continue;
        }
        let quote = card
            .select(&price_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();
        // `&nbsp;` normalisieren (`"pro Gramm"` enthält sonst kein
        // `"pro gramm"` und jede Karte skippte fälschlich die Einheit).
        let quote_norm = quote.split_whitespace().collect::<Vec<_>>().join(" ");
        let Some(price) = parse_eur(&quote_norm) else {
            skips.push(format!("{label} (Preis unverständlich: {})", quote.trim()));
            continue;
        };
        if !price.is_finite() || price <= 0.0 {
            skips.push(format!("{label} ({}: kein Ankaufspreis)", fmt_eur(price)));
            continue;
        }
        // Laut skippen statt Default: ein Kilo-Preis als Gramm verbucht
        // wäre ein 1000×-Fehler.
        let Some(unit) = unit_of(&quote_norm) else {
            skips.push(format!(
                "{label} (Einheit unverständlich: {})",
                quote.trim()
            ));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: PRICE_URL.to_owned(),
            detail: "Legierungspreise: keine Preiskarten".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke Unit-Matcher DIESER Karten (live: "32,41 € pro Gramm").
/// Nur g existiert hier — alles andere skippt laut am Call-Site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("pro gramm") || lower.contains("/g") {
        Some("EUR/g")
    } else {
        None
    }
}

/// Seitendatum aus `<div class="date">Stand: 28.09.2026 …</div>` →
/// RFC 3339 (Mitternacht UTC) via `parse_de_date`. Kein Datum → `None`
/// (Alter = `observed_at`); ein Datum scheitert hier nie den Step.
fn published_at_of(html: &str) -> Option<String> {
    let at = html.find("Stand:")?;
    let tail = &html[at + "Stand:".len()..];
    let b = tail.as_bytes();
    let mut i = 0;
    while i + 10 <= b.len() {
        let s = &tail[i..i + 10];
        let c: Vec<char> = s.chars().collect();
        if c.len() == 10
            && c[0].is_ascii_digit()
            && c[1].is_ascii_digit()
            && c[2] == '.'
            && c[3].is_ascii_digit()
            && c[4].is_ascii_digit()
            && c[5] == '.'
            && c[6].is_ascii_digit()
            && c[7].is_ascii_digit()
            && c[8].is_ascii_digit()
            && c[9].is_ascii_digit()
        {
            return parse_de_date(
                &s[0..2].to_owned(),
                &s[3..5].to_owned(),
                &s[6..10].to_owned(),
            );
        }
        i += 1;
    }
    None
}

/// Bespoke Kontakt-Extraktion NUR für diese Filialseiten-Bauart: die
/// `div.store-address` ("Friedensstraße 6-10, 60311 Frankfurt am Main",
/// `&nbsp;` inklusive) plus `tel:`-/`mailto:`-Links der Kontakt-Kachel
/// (Anzeige-Text gewinnt). Fehlender Adress-Anker oder falsche Straße →
/// lauter Fehler, nie geraten. Leeres Telefon (Hanau: nur Mail) ist
/// normal und kein Fehler.
fn extract_info(html: &str, street_guard: &str, info_url: &str) -> Result<TraderInfo, IngestError> {
    let missing = |detail: &str| IngestError::Parse {
        url: info_url.to_owned(),
        detail: detail.to_owned(),
    };
    let a_start = html
        .find("store-address")
        .ok_or_else(|| missing("Adress-Block fehlt"))?;
    let a_tail = &html[a_start..];
    let a_end = a_tail
        .find("</div>")
        .ok_or_else(|| missing("Adress-Block unvollständig"))?;
    if !a_tail[..a_end].contains(street_guard) {
        return Err(missing(&format!("falsche Filiale (kein {street_guard})")));
    }
    let doc = Html::parse_document(html);
    // Kontakt-Kachel als Muss-Anker: tel/mailto werden NUR in ihr gelesen
    // (dokumentweit fände man sonst die Mobile-Nav-Buttons zuerst).
    let tile_sel = Selector::parse("div.contact-tile").expect("valid selector");
    let tile = doc
        .select(&tile_sel)
        .next()
        .ok_or_else(|| missing("Kontakt-Kachel fehlt"))?;
    let addr_sel = Selector::parse("div.store-address").expect("valid selector");
    let addr_text: String = doc
        .select(&addr_sel)
        .next()
        .map(|el| el.text().collect())
        .unwrap_or_default();
    let addr_text = addr_text.split_whitespace().collect::<Vec<_>>().join(" ");
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if let Some((left, right)) = addr_text.split_once(',') {
        street = left.trim().to_owned();
        let mut it = right.split_whitespace();
        if let Some(pc) = it.next() {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.collect::<Vec<_>>().join(" ");
            }
        }
    }
    let link_text = |prefix: &str| {
        let sel = Selector::parse(&format!("a[href^=\"{prefix}\"]")).expect("valid selector");
        tile.select(&sel)
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
    use super::{extract_info, fineness, grade_for, parse, published_at_of, unit_of};

    // Realer Live-Ausschnitt der Goldankaufseite (Karten-Bauart h3 +
    // dimension + h4.realtime-price, Stand-Datum, Teaser-Terminator),
    // auf alle sechs Legierungen bestückt, Preise = Stand 28.09.2026.
    const FIXTURE: &str = "Unsere aktuellen Legierungspreise</h2></div></div>\
        <div class=\"col-12 col-md-4 col-lg-3\"><div class=\"goldprice-realtime\">\
        <div class=\"price-box-wrapper\"><h3 class=\"h3 text-uppercase\">333 Goldlegierung</h3>\
        <div class=\"dimension\">333/1000</div><div class=\"mobile-wrapper\">\
        <h4 class=\"h4 realtime-price text-uppercase\">32,41&nbsp;€&nbsp;\
        <span style=\"font-size:14px\">pro&nbsp;Gramm</span></h4></div></div></div></div>\
        <div class=\"col-12 col-md-4 col-lg-3\"><div class=\"goldprice-realtime\">\
        <div class=\"price-box-wrapper\"><h3 class=\"h3 text-uppercase\">585 Goldlegierung</h3>\
        <div class=\"dimension\">585/1000</div><div class=\"mobile-wrapper\">\
        <h4 class=\"h4 realtime-price text-uppercase\">56,88&nbsp;€&nbsp;\
        <span style=\"font-size:14px\">pro&nbsp;Gramm</span></h4></div></div></div></div>\
        <div class=\"col-12 col-md-4 col-lg-3\"><div class=\"goldprice-realtime\">\
        <div class=\"price-box-wrapper\"><h3 class=\"h3 text-uppercase\">750 Goldlegierung</h3>\
        <div class=\"dimension\">750/1000</div><div class=\"mobile-wrapper\">\
        <h4 class=\"h4 realtime-price text-uppercase\">72,92&nbsp;€&nbsp;\
        <span style=\"font-size:14px\">pro&nbsp;Gramm</span></h4></div></div></div></div>\
        <div class=\"col-12 col-md-4 col-lg-3\"><div class=\"goldprice-realtime\">\
        <div class=\"price-box-wrapper\"><h3 class=\"h3 text-uppercase\">900 Goldlegierung</h3>\
        <div class=\"dimension\">900/1000</div><div class=\"mobile-wrapper\">\
        <h4 class=\"h4 realtime-price text-uppercase\">89,65&nbsp;€&nbsp;\
        <span style=\"font-size:14px\">pro&nbsp;Gramm</span></h4></div></div></div></div>\
        <div class=\"col-12 col-md-4 col-lg-3\"><div class=\"goldprice-realtime\">\
        <div class=\"price-box-wrapper\"><h3 class=\"h3 text-uppercase\">916 Goldlegierung</h3>\
        <div class=\"dimension\">916/1000</div><div class=\"mobile-wrapper\">\
        <h4 class=\"h4 realtime-price text-uppercase\">91,24&nbsp;€&nbsp;\
        <span style=\"font-size:14px\">pro&nbsp;Gramm</span></h4></div></div></div></div>\
        <div class=\"col-12 col-md-4 col-lg-3\"><div class=\"goldprice-realtime\">\
        <div class=\"price-box-wrapper\"><h3 class=\"h3 text-uppercase\">999 Goldlegierung</h3>\
        <div class=\"dimension\">999/1000</div><div class=\"mobile-wrapper\">\
        <h4 class=\"h4 realtime-price text-uppercase\">108,42&nbsp;€&nbsp;\
        <span style=\"font-size:14px\">pro&nbsp;Gramm</span></h4></div></div></div></div>\
        <div class=\"date\">Stand: 28.09.2026 18:50:41</div>\
        <div class=\"further-services no-margin\">";

    #[test]
    fn six_cards_become_rows() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty());
        assert_eq!(rows.len(), 6);
        assert_eq!(rows[0], ("333 Goldlegierung".to_owned(), 32.41, "EUR/g"));
        assert_eq!(rows[1], ("585 Goldlegierung".to_owned(), 56.88, "EUR/g"));
        assert_eq!(rows[2], ("750 Goldlegierung".to_owned(), 72.92, "EUR/g"));
        assert_eq!(rows[3], ("900 Goldlegierung".to_owned(), 89.65, "EUR/g"));
        assert_eq!(rows[4], ("916 Goldlegierung".to_owned(), 91.24, "EUR/g"));
        assert_eq!(rows[5], ("999 Goldlegierung".to_owned(), 108.42, "EUR/g"));
        assert_eq!(unit_of("32,41 € pro Gramm"), Some("EUR/g"));
        assert_eq!(unit_of("pro Sack"), None);
        assert!(parse("<div>Redesign ohne Liste</div>").is_err());
        assert!(parse("Unsere aktuellen Legierungspreise ohne Karten").is_err());
    }

    #[test]
    fn zero_price_skips_loudly() {
        let html = FIXTURE.replace(
            "108,42&nbsp;€",
            "0,00&nbsp;€",
        );
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 5);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("999 Goldlegierung"));
        assert!(skips[0].contains("kein Ankaufspreis"));
    }

    #[test]
    fn fineness_rides_in_variant() {
        assert_eq!(grade_for("333 Goldlegierung"), Some(("gold", "333")));
        assert_eq!(grade_for("585 Goldlegierung"), Some(("gold", "585")));
        assert_eq!(grade_for("750 Goldlegierung"), Some(("gold", "750")));
        assert_eq!(grade_for("900 Goldlegierung"), Some(("gold", "900")));
        assert_eq!(grade_for("916 Goldlegierung"), Some(("gold", "916")));
        assert_eq!(grade_for("999 Goldlegierung"), Some(("gold", "999")));
        // Künftige Silber-/Platin-/Palladium-Karten landen je Metall
        // (nur belegte Feingehalte — nie geraten).
        assert_eq!(grade_for("999 Silberlegierung"), Some(("silber", "999")));
        assert_eq!(grade_for("900 Platinlegierung"), Some(("platin", "900")));
        assert_eq!(
            grade_for("750 Palladiumlegierung"),
            Some(("palladium", "750"))
        );
        // Unbelegte Feingehalte und metallose Labels: kein Raten.
        assert_eq!(grade_for("925 Silberlegierung"), None);
        assert_eq!(grade_for("Altgold-Ankauf"), None);
        assert_eq!(grade_for("Zahngold"), None);
        assert_eq!(fineness("goldlegierung"), "");
    }

    #[test]
    fn stand_date_becomes_published_at() {
        assert_eq!(
            published_at_of(FIXTURE),
            Some("2026-09-28T00:00:00+00:00".to_owned())
        );
        assert_eq!(published_at_of("<div>ohne Datum</div>"), None);
    }

    #[test]
    fn frankfurt_contact_block() {
        // Realer Ausschnitt der Live-Filialseite /filialen/frankfurt.
        let imp = "<div class=\"store-shop-details-tile mr-24\">\
            <div class=\"h2 text-uppercase\">OPHIRUM Frankfurt/Main</div>\
            <div class=\"store-address\">Friedensstraße 6-10, 60311&nbsp;Frankfurt am Main</div></div>\
            <div class=\"store-shop-details-tile-inner contact-tile\"><div class=\"contact-icon store-shop-tel\">\
            <a href=\"tel:004969210295821\">(069) 210 295 821</a></div>\
            <div class=\"store-shop-details-tile-inner contact-tile\"><div class=\"contact-icon store-shop-email\">\
            <a href=\"mailto:frankfurt@ophirum.de\">frankfurt@ophirum.de</a></div></div></div>";
        let info = extract_info(imp, "Friedensstraße 6-10", "https://x/frankfurt").expect("parses");
        assert_eq!(info.street, "Friedensstraße 6-10");
        assert_eq!(info.postcode, "60311");
        assert_eq!(info.city, "Frankfurt am Main");
        assert_eq!(info.phone, "(069) 210 295 821");
        assert_eq!(info.email, "frankfurt@ophirum.de");
        assert!(extract_info("<div>ohne Adressblock</div>", "Friedensstraße 6-10", "https://x").is_err());
        // Fremde Filiale scheitert am Straßen-Anker.
        let other = imp.replace("Friedensstraße 6-10", "Fedelhören 12");
        assert!(extract_info(&other, "Friedensstraße 6-10", "https://x").is_err());
    }

    #[test]
    fn bremen_contact_block() {
        // Realer Ausschnitt der Live-Filialseite /filialen/bremen-goldfuxx.
        let imp = "<div class=\"store-shop-details-tile mr-24\">\
            <div class=\"h2 text-uppercase\">OPHIRUM Bremen by GOLDFUXX</div>\
            <div class=\"store-address\">Fedelhören 12, 28203&nbsp;Bremen</div></div>\
            <div class=\"store-shop-details-tile-inner contact-tile\"><div class=\"contact-icon store-shop-tel\">\
            <a href=\"tel:004942141650555\">(0421) 416 50555</a></div>\
            <div class=\"contact-icon store-shop-email\">\
            <a href=\"mailto:service@goldfuxx.de\">service@goldfuxx.de</a></div></div>";
        let info =
            extract_info(imp, "Fedelhören 12", "https://x/bremen").expect("parses");
        assert_eq!(info.street, "Fedelhören 12");
        assert_eq!(info.postcode, "28203");
        assert_eq!(info.city, "Bremen");
        assert_eq!(info.phone, "(0421) 416 50555");
        assert_eq!(info.email, "service@goldfuxx.de");
    }

    #[test]
    fn hanau_contact_block_without_phone() {
        // Realer Ausschnitt der Live-Filialseite /filialen/hanau: kein
        // Telefon, nur Mail — das ist normal, kein Fehler.
        let imp = "<div class=\"store-shop-details-tile mr-24\">\
            <div class=\"h2 text-uppercase\">OPHIRUM Hanau by GOLDFUXX</div>\
            <div class=\"store-address\">Hirschstr. 11, 63450&nbsp;Hanau</div></div>\
            <div class=\"store-shop-details-tile-inner contact-tile\"><div class=\"contact-icon store-shop-email\">\
            <a href=\"mailto:service@goldfuxx.de\">service@goldfuxx.de</a></div></div>";
        let info = extract_info(imp, "Hirschstr. 11", "https://x/hanau").expect("parses");
        assert_eq!(info.street, "Hirschstr. 11");
        assert_eq!(info.postcode, "63450");
        assert_eq!(info.city, "Hanau");
        assert_eq!(info.phone, "");
        assert_eq!(info.email, "service@goldfuxx.de");
    }
}
