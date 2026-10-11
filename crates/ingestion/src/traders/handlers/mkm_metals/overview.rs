//! Full article-aware overview parser; parsing itself performs no DB writes.
//! Source metadata travels beside the legacy HandlerOutcome through MKM's
//! typed scheduler/recording boundary, preserving identities, tiers and dates.

use std::collections::BTreeSet;

use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::traders::{HandlerOutcome, ScrapedAcceptance, ScrapedPrice};
use crate::IngestError;

use super::{extract_info, parse_de_date, URL};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Tier {
    pub ab: f64,
    pub preis: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Article {
    pub article: String,
    pub group: String,
    pub label: String,
    pub url: String,
    pub price_cell: String,
    pub price_eur_per_kg: Option<f64>,
    pub product_change_text: String,
    pub product_change_date: Option<String>,
    pub tiers: Vec<Tier>,
    pub minimum_for_upper_bound_kg: Option<f64>,
    pub material: Option<&'static str>,
    pub variant: String,
    pub request_only: bool,
}

#[derive(Debug)]
pub struct Snapshot {
    pub articles: Vec<Article>,
    pub outcome: HandlerOutcome,
    pub page_change_date: Option<String>,
    pub diagnostics: Vec<String>,
}

impl Snapshot {
    /// Inventory evidence, not approval. The typed seam is isolated and tested;
    /// production still requires owner review/integration/deployment.
    pub fn report(&self) -> serde_json::Value {
        serde_json::json!({
            "source": URL,
            "production_ready": false,
            "activation_blocker": "owner full review and integration/deployment of typed MKM transaction candidate",
            "recording_contract": "MKM typed 60-article validation; atomic quote/acceptance/manifest/current-pointer reconciliation; immutable price metadata",
            "page_change_precision": "calendar_day; UTC midnight is normalization, not source time",
            "visibility_scope": "Explicit hidden attributes and inline display:none on products/ancestors; no external CSS/browser visibility proof.",
            "required_alias_migrations": [{
                "article": "MKM-CU-01", "source_url": URL,
                "from": ["kupfer-berry", "Kerze"],
                "to": ["kupfer-stanzabfaelle", "Kerze"],
                "scope": "MKM trader and verified overview/product provenance only; preserve history and protect foreign sources"
            }],
            "page_change_date": self.page_change_date,
            "groups": self.articles.iter().map(|a| &a.group).collect::<BTreeSet<_>>().len(),
            "products": self.articles.len(),
            "numeric": self.outcome.prices.len(),
            "request_only": self.outcome.acceptances.len(),
            "diagnostics": self.diagnostics,
            "articles": self.articles,
        })
    }
}

// Article + exact source title + exact deep-link identity, never keyword
// inference. Changing title or URL forces renewed semantic verification.
// Ten other Legacy identities stay stable. Kerze's demonstrated domain error
// is corrected here; the typed recording seam performs source-scoped alias
// retirement while preserving history, never silently retaining Berry wire.
// A group move is reported separately rather than silently approved.
type Mapping = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    Option<&'static str>,
    usize,
);
pub(super) const MAPPINGS: &[Mapping] = &[
    (
        "MET-KU-MILL",
        "Kupfer Millberry",
        "kupfer-millberry",
        "kupfer-millberry",
        Some(""),
        0,
    ),
    (
        "MKM-CU-01",
        "Kupfer Kerze",
        "kupfer-kerze",
        "kupfer-stanzabfaelle",
        Some("Kerze"),
        0,
    ),
    (
        "MKM-CU-03",
        "Kupfer Sammelschienen",
        "kupfer-sammelschienen",
        "kupfer-gemischt",
        Some("Sammelschienen"),
        0,
    ),
    (
        "MKM-CU-02",
        "Kupfergranulat",
        "kupfergranulat",
        "kupfer-granulat",
        None,
        0,
    ),
    (
        "SW10013",
        "Kupfer (Cu) schwer Basis 95% Kupfer-Inhalt",
        "cu-schwer-basis-95-kupfer-inhalt",
        "kupfer-gemischt",
        Some("schwer"),
        0,
    ),
    (
        "SW10014",
        "Kupfer Berry/98%",
        "cu-berry-98",
        "kupfer-berry",
        Some("Berry 98%"),
        0,
    ),
    (
        "MET-KU-RAFF",
        "Kupfer Raff",
        "kupfer-raff",
        "kupfer-gemischt",
        Some("Raff"),
        0,
    ),
    (
        "MKM-CU-04",
        "Kupfer leicht",
        "kupfer-leicht",
        "kupfer-gemischt",
        Some("leicht"),
        0,
    ),
    (
        "MET-KA-70",
        "Kupferkabel ca. 70%",
        "kupferkabel-ca-70",
        "kabel-kupfer",
        Some("70%"),
        0,
    ),
    (
        "SW10017",
        "Kupfer/Messing Kühler ohne Eisen",
        "cu-ms-kuehler-ohne-fe",
        "cu-ms-kuehler",
        None,
        0,
    ),
    (
        "MET-KA-60",
        "Kupferkabel ca. 60%",
        "kupferkabel-ca-60",
        "kabel-kupfer",
        Some("60%"),
        0,
    ),
    (
        "SW10015",
        "Kupferkabel gemischt (38-40% Kupfer)",
        "cu-kabel-gemischt-38-40-cu",
        "kabel-kupfer",
        Some("38-40%"),
        0,
    ),
    (
        "SW10016",
        "Kupfer Steckerkabel (30% Kupfer)",
        "cu-steckerkabel-30-cu",
        "kabel-mit-stecker",
        Some("Stecker 30%"),
        0,
    ),
    (
        "SW10035",
        "Alu Kupfer Kabel",
        "alu-kupfer-kabel",
        "kabel-alu-cu",
        None,
        0,
    ),
    (
        "SW10024",
        "Rotguss-Stücke",
        "rotguss-stuecke",
        "bronze-rotguss",
        None,
        1,
    ),
    (
        "SW10025",
        "Rotguss-Späne",
        "rotguss-spaene",
        "bronze-rotguss",
        None,
        1,
    ),
    (
        "SW10020",
        "Messing Hülsen",
        "messing-huelsen",
        "messing",
        None,
        1,
    ),
    (
        "SW10019",
        "Messing 63 Abfälle",
        "messing-63-abfaelle",
        "messing",
        None,
        1,
    ),
    (
        "SW10018",
        "Messing 58 Abfälle",
        "messing-58-abfaelle",
        "messing",
        None,
        1,
    ),
    (
        "SW10021",
        "Messing schwer",
        "messing-schwer",
        "messing",
        None,
        1,
    ),
    (
        "SW10023",
        "Messing Späne",
        "messing-spaene",
        "messing",
        None,
        1,
    ),
    (
        "SW10022",
        "Messing leicht",
        "messing-leicht",
        "messing-leicht",
        None,
        1,
    ),
    (
        "SW10011",
        "Alu Kupfer Kühler Fe frei",
        "alu-kupfer-kuehler-fe-frei",
        "alu-cu-kuehler",
        None,
        2,
    ),
    (
        "MKM-AL-06",
        "Alu Profile Neu",
        "alu-profile",
        "aluminium-profile",
        None,
        2,
    ),
    (
        "SW10001",
        "Alu Profile blank/max. 10%eloxiert",
        "alu-profile-blank-max-10-eloxiert",
        "aluminium-profile",
        None,
        2,
    ),
    (
        "SW10002",
        "Alu Felgen sauber",
        "alu-felgen-sauber",
        "aluminium-felgen",
        None,
        2,
    ),
    (
        "SW10000",
        "Alu Iso Profile Ausbau",
        "alu-iso-profile-ausbau",
        "aluminium-isoprofile",
        None,
        2,
    ),
    (
        "MKM-AL-01",
        "Alu Profile mit Farbe",
        "alu-profile-farbe",
        "aluminium-profile-beschichtet",
        None,
        2,
    ),
    (
        "SW10003",
        "Alu Felgen unsauber",
        "alu-felgen-unsauber",
        "aluminium-felgen",
        None,
        2,
    ),
    (
        "SW10004",
        "Alu Geschirr max. 10% Anhaftungen",
        "alu-geschirr-max-10-anhaftungen",
        "aluminium-gemischt",
        None,
        2,
    ),
    (
        "MKM-AL-02",
        "Alu Bleche neu blank",
        "alu-bleche-neu",
        "aluminium-blech",
        None,
        2,
    ),
    (
        "MKM-AL-03",
        "Aluminium gemischt",
        "aluminium-gemischt",
        "aluminium-gemischt",
        None,
        2,
    ),
    (
        "MKM-AL-04",
        "Alu Guss sauber",
        "alu-guss-sauber",
        "aluminium-guss",
        None,
        2,
    ),
    (
        "MKM-AL-05",
        "Aluminiumkabel",
        "aluminiumkabel",
        "kabel-alu",
        None,
        2,
    ),
    (
        "SW10012",
        "Alu Späne Basis 5%",
        "alu-spaene-basis-5",
        "aluminium-spaene",
        None,
        2,
    ),
    (
        "SW10028",
        "V4A Abfälle bis 1,5m",
        "v4a-abfaelle-bis-1-5m",
        "edelstahl-v4a",
        None,
        3,
    ),
    (
        "SW10026",
        "V2A Abfälle bis 1,5m",
        "v2a-abfaelle-bis-1-5m",
        "edelstahl-v2a",
        None,
        3,
    ),
    (
        "SW10029",
        "V2A-Späne",
        "v2a-spaene",
        "edelstahl-v2a",
        None,
        3,
    ),
    (
        "MKM-NE-06",
        "Wolfram ab 99 %",
        "wolfram",
        "wolfram",
        None,
        4,
    ),
    (
        "SW10040",
        "Zinn 99%",
        "zinn-99",
        "zinn-rein-formgemischt",
        None,
        4,
    ),
    (
        "SW10039",
        "Zinn 90%",
        "zinn-90",
        "zinn-legierungen",
        None,
        4,
    ),
    ("SW10042", "Zinn 60%", "zinn-60", "loetzinn", None, 4),
    (
        "MKM-NE-05",
        "Nickel rein ab 99 %",
        "nickel-rein",
        "nickel",
        None,
        4,
    ),
    ("MKM-NE-03", "Lötzinn", "loetzinn", "loetzinn", None, 4),
    ("SW10041", "Zinn 30%", "zinn-30", "loetzinn", None, 4),
    (
        "MKM-NE-04",
        "Nickel 60-99 %",
        "nickel-60-99",
        "nickel",
        None,
        4,
    ),
    (
        "MKM-NE-01",
        "Leiterplatten",
        "leiterplatten",
        "platinen",
        None,
        4,
    ),
    (
        "MKM-NE-02",
        "Neodym-Magnete",
        "neodym-magnete",
        "neodym-magnete",
        None,
        4,
    ),
    ("SW10031", "Zink", "zink", "zink", None, 5),
    ("SW10030", "Blei", "blei", "blei-gemischt", None, 5),
    (
        "SW10033",
        "E-Motoren ohne Anbauteile max. 500KG",
        "e-motoren-ohne-anbauteile-max-500kg",
        "elektromotoren",
        None,
        5,
    ),
    (
        "MKM-BL-01",
        "Geschossblei",
        "geschossblei",
        "blei-geschoss",
        None,
        5,
    ),
    (
        "MKM-HM-01",
        "Hartmetall gemischt",
        "hartmetall-gemischt",
        "hartmetall",
        None,
        6,
    ),
    ("SW10037", "VHM Bohrer", "vhm-bohrer", "hartmetall", None, 6),
    (
        "SW10036",
        "Widia Wendeschneidplättchen",
        "widia-wendeschneidplaettchen",
        "hartmetall",
        None,
        6,
    ),
    (
        "MKM-HM-02",
        "Hartmetall Stückschrott",
        "hartmetall-stueckschrott",
        "hartmetall",
        None,
        6,
    ),
    (
        "SW10038",
        "HSS Schnellarbeitsstahl",
        "hss-schnellarbeitsstahl",
        "hss-werkzeuge",
        None,
        6,
    ),
    (
        "MKM-SCH-01",
        "Eisenschrott",
        "eisenschrott",
        "mischschrott",
        None,
        7,
    ),
    (
        "MKM-SCH-03",
        "Metallschrott",
        "metallschrott",
        "metallschrott-gemischt",
        None,
        7,
    ),
    (
        "MKM-SCH-02",
        "Mischschrott",
        "mischschrott",
        "schrott-eisen-ne-gemischt",
        None,
        7,
    ),
];
pub(super) const GROUPS: &[&str] = &[
    "Ankaufpreise für Kupfer",
    "Ankaufpreise für Messing & Rotguss",
    "Ankaufpreise für Aluminium",
    "Ankaufpreise für Edelstahl",
    "Ankaufpreise für NE-Metalle",
    "Ankaufpreise für Blei, Zink & E-Motoren",
    "Ankaufpreise für Hartmetall",
    "Ankaufpreise für Schrott",
];

