//! MADI Metall Recycling GmbH (Hamburg-Hammerbrook): "bis zu" (up-to)
//! Tagespreise in Bricks cards on the homepage — the price block IS the
//! homepage, so `URL` is the homepage by design (like kupferhelden).
//! Window ("Aktuelle Metallschrott Preise" … "Know-How aus der Welt der
//! Metalle"): the lede sentence carries Eisenschrott/Mischschrott per
//! tonne, four cards carry Kupfer/Stahl/Kabel/Messing. Every row says
//! "bis zu", so price = price_max = advertised value at confidence 0.5.
//! No price date anywhere → `published_at` stays `None`. (A second seed
//! entry, hh-nenndorf-madi-metall-recycling, shares this same website.)

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "hh-hammerbrook-madi-metall-recycling";
/// Bespoke, live-verified impressum URL (site footer link). A move fails
/// the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.madi-schrott.de/impressum-datenschutz/";

/// The price block lives on the homepage — no separate price page exists.
pub const URL: &str = "https://www.madi-schrott.de";

const START_ANCHOR: &str = "Aktuelle Metallschrott Preise";
const END_ANCHOR: &str = "Know-How aus der Welt der Metalle";
const LEDE_ANCHOR: &str = "zahlen wir bis zu";
const CARD_HEAD_MARKER: &str = "fr-feature-card-charlie__heading\">";
const CARD_LEDE_MARKER: &str = "fr-feature-card-charlie__lede\">";

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
    // All rows are advertised upper bounds ("bis zu … je nach Sorte"):
    // price = price_max = beworben, confidence 0.5, kind upto.
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit,
                price_kind: "upto",
                price_min: None,
                price_max: Some(price),
                confidence: Some(0.5),
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
        published_at: None,
    })
}

/// Explicit label → (material, variant) mapping. The iron generics all
/// land on `mischschrott` with the trader's own word as variant, so the
/// three per-tonne rows never collapse. Anything unlisted skips loudly.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("kupferschrott") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("kabelschrott") {
        // Generic cable; the card photo shows copper cable
        // ("kupfer-kabel-70-prozent"), so kabel-kupfer, not kabel-alu.
        Some(("kabel-kupfer", ""))
    } else if l.contains("messingschrott") {
        Some(("messing", ""))
    } else if l.contains("eisenschrott") {
        Some(("mischschrott", "Eisenschrott"))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", "Mischschrott"))
    } else if l.contains("stahlschrott") {
        Some(("mischschrott", "Stahlschrott"))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: anchored on the
/// "Pflichtangaben" eyebrow + the "Impressum" heading — without both the
/// page changed shape → loud error. The address `<p>` holds street + PLZ
/// city + "Tel. …" lines over `<br />`; the mail comes from the `mailto:`
/// href (scraper `text()` would glue it to the phone line).
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let a = Selector::parse("a[href^=\"mailto:\"]").expect("valid selector");
    let body_text: String = doc.root_element().text().collect();
    if !body_text.contains("Pflichtangaben") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Pflichtangaben-Block fehlt".to_owned(),
        });
    }
    if !doc
        .select(&h2)
        .any(|h| h.text().collect::<String>().trim() == "Impressum")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let addr_p = doc
        .select(&p)
        .find(|el| el.inner_html().contains("Billwerder Steindamm"));
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let mut lines = Vec::new();
    for part in addr_p.inner_html().split("<br") {
        let t = strip_fragment(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    let (mut street, mut postcode, mut city, mut phone) =
        (String::new(), String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        if let Some(rest) = line.strip_prefix("Tel.") {
            phone = rest.trim().to_owned();
            continue;
        }
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, w| a + " " + w);
                if k > 0 && street.is_empty() {
                    street = lines[k - 1].clone();
                }
            }
        }
    }
    if street.is_empty() {
        for line in &lines {
            if line.contains("Steindamm") {
                street = line.clone();
                break;
            }
        }
    }
    let email = doc
        .select(&a)
        .filter_map(|el| el.value().attr("href"))
        .next()
        .and_then(|h| h.strip_prefix("mailto:"))
        .map(|s| s.to_owned())
        .unwrap_or_default();
    let email = if email.contains("madi-schrott.de") {
        email
    } else {
        String::new()
    };
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

