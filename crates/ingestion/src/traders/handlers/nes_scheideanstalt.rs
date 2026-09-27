//! NES Scheideanstalt Hamburg (Altstadt branch of Norddeutsche
//! Edelmetall Scheideanstalt GmbH): exact fine-metal prices in a
//! server-rendered wpDataTable on /edelmetallpreise/ ("Feinmetalle" header
//! selects the table, never the first). Columns: Datum (live empty) |
//! Feinmetalle | VK >999 uv | AK >999 hf | AK >999 nh. Only the AK
//! (Ankauf) columns are recorded — VK is the trader's sell price, not a
//! purchase price. Fineness rides in the variant ("999 hf" / "999 nh").
//! The page quotes per gram for Au/Pt/Pd but per KILOGRAM for silver
//! ("Edelmetallpreise in €/g  Silber €/kg"); the catalog is EUR/g and
//! this handler never converts, so silver skips loudly. No Stand date,
//! empty Datum cells → `published_at` stays `None`. Contact comes from
//! the "Standort Hamburg" sidebar widget (this trader is the Hamburg
//! branch, not the Norderstedt HQ in the main impressum text).

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "hh-altstadt-norddeutsche-edelmetall-scheideanstalt-n";
/// Bespoke, live-verified impressum URL (site footer link). A move fails
/// the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://scheideanstalt-hamburg.de/impressum/";

pub const URL: &str = "https://scheideanstalt-hamburg.de/edelmetallpreise/";

