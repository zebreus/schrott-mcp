//! WKR GmbH (Köln-Longerich/Rodenkirchen/Niehl): `/preise/` renders eleven
//! counter modules with zero placeholders; its first-party page-layout JS
//! carries and animates the displayed price values. Read that linked asset
//! without executing it, bind settings to each module ID, exact label and
//! unit, and fail closed on source drift. The page states prices apply from
//! 50 kg per grade but gives no quote date, so `published_at` stays `None`.

use std::collections::{HashMap, HashSet};

use scraper::{Html, Selector};

use super::super::{
    fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-koln-wkr";
/// Bespoke, live-verified price URL (Footer-Link "Altmetall-Preise"). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://www.wkr-schrott.de/preise/";
/// Bespoke, live-verified impressum URL (Footer-Link "Impressum"). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.wkr-schrott.de/kontakt/impressum/";
const LAYOUT_SCRIPT_PATH: &str = "/wp-content/uploads/bb-plugin/cache/240-layout.js";
const LAYOUT_SCRIPT_PREFIX: &str =
    "https://www.wkr-schrott.de/wp-content/uploads/bb-plugin/cache/240-layout.js?ver=";

const PRICE_LABELS: [&str; 11] = [
    "Alu Geschirr 10%",
    "V2A",
    "V4A",
    "Zink",
    "Mischschrott",
    "Millberry",
    "Kupferschwer",
    "Kupferkabel 40% ohne Stecker",
    "Kupferschälkabel, 50%",
    "Kupferlitzenkabel, 50%",
    "Messing",
];

#[derive(Debug)]
struct CounterModule {
    id: String,
    label: String,
    initial_value: f64,
    display_unit: String,
}

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
    let script_url = layout_script_url(&html)?;
    let script = fetch_layout_script(client, &script_url).await?;
    let prices = parse_js_prices(&html, &script)?;
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
            None => skipped_labels.push(format!("{label} (kein Katalogmaterial)")),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        // Every quote has an explicit qualified acceptance below. Do not add
        // unqualified fallback acceptances from the price-only projection.
        skip_price_acceptances: true,
        prices,
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

fn source_error(url: &str, detail: impl Into<String>) -> IngestError {
    IngestError::Parse {
        url: url.to_owned(),
        detail: detail.into(),
    }
}

/// Prices are only quoted for at least 50 kg per grade; smaller quantities
/// receive a deduction. Since variants carry this qualification, require both
/// source statements before publishing any prices.
fn validate_price_terms(html: &str) -> Result<(), IngestError> {
    let doc = Html::parse_document(html);
    let paragraph_selector = Selector::parse("p").expect("valid selector");
    let paragraphs = doc
        .select(&paragraph_selector)
        .map(|paragraph| normalized_text(&paragraph.text().collect::<String>()))
        .collect::<Vec<_>>();
    for required in [
        "gelten für eine Abgabemenge ab 50 kg je Sorte",
        "Bei kleineren Menge erfolgt ein geringer Preisabzug",
    ] {
        if !paragraphs.iter().any(|text| text.contains(required)) {
            return Err(source_error(
                URL,
                format!("WKR price conditions changed; missing: {required}"),
            ));
        }
    }
    Ok(())
}

/// The builder's page-specific JS is the only source of the counter values.
/// Require the exact first-party layout asset and a version query parameter.
fn layout_script_url(html: &str) -> Result<String, IngestError> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("script[src]").expect("valid selector");
    let scripts: Vec<_> = doc
        .select(&selector)
        .filter_map(|script| script.value().attr("src"))
        .filter(|src| src.contains(LAYOUT_SCRIPT_PATH))
        .collect();
    if scripts.len() != 1 {
        return Err(source_error(
            URL,
            format!(
                "expected one WKR page layout script, found {}",
                scripts.len()
            ),
        ));
    }
    let src = scripts[0];
    let Some(version) = src.strip_prefix(LAYOUT_SCRIPT_PREFIX) else {
        return Err(source_error(URL, "WKR layout script URL changed"));
    };
    if version.is_empty()
        || !version
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
    {
        return Err(source_error(URL, "WKR layout script version is invalid"));
    }
    Ok(src.to_owned())
}

