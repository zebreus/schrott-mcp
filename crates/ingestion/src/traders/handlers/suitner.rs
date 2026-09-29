//! Edelmetallhandel Lübeck Dennis Suitner (Lübeck): per-gram purchase
//! prices in the Shopware category listing `ankaufspreise/` (page 1 + 2,
//! merged and deduped). Each `div.product--box` carries the title in
//! `a.product--title` and the quote in `span.price--default` ("38,62 €").
//! Only fineness rows ("Ankauf 333 Gold", "Ankauf 999 Silber",
//! "Ankauf Platin 999" …) are per-gram quotes (detail pages prove
//! "Aktueller Preis im Ankauf, per Gramm"; 999 gold 117,76 vs ~120,95
//! spot cross-checks): they map to `gold`/`silber`/`platin`/`palladium`/
//! `zahngold` with fineness in the variant, honestly EUR/g. Coin and bar
//! rows ("1 Unze …", "10 Gramm Goldbarren", "Silber Unzen") quote ITEM
//! prices, not per-gram rates — deriving gram prices by dividing weights
//! out of labels would be guesswork, so all 24 skip loudly. Page 3+
//! would be silently missed; the fixed two-page fetch is documented and
//! the `?p=2` fetch fails loudly on redesign.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sh-lubeck-23552-edelmetallhandel-lubeck-dennis-suitner";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.goldankauf-luebeck.de/impressum";

pub const URL: &str = "https://www.goldankauf-luebeck.de/ankaufspreise/";
/// Second listing page (verified live: 15 rows, no overlap with page 1).
/// A future page 3 would be missed until noticed — documented, not hidden.
pub const URL_PAGE2: &str = "https://www.goldankauf-luebeck.de/ankaufspreise/?p=2";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, p1) = fetch_text(client, URL).await?;
    let (_, p2) = fetch_text(client, URL_PAGE2).await?;
    let (mut rows, mut skipped_labels) = parse(&p1)?;
    let (rows2, skips2) = parse(&p2)?;
    rows.extend(rows2);
    skipped_labels.extend(skips2);
    // Page 1 + 2 must not overlap: dedupe identical (label, price) pairs.
    let mut seen = std::collections::HashSet::new();
    rows.retain(|(l, p, _)| seen.insert((l.clone(), p.to_bits())));
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
            None => skipped_labels.push(format!(
                "{label} ({price:.2} {unit}, Stückpreis ohne Gramm-Notierung)"
            )),
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
        byte_len: p1.len() + p2.len(),
        published_at: None,
    })
}

/// Explicit label → (material, variant) mapping. Only fineness rows
/// ("Ankauf 333 Gold", "Ankauf Platin 950") are per-gram quotes.
/// Coin/bar/item rows ("1 Unze …", "… Gramm Goldbarren", "Silber Unzen",
/// "Sovereign", "Dukaten", "Rubel", "Kronen", "Franken", "Mark") quote
/// item prices and skip loudly — no weight division out of labels.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    for item in [
        "unze",
        "gramm",
        "sovereign",
        "dukaten",
        "rubel",
        "kronen",
        "franken",
        "mark",
        "philharmoniker",
        "maple",
        "krügerrand",
        "rand",
        "münze",
        "muenze",
        "münz",
        "vreneli",
        "corona",
        "silberbarren",
    ] {
        if l.contains(item) {
            return None;
        }
    }
    if l.contains("zahngold") {
        return Some(("zahngold", ""));
    }
    if l.contains("platin") {
        return Some(("platin", fineness(&l)));
    }
    if l.contains("palladium") {
        return Some(("palladium", fineness(&l)));
    }
    if l.contains("silber") {
        return Some(("silber", fineness(&l)));
    }
    if l.contains("gold") {
        return Some(("gold", fineness(&l)));
    }
    None
}

/// First fineness run in the label ("Ankauf 585 Gold" → "585").
fn fineness(l: &str) -> &'static str {
    for fin in [
        "999", "986", "950", "925", "900", "835", "800", "750", "585", "333",
    ] {
        if l.contains(fin) {
            return fin;
        }
    }
    ""
}

