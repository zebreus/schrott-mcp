//! WirKaufenDeinGold.de (Rohat Erdem, Bremen-Findorff, Edelmetall-Ankauf).
//! The homepage publishes gold/silver spot references and calculates a
//! separate indicative purchase estimate from customer-specific inputs.
//! The spot values are not the trader's buy quotes, so they are skipped and
//! only the directly evidenced material acceptances are recorded.
//!
//! Die Homepage trägt drei server-renderte
//! Tagespreis-Elemente — `span#calc-spot` ("122,00 €/g", Label
//! `span#calc-spot-lbl` "Tageskurs Feingold"), `p.bars-note` ("… Gold
//! 122,00 €/g, Silber 1,80 €/g …") and `p#hero-gold-price` (a per-ounce
//! display). The actual `#calc-result-low` is an indicative estimate based
//! on purity, weight and payout factor, not a general price per gram. The
//! spot and ounce figures are retained as explicit skips, never as buy
//! prices. No publication date is stated.
//!
//! Kontakt (`INFO_URL`, live-verifiziert 28.09.2026): Impressum mit
//! `<address>` ("Rohat Erdem … Admiralstraße 111, 28215
//! Bremen-Findorff") plus `tel:`-/`mailto:`-Links. Fehlt der
//! Diensteanbieter-Anker oder die Straße → lauter Fehler, nie raten.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo,
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
    let mut skipped_labels = parse(&html)?;
    // Kontaktseiten-Fehler scheitern laut per Design: ein gezogenes
    // Impressum braucht Augen, bevor ihm wieder vertraut wird.
    let (_, info_html) = fetch_text(client, INFO_URL).await?;
    let trader_info = extract_info(&info_html)?;
    // Der Unzen-Hero ist ein sichtbarer Preis ohne Katalogeinheit —
    // laut skippen, nie umrechnen, nie still fallenlassen.
    skipped_labels.extend(hero_unze_skip(&html));
    Ok(HandlerOutcome {
        prices: vec![],
        acceptances: vec![
            ScrapedAcceptance {
                material: "gold",
                conditions: "Goldankauf; konkreter Preis nach Prüfung, Tageskurs nur Referenz"
                    .to_owned(),
                label: "Goldankauf auf der Betreiberseite".to_owned(),
            },
            ScrapedAcceptance {
                material: "silber",
                conditions: "Silberankauf; konkreter Preis nach Prüfung, Tageskurs nur Referenz"
                    .to_owned(),
                label: "Silberankauf auf der Betreiberseite".to_owned(),
            },
        ],
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: PRICE_URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

/// Validate the public reference-rate anchors and return skips only. There
/// is no site-wide, unconditional purchase price to record from these values.
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
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
    let mut skips = Vec::new();
    // Gold: the independently labelled spot quote is a calculation input,
    // not the trader's payout. Keep it visible in the run's skip diagnostics.
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
            Some(u) => skips.push(format!(
                "Tageskurs Feingold {} {u} (Spot-Referenz, kein Händler-Ankaufspreis)",
                fmt_eur(p)
            )),
            None => skips.push(format!(
                "Tageskurs Feingold (Einheit unverständlich: {gold_txt})"
            )),
        },
        Some(p) => skips.push(format!("Tageskurs Feingold ({}: kein Kurs)", fmt_eur(p))),
        None => skips.push(format!(
            "Tageskurs Feingold (Preis unverständlich: {gold_txt})"
        )),
    }
    // Silber: the note states a second spot reference, not a guaranteed
    // customer payout. Read only after "Silber" to avoid the preceding gold.
    let note = text_of("p.bars-note");
    match note.split_once("Silber") {
        Some((_, tail)) => match parse_eur(tail) {
            Some(p) if p.is_finite() && p > 0.0 => match unit_of(tail) {
                Some(u) => skips.push(format!(
                    "Silber-Tageskurs {} {u} (Spot-Referenz, kein Händler-Ankaufspreis)",
                    fmt_eur(p)
                )),
                None => skips.push(format!(
                    "Silber-Tagespreis (Einheit unverständlich: {})",
                    tail.trim()
                )),
            },
            Some(p) => skips.push(format!("Silber-Tagespreis ({}: kein Kurs)", fmt_eur(p))),
            None => skips.push(format!(
                "Silber-Tagespreis (Preis unverständlich: {})",
                tail.trim()
            )),
        },
        None => skips.push("(Silber-Tagespreis: Hinweis fehlt)".to_owned()),
    }
    if !skips.iter().any(|s| s.contains("Spot-Referenz")) {
        return Err(missing("keine validierbaren Tageskurs-Referenzen gefunden"));
    }
    Ok(skips)
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
    fn spot_prices_are_skipped_not_recorded_as_buy_rows() {
        let skips = parse(FIXTURE).expect("parses");
        assert_eq!(skips.len(), 2);
        assert!(skips.iter().all(|s| s.contains("Spot-Referenz")));
        assert!(skips
            .iter()
            .all(|s| s.contains("kein Händler-Ankaufspreis")));
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
        let skips = parse(&html).expect("parses");
        assert_eq!(skips.len(), 2);
        assert!(skips
            .iter()
            .any(|s| s.contains("Tageskurs Feingold") && s.contains("kein Kurs")));
        assert!(skips
            .iter()
            .any(|s| s.contains("Silber-Tageskurs") && s.contains("Spot-Referenz")));
    }

    #[test]
    fn missing_silver_note_skips_loudly() {
        let html = FIXTURE.replace("Silber", "Platin");
        let skips = parse(&html).expect("parses");
        assert_eq!(skips.len(), 2);
        assert!(skips
            .iter()
            .any(|s| s.contains("Tageskurs Feingold") && s.contains("Spot-Referenz")));
        assert!(skips
            .iter()
            .any(|s| s.contains("Silber-Tagespreis") && s.contains("Hinweis fehlt")));
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
