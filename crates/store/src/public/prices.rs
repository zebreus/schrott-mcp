//! Prices: append-only observations plus a materialized "current" table.
//!
//! Every observation answers five questions at once:
//! - *What?* `trader_id` + `material_id` + `variant`, `price` in `currency`
//!   per `unit`. The variant is the trader's own sub-grade ("große Teile",
//!   "80-98%", "min 60%") — `''` means the standard grade. Without it,
//!   two grades at different prices would collapse into one meaningless
//!   "current" price.
//! - *How sure?* `price_min`/`price_max` span the plausible range
//!   (`NULL` = exact); `confidence` is 0..1 (`NULL` = unknown).
//! - *From whom?* `source_type` (`haendler_angabe`, `portal`, `dritte`,
//!   `telefonisch`, `vor_ort`, `schaetzung`, `unbekannt`) plus `published`
//!   (1 = the trader published the price themselves) and `source_url`.
//! - *When true?* `observed_at` (when we saw it), `published_at` (the
//!   page-stated date or, for a detected price change without a page date,
//!   the inferred calendar day; `NULL` = unknown), `valid_from`/`valid_to`
//!   (validity window, `NULL` = open-ended). Inferred dates carry their basis
//!   in `extra_json.published_at_basis`.
//!
//! `current_prices` holds exactly one row per trader + material + variant —
//! the id of the newest observation by `observed_at` — so "what does X pay
//! for Y right now?" is a single indexed join instead of a window-function
//! scan. The `v_current_prices` view pre-joins everything agents usually
//! want.

use rusqlite::{params, OptionalExtension as _};

use super::PublicDb;
use crate::error::StoreError;

pub(super) const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS prices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    trader_id INTEGER NOT NULL REFERENCES traders(id),
    material_id INTEGER NOT NULL REFERENCES materials(id),
    variant TEXT NOT NULL DEFAULT '',
    price REAL NOT NULL,
    currency TEXT NOT NULL DEFAULT 'EUR',
    unit TEXT NOT NULL DEFAULT 'EUR/kg',
    price_kind TEXT NOT NULL DEFAULT 'exact',
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
    variant TEXT NOT NULL DEFAULT '',
    price_id INTEGER NOT NULL REFERENCES prices(id),
    updated_at TEXT NOT NULL,
    PRIMARY KEY (trader_id, material_id, variant)
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
";

/// View + variant-index DDL, applied AFTER migrations (SQLite validates
/// indexes and view bodies at CREATE time, so they must come after the
/// columns exist).
pub(super) const VIEW: &str = "
CREATE INDEX IF NOT EXISTS idx_prices_trader_material_variant
    ON prices(trader_id, material_id, variant);
DROP VIEW IF EXISTS v_current_prices;
CREATE VIEW v_current_prices AS
SELECT t.slug AS trader_slug, t.name AS trader, t.city, t.postcode, t.state,
       m.slug AS material_slug, m.name_de AS material, p.variant, m.category,
       p.price, p.currency, p.unit, p.price_kind, p.price_min, p.price_max, p.confidence,
       p.source_type, p.published, p.source_url,
       p.observed_at, p.published_at, p.valid_from, p.valid_to
FROM current_prices c
JOIN prices p ON p.id = c.price_id
JOIN traders t ON t.id = c.trader_id
JOIN materials m ON m.id = c.material_id;
";

/// Price kinds: `exact` (list price), `upto` (upper bound, "bis zu"),
/// `range` (use price_min/max), `approx` (rounded/guide price).
pub const PRICE_KINDS: &[&str] = &["exact", "upto", "range", "approx"];

