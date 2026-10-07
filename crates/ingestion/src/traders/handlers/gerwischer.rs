//! Gerwischer Rohstoffrecycling & Verwertungs GmbH (Gerwisch b. Magdeburg):
//! static Elementor price lists under four `h2.cms-heading` sections
//! ("Eisenschrott", "Edelstahl", "Buntmetalle", "Gussschrott") — one
//! `div.cms-list > span.flex-basic` per grade ("Label – 180,00 €/t").
//! The page calls the numbers "unverbindlich" / "Richtwerte" (hero lede
//! + note box), so everything is `price_kind "approx"` at confidence
//! 0.5 (same precedent as schrottabholung_zentrale): if the disclaimer
//! disappears the step fails loudly instead of silently upgrading to
//! exact. No page date ("Stand:") exists → `published_at` is None.
//!
//! Loud skips (page's own wording, never guessed): "Brennerschrott"
//! has no catalog bucket, bare "Kupfer (blank)" is unspecific (no
//! Millberry qualifier — generic never maps to a specific grade),
//! bare "Aluminium (rein)" has no sortenrein material, and
//! "Messing (gemischt)" quotes a truncated unit ("€/k").

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "st-gerwisch-gerwischer-rohstoffrecycling-verwertungs";
/// Bespoke, live-verified impressum URL (the site's own footer link).
/// A move fails the step loudly (fix the URL) — never guessed, never
/// shared.
pub const IMPRESSUM_URL: &str = "https://gerwischer-recycling.de/impressum/";

pub const URL: &str = "https://gerwischer-recycling.de/ankaufspreise/";

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
    // "unverbindlich" guide rates ("Richtwerte"): approx at 0.5.
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit,
                price_kind: "approx",
                price_min: None,
                price_max: None,
                confidence: Some(0.5),
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
/// skipped. Specific-before-generic: "neuschrott" before anything
/// containing bare "schrott", "sphäroguss"/"stahlguss"/"grauguss"
/// before a generic guss arm (there is none — bare guss stays
/// unmapped). One material, several priced grades → distinct variants
/// so they never collapse onto one current price.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // Eisen: dealer grades map to the catalog buckets whose definitions
    // name them ("Neuschrott aus der Verarbeitung"; shredder feed =
    // lighter unzerkleinerter Blech-/Mischschrott).
    if l.contains("neuschrott") {
        return Some(("stahlschrott-sorte-1", ""));
    }
    if l.contains("mischschrott") {
        return Some(("mischschrott", ""));
    }
    if l.contains("leichtschrott") {
        return Some(("stahlschrott-shredder", ""));
    }
    // Guss: each priced grade gets its own variant.
    if l.contains("maschinenguss") {
        return Some(("eisenschrott-gussbruch", "Maschinenguss"));
    }
    if l.contains("sphäroguss") || l.contains("sphaeroguss") {
        return Some(("eisenschrott-gussbruch", "Sphäroguss"));
    }
    if l.contains("stahlguss") {
        return Some(("eisenschrott-gussbruch", "Stahlguss"));
    }
    if l.contains("grauguss") {
        return Some(("eisenschrott-gussbruch", "Grauguss"));
    }
    // "Brennerschrott" has no catalog bucket (torch-cut heavy scrap is
    // neither proven scheren nor misch) → None, loud skip.
    // Edelstahl grades are explicit.
    if l.contains("v4a") {
        return Some(("edelstahl-v4a", ""));
    }
    if l.contains("v2a") {
        return Some(("edelstahl-v2a", ""));
    }
    // Kupfer: only the qualified Berry grade maps. Bare "Kupfer (blank)"
    // carries no Millberry qualifier, so it must not land on a specific
    // grade — and no generic blank-copper material exists → None.
    if l.contains("berry") {
        return Some(("kupfer-berry", ""));
    }
    if l.contains("kupfer") {
        return None;
    }
    // Aluminium: only the mixed grade maps. Bare "Aluminium (rein)" has
    // no sortenrein material → None.
    if l.contains("alu") {
        if l.contains("gemischt") || l.contains("gemischt") {
            return Some(("aluminium-gemischt", ""));
        }
        return None;
    }
    if l.contains("messing") {
        return Some(("messing", ""));
    }
    if l.contains("zink") {
        return Some(("zink", ""));
    }
    if l.contains("blei") {
        return Some(("blei", ""));
    }
    None
}

