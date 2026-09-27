# Offsite Data

One Rust process serving both the marketing/auth website and a remote MCP
server at `https://data.offsite.lol`, backed by two SQLite databases.

With love as the secret ingredient.

## Layout (cargo workspace)

- `crates/core` — config, username/password validation, shared data-model types.
- `crates/store` — SQLite access. Two files live in the data dir:
  - `internal.db` — users, sessions, API tokens, OAuth clients/codes/tokens,
    ingestion runs/steps and raw fetch log (private).
  - `public.db` — `sources`, `datasets`, `items`: the entire queriable data
    set exposed to every MCP user (not user specific).
  - `internal/` is split by domain (`users`, `oauth`, `pipeline`, `sharing`),
    each owning its tables, row types and queries.
- `crates/auth` — argon2 password hashing, random tokens, SHA-256, PKCE-S256.
- `crates/ingestion` — minimal pipeline (`fetch -> parse -> normalize ->
  diff -> upsert`) plus three wildly different example scrapers and a
  `LlmChangeChecker` hook for later unstructured-change detection.
- `crates/server` — axum web app: marketing page, signup/login (username +
  password + "professional data-user" checkbox, nothing else), dashboard,
  OAuth 2.0 authorization server with dynamic client registration (so an MCP
  host can just be pointed at the URL), and the MCP Streamable-HTTP endpoint.
  Split by surface: `web/` (`pages` templates + `style`, `auth`, `dashboard`,
  `site`), `mcp/` (`protocol` transport, `tools`, isolated `worker`),
  `oauth/` (one file per flow: `discovery`, `register`, `authorize`, `token`),
  and shared `state.rs` (sessions, CSRF, flash) plus `respond.rs`.
  Shared `respond` module for consistent responses; one-time secrets travel
  in server-side flash state (never URLs); cookie POSTs carry CSRF tokens.
  Unit tests live next to the code (`cargo test --workspace`).
- `crates/query-worker` — tiny second binary executing one ad-hoc SQL query
  against the public database in read-only mode. The server spawns it per
  `data_query_sql` call under OS confinement (512 MB address space, 30 s CPU,
  no core dumps, no new processes, `NO_NEW_PRIVS`, dropped to the `nobody`
  user, empty environment) with a 60 s wall-clock kill switch. A runaway
  query kills the worker; the main process never feels it.

## Why SQLite and not Postgres?

Single node, zero operations, backups are file copies, and the read/write
split we need maps naturally onto two SQLite files (internal vs. public).
If we ever need multiple writers or horizontal scale, Postgres becomes the
better choice — the `store` crate boundary is where that swap would happen.

## Run

```sh
cargo run -p offsite-data-server -- --bind 127.0.0.1:4000 \
    --data-dir ./data --base-url http://localhost:4000
```

Env fallbacks: `BIND`, `DATA_DIR`, `BASE_URL`. The ingestion scheduler runs
inside the same process (first run shortly after boot, then every 6h), and
`POST /api/ingest/run` (logged in) triggers a manual run.

## Deploy

Systemd unit `offsite-data.service` runs the release binary as root behind
the existing nginx, which terminates TLS for `data.offsite.lol`.
