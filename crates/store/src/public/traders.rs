//! Traders: every Schrotthändler, Wertstoffhändler, Metallhändler,
//! Autoverwerter, Containerdienst, … in Germany.
//!
//! `trader_type` is one of: `schrotthaendler`, `wertstoffhaendler`,
//! `metallhaendler`, `autoverwertung`, `containerdienst`, `schrottplatz`,
//! `mobil` (itinerant buyer), `sonstige`.
//! `status` is one of: `aktiv`, `geschlossen`, `pruefung`, `unbekannt`.
//! `state` holds the Bundesland code (`BW`, `BY`, `BE`, `BB`, `HB`, `HH`,
//! `HE`, `MV`, `NI`, `NW`, `RP`, `SL`, `SN`, `ST`, `SH`, `TH`).
//!
//! Service conditions (`dropoff_json`, `pickup_json`) are JSON objects —
//! drop-off/pickup is never a plain yes/no but tied to conditions:
//! ```json
//! {"allowed": true, "customer_types": ["privat", "gewerbe"],
//!  "days": ["Mo", "Di", "Mi", "Do", "Fr"],
//!  "time_windows": ["08:00-16:00"],
//!  "min_quantity_kg": 50, "max_quantity_kg": null,
//!  "conditions": "nur mit Termin, keine Altautos"}
//! ```
//! A missing `allowed` (or `'{}'`) means unknown. `customer_types` uses
//! `privat`/`gewerbe`; `days` uses `Mo Di Mi Do Fr Sa So`.
//!
//! `extra_json` key registry (everything else stays in typed columns):
//! `seed_file`, `seed_section`, `ankauf_raw`, `seed_hash` (seed importer),
//! `review` (human review notes), `aliases` (array of former names),
//! `other_urls` (array of further web presences: Facebook, Kleinanzeigen…).
//! `notes` remains free prose (specialties from research); `description`
//! is the curated German description (filled by scrapers/enrichment).

use rusqlite::{params, OptionalExtension as _};

use super::PublicDb;
use crate::error::StoreError;

pub(super) const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS traders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    trader_type TEXT NOT NULL DEFAULT 'sonstige',
    description TEXT NOT NULL DEFAULT '',
    street TEXT NOT NULL DEFAULT '',
    postcode TEXT NOT NULL DEFAULT '',
    city TEXT NOT NULL DEFAULT '',
    state TEXT NOT NULL DEFAULT '',
    country TEXT NOT NULL DEFAULT 'DE',
    lat REAL,
    lon REAL,
    phone TEXT NOT NULL DEFAULT '',
    email TEXT NOT NULL DEFAULT '',
    website TEXT NOT NULL DEFAULT '',
    website_status TEXT NOT NULL DEFAULT 'unbekannt',
    website_checked_at TEXT NOT NULL DEFAULT '',
    opening_hours TEXT NOT NULL DEFAULT '',
    dropoff_json TEXT NOT NULL DEFAULT '{}',
    pickup_json TEXT NOT NULL DEFAULT '{}',
    min_quantity_kg REAL,
    max_quantity_kg REAL,
    certifications TEXT NOT NULL DEFAULT '[]',
    status TEXT NOT NULL DEFAULT 'aktiv',
    notes TEXT NOT NULL DEFAULT '',
    extra_json TEXT NOT NULL DEFAULT '{}',
    first_seen_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_traders_city ON traders(city);
CREATE INDEX IF NOT EXISTS idx_traders_postcode ON traders(postcode);
CREATE INDEX IF NOT EXISTS idx_traders_state ON traders(state);
CREATE INDEX IF NOT EXISTS idx_traders_type ON traders(trader_type);
CREATE INDEX IF NOT EXISTS idx_traders_status ON traders(status);
CREATE VIRTUAL TABLE IF NOT EXISTS traders_fts USING fts5(
    name, city, postcode, content='traders', content_rowid='id'
);
";

