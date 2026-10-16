//! One-time Hofmann Zinn -> Zinn alt cutover, not a recurring source snapshot.
//! A persisted decision also remembers explicit coexistence (including requests).
use rusqlite::{params, OptionalExtension as _, TransactionBehavior};

use super::{prices, NewPrice, PriceRow, PublicDb};
use crate::error::StoreError;

const URL: &str = "https://hofmann-entsorgung.de/preise-und-verguetungen/";
pub(super) const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS hofmann_tin_cutovers (
    trader_id INTEGER NOT NULL REFERENCES traders(id),
    material_id INTEGER NOT NULL REFERENCES materials(id),
    source_url TEXT NOT NULL,
    decided_at TEXT NOT NULL,
    retired_price_id INTEGER REFERENCES prices(id),
    PRIMARY KEY(trader_id, material_id)
);";

// One strict, calendar-valid UTC ordering key for guard AND pointer comparison.
// Merely accepting a date via SQLite would normalize impossible calendar days.
pub(super) fn utc_key(raw: &str) -> Result<String, StoreError> {
    let body = raw
        .strip_suffix("+00:00")
        .or_else(|| raw.strip_suffix('Z'))
        .ok_or(StoreError::Rejected("Hofmann timestamp must be UTC"))?;
    let (seconds, fraction) = body.split_once('.').unwrap_or((body, ""));
    if seconds.len() != 19
        || !seconds.is_ascii()
        || seconds.as_bytes()[10] != b'T'
        || fraction.len() > 9
        || !fraction.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(StoreError::Rejected("invalid Hofmann timestamp"));
    }
    let parsed = chrono::DateTime::parse_from_rfc3339(raw)
        .map_err(|_| StoreError::Rejected("invalid Hofmann timestamp"))?;
    // Keep the existing ordinary UTC-second contract; do not introduce leap
    // seconds, which SQLite and the original cutover format did not support.
    if parsed.timestamp_subsec_nanos() >= 1_000_000_000 {
        return Err(StoreError::Rejected("invalid Hofmann timestamp"));
    }
    Ok(parsed.to_rfc3339_opts(chrono::SecondsFormat::Nanos, false))
}

fn current_on(
    conn: &rusqlite::Connection,
    p: &NewPrice<'_>,
    variant: &str,
) -> Result<Option<PriceRow>, StoreError> {
    conn.query_row(
        "SELECT p.id,p.trader_id,p.material_id,p.variant,p.price,p.currency,p.unit,p.price_kind,
         p.price_min,p.price_max,p.confidence,p.source_type,p.published,p.source_url,
         p.observed_at,p.published_at,p.valid_from,p.valid_to,p.notes,p.extra_json,p.ingested_at
         FROM current_prices c JOIN prices p ON p.id=c.price_id
         WHERE c.trader_id=?1 AND c.material_id=?2 AND c.variant=?3",
        params![p.trader_id, p.material_id, variant],
        prices::row_to_price,
    )
    .optional()
    .map_err(StoreError::from)
}

fn proven_legacy(old: &PriceRow) -> bool {
    let extra = serde_json::from_str::<serde_json::Value>(&old.extra_json).ok();
    old.variant.is_empty()
        && old.source_url == URL
        && old.source_type == "haendler_angabe"
        && old.published
        && old.notes == "Zinn"
        && old.currency == "EUR"
        && old.unit == "EUR/kg"
        && old.price.is_finite()
        && old.price > 0.0
        && old.price_kind == "exact"
        && old.confidence == Some(1.0)
        && old.price_min.is_none()
        && old.price_max.is_none()
        && old.valid_from.is_none()
        && old.valid_to.is_none()
        && extra.as_ref().is_some_and(|e| {
            e["map_v"] == 2
                && matches!(
                    e["published_at_basis"].as_str(),
                    Some("page_stated" | "row_stated")
                )
                && e.get("snapshot_owner")
                    .is_none_or(|v| v.is_null() || v == "")
        })
}

