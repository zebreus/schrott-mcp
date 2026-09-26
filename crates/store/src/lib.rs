//! Two SQLite databases: private internal state vs. the public queriable set.

pub mod error;
pub mod internal;
pub mod public;

pub use error::StoreError;
pub use internal::{FetchRecord, InternalDb};
pub use public::{PublicDb, SqlColumn, SqlResult};
