//! Reviewer-only adversarial tests; not part of hofmann-upgrade-v2.patch.
use schrott_mcp_store::{NewPrice, PublicDb};
use std::path::PathBuf;

const URL: &str = "https://hofmann-entsorgung.de/preise-und-verguetungen/";

fn setup(label: &str) -> (PathBuf, PublicDb, i64, i64) {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "hofmann-v2-review-{label}-{}-{nonce}",
        std::process::id()
    ));
    let db = PublicDb::open(&dir).unwrap();
    let conn = rusqlite::Connection::open(dir.join("public.db")).unwrap();
    conn.execute("INSERT INTO traders(slug,name,first_seen_at,updated_at) VALUES('bw-rastatt-hofmann','Hofmann','2026-10-14T03:00:00Z','2026-10-14T03:00:00Z')", []).unwrap();
    let trader = conn.last_insert_rowid();
    conn.execute("INSERT INTO materials(slug,name_de,category,unit,updated_at) VALUES('zinn','Zinn','nichteisen','EUR/kg','2026-10-14T03:00:00Z')", []).unwrap();
    let material = conn.last_insert_rowid();
    drop(conn);
    (dir, db, trader, material)
}

fn quote<'a>(
    trader: i64,
    material: i64,
    variant: &'a str,
    at: &'a str,
    value: f64,
) -> NewPrice<'a> {
    NewPrice {
        trader_id: trader,
        material_id: material,
        variant,
        price: value,
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

fn cutover(db: &PublicDb, trader: i64, material: i64) {
    db.record_price(&quote(trader, material, "", "2026-10-14T03:00:00Z", 11.5))
        .unwrap();
    db.record_hofmann_tin_price(
        &quote(trader, material, "alt", "2026-10-16T03:00:00Z", 11.5),
        false,
    )
    .unwrap();
}

#[test]
fn review_generic_newer_fraction_must_advance_current() {
    let (_dir, db, trader, material) = setup("generic-fraction");
    cutover(&db, trader, material);
    let first = db
        .record_price(&quote(trader, material, "", "2026-10-16T03:00:01.1Z", 12.0))
        .unwrap();
    let second = db
        .record_price(&quote(
            trader,
            material,
            "",
            "2026-10-16T03:00:01.100000001Z",
            13.0,
        ))
        .unwrap();
    assert_ne!(
        first, second,
        "both changed-price observations were appended"
    );
    let current = db.current_price_for(trader, material, "").unwrap().unwrap();
    assert_eq!(
        current.id, second,
        "a valid +1ns later generic observation must become Current; old price is {}",
        current.price
    );
    assert_eq!(current.price, 13.0);
}

#[test]
fn review_typed_newer_fraction_must_advance_existing_generic_current() {
    let (_dir, db, trader, material) = setup("typed-fraction");
    cutover(&db, trader, material);
    db.record_price(&quote(
        trader,
        material,
        "alt",
        "2026-10-16T03:00:01Z",
        12.0,
    ))
    .unwrap();
    let second = db
        .record_hofmann_tin_price(
            &quote(
                trader,
                material,
                "alt",
                "2026-10-16T03:00:01.000000001Z",
                13.0,
            ),
            false,
        )
        .unwrap();
    let current = db
        .current_price_for(trader, material, "alt")
        .unwrap()
        .unwrap();
    assert_eq!(
        current.id, second,
        "typed write accepted +1ns after generic Current, but must also advance its pointer; old price is {}",
        current.price
    );
    assert_eq!(current.price, 13.0);
}

#[test]
fn review_invalid_calendar_date_must_not_be_accepted_as_utc() {
    let (_dir, db, trader, material) = setup("invalid-calendar");
    let invalid = quote(trader, material, "alt", "2026-02-31T03:00:00Z", 11.5);
    assert!(
        db.record_hofmann_tin_price(&invalid, false).is_err(),
        "February 31 must not establish a persistent cutover"
    );
}

#[test]
fn review_boot_from_v6_without_ledger_is_additive_and_preserves_prices() {
    let (dir, db, trader, material) = setup("boot-v6");
    db.record_price(&quote(trader, material, "", "2026-10-14T03:00:00Z", 11.5))
        .unwrap();
    let before = db
        .query_sql("SELECT * FROM prices ORDER BY id")
        .unwrap()
        .rows;
    let current_before = db
        .query_sql("SELECT * FROM current_prices ORDER BY price_id")
        .unwrap()
        .rows;
    drop(db);
    let conn = rusqlite::Connection::open(dir.join("public.db")).unwrap();
    conn.execute("DROP TABLE hofmann_tin_cutovers", []).unwrap();
    // setup() opens the latest schema. Reconstruct the explicit v6 legacy
    // state under test rather than requiring the latest schema to stay at 6.
    conn.pragma_update(None, "user_version", 6).unwrap();
    let version: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(version, 6);
    drop(conn);
    let db = PublicDb::open(&dir).unwrap();
    assert_eq!(
        db.query_sql("SELECT * FROM prices ORDER BY id")
            .unwrap()
            .rows,
        before
    );
    assert_eq!(
        db.query_sql("SELECT * FROM current_prices ORDER BY price_id")
            .unwrap()
            .rows,
        current_before
    );
    assert_eq!(
        db.query_sql("SELECT COUNT(*) FROM hofmann_tin_cutovers")
            .unwrap()
            .rows[0][0],
        0
    );
    assert_eq!(
        db.query_sql("SELECT user_version FROM pragma_user_version")
            .unwrap()
            .rows[0][0],
        7
    );
}

#[test]
fn v3_guarded_pointer_orders_valid_utc_forms_and_preserves_existing_history() {
    for variant in ["", "alt"] {
        for typed in [false, true] {
            for (first_at, second_at) in [
                ("2026-10-16T03:00:01.1Z", "2026-10-16T03:00:01.100000001Z"),
                ("2026-10-16T03:00:01Z", "2026-10-16T03:00:01.000000001Z"),
                // Same instant: keep the Store's existing higher-ID tie break.
                (
                    "2026-10-16T03:00:01.1Z",
                    "2026-10-16T03:00:01.100000000+00:00",
                ),
                (
                    "2026-10-16T03:00:01+00:00",
                    "2026-10-16T03:00:01.000000000Z",
                ),
                (
                    "2026-10-16T03:00:01.100000001Z",
                    "2026-10-16T03:00:01.100000002+00:00",
                ),
            ] {
                let (_dir, db, trader, material) = setup("v3-order-matrix");
                cutover(&db, trader, material);
                let first = db
                    .record_price(&quote(trader, material, variant, first_at, 12.0))
                    .unwrap();
                let old_row = db
                    .query_sql(&format!("SELECT * FROM prices WHERE id={first}"))
                    .unwrap()
                    .rows;
                let next = quote(trader, material, variant, second_at, 13.0);
                let second = if typed {
                    db.record_hofmann_tin_price(&next, variant.is_empty())
                        .unwrap()
                } else {
                    db.record_price(&next).unwrap()
                };
                assert_ne!(first, second);
                let current = db
                    .current_price_for(trader, material, variant)
                    .unwrap()
                    .unwrap();
                assert_eq!(
                    (current.id, current.price),
                    (second, 13.0),
                    "variant={variant:?}, typed={typed}, first={first_at}, second={second_at}"
                );
                assert_eq!(
                    db.query_sql(&format!("SELECT * FROM prices WHERE id={first}"))
                        .unwrap()
                        .rows,
                    old_row,
                    "no historical timestamp normalization or rewrite"
                );
            }
        }
    }
}

fn persisted_state(db: &PublicDb) -> [Vec<Vec<serde_json::Value>>; 3] {
    [
        "SELECT * FROM prices ORDER BY id",
        "SELECT * FROM current_prices ORDER BY price_id",
        "SELECT * FROM hofmann_tin_cutovers ORDER BY trader_id,material_id",
    ]
    .map(|sql| db.query_sql(sql).unwrap().rows)
}

#[test]
fn v3_calendar_validation_is_strict_for_public_tin_apis_without_partial_writes() {
    let (_dir, db, trader, material) = setup("v3-invalid-calendar");
    let empty = persisted_state(&db);
    for invalid in [
        "2026-02-31T03:00:00Z",
        "2026-02-29T03:00:00Z",
        "1900-02-29T03:00:00Z",
        "2026-04-31T03:00:00Z",
        "2026-00-10T03:00:00Z",
        "2026-13-10T03:00:00Z",
        "2026-10-00T03:00:00Z",
        "2026-10-32T03:00:00Z",
        "2026-10-16T24:00:00Z",
        "2026-10-16T03:60:00Z",
        "2026-10-16T03:00:60Z",
    ] {
        assert!(
            db.record_hofmann_tin_price(&quote(trader, material, "alt", invalid, 11.5), false)
                .is_err(),
            "observed_at={invalid}"
        );
        assert_eq!(persisted_state(&db), empty);
        let mut valid = quote(trader, material, "alt", "2026-10-16T03:00:00Z", 11.5);
        valid.ingested_at = invalid;
        assert!(
            db.record_hofmann_tin_price(&valid, false).is_err(),
            "ingested_at={invalid}"
        );
        assert_eq!(persisted_state(&db), empty);
        assert!(
            db.preserve_hofmann_tin_grades(trader, invalid).is_err(),
            "coexistence={invalid}"
        );
        assert_eq!(persisted_state(&db), empty);
    }
    cutover(&db, trader, material);
    let before = persisted_state(&db);
    assert!(
        db.record_price(&quote(trader, material, "", "2027-02-31T03:00:00Z", 12.0))
            .is_err(),
        "generic guarded writer must strictly validate too, not merely compare an impossible future day"
    );
    assert_eq!(persisted_state(&db), before);
}

#[test]
fn v3_calendar_accepts_real_leap_days_and_valid_month_ends() {
    for valid in [
        "2000-02-29T03:00:00Z",
        "2024-02-29T03:00:00.1+00:00",
        "2026-02-28T03:00:00Z",
        "2026-04-30T03:00:00.000000001Z",
    ] {
        let (_dir, db, trader, material) = setup("v3-valid-calendar");
        let id = db
            .record_hofmann_tin_price(&quote(trader, material, "alt", valid, 11.5), false)
            .unwrap();
        assert_eq!(
            db.current_price_for(trader, material, "alt")
                .unwrap()
                .unwrap()
                .id,
            id
        );
        assert_eq!(
            db.query_sql("SELECT COUNT(*) FROM hofmann_tin_cutovers")
                .unwrap()
                .rows[0][0],
            1
        );
    }
}

#[test]
fn v3_first_typed_write_also_advances_existing_generic_utc_current() {
    for variant in ["", "alt"] {
        let (_dir, db, trader, material) = setup("v3-first-typed-write");
        // No cutover yet: the typed write must still honor its own accepted
        // normalized time when Current was written with another UTC spelling.
        let first = db
            .record_price(&quote(
                trader,
                material,
                variant,
                "2026-10-16T03:00:01Z",
                12.0,
            ))
            .unwrap();
        let before = db
            .query_sql(&format!("SELECT * FROM prices WHERE id={first}"))
            .unwrap()
            .rows;
        let second = db
            .record_hofmann_tin_price(
                &quote(
                    trader,
                    material,
                    variant,
                    "2026-10-16T03:00:01.000000001Z",
                    13.0,
                ),
                variant.is_empty(),
            )
            .unwrap();
        let current = db
            .current_price_for(trader, material, variant)
            .unwrap()
            .unwrap();
        assert_eq!((current.id, current.price), (second, 13.0));
        assert_eq!(
            db.query_sql(&format!("SELECT * FROM prices WHERE id={first}"))
                .unwrap()
                .rows,
            before
        );
    }
}
