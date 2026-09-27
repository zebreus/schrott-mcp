//! EDELCAT GmbH (Nettetal): catalyst recycler. NOTE on the URL: the
//! assigned page `/edelmetallpreise` shows only external chart images
//! (kitconet/weblinks247 GIFs under "Basismetalle"/"Edelmetallpreise" —
//! no numbers in the HTML), so there is nothing to scrape there. The
//! only written purchase prices on the site live on `/katalog`
//! (`div.list-text-inner` items): "Preise bis zu 350,00.-Euro/Stück"
//! (high-grade catalysts) and "Wert von bis 200,00.-Euro/Stück" (small
//! inconspicuous ones). Both are upper bounds: price = price_max =
//! advertised value, confidence 0.5, kind upto — never price_max
//! without kind.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-nettetal-edelcat";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.edelcat.de/impressum";

/// `/katalog`, not `/edelmetallpreise` (see module docs): the price
/// page assignment holds no numbers, only third-party chart images.
pub const URL: &str = "https://www.edelcat.de/katalog";

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
    for (label, price, unit) in rows {
        let Some((material, variant)) = grade_for(&label) else {
            skipped_labels.push(format!("{label} (Sorte unverständlich)"));
            continue;
        };
        // "bis zu"/"von bis" rows carry the bound as price_max at 0.5;
        // anything else would be an exact list price (never assumed).
        let (price_kind, price_max, confidence) = if is_upto(&label) {
            ("upto", Some(price), Some(0.5))
        } else {
            ("exact", None, Some(1.0))
        };
        prices.push(ScrapedPrice {
            material,
            variant,
            price,
            currency: "EUR",
            unit,
            price_kind,
            price_min: None,
            price_max,
            confidence,
            label,
        });
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
        published_at: None,
    })
}

/// Explicit group → material. Specific before generic: "hochwertig"
/// must not fall through to a lesser arm.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    if l.contains("hochwertig") {
        Some(("katalysatoren", "hochwertig"))
    } else if l.contains("klein") {
        Some(("katalysatoren", "klein"))
    } else {
        None
    }
}

/// Bespoke: this page quotes catalysts per piece only ("350,00.-Euro/
/// Stück"). Anything without "Stück" skips loudly — a per-gram or
/// per-kg figure recorded as per-piece would be orders off.
fn unit_of(t: &str) -> Option<&'static str> {
    if t.to_lowercase().contains("stück") {
        Some("EUR/Stk")
    } else {
        None
    }
}

/// "bis zu" (and the page's own "Wert von bis …" phrasing) mark upper
/// bounds; anything else with € and digits would be an exact list price.
fn is_upto(t: &str) -> bool {
    let l = t.to_lowercase();
    l.contains("bis zu") || l.contains("von bis")
}

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    // Window: the prose list between the catalog-download heading and
    // the "ALLE ANGABEN OHNE GEWÄHR" disclaimer. Nav, footer and brand
    // catalog download links stay outside.
    let start = html
        .find("Kataloge zum Download")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Katalog-Block fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("ALLE ANGABEN OHNE GEW").unwrap_or(tail.len());
    let window = &tail[..end];
    // Content elements only: the list items sit in div.list-text-inner;
    // <script>/<style>/JSON-LD must never read as text.
    let doc = Html::parse_fragment(window);
    let sel = Selector::parse("div.list-text-inner p").expect("valid selector");
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for el in doc.select(&sel) {
        let t = el.text().collect::<String>();
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if t.is_empty() || !t.contains("bis") {
            continue;
        }
        let Some(price) = parse_eur(&t) else { continue };
        if !t.contains('€') && !t.to_lowercase().contains("euro") {
            continue;
        }
        match unit_of(&t) {
            Some(unit) => rows.push((t, price, unit)),
            None => skipped.push(format!("{t} (Einheit unverständlich: {t})")),
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Katalogpreise".to_owned(),
        });
    }
    Ok((rows, skipped))
}

