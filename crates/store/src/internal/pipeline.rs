//! Ingestion bookkeeping: runs, per-scraper steps and the raw fetch log.

use rusqlite::params;

use super::InternalDb;
use crate::error::StoreError;

pub(super) const SCHEMA: &str = "
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
CREATE INDEX IF NOT EXISTS idx_steps_run ON ingestion_steps(run_id);
";

#[derive(Debug, Clone)]
pub struct RunRow {
    pub id: i64,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub status: String,
    pub detail: String,
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
