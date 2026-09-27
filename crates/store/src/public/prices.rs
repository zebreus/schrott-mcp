//! Prices: append-only observations plus a materialized "current" table.
//!
//! Every observation answers four questions at once:
//! - *What?* `trader_id` + `material_id`, `price` in `currency` per `unit`.
//! - *How sure?* `price_min`/`price_max` span the plausible range
//!   (`NULL` = exact); `confidence` is 0..1 (`NULL` = unknown).
//! - *From whom?* `source_type` (`haendler_angabe`, `portal`, `dritte`,
//!   `telefonisch`, `vor_ort`, `schaetzung`, `unbekannt`) plus `published`
//!   (1 = the trader published the price themselves) and `source_url`.
//! - *When true?* `observed_at` (when we saw it), `published_at` (when the
//!   trader published it, `NULL` = unknown), `valid_from`/`valid_to`
//!   (validity window, `NULL` = open-ended).
//!
//! `current_prices` holds exactly one row per trader + material — the id of
//! the newest observation by `observed_at` — so "what does X pay right now?"
//! is a single indexed join instead of a window-function scan. The
//! `v_current_prices` view pre-joins everything agents usually want.

use rusqlite::{params, OptionalExtension as _};

use super::PublicDb;
use crate::error::StoreError;

pub(super) const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS prices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    trader_id INTEGER NOT NULL REFERENCES traders(id),
    material_id INTEGER NOT NULL REFERENCES materials(id),
    price REAL NOT NULL,
    currency TEXT NOT NULL DEFAULT 'EUR',
    unit TEXT NOT NULL DEFAULT 'EUR/kg',
    price_min REAL,
    price_max REAL,
    confidence REAL,
    source_type TEXT NOT NULL DEFAULT 'unbekannt',
    published INTEGER NOT NULL DEFAULT 0,
    source_url TEXT NOT NULL DEFAULT '',
    observed_at TEXT NOT NULL,
    published_at TEXT,
    valid_from TEXT,
    valid_to TEXT,
    notes TEXT NOT NULL DEFAULT '',
    extra_json TEXT NOT NULL DEFAULT '{}',
    ingested_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_prices_material_time ON prices(material_id, observed_at);
CREATE INDEX IF NOT EXISTS idx_prices_trader_time ON prices(trader_id, observed_at);
CREATE INDEX IF NOT EXISTS idx_prices_trader_material_time
    ON prices(trader_id, material_id, observed_at);
CREATE TABLE IF NOT EXISTS current_prices (
    trader_id INTEGER NOT NULL,
    material_id INTEGER NOT NULL,
    price_id INTEGER NOT NULL REFERENCES prices(id),
    updated_at TEXT NOT NULL,
    PRIMARY KEY (trader_id, material_id)
);
CREATE TABLE IF NOT EXISTS trader_materials (
    trader_id INTEGER NOT NULL REFERENCES traders(id),
    material_id INTEGER NOT NULL REFERENCES materials(id),
    accepts INTEGER NOT NULL DEFAULT 1,
    conditions TEXT NOT NULL DEFAULT '',
    valid_from TEXT,
    valid_to TEXT,
    observed_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (trader_id, material_id)
);
CREATE INDEX IF NOT EXISTS idx_trader_materials_material ON trader_materials(material_id);
CREATE VIEW IF NOT EXISTS v_current_prices AS
SELECT t.slug AS trader_slug, t.name AS trader, t.city, t.postcode, t.state,
       m.slug AS material_slug, m.name_de AS material, m.category,
       p.price, p.currency, p.unit, p.price_min, p.price_max, p.confidence,
       p.source_type, p.published, p.source_url,
       p.observed_at, p.published_at, p.valid_from, p.valid_to
FROM current_prices c
JOIN prices p ON p.id = c.price_id
JOIN traders t ON t.id = c.trader_id
JOIN materials m ON m.id = c.material_id;
";

/// One price observation, as stored.
#[derive(Debug, Clone)]
pub struct PriceRow {
    pub id: i64,
    pub trader_id: i64,
    pub material_id: i64,
    pub price: f64,
    pub currency: String,
    pub unit: String,
    pub price_min: Option<f64>,
    pub price_max: Option<f64>,
    pub confidence: Option<f64>,
    pub source_type: String,
    pub published: bool,
    pub source_url: String,
    pub observed_at: String,
    pub published_at: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub notes: String,
    pub extra_json: String,
    pub ingested_at: String,
}

/// Fields for [`PublicDb::record_price`].
pub struct NewPrice<'a> {
    pub trader_id: i64,
    pub material_id: i64,
    pub price: f64,
    pub currency: &'a str,
    pub unit: &'a str,
    pub price_min: Option<f64>,
    pub price_max: Option<f64>,
    pub confidence: Option<f64>,
    pub source_type: &'a str,
    pub published: bool,
    pub source_url: &'a str,
    pub observed_at: &'a str,
    pub published_at: Option<&'a str>,
    pub valid_from: Option<&'a str>,
    pub valid_to: Option<&'a str>,
    pub notes: &'a str,
    pub extra_json: &'a str,
    pub ingested_at: &'a str,
}

