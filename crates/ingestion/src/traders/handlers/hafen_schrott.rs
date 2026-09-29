//! HAFEN Metall & Schrott (Mannheim): acceptance list without prices.
//! The homepage's "Diese Metalle kaufen wir an" board lists 51 grades in
//! `<ul class="sorten">` blocks (`<details>` per category) and closes
//! with "Preise ändern sich täglich … rufen Sie an für den aktuellen
//! Tagespreis" — Tagespreis by phone only. This handler turns the grades
//! into `trader_materials` acceptances plus contact enrichment. Zero
//! prices with resolved acceptances is normal operation, not a canary
//! trip.

use scraper::{ElementRef, Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "bw-mannheim-hafen-metall-schrott";
/// Bespoke, live-verified impressum URL (site footer links the relative
/// "impressum.html"). A move fails the step loudly (fix the URL) — never
/// guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://hafen-schrott.de/impressum.html";

pub const URL: &str = "https://hafen-schrott.de/";

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

/// Explicit grade → acceptances, specific before generic. Mixed-metal
/// grades ("Kupfer-Messing-Kühler", "Alu-Kupfer-Kühler") map to nothing —
/// attributing them to one side would be a wrong acceptance. E-grades
/// without a catalog material (CPUs, RAM, drives, PSUs, bare PCs) and
/// "Schwerschrott E3" are skipped loudly (see proposals in the step
/// report).
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(vec![("kupfer-millberry", "")])
    } else if l.contains("kerze") {
        Some(vec![("kupfer-berry", "Kerze")])
    } else if l.contains("berry") {
        Some(vec![("kupfer-berry", "1A")])
    } else if l.contains("kupferkabel") {
        // Grades: the Cu share IS the grade ("min. 38 % Cu" …).
        if l.contains("38") {
            Some(vec![("kabel-kupfer", "min. 38 % Cu")])
        } else if l.contains("60") {
            Some(vec![("kabel-kupfer", "min. 60 % Cu")])
        } else if l.contains("70") {
            Some(vec![("kabel-kupfer", "min. 70 % Cu")])
        } else {
            Some(vec![("kabel-kupfer", "neu")])
        }
    } else if l.contains("kühler") || l.contains("kuehler") {
        if l.contains("kupfer-messing") || l.contains("alu-kupfer") {
            None // mixed-metal, unattributable
        } else {
            Some(vec![("aluminium-gemischt", "")])
        }
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("rotguss") {
        Some(vec![("bronze-rotguss", "")])
    } else if l.contains("messing") {
        if l.contains("ms58") && l.contains("neu") {
            Some(vec![("messing", "MS58 neu")])
        } else if l.contains("patronen") || l.contains("hülsen") || l.contains("huelsen") {
            Some(vec![("messing", "Patronenhülsen")])
        } else if l.contains("späne") || l.contains("spaene") {
            Some(vec![("messing", "Späne")])
        } else {
            Some(vec![("messing", "")])
        }
    } else if l.contains("schälblei") || l.contains("schaelblei") {
        Some(vec![("blei", "Kabel-Schälblei")])
    } else if l.contains("auswucht") || l.contains("wuchtgewicht") {
        Some(vec![("blei", "Auswucht")])
    } else if l.contains("blei") {
        if l.contains("batter") {
            Some(vec![("blei", "Batterien")])
        } else {
            Some(vec![("blei", "")])
        }
    } else if l.contains("v4a") {
        Some(vec![("edelstahl-v4a", "")])
    } else if l.contains("v2a") || l.contains("nirosta") {
        Some(vec![("edelstahl-v2a", "")])
    } else if l.contains("edelstahl") {
        Some(vec![("edelstahl-gemischt", "Späne")])
    } else if l.contains("alu") || l.contains("aluminium") {
        if l.contains("späne") || l.contains("spaene") {
            Some(vec![("aluminium-gemischt", "Späne")])
        } else if l.contains("blech") {
            Some(vec![("aluminium-blech", "")])
        } else if l.contains("dosen") || l.contains("offset") {
            Some(vec![("aluminium-blech", "")])
        } else if l.contains("profile") {
            if l.contains("färbig") || l.contains("faerbig") {
                Some(vec![("aluminium-profile", "färbig")])
            } else {
                Some(vec![("aluminium-profile", "")])
            }
        } else if l.contains("felgen") || l.contains("guss") {
            Some(vec![("aluminium-guss", "")])
        } else {
            Some(vec![("aluminium-gemischt", "")])
        }
    } else if l.contains("elektro-motoren") || l.contains("elektromotor") {
        Some(vec![("elektromotoren", "")])
    } else if l.contains("leiterplatten") {
        if l.contains("1a") {
            Some(vec![("platinen", "Klasse 1A")])
        } else {
            Some(vec![("platinen", "Klasse 2")])
        }
    } else if l.contains("mischschrott") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else {
        None
    }
}

