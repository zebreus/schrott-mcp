//! Metallorum Edelmetallhandels GmbH, Filiale Aschaffenburg: statische
//! An-/Verkaufs-Preisliste (`table.hwt-mp-table` auf
//! `/unser-service/preislisten/`, live 28.09.2026: 88 Produktzeilen =
//! 44 Produkte × An-/Verkauf, je Zeile `data-metal`/`data-direction`/
//! `data-product` plus sichtbare Zellen Produkt/Gewicht/brutto).
//!
//! Nur `data-direction="ankauf"` wird übernommen (das zahlt der Händler
//! aus); jede Verkauf-Zeile skippt laut ("Verkaufspreis, kein Ankauf").
//! Katalogeinheit ist EUR/g, die Seite quotiert Summen je Produkt —
//! `brutto / Gewicht` ist exakte Arithmetik mit zwei gedruckten Zahlen,
//! kein Raten (Gewicht steht in jeder Zeile, alles Gramm).
//!
//! Feingehalt in der Variante: Anlagebarren/-münzen (Gold-Verkauf 0 %
//! MwSt. = Anlagegold), Numismatik-Standard Krügerrand/Eagle 916,
//! Vreneli 900, Philharmoniker/Maple Leaf/Känguru/Buffalo 999,
//! Feinsilber 999. Jede Sorte bekommt eine eigene Variante, damit
//! Produkte nicht auf einen willkürlichen Current-Preis kollabieren.
//!
//! WALLED (bewusst nicht geparst, keine Fakes): der Ankaufsrechner
//! (`/unser-service/ankaufsrechner/`) lädt Tagespreise je Feingehalt
//! (Gold 333–999, Silber, Platin, Palladium) per Preis-API per JS —
//! statisch steht dort überall 0,00 €. Platin/Palladium gibt es nur
//! dort, nicht in der statischen Preisliste (nur Gold/Silber-Tabs).
//!
//! Kein Seitendatum ("alle fünf Minuten aktualisiert", kein Stand) →
//! `published_at = None`. Kontakt kommt von der Filialseite
//! Aschaffenburg ("Anfahrt und Öffnungszeiten"); das rechtliche
//! Impressum nennt nur die Zentrale Unterpleichfeld und darf die
//! Filialadresse nicht überschreiben.

use std::collections::HashSet;

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "by-aschaffenburg-63739-metallorum-edelmetallhandels";
/// Bespoke, live-verified Preislisten-URL (Footer "Preislisten"). Umzug
/// → lauter Step-Fehler, nie raten/teilen.
pub const URL: &str = "https://metallorum.de/unser-service/preislisten/";
/// Bespoke, live-verified Filialseite Aschaffenburg (Kontaktblock
/// "Anfahrt und Öffnungszeiten"). Das Impressum
/// (https://metallorum.de/impressum/) nennt nur die Zentrale
/// Unterpleichfeld und wird bewusst NICHT gelesen.
pub const BRANCH_URL: &str = "https://metallorum.de/verkaufsstellen/aschaffenburg/";

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
    let mut seen = HashSet::new();
    for (label, metal, price_per_g) in rows {
        match grade_for(&metal, &label) {
            Some((material, variant)) => {
                // Gleiche (Material, Variante, Preis)-Triples dedupen
                // (nach dem Mapping, nicht auf Rohlabels).
                let key = (material, variant, price_per_g.to_bits());
                if seen.insert(key) {
                    prices.push(ScrapedPrice {
                        material,
                        variant,
                        price: price_per_g,
                        currency: "EUR",
                        unit: "EUR/g",
                        price_kind: "exact",
                        price_min: None,
                        price_max: None,
                        confidence: Some(1.0),
                        label,
                    });
                }
            }
            None => skipped_labels.push(format!("{label} ({metal}, kein Katalogmaterial)")),
        }
    }
    // Filialseiten-Umzug scheitert laut: neue Kontaktseite braucht
    // Eyeballs, bevor ihr wieder vertraut wird.
    let (_, branch_html) = fetch_text(client, BRANCH_URL).await?;
    let trader_info = extract_info(&branch_html)?;
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

