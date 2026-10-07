//! Vedder & Stockrahm (Delmenhorst-Filiale): teilt sich die Bremer
//! Preisseite (`/ankauf/`), daher gleiche Tabellenform wie `vedder.rs`.
//! Unterschiede hier: eigene Slug/URLs (ohne `www`), leere Preiszellen
//! und `0,00`-Zeilen werden laut geskippt (nie als Preis 0 erfasst), und
//! die E-Mail kommt als Hex-mailto (`&#x6d;&#97;…`) und wird aus dem
//! `mailto:`-href gelesen. Hartmetall hat einen Katalogeintrag und wird
//! gemappt; versilberte Waren haben keinen und werden laut geskippt.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "ni-delmenhorst-vedder-stockrahm";
/// Bespoke, live-verified price URL (no `www`). A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://vedder-stockrahm.de/ankauf/";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://vedder-stockrahm.de/impressum/";

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
/// skipped. The variant keeps the trader's own grade wording.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kerze") {
        Some(("kupfer-berry", "Kerze"))
    } else if l.contains("raff") && !l.contains("kabel") {
        Some(("kupfer-gemischt", "Alt"))
    } else if l.contains("kabel") && l.contains("kupfer") {
        Some(("kabel-kupfer", "38%"))
    } else if l.contains("rotguss") || l.contains("bronze") {
        Some(("bronze-rotguss", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", ""))
    } else if l.contains("v2a") || l.contains("edelstahl") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("hartmetall") || l.contains("widia") {
        Some(("hartmetall", ""))
    } else if l.contains("profile") {
        Some(("aluminium-profile", ""))
    } else if l.contains("blech") {
        Some(("aluminium-blech", ""))
    } else if l.contains("guss") && l.contains("alu") {
        Some(("aluminium-guss", ""))
    } else if l.contains("zinn") {
        // Grades: the range IS the grade ("Zinn 80% - 98% (Geschirr)" …).
        if l.contains("80%") {
            Some(("zinn-geschirr", "80-98%"))
        } else if l.contains("70%") {
            Some(("zinn-geschirr", "70-79%"))
        } else if l.contains("60%") {
            Some(("zinn-geschirr", "60-69%"))
        } else if l.contains("50%") {
            Some(("zinn-geschirr", "50-59%"))
        } else {
            Some(("zinn", ""))
        }
    } else if l.contains("geschirr") {
        Some(("aluminium-blech", "Geschirr"))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after
/// the "Anschrift" heading holds firm lines + street + PLZ city, and the
/// `<dl>` carries labeled Telefon/E-Mail rows. The E-Mail lives behind a
/// hex-encoded mailto (`&#x6d;&#97;…`), so it is read from the link href,
/// falling back to the cell text. Missing anchors mean the page changed
/// shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h3 = Selector::parse("h3").expect("valid selector");
    let dt = Selector::parse("dt").expect("valid selector");
    let dd = Selector::parse("dd").expect("valid selector");
    let link = Selector::parse("a").expect("valid selector");
    let anchor = doc
        .select(&h3)
        .find(|h| h.text().collect::<String>().trim() == "Anschrift");
    let Some(anchor) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Anschrift-Block fehlt".to_owned(),
        });
    };
    // Address lines: first <p> sibling after the heading.
    let addr_p = anchor
        .next_siblings()
        .filter_map(ElementRef::wrap)
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
    // "Senator-Bömers Straße 10" / "28197 Bremen" (last two lines).
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
    // Labeled contact rows, paired by document order.
    let mut phone = String::new();
    let mut email = String::new();
    let dts: Vec<ElementRef> = doc.select(&dt).collect();
    let dds: Vec<ElementRef> = doc.select(&dd).collect();
    for (t, d) in dts.iter().zip(dds.iter()) {
        let label: String = t.text().collect();
        let value: String = d.text().collect();
        let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
        match label.trim() {
            "Telefon" if phone.is_empty() => phone = value,
            "E-Mail" if email.is_empty() => {
                // Hex-mailto quirk: the address is entity-encoded, so take
                // the decoded mailto href over the cell text.
                email = d
                    .select(&link)
                    .filter_map(|a| a.value().attr("href"))
                    .find_map(|h| h.strip_prefix("mailto:"))
                    .map(str::trim)
                    .filter(|h| h.contains('@'))
                    .unwrap_or(&value)
                    .to_owned();
            }
            _ => {}
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
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tbody tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
    let h2 = Selector::parse("h2").expect("valid selector");
    let mut published_at = None;
    for el in doc.select(&h2) {
        let text: String = el.text().collect();
        // "Unverbindliche Ankaufspreise28.09.2026"
        if let Some(date) = text.split("Ankaufspreise").nth(1) {
            let parts: Vec<&str> = date.trim().split('.').collect();
            if parts.len() == 3 {
                published_at = parse_de_date(parts[0], parts[1], parts[2]);
            }
        }
    }
    // Never trust page order: take the table carrying the price header,
    // not just the first <table> on the page.
    let table = doc.select(&table).find(|t| {
        t.select(&head).any(|h| {
            h.text()
                .collect::<String>()
                .to_lowercase()
                .contains("material")
        })
    });
    let Some(table) = table else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    };
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for tr in table.select(&row) {
        let cells: Vec<String> = tr.select(&cell).map(|c| c.text().collect()).collect();
        if cells.len() < 3 {
            continue;
        }
        let label = cells[0].trim().replace(['\u{a0}'], " ");
        // Empty price cell: loud skip, never a silent zero.
        let Some(price) = parse_eur(&cells[1]) else {
            skipped.push(format!("{label} (kein Preis: {})", cells[1].trim()));
            continue;
        };
        // A "0,00" row is "no quote", not a free gift: loud skip.
        if price == 0.0 {
            skipped.push(format!("{label} (Preis 0,00)"));
            continue;
        }
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let Some(unit) = unit_of(&cells[2]) else {
            skipped.push(format!(
                "{label} (Einheit unverständlich: {})",
                cells[2].trim()
            ));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    Ok((published_at, rows, skipped))
}

/// Bespoke unit matcher for THIS table's Einheit column (live: "EUR /
/// KG"; tonne rows would read "EUR / T"). Only kg/t exist here — anything
/// else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|t| t == "t")
    {
        Some("EUR/t")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    // Real excerpt of the live page (28.09.2026), trimmed to representative
    // rows. Entities kept verbatim (`&#160;`, `<br />` aside shape).
    const FIXTURE: &str = "<h2>Unverbindliche Ankaufspreise28.09.2026</h2>\
        <table><thead><tr><th>Materialbezeichnung</th><th>Preis</th><th>Einheit</th></tr></thead>\
        <tbody><tr><td>Kupfer-Kabel blank (Millberry)</td><td>9,80</td><td>EUR / KG</td></tr>\
        <tr><td>Kupfer blank (Kerze)</td><td>9,70</td><td>EUR / KG</td></tr>\
        <tr><td>Kupfer-Kabel 38&#160;% Inhalt</td><td>3,30</td><td>EUR / KG</td></tr>\
        <tr><td>Rotguss Stücke sauber</td><td>7,90</td><td>EUR / KG</td></tr>\
        <tr><td>Messing gemischt</td><td>4,90</td><td>EUR / KG</td></tr>\
        <tr><td>Edelstahlabfälle V4A</td><td>1,55</td><td>EUR / KG</td></tr>\
        <tr><td>Alu-Profile Blank, kurz</td><td>1,85</td><td>EUR / KG</td></tr>\
        <tr><td>Hartmetall Widia Platten und Bohrer</td><td>30,00</td><td>EUR / KG</td></tr>\
        <tr><td>Zinn 80% - 98% (Geschirr)</td><td>14,00</td><td>EUR / KG</td></tr>\
        <tr><td>Versilberte Messer</td><td>11,50</td><td>EUR / KG</td></tr>\
        </tbody></table>";

    #[test]
    fn table_and_date_parse() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-28T00:00:00+00:00"));
        assert_eq!(rows.len(), 10);
        assert!(skips.is_empty());
        assert_eq!(rows[0].0, "Kupfer-Kabel blank (Millberry)");
        assert_eq!(rows[0].1, 9.8);
        assert_eq!(rows[0].2, "EUR/kg");
        assert_eq!(rows[7].1, 30.0);
        assert_eq!(rows[8].1, 14.0);
    }

    #[test]
    fn empty_and_zero_price_cells_skip_loudly() {
        let html = FIXTURE.replacen("<td>9,80</td>", "<td></td>", 1).replacen(
            "<td>9,70</td>",
            "<td>0,00</td>",
            1,
        );
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 8);
        assert_eq!(skips.len(), 2);
        assert!(skips[0].contains("Millberry") && skips[0].contains("kein Preis"));
        assert!(skips[1].contains("Kerze") && skips[1].contains("0,00"));
    }

    #[test]
    fn wrong_table_and_unit_are_rejected_loudly() {
        // A layout table before the price table must not win.
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (_, rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 10);
        // Unknown unit: skipped loudly, valid rows survive.
        let html = FIXTURE.replacen("EUR / KG", "pro Sack", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 9);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Millberry"));
        // Every row unparseable: loud error, not silent success.
        let html = FIXTURE.replace("EUR / KG", "pro Sack");
        let err = parse(&html).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn impressum_extracts_contact_with_hex_mailto() {
        // Real fragment shape: hex-encoded mailto href AND link text.
        let imp = "<h3>Anschrift</h3><p>Vedder &#038; Stockrahm GmbH &#038; Co. KG<br />\
            Altmetallrecycling &#038; Containerdienst<br />\
            Senator-Bömers Straße 10<br />28197 Bremen</p>\
            <h3>Kontakt</h3><dl><dt>Telefon</dt><dd>+49 (0) 421&#160;54&#160;25&#160;54</dd>\
            <dt>Fax</dt><dd>+49 (0) 421&#160;54&#160;25&#160;53</dd>\
            <dt>E-Mail</dt><dd><a href=\"&#x6d;&#97;&#105;&#x6c;&#116;&#111;&#x3a;&#x69;&#110;&#x66;&#x6f;&#64;&#118;&#x65;&#100;&#100;&#x65;&#x72;&#45;&#x73;&#x74;&#111;&#x63;&#x6b;&#114;&#97;&#x68;&#x6d;&#46;&#x64;&#x65;\">&#x69;&#110;&#x66;&#x6f;&#64;&#118;&#x65;&#100;&#100;&#x65;&#x72;&#45;&#x73;&#x74;&#111;&#x63;&#x6b;&#114;&#97;&#x68;&#x6d;&#46;&#x64;&#x65;</a></dd></dl>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Senator-Bömers Straße 10");
        assert_eq!(info.postcode, "28197");
        assert_eq!(info.city, "Bremen");
        assert_eq!(info.phone, "+49 (0) 421 54 25 54");
        assert_eq!(info.email, "info@vedder-stockrahm.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }

    #[test]
    fn mapping_covers_live_table() {
        assert_eq!(
            grade_for("Kupfer-Kabel blank (Millberry)"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Kupfer blank (Kerze)"),
            Some(("kupfer-berry", "Kerze"))
        );
        assert_eq!(
            grade_for("Kupfer Raff  (Alt)"),
            Some(("kupfer-gemischt", "Alt"))
        );
        assert_eq!(
            grade_for("Kupfer-Kabel 38 % Inhalt"),
            Some(("kabel-kupfer", "38%"))
        );
        assert_eq!(
            grade_for("Rotguss Stücke sauber"),
            Some(("bronze-rotguss", ""))
        );
        assert_eq!(grade_for("Messing gemischt"), Some(("messing", "")));
        assert_eq!(grade_for("Alt-Blei"), Some(("blei", "")));
        assert_eq!(grade_for("Alt-Zink"), Some(("zink", "")));
        assert_eq!(
            grade_for("Edelstahlabfälle V2A"),
            Some(("edelstahl-v2a", ""))
        );
        assert_eq!(
            grade_for("Edelstahlabfälle V4A"),
            Some(("edelstahl-v4a", ""))
        );
        assert_eq!(
            grade_for("Alu-Geschirr alt 2 % FE"),
            Some(("aluminium-blech", "Geschirr"))
        );
        assert_eq!(
            grade_for("Alu-Profile Blank, kurz"),
            Some(("aluminium-profile", ""))
        );
        assert_eq!(
            grade_for("Alu-Bleche neu Blank"),
            Some(("aluminium-blech", ""))
        );
        assert_eq!(
            grade_for("Alu-Guss 5 % Anhaftung"),
            Some(("aluminium-guss", ""))
        );
        assert_eq!(
            grade_for("Hartmetall Widia Platten und Bohrer"),
            Some(("hartmetall", ""))
        );
        assert_eq!(
            grade_for("Zinn 80% - 98% (Geschirr)"),
            Some(("zinn-geschirr", "80-98%"))
        );
        assert_eq!(
            grade_for("Zinn 70% - 79%"),
            Some(("zinn-geschirr", "70-79%"))
        );
        assert_eq!(
            grade_for("Zinn 60% - 69%"),
            Some(("zinn-geschirr", "60-69%"))
        );
        assert_eq!(
            grade_for("Zinn 50% - 59%"),
            Some(("zinn-geschirr", "50-59%"))
        );
        // No catalog material for silver-plated cutlery (silber is EUR/g).
        assert_eq!(grade_for("Versilbertes Besteck 90 / 100"), None);
        assert_eq!(grade_for("Versilberte Messer"), None);
    }
}
