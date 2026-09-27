//! Due-list scheduler: one sequential loop over all handlers.
//!
//! Each handler has a next-due timestamp (UTC). Every run executes what's
//! due and advances its stamp — sequential, so overlap is impossible by
//! construction and one failing trader never stops the rest. Initial
//! stamps spread across the interval by slug hash (no thundering herd
//! after restarts); `DailyAt` stamps are the configured local times.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use chrono::{DateTime, Utc};
use chrono_tz::Europe::Berlin;
use schrott_mcp_store::{InternalDb, PublicDb};

use super::{Handler, Schedule};

/// Per-handler timeout: a hanging site delays the loop at most this long.
const HANDLER_TIMEOUT: Duration = Duration::from_secs(120);

/// Next-due stamps, shared by the background loop and manual triggers.
fn due_map() -> &'static Mutex<HashMap<&'static str, DateTime<Utc>>> {
    static DUE: std::sync::OnceLock<Mutex<HashMap<&'static str, DateTime<Utc>>>> =
        std::sync::OnceLock::new();
    DUE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn stagger_secs(slug: &str, interval_secs: u64) -> u64 {
    let h: u64 = slug
        .bytes()
        .fold(0, |a, b| a.wrapping_mul(31).wrapping_add(b as u64));
    h % interval_secs.max(1)
}

/// First due stamp for a handler at boot.
pub fn initial_due(boot: DateTime<Utc>, slug: &str, schedule: &Schedule) -> DateTime<Utc> {
    match schedule {
        Schedule::Every { secs } => {
            boot + chrono::Duration::seconds(stagger_secs(slug, *secs) as i64)
        }
        Schedule::DailyAt { times } => next_daily_after(boot, times),
    }
}

/// Next stamp after a run finished at `from`.
pub fn next_after(from: DateTime<Utc>, schedule: &Schedule) -> DateTime<Utc> {
    match schedule {
        Schedule::Every { secs } => from + chrono::Duration::seconds(*secs as i64),
        Schedule::DailyAt { times } => next_daily_after(from, times),
    }
}

/// Next Berlin-local (hh:mm) occurrence strictly after `from`, as UTC.
pub fn next_daily_after(from: DateTime<Utc>, times: &[(u8, u8)]) -> DateTime<Utc> {
    let local = from.with_timezone(&Berlin);
    let mut best: Option<DateTime<Utc>> = None;
    for day_offset in 0..=1 {
        let date = local.date_naive() + chrono::Duration::days(day_offset);
        for (h, m) in times {
            let Some(naive) = date.and_hms_opt((*h).into(), (*m).into(), 0) else {
                continue;
            };
            // Berlin has no ambiguity handling needs here: spring-forward
            // gaps resolve to the later offset, fall-back folds to the
            // first occurrence — both fine for a polling schedule.
            let candidate = match naive.and_local_timezone(Berlin) {
                chrono::LocalResult::Single(t) => t,
                chrono::LocalResult::Ambiguous(a, _) => a,
                chrono::LocalResult::None => continue,
            }
            .with_timezone(&Utc);
            if candidate > from && best.is_none_or(|b| candidate < b) {
                best = Some(candidate);
            }
        }
        if best.is_some() && day_offset == 0 {
            break;
        }
    }
    best.unwrap_or(from + chrono::Duration::hours(6))
}

/// Run all due handlers (or all, when `force`), sequentially with step
/// bookkeeping. Returns (recorded prices, failed handler slugs).
pub async fn run_due_with(
    handlers: &[Handler],
    internal: &InternalDb,
    public: &PublicDb,
    client: &reqwest::Client,
    run_id: i64,
    force: bool,
) -> (i64, Vec<String>) {
    let now = Utc::now();
    let mut due: Vec<&Handler> = Vec::new();
    if let Ok(mut map) = due_map().lock() {
        for h in handlers {
            let next = map
                .entry(h.slug)
                .or_insert_with(|| initial_due(now, h.slug, &h.schedule));
            if force || *next <= now {
                due.push(h);
            }
        }
    }
    let mut recorded = 0i64;
    let mut failed = Vec::new();
    for h in due {
        let step_at = Utc::now().to_rfc3339();
        let step_id = internal.create_step(run_id, h.slug, &step_at).unwrap_or(0);
        let outcome = tokio::time::timeout(HANDLER_TIMEOUT, (h.scrape)(client)).await;
        let stamped = Utc::now();
        if let Ok(mut map) = due_map().lock() {
            map.insert(h.slug, next_after(stamped, &h.schedule));
        }
        match outcome {
            Err(_) => {
                tracing::warn!("ingestion: handler {} timed out", h.slug);
                failed.push(format!("{}: timeout", h.slug));
                if step_id != 0 {
                    let _ = internal.finish_step(
                        step_id,
                        "failed",
                        0,
                        "timeout nach 120s",
                        &Utc::now().to_rfc3339(),
                    );
                }
            }
            Ok(Err(e)) => {
                tracing::warn!("ingestion: handler {} failed: {e}", h.slug);
                failed.push(format!("{}: {e}", h.slug));
                if step_id != 0 {
                    let _ = internal.finish_step(
                        step_id,
                        "failed",
                        0,
                        &format!("{e}"),
                        &Utc::now().to_rfc3339(),
                    );
                }
            }
            Ok(Ok(out)) => {
                match super::record(public, internal, run_id, h.slug, &out, &stamped).await
                {
                    Err(e) => {
                        tracing::warn!("ingestion: record failed for {}: {e}", h.slug);
                        failed.push(format!("{}: {e}", h.slug));
                        if step_id != 0 {
                            let _ = internal.finish_step(
                                step_id,
                                "failed",
                                0,
                                &format!("{e}"),
                                &Utc::now().to_rfc3339(),
                            );
                        }
                    }
                    Ok((n, skipped)) => {
                        recorded += n;
                        let mut detail = format!("{n} Preise übernommen");
                        if !skipped.is_empty() {
                            let shown: Vec<&str> =
                                skipped.iter().take(5).map(String::as_str).collect();
                            detail.push_str(&format!(
                                ", {} übersprungen ({}{})",
                                skipped.len(),
                                shown.join("; "),
                                if skipped.len() > 5 { "; …" } else { "" }
                            ));
                        }
                        if step_id != 0 {
                            let _ = internal.finish_step(
                                step_id,
                                "ok",
                                n,
                                &detail,
                                &Utc::now().to_rfc3339(),
                            );
                        }
                    }
                }
            }
        }
    }
    (recorded, failed)
}

#[cfg(test)]
mod tests {
    use super::{initial_due, next_after, next_daily_after};
    use super::{Handler, Schedule};
    use chrono::{TimeZone, Utc};
    use schrott_mcp_store::{InternalDb, PublicDb};

    #[test]
    fn stagger_spreads_handlers() {
        let boot = Utc.with_ymd_and_hms(2026, 9, 27, 12, 0, 0).unwrap();
        let slugs = [
            "hb-woltmershausen-vedder-stockrahm",
            "bb-lauchhammer-ost-lausitz-recycling",
            "nw-essen-vogelheim-tappe-rohstoffhandel",
            "he-hattersheim-kupferhelden",
            "ni-marxen-metallankauf24-andre-owsianski-ne-spezia",
        ];
        let dues: Vec<_> = slugs
            .iter()
            .map(|s| initial_due(boot, s, &Schedule::every_6h()))
            .collect();
        assert!(dues.iter().all(|d| *d >= boot && *d <= boot + chrono::Duration::hours(6)));
        let span = dues.iter().max().unwrap().signed_duration_since(*dues.iter().min().unwrap());
        assert!(span > chrono::Duration::minutes(30), "span {span}");
    }

    #[test]
    fn daily_times_pick_next_berlin_occurrence() {
        // 12:00 UTC = 14:00 Berlin (CEST): next of 08:00/16:00 is 16:00.
        let now = Utc.with_ymd_and_hms(2026, 9, 27, 12, 0, 0).unwrap();
        let next = next_daily_after(now, &[(8, 0), (16, 0)]);
        assert_eq!(next, Utc.with_ymd_and_hms(2026, 9, 27, 14, 0, 0).unwrap());
        // After the last time: tomorrow morning.
        let late = Utc.with_ymd_and_hms(2026, 9, 27, 15, 0, 0).unwrap();
        let next = next_daily_after(late, &[(8, 0), (16, 0)]);
        assert_eq!(next, Utc.with_ymd_and_hms(2026, 9, 28, 6, 0, 0).unwrap());
        // Interval schedules just add the period.
        let after = next_after(now, &Schedule::every_6h());
        assert_eq!(after, now + chrono::Duration::hours(6));
    }

    #[tokio::test]
    async fn failing_handler_does_not_stop_loop() {
        use crate::traders::{HandlerOutcome, ScrapedPrice};
        async fn ok(_: &reqwest::Client) -> Result<HandlerOutcome, crate::IngestError> {
            Ok(HandlerOutcome {
                prices: vec![ScrapedPrice {
                    material: "nope-not-a-material",
                    variant: "",
                    price: 1.0,
                    currency: "EUR",
                    unit: "EUR/kg",
                    price_min: None,
                    price_max: None,
                    confidence: None,
                    label: "Test".to_owned(),
                    published_at: None,
                    valid_from: None,
                    valid_to: None,
                }],
                skipped_labels: vec![],
                fetch_url: "https://example.test/".to_owned(),
                status_code: 200,
                byte_len: 10,
                published_at: None,
            })
        }
        async fn bad(_: &reqwest::Client) -> Result<HandlerOutcome, crate::IngestError> {
            Err(crate::IngestError::Parse {
                url: "https://example.test/".to_owned(),
                detail: "kaputt".to_owned(),
            })
        }
        fn boxed(
            f: for<'a> fn(
                &'a reqwest::Client,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<HandlerOutcome, crate::IngestError>> + Send + 'a>,
            >,
        ) -> super::super::ScrapeFn {
            f
        }
        let ok_fn = boxed(|c| Box::pin(ok(c)));
        let bad_fn = boxed(|c| Box::pin(bad(c)));
        let dir = std::env::temp_dir().join(format!("schrott-duetest-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let public = PublicDb::open(&dir).expect("db");
        let internal = InternalDb::open(&dir).expect("internal");
        // The ok-handler's trader must exist; its test material does not
        // (unknown materials skip loudly instead of failing).
        public
            .upsert_trader(&schrott_mcp_store::NewTrader {
                slug: "bb-lauchhammer-ost-lausitz-recycling",
                name: "Test",
                trader_type: "schrotthaendler",
                description: "",
                street: "",
                postcode: "",
                city: "Test",
                state: "BB",
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
                now: "2026-09-27T00:00:00Z",
            })
            .expect("trader");
        let run = internal.create_run("2026-09-27T00:00:00Z").expect("run");
        let client = reqwest::Client::new();
        let handlers = vec![
            Handler { slug: "bb-lauchhammer-ost-lausitz-recycling", schedule: Schedule::every_6h(), scrape: ok_fn },
            Handler { slug: "kaputt-test", schedule: Schedule::every_6h(), scrape: bad_fn },
        ];
        let (recorded, failed) =
            super::run_due_with(&handlers, &internal, &public, &client, run, true).await;
        // ok-handler wrote nothing (unknown material skipped) but did not fail;
        // bad-handler failed loudly; loop survived both.
        assert_eq!(recorded, 0);
        assert_eq!(failed.len(), 1);
        assert!(failed[0].starts_with("kaputt-test"));
    }

    #[tokio::test]
    async fn per_material_validity_dates() {
        use crate::traders::{HandlerOutcome, ScrapedPrice};
        use schrott_mcp_store::NewMaterial;
        async fn two_dates(_: &reqwest::Client) -> Result<HandlerOutcome, crate::IngestError> {
            Ok(HandlerOutcome {
                prices: vec![
                    ScrapedPrice {
                        material: "kupfer-millberry",
                        variant: "80-98%",
                        price: 9.8,
                        currency: "EUR",
                        unit: "EUR/kg",
                        price_min: None,
                        price_max: None,
                        confidence: Some(1.0),
                        label: "Cu neu".to_owned(),
                        published_at: Some("2026-09-20T00:00:00+00:00".to_owned()),
                        valid_from: Some("2026-09-20T00:00:00+00:00".to_owned()),
                        valid_to: Some("2026-09-27T00:00:00+00:00".to_owned()),
                    },
                    ScrapedPrice {
                        material: "messing",
                        variant: "",
                        price: 4.9,
                        currency: "EUR",
                        unit: "EUR/kg",
                        price_min: None,
                        price_max: None,
                        confidence: Some(1.0),
                        label: "Ms alt".to_owned(),
                        // no own date -> page fallback applies
                        published_at: None,
                        valid_from: None,
                        valid_to: None,
                    },
                ],
                skipped_labels: vec![],
                fetch_url: "https://example.test/preise".to_owned(),
                status_code: 200,
                byte_len: 10,
                published_at: Some("2026-09-27T00:00:00+00:00".to_owned()),
            })
        }
        let dir = std::env::temp_dir().join(format!("schrott-dates-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let public = PublicDb::open(&dir).expect("db");
        let internal = InternalDb::open(&dir).expect("internal");
        let now = "2026-09-27T00:00:00Z";
        for (slug, name) in [("kupfer-millberry", "Kupfer"), ("messing", "Messing")] {
            public
                .upsert_material(&NewMaterial {
                    slug,
                    name_de: name,
                    category: "nichteisen",
                    unit: "EUR/kg",
                    description: "",
                    updated_at: now,
                })
                .expect("material");
        }
        public
            .upsert_trader(&schrott_mcp_store::NewTrader {
                slug: "datums-test",
                name: "Datums Test",
                trader_type: "schrotthaendler",
                description: "",
                street: "",
                postcode: "",
                city: "Test",
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
                now,
            })
            .expect("trader");
        let run = internal.create_run(now).expect("run");
        let client = reqwest::Client::new();
        let two_fn: super::super::ScrapeFn = |c| Box::pin(two_dates(c));
        let handlers = vec![Handler {
            slug: "datums-test",
            schedule: Schedule::every_6h(),
            scrape: two_fn,
        }];
        let (recorded, failed) =
            super::run_due_with(&handlers, &internal, &public, &client, run, true).await;
        assert_eq!((recorded, failed.len()), (2, 0));
        let res = public
            .query_sql(
                "SELECT m.slug, p.variant, p.published_at, p.valid_from, p.valid_to
                 FROM prices p JOIN materials m ON m.id = p.material_id
                 ORDER BY m.slug",
            )
            .expect("query");
        assert_eq!(res.rows.len(), 2);
        assert_eq!(res.rows[0][1].as_str(), Some("80-98%"));
        // kupfer-millberry keeps its own dates…
        assert_eq!(res.rows[0][2].as_str(), Some("2026-09-20T00:00:00+00:00"));
        assert_eq!(res.rows[0][3].as_str(), Some("2026-09-20T00:00:00+00:00"));
        assert_eq!(res.rows[0][4].as_str(), Some("2026-09-27T00:00:00+00:00"));
        // …messing falls back to the page date, validity stays open.
        assert_eq!(res.rows[1][2].as_str(), Some("2026-09-27T00:00:00+00:00"));
        assert!(res.rows[1][3].is_null());
        assert!(res.rows[1][4].is_null());
    }
}
