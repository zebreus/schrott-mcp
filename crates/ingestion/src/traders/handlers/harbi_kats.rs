//! Harbi Kats Recycling (Ali Harbi, Berlin-Spandau): catalyst buyer whose
//! homepage FAQ ("Welche Art von Produkten kaufen wir an?") names two
//! accepted products — Katalysatoren and Rußpartikelfilter — but quotes no
//! public prices: the Tagespreis-Katalog lives behind the login (`/login`
//! is a bare sign-in form, verified live). So this handler is
//! acceptance-only (esh.rs pattern): `katalysatoren` resolves, Rußpartikel-
//! filter/DPF skips loudly with a new-material proposal (fairkat precedent
//! — a filter is not a catalyst and must never hide under `katalysatoren`).
//! No page date → `published_at` stays `None`.

use scraper::{Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "be-spandau-13587-harbi-kats-recycling-ali-harbi";
/// Bespoke, live-verified impressum URL (site footer's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "https://harbi-kats.de/impressum";

/// Acceptance page = homepage: the FAQ accordion holds the product list.
/// (`/login` is sign-in only, `/about` repeats the same two products in
/// prose — no second list to crawl.)
pub const URL: &str = "https://harbi-kats.de";

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
            None => skipped_labels.push(format!("{label}{}", skip_reason(&label))),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again. The e-mail comes from the already-fetched
    // homepage (own block below) — the impressum carries none (its
    // E-Mail line is commented out live).
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html, &html)?;
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

/// Explicit product → acceptances. Filter arms first so an ambiguous
/// combo ("Katalysatoren und Rußpartikelfilter" in one label) lands on
/// `None` rather than half-mapping to ceramic catalysts.
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    if l.contains("partikel") || l.contains("ruß") || l.contains("russ") || l.contains("dpf") {
        None
    } else if l.contains("katalysator") {
        Some(vec![("katalysatoren", "")])
    } else {
        None
    }
}

/// Loud reason for every unmapped product — the new-material proposal
/// lives here, not in a silent default.
fn skip_reason(label: &str) -> &'static str {
    let l = label.to_lowercase();
    if l.contains("partikel") || l.contains("ruß") || l.contains("russ") || l.contains("dpf") {
        " (kein Katalysator, sondern Rußpartikelfilter/DPF — Vorschlag: partikelfilter-dpf)"
    } else {
        " (unbekannte Annahme, kein Katalogmaterial)"
    }
}

fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    // Window: the first FAQ accordion body only — later bodies hold prose
    // ("täglich anhand der Edelmetallpreise …") whose <strong> phrases must
    // never leak in as products. Both anchors required (guide: Fenster,
    // nie Ganzseite).
    let start = html
        .find("Welche Art von Produkten kaufen wir an?")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahme-Block fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("Wie wird der Wert eines Katalysators bestimmt")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahme-Block unvollständig".to_owned(),
        })?;
    let window = &tail[..end];
    // Content elements only (`div.accordion-body strong`); scripts and the
    // nav stay out.
    let doc = Html::parse_fragment(window);
    let sel = Selector::parse("div.accordion-body strong").expect("valid selector");
    let labels: Vec<String> = doc
        .select(&sel)
        .map(|el| el.text().collect::<String>())
        .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|t| !t.is_empty())
        .collect();
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Annahmeliste leer".to_owned(),
        });
    }
    Ok(labels)
}

/// Bespoke contact extraction. Block 1+2 (THIS impressum only): address
/// lines sit as bare `<br>` text between `<h4>Angaben gemäß § 5 TMG:</h4>`
/// and `<h4>Kontakt</h4>` ("Ali Harbi" / firm / street / "13587 Berlin"),
/// the phone on the "Handy:" line after Kontakt. Block 3 (homepage, own
/// anchored block): the header `mailto:` — the impressum has no e-mail.
/// Missing impressum anchors → loud error, never guessed.
fn extract_info(imp: &str, home: &str) -> Result<TraderInfo, IngestError> {
    let (_, after_tmg) =
        imp.split_once("Angaben gemäß § 5 TMG:")
            .ok_or_else(|| IngestError::Parse {
                url: IMPRESSUM_URL.to_owned(),
                detail: "TMG-Block fehlt".to_owned(),
            })?;
    let (addr_raw, after_kontakt) =
        after_tmg
            .split_once("Kontakt")
            .ok_or_else(|| IngestError::Parse {
                url: IMPRESSUM_URL.to_owned(),
                detail: "Kontakt-Block fehlt".to_owned(),
            })?;
    let lines = text_lines(addr_raw);
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5
                && pc.chars().all(|c| c.is_ascii_digit())
                && ci.chars().next().is_some_and(|c| c.is_uppercase())
            {
                postcode = pc.to_owned();
                city = ci.to_owned();
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                break;
            }
        }
    }
    if postcode.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "PLZ/Ort fehlt".to_owned(),
        });
    }
    let ls = text_lines(after_kontakt);
    eprintln!("HARBI DEBUG: {:?}", ls);
    let phone = ls
        .iter()
        .find_map(|line| line.strip_prefix("Handy:").map(|v| v.trim().to_owned()))
        .unwrap_or_default();
    // Homepage header mailto — its own rule (a phone-style token filter
    // would stop at the first letter). Missing → empty, never fatal: the
    // homepage's job here is the acceptance list, not the address.
    let email = home
        .split_once("mailto:")
        .map(|(_, after)| {
            after
                .split('"')
                .next()
                .unwrap_or_default()
                .trim()
                .to_owned()
        })
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

