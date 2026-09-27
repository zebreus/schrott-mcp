//! DSH Duisburger Schrotthandel (Duisburg-Beeck): acceptance list
//! without prices (the four "Category: items" `<li>` rows under
//! "Welche Schrottarten kaufen wir an?"). Vergütung is by "aktuellen
//! Marktpreisen" via individual offers only, so this handler only
//! fills `trader_materials` plus contact enrichment. Zero prices with
//! resolved acceptances is normal operation.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "nw-duisburg-dsh-duisburger-schrotthandel";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://dsh-schrotthandel.de/impressum";

pub const URL: &str = "https://dsh-schrotthandel.de/schrottankauf";

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
    let labels = parse(&html)?;
    let mut acceptances = Vec::with_capacity(labels.len());
    let mut skipped_labels = Vec::new();
    for label in labels {
        match grade_for(&label) {
            Some(materials) => {
                for (material, conditions) in materials {
                    acceptances.push(ScrapedAcceptance {
                        material,
                        conditions: conditions.to_owned(),
                        label: label.clone(),
                    });
                }
            }
            None => skipped_labels.push(label),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        prices: vec![],
        acceptances,
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

/// Explicit label → acceptances. Bare "Eisen"/"Stahl" land on the
/// generic iron bucket (esh precedent: "Eisenschrott" →
/// `mischschrott`). Category words ("Altmetall", "Elektronikschrott",
/// …) skip — their items are ingested individually. Bare "Kabel" is
/// Cu-vs-Alu unattributable (quell only maps qualified Kabel);
/// "Motoren" in vehicle-scrap context may be combustion engines, not
/// `elektromotoren`; "Rohre", "Gerüste", "Maschinenteile",
/// "Autoteile", "Batterien", "Computer­teile", "Haushaltsgeräte" name
/// no catalog material — all skipped loudly, never guessed.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("aluminium") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("eisen") || l.contains("stahl") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("blei") {
        Some(vec![("blei", "")])
    } else {
        None
    }
}

fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    // "Category: item, item, …" rows between the offer heading and the
    // "Warum sich Schrottankauf …" sales section. Both anchors are
    // mandatory; rows without the bespoke colon shape fail loudly.
    let start = html
        .find("Welche Schrottarten kaufen wir an?")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Warum sich Schrottankauf")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeblock-Ende fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    let li = Selector::parse("li").expect("valid selector");
    let mut labels = Vec::new();
    for el in doc.select(&li) {
        let row: String = el.text().collect();
        let row = row.split_whitespace().collect::<Vec<_>>().join(" ");
        let (category, items) = row.split_once(':').ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: format!("Annahmezeile ohne Kategorie-Doppelpunkt: {row}"),
        })?;
        let category = category.trim().to_owned();
        if !category.is_empty() {
            labels.push(category);
        }
        for item in items.split(',') {
            let t = item.split_whitespace().collect::<Vec<_>>().join(" ");
            if !t.is_empty() {
                labels.push(t);
            }
        }
    }
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    Ok(labels)
}

/// Bespoke contact extraction for THIS impressum only: the
/// `<p><strong>DSH Schrotthandel</strong><br />Am Nienhaushof
/// 29<br />47139 Duisburg…` address block plus the
/// `<p><strong>Kontakt:</strong><br />Telefon: [0203 …]<br />E-Mail:
/// [info@…]` row (brackets are literal page content and get
/// stripped). Anchored on "Vertreten durch:" — without it the page
/// changed shape → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let all = doc
        .root_element()
        .text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if !all.contains("Vertreten durch:") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Vertreten-durch-Block fehlt".to_owned(),
        });
    }
    // Address lines from the firm <p> ("DSH Schrotthandel" / "Am
    // Nienhaushof 29" / "47139 Duisburg"): scraper text() glues adjacent
    // nodes ("29"+"47139"), so split the <p> on <br> into real lines
    // instead of tokenizing the glued whole-page text.
    let p_sel = Selector::parse("p").expect("valid selector");
    let firm_p = doc
        .select(&p_sel)
        .find(|p| p.text().collect::<String>().contains("DSH Schrotthandel"));
    let Some(firm_p) = firm_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let lines: Vec<String> = firm_p
        .inner_html()
        .split("<br")
        .map(|s| strip_fragment(s))
        .filter(|s| !s.is_empty())
        .collect();
    // Street: the line before the PLZ line; postcode + city from it.
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5
                && pc.chars().all(|c| c.is_ascii_digit())
                && ci.chars().next().is_some_and(|c| c.is_uppercase())
            {
                postcode = pc.to_owned();
                city = ci.trim_matches(|c| c == '[' || c == ']').to_owned();
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                break;
            }
        }
    }
    let strip_brackets = |s: &str| {
        s.trim()
            .trim_matches(|c| c == '[' || c == ']')
            .trim()
            .to_owned()
    };
    let phone = all
        .split_once("Telefon:")
        .map(|(_, r)| {
            let end = r.find("E-Mail:").unwrap_or(r.len());
            strip_brackets(&r[..end])
        })
        .unwrap_or_default();
    // Email: glued block boundaries ("…de]Umsatzsteuer-ID:…") defeat
    // token splitting — expand from the '@' over email characters.
    let email = all
        .split_once("E-Mail:")
        .map(|(_, r)| email_token(r))
        .unwrap_or_default();
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

