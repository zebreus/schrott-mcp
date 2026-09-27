//! Papierfritze (Berlin-Biesdorf, Zweitmarke Schrottfritze): exact copper
//! and cable purchase prices as one card per grade
//! (`div.jet-listing-grid__item`: category + quoted title in
//! `.elementor-heading-title`, "ANKAUFSPREIS / KG" header, then BAR and
//! ÜBERWEISUNG price fields like "10,75 € / AB 1 KG").
//!
//! One row per card: the BAR ab-1-kg base price (the page's own Sockel;
//! quantity tiers and Überweisung rows are dropped on purpose — one more
//! variant dimension would burst the catalog). The `/kupferpreis/` listing
//! carries copper/cable cards only; paper, pallets and e-scrap live on
//! other URLs and stay out of this handler (no crawler).
//!
//! "Blei-Kupfer-Kabel" (lead-sheathed cable) has no catalog material and
//! is skipped loudly.

use scraper::{ElementRef, Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "be-marzahn-hellersdorf-papierfritze-schrott-altpapierankauf";
/// Bespoke, live-verified impressum URL (site footer links
/// `/impressum/`). A move fails the step loudly (fix the URL) — never
/// guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.papierfritze.de/impressum/";

pub const URL: &str = "https://www.papierfritze.de/kupferpreis/";

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
            // No catalog material for lead-sheathed cable: keep the
            // quoted price as evidence in the skip, never drop it silently.
            None => skipped_labels.push(format!(
                "{label} ({}, {unit}, kein Katalogmaterial: Blei/Kupfer-Mischung)",
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
        // No page date on the listing (article timestamps are CMS
        // metadata, not price validity) — the observation age stays the
        // provenance.
        published_at: None,
    })
}

fn fmt_eur(price: f64) -> String {
    format!("{price:.2}").replace('.', ",")
}

/// Explicit card title → (material, variant). Specific before generic:
/// "Millberry" must not fall into plain copper, "mit Stecker" must not
/// collapse into the bare cable grade. Two copper mixes share
/// `kupfer-gemischt` and split on the variant ("Raff" vs "Späne").
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("schwer")
        || l.contains("berry")
        || l.contains("candy")
        || l.contains("kerze")
    {
        Some(("kupfer-berry", "Schwer"))
    } else if l.contains("raff") {
        Some(("kupfer-gemischt", "Raff"))
    } else if l.contains("späne") || l.contains("spaene") {
        Some(("kupfer-gemischt", "Späne"))
    } else if l.contains("elektromotor") || l.contains("trafo") || l.contains("vorschalt") {
        Some(("elektromotoren", ""))
    } else if l.contains("kabel") && l.contains("stecker") {
        Some(("kabel-kupfer", "mit Stecker"))
    } else if l.contains("kabel") && l.contains("40") {
        Some(("kabel-kupfer", "40%"))
    } else if l.contains("kabel") && l.contains("50") {
        Some(("kabel-kupfer", "50%"))
    } else if l.contains("kabel") && l.contains("60") {
        Some(("kabel-kupfer", "60%"))
    } else if l.contains("kabel") && l.contains("70") {
        Some(("kabel-kupfer", "70%"))
    } else if l.contains("kabel") && l.contains("80") {
        Some(("kabel-kupfer", "80%"))
    } else if l.contains("kabel") && l.contains("90") {
        Some(("kabel-kupfer", "90%"))
    } else {
        None
    }
}

