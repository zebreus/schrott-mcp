//! WirKaufenDeinGold.de (Rohat Erdem, Bremen-Findorff, Edelmetall-Ankauf).
//! The homepage loads Gold-API spot references and FX rates client-side,
//! leaving placeholders in server-rendered HTML. The same live feeds are
//! fetched here and retained as explicit skips, never as buy quotes. The
//! actual `#calc-result-low` is a customer-specific estimate based on purity,
//! weight and payout factor, not a general price per gram. Only directly
//! evidenced material acceptances are recorded.
//!
//! Kontakt (`INFO_URL`, live-verifiziert 28.09.2026): Impressum mit
//! `<address>` ("Rohat Erdem … Admiralstraße 111, 28215
//! Bremen-Findorff") plus `tel:`-/`mailto:`-Links. Fehlt der
//! Diensteanbieter-Anker oder die Straße → lauter Fehler, nie raten.

use scraper::{Html, Selector};
use serde_json::Value;
use std::time::Duration;

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "hb-bremen-findorff-28215-wirkaufendeingold-de-rohat-erdem";

/// Homepage mit den Preis-Widgets und dem clientseitigen Spot-Feed.
/// Ein Umbau der Widgets/Feed-Verträge lässt den Step laut scheitern.
pub const PRICE_URL: &str = "https://wirkaufendeingold.de";
/// Bespoke, live-verifizierte Impressum-Seite (eigene URL + eigener
/// Block, kein Crawler).
pub const INFO_URL: &str = "https://wirkaufendeingold.de/impressum";
/// Straßen-Anker: schützt davor, dass die Seite still ein anderes
/// Unternehmen ausliefert.
const STREET_GUARD: &str = "Admiralstraße 111";
const TROY_OUNCE_GRAMS: f64 = 31.1035;
const GOLD_API_XAU: &str = "https://api.gold-api.com/price/XAU";
const GOLD_API_XAG: &str = "https://api.gold-api.com/price/XAG";
const GOLD_API_XPT: &str = "https://api.gold-api.com/price/XPT";
const GOLD_API_XPD: &str = "https://api.gold-api.com/price/XPD";
const COINBASE_FX: &str = "https://api.coinbase.com/v2/exchange-rates?currency=USD";
const FX_BACKUP: &str = "https://open.er-api.com/v6/latest/USD";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: PRICE_URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, PRICE_URL).await?;
    validate_price_page(&html)?;
    // Kontaktseiten-Fehler scheitern laut per Design: ein gezogenes
    // Impressum braucht Augen, bevor ihm wieder vertraut wird.
    let (_, info_html) = fetch_text(client, INFO_URL).await?;
    let trader_info = extract_info(&info_html)?;
    // The browser loads live spot data from the same public APIs. Do not use
    // its localStorage or hard-coded fallback prices: they may be stale.
    let skipped_labels = fetch_live_spot_rates(client).await?.skip_labels();
    Ok(HandlerOutcome {
        skip_price_acceptances: false,
        prices: vec![],
        acceptances: vec![
            ScrapedAcceptance {
                material: "gold",
                conditions: "Goldankauf; konkreter Preis nach Prüfung, Tageskurs nur Referenz"
                    .to_owned(),
                label: "Goldankauf auf der Betreiberseite".to_owned(),
            },
            ScrapedAcceptance {
                material: "silber",
                conditions: "Silberankauf; konkreter Preis nach Prüfung, Tageskurs nur Referenz"
                    .to_owned(),
                label: "Silberankauf auf der Betreiberseite".to_owned(),
            },
        ],
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: PRICE_URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

/// Rates are USD per troy ounce from the same public feeds used by the page.
/// They are only market references, never the trader's purchase prices.
#[derive(Debug, Clone, Copy)]
struct LiveSpotRates {
    gold_usd_per_ounce: f64,
    silver_usd_per_ounce: f64,
    platinum_usd_per_ounce: f64,
    palladium_usd_per_ounce: f64,
    eur_per_usd: f64,
    fx_source: &'static str,
}

impl LiveSpotRates {
    fn eur_per_gram(self, usd_per_ounce: f64) -> f64 {
        usd_per_ounce / TROY_OUNCE_GRAMS * self.eur_per_usd
    }

    fn skip_labels(self) -> Vec<String> {
        [
            ("Feingold", "XAU", self.gold_usd_per_ounce),
            ("Feinsilber", "XAG", self.silver_usd_per_ounce),
            ("Feinplatin", "XPT", self.platinum_usd_per_ounce),
            ("Feinpalladium", "XPD", self.palladium_usd_per_ounce),
        ]
        .into_iter()
        .map(|(metal, symbol, usd_per_ounce)| {
            format!(
                "Tageskurs {metal} {} EUR/g (Gold-API {symbol} + {}; Spot-Referenz, kein Händler-Ankaufspreis)",
                fmt_eur(self.eur_per_gram(usd_per_ounce)),
                self.fx_source
            )
        })
        .collect()
    }
}

/// Validate the page anchors and the client-side feed contract before using
/// the same feeds from the server. This prevents silently following a redesign
/// or treating the page's static placeholder/fallback as a live quote.
fn validate_price_page(html: &str) -> Result<(), IngestError> {
    let missing = |detail: &str| IngestError::Parse {
        url: PRICE_URL.to_owned(),
        detail: detail.to_owned(),
    };
    let doc = Html::parse_document(html);
    let sel = |s: &str| Selector::parse(s).expect("valid selector");
    let text_of = |css: &str| {
        doc.select(&sel(css))
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };
    // The independently labelled gold rate is a calculation input, not the
    // trader's payout. Keep its widget present even though JS fills its value.
    let gold_lbl = text_of("span#calc-spot-lbl");
    if !gold_lbl.contains("Feingold") {
        return Err(missing("Tageskurs-Label (Feingold) fehlt"));
    }
    for selector in ["span#calc-spot", "span#bars-spot", "span#bars-spot-silver"] {
        if doc.select(&sel(selector)).next().is_none() {
            return Err(missing(&format!("Kurselement ({selector}) fehlt")));
        }
    }
    let bars_note = text_of("p.bars-note");
    if !bars_note.contains("Gold") || !bars_note.contains("Silber") {
        return Err(missing("Gold-/Silber-Referenz in .bars-note fehlt"));
    }

    for marker in [
        "fetch('https://api.gold-api.com/price/XAU'",
        "fetch('https://api.gold-api.com/price/XAG'",
        "fetch('https://api.gold-api.com/price/XPT'",
        "fetch('https://api.gold-api.com/price/XPD'",
        "fetch('https://api.coinbase.com/v2/exchange-rates?currency=USD'",
        "fetch('https://open.er-api.com/v6/latest/USD'",
        "USD/troy oz",
        "var TROY_OZ    = 31.1035;",
        "fx.data.rates.EUR",
        "fxBk.rates.EUR",
        "spot.gold      = (gold.price   / TROY_OZ) * eurPerUsd;",
        "spot.silver    = (silver.price / TROY_OZ) * eurPerUsd;",
        "spot.platinum  = (plat.price   / TROY_OZ) * eurPerUsd;",
        "spot.palladium = (pall.price   / TROY_OZ) * eurPerUsd;",
    ] {
        if !html.contains(marker) {
            return Err(missing(&format!(
                "clientseitiger Kursvertrag fehlt ({marker})"
            )));
        }
    }

    Ok(())
}

async fn fetch_live_spot_rates(client: &reqwest::Client) -> Result<LiveSpotRates, IngestError> {
    let (gold, silver, platinum, palladium, coinbase, fx_backup) =
        tokio::time::timeout(Duration::from_secs(8), async {
            tokio::join!(
                fetch_json(client, GOLD_API_XAU),
                fetch_json(client, GOLD_API_XAG),
                fetch_json(client, GOLD_API_XPT),
                fetch_json(client, GOLD_API_XPD),
                fetch_json(client, COINBASE_FX),
                fetch_json(client, FX_BACKUP),
            )
        })
        .await
        .map_err(|_| IngestError::Parse {
            url: PRICE_URL.to_owned(),
            detail: "clientseitige Spot-APIs überschritten 8 Sekunden".to_owned(),
        })?;

    let gold_usd_per_ounce = parse_metal_quote(&gold?, "XAU", GOLD_API_XAU)?;
    let silver_usd_per_ounce = parse_metal_quote(&silver?, "XAG", GOLD_API_XAG)?;
    let platinum_usd_per_ounce = parse_metal_quote(&platinum?, "XPT", GOLD_API_XPT)?;
    let palladium_usd_per_ounce = parse_metal_quote(&palladium?, "XPD", GOLD_API_XPD)?;

    // Match the browser: use Coinbase's live USD/EUR rate first, then the
    // daily ExchangeRate-API rate. Never use the browser's localStorage or
    // its hard-coded FALLBACK values when both live FX feeds are unavailable.
    let fx = coinbase
        .ok()
        .and_then(|value| coinbase_eur_per_usd(&value))
        .map(|rate| (rate, "Coinbase USD/EUR"))
        .or_else(|| {
            fx_backup
                .ok()
                .and_then(|value| backup_eur_per_usd(&value))
                .map(|rate| (rate, "ExchangeRate-API USD/EUR (Fallback)"))
        })
        .ok_or_else(|| IngestError::Parse {
            url: COINBASE_FX.to_owned(),
            detail: "kein gültiger USD/EUR-Kurs von Coinbase oder ExchangeRate-API".to_owned(),
        })?;

    Ok(LiveSpotRates {
        gold_usd_per_ounce,
        silver_usd_per_ounce,
        platinum_usd_per_ounce,
        palladium_usd_per_ounce,
        eur_per_usd: fx.0,
        fx_source: fx.1,
    })
}

async fn fetch_json(client: &reqwest::Client, url: &str) -> Result<Value, IngestError> {
    let response = client
        .get(url)
        .timeout(Duration::from_secs(8))
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .map_err(|source| IngestError::Fetch {
            url: url.to_owned(),
            source,
        })?;
    let status = response.status().as_u16();
    if !response.status().is_success() {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: format!("HTTP {status}"),
        });
    }
    const MAX_API_RESPONSE_BYTES: usize = 256 * 1024;
    if response
        .content_length()
        .is_some_and(|length| length > MAX_API_RESPONSE_BYTES as u64)
    {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: "API-Antwort überschreitet 256 KiB".to_owned(),
        });
    }
    let body = response.bytes().await.map_err(|source| IngestError::Body {
        url: url.to_owned(),
        source,
    })?;
    if body.len() > MAX_API_RESPONSE_BYTES {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: "API-Antwort überschreitet 256 KiB".to_owned(),
        });
    }
    let body = std::str::from_utf8(&body).map_err(|error| IngestError::Parse {
        url: url.to_owned(),
        detail: format!("API-Antwort ist kein UTF-8: {error}"),
    })?;
    serde_json::from_str(body).map_err(|error| IngestError::Parse {
        url: url.to_owned(),
        detail: format!("JSON ungültig: {error}"),
    })
}

