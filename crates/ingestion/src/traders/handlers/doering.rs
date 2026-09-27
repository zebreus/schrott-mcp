//! Schrott- & Metallhandel Döring (Flörsheim-Dalsheim): exact per-kg
//! prices in the "Preisliste" table on the Ankauf page (`ankauf.php`,
//! reached via the homepage "Ankauf" nav — the homepage carries the same
//! table, but the Ankauf page is the dedicated price page). Columns `#`
//! / `Bezeichnung` / `Hinweis` / `Preis / Einheit` (live all "€ / Kg").
//!
//! Deliberately OUT of the window: the "Angebote" carousel above the
//! table (teaser rates in MIXED units — "Schrottpreis 140",
//! "Bremsscheiben 200" read per-tonne against "Millbery 9.00" per-kg —
//! and stale against the table: Kupfer 8.60 vs 8.80, Zinn 16 vs 20.-,
//! Hartmetall 30.- vs 40.00). Its date ("Angebote - 27.09.2026") belongs
//! to the carousel, not the table → `published_at` is None.
//!
//! The Katalysator row names no price ("Befundung nach Edelmetallgehalt")
//! and becomes a `ScrapedAcceptance`; the parking-notice joke row
//! ("XXX € / XXX") skips loudly.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "rp-florsheim-dalsheim-schrott-metallhandel-doring";
/// Bespoke, live-verified impressum URL (site nav's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "https://schrott-metallhandel-doering.de/impressum.php";

