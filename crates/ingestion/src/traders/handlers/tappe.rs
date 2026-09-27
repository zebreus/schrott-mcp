//! Tappe Rohstoffhandel (Essen): price block embedded in the homepage,
//! with the price date next to the heading ("Aktuelle Schrottpreise /
//! 24.09.2026"). Labels and prices arrive as alternating text nodes;
//! multi-line labels ("Kupferschrott 1" + "ECU/Milb.") accumulate until
//! a price node closes the pair. Daily schedule: they refresh mornings.

use super::super::{
    eur_unit, fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule,
    ScrapedPrice,
};
use crate::IngestError;

pub const SLUG: &str = "nw-essen-vogelheim-tappe-rohstoffhandel";
pub const URL: &str = "https://www.tappe-recycling.de/";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        schedule: Schedule::DailyAt { times: vec![(7, 30)] },
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (published_at, rows) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    let mut skipped_labels = Vec::new();
    for (label, price, unit) in rows {
        match material_for(&label) {
            Some(material) => prices.push(ScrapedPrice {
                material,
                price,
                currency: "EUR",
                unit,
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label,
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
        published_at,
    })
}

/// Explicit mapping; generic page labels land on the closest grade and
/// keep the raw label in `notes` for traceability.
fn material_for(label: &str) -> Option<&'static str> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("mischschrott") {
        Some("mischschrott")
    } else if l.contains("schredder") || l.contains("shredder") {
        Some("stahlschrott-scheren")
    } else if l.contains("kabel") {
        Some("kabel-kupfer")
    } else if l.contains("kupferschrott 1") || l.contains("ecu") || l.contains("milb") {
        Some("kupfer-millberry")
    } else if l.contains("kupfer") {
        Some("kupfer-berry")
    } else if l.contains("messing") {
        Some("messing")
    } else if l.contains("alu") {
        Some("aluminium-profile")
    } else if l.contains("blei") {
        Some("blei")
    } else if l.contains("edelstahl") || l.contains("va ") || l == "va" {
        Some("edelstahl-v2a")
    } else if l.contains("zink") {
        Some("zink")
    } else {
        None
    }
}

fn parse(html: &str) -> Result<(Option<String>, Vec<(String, f64, &'static str)>), IngestError> {
    // Only the price box: from its heading to the contact link run-out.
    let start = html.find("Aktuelle Schrottpreise").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisbox fehlt".to_owned(),
    })?;
    let window = &html[start..(start + html[start..].len().min(12_000))];
    let published_at = date_in(window);
    // Collect <p> texts in order.
    let mut texts = Vec::new();
    let mut rest = window;
    while let Some(a) = rest.find("<p") {
        let after = &rest[a..];
        let Some(b) = after.find('>') else { break };
        let after = &after[b + 1..];
        let Some(c) = after.find("</p>") else { break };
        let mut t = after[..c].to_owned();
        // strip nested tags, decode the entities we care about
        loop {
            let Some(x) = t.find('<') else { break };
            let Some(y) = t[x..].find('>') else { break };
            t.replace_range(x..x + y + 1, " ");
        }
        let t = t
            .replace("&nbsp;", " ")
            .replace("&#160;", " ")
            .replace(['\u{a0}'], " ");
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if !t.is_empty() {
            texts.push(t);
        }
        rest = &after[c + 4..];
    }
    let mut rows = Vec::new();
    let mut pending: Vec<String> = Vec::new();
    for t in texts {
        if is_price(&t) {
            if let Some(price) = parse_eur(&t) {
                let label = pending.join(" ").trim().to_owned();
                pending.clear();
                if !label.is_empty() {
                    rows.push((label, price, eur_unit(&t).unwrap_or("EUR/kg")));
                }
            }
        } else if is_header(&t) {
            pending.clear();
        } else {
            pending.push(t);
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "keine Preispaare".to_owned() });
    }
    Ok((published_at, rows))
}

fn is_price(t: &str) -> bool {
    t.contains('€') && parse_eur(t).is_some()
}

fn is_header(t: &str) -> bool {
    matches!(t, "Schrottsorte" | "€ pro kg" | "Aktuelle Schrottpreise")
        || t.contains("weitere")
        || t.contains("Anfrage")
        || t.len() > 120
}

fn date_in(window: &str) -> Option<String> {
    let bytes = window.as_bytes();
    let mut i = 0;
    while i + 10 <= bytes.len() {
        if bytes[i].is_ascii_digit()
            && bytes[i + 2] == b'.'
            && bytes[i + 5] == b'.'
            && bytes[i + 6..].iter().take(4).all(|c| c.is_ascii_digit())
        {
            let (d, m, y) = (&window[i..i + 2], &window[i + 3..i + 5], &window[i + 6..i + 10]);
            if let Some(rfc) = parse_de_date(d, m, y) {
                return Some(rfc);
            }
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{date_in, material_for, parse};

    const FIXTURE: &str = "<p><strong>Aktuelle Schrottpreise </strong></p><p>24.09.2026</p>\
        <p>Schrottsorte</p><p>€ pro kg</p>\
        <p>Mischschrott</p><p>0,170 €</p>\
        <p>Kupferschrott 1</p><p>ECU/Milb.</p><p>11,20 €</p>\
        <p>... weitere auf Anfrage</p>";

    #[test]
    fn pairs_date_and_mapping() {
        let (published_at, rows) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-24T00:00:00+00:00"));
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "Mischschrott");
        assert_eq!(rows[0].1, 0.17);
        assert_eq!(rows[1].0, "Kupferschrott 1 ECU/Milb.");
        assert_eq!(rows[1].1, 11.2);
        assert_eq!(material_for("Mischschrott"), Some("mischschrott"));
        assert_eq!(material_for("Kupferschrott 1 ECU/Milb."), Some("kupfer-millberry"));
        assert_eq!(material_for("Kabelschrott (Basis 40% Kupfer)"), Some("kabel-kupfer"));
    }

    #[test]
    fn date_scan() {
        assert_eq!(
            date_in("Preise 24.09.2026 Liste").as_deref(),
            Some("2026-09-24T00:00:00+00:00")
        );
        assert_eq!(date_in("kein Datum"), None);
    }
}
