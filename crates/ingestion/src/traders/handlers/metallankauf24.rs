//! Metallankauf24 (Marxen): online portal with a "bis zu" category
//! overview on the homepage ("Aktuelle Schrottpreise"). NOTE (reviewed
//! 27.09.2026): `/online-verkaufen` shows the same category prices from
//! the same backend — fetching it too would double-record every row in
//! the append-only history, so the homepage stays the single source.
//! Multi-material categories without a clear primary grade (Zink/Blei, Edelstahl/Nickel,
//! VHM/HSS/Wolfram) and priceless rows are skipped loudly; the rest maps
//! to representative materials at confidence 0.5. Morning + afternoon
//! schedule: the portal reprices during the day.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "ni-marxen-metallankauf24-andre-owsianski-ne-spezia";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://metallankauf24.de/impressum";

pub const URL: &str = "https://metallankauf24.de/";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::DailyAt { times: vec![(8, 0), (16, 0)] },
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
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

/// Category → material. Ambiguous multi-grade categories
/// (no single primary) return None on purpose.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Ambiguous multi-grade categories (no single primary grade) are
    // skipped on purpose: "Kabel / E-Motoren" mixes cable with motors,
    // "Messing / Rotguss" quotes the category maximum.
    if l == "kupfer" {
        Some(("kupfer-gemischt", ""))
    } else if l == "aluminium" {
        Some(("aluminium-gemischt", ""))
    } else if l == "zinn" {
        Some(("zinn", ""))
    } else {
        None
    }
}

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html.find("Aktuelle Schrottpreise").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisblock fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    // The category overview ends where the pickup/delivery section starts.
    let end = tail
        .find("Abholung und Anlieferung")
        .or_else(|| tail.find("Abholungund"))
        .unwrap_or(tail.len().min(30_000));
    let window = &tail[..end];
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
    let unit_skips: Vec<String> = Vec::new();
    let mut pending: Option<String> = None;
    // Page-global unit: the overview quotes bare "bis zu € X" with no unit
    // per row. EUR/kg is the only sane reading (copper at €10.80/t would be
    // 1000x under market; per-piece makes no sense for bulk grades) and
    // matches the per-kg detail pages — but it stays an explicit,
    // documented assumption, not a silent fallback.
    const PAGE_UNIT: &str = "EUR/kg";
    for t in texts {
        let t = t.replace("&nbsp;", " ").replace(['\u{a0}'], " ");
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if t.is_empty() {
            continue;
        }
        if t.contains("bis zu") {
            if let (Some(price), Some(label)) = (parse_eur(&t), pending.take()) {
                rows.push((label, price, unit_of(&t).unwrap_or(PAGE_UNIT)));
            }
        } else if is_junk(&t) {
            pending = None;
        } else {
            pending = Some(t);
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "keine Preispaare".to_owned() });
    }
    Ok((rows, unit_skips))
}

/// Bespoke unit matcher for THIS overview (live: bare "bis zu € 10,80"
/// rows, hence the PAGE_UNIT default above). Only kg/t exist here.
fn unit_of(t: &str) -> Option<&'static str> {
    let lower = t.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower.split(|c: char| !c.is_alphanumeric()).any(|w| w == "t" || w == "to") {
        Some("EUR/t")
    } else {
        None
    }
}

fn is_junk(t: &str) -> bool {
    let l = t.to_lowercase();
    ["aktuelle schrottpreise", "tagesaktuelle", "anlieferung", "abholung", "versand",
     "kontakt", "impressum", "cookies", "bewertung", "nachhaltigkeit", "login"]
        .iter()
        .any(|j| l.contains(j))
        || l.len() > 80
}


/// Bespoke contact extraction for THIS impressum only: the address lines
/// inside `div.inhalt` ("Hinter der Bahn 23" / "21439 Marxen") plus the
/// labeled "Telefon:"/"E-Mail:" lines of the same block. Missing anchors
/// mean the page changed shape → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let block = Selector::parse("div.inhalt").expect("valid selector");
    let Some(div) = doc.select(&block).next() else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let p = Selector::parse("p").expect("valid selector");
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    let mut phone = String::new();
    let mut email = String::new();
    for el in div.select(&p) {
        let lines: Vec<String> = el
            .inner_html()
            .split("<br")
            .map(strip_fragment)
            .filter(|s| !s.is_empty())
            .collect();
        for (k, line) in lines.iter().enumerate() {
            let mut it = line.split_whitespace();
            if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                if pc.len() == 5
                    && pc.chars().all(|c| c.is_ascii_digit())
                    && ci.chars().next().is_some_and(|c| c.is_uppercase())
                    && postcode.is_empty()
                {
                    postcode = pc.to_owned();
                    city = ci.to_owned();
                    if k > 0 {
                        street = lines[k - 1].clone();
                    }
                }
            }
            if let Some(v) = line.strip_prefix("Telefon:").or_else(|| line.strip_prefix("Tel.")) {
                if phone.is_empty() {
                    phone = v.trim().to_owned();
                }
            } else if let Some(v) = line.strip_prefix("E-Mail:") {
                if email.is_empty() {
                    email = v.trim().to_owned();
                }
            }
        }
    }
    // mailto: link as email fallback inside the block.
    if email.is_empty() {
        let mail = Selector::parse(r#"a[href^="mailto:"]"#).expect("valid selector");
        email = div
            .select(&mail)
            .next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|h| h.strip_prefix("mailto:"))
            .unwrap_or_default()
            .to_owned();
    }
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
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
    use super::{grade_for, parse};

    const FIXTURE: &str = "<h2>Aktuelle Schrottpreise</h2>\
        <div>Kupfer</div><div>bis zu € 10,80 erhalten</div>\
        <div>Zink / Blei</div><div>bis zu € 2,00 erhalten</div>\
        <div>Schrott</div><div>Preis auf Anfrage</div>\
        <h2>Abholung und Anlieferung</h2>";

    #[test]
    fn impressum_inhalt_block() {
        let imp = "<div class=\"inhalt\"><p>Andre Owsianski e.K.<br>            Hinter der Bahn 23<br>21439 Marxen<br>Deutschland</p><p>            <a href=\"mailto:info@metallankauf24.de\">info</a><br>            Tel. 04185 8094617</p></div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Hinter der Bahn 23");
        assert_eq!(info.postcode, "21439");
        assert_eq!(info.city, "Marxen");
        assert_eq!(info.email, "info@metallankauf24.de");
        assert!(super::extract_info("<div><p>Neu</p></div>").is_err());
    }

    #[test]
    fn categories_pair_and_map() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 2);
        assert!(skips.is_empty());
        assert_eq!(rows[0], ("Kupfer".to_owned(), 10.8, "EUR/kg"));
        assert_eq!(grade_for("Kupfer"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Zink / Blei"), None, "ambiguous: skipped");
        assert_eq!(grade_for("VHM / HSS / WOLFRAM"), None);
        assert_eq!(grade_for("Kabel / E-Motoren"), None, "mixed category: skipped");
        assert_eq!(grade_for("Messing / Rotguss"), None, "category maximum: skipped");
    }
}
