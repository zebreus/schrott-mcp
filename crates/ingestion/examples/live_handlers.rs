//! Dev tool: run trader handlers live without touching any database.
//! Usage: cargo run -p schrott-mcp-ingestion --example live_handlers [slug]
//! Prints parsed prices per handler — the fastest way to check a handler
//! against the real page while developing it.
//! Exit 1 if no handler matches or any selected scrape fails; otherwise exit 0.

use schrott_mcp_ingestion::traders::handlers;
use schrott_mcp_ingestion::traders::Handler;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let want = std::env::args().nth(1);
    let client = reqwest::Client::builder()
        .user_agent("schrott-mcp-ingestion/0.1")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("client");
    run_handlers(&client, handlers::all(), want.as_deref()).await
}

async fn run_handlers(
    client: &reqwest::Client,
    handlers: Vec<Handler>,
    want: Option<&str>,
) -> ExitCode {
    let mut matched = false;
    let mut failed = false;
    for h in handlers {
        if let Some(w) = want {
            if h.slug != w {
                continue;
            }
        }
        matched = true;
        println!("=== {} ===", h.slug);
        match schrott_mcp_ingestion::traders::scrape_record_outcome(&h, client).await {
            Err(e) => {
                failed = true;
                eprintln!("FAILED: {e}");
            }
            Ok(record_outcome) => {
                let out = record_outcome.observation();
                println!(
                    "status={} bytes={} published_at={:?} acceptances={} alive={} info={:?}",
                    out.status_code,
                    out.byte_len,
                    out.published_at,
                    out.acceptances.len(),
                    out.website_alive,
                    out.trader_info,
                );
                for (index, p) in out.prices.iter().enumerate() {
                    println!(
                        "  {:22} {:>8.3} {:7} conf={:?} published_at={:?} [{}]",
                        p.material,
                        p.price,
                        p.unit,
                        p.confidence,
                        record_outcome.price_published_at(index),
                        p.label.chars().take(48).collect::<String>()
                    );
                }
                for a in &out.acceptances {
                    println!(
                        "  A {:22} cond={:?} [{}]",
                        a.material,
                        a.conditions,
                        a.label.chars().take(48).collect::<String>()
                    );
                }
                for s in &out.skipped_labels {
                    println!("  SKIP: {s}");
                }
            }
        }
    }
    if !matched {
        eprintln!("FAILED: no handler matched {:?}", want);
    }
    if matched && !failed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use schrott_mcp_ingestion::traders::{HandlerOutcome, Schedule};
    use schrott_mcp_ingestion::IngestError;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static SUCCESS_CALLS: AtomicUsize = AtomicUsize::new(0);

    fn success() -> Handler {
        Handler {
            slug: "success",
            url: "https://example.invalid/success",
            schedule: Schedule::every_6h(),
            scrape: |_| {
                Box::pin(async {
                    SUCCESS_CALLS.fetch_add(1, Ordering::SeqCst);
                    Ok(HandlerOutcome::default())
                })
            },
        }
    }

    fn failure() -> Handler {
        Handler {
            slug: "failure",
            url: "https://example.invalid/failure",
            schedule: Schedule::every_6h(),
            scrape: |_| {
                Box::pin(async {
                    Err(IngestError::Parse {
                        url: "https://example.invalid/failure".into(),
                        detail: "controlled scrape failure".into(),
                    })
                })
            },
        }
    }

    // One async test keeps the call counter deterministic. No HTTP or DB access.
    #[tokio::test]
    async fn exit_status_tracks_selected_scrapes_without_stopping_after_failure() {
        let client = reqwest::Client::new();
        assert_eq!(
            run_handlers(&client, vec![failure()], Some("failure")).await,
            ExitCode::FAILURE
        );
        assert_eq!(
            run_handlers(&client, vec![success()], Some("unknown")).await,
            ExitCode::FAILURE
        );
        assert_eq!(SUCCESS_CALLS.load(Ordering::SeqCst), 0);
        assert_eq!(run_handlers(&client, vec![], None).await, ExitCode::FAILURE);
        assert_eq!(
            run_handlers(&client, vec![failure(), success()], None).await,
            ExitCode::FAILURE
        );
        assert_eq!(SUCCESS_CALLS.load(Ordering::SeqCst), 1);
        assert_eq!(
            run_handlers(&client, vec![failure(), success()], Some("success")).await,
            ExitCode::SUCCESS
        );
        assert_eq!(SUCCESS_CALLS.load(Ordering::SeqCst), 2);
    }
}
