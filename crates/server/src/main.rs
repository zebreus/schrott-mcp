//! One process: website + OAuth server + MCP server + ingestion scheduler.

mod mcp;
mod oauth;
mod respond;
mod state;
mod web;

use std::sync::Arc;
use std::time::Duration;

use axum::{
    routing::{get, post},
    Router,
};
use offsite_data_core::AppConfig;
use offsite_data_ingestion::seed_metadata;
use offsite_data_store::{InternalDb, PublicDb};
use tracing_subscriber::{fmt, EnvFilter};

use state::AppState;

fn usage() -> String {
    "usage: offsite-data-server [--bind ADDR] [--data-dir DIR] [--base-url URL]".to_owned()
}

/// Parsed `--key value` flags: (bind, data_dir, base_url).
type CliArgs = (Option<String>, Option<String>, Option<String>);

/// Parse `--key value` CLI flags (env vars fill the gaps inside AppConfig).
fn parse_args() -> Result<CliArgs, String> {
    let mut bind = None;
    let mut data_dir = None;
    let mut base_url = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bind" => bind = args.next(),
            "--data-dir" => data_dir = args.next(),
            "--base-url" => base_url = args.next(),
            "--help" | "-h" => return Err(usage()),
            other => return Err(format!("unknown argument: {other}\n{}", usage())),
        }
    }
    if bind.is_none() && data_dir.is_none() && base_url.is_none() {
        // No flags at all is fine — defaults + env apply.
    }
    Ok((bind, data_dir, base_url))
}

#[tokio::main]
async fn main() {
    fmt()
        .with_env_filter(
            EnvFilter::from_env("RUST_LOG")
                .add_directive("offsite_data_server=info".parse().expect("valid directive"))
                .add_directive(
                    "offsite_data_ingestion=info"
                        .parse()
                        .expect("valid directive"),
                ),
        )
        .init();

    let (bind, data_dir, base_url) = parse_args().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(2);
    });
    let config = AppConfig::from_parts(bind, data_dir, base_url).unwrap_or_else(|e| {
        eprintln!("configuration error: {e}");
        std::process::exit(2);
    });

    let internal = Arc::new(InternalDb::open(&config.data_dir).unwrap_or_else(|e| {
        eprintln!("cannot open internal.db: {e}");
        std::process::exit(1);
    }));
    let public = Arc::new(PublicDb::open(&config.data_dir).unwrap_or_else(|e| {
        eprintln!("cannot open public.db: {e}");
        std::process::exit(1);
    }));
    if let Err(e) = seed_metadata(&public) {
        eprintln!("cannot seed catalog: {e}");
        std::process::exit(1);
    }

    let http = reqwest::Client::builder()
        .user_agent("offsite-data-server/0.1")
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap_or_else(|e| {
            eprintln!("cannot build http client: {e}");
            std::process::exit(1);
        });

    let state = AppState {
        internal: Arc::clone(&internal),
        public: Arc::clone(&public),
        http,
        base_url: config.base_url.clone(),
        data_dir: config.data_dir.clone(),
        flash: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        csrf: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
    };

    // Ingestion lives in this same process.
    let _scheduler = offsite_data_ingestion::spawn_scheduler(internal, public, 6 * 3600);

    let app = Router::new()
        .route("/health", get(web::site::health))
        .route("/", get(web::site::index))
        .route(
            "/signup",
            get(web::auth::signup_form).post(web::auth::signup_submit),
        )
        .route(
            "/login",
            get(web::auth::login_form).post(web::auth::login_submit),
        )
        .route("/logout", post(web::auth::logout))
        .route("/dashboard", get(web::dashboard::dashboard))
        .route("/tokens/create", post(web::dashboard::create_token))
        .route("/tokens/delete", post(web::dashboard::delete_token))
        .route("/api/ingest/run", post(web::dashboard::ingest_run))
        .route(
            "/.well-known/oauth-authorization-server",
            get(oauth::server_metadata),
        )
        // Some strict clients probe the resource-scoped metadata path.
        .route(
            "/.well-known/oauth-authorization-server/mcp",
            get(oauth::server_metadata),
        )
        .route(
            "/.well-known/oauth-protected-resource",
            get(oauth::protected_resource),
        )
        .route("/oauth/register", post(oauth::register))
        .route(
            "/oauth/authorize",
            get(oauth::authorize_get).post(oauth::authorize_post),
        )
        .route("/oauth/token", post(oauth::token))
        .route("/mcp", get(mcp::mcp_get).post(mcp::mcp_post))
        .route("/d/{secret}/result.json", get(web::site::download_result))
        .fallback(web::site::fallback_404)
        .with_state(state);

    tracing::info!("offsite-data listening on {}", config.bind);
    let listener = tokio::net::TcpListener::bind(config.bind)
        .await
        .unwrap_or_else(|e| {
            eprintln!("cannot bind {}: {e}", config.bind);
            std::process::exit(1);
        });
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        eprintln!("server error: {e}");
        std::process::exit(1);
    });
}
