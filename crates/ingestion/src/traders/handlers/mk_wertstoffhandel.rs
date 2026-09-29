//! MK Wertstoffhandel (Marzahn-Hellersdorf, Berlin): exact per-kg prices in
//! twelve `table.price-table` blocks (thead "Material"/"Preis"), one block
//! per section (Buntmetall, Messing/Rotguss, Aluminium, Blei, Edelstahl,
//! Zink, Zinn, Schrott, Elektromotoren, Papier, Elektronik, Entsorgung).
//! Labels are `span.item-name` plus an optional `span.item-quality`
//! qualifier; the unit lives in `span.price-unit` ("€/kg", "ct/kg" —
//! cents are converted to EUR at parse time — or "€/m³", which skips
//! loudly). The Mischschrott row carries a second tier in
//! `span.item-note` ("ab 1000 kg: 12,0 ct/kg") recorded as its own
//! variant. Paper rows, Handys and Pappe have no catalog material or no
//! kg/t unit and are skipped loudly. No page date is shown ("täglich
//! aktualisiert" is not a date), so `published_at` stays `None`.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "be-marzahn-hellersdorf-mk-wertstoffhandel-daniel-hartwig";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://mk-wertstoffhandel.de/impressum";

pub const URL: &str = "https://mk-wertstoffhandel.de/preise";

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
    let (rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (label, tier, price, unit) in rows {
        match grade_for(&label, &tier) {
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

/// Explicit (label, tier) → (material, variant) mapping. Anything unlisted
/// is skipped. The variant keeps the trader's own grade wording; the only
/// quantity tier ("ab 1000 kg" on Mischschrott) rides along mkr-style.
fn grade_for(label: &str, tier: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Percent figures arrive spaced ("40 %") or glued ("40%") — match on
    // the spaceless form so neither spelling slips through.
    let nospace: String = l.split_whitespace().collect();
    let pct = |n: &str| nospace.contains(n);
    let (material, detail): (&'static str, &'static str) = if l.contains("millberry") {
        ("kupfer-millberry", "")
    } else if l.contains("milbe") {
        ("kupfer-millberry", "Milbe dünn")
    } else if l.contains("kerze") {
        ("kupfer-berry", "Kerze")
    } else if l.contains("raff") && !l.contains("kabel") {
        ("kupfer-gemischt", "Raff")
    } else if l.contains("erdkabel") {
        ("kabel-kupfer", "Erdkabel")
    } else if l.contains("kabel") && pct("40%") {
        ("kabel-kupfer", "40%")
    } else if l.contains("kabel") && pct("50%") {
        ("kabel-kupfer", "50%")
    } else if l.contains("kabel") && pct("60%") {
        ("kabel-kupfer", "60%")
    } else if l.contains("kabel") && pct("80%") {
        ("kabel-kupfer", "80%")
    } else if l.contains("rotguss") {
        ("bronze-rotguss", "")
    } else if l.contains("wasseruhren") && l.contains("ohne plastik") {
        ("messing", "Wasseruhren ohne Plastik")
    } else if l.contains("wasseruhren") {
        ("messing", "Wasseruhren mit Plastik")
    } else if l.contains("messing") && l.contains("anhaftung") {
        ("messing", "mit Anhaftungen")
    } else if l.contains("messing") {
        ("messing", "")
    } else if l.contains("alu") && l.contains("profil") {
        ("aluminium-profile", "")
    } else if l.contains("offset") {
        ("aluminium-blech", "Offset")
    } else if l.contains("felgen") {
        ("aluminium-guss", "Felgen")
    } else if l.contains("alu") && l.contains("anhaftung") {
        ("aluminium-gemischt", "mit Anhaftungen")
    } else if l.contains("alu") {
        ("aluminium-gemischt", "")
    } else if l.contains("auswucht") {
        ("blei-auswucht", "Auswucht")
    } else if l.contains("blei") {
        ("blei", "")
    } else if l.contains("v4a") {
        ("edelstahl-v4a", "")
    } else if l.contains("v2a") && l.contains("brenner") {
        ("edelstahl-v2a", "Brenner")
    } else if l.contains("v2a") {
        ("edelstahl-v2a", "kurz")
    } else if l.contains("zink") && l.contains("neu") {
        ("zink", "neu")
    } else if l.contains("zink") && l.contains("alt") {
        ("zink", "alt")
    } else if l.contains("zink") {
        ("zink", "")
    } else if l.contains("lötzinn") || l.contains("loetzinn") {
        ("loetzinn", "Lötzinn")
    } else if l.contains("zinngeschirr") {
        ("zinn-geschirr", "Geschirr")
    } else if l.contains("zinn") {
        ("zinn", "rein")
    } else if l.contains("mischschrott") || l.contains("altmetall") {
        ("mischschrott", "")
    } else if l.contains("bremsscheiben") {
        ("eisenschrott-gussbruch", "Bremsscheiben")
    } else if l.contains("elektromotor") && l.contains("anhaftung") {
        ("elektromotoren", "mit Anhaftungen")
    } else if l.contains("elektromotor") {
        ("elektromotoren", "")
    } else {
        return None;
    };
    let variant: &'static str = match (detail, tier) {
        ("", "") => "",
        ("", "ab 1000 kg") => "ab 1000 kg",
        _ if tier.is_empty() => detail,
        _ => return None,
    };
    Some((material, variant))
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after the
/// "Angaben gemäß § 5 DDG" heading holds firm lines + street + PLZ city,
/// and the `<p>` after the "Kontakt" heading carries `tel:`/`mailto:`
/// links (hrefs, never token-split — scraper glues "Telefon:" to the
/// number). Missing headings mean the page changed shape → loud error,
///
/// never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let anchor = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Angaben gemäß § 5 DDG");
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    let addr_p = anchor
        .next_siblings()
        .filter_map(scraper::ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let mut lines = Vec::new();
    if let Some(p) = addr_p {
        for part in p.inner_html().split("<br") {
            let t = strip_tags(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
    // "Grabensprung 1" / "12683 Berlin" (last two lines).
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if lines.len() >= 2 {
        let last = lines.last().expect("len checked");
        let mut it = last.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = ci.to_owned();
                street = lines[lines.len() - 2].clone();
            }
        }
    }
    let kontakt = doc
        .select(&h2)
        .find(|h| h.text().collect::<String>().trim() == "Kontakt");
    let Some(kontakt) = kontakt else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let mut phone = String::new();
    let mut email = String::new();
    if let Some(p) = kontakt
        .next_siblings()
        .filter_map(scraper::ElementRef::wrap)
        .find(|e| e.value().name() == "p")
    {
        let a = Selector::parse("a").expect("valid selector");
        for link in p.select(&a) {
            if let Some(href) = link.value().attr("href") {
                if let Some(num) = href.strip_prefix("tel:") {
                    phone = num.to_owned();
                } else if let Some(addr) = href.strip_prefix("mailto:") {
                    email = addr.to_owned();
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

/// Strip tags from a fragment (entities are already decoded by html5ever).
fn strip_tags(s: &str) -> String {
    // Drop tag rests first: a `<br`-split leaves `class="…"` behind which
    // would otherwise parse as text.
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
) -> Result<(Vec<(String, String, f64, &'static str)>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let table = Selector::parse("table").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
    let row = Selector::parse("tbody tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let name = Selector::parse("span.item-name").expect("valid selector");
    let quality = Selector::parse("span.item-quality").expect("valid selector");
    let note = Selector::parse("span.item-note").expect("valid selector");
    let price_val = Selector::parse("span.price-val").expect("valid selector");
    let price_unit = Selector::parse("span.price-unit").expect("valid selector");
    // Never trust page order: take every table carrying the price header,
    // not just the first <table> on the page.
    let tables: Vec<_> = doc
        .select(&table)
        .filter(|t| {
            t.select(&head)
                .any(|h| h.text().collect::<String>().trim() == "Material")
        })
        .collect();
    if tables.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    }
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for table in tables {
        for tr in table.select(&row) {
            let cells: Vec<_> = tr.select(&cell).collect();
            if cells.len() < 2 {
                continue;
            }
            let item: String = cells[0]
                .select(&name)
                .next()
                .map(|e| e.text().collect())
                .unwrap_or_default();
            let item: String = item.split_whitespace().collect::<Vec<_>>().join(" ");
            if item.is_empty() {
                continue;
            }
            let qual: String = cells[0]
                .select(&quality)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
            let qual: String = qual.split_whitespace().collect::<Vec<_>>().join(" ");
            let label = if qual.is_empty() {
                item.clone()
            } else {
                format!("{item} {qual}")
            };
            if label.len() > 120 {
                continue; // Prosa, kein Label.
            }
            let price_text: String = cells[1]
                .select(&price_val)
                .next()
                .map(|e| e.text().collect())
                .unwrap_or_default();
            let unit_text: String = cells[1]
                .select(&price_unit)
                .next()
                .map(|e| e.text().collect())
                .unwrap_or_default();
            let Some(price) = parse_eur(&price_text) else {
                skips.push(format!(
                    "{label} (Preis unverständlich: {})",
                    price_text.trim()
                ));
                continue;
            };
            // An unparseable unit is a loud skip, never a silent default: a
            // per-m³ fee recorded as per-kg would be a nonsense price.
            let Some((unit, factor)) = unit_of(&unit_text) else {
                skips.push(format!(
                    "{label} (Einheit unverständlich: {})",
                    unit_text.trim()
                ));
                continue;
            };
            rows.push((label.clone(), String::new(), price * factor, unit));
            // Second quantity tier from the item note ("ab 1000 kg:
            // 12,0 ct/kg"): its own row, its own variant.
            let note_text: String = cells[0]
                .select(&note)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
            let note_text: String = note_text.split_whitespace().collect::<Vec<_>>().join(" ");
            if !note_text.is_empty() {
                match parse_tier(&note_text) {
                    Some((tier, tier_price)) => {
                        rows.push((label, tier, tier_price, unit));
                    }
                    None => skips.push(format!("{label} (Staffel unverständlich: {note_text})")),
                }
            }
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke tier parser for THIS page's item notes ("ab 1000 kg:
/// 12,0 ct/kg" → ("ab 1000 kg", 0.12)). Cents convert like the main
/// unit; anything else in a note is not a price we can prove.
fn parse_tier(note: &str) -> Option<(String, f64)> {
    let (tier_part, price_part) = note.split_once("kg:")?;
    let tier = format!("{} kg", tier_part.trim());
    if !tier.starts_with("ab ") {
        return None;
    }
    let price = parse_eur(price_part)?;
    let factor = if price_part.contains("ct") { 0.01 } else { 1.0 };
    Some((tier, price * factor))
}

/// Bespoke unit matcher for THIS page's price-unit spans (live: "€/kg",
/// "ct/kg" on Schrott/Papier rows, "€/m³" on Pappe fees). Returns the
/// catalog unit plus the EUR conversion factor (ct → EUR is exact).
/// Only kg/t exist here — anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<(&'static str, f64)> {
    let lower = cell.to_lowercase().replace([' ', '\u{a0}'], "");
    if lower.contains("ct/kg") {
        Some(("EUR/kg", 0.01))
    } else if lower.contains("€/kg") || lower.contains("eur/kg") {
        Some(("EUR/kg", 1.0))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, parse_tier, unit_of};

    const FIXTURE: &str = "<table class=\"price-table\"><thead><tr><th scope=\"col\">Material</th><th scope=\"col\">Preis</th></tr></thead>\
        <tbody><tr><td><span class=\"item-name\">Kupfer Millberry</span></td><td><span class=\"price-val\">10,30<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Kupfer Milbe dünn</span><span class=\"item-quality\">feiner Draht</span></td><td><span class=\"price-val\">10,10<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Kupfer Kerze</span><span class=\"item-quality\">schwer, sauber</span></td><td><span class=\"price-val\">9,00<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Kupfer raff</span><span class=\"item-quality\">95 % mit Anhaftungen</span></td><td><span class=\"price-val\">9,60<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Kupfer Kabel 40 %</span><span class=\"item-quality\">Haushaltskabel</span></td><td><span class=\"price-val\">3,40<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Kupfer Erdkabel</span><span class=\"item-quality\">fetthaltig</span></td><td><span class=\"price-val\">0,20<span class=\"price-unit\">€/kg</span></span></td></tr>\
        </tbody></table>\
        <table class=\"price-table\"><thead><tr><th scope=\"col\">Material</th><th scope=\"col\">Preis</th></tr></thead>\
        <tbody><tr><td><span class=\"item-name\">Rotguss</span></td><td><span class=\"price-val\">7,00<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Messing Wasseruhren</span><span class=\"item-quality\">ohne Plastik</span></td><td><span class=\"price-val\">2,50<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Alu Felgen</span><span class=\"item-quality\">sauber</span></td><td><span class=\"price-val\">1,60<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Alu sauber</span><span class=\"item-quality\">kein Alu Guss</span></td><td><span class=\"price-val\">1,30<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">V2A Brenner</span><span class=\"item-quality\">Gastrogeräte</span></td><td><span class=\"price-val\">0,35<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Lötzinn</span><span class=\"item-quality\">40 %</span></td><td><span class=\"price-val\">6,00<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Altmetall / Mischschrott</span><span class=\"item-note\">ab 1000 kg: 12,0 ct/kg</span></td><td><span class=\"price-val\">10,0<span class=\"price-unit\">ct/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Elektromotoren</span><span class=\"item-quality\">mit Anhaftungen</span></td><td><span class=\"price-val\">0,25<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Gemischtes Papier / Illustrierte</span></td><td><span class=\"price-val\">10,0<span class=\"price-unit\">ct/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Handys</span><span class=\"item-quality\">ohne Akku</span></td><td><span class=\"price-val\">3,00<span class=\"price-unit\">€/kg</span></span></td></tr>\
        <tr><td><span class=\"item-name\">Pappe privat</span><span class=\"item-quality\">bis 1 m³ kostenlos, danach Gebühr</span></td><td><span class=\"price-val\">5,00<span class=\"price-unit\">€/m³</span></span></td></tr>\
        </tbody></table>";

    #[test]
    fn tables_units_and_tier_parse() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // 16 priced rows + 1 staffel row; Pappe (€/m³) skips loudly.
        assert_eq!(rows.len(), 17, "{rows:?}");
        assert_eq!(skips.len(), 1, "{skips:?}");
        assert!(
            skips[0].contains("Pappe privat") && skips[0].contains("€/m³"),
            "{skips:?}"
        );
        assert_eq!(
            rows[0],
            ("Kupfer Millberry".to_owned(), String::new(), 10.3, "EUR/kg")
        );
        assert_eq!(rows[1].0, "Kupfer Milbe dünn feiner Draht");
        // Cents convert to EUR.
        let misch: Vec<_> = rows
            .iter()
            .filter(|r| r.0.contains("Mischschrott"))
            .collect();
        assert_eq!(misch.len(), 2);
        assert_eq!((misch[0].1.as_str(), misch[0].2), ("", 0.1));
        assert_eq!((misch[1].1.as_str(), misch[1].2), ("ab 1000 kg", 0.12));
        let papier = rows
            .iter()
            .find(|r| r.0.contains("Papier"))
            .expect("papier row");
        assert_eq!(papier.2, 0.1);
    }

    #[test]
    fn wrong_table_unit_and_tier_are_rejected_loudly() {
        // A layout table before the price tables must not win.
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (rows, _) = parse(&html).expect("finds the price tables");
        assert_eq!(rows.len(), 17);
        // Unknown unit: skipped loudly, valid rows survive.
        let html = FIXTURE.replacen("€/kg", "pro Sack", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 16);
        assert_eq!(skips.len(), 2);
        assert!(skips.iter().any(|s| s.contains("Millberry")), "{skips:?}");
        // Every row unparseable: loud error, not silent success.
        let html = FIXTURE
            .replace("€/kg", "pro Sack")
            .replace("ct/kg", "pro Sack")
            .replace("€/m³", "pro Sack");
        let err = parse(&html).expect_err("empty tables error");
        assert!(err.to_string().contains("leer"));
        // No price table at all: loud error.
        assert!(parse("<html><body><p>Neu hier</p></body></html>").is_err());
        // Broken staffel note: base row survives, note skips loudly.
        assert!(parse_tier("ab 1000 kg: 12,0 ct/kg") == Some(("ab 1000 kg".to_owned(), 0.12)));
        assert!(parse_tier("Preis auf Anfrage").is_none());
        assert_eq!(unit_of("€/kg"), Some(("EUR/kg", 1.0)));
        assert_eq!(unit_of("ct/kg"), Some(("EUR/kg", 0.01)));
        assert_eq!(unit_of("€/m³"), None);
    }

    #[test]
    fn mapping_covers_live_labels() {
        assert_eq!(
            grade_for("Kupfer Millberry", ""),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Kupfer Milbe dünn feiner Draht", ""),
            Some(("kupfer-millberry", "Milbe dünn"))
        );
        assert_eq!(
            grade_for("Kupfer Kerze schwer, sauber", ""),
            Some(("kupfer-berry", "Kerze"))
        );
        assert_eq!(
            grade_for("Kupfer raff 95 % mit Anhaftungen", ""),
            Some(("kupfer-gemischt", "Raff"))
        );
        assert_eq!(
            grade_for("Kupfer Kabel 40 % Haushaltskabel", ""),
            Some(("kabel-kupfer", "40%"))
        );
        assert_eq!(
            grade_for("Kupfer Erdkabel fetthaltig", ""),
            Some(("kabel-kupfer", "Erdkabel"))
        );
        assert_eq!(grade_for("Rotguss", ""), Some(("bronze-rotguss", "")));
        assert_eq!(
            grade_for("Messing Wasseruhren ohne Plastik", ""),
            Some(("messing", "Wasseruhren ohne Plastik"))
        );
        assert_eq!(
            grade_for("Messing Wasseruhren mit Plastik", ""),
            Some(("messing", "Wasseruhren mit Plastik"))
        );
        assert_eq!(
            grade_for("Messingschrott sauber", ""),
            Some(("messing", ""))
        );
        assert_eq!(
            grade_for("Messing mit Anhaftungen", ""),
            Some(("messing", "mit Anhaftungen"))
        );
        assert_eq!(
            grade_for("Alu Profile blank/sauber", ""),
            Some(("aluminium-profile", ""))
        );
        assert_eq!(
            grade_for("Alu Offset Bleche", ""),
            Some(("aluminium-blech", "Offset"))
        );
        assert_eq!(
            grade_for("Alu Felgen sauber", ""),
            Some(("aluminium-guss", "Felgen"))
        );
        assert_eq!(
            grade_for("Alu sauber kein Alu Guss", ""),
            Some(("aluminium-gemischt", ""))
        );
        assert_eq!(
            grade_for("Alu mit Anhaftungen maximal 5 %", ""),
            Some(("aluminium-gemischt", "mit Anhaftungen"))
        );
        assert_eq!(grade_for("Bleischrott sauber", ""), Some(("blei", "")));
        assert_eq!(grade_for("Auswuchtblei", ""), Some(("blei-auswucht", "Auswucht")));
        assert_eq!(
            grade_for("V4A Edelstahl Sofortanalyse", ""),
            Some(("edelstahl-v4a", ""))
        );
        assert_eq!(
            grade_for("V2A Edelstahl kurz", ""),
            Some(("edelstahl-v2a", "kurz"))
        );
        assert_eq!(
            grade_for("V2A Brenner Gastrogeräte", ""),
            Some(("edelstahl-v2a", "Brenner"))
        );
        assert_eq!(
            grade_for("Zink neu sauber, ohne Lötstellen", ""),
            Some(("zink", "neu"))
        );
        assert_eq!(
            grade_for("Zink alt mit Lötstellen, ohne Dachpappe", ""),
            Some(("zink", "alt"))
        );
        assert_eq!(grade_for("Lötzinn 40 %", ""), Some(("loetzinn", "Lötzinn")));
        assert_eq!(
            grade_for("Zinngeschirr 95 % mit Stempel", ""),
            Some(("zinn-geschirr", "Geschirr"))
        );
        assert_eq!(
            grade_for("Zinn rein 99 %, Sofortanalyse", ""),
            Some(("zinn", "rein"))
        );
        assert_eq!(
            grade_for("Altmetall / Mischschrott", ""),
            Some(("mischschrott", ""))
        );
        assert_eq!(
            grade_for("Altmetall / Mischschrott", "ab 1000 kg"),
            Some(("mischschrott", "ab 1000 kg"))
        );
        assert_eq!(
            grade_for("Bremsscheiben", ""),
            Some(("eisenschrott-gussbruch", "Bremsscheiben"))
        );
        assert_eq!(
            grade_for("Elektromotoren sauber", ""),
            Some(("elektromotoren", ""))
        );
        assert_eq!(
            grade_for("Elektromotoren mit Anhaftungen", ""),
            Some(("elektromotoren", "mit Anhaftungen"))
        );
        // No catalog material: paper, phones.
        assert_eq!(grade_for("Gemischtes Papier / Illustrierte", ""), None);
        assert_eq!(grade_for("Streifenschnitt", ""), None);
        assert_eq!(grade_for("Bücher", ""), None);
        assert_eq!(grade_for("Handys ohne Akku", ""), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h2>Angaben gemäß § 5 DDG</h2><p><strong>MK-Wertstoffhandel</strong><br>\
            Betreiber: Daniel Hartwig<br>DH Servicedienstleistungen<br>Einzelunternehmer<br>\
            Grabensprung 1<br>12683 Berlin</p>\
            <h2>Kontakt</h2><p>Telefon:<a href=\"tel:+4915251605977\">01525 160 59 77</a><br>\
            E-Mail:<a href=\"mailto:dh-servicedienstleistungen@gmx.net\">dh-servicedienstleistungen@gmx.net</a></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Grabensprung 1");
        assert_eq!(info.postcode, "12683");
        assert_eq!(info.city, "Berlin");
        assert_eq!(info.phone, "+4915251605977");
        assert_eq!(info.email, "dh-servicedienstleistungen@gmx.net");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(
            extract_info("<h2>Angaben gemäß § 5 DDG</h2><p>Grabensprung 1<br>12683 Berlin</p>")
                .is_err(),
            "missing Kontakt heading errors"
        );
    }
}
