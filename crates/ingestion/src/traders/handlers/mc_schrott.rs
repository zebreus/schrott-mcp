//! MC Schrott (Brandenburg a.d. Havel; the same list serves all MC
//! Annahmestellen): exact prices in seven `table.price-table` lists
//! ("Bezeichnung" / "Preis €/kg" — every table quotes EUR/kg per its
//! caption "Angaben in Euro pro Kilogramm" and the column header; a
//! table whose price header stops saying kg fails loudly). Page date
//! "Stand: 22.09.2026." in the hero lede → `published_at`. The
//! Preisrechner (`select#calc-material`) and the Standort switch are JS
//! and never touched — only the static tables are parsed. "auf Anfrage"
//! rows (Chromstahl, Widia, Bohrer, 2× Katalysator) skip loudly.
//!
//! NOTE (Korrektur 27.10.2026): Das Impressum nennt den Sitz des
//! Diensteanbieters (Rathenow). Es ist der **Betreiber**, nicht der
//! Standort — der Sitz-Rathenow-Datensatz war eine Fehlzuordnung auf den
//! Brandenburg-Slug. Jeder Annahmestellen-Slug holt sich Kontakt und
//! Adresse deshalb von seiner eigenen Standortseite; das Impressum wird
//! nicht mehr gelesen.
//!
//! Standortabdeckung: Die Liste gilt für alle fünf Brandenburger Annahmestellen
//! (Brandenburg, Rathenow, Nauen, Kloster Lehnin/OT Reckahn, Werder/OT Plötzin);
//! der sichtbare Zellwert ist dieser gemeinsame Satz. Rostock hat einen eigenen
//! Satz, der im Attribut `data-price-ros` steht (7 Sorten weichen ab) — die
//! sichtbare Zelle zeigt dort immer den Brandenburger Wert. Ein `data-only`
//! -Attribut beschränkt einzelne Sorten auf genannte Standorte; an den
//! anderen wird die Zeile laut übersprungen.
//! Proposals (new catalog materials, never crammed): `kuehler-verbund`
//! ("Kupfer Messing Kühler", "Kupfer Alu Kühler", "Kupfer Eisen
//! Kühler", "Alu Kühler"), `alu-verbund-profile` ("Alu Iso-Profile"),
//! `alu-draht-stahlseele`, `batterien` (Starterbatterien/Bleiakkus).
//! "Moniereisen (fertig)" and "Vorbrennermaterial" map to
//! `stahlschrott-scheren` with variants — closest bucket, flagged as
//! uncertain. Bare "Bohrer" stays unmapped: the description says
//! Hartmetall, but unqualified Bohrer could be HSS (cf. Hein's "HSS
//! Bohrer mit Schaft").

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bb-brandenburg-an-der-havel-mc-schrott-annahmestelle";
/// Die weiteren Annahmestellen desselben Betreibers: fünf Brandenburger
/// Standorte teilen sich die sichtbaren Preise, Rostock hat einen eigenen Satz
/// (`data-price-ros`).
pub const SLUG_RATHENOW: &str = "bb-rathenow-mc-schrott-annahmestelle";
pub const SLUG_NAUEN: &str = "bb-nauen-mc-schrott-annahmestelle";
pub const SLUG_RECKAHN: &str = "bb-kloster-lehnin-ot-reckahn-mc-schrott-annahmestelle";
pub const SLUG_PLOTZIN: &str = "bb-werder-ot-plotzin-mc-schrott-annahmestelle";
pub const SLUG_ROSTOCK: &str = "mv-rostock-mc-schrott-rostock";

/// Standortschlüssel (wie im `data-only`-Attribut und in den URLs der Seite),
/// Slug, eigene Kontaktseite und ob der eigene Rostock-Preissatz gilt.
type Site = (&'static str, &'static str, &'static str, bool);

const SITES: &[Site] = &[
    (
        "brandenburg",
        SLUG,
        "https://www.mcschrott.de/standorte/brandenburg/",
        false,
    ),
    (
        "rathenow",
        SLUG_RATHENOW,
        "https://www.mcschrott.de/standorte/rathenow/",
        false,
    ),
    (
        "nauen",
        SLUG_NAUEN,
        "https://www.mcschrott.de/standorte/nauen/",
        false,
    ),
    (
        "kloster-lehnin-reckahn",
        SLUG_RECKAHN,
        "https://www.mcschrott.de/standorte/kloster-lehnin-reckahn/",
        false,
    ),
    (
        "werder-ploetzin",
        SLUG_PLOTZIN,
        "https://www.mcschrott.de/standorte/werder-ploetzin/",
        false,
    ),
    (
        "rostock",
        SLUG_ROSTOCK,
        "https://www.mcschrott.de/standorte/rostock/",
        true,
    ),
];
pub const URL: &str = "https://www.mcschrott.de/preise/";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_brandenburg(c)),
    }
}

