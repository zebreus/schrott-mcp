//! Kupferhelden (Hattersheim): cable/copper specialist with "bis zu"
//! (up-to) Tagespreise in Elementor cards — no table, no date. The upper
//! bound is honest data for our uncertainty model: price = price_max =
//! advertised value, confidence 0.5. All four grades map to `kabel-kupfer`
//! with the raw grade in the label.

use scraper::{Html, Selector};

use super::super::{
    eur_unit, fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "he-hattersheim-kupferhelden";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://kupferhelden.de/impressum/";

pub const URL: &str = "https://kupferhelden.de/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
    // Exact list prices (no "bis zu") are first-class rows at full
    // confidence; "bis zu" rows carry the bound as price_max at 0.5.
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit, upto) in rows {
        let (price_max, confidence, price_kind) = if upto {
            (Some(price), Some(0.5), "upto")
        } else {
            (None, Some(1.0), "exact")
        };
        prices.push(ScrapedPrice {
            material: "kabel-kupfer",
            variant: grade_variant(&label),
            price,
            currency: "EUR",
            unit,
            price_kind,
            price_min: None,
            price_max,
            confidence,
            label,
            published_at: None,
            valid_from: None,
            valid_to: None,
        });
    }
    // Grades the variant extractor does not know land here, not in the DB.
    prices.retain(|p| {
        if p.variant.is_empty() && p.label.len() > 4 {
            skipped_labels.push(format!("{} (Sorte unverständlich)", p.label));
            false
        } else {
            true
        }
    });
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

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str, bool)>, Vec<String>), IngestError> {
    // Window: price cards live between TAGESPREISE and the footer links.
    let start = html.find("TAGESPREISE").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "TAGESPREISE fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail
        .find("Quick Links")
        .or_else(|| tail.find("Impressum"))
        .unwrap_or(tail.len());
    let window = &tail[..end];
    // Text-node walk; an € price closes the pair with the previous text
    // as the grade label. Returns (label, price, unit, upto).
    let mut texts = Vec::new();
    let mut in_tag = false;
    let mut cur = String::new();
    for c in window.chars() {
        if c == '<' {
            if !cur.trim().is_empty() {
                texts.push(cur.trim().to_owned());
            }
            cur.clear();
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            cur.push(c);
        }
    }
    let mut rows = Vec::new();
    let mut unit_skips = Vec::new();
    let mut pending: Option<String> = None;
    for t in texts {
        let t = t.replace("&nbsp;", " ").replace(['\u{a0}'], " ");
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if t.is_empty() {
            continue;
        }
        if t.contains('€') && parse_eur(&t).is_some() {
            if let Some(label) = pending.take() {
                let Some(unit) = eur_unit(&t) else {
                    unit_skips.push(format!("{label} (Einheit unverständlich: {t})"));
                    continue;
                };
                rows.push((label, parse_eur(&t).expect("checked"), unit, is_upto(&t)));
            }
        } else if is_junk(&t) {
            pending = None;
        } else {
            pending = Some(match pending {
                Some(p) => format!("{p} / {t}"),
                None => t,
            });
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "keine Preispaare".to_owned() });
    }
    Ok((rows, unit_skips))
}

/// "bis zu" (and variants) mark upper bounds; anything else with € and
/// digits is an exact list price.
fn is_upto(t: &str) -> bool {
    let l = t.to_lowercase();
    l.contains("bis zu") || l.contains("biszu") || l.contains("max.")
}

fn grade_variant(label: &str) -> &'static str {
    let l = label.to_lowercase();
    if l.contains("37%") {
        "bis 37%"
    } else if l.contains("38%") {
        "min 38%"
    } else if l.contains("60%") {
        "min 60%"
    } else if l.contains("70%") {
        "min 70%"
    } else {
        ""
    }
}

fn is_junk(t: &str) -> bool {
    let l = t.to_lowercase();
    ["willkommen", "kontakt", "impressum", "datenschutz", "tagespreise"]
        .iter()
        .any(|j| l.contains(j))
        || l.len() > 120
}