/// Reject script redirects and non-200 responses. The page's values are in
/// executable first-party JS, so a moved asset must not silently become input.
async fn fetch_layout_script(client: &reqwest::Client, url: &str) -> Result<String, IngestError> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|source| IngestError::Fetch {
            url: url.to_owned(),
            source,
        })?;
    let status = response.status().as_u16();
    if response.url().as_str() != url || status != 200 {
        return Err(source_error(
            url,
            format!(
                "changed WKR layout script identity/status: {} HTTP {status}",
                response.url()
            ),
        ));
    }
    response.text().await.map_err(|source| IngestError::Body {
        url: url.to_owned(),
        source,
    })
}

/// Extract prices from UABBNumber settings without executing the source JS.
/// Module IDs bind labels and units to values; conflicting, missing, zero or
/// otherwise unreviewed values reject the complete quote snapshot.
fn parse_js_prices(html: &str, script: &str) -> Result<Vec<ScrapedPrice>, IngestError> {
    let modules = parse_counter_modules(html)?;
    let settings = counter_settings(script, &modules)?;
    let mut prices = Vec::with_capacity(modules.len());
    for module in modules {
        let setting = settings
            .get(&module.id)
            .expect("counter_settings validates each module");
        if module.initial_value > 0.0 && (module.initial_value - setting).abs() > 0.000_001 {
            return Err(source_error(
                URL,
                format!("static and JS price disagree for {}", module.label),
            ));
        }
        let (material, variant, unit, display_unit) =
            price_spec(&module.label).expect("parse_counter_modules validates exact price labels");
        if module.display_unit != display_unit {
            return Err(source_error(
                URL,
                format!("unexpected price unit for {}", module.label),
            ));
        }
        prices.push(ScrapedPrice {
            material,
            variant: variant.into(),
            price: *setting,
            currency: "EUR",
            unit,
            price_kind: "exact",
            price_min: None,
            price_max: None,
            confidence: Some(1.0),
            label: module.label,
        });
    }
    Ok(prices)
}

