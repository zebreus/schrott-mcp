//! Hein Schrotthandel (Schöneiche): exact daily prices in a single
//! `table.custom-datatable` ("Material" / "Preis (pro Tonne)" — all 130
//! rows quote EUR/t, market-plausibilized: Millberry 11.400 €/t ≈ 11,40
//! €/kg, Mischschrott 120 €/t). No page date ("täglich überprüft" prose
//! only), so `published_at` stays `None`. The Schrottpreisrechner is JS
//! (`select`/`input`, no table) and is never touched — only the static
//! table is parsed. "Katalysator Keramik Stückpreis" contradicts the
//! tonne header and skips loudly; the ~30 Elektronik rows without a
//! catalog material (CPUs, RAM, ICs, Handys, …) skip loudly — see the
//! proposals below. Zero quotes ("Aluminium Kabel dünn", "Kupferkabel
//! (Telefon)", "Katalysator Aftermarket") mean "no current price" and
//! skip loudly — never an exact 0.0 observation.
//!
//! Proposals (new catalog materials, never crammed):
//! - `elektronik-cpu` (9 rows: Keramik/Kunststoff/Slot/286-486 …)
//! - `elektronik-ram` (3 rows: Goldkante, Alukappe, Silberkante)
//! - `elektronik-ic` ("IC/Eprom", "IC/Eprom Gold")
//! - `handy-schrott` ("Handy Mix", "Handy Kinder/Baustelle")
//! - `notebook-schrott` (2 Laptop rows), `festplatten-schrott`,
//!   `netzteile-schrott` (2), `laufwerke-schrott`
//! - `kuehler-verbund` ("Aluminium Kupfer Kühler mit/ohne Fe",
//!   "Kupfer-Fe Kühler", "Kupfer-VA Kühler")
//! - `lambdasonden`, `zuendkerzen`
//! - "Hartmetall-HSS" names two materials in one label (ambiguous);
//!   "Messing Rotguss stückig" is a two-grade maximum (ambiguous).

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "be-sitz-schoneiche-hein-schrotthandel";
/// Bespoke, live-verified impressum URL (the site's own footer link).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.hein-schrotthandel.de/impressum";

