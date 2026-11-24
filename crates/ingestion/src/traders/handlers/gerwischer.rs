//! Gerwischer Rohstoffrecycling & Verwertungs GmbH (Gerwisch b. Magdeburg):
//! the site was rebuilt on 24.11.2026 (WordPress/Elementor became a
//! Next.js application) and the price list is gone: `…/ankaufspreise/`
//! answers `HTTP 404` since between 11:24 and 17:37 UTC that day. The
//! new site publishes **no** price at all — the fourteen material pages
//! under `/materialien/` name the qualities the yard buys and point to
//! a "Preisanfrage" (price inquiry); the header calls it
//! "Unverbindlich Preisanfrage".
//!
//! The handler therefore records **acceptances** ("auf Anfrage") for the
//! qualities whose catalog material is unambiguous and skips the rest
//! loudly — the same pattern it has always used for unspecific rows
//! ("Kupfer (blank)", "Aluminium (rein)"). A price is never invented and
//! a stale price is never kept: the thirteen current pointers of this
//! trader were retired by migration 13 (24.11.2026), the history stays.
//!
//! Loud skips (the page's own wording, never guessed):
//! "Aluminiumqualitäten", "Kupferqualitäten" and "Messingqualitäten"
//! name several grades without naming one, "Mischschrott leicht" and
//! "Mischschrott schwer" do not name a catalog material without doubt
//! (the old price list had "Neuschrott", "Mischschrott" and
//! "Leichtschrott" — three different grades).

use scraper::{Html, Selector};

use super::super::{
    fetch_text, normalize_phone, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "st-gerwisch-gerwischer-rohstoffrecycling-verwertungs";
/// Bespoke, live-verified impressum URL (the site's own footer link).
/// A move fails the step loudly (fix the URL) — never guessed, never
/// shared.
pub const IMPRESSUM_URL: &str = "https://gerwischer-recycling.de/impressum/";
/// The material pages of the rebuilt site (live-verified 24.11.2026):
/// fourteen `a[data-testid="material-card"]` entries, no price on the
/// page and none on the cards.
pub const URL: &str = "https://gerwischer-recycling.de/materialien/";

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
    let (karten, mut skipped_labels) = parse_materialien(&html)?;
    // Die Materialien, deren Katalogmaterial eindeutig ist, werden als
    // Annahme ("auf Anfrage") gefuehrt - der Betrieb verkauft sie, aber
    // nennt keinen Kurs.
    let mut acceptances = Vec::with_capacity(karten.len());
    for (slug, titel) in karten {
        match material_fuer(&slug) {
            Some(material) => acceptances.push(ScrapedAcceptance {
                material,
                conditions: format!(
                    "{titel}; auf Anfrage; https://gerwischer-recycling.de/materialien/{slug}"
                ),
                label: titel.clone(),
            }),
            None => skipped_labels.push(format!("{titel} (kein eindeutiges Katalogmaterial)")),
        }
    }
    if acceptances.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine eindeutig zuordenbaren Materialien".to_owned(),
        });
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        skip_price_acceptances: false,
        prices: Vec::new(),
        acceptances,
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

/// The explicit slug → catalog material table. Anything unlisted is
/// skipped loudly: a quality that names several grades must not land on
/// one specific grade, and a quality whose catalog bucket is in doubt
/// must not be guessed. The page's own name travels in the conditions
/// and in the label, so the traceability never depends on the slug.
fn material_fuer(slug: &str) -> Option<&'static str> {
    match slug {
        // Guss: each quality is one grade of the cast-iron bucket.
        "grauguss" => Some("eisenschrott-gussbruch"),
        "sphaeroguss" => Some("eisenschrott-gussbruch"),
        "stahlguss" => Some("eisenschrott-gussbruch"),
        "maschinenguss-2b" => Some("eisenschrott-gussbruch"),
        "handelsguss-3b" => Some("eisenschrott-gussbruch"),
        // Kupfer: the two qualified grades.
        "kupfer-berry" => Some("kupfer-berry"),
        "kupfer-millberry" => Some("kupfer-millberry"),
        // Edelstahl: the two grades the old price list named as well.
        "v2a-rostfrei" => Some("edelstahl-v2a"),
        "v4a-saeurefest" => Some("edelstahl-v4a"),
        // "aluminiumqualitaeten", "kupferqualitaeten", "messingqualitaeten",
        // "mischschrott-leicht" and "mischschrott-schwer" stay unmapped:
        // they name several grades or a bucket in doubt.
        _ => None,
    }
}

