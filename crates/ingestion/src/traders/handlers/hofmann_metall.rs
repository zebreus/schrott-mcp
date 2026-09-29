//! Hofmann Metall GmbH (Zwickau Hauptsitz + Chemnitz Betriebsteil):
//! KEINE eigenen Ankaufspreise auf der Site (verifiziert 28.09.2026).
//! Die Homepage-Box "Marktpreise" zeigt AUSSCHLIESSLICH Fremddaten:
//! Stahl Sorten 1-5 in EUR/t als "BDSV bundesweit" (mit Link auf die
//! bdsv.org-Historie) sowie LME-"Börsenpreise (Vortag)" in US-Dollar.
//! Diese als `haendler_angabe`-Preise zu verbuchen wären Fake-Preise —
//! daher füllt dieser Handler wie `esh`/`quell` nur `trader_materials`
//! aus der belegten Sorten-Übersicht (`/privat-kleingewerbekunden/`:
//! 11 H2-Rubriken mit `toggler`-Sortenlabels) plus Kontakt.
//! Eine Datei für beide Slugs: gleiche Seitenform (eine Annahmenseite,
//! Kontaktblöcke gleicher Bauart). Kein Datum auf der Seite →
//! `published_at: None`. Belegte Nicht-Annahmen ("... wird von uns
//! nicht angenommen!": Elektronikschrott, Weiße Ware, Autokarossen,
//! Katalysatoren) werden als laut geskippte Labels dokumentiert —
//! insbesondere Katalysatoren (Katalogmaterial!) dürfen nie als
//! Annahme auftauchen.

use scraper::{ElementRef, Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG_CHEMNITZ: &str = "sn-chemnitz-hofmann-metall-betriebsteil-chemnitz";
pub const SLUG_ZWICKAU: &str = "sn-zwickau-hofmann-metall-hauptsitz";
/// Bespoke, live-verified acceptance page (shared by both sites — the
/// single source; the homepage short list repeats the same categories,
/// fetching it too would double-record every row).
pub const URL: &str = "https://hofmann-metall.de/privat-kleingewerbekunden/";
/// Bespoke, live-verified impressum URL (company block = Hauptsitz
/// Zwickau — therefore the ZWICKAU contact source; the impressum names
/// no Chemnitz address, so Chemnitz contact comes from its own
/// Betriebsteil page below, per-page URL + per-page block).
pub const IMPRESSUM_URL: &str = "https://hofmann-metall.de/impressum/";
/// Bespoke, live-verified Chemnitz contact page (KONTAKT-Block).
pub const CHEMNITZ_URL: &str = "https://hofmann-metall.de/standorte/chemnitz/";

pub fn handler_chemnitz() -> Handler {
    Handler {
        slug: SLUG_CHEMNITZ,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_chemnitz(c)),
    }
}

pub fn handler_zwickau() -> Handler {
    Handler {
        slug: SLUG_ZWICKAU,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape_zwickau(c)),
    }
}

async fn scrape_chemnitz(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let outcome = build_outcome(status, &html, fetch_contact_chemnitz(client).await?)?;
    Ok(outcome)
}

async fn scrape_zwickau(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info_zwickau(&imp_html)?;
    Ok(build_outcome(status, &html, trader_info)?)
}

async fn fetch_contact_chemnitz(client: &reqwest::Client) -> Result<TraderInfo, IngestError> {
    // Same loud rule as the impressum: a moved Betriebsteil page means
    // the site changed shape and needs eyeballs.
    let (_, html) = fetch_text(client, CHEMNITZ_URL).await?;
    extract_info_chemnitz(&html)
}

