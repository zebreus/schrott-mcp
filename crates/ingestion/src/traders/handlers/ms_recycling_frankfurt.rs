//! MS Recycling Frankfurt e.K. (Frankfurt-Griesheim): nine exact-quoted
//! price *ranges* in GenerateBlocks cards below the `#preise` anchor
//! ("Kupferkabel / Haushaltskabel / 2,60 – 3,00 €/kg"). Every live row is
//! two-sided, so each becomes `price_kind: "range"` with price_min/max and
//! the midpoint as `price` at confidence 0.5 — never a silent exact. A
//! one-sided row ("bis zu" → upto, "ab" → approx) would get its own kind
//! deliberately per row, never price_max without kind. The tenth card
//! ("Weitere Sorten / Auf Anfrage") carries no price and skips loudly.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "he-frankfurt-griesheim-ms-recycling-frankfurt";
/// Bespoke, live-verified impressum URL (the site footer's own
/// "Impressum" link). A move fails the step loudly (fix the URL) —
/// never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.msr-frankfurt.de/impressum/";

pub const URL: &str = "https://www.msr-frankfurt.de/altmetall-ankauf/";

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
    for (label, price, price_min, price_max, price_kind, confidence, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit,
                price_kind,
                price_min,
                price_max,
                confidence,
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

/// Explicit label → (material, variant) mapping, specific before generic
/// ("kupfer" would otherwise catch "Kupferkabel" and "Kupferschrott").
/// Anything unlisted is skipped. The variant keeps the trader's own grade
/// wording so the two cable and the two Kupferschrott rows never collapse.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("schälkabel") || l.contains("schalkabel") {
        Some(("kabel-kupfer", "Schälkabel"))
    } else if l.contains("kupferkabel") {
        Some(("kabel-kupfer", "Kupferkabel"))
    } else if l.contains("millberry") || l.contains("kupferdraht") {
        // After the cable arms: "Schälkabel / mit dickem Kupferdraht"
        // is cable, not millberry.
        Some(("kupfer-millberry", ""))
    } else if l.contains("kupferschrott") {
        if l.contains("leicht") {
            Some(("kupfer-gemischt", "leicht"))
        } else if l.contains("schwer") {
            Some(("kupfer-gemischt", "schwer"))
        } else {
            Some(("kupfer-gemischt", ""))
        }
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("elektromotor") {
        Some(("elektromotoren", ""))
    } else if l.contains("eisenschrott") || l.contains("stahlschrott") {
        Some(("mischschrott", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the address `<p>`
/// (firm lines + street + "PLZ Ort (Stadtteil …)") sits before the
/// `<h2>Kontakt</h2>` heading, whose following `<p>` holds the labeled
/// "Telefon:"/"E-Mail:" lines. Missing anchors mean the page changed
/// shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    if !doc
        .select(&h2)
        .any(|h| h.text().collect::<String>().trim() == "Kontakt")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    }
    // Address block: the <p> whose <br> lines hold a PLZ line.
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for el in doc.select(&p) {
        let lines: Vec<String> = el
            .inner_html()
            .split("<br")
            .map(strip_fragment)
            .filter(|s| !s.is_empty())
            .collect();
        for (k, line) in lines.iter().enumerate() {
            let mut it = line.split_whitespace();
            if let (Some(pc), Some(_)) = (it.next(), it.next()) {
                if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    // "65933 Frankfurt/Main (Stadtteil Griesheim)": city runs
                    // to the parenthetical, which stays out.
                    city = line[pc.len()..]
                        .trim()
                        .split('(')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_owned();
                    if k > 0 {
                        street = lines[k - 1].clone();
                    }
                    break;
                }
            }
        }
        if !postcode.is_empty() {
            break;
        }
    }
    // "Kontakt" heading → next <p> holds the labeled lines.
    let mut phone = String::new();
    let mut email = String::new();
    for el in doc.select(&h2) {
        if el.text().collect::<String>().trim() != "Kontakt" {
            continue;
        }
        let sib = el
            .next_siblings()
            .filter_map(ElementRef::wrap)
            .find(|e| e.value().name() == "p");
        if let Some(p) = sib {
            for part in p.inner_html().split("<br") {
                let t = strip_fragment(part);
                if let Some(v) = t.strip_prefix("Telefon:") {
                    if phone.is_empty() {
                        phone = v.trim().to_owned();
                    }
                } else if let Some(v) = t.strip_prefix("E-Mail:") {
                    if email.is_empty() {
                        email = v.trim().to_owned();
                    }
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

/// Strip tags from a `<br>`-split fragment. Fragments start with a tag
/// remnant — drop everything up to the first '>' first, or attributes
/// parse as text.
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

/// One parsed card: (label, price, min, max, kind, confidence, unit).
type Row = (
    String,
    f64,
    Option<f64>,
    Option<f64>,
    &'static str,
    Option<f64>,
    &'static str,
);

/// Parse the price cards between the `#preise` anchor and the section CTA.
/// Returns (rows, skips); cards without a price ("Auf Anfrage") skip
/// loudly, 0 priced rows is an error.
fn parse(html: &str) -> Result<(Vec<Row>, Vec<String>), IngestError> {
    let start = html
        .find("id=\"preise\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbox fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("href=\"#cta\"")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbox unvollständig".to_owned(),
        })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(window);
    let card = Selector::parse("div.rounded-borders").expect("valid selector");
    let h3 = Selector::parse("h3").expect("valid selector");
    let psel = Selector::parse("p").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for el in frag.select(&card) {
        let head = el
            .select(&h3)
            .next()
            .map(|h| {
                h.text()
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default();
        let mut sub = String::new();
        let mut price_raw = String::new();
        for p in el.select(&psel) {
            let t = p
                .text()
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            if t.is_empty() {
                continue;
            }
            // The price cell is the one carrying digits + €.
            if t.contains('€') && parse_eur(&t).is_some() {
                if price_raw.is_empty() {
                    price_raw = t;
                }
            } else if sub.is_empty() {
                sub = t;
            }
        }
        let label = if sub.is_empty() {
            head.clone()
        } else {
            format!("{head} / {sub}")
        }
        .trim()
        .to_owned();
        if price_raw.is_empty() {
            skips.push(format!("{label} (Auf Anfrage, kein Preis)"));
            continue;
        }
        let Some(unit) = unit_of(&price_raw) else {
            skips.push(format!("{label} (Einheit unverständlich: {price_raw})"));
            continue;
        };
        let Some((price, min, max, kind, confidence)) = parse_span(&price_raw) else {
            skips.push(format!("{label} (Preis unverständlich: {price_raw})"));
            continue;
        };
        rows.push((label, price, min, max, kind, confidence, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preispaare".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke span parser for THIS page's price cells (live: "2,60 – 3,00
/// €/kg"). Two numbers → range (midpoint as price, 0.5); "bis zu" → upto
/// (bound as price_max, 0.5); "ab" → approx (bound as price_min, 0.5);
/// single → exact (1.0). The kind is deliberate per row.
fn parse_span(cell: &str) -> Option<(f64, Option<f64>, Option<f64>, &'static str, Option<f64>)> {
    let lower = cell.to_lowercase();
    // Range dashes: en dash (live), hyphen, em dash.
    for sep in ['–', '—', '-'] {
        if let Some((a, b)) = cell.split_once(sep) {
            if let (Some(lo), Some(hi)) = (parse_eur(a), parse_eur(b)) {
                if hi > 0.0 && hi >= lo {
                    return Some(((lo + hi) / 2.0, Some(lo), Some(hi), "range", Some(0.5)));
                }
            }
        }
    }
    if lower.contains("bis zu") || lower.contains("biszu") {
        return parse_eur(cell).map(|v| (v, None, Some(v), "upto", Some(0.5)));
    }
    if lower.split_whitespace().any(|w| w == "ab") {
        return parse_eur(cell).map(|v| (v, Some(v), None, "approx", Some(0.5)));
    }
    parse_eur(cell).map(|v| (v, None, None, "exact", Some(1.0)))
}

/// Bespoke unit matcher for THIS page's price cells (live: "€/kg").
/// Explicit-but-foreign (`/` or "pro" without kg/t) skips loudly at the
/// call site instead of guessing.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
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

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, parse_span, unit_of};

    // Real card shape (GenerateBlocks headline + sub + strong price),
    // two priced cards plus the "Auf Anfrage" card exactly like live.
    const FIXTURE: &str = "<div id=\"preise\"><div>\
        <div class=\"rounded-borders dropshadow transform padd20 gb-element-1e6c2a80\">\
        <h3 class=\"gb-headline gb-headline-text\">Kupferkabel</h3>\
        <p class=\"gb-headline gb-headline-text\">Haushaltskabel</p>\
        <p class=\"gb-headline gb-headline-text\"><strong>2,60 – 3,00 €/kg</strong></p></div>\
        <div class=\"rounded-borders dropshadow transform padd20 gb-element-acf6206c\">\
        <h3 class=\"gb-headline gb-headline-text\">Blei</h3>\
        <p class=\"gb-headline gb-headline-text\">z.B. Bleigeschirr, Bleistangen</p>\
        <p class=\"gb-headline gb-headline-text\"><strong>0,60 – 0,80 €/kg</strong></p></div>\
        <div class=\"rounded-borders dropshadow transform padd20 gb-element-254b8576\">\
        <h3 class=\"gb-headline gb-headline-text\">Weitere Sorten</h3>\
        <p class=\"gb-headline gb-headline-text\">Silber, Gold, Hartmetalle, Kupferschienen usw.</p>\
        <p class=\"gb-headline gb-headline-text\"><strong>Auf Anfrage</strong></p></div>\
        </div><div><a class=\"gb-button smooth-scroll\" href=\"#cta\">Anfragen</a></div></div>";

    #[test]
    fn ranges_midpoint_and_kinds() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Weitere Sorten"), "{skips:?}");
        assert_eq!(rows[0].0, "Kupferkabel / Haushaltskabel");
        assert!((rows[0].1 - 2.8).abs() < 1e-9, "midpoint as price");
        assert_eq!(rows[0].2, Some(2.6));
        assert_eq!(rows[0].3, Some(3.0));
        assert_eq!(rows[0].4, "range");
        assert_eq!(rows[0].5, Some(0.5));
        assert_eq!(rows[0].6, "EUR/kg");
        assert_eq!(rows[1].0, "Blei / z.B. Bleigeschirr, Bleistangen");
        assert!((rows[1].1 - 0.7).abs() < 1e-9);
        // One-sided cells get their own kind, never a bare bound.
        // Tuple: (kind, min, max).
        assert_eq!(
            parse_span("bis zu 1,80 €/kg").map(|r| (r.3, r.1, r.2)),
            Some(("upto", None, Some(1.8)))
        );
        assert_eq!(
            parse_span("ab 0,40 €/kg").map(|r| (r.3, r.1, r.2)),
            Some(("approx", Some(0.4), None))
        );
        assert_eq!(
            parse_span("4,20 €/kg").map(|r| (r.3, r.1, r.2)),
            Some(("exact", None, None))
        );
        assert_eq!(unit_of("2,60 – 3,00 €/kg"), Some("EUR/kg"));
        assert_eq!(unit_of("5 € pro Stück"), None);
        // Anchors gone: loud error, not silent success.
        assert!(parse("<div>Redesign ohne Karten</div>").is_err());
    }

    #[test]
    fn mapping_covers_all_live_labels() {
        assert_eq!(
            grade_for("Kupferkabel / Haushaltskabel"),
            Some(("kabel-kupfer", "Kupferkabel"))
        );
        assert_eq!(
            grade_for("Schälkabel / mit dickem Kupferdraht"),
            Some(("kabel-kupfer", "Schälkabel"))
        );
        assert_eq!(
            grade_for("Kupferdraht / z.B. Milberry"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Kupferschrott schwer / z.B. Rohre"),
            Some(("kupfer-gemischt", "schwer"))
        );
        assert_eq!(
            grade_for("Kupferschrott leicht / mit leichten Anhaftungen oder lackiert"),
            Some(("kupfer-gemischt", "leicht"))
        );
        assert_eq!(
            grade_for("Messing schwer / saubere Armaturen"),
            Some(("messing", ""))
        );
        assert_eq!(
            grade_for("Blei / z.B. Bleigeschirr, Bleistangen"),
            Some(("blei", ""))
        );
        assert_eq!(
            grade_for("Elektromotoren / mit und ohne Anhaftung"),
            Some(("elektromotoren", ""))
        );
        assert_eq!(
            grade_for("Eisenschrotte und Stahlschrotte / verschiedene Sorten"),
            Some(("mischschrott", ""))
        );
        assert_eq!(
            grade_for("Weitere Sorten / Silber, Gold, Hartmetalle"),
            None
        );
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h1>Impressum</h1>\
            <p class=\"gb-text gb-text-fb36f3fe\">Christian Jungk<br>\
            MS Recycling Frankfurt e.K. Inh. Christian Jungk<br>Eichenstraße 25<br>\
            65933 Frankfurt/Main (Stadtteil Griesheim)</p>\
            <h2 class=\"wp-block-heading\">Kontakt</h2>\
            <p class=\"wp-block-paragraph\">Telefon: 069 38280203<br>Telefax: 069 3904542<br>\
            E-Mail: info@msr-frankfurt.de</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Eichenstraße 25");
        assert_eq!(info.postcode, "65933");
        assert_eq!(info.city, "Frankfurt/Main");
        assert_eq!(info.phone, "069 38280203");
        assert_eq!(info.email, "info@msr-frankfurt.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<p>Neu hier</p>").is_err());
    }
}