fn text(el: ElementRef<'_>) -> String {
    el.text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn fail(detail: impl Into<String>) -> IngestError {
    IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.into(),
    }
}

// Explicit DOM/inline hiding only; external CSS is not evaluated.
// Accessibility captions with class=visually-hidden remain legitimate.
fn explicitly_hidden(el: ElementRef<'_>) -> bool {
    std::iter::once(el)
        .chain(el.ancestors().filter_map(ElementRef::wrap))
        .any(|el| {
            el.value().attr("hidden").is_some()
                || el.value().attr("style").is_some_and(|style| {
                    style.split(';').any(|declaration| {
                        let Some((property, value)) = declaration.split_once(':') else {
                            return false;
                        };
                        let value: String = value
                            .chars()
                            .filter(|c| !c.is_whitespace())
                            .flat_map(char::to_lowercase)
                            .collect();
                        property.trim().eq_ignore_ascii_case("display")
                            && (value == "none" || value == "none!important")
                    })
                })
        })
}

fn german_integer(raw: &str) -> Option<f64> {
    let parts: Vec<_> = raw.split('.').collect();
    let first = *parts.first()?;
    if first.is_empty()
        || (first.len() > 1 && first.starts_with('0'))
        || !first.chars().all(|c| c.is_ascii_digit())
        || (parts.len() > 1 && first.len() > 3)
        || parts
            .iter()
            .skip(1)
            .any(|p| p.len() != 3 || !p.chars().all(|c| c.is_ascii_digit()))
    {
        return None;
    }
    raw.replace('.', "")
        .parse::<f64>()
        .ok()
        .filter(|n| n.is_finite())
}