/// Explizite (Metall, Produkt) → (Material, Variante)-Tabelle.
/// `metal` ist der Seiten-`data-metal`-Wert (gold/silver), das Produkt
/// die sichtbare Produktzelle. Spezifisch-vor-generisch: Sonderprägungen
/// (Valcambi, Combi, Vreneli, Buffalo, Eagle) vor den Münz-/Barren-
/// Familien; unbekannte Produkte/Metalle (z. B. künftig Platin) → None.
fn grade_for(metal: &str, product: &str) -> Option<(&'static str, &'static str)> {
    let l = product.to_lowercase();
    let l = l.as_str();
    match metal {
        "gold" => {
            if l.contains("valcambi") {
                Some(("gold", "999-combibar-valcambi-100x1g"))
            } else if l.contains("combibar") || l.contains("tafelbarren") || l.contains("goldtafel")
            {
                Some(("gold", "999-combitafel-50x1g"))
            } else if l.contains("vreneli") {
                Some(("gold", "900-vreneli-20-chf"))
            } else if l.contains("buffalo") {
                Some(("gold", "999-buffalo-1oz"))
            } else if l.contains("eagle") {
                Some(("gold", "916-eagle-1oz"))
            } else if l.contains("krügerrand") || l.contains("krugerrand") {
                if l.contains("1/10") {
                    Some(("gold", "916-kruegerrand-1-10oz"))
                } else if l.contains("1/4") {
                    Some(("gold", "916-kruegerrand-1-4oz"))
                } else if l.contains("1/2") {
                    Some(("gold", "916-kruegerrand-1-2oz"))
                } else if l.contains("1 unze") {
                    Some(("gold", "916-kruegerrand-1oz"))
                } else {
                    None
                }
            } else if l.contains("philharmoniker") {
                if l.contains("1/10") {
                    Some(("gold", "999-philharmoniker-1-10oz"))
                } else if l.contains("1/4") {
                    Some(("gold", "999-philharmoniker-1-4oz"))
                } else if l.contains("1/2") {
                    Some(("gold", "999-philharmoniker-1-2oz"))
                } else if l.contains("1 unze") {
                    Some(("gold", "999-philharmoniker-1oz"))
                } else {
                    None
                }
            } else if l.contains("maple leaf") {
                if l.contains("1/10") {
                    Some(("gold", "999-maple-leaf-1-10oz"))
                } else if l.contains("1/4") {
                    Some(("gold", "999-maple-leaf-1-4oz"))
                } else if l.contains("1/2") {
                    Some(("gold", "999-maple-leaf-1-2oz"))
                } else if l.contains("1 unze") {
                    Some(("gold", "999-maple-leaf-1oz"))
                } else {
                    None
                }
            } else if l.contains("känguru") || l.contains("kanguru") || l.contains("nugget") {
                if l.contains("1/10") {
                    Some(("gold", "999-kaenguru-1-10oz"))
                } else if l.contains("1/4") {
                    Some(("gold", "999-kaenguru-1-4oz"))
                } else if l.contains("1/2") {
                    Some(("gold", "999-kaenguru-1-2oz"))
                } else if l.contains("1 unze") {
                    Some(("gold", "999-kaenguru-1oz"))
                } else {
                    None
                }
            } else if l.contains("goldbarren") {
                // "25 Gramm" vor "5 Gramm", "250 Gramm" vor "50 Gramm":
                // die kurzen Nadeln stecken in den langen.
                if l.contains("1 gramm") {
                    Some(("gold", "999-barren-1g"))
                } else if l.contains("25 gramm") {
                    Some(("gold", "999-barren-hafner-25g"))
                } else if l.contains("5 gramm") {
                    Some(("gold", "999-barren-5g"))
                } else if l.contains("10 gramm") {
                    Some(("gold", "999-barren-10g"))
                } else if l.contains("1/2 unze") {
                    Some(("gold", "999-barren-hafner-1-2oz"))
                } else if l.contains("20 gramm") {
                    Some(("gold", "999-barren-20g"))
                } else if l.contains("1 unze") && l.contains("hafner") {
                    Some(("gold", "999-barren-hafner-1oz"))
                } else if l.contains("1 unze") {
                    Some(("gold", "999-barren-1oz"))
                } else if l.contains("250 gramm") {
                    Some(("gold", "999-barren-250g"))
                } else if l.contains("50 gramm") {
                    Some(("gold", "999-barren-50g"))
                } else if l.contains("100 gramm") {
                    Some(("gold", "999-barren-100g"))
                } else if l.contains("500 gramm") {
                    Some(("gold", "999-barren-500g"))
                } else {
                    None
                }
            } else {
                None
            }
        }
        "silver" => {
            if l.contains("unitybox") {
                Some(("silber", "999-silberbarren-unitybox-100x1g"))
            } else if l.contains("heraeus") {
                Some(("silber", "999-silberbarren-heraeus-250g"))
            } else if l.contains("silberbarren") {
                if l.contains("100 gramm") {
                    Some(("silber", "999-silberbarren-100g"))
                } else if l.contains("500") {
                    Some(("silber", "999-silberbarren-500g"))
                } else if l.contains("1kg") || l.contains("1.000") || l.contains("1000") {
                    Some(("silber", "999-silberbarren-1kg"))
                } else {
                    None
                }
            } else if l.contains("maple leaf") {
                if l.contains("diff") {
                    Some(("silber", "999-maple-leaf-1oz-diff"))
                } else if l.contains("1 unze") {
                    Some(("silber", "999-maple-leaf-1oz-19pct"))
                } else {
                    None
                }
            } else if l.contains("krügerrand") || l.contains("krugerrand") {
                if l.contains("diff") {
                    Some(("silber", "999-kruegerrand-1oz-diff"))
                } else if l.contains("1 unze") {
                    Some(("silber", "999-kruegerrand-1oz-19pct"))
                } else {
                    None
                }
            } else if l.contains("känguru") || l.contains("kanguru") || l.contains("nugget") {
                if l.contains("diff") {
                    Some(("silber", "999-kaenguru-1oz-diff"))
                } else if l.contains("1 unze") {
                    Some(("silber", "999-kaenguru-1oz-19pct"))
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Parst das Fenster `table.hwt-mp-table` (Start-Anker; Ende am
/// `</table>` — Footer-€ dahinter paart sich sonst mit Labels).
/// Liefert (Produktlabel, Metall, €/g aus Brutto/Gewicht) plus Skips.
/// Verkauf-Zeilen sind dokumentierte Filter-Skips, kein Mapping-Fehler.
fn parse(html: &str) -> Result<(Vec<(String, String, f64)>, Vec<String>), IngestError> {
    let mark = html
        .find("hwt-mp-table")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preislisten-Tabelle fehlt".to_owned(),
        })?;
    // Am öffnenden <table> starten, nicht mitten im Tag (sonst droppt
    // der Parser die Zeilen als textuelle Waisen).
    let start = html[..mark].rfind("<table").unwrap_or(mark);
    let tail = &html[start..];
    let end = tail.find("</table>").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preislisten-Tabelle unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let row_sel = Selector::parse("tr.hwt-mp-row").expect("valid selector");
    let cell_sel = Selector::parse("td").expect("valid selector");
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for row in frag.select(&row_sel) {
        let direction = row.value().attr("data-direction").unwrap_or("").trim();
        let metal = row
            .value()
            .attr("data-metal")
            .unwrap_or("")
            .trim()
            .to_lowercase();
        let cells: Vec<String> = row
            .select(&cell_sel)
            .map(|c| clean(&c.text().collect::<String>()))
            .collect();
        if cells.len() < 5 || metal.is_empty() {
            skipped.push(format!(
                "Tabellenzeile unvollständig: {}",
                cells.join(" / ")
            ));
            continue;
        }
        let (product, weight_cell, gross_cell) =
            (cells[0].clone(), cells[1].clone(), cells[4].clone());
        if product.is_empty() || product.len() > 120 {
            continue;
        }
        if direction == "verkauf" {
            skipped.push(format!("{product} (Verkaufspreis, kein Ankauf)"));
            continue;
        }
        if direction != "ankauf" {
            skipped.push(format!("{product} (Richtung unverständlich: {direction})"));
            continue;
        }
        let Some(weight) = parse_eur(&weight_cell) else {
            skipped.push(format!(
                "{product} (Gewicht unverständlich: {weight_cell})"
            ));
            continue;
        };
        // Gewicht steht seitenweit in Gramm; kg käme nur als Text und
        // wird hier in Gramm normiert (beweisbar, pro Zeile gedruckt).
        let lower_w = weight_cell.to_lowercase();
        let weight_g = if lower_w.contains("kg") {
            weight * 1000.0
        } else if lower_w.contains('g') {
            weight
        } else {
            skipped.push(format!(
                "{product} (Einheit unverständlich: {weight_cell})"
            ));
            continue;
        };
        if weight_g <= 0.0 {
            skipped.push(format!("{product} (Gewicht 0)"));
            continue;
        }
        let Some(gross) = parse_eur(&gross_cell) else {
            skipped.push(format!(
                "{product} (Preis unverständlich: {gross_cell})"
            ));
            continue;
        };
        // "0,00 €" heißt kein Tagespreis, kein Gratis-Geschenk.
        if gross == 0.0 {
            skipped.push(format!("{product} (Preis 0,00, kein Tagespreis)"));
            continue;
        }
        rows.push((product, metal, gross / weight_g));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preislisten-Tabelle leer".to_owned(),
        });
    }
    Ok((rows, skipped))
}

/// Bespoke Kontakt-Extraktion NUR für die Filialseite Aschaffenburg:
/// Anker-Heading "Anfahrt und Öffnungszeiten" ist Pflicht, danach die
/// `<p>`-Zeilen "Straße | PLZ Ort" und "E-Mail | Tel.: Nummer".
/// Fehlende Anker → Parse-Error, nie raten/fallback.
fn extract_info(branch: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(branch);
    let h3 = Selector::parse("h3").expect("valid selector");
    if !doc.select(&h3).any(|h| {
        h.text()
            .collect::<String>()
            .contains("Anfahrt und Öffnungszeiten")
    }) {
        return Err(IngestError::Parse {
            url: BRANCH_URL.to_owned(),
            detail: "Anfahrt-Block fehlt".to_owned(),
        });
    }
    let p = Selector::parse("p").expect("valid selector");
    let mut address: Option<(String, String, String)> = None;
    let mut phone = String::new();
    let mut email = String::new();
    for el in doc.select(&p) {
        let t = clean(&el.text().collect::<String>());
        if address.is_none() && t.contains('|') {
            let parts: Vec<&str> = t.split('|').map(str::trim).collect();
            if parts.len() == 2 {
                let mut it = parts[1].split_whitespace();
                if let (Some(pc), Some(_)) = (it.next(), it.next()) {
                    if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                        address = Some((
                            parts[0].to_owned(),
                            pc.to_owned(),
                            parts[1][pc.len()..].trim().to_owned(),
                        ));
                        continue;
                    }
                }
            }
        }
        if t.contains('@') && t.contains("Tel") {
            for tok in t.split_whitespace() {
                if email.is_empty() && tok.contains('@') {
                    email = tok.trim_matches([',', ';', '|']).to_owned();
                }
            }
            if let Some((_, after)) = t.split_once("Tel.:") {
                phone = after.trim().trim_matches('|').trim().to_owned();
            } else if let Some((_, after)) = t.split_once("Tel:") {
                phone = after.trim().trim_matches('|').trim().to_owned();
            }
        }
    }
    let Some((street, postcode, city)) = address else {
        return Err(IngestError::Parse {
            url: BRANCH_URL.to_owned(),
            detail: "Filialadress-Zeile fehlt".to_owned(),
        });
    };
    if phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: BRANCH_URL.to_owned(),
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

/// Whitespace kollabieren (scraper-`text()` + `&nbsp;` als \u{a0}).
fn clean(raw: &str) -> String {
    raw.replace(['\u{a0}'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    // Reale Form der Live-Tabelle (28.09.2026): tr-Attribute,
    // Zellklassen, deutsche Zahlen, MwSt.-Varianten; je Richtung,
    // Metall und Produkttyp mindestens eine Zeile plus 0,00-Fall.
    const FIXTURE: &str = "<table class=\"hwt-mp-table\"><thead><tr>\
        <th class=\"hwt-mp-col-product\">Produkt</th>\
        <th class=\"hwt-mp-col-weight\">Gewicht</th>\
        <th class=\"hwt-mp-col-price-net\">Preis (netto)</th>\
        <th class=\"hwt-mp-col-vat\">MwSt.</th>\
        <th class=\"hwt-mp-col-price-gross\">Preis (brutto)</th></tr></thead><tbody>\
        <tr class=\"hwt-mp-row\" data-metal=\"gold\" data-type=\"barren\" data-direction=\"verkauf\" data-product=\"1 Gramm Goldbarren (diverse Hersteller)\">\
        <td class=\"hwt-mp-col-product\">1 Gramm Goldbarren (diverse Hersteller)</td>\
        <td class=\"hwt-mp-col-weight\">1,0000 g</td>\
        <td class=\"hwt-mp-col-price-net\">142,63 €</td>\
        <td class=\"hwt-mp-col-vat\">0 % (0,00 €)</td>\
        <td class=\"hwt-mp-col-price-gross\">142,63 €</td></tr>\
        <tr class=\"hwt-mp-row\" data-metal=\"gold\" data-type=\"barren\" data-direction=\"ankauf\" data-product=\"1 Gramm Goldbarren (diverse Hersteller)\">\
        <td class=\"hwt-mp-col-product\">1 Gramm Goldbarren (diverse Hersteller)</td>\
        <td class=\"hwt-mp-col-weight\">1,0000 g</td>\
        <td class=\"hwt-mp-col-price-net\">113,39 €</td>\
        <td class=\"hwt-mp-col-vat\">0 % (0,00 €)</td>\
        <td class=\"hwt-mp-col-price-gross\">113,39 €</td></tr>\
        <tr class=\"hwt-mp-row\" data-metal=\"gold\" data-type=\"munzen\" data-direction=\"ankauf\" data-product=\"1 Unze Goldmünze Krügerrand (diverse Jahrgänge)\">\
        <td class=\"hwt-mp-col-product\">1 Unze Goldmünze Krügerrand (diverse Jahrgänge)</td>\
        <td class=\"hwt-mp-col-weight\">31,1035 g</td>\
        <td class=\"hwt-mp-col-price-net\">3.562,89 €</td>\
        <td class=\"hwt-mp-col-vat\">0 % (0,00 €)</td>\
        <td class=\"hwt-mp-col-price-gross\">3.562,89 €</td></tr>\
        <tr class=\"hwt-mp-row\" data-metal=\"gold\" data-type=\"munzen\" data-direction=\"ankauf\" data-product=\"20 Schweizer Franken Vreneli\">\
        <td class=\"hwt-mp-col-product\">20 Schweizer Franken Vreneli</td>\
        <td class=\"hwt-mp-col-weight\">5,8100 g</td>\
        <td class=\"hwt-mp-col-price-net\">665,54 €</td>\
        <td class=\"hwt-mp-col-vat\">0 % (0,00 €)</td>\
        <td class=\"hwt-mp-col-price-gross\">665,54 €</td></tr>\
        <tr class=\"hwt-mp-row\" data-metal=\"silver\" data-type=\"barren\" data-direction=\"ankauf\" data-product=\"1kg Silberbarren (19 % MwSt)\">\
        <td class=\"hwt-mp-col-product\">1kg Silberbarren (19 % MwSt)</td>\
        <td class=\"hwt-mp-col-weight\">1.000,0000 g</td>\
        <td class=\"hwt-mp-col-price-net\">1.595,77 €</td>\
        <td class=\"hwt-mp-col-vat\">19 % (303,20 €)</td>\
        <td class=\"hwt-mp-col-price-gross\">1.898,97 €</td></tr>\
        <tr class=\"hwt-mp-row\" data-metal=\"silver\" data-type=\"munzen\" data-direction=\"ankauf\" data-product=\"1 Unze Silbermünze Känguru Nugget (Diff.-besteuert, diverse Jahrgänge)\">\
        <td class=\"hwt-mp-col-product\">1 Unze Silbermünze Känguru Nugget (Diff.-besteuert, diverse Jahrgänge)</td>\
        <td class=\"hwt-mp-col-weight\">31,1000 g</td>\
        <td class=\"hwt-mp-col-price-net\">55,42 €</td>\
        <td class=\"hwt-mp-col-vat\">0 % (0,00 €)</td>\
        <td class=\"hwt-mp-col-price-gross\">55,42 €</td></tr>\
        <tr class=\"hwt-mp-row\" data-metal=\"gold\" data-type=\"barren\" data-direction=\"ankauf\" data-product=\"10 Gramm Goldbarren (diverse Hersteller)\">\
        <td class=\"hwt-mp-col-product\">10 Gramm Goldbarren (diverse Hersteller)</td>\
        <td class=\"hwt-mp-col-weight\">10,0000 g</td>\
        <td class=\"hwt-mp-col-price-net\">0,00 €</td>\
        <td class=\"hwt-mp-col-vat\">0 % (0,00 €)</td>\
        <td class=\"hwt-mp-col-price-gross\">0,00 €</td></tr>\
        </tbody></table>";

    #[test]
    fn ankauf_rows_become_per_gram_prices() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // 5 Ankauf-Zeilen mit Preis; Verkauf + 0,00 skippen laut.
        assert_eq!(rows.len(), 5);
        assert_eq!(skips.len(), 2);
        assert!(skips
            .iter()
            .any(|s| s.contains("1 Gramm Goldbarren") && s.contains("Verkaufspreis")));
        assert!(skips
            .iter()
            .any(|s| s.contains("10 Gramm Goldbarren") && s.contains("0,00")));
        // Brutto durch gedrucktes Gewicht: Anzeige gewinnt, kein Raten.
        assert_eq!(rows[0].0, "1 Gramm Goldbarren (diverse Hersteller)");
        assert!((rows[0].2 - 113.39).abs() < 1e-9);
        // Tausenderpunkt im Gewicht ("1.000,0000 g") und im Preis.
        let kg = rows.iter().find(|(l, _, _)| l.contains("1kg")).expect("kg row");
        assert!((kg.2 - 1.89897).abs() < 1e-9, "brutto/g: {kg:?}");
        let kru = rows
            .iter()
            .find(|(l, _, _)| l.contains("Krügerrand"))
            .expect("krugerrand");
        assert!((kru.2 - 3562.89 / 31.1035).abs() < 1e-9);
        assert!(rows.iter().all(|(_, m, _)| m == "gold" || m == "silver"));
        // Redesign ohne Tabelle / leere Tabelle: lauter Fehler.
        assert!(parse("<div>Redesign ohne Tabelle</div>").is_err());
        assert!(parse("<table class=\"hwt-mp-table\"></table>").is_err());
    }

    #[test]
    fn every_live_product_maps_to_its_own_variant() {
        // Alle 44 Ankauf-Produkte der Live-Seite (28.09.2026): kein Label
        // bleibt ungemappt, keine zwei teilen (Material, Variante).
        let live: &[(&str, &str, &str)] = &[
            ("gold", "1 Gramm Goldbarren (diverse Hersteller)", "999-barren-1g"),
            ("gold", "5 Gramm Goldbarren (diverse Hersteller)", "999-barren-5g"),
            ("gold", "10 Gramm Goldbarren (diverse Hersteller)", "999-barren-10g"),
            ("gold", "1/2 Unze Goldbarren C. Hafner geprägt", "999-barren-hafner-1-2oz"),
            ("gold", "20 Gramm Goldbarren (diverse Hersteller)", "999-barren-20g"),
            ("gold", "25 Gramm Goldbarren Feingold C. Hafner", "999-barren-hafner-25g"),
            ("gold", "1 Unze Goldbarren C. Hafner geprägt", "999-barren-hafner-1oz"),
            ("gold", "1 Unze Goldbarren (diverse Hersteller)", "999-barren-1oz"),
            ("gold", "50 Gramm Goldbarren (diverse Hersteller)", "999-barren-50g"),
            ("gold", "50 x 1 Gramm Goldbarren Combibarren / Tafelbarren / Goldtafel", "999-combitafel-50x1g"),
            ("gold", "100 x 1g Goldbarren Valcambi CombiBar®", "999-combibar-valcambi-100x1g"),
            ("gold", "100 Gramm Goldbarren (diverse Hersteller)", "999-barren-100g"),
            ("gold", "250 Gramm Goldbarren (diverse Hersteller)", "999-barren-250g"),
            ("gold", "500 Gramm Goldbarren (diverse Hersteller)", "999-barren-500g"),
            ("gold", "1/10 Unze Goldmünze Wiener Philharmoniker (diverse Jahrgänge)", "999-philharmoniker-1-10oz"),
            ("gold", "1/10 Unze Goldmünze Krügerrand (diverse Jahrgänge)", "916-kruegerrand-1-10oz"),
            ("gold", "1/10 Unze Goldmünze Känguru Nugget (diverse Jahrgänge)", "999-kaenguru-1-10oz"),
            ("gold", "1/10 Unze Goldmünze Maple Leaf (diverse Jahrgänge)", "999-maple-leaf-1-10oz"),
            ("gold", "20 Schweizer Franken Vreneli", "900-vreneli-20-chf"),
            ("gold", "1/4 Unze Goldmünze Wiener Philharmoniker (diverse Jahrgänge)", "999-philharmoniker-1-4oz"),
            ("gold", "1/4 Unze Goldmünze Maple Leaf (diverse Jahrgänge)", "999-maple-leaf-1-4oz"),
            ("gold", "1/4 Unze Goldmünze Krügerrand (diverse Jahrgänge)", "916-kruegerrand-1-4oz"),
            ("gold", "1/4 Unze Goldmünze Känguru Nugget (diverse Jahrgänge)", "999-kaenguru-1-4oz"),
            ("gold", "1/2 Unze Goldmünze Wiener Philharmoniker (diverse Jahrgänge)", "999-philharmoniker-1-2oz"),
            ("gold", "1/2 Unze Goldmünze Maple Leaf (diverse Jahrgänge)", "999-maple-leaf-1-2oz"),
            ("gold", "1/2 Unze Goldmünze Krügerrand (diverse Jahrgänge)", "916-kruegerrand-1-2oz"),
            ("gold", "1/2 Unze Goldmünze Känguru Nugget (diverse Jahrgänge)", "999-kaenguru-1-2oz"),
            ("gold", "1 Unze Goldmünze American Buffalo (diverse Jahrgänge)", "999-buffalo-1oz"),
            ("gold", "1 Unze Goldmünze Wiener Philharmoniker (diverse Jahrgänge)", "999-philharmoniker-1oz"),
            ("gold", "1 Unze Goldmünze Maple Leaf (diverse Jahrgänge)", "999-maple-leaf-1oz"),
            ("gold", "1 Unze Goldmünze American Eagle (diverse Jahrgänge)", "916-eagle-1oz"),
            ("gold", "1 Unze Goldmünze Känguru Nugget (diverse Jahrgänge)", "999-kaenguru-1oz"),
            ("gold", "1 Unze Goldmünze Krügerrand (diverse Jahrgänge)", "916-kruegerrand-1oz"),
            ("silver", "100 x 1g Silberbarren UnityBox (Heimerle und Meule) (19 % MwSt)", "999-silberbarren-unitybox-100x1g"),
            ("silver", "100 Gramm Silberbarren (diverse Hersteller)", "999-silberbarren-100g"),
            ("silver", "250 Gramm Silberbarren Heraeus gegossen", "999-silberbarren-heraeus-250g"),
            ("silver", "500g Silberbarren ( diverse Hersteller)", "999-silberbarren-500g"),
            ("silver", "1kg Silberbarren (19 % MwSt)", "999-silberbarren-1kg"),
            ("silver", "1 Unze Silbermünze Maple Leaf (19 % MwSt)", "999-maple-leaf-1oz-19pct"),
            ("silver", "1 Unze Silbermünze Krügerrand (19 % MwSt)", "999-kruegerrand-1oz-19pct"),
            ("silver", "1 Unze Silbermünze Känguru Nugget (19 % MwSt)", "999-kaenguru-1oz-19pct"),
            ("silver", "1 Unze Silbermünze Känguru Nugget (Diff.-besteuert, diverse Jahrgänge)", "999-kaenguru-1oz-diff"),
            ("silver", "1 Unze Silbermünze Krügerrand (Diff.-besteuert, diverse Jahrgänge)", "999-kruegerrand-1oz-diff"),
            ("silver", "1 Unze Silbermünze Maple Leaf (Diff.-besteuert, diverse Jahrgänge)", "999-maple-leaf-1oz-diff"),
        ];
        assert_eq!(live.len(), 44);
        let mut seen = std::collections::HashSet::new();
        for (metal, product, variant) in live {
            let got = grade_for(metal, product);
            let material = if *metal == "gold" { "gold" } else { "silber" };
            assert_eq!(got, Some((material, *variant)), "{product}");
            assert!(seen.insert((material, *variant)), "Kollaps: {variant}");
        }
        // Fremdmetall, unbekanntes Produkt, Größen-Fallthrough: laut None.
        assert_eq!(grade_for("platin", "1 Gramm Platinbarren"), None);
        assert_eq!(grade_for("gold", "Goldschmuck pauschal"), None);
        assert_eq!(grade_for("gold", "2 Unze Goldmünze Krügerrand"), None);
        assert_eq!(grade_for("", "1 Gramm Goldbarren"), None);
    }

    #[test]
    fn branch_contact_block() {
        // Realer Ausschnitt der Filialseite Aschaffenburg (28.09.2026).
        let branch = "<h3>Anfahrt und Öffnungszeiten</h3>\
            <p>Weißenburger Str. 18 | 63739 Aschaffenburg</p>\
            <p>info@edelmetallshop-aschaffenburg.de | Tel.: 06021 4542399</p>";
        let info = extract_info(branch).expect("parses");
        assert_eq!(info.street, "Weißenburger Str. 18");
        assert_eq!(info.postcode, "63739");
        assert_eq!(info.city, "Aschaffenburg");
        assert_eq!(info.phone, "06021 4542399");
        assert_eq!(info.email, "info@edelmetallshop-aschaffenburg.de");
        // Fehlende Anker scheitern laut statt zu raten.
        assert!(extract_info("<h3>Anfahrt</h3><p>Weißenburger Str. 18 | 63739 Aschaffenburg</p>").is_err());
        assert!(extract_info("<p>Neu hier</p>").is_err());
    }
}