fn build_outcome(
    status: u16,
    html: &str,
    trader_info: TraderInfo,
) -> Result<HandlerOutcome, IngestError> {
    let labels = parse(html)?;
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

/// Explicit label → acceptances. Arme stehen spezifisch-vor-generisch:
/// Token-Matching (`Al`/`Cu`/`Ms`/`VA` als ganze Tokens) statt Substring,
/// sonst fängt `al` in `Stahl-Alt-Schrott` oder `legiert` in
/// `Cu-Draht, legiert, Berry`. Eine Sorte, ein Material, zwei Preise
/// gibt es hier nicht (acceptance-only) — `conditions` trägt nur die
/// belegte "Späne"/"Auswucht"-Ausprägung, der Rest steht im Rohlabel.
/// Fan-out wo belegt ("Kabel" → Cu- UND Alu-Kabel, "legierter Schrott"
/// → VA-Stahl UND Hartmetall). Chrom, Papier, Akkus/Batterien haben
/// keinen Katalogeintrag; gemischte/mehrdeutige Sorten (`Cu-Ms Kühler`,
/// vgl. `huth`) sowie Alt-Schrott ohne Aufbereitungsangabe → `None`:
/// eine falsche Annahme ist schlimmer als eine geloggte Lücke.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    let tok: Vec<&str> = l.split(|c: char| !c.is_alphanumeric()).collect();
    let has = |t: &str| tok.contains(&t);
    // Belegte Nicht-Annahmen zuerst: "Katalysatoren (keine Annahme)"
    // darf nie auf das Katalogmaterial `katalysatoren` fallen.
    if l.contains("keine annahme") {
        None
    } else if l.contains("katalysator") {
        // Kritisch explizit: Katalogeintrag vorhanden, Annahme belegt
        // ausgeschlossen ("Katalysatoren werden von uns nicht angenommen!").
        None
    } else if l.contains("kühler") || l.contains("kuehler") {
        // "Cu-Ms Kühler": Kupfer/Messing-Mix ohne Primärgrad.
        None
    } else if l.contains("chrom") {
        None
    } else if l.contains("papier") || l.contains("akku") || l.contains("batterie") {
        // Altpapier, Blei-Akkus: kein Katalogeintrag (vgl. `metcera`).
        None
    } else if has("sorte") {
        // "Sorte 2/8 Neu-Schrott" = Neuschrott; "Sorte 4 Schredder-" =
        // Shreddervormaterial. "Sorte 1/3 Alt-Schrott" ohne
        // Aufbereitungsangabe ist weder belegter Neuschrott noch
        // belegter Scherenschrott; "Sorte 5 Stahlspäne" (C-Stahl) hat
        // keinen Eintrag.
        if l.contains("neu") {
            Some(vec![("stahlschrott-sorte-1", "")])
        } else if l.contains("schredder") || l.contains("shredder") {
            Some(vec![("stahlschrott-shredder", "")])
        } else {
            None
        }
    } else if l.contains("millberry") {
        Some(vec![("kupfer-millberry", "")])
    } else if l.contains("berry") {
        Some(vec![("kupfer-berry", "")])
    } else if l.contains("kabel") {
        // Belegt beide Ausprägungen (Cu- und Alu-Kabel); die Rubrik ohne
        // Token fächert auf beide auf.
        if has("cu") || l.contains("kupfer") {
            Some(vec![("kabel-kupfer", "")])
        } else if has("al") || l.contains("aluminium") {
            Some(vec![("kabel-alu", "")])
        } else {
            Some(vec![("kabel-kupfer", ""), ("kabel-alu", "")])
        }
    } else if l.contains("messing") || has("ms") {
        Some(vec![if l.contains("spän") {
            ("messing", "Späne")
        } else {
            ("messing", "")
        }])
    } else if has("va") {
        // Ohne V2A/V4A-Trennung → gemischt (vgl. `esh`); Späne als Ausprägung.
        Some(vec![if l.contains("spän") {
            ("edelstahl-gemischt", "Späne")
        } else {
            ("edelstahl-gemischt", "")
        }])
    } else if l.contains("hartmetall") {
        Some(vec![("hartmetall", "")])
    } else if l.contains("elektromotor") {
        Some(vec![("elektromotoren", "")])
    } else if has("al") || l.contains("aluminium") {
        // "Al-Guss" vor Eisen-Guss (Arm weiter unten); Alu-Motor =
        // Gussgehäuse ("Aluminiumguss aus Motoren und Gehäusen").
        if l.contains("guss") || l.contains("verbrennungsmotor") {
            Some(vec![("aluminium-guss", "")])
        } else if l.contains("blech") {
            Some(vec![("aluminium-blech", "")])
        } else if l.contains("profil") {
            Some(vec![("aluminium-profile", "")])
        } else {
            Some(vec![("aluminium-gemischt", "")])
        }
    } else if l.contains("zink") {
        // Vor Guss- und Legiert-Armen: "Zink-Druck-Guss",
        // "Zink, legiert, sauber".
        Some(vec![("zink", "")])
    } else if l.contains("bremsscheib") || l.contains("guss") || l.contains("verbrennungsmotor") {
        // Stahl-Verbrennungsmotor = Gussmotorenblock (Rubriktext belegt).
        Some(vec![if l.contains("spän") {
            ("eisenschrott-gussbruch", "Späne")
        } else {
            ("eisenschrott-gussbruch", "")
        }])
    } else if l.contains("auswucht") {
        Some(vec![("blei", "Auswucht")])
    } else if l.contains("blei") {
        // Nur Akkus (oben ausgeschlossen) — sonst kein Beleg.
        None
    } else if l.contains("mischschrott")
        || l.contains("scherenvormaterial")
        || l.contains("zerlegematerial")
    {
        // Scherenvormaterial = Sammelware vor der Schere, Zerlegematerial
        // = Stahl/Bunt-Mix: gemischt ohne Aufbereitung.
        Some(vec![("mischschrott", "")])
    } else if l.contains("gemischte metalle") {
        // Rubrik belegt Kupfer-E-Motoren (Guss/Eisenkern + Spulen).
        Some(vec![("elektromotoren", "")])
    } else if l.contains("legiert") {
        // Rubrik belegt VA-Stahl ("VA-Schrott wird auch als Edelstahl
        // bezeichnet") UND "Wendeschneidplatten aus Hartmetall".
        Some(vec![("edelstahl-gemischt", ""), ("hartmetall", "")])
    } else if l.contains("kupfer") || has("cu") {
        // Generisch → generisch ("Kupfer - Schrott" deckt Millberry bis
        // Cu-schwer ab, nie eine Sorte).
        Some(vec![("kupfer-gemischt", "")])
    } else if l == "sonstiges" {
        // Togglers belegen Auswuchtblei + Stahl/Bunt-Zerlegematerial.
        Some(vec![
            ("blei", "Auswucht"),
            ("mischschrott", "Stahl/Bunt-Mix"),
        ])
    } else {
        // "Fertigsorten/Vormaterial Stahlschrott" (kein Einzelgrad
        // belegbar), "Armierungseisen" (kein Eintrag), Togglers
        // "Autokarosse"/"Elektronikschrott"/"Weiße Ware" (belegte
        // Ausschlüsse, ohne "(keine Annahme)"-Suffix aus <p> belegt).
        None
    }
}