fn parse_metal_quote(value: &Value, expected_symbol: &str, url: &str) -> Result<f64, IngestError> {
    let invalid = |detail: &str| IngestError::Parse {
        url: url.to_owned(),
        detail: detail.to_owned(),
    };
    if value.get("symbol").and_then(Value::as_str) != Some(expected_symbol) {
        return Err(invalid(
            "Metall-Symbol stimmt nicht mit dem API-Endpunkt überein",
        ));
    }
    if value.get("currency").and_then(Value::as_str) != Some("USD") {
        return Err(invalid("API-Kurs ist nicht in USD"));
    }
    value
        .get("price")
        .and_then(Value::as_f64)
        .filter(|price| price.is_finite() && *price > 0.0)
        .ok_or_else(|| invalid("USD-Preis fehlt, ist ungültig oder nicht positiv"))
}

fn positive_json_number(value: &Value) -> Option<f64> {
    let number = value
        .as_f64()
        .or_else(|| value.as_str().and_then(|raw| raw.parse::<f64>().ok()))?;
    (number.is_finite() && number > 0.0).then_some(number)
}

fn coinbase_eur_per_usd(value: &Value) -> Option<f64> {
    if value.pointer("/data/currency")?.as_str()? != "USD" {
        return None;
    }
    positive_json_number(value.pointer("/data/rates/EUR")?)
}

