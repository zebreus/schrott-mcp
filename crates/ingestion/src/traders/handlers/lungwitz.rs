//! Volker Lungwitz Schrotthandel e.K. (Frankenberg): exact net purchase
//! prices ("Einkaufspreise … netto frei Lager Frankenberg") in a single
//! TablePress table (`tablepress-7`). The thead carries the list date
//! ("Preisliste vom" + "24.09.26 12:03"); every price cell quotes its own
//! unit inline ("€/kg" / "€/t"). Label-only rows are section headers
//! (Fe-Metalle, Kupfer, …) and carry no price. Paper, mixed coolers,
//! batteries and transformers have no catalog material and are skipped
//! loudly.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sn-frankenberg-volker-lungwitz-schrotthandel";
/// Bespoke, live-verified impressum URL (site nav "Impressum"). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://vlschrott.de/impressum/";

pub const URL: &str = "https://vlschrott.de/einkaufspreise/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
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
/// skipped. Notes on judgement calls:
/// - "Cu Kanal 95% / Cu verzinnt" names two grades at one price; the
///   generic parent (kupfer-gemischt) covers both without false precision.
/// - "Al Blech / Guß …" mixes sheet and cast; the generic parent covers
///   the mix, the Fe share stays in the variant.
/// - "Chromstahl" is unstamped stainless → generic edelstahl-gemischt,
///   never a guessed V2A/V4A.
/// - Mixed coolers (Cu-Ms, Al-Cu), batteries, transformers and paper have
///   no catalog material → None (proposals, not crammed).
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // No-catalog guards first (proposal material, never crammed).
    if l.contains("kühler") || l.contains("kuehler") {
        return None;
    }
    if l.contains("batterie") || l.contains("bleiakku") {
        return None;
    }
    if l.contains("trafo") {
        return None;
    }
    // Paper has no catalog material — but the lead-sheathed copper cable
    // ("… m. Papier") is a cable grade, not paper.
    if (l.contains("zeitung") || l.contains("papier")) && !l.contains("kabel") {
        return None;
    }
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("lackdraht") || l.contains("berry") {
        Some(("kupfer-berry", ""))
    } else if l.contains("cu raff") || (l.contains("raff") && l.contains("92")) {
        Some(("kupfer-gemischt", "Raff 92%"))
    } else if l.contains("kanal") || l.contains("verzinnt") {
        Some(("kupfer-gemischt", "Kanal 95%/verzinnt"))
    } else if l.contains("cu schwer neu") || l.contains("cu schwer, neu") {
        Some(("kupfer-gemischt", "schwer neu"))
    } else if l.contains("cu schwer") {
        Some(("kupfer-gemischt", "schwer"))
    } else if l.contains("oberleitung") || (l.contains("freileitung") && l.contains("cu")) {
        Some(("kupfer-gemischt", "Oberleitung/Freileitung"))
    } else if l.contains("schlitzkabel") && l.contains("cu") {
        Some(("kabel-kupfer", "60%"))
    } else if l.contains("shredderkabel") && l.contains("cu") {
        Some(("kabel-kupfer", "Shredder"))
    } else if l.contains("cu-pb") || l.contains("cu–pb") {
        Some(("kabel-kupfer", "Cu-Pb Papier"))
    } else if l.contains("schlitzkabel") {
        Some(("kabel-alu", "60%"))
    } else if l.contains("rotgu") {
        Some(("bronze-rotguss", ""))
    } else if l.contains("erodierdraht") {
        Some(("messing", "Erodierdraht"))
    } else if l.contains("ms 58") {
        Some(("messing", "Ms 58"))
    } else if l.contains("ms 63") {
        Some(("messing", "Ms 63"))
    } else if l.contains("ms raff sp") {
        Some(("messing", "Raff Späne"))
    } else if l.contains("ms raff") || l.contains("messing raff") {
        Some(("messing", "Raff"))
    } else if l.contains("ms schwer") || l.contains("messing schwer") {
        Some(("messing", "schwer"))
    } else if l.contains("offset") {
        Some(("aluminium-blech", "Offset"))
    } else if l.contains("profil") && l.contains("lack") {
        Some(("aluminium-profile", "lackiert"))
    } else if l.contains("iso") && l.contains("profil") {
        Some(("aluminium-profile", "ISO"))
    } else if l.contains("profil") {
        Some(("aluminium-profile", "blank"))
    } else if l.contains("felgen") {
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("getriebemotoren") {
        Some(("elektromotoren", "Getriebe"))
    } else if l.contains("getriebe") {
        Some(("aluminium-guss", "Getriebe"))
    } else if l.contains("blech") && l.contains("20-50") {
        Some(("aluminium-gemischt", "Blech 20-50% Fe"))
    } else if l.contains("blech") && l.contains("10%") {
        Some(("aluminium-gemischt", "Blech/Guß 10% Fe"))
    } else if l.contains("blech") && l.contains("5%") {
        Some(("aluminium-gemischt", "Blech/Guß 5% Fe"))
    } else if l.contains("blech") {
        Some(("aluminium-gemischt", "Blech/Guß o. Fe"))
    } else if l.contains("al draht") || l.contains("alu draht") {
        Some(("aluminium-gemischt", "Draht blank"))
    } else if l.contains("freileitung") {
        Some(("aluminium-gemischt", "Freileitung o. Fe"))
    } else if l.contains("leitschien") {
        Some(("aluminium-gemischt", "Leitschienen"))
    } else if l.contains("späne") || l.contains("spaene") {
        Some(("aluminium-gemischt", "Späne"))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("kabelblei") {
        Some(("blei", "Kabelblei"))
    } else if l.contains("wuchtblei") {
        Some(("blei", "Wuchtblei"))
    } else if l.contains("altblei") {
        Some(("blei", ""))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", ""))
    } else if l.contains("chromstahl") {
        Some(("edelstahl-gemischt", ""))
    } else if l.contains("e-motor") || l.contains("emotor") || l.contains("e/motor") {
        Some(("elektromotoren", ""))
    } else if l.contains("kernschrott") {
        Some(("stahlschrott-scheren", "Kernschrott ab 6 mm"))
    } else if l.contains("scherenvormaterial schwer") {
        Some(("stahlschrott-scheren", "schwer"))
    } else if l.contains("scherenvormaterial leicht") {
        Some(("stahlschrott-scheren", "leicht"))
    } else if l.contains("bremsscheiben") {
        Some(("eisenschrott-gussbruch", "Bremsscheiben"))
    } else if l.contains("gußschrott") || l.contains("gussschrott") {
        Some(("eisenschrott-gussbruch", ""))
    } else if l.contains("shreddervormaterial") {
        Some(("stahlschrott-shredder", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the page has
/// `<p>Anschrift und Kontakt:</p>` followed by a firm/street/PLZ `<p>`
/// (`<br>` lines) and a `Telefon: …<br>E-Mail: …` `<p>`. Missing anchors
/// mean the page changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p = Selector::parse("p").expect("valid selector");
    let paras: Vec<ElementRef> = doc.select(&p).collect();
    let texts: Vec<String> = paras
        .iter()
        .map(|e| e.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" "))
        .collect();
    let anchor = texts.iter().position(|t| t == "Anschrift und Kontakt:");
    let Some(k) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Anschrift-Block fehlt".to_owned(),
        });
    };
    // Address lines from the next <p>'s <br> rows (scraper text() glues
    // "Mühlenstraße 7"+"09669 Frankenberg" without space, so split the
    // raw inner HTML instead).
    let addr_html = paras.get(k + 1).map(|e| e.inner_html()).unwrap_or_default();
    let mut lines = Vec::new();
    for part in addr_html.split("<br") {
        let t = strip_tags(part);
        if !t.is_empty() {
            lines.push(t);
        }
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if lines.len() >= 2 {
        street = lines[lines.len() - 2].clone();
        let last = lines[lines.len() - 1].clone();
        let mut it = last.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| a + " " + b);
            }
        }
    }
    // Phone + mail from the <p> after the address. Cut at the next
    // label first: scraper text() glues "82055"+"E-Mail:" into one
    // token and the digit filter would stop one token early.
    let contact = texts.get(k + 2).cloned().unwrap_or_default();
    let phone = {
        let after = after_marker(&contact, "Telefon:");
        let end = ["Mobil:", "Telefax:", "Fax:", "E-Mail:", "Internet:"]
            .iter()
            .filter_map(|m| after.find(m))
            .min()
            .unwrap_or(after.len());
        after[..end]
            .split_whitespace()
            .take_while(|t| t.chars().all(|c| c.is_ascii_digit() || "+/().-".contains(c)))
            .collect::<Vec<_>>()
            .join(" ")
    };
    // E-mail needs its own rule: the phone-style filter stops at the
    // first letter, so take the @ token instead.
    let email = contact
        .split_whitespace()
        .find(|t| t.contains('@'))
        .unwrap_or_default()
        .trim_matches([',', ';', '.'])
        .to_owned();
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo { street, postcode, city, phone, email })
}

