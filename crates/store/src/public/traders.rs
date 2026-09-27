//! Traders: every Schrotthändler, Wertstoffhändler, Metallhändler,
//! Autoverwerter, Containerdienst, … in Germany.
//!
//! `trader_type` is one of: `schrotthaendler`, `wertstoffhaendler`,
//! `metallhaendler`, `autoverwertung`, `containerdienst`, `schrottplatz`,
//! `mobil` (itinerant buyer), `sonstige`.
//! `status` is one of: `aktiv`, `geschlossen`, `pruefung`, `unbekannt`.
//! `state` holds the Bundesland code (`BW`, `BY`, `BE`, `BB`, `HB`, `HH`,
//! `HE`, `MV`, `NI`, `NW`, `RP`, `SL`, `SN`, `ST`, `SH`, `TH`).

use rusqlite::{params, OptionalExtension as _};

use super::PublicDb;
use crate::error::StoreError;

pub(super) const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS traders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    trader_type TEXT NOT NULL DEFAULT 'sonstige',
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
    opening_hours TEXT NOT NULL DEFAULT '',
    accepts_dropoff INTEGER NOT NULL DEFAULT 1,
    accepts_pickup INTEGER NOT NULL DEFAULT 0,
    min_quantity_kg REAL,
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
CREATE TRIGGER IF NOT EXISTS traders_ai AFTER INSERT ON traders BEGIN
    INSERT INTO traders_fts(rowid, name, city, postcode)
    VALUES (new.id, new.name, new.city, new.postcode);
END;
CREATE TRIGGER IF NOT EXISTS traders_ad AFTER DELETE ON traders BEGIN
    INSERT INTO traders_fts(traders_fts, rowid, name, city, postcode)
    VALUES ('delete', old.id, old.name, old.city, old.postcode);
END;
CREATE TRIGGER IF NOT EXISTS traders_au AFTER UPDATE ON traders BEGIN
    INSERT INTO traders_fts(traders_fts, rowid, name, city, postcode)
    VALUES ('delete', old.id, old.name, old.city, old.postcode);
    INSERT INTO traders_fts(rowid, name, city, postcode)
    VALUES (new.id, new.name, new.city, new.postcode);
END;
";

/// One trader row, as stored.
#[derive(Debug, Clone)]
pub struct TraderRow {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub trader_type: String,
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
    pub opening_hours: String,
    pub accepts_dropoff: bool,
    pub accepts_pickup: bool,
    pub min_quantity_kg: Option<f64>,
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
    pub opening_hours: &'a str,
    pub accepts_dropoff: bool,
    pub accepts_pickup: bool,
    pub min_quantity_kg: Option<f64>,
    pub certifications: &'a str,
    pub status: &'a str,
    pub notes: &'a str,
    pub extra_json: &'a str,
    pub now: &'a str,
}

impl PublicDb {
    /// Insert a trader or refresh a known one (matched by `slug`).
    pub fn upsert_trader(&self, t: &NewTrader<'_>) -> Result<i64, StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO traders
             (slug, name, trader_type, street, postcode, city, state, country,
              lat, lon, phone, email, website, opening_hours,
              accepts_dropoff, accepts_pickup, min_quantity_kg,
              certifications, status, notes, extra_json, first_seen_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
                     ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?22)
             ON CONFLICT(slug) DO UPDATE SET
              name = excluded.name, trader_type = excluded.trader_type,
              street = excluded.street, postcode = excluded.postcode,
              city = excluded.city, state = excluded.state,
              country = excluded.country, lat = excluded.lat, lon = excluded.lon,
              phone = excluded.phone, email = excluded.email,
              website = excluded.website, opening_hours = excluded.opening_hours,
              accepts_dropoff = excluded.accepts_dropoff,
              accepts_pickup = excluded.accepts_pickup,
              min_quantity_kg = excluded.min_quantity_kg,
              certifications = excluded.certifications, status = excluded.status,
              notes = excluded.notes, extra_json = excluded.extra_json,
              updated_at = excluded.updated_at",
            params![
                t.slug,
                t.name,
                t.trader_type,
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
                t.opening_hours,
                i64::from(t.accepts_dropoff),
                i64::from(t.accepts_pickup),
                t.min_quantity_kg,
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

    /// Stored seed payload hash for a trader slug, if it was seed-imported.
    /// The seed importer compares this to skip unchanged rows.
    pub fn trader_seed_hash(&self, slug: &str) -> Result<Option<String>, StoreError> {
        let conn = self.lock()?;
        let extra: Option<String> = conn
            .query_row(
                "SELECT extra_json FROM traders WHERE slug = ?1",
                params![slug],
                |r| r.get(0),
            )
            .optional()
            .map_err(StoreError::from)?;
        Ok(extra.and_then(|e| {
            serde_json::from_str::<serde_json::Value>(&e)
                .ok()?
                .get("seed_hash")?
                .as_str()
                .map(str::to_owned)
        }))
    }

    /// Full-text search over name/city/postcode (FTS5, prefix matching).
    /// Used by tests and future ingestion; agents can use `traders_fts`
    /// directly in SQL (`JOIN traders_fts f ON t.id = f.rowid`).
    pub fn search_traders(&self, query: &str, limit: i64) -> Result<Vec<TraderRow>, StoreError> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare(
            "SELECT t.id, t.slug, t.name, t.trader_type, t.street, t.postcode,
                    t.city, t.state, t.country, t.lat, t.lon, t.phone, t.email,
                    t.website, t.opening_hours, t.accepts_dropoff, t.accepts_pickup,
                    t.min_quantity_kg, t.certifications, t.status, t.notes,
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
        street: r.get(4)?,
        postcode: r.get(5)?,
        city: r.get(6)?,
        state: r.get(7)?,
        country: r.get(8)?,
        lat: r.get(9)?,
        lon: r.get(10)?,
        phone: r.get(11)?,
        email: r.get(12)?,
        website: r.get(13)?,
        opening_hours: r.get(14)?,
        accepts_dropoff: r.get::<_, i64>(15)? != 0,
        accepts_pickup: r.get::<_, i64>(16)? != 0,
        min_quantity_kg: r.get(17)?,
        certifications: r.get(18)?,
        status: r.get(19)?,
        notes: r.get(20)?,
        extra_json: r.get(21)?,
        first_seen_at: r.get(22)?,
        updated_at: r.get(23)?,
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
            opening_hours: "",
            accepts_dropoff: true,
            accepts_pickup: false,
            min_quantity_kg: None,
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
            .upsert_trader(&trader("mueller-berlin", "Müller Schrott GmbH", "Berlin", now))
            .expect("insert");
        assert_eq!(db.find_trader_id("mueller-berlin").expect("lookup"), Some(id));
        // Update keeps the id, refreshes the payload.
        let id2 = db
            .upsert_trader(&trader("mueller-berlin", "Müller Schrott AG", "Berlin", now))
            .expect("update");
        assert_eq!(id, id2);
        let hits = db.search_traders("Müller*", 10).expect("fts");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].name, "Müller Schrott AG");
        let none = db.search_traders("Hamburg*", 10).expect("fts");
        assert!(none.is_empty());
    }
}