/// All public price writers funnel through this guard. Even direct legacy
/// record_price/backfill calls cannot resurrect the retired key at/before the
/// cutover or rewind a later genuine grade by deduping under the same ID.
pub(super) fn guard_write_on(
    conn: &rusqlite::Connection,
    p: &NewPrice<'_>,
) -> Result<Option<String>, StoreError> {
    if !matches!(p.variant, "" | "alt") {
        return Ok(None);
    }
    let decision: Option<(String, Option<i64>)> = conn.query_row(
        "SELECT decided_at,retired_price_id FROM hofmann_tin_cutovers WHERE trader_id=?1 AND material_id=?2",
        params![p.trader_id,p.material_id], |r| Ok((r.get(0)?,r.get(1)?))).optional()?;
    if let Some((at, retired)) = decision {
        let incoming = utc_key(p.observed_at)?;
        let boundary = utc_key(&at)?;
        if incoming < boundary
            || (p.variant.is_empty() && retired.is_some() && incoming == boundary)
        {
            return Err(StoreError::Rejected("Hofmann tin replay predates cutover"));
        }
        if let Some(current) = current_on(conn, p, p.variant)? {
            if incoming < utc_key(&current.observed_at)? {
                return Err(StoreError::Rejected("Hofmann tin replay predates current"));
            }
        }
        return Ok(Some(incoming));
    }
    // The first typed quote also reaches record_price_on before its cutover
    // ledger is committed. Keep known Hofmann tin source keys on the same
    // pointer ordering already then; do not fall back to raw strings for an
    // existing generic Current merely because this is the first decision.
    if p.source_url == URL {
        let scope: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM traders t,materials m WHERE t.id=?1 AND m.id=?2
             AND t.slug='bw-rastatt-hofmann' AND m.slug='zinn')",
            params![p.trader_id, p.material_id],
            |r| r.get(0),
        )?;
        if scope {
            let incoming = utc_key(p.observed_at)?;
            if let Some(current) = current_on(conn, p, p.variant)? {
                if incoming < utc_key(&current.observed_at)? {
                    return Err(StoreError::Rejected("Hofmann tin replay predates current"));
                }
            }
            return Ok(Some(incoming));
        }
    }
    Ok(None)
}