/// Strip tags from a raw slice (kupferhelden-style text walk — the address
/// lines here are bare `<br>` text nodes, not `<p>` elements, so no
/// selector reaches them). HTML comments are removed first: the commented
/// out `<!--E-Mail: …-->` line would otherwise leak "-->" text, glued onto
/// the Handy line when no newline separates them.
fn text_lines(s: &str) -> Vec<String> {
    let mut no_comments = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find("<!--") {
        no_comments.push_str(&rest[..i]);
        no_comments.push('\n');
        rest = match rest[i..].find("-->") {
            Some(j) => &rest[i + j + 3..],
            None => "",
        };
    }
    no_comments.push_str(rest);
    let s = no_comments.as_str();
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        if c == '<' {
            in_tag = true;
            out.push('\n');
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            out.push(c);
        }
    }
    out.lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|l| !l.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, skip_reason};

    // Real structure, shortened: first FAQ item (anchor button + body with
    // the two <strong> products), terminated by the next question header.
    const FIXTURE: &str = "<button class=\"accordion-button\" type=\"button\" \
        data-bs-toggle=\"collapse\" data-bs-target=\"#collapseOne\" aria-expanded=\"true\" \
        aria-controls=\"collapseOne\">Welche Art von Produkten kaufen wir an?</button>\
        <div id=\"collapseOne\" class=\"accordion-collapse collapse show\">\
        <div class=\"accordion-body\">Unser Unternehmen kauft gebrauchte/defekte \
        <strong>Katalysatoren</strong> und <strong>Rußpartikelfilter</strong> an.</div></div>\
        <button class=\"accordion-button collapsed\" type=\"button\">\
        Wie wird der Wert eines Katalysators bestimmt und wie oft ändern sich die Preise?</button>\
        <div class=\"accordion-body\">Jeder Katalysator hat einen anderen Wert und die Preise werden \
        <strong>täglich anhand der Edelmetallpreise an den Weltbörsen berechnet</strong> \
        und sofort dokumentiert.</div>";

    const HOME: &str =
        "<a class=\"btn btn-dark px-3 py-2 m-1\" href=\"mailto:info@harbi-kats.de\">\
        <i class=\"fa fa-fw fa-envelope opacity-50 me-1\"></i>info@harbi-kats.de</a>";

    // Real impressum shape: bare <br> lines between the two <h4> anchors,
    // commented-out E-Mail, Handy line after Kontakt.
    const IMP: &str = "<h1 class=\"text-center\">Impressum</h1>\
        <h4>Angaben gemäß § 5 TMG:</h4>\
        <br> Ali Harbi<br>Harbi Katalysatoren Recycling<br>Rauchstraße 43<br>13587 Berlin <br><br>\
        <h4>Kontakt</h4><!--E-Mail: -<br> <br>-->\
        Handy: 01795950837<br> <br>Steuer Nr. 119/331/00637 / Id: 42836704191<br> <br>";

    #[test]
    fn products_map_and_skip() {
        let labels = parse(FIXTURE).expect("parses");
        assert_eq!(labels, vec!["Katalysatoren", "Rußpartikelfilter"]);
        assert_eq!(
            grade_for("Katalysatoren"),
            Some(vec![("katalysatoren", "")])
        );
        assert_eq!(grade_for("Rußpartikelfilter"), None);
        assert_eq!(grade_for("DPF"), None);
        // Ambiguous combos never half-map.
        assert_eq!(grade_for("Katalysatoren und Rußpartikelfilter"), None);
        assert!(skip_reason("Rußpartikelfilter").contains("partikelfilter-dpf"));
        assert!(skip_reason("Irgendwas").contains("unbekannte Annahme"));
    }

    #[test]
    fn window_and_anchors_hold() {
        assert!(parse("<p>Kein FAQ hier</p>").is_err());
        // End anchor missing → loud error, not a whole-page walk.
        assert!(parse("Welche Art von Produkten kaufen wir an? <strong>Kat</strong>").is_err());
        // Bodies without <strong> products → empty list error.
        assert!(parse(
            "Welche Art von Produkten kaufen wir an?<div>nichts</div>\
            Wie wird der Wert eines Katalysators bestimmt"
        )
        .is_err());
        assert!(extract_info("<p>Neu hier</p>", HOME).is_err());
        assert!(extract_info("<h4>Angaben gemäß § 5 TMG:</h4><br>X<br>", HOME).is_err());
    }

    #[test]
    fn impressum_blocks() {
        let info = extract_info(IMP, HOME).expect("parses");
        assert_eq!(info.street, "Rauchstraße 43");
        assert_eq!(info.postcode, "13587");
        assert_eq!(info.city, "Berlin");
        assert_eq!(info.phone, "01795950837");
        assert_eq!(info.email, "info@harbi-kats.de");
        // Homepage without mailto → empty e-mail, rest still parses.
        let info = extract_info(IMP, "<p>kein Kontakt hier</p>").expect("parses");
        assert!(info.email.is_empty());
        assert_eq!(info.street, "Rauchstraße 43");
    }
}
