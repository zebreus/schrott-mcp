//! Kulisch & Co. / KRP Kabel Recycling Potsdam: per-kg E-Schrott prices in
//! headed `div.row` cards (one `<u>` label plus a "Preis pro Kg: … Euro"
//! line each), windowed between `<h1>Elektrorecycling</h1>` and
//! `id="content_footer"` so footer/sidebar text can never pair into phantom
//! prices. Live 28.09.2026: 19 cards — 18 numeric prices plus 2 "auf
//! Anfrage" Keramik-CPU cards (skipped loudly); the PC-Stecker card carries
//! a second inline price ("… werden mit 0,35 Euro pro Kg vergütet" for
//! gemischte PC-Stecker). The shop splits some prices across `<strong>`
//! nodes ("7" + ",50"), hence text is joined before parsing.
//!
//! Leiterplatten and Steckkarten map to `platinen` (Sorte variants keep the
//! grades apart); CPUs, RAM, HDDs, Netzteile, Laufwerke, Handys,
//! Smartphones, Stecker and Komplett-PCs have no catalog material and skip
//! loudly (same gaps as Koppe Strausberg: cpu/ram/e-schrott-geraete/handy
//! would be separate catalog steps, never crammed into `platinen`). The
//! page carries no date anywhere (live checked) → `published_at` is None.
//! Contact enrichment via the PortUNA impressum (`<h4>Herausgeber</h4>` +
//! `address.imprint-contact-address` + `dl.imprint-contact-data`); the
//! e-mail is JS-obfuscated (`emaillink('ecycling','kulischundco','de','')`
//! with an `ecycling(at)kulischundco.de` noscript fallback).

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "bb-potsdam-kulisch-co-fahrzeug-handels-und-verwertu";
/// Bespoke, live-verified impressum URL (the site's own footer link).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.kulischundco.de/impressum/index.php";

pub const URL: &str = "https://www.kulischundco.de/seite/402791/elektronikschrott.html";

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

