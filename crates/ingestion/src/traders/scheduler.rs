//! Due-list scheduler: one sequential loop over all handlers.
//!
//! Each handler has a next-due timestamp (UTC). Every run executes what's
//! due and advances its stamp — sequential, so overlap is impossible by
//! construction and one failing trader never stops the rest. Initial
//! interval phases are stable across restarts and spread by slug hash (no
//! thundering herd); `DailyAt` stamps are the configured local times.

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
            let interval = (*secs).max(1) as i64;
            let phase = (stagger_secs(slug, *secs) as i64).rem_euclid(interval);
            let elapsed = boot.timestamp().rem_euclid(interval);
            let mut until_phase = (phase - elapsed).rem_euclid(interval);
            if until_phase == 0 && boot.timestamp_subsec_nanos() > 0 {
                until_phase = interval;
            }
            boot + chrono::Duration::seconds(until_phase)
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
) -> (i64, Vec<String>, Vec<String>) {
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
    let mut warnings = Vec::new();
    for h in due {
        // Structured fields on every event (JSON logs carry them as
        // top-level journal fields). No span guard: `Entered` is !Send.
        let step_at = Utc::now().to_rfc3339();
        let step_id = match internal.create_step(run_id, h.slug, &step_at) {
            Ok(id) => id,
            Err(e) => {
                tracing::warn!(
                    slug = h.slug,
                    "step bookkeeping failed, running unrecorded: {e}"
                );
                0
            }
        };
        let outcome = tokio::time::timeout(HANDLER_TIMEOUT, (h.scrape)(client)).await;
        let stamped = Utc::now();
        if let Ok(mut map) = due_map().lock() {
            map.insert(h.slug, next_after(stamped, &h.schedule));
        }
        match outcome {
            Err(_) => {
                tracing::warn!(slug = h.slug, "timeout nach 120s");
                journal(internal, run_id, h.slug, h.url, 0, 0, &stamped);
                failed.push(format!("{}: timeout", h.slug));
                close_step(internal, step_id, "failed", 0, "timeout nach 120s");
            }
            Ok(Err(e)) => {
                tracing::warn!(slug = h.slug, "scrape failed: {e}");
                journal(internal, run_id, h.slug, h.url, 0, 0, &stamped);
                failed.push(format!("{}: {e}", h.slug));
                close_step(internal, step_id, "failed", 0, &format!("{e}"));
            }
            Ok(Ok(out)) => {
                journal(
                    internal,
                    run_id,
                    h.slug,
                    &out.fetch_url,
                    i64::from(out.status_code),
                    out.byte_len as i64,
                    &stamped,
                );
                match super::record(public, internal, h.slug, &out, &stamped).await {
                    Err(e) => {
                        tracing::warn!(slug = h.slug, "record failed: {e}");
                        failed.push(format!("{}: {e}", h.slug));
                        close_step(internal, step_id, "failed", 0, &format!("{e}"));
                    }
                    Ok(summary) => {
                        recorded += summary.recorded;
                        let n = summary.recorded;
                        let mut detail = format!("{n} Preise übernommen");
                        if summary.accepted > 0 {
                            detail.push_str(&format!(", {} Annahmen", summary.accepted));
                        }
                        let skipped = summary.skipped;
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
                        // Canary trips are warnings, not failures: the data
                        // is recorded, but a human should check the scraper.
                        // They travel in their own vec so the run status can
                        // distinguish "broken" from "look at this".
                        let status = if summary.canaries.is_empty() {
                            "ok"
                        } else {
                            "warning"
                        };
                        for c in &summary.canaries {
                            tracing::warn!(slug = h.slug, "canary: {c}");
                            warnings.push(format!("{}: {c}", h.slug));
                            detail.push_str(&format!(" | CANARY: {c}"));
                        }
                        tracing::info!(slug = h.slug, "{detail}");
                        close_step(internal, step_id, status, n, &detail);
                    }
                }
            }
        }
    }
    (recorded, failed, warnings)
}

/// Journal one fetch attempt (success or failure) into `raw_fetches`.
/// Failures carry status 0 and no bytes — the attempt itself is the data.
fn journal(
    internal: &InternalDb,
    run_id: i64,
    slug: &str,
    url: &str,
    status: i64,
    bytes: i64,
    at: &chrono::DateTime<Utc>,
) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    (slug, url, status, bytes).hash(&mut h);
    if let Err(e) = internal.log_fetch(&schrott_mcp_store::FetchRecord {
        run_id,
        scraper: slug,
        url,
        status_code: status,
        content_hash: &format!("{:016x}", h.finish()),
        byte_len: bytes,
        fetched_at: &at.to_rfc3339(),
    }) {
        tracing::warn!(slug, "fetch journal failed: {e}");
    }
}

