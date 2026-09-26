//! Private database (`internal.db`): users, sessions, tokens, OAuth state,
//! ingestion bookkeeping and the raw fetch log. Never exposed via MCP.

use std::sync::Mutex;

use rusqlite::{params, OptionalExtension};

use super::error::StoreError;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    professional INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS sessions (
    token TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS api_tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    prefix TEXT NOT NULL,
    hash TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    last_used_at TEXT
);
CREATE TABLE IF NOT EXISTS oauth_clients (
    client_id TEXT PRIMARY KEY,
    redirect_uris TEXT NOT NULL,
    created_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS oauth_codes (
    code TEXT PRIMARY KEY,
    client_id TEXT NOT NULL,
    redirect_uri TEXT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    code_challenge TEXT NOT NULL,
    code_challenge_method TEXT NOT NULL,
    scope TEXT NOT NULL DEFAULT 'read',
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS oauth_access_tokens (
    token TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    client_id TEXT NOT NULL,
    scope TEXT NOT NULL DEFAULT 'read',
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS oauth_refresh_tokens (
    token TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    client_id TEXT NOT NULL,
    scope TEXT NOT NULL DEFAULT 'read',
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS ingestion_runs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at TEXT NOT NULL,
    finished_at TEXT,
    status TEXT NOT NULL DEFAULT 'running',
    detail TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS ingestion_steps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id INTEGER NOT NULL REFERENCES ingestion_runs(id) ON DELETE CASCADE,
    scraper TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'running',
    items_upserted INTEGER NOT NULL DEFAULT 0,
    message TEXT NOT NULL DEFAULT '',
    started_at TEXT NOT NULL,
    finished_at TEXT
);
CREATE TABLE IF NOT EXISTS raw_fetches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id INTEGER NOT NULL REFERENCES ingestion_runs(id) ON DELETE CASCADE,
    scraper TEXT NOT NULL,
    url TEXT NOT NULL,
    status_code INTEGER NOT NULL DEFAULT 0,
    content_hash TEXT NOT NULL DEFAULT '',
    byte_len INTEGER NOT NULL DEFAULT 0,
    fetched_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS result_blobs (
    id TEXT PRIMARY KEY,
    payload TEXT NOT NULL,
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_api_tokens_user ON api_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_steps_run ON ingestion_steps(run_id);
";

/// Row types returned to callers.
#[derive(Debug, Clone)]
pub struct UserRow {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub professional: bool,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct SessionRow {
    pub user_id: i64,
    pub expires_at: String,
}

#[derive(Debug, Clone)]
pub struct ApiTokenView {
    pub id: i64,
    pub name: String,
    pub prefix: String,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ApiTokenSecret {
    pub id: i64,
    pub user_id: i64,
    pub hash: String,
}

#[derive(Debug, Clone)]
pub struct OAuthClientRow {
    pub client_id: String,
    pub redirect_uris: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OAuthCodeRow {
    pub code: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub user_id: i64,
    pub code_challenge: String,
    pub code_challenge_method: String,
    pub scope: String,
    pub expires_at: String,
}

#[derive(Debug, Clone)]
pub struct AccessTokenRow {
    pub user_id: i64,
    pub client_id: String,
    pub expires_at: String,
}

#[derive(Debug, Clone)]
pub struct RefreshTokenRow {
    pub user_id: i64,
    pub client_id: String,
    pub scope: String,
    pub expires_at: String,
}

#[derive(Debug, Clone)]
pub struct RunRow {
    pub id: i64,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub status: String,
    pub detail: String,
}

/// Private internal database handle.
pub struct InternalDb {
    conn: Mutex<rusqlite::Connection>,
}

/// One entry for the raw fetch journal.
#[derive(Debug, Clone)]
pub struct FetchRecord<'a> {
    pub run_id: i64,
    pub scraper: &'a str,
    pub url: &'a str,
    pub status_code: i64,
    pub content_hash: &'a str,
    pub byte_len: i64,
    pub fetched_at: &'a str,
}

impl InternalDb {
    /// Open (creating parent dirs and schema) the internal database.
    pub fn open(data_dir: &std::path::Path) -> Result<Self, StoreError> {
        std::fs::create_dir_all(data_dir).map_err(|e| {
            StoreError::Db(rusqlite::Error::InvalidPath(data_dir.join(e.to_string())))
        })?;
        let conn = rusqlite::Connection::open(data_dir.join("internal.db"))?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, rusqlite::Connection>, StoreError> {
        self.conn.lock().map_err(|_| StoreError::Lock)
    }

    // -- users ----------------------------------------------------------

    /// Insert a user; fails with `Exists` when the username is taken.
    pub fn create_user(
        &self,
        username: &str,
        password_hash: &str,
        professional: bool,
        now: &str,
    ) -> Result<i64, StoreError> {
        let conn = self.lock()?;
        let taken: Option<i64> = conn
            .query_row(
                "SELECT id FROM users WHERE username = ?1",
                params![username],
                |r| r.get(0),
            )
            .optional()?;
        if taken.is_some() {
            return Err(StoreError::Exists("username"));
        }
        conn.execute(
            "INSERT INTO users (username, password_hash, professional, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![username, password_hash, i64::from(professional), now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Look a user up by username.
    pub fn find_user_by_username(&self, username: &str) -> Result<Option<UserRow>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT id, username, password_hash, professional, created_at FROM users WHERE username = ?1",
            params![username],
            |r| {
                Ok(UserRow {
                    id: r.get(0)?,
                    username: r.get(1)?,
                    password_hash: r.get(2)?,
                    professional: r.get::<_, i64>(3)? != 0,
                    created_at: r.get(4)?,
                })
            },
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Look a user up by id.
    pub fn find_user_by_id(&self, id: i64) -> Result<Option<UserRow>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT id, username, password_hash, professional, created_at FROM users WHERE id = ?1",
            params![id],
            |r| {
                Ok(UserRow {
                    id: r.get(0)?,
                    username: r.get(1)?,
                    password_hash: r.get(2)?,
                    professional: r.get::<_, i64>(3)? != 0,
                    created_at: r.get(4)?,
                })
            },
        )
        .optional()
        .map_err(StoreError::from)
    }

    // -- sessions -------------------------------------------------------

    /// Store a browser session token.
    pub fn create_session(
        &self,
        token: &str,
        user_id: i64,
        now: &str,
        expires_at: &str,
    ) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO sessions (token, user_id, created_at, expires_at) VALUES (?1, ?2, ?3, ?4)",
            params![token, user_id, now, expires_at],
        )?;
        Ok(())
    }

    /// Resolve a browser session token.
    pub fn find_session(&self, token: &str) -> Result<Option<SessionRow>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT user_id, expires_at FROM sessions WHERE token = ?1",
            params![token],
            |r| {
                Ok(SessionRow {
                    user_id: r.get(0)?,
                    expires_at: r.get(1)?,
                })
            },
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Drop a browser session token.
    pub fn delete_session(&self, token: &str) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute("DELETE FROM sessions WHERE token = ?1", params![token])?;
        Ok(())
    }

    // -- API tokens -----------------------------------------------------

    /// Store a personal access token (only its hash).
    pub fn create_api_token(
        &self,
        user_id: i64,
        name: &str,
        prefix: &str,
        hash: &str,
        now: &str,
    ) -> Result<i64, StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO api_tokens (user_id, name, prefix, hash, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![user_id, name, prefix, hash, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// List a user's tokens (metadata only, never secrets).
    pub fn list_api_tokens(&self, user_id: i64) -> Result<Vec<ApiTokenView>, StoreError> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, prefix, created_at, last_used_at FROM api_tokens
             WHERE user_id = ?1 ORDER BY id DESC",
        )?;
        let rows = stmt.query_map(params![user_id], |r| {
            Ok(ApiTokenView {
                id: r.get(0)?,
                name: r.get(1)?,
                prefix: r.get(2)?,
                created_at: r.get(3)?,
                last_used_at: r.get(4)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(StoreError::from)
    }

    /// Look a token up by its hash.
    pub fn find_api_token_by_hash(&self, hash: &str) -> Result<Option<ApiTokenSecret>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT id, user_id, hash FROM api_tokens WHERE hash = ?1",
            params![hash],
            |r| {
                Ok(ApiTokenSecret {
                    id: r.get(0)?,
                    user_id: r.get(1)?,
                    hash: r.get(2)?,
                })
            },
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Refresh a token's last-used timestamp.
    pub fn touch_api_token(&self, id: i64, now: &str) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "UPDATE api_tokens SET last_used_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    /// Delete one of a user's tokens.
    pub fn delete_api_token(&self, id: i64, user_id: i64) -> Result<bool, StoreError> {
        let conn = self.lock()?;
        let n = conn.execute(
            "DELETE FROM api_tokens WHERE id = ?1 AND user_id = ?2",
            params![id, user_id],
        )?;
        Ok(n == 1)
    }

    // -- OAuth ----------------------------------------------------------

    /// Register (or re-register) a dynamic OAuth client.
    pub fn upsert_oauth_client(
        &self,
        client_id: &str,
        redirect_uris: &[String],
        now: &str,
    ) -> Result<(), StoreError> {
        let conn = self.lock()?;
        let uris = serde_json::to_string(redirect_uris).unwrap_or_else(|_| "[]".to_owned());
        conn.execute(
            "INSERT INTO oauth_clients (client_id, redirect_uris, created_at)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(client_id) DO UPDATE SET redirect_uris = excluded.redirect_uris",
            params![client_id, uris, now],
        )?;
        Ok(())
    }

    /// Look an OAuth client up.
    pub fn find_oauth_client(&self, client_id: &str) -> Result<Option<OAuthClientRow>, StoreError> {
        let conn = self.lock()?;
        let row: Option<(String, String)> = conn
            .query_row(
                "SELECT client_id, redirect_uris FROM oauth_clients WHERE client_id = ?1",
                params![client_id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        Ok(row.map(|(client_id, uris)| OAuthClientRow {
            client_id,
            redirect_uris: serde_json::from_str(&uris).unwrap_or_default(),
        }))
    }

    /// Store an authorization code.
    #[allow(clippy::too_many_arguments)]
    pub fn create_oauth_code(
        &self,
        code: &str,
        client_id: &str,
        redirect_uri: &str,
        user_id: i64,
        challenge: &str,
        method: &str,
        scope: &str,
        now: &str,
        expires_at: &str,
    ) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO oauth_codes
             (code, client_id, redirect_uri, user_id, code_challenge,
              code_challenge_method, scope, created_at, expires_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                code,
                client_id,
                redirect_uri,
                user_id,
                challenge,
                method,
                scope,
                now,
                expires_at
            ],
        )?;
        Ok(())
    }

    /// Consume (select + delete) an authorization code.
    pub fn take_oauth_code(&self, code: &str) -> Result<Option<OAuthCodeRow>, StoreError> {
        let conn = self.lock()?;
        let row: Option<OAuthCodeRow> = conn
            .query_row(
                "SELECT code, client_id, redirect_uri, user_id, code_challenge,
                        code_challenge_method, scope, expires_at
                 FROM oauth_codes WHERE code = ?1",
                params![code],
                |r| {
                    Ok(OAuthCodeRow {
                        code: r.get(0)?,
                        client_id: r.get(1)?,
                        redirect_uri: r.get(2)?,
                        user_id: r.get(3)?,
                        code_challenge: r.get(4)?,
                        code_challenge_method: r.get(5)?,
                        scope: r.get(6)?,
                        expires_at: r.get(7)?,
                    })
                },
            )
            .optional()?;
        if row.is_some() {
            conn.execute("DELETE FROM oauth_codes WHERE code = ?1", params![code])?;
        }
        Ok(row)
    }

    /// Store a fresh access/refresh token pair.
    #[allow(clippy::too_many_arguments)]
    pub fn create_oauth_tokens(
        &self,
        access: &str,
        refresh: &str,
        user_id: i64,
        client_id: &str,
        scope: &str,
        now: &str,
        access_exp: &str,
        refresh_exp: &str,
    ) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO oauth_access_tokens
             (token, user_id, client_id, scope, created_at, expires_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![access, user_id, client_id, scope, now, access_exp],
        )?;
        conn.execute(
            "INSERT INTO oauth_refresh_tokens
             (token, user_id, client_id, scope, created_at, expires_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![refresh, user_id, client_id, scope, now, refresh_exp],
        )?;
        Ok(())
    }

    /// Look an access token up.
    pub fn find_access_token(&self, token: &str) -> Result<Option<AccessTokenRow>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT user_id, client_id, expires_at FROM oauth_access_tokens WHERE token = ?1",
            params![token],
            |r| {
                Ok(AccessTokenRow {
                    user_id: r.get(0)?,
                    client_id: r.get(1)?,
                    expires_at: r.get(2)?,
                })
            },
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Look a refresh token up.
    pub fn find_refresh_token(&self, token: &str) -> Result<Option<RefreshTokenRow>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT user_id, client_id, scope, expires_at FROM oauth_refresh_tokens WHERE token = ?1",
            params![token],
            |r| {
                Ok(RefreshTokenRow {
                    user_id: r.get(0)?,
                    client_id: r.get(1)?,
                    scope: r.get(2)?,
                    expires_at: r.get(3)?,
                })
            },
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Drop a refresh token (used when rotating).
    pub fn delete_refresh_token(&self, token: &str) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "DELETE FROM oauth_refresh_tokens WHERE token = ?1",
            params![token],
        )?;
        Ok(())
    }

    // -- ingestion bookkeeping ------------------------------------------

    /// Start a pipeline run, returning its id.
    pub fn create_run(&self, now: &str) -> Result<i64, StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO ingestion_runs (started_at) VALUES (?1)",
            params![now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Finish a pipeline run.
    pub fn finish_run(
        &self,
        id: i64,
        status: &str,
        detail: &str,
        now: &str,
    ) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "UPDATE ingestion_runs SET finished_at = ?1, status = ?2, detail = ?3 WHERE id = ?4",
            params![now, status, detail, id],
        )?;
        Ok(())
    }

    /// Start tracking one scraper inside a run.
    pub fn create_step(&self, run_id: i64, scraper: &str, now: &str) -> Result<i64, StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO ingestion_steps (run_id, scraper, started_at) VALUES (?1, ?2, ?3)",
            params![run_id, scraper, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Finish one scraper step.
    pub fn finish_step(
        &self,
        id: i64,
        status: &str,
        items_upserted: i64,
        message: &str,
        now: &str,
    ) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "UPDATE ingestion_steps
             SET finished_at = ?1, status = ?2, items_upserted = ?3, message = ?4
             WHERE id = ?5",
            params![now, status, items_upserted, message, id],
        )?;
        Ok(())
    }

    /// Append to the raw fetch log.
    pub fn log_fetch(&self, rec: &FetchRecord<'_>) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO raw_fetches
             (run_id, scraper, url, status_code, content_hash, byte_len, fetched_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                rec.run_id,
                rec.scraper,
                rec.url,
                rec.status_code,
                rec.content_hash,
                rec.byte_len,
                rec.fetched_at
            ],
        )?;
        Ok(())
    }

    /// Store a downloadable result blob under its secret id, pruning
    /// expired blobs along the way.
    pub fn create_result_blob(
        &self,
        id: &str,
        payload: &str,
        now: &str,
        expires_at: &str,
    ) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "DELETE FROM result_blobs WHERE expires_at <= ?1",
            params![now],
        )?;
        conn.execute(
            "INSERT INTO result_blobs (id, payload, created_at, expires_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![id, payload, now, expires_at],
        )?;
        Ok(())
    }

    /// Fetch a blob's payload and expiry, if it exists.
    pub fn find_result_blob(&self, id: &str) -> Result<Option<(String, String)>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT payload, expires_at FROM result_blobs WHERE id = ?1",
            params![id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Drop a blob (used once expired).
    pub fn delete_result_blob(&self, id: &str) -> Result<(), StoreError> {
        let conn = self.lock()?;
        conn.execute("DELETE FROM result_blobs WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Most recent pipeline runs (newest first).
    pub fn last_runs(&self, limit: i64) -> Result<Vec<RunRow>, StoreError> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, started_at, finished_at, status, detail FROM ingestion_runs
             ORDER BY id DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |r| {
            Ok(RunRow {
                id: r.get(0)?,
                started_at: r.get(1)?,
                finished_at: r.get(2)?,
                status: r.get(3)?,
                detail: r.get(4)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(StoreError::from)
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
}
