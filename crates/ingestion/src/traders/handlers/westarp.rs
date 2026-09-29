//! BERNHARD WESTARP GmbH & Co. KG (Aschaffenburg): gestaffelte
//! Einkaufspreise in 5 Tabellen (`Cu / Ms`, `Al / Zn / Pb / VA`, E-Schrott,
//! `VA / Fe` + Hartmetall/HSS-Block). Jede Datentabelle trägt ihren
//! Staffelkopf (`Menge:` + `bis/ab … kg`) im `<thead>`; die 6. Tabelle
//! (Öffnungszeiten, kein `Menge:`-Kopf) wird nie angerührt. Preise sind
//! bare Zahlen ohne Einheitsspalte (siehe `UNIT`-Doku); `Zuzahlung`-Zeilen
//! (Erdkabel-Muffen, Tresore) und Kühler-Mischprodukte werden laut
//! geskippt, ebenso CPUs/RAM/Geräte ohne Katalogmaterial.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "by-aschaffenburg-bernhard-westarp";
/// Bespoke, live-verified price URL (plain http, live 28.09.2026). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "http://www.westarp-kg.de/de/einkaufspreise";
/// Bespoke, live-verified impressum URL. A move fails the step loudly
/// (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "http://www.westarp-kg.de/de/impressum";