fn parse_counter_modules(html: &str) -> Result<Vec<CounterModule>, IngestError> {
    validate_price_terms(html)?;
    let start = html
        .find("Altmetall-Preise")
        .ok_or_else(|| source_error(URL, "Preisblock fehlt"))?;
    let tail = &html[start..];
    let end = tail
        .find("Die angegebenen Preise sind freibleibend")
        .ok_or_else(|| source_error(URL, "Preisblock unvollständig"))?;
    let doc = Html::parse_fragment(&format!("<div>{}</div>", &tail[..end]));
    let module_selector =
        Selector::parse("div.fl-module-uabb-numbers[data-node]").expect("valid selector");
    let label_selector = Selector::parse("span.uabb-number-before-text").expect("valid selector");
    let price_selector = Selector::parse("h2.uabb-number-string").expect("valid selector");
    let value_selector = Selector::parse("span.uabb-number-int").expect("valid selector");
    let mut modules = Vec::new();
    let mut ids = HashSet::new();
    for module in doc.select(&module_selector) {
        let Some(id) = module.value().attr("data-node").filter(|id| !id.is_empty()) else {
            return Err(source_error(URL, "WKR counter module ID missing"));
        };
        if !ids.insert(id.to_owned()) {
            return Err(source_error(URL, "duplicate WKR counter module ID"));
        }
        let labels: Vec<_> = module.select(&label_selector).collect();
        let prices: Vec<_> = module.select(&price_selector).collect();
        if labels.len() != 1 || prices.len() != 1 {
            return Err(source_error(URL, "WKR counter module shape changed"));
        }
        let label = normalized_text(labels[0].text().collect::<String>().as_str());
        let Some((_, _, _, expected_display_unit)) = price_spec(&label) else {
            return Err(source_error(
                URL,
                format!("unreviewed WKR price label: {label}"),
            ));
        };
        let values: Vec<_> = prices[0].select(&value_selector).collect();
        if values.len() != 1 {
            return Err(source_error(
                URL,
                format!("WKR counter value missing or duplicated for {label}"),
            ));
        }
        let value_text = normalized_text(values[0].text().collect::<String>().as_str());
        let initial_value = super::super::parse_eur(&value_text)
            .ok_or_else(|| source_error(URL, format!("invalid static counter for {label}")))?;
        if !initial_value.is_finite() || initial_value < 0.0 {
            return Err(source_error(
                URL,
                format!("invalid static counter for {label}"),
            ));
        }
        let full_text = normalized_text(prices[0].text().collect::<String>().as_str());
        let Some(display_unit) = full_text.strip_prefix(&value_text) else {
            return Err(source_error(
                URL,
                format!("counter text changed for {label}"),
            ));
        };
        let display_unit = normalized_text(display_unit);
        if display_unit != expected_display_unit {
            return Err(source_error(
                URL,
                format!("unexpected price unit for {label}"),
            ));
        }
        modules.push(CounterModule {
            id: id.to_owned(),
            label,
            initial_value,
            display_unit,
        });
    }
    let labels: Vec<_> = modules.iter().map(|module| module.label.as_str()).collect();
    if labels != PRICE_LABELS {
        return Err(source_error(
            URL,
            format!("WKR price label sequence changed: {labels:?}"),
        ));
    }
    Ok(modules)
}

