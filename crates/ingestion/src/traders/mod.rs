//! Trader price ingestion: one small handler per Händler.
//!
//! Scaling concept (deliberately boring, so it survives 500 handlers):
//! - Every handler is ONE file in `handlers/` with zero shared parsing
//!   code — only `fetch_text` (HTTP) and `parse_eur` (German numbers) are
//!   shared, because every price page is shaped differently. A handler is
//!   a slug, a [`Schedule`] and a `scrape` fn returning [`ScrapedPrice`]s
//!   with explicit per-label material mapping. Unknown labels are skipped
//!   loudly (counted in the step detail), never guessed.
//! - The [`scheduler`] runs what's due in ONE sequential loop: staggered
//!   6 h cadence per handler (hash offset, no thundering herd), optional
//!   [`Schedule::DailyAt`] times (Europe/Berlin) per trader, per-handler
//!   timeouts, failure isolation. Sequential = overlap-safe by
//!   construction; at ~seconds per handler the loop serves hundreds of
//!   traders inside one 6 h window. If it ever doesn't, shard the due
//!   list across tasks — the handler interface stays untouched.
//! - Writes go through [`record`] into `prices` with provenance
//!   (`source_type=haendler_angabe`, `published=true`, `source_url`,
//!   `published_at` when the page shows a date). `current_prices`
//!   follows automatically; history is append-only.

pub mod handlers;
pub mod scheduler;

use std::future::Future;
use std::pin::Pin;

use chrono::{DateTime, Utc};
use schrott_mcp_store::{InternalDb, PublicDb};

/// When a handler runs. `Every` staggers by slug hash; `DailyAt` fires at
/// fixed local times (e.g. a trader publishing morning prices).
#[derive(Debug, Clone)]
pub enum Schedule {
    Every { secs: u64 },
    DailyAt { times: Vec<(u8, u8)> },
}

impl Schedule {
    /// Default cadence: every 6 hours, staggered.
    pub fn every_6h() -> Self {
        Self::Every { secs: 6 * 3600 }
    }
}

/// One trader's pipeline: identity + schedule + scrape fn. The fn pointer
/// keeps handlers dyn-safe without an async-trait dependency; each handler
/// file owns its URL, selectors and material mapping.
pub struct Handler {
    /// `traders.slug` this handler writes prices for.
    pub slug: &'static str,
    pub schedule: Schedule,
    pub scrape: ScrapeFn,
}

pub type ScrapeFn = for<'a> fn(
    &'a reqwest::Client,
) -> Pin<Box<dyn Future<Output = Result<HandlerOutcome, super::IngestError>> + Send + 'a>>;

/// One parsed price point, ready to record.
#[derive(Debug, Clone)]
pub struct ScrapedPrice {
    /// `materials.slug` from the handler's explicit mapping table.
    pub material: &'static str,
    pub price: f64,
    pub currency: &'static str,
    pub unit: &'static str,
    pub price_min: Option<f64>,
    pub price_max: Option<f64>,
    /// 1.0 = exact list price, lower = vaguer ("bis zu", ranges).
    pub confidence: Option<f64>,
    /// Raw label from the page, always kept for traceability.
    pub label: String,
}

/// Everything one scrape produced.
#[derive(Debug, Clone, Default)]
pub struct HandlerOutcome {
    pub prices: Vec<ScrapedPrice>,
    /// Labels the handler saw but could not map (logged, counted, skipped).
    pub skipped_labels: Vec<String>,
    /// The price page URL (provenance for every observation).
    pub fetch_url: String,
    pub status_code: u16,
    pub byte_len: usize,
    /// Page-stated validity date (RFC 3339), if the page shows one.
    pub published_at: Option<String>,
}

/// GET a page; non-2xx is a `Parse` error carrying the status.
pub async fn fetch_text(
    client: &reqwest::Client,
    url: &str,
) -> Result<(u16, String), super::IngestError> {
    use super::IngestError;
    let res = client
        .get(url)
        .send()
        .await
        .map_err(|source| IngestError::Fetch { url: url.to_owned(), source })?;
    let status = res.status().as_u16();
    if !res.status().is_success() {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: format!("HTTP {status}"),
        });
    }
    let body = res
        .text()
        .await
        .map_err(|source| IngestError::Body { url: url.to_owned(), source })?;
    Ok((status, body))
}

/// Parse the first German-formatted number ("9,80" → 9.8, "1.234,56" →
/// 1234.56, "0,170" → 0.17). Dots are thousands separators only when a
/// comma is present.
pub fn parse_eur(raw: &str) -> Option<f64> {
    let tok: String = raw
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
        .collect();
    let tok = tok.trim_matches(['.', ',']);
    if tok.is_empty() || !tok.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }
    let norm = if tok.contains(',') {
        tok.replace('.', "").replace(',', ".")
    } else {
        tok.to_owned()
    };
    norm.parse().ok()
}