pub fn handler_rathenow() -> Handler {
    Handler {
        slug: SLUG_RATHENOW,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_rathenow(c)),
    }
}

pub fn handler_nauen() -> Handler {
    Handler {
        slug: SLUG_NAUEN,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_nauen(c)),
    }
}

pub fn handler_reckahn() -> Handler {
    Handler {
        slug: SLUG_RECKAHN,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_reckahn(c)),
    }
}

pub fn handler_plotzin() -> Handler {
    Handler {
        slug: SLUG_PLOTZIN,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_plotzin(c)),
    }
}

pub fn handler_rostock() -> Handler {
    Handler {
        slug: SLUG_ROSTOCK,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_rostock(c)),
    }
}

async fn scrape_brandenburg(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape(client, "brandenburg").await
}

async fn scrape_rathenow(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape(client, "rathenow").await
}

async fn scrape_nauen(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape(client, "nauen").await
}

async fn scrape_reckahn(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape(client, "reckahn").await
}

async fn scrape_plotzin(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape(client, "plotzin").await
}

async fn scrape_rostock(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    scrape(client, "rostock").await
}

async fn scrape(client: &reqwest::Client, site_key: &str) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (published_at, rows, mut skipped_labels) = parse(&html)?;
    let Some((_, _, site_url, rostock)) = SITES.iter().find(|(key, _, _, _)| *key == site_key)
    else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: format!("unbekannter Standortschlüssel {site_key}"),
        });
    };
    let mut prices = Vec::with_capacity(rows.len());
    for row in rows {
        // `data-only` beschränkt die Sorte auf genannte Standorte; an den
        // anderen wird die Zeile laut übersprungen statt still übernommen.
        if let Some(only) = &row.only {
            if !only.iter().any(|key| key == site_key) {
                skipped_labels.push(row.label);
                continue;
            }
        }
        // Rostock hat einen eigenen Satz im Attribut `data-price-ros`; ohne
        // dieses Attribut ist für Rostock kein eigener Preis belegt.
        let price = if *rostock {
            match row.rostock_price {
                Some(price) => price,
                None => {
                    skipped_labels.push(row.label);
                    continue;
                }
            }
        } else {
            row.price
        };
        match grade_for(&row.label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant: variant.into(),
                price,
                currency: "EUR",
                unit: row.unit,
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label: row.label,
            }),
            None => skipped_labels.push(row.label),
        }
    }
    // Ein Standort ohne verwertbaren Preis ist ein Seitenumbruch, kein
    // leerer Erfolg.
    if prices.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: format!("keine verwertbaren Preise für Standort {site_key}"),
        });
    }
    // Standortseiten-Fehler lässt den ganzen Schritt scheitern: eine
    // verschobene Kontaktseite heißt, dass sich die Seite geändert hat und
    // vorher eyeballs nötig sind.
    let (_, site_html) = fetch_text(client, site_url).await?;
    let trader_info = extract_site_info(&site_html, site_url)?;
    Ok(HandlerOutcome {
        skip_price_acceptances: false,
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

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific-before-generic: "Millberry" wins over "Berry",
/// "Kerze" over bare copper, "Alu Kupfer Kabel" (CCA) over the Kupfer
/// branch. Two cable grades at different prices always get distinct
/// variants so they never collapse.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        return Some(("kupfer-millberry", ""));
    }
    if l.contains("kerze") {
        return Some(("kupfer-berry", "Kerze"));
    }
    if l.contains("berry") {
        return Some(("kupfer-berry", ""));
    }
    if l.contains("raff") {
        return Some(("kupfer-gemischt", "Raff"));
    }
    // Verbundkühler (Cu/Messing, Cu/Alu, Cu/Fe, Alu/Kunststoff) have no
    // catalog material.
    if l.contains("kühler") || l.contains("kuehler") {
        return None;
    }
    // CCA-Lautsprecherkabel: sold as Alu cable, copper-clad.
    if l.contains("alu") && l.contains("kupfer") && l.contains("kabel") {
        return Some(("kabel-alu", "CCA"));
    }
    // Kupfer family.
    if l.contains("kupfer") {
        if l.contains("leitschiene") {
            return Some(("kupfer-gemischt", "Leitschiene"));
        }
        if l.contains("späne") || l.contains("spaene") {
            return Some(("kupfer-gemischt", "Späne"));
        }
        if l.contains("kabel") {
            if l.contains("dünn") || l.contains("duenn") {
                return Some(("kabel-kupfer", "dünn"));
            }
            if l.contains("dick") {
                return Some(("kabel-kupfer", "dick"));
            }
            if l.contains("stecker") {
                return Some(("kabel-mit-stecker", "mit Stecker"));
            }
            if l.contains("blei") {
                return Some(("kabel-blei", "Blei"));
            }
            if l.contains("eisenmantel") {
                return Some(("kabel-kupfer", "Eisenmantel"));
            }
            return None;
        }
        if l.contains("eisen") && l.contains("schrott") {
            // Cu-Fe-Verbund, kein FE-Misch (FE-Audit).
            return None;
        }
        return None;
    }
    // Alu family.
    if l.contains("alu") {
        if l.contains("draht") {
            if l.contains("stahlseele") {
                // Steel-cored overhead line: substantial Fe — no bucket.
                return None;
            }
            if l.contains("blank") {
                return Some(("aluminium-gemischt", "Draht blank"));
            }
            return Some(("aluminium-gemischt", "Draht luftgeschwärzt"));
        }
        if l.contains("gemisch") {
            if l.contains("sauber") {
                return Some(("aluminium-gemischt", ""));
            }
            return Some(("aluminium-gemischt", "2-5% Anhaftung"));
        }
        if l.contains("guss") || l.contains("guß") {
            if l.contains("sauber") {
                return Some(("aluminium-guss", ""));
            }
            return Some(("aluminium-guss", "2-5% Anhaftung"));
        }
        if l.contains("kabel") {
            if l.contains("dünn") || l.contains("duenn") {
                return Some(("kabel-alu", "dünn"));
            }
            return Some(("kabel-alu", "dick"));
        }
        if l.contains("leitschiene") {
            return Some(("aluminium-gemischt", "Leitschiene"));
        }
        if l.contains("profile") || l.contains("profil") {
            if l.contains("blank") {
                return Some(("aluminium-profile", "blank"));
            }
            if l.contains("farbig") {
                return Some(("aluminium-profile", "farbig"));
            }
            if l.contains("2 %") || l.contains("2%") {
                return Some(("aluminium-profile", "2% Anhaftung"));
            }
            return None;
        }
        if l.contains("iso") {
            // Alu-Kunststoff-Verbundprofile: no bucket.
            return None;
        }
        if l.contains("felgen") {
            if l.contains("unsauber") {
                return Some(("aluminium-guss", "Felgen unsauber"));
            }
            return Some(("aluminium-guss", "Felgen sauber"));
        }
        if l.contains("späne") || l.contains("spaene") {
            return Some(("aluminium-gemischt", "Späne"));
        }
        return None;
    }
    // Messing.
    if l.contains("messing") {
        if l.contains("späne") || l.contains("spaene") {
            return Some(("messing", "Späne"));
        }
        return Some(("messing", ""));
    }
    // Edelstahl (unmarked Chromstahl → generic, never a specific grade).
    if l.contains("v4a") {
        return Some(("edelstahl-v4a", ""));
    }
    if l.contains("v2a") {
        if l.contains("späne") || l.contains("spaene") {
            return Some(("edelstahl-v2a", "Späne"));
        }
        return Some(("edelstahl-v2a", ""));
    }
    if l.contains("chromstahl") {
        return Some(("edelstahl-gemischt", "Chromstahl"));
    }
    // Hartmetall (priced rows only; live rows are "auf Anfrage").
    if l.contains("widia") {
        return Some(("hartmetall", "Widiaplättchen"));
    }
    // Bare "Bohrer" is unattributable (Hartmetall vs. HSS) → None.
    // Katalysatoren (live "auf Anfrage", mapped for when priced).
    if l.contains("katalysator") {
        if l.contains("klein") {
            return Some(("katalysatoren", "klein"));
        }
        if l.contains("groß") || l.contains("gross") {
            return Some(("katalysatoren", "groß"));
        }
        return Some(("katalysatoren", ""));
    }
    // Blei & Zink.
    if l.contains("schälblei") || l.contains("schaelblei") {
        return Some(("blei-auswucht", "Schälblei"));
    }
    if l.contains("auswuchtblei") {
        return Some(("blei-auswucht", "Auswuchtblei"));
    }
    if l.contains("blei") {
        return Some(("blei", ""));
    }
    if l.contains("zink") {
        return Some(("zink", ""));
    }
    // Eisen & Mischschrott.
    if l.contains("sorte 3") || l.contains("s3") {
        return Some(("stahlschrott-scheren", "S3"));
    }
    if l.contains("mischschrott") {
        if l.contains("leicht") || l.contains("waschmaschinen") {
            return Some(("mischschrott", "leicht"));
        }
        return Some(("mischschrott", ""));
    }
    if l.contains("guss") || l.contains("guß") {
        if l.contains("bremsscheiben") {
            return Some(("eisenschrott-gussbruch", "Bremsscheiben"));
        }
        return Some(("eisenschrott-gussbruch", ""));
    }
    if l.contains("bremsscheiben") {
        return Some(("eisenschrott-gussbruch", "Bremsscheiben"));
    }
    if l.contains("vorbrenner") {
        return Some(("stahlschrott-scheren", "Vorbrenner"));
    }
    if l.contains("moniereisen") {
        if l.contains("fertig") {
            return Some(("stahlschrott-scheren", "Moniereisen fertig"));
        }
        return Some(("stahlschrott-scheren", "Moniereisen"));
    }
    // E-Motoren.
    if l.contains("e-motor") || l.contains("e motor") || l.contains("emotor") {
        if l.contains("getriebe") {
            return Some(("elektromotoren", "mit Getriebe"));
        }
        return Some(("elektromotoren", ""));
    }
    // Batterien and paper have no catalog material.
    None
}

