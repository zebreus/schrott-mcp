//! VHM Hartmetall Ankauf (Remscheid): the page's script loads current
//! prices from `/vhm-preise-aktuell.php`, not the placeholder HTML cards.
//! Four fixed EUR/kg prices retain their historical variants; Schlamm
//! remains acceptance-only ("nach Analyse"). API errors, contradictory
//! amounts or unknown units fail loudly without HTML/old-price fallbacks.
//!
//! Mapping: all five bought grades map to the `hartmetall` catalog
//! material, each with its own `variant` (conditions) so nothing
//! collapses. Cramming tungsten carbide into Kupfer/Messing would corrupt
//! those histories, hence a dedicated material or nothing.

use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;
use std::collections::BTreeMap;

use super::super::{
    fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, ScrapedPrice, TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "nw-remscheid-vhm-hartmetall-ankauf";
/// Bespoke, live-verified impressum URL (the site's own `/impressum` link).
/// A move fails the step loudly — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.vhm-hartmetall.de/impressum";

pub const URL: &str = "https://www.vhm-hartmetall.de/aktueller-hartmetall-preis";
pub const API_URL: &str = "https://www.vhm-hartmetall.de/vhm-preise-aktuell.php";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PriceApi {
    ok: bool,
    prices: BTreeMap<String, ApiPrice>,
    fields: ApiFields,
    updated_at: String,
    revision: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiPrice {
    text: String,
    amount: serde_json::Value,
    schema_amount: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiFields {
    einheit: String,
    markt_hinweis: String,
}

fn parse_api(body: &str) -> Result<HandlerOutcome, IngestError> {
    let error = |detail: String| IngestError::Parse {
        url: API_URL.to_owned(),
        detail,
    };
    let mut api: PriceApi =
        serde_json::from_str(body).map_err(|e| error(format!("Preis-API JSON: {e}")))?;
    if !api.ok {
        return Err(error("Preis-API ok ist nicht true".to_owned()));
    }
    if api.fields.einheit.trim() != "alle Preise pro kg" {
        return Err(error(format!(
            "Preis-API Einheit unverständlich: {}",
            api.fields.einheit
        )));
    }
    if api.fields.markt_hinweis.trim().is_empty() || api.revision.trim().is_empty() {
        return Err(error(
            "Preis-API Markthinweis oder Revision leer".to_owned(),
        ));
    }
    let date = chrono::DateTime::parse_from_rfc3339(&api.updated_at)
        .map_err(|e| error(format!("Preis-API updatedAt: {e}")))?;
    let mut out = HandlerOutcome {
        published_at: Some(date.format("%Y-%m-%d").to_string()),
        ..HandlerOutcome::default()
    };
    for (key, label) in [
        ("wendeschneidplatten", "Wendeschneidplatten"),
        ("hartmetallGemischt", "Hartmetall gemischt"),
        ("vhmFraeserBohrer", "VHM-Fräser & VHM-Bohrer"),
        ("widia", "Widia"),
        ("hartmetallschlamm", "Hartmetallschlamm"),
    ] {
        let row = api
            .prices
            .remove(key)
            .ok_or_else(|| error(format!("Preis-API Sorte fehlt: {key}")))?;
        let (material, variant) = grade_for(label).expect("explicit mapped API label");
        if key == "hartmetallschlamm" {
            if !row.amount.is_null()
                || !row.schema_amount.is_null()
                || row.text.trim().to_lowercase() != "nach analyse"
            {
                return Err(error(
                    "Preis-API Schlamm ist nicht ohne Festpreis / nach Analyse".to_owned(),
                ));
            }
            out.acceptances.push(ScrapedAcceptance {
                material,
                conditions: variant.to_owned(),
                label: label.to_owned(),
            });
        } else {
            let price = row
                .amount
                .as_f64()
                .ok_or_else(|| error(format!("Preis-API Festpreis fehlt: {key}")))?;
            // Require three agreeing source representations, not a guessed
            // unit or an old HTML/JSON-LD fallback. Prefixes such as "bis zu"
            // are not exact prices.
            let displayed = row
                .text
                .trim()
                .strip_suffix(" €/kg")
                .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit() || c == ','))
                .and_then(|s| s.replace(',', ".").parse::<f64>().ok());
            if !price.is_finite()
                || price <= 0.0
                || row.schema_amount.as_f64() != Some(price)
                || displayed != Some(price)
            {
                return Err(error(format!(
                    "Preis-API Preis/Einheit widersprüchlich: {key}"
                )));
            }
            out.prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit: "EUR/kg",
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label: label.to_owned(),
            });
        }
    }
    out.skipped_labels.extend(
        api.prices
            .into_keys()
            .map(|key| format!("{key} (unbekannte API-Sorte)")),
    );
    Ok(out)
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
    // Match the site's Date.now() cache buster and fetch cache: no-store.
    // Keep the canonical API URL in provenance rather than the transient _.
    let response = client
        .get(API_URL)
        .query(&[("_", chrono::Utc::now().timestamp_millis())])
        .header(reqwest::header::CACHE_CONTROL, "no-cache, no-store")
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .map_err(|source| IngestError::Fetch {
            url: API_URL.to_owned(),
            source,
        })?;
    let status = response.status().as_u16();
    if !response.status().is_success() {
        return Err(IngestError::Parse {
            url: API_URL.to_owned(),
            detail: format!("HTTP {status}"),
        });
    }
    let body = response.text().await.map_err(|source| IngestError::Body {
        url: API_URL.to_owned(),
        source,
    })?;
    let mut out = parse_api(&body)?;
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    out.trader_info = extract_info(&imp_html)?;
    out.website_alive = true;
    out.fetch_url = API_URL.to_owned();
    out.status_code = status;
    out.byte_len = body.len();
    Ok(out)
}

