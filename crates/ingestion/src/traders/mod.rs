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
    /// Price page URL (also used for failure journaling).
    pub url: &'static str,
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
    /// The trader's own sub-grade (`''` = standard grade). Two grades at
    /// different prices must never collapse into one material row.
    pub variant: &'static str,
    pub price: f64,
    pub currency: &'static str,
    pub unit: &'static str,
    /// One of `exact`, `upto`, `range`, `approx` (see [`PRICE_KINDS`]).
    pub price_kind: &'static str,
    pub price_min: Option<f64>,
    pub price_max: Option<f64>,
    /// 1.0 = exact list price, lower = vaguer ("bis zu", ranges).
    pub confidence: Option<f64>,
    /// Raw label from the page, always kept for traceability.
    pub label: String,
    /// Page-stated date for THIS material (RFC 3339). Falls back to the
    /// outcome default when the page shows one date for everything.
    pub published_at: Option<String>,
    /// Validity window for THIS material; None = open-ended.
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
}

/// One accepted material without a price (product/acceptance lists).
#[derive(Debug, Clone)]
pub struct ScrapedAcceptance {
    /// `materials.slug` from the handler's explicit mapping table.
    pub material: &'static str,
    /// Conditions as written ("nur blank", "ab 50 kg", "Späne").
    pub conditions: String,
    /// Raw label from the page, always kept for traceability.
    pub label: String,
}

/// Contact/address enrichment from website + Impressum. Empty = not found;
/// the importer only ever fills or upgrades, never blanks.
#[derive(Debug, Clone, Default)]
pub struct TraderInfo {
    pub street: String,
    pub postcode: String,
    pub city: String,
    pub phone: String,
    pub email: String,
}

impl TraderInfo {
    pub fn is_empty(&self) -> bool {
        self.street.is_empty()
            && self.postcode.is_empty()
            && self.city.is_empty()
            && self.phone.is_empty()
            && self.email.is_empty()
    }
}

/// Everything one scrape produced.

/// Everything one scrape produced.
#[derive(Debug, Clone, Default)]
pub struct HandlerOutcome {
    pub prices: Vec<ScrapedPrice>,
    /// Accepted materials without prices (product lists).
    pub acceptances: Vec<ScrapedAcceptance>,
    /// Contact/address enrichment (Impressum + page).
    pub trader_info: TraderInfo,
    /// True when at least one HTTP fetch succeeded (marks website aktiv).
    pub website_alive: bool,
    /// Labels the handler saw but could not map (logged, counted, skipped).
    pub skipped_labels: Vec<String>,
    /// The price page URL (provenance for every observation).
    pub fetch_url: String,
    pub status_code: u16,
    pub byte_len: usize,
    /// Page-stated validity date for the whole page (fallback for prices
    /// without their own date).
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
/// 1234.56, "0,170" → 0.17, "1.100" → 1100). Dots are thousands
/// separators when a comma is present — or when they group exactly three
/// digits ("1.100 €" is eleven hundred, not 1.1).
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
    } else if is_thousands_grouped(tok) {
        tok.replace('.', "")
    } else {
        tok.to_owned()
    };
    norm.parse().ok()
}

/// True for "1.100" / "12.345.678" (dot groups of exactly three digits).
/// "11.20" is a decimal point, not a thousands separator.
fn is_thousands_grouped(tok: &str) -> bool {
    let mut parts = tok.split('.');
    match parts.next() {
        Some(first) if (1..=3).contains(&first.len()) && first.chars().all(|c| c.is_ascii_digit()) => {
            parts.all(|p| p.len() == 3 && p.chars().all(|c| c.is_ascii_digit()))
                && tok.contains('.')
        }
        _ => false,
    }
}

