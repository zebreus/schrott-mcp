//! AllgäuZinn (Kempten): Zinn-Ankauf mit einem statischen "bis zu"-Preis
//! im Fließtext — keine Tabelle, kein Datum. Die Seite nennt genau einen
//! Angebotspreis für 90–95 % Zinnanteil (28,00 €/kg), einmal schlicht
//! ("zahlt AllgäuZinn aktuell 28,00 € pro kg"), einmal als Obergrenze
//! ("zahlen wir aktuell bis zu 28,00 € pro kg"). Beide Sätze mergen auf
//! EINE Zeile — kupferhelden-Präzedenz: price = price_max = beworben,
//! confidence 0.5, price_kind "upto". Der JS-Preisrechner dahinter ist
//! WALLED (rechnet clientseitig, keine statischen Werte) und die
//! Schwester-Seiten (Kupfer/Messing/Kabel/Alu) nennen nur "Preisanfrage"
//! ohne Preise — beides wird bewusst NICHT erfunden, nur der statische
//! Satz zählt.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "by-kempten-allgauzinn";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.allgaeuzinn.de/impressum";

pub const URL: &str = "https://www.allgaeuzinn.de/zinn-verkaufen";

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
    let (rows, skipped_labels) = parse(&html)?;
    // Ein Angebotspreis je (Material, Sorte): "bis zu" trägt die
    // Obergrenze als price_max bei 0.5, exakte Listenpreise bei 1.0.
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, upto) in rows {
        let (price_max, confidence, price_kind) = if upto {
            (Some(price), Some(0.5), "upto")
        } else {
            (None, Some(1.0), "exact")
        };
        prices.push(ScrapedPrice {
            material: "zinn",
            variant: "90-95%",
            price,
            currency: "EUR",
            unit: "EUR/kg",
            price_kind,
            price_min: None,
            price_max,
            confidence,
            label,
        });
    }
    // Doppelblöcke nach dem Mapping mergen: derselbe Wert steht zweimal
    // im Text (einmal mit, einmal ohne "bis zu") — die schärfere
    // Unsicherheit (upto) gewinnt, sonst kollabieren zwei Kinds auf einen
    // willkürlichen Current-Preis.
    let merged = merge_rows(prices);
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        prices: merged,
        acceptances: vec![],
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        // Nur "aktuell" auf der Seite; "© 2025" ist Copyright, kein
        // Preisdatum — observed_at reicht.
        published_at: None,
    })
}

