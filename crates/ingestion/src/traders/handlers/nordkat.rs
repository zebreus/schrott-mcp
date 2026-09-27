//! NORDKAT GmbH (Harsefeld): catalyst / spark-plug / sensor buyer whose
//! price page lists four product groups as PDF downloads — no inline
//! prices. The PDFs are image-based (no text layer, no OCR in the
//! pipeline) and the only inline figure is a "bis 25,- Euro pro Stück"
//! spark-plug example. So this handler records acceptances for what the
//! catalog covers (ceramic catalysts) and skips the rest loudly:
//! metal-substrate and biogas catalysts, spark plugs and lambda/NOx
//! sensors have no catalog material (proposals in the skip labels).

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "ni-harsefeld-21698-nordkat";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.nordkat.de/Impressum/";

pub const URL: &str = "https://www.nordkat.de/Preislisten-ankaufspreise-edelmetalle/";

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
    let (labels, mut skipped_labels) = parse(&html)?;
    let mut acceptances = Vec::new();
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
            None => skipped_labels.push(format!("{label}{}", skip_reason(&label))),
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

/// Explicit group → acceptances. Only ceramic catalysts fit the catalog
/// (`katalysatoren` = "Katalysatoren (Keramik)"): metal-substrate and
/// biogas/BHKW catalysts are different products with different value
/// bands and must not hide under the ceramic label; spark plugs and
/// sensors have no material at all.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    if l.contains("keramik") && l.contains("katalysator") {
        Some(vec![("katalysatoren", "Keramik")])
    } else {
        None
    }
}

/// Loud reason for every unmapped group — the new-material proposals
/// live here, not in a silent default.
fn skip_reason(label: &str) -> &'static str {
    let l = label.to_lowercase();
    if l.contains("metall") && l.contains("katalysator") {
        " (Metallträger-Kat, kein Katalogmaterial — Vorschlag: katalysatoren-metall)"
    } else if l.contains("biogas") {
        " (Biogas/BHKW-Kat, kein Katalogmaterial — Vorschlag: katalysatoren-metall)"
    } else if l.contains("zündkerze") || l.contains("sonde") {
        " (kein Katalogmaterial — Vorschläge: zuendkerzen, lambdasonden)"
    } else if l.contains("beispielpreis") {
        " (Beispielpreis ohne Katalogmaterial: Zündkerze)"
    } else {
        " (unbekannte Gruppe, kein Katalogmaterial)"
    }
}

fn parse(html: &str) -> Result<(Vec<String>, Vec<String>), IngestError> {
    // Window: the four "Preisliste …" groups live between the page H1 and
    // the shipping-note section ("Begleitschreiben"). Footer link lists
    // must never leak in as groups.
    let start = html
        .find("Preislisten zu unseren Ankaufspreisen")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preislisten-Kopf fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("Begleitschreiben").unwrap_or(tail.len());
    let window = &tail[..end];
    // NO <script>/<style> as text: photo-gallery widgets inject scripts
    // between the headers — select content elements only.
    let doc = Html::parse_fragment(window);
    let sel = Selector::parse("h2, h3, p").expect("valid selector");
    // Headers come in pairs ("Preisliste" + product); the <p> "Info:
    // Preise Zündkerze bis 25,- Euro pro Stück …" is a price candidate
    // without catalog material → loud skip, never a row.
    let mut labels = Vec::new();
    let mut skipped = Vec::new();
    let mut pending_preisliste = false;
    for el in doc.select(&sel) {
        let t = el.text().collect::<String>();
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if t == "Preisliste" {
            pending_preisliste = true;
            continue;
        }
        if pending_preisliste {
            pending_preisliste = false;
            let name = t.trim_end_matches(':').trim().to_owned();
            if !name.is_empty() && name.len() <= 120 {
                labels.push(name);
            }
            continue;
        }
        if t.starts_with("Preisliste ") {
            let name = t["Preisliste ".len()..]
                .trim_end_matches(':')
                .trim()
                .to_owned();
            if !name.is_empty() {
                labels.push(name);
            }
            continue;
        }
        if t.contains("Euro pro Stück") || t.contains("Euro/Stück") {
            skipped.push(format!("{t}{}", skip_reason(&t)));
        }
    }
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preislisten-Gruppen".to_owned(),
        });
    }
    Ok((labels, skipped))
}