fn after_marker<'a>(text: &'a str, marker: &str) -> &'a str {
    text.find(marker).map(|i| &text[i + marker.len()..]).unwrap_or("")
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
) -> Result<(Option<String>, Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tbody tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
    // Never trust page order: take the table carrying the "Preisliste
    // vom" header, not just the first <table> on the page.
    let table = doc.select(&table).find(|t| {
        t.select(&head).any(|h| {
            h.text().collect::<String>().to_lowercase().contains("preisliste vom")
        })
    });
    let Some(table) = table else {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "keine Preistabelle".to_owned() });
    };
    // List date from the header cell next to "Preisliste vom"
    // (live: "24.09.26 12:03" — day.month.2-digit-year, time ignored).
    let mut published_at = None;
    let heads: Vec<String> = table.select(&head).map(|h| h.text().collect()).collect();
    if heads.len() >= 2 {
        let date = heads[1].split_whitespace().next().unwrap_or("");
        let parts: Vec<&str> = date.split('.').collect();
        if parts.len() == 3 && parts[2].len() == 2 {
            published_at = parse_de_date(parts[0], parts[1], &format!("20{}", parts[2]));
        }
    }
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in table.select(&row) {
        let cells: Vec<String> = tr.select(&cell).map(|c| c.text().collect()).collect();
        if cells.len() < 2 {
            continue;
        }
        let label = cells[0].replace(['\u{a0}', '\u{feff}'], " ");
        let label = label.split_whitespace().collect::<Vec<_>>().join(" ");
        let price_raw = cells[1].replace('\u{a0}', " ");
        // Label-only rows are section headers (Fe-Metalle, Kupfer, …),
        // not prices — structural, not skips.
        if price_raw.trim().is_empty() {
            continue;
        }
        let Some(price) = parse_eur(&price_raw) else {
            skips.push(format!("{label} (kein Preis: {})", price_raw.trim()));
            continue;
        };
        if price == 0.0 {
            skips.push(format!("{label} (0,00 — kein Ankauf)"));
            continue;
        }
        // An unparseable unit is a loud skip, never a silent default: a
        // per-tonne price recorded as per-kg would be a 1000x error.
        let Some(unit) = unit_of(&price_raw) else {
            skips.push(format!("{label} (Einheit unverständlich: {})", price_raw.trim()));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Preistabelle leer".to_owned() });
    }
    Ok((published_at, rows, skips))
}

