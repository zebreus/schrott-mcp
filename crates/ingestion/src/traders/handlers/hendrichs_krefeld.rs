//! Josef Hendrichs Metallhandels GmbH & Co. KG (Krefeld): Wix-CMS
//! `font_2`-Absätze auf `/ankaufspreise-kleinanlieferer` — keine Tabelle,
//! sondern je Material eine Zeile `Label: Basispreis (bis 100kg) +
//! Von-bis-Spanne (über 100kg)`, z. B. `Kupfer, Millberry: 10,40 €
//! 10,50€ - 10,60€`. Header (`Tagesaktuelle Preise pro KG … von / bis`)
//! belegt kg als Seiteneinheit; `Mischschrott` trägt die zweite Stufe als
//! `ab 1000kg`. Basis → `exact`/1.0, Spanne → `range`/0.5 (VANA-Muster).
//! Die Staffel steht im `variant` (`bis 100kg`/`über 100kg`/`ab 1000kg`,
//! mit Händlersorte wo nötig: `38% …`, `Geschirr 5% …`), das Rohlabel in
//! `notes`. Tückisch: Labels sind über mehrere Spans zerschnitten
//! (`Kupfer, R` + `aff. 95%`), daher Textknoten ohne Trenner fügen (visuelle
//! Wahrheit) — mit Leerzeichen käme `R aff.` heraus und das Raff-Mapping
//! griffe nicht. Kommalose Zahlen (`1000kg`) sind keine Preise und werden
//! gezielt ignoriert; `Kats … auf Anfrage` skippt laut. Kein Seitendatum
//! (`täglich um 8:00` ist Rhythmus-Prosa) → `published_at` None.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-krefeld-josef-hendrichs-metallhandels";
/// Bespoke, live-verified price URL (canonical, 200 am 28.09.2026). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://www.hendrichs-metall.de/ankaufspreise-kleinanlieferer";
/// Bespoke, live-verified impressum URL (footer/canonical, 200 am
/// 28.09.2026). A move fails the step loudly — never guessed.
pub const IMPRESSUM_URL: &str = "https://www.hendrichs-metall.de/impressum";

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
    let mut prices = Vec::with_capacity(rows.len() * 2);
    // Doppelblock-Guard: gleiche (Material, Variante, Preis) nur einmal.
    let mut seen = std::collections::HashSet::new();
    for (label, tiers, unit) in rows {
        let Some((material, grade)) = grade_for(&label) else {
            skipped_labels.push(label);
            continue;
        };
        for (tier, price, min, max) in tiers {
            if price == 0.0 {
                skipped_labels.push(format!("{label} ({tier}: Preis 0,00)"));
                continue;
            }
            // Starre Templates: alle (Sorte × Staffel)-Kombis dieser Seite
            // als &'static str, die Sorte steht zusätzlich in `notes`.
            let variant = variant_for(grade, tier);
            if !seen.insert((material, variant, price.to_bits())) {
                continue;
            }
            let (price_kind, confidence, price_min, price_max) = match (min, max) {
                (Some(lo), Some(hi)) => ("range", Some(0.5), Some(lo), Some(hi)),
                _ => ("exact", Some(1.0), None, None),
            };
            prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit,
                price_kind,
                price_min,
                price_max,
                confidence,
                label: label.clone(),
            });
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