/// Content-sync triggers for `traders_fts`. Kept separate from [`SCHEMA`]
/// because migrations must drop them first: any ALTER/UPDATE on an FTS5
/// content table with these triggers present corrupts the schema.
pub(super) const TRIGGERS: &str = "
CREATE TRIGGER traders_ai AFTER INSERT ON traders BEGIN
    INSERT INTO traders_fts(rowid, name, city, postcode)
    VALUES (new.id, new.name, new.city, new.postcode);
END;
CREATE TRIGGER traders_ad AFTER DELETE ON traders BEGIN
    INSERT INTO traders_fts(traders_fts, rowid, name, city, postcode)
    VALUES ('delete', old.id, old.name, old.city, old.postcode);
END;
CREATE TRIGGER traders_au AFTER UPDATE ON traders BEGIN
    INSERT INTO traders_fts(traders_fts, rowid, name, city, postcode)
    VALUES ('delete', old.id, old.name, old.city, old.postcode);
    INSERT INTO traders_fts(rowid, name, city, postcode)
    VALUES (new.id, new.name, new.city, new.postcode);
END;
";

/// Bring an existing `traders` table up to the current schema.
/// Additive only (new code never depends on column order): missing columns
/// are added, and the service-condition JSON is backfilled once from the
/// legacy boolean flags, which stay in old files as inert leftovers.
/// Fresh databases already match.
pub(super) fn migrate(conn: &rusqlite::Connection) -> Result<(), StoreError> {
    // Triggers first: DDL/DML on an FTS5 content table with live
    // content-sync triggers corrupts the schema. The backfill below only
    // touches non-indexed columns, so no FTS rebuild is needed.
    conn.execute_batch(
        "DROP TRIGGER IF EXISTS traders_ai;
         DROP TRIGGER IF EXISTS traders_ad;
         DROP TRIGGER IF EXISTS traders_au;",
    )?;
    let mut cols = std::collections::HashSet::new();
    for col in conn
        .prepare("SELECT name FROM pragma_table_info('traders')")?
        .query_map([], |r| r.get::<_, String>(0))?
    {
        cols.insert(col?);
    }
    let add = |sql: &str| conn.execute_batch(sql);
    if !cols.contains("description") {
        add("ALTER TABLE traders ADD COLUMN description TEXT NOT NULL DEFAULT '';")?;
    }
    if !cols.contains("dropoff_json") {
        add("ALTER TABLE traders ADD COLUMN dropoff_json TEXT NOT NULL DEFAULT '{}';")?;
    }
    if !cols.contains("pickup_json") {
        add("ALTER TABLE traders ADD COLUMN pickup_json TEXT NOT NULL DEFAULT '{}';")?;
    }
    if !cols.contains("max_quantity_kg") {
        add("ALTER TABLE traders ADD COLUMN max_quantity_kg REAL;")?;
    }
    if !cols.contains("website_status") {
        add("ALTER TABLE traders ADD COLUMN website_status TEXT NOT NULL DEFAULT 'unbekannt';")?;
    }
    if !cols.contains("website_checked_at") {
        add("ALTER TABLE traders ADD COLUMN website_checked_at TEXT NOT NULL DEFAULT '';")?;
    }
    // Legacy boolean flags -> condition JSON happened once, long ago
    // (every live database already went through it). The '{}' value now
    // legitimately means "unknown", so no data backfill may ever rerun:
    // PRAGMA user_version merely marks the migration done. The flag
    // columns stay in old files as inert leftovers (DROP COLUMN on an
    // FTS5 content table corrupts the schema).
    let version: i64 = conn.query_row(
        "SELECT COALESCE(MAX(user_version), 0) FROM pragma_user_version",
        [],
        |r| r.get(0),
    )?;
    if version < 1 {
        conn.execute_batch("PRAGMA user_version = 1;")?;
    }
    conn.execute_batch(TRIGGERS)?;
    Ok(())
}

