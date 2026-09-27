//! Scraper slots for the Schrott price ingestion (not implemented yet).
//!
//! Each future scraper fetches one Händler website or portal and returns the
//! number of records it wrote via [`PublicDb`] (traders, prices, …).
//! Failures are returned per scraper so one bad website never stops the rest
//! of the pipeline. Each scrape is timed in the log so slow stages stay
//! attributable.

use super::IngestError;

/// Run every bundled scraper. Currently none — the Händler scrapers that
/// fill traders/prices are the next milestone after the data model.
pub async fn scrape_all() -> Vec<(&'static str, Result<i64, IngestError>)> {
    Vec::new()
}
