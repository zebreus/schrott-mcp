//! Probe: MKM live scrapen und in eine *Kopie* der Produktionsdatenbank
//! aufzeichnen. Die Produktion wird nicht angefasst.
//!
//! Aufruf: `cargo run --release --example mkm_probe -- <ziel-verzeichnis>`
//!
//! Die Probe liefert den Nachweis, dass der Handler gegen die *live* Seite
//! läuft: sie nennt den Cache-Buster der Seite, den SHA-256 des geholten
//! Stylesheets und die Zahl der übernommenen Preise.

use chrono::Utc;
use schrott_mcp_ingestion::traders::handlers::mkm_metals;
use schrott_mcp_store::PublicDb;
use sha2::{Digest, Sha256};

/// Der Pfad, den der Handler als Reviewed-Asset prüft.
const STYLESHEET_PATH: &str = "/assets/css/gesamt.min.css";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = std::env::args().nth(1).expect("Ziel-Verzeichnis angeben");
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) Chrome/120")
        .build()?;

    // Der Nachweis, gegen wen der Handler laeuft: die Seite liefert den
    // Cache-Buster, das Stylesheet wird mit genau dieser Query geholt.
    let page = client
        .get(mkm_metals::URL)
        .send()
        .await?
        .text()
        .await?;
    let buster = page
        .split("gesamt.min.css")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .filter(|q| q.starts_with('?'))
        .unwrap_or("(kein)");
    let css_url = format!("https://mkm-metals.de{STYLESHEET_PATH}{buster}");
    let css = client.get(&css_url).send().await?.text().await?;
    let normalized = css.replace("\r\n", "\n").replace('\r', "\n");
    println!("beobachtet_utc: {}", Utc::now().to_rfc3339());
    println!("seite: {}", mkm_metals::URL);
    println!("stylesheet_url: {css_url}");
    println!("cache_buster: {buster}");
    println!("stylesheet_sha256: {:x}", Sha256::digest(normalized.as_bytes()));
    println!("stylesheet_bytes: {}", normalized.len());

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
