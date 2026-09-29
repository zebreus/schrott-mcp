//! Bio Goldankauf (Patrick Müller, Pforzheim 75177): acceptance-only
//! handler. REVIEW DECISION (WALLED for prices): the Echtzeit-Goldrechner
//! on /goldrechner/ is JS-only — the static HTML carries only the metal
//! dropdown (`select#metall`: Gold, Silber, Platin, Palladium,
//! "Zahngold mind. 60%", "Versilbertes Besteck", "Messer versilbert",
//! "Zinn mind. 95%") plus a `goldrechner_ajax.legierungen` fineness list
//! ([999.9, 916, 900, …], no prices); the calculator window between
//! `class="goldrechner"` and `id="ergebnis"` holds zero price numbers and
//! `#ergebnis` is empty until `calculator_1.7.js` POSTs the weight to
//! admin-ajax.php (action `goldrechner_berechnen`) and renders the payout.
//! No ajax-internals are chased (fragile, Hensel precedent —
//! `fetch_text` is GET-only anyway). What the page statically proves is
//! the acceptance list (the 8 dropdown options): Au/Ag/Pt/Pd map to
//! `gold`/`silber`/`platin`/`palladium` (EUR/g catalog unit, no prices
//! taken), Zahngold/Zinn map with their stated minimum fineness as
//! conditions; versilbertes Besteck/Messer are plated cutlery with no
//! catalog material and skip loudly (geld_fuer_gold precedent — never
//! crammed into solid-silver `silber`). No quote date is stated for the
//! acceptance list (the JS "Preise aktualisiert am" timestamp belongs to
//! prices we do not take) → `published_at` stays `None`. Zero prices with
//! resolved acceptances is normal operation, not a canary trip.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "bw-pforzheim-75177-bio-goldankauf-patrick-muller";
/// Bespoke, live-verified impressum URL (site footer's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "https://bio-goldankauf.de/impressum/";

pub const URL: &str = "https://bio-goldankauf.de/goldrechner/";

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
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
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

/// Explicit label → acceptances. Plated cutlery (`versilbert`) has no
/// catalog material and stays `None` — the arm runs FIRST because
/// "versilbert" contains "silber" and a generic silber arm would catch
/// it otherwise. "Zahngold" contains "gold", so it likewise precedes the
/// gold arm. Threshold qualifiers ("mind. 60 %"/"mind. 95 %") ride in
/// `conditions` only when the label actually states them — never
/// defaulted.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    if l.contains("versilbert") {
        None
    } else if l.contains("zahngold") {
        let cond = if l.contains("60") { "mind. 60%" } else { "" };
        Some(vec![("zahngold", cond)])
    } else if l.contains("gold") {
        Some(vec![("gold", "")])
    } else if l.contains("silber") {
        Some(vec![("silber", "")])
    } else if l.contains("platin") {
        Some(vec![("platin", "")])
    } else if l.contains("palladium") {
        Some(vec![("palladium", "")])
    } else if l.contains("zinn") {
        let cond = if l.contains("95") { "mind. 95%" } else { "" };
        Some(vec![("zinn", cond)])
    } else {
        None
    }
}

/// The static metal dropdown between `id="metall"` and its closing
/// `</select>`. Both anchors mandatory; an empty option list is `Err`
/// (a silent empty success would hide a redesign of the calculator).
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    let start = html.find("id=\"metall\"").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Metall-Auswahl fehlt".to_owned(),
    })?;
    let tail = &html[start..];
    let end = tail.find("</select>").ok_or_else(|| IngestError::Parse {
        url: URL.to_owned(),
        detail: "Metall-Auswahl unvollständig".to_owned(),
    })?;
    let window = &tail[..end];
    let frag = Html::parse_fragment(&format!("<select>{window}</select>"));
    let opt_sel = Selector::parse("option").expect("valid selector");
    let mut labels = Vec::new();
    for el in frag.select(&opt_sel) {
        let t: String = el
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if t.is_empty() || t.len() > 120 {
            continue;
        }
        if !labels.contains(&t) {
            labels.push(t);
        }
    }
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Metall-Auswahl leer".to_owned(),
        });
    }
    Ok(labels)
}