/// Bring existing price tables up to the variant schema: add the column,
/// then rebuild `current_prices` keyed by (trader, material, variant).
/// No FTS triggers touch these tables, so rebuild-by-copy is safe.
pub(super) fn migrate(conn: &rusqlite::Connection) -> Result<(), StoreError> {
    let mut prices_cols = std::collections::HashSet::new();
    for col in conn
        .prepare("SELECT name FROM pragma_table_info('prices')")?
        .query_map([], |r| r.get::<_, String>(0))?
    {
        prices_cols.insert(col?);
    }
    if !prices_cols.contains("variant") {
        conn.execute_batch("ALTER TABLE prices ADD COLUMN variant TEXT NOT NULL DEFAULT '';")?;
    }
    if !prices_cols.contains("price_kind") {
        conn.execute_batch(
            "ALTER TABLE prices ADD COLUMN price_kind TEXT NOT NULL DEFAULT 'exact';",
        )?;
        // Backfill: the "bis zu" encoding (max == price, conf 0.5, no min)
        // becomes an explicit kind. Exact rows carry no max.
        conn.execute_batch(
            "UPDATE prices SET price_kind = 'upto'
             WHERE confidence = 0.5 AND price_min IS NULL
               AND price_max IS NOT NULL AND price_max = price;",
        )?;
    }
    // One-time rebuild: old rows carry variant '' by construction, so the
    // copy preserves every pointer. Never rerun once migrated.
    let mut current_variant = false;
    for col in conn
        .prepare("SELECT name FROM pragma_table_info('current_prices')")?
        .query_map([], |r| r.get::<_, String>(0))?
    {
        if col? == "variant" {
            current_variant = true;
        }
    }
    if !current_variant {
        conn.execute_batch(
            "CREATE TABLE current_prices_new (
                trader_id INTEGER NOT NULL,
                material_id INTEGER NOT NULL,
                variant TEXT NOT NULL DEFAULT '',
                price_id INTEGER NOT NULL REFERENCES prices(id),
                updated_at TEXT NOT NULL,
                PRIMARY KEY (trader_id, material_id, variant)
            );
            INSERT INTO current_prices_new
                SELECT trader_id, material_id, '', price_id, updated_at FROM current_prices;
            DROP TABLE current_prices;
            ALTER TABLE current_prices_new RENAME TO current_prices;",
        )?;
    }

    // Migration 3: the WKG handler historically stored public spot references
    // as exact buy prices. Keep those observations in history, correct their
    // classification, and move/remove current pointers so they are no longer
    // presented as what the trader pays.
    let version: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    if version < 3 {
        conn.execute_batch(
            "UPDATE prices
             SET price_kind = 'approx', confidence = 0.5,
                 notes = CASE notes
                     WHEN 'Tageskurs Feingold' THEN 'Spot-Referenz, kein Händler-Ankaufspreis (Feingold)'
                     WHEN 'Silber-Tagespreis' THEN 'Spot-Referenz, kein Händler-Ankaufspreis (Silber)'
                     ELSE notes
                 END
             WHERE trader_id IN (SELECT id FROM traders
                                 WHERE slug = 'hb-bremen-findorff-28215-wirkaufendeingold-de-rohat-erdem')
               AND source_url = 'https://wirkaufendeingold.de'
               AND price_kind = 'exact'
               AND notes IN ('Tageskurs Feingold', 'Silber-Tagespreis');

             UPDATE current_prices AS c
             SET price_id = COALESCE((
                     SELECT p.id FROM prices p
                     WHERE p.trader_id = c.trader_id
                       AND p.material_id = c.material_id
                       AND p.variant = c.variant
                       AND NOT (p.source_url = 'https://wirkaufendeingold.de'
                                AND p.notes IN (
                                    'Spot-Referenz, kein Händler-Ankaufspreis (Feingold)',
                                    'Spot-Referenz, kein Händler-Ankaufspreis (Silber)'))
                     ORDER BY p.observed_at DESC, p.id DESC LIMIT 1
                 ), c.price_id),
                 updated_at = COALESCE((
                     SELECT p.ingested_at FROM prices p
                     WHERE p.id = (
                         SELECT p2.id FROM prices p2
                         WHERE p2.trader_id = c.trader_id
                           AND p2.material_id = c.material_id
                           AND p2.variant = c.variant
                           AND NOT (p2.source_url = 'https://wirkaufendeingold.de'
                                    AND p2.notes IN (
                                        'Spot-Referenz, kein Händler-Ankaufspreis (Feingold)',
                                        'Spot-Referenz, kein Händler-Ankaufspreis (Silber)'))
                         ORDER BY p2.observed_at DESC, p2.id DESC LIMIT 1
                     )
                 ), c.updated_at)
             WHERE c.price_id IN (
                 SELECT p.id FROM prices p
                 JOIN traders t ON t.id = p.trader_id
                 WHERE t.slug = 'hb-bremen-findorff-28215-wirkaufendeingold-de-rohat-erdem'
                   AND p.source_url = 'https://wirkaufendeingold.de'
                   AND p.notes IN (
                       'Spot-Referenz, kein Händler-Ankaufspreis (Feingold)',
                       'Spot-Referenz, kein Händler-Ankaufspreis (Silber)')
             );

             DELETE FROM current_prices
             WHERE price_id IN (
                 SELECT p.id FROM prices p
                 JOIN traders t ON t.id = p.trader_id
                 WHERE t.slug = 'hb-bremen-findorff-28215-wirkaufendeingold-de-rohat-erdem'
                   AND p.source_url = 'https://wirkaufendeingold.de'
                   AND p.notes IN (
                       'Spot-Referenz, kein Händler-Ankaufspreis (Feingold)',
                       'Spot-Referenz, kein Händler-Ankaufspreis (Silber)')
             );
             PRAGMA user_version = 3;",
        )?;
    }
    // Migration 4: A-Z Recycling Münster published a mechanically ascending
    // cross-material ladder (feedback 4538). Preserve those observations as
    // flagged history, but remove them from current buy-price results. The
    // exact values and labels keep a later corrected price list untouched.
    if version < 4 {
        conn.execute_batch(
            "DROP TABLE IF EXISTS temp.migration_4_az_suspect_prices;
             CREATE TEMP TABLE migration_4_az_suspect_prices (id INTEGER PRIMARY KEY);
             INSERT INTO migration_4_az_suspect_prices (id)
             SELECT p.id
             FROM prices p
             JOIN traders t ON t.id = p.trader_id
             WHERE t.slug = 'nw-munster-a-z-recycling-udo-salzsieder'
               AND p.source_url = 'https://xn--schrottplatz-mnster-jbc.de/'
               AND p.price_kind = 'range'
               AND p.confidence = 0.5
               AND (
                   (p.notes = 'Aluminium gemischt' AND p.unit = 'EUR/kg'
                    AND ABS(p.price - 2.22) < 0.000001
                    AND ABS(p.price_min - 1.55) < 0.000001
                    AND ABS(p.price_max - 2.22) < 0.000001)
                OR (p.notes = 'Mischschrott' AND p.unit = 'EUR/t'
                    AND ABS(p.price - 1250.0) < 0.000001
                    AND ABS(p.price_min - 1230.0) < 0.000001
                    AND ABS(p.price_max - 1250.0) < 0.000001)
                OR (p.notes = 'Zink' AND p.unit = 'EUR/kg'
                    AND ABS(p.price - 1.28) < 0.000001
                    AND ABS(p.price_min - 1.26) < 0.000001
                    AND ABS(p.price_max - 1.28) < 0.000001)
                OR (p.notes = 'Blei' AND p.unit = 'EUR/kg'
                    AND ABS(p.price - 1.31) < 0.000001
                    AND ABS(p.price_min - 1.29) < 0.000001
                    AND ABS(p.price_max - 1.31) < 0.000001)
                OR (p.notes = 'Kupfer' AND p.unit = 'EUR/kg'
                    AND ABS(p.price - 1.34) < 0.000001
                    AND ABS(p.price_min - 1.32) < 0.000001
                    AND ABS(p.price_max - 1.34) < 0.000001)
                OR (p.notes = 'Messing' AND p.unit = 'EUR/kg'
                    AND ABS(p.price - 1.37) < 0.000001
                    AND ABS(p.price_min - 1.35) < 0.000001
                    AND ABS(p.price_max - 1.37) < 0.000001)
                OR (p.notes = 'Haushaltskabel 38% (Ohne Stecker)'
                    AND p.variant = '38% ohne Stecker' AND p.unit = 'EUR/kg'
                    AND ABS(p.price - 1.40) < 0.000001
                    AND ABS(p.price_min - 1.38) < 0.000001
                    AND ABS(p.price_max - 1.40) < 0.000001)
                OR (p.notes = 'Edelstahl V2A' AND p.unit = 'EUR/kg'
                    AND ABS(p.price - 1.43) < 0.000001
                    AND ABS(p.price_min - 1.41) < 0.000001
                    AND ABS(p.price_max - 1.43) < 0.000001)
               );

             UPDATE prices
             SET price_kind = 'approx', confidence = 0.0,
                 notes = 'Plausibility hold 2026-10-04: mechanically ascending source list; not a verified buy price'
             WHERE id IN (SELECT id FROM migration_4_az_suspect_prices);

             UPDATE current_prices AS c
             SET price_id = COALESCE((
                     SELECT p.id FROM prices p
                     WHERE p.trader_id = c.trader_id
                       AND p.material_id = c.material_id
                       AND p.variant = c.variant
                       AND p.id NOT IN (SELECT id FROM migration_4_az_suspect_prices)
                     ORDER BY p.observed_at DESC, p.id DESC LIMIT 1
                 ), c.price_id),
                 updated_at = COALESCE((
                     SELECT p.ingested_at FROM prices p
                     WHERE p.id = (
                         SELECT p2.id FROM prices p2
                         WHERE p2.trader_id = c.trader_id
                           AND p2.material_id = c.material_id
                           AND p2.variant = c.variant
                           AND p2.id NOT IN (SELECT id FROM migration_4_az_suspect_prices)
                         ORDER BY p2.observed_at DESC, p2.id DESC LIMIT 1
                     )
                 ), c.updated_at)
             WHERE c.price_id IN (SELECT id FROM migration_4_az_suspect_prices);

             DELETE FROM current_prices
             WHERE price_id IN (SELECT id FROM migration_4_az_suspect_prices);
             DROP TABLE migration_4_az_suspect_prices;
             PRAGMA user_version = 4;",
        )?;
    }
    // Migration 5: the former Schiefer handler read a labeled sale-price
    // ticker as purchase prices. Keep those old observations flagged in
    // history, but retire their current pointers before the corrected handler
    // has a chance to run.
    if version < 5 {
        conn.execute_batch(
            "DROP TABLE IF EXISTS temp.migration_5_schiefer_sale_prices;
             CREATE TEMP TABLE migration_5_schiefer_sale_prices (id INTEGER PRIMARY KEY);
             INSERT INTO migration_5_schiefer_sale_prices (id)
             SELECT p.id
             FROM prices p
             JOIN traders t ON t.id = p.trader_id
             WHERE t.slug = 'hh-st-georg-schiefer-co-edelmetall-scheideanstalt'
               AND p.source_url = 'https://schieferco.de/aktuelle-preise'
               AND p.price_kind = 'exact'
               AND p.notes IN ('Au', 'Ag', 'Pt', 'Pd');

             UPDATE prices
             SET price_kind = 'approx', confidence = 0.0,
                 notes = 'Plausibility hold 2026-10-04: legacy parser recorded Verkaufspreise as Ankaufspreise; not a buy quote'
             WHERE id IN (SELECT id FROM migration_5_schiefer_sale_prices);

             UPDATE current_prices AS c
             SET price_id = COALESCE((
                     SELECT p.id FROM prices p
                     WHERE p.trader_id = c.trader_id
                       AND p.material_id = c.material_id
                       AND p.variant = c.variant
                       AND p.id NOT IN (SELECT id FROM migration_5_schiefer_sale_prices)
                     ORDER BY p.observed_at DESC, p.id DESC LIMIT 1
                 ), c.price_id),
                 updated_at = COALESCE((
                     SELECT p.ingested_at FROM prices p
                     WHERE p.id = (
                         SELECT p2.id FROM prices p2
                         WHERE p2.trader_id = c.trader_id
                           AND p2.material_id = c.material_id
                           AND p2.variant = c.variant
                           AND p2.id NOT IN (SELECT id FROM migration_5_schiefer_sale_prices)
                         ORDER BY p2.observed_at DESC, p2.id DESC LIMIT 1
                     )
                 ), c.updated_at)
             WHERE c.price_id IN (SELECT id FROM migration_5_schiefer_sale_prices);

             DELETE FROM current_prices
             WHERE price_id IN (SELECT id FROM migration_5_schiefer_sale_prices);
             DROP TABLE migration_5_schiefer_sale_prices;
             PRAGMA user_version = 5;",
        )?;
    }
    Ok(())
}