/// One trader row, as stored.
#[derive(Debug, Clone)]
pub struct TraderRow {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub trader_type: String,
    pub description: String,
    pub street: String,
    pub postcode: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub phone: String,
    pub email: String,
    pub website: String,
    pub website_status: String,
    pub website_checked_at: String,
    pub opening_hours: String,
    pub dropoff_json: String,
    pub pickup_json: String,
    pub min_quantity_kg: Option<f64>,
    pub max_quantity_kg: Option<f64>,
    pub certifications: String,
    pub status: String,
    pub notes: String,
    pub extra_json: String,
    pub first_seen_at: String,
    pub updated_at: String,
}

/// Fields for [`PublicDb::upsert_trader`]. Identity (`slug`) plus payload;
/// `first_seen_at` is only set on insert, everything else is refreshed.
pub struct NewTrader<'a> {
    pub slug: &'a str,
    pub name: &'a str,
    pub trader_type: &'a str,
    pub description: &'a str,
    pub street: &'a str,
    pub postcode: &'a str,
    pub city: &'a str,
    pub state: &'a str,
    pub country: &'a str,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub phone: &'a str,
    pub email: &'a str,
    pub website: &'a str,
    pub website_status: &'a str,
    pub website_checked_at: &'a str,
    pub opening_hours: &'a str,
    pub dropoff_json: &'a str,
    pub pickup_json: &'a str,
    pub min_quantity_kg: Option<f64>,
    pub max_quantity_kg: Option<f64>,
    pub certifications: &'a str,
    pub status: &'a str,
    pub notes: &'a str,
    pub extra_json: &'a str,
    pub now: &'a str,
}

/// Enrichment-owned columns the seed importer preserves: when the seed
/// row leaves them empty, the stored values survive (seed never clobbers
/// scraper/human enrichment).
#[derive(Debug, Clone, Default)]
pub struct SeedKept {
    pub seed_hash: Option<String>,
    pub description: String,
    pub dropoff_json: String,
    pub pickup_json: String,
    pub website: String,
    pub website_status: String,
    pub website_checked_at: String,
    pub phone: String,
    pub email: String,
    pub opening_hours: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

impl PublicDb {
    /// Insert a trader or refresh a known one (matched by `slug`).
    pub fn upsert_trader(&self, t: &NewTrader<'_>) -> Result<i64, StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO traders
             (slug, name, trader_type, description, street, postcode, city, state, country,
              lat, lon, phone, email, website, website_status, website_checked_at,
              opening_hours, dropoff_json, pickup_json,
              min_quantity_kg, max_quantity_kg,
              certifications, status, notes, extra_json, first_seen_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15,
                     ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?26)
             ON CONFLICT(slug) DO UPDATE SET
              name = excluded.name, trader_type = excluded.trader_type,
              description = excluded.description,
              street = excluded.street, postcode = excluded.postcode,
              city = excluded.city, state = excluded.state,
              country = excluded.country, lat = excluded.lat, lon = excluded.lon,
              phone = excluded.phone, email = excluded.email,
              website = excluded.website, website_status = excluded.website_status,
              website_checked_at = excluded.website_checked_at,
              opening_hours = excluded.opening_hours,
              dropoff_json = excluded.dropoff_json,
              pickup_json = excluded.pickup_json,
              min_quantity_kg = excluded.min_quantity_kg,
              max_quantity_kg = excluded.max_quantity_kg,
              certifications = excluded.certifications, status = excluded.status,
              notes = excluded.notes, extra_json = excluded.extra_json,
              updated_at = excluded.updated_at",
            params![
                t.slug,
                t.name,
                t.trader_type,
                t.description,
                t.street,
                t.postcode,
                t.city,
                t.state,
                t.country,
                t.lat,
                t.lon,
                t.phone,
                t.email,
                t.website,
                t.website_status,
                t.website_checked_at,
                t.opening_hours,
                t.dropoff_json,
                t.pickup_json,
                t.min_quantity_kg,
                t.max_quantity_kg,
                t.certifications,
                t.status,
                t.notes,
                t.extra_json,
                t.now,
            ],
        )?;
        let id: i64 = conn.query_row(
            "SELECT id FROM traders WHERE slug = ?1",
            params![t.slug],
            |r| r.get(0),
        )?;
        Ok(id)
    }

