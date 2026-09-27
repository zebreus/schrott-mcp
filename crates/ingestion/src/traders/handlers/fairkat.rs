//! Fair-Kat (Kenan Özdin, Ottersberg; site kaufe-katalysatoren.de —
//! fair-kat.de redirects there): catalyst buyer with a Kat-DB (9755
//! types). The homepage slider (`div.flexslider > ul.slides > li >
//! div.slide.promote`) shows live per-type prices: `div.model`
//! ("Fahrzeug<br>Kat-Nr") plus `div.price` ("361,6 €"). No "bis zu" —
//! these are exact list prices, one per catalyst type, so the Kat-Nr
//! becomes the variant: 30 types must never collapse onto one current
//! price. DPF entries are particulate filters, not catalysts → loud
//! skip with a new-material proposal, never crammed into `katalysatoren`.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "ni-ottersberg-28870-fair-kat-kenan-ozdin";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.kaufe-katalysatoren.de/impressum.php";

pub const URL: &str = "https://www.kaufe-katalysatoren.de/";

pub fn handler() -> Handler {
    Handler { slug: SLUG, url: URL, schedule: Schedule::every_6h(), scrape: |c| Box::pin(scrape(c)) }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (vehicle, katnr, price, unit) in rows {
        let label = if katnr.is_empty() {
            vehicle.clone()
        } else {
            format!("{vehicle} / {katnr}")
        };
        let Some(material) = grade_for(&vehicle) else {
            skipped_labels.push(format!(
                "{label} (DPF, kein Katalysator — Vorschlag: partikelfilter-dpf)"
            ));
            continue;
        };
        if katnr.is_empty() {
            skipped_labels.push(format!("{label} (Kat-Nr fehlt, keine Variante)"));
            continue;
        }
        // Exact per-type list prices at full confidence; the Kat-Nr is
        // the variant. Leaked once per card per run (bounded by the
        // slider size) so types never collapse onto one current price.
        let variant: &'static str = Box::leak(katnr.into_boxed_str());
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

/// Explicit product → material. DPFs (diesel particulate filters) are a
/// different product class from catalysts → None (loud skip with a
/// new-material proposal in the caller). Everything else on the slider
/// — Pkw, Lkw and Motorrad catalysts — is a catalyst.
fn grade_for(vehicle: &str) -> Option<&'static str> {
    if vehicle.to_lowercase().starts_with("dpf") {
        None
    } else {
        Some("katalysatoren")
    }
}

/// Bespoke: the slider quotes bare "€" figures. These are Kat-DB
/// Stückpreise per type (site: "Kat-DB mit Preisen"; settlement "ab 25
/// Stück"; values 23–1548 € fit piece prices) — documented default
/// EUR/Stk. Anything explicit (`/` or "pro") skips loudly instead.
fn unit_of(t: &str) -> Option<&'static str> {
    let l = t.to_lowercase();
    if (l.contains('€') || l.contains("eur")) && !l.contains('/') && !l.contains("pro") {
        Some("EUR/Stk")
    } else {
        None
    }
}