/// Seiten-Default, dokumentiert und markt-plausibilisiert: alle ~200
/// Staffelwerte liegen zwischen 0,03 und 66 — durchweg €/kg-plausibel
/// (Cu 8–12, Messing 4–9, Alu 0,1–2,3, Zinn 30–40, Keramik-CPU 50–66).
/// Die einzigen €/t-Zeilen der Seite (`150 € / t`, `100 € / to`) stehen in
/// `Zuzahlung`-Sonderzeilen und werden als Ganzes geskippt; Zellen mit
/// `/` oder `pro` skippen als explizit-fremd. Eine Tonne-als-Kilo-
/// Fehlbuchung (1000×) ist damit ausgeschlossen.
const UNIT: &str = "EUR/kg";

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
    for (label, tier, price) in rows {
        match grade_for(&label) {
            Some((material, grade)) => {
                let variant: &'static str = if grade.is_empty() {
                    tier
                } else {
                    // Bounded: one small alloc per live row (fairkat-Präzedenz).
                    Box::leak(format!("{grade} / {tier}").into_boxed_str())
                };
                prices.push(ScrapedPrice {
                    material,
                    variant,
                    price,
                    currency: "EUR",
                    unit: UNIT,
                    price_kind: "exact",
                    price_min: None,
                    price_max: None,
                    confidence: Some(1.0),
                    label,
                });
            }
            None => skipped_labels.push(format!("{} ({})", label, skip_reason(&label))),
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

/// Explicit label → (material, grade) mapping. Specific before generic:
/// `Kühler` (Mischprodukt ohne Katalogmaterial) steht vor allem, was
/// `kupfer`/`messing`/`alu` fängt; `Erdkabel` vor `Kabel`; `Filz` gibt es
/// nicht. Anything unlisted is skipped loudly at the call site.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Mischprodukte ohne Katalogmaterial (doering/db-Präzedenz).
    if l.contains("kühler") || l.contains("kuehler") {
        return None;
    }
    // Kupfer.
    if l.contains("millberry") {
        Some(("kupfer-millberry", "Millberry"))
    } else if l.contains("kerze") {
        Some(("kupfer-berry", "Kerze"))
    } else if l.contains("raff") || l.contains("berry") {
        Some(("kupfer-gemischt", "Raff"))
    } else if l.contains("kupfer") && l.contains("leicht") {
        Some(("kupfer-gemischt", "leicht"))
    // Kabel: Alu vor Kupfer (Erdkabel enthält "kabel" als Teilwort).
    // Muffen sind kein Kabelschrott, sondern Zuzahlungs-Sonderfall.
    } else if l.contains("muffen") {
        None
    } else if l.contains("kabel") && l.contains("alu") {
        if l.contains("erdkabel") {
            Some(("kabel-alu", "Erdkabel"))
        } else {
            Some(("kabel-alu", "40%"))
        }
    } else if l.contains("erdkabel") {
        Some(("kabel-kupfer", "Erdkabel"))
    } else if l.contains("kabel") && l.contains("stecker") {
        Some(("kabel-kupfer", "mit Stecker"))
    } else if l.contains("kabel") && l.contains("70") {
        Some(("kabel-kupfer", "70%"))
    } else if l.contains("kabel") && l.contains("38") {
        Some(("kabel-kupfer", "38%"))
    // Rotguss / Messing.
    } else if l.contains("rotguss") {
        if l.contains("späne") || l.contains("spaene") {
            Some(("bronze-rotguss", "Späne"))
        } else {
            Some(("bronze-rotguss", ""))
        }
    } else if l.contains("messing") {
        if l.contains("ms58") {
            Some(("messing", "Ms58"))
        } else if l.contains("späne") || l.contains("spaene") {
            Some(("messing", "Späne"))
        } else if l.contains("schwer") {
            Some(("messing", "schwer"))
        } else {
            Some(("messing", "leicht"))
        }
    // Zinn / Zink / Blei.
    } else if l.contains("lötzinn") || l.contains("loetzinn") {
        Some(("loetzinn", "Lötzinn"))
    } else if l.contains("geschirrzinn") {
        Some(("zinn-geschirr", "Geschirr"))
    } else if l.contains("zinn") {
        Some(("zinn", "99%"))
    } else if l.contains("zink") {
        if l.contains("alt") {
            Some(("zink", "alt"))
        } else {
            Some(("zink", "neu"))
        }
    } else if l.contains("auswucht") || l.contains("wucht") {
        Some(("blei-auswucht", "Auswucht"))
    } else if l.contains("batterie") {
        None
    } else if l.contains("blei") {
        if l.contains("weich") {
            Some(("blei", "weich"))
        } else {
            Some(("blei", "gemischt"))
        }
    // Edelstahl vor Aluminium: V2A/V4A-Späne dürfen nicht im
    // Alu-Späne-Arm landen (db-Präzedenz: Chromstahl → generisch).
    } else if l.contains("v4a") {
        if l.contains("späne") || l.contains("spaene") {
            Some(("edelstahl-v4a", "Späne"))
        } else {
            Some(("edelstahl-v4a", ""))
        }
    } else if l.contains("v2a") || l.contains("nirosta") {
        if l.contains("späne") || l.contains("spaene") {
            Some(("edelstahl-v2a", "Späne"))
        } else {
            Some(("edelstahl-v2a", ""))
        }
    } else if l.contains("chromstahl") {
        Some(("edelstahl-gemischt", "Chromstahl"))
    // Aluminium (Offset/Felgen vor Blech/Guss; db-Präzedenz: Felgen sind
    // Guss-Alu).
    } else if l.contains("offset") {
        Some(("aluminium-blech", "Offset"))
    } else if l.contains("felgen") {
        if l.contains("behaftet") {
            Some(("aluminium-guss", "Felgen behaftet"))
        } else {
            Some(("aluminium-guss", "Felgen sauber"))
        }
    } else if l.contains("profil") {
        if l.contains("lackiert") {
            Some(("aluminium-profile", "lackiert"))
        } else if l.contains("alt") || l.contains("ausbau") {
            Some(("aluminium-profile", "alt"))
        } else {
            Some(("aluminium-profile", "blank"))
        }
    } else if l.contains("blech") {
        if l.contains("5000") || l.contains("6000") {
            Some(("aluminium-blech", "5000/6000"))
        } else if l.contains("lackiert") {
            Some(("aluminium-blech", "lackiert"))
        } else {
            Some(("aluminium-blech", "legiert"))
        }
    } else if l.contains("guss") && l.contains("alu") {
        if l.contains("alt") {
            Some(("aluminium-guss", "alt"))
        } else {
            Some(("aluminium-guss", "neu"))
        }
    } else if l.contains("geschirr") {
        Some(("aluminium-blech", "Geschirr"))
    } else if l.contains("späne") || l.contains("spaene") {
        // Nur Alu-Späne erreichen diesen Arm (Messing-/Rotguss-/V2A-Späne
        // sind oben schon gefangen); FE-Späne fallen unten raus.
        if l.contains("alu") {
            Some(("aluminium-gemischt", "Späne"))
        } else {
            None
        }
    } else if l.contains("aluminium") || l == "alu" {
        Some(("aluminium-gemischt", ""))
    // Eisen/Stahl (V2A/V4A/Chromstahl stehen oben vor dem Alu-Block).
    } else if l.contains("shreddervormaterial") {
        Some(("stahlschrott-shredder", ""))
    } else if l.contains("tresor") {
        None
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("schwer") || l.contains("handelsgu") {
        Some(("eisenschrott-gussbruch", "schwer"))
    } else if l.contains("neuschrott") {
        Some(("stahlschrott-sorte-1", ""))
    // E-Schrott: Leiterplatten → platinen (Handy-Platinen → handys),
    // CPUs/RAM/Geräte skippen laut (Frisch-Präzedenz: kein Fake-Material).
    } else if l.contains("leiterplat") || l.contains("platine") {
        if l.contains("handy") || l.contains("smartphone") {
            Some(("handys", "Handy"))
        } else if l.contains("1a") {
            Some(("platinen", "1A"))
        } else if l.contains("klasse 2") || l.contains("1b") {
            Some(("platinen", "2/1B"))
        } else if l.contains("klasse 3") {
            Some(("platinen", "3"))
        } else {
            Some(("platinen", ""))
        }
    } else if l.contains("ram") && !l.contains("keramik") {
        // RAM modules are their own material (gold vs. silver contacts
        // price apart); the fallback table records platinen acceptance
        // alongside. (CPUs stay unmapped below.)
        if l.contains("silber") || l.contains("silver") {
            Some(("ram", "Silberkontakte"))
        } else {
            Some(("ram", "Goldkontakte"))
        }
    } else if l.contains("cpu")
        || l.contains("festplatte")
        || l.contains("laufwerk")
        || l.contains("netzteil")
        || l.contains("silberkontakt")
        || l.contains("trafo")
        || l.contains("handy")
        || l.contains("smartphone")
        || l.contains("elektronikschrott")
        || l.contains("unberaubt")
    {
        None
    // Motoren / Hartmetall / HSS.
    } else if l.contains("motor") {
        Some(("elektromotoren", ""))
    } else if l.contains("hartmetall") {
        if l.contains("bohrer") || l.contains("fräser") || l.contains("fraeser") {
            Some(("hartmetall", "Bohrer/Fräser"))
        } else if l.contains("widia") || l.contains("wende") {
            Some(("hartmetall", "Widia"))
        } else if l.contains("säge") || l.contains("saege") {
            Some(("hartmetall", "Sägeblätter"))
        } else {
            Some(("hartmetall", "gemischt"))
        }
    } else if l.contains("hss") || l.contains("schnellarbeitsstahl") {
        Some(("hss-werkzeuge", ""))
    } else {
        None
    }
}

/// Reason for every loud skip (audit trail, no silent drops).
fn skip_reason(label: &str) -> &'static str {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("kühler") || l.contains("kuehler") {
        "Mischprodukt ohne Katalogmaterial"
    } else if l.contains("cpu") {
        "kein CPU-Material im Katalog"
    } else if l.contains("silberkontakt") {
        "Silber in EUR/g, Seite nennt EUR/kg"
    } else if l.contains("trafo") {
        "kein Trafo-Material im Katalog"
    } else if l.contains("batterie") {
        "kein Batterie-Material im Katalog"
    } else if l.contains("tresor") || l.contains("muffen") {
        "Zuzahlung, kein Ankaufspreis"
    } else if l.contains("handy")
        || l.contains("smartphone")
        || l.contains("festplatte")
        || l.contains("laufwerk")
        || l.contains("netzteil")
        || l.contains("elektronikschrott")
        || l.contains("unberaubt")
    {
        "Gerät, kein Material im Katalog"
    } else if l.contains("späne") || l.contains("spaene") {
        "Spänensorte uneindeutig"
    } else {
        "kein Katalogmaterial"
    }
}

