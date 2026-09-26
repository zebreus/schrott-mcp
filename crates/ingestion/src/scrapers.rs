//! Three deliberately different scrapers: a JSON search API, a JSON REST API
//! needing custom headers, and plain HTML parsed with CSS selectors.

use serde_json::Value;

use super::IngestError;

/// One normalized record produced by a scraper, before hashing/upsert.
#[derive(Debug, Clone)]
pub struct RawItem {
    /// Stable id within its dataset.
    pub external_id: String,
    /// Dataset this item belongs to (e.g. `hn-front-page`).
    pub dataset: &'static str,
    pub title: String,
    pub url: String,
    pub published_at: String,
    /// Short human-readable excerpt shown in listings.
    pub summary: String,
    /// Scraper-specific structured fields.
    pub data: Value,
    /// Unstructured text used for change detection (and the future LLM hook).
    pub unstructured_text: String,
}

/// Scrape result: the items plus a log line for the fetch journal.
pub struct ScrapeOutcome {
    pub items: Vec<RawItem>,
    pub fetch_url: String,
    pub status_code: u16,
    pub byte_len: usize,
}

/// Run every bundled scraper; failures are returned per scraper so one bad
/// website never stops the rest of the pipeline.
pub async fn scrape_all(
    client: &reqwest::Client,
) -> Vec<(&'static str, Result<ScrapeOutcome, IngestError>)> {
    vec![
        ("hn-front-page", scrape_hn_front_page(client).await),
        ("rust-releases", scrape_rust_releases(client).await),
        ("example-html", scrape_example_html(client).await),
    ]
}

/// Hacker News front page via the Algolia JSON search API.
async fn scrape_hn_front_page(client: &reqwest::Client) -> Result<ScrapeOutcome, IngestError> {
    let url = "https://hn.algolia.com/api/v1/search?tags=front_page";
    let res = client
        .get(url)
        .send()
        .await
        .map_err(|source| IngestError::Fetch {
            url: url.to_owned(),
            source,
        })?;
    let status = res.status().as_u16();
    let body = res.text().await.map_err(|source| IngestError::Body {
        url: url.to_owned(),
        source,
    })?;
    let byte_len = body.len();
    let json: Value = serde_json::from_str(&body).map_err(|e| IngestError::Parse {
        url: url.to_owned(),
        detail: e.to_string(),
    })?;
    let mut items = Vec::new();
    if let Some(hits) = json.get("hits").and_then(Value::as_array) {
        for hit in hits.iter().take(30) {
            let id = hit
                .get("objectID")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if id.is_empty() {
                continue;
            }
            let title = hit
                .get("title")
                .and_then(Value::as_str)
                .unwrap_or("(untitled)");
            let item_url = hit.get("url").and_then(Value::as_str).unwrap_or_default();
            let points = hit.get("points").and_then(Value::as_u64).unwrap_or(0);
            let comments = hit.get("num_comments").and_then(Value::as_u64).unwrap_or(0);
            let author = hit
                .get("author")
                .and_then(Value::as_str)
                .unwrap_or_default();
            items.push(RawItem {
                external_id: id.to_owned(),
                dataset: "hn-front-page",
                title: title.to_owned(),
                url: item_url.to_owned(),
                published_at: hit
                    .get("created_at")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
                summary: format!("{points} points by {author} · {comments} comments"),
                data: serde_json::json!({
                    "points": points,
                    "comments": comments,
                    "author": author,
                }),
                unstructured_text: format!("{title} {points} {comments}"),
            });
        }
    }
    Ok(ScrapeOutcome {
        items,
        fetch_url: url.to_owned(),
        status_code: status,
        byte_len,
    })
}

/// Rust releases via the GitHub JSON REST API (needs a `User-Agent` header).
async fn scrape_rust_releases(client: &reqwest::Client) -> Result<ScrapeOutcome, IngestError> {
    let url = "https://api.github.com/repos/rust-lang/rust/releases?per_page=10";
    let res = client
        .get(url)
        .header("User-Agent", "offsite-data-ingestion/0.1")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|source| IngestError::Fetch {
            url: url.to_owned(),
            source,
        })?;
    let status = res.status().as_u16();
    let body = res.text().await.map_err(|source| IngestError::Body {
        url: url.to_owned(),
        source,
    })?;
    let byte_len = body.len();
    let json: Value = serde_json::from_str(&body).map_err(|e| IngestError::Parse {
        url: url.to_owned(),
        detail: e.to_string(),
    })?;
    let mut items = Vec::new();
    if let Some(releases) = json.as_array() {
        for rel in releases {
            let id = rel.get("id").map(|v| v.to_string()).unwrap_or_default();
            if id.is_empty() {
                continue;
            }
            let tag = rel
                .get("tag_name")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let name = rel.get("name").and_then(Value::as_str).unwrap_or(tag);
            let notes = rel.get("body").and_then(Value::as_str).unwrap_or_default();
            let summary: String = notes.chars().take(280).collect();
            items.push(RawItem {
                external_id: id,
                dataset: "rust-releases",
                title: format!("Rust {tag} — {name}"),
                url: rel
                    .get("html_url")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
                published_at: rel
                    .get("published_at")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
                summary,
                data: serde_json::json!({
                    "tag": tag,
                    "draft": rel.get("draft"),
                    "prerelease": rel.get("prerelease"),
                }),
                unstructured_text: notes.to_owned(),
            });
        }
    }
    Ok(ScrapeOutcome {
        items,
        fetch_url: url.to_owned(),
        status_code: status,
        byte_len,
    })
}

/// Plain HTML page parsed with CSS selectors — the odd one out on purpose.
async fn scrape_example_html(client: &reqwest::Client) -> Result<ScrapeOutcome, IngestError> {
    let url = "https://example.com";
    let res = client
        .get(url)
        .send()
        .await
        .map_err(|source| IngestError::Fetch {
            url: url.to_owned(),
            source,
        })?;
    let status = res.status().as_u16();
    let body = res.text().await.map_err(|source| IngestError::Body {
        url: url.to_owned(),
        source,
    })?;
    let byte_len = body.len();
    let doc = scraper::Html::parse_document(&body);
    let h1 = scraper::Selector::parse("h1").expect("valid selector");
    let p = scraper::Selector::parse("p").expect("valid selector");
    let heading = doc
        .select(&h1)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_owned())
        .unwrap_or_default();
    let paragraphs: Vec<String> = doc
        .select(&p)
        .map(|el| el.text().collect::<String>().trim().to_owned())
        .filter(|t| !t.is_empty())
        .collect();
    let summary = paragraphs.first().cloned().unwrap_or_default();
    let unstructured_text = format!("{heading}\n{}", paragraphs.join("\n"));
    Ok(ScrapeOutcome {
        items: vec![RawItem {
            external_id: "example-com".to_owned(),
            dataset: "example-html",
            title: if heading.is_empty() {
                "Example Domain".to_owned()
            } else {
                heading
            },
            url: url.to_owned(),
            published_at: String::new(),
            summary,
            data: serde_json::json!({ "paragraphs": paragraphs.len() }),
            unstructured_text,
        }],
        fetch_url: url.to_owned(),
        status_code: status,
        byte_len,
    })
}
