//! Dev tool: run trader handlers live without touching any database.
//! Usage: cargo run -p schrott-mcp-ingestion --example live_handlers [slug]
//! Prints parsed prices per handler — the fastest way to check a handler
//! against the real page while developing it.

use schrott_mcp_ingestion::traders::handlers;

#[tokio::main]
async fn main() {
    let want = std::env::args().nth(1);
    let client = reqwest::Client::builder()
        .user_agent("schrott-mcp-ingestion/0.1")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("client");
    for h in handlers::all() {
        if let Some(ref w) = want {
            if h.slug != w {
                continue;
            }
        }
        println!("=== {} ===", h.slug);
        match (h.scrape)(&client).await {
            Err(e) => println!("FAILED: {e}"),
            Ok(out) => {
                println!(
                    "status={} bytes={} published_at={:?} acceptances={} alive={} info={:?}",
                    out.status_code,
                    out.byte_len,
                    out.published_at,
                    out.acceptances.len(),
                    out.website_alive,
                    out.trader_info,
                );
                for p in &out.prices {
                    println!(
                        "  {:22} {:>8.3} {:7} conf={:?} [{}]",
                        p.material,
                        p.price,
                        p.unit,
                        p.confidence,
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
}
