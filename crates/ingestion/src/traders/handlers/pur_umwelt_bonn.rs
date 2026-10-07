//! PUR Umwelt — P. U. Richter Umweltdienste Rheinland GmbH (Bonn):
//! the dated price list lives in the news-ticker `<a>` ("ALTMETALLPREISE
//! Stand 21.09.2026 …", NE metals in €/kg, FE scrap in €/to, terminated
//! by "Keine Annahme von: …"). The pricing-table cards further down are
//! an undated visual subset quoted in €/t and are deliberately NOT
//! parsed (no page date, would duplicate the ticker rows).
//! Segments split on `|`; the price is the last number token before
//! `€` (needed: "Misch./Privat ab 100 kg 135,-€/to" leads with a
//! threshold number). Category prefixes ("FE-Schrott:", "NE-Metalle:")
//! are stripped explicitly. "Eisenspäne" (no FE-späne catalog entry)
//! and "Verhüttung" (smelting fraction, no catalog entry) skip loudly.
//! "Hartmetall/Widia" maps to `hartmetall`: Widia IS hard metal (the
//! catalog entry reads "Hartmetall / VHM / Widia"), not an ambiguous
//! either/or label.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-bonn-pur-umwelt-p-u-richter";
/// Bespoke, live-verified price URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://www.pur-umwelt.com/schrottpreise/";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.pur-umwelt.com/impressum";

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
    let (published_at, rows, mut skipped_labels) = parse(&html)?;
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
        byte_len: html.len(),
        published_at,
    })
}

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific-before-generic: `alu` arms precede the `fe-guss`
/// arm ("Alu-Guss ohne Fe" must not land on iron), `akku` precedes
/// bare `blei`, `millberry` precedes every copper arm.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("alu") && l.contains("kabel") {
        Some(("kabel-alu", ""))
    } else if l.contains("alu") && l.contains("guss") {
        Some(("aluminium-guss", "ohne Fe"))
    } else if l.contains("alu") && (l.contains("späne") || l.contains("spaene")) {
        Some(("aluminium-gemisch", "Späne"))
    } else if l.contains("alu") && (l.contains("iso") || l.contains("profil")) {
        Some(("aluminium-profile", "Iso"))
    } else if l.contains("alu") && l.contains("felg") {
        Some(("aluminium-guss", "Felgen unsauber"))
    } else if l.contains("alu") && l.contains("blech") {
        Some(("aluminium-blech", "rein alt"))
    } else if l.contains("alu") && l.contains("geschirr") {
        Some(("aluminium-blech", "Geschirr"))
    } else if l.contains("alu") {
        Some(("aluminium-gemisch", ""))
    } else if l.contains("e-motor") {
        if l.contains("klein") {
            Some(("elektromotoren", "klein"))
        } else {
            Some(("elektromotoren", ""))
        }
    } else if l.contains("akku") {
        Some(("blei", "Akku"))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("kupfer") && (l.contains("späne") || l.contains("spaene")) {
        Some(("kupfer-gemischt", "Späne"))
    } else if l.contains("raff") {
        Some(("kupfer-gemischt", "Raff 95%"))
    } else if l.contains("kabel") && l.contains("ohne stecker") {
        Some(("kabel-kupfer", "ohne Stecker"))
    } else if l.contains("kabel") && l.contains("mit stecker") {
        Some(("kabel-kupfer", "mit Stecker"))
    } else if l.contains("kabel") {
        Some(("kabel-kupfer", ""))
    } else if l.contains("messing") && (l.contains("späne") || l.contains("spaene")) {
        Some(("messing", "Späne"))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("misch") {
        if l.contains("privat") {
            Some(("mischschrott", "Privat ab 100 kg"))
        } else if l.contains("händler") || l.contains("haendler") {
            Some(("mischschrott", "Händler"))
        } else {
            Some(("mischschrott", ""))
        }
    } else if l.contains("bremsscheib") {
        Some(("eisenschrott-gussbruch", "Bremsscheiben"))
    } else if l.contains("fe-guss") || (l.contains("guss") && l.contains("fe")) {
        Some(("eisenschrott-gussbruch", ""))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", ""))
    } else if l.contains("v2a") && (l.contains("späne") || l.contains("spaene")) {
        Some(("edelstahl-v2a", "Späne"))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("zinn") {
        if l.contains("geschirr") {
            Some(("zinn-geschirr", "Geschirr"))
        } else {
            Some(("zinn", ""))
        }
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("hartmetall") || l.contains("widia") {
        Some(("hartmetall", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<h1>` must
/// read "Impressum", the `<p>` after the "Angaben gemäß § 5 TMG"
/// heading holds firm lines + street + PLZ city, and the `<p>` after
/// the "Kontakt" heading holds labeled Telefon/E-Mail lines. Missing
/// anchors mean the page changed shape → loud error, never a guessed
/// fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let h2 = Selector::parse("h2").expect("valid selector");
    let anchor = doc
        .select(&h1)
        .find(|h| h.text().collect::<String>().trim() == "Impressum");
    let Some(_) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Anker fehlt".to_owned(),
        });
    };
    let heading_p = |title: &str| {
        doc.select(&h2)
            .find(|h| h.text().collect::<String>().trim() == title)
            .and_then(next_p)
    };
    // Firm lines + street + "53175 Bonn" (last two lines).
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if let Some(p) = heading_p("Angaben gemäß § 5 TMG") {
        let lines = br_lines(p);
        if lines.len() >= 2 {
            let last = lines.last().expect("len checked");
            let mut it = last.split_whitespace();
            if let Some(pc) = it.next() {
                if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    city = it.collect::<Vec<_>>().join(" ");
                    street = lines[lines.len() - 2].clone();
                }
            }
        }
    }
    // Labeled contact lines ("Telefon: …", "E-Mail: …").
    let (mut phone, mut email) = (String::new(), String::new());
    if let Some(p) = heading_p("Kontakt") {
        for line in br_lines(p) {
            let lower = line.to_lowercase();
            if lower.starts_with("telefon:") && phone.is_empty() {
                phone = line["Telefon:".len()..].trim().to_owned();
            } else if (lower.starts_with("e-mail:") || lower.starts_with("email:"))
                && email.is_empty()
            {
                let addr = line.splitn(2, ':').nth(1).unwrap_or("").trim().to_owned();
                if addr.contains('@') {
                    email = addr;
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

/// First `<p>` sibling after a heading element.
fn next_p(el: ElementRef) -> Option<ElementRef> {
    el.next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p")
}

/// `<br>`-separated lines of a `<p>` (tags stripped; entities are
/// already decoded by html5ever).
fn br_lines(p: ElementRef) -> Vec<String> {
    let mut lines = Vec::new();
    for part in p.inner_html().split("<br") {
        let t = strip_tags(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    lines
}

/// Strip tags from a fragment.
fn strip_tags(s: &str) -> String {
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

fn parse(
    html: &str,
) -> Result<
    (
        Option<String>,
        Vec<(String, f64, &'static str)>,
        Vec<String>,
    ),
    IngestError,
> {
    let doc = Html::parse_document(html);
    let anchor_sel = Selector::parse("a").expect("valid selector");
    let h3 = Selector::parse("h3").expect("valid selector");
    // Never trust page order: take the ticker anchor carrying the dated
    // list, not just the first <a> on the page.
    let ticker = doc
        .select(&anchor_sel)
        .find(|a| a.text().collect::<String>().contains("ALTMETALLPREISE"));
    let Some(ticker) = ticker else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "kein Preisticker".to_owned(),
        });
    };
    let full: String = ticker.text().collect();
    let full = full.split_whitespace().collect::<Vec<_>>().join(" ");
    // Window: start at the list head, end at the exclusion terminator
    // ("Keine Annahme von: …" is not a price, it ends the box).
    let start = full.find("ALTMETALLPREISE").expect("anchor checked");
    let end = full.find("Keine Annahme von:").unwrap_or(full.len());
    let body = &full[start..end];
    // Page date: the ticker "Stand …" first, the "gültig ab: …" heading
    // as fallback (both live: 21.09.2026).
    let mut published_at = de_date_after(body, "Stand");
    if published_at.is_none() {
        published_at = doc
            .select(&h3)
            .find(|h| {
                h.text()
                    .collect::<String>()
                    .to_lowercase()
                    .contains("gültig ab")
            })
            .and_then(|h| {
                let t: String = h.text().collect();
                de_date_after(&t, "gültig ab")
            });
    }
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for seg in body.split('|') {
        let seg = seg.trim();
        if seg.is_empty() {
            continue;
        }
        let seg = strip_category(seg);
        let Some((label, price, unit_frag)) = split_pair(seg) else {
            skipped.push(format!("{seg} (kein Preis)"));
            continue;
        };
        // Prose is not a label.
        if label.chars().count() > 120 {
            skipped.push(format!("{label} (Prosa, kein Preis)"));
            continue;
        }
        // A "0,00" row is "no quote", not a free gift: loud skip.
        if price == 0.0 {
            skipped.push(format!("{label} (Preis 0,00)"));
            continue;
        }
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let Some(unit) = unit_of(&unit_frag) else {
            skipped.push(format!(
                "{label} (Einheit unverständlich: {})",
                unit_frag.trim()
            ));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisticker leer".to_owned(),
        });
    }
    Ok((published_at, rows, skipped))
}

/// Live category prefixes heading ticker segments ("FE-Schrott: …",
/// "NE-Metalle: …"): strip them so the raw label stays mappable.
fn strip_category(seg: &str) -> &str {
    for prefix in ["FE-Schrott:", "NE-Metalle:"] {
        if let Some(i) = seg.find(prefix) {
            return seg[i + prefix.len()..].trim_start();
        }
    }
    seg
}

/// Split one ticker segment into (label, price, unit fragment). The
/// price is the last number token before `€` — leading numbers belong
/// to the label ("Misch./Privat ab 100 kg 135,-€/to"). The unit
/// fragment is strictly the text after `€`, so the "100 kg" threshold
/// can never leak into unit detection.
fn split_pair(seg: &str) -> Option<(String, f64, String)> {
    let euro = seg.find('€')?;
    let before = seg[..euro].trim_end();
    let after = seg[euro + '€'.len_utf8()..].trim().to_owned();
    let token = before.split_whitespace().last()?;
    let price = parse_eur(token)?;
    let label = before[..before.len() - token.len()].trim().to_owned();
    if label.is_empty() {
        return None;
    }
    Some((label, price, after))
}

/// Bespoke unit matcher for THIS ticker's unit fragments (live: "€/kg"
/// for NE metals, "€/to" for FE scrap). Only kg/t exist here — anything
/// else skips loudly at the call site.
fn unit_of(frag: &str) -> Option<&'static str> {
    let lower = frag.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|t| t == "t" || t == "to")
    {
        Some("EUR/t")
    } else {
        None
    }
}

/// First German calendar date (dd.mm.yyyy) after `marker`, as RFC 3339
/// UTC midnight. Invalid dates yield None, never a guessed date.
fn de_date_after(hay: &str, marker: &str) -> Option<String> {
    let i = hay.to_lowercase().find(&marker.to_lowercase())?;
    let after = &hay[i + marker.len()..];
    for token in after.split_whitespace().take(6) {
        let t = token.trim_matches(|c: char| c == ',' || c == ';' || c == ':');
        let mut parts = t.split('.');
        if let (Some(d), Some(m), Some(y)) = (parts.next(), parts.next(), parts.next()) {
            if parts.next().is_none() {
                if let Some(date) = parse_de_date(d, m, y) {
                    return Some(date);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{de_date_after, extract_info, grade_for, parse, split_pair};

    // Real excerpt of the live page (21.09.2026): news-ticker anchor with
    // the full dated list plus a decoy link before it and the "gültig
    // ab" heading. Ticker text kept verbatim (emojis, "175,-€/to").
    const FIXTURE: &str = "<div><a href=\"/\">Startseite</a>\
        <h3><span>Aktuelle Preisliste für Schrott und Metalle, gültig ab: 21.09.2026, 11:00 Uhr<br /></span></h3>\
        <div id=\"scroll-ntb-elem\"><span><a style='cursor:text !important;' href='#'>🚀🚀🚀 ALTMETALLPREISE Stand 21.09.2026 🚀🚀🚀 FE-Schrott: Misch./Händler 175,-€/to | Misch./Privat ab 100 kg 135,-€/to | FE-Guss 180,-€/to | Bremsscheiben 200,-€/to | Eisenspäne 140,-€/to | NE-Metalle: Aluminium Bleche rein-alt 1,80 €/kg | Alu-Felgen unsauber 2,20 €/kg | Alu-Kabel 1,00 €/kg | Alu-Guss ohne Fe 1,60 €/kg | Alu-Späne 1,00 €/kg | Alu-Isoprofile 1,80 €/kg | E-Motoren-klein 0,90 €/kg | Blei-Akku 0,30 €/kg | Blei 1,10 €/kg | Kupfer raff. 95% 8,50 €/kg | Millberry 10,- €/kg | Kupfer-Späne 8,50 €/kg | Kabel ohne Stecker 3,80 €/kg | Kabel mit Stecker 1,70 €/kg | Messing 5,70 €/kg | Messing Späne 5,00 €/kg | V2A 1,00 €/kg | V4A 1,80 €/kg | V2A-Späne 0,80 €/kg | Verhüttung 0,35 €/kg | Zink 2,00 €/kg | Zinngeschirr 14,- €/kg | Hartmetall/Widia 40,- €/kg | Keine Annahme von: E-Schrott, weißer Ware, Katalysatoren</a></span></div></div>";

    #[test]
    fn ticker_and_date_parse() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-21T00:00:00+00:00"));
        // 28 ticker segments, all with prices and known units.
        assert_eq!(rows.len(), 28);
        assert!(skips.is_empty(), "{skips:?}");
        assert_eq!(rows[0], ("Misch./Händler".to_owned(), 175.0, "EUR/t"));
        assert_eq!(rows[1].0, "Misch./Privat ab 100 kg");
        assert_eq!(rows[1].1, 135.0);
        assert_eq!(rows[1].2, "EUR/t");
        // The "100 kg" threshold belongs to the label, not the price.
        let (_, price, _) = split_pair("Misch./Privat ab 100 kg 135,-€/to").expect("pair");
        assert_eq!(price, 135.0);
        assert_eq!(rows[15], ("Millberry".to_owned(), 10.0, "EUR/kg"));
        assert_eq!(rows[27], ("Hartmetall/Widia".to_owned(), 40.0, "EUR/kg"));
        // Terminator cut the box: no exclusion text becomes a row.
        assert!(rows.iter().all(|r| !r.0.contains("Annahme")));
        assert!(rows.iter().all(|r| !r.0.contains("Katalysator")));
    }

    #[test]
    fn heading_date_is_fallback() {
        // Ticker without "Stand …" still dates via the "gültig ab" h3.
        let html = FIXTURE.replacen("Stand 21.09.2026", "Stand", 1);
        let (published_at, rows, _) = parse(&html).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-21T00:00:00+00:00"));
        assert_eq!(rows.len(), 28);
        assert_eq!(
            de_date_after("gültig ab: 21.09.2026, 11:00", "gültig ab").as_deref(),
            Some("2026-09-21T00:00:00+00:00")
        );
        assert_eq!(de_date_after("Stand 32.13.2026", "Stand"), None);
    }

    #[test]
    fn missing_ticker_and_empty_list_error_loudly() {
        // Only a nav link, no ticker anchor: loud error, not the first <a>.
        let err = parse("<div><a href=\"/\">Startseite</a></div>").expect_err("errors");
        assert!(err.to_string().contains("Preisticker"));
        // Every segment unparseable: loud error, not silent success.
        let html = FIXTURE.replace('€', "auf Anfrage");
        let err = parse(&html).expect_err("empty ticker errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn zero_unknown_unit_and_prose_skip_loudly() {
        // "0,00" is "no quote", never a price.
        let html = FIXTURE.replacen("V2A 1,00 €/kg", "V2A 0,00 €/kg", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 27);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("V2A") && skips[0].contains("0,00"));
        // Unknown unit: skipped loudly, valid rows survive.
        let html = FIXTURE.replacen("Millberry 10,- €/kg", "Millberry 10,- €/Sack", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 27);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("unverständlich"));
        // Prose segment without €: loud skip, never a label.
        let html = FIXTURE.replacen(
            "Verhüttung 0,35 €/kg",
            "Verhüttung 0,35 €/kg | Bitte rufen Sie uns an",
            1,
        );
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 28);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("kein Preis"));
    }

    #[test]
    fn impressum_extracts_contact() {
        // Real shape: h1 anchor, h2 + <p> with <br /> lines.
        let imp = "<h1 class=\"page-title\">Impressum</h1>\
            <h2>Angaben gemäß § 5 TMG</h2>\
            <p>P. U. Richter Umweltdienste Rheinland GmbH<br />\nFriesdorfer Straße 176<br />\n53175 Bonn</p>\
            <h2>Kontakt</h2>\
            <p>Telefon: +49 228 95129-18<br />\nTelefax: +49 228 95129-99<br />\nE-Mail: entsorgung@pur-umwelt.com</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Friesdorfer Straße 176");
        assert_eq!(info.postcode, "53175");
        assert_eq!(info.city, "Bonn");
        assert_eq!(info.phone, "+49 228 95129-18");
        assert_eq!(info.email, "entsorgung@pur-umwelt.com");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(extract_info("<h1 class=\"page-title\">Impressum</h1><p>leer</p>").is_err());
    }

    #[test]
    fn mapping_covers_live_ticker() {
        assert_eq!(
            grade_for("Misch./Händler"),
            Some(("mischschrott", "Händler"))
        );
        assert_eq!(
            grade_for("Misch./Privat ab 100 kg"),
            Some(("mischschrott", "Privat ab 100 kg"))
        );
        assert_eq!(grade_for("FE-Guss"), Some(("eisenschrott-gussbruch", "")));
        assert_eq!(
            grade_for("Bremsscheiben"),
            Some(("eisenschrott-gussbruch", "Bremsscheiben"))
        );
        assert_eq!(
            grade_for("Aluminium Bleche rein-alt"),
            Some(("aluminium-blech", "rein alt"))
        );
        assert_eq!(
            grade_for("Alu-Felgen unsauber"),
            Some(("aluminium-guss", "Felgen unsauber"))
        );
        assert_eq!(grade_for("Alu-Kabel"), Some(("kabel-alu", "")));
        assert_eq!(
            grade_for("Alu-Guss ohne Fe"),
            Some(("aluminium-guss", "ohne Fe"))
        );
        assert_eq!(grade_for("Alu-Späne"), Some(("aluminium-gemisch", "Späne")));
        assert_eq!(
            grade_for("Alu-Isoprofile"),
            Some(("aluminium-profile", "Iso"))
        );
        assert_eq!(
            grade_for("E-Motoren-klein"),
            Some(("elektromotoren", "klein"))
        );
        assert_eq!(grade_for("Blei-Akku"), Some(("blei", "Akku")));
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(
            grade_for("Kupfer raff. 95%"),
            Some(("kupfer-gemischt", "Raff 95%"))
        );
        assert_eq!(grade_for("Millberry"), Some(("kupfer-millberry", "")));
        assert_eq!(
            grade_for("Kupfer-Späne"),
            Some(("kupfer-gemischt", "Späne"))
        );
        assert_eq!(
            grade_for("Kabel ohne Stecker"),
            Some(("kabel-kupfer", "ohne Stecker"))
        );
        assert_eq!(
            grade_for("Kabel mit Stecker"),
            Some(("kabel-kupfer", "mit Stecker"))
        );
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Messing Späne"), Some(("messing", "Späne")));
        assert_eq!(grade_for("V2A"), Some(("edelstahl-v2a", "")));
        assert_eq!(grade_for("V4A"), Some(("edelstahl-v4a", "")));
        assert_eq!(grade_for("V2A-Späne"), Some(("edelstahl-v2a", "Späne")));
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(
            grade_for("Zinngeschirr"),
            Some(("zinn-geschirr", "Geschirr"))
        );
        // Widia is hard metal, not an ambiguous either/or label.
        assert_eq!(grade_for("Hartmetall/Widia"), Some(("hartmetall", "")));
        // No catalog material: loud skips, never guessed.
        assert_eq!(grade_for("Eisenspäne"), None);
        assert_eq!(grade_for("Verhüttung"), None);
    }
}