    /// Internal id for a trader slug, if known.
    pub fn find_trader_id(&self, slug: &str) -> Result<Option<i64>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT id FROM traders WHERE slug = ?1",
            params![slug],
            |r| r.get(0),
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Update contact/address fields from website enrichment (Impressum).
    /// Only non-empty extracted values are applied; `city` fills only an
    /// empty cell (seed districts like "Reinickendorf" are more precise
    /// than an Impressum "Berlin"). Returns the changed field names.
    /// `website_alive` marks a successfully fetched site (aktiv + check
    /// timestamp); a missing Impressum never marks anything tot.
    #[allow(clippy::too_many_arguments)]
    pub fn set_trader_info(
        &self,
        slug: &str,
        street: &str,
        postcode: &str,
        city: &str,
        phone: &str,
        email: &str,
        website_alive: bool,
        now: &str,
    ) -> Result<Vec<String>, StoreError> {
        let conn = self.lock()?;
        let cur: Option<(String, String, String, String, String)> = conn
            .query_row(
                "SELECT street, postcode, city, phone, email FROM traders WHERE slug = ?1",
                params![slug],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            )
            .optional()
            .map_err(StoreError::from)?;
        let Some((c_street, c_postcode, c_city, c_phone, c_email)) = cur else {
            return Ok(Vec::new());
        };
        let mut changed = Vec::new();
        let mut set = Vec::new();
        let mut args: Vec<rusqlite::types::Value> = Vec::new();
        // (column, extracted, current, fill_empty_only)
        let fields = [
            ("street", street, c_street.as_str(), false),
            ("postcode", postcode, c_postcode.as_str(), false),
            ("city", city, c_city.as_str(), true),
            ("phone", phone, c_phone.as_str(), false),
            ("email", email, c_email.as_str(), false),
        ];
        for (col, new, old, empty_only) in fields {
            if new.is_empty() || new == old || (empty_only && !old.is_empty()) {
                continue;
            }
            changed.push(col.to_owned());
            set.push(format!("{col} = ?"));
            args.push(rusqlite::types::Value::Text(new.to_owned()));
        }
        if website_alive {
            changed.push("website_status".to_owned());
            set.push("website_status = 'aktiv'".to_owned());
            changed.push("website_checked_at".to_owned());
            set.push("website_checked_at = ?".to_owned());
            args.push(rusqlite::types::Value::Text(now.to_owned()));
        }
        if set.is_empty() {
            return Ok(changed);
        }
        args.push(rusqlite::types::Value::Text(now.to_owned()));
        args.push(rusqlite::types::Value::Text(slug.to_owned()));
        let sql = format!(
            "UPDATE traders SET {}, updated_at = ? WHERE slug = ?",
            set.join(", ")
        );
        conn.execute(&sql, rusqlite::params_from_iter(args))?;
        Ok(changed)
    }

