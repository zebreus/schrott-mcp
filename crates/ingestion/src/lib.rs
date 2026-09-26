//! Minimal ingestion pipeline: `fetch -> parse -> normalize -> diff -> upsert`.
//!
//! Each scraper fetches one wildly different website/API and normalizes it
//! into [`RawItem`]s. The pipeline hashes every item and only writes to the
//! public database when something actually changed. A [`LlmChangeChecker`]
//! hook is reserved for later: an LLM judging whether the *unstructured*
//! part of a page meaningfully changed (never for the data itself).

pub mod pipeline;
pub mod scrapers;

pub use pipeline::{run_once, seed_metadata, spawn_scheduler, IngestSummary};
pub use scrapers::{scrape_all, RawItem};

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
        source: offsite_data_store::StoreError,
    },
}

/// Optional LLM-backed judge for unstructured change detection.
///
/// The default [`NoopChecker`] treats the content hash as the whole truth.
/// A future implementation can call an LLM here to decide whether new text
/// *means* something different before paying for a full re-ingest.
pub trait LlmChangeChecker: Send + Sync {
    /// Decide whether new unstructured text differs meaningfully from old.
    fn unstructured_changed(&self, old_text: &str, new_text: &str) -> bool;
}

/// Default checker: any byte-level change counts.
pub struct NoopChecker;

impl LlmChangeChecker for NoopChecker {
    fn unstructured_changed(&self, old_text: &str, new_text: &str) -> bool {
        old_text != new_text
    }
}