/// Bespoke contact extraction for THIS impressum only: the first
/// `<address>` after the "Angaben gemäß § 5 DDG" heading is the Rathenow
/// seat (the Diensteanbieter named below the addresses), and the `<p>`
/// after the "Kontakt" heading carries the `tel:`/`mailto:` links.
/// Missing anchors mean the page changed shape → loud error.
/// Kontakt der Annahmestelle aus ihrer Standortseite: die Definition-Liste
/// `dl.location-meta` führt je Zeile „Adresse“, „Telefon“ und „E-Mail“. Fehlt
/// ein Feld oder ist die Adresse nicht lesbar, endet der Lauf laut — eine
/// halb gelesene Kontaktseite ist kein Kontakt.
fn extract_site_info(html: &str, url: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(html);
    let dt_sel = Selector::parse("dt").expect("valid selector");
    let mut address = String::new();
    let mut phone = String::new();
    let mut email = String::new();
    for dt in doc.select(&dt_sel) {
        let key = dt.text().collect::<String>().trim().to_lowercase();
        let Some(dd) = dt
            .next_siblings()
            .filter_map(ElementRef::wrap)
            .find(|e| e.value().name() == "dd")
        else {
            continue;
        };
        // The address cell carries a muted hint after a <br> ("im Businesspark",
        // "nähe KFZ-Zulassungsstelle") — that is not part of the address.
        let value = cell_text(&dd.inner_html());
        match key.as_str() {
            "adresse" => address = value,
            // "0152 59689044 (mobil)" — die Mobil-Anmerkung ist keine
            // Teil der Rufnummer.
            "telefon" => phone = strip_mobil_hint(&value),
            "e-mail" => email = value,
            _ => {}
        }
    }
    if address.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    }
    let (street, postcode, city) = split_address(&address).ok_or_else(|| IngestError::Parse {
        url: url.to_owned(),
        detail: "Adresszeile unlesbar".to_owned(),
    })?;
    if street.is_empty() || postcode.is_empty() || city.is_empty() {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: "Adresszeile unvollständig".to_owned(),
        });
    }
    if phone.is_empty() || email.is_empty() {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: "Kontaktdaten unvollständig".to_owned(),
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

/// Text einer Tabellenzelle ohne den nachfolgenden Hinweis-Absatz und ohne
/// Tags.
fn cell_text(html: &str) -> String {
    let head = html.split("<br").next().unwrap_or_default();
    let text = strip_fragment(head);
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Trailing "(mobil)"-Anmerkung aus einer Telefonzelle entfernen.
fn strip_mobil_hint(phone: &str) -> String {
    let trimmed = phone.trim();
    match trimmed.strip_suffix("(mobil)") {
        Some(without) => without.trim().to_owned(),
        None => trimmed.to_owned(),
    }
}

/// "Kummerlestraße 1A, 14770 Brandenburg an der Havel" → (street, PLZ, Ort).
/// Die Stadt kann mehrere Wörter haben ("Brandenburg an der Havel").
fn split_address(address: &str) -> Option<(String, String, String)> {
    let (street, rest) = address.split_once(',')?;
    let rest = rest.trim();
    let (postcode, city) = rest.split_once(' ')?;
    let postcode = postcode.trim();
    if postcode.len() != 5 || !postcode.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let city = city.trim();
    if city.is_empty() {
        return None;
    }
    Some((
        street.trim().to_owned(),
        postcode.to_owned(),
        city.to_owned(),
    ))
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

/// Page date from the hero lede ("… Stand: 22.09.2026."). Missing date →
/// `None` (provenance only, never an error).
fn date_in(html: &str) -> Option<String> {
    let i = html.find("Stand:")?;
    let tail = html[i + "Stand:".len()..].trim_start();
    let mut parts = tail.split('.');
    let day = parts.next()?.trim();
    let month = parts.next()?.trim();
    let year: String = parts
        .next()?
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    parse_de_date(day, month, &year)
}

/// One price row of the price tables.
#[derive(Debug)]
struct Row {
    /// Sort label without the description span.
    label: String,
    /// Price of the visible cell (the set shared by the five Brandenburg sites).
    price: f64,
    /// Page-global unit per table (always kg on this page).
    unit: &'static str,
    /// Price of the Rostock set, if the row carries one.
    rostock_price: Option<f64>,
    /// Locations the row is offered at (`data-only`), if restricted.
    only: Option<Vec<String>>,
}

fn parse(html: &str) -> Result<(Option<String>, Vec<Row>, Vec<String>), IngestError> {
    let published_at = date_in(html);
    let doc = Html::parse_document(html);
    let table_sel = Selector::parse("table.price-table").expect("valid selector");
    let row_sel = Selector::parse("tbody tr").expect("valid selector");
    let th_sel = Selector::parse("th").expect("valid selector");
    let head_th_sel = Selector::parse("thead th").expect("valid selector");
    let td_sel = Selector::parse("td").expect("valid selector");
    let tables: Vec<_> = doc.select(&table_sel).collect();
    if tables.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabellen".to_owned(),
        });
    }
    // Page-global unit per table: caption ("Angaben in Euro pro
    // Kilogramm") and column header ("Preis €/kg") both say kg. A table
    // whose price header stops saying kg fails loudly — a per-tonne
    // price recorded as per-kg would be a 1000x error.
    const PAGE_UNIT: &str = "EUR/kg";
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for table in tables {
        let head: Vec<String> = table
            .select(&head_th_sel)
            .map(|h| h.text().collect())
            .collect();
        let head = head.join(" ").to_lowercase();
        if !head.contains("bezeichnung") || !head.contains("preis") {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: "Preistabelle ohne Kopf".to_owned(),
            });
        }
        if !head.contains("kg") {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: "Einheitenkopf geändert (kein Kilo-Preis)".to_owned(),
            });
        }
        for tr in table.select(&row_sel) {
            let Some(th) = tr.select(&th_sel).next() else {
                continue;
            };
            // First text node only: the <span class="item-desc">
            // description must not glue onto the label ("Millberry"+
            // "99,9 %").
            let label = th
                .text()
                .find(|t| !t.trim().is_empty())
                .unwrap_or_default()
                .replace(['\u{a0}'], " ");
            let label = label.split_whitespace().collect::<Vec<_>>().join(" ");
            if label.is_empty() || label == "Bezeichnung" {
                continue;
            }
            let price_raw: String = tr
                .select(&td_sel)
                .next()
                .map(|td| td.text().collect())
                .unwrap_or_default();
            let Some(price) = parse_eur(&price_raw) else {
                skips.push(format!("{label} (kein Preis: {})", price_raw.trim()));
                continue;
            };
            let rostock_price = tr
                .value()
                .attr("data-price-ros")
                .and_then(|value| value.trim().parse::<f64>().ok());
            let only = tr.value().attr("data-only").map(|value| {
                value
                    .split_whitespace()
                    .map(|part| part.to_lowercase())
                    .collect::<Vec<_>>()
            });
            rows.push(Row {
                label,
                price,
                unit: PAGE_UNIT,
                rostock_price,
                only,
            });
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabellen leer".to_owned(),
        });
    }
    Ok((published_at, rows, skips))
}

