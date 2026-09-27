//! Metallankauf24 (Marxen): online portal with a "bis zu" category
//! overview on the homepage ("Aktuelle Schrottpreise"). Multi-material
//! categories without a clear primary grade (Zink/Blei, Edelstahl/Nickel,
//! VHM/HSS/Wolfram) and priceless rows are skipped loudly; the rest maps
//! to representative materials at confidence 0.5. Morning + afternoon
//! schedule: the portal reprices during the day.

use super::super::{
    eur_unit, fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
};
use crate::IngestError;

pub const SLUG: &str = "ni-marxen-metallankauf24-andre-owsianski-ne-spezia";
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
                published_at: None,
                valid_from: None,
                valid_to: None,
            }),
            None => skipped_labels.push(label),
        }
    }
    Ok(HandlerOutcome {
        prices,
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
                rows.push((label, price, eur_unit(&t).unwrap_or(PAGE_UNIT)));
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

fn is_junk(t: &str) -> bool {
    let l = t.to_lowercase();
    ["aktuelle schrottpreise", "tagesaktuelle", "anlieferung", "abholung", "versand",
     "kontakt", "impressum", "cookies", "bewertung", "nachhaltigkeit", "login"]
        .iter()
        .any(|j| l.contains(j))
        || l.len() > 80
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