/// Bespoke contact extraction for THIS impressum only: the address `<p>`
/// (firm lines + street + PLZ city) and the `<p>` after the "Kontakt"
/// heading ("Telefon:" / "E-Mail:" lines). Missing anchors → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p = Selector::parse("p").expect("valid selector");
    let h2 = Selector::parse("h2").expect("valid selector");
    // Address block: the <p> holding a PLZ + city line.
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    for el in doc.select(&p) {
        let lines: Vec<String> = el
            .inner_html()
            .split("<br")
            .map(|s| strip_fragment(s))
            .filter(|s| !s.is_empty())
            .collect();
        for (k, line) in lines.iter().enumerate() {
            let mut it = line.split_whitespace();
            // PLZ city [am Main …]: first token 5 digits, second capitalized.
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
        if !postcode.is_empty() {
            break;
        }
    }
    // "Kontakt" heading → next <p> holds the labeled lines.
    let mut phone = String::new();
    let mut email = String::new();
    let mut found_kontakt = false;
    for el in doc.select(&h2) {
        if el.text().collect::<String>().trim() == "Kontakt" {
            found_kontakt = true;
            let mut sib = el.next_siblings();
            let txt = sib
                .find_map(|n| scraper::ElementRef::wrap(n).filter(|e| e.value().name() == "p"))
                .map(|p| p.inner_html());
            if let Some(html) = txt {
                for part in html.split("<br") {
                    let t = strip_fragment(part);
                    if let Some(v) = t.strip_prefix("Telefon:") {
                        phone = v.trim().to_owned();
                    } else if let Some(v) = t.strip_prefix("E-Mail:") {
                        email = v.trim().to_owned();
                    }
                }
            }
        }
    }
    if !found_kontakt || (street.is_empty() && phone.is_empty() && email.is_empty()) {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    }
    Ok(TraderInfo { street, postcode, city, phone, email })
}

/// Strip tags from a fragment (html5ever already decoded entities).
/// Fragments from splitting on "<br" start with a tag remnant
/// (` class="…"`) — drop everything up to the first '>' first, or the
/// attributes parse as text.
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
    use super::{grade_variant, parse};

    const FIXTURE: &str = "<h2>TAGESPREISE</h2>\
        <h4>Kabel mit Stecker, bis 37%</h4><div>bis zu 0,45 €/KG</div>\
        <h4>Kupfer ohne Stecker, min 38%,cu</h4><div>bis zu* 1,80 €/KG</div>\
        <h4>Alukabel sortiert</h4><div>bis zu 0,90 €/KG</div>\
        <h4>Messing Armaturen</h4><div>4,20 €/KG</div>\
        <footer>Quick Links</footer>";

    #[test]
    fn impressum_blocks() {
        let imp = "<p>Tayfun Karaca<br>Kupferhelden Hattersheim<br>Im Boden 23<br>            65795 Hattersheim am Main</p><h2>Kontakt</h2>            <p>Telefon: 01623060230<br>E-Mail: info@kupferhelden.de</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Im Boden 23");
        assert_eq!(info.postcode, "65795");
        assert_eq!(info.city, "Hattersheim");
        assert_eq!(info.phone, "01623060230");
        assert_eq!(info.email, "info@kupferhelden.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }

    #[test]
    fn bis_zu_pairs() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 4);
        assert!(skips.is_empty());
        assert_eq!(rows[0].0, "Kabel mit Stecker, bis 37%");
        assert_eq!(rows[0].1, 0.45);
        assert!(rows[0].3, "bis zu flag");
        assert_eq!(rows[1].0, "Kupfer ohne Stecker, min 38%,cu");
        assert_eq!(rows[1].1, 1.8);
        assert!(!rows[3].3, "exact price has no upto flag");
        assert_eq!(rows[3].1, 4.2);
        // Unknown grades never reach the DB as copper cable.
        assert_eq!(grade_variant("Alukabel sortiert"), "");
        assert_eq!(grade_variant("Kupferkabel, min 60%,cu"), "min 60%");
    }
}