#[cfg(test)]
mod tests {
    use super::{
        date_in, extract_site_info, grade_for, parse, SITES, SLUG, SLUG_NAUEN, SLUG_PLOTZIN,
        SLUG_RATHENOW, SLUG_RECKAHN, SLUG_ROSTOCK,
    };

    // Real shape (live 27.09.2026): <table class="price-table"> with
    // <caption>, <thead> "Bezeichnung" / "Preis €/kg", and
    // <th scope="row">Label<span class="item-desc">…</span></th> rows.
    const FIXTURE: &str = "<h1>Ankaufspreise</h1>\
        <p class=\"lede\">Unsere komplette Annahmepreisliste mit bis zu 60 Positionen. \
        Stand: 22.09.2026.</p>\
        <table class=\"price-table\">\
        <caption class=\"visually-hidden\">Ankaufspreise Kupfer, Angaben in Euro pro Kilogramm</caption>\
        <thead><tr><th scope=\"col\">Bezeichnung</th>\
        <th scope=\"col\" class=\"col-price\">Preis €/kg</th></tr></thead><tbody>\
        <tr data-search=\"kupfer millberry\"><th scope=\"row\">Kupfer Millberry\
        <span class=\"item-desc\">99,9 %, Drähte aus Kabeln stärker als 1 mm</span></th>\
        <td class=\"col-price\">10,50 €</td></tr>\
        <tr data-search=\"kupfer kabel duenn\"><th scope=\"row\">Kupfer Kabel dünn\
        <span class=\"item-desc\">bis 10 mm² Querschnitt</span></th>\
        <td class=\"col-price\">3,00 €</td></tr>\
        <tr data-search=\"chromstahl\"><th scope=\"row\">Chromstahl\
        <span class=\"item-desc\">Stahl mit über 10 % Chromanteil</span></th>\
        <td class=\"col-price\">auf Anfrage</td></tr>\
        </tbody></table>";

