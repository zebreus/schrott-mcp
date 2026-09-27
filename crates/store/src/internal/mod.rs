//! Private database (`internal.db`): users, sessions, tokens, OAuth state,
//! ingestion bookkeeping and the raw fetch log. Never exposed via MCP.
//! Split by domain; each submodule owns its tables, row types and queries.

use std::sync::Mutex;

use super::error::StoreError;

pub mod oauth;
pub mod pipeline;
pub mod sharing;
pub mod users;

pub use oauth::{AccessTokenRow, OAuthClientRow, OAuthCodeRow, RefreshTokenRow};
pub use pipeline::{FetchRecord, RunRow};
pub use users::{ApiTokenSecret, ApiTokenView, SessionRow, UserRow};

/// Private internal database handle.
pub struct InternalDb {
    conn: Mutex<rusqlite::Connection>,
}

impl InternalDb {
    /// Open (creating parent dirs and schema) the internal database.
    pub fn open(data_dir: &std::path::Path) -> Result<Self, StoreError> {
        std::fs::create_dir_all(data_dir).map_err(|e| {
            StoreError::Db(rusqlite::Error::InvalidPath(data_dir.join(e.to_string())))
        })?;
        let conn = rusqlite::Connection::open(data_dir.join("internal.db"))?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        // Domain schemas, in dependency order (users first for FK targets).
        conn.execute_batch(users::SCHEMA)?;
        conn.execute_batch(oauth::SCHEMA)?;
        conn.execute_batch(pipeline::SCHEMA)?;
        conn.execute_batch(sharing::SCHEMA)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, rusqlite::Connection>, StoreError> {
        self.conn.lock().map_err(|_| StoreError::Lock)
    }
}

#[cfg(test)]
mod tests {
    use super::InternalDb;

    fn temp_db(name: &str) -> InternalDb {
        let dir =
            std::env::temp_dir().join(format!("offsite-test-{}-{}", name, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        InternalDb::open(&dir).expect("test db opens")
    }

    #[test]
    fn oauth_code_is_single_use() {
        let db = temp_db("single-use");
        let now = "2026-01-01T00:00:00Z";
        let far = "2027-01-01T00:00:00Z";
        db.upsert_oauth_client("cli_test", &["https://app.test/cb".to_owned()], now)
            .expect("client registers");
        let uid = db
            .create_user("codeuser", "hash", true, now)
            .expect("user created");
        db.create_oauth_code(
            "code_abc",
            "cli_test",
            "https://app.test/cb",
            uid,
            "challenge",
            "S256",
            "read",
            now,
            far,
        )
        .expect("code stored");
        assert!(db
            .take_oauth_code("code_abc")
            .expect("take works")
            .is_some());
        assert!(db
            .take_oauth_code("code_abc")
            .expect("take works")
            .is_none());
    }

    #[test]
    fn duplicate_username_is_rejected() {
        let db = temp_db("dup-user");
        let now = "2026-01-01T00:00:00Z";
        db.create_user("dup", "h1", true, now).expect("first ok");
        assert!(db.create_user("dup", "h2", true, now).is_err());
    }

    #[test]
    fn blobs_round_trip_and_expire() {
        let db = temp_db("blob");
        db.create_result_blob(
            "abc",
            "{\"a\":1}",
            "2026-01-01T00:00:00Z",
            "2026-01-08T00:00:00Z",
        )
        .expect("stored");
        let got = db.find_result_blob("abc").expect("lookup works");
        assert_eq!(got.map(|(p, _)| p).as_deref(), Some("{\"a\":1}"));
        assert!(db.find_result_blob("nope").expect("lookup works").is_none());
        // Storing prunes expired rows.
        db.create_result_blob(
            "fresh",
            "{}",
            "2026-02-01T00:00:00Z",
            "2026-02-08T00:00:00Z",
        )
        .expect("stored");
        assert!(db.find_result_blob("abc").expect("lookup works").is_none());
        assert!(db
            .find_result_blob("fresh")
            .expect("lookup works")
            .is_some());
    }
    #[test]
    fn feedback_stores_and_rejects_bad_severity() {
        let dir = std::env::temp_dir().join(format!("offsite-fb-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let db = InternalDb::open(&dir).expect("test db opens");
        let uid = db
            .create_user("reporter", "hash", true, "2026-01-01T00:00:00Z")
            .expect("user created");
        let id = db
            .create_feedback(
                Some(uid),
                "high",
                "stale price",
                "item 12",
                "2026-01-01T00:00:00Z",
            )
            .expect("valid feedback stores");
        assert_eq!(id, 1);
        assert!(db
            .create_feedback(None, "cosmic", "x", "", "2026-01-01T00:00:00Z")
            .is_err());
    }
}