/// Satzweise Suche im Fenster zwischen Preis-Heading und
/// Regionalteil. Gibt (Satz, Preis, upto) zurück plus laut gezählte
/// Skips.
fn parse(html: &str) -> Result<(Vec<(String, f64, bool)>, Vec<String>), IngestError> {
    // Nur Content-Elemente: <title>/<meta>/JSON-LD enthalten ähnliche
    // Worte ("Zinnpreis pro kg"), Skripte sowieso — Fenster auf echten
    // Textknoten, nie auf der Ganzseite.
    let doc = Html::parse_document(html);
    let sel = Selector::parse("h1, h2, h3, h4, p, li").expect("valid selector");
    let texts: Vec<String> = doc
        .select(&sel)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|t| !t.is_empty())
        .collect();
    let start = texts
        .iter()
        .position(|t| t.contains("Zinnpreis pro kg"))
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock fehlt".to_owned(),
        })?;
    let tail = &texts[start..];
    // Ende NACH dem zweiten Preissatz ("Altes Zinn … bis zu 28,00 €"):
    // die Annahmeliste steht ZWISCHEN den Sätzen und taugt nicht als
    // Ende — erst "Zinnankauf in Kempten" schließt den Preisbereich ab
    // (danach nur Abholgebiet/Footer ohne €).
    let end = tail
        .iter()
        .position(|t| t.contains("Zinnankauf in Kempten"))
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock-Ende fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for t in window {
        for raw in t.split(['.', '!', '?']) {
            let s = raw.split_whitespace().collect::<Vec<_>>().join(" ");
            if !s.contains('€') {
                continue;
            }
            let Some(price) = price_near_euro(&s) else {
                skips.push(format!("{s} (kein Zahlenwert)"));
                continue;
            };
            if price == 0.0 {
                skips.push(format!("{s} (0,00 € – kein Angebot)"));
                continue;
            }
            // Bespoke: diese Seite zitiert nur pro kg ("28,00 € pro
            // kg"). Ohne "kg" laut skippen — ein Stück- oder
            // Versandpreis als Kilo wäre Größenordnungen daneben.
            if !s.to_lowercase().contains("kg") {
                skips.push(format!("{s} (Einheit unverständlich)"));
                continue;
            }
            // Einzige offerierte Sorte dieser Seite: 90–95 % Anteil.
            // Sätze ohne Sortenbezug (Versand, Telefon) sind kein Angebot.
            if !s.contains("90") {
                skips.push(format!("{s} (Sorte unverständlich)"));
                continue;
            }
            let upto = s.to_lowercase().contains("bis zu");
            rows.push((s, price, upto));
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Zinnpreise".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Der Angebotspreis steht direkt VOR dem € ("28,00 € pro kg"), davor
/// aber die Sorte ("90–95 %"): `parse_eur` auf dem ganzen Satz griffe
/// die 90. Deshalb nur das enge Fenster vor dem € parsen — "€ 28"-Stil
/// gäbe hier bewusst None statt einer fremden Zahl.
fn price_near_euro(s: &str) -> Option<f64> {
    let i = s.find('€')?;
    let before = &s[..i];
    let tail: String = before
        .chars()
        .rev()
        .take(16)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    parse_eur(&tail)
}

/// Doppelblock-Merge nach dem Mapping: gleiche (Material, Sorte, Preis)
/// aus zwei Textstellen → eine Zeile, "bis zu" gewinnt über exakt.
fn merge_rows(prices: Vec<ScrapedPrice>) -> Vec<ScrapedPrice> {
    let mut merged: Vec<ScrapedPrice> = Vec::new();
    for p in prices {
        if let Some(m) = merged
            .iter_mut()
            .find(|m| m.price.to_bits() == p.price.to_bits())
        {
            if p.price_kind == "upto" && m.price_kind != "upto" {
                m.price_kind = p.price_kind;
                m.price_max = p.price_max;
                m.confidence = p.confidence;
                m.label = p.label;
            }
        } else {
            merged.push(p);
        }
    }
    merged
}

/// Bespoke contact extraction for THIS impressum only: Wix-rich-text
/// `<p>`-Blöcke mit `<br>`-Zeilen — erst der TMG-Block ("Nico Tornes /
/// AllgäuZinn Kempten / Hieberstraße 18 / 87435 Kempten / Deutschland"),
/// dann der "Kontakt:"-Block ("Telefon:" / "E-Mail:"-Zeilen).
/// `<h1>Impressum` ist Pflicht-Anker; fehlende Blöcke → lauter Error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    if !doc
        .select(&h1)
        .any(|el| el.text().collect::<String>().contains("Impressum"))
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let p = Selector::parse("p").expect("valid selector");
    // Adressblock: der <p>, dessen <br>-Zeilen PLZ+Stadt UND den
    // Firmennamen tragen (Footer-Kontaktspalten stehen in eigenen <p>s
    // und fallen hier raus).
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    for el in doc.select(&p) {
        let lines = br_lines(&el.inner_html());
        if !lines
            .iter()
            .any(|l| l.contains("AllgäuZinn") || l.contains("Tornes"))
        {
            continue;
        }
        for (k, line) in lines.iter().enumerate() {
            let mut it = line.split_whitespace();
            if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
                if pc.len() == 5
                    && pc.chars().all(|c| c.is_ascii_digit())
                    && ci.chars().next().is_some_and(|c| c.is_uppercase())
                {
                    postcode = pc.to_owned();
                    city = (*ci).to_owned();
                    if k > 0 {
                        street = lines[k - 1].clone();
                    }
                    break;
                }
            }
        }
        if !postcode.is_empty() {
            break;
        }
    }
    // "Kontakt:"-Block → beschriftete Folgezeilen im selben <p>.
    let mut phone = String::new();
    let mut email = String::new();
    let mut found_kontakt = false;
    for el in doc.select(&p) {
        let lines = br_lines(&el.inner_html());
        if !lines.iter().any(|l| l == "Kontakt:") {
            continue;
        }
        found_kontakt = true;
        for line in &lines {
            if let Some(v) = line.strip_prefix("Telefon:") {
                phone = v.trim().to_owned();
            } else if let Some(v) = line.strip_prefix("E-Mail:") {
                email = v.trim().to_owned();
            }
        }
        break;
    }
    if !found_kontakt || (street.is_empty() && phone.is_empty() && email.is_empty()) {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
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

/// `<br>`-Zeilen eines `<p>`-innerHTML. Fragments beginnen mit einem
/// Tag-Rest (` class="…"`) — erst alles bis zum ersten `>` verwerfen,
/// sonst parsen Attribute als Text. MSS: html5ever läuft hier nicht,
/// Entities der Seite werden per decode_entities aufgelöst.
fn br_lines(inner: &str) -> Vec<String> {
    inner
        .split("<br")
        .map(strip_fragment)
        .map(|s| decode_entities(&s))
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| !s.is_empty())
        .collect()
}

/// Strip tags from a fragment (gleicher Schnitt wie kupferhelden:
/// erst Tag-Rest bis `>`, dann tagfreier Rest).
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
    out
}

