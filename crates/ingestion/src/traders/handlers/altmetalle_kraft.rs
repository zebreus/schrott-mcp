//! Altmetalle Kraft (Dormagen, Robert-Boschstr. 24): exact per-kg prices in
//! the single `<ul>` price list under the "Ankaufspreise pro kg:" heading
//! (windowed from the heading to the closing `</ul>` — never page-wide).
//! Each `<li>` pairs a sort label with a trailing price ("Kabel 38 % ...
//! 3,90 €"); the five Kabel copper-share tiers (38/50/60/70/80 %) ride in
//! the variant (Böhner-Staffel-Präzedenz im selben Gebiet), as do the three
//! colliding Alu-Profil sorts ("", "lackiert", "Iso"). Live 28.09.2026:
//! 20 list points → 19 prices (all bare "€", i.e. EUR/kg per the heading)
//! + 1 loud skip ("Verhüttung" is a process, Kalkmann-Präzedenz). No price
//! date on the page → `published_at` stays `None` (`observed_at` = age).

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-dormagen-altmetalle-kraft";
/// Bespoke, live-verified impressum URL (footer nav link, HTTP 200 am
/// 28.09.2026 mit Adress-/Kontakt-Block). A move fails the step loudly
/// (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://schrott-koeln.de/impressum/";

pub const URL: &str = "https://schrott-koeln.de/preise/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
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
        published_at: None,
    })
}

/// Explicit label → (material, variant) mapping. Arms are ordered
/// specific-before-generic: "Millberry" before bare "Kupfer", "Alu Profil
/// lackiert"/"Alu Profil Iso" before bare "Alu Profil", "Kabel" before
/// "Kupfer" (a future "Kupferkabel NN %" still lands on cable with its
/// tier, never on gemischt Kupfer). Sorts sharing one
/// material carry the sort word in the variant so they never collapse onto
/// one current price; the Kabel copper-share tiers do the same (Staffel).
/// "Verhüttung" is a process, not a material (Kalkmann-Präzedenz), and
/// anything unlisted skips loudly.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kabel") {
        kabel_variant(label).map(|v| ("kabel-kupfer", v))
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("profil") && l.contains("lackiert") {
        Some(("aluminium-profile", "lackiert"))
    } else if l.contains("profil") && l.contains("iso") {
        Some(("aluminium-profile", "Iso"))
    } else if l.contains("profil") {
        Some(("aluminium-profile", ""))
    } else if l.contains("geschirr") {
        // "Alu Geschirr" → Blech (Gutzmann-Präzedenz), nie Zinn-Geschirr.
        Some(("aluminium-blech", "Geschirr"))
    } else if l.contains("felgen") {
        // Cast wheels → Guss (Albus/Buntmetall/DB-Präzedenz).
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("e-motor") || l.contains("e motor") || l.contains("elektromotor") {
        Some(("elektromotoren", ""))
    } else {
        None
    }
}

/// Copper-share tier of a "Kabel NN %" label as variant ("38 %" … "80 %").
/// Anything else (unknown share, no share at all) is `None` — a guessed
/// tier would corrupt the Staffel, so it skips loudly at the call site.
fn kabel_variant(label: &str) -> Option<&'static str> {
    let toks: Vec<&str> = label.split_whitespace().collect();
    let i = toks.iter().position(|t| t.contains('%'))?;
    let num_tok = if toks[i].chars().any(|c| c.is_ascii_digit()) {
        toks[i]
    } else if i > 0 {
        toks[i - 1]
    } else {
        return None;
    };
    let digits: String = num_tok.chars().filter(|c| c.is_ascii_digit()).collect();
    match digits.as_str() {
        "38" => Some("38 %"),
        "50" => Some("50 %"),
        "60" => Some("60 %"),
        "70" => Some("70 %"),
        "80" => Some("80 %"),
        _ => None,
    }
}

