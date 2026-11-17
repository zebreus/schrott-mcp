//! Probe: MKM live scrapen und in eine *Kopie* der Produktionsdatenbank
//! aufzeichnen. Die Produktion wird nicht angefasst.
//!
//! Aufruf: `cargo run --release --example mkm_probe -- <ziel-verzeichnis>`

use chrono::Utc;
use schrott_mcp_ingestion::traders::handlers::mkm_metals;
use schrott_mcp_store::PublicDb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = std::env::args().nth(1).expect("Ziel-Verzeichnis angeben");
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) Chrome/120")
        .build()?;
    let snapshot = mkm_metals::scrape_articles(&client).await?;
    println!(
        "live gescrapt: {} Preise, {} Annahmen, {} Artikel",
        snapshot.outcome.prices.len(),
        snapshot.outcome.acceptances.len(),
        snapshot.articles.len()
    );
    let public = PublicDb::open(std::path::Path::new(&target))?;
    let summary = mkm_metals::record_articles(&public, &snapshot, &Utc::now())?;
    println!(
        "aufgezeichnet: {} Preise, {} Annahmen, {} übersprungen, {} Canaries",
        summary.recorded,
        summary.accepted,
        summary.skipped.len(),
        summary.canaries.len()
    );
    Ok(())
}