fn parse(html: &str) -> Result<(Vec<(String, String, f64, &'static str)>, Vec<String>), IngestError> {
    // Window: the slider list only. The login box ("Kundenbereich")
    // and footer follow `</ul>` and must never contribute labels.
    let start = html.find("<ul class=\"slides\">").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Kat-Slider fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("</ul>").unwrap_or(tail.len());
    let window = &tail[..end];
    // Content elements only (`div.slide.promote` cards); scripts and
    // the login form stay out.
    let doc = Html::parse_fragment(window);
    let card = Selector::parse("div.slide.promote").expect("valid selector");
    let model = Selector::parse("div.model").expect("valid selector");
    let price = Selector::parse("div.price").expect("valid selector");
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for el in doc.select(&card) {
        let Some(m) = el.select(&model).next() else { continue };
        let Some(p) = el.select(&price).next() else { continue };
        // "Fahrzeug<br>Kat-Nr": split on <br first (guide gotcha —
        // tag remnants parse as text otherwise), then strip tags.
        let parts: Vec<String> = m
            .inner_html()
            .split("<br")
            .map(strip_fragment)
            .filter(|s| !s.is_empty())
            .collect();
        if parts.is_empty() {
            continue;
        }
        let vehicle = parts[0].clone();
        let katnr = parts.last().cloned().unwrap_or_default();
        let katnr = if parts.len() > 1 { katnr } else { String::new() };
        let raw = p.text().collect::<String>();
        let raw = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        let Some(value) = parse_eur(&raw) else {
            skipped.push(format!("{} (Preis unverständlich: {raw})", parts.join(" / ")));
            continue;
        };
        match unit_of(&raw) {
            Some(unit) => rows.push((vehicle, katnr, value, unit)),
            None => skipped.push(format!("{} (Einheit unverständlich: {raw})", parts.join(" / "))),
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Kat-Preise".to_owned(),
        });
    }
    Ok((rows, skipped))
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

/// Bespoke contact extraction for THIS impressum only: the
/// `<h2>Verantwortlich für den Inhalt:</h2>` heading with the
/// following `<p>` ("Kenan Özdin<br>Breslauer Str. 13<br>28870
/// Ottersberg<br>kenan@fair-kat.de"). No phone on the page — empty
/// stays empty, never guessed. Missing anchors → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let mut addr_html: Option<String> = None;
    for el in doc.select(&h2) {
        if el.text().collect::<String>().trim() == "Verantwortlich für den Inhalt:" {
            let mut sib = el.next_siblings();
            addr_html = sib
                .find_map(|n| scraper::ElementRef::wrap(n).filter(|e| e.value().name() == "p"))
                .map(|p| p.inner_html());
            break;
        }
    }
    let Some(html) = addr_html else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let lines: Vec<String> =
        html.split("<br").map(strip_fragment).filter(|s| !s.is_empty()).collect();
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    let mut email = String::new();
    for line in &lines {
        if line.contains("Str.") && street.is_empty() {
            street = line.clone();
        }
        if line.contains('@') && email.is_empty() {
            email = line.clone();
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        for (k, tok) in toks.iter().enumerate() {
            if tok.len() == 5 && tok.chars().all(|c| c.is_ascii_digit()) {
                if let Some(ci) = toks.get(k + 1) {
                    if ci.chars().next().is_some_and(|c| c.is_uppercase()) {
                        postcode = (*tok).to_owned();
                        city = (*ci).to_owned();
                    }
                }
            }
        }
    }
    if street.is_empty() && postcode.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo { street, postcode, city, phone: String::new(), email })
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    // Real structure, shortened: flexslider with model/price cards
    // (incl. a DPF card and a thousands-grouped Lkw price), terminated
    // by </ul> before the login box.
    const FIXTURE: &str = "<div class=\"flexslider\"><ul class=\"slides\">\
        <li><div class=\"slide promote\">\
        <img src=\"/_thumbnails_/3893_1_20210111_153351.jpg\" class=\"anpic\" name=\"Foto_1\">\
        <div><div class=\"model\">Mercedes SLK 200 R170<br>KT0138</div>\
        <div class=\"price\">361,6 €</div></div></div></li>\
        <li><div class=\"slide promote\"><div>\
        <div class=\"model\">DPF IVECO Daily<br>5802025884</div>\
        <div class=\"price\">443,4 €</div></div></div></li>\
        <li><div class=\"slide promote\"><div>\
        <div class=\"model\">LKW IVECO<br>5802073552</div>\
        <div class=\"price\">1.548,1 €</div></div></div></li>\
        <li><div class=\"slide promote\"><div>\
        <div class=\"model\">Motorrad BMW K50 / K51 …<br>8556446</div>\
        <div class=\"price\">23,5 €</div></div></div></li>\
        </ul></div><div class=\"box__login\"><div class=\"box__head\">Kundenbereich</div></div>";

    #[test]
    fn slider_cards() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty(), "{skips:?}");
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].0, "Mercedes SLK 200 R170");
        assert_eq!(rows[0].1, "KT0138");
        assert_eq!(rows[0].2, 361.6);
        assert_eq!(rows[0].3, "EUR/Stk");
        // Thousands rule: "1.548,1" is fifteen-forty-eight, not 1.5.
        assert_eq!(rows[2].2, 1548.1);
        assert_eq!(rows[3].2, 23.5);
        // DPFs are not catalysts; everything else (inkl. Lkw/Motorrad) is.
        assert_eq!(grade_for("DPF IVECO Daily"), None);
        assert_eq!(grade_for("DPF Hyundai Santa"), None);
        assert_eq!(grade_for("Mercedes SLK 200 R170"), Some("katalysatoren"));
        assert_eq!(grade_for("LKW IVECO"), Some("katalysatoren"));
        assert_eq!(grade_for("Motorrad BMW K50 / K51"), Some("katalysatoren"));
        // Only this page's spellings: bare € → Stk default, explicit → skip.
        assert_eq!(unit_of("361,6 €"), Some("EUR/Stk"));
        assert_eq!(unit_of("5 €/kg"), None);
        assert_eq!(unit_of("10 Euro pro Stück"), None);
    }

    #[test]
    fn window_and_anchors_hold() {
        assert!(parse("<p>Kein Slider</p>").is_err());
        assert!(parse("<ul class=\"slides\"></ul>").is_err());
        assert!(extract_info("<p>Neu hier</p>").is_err());
    }

    #[test]
    fn impressum_block() {
        let imp = "<h2>Impressum / Datenschutz</h2><h2>&nbsp;</h2>\
            <h2>Verantwortlich für den Inhalt:</h2>\
            <p>Kenan Özdin<br>Breslauer Str. 13<br>28870 Ottersberg<br>kenan@fair-kat.de</p>\
            <p>Finanzamt Verden<br>USTID: DE217963452</p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Breslauer Str. 13");
        assert_eq!(info.postcode, "28870");
        assert_eq!(info.city, "Ottersberg");
        assert!(info.phone.is_empty(), "no phone on the page");
        assert_eq!(info.email, "kenan@fair-kat.de");
    }
}