/// Bespoke contact extraction for THIS impressum only: the
/// `div.imp-container` holds the firm `<p>` ("EDELCAT GmbH"), the street
/// and PLZ-city `<p>`s, a KONTAKT box with `tel:`/`mailto:` links and a
/// "Geschäftsführer:" block. Missing anchors → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let cont = Selector::parse("div.imp-container").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let a = Selector::parse("a").expect("valid selector");
    let Some(root) = doc.select(&cont).next() else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    };
    let texts: Vec<String> = root
        .select(&p)
        .map(|el| {
            el.text()
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();
    if !texts.iter().any(|t| t.contains("EDELCAT GmbH")) {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmen-Block fehlt".to_owned(),
        });
    }
    // Street: the <p> holding this trader's street (verified live).
    let mut street = String::new();
    for t in &texts {
        if t.contains("Wambacherstr.") {
            street = t.clone();
            break;
        }
    }
    let (mut postcode, mut city) = (String::new(), String::new());
    for t in &texts {
        let toks: Vec<&str> = t.split_whitespace().collect();
        for (k, tok) in toks.iter().enumerate() {
            if tok.len() == 5 && tok.chars().all(|c| c.is_ascii_digit()) {
                if let Some(ci) = toks.get(k + 1) {
                    if ci.chars().next().is_some_and(|c| c.is_uppercase()) {
                        postcode = (*tok).to_owned();
                        city = (*ci).to_owned();
                        break;
                    }
                }
            }
        }
        if !postcode.is_empty() {
            break;
        }
    }
    if street.is_empty() || postcode.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    }
    // Phone and mail come from the KONTAKT box links (own rules: the
    // phone token filter would stop an e-mail at the first letter).
    let mut phone = String::new();
    let mut email = String::new();
    for el in root.select(&a) {
        if let Some(href) = el.value().attr("href") {
            if let Some(num) = href.strip_prefix("tel:") {
                phone = num.trim().to_owned();
            } else if href.starts_with("mailto:") {
                email = el.text().collect::<String>().trim().to_owned();
            }
        }
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
    use super::{extract_info, grade_for, is_upto, parse, unit_of};

    // Real structure, shortened: prose list items in div.list-text-inner
    // between the download heading and the disclaimer; brand catalog
    // links and footer stay outside the window.
    const FIXTURE: &str = "<p>hier möchten wir Ihnen unsere aktuellen \
        Katalysatoren Kataloge zum Download anbieten.</p>\
        <div class=\"list-text-inner\"><p>Wenn Sie als Autoverwerter oder Händler \
        Ihre Katalysatoren für einen Mix-Preis verkaufen, können Sie unter \
        Umständen erhebliche Wertverluste erleiden.</p></div>\
        <div class=\"list-text-inner\"><p>Gerade für die hochwertigen Katalysatoren \
        sind Preise bis zu 350,00.-Euro/Stück möglich.</p></div>\
        <div class=\"list-text-inner\"><p>Oftmals haben auch kleine unscheinbare \
        Katalysatoren einen Wert von bis 200,00.-Euro/Stück.</p></div>\
        <div class=\"list-text-inner\"><p>Gerne übernehmen wir für Sie die \
        Sortierung nach Gruppen um einen exakten Wert Ihrer Ware zu ermitteln.</p></div>\
        <p>ALLE ANGABEN OHNE GEW&Auml;HR.</p>\
        <footer><p>BMW Katalysatoren Katalog</p></footer>";

    #[test]
    fn bis_zu_stueckpreise() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty(), "{skips:?}");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].1, 350.0);
        assert_eq!(rows[0].2, "EUR/Stk");
        assert!(is_upto(&rows[0].0), "bis zu flag");
        assert_eq!(rows[1].1, 200.0);
        assert!(is_upto(&rows[1].0), "von bis flag");
        assert_eq!(grade_for(&rows[0].0), Some(("katalysatoren", "hochwertig")));
        assert_eq!(grade_for(&rows[1].0), Some(("katalysatoren", "klein")));
        assert_eq!(grade_for("Mix-Preis"), None);
        // Only this page's spellings: per-gram/per-kg figures skip loudly.
        assert_eq!(unit_of("350,00.-Euro/Stück"), Some("EUR/Stk"));
        assert_eq!(unit_of("Platin = 42,00.-Euro/Gramm"), None);
    }

    #[test]
    fn window_and_anchors_hold() {
        assert!(parse("<p>Nichts hier</p>").is_err());
        assert!(parse("<p>Kataloge zum Download</p><p>ohne Preise</p>").is_err());
        assert!(extract_info("<div><p>Neu hier</p></div>").is_err());
    }

    #[test]
    fn impressum_block() {
        let imp = "<div class=\"imp-container de\"><h1>Impressum</h1>\
            <div class=\"box dmNewParagraph\"><p><b>EDELCAT GmbH</b></p>\
            <p>Wambacherstr. 25 B</p><p>41334 Nettetal</p></div>\
            <div class=\"box dmNewParagraph\"><p>\
            <a href=\"tel:+4921578704710\">+4921578704710</a></p><p>\
            <a href=\"mailto:stephan.sitnevski@edelcat.de\">\
            stephan.sitnevski@edelcat.de</a></p></div>\
            <div class=\"box dmNewParagraph\"><p><b>Gesch&auml;ftsf&uuml;hrer:</b> \
            Christos R&ouml;ser</p></div></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Wambacherstr. 25 B");
        assert_eq!(info.postcode, "41334");
        assert_eq!(info.city, "Nettetal");
        assert_eq!(info.phone, "+4921578704710");
        assert_eq!(info.email, "stephan.sitnevski@edelcat.de");
    }
}