impl PublicDb {
    /// Append one observation and refresh the materialized current price.
    /// Out-of-order backfills never clobber newer data: `current_prices`
    /// only moves forward in `observed_at` (ties: higher id wins).
    pub fn record_price(&self, p: &NewPrice<'_>) -> Result<i64, StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO prices
             (trader_id, material_id, price, currency, unit,
              price_min, price_max, confidence, source_type, published, source_url,
              observed_at, published_at, valid_from, valid_to,
              notes, extra_json, ingested_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            params![
                p.trader_id,
                p.material_id,
                p.price,
                p.currency,
                p.unit,
                p.price_min,
                p.price_max,
                p.confidence,
                p.source_type,
                i64::from(p.published),
                p.source_url,
                p.observed_at,
                p.published_at,
                p.valid_from,
                p.valid_to,
                p.notes,
                p.extra_json,
                p.ingested_at,
            ],
        )?;
        let price_id = conn.last_insert_rowid();
        // Move the materialized pointer only forward in observation time.
        let current: Option<(i64, String)> = conn
            .query_row(
                "SELECT c.price_id, p.observed_at FROM current_prices c
                 JOIN prices p ON p.id = c.price_id
                 WHERE c.trader_id = ?1 AND c.material_id = ?2",
                params![p.trader_id, p.material_id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        let newer = match current {
            None => true,
            Some((old_id, old_observed)) => {
                (p.observed_at, price_id) > (old_observed.as_str(), old_id)
            }
        };
        if newer {
            conn.execute(
                "INSERT INTO current_prices (trader_id, material_id, price_id, updated_at)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(trader_id, material_id) DO UPDATE SET
                  price_id = excluded.price_id, updated_at = excluded.updated_at",
                params![p.trader_id, p.material_id, price_id, p.ingested_at],
            )?;
        }
        Ok(price_id)
    }

    /// Newest observation for one trader + material, if any.
    pub fn current_price_for(
        &self,
        trader_id: i64,
        material_id: i64,
    ) -> Result<Option<PriceRow>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT p.id, p.trader_id, p.material_id, p.price, p.currency, p.unit,
                    p.price_min, p.price_max, p.confidence, p.source_type, p.published,
                    p.source_url, p.observed_at, p.published_at, p.valid_from, p.valid_to,
                    p.notes, p.extra_json, p.ingested_at
             FROM current_prices c JOIN prices p ON p.id = c.price_id
             WHERE c.trader_id = ?1 AND c.material_id = ?2",
            params![trader_id, material_id],
            row_to_price,
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Full history for one trader + material, newest first.
    pub fn price_history(
        &self,
        trader_id: i64,
        material_id: i64,
        limit: i64,
    ) -> Result<Vec<PriceRow>, StoreError> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, trader_id, material_id, price, currency, unit,
                    price_min, price_max, confidence, source_type, published,
                    source_url, observed_at, published_at, valid_from, valid_to,
                    notes, extra_json, ingested_at
             FROM prices WHERE trader_id = ?1 AND material_id = ?2
             ORDER BY observed_at DESC, id DESC LIMIT ?3",
        )?;
        let rows = stmt
            .query_map(params![trader_id, material_id, limit], row_to_price)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Which materials a trader accepts, with conditions and validity.
    pub fn set_acceptance(
        &self,
        trader_id: i64,
        material_id: i64,
        accepts: bool,
        conditions: &str,
        valid_from: Option<&str>,
        valid_to: Option<&str>,
        observed_at: &str,
        now: &str,
    ) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO trader_materials
             (trader_id, material_id, accepts, conditions,
              valid_from, valid_to, observed_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(trader_id, material_id) DO UPDATE SET
              accepts = excluded.accepts, conditions = excluded.conditions,
              valid_from = excluded.valid_from, valid_to = excluded.valid_to,
              observed_at = excluded.observed_at, updated_at = excluded.updated_at",
            params![
                trader_id,
                material_id,
                i64::from(accepts),
                conditions,
                valid_from,
                valid_to,
                observed_at,
                now
            ],
        )?;
        Ok(())
    }
}