pub const URL: &str = "https://www.hein-schrotthandel.de/schrottpreise/";

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

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific-before-generic per family: "Millberry" wins over
/// bare "Kupfer", "Ausbauprofile" over "Profile", "Hartzink" over "Zink",
/// "Lötzinn" over "Zinn". Family branches never catch-all: an Alu label
/// matching no Alu arm (e.g. "Elektro-Motor/Trafo mit
/// Aluminiumwicklung") falls through to the motor arms below.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Verbundkühler (Kupfer/Alu mit Fe-Anteil) have no catalog material.
    if l.contains("kühler") {
        return None;
    }
    // "Messing Rotguss stückig" is a two-grade maximum — ambiguous.
    if l.contains("rotguss") {
        return None;
    }
    // "Hartmetall-HSS" names two materials in one label — ambiguous.
    if l.contains("hartmetall") && l.contains("hss") {
        return None;
    }
    if l.contains("elektronik") {
        if l.contains("leiterplatten") {
            if l.contains("1a") {
                return Some(("platinen", "Sorte 1a"));
            }
            if l.contains("1b") {
                return Some(("platinen", "Sorte 1b"));
            }
            if l.contains("2a") {
                return Some(("platinen", "Sorte 2a"));
            }
            if l.contains("2b") {
                return Some(("platinen", "Sorte 2b"));
            }
            if l.contains("sorte 3") {
                return Some(("platinen", "Sorte 3"));
            }
            if l.contains("laptop") {
                return Some(("platinen", "Laptop"));
            }
            if l.contains("einschübe") {
                return Some(("platinen", "Einschübe"));
            }
            if l.contains("ohne blende") {
                return Some(("platinen", "PCI ohne Blende"));
            }
            if l.contains("pci") {
                return Some(("platinen", "PCI mit Blende"));
            }
            return None;
        }
        if l.contains("platinen") {
            if l.contains("handy") {
                return Some(("platinen", "Handy"));
            }
            if l.contains("laufwerk") {
                return Some(("platinen", "Laufwerk"));
            }
            if l.contains("festplatten") {
                return Some(("platinen", "Festplatten Mix"));
            }
            return None;
        }
        if l.contains("motherboard") {
            return Some(("platinen", "Motherboards"));
        }
        if l.contains("rückwände") || l.contains("rueckwaende") {
            return Some(("platinen", "Server-Rückwände"));
        }
        // CPUs, RAM, ICs, Handys, Laptops, Festplatten, Netzteile,
        // Laufwerke: no catalog material (proposals in the module doc).
        return None;
    }
    if l.contains("katalysator") {
        if l.contains("aftermarket") {
            return Some(("katalysatoren", "Aftermarket"));
        }
        if l.contains("monolith") {
            return Some(("katalysatoren", "Monolith lose"));
        }
        // "Keramik Stückpreis" never reaches here (parse-level skip:
        // per-piece label under a per-tonne header).
        return None;
    }
    if l.contains("lambdasonde") || l.contains("zündkerzen") {
        return None;
    }
    if l.contains("wendeschneidplatten") {
        return Some(("hartmetall", "Wendeschneidplatten"));
    }
    if l.contains("wolframkarbid") || l.contains("82%") {
        return Some(("hartmetall", "82% WC"));
    }
    if l.contains("hss") || l.contains("bohrer") {
        return Some(("hss-werkzeuge", "Bohrer mit Schaft"));
    }
    // Kupfer family (Alu-Kupfer-Kühler already returned above).
    if l.contains("kupfer") {
        if l.contains("millberry") {
            if l.contains("unter 1mm") || l.contains("unter 1 mm") {
                return Some(("kupfer-millberry", "unter 1mm"));
            }
            return Some(("kupfer-millberry", ""));
        }
        if l.contains("kabel") {
            if l.contains("15-20") {
                return Some(("kabel-kupfer", "15-20%"));
            }
            if l.contains("25-30") {
                return Some(("kabel-kupfer", "25-30%"));
            }
            if l.contains("38-40") {
                return Some(("kabel-kupfer", "38-40%"));
            }
            if l.contains("35%") {
                return Some(("kabel-kupfer", "35%"));
            }
            if l.contains("50%") {
                return Some(("kabel-kupfer", "50%"));
            }
            if l.contains("60%") {
                return Some(("kabel-kupfer", "60%"));
            }
            if l.contains("70%") {
                return Some(("kabel-kupfer", "70%"));
            }
            if l.contains("80%") {
                return Some(("kabel-kupfer", "80%"));
            }
            if l.contains("90%") {
                return Some(("kabel-kupfer", "90%"));
            }
            if l.contains("blei") {
                // Bleimantel-Kabel (auch Kupfer-Blei-Mischkabel) ist kein
                // Kupferkabel: 0,45 €/kg würde die Cu-Kabel-Reihe (3-5 €/kg)
                // korrumpieren. Eigenes Material seit Feedback #133.
                return Some(("kabel-blei", "Blei"));
            }
            if l.contains("stecker") {
                return Some(("kabel-mit-stecker", "mit Stecker"));
            }
            if l.contains("telefon") {
                return Some(("kabel-kupfer", "Telefon"));
            }
            return None;
        }
        if l.contains("raff") {
            return Some(("kupfer-gemischt", "Raff 95%"));
        }
        if l.contains("motorenwicklung") {
            if l.contains("90%") {
                return Some(("kupfer-gemischt", "90% Motorenwicklung"));
            }
            return Some(("kupfer-gemischt", "70% Motorenwicklung"));
        }
        if l.contains("abgebrannt") {
            return Some(("kupfer-gemischt", "abgebrannt 93%"));
        }
        if l.contains("blech") {
            return Some(("kupfer-gemischt", "Blech neu blank"));
        }
        if l.contains("leicht") {
            return Some(("kupfer-gemischt", "Leicht"));
        }
        if l.contains("oberleitung") {
            return Some(("kupfer-gemischt", "Oberleitungsdraht"));
        }
        if l.contains("leitschiene") {
            if l.contains("farbe") {
                return Some(("kupfer-gemischt", "Leitschiene mit Farbe"));
            }
            if l.contains("0,4") {
                return Some(("kupfer-gemischt", "Leitschiene blank kurz"));
            }
            return Some(("kupfer-gemischt", "Leitschiene blank"));
        }
        if l.contains("späne") || l.contains("spaene") {
            return Some(("kupfer-gemischt", "Späne"));
        }
        return None;
    }
    // Blei family: the Kupfer block above already claimed
    // "Kupfer-Blei-Kabel", and these arms have no catch-all, so nothing
    // else misroutes here.
    if l.contains("auswuchtblei") {
        return Some(("blei", "Auswuchtblei"));
    }
    if l.contains("kabelschälblei") {
        return Some(("blei", "Kabelschälblei"));
    }
    if l.contains("altblei") {
        return Some(("blei", ""));
    }
    // Alu family — no catch-all arm (see fn doc).
    if l.contains("alu") {
        if l.contains("kabel") {
            if l.contains("dick") {
                return Some(("kabel-alu", "dick schlitzfähig"));
            }
            return Some(("kabel-alu", "dünn"));
        }
        if l.contains("ausbauprofile") {
            return Some(("aluminium-profile", "Ausbau"));
        }
        if l.contains("profil") {
            if l.contains("lackiert") {
                if l.contains("über 50") {
                    return Some(("aluminium-profile", "lackiert >50cm"));
                }
                return Some(("aluminium-profile", "lackiert <50cm"));
            }
            if l.contains("über 50") {
                return Some(("aluminium-profile", "neu blank >50cm"));
            }
            return Some(("aluminium-profile", "neu blank <50cm"));
        }
        if l.contains("nummernschilder") {
            return Some(("aluminium-blech", "Nummernschilder"));
        }
        if l.contains("blech") {
            return Some(("aluminium-blech", "neu/blank"));
        }
        if l.contains("felgen") {
            if l.contains("unsauber") {
                return Some(("aluminium-guss", "Felgen unsauber"));
            }
            return Some(("aluminium-guss", "Felgen sauber"));
        }
        if l.contains("guss") || l.contains("guß") {
            if l.contains("50%") {
                return Some(("aluminium-guss", "50% Fe"));
            }
            if l.contains("10%") {
                return Some(("aluminium-guss", "10% Fe"));
            }
            return Some(("aluminium-guss", ""));
        }
        if l.contains("draht") {
            if l.contains("luftgeschwärzt") {
                return Some(("aluminium-gemischt", "Draht luftgeschwärzt"));
            }
            return Some(("aluminium-gemischt", "Draht blank"));
        }
        if l.contains("leitschiene") {
            return Some(("aluminium-gemischt", "Leitschiene blank"));
        }
        if l.contains("sorte 2") {
            return Some(("aluminium-gemischt", "Sorte 2"));
        }
        if l.contains("sorte 1") {
            return Some(("aluminium-gemischt", ""));
        }
        if l.contains("späne") || l.contains("spaene") {
            return Some(("aluminium-gemischt", "Späne"));
        }
    }
    // Eisen family.
    if l.contains("bremsscheiben") {
        return Some(("eisenschrott-gussbruch", "Bremsscheiben"));
    }
    if l.contains("gusseisen") || l.contains("gußeisen") {
        if l.contains("unzerkleinert") {
            return Some(("eisenschrott-gussbruch", "Maschine unzerkleinert"));
        }
        return Some(("eisenschrott-gussbruch", ""));
    }
    if l.contains("mischschrott") {
        return Some(("mischschrott", ""));
    }
    if l.contains("schreddervormaterial") {
        return Some(("stahlschrott-shredder", ""));
    }
    // Kabelbäume (harness, no Cu/Alu qualifier on the page).
    if l.contains("kabelbäume") || l.contains("kabelbaeume") {
        return Some(("kabel-kupfer", "Kabelbäume"));
    }
    // E-Motoren (reached by "Elektro-Motor/Trafo mit
    // Aluminiumwicklung" via Alu fall-through).
    if l.contains("motor") {
        if l.contains("getriebe") {
            return Some(("elektromotoren", "mit Getriebe"));
        }
        if l.contains("alu") {
            return Some(("elektromotoren", "Aluwicklung"));
        }
        if l.contains("300") {
            return Some(("elektromotoren", "ab 300 kg"));
        }
        return Some(("elektromotoren", ""));
    }
    // Edelstahl.
    if l.contains("v4a") {
        if l.contains("übermaß") || l.contains("uebermass") {
            return Some(("edelstahl-v4a", "Übermaß"));
        }
        if l.contains("schredder") {
            return Some(("edelstahl-v4a", "Schredder"));
        }
        return Some(("edelstahl-v4a", ""));
    }
    if l.contains("v2a") {
        if l.contains("übermaß") || l.contains("uebermass") {
            return Some(("edelstahl-v2a", "Übermaß"));
        }
        if l.contains("mit fett") {
            return Some(("edelstahl-v2a", "Schredder mit Fett"));
        }
        if l.contains("schredder") {
            return Some(("edelstahl-v2a", "Schredder fettfrei"));
        }
        if l.contains("späne") || l.contains("spaene") {
            return Some(("edelstahl-v2a", "Späne"));
        }
        return Some(("edelstahl-v2a", ""));
    }
    // Zink ("Hartzink" before bare "Zink").
    if l.contains("hartzink") {
        return Some(("zink", "Hartzink"));
    }
    if l.contains("zink") {
        if l.contains("(alt)") {
            return Some(("zink", "alt"));
        }
        if l.contains("(neu)") {
            return Some(("zink", "neu"));
        }
        return None;
    }
    // Zinn ("Lötzinn" before bare "Zinn").
    if l.contains("lötzinn") || l.contains("loetzinn") {
        if l.contains("30sn") {
            return Some(("zinn", "30Sn/70Pb"));
        }
        if l.contains("50sn") {
            return Some(("zinn", "50Sn/50Pb"));
        }
        if l.contains("60sn") {
            return Some(("zinn", "60Sn/40Pb"));
        }
        return None;
    }
    if l.contains("zinn") {
        if l.contains("80") {
            return Some(("zinn", "80-85%"));
        }
        if l.contains("92") || l.contains("95") {
            return Some(("zinn", "92-95%"));
        }
        if l.contains("99") {
            return Some(("zinn", "99%"));
        }
        return None;
    }
    // Messing (Rotguss already returned above).
    if l.contains("messing") {
        if l.contains("patronenhülsen") {
            return Some(("messing", "Patronenhülsen"));
        }
        if l.contains("wasseruhren") {
            return Some(("messing", "Wasseruhren"));
        }
        if l.contains("anhaftung") {
            return Some(("messing", "mit Anhaftung"));
        }
        if l.contains("späne") || l.contains("spaene") {
            return Some(("messing", "Späne"));
        }
        if l.contains("schwer") {
            return Some(("messing", ""));
        }
        return None;
    }
    None
}