/// Strip tags from a `<br`-split fragment (html5ever already decoded
/// entities). Fragments start with a tag remnant — drop everything up to
/// the first '>' first, or attributes parse as text.
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

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html.find(START_ANCHOR).ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisblock fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find(END_ANCHOR).ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisblock-Ende fehlt".to_owned(),
    })?;
    let window = &tail[..end];
    let mut rows: Vec<(String, f64, &'static str)> = Vec::new();
    let mut skips: Vec<String> = Vec::new();
    // Lede sentence: "Für Eisenschrott zahlen wir bis zu 220 €/t, für
    // Mischschrott bis zu 210 €/t - je nach Sortenmischung." — the whole
    // sentence sits in one <strong>; without it the intro changed shape
    // → loud error, never a silent card-only run.
    let lede = window.find(LEDE_ANCHOR).ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preis-Satz fehlt".to_owned(),
    })?;
    let lstart = window[..lede]
        .rfind("<strong>")
        .map(|i| i + 8)
        .unwrap_or(lede);
    let send = window[lede..]
        .find("</strong>")
        .map(|e| lede + e)
        .unwrap_or(window.len());
    let lede_text = strip_tags(&window[lstart..send]);
    let norm = lede_text.replacen("Für ", "für ", 1);
    for chunk in norm.split("für ").skip(1) {
        let chunk = chunk.replace("&nbsp;", " ").replace(['\u{a0}'], " ");
        let label = chunk
            .split(" zahlen")
            .next()
            .unwrap_or("")
            .split(" bis zu")
            .next()
            .unwrap_or("")
            .split(',')
            .next()
            .unwrap_or("")
            .trim()
            .to_owned();
        let Some(price) = parse_eur(&chunk) else {
            skips.push(format!("{label} (Preis unverständlich: {chunk})"));
            continue;
        };
        let Some(unit) = unit_of(&chunk) else {
            skips.push(format!("{label} (Einheit unverständlich: {chunk})"));
            continue;
        };
        if label.is_empty() || label.len() > 40 {
            skips.push(format!("Preis ohne Sorte: {chunk}"));
            continue;
        }
        rows.push((label, price, unit));
    }
    // Cards: price-first `h3` + `<strong>` grade in the following lede
    // `<p>`. No cards at all means the grid moved → loud error.
    let mut k = 0;
    let mut cards = 0;
    while let Some(h) = window[k..].find(CARD_HEAD_MARKER) {
        let hs = k + h + CARD_HEAD_MARKER.len();
        let Some(he) = window[hs..].find("</h3>") else {
            break;
        };
        let price_text = window[hs..hs + he]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        k = hs + he;
        let Some(d) = window[k..].find(CARD_LEDE_MARKER) else {
            skips.push(format!("{price_text} (Sorte fehlt)"));
            continue;
        };
        let ds = k + d + CARD_LEDE_MARKER.len();
        let Some(de) = window[ds..].find("</p>") else {
            skips.push(format!("{price_text} (Sorte fehlt)"));
            continue;
        };
        let lede_html = &window[ds..ds + de];
        k = ds + de;
        let label = match lede_html.find("<strong>") {
            Some(s) => {
                let ls = s + 8;
                match lede_html[ls..].find("</strong>") {
                    Some(e) => lede_html[ls..ls + e].trim().to_owned(),
                    None => String::new(),
                }
            }
            None => String::new(),
        };
        if label.is_empty() {
            skips.push(format!("{price_text} (Sorte fehlt)"));
            continue;
        }
        let Some(price) = parse_eur(&price_text) else {
            skips.push(format!("{label} (Preis unverständlich: {price_text})"));
            continue;
        };
        let Some(unit) = unit_of(&price_text) else {
            skips.push(format!("{label} (Einheit unverständlich: {price_text})"));
            continue;
        };
        cards += 1;
        rows.push((label, price, unit));
    }
    if cards == 0 {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preiskarten".to_owned(),
        });
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preispaare".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS block (live: "bis zu 9,50 €/kg",
/// "bis zu 220 €/t"). Only kg/t exist here — anything else skips loudly
/// at the call site.
fn unit_of(text: &str) -> Option<&'static str> {
    let lower = text.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|w| w == "t" || w == "to")
    {
        Some("EUR/t")
    } else {
        None
    }
}