/// Explicit label → (material, variant) mapping, specific-before-generic.
/// Anything unlisted returns None (loud skip at the call site).
///
/// Catalog-gap proposals (do NOT cram):
/// - "Arbeitsspeicher mit Gold-/Silberkante" → new `ram` material.
/// - "Plastik CPU …" / "Keramik CPU …" → new `cpu` material.
/// - "Festplatten / HDD", "Laufwerke / LW", "Netzteile … / NT±",
///   "Computer und Laptops komplett" → new `e-schrott-geraete` material?
/// - "Handy" / "Smartphone" → new `handy` material?
/// - "Computer-Stecker / PC-Stecker" (0,85 vergoldet, 0,35 gemischt) →
///   no connector material; not `platinen` (Stecker sind keine Platinen).
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("leiterplatten") {
        if l.contains("1a") {
            Some(("platinen", "Sorte 1a"))
        } else if l.contains("1b") {
            Some(("platinen", "Sorte 1b"))
        } else if l.contains("2a") {
            if l.contains("sauber") {
                Some(("platinen", "Sorte 2a sauber"))
            } else {
                Some(("platinen", "Sorte 2a"))
            }
        } else if l.contains("sorte 3") {
            Some(("platinen", "Sorte 3"))
        } else {
            Some(("platinen", ""))
        }
    } else if l.contains("steckkarte") {
        Some(("platinen", "Steckkarte"))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS PortUNA impressum only: the
/// `<h4>Herausgeber</h4>` anchor is mandatory, the street/PLZ/city come
/// from the `<br>` lines of `address.imprint-contact-address`
/// ("Zum Heizwerk 16" / "14478 Potsdam"), Telefon from the labeled
/// `dl.imprint-contact-data` row, and the e-mail from the JS
/// `emaillink('ecycling','kulischundco','de','')` call (noscript fallback:
/// `ecycling(at)kulischundco.de`). Missing anchors mean the page changed
/// shape → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h4 = Selector::parse("h4").expect("valid selector");
    let anchor = doc
        .select(&h4)
        .find(|h| h.text().collect::<String>().trim() == "Herausgeber");
    let Some(_) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Herausgeber-Block fehlt".to_owned(),
        });
    };
    // Address lines: <p> by <p>, split on <br> inside each (raw splitting
    // would glue separate <p> elements, scraper text() would glue
    // "Zum Heizwerk 16" and "14478" into fewer tokens — so line-wise).
    let addr_sel = Selector::parse("address.imprint-contact-address p").expect("valid selector");
    let mut lines: Vec<String> = Vec::new();
    for el in doc.select(&addr_sel) {
        for part in el.inner_html().split("<br") {
            let t = strip_fragment(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
    // "Zum Heizwerk 16" is the line before the PLZ line ("14478 Potsdam").
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5
                && pc.chars().all(|c| c.is_ascii_digit())
                && ci.chars().next().is_some_and(|c| c.is_uppercase())
            {
                postcode = pc.to_owned();
                city = ci.trim_matches(',').to_owned();
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                break;
            }
        }
    }
    // Labeled contact rows, paired by document order.
    let dt_sel = Selector::parse("dl.imprint-contact-data dt").expect("valid selector");
    let dd_sel = Selector::parse("dl.imprint-contact-data dd").expect("valid selector");
    let dts: Vec<String> = doc
        .select(&dt_sel)
        .map(|e| e.text().collect::<String>())
        .collect();
    let dds: Vec<String> = doc
        .select(&dd_sel)
        .map(|e| e.text().collect::<Vec<_>>().join(""))
        .collect();
    let mut phone = String::new();
    for (t, d) in dts.iter().zip(dds.iter()) {
        if t.trim().trim_end_matches(':') == "Telefon" && phone.is_empty() {
            phone = d.split_whitespace().collect::<Vec<_>>().join(" ");
        }
    }
    let email = email_from_page(imp);
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

/// E-mail for THIS impressum: the JS `emaillink('user','domain','tld','')`
/// call first, the `(at)` noscript text as tolerated fallback (with test).
fn email_from_page(imp: &str) -> String {
    if let Some(i) = imp.find("emaillink('") {
        let rest = &imp[i + "emaillink('".len()..];
        let q: Vec<&str> = rest.split('\'').collect();
        if q.len() >= 5
            && !q[0].is_empty()
            && !q[2].is_empty()
            && !q[4].is_empty()
            && q[4].chars().all(|c| c.is_ascii_alphabetic())
        {
            return format!("{}@{}.{}", q[0], q[2], q[4]);
        }
    }
    if imp.contains("(at)") {
        let flat = imp.replace("(at)", "@");
        return email_token(&flat);
    }
    String::new()
}

/// First email address in the text: expand from the '@' over email
/// characters (glued neighbours defeat token splitting). Cut at the
/// domain end so trailing prose never sticks.
fn email_token(r: &str) -> String {
    let Some(at) = r.find('@') else {
        return String::new();
    };
    let b = r.as_bytes();
    let is_email = |c: u8| c.is_ascii_alphanumeric() || b".-_+@".contains(&c);
    let mut s = at;
    while s > 0 && is_email(b[s - 1]) {
        s -= 1;
    }
    let mut e = at + 1;
    while e < b.len() && is_email(b[e]) {
        e += 1;
    }
    let cand = &r[s..e];
    for suffix in [".de", ".com", ".net", ".org", ".eu", ".info", ".biz"] {
        if let Some(p) = cand.rfind(suffix) {
            let cut = cand[..p + suffix.len()].to_owned();
            if cut.contains('@') && !cut.starts_with('@') {
                return cut;
            }
        }
    }
    String::new()
}

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant (`/>`) — drop everything up to the first '>' first, or the
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
    // Window: the price cards only — footer/sidebar text must never pair a
    // stray € with a label into a phantom price.
    let start = html
        .find("<h1>Elektrorecycling</h1>")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Elektrorecycling-Block fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("content_footer")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Elektrorecycling-Block fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    // Only content elements, never scripts/styles: one card per div.row,
    // each carrying exactly one <u> heading.
    let row_sel = Selector::parse("div.row").expect("valid selector");
    let head_sel = Selector::parse("u").expect("valid selector");
    let mut rows: Vec<(String, f64, &'static str)> = Vec::new();
    let mut skips = Vec::new();
    for row in doc.select(&row_sel) {
        let Some(head) = row.select(&head_sel).next() else {
            continue; // layout row without a price heading.
        };
        let label = head.text().collect::<String>().replace(['\u{a0}'], " ");
        let label = label.split_whitespace().collect::<Vec<_>>().join(" ");
        if label.is_empty() || label.len() > 120 {
            continue; // €-header/prose, never a label.
        }
        // join("") on purpose: live markup splits prices across <strong>
        // nodes ("7" + ",50 Euro") which belong together.
        let text = row
            .text()
            .collect::<Vec<_>>()
            .join("")
            .replace(['\u{a0}'], " ");
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        let lower = text.to_lowercase();
        let Some(pos) = lower.find("preis pro ") else {
            skips.push(format!("{label} (Preiszeile fehlt)"));
            continue;
        };
        let after = &text[pos + "preis pro ".len()..];
        // The unit rides on the marker itself ("Preis pro Kg"): take the
        // token right after it — only kg/t exist here, anything else
        // skips loudly (a per-tonne price recorded as per-kg would be a
        // 1000x error).
        let unit_tok: String = after
            .chars()
            .take_while(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        let Some(unit) = unit_of(&unit_tok) else {
            skips.push(format!("{label} (Einheit unverständlich: {unit_tok})"));
            continue;
        };
        if after.to_lowercase().contains("auf anfrage") {
            skips.push(format!("{label} (kein Preis: auf Anfrage)"));
            continue;
        }
        let Some(price) = parse_eur(after) else {
            skips.push(format!("{label} (Preis unverständlich: {})", after.trim()));
            continue;
        };
        if price == 0.0 {
            skips.push(format!("{label} (Preis 0,00 wird nicht übernommen)"));
            continue;
        }
        push_dedup(&mut rows, (label.clone(), price, unit));
        // Second inline price inside the same card ("… werden mit 0,35
        // Euro pro Kg vergütet"): its own row, same unit.
        if let Some(sub) = sub_price(&lower, price) {
            let sub_label = if label.to_lowercase().contains("stecker")
                && !label.to_lowercase().contains("steckkarte")
            {
                "gemischte PC-Stecker".to_owned()
            } else {
                format!("{label} (vergütet)")
            };
            push_dedup(&mut rows, (sub_label, sub, unit));
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste leer".to_owned(),
        });
    }
    // No page date anywhere (live checked) → None (observed_at = age).
    Ok((None, rows, skips))
}

/// A second € number in the "vergütet" sentence of the same card
/// (live: gemischte PC-Stecker 0,35 next to the card's main 0,85).
fn sub_price(lower_text: &str, main: f64) -> Option<f64> {
    let pos = lower_text.find("vergütet")?;
    let before = &lower_text[..pos];
    let start = before.rfind(['.', '!', ':']).map_or(0, |i| i + 1);
    let price = parse_eur(&before[start..])?;
    if price > 0.0 && (price - main).abs() > 1e-9 {
        Some(price)
    } else {
        None
    }
}

/// Dedup repeat blocks by (label, price): the same card rendered twice
/// must not double the row.
fn push_dedup(rows: &mut Vec<(String, f64, &'static str)>, row: (String, f64, &'static str)) {
    if !rows
        .iter()
        .any(|(l, p, _)| *l == row.0 && (*p - row.1).abs() < 1e-9)
    {
        rows.push(row);
    }
}

/// Bespoke unit matcher for THIS page's "Preis pro X" marker (live: "Kg").
/// Only kg/t exist here — anything else skips loudly at the call site.
fn unit_of(token: &str) -> Option<&'static str> {
    match token {
        "kg" => Some("EUR/kg"),
        "t" | "to" | "tonne" | "tonnen" => Some("EUR/t"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    /// Verbatim page excerpts (split `<strong>` price nodes, `&nbsp;`
    /// entities, `<u>` headings, the fancybox `<script>` after the `<h1>`,
    /// and the `content_footer` terminator kept as-is).
    const FIXTURE: &str = "<h1>Elektrorecycling</h1>\
        <script >$( document ).ready( function(){ portunaHelper.attachFancybox('template_page') });</script>\
        <div class=\"row\"><div class=\"col-xs-12 col-sm-8 col-md-8\">\
        <p class=\"tiny_p\"><span style=\"font-size:14px;\"><strong><u>Leiterplatten Sorte 1a :</u></strong></span></p>\
        <p class=\"tiny_p\">&nbsp;</p>\
        <p class=\"tiny_p\"><span style=\"font-size:16px;\"><strong>Preis pro Kg:&nbsp; 7</strong></span>\
        <span style=\"color:#d35400;font-size:16px;\"><strong>,50</strong></span>\
        <span style=\"font-size:16px;\"><strong> Euro</strong></span></p></div></div>\
        <div class=\"row\"><div class=\"col-xs-12 col-sm-8 col-md-8\">\
        <p class=\"tiny_p\"><span style=\"font-size:14px;\"><strong><u>Steckkarten :</u></strong></span></p>\
        <p class=\"tiny_p\"><span style=\"font-size:16px;\"><strong>Preis pro Kg:&nbsp;10</strong></span>\
        <span style=\"color:#c0392b;font-size:16px;\"><strong>,20</strong></span>\
        <span style=\"font-size:16px;\"><strong> Euro</strong></span></p></div></div>\
        <div class=\"row\"><div class=\"col-xs-12 col-sm-8 col-md-8\">\
        <p class=\"tiny_p\"><span style=\"font-size:14px;\"><strong><u>Computer-Stecker / PC-Stecker :</u></strong></span></p>\
        <p class=\"tiny_p\"><span style=\"font-size:16px;\"><strong>Preis pro Kg:&nbsp; </strong></span>\
        <span style=\"color:#c0392b;font-size:16px;\"><strong>0,85</strong></span>\
        <span style=\"font-size:16px;\"><strong> Euro&nbsp;</strong></span></p>\
        <p class=\"tiny_p\">Achtung: USB-Stecker, SATA-Stecker, Netzwerk-Stecker usw.</p>\
        <p class=\"tiny_p\">&nbsp; oder gemischte PC-Stecker</p>\
        <p class=\"tiny_p\">&nbsp; werden mit <span style=\"color:#c0392b;\"><strong>0,35 Euro</strong></span> pro Kg vergütet</p></div></div>\
        <div class=\"row\"><div class=\"col-xs-12 col-sm-8 col-md-8\">\
        <p class=\"tiny_p\"><span style=\"font-size:14px;\"><strong><u>Arbeitsspeicher mit Goldkante :</u></strong></span></p>\
        <p class=\"tiny_p\"><span style=\"font-size:16px;\"><strong>Preis pro Kg:&nbsp; </strong></span>\
        <span style=\"color:#c0392b;font-size:16px;\"><strong>36,50</strong></span>\
        <span style=\"font-size:16px;\"><strong> Euro</strong></span></p></div></div>\
        <div class=\"row\"><div class=\"col-xs-12 col-sm-8 col-md-8\">\
        <p class=\"tiny_p\"><span style=\"font-size:14px;\"><strong><u>Keramik CPU Intel / AMD :</u></strong></span></p>\
        <p class=\"tiny_p\"><span style=\"font-size:16px;\"><strong>Preis pro Kg:&nbsp; auf Anfrage</strong></span></p></div></div>\
        <div class=\"row\"><div class=\"col-xs-12\">Layout ohne Preiskopf, Tel.: (0331) 8712772</div></div>\
        </div><div class=\"cleaner\"></div><div id=\"content_footer\" class=\"no-print\">Fuss</div>";

    #[test]
    fn cards_split_prices_and_sub_price_parse() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at, None);
        // Split <strong> nodes rejoin: "7"+",50" → 7.50, "10"+",20" → 10.20.
        assert!(rows.iter().any(|(l, p, u)| l == "Leiterplatten Sorte 1a :"
            && (*p - 7.5).abs() < 1e-9
            && *u == "EUR/kg"));
        assert!(rows
            .iter()
            .any(|(l, p, _)| l == "Steckkarten :" && (*p - 10.2).abs() < 1e-9));
        assert!(rows
            .iter()
            .any(|(l, p, _)| l == "Computer-Stecker / PC-Stecker :" && (*p - 0.85).abs() < 1e-9));
        // Inline second price becomes its own row.
        assert!(rows
            .iter()
            .any(|(l, p, _)| l == "gemischte PC-Stecker" && (*p - 0.35).abs() < 1e-9));
        assert!(rows
            .iter()
            .any(|(l, p, _)| l == "Arbeitsspeicher mit Goldkante :" && (*p - 36.5).abs() < 1e-9));
        assert_eq!(rows.len(), 5);
        // "auf Anfrage" skips loudly; the layout row and the script stay silent.
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Keramik CPU Intel / AMD") && skips[0].contains("auf Anfrage"));
    }

    #[test]
    fn anchors_zero_and_unit_fail_loudly() {
        assert!(parse("<p>Kein Fenster hier</p>").is_err());
        // Window without any priced card errors, it never succeeds empty.
        let html = "<h1>Elektrorecycling</h1><div class=\"row\"><div><p><u>Leer :</u></p>\
            <p>Preis pro Kg:&nbsp; auf Anfrage</p></div></div><div id=\"content_footer\">x</div>";
        assert!(parse(html).is_err());
        // 0,00 prices are skipped, never recorded.
        let html = FIXTURE.replacen("36,50", "0,00", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 4);
        assert!(skips
            .iter()
            .any(|s| s.contains("Goldkante") && s.contains("0,00")));
        // Unknown unit: skipped loudly, valid rows survive.
        let html = FIXTURE.replacen("Preis pro Kg", "Preis pro Sack", 1);
        let (_, rows, skips) = parse(&html).expect("parses");
        assert_eq!(rows.len(), 4);
        assert!(skips
            .iter()
            .any(|s| s.contains("1a") && s.contains("unverständlich")));
        assert!(rows.iter().any(|(l, _, _)| l == "gemischte PC-Stecker"));
        // Every row unparseable: loud error, not silent success.
        let html = FIXTURE.replace("Preis pro Kg", "Preis pro Sack");
        let err = parse(&html).expect_err("empty table errors");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn mapping_covers_all_live_labels() {
        assert_eq!(
            grade_for("Leiterplatten Sorte 1a :"),
            Some(("platinen", "Sorte 1a"))
        );
        assert_eq!(
            grade_for("Leiterplatten Sorte 1b :"),
            Some(("platinen", "Sorte 1b"))
        );
        assert_eq!(
            grade_for("Leiterplatten Sorte 2a sauber :"),
            Some(("platinen", "Sorte 2a sauber"))
        );
        assert_eq!(
            grade_for("Leiterplatten Sorte 3 :"),
            Some(("platinen", "Sorte 3"))
        );
        assert_eq!(grade_for("Steckkarten :"), Some(("platinen", "Steckkarte")));
        // No catalog material → loud skips (cpu/ram/e-schrott-geraete/handy
        // would be separate catalog steps).
        for label in [
            "Handy :",
            "Smartphone :",
            "Festplatten / HDD :",
            "Netzteile mit Kabel / NT+ :",
            "Netzteile ohne Kabel / NT- :",
            "Laufwerke / LW :",
            "Computer-Stecker / PC-Stecker :",
            "gemischte PC-Stecker",
            "Arbeitsspeicher mit Goldkante :",
            "Plastik CPU :",
            "Plastik CPU mit Kühlplatte :",
            "Computer und Laptops komplett ( unberaubt ) :",
            "Arbeitsspeicher mit Silberkante :",
            "Keramik CPU mit Goldecap :",
            "Keramik CPU Intel / AMD :",
        ] {
            assert_eq!(grade_for(label), None, "{label}");
        }
    }

    #[test]
    fn impressum_js_mail_and_noscript_fallback() {
        let imp = "<h4>Herausgeber</h4>\
            <address class=\"imprint-contact-address\">\
            <p class=\"tiny_p\"><strong>Kulisch &amp; Co. GmbH</strong></p>\
            <p class=\"tiny_p\">René Hoffmann</p>\
            <p class=\"tiny_p\">Zum Heizwerk 16<br />14478 Potsdam</p></address>\
            <dl class=\"imprint-contact-data\">\
            <dt class=\"siteinfo-group-1\">Telefon:</dt>\
            <dd class=\"siteinfo-group-1\"><img src=\"x.gif\" alt=\"Telefon\"/> (0331) 8712772</dd>\
            <dt class=\"siteinfo-group-1\">E-Mail:</dt>\
            <dd class=\"siteinfo-group-1\"><img src=\"y.gif\" alt=\"E-Mail\"> \
            <script >emaillink('ecycling','kulischundco','de','');</script>\
            <noscript>ecycling(at)kulischundco.de</noscript></dd></dl>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Zum Heizwerk 16");
        assert_eq!(info.postcode, "14478");
        assert_eq!(info.city, "Potsdam");
        assert_eq!(info.phone, "(0331) 8712772");
        assert_eq!(info.email, "ecycling@kulischundco.de");
        // Noscript-only page still yields the (at)-address.
        let noscript_only = imp.replace("emaillink('ecycling','kulischundco','de','');", "");
        let info = extract_info(&noscript_only).expect("parses");
        assert_eq!(info.email, "ecycling@kulischundco.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
