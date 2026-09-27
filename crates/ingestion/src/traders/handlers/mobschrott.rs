//! Mobiler Schrotthandel Cottbus (Christian Lucia, Drebkau): exact
//! MONTHLY steel prices per tonne in a plain `<ul>` ("Auflistung der
//! gängigsten Posten für den November. Preise pro Tonne."). Live 27.09.2026:
//! 4 rows (Mischschrott 125, schwere Schere 135, Sorte 3 (Kernschrott) 165,
//! Handelsguss 135 €/t). NE-metal prices are phone-only ("tagesaktuell
//! erfragen") with no grade list, so nothing mappable comes from them.
//! "Sorte 3 (Kernschrott)" is skipped loudly: the page never proves it is
//! shear scrap (see deliberation in `grade_for`). Bare month name without
//! a year is not a date → `published_at` stays None.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bb-cottbus-mobiler-schrotthandel-cottbus-mobschrott";
/// Bespoke, live-verified impressum URL (homepage nav "Impressum.htm"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "http://mobschrott.de/Impressum.htm";

pub const URL: &str = "http://mobschrott.de/Schrottpreise.htm";

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
/// skipped.
///
/// Deliberation on the steel grades: "schwere Schere" maps to
/// `stahlschrott-scheren` ("Scherenschrott") with variant "schwer" — the
/// label itself proves shear scrap, in a monthly steel-price list at
/// shear-scrap prices. "Sorte 3 (Kernschrott)" does NOT map: Sorte 3 reads
/// like a BDSV steel grade, but the page never defines it as scheren, so
/// cramming it into `stahlschrott-scheren` (or `mischschrott`) would be a
/// guess → loud skip + proposal for a new `stahlschrott-sorte-3` material.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("schwere schere") || (l.contains("schere") && !l.contains("sorte")) {
        Some(("stahlschrott-scheren", "schwer"))
    } else if l.contains("handelsguss") || l.contains("guss") {
        Some(("eisenschrott-gussbruch", "Handel"))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after the
/// "Inhaber der Domain …" heading holds firm lines + street + PLZ city +
/// a "Telefon:" line, and the sibling mailto link holds the email (read
/// from the href — `text()` would glue it to the address). Missing anchors
/// mean the page changed shape → loud error, never a guessed fallback.
/// Note: the address is Drebkau (Auraser Dorfstraße 2), not Cottbus.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let mailto = Selector::parse("a[href^=\"mailto:\"]").expect("valid selector");
    let anchor = doc
        .select(&h1)
        .find(|h| h.text().collect::<String>().contains("Inhaber der Domain"));
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Inhaber-Block fehlt".to_owned(),
        });
    };
    let addr_p = anchor
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let (mut street, mut postcode, mut city, mut phone) =
        (String::new(), String::new(), String::new(), String::new());
    if let Some(p) = addr_p {
        let mut lines = Vec::new();
        for part in p.inner_html().split("<br") {
            // Drop tag remnants first ("<br />" → "/>"), then strip.
            let frag = part.split('>').nth(1).unwrap_or(part);
            let t = strip_tags(frag);
            if !t.is_empty() {
                lines.push(t);
            }
        }
        for (k, line) in lines.iter().enumerate() {
            let mut it = line.split_whitespace();
            if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    city = ci.to_owned();
                    // Street: nearest line above with a house number.
                    for prev in lines[..k].iter().rev() {
                        if prev.chars().any(|c| c.is_ascii_digit()) && !prev.starts_with("Telefon")
                        {
                            street = prev.clone();
                            break;
                        }
                    }
                }
            }
            if let Some(rest) = line.strip_prefix("Telefon:") {
                phone = rest.trim().to_owned();
            }
        }
    }
    let email = doc
        .select(&mailto)
        .filter_map(|a| a.value().attr("href"))
        .find_map(|h| h.strip_prefix("mailto:"))
        .unwrap_or_default()
        .trim()
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