/// Map a unit string to a quotation unit.
pub fn eur_unit(raw: &str) -> Option<&'static str> {
    let s = raw.to_lowercase();
    if s.contains("kg") {
        Some("EUR/kg")
    } else if s.contains("/t") || s.contains("pro to") || s.contains("tonne") || s.contains(" €/t") {
        Some("EUR/t")
    } else if s.contains("stk") || s.contains("stück") || s.contains("stck") {
        Some("EUR/Stk")
    } else {
        None
    }
}

/// Parse a German calendar date (dd.mm.yyyy) to RFC 3339 UTC midnight.
pub fn parse_de_date(day: &str, month: &str, year: &str) -> Option<String> {
    let (d, m, y): (u32, u32, i32) = (day.parse().ok()?, month.parse().ok()?, year.parse().ok()?);
    chrono::NaiveDate::from_ymd_opt(y, m, d).map(|d| {
        chrono::NaiveDateTime::new(d, chrono::NaiveTime::MIN).and_utc().to_rfc3339()
    })
}

/// Record one handler outcome: resolve ids, append observations, journal
/// the fetch. Unknown materials are skipped loudly; an unknown trader
/// slug fails the whole step (misconfiguration, must be heard).
#[allow(clippy::too_many_arguments)]
pub async fn record(
    public: &PublicDb,
    internal: &InternalDb,
    run_id: i64,
    handler_slug: &str,
    outcome: &HandlerOutcome,
    now: &DateTime<Utc>,
) -> Result<(i64, Vec<String>), super::IngestError> {
    use super::IngestError;
    let Some(trader_id) = public.find_trader_id(handler_slug).map_err(|source| {
        IngestError::Catalog { what: "trader", name: handler_slug.to_owned(), source }
    })?
    else {
        return Err(IngestError::Parse {
            url: outcome.fetch_url.clone(),
            detail: format!("unknown trader slug '{handler_slug}' — handler misconfigured"),
        });
    };
    let now_s = now.to_rfc3339();
    let content_hash = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        outcome.fetch_url.hash(&mut h);
        outcome.byte_len.hash(&mut h);
        format!("{:016x}", h.finish())
    };
    if let Err(e) = internal.log_fetch(&schrott_mcp_store::FetchRecord {
        run_id,
        scraper: handler_slug,
        url: &outcome.fetch_url,
        status_code: i64::from(outcome.status_code),
        content_hash: &content_hash,
        byte_len: outcome.byte_len as i64,
        fetched_at: &now_s,
    }) {
        tracing::warn!("ingestion: fetch journal failed for {handler_slug}: {e}");
    }
    let mut recorded = 0i64;
    let mut skipped = outcome.skipped_labels.clone();
    for p in &outcome.prices {
        let Some(material_id) = public.find_material_id(p.material).map_err(|source| {
            IngestError::Catalog { what: "material", name: p.material.to_owned(), source }
        })?
        else {
            skipped.push(format!("{} (unbekanntes Material)", p.label));
            continue;
        };
        match public.record_price(&schrott_mcp_store::NewPrice {
            trader_id,
            material_id,
            price: p.price,
            currency: p.currency,
            unit: p.unit,
            price_min: p.price_min,
            price_max: p.price_max,
            confidence: p.confidence,
            source_type: "haendler_angabe",
            published: true,
            source_url: &outcome.fetch_url,
            observed_at: &now_s,
            published_at: outcome.published_at.as_deref(),
            valid_from: None,
            valid_to: None,
            notes: &p.label,
            extra_json: "{}",
            ingested_at: &now_s,
        }) {
            Ok(_) => recorded += 1,
            Err(e) => {
                tracing::warn!("ingestion: price write failed for {handler_slug}: {e}");
                skipped.push(format!("{} (Schreibfehler)", p.label));
            }
        }
    }
    Ok((recorded, skipped))
}

#[cfg(test)]
mod tests {
    use super::{eur_unit, parse_de_date, parse_eur};

    #[test]
    fn german_numbers() {
        assert_eq!(parse_eur("9,80"), Some(9.8));
        assert_eq!(parse_eur("0,170 €"), Some(0.17));
        assert_eq!(parse_eur("€ 100 x pro to"), Some(100.0));
        assert_eq!(parse_eur("bis zu € 10,80 erhalten"), Some(10.8));
        assert_eq!(parse_eur("1.234,56"), Some(1234.56));
        assert_eq!(parse_eur("11.20 €"), Some(11.2));
        assert_eq!(parse_eur("Preis auf Anfrage"), None);
        assert_eq!(parse_eur(""), None);
    }

    #[test]
    fn units_and_dates() {
        assert_eq!(eur_unit("EUR / KG"), Some("EUR/kg"));
        assert_eq!(eur_unit("€ pro kg"), Some("EUR/kg"));
        assert_eq!(eur_unit("€ 100 x pro to"), Some("EUR/t"));
        assert_eq!(eur_unit("30,00 €/Stk."), Some("EUR/Stk"));
        assert_eq!(eur_unit("unbekannt"), None);
        assert_eq!(
            parse_de_date("27", "09", "2026").as_deref(),
            Some("2026-09-27T00:00:00+00:00")
        );
        assert_eq!(parse_de_date("31", "02", "2026"), None);
    }
}
