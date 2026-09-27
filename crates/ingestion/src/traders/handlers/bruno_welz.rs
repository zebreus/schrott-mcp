//! Bruno Welz Scheideanstalt (Schwäbisch Gmünd): exact per-gram
//! purchase prices from the shop's own JSON endpoint
//! (`/ankaufpreise/import`: {"date": "27.09.2026", "333er Gold": 39.12,
//! … "999 - Silberbarren/…": 1.67}). The visible shop spans render the
//! same numbers client-side ("bis zu €/g"), so the shop page is fetched
//! first: it must still name the endpoint context ("Ankauf" + "€/g"),
//! otherwise the step fails loudly instead of trusting a stale URL.
//! Fineness rides in the variant ("999er Gold" → `gold`/`999`);
//! "Goldbarren" carries the barren premium as its own variant. Quoted
//! unit honestly EUR/g (shop spans + catalog units match).

use super::super::{
    fetch_text, parse_de_date, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;
use scraper::{Html, Selector};

pub const SLUG: &str = "bw-schwabisch-gmund-bruno-welz-scheideanstalt";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://shop.bruno-welz.de/impressum/";

pub const URL: &str = "https://shop.bruno-welz.de/edelmetall-ankauf/";
/// The shop's own price-import endpoint (same origin, no crawler).
/// Only called while the shop page still frames it as Ankaufpreise.
pub const PRICES_URL: &str = "https://shop.bruno-welz.de/ankaufpreise/import";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, shop) = fetch_text(client, URL).await?;
    // The shop page must still present this as per-gram Ankaufpreise —
    // otherwise the JSON below has no live unit grounding.
    for anchor in ["Ankauf", "€/g"] {
        if !shop.contains(anchor) {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: format!("Ankauf-Anker fehlt: {anchor}"),
            });
        }
    }
    let (_, json) = fetch_text(client, PRICES_URL).await?;
    let (published_at, rows, mut skipped_labels) = parse(&json)?;
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
            None => skipped_labels.push(format!("{label} ({price:.2} {unit}, kein Katalogmaterial")),
        }
    }
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
        byte_len: shop.len() + json.len(),
        published_at,
    })
}

/// Explicit label → (material, variant) mapping. Fineness rides in the
/// variant; "Goldbarren" is the barren premium, never plain 999.
/// Silver barren/granulat rows keep their form in the variant so bank
/// trade and granulate never collapse.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    if l.contains("goldbarren") {
        return Some(("gold", "Barren"));
    }
    if l.contains("silberbarren") || l.contains("silbermünzen") {
        return Some(("silber", "999 Barren/Münzen"));
    }
    if l.contains("granulat") {
        return Some(("silber", "999 Granulat"));
    }
    if l.contains("gold") {
        return Some(("gold", fineness(&l)));
    }
    if l.contains("silber") {
        return Some(("silber", fineness(&l)));
    }
    None
}

/// First fineness run in the label ("999er Gold" → "999").
fn fineness(l: &str) -> &'static str {
    for fin in ["999", "986", "980", "965", "925", "916", "900", "875", "835", "800", "750", "625", "585", "375", "333"] {
        if l.contains(fin) {
            return fin;
        }
    }
    ""
}