/// Explicit label → (material, trader grade) mapping. Anything unlisted is
/// skipped. Specific-before-generic: `millberry`/`raff`/`kabel` vor nichts —
/// hier steht kein nacktes `Kupfer`, aber `Raff.` darf nie als Millberry
/// durchgehen. `v4a` steht vor `v2a` (reine Vorsicht, kein Präfix-Risiko).
/// `Kats` hat keinen Katalogpreis in kg (EUR/Stk) und skippt laut.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("kabel") {
        if l.contains("alu") {
            None
        } else {
            Some(("kabel-kupfer", "38%"))
        }
    } else if l.contains("raff") {
        Some(("kupfer-gemischt", "Raff 95%"))
    } else if l.contains("messing") {
        Some(("messing", ""))
    } else if l.contains("rotguss") || l.contains("bronze") {
        Some(("bronze-rotguss", ""))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", ""))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("blei") {
        Some(("blei", ""))
    } else if l.contains("motor") {
        Some(("elektromotoren", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("geschirr") {
        Some(("aluminium-blech", "Geschirr 5%"))
    } else if l.contains("alu") {
        // Nur `Alu, neu` (Neuschrott, vgl. Vedders `Alu-Bleche neu Blank`
        // → blech). Generisches `Aluminium` ohne Sorte wäre gemischt —
        // diese Seite nennt keines.
        Some(("aluminium-blech", "neu"))
    } else {
        None
    }
}

/// Alle (Sorte × Staffel)-Kombis dieser Seite als starre Templates, damit
/// zwei Sorten eines Materials nie auf einen Current-Preis kollabieren.
/// Unbekannte Kombi fällt auf die Staffel zurück (Sorte bleibt in `notes`).
fn variant_for(grade: &str, tier: &'static str) -> &'static str {
    match (grade, tier) {
        ("", "bis 100kg") => "bis 100kg",
        ("", "über 100kg") => "über 100kg",
        ("", "ab 1000kg") => "ab 1000kg",
        ("38%", "bis 100kg") => "38% bis 100kg",
        ("38%", "über 100kg") => "38% über 100kg",
        ("Raff 95%", "bis 100kg") => "Raff 95% bis 100kg",
        ("Raff 95%", "über 100kg") => "Raff 95% über 100kg",
        ("Geschirr 5%", "bis 100kg") => "Geschirr 5% bis 100kg",
        ("Geschirr 5%", "über 100kg") => "Geschirr 5% über 100kg",
        ("neu", "bis 100kg") => "neu bis 100kg",
        ("neu", "über 100kg") => "neu über 100kg",
        _ => tier,
    }
}

/// Bespoke contact extraction for THIS impressum only: durchnummerierte
/// `p.font_2`-Blöcke — nach der `Impressum`-Überschrift folgen Firma,
/// Straße, `PLZ Ort`; nach `Kontakt` kommen `Telefon:`- und `Mail:`-Zeilen
/// (E-Mail bevorzugt aus dem mailto-href). Fehlende Anker → lauter Error,
/// nie raten/fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p = Selector::parse("p").expect("valid selector");
    let link = Selector::parse("a").expect("valid selector");
    let paras: Vec<ElementRef> = doc.select(&p).collect();
    let texts: Vec<String> = paras.iter().map(|e| para_text(e)).collect();
    let fail = |detail: &str| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: detail.to_owned(),
    };
    let imp_idx = texts
        .iter()
        .position(|t| t == "Impressum")
        .ok_or_else(|| fail("Impressum-Anker fehlt"))?;
    // Firma, Straße, PLZ Ort = nächste nicht-leere Blöcke.
    let mut after = texts[imp_idx + 1..]
        .iter()
        .filter(|t| !t.is_empty())
        .take(3);
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if let (Some(_firm), Some(st), Some(pc_city)) = (after.next(), after.next(), after.next()) {
        street = st.clone();
        let mut it = pc_city.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = ci.to_owned();
            }
        }
    }
    let kontakt_idx = texts
        .iter()
        .position(|t| t == "Kontakt")
        .ok_or_else(|| fail("Kontakt-Anker fehlt"))?;
    let mut phone = String::new();
    let mut email = String::new();
    for (el, t) in paras[kontakt_idx + 1..]
        .iter()
        .zip(texts[kontakt_idx + 1..].iter())
    {
        if phone.is_empty() {
            if let Some(v) = t.strip_prefix("Telefon:") {
                phone = v.trim().to_owned();
            }
        }
        if email.is_empty() {
            if let Some(addr) = el
                .select(&link)
                .filter_map(|a| a.value().attr("href"))
                .find_map(|h| h.strip_prefix("mailto:"))
            {
                email = addr.trim().to_owned();
            } else if t.contains('@') {
                email = t
                    .split_whitespace()
                    .find(|w| w.contains('@'))
                    .unwrap_or_default()
                    .trim_matches([',', ';'])
                    .to_owned();
            }
        }
        if !phone.is_empty() && !email.is_empty() {
            break;
        }
    }
    if street.is_empty() {
        return Err(fail("Adress-Block fehlt"));
    }
    if phone.is_empty() && email.is_empty() {
        return Err(fail("keine Kontaktdaten gefunden"));
    }
    Ok(TraderInfo {
        street,
        postcode,
        city,
        phone,
        email,
    })
}

