//! Hammer – László Hunyak e.K. (Leipzig): acceptance-only (esh/quell-Muster).
//! Die Preis-URL (`/preise/`) trägt KEINE maschinenlesbaren Preise
//! (verifiziert 28.09.2026 per curl: 145 KB HTML, sichtbarer Text ohne eine
//! einzige Preis-Ziffer — die Tafel sind 3 Foto-JPGs, kein OCR in der
//! Pipeline). Daher füllt dieser Handler wie `esh`/`quell` nur
//! `trader_materials` aus der belegten Annahmeliste (`/dienstleistungen/`:
//! "Wir nehmen auch Eisen- und Buntmetallschrott (z. B.: Aluminium, Kupfer,
//! Zink, rostfreie Materialien, Blei usw.) und Buntmetallhaltige Abfälle
//! (z. B.: isolierte Kupferkabel, Elektromotoren) an. Wir kaufen alle
//! Altmetalle an.") plus Kontakt. Papier ist auf der Homepage als Ankauf
//! belegt, hat aber keinen Katalogeintrag (hofmann-Präzedenz) und bleibt
//! außerhalb dieser Quelle. Kein Datum auf der Seite → `published_at: None`.
//! Kein separates Impressum (`/impressum/` leitet auf `/` um, verifiziert) —
//! Kontakt kommt von `/kontaktdaten/` (Adresse- und E-Mail-Listenpunkte)
//! plus der Footer-Tel-Zeile derselben Seite.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "sn-leipzig-hammer-laszlo-hunyak";
/// Bespoke, live-verified acceptance page: the only machine-readable
/// purchase list on the site (photo prices on `/preise/` are not parsable).
pub const URL: &str = "https://hammerschrott.de/dienstleistungen/";
/// Bespoke, live-verified contact page. There is no standalone impressum
/// (`/impressum/` redirects to `/`); the footer on every page carries the
/// HRA/Steuer/Tel block, the address + mail live in the icon-list here.
pub const KONTAKT_URL: &str = "https://hammerschrott.de/kontaktdaten/";

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
    let labels = parse(&html)?;
    let mut acceptances = Vec::with_capacity(labels.len());
    let mut skipped_labels = Vec::new();
    for label in labels {
        match grade_for(&label) {
            Some(materials) => {
                for (material, conditions) in materials {
                    acceptances.push(ScrapedAcceptance {
                        material,
                        conditions: conditions.to_owned(),
                        label: label.clone(),
                    });
                }
            }
            None => skipped_labels.push(label),
        }
    }
    // Kontakt failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, kontakt_html) = fetch_text(client, KONTAKT_URL).await?;
    let trader_info = extract_info(&kontakt_html)?;
    Ok(HandlerOutcome {
        prices: vec![],
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

/// Explicit label → acceptances. Generics map to generics ("Kupfer" →
/// kupfer-gemischt, never a millberry grade); "rostfreie Materialien" without
/// a V2A/V4A split maps to mixed (hofmann precedent). Unknown future items
/// skip loudly.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("kupferkabel") || (l.contains("kabel") && l.contains("kupfer")) {
        Some(vec![("kabel-kupfer", "")])
    } else if l.contains("elektromotor") {
        Some(vec![("elektromotoren", "")])
    } else if l.contains("rostfrei") {
        Some(vec![("edelstahl-gemischt", "")])
    } else if l.contains("aluminium") {
        Some(vec![("aluminium-gemischt", "")])
    } else if l.contains("kupfer") {
        Some(vec![("kupfer-gemischt", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("blei") {
        Some(vec![("blei", "")])
    } else if l.contains("eisenschrott") || l == "eisen" {
        // The "Eisen-" half of "Eisen- und Buntmetallschrott" (the
        // Buntmetall half is covered by the explicit grades above).
        Some(vec![("mischschrott", "")])
    } else {
        None
    }
}

fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    // Purchase-intent proof: without this sentence the page could turn
    // into pure Entsorgung prose and we would fabricate acceptances.
    let start = html
        .find("Wir nehmen auch Eisen- und Buntmetallschrott")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Wir kaufen alle Altmetalle an.")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankauf-Beleg fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    let mut out = vec!["Eisenschrott".to_owned()];
    // The two "(z. B.: …)" groups hold the explicit grades.
    let mut rest = window;
    while let Some(i) = rest.find("(z. B.:") {
        let tail2 = &rest[i + "(z. B.:".len()..];
        let Some(j) = tail2.find(')') else {
            break;
        };
        for item in tail2[..j].split(',') {
            let t = item
                .replace("usw.", "")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            if !t.is_empty() && t.len() <= 60 {
                out.push(t);
            }
        }
        rest = &tail2[j + 1..];
    }
    if out.len() <= 1 {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    Ok(out)
}

/// Bespoke contact extraction for THIS kontakt page only: the
/// `elementor-icon-list-item` rows ("Adresse:" with street + PLZ city,
/// "E-mail:") plus the footer "Tel:" line. Element-scoped reads (scraper
/// text() glues neighbour nodes without spaces). Missing "Adresse:" means
/// the page changed shape → loud error, never a guessed fallback.
fn extract_info(html: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(html);
    let li = Selector::parse("li").expect("valid selector");
    if !doc
        .select(&li)
        .any(|el| el.text().collect::<String>().contains("Adresse:"))
    {
        return Err(IngestError::Parse {
            url: KONTAKT_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    }
    let row = |marker: &str| {
        doc.select(&li)
            .map(|el| el.text().collect::<Vec<_>>().join(" "))
            .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
            .find(|t| t.contains(marker))
            .map(|t| {
                t.split_once(marker)
                    .map(|(_, v)| v.trim().to_owned())
                    .unwrap_or_default()
            })
    };
    // "Anton-Zickmantel-Straße 41, 04249 Leipzig".
    let addr = row("Adresse:").unwrap_or_default();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    if let Some((s, rest)) = addr.split_once(',') {
        street = s.trim().to_owned();
        let mut it = rest.split_whitespace();
        if let Some(pc) = it.next() {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.collect::<Vec<_>>().join(" ");
            }
        }
    }
    let email = row("E-mail:")
        .or_else(|| row("E-Mail:"))
        .unwrap_or_default()
        .split_whitespace()
        .find(|t| t.contains('@'))
        .unwrap_or_default()
        .to_owned();
    // Footer "Tel:+49 173 419 1866" line (newline-terminated, no glue risk).
    let body: String = doc
        .root_element()
        .text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let phone = body
        .find("Tel:")
        .map(|i| {
            body[i + "Tel:".len()..]
                .split_whitespace()
                .take_while(|t| {
                    t.chars()
                        .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: KONTAKT_URL.to_owned(),
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

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    // Real excerpt of the live page (28.09.2026), verbatim sentence shape.
    const FIXTURE: &str = "<div class=\"elementor-widget-container\">\
        <p><span lang=\"de-DE\">Wir nehmen auch Eisen- und Buntmetallschrott \
        (z. B.: Aluminium, Kupfer, Zink, rostfreie Materialien, Blei usw.) und \
        Buntmetallhaltige Abfälle (z. B.: isolierte Kupferkabel, Elektromotoren) \
        an. Wir kaufen alle Altmetalle an. </span></p></div>";

    #[test]
    fn sentence_parses_to_grades() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(
            labels,
            vec![
                "Eisenschrott",
                "Aluminium",
                "Kupfer",
                "Zink",
                "rostfreie Materialien",
                "Blei",
                "isolierte Kupferkabel",
                "Elektromotoren",
            ]
        );
        assert!(parse("<p>Kein Beleg hier</p>").is_err());
        // Purchase proof gone (pure Entsorgung prose) → loud error.
        let no_proof = FIXTURE.replacen("Wir kaufen alle Altmetalle an.", "Wir entsorgen.", 1);
        assert!(parse(&no_proof).is_err());
        // Empty example groups → loud error, not silent success.
        let empty = FIXTURE
            .replacen(
                "(z. B.: Aluminium, Kupfer, Zink, rostfreie Materialien, Blei usw.)",
                "",
                1,
            )
            .replacen("(z. B.: isolierte Kupferkabel, Elektromotoren)", "", 1);
        assert!(parse(&empty).is_err());
    }

    #[test]
    fn mapping_covers_live_grades() {
        assert_eq!(grade_for("Eisenschrott"), Some(vec![("mischschrott", "")]));
        assert_eq!(
            grade_for("Aluminium"),
            Some(vec![("aluminium-gemischt", "")])
        );
        assert_eq!(grade_for("Kupfer"), Some(vec![("kupfer-gemischt", "")]));
        assert_eq!(grade_for("Zink"), Some(vec![("zink", "")]));
        assert_eq!(
            grade_for("rostfreie Materialien"),
            Some(vec![("edelstahl-gemischt", "")])
        );
        assert_eq!(grade_for("Blei"), Some(vec![("blei", "")]));
        // Specific-before-generic: kabel wins over the kupfer arm.
        assert_eq!(
            grade_for("isolierte Kupferkabel"),
            Some(vec![("kabel-kupfer", "")])
        );
        assert_eq!(
            grade_for("Elektromotoren"),
            Some(vec![("elektromotoren", "")])
        );
        // Paper has no catalog entry; unknowns skip loudly.
        assert_eq!(grade_for("Papier"), None);
        assert_eq!(grade_for("Irgendwas Neues"), None);
    }

    #[test]
    fn kontakt_extracts_address_mail_and_footer_phone() {
        // Real fragment shapes of the live kontakt page (28.09.2026).
        let html = "<ul class=\"elementor-icon-list-items\">\
            <li class=\"elementor-icon-list-item\">\
            <span class=\"elementor-icon-list-text\"><b>Adresse:</b> \
            Anton-Zickmantel-Straße 41, 04249 Leipzig </span></li>\
            <li class=\"elementor-icon-list-item\">\
            <span class=\"elementor-icon-list-text\"><b>E-mail:</b> \
            hammer2017@web.de</span></li></ul>\
            <div class=\"footeradatok\" style=\"padding: 10px;\"><center>\
            László Hunyak e.K HRA 19802<br>\n04249 Leipzig Anton-Zickmatel-Str. 41<br>\n\
            Stnr.232/234/06947<br>\nUst-IdNr.DE311496102<br>\n\
            Verantwortlich: Laszlo Hunyak<br>\nAufsichtsbehörde: Finanzamt Leipzig I<br>\n\
            Tel:+49 173 419 1866\n</center></div>";
        let info = extract_info(html).expect("parses");
        assert_eq!(info.street, "Anton-Zickmantel-Straße 41");
        assert_eq!(info.postcode, "04249");
        assert_eq!(info.city, "Leipzig");
        assert_eq!(info.phone, "+49 173 419 1866");
        assert_eq!(info.email, "hammer2017@web.de");
        assert!(extract_info("<p>Neu hier</p>").is_err());
    }
}
