//! The pipeline: one run fans out to every scraper, hashes each item and
//! only touches the public database when content actually changed.

use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use offsite_data_auth::sha256_hex;
use offsite_data_store::{InternalDb, PublicDb};
use tokio::task::JoinHandle;

use super::scrapers::scrape_all;
use super::{LlmChangeChecker, NoopChecker};

/// Metadata for every bundled dataset. The pipeline upserts this first so
/// the catalog exists even before the first successful fetch.
const CATALOG: &[(&str, &str, &str, &str, &str, &str, &str)] = &[
    (
        "hn",
        "Hacker News",
        "https://news.ycombinator.com",
        "Community-curated tech stories.",
        "hn-front-page",
        "Front Page",
        "Current Hacker News front page via the Algolia search API.",
    ),
    (
        "rust",
        "Rust Project",
        "https://www.rust-lang.org",
        "The Rust programming language project.",
        "rust-releases",
        "Releases",
        "rust-lang/rust releases via the GitHub REST API.",
    ),
    (
        "example",
        "Example Domain",
        "https://example.com",
        "A tiny static page, parsed as HTML with CSS selectors.",
        "example-html",
        "Snapshot",
        "Heading and paragraphs of example.com as a single record.",
    ),
];

/// Outcome of one pipeline run.
#[derive(Debug, Clone, Default)]
pub struct IngestSummary {
    /// Total items written across all scrapers.
    pub upserted: i64,
    /// Items whose write failed (should stay zero; surfaced, not swallowed).
    pub write_failures: i64,
    /// Scrapers that failed, with reasons.
    pub failed: Vec<String>,
}

/// Guard so the scheduler and manual triggers never run concurrently.
static RUNNING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Make sure every source/dataset exists in the public catalog.
pub fn seed_metadata(public: &PublicDb) -> Result<(), super::IngestError> {
    use super::IngestError;
    for (slug, name, url, desc, ds, ds_name, ds_desc) in CATALOG {
        public
            .upsert_source(slug, name, url, desc)
            .map_err(|source| IngestError::Catalog {
                what: "source",
                name: (*slug).to_owned(),
                source,
            })?;
        public
            .upsert_dataset(ds, slug, ds_name, ds_desc)
            .map_err(|source| IngestError::Catalog {
                what: "dataset",
                name: (*ds).to_owned(),
                source,
            })?;
    }
    Ok(())
}

/// Source slug owning a dataset slug, derived from the catalog so new
/// datasets only need one edit.
fn source_of(dataset: &str) -> &'static str {
    for (source, _, _, _, ds, _, _) in CATALOG {
        if *ds == dataset {
            return source;
        }
    }
    "example"
}

/// Stable hash over everything queriable about an item.
fn item_hash(item: &super::scrapers::RawItem) -> String {
    sha256_hex(&format!(
        "{}|{}|{}|{}|{}|{}|{}",
        item.dataset,
        item.external_id,
        item.title,
        item.url,
        item.summary,
        item.data,
        item.unstructured_text
    ))
}

/// Run the whole pipeline once: fetch, diff, upsert, journal.
///
/// The hash decides *whether* something changed; the [`LlmChangeChecker`]
/// hook (currently [`NoopChecker`]) gets the final say on unstructured
/// text before paying for a write — the seam where a future LLM judge plugs
/// in without touching the data path.
pub async fn run_once(
    internal: &InternalDb,
    public: &PublicDb,
    client: &reqwest::Client,
) -> IngestSummary {
    use std::sync::atomic::Ordering;
    if RUNNING.swap(true, Ordering::SeqCst) {
        tracing::warn!("ingestion: previous run still active, skipping overlap");
        return IngestSummary {
            failed: vec!["previous run still active".to_owned()],
            ..IngestSummary::default()
        };
    }
    let summary = run_once_inner(internal, public, client).await;
    RUNNING.store(false, Ordering::SeqCst);
    summary
}