/// Parse the price-list window (heading-selected, never the first list or
/// the whole page). Returns rows of (sort label, price, unit) plus loud
/// skips. `li` elements without `€` are structure/headers, never labels;
/// `€` text without digits is a header, not a label. 0 priced rows = `Err`.
fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html.find("Ankaufspreise pro kg").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisliste fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("</ul>").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisliste unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    // Proper element selection: a `<link>` tag starts with "<li" too, so
    // only real `li` elements count — never substring matches.
    let frag = Html::parse_fragment(window);
    let li_sel = Selector::parse("li").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for li in frag.select(&li_sel) {
        let t = li
            .text()
            .collect::<String>()
            .replace(['\u{a0}'], " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if t.is_empty() || t.len() > 120 {
            continue;
        }
        if !t.contains('€') || parse_eur(&t).is_none() {
            // Structure/header inside the list ("Tagespreise", "€"-only
            // text without digits), never a label.
            continue;
        }
        // The price is the last token before "€" — labels themselves hold
        // digits ("Kabel 38 %"), so splitting must run from the right.
        let euro = t.find('€').expect("checked above");
        let before = t[..euro].trim();
        let mut parts = before.split_whitespace();
        let price_tok = parts.next_back().unwrap_or("");
        let Some(price) = parse_eur(price_tok) else {
            skips.push(format!("{t} (Preis unverständlich: {price_tok})"));
            continue;
        };
        let label = parts.collect::<Vec<_>>().join(" ");
        if label.is_empty() {
            skips.push(format!("{t} (Label unverständlich)"));
            continue;
        }
        // A "0,00" cell is "no quote", not a free gift: loud skip.
        if price == 0.0 {
            skips.push(format!("{label} (Preis 0,00)"));
            continue;
        };
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let Some(unit) = unit_of(&t) else {
            skips.push(format!("{label} (Einheit unverständlich: {t})"));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Preisliste leer".to_owned() });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THIS list (live: heading "Ankaufspreise pro
/// kg:", rows carry a bare "€"). Only kg/t exist here — anything else
/// skips loudly at the call site. The bare-"€" default is pinned by the
/// parse anchor itself: a retitled heading ("pro to") misses
/// "Ankaufspreise pro kg" and fails loudly instead of misreading every
/// unit 1000x off.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower.split(|c: char| !c.is_alphanumeric()).any(|t| t == "t" || t == "to") {
        Some("EUR/t")
    } else if lower.contains('/') || lower.contains("pro ") || lower.contains("je ") {
        None
    } else if lower.contains('€') {
        Some("EUR/kg")
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: anchored on the
/// `h2` ("Impressum") and the "Inhaber:" `<p>` (firm + street + PLZ city
/// over `<br>`); the "Tel.:" `<p>` holds phone ("Tel.:") and e-mail
/// ("E-Mail:", own rule — a phone-token filter stops at the first
/// letter). Page spelling is kept verbatim ("Robert-Boschstr. 24", never
/// normalized). Missing anchors mean the page changed shape → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    if !doc.select(&h2).any(|h| h.text().collect::<String>().contains("Impressum")) {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let p_sel = Selector::parse("p").expect("valid selector");
    let addr_p = doc.select(&p_sel).find(|el| el.inner_html().contains("Inhaber:"));
    let contact_p = doc.select(&p_sel).find(|el| el.inner_html().contains("Tel.:"));
    let (Some(addr_p), Some(contact_p)) = (addr_p, contact_p) else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-/Kontakt-Block fehlt".to_owned(),
        });
    };
    let addr_lines: Vec<String> = addr_p
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in addr_lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| format!("{a} {b}"));
                if k > 0 {
                    street = addr_lines[k - 1].clone();
                }
                break;
            }
        }
    }
    let mut phone = String::new();
    let mut email = String::new();
    for part in contact_p.inner_html().split("<br") {
        let t = strip_fragment(part);
        if let Some(v) = t.strip_prefix("Tel.:") {
            phone = v.trim().to_owned();
        } else if let Some(v) = t.strip_prefix("E-Mail:") {
            email = v.trim().to_owned();
        }
    }
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo { street, postcode, city, phone, email })
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
    use super::{grade_for, kabel_variant, parse, unit_of};

    // Real excerpt of the live list (heading + all 20 `<li>`, 28.09.2026),
    // only wrapped for the fragment parser — labels, entities and spacing
    // are verbatim so the fixture cannot drift from the page.
    const FIXTURE: &str = "<h5><span style=\"font-family:Arial;\">Ankaufspreise pro kg:</span></h5>\
        <div><ul><li>Mischschrott &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;0,17 €</li>\
        <li>Alu Geschirr &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; 1,20 €</li>\
        <li>Alu Profil &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; 1,80 €</li>\
        <li>Alu Profil lackiert &nbsp;1,50 €</li>\
        <li>Alu Profil Iso &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;1,50 €</li>\
        <li>Alu Felgen &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;1,70 €</li>\
        <li>V2A &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;0,70 €</li>\
        <li>V4A &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;1,40 €</li>\
        <li>Zink &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; 1,60 €</li>\
        <li>Blei &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;1,20 €</li>\
        <li>Kupfer schwer &nbsp; &nbsp; &nbsp; &nbsp;10,50 €</li>\
        <li>Kupfer Millberry &nbsp; &nbsp;11,50 €</li>\
        <li>Kabel 38 % &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;3,90 €</li>\
        <li>Kabel 50 % &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;4,20 €</li>\
        <li>Kabel 60 % &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;4,50 €</li>\
        <li>Kabel 70 % &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;5,20 €</li>\
        <li>Kabel 80 % &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;5,60 €</li>\
        <li>Messing &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; 6,70 €</li>\
        <li>E-Motor &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; 0,30 €</li>\
        <li>Verhüttung &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;0,20 €</li></ul></div>";

    #[test]
    fn full_list_rows_and_spot_prices() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // 20 list points: 19 priced rows, "Verhüttung" maps to no material
        // and is counted in `skipped_labels` by the caller, not here.
        assert_eq!(rows.len(), 20);
        assert!(skips.is_empty());
        let get = |label: &str| rows.iter().find(|(l, _, _)| l == label).expect(label);
        assert_eq!(get("Mischschrott"), &("Mischschrott".to_owned(), 0.17, "EUR/kg"));
        assert_eq!(get("Kupfer Millberry"), &("Kupfer Millberry".to_owned(), 11.5, "EUR/kg"));
        assert_eq!(get("Messing"), &("Messing".to_owned(), 6.7, "EUR/kg"));
        assert_eq!(get("Kabel 38 %"), &("Kabel 38 %".to_owned(), 3.9, "EUR/kg"));
        assert_eq!(get("Kabel 80 %"), &("Kabel 80 %".to_owned(), 5.6, "EUR/kg"));
        assert_eq!(get("E-Motor"), &("E-Motor".to_owned(), 0.3, "EUR/kg"));
        assert!(rows.iter().all(|(_, _, u)| *u == "EUR/kg"));
        assert!(rows.iter().all(|(_, p, _)| *p > 0.0));
        // Missing heading or missing list end fails loudly (redesign).
        assert!(parse("<div><ul><li>Mischschrott 0,17 €</li></ul></div>").is_err());
        assert!(parse("Ankaufspreise pro kg: ohne Liste").is_err());
        assert!(parse("Ankaufspreise pro kg:<ul></ul>").is_err());
    }

    #[test]
    fn zero_price_skips_loudly() {
        let html = FIXTURE.replacen("0,17 €", "0,00 €", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 19);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Mischschrott") && skips[0].contains("0,00"));
        assert!(rows.iter().all(|(_, p, _)| *p > 0.0));
    }

    #[test]
    fn mapping_keeps_sorts_apart() {
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Alu Geschirr"), Some(("aluminium-blech", "Geschirr")));
        assert_eq!(grade_for("Alu Profil"), Some(("aluminium-profile", "")));
        assert_eq!(grade_for("Alu Profil lackiert"), Some(("aluminium-profile", "lackiert")));
        assert_eq!(grade_for("Alu Profil Iso"), Some(("aluminium-profile", "Iso")));
        assert_eq!(grade_for("Alu Felgen"), Some(("aluminium-guss", "Felgen")));
        assert_eq!(grade_for("V2A"), Some(("edelstahl-v2a", "")));
        assert_eq!(grade_for("V4A"), Some(("edelstahl-v4a", "")));
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        // Specific before generic: Millberry must not land on gemischt.
        assert_eq!(grade_for("Kupfer Millberry"), Some(("kupfer-millberry", "")));
        assert_eq!(grade_for("Kupfer schwer"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Kabel 38 %"), Some(("kabel-kupfer", "38 %")));
        assert_eq!(grade_for("Kabel 50 %"), Some(("kabel-kupfer", "50 %")));
        assert_eq!(grade_for("Kabel 60 %"), Some(("kabel-kupfer", "60 %")));
        assert_eq!(grade_for("Kabel 70 %"), Some(("kabel-kupfer", "70 %")));
        assert_eq!(grade_for("Kabel 80 %"), Some(("kabel-kupfer", "80 %")));
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("E-Motor"), Some(("elektromotoren", "")));
        // Process, not a material — loud skip, never a guessed row.
        // Tier-less cable ("Kupferkabel") skips too: no tier, no variant.
        assert_eq!(grade_for("Verhüttung"), None);
        assert_eq!(grade_for("Kupferkabel"), None);
        assert_eq!(grade_for("Kupferkabel 50 %"), Some(("kabel-kupfer", "50 %")));
        assert_eq!(grade_for("Katalysatoren"), None);
        assert_eq!(kabel_variant("Kabel 38 %"), Some("38 %"));
        assert_eq!(kabel_variant("Kabel 80%"), Some("80 %"));
        assert_eq!(kabel_variant("Kabel 45 %"), None);
        assert_eq!(kabel_variant("Kabel"), None);
    }

    #[test]
    fn units_bare_eur_default_and_foreign_reject() {
        assert_eq!(unit_of("0,17 €"), Some("EUR/kg"));
        assert_eq!(unit_of("1,50 €/kg"), Some("EUR/kg"));
        assert_eq!(unit_of("100,00 €/t"), Some("EUR/t"));
        assert_eq!(unit_of("0,170 € pro Sack"), None);
        assert_eq!(unit_of("Preis auf Anfrage"), None);
    }

    #[test]
    fn impressum_blocks() {
        let imp = "<h2 style=\"text-align:center\"><span>Impressum</span></h2>\
            <div><p>Mickey Jerome Kraft<br>Altmetalle Kraft Inhaber:Mickey Jerome Kraft<br>\
            Robert-Boschstr. 24<br>41541 Dormagen<br>Deutschland</p>\
            <p>Tel.: +491635927189<br>E-Mail: info@schrott-koeln.de</p></div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Robert-Boschstr. 24");
        assert_eq!(info.postcode, "41541");
        assert_eq!(info.city, "Dormagen");
        assert_eq!(info.phone, "+491635927189");
        assert_eq!(info.email, "info@schrott-koeln.de");
        assert!(super::extract_info("<h2>Sonst was</h2>").is_err());
        assert!(super::extract_info("<h2>Impressum</h2><p>Ohne Anker</p>").is_err());
    }
}