/// Labels = H2-Rubriken + `toggler`-Sortenlabels zwischen Start- und
/// End-Anker (Single-Word-Anker: live steht `<br />` mitten in den
/// Rubriken, z. B. `<h2>Fertigsorten<br />Stahlschrott</h2>`) plus
/// belegte Nicht-Annahmen aus den Rubriktexten ("X wird von uns nicht
/// angenommen!"). Start oder Ende fehlt → lauter Error, nie Ganzseite.
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let found = html
        .find("Fertigsorten")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste fehlt".to_owned(),
        })?;
    // Back up to the enclosing tag start: cutting inside `<h2>` would
    // hide the first rubric from the selector below.
    let start = html[..found].rfind('<').unwrap_or(found);
    let tail = &html[start..];
    let end = tail
        .find("Sie haben Fragen")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste ohne Ende".to_owned(),
        })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(window);
    let sel = Selector::parse("h2, p, div.toggler").expect("valid selector");
    let mut out = Vec::new();
    for el in doc.select(&sel) {
        // Join with spaces: scraper text() glues adjacent nodes
        // ("1"+"E-Mail:"), which would corrupt phone/email reads.
        let text: String = el.text().collect::<Vec<_>>().join(" ");
        let t = text.split_whitespace().collect::<Vec<_>>().join(" ");
        match el.value().name() {
            // Rubriken und Sortenlabels; Prosa (>120 Zeichen) ist kein Label.
            "h2" | "div" if !t.is_empty() && t.len() <= 120 => out.push(t),
            "p" => {
                if let Some(neg) = negative_label(&t) {
                    out.push(neg);
                }
            }
            _ => {}
        }
    }
    if out.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    Ok(out)
}

/// "Elektronikschrott wird von uns nicht angenommen! Dazu gehören:" →
/// `Some("Elektronikschrott (keine Annahme)")`. Sonst `None`.
fn negative_label(text: &str) -> Option<String> {
    let i = text.find("von uns nicht angenommen")?;
    let prefix = text[..i].trim_end();
    let mut words: Vec<&str> = prefix.split_whitespace().collect();
    if let Some(last) = words.last() {
        if *last == "wird" || *last == "werden" {
            words.pop();
        }
    }
    let subject = words.join(" ");
    if subject.is_empty() || subject.len() > 40 {
        return None;
    }
    Some(format!("{subject} (keine Annahme)"))
}