/// Parse the import JSON: `date` becomes published_at, every other
/// key/value pair with a positive number becomes a row. Non-numeric or
/// non-positive entries skip loudly; 0 rows → Err.
fn parse(json: &str) -> Result<(Option<String>, Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let v: serde_json::Value = serde_json::from_str(json).map_err(|_| IngestError::Parse {
        url: PRICES_URL.to_owned(),
        detail: "Preisantwort kein JSON".to_owned(),
    })?;
    let obj = v.as_object().ok_or_else(|| IngestError::Parse {
        url: PRICES_URL.to_owned(),
        detail: "Preisantwort kein Objekt".to_owned(),
    })?;
    let published_at = obj
        .get("date")
        .and_then(|d| d.as_str())
        .and_then(|d| {
            let mut p = d.split('.');
            parse_de_date(p.next()?, p.next()?, p.next()?)
        });
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for (k, val) in obj {
        if k == "date" || k == "time" {
            continue;
        }
        match val.as_f64() {
            Some(p) if p > 0.0 => rows.push((k.clone(), p, "EUR/g")),
            _ => skips.push(format!("{k} (Preis unverständlich)")),
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: PRICES_URL.to_owned(), detail: "Preisliste leer".to_owned() });
    }
    Ok((published_at, rows, skips))
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` lines
/// ("Bruno Welz GmbH" / "Imhofstr. 2" / "73525 Schwäbisch Gmünd" /
/// "Tel.:" / "E-Mail:"). Missing firm anchor → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    if !doc
        .root_element()
        .text()
        .collect::<String>()
        .contains("Bruno Welz GmbH")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmen-Block fehlt".to_owned(),
        });
    }
    let p_sel = Selector::parse("p").expect("valid selector");
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let (mut phone, mut email) = (String::new(), String::new());
    for el in doc.select(&p_sel) {
        let lines: Vec<String> = el
            .inner_html()
            .split("<br")
            .map(|s| strip_fragment(s))
            .filter(|s| !s.is_empty())
            .collect();
        for (k, line) in lines.iter().enumerate() {
            let mut it = line.split_whitespace();
            if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                if postcode.is_empty() && pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    city = [ci.to_owned(), it.collect::<Vec<_>>().join(" ")].join(" ").trim().to_owned();
                    if k > 0 {
                        street = lines[k - 1].clone();
                    }
                    continue;
                }
            }
            if phone.is_empty() {
                if let Some(v) = line.strip_prefix("Tel.:").or_else(|| line.strip_prefix("Telefon:")) {
                    phone = v.trim().to_owned();
                }
            }
            if email.is_empty() {
                if let Some(v) = line.strip_prefix("E-Mail:") {
                    email = v.trim().to_owned();
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
    Ok(TraderInfo { street, postcode, city, phone, email })
}

/// Strip tags from a `<br`-split fragment (drop up to the first '>').
fn strip_fragment(s: &str) -> String {
    let s = match s.find('>') {
        Some(i) => &s[i + 1..],
        None => s,
    };
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

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    // Real endpoint shape (date + fineness keys), trimmed.
    const FIXTURE: &str = "{\"date\":\"27.09.2026\",\"time\":\"18:45:07\",\
        \"999er Gold\":117.7,\"750er Gold\":87.84,\"333er Gold\":39.12,\
        \"Goldbarren\":118.91,\"925er Silber\":1.32,\
        \"999 - Silberbarren/Silbermünzen bankhandelsfähig\":1.67,\
        \"999 - Feinsilber Granulat und sonstiges, nicht bankhandelsfähig\":1.4,\
        \"Platin\":null}";

    #[test]
    fn json_rows_date_and_gaps() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-27T00:00:00+00:00"));
        assert_eq!(rows.len(), 7);
        // serde_json maps sort keys — assert by label, never by order.
        let row = |l: &str| rows.iter().find(|r| r.0 == l).expect(l).clone();
        assert_eq!(row("999er Gold"), ("999er Gold".to_owned(), 117.7, "EUR/g"));
        assert_eq!(row("333er Gold"), ("333er Gold".to_owned(), 39.12, "EUR/g"));
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Platin"));
        assert!(parse("{\"date\":\"27.09.2026\"}").is_err(), "0 rows = Err");
        assert!(parse("kein json").is_err());
    }

    #[test]
    fn fineness_and_barren_variants() {
        assert_eq!(grade_for("999er Gold"), Some(("gold", "999")));
        assert_eq!(grade_for("333er Gold"), Some(("gold", "333")));
        assert_eq!(grade_for("Goldbarren"), Some(("gold", "Barren")));
        assert_eq!(grade_for("925er Silber"), Some(("silber", "925")));
        assert_eq!(
            grade_for("999 - Silberbarren/Silbermünzen bankhandelsfähig"),
            Some(("silber", "999 Barren/Münzen"))
        );
        assert_eq!(
            grade_for("999 - Feinsilber Granulat und sonstiges, nicht bankhandelsfähig"),
            Some(("silber", "999 Granulat"))
        );
        assert_eq!(grade_for("Platin"), None);
    }

    #[test]
    fn impressum_paragraph_lines() {
        let imp = "<h1>Impressum</h1><p>Bruno Welz GmbH<br />Imhofstr. 2<br />\
            73525 Schwäbisch Gmünd<br />Deutschland</p>\
            <p>Tel.: 07171 66241<br />E-Mail: info@bruno-welz.de</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Imhofstr. 2");
        assert_eq!(info.postcode, "73525");
        assert_eq!(info.city, "Schwäbisch Gmünd");
        assert_eq!(info.phone, "07171 66241");
        assert_eq!(info.email, "info@bruno-welz.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}
