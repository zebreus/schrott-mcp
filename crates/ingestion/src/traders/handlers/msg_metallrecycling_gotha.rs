//! MSG Metallrecycling Gotha: Webflow-CMS-Preisliste (`/preisliste`) als
//! Produktkarten (`a.product4_text-link`) in `section#preiseSchrott`.
//! Jede Karte nennt eine Sorte mit bis zu drei €/kg-Mengenstaffeln
//! ("ab 1 kg" / "ab 100 kg" / "ab 1000 kg", englischer Dezimalpunkt,
//! leere Staffeln als `w-dyn-bind-empty`). Erfasst wird nur die kleinste
//! bepreiste Staffel je Sorte (Jedermann-Preis; höhere Staffeln brauchen
//! 100/1000 kg) — höhere Staffeln werden als eine Summenzeile geskippt,
//! damit sie die interessanten Skips nicht zuschütten. Variante trägt die
//! Händlersorte (statisch, damit nichts kollabiert). Einheit steht nie
//! explizit da: bloßes "€" neben "ab N kg"-Staffeln, also dokumentierte
//! EUR/kg-Konstante (Alu ~1,3 / Bronze 5,3 / Cu ~9 / Stahl ~0,1 —
//! markt-plausibel). Kein Seitendatum ("tagesaktuell" ist kein Datum).
//! `0,00` (Hartmetall Schlamm) = kein Ankaufspreis, lauter Skip.
//! Versilbertes/Silberbesteck hat keinen Katalogeintrag (Silber ist EUR/g)
//! und wird laut geskippt; Edel-/Sondermetalle ohne Preis ("Preis auf
//! Anfrage": Inconell, Nickel, Zinn Krätze, Au/Ag-Abfälle, Pd, Pt, Rh, W)
//! ebenfalls; In/Ir/Co/Mo/Nb/Ta-Dummypreise (durchgehend 10,00) fallen
//! über fehlende Katalogmaterialien raus.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "th-gotha-msg-metallrecycling-msg-schrott-und-meta";
/// Bespoke, live-verified price URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://www.msg-metallrecycling.de/preisliste";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.msg-metallrecycling.de/legal/impressum";

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
    // Dedup identical (material, variant, price) triples after mapping.
    let mut seen = std::collections::HashSet::new();
    for row in rows {
        match grade_for(&row.title) {
            Some((material, variant)) => {
                if seen.insert((material, variant, row.price.to_bits())) {
                    prices.push(ScrapedPrice {
                        material,
                        variant,
                        price: row.price,
                        currency: "EUR",
                        unit: row.unit,
                        price_kind: "exact",
                        price_min: None,
                        price_max: None,
                        confidence: Some(1.0),
                        // Rohlabel inkl. Staffel → landet als notes.
                        label: format!("{} ({})", row.title, row.tier),
                    });
                }
            }
            None => skipped_labels.push(format!("{} ({})", row.title, row.tier)),
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

/// One kept price row: trader title, kept tier, price, unit.
#[derive(Debug)]
struct Row {
    title: String,
    tier: String,
    price: f64,
    unit: &'static str,
}

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Specific-before-generic: "Kupfer" fängt sonst alles mit Cu.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", "Millberry"))
    } else if l.contains("lackdraht") {
        Some(("kupfer-berry", "Lackdraht"))
    } else if l.contains("leitschienen") {
        Some(("kupfer-gemischt", "Leitschienen Blank"))
    } else if l.contains("kupfer leicht") {
        Some(("kupfer-gemischt", "Leicht"))
    } else if l.contains("kupfer schwer") {
        Some(("kupfer-gemischt", "Schwer"))
    } else if l.contains("kupfer späne") || l.contains("kupfer spaene") {
        Some(("kupfer-gemischt", "Späne"))
    } else if l.contains("alu kabel") {
        Some(("kabel-alu", ""))
    } else if l.contains("kabel 50%") {
        Some(("kabel-kupfer", "50%"))
    } else if l.contains("kabel 60%") {
        Some(("kabel-kupfer", "60%"))
    } else if l.contains("kabel 80%") {
        Some(("kabel-kupfer", "80%"))
    } else if l.contains("kabel") {
        Some(("kabel-kupfer", ""))
    } else if l.contains("schwermessing") {
        Some(("messing", "Schwermessing"))
    } else if l.contains("ms 58") {
        Some(("messing", "Ms 58 Abfälle"))
    } else if l.contains("erodierdraht") {
        Some(("messing", "Ms Erodierdraht"))
    } else if l.contains("ms leicht") {
        Some(("messing", "Ms leicht"))
    } else if l.contains("ms späne") || l.contains("ms spaene") {
        Some(("messing", "Ms Späne gemischt"))
    } else if l.contains("wasseruhren") {
        Some(("messing-leicht", "Wasseruhren"))
    } else if l.contains("rotguss späne") || l.contains("rotguss spaene") {
        Some(("bronze-rotguss", "Rotguss Späne"))
    } else if l.contains("rotguss stücke") || l.contains("rotguss stuecke") {
        Some(("bronze-rotguss", "Rotguss Stücke"))
    } else if l.contains("bronze schrott") {
        Some(("bronze-rotguss", "Bronze Schrott"))
    } else if l.contains("bronze späne") || l.contains("bronze spaene") {
        Some(("bronze-rotguss", "Bronze Späne"))
    } else if l.contains("alu bleche blank") {
        Some(("aluminium-blech", "Bleche blank"))
    } else if l.contains("alu bleche farbe") {
        Some(("aluminium-blech", "Bleche Farbe"))
    } else if l.contains("alu geschirr") {
        Some(("aluminium-blech", "Geschirr"))
    } else if l.contains("alu felgen") {
        // Felgen sind Gusslegierung (AlSi) → Guss, Sorte bleibt stehen.
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("alu guss") {
        Some(("aluminium-guss", "Guss"))
    } else if l.contains("alu profile blank") {
        Some(("aluminium-profile", "Profile blank"))
    } else if l.contains("alu profile iso") {
        Some(("aluminium-profile", "Profile iso"))
    } else if l.contains("alu späne") || l.contains("alu spaene") {
        Some(("aluminium-gemischt", "Späne"))
    } else if l.contains("altblei") {
        Some(("blei", ""))
    } else if l.contains("v4a") {
        if l.contains("späne") || l.contains("spaene") {
            Some(("edelstahl-v4a", "Späne"))
        } else {
            Some(("edelstahl-v4a", ""))
        }
    } else if l.contains("v2a") {
        if l.contains("späne") || l.contains("spaene") {
            Some(("edelstahl-v2a", "Späne"))
        } else {
            Some(("edelstahl-v2a", ""))
        }
    } else if l.contains("elektromotoren") {
        Some(("elektromotoren", ""))
    } else if l.contains("hartmetall") {
        if l.contains("bohrer") || l.contains("fräser") || l.contains("plättchen") {
            Some(("hartmetall", "Bohrer/Fräser/Plättchen"))
        } else if l.contains("stückschrott") {
            Some(("hartmetall", "Stückschrott"))
        } else if l.contains("schlamm") {
            Some(("hartmetall", "Schlamm"))
        } else {
            Some(("hartmetall", ""))
        }
    } else if l.contains("bremsscheiben") {
        Some(("eisenschrott-gussbruch", "Bremsscheiben"))
    } else if l.contains("gusseisen") {
        Some(("eisenschrott-gussbruch", "Gusseisen"))
    } else if l.contains("mischschrott schwer") {
        Some(("mischschrott", "schwer"))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("schredder") {
        Some(("stahlschrott-shredder", ""))
    } else if l.contains("zinn geschirr") {
        Some(("zinn-geschirr", "Geschirr"))
    } else if l.contains("zinn krätze") || l.contains("zinn kraetze") {
        // Zinnkrätze (Dross) hat niedrigen, unbelegten Sn-Gehalt — kein
        // Reinzinn-Preis darauf (FE/NE-Audits).
        None
    } else if l.contains("zinkblech") {
        Some(("zink", "Blech neu und alt"))
    } else if l.contains("zinkguss") {
        Some(("zink", "Guss"))
    } else {
        // Lauter Skip: Alu Cu Kühler (Mischmetall), Stahl- und Gussspäne
        // (zwei Materialien), Versilbertes/Silberbesteck (kein
        // Katalogmaterial — Silber ist EUR/g), Inconell/Nickel und
        // In/Ir/Co/Mo/Nb/Ta/W (kein Katalogmaterial).
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after the
/// `<h1>Impressum</h1>` carries "Angaben gemäß § 5 TMG" plus firm lines,
/// street, "PLZ Ort, DE", "Telefon:" and "Mail:" rows separated by `<br>`.
/// A zero-width joiner (U+200D) clings to some breaks and is stripped.
/// Missing anchors mean the page changed shape → loud error, never a
/// guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let anchor = doc
        .select(&h1)
        .find(|h| h.text().collect::<String>().trim() == "Impressum");
    if anchor.is_none() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Heading fehlt".to_owned(),
        });
    }
    let body = doc.select(&p).find(|e| {
        let t: String = e.text().collect();
        t.contains("Angaben gem") && t.contains("5 TMG")
    });
    let Some(body) = body else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    // <br-split discipline: discard everything up to the first '>' so no
    // tag remnants ("class=…") parse as text.
    let mut lines = Vec::new();
    for part in body.inner_html().split("<br") {
        let t = strip_tags(part);
        let t: String = t
            .chars()
            .filter(|c| !matches!(c, '\u{200b}' | '\u{200d}' | '\u{feff}'))
            .collect();
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if !t.is_empty() {
            lines.push(t);
        }
    }
    let mut street = String::new();
    let mut postcode = String::new();
    let mut city = String::new();
    let mut phone = String::new();
    let mut email = String::new();
    for (i, line) in lines.iter().enumerate() {
        // "99867 Gotha, DE" — PLZ zuerst, Stadt dahinter.
        if postcode.is_empty() {
            let mut it = line.split_whitespace();
            if let (Some(pc), Some(_)) = (it.next(), it.next()) {
                if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    // "99867 Gotha, DE" → Stadt ohne Länderkürzel.
                    let after = line[pc.len()..].trim();
                    let after = after
                        .trim_end_matches(", DE")
                        .trim_end_matches(",DE")
                        .trim();
                    city = after.to_owned();
                    // Straße = Zeile direkt davor, wenn sie eine Nummer trägt.
                    if i > 0 && lines[i - 1].chars().any(|c| c.is_ascii_digit()) {
                        street = lines[i - 1].clone();
                    }
                }
            }
        }
        if let Some(v) = line.strip_prefix("Telefon:") {
            if phone.is_empty() {
                phone = v.trim().to_owned();
            }
        }
        if let Some(v) = line.strip_prefix("Mail:") {
            let v = v.trim().to_owned();
            if email.is_empty() && v.contains('@') {
                email = v;
            }
        }
    }
    if street.is_empty() && postcode.is_empty() && phone.is_empty() && email.is_empty() {
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

/// Page unit: the cards quote bare "€" next to "ab N kg" tiers, so EUR/kg
/// is the documented constant (kg-plausibel: Alu ~1,3 / Bronze 5,3 /
/// Cu ~9 / Stahl ~0,1). A tier naming no kg skips loudly instead.
fn unit_of(tier: &str) -> Option<&'static str> {
    if tier.to_lowercase().contains("kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

fn parse(html: &str) -> Result<(Vec<Row>, Vec<String>), IngestError> {
    // Window discipline: only section#preiseSchrott. A stray "49 €" in
    // header/footer must never pair with a label into a phantom price.
    let start = html
        .find("preiseSchrott")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisbereich fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("<footer").unwrap_or(tail.len());
    let window = &tail[..end];
    let doc = Html::parse_document(window);
    let card = Selector::parse("a.product4_text-link").expect("valid selector");
    let title = Selector::parse("div.text-weight-semibold").expect("valid selector");
    let tier_sel = Selector::parse("div.pricedisplay").expect("valid selector");
    let div = Selector::parse("div").expect("valid selector");
    let cards: Vec<ElementRef> = doc.select(&card).collect();
    if cards.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preiskarten".to_owned(),
        });
    }
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    let mut higher_tiers = 0usize;
    for c in cards {
        let Some(t) = c.select(&title).next() else {
            continue;
        };
        let name = t
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if name.is_empty() || name.len() > 120 {
            continue;
        }
        let mut kept = false;
        let mut zero_skipped = false;
        for tier_el in c.select(&tier_sel) {
            // Four cells: tier ("ab 1 kg:"), hidden tier, value, "€".
            let cells: Vec<String> = tier_el
                .select(&div)
                .map(|d| {
                    d.text()
                        .collect::<String>()
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect();
            if cells.len() < 3 {
                continue;
            }
            let tier = cells[0].trim().to_owned();
            let value = cells[2].trim().to_owned();
            if value.is_empty() {
                // Empty CMS field (w-dyn-bind-empty): layout noise, silent
                // unless the whole card has no price at all.
                continue;
            }
            let Some(price) = parse_eur(&value) else {
                skipped.push(format!("{name} ({tier}: kein Preis: {value})"));
                continue;
            };
            // A "0,00" tier is "no quote", not a free gift: loud skip,
            // and the card is done (no fallthrough to higher tiers).
            if price == 0.0 {
                skipped.push(format!("{name} ({tier}: Preis 0,00 — kein Ankaufspreis)"));
                zero_skipped = true;
                kept = true;
                break;
            }
            let Some(unit) = unit_of(&tier) else {
                skipped.push(format!("{name} (Einheit unverständlich: {tier})"));
                continue;
            };
            if !kept {
                rows.push(Row {
                    title: name.clone(),
                    tier,
                    price,
                    unit,
                });
                kept = true;
            } else {
                // Higher quantity tier: counted once below, not per row.
                higher_tiers += 1;
            }
        }
        if !kept && !zero_skipped {
            skipped.push(format!("{name} (kein Preis: Preis auf Anfrage)"));
        }
    }
    if higher_tiers > 0 {
        skipped.push(format!(
            "{higher_tiers} höhere Staffeln übersprungen (nur kleinste Staffel je Sorte erfasst)"
        ));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste leer".to_owned(),
        });
    }
    Ok((rows, skipped))
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    // Real excerpt shape of the live page (Webflow CMS cards, 28.09.2026):
    // title + up-to-three pricedisplay tiers (value cell or
    // w-dyn-bind-empty), "0,00" tier, all-empty card, first-tier-empty
    // card, stray euro amounts outside the price window.
    const FIXTURE: &str = "<p>Versand ab 49 €</p>\
        <section id=\"preiseSchrott\" class=\"section_product4\">\
        <div role=\"listitem\" class=\"w-dyn-item\"><div class=\"product4_item\">\
        <a href=\"#\" class=\"product4_text-link w-inline-block\">\
        <div class=\"text-size-medium text-weight-semibold\">Alu Bleche blank</div>\
        <div class=\"spacer-xxsmall\"></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>\
        <div class=\"text-size-medium\">1.30</div><div class=\"text-size-medium\">€</div></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 100 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 100 t:</div>\
        <div class=\"text-size-medium\">1.45</div><div class=\"text-size-medium\">€</div></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1000 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1000 t:</div>\
        <div class=\"text-size-medium\">1.65</div><div class=\"text-size-medium\">€</div></div>\
        </a></div></div>\
        <div role=\"listitem\" class=\"w-dyn-item\"><div class=\"product4_item\">\
        <a href=\"#\" class=\"product4_text-link w-inline-block\">\
        <div class=\"text-size-medium text-weight-semibold\">Bronze Schrott</div>\
        <div class=\"spacer-xxsmall\"></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>\
        <div class=\"text-size-medium\">5.30</div><div class=\"text-size-medium\">€</div></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 100 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 100 t:</div>\
        <div class=\"text-size-medium w-dyn-bind-empty\"></div><div class=\"text-size-medium\">€</div></div>\
        </a></div></div>\
        <div role=\"listitem\" class=\"w-dyn-item\"><div class=\"product4_item\">\
        <a href=\"#\" class=\"product4_text-link w-inline-block\">\
        <div class=\"text-size-medium text-weight-semibold\">Hartmetall Schlamm</div>\
        <div class=\"spacer-xxsmall\"></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>\
        <div class=\"text-size-medium\">0.00</div><div class=\"text-size-medium\">€</div></div>\
        </a></div></div>\
        <div role=\"listitem\" class=\"w-dyn-item\"><div class=\"product4_item\">\
        <a href=\"#\" class=\"product4_text-link w-inline-block\">\
        <div class=\"text-size-medium text-weight-semibold\">Bremsscheiben</div>\
        <div class=\"spacer-xxsmall\"></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>\
        <div class=\"text-size-medium w-dyn-bind-empty\"></div><div class=\"text-size-medium\">€</div></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 100 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 100 t:</div>\
        <div class=\"text-size-medium\">0.19</div><div class=\"text-size-medium\">€</div></div>\
        </a></div></div>\
        <div role=\"listitem\" class=\"w-dyn-item\"><div class=\"product4_item\">\
        <a href=\"#\" class=\"product4_text-link w-inline-block\">\
        <div class=\"text-size-medium text-weight-semibold\">Inconell</div>\
        <div class=\"spacer-xxsmall\"></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>\
        <div class=\"text-size-medium w-dyn-bind-empty\"></div><div class=\"text-size-medium\">€</div></div>\
        </a></div></div>\
        </section><footer><p>Container ab 99 €</p></footer>";

    #[test]
    fn smallest_tier_wins_and_window_holds() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // Alu blank (1.30), Bronze Schrott (5.30), Bremsscheiben (0.19 ab
        // 100 kg): stray "49 €" before and "99 €" in the footer pair with
        // nothing. Schlamm-0.00 and Inconell-empty yield no rows.
        assert_eq!(rows.len(), 3, "rows: {rows:?}");
        assert_eq!(rows[0].title, "Alu Bleche blank");
        assert_eq!(rows[0].tier, "ab 1 kg:");
        assert_eq!(rows[0].price, 1.3);
        assert_eq!(rows[0].unit, "EUR/kg");
        assert_eq!(rows[1].title, "Bronze Schrott");
        assert_eq!(rows[1].price, 5.3);
        assert_eq!(rows[2].title, "Bremsscheiben");
        assert_eq!(rows[2].tier, "ab 100 kg:");
        assert_eq!(rows[2].price, 0.19);
        assert!(
            skips
                .iter()
                .any(|s| s.contains("Schlamm") && s.contains("0,00")),
            "{skips:?}"
        );
        assert!(
            skips
                .iter()
                .any(|s| s.contains("Inconell") && s.contains("kein Preis")),
            "{skips:?}"
        );
        assert!(
            skips.iter().any(|s| s.contains("höhere Staffeln")),
            "{skips:?}"
        );
    }

    #[test]
    fn foreign_tier_skips_loudly() {
        let html = FIXTURE.replace("ab 1 kg:", "pro Sack:");
        let (rows, skips) = parse(&html).expect("parses");
        // Alu blank loses its kept tier (falls through: higher tiers are
        // still kg and parse — smallest priced tier wins).
        assert!(
            rows.iter()
                .any(|r| r.title == "Alu Bleche blank" && r.tier == "ab 100 kg:"),
            "{rows:?}"
        );
        assert!(
            skips.iter().any(|s| s.contains("Einheit unverständlich")),
            "{skips:?}"
        );
    }

    #[test]
    fn missing_window_and_empty_cards_error() {
        let err = parse("<html><body><p>Neu hier</p></body></html>").expect_err("no window");
        assert!(err.to_string().contains("Preisbereich"), "{err}");
        let mut html = FIXTURE.to_owned();
        for v in ["1.30", "1.45", "1.65", "5.30", "0.19", "0.00"] {
            html = html.replace(&format!(">{v}<"), "><");
        }
        let err = parse(&html).expect_err("all empty");
        assert!(err.to_string().contains("leer"), "{err}");
    }

    #[test]
    fn impressum_extracts_contact() {
        // Real fragment shape incl. zero-width joiner after Inhaber line.
        let imp = "<h1 class=\"heading-style-h1\">Impressum</h1>\
            <p class=\"text-size-medium\"><strong>Angaben gemäß § 5 TMG<br/><br/></strong>\
            MSG Recycling<br/>Gleichenstraße 36<br/>99867 Gotha, DE<br/><br/>\
            Inhaber: Helge Göbbel<br/>\u{200d}<br/><strong>Kontakt</strong><br/>\
            Telefon: +49 (0) 3621 3394 851<br/>Mail: info@msg-metallrecycling.de<br/></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Gleichenstraße 36");
        assert_eq!(info.postcode, "99867");
        assert_eq!(info.city, "Gotha");
        assert_eq!(info.phone, "+49 (0) 3621 3394 851");
        assert_eq!(info.email, "info@msg-metallrecycling.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(extract_info("<h1>Impressum</h1><p>leer</p>").is_err());
    }

    #[test]
    fn mapping_covers_live_cards() {
        assert_eq!(
            grade_for("Millberry (ab 1 kg:)"),
            Some(("kupfer-millberry", "Millberry"))
        );
        assert_eq!(
            grade_for("Kupfer Lackdraht (ab 1 kg:)"),
            Some(("kupfer-berry", "Lackdraht"))
        );
        assert_eq!(
            grade_for("Kupfer Leitschienen Blank (ab 1 kg:)"),
            Some(("kupfer-gemischt", "Leitschienen Blank"))
        );
        assert_eq!(
            grade_for("Kupfer Schwer (ab 1 kg:)"),
            Some(("kupfer-gemischt", "Schwer"))
        );
        assert_eq!(
            grade_for("Kupfer Leicht (ab 1 kg:)"),
            Some(("kupfer-gemischt", "Leicht"))
        );
        assert_eq!(
            grade_for("Kupfer Späne (ab 1 kg:)"),
            Some(("kupfer-gemischt", "Späne"))
        );
        assert_eq!(grade_for("Alu Kabel (ab 1 kg:)"), Some(("kabel-alu", "")));
        assert_eq!(
            grade_for("Kabel 50% Kupfer (ab 1 kg:)"),
            Some(("kabel-kupfer", "50%"))
        );
        assert_eq!(
            grade_for("Kabel 60% Kupfer (ab 1 kg:)"),
            Some(("kabel-kupfer", "60%"))
        );
        assert_eq!(
            grade_for("Kabel 80% Kupfer (ab 1 kg:)"),
            Some(("kabel-kupfer", "80%"))
        );
        assert_eq!(grade_for("Kabel (ab 1 kg:)"), Some(("kabel-kupfer", "")));
        assert_eq!(
            grade_for("Ms 58 Abfälle (ab 1 kg:)"),
            Some(("messing", "Ms 58 Abfälle"))
        );
        assert_eq!(
            grade_for("Ms Erodierdraht (ab 1 kg:)"),
            Some(("messing", "Ms Erodierdraht"))
        );
        assert_eq!(
            grade_for("Ms leicht (ab 1 kg:)"),
            Some(("messing", "Ms leicht"))
        );
        assert_eq!(
            grade_for("Ms Späne gemischt (ab 1 kg:)"),
            Some(("messing", "Ms Späne gemischt"))
        );
        assert_eq!(
            grade_for("Schwermessing (ab 1 kg:)"),
            Some(("messing", "Schwermessing"))
        );
        assert_eq!(
            grade_for("Wasseruhren (ab 1 kg:)"),
            Some(("messing-leicht", "Wasseruhren"))
        );
        assert_eq!(
            grade_for("Bronze Schrott (ab 1 kg:)"),
            Some(("bronze-rotguss", "Bronze Schrott"))
        );
        assert_eq!(
            grade_for("Bronze Späne (ab 1 kg:)"),
            Some(("bronze-rotguss", "Bronze Späne"))
        );
        assert_eq!(
            grade_for("Rotguss Späne (ab 1 kg:)"),
            Some(("bronze-rotguss", "Rotguss Späne"))
        );
        assert_eq!(
            grade_for("Rotguss Stücke (ab 1 kg:)"),
            Some(("bronze-rotguss", "Rotguss Stücke"))
        );
        assert_eq!(
            grade_for("Alu Bleche blank (ab 1 kg:)"),
            Some(("aluminium-blech", "Bleche blank"))
        );
        assert_eq!(
            grade_for("Alu Bleche Farbe (ab 1 kg:)"),
            Some(("aluminium-blech", "Bleche Farbe"))
        );
        assert_eq!(
            grade_for("Alu Geschirr (ab 1 kg:)"),
            Some(("aluminium-blech", "Geschirr"))
        );
        assert_eq!(
            grade_for("Alu Felgen (ab 1 kg:)"),
            Some(("aluminium-guss", "Felgen"))
        );
        assert_eq!(
            grade_for("Alu Guss (ab 1 kg:)"),
            Some(("aluminium-guss", "Guss"))
        );
        assert_eq!(
            grade_for("Alu Profile blank (ab 1 kg:)"),
            Some(("aluminium-profile", "Profile blank"))
        );
        assert_eq!(
            grade_for("Alu Profile iso (ab 1 kg:)"),
            Some(("aluminium-profile", "Profile iso"))
        );
        assert_eq!(
            grade_for("Alu Späne (ab 1 kg:)"),
            Some(("aluminium-gemischt", "Späne"))
        );
        assert_eq!(grade_for("Altblei (ab 1 kg:)"), Some(("blei", "")));
        assert_eq!(
            grade_for("V2A Edelstahl (ab 1 kg:)"),
            Some(("edelstahl-v2a", ""))
        );
        assert_eq!(
            grade_for("V2A Edelstahl Späne (ab 1 kg:)"),
            Some(("edelstahl-v2a", "Späne"))
        );
        assert_eq!(
            grade_for("V4A Edelstahl (ab 1 kg:)"),
            Some(("edelstahl-v4a", ""))
        );
        assert_eq!(
            grade_for("V4A Edelstahl Späne (ab 1 kg:)"),
            Some(("edelstahl-v4a", "Späne"))
        );
        assert_eq!(
            grade_for("Elektromotoren (ab 1 kg:)"),
            Some(("elektromotoren", ""))
        );
        assert_eq!(
            grade_for("Hartmetall Bohrer/Fräser/Plättchen (ab 1 kg:)"),
            Some(("hartmetall", "Bohrer/Fräser/Plättchen"))
        );
        assert_eq!(
            grade_for("Hartmetall Stückschrott (ab 1 kg:)"),
            Some(("hartmetall", "Stückschrott"))
        );
        assert_eq!(
            grade_for("Bremsscheiben (ab 100 kg:)"),
            Some(("eisenschrott-gussbruch", "Bremsscheiben"))
        );
        assert_eq!(
            grade_for("Gusseisen (ab 100 kg:)"),
            Some(("eisenschrott-gussbruch", "Gusseisen"))
        );
        assert_eq!(
            grade_for("Mischschrott (ab 100 kg:)"),
            Some(("mischschrott", ""))
        );
        assert_eq!(
            grade_for("Mischschrott schwer (ab 100 kg:)"),
            Some(("mischschrott", "schwer"))
        );
        assert_eq!(
            grade_for("Schredderschrott (ab 100 kg:)"),
            Some(("stahlschrott-shredder", ""))
        );
        assert_eq!(
            grade_for("Zinn Geschirr (ab 1 kg:)"),
            Some(("zinn-geschirr", "Geschirr"))
        );
        assert_eq!(
            grade_for("Zinkblech neu und alt (ab 1 kg:)"),
            Some(("zink", "Blech neu und alt"))
        );
        assert_eq!(grade_for("Zinkguss (ab 1 kg:)"), Some(("zink", "Guss")));
        // Lauter Skip: Mischmetall, Doppel-Material, kein Katalogmaterial.
        assert_eq!(grade_for("Alu Cu Kühler (ab 1 kg:)"), None);
        assert_eq!(grade_for("Stahl- und Gussspäne (ab 100 kg:)"), None);
        assert_eq!(grade_for("Messer versilbert (ab 1 kg:)"), None);
        assert_eq!(grade_for("Silberbesteck 800 (ab 1 kg:)"), None);
        assert_eq!(
            grade_for("Versilbertes Besteck gestempelt 60/40/20 (ab 1 kg:)"),
            None
        );
        assert_eq!(
            grade_for("Versilbertes Besteck gestempelt 80/90/100 (ab 1 kg:)"),
            None
        );
        assert_eq!(grade_for("Inconell (ab 1 kg:)"), None);
        assert_eq!(grade_for("Nickel (ab 1 kg:)"), None);
        assert_eq!(grade_for("Indium (ab 1 kg:)"), None);
        assert_eq!(grade_for("Wolfram (ab 1 kg:)"), None);
        assert_eq!(grade_for("Gold und silberhaltige Abfälle (ab 1 kg:)"), None);
        assert_eq!(grade_for("Palladium (ab 1 kg:)"), None);
    }
}