fn normalized_text(text: &str) -> String {
    text.replace('\u{a0}', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn price_spec(label: &str) -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    let spec = match label {
        "Alu Geschirr 10%" => (
            "aluminium-blech",
            "Geschirr 10%, ab 50 kg",
            "EUR/kg",
            "€ / kg",
        ),
        "V2A" => ("edelstahl-v2a", "ab 50 kg", "EUR/kg", "€ / kg"),
        "V4A" => ("edelstahl-v4a", "ab 50 kg", "EUR/kg", "€ / kg"),
        "Zink" => ("zink", "ab 50 kg", "EUR/kg", "€ / kg"),
        "Mischschrott" => ("mischschrott", "ab 50 kg", "EUR/t", "€ / t."),
        "Millberry" => ("kupfer-millberry", "ab 50 kg", "EUR/kg", "€ / kg"),
        "Kupferschwer" => ("kupfer-gemischt", "schwer, ab 50 kg", "EUR/kg", "€ / kg"),
        "Kupferkabel 40% ohne Stecker" => (
            "kabel-kupfer",
            "40% ohne Stecker, ab 50 kg",
            "EUR/kg",
            "€ / kg",
        ),
        "Kupferschälkabel, 50%" => (
            "kabel-kupfer",
            "Schälkabel 50%, ab 50 kg",
            "EUR/kg",
            "€ / kg",
        ),
        "Kupferlitzenkabel, 50%" => (
            "kabel-kupfer",
            "Litzenkabel 50%, ab 50 kg",
            "EUR/kg",
            "€ / kg",
        ),
        "Messing" => ("messing", "ab 50 kg", "EUR/kg", "€ / kg"),
        _ => return None,
    };
    Some(spec)
}

fn counter_settings(
    script: &str,
    modules: &[CounterModule],
) -> Result<HashMap<String, f64>, IngestError> {
    for marker in [
        "UABBNumber=function(settings)",
        "this.number=settings.number",
        "find('.uabb-number-int')",
        "animate({Counter:self.number}",
        "$string.text($counter)",
    ] {
        if !script.contains(marker) {
            return Err(source_error(URL, "WKR counter runtime changed"));
        }
    }

    let module_ids: HashSet<_> = modules.iter().map(|module| module.id.as_str()).collect();
    let mut configs: HashMap<String, Vec<(f64, bool)>> = HashMap::new();
    for call in script.split("new UABBNumber").skip(1) {
        let call = call
            .trim_start()
            .strip_prefix('(')
            .and_then(|call| call.trim_start().strip_prefix('{'))
            .ok_or_else(|| source_error(URL, "WKR counter initializer changed"))?;
        let end = call
            .find("})")
            .ok_or_else(|| source_error(URL, "WKR counter settings are malformed"))?;
        let object = &call[..end];
        if let Some(id) = js_string_property(object, "id") {
            if module_ids.contains(id) {
                let value = js_number_property(object, "number")
                    .filter(|value| value.is_finite() && *value > 0.0)
                    .ok_or_else(|| {
                        source_error(URL, format!("invalid JS counter value for module {id}"))
                    })?;
                let layout = js_string_property(object, "layout");
                let kind = js_string_property(object, "type");
                let maximum = js_number_property(object, "max");
                let format = js_string_property(object, "numberFormat");
                let locale = js_string_property(object, "locale");
                if layout != Some("default")
                    || kind != Some("standard")
                    || maximum.is_none_or(|max| (max - value).abs() > 0.000_001)
                    || format.is_some_and(|format| format != "none")
                    || locale.is_some_and(|locale| locale != "de_DE")
                {
                    return Err(source_error(
                        URL,
                        format!("WKR counter format changed for module {id}"),
                    ));
                }
                configs
                    .entry(id.to_owned())
                    .or_default()
                    .push((value, format == Some("none") && locale == Some("de_DE")));
            }
        }
    }

    let mut settings = HashMap::new();
    for module in modules {
        let Some(candidates) = configs.get(&module.id) else {
            return Err(source_error(
                URL,
                format!("WKR JS price missing for {}", module.label),
            ));
        };
        let value = candidates[0].0;
        if candidates
            .iter()
            .any(|candidate| (candidate.0 - value).abs() > 0.000_001)
            || !candidates.iter().any(|candidate| candidate.1)
        {
            return Err(source_error(
                URL,
                format!(
                    "conflicting or incomplete WKR JS price for {}",
                    module.label
                ),
            ));
        }
        settings.insert(module.id.clone(), value);
    }
    Ok(settings)
}

fn js_string_property<'a>(object: &'a str, key: &str) -> Option<&'a str> {
    let value = js_object_property(object, key)?;
    let quote = value.as_bytes().first().copied()?;
    if !matches!(quote, b'\'' | b'"') || value.as_bytes().last().copied() != Some(quote) {
        return None;
    }
    Some(&value[1..value.len() - 1])
}

fn js_number_property(object: &str, key: &str) -> Option<f64> {
    js_object_property(object, key)?.parse().ok()
}

fn js_object_property<'a>(object: &'a str, key: &str) -> Option<&'a str> {
    object.split(',').find_map(|field| {
        let (name, value) = field.split_once(':')?;
        (name.trim() == key).then_some(value.trim())
    })
}