/// The 51 grades in the `<ul class="sorten">` blocks between the
/// "Diese Metalle kaufen wir an" heading and the "tafel-fuss" call box
/// ("Tagespreis erfragen"). Missing anchors or zero grades fail loudly —
/// a silent success would hide a redesign.
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let start = html
        .find("Diese Metalle kaufen wir an")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankauf-Liste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("tafel-fuss").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Ankauf-Liste ohne Ende".to_owned(),
    })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    let li = Selector::parse("ul.sorten li").expect("valid selector");
    let labels: Vec<String> = doc
        .select(&li)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|t| !t.is_empty())
        .collect();
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    Ok(labels)
}

/// Bespoke contact extraction for THIS impressum only: the
/// `div.adresse` block holds firm + street + "D-68169 Mannheim", and the
/// `<p>` after the "Kontakt" heading holds labeled Telefon/Telefax/
/// E-Mail lines. Missing anchors → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let addr = Selector::parse("div.adresse").expect("valid selector");
    let h2 = Selector::parse("h2").expect("valid selector");
    let addr_el = doc.select(&addr).next().ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Adress-Block fehlt".to_owned(),
    })?;
    // "HAFEN Metall & Schrott GmbH<br>Inselstraße 8<br>D-68169
    // Mannheim" — split on "<br", drop tag remnants first.
    let lines: Vec<String> = addr_el
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    for line in &lines {
        let low = line.to_lowercase();
        if low.contains("straße") || low.contains("strasse") || low.contains("str.") {
            street = line.clone();
        }
        for tok in line.split_whitespace() {
            let digits = tok.trim_start_matches("D-");
            if digits.len() == 5 && digits.chars().all(|c| c.is_ascii_digit()) {
                postcode = digits.to_owned();
                // City follows the PLZ token inside the same line
                // ("D-68169 Mannheim").
                let rest: Vec<&str> = line.split_whitespace().collect();
                if let Some(pos) = rest.iter().position(|t| *t == tok) {
                    city = rest[pos + 1..].join(" ");
                }
            }
        }
    }
    // Contact lines: first <p> sibling after the "Kontakt" heading.
    let anchor = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Kontakt");
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let contact_p = anchor
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let mut phone = String::new();
    let mut email = String::new();
    if let Some(p) = contact_p {
        for part in p.inner_html().split("<br") {
            let t = strip_fragment(part);
            if let Some(rest) = t.strip_prefix("Telefon:") {
                phone = rest
                    .split_whitespace()
                    .take_while(|tok| {
                        tok.chars()
                            .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
            } else if let Some(rest) = t.strip_prefix("E-Mail:") {
                // E-mail needs its own rule: the phone-style take_while
                // would stop at the first letter.
                email = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .to_owned();
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

/// Strip tags from a `<br`-split fragment (drop everything up to the
/// first '>' first, or tag attributes parse as text).
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
    out.replace("&nbsp;", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    // Real shapes from the live homepage (entities verbatim).
    const FIXTURE: &str = "<h2>Diese Metalle kaufen wir an</h2>\
        <div class=\"tafel\"><details><summary><span class=\"name\">Kupfer</span></summary>\
        <ul class=\"sorten\"><li>Kupfer 1A, Berry, min. 98&nbsp;% Cu</li>\
        <li>Kupfer blank I, Millberry</li><li>Kupfer blank II, Kerze</li>\
        <li>Kupfer-Messing-Kühler, rein</li><li>Kupfer, gemischt, alt</li></ul></details>\
        <details><summary><span class=\"name\">Blei</span></summary>\
        <ul class=\"sorten\"><li>Kabel-Schälblei</li><li>Schwerschrott E3</li>\
        <li>CPU, Keramik, gemischt</li><li>Elektro-Motoren</li>\
        <li>Leiterplatten Klasse 1A, alt</li></ul></details>\
        <div class=\"tafel-fuss\"><p>Preise ändern sich täglich</p></div></div>";

    #[test]
    fn impressum_adresse_block() {
        let imp = "<div class=\"adresse\"><b>HAFEN Metall &amp; Schrott GmbH</b><br>\
            Inselstraße 8<br>D-68169 Mannheim</div>\
            <h2>Kontakt</h2><p>Telefon: 0621&nbsp;799&nbsp;3777&nbsp;4<br>\
            Telefax: 0621&nbsp;799&nbsp;3777&nbsp;6<br>E-Mail: hafen-schrotthandel@gmx.de</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Inselstraße 8");
        assert_eq!(info.postcode, "68169");
        assert_eq!(info.city, "Mannheim");
        assert_eq!(info.phone, "0621 799 3777 4");
        assert_eq!(info.email, "hafen-schrotthandel@gmx.de");
        assert!(super::extract_info("<div>Ohne Adresse</div>").is_err());
    }

    #[test]
    fn sorten_window_maps_and_skips() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels.len(), 10);
        assert!(labels.contains(&"Kupfer 1A, Berry, min. 98 % Cu".to_owned()));
        assert_eq!(
            grade_for("Kupfer 1A, Berry, min. 98 % Cu"),
            Some(vec![("kupfer-berry", "1A")])
        );
        assert_eq!(
            grade_for("Kupfer blank I, Millberry"),
            Some(vec![("kupfer-millberry", "")])
        );
        assert_eq!(
            grade_for("Kupfer blank II, Kerze"),
            Some(vec![("kupfer-berry", "Kerze")])
        );
        assert_eq!(
            grade_for("Kupferkabel, min. 60 % Cu"),
            Some(vec![("kabel-kupfer", "min. 60 % Cu")])
        );
        assert_eq!(
            grade_for("Kupfer-Messing-Kühler, rein"),
            None,
            "mixed-metal"
        );
        assert_eq!(grade_for("Alu-Kupfer-Kühler, rein"), None, "mixed-metal");
        assert_eq!(
            grade_for("Alu-Kühler, rein"),
            Some(vec![("aluminium-gemischt", "")])
        );
        assert_eq!(
            grade_for("Kabel-Schälblei"),
            Some(vec![("blei", "Kabel-Schälblei")])
        );
        assert_eq!(
            grade_for("Auswuchtblei, Wuchtgewichte"),
            Some(vec![("blei", "Auswucht")])
        );
        assert_eq!(
            grade_for("Edelstahl, V4A Schrott"),
            Some(vec![("edelstahl-v4a", "")])
        );
        assert_eq!(
            grade_for("Edelstahl-Späne"),
            Some(vec![("edelstahl-gemischt", "Späne")])
        );
        assert_eq!(
            grade_for("Elektro-Motoren"),
            Some(vec![("elektromotoren", "")])
        );
        assert_eq!(
            grade_for("Leiterplatten Klasse 1A, alt"),
            Some(vec![("platinen", "Klasse 1A")])
        );
        assert_eq!(
            grade_for("Schwerschrott E3"),
            None,
            "no heavy-scrap material"
        );
        assert_eq!(grade_for("CPU, Keramik, gemischt"), None, "no CPU material");
        assert_eq!(grade_for("RAM mit Gold-Kontakten"), None);
        // Missing anchors / empty lists fail loudly.
        assert!(parse("<html><body>Neu hier</body></html>").is_err());
        assert!(parse("<h2>Diese Metalle kaufen wir an</h2><div></div>").is_err());
    }
}
