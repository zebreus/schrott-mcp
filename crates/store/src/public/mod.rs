//! Public database (`public.db`): the entire queriable data set.
//!
//! Everything here is exposed to all MCP users — nothing is user specific.
//!
//! Domain model (German scrap trade):
//! - `traders` — Schrotthändler, Wertstoffhändler, Metallhändler,
//!   Autoverwerter, Containerdienste, … in Deutschland.
//! - `materials` — the price catalog (Kupfer, Messing, Stahlschrott, …).
//! - `trader_materials` — which trader accepts which material.
//! - `prices` — append-only price observations (latest + history).
//! - `current_prices` — materialized latest price per trader + material.
//! - `v_current_prices` — convenience view joining all of the above.
//!
//! Conventions for agents and future scrapers:
//! - All timestamps are RFC 3339 strings (UTC).
//! - `valid_from` / `valid_to` bound the time a fact is/was true;
//!   `NULL` means open-ended (still true / true since forever).
//! - Uncertainty is explicit: `price_min` / `price_max` span the plausible
//!   range (`NULL` = exact), `confidence` is 0..1 (`NULL` = unknown),
//!   `source_type` says where the number came from and `published` says
//!   whether the trader published it themselves.
//! - `extra_json` on every table is reserved headroom for future fields —
//!   the schema grows by adding columns, never by breaking old ones.

pub mod materials;
pub mod prices;
pub mod traders;

pub use materials::{MaterialRow, NewMaterial};
pub use prices::{NewPrice, PriceRow};
pub use traders::{NewTrader, SeedKept, TraderRow};

use std::sync::Mutex;

use schrott_mcp_core::Stats;

use super::error::StoreError;

/// Tables from the old generic demo corpus. The Schrott domain model
/// replaces them; dropping keeps agents on the one true schema.
const LEGACY_DROP: &str = "
DROP TABLE IF EXISTS items;
DROP TABLE IF EXISTS datasets;
DROP TABLE IF EXISTS sources;
";

/// One result column: name plus SQLite type.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SqlColumn {
    pub name: String,
    /// Value type inferred from the first non-null value
    /// (`integer`, `real`, `text`, or `null` when the column is all null).
    #[serde(rename = "type")]
    pub dtype: String,
}

/// A read-only query result: typed columns plus JSON rows.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SqlResult {
    pub columns: Vec<SqlColumn>,
    pub rows: Vec<Vec<serde_json::Value>>,
}

/// Words that must never appear outside string literals in ad-hoc SQL.
/// (`replace` is deliberately absent: it is a legitimate SELECT function.)
/// Matching is per token, so `updated` or prose inside literals never trips
/// the list — only a bare `update`, `delete`, … does.
const FORBIDDEN_WORDS: &[&str] = &[
    "insert",
    "update",
    "delete",
    "drop",
    "alter",
    "create",
    "attach",
    "detach",
    "pragma",
    "vacuum",
    "reindex",
    "analyze",
    "savepoint",
    "release",
    "begin",
    "commit",
    "rollback",
    "transaction",
    "copy",
    "grant",
    "revoke",
];