/// Minimaler Entity-Decoder für genau diese Seite (Wix: &szlig; &auml;
/// &euro; &ndash; …) plus numerische Referenzen. Unbekanntes bleibt roh
/// und fällt in Tests auf — nie still raten.
fn decode_entities(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        let tail = &rest[i..];
        let Some(semi) = tail.find(';') else {
            out.push_str(tail);
            break;
        };
        if semi > 10 {
            out.push('&');
            rest = &tail[1..];
            continue;
        }
        let ent = &tail[1..semi];
        if let Some(c) = named_entity(ent) {
            out.push(c);
        } else if let Some(c) = numeric_entity(ent) {
            out.push(c);
        } else {
            out.push_str(&tail[..=semi]);
        }
        rest = &tail[semi + 1..];
    }
    out.push_str(rest);
    out
}

fn named_entity(ent: &str) -> Option<char> {
    match ent {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "nbsp" => Some('\u{a0}'),
        "euro" => Some('€'),
        "ndash" => Some('–'),
        "mdash" => Some('—'),
        "szlig" => Some('ß'),
        "auml" => Some('ä'),
        "ouml" => Some('ö'),
        "uuml" => Some('ü'),
        "Auml" => Some('Ä'),
        "Ouml" => Some('Ö'),
        "Uuml" => Some('Ü'),
        "sect" => Some('§'),
        _ => None,
    }
}

