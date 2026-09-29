//! Geld für Gold / Edelmetallkontor (Bad Kreuznach): exact per-gram
//! purchase prices in the server-rendered Ankaufsrechner (`div.gfg-calc`:
//! one `div.gfg-col` per metal with the title in `.gfg-col-title`
//! (GOLD/SILBER/PLATIN) and rows in `.gfg-row` whose `.gfg-fine` reads
//! like "999er<br><span>(110,00 €/g)</span>"). The fineness rides in the
//! variant (hansa pattern); the page states "Werte in €/g" (catalog unit
//! for `gold`/`silber`/`platin` is EUR/g — no conversion anywhere).
//! "ab"-prices do not occur here; the versilberte-Besteck box below the
//! calculator quotes EUR/kg for silver-PLATED cutlery, which has no
//! catalog material and is skipped loudly (never crammed into `silber`).
//! No quote date on the page ("aktualisieren sich automatisch") →
//! `published_at` stays `None`.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "rp-bad-kreuznach-geld-fur-gold-dagmar-muller-edelmetallko";
/// Bespoke, live-verified impressum URL (site footer's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "https://geldfuergold.de/impressum/";

pub const URL: &str = "https://geldfuergold.de/ankaufsrechner/";

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
            // No catalog material (versilbertes Besteck): keep the quoted
            // price as evidence in the skip, never drop it silently and
            // never cram plated cutlery into solid-silver `silber`.
            None => skipped_labels.push(format!(
                "{label} ({}, {unit}, kein Katalogmaterial: versilbertes Besteck)",
                fmt_eur(price)
            )),
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

fn fmt_eur(price: f64) -> String {
    format!("{price:.2}").replace('.', ",")
}

/// Explicit label → (material, variant) mapping. The metal word comes
/// from the column title (`parse` builds labels like "Gold 999er",
/// "Silber 925er", "Platin 999er"); fineness rides in the variant.
/// Anything else (versilbertes Besteck, prose) stays `None`.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    if l.contains("platin") {
        return Some(("platin", fineness(&l)));
    }
    if l.contains("silber") {
        return Some(("silber", fineness(&l)));
    }
    if l.contains("gold") {
        return Some(("gold", fineness(&l)));
    }
    None
}

/// First 3-digit run in the label ("986er" → "986"). Every calculator
/// row carries one; labels without stay variant-less (""), never guessed.
fn fineness(l: &str) -> &'static str {
    let b = l.as_bytes();
    let mut i = 0;
    while i + 3 <= b.len() {
        if b[i].is_ascii_digit() && b[i + 1].is_ascii_digit() && b[i + 2].is_ascii_digit() {
            return match &l[i..i + 3] {
                "999" => "999",
                "986" => "986",
                "950" => "950",
                "925" => "925",
                "916" => "916",
                "900" => "900",
                "835" => "835",
                "800" => "800",
                "750" => "750",
                "700" => "700",
                "625" => "625",
                "585" => "585",
                "500" => "500",
                "417" => "417",
                "375" => "375",
                "333" => "333",
                _ => "",
            };
        }
        i += 1;
    }
    ""
}