/// Byte inside an unquoted word token (`[A-Za-z0-9_]`).
#[inline]
fn is_word_byte(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

/// Case-insensitive ASCII match against the denylist (words are ASCII-only
/// by construction, so no allocation and no Unicode case-fold surprises).
fn is_forbidden_word(word: &str) -> bool {
    FORBIDDEN_WORDS
        .iter()
        .any(|deny| word.eq_ignore_ascii_case(deny))
}

/// Accept exactly one read-only statement: `SELECT ...` or `WITH ...`
/// (whose writes, if any, are caught by the denylist below). Everything
/// else — stacked statements, writes smuggled into CTEs, pragmas — is out.
///
/// One byte scan classifies everything SQLite treats as non-code —
/// `'...'` literals (`''` escapes), `"..."` / `` `...` `` / `[...]`
/// quoted identifiers, `--` and `/* */` comments — and in that single pass
/// collects the first word (must be `SELECT`/`WITH`), any denylist hit,
/// and the statement separator (`;` outside non-code, at most a trailing
/// one). Anything but whitespace/comments after a trailing `;` is stacking.
///
/// Deliberately not delegated to `Connection::prepare`: without rusqlite's
/// `extra_check` feature `prepare` succeeds on stacked input and silently
/// drops the tail — only the first statement runs (see
/// `prepare_ignores_stacked_tail_without_validator`). The scanner is also
/// what keeps this check pure: no database handle, clear reject reasons.
pub fn validate_readonly_sql(sql: &str) -> Result<(), StoreError> {
    if sql.trim().is_empty() {
        return Err(StoreError::Rejected("empty query"));
    }
    let bytes = sql.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut first: Option<&str> = None;
    let mut forbidden = false;
    let mut terminated = false;
    let mut stacked = false;
    while i < len {
        match bytes[i] {
            b'\'' => {
                // `'...'` literal with `''` escapes.
                i += 1;
                while i < len {
                    if bytes[i] == b'\'' {
                        if bytes.get(i + 1) == Some(&b'\'') {
                            i += 2;
                        } else {
                            i += 1;
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }
                stacked |= terminated;
            }
            b'"' | b'`' => {
                let quote = bytes[i];
                i += 1;
                while i < len {
                    if bytes[i] == quote {
                        // `"a""b"` is one escaped identifier.
                        if quote == b'"' && bytes.get(i + 1) == Some(&b'"') {
                            i += 2;
                            continue;
                        }
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                stacked |= terminated;
            }
            b'[' => {
                while i < len && bytes[i] != b']' {
                    i += 1;
                }
                i = (i + 1).min(len);
                stacked |= terminated;
            }
            b'-' if bytes.get(i + 1) == Some(&b'-') => {
                while i < len && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                i += 2;
                while i < len && !(bytes[i] == b'*' && bytes.get(i + 1) == Some(&b'/')) {
                    i += 1;
                }
                i = (i + 2).min(len);
            }
            b';' => {
                // A second separator can never be a lone terminator.
                stacked |= terminated;
                terminated = true;
                i += 1;
            }
            c if is_word_byte(c) => {
                let start = i;
                while i < len && is_word_byte(bytes[i]) {
                    i += 1;
                }
                // Word bytes are ASCII, so the slice always lands on char
                // boundaries.
                let word = &sql[start..i];
                if first.is_none() {
                    first = Some(word);
                }
                forbidden |= is_forbidden_word(word);
                stacked |= terminated;
            }
            c => {
                // Any other visible character after the terminator (a second
                // statement's punctuation) is stacking, not trailing noise.
                stacked |= terminated && !c.is_ascii_whitespace();
                i += 1;
            }
        }
    }
    if stacked {
        return Err(StoreError::Rejected(
            "multiple statements are not allowed; send one SELECT at a time",
        ));
    }
    match first {
        Some(w) if w.eq_ignore_ascii_case("select") || w.eq_ignore_ascii_case("with") => {}
        _ => {
            return Err(StoreError::Rejected(
                "only SELECT (or WITH … SELECT) queries are allowed",
            ));
        }
    }
    if forbidden {
        return Err(StoreError::Rejected(
            "write statements are not allowed; this tool is read-only",
        ));
    }
    Ok(())
}

fn sql_value_to_json(value: rusqlite::types::Value) -> serde_json::Value {
    use rusqlite::types::Value as Raw;
    match value {
        Raw::Null => serde_json::Value::Null,
        Raw::Integer(i) => serde_json::json!(i),
        Raw::Real(f) => serde_json::Number::from_f64(f)
            .map_or(serde_json::Value::Null, serde_json::Value::Number),
        Raw::Text(t) => serde_json::Value::String(t),
        Raw::Blob(b) => serde_json::Value::String(String::from_utf8_lossy(&b).into_owned()),
    }
}

/// Public queriable database handle.
pub struct PublicDb {
    conn: Mutex<rusqlite::Connection>,
}

impl PublicDb {
    /// Open the public database strictly read-only. Used by the isolated
    /// query worker: even a validator bypass cannot write through this.
    pub fn open_read_only(data_dir: &std::path::Path) -> Result<Self, StoreError> {
        use rusqlite::OpenFlags;
        let conn = rusqlite::Connection::open_with_flags(
            data_dir.join("public.db"),
            OpenFlags::SQLITE_OPEN_READ_ONLY,
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Open (creating parent dirs and schema) the public database.
    pub fn open(data_dir: &std::path::Path) -> Result<Self, StoreError> {
        std::fs::create_dir_all(data_dir).map_err(|e| {
            StoreError::Db(rusqlite::Error::InvalidPath(data_dir.join(e.to_string())))
        })?;
        let conn = rusqlite::Connection::open(data_dir.join("public.db"))?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        conn.execute_batch(traders::SCHEMA)?;
        conn.execute_batch(materials::SCHEMA)?;
        conn.execute_batch(prices::SCHEMA)?;
        conn.execute_batch(LEGACY_DROP)?;
        traders::migrate(&conn)?;
        prices::migrate(&conn)?;
        conn.execute_batch(prices::VIEW)?;
        // Consistency tripwire: pointers are rebuilt deterministically from
        // prices, so "prices but no pointers" is always a bug — scream.
        let (n_prices, n_current): (i64, i64) = (
            conn.query_row("SELECT COUNT(*) FROM prices", [], |r| r.get(0))?,
            conn.query_row("SELECT COUNT(*) FROM current_prices", [], |r| r.get(0))?,
        );
        if n_prices > 0 && n_current == 0 {
            tracing::error!("public.db inconsistent: {n_prices} prices but no current pointers");
        }
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, rusqlite::Connection>, StoreError> {
        self.conn.lock().map_err(|_| StoreError::Lock)
    }

    /// Run one validated read-only query and materialize every row.
    /// Queries must be self-contained: no bound parameters. An optional
    /// trailing semicolon is allowed; anything stacked behind it is rejected.
    /// A hard row cap keeps a careless `SELECT` from eating
    /// the server; callers page/preview on top.
    pub fn query_sql(&self, sql: &str) -> Result<SqlResult, StoreError> {
        const FULL_ROW_CAP: usize = 20_000;
        validate_readonly_sql(sql)?;
        let conn = self.lock()?;
        let mut stmt = conn.prepare(sql)?;
        let width = stmt.column_count();
        let mut columns = Vec::with_capacity(width);
        for i in 0..width {
            let name = stmt.column_name(i).unwrap_or("?").to_owned();
            // rusqlite exposes no declared-type accessor here, so every
            // column starts untyped and is inferred from values below.
            columns.push(SqlColumn {
                name,
                dtype: String::new(),
            });
        }
        let mut query_rows = stmt.query([])?;
        let mut rows = Vec::new();
        while let Some(row) = query_rows.next()? {
            if rows.len() >= FULL_ROW_CAP {
                return Err(StoreError::Rejected(
                    "result too large; narrow it with WHERE / LIMIT",
                ));
            }
            let mut record = Vec::with_capacity(width);
            for i in 0..width {
                record.push(sql_value_to_json(row.get(i)?));
            }
            rows.push(record);
        }
        // Fill in types the query did not declare, from first non-null value.
        for (col, dtype) in columns.iter_mut().enumerate() {
            if dtype.dtype.is_empty() {
                dtype.dtype = rows
                    .iter()
                    .filter_map(|r| r.get(col))
                    .find(|v| !v.is_null())
                    .map(|v| {
                        if v.is_i64() || v.is_u64() {
                            "integer"
                        } else if v.is_f64() {
                            "real"
                        } else if v.is_string() {
                            "text"
                        } else {
                            "null"
                        }
                    })
                    .unwrap_or("null")
                    .to_owned();
            }
        }
        Ok(SqlResult { columns, rows })
    }

    /// Corpus-wide counters.
    pub fn counts(&self) -> Result<Stats, StoreError> {
        let conn = self.lock()?;
        let traders: i64 = conn.query_row("SELECT COUNT(*) FROM traders", [], |r| r.get(0))?;
        let materials: i64 = conn.query_row("SELECT COUNT(*) FROM materials", [], |r| r.get(0))?;
        let prices: i64 = conn.query_row("SELECT COUNT(*) FROM prices", [], |r| r.get(0))?;
        Ok(Stats {
            traders,
            materials,
            prices,
        })
    }

    /// Helper for tests: run a scalar query and return the raw JSON rows.
    #[cfg(test)]
    pub(super) fn test_query(&self, sql: &str) -> Vec<Vec<serde_json::Value>> {
        self.query_sql(sql).expect("test query works").rows
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn readonly_sql_accepts_plain_selects() {
        use super::validate_readonly_sql;
        assert!(validate_readonly_sql("SELECT 1").is_ok());
        assert!(
            validate_readonly_sql("  -- a comment\nSELECT slug, name_de FROM materials").is_ok()
        );
        assert!(validate_readonly_sql("/* c */ WITH x AS (SELECT 1) SELECT * FROM x").is_ok());
        // Prose mentioning forbidden words inside literals is fine.
        assert!(
            validate_readonly_sql("SELECT * FROM traders WHERE notes LIKE '%update dropped%'")
                .is_ok()
        );
    }

    #[test]
    fn readonly_sql_rejects_writes_and_stacks() {
        use super::validate_readonly_sql;
        assert!(validate_readonly_sql("").is_err());
        assert!(validate_readonly_sql("DROP TABLE traders").is_err());
        assert!(validate_readonly_sql("SELECT 1; DELETE FROM prices").is_err());
        // A lone trailing terminator is fine; only stacking is out.
        assert!(validate_readonly_sql("SELECT 1;").is_ok());
        // Semicolons inside literals are data, not separators.
        assert!(
            validate_readonly_sql("SELECT COUNT(*) FROM traders WHERE city LIKE '%;%'").is_ok()
        );
        assert!(validate_readonly_sql("SELECT group_concat(slug, '; ') FROM traders").is_ok());
        assert!(validate_readonly_sql("SELECT 'a;b'; SELECT 2").is_err());
        assert!(validate_readonly_sql("WITH x AS (SELECT 1) UPDATE prices SET price=1.0").is_err());
        assert!(validate_readonly_sql("PRAGMA table_info(traders)").is_err());
        assert!(validate_readonly_sql("EXPLAIN SELECT 1").is_err());
        assert!(validate_readonly_sql("VACUUM").is_err());
    }

    #[test]
    fn readonly_sql_scanner_treats_comments_and_quotes_as_non_code() {
        use super::validate_readonly_sql;
        // A `;` inside a comment is not a separator (the old literal-only
        // strip rejected these as stacking).
        assert!(validate_readonly_sql("SELECT 1 /* ; */").is_ok());
        assert!(validate_readonly_sql("-- pick ; here\nSELECT 1").is_ok());
        assert!(validate_readonly_sql("SELECT 1 -- trailing ; remark").is_ok());
        // Comments after a trailing terminator are not a second statement.
        assert!(validate_readonly_sql("SELECT 1; -- done").is_ok());
        assert!(validate_readonly_sql("SELECT 1; /* done */").is_ok());
        // …but a second separator never is a lone terminator.
        assert!(validate_readonly_sql("SELECT 1;;").is_err());
        assert!(validate_readonly_sql("; SELECT 1").is_err());
        // Denylist words inside comments or quoted identifiers are prose,
        // not writes …
        assert!(validate_readonly_sql("SELECT 1 /* drop the mic */").is_ok());
        assert!(validate_readonly_sql("SELECT 1 -- delete me").is_ok());
        assert!(validate_readonly_sql(r#"SELECT "update" FROM materials"#).is_ok());
        assert!(validate_readonly_sql("SELECT `delete` FROM materials").is_ok());
        // … while token matching still rejects the bare words, in any case …
        assert!(validate_readonly_sql("select 1").is_ok());
        assert!(validate_readonly_sql("SeLeCt 1").is_ok());
        assert!(validate_readonly_sql("DrOp TABLE traders").is_err());
        assert!(validate_readonly_sql("SELECT update FROM materials").is_err());
        // … and longer tokens containing a deny word stay allowed.
        assert!(validate_readonly_sql("SELECT updated, x FROM t WHERE y = 1").is_ok());
        assert!(validate_readonly_sql("SELECT * FROM t WHERE note = 'it''s; deleted'").is_ok());
    }

    #[test]
    fn prepare_ignores_stacked_tail_without_validator() {
        // Proof that the single-statement guarantee cannot come from the
        // SQLite API: without rusqlite's `extra_check` feature `prepare`
        // succeeds on stacked input, runs only the first statement, and
        // silently drops the tail — so the scanner above must reject
        // stacking itself.
        let conn = rusqlite::Connection::open_in_memory().expect("memory db");
        conn.execute_batch(
            "CREATE TABLE t (id INTEGER PRIMARY KEY); INSERT INTO t (id) VALUES (1);",
        )
        .expect("seed");
        let mut stmt = conn
            .prepare("SELECT id FROM t; DELETE FROM t")
            .expect("prepare drops the tail silently");
        let n: i64 = stmt
            .query_row([], |r| r.get(0))
            .expect("first statement runs");
        assert_eq!(n, 1);
        let left: i64 = conn
            .query_row("SELECT COUNT(*) FROM t", [], |r| r.get(0))
            .expect("count");
        assert_eq!(
            left, 1,
            "the stacked DELETE never ran — the validator must reject it"
        );
        assert!(super::validate_readonly_sql("SELECT id FROM t; DELETE FROM t").is_err());
    }

    #[test]
    fn sql_tool_round_trip_with_truncation() {
        use super::PublicDb;
        let dir = crate::test_support::TempDbDir::new("sql");
        let db = PublicDb::open(dir.path()).expect("test db opens");
        db.upsert_material(&super::NewMaterial {
            slug: "kupfer-test",
            name_de: "Kupfer Test",
            category: "nichteisen",
            unit: "EUR/kg",
            description: "",
            updated_at: "2026-01-01T00:00:00Z",
        })
        .expect("material");
        let r = db
            .query_sql("SELECT id, slug FROM materials ORDER BY id")
            .expect("select works");
        let names: Vec<&str> = r.columns.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, vec!["id", "slug"]);
        assert_eq!(r.columns[0].dtype, "integer");
        assert_eq!(r.columns[1].dtype, "text");
        assert_eq!(r.rows.len(), 1);
        let r = db
            .query_sql("SELECT COUNT(*) AS n, 1.5 AS f FROM materials")
            .expect("select works");
        assert_eq!(r.columns[0].dtype, "integer");
        assert_eq!(r.columns[1].dtype, "real");
        assert_eq!(r.rows.len(), 1);
        assert!(db.query_sql("DELETE FROM materials").is_err());
    }

    #[test]
    fn legacy_demo_tables_are_gone() {
        use super::PublicDb;
        let dir = crate::test_support::TempDbDir::new("legacy");
        // Simulate an old database file with the demo corpus still inside.
        {
            let conn = rusqlite::Connection::open(dir.path().join("public.db")).expect("old db");
            conn.execute_batch(
                "CREATE TABLE sources (slug TEXT PRIMARY KEY, name TEXT NOT NULL, url TEXT NOT NULL, description TEXT NOT NULL DEFAULT '');
                 CREATE TABLE datasets (slug TEXT PRIMARY KEY, source_slug TEXT NOT NULL, name TEXT NOT NULL, description TEXT NOT NULL DEFAULT '');
                 CREATE TABLE items (id INTEGER PRIMARY KEY AUTOINCREMENT, dataset_slug TEXT NOT NULL, external_id TEXT NOT NULL);",
            )
            .expect("legacy schema");
        }
        let db = PublicDb::open(dir.path()).expect("open migrates");
        let tables = db.test_query(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name IN ('sources', 'datasets', 'items')",
        );
        assert!(tables.is_empty(), "legacy tables are dropped");
        // …while the new model exists.
        let tables = db.test_query(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name IN ('traders', 'materials', 'prices', 'current_prices') ORDER BY name",
        );
        assert_eq!(tables.len(), 4);
    }
}
