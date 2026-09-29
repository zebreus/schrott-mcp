//! H. Henken GmbH (Friesoythe): TablePress-Tagespreise
//! (`/preise-schrott-und-buntmetalle/`) mit den Spalten Bezeichnung /
//! Preis / Einheit (`kg`, `Tonne`, `Stück`). Preise stehen mit Punkt als
//! Dezimaltrenner (`1.95 €`, `110.00 €`) — `parse_eur` löst das korrekt,
//! solange kein Tausenderpunkt mit 3er-Gruppe folgt. Gültigkeitsdatum
//! steht als `Stand: TT.MM.JJJJ` über der Tabelle. Gemischte
//! Verbundsorten (`Kupfer-Alu-Kühler`, `Kupfer-Messing-Kühler`) sowie
//! Altautos und Batterien haben keinen Katalogeintrag und werden laut
//! geskippt (vgl. `hofmann_metall`: Kühler/Batterien → `None`);
//! Alu-Felgen landen wie bei `albus_leipzig` auf `aluminium-guss`.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "ni-friesoythe-h-henken-friesoythe-auch-cloppenburg-pap";
/// Bespoke, live-verified price URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://henkengmbh.de/preise-schrott-und-buntmetalle/";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://henkengmbh.de/impressum/";

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

/// Explicit label → (material, variant) mapping. Arms are ordered
/// specific-before-generic: `millberry` before `mischkupfer`, the
/// `neu-schrott` row before the `zink` arm (`verzinkte` contains it),
/// `kühler`/`batterie`/`altauto` (no catalog material) before every
/// family arm, and the `motoren+alu` row before the `elektromotoren`
/// arm. Anything unlisted is skipped. The variant keeps the trader's
/// own grade wording; `''` = standard grade.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Documented non-materials first: cooler composites, whole cars and
    // starter batteries have no catalog entry (extension = separate step).
    if l.contains("kühler") || l.contains("kuehler") {
        return None;
    }
    if l.contains("altauto") {
        return None;
    }
    if l.contains("batterie") {
        return None;
    }
    if l.contains("katalysator") || (l.contains("kat ") || l.starts_with("kat ")) {
        if l.contains("klein") {
            return Some(("katalysatoren", "klein"));
        } else if l.contains("normal") {
            return Some(("katalysatoren", "normal"));
        } else if l.contains("groß") || l.contains("gross") {
            return Some(("katalysatoren", "groß"));
        }
        return Some(("katalysatoren", ""));
    }
    // Copper family.
    if l.contains("millberry") || l.contains("geschälter kupferdraht") {
        return Some(("kupfer-millberry", ""));
    }
    if l.contains("erdkabel") && l.contains("alu") {
        return Some(("kabel-alu", "Erdkabel"));
    }
    if l.contains("erdkabel") || l.contains("seekabel") {
        return Some(("kabel-kupfer", "Erdkabel/Seekabel"));
    }
    if l.contains("alu") && (l.contains("kabel") || l.contains("schälkabel")) {
        return Some(("kabel-alu", ""));
    }
    if l.contains("kabel") || l.contains("schälkabel") {
        if l.contains("ohne stecker") {
            return Some(("kabel-kupfer", ""));
        } else if l.contains("stecker") {
            return Some(("kabel-kupfer", "mit Steckeranteilen"));
        } else if l.contains("schälkabel") || l.contains("gummikabel") {
            return Some(("kabel-kupfer", "Schälkabel"));
        }
        return Some(("kabel-kupfer", ""));
    }
    if l.contains("mischkupfer") {
        return Some(("kupfer-gemischt", ""));
    }
    if l.contains("kupfer-neu") {
        return Some(("kupfer-gemischt", "Neu"));
    }
    if l.contains("kupfer") && l.contains("iso") {
        return Some(("kupfer-gemischt", "ISO"));
    }
    if l.contains("kupfer") && l.contains("anhaftung") {
        return Some(("kupfer-gemischt", "mit Anhaftungen"));
    }
    // Messing / Rotguss.
    if l.contains("messing") {
        if l.contains("hülse") || l.contains("huelse") {
            return Some(("messing", "Hülsen"));
        } else if l.contains("späne") || l.contains("spaene") {
            return Some(("messing", "Späne"));
        }
        return Some(("messing", ""));
    }
    if l.contains("rotgu") {
        return Some(("bronze-rotguss", ""));
    }
    // Edelstahl (VA 2 = V2A).
    if l.contains("va 2") || l.contains("rostfrei") {
        if l.contains("späne") || l.contains("spaene") {
            return Some(("edelstahl-v2a", "Späne"));
        } else if l.contains("schredder") {
            return Some(("edelstahl-v2a", "Schredder"));
        }
        return Some(("edelstahl-v2a", ""));
    }
    // E-Motoren (iron-quoted motor rows reach the steel arms below, so
    // the alu-gearbox row must come before this arm; complete car
    // engines/gearboxes contain "getriebe" and fall through to steel).
    if l.contains("motor") && l.contains("alu") {
        return Some(("aluminium-gemischt", "Getriebe"));
    }
    if (l.contains("motor") || l.contains("e-motor")) && !l.contains("getriebe") {
        if l.contains("sauber") {
            return Some(("elektromotoren", "sauber"));
        } else if l.contains("anhaftung") || l.contains("kunststoff") {
            return Some(("elektromotoren", "mit Anhaftungen"));
        }
        return Some(("elektromotoren", "sauber"));
    }
    // Aluminium family.
    if l.contains("felge") {
        return Some(("aluminium-guss", "Felgen"));
    }
    if l.contains("profil") {
        if l.contains("iso") && l.contains("alt") {
            return Some(("aluminium-profile", "ISO Alt"));
        } else if l.contains("iso") {
            return Some(("aluminium-profile", "ISO Neu"));
        } else if l.contains("bunt") || l.contains("farbig") {
            return Some(("aluminium-profile", "Bunt"));
        }
        return Some(("aluminium-profile", "Neu"));
    }
    if l.contains("alu") {
        if l.contains("geschirr") || l.contains("guß") || l.contains("guss") {
            return Some(("aluminium-gemischt", "Geschirr u. Guß"));
        } else if l.contains("neu") {
            return Some(("aluminium-gemischt", "Neu"));
        } else if l.contains("schredder") || l.contains("anhaftung") {
            return Some(("aluminium-gemischt", "Schredder"));
        } else if l.contains("späne") || l.contains("spaene") {
            return Some(("aluminium-gemischt", "Späne"));
        } else if l.contains("offset") {
            return Some(("aluminium-gemischt", "Offset"));
        }
        return Some(("aluminium-gemischt", ""));
    }
    if l.contains("offset") {
        return Some(("aluminium-gemischt", "Offset"));
    }
    // Lead / zinc / tin. The `neu-schrott` row must come first:
    // `verzinkte` contains the `zink` token but is steel.
    if l.contains("neu-schrott") || l.contains("neuschrott") {
        return Some(("stahlschrott-sorte-1", "Neu-Schrott"));
    }
    if l.contains("auswucht") {
        return Some(("blei-auswucht", "Auswuchtblei"));
    }
    if l.contains("blei") {
        return Some(("blei", ""));
    }
    if l.contains("zink") {
        return Some(("zink", ""));
    }
    if l.contains("zinn") {
        return Some(("zinn", ""));
    }
    // Iron & steel (per-tonne rows).
    if l.contains("mischschrott") || l.contains("leicht") {
        return Some(("mischschrott", ""));
    }
    if l.contains("kernschrott") || l.contains("schwer") {
        return Some(("stahlschrott-scheren", "Kernschrott"));
    }
    if l.contains("eisen") && (l.contains("guß") || l.contains("guss")) {
        return Some(("eisenschrott-gussbruch", ""));
    }
    if l.contains("blech 1") {
        return Some(("mischschrott", "Blech 1"));
    }
    if l.contains("blech 2") {
        return Some(("mischschrott", "Blech 2"));
    }
    if l.contains("späne") || l.contains("spaene") {
        return Some(("mischschrott", "Späne"));
    }
    if l.contains("getriebe") {
        return Some(("mischschrott", "Motoren u. Getriebe"));
    }
    if l.contains("stahl") || l.contains("schrott") {
        return Some(("mischschrott", "Stahl"));
    }
    None
}