impl PublicDb {
    /// Persist validated explicit coexistence, even with two request-only rows.
    /// This conservative decision never removes a quote and cannot reopen an
    /// already decided upgrade. It is separate from best-effort price writes.
    pub fn preserve_hofmann_tin_grades(
        &self,
        trader_id: i64,
        observed_at: &str,
    ) -> Result<(), StoreError> {
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let material: i64 = tx.query_row(
            "SELECT m.id FROM traders t,materials m WHERE t.id=?1 AND t.slug='bw-rastatt-hofmann' AND m.slug='zinn'",
            [trader_id], |r| r.get(0))?;
        let at = utc_key(observed_at)?;
        tx.execute("INSERT OR IGNORE INTO hofmann_tin_cutovers(trader_id,material_id,source_url,decided_at,retired_price_id)
            VALUES(?1,?2,?3,?4,NULL)", params![trader_id,material,URL,at])?;
        tx.commit()?;
        Ok(())
    }

    /// Publish one typed Hofmann tin quote. `ungraded_present` is source-row
    /// presence, INCLUDING request-only Zinn, not merely numeric-price presence.
    /// The first successful alt observation durably closes the legacy window:
    /// retire a fully proven old key only if Zinn is absent; otherwise preserve
    /// the distinct grade. Every later alt write is observation-only.
    pub fn record_hofmann_tin_price(
        &self,
        p: &NewPrice<'_>,
        ungraded_present: bool,
    ) -> Result<i64, StoreError> {
        let extra = serde_json::from_str::<serde_json::Value>(p.extra_json).ok();
        if p.source_url != URL
            || p.source_type != "haendler_angabe"
            || !p.published
            || p.currency != "EUR"
            || p.unit != "EUR/kg"
            || p.price_kind != "exact"
            || !p.price.is_finite()
            || p.price <= 0.0
            || p.confidence != Some(1.0)
            || p.price_min.is_some()
            || p.price_max.is_some()
            || p.valid_from.is_some()
            || p.valid_to.is_some()
            || !matches!((p.variant, p.notes), ("", "Zinn") | ("alt", "Zinn alt"))
            || (p.variant.is_empty() && !ungraded_present)
            || !extra.as_ref().is_some_and(|e| {
                e["map_v"] == 2
                    && e["published_at_basis"] == "row_stated"
                    && e.get("snapshot_owner")
                        .is_none_or(|v| v.is_null() || v == "")
            })
        {
            return Err(StoreError::Rejected("invalid typed Hofmann tin quote"));
        }
        let mut conn = self.lock()?;
        // Reserve the write lock BEFORE selecting evidence; no external ID or
        // label/metadata fingerprint can go stale between proof and deletion.
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let scope: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM traders t, materials m WHERE t.id=?1 AND m.id=?2
             AND t.slug='bw-rastatt-hofmann' AND m.slug='zinn')",
            params![p.trader_id, p.material_id],
            |r| r.get(0),
        )?;
        if !scope {
            return Err(StoreError::Rejected("wrong Hofmann tin scope"));
        }
        let observed = utc_key(p.observed_at)?;
        let ingested = utc_key(p.ingested_at)?;
        let normalized = NewPrice {
            observed_at: &observed,
            ingested_at: &ingested,
            ..*p
        };
        let p = &normalized;
        let _ = guard_write_on(&tx, p)?;
        if let Some(current) = current_on(&tx, p, p.variant)? {
            let marker = serde_json::from_str::<serde_json::Value>(&current.extra_json).ok();
            if current.source_url != URL
                || current.source_type != "haendler_angabe"
                || !current.published
                || marker.as_ref().is_none_or(|e| {
                    e.get("snapshot_owner")
                        .is_some_and(|v| !v.is_null() && v != "" && v != "hofmann-zinn-alt-v1")
                })
                || utc_key(&current.observed_at)? > observed
            {
                return Err(StoreError::Rejected("foreign or newer Hofmann tin current"));
            }
        }
        let decided: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM hofmann_tin_cutovers WHERE trader_id=?1 AND material_id=?2)",
            params![p.trader_id,p.material_id], |r| r.get(0))?;
        let legacy = if p.variant == "alt" && !decided && !ungraded_present {
            current_on(&tx, p, "")?.filter(proven_legacy)
        } else {
            None
        };
        if let Some(old) = &legacy {
            if utc_key(&old.observed_at)? > observed {
                return Err(StoreError::Rejected("late Hofmann tin cutover"));
            }
        }
        let id = Self::record_price_on(&tx, p)?;
        if let Some(old) = &legacy {
            tx.execute("DELETE FROM current_prices WHERE trader_id=?1 AND material_id=?2 AND variant='' AND price_id=?3",
                params![p.trader_id,p.material_id,old.id])?;
        }
        if p.variant == "alt" && !decided {
            tx.execute("INSERT INTO hofmann_tin_cutovers(trader_id,material_id,source_url,decided_at,retired_price_id)
                VALUES(?1,?2,?3,?4,?5)", params![p.trader_id,p.material_id,URL,observed,legacy.as_ref().map(|p|p.id)])?;
        }
        tx.commit()?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::NewMaterial;
    use crate::test_support::TempDbDir;

    fn setup(label: &str) -> (TempDbDir, PublicDb, i64, i64) {
        let dir = TempDbDir::new(label);
        let db = PublicDb::open(dir.path()).unwrap();
        // Fixture SQL is restricted to this newly-created disposable database.
        let trader = {
            let conn = db.lock().unwrap();
            conn.execute("INSERT INTO traders(slug,name,first_seen_at,updated_at) VALUES('bw-rastatt-hofmann','Hofmann','2026-10-14T03:00:00Z','2026-10-14T03:00:00Z')", []).unwrap();
            conn.last_insert_rowid()
        };
        let material = db
            .upsert_material(&NewMaterial {
                slug: "zinn",
                name_de: "Zinn",
                category: "nichteisen",
                unit: "EUR/kg",
                description: "",
                updated_at: "2026-10-14T03:00:00Z",
            })
            .unwrap();
        (dir, db, trader, material)
    }

    fn quote<'a>(trader: i64, material: i64, variant: &'a str, at: &'a str) -> NewPrice<'a> {
        NewPrice {
            trader_id: trader,
            material_id: material,
            variant,
            price: 11.5,
            currency: "EUR",
            unit: "EUR/kg",
            price_kind: "exact",
            price_min: None,
            price_max: None,
            confidence: Some(1.0),
            source_type: "haendler_angabe",
            published: true,
            source_url: URL,
            observed_at: at,
            published_at: Some("2026-09-30T00:00:00+00:00"),
            valid_from: None,
            valid_to: None,
            notes: if variant == "alt" { "Zinn alt" } else { "Zinn" },
            extra_json: "{\"map_v\":2,\"published_at_basis\":\"row_stated\"}",
            ingested_at: at,
        }
    }

    fn state(db: &PublicDb) -> Vec<Vec<Vec<serde_json::Value>>> {
        [
            "SELECT * FROM prices ORDER BY id",
            "SELECT * FROM current_prices ORDER BY trader_id,material_id,variant",
            "SELECT * FROM hofmann_tin_cutovers ORDER BY trader_id,material_id",
        ]
        .map(|sql| db.query_sql(sql).unwrap().rows)
        .into()
    }

    #[test]
    fn hofmann_cutover_rolls_back_price_retirement_and_ledger_on_each_failure() {
        for trigger in [
            "CREATE TRIGGER fail BEFORE INSERT ON prices WHEN NEW.variant='alt' BEGIN SELECT RAISE(ABORT,'price failure'); END",
            "CREATE TRIGGER fail BEFORE DELETE ON current_prices BEGIN SELECT RAISE(ABORT,'retire failure'); END",
            "CREATE TRIGGER fail BEFORE INSERT ON hofmann_tin_cutovers BEGIN SELECT RAISE(ABORT,'ledger failure'); END",
        ] {
            let (_dir,db,trader,material) = setup("hofmann-cutover-rollback");
            let old = quote(trader,material,"","2026-10-14T03:00:00Z");
            db.record_price(&old).unwrap();
            let before = state(&db);
            db.lock().unwrap().execute_batch(trigger).unwrap();
            let new = quote(trader,material,"alt","2026-10-16T03:00:00Z");
            assert!(db.record_hofmann_tin_price(&new,false).is_err());
            assert_eq!(state(&db),before,"all three persisted surfaces must roll back");
            db.lock().unwrap().execute_batch("DROP TRIGGER fail").unwrap();
            db.record_hofmann_tin_price(&new,false).unwrap();
            assert!(db.current_price_for(trader,material,"").unwrap().is_none());
            assert_eq!(db.query_sql("SELECT COUNT(*) FROM hofmann_tin_cutovers").unwrap().rows[0][0],1);
            assert_eq!(db.query_sql("SELECT * FROM prices WHERE variant=''").unwrap().rows,before[0]);
        }
    }

    #[test]
    fn hofmann_cutover_survives_reopen_and_guards_every_public_price_writer() {
        let (dir, db, trader, material) = setup("hofmann-cutover-reopen");
        db.record_price(&quote(trader, material, "", "2026-10-14T03:00:00Z"))
            .unwrap();
        db.record_hofmann_tin_price(
            &quote(trader, material, "alt", "2026-10-16T03:00:00.000000001Z"),
            false,
        )
        .unwrap();
        let before = state(&db);
        drop(db);
        let db = PublicDb::open(dir.path()).unwrap();
        for at in [
            "2026-10-15T03:00:00Z",
            "2026-10-16T03:00:00.000000000+00:00",
            "2026-10-16T03:00:00.000000001Z",
        ] {
            assert!(
                db.record_price(&quote(trader, material, "", at)).is_err(),
                "generic writer replay {at}"
            );
            assert_eq!(state(&db), before);
        }
        // A real later grade is allowed even one nanosecond beyond cutover.
        db.record_price(&quote(
            trader,
            material,
            "",
            "2026-10-16T03:00:00.000000002Z",
        ))
        .unwrap();
        let distinct = db.current_price_for(trader, material, "").unwrap().unwrap();
        let after = state(&db);
        assert!(db
            .record_price(&quote(
                trader,
                material,
                "",
                "2026-10-16T03:00:00.000000001Z"
            ))
            .is_err());
        assert_eq!(
            state(&db),
            after,
            "same-ID dedupe must not rewind the later grade"
        );
        db.record_hofmann_tin_price(
            &quote(trader, material, "alt", "2026-10-17T03:00:00Z"),
            false,
        )
        .unwrap();
        assert_eq!(
            db.current_price_for(trader, material, "")
                .unwrap()
                .unwrap()
                .id,
            distinct.id
        );
        assert_eq!(
            db.query_sql("SELECT * FROM hofmann_tin_cutovers")
                .unwrap()
                .rows,
            before[2],
            "decision is immutable, not a moving sweep cutoff"
        );
    }

    #[test]
    fn hofmann_cutover_rejects_wrong_scope_and_foreign_typed_contract_without_writes() {
        let (_dir, db, trader, material) = setup("hofmann-cutover-scope");
        let before = state(&db);
        for case in [
            "label",
            "source",
            "basis",
            "unit",
            "trader",
            "variant",
            "currency",
            "unpublished",
            "producer",
        ] {
            let mut p = quote(trader, material, "alt", "2026-10-16T03:00:00Z");
            match case {
                "label" => p.notes = "Andere Zinnfraktion",
                "source" => p.source_url = "https://example.invalid/",
                "basis" => p.extra_json = "{\"map_v\":2,\"published_at_basis\":\"page_stated\"}",
                "unit" => p.unit = "EUR/t",
                "trader" => p.trader_id += 1,
                "variant" => p.variant = "rein",
                "currency" => p.currency = "USD",
                "unpublished" => p.published = false,
                "producer" => p.extra_json = "{\"map_v\":2,\"published_at_basis\":\"row_stated\",\"snapshot_owner\":\"other-producer\"}",
                _ => unreachable!(),
            }
            assert!(db.record_hofmann_tin_price(&p, false).is_err(), "{case}");
            assert_eq!(state(&db), before, "{case}: no partial quote or ledger");
        }
    }
}