fn backup_eur_per_usd(value: &Value) -> Option<f64> {
    if value.get("result")?.as_str()? != "success" || value.get("base_code")?.as_str()? != "USD" {
        return None;
    }
    positive_json_number(value.pointer("/rates/EUR")?)
}

fn fmt_eur(price: f64) -> String {
    format!("{price:.2}").replace('.', ",")
}

/// Bespoke Kontakt-Extraktion NUR für diese Impressum-Bauart:
/// `<address>` ("Rohat Erdem … Admiralstraße 111 / 28215
/// Bremen-Findorff") plus `tel:`-/`mailto:`-Links (Anzeige-Text
/// gewinnt). Fehlender Diensteanbieter-Anker oder falsche Straße →
/// lauter Fehler, nie geraten.
fn extract_info(html: &str) -> Result<TraderInfo, IngestError> {
    let missing = |detail: &str| IngestError::Parse {
        url: INFO_URL.to_owned(),
        detail: detail.to_owned(),
    };
    if !html.contains("Diensteanbieter") {
        return Err(missing("Diensteanbieter-Anker fehlt"));
    }
    if !html.contains(STREET_GUARD) {
        return Err(missing(&format!("falsche Seite (kein {STREET_GUARD})")));
    }
    let a_start = html
        .find("<address")
        .ok_or_else(|| missing("Adress-Block fehlt"))?;
    let a_tail = &html[a_start..];
    let a_end = a_tail
        .find("</address>")
        .ok_or_else(|| missing("Adress-Block unvollständig"))?;
    let block = a_tail[..a_end]
        .replace("<br />", "\n")
        .replace("<br/>", "\n")
        .replace("<br>", "\n");
    let lines: Vec<String> = block
        .split('\n')
        .map(|l| {
            // `<br`-Split hinterlässt Tag-Reste → erst alles bis zum
            // ersten `>` verwerfen, dann whitespace-normieren.
            let no_tags: String = l
                .split('>')
                .next_back()
                .unwrap_or("")
                .split('<')
                .next()
                .unwrap_or("")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            no_tags
        })
        .filter(|l| !l.is_empty())
        .collect();
    let street = lines
        .iter()
        .find(|l| l.contains(STREET_GUARD))
        .cloned()
        .unwrap_or_default();
    let (mut postcode, mut city) = (String::new(), String::new());
    for l in &lines {
        let mut it = l.split_whitespace();
        let Some(pc) = it.next() else { continue };
        if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
            postcode = pc.to_owned();
            city = it.collect::<Vec<_>>().join(" ");
            break;
        }
    }
    let doc = Html::parse_document(html);
    let link_text = |prefix: &str| {
        let sel = Selector::parse(&format!("a[href^=\"{prefix}\"]")).expect("valid selector");
        doc.select(&sel)
            .next()
            .map(|el| {
                el.text()
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default()
    };
    let phone = link_text("tel:");
    let email = link_text("mailto:");
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(missing("keine Kontaktdaten gefunden"));
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
    use super::{
        backup_eur_per_usd, coinbase_eur_per_usd, extract_info, parse_metal_quote,
        validate_price_page, LiveSpotRates, COINBASE_FX, GOLD_API_XAU, TROY_OUNCE_GRAMS,
    };
    use serde_json::{json, Value};

    // Captured source structure: the browser fills these placeholders from its
    // six live API requests after the server-rendered page has loaded.
    const LIVE_PAGE_FIXTURE: &str = r#"
        <span id="calc-spot-lbl">Tageskurs Feingold</span>
        <span id="calc-spot">…</span>
        <p class="bars-note">* Richtwerte auf Basis des aktuellen Tagespreises
        (Gold <span id="bars-spot">…</span>/g, Silber
        <span id="bars-spot-silver">…</span>/g).</p>
        <script>
        fetch('https://api.gold-api.com/price/XAU'
        fetch('https://api.gold-api.com/price/XAG'
        fetch('https://api.gold-api.com/price/XPT'
        fetch('https://api.gold-api.com/price/XPD'
        fetch('https://api.coinbase.com/v2/exchange-rates?currency=USD'
        fetch('https://open.er-api.com/v6/latest/USD'
        USD/troy oz; var TROY_OZ    = 31.1035;
        fx.data.rates.EUR; fxBk.rates.EUR;
        spot.gold      = (gold.price   / TROY_OZ) * eurPerUsd;
        spot.silver    = (silver.price / TROY_OZ) * eurPerUsd;
        spot.platinum  = (plat.price   / TROY_OZ) * eurPerUsd;
        spot.palladium = (pall.price   / TROY_OZ) * eurPerUsd;
        </script>
    "#;

    #[test]
    fn current_page_placeholders_are_resolved_by_the_live_feed_contract() {
        validate_price_page(LIVE_PAGE_FIXTURE).expect("known client-side layout");

        let changed_feed = LIVE_PAGE_FIXTURE.replace("price/XAU", "price/XAG");
        assert!(validate_price_page(&changed_feed).is_err());
        let missing_widget = LIVE_PAGE_FIXTURE.replace("id=\"calc-spot\"", "id=\"changed\"");
        assert!(validate_price_page(&missing_widget).is_err());
    }

    fn quote(symbol: &str, price: Value) -> Value {
        json!({"symbol": symbol, "currency": "USD", "price": price})
    }

    #[test]
    fn metal_feed_requires_matching_symbol_currency_and_positive_number() {
        let parsed = parse_metal_quote(&quote("XAU", json!(4023.199951)), "XAU", GOLD_API_XAU)
            .expect("valid XAU feed");
        assert_eq!(parsed, 4023.199951);

        for invalid in [
            json!({"symbol":"XAG", "currency":"USD", "price":4023.2}),
            json!({"symbol":"XAU", "currency":"EUR", "price":4023.2}),
            json!({"symbol":"XAU", "currency":"USD", "price":0}),
            json!({"symbol":"XAU", "currency":"USD", "price":-1}),
            json!({"symbol":"XAU", "currency":"USD", "price":"4023.2"}),
        ] {
            assert!(parse_metal_quote(&invalid, "XAU", GOLD_API_XAU).is_err());
        }
    }

    #[test]
    fn fx_uses_coinbase_first_and_accepts_only_usd_base_backup() {
        let coinbase = json!({
            "data": {"currency":"USD", "rates":{"EUR":"0.89177095"}}
        });
        assert_eq!(coinbase_eur_per_usd(&coinbase), Some(0.89177095));

        let backup = json!({
            "result":"success", "base_code":"USD", "rates":{"EUR":0.890292}
        });
        assert_eq!(backup_eur_per_usd(&backup), Some(0.890292));
        assert_eq!(backup_eur_per_usd(&json!({"result":"error"})), None);
        assert_eq!(
            backup_eur_per_usd(&json!({
                "result":"success", "base_code":"EUR", "rates":{"EUR":1.0}
            })),
            None
        );
        assert_eq!(
            coinbase_eur_per_usd(&json!({
                "data": {"currency":"EUR", "rates":{"EUR":"1"}}
            })),
            None
        );
    }

    #[test]
    fn client_rates_convert_usd_per_troy_ounce_to_eur_per_gram_and_stay_skips() {
        let rates = LiveSpotRates {
            gold_usd_per_ounce: 4023.199951,
            silver_usd_per_ounce: 54.678001,
            platinum_usd_per_ounce: 1568.0,
            palladium_usd_per_ounce: 1077.0,
            eur_per_usd: 0.8917709509814413,
            fx_source: "Coinbase USD/EUR",
        };
        let labels = rates.skip_labels();
        assert_eq!(labels.len(), 4);
        assert!(labels[0].contains("115,35 EUR/g"));
        assert!(labels[1].contains("1,57 EUR/g"));
        assert!(labels[2].contains("44,96 EUR/g"));
        assert!(labels[3].contains("30,88 EUR/g"));
        assert!(labels
            .iter()
            .all(|label| label.contains("Spot-Referenz, kein Händler-Ankaufspreis")));
        assert!(
            (rates.eur_per_gram(rates.gold_usd_per_ounce)
                - rates.gold_usd_per_ounce / TROY_OUNCE_GRAMS * rates.eur_per_usd)
                .abs()
                < f64::EPSILON
        );
        assert_eq!(
            COINBASE_FX,
            "https://api.coinbase.com/v2/exchange-rates?currency=USD"
        );
    }

    #[test]
    fn impressum_contact_block() {
        // Realer Ausschnitt der Live-Impressumseite /impressum.
        let imp = "<h2>Diensteanbieter</h2><address><strong>Rohat Erdem</strong><br />\
            Einzelunternehmen, handelnd unter wirkaufendeingold.de<br />\
            Admiralstraße 111<br />28215 Bremen-Findorff<br />Deutschland</address>\
            <h2>Kontakt</h2><p>Telefon: <a href=\"tel:015150247474\">0151 5024 7474</a><br />\
            E-Mail: <a href=\"mailto:info@wirkaufendeingold.de\">info@wirkaufendeingold.de</a><br />\
            Internet: <a href=\"https://wirkaufendeingold.de\">wirkaufendeingold.de</a></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Admiralstraße 111");
        assert_eq!(info.postcode, "28215");
        assert_eq!(info.city, "Bremen-Findorff");
        assert_eq!(info.phone, "0151 5024 7474");
        assert_eq!(info.email, "info@wirkaufendeingold.de");
        assert!(extract_info("<div>ohne Anker</div>").is_err());
        // Fremde Seite scheitert am Straßen-Anker.
        let other = imp.replace("Admiralstraße 111", "Sögestraße 1");
        assert!(extract_info(&other).is_err());
    }

    #[tokio::test]
    #[ignore = "read-only live network check; fetches public pages/APIs, never records data"]
    async fn live_handler_loads_client_side_rates_without_recording_them_as_prices() {
        let client = reqwest::Client::builder()
            .user_agent("schrott-mcp-ingestion/0.1")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        let outcome = super::scrape(&client)
            .await
            .expect("live page, public price feeds and Impressum");

        assert!(outcome.prices.is_empty());
        assert_eq!(outcome.skipped_labels.len(), 4);
        assert!(outcome
            .skipped_labels
            .iter()
            .all(|label| label.contains("EUR/g") && label.contains("Spot-Referenz")));
    }
}
