//! Public database (`public.db`): the entire queriable data set.
//!
//! Everything here is exposed to all MCP users — nothing is user specific.

use std::sync::Mutex;

use offsite_data_core::Stats;
use rusqlite::{params, OptionalExtension as _};

use super::error::StoreError;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS sources (
    slug TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS datasets (
    slug TEXT PRIMARY KEY,
    source_slug TEXT NOT NULL REFERENCES sources(slug),
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dataset_slug TEXT NOT NULL REFERENCES datasets(slug),
    source_slug TEXT NOT NULL REFERENCES sources(slug),
    external_id TEXT NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL DEFAULT '',
    published_at TEXT NOT NULL DEFAULT '',
    summary TEXT NOT NULL DEFAULT '',
    content_hash TEXT NOT NULL DEFAULT '',
    data_json TEXT NOT NULL DEFAULT '{}',
    fetched_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(dataset_slug, external_id)
);
CREATE INDEX IF NOT EXISTS idx_items_dataset ON items(dataset_slug);
CREATE INDEX IF NOT EXISTS idx_items_source ON items(source_slug);
";

/// One result column: name plus SQLite type.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SqlColumn {
    pub name: String,
    /// Value type inferred from the first non-null value
    /// (`integer`, `real`, `text`, or `null` when the column is all null).
    #[serde(rename = "type")]
    pub dtype: String,
}

/// A read-only query result: typed columns plus JSON rows.
#[derive(Debug, Clone, serde::Serialize)]
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
    if sql.contains(';') {
        return Err(StoreError::Rejected(
            "multiple statements are not allowed; send one SELECT at a time",
        ));
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
    /// Open (creating parent dirs and schema) the public database.
    pub fn open(data_dir: &std::path::Path) -> Result<Self, StoreError> {
        std::fs::create_dir_all(data_dir).map_err(|e| {
            StoreError::Db(rusqlite::Error::InvalidPath(data_dir.join(e.to_string())))
        })?;
        let conn = rusqlite::Connection::open(data_dir.join("public.db"))?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, rusqlite::Connection>, StoreError> {
        self.conn.lock().map_err(|_| StoreError::Lock)
    }

    /// Insert or update a source's metadata.
    pub fn upsert_source(
        &self,
        slug: &str,
        name: &str,
        url: &str,
        description: &str,
    ) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO sources (slug, name, url, description) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(slug) DO UPDATE SET name = excluded.name, url = excluded.url,
             description = excluded.description",
            params![slug, name, url, description],
        )?;
        Ok(())
    }

    /// Insert or update a dataset's metadata.
    pub fn upsert_dataset(
        &self,
        slug: &str,
        source_slug: &str,
        name: &str,
        description: &str,
    ) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO datasets (slug, source_slug, name, description)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(slug) DO UPDATE SET source_slug = excluded.source_slug,
             name = excluded.name, description = excluded.description",
            params![slug, source_slug, name, description],
        )?;
        Ok(())
    }

    /// Current content hash and summary for an item, if it exists.
    /// Used by the ingestion diff (and the LLM change-check hook).
    pub fn existing_item_meta(
        &self,
        dataset_slug: &str,
        external_id: &str,
    ) -> Result<Option<(String, String)>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT content_hash, summary FROM items WHERE dataset_slug = ?1 AND external_id = ?2",
            params![dataset_slug, external_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Insert a new item or refresh a changed one.
    #[allow(clippy::too_many_arguments)]
    pub fn upsert_item(
        &self,
        dataset_slug: &str,
        source_slug: &str,
        external_id: &str,
        title: &str,
        url: &str,
        published_at: &str,
        summary: &str,
        content_hash: &str,
        data_json: &str,
        now: &str,
    ) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO items
             (dataset_slug, source_slug, external_id, title, url, published_at,
              summary, content_hash, data_json, fetched_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?10)
             ON CONFLICT(dataset_slug, external_id) DO UPDATE SET
              title = excluded.title, url = excluded.url,
              published_at = excluded.published_at, summary = excluded.summary,
              content_hash = excluded.content_hash, data_json = excluded.data_json,
              updated_at = excluded.updated_at",
            params![
                dataset_slug,
                source_slug,
                external_id,
                title,
                url,
                published_at,
                summary,
                content_hash,
                data_json,
                now
            ],
        )?;
        Ok(())
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
        let sources: i64 = conn.query_row("SELECT COUNT(*) FROM sources", [], |r| r.get(0))?;
        let datasets: i64 = conn.query_row("SELECT COUNT(*) FROM datasets", [], |r| r.get(0))?;
        let items: i64 = conn.query_row("SELECT COUNT(*) FROM items", [], |r| r.get(0))?;
        Ok(Stats {
            sources,
            datasets,
            items,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn readonly_sql_accepts_plain_selects() {
        use super::validate_readonly_sql;
        assert!(validate_readonly_sql("SELECT 1").is_ok());
        assert!(validate_readonly_sql("  -- a comment\nSELECT id, title FROM items").is_ok());
        assert!(validate_readonly_sql("/* c */ WITH x AS (SELECT 1) SELECT * FROM x").is_ok());
        // Prose mentioning forbidden words inside literals is fine.
        assert!(
            validate_readonly_sql("SELECT * FROM items WHERE summary LIKE '%update dropped%'")
                .is_ok()
        );
    }

    #[test]
    fn readonly_sql_rejects_writes_and_stacks() {
        use super::validate_readonly_sql;
        assert!(validate_readonly_sql("").is_err());
        assert!(validate_readonly_sql("DROP TABLE items").is_err());
        assert!(validate_readonly_sql("SELECT 1; DELETE FROM items").is_err());
        assert!(validate_readonly_sql("SELECT 1;").is_err());
        assert!(validate_readonly_sql("WITH x AS (SELECT 1) UPDATE items SET title='h'").is_err());
        assert!(validate_readonly_sql("PRAGMA table_info(items)").is_err());
        assert!(validate_readonly_sql("EXPLAIN SELECT 1").is_err());
        assert!(validate_readonly_sql("VACUUM").is_err());
    }

    #[test]
    fn sql_tool_round_trip_with_truncation() {
        use super::PublicDb;
        let dir = std::env::temp_dir().join(format!("offsite-sql-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let db = PublicDb::open(&dir).expect("test db opens");
        db.upsert_source("s", "S", "https://s.test", "")
            .expect("source");
        db.upsert_dataset("d", "s", "D", "").expect("dataset");
        for i in 0..3 {
            db.upsert_item(
                "d",
                "s",
                &format!("e{i}"),
                &format!("T{i}"),
                "",
                "",
                "",
                &format!("h{i}"),
                "{}",
                "2026-01-01T00:00:00Z",
            )
            .expect("item");
        }
        let r = db
            .query_sql("SELECT id, title FROM items ORDER BY id")
            .expect("select works");
        let names: Vec<&str> = r.columns.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, vec!["id", "title"]);
        assert_eq!(r.columns[0].dtype, "integer");
        assert_eq!(r.columns[1].dtype, "text");
        assert_eq!(r.rows.len(), 3);
        // Declared type wins; expression columns infer from values.
        let r = db
            .query_sql("SELECT COUNT(*) AS n, 1.5 AS f FROM items")
            .expect("select works");
        assert_eq!(r.columns[0].dtype, "integer");
        assert_eq!(r.columns[1].dtype, "real");
        assert_eq!(r.rows.len(), 1);
        assert!(db.query_sql("DELETE FROM items").is_err());
    }
}