/// Explicit label → acceptances. One label may fan out to several materials;
/// anything unlisted skips loudly. "Kupferschwer" (blankes, schweres Kupfer)
/// ist kein Kabel und landet auf `kupfer-gemischt`, die drei Kabelsorten mit
/// genannter Kupferausbeute auf `kabel-kupfer` (Bedingung hält die Ausbeute,
/// das Rohlabel steht ohnehin im Record).
fn grade_for(label: &str) -> Option<Vec<(&'static str, &'static str)>> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("millberry") {
        Some(vec![("kupfer-millberry", "")])
    } else if l.contains("kupferschwer") {
        Some(vec![("kupfer-gemischt", "schwer")])
    } else if l.contains("kabel") {
        // Ambiguous mixes ("Kabel / E-Motoren") name two catalog areas and
        // stay out loudly — a wrong acceptance is worse than a logged gap.
        if l.contains("motor") || l.contains("elektro") {
            return None;
        }
        // "Kupferkabel 40% ohne Stecker", "Kupferschälkabel, 50%",
        // "Kupferlitzenkabel, 50%": Ausbeute als Bedingung.
        if l.contains("40%") {
            Some(vec![("kabel-kupfer", "40% ohne Stecker")])
        } else if l.contains("schälkabel") || l.contains("schalkabel") {
            Some(vec![("kabel-kupfer", "Schälkabel 50%")])
        } else if l.contains("litzen") {
            Some(vec![("kabel-kupfer", "Litzenkabel 50%")])
        } else {
            Some(vec![("kabel-kupfer", "")])
        }
    } else if l.contains("v4a") {
        Some(vec![("edelstahl-v4a", "")])
    } else if l.contains("v2a") {
        Some(vec![("edelstahl-v2a", "")])
    } else if l.contains("zink") {
        Some(vec![("zink", "")])
    } else if l.contains("messing") {
        Some(vec![("messing", "")])
    } else if l.contains("mischschrott") {
        Some(vec![("mischschrott", "")])
    } else if l.contains("alu") && l.contains("geschirr") {
        Some(vec![("aluminium-blech", "Geschirr")])
    } else {
        None
    }
}

/// Extract the reviewed labels from the counter modules. Their static HTML
/// values may be zero placeholders; `parse_js_prices` binds actual prices to
/// these exact module IDs, labels, units, and published conditions.
fn parse(html: &str) -> Result<Vec<String>, IngestError> {
    Ok(parse_counter_modules(html)?
        .into_iter()
        .map(|module| module.label)
        .collect())
}

