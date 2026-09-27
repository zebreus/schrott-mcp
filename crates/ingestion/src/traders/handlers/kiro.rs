//! Kiro Schrotthandel (Pankow, Berlin): exact per-kg prices as SiteOrigin
//! panel pairs (`div.textwidget` label `<p>` + price `<span>`, live e.g.
//! "10.10 €/kg (Ab 750 kg: 10.30 €)") between the "Schrottankauf und
//! Schrottpreise Berlin bei Kiro" heading and the "Aktuelle Schrottpreise
//! Berlin im Überblick" section, plus two `table.footable` summary tables
//! (header "Schrottsorte") for Eisen/Buntmetall rows. Every panel price
//! carries a bulk tier ("Ab 750 kg: …") recorded as its own variant
//! mkr-style; panel/table duplicates collapse via (material, variant,
//! price) dedup after mapping. "Scherenschrott / Gussschrott" prices two
//! sorts at once → loud skip; "Zinnschrott 99%" has no price → loud skip;
//! "Alu-Kupferkühler" is a composite with no catalog material → loud
//! skip. No visible page date (only meta modified_time), so
//! `published_at` stays `None`.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "be-pankow-kiro-schrotthandel-kafedzhiev";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://schrotthandel-berlin.com/impressum/";

pub const URL: &str = "https://schrotthandel-berlin.com/schrottpreise/";

