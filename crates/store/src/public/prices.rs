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
CREATE TABLE IF NOT EXISTS source_article_snapshots (
    id INTEGER PRIMARY KEY, trader_id INTEGER NOT NULL REFERENCES traders(id),
    source_url TEXT NOT NULL, owner TEXT NOT NULL, observed_at TEXT NOT NULL,
    manifest_json TEXT NOT NULL,
    UNIQUE(trader_id,source_url,owner,observed_at)
);
CREATE TABLE IF NOT EXISTS article_acceptance_owners (
    trader_id INTEGER NOT NULL REFERENCES traders(id), material_id INTEGER NOT NULL REFERENCES materials(id),
    source_url TEXT NOT NULL, owner TEXT NOT NULL,
    conditions TEXT NOT NULL, observed_at TEXT NOT NULL,
    PRIMARY KEY(trader_id,material_id)
);
";

/// View + variant-index DDL, applied AFTER migrations (SQLite validates
/// indexes and view bodies at CREATE time, so they must come after the
/// columns exist). Wahl & Co.'s operator-labeled daily-dependent prices are
/// visible only while their own source timestamp is no more than 24 hours old.
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
JOIN materials m ON m.id = c.material_id
WHERE t.slug <> 'th-zella-mehlis-wahl-co-inh-thomas-madeiski'
   OR p.source_url <> 'https://wahlundco.de/'
   OR (
       p.published_at IS NOT NULL
       AND julianday(p.published_at) >= julianday('now', '-24 hours')
       AND julianday(p.published_at) <= julianday('now')
   );
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
    // Migration 6 (#4782): retire only source-label/key contradictions proven
    // by the six corrected handlers (21be8fa). Age and map_v alone prove
    // nothing (#4783). Preserve the source label, amount, timestamps and
    // provenance in history; this was a mapping error, not evidence that the
    // trader's quote itself was wrong. Do not manufacture a corrected quote
    // or restore a potentially overwritten older grade: fresh ingestion owns
    // the corrected keys. A legitimate current pointer is never moved.
    if version < 6 {
        // Annotation and pointer retirement must succeed together, otherwise
        // the exact-label matcher could miss an annotated row on retry.
        let tx = conn.unchecked_transaction()?;
        tx.execute_batch(
            "DROP TABLE IF EXISTS temp.migration_6_mapping_errors;
             CREATE TEMP TABLE migration_6_mapping_errors (id INTEGER PRIMARY KEY);
             INSERT INTO migration_6_mapping_errors (id)
             SELECT p.id FROM prices p
             JOIN traders t ON t.id = p.trader_id
             JOIN materials m ON m.id = p.material_id
             WHERE p.price_kind = 'exact' AND (
                 (t.slug = 'hh-lokstedt-hansa-goldankauf'
                  AND p.source_url = 'https://hansa-goldankauf.de/'
                  AND m.slug = 'gold' AND p.variant = ''
                  AND p.notes = '875er Gold')
              OR (t.slug = 'th-erfurt-goldankauf-boerse-filiale-erfurt'
                  AND p.source_url = 'https://www.goldankauf-boerse.de/ankaufsrechner/'
                  AND m.slug = 'silber' AND p.variant = ''
                  AND p.notes = '625er Silber')
              OR (t.slug = 'sn-frankenberg-volker-lungwitz-schrotthandel'
                  AND p.source_url = 'https://vlschrott.de/einkaufspreise/'
                  AND m.slug = 'aluminium-gemischt' AND p.variant = 'Blech/Guß o. Fe'
                  AND p.notes = 'Zinkblech')
              OR (t.slug = 'sn-rochlitz-db-recycling-denis-blum'
                  AND p.source_url = 'https://www.dein-schrottplatz.de/pages/preise.php'
                  AND m.slug = 'aluminium-gemischt' AND p.variant = 'Blech/Guß 2% Fe'
                  AND p.notes = 'Zinkblech')
              OR (t.slug = 'ni-wangerland-hooksiel-26434-manuel-triebsch-autoverwertung-altmetall'
                  AND p.source_url = 'https://www.schrott-triebsch.de/'
                  AND m.slug = 'kupfer-gemischt' AND p.variant = ''
                  AND p.notes = 'Kupferkabel o.Stecker')
              OR (t.slug = 'sn-leipzig-albus-leipzig'
                  AND p.source_url = 'https://www.albus-leipzig.de/preise'
                  AND p.variant IN ('mit Kundenkarte', 'ohne Kundenkarte')
                  AND ((m.slug = 'aluminium-gemischt' AND p.notes = 'Alu-Profile')
                    OR (m.slug = 'aluminium-guss' AND p.notes = 'Alufelgen sauber')
                    OR (m.slug = 'eisenschrott-gussbruch' AND p.notes = 'Bremsscheiben')))
             );

             UPDATE prices
             SET price_kind = 'approx', confidence = 0.0,
                 notes = notes || '; Fehlmapping #4782 (2026-10-08): Quelllabel unter falschem Material/Variante gespeichert; historische Fehlzuordnung, kein aktueller Preis dieser Sorte'
             WHERE id IN (SELECT id FROM migration_6_mapping_errors);

             DELETE FROM current_prices
             WHERE price_id IN (SELECT id FROM migration_6_mapping_errors);
             DROP TABLE migration_6_mapping_errors;
             PRAGMA user_version = 6;",
        )?;
        tx.commit()?;
    }
    // Migration 7: Goldtrans published zero quotes for these 14 known keys.
    // Retire only their current pointers; keep every historical field intact
    // and never present an older positive observation as a fresh buy quote.
    if version < 7 {
        let tx = conn.unchecked_transaction()?;
        tx.execute_batch(
            "DELETE FROM current_prices AS c
             WHERE EXISTS (
                 SELECT 1 FROM prices p
                 JOIN traders t ON t.id = p.trader_id
                 JOIN materials m ON m.id = p.material_id
                 WHERE p.id = c.price_id
                   AND p.trader_id = c.trader_id
                   AND p.material_id = c.material_id
                   AND p.variant = c.variant
                   AND t.slug = 'hh-wandsbek-goldtrans-edelmetallhandel'
                   AND p.source_url = 'https://www.goldtrans.de/goldankauf-preise-aktueller-goldpreis-ankauf-in-hamburg.html'
                   AND p.currency = 'EUR' AND p.unit = 'EUR/g'
                   AND p.price = 0
                   AND ((m.slug = 'gold' AND p.variant IN (
                       '999','986','980','965','916','900','875',
                       '833','750','585','416','375','333'))
                     OR (m.slug = 'zahngold' AND p.variant = ''))
             );
             PRAGMA user_version = 7;",
        )?;
        tx.commit()?;
    }
    // Migration 8: Heraeus currently states on its official Hanau branch page
    // that precious-metal buying has been stopped for technical reasons. The
    // old Ankauf page now returns 404. Preserve the historical quotes, but do
    // not leave them or their source-derived acceptances presented as current.
    if version < 8 {
        let tx = conn.unchecked_transaction()?;
        tx.execute_batch(
            "DELETE FROM current_prices AS c
             WHERE EXISTS (
                 SELECT 1 FROM prices p
                 JOIN traders t ON t.id = p.trader_id
                 WHERE p.id = c.price_id
                   AND p.trader_id = c.trader_id
                   AND p.material_id = c.material_id
                   AND p.variant = c.variant
                   AND t.slug = 'he-hanau-heraeus-goldstore-hanau-argor-heraeus-de'
                   AND p.source_url = 'https://store.heraeus-gold.com/ankauf/'
                   AND p.source_type = 'haendler_angabe'
                   AND p.published = 1
             );

             UPDATE trader_materials
             SET accepts = 0,
                 conditions = 'Edelmetallankauf laut Betreiber aus technischen Gründen eingestellt (Quelle geprüft am 23.10.2026); frühere Bedingungen: ' || conditions,
                 observed_at = strftime('%Y-%m-%dT%H:%M:%f+00:00', 'now'),
                 updated_at = strftime('%Y-%m-%dT%H:%M:%f+00:00', 'now')
             WHERE trader_id IN (
                 SELECT id FROM traders
                 WHERE slug = 'he-hanau-heraeus-goldstore-hanau-argor-heraeus-de'
             )
               AND accepts = 1
               AND conditions LIKE 'Unverbindlicher Ankauf-Richtwert;%';
             PRAGMA user_version = 8;",
        )?;
        tx.commit()?;
    }
    // Migration 9: the current Wahl & Co. price pointers all refer to one
    // pre-update scrape, before the operator's dated, daily-dependent guide
    // prices. Keep those observations as history, but do not present them as
    // current until the source publishes a fresh timestamp and a new snapshot
    // is recorded. The exact timestamp identifies candidates; all other source
    // and pointer fields plus the 30/2 split validate the reviewed population.
    if version < 9 {
        let tx = conn.unchecked_transaction()?;
        tx.execute_batch(
            "DROP TABLE IF EXISTS temp.migration_9_wahl_stale_prices;
             CREATE TEMP TABLE migration_9_wahl_stale_prices (
                 trader_id INTEGER NOT NULL,
                 material_id INTEGER NOT NULL,
                 variant TEXT NOT NULL,
                 price_id INTEGER NOT NULL,
                 PRIMARY KEY (trader_id, material_id, variant)
             );
             INSERT INTO migration_9_wahl_stale_prices (trader_id, material_id, variant, price_id)
             SELECT c.trader_id, c.material_id, c.variant, c.price_id
             FROM current_prices c
             JOIN prices p ON p.id = c.price_id
             JOIN traders t ON t.id = p.trader_id
             WHERE t.slug = 'th-zella-mehlis-wahl-co-inh-thomas-madeiski'
               AND p.observed_at = '2026-10-21T03:30:55.060415905+00:00';",
        )?;
        let counts: (i64, i64, i64, i64) = tx.query_row(
            "SELECT COUNT(*),
                    COALESCE(SUM(p.price_kind = 'exact'), 0),
                    COALESCE(SUM(p.price_kind = 'upto'), 0),
                    COALESCE(SUM(
                        c.trader_id = p.trader_id
                        AND c.material_id = p.material_id
                        AND c.variant = p.variant
                        AND p.source_url = 'https://wahlundco.de/'
                        AND p.published_at IS NULL
                        AND p.extra_json = '{}'
                        AND p.source_type = 'haendler_angabe'
                        AND p.published = 1
                        AND p.currency = 'EUR'
                        AND p.unit = 'EUR/kg'
                        AND p.price_kind IN ('exact', 'upto')
                    ), 0)
             FROM migration_9_wahl_stale_prices s
             JOIN current_prices c
               ON c.trader_id = s.trader_id
              AND c.material_id = s.material_id
              AND c.variant = s.variant
              AND c.price_id = s.price_id
             JOIN prices p ON p.id = s.price_id",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )?;
        if counts == (32, 30, 2, 32) {
            tx.execute(
                "DELETE FROM current_prices AS c
                 WHERE EXISTS (
                     SELECT 1 FROM migration_9_wahl_stale_prices s
                     WHERE s.trader_id = c.trader_id
                       AND s.material_id = c.material_id
                       AND s.variant = c.variant
                       AND s.price_id = c.price_id
                 )",
                [],
            )?;
        } else if counts != (0, 0, 0, 0) {
            return Err(StoreError::Rejected(
                "unexpected Wahl current-price population for migration 9",
            ));
        }
        tx.execute_batch(
            "DROP TABLE migration_9_wahl_stale_prices;
             PRAGMA user_version = 9;",
        )?;
        tx.commit()?;
    }
    Ok(())
}