/// Parse the calculator window (`gfg-grid` … `gfg-actions`) plus the
/// versilberte-Besteck box below it. Returns (rows, unit_skips).
fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html.find("gfg-grid").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Ankaufsrechner fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("gfg-actions").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Ankaufsrechner unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let col_sel = Selector::parse("div.gfg-col").expect("valid selector");
    let title_sel = Selector::parse("div.gfg-col-title").expect("valid selector");
    let row_sel = Selector::parse("div.gfg-row").expect("valid selector");
    let fine_sel = Selector::parse("div.gfg-fine").expect("valid selector");
    // The quote lives in the `<span>` inside `.gfg-fine` ("(110,00 €/g)") —
    // never parse the whole fine cell: its head ("999er") starts with a
    // number and would win as a phantom price.
    let span_sel = Selector::parse("span").expect("valid selector");
    let mut rows = Vec::new();
    let mut unit_skips = Vec::new();
    let mut cols = 0;
    for col in frag.select(&col_sel) {
        cols += 1;
        let title = col
            .select(&title_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        // The metal word disambiguates bare "999er" fines — an unknown
        // column title skips its rows loudly instead of guessing.
        let Some(metal) = column_metal(&title) else {
            unit_skips.push(format!("Rechner-Spalte ohne Metallzuordnung: {title}"));
            continue;
        };
        for row in col.select(&row_sel) {
            let Some(fine_el) = row.select(&fine_sel).next() else {
                continue;
            };
            let fine = fine_el
                .text()
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            if fine.is_empty() || fine.len() > 120 {
                continue;
            }
            // "999er (110,00 €/g)" → head "999er"; platin rows already
            // carry the metal word ("Platin 999er") and keep it as-is.
            let head = fine.split('(').next().unwrap_or("").trim().to_owned();
            if head.is_empty() {
                continue;
            }
            let quote = fine_el
                .select(&span_sel)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();
            let Some(price) = parse_eur(&quote) else {
                unit_skips.push(format!("{head} (Preis fehlt)"));
                continue;
            };
            // An unparseable unit is a loud skip, never a silent default:
            // a per-kilo price recorded as per-gram would be a 1000x error.
            let Some(unit) = unit_of(&quote) else {
                unit_skips.push(format!("{head} (Einheit unverständlich: {})", quote.trim()));
                continue;
            };
            let label = if head.to_lowercase().contains(metal_word(metal)) {
                head
            } else {
                format!("{metal} {head}")
            };
            rows.push((label, price, unit));
        }
    }
    if cols == 0 {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Rechner-Spalten fehlen".to_owned(),
        });
    }
    // Versilbertes Besteck: per-kilo rows for plated cutlery — parsed for
    // the evidence trail, mapped to `None` (no catalog material). The box
    // is a side note, not the price core: if it vanishes the calculator
    // rows still flow and the loss is a loud skip, not a failed step.
    match parse_besteck(html, &mut unit_skips) {
        Ok(besteck) => rows.extend(besteck),
        Err(_) => unit_skips.push("Besteck-Box fehlt (Annahmebox unauffindbar)".to_owned()),
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankaufsrechner leer".to_owned(),
        });
    }
    Ok((rows, unit_skips))
}

/// Column title → display metal word. Unknown titles are `None` (loud
/// skip at the call site), never a guessed default.
fn column_metal(title: &str) -> Option<&'static str> {
    match title.trim() {
        "GOLD" => Some("Gold"),
        "SILBER" => Some("Silber"),
        "PLATIN" => Some("Platin"),
        _ => None,
    }
}

fn metal_word(metal: &str) -> &'static str {
    match metal {
        "Gold" => "gold",
        "Silber" => "silber",
        "Platin" => "platin",
        _ => "",
    }
}

