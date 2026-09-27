//! Account tables: users, browser sessions and personal API tokens.

use rusqlite::{params, OptionalExtension};

use super::InternalDb;
use crate::error::StoreError;

pub(super) const SCHEMA: &str = "
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
CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_api_tokens_user ON api_tokens(user_id);
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

impl InternalDb {
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
}
