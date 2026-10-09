//! Beller Berlin: server-rendered PREISLISTE ANKAUF in #modal-altmetall.
//! No page-stated price date. Open-ended `ab` quotes are acceptance-only:
//! the catalog has no lower-bound price kind. Zuschlag is not a material.
use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;
use scraper::{Html, Selector};

pub const SLUG: &str = "be-neukolln-beller-demontagen-altmetall-schrott";
pub const URL: &str = "https://www.beller-das.de/";

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
    build_outcome(status, &html)
}
fn error(detail: &str) -> IngestError {
    IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.to_owned(),
    }
}
fn text(el: scraper::ElementRef<'_>) -> String {
    el.text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

// Exact live labels: unknown additions skip loudly rather than guessing.
// Every sub-grade/quantity condition is preserved as its own variant.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    GRADES
        .iter()
        .find(|(l, _)| *l == label)
        .map(|(l, m)| (*m, *l))
}
const GRADES: &[(&str, &str)] = &[
    ("Kupfer Draht blank \"Millberry\"", "kupfer-millberry"),
    ("Elektrolytkupfer, \"E-Cu\"", "kupfer-gemischt"),
    ("Kupfer Rohre, blank \"Candy\"", "kupfer-candy"),
    (
        "Kupfer Rohre mit Anhaftungen (Lötzinn/Farbe)",
        "kupfer-gemischt",
    ),
    (
        "Kupfer Apparate mind. 85% Cu, keine Durchlauferhitzer o.ä.",
        "kupfer-gemischt",
    ),
    ("Kupfer Kabel zum Schälen (dick)", "kabel-kupfer"),
    ("Kupfer Kabel dünn bis 25qmm (ohne Stecker)", "kabel-kupfer"),
    ("Kupfer Kabel w.o., flexibel (ohne Stecker)", "kabel-kupfer"),
    (
        "Kupfer Kabel, PC + Antenn.k. (ohne Stecker)",
        "kabel-kupfer",
    ),
    ("ALU Blech, neu", "aluminium-blech"),
    ("ALU Profile blank, neu (Al Mg Si 05)", "aluminium-profile"),
    (
        "ALU Blechabfälle alt/ neu gemischt, ohne Anhaft., S.1",
        "aluminium-blech",
    ),
    ("ALU Blech anhaftend max. 2 % Fe, S.2", "aluminium-blech"),
    ("ALU Profile Farbe / PVC", "aluminium-profile"),
    ("ALU Guß sauber, neu ohne Fe + Anhaft.", "aluminium-guss"),
    ("ALU Guß anhaftend max. 2 % Fe", "aluminium-guss"),
    ("Zinkabfälle neu", "zink"),
    ("Zinkabfälle alt", "zink"),
    ("Altblei, sauber", "blei"),
    ("Kabelschälblei ohne Anhaftung", "blei"),
    ("Altblei, unsauber", "blei"),
    ("Messing 58 / 63 Stangen, Bleche, grün", "messing"),
    ("Sammelmessing o. Anhaftungen", "messing"),
    ("Sammelmessing m. Anhaftungen (max. 1%)", "messing"),
    (
        "Sammelmessing m. Anhaftungen (max. 20%): gilt NICHT für Wasseruhren",
        "messing",
    ),
    ("V2a neu, Blechabfälle", "edelstahl-v2a"),
    ("V2a alt", "edelstahl-v2a"),
    ("E-Motore (bis 300 kg)", "elektromotoren"),
    ("Fe-Guss (Gusseisen)", "eisenschrott-gussbruch"),
    ("Mischschrott (mind. 100 kg)", "mischschrott"),
    (
        "Bremsscheiben (nur PKW, ohne Anhaftungen)",
        "eisenschrott-gussbruch",
    ),
    ("Stahl kurz, Schienen unter 150 cm", "stahlschrott-scheren"),
];