/// This page's own unit spellings — kg/t only. Anything else (notably
/// the truncated "€/k" on the Messing row) is a loud skip, never a
/// default: tonne-as-kilo would be a 1000x error.
fn unit_of(raw: &str) -> Option<&'static str> {
    let u = raw.trim();
    if u == "€/t" {
        Some("EUR/t")
    } else if u == "€/kg" {
        Some("EUR/kg")
    } else {
        None
    }
}

fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    // Richtwert disclaimer must be present or the page stopped calling
    // these numbers guide rates → loud error, never silent exact.
    if !(html.contains("Richtwerte") && html.contains("unverbindlich")) {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Richtwert-Hinweis fehlt".to_owned(),
        });
    }
    // Window, never whole page: head CSS and CTAs also carry €/spans.
    let start = html
        .find("Ankaufpreise</h1>")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preis-Anker fehlt".to_owned(),
        })?;
    let end = html[start..]
        .find("Individuelles Angebot")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Fenster-Ende fehlt".to_owned(),
        })?;
    let window = &html[start..start + end];
    let doc = Html::parse_fragment(window);
    let row_sel = Selector::parse("div.cms-list span.flex-basic").expect("valid selector");
    let rows: Vec<String> = doc
        .select(&row_sel)
        .map(|e| e.text().collect::<String>())
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| !s.is_empty())
        .collect();
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preiszeilen".to_owned(),
        });
    }
    let mut out = Vec::new();
    let mut skips = Vec::new();
    for row in rows {
        let (label, right) = match row.split_once('–') {
            Some((l, r)) => (l.trim().to_owned(), r.trim().to_owned()),
            None => {
                skips.push(format!("{row} (kein Trennstrich)"));
                continue;
            }
        };
        let price = match parse_eur(&right) {
            Some(p) => p,
            None => {
                skips.push(format!("{label} (Preis unverständlich: {right})"));
                continue;
            }
        };
        if price <= 0.0 {
            skips.push(format!("{label} (0.00-Preis)"));
            continue;
        }
        // Unit is whatever trails the number ("180,00 €/t" → "€/t").
        let unit_raw = right
            .chars()
            .skip_while(|c| c.is_ascii_digit() || *c == '.' || *c == ',' || c.is_whitespace())
            .collect::<String>();
        match unit_of(&unit_raw) {
            Some(unit) => out.push((label, price, unit)),
            None => skips.push(format!("{label} (Einheit unverständlich: {unit_raw})")),
        }
    }
    if out.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine parsebaren Preiszeilen".to_owned(),
        });
    }
    Ok((out, skips))
}

