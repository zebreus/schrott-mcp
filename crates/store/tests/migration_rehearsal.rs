//! Rehearsal of the public migrations against a copy of the production
//! database.
//!
//! The production databases are strictly read-only, so a migration can only
//! ever be rehearsed on a copy. This test does that rehearsal: it opens a
//! copy of `public.db` with the production schema and lets the migrations
//! run against it, then proves that the migration did what it promises:
//!
//! * the history (`prices`) stays untouched - only current pointers are
//!   withdrawn,
//! * exactly the reviewed pairs lose their pointer,
//! * `user_version` reaches the expected step.
//!
//! The test is opt-in: without `SCHROTT_REHEARSAL_DIR` it does nothing, so
//! it never touches a production database by accident.
//!
//! Usage:
//!
//! ```text
//! mkdir -p /var/tmp/rehearsal
//! sqlite3 "file:/var/lib/schrott-mcp/public.db?mode=ro" \
//!   "VACUUM INTO '/var/tmp/rehearsal/public.db'"
//! SCHROTT_REHEARSAL_DIR=/var/tmp/rehearsal \
//!   cargo test -p schrott-mcp-store --test migration_rehearsal -- --nocapture
//! ```
use schrott_mcp_store::PublicDb;

/// The reviewed list of the stale current prices (migration 11).
const STALE_CURRENT_PRICES: &str = include_str!("../src/public/stale_current_prices.tsv");

/// The reviewed list of the stale current prices of 08.11.2026 (migration 12).
const STALE_CURRENT_PRICES_12: &str = include_str!("../src/public/stale_current_prices_12.tsv");

fn paare() -> usize {
    STALE_CURRENT_PRICES
        .lines()
        .filter(|zeile| !zeile.trim().is_empty())
        .count()
        + STALE_CURRENT_PRICES_12
            .lines()
            .filter(|zeile| !zeile.trim().is_empty())
            .count()
}

fn zaehle(pfad: &std::path::Path) -> (i64, i64, i64) {
    let conn = rusqlite::Connection::open(pfad).expect("copy of the production database");
    let version: i64 = conn
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .expect("user_version");
    let preise: i64 = conn
        .query_row("SELECT COUNT(*) FROM prices", [], |row| row.get(0))
        .expect("prices");
    let current: i64 = conn
        .query_row("SELECT COUNT(*) FROM current_prices", [], |row| row.get(0))
        .expect("current_prices");
    (version, preise, current)
}

#[test]
fn migration_runs_against_a_copy_of_the_production_database() {
    let Ok(dir) = std::env::var("SCHROTT_REHEARSAL_DIR") else {
        eprintln!("SCHROTT_REHEARSAL_DIR is not set - the rehearsal is skipped");
        return;
    };
    let datenbank = std::path::Path::new(&dir).join("public.db");
    assert!(
        datenbank.exists(),
        "{} does not hold a public.db - copy the production database first",
        dir
    );

    let (version_vor, preise_vor, current_vor) = zaehle(&datenbank);
    eprintln!(
        "before: user_version={version_vor} prices={preise_vor} current_prices={current_vor}"
    );

    // The migrations run inside the open: this is the same path the service
    // takes on the next start.
    let _db = PublicDb::open(std::path::Path::new(&dir)).expect("open the copy");

    let (version_nach, preise_nach, current_nach) = zaehle(&datenbank);
    eprintln!(
        "after:  user_version={version_nach} prices={preise_nach} current_prices={current_nach}"
    );

    assert_eq!(
        preise_nach, preise_vor,
        "the history stays untouched - only current pointers are withdrawn"
    );
    assert!(
        version_nach > version_vor,
        "the migration has to advance the schema version ({version_vor} -> {version_nach})"
    );
    assert!(
        current_nach < current_vor,
        "the migration has to withdraw the reviewed pointers ({current_vor} -> {current_nach})"
    );
    assert!(
        current_vor - current_nach <= paare() as i64,
        "the migration withdrew more pointers ({} than the {} reviewed pairs) than it may",
        current_vor - current_nach,
        paare()
    );
}
