//! WKR GmbH (Köln-Longerich/Rodenkirchen/Niehl): JS-only Tagespreisliste.
//! Die `/preise/`-Seite zeigt elf Zähler-Module (`span.uabb-number-before-text`:
//! "Alu Geschirr 10%", "V2A", "V4A", "Zink", "Mischschrott", "Millberry",
//! "Kupferschwer", "Kupferkabel 40% ohne Stecker", "Kupferschälkabel, 50%",
//! "Kupferlitzenkabel, 50%", "Messing" — 10× "€ / kg", "Mischschrott" als
//! einziges in "€ / t."). Live verifiziert (28.09.2026, ~103 kB statisches
//! HTML): ALLE `span.uabb-number-int`-Werte stehen statisch auf "0", die
//! echten Tagespreise werden erst per JS nachgeladen. Es gibt daher nichts
//! zu parsen, was ein Preis wäre — stattdessen ist dieser Handler Kontakt +
//! Annahme-only (esh.rs-/gold_richtig.rs-Muster): die Zähler-Labels sind der
//! maschinenlesbare Annahmenachweis, der JS-only-Zustand wird als laute
//! Notiz in `skipped_labels` gemeldet, nie als erfundene Preiszeile. Kein
//! Seitendatum (die Seite nennt keines) → `published_at` ist `None`.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "nw-koln-wkr";
/// Bespoke, live-verified price URL (Footer-Link "Altmetall-Preise"). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://www.wkr-schrott.de/preise/";
/// Bespoke, live-verified impressum URL (Footer-Link "Impressum"). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.wkr-schrott.de/kontakt/impressum/";

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
    let (labels, js_notes) = parse(&html)?;
    let mut acceptances = Vec::with_capacity(labels.len());
    let mut skipped_labels = js_notes;
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
            None => skipped_labels.push(format!("{label} (kein Katalogmaterial)")),
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

