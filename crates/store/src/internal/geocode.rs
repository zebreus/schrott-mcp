//! Address → coordinates cache, independent of trader rows.
//!
//! One external lookup per distinct address: traders sharing an address
//! reuse the cached result instead of re-querying. Entries are never
//! updated or deleted — when a trader moves, the new address gets its own
//! row and the old one stays for whoever still lives there. First result
//! wins (`INSERT OR IGNORE`).

use rusqlite::params;

use super::InternalDb;
use crate::error::StoreError;

pub(super) const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS geocode_cache (
    address_key TEXT PRIMARY KEY,
    street TEXT NOT NULL DEFAULT '',
    postcode TEXT NOT NULL DEFAULT '',
    city TEXT NOT NULL DEFAULT '',
    lat REAL NOT NULL,
    lon REAL NOT NULL,
    source TEXT NOT NULL DEFAULT '',
    geocoded_at TEXT NOT NULL
);
";

/// Canonical cache key: `street|postcode|city`, lowercased and
/// whitespace-collapsed. Must stay in sync with `tools/geocode_full.py`
/// and the serial Nominatim script (same normalization there).
pub fn geocode_key(street: &str, postcode: &str, city: &str) -> String {
    fn norm(s: &str) -> String {
        s.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase()
    }
    format!("{}|{}|{}", norm(street), postcode.trim(), norm(city))
}

impl InternalDb {
    /// Cached coordinates for an address key, if any.
    pub fn geocode_lookup(&self, address_key: &str) -> Result<Option<(f64, f64)>, StoreError> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT lat, lon FROM geocode_cache WHERE address_key = ?1")?;
        let mut rows = stmt.query(params![address_key])?;
        if let Some(r) = rows.next()? {
            Ok(Some((r.get(0)?, r.get(1)?)))
        } else {
            Ok(None)
        }
    }

    /// Store a fresh result. Existing entries are kept (first result wins).
    /// Returns true when a new row was inserted.
    #[allow(clippy::too_many_arguments)]
    pub fn geocode_store(
        &self,
        address_key: &str,
        street: &str,
        postcode: &str,
        city: &str,
        lat: f64,
        lon: f64,
        source: &str,
        now: &str,
    ) -> Result<bool, StoreError> {
        let conn = self.lock()?;
        let n = conn.execute(
            "INSERT OR IGNORE INTO geocode_cache
             (address_key, street, postcode, city, lat, lon, source, geocoded_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![address_key, street, postcode, city, lat, lon, source, now],
        )?;
        Ok(n == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::{geocode_key, InternalDb};
    use crate::test_support::TempDbDir;

    fn temp_db(name: &str) -> (TempDbDir, InternalDb) {
        let dir = TempDbDir::new(name);
        let db = InternalDb::open(dir.path()).expect("test db opens");
        (dir, db)
    }

    #[test]
    fn key_normalization_is_stable() {
        assert_eq!(
            geocode_key("  Werner-von-Siemens-Str. 12 ", "15566", "Schöneiche"),
            "werner-von-siemens-str. 12|15566|schöneiche"
        );
    }

    #[test]
    fn first_result_wins_and_survives() {
        let (_dir, db) = temp_db("geocache");
        let now = "2026-09-30T00:00:00Z";
        assert!(db.geocode_lookup("a|1|b").expect("lookup").is_none());
        assert!(db
            .geocode_store("a|1|b", "A", "1", "B", 52.5, 13.4, "nominatim", now)
            .expect("store"));
        assert_eq!(
            db.geocode_lookup("a|1|b").expect("lookup"),
            Some((52.5, 13.4))
        );
        // Second store for the same address does not overwrite.
        assert!(!db
            .geocode_store("a|1|b", "A", "1", "B", 0.0, 0.0, "other", now)
            .expect("store"));
        assert_eq!(
            db.geocode_lookup("a|1|b").expect("lookup"),
            Some((52.5, 13.4))
        );
    }
}