/// One price observation, as stored.
#[derive(Debug, Clone, PartialEq)]
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
        if serde_json::from_str::<serde_json::Value>(p.extra_json)
            .ok()
            .is_some_and(|v| v["snapshot_owner"] == "mkm-article-v1")
        {
            return Err(StoreError::Rejected(
                "MKM article owner requires typed manifest transaction",
            ));
        }
        // Read ownership, append/dedupe and move Current in one SQLite
        // transaction, including across independently opened connections.
        let mut conn = self.lock()?;
        let tx = conn.transaction()?;
        let id = Self::record_price_on(&tx, p)?;
        tx.commit()?;
        Ok(id)
    }

    /// Atomically publish a validated, complete source snapshot and retire only
    /// explicitly proven legacy observations or observations owned by this
    /// snapshot producer. History is never deleted/rewritten by retirement.
    /// Foreign-source/producer key collisions and late snapshots fail without changes.
    pub fn record_price_snapshot(
        &self,
        prices: &[NewPrice<'_>],
        legacy_rows: &[PriceRow],
        owner: &str,
    ) -> Result<usize, StoreError> {
        if owner == "mkm-article-v1" {
            return Err(StoreError::Rejected(
                "article owner requires typed manifest transaction",
            ));
        }
        self.record_snapshot_transaction(prices, legacy_rows, owner, None)
    }

    /// MKM-only extension of the existing atomic price-snapshot contract.
    /// The caller must supply a validated complete typed article manifest.
    /// Semantic price metadata is immutable; request evidence and material
    /// acceptances commit with prices/retirement or all roll back together.
    /// Legacy rows are passed as read snapshots and revalidated under the
    /// transaction lock before their current pointers can be retired.
    pub fn record_article_price_snapshot(
        &self,
        prices: &[NewPrice<'_>],
        legacy_rows: &[PriceRow],
        manifest_json: &str,
        acceptances: &[(i64, String)],
    ) -> Result<usize, StoreError> {
        self.record_snapshot_transaction(
            prices,
            legacy_rows,
            "mkm-article-v1",
            Some((manifest_json, acceptances)),
        )
    }

    fn record_snapshot_transaction(
        &self,
        prices: &[NewPrice<'_>],
        legacy_rows: &[PriceRow],
        owner: &str,
        article_commit: Option<(&str, &[(i64, String)])>,
    ) -> Result<usize, StoreError> {
        // Current reviewed MKM contract: the original 60 identities plus
        // MKM-MS-01 (blank new MS58). Historical ledgers remain readable.
        const MKM_ARTICLE_COUNT: usize = 61;
        let manifest_value = article_commit
            .map(|(manifest, _)| {
                serde_json::from_str::<serde_json::Value>(manifest)
                    .map_err(|_| StoreError::Rejected("invalid article manifest"))
            })
            .transpose()?;
        struct Scope<'a> {
            trader_id: i64,
            source_url: &'a str,
            observed_at: &'a str,
        }
        let first = if let Some(p) = prices.first() {
            Scope {
                trader_id: p.trader_id,
                source_url: p.source_url,
                observed_at: p.observed_at,
            }
        } else if let Some(manifest) = &manifest_value {
            Scope {
                trader_id: manifest["trader_id"]
                    .as_i64()
                    .ok_or(StoreError::Rejected("missing article trader"))?,
                source_url: manifest["source_url"]
                    .as_str()
                    .ok_or(StoreError::Rejected("missing article source"))?,
                observed_at: manifest["observed_at"]
                    .as_str()
                    .ok_or(StoreError::Rejected("missing article time"))?,
            }
        } else {
            return Err(StoreError::Rejected("empty price snapshot"));
        };
        if owner.is_empty() || first.source_url.is_empty() {
            return Err(StoreError::Rejected("snapshot scope missing"));
        }
        let mut legacy_ids = std::collections::HashSet::new();
        for row in legacy_rows {
            if !legacy_ids.insert(row.id)
                || row.trader_id != first.trader_id
                || row.source_url != first.source_url
            {
                return Err(StoreError::Rejected("invalid legacy retirement evidence"));
            }
        }
        let mut keys = std::collections::HashSet::new();
        for p in prices {
            let marker = serde_json::from_str::<serde_json::Value>(p.extra_json).ok();
            if p.trader_id != first.trader_id
                || p.source_url != first.source_url
                || p.observed_at != first.observed_at
                || !p.published
                || p.source_type != "haendler_angabe"
                || !p.price.is_finite()
                || p.price <= 0.0
                || marker
                    .as_ref()
                    .and_then(|v| v.get("snapshot_owner"))
                    .and_then(|v| v.as_str())
                    != Some(owner)
                || !keys.insert((p.material_id, p.variant))
            {
                return Err(StoreError::Rejected("invalid or duplicate price snapshot"));
            }
        }
        let mut conn = self.lock()?;
        let tx = conn.transaction()?;
        if let Some((manifest, acceptances)) = article_commit {
            let json = manifest_value.as_ref().expect("validated manifest");
            let slug: String = tx.query_row(
                "SELECT slug FROM traders WHERE id=?1",
                [first.trader_id],
                |r| r.get(0),
            )?;
            if slug != "rp-ludwigshafen-mkm-metals"
                || first.source_url != "https://mkm-metals.de/metallpreise"
                || json["owner"] != owner
                || json["source_url"] != first.source_url
                || json["trader_id"] != first.trader_id
                || json["observed_at"] != first.observed_at
                || first.observed_at.len() != 35
                || !first.observed_at.ends_with("+00:00")
                || json["articles"]
                    .as_array()
                    .is_none_or(|a| a.len() != MKM_ARTICLE_COUNT)
            {
                return Err(StoreError::Rejected(
                    "article manifest scope/completeness mismatch",
                ));
            }
            let mut article_ids = std::collections::HashSet::new();
            for a in json["articles"].as_array().expect("validated array") {
                let id = a["article"]
                    .as_str()
                    .filter(|s| !s.is_empty())
                    .ok_or(StoreError::Rejected("missing manifest article"))?;
                if !article_ids.insert(id) {
                    return Err(StoreError::Rejected("duplicate manifest article"));
                }
            }
            let previous: Option<(String,String)> = tx.query_row(
                "SELECT observed_at,manifest_json FROM source_article_snapshots WHERE trader_id=?1 AND source_url=?2 AND owner=?3 ORDER BY observed_at DESC LIMIT 1",
                params![first.trader_id,first.source_url,owner], |r| Ok((r.get(0)?,r.get(1)?))).optional()?;
            if previous.as_ref().is_some_and(|(at, old)| {
                at.as_str() > first.observed_at || (at == first.observed_at && old != manifest)
            }) {
                return Err(StoreError::Rejected("late or conflicting article snapshot"));
            }
            let mut mids = std::collections::HashSet::new();
            for (mid, conditions) in acceptances {
                if !mids.insert(*mid) || conditions.is_empty() {
                    return Err(StoreError::Rejected("invalid article acceptance"));
                }
                let ownership: Option<(String,String,String,String)> = tx.query_row("SELECT source_url,owner,conditions,observed_at FROM article_acceptance_owners WHERE trader_id=?1 AND material_id=?2",
                    params![first.trader_id,mid], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?))).optional()?;
                if ownership.as_ref().is_some_and(|(source, producer, _, _)| {
                    source != first.source_url || producer != owner
                }) {
                    return Err(StoreError::Rejected("foreign acceptance producer"));
                }
                let old: Option<(bool,String,String,Option<String>,Option<String>)> = tx.query_row("SELECT accepts,conditions,observed_at,valid_from,valid_to FROM trader_materials WHERE trader_id=?1 AND material_id=?2",
                    params![first.trader_id,mid], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?))).optional()?;
                if old.as_ref().is_some_and(|(_, _, at, from, to)| {
                    at.as_str() > first.observed_at || from.is_some() || to.is_some()
                }) {
                    return Err(StoreError::Rejected("newer or foreign acceptance validity"));
                }
                if let Some((_, _, owned_conditions, owned_at)) = &ownership {
                    if old.as_ref().is_none_or(|(accepts, conditions, at, _, _)| {
                        !accepts || conditions != owned_conditions || at != owned_at
                    }) {
                        return Err(StoreError::Rejected(
                            "acceptance changed outside owning article producer",
                        ));
                    }
                }
                if ownership.is_none()
                    && old.as_ref().is_some_and(|(accepts, old, _, _, _)| {
                        !accepts || (!old.is_empty() && old != conditions)
                    })
                {
                    return Err(StoreError::Rejected(
                        "unowned acceptance evidence conflicts",
                    ));
                }
            }
        }
        // Refuse foreign sources/producers; same-source unmarked rows may be adopted.
        for p in prices {
            let conflict: bool = tx.query_row(
                "SELECT EXISTS(SELECT 1 FROM current_prices c JOIN prices p ON p.id=c.price_id
                 WHERE c.trader_id=?1 AND c.material_id=?2 AND c.variant=?3 AND
                  (p.source_url<>?4 OR p.source_type<>'haendler_angabe' OR p.published<>1 OR p.observed_at>?5
                   OR COALESCE(json_extract(p.extra_json,'$.snapshot_owner'),'') NOT IN ('',?6)))",
                params![p.trader_id, p.material_id, p.variant, p.source_url, p.observed_at, owner], |r| r.get(0))?;
            if conflict {
                return Err(StoreError::Rejected("foreign or newer current price"));
            }
        }
        let candidates: Vec<PriceRow> = {
            let mut stmt = tx.prepare(
                "SELECT p.id,p.trader_id,p.material_id,p.variant,p.price,p.currency,p.unit,p.price_kind,
                  p.price_min,p.price_max,p.confidence,p.source_type,p.published,p.source_url,
                  p.observed_at,p.published_at,p.valid_from,p.valid_to,p.notes,p.extra_json,p.ingested_at
                 FROM current_prices c JOIN prices p ON p.id=c.price_id
                 WHERE c.trader_id=?1 AND p.source_url=?2 AND p.source_type='haendler_angabe' AND p.published=1")?;
            let rows = stmt
                .query_map(params![first.trader_id, first.source_url], row_to_price)?
                .collect::<Result<Vec<_>, _>>()?;
            rows
        };
        let mut retire = Vec::new();
        let mut matched_legacy_ids = std::collections::HashSet::new();
        for candidate in candidates {
            let id = candidate.id;
            let Ok(extra) = serde_json::from_str::<serde_json::Value>(&candidate.extra_json) else {
                continue;
            };
            let marker = extra.get("snapshot_owner");
            if marker.is_some_and(|v| {
                !v.is_null() && v.as_str() != Some("") && v.as_str() != Some(owner)
            }) {
                continue; // Explicit legacy IDs cannot retire another producer either.
            }
            let managed = marker.and_then(|v| v.as_str()) == Some(owner);
            if managed || legacy_ids.contains(&id) {
                if legacy_ids.contains(&id) {
                    let Some(expected) = legacy_rows.iter().find(|row| row.id == id) else {
                        return Err(StoreError::Rejected("missing legacy retirement evidence"));
                    };
                    if expected != &candidate {
                        return Err(StoreError::Rejected(
                            "legacy candidate changed during snapshot",
                        ));
                    }
                    matched_legacy_ids.insert(id);
                }
                if candidate.observed_at.as_str() > first.observed_at {
                    return Err(StoreError::Rejected("late price snapshot"));
                }
                if !keys.contains(&(candidate.material_id, candidate.variant.as_str())) {
                    retire.push(id);
                }
            }
        }
        if matched_legacy_ids.len() != legacy_rows.len() {
            return Err(StoreError::Rejected(
                "legacy candidate is no longer current",
            ));
        }
        for p in prices {
            let id = Self::record_price_on(&tx, p)?;
            // An unchanged observation can predate the producer marker.
            // Refresh ownership metadata without losing its retained date basis.
            if owner != "mkm-article-v1" {
                tx.execute(
                    "UPDATE prices SET extra_json=json_patch(extra_json,?1) WHERE id=?2",
                    params![p.extra_json, id],
                )?;
            }
        }
        for id in &retire {
            tx.execute(
                "DELETE FROM current_prices WHERE trader_id=?1 AND price_id=?2",
                params![first.trader_id, id],
            )?;
        }
        if let Some((manifest, acceptances)) = article_commit {
            for (mid, conditions) in acceptances {
                tx.execute("INSERT INTO trader_materials(trader_id,material_id,accepts,conditions,observed_at,updated_at) VALUES(?1,?2,1,?3,?4,?4)
                    ON CONFLICT(trader_id,material_id) DO UPDATE SET accepts=1,conditions=excluded.conditions,observed_at=excluded.observed_at,updated_at=excluded.updated_at",
                    params![first.trader_id,mid,conditions,first.observed_at])?;
                tx.execute("INSERT INTO article_acceptance_owners(trader_id,material_id,source_url,owner,conditions,observed_at) VALUES(?1,?2,?3,?4,?5,?6)
                    ON CONFLICT(trader_id,material_id) DO UPDATE SET conditions=excluded.conditions,observed_at=excluded.observed_at", params![first.trader_id,mid,first.source_url,owner,conditions,first.observed_at])?;
            }
            tx.execute("INSERT INTO source_article_snapshots(trader_id,source_url,owner,observed_at,manifest_json) VALUES(?1,?2,?3,?4,?5)
                ON CONFLICT(trader_id,source_url,owner,observed_at) DO NOTHING", params![first.trader_id,first.source_url,owner,first.observed_at,manifest])?;
        }
        tx.commit()?;
        Ok(retire.len())
    }

    /// Current observations from one source, for evidence-based legacy matching.
    pub fn current_prices_from(
        &self,
        trader_id: i64,
        source_url: &str,
    ) -> Result<Vec<PriceRow>, StoreError> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare(
            "SELECT p.id,p.trader_id,p.material_id,p.variant,p.price,p.currency,p.unit,p.price_kind,
             p.price_min,p.price_max,p.confidence,p.source_type,p.published,p.source_url,
             p.observed_at,p.published_at,p.valid_from,p.valid_to,p.notes,p.extra_json,p.ingested_at
             FROM current_prices c JOIN prices p ON p.id=c.price_id WHERE c.trader_id=?1 AND p.source_url=?2")?;
        let rows = stmt
            .query_map(params![trader_id, source_url], row_to_price)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub(super) fn record_price_on(
        conn: &rusqlite::Connection,
        p: &NewPrice<'_>,
    ) -> Result<i64, StoreError> {
        let hofmann_time = super::hofmann_tin::guard_write_on(conn, p)?;
        if !PRICE_KINDS.contains(&p.price_kind) {
            return Err(StoreError::Rejected("unknown price_kind"));
        }
        // Dedupe only against the current observation. If a value returns
        // after a change (A -> B -> A), that is a new price-change event and
        // must not rewrite the older A row.
        let immutable = serde_json::from_str::<serde_json::Value>(p.extra_json)
            .ok()
            .is_some_and(|v| v["snapshot_owner"] == "mkm-article-v1");
        // Ownership belongs to the existing observation too, not just the
        // incoming marker. Every public price writer funnels through here.
        // An unmarked/foreign writer must not mutate or replace owned MKM
        // evidence, even when its numeric contract happens to be identical.
        let existing_owner: Option<(String,String,bool)> = conn.query_row(
            "SELECT p.source_url,p.source_type,p.published FROM current_prices c JOIN prices p ON p.id=c.price_id
             WHERE c.trader_id=?1 AND c.material_id=?2 AND c.variant=?3
               AND json_extract(p.extra_json,'$.snapshot_owner')='mkm-article-v1'",
            params![p.trader_id,p.material_id,p.variant],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).optional()?;
        if existing_owner
            .as_ref()
            .is_some_and(|(source, kind, published)| {
                !immutable
                    || source != p.source_url
                    || kind != p.source_type
                    || *published != p.published
            })
        {
            return Err(StoreError::Rejected(
                "existing MKM observation belongs to another write contract",
            ));
        }
        let dup: Option<i64> = conn
            .query_row(
                "SELECT p.id FROM current_prices c JOIN prices p ON p.id = c.price_id
                 WHERE c.trader_id = ?1 AND c.material_id = ?2 AND c.variant = ?3
                   AND p.price = ?4 AND p.currency = ?5 AND p.unit = ?6 AND p.price_kind = ?7
                    AND p.price_min IS ?8 AND p.price_max IS ?9 AND p.confidence IS ?10
                    AND (?11=0 OR (p.extra_json=?12 AND p.notes=?13 AND p.source_url=?14 AND p.source_type=?15
                        AND p.published=?16 AND p.published_at IS ?17 AND p.valid_from IS ?18 AND p.valid_to IS ?19))",
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
                    immutable, p.extra_json, p.notes, p.source_url, p.source_type, p.published,
                    p.published_at, p.valid_from, p.valid_to,
                ],
                |r| r.get(0),
            )
            .optional()?;
        let price_id = if let Some(id) = dup {
            if !immutable {
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
            }
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
                if let Some(incoming) = &hofmann_time {
                    // Guarded keys accept several UTC spellings, including
                    // existing raw rows. Carry the exact same time ordering
                    // through to Current, without rewriting their history.
                    let previous = super::hofmann_tin::utc_key(&old_observed)?;
                    (incoming.as_str(), price_id) > (previous.as_str(), old_id)
                } else {
                    (p.observed_at, price_id) > (old_observed.as_str(), old_id)
                }
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

pub(super) fn row_to_price(r: &rusqlite::Row<'_>) -> rusqlite::Result<PriceRow> {
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
    use super::{NewPrice, PriceRow, PublicDb};
    use crate::public::{NewMaterial, NewTrader};
    use crate::test_support::TempDbDir;

    fn setup(name: &str) -> (TempDbDir, PublicDb, i64, i64) {
        let dir = TempDbDir::new(&format!("prices-{name}"));
        let db = PublicDb::open(dir.path()).expect("test db opens");
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
        (dir, db, trader, material)
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

    fn record_mkm_test_snapshot(db: &PublicDb, p: &NewPrice<'_>) {
        const URL: &str = "https://mkm-metals.de/metallpreise";
        const OWNER: &str = "mkm-article-v1";
        db.lock()
            .unwrap()
            .execute(
                "UPDATE traders SET slug='rp-ludwigshafen-mkm-metals' WHERE id=?1",
                [p.trader_id],
            )
            .unwrap();
        let manifest = serde_json::json!({
            "owner": OWNER,
            "trader_id": p.trader_id,
            "source_url": URL,
            "observed_at": p.observed_at,
            "articles": (0..61).map(|i| serde_json::json!({"article": format!("article-{i}")})).collect::<Vec<_>>()
        })
        .to_string();
        db.record_article_price_snapshot(std::slice::from_ref(p), &[], &manifest, &[])
            .unwrap();
    }

    fn price_row_by_id(db: &PublicDb, id: i64) -> PriceRow {
        db.lock()
            .unwrap()
            .query_row(
                "SELECT id,trader_id,material_id,variant,price,currency,unit,price_kind,
                    price_min,price_max,confidence,source_type,published,source_url,
                    observed_at,published_at,valid_from,valid_to,notes,extra_json,ingested_at
                 FROM prices WHERE id=?1",
                [id],
                super::row_to_price,
            )
            .unwrap()
    }

    #[test]
    fn article_semantic_metadata_change_must_append_immutable_evidence() {
        let (_dir, db, trader, material) = setup("article-metadata");
        let mut a = price(
            trader,
            material,
            11.13,
            "2026-10-10T00:00:00.000000000+00:00",
            "2026-10-10T00:00:00.000000000+00:00",
        );
        a.source_url = "https://mkm-metals.de/metallpreise";
        a.extra_json = "{\"snapshot_owner\":\"mkm-article-v1\",\"minimum_kg\":250}";
        record_mkm_test_snapshot(&db, &a);
        let before = db.price_history(trader, material, "", 10).unwrap()[0].clone();
        let mut b = price(
            trader,
            material,
            11.13,
            "2026-10-11T00:00:00.000000000+00:00",
            "2026-10-11T00:00:00.000000000+00:00",
        );
        b.source_url = "https://mkm-metals.de/metallpreise";
        b.extra_json = "{\"snapshot_owner\":\"mkm-article-v1\",\"minimum_kg\":500}";
        record_mkm_test_snapshot(&db, &b);
        let history = db.price_history(trader, material, "", 10).unwrap();
        assert_eq!(
            history.len(),
            2,
            "same amount, different source conditions is a new observation"
        );
        assert_eq!(history[1].extra_json, before.extra_json);
        assert_eq!(history[1].observed_at, before.observed_at);
    }

    #[test]
    fn review_f2_public_single_write_keeps_price_and_pointer_atomic() {
        let (_dir, db, trader, material) = setup("mkm-public-write-rollback");
        let mut owned = price(
            trader,
            material,
            11.13,
            "2026-10-11T00:00:00.000000000+00:00",
            "2026-10-11T00:00:00.000000000+00:00",
        );
        owned.source_url = "https://example.test/prices";
        db.record_price(&owned).unwrap();
        let before = db.current_price_for(trader, material, "").unwrap().unwrap();
        db.lock().unwrap().execute_batch("CREATE TRIGGER reject_current BEFORE INSERT ON current_prices BEGIN SELECT RAISE(ABORT,'test failure'); END;").unwrap();
        let changed = NewPrice {
            price: 12.0,
            observed_at: "2026-10-12T00:00:00Z",
            ..owned
        };
        assert!(db.record_price(&changed).is_err());
        assert_eq!(
            db.price_history(trader, material, "", 10).unwrap().len(),
            1,
            "failed public write must not leave partial evidence"
        );
        let after = db.current_price_for(trader, material, "").unwrap().unwrap();
        assert_eq!(format!("{after:?}"), format!("{before:?}"));
    }

    #[test]
    fn review_f2_unmarked_or_foreign_writer_cannot_mutate_owned_history() {
        let (_dir, db, trader, material) = setup("mkm-existing-owner");
        let mut owned = price(
            trader,
            material,
            11.13,
            "2026-10-11T00:00:00.000000000+00:00",
            "2026-10-11T00:00:00.000000000+00:00",
        );
        owned.source_url = "https://mkm-metals.de/metallpreise";
        owned.extra_json = "{\"snapshot_owner\":\"mkm-article-v1\",\"article\":\"MET-KU-MILL\"}";
        record_mkm_test_snapshot(&db, &owned);
        let id = db
            .current_price_for(trader, material, "")
            .unwrap()
            .unwrap()
            .id;
        let before = db.current_price_for(trader, material, "").unwrap().unwrap();
        for extra in ["{}", "{\"snapshot_owner\":\"other-producer\"}"] {
            let mut incoming = price(
                trader,
                material,
                11.13,
                "2026-10-12T00:00:00Z",
                "2026-10-12T00:00:00Z",
            );
            incoming.extra_json = extra;
            incoming.source_url = owned.source_url;
            assert!(
                db.record_price(&incoming).is_err(),
                "foreign/unmarked API write must reject owned key"
            );
            assert!(db
                .record_price_snapshot(&[incoming], &[], "other-producer")
                .is_err());
            let after = db.current_price_for(trader, material, "").unwrap().unwrap();
            assert_eq!(after.id, id);
            assert_eq!(format!("{after:?}"), format!("{before:?}"));
            assert_eq!(db.price_history(trader, material, "", 10).unwrap().len(), 1);
        }
    }

    #[test]
    fn article_owner_marker_requires_typed_manifest_transaction() {
        let (_dir, db, trader, material) = setup("mkm-single-write-bypass");
        let mut p = price(
            trader,
            material,
            11.13,
            "2026-10-11T00:00:00Z",
            "2026-10-11T00:00:00Z",
        );
        p.source_url = "https://mkm-metals.de/metallpreise";
        p.extra_json = r#"{"snapshot_owner":"mkm-article-v1"}"#;

        assert!(db.record_price(&p).is_err());
        assert!(db
            .price_history(trader, material, "", 10)
            .unwrap()
            .is_empty());
        assert!(db
            .current_price_for(trader, material, "")
            .unwrap()
            .is_none());
    }

    #[test]
    fn article_snapshot_rejects_changed_legacy_candidate() {
        let (_dir, db, trader, material) = setup("mkm-stale-legacy-candidate");
        const URL: &str = "https://mkm-metals.de/metallpreise";
        const OWNER: &str = "mkm-article-v1";
        const NOW: &str = "2026-10-11T00:00:00.000000000+00:00";
        db.lock()
            .unwrap()
            .execute(
                "UPDATE traders SET slug='rp-ludwigshafen-mkm-metals' WHERE id=?1",
                [trader],
            )
            .unwrap();
        let mut legacy = price(trader, material, 7.0, NOW, NOW);
        legacy.source_url = URL;
        legacy.variant = "legacy";
        legacy.notes = "Kupfer Kerze";
        let id = db.record_price(&legacy).unwrap();
        let before = db
            .current_price_for(trader, material, "legacy")
            .unwrap()
            .unwrap();
        db.lock()
            .unwrap()
            .execute(
                "UPDATE prices SET notes='changed after selection' WHERE id=?1",
                [id],
            )
            .unwrap();
        let mut offered = price(trader, material, 11.13, NOW, NOW);
        offered.source_url = URL;
        offered.variant = "new";
        offered.extra_json = r#"{"snapshot_owner":"mkm-article-v1"}"#;
        let manifest = serde_json::json!({
            "owner": OWNER,
            "trader_id": trader,
            "source_url": URL,
            "observed_at": NOW,
            "articles": (0..61).map(|i| serde_json::json!({"article": format!("article-{i}")})).collect::<Vec<_>>()
        })
        .to_string();

        assert!(db
            .record_article_price_snapshot(&[offered], &[before.clone()], &manifest, &[])
            .is_err());
        let after = db
            .current_price_for(trader, material, "legacy")
            .unwrap()
            .unwrap();
        assert_eq!(after.id, before.id);
        assert_eq!(after.notes, "changed after selection");
        assert!(db
            .current_price_for(trader, material, "new")
            .unwrap()
            .is_none());
        assert!(db
            .query_sql("SELECT * FROM source_article_snapshots")
            .unwrap()
            .rows
            .is_empty());
    }

    #[test]
    fn article_owner_cannot_bypass_manifest_via_legacy_snapshot_api() {
        let (_dir, db, trader, material) = setup("article-bypass");
        let mut p = price(
            trader,
            material,
            11.13,
            "2026-10-11T00:00:00Z",
            "2026-10-11T00:00:00Z",
        );
        p.source_url = "https://mkm-metals.de/metallpreise";
        p.extra_json = "{\"snapshot_owner\":\"mkm-article-v1\"}";
        assert!(db
            .record_price_snapshot(&[p], &[], "mkm-article-v1")
            .is_err());
        assert!(db
            .current_price_for(trader, material, "")
            .unwrap()
            .is_none());
    }

    #[test]
    fn article_transaction_rolls_back_after_acceptance_or_manifest_failure() {
        for table in ["trader_materials", "source_article_snapshots"] {
            let (_dir, db, trader, material) = setup(table);
            db.lock()
                .unwrap()
                .execute(
                    "UPDATE traders SET slug='rp-ludwigshafen-mkm-metals' WHERE id=?1",
                    [trader],
                )
                .unwrap();
            let now = "2026-10-11T00:00:00.000000000+00:00";
            let mut old = price(
                trader,
                material,
                7.0,
                "2026-10-10T00:00:00Z",
                "2026-10-10T00:00:00Z",
            );
            old.source_url = "https://mkm-metals.de/metallpreise";
            old.variant = "legacy";
            let id = db.record_price(&old).unwrap();
            let old_row = db
                .current_price_for(trader, material, "legacy")
                .unwrap()
                .unwrap();
            let mut p = price(trader, material, 11.13, now, now);
            p.source_url = old.source_url;
            p.extra_json = "{\"snapshot_owner\":\"mkm-article-v1\"}";
            let manifest = serde_json::json!({"owner":"mkm-article-v1","trader_id":trader,"source_url":p.source_url,"observed_at":now,
                "articles":(0..61).map(|i| serde_json::json!({"article":format!("article-{i}")})).collect::<Vec<_>>()}).to_string();
            db.lock().unwrap().execute_batch(&format!("CREATE TRIGGER reject_article BEFORE INSERT ON {table} BEGIN SELECT RAISE(ABORT,'test rollback'); END;")).unwrap();
            let error = db
                .record_article_price_snapshot(
                    &[p],
                    &[old_row],
                    &manifest,
                    &[(material, "conditions".to_owned())],
                )
                .unwrap_err();
            // Prove the injected write failure was reached, not merely an
            // earlier wrong-cardinality rejection masquerading as rollback.
            assert!(error.to_string().contains("test rollback"), "{error}");
            assert_eq!(
                db.current_price_for(trader, material, "legacy")
                    .unwrap()
                    .unwrap()
                    .id,
                id
            );
            assert!(db
                .current_price_for(trader, material, "")
                .unwrap()
                .is_none());
            assert_eq!(
                db.price_history(trader, material, "legacy", 10)
                    .unwrap()
                    .len(),
                1
            );
            assert!(db.existing_acceptance(trader, material).unwrap().is_none());
            let conn = db.lock().unwrap();
            for ledger in ["source_article_snapshots", "article_acceptance_owners"] {
                let count: i64 = conn
                    .query_row(&format!("SELECT COUNT(*) FROM {ledger}"), [], |r| r.get(0))
                    .unwrap();
                assert_eq!(count, 0);
            }
        }
    }

    #[test]
    fn reviewed_mkm_61_contract_preserves_prior_60_ledger_and_rejects_partial_retirement() {
        // Structural Store contract test. Synthetic article IDs deliberately
        // do not claim to replace the real parser/capture/mapping regressions.
        let (_dir, db, trader, material) = setup("mkm-60-to-61-ledger");
        const URL: &str = "https://mkm-metals.de/metallpreise";
        const OWNER: &str = "mkm-article-v1";
        const BEFORE: &str = "2026-10-17T00:00:00.000000000+00:00";
        const NOW: &str = "2026-10-18T00:00:00.000000000+00:00";
        let manifest = |count: usize, at: &str| {
            serde_json::json!({
            "owner": OWNER, "trader_id": trader, "source_url": URL, "observed_at": at,
            "articles": (0..count).map(|i| serde_json::json!({"article": format!("article-{i}")})).collect::<Vec<_>>()
        }).to_string()
        };
        let old_manifest = manifest(60, BEFORE);
        {
            // Seed only historical evidence; never make the production writer
            // accept stale 60-row sources merely to construct a test fixture.
            let conn = db.lock().unwrap();
            conn.execute(
                "UPDATE traders SET slug='rp-ludwigshafen-mkm-metals' WHERE id=?1",
                [trader],
            )
            .unwrap();
            conn.execute("INSERT INTO source_article_snapshots(trader_id,source_url,owner,observed_at,manifest_json) VALUES(?1,?2,?3,?4,?5)",
                rusqlite::params![trader, URL, OWNER, BEFORE, old_manifest]).unwrap();
        }
        let mut old = price(trader, material, 7.0, BEFORE, BEFORE);
        old.variant = "legacy";
        old.source_url = URL;
        old.extra_json = "{\"snapshot_owner\":\"mkm-article-v1\"}";
        // The old 60-article snapshot predates the typed transaction writer;
        // seed this historical state directly rather than minting its reserved
        // owner marker through the current single-price API.
        {
            let conn = db.lock().unwrap();
            conn.execute(
                "INSERT INTO prices(trader_id,material_id,variant,price,currency,unit,price_kind,
                    price_min,price_max,confidence,source_type,published,source_url,observed_at,
                    published_at,valid_from,valid_to,notes,extra_json,ingested_at)
                 VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20)",
                rusqlite::params![
                    old.trader_id,
                    old.material_id,
                    old.variant,
                    old.price,
                    old.currency,
                    old.unit,
                    old.price_kind,
                    old.price_min,
                    old.price_max,
                    old.confidence,
                    old.source_type,
                    i64::from(old.published),
                    old.source_url,
                    old.observed_at,
                    old.published_at,
                    old.valid_from,
                    old.valid_to,
                    old.notes,
                    old.extra_json,
                    old.ingested_at,
                ],
            )
            .unwrap();
            let id = conn.last_insert_rowid();
            conn.execute(
                "INSERT INTO current_prices(trader_id,material_id,variant,price_id,updated_at)
                 VALUES(?1,?2,?3,?4,?5)",
                rusqlite::params![
                    old.trader_id,
                    old.material_id,
                    old.variant,
                    id,
                    old.observed_at
                ],
            )
            .unwrap();
        }
        let old_row = db
            .current_price_for(trader, material, "legacy")
            .unwrap()
            .unwrap();
        let original_history = db
            .query_sql("SELECT * FROM prices ORDER BY id")
            .unwrap()
            .rows;
        let evidence = || {
            [
                "prices",
                "current_prices",
                "trader_materials",
                "source_article_snapshots",
                "article_acceptance_owners",
            ]
            .iter()
            .map(|t| {
                db.query_sql(&format!("SELECT * FROM {t} ORDER BY rowid"))
                    .unwrap()
                    .rows
            })
            .collect::<Vec<_>>()
        };
        let before = evidence();
        for count in [60, 62] {
            let mut p = price(trader, material, 11.13, NOW, NOW);
            p.source_url = URL;
            p.extra_json = old.extra_json;
            let error = db
                .record_article_price_snapshot(
                    &[p],
                    &[old_row.clone()],
                    &manifest(count, NOW),
                    &[(material, "conditions".to_owned())],
                )
                .unwrap_err();
            assert!(error
                .to_string()
                .contains("article manifest scope/completeness mismatch"));
            assert_eq!(
                evidence(),
                before,
                "wrong cardinality cannot retire or write evidence"
            );
        }
        let mut p = price(trader, material, 11.13, NOW, NOW);
        p.source_url = URL;
        p.extra_json = old.extra_json;
        let new_manifest = manifest(61, NOW);
        db.record_article_price_snapshot(
            &[p],
            &[old_row],
            &new_manifest,
            &[(material, "conditions".to_owned())],
        )
        .unwrap();
        let ledger = db
            .query_sql("SELECT manifest_json FROM source_article_snapshots ORDER BY id")
            .unwrap()
            .rows;
        assert_eq!(ledger.len(), 2);
        assert_eq!(ledger[0][0].as_str(), Some(old_manifest.as_str()));
        assert_eq!(ledger[1][0].as_str(), Some(new_manifest.as_str()));
        assert!(db
            .current_price_for(trader, material, "legacy")
            .unwrap()
            .is_none());
        assert_eq!(
            db.price_history(trader, material, "legacy", 10)
                .unwrap()
                .len(),
            1
        );
        let after = db
            .query_sql("SELECT * FROM prices ORDER BY id")
            .unwrap()
            .rows;
        for row in original_history {
            assert!(
                after.contains(&row),
                "historical stored fields remain immutable"
            );
        }
    }

    #[test]
    fn snapshot_failure_rolls_back_prices_and_retirement() {
        let (_dir, db, trader, material) = setup("snapshot");
        let mut old = price(
            trader,
            material,
            7.0,
            "2026-10-09T00:00:00Z",
            "2026-10-09T00:00:00Z",
        );
        old.source_url = "https://example.test/prices";
        let legacy = db.record_price(&old).unwrap();
        let legacy_row = price_row_by_id(&db, legacy);
        let mut a = price(
            trader,
            material,
            8.0,
            "2026-10-10T00:00:00Z",
            "2026-10-10T00:00:00Z",
        );
        a.source_url = old.source_url;
        a.variant = "a";
        a.extra_json = "{\"snapshot_owner\":\"test\"}";
        let b = NewPrice {
            variant: "b",
            ..price(trader, material, 9.0, a.observed_at, a.ingested_at)
        };
        let b = NewPrice {
            source_url: a.source_url,
            extra_json: a.extra_json,
            ..b
        };
        db.lock().unwrap().execute_batch("CREATE TRIGGER fail_b BEFORE INSERT ON prices WHEN NEW.variant='b' BEGIN SELECT RAISE(ABORT,'injected failure'); END;").unwrap();
        assert!(db
            .record_price_snapshot(&[a, b], &[legacy_row], "test")
            .is_err());
        assert_eq!(
            db.current_price_for(trader, material, "")
                .unwrap()
                .unwrap()
                .id,
            legacy
        );
        assert!(db
            .current_price_for(trader, material, "a")
            .unwrap()
            .is_none());
        assert_eq!(
            db.test_query("SELECT COUNT(*) FROM prices")[0][0],
            serde_json::json!(1)
        );
    }

    #[test]
    fn snapshot_retirement_failure_foreign_collision_and_late_input_are_safe() {
        let (_dir, db, trader, material) = setup("snapshot-retire");
        let mut old = price(
            trader,
            material,
            7.0,
            "2026-10-09T00:00:00Z",
            "2026-10-09T00:00:00Z",
        );
        old.source_url = "https://example.test/prices";
        let legacy = db.record_price(&old).unwrap();
        let legacy_row = price_row_by_id(&db, legacy);
        let make_new = || NewPrice {
            variant: "dynamic / ab 137 kg",
            source_url: old.source_url,
            extra_json: "{\"snapshot_owner\":\"test\"}",
            ..price(
                trader,
                material,
                8.0,
                "2026-10-10T00:00:00Z",
                "2026-10-10T00:00:00Z",
            )
        };
        db.lock().unwrap().execute_batch("CREATE TRIGGER fail_retire BEFORE DELETE ON current_prices BEGIN SELECT RAISE(ABORT,'retirement failure'); END;").unwrap();
        assert!(db
            .record_price_snapshot(&[make_new()], &[legacy_row.clone()], "test")
            .is_err());
        assert!(db
            .current_price_for(trader, material, "dynamic / ab 137 kg")
            .unwrap()
            .is_none());
        assert_eq!(
            db.current_price_for(trader, material, "")
                .unwrap()
                .unwrap()
                .id,
            legacy
        );
        db.lock()
            .unwrap()
            .execute_batch("DROP TRIGGER fail_retire;")
            .unwrap();
        let mut foreign = make_new();
        foreign.source_url = "https://other.test/";
        let foreign_id = db.record_price(&foreign).unwrap();
        assert!(db
            .record_price_snapshot(&[make_new()], &[legacy_row.clone()], "test")
            .is_err());
        assert_eq!(
            db.current_price_for(trader, material, foreign.variant)
                .unwrap()
                .unwrap()
                .id,
            foreign_id
        );
        assert_eq!(
            db.current_price_for(trader, material, "")
                .unwrap()
                .unwrap()
                .id,
            legacy
        );
        // Use a different new key to keep the foreign-source pointer untouched.
        let mut fresh = make_new();
        fresh.variant = "new / ab 250 kg";
        assert_eq!(
            db.record_price_snapshot(&[fresh], &[legacy_row], "test")
                .unwrap(),
            1
        );
        assert_eq!(
            db.price_history(trader, material, "", 10).unwrap()[0].id,
            legacy
        );
        let mut late = make_new();
        late.variant = "too late";
        late.observed_at = "2026-10-08T00:00:00Z";
        assert!(db.record_price_snapshot(&[late], &[], "test").is_err());
        assert!(db
            .current_price_for(trader, material, "new / ab 250 kg")
            .unwrap()
            .is_some());
        assert_eq!(
            db.current_price_for(trader, material, foreign.variant)
                .unwrap()
                .unwrap()
                .id,
            foreign_id
        );
        assert!(db
            .current_price_for(trader, material, "too late")
            .unwrap()
            .is_none());
    }

    #[test]
    fn snapshot_rejects_foreign_producer_collision_without_changes() {
        let (_dir, db, trader, material) = setup("snapshot-producer");
        let mut legacy = price(
            trader,
            material,
            7.0,
            "2026-10-09T00:00:00Z",
            "2026-10-09T00:00:00Z",
        );
        legacy.source_url = "https://example.test/prices";
        let legacy_id = db.record_price(&legacy).unwrap();
        let legacy_row = price_row_by_id(&db, legacy_id);
        let mut foreign = price(
            trader,
            material,
            8.0,
            legacy.observed_at,
            legacy.ingested_at,
        );
        foreign.source_url = legacy.source_url;
        foreign.variant = "canonical / ab 137 kg";
        foreign.extra_json = "{\"snapshot_owner\":\"other-producer\"}";
        let foreign_id = db.record_price(&foreign).unwrap();
        let before = (
            db.test_query("SELECT * FROM current_prices ORDER BY variant"),
            db.test_query("SELECT * FROM prices ORDER BY id"),
        );
        let mut offered = price(
            trader,
            material,
            8.0,
            "2026-10-10T00:00:00Z",
            "2026-10-10T00:00:00Z",
        );
        offered.source_url = legacy.source_url;
        offered.variant = foreign.variant;
        offered.extra_json = "{\"snapshot_owner\":\"this-producer\"}";
        assert!(db
            .record_price_snapshot(&[offered], &[legacy_row.clone()], "this-producer")
            .is_err());
        assert_eq!(
            (
                db.test_query("SELECT * FROM current_prices ORDER BY variant"),
                db.test_query("SELECT * FROM prices ORDER BY id")
            ),
            before
        );
        let mut fresh = price(
            trader,
            material,
            9.0,
            "2026-10-10T00:00:00Z",
            "2026-10-10T00:00:00Z",
        );
        fresh.source_url = legacy.source_url;
        fresh.variant = "fresh / ab 250 kg";
        fresh.extra_json = "{\"snapshot_owner\":\"this-producer\"}";
        assert_eq!(
            db.record_price_snapshot(&[fresh], &[legacy_row], "this-producer")
                .unwrap(),
            1
        );
        assert!(db
            .current_price_for(trader, material, "")
            .unwrap()
            .is_none());
        assert_eq!(
            db.current_price_for(trader, material, foreign.variant)
                .unwrap()
                .unwrap()
                .id,
            foreign_id
        );
        assert_eq!(
            db.price_history(trader, material, foreign.variant, 10)
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn snapshot_adopts_explicitly_empty_producer_marker() {
        let (_dir, db, trader, material) = setup("snapshot-empty-producer");
        let mut p = price(
            trader,
            material,
            7.0,
            "2026-10-09T00:00:00Z",
            "2026-10-09T00:00:00Z",
        );
        p.source_url = "https://example.test/prices";
        p.variant = "canonical / ab 137 kg";
        p.extra_json = "{\"snapshot_owner\":\"\"}";
        let original = db.record_price(&p).unwrap();
        p.observed_at = "2026-10-10T00:00:00Z";
        p.extra_json = "{\"snapshot_owner\":\"test\"}";
        assert_eq!(db.record_price_snapshot(&[p], &[], "test").unwrap(), 0);
        let current = db
            .current_price_for(trader, material, "canonical / ab 137 kg")
            .unwrap()
            .unwrap();
        assert_eq!(current.id, original);
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&current.extra_json).unwrap()
                ["snapshot_owner"],
            "test"
        );
    }

    #[test]
    fn snapshot_refresh_stamps_ownership_and_keeps_publication_basis() {
        let (_dir, db, trader, material) = setup("snapshot-marker");
        let mut p = price(
            trader,
            material,
            7.0,
            "2026-10-09T00:00:00Z",
            "2026-10-09T00:00:00Z",
        );
        p.variant = "dynamic / ab 137 kg";
        p.source_url = "https://example.test/prices";
        p.extra_json = "{\"published_at_basis\":\"observed_price_change\"}";
        p.published_at = Some("2026-10-09T00:00:00Z");
        let original = db.record_price(&p).unwrap();
        p.observed_at = "2026-10-10T00:00:00Z";
        p.published_at = None;
        p.extra_json = "{\"snapshot_owner\":\"test\"}";
        assert_eq!(db.record_price_snapshot(&[p], &[], "test").unwrap(), 0);
        let current = db
            .current_price_for(trader, material, "dynamic / ab 137 kg")
            .unwrap()
            .unwrap();
        assert_eq!(current.id, original);
        let extra: serde_json::Value = serde_json::from_str(&current.extra_json).unwrap();
        assert_eq!(extra["snapshot_owner"], "test");
        assert_eq!(extra["published_at_basis"], "observed_price_change");
        let mut next = price(
            trader,
            material,
            7.0,
            "2026-10-11T00:00:00Z",
            "2026-10-11T00:00:00Z",
        );
        next.variant = "new / ab 250 kg";
        next.source_url = "https://example.test/prices";
        next.extra_json = "{\"snapshot_owner\":\"test\"}";
        assert_eq!(db.record_price_snapshot(&[next], &[], "test").unwrap(), 1);
        assert!(db
            .current_price_for(trader, material, "dynamic / ab 137 kg")
            .unwrap()
            .is_none());
        assert_eq!(
            db.price_history(trader, material, "dynamic / ab 137 kg", 10)
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn current_moves_forward_only() {
        let (_dir, db, trader, material) = setup("t");
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
        let (_dir, db, trader, material) = setup("t");
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
        let (_dir, db, trader, material) = setup("t");
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
    fn current_view_hides_wahl_quotes_outside_the_daily_source_window() {
        const WAHL_SLUG: &str = "th-zella-mehlis-wahl-co-inh-thomas-madeiski";
        const WAHL_URL: &str = "https://wahlundco.de/";
        let (_dir, db, trader, material) = setup("wahl-view-expiry");
        let (fresh, stale, future): (String, String, String) = db
            .lock()
            .expect("lock")
            .query_row(
                "SELECT strftime('%Y-%m-%dT%H:%M:%SZ', 'now', '-23 hours'),
                        strftime('%Y-%m-%dT%H:%M:%SZ', 'now', '-25 hours'),
                        strftime('%Y-%m-%dT%H:%M:%SZ', 'now', '+25 hours')",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .expect("SQLite clock values");

        db.lock()
            .expect("lock")
            .execute(
                "UPDATE traders SET slug = ?1 WHERE id = ?2",
                rusqlite::params![WAHL_SLUG, trader],
            )
            .expect("set Wahl slug");

        for (variant, published_at, source_url) in [
            ("fresh", fresh.as_str(), WAHL_URL),
            ("stale", stale.as_str(), WAHL_URL),
            ("future", future.as_str(), WAHL_URL),
            (
                "other-source-stale",
                stale.as_str(),
                "https://other.example/prices",
            ),
        ] {
            db.record_price(&NewPrice {
                variant,
                source_url,
                published_at: Some(published_at),
                observed_at: published_at,
                ingested_at: published_at,
                ..price(trader, material, 5.0, published_at, published_at)
            })
            .expect("record source observation");
        }

        let visible = db.test_query(&format!(
            "SELECT variant FROM v_current_prices WHERE trader_slug = '{WAHL_SLUG}' ORDER BY variant"
        ));
        assert_eq!(
            visible,
            vec![
                vec!["fresh".to_owned()],
                vec!["other-source-stale".to_owned()]
            ]
        );
        assert_eq!(
            db.test_query("SELECT COUNT(*) FROM current_prices"),
            vec![vec![serde_json::json!(4)]]
        );
    }

    #[test]
    fn variants_stay_separate_per_grade() {
        let (_dir, db, trader, material) = setup("t");
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
        let dir = TempDbDir::new("prices-repro");
        {
            let conn = rusqlite::Connection::open(dir.path().join("public.db")).expect("old db");
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
        let db = PublicDb::open(dir.path()).expect("open migrates");
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
        let (_dir, db, trader, _) = setup("wkg-spot-fix");
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
        let (_dir, db, trader, copper) = setup("az-ladder-fix");
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
        assert_eq!(version, 9);
    }

    #[test]
    fn migration_9_retires_only_the_reviewed_stale_wahl_current_snapshot() {
        const WAHL_URL: &str = "https://wahlundco.de/";
        const OLD_OBSERVED_AT: &str = "2026-10-21T03:30:55.060415905+00:00";
        let (_dir, db, trader, material) = setup("wahl-stale-current");

        for index in 0..32 {
            let variant = format!("legacy-{index:02}");
            let kind = if index < 30 { "exact" } else { "upto" };
            let eur = 5.0 + f64::from(index) / 100.0;
            db.record_price(&NewPrice {
                variant: &variant,
                price_kind: kind,
                price: eur,
                confidence: Some(if kind == "exact" { 1.0 } else { 0.5 }),
                price_min: None,
                price_max: (kind == "upto").then_some(eur),
                source_url: WAHL_URL,
                observed_at: OLD_OBSERVED_AT,
                ingested_at: OLD_OBSERVED_AT,
                notes: "reviewed pre-update observation",
                extra_json: "{}",
                ..price(trader, material, eur, OLD_OBSERVED_AT, OLD_OBSERVED_AT)
            })
            .expect("legacy Wahl row");
        }

        let fresh = db
            .record_price(&NewPrice {
                variant: "fresh-source-observation",
                price_kind: "approx",
                confidence: Some(0.5),
                source_url: WAHL_URL,
                observed_at: "2026-10-26T23:00:00Z",
                ingested_at: "2026-10-26T23:00:00Z",
                notes: "later observation must remain current",
                ..price(
                    trader,
                    material,
                    8.0,
                    "2026-10-26T23:00:00Z",
                    "2026-10-26T23:00:00Z",
                )
            })
            .expect("later unrelated observation");

        {
            let conn = db.conn.lock().expect("lock");
            conn.execute(
                "UPDATE traders SET slug = ?1 WHERE id = ?2",
                rusqlite::params!["th-zella-mehlis-wahl-co-inh-thomas-madeiski", trader],
            )
            .expect("set Wahl slug");
            conn.execute_batch("PRAGMA user_version = 8;")
                .expect("simulate pre-migration database");
            super::migrate(&conn).expect("retire reviewed old current snapshot");
        }

        for index in 0..32 {
            let variant = format!("legacy-{index:02}");
            assert!(db
                .current_price_for(trader, material, &variant)
                .expect("current pointer query")
                .is_none());
            let history = db
                .price_history(trader, material, &variant, 10)
                .expect("historical observation");
            assert_eq!(history.len(), 1);
            assert_eq!(
                history[0].price_kind,
                if index < 30 { "exact" } else { "upto" }
            );
            assert_eq!(history[0].notes, "reviewed pre-update observation");
        }
        assert_eq!(
            db.current_price_for(trader, material, "fresh-source-observation")
                .expect("newer pointer query")
                .expect("newer source row remains current")
                .id,
            fresh
        );
        let version: i64 = db
            .conn
            .lock()
            .expect("lock")
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("version");
        assert_eq!(version, 9);
    }

    #[test]
    fn migration_9_fails_closed_on_a_partial_wahl_population() {
        const WAHL_URL: &str = "https://wahlundco.de/";
        const OLD_OBSERVED_AT: &str = "2026-10-21T03:30:55.060415905+00:00";
        let (_dir, db, trader, material) = setup("wahl-partial-stale-current");
        db.record_price(&NewPrice {
            source_url: WAHL_URL,
            observed_at: OLD_OBSERVED_AT,
            ingested_at: OLD_OBSERVED_AT,
            extra_json: "{}",
            ..price(trader, material, 5.0, OLD_OBSERVED_AT, OLD_OBSERVED_AT)
        })
        .expect("partial legacy row");

        let conn = db.conn.lock().expect("lock");
        conn.execute(
            "UPDATE traders SET slug = ?1 WHERE id = ?2",
            rusqlite::params!["th-zella-mehlis-wahl-co-inh-thomas-madeiski", trader],
        )
        .expect("set Wahl slug");
        conn.execute_batch("PRAGMA user_version = 8;")
            .expect("simulate pre-migration database");
        assert!(
            super::migrate(&conn).is_err(),
            "partial set must not be retired"
        );
        let version: i64 = conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("version");
        assert_eq!(version, 8, "failed migration must not advance the version");
        drop(conn);
        assert!(db
            .current_price_for(trader, material, "")
            .expect("pointer query")
            .is_some());
    }

    #[test]
    fn migration_9_rejects_a_stale_timestamp_with_unreviewed_source_metadata() {
        const OLD_OBSERVED_AT: &str = "2026-10-21T03:30:55.060415905+00:00";
        let (_dir, db, trader, material) = setup("wahl-stale-unreviewed-metadata");
        let stale = db
            .record_price(&NewPrice {
                source_url: "https://unexpected.example/",
                observed_at: OLD_OBSERVED_AT,
                ingested_at: OLD_OBSERVED_AT,
                ..price(trader, material, 5.0, OLD_OBSERVED_AT, OLD_OBSERVED_AT)
            })
            .expect("timestamp-matching row with different source");

        let conn = db.conn.lock().expect("lock");
        conn.execute(
            "UPDATE traders SET slug = ?1 WHERE id = ?2",
            rusqlite::params!["th-zella-mehlis-wahl-co-inh-thomas-madeiski", trader],
        )
        .expect("set Wahl slug");
        conn.execute_batch("PRAGMA user_version = 8;")
            .expect("simulate pre-migration database");
        assert!(
            super::migrate(&conn).is_err(),
            "unreviewed source metadata must not be treated as no candidates"
        );
        assert_eq!(
            conn.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
                .expect("version"),
            8
        );
        assert_eq!(
            conn.query_row(
                "SELECT COUNT(*) FROM current_prices WHERE price_id = ?1",
                [stale],
                |row| row.get::<_, i64>(0)
            )
            .expect("pointer remains"),
            1
        );
    }

    #[test]
    fn migration_9_rejects_a_stale_price_with_a_mismatched_current_pointer_key() {
        const WAHL_URL: &str = "https://wahlundco.de/";
        const OLD_OBSERVED_AT: &str = "2026-10-21T03:30:55.060415905+00:00";
        let (_dir, db, trader, material) = setup("wahl-stale-mismatched-pointer");
        let stale = db
            .record_price(&NewPrice {
                source_url: WAHL_URL,
                observed_at: OLD_OBSERVED_AT,
                ingested_at: OLD_OBSERVED_AT,
                ..price(trader, material, 5.0, OLD_OBSERVED_AT, OLD_OBSERVED_AT)
            })
            .expect("legacy Wahl row");

        let conn = db.conn.lock().expect("lock");
        conn.execute(
            "UPDATE traders SET slug = ?1 WHERE id = ?2",
            rusqlite::params!["th-zella-mehlis-wahl-co-inh-thomas-madeiski", trader],
        )
        .expect("set Wahl slug");
        conn.execute(
            "UPDATE current_prices SET variant = 'mismatched-key' WHERE price_id = ?1",
            [stale],
        )
        .expect("corrupt pointer key for migration test");
        conn.execute_batch("PRAGMA user_version = 8;")
            .expect("simulate pre-migration database");
        assert!(
            super::migrate(&conn).is_err(),
            "a pointer whose key disagrees with its price must not be retired"
        );
        assert_eq!(
            conn.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
                .expect("version"),
            8
        );
        assert_eq!(
            conn.query_row(
                "SELECT COUNT(*) FROM current_prices WHERE price_id = ?1 AND variant = 'mismatched-key'",
                [stale],
                |row| row.get::<_, i64>(0)
            )
            .expect("pointer remains"),
            1
        );
    }

    #[test]
    fn migration_retires_schiefer_sale_quotes_from_current_prices() {
        const SCHIEFER_URL: &str = "https://schieferco.de/aktuelle-preise";
        let (_dir, db, trader, _) = setup("schiefer-sales-fix");
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
        assert_eq!(version, 9);
    }

    #[test]
    fn migration_retires_4782_mismappings_without_inventing_replacements() {
        for (slug, url, material_slug, variant, label, corrected_material, corrected_variant) in [
            (
                "hh-lokstedt-hansa-goldankauf",
                "https://hansa-goldankauf.de/",
                "gold",
                "",
                "875er Gold",
                "gold",
                "875",
            ),
            (
                "th-erfurt-goldankauf-boerse-filiale-erfurt",
                "https://www.goldankauf-boerse.de/ankaufsrechner/",
                "silber",
                "",
                "625er Silber",
                "silber",
                "625",
            ),
            (
                "sn-frankenberg-volker-lungwitz-schrotthandel",
                "https://vlschrott.de/einkaufspreise/",
                "aluminium-gemischt",
                "Blech/Guß o. Fe",
                "Zinkblech",
                "zink",
                "",
            ),
            (
                "sn-rochlitz-db-recycling-denis-blum",
                "https://www.dein-schrottplatz.de/pages/preise.php",
                "aluminium-gemischt",
                "Blech/Guß 2% Fe",
                "Zinkblech",
                "zink",
                "",
            ),
            (
                "ni-wangerland-hooksiel-26434-manuel-triebsch-autoverwertung-altmetall",
                "https://www.schrott-triebsch.de/",
                "kupfer-gemischt",
                "",
                "Kupferkabel o.Stecker",
                "kabel-kupfer",
                "o. Stecker",
            ),
            (
                "sn-leipzig-albus-leipzig",
                "https://www.albus-leipzig.de/preise",
                "aluminium-gemischt",
                "mit Kundenkarte",
                "Alu-Profile",
                "aluminium-profile",
                "mit Kundenkarte",
            ),
            (
                "sn-leipzig-albus-leipzig",
                "https://www.albus-leipzig.de/preise",
                "aluminium-gemischt",
                "ohne Kundenkarte",
                "Alu-Profile",
                "aluminium-profile",
                "ohne Kundenkarte",
            ),
            (
                "sn-leipzig-albus-leipzig",
                "https://www.albus-leipzig.de/preise",
                "aluminium-guss",
                "mit Kundenkarte",
                "Alufelgen sauber",
                "aluminium-guss",
                "Felgen, mit Kundenkarte",
            ),
            (
                "sn-leipzig-albus-leipzig",
                "https://www.albus-leipzig.de/preise",
                "aluminium-guss",
                "ohne Kundenkarte",
                "Alufelgen sauber",
                "aluminium-guss",
                "Felgen, ohne Kundenkarte",
            ),
            (
                "sn-leipzig-albus-leipzig",
                "https://www.albus-leipzig.de/preise",
                "eisenschrott-gussbruch",
                "mit Kundenkarte",
                "Bremsscheiben",
                "eisenschrott-gussbruch",
                "Bremsscheiben, mit Kundenkarte",
            ),
            (
                "sn-leipzig-albus-leipzig",
                "https://www.albus-leipzig.de/preise",
                "eisenschrott-gussbruch",
                "ohne Kundenkarte",
                "Bremsscheiben",
                "eisenschrott-gussbruch",
                "Bremsscheiben, ohne Kundenkarte",
            ),
        ] {
            let (_dir, db, trader, _) = setup("4782-mismapping");
            let add_material = |slug| {
                db.upsert_material(&NewMaterial {
                    slug,
                    name_de: slug,
                    category: "nichteisen",
                    unit: "EUR/kg",
                    description: "",
                    updated_at: "2026-10-08T00:00:00Z",
                })
                .expect("material")
            };
            let material = add_material(material_slug);
            let corrected = add_material(corrected_material);
            let bad_id = db
                .record_price(&NewPrice {
                    variant,
                    source_url: url,
                    notes: label,
                    extra_json: "{\"map_v\":1,\"published_at_basis\":\"page_stated\"}",
                    ..price(
                        trader,
                        material,
                        2.0,
                        "2026-10-08T00:00:00Z",
                        "2026-10-08T00:00:00Z",
                    )
                })
                .expect("legacy mapping");
            let good_id = db
                .record_price(&NewPrice {
                    variant: corrected_variant,
                    source_url: url,
                    notes: label,
                    extra_json: "{\"map_v\":2}",
                    ..price(
                        trader,
                        corrected,
                        2.0,
                        "2026-10-08T01:00:00Z",
                        "2026-10-08T01:00:00Z",
                    )
                })
                .expect("corrected mapping");
            {
                let conn = db.conn.lock().expect("lock");
                conn.execute(
                    "UPDATE traders SET slug = ?1 WHERE id = ?2",
                    rusqlite::params![slug, trader],
                )
                .expect("trader identity");
                conn.execute_batch("PRAGMA user_version = 5;")
                    .expect("pre-migration");
                super::migrate(&conn).expect("migration");
                super::migrate(&conn).expect("idempotent rerun");
            }
            assert!(
                db.current_price_for(trader, material, variant)
                    .expect("current")
                    .is_none(),
                "{slug}/{material_slug}/{variant}"
            );
            let history = db
                .price_history(trader, material, variant, 10)
                .expect("history");
            assert_eq!(history.len(), 1, "no replacement observation");
            assert_eq!(history[0].id, bad_id);
            assert_eq!(history[0].price, 2.0);
            assert_eq!(history[0].observed_at, "2026-10-08T00:00:00Z");
            assert_eq!(
                history[0].extra_json,
                "{\"map_v\":1,\"published_at_basis\":\"page_stated\"}"
            );
            assert_eq!(history[0].price_kind, "approx");
            assert_eq!(history[0].confidence, Some(0.0));
            assert!(history[0].notes.starts_with(label), "source label retained");
            assert!(history[0].notes.contains("Fehlmapping #4782"));
            let good = db
                .current_price_for(trader, corrected, corrected_variant)
                .expect("corrected current")
                .expect("corrected grade preserved");
            assert_eq!(good.id, good_id);
            assert_eq!(good.price_kind, "exact");
            assert_eq!(good.confidence, Some(1.0));
            assert_eq!(good.notes, label);
        }
    }

    #[test]
    fn migration_retires_suspended_heraeus_buy_quotes_and_acceptances_only() {
        const HERAEUS_SLUG: &str = "he-hanau-heraeus-goldstore-hanau-argor-heraeus-de";
        const HERAEUS_URL: &str = "https://store.heraeus-gold.com/ankauf/";
        let (_dir, db, trader, copper) = setup("heraeus-buy-suspended");
        let gold = db
            .upsert_material(&NewMaterial {
                slug: "gold",
                name_de: "Gold",
                category: "edelmetall",
                unit: "EUR/g",
                description: "",
                updated_at: "2026-10-23T00:00:00Z",
            })
            .expect("gold");
        let silver = db
            .upsert_material(&NewMaterial {
                slug: "silber",
                name_de: "Silber",
                category: "edelmetall",
                unit: "EUR/g",
                description: "",
                updated_at: "2026-10-23T00:00:00Z",
            })
            .expect("silver");

        let heraeus_gold = db
            .record_price(&NewPrice {
                unit: "EUR/g",
                source_url: HERAEUS_URL,
                notes: "Gold (999)",
                ..price(
                    trader,
                    gold,
                    110.30,
                    "2026-10-23T12:14:25Z",
                    "2026-10-23T12:14:25Z",
                )
            })
            .expect("Heraeus gold quote");
        let unrelated_copper = db
            .record_price(&NewPrice {
                source_url: "https://other.example/buy-prices",
                notes: "independent copper quote",
                ..price(
                    trader,
                    copper,
                    8.5,
                    "2026-10-23T12:14:25Z",
                    "2026-10-23T12:14:25Z",
                )
            })
            .expect("unrelated quote");

        db.set_acceptance(
            trader,
            gold,
            true,
            "Unverbindlicher Ankauf-Richtwert; mindestens 10 g",
            None,
            None,
            "2026-10-23T12:14:25Z",
            "2026-10-23T12:14:25Z",
        )
        .expect("Heraeus gold acceptance");
        db.set_acceptance(
            trader,
            silver,
            true,
            "Unverbindlicher Ankauf-Richtwert; mindestens 10 g",
            None,
            None,
            "2026-10-23T12:14:25Z",
            "2026-10-23T12:14:25Z",
        )
        .expect("Heraeus silver acceptance");
        db.set_acceptance(
            trader,
            copper,
            true,
            "independently reviewed acceptance",
            None,
            None,
            "2026-10-23T12:14:25Z",
            "2026-10-23T12:14:25Z",
        )
        .expect("unrelated acceptance");

        {
            let conn = db.conn.lock().expect("lock");
            conn.execute(
                "UPDATE traders SET slug = ?1 WHERE id = ?2",
                rusqlite::params![HERAEUS_SLUG, trader],
            )
            .expect("set Heraeus slug");
            conn.execute_batch("PRAGMA user_version = 7;")
                .expect("simulate pre-migration database");
            super::migrate(&conn).expect("migration");
        }

        assert!(db
            .current_price_for(trader, gold, "")
            .expect("current gold")
            .is_none());
        assert_eq!(
            db.current_price_for(trader, copper, "")
                .expect("current copper")
                .expect("unrelated quote remains current")
                .id,
            unrelated_copper
        );
        let history = db
            .price_history(trader, gold, "", 10)
            .expect("gold history");
        let retained = history
            .iter()
            .find(|row| row.id == heraeus_gold)
            .expect("historical Heraeus quote retained");
        assert_eq!(retained.price, 110.30);
        assert_eq!(retained.notes, "Gold (999)");

        let gold_acceptance = db
            .existing_acceptance(trader, gold)
            .expect("gold acceptance")
            .expect("gold acceptance row");
        assert!(!gold_acceptance.0);
        assert!(gold_acceptance
            .1
            .contains("aus technischen Gründen eingestellt"));
        assert!(gold_acceptance.1.contains("frühere Bedingungen"));
        let silver_acceptance = db
            .existing_acceptance(trader, silver)
            .expect("silver acceptance")
            .expect("silver acceptance row");
        assert!(!silver_acceptance.0);
        assert_eq!(
            db.existing_acceptance(trader, copper)
                .expect("unrelated acceptance")
                .expect("unrelated acceptance row"),
            (true, "independently reviewed acceptance".into())
        );
        let version: i64 = db
            .conn
            .lock()
            .expect("lock")
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("version");
        assert_eq!(version, 9);
    }

    #[test]
    fn migration_4782_preserves_neighboring_keys_sources_labels_and_old_quotes() {
        // Change each evidence dimension independently. An old timestamp or
        // map_v=1 is not a deletion criterion; genuine neighboring grades
        // can have exactly the same price as a mismapped observation.
        for (slug, url, material_slug, variant, label) in [
            (
                "another-trader",
                "https://hansa-goldankauf.de/",
                "gold",
                "",
                "875er Gold",
            ),
            (
                "hh-lokstedt-hansa-goldankauf",
                "https://example.test/",
                "gold",
                "",
                "875er Gold",
            ),
            (
                "hh-lokstedt-hansa-goldankauf",
                "https://hansa-goldankauf.de/",
                "silber",
                "",
                "875er Gold",
            ),
            (
                "hh-lokstedt-hansa-goldankauf",
                "https://hansa-goldankauf.de/",
                "gold",
                "875",
                "875er Gold",
            ),
            (
                "hh-lokstedt-hansa-goldankauf",
                "https://hansa-goldankauf.de/",
                "gold",
                "",
                "Gold",
            ),
            (
                "th-erfurt-goldankauf-boerse-filiale-erfurt",
                "https://www.goldankauf-boerse.de/ankaufsrechner/",
                "silber",
                "500",
                "500er Silber",
            ),
            (
                "sn-frankenberg-volker-lungwitz-schrotthandel",
                "https://vlschrott.de/einkaufspreise/",
                "aluminium-gemischt",
                "Blech/Guß o. Fe",
                "Al Blech / Guß o. Fe",
            ),
            (
                "sn-rochlitz-db-recycling-denis-blum",
                "https://www.dein-schrottplatz.de/pages/preise.php",
                "aluminium-gemischt",
                "Blech/Guß 2% Fe",
                "Al Blech/Guß mit 2% Fe",
            ),
            (
                "ni-wangerland-hooksiel-26434-manuel-triebsch-autoverwertung-altmetall",
                "https://www.schrott-triebsch.de/",
                "kupfer-gemischt",
                "",
                "Kupfer",
            ),
            (
                "sn-leipzig-albus-leipzig",
                "https://www.albus-leipzig.de/preise",
                "aluminium-gemischt",
                "mit Kundenkarte",
                "Alu",
            ),
            (
                "sn-leipzig-albus-leipzig",
                "https://www.albus-leipzig.de/preise",
                "aluminium-gemischt",
                "ohne Kundenkarte",
                "Alu",
            ),
            (
                "sn-leipzig-albus-leipzig",
                "https://www.albus-leipzig.de/preise",
                "eisenschrott-gussbruch",
                "mit Kundenkarte",
                "Eisenguss",
            ),
            (
                "sn-leipzig-albus-leipzig",
                "https://www.albus-leipzig.de/preise",
                "eisenschrott-gussbruch",
                "ohne Kundenkarte",
                "Eisenguss",
            ),
        ] {
            let (_dir, db, trader, _) = setup("4782-neighbors");
            let material = db
                .upsert_material(&NewMaterial {
                    slug: material_slug,
                    name_de: material_slug,
                    category: "nichteisen",
                    unit: "EUR/kg",
                    description: "",
                    updated_at: "2026-01-01T00:00:00Z",
                })
                .expect("material");
            let id = db
                .record_price(&NewPrice {
                    variant,
                    source_url: url,
                    notes: label,
                    extra_json: "{\"map_v\":1}",
                    ..price(
                        trader,
                        material,
                        2.0,
                        "2026-01-01T00:00:00Z",
                        "2026-01-01T00:00:00Z",
                    )
                })
                .expect("neighbor quote");
            {
                let conn = db.conn.lock().expect("lock");
                conn.execute(
                    "UPDATE traders SET slug = ?1 WHERE id = ?2",
                    rusqlite::params![slug, trader],
                )
                .expect("trader identity");
                conn.execute_batch("PRAGMA user_version = 5;")
                    .expect("pre-migration");
                super::migrate(&conn).expect("migration");
            }
            let current = db
                .current_price_for(trader, material, variant)
                .expect("current")
                .expect("neighbor retained");
            assert_eq!(current.id, id);
            assert_eq!(current.price_kind, "exact");
            assert_eq!(current.confidence, Some(1.0));
            assert_eq!(current.notes, label);
            assert_eq!(current.observed_at, "2026-01-01T00:00:00Z");
            assert_eq!(current.extra_json, "{\"map_v\":1}");
        }
    }

    #[test]
    fn migration_4782_flags_history_but_does_not_move_a_legitimate_current_pointer() {
        let (_dir, db, trader, material) = setup("4782-current-survives");
        let url = "https://www.schrott-triebsch.de/";
        let bad_id = db
            .record_price(&NewPrice {
                source_url: url,
                notes: "Kupferkabel o.Stecker",
                ..price(
                    trader,
                    material,
                    3.3,
                    "2026-10-07T00:00:00Z",
                    "2026-10-07T00:00:00Z",
                )
            })
            .expect("mismapped history");
        let good_id = db
            .record_price(&NewPrice {
                source_url: url,
                notes: "Kupfer",
                ..price(
                    trader,
                    material,
                    9.5,
                    "2026-10-08T00:00:00Z",
                    "2026-10-08T00:00:00Z",
                )
            })
            .expect("genuine newer copper quote");
        {
            let conn = db.conn.lock().expect("lock");
            conn.execute(
                "UPDATE materials SET slug = 'kupfer-gemischt' WHERE id = ?1",
                [material],
            )
            .expect("material identity");
            conn.execute("UPDATE traders SET slug = 'ni-wangerland-hooksiel-26434-manuel-triebsch-autoverwertung-altmetall' WHERE id = ?1", [trader]).expect("trader identity");
            conn.execute_batch("PRAGMA user_version = 5;")
                .expect("pre-migration");
            super::migrate(&conn).expect("migration");
            // Even an interrupted version bookkeeping / explicit replay must
            // not append the diagnostic twice or invalidate a newer quote.
            conn.execute_batch("PRAGMA user_version = 5;")
                .expect("replay");
            super::migrate(&conn).expect("idempotent replay");
        }
        assert_eq!(
            db.current_price_for(trader, material, "")
                .expect("current")
                .expect("exists")
                .id,
            good_id
        );
        let history = db.price_history(trader, material, "", 10).expect("history");
        assert_eq!(history.len(), 2);
        assert_eq!(history[1].id, bad_id);
        assert_eq!(history[1].notes.matches("Fehlmapping #4782").count(), 1);
        assert_eq!(history[0].notes, "Kupfer");
    }

    #[test]
    fn migration_4782_rolls_back_history_annotation_if_pointer_retirement_fails() {
        let (_dir, db, trader, material) = setup("4782-atomic");
        db.record_price(&NewPrice {
            source_url: "https://hansa-goldankauf.de/",
            notes: "875er Gold",
            ..price(
                trader,
                material,
                95.7,
                "2026-10-08T00:00:00Z",
                "2026-10-08T00:00:00Z",
            )
        })
        .expect("legacy quote");
        {
            let conn = db.conn.lock().expect("lock");
            conn.execute(
                "UPDATE materials SET slug = 'gold' WHERE id = ?1",
                [material],
            )
            .expect("material identity");
            conn.execute(
                "UPDATE traders SET slug = 'hh-lokstedt-hansa-goldankauf' WHERE id = ?1",
                [trader],
            )
            .expect("trader identity");
            conn.execute_batch(
                "PRAGMA user_version = 5;
                CREATE TEMP TRIGGER fail_retirement BEFORE DELETE ON current_prices
                BEGIN SELECT RAISE(ABORT, 'test retirement failure'); END;",
            )
            .expect("failure injection");
            assert!(super::migrate(&conn).is_err());
        }
        let row = db
            .current_price_for(trader, material, "")
            .expect("current")
            .expect("unchanged pointer");
        assert_eq!(
            row.price_kind, "exact",
            "failed migration must remain retryable"
        );
        assert_eq!(row.notes, "875er Gold");
        {
            let conn = db.conn.lock().expect("lock");
            conn.execute_batch("DROP TRIGGER fail_retirement;")
                .expect("allow retry");
            super::migrate(&conn).expect("retry migration");
        }
        assert!(db
            .current_price_for(trader, material, "")
            .expect("current")
            .is_none());
    }

    // Legacy repair regression only: seed historical evidence directly in an
    // isolated database, then exercise the normal application startup seam.
    // Owner observed the pre-fix red run (14 remaining instead of 0) before
    // authorizing migration 7; see the Goldtrans current-repair report.
    #[test]
    fn startup_migration_retires_only_14_goldtrans_zero_current_keys_preserving_history() {
        const SLUG: &str = "hh-wandsbek-goldtrans-edelmetallhandel";
        const URL: &str =
            "https://www.goldtrans.de/goldankauf-preise-aktueller-goldpreis-ankauf-in-hamburg.html";
        const BEFORE: &str = "2026-10-18T12:30:09.018060442+00:00";
        const NOW: &str = "2026-10-18T18:32:56.399800153+00:00";
        const GRADES: [(&str, f64); 13] = [
            ("999", 110.70),
            ("986", 109.15),
            ("980", 108.49),
            ("965", 106.83),
            ("916", 101.40),
            ("900", 99.63),
            ("875", 96.86),
            ("833", 92.21),
            ("750", 83.03),
            ("585", 64.76),
            ("416", 46.05),
            ("375", 41.51),
            ("333", 36.86),
        ];

        // Each guard case uses the same known keys, not a different variant
        // that would hide an overbroad source/currency/unit/amount predicate.
        for (case, amount, source, currency, unit, retire) in [
            ("zero", 0.0, URL, "EUR", "EUR/g", true),
            ("positive", 1.0, URL, "EUR", "EUR/g", false),
            (
                "other-source",
                0.0,
                "https://example.test/buy",
                "EUR",
                "EUR/g",
                false,
            ),
            ("other-currency", 0.0, URL, "USD", "EUR/g", false),
            ("other-unit", 0.0, URL, "EUR", "EUR/kg", false),
        ] {
            let (dir, db, trader, copper) = setup(&format!("goldtrans-current-{case}"));
            let mut materials = Vec::new();
            for (slug, name) in [("gold", "Gold"), ("zahngold", "Zahngold")] {
                materials.push(
                    db.upsert_material(&NewMaterial {
                        slug,
                        name_de: name,
                        category: "edelmetall",
                        unit: "EUR/g",
                        description: "",
                        updated_at: BEFORE,
                    })
                    .expect("fixture material"),
                );
            }
            let gold = materials[0];
            let dental = materials[1];
            let keys: Vec<_> = GRADES
                .iter()
                .map(|&(v, p)| (gold, v, p))
                .chain(std::iter::once((dental, "", 68.63)))
                .collect();
            let mut candidate_ids = Vec::new();
            {
                let conn = db.lock().expect("fixture lock");
                conn.execute(
                    "UPDATE traders SET slug=?1 WHERE id=?2",
                    rusqlite::params![SLUG, trader],
                )
                .expect("fixture Goldtrans identity");
                conn.execute(
                    "INSERT INTO traders (slug,name,first_seen_at,updated_at) VALUES ('other-goldtrader','Other',?1,?1)",
                    [BEFORE],
                ).expect("fixture other trader");
                let other = conn.last_insert_rowid();
                // Raw inserts model already-persisted legacy evidence without
                // depending on future handler/write guards or write deduping.
                let insert = |tid: i64,
                              mid: i64,
                              variant: &str,
                              value: f64,
                              cur: &str,
                              u: &str,
                              src: &str,
                              at: &str,
                              current: bool| {
                    conn.execute(
                        "INSERT INTO prices (trader_id,material_id,variant,price,currency,unit,
                         price_kind,confidence,source_type,published,source_url,observed_at,published_at,
                         notes,extra_json,ingested_at)
                         VALUES (?1,?2,?3,?4,?5,?6,'exact',1.0,'haendler_angabe',1,?7,?8,
                         '2026-10-18T00:00:00+00:00','retained source label',
                         '{\"map_v\":2,\"published_at_basis\":\"page_stated\"}',?8)",
                        rusqlite::params![tid,mid,variant,value,cur,u,src,at],
                    ).expect("legacy history fixture");
                    let id = conn.last_insert_rowid();
                    if current {
                        conn.execute(
                            "INSERT INTO current_prices (trader_id,material_id,variant,price_id,updated_at)
                             VALUES (?1,?2,?3,?4,?5)",
                            rusqlite::params![tid,mid,variant,id,at],
                        ).expect("legacy current fixture");
                    }
                    id
                };
                // An older zero that is no longer Current must remain raw
                // history too. Every affected key also has a positive ancestor:
                // retiring the zero must NOT resurrect that stale quote.
                insert(
                    trader,
                    gold,
                    "916",
                    0.0,
                    "EUR",
                    "EUR/g",
                    URL,
                    "2026-09-30T17:06:58+00:00",
                    false,
                );
                for &(mid, variant, positive) in &keys {
                    insert(
                        trader, mid, variant, positive, "EUR", "EUR/g", URL, BEFORE, false,
                    );
                    candidate_ids.push(insert(
                        trader, mid, variant, amount, currency, unit, source, NOW, true,
                    ));
                }
                // Neighboring material/variant/trader identities are outside
                // the confirmed 14-key scope even at the same zero/source.
                for (tid, mid, variant) in [
                    (other, gold, "999"),
                    (other, dental, ""),
                    (trader, copper, "999"),
                    (trader, gold, ""),
                    (trader, gold, "9999"),
                    (trader, dental, "999"),
                ] {
                    insert(tid, mid, variant, 0.0, "EUR", "EUR/g", URL, NOW, true);
                }
                // Seed explicit acceptance evidence so absence of changes is
                // tested against actual rows rather than an empty table.
                conn.execute(
                    "INSERT INTO trader_materials
                     (trader_id,material_id,accepts,conditions,observed_at,updated_at)
                     VALUES (?1,?2,0,'acceptance must remain unchanged',?3,?3)",
                    rusqlite::params![trader, gold, BEFORE],
                )
                .expect("acceptance fixture");
                conn.execute_batch("PRAGMA user_version = 6;")
                    .expect("simulate existing pre-repair application database");
            }
            assert_eq!(keys.len(), 14);
            let history = db.test_query("SELECT * FROM prices ORDER BY id");
            let traders = db.test_query("SELECT * FROM traders ORDER BY id");
            let materials = db.test_query("SELECT * FROM materials ORDER BY id");
            let acceptance =
                db.test_query("SELECT * FROM trader_materials ORDER BY trader_id,material_id");
            let pointers = db
                .test_query("SELECT * FROM current_prices ORDER BY trader_id,material_id,variant");
            let expected_pointers: Vec<_> = pointers
                .iter()
                .filter(|row| {
                    !retire
                        || !candidate_ids
                            .iter()
                            .any(|id| row[3] == serde_json::json!(id))
                })
                .cloned()
                .collect();
            assert_eq!(
                pointers.len() - expected_pointers.len(),
                if retire { 14 } else { 0 }
            );
            drop(db);

            // PublicDb::open is the real startup caller, not a replacement
            // DELETE in the test. The second reopen checks idempotence and
            // prevents history-based pointer reconstruction on later startups.
            for startup in 1..=2 {
                let db = PublicDb::open(dir.path()).expect("normal startup migration");
                let version: i64 = db
                    .lock()
                    .expect("version lock")
                    .query_row("PRAGMA user_version", [], |r| r.get(0))
                    .expect("version");
                assert_eq!(version, 9);
                assert_eq!(
                    db.test_query("SELECT * FROM prices ORDER BY id"),
                    history,
                    "{case}, startup {startup}: raw SQL history must remain byte-for-byte in its stored fields"
                );
                assert_eq!(db.test_query("SELECT * FROM traders ORDER BY id"), traders);
                assert_eq!(
                    db.test_query("SELECT * FROM materials ORDER BY id"),
                    materials
                );
                assert_eq!(
                    db.test_query("SELECT * FROM trader_materials ORDER BY trader_id,material_id"),
                    acceptance
                );
                let mut remaining = 0;
                for &(mid, variant, _) in &keys {
                    let current = db
                        .current_price_for(trader, mid, variant)
                        .expect("current API");
                    remaining += usize::from(current.is_some());
                    let rows = db
                        .price_history(trader, mid, variant, 10)
                        .expect("history API");
                    assert_eq!(
                        rows.len(),
                        if mid == gold && variant == "916" {
                            3
                        } else {
                            2
                        }
                    );
                    assert_eq!(rows[0].price, amount);
                }
                assert_eq!(
                    remaining,
                    if retire { 0 } else { 14 },
                    "{case}, startup {startup}: retire only confirmed zero Current; never revive positive ancestors"
                );
                assert_eq!(
                    db.test_query(
                        "SELECT * FROM current_prices ORDER BY trader_id,material_id,variant"
                    ),
                    expected_pointers,
                    "{case}, startup {startup}: preserve every out-of-scope pointer and timestamp"
                );
                let view = db.query_sql(&format!(
                    "SELECT material_slug,variant,price FROM v_current_prices WHERE trader_slug='{SLUG}'
                     AND ((material_slug='gold' AND variant IN
                     ('999','986','980','965','916','900','875','833','750','585','416','375','333'))
                     OR (material_slug='zahngold' AND variant=''))"
                )).expect("public current view");
                assert_eq!(view.rows.len(), if retire { 0 } else { 14 });
            }
        }
    }

    #[test]
    fn migration_7_goldtrans_rolls_back_partial_retirement_and_allows_retry() {
        let (_dir, db, trader, _) = setup("goldtrans-retirement-rollback");
        let now = "2026-10-18T18:32:56.399800153+00:00";
        let gold = db
            .upsert_material(&NewMaterial {
                slug: "gold",
                name_de: "Gold",
                category: "edelmetall",
                unit: "EUR/g",
                description: "",
                updated_at: now,
            })
            .expect("gold fixture");
        for variant in ["999", "585"] {
            db.record_price(&NewPrice {
                variant,
                unit: "EUR/g",
                source_url: "https://www.goldtrans.de/goldankauf-preise-aktueller-goldpreis-ankauf-in-hamburg.html",
                ..price(trader, gold, 0.0, now, now)
            })
            .expect("legacy zero fixture");
        }
        {
            let conn = db.lock().expect("fixture lock");
            conn.execute(
                "UPDATE traders SET slug='hh-wandsbek-goldtrans-edelmetallhandel' WHERE id=?1",
                [trader],
            )
            .expect("Goldtrans identity");
            // FAIL in an AFTER trigger leaves the first deletion applied to
            // the statement. Only transaction rollback restores that pointer.
            conn.execute_batch(
                "PRAGMA user_version = 6;
                 CREATE TEMP TRIGGER fail_goldtrans_retirement AFTER DELETE ON current_prices
                 BEGIN SELECT RAISE(FAIL, 'injected partial retirement'); END;",
            )
            .expect("failure injection");
        }
        let history = db.test_query("SELECT * FROM prices ORDER BY id");
        let pointers = db.test_query("SELECT * FROM current_prices ORDER BY variant");
        assert_eq!(pointers.len(), 2);
        {
            let conn = db.lock().expect("migration lock");
            assert!(super::migrate(&conn).is_err());
            assert!(
                conn.is_autocommit(),
                "failed migration must end its transaction"
            );
            let version: i64 = conn
                .query_row("PRAGMA user_version", [], |r| r.get(0))
                .expect("failed migration version");
            assert_eq!(version, 6, "failure must leave migration retryable");
        }
        assert_eq!(db.test_query("SELECT * FROM prices ORDER BY id"), history);
        assert_eq!(
            db.test_query("SELECT * FROM current_prices ORDER BY variant"),
            pointers
        );
        {
            let conn = db.lock().expect("retry lock");
            conn.execute_batch("DROP TRIGGER fail_goldtrans_retirement;")
                .expect("allow retry");
            super::migrate(&conn).expect("retry migration");
            let version: i64 = conn
                .query_row("PRAGMA user_version", [], |r| r.get(0))
                .expect("successful retry version");
            assert_eq!(version, 9);
        }
        assert_eq!(db.test_query("SELECT * FROM prices ORDER BY id"), history);
        assert!(db.test_query("SELECT * FROM current_prices").is_empty());
    }

    #[test]
    fn acceptance_matrix() {
        let (_dir, db, trader, material) = setup("t");
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