const UNIT_ANCHOR: &str = "Edelmetallpreise in €/g";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
    // Exact per-gram list prices at full confidence. Silver never reaches
    // this loop (unit skip inside parse); unknown metals skip loudly.
    let mut prices = Vec::with_capacity(rows.len());
    for (label, col, price, unit) in rows {
        match grade_for(&label, &col) {
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
            None => skipped_labels.push(format!("{label} {col} (Sorte unverständlich)")),
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

/// Explicit (metal, AK-column) → (material, variant) mapping. VK columns
/// never arrive here (sell prices, dropped by design in parse); silver
/// never arrives here (per-kg quote, unit-skipped in parse).
fn grade_for(metal: &str, col: &str) -> Option<(&'static str, &'static str)> {
    let m = metal.to_lowercase();
    let hf = col.contains("hf");
    let nh = col.contains("nh");
    let variant: &'static str = if hf {
        "999 hf"
    } else if nh {
        "999 nh"
    } else {
        return None;
    };
    if m.contains("gold") {
        Some(("gold", variant))
    } else if m.contains("platin") && !m.contains("palladium") {
        Some(("platin", variant))
    } else if m.contains("palladium") {
        Some(("palladium", variant))
    } else {
        None
    }
}

/// Bespoke unit rule for THIS table only: the heading says per gram —
/// except silver, which is quoted per kg. Anything but EUR/g skips loudly
/// at the call site; conversions are out of scope for fine metals.
fn unit_of_metal(metal: &str) -> Option<&'static str> {
    let m = metal.to_lowercase();
    if m.contains("silber") {
        None
    } else if m.contains("gold") || m.contains("platin") || m.contains("palladium") {
        Some("EUR/g")
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: anchored on the
/// `h1.entry-title` "Impressum" plus the `h2.widget-title` "Standort
/// Hamburg" sidebar (the branch this handler covers — the main text is
/// the Norderstedt HQ). Street + PLZ city come from the widget's `<br>`
/// lines, the phone from its `tel:` link, the mail from the obfuscated
/// `[eeb_email …kontakt@norddeutsche-es.de…]` shortcode plaintext via
/// `@`-expansion. Missing anchors → loud error, never HQ fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let h2 = Selector::parse("h2").expect("valid selector");
    if !doc.select(&h1).any(|h| h.text().collect::<String>().trim() == "Impressum") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let hamburg = doc.select(&h2).find(|h| {
        h.text().collect::<String>().trim() == "Standort Hamburg"
    });
    let Some(hamburg) = hamburg else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Standort-Hamburg-Block fehlt".to_owned(),
        });
    };
    // The widget's address <p>s after the heading hold the address.
    // NOTE: never parse the wrapping <div>'s inner_html wholesale — its
    // second <p> ("Telefon: …") glues onto the PLZ line ("20354
    // Hamburg"+"Telefon:"), so work <p>-by-<p>.
    let p_sel = Selector::parse("p").expect("valid selector");
    let mut paras: Vec<scraper::ElementRef> = Vec::new();
    let mut sib = hamburg.next_siblings();
    let mut scanned = 0;
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    while let Some(node) = sib.next() {
        scanned += 1;
        if scanned > 12 {
            break;
        }
        let Some(el) = scraper::ElementRef::wrap(node) else { continue };
        let name = el.value().name();
        if name == "h2" {
            break;
        }
        if name == "p" {
            paras.push(el);
        } else if name == "div" {
            paras.extend(el.select(&p_sel));
        }
    }
    for el in &paras {
        for part in el.inner_html().split("<br") {
            let t = strip_fragment(part);
            if t.is_empty() {
                continue;
            }
            if street.is_empty() && t.contains("Neuer Wall") {
                street = t.clone();
                continue;
            }
            let mut it = t.split_whitespace();
            if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                if postcode.is_empty()
                    && pc.len() == 5
                    && pc.chars().all(|c| c.is_ascii_digit())
                    && ci.chars().next().is_some_and(|c| c.is_uppercase())
                {
                    postcode = pc.to_owned();
                    city = ci.to_owned();
                }
            }
        }
    }
    // tel: link inside the widget ("tel:+494060926890").
    let mut phone = String::new();
    {
        let a = Selector::parse("a[href^=\"tel:\"]").expect("valid selector");
        for el in &paras {
            if let Some(link) = el.select(&a).next() {
                if link.value().attr("href").is_some() {
                    phone = link.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ");
                    if !phone.is_empty() {
                        break;
                    }
                }
            }
        }
    }
    // Obfuscated shortcode carries the plaintext address
    // ("…email=…kontakt@norddeutsche-es.de…"); expand around '@'.
    // Char-based (not byte-based): the fixture uses multibyte
    // curly quotes that byte-slicing would glue or split.
    let mut email = String::new();
    if let Some(at) = imp.find('@') {
        let chars: Vec<char> = imp.chars().collect();
        let at_c = imp[..at].chars().count();
        let stop = |c: char| c.is_whitespace() || "<>\"'|=()[]“”&#;".contains(c);
        let mut l = at_c;
        while l > 0 && !stop(chars[l - 1]) {
            l -= 1;
        }
        let mut r = at_c;
        while r < chars.len() && !stop(chars[r]) {
            r += 1;
        }
        let cand: String = chars[l..r].iter().collect();
        if cand.contains('.') && cand.ends_with("norddeutsche-es.de") {
            email = cand;
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

/// Strip tags from a `<br`-split fragment (html5ever already decoded
/// entities). Fragments start with a tag remnant — drop everything up to
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

fn parse(
    html: &str,
) -> Result<(Vec<(String, String, f64, &'static str)>, Vec<String>), IngestError> {
    // Unit honesty first: the heading must state the €/g regime (with the
    // silver-€/kg exception). Without it the table moved → loud error.
    if !html.contains(UNIT_ANCHOR) {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Einheiten-Hinweis fehlt".to_owned(),
        });
    }
    let doc = Html::parse_document(html);
    let table = Selector::parse("table").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
    let row = Selector::parse("tbody tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    // Never trust page order: take the table carrying the Feinmetalle
    // header, not just the first <table> on the page.
    let table = doc.select(&table).find(|t| {
        t.select(&head).any(|h| {
            h.text().collect::<String>().to_lowercase().contains("feinmetalle")
        })
    });
    let Some(table) = table else {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "keine Feinmetalltabelle".to_owned() });
    };
    let heads: Vec<String> = table.select(&head).map(|h| h.text().collect()).collect();
    let norm: Vec<String> =
        heads.iter().map(|h| h.split_whitespace().collect::<Vec<_>>().join(" ")).collect();
    // Only AK (Ankauf) columns are recorded; VK (Verkauf) is the trader's
    // sell price by design, documented here — not a skip, not a row.
    let hf_idx = norm.iter().position(|h| h.starts_with("AK") && h.contains("hf"));
    let nh_idx = norm.iter().position(|h| h.starts_with("AK") && h.contains("nh"));
    let (Some(hf_idx), Some(nh_idx)) = (hf_idx, nh_idx) else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "AK-Spalten fehlen".to_owned(),
        });
    };
    let mut rows: Vec<(String, String, f64, &'static str)> = Vec::new();
    let mut skips: Vec<String> = Vec::new();
    for tr in table.select(&row) {
        let cells: Vec<String> = tr.select(&cell).map(|c| c.text().collect()).collect();
        if cells.len() <= hf_idx.max(nh_idx) || cells.len() < 2 {
            continue;
        }
        let metal = cells[1].split_whitespace().collect::<Vec<_>>().join(" ");
        if metal.is_empty() {
            continue;
        }
        // Silver is quoted per kg while the catalog is per gram — no
        // conversion for fine metals, so both its AK cells skip loudly.
        let Some(unit) = unit_of_metal(&metal) else {
            skips.push(format!(
                "{metal} (Notierung in €/kg, Katalog EUR/g — ohne Umrechnung geskippt)"
            ));
            continue;
        };
        for (idx, col) in [(hf_idx, norm[hf_idx].clone()), (nh_idx, norm[nh_idx].clone())] {
            match parse_eur(&cells[idx]) {
                Some(price) => rows.push((metal.clone(), col, price, unit)),
                None => skips.push(format!("{metal} {col} (Preis unverständlich: {})", cells[idx].trim())),
            }
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse { url: URL.to_owned(), detail: "Feinmetalltabelle leer".to_owned() });
    }
    Ok((rows, skips))
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of_metal};

    /// Real live markup: wpDataTable thead + four body rows.
    const FIXTURE: &str = "<h2>Edelmetallpreise in €/g  Silber €/kg</h2>\
        <table><thead><tr><th>Datum</th><th>Feinmetalle</th>\
        <th>VK &gt;999 uv</th><th>AK &gt;999 hf</th><th>AK &gt;999 nh</th></tr></thead>\
        <tbody><tr><td></td><td>Gold</td><td>125,32</td><td>116,68</td><td>114,29</td></tr>\
        <tr><td></td><td>Silber</td><td>1.902,96</td><td>1.709,14</td><td>1.673,90</td></tr>\
        <tr><td></td><td>Platin</td><td>52,36</td><td>46,27</td><td>43,84</td></tr>\
        <tr><td></td><td>Palladium</td><td>37,33</td><td>32,99</td><td>31,25</td></tr>\
        </tbody></table>";

    #[test]
    fn ak_columns_parse_and_silver_skips() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 6, "3 metals x 2 AK columns");
        assert_eq!(rows[0], ("Gold".to_owned(), "AK >999 hf".to_owned(), 116.68, "EUR/g"));
        assert_eq!(rows[1], ("Gold".to_owned(), "AK >999 nh".to_owned(), 114.29, "EUR/g"));
        assert_eq!(rows[2].0, "Platin");
        assert_eq!(rows[4].0, "Palladium");
        assert_eq!(rows[5].2, 31.25);
        // VK never becomes a row; silver skips loudly with the unit reason.
        assert!(rows.iter().all(|r| !r.1.contains("VK")));
        assert!(!rows.iter().any(|r| r.0 == "Silber"));
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Silber") && skips[0].contains("€/kg"));
        assert_eq!(unit_of_metal("Silber"), None);
        assert_eq!(unit_of_metal("Gold"), Some("EUR/g"));
    }

    #[test]
    fn wrong_table_and_headers_fail_loudly() {
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 6);
        assert!(parse("<h2>Sonst was</h2>").is_err(), "unit anchor missing");
        let no_ak = FIXTURE.replace("AK &gt;999 hf", "EK &gt;999 hf");
        assert!(parse(&no_ak).is_err(), "AK columns missing");
        let empty = FIXTURE
            .replace("116,68", "auf Anfrage")
            .replace("114,29", "auf Anfrage")
            .replace("46,27", "auf Anfrage")
            .replace("43,84", "auf Anfrage")
            .replace("32,99", "auf Anfrage")
            .replace("31,25", "auf Anfrage");
        assert!(parse(&empty).is_err(), "empty table errors");
    }

    #[test]
    fn impressum_extracts_branch_contact() {
        let imp = "<h1 class=\"entry-title\">Impressum</h1>\
            <p><strong>Gold- und Silberscheideanstalt<br>Hauptsitz: Materialannahme und Edelmetallwerk<br>D- 22844 Norderstedt – Oststrasse 128<br>Telefon: +49 (0)40 609 26 89-0</strong></p>\
            <aside><h2 class=\"widget-title\">Standort Hamburg</h2><div class=\"textwidget\">\
            <p>COLLECTION Servicebüros<br />der NES Scheideanstalt<br />Neuer Wall 80<br />20354 Hamburg</p>\
            <p>Telefon: <a href=\"tel:+494060926890\">+49(0)40 60926890</a></p></div></aside>\
            <p>Email: [eeb_email email=“kontakt@norddeutsche-es.de“ display=“x“]</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Neuer Wall 80");
        assert_eq!(info.postcode, "20354");
        assert_eq!(info.city, "Hamburg");
        assert_eq!(info.phone, "+49(0)40 60926890");
        assert_eq!(info.email, "kontakt@norddeutsche-es.de");
        // HQ-only page (no Hamburg widget) fails loudly — no HQ fallback.
        assert!(extract_info("<h1 class=\"entry-title\">Impressum</h1><p>HQ only</p>").is_err());
        assert!(extract_info("<p>Ohne Titel</p>").is_err());
    }

    #[test]
    fn mapping_uses_fineness_variants() {
        assert_eq!(grade_for("Gold", "AK >999 hf"), Some(("gold", "999 hf")));
        assert_eq!(grade_for("Gold", "AK >999 nh"), Some(("gold", "999 nh")));
        assert_eq!(grade_for("Platin", "AK >999 hf"), Some(("platin", "999 hf")));
        assert_eq!(grade_for("Palladium", "AK >999 nh"), Some(("palladium", "999 nh")));
        assert_eq!(grade_for("Silber", "AK >999 hf"), None);
        assert_eq!(grade_for("Gold", "VK >999 uv"), None);
    }
}