/// Strip tags from a fragment (entities already decoded by html5ever).
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
    // Window: the monthly list only — nav/footer must never leak in.
    let start = html
        .find("Auflistung der gängigsten Posten")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("Facebook-Seite").unwrap_or(tail.len());
    let window = &tail[..end];
    // The unit lives in the list header ("Preise pro Tonne"), not per row:
    // without it we refuse to guess — tonne-as-kilo would be a 1000x error.
    if !window.to_lowercase().contains("preise pro tonne") {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Einheit unklar (kein pro-Tonne-Vermerk)".to_owned(),
        });
    }
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    // Only list items, never scripts/styles (the page carries Dreamweaver JS).
    let li = Selector::parse("li").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for el in doc.select(&li) {
        let raw: String = el.text().collect();
        let text = raw.replace(['\u{a0}'], " ");
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if text.len() > 120 {
            continue; // prose, never a label.
        }
        let Some((label, rest)) = text.split_once(':') else {
            skips.push(format!("{text} (kein Preis)"));
            continue;
        };
        let label = label.trim().to_owned();
        let Some(price) = parse_eur(rest) else {
            skips.push(format!("{label} (kein Preis)"));
            continue;
        };
        // An explicit per-row kg unit would contradict the page default.
        let Some(unit) = unit_of(&text) else {
            skips.push(format!("{label} (Einheit unverständlich: {text})"));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste leer".to_owned(),
        });
    }
    // Only a bare month name ("für den November", no year) — not a date.
    Ok((None, rows, skips))
}

/// Bespoke unit rule for THIS list: the header fixes EUR/t. A per-row kg
/// mention contradicts it and skips loudly instead of guessing.
fn unit_of(row: &str) -> Option<&'static str> {
    let lower = row.to_lowercase();
    if lower.contains("kg") {
        None
    } else {
        Some("EUR/t")
    }
}

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    // Real live-list excerpts (verbatim rows + header/terminator).
    const FIXTURE: &str = concat!(
        "<p>Hier finden Sie eine Auflistung der gängigsten Posten für den November. ",
        "Preise pro Tonne.</p><br /><ul>",
        "<li>Mischschrott:\u{a0}\u{a0}125,00 €</li>",
        "<li>schwere Schere:\u{a0}135,00 €</li>",
        "<li>Sorte 3 (Kernschrott):\u{a0}165,00 €</li>",
        "<li>Handelsguss:\u{a0}135,00 €</li>",
        "</ul><br/><p>Sie finden die monatsaktuellen Schrottpreise ausführlicher<br/> ",
        "auch auf unserer <a href=\"http://www.facebook.com/\">Facebook-Seite</a>.</p>",
    );

    #[test]
    fn list_parses_per_tonne() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at, None, "bare month is not a date");
        assert_eq!(rows.len(), 4, "{rows:?}");
        assert!(skips.is_empty());
        assert_eq!(rows[0], ("Mischschrott".to_owned(), 125.0, "EUR/t"));
        assert_eq!(rows[1], ("schwere Schere".to_owned(), 135.0, "EUR/t"));
        assert_eq!(
            rows[2],
            ("Sorte 3 (Kernschrott)".to_owned(), 165.0, "EUR/t")
        );
        assert_eq!(rows[3], ("Handelsguss".to_owned(), 135.0, "EUR/t"));
    }

    #[test]
    fn anchors_and_unit_are_loud() {
        // Missing list anchor: error, not silent success.
        assert!(parse("<ul><li>Mischschrott: 125,00 €</li></ul>").is_err());
        // Missing tonne header: error, never a guessed unit.
        let html = FIXTURE.replace("Preise pro Tonne.", "Preise.");
        assert!(parse(&html)
            .expect_err("unit anchor")
            .to_string()
            .contains("Einheit"));
        // Per-row kg mention contradicts the page default → loud skip.
        let html = FIXTURE.replacen("135,00 €</li>", "135,00 € pro kg</li>", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 3);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("schwere Schere"));
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<div><h1>Inhaber der Domain<br />und verantwortlich:</h1><br/>\
            <p>Mobiler Schrottankauf Christian Lucia<br />Christian Lucia<br />\
            Auraser Dorfstraße 2 <br />03116 Drebkau<br />Telefon: 035602 22909</p>\
            <br /><a href=\"mailto:c.lucia@t-online.de\">c.lucia@t-online.de</a></div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Auraser Dorfstraße 2");
        assert_eq!(info.postcode, "03116");
        assert_eq!(info.city, "Drebkau");
        assert_eq!(info.phone, "035602 22909");
        assert_eq!(info.email, "c.lucia@t-online.de");
        assert!(super::extract_info("<h1>Neu</h1><p>x</p>").is_err());
    }

    #[test]
    fn mapping_proves_scheren_only() {
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(
            grade_for("schwere Schere"),
            Some(("stahlschrott-scheren", "schwer"))
        );
        assert_eq!(
            grade_for("Handelsguss"),
            Some(("eisenschrott-gussbruch", "Handel"))
        );
        // Sorte 3 unproven → loud skip (proposal: stahlschrott-sorte-3).
        assert_eq!(grade_for("Sorte 3 (Kernschrott)"), None);
    }
}
