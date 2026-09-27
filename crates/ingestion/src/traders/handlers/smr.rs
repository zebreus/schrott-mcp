//! SMR GmbH (Spreewald Metall Recycling, Lübben-Neuendorf): exact prices
//! in the htmx-loaded price list behind "SMR Ankaufspreise" on
//! /smr-preise/ (`hx-get="…/ssp/get/PriceList/?f=html&t=p"`). The table
//! carries Nr/Name/stk. Preis/Einheit columns, all rows live in "kg".
//! Altpapier has no catalog material and is skipped loudly. No page date
//! (the "Datum ab/bis" fields belong to the Schrottrechner form), so
//! `published_at` stays `None`.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bb-lubben-neuendorf-smr";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://smr-luebben.de/impressum/";

pub const URL: &str = "https://smr-luebben.de/smr-preise/";
/// Marker of the htmx price embed on URL. The endpoint URL itself is
/// extracted live from the page's `hx-get` (tunnel hostnames rot) —
/// a hardcoded copy would silently go stale.

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, page) = fetch_text(client, URL).await?;
    let price_url = price_endpoint(&page)?;
    let (_, list_html) = fetch_text(client, price_url).await?;
    let (rows, mut skipped_labels) = parse(&list_html)?;
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
        byte_len: page.len(),
        published_at: None,
    })
}

