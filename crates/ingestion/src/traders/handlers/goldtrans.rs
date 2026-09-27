//! Goldtrans Edelmetallhandel (Hamburg-Wandsbek): exact per-gram purchase
//! prices in one clean HTML table ("Gold Legierung" / "Preis", €/g in
//! every row, page-stated "Stand: dd.mm.yyyy"). 13 gold alloys plus a
//! separate Zahngold row (live 27.09.2026: 14 rows). Quoted unit is
//! honestly EUR/g (catalog units match — no conversion anywhere).
//! Fineness rides in the variant ("Gold 999 24 Karat" → `gold`/`999`);
//! Zahngold maps to `zahngold` (dental alloy, never a gold alias).

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "hh-wandsbek-goldtrans-edelmetallhandel";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.goldtrans.de/impressum.html";

pub const URL: &str =
    "https://www.goldtrans.de/goldankauf-preise-aktueller-goldpreis-ankauf-in-hamburg.html";

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
    // Exact per-gram list prices at full confidence. Fineness rides in
    // the variant; rows without metal word skip loudly with evidence.
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
            None => skipped_labels.push(format!(
                "{label} (Sorte unverstaendlich: {price:.2} {unit})"
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
        published_at,
    })
}

/// Explicit label → (material, variant) mapping. Fineness rides in the
/// variant ("Gold 999 24 Karat" → `gold`/`999`); Zahngold is dental
/// alloy, never a gold alias. Labels without metal word stay `None`.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    if l.contains("zahngold") {
        return Some(("zahngold", ""));
    }
    if l.contains("gold") {
        return Some(("gold", fineness(&l)));
    }
    if l.contains("silber") {
        return Some(("silber", fineness(&l)));
    }
    None
}

/// First fineness run in the label ("Gold 999 24 Karat" → "999").
// "Zahngold" carries no digits, so it is checked first above.
fn fineness(l: &str) -> &'static str {
    for fin in [
        "999", "986", "980", "965", "916", "900", "875", "833", "750", "585", "416", "375", "333",
    ] {
        if l.contains(fin) {
            return fin;
        }
    }
    ""
}

/// Bespoke unit rule for THIS table only: every price cell quotes "€/g"
/// (per gram). Anything else is a redesign → loud skip, never a default.
fn unit_of(price_cell: &str) -> Option<&'static str> {
    if price_cell.to_lowercase().contains("/g") {
        Some("EUR/g")
    } else {
        None
    }
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
    // Window: the price block between its intro line and the "Bitte
    // Beachten" notes. The footer maps <table> sits outside this window
    // and is excluded twice over by the header selection below.
    let start = html
        .find("Hier aktuelle Ankaufspreise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("Bitte Beachten").unwrap_or(tail.len());
    let window = &tail[..end];
    // Kopf-Selektion: the table whose header reads "Gold Legierung" —
    // never the first table on the page. The <style> block inside the
    // window is never read as text (only table cells are selected).
    let doc = Html::parse_fragment(window);
    let table_sel = Selector::parse("table").expect("valid selector");
    let th_sel = Selector::parse("th").expect("valid selector");
    let tr_sel = Selector::parse("tr").expect("valid selector");
    let td_sel = Selector::parse("td").expect("valid selector");
    let table = doc
        .select(&table_sel)
        .find(|t| {
            t.select(&th_sel)
                .any(|h| h.text().collect::<String>().contains("Gold Legierung"))
        })
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle (Gold Legierung) fehlt".to_owned(),
        })?;
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in table.select(&tr_sel) {
        let cells: Vec<String> = tr
            .select(&td_sel)
            .map(|c| c.text().collect::<String>())
            .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        if cells.len() < 2 || cells[0].is_empty() {
            continue;
        }
        let (label, price_raw) = (cells[0].clone(), cells[1].clone());
        let Some(price) = parse_eur(&price_raw) else {
            skips.push(format!("{label} (Preis unverstaendlich: {price_raw})"));
            continue;
        };
        let Some(unit) = unit_of(&price_raw) else {
            skips.push(format!("{label} (Einheit unverstaendlich: {price_raw})"));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preiszeilen".to_owned(),
        });
    }
    // Page-stated validity ("Stand : 27.09.2026"); absent → None
    // (observed_at = age). Anchored on "Stand" so stray dates elsewhere
    // cannot leak in.
    let published_at = window.find("Stand").and_then(|i| date_in(&window[i..]));
    Ok((published_at, rows, skips))
}