/// Bespoke contact extraction for THIS impressum only: `<p>Angaben gemäß
/// § 5 TMG:</p>` verankert den Adressblock (Folge-`<p>`: Branche, Firma,
/// Straße, `PLZ Ort`); das `Kontakt:`-`<p>` trägt `Telefon:`/`E-Mail:`.
/// `<br`-Splits verwerfen alles bis zum ersten `>` (Tag-Rest-Gotcha),
/// sonst parst `class="…"` als Text. Fehlende Anker → lauter Error, nie
/// raten/fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let fail = |detail: &str| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: detail.to_owned(),
    };
    let doc = Html::parse_document(imp);
    let p = Selector::parse("p").expect("valid selector");
    let paras: Vec<ElementRef> = doc.select(&p).collect();
    let addr_idx = paras
        .iter()
        .position(|e| e.text().collect::<String>().trim() == "Angaben gemäß § 5 TMG:")
        .ok_or_else(|| fail("TMG-Anker fehlt"))?;
    let addr_lines = br_lines(
        paras
            .get(addr_idx + 1)
            .ok_or_else(|| fail("Adressblock fehlt"))?,
    );
    if addr_lines.len() < 4 || !addr_lines[0].contains("Rohstoffhandel") {
        return Err(fail("Adressblock-Form unerwartet"));
    }
    let street = addr_lines[addr_lines.len() - 2].clone();
    let last = addr_lines.last().expect("len checked").clone();
    let mut it = last.split_whitespace();
    let (postcode, city) = match (it.next(), it.next()) {
        (Some(pc), Some(ci)) if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) => {
            (pc.to_owned(), ci.to_owned())
        }
        _ => return Err(fail("PLZ/Ort-Zeile unerwartet")),
    };
    let contact = paras
        .iter()
        .find(|e| {
            e.text()
                .collect::<String>()
                .trim_start()
                .starts_with("Kontakt:")
        })
        .ok_or_else(|| fail("Kontakt-Block fehlt"))?;
    let mut phone = String::new();
    let mut email = String::new();
    for line in br_lines(contact).into_iter().skip(1) {
        if let Some(v) = line.strip_prefix("Telefon:") {
            if phone.is_empty() {
                phone = v.trim().to_owned();
            }
        } else if let Some(v) = line.strip_prefix("E-Mail:") {
            if email.is_empty() {
                email = v.trim().to_owned();
            }
        }
    }
    if street.is_empty() || phone.is_empty() || email.is_empty() {
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

/// `<br`-Zeilen eines Elements; alles bis zum ersten `>` wird verworfen
/// (Tag-Rest-Gotcha), dann Tags gestrippt.
fn br_lines(e: &ElementRef) -> Vec<String> {
    let mut out = Vec::new();
    for part in e.inner_html().split("<br") {
        let after = part.find('>').map(|i| &part[i + 1..]).unwrap_or(part);
        let t = strip_tags(after);
        if !t.is_empty() {
            out.push(t);
        }
    }
    out
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
        Vec<(String, &'static str, f64)>,
        Vec<String>,
    ),
    IngestError,
> {
    let doc = Html::parse_document(html);
    let table = Selector::parse("table").expect("valid selector");
    let row = Selector::parse("tr").expect("valid selector");
    let cell = Selector::parse("td").expect("valid selector");
    let head = Selector::parse("th").expect("valid selector");
    let strong = Selector::parse("p strong").expect("valid selector");
    // "Metalleinkaufspreise ab 28.09.2026" — ein Datum pro Seite.
    let mut published_at = None;
    for el in doc.select(&strong) {
        let text: String = el.text().collect();
        if let Some(date) = text.split("Metalleinkaufspreise ab").nth(1) {
            let parts: Vec<&str> = date.trim().split('.').collect();
            if parts.len() == 3 {
                published_at = parse_de_date(parts[0], parts[1], parts[2]);
            }
        }
    }
    // Never trust page order: only tables carrying a `Menge:` staffel
    // header are price tables — the opening-hours table has none and is
    // never touched.
    let mut found = false;
    let mut rows: Vec<(String, &'static str, f64)> = Vec::new();
    let mut skipped = Vec::new();
    for t in doc.select(&table) {
        let has_staffel = t.select(&head).any(|h| {
            h.text()
                .collect::<String>()
                .to_lowercase()
                .contains("menge")
        });
        if !has_staffel {
            continue;
        }
        found = true;
        // Tiers gelten pro Tabelle: stale tiers aus der Vortabelle wären
        // falsche Varianten (T1/T3/T4 mischen Staffelsätze).
        let mut tiers: Vec<Option<&'static str>> = Vec::new();
        for tr in t.select(&row) {
            let heads: Vec<String> = tr.select(&head).map(|c| c.text().collect()).collect();
            if !heads.is_empty() {
                if heads[0].to_lowercase().contains("menge") {
                    tiers = heads[1..].iter().map(|h| tier_static(h)).collect();
                }
                continue;
            }
            let cells: Vec<ElementRef> = tr.select(&cell).collect();
            if cells.is_empty() {
                continue;
            }
            let texts: Vec<String> = cells
                .iter()
                .map(|c| {
                    c.text()
                        .collect::<String>()
                        .replace(['\u{a0}'], " ")
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect();
            let label = texts[0].clone();
            if label.is_empty() {
                continue;
            }
            // Unter-Köpfe als <td>-Zeilen ("Menge: …"): aktualisieren die
            // Staffeln statt Daten zu liefern (T1/T3/T4 mischen mehrere
            // Staffelsätze in einer Tabelle; stale tiers = falsche Varianten).
            if label.trim().trim_end_matches(':').to_lowercase() == "menge" {
                tiers = texts[1..].iter().map(|h| tier_static(h)).collect();
                continue;
            }
            if texts.iter().any(|c| c.to_lowercase().contains("zuzahlung")) {
                skipped.push(format!("{label} (Zuzahlung, kein Ankaufspreis)"));
                continue;
            }
            if tiers.is_empty() {
                return Err(IngestError::Parse {
                    url: URL.to_owned(),
                    detail: "Staffelkopf fehlt".to_owned(),
                });
            }
            if texts.len() - 1 != tiers.len() {
                skipped.push(format!(
                    "{label} (Spaltenzahl passt nicht zu Staffeln: {})",
                    texts.len() - 1
                ));
                continue;
            }
            for (tier, val) in tiers.iter().zip(texts[1..].iter()) {
                let Some(tier) = tier else {
                    skipped.push(format!("{label} (Staffel unbekannt)"));
                    continue;
                };
                let tier: &'static str = tier;
                // Explizit-fremde Einheiten ("/ t", "pro …") skippen laut —
                // nie in den kg-Default zwingen.
                if val.contains('/') || val.to_lowercase().contains("pro ") {
                    skipped.push(format!("{label} [{tier}] (Einheit unverständlich: {val})"));
                    continue;
                }
                let Some(price) = parse_eur(val) else {
                    skipped.push(format!("{label} [{tier}] (kein Preis: {val})"));
                    continue;
                };
                if price == 0.0 {
                    skipped.push(format!("{label} [{tier}] (Preis 0,00)"));
                    continue;
                }
                rows.push((label.clone(), tier, price));
            }
        }
    }
    if !found {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabelle".to_owned(),
        });
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle leer".to_owned(),
        });
    }
    Ok((published_at, rows, skipped))
}

/// Staffel-Header dieser Seite auf statische Literale (Tausenderpunkt
/// `1.000` wird normalisiert, sonst kollabieren `ab 1000 kg`/`ab 1.000 kg`
/// in verschiedene Varianten). Unbekannt → lauter Skip am Call-Site, nie
/// raten.
fn tier_static(h: &str) -> Option<&'static str> {
    match h
        .to_lowercase()
        .replace(['\u{a0}'], " ")
        .replace('.', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .as_str()
    {
        "bis 25 kg" => Some("bis 25 kg"),
        "ab 25 kg" => Some("ab 25 kg"),
        "ab 250 kg" => Some("ab 250 kg"),
        "ab 1000 kg" => Some("ab 1000 kg"),
        "bis 10 kg" => Some("bis 10 kg"),
        "ab 10 kg" => Some("ab 10 kg"),
        "ab 100 kg" => Some("ab 100 kg"),
        "ab 500 kg" => Some("ab 500 kg"),
        "ab 300 kg" => Some("ab 300 kg"),
        "ab 5 kg" => Some("ab 5 kg"),
        "ab 30 kg" => Some("ab 30 kg"),
        "ab 50 kg" => Some("ab 50 kg"),
        "ab 200 kg" => Some("ab 200 kg"),
        "ab 15 kg" => Some("ab 15 kg"),
        "ab 20 kg" => Some("ab 20 kg"),
        "ab 3 kg" => Some("ab 3 kg"),
        "ab 350 kg" => Some("ab 350 kg"),
        "ab 1000 kg bis 5000 kg" | "1000 kg bis 5000 kg" => Some("1000-5000 kg"),
        "ab 5000 kg" => Some("ab 5000 kg"),
        "25 kg bis 100 kg" => Some("25-100 kg"),
        "100 kg bis 250 kg" => Some("100-250 kg"),
        "250 kg bis 1000 kg" => Some("250-1000 kg"),
        "100 kg bis 1000 kg" => Some("100-1000 kg"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, skip_reason, tier_static};

    // Real excerpts of the live page (28.09.2026): date block, Cu table
    // head + Millberry/Kerze/Kühler rows, Alu Zuzahlung row, CPU skip row,
    // 0,00 row, V2A row — plus the opening-hours table (no Menge: head,
    // must stay untouched).
    const FIXTURE: &str = "<div class=\"purchaseprices\">\
        <p><strong>Metalleinkaufspreise ab 28.09.2026</strong><br />alle Angaben sind ohne Gewähr </p>\
        <h3>Cu / Ms</h3><table><thead><tr><th>Menge:</th><th>bis 25 kg</th>\
        <th>ab 25 kg</th><th>ab 250 kg</th><th>ab 1.000 kg</th></tr></thead><tbody>\
        <tr><td><figure><a href=\"http://www.westarp-kg.de/de/einkaufspreise/kupfer-blank-i-millberry\">\
        <img alt=\"\" src=\"x\"></a></figure>Kupfer, blank I, Millberry</td>\
        <td>9,91</td><td>11,16</td><td>11,31</td><td>11,86</td></tr>\
        <tr><td>Kupfer, blank II, Kerze</td><td>9,56</td><td>10,76</td><td>11,06</td><td>11,36</td></tr>\
        <tr><td>Kupfer-Messing-Kühler, rein</td><td>3,07</td><td>3,84</td><td>4,23</td><td>5,00</td></tr>\
        <tr><td>Messing, schwer</td><td>0,00</td><td>5,40</td><td>5,70</td><td>6,30</td></tr>\
        </tbody></table>\
        <h3>Al / Zn / Pb / VA</h3><table><thead><tr><th>Menge:</th><th>bis 25 kg</th>\
        <th>ab 25 kg</th><th>ab 250 kg</th><th>ab 1.000 kg</th></tr></thead><tbody>\
        <tr><td>Erdkabel Muffen</td><td>Zuzahlung</td><td>100 €</td><td>/ to</td><td></td></tr>\
        <tr><td><strong>Menge: </strong></td><td><strong>bis 10 kg</strong></td>\
        <td><strong>ab 10 kg</strong></td><td><strong>ab 100 kg</strong></td>\
        <td><strong>ab 500 kg</strong></td></tr>\
        <tr><td>Altblei, weich, Weichblei ohne Anhaftungen</td>\
        <td>0,60</td><td>0,75</td><td>0,83</td><td>1,00</td></tr>\
        </tbody></table>\
        <table><thead><tr><th>Menge:</th><th>ab 5 kg</th><th>ab 30 kg</th><th>ab 100 kg</th></tr></thead>\
        <tbody><tr><td>CPU, Keramik, gemischt</td><td>50,00</td><td>55,00</td><td>66,00</td></tr>\
        <tr><td>Edelstahl, V2A, Nirosta, 18/8</td><td>0,50</td><td>0,70</td><td>0,78</td></tr>\
        </tbody></table>\
        <table><tr><td>Montag - Freitag</td><td>07:30 - 12:00 Uhr</td></tr></table></div>";

    const IMPRESSUM_FIXTURE: &str = "<main><h1>Impressum</h1><div>\
        <p>Angaben gemäß § 5 TMG:</p>\
        <p>Rohstoffhandel<br />BERNHARD WESTARP GmbH &amp; Co. KG<br />\
        Hafenrandstraße 5-6<br />63741 Aschaffenburg</p>\
        <p>Vertreten durch:<br />Jürgen Westarp</p>\
        <p>Kontakt:<br />Telefon: 06021/8460-0<br />Telefax: 06021/80522<br />\
        E-Mail: info@westarp-kg.de<br />Handelsregister Amtsgericht Aschaffenburg HRA-Nr. 466</p></div></main>";

    #[test]
    fn table_selection_and_date() {
        let (published, rows, skipped) = parse(FIXTURE).expect("fixture parses");
        assert_eq!(published.as_deref(), Some("2026-09-28T00:00:00+00:00"));
        // parse() liefert Rohzeilen (Mapping-Skips passieren in scrape()):
        // Millberry 4 + Kerze 4 + Kühler 4 + Messing-schwer 3 (eine 0,00)
        // + CPU 3 + V2A 3 + Altblei 4 (td-Unterkopf mit eigenem Staffelsatz).
        // Kühler/CPU fallen erst im Mapping raus.
        assert_eq!(rows.len(), 25);
        assert_eq!(
            rows[0],
            ("Kupfer, blank I, Millberry".to_owned(), "bis 25 kg", 9.91)
        );
        assert_eq!(
            rows[3],
            ("Kupfer, blank I, Millberry".to_owned(), "ab 1000 kg", 11.86)
        );
        // Parse-Level-Skips sind laut und begründet (Mapping-Skips für
        // Kühler/CPU deckt der Mapping-Test ab).
        assert!(skipped
            .iter()
            .any(|s| s.contains("Erdkabel Muffen") && s.contains("Zuzahlung")));
        assert!(skipped
            .iter()
            .any(|s| s.contains("Messing, schwer") && s.contains("0,00")));
        // Kühler + CPU stehen als Rohzeilen drin, fallen im Mapping raus.
        assert!(rows
            .iter()
            .any(|(l, _, _)| l.contains("Kupfer-Messing-Kühler")));
        assert!(rows.iter().any(|(l, _, _)| l.contains("CPU, Keramik")));
        // td-Unterkopf überschreibt die Staffeln (keine stale tiers).
        assert!(rows
            .iter()
            .any(|(l, t, p)| l.contains("Altblei") && *t == "bis 10 kg" && *p == 0.60));
        assert!(rows
            .iter()
            .any(|(l, t, p)| l.contains("Altblei") && *t == "ab 500 kg" && *p == 1.00));
        // Öffnungszeiten-Tabelle liefert keine Zeilen.
        assert!(!rows.iter().any(|(l, _, _)| l.contains("Montag")));
    }

    #[test]
    fn tier_thousands_normalized() {
        assert_eq!(tier_static("ab 1.000 kg"), Some("ab 1000 kg"));
        assert_eq!(tier_static("ab 1000 kg"), Some("ab 1000 kg"));
        assert_eq!(tier_static("25 kg bis 100 kg"), Some("25-100 kg"));
        assert_eq!(tier_static("ab 99 kg"), None);
    }

    #[test]
    fn mapping_covers_every_live_label() {
        // Jede Live-Bezeichnung landet bewusst auf einem Material — oder
        // wird laut geskippt. Stand: Recon 28.09.2026 (80 Labels).
        let mapped = [
            "Kupfer, blank I, Millberry",
            "Kupfer, blank II, Kerze",
            "Kupfer, gemischt, Raff, alt 95 %, Berry",
            "Kupfer, leicht",
            "Kupfer-Messing-Kühler, max. 5 % Anhaftungen",
            "Kupfer-Messing-Kühler, rein",
            "Kupferkabel mit Stecker und Kupferkabel, mind. 25 % Cu",
            "Kupfer-Erdkabel, mind. 25 % Cu",
            "Kupferkabel, mind. 38 % Cu",
            "Kupferkabel, mind. 70 % Cu",
            "Messing Ms58, neu, stückig",
            "Messingspäne, gemischt, max. 5 % Verunreinigungen",
            "Messing, schwer",
            "Messing, leicht und Hülsen (separat), Messing mit Schläuchen",
            "Rotguss, stückig",
            "Rotgussspäne",
            "Aluminium-Geschirr, max. 5 % Anhaftungen",
            "Aluminiumbleche, blank, neu, 5000 oder 6000",
            "Aluminiumbleche, blank, neu, mit Sn, Zn, Cu, Mn etc.",
            "Aluminiumbleche, alt, lackiert",
            "Aluminiumprofile, blank, neu",
            "Aluminiumprofile, alt / Ausbauprofile abzgl. Anhaftungen",
            "Aluminiumprofile, lackiert",
            "Aluminiumguss, neu, ohne Anhaftungen",
            "Aluminiumguss, alt, max. 5 % Anhaftungen",
            "Aluminiumfelgen, sauber",
            "Aluminiumfelgen, behaftet",
            "Aluminium-Offsetplatten",
            "Aluminiumspäne, max. 5 % Anhaftungen",
            "Aluminium-Kabel, 40 % ohne Cu-Ummantelung Al",
            "Aluminium-Erdkabel",
            "Erdkabel Muffen",
            "Aluminium-Kühler, rein",
            "Aluminium-Kühler, behaftet, abzgl. Anhaftungen",
            "Aluminium-Kupfer-Kühler, rein",
            "Aluminium-Kupfer-Kühler, behaftet, abzgl. Anhaftungen",
            "Zink, neu",
            "Zink, alt",
            "Altblei, weich, Weichblei ohne Anhaftungen",
            "Altblei, gemischt",
            "Bleibatterien",
            "Auswuchtblei, Wuchtgewichte Pb",
            "Zinn, rein, mind. 99 %",
            "Zinnschrott, Lötzinn",
            "Geschirrzinn",
            "CPU, Kunststoff, behaftet",
            "CPU, Keramik, gemischt",
            "CPU, Kunststoff, gemischt",
            "Slot-CPU",
            "Handys, Smartphones ohne Akku",
            "Handy-, Smartphoneplatinen",
            "Leiterplatten, Klasse 1A",
            "Leiterplatten, Klasse 2 und 1B",
            "Leiterplatten, Klasse 3",
            "RAM, mit Goldkontakten",
            "RAM, mit Silberkontakten",
            "Festplatten",
            "Laufwerke",
            "Netzteile",
            "PCs, unberaubt",
            "Elektronik-Motoren, ohne Getriebe etc.",
            "Kleintrafos",
            "Elektronikschrott, gemischt",
            "Silberkontakte",
            "Hartmetall, Bohrer und Fräser",
            "Hartmetall, Wendeschneidplatten Widia",
            "Hartmetall, gemischt",
            "Hartmetall, Sägeblätter",
            "HSS, gemischt (Schnellarbeitsstahl)",
            "Edelstahl, V2A, Nirosta, 18/8",
            "Edelstahl, V4A, Schrott, 20/10/2",
            "Chromstahl",
            "V2A-Späne",
            "V4A-Späne",
            "Shreddervormaterial",
            "Tresore",
            "Mischschrott",
            "Schwerschrott / Handelsguß",
            "Späne, sauber",
            "Neuschrott",
        ];
        assert_eq!(mapped.len(), 80);
        let mut unmapped = Vec::new();
        for label in mapped {
            let l = label.to_lowercase();
            let is_skip = grade_for(label).is_none();
            // Erwartete Skips (laut, mit Grund): Kühler, CPUs/Geräte,
            // Batterien, Tresore/Muffen, Silberkontakte, Trafos, Späne.
            // RAM hat eigenes Material (Gold-/Silberkontakte als Variante).
            let expect_skip = l.contains("kühler")
                || l.contains("kuehler")
                || l.contains("cpu")
                || (l.contains("handy") && !l.contains("platine"))
                || (l.contains("smartphone") && !l.contains("platine"))
                || l.contains("festplatte")
                || l.contains("laufwerk")
                || l.contains("netzteil")
                || (l.contains("silberkontakt") && !l.contains("ram"))
                || l.contains("trafo")
                || l.contains("elektronikschrott")
                || l.contains("unberaubt")
                || l.contains("batterie")
                || l.contains("tresor")
                || l.contains("muffen")
                || l == "späne, sauber";
            if is_skip != expect_skip {
                unmapped.push(label);
            }
        }
        assert!(unmapped.is_empty(), "mapping drift: {unmapped:?}");
        // Spot-Checks spezifisch-vor-generisch.
        assert_eq!(
            grade_for("Kupfer-Messing-Kühler, rein"),
            None,
            "Kühler cramt nicht in Kupfer/Messing"
        );
        assert_eq!(
            grade_for("Kupfer, blank II, Kerze"),
            Some(("kupfer-berry", "Kerze"))
        );
        assert_eq!(
            grade_for("Aluminium-Erdkabel"),
            Some(("kabel-alu", "Erdkabel")),
            "Erdkabel cramt nicht in Kupfer-Kabel"
        );
        assert_eq!(
            grade_for("Chromstahl"),
            Some(("edelstahl-gemischt", "Chromstahl"))
        );
        assert_eq!(grade_for("Neuschrott"), Some(("stahlschrott-sorte-1", "")));
        assert_eq!(
            grade_for("Handy-, Smartphoneplatinen"),
            Some(("handys", "Handy"))
        );
        let _ = skip_reason;
    }

    #[test]
    fn impressum_extracts_contact() {
        let info = extract_info(IMPRESSUM_FIXTURE).expect("impressum parses");
        assert_eq!(info.street, "Hafenrandstraße 5-6");
        assert_eq!(info.postcode, "63741");
        assert_eq!(info.city, "Aschaffenburg");
        assert_eq!(info.phone, "06021/8460-0");
        assert_eq!(info.email, "info@westarp-kg.de");
    }

    #[test]
    fn impressum_anchors_fail_loudly() {
        assert!(extract_info("<main><p>leer</p></main>").is_err());
    }
}
