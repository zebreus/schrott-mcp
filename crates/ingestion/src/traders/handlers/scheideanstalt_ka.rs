//! Scheideanstalt Karlsruhe (Karlstr. 25): the Edelmetallrechner page carries
//! a server-rendered "Unsere Goldpreise" table (`table.goldpreis-tabelle`:
//! `td.name` + `td.preis` like "38.30/g") with the quote date in `<tfoot>`
//! ("27.09.2026 22:50"). Units are honestly per-gram (catalog unit for
//! `gold`/`zahngold` is EUR/g — no conversion anywhere). Fineness rides in
//! the variant (hansa pattern); coin/bar rows pay above plain alloy of the
//! same fineness, so they keep their own variants and never collapse.
//! "Zahngold gelb ab" is a from-price: price = price_min = quoted value,
//! kind `approx` at 0.5 — never a silent exact. Silver, platinum and
//! palladium have calculator inputs but no price table → no rows.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bw-karlsruhe-76133-scheideanstalt-karlsruhe";
/// Bespoke, live-verified impressum URL (site footer's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "https://scheideanstaltka.de/impressum-2/";

/// Price-table page (the homepage quotes no prices — only prose).
pub const URL: &str = "https://scheideanstaltka.de/edelmetallrechner/";

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
            Some((material, variant, approx)) => {
                // From-prices ("ab") mirror the upto pattern: the bound is
                // honest data with its own kind, never a silent exact.
                let (price_kind, price_min, confidence) = if approx {
                    ("approx", Some(price), Some(0.5))
                } else {
                    ("exact", None, Some(1.0))
                };
                prices.push(ScrapedPrice {
                    material,
                    variant,
                    price,
                    currency: "EUR",
                    unit,
                    price_kind,
                    price_min,
                    price_max: None,
                    confidence,
                    label,
                });
            }
            // No catalog material for precious metal: keep the quoted
            // price as evidence in the skip, never drop it silently.
            None => skipped_labels.push(format!(
                "{label} ({}, {unit}, kein Katalogmaterial: Edelmetall)",
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
        published_at,
    })
}

fn fmt_eur(price: f64) -> String {
    format!("{price:.2}").replace('.', ",")
}

/// Explicit label → (material, variant, approx-flag). Zahngold arms first:
/// "Zahngold …" contains "gold" as a substring and would otherwise land on
/// plain `gold`. Coin/bar arms before plain fineness: same fineness, higher
/// price → own variants or they collapse onto one current price.
fn grade_for(label: &str) -> Option<(&'static str, &'static str, bool)> {
    let norm = label.trim_end_matches('*').trim().to_lowercase();
    let l = norm.as_str();
    if l.contains("zahngold") {
        if l.contains("weiss") {
            Some(("zahngold", "weiss", false))
        } else if l.contains("gelb") {
            Some(("zahngold", "gelb", is_ab(l)))
        } else {
            Some(("zahngold", "", false))
        }
    } else if l.contains("münzen") || l.contains("muenzen") || l.contains("barren") {
        let coined = l.contains("münzen") || l.contains("muenzen");
        let barred = l.contains("barren");
        match (coined, barred, fineness(l)) {
            (true, true, "900") => Some(("gold", "900 Münzen/Barren", false)),
            (true, true, "916") => Some(("gold", "916 Münzen/Barren", false)),
            (true, true, "986") => Some(("gold", "986 Münzen/Barren", false)),
            (false, true, "999") => Some(("gold", "999 Barren", false)),
            (true, false, "999") => Some(("gold", "999 Münzen", false)),
            _ => None,
        }
    } else if l.contains("gold") {
        match fineness(l) {
            "" => None,
            fin => Some(("gold", fin, false)),
        }
    } else {
        None
    }
}

/// First known 3-digit fineness run in the label ("900er Gold 21,6kt" →
/// "900" — the scan hits the leading run first). Labels without one stay
/// variant-less at the call site, never guessed.
fn fineness(l: &str) -> &'static str {
    let b = l.as_bytes();
    let mut i = 0;
    while i + 3 <= b.len() {
        if b[i].is_ascii_digit() && b[i + 1].is_ascii_digit() && b[i + 2].is_ascii_digit() {
            match &l[i..i + 3] {
                "333" => return "333",
                "375" => return "375",
                "585" => return "585",
                "750" => return "750",
                "833" => return "833",
                "875" => return "875",
                "900" => return "900",
                "916" => return "916",
                "986" => return "986",
                "999" => return "999",
                _ => {}
            }
        }
        i += 1;
    }
    ""
}

/// Trailing "ab" token ("Zahngold gelb ab") marks a from-price.
fn is_ab(l: &str) -> bool {
    l.split_whitespace().last() == Some("ab")
}