/// One price observation, as stored.
#[derive(Debug, Clone)]
pub struct PriceRow {
    pub id: i64,
    pub trader_id: i64,
    pub material_id: i64,
    pub variant: String,
    pub price: f64,
    pub currency: String,
    pub unit: String,
    pub price_kind: String,
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
    /// Trader's sub-grade (`''` = standard grade).
    pub variant: &'a str,
    pub price: f64,
    pub currency: &'a str,
    pub unit: &'a str,
    /// One of [`PRICE_KINDS`]; anything else is rejected.
    pub price_kind: &'a str,
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
    /// Append one observation and refresh the materialized current price
    /// for its (trader, material, variant).
    /// Out-of-order backfills never clobber newer data: `current_prices`
    /// only moves forward in `observed_at` (ties: higher id wins).
    pub fn record_price(&self, p: &NewPrice<'_>) -> Result<i64, StoreError> {
        if !PRICE_KINDS.contains(&p.price_kind) {
            return Err(StoreError::Rejected("unknown price_kind"));
        }
        let conn = self.lock()?;
        // Dedupe only against the current observation. If a value returns
        // after a change (A -> B -> A), that is a new price-change event and
        // must not rewrite the older A row.
        let dup: Option<i64> = conn
            .query_row(
                "SELECT p.id FROM current_prices c JOIN prices p ON p.id = c.price_id
                 WHERE c.trader_id = ?1 AND c.material_id = ?2 AND c.variant = ?3
                   AND p.price = ?4 AND p.currency = ?5 AND p.unit = ?6 AND p.price_kind = ?7
                   AND p.price_min IS ?8 AND p.price_max IS ?9 AND p.confidence IS ?10",
                params![
                    p.trader_id,
                    p.material_id,
                    p.variant,
                    p.price,
                    p.currency,
                    p.unit,
                    p.price_kind,
                    p.price_min,
                    p.price_max,
                    p.confidence,
                ],
                |r| r.get(0),
            )
            .optional()?;
        let price_id = if let Some(id) = dup {
            conn.execute(
                "UPDATE prices SET observed_at = ?1, ingested_at = ?2, notes = ?3,
                 source_url = ?4, published = ?5,
                 published_at = COALESCE(?6, published_at),
                 extra_json = CASE WHEN ?6 IS NOT NULL THEN ?7 ELSE extra_json END
                 WHERE id = ?8",
                params![
                    p.observed_at,
                    p.ingested_at,
                    p.notes,
                    p.source_url,
                    i64::from(p.published),
                    p.published_at,
                    p.extra_json,
                    id
                ],
            )?;
            id
        } else {
            conn.execute(
                "INSERT INTO prices
             (trader_id, material_id, variant, price, currency, unit, price_kind,
              price_min, price_max, confidence, source_type, published, source_url,
              observed_at, published_at, valid_from, valid_to,
              notes, extra_json, ingested_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
                params![
                    p.trader_id,
                    p.material_id,
                    p.variant,
                    p.price,
                    p.currency,
                    p.unit,
                    p.price_kind,
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
            conn.last_insert_rowid()
        };
        // Move the materialized pointer only forward in observation time.
        let current: Option<(i64, String)> = conn
            .query_row(
                "SELECT c.price_id, p.observed_at FROM current_prices c
                 JOIN prices p ON p.id = c.price_id
                 WHERE c.trader_id = ?1 AND c.material_id = ?2 AND c.variant = ?3",
                params![p.trader_id, p.material_id, p.variant],
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
                "INSERT INTO current_prices (trader_id, material_id, variant, price_id, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(trader_id, material_id, variant) DO UPDATE SET
                  price_id = excluded.price_id, updated_at = excluded.updated_at",
                params![
                    p.trader_id,
                    p.material_id,
                    p.variant,
                    price_id,
                    p.ingested_at
                ],
            )?;
        }
        Ok(price_id)
    }

    /// Newest observation for one trader + material + variant, if any.
    pub fn current_price_for(
        &self,
        trader_id: i64,
        material_id: i64,
        variant: &str,
    ) -> Result<Option<PriceRow>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT p.id, p.trader_id, p.material_id, p.variant, p.price, p.currency, p.unit,
                    p.price_kind, p.price_min, p.price_max, p.confidence, p.source_type, p.published,
                    p.source_url, p.observed_at, p.published_at, p.valid_from, p.valid_to,
                    p.notes, p.extra_json, p.ingested_at
             FROM current_prices c JOIN prices p ON p.id = c.price_id
             WHERE c.trader_id = ?1 AND c.material_id = ?2 AND c.variant = ?3",
            params![trader_id, material_id, variant],
            row_to_price,
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Full history for one trader + material + variant, newest first.
    pub fn price_history(
        &self,
        trader_id: i64,
        material_id: i64,
        variant: &str,
        limit: i64,
    ) -> Result<Vec<PriceRow>, StoreError> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, trader_id, material_id, variant, price, currency, unit,
                    price_kind, price_min, price_max, confidence, source_type, published,
                    source_url, observed_at, published_at, valid_from, valid_to,
                    notes, extra_json, ingested_at
             FROM prices WHERE trader_id = ?1 AND material_id = ?2 AND variant = ?3
             ORDER BY observed_at DESC, id DESC LIMIT ?4",
        )?;
        let rows = stmt
            .query_map(
                params![trader_id, material_id, variant, limit],
                row_to_price,
            )?
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

    /// Current acceptance state for one trader + material, if any.
    pub fn existing_acceptance(
        &self,
        trader_id: i64,
        material_id: i64,
    ) -> Result<Option<(bool, String)>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT accepts, conditions FROM trader_materials
             WHERE trader_id = ?1 AND material_id = ?2",
            params![trader_id, material_id],
            |r| Ok((r.get::<_, i64>(0)? != 0, r.get(1)?)),
        )
        .optional()
        .map_err(StoreError::from)
    }
}