    #[test]
    fn tables_date_and_requests_parse() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-22T00:00:00+00:00"));
        assert_eq!(rows.len(), 2, "{rows:?}");
        // Label without the glued item-desc.
        assert_eq!(rows[0].label, "Kupfer Millberry");
        assert_eq!(rows[0].price, 10.5);
        assert_eq!(rows[0].unit, "EUR/kg");
        assert_eq!(rows[0].rostock_price, None);
        assert_eq!(rows[0].only, None);
        assert_eq!(rows[1].label, "Kupfer Kabel dünn");
        assert_eq!(rows[1].price, 3.0);
        assert_eq!(skips.len(), 1, "{skips:?}");
        assert!(skips[0].contains("Chromstahl") && skips[0].contains("auf Anfrage"));
        assert!(date_in("ohne Datum").is_none());
    }

    #[test]
    fn header_and_unit_guards_fail_loudly() {
        // Calculator selects/inputs are no tables: untouched by design.
        let html = "<select id=\"calc-material\"></select>".to_owned() + FIXTURE;
        let (_, rows, _) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 2);
        // No price tables at all: loud error.
        assert!(parse("<html><body><p>Neu</p></body></html>").is_err());
        // Unit basis changed: loud error, not silent 1000x prices.
        let html = FIXTURE.replace("Preis €/kg", "Preis €/t");
        assert!(parse(&html).is_err());
        // Every row unpriceable: loud error, not silent success.
        let html = FIXTURE
            .replace("10,50 €", "auf Anfrage")
            .replace("3,00 €", "auf Anfrage");
        assert!(parse(&html).is_err());
    }

    /// Live shape of the site rows (27.10.2026): the visible cell carries the
    /// price of the five Brandenburg sites, `data-price-ros` the Rostock set and
    /// `data-only` restricts a sort to named locations.
    const FIXTURE_SITES: &str = "<h1>Ankaufspreise</h1>\
        <p class=\"lede\">Stand: 23.10.2026.</p>\
        <table class=\"price-table\">\
        <caption class=\"visually-hidden\">Ankaufspreise Kupfer, Angaben in Euro pro Kilogramm</caption>\
        <thead><tr><th scope=\"col\">Bezeichnung</th>\
        <th scope=\"col\" class=\"col-price\">Preis €/kg</th></tr></thead><tbody>\
        <tr data-price-ros=\"10.5\"><th scope=\"row\">Kupfer Millberry\
        <span class=\"item-desc\">99,9 %</span></th>\
        <td class=\"col-price\">10,50 €</td></tr>\
        <tr data-price-ros=\"0.14\"><th scope=\"row\">Altschrott Sorte 3</th>\
        <td class=\"col-price\">0,17 €</td></tr>\
        <tr data-only=\"brandenburg rathenow nauen\" data-price-ros=\"0.1\">\
        <th scope=\"row\">Papier</th><td class=\"col-price\">0,10 €</td></tr>\
        </tbody></table>";

    #[test]
    fn site_rows_carry_rostock_price_and_only_restriction() {
        let (_, rows, _) = parse(FIXTURE_SITES).expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].rostock_price, Some(10.5));
        assert_eq!(rows[0].only, None);
        // Rostock prices its own set: 0,14 instead of the visible 0,17.
        assert_eq!(rows[1].price, 0.17);
        assert_eq!(rows[1].rostock_price, Some(0.14));
        // "Papier" is restricted to three locations.
        let only = rows[2].only.as_ref().expect("data-only present");
        assert_eq!(
            only,
            &vec![
                "brandenburg".to_owned(),
                "rathenow".to_owned(),
                "nauen".to_owned()
            ]
        );
    }

    #[test]
    fn every_site_has_its_own_slug_and_price_set() {
        assert_eq!(SITES.len(), 6);
        let slugs: Vec<&str> = SITES.iter().map(|(_, slug, _, _)| *slug).collect();
        assert!(slugs.contains(&SLUG));
        assert!(slugs.contains(&SLUG_RATHENOW));
        assert!(slugs.contains(&SLUG_NAUEN));
        assert!(slugs.contains(&SLUG_RECKAHN));
        assert!(slugs.contains(&SLUG_PLOTZIN));
        assert!(slugs.contains(&SLUG_ROSTOCK));
        // Only Rostock has its own price set.
        assert_eq!(
            SITES.iter().filter(|(_, _, _, rostock)| *rostock).count(),
            1
        );
    }

    /// Live shape of a location page (27.10.2026): the definition list
    /// `dl.location-meta` with "Adresse", "Telefon" and "E-Mail" rows; the
    /// address cell carries a muted hint after a <br>.
    const FIXTURE_SITE_PAGE: &str = r#"<article class="location-card" data-location="rostock">
          <div class="location-head"><h2>Kontakt &amp; Anfahrt</h2></div>
          <dl class="location-meta">
            <div>
