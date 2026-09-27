//! Sharing tables: download blobs and user feedback reports.

use rusqlite::{params, OptionalExtension};

use super::InternalDb;
use crate::error::StoreError;

pub(super) const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS result_blobs (
    id TEXT PRIMARY KEY,
    payload TEXT NOT NULL,
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS feedback (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    severity TEXT NOT NULL CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    feedback TEXT NOT NULL,
    details TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL
);
DROP TABLE IF EXISTS data_feedback;
";

impl InternalDb {
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

    /// Store one MCP user feedback report for later human review.
    pub fn create_feedback(
        &self,
        user_id: Option<i64>,
        severity: &str,
        feedback: &str,
        details: &str,
        now: &str,
    ) -> Result<i64, StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO feedback (user_id, severity, feedback, details, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![user_id, severity, feedback, details, now],
        )?;
        Ok(conn.last_insert_rowid())
    }
}