/// Ankauf page: Preisliste table + Katalysatoren-Ankauf (found live from
/// the homepage nav — the audit named no URL).
pub const URL: &str = "https://schrott-metallhandel-doering.de/ankauf.php";

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
    let (_, rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::new();
    let mut acceptances = Vec::new();
    for (label, hinweis, price) in rows {
        match price {
            Some(p) => match grade_for(&label) {
                Some((material, variant)) => prices.push(ScrapedPrice {
                    material,
                    variant,
                    price: p,
                    currency: "EUR",
                    unit: "EUR/kg",
                    price_kind: "exact",
                    price_min: None,
                    price_max: None,
                    confidence: Some(1.0),
                    label,
                }),
                None => skipped_labels.push(label),
            },
            // Priceless rows are acceptance candidates (Katalysatoren nach
            // Befundung), never silent drops.
            None => match acceptance_for(&label, &hinweis) {
                Some((material, conditions)) => acceptances.push(ScrapedAcceptance {
                    material,
                    conditions: conditions.to_owned(),
                    label,
                }),
                None => skipped_labels.push(format!("{label} (kein Preis)")),
            },
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        prices,
        acceptances,
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific-before-generic throughout: "Millbery blankes
/// Kupfer" contains "kupfer", "Lackprofil" contains "profil", and the
/// "Alu Kupfer Kühler" row must die on the kühler arm before the kupfer
/// arm claims it.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millbery") || l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kühler") || l.contains("kuehler") {
        // Cu/Alu-Mischprodukt ohne Katalogmaterial (Vorschlag: kühler).
        None
    } else if l.contains("felgen") {
        // Alufelgen ohne Katalogmaterial (Vorschlag: aluminium-felgen).
        None
    } else if l.contains("zinn") {
        // Grade IS the variant ("Zinn Ab 90%").
        if l.contains("90%") {
            Some(("zinn", "90%"))
        } else {
            Some(("zinn", ""))
        }
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("bremsscheiben") {
        // Grauguss-Bremsscheiben → Gussbruch, eigene Variante.
        Some(("eisenschrott-gussbruch", "Bremsscheiben"))
    } else if l.contains("haushaltskabel") {
        // NYM-Haushaltskabel ist normiert Kupfer (kein Alu-Haushaltskabel
        // am Markt); Unsicherheit siehe Handler-Report.
        Some(("kabel-kupfer", "Haushalt"))
    } else if l.contains("e motor") || l.contains("e-motor") || l.contains("elektromotor") {
        Some(("elektromotoren", ""))
    } else if l.contains("iso") {
        // ISO-Profil mit Kunststoff-Trennsteg → generisch, nie blank.
        Some(("aluminium-gemischt", "ISO"))
    } else if l.contains("geschirr") {
        Some(("aluminium-blech", "Geschirr"))
    } else if l.contains("lack") {
        // Lackiertes Profil → generisch, nie blank.
        Some(("aluminium-gemischt", "Lack"))
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("hartmetall") || l.contains("widea") || l.contains("widia") {
        Some(("hartmetall", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l == "va" {
        // Blankes "VA" → generisches Edelstahl, nie eine spezifische Sorte.
        Some(("edelstahl-gemischt", ""))
    } else if l.contains("profil") {
        Some(("aluminium-profile", ""))
    } else {
        None
    }
}

/// Priceless rows → acceptance. Only the Katalysator row qualifies
/// (assay-based purchase, "Befundung nach Edelmetallgehalt" as
/// conditions); everything else (parking notice) skips loudly.
fn acceptance_for<'a>(label: &str, hinweis: &'a str) -> Option<(&'static str, &'a str)> {
    if label.to_lowercase().contains("katalysator") {
        Some(("katalysatoren", hinweis))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after
/// "Angaben gemäß § 5 TMG:" holds firm lines + street + PLZ city, and
/// the `<p>` after "Kontakt:" holds "Telefon:" / "Telefax:" / "E-Mail:"
/// lines (note the trailing colons on both headings). Missing anchors
/// mean the page changed shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let anchor = |title: &str| {
        doc.select(&h2)
            .find(|h| h.text().collect::<String>().trim() == title)
    };
    let Some(addr_h) = anchor("Angaben gemäß § 5 TMG:") else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    let addr_p = addr_h
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    let mut lines = Vec::new();
    if let Some(p) = addr_p {
        for part in p.inner_html().split("<br") {
            let t = strip_fragment(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
    // "Am Trappenberg 7" / "67592 Flörsheim-Dalsheim" (last two lines).
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if lines.len() >= 2 {
        let last = lines.last().expect("len checked");
        let mut it = last.split_whitespace();
        if let (Some(pc), Some(rest)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                // Multi-word city ("Flörsheim-Dalsheim" is one token, but
                // never assume): keep the whole remainder.
                city = format!("{rest} {}", it.collect::<Vec<_>>().join(" "))
                    .trim_end()
                    .to_owned();
                street = lines[lines.len() - 2].clone();
            }
        }
    }
    let Some(kontakt_h) = anchor("Kontakt:") else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    let mut phone = String::new();
    let mut email = String::new();
    if let Some(p) = kontakt_h
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p")
    {
        for part in p.inner_html().split("<br") {
            let t = strip_fragment(part);
            if let Some(v) = t.strip_prefix("Telefon:") {
                if phone.is_empty() {
                    phone = v.trim().to_owned();
                }
            } else if let Some(v) = t.strip_prefix("E-Mail:") {
                // Own rule for mail (the phone-style take_while above
                // would stop at the first letter).
                email = v.split_whitespace().next().unwrap_or_default().to_owned();
            }
        }
    }
    if street.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    }
    if phone.is_empty() && email.is_empty() {
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

/// Strip tags from a `<br>`-split fragment (html5ever already decoded
/// entities). Only fragments that START with a tag remnant (` />…` after
/// the `<br` split point) drop everything up to the first '>' — a
/// fragment starting with text keeps its label.
fn strip_fragment(s: &str) -> String {
    let s = s.trim_start();
    let s = if s.starts_with('<') {
        match s.find('>') {
            Some(i) => &s[i + 1..],
            None => s,
        }
    } else {
        s
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

/// Bespoke price reader for THIS table: the page writes decimals with a
/// DOT ("0.150", "0.20", "3.00" €/kg — a thousands reading of "0.150"
/// would be a 1000× error on 15-cent Mischschrott, and no thousands-dot
/// occurs anywhere on the live page). Normalise dot → comma before the
/// shared helper; comma prices ("8,80") pass through untouched.
fn parse_preis(cell: &str) -> Option<f64> {
    parse_eur(&cell.replace('.', ","))
}

fn parse(
    html: &str,
) -> Result<
    (
        Option<String>,
        Vec<(String, String, Option<f64>)>,
        Vec<String>,
    ),
    IngestError,
> {
    // Window: the Preisliste table only — start AND end anchored
    // ("Preisliste" … "</table>"). The Angebote carousel above shares the
    // page but not the units; a whole-page walk would glue its teaser
    // rates onto table labels.
    let start = html.find("Preisliste").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisliste fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("</table>").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Preisliste offen".to_owned(),
    })?;
    let window = &tail[..end];
    // Head-content check (never the first table): the price header must
    // be in the window, or the page changed shape → loud error.
    if !(window.contains("Bezeichnung") && window.contains("Preis / Einheit")) {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste fehlt".to_owned(),
        });
    }
    let doc = Html::parse_fragment(window);
    let row = Selector::parse("tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    // Live quirk: rows never close (`<tr><td>…<tr><td>…`) — html5ever
    // auto-closes, so every data row still selects with 4 cells.
    let mut rows = Vec::new();
    let mut unit_skips = Vec::new();
    for tr in doc.select(&row) {
        let cells: Vec<String> = tr.select(&cell).map(|c| c.text().collect()).collect();
        if cells.len() < 4 {
            continue;
        }
        let norm = |s: &str| {
            s.replace(['\u{a0}'], " ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        };
        let bezeichnung = norm(&cells[1]);
        let hinweis = norm(&cells[2]);
        if bezeichnung.is_empty() {
            continue;
        }
        let label = if hinweis.is_empty() {
            bezeichnung.clone()
        } else {
            format!("{bezeichnung} {hinweis}")
        };
        let price = parse_preis(&cells[3]);
        if let Some(p) = price {
            // An unparseable unit is a loud skip, never a silent default:
            // a per-tonne price recorded as per-kg would be a 1000x error.
            if unit_of(&cells[3]).is_none() {
                unit_skips.push(format!(
                    "{label} (Einheit unverständlich: {})",
                    cells[3].trim()
                ));
                continue;
            }
            rows.push((label, hinweis, Some(p)));
        } else {
            // No parseable price (joke row, assay-based Katalysatoren):
            // kept for the acceptance path at the call site.
            rows.push((label, hinweis, None));
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste leer".to_owned(),
        });
    }
    Ok((None, rows, unit_skips))
}

/// Bespoke unit matcher for THIS table's Preis column (live: "€ / Kg"
/// on every priced row). Only kg exists here — anything else skips
/// loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    if cell.to_lowercase().contains("kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{acceptance_for, grade_for, parse, parse_preis};

    // Real live markup (rows unclosed in source, "XXX"/"€ € / €" rows
    // included, thead with bare <th> foster-parented away).
    const FIXTURE: &str = "<div><h1>Preisliste</h1></div>\
        <table><thead><th>#</th><th>Bezeichnung</th><th>Hinweis</th>\
        <th>Preis / Einheit</th></thead><tbody>\
        <tr><td>1</td><td>Zinn</td><td><small><i>Ab 90%</i></small></td><td>20.- € / Kg</td>\
        <tr><td>2</td><td>Mischschrott</td><td><small><i></i></small></td><td>0.150 € / Kg</td>\
        <tr><td>10</td><td>Kupfer</td><td><small><i></i></small></td><td>8,80 € / Kg</td>\
        <tr><td>12</td><td>Millbery  blankes Kupfer </td><td><small><i></i></small></td><td>9,20 € / Kg </td>\
        <tr><td>14</td><td>Ein - Ausfahrten nicht Parken</td><td><small><i>Bitte daran halten danke :-)</i></small></td><td>XXX € / XXX</td>\
        <tr><td>15</td><td>Katalysator Keramik Metall Lkw Biogas </td><td><small><i>Befundung nach Edelmetallgehalt</i></small></td><td>€ € / €</td>\
        <tr><td>16</td><td>VA</td><td><small><i></i></small></td><td>0.70 € / Kg</td>\
        </tbody></table>";

    #[test]
    fn table_parses_with_priceless_rows_kept() {
        let (_, rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty());
        assert_eq!(rows.len(), 7);
        assert_eq!(rows[0].0, "Zinn Ab 90%");
        assert_eq!(rows[0].2, Some(20.0));
        assert_eq!(rows[1].0, "Mischschrott");
        assert_eq!(rows[1].2, Some(0.15));
        assert_eq!(rows[2].0, "Kupfer");
        assert_eq!(rows[2].2, Some(8.8));
        // Priceless rows survive for the acceptance path.
        assert_eq!(rows[4].2, None);
        assert_eq!(
            rows[5].0,
            "Katalysator Keramik Metall Lkw Biogas Befundung nach Edelmetallgehalt"
        );
        assert_eq!(rows[5].1, "Befundung nach Edelmetallgehalt");
        assert_eq!(rows[5].2, None);
    }

    #[test]
    fn decimal_dot_is_not_thousands() {
        // Live convention: dot = decimal ("0.150" € = 15 Cent, not 150 €).
        assert_eq!(parse_preis("0.150 € / Kg"), Some(0.15));
        assert_eq!(parse_preis("0.20 € / Kg"), Some(0.2));
        assert_eq!(parse_preis("3.00 € / Kg"), Some(3.0));
        assert_eq!(parse_preis("8,80 € / Kg"), Some(8.8));
        assert_eq!(parse_preis("20.- € / Kg"), Some(20.0));
        assert_eq!(parse_preis("XXX € / XXX"), None);
    }

    #[test]
    fn wrong_table_and_unit_are_rejected_loudly() {
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (_, rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 7);
        let html = FIXTURE.replacen("€ / Kg", "pro Sack", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 6);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Zinn"));
        // No price table at all: loud error, not silent success.
        assert!(parse("<html><body><p>Neu hier</p></body></html>").is_err());
    }

    #[test]
    fn mapping_orders_specific_before_generic() {
        assert_eq!(
            grade_for("Millbery blankes Kupfer"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(grade_for("Kupfer"), Some(("kupfer-gemischt", "")));
        assert_eq!(
            grade_for("Alu Kupfer Kühler sauber"),
            None,
            "Kühler cramt nicht"
        );
        assert_eq!(grade_for("Alufelgen"), None, "kein Felgen-Material");
        assert_eq!(grade_for("Zinn Ab 90%"), Some(("zinn", "90%")));
        assert_eq!(
            grade_for("Bremsscheiben"),
            Some(("eisenschrott-gussbruch", "Bremsscheiben"))
        );
        assert_eq!(
            grade_for("Haushaltskabel"),
            Some(("kabel-kupfer", "Haushalt"))
        );
        assert_eq!(
            grade_for("ISO Profile"),
            Some(("aluminium-gemischt", "ISO"))
        );
        assert_eq!(grade_for("Blanke Profile"), Some(("aluminium-profile", "")));
        assert_eq!(
            grade_for("Lackprofil"),
            Some(("aluminium-gemischt", "Lack"))
        );
        assert_eq!(
            grade_for("Aluminium Geschirr"),
            Some(("aluminium-blech", "Geschirr"))
        );
        assert_eq!(grade_for("E Motor"), Some(("elektromotoren", "")));
        assert_eq!(grade_for("VA"), Some(("edelstahl-gemischt", "")));
        assert_eq!(grade_for("Hartmetall"), Some(("hartmetall", "")));
    }

    #[test]
    fn katalysator_becomes_acceptance() {
        assert_eq!(
            acceptance_for(
                "Katalysator Keramik Metall Lkw Biogas Befundung nach Edelmetallgehalt",
                "Befundung nach Edelmetallgehalt"
            ),
            Some(("katalysatoren", "Befundung nach Edelmetallgehalt"))
        );
        assert_eq!(
            acceptance_for(
                "Ein - Ausfahrten nicht Parken Bitte daran halten danke :-)",
                "Bitte daran halten danke :-)"
            ),
            None
        );
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h1><span>Impressum</span></h1>\
            <h2>Angaben gemäß § 5 TMG:</h2>\
            <p>Schrott und Metallhandel Döring GmbH<br />Am Trappenberg 7<br />67592 Flörsheim-Dalsheim</p>\
            <h2>Vertreten durch:</h2><p>GF Daniel Döring</p>\
            <h2>Kontakt:</h2><p>Telefon: 06243 900 204 2<br />Telefax: 06243 900 204 3<br />E-Mail: kat-guru@gmx.de</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Am Trappenberg 7");
        assert_eq!(info.postcode, "67592");
        assert_eq!(info.city, "Flörsheim-Dalsheim");
        assert_eq!(info.phone, "06243 900 204 2");
        assert_eq!(info.email, "kat-guru@gmx.de");
        // Redesign without anchors fails loudly.
        assert!(super::extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