/// Parse the Goldpreise window between the widget heading and the chart
/// heading. Returns (published_at, rows, unit_skips).
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
    let start = html
        .find("Unsere Goldpreise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Goldpreistabelle fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Goldkurs in Euro")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Goldpreistabelle unvollständig".to_owned(),
        })?;
    let window = &tail[..end];
    // Head-anchored table choice (guide: Kopfinhalt, nie die erste): the
    // thead here reads "Name" + "Preis".
    if !(window.contains(">Name<") && window.contains("Preis")) {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Tabellenkopf fehlt".to_owned(),
        });
    }
    let frag = Html::parse_fragment(window);
    let tr = Selector::parse("tr").expect("valid selector");
    let name = Selector::parse("td.name").expect("valid selector");
    let preis = Selector::parse("td.preis").expect("valid selector");
    let mut rows = Vec::new();
    let mut unit_skips = Vec::new();
    for row in frag.select(&tr) {
        let (Some(n), Some(p)) = (row.select(&name).next(), row.select(&preis).next()) else {
            continue;
        };
        // The footnote star rides in a <sup> ("999er Gold 24kt*") — text()
        // keeps it; grade_for trims it. Labels >120 chars are prose.
        let label = n
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if label.is_empty() || label.len() > 120 {
            continue;
        }
        let raw = p.text().collect::<String>();
        let raw = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        let Some(price) = parse_eur(&raw) else {
            unit_skips.push(format!("{label} (Preis unverständlich: {raw})"));
            continue;
        };
        // A per-kilo price recorded as per-gram would be a 1000x error —
        // unparseable or foreign units skip loudly, never default.
        let Some(unit) = unit_of(&raw) else {
            unit_skips.push(format!("{label} (Einheit unverständlich: {raw})"));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Goldpreistabelle leer".to_owned(),
        });
    }
    Ok((find_date(window), rows, unit_skips))
}