/// Plain text of an HTML slice (no scripts/styles in this window — the
/// lede `<p>` only — but entities still need decoding).
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
    out.replace("&nbsp;", " ").replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    /// Real live markup: lede sentence + two of the four Bricks cards.
    const FIXTURE: &str = "Aktuelle Metallschrott Preise<h2>Erhalten Sie gutes Geld für Buntmetalle &amp; Altmetallschrott.</h2>\
        <p><strong>Für Eisenschrott zahlen wir bis zu 220 €/t, für Mischschrott&nbsp;  bis zu 210 €/t - je nach Sortenmischung.</strong> Nachfolgend finden Sie weitere aktuelle Preise.</p>\
        <ul><li><div><h3 class=\"brxe-heading fr-feature-card-charlie__heading\">bis zu 9,50 €/kg</h3>\
        <p class=\"brxe-text-basic fr-feature-card-charlie__lede\"><strong>Kupferschrott</strong>, je nach Sorte.</p></div></li>\
        <li><div><h3 class=\"brxe-heading fr-feature-card-charlie__heading\">bis zu 220 €/t</h3>\
        <p class=\"brxe-text-basic fr-feature-card-charlie__lede\"><strong>Stahlschrott</strong>, je nach Sorte.</p></div></li></ul>\
        Know-How aus der Welt der Metalle";

    #[test]
    fn lede_and_cards_parse() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 4);
        assert!(skips.is_empty());
        assert_eq!(rows[0], ("Eisenschrott".to_owned(), 220.0, "EUR/t"));
        assert_eq!(rows[1], ("Mischschrott".to_owned(), 210.0, "EUR/t"));
        assert_eq!(rows[2], ("Kupferschrott".to_owned(), 9.5, "EUR/kg"));
        assert_eq!(rows[3], ("Stahlschrott".to_owned(), 220.0, "EUR/t"));
    }

    #[test]
    fn anchors_and_units_fail_loudly() {
        assert!(parse("<p>Sonst was</p>").is_err(), "start anchor missing");
        let no_end = FIXTURE.replacen("Know-How aus der Welt der Metalle", "Sonst was", 1);
        assert!(parse(&no_end).is_err(), "end anchor missing");
        let no_lede = FIXTURE.replacen("zahlen wir bis zu", "geben Sie uns", 1);
        assert!(parse(&no_lede).is_err(), "lede sentence missing");
        // A card without parseable unit skips loudly, valid rows survive.
        let bad_unit = FIXTURE.replacen("bis zu 9,50 €/kg", "bis zu 9,50 €/Sack", 1);
        let (rows, skips) = parse(&bad_unit).expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Kupferschrott"));
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h1>Header</h1><p>Pflichtangaben</p><h2>Impressum</h2>\
            <p>MADI Metall Recycling GmbH\n</p>\
            <div><p>Billwerder Steindamm 15<br />20537 Hamburg<br />Tel. 0171-9001932<br />\
            <a href=\"mailto:info@madi-schrott.de\">info@madi-schrott.de</a></p></div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Billwerder Steindamm 15");
        assert_eq!(info.postcode, "20537");
        assert_eq!(info.city, "Hamburg");
        assert_eq!(info.phone, "0171-9001932");
        assert_eq!(info.email, "info@madi-schrott.de");
        assert!(extract_info("<h2>Impressum</h2><p>Neu hier</p>").is_err());
        assert!(extract_info("<p>Pflichtangaben</p><p>Ohne Titel</p>").is_err());
    }

    #[test]
    fn mapping_keeps_iron_generics_apart() {
        assert_eq!(grade_for("Kupferschrott"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Kabelschrott"), Some(("kabel-kupfer", "")));
        assert_eq!(grade_for("Messingschrott"), Some(("messing", "")));
        assert_eq!(
            grade_for("Eisenschrott"),
            Some(("mischschrott", "Eisenschrott"))
        );
        assert_eq!(
            grade_for("Mischschrott"),
            Some(("mischschrott", "Mischschrott"))
        );
        assert_eq!(
            grade_for("Stahlschrott"),
            Some(("mischschrott", "Stahlschrott"))
        );
        assert_eq!(grade_for("Aluminiumschrott"), None);
    }
}