/// First email address in the text after a label: expand from the '@'
/// over email characters (glued block boundaries defeat tokenizing).
fn email_token(r: &str) -> String {
    let Some(at) = r.find('@') else {
        return String::new();
    };
    let b = r.as_bytes();
    let is_email = |c: u8| c.is_ascii_alphanumeric() || b".-_+@".contains(&c);
    let mut s = at;
    while s > 0 && is_email(b[s - 1]) {
        s -= 1;
    }
    let mut e = at + 1;
    while e < b.len() && is_email(b[e]) {
        e += 1;
    }
    let cand = &r[s..e];
    // Glued trailing prose ("…deGeschäftsführer:") survives the
    // expansion (all alphanumeric) — cut at the end of the domain.
    for suffix in [".de", ".com", ".net", ".org", ".eu", ".info", ".biz"] {
        if let Some(p) = cand.rfind(suffix) {
            let cut = cand[..p + suffix.len()]
                .trim_matches(|c| c == '[' || c == ']')
                .to_owned();
            if cut.contains('@') && !cut.starts_with('@') {
                return cut;
            }
        }
    }
    String::new()
}

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant (` />`, `class="…"`) — drop everything up to the first '>'
/// first, or the attributes parse as text.
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

    const FIXTURE: &str = "<h2>Welche Schrottarten kaufen wir an?</h2>\
        <p>DSH Schrotthandel kauft eine Vielzahl von Schrottarten an. Egal, ob es sich um kleine Mengen oder große Chargen handelt, wir nehmen:</p>\
        <ul><li><strong>Altmetall</strong>: Kupfer, Aluminium, Stahl, Eisen, Blei</li>\
        <li><strong>Elektronikschrott</strong>: Kabel, Computerteile, Haushaltsgeräte</li>\
        <li><strong>Fahrzeugschrott</strong>: Autoteile, Batterien, Motoren</li>\
        <li><strong>Baustellenschrott</strong>: Rohre, Gerüste, Maschinenteile</li></ul>\
        <h2>Warum sich Schrottankauf bei DSH Schrotthandel lohnt</h2>";

    #[test]
    fn colon_rows_split_into_items() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 18);
        assert_eq!(
            &labels[..6],
            &["Altmetall", "Kupfer", "Aluminium", "Stahl", "Eisen", "Blei"]
        );
        assert_eq!(
            &labels[6..10],
            &[
                "Elektronikschrott",
                "Kabel",
                "Computerteile",
                "Haushaltsgeräte"
            ]
        );
        assert_eq!(
            &labels[10..14],
            &["Fahrzeugschrott", "Autoteile", "Batterien", "Motoren"]
        );
        // Anchors are mandatory: redesign fails loudly.
        assert!(parse("<p>Neu hier</p>").is_err());
        assert!(parse(
            "<h2>Welche Schrottarten kaufen wir an?</h2><ul><li>Kein Doppelpunkt</li></ul>\
            <h2>Warum sich Schrottankauf lohnt</h2>"
        )
        .is_err());
    }

    #[test]
    fn impressum_bracketed_contact() {
        let imp = "<h2>Impressum</h2>\
            <p><strong>DSH Schrotthandel</strong><br />Am Nienhaushof 29<br />47139 Duisburg<br />Deutschland</p>\
            <p><strong>Vertreten durch:</strong><br />Geschäftsführer: [Ali El baba]</p>\
            <p><strong>Kontakt:</strong><br />Telefon: [0203 57846500]<br />E-Mail: [info@dsh-schrotthandel.de]</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Am Nienhaushof 29");
        assert_eq!(info.postcode, "47139");
        assert_eq!(info.city, "Duisburg");
        assert_eq!(info.phone, "0203 57846500");
        assert_eq!(info.email, "info@dsh-schrotthandel.de");
        assert!(super::extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }

    #[test]
    fn mapping_maps_plain_and_skips_ambiguous() {
        assert_eq!(grade_for("Kupfer"), Some(vec![("kupfer-gemischt", "")]));
        assert_eq!(
            grade_for("Aluminium"),
            Some(vec![("aluminium-gemischt", "")])
        );
        assert_eq!(grade_for("Stahl"), Some(vec![("mischschrott", "")]));
        assert_eq!(grade_for("Eisen"), Some(vec![("mischschrott", "")]));
        assert_eq!(grade_for("Blei"), Some(vec![("blei", "")]));
        for skip in [
            "Altmetall",
            "Elektronikschrott",
            "Fahrzeugschrott",
            "Baustellenschrott",
            "Kabel",
            "Computerteile",
            "Haushaltsgeräte",
            "Autoteile",
            "Batterien",
            "Motoren",
            "Rohre",
            "Gerüste",
            "Maschinenteile",
        ] {
            assert_eq!(grade_for(skip), None, "{skip} must skip loudly");
        }
    }
}