/// Parse the card listing. Returns (rows, unit_skips); cards whose shape
/// broke (no title, no BAR base price) skip loudly, an empty listing is a
/// loud error, never a silent success.
fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let hit = html
        .find("jet-listing-grid__item")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preiskarten fehlen".to_owned(),
        })?;
    // Back up over the opening tag: cutting inside `<div class="…">`
    // would destroy the first card's element.
    let start = html[..hit].rfind('<').unwrap_or(hit);
    let frag = Html::parse_fragment(&format!("<div>{}</div>", &html[start..]));
    let card_sel = Selector::parse("div.jet-listing-grid__item").expect("valid selector");
    let head_sel = Selector::parse(".elementor-heading-title").expect("valid selector");
    let link_sel = Selector::parse("a").expect("valid selector");
    let field_sel = Selector::parse(".jet-listing-dynamic-field__content").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for card in frag.select(&card_sel) {
        // Title = longest LINKED heading: the short category ("Kupfer")
        // and the quoted grade title both link to the grade page, while
        // "ANKAUFSPREIS / KG" is a bare header (and ties the title length).
        let title = card
            .select(&head_sel)
            .filter(|el| el.select(&link_sel).next().is_some())
            .map(|el| el.text().collect::<String>())
            .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
            .filter(|t| !t.is_empty() && t.len() <= 120)
            .max_by_key(|t| t.len())
            .unwrap_or_default();
        if title.is_empty() {
            skips.push("(Karte ohne Titel, übersprungen)".to_owned());
            continue;
        }
        let card_text: String = card.text().collect();
        // The per-card unit anchor — without it a quote has no scale.
        let Some(unit) = unit_of(&card_text) else {
            skips.push(format!("{title} (Einheit unverständlich)"));
            continue;
        };
        // Base price = first "X € / AB …" field (BAR ab 1 kg block).
        let mut base: Option<f64> = None;
        for field in card.select(&field_sel) {
            let text: String = field.text().collect();
            if text.contains('€') && text.contains("AB") {
                base = parse_eur(&text);
                break;
            }
        }
        let Some(price) = base else {
            skips.push(format!("{title} (kein BAR-Basispreis)"));
            continue;
        };
        rows.push((title, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preiskarten leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit matcher for THESE cards' "ANKAUFSPREIS / KG" header. Only
/// kg exists here — anything else skips loudly at the call site.
fn unit_of(card_text: &str) -> Option<&'static str> {
    let lower = card_text.to_lowercase().replace(' ', "");
    if lower.contains("ankaufspreis/kg") || lower.contains("preis/kg") {
        Some("EUR/kg")
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<h4>Kontakt`
/// block holds the `tel:` phone, and the `<h4>Impressum` block holds
/// "Rustech Recycling GmbH" + street + PLZ city. The mail address is
/// Cloudflare-protected — decoded from its `data-cfemail` (key = first
/// byte, rest XORed). Missing headings → loud error, never a guessed
/// fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h4 = Selector::parse("h4").expect("valid selector");
    let anchors: Vec<String> = doc.select(&h4).map(|h| h.text().collect()).collect();
    if !anchors.iter().any(|h| h.trim() == "Kontakt")
        || !anchors.iter().any(|h| h.trim() == "Impressum")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt/Impressum-Blöcke fehlen".to_owned(),
        });
    }
    // Address lines: <br>-split firm <p> right after the Impressum heading.
    let addr_p = doc
        .select(&h4)
        .find(|h| h.text().collect::<String>().trim() == "Impressum")
        .and_then(|h| {
            h.next_siblings()
                .filter_map(ElementRef::wrap)
                .find(|e| e.value().name() == "p")
        });
    let mut lines = Vec::new();
    if let Some(p) = addr_p {
        for part in p.inner_html().split("<br") {
            let t = strip_tags(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
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
    // Phone rides on the tel: link, email on the Cloudflare attribute.
    let a = Selector::parse("a").expect("valid selector");
    let mut phone = String::new();
    for el in doc.select(&a) {
        if el
            .value()
            .attr("href")
            .is_some_and(|h| h.starts_with("tel:"))
            && phone.is_empty()
        {
            phone = el
                .text()
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
        }
    }
    let cf = Selector::parse("[data-cfemail]").expect("valid selector");
    let email = doc
        .select(&cf)
        .filter_map(|el| el.value().attr("data-cfemail"))
        .find_map(decode_cfemail)
        .unwrap_or_default();
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

/// Cloudflare email protection: first byte is the key, the rest is the
/// address XORed with it. Tolerated variant of THIS page, with test.
fn decode_cfemail(hex: &str) -> Option<String> {
    if hex.len() < 4 || hex.len() % 2 != 0 {
        return None;
    }
    let bytes: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
        .collect::<Option<_>>()?;
    let (key, addr) = bytes.split_first()?;
    addr.iter()
        .map(|b| char::from(b ^ key))
        .collect::<String>()
        .into()
}

/// Strip tags from a fragment (entities are already decoded by html5ever).
fn strip_tags(s: &str) -> String {
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
    use super::{decode_cfemail, extract_info, grade_for, parse};

    // Real shape of the live cards (grid item, two headings, dynamic
    // fields, BAR/ÜBERWEISUNG blocks), trimmed to three cards.
    const FIXTURE: &str = "<div class=\"jet-listing-grid__item jet-listing-dynamic-post-2672\">\
        <div class=\"elementor-heading-title elementor-size-default\"><a>Kupfer</a></div>\
        <div class=\"elementor-heading-title elementor-size-default\"><a>Kupfer-Schrott \"Millberry\"</a></div>\
        <div class=\"elementor-heading-title elementor-size-default\">ANKAUFSPREIS / KG</div>\
        <div class=\"elementor-heading-title elementor-size-default\">BAR</div>\
        <div class=\"jet-listing-dynamic-field__content\">ab 1mm Durchmesser, nicht angelaufen, nicht lackiert</div>\
        <div class=\"jet-listing-dynamic-field__content\">10,75 € / AB 1 KG</div>\
        <div class=\"jet-listing-dynamic-field__content\">10,80 € / AB 200 KG</div>\
        <div class=\"elementor-heading-title elementor-size-default\">ÜBERWEISUNG</div>\
        <div class=\"jet-listing-dynamic-field__content\">10,85 € / AB 1 KG</div></div>\
        <div class=\"jet-listing-grid__item jet-listing-dynamic-post-2682\">\
        <div class=\"elementor-heading-title elementor-size-default\"><a>Kabel</a></div>\
        <div class=\"elementor-heading-title elementor-size-default\"><a>Kupfer-Kabel 40 %</a></div>\
        <div class=\"elementor-heading-title elementor-size-default\">ANKAUFSPREIS / KG</div>\
        <div class=\"elementor-heading-title elementor-size-default\">BAR</div>\
        <div class=\"jet-listing-dynamic-field__content\">Kabelschrott ohne Stecker, Kupferanteil ab 40%</div>\
        <div class=\"jet-listing-dynamic-field__content\">3,45 € / AB 1 KG</div>\
        <div class=\"jet-listing-dynamic-field__content\">3,50 € / AB 200 KG</div></div>\
        <div class=\"jet-listing-grid__item jet-listing-dynamic-post-2659\">\
        <div class=\"elementor-heading-title elementor-size-default\"><a>Blei</a></div>\
        <div class=\"elementor-heading-title elementor-size-default\"><a>Blei-Kupfer-Kabel</a></div>\
        <div class=\"elementor-heading-title elementor-size-default\">ANKAUFSPREIS / KG</div>\
        <div class=\"elementor-heading-title elementor-size-default\">BAR</div>\
        <div class=\"jet-listing-dynamic-field__content\">frei von Öl und anderen Schmierstoffen</div>\
        <div class=\"jet-listing-dynamic-field__content\">0,30 € / AB 1 KG</div></div>";

    #[test]
    fn cards_take_bar_base_price() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 3);
        assert!(skips.is_empty());
        // First €/AB field wins (BAR ab 1 kg), tiers are dropped.
        assert_eq!(
            rows[0],
            ("Kupfer-Schrott \"Millberry\"".to_owned(), 10.75, "EUR/kg")
        );
        assert_eq!(rows[1], ("Kupfer-Kabel 40 %".to_owned(), 3.45, "EUR/kg"));
        assert_eq!(rows[2], ("Blei-Kupfer-Kabel".to_owned(), 0.3, "EUR/kg"));
        assert!(parse("<div>Redesign ohne Karten</div>").is_err());
    }

    #[test]
    fn mapping_splits_sorts_and_skips_lead_cable() {
        assert_eq!(
            grade_for("Kupfer-Schrott \"Millberry\""),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(
            grade_for("Kupfer-Schrott \"Schwer\" (Berry / Candy / Kerze)"),
            Some(("kupfer-berry", "Schwer"))
        );
        assert_eq!(
            grade_for("Kupfer-Schrott \"Raff\""),
            Some(("kupfer-gemischt", "Raff"))
        );
        assert_eq!(
            grade_for("Kupfer-Späne (trocken)"),
            Some(("kupfer-gemischt", "Späne"))
        );
        assert_eq!(
            grade_for("Elektromotoren / Trafos / Vorschaltgeräte"),
            Some(("elektromotoren", ""))
        );
        assert_eq!(
            grade_for("Kupfer-Kabel 40 %"),
            Some(("kabel-kupfer", "40%"))
        );
        assert_eq!(
            grade_for("Kupfer-Kabel 90 %"),
            Some(("kabel-kupfer", "90%"))
        );
        assert_eq!(
            grade_for("Kupfer-Kabel mit Stecker"),
            Some(("kabel-kupfer", "mit Stecker"))
        );
        assert_eq!(grade_for("Blei-Kupfer-Kabel"), None);
    }

    #[test]
    fn impressum_blocks_and_cfemail() {
        // Live cfemail value decodes to the trader's mailbox.
        assert_eq!(
            decode_cfemail("a4cdcac2cbe4d4c5d4cdc1d6c2d6cdd0dec18ac0c1").as_deref(),
            Some("info@papierfritze.de")
        );
        assert_eq!(decode_cfemail("zz"), None);
        let imp = "<h4>Kontakt</h4><p>Telefon: <a href=\"tel:08004020050\">0800 40 200 50</a><br/>\
            Email: <a href=\"/cdn-cgi/l/email-protection#c1\"><span class=\"__cf_email__\" data-cfemail=\"a4cdcac2cbe4d4c5d4cdc1d6c2d6cdd0dec18ac0c1\">[email protected]</span></a></p>\
            <h4>Impressum</h4><p>Rustech Recycling GmbH<br/>Alt-Biesdorf 12<br/>12683 Berlin-Biesdorf</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Alt-Biesdorf 12");
        assert_eq!(info.postcode, "12683");
        assert_eq!(info.city, "Berlin-Biesdorf");
        assert_eq!(info.phone, "0800 40 200 50");
        assert_eq!(info.email, "info@papierfritze.de");
        assert!(extract_info("<h4>Neu hier</h4>").is_err());
    }
}
