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

/// Blank out `'...'`, `"..."`, `` `...` `` and `[...]` literals (with
/// `''` escape handling) so keyword scans never trip over prose.
fn strip_literals(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' {
            // Single-quoted literal with '' escapes.
            out.push(' ');
            i += 1;
            while i < chars.len() {
                if chars[i] == '\'' {
                    if chars.get(i + 1) == Some(&'\'') {
                        i += 2;
                    } else {
                        i += 1;
                        break;
                    }
                } else {
                    i += 1;
                }
            }
        } else if c == '"' || c == '`' {
            out.push(' ');
            i += 1;
            while i < chars.len() && chars[i] != c {
                i += 1;
            }
            i += 1;
        } else if c == '[' {
            out.push(' ');
            i += 1;
            while i < chars.len() && chars[i] != ']' {
                i += 1;
            }
            i += 1;
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

/// Drop leading `--` and `/* */` comments for the statement-type check.
fn strip_leading_comments(mut s: &str) -> &str {
    loop {
        s = s.trim_start();
        if let Some(rest) = s.strip_prefix("--") {
            s = match rest.find('\n') {
                Some(i) => &rest[i + 1..],
                None => "",
            };
        } else if let Some(rest) = s.strip_prefix("/*") {
            match rest.find("*/") {
                Some(i) => s = &rest[i + 2..],
                None => return "",
            }
        } else {
            return s;
        }
    }
}

/// Accept exactly one read-only statement: `SELECT ...` or `WITH ...`
/// (whose writes, if any, are caught by the denylist below). Everything
/// else — stacked statements, writes smuggled into CTEs, pragmas — is out.
pub fn validate_readonly_sql(sql: &str) -> Result<(), StoreError> {
    if sql.trim().is_empty() {
        return Err(StoreError::Rejected("empty query"));
    }
    // Semicolons inside string literals are data, not statement separators
    // ("%;%" must work) — look at literal-stripped code and allow at most
    // a trailing terminator. Anything after it is stacking.
    let code = strip_literals(sql);
    if let Some(idx) = code.find(';') {
        if !code[idx + 1..].trim().is_empty() {
            return Err(StoreError::Rejected(
                "multiple statements are not allowed; send one SELECT at a time",
            ));
        }
    }
    let body = strip_leading_comments(sql);
    let first = body
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .find(|w| !w.is_empty())
        .unwrap_or_default()
        .to_uppercase();
    if first != "SELECT" && first != "WITH" {
        return Err(StoreError::Rejected(
            "only SELECT (or WITH … SELECT) queries are allowed",
        ));
    }
    let code = strip_literals(body);
    for word in code.split(|c: char| !c.is_ascii_alphanumeric() && c != '_') {
        if !word.is_empty() && FORBIDDEN_WORDS.contains(&word.to_lowercase().as_str()) {
            return Err(StoreError::Rejected(
                "write statements are not allowed; this tool is read-only",
            ));
        }
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
    /// Queries must be self-contained: no bound parameters, no trailing
    /// semicolons. A hard row cap keeps a careless `SELECT` from eating
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
        assert!(validate_readonly_sql("SELECT COUNT(*) FROM traders WHERE city LIKE '%;%'").is_ok());
        assert!(validate_readonly_sql("SELECT group_concat(slug, '; ') FROM traders").is_ok());
        assert!(validate_readonly_sql("SELECT 'a;b'; SELECT 2").is_err());
        assert!(validate_readonly_sql("WITH x AS (SELECT 1) UPDATE prices SET price=1.0").is_err());
        assert!(validate_readonly_sql("PRAGMA table_info(traders)").is_err());
        assert!(validate_readonly_sql("EXPLAIN SELECT 1").is_err());
        assert!(validate_readonly_sql("VACUUM").is_err());
    }

    #[test]
    fn sql_tool_round_trip_with_truncation() {
        use super::PublicDb;
        let dir = std::env::temp_dir().join(format!("schrott-sql-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let db = PublicDb::open(&dir).expect("test db opens");
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
        let dir = std::env::temp_dir().join(format!("schrott-legacy-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("dir");
        // Simulate an old database file with the demo corpus still inside.
        {
            let conn = rusqlite::Connection::open(dir.join("public.db")).expect("old db");
            conn.execute_batch(
                "CREATE TABLE sources (slug TEXT PRIMARY KEY, name TEXT NOT NULL, url TEXT NOT NULL, description TEXT NOT NULL DEFAULT '');
                 CREATE TABLE datasets (slug TEXT PRIMARY KEY, source_slug TEXT NOT NULL, name TEXT NOT NULL, description TEXT NOT NULL DEFAULT '');
                 CREATE TABLE items (id INTEGER PRIMARY KEY AUTOINCREMENT, dataset_slug TEXT NOT NULL, external_id TEXT NOT NULL);",
            )
            .expect("legacy schema");
        }
        let db = PublicDb::open(&dir).expect("open migrates");
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