/// Bespoke unit matcher for THIS table's price cells (live: "38.30/g").
/// Explicit-but-foreign (`/` or "pro" without `/g`) skips loudly at the
/// call site instead of guessing.
fn unit_of(cell: &str) -> Option<&'static str> {
    let l = cell.to_lowercase();
    if l.contains("/g") || l.contains("pro gramm") {
        Some("EUR/g")
    } else if l.contains("kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

/// Bespoke date finder for THIS table: `<tfoot><tr><td colspan="2">`
/// carries "27.09.2026 22:50" (time ignored — one date per page). No
/// anchor → None (the observation age stays the provenance).
fn find_date(window: &str) -> Option<String> {
    let (_, after) = window.split_once("<tfoot>")?;
    let tok = after
        .split(|c: char| !(c.is_ascii_digit() || c == '.' || c == ':'))
        .find(|t| t.len() == 10 && t.chars().filter(|c| *c == '.').count() == 2)?;
    let parts: Vec<&str> = tok.split('.').collect();
    parse_de_date(parts[0], parts[1], parts[2])
}

/// Bespoke contact extraction for THIS impressum only: `<h2>Filiale
/// Karlsruhe</h2>` followed by the address `<p>` (firm + street + PLZ city
/// lines, "Tel."/"Mail:" lines; the phone glues a `<b>` part
/// ("0721."+"98 19 36 62") exactly like the guide's glued-nodes gotcha).
/// Missing anchors → loud error, never guessed.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    if !imp.contains("Filiale Karlsruhe") {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Filiale-Block fehlt".to_owned(),
        });
    }
    let after = &imp[imp.find("Filiale Karlsruhe").expect("checked")..];
    let p_start = after.find("<p>").ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Adress-Block fehlt".to_owned(),
    })?;
    let body = &after[p_start..];
    let p_end = body.find("</p>").ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Adress-Block fehlt".to_owned(),
    })?;
    let lines: Vec<String> = body[..p_end]
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| format!("{a} {b}"));
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                break;
            }
        }
    }
    if postcode.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "PLZ/Ort fehlt".to_owned(),
        });
    }
    let mut phone = String::new();
    let mut email = String::new();
    for line in &lines {
        if let Some(v) = line.strip_prefix("Tel.") {
            phone = v.trim().to_owned();
        } else if let Some(v) = line.strip_prefix("Mail:") {
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
    use super::{extract_info, find_date, fineness, grade_for, is_ab, parse, unit_of};

    // Real table shape (class names, sup footnote stars, Zahngold rows,
    // tfoot quote date), trimmed to five rows, terminated by the chart
    // heading exactly like live.
    const FIXTURE: &str = "<h4 class=\"main-title widget-title\">Unsere Goldpreise</h4>\
        <div class=\"widget\"><div class=\"goldpreis-widget goldpreis-tab_Gold\">\
        <table border=\"1\" cellspacing=\"0\" class=\"goldpreis-tabelle goldpreis-tabelle-Gold\">\
        <caption>Gold</caption><thead><tr><th width=\"70%\">Name</th>\
        <th>Preis <small>(&euro;)</small></th></tr></thead><tbody>\
        <tr class=\"even\"><td class=\"name\">585er Gold 14kt</td><td class=\"preis\">67.29/g</td></tr>\
        <tr class=\"odd\"><td class=\"name\">999er Gold 24kt<sup>*</sup></td>\
        <td class=\"preis\">116.02/g</td></tr>\
        <tr class=\"even\"><td class=\"name\">Münzen/Barren 900<sup>*</sup></td>\
        <td class=\"preis\">104.42/g</td></tr>\
        <tr class=\"odd\"><td class=\"name\">Zahngold weiss pd, pt, zn</td>\
        <td class=\"preis\">30.00/g</td></tr>\
        <tr class=\"even\"><td class=\"name\">Zahngold gelb ab</td>\
        <td class=\"preis\">70.00/g</td></tr>\
        </tbody><tfoot><tr><td colspan=\"2\">27.09.2026 22:50</td></tr></tfoot>\
        </table></div></div><h4 class=\"main-title widget-title\">Goldkurs in Euro</h4>";

    #[test]
    fn table_rows_units_and_date() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-27T00:00:00+00:00"));
        assert_eq!(rows.len(), 5);
        assert!(skips.is_empty(), "{skips:?}");
        assert_eq!(rows[0], ("585er Gold 14kt".to_owned(), 67.29, "EUR/g"));
        // Footnote stars stay in the label (traceability), trimmed for mapping.
        assert_eq!(rows[1].0, "999er Gold 24kt*");
        assert_eq!(rows[1].1, 116.02);
        assert_eq!(rows[4], ("Zahngold gelb ab".to_owned(), 70.0, "EUR/g"));
        assert_eq!(unit_of("38.30/g"), Some("EUR/g"));
        assert_eq!(unit_of("5 €/kg"), Some("EUR/kg"));
        assert_eq!(unit_of("10 Euro pro Stück"), None);
        assert_eq!(find_date("ohne tfoot"), None);
        assert!(parse("<div>Redesign ohne Tabelle</div>").is_err());
        assert!(parse("<p>Unsere Goldpreise ohne Ende</p>").is_err());
    }

    #[test]
    fn all_live_labels_map() {
        // Every row of the live 17-row table, in page order — specific
        // before generic (Zahngold contains "gold").
        let cases = [
            ("333er Gold 8kt", ("gold", "333")),
            ("375er Gold 9kt", ("gold", "375")),
            ("585er Gold 14kt", ("gold", "585")),
            ("750er Gold 18kt", ("gold", "750")),
            ("833er Gold", ("gold", "833")),
            ("875er Gold 21kt", ("gold", "875")),
            ("900er Gold 21,6kt", ("gold", "900")),
            ("916er Gold 22kt", ("gold", "916")),
            ("986er Gold 23,6kt", ("gold", "986")),
            ("999er Gold 24kt*", ("gold", "999")),
            ("Münzen/Barren 900*", ("gold", "900 Münzen/Barren")),
            ("Münzen/Barren 916*", ("gold", "916 Münzen/Barren")),
            ("Münzen/Barren 986*", ("gold", "986 Münzen/Barren")),
            ("Barren 999*", ("gold", "999 Barren")),
            ("Münzen 999*", ("gold", "999 Münzen")),
            ("Zahngold weiss pd, pt, zn", ("zahngold", "weiss")),
            ("Zahngold gelb ab", ("zahngold", "gelb")),
        ];
        for (label, (material, variant)) in cases {
            let got = grade_for(label);
            assert_eq!(
                got.map(|(m, v, _)| (m, v)),
                Some((material, variant)),
                "{label}"
            );
        }
        // The "ab" row is a from-price; everything else is exact.
        assert!(grade_for("Zahngold gelb ab").expect("maps").2);
        assert!(!grade_for("Zahngold weiss pd, pt, zn").expect("maps").2);
        assert!(!grade_for("585er Gold 14kt").expect("maps").2);
        assert!(is_ab("zahngold gelb ab"));
        assert!(!is_ab("585er gold 14kt"));
        // No fineness, no mapping.
        assert_eq!(grade_for("Gold"), None);
        assert_eq!(grade_for("Ankaufbedingungen"), None);
        assert_eq!(fineness("900er gold 21,6kt"), "900");
        assert_eq!(fineness("gold ohne gehalt"), "");
    }

    #[test]
    fn impressum_filiale_block() {
        // Real shape incl. the glued phone ("0721." + <b> part).
        let imp = "<h1 class=\"title\">Impressum</h1><h2>Filiale Karlsruhe</h2>\
            <p>ScheideanstaltKa.GmbH<br />Karlstr. 25<br />76133 Karlsruhe<br />HRB : 721844<br />\
            Tel. 0721.<b>98 19 36 62</b><br />Fax 0721.1519740<br />\
            Mail: info@scheideanstaltka.de<br />Geschäftsleitung: Christian Kratz<br />\
            Steuernummer: 35008/16022<br />Ust:301164166</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Karlstr. 25");
        assert_eq!(info.postcode, "76133");
        assert_eq!(info.city, "Karlsruhe");
        assert_eq!(info.phone, "0721.98 19 36 62");
        assert_eq!(info.email, "info@scheideanstaltka.de");
        assert!(extract_info("<p>Neu hier</p>").is_err());
        assert!(extract_info("<h2>Filiale Karlsruhe</h2><p>ohne PLZ</p>").is_err());
    }
}