<dt>Adresse</dt>
<dd>Werftstraße 20, 18057 Rostock<br>
<span class="muted">im Businesspark</span>
</dd>
</div>
            <div>
<dt>Telefon</dt>
<dd>
<a href="tel:+4938180876177">0381 80876177</a>
</dd>
</div>
            <div>
<dt>E-Mail</dt>
<dd>
<a href="mailto:rostock@mcschrott.de">rostock@mcschrott.de</a>
</dd>
</div>
          </dl>
        </article>"#;

    #[test]
    fn location_page_contact_parses() {
        let info = extract_site_info(
            FIXTURE_SITE_PAGE,
            "https://www.mcschrott.de/standorte/rostock/",
        )
        .expect("parses");
        assert_eq!(info.street, "Werftstraße 20");
        assert_eq!(info.postcode, "18057");
        assert_eq!(info.city, "Rostock");
        assert_eq!(info.phone, "0381 80876177");
        assert_eq!(info.email, "rostock@mcschrott.de");
    }

    #[test]
    fn mobil_hint_is_stripped_from_the_phone() {
        let page = FIXTURE_SITE_PAGE.replace("0381 80876177", "0152 59689044 (mobil)");
        let info =
            extract_site_info(&page, "https://www.mcschrott.de/standorte/nauen/").expect("parses");
        assert_eq!(info.phone, "0152 59689044");
    }

    #[test]
    fn location_page_without_contact_fails_loudly() {
        let url = "https://www.mcschrott.de/standorte/rostock/";
        // kein Kontakt-Block
        assert!(extract_site_info("<main><p>Neu hier</p></main>", url).is_err());
        // Adresse ohne PLZ
        assert!(extract_site_info(
            "<dl class=\"location-meta\"><div><dt>Adresse</dt><dd>Werftstraße 20</dd></div>\
             <div><dt>Telefon</dt><dd>0381 80876177</dd></div>\
             <div><dt>E-Mail</dt><dd>rostock@mcschrott.de</dd></div></dl>",
            url
        )
        .is_err());
        // Telefon fehlt
        assert!(extract_site_info(
            "<dl class=\"location-meta\"><div><dt>Adresse</dt><dd>Werftstraße 20, 18057 Rostock</dd></div>\
             <div><dt>E-Mail</dt><dd>rostock@mcschrott.de</dd></div></dl>",
            url
        )
        .is_err());
    }

    #[test]
    fn mapping_covers_every_arm() {
        let cases: &[(&str, Option<(&str, &str)>)] = &[
            ("Kupfer Millberry", Some(("kupfer-millberry", ""))),
            ("Kupfer Raff", Some(("kupfer-gemischt", "Raff"))),
            ("Kupfer Kabel dünn", Some(("kabel-kupfer", "dünn"))),
            ("Kupfer Kerze", Some(("kupfer-berry", "Kerze"))),
            ("Kupfer Kabel dick", Some(("kabel-kupfer", "dick"))),
            (
                "Kupfer Leitschiene",
                Some(("kupfer-gemischt", "Leitschiene")),
            ),
            ("Kupfer Messing Kühler", None),
            ("Kupfer Alu Kühler", None),
            ("Kupfer Berry", Some(("kupfer-berry", ""))),
            ("Kupfer Späne", Some(("kupfer-gemischt", "Späne"))),
            ("Kupfer Eisen Kühler", None),
            (
                "Kupfer Eisen Schrott",
                None, // Cu-Fe-Verbund, kein FE-Misch (FE-Audit)
            ),
            ("Kupfer Blei Kabel", Some(("kabel-blei", "Blei"))),
            (
                "Kupfer Kabel mit Eisenmantel",
                Some(("kabel-kupfer", "Eisenmantel")),
            ),
            (
                "Kupfer Kabel mit Stecker",
                Some(("kabel-mit-stecker", "mit Stecker")),
            ),
            (
                "Alu Draht luftgeschwärzt",
                Some(("aluminium-gemischt", "Draht luftgeschwärzt")),
            ),
            (
                "Alu Draht blank",
                Some(("aluminium-gemischt", "Draht blank")),
            ),
            ("Alu Gemisch sauber", Some(("aluminium-gemischt", ""))),
            (
                "Alu Gemisch, 2–5 % Anhaftung",
                Some(("aluminium-gemischt", "2-5% Anhaftung")),
            ),
            ("Alu Guss sauber", Some(("aluminium-guss", ""))),
            (
                "Alu Guss, 2–5 % Anhaftung",
                Some(("aluminium-guss", "2-5% Anhaftung")),
            ),
            ("Alu Kabel dünn", Some(("kabel-alu", "dünn"))),
            ("Alu Kabel dick", Some(("kabel-alu", "dick"))),
            (
                "Alu Leitschiene",
                Some(("aluminium-gemischt", "Leitschiene")),
            ),
            ("Alu Profile blank", Some(("aluminium-profile", "blank"))),
            (
                "Alu Profile farbig, ohne Anhaftung",
                Some(("aluminium-profile", "farbig")),
            ),
            (
                "Alu Profile, Anhaftung max. 2 %",
                Some(("aluminium-profile", "2% Anhaftung")),
            ),
            ("Alu Kupfer Kabel", Some(("kabel-alu", "CCA"))),
            (
                "Alu Felgen sauber",
                Some(("aluminium-guss", "Felgen sauber")),
            ),
            (
                "Alu Felgen unsauber",
                Some(("aluminium-guss", "Felgen unsauber")),
            ),
            ("Alu Kühler", None),
            ("Alu Draht mit Stahlseele", None),
            ("Alu Iso-Profile", None),
            ("Alu Späne", Some(("aluminium-gemischt", "Späne"))),
            ("Messing", Some(("messing", ""))),
            ("Messing Späne", Some(("messing", "Späne"))),
            ("V2A Stahl", Some(("edelstahl-v2a", ""))),
            ("V4A Stahl", Some(("edelstahl-v4a", ""))),
            ("V2A Späne", Some(("edelstahl-v2a", "Späne"))),
            ("Chromstahl", Some(("edelstahl-gemischt", "Chromstahl"))),
            ("Widiaplättchen", Some(("hartmetall", "Widiaplättchen"))),
            ("Bohrer", None),
            ("Zink alt/neu", Some(("zink", ""))),
            ("Blei", Some(("blei", ""))),
            ("Schälblei", Some(("blei-auswucht", "Schälblei"))),
            ("Auswuchtblei", Some(("blei-auswucht", "Auswuchtblei"))),
            ("Altschrott Sorte 3", Some(("stahlschrott-scheren", "S3"))),
            ("Mischschrott", Some(("mischschrott", ""))),
            ("Gussschrott", Some(("eisenschrott-gussbruch", ""))),
            (
                "Bremsscheiben",
                Some(("eisenschrott-gussbruch", "Bremsscheiben")),
            ),
            (
                "Vorbrennermaterial",
                Some(("stahlschrott-scheren", "Vorbrenner")),
            ),
            (
                "Moniereisen fertig",
                Some(("stahlschrott-scheren", "Moniereisen fertig")),
            ),
            ("Moniereisen", Some(("stahlschrott-scheren", "Moniereisen"))),
            (
                "Mischschrott leicht / Waschmaschinen",
                Some(("mischschrott", "leicht")),
            ),
            ("E-Motoren", Some(("elektromotoren", ""))),
            (
                "E-Motoren mit Getriebe",
                Some(("elektromotoren", "mit Getriebe")),
            ),
            ("Batterien", None),
            ("Papier", None),
            ("Katalysator klein", Some(("katalysatoren", "klein"))),
            ("Katalysator groß", Some(("katalysatoren", "groß"))),
        ];
        for (label, want) in cases {
            assert_eq!(&grade_for(label), want, "{label}");
        }
    }
}