fn build_outcome(status: u16, html: &str) -> Result<HandlerOutcome, IngestError> {
    let doc = Html::parse_document(html);
    let table_sel = Selector::parse("#modal-altmetall table").unwrap();
    let table = doc
        .select(&table_sel)
        .next()
        .ok_or_else(|| error("Ankaufstabelle fehlt"))?;
    let heading_sel = Selector::parse("#modal-altmetall h3").unwrap();
    if !doc
        .select(&heading_sel)
        .any(|e| text(e) == "PREISLISTE ANKAUF")
    {
        return Err(error("Ankauf-Anker fehlt"));
    }
    let header_sel = Selector::parse("th").unwrap();
    let headers: Vec<_> = table.select(&header_sel).map(text).collect();
    if headers != ["Bezeichnung", "Preis €/kg"] {
        return Err(error("Preis-Spalten oder EUR/kg-Einheit geändert"));
    }
    let mut out = HandlerOutcome {
        trader_info: extract_info(&doc)?,
        website_alive: true,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        ..Default::default()
    };
    let rows = Selector::parse("tr").unwrap();
    let cells = Selector::parse("td").unwrap();
    for row in table.select(&rows) {
        let c: Vec<_> = row.select(&cells).map(text).collect();
        if c.is_empty() {
            continue;
        }
        if c.len() != 2 {
            return Err(error("Preiszeile hat nicht zwei Spalten"));
        }
        let Some((material, variant)) = grade_for(&c[0]) else {
            out.skipped_labels
                .push(format!("{} (kein Materialpreis/kein Katalogmapping)", c[0]));
            continue;
        };
        if c[1].starts_with("ab ") {
            // A malformed minimum quote must not masquerade as a valid condition.
            parse_price(c[1].trim_start_matches("ab "))?;
            out.acceptances.push(ScrapedAcceptance {
                material,
                conditions: format!("{}; {} /kg", c[0], c[1]),
                label: c[0].clone(),
            });
            out.skipped_labels
                .push(format!("{} (offener Mindestpreis: {} /kg)", c[0], c[1]));
            continue;
        }
        let (price, min, max, kind) = parse_price(&c[1])?;
        out.prices.push(ScrapedPrice {
            material,
            variant,
            price,
            currency: "EUR",
            unit: "EUR/kg",
            price_kind: kind,
            price_min: min,
            price_max: max,
            confidence: Some(if kind == "exact" { 1.0 } else { 0.8 }),
            label: c[0].clone(),
        });
    }
    if out.prices.is_empty() {
        return Err(error("Keine gemappten Ankaufspreise"));
    }
    Ok(out)
}

fn parse_price(raw: &str) -> Result<(f64, Option<f64>, Option<f64>, &'static str), IngestError> {
    let number = |s: &str| -> Result<f64, IngestError> {
        let s = s.trim().trim_end_matches('€').trim();
        if s.is_empty()
            || !s
                .chars()
                .all(|c| c.is_ascii_digit() || c == ',' || c == '.')
        {
            return Err(error("Unverständlicher Preis"));
        }
        parse_eur(s)
            .filter(|n| n.is_finite() && *n > 0.0)
            .ok_or_else(|| error("Ungültiger Preis"))
    };
    if let Some((a, b)) = raw.split_once(['-', '–']) {
        let (a, b) = (number(a)?, number(b)?);
        if a > b {
            return Err(error("Umgekehrte Preisspanne"));
        }
        Ok(((a + b) / 2.0, Some(a), Some(b), "range"))
    } else {
        Ok((number(raw)?, None, None, "exact"))
    }
}

