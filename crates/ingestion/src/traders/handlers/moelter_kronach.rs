//! Mölter GmbH (Kronach-Neuses): acceptance list WITHOUT prices.
//!
//! The only price publication is a PDF (`/media/preise.pdf`, "Download
//! Liste Tagespreise", live, ~65 kB) — the workspace has no PDF-text
//! dependency and none is added, so PDF-only price lists stay out of
//! reach (provenance: no static HTML). This handler therefore records
//! NO prices and fills `trader_materials` from the static "Schrott+Metall"
//! teaser paragraph on the homepage ("kauft alle Schrottsorten wie …,
//! täglich ankaufen") plus contact enrichment from the impressum page.
//! Zero prices with resolved acceptances is normal operation, not a
//! canary trip. Should a static HTML price table ever appear, this
//! handler wants a price parse on top.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "by-kronach-neuses-molter";
/// Bespoke, live-verified impressum URL (site footer nav's own
/// "Impressum" link). A move fails the step loudly (fix the URL) —
/// never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.moelter-kronach.de/impressum/";

/// Homepage: the "Schrott+Metall" teaser carries the only static
/// purchase categories; the "Preise" section links only to the PDF.
pub const URL: &str = "https://www.moelter-kronach.de/";

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
            // Batteries have no catalog material (altmittweida/frisch
            // precedent); the proposal lives in the skip, not in a guess.
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

/// Explicit grade → acceptances. Generic iron scrap (steel, chips,
/// shredder feedstock, car bodies, beams) lands on the generic
/// `mischschrott` bucket (esh/madi/frisch precedent); cast grades ride
/// as `eisenschrott-gussbruch` (altmittweida "guss" precedent); bare
/// NE metals land on their generic materials, never a specific sort.
/// Lead-acid batteries stay `None`: no `starterbatterien` material in
/// the catalog (proposal in the skip label).
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("batterie") || l.contains("akku") {
        None
    } else if l.contains("alu") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("hartmetall") {
        Some(vec![("hartmetall", "")])
    } else if l.contains("blei") {
        Some(vec![("blei", "")])
    } else if l.contains("guss") || l.contains("gießerei") || l.contains("giesserei") {
        Some(vec![("eisenschrott-gussbruch", "")])
    } else if l.contains("stahlschrott")
        || l.contains("stahlspäne")
        || l.contains("stahlspaene")
        || l.contains("schredder")
        || l.contains("kaross")
        || l.contains("träger")
        || l.contains("traeger")
    {
        Some(vec![("mischschrott", "")])
    } else {
        None
    }
}

/// Loud reason for every unmapped grade — new-material proposals live
/// here, not in a silent default.
fn skip_reason(label: &str) -> &'static str {
    let l = label.to_lowercase();
    if l.contains("batterie") || l.contains("akku") {
        " (Bleibatterien, kein Katalogmaterial — Vorschlag: starterbatterien)"
    } else {
        " (unbekannte Sorte, kein Katalogmaterial)"
    }
}

