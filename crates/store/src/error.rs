//! Store error type shared by both databases.

use thiserror::Error;

/// Errors the store layer can return.
#[derive(Debug, Error)]
pub enum StoreError {
    /// Underlying SQLite failure.
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
    /// The database mutex was poisoned by a panicking thread.
    #[error("database lock poisoned")]
    Lock,
    /// Insert would violate a uniqueness rule (e.g. username taken).
    #[error("already exists: {0}")]
    Exists(&'static str),
    /// Ad-hoc SQL that is not a single read-only SELECT.
    #[error("rejected SQL: {0}")]
    Rejected(&'static str),
}