/// The material cards of the new site: `a[data-testid="material-card"]`
/// with the link to the quality page and the card's title. A missing
/// card set fails the step loudly — a page without cards is a changed
/// page, not an empty yard.
fn parse_materialien(html: &str) -> Result<(Vec<(String, String)>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let karten_sel = Selector::parse("a[data-testid=\"material-card\"]").expect("valid selector");
    let titel_sel = Selector::parse("span.text-lg").expect("valid selector");
    let mut out = Vec::new();
    for karte in doc.select(&karten_sel) {
        let href = karte.value().attr("href").unwrap_or_default().to_owned();
        let slug = href.trim_start_matches("/materialien/").to_owned();
        if slug.is_empty() || slug.contains('/') {
            continue;
        }
        let titel = karte
            .select(&titel_sel)
            .next()
            .map(|e| {
                e.text()
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default();
        if titel.is_empty() {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: format!("Materialkarte {slug} ohne Titel"),
            });
        }
        out.push((slug, titel));
    }
    if out.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Materialkarten".to_owned(),
        });
    }
    Ok((out, Vec::new()))
}

/// Bespoke contact extraction for THIS impressum only. The rebuilt page
/// carries the blocks as plain paragraphs: the "Angaben gemäß § 5 TMG"
/// heading is followed by the company, the street, the PLZ line, the
/// tax number and the register, the "Kontakt" heading by the telephone
/// and the mail. The mail address itself is hidden behind the CDN
/// protection of the page, so it is simply not found — the importer only
/// fills or upgrades, never blanks.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2_sel = Selector::parse("h2").expect("valid selector");
    let p_sel = Selector::parse("p").expect("valid selector");
    let hat_anker = doc
        .select(&h2_sel)
        .any(|h| h.text().collect::<String>().contains("Angaben gemäß § 5 TMG"));
    if !hat_anker {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "TMG-Block fehlt".to_owned(),
        });
    }
    let zeilen: Vec<String> = doc
        .select(&p_sel)
        .map(|p| {
            p.text()
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, zeile) in zeilen.iter().enumerate() {
        let mut it = zeile.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5
                && pc.chars().all(|c| c.is_ascii_digit())
                && ci.chars().next().is_some_and(|c| c.is_uppercase())
            {
                postcode = pc.to_owned();
                city = zeile[pc.len()..].trim().to_owned();
                if k > 0 {
                    street = zeilen[k - 1].clone();
                }
                break;
            }
        }
    }
    let mut phone = String::new();
    for zeile in &zeilen {
        if let Some(rest) = zeile.strip_prefix("Telefon:") {
            if phone.is_empty() {
                phone = rest.trim().to_owned();
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
        phone: normalize_phone(&phone),
        email: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::{extract_info, material_fuer, parse_materialien};

    /// Real excerpt from the live page of 24.11.2026 (whitespace
    /// collapsed, classes verbatim): the fourteen material cards with
    /// their title. No price, no euro amount, no kilogram.
    const FIXTURE: &str = "<div class=\"grid gap-6 sm:grid-cols-2 xl:grid-cols-4\">\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/aluminiumqualitaeten\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Aluminiumqualitäten</span>\
        <span class=\"mt-2 flex-1 text-sm\">Bunte und Oxidationshaut.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/grauguss\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Grauguss</span>\
        <span class=\"mt-2 flex-1 text-sm\">Gusseisen mit Graphit.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/handelsguss-3b\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Handelsguss 3b</span>\
        <span class=\"mt-2 flex-1 text-sm\">Guss für den Handel.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/kupfer-berry\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Kupfer (Berry)</span>\
        <span class=\"mt-2 flex-1 text-sm\">Kupfer mit hohem Nickelanteil.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/kupfer-millberry\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Kupfer (Millberry)</span>\
        <span class=\"mt-2 flex-1 text-sm\">Kupfer mit mindestens 99 %.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/kupferqualitaeten\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Kupferqualitäten</span>\
        <span class=\"mt-2 flex-1 text-sm\">Der Kupfer-Schnellschnelltest.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/maschinenguss-2b\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Maschinenguss 2b</span>\
        <span class=\"mt-2 flex-1 text-sm\">Maschinenguss mit hoher Festigkeit.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/messingqualitaeten\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Messingqualitäten</span>\
        <span class=\"mt-2 flex-1 text-sm\">Messing mit Zinngehalt.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/mischschrott-leicht\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Mischschrott leicht</span>\
        <span class=\"mt-2 flex-1 text-sm\">Dünnwandiger Stahlschrott.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/mischschrott-schwer\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Mischschrott schwer</span>\
        <span class=\"mt-2 flex-1 text-sm\">Stahlschrott mit dickem Material.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/sphaeroguss\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Sphäroguss</span>\
        <span class=\"mt-2 flex-1 text-sm\">Gusseisen mit Graphitkugeln.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/stahlguss\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">Stahlguss</span>\
        <span class=\"mt-2 flex-1 text-sm\">Stahlguss aus dem Ofen.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/v2a-rostfrei\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">V2A (rostfrei)</span>\
        <span class=\"mt-2 flex-1 text-sm\">Edelstahl mit Chrom und Nickel.</span></span></a>\
        <a data-testid=\"material-card\" class=\"card\" href=\"/materialien/v4a-saeurefest\">\
        <span class=\"relative block aspect-[4/3]\"></span>\
        <span class=\"flex flex-1 flex-col p-5\">\
        <span class=\"text-lg font-bold text-brand-dark\">V4A (säurefest)</span>\
        <span class=\"mt-2 flex-1 text-sm\">Edelstahl mit Molybdän.</span></span></a></div>";

    #[test]
    fn parses_all_material_cards() {
        let (karten, skips) = parse_materialien(FIXTURE).expect("parses");
        assert_eq!(karten.len(), 14, "karten: {karten:?}");
        assert!(skips.is_empty());
        assert_eq!(karten[0].0, "aluminiumqualitaeten");
        assert_eq!(karten[0].1, "Aluminiumqualitäten");
        assert_eq!(karten[13].0, "v4a-saeurefest");
        assert_eq!(karten[13].1, "V4A (säurefest)");
    }

    #[test]
    fn a_page_without_cards_is_loud() {
        let html = "<html><body><h1>Materialien</h1><p>Der Preis steht hier.</p></body></html>";
        assert!(parse_materialien(html).is_err());
    }

    #[test]
    fn a_card_without_a_title_is_loud() {
        let html = "<a data-testid=\"material-card\" href=\"/materialien/grauguss\">\
            <span class=\"text-sm\">Gusseisen mit Graphit.</span></a>";
        assert!(parse_materialien(html).is_err());
    }

    #[test]
    fn the_material_table_is_explicit() {
        assert_eq!(material_fuer("grauguss"), Some("eisenschrott-gussbruch"));
        assert_eq!(material_fuer("sphaeroguss"), Some("eisenschrott-gussbruch"));
        assert_eq!(material_fuer("stahlguss"), Some("eisenschrott-gussbruch"));
        assert_eq!(
            material_fuer("maschinenguss-2b"),
            Some("eisenschrott-gussbruch")
        );
        assert_eq!(
            material_fuer("handelsguss-3b"),
            Some("eisenschrott-gussbruch")
        );
        assert_eq!(material_fuer("kupfer-berry"), Some("kupfer-berry"));
        assert_eq!(material_fuer("kupfer-millberry"), Some("kupfer-millberry"));
        assert_eq!(material_fuer("v2a-rostfrei"), Some("edelstahl-v2a"));
        assert_eq!(material_fuer("v4a-saeurefest"), Some("edelstahl-v4a"));
        // Mehrdeutig oder unspezifisch: nie geraten.
        assert_eq!(material_fuer("aluminiumqualitaeten"), None);
        assert_eq!(material_fuer("kupferqualitaeten"), None);
        assert_eq!(material_fuer("messingqualitaeten"), None);
        assert_eq!(material_fuer("mischschrott-leicht"), None);
        assert_eq!(material_fuer("mischschrott-schwer"), None);
        assert_eq!(material_fuer("unbekannt"), None);
    }

    #[test]
    fn nine_of_the_fourteen_cards_map_and_five_skip_loudly() {
        let (karten, _) = parse_materialien(FIXTURE).expect("parses");
        let gemappt: Vec<&str> = karten
            .iter()
            .filter(|(slug, _)| material_fuer(slug).is_some())
            .map(|(slug, _)| slug.as_str())
            .collect();
        assert_eq!(gemappt.len(), 9, "gemappt: {gemappt:?}");
        let uebersprungen: Vec<&str> = karten
            .iter()
            .filter(|(slug, _)| material_fuer(slug).is_none())
            .map(|(slug, _)| slug.as_str())
            .collect();
        assert_eq!(uebersprungen.len(), 5, "uebersprungen: {uebersprungen:?}");
    }

    #[test]
    fn impressum_parses() {
        // Live shape of 24.11.2026: the blocks are plain paragraphs
        // inside one div; the mail address is hidden behind the CDN
        // protection of the page and therefore simply not found.
        let imp = "<div class=\"rich\">\
            <h2>Angaben gemäß § 5 TMG</h2>\
            <p>Gerwischer Rohstoffrecycling und Verwertungs GmbH</p>\
            <p>Lostauer Straße 5</p>\
            <p>39175 Biederitz OT Gerwisch</p>\
            <p>Ust.-ID DE 158 243 753</p>\
            <p>Amtsgericht Stendal | HRB 1316</p>\
            <h2>Kontakt</h2>\
            <p>Telefon: +49 (0) 39292 / 20 90</p>\
            <p>Fax: +49 (0) 39292 / 20 78</p>\
            <p>E-Mail: <a href=\"/cdn-cgi/l/email-protection\">[email protected]</a></p>\
            </div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Lostauer Straße 5");
        assert_eq!(info.postcode, "39175");
        assert_eq!(info.city, "Biederitz OT Gerwisch");
        assert_eq!(info.phone, "039292 2090");
        assert_eq!(info.email, "", "die Mailadresse ist nicht lesbar");
    }

    #[test]
    fn impressum_without_anchor_fails() {
        let imp = "<div class=\"rich\"><h2>Kontakt</h2>\
            <p>Telefon: +49 (0) 39292 / 20 90</p></div>";
        assert!(extract_info(imp).is_err());
    }

    #[tokio::test]
    #[ignore = "read-only live check; fetches the public material page and Impressum"]
    async fn live_handler_records_acceptances_without_prices() {
        let client = reqwest::Client::builder()
            .user_agent("schrott-mcp-ingestion/0.1")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("HTTP client");
        let scrape = super::handler().scrape;
        let outcome = scrape(&client)
            .await
            .expect("live material page and Impressum");

        assert_eq!(outcome.status_code, 200);
        assert!(outcome.prices.is_empty(), "die Seite nennt keine Preise");
        assert_eq!(outcome.acceptances.len(), 9);
        assert!(outcome
            .acceptances
            .iter()
            .all(|a| a.conditions.contains("auf Anfrage")));
        assert_eq!(outcome.skipped_labels.len(), 5);
        assert!(outcome.published_at.is_none());
    }
}
