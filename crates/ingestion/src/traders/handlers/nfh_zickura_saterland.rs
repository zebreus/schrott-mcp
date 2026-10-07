//! NFH Zickura GmbH (Saterland): single price table (`/preisliste-1`)
//! with header `Material 03.08.2026` / `€/kg` — the date lives in the
//! `<th>`, there are no headings on the page. One table only, no double
//! blocks. Tyre rows (`10€+19%`) are disposal fees, not purchase prices,
//! and skip loudly by label. Mixed assemblies (`Motoren und Getriebe`,
//! `Alu-Kupfer-Kühler`, `Kupfer-Messing-Kühler`, `Altauto …`, `Batterien`)
//! map to nothing. The site has no impressum (sitemap lists only `/`,
//! `/preisliste-1`, `/kontakt`), so contact comes from the bespoke
//! `/kontakt` page: `Tel.:`/`E-Mail.:` paragraphs plus the `<p>` behind
//! the `Standort` heading (map `data-location` corroborates the address).

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "ni-saterland-nfh-zickura";
/// Bespoke, live-verified price URL (non-www, as listed). A move fails the
/// step loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://nfh-zickuragmbh.de/preisliste-1";
/// Bespoke, live-verified contact URL. The site has no impressum page
/// (sitemap 28.09.2026: only `/`, `/preisliste-1`, `/kontakt`), so the
/// Kontakt page is the contact source. A move fails loudly.
pub const CONTACT_URL: &str = "https://nfh-zickuragmbh.de/kontakt";

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
    // Contact failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, contact_html) = fetch_text(client, CONTACT_URL).await?;
    let trader_info = extract_info(&contact_html)?;
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
/// skipped. Arms are specific-before-generic (`unsauber` before `sauber`,
/// `Iso Neu` before `Neu`, cooler mixes first). The variant keeps the
/// trader's own grade wording; `''` = standard grade.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Mixed-metal coolers and assemblies: no single catalog material.
    if l.contains("kühler")
        && (l.contains("messing") || (l.contains("alu") && l.contains("kupfer")))
    {
        return None;
    }
    if l.contains("getriebe") || l.contains("altauto") || l.contains("batterie") {
        return None;
    }
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("e-motor") {
        // "unsauber" contains "sauber" — check it first.
        if l.contains("unsauber") {
            Some(("elektromotoren", "unsauber"))
        } else if l.contains("sauber") {
            Some(("elektromotoren", "sauber"))
        } else {
            Some(("elektromotoren", ""))
        }
    } else if l.contains("kat ") || l == "kat" {
        if l.contains("klein") {
            Some(("katalysatoren", "klein"))
        } else if l.contains("mittel") {
            Some(("katalysatoren", "mittel"))
        } else if l.contains("groß") || l.contains("gross") {
            Some(("katalysatoren", "groß"))
        } else {
            Some(("katalysatoren", ""))
        }
    } else if l.contains("erdkabel") {
        if l.contains("alu") {
            Some(("kabel-alu", "Erdkabel"))
        } else {
            Some(("kabel-kupfer", "Erdkabel"))
        }
    } else if l.contains("alu-kabel") {
        Some(("kabel-alu", ""))
    } else if l.contains("schälkabel") || l.contains("schalkabel") {
        Some(("kabel-kupfer", "Schälkabel"))
    } else if l.contains("kabel") && l.contains("unsauber") {
        Some(("kabel-kupfer", "unsauber"))
    } else if l.contains("kabel") && l.contains("sauber") {
        Some(("kabel-kupfer", "sauber"))
    } else if l.contains("kupfer-iso") || l.contains("kupfer iso") {
        Some(("kabel-kupfer", "ISO"))
    } else if l.contains("kupfer-neu") || l.contains("kupfer neu") {
        Some(("kupfer-gemischt", "Neu"))
    } else if l.contains("kupfer-misch") || l.contains("kupfer misch") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("rotgu") {
        // Matches Rotguss/Rotguß (page spells Rotguß).
        Some(("bronze-rotguss", ""))
    } else if l.contains("messing") {
        if l.contains("spän") || l.contains("span") {
            Some(("messing", "Späne"))
        } else {
            Some(("messing", ""))
        }
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", ""))
    } else if l.contains("v2a") {
        if l.contains("spän") || l.contains("span") {
            Some(("edelstahl-v2a", "Späne"))
        } else {
            Some(("edelstahl-v2a", ""))
        }
    } else if l.contains("auswuch") {
        // Page spells "Auswuchblei" (Auswuchtblei typo kept in notes).
        Some(("blei", "Auswuchtblei"))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("zinn") {
        Some(("zinn", ""))
    } else if l.contains("alu-felgen") || l.contains("alu felgen") {
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("geschirr") {
        Some(("aluminium-blech", "Geschirr"))
    } else if l.contains("offset") {
        Some(("aluminium-blech", "Offset"))
    } else if l.contains("alu-spän") || l.contains("alu-span") || l.contains("alu spän") {
        Some(("aluminium-gemischt", "Späne"))
    } else if l.contains("alu-schredder") || l.contains("schredder") {
        Some(("aluminium-gemischt", "Schredder"))
    } else if l.contains("alu-neu") || l.contains("alu neu") {
        Some(("aluminium-gemischt", "Neu"))
    } else if l.contains("profil") {
        if l.contains("bunt") {
            Some(("aluminium-profile", "Bunt unsauber"))
        } else if l.contains("iso") {
            Some(("aluminium-profile", "Iso Neu"))
        } else if l.contains("neu") {
            Some(("aluminium-profile", "Neu"))
        } else {
            Some(("aluminium-profile", ""))
        }
    } else if l.contains("eisengu") {
        // Matches Eisenguss/Eisenguß.
        Some(("eisenschrott-gussbruch", ""))
    } else if l.contains("schrott-neu") || l.contains("schrott neu") {
        Some(("stahlschrott-sorte-1", ""))
    } else if l.contains("schrott-misch") || l.contains("schrott misch") {
        Some(("mischschrott", ""))
    } else if l.contains("schrott-schwer") || l.contains("schrott schwer") {
        // Schwerschrott ist Scherenschrott, kein Misch (FE-Audit).
        Some(("stahlschrott-scheren", "Schwer"))
    } else if l.contains("schrott-blech") || l.contains("schrott blech") {
        // Blech ist Shredder-Input, kein Misch (FE-Audit).
        Some(("stahlschrott-shredder", "Blech"))
    } else if l.contains("schrott-spän") || l.contains("schrott-spa") {
        Some(("mischschrott", "Späne"))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS kontakt page only: the `<p>`s carry
/// `Tel.:`/`E-Mail.:` prefixes, and the `<p>` after the `Standort`
/// heading holds street + PLZ city across a `<br>`. Missing anchors mean
/// the page changed shape → loud error, never a guessed fallback.
fn extract_info(contact: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(contact);
    let h1 = Selector::parse("h1").expect("valid selector");
    let h3 = Selector::parse("h3").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    if !doc
        .select(&h1)
        .any(|h| h.text().collect::<String>().trim() == "Kontaktiere uns")
    {
        return Err(IngestError::Parse {
            url: CONTACT_URL.to_owned(),
            detail: "Kontakt-Anker fehlt".to_owned(),
        });
    }
    let anchor = doc
        .select(&h3)
        .find(|h| h.text().collect::<String>().trim() == "Standort");
    if anchor.is_none() {
        return Err(IngestError::Parse {
            url: CONTACT_URL.to_owned(),
            detail: "Standort-Block fehlt".to_owned(),
        });
    }
    // The heading nests inside Webador divs, so no sibling walk: search
    // document-wide, but only behind the anchored heading. The address
    // <p> is the first one with a <br> after it.
    let mixed = Selector::parse("h3, p").expect("valid selector");
    let mut past = false;
    let mut addr_html: Option<String> = None;
    for el in doc.select(&mixed) {
        if el.value().name() == "h3" {
            if el.text().collect::<String>().trim() == "Standort" {
                past = true;
            }
        } else if past && addr_html.is_none() && el.inner_html().contains("<br") {
            addr_html = Some(el.inner_html());
        }
    }
    let Some(addr_html) = addr_html else {
        return Err(IngestError::Parse {
            url: CONTACT_URL.to_owned(),
            detail: "Standort-Adresse fehlt".to_owned(),
        });
    };
    let mut lines = Vec::new();
    for part in addr_html.split("<br") {
        let t = strip_tags(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    // "Rudolf-Diesel-Str. 11" / "26683 Saterland" (last two lines).
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if lines.len() >= 2 {
        let last = lines.last().expect("len checked");
        let mut it = last.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, w| a + " " + w);
                street = lines[lines.len() - 2].clone();
            }
        }
    }
    // Labeled contact paragraphs, document-wide but prefix-anchored.
    let mut phone = String::new();
    let mut email = String::new();
    for el in doc.select(&p) {
        let text: String = el.text().collect();
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if let Some(rest) = text.strip_prefix("Tel.:") {
            if phone.is_empty() {
                phone = rest.trim().to_owned();
            }
        } else if email.is_empty() && text.starts_with("E-Mail") {
            if let Some((_, rest)) = text.split_once(':') {
                let rest = rest.trim().to_owned();
                if rest.contains('@') {
                    email = rest;
                }
            }
        }
    }
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: CONTACT_URL.to_owned(),
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
    // The page date lives in the table header ("Material 03.08.2026"):
    // scan header cells for a dd.mm.yyyy token.
    let mut published_at = None;
    // Never trust page order: take the table carrying the price header,
    // not just the first <table> on the page.
    let price_table = doc.select(&table).find(|t| {
        t.select(&head).any(|h| {
            h.text()
                .collect::<String>()
                .to_lowercase()
                .contains("material")
        })
    });
    let Some(price_table) = price_table else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    };
    for h in price_table.select(&head) {
        let text: String = h.text().collect();
        for tok in text.split_whitespace() {
            let parts: Vec<&str> = tok
                .trim_matches(|c: char| !c.is_ascii_digit())
                .split('.')
                .collect();
            if parts.len() == 3 {
                if let Some(date) = parse_de_date(parts[0], parts[1], parts[2]) {
                    published_at = Some(date);
                    break;
                }
            }
        }
    }
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for tr in price_table.select(&row) {
        let cells: Vec<String> = tr.select(&cell).map(|c| c.text().collect()).collect();
        if cells.len() < 2 {
            continue;
        }
        let label = cells[0].trim().replace(['\u{a0}'], " ");
        let label = label.split_whitespace().collect::<Vec<_>>().join(" ");
        if label.is_empty() || label.len() > 120 {
            continue;
        }
        // Tyre rows are disposal fees ("10€+19%"), not purchase prices.
        if label.to_lowercase().contains("reifen") {
            skipped.push(format!(
                "{label} (kein Ankauf: Entsorgungsgebühr {})",
                cells[1].trim()
            ));
            continue;
        }
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
        let Some(unit) = unit_of(&cells[1]) else {
            skipped.push(format!(
                "{label} (Einheit unverständlich: {})",
                cells[1].trim()
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
    // Same (material, variant, price) twice = repeated block, not two
    // observations. The live page shows each grade once; dedup guards a
    // future repeat without hiding redesigns (0 rows still errors).
    let mut seen = std::collections::HashSet::new();
    rows.retain(|(label, price, _)| {
        let key = match grade_for(label) {
            Some((m, v)) => (m.to_owned(), v.to_owned(), price.to_bits()),
            None => (label.clone(), String::new(), price.to_bits()),
        };
        seen.insert(key)
    });
    Ok((published_at, rows, skipped))
}

/// Bespoke unit matcher for THIS table's price cells (live: `"8,50€ kg."`,
/// `"0,175 kg"`, `"25,00€ St."`, fees like `"10€+19%"`). Only kg/Stk exist
/// here — anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("kg") {
        Some("EUR/kg")
    } else if lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|t| t == "st" || t == "stk" || t == "stuck" || t == "stück" || t == "stueck")
    {
        Some("EUR/Stk")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    // Real excerpt of the live page (28.09.2026: Preisliste 03.08.2026),
    // shape kept verbatim (thead + whitespace-heavy cells).
    const FIXTURE: &str = "<table width=\"100%\" class=\"jw-table jw-table--header\">\
        <thead><tr><th width=\"50%\">Material 03.08.2026</th>\
        <th width=\"50%\">€/kg</th></tr></thead><tbody>\
        <tr><td width=\"50%\"> Schrott-Misch </td>\
        <td width=\"50%\"> 0,175 kg </td></tr>\
        <tr><td> Kupfer-Misch </td><td> 8,00€ kg. </td></tr>\
        <tr><td> Kupfer-Millberry </td><td> 8,50€ kg. </td></tr>\
        <tr><td> Kupfer-Kabel-unsauber </td><td> 2,00€ kg. </td></tr>\
        <tr><td> Messing </td><td> 4,20€ kg. </td></tr>\
        <tr><td> V2A </td><td> 0,80kg </td></tr>\
        <tr><td> Rotguß </td><td> 5,25€ kg. </td></tr>\
        <tr><td> E-Motor sauber </td><td> 0,75€ kg. </td></tr>\
        <tr><td> Alu-Kupfer-Kühler </td><td> 2,90€ kg. </td></tr>\
        <tr><td> Motoren und Getriebe </td><td> 0,240 kg </td></tr>\
        <tr><td> Kat klein </td><td> 25,00€ St. </td></tr>\
        <tr><td> Reifen klein </td><td> 10€+19% </td></tr>\
        <tr><td> Erdkabel-Kupfer </td><td> 1,10€ kg </td></tr>\
        </tbody></table>";

    #[test]
    fn table_and_date_parse() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-08-03T00:00:00+00:00"));
        // 13 rows minus the tyre fee = 12 parsed rows.
        assert_eq!(rows.len(), 12);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Reifen") && skips[0].contains("Entsorgungsgebühr"));
        assert_eq!(rows[0].0, "Schrott-Misch");
        assert_eq!(rows[0].1, 0.175);
        assert_eq!(rows[0].2, "EUR/kg");
        assert_eq!(rows[2].0, "Kupfer-Millberry");
        assert_eq!(rows[2].1, 8.5);
        // Kat row carries the per-piece unit.
        let kat = rows.iter().find(|r| r.0 == "Kat klein").expect("kat row");
        assert_eq!(kat.1, 25.0);
        assert_eq!(kat.2, "EUR/Stk");
    }

    #[test]
    fn empty_and_zero_price_cells_skip_loudly() {
        let html = FIXTURE
            .replacen("8,50€ kg.", "", 1)
            .replacen("4,20€ kg.", "0,00€ kg.", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 10);
        assert_eq!(skips.len(), 3);
        assert!(skips
            .iter()
            .any(|s| s.contains("Millberry") && s.contains("kein Preis")));
        assert!(skips
            .iter()
            .any(|s| s.contains("Messing") && s.contains("0,00")));
    }

    #[test]
    fn wrong_table_and_unit_are_rejected_loudly() {
        // A layout table before the price table must not win.
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (_, rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 12);
        // Unknown unit: skipped loudly, valid rows survive.
        let html = FIXTURE.replacen("8,50€ kg.", "8,50 pro Sack", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 11);
        assert_eq!(skips.len(), 2);
        assert!(skips.iter().any(|s| s.contains("Millberry")));
        // Every row unparseable: loud error, not silent success.
        let html = FIXTURE.replace("kg", "Sack").replace("St.", "Sack");
        let err = parse(&html).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn kontakt_extracts_contact() {
        // Real fragment shape: prefixed <p> rows + Standort address block.
        let kontakt = "<h1 class=\"jw-heading-130\">Kontaktiere uns</h1>\
            <div><p>&nbsp;</p><p>Tel.: 04498/9232777</p>\
            <p>Fax.: 04498/9232776</p>\
            <p>E-Mail.: info-nfh-zickura@t-online.de</p></div>\
            <h3 class=\"jw-heading-70\">Standort</h3>\
            <div><div><p>Rudolf-Diesel-Str. 11<br />26683 Saterland</p></div></div>";
        let info = extract_info(kontakt).expect("parses");
        assert_eq!(info.street, "Rudolf-Diesel-Str. 11");
        assert_eq!(info.postcode, "26683");
        assert_eq!(info.city, "Saterland");
        assert_eq!(info.phone, "04498/9232777");
        assert_eq!(info.email, "info-nfh-zickura@t-online.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }

    #[test]
    fn mapping_covers_live_table() {
        assert_eq!(grade_for("Schrott-Misch"), Some(("mischschrott", "")));
        assert_eq!(
            grade_for("Schrott-Schwer"),
            Some(("stahlschrott-scheren", "Schwer"))
        );
        assert_eq!(grade_for("Schrott-Neu"), Some(("stahlschrott-sorte-1", "")));
        assert_eq!(
            grade_for("Schrott-Blech"),
            Some(("stahlschrott-shredder", "Blech"))
        );
        assert_eq!(grade_for("Schrott-Späne"), Some(("mischschrott", "Späne")));
        assert_eq!(grade_for("Eisenguß"), Some(("eisenschrott-gussbruch", "")));
        assert_eq!(
            grade_for("Alu-Späne"),
            Some(("aluminium-gemischt", "Späne"))
        );
        assert_eq!(grade_for("Alu-Felgen"), Some(("aluminium-guss", "Felgen")));
        assert_eq!(
            grade_for("Alu-Geschirr"),
            Some(("aluminium-blech", "Geschirr"))
        );
        assert_eq!(
            grade_for("Alu-Schredder"),
            Some(("aluminium-gemischt", "Schredder"))
        );
        assert_eq!(grade_for("Alu-Neu"), Some(("aluminium-gemischt", "Neu")));
        assert_eq!(grade_for("Alu-Kabel"), Some(("kabel-alu", "")));
        assert_eq!(grade_for("Erdkabel-Alu"), Some(("kabel-alu", "Erdkabel")));
        assert_eq!(
            grade_for("Erdkabel-Kupfer"),
            Some(("kabel-kupfer", "Erdkabel"))
        );
        assert_eq!(grade_for("Kupfer-Misch"), Some(("kupfer-gemischt", "")));
        assert_eq!(grade_for("Kupfer-ISO"), Some(("kabel-kupfer", "ISO")));
        assert_eq!(grade_for("Kupfer-Neu"), Some(("kupfer-gemischt", "Neu")));
        assert_eq!(
            grade_for("Kupfer-Millberry"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Kupfer-Schälkabel"),
            Some(("kabel-kupfer", "Schälkabel"))
        );
        assert_eq!(
            grade_for("Kupfer-Kabel-unsauber"),
            Some(("kabel-kupfer", "unsauber"))
        );
        assert_eq!(
            grade_for("Kupfer-Kabel-sauber"),
            Some(("kabel-kupfer", "sauber"))
        );
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Messing-Späne"), Some(("messing", "Späne")));
        assert_eq!(grade_for("V2A"), Some(("edelstahl-v2a", "")));
        assert_eq!(grade_for("V4A"), Some(("edelstahl-v4a", "")));
        assert_eq!(grade_for("V2A-Späne"), Some(("edelstahl-v2a", "Späne")));
        assert_eq!(
            grade_for("E-Motor sauber"),
            Some(("elektromotoren", "sauber"))
        );
        assert_eq!(
            grade_for("E-Motor unsauber"),
            Some(("elektromotoren", "unsauber"))
        );
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(grade_for("Auswuchblei"), Some(("blei", "Auswuchtblei")));
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Zinn"), Some(("zinn", "")));
        assert_eq!(grade_for("Offset Alu"), Some(("aluminium-blech", "Offset")));
        assert_eq!(grade_for("Rotguß"), Some(("bronze-rotguss", "")));
        assert_eq!(grade_for("Profile Neu"), Some(("aluminium-profile", "Neu")));
        assert_eq!(
            grade_for("Profile Iso Neu"),
            Some(("aluminium-profile", "Iso Neu"))
        );
        assert_eq!(
            grade_for("Profile Bunt  (unsauber)"),
            Some(("aluminium-profile", "Bunt unsauber"))
        );
        assert_eq!(grade_for("Kat klein"), Some(("katalysatoren", "klein")));
        assert_eq!(grade_for("Kat mittel"), Some(("katalysatoren", "mittel")));
        assert_eq!(grade_for("Kat groß"), Some(("katalysatoren", "groß")));
        // Mixed assemblies and non-catalog rows skip loudly.
        assert_eq!(grade_for("Motoren und Getriebe"), None);
        assert_eq!(grade_for("Alu-Kupfer-Kühler"), None);
        assert_eq!(grade_for("Kupfer-Messing-Kühler"), None);
        assert_eq!(grade_for("Altauto mit Motor"), None);
        assert_eq!(grade_for("Altauto ohne Motor"), None);
        assert_eq!(grade_for("Batterien"), None);
    }
}