/// True when a price text carries explicit unit markers ("/", "pro",
/// "je", "per") — as opposed to a bare number where the page-global unit
/// applies. An explicit-but-unknown unit ("pro Sack") must skip loudly
/// instead of inheriting the page default.
pub fn has_unit_markers(raw: &str) -> bool {
    let lower = raw.to_lowercase();
    if lower.contains('/') {
        return true;
    }
    lower
        .split(|c: char| !c.is_alphanumeric())
        .any(|t| matches!(t, "pro" | "je" | "per"))
}

/// Map a unit string to a quotation unit, token-based ("EUR / T" must not
/// slip through just because the slash and letter don't touch).
pub fn eur_unit(raw: &str) -> Option<&'static str> {
    let lower = raw.to_lowercase();
    let tokens: Vec<&str> = lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .collect();
    if tokens.iter().any(|t| *t == "kg") {
        Some("EUR/kg")
    } else if tokens.iter().any(|t| ["t", "to", "tonne", "tonnen"].contains(t)) {
        Some("EUR/t")
    } else if tokens
        .iter()
        .any(|t| ["stk", "st", "stueck", "stück", "stck"].contains(t))
    {
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


/// What one `record()` call produced. `accepted` counts resolved
/// material acceptances (info present), whether or not they changed rows.
#[derive(Debug, Clone, Default)]
pub struct RecordSummary {
    pub recorded: i64,
    pub accepted: usize,
    pub skipped: Vec<String>,
    pub canaries: Vec<String>,
}

/// Canary thresholds (documented, tunable). Baselines come from the last
/// finished non-failed step; the very first run has no baseline and only
/// the zero rule applies.
pub const CANARY_MIN_BASELINE: i64 = 5;
/// Recorded count below this fraction of baseline → page lost rows.
pub const CANARY_DROP_RATIO: f64 = 0.5;
/// Recorded count above this multiple of baseline → duplication smell.
pub const CANARY_GROWTH_FACTOR: f64 = 3.0;
/// Single price moving by this factor (or its inverse) → decimal/unit smell.
pub const CANARY_JUMP_RATIO: f64 = 3.0;

/// Record one handler outcome: resolve ids, append observations.
/// Unknown materials are skipped loudly; an unknown trader slug fails the
/// whole step (misconfiguration, must be heard). Fetch journaling lives
/// in the scheduler so failed attempts are journaled too.
/// Returns (recorded, skipped, canary_notes).
#[allow(clippy::too_many_arguments)]
pub async fn record(
    public: &PublicDb,
    internal: &InternalDb,
    handler_slug: &str,
    outcome: &HandlerOutcome,
    now: &DateTime<Utc>,
) -> Result<RecordSummary, super::IngestError> {
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
    let baseline = internal
        .last_ok_step_items(handler_slug)
        .unwrap_or(None);
    // Snapshot current prices BEFORE writing, for jump detection.
    let mut before: std::collections::HashMap<(i64, String), (f64, String, String)> =
        std::collections::HashMap::new();
    for p in &outcome.prices {
        if let Ok(Some(mid)) = public.find_material_id(p.material) {
            let key = (mid, p.variant.to_owned());
            if !before.contains_key(&key) {
                if let Ok(Some(cur)) = public.current_price_for(trader_id, mid, &p.variant) {
                    before.insert(key, (cur.price, cur.currency, cur.unit));
                }
            }
        }
    }
    let mut recorded = 0i64;
    let mut skipped = outcome.skipped_labels.clone();
    let mut canaries: Vec<String> = Vec::new();
    for p in &outcome.prices {
        let Some(material_id) = public.find_material_id(p.material).map_err(|source| {
            IngestError::Catalog { what: "material", name: p.material.to_owned(), source }
        })?
        else {
            skipped.push(format!("{} (unbekanntes Material)", p.label));
            continue;
        };
        // Normalize into the catalog unit (kg<->t); anything else stays
        // as quoted — conversions we cannot prove stay untouched.
        let (price, price_min, price_max, unit) =
            normalize_unit(public, material_id, p.price, p.price_min, p.price_max, p.unit)?;
        match public.record_price(&schrott_mcp_store::NewPrice {
            trader_id,
            material_id,
            variant: &p.variant,
            price,
            currency: p.currency,
            unit: &unit,
            price_kind: p.price_kind,
            price_min,
            price_max,
            confidence: p.confidence,
            source_type: "haendler_angabe",
            published: true,
            source_url: &outcome.fetch_url,
            observed_at: &now_s,
            published_at: p
                .published_at
                .as_deref()
                .or(outcome.published_at.as_deref()),
            valid_from: p.valid_from.as_deref(),
            valid_to: p.valid_to.as_deref(),
            notes: &p.label,
            extra_json: "{}",
            ingested_at: &now_s,
        }) {
            Ok(_) => {
                recorded += 1;
                // Jump canary against the pre-write snapshot (same
                // currency+unit only — anything else is not comparable).
                if let Some((old_price, old_cur, old_unit)) =
                    before.get(&(material_id, p.variant.to_owned()))
                {
                    if *old_cur == p.currency
                        && *old_unit == unit
                        && *old_price > 0.0
                    {
                        let ratio = price / old_price;
                        if ratio >= CANARY_JUMP_RATIO || ratio <= 1.0 / CANARY_JUMP_RATIO {
                            canaries.push(format!(
                                "Preissprung {} ({}): {:.3} -> {:.3} {} (x{:.1})",
                                p.material, p.variant, old_price, price, unit, ratio
                            ));
                        }
                    }
                }
                // A published price proves acceptance of the material.
                if let Err(e) = public.set_acceptance(
                    trader_id,
                    material_id,
                    true,
                    "",
                    None,
                    None,
                    &now_s,
                    &now_s,
                ) {
                    tracing::warn!("ingestion: acceptance write failed for {handler_slug}: {e}");
                }
            }
            Err(e) => {
                tracing::warn!("ingestion: price write failed for {handler_slug}: {e}");
                skipped.push(format!("{} (Schreibfehler)", p.label));
            }
        }
    }
    // Material acceptances (product lists without prices): resolve, write
    // only on change, count everything resolved.
    let mut accepted = 0usize;
    for a in &outcome.acceptances {
        let Some(material_id) = public.find_material_id(a.material).map_err(|source| {
            IngestError::Catalog { what: "material", name: a.material.to_owned(), source }
        })?
        else {
            skipped.push(format!("{} (unbekanntes Material)", a.label));
            continue;
        };
        accepted += 1;
        let same = public
            .existing_acceptance(trader_id, material_id)
            .map_err(|source| IngestError::Catalog {
                what: "trader_materials",
                name: format!("{handler_slug}/{}", a.material),
                source,
            })?;
        if same != Some((true, a.conditions.clone())) {
            if let Err(e) = public.set_acceptance(
                trader_id,
                material_id,
                true,
                &a.conditions,
                None,
                None,
                &now_s,
                &now_s,
            ) {
                tracing::warn!("ingestion: acceptance write failed for {handler_slug}: {e}");
                skipped.push(format!("{} (Schreibfehler)", a.label));
            }
        }
    }
    // Contact/address enrichment (Impressum): applied on change, logged.
    if !outcome.trader_info.is_empty() {
        let info = &outcome.trader_info;
        match public.set_trader_info(
            handler_slug,
            &info.street,
            &info.postcode,
            &info.city,
            &info.phone,
            &info.email,
            outcome.website_alive,
            &now_s,
        ) {
            Ok(changed) if !changed.is_empty() => {
                tracing::info!(
                    slug = handler_slug,
                    "Kontaktdaten aktualisiert: {}",
                    changed.join(", ")
                );
            }
            Err(e) => {
                tracing::warn!("ingestion: contact update failed for {handler_slug}: {e}");
            }
            Ok(_) => {}
        }
    }
    // Count canaries against the last good run. Acceptance-only runs
    // (no prices attempted, but acceptances resolved) are normal operation,
    // not a signal: only a run yielding nothing at all trips the zero rule.
    if recorded == 0 && accepted == 0 {
        canaries.push(format!(
            "0 Preise und 0 Annahmen übernommen ({} Parses, {} Skips) — Seite prüfen",
            outcome.prices.len(),
            skipped.len()
        ));
    } else if !outcome.prices.is_empty() {
        if let Some(base) = baseline {
            if base >= CANARY_MIN_BASELINE {
                let base_f = base as f64;
                if (recorded as f64) < base_f * CANARY_DROP_RATIO {
                    canaries.push(format!(
                        "Einbruch: {recorded} statt {base} Preisen — Seite prüfen"
                    ));
                } else if (recorded as f64) > base_f * CANARY_GROWTH_FACTOR {
                    canaries.push(format!(
                        "Explosion: {recorded} statt {base} Preisen — Duplikate prüfen"
                    ));
                }
            }
        }
    }
    Ok(RecordSummary { recorded, accepted, skipped, canaries })
}

/// Convert a quoted price into the catalog unit when the conversion is
/// exact (kg<->t). Returns (price, min, max, unit-as-stored).
fn normalize_unit(
    public: &PublicDb,
    material_id: i64,
    price: f64,
    min: Option<f64>,
    max: Option<f64>,
    unit: &str,
) -> Result<(f64, Option<f64>, Option<f64>, String), super::IngestError> {
    let target = public
        .material_unit(material_id)
        .map_err(|source| super::IngestError::Catalog {
            what: "material",
            name: format!("unit of #{material_id}"),
            source,
        })?
        .unwrap_or_else(|| unit.to_owned());
    if target == unit {
        return Ok((price, min, max, target));
    }
    let factor = match (unit, target.as_str()) {
        ("EUR/kg", "EUR/t") => 1000.0,
        ("EUR/t", "EUR/kg") => 1.0 / 1000.0,
        _ => return Ok((price, min, max, unit.to_owned())),
    };
    Ok((
        price * factor,
        min.map(|v| v * factor),
        max.map(|v| v * factor),
        target,
    ))
}

#[cfg(test)]
mod tests {
    use super::{eur_unit, has_unit_markers, parse_de_date, parse_eur};

    #[test]
    fn german_numbers() {
        assert_eq!(parse_eur("9,80"), Some(9.8));
        assert_eq!(parse_eur("0,170 €"), Some(0.17));
        assert_eq!(parse_eur("€ 100 x pro to"), Some(100.0));
        assert_eq!(parse_eur("bis zu € 10,80 erhalten"), Some(10.8));
        assert_eq!(parse_eur("1.234,56"), Some(1234.56));
        assert_eq!(parse_eur("11.20 €"), Some(11.2));
        assert_eq!(parse_eur("1.100 €"), Some(1100.0), "Tausenderpunkt");
        assert_eq!(parse_eur("12.345.678 €"), Some(12345678.0));
        assert_eq!(parse_eur("Preis auf Anfrage"), None);
        assert_eq!(parse_eur(""), None);
    }

    #[test]
    fn units_markers_and_dates() {
        assert_eq!(eur_unit("EUR / KG"), Some("EUR/kg"));
        assert_eq!(eur_unit("EUR / T"), Some("EUR/t"), "Leerzeichen egal");
        assert_eq!(eur_unit("€ pro kg"), Some("EUR/kg"));
        assert_eq!(eur_unit("€ 100 x pro to"), Some("EUR/t"));
        assert_eq!(eur_unit("30,00 €/Stk."), Some("EUR/Stk"));
        assert_eq!(eur_unit("100,00 € St."), Some("EUR/Stk"));
        assert_eq!(eur_unit("unbekannt"), None);
        assert_eq!(eur_unit("pro Sack"), None);
        assert!(has_unit_markers("0,170 € pro Sack"));
        assert!(has_unit_markers("11,20 €/KG"));
        assert!(!has_unit_markers("11,20 €"));
        assert!(!has_unit_markers("Probe 11,20 €"));
        assert_eq!(
            parse_de_date("27", "09", "2026").as_deref(),
            Some("2026-09-27T00:00:00+00:00")
        );
        assert_eq!(parse_de_date("31", "02", "2026"), None);
    }

    /// Canary matrix: drop / growth / jump / zero, all through record().
    #[tokio::test]
    async fn canary_rules() {
        use super::{HandlerOutcome, ScrapedPrice};
        use schrott_mcp_store::{InternalDb, NewMaterial, NewTrader, PublicDb};
        async fn pronto() -> HandlerOutcome {
            HandlerOutcome {
                prices: (0..10)
                    .map(|i| ScrapedPrice {
                        material: "kupfer-millberry",
                        variant: if i % 2 == 0 { "a" } else { "b" },
                        price: 9.8,
                        currency: "EUR",
                        unit: "EUR/kg",
                        price_kind: "exact",
                        price_min: None,
                        price_max: None,
                        confidence: Some(1.0),
                        label: format!("L{i}"),
                        published_at: None,
                        valid_from: None,
                        valid_to: None,
                    })
                    .collect(),
                acceptances: vec![],
                trader_info: super::TraderInfo::default(),
                website_alive: false,
                skipped_labels: vec![],
                fetch_url: "https://example.test/".to_owned(),
                status_code: 200,
                byte_len: 10,
                published_at: None,
            }
        }
        let dir = std::env::temp_dir().join(format!("schrott-canary-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let public = PublicDb::open(&dir).expect("db");
        let internal = InternalDb::open(&dir).expect("internal");
        let now = chrono::Utc::now();
        let now_s = now.to_rfc3339();
        public
            .upsert_trader(&NewTrader {
                slug: "canary-test",
                name: "Canary",
                trader_type: "schrotthaendler",
                description: "",
                street: "",
                postcode: "",
                city: "T",
                state: "BE",
                country: "DE",
                lat: None,
                lon: None,
                phone: "",
                email: "",
                website: "",
                website_status: "unbekannt",
                website_checked_at: "",
                opening_hours: "",
                dropoff_json: "{}",
                pickup_json: "{}",
                min_quantity_kg: None,
                max_quantity_kg: None,
                certifications: "[]",
                status: "aktiv",
                notes: "",
                extra_json: "{}",
                now: &now_s,
            })
            .expect("trader");
        public
            .upsert_material(&NewMaterial {
                slug: "kupfer-millberry",
                name_de: "Kupfer",
                category: "nichteisen",
                unit: "EUR/kg",
                description: "",
                updated_at: &now_s,
            })
            .expect("material");
        // Baseline run: 10 recorded, then a good step with 10 items.
        let r = super::record(&public, &internal, "canary-test", &pronto().await, &now).await.expect("run1");
        assert_eq!((r.recorded, r.canaries.len()), (10, 0), "first run: no baseline yet");
        let run = internal.create_run(&now_s).expect("run");
        let step = internal.create_step(run, "canary-test", &now_s).expect("step");
        internal.finish_step(step, "ok", 10, "10 Preise übernommen", &now_s).expect("close");
        // Same volume again: quiet.
        let r = super::record(&public, &internal, "canary-test", &pronto().await, &now).await.expect("run2");
        assert_eq!((r.recorded, r.canaries.len()), (10, 0));
        // Collapse to 2: drop canary.
        let mut few = pronto().await;
        few.prices.truncate(2);
        let r = super::record(&public, &internal, "canary-test", &few, &now).await.expect("run3");
        assert_eq!(r.canaries.len(), 1);
        assert!(r.canaries[0].contains("Einbruch"), "{:?}", r.canaries);
        // One price explodes 10x: jump canary, data still recorded.
        let mut jump = pronto().await;
        jump.prices[0].price = 98.0;
        let r = super::record(&public, &internal, "canary-test", &jump, &now).await.expect("run4");
        assert_eq!(r.recorded, 10);
        assert!(r.canaries.iter().any(|c| c.contains("Preissprung")), "{:?}", r.canaries);
    }
}