/// Bespoke unit matcher for THIS calculator (live: "(110,00 €/g)" spans)
/// and the Besteck box ("20,00 Euro pro Kilogramm"). Only g/kg exist
/// here — anything else skips loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("/g") || lower.contains("pro gramm") {
        Some("EUR/g")
    } else if lower.contains("kilogramm") || lower.contains("/ kg") || lower.contains("/kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

/// The "Ankauf von versilbertem Besteck" box between its heading and the
/// "90er/100er Prägung" note. Rows are `<li>` like "Besteck ab 90er
/// Auflage: 20,00 Euro pro Kilogramm".
fn parse_besteck(
    html: &str,
    unit_skips: &mut Vec<String>,
) -> Result<Vec<(String, f64, &'static str)>, IngestError> {
    let start = html
        .find("versilbertem Besteck")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Besteck-Box fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("90er/100er").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Besteck-Box unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<ul>{window}</ul>"));
    let li = Selector::parse("li").expect("valid selector");
    let mut rows = Vec::new();
    for el in frag.select(&li) {
        let label = el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if label.is_empty() || label.len() > 200 {
            continue;
        }
        // The quote sits after the colon ("… Auflage: 20,00 Euro pro
        // Kilogramm") — never parse the whole line: the "90er" grade
        // starts with a number and would win as a phantom price.
        let quote = label.split(':').next_back().unwrap_or(label.as_str());
        let Some(price) = parse_eur(quote) else {
            continue;
        };
        let Some(unit) = unit_of(&label) else {
            unit_skips.push(format!("{label} (Einheit unverständlich)"));
            continue;
        };
        rows.push((label, price, unit));
    }
    Ok(rows)
}

/// Bespoke contact extraction for THIS impressum only: `div.et_pb_text_inner`
/// holds `<h1>Impressum</h1>`, an address `<p>` ("GeldFuerGold.de … Mannheimer
/// Str. 65-67 … 55545 Bad Kreuznach") and a contact `<p>` ("Telefon: …",
/// "E-mail: …"). Missing anchors → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    if !doc
        .select(&h1)
        .any(|h| h.text().collect::<String>().trim() == "Impressum")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let addr_p = doc.select(&p).find(|el| {
        let t: String = el.text().collect();
        t.contains("Mannheimer Str.")
    });
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
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
    let cont_p = doc.select(&p).find(|el| {
        let t: String = el.text().collect();
        t.contains("Telefon:")
    });
    let (mut phone, mut email) = (String::new(), String::new());
    if let Some(el) = cont_p {
        for part in el.inner_html().split("<br") {
            let t = strip_fragment(part);
            if let Some(v) = t.strip_prefix("Telefon:") {
                phone = v.trim().to_owned();
            } else if let Some(v) = t.strip_prefix("E-mail:") {
                email = v.trim().to_owned();
            } else if let Some(v) = t.strip_prefix("E-Mail:") {
                email = v.trim().to_owned();
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
    use super::{column_metal, fineness, grade_for, parse, unit_of};

    // Real shape of the live calculator (col classes/titles, fine cells
    // with display span, platin rows carrying the metal word) plus the
    // versilberte-Besteck box, trimmed to a few rows.
    const FIXTURE: &str = "<div class=\"gfg-calc\"><div class=\"gfg-grid\">\
        <div class=\"gfg-col gfg-gold\"><div class=\"gfg-col-title\">GOLD</div>\
        <div class=\"gfg-row\"><div class=\"gfg-fine\">999er<br><span>(110,00 €/g)</span></div>\
        <div class=\"gfg-input\"><input placeholder=\"Gramm\"><span>Gramm</span></div></div>\
        <div class=\"gfg-row\"><div class=\"gfg-fine\">585er<br><span>(64,35 €/g)</span></div>\
        <div class=\"gfg-input\"><input placeholder=\"Gramm\"><span>Gramm</span></div></div></div>\
        <div class=\"gfg-col gfg-silver\"><div class=\"gfg-col-title\">SILBER</div>\
        <div class=\"gfg-row\"><div class=\"gfg-fine\">925er<br><span>(1,29 €/g)</span></div>\
        <div class=\"gfg-input\"><input placeholder=\"Gramm\"><span>Gramm</span></div></div></div>\
        <div class=\"gfg-col gfg-other\"><div class=\"gfg-col-title\">PLATIN</div>\
        <div class=\"gfg-row\"><div class=\"gfg-fine\">Platin 950er<br><span>(33,25 €/g)</span></div>\
        <div class=\"gfg-input\"><input placeholder=\"Gramm\"><span>Gramm</span></div></div>\
        </div></div><div class=\"gfg-actions\"><button>JETZT WERT BERECHNEN</button></div></div>\
        <h3>Ankauf von versilbertem Besteck</h3>\
        <ul><li>Besteck ab 90er Auflage: 20,00 Euro pro Kilogramm</li>\
        <li>Messer ab 90er Auflage: 10,00 Euro pro Kilogramm</li></ul>\
        <p>Hinweis: Es wird ausschließlich Besteck mit erkennbarer 90er/100er Prägung angenommen.</p>";

    #[test]
    fn calculator_columns_units_and_besteck() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty());
        // 4 calculator rows + 2 Besteck rows.
        assert_eq!(rows.len(), 6);
        assert_eq!(rows[0], ("Gold 999er".to_owned(), 110.0, "EUR/g"));
        assert_eq!(rows[1], ("Gold 585er".to_owned(), 64.35, "EUR/g"));
        assert_eq!(rows[2], ("Silber 925er".to_owned(), 1.29, "EUR/g"));
        // No doubled metal word on platin rows.
        assert_eq!(rows[3], ("Platin 950er".to_owned(), 33.25, "EUR/g"));
        // Besteck quotes sit after the colon — the "90er" grade must not
        // win as a phantom price.
        assert_eq!(rows[4].1, 20.0);
        assert_eq!(rows[4].2, "EUR/kg");
        assert!(rows[4].0.contains("Besteck ab 90er"));
        assert_eq!(rows[5].1, 10.0);
        assert_eq!(unit_of("(110,00 €/g)"), Some("EUR/g"));
        assert_eq!(unit_of("20,00 Euro pro Kilogramm"), Some("EUR/kg"));
        assert_eq!(unit_of("pro Sack"), None);
        assert_eq!(column_metal("GOLD"), Some("Gold"));
        assert_eq!(column_metal("PALLADIUM"), None);
        assert!(parse("<div>Redesign ohne Rechner</div>").is_err());
    }

    #[test]
    fn fineness_rides_in_variant_and_besteck_skips() {
        assert_eq!(grade_for("Gold 999er"), Some(("gold", "999")));
        assert_eq!(grade_for("Gold 986er"), Some(("gold", "986")));
        assert_eq!(grade_for("Gold 585er"), Some(("gold", "585")));
        assert_eq!(grade_for("Gold 333er"), Some(("gold", "333")));
        assert_eq!(grade_for("Silber 925er"), Some(("silber", "925")));
        assert_eq!(grade_for("Silber 800er"), Some(("silber", "800")));
        assert_eq!(grade_for("Platin 999er"), Some(("platin", "999")));
        assert_eq!(grade_for("Platin 950er"), Some(("platin", "950")));
        // Plated cutlery is not solid silver — never crammed.
        assert_eq!(
            grade_for("Besteck ab 90er Auflage: 20,00 Euro pro Kilogramm"),
            None
        );
        assert_eq!(
            grade_for("Messer ab 90er Auflage: 10,00 Euro pro Kilogramm"),
            None
        );
        assert_eq!(fineness("gold"), "");
    }

    #[test]
    fn impressum_blocks() {
        let imp = "<div class=\"et_pb_text_inner\"><h1>Impressum</h1>\
            <p>GeldFuerGold.de<br />C/O Edelmetallkontor<br />Inh. Dagmar Müller<br />\
            Mannheimer Str. 65-67<br />55545 Bad Kreuznach<br />(unter dieser Adresse werden Edelmetalle<br />\
            weder verarbeitet, noch gelagert)</p>\
            <p>Telefon: 06 71 – 9 20 07 82<br />Mobil: <span>0151-21991132</span><br />\
            E-mail: kontakt@GeldFuerGold.de</p></div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Mannheimer Str. 65-67");
        assert_eq!(info.postcode, "55545");
        assert_eq!(info.city, "Bad Kreuznach");
        assert_eq!(info.phone, "06 71 – 9 20 07 82");
        assert_eq!(info.email, "kontakt@GeldFuerGold.de");
        assert!(super::extract_info("<h1>Neu hier</h1>").is_err());
    }
}