/// Bespoke contact extraction for THIS impressum only: the first `<p>` holding
/// "WKR GmbH" + "Robert-Bosch-Straße" carries firm + street + PLZ city as
/// `<br>` lines ("WKR GmbH" / "Robert-Bosch-Straße 20-22" / "50739
/// Köln-Longerich"), and the `<p>` after the `<h2>Kontakt</h2>` heading holds
/// "Telefon: …" + "E-Mail: …" as `<br>` lines. Missing anchors mean the page
/// changed shape → loud error, never a guessed fallback. The Longerich
/// address wins (seed city is Köln).
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    if !doc
        .select(&h2)
        .any(|el| el.text().collect::<String>().trim() == "Kontakt")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    }
    // Address lines from the <br>-split firm paragraph (drop up to the first
    // '>' first so tag attributes never parse as text).
    let addr_p = doc.select(&p).find(|el| {
        let t: String = el.text().collect();
        t.contains("WKR GmbH") && t.contains("Robert-Bosch")
    });
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let lines: Vec<String> = addr_p
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let mut street = String::new();
    let (mut postcode, mut city) = (String::new(), String::new());
    for line in &lines {
        if line.contains("Stra") && street.is_empty() {
            street = line.clone();
        }
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = std::iter::once(ci).chain(it).collect::<Vec<_>>().join(" ");
            }
        }
    }
    // Contact lines from the <br>-split paragraph after the Kontakt heading.
    let contact_p = doc.select(&p).find(|el| {
        let t: String = el.text().collect();
        t.contains("Telefon:") && t.contains('@')
    });
    let (mut phone, mut email) = (String::new(), String::new());
    if let Some(el) = contact_p {
        for line in el
            .inner_html()
            .split("<br")
            .map(strip_fragment)
            .filter(|s| !s.is_empty())
        {
            if let Some(rest) = line.strip_prefix("Telefon:") {
                phone = rest.trim().to_owned();
            } else if let Some(rest) = line.strip_prefix("E-Mail:") {
                email = rest.trim().to_owned();
            }
        }
    }
    if email.is_empty() {
        // Plain-text fallback: first @-token in the contact paragraph.
        if let Some(el) = contact_p {
            let t: String = el.text().collect();
            if let Some(tok) = t.split_whitespace().find(|w| w.contains('@')) {
                email = tok.trim_matches([',', ';', '.']).to_owned();
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

/// Strip tags from a `<br>`-split fragment (drop up to the first '>'
/// first so attributes never parse as text).
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

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, parse_js_prices};

    const MODULES: [(&str, &str, &str); 11] = [
        ("61a5eb4e45517", "Alu Geschirr 10%", "kg"),
        ("61a5e85f9b4c5", "V2A", "kg"),
        ("61a5e8825d698", "V4A", "kg"),
        ("61a5e89fb5765", "Zink", "kg"),
        ("61a5e8d0caa6c", "Mischschrott", "t."),
        ("61a5e8bfc6884", "Millberry", "kg"),
        ("61a5ea6ebb614", "Kupferschwer", "kg"),
        ("61a5eac7d949f", "Kupferkabel 40% ohne Stecker", "kg"),
        ("61a5eaea336a4", "Kupferschälkabel, 50%", "kg"),
        ("61a5eb07ed714", "Kupferlitzenkabel, 50%", "kg"),
        ("61a5eb38b92be", "Messing", "kg"),
    ];

    const CLIENT_COUNTERS: &str = concat!(
        "UABBNumber=function(settings){this.number=settings.number;find('.uabb-number-int');animate({Counter:self.number});$string.text($counter);};",
        "new UABBNumber({id:'61a5eb4e45517',layout:'default',type:'standard',number:1.20,numberFormat:'none',locale:'de_DE',max:1.20});",
        "new UABBNumber({id:'61a5e85f9b4c5',layout:'default',type:'standard',number:0.70,numberFormat:'none',locale:'de_DE',max:0.70});",
        "new UABBNumber({id:'61a5e8825d698',layout:'default',type:'standard',number:1.40,numberFormat:'none',locale:'de_DE',max:1.40});",
        "new UABBNumber({id:'61a5e89fb5765',layout:'default',type:'standard',number:1.50,numberFormat:'none',locale:'de_DE',max:1.50});",
        "new UABBNumber({id:'61a5e8d0caa6c',layout:'default',type:'standard',number:120,numberFormat:'none',locale:'de_DE',max:120});",
        "new UABBNumber({id:'61a5e8bfc6884',layout:'default',type:'standard',number:10.50,numberFormat:'none',locale:'de_DE',max:10.50});",
        "new UABBNumber({id:'61a5ea6ebb614',layout:'default',type:'standard',number:9.50,numberFormat:'none',locale:'de_DE',max:9.50});",
        "new UABBNumber({id:'61a5eac7d949f',layout:'default',type:'standard',number:3.80,numberFormat:'none',locale:'de_DE',max:3.80});",
        "new UABBNumber({id:'61a5eaea336a4',layout:'default',type:'standard',number:4.60,numberFormat:'none',locale:'de_DE',max:4.60});",
        "new UABBNumber({id:'61a5eb07ed714',layout:'default',type:'standard',number:4.10,numberFormat:'none',locale:'de_DE',max:4.10});",
        "new UABBNumber({id:'61a5eb38b92be',layout:'default',type:'standard',number:5.70,numberFormat:'none',locale:'de_DE',max:5.70});",
        "new UABBNumber({id:'61a5eb4e45517',layout:'default',type:'standard',number:1.20,max:1.20});"
    );

    fn client_price_html() -> String {
        let mut html = String::from(
            "<h2>Altmetall-Preise</h2><script src=\"https://www.wkr-schrott.de/wp-content/uploads/bb-plugin/cache/240-layout.js?ver=reviewed\"></script>",
        );
        for (id, label, unit) in MODULES {
            html.push_str(&format!(
                "<div class=\"fl-module fl-module-uabb-numbers\" data-node=\"{id}\"><span class=\"uabb-number-before-text\">{label}</span><h2 class=\"uabb-number-string\"><span class=\"uabb-number-int\">0</span> € / {unit}</h2></div>"
            ));
        }
        html.push_str(
            "<p>Die angegebenen Preise gelten für eine Abgabemenge ab 50 kg je Sorte. \
             Bei kleineren Menge erfolgt ein geringer Preisabzug. Die angegebenen Preise sind freibleibend</p>",
        );
        html
    }

    #[test]
    fn client_counter_settings_become_exact_typed_prices_by_module_id() {
        assert_eq!(
            super::layout_script_url(&client_price_html()).unwrap(),
            "https://www.wkr-schrott.de/wp-content/uploads/bb-plugin/cache/240-layout.js?ver=reviewed"
        );
        let prices = parse_js_prices(&client_price_html(), CLIENT_COUNTERS).expect("client prices");
        let actual: Vec<_> = prices
            .iter()
            .map(|p| (p.label.as_str(), p.price, p.unit))
            .collect();
        assert_eq!(
            actual,
            vec![
                ("Alu Geschirr 10%", 1.20, "EUR/kg"),
                ("V2A", 0.70, "EUR/kg"),
                ("V4A", 1.40, "EUR/kg"),
                ("Zink", 1.50, "EUR/kg"),
                ("Mischschrott", 120.0, "EUR/t"),
                ("Millberry", 10.50, "EUR/kg"),
                ("Kupferschwer", 9.50, "EUR/kg"),
                ("Kupferkabel 40% ohne Stecker", 3.80, "EUR/kg"),
                ("Kupferschälkabel, 50%", 4.60, "EUR/kg"),
                ("Kupferlitzenkabel, 50%", 4.10, "EUR/kg"),
                ("Messing", 5.70, "EUR/kg"),
            ]
        );
        assert_eq!(prices[0].variant, "Geschirr 10%, ab 50 kg");
        assert_eq!(prices[4].variant, "ab 50 kg");
        assert_eq!(prices[6].variant, "schwer, ab 50 kg");
        assert_eq!(prices[8].variant, "Schälkabel 50%, ab 50 kg");
    }

    #[test]
    fn conflicting_or_missing_client_counter_values_fail_closed() {
        let html = client_price_html();
        let conflicting = format!(
            "{CLIENT_COUNTERS}new UABBNumber({{id:'61a5eb4e45517',layout:'default',type:'standard',number:99,max:99}});"
        );
        assert!(parse_js_prices(&html, &conflicting).is_err());
        assert!(parse_js_prices(
            &html,
            &CLIENT_COUNTERS.replace("61a5eb4e45517", "missing-node")
        )
        .is_err());
        assert!(
            parse_js_prices(&html.replace("€ / kg</h2>", "€ / t.</h2>"), CLIENT_COUNTERS).is_err()
        );
        assert!(parse_js_prices(
            &html.replace(
                "<span class=\"uabb-number-int\">0</span>",
                "<span class=\"uabb-number-int\">99</span>"
            ),
            CLIENT_COUNTERS
        )
        .is_err());
        assert!(parse_js_prices(
            &html.replace(
                "gelten für eine Abgabemenge ab 50 kg je Sorte",
                "gelten für eine Abgabemenge ab 25 kg je Sorte"
            ),
            CLIENT_COUNTERS
        )
        .is_err());
        assert!(parse_js_prices(
            &html.replace("Bei kleineren Menge erfolgt ein geringer Preisabzug.", ""),
            CLIENT_COUNTERS
        )
        .is_err());
    }

    #[tokio::test]
    #[ignore = "live WKR page and linked counter asset"]
    async fn live_handler_loads_prices_from_the_linked_counter_script() {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("HTTP client");
        let scrape = super::handler().scrape;
        let outcome = scrape(&client).await.expect("live WKR scrape");
        assert_eq!(outcome.prices.len(), 11);
        assert_eq!(outcome.acceptances.len(), 11);
        assert!(outcome.skipped_labels.is_empty());
        assert!(outcome.published_at.is_none());
        assert!(outcome.skip_price_acceptances);
        assert!(outcome.prices.iter().all(|price| price.price > 0.0));
        assert!(outcome
            .prices
            .iter()
            .all(|price| price.variant.contains("ab 50 kg")));
        assert!(outcome
            .prices
            .iter()
            .any(|price| price.label == "Mischschrott" && price.unit == "EUR/t"));
    }

    #[test]
    fn static_counter_markup_yields_reviewed_price_labels() {
        let html = client_price_html();
        let labels = parse(&html).expect("parses");
        assert_eq!(labels.len(), 11);
        assert_eq!(labels[0], "Alu Geschirr 10%");
        assert_eq!(labels[4], "Mischschrott");
        assert_eq!(labels[10], "Messing");
        // Redesign or missing published conditions fail loudly.
        assert!(parse("<div>Redesign ohne Block</div>").is_err());
        assert!(parse(&html.replace("ab 50 kg je Sorte", "ab 25 kg je Sorte")).is_err());
        assert!(
            parse(&html.replace("Bei kleineren Menge erfolgt ein geringer Preisabzug.", ""))
                .is_err()
        );
    }

    #[test]
    fn mapping_covers_live_counters() {
        assert_eq!(
            grade_for("Alu Geschirr 10%"),
            Some(vec![("aluminium-blech", "Geschirr")])
        );
        assert_eq!(grade_for("V2A"), Some(vec![("edelstahl-v2a", "")]));
        assert_eq!(grade_for("V4A"), Some(vec![("edelstahl-v4a", "")]));
        assert_eq!(grade_for("Zink"), Some(vec![("zink", "")]));
        assert_eq!(grade_for("Mischschrott"), Some(vec![("mischschrott", "")]));
        assert_eq!(grade_for("Millberry"), Some(vec![("kupfer-millberry", "")]));
        assert_eq!(
            grade_for("Kupferschwer"),
            Some(vec![("kupfer-gemischt", "schwer")])
        );
        assert_eq!(
            grade_for("Kupferkabel 40% ohne Stecker"),
            Some(vec![("kabel-kupfer", "40% ohne Stecker")])
        );
        assert_eq!(
            grade_for("Kupferschälkabel, 50%"),
            Some(vec![("kabel-kupfer", "Schälkabel 50%")])
        );
        assert_eq!(
            grade_for("Kupferlitzenkabel, 50%"),
            Some(vec![("kabel-kupfer", "Litzenkabel 50%")])
        );
        assert_eq!(grade_for("Messing"), Some(vec![("messing", "")]));
        // Unlisted labels stay out loudly.
        assert_eq!(grade_for("Aluminium"), None);
        assert_eq!(grade_for("Kabel / E-Motoren"), None);
    }

    #[test]
    fn impressum_extracts_longerich_contact() {
        // Real fragment shape of /kontakt/impressum/ (entities as served).
        let imp = "<div class=\"fl-rich-text\"><p><div style=\"word-wrap: break-word;\">\
            <p>WKR GmbH<br />Robert-Bosch-Stra&szlig;e 20-22<br />\
            50739 K&ouml;ln-Longerich</p>\
            <p>Handelsregister: HRB 63135<br />\
            Registergericht: Amtsgericht K&ouml;ln</p>\
            <h2>Kontakt</h2><p>Telefon: 0221 - 97 06 06 22<br />\
            E-Mail: info@wkr-schrott.de</p></div></p></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Robert-Bosch-Straße 20-22");
        assert_eq!(info.postcode, "50739");
        assert_eq!(info.city, "Köln-Longerich");
        assert_eq!(info.phone, "0221 - 97 06 06 22");
        assert_eq!(info.email, "info@wkr-schrott.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(extract_info("<h2>Kontakt</h2><p>Telefon: 1</p>").is_err());
    }
}