/// Bespoke contact extraction for THIS impressum only: the
/// `div.cms-desc` inside the same `.cms-eheading` as the
/// "Angaben gemäß § 5 TMG" heading holds `<br>`-separated address lines
/// (street = line before the PLZ line); the `div.cms-desc` next to the
/// "Kontakt" heading holds "Telefon:"/"E-Mail:" lines. Missing anchors
/// mean the page changed shape → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2_sel = Selector::parse("h2").expect("valid selector");
    let anchor = doc.select(&h2_sel).find(|h| {
        h.text()
            .collect::<String>()
            .contains("Angaben gemäß § 5 TMG")
    });
    if anchor.is_none() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "TMG-Block fehlt".to_owned(),
        });
    }
    let desc_sel = Selector::parse("div.cms-desc").expect("valid selector");
    // Address block: cms-desc sharing the anchor's parent (.cms-eheading).
    let addr_html = anchor
        .and_then(|h| h.parent())
        .and_then(|p| scraper::ElementRef::wrap(p))
        .and_then(|e| e.select(&desc_sel).next())
        .map(|d| d.inner_html())
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        })?;
    // ["Gerwischer Rohstoffrecycling und Verwertungs GmbH",
    //  "Lostauer Straße 5", "39175 Biederitz OT Gerwisch", ...]:
    // street is the line before the PLZ line.
    let lines: Vec<String> = addr_html
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5
                && pc.chars().all(|c| c.is_ascii_digit())
                && ci.chars().next().is_some_and(|c| c.is_uppercase())
            {
                postcode = pc.to_owned();
                city = line[pc.len()..].trim().to_owned();
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                break;
            }
        }
    }
    // Kontakt block: same parent trick on the "Kontakt" heading.
    let (mut phone, mut email) = (String::new(), String::new());
    if let Some(h) = doc
        .select(&h2_sel)
        .find(|h| h.text().collect::<String>().trim() == "Kontakt")
    {
        if let Some(div) = h
            .parent()
            .and_then(scraper::ElementRef::wrap)
            .and_then(|e| e.select(&desc_sel).next())
        {
            let contact: Vec<String> = div
                .inner_html()
                .split("<br")
                .map(strip_fragment)
                .filter(|s| !s.is_empty())
                .collect();
            for line in contact {
                if let Some(rest) = line.strip_prefix("Telefon:") {
                    if phone.is_empty() {
                        phone = rest.trim().to_owned();
                    }
                } else if let Some(rest) = line.strip_prefix("E-Mail:") {
                    if email.is_empty() {
                        email = rest.trim().to_owned();
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    /// Real excerpt from the live page (whitespace collapsed, tags and
    /// classes verbatim): hero disclaimer + all four section headings +
    /// all 17 grade rows.
    const FIXTURE: &str = "<h1 class=\"cms-title\">Ankaufpreise</h1>\
        <div class=\"cms-desc\">Unsere Preise orientieren sich an den täglichen Marktnotierungen. \
        Die folgenden Richtwerte dienen Ihrer ersten Orientierung</div>\
        <div class=\"cms-desc\">Hinweis: Die angegebenen Preise sind unverbindlich und können \
        je nach Menge, Qualität und Marktsituation abweichen.</div>\
        <h2 class=\"cms-heading\">Eisenschrott</h2>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Mischschrott (Sorte 3A) – 180,00 €/t</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Schwerer Neuschrott (Sorte 2A) – 210,00 €/t</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Maschinenguss – 195,00 €/t</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Brennerschrott – 165,00 €/t</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Leichtschrott – 120,00 €/t</span></div>\
        <h2 class=\"cms-heading\">Edelstahl</h2>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">V2A (rostfrei) – 1,05 €/kg</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">V4A (säurefest) – 1,60 €/kg</span></div>\
        <h2 class=\"cms-heading\">Buntmetalle</h2>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Kupfer (blank) – 7,20 €/kg</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Kupfer (Berry) – 6,50 €/kg</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Messing (gemischt) – 4,10 €/k</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Aluminium (rein) – 1,40 €/kg</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Aluminium (gemischt) – 0,85 €/kg</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Zink – 1,10 €/kg</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Blei – 0,95 €/kg</span></div>\
        <h2 class=\"cms-heading\">Gussschrott</h2>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Grauguss – 175,00 €/t</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Stahlguss – 185,00 €/t</span></div>\
        <div class=\"cms-list d-flex\"><span class=\"cms-list-icon cmsi-check flex-auto\"></span>\
        <span class=\"flex-basic\">Sphäroguss – 190,00 €/t</span></div>\
        <h2>Individuelles Angebot</h2>";

    #[test]
    fn parses_all_rows_with_loud_skips() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // 17 rows: only Messing drops here (truncated unit "€/k").
        // Brennerschrott, Kupfer blank and Alu rein parse fine but map
        // to no material in grade_for (tested below).
        assert_eq!(rows.len(), 16, "rows: {rows:?}");
        assert_eq!(skips.len(), 1, "skips: {skips:?}");
        assert!(skips[0].contains("Messing") && skips[0].contains("€/k"));
        let first = &rows[0];
        assert_eq!(first.0, "Mischschrott (Sorte 3A)");
        assert_eq!(first.1, 180.0);
        assert_eq!(first.2, "EUR/t");
        // Grading over the parsed rows: 13 prices, 3 loud material
        // skips (Brennerschrott: no bucket; Kupfer blank: unspecific;
        // Aluminium rein: no material).
        let mut priced = 0;
        let mut unmapped = Vec::new();
        for (label, _, _) in &rows {
            if grade_for(label).is_some() {
                priced += 1;
            } else {
                unmapped.push(label.clone());
            }
        }
        assert_eq!(priced, 13);
        assert_eq!(unmapped.len(), 3, "unmapped: {unmapped:?}");
        assert!(unmapped.iter().any(|s| s.starts_with("Brennerschrott")));
        assert!(unmapped.iter().any(|s| s.starts_with("Kupfer (blank)")));
        assert!(unmapped.iter().any(|s| s.starts_with("Aluminium (rein)")));
    }

    #[test]
    fn missing_disclaimer_is_loud() {
        let html = FIXTURE
            .replace("Richtwerte", "Festpreise")
            .replace("unverbindlich", "verbindlich");
        assert!(parse(&html).is_err());
    }

    #[test]
    fn grade_table() {
        assert_eq!(
            grade_for("Mischschrott (Sorte 3A)"),
            Some(("mischschrott", ""))
        );
        assert_eq!(
            grade_for("Schwerer Neuschrott (Sorte 2A)"),
            Some(("stahlschrott-sorte-1", ""))
        );
        assert_eq!(
            grade_for("Maschinenguss"),
            Some(("eisenschrott-gussbruch", "Maschinenguss"))
        );
        assert_eq!(grade_for("Brennerschrott"), None, "no bucket");
        assert_eq!(
            grade_for("Leichtschrott"),
            Some(("stahlschrott-shredder", ""))
        );
        assert_eq!(grade_for("V2A (rostfrei)"), Some(("edelstahl-v2a", "")));
        assert_eq!(grade_for("V4A (säurefest)"), Some(("edelstahl-v4a", "")));
        assert_eq!(grade_for("Kupfer (blank)"), None, "unspecific");
        assert_eq!(grade_for("Kupfer (Berry)"), Some(("kupfer-berry", "")));
        assert_eq!(
            grade_for("Aluminium (gemischt)"),
            Some(("aluminium-gemischt", ""))
        );
        assert_eq!(grade_for("Aluminium (rein)"), None, "no material");
        assert_eq!(grade_for("Zink"), Some(("zink", "")));
        assert_eq!(grade_for("Blei"), Some(("blei", "")));
        assert_eq!(
            grade_for("Grauguss"),
            Some(("eisenschrott-gussbruch", "Grauguss"))
        );
        assert_eq!(
            grade_for("Stahlguss"),
            Some(("eisenschrott-gussbruch", "Stahlguss"))
        );
        assert_eq!(
            grade_for("Sphäroguss"),
            Some(("eisenschrott-gussbruch", "Sphäroguss"))
        );
    }

    #[test]
    fn units_strict() {
        assert_eq!(unit_of("€/t"), Some("EUR/t"));
        assert_eq!(unit_of("€/kg"), Some("EUR/kg"));
        assert_eq!(unit_of("€/k"), None, "truncated unit skips");
    }

    #[test]
    fn impressum_parses() {
        // Live shape: each h2 shares its .cms-eheading wrapper with its
        // div.cms-desc (the parent trick in extract_info relies on it).
        let imp = "<div class=\"cms-eheading\">\
            <h2 class=\"cms-heading\">Angaben gemäß § 5 TMG</h2>\
            <div class=\"cms-desc\">Gerwischer Rohstoffrecycling und Verwertungs GmbH<br />\
            Lostauer Straße 5<br />39175 Biederitz OT Gerwisch<br /><br />\
            Ust.-ID\tDE 158 243 753<br />Amtsgericht Stendal | HRB 1316</div></div>\
            <div class=\"cms-eheading\">\
            <h2 class=\"cms-heading\">Kontakt</h2>\
            <div class=\"cms-desc\">Telefon: +49 (0) 39292 / 20 90<br />\
            Fax: +49 (0) 39292 / 20 78<br />E-Mail: info@gerwischer-recycling.de</div></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Lostauer Straße 5");
        assert_eq!(info.postcode, "39175");
        assert_eq!(info.city, "Biederitz OT Gerwisch");
        assert_eq!(info.phone, "+49 (0) 39292 / 20 90");
        assert_eq!(info.email, "info@gerwischer-recycling.de");
    }

    #[test]
    fn impressum_without_anchor_fails() {
        let imp = "<h2 class=\"cms-heading\">Kontakt</h2>\
            <div class=\"cms-desc\">Telefon: +49 (0) 39292 / 20 90</div>";
        assert!(extract_info(imp).is_err());
    }
}