/// Grades from the "Schrott+Metall" teaser between its purchase sentence
/// and the NE-list terminator. The paragraph is prose, not a list, so
/// sentences split on '.' first (the "… Metallabfälle an. Aluminium …"
/// boundary would otherwise glue Aluminium onto a filler chunk), then
/// grades split on ',' / 'und' / 'als auch'. Sentence scaffolding
/// ("kauft alle Schrottsorten", "alle anderen Metallabfälle") is
/// filtered as filler — documented here, not counted as skips. Missing
/// anchors or 0 grades → `Err`, never an empty success.
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let start = html
        .find("kauft alle Schrottsorten")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankaufabsatz fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("ist eine kleine Auswahl")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "NE-Listenende fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    // Strip tags (each tag becomes a newline so words never glue), decode
    // the entities this paragraph uses, normalize whitespace.
    let mut text = String::with_capacity(window.len() / 2);
    let mut in_tag = false;
    for c in window.chars() {
        if c == '<' {
            in_tag = true;
            text.push('\n');
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            text.push(c);
        }
    }
    let text = decode_entities(&text);
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut out = Vec::new();
    for sentence in text.split(['.', ';']) {
        for chunk in sentence.split(',') {
            // One grade per chunk: conjunctions separate grades inside a
            // chunk ("… Maschinenguss und alle anderen …", "Hartmetall
            // als auch Bleibatterien …", "… wie Stahlschrott").
            let norm = format!(" {chunk} ")
                .replace(" als auch ", ",")
                .replace(" sowie ", ",")
                .replace(" und ", ",")
                .replace(" wie ", ",");
            for raw in norm.split(',') {
                let t = raw.split_whitespace().collect::<Vec<_>>().join(" ");
                if t.is_empty() || t.len() > 60 {
                    continue;
                }
                let fl = t.to_lowercase();
                // Sentence scaffolding, not grades.
                if fl.contains("anderen")
                    || fl.contains("mölter")
                    || fl.contains("moelter")
                    || fl.contains("schrottsorten")
                {
                    continue;
                }
                out.push(t);
            }
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

/// The few entities this paragraph uses (html5ever is not involved —
/// tags are stripped manually above).
fn decode_entities(s: &str) -> String {
    s.replace("&auml;", "ä")
        .replace("&ouml;", "ö")
        .replace("&uuml;", "ü")
        .replace("&Auml;", "Ä")
        .replace("&Ouml;", "Ö")
        .replace("&Uuml;", "Ü")
        .replace("&szlig;", "ß")
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

/// Bespoke contact extraction for THIS impressum only: `<h1>Impressum</h1>`
/// followed by content `<p>` blocks ("Hohe Weide 5<br>96317
/// Kronach-Neuses", "Tel: 09261/3418<br>Fax: …", bare
/// "info@moelter-kronach.de"). Address lines come from the `<br>`-split
/// of the PLZ-carrying `<p>` (street = previous line); the phone rides
/// its "Tel:" line; the e-mail prefers the mailto-href and falls back
/// to the bare-address paragraph. Missing anchors → loud error, never
/// a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    if !doc
        .select(&h1)
        .any(|h| h.text().collect::<String>().trim() == "Impressum")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let p = Selector::parse("p").expect("valid selector");
    let a = Selector::parse("a").expect("valid selector");
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    let mut phone = String::new();
    let mut bare_mail = String::new();
    for el in doc.select(&p) {
        let lines: Vec<String> = el
            .inner_html()
            .split("<br")
            .map(strip_fragment)
            .filter(|s| !s.is_empty())
            .collect();
        for (k, line) in lines.iter().enumerate() {
            // PLZ line: optional "D-" country prefix, then city.
            let bare = line.replace("D-", "");
            let mut it = bare.split_whitespace();
            if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                if pc.len() == 5
                    && pc.chars().all(|c| c.is_ascii_digit())
                    && ci.chars().next().is_some_and(|c| c.is_uppercase())
                    && postcode.is_empty()
                {
                    postcode = pc.to_owned();
                    city = it.fold(ci.to_owned(), |acc, w| format!("{acc} {w}"));
                    if k > 0 {
                        street = lines[k - 1].clone();
                    }
                }
            }
            if phone.is_empty() {
                if let Some(v) = line.strip_prefix("Tel:") {
                    phone = v.trim().to_owned();
                }
            }
        }
        // Bare-address paragraph fallback (this page's mail `<p>` holds
        // only the address — single-token check keeps glued neighbours
        // out).
        if bare_mail.is_empty() {
            let t: String = el.text().collect();
            let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
            if t.contains('@') && !t.contains(' ') {
                bare_mail = t;
            }
        }
    }
    // E-mail per mailto-href first, never per token split of glued text.
    let mut email = String::new();
    for el in doc.select(&a) {
        if let Some(href) = el.value().attr("href") {
            if let Some(addr) = href.strip_prefix("mailto:") {
                let addr = addr.split('?').next().unwrap_or_default().trim().to_owned();
                if addr.contains('@') {
                    email = addr;
                    break;
                }
            }
        }
    }
    if email.is_empty() {
        email = bare_mail;
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

/// Strip tags from a `<br>`-split fragment. Fragments start with a tag
/// remnant — drop everything up to the first '>' first, or attributes
/// parse as text.
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
    use super::{decode_entities, grade_for, parse, skip_reason};

    // Real shape of the live "Schrott+Metall" teaser (entities intact),
    // trimmed after the NE-list terminator.
    const FIXTURE: &str = "<h3>Schrott+Metall</h3>\
        <p>Die M&ouml;lter GmbH kauft alle Schrottsorten wie Stahlschrott, Stahlsp&auml;ne, \
        Schreddervormaterial, Karossen, Tr&auml;gerschrott, Gie&szlig;ereiabf&auml;lle, \
        Maschinenguss und alle anderen Metallabf&auml;lle an. Aluminium, Kupfer, Messing, Blei, \
        Zink, Hartmetall als auch Bleibatterien aus Fahrzeugen ist eine kleine Auswahl der \
        NE-Metallsorten, die wir t&auml;glich ankaufen.</p>";

    #[test]
    fn teaser_window_and_mapping() {
        assert_eq!(decode_entities("Tr&auml;ger"), "Träger");
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(
            labels.iter().map(String::as_str).collect::<Vec<_>>(),
            vec![
                "Stahlschrott",
                "Stahlspäne",
                "Schreddervormaterial",
                "Karossen",
                "Trägerschrott",
                "Gießereiabfälle",
                "Maschinenguss",
                "Aluminium",
                "Kupfer",
                "Messing",
                "Blei",
                "Zink",
                "Hartmetall",
                "Bleibatterien aus Fahrzeugen",
            ]
        );
        // Generic iron → generic bucket; cast grades → gussbruch.
        assert_eq!(grade_for("Stahlschrott"), Some(vec![("mischschrott", "")]));
        assert_eq!(grade_for("Stahlspäne"), Some(vec![("mischschrott", "")]));
        assert_eq!(
            grade_for("Schreddervormaterial"),
            Some(vec![("mischschrott", "")])
        );
        assert_eq!(grade_for("Karossen"), Some(vec![("mischschrott", "")]));
        assert_eq!(grade_for("Trägerschrott"), Some(vec![("mischschrott", "")]));
        assert_eq!(
            grade_for("Gießereiabfälle"),
            Some(vec![("eisenschrott-gussbruch", "")])
        );
        assert_eq!(
            grade_for("Maschinenguss"),
            Some(vec![("eisenschrott-gussbruch", "")])
        );
        // Bare NE metals → generic materials, never a specific sort.
        assert_eq!(
            grade_for("Aluminium"),
            Some(vec![("aluminium-gemischt", "")])
        );
        assert_eq!(grade_for("Kupfer"), Some(vec![("kupfer-gemischt", "")]));
        assert_eq!(grade_for("Messing"), Some(vec![("messing", "")]));
        assert_eq!(grade_for("Blei"), Some(vec![("blei", "")]));
        assert_eq!(grade_for("Zink"), Some(vec![("zink", "")]));
        assert_eq!(grade_for("Hartmetall"), Some(vec![("hartmetall", "")]));
        // Batteries hold blei (altmittweida/frisch precedent).
        assert_eq!(grade_for("Bleibatterien aus Fahrzeugen"), None);
        assert!(skip_reason("Bleibatterien aus Fahrzeugen").contains("starterbatterien"));
        assert!(parse("<div>Redesign ohne Absatz</div>").is_err());
        assert!(parse("<p>kauft alle Schrottsorten ohne Ende</p>").is_err());
    }

    #[test]
    fn impressum_addres_tel_and_mail() {
        // Real shape of the live impressum content blocks plus the
        // footer mailto (href-first e-mail rule).
        let imp = "<h1>Impressum</h1>\
            <p><strong>Verantwortlich:</strong></p><p>M&ouml;lter GmbH</p>\
            <p>Gesch&auml;ftsf&uuml;hrer:<br>Frank M&ouml;lter, Jochen M&ouml;lter</p>\
            <p>Hohe Weide 5<br>96317 Kronach-Neuses</p>\
            <p>Tel: 09261/3418<br>Fax: 09261/94778</p>\
            <p>info@moelter-kronach.de</p>\
            <p><a href=\"mailto:info@moelter-kronach.de\">Mail</a></p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Hohe Weide 5");
        assert_eq!(info.postcode, "96317");
        assert_eq!(info.city, "Kronach-Neuses");
        assert_eq!(info.phone, "09261/3418");
        assert_eq!(info.email, "info@moelter-kronach.de");
        // Bare-address fallback when no mailto exists.
        let bare = "<h1>Impressum</h1><p>Hohe Weide 5<br>96317 Kronach-Neuses</p>\
            <p>Tel: 09261/3418</p><p>info@moelter-kronach.de</p>";
        assert_eq!(
            super::extract_info(bare).expect("parses").email,
            "info@moelter-kronach.de"
        );
        assert!(super::extract_info("<h1>Neu hier</h1>").is_err());
    }
}