fn extract_info(doc: &Html) -> Result<TraderInfo, IngestError> {
    let sel = Selector::parse("#modal-impressum p").unwrap();
    let paragraphs: Vec<_> = doc.select(&sel).collect();
    let firm = paragraphs.first().ok_or_else(|| error("Impressum fehlt"))?;
    let lines: Vec<_> = firm
        .text()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if lines.len() != 4
        || lines[0] != "BELLER Demontagen Altmetall Schrott GmbH"
        || !lines[1].starts_with("Geschäftsführer:")
    {
        return Err(error("Betreiberblock geändert"));
    }
    let (postcode, city) = lines[3]
        .split_once(' ')
        .ok_or_else(|| error("Adressblock geändert"))?;
    if postcode != "12359" || city != "Berlin" || lines[2] != "Späthstraße 145" {
        return Err(error("Beller Standort geändert; Zuordnung prüfen"));
    }
    let contact = paragraphs
        .get(1)
        .ok_or_else(|| error("Kontaktblock fehlt"))?;
    let phone = contact
        .text()
        .find_map(|s| s.trim().strip_prefix("fon:").map(|p| p.trim().to_owned()))
        .ok_or_else(|| error("Telefon fehlt"))?;
    let mail_sel = Selector::parse("a[href^='mailto:']").unwrap();
    let email = contact
        .select(&mail_sel)
        .next()
        .and_then(|e| e.value().attr("href"))
        .and_then(|s| s.strip_prefix("mailto:"))
        .ok_or_else(|| error("E-Mail fehlt"))?;
    Ok(TraderInfo {
        street: lines[2].to_owned(),
        postcode: postcode.to_owned(),
        city: city.to_owned(),
        phone,
        email: email.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> String {
        let mut s = String::from("<div id='modal-altmetall'><h3>PREISLISTE ANKAUF</h3><table><tr><th>Bezeichnung</th><th>Preis €/kg</th></tr>");
        for (label, _) in GRADES {
            let price = if label.contains("Apparate") {
                "2,40 € - 4,40 €"
            } else if label.contains("Schälen") {
                "ab 3,40 €"
            } else {
                "1,50 €"
            };
            s.push_str(&format!("<tr><td>{label}</td><td>{price}</td></tr>"));
        }
        s.push_str("<tr><td>Zuschlag (Material zum Zerlegen)</td><td>0,25 €</td></tr></table></div><div id='modal-impressum'><p>BELLER Demontagen Altmetall Schrott GmbH<br>Geschäftsführer: Jörg Beller<br>Späthstraße 145<br>12359 Berlin</p><p>fon: 030 &#8211; 33 44 889<br>fax: 030 - 35 40 35 10<br>eMail: <a href='mailto:info@beller-das.de'>info@beller-das.de</a></p></div>");
        s
    }
    #[test]
    fn live_grades_range_and_quantity_conditions() {
        let out = build_outcome(200, &fixture()).unwrap();
        assert_eq!(out.prices.len(), 31);
        assert_eq!(out.acceptances.len(), 1);
        assert_eq!(out.skipped_labels.len(), 2);
        assert_eq!(out.published_at, None);
        assert_eq!(out.trader_info.email, "info@beller-das.de");
        assert_eq!(out.trader_info.phone, "030 – 33 44 889");
        let range = out.prices.iter().find(|p| p.price_kind == "range").unwrap();
        assert!((range.price - 3.4).abs() < 1e-9);
        assert_eq!((range.price_min, range.price_max), (Some(2.4), Some(4.4)));
        assert!(out
            .prices
            .iter()
            .any(|p| p.variant == "Mischschrott (mind. 100 kg)"));
        assert!(out.prices.iter().all(|p| p.unit == "EUR/kg"));
        let keys: std::collections::HashSet<_> =
            out.prices.iter().map(|p| (p.material, p.variant)).collect();
        assert_eq!(keys.len(), 31);
    }
    #[test]
    fn fail_closed_and_report_unknowns() {
        for (from, to) in [
            ("Preis €/kg", "Preis €/t"),
            ("PREISLISTE ANKAUF", "VERKAUF"),
            ("Späthstraße 145", "Andere Straße 1"),
            ("1,50 €", "bis 1,50 €"),
        ] {
            assert!(
                build_outcome(200, &fixture().replace(from, to)).is_err(),
                "{from}"
            );
        }
        assert!(build_outcome(200, "<table><td>Kupfer</td><td>10 €</td></table>").is_err());
        assert!(parse_price("4,40 € - 2,40 €").is_err());
        assert!(parse_price("0,00 €").is_err());
        let html = format!(
            "<table><tr><td>Gold</td><td>999 €</td></tr></table>{}",
            fixture().replace("Zuschlag (Material zum Zerlegen)", "Unbekannte Sorte")
        );
        let out = build_outcome(200, &html).unwrap();
        assert_eq!(out.prices.len(), 31);
        assert!(out
            .skipped_labels
            .iter()
            .any(|s| s.contains("Unbekannte Sorte")));
        assert_eq!(grade_for("Gold"), None);
    }

    #[test]
    fn independently_quoted_live_rows_and_dynamic_changes() {
        // Live excerpt 09.10.2026, independent of the mapping table.
        let rows = "<tr><td>Kupfer Draht blank &quot;Millberry&quot;</td><td>10,10 €</td></tr>\
            <tr><td>Kupfer Rohre, blank  &quot;Candy&quot;</td><td>9,70 €</td></tr>\
            <tr><td>Kupfer Kabel zum Schälen (dick)</td><td>ab 3,40 €</td></tr>\
            <tr><td>Mischschrott (mind. 100 kg)</td><td>0,10 €</td></tr>";
        let base = fixture();
        let start = base.find("</tr>").unwrap() + 5;
        let end = base.find("</table>").unwrap();
        let html = format!("{}{rows}{}", &base[..start], &base[end..]);
        let out = build_outcome(200, &html).unwrap();
        assert_eq!(out.prices.len(), 3);
        assert_eq!(out.prices[0].material, "kupfer-millberry");
        assert_eq!(out.prices[0].price, 10.1);
        assert_eq!(out.prices[1].material, "kupfer-candy");
        assert_eq!(out.prices[2].price, 0.1);
        assert!(out.acceptances[0].conditions.contains("ab 3,40 € /kg"));
        let changed = build_outcome(200, &html.replace("10,10 €", "10,25 €")).unwrap();
        assert_eq!(changed.prices[0].price, 10.25);
        assert!(build_outcome(200, &html.replace("ab 3,40 €", "ab Anfrage")).is_err());
    }
}