    /// Enrichment-owned columns for a trader slug: seed hash plus every
    /// column the seed importer preserves when the seed row leaves it
    /// empty (description, service conditions, website state).
    pub fn existing_seed_state(&self, slug: &str) -> Result<Option<SeedKept>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT extra_json, description, dropoff_json, pickup_json,
                    website, website_status, website_checked_at, phone,
                    email, opening_hours, lat, lon
             FROM traders WHERE slug = ?1",
            params![slug],
            |r| {
                let extra: String = r.get(0)?;
                let seed_hash = serde_json::from_str::<serde_json::Value>(&extra)
                    .ok()
                    .and_then(|v| v.get("seed_hash")?.as_str().map(str::to_owned));
                Ok(SeedKept {
                    seed_hash,
                    description: r.get(1)?,
                    dropoff_json: r.get(2)?,
                    pickup_json: r.get(3)?,
                    website: r.get(4)?,
                    website_status: r.get(5)?,
                    website_checked_at: r.get(6)?,
                    phone: r.get(7)?,
                    email: r.get(8)?,
                    opening_hours: r.get(9)?,
                    lat: r.get(10)?,
                    lon: r.get(11)?,
                })
            },
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Full-text search over name/city/postcode (FTS5, prefix matching).
    /// Used by tests and future ingestion; agents can use `traders_fts`
    /// directly in SQL (`JOIN traders_fts f ON t.id = f.rowid`).
    pub fn search_traders(&self, query: &str, limit: i64) -> Result<Vec<TraderRow>, StoreError> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare(
            "SELECT t.id, t.slug, t.name, t.trader_type, t.description, t.street, t.postcode,
                    t.city, t.state, t.country, t.lat, t.lon, t.phone, t.email,
                    t.website, t.website_status, t.website_checked_at, t.opening_hours,
                    t.dropoff_json, t.pickup_json,
                    t.min_quantity_kg, t.max_quantity_kg, t.certifications, t.status, t.notes,
                    t.extra_json, t.first_seen_at, t.updated_at
             FROM traders_fts f JOIN traders t ON t.id = f.rowid
             WHERE traders_fts MATCH ?1 ORDER BY rank LIMIT ?2",
        )?;
        let rows = stmt
            .query_map(params![query, limit], row_to_trader)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }
}

fn row_to_trader(r: &rusqlite::Row<'_>) -> rusqlite::Result<TraderRow> {
    Ok(TraderRow {
        id: r.get(0)?,
        slug: r.get(1)?,
        name: r.get(2)?,
        trader_type: r.get(3)?,
        description: r.get(4)?,
        street: r.get(5)?,
        postcode: r.get(6)?,
        city: r.get(7)?,
        state: r.get(8)?,
        country: r.get(9)?,
        lat: r.get(10)?,
        lon: r.get(11)?,
        phone: r.get(12)?,
        email: r.get(13)?,
        website: r.get(14)?,
        website_status: r.get(15)?,
        website_checked_at: r.get(16)?,
        opening_hours: r.get(17)?,
        dropoff_json: r.get(18)?,
        pickup_json: r.get(19)?,
        min_quantity_kg: r.get(20)?,
        max_quantity_kg: r.get(21)?,
        certifications: r.get(22)?,
        status: r.get(23)?,
        notes: r.get(24)?,
        extra_json: r.get(25)?,
        first_seen_at: r.get(26)?,
        updated_at: r.get(27)?,
    })
}

#[cfg(test)]
mod tests {
    use super::{NewTrader, PublicDb};

    fn trader<'a>(slug: &'a str, name: &'a str, city: &'a str, now: &'a str) -> NewTrader<'a> {
        NewTrader {
            slug,
            name,
            trader_type: "schrotthaendler",
            description: "Ankauf von Schrott und Metallen.",
            street: "",
            postcode: "",
            city,
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
            dropoff_json: "{\"allowed\":true,\"customer_types\":[\"privat\",\"gewerbe\"]}",
            pickup_json: "{\"allowed\":false}",
            min_quantity_kg: None,
            max_quantity_kg: None,
            certifications: "[]",
            status: "aktiv",
            notes: "",
            extra_json: "{}",
            now,
        }
    }

