//! Willi Plum u. Sohn (Wegberg): acceptance list without prices
//! (the "Wir nehmen unter anderem …" prose paragraph with
//! `<strong>` grades plus the "wie Kupfer, Messing und Aluminium"
//! item list). No prices anywhere on the page; the long exclusion
//! list after "Sicherheits- und Annahmegründen nicht
//! entgegennehmen" is deliberately NOT ingested. This handler only
//! fills `trader_materials` plus contact enrichment. Zero prices
//! with resolved acceptances is normal operation.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "nw-wegberg-willi-plum-u-sohn";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://plum-wegberg.de/impressum/";

pub const URL: &str = "https://plum-wegberg.de/leistungen-metallankauf-schrottentsorgung-wegberg/";

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

/// Explicit label → acceptances. "Eisen- und Stahlschrott" is the
/// catalog's `mischschrott` description almost verbatim; "Edelstahl
/// (V2A und V4A)" fans out (esh precedent); "Elektrokabel und
/// Kabelschrott" is copper installation cable → `kabel-kupfer`.
/// "Buntmetalle" is only the category word — its items are ingested
/// individually — so it skips loudly.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("eisen") && l.contains("stahl") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("v2a") && l.contains("v4a") {
        Some(vec![("edelstahl-v2a", ""), ("edelstahl-v4a", "")])
    } else if l.contains("edelstahl") {
        Some(vec![("edelstahl-gemischt", "")])
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("alu") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("kabel") {
        Some(vec![("kabel-kupfer", "")])
    } else {
        None
    }
}

fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    // The single acceptance paragraph between "Wir nehmen unter
    // anderem" and the "Sicherheits- und Annahmegründen nicht
    // entgegennehmen" exclusion block (which ends the window so
    // excluded materials can never leak into acceptances).
    let start = html
        .find("Wir nehmen unter anderem")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeabsatz fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Sicherheits- und Annahmegründen")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeblock-Ende fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    let strong = Selector::parse("strong").expect("valid selector");
    let strongs: Vec<String> = doc
        .select(&strong)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|t| !t.is_empty())
        .collect();
    if strongs.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeabsatz leer".to_owned(),
        });
    }
    // "Buntmetalle wie Kupfer, Messing und Aluminium sowie" — the
    // category's items live in running text, not in their own tags.
    let b = window
        .find("Buntmetalle")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Buntmetall-Aufzählung fehlt".to_owned(),
        })?;
    let rest = &window[b + "Buntmetalle".len()..];
    let wie = rest.find("wie ").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Buntmetall-Aufzählung fehlt".to_owned(),
    })?;
    let after_wie = &rest[wie + "wie ".len()..];
    let sowie = after_wie.find(" sowie").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Buntmetall-Aufzählung offen".to_owned(),
    })?;
    let items = after_wie[..sowie].replace(" und ", ",");
    let items: Vec<String> = items
        .split(',')
        .map(|p| {
            scraper_text(p)
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|t| !t.is_empty())
        .collect();
    if items.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Buntmetall-Aufzählung leer".to_owned(),
        });
    }
    // Document order: Eisen- und Stahlschrott, Buntmetalle (+ items),
    // Edelstahl, Kabel.
    let mut labels = Vec::with_capacity(strongs.len() + items.len());
    for s in strongs {
        labels.push(s.clone());
        if s.to_lowercase().contains("buntmetall") {
            labels.extend(items.clone());
        }
    }
    Ok(labels)
}

/// Strip stray tags from a small inline fragment (the "wie … sowie"
/// slice can carry a closing `</strong>` tail).
fn scraper_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
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
    out
}

