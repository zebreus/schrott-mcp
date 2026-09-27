//! OAuth tables: dynamic clients, codes and issued token pairs.

use rusqlite::{params, OptionalExtension};

use super::InternalDb;
use crate::error::StoreError;

pub(super) const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS oauth_clients (
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
";

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

impl InternalDb {
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
}
