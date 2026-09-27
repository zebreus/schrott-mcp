//! Ingestion pipeline: seed the material catalog, then run scrapers.
//!
//! The Händler scrapers that will fill traders/prices are not built yet —
//! runs currently only refresh the static catalog. The run/step/fetch
//! bookkeeping in the internal database already works, so scheduling,
//! dashboard and manual triggers behave the same before and after.

pub mod pipeline;
pub mod scrapers;

pub use pipeline::{run_once, seed_metadata, spawn_scheduler, IngestSummary};
pub use scrapers::scrape_all;

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