/// Close a step unless bookkeeping already failed (step_id 0).
fn close_step(internal: &InternalDb, step_id: i64, status: &str, n: i64, detail: &str) {
    if step_id != 0 {
        if let Err(e) = internal.finish_step(step_id, status, n, detail, &Utc::now().to_rfc3339()) {
            tracing::warn!("step bookkeeping failed: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::TraderInfo;
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
        assert!(dues
            .iter()
            .all(|d| *d >= boot && *d <= boot + chrono::Duration::hours(6)));
        let span = dues
            .iter()
            .max()
            .unwrap()
            .signed_duration_since(*dues.iter().min().unwrap());
        assert!(span > chrono::Duration::minutes(30), "span {span}");
    }

    #[test]
    fn interval_stagger_is_stable_across_restarts() {
        let slug = "he-hanau-63450-goldfuxx-hanau-ophirum";
        let period = 6 * 3600;
        let phase = super::stagger_secs(slug, period);
        let target = Utc.timestamp_opt((period * 100 + phase) as i64, 0).unwrap();
        let schedule = Schedule::every_6h();
        let first_boot = target - chrono::Duration::hours(3);
        let second_boot = target - chrono::Duration::hours(2);

        assert_eq!(initial_due(first_boot, slug, &schedule), target);
        assert_eq!(initial_due(second_boot, slug, &schedule), target);
    }

    #[tokio::test]
    async fn fallback_acceptance_recorded_without_invented_prices() {
        use crate::traders::{HandlerOutcome, ScrapedPrice};
        use schrott_mcp_store::{NewMaterial, NewTrader};
        let dir = crate::test_support::TempDbDir::new("fallback");
        let public = PublicDb::open(dir.path()).expect("db");
        let internal = InternalDb::open(dir.path()).expect("internal");
        let now = "2026-09-29T00:00:00Z";
        public
            .upsert_trader(&NewTrader {
                slug: "t-ram",
                name: "T",
                trader_type: "schrotthaendler",
                description: "",
                street: "",
                postcode: "",
                city: "C",
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
                certifications: None,
                status: "aktiv",
                notes: "",
                extra_json: "{}",
                now,
            })
            .expect("trader");
        for (slug, cat) in [("ram", "elektronik"), ("platinen", "elektronik")] {
            public
                .upsert_material(&NewMaterial {
                    slug,
                    name_de: slug,
                    category: cat,
                    unit: "EUR/kg",
                    description: "",
                    updated_at: now,
                })
                .expect("material");
        }
        let out = HandlerOutcome {
            prices: vec![ScrapedPrice {
                material: "ram",
                variant: "Goldkante",
                price: 70.0,
                currency: "EUR",
                unit: "EUR/kg",
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label: "RAM Goldkante".to_owned(),
            }],
            acceptances: vec![],
            trader_info: TraderInfo::default(),
            website_alive: false,
            skipped_labels: vec![],
            fetch_url: "https://example.test/".to_owned(),
            status_code: 200,
            byte_len: 10,
            published_at: None,
        };
        let summary = super::super::record(
            &public,
            &internal,
            "t-ram",
            &out,
            &chrono::DateTime::parse_from_rfc3339(now)
                .unwrap()
                .with_timezone(&chrono::Utc),
        )
        .await
        .expect("record");
        assert_eq!(summary.recorded, 1);
        // No invented platinen price …
        let plat = public
            .find_material_id("platinen")
            .expect("find")
            .expect("exists");
        let tid = public.find_trader_id("t-ram").expect("t").expect("t");
        assert!(public
            .current_price_for(tid, plat, "")
            .expect("q")
            .is_none());
        // … but a documented fallback acceptance exists.
        let acc = public
            .existing_acceptance(tid, plat)
            .expect("q")
            .expect("fallback acceptance");
        assert!(acc.0);
        assert!(acc.1.contains("ram"), "conditions: {}", acc.1);
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
                    price_kind: "exact",
                    price_min: None,
                    price_max: None,
                    confidence: None,
                    label: "Test".to_owned(),
                }],
                acceptances: vec![],
                trader_info: TraderInfo::default(),
                website_alive: false,
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
                Box<
                    dyn std::future::Future<Output = Result<HandlerOutcome, crate::IngestError>>
                        + Send
                        + 'a,
                >,
            >,
        ) -> super::super::ScrapeFn {
            f
        }
        let ok_fn = boxed(|c| Box::pin(ok(c)));
        let bad_fn = boxed(|c| Box::pin(bad(c)));
        let dir = crate::test_support::TempDbDir::new("due-loop");
        let public = PublicDb::open(dir.path()).expect("db");
        let internal = InternalDb::open(dir.path()).expect("internal");
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
                certifications: None,
                status: "aktiv",
                notes: "",
                extra_json: "{}",
                now: "2026-09-27T00:00:00Z",
            })
            .expect("trader");
        let run = internal.create_run("2026-09-27T00:00:00Z").expect("run");
        let client = reqwest::Client::new();
        let handlers = vec![
            Handler {
                slug: "bb-lauchhammer-ost-lausitz-recycling",
                url: "https://example.test/",
                schedule: Schedule::every_6h(),
                scrape: ok_fn,
            },
            Handler {
                slug: "kaputt-test",
                url: "https://example.test/",
                schedule: Schedule::every_6h(),
                scrape: bad_fn,
            },
        ];
        let (recorded, failed, warnings) =
            super::run_due_with(&handlers, &internal, &public, &client, run, true).await;
        // ok-handler wrote nothing (unknown material skipped) but did not fail;
        // bad-handler failed loudly; loop survived both. Zero output trips
        // the canary without failing the step.
        assert_eq!(recorded, 0);
        assert_eq!(failed.len(), 1);
        assert!(failed[0].starts_with("kaputt-test"));
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("0 Preise"));
    }
}
