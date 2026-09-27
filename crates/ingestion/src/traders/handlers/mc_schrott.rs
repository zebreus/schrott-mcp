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
//! NOTE: the impressum names the Rathenow seat (Diensteanbieter) while
//! this handler writes the Brandenburg slug — same site, shared list.
//! Proposals (new catalog materials, never crammed): `kuehler-verbund`
//! ("Kupfer Messing Kühler", "Kupfer Alu Kühler", "Kupfer Eisen
//! Kühler", "Alu Kühler"), `alu-verbund-profile` ("Alu Iso-Profile"),
//! `alu-draht-stahlseele`, `batterien` (Starterbatterien/Bleiakkus).
//! "Moniereisen (fertig)" and "Vorbrennermaterial" map to
//! `stahlschrott-scheren` with variants — closest bucket, flagged as
//! uncertain. Bare "Bohrer" stays unmapped: the description says
//! Hartmetall, but unqualified Bohrer could be HSS (cf. Hein's "HSS
//! Bohrer mit Schaft").

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bb-brandenburg-an-der-havel-mc-schrott-annahmestelle";
/// Bespoke, live-verified impressum URL (the site's own footer link
/// "../impressum/" resolved). A move fails the step loudly (fix the
/// URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.mcschrott.de/impressum/";

pub const URL: &str = "https://www.mcschrott.de/preise/";

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
                return Some(("kabel-kupfer", "mit Stecker"));
            }
            if l.contains("blei") {
                return Some(("kabel-kupfer", "Blei"));
            }
            if l.contains("eisenmantel") {
                return Some(("kabel-kupfer", "Eisenmantel"));
            }
            return None;
        }
        if l.contains("eisen") && l.contains("schrott") {
            return Some(("mischschrott", "Kupfer-Eisen"));
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
        return Some(("blei", "Schälblei"));
    }
    if l.contains("auswuchtblei") {
        return Some(("blei", "Auswuchtblei"));
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
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2_sel = Selector::parse("h2").expect("valid selector");
    let anchor = doc.select(&h2_sel).find(|h| {
        h.text()
            .collect::<String>()
            .contains("Angaben gemäß § 5 DDG")
    });
    if anchor.is_none() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "DDG-Block fehlt".to_owned(),
        });
    }
    let addr_sel = Selector::parse("address").expect("valid selector");
    let addr = doc
        .select(&addr_sel)
        .next()
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        })?;
    // ["MC Schrott Rathenow GmbH", "Milower Landstraße 7",
    //  "14712 Rathenow"]: street is the line before the PLZ line.
    let lines: Vec<String> = addr
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5
                && pc.chars().all(|c| c.is_ascii_digit())
                && ci.chars().next().is_some_and(|c| c.is_uppercase())
            {
                postcode = pc.to_owned();
                city = ci.to_owned();
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                break;
            }
        }
    }
    // Phone + mail from the Kontakt section's links (header/footer links
    // must never win, so scope to the <p> after the Kontakt heading).
    let (mut phone, mut email) = (String::new(), String::new());
    if let Some(h) = doc
        .select(&h2_sel)
        .find(|h| h.text().collect::<String>().trim() == "Kontakt")
    {
        let p_sel = Selector::parse("p").expect("valid selector");
        let a_sel = Selector::parse("a[href]").expect("valid selector");
        // The sibling may BE the <p> (flat impressum) or hold it nested.
        if let Some(p) = h
            .next_siblings()
            .filter_map(scraper::ElementRef::wrap)
            .flat_map(|e| {
                let mut v = Vec::new();
                if e.value().name() == "p" {
                    v.push(e);
                }
                v.extend(e.select(&p_sel));
                v
            })
            .next()
        {
            for a in p.select(&a_sel) {
                let href = a.value().attr("href").unwrap_or_default();
                let text: String = a.text().collect();
                let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
                if href.starts_with("tel:") && phone.is_empty() {
                    phone = text;
                } else if href.starts_with("mailto:") && email.is_empty() {
                    email = href.strip_prefix("mailto:").unwrap_or_default().to_owned();
                }
            }
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
            rows.push((label, price, PAGE_UNIT));
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
    use super::{date_in, extract_info, grade_for, parse};

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
        assert_eq!(rows[0], ("Kupfer Millberry".to_owned(), 10.5, "EUR/kg"));
        assert_eq!(rows[1], ("Kupfer Kabel dünn".to_owned(), 3.0, "EUR/kg"));
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

    #[test]
    fn impressum_takes_rathenow_seat_and_kontakt_links() {
        let imp = "<main><h2>Angaben gemäß § 5 DDG</h2>\
            <div class=\"grid grid-2\"><address><strong>MC Schrott Rathenow GmbH</strong><br>\
            Milower Landstraße 7<br>14712 Rathenow</address>\
            <address><strong>MC Schrott Rostock GmbH</strong><br>\
            Werftstraße 20<br>18057 Rostock</address></div>\
            <h2>Kontakt</h2><p>Telefon: <a href=\"tel:+4933818900044\">03381 8900044</a><br>\
            E-Mail: <a href=\"mailto:info@mcschrott.de\">info@mcschrott.de</a></p></main>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Milower Landstraße 7");
        assert_eq!(info.postcode, "14712");
        assert_eq!(info.city, "Rathenow");
        assert_eq!(info.phone, "03381 8900044");
        assert_eq!(info.email, "info@mcschrott.de");
        assert!(extract_info("<main><p>Neu hier</p></main>").is_err());
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
                Some(("mischschrott", "Kupfer-Eisen")),
            ),
            ("Kupfer Blei Kabel", Some(("kabel-kupfer", "Blei"))),
            (
                "Kupfer Kabel mit Eisenmantel",
                Some(("kabel-kupfer", "Eisenmantel")),
            ),
            (
                "Kupfer Kabel mit Stecker",
                Some(("kabel-kupfer", "mit Stecker")),
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
            ("Schälblei", Some(("blei", "Schälblei"))),
            ("Auswuchtblei", Some(("blei", "Auswuchtblei"))),
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
