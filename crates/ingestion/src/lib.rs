//! Ingestion pipeline: seed the catalog, then run due trader handlers.
//!
//! Five trader price handlers are live (see `traders/handlers/`); the rest
//! of the 2.300 seeded traders get handlers one file at a time.

pub mod pipeline;
pub mod seed_traders;
pub mod traders;

pub use pipeline::{run_once, seed_metadata, spawn_scheduler, IngestSummary};
pub use seed_traders::{load_seeds, seed_traders, validate_seeds};

/// Every way ingestion can fail. Carries the scraper and URL for context
/// instead of pre-formatted strings, so callers decide how to render.
#[derive(Debug, thiserror::Error)]
pub enum IngestError {
    /// The HTTP request itself failed.
    #[error("request to {url} failed: {source}")]
    Fetch {
        url: String,
        #[source]
        source: reqwest::Error,
    },
    /// The response body could not be read.
    #[error("reading response from {url} failed: {source}")]
    Body {
        url: String,
        #[source]
        source: reqwest::Error,
    },
    /// The body was not the expected shape.
    #[error("response from {url} was not usable: {detail}")]
    Parse { url: String, detail: String },
    /// A catalog write failed.
    #[error("catalog update for {what} '{name}' failed: {source}")]
    Catalog {
        what: &'static str,
        name: String,
        #[source]
        source: schrott_mcp_store::StoreError,
    },
}