/// Explicit label → (material, conditions) mapping. All five bought
/// grades land on `hartmetall` with the trader's own grade wording as
/// conditions (nothing collapses). Anything unlisted is skipped loudly,
/// never guessed.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    if l.contains("fräser") || l.contains("fraeser") || l.contains("bohrer") {
        Some(("hartmetall", "VHM-Fräser & Bohrer"))
    } else if l.contains("wende") || l.contains("wsp") || l.contains("insert") {
        Some(("hartmetall", "Wendeschneidplatten"))
    } else if l.contains("widia") || l.contains("stück") || l.contains("stueck") {
        Some(("hartmetall", "Widia"))
    } else if l.contains("schlamm") || l.contains("rückstand") || l.contains("rueckstand") {
        Some(("hartmetall", "Schlamm"))
    } else if l.contains("gemischt") {
        Some(("hartmetall", "gemischt"))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after the
/// "Angaben gemäß § 5 DDG" heading holds firm / name / street / PLZ city /
/// country lines, and the `<p>` after the "Kontakt" heading holds the
/// "Telefon:" / "E-Mail:" lines (numbers inside tel:/mailto: links).
/// Missing headings → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h2 = Selector::parse("h2").expect("valid selector");
    let headings: Vec<ElementRef> = doc.select(&h2).collect();
    let block_after = |title: &str| -> Option<Vec<String>> {
        headings
            .iter()
            .find(|h| h.text().collect::<String>().trim() == title)
            .and_then(|h| {
                h.next_siblings()
                    .filter_map(ElementRef::wrap)
                    .find(|e| e.value().name() == "p")
                    .map(|p| block_lines(&p.inner_html()))
            })
    };
    let Some(addr) = block_after("Angaben gemäß § 5 DDG") else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    let Some(contact) = block_after("Kontakt") else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Kontakt-Block fehlt".to_owned(),
        });
    };
    // "... Grünenplatzstraße 1a / 42899 Remscheid / Deutschland": the PLZ
    // line carries postcode + city, street is the line right before it.
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in addr.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, w| a + " " + w);
                if k > 0 {
                    street = addr[k - 1].clone();
                }
                break;
            }
        }
    }
    let mut phone = String::new();
    let mut email = String::new();
    for line in &contact {
        if let Some(v) = line.strip_prefix("Telefon:") {
            phone = v
                .split_whitespace()
                .take_while(|t| {
                    t.chars()
                        .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
                })
                .collect::<Vec<_>>()
                .join(" ");
        } else if let Some(v) = line.strip_prefix("E-Mail:") {
            // Email needs its own rule: the phone-style take_while above
            // would stop at the first letter.
            email = v.split_whitespace().next().unwrap_or_default().to_owned();
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

/// Split an inner-HTML block on `<br` into plain-text lines. Newlines are
/// planted BEFORE stripping tags so the "Telefon:"/"E-Mail:" labels survive
/// (a drop-to-first-'>' strip would eat them together with the `<a ...>`
/// opener), and `<br/>` tag remnants (`/>`) are trimmed per line.
fn block_lines(inner: &str) -> Vec<String> {
    inner
        .replace("<br", "\n")
        .split('\n')
        .map(|part| {
            let part = part.trim_start_matches("/>").trim_start_matches('>').trim();
            let mut out = String::new();
            let mut in_tag = false;
            for c in part.chars() {
                if c == '<' {
                    in_tag = true;
                } else if c == '>' {
                    in_tag = false;
                } else if !in_tag {
                    out.push(c);
                }
            }
            out.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for};

    // Verbatim API response, read-only verified on 2026-10-08.
    const API_FIXTURE: &str = r#"{"ok":true,"prices":{"wendeschneidplatten":{"text":"53,00 €/kg","amount":53,"schemaAmount":53},"hartmetallGemischt":{"text":"50,00 €/kg","amount":50,"schemaAmount":50},"vhmFraeserBohrer":{"text":"53,00 €/kg","amount":53,"schemaAmount":53},"widia":{"text":"50,00 €/kg","amount":50,"schemaAmount":50},"hartmetallschlamm":{"text":"nach Analyse","amount":null,"schemaAmount":null}},"fields":{"marktHinweis":"Aktuelle Ankaufspreise pro kg bei passender Sorte und sauber sortiertem Material","einheit":"alle Preise pro kg","kontakt":"info@vhm-hartmetall.de"},"updatedAt":"2026-10-05T08:51:01+00:00","revision":"b850f7af0b621ab51967ade0ecdb6f69f2714b5146e4e3b3cb41fb3b611ff49a"}"#;

    #[test]
    fn api_restores_four_stable_variants_and_source_date() {
        let out = super::parse_api(API_FIXTURE).expect("live API structure");
        let rows: Vec<_> = out
            .prices
            .iter()
            .map(|p| {
                (
                    p.material,
                    p.variant,
                    p.price,
                    p.currency,
                    p.unit,
                    p.price_kind,
                )
            })
            .collect();
        assert_eq!(
            rows,
            vec![
                (
                    "hartmetall",
                    "Wendeschneidplatten",
                    53.0,
                    "EUR",
                    "EUR/kg",
                    "exact"
                ),
                ("hartmetall", "gemischt", 50.0, "EUR", "EUR/kg", "exact"),
                (
                    "hartmetall",
                    "VHM-Fräser & Bohrer",
                    53.0,
                    "EUR",
                    "EUR/kg",
                    "exact"
                ),
                ("hartmetall", "Widia", 50.0, "EUR", "EUR/kg", "exact"),
            ]
        );
        assert_eq!(out.published_at.as_deref(), Some("2026-10-05"));
        assert_eq!(out.acceptances.len(), 1);
        assert_eq!(out.acceptances[0].material, "hartmetall");
        assert_eq!(out.acceptances[0].conditions, "Schlamm");
        assert!(out.skipped_labels.is_empty());
    }

    #[test]
    fn api_rejects_invalid_responses_without_fallback_prices() {
        for (pointer, value) in [
            ("/ok", serde_json::json!(false)),
            ("/fields/einheit", serde_json::json!("alle Preise pro t")),
            ("/fields/marktHinweis", serde_json::json!("")),
            ("/updatedAt", serde_json::json!("heute")),
            ("/revision", serde_json::json!("")),
            ("/prices/widia/amount", serde_json::json!(null)),
            ("/prices/widia/amount", serde_json::json!("50")),
            ("/prices/widia/amount", serde_json::json!(0)),
            ("/prices/widia/amount", serde_json::json!(-50)),
            ("/prices/widia/schemaAmount", serde_json::json!(65)),
            ("/prices/widia/text", serde_json::json!("50,00 €/t")),
            ("/prices/widia/text", serde_json::json!("65,00 €/kg")),
            ("/prices/widia/text", serde_json::json!("bis zu 50,00 €/kg")),
            ("/prices/hartmetallschlamm/amount", serde_json::json!(50)),
            (
                "/prices/hartmetallschlamm/schemaAmount",
                serde_json::json!(50),
            ),
            (
                "/prices/hartmetallschlamm/text",
                serde_json::json!("50,00 €/kg"),
            ),
        ] {
            let mut body: serde_json::Value = serde_json::from_str(API_FIXTURE).unwrap();
            *body.pointer_mut(pointer).unwrap() = value;
            assert!(
                super::parse_api(&body.to_string()).is_err(),
                "accepted invalid {pointer}: {body}"
            );
        }
        for pointer in [
            "/prices",
            "/fields",
            "/prices/widia",
            "/prices/hartmetallschlamm",
            "/prices/hartmetallschlamm/amount",
            "/prices/hartmetallschlamm/schemaAmount",
        ] {
            let mut body: serde_json::Value = serde_json::from_str(API_FIXTURE).unwrap();
            let (parent, key) = pointer.rsplit_once('/').unwrap();
            body.pointer_mut(parent)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(key);
            assert!(
                super::parse_api(&body.to_string()).is_err(),
                "accepted missing {pointer}"
            );
        }
        for body in [
            "",
            "<html>Preis anfragen 65,00 €/kg</html>",
            "{}",
            r#"{"ok":false,"error":"unavailable"}"#,
        ] {
            assert!(super::parse_api(body).is_err());
        }
    }

    #[test]
    fn api_uses_changed_source_amounts_and_flags_unknown_grades() {
        let mut body: serde_json::Value = serde_json::from_str(API_FIXTURE).unwrap();
        body["prices"]["widia"] =
            serde_json::json!({"text":"49,25 €/kg", "amount":49.25, "schemaAmount":49.25});
        body["prices"]["neueSorte"] =
            serde_json::json!({"text":"99,00 €/kg", "amount":99, "schemaAmount":99});
        let out = super::parse_api(&body.to_string()).expect("new prices, no hardcoded fallback");
        assert_eq!(out.prices.len(), 4);
        assert_eq!(out.prices[3].price, 49.25);
        assert_eq!(out.skipped_labels, vec!["neueSorte (unbekannte API-Sorte)"]);
    }

    #[tokio::test]
    #[ignore = "read-only live network verification; no store or record()"]
    async fn live_handler_uses_api_prices_and_source_date() {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        let out = (super::handler().scrape)(&client)
            .await
            .expect("exact public live handler");
        assert_eq!(out.fetch_url, super::API_URL);
        assert_eq!(out.status_code, 200);
        assert!(out.website_alive);
        assert!(out.byte_len > 0);
        assert_eq!(out.published_at.as_deref(), Some("2026-10-05"));
        assert_eq!(
            out.prices
                .iter()
                .map(|p| (p.variant, p.price))
                .collect::<Vec<_>>(),
            vec![
                ("Wendeschneidplatten", 53.0),
                ("gemischt", 50.0),
                ("VHM-Fräser & Bohrer", 53.0),
                ("Widia", 50.0),
            ]
        );
        assert_eq!(out.acceptances.len(), 1);
        assert_eq!(out.acceptances[0].conditions, "Schlamm");
        assert!(out.skipped_labels.is_empty());
        assert_eq!(out.trader_info.postcode, "42899");
        assert_eq!(out.trader_info.email, "info@vhm-hartmetall.de");
        eprintln!("Livehandler: {:?}", out);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<h2>Angaben gemäß § 5 DDG</h2>\
            <p><strong>VHM Hartmetall Ankauf</strong><br/>Yehya Jadouh<br/>\
            Grünenplatzstraße 1a<br/>42899 Remscheid<br/>Deutschland</p>\
            <h2>Kontakt</h2><p>Telefon: <a href=\"tel:+4917670524959\">017670524959</a><br/>\
            E-Mail: <a href=\"mailto:info@vhm-hartmetall.de\">info@vhm-hartmetall.de</a></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Grünenplatzstraße 1a");
        assert_eq!(info.postcode, "42899");
        assert_eq!(info.city, "Remscheid");
        assert_eq!(info.phone, "017670524959");
        assert_eq!(info.email, "info@vhm-hartmetall.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<h2>Neu</h2><p>x</p>").is_err());
    }

    #[test]
    fn mapping_resolves_all_grades() {
        // One material, five conditions: nothing collapses.
        assert_eq!(
            grade_for("VHM-Fräser & VHM-Bohrer"),
            Some(("hartmetall", "VHM-Fräser & Bohrer"))
        );
        assert_eq!(
            grade_for("Wendeschneidplatten"),
            Some(("hartmetall", "Wendeschneidplatten"))
        );
        assert_eq!(grade_for("Widia"), Some(("hartmetall", "Widia")));
        assert_eq!(
            grade_for("Hartmetall gemischt"),
            Some(("hartmetall", "gemischt"))
        );
        assert_eq!(
            grade_for("Hartmetallschlamm"),
            Some(("hartmetall", "Schlamm"))
        );
        assert_eq!(grade_for("Ankaufspreise gelten bei passender Sorte"), None);
    }
}