fn numeric_entity(ent: &str) -> Option<char> {
    if let Some(hex) = ent.strip_prefix("#x").or_else(|| ent.strip_prefix("#X")) {
        u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)
    } else if let Some(dec) = ent.strip_prefix('#') {
        dec.parse::<u32>().ok().and_then(char::from_u32)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{br_lines, decode_entities, parse};

    /// Realer HTML-Ausschnitt der Live-Seite (Wix-rich-text, gekürzt —
    /// Struktur, Entities und beide Preis-Sätze in echter Reihenfolge:
    /// Preissatz, Annahmeliste dazwischen, bis-zu-Satz, Regionalteil).
    const FIXTURE: &str = "<div class=\"N8MGzv comp-mu5tg6xj2 wixui-rich-text\">\
        <h2 class=\"font_2 wixui-rich-text__text\">\
        <span class=\"wixui-rich-text__text\">Zinnpreis pro kg &ndash; Was ist mein Zinn wert?</span></h2></div>\
        <div class=\"N8MGzv comp-mu5tg6xl wixui-rich-text\">\
        <p class=\"font_8 wixui-rich-text__text\">\
        <span class=\"wixui-rich-text__text\">Der aktuelle Zinnpreis und damit der Ankaufspreis f&uuml;r Zinn \
        richtet sich vor allem nach Zinnanteil und Reinheitsgrad des Materials. F&uuml;r Zinn mit einem Anteil \
        von 90&ndash;95 % zahlt Allg&auml;uZinn aktuell 28,00 &euro; pro kg. Damit erhalten Sie direkt eine \
        Orientierung zum </span></p>\
        <p class=\"font_8 wixui-rich-text__text\">\
        <span class=\"wixui-rich-text__text\">95% Zinn Preis pro kg beim Ankauf. Der genaue Zinnwert h&auml;ngt \
        au&szlig;erdem von der Art des Zinnartikels, seinem Gewicht, der Zusammensetzung und der aktuellen \
        Marktlage ab.</span></p></div>\
        <div class=\"N8MGzv comp-mpy48isu wixui-rich-text\">\
        <h3 class=\"font_3 wixui-rich-text__text\">Diese Zinnartikel kaufen wir an&nbsp;</h3></div>\
        <h3>Zinngeschirr</h3><h3>Zinnfiguren</h3>\
        <p>Bemalte Zinnfiguren, Unbemalte Zinnfiguren, Milit&auml;rfiguren,</p>\
        <h3>Altes Zinn und Zinn aus Nachl&auml;ssen verkaufen</h3>\
        <p class=\"font_8 wixui-rich-text__text\">Ob ein einzelner Zinngegenstand oder mehrere Kilogramm Zinn \
        vorhanden sind: Entscheidend f&uuml;r den Ankauf sind unter anderem Materialart, Gewicht und Zinnanteil. \
        F&uuml;r Zinn mit einem Anteil von 90&ndash;95 % zahlen wir aktuell bis zu 28,00 &euro; pro kg.&nbsp;</p>\
        <h2>Zinnankauf in Kempten und im Allg&auml;u</h2>";

    const IMP_FIXTURE: &str = "<h1 class=\"font_0 wixui-rich-text__text\">\
        <span class=\"color_11 wixui-rich-text__text\">Impressum</span></h1>\
        <div class=\"N8MGzv comp-kgovomwt5 wixui-rich-text\">\
        <p class=\"font_8 wixui-rich-text__text\">\
        <span class=\"wixui-rich-text__text\">Angaben gem&auml;&szlig; &sect; 5 TMG:</span></p>\
        <p class=\"font_8 wixui-rich-text__text\">\
        <span class=\"wixui-rich-text__text\">Nico Tornes<br class=\"wixui-rich-text__text\">\n\
        Allg&auml;uZinn Kempten<br class=\"wixui-rich-text__text\">\n\
        Hieberstra&szlig;e 18<br class=\"wixui-rich-text__text\">\n\
        87435 Kempten<br class=\"wixui-rich-text__text\">\nDeutschland</span></p>\
        <p class=\"font_8 wixui-rich-text__text\">\
        <span class=\"wixui-rich-text__text\">Kontakt:<br class=\"wixui-rich-text__text\">\n\
        Telefon: 0159 / 01271643<br class=\"wixui-rich-text__text\">\n\
        E-Mail: kontakt@allgaeuzinn.de<br class=\"wixui-rich-text__text\">\n\
        Website: www.allgaeuzinn.de</span></p></div>";

    #[test]
    fn bis_zu_satz_trifft() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // Beide Preis-Sätze meinen dieselbe Obergrenze → zwei Treffer,
        // der Merge in scrape() macht daraus eine upto-Zeile.
        assert_eq!(rows.len(), 2);
        assert!(skips.is_empty());
        assert_eq!(rows[0].1, 28.0);
        assert!(!rows[0].2, "erster Satz ohne bis-zu");
        assert_eq!(rows[1].1, 28.0);
        assert!(rows[1].2, "zweiter Satz mit bis-zu");
        assert!(rows[1].0.contains("90–95"), "Sortenbezug im Label");
    }

    #[test]
    fn skips_und_fehler() {
        // Gemischt: ein gültiger Satz plus je ein 0,00-€- und
        // Fremdeinheiten-Satz → 1 Zeile + 2 begründete Skips.
        let mixed = "<h2>Zinnpreis pro kg XX</h2>\
            <p>Für Zinn mit einem Anteil von 90–95 % zahlen wir aktuell bis zu 28,00 € pro kg. \
            Für Zinn mit 90 % Anteil aktuell 0,00 € pro kg. \
            Für Zinn mit 90 % Anteil aktuell 5 € pro Sack.</p>\
            <h2>Zinnankauf in Kempten</h2>";
        let (rows, skips) = parse(mixed).expect("parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].1, 28.0);
        assert!(rows[0].2);
        assert_eq!(skips.len(), 2, "{skips:?}");
        assert!(skips.iter().any(|s| s.contains("0,00")), "{skips:?}");
        assert!(skips.iter().any(|s| s.contains("Einheit")), "{skips:?}");
        // Bleibt nach Skips gar nichts, ist das ein Fehler
        // (0 Zeilen = Err), kein leerer Erfolg.
        let zero =
            "<h2>Zinnpreis pro kg XX</h2><p>Für Zinn mit 90 % Anteil aktuell 0,00 € pro kg.</p>\
            <h2>Zinnankauf in Kempten</h2>";
        assert!(parse(zero).is_err());
        // Fehlende Anker und leere Fenster sind Fehler, kein Erfolg.
        assert!(parse("<p>Preis 28,00 € pro kg für 90 % Zinn.</p>").is_err());
        assert!(parse(
            "<h2>Zinnpreis pro kg XX</h2><p>Nur Text ohne Euro.</p>\
            <h2>Zinnankauf in Kempten</h2>"
        )
        .is_err());
    }

    #[test]
    fn upto_merge_gewinnt() {
        use super::super::super::ScrapedPrice;
        let row = |kind: &'static str, label: &str| ScrapedPrice {
            material: "zinn",
            variant: "90-95%",
            price: 28.0,
            currency: "EUR",
            unit: "EUR/kg",
            price_kind: kind,
            price_min: None,
            price_max: if kind == "upto" { Some(28.0) } else { None },
            confidence: Some(if kind == "upto" { 0.5 } else { 1.0 }),
            label: label.to_owned(),
        };
        // Live-Reihenfolge: exakt zuerst, bis-zu danach → eine upto-Zeile.
        let merged = super::merge_rows(vec![
            row("exact", "zahlt 28,00"),
            row("upto", "bis zu 28,00"),
        ]);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].price_kind, "upto");
        assert_eq!(merged[0].price_max, Some(28.0));
        assert_eq!(merged[0].confidence, Some(0.5));
        assert!(merged[0].label.contains("bis zu"));
        // Umgekehrte Reihenfolge → ebenfalls upto.
        let merged = super::merge_rows(vec![
            row("upto", "bis zu 28,00"),
            row("exact", "zahlt 28,00"),
        ]);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].price_kind, "upto");
    }

    #[test]
    fn impressum_bloecke() {
        let info = super::extract_info(IMP_FIXTURE).expect("parses");
        assert_eq!(info.street, "Hieberstraße 18");
        assert_eq!(info.postcode, "87435");
        assert_eq!(info.city, "Kempten");
        assert_eq!(info.phone, "0159 / 01271643");
        assert_eq!(info.email, "kontakt@allgaeuzinn.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
        assert!(super::extract_info("<h1>Impressum</h1><p>Ohne Blöcke</p>").is_err());
    }

    #[test]
    fn entities_und_br_zeilen() {
        assert_eq!(decode_entities("Hieberstra&szlig;e 18"), "Hieberstraße 18");
        assert_eq!(decode_entities("28,00 &euro; pro kg"), "28,00 € pro kg");
        assert_eq!(decode_entities("90&ndash;95 %"), "90–95 %");
        let lines = br_lines("Nico Tornes<br class=\"wixui-rich-text__text\">\nAllg&auml;uZinn");
        assert_eq!(lines, vec!["Nico Tornes", "AllgäuZinn"]);
    }
}