/// Explicit label → acceptances. One label may fan out to several materials;
/// anything unlisted skips loudly. "Kupferschwer" (blankes, schweres Kupfer)
/// ist kein Kabel und landet auf `kupfer-gemischt`, die drei Kabelsorten mit
/// genannter Kupferausbeute auf `kabel-kupfer` (Bedingung hält die Ausbeute,
/// das Rohlabel steht ohnehin im Record).
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(vec![("kupfer-millberry", "")])
    } else if l.contains("kupferschwer") {
        Some(vec![("kupfer-gemischt", "schwer")])
    } else if l.contains("kabel") {
        // Ambiguous mixes ("Kabel / E-Motoren") name two catalog areas and
        // stay out loudly — a wrong acceptance is worse than a logged gap.
        if l.contains("motor") || l.contains("elektro") {
            return None;
        }
        // "Kupferkabel 40% ohne Stecker", "Kupferschälkabel, 50%",
        // "Kupferlitzenkabel, 50%": Ausbeute als Bedingung.
        if l.contains("40%") {
            Some(vec![("kabel-kupfer", "40% ohne Stecker")])
        } else if l.contains("schälkabel") || l.contains("schalkabel") {
            Some(vec![("kabel-kupfer", "Schälkabel 50%")])
        } else if l.contains("litzen") {
            Some(vec![("kabel-kupfer", "Litzenkabel 50%")])
        } else {
            Some(vec![("kabel-kupfer", "")])
        }
    } else if l.contains("v4a") {
        Some(vec![("edelstahl-v4a", "")])
    } else if l.contains("v2a") {
        Some(vec![("edelstahl-v2a", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("mischschrott") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("alu") && l.contains("geschirr") {
        Some(vec![("aluminium-blech", "Geschirr")])
    } else {
        None
    }
}

/// The counter labels in the price window between the "Altmetall-Preise"
/// heading and the "Die angegebenen Preise sind freibleibend" conditions
/// paragraph. Returns (labels, js_notes): the counter values carry no static
/// prices (all "0" until JS runs), so the JS-only state is reported as one
/// loud note, never a faked row. If static values ever DO appear, the note
/// says so loudly — that is the signal to upgrade to a full price handler.
fn parse(html: &str) -> Result<(Vec<String>, Vec<String>), IngestError> {
    let start = html
        .find("Altmetall-Preise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Die angegebenen Preise sind freibleibend")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock unvollständig".to_owned(),
        })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let label_sel = Selector::parse("span.uabb-number-before-text").expect("valid selector");
    let value_sel = Selector::parse("span.uabb-number-int").expect("valid selector");
    let labels: Vec<String> = frag
        .select(&label_sel)
        .map(|el| {
            el.text()
                .collect::<String>()
                .replace(['\u{a0}'], " ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|t| !t.is_empty() && t.len() <= 120)
        .collect();
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    // JS evidence: counter values hold "0" until the price script runs.
    let values: Vec<String> = frag
        .select(&value_sel)
        .map(|el| el.text().collect::<String>().trim().to_owned())
        .collect();
    let mut js_notes = Vec::new();
    if !values.is_empty()
        && values
            .iter()
            .all(|v| v == "0" || v == "–" || v == "-" || v.is_empty())
    {
        js_notes.push(
            "Tagespreise nur per JS (uabb-number-int statisch 0), keine Preise übernommen"
                .to_owned(),
        );
    } else if values
        .iter()
        .any(|v| v.chars().any(|c| c.is_ascii_digit()) && v != "0" && !v.is_empty())
    {
        js_notes.push("Statische Zählerwerte erkannt — voller Preis-Handler prüfen".to_owned());
    }
    Ok((labels, js_notes))
}

/// Bespoke contact extraction for THIS impressum only: the first `<p>` holding
/// "WKR GmbH" + "Robert-Bosch-Straße" carries firm + street + PLZ city as
/// `<br>` lines ("WKR GmbH" / "Robert-Bosch-Straße 20-22" / "50739
/// Köln-Longerich"), and the `<p>` after the `<h2>Kontakt</h2>` heading holds
/// "Telefon: …" + "E-Mail: …" as `<br>` lines. Missing anchors mean the page
/// changed shape → loud error, never a guessed fallback. The Longerich
/// address wins (seed city is Köln).
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    if !doc
        .select(&h2)
        .any(|el| el.text().collect::<String>().trim() == "Kontakt")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    }
    // Address lines from the <br>-split firm paragraph (drop up to the first
    // '>' first so tag attributes never parse as text).
    let addr_p = doc.select(&p).find(|el| {
        let t: String = el.text().collect();
        t.contains("WKR GmbH") && t.contains("Robert-Bosch")
    });
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let lines: Vec<String> = addr_p
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    for line in &lines {
        if line.contains("Stra") && street.is_empty() {
            street = line.clone();
        }
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = std::iter::once(ci).chain(it).collect::<Vec<_>>().join(" ");
            }
        }
    }
    // Contact lines from the <br>-split paragraph after the Kontakt heading.
    let contact_p = doc.select(&p).find(|el| {
        let t: String = el.text().collect();
        t.contains("Telefon:") && t.contains('@')
    });
    let (mut phone, mut email) = (String::new(), String::new());
    if let Some(el) = contact_p {
        for line in el
            .inner_html()
            .split("<br")
            .map(strip_fragment)
            .filter(|s| !s.is_empty())
        {
            if let Some(rest) = line.strip_prefix("Telefon:") {
                phone = rest.trim().to_owned();
            } else if let Some(rest) = line.strip_prefix("E-Mail:") {
                email = rest.trim().to_owned();
            }
        }
    }
    if email.is_empty() {
        // Plain-text fallback: first @-token in the contact paragraph.
        if let Some(el) = contact_p {
            let t: String = el.text().collect();
            if let Some(tok) = t.split_whitespace().find(|w| w.contains('@')) {
                email = tok.trim_matches([',', ';', '.']).to_owned();
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

/// Strip tags from a `<br>`-split fragment (drop up to the first '>'
/// first so attributes never parse as text).
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
    use super::{extract_info, grade_for, parse};

    // Real shape of the live price window (28.09.2026): heading anchor,
    // all 11 counter modules verbatim (labels + units, values statically
    // "0"), conditions-paragraph anchor. Trimmed to the counter blocks.
    const FIXTURE: &str = "<h2 class=\"fl-heading\">\
        <span class=\"fl-heading-text\">Altmetall-Preise: Immer tagesaktuelle Kurse</span></h2>\
        <span class=\"uabb-number-before-text\">Alu Geschirr 10%</span>\
        <h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / kg</h2>\
        <span class=\"uabb-number-before-text\">V2A</span>\
        <h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / kg</h2>\
        <span class=\"uabb-number-before-text\">V4A</span>\
        <h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / kg</h2>\
        <span class=\"uabb-number-before-text\">Zink</span>\
        <h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / kg</h2>\
        <span class=\"uabb-number-before-text\">Mischschrott</span>\
        <h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / t.</h2>\
        <span class=\"uabb-number-before-text\">Millberry</span>\
        <h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / kg</h2>\
        <span class=\"uabb-number-before-text\">Kupferschwer</span>\
        <h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / kg</h2>\
        <span class=\"uabb-number-before-text\">Kupferkabel 40% ohne Stecker</span>\
        <h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / kg</h2>\
        <span class=\"uabb-number-before-text\">Kupferschälkabel, 50%</span>\
        <h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / kg</h2>\
        <span class=\"uabb-number-before-text\">Kupferlitzenkabel, 50%</span>\
        <h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / kg</h2>\
        <span class=\"uabb-number-before-text\">Messing</span>\
        <h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / kg</h2>\
        <p>Die angegebenen Preise sind freibleibend, frei geliefert zu unserem \
        Lager in Köln und gelten für eine Abgabemenge ab 50 kg je Sorte.</p>";

    #[test]
    fn counters_yield_labels_and_js_note() {
        let (labels, notes) = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 11);
        assert_eq!(labels[0], "Alu Geschirr 10%");
        assert_eq!(labels[4], "Mischschrott");
        assert_eq!(labels[10], "Messing");
        assert_eq!(notes.len(), 1);
        assert!(notes[0].contains("nur per JS"), "{}", notes[0]);
        // Redesign without anchors fails loudly.
        assert!(parse("<div>Redesign ohne Block</div>").is_err());
        assert!(parse("<h2>Altmetall-Preise ohne Ende").is_err());
    }

    #[test]
    fn static_values_raise_upgrade_note() {
        // If the counters ever render real numbers, the handler must say so
        // loudly (upgrade to a price handler) instead of ignoring them.
        let html = FIXTURE.replacen(
            "<span class=\"uabb-number-int\">0</span>",
            "<span class=\"uabb-number-int\">8,23</span>",
            1,
        );
        let (labels, notes) = parse(&html).expect("parses");
        assert_eq!(labels.len(), 11);
        assert_eq!(notes.len(), 1);
        assert!(notes[0].contains("Statische Zählerwerte"), "{}", notes[0]);
    }

    #[test]
    fn mapping_covers_live_counters() {
        assert_eq!(
            grade_for("Alu Geschirr 10%"),
            Some(vec![("aluminium-blech", "Geschirr")])
        );
        assert_eq!(grade_for("V2A"), Some(vec![("edelstahl-v2a", "")]));
        assert_eq!(grade_for("V4A"), Some(vec![("edelstahl-v4a", "")]));
        assert_eq!(grade_for("Zink"), Some(vec![("zink", "")]));
        assert_eq!(grade_for("Mischschrott"), Some(vec![("mischschrott", "")]));
        assert_eq!(grade_for("Millberry"), Some(vec![("kupfer-millberry", "")]));
        assert_eq!(
            grade_for("Kupferschwer"),
            Some(vec![("kupfer-gemischt", "schwer")])
        );
        assert_eq!(
            grade_for("Kupferkabel 40% ohne Stecker"),
            Some(vec![("kabel-kupfer", "40% ohne Stecker")])
        );
        assert_eq!(
            grade_for("Kupferschälkabel, 50%"),
            Some(vec![("kabel-kupfer", "Schälkabel 50%")])
        );
        assert_eq!(
            grade_for("Kupferlitzenkabel, 50%"),
            Some(vec![("kabel-kupfer", "Litzenkabel 50%")])
        );
        assert_eq!(grade_for("Messing"), Some(vec![("messing", "")]));
        // Unlisted labels stay out loudly.
        assert_eq!(grade_for("Aluminium"), None);
        assert_eq!(grade_for("Kabel / E-Motoren"), None);
    }

    #[test]
    fn impressum_extracts_longerich_contact() {
        // Real fragment shape of /kontakt/impressum/ (entities as served).
        let imp = "<div class=\"fl-rich-text\"><p><div style=\"word-wrap: break-word;\">\
            <p>WKR GmbH<br />Robert-Bosch-Stra&szlig;e 20-22<br />\
            50739 K&ouml;ln-Longerich</p>\
            <p>Handelsregister: HRB 63135<br />\
            Registergericht: Amtsgericht K&ouml;ln</p>\
            <h2>Kontakt</h2><p>Telefon: 0221 - 97 06 06 22<br />\
            E-Mail: info@wkr-schrott.de</p></div></p></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Robert-Bosch-Straße 20-22");
        assert_eq!(info.postcode, "50739");
        assert_eq!(info.city, "Köln-Longerich");
        assert_eq!(info.phone, "0221 - 97 06 06 22");
        assert_eq!(info.email, "info@wkr-schrott.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(extract_info("<h2>Kontakt</h2><p>Telefon: 1</p>").is_err());
    }
}