/// Extract the live htmx price endpoint from the page's `hx-get`.
/// The tunnel hostname rots — only a live-embedded URL is ever fetched.
/// Anything else (moved embed, foreign host) fails loudly.
fn price_endpoint(page: &str) -> Result<&str, IngestError> {
    let attr = page.find("hx-get=\"").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preislisten-Einbettung fehlt".to_owned(),
    })?;
    let rest = &page[attr + "hx-get=\"".len()..];
    let end = rest.find('"').ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preislisten-Einbettung unvollständig".to_owned(),
    })?;
    let url = &rest[..end];
    if !url.contains("PriceList/") || !url.starts_with("https://") {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preislisten-Einbettung fremd".to_owned(),
        });
    }
    Ok(url)
}

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific-before-generic: "Millberry" must win over "Berry".
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("berry") {
        Some(("kupfer-berry", ""))
    } else if l.contains("raff") {
        Some(("kupfer-gemischt", "Raff 95% Cu"))
    } else if l.contains("späne") || l.contains("spaene") {
        Some(("kupfer-gemischt", "Späne"))
    } else if l.contains("kabel") && l.contains("stecker") {
        Some(("kabel-kupfer", "mit Stecker"))
    } else if l.contains("kabel") {
        Some(("kabel-kupfer", "Basis 50% ohne Anhaftung"))
    } else if l.contains("motor") {
        Some(("elektromotoren", ""))
    } else if l.contains("milbe") || l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("dosen") || l.contains("assietten") {
        Some(("aluminium-blech", "Dosen, Assietten"))
    } else if l.contains("blech") {
        if l.contains("5%") {
            Some(("aluminium-blech", "max. 5% Anhaftung"))
        } else {
            Some(("aluminium-blech", "ohne Anhaftung"))
        }
    } else if l.contains("guss") {
        if l.contains("5%") {
            Some(("aluminium-guss", "max. 5% Anhaftung"))
        } else {
            Some(("aluminium-guss", "ohne Anhaftung"))
        }
    } else if l.contains("edelstahl") && l.contains("50x50x150") {
        Some(("edelstahl-v2a", "50x50x150cm"))
    } else if l.contains("edelstahl") {
        // "Edelstahl großstückig" carries no V2A marking — generic, never
        // the specific grade.
        Some(("edelstahl-gemischt", "großstückig"))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", "Leicht"))
    } else if l.contains("stahlschrott") || l.contains("s3") {
        Some(("stahlschrott-scheren", "S3 schwer"))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the block after
/// "Diese Website wird betrieben von" holds firm lines + street + PLZ
/// city as `<br>`-separated lines, plus "Tel:" and an obfuscated
/// "Mail: info(at)smr-luebben.de". Missing anchor → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let start = imp
        .find("Diese Website wird betrieben von")
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Betreiber-Block fehlt".to_owned(),
        })?;
    let tail = &imp[start..];
    let end = tail
        .find("Verantwortlich für den Inhalt")
        .unwrap_or(tail.len());
    let window = &tail[..end];
    let mut lines = Vec::new();
    for part in window.split("<br") {
        let t = strip_fragment(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| a + " " + b);
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                break;
            }
        }
    }
    let mut phone = String::new();
    let mut email = String::new();
    for line in &lines {
        if phone.is_empty() {
            if let Some(rest) = line.strip_prefix("Tel:") {
                phone = rest.trim().to_owned();
            }
        }
        if email.is_empty() {
            if let Some(rest) = line.strip_prefix("Mail:") {
                email = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .replace("(at)", "@");
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

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant (` class="…"`) — drop everything up to the first '>' first,
/// or the attributes parse as text.
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
    let doc = Html::parse_document(html);
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tbody tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
    // Never trust page order: take the table carrying the material AND
    // price headers ("Name" + "Preis"), not just the first <table>.
    let table = doc.select(&table).find(|t| {
        let heads: Vec<String> = t
            .select(&head)
            .map(|h| h.text().collect::<String>().to_lowercase())
            .collect();
        heads.iter().any(|h| h.contains("name")) && heads.iter().any(|h| h.contains("preis"))
    });
    let Some(table) = table else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    };
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in table.select(&row) {
        let cells: Vec<ElementRef> = tr.select(&cell).collect();
        if cells.len() < 4 {
            continue;
        }
        let label = cells[1]
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if label.is_empty() || label.len() > 120 {
            continue;
        }
        let price_text: String = cells[2].text().collect();
        let Some(price) = parse_eur(&price_text) else {
            skips.push(format!(
                "{label} (Preis unverständlich: {})",
                price_text.trim()
            ));
            continue;
        };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let unit_text: String = cells[3].text().collect();
        let Some(unit) = unit_of(&unit_text) else {
            skips.push(format!(
                "{label} (Einheit unverständlich: {})",
                unit_text.trim()
            ));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS list's Einheit column (live: "kg").
/// Only kg/t exist here — anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|t| t == "t")
        || lower.contains("tonne")
        || lower.contains(" to ")
    {
        Some("EUR/t")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    const PAGE: &str = "<div hx-get=\"https://wafery-mole-3614.dataplicity.io/ssp/get/PriceList/?f=html&t=p\" hx-swap=\"innerHTML\" hx-trigger=\"load\">Unsere aktuellen Preise werden abgerufen ...</div>";

    const LIST: &str = "<table><thead><tr><th>Nr</th><th>Name</th>\
        <th>stk. Preis</th><th>Einheit</th></tr></thead><tbody>\
        <tr><td>200</td><td>Cu- Millberry, blank über 1,2mm</td><td>10,62</td><td>kg</td></tr>\
        <tr><td>136</td><td>Al- Blech, max. 5% Anhaftung</td><td>1,31</td><td>kg</td></tr>\
        <tr><td>360</td><td>Ms- Milbe</td><td>5,78</td><td>kg</td></tr>\
        <tr><td>1</td><td>Altpapier</td><td>0,05</td><td>kg</td></tr>\
        <tr><td>710</td><td>Edelstahl großstückig</td><td>0,74</td><td>kg</td></tr>\
        </tbody></table>";

    #[test]
    fn embed_extracts_live_endpoint() {
        let url = super::price_endpoint(PAGE).expect("embeds");
        assert!(url.contains("PriceList/"), "{url}");
        assert!(super::price_endpoint("<div>Kein Embed</div>").is_err());
        assert!(super::price_endpoint("<div hx-get=\"http://fremd/x\">x</div>").is_err());
    }

    #[test]
    fn table_parses() {
        assert!(PAGE.contains("PriceList/"));
        let (rows, skips) = parse(LIST).expect("parses");
        assert_eq!(rows.len(), 5);
        assert!(skips.is_empty());
        assert_eq!(rows[0].0, "Cu- Millberry, blank über 1,2mm");
        assert_eq!(rows[0].1, 10.62);
        assert_eq!(rows[0].2, "EUR/kg");
    }

    #[test]
    fn wrong_table_and_unit_are_rejected_loudly() {
        // A layout table before the price table must not win.
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + LIST;
        let (rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 5);
        // Unknown unit: skipped loudly, valid rows survive.
        let html = LIST.replacen("<td>kg</td>", "<td>pro Sack</td>", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 4);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Millberry"));
        // Every row unparseable: loud error, not silent success.
        let html = LIST.replace("<td>kg</td>", "<td>pro Sack</td>");
        let err = parse(&html).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "Diese Website wird betrieben von</p><h4>SMR GmbH</h4>\n\
            Spreewald Metall Recycling<br />Mühlbergweg 10<br />15907 Lübben-Neuendorf<br />\
            Deutschland<br /><br />Tel: +49 3546 219 98 81<br />\
            Mail: <a href=\"/kontakt/#kontakt\">info(at)smr-luebben.de</a><br />\
            <p>Verantwortlich für den Inhalt</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Mühlbergweg 10");
        assert_eq!(info.postcode, "15907");
        assert_eq!(info.city, "Lübben-Neuendorf");
        assert_eq!(info.phone, "+49 3546 219 98 81");
        assert_eq!(info.email, "info@smr-luebben.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }

    #[test]
    fn mapping_splits_grades_and_skips_paper() {
        assert_eq!(
            grade_for("Cu- Millberry, blank über 1,2mm"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Cu- Berry, Draht mit Anhaftung"),
            Some(("kupfer-berry", ""))
        );
        assert_eq!(
            grade_for("Cu- Raff 95% Cu"),
            Some(("kupfer-gemischt", "Raff 95% Cu"))
        );
        assert_eq!(grade_for("Ms- Milbe"), Some(("messing", "")));
        assert_eq!(
            grade_for("Al- Blech, max. 5% Anhaftung"),
            Some(("aluminium-blech", "max. 5% Anhaftung"))
        );
        assert_eq!(
            grade_for("Edelstahl großstückig"),
            Some(("edelstahl-gemischt", "großstückig"))
        );
        assert_eq!(
            grade_for("Edelstahl V2A, 50x50x150cm"),
            Some(("edelstahl-v2a", "50x50x150cm"))
        );
        assert_eq!(grade_for("Altpapier"), None, "paper has no metal material");
    }
}
