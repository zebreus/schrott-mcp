//! DB Recycling / Dein-Schrottplatz (Rochlitz, Inh. Denis Blum): exact
//! purchase prices ("Preisliste Schrott und Buntmetall") in one plain
//! two-column table (`Schrottart` / `Vergütung`). Section subheads repeat
//! "Preis/kg", so bare "9,00 €" cells are quoted per kg; only the
//! disposal rows use "/t" spellings. One combined row ("Buntmetall" |
//! "Hartmetall 60,00€/kg") carries material and price in a single cell.
//! The last row ("Gültig ab 09.09.2026") dates the whole list. Disposal
//! fees, "Annahmestopp" rows, PC parts, paper and textiles have no
//! catalog material and are skipped loudly.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sn-rochlitz-db-recycling-denis-blum";
/// Bespoke, live-verified impressum URL (site footer "Impressum"). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.dein-schrottplatz.de/pages/kontakt/impressum.php";

pub const URL: &str = "https://www.dein-schrottplatz.de/pages/preise.php";

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
/// skipped. Notes on judgement calls:
/// - "Chromstahl" is unstamped stainless → generic edelstahl-gemischt.
/// - "Al Blech/Guß mit 2% Fe" mixes sheet and cast; the generic parent
///   covers the mix, the Fe share stays in the variant.
/// - Disposal rows (Sperrmüll, Bauschutt, …), Transport, tires, PC
///   parts, paper/textiles have no catalog material → None.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // No-catalog guards first (proposal material, never crammed).
    if l.contains("kühler") || l.contains("kuehler") {
        return None;
    }
    if l.contains("batterie") || l.contains("bleiakku") {
        return None;
    }
    if l.contains("transport")
        || l.contains("sperrmüll")
        || l.contains("sperrmuell")
        || l.contains("autoreifen")
        || l.contains("reifen")
        || l.contains("bauschutt")
        || l.contains("grünschnitt")
        || l.contains("gruenschnitt")
        || l.contains("altholz")
        || l.contains("entsorgung")
    {
        debug_assert!(is_service(l));
        return None;
    }
    if l.contains("textil") || l.contains("schuhe") || l.contains("papier") {
        return None;
    }
    if l.contains("pc-komplett") || l.contains("pc komplett") {
        return None;
    }
    if l.contains("laufwerk") || l.contains("festplatte") || l.contains("netzteil") {
        return None;
    }
    if l.contains("gültig ab") || l.contains("gueltig ab") {
        return None;
    }
    if l.contains("hartmetall") {
        Some(("hartmetall", ""))
    } else if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("lackdraht") || l.contains("berry") {
        Some(("kupfer-berry", ""))
    } else if l.contains("cu raff") {
        Some(("kupfer-gemischt", "Raff 92%"))
    } else if l.contains("cu schwer") {
        Some(("kupfer-gemischt", "schwer"))
    } else if l.contains("oberleitung") {
        Some(("kupfer-gemischt", "Oberleitung/Freileitung"))
    } else if l.contains("schlitzkabel") && l.contains("cu") {
        Some(("kabel-kupfer", "60%"))
    } else if l.contains("shredderkabel") && l.contains("cu") {
        Some(("kabel-kupfer", "Shredder"))
    } else if l.contains("schlitzkabel") {
        Some(("kabel-alu", ""))
    } else if l.contains("shredderkabel") {
        Some(("kabel-alu", "Shredder"))
    } else if l.contains("leiterplatte") && l.contains("klasse 1") {
        Some(("platinen", "Klasse 1"))
    } else if l.contains("leiterplatte") && l.contains("klasse 3") {
        Some(("platinen", "Klasse 3"))
    } else if l.contains("reinzinn") || l.trim() == "zinn" {
        Some(("zinn", ""))
    } else if l.contains("ms raff sp") {
        Some(("messing", "Raff Späne"))
    } else if l.contains("ms raff") {
        Some(("messing", "Raff"))
    } else if l.contains("ms schwer") {
        Some(("messing", "schwer"))
    } else if l.contains("al profil") {
        Some(("aluminium-profile", "blank"))
    } else if l.contains("al felgen") || l.contains("felgen") {
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("al getriebe") || (l.contains("getriebe") && !l.contains("motor")) {
        Some(("aluminium-guss", "Getriebe"))
    } else if l.contains("blech") {
        Some(("aluminium-gemischt", "Blech/Guß 2% Fe"))
    } else if l.contains("al draht") {
        Some(("aluminium-gemischt", "Draht blank"))
    } else if l.contains("freileitung") {
        Some(("aluminium-gemischt", "Freileitung o. Fe"))
    } else if l.contains("wucht") && l.contains("kabel") {
        Some(("blei", "Wucht-/Kabelblei"))
    } else if l.contains("altblei") {
        Some(("blei", ""))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", ""))
    } else if l.contains("chromstahl") {
        Some(("edelstahl-gemischt", ""))
    } else if l.contains("getriebemotoren") {
        Some(("elektromotoren", "Getriebe"))
    } else if l.contains("e-motor") || l.contains("emotor") {
        Some(("elektromotoren", ""))
    } else if l.contains("schwerer scherenschrott") {
        Some(("stahlschrott-scheren", "schwer"))
    } else if l.contains("leichter scherenschrott") {
        Some(("stahlschrott-scheren", "leicht"))
    } else if l.contains("kernschrott") {
        Some(("stahlschrott-scheren", "Kernschrott"))
    } else if l.contains("gußschrott") || l.contains("gussschrott") {
        Some(("eisenschrott-gussbruch", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("shreddervormaterial") || l.contains("schreddervormaterial") {
        Some(("stahlschrott-shredder", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: `<h3>DB
/// Recycling</h3>` heads an `<address>` (Inhaber / street / PLZ city via
/// `<br>`), followed by labeled Telefon/Fax/Mobil and Internet/E-Mail
/// `<p>` rows. Missing anchors mean the page changed shape → loud
/// error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h3 = Selector::parse("h3").expect("valid selector");
    let addr = Selector::parse("address").expect("valid selector");
    let anchor = doc
        .select(&h3)
        .find(|h| h.text().collect::<String>().trim() == "DB Recycling");
    let Some(_) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "DB-Recycling-Block fehlt".to_owned(),
        });
    };
    let addr_el = doc.select(&addr).next();
    let Some(addr_el) = addr_el else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    // <br> rows: "Inhaber: Denis Blum" / "Schützenstr. 6" / "09306 Rochlitz".
    let mut lines = Vec::new();
    for part in addr_el.inner_html().split("<br") {
        let t = strip_tags(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for line in &lines {
        if line.contains("Inhaber:") {
            continue;
        }
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| a + " " + b);
                continue;
            }
        }
        if street.is_empty() && line.chars().any(|c| c.is_ascii_digit()) {
            street = line.clone();
        }
    }
    // Phone + mail from the whole page text (labeled rows).
    let all: String = doc.root_element().text().collect();
    let flat = all.split_whitespace().collect::<Vec<_>>().join(" ");
    let phone = {
        let after = after_marker(&flat, "Telefon:");
        // Cut at the next label: scraper text() glues "15"+"Mobil:" into
        // one token and the digit filter would stop one token early.
        let end = ["Mobil:", "Telefax:", "Fax:", "E-Mail:"]
            .iter()
            .filter_map(|m| after.find(m))
            .min()
            .unwrap_or(after.len());
        after[..end]
            .split_whitespace()
            .take_while(|t| {
                t.chars()
                    .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
            })
            .collect::<Vec<_>>()
            .join(" ")
    };
    // E-mail needs its own rule: the phone-style filter stops at the
    // first letter, so take the @ token instead.
    let email = flat
        .split_whitespace()
        .find(|t| t.contains('@'))
        .unwrap_or_default()
        .trim_matches([',', ';', '.'])
        .to_owned();
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

fn after_marker<'a>(text: &'a str, marker: &str) -> &'a str {
    text.find(marker)
        .map(|i| &text[i + marker.len()..])
        .unwrap_or("")
}

/// Strip tags from a fragment (entities are already decoded by html5ever).
fn strip_tags(s: &str) -> String {
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
    let doc = Html::parse_document(html);
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    // Never trust page order: take the table carrying the "Schrottart"
    // header cell, not just the first <table> on the page.
    let table = doc.select(&table).find(|t| {
        t.select(&cell).any(|c| {
            c.text()
                .collect::<String>()
                .trim()
                .trim_start_matches('\u{feff}')
                == "Schrottart"
        })
    });
    let Some(table) = table else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    };
    let mut published_at = None;
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in table.select(&row) {
        let cells: Vec<String> = tr.select(&cell).map(|c| c.text().collect()).collect();
        if cells.len() < 2 {
            continue;
        }
        let label = cells[0].replace(['\u{a0}', '\u{feff}'], " ");
        let label = label.split_whitespace().collect::<Vec<_>>().join(" ");
        let price_raw = cells[1].replace('\u{a0}', " ");
        let price_trim = price_raw.trim().to_owned();
        // The date row ("Gültig ab 09.09.2026" | empty) dates the list —
        // structural, and the page's only date.
        if label.to_lowercase().starts_with("gültig ab")
            || label.to_lowercase().starts_with("gueltig ab")
        {
            let parts: Vec<&str> = label.split_whitespace().collect();
            if let Some(date) = parts.last() {
                let d: Vec<&str> = date.split('.').collect();
                if d.len() == 3 {
                    published_at = parse_de_date(d[0], d[1], d[2]);
                }
            }
            continue;
        }
        if label.eq_ignore_ascii_case("schrottart") {
            continue;
        }
        // Label-only section headers ("Kupfer" | "Preis/kg", "Aluminium"
        // | …) carry no price — structural, not skips. The repeated
        // "Preis/kg" also documents the default unit below.
        if price_trim.is_empty() || price_trim.eq_ignore_ascii_case("preis/kg") {
            continue;
        }
        if label.is_empty() {
            continue;
        }
        // Disposal fees are quoted "Netto 0,3= 300€/t" — parse_eur would
        // take the leading 0,3, so these rows skip before any number
        // (and before the service check: the fee reason is precise).
        if price_trim.to_lowercase().contains("netto") {
            skips.push(format!("{label} (Entsorgungskosten, kein Ankauf)"));
            continue;
        }
        // Service and disposal rows (Transport, Autoreifen, Sperrmüll,
        // …) are fees, not purchase prices — loud skip before any number.
        if is_service(&label.to_lowercase()) {
            skips.push(format!("{label} (Dienstleistung/Entsorgung, kein Ankauf)"));
            continue;
        }
        let Some(price) = parse_eur(&price_trim) else {
            skips.push(format!("{label} (kein Preis: {price_trim})"));
            continue;
        };
        if price == 0.0 {
            skips.push(format!("{label} (0,00 — kein Ankauf)"));
            continue;
        }
        // Combined cell: "Buntmetall" | "Hartmetall 60,00€/kg" names the
        // material inside the price column.
        let (label, price_raw) = if price_trim.contains("Hartmetall") {
            ("Hartmetall".to_owned(), price_trim.clone())
        } else {
            (label, price_trim.clone())
        };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let Some(unit) = unit_of(&price_raw) else {
            skips.push(format!("{label} (Einheit unverständlich: {price_trim})"));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    Ok((published_at, rows, skips))
}

/// Bespoke service guard for THIS table: fees and disposal lines are
/// not purchase prices. Kept in one fn so parse() and grade_for()
/// agree; grade_for keeps its own arms as backstop.
fn is_service(l: &str) -> bool {
    l.contains("transport")
        || l.contains("sperrmüll")
        || l.contains("sperrmuell")
        || l.contains("autoreifen")
        || l.contains("reifen")
        || l.contains("bauschutt")
        || l.contains("grünschnitt")
        || l.contains("gruenschnitt")
        || l.contains("altholz")
        || l.contains("entsorgung")
}

/// Bespoke unit matcher for THIS table: section subheads repeat
/// "Preis/kg", so bare "9,00 €" cells are quoted per kg (Fe rows at
/// 0,05–0,18 € are only plausible per kg ≈ 50–180 €/t). Explicit "/t"
/// still wins when present. Anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("/kg") {
        Some("EUR/kg")
    } else if lower.contains("/t") {
        Some("EUR/t")
    } else if lower.contains('€') || lower.contains("eur") {
        Some("EUR/kg")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    // Real table shape, condensed: BOM in the header cell, section
    // subheads, the combined Hartmetall cell, Annahmestopp/Netto rows,
    // and the trailing date row.
    const FIXTURE: &str = "<table><tr class=\"row_1\"><td class=\"col_0\">\u{feff}Schrottart</td>\
        <td class=\"col_1\">Vergütung</td></tr>\
        <tr class=\"row_2\"><td class=\"col_0\">Buntmetall</td><td class=\"col_1\">Hartmetall 60,00€/kg</td></tr>\
        <tr class=\"row_3\"><td class=\"col_0\">Kupfer</td><td class=\"col_1\"> Preis/kg</td></tr>\
        <tr class=\"row_4\"><td class=\"col_0\">Cu Raff 92%</td><td class=\"col_1\">9,00 €</td></tr>\
        <tr class=\"row_8\"><td class=\"col_0\">Cu Millberry</td><td class=\"col_1\">11,00 €</td></tr>\
        <tr class=\"row_9\"><td class=\"col_0\">Messing</td><td class=\"col_1\"> Preis/kg</td></tr>\
        <tr class=\"row_13\"><td class=\"col_0\">Cu-Ms Kühler o. Fe</td><td class=\"col_1\">4,80 €</td></tr>\
        <tr class=\"row_21\"><td class=\"col_0\">Reinzinn</td><td class=\"col_1\">30,00 €</td></tr>\
        <tr class=\"row_24\"><td class=\"col_0\">Altblei</td><td class=\"col_1\">1,20 €</td></tr>\
        <tr class=\"row_28\"><td class=\"col_0\">V2A</td><td class=\"col_1\">0,80 €</td></tr>\
        <tr class=\"row_33\"><td class=\"col_0\">Schwerer Scherenschrott</td><td class=\"col_1\">0,15 €</td></tr>\
        <tr class=\"row_44\"><td class=\"col_0\">Leiterplatte Klasse 1</td><td class=\"col_1\">10,00 €</td></tr>\
        <tr class=\"row_45\"><td class=\"col_0\">PC-komplett</td><td class=\"col_1\">0,60 €</td></tr>\
        <tr class=\"row_56\"><td class=\"col_0\">Altpapier</td><td class=\"col_1\">0,08 €</td></tr>\
        <tr class=\"row_57\"><td class=\"col_0\">Textilien</td><td class=\"col_1\">Annahmestopp</td></tr>\
        <tr class=\"row_60\"><td class=\"col_0\">Sperrmüll</td><td class=\"col_1\">Netto 0,3= 300€/t</td></tr>\
        <tr class=\"row_65\"><td class=\"col_0\">Transport</td><td class=\"col_1\">90,00 €</td></tr>\
        <tr class=\"row_66\"><td class=\"col_0\">Gültig ab 09.09.2026</td><td class=\"col_1\"></td></tr>\
        </table>";

    #[test]
    fn table_and_date_parse() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-09T00:00:00+00:00"));
        // 12 price cells minus Transport (service) — the date row and
        // section headers are structural, Annahmestopp/Netto skip loudly.
        assert_eq!(rows.len(), 11, "{rows:?}");
        assert_eq!(skips.len(), 3, "{skips:?}");
        assert!(skips
            .iter()
            .any(|s| s.contains("Textilien") && s.contains("Annahmestopp")));
        assert!(skips
            .iter()
            .any(|s| s.contains("Sperrmüll") && s.contains("Entsorgungskosten")));
        assert!(skips.iter().any(|s| s.contains("Transport")));
        let hart = rows
            .iter()
            .find(|r| r.0 == "Hartmetall")
            .expect("hartmetall");
        assert_eq!((hart.1, hart.2), (60.0, "EUR/kg"));
        let fe = rows
            .iter()
            .find(|r| r.0.contains("Scherenschrott"))
            .expect("fe");
        assert_eq!((fe.1, fe.2), (0.15, "EUR/kg"));
    }

    #[test]
    fn wrong_table_is_rejected_loudly() {
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (_, rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 11);
        assert!(parse("<html><body>keine Tabelle</body></html>").is_err());
        // Every row unparseable: loud error, not silent success.
        let html = FIXTURE
            .replace("€/kg", "pro Sack")
            .replace(" €", " pro Sack");
        let err = parse(&html).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn mapping_covers_every_fixture_label() {
        assert_eq!(
            grade_for("Cu Raff 92%"),
            Some(("kupfer-gemischt", "Raff 92%"))
        );
        assert_eq!(grade_for("Cu Millberry"), Some(("kupfer-millberry", "")));
        assert_eq!(grade_for("Cu Lackdraht/Berry"), Some(("kupfer-berry", "")));
        assert_eq!(grade_for("Hartmetall"), Some(("hartmetall", "")));
        assert_eq!(grade_for("Reinzinn"), Some(("zinn", "")));
        assert_eq!(
            grade_for("Leiterplatte Klasse 1"),
            Some(("platinen", "Klasse 1"))
        );
        assert_eq!(
            grade_for("Leiterplatte Klasse 3"),
            Some(("platinen", "Klasse 3"))
        );
        assert_eq!(
            grade_for("Wucht-/Kabelblei"),
            Some(("blei", "Wucht-/Kabelblei"))
        );
        assert_eq!(
            grade_for("Schwerer Scherenschrott"),
            Some(("stahlschrott-scheren", "schwer"))
        );
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Al-Schlitzkabel"), Some(("kabel-alu", "")));
        // Loud skips: no catalog material or service rows.
        assert_eq!(grade_for("Cu-Ms Kühler o. Fe"), None);
        assert_eq!(grade_for("Batterieblei/Bleiakkus"), None);
        assert_eq!(grade_for("PC-komplett"), None);
        assert_eq!(grade_for("Festplatte"), None);
        assert_eq!(grade_for("Altpapier"), None);
        assert_eq!(grade_for("Textilien"), None);
        assert_eq!(grade_for("Sperrmüll"), None);
        assert_eq!(grade_for("Transport"), None);
        assert_eq!(grade_for("Autoreifen (mit und ohne Felge)"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h3>DB Recycling</h3><address><strong>Inhaber:</strong> Denis Blum<br />\
            Schützenstr. 6<br />09306 Rochlitz</address>\
            <p><strong>Telefon:</strong> (0 37 37) 7 86 43 15<br />\
            <strong>Mobil: </strong>(0162) 8938850</p>\
            <p><strong>E-Mail: </strong>  info@dein-schrottplatz.de</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Schützenstr. 6");
        assert_eq!(info.postcode, "09306");
        assert_eq!(info.city, "Rochlitz");
        assert_eq!(info.phone, "(0 37 37) 7 86 43 15");
        assert_eq!(info.email, "info@dein-schrottplatz.de");
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