/// Bespoke contact extraction for THIS impressum only: the `<h1>`
/// `Impressum` anchors the page, the first `<p>` naming `Henken GmbH`
/// holds firm + street + PLZ city as `<br>` lines, and the `<h2>`
/// `Kontakt` paragraph holds `Telefon:`/`E-Mail:` lines (mailto-href
/// preferred when present). Missing anchors mean the page changed
/// shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let h2 = Selector::parse("h2").expect("valid selector");
    let para = Selector::parse("p").expect("valid selector");
    let link = Selector::parse("a").expect("valid selector");
    let anchor = doc.select(&h1).find(|h| {
        h.text()
            .collect::<String>()
            .trim()
            .eq_ignore_ascii_case("Impressum")
    });
    let Some(_) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Anker fehlt".to_owned(),
        });
    };
    // Address lines: first <p> after the anchor naming Henken GmbH.
    // `<br>`-split via inner HTML (scraper text() would glue lines).
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    let addr_p = doc.select(&para).find(|p| {
        p.text()
            .collect::<String>()
            .contains("Henken GmbH")
    });
    if let Some(p) = addr_p {
        for part in p.inner_html().split("<br") {
            // Drop tag residue first (`<br />` leaves `/>` behind).
            let t = strip_tags(part);
            if t.is_empty() {
                continue;
            }
            let low = t.to_lowercase();
            if low.contains("straß") || low.contains("strass") || low.contains("weg") {
                street = t.clone();
                continue;
            }
            let mut it = t.split_whitespace();
            if let (Some(pc), Some(rest)) = (it.next(), it.next()) {
                if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    city = std::iter::once(rest)
                        .chain(it)
                        .collect::<Vec<_>>()
                        .join(" ");
                }
            }
        }
    }
    // Contact paragraph after the "Kontakt" heading.
    let kontakt = doc.select(&h2).find(|h| {
        h.text()
            .collect::<String>()
            .trim()
            .eq_ignore_ascii_case("Kontakt")
    });
    let Some(kontakt) = kontakt else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Anker fehlt".to_owned(),
        });
    };
    let mut phone = String::new();
    let mut email = String::new();
    let contact_p = kontakt
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p");
    if let Some(p) = contact_p {
        // Prefer a mailto href over the visible text.
        email = p
            .select(&link)
            .filter_map(|a| a.value().attr("href"))
            .find_map(|h| h.strip_prefix("mailto:"))
            .map(str::trim)
            .filter(|h| h.contains('@'))
            .unwrap_or("")
            .to_owned();
        for part in p.inner_html().split("<br") {
            let t = strip_tags(part);
            if t.is_empty() {
                continue;
            }
            if let Some(rest) = t.strip_prefix("Telefon:") {
                phone = rest.trim().to_owned();
            } else if let Some(rest) = t.strip_prefix("E-Mail:") {
                if email.is_empty() {
                    email = rest.trim().to_owned();
                }
            } else if t.contains('@') && email.is_empty() {
                email = t;
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
    // Page date: "Unsere aktuellen Schrottpreise" + "Stand: 28.09.2026".
    let mut published_at = None;
    if let Some(pos) = html.find("Stand:") {
        let tail = &html[pos + "Stand:".len()..];
        let digits: String = tail
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == ',' || *c == '/')
            .collect();
        let parts: Vec<&str> = digits.split(['.', '/']).collect();
        if parts.len() == 3 {
            published_at = parse_de_date(parts[0], parts[1], parts[2]);
        }
    }
    let doc = Html::parse_document(html);
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tbody tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
    // Never trust page order: take the table carrying the price header,
    // not just the first <table> on the page.
    let table = doc.select(&table).find(|t| {
        t.select(&head).any(|h| {
            h.text()
                .collect::<String>()
                .to_lowercase()
                .contains("bezeichnung")
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
        let label = label.split_whitespace().collect::<Vec<_>>().join(" ");
        if label.is_empty() || label.len() > 120 {
            continue;
        }
        // Empty price cell: loud skip, never a silent zero.
        let Some(price) = parse_eur(&cells[1]) else {
            skipped.push(format!("{label} (kein Preis: {})", cells[1].trim()));
            continue;
        };
        // A "0.00" row is "no quote", not a free gift: loud skip.
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

/// Bespoke unit matcher for THIS table's Einheit column (live: `kg`,
/// `Tonne`, `Stück`). Only kg/t/Stk exist here — anything else skips
/// loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("stück") || lower.contains("stuck") || lower.contains("stk") {
        Some("EUR/Stk")
    } else if lower.contains("tonne")
        || lower
            .split(|c: char| !c.is_alphanumeric())
            .any(|t| t == "t")
    {
        Some("EUR/t")
    } else if lower.contains("kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    // Real excerpt of the live page (28.09.2026): TablePress shape with
    // header row, dot-decimal prices (`1.95 €`) and all three units.
    const FIXTURE: &str = "<p>Unsere aktuellen Schrottpreise</p><p>Stand: 28.09.2026</p>\
        <table id=\"tablepress-12\"><thead><tr><th>Bezeichnung</th><th>Preis</th><th></th></tr></thead>\
        <tbody>\
        <tr><td>Alu - Felgen</td><td>1.95 €</td><td>kg</td></tr>\
        <tr><td>Alu - Geschirr u. Alu Guß</td><td>1.20 €</td><td>kg</td></tr>\
        <tr><td>Alu - Kabel   oder  Alu-Schälkabel</td><td>0.75 €</td><td>kg</td></tr>\
        <tr><td>Kupfer-Millberry  / geschälter Kupferdraht</td><td>8.50 €</td><td>kg</td></tr>\
        <tr><td>Kupfer-Kabel     (Elektriker Kabel ohne Stecker)</td><td>2.80 €</td><td>kg</td></tr>\
        <tr><td>Erdkabel mit Alu</td><td>0.45 €</td><td>kg</td></tr>\
        <tr><td>Messing                      (Badezimmeramaturen, Wasserhahn)</td><td>4.20 €</td><td>kg</td></tr>\
        <tr><td>Rotguß</td><td>5.40 €</td><td>kg</td></tr>\
        <tr><td>VA 2  / (Rostfreier-Edelstahl)</td><td>0.80 €</td><td>kg</td></tr>\
        <tr><td>Elektromotoren sauber frei von Anhaftungen</td><td>0.80 €</td><td>kg</td></tr>\
        <tr><td>Schrott - leicht  (Mischschrott)</td><td>180.00 €</td><td>Tonne</td></tr>\
        <tr><td>Schrott - schwer - Kernschrott     (Stärke von 0,6 cm)</td><td>205.00 €</td><td>Tonne</td></tr>\
        <tr><td>Eisen Guß sauber   getrennt geliefert sortiert</td><td>210.00 €</td><td>Tonne</td></tr>\
        <tr><td>Neu-Schrott u. verzinkte Bleche</td><td>215.00 €</td><td>Tonne</td></tr>\
        <tr><td>Kat klein</td><td>25.00 €</td><td>Stück</td></tr>\
        <tr><td>Kupfer-Alu-Kühler</td><td>3.00 €</td><td>kg</td></tr>\
        <tr><td>Altautos mit Brief (bei Anlieferung) / Busse</td><td>100.00 €</td><td>Tonne</td></tr>\
        <tr><td>Batterien (Blei  =Pb )</td><td>300.00 €</td><td>Tonne</td></tr>\
        </tbody></table>";

    #[test]
    fn table_and_date_parse() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-28T00:00:00+00:00"));
        assert_eq!(rows.len(), 18);
        assert!(skips.is_empty());
        assert_eq!(rows[0], ("Alu - Felgen".to_owned(), 1.95, "EUR/kg"));
        assert_eq!(
            rows[3],
            (
                "Kupfer-Millberry / geschälter Kupferdraht".to_owned(),
                8.5,
                "EUR/kg"
            )
        );
        assert_eq!(
            rows[10],
            (
                "Schrott - leicht (Mischschrott)".to_owned(),
                180.0,
                "EUR/t"
            )
        );
        assert_eq!(rows[14], ("Kat klein".to_owned(), 25.0, "EUR/Stk"));
    }

    #[test]
    fn empty_and_zero_price_cells_skip_loudly() {
        let html = FIXTURE
            .replacen("<td>1.95 €</td>", "<td></td>", 1)
            .replacen("<td>8.50 €</td>", "<td>0.00 €</td>", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 16);
        assert_eq!(skips.len(), 2);
        assert!(skips[0].contains("Felgen") && skips[0].contains("kein Preis"));
        assert!(skips[1].contains("Millberry") && skips[1].contains("0,00"));
    }

    #[test]
    fn wrong_table_and_unit_are_rejected_loudly() {
        // A layout table before the price table must not win.
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (_, rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 18);
        // Unknown unit: skipped loudly, valid rows survive.
        let html = FIXTURE.replacen("<td>kg</td>", "<td>pro Sack</td>", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 17);
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Felgen") && skips[0].contains("Einheit"));
        // Every row unparseable: loud error, not silent success.
        let html = FIXTURE.replace("<td>kg</td>", "<td>pro Sack</td>");
        let html = html
            .replace("<td>Tonne</td>", "<td>pro Sack</td>")
            .replace("<td>Stück</td>", "<td>pro Sack</td>");
        let err = parse(&html).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn units_cover_live_column() {
        assert_eq!(unit_of("kg"), Some("EUR/kg"));
        assert_eq!(unit_of("Tonne"), Some("EUR/t"));
        assert_eq!(unit_of("Stück"), Some("EUR/Stk"));
        assert_eq!(unit_of("pro Sack"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        // Real fragment shape: h1-anchored address <p>, h2 Kontakt <p>.
        let imp = "<h1>Impressum</h1>\
            <p>H. Henken GmbH<br />Jadestra&szlig;e 8<br />26169 Friesoythe</p>\
            <p>Handelsregister: HRB150697<br />Registergericht: Amtsgericht Oldenburg</p>\
            <h2>Kontakt</h2><p>Telefon: 04491 21 91<br />E-Mail: auftrag@henkengmbh.de</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Jadestraße 8");
        assert_eq!(info.postcode, "26169");
        assert_eq!(info.city, "Friesoythe");
        assert_eq!(info.phone, "04491 21 91");
        assert_eq!(info.email, "auftrag@henkengmbh.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(extract_info("<h1>Impressum</h1><p>H. Henken GmbH</p>").is_err());
    }

    #[test]
    fn mapping_covers_live_table() {
        // Every live label lands deliberately — or on None with a reason.
        assert_eq!(grade_for("Alu - Felgen"), Some(("aluminium-guss", "Felgen")));
        assert_eq!(
            grade_for("Alu - Geschirr u. Alu Guß"),
            Some(("aluminium-gemischt", "Geschirr u. Guß"))
        );
        assert_eq!(
            grade_for("Alu - Kabel   oder  Alu-Schälkabel"),
            Some(("kabel-alu", ""))
        );
        assert_eq!(
            grade_for("Alu - Neu"),
            Some(("aluminium-gemischt", "Neu"))
        );
        assert_eq!(
            grade_for("Alu - Schredder / Alu mit Anhaftungen"),
            Some(("aluminium-gemischt", "Schredder"))
        );
        assert_eq!(
            grade_for("Alu - Späne"),
            Some(("aluminium-gemischt", "Späne"))
        );
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(
            grade_for("Blei - Auswuchtblei    (Gewichte von Autoreifen, Gardienenblei)"),
            Some(("blei-auswucht", "Auswuchtblei"))
        );
        assert_eq!(
            grade_for("Elektromotoren sauber frei von Anhaftungen"),
            Some(("elektromotoren", "sauber"))
        );
        assert_eq!(
            grade_for("E-Motoren-mit leichten Eisenanhaftungen, Kunststoff E-Motoren"),
            Some(("elektromotoren", "mit Anhaftungen"))
        );
        // Verbundkühler: kein Katalogmaterial.
        assert_eq!(grade_for("Kupfer-Alu-Kühler"), None);
        assert_eq!(grade_for("Kupfer-Messing-Kühler (Kupfer)"), None);
        assert_eq!(
            grade_for("Kupfer - mit Anhaftungen"),
            Some(("kupfer-gemischt", "mit Anhaftungen"))
        );
        assert_eq!(
            grade_for("Kupfer-ISO         (Isolierung, Schaumstoff, Gummi am Kupfer)"),
            Some(("kupfer-gemischt", "ISO"))
        );
        assert_eq!(
            grade_for("Kupfer-Kabel     (Elektriker Kabel ohne Stecker)"),
            Some(("kabel-kupfer", ""))
        );
        assert_eq!(
            grade_for("Erdkabel mit Alu"),
            Some(("kabel-alu", "Erdkabel"))
        );
        assert_eq!(
            grade_for("Erdkabel Mit Kupfer oder Seekabel"),
            Some(("kabel-kupfer", "Erdkabel/Seekabel"))
        );
        assert_eq!(
            grade_for("Kupfer-Kabel  mit Steckeranteilen"),
            Some(("kabel-kupfer", "mit Steckeranteilen"))
        );
        assert_eq!(
            grade_for("Kupfer-Millberry  / geschälter Kupferdraht"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Kupfer-Mischkupfer"),
            Some(("kupfer-gemischt", ""))
        );
        assert_eq!(
            grade_for("Kupfer-Neu"),
            Some(("kupfer-gemischt", "Neu"))
        );
        assert_eq!(
            grade_for("Kupfer-Schälkabel    (Gummikabel)"),
            Some(("kabel-kupfer", "Schälkabel"))
        );
        assert_eq!(
            grade_for("Messing                      (Badezimmeramaturen, Wasserhahn)"),
            Some(("messing", ""))
        );
        assert_eq!(
            grade_for("Messinghülsen"),
            Some(("messing", "Hülsen"))
        );
        assert_eq!(
            grade_for("Messingspäne"),
            Some(("messing", "Späne"))
        );
        assert_eq!(grade_for("Rotguß"), Some(("bronze-rotguss", "")));
        assert_eq!(
            grade_for("Offset (Alu)"),
            Some(("aluminium-gemischt", "Offset"))
        );
        assert_eq!(
            grade_for("Profile - ISO ALT (Alu) vom Fensterbauer"),
            Some(("aluminium-profile", "ISO Alt"))
        );
        assert_eq!(
            grade_for("Profile - ISO Neu  (Abfälle)  gute"),
            Some(("aluminium-profile", "ISO Neu"))
        );
        assert_eq!(
            grade_for("Profile - Neu  (nur 1 m Lang, nur glänzend)"),
            Some(("aluminium-profile", "Neu"))
        );
        assert_eq!(
            grade_for("Profile - Bunt (farbige Fensterrahmen)"),
            Some(("aluminium-profile", "Bunt"))
        );
        assert_eq!(
            grade_for("VA 2  / (Rostfreier-Edelstahl)"),
            Some(("edelstahl-v2a", ""))
        );
        assert_eq!(
            grade_for("VA 2 /   Späne"),
            Some(("edelstahl-v2a", "Späne"))
        );
        assert_eq!(
            grade_for("VA 2 / Schredder"),
            Some(("edelstahl-v2a", "Schredder"))
        );
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Zinn"), Some(("zinn", "")));
        assert_eq!(
            grade_for("Schrott - Späne"),
            Some(("mischschrott", "Späne"))
        );
        assert_eq!(
            grade_for("Blech 1 - Waschmaschine, Trockner, Fahrrad 20-30 % Müllanteil"),
            Some(("mischschrott", "Blech 1"))
        );
        assert_eq!(
            grade_for("Blech 2 - Kühltheken oh. Gefahrstoffe, Draht mit Anhaftung - 30-50 % Müllanteil"),
            Some(("mischschrott", "Blech 2"))
        );
        assert_eq!(
            grade_for("Schrott - leicht  (Mischschrott)"),
            Some(("mischschrott", ""))
        );
        assert_eq!(
            grade_for("Schrott - schwer - Kernschrott     (Stärke von 0,6 cm)"),
            Some(("stahlschrott-scheren", "Kernschrott"))
        );
        assert_eq!(
            grade_for("Stahl (Stärke von 0,6 cm)"),
            Some(("mischschrott", "Stahl"))
        );
        assert_eq!(
            grade_for("Motoren u. Getriebe"),
            Some(("mischschrott", "Motoren u. Getriebe"))
        );
        assert_eq!(
            grade_for("nur Getriebe o. Motoren Alu"),
            Some(("aluminium-gemischt", "Getriebe"))
        );
        assert_eq!(
            grade_for("Eisen Guß sauber   getrennt geliefert sortiert"),
            Some(("eisenschrott-gussbruch", ""))
        );
        assert_eq!(
            grade_for("Neu-Schrott u. verzinkte Bleche"),
            Some(("stahlschrott-sorte-1", "Neu-Schrott"))
        );
        // Altautos und Batterien: kein Katalogmaterial.
        assert_eq!(
            grade_for("Altautos mit Brief (bei Anlieferung) / Busse"),
            None
        );
        assert_eq!(
            grade_for("Altautos mit Brief (bei Anlieferung) / Busse ohne Motor"),
            None
        );
        assert_eq!(
            grade_for("Altauto ohne Papiere (bei Anlieferung)"),
            None
        );
        assert_eq!(
            grade_for("Altauto ohne Papiere (bei Anlieferung) ohne Motor"),
            None
        );
        assert_eq!(grade_for("Batterien (Blei  =Pb )"), None);
        // Katalysatoren je Stück.
        assert_eq!(
            grade_for("Kat klein"),
            Some(("katalysatoren", "klein"))
        );
        assert_eq!(
            grade_for("Kat normal (magnetisch)   nicht klappern u. keine Runden Netze)."),
            Some(("katalysatoren", "normal"))
        );
        assert_eq!(
            grade_for("Kat große (nur BMW u. Mercedes)"),
            Some(("katalysatoren", "groß"))
        );
    }
}