    #[test]
    fn upsert_and_fts_search() {
        let dir = std::env::temp_dir().join(format!("schrott-traders-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let db = PublicDb::open(&dir).expect("test db opens");
        let now = "2026-09-27T00:00:00Z";
        let id = db
            .upsert_trader(&trader(
                "mueller-berlin",
                "Müller Schrott GmbH",
                "Berlin",
                now,
            ))
            .expect("insert");
        assert_eq!(
            db.find_trader_id("mueller-berlin").expect("lookup"),
            Some(id)
        );
        // Update keeps the id, refreshes the payload.
        let id2 = db
            .upsert_trader(&trader(
                "mueller-berlin",
                "Müller Schrott AG",
                "Berlin",
                now,
            ))
            .expect("update");
        assert_eq!(id, id2);
        let hits = db.search_traders("Müller*", 10).expect("fts");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].name, "Müller Schrott AG");
        assert_eq!(
            hits[0].dropoff_json,
            "{\"allowed\":true,\"customer_types\":[\"privat\",\"gewerbe\"]}"
        );
        let none = db.search_traders("Hamburg*", 10).expect("fts");
        assert!(none.is_empty());
    }

    #[test]
    fn legacy_boolean_flags_stay_inert() {
        let dir = std::env::temp_dir().join(format!("schrott-migrate-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("dir");
        // Simulate a pre-migration database file with the boolean flags.
        {
            let conn = rusqlite::Connection::open(dir.join("public.db")).expect("old db");
            conn.execute_batch(
                "CREATE TABLE traders (
                    id INTEGER PRIMARY KEY AUTOINCREMENT, slug TEXT NOT NULL UNIQUE,
                    name TEXT NOT NULL, trader_type TEXT NOT NULL DEFAULT 'sonstige',
                    street TEXT NOT NULL DEFAULT '', postcode TEXT NOT NULL DEFAULT '',
                    city TEXT NOT NULL DEFAULT '', state TEXT NOT NULL DEFAULT '',
                    country TEXT NOT NULL DEFAULT 'DE', lat REAL, lon REAL,
                    phone TEXT NOT NULL DEFAULT '', email TEXT NOT NULL DEFAULT '',
                    website TEXT NOT NULL DEFAULT '', opening_hours TEXT NOT NULL DEFAULT '',
                    accepts_dropoff INTEGER NOT NULL DEFAULT 1,
                    accepts_pickup INTEGER NOT NULL DEFAULT 0,
                    min_quantity_kg REAL, certifications TEXT NOT NULL DEFAULT '[]',
                    status TEXT NOT NULL DEFAULT 'aktiv', notes TEXT NOT NULL DEFAULT '',
                    extra_json TEXT NOT NULL DEFAULT '{}',
                    first_seen_at TEXT NOT NULL, updated_at TEXT NOT NULL);",
            )
            .expect("old schema");
            conn.execute(
                "INSERT INTO traders (slug, name, accepts_dropoff, accepts_pickup,
                                      first_seen_at, updated_at)
                 VALUES ('alt-dealer', 'Alt Dealer', 1, 0, '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z')",
                [],
            )
            .expect("old row");
        }
        let db = PublicDb::open(&dir).expect("open migrates");
        // New columns exist; legacy flags are NOT reinterpreted (an empty
        // condition object means unknown — never backfilled from stale
        // booleans, which would clobber honest '{}' rows on every open).
        let res = db
            .query_sql(
                "SELECT dropoff_json, pickup_json, description, max_quantity_kg,
                        website_status FROM traders WHERE slug = 'alt-dealer'",
            )
            .expect("migrated columns");
        assert_eq!(res.rows.len(), 1);
        assert_eq!(res.rows[0][0].as_str(), Some("{}"));
        assert_eq!(res.rows[0][1].as_str(), Some("{}"));
        let cols = db
            .query_sql("SELECT name FROM pragma_table_info('traders')")
            .expect("pragma");
        let names: Vec<&str> = cols
            .rows
            .iter()
            .filter_map(|r| r.first()?.as_str())
            .collect();
        assert!(names.contains(&"dropoff_json"));
        assert!(names.contains(&"description"));
        assert!(names.contains(&"website_status"));
    }
}