/// dd.mm.yyyy → RFC 3339 UTC midnight (tappe-style byte scan; this
/// handler's own copy, no shared helper).
fn date_in(window: &str) -> Option<String> {
    let bytes = window.as_bytes();
    let mut i = 0;
    while i + 10 <= bytes.len() {
        if bytes[i].is_ascii_digit()
            && bytes[i + 2] == b'.'
            && bytes[i + 5] == b'.'
            && bytes[i + 6..].iter().take(4).all(|c| c.is_ascii_digit())
        {
            let (d, m, y) = (
                &window[i..i + 2],
                &window[i + 3..i + 5],
                &window[i + 6..i + 10],
            );
            if let Some(rfc) = parse_de_date(d, m, y) {
                return Some(rfc);
            }
        }
        i += 1;
    }
    None
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` holding
/// "GOLDTRANS Edelmetallhandel e.K." carries `<br/>`-separated lines
/// (firm, street + PLZ city on one line, "Tel.: … / Fax.: …", "E-Mail:
/// …"). Missing anchor → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p = Selector::parse("p").expect("valid selector");
    let block = doc
        .select(&p)
        .find(|el| {
            el.text()
                .collect::<String>()
                .contains("GOLDTRANS Edelmetallhandel e.K.")
        })
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        })?;
    let lines: Vec<String> = block
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    // "Ahrensburger Strasse 69 22041 Hamburg": street is everything
    // before the 5-digit PLZ token, city the token after it.
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for line in lines.iter() {
        let toks: Vec<&str> = line.split_whitespace().collect();
        for (j, t) in toks.iter().enumerate() {
            if t.len() == 5 && t.chars().all(|c| c.is_ascii_digit()) {
                postcode = (*t).to_owned();
                if let Some(ci) = toks.get(j + 1) {
                    city = (*ci).to_owned();
                }
                street = toks[..j].join(" ");
                break;
            }
        }
        if !postcode.is_empty() {
            break;
        }
    }
    let mut phone = String::new();
    let mut email = String::new();
    for line in &lines {
        if let Some(v) = line.strip_prefix("Tel.:") {
            phone = v.split('/').next().unwrap_or_default().trim().to_owned();
        } else if let Some(v) = line.strip_prefix("E-Mail:") {
            email = v.trim().to_owned();
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

/// Strip tags from a fragment (html5ever already decoded entities).
/// Fragments from splitting on "<br" start with a tag remnant
/// (` class="…"`) — drop everything up to the first '>' first, or the
/// attributes parse as text.
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
    use super::{date_in, grade_for, parse, unit_of};

    // Real live excerpt (27.09.2026): intro + <style> + table head +
    // three rows (incl. the Zahngold row with entity) + Stand line.
    const FIXTURE: &str = "<p>Hier aktuelle Ankaufspreise f&uuml;r Altgold in Hamburg f&uuml;r unseren Goldankauf<style type=\"text/css\">table { border-collapse: collapse; }</style>\
        <table><tr><th width=\"266\"><div align=\"left\">Gold Legierung </div></th><th width=\"270\"><div align=\"left\">Preis</div></th></tr>\
        <tr><td>Gold 999 24 Karat</td><td>111,33 &euro;/g</td></tr>\
        <tr><td>Gold 750 18 Karat</td><td>83,50 &euro;/g </td></tr>\
        <tr><td>Zahngold (Ohne Z&auml;hne, Gelb) </td><td>69,02 &euro;/g </td></tr>\
        </table><p><span class=\"style2\">Stand : 27.09.2026</span> Hamburg</p>";

    #[test]
    fn table_rows_and_stand_date() {
        let (published, rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty());
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].0, "Gold 999 24 Karat");
        assert_eq!(rows[0].1, 111.33);
        assert_eq!(rows[0].2, "EUR/g");
        assert_eq!(rows[2].0, "Zahngold (Ohne Z\u{00e4}hne, Gelb)");
        assert_eq!(rows[2].1, 69.02);
        assert!(published
            .as_deref()
            .unwrap_or_default()
            .starts_with("2026-09-27"));
        assert_eq!(unit_of("111,33 \u{20ac}/g"), Some("EUR/g"));
        assert_eq!(unit_of("auf Anfrage"), None);
        assert!(
            parse("<p>Hier aktuelle Ankaufspreise X</p>").is_err(),
            "leere Tabelle = Err"
        );
        assert!(
            parse("<p>anderer Inhalt ohne Block</p>").is_err(),
            "missing anchor = Err"
        );
        assert!(date_in("Stand : 27.09.2026 Hamburg").is_some());
        assert_eq!(date_in("kein Datum"), None);
    }

    #[test]
    fn fineness_rides_in_variant() {
        // Every live label (27.09.2026: 13 alloys + Zahngold) maps now
        // that the catalog holds EUR/g precious metals.
        for (label, material, variant) in [
            ("Gold 999 24 Karat", "gold", "999"),
            ("Gold 986 23,6 Karat", "gold", "986"),
            ("Gold 980 23,5 Karat", "gold", "980"),
            ("Gold 965 23 Karat", "gold", "965"),
            ("Gold 916 22 Karat", "gold", "916"),
            ("Gold 900 21.6 Karat", "gold", "900"),
            ("Gold 875 21 Karat", "gold", "875"),
            ("Gold 833 20 Karat", "gold", "833"),
            ("Gold 750 18 Karat", "gold", "750"),
            ("Gold 585 14 Karat", "gold", "585"),
            ("Gold 416 10 Karat", "gold", "416"),
            ("Gold 375 9 Karat", "gold", "375"),
            ("Gold 333 8 Karat", "gold", "333"),
            ("Zahngold (Ohne Z\u{00e4}hne, Gelb)", "zahngold", ""),
        ] {
            assert_eq!(grade_for(label), Some((material, variant)), "{label}");
        }
        assert_eq!(grade_for("Ankaufsbedingungen"), None);
    }

    #[test]
    fn impressum_block() {
        let imp = "<div class=\"entry-content\"><p>GOLDTRANS Edelmetallhandel e.K.<br />Inh. Tarkan Yilmaz<br />Ahrensburger Strasse 69 22041 Hamburg<br />Tel.: 040 970 79 580 / Fax.: 040 970 79 581<br />E-Mail: info@goldtrans.de<br />St.-Nr.:51/669/00559</p></div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Ahrensburger Strasse 69");
        assert_eq!(info.postcode, "22041");
        assert_eq!(info.city, "Hamburg");
        assert_eq!(info.phone, "040 970 79 580");
        assert_eq!(info.email, "info@goldtrans.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}
