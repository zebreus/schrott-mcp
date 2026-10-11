//! Read-only proof command. Never calls record(), seeding or the scheduler.
//! With --live fetches the overview and impressum; otherwise reads fixtures
//! or two supplied local HTML paths. JSON retains per-article metadata.
use schrott_mcp_ingestion::traders::{
    fetch_text,
    handlers::mkm_metals::{self, overview},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.as_slice() == ["--registered"] {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(45))
            .build()?;
        let handler = mkm_metals::handler();
        let typed =
            schrott_mcp_ingestion::traders::scrape_record_outcome(&handler, &client).await?;
        let schrott_mcp_ingestion::traders::RecordOutcome::MkmArticles(snapshot) = typed else {
            return Err("registered MKM route lost typed articles".into());
        };
        println!("{}", serde_json::to_string_pretty(&snapshot.report())?);
        if !snapshot.diagnostics.is_empty() {
            return Err("registered route coverage diagnostics".into());
        }
        return Ok(());
    }
    let (status, html, imp) = match args.as_slice() {
        [live] if live == "--live" => {
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(45))
                .build()?;
            let (status, html) = fetch_text(&client, mkm_metals::URL).await?;
            let (_, imp) = fetch_text(&client, mkm_metals::IMPRESSUM_URL).await?;
            (status, html, imp)
        }
        [] => (
            200,
            include_str!("../src/traders/handlers/fixtures/mkm-overview-20261010.html").to_owned(),
            include_str!("../src/traders/handlers/fixtures/mkm-impressum-20261010.html").to_owned(),
        ),
        [html, imp] => (
            200,
            std::fs::read_to_string(html)?,
            std::fs::read_to_string(imp)?,
        ),
        _ => {
            return Err(
                "usage: mkm_inventory [--registered | --live | overview.html impressum.html]"
                    .into(),
            )
        }
    };
    let snapshot = overview::parse(&html, &imp, status)?;
    println!("{}", serde_json::to_string_pretty(&snapshot.report())?);
    if !snapshot.diagnostics.is_empty() {
        return Err("article coverage diagnostics; not complete".into());
    }
    Ok(())
}