/// Bespoke contact extraction for THIS impressum only: `<h1>Impressum</h1>`
/// plus the address `<p>` ("Patrick Müller<br>Salierstraße 29A<br>75177
/// Pforzheim", anchored on "Salierstra" so ß/ss variants still match)
/// and the contact `<p>` ("Telefon: …<br>E-Mail: …"). Missing anchors →
/// loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    if !doc
        .select(&h1)
        .any(|h| h.text().collect::<String>().trim() == "Impressum")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let p = Selector::parse("p").expect("valid selector");
    let addr_p = doc.select(&p).find(|el| {
        let t: String = el.text().collect();
        t.contains("Salierstra")
    });
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let addr_lines: Vec<String> = addr_p
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in addr_lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| format!("{a} {b}"));
                if k > 0 {
                    street = addr_lines[k - 1].clone();
                }
                break;
            }
        }
    }
    let cont_p = doc.select(&p).find(|el| {
        let t: String = el.text().collect();
        t.contains("Telefon:")
    });
    let (mut phone, mut email) = (String::new(), String::new());
    if let Some(el) = cont_p {
        for part in el.inner_html().split("<br") {
            let t = strip_fragment(part);
            if let Some(v) = t.strip_prefix("Telefon:") {
                phone = v.trim().to_owned();
            } else if let Some(v) = t.strip_prefix("E-Mail:") {
                email = v.trim().to_owned();
            } else if let Some(v) = t.strip_prefix("E-mail:") {
                email = v.trim().to_owned();
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

/// Strip tags from a `<br>`-split fragment. Fragments start either with
/// real text ("Telefon: …") or with tag/attribute remnants (" />",
/// `class="…" />`): remnants carry '=' before any '<' (or start with
/// '/'), real label text doesn't — dropping blindly up to the first
/// '>' would eat the label itself.
fn strip_fragment(s: &str) -> String {
    let mut s = s;
    if let Some(i) = s.find('<') {
        if s[..i].contains('=') {
            s = &s[s.find('>').map(|j| j + 1).unwrap_or(s.len())..];
        }
    }
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
    out.replace("&nbsp;", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_start_matches(|c| c == '/' || c == '>')
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::{grade_for, parse};

    // Real shape of the live calculator dropdown (option values + display
    // texts), trimmed to nothing — all 8 live options are present.
    const FIXTURE: &str = "<div class=\"goldrechner\"><label>Metall</label>\
        <select id=\"metall\">\
        <option value=\"Gold\">Gold</option>\
        <option value=\"Silber\">Silber</option>\
        <option value=\"Platin\">Platin</option>\
        <option value=\"Palladium\">Palladium</option>\
        <option value=\"Zahngold\">Zahngold mind. 60%</option>\
        <option value=\"Besteck\">Versilbertes Besteck</option>\
        <option value=\"Messer\">Messer versilbert</option>\
        <option value=\"Zinn\">Zinn mind. 95%</option>\
        </select><label>Legierung</label><select id=\"legierung\"></select></div>";

    #[test]
    fn dropdown_options_parse() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(
            labels,
            vec![
                "Gold",
                "Silber",
                "Platin",
                "Palladium",
                "Zahngold mind. 60%",
                "Versilbertes Besteck",
                "Messer versilbert",
                "Zinn mind. 95%",
            ]
        );
        assert!(parse("<div>Redesign ohne Rechner</div>").is_err());
        assert!(parse("<select id=\"metall\"><option>Gold</option>").is_err());
    }

    #[test]
    fn metals_map_and_plated_skips() {
        assert_eq!(grade_for("Gold"), Some(vec![("gold", "")]));
        assert_eq!(grade_for("Silber"), Some(vec![("silber", "")]));
        assert_eq!(grade_for("Platin"), Some(vec![("platin", "")]));
        assert_eq!(grade_for("Palladium"), Some(vec![("palladium", "")]));
        assert_eq!(
            grade_for("Zahngold mind. 60%"),
            Some(vec![("zahngold", "mind. 60%")])
        );
        assert_eq!(
            grade_for("Zinn mind. 95%"),
            Some(vec![("zinn", "mind. 95%")])
        );
        // "Zahngold" contains "gold" — must not land on `gold`.
        assert_eq!(grade_for("Zahngold mind. 60%").unwrap()[0].0, "zahngold");
        // Plated cutlery is not solid silver — "versilbert" contains
        // "silber", so the plated arm must run first. Never crammed.
        assert_eq!(grade_for("Versilbertes Besteck"), None);
        assert_eq!(grade_for("Messer versilbert"), None);
        // Thresholds are read off the label, never defaulted.
        assert_eq!(grade_for("Zahngold"), Some(vec![("zahngold", "")]));
        assert_eq!(grade_for("Zinn"), Some(vec![("zinn", "")]));
    }

    #[test]
    fn impressum_blocks() {
        let imp = "<h1>Impressum</h1>\
            <h2>Angaben gemäß § 5 TMG</h2>\
            <p>Patrick Müller<br>Salierstraße 29A<br>75177 Pforzheim</p>\
            <p>Telefon: 07231 607 799 0<br>E-Mail: info@bio-goldankauf.de</p>\
            <p>Inhaber: Patrick Müller<br>Umsatzsteuer-ID: DE 188321796</p>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Salierstraße 29A");
        assert_eq!(info.postcode, "75177");
        assert_eq!(info.city, "Pforzheim");
        assert_eq!(info.phone, "07231 607 799 0");
        assert_eq!(info.email, "info@bio-goldankauf.de");
        assert!(super::extract_info("<h1>Neu hier</h1>").is_err());
        assert!(super::extract_info("<h1>Impressum</h1><p>Kein Kontakt</p>").is_err());
    }
}