/// Bespoke Zwickau contact: the impressum company block after
/// `<h1>Impressum</h1>` ("Hofmann Metall GmbH / Äußere Dresdner
/// Straße 80 / 08066 Zwickau") plus the labeled Telefon/E-Mail lines.
/// Missing anchors mean the page changed shape → loud error.
fn extract_info_zwickau(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let anchor = doc
        .select(&h1)
        .find(|h| h.text().collect::<String>().contains("Impressum"));
    let Some(_) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    };
    let p = Selector::parse("p").expect("valid selector");
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    let (mut phone, mut email) = (String::new(), String::new());
    for el in doc.select(&p) {
        let lines: Vec<String> = el
            .inner_html()
            .split("<br")
            .map(strip_fragment)
            .filter(|s| !s.is_empty())
            .collect();
        if lines.iter().any(|l| l == "Hofmann Metall GmbH") && street.is_empty() {
            // Exact firm line (HQ block) — footer Standort blocks read
            // "Hofmann Metall GmbH Betriebsteil X" and must never win;
            // first match wins, later blocks don't overwrite.
            if let Some(addr) = firm_block(&lines) {
                street = addr.0;
                postcode = addr.1;
                city = addr.2;
            }
        }
        for line in &lines {
            if phone.is_empty() {
                if let Some(rest) = line.split_once("Telefon:") {
                    phone = phone_tokens(rest.1);
                }
            }
            if email.is_empty() {
                if let Some(rest) = line.split_once("E-Mail:") {
                    email = rest
                        .1
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
                        .to_owned();
                }
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

/// Bespoke Chemnitz contact: the `<p>` siblings after
/// `<h3>KONTAKT & ÖFFNUNGSZEITEN</h3>` on the Betriebsteil page
/// (address lines, then the Büro/E-Mail row with entity-encoded mail).
/// Missing anchors mean the page changed shape → loud error.
fn extract_info_chemnitz(html: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(html);
    let h3 = Selector::parse("h3").expect("valid selector");
    let anchor = doc.select(&h3).find(|h| {
        h.text()
            .collect::<String>()
            .to_lowercase()
            .contains("kontakt")
    });
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: CHEMNITZ_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let paras: Vec<ElementRef> = anchor
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .filter(|e| e.value().name() == "p")
        .take(2)
        .collect();
    let Some(addr_p) = paras.first() else {
        return Err(IngestError::Parse {
            url: CHEMNITZ_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let lines: Vec<String> = addr_p
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if let Some(addr) = firm_block(&lines) {
        street = addr.0;
        postcode = addr.1;
        city = addr.2;
    }
    let (mut phone, mut email) = (String::new(), String::new());
    if let Some(contact_p) = paras.get(1) {
        // Entities (&#99;…) are already decoded by html5ever here; join
        // with spaces so "1" and "E-Mail:" don't glue into one token.
        let joined: String = contact_p.text().collect::<Vec<_>>().join(" ");
        let flat = joined.split_whitespace().collect::<Vec<_>>().join(" ");
        if let Some(i) = flat.find("Büro:") {
            phone = phone_tokens(&flat[i + "Büro:".len()..]);
        }
        email = flat
            .split_whitespace()
            .find(|t| t.contains('@') && t.contains('.'))
            .unwrap_or_default()
            .to_owned();
    }
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: CHEMNITZ_URL.to_owned(),
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

/// ["Hofmann Metall GmbH", "Äußere Dresdner Straße 80", "08066 Zwickau"]
/// → (street, postcode, city). The firm line itself never wins.
fn firm_block(lines: &[String]) -> Option<(String, String, String)> {
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), rest) = (it.next(), it.collect::<Vec<_>>().join(" ")) {
            if pc.len() == 5
                && pc.chars().all(|c| c.is_ascii_digit())
                && rest.chars().next().is_some_and(|c| c.is_uppercase())
                && k > 0
            {
                let city = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .to_owned();
                return Some((lines[k - 1].clone(), pc.to_owned(), city));
            }
        }
    }
    None
}

/// Phone-style token run ("0375 27 13 46 0", stops at letters).
fn phone_tokens(s: &str) -> String {
    s.split_whitespace()
        .take_while(|t| {
            t.chars()
                .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant (` class="…"` from `<br />` splits) — drop everything up to
/// the first '>' first, or attributes parse as text.
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
    use super::{extract_info_chemnitz, extract_info_zwickau, grade_for, negative_label, parse};

    // Real excerpt shapes of the live page (28.09.2026): `<br />`
    // inside rubrics, `toggler` sort labels, `<strong>`-wrapped
    // non-acceptance paragraphs.
    const FIXTURE: &str = "<h2>Fertigsorten<br />Stahlschrott</h2>\
        <div class=\"toggler\"> Sorte 2 Schwerer Stahl-Neu-Schrott </div>\
        <div class=\"toggler\"> Sorte 5 Stahlspäne </div>\
        <h2 class=\"ce_headline\"> Vormaterial<br />Stahlschrott</h2>\
        <div class=\"toggler\"> Autokarosse </div>\
        <div class=\"ce_text block\"><p class=\"p1\"><strong>Autokarossen werden von uns nicht angenommen!</strong></p></div>\
        <h2 class=\"ce_headline\"> Kupfer - Schrott</h2>\
        <div class=\"toggler\"> Cu-Draht neu, blank, Millberry </div>\
        <div class=\"toggler\"> Cu-Ms Kühler </div>\
        <h2>Sie haben Fragen oder wollen<br>einen Termin vereinbaren?</h2>";

    #[test]
    fn rubrics_togglers_and_negatives_parse() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(
            labels,
            vec![
                "Fertigsorten Stahlschrott",
                "Sorte 2 Schwerer Stahl-Neu-Schrott",
                "Sorte 5 Stahlspäne",
                "Vormaterial Stahlschrott",
                "Autokarosse",
                "Autokarossen (keine Annahme)",
                "Kupfer - Schrott",
                "Cu-Draht neu, blank, Millberry",
                "Cu-Ms Kühler",
            ]
        );
        assert!(parse("<h1>Ohne Liste</h1>").is_err());
        assert!(parse("<h2>Fertigsorten<br />Stahlschrott</h2><p>x</p>").is_err());
    }

    #[test]
    fn negatives_detected() {
        assert_eq!(
            negative_label("Elektronikschrott wird von uns nicht angenommen! Dazu gehören:"),
            Some("Elektronikschrott (keine Annahme)".to_owned())
        );
        assert_eq!(
            negative_label("Autokarossen werden von uns nicht angenommen!"),
            Some("Autokarossen (keine Annahme)".to_owned())
        );
        assert_eq!(
            negative_label("Weiße Ware wird von uns nicht angenommen! Dazu gehört:"),
            Some("Weiße Ware (keine Annahme)".to_owned())
        );
        assert_eq!(
            negative_label("Nach dem Auswiegen erfolgt der Transport."),
            None
        );
    }

    #[test]
    fn mapping_covers_every_live_label() {
        // Every `toggler` + H2 of the live page (28.09.2026), each
        // explicitly accepted or loudly skipped — no fuzzy fallthrough.
        let mapped: &[(&str, &[(&str, &str)])] = &[
            (
                "Sorte 2 Schwerer Stahl-Neu-Schrott",
                &[("stahlschrott-sorte-1", "")],
            ),
            (
                "Sorte 8 Leichter Stahl-Neu-Schrott",
                &[("stahlschrott-sorte-1", "")],
            ),
            ("Sorte 4 Schredderschrott", &[("stahlschrott-shredder", "")]),
            ("Mischschrott", &[("mischschrott", "")]),
            ("Scherenvormaterial / Träger", &[("mischschrott", "")]),
            ("Scherenvormaterial, leicht", &[("mischschrott", "")]),
            ("Scherenvormaterial, schwer", &[("mischschrott", "")]),
            ("Bremsscheiben", &[("eisenschrott-gussbruch", "")]),
            ("Gussspäne", &[("eisenschrott-gussbruch", "Späne")]),
            (
                "Handelsguss 3b, nicht chargierfähig",
                &[("eisenschrott-gussbruch", "")],
            ),
            (
                "Handelsgussbruch 3a, chargierfähig",
                &[("eisenschrott-gussbruch", "")],
            ),
            (
                "Maschinenguss 2b, nicht chargierfähig",
                &[("eisenschrott-gussbruch", "")],
            ),
            (
                "Maschinengussbruch 2a, chargierfähig",
                &[("eisenschrott-gussbruch", "")],
            ),
            (
                "Elektromotor - Aluminium-Wicklung",
                &[("elektromotoren", "")],
            ),
            ("Elektromotor - Kupfer-Wicklung", &[("elektromotoren", "")]),
            ("Verbrennungsmotor - Aluminium", &[("aluminium-guss", "")]),
            (
                "Verbrennungsmotor - Stahl",
                &[("eisenschrott-gussbruch", "")],
            ),
            ("Hartmetall", &[("hartmetall", "")]),
            ("VA-Schrott, kleinstückig", &[("edelstahl-gemischt", "")]),
            (
                "VA-Schrott, nicht chargierfähig",
                &[("edelstahl-gemischt", "")],
            ),
            ("VA-Späne", &[("edelstahl-gemischt", "Späne")]),
            ("Al-Kabel", &[("kabel-alu", "")]),
            ("Al-Schälkabel", &[("kabel-alu", "")]),
            ("Cu-Kabel", &[("kabel-kupfer", "")]),
            ("Cu-Kabel mit Stecker", &[("kabel-kupfer", "")]),
            ("Cu-Pb Kabel", &[("kabel-kupfer", "")]),
            ("Cu-Schälkabel", &[("kabel-kupfer", "")]),
            (
                "Cu-Draht neu, blank, Millberry",
                &[("kupfer-millberry", "")],
            ),
            ("Cu-Draht, legiert, Berry", &[("kupfer-berry", "")]),
            ("Cu-Leitschienen, blank", &[("kupfer-gemischt", "")]),
            ("Cu-Raff 95% Cu", &[("kupfer-gemischt", "")]),
            ("Cu-Schwer", &[("kupfer-gemischt", "")]),
            ("Cu-Schwer, verzinnt", &[("kupfer-gemischt", "")]),
            ("Messing-Hülsen", &[("messing", "")]),
            ("Ms schwer", &[("messing", "")]),
            ("Ms-58-Schrott", &[("messing", "")]),
            ("Ms-Raff-Material", &[("messing", "")]),
            ("Messing-Späne", &[("messing", "Späne")]),
            (
                "Al-Blech alt / neu, ohne Anhaftung",
                &[("aluminium-blech", "")],
            ),
            ("Al-Blech, max. 5% Fe", &[("aluminium-blech", "")]),
            ("Al-Felgen", &[("aluminium-gemischt", "")]),
            ("Al-Guss alt, ohne Fe", &[("aluminium-guss", "")]),
            ("Al-Profile, lackiert", &[("aluminium-profile", "")]),
            ("Al-Profile, Si 0,5 blank", &[("aluminium-profile", "")]),
            ("Zink, legiert, sauber", &[("zink", "")]),
            ("Zink-Bleche bis 5% Anhaftung", &[("zink", "")]),
            ("Zink-Bleche neu", &[("zink", "")]),
            ("Zink-Bleche sauber", &[("zink", "")]),
            ("Zink-Druck-Guss", &[("zink", "")]),
            ("Zink-Schrott", &[("zink", "")]),
            ("Auswuchtblei", &[("blei", "Auswucht")]),
            ("Zerlegematerial", &[("mischschrott", "")]),
            // H2 fallbacks (generic → generic).
            ("Guss - Schrott", &[("eisenschrott-gussbruch", "")]),
            ("Gemischte Metalle", &[("elektromotoren", "")]),
            (
                "legierter Schrott",
                &[("edelstahl-gemischt", ""), ("hartmetall", "")],
            ),
            (
                "Kabel - Schrott",
                &[("kabel-kupfer", ""), ("kabel-alu", "")],
            ),
            ("Kupfer - Schrott", &[("kupfer-gemischt", "")]),
            ("Messing - Schrott", &[("messing", "")]),
            ("Aluminium - Schrott", &[("aluminium-gemischt", "")]),
            ("Zink - Schrott", &[("zink", "")]),
            (
                "Sonstiges",
                &[("blei", "Auswucht"), ("mischschrott", "Stahl/Bunt-Mix")],
            ),
        ];
        for (label, want) in mapped {
            assert_eq!(&grade_for(label).unwrap_or_default(), want, "{label}");
        }
        // Loud skips: sorted Alt grades without preparation proof,
        // carbon chips, rebar, mixed Cu-Ms, chrome, paper, lead-acid
        // batteries, proven non-acceptances (toggler + <p> forms) and the
        // unmappable H2 plurals.
        let skipped = [
            "Sorte 1 Leichter Stahl-Alt-Schrott",
            "Sorte 3 Schwerer Stahl-Alt-Schrott",
            "Sorte 5 Stahlspäne",
            "Armierungseisen",
            "Autokarosse",
            "Autokarossen (keine Annahme)",
            "Elektronikschrott",
            "Elektronikschrott (keine Annahme)",
            "Weiße Ware",
            "Weiße Ware (keine Annahme)",
            "Katalysator",
            "Katalysatoren (keine Annahme)",
            "Chromschrott",
            "Chromspäne",
            "Cu-Ms Kühler",
            "Altpapier",
            "Blei-Akkus, mit Säure",
            "Blei-Akkus, ohne Säure",
            "Fertigsorten Stahlschrott",
            "Vormaterial Stahlschrott",
            "Irgendwas Neues",
        ];
        for label in skipped {
            assert_eq!(grade_for(label), None, "{label}");
        }
    }

    #[test]
    fn impressum_extracts_zwickau_hq() {
        // Real fragment shape of the live impressum (28.09.2026).
        let imp = "<h1>Impressum</h1>\
            <p><strong>Hofmann Metall GmbH</strong><br>Äußere Dresdner Straße 80<br>08066 Zwickau</p>\
            <p>Telefon: 0375 27 13 46 0<br>Fax: 0375 27 13 46 12<br>E-Mail: info@hofmann-metall.de<br>Web: www.hofmann-metall.de</p>";
        let info = extract_info_zwickau(imp).expect("parses");
        assert_eq!(info.street, "Äußere Dresdner Straße 80");
        assert_eq!(info.postcode, "08066");
        assert_eq!(info.city, "Zwickau");
        assert_eq!(info.phone, "0375 27 13 46 0");
        assert_eq!(info.email, "info@hofmann-metall.de");
        assert!(extract_info_zwickau("<h1>Neu hier</h1>").is_err());
    }

    #[test]
    fn impressum_footer_standorte_never_win() {
        // The live impressum embeds a footer with ALL Betriebsteile;
        // the HQ block (exact firm line, first) must win over e.g.
        // "Hofmann Metall GmbH Betriebsteil Elstertrebnitz".
        let imp = "<h1>Impressum</h1>\
            <p><strong>Hofmann Metall GmbH</strong><br>Äußere Dresdner Straße 80<br>08066 Zwickau</p>\
            <p>Telefon: 0375 27 13 46 0<br>E-Mail: info@hofmann-metall.de</p>\
            <h4>Elstertrebnitz</h4>\
            <p>Hofmann Metall GmbH Betriebsteil Elstertrebnitz<br>B 10<br>04523 Elstertrebnitz<br>Büro: 034296 / 49 59 0</p>";
        let info = extract_info_zwickau(imp).expect("parses");
        assert_eq!(info.street, "Äußere Dresdner Straße 80");
        assert_eq!(info.postcode, "08066");
        assert_eq!(info.city, "Zwickau");
    }

    #[test]
    fn standort_extracts_chemnitz_with_entity_mail() {
        // Real fragment shape of the live Standort page (28.09.2026),
        // entity-encoded mail kept verbatim.
        let html = "<h3>KONTAKT & ÖFFNUNGSZEITEN</h3>\
            <p>Blankenburgstraße 104<br>09114 Chemnitz</p>\
            <p><span class=\"tab\">Büro:</span> 0371 / 33 49 79 1<br>\
            <span class=\"tab\">E-Mail: </span> &#99;&#x68;&#101;&#x6D;&#110;&#x69;&#116;&#x7A;&#64;&#x68;&#111;&#x66;&#109;&#x61;&#110;&#x6E;&#45;&#x6D;&#101;&#x74;&#97;&#x6C;&#108;&#x2E;&#100;&#x65;</p>";
        let info = extract_info_chemnitz(html).expect("parses");
        assert_eq!(info.street, "Blankenburgstraße 104");
        assert_eq!(info.postcode, "09114");
        assert_eq!(info.city, "Chemnitz");
        assert_eq!(info.phone, "0371 / 33 49 79 1");
        assert_eq!(info.email, "chemnitz@hofmann-metall.de");
        assert!(extract_info_chemnitz("<h3>Anders</h3>").is_err());
    }
}