/// Bespoke contact extraction for THIS impressum only: the firm `<p>`
/// holding "Geschäftsführerin : …" plus "PLZ Ort, Straße" on its `<br>`
/// line, the "Telefon:" `<p>` with one number per `<br>` line, and the
/// "E-Mail:" `<p>` whose address is a plain-text link to /kontakt/.
/// Missing anchors mean the page changed shape → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p_sel = Selector::parse("p").expect("valid selector");
    let paras: Vec<ElementRef> = doc.select(&p_sel).collect();
    let firm = paras
        .iter()
        .find(|p| p.text().collect::<String>().contains("Geschäftsführerin"))
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Geschäftsführer-Block fehlt".to_owned(),
        })?;
    // "15566 Schöneiche bei Berlin, Werner-von-Siemens-Str. 12": PLZ +
    // Ort left of the comma, street right of it.
    let lines: Vec<String> = firm
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for line in &lines {
        if let Some((left, right)) = line.split_once(',') {
            let toks: Vec<&str> = left.split_whitespace().collect();
            if let Some(pc) = toks.first() {
                if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = (*pc).to_owned();
                    city = toks.get(1).unwrap_or(&"").to_string();
                    street = right.trim().to_owned();
                    break;
                }
            }
        }
    }
    // Phone: first number line after the "Telefon:" head line.
    let mut phone = String::new();
    if let Some(p) = paras.iter().find(|p| {
        p.text()
            .collect::<String>()
            .trim_start()
            .starts_with("Telefon:")
    }) {
        let rows: Vec<String> = p
            .inner_html()
            .split("<br")
            .map(strip_fragment)
            .filter(|s| !s.is_empty())
            .collect();
        for row in rows.iter().skip(1) {
            if !row.is_empty() {
                phone = row.clone();
                break;
            }
        }
    }
    // E-Mail: the token with '@' in the "E-Mail:" paragraph.
    let mut email = String::new();
    if let Some(p) = paras
        .iter()
        .find(|p| p.text().collect::<String>().contains("E-Mail:"))
    {
        let text: String = p.text().collect();
        for tok in text.split_whitespace() {
            if tok.contains('@') {
                email = tok.trim_matches([',', ';']).to_owned();
                break;
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

/// Strip tags from a `<br>`-split fragment (discard everything up to the
/// first `>` so no `class="…"` rest parses as text).
fn strip_fragment(s: &str) -> String {
    let mut plain = String::new();
    let mut tag = false;
    for c in s.chars() {
        if c == '<' {
            tag = true;
        } else if c == '>' {
            tag = false;
        } else if !tag {
            plain.push(c);
        }
    }
    plain.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let table_sel = Selector::parse("table").expect("valid selector");
    let row_sel = Selector::parse("tbody tr").expect("valid selector");
    let cell_sel = Selector::parse("td").expect("valid selector");
    let head_sel = Selector::parse("th").expect("valid selector");
    // Never trust page order: take the table carrying the Material
    // header, not just the first <table> on the page.
    let table = doc.select(&table_sel).find(|t| {
        t.select(&head_sel).any(|h| {
            h.text()
                .collect::<String>()
                .to_lowercase()
                .contains("material")
        })
    });
    let Some(table) = table else {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    };
    // Unit basis guard: the price column must still quote per tonne. A
    // header change here would silently 1000x every price → loud error.
    let per_tonne = table.select(&head_sel).any(|h| {
        h.text()
            .collect::<String>()
            .to_lowercase()
            .contains("tonne")
    });
    if !per_tonne {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Einheitenkopf geändert (kein Tonnen-Preis)".to_owned(),
        });
    }
    // Page-global unit from the header ("Preis (pro Tonne)"):
    // Millberry 11.400 €/t ≈ 11,40 €/kg and Mischschrott 120 €/t are
    // market-plausible; record() normalizes into the catalog unit.
    const PAGE_UNIT: &str = "EUR/t";
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in table.select(&row_sel) {
        let cells: Vec<String> = tr.select(&cell_sel).map(|c| c.text().collect()).collect();
        if cells.len() < 2 {
            continue;
        }
        let label = cells[0].replace(['\u{a0}'], " ");
        let label = label.split_whitespace().collect::<Vec<_>>().join(" ");
        if label.is_empty() {
            continue;
        }
        // Per-piece label under a per-tonne header: quoting it as EUR/t
        // would be wrong → loud skip, never a silent default.
        if label.to_lowercase().contains("stückpreis") {
            skips.push(format!(
                "{label} (Stückpreis passt nicht zum Tonnen-Header)"
            ));
            continue;
        }
        let Some(price) = parse_eur(&cells[1]) else {
            skips.push(format!(
                "{label} (Preis unverständlich: {})",
                cells[1].trim()
            ));
            continue;
        };
        // A quoted 0,00 is "no current price" (same as the "(Telefon)"
        // rows), never an exact 0.0 observation: recording it would set
        // the current price to 0 and trip jump canaries forever.
        if price <= 0.0 {
            skips.push(format!("{label} (Preis 0,00: kein Ankaufspreis)"));
            continue;
        }
        rows.push((label, price, PAGE_UNIT));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    // Real shape (live 27.09.2026): <table class="custom-datatable">
    // with <thead> "Material" / "Preis (pro Tonne)" and
    // <td class="price"> cells.
    const FIXTURE: &str = "<h2>Aktuelle Schrottpreise von Hein Schrotthandel GmbH</h2>\
        <table class=\"custom-datatable\"><thead><tr><th>Material</th>\
        <th class=\"price\">Preis (pro Tonne)</th></tr></thead><tbody>\
        <tr><td>Kupfer Millberry</td><td class=\"price\">11.400,00 €</td></tr>\
        <tr><td>Kupfer Millberry unter 1mm</td><td class=\"price\">11.250,00 €</td></tr>\
        <tr><td>Mischschrott</td><td class=\"price\">120,00 €</td></tr>\
        <tr><td>Aluminium Kabel dünn</td><td class=\"price\">0,00 €</td></tr>\
        <tr><td>Messing Rotguss stückig</td><td class=\"price\">9.270,00 €</td></tr>\
        <tr><td>Katalysator Keramik Stückpreis</td><td class=\"price\">30,00 €</td></tr>\
        <tr><td>Elektronik CPU Keramik Mix</td><td class=\"price\">120.000,00 €</td></tr>\
        <tr><td>Elektronik Leiterplatten Sorte 1a</td><td class=\"price\">4.950,00 €</td></tr>\
        </tbody></table>";

    #[test]
    fn table_parses_tonne_prices_and_skips_loudly() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 6, "{rows:?}");
        assert_eq!(rows[0], ("Kupfer Millberry".to_owned(), 11400.0, "EUR/t"));
        assert_eq!(rows[2], ("Mischschrott".to_owned(), 120.0, "EUR/t"));
        assert_eq!(
            rows[3],
            ("Messing Rotguss stückig".to_owned(), 9270.0, "EUR/t")
        );
        assert_eq!(
            rows[5],
            (
                "Elektronik Leiterplatten Sorte 1a".to_owned(),
                4950.0,
                "EUR/t"
            )
        );
        assert_eq!(skips.len(), 2, "{skips:?}");
        assert!(skips.iter().any(|s| s.contains("Stückpreis")), "{skips:?}");
        assert!(
            skips.iter().any(|s| s.contains("Tonnen-Header")),
            "{skips:?}"
        );
        assert!(
            skips
                .iter()
                .any(|s| s.contains("Aluminium Kabel dünn") && s.contains("0,00")),
            "{skips:?}"
        );
    }

    #[test]
    fn header_guards_fail_loudly() {
        // Wrong table (no Material header) must not win.
        let html = "<table><tr><td>Nav</td></tr></table>".to_owned() + FIXTURE;
        let (rows, _) = parse(&html).expect("finds the price table");
        assert_eq!(rows.len(), 6);
        // Unit basis changed: loud error, not silent 1000x prices.
        let html = FIXTURE.replace("Preis (pro Tonne)", "Preis (pro Kilo)");
        assert!(parse(&html).is_err());
        // Empty table: loud error, not silent success.
        let html = FIXTURE.replace(
            "<tr><td>Kupfer Millberry</td><td class=\"price\">11.400,00 €</td></tr>",
            "",
        );
        let (rows, _) = parse(&html).expect("other rows survive");
        assert_eq!(rows.len(), 5);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<div class=\"fusion-text fusion-text-2\">\
            <h1><strong>HEIN</strong><strong> Schrotthandel GmbH</strong></h1>\
            <p>Geschäftsführerin : Kathrin Hein, Andrea Schiroslawsky<br />\
            15566 Schöneiche bei Berlin, Werner-von-Siemens-Str. 12</p>\
            <p>Telefon:<br />030 64 38 77 10<br />0152 34 32 63 91</p>\
            <p>E-Mail: <a href=\"https://www.hein-schrotthandel.de/kontakt/\">\
            info@hein-schrotthandel.de</a></p></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Werner-von-Siemens-Str. 12");
        assert_eq!(info.postcode, "15566");
        assert_eq!(info.city, "Schöneiche");
        assert_eq!(info.phone, "030 64 38 77 10");
        assert_eq!(info.email, "info@hein-schrotthandel.de");
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }

    #[test]
    fn mapping_covers_every_arm() {
        let cases: &[(&str, Option<(&str, &str)>)] = &[
            ("Altblei", Some(("blei", ""))),
            ("Altblei (Auswuchtblei)", Some(("blei", "Auswuchtblei"))),
            (
                "Altblei (Kabelschälblei) mit Anhaftungen",
                Some(("blei", "Kabelschälblei")),
            ),
            (
                "Aluminium Ausbauprofile",
                Some(("aluminium-profile", "Ausbau")),
            ),
            (
                "Aluminium Blech neu/blank",
                Some(("aluminium-blech", "neu/blank")),
            ),
            (
                "Aluminium Felgen sauber",
                Some(("aluminium-guss", "Felgen sauber")),
            ),
            (
                "Aluminium Felgen unsauber",
                Some(("aluminium-guss", "Felgen unsauber")),
            ),
            (
                "Aluminium Kabel dick schlitzfähig",
                Some(("kabel-alu", "dick schlitzfähig")),
            ),
            ("Aluminium Kabel dünn", Some(("kabel-alu", "dünn"))),
            ("Aluminium Kupfer Kühler mit Fe", None),
            ("Aluminium Kupfer Kühler ohne Fe", None),
            (
                "Aluminium Leitschiene blank",
                Some(("aluminium-gemischt", "Leitschiene blank")),
            ),
            (
                "Aluminium Nummernschilder",
                Some(("aluminium-blech", "Nummernschilder")),
            ),
            (
                "Aluminium Profile Abfälle neu blank (über 50 cm)",
                Some(("aluminium-profile", "neu blank >50cm")),
            ),
            (
                "Aluminium Profile Abfälle neu blank (unter 50 cm)",
                Some(("aluminium-profile", "neu blank <50cm")),
            ),
            (
                "Aluminium Profile lackiert (über 50 cm)",
                Some(("aluminium-profile", "lackiert >50cm")),
            ),
            (
                "Aluminium Profile lackiert (unter 50 cm)",
                Some(("aluminium-profile", "lackiert <50cm")),
            ),
            ("Aluminium Sorte 1", Some(("aluminium-gemischt", ""))),
            (
                "Aluminium Sorte 2 max. 5% Anhaftung",
                Some(("aluminium-gemischt", "Sorte 2")),
            ),
            (
                "Aluminium Späne, trocken",
                Some(("aluminium-gemischt", "Späne")),
            ),
            (
                "Aluminiumdraht blank (fettfrei)",
                Some(("aluminium-gemischt", "Draht blank")),
            ),
            (
                "Aluminiumdraht luftgeschwärzt (fettfrei)",
                Some(("aluminium-gemischt", "Draht luftgeschwärzt")),
            ),
            ("Aluminiumguß ohne Eisen", Some(("aluminium-guss", ""))),
            (
                "Aluminiumguss 50% Fe ohne Öl",
                Some(("aluminium-guss", "50% Fe")),
            ),
            (
                "Aluminiumguss max. 10% Fe ohne Öl",
                Some(("aluminium-guss", "10% Fe")),
            ),
            (
                "Bremsscheiben ohne Lager PKW",
                Some(("eisenschrott-gussbruch", "Bremsscheiben")),
            ),
            ("Elektro-Motor", Some(("elektromotoren", ""))),
            (
                "Elektro-Motor ab 300 kg",
                Some(("elektromotoren", "ab 300 kg")),
            ),
            (
                "Elektro-Motor mit Getriebe",
                Some(("elektromotoren", "mit Getriebe")),
            ),
            (
                "Elektro-Motor/Trafo mit Aluminiumwicklung",
                Some(("elektromotoren", "Aluwicklung")),
            ),
            ("Elektronik CPU Keramik Mix", None),
            ("Elektronik CPU Kunststoff Kl. C Kupferkern", None),
            ("Elektronik Handy Mix ohne Akku", None),
            ("Elektronik RAM Arbeitsspeicher mit Goldkante", None),
            ("Elektronik Netzteile, intern mit Kabel", None),
            ("Elektronik Laptops ohne Display, ohne Akku", None),
            ("Elektronik IC/Eprom Gold", None),
            ("Elektronik Festplatten", None),
            ("Elektronik Laufwerke", None),
            (
                "Elektronik Festplatten Platinen Mix",
                Some(("platinen", "Festplatten Mix")),
            ),
            ("Elektronik Handy Platinen", Some(("platinen", "Handy"))),
            (
                "Elektronik Laufwerk Platinen",
                Some(("platinen", "Laufwerk")),
            ),
            (
                "Elektronik Leiterplatten Sorte 1a",
                Some(("platinen", "Sorte 1a")),
            ),
            (
                "Elektronik Leiterplatten Sorte 1b",
                Some(("platinen", "Sorte 1b")),
            ),
            (
                "Elektronik Leiterplatten Sorte 2a",
                Some(("platinen", "Sorte 2a")),
            ),
            (
                "Elektronik Leiterplatten Sorte 2b",
                Some(("platinen", "Sorte 2b")),
            ),
            (
                "Elektronik Leiterplatten, Sorte 3",
                Some(("platinen", "Sorte 3")),
            ),
            (
                "Elektronik Leiterplatten Laptop",
                Some(("platinen", "Laptop")),
            ),
            (
                "Elektronik Leiterplatten Einschübe",
                Some(("platinen", "Einschübe")),
            ),
            (
                "Elektronik Leiterplatten, PCI Steckkarten mit Blende",
                Some(("platinen", "PCI mit Blende")),
            ),
            (
                "Elektronik Leiterplatten, PCI Steckkarten ohne Blende und Kühler",
                // Kühler guard hits first: Verbund label, no catalog material.
                None,
            ),
            (
                "Elektronik Motherboards - Sockel 462 + 423",
                Some(("platinen", "Motherboards")),
            ),
            (
                "Elektronik Server-Rückwände",
                Some(("platinen", "Server-Rückwände")),
            ),
            (
                "Gußeisen Maschine unzerkleinert",
                Some(("eisenschrott-gussbruch", "Maschine unzerkleinert")),
            ),
            ("Gusseisen", Some(("eisenschrott-gussbruch", ""))),
            (
                "Hartmetall Wendeschneidplatten",
                Some(("hartmetall", "Wendeschneidplatten")),
            ),
            ("Hartmetall-HSS", None),
            (
                "Hartmetalle, mind. 82% Wolframkarbid (Sofortanalyse)",
                Some(("hartmetall", "82% WC")),
            ),
            (
                "HSS Bohrer mit Schaft",
                Some(("hss-werkzeuge", "Bohrer mit Schaft")),
            ),
            (
                "Kabelbäume ohne größere Stecker",
                Some(("kabel-kupfer", "Kabelbäume")),
            ),
            (
                "Katalysator Aftermarket oder geschweißt",
                Some(("katalysatoren", "Aftermarket")),
            ),
            (
                "Katalysator Keramik Monolith lose",
                Some(("katalysatoren", "Monolith lose")),
            ),
            (
                "Kupfer 70% (Motorenwicklungen)",
                Some(("kupfer-gemischt", "70% Motorenwicklung")),
            ),
            (
                "Kupfer 90% (Motorenwicklungen)",
                Some(("kupfer-gemischt", "90% Motorenwicklung")),
            ),
            (
                "Kupfer abgebrannt 93%",
                Some(("kupfer-gemischt", "abgebrannt 93%")),
            ),
            (
                "Kupfer Blech neu blank",
                Some(("kupfer-gemischt", "Blech neu blank")),
            ),
            ("Kupfer Leicht", Some(("kupfer-gemischt", "Leicht"))),
            ("Kupfer Millberry", Some(("kupfer-millberry", ""))),
            (
                "Kupfer Millberry unter 1mm",
                Some(("kupfer-millberry", "unter 1mm")),
            ),
            (
                "Kupfer Oberleitungsdraht",
                Some(("kupfer-gemischt", "Oberleitungsdraht")),
            ),
            ("Kupfer Raff 95%", Some(("kupfer-gemischt", "Raff 95%"))),
            (
                "Kupfer Späne sauber, trocken",
                Some(("kupfer-gemischt", "Späne")),
            ),
            (
                "Kupfer-Blei-Kabel ohne Teer (fettfrei)",
                Some(("kabel-blei", "Blei")),
            ),
            ("Kupfer-Fe Kühler", None),
            (
                "Kupfer-Leitschiene blank (ab 5 mm Materialstärke)",
                Some(("kupfer-gemischt", "Leitschiene blank")),
            ),
            (
                "Kupfer-Leitschiene blank bis 0,4 m (ab 5 mm Materialstärke)",
                Some(("kupfer-gemischt", "Leitschiene blank kurz")),
            ),
            (
                "Kupfer-Leitschiene mit Farbe",
                Some(("kupfer-gemischt", "Leitschiene mit Farbe")),
            ),
            ("Kupfer-VA Kühler", None),
            (
                "Kupferkabel (Telefon,starr,dünn)",
                Some(("kabel-kupfer", "Telefon")),
            ),
            ("Kupferkabel 15-20%", Some(("kabel-kupfer", "15-20%"))),
            ("Kupferkabel 25-30%", Some(("kabel-kupfer", "25-30%"))),
            ("Kupferkabel 35%", Some(("kabel-kupfer", "35%"))),
            ("Kupferkabel 38-40%", Some(("kabel-kupfer", "38-40%"))),
            (
                "Kupferkabel mit Stecker",
                Some(("kabel-mit-stecker", "mit Stecker")),
            ),
            ("Kupferkabel unverzinnt 50%", Some(("kabel-kupfer", "50%"))),
            ("Kupferkabel unverzinnt 60%", Some(("kabel-kupfer", "60%"))),
            ("Kupferkabel unverzinnt 70%", Some(("kabel-kupfer", "70%"))),
            ("Kupferkabel unverzinnt 80%", Some(("kabel-kupfer", "80%"))),
            ("Kupferkabel unverzinnt 90%", Some(("kabel-kupfer", "90%"))),
            ("Lambdasonde mit Kabel", None),
            ("Messing mit Anhaftung", Some(("messing", "mit Anhaftung"))),
            (
                "Messing Patronenhülsen",
                Some(("messing", "Patronenhülsen")),
            ),
            ("Messing Rotguss stückig", None),
            ("Messing schwer", Some(("messing", ""))),
            ("Messing Späne trocken gemischt", Some(("messing", "Späne"))),
            ("Messing Wasseruhren", Some(("messing", "Wasseruhren"))),
            ("Mischschrott", Some(("mischschrott", ""))),
            ("Schreddervormaterial", Some(("stahlschrott-shredder", ""))),
            (
                "V2A (Chrom-Nickel-Stahl) bis 1,5m",
                Some(("edelstahl-v2a", "")),
            ),
            (
                "V2A (Chrom-Nickel-Stahl) Übermaß",
                Some(("edelstahl-v2a", "Übermaß")),
            ),
            (
                "V2A Schredder (Tische, große Spülen etc), ohne Dämmung, fettfrei",
                Some(("edelstahl-v2a", "Schredder fettfrei")),
            ),
            (
                "V2A Schredder ohne Dämmung mit Fett",
                Some(("edelstahl-v2a", "Schredder mit Fett")),
            ),
            ("V2A Späne trocken", Some(("edelstahl-v2a", "Späne"))),
            (
                "V4A (Chrom-Nickel-Molybdän-Stahl) bis 1,5m",
                Some(("edelstahl-v4a", "")),
            ),
            (
                "V4A (Chrom-Nickel-Molybdän-Stahl) Übermaß",
                Some(("edelstahl-v4a", "Übermaß")),
            ),
            (
                "V4A Schredder (großstückig über 1,5 m), ohne Dämmung, fettfrei",
                Some(("edelstahl-v4a", "Schredder")),
            ),
            ("Zink (alt)", Some(("zink", "alt"))),
            ("Zink (neu)", Some(("zink", "neu"))),
            ("Zinkguss Hartzink ohne Fe", Some(("zink", "Hartzink"))),
            ("Zinn 80-85 % (Sofortanalyse)", Some(("zinn", "80-85%"))),
            ("Zinn 92%-95% (Sofortanalyse)", Some(("zinn", "92-95%"))),
            ("Zinn 99% (Sofortanalyse)", Some(("zinn", "99%"))),
            ("Zinn/Lötzinn 30Sn/70Pb", Some(("zinn", "30Sn/70Pb"))),
            ("Zinn/Lötzinn 50Sn/50Pb", Some(("zinn", "50Sn/50Pb"))),
            ("Zinn/Lötzinn 60Sn/40Pb", Some(("zinn", "60Sn/40Pb"))),
            ("Zündkerzen PKW", None),
        ];
        for (label, want) in cases {
            assert_eq!(&grade_for(label), want, "{label}");
        }
    }
}