fn row_to_price(r: &rusqlite::Row<'_>) -> rusqlite::Result<PriceRow> {
    Ok(PriceRow {
        id: r.get(0)?,
        trader_id: r.get(1)?,
        material_id: r.get(2)?,
        variant: r.get(3)?,
        price: r.get(4)?,
        currency: r.get(5)?,
        unit: r.get(6)?,
        price_kind: r.get(7)?,
        price_min: r.get(8)?,
        price_max: r.get(9)?,
        confidence: r.get(10)?,
        source_type: r.get(11)?,
        published: r.get::<_, i64>(12)? != 0,
        source_url: r.get(13)?,
        observed_at: r.get(14)?,
        published_at: r.get(15)?,
        valid_from: r.get(16)?,
        valid_to: r.get(17)?,
        notes: r.get(18)?,
        extra_json: r.get(19)?,
        ingested_at: r.get(20)?,
    })
}

#[cfg(test)]
mod tests {
    use super::{NewPrice, PublicDb};
    use crate::public::{NewMaterial, NewTrader};

    fn setup(name: &str) -> (PublicDb, i64, i64) {
        // Unique dir per test: parallel tests share the process id, so a
        // pid-only dir lets them trample each other's rows (flaky).
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "schrott-prices-{}-{}-{}",
            std::process::id(),
            n,
            name
        ));
        let _ = std::fs::remove_dir_all(&dir);
        let db = PublicDb::open(&dir).expect("test db opens");
        let now = "2026-09-27T00:00:00Z";
        let trader = db
            .upsert_trader(&NewTrader {
                slug: "h",
                name: "H",
                trader_type: "schrotthaendler",
                description: "",
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
                website_status: "unbekannt",
                website_checked_at: "",
                opening_hours: "",
                dropoff_json: "{\"allowed\":true}",
                pickup_json: "{\"allowed\":false}",
                min_quantity_kg: None,
                max_quantity_kg: None,
                certifications: Some("[]"),
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
            variant: "",
            price_kind: "exact",
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
        let (db, trader, material) = setup("t");
        db.record_price(&price(
            trader,
            material,
            7.10,
            "2026-09-20T00:00:00Z",
            "2026-09-20T00:00:00Z",
        ))
        .expect("first");
        db.record_price(&price(
            trader,
            material,
            7.25,
            "2026-09-27T00:00:00Z",
            "2026-09-27T00:00:00Z",
        ))
        .expect("second");
        // Late-arriving backfill must not clobber the newer observation.
        db.record_price(&price(
            trader,
            material,
            6.90,
            "2026-09-10T00:00:00Z",
            "2026-09-28T00:00:00Z",
        ))
        .expect("backfill");
        let cur = db
            .current_price_for(trader, material, "")
            .expect("current")
            .expect("exists");
        assert_eq!(cur.price, 7.25);
        let hist = db.price_history(trader, material, "", 10).expect("history");
        assert_eq!(hist.len(), 3);
        assert_eq!(hist[0].price, 7.25);
        assert_eq!(hist[2].price, 6.90);
    }

    #[test]
    fn identical_observations_refresh_instead_of_duplicating() {
        let (db, trader, material) = setup("t");
        let first = db
            .record_price(&price(
                trader,
                material,
                7.10,
                "2026-09-20T00:00:00Z",
                "2026-09-20T00:00:00Z",
            ))
            .expect("first");
        // Same observation a week later: no new row, timestamps move.
        let second = db
            .record_price(&price(
                trader,
                material,
                7.10,
                "2026-09-27T00:00:00Z",
                "2026-09-27T00:00:00Z",
            ))
            .expect("repeat");
        assert_eq!(first, second);
        let hist = db.price_history(trader, material, "", 10).expect("history");
        assert_eq!(hist.len(), 1);
        assert_eq!(hist[0].observed_at, "2026-09-27T00:00:00Z");
        // Changed price still appends.
        db.record_price(&price(
            trader,
            material,
            7.25,
            "2026-09-28T00:00:00Z",
            "2026-09-28T00:00:00Z",
        ))
        .expect("changed");
        let hist = db.price_history(trader, material, "", 10).expect("history");
        assert_eq!(hist.len(), 2);
    }

    #[test]
    fn uncertainty_and_validity_round_trip() {
        let (db, trader, material) = setup("t");
        db.record_price(&NewPrice {
            variant: "",
            price_kind: "exact",
            price_min: Some(6.80),
            price_max: Some(7.40),
            confidence: Some(0.6),
            source_type: "schaetzung",
            published: false,
            valid_from: Some("2026-09-01T00:00:00Z"),
            valid_to: Some("2026-10-01T00:00:00Z"),
            ..price(
                trader,
                material,
                7.10,
                "2026-09-27T00:00:00Z",
                "2026-09-27T00:00:00Z",
            )
        })
        .expect("uncertain price");
        let cur = db
            .current_price_for(trader, material, "")
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
    fn variants_stay_separate_per_grade() {
        let (db, trader, material) = setup("t");
        // Same trader + material, two grades: "große Teile" vs "kleine Teile".
        for (variant, eur) in [("große Teile", 5.20), ("kleine Teile", 4.10)] {
            db.record_price(&NewPrice {
                variant,
                price_kind: "exact",
                ..price(
                    trader,
                    material,
                    eur,
                    "2026-09-27T00:00:00Z",
                    "2026-09-27T00:00:00Z",
                )
            })
            .expect("grade records");
        }
        let big = db
            .current_price_for(trader, material, "große Teile")
            .expect("current")
            .expect("exists");
        let small = db
            .current_price_for(trader, material, "kleine Teile")
            .expect("current")
            .expect("exists");
        assert_eq!(big.price, 5.20);
        assert_eq!(small.price, 4.10);
        // The view carries both grades side by side.
        let view = db.test_query("SELECT variant, price FROM v_current_prices ORDER BY price DESC");
        assert_eq!(view.len(), 2);
    }

    #[test]
    fn migrate_preserves_current_pointers() {
        let dir = std::env::temp_dir().join(format!("schrott-repro-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("dir");
        {
            let conn = rusqlite::Connection::open(dir.join("public.db")).expect("old db");
            conn.execute_batch(
                "CREATE TABLE traders (id INTEGER PRIMARY KEY AUTOINCREMENT, slug TEXT NOT NULL UNIQUE, name TEXT NOT NULL, trader_type TEXT NOT NULL DEFAULT '', description TEXT NOT NULL DEFAULT '', street TEXT NOT NULL DEFAULT '', postcode TEXT NOT NULL DEFAULT '', city TEXT NOT NULL DEFAULT '', state TEXT NOT NULL DEFAULT '', country TEXT NOT NULL DEFAULT 'DE', lat REAL, lon REAL, phone TEXT NOT NULL DEFAULT '', email TEXT NOT NULL DEFAULT '', website TEXT NOT NULL DEFAULT '', website_status TEXT NOT NULL DEFAULT '', website_checked_at TEXT NOT NULL DEFAULT '', opening_hours TEXT NOT NULL DEFAULT '', dropoff_json TEXT NOT NULL DEFAULT '{}', pickup_json TEXT NOT NULL DEFAULT '{}', min_quantity_kg REAL, max_quantity_kg REAL, certifications TEXT NOT NULL DEFAULT '[]', status TEXT NOT NULL DEFAULT '', notes TEXT NOT NULL DEFAULT '', extra_json TEXT NOT NULL DEFAULT '{}', first_seen_at TEXT NOT NULL DEFAULT '', updated_at TEXT NOT NULL DEFAULT '');
                 CREATE TABLE materials (id INTEGER PRIMARY KEY AUTOINCREMENT, slug TEXT NOT NULL UNIQUE, name_de TEXT NOT NULL, category TEXT NOT NULL DEFAULT '', unit TEXT NOT NULL DEFAULT '', description TEXT NOT NULL DEFAULT '', extra_json TEXT NOT NULL DEFAULT '{}', updated_at TEXT NOT NULL DEFAULT '');
                 CREATE TABLE prices (id INTEGER PRIMARY KEY AUTOINCREMENT, trader_id INTEGER NOT NULL, material_id INTEGER NOT NULL, price REAL NOT NULL, currency TEXT NOT NULL DEFAULT 'EUR', unit TEXT NOT NULL DEFAULT 'EUR/kg', price_min REAL, price_max REAL, confidence REAL, source_type TEXT NOT NULL DEFAULT '', published INTEGER NOT NULL DEFAULT 0, source_url TEXT NOT NULL DEFAULT '', observed_at TEXT NOT NULL, published_at TEXT, valid_from TEXT, valid_to TEXT, notes TEXT NOT NULL DEFAULT '', extra_json TEXT NOT NULL DEFAULT '{}', ingested_at TEXT NOT NULL);
                 CREATE TABLE current_prices (trader_id INTEGER NOT NULL, material_id INTEGER NOT NULL, price_id INTEGER NOT NULL, updated_at TEXT NOT NULL, PRIMARY KEY (trader_id, material_id));
                 INSERT INTO traders (slug, name) VALUES ('t','T');
                 INSERT INTO materials (slug, name_de) VALUES ('m','M');
                 INSERT INTO prices (trader_id, material_id, price, observed_at, ingested_at) VALUES (1, 1, 9.8, '2026-09-27T00:00:00Z', '2026-09-27T00:00:00Z');
                 INSERT INTO current_prices VALUES (1, 1, 1, '2026-09-27T00:00:00Z');",
            )
            .expect("old shape");
        }
        let db = PublicDb::open(&dir).expect("open migrates");
        let n: i64 = db
            .conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM current_prices", [], |r| r.get(0))
            .unwrap();
        assert_eq!(n, 1, "pointer survived");
    }

    #[test]
    fn migration_reclassifies_wkg_spot_rows_and_retires_current_pointers() {
        let (db, trader, _) = setup("wkg-spot-fix");
        let gold = db
            .upsert_material(&NewMaterial {
                slug: "gold",
                name_de: "Gold",
                category: "edelmetall",
                unit: "EUR/g",
                description: "",
                updated_at: "2026-10-04T00:00:00Z",
            })
            .expect("gold");
        let silber = db
            .upsert_material(&NewMaterial {
                slug: "silber",
                name_de: "Silber",
                category: "edelmetall",
                unit: "EUR/g",
                description: "",
                updated_at: "2026-10-04T00:00:00Z",
            })
            .expect("silber");

        let prior_gold = db
            .record_price(&NewPrice {
                variant: "999",
                unit: "EUR/g",
                source_url: "https://example.test/buy",
                notes: "previous actual buy quote",
                ..price(
                    trader,
                    gold,
                    91.0,
                    "2026-10-03T12:00:00Z",
                    "2026-10-03T12:00:00Z",
                )
            })
            .expect("prior buy quote");
        for (material, value, note) in [
            (gold, 122.0, "Tageskurs Feingold"),
            (silber, 1.8, "Silber-Tagespreis"),
        ] {
            db.record_price(&NewPrice {
                variant: "999",
                unit: "EUR/g",
                source_url: "https://wirkaufendeingold.de",
                notes: note,
                ..price(
                    trader,
                    material,
                    value,
                    "2026-10-04T12:00:00Z",
                    "2026-10-04T12:00:00Z",
                )
            })
            .expect("spot reference");
        }

        {
            let conn = db.conn.lock().expect("lock");
            conn.execute(
                "UPDATE traders SET slug = ?1 WHERE id = ?2",
                rusqlite::params![
                    "hb-bremen-findorff-28215-wirkaufendeingold-de-rohat-erdem",
                    trader
                ],
            )
            .expect("set WKG slug");
            conn.execute_batch("PRAGMA user_version = 2;")
                .expect("simulate pre-migration database");
            super::migrate(&conn).expect("migration");
        }

        let current_gold = db
            .current_price_for(trader, gold, "999")
            .expect("gold current")
            .expect("prior valid gold buy quote restored");
        assert_eq!(current_gold.id, prior_gold);
        assert!(db
            .current_price_for(trader, silber, "999")
            .expect("silver current")
            .is_none());

        let history = db.price_history(trader, gold, "999", 10).expect("history");
        assert_eq!(history[0].price_kind, "approx");
        assert_eq!(history[0].confidence, Some(0.5));
        assert!(history[0].notes.contains("kein Händler-Ankaufspreis"));
    }

    #[test]
    fn migration_retires_a_z_ladder_from_current_prices() {
        const AZ_URL: &str = "https://xn--schrottplatz-mnster-jbc.de/";
        let (db, trader, copper) = setup("az-ladder-fix");
        let prior_copper = db
            .record_price(&NewPrice {
                source_url: "https://example.test/verified-buy-list",
                notes: "prior verified buy quote",
                ..price(
                    trader,
                    copper,
                    8.50,
                    "2026-10-03T12:00:00Z",
                    "2026-10-03T12:00:00Z",
                )
            })
            .expect("prior copper quote");
        let mut suspects = Vec::new();
        for (label, slug, variant, unit, value, low, high) in [
            (
                "Aluminium gemischt",
                "aluminium-gemischt",
                "",
                "EUR/kg",
                2.22,
                1.55,
                2.22,
            ),
            (
                "Mischschrott",
                "mischschrott",
                "",
                "EUR/t",
                1250.0,
                1230.0,
                1250.0,
            ),
            ("Zink", "zink", "", "EUR/kg", 1.28, 1.26, 1.28),
            ("Blei", "blei", "", "EUR/kg", 1.31, 1.29, 1.31),
            ("Kupfer", "kupfer-millberry", "", "EUR/kg", 1.34, 1.32, 1.34),
            ("Messing", "messing", "", "EUR/kg", 1.37, 1.35, 1.37),
            (
                "Haushaltskabel 38% (Ohne Stecker)",
                "kabel-kupfer",
                "38% ohne Stecker",
                "EUR/kg",
                1.40,
                1.38,
                1.40,
            ),
            (
                "Edelstahl V2A",
                "edelstahl-v2a",
                "",
                "EUR/kg",
                1.43,
                1.41,
                1.43,
            ),
        ] {
            let material = if slug == "kupfer-millberry" {
                copper
            } else {
                db.upsert_material(&NewMaterial {
                    slug,
                    name_de: label,
                    category: if slug == "mischschrott" {
                        "eisen"
                    } else {
                        "nichteisen"
                    },
                    unit,
                    description: "",
                    updated_at: "2026-10-04T00:00:00Z",
                })
                .expect("material")
            };
            let id = db
                .record_price(&NewPrice {
                    variant,
                    unit,
                    price_kind: "range",
                    price: value,
                    price_min: Some(low),
                    price_max: Some(high),
                    confidence: Some(0.5),
                    source_url: AZ_URL,
                    notes: label,
                    ..price(
                        trader,
                        material,
                        value,
                        "2026-10-04T08:51:38.015924722+00:00",
                        "2026-10-04T08:51:38.015924722+00:00",
                    )
                })
                .expect("suspect source price");
            suspects.push((material, variant.to_owned(), id));
        }

        {
            let conn = db.conn.lock().expect("lock");
            conn.execute(
                "UPDATE traders SET slug = ?1 WHERE id = ?2",
                rusqlite::params!["nw-munster-a-z-recycling-udo-salzsieder", trader],
            )
            .expect("set A-Z slug");
            conn.execute_batch("PRAGMA user_version = 2;")
                .expect("simulate pre-migration database");
            super::migrate(&conn).expect("migration");
        }

        let current_copper = db
            .current_price_for(trader, copper, "")
            .expect("current copper")
            .expect("prior verified price restored");
        assert_eq!(current_copper.id, prior_copper);
        for (material, variant, suspect_id) in suspects {
            let current = db
                .current_price_for(trader, material, &variant)
                .expect("current price query");
            if material == copper && variant.is_empty() {
                assert_eq!(current.expect("prior copper quote").id, prior_copper);
            } else {
                assert!(current.is_none(), "suspect current price {variant:?}");
            }

            let history = db
                .price_history(trader, material, &variant, 10)
                .expect("price history");
            let retired = history
                .iter()
                .find(|row| row.id == suspect_id)
                .expect("suspect history retained");
            assert_eq!(retired.price_kind, "approx");
            assert_eq!(retired.confidence, Some(0.0));
            assert!(retired.notes.contains("not a verified buy price"));
        }

        let version: i64 = db
            .conn
            .lock()
            .expect("lock")
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("version");
        assert_eq!(version, 5);
    }

    #[test]
    fn migration_retires_schiefer_sale_quotes_from_current_prices() {
        const SCHIEFER_URL: &str = "https://schieferco.de/aktuelle-preise";
        let (db, trader, _) = setup("schiefer-sales-fix");
        let gold = db
            .upsert_material(&NewMaterial {
                slug: "gold",
                name_de: "Gold",
                category: "edelmetall",
                unit: "EUR/g",
                description: "",
                updated_at: "2026-10-04T00:00:00Z",
            })
            .expect("gold");
        let silber = db
            .upsert_material(&NewMaterial {
                slug: "silber",
                name_de: "Silber",
                category: "edelmetall",
                unit: "EUR/g",
                description: "",
                updated_at: "2026-10-04T00:00:00Z",
            })
            .expect("silber");

        let prior_gold = db
            .record_price(&NewPrice {
                unit: "EUR/g",
                source_url: "https://example.test/verified-gold-buy",
                notes: "prior verified gold buy quote",
                ..price(
                    trader,
                    gold,
                    114.0,
                    "2026-10-03T12:00:00Z",
                    "2026-10-03T12:00:00Z",
                )
            })
            .expect("prior gold quote");
        let old_gold_sale = db
            .record_price(&NewPrice {
                unit: "EUR/g",
                source_url: SCHIEFER_URL,
                notes: "Au",
                ..price(
                    trader,
                    gold,
                    126.74,
                    "2026-10-04T08:06:27.577228082+00:00",
                    "2026-10-04T08:06:27.577228082+00:00",
                )
            })
            .expect("old gold sale quote");
        let old_silver_sale = db
            .record_price(&NewPrice {
                unit: "EUR/g",
                source_url: SCHIEFER_URL,
                notes: "Ag",
                ..price(
                    trader,
                    silber,
                    1.911,
                    "2026-10-04T08:06:27.577228082+00:00",
                    "2026-10-04T08:06:27.577228082+00:00",
                )
            })
            .expect("old silver sale quote");

        {
            let conn = db.conn.lock().expect("lock");
            conn.execute(
                "UPDATE traders SET slug = ?1 WHERE id = ?2",
                rusqlite::params!["hh-st-georg-schiefer-co-edelmetall-scheideanstalt", trader],
            )
            .expect("set Schiefer slug");
            conn.execute_batch("PRAGMA user_version = 4;")
                .expect("simulate pre-migration database");
            super::migrate(&conn).expect("migration");
        }

        assert_eq!(
            db.current_price_for(trader, gold, "")
                .expect("current gold")
                .expect("prior verified gold buy quote")
                .id,
            prior_gold
        );
        assert!(db
            .current_price_for(trader, silber, "")
            .expect("current silver")
            .is_none());

        for (material, suspect_id) in [(gold, old_gold_sale), (silber, old_silver_sale)] {
            let history = db.price_history(trader, material, "", 10).expect("history");
            let retired = history
                .iter()
                .find(|row| row.id == suspect_id)
                .expect("old sale quote retained in history");
            assert_eq!(retired.price_kind, "approx");
            assert_eq!(retired.confidence, Some(0.0));
            assert!(retired.notes.contains("Verkaufspreise as Ankaufspreise"));
        }

        let version: i64 = db
            .conn
            .lock()
            .expect("lock")
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("version");
        assert_eq!(version, 5);
    }

    #[test]
    fn acceptance_matrix() {
        let (db, trader, material) = setup("t");
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