/// Bespoke unit rule for THIS listing: fineness rows are per-gram quotes
/// (detail pages: "Aktueller Preis im Ankauf, per Gramm"). The listing
/// itself prints no unit, so the fineness match IS the unit proof —
/// anything else skips loudly via grade_for, never a default.
fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html
        .find("listing--container")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Produktliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("listing--bottom")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Produktliste unvollständig".to_owned(),
        })?;
    let frag = Html::parse_fragment(&format!("<div>{}</div>", &tail[..end]));
    let box_sel = Selector::parse("div.product--box").expect("valid selector");
    let title_sel = Selector::parse("a.product--title").expect("valid selector");
    let price_sel = Selector::parse("span.price--default").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for b in frag.select(&box_sel) {
        let label: String = b
            .select(&title_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let price_text: String = b
            .select(&price_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();
        if label.is_empty() || label.len() > 120 {
            continue;
        }
        let Some(price) = parse_eur(&price_text) else {
            skips.push(format!(
                "{label} (Preis unverständlich: {})",
                price_text.trim()
            ));
            continue;
        };
        rows.push((label, price, "EUR/g"));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Produktliste leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after
/// "Gesetzliche Anbieterkennung:" ("Dennis Suitner" / "Mühlenbrücke 1" /
/// "23552 Lübeck" / "Telefon:" / "E-Mail:"). Missing anchor → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    if !doc
        .root_element()
        .text()
        .collect::<String>()
        .contains("Gesetzliche Anbieterkennung:")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Anbieterkennzeichnung fehlt".to_owned(),
        });
    }
    let p_sel = Selector::parse("p").expect("valid selector");
    let mut lines: Vec<String> = Vec::new();
    for el in doc.select(&p_sel) {
        let ls: Vec<String> = el
            .inner_html()
            .split("<br")
            .map(|s| strip_fragment(s))
            .filter(|s| !s.is_empty())
            .collect();
        if ls.iter().any(|l| l.contains("Suitner")) {
            lines = ls;
            break;
        }
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let (mut phone, mut email) = (String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = ci.to_owned();
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                continue;
            }
        }
        if let Some(v) = line.strip_prefix("Telefon:") {
            phone = v.trim().to_owned();
        } else if let Some(v) = line.strip_prefix("E-Mail:") {
            email = v.trim().to_owned();
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

    // Real Shopware box shape (title + price spans), trimmed to 3 boxes.
    const FIXTURE: &str = "<div class=\"listing--container\"><div class=\"listing\">\
        <div class=\"product--box\"><a class=\"product--title\" title=\"Ankauf 585 Gold\">Ankauf 585 Gold</a>\
        <span class=\"price--default is--nowrap\"> 68,01&nbsp;&euro; * </span></div>\
        <div class=\"product--box\"><a class=\"product--title\" title=\"Ankauf 1 Unze Feingold Krügerrand\">Ankauf 1 Unze Feingold Krügerrand</a>\
        <span class=\"price--default is--nowrap\"> 3.677,90&nbsp;&euro; * </span></div>\
        <div class=\"product--box\"><a class=\"product--title\" title=\"Ankauf Platin 950\">Ankauf Platin 950</a>\
        <span class=\"price--default is--nowrap\"> 42,07&nbsp;&euro; * </span></div>\
        </div></div><div class=\"listing--bottom\"></div>";

    #[test]
    fn boxes_parse_with_units() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 3);
        assert!(skips.is_empty());
        assert_eq!(rows[0], ("Ankauf 585 Gold".to_owned(), 68.01, "EUR/g"));
        assert_eq!(rows[1].1, 3677.9);
        assert_eq!(rows[2], ("Ankauf Platin 950".to_owned(), 42.07, "EUR/g"));
        assert!(parse("<div>Kein Shop hier</div>").is_err());
        assert!(parse("<div class=\"listing--container\">ohne Ende").is_err());
    }

    #[test]
    fn fineness_maps_items_skip() {
        assert_eq!(grade_for("Ankauf 585 Gold"), Some(("gold", "585")));
        assert_eq!(
            grade_for("Ankauf 999 Gold Schmelzware"),
            Some(("gold", "999"))
        );
        assert_eq!(
            grade_for("Ankauf Zahngold (ohne Zähne)"),
            Some(("zahngold", ""))
        );
        assert_eq!(grade_for("Ankauf 925 Silber"), Some(("silber", "925")));
        assert_eq!(grade_for("Ankauf Platin 999"), Some(("platin", "999")));
        assert_eq!(
            grade_for("Ankauf Palladium 999"),
            Some(("palladium", "999"))
        );
        // Item prices, never gram rates.
        for label in [
            "Ankauf 1 Unze Feingold Krügerrand",
            "10 Gramm Goldbarren",
            "1 Gramm Goldbarren",
            "Ankauf Silber Unzen (handelsfähig)",
            "Ankauf 1 Dukaten 986 Gold - Österreich - 1915",
            "Ankauf 1kg Silberbarren",
            "Ankauf 2 Rand Goldmünze - Südafrika",
        ] {
            assert_eq!(grade_for(label), None, "{label}");
        }
    }

    #[test]
    fn impressum_anbieterkennzeichnung() {
        let imp = "<h1>Impressum</h1><p>Gesetzliche Anbieterkennung:</p>\
            <p><strong>Dennis Suitner</strong><br>TraveAntik Travemünde e.K.<br>Mühlenbrücke 1<br>\
            23552 Lübeck<br>Deutschland<br>Telefon: 0451 73993<br>E-Mail: info@edelmetallhandel-luebeck.de<br></p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Mühlenbrücke 1");
        assert_eq!(info.postcode, "23552");
        assert_eq!(info.city, "Lübeck");
        assert_eq!(info.phone, "0451 73993");
        assert_eq!(info.email, "info@edelmetallhandel-luebeck.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}