/// Visual-truth paragraph text: Textknoten OHNE Trenner fügen (Wix
/// zerschneidet Wörter in Spans: `Kupfer, R`+`aff. 95%` → `Raff.`), dann
/// Whitespace kollabieren. Mit Leerzeichen-Join käme `R aff.` heraus.
fn para_text(el: &ElementRef) -> String {
    let glued: String = el.text().collect();
    glued
        .replace(['\u{a0}', '\u{200b}'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Label + deutsche Komma-Preise eines Absatzes trennen. Nur Tokens mit
/// Komma zählen (`1000kg` ist Menge, kein Preis). Gibt (Label, Preise).
fn split_label_prices(text: &str) -> (String, Vec<f64>) {
    let bytes = text.as_bytes();
    let mut i = 0;
    let mut first_start: Option<usize> = None;
    let mut prices = Vec::new();
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let mut j = i;
            while j < bytes.len()
                && (bytes[j].is_ascii_digit() || bytes[j] == b'.' || bytes[j] == b',')
            {
                j += 1;
            }
            let tok = &text[i..j];
            if tok.contains(',') {
                if first_start.is_none() {
                    first_start = Some(i);
                }
                if let Some(p) = parse_eur(tok) {
                    prices.push(p);
                }
            }
            i = j;
        } else {
            // UTF-8-safe vorrücken.
            i += text[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
        }
    }
    let label = first_start
        .map(|s| {
            text[..s]
                .replace(['\u{a0}', '\u{200b}'], " ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .trim_end_matches(':')
                .trim()
                .to_owned()
        })
        .unwrap_or_default();
    (label, prices)
}

/// Eine Staffel: (Staffelname, Preis, min, max). Basis exakt, Spanne range.
/// `row` ist der volle Zeilentext (die `ab 1000kg`-Marke steht hinter dem
/// Label und wäre dort nicht sichtbar).
fn tiers_for(row: &str, prices: &[f64]) -> Vec<(&'static str, f64, Option<f64>, Option<f64>)> {
    if prices.is_empty() {
        return vec![];
    }
    let mut out = vec![("bis 100kg", prices[0], None, None)];
    match &prices[1..] {
        [] => {}
        [single] => {
            let tier = if row.to_lowercase().contains("ab") {
                "ab 1000kg"
            } else {
                "über 100kg"
            };
            out.push((tier, *single, None, None));
        }
        rest => {
            let lo = rest.iter().cloned().fold(f64::INFINITY, f64::min);
            let hi = rest.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            out.push(("über 100kg", hi, Some(lo), Some(hi)));
        }
    }
    out
}

fn parse(
    html: &str,
) -> Result<
    (
        Vec<(
            String,
            Vec<(&'static str, f64, Option<f64>, Option<f64>)>,
            &'static str,
        )>,
        Vec<String>,
    ),
    IngestError,
> {
    // Fenster: Preisblock nur — nie die Ganzseite (Nav/Footer-/JSON-Zahlen
    // dürfen sich nicht mit Labels zu Phantompreisen paaren).
    let start = html
        .find("Tagesaktueller Stand")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock-Anker fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end_rel = tail
        .find("Preis auf Anfrage")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock-Ende fehlt".to_owned(),
        })?;
    let window = &tail[..end_rel + "Preis auf Anfrage".len()];
    let doc = Html::parse_fragment(window);
    let p = Selector::parse("p").expect("valid selector");
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for el in doc.select(&p) {
        let text = para_text(&el);
        if text.len() < 2 || text.len() > 120 {
            continue; // Prosa/Trennzeilen, kein Label.
        }
        let (label, prices) = split_label_prices(&text);
        if prices.is_empty() {
            if text.to_lowercase().contains("anfrage") {
                skipped.push(format!("{text} (Preis auf Anfrage, kein Preis)"));
            } else if text.contains('€') {
                skipped.push(format!("{text} (kein Preis)"));
            }
            // Header ohne € (`Tagesaktuelle Preise pro KG …`, `von / bis`):
            // kein Label, still weiter.
            continue;
        }
        if label.is_empty() {
            skipped.push(format!("{text} (Label unverständlich)"));
            continue;
        }
        // Einheit: Header belegt `pro KG`; explizit Fremdes (`pro Sack`,
        // Stück …) skippt laut — Kilo-als-Tonne wäre ein 1000×-Fehler.
        let Some(unit) = unit_of(&text) else {
            skipped.push(format!("{label} (Einheit unverständlich: {text})"));
            continue;
        };
        rows.push((label.clone(), tiers_for(&text, &prices), unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock leer".to_owned(),
        });
    }
    Ok((rows, skipped))
}

/// Bespoke unit matcher für DIESE Zeilen (live: `€`-Preise, Header `pro
/// KG`). Nur kg existiert hier — alles andere skippt laut am Call-Site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("stück") || lower.contains("stk") || lower.contains("sack") {
        None
    } else if cell.contains('€') || lower.contains("kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, split_label_prices};

    // Reale Live-Ausschnitte vom 28.09.2026 (verbatim, nur gekürzte
    // Style-Attribute): zerschnittene Spans (`Kupfer, R`+`aff.`), &nbsp;- und
    // &euro;-Entities, `ab 1000kg`-Sonderstaffel, `auf Anfrage`-Zeile.
    const FIXTURE: &str = "<p class=\"font_2\" style=\"font-size:26px;\"><span><span><span><span>Tagesaktueller Stand t&auml;glich um 8:00</span></span></span></span></p>\
        <p class=\"font_2\" style=\"font-size:16px;\"><span><span><span><span>Ta</span></span></span></span><span><span><span><span>gesaktuelle Preise  pro KG &nbsp; &nbsp; &nbsp;  bis 100kg&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;  über 100kg&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;   &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;       von&nbsp;   &nbsp;/&nbsp; &nbsp; bis</span></span></span></span></p>\
        <p class=\"font_2\" style=\"font-size:16px;\"><span><span>Alu-Geschirr 5%:&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;1,20</span></span><span>&euro;</span><span><span><span>&nbsp; &nbsp;</span></span></span><span><span><span> &nbsp; &nbsp;&nbsp;</span>&nbsp; &nbsp; &nbsp; 1,25</span></span><span>&euro; - 1,30&euro; &nbsp;</span></p>\
        <p class=\"font_2\" style=\"font-size:16px;\"><span><span><span><span>Kupfer, Millberry:&nbsp; &nbsp; &nbsp; 10,40</span></span></span></span></span><span><span><span><span>&euro;&nbsp; &nbsp; &nbsp; 10,50&euro; - 10,60&euro;</span></span></span></span></p>\
        <p class=\"font_2\" style=\"font-size:16px;\"><span><span><span><span>Kupfer, R</span></span></span></span></span><span><span><span><span>aff. 95%:&nbsp; &nbsp; &nbsp; 9,40</span></span></span></span></span><span><span><span><span>&euro;&nbsp; &nbsp; &nbsp; 9,50&euro; - 9,60&euro;</span></span></span></span></p>\
        <p class=\"font_2\" style=\"font-size:16px;\"><span><span><span><span>Messing:&nbsp; &nbsp; &nbsp;5,50&euro;&nbsp; &nbsp; &nbsp; 5,60&euro; - 5,70&euro;</span></span></span></span></span></p>\
        <p class=\"font_2\" style=\"font-size:16px;\"><span><span><span><span>V2A:&nbsp; &nbsp; &nbsp; 0,80&euro;&nbsp; &nbsp; &nbsp; 0,85&euro; - 0,95&euro;</span></span></span></span></span></p>\
        <p class=\"font_2\" style=\"font-size:16px;\"><span><span><span><span>Mischschrott:&nbsp; &nbsp; &nbsp; 0,15 &euro; &nbsp; &nbsp; &nbsp; ab 1000kg &nbsp; &nbsp; &nbsp; 0,17 &euro;</span></span></span></span></span></p>\
        <p class=\"font_2\" style=\"font-size:16px;\"><span><span><span><span>Kats + LKW Kats:&nbsp; &nbsp; &nbsp;Preis auf Anfrage</span></span></span></span></p>";

    #[test]
    fn block_parses_with_tiers() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 6, "6 Preiszeilen: {rows:?}");
        assert_eq!(skips.len(), 1, "nur Kats skippt: {skips:?}");
        assert!(skips[0].contains("Kats") && skips[0].contains("Anfrage"));
        // Millberry: Basis exakt + Range-Staffel.
        let mill = rows
            .iter()
            .find(|(l, _, _)| l.contains("Millberry"))
            .expect("millberry");
        assert_eq!(mill.0, "Kupfer, Millberry");
        assert_eq!(mill.1.len(), 2);
        assert_eq!(mill.1[0], ("bis 100kg", 10.4, None, None));
        assert_eq!(mill.1[1], ("über 100kg", 10.6, Some(10.5), Some(10.6)));
        assert_eq!(mill.2, "EUR/kg");
        // Zerschnittenes Label fügt sich visuell: `Raff.`, nicht `R aff.`.
        let raff = rows
            .iter()
            .find(|(l, _, _)| l.contains("Raff"))
            .expect("raff");
        assert_eq!(raff.0, "Kupfer, Raff. 95%");
        assert_eq!(raff.1[0], ("bis 100kg", 9.4, None, None));
        assert_eq!(raff.1[1], ("über 100kg", 9.6, Some(9.5), Some(9.6)));
        // Mischschrott: `1000kg` ist keine 1000,00 — zwei exakte Staffeln.
        let misch = rows
            .iter()
            .find(|(l, _, _)| l.contains("Misch"))
            .expect("misch");
        assert_eq!(misch.1.len(), 2);
        assert_eq!(misch.1[0], ("bis 100kg", 0.15, None, None));
        assert_eq!(misch.1[1], ("ab 1000kg", 0.17, None, None));
    }

    #[test]
    fn label_split_keeps_percent_in_label() {
        let (label, prices) =
            split_label_prices("Kupfer-Kabel 38% (ohne St.): 3,20 € 3,30 € - 3,40€");
        assert_eq!(label, "Kupfer-Kabel 38% (ohne St.)");
        assert_eq!(prices, vec![3.2, 3.3, 3.4]);
        let (label, prices) = split_label_prices("Alu-Geschirr 5%: 1,20 € 1,25 € - 1,30€");
        assert_eq!(label, "Alu-Geschirr 5%");
        assert_eq!(prices, vec![1.2, 1.25, 1.3]);
        // Kommalose Menge ist kein Preis.
        let (_, prices) = split_label_prices("Mischschrott: 0,15 € ab 1000kg 0,17 €");
        assert_eq!(prices, vec![0.15, 0.17]);
    }

    #[test]
    fn anchors_fail_loudly() {
        assert!(parse("<html><body><p>Neu hier</p></body></html>").is_err());
        let no_end = FIXTURE.replacen("Preis auf Anfrage", "Preis nach Laune", 1);
        assert!(parse(&no_end).is_err());
        // Jede Zeile ohne Einheit: lauter Error statt stillem Erfolg.
        let no_units = FIXTURE
            .replace("&euro;", "")
            .replace('€', "")
            .replace("KG", "Sack")
            .replace("kg", "Sack");
        assert!(parse(&no_units).is_err());
    }

    #[test]
    fn mapping_covers_live_board() {
        assert_eq!(
            grade_for("Kupfer, Millberry"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Kupfer, Raff. 95%"),
            Some(("kupfer-gemischt", "Raff 95%"))
        );
        assert_eq!(
            grade_for("Kupfer-Kabel 38% (ohne St.)"),
            Some(("kabel-kupfer", "38%"))
        );
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Rotguss"), Some(("bronze-rotguss", "")));
        assert_eq!(grade_for("V2A"), Some(("edelstahl-v2a", "")));
        assert_eq!(grade_for("V4A"), Some(("edelstahl-v4a", "")));
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(grade_for("E-Motore"), Some(("elektromotoren", "")));
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(
            grade_for("Alu-Geschirr 5%"),
            Some(("aluminium-blech", "Geschirr 5%"))
        );
        assert_eq!(grade_for("Alu, neu"), Some(("aluminium-blech", "neu")));
        // Kein Katalog-kg-Preis für Katalysatoren → laut skippen.
        assert_eq!(grade_for("Kats + LKW Kats"), None);
        assert_eq!(grade_for("Irgendwas Neues"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        // Reale Blockform: `Impressum`-Überschrift, Firma/Straße/PLZ-Ort,
        // `Kontakt`-Überschrift, `Telefon:`-Zeile, Mail per mailto.
        let imp = "<p class=\"font_2\">Impressum</p>\
            <p class=\"font_2\">Josef Hendrichs Metallhandels GmbH &amp; Co. KG</p>\
            <p class=\"font_2\">Neue Ritterstra&szlig;e 27</p>\
            <p class=\"font_2\">47805 Krefeld&nbsp;</p>\
            <p class=\"font_2\">\u{200b}</p>\
            <p class=\"font_2\">Kontakt</p>\
            <p class=\"font_2\">Telefon:&nbsp;02151/821060</p>\
            <p class=\"font_2\">Fax: 02151/82106-21</p>\
            <p class=\"font_2\">Mail: <a href=\"mailto:Kundenbetreuung@hendrichs-metall.de\">Kundenbetreuung@hendrichs-metall.de</a></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Neue Ritterstraße 27");
        assert_eq!(info.postcode, "47805");
        assert_eq!(info.city, "Krefeld");
        assert_eq!(info.phone, "02151/821060");
        assert_eq!(info.email, "Kundenbetreuung@hendrichs-metall.de");
        // Redesign ohne Anker scheitert laut.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(extract_info("<p>Impressum</p><p>Kontakt</p>").is_err());
    }
}