/// Entire normalized cell: German decimal (two cents) € /kg ab integer kg.
/// No suffix contracts, alternate units or permissive Legacy number parsing.
pub(super) fn quote_cell(cell: &str) -> Option<(f64, f64)> {
    let tokens: Vec<_> = cell.split_whitespace().collect();
    if tokens.len() != 6
        || tokens[1] != "€"
        || tokens[2] != "/kg"
        || tokens[3] != "ab"
        || tokens[5] != "kg"
    {
        return None;
    }
    let (whole, cents) = tokens[0].split_once(',')?;
    if cents.len() != 2 || !cents.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    german_integer(whole)?;
    let price = tokens[0]
        .replace('.', "")
        .replace(',', ".")
        .parse::<f64>()
        .ok()?;
    if price <= 0.0 {
        return None;
    }
    Some((price, german_integer(tokens[4])?))
}

/// Parse exactly captioned purchase tables, preserving every source row in
/// the inventory. Unknown/changed identities produce diagnostics, not prices.
pub fn parse(html: &str, impressum: &str, status: u16) -> Result<Snapshot, IngestError> {
    if status != 200 {
        return Err(fail(format!("unexpected HTTP {status}")));
    }
    let identity_doc = Html::parse_document(impressum);
    let heading_sel = Selector::parse("h2").expect("static selector");
    let address_heading = identity_doc
        .select(&heading_sel)
        .find(|h| text(*h) == "Angaben gemäß § 5 DDG")
        .ok_or_else(|| fail("operator identity block missing"))?;
    let address = address_heading
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p")
        .map(text)
        .ok_or_else(|| fail("operator identity paragraph missing"))?;
    if address != "MKM Metals e.K. Inhaber: Mubarik Ahmad Industriestraße 4 67063 Ludwigshafen am Rhein Deutschland" {
        return Err(fail(format!("operator identity/address changed: {address}")));
    }
    let register_heading = identity_doc
        .select(&heading_sel)
        .find(|h| text(*h) == "Registereintrag")
        .ok_or_else(|| fail("operator register block missing"))?;
    let register = register_heading
        .next_siblings()
        .filter_map(ElementRef::wrap)
        .find(|e| e.value().name() == "p")
        .map(text)
        .ok_or_else(|| fail("operator register paragraph missing"))?;
    if register
        .split_once("Registernummer:")
        .map(|(_, number)| number.trim())
        != Some("HRA 61664")
    {
        return Err(fail(
            "operator register identity changed: HRA 61664 missing",
        ));
    }
    let doc = Html::parse_document(html);
    let selector = |s| Selector::parse(s).expect("static selector");
    let table_sel = selector("table");
    let caption_sel = selector("caption");
    let row_sel = selector("tbody > tr");
    let link_sel = selector("th[scope=row] > a");
    let cell_sel = selector("td");
    let time_sel = selector("time");
    let tier_sel = selector("[data-staffel]");
    let strong_sel = selector("strong");
    let eyebrow_sel = selector(".eyebrow");
    let page_dates: Vec<_> = doc
        .select(&eyebrow_sel)
        .map(text)
        .filter(|t| t.starts_with("Preisliste · zuletzt geändert"))
        .collect();
    if page_dates.len() > 1 {
        return Err(fail("ambiguous page change date"));
    }
    let page_change_date = page_dates
        .first()
        .map(|t| {
            let rest = t
                .strip_prefix("Preisliste · zuletzt geändert ")
                .ok_or_else(|| fail("invalid page change date syntax"))?;
            let parts: Vec<_> = rest.split('.').collect();
            if parts.len() != 3 {
                return Err(fail("invalid page change date syntax"));
            }
            parse_de_date(parts[0], parts[1], parts[2])
                .ok_or_else(|| fail("invalid present page change date"))
        })
        .transpose()?;
    let mut outcome = HandlerOutcome {
        trader_info: extract_info(impressum)?,
        website_alive: true,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        // Page change is not a common price publication date.
        published_at: None,
        ..HandlerOutcome::default()
    };
    let mut articles = Vec::new();
    let mut diagnostics = Vec::new();
    let mut seen = BTreeSet::new();
    let mut seen_groups = BTreeSet::new();
    for table in doc.select(&table_sel) {
        let Some(caption) = table.select(&caption_sel).next() else {
            continue;
        };
        let group = text(caption);
        if !group.starts_with("Ankaufpreise für ") {
            continue;
        }
        if explicitly_hidden(table) {
            continue;
        }
        if !seen_groups.insert(group.clone()) {
            return Err(fail(format!("duplicate purchase table {group}")));
        }
        if !GROUPS.contains(&group.as_str()) {
            diagnostics.push(format!("unknown purchase group: {group}"));
        }
        let mut group_rows = 0;
        for tr in table.select(&row_sel) {
            group_rows += 1;
            if explicitly_hidden(tr) {
                continue;
            }
            let links: Vec<_> = tr.select(&link_sel).collect();
            let cells: Vec<_> = tr.select(&cell_sel).collect();
            if links.len() != 1 || cells.len() != 4 {
                return Err(fail(format!("changed row layout in {group}")));
            }
            if explicitly_hidden(links[0])
                || links[0]
                    .descendants()
                    .filter_map(ElementRef::wrap)
                    .any(explicitly_hidden)
                || cells[0]
                    .descendants()
                    .filter_map(ElementRef::wrap)
                    .any(explicitly_hidden)
                || explicitly_hidden(cells[1])
                || cells[1]
                    .descendants()
                    .filter_map(ElementRef::wrap)
                    .any(explicitly_hidden)
            {
                continue;
            }
            let label = text(links[0]);
            let article = text(cells[0]);
            let href = links[0]
                .value()
                .attr("href")
                .ok_or_else(|| fail("product link without href"))?;
            if label.is_empty() || article.is_empty() || !href.starts_with("/metallankauf/produkt/")
            {
                return Err(fail(format!("empty/invalid product identity in {group}")));
            }
            if !seen.insert(article.clone()) {
                return Err(fail(format!("duplicate article {article}")));
            }
            let price_cell = text(cells[1]);
            let request_only = price_cell == "auf Anfrage";
            let quote = if request_only {
                None
            } else {
                Some(quote_cell(&price_cell).ok_or_else(|| {
                    fail(format!(
                        "{article}: unknown complete price-cell grammar: {price_cell}"
                    ))
                })?)
            };
            let price = quote.map(|q| q.0);
            let minimum = quote.map(|q| q.1);
            if quote.is_some() {
                let strong: Vec<_> = cells[1].select(&strong_sel).collect();
                if strong.len() != 1 {
                    return Err(fail(format!("{article}: ambiguous price")));
                }
                let expected = format!(
                    "{} €",
                    price_cell
                        .split_whitespace()
                        .next()
                        .expect("validated grammar")
                );
                if text(strong[0]) != expected {
                    return Err(fail(format!(
                        "{article}: strong quote differs from full price cell"
                    )));
                }
            }
            let times: Vec<_> = cells[2].select(&time_sel).collect();
            if times.len() > 1 {
                return Err(fail(format!("{article}: ambiguous product date")));
            }
            let product_change_date = times
                .first()
                .map(|time| {
                    let date = time
                        .value()
                        .attr("datetime")
                        .ok_or_else(|| fail(format!("{article}: datetime missing")))?;
                    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
                        .map_err(|_| fail(format!("{article}: invalid date {date}")))?;
                    Ok::<_, IngestError>(date.to_owned())
                })
                .transpose()?;
            if price.is_some() && product_change_date.is_none() {
                diagnostics.push(format!(
                    "{article}: numeric article without machine-readable product change date: {}",
                    text(cells[2])
                ));
            }
            let tier_inputs: Vec<_> = cells[3].select(&tier_sel).collect();
            if tier_inputs.len() > 1 {
                return Err(fail(format!("{article}: duplicate tier inputs")));
            }
            let tiers: Vec<Tier> = tier_inputs
                .first()
                .map(|input| {
                    serde_json::from_str(input.value().attr("data-staffel").expect("matched attr"))
                        .map_err(|e| fail(format!("{article}: invalid tiers: {e}")))
                })
                .transpose()?
                .unwrap_or_default();
            if let Some(price) = price {
                if tiers.is_empty()
                    || tiers[0].ab != 0.0
                    || tiers.iter().any(|t| {
                        !t.ab.is_finite() || t.ab < 0.0 || !t.preis.is_finite() || t.preis <= 0.0
                    })
                    || tiers
                        .windows(2)
                        .any(|t| t[0].ab >= t[1].ab || t[0].preis > t[1].preis)
                    || tiers
                        .last()
                        .is_none_or(|t| (t.preis - price).abs() > 0.000001 || Some(t.ab) != minimum)
                {
                    return Err(fail(format!(
                        "{article}: tiers disagree with visible upper bound/quantity"
                    )));
                }
                let baseline = tier_inputs[0]
                    .value()
                    .attr("data-price")
                    .and_then(|p| p.parse::<f64>().ok());
                if baseline.is_none_or(|p| !p.is_finite() || (p - tiers[0].preis).abs() > 0.000001)
                {
                    return Err(fail(format!(
                        "{article}: hidden base price disagrees with tiers"
                    )));
                }
            } else if !tiers.is_empty() {
                return Err(fail(format!("{article}: request with numeric tiers")));
            }
            let mapping = MAPPINGS.iter().find(|m| m.0 == article);
            let material = match mapping {
                Some(m)
                    if m.1 == label
                        && href == format!("/metallankauf/produkt/{}", m.2)
                        && GROUPS[m.5] == group =>
                {
                    Some(m.3)
                }
                Some(_) => {
                    diagnostics.push(format!(
                        "{article}: changed identity/group: {label} | {href} | {group}"
                    ));
                    None
                }
                None => {
                    diagnostics.push(format!(
                        "{article}: unknown article: {label} | {href} | {group}"
                    ));
                    None
                }
            };
            let variant = mapping.and_then(|m| m.4).unwrap_or(&label).to_owned();
            if let Some(material) = material {
                if let Some(price) = price {
                    outcome.prices.push(ScrapedPrice {
                        material,
                        variant: variant.clone().into(),
                        price,
                        currency: "EUR",
                        unit: "EUR/kg",
                        price_kind: "upto",
                        price_min: None,
                        price_max: Some(price),
                        confidence: Some(0.5),
                        label: label.clone(),
                    });
                } else {
                    outcome.acceptances.push(ScrapedAcceptance {
                        material,
                        conditions: format!(
                            "{label}; auf Anfrage; Artikel {article}; https://mkm-metals.de{href}"
                        ),
                        label: label.clone(),
                    });
                }
            }
            articles.push(Article {
                article,
                group: group.clone(),
                label,
                url: format!("https://mkm-metals.de{href}"),
                price_cell,
                price_eur_per_kg: price,
                product_change_text: text(cells[2]),
                product_change_date,
                tiers,
                minimum_for_upper_bound_kg: minimum,
                material,
                variant,
                request_only,
            });
        }
        if group_rows == 0 {
            return Err(fail(format!("empty purchase table {group}")));
        }
    }
    if articles.is_empty() {
        return Err(fail("no purchase articles"));
    }
    for mapping in MAPPINGS {
        if !seen.contains(mapping.0) {
            diagnostics.push(format!(
                "{}: missing known article; retirement NOT authorized",
                mapping.0
            ));
        }
    }
    let mut keys = BTreeSet::new();
    for p in &outcome.prices {
        if !keys.insert((p.material, p.variant.as_ref())) {
            return Err(fail(format!(
                "duplicate mapped price key {} / {}",
                p.material, p.variant
            )));
        }
    }
    outcome.skipped_labels = diagnostics.clone();
    Ok(Snapshot {
        articles,
        outcome,
        page_change_date,
        diagnostics,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const HTML: &str = include_str!("../fixtures/mkm-overview-20261010.html");
    const IMP: &str = include_str!("../fixtures/mkm-impressum-20261010.html");
    const ORACLE: &str = include_str!("../fixtures/mkm-reviewed-mapping-oracle-20261011.json");

    fn snapshot(html: &str) -> Snapshot {
        parse(html, IMP, 200).expect("operator snapshot")
    }

    fn first_product_row(html: &str, attrs: &str) -> String {
        let article = html
            .find("<td class=\"muted small\">MET-KU-MILL</td>")
            .unwrap();
        let row = html[..article].rfind("<tr>").unwrap();
        format!("{}<tr {attrs}>{}", &html[..row], &html[row + 4..])
    }

    fn mutate_first_product(before: &str, after: &str) -> String {
        let article = HTML
            .find("<td class=\"muted small\">MET-KU-MILL</td>")
            .unwrap();
        let start = HTML[..article].rfind("<tr>").unwrap();
        let end = article + HTML[article..].find("</tr>").unwrap() + 5;
        let changed = HTML[start..end].replacen(before, after, 1);
        assert_ne!(
            changed,
            HTML[start..end],
            "mutation must reach actual product row"
        );
        format!("{}{}{}", &HTML[..start], changed, &HTML[end..])
    }

    #[test]
    fn p2_1_explicitly_hidden_products_and_ancestors_are_blocked() {
        for html in [
            first_product_row(HTML, "hidden"),
            first_product_row(HTML, "style=\"DISPLAY: none !important\""),
            HTML.replacen("id=\"kupfer\"", "hidden id=\"kupfer\"", 1),
            HTML.replacen("id=\"kupfer\"", "style=\"display: none\" id=\"kupfer\"", 1),
            mutate_first_product("<td class=\"num\">", "<td hidden class=\"num\">"),
            mutate_first_product(
                "<strong style=\"color: var(--payout);\">",
                "<strong hidden style=\"color: var(--payout);\">",
            ),
        ] {
            assert_ne!(html, HTML, "mutation must reach real fixture");
            let s = snapshot(&html);
            assert!(!s
                .outcome
                .prices
                .iter()
                .any(|p| p.material == "kupfer-millberry"));
            assert!(s
                .diagnostics
                .iter()
                .any(|d| d.contains("MET-KU-MILL: missing known article")));
        }
        assert_eq!(
            snapshot(HTML).outcome.prices.len(),
            55,
            "visually-hidden captions are legitimate"
        );
    }

    #[test]
    fn p2_2_entire_price_cell_grammar_is_strict() {
        for invalid in [
            "/kgCO2",
            "/kg nur Sondervertrag",
            "/kg ab 250 kg",
            "/kg netto",
        ] {
            assert!(
                parse(
                    &HTML.replacen("/kg</span>", &format!("{invalid}</span>"), 1),
                    IMP,
                    200
                )
                .is_err(),
                "{invalid}"
            );
        }
        for invalid in ["1.1,13 €", "011,13 €", "11.13 €", "11,130 €"] {
            assert!(
                parse(&HTML.replace("11,13 €", invalid), IMP, 200).is_err(),
                "{invalid}"
            );
        }
        assert_eq!(
            quote_cell("1.111,13 € /kg ab 10.000 kg"),
            Some((1111.13, 10000.0))
        );
        for invalid in [
            "11,13 € /kg ab 25.0 kg",
            "11,13 € /kg ab 250 kg ab 500 kg",
            "11,13 € /kg ab 250 kg netto",
            "11,13 €/kg ab 250 kg",
            "11,13 € /kg ab 250,0 kg",
            "11,13 USD /kg ab 250 kg",
        ] {
            assert_eq!(quote_cell(invalid), None, "{invalid}");
        }
    }

    #[test]
    fn p2_3_mutated_real_operator_identity_is_not_prefix_matching() {
        for (old, new) in [
            ("Industriestraße 4", "Industriestraße 40"),
            ("HRA 61664", "HRA 616640"),
            (
                "67063 Ludwigshafen am Rhein",
                "670630 Ludwigshafen am Rhein",
            ),
            ("MKM Metals e.K.", "MKM Metals e.K. Fremdfirma"),
        ] {
            assert!(parse(HTML, &IMP.replace(old, new), 200).is_err(), "{new}");
        }
        assert!(parse(
            HTML,
            &IMP.replace("HRA 61664", "HRA 61664 Registernummer: HRA 616640"),
            200
        )
        .is_err());
    }

    #[test]
    fn p2_4_whole_tin_99_offer_includes_high_purity_tableware() {
        let s = snapshot(HTML);
        let tin = s.articles.iter().find(|a| a.article == "SW10040").unwrap();
        assert_eq!(tin.material, Some("zinn-rein-formgemischt"));
        assert_eq!(
            s.outcome
                .prices
                .iter()
                .filter(|p| p.label == "Zinn 99%")
                .count(),
            1
        );
        let dir = crate::test_support::TempDbDir::new("mkm-tin-whole-fraktion");
        let db = schrott_mcp_store::PublicDb::open(dir.path()).unwrap();
        crate::pipeline::seed_metadata(&db).unwrap();
        let catalog = db.query_sql("SELECT slug, description FROM materials WHERE slug IN ('zinn', 'zinn-geschirr', 'zinn-rein-formgemischt') ORDER BY slug").unwrap();
        assert_eq!(catalog.rows.len(), 3);
        for row in catalog.rows {
            let slug = row[0].as_str().unwrap();
            let description = row[1].as_str().unwrap();
            if slug == "zinn" {
                assert_eq!(
                    description,
                    "Reinzinn und hochprozentiges Zinn (ohne Löt- und Geschirrwaren)."
                );
            } else if slug == "zinn-rein-formgemischt" {
                for part in [
                    "Anoden",
                    "Barren",
                    "Geschirr",
                    "Keine niedrigprozentigen Zinnlegierungen",
                    "nicht nur Geschirr",
                ] {
                    assert!(
                        description.contains(part),
                        "catalog must describe whole source fraction: {part}"
                    );
                }
            }
        }
    }

    #[test]
    fn every_article_partitioned_with_source_contract() {
        let s = snapshot(HTML);
        let oracle: serde_json::Value = serde_json::from_str(ORACLE).unwrap();
        let expected: BTreeSet<_> = oracle["expected"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        let actual: BTreeSet<_> = s.articles.iter().map(|a| a.article.as_str()).collect();
        assert_eq!(actual, expected);
        assert_eq!(s.articles.len(), 60);
        assert_eq!(s.outcome.prices.len(), 55);
        assert_eq!(s.outcome.acceptances.len(), 5);
        assert_eq!(s.report()["groups"], 8);
        assert!(s.diagnostics.is_empty());
        assert_eq!(s.outcome.published_at, None);
        assert_eq!(
            s.page_change_date.as_deref(),
            Some("2026-10-09T00:00:00+00:00")
        );
        assert_eq!(s.outcome.trader_info.street, "Industriestraße 4");
        assert_eq!(s.outcome.trader_info.postcode, "67063");
        assert_eq!(s.outcome.trader_info.phone, "0176 220 168 40");
        for a in &s.articles {
            assert!(a.material.is_some());
            let reviewed = &oracle["expected"][&a.article];
            assert_eq!(
                a.material,
                reviewed[0].as_str(),
                "independent material oracle: {}",
                a.article
            );
            assert_eq!(
                a.variant,
                reviewed[1].as_str().unwrap(),
                "independent variant oracle: {}",
                a.article
            );
            if a.request_only {
                assert!(a.tiers.is_empty());
                assert!(a.price_eur_per_kg.is_none());
                assert!(!s.outcome.prices.iter().any(|p| p.label == a.label));
                assert!(s
                    .outcome
                    .acceptances
                    .iter()
                    .any(|p| p.label == a.label && p.conditions.contains(&a.article)));
            } else {
                let p = s
                    .outcome
                    .prices
                    .iter()
                    .find(|p| p.label == a.label)
                    .expect("mapped numeric article");
                assert_eq!(p.material, a.material.unwrap());
                assert_eq!(p.variant, a.variant);
                assert_eq!(p.price_kind, "upto");
                assert_eq!(p.price_max, a.price_eur_per_kg);
                assert_eq!(p.price_min, None);
                assert_eq!(p.unit, "EUR/kg");
                assert_eq!(p.confidence, Some(0.5));
                assert_eq!(
                    a.tiers.last().unwrap().ab,
                    a.minimum_for_upper_bound_kg.unwrap()
                );
            }
        }
        let mill = s
            .articles
            .iter()
            .find(|a| a.article == "MET-KU-MILL")
            .unwrap();
        assert_eq!(mill.product_change_date.as_deref(), Some("2026-09-28"));
        assert_eq!(
            mill.tiers,
            vec![
                Tier {
                    ab: 0.0,
                    preis: 10.6
                },
                Tier {
                    ab: 40.0,
                    preis: 10.81
                },
                Tier {
                    ab: 100.0,
                    preis: 10.97
                },
                Tier {
                    ab: 250.0,
                    preis: 11.13
                }
            ]
        );
        let dirty = s.articles.iter().find(|a| a.article == "SW10003").unwrap();
        assert_eq!(dirty.minimum_for_upper_bound_kg, Some(10000.0));
        assert_ne!(
            dirty.variant,
            s.articles
                .iter()
                .find(|a| a.article == "SW10002")
                .unwrap()
                .variant
        );
        let motor = s.articles.iter().find(|a| a.article == "SW10033").unwrap();
        assert!(motor.label.contains("max. 500KG"));
        assert_eq!(motor.minimum_for_upper_bound_kg, Some(500.0));
        assert_eq!(s.report()["production_ready"], false);
    }

    #[test]
    fn dangerous_cross_material_guesses_never_happen() {
        let s = snapshot(HTML);
        for (id, material) in [
            ("SW10021", "messing"),
            ("SW10022", "messing-leicht"),
            ("MKM-NE-06", "wolfram"),
            ("MKM-NE-05", "nickel"),
            ("MKM-NE-02", "neodym-magnete"),
            ("SW10017", "cu-ms-kuehler"),
            ("SW10011", "alu-cu-kuehler"),
            ("SW10042", "loetzinn"),
            ("SW10041", "loetzinn"),
            ("SW10039", "zinn-legierungen"),
            ("SW10030", "blei-gemischt"),
            ("SW10038", "hss-werkzeuge"),
            ("MKM-SCH-03", "metallschrott-gemischt"),
        ] {
            assert_eq!(
                s.articles
                    .iter()
                    .find(|a| a.article == id)
                    .unwrap()
                    .material,
                Some(material),
                "{id}"
            );
        }
        let (_, old_rows, _) = super::super::parse(HTML).unwrap();
        for (label, price, _) in old_rows {
            if let Some((material, variant)) = super::super::grade_for(&label) {
                let p = s.outcome.prices.iter().find(|p| p.label == label).unwrap();
                if label == "Kupfer Kerze" {
                    assert_eq!(
                        (material, variant),
                        ("kupfer-berry", "Kerze"),
                        "legacy domain error identified, not approved"
                    );
                    assert_eq!(
                        (p.material, p.variant.as_ref()),
                        ("kupfer-stanzabfaelle", "Kerze")
                    );
                    assert_eq!(
                        s.report()["required_alias_migrations"][0]["article"],
                        "MKM-CU-01"
                    );
                    continue;
                }
                assert_eq!(
                    (p.material, p.variant.as_ref(), p.price),
                    (material, variant, price),
                    "legacy identity {label}"
                );
            }
        }
    }

    #[test]
    fn same_count_unknown_and_changed_identity_are_not_complete() {
        let s = snapshot(&HTML.replacen("MKM-CU-02", "NEW-CU-02", 1));
        assert_eq!(s.articles.len(), 60);
        assert_eq!(s.outcome.prices.len(), 54);
        assert!(s
            .diagnostics
            .iter()
            .any(|d| d.contains("NEW-CU-02: unknown article")));
        assert!(s
            .diagnostics
            .iter()
            .any(|d| d.contains("MKM-CU-02: missing known article")));
        let s = snapshot(&HTML.replace("Kupfergranulat</a>", "Kupfergranulat neu</a>"));
        assert!(s
            .diagnostics
            .iter()
            .any(|d| d.contains("MKM-CU-02: changed identity")));
        let s = snapshot(&HTML.replace(
            "href=\"/metallankauf/produkt/kupfergranulat\"",
            "href=\"/metallankauf/produkt/anderer-artikel\"",
        ));
        assert!(s
            .diagnostics
            .iter()
            .any(|d| d.contains("MKM-CU-02: changed identity")));
    }

    #[test]
    fn missing_and_duplicate_articles_cannot_authorize_retirement() {
        let s = snapshot(&HTML.replace(
            "Ankaufpreise für Hartmetall",
            "Verkaufpreise für Hartmetall",
        ));
        assert_eq!(s.articles.len(), 55);
        assert!(s
            .diagnostics
            .iter()
            .any(|d| d.contains("SW10038: missing known article; retirement NOT authorized")));
        assert!(parse(&HTML.replacen("MKM-CU-02", "MET-KU-MILL", 1), IMP, 200).is_err());
        assert!(parse(HTML, IMP, 206).is_err());
        assert!(parse(
            "<table><caption>Ankaufpreise für Kupfer</caption><tbody></tbody></table>",
            IMP,
            200
        )
        .is_err());
    }

    #[test]
    fn malformed_price_unit_quantity_and_tiers_fail_closed() {
        // First occurrence can be a section header decoy, so mutate all
        // occurrences to ensure the real visible product cell is exercised.
        assert!(parse(&HTML.replace("11,13 €", "0,00 €"), IMP, 200).is_err());
        assert!(parse(&HTML.replacen("/kg</span>", "/t</span>", 1), IMP, 200).is_err());
        assert!(parse(
            &HTML.replacen("ab 250 kg</span>", "ab 300 kg</span>", 1),
            IMP,
            200
        )
        .is_err());
        assert!(parse(
            &HTML.replacen("&quot;preis&quot;:11.13", "&quot;preis&quot;:99", 1),
            IMP,
            200
        )
        .is_err());
        assert!(parse(
            &HTML.replacen("data-price=\"10.6000\"", "data-price=\"99\"", 1),
            IMP,
            200
        )
        .is_err());
        assert!(parse(&HTML.replace("11,13 €", "-11,13 €"), IMP, 200).is_err());
        assert!(parse(
            &HTML.replacen("ab 250 kg</span>", "ab -250 kg</span>", 1),
            IMP,
            200
        )
        .is_err());
        assert!(parse(
            HTML,
            &IMP.replace("Inhaber: Mubarik Ahmad", "Inhaber: Andere Firma"),
            200
        )
        .is_err());
    }

    #[test]
    fn selling_fee_and_navigation_decoys_are_ignored() {
        let extra = "<table><caption>Verkaufpreise für Kupfer</caption><tbody><tr><th scope=row><a href='/metallankauf/produkt/fee'>Versand 6 Euro</a></th><td>FEE</td><td>6 €/kg</td><td></td><td></td></tr></tbody></table>";
        let s = snapshot(&(HTML.to_owned() + extra));
        assert_eq!(s.articles.len(), 60);
        assert_eq!(s.outcome.prices.len(), 55);
        assert!(s.diagnostics.is_empty());
    }

    #[tokio::test]
    async fn typed_article_outcome_roundtrips_metadata_through_real_record() {
        let dir = crate::test_support::TempDbDir::new("mkm-typed-roundtrip");
        let public = schrott_mcp_store::PublicDb::open(dir.path()).unwrap();
        let internal = schrott_mcp_store::InternalDb::open(dir.path()).unwrap();
        crate::seed_metadata(&public).unwrap();
        let snapshot = snapshot(HTML);
        let outcome = crate::traders::RecordOutcome::MkmArticles(snapshot);
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-11T01:00:00+00:00")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let result =
            crate::traders::record_outcome(&public, &internal, super::super::SLUG, &outcome, &now)
                .await
                .unwrap();
        assert_eq!((result.recorded, result.accepted), (55, 5));
        let tid = public.find_trader_id(super::super::SLUG).unwrap().unwrap();
        let mid = public
            .find_material_id("kupfer-millberry")
            .unwrap()
            .unwrap();
        let current = public.current_price_for(tid, mid, "").unwrap().unwrap();
        let extra: serde_json::Value = serde_json::from_str(&current.extra_json).unwrap();
        assert_eq!(extra["article"]["article"], "MET-KU-MILL");
        assert_eq!(extra["article"]["minimum_for_upper_bound_kg"], 250.0);
        assert_eq!(extra["article"]["tiers"].as_array().unwrap().len(), 4);
        assert_eq!(extra["article"]["product_change_date"], "2026-09-28");
        assert_eq!(extra["page_change_date"], "2026-10-09T00:00:00+00:00");
        assert_eq!(current.published_at, None);
    }

    #[tokio::test]
    async fn real_tempdb_record_path_proves_catalog_coverage_and_shared_seam_gaps() {
        // Historical evidence of the old generic writer's gaps. Production
        // record() now rejects this lossy projection; only this test calls
        // the private legacy implementation to preserve the original repro.
        use crate::traders::record_observation as record;
        use schrott_mcp_store::{InternalDb, PublicDb};

        let dir = crate::test_support::TempDbDir::new("mkm-full-proof-only");
        let public = PublicDb::open(dir.path()).unwrap();
        let internal = InternalDb::open(dir.path()).unwrap();
        // Actual additive catalog and actual dossier trader seed, no mock
        // materials, no production path, no scheduler, no geocoding.
        crate::pipeline::seed_metadata(&public).unwrap();
        let s = snapshot(HTML);
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-10T12:00:00+00:00")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let tid = public.find_trader_id(super::super::SLUG).unwrap().unwrap();
        for a in &s.articles {
            let mid = public
                .find_material_id(a.material.unwrap())
                .unwrap()
                .expect("real catalog material");
            assert!(mid > 0);
        }
        // Seed the historical wrong key and a no-longer-delivered key in
        // this disposable DB only. They must remain until shared retirement.
        let mut old_plug = s
            .outcome
            .prices
            .iter()
            .find(|p| p.variant == "Stecker 30%")
            .unwrap()
            .clone();
        old_plug.material = "kabel-kupfer";
        let mut withdrawn = old_plug.clone();
        withdrawn.variant = "historisch entfernt".into();
        let historical = HandlerOutcome {
            prices: vec![old_plug, withdrawn],
            fetch_url: URL.to_owned(),
            ..HandlerOutcome::default()
        };
        let first = now - chrono::Duration::days(1);
        assert_eq!(
            record(&public, &internal, super::super::SLUG, &historical, &first)
                .await
                .unwrap()
                .recorded,
            2
        );
        let result = record(&public, &internal, super::super::SLUG, &s.outcome, &now)
            .await
            .unwrap();
        assert_eq!(result.recorded, 55);
        assert_eq!(result.accepted, 5);
        assert!(result.skipped.is_empty(), "{:?}", result.skipped);
        for p in &s.outcome.prices {
            let mid = public.find_material_id(p.material).unwrap().unwrap();
            let current = public
                .current_price_for(tid, mid, p.variant.as_ref())
                .unwrap()
                .unwrap();
            assert_eq!(current.price, p.price);
            assert_eq!(current.price_kind, "upto");
            assert_eq!(current.price_max, Some(p.price));
            assert_eq!(current.notes, p.label);
            assert_eq!(current.source_url, URL);
            assert_eq!(
                current.published_at, None,
                "heterogeneous product dates cannot be passed as page date"
            );
        }
        for a in &s.outcome.acceptances {
            let mid = public.find_material_id(a.material).unwrap().unwrap();
            assert_eq!(
                public.existing_acceptance(tid, mid).unwrap(),
                Some((true, a.conditions.clone()))
            );
            assert!(public
                .current_price_for(tid, mid, &a.label)
                .unwrap()
                .is_none());
        }
        // Explicitly verify what remains unsafe rather than calling this a
        // complete production fix. Append-only history is not touched.
        let mid = public.find_material_id("kabel-kupfer").unwrap().unwrap();
        assert!(public
            .current_price_for(tid, mid, "Stecker 30%")
            .unwrap()
            .is_some());
        assert!(public
            .current_price_for(tid, mid, "historisch entfernt")
            .unwrap()
            .is_some());
        assert_eq!(
            public
                .price_history(tid, mid, "Stecker 30%", 10)
                .unwrap()
                .len(),
            1
        );
        let mill_id = public
            .find_material_id("kupfer-millberry")
            .unwrap()
            .unwrap();
        let mill = public.current_price_for(tid, mill_id, "").unwrap().unwrap();
        let extra: serde_json::Value = serde_json::from_str(&mill.extra_json).unwrap();
        assert!(
            extra.get("article").is_none(),
            "legacy record loses article identity"
        );
        assert!(
            extra.get("tiers").is_none(),
            "legacy record loses quantity contract"
        );
        assert!(
            extra.get("product_change_date").is_none(),
            "legacy record loses product date"
        );
        assert_eq!(
            record(
                &public,
                &internal,
                super::super::SLUG,
                &s.outcome,
                &(now + chrono::Duration::hours(1))
            )
            .await
            .unwrap()
            .recorded,
            55
        );
        assert_eq!(
            public.price_history(tid, mill_id, "", 10).unwrap().len(),
            1,
            "unchanged outcome dedupes"
        );
    }
}