/// Bespoke contact extraction for THIS impressum only: `h2.cm-h1`
/// headings ("NORDKAT GmbH", "Geschäftsführer: Dietmar Kaun",
/// "Am Bauhof 5"), a `<p>` with "21698 Harsefeld", "Telefon Einkauf:"
/// lines and a hex-encoded mailto (`mailto:%6B%6F…` = kontakt@…).
/// Missing anchors → loud error, never guessed.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let all = doc
        .root_element()
        .text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for anchor in ["Geschäftsführer:", "Am Bauhof 5"] {
        if !all.contains(anchor) {
            return Err(IngestError::Parse {
                url: IMPRESSUM_URL.to_owned(),
                detail: format!("{anchor} fehlt"),
            });
        }
    }
    let street = "Am Bauhof 5".to_owned();
    // Postcode + city: scraper text() glues adjacent nodes ("5"+"21698"
    // across the h2/p boundary), so scan <p> elements for one whose
    // first token is a bare 5-digit PLZ instead of tokenizing the
    // glued whole-page text.
    let p_sel = Selector::parse("p").expect("valid selector");
    let (mut postcode, mut city) = (String::new(), String::new());
    for el in doc.select(&p_sel) {
        let t: String = el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let mut it = t.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5
                && pc.chars().all(|c| c.is_ascii_digit())
                && ci.chars().next().is_some_and(|c| c.is_uppercase())
            {
                postcode = pc.to_owned();
                city = ci.to_owned();
                break;
            }
        }
    }
    if postcode.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "PLZ/Ort fehlt".to_owned(),
        });
    }
    let phone = all
        .find("Telefon Einkauf:")
        .map(|i| {
            all[i + "Telefon Einkauf:".len()..]
                .split_whitespace()
                .take_while(|t| {
                    t.chars()
                        .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();
    // E-Mail lives in a hex-encoded mailto (see module docs); the phone
    // token filter above would stop at the first letter, so mail needs
    // its own rule anchored on the href.
    let a = Selector::parse("a").expect("valid selector");
    let mut email = String::new();
    for el in doc.select(&a) {
        if let Some(href) = el.value().attr("href") {
            if let Some(enc) = href.strip_prefix("mailto:") {
                email = percent_decode(enc);
                break;
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

/// Decode the hex-mailto this impressum uses (`%6B` → `k`, …).
fn percent_decode(enc: &str) -> String {
    let bytes = enc.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut k = 0;
    while k < bytes.len() {
        if bytes[k] == b'%' && k + 2 < bytes.len() {
            if let Ok(hex) = std::str::from_utf8(&bytes[k + 1..k + 3]) {
                if let Ok(b) = u8::from_str_radix(hex, 16) {
                    out.push(b);
                    k += 3;
                    continue;
                }
            }
        }
        out.push(bytes[k]);
        k += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, percent_decode, skip_reason};

    // Real structure, shortened: cm_table with paired h3 headers (h2 for
    // the last group), download widgets, the "bis 25" info paragraph,
    // terminated by the Begleitschreiben section.
    const FIXTURE: &str = "<h1><span>Preislisten zu unseren Ankaufspreisen: \
        Unkompliziert online checken.</span></h1>\
        <table id=\"CM19e41244a7250475f25500cf\" class=\"cm_table\"><tbody>\
        <tr><td><h3><span>Preisliste </span></h3>\
        <h3><span>Zündkerzen + Sonden BHKW/Pkw:</span><br></h3></td>\
        <td><h3>Preisliste </h3><h3>Metall-Katalysatoren:</h3></td>\
        <td><h3>Preisliste </h3><h3>Biogas-Katalysatoren:</h3></td>\
        <td><h2><span>Preisliste </span></h2><h2>Keramik-Katalysatoren:</h2></td></tr>\
        <tr><td><b>Aktuelle Ankaufspreise für gebrauchte Zündkerzen</b><br>\
        Download Preisliste PDF-Datei<br>\
        <a href=\"/.cm4all/uproc.php/0/NK%20Preisliste%20Z%C3%BCndkerzen%20260926.pdf\">\
        NK Preisliste Zündkerzen 260926.pdf</a> (98.04KB)\
        <p><em>Info: </em>Preise Zündkerze bis 25,- Euro pro Stück möglich \
        (Beispielpreis für Zündkerze von 2G, 4 polig, alte Generation)<br></p></td>\
        <td>Aktuelle Ankaufspreise für gebrauchte Metall-Katalysatoren</td></tr>\
        </tbody></table><p>Preis-Anfrage ► Begleitschreiben für Ihr Paket an uns</p>\
        <footer><h3>Home</h3><h3>Kontakt</h3></footer>";

    #[test]
    fn groups_map_and_skip() {
        let (labels, skipped) = parse(FIXTURE).expect("parses");
        assert_eq!(
            labels,
            vec![
                "Zündkerzen + Sonden BHKW/Pkw",
                "Metall-Katalysatoren",
                "Biogas-Katalysatoren",
                "Keramik-Katalysatoren",
            ]
        );
        // The "bis 25" example is a price candidate without material.
        assert_eq!(skipped.len(), 1);
        assert!(skipped[0].contains("bis 25"), "{skipped:?}");
        assert!(skipped[0].contains("Beispielpreis"), "{skipped:?}");
        // Only ceramic fits the catalog; the rest propose new materials.
        assert_eq!(
            grade_for("Keramik-Katalysatoren"),
            Some(vec![("katalysatoren", "Keramik")])
        );
        assert_eq!(grade_for("Metall-Katalysatoren"), None);
        assert_eq!(grade_for("Biogas-Katalysatoren"), None);
        assert_eq!(grade_for("Zündkerzen + Sonden BHKW/Pkw"), None);
        assert!(skip_reason("Metall-Katalysatoren").contains("katalysatoren-metall"));
        assert!(skip_reason("Biogas-Katalysatoren").contains("katalysatoren-metall"));
        assert!(skip_reason("Zündkerzen + Sonden BHKW/Pkw").contains("zuendkerzen"));
        assert!(skip_reason("Zündkerzen + Sonden BHKW/Pkw").contains("lambdasonden"));
    }

    #[test]
    fn window_and_anchors_hold() {
        assert!(parse("<h1>Preislisten zu unseren Ankaufspreisen</h1>").is_err());
        assert!(parse("<p>Kein Kopf hier</p>").is_err());
        assert!(extract_info("<p>Neu hier</p>").is_err());
        assert!(
            extract_info("<p>Geschäftsführer: X</p>").is_err(),
            "street anchor"
        );
    }

    #[test]
    fn impressum_hex_mailto() {
        assert_eq!(
            percent_decode("%6B%6F%6E%74%61%6B%74%40nordkat.de"),
            "kontakt@nordkat.de"
        );
        let imp = "<h2 class=\"cm-h1\"><span>NORD</span>KAT <span>GmbH</span></h2>\
            <h2 class=\"cm-h1\"><span><span>Geschäftsführer:</span><span> Dietmar Kaun</span></span></h2>\
            <h2 class=\"cm-h1\"><span>Am Bauhof 5</span></h2>\
            <p><span>21698 Harsefeld</span></p>\
            <p><span>Telefon Einkauf: +49 - (0) 4164/ 87 99 368 </span></p>\
            <p><span>E-Mail:</span>\
            <a href=\"mailto:%6B%6F%6E%74%61%6B%74%40nordkat.de\" class=\"cm_anchor\">\
            kontakt@nordkat.de</a></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Am Bauhof 5");
        assert_eq!(info.postcode, "21698");
        assert_eq!(info.city, "Harsefeld");
        assert_eq!(info.phone, "+49 - (0) 4164/ 87 99 368");
        assert_eq!(info.email, "kontakt@nordkat.de");
    }
}