/// Bespoke contact extraction for THIS impressum only: the
/// `<p><strong>Willi Plum u. Sohn GmbH & Co. KG</strong><br
/// />Friedrich List Allee 19<br />41844 Wegberg</p>` block plus the
/// "Telefon:"/"E-Mail:" paragraph. Anchored on "Willi Plum u.
/// Sohn" — without it the page changed shape → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let all = doc
        .root_element()
        .text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if !all.contains("Willi Plum u. Sohn") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Firmen-Block fehlt".to_owned(),
        });
    }
    // Address lines from the firm <p> ("Willi Plum u. Sohn …" /
    // "Friedrich List Allee 19" / "41844 Wegberg"): scraper text()
    // glues adjacent nodes ("KG"+"Friedrich", "19"+"41844"), so split
    // the <p> on <br> into real lines instead of tokenizing glued text.
    let p_sel = Selector::parse("p").expect("valid selector");
    let firm_p = doc.select(&p_sel).find(|p| {
        p.text().collect::<String>().contains("Willi Plum u. Sohn")
    });
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
                city = (*ci).to_owned();
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                break;
            }
        }
    }
    let phone = all
        .split_once("Telefon:")
        .map(|(_, r)| {
            let end = r.find("E-Mail:").unwrap_or(r.len());
            r[..end]
                .split_whitespace()
                .take_while(|t| {
                    t.chars()
                        .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();
    // Email is one token (the phone-style take_while above would stop
    // at the first letter — emails need their own rule). Glued block
    // boundaries ("…deGeschäftsführer:") defeat token splitting, so
    // expand from the '@' over email characters instead.
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
    let Some(at) = r.find('@') else { return String::new() };
    let b = r.as_bytes();
    let is_email = |c: u8| c.is_ascii_alphanumeric() || b".-_+@".contains(&c);
    let mut s = at;
    while s > 0 && is_email(b[s - 1]) { s -= 1; }
    let mut e = at + 1;
    while e < b.len() && is_email(b[e]) { e += 1; }
    let cand = &r[s..e];
    // Glued trailing prose ("…deGeschäftsführer:") survives the
    // expansion (all alphanumeric) — cut at the end of the domain.
    for suffix in [".de", ".com", ".net", ".org", ".eu", ".info", ".biz"] {
        if let Some(p) = cand.rfind(suffix) {
            let cut = cand[..p + suffix.len()].trim_matches(|c| c == '[' || c == ']').to_owned();
            if cut.contains('@') && !cut.starts_with('@') {
                return cut;
            }
        }
    }
    String::new()
}

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant (` />`) — drop everything up to the first '>' first, or the
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

    const FIXTURE: &str = "<p class=\"isSelectedEnd\">Wir nehmen unter anderem <strong>Eisen- und Stahlschrott</strong> \
        aus Bau, Abbruch, Werkstatt und Produktion, <strong>Buntmetalle</strong> wie Kupfer, Messing und Aluminium sowie \
        <strong>Edelstahl (V2A und V4A)</strong> in unterschiedlichen Formen an. Auch \
        <strong>Elektrokabel und Kabelschrott</strong> gehören zu unserem Annahmesortiment.</p>\
        <p class=\"isSelectedEnd\">Einige Materialien können wir aus <strong>Sicherheits- und Annahmegründen nicht \
        entgegennehmen</strong>. Dazu gehören unter anderem Mineralwolle, Asbest, Öle, Batterien.</p>";

    #[test]
    fn paragraph_and_items_parse_in_order() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(
            labels,
            vec![
                "Eisen- und Stahlschrott",
                "Buntmetalle",
                "Kupfer",
                "Messing",
                "Aluminium",
                "Edelstahl (V2A und V4A)",
                "Elektrokabel und Kabelschrott",
            ]
        );
        // Excluded materials must never leak in.
        assert!(!labels
            .iter()
            .any(|l| l.contains("Batterien") || l.contains("Asbest")));
        // Anchors are mandatory: redesign fails loudly.
        assert!(parse("<p>Neu hier</p>").is_err());
        assert!(parse("<p>Wir nehmen unter anderem <strong>X</strong></p>").is_err());
    }

    #[test]
    fn impressum_firm_block() {
        let imp = "<h2>Impressum</h2>\
            <p><strong>Willi Plum u. Sohn GmbH &amp; Co. KG</strong><br />Friedrich List Allee 19<br />41844 Wegberg</p>\
            <p>Telefon: +49 (0)2432 933 81 88<br />E-Mail: info@plum-baesweiler.de</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Friedrich List Allee 19");
        assert_eq!(info.postcode, "41844");
        assert_eq!(info.city, "Wegberg");
        assert_eq!(info.phone, "+49 (0)2432 933 81 88");
        assert_eq!(info.email, "info@plum-baesweiler.de");
        assert!(super::extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }

    #[test]
    fn mapping_maps_and_skips_category() {
        assert_eq!(
            grade_for("Eisen- und Stahlschrott"),
            Some(vec![("mischschrott", "")])
        );
        assert_eq!(grade_for("Kupfer"), Some(vec![("kupfer-gemischt", "")]));
        assert_eq!(grade_for("Messing"), Some(vec![("messing", "")]));
        assert_eq!(
            grade_for("Aluminium"),
            Some(vec![("aluminium-gemischt", "")])
        );
        assert_eq!(
            grade_for("Edelstahl (V2A und V4A)"),
            Some(vec![("edelstahl-v2a", ""), ("edelstahl-v4a", "")])
        );
        assert_eq!(
            grade_for("Elektrokabel und Kabelschrott"),
            Some(vec![("kabel-kupfer", "")])
        );
        assert_eq!(
            grade_for("Buntmetalle"),
            None,
            "category word, items ingested separately"
        );
    }
}