/// Bespoke unit matcher for THIS table: the unit rides inline in the
/// price cell (live: "9,15 €/kg", "170,00 €/t"). Only kg/t exist here —
/// anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("/kg") {
        Some("EUR/kg")
    } else if lower.contains("/t") {
        Some("EUR/t")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    // Real table shape, condensed: TablePress ids/classes, thead date,
    // empty structural rows, section headers, inline units.
    const FIXTURE: &str = "<table id=\"tablepress-7\" class=\"tablepress tablepress-id-7\">\
        <thead><tr class=\"row-1\"><th class=\"column-1\">Preisliste vom</th>\
        <th class=\"column-2\">24.09.26 12:03</th></tr></thead>\
        <tbody class=\"row-striping row-hover\">\
        <tr class=\"row-2\"><td class=\"column-1\"></td><td class=\"column-2\"></td></tr>\
        <tr class=\"row-3\"><td class=\"column-1\">Zeitungspapier</td><td class=\"column-2\">0,07 \u{20ac}/kg</td></tr>\
        <tr class=\"row-5\"><td class=\"column-1\">Fe-Metalle</td><td class=\"column-2\"></td></tr>\
        <tr class=\"row-7\"><td class=\"column-1\">Kernschrott ab 6 mm</td><td class=\"column-2\">170,00 \u{20ac}/t</td></tr>\
        <tr class=\"row-17\"><td class=\"column-1\">Cu Raff 92%</td><td class=\"column-2\">9,15 \u{20ac}/kg</td></tr>\
        <tr class=\"row-18\"><td class=\"column-1\">Cu Kanal 95% / Cu verzinnt</td><td class=\"column-2\">9,35 \u{20ac}/kg</td></tr>\
        <tr class=\"row-23\"><td class=\"column-1\">Cu Millberry</td><td class=\"column-2\">11,30 \u{20ac}/kg</td></tr>\
        <tr class=\"row-32\"><td class=\"column-1\">Rotguß</td><td class=\"column-2\">8,10 \u{20ac}/kg</td></tr>\
        <tr class=\"row-33\"><td class=\"column-1\">Cu-Ms Kühler o. Fe</td><td class=\"column-2\">4,95 \u{20ac}/kg</td></tr>\
        <tr class=\"row-60\"><td class=\"column-1\">V2A</td><td class=\"column-2\">0,65 \u{20ac}/kg</td></tr>\
        <tr class=\"row-64\"><td class=\"column-1\">E-motoren bis 300 kg/St.</td><td class=\"column-2\">0,82 \u{20ac}/kg</td></tr>\
        <tr class=\"row-70\"><td class=\"column-1\">Al Shredderkabel</td><td class=\"column-2\">0,00 \u{20ac}/kg</td></tr>\
        </tbody></table>";

    #[test]
    fn table_and_date_parse() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-24T00:00:00+00:00"));
        // 10 price rows minus the 0,00 no-purchase row.
        assert_eq!(rows.len(), 9, "{rows:?}");
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Shredderkabel") && skips[0].contains("kein Ankauf"));
        assert_eq!(rows[0].0, "Zeitungspapier");
        assert_eq!(rows[0].1, 0.07);
        assert_eq!(rows[0].2, "EUR/kg");
        let kern = rows.iter().find(|r| r.0.contains("Kernschrott")).expect("kern");
        assert_eq!((kern.1, kern.2), (170.0, "EUR/t"));
    }

    #[test]
    fn wrong_table_and_unit_are_rejected_loudly() {
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (_, rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 9);
        let html = FIXTURE.replacen("\u{20ac}/kg", "pro Sack", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 8);
        assert_eq!(skips.len(), 2);
        assert!(skips[0].contains("Einheit unverständlich"));
        let html = FIXTURE.replace("\u{20ac}/kg", "pro Sack").replace("\u{20ac}/t", "pro Sack");
        let err = parse(&html).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
        assert!(parse("<html><body>keine Tabelle</body></html>").is_err());
    }

    #[test]
    fn mapping_covers_every_fixture_label() {
        assert_eq!(grade_for("Cu Raff 92%"), Some(("kupfer-gemischt", "Raff 92%")));
        assert_eq!(
            grade_for("Cu Kanal 95% / Cu verzinnt"),
            Some(("kupfer-gemischt", "Kanal 95%/verzinnt"))
        );
        assert_eq!(grade_for("Cu Millberry"), Some(("kupfer-millberry", "")));
        assert_eq!(grade_for("Cu Lackdraht / Berry"), Some(("kupfer-berry", "")));
        assert_eq!(grade_for("Rotguß"), Some(("bronze-rotguss", "")));
        assert_eq!(grade_for("V2A"), Some(("edelstahl-v2a", "")));
        assert_eq!(grade_for("Chromstahl"), Some(("edelstahl-gemischt", "")));
        assert_eq!(grade_for("E-motoren bis 300 kg/St."), Some(("elektromotoren", "")));
        assert_eq!(grade_for("Getriebemotoren"), Some(("elektromotoren", "Getriebe")));
        assert_eq!(grade_for("Kernschrott ab 6 mm"), Some(("stahlschrott-scheren", "Kernschrott ab 6 mm")));
        // Loud skips: no catalog material.
        assert_eq!(grade_for("Zeitungspapier"), None);
        assert_eq!(grade_for("Cu-Ms Kühler o. Fe"), None);
        assert_eq!(grade_for("Al-Cu Kühler o. Fe"), None);
        assert_eq!(grade_for("Batterieblei / Bleiakkus"), None);
        assert_eq!(grade_for("Cu Trafos"), None);
        // The paper guard must not catch the lead-sheathed cable grade.
        assert_eq!(
            grade_for("Cu-Pb Kabel m. Papier ab 5 cm"),
            Some(("kabel-kupfer", "Cu-Pb Papier"))
        );
        // Specific-before-generic: motor guard wins over Alu-Getriebe.
        assert_eq!(grade_for("Al Getriebe"), Some(("aluminium-guss", "Getriebe")));
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h1 class=\"page-title\">Impressum</h1><div class=\"post-content\">\
            <p class=\"wp-block-paragraph\">Anschrift und Kontakt:</p>\
            <p class=\"wp-block-paragraph\">Volker Lungwitz Schrotthandel e.K.<br>Mühlenstraße 7<br>09669 Frankenberg</p>\
            <p class=\"wp-block-paragraph\">Telefon: 037206 82055<br>E-Mail: info@VLSchrott.de</p>\
            <p class=\"wp-block-paragraph\">Inhaber: Herr Matthias Lungwitz</p></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Mühlenstraße 7");
        assert_eq!(info.postcode, "09669");
        assert_eq!(info.city, "Frankenberg");
        assert_eq!(info.phone, "037206 82055");
        assert_eq!(info.email, "info@VLSchrott.de");
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