/// Panel window anchors on the live page.
const PANEL_START: &str = "Schrottankauf und Schrottpreise Berlin bei Kiro";
const PANEL_END: &str = "Aktuelle Schrottpreise Berlin im Überblick";
/// Bulk-tier marker inside price cells ("(Ab 750 kg: 10.30 €)").
const BULK_MARKER: &str = "Ab 750 kg";
const BULK_TIER: &str = "ab 750 kg";

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
    let mut seen: Vec<(&'static str, &'static str, u64)> = Vec::with_capacity(rows.len());
    for (label, tier, price, unit) in rows {
        match grade_for(&label, &tier) {
            Some((material, variant)) => {
                // Panel/table Doppelblöcke quote the same grades twice —
                // dedup after mapping, never on raw labels.
                let key = (material, variant, price.to_bits());
                if seen.contains(&key) {
                    continue;
                }
                seen.push(key);
                prices.push(ScrapedPrice {
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
                });
            }
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

/// Explicit (label, tier) → (material, variant) mapping. Tiers are "" and
/// "ab 750 kg"; the variant keeps the trader's own grade wording plus the
/// tier mkr-style. Anything unlisted is skipped.
fn grade_for(label: &str, tier: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    let (material, detail): (&'static str, &'static str) = if l.contains("millberry") {
        ("kupfer-millberry", "")
    } else if l.contains("kerze") {
        ("kupfer-berry", "Kerze")
    } else if l.contains("raff") && !l.contains("kabel") {
        ("kupfer-gemischt", "")
    } else if l.contains("kupferkabel") || (l.contains("kabel") && !l.contains('%')) {
        ("kabel-kupfer", "")
    } else if l.contains("kabel") && l.contains("50%") {
        ("kabel-kupfer", "50%")
    } else if l.contains("kabel") && l.contains("60%") {
        ("kabel-kupfer", "60%")
    } else if l.contains("kabel") && l.contains("70%") {
        ("kabel-kupfer", "70%")
    } else if l.contains("kabel") && l.contains("80%") {
        ("kabel-kupfer", "80%")
    } else if l.contains("rotguss") {
        ("bronze-rotguss", "")
    } else if l.contains("hülsen") || l.contains("huelsen") {
        ("messing", "Hülsen")
    } else if l.contains("wasseruhren") {
        ("messing", "Wasseruhren")
    } else if l.contains("messing") {
        ("messing", "")
    } else if l.contains("kühler") || l.contains("kuehler") {
        // Alu-Cu-Verbundkühler: no catalog material fits.
        return None;
    } else if l.contains("ausbau") && l.contains("profil") {
        ("aluminium-gemischt", "Ausbau-Profile")
    } else if l.contains("profil") && l.contains("blank") {
        ("aluminium-profile", "")
    } else if l.contains("profil") && l.contains("farbe") {
        ("aluminium-profile", "Farbe")
    } else if l.contains("alu") && l.contains("anhaftung") {
        ("aluminium-gemischt", "5% Anhaftung")
    } else if l.contains("alu") {
        ("aluminium-gemischt", "")
    } else if l.contains("zink") {
        ("zink", "")
    } else if l.contains("mischschrott") {
        ("mischschrott", "")
    } else {
        return None;
    };
    let variant: &'static str = match (detail, tier) {
        ("", "") => "",
        ("", BULK_TIER) => BULK_TIER,
        ("Kerze", "") => "Kerze",
        ("Kerze", BULK_TIER) => "Kerze, ab 750 kg",
        ("50%", "") => "50%",
        ("50%", BULK_TIER) => "50%, ab 750 kg",
        ("60%", "") => "60%",
        ("60%", BULK_TIER) => "60%, ab 750 kg",
        ("70%", "") => "70%",
        ("70%", BULK_TIER) => "70%, ab 750 kg",
        ("80%", "") => "80%",
        ("80%", BULK_TIER) => "80%, ab 750 kg",
        ("Hülsen", "") => "Hülsen",
        ("Hülsen", BULK_TIER) => "Hülsen, ab 750 kg",
        ("Wasseruhren", "") => "Wasseruhren",
        ("Wasseruhren", BULK_TIER) => "Wasseruhren, ab 750 kg",
        ("Ausbau-Profile", "") => "Ausbau-Profile",
        ("Ausbau-Profile", BULK_TIER) => "Ausbau-Profile, ab 750 kg",
        ("Farbe", "") => "Farbe",
        ("Farbe", BULK_TIER) => "Farbe, ab 750 kg",
        ("5% Anhaftung", "") => "5% Anhaftung",
        ("5% Anhaftung", BULK_TIER) => "5% Anhaftung, ab 750 kg",
        _ => return None,
    };
    Some((material, variant))
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` holding
/// `<b>Anschrift:</b>` carries street + PLZ city as `<br>` lines, and
/// phone/e-mail come from `tel:`/`mailto:` hrefs (never token-split —
/// scraper glues the label to the number). Missing anchors mean the page
/// changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let anchor = "Anschrift:";
    let idx = imp.find(anchor).ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Anschrift-Block fehlt".to_owned(),
    })?;
    // The address <p>: from the enclosing paragraph start to </p>.
    let p_start = imp[..idx].rfind("<p").ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Anschrift-Block fehlt".to_owned(),
    })?;
    let p_end = imp[idx..].find("</p>").ok_or_else(|| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: "Anschrift-Block fehlt".to_owned(),
    })?;
    let block = &imp[p_start..idx + p_end];
    let mut lines = Vec::new();
    for part in block.split("<br") {
        let t = strip_fragment(part);
        if t.is_empty() || t == anchor {
            continue;
        }
        lines.push(t);
    }
    // "Blankenburger Str. 18 - 20" / "13089 Berlin" (last two lines).
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
    // Phone/e-mail from link hrefs (glued-text safe).
    let doc = Html::parse_document(imp);
    let a = Selector::parse("a").expect("valid selector");
    let mut phone = String::new();
    let mut email = String::new();
    for link in doc.select(&a) {
        if let Some(href) = link.value().attr("href") {
            if phone.is_empty() && href.starts_with("tel:") {
                phone = link
                    .text()
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
            } else if email.is_empty() {
                if let Some(addr) = href.strip_prefix("mailto:") {
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

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant (` />`) — drop everything up to the first '>' first, or the
/// remnant glues onto the text (" />13089" ≠ 5-digit PLZ).
fn strip_fragment(s: &str) -> String {
    let s = match s.find('>') {
        Some(i) => &s[i + 1..],
        None => s,
    };
    strip_tags(s)
}

fn parse(
    html: &str,
) -> Result<(Vec<(String, String, f64, &'static str)>, Vec<String>), IngestError> {
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    parse_panels(html, &mut rows, &mut skips)?;
    parse_tables(html, &mut rows, &mut skips)?;
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Panel pairs between the page heading and the Eisen/Buntmetall summary:
/// consecutive `div.textwidget`s alternate label / price. A label with no
/// following price ("Zinnschrott 99%") skips loudly; the "weitere
/// Metalle" link blob is prose (>120 Zeichen), not a label.
fn parse_panels(
    html: &str,
    rows: &mut Vec<(String, String, f64, &'static str)>,
    skips: &mut Vec<String>,
) -> Result<(), IngestError> {
    let start = html.find(PANEL_START).ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preispanels fehlen".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find(PANEL_END).ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preispanels fehlen".to_owned(),
    })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    let widget = Selector::parse("div.textwidget").expect("valid selector");
    let mut pending: Option<String> = None;
    let flush = |pending: &mut Option<String>, skips: &mut Vec<String>| {
        if let Some(label) = pending.take() {
            skips.push(format!("{label} (ohne Preis)"));
        }
    };
    for el in doc.select(&widget) {
        let text: String = el.text().collect();
        let text: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if text.is_empty() || text.contains(PANEL_START) {
            continue;
        }
        if text.len() > 120 {
            flush(&mut pending, skips); // Prosa, kein Label.
            continue;
        }
        if text.contains('€') {
            let Some(label) = pending.take() else {
                skips.push(format!("Preis ohne Label: {text}"));
                continue;
            };
            push_price_cell(&label, &text, rows, skips);
        } else {
            flush(&mut pending, skips);
            pending = Some(text);
        }
    }
    flush(&mut pending, skips);
    Ok(())
}

/// Summary tables (header "Schrottsorte"): Eisen rows live only here, the
/// Buntmetall rows repeat panel grades (deduped later after mapping).
fn parse_tables(
    html: &str,
    rows: &mut Vec<(String, String, f64, &'static str)>,
    skips: &mut Vec<String>,
) -> Result<(), IngestError> {
    let doc = Html::parse_document(html);
    let table = Selector::parse("table").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
    let row = Selector::parse("tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let tables: Vec<_> = doc
        .select(&table)
        .filter(|t| {
            t.select(&head)
                .any(|h| h.text().collect::<String>().trim() == "Schrottsorte")
        })
        .collect();
    if tables.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Schrottsorten-Tabellen fehlen".to_owned(),
        });
    }
    for table in tables {
        for tr in table.select(&row) {
            let cells: Vec<String> = tr
                .select(&cell)
                .map(|c| c.text().collect::<String>())
                .map(|t| t.replace(['\u{a0}'], " "))
                .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
                .collect();
            if cells.is_empty() {
                continue; // header row (th), not data
            }
            if cells.len() < 2 {
                skips.push(format!(
                    "Tabellenzeile unverständlich: {}",
                    cells.join(" / ")
                ));
                continue;
            }
            let label = cells[0].clone();
            if label.len() > 120 {
                continue; // Prosa, kein Label.
            }
            push_price_cell(&label, &cells[1], rows, skips);
        }
    }
    Ok(())
}

/// One price cell → base row plus optional bulk-tier row. Both tiers quote
/// €/kg; anything else skips loudly at the call site.
fn push_price_cell(
    label: &str,
    cell: &str,
    rows: &mut Vec<(String, String, f64, &'static str)>,
    skips: &mut Vec<String>,
) {
    let Some(base) = parse_eur(cell) else {
        skips.push(format!("{label} (Preis unverständlich: {cell})"));
        return;
    };
    let Some(unit) = unit_of(cell) else {
        skips.push(format!("{label} (Einheit unverständlich: {cell})"));
        return;
    };
    rows.push((label.to_owned(), String::new(), base, unit));
    if let Some((_, bulk_part)) = cell.split_once(BULK_MARKER) {
        match parse_eur(bulk_part) {
            Some(bulk) => rows.push((label.to_owned(), BULK_TIER.to_owned(), bulk, unit)),
            None => skips.push(format!("{label} (Staffel unverständlich: {cell})")),
        }
    }
}

/// Bespoke unit matcher for THIS page's price cells (live: "€/kg" with
/// dot decimals). Only kg/t exist here — anything else skips loudly at
/// the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase().replace([' ', '\u{a0}'], "");
    if lower.contains("€/kg") {
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
    use super::{extract_info, grade_for, parse, unit_of, BULK_TIER};

    const PANELS: &str = "<h1>Schrottankauf und Schrottpreise Berlin bei Kiro</h1>\
        <div class=\"textwidget\"><p>Kupfer Raff</p></div>\
        <div class=\"textwidget\"><span>9.45 €/kg</span><span class=\"fn-from-preis\" style=\"margin-left: 10px;\">(Ab 750 kg: 9.65 €)</span></div>\
        <div class=\"textwidget\"><p>Kupfer Millberry</p></div>\
        <div class=\"textwidget\"><span>10.10 €/kg</span><span class=\"fn-from-preis\" style=\"margin-left: 10px;\">(Ab 750 kg: 10.30 €)</span></div>\
        <div class=\"textwidget\"><p>Kupfer Kerze</p></div>\
        <div class=\"textwidget\"><span>9.70 €/kg</span><span class=\"fn-from-preis\" style=\"margin-left: 10px;\">(Ab 750 kg: 9.90 €)</span></div>\
        <div class=\"textwidget\"><p>Kupferkabel</p></div>\
        <div class=\"textwidget\"><span>3.15 €/kg</span><span class=\"fn-from-preis\" style=\"margin-left: 10px;\">(Ab 750 kg: 3.35 €)</span></div>\
        <div class=\"textwidget\"><p>Messing – Hülsen</p></div>\
        <div class=\"textwidget\"><span>5.65 €/kg</span><span class=\"fn-from-preis\" style=\"margin-left: 10px;\">(Ab 750 kg: 5.85 €)</span></div>\
        <div class=\"textwidget\"><p>Kabel 60%</p></div>\
        <div class=\"textwidget\"><span>4.60 €/kg</span><span class=\"fn-from-preis\" style=\"margin-left: 10px;\">(Ab 750 kg: 4.80 €)</span></div>\
        <div class=\"textwidget\"><p>Alu-Kupferkühler</p></div>\
        <div class=\"textwidget\"><span>3.20 €/kg</span><span class=\"fn-from-preis\" style=\"margin-left: 10px;\">(Ab 750 kg: 3.40 €)</span></div>\
        <div class=\"textwidget\"><p>Zink</p></div>\
        <div class=\"textwidget\"><span>1.50 €/kg</span><span class=\"fn-from-preis\" style=\"margin-left: 10px;\">(Ab 750 kg: 1.60 €)</span></div>\
        <div class=\"textwidget\"><p>Ankauf weitere Metalle Alufelgen mit Reifen Aluminiumkabel dick Alu-Offset-Bleche Sauber Alu-Blech-neu blank Blei Bleibatterie Edilstahl-V2A-V4A Edelstahl-Schredder Erdkabel-Kupfer Kupfer-Kabel mit Stecker und noch viel mehr Worte damit das sicher Prosa über hundertzwanzig Zeichen lang ist</p></div>\
        <div class=\"textwidget\"><p>Zinnschrott 99%</p></div>\
        <h2>Aktuelle Schrottpreise Berlin im Überblick</h2>";

    const TABLES: &str = "<table class=\"footable\"><tr id=\"table-head\"><th class=\"tg-n8rv\">Schrottsorte</th><th class=\"tg-o31p\">Preis/kg</th></tr>\
        <tr><td class=\"tg-0pja\">Mischschrott</td><td class=\"tg-nr20\"><span>0.12 €/kg</span></td></tr>\
        <tr><td class=\"tg-ljhg\">Scherenschrott / Gussschrott</td><td class=\"tg-ddb2\"><span>0.14 €/kg</span></td></tr></table>\
        <table class=\"footable\"><tr id=\"table-head\"><th class=\"tg-n8rv\">Schrottsorte</th><th class=\"tg-o31p\">Preis/kg</th></tr>\
        <tr><td class=\"tg-0pja\">Messing - Rotguss</td><td class=\"tg-nr20\"><span>8.10 €/kg</span><span class=\"fn-from-preis\" style=\"margin-left: 10px;\">(Ab 750 kg: 8.30 €)</span></td></tr>\
        <tr><td class=\"tg-0pja\">Messing Wasseruhren</td><td class=\"tg-nr20\"><span>2.40 €/kg</span><span class=\"fn-from-preis\" style=\"margin-left: 10px;\">(Ab 750 kg: 2.60 €)</span></td></tr></table>";

    #[test]
    fn panels_and_tables_parse_with_bulk_tiers() {
        let html = PANELS.to_owned() + TABLES;
        let (rows, skips) = parse(&html).expect("parses");
        // 8 panel pairs × 2 tiers + Mischschrott + Scherenschrott + Rotguss × 2 + Wasseruhren × 2.
        assert_eq!(rows.len(), 22, "{rows:?}");
        assert_eq!(
            rows[0],
            ("Kupfer Raff".to_owned(), String::new(), 9.45, "EUR/kg")
        );
        assert_eq!(
            rows[1],
            (
                "Kupfer Raff".to_owned(),
                BULK_TIER.to_owned(),
                9.65,
                "EUR/kg"
            )
        );
        let kuehler: Vec<_> = rows.iter().filter(|r| r.0.to_lowercase().contains("kühler")).collect();
        assert_eq!(kuehler.len(), 2, "cooler rows parse (mapping skips them)");
        // Zinnschrott without price skips loudly; prose blob is no label.
        assert_eq!(skips.len(), 1, "{skips:?}");
        assert!(skips[0].contains("Zinnschrott 99%"), "{skips:?}");
    }

    #[test]
    fn anchors_and_units_are_rejected_loudly() {
        // Missing panel window: loud error.
        assert!(parse(TABLES).is_err());
        // Missing tables: loud error.
        assert!(parse(PANELS).is_err());
        // Unknown unit: skipped loudly, valid rows survive.
        let html = (PANELS.to_owned() + TABLES).replacen("€/kg", "pro Sack", 1);
        let (rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 20);
        assert!(skips.iter().any(|s| s.contains("Kupfer Raff")), "{skips:?}");
        // Every row unparseable: loud error, not silent success.
        let html = (PANELS.to_owned() + TABLES).replace("€/kg", "pro Sack");
        assert!(parse(&html).is_err());
        assert_eq!(unit_of("10.10 €/kg"), Some("EUR/kg"));
        assert_eq!(unit_of("pro Sack"), None);
    }

    #[test]
    fn mapping_covers_live_labels() {
        assert_eq!(grade_for("Kupfer Raff", ""), Some(("kupfer-gemischt", "")));
        assert_eq!(
            grade_for("Kupfer Raff", BULK_TIER),
            Some(("kupfer-gemischt", BULK_TIER))
        );
        assert_eq!(
            grade_for("Kupfer Millberry", ""),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Kupfer Kerze", ""),
            Some(("kupfer-berry", "Kerze"))
        );
        assert_eq!(
            grade_for("Kupfer Kerze", BULK_TIER),
            Some(("kupfer-berry", "Kerze, ab 750 kg"))
        );
        assert_eq!(grade_for("Kupferkabel", ""), Some(("kabel-kupfer", "")));
        assert_eq!(grade_for("Kabel 50%", ""), Some(("kabel-kupfer", "50%")));
        assert_eq!(
            grade_for("Kabel 80%", BULK_TIER),
            Some(("kabel-kupfer", "80%, ab 750 kg"))
        );
        assert_eq!(grade_for("Messing", ""), Some(("messing", "")));
        assert_eq!(
            grade_for("Messing – Hülsen", ""),
            Some(("messing", "Hülsen"))
        );
        assert_eq!(
            grade_for("Messing – Wasseruhren", BULK_TIER),
            Some(("messing", "Wasseruhren, ab 750 kg"))
        );
        assert_eq!(
            grade_for("Messing – Rotguss", ""),
            Some(("bronze-rotguss", ""))
        );
        assert_eq!(
            grade_for("Alu Sauber", ""),
            Some(("aluminium-gemischt", ""))
        );
        assert_eq!(
            grade_for("Alu 5% Anhaftung", ""),
            Some(("aluminium-gemischt", "5% Anhaftung"))
        );
        assert_eq!(
            grade_for("Alu-Ausbau-Profile", ""),
            Some(("aluminium-gemischt", "Ausbau-Profile"))
        );
        assert_eq!(
            grade_for("Alu-Profile-Blank", ""),
            Some(("aluminium-profile", ""))
        );
        assert_eq!(
            grade_for("Alu-Profile-Farbe", ""),
            Some(("aluminium-profile", "Farbe"))
        );
        assert_eq!(grade_for("Zink", ""), Some(("zink", "")));
        assert_eq!(grade_for("Mischschrott", ""), Some(("mischschrott", "")));
        // Two sorts, one price: ambiguous → None.
        assert_eq!(grade_for("Scherenschrott / Gussschrott", ""), None);
        // Composite cooler: no catalog material.
        assert_eq!(grade_for("Alu-Kupferkühler", ""), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h1>Impressum</h1><p>Verantwortlich für den Inhalt der Website §6 MDStV: Kiro Kafedzhiev</p>\
            <p><b>Anschrift:</b><br />Blankenburger Str. 18 - 20<br />13089 Berlin</p>\
            <p><b>Telefon:</b><br /><a href=\"tel:01723086880\">+49 (0)172 - 30 86 880</a></p>\
            <p><b>E-Mail:</b><br /><a href=\"mailto:info@schrotthandel-berlin.com\">info@schrotthandel-berlin.com</a></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Blankenburger Str. 18 - 20");
        assert_eq!(info.postcode, "13089");
        assert_eq!(info.city, "Berlin");
        assert_eq!(info.phone, "+49 (0)172 - 30 86 880");
        assert_eq!(info.email, "info@schrotthandel-berlin.com");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