fn row_to_price(r: &rusqlite::Row<'_>) -> rusqlite::Result<PriceRow> {
    Ok(PriceRow {
        id: r.get(0)?,
        trader_id: r.get(1)?,
        material_id: r.get(2)?,
        price: r.get(3)?,
        currency: r.get(4)?,
        unit: r.get(5)?,
        price_min: r.get(6)?,
        price_max: r.get(7)?,
        confidence: r.get(8)?,
        source_type: r.get(9)?,
        published: r.get::<_, i64>(10)? != 0,
        source_url: r.get(11)?,
        observed_at: r.get(12)?,
        published_at: r.get(13)?,
        valid_from: r.get(14)?,
        valid_to: r.get(15)?,
        notes: r.get(16)?,
        extra_json: r.get(17)?,
        ingested_at: r.get(18)?,
    })
}

#[cfg(test)]
mod tests {
    use super::{NewPrice, PublicDb};
    use crate::public::{NewMaterial, NewTrader};

    fn setup() -> (PublicDb, i64, i64) {
        let dir = std::env::temp_dir().join(format!("schrott-prices-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let db = PublicDb::open(&dir).expect("test db opens");
        let now = "2026-09-27T00:00:00Z";
        let trader = db
            .upsert_trader(&NewTrader {
                slug: "h",
                name: "H",
                trader_type: "schrotthaendler",
                street: "",
                postcode: "10115",
                city: "Berlin",
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
            })
            .expect("trader");
        let material = db
            .upsert_material(&NewMaterial {
                slug: "kupfer-millberry",
                name_de: "Kupfer Millberry",
                category: "nichteisen",
                unit: "EUR/kg",
                description: "",
                updated_at: now,
            })
            .expect("material");
        (db, trader, material)
    }

    fn price<'a>(
        trader: i64,
        material: i64,
        eur: f64,
        observed: &'a str,
        ingested: &'a str,
    ) -> NewPrice<'a> {
        NewPrice {
            trader_id: trader,
            material_id: material,
            price: eur,
            currency: "EUR",
            unit: "EUR/kg",
            price_min: None,
            price_max: None,
            confidence: Some(1.0),
            source_type: "haendler_angabe",
            published: true,
            source_url: "",
            observed_at: observed,
            published_at: None,
            valid_from: None,
            valid_to: None,
            notes: "",
            extra_json: "{}",
            ingested_at: ingested,
        }
    }

    #[test]
    fn current_moves_forward_only() {
        let (db, trader, material) = setup();
        db.record_price(&price(trader, material, 7.10, "2026-09-20T00:00:00Z", "2026-09-20T00:00:00Z"))
            .expect("first");
        db.record_price(&price(trader, material, 7.25, "2026-09-27T00:00:00Z", "2026-09-27T00:00:00Z"))
            .expect("second");
        // Late-arriving backfill must not clobber the newer observation.
        db.record_price(&price(trader, material, 6.90, "2026-09-10T00:00:00Z", "2026-09-28T00:00:00Z"))
            .expect("backfill");
        let cur = db
            .current_price_for(trader, material)
            .expect("current")
            .expect("exists");
        assert_eq!(cur.price, 7.25);
        let hist = db.price_history(trader, material, 10).expect("history");
        assert_eq!(hist.len(), 3);
        assert_eq!(hist[0].price, 7.25);
        assert_eq!(hist[2].price, 6.90);
    }

    #[test]
    fn uncertainty_and_validity_round_trip() {
        let (db, trader, material) = setup();
        db.record_price(&NewPrice {
            price_min: Some(6.80),
            price_max: Some(7.40),
            confidence: Some(0.6),
            source_type: "schaetzung",
            published: false,
            valid_from: Some("2026-09-01T00:00:00Z"),
            valid_to: Some("2026-10-01T00:00:00Z"),
            ..price(trader, material, 7.10, "2026-09-27T00:00:00Z", "2026-09-27T00:00:00Z")
        })
        .expect("uncertain price");
        let cur = db
            .current_price_for(trader, material)
            .expect("current")
            .expect("exists");
        assert_eq!(cur.price_min, Some(6.80));
        assert_eq!(cur.price_max, Some(7.40));
        assert!(!cur.published);
        // Validity-window query: which prices cover 2026-09-15?
        let rows = db.test_query(
            "SELECT price FROM prices
             WHERE (valid_from IS NULL OR valid_from <= '2026-09-15T00:00:00Z')
               AND (valid_to IS NULL OR valid_to > '2026-09-15T00:00:00Z')",
        );
        assert_eq!(rows.len(), 1);
        // The convenience view exposes the same row pre-joined.
        let view = db.test_query("SELECT trader, material, price, published FROM v_current_prices");
        assert_eq!(view.len(), 1);
    }

    #[test]
    fn acceptance_matrix() {
        let (db, trader, material) = setup();
        db.set_acceptance(
            trader,
            material,
            true,
            "nur blank, ab 50 kg",
            None,
            None,
            "2026-09-27T00:00:00Z",
            "2026-09-27T00:00:00Z",
        )
        .expect("acceptance");
        let rows = db.test_query("SELECT accepts FROM trader_materials");
        assert_eq!(rows.len(), 1);
    }
}