async fn run_once_inner(
    internal: &InternalDb,
    public: &PublicDb,
    client: &reqwest::Client,
) -> IngestSummary {
    let started = Utc::now().to_rfc3339();
    let run_id = match internal.create_run(&started) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("ingestion: cannot open run: {e}");
            return IngestSummary::default();
        }
    };
    if let Err(e) = seed_metadata(public) {
        tracing::warn!("ingestion: seeding catalog failed: {e}");
    }

    let mut summary = IngestSummary::default();
    for (slug, outcome) in scrape_all(client).await {
        let step_at = Utc::now().to_rfc3339();
        let step_id = internal.create_step(run_id, slug, &step_at).unwrap_or(0);
        match outcome {
            Err(e) => {
                tracing::warn!("ingestion: scraper {slug} failed: {e}");
                let detail = format!("{e}");
                summary.failed.push(format!("{slug}: {detail}"));
                if step_id != 0 {
                    if let Err(e) = internal.finish_step(
                        step_id,
                        "failed",
                        0,
                        &detail,
                        &Utc::now().to_rfc3339(),
                    ) {
                        tracing::warn!("ingestion: cannot close failed step: {e}");
                    }
                }
            }
            Ok(out) => {
                let now = Utc::now().to_rfc3339();
                let fetch_hash = sha256_hex(&format!("{}:{}", out.fetch_url, out.byte_len));
                if let Err(e) = internal.log_fetch(&offsite_data_store::FetchRecord {
                    run_id,
                    scraper: slug,
                    url: &out.fetch_url,
                    status_code: i64::from(out.status_code),
                    content_hash: &fetch_hash,
                    byte_len: out.byte_len as i64,
                    fetched_at: &now,
                }) {
                    tracing::warn!("ingestion: fetch journal failed for {slug}: {e}");
                }
                let checker = NoopChecker;
                let mut wrote = 0i64;
                for item in &out.items {
                    let hash = item_hash(item);
                    // Hash first; the checker judges the unstructured text.
                    let changed = match public.existing_item_meta(item.dataset, &item.external_id) {
                        Ok(Some((old_hash, old_summary))) => {
                            old_hash != hash
                                && checker.unstructured_changed(&old_summary, &item.summary)
                        }
                        Ok(None) => true,
                        Err(e) => {
                            tracing::warn!("ingestion: meta lookup failed: {e}");
                            true
                        }
                    };
                    if !changed {
                        continue;
                    }
                    let data_json = item.data.to_string();
                    match public.upsert_item(
                        item.dataset,
                        source_of(item.dataset),
                        &item.external_id,
                        &item.title,
                        &item.url,
                        &item.published_at,
                        &item.summary,
                        &hash,
                        &data_json,
                        &now,
                    ) {
                        Ok(()) => wrote += 1,
                        Err(e) => {
                            tracing::warn!(
                                "ingestion: upsert failed for {}:{}: {e}",
                                item.dataset,
                                item.external_id
                            );
                            summary.write_failures += 1;
                        }
                    }
                }
                summary.upserted += wrote;
                let msg = format!("fetched {} items, upserted {wrote}", out.items.len());
                if step_id != 0 {
                    if let Err(e) =
                        internal.finish_step(step_id, "ok", wrote, &msg, &Utc::now().to_rfc3339())
                    {
                        tracing::warn!("ingestion: cannot close step: {e}");
                    }
                }
            }
        }
    }

    let status = if summary.write_failures > 0 || !summary.failed.is_empty() {
        "partial"
    } else {
        "ok"
    };
    let detail = format!(
        "upserted {} items, {} writes failed, {} scrapers failed",
        summary.upserted,
        summary.write_failures,
        summary.failed.len()
    );
    if let Err(e) = internal.finish_run(run_id, status, &detail, &Utc::now().to_rfc3339()) {
        tracing::warn!("ingestion: cannot close run {run_id}: {e}");
    }
    tracing::info!("ingestion run {run_id} finished: {detail}");
    summary
}

/// Background scheduler: first run after a short delay, then periodically.
/// Runs inside the same process as the web + MCP server.
pub fn spawn_scheduler(
    internal: Arc<InternalDb>,
    public: Arc<PublicDb>,
    interval_secs: u64,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let client = reqwest::Client::builder()
            .user_agent("offsite-data-ingestion/0.1")
            .timeout(Duration::from_secs(30))
            .build()
            .expect("ingestion http client builds");
        // Let the server finish booting before the first run.
        tokio::time::sleep(Duration::from_secs(15)).await;
        loop {
            run_once(&internal, &public, &client).await;
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
        }
    })
}
