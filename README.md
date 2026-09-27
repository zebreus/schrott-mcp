# Schrott MCP

One Rust process serving both the marketing/auth website and a remote MCP
server at `https://schrott.offsite.lol`, backed by two SQLite databases.

The Schrott MCP collects Schrotthändler, Wertstoffhändler & Co. from all over
Germany — with current and historical prices, transparent sourcing, and access
via the Model Context Protocol. Website language is German; code and schema
identifiers stay English so agents query compactly.

With love as the secret ingredient.

## Layout (cargo workspace)

- `crates/core` — config, username/password validation, shared data-model types.
- `crates/store` — SQLite access. Two files live in the data dir:
  - `internal.db` — users, sessions, API tokens, OAuth clients/codes/tokens,
    ingestion runs/steps and raw fetch log (private).
  - `public.db` — the entire queriable Schrott data set exposed to every MCP
    user (not user specific): `traders`, `materials`, `trader_materials`,
    `prices`, `current_prices` plus the `v_current_prices` view and the
    `traders_fts` full-text index.
  - `internal/` and `public/` are each split by domain, every file owning its
    tables, row types and queries.
- `crates/auth` — argon2 password hashing, random tokens, SHA-256, PKCE-S256.
- `crates/ingestion` — pipeline skeleton (`seed -> scrape -> journal`) plus
  the static material catalog. Händler scrapers are not built yet; runs only
  refresh the catalog until then.
- `crates/server` — axum web app: German marketing page, signup/login
  (username + password + "professional data-user" checkbox, nothing else),
  dashboard, OAuth 2.0 authorization server with dynamic client registration
  (so an MCP host can just be pointed at the URL), and the MCP
  Streamable-HTTP endpoint with the tools `schrott_query_sql` (read-only SQL)
  and `schrott_feedback` (data-issue reports).
  Split by surface: `web/` (`pages` templates + `style`, `auth`, `dashboard`,
  `site`), `mcp/` (`protocol` transport, `tools`, isolated `worker`),
  `oauth/` (one file per flow: `discovery`, `register`, `authorize`, `token`),
  and shared `state.rs` (sessions, CSRF, flash) plus `respond.rs`.
  Shared `respond` module for consistent responses; one-time secrets travel
  in server-side flash state (never URLs); cookie POSTs carry CSRF tokens.
  Unit tests live next to the code (`cargo test --workspace`).
- `crates/query-worker` — tiny second binary executing one ad-hoc SQL query
  against the public database in read-only mode. The server spawns it per
  `schrott_query_sql` call under OS confinement (512 MB address space, 30 s
  CPU, no core dumps, no new processes, `NO_NEW_PRIVS`, dropped to the
  `nobody` user, empty environment) with a 60 s wall-clock kill switch. A
  runaway query kills the worker; the main process never feels it.

## Queryable data model

Designed for AI agents, superfast queries, and future growth:

- `traders` — one row per Händler (slug, name, type, address, geo, contact,
  opening hours, pickup/dropoff, certifications, status). `traders_fts`
  (FTS5) makes name/city/postcode search instant.
- `materials` — the static price catalog (slug, German name, category, unit).
  Seeded by ingestion; scrapers never invent materials.
- `trader_materials` — which trader accepts which material, with conditions
  and validity window.
- `prices` — append-only observations: price + currency/unit, uncertainty
  (`price_min`/`price_max`, `confidence`), provenance (`source_type`,
  `published` = trader published it themselves, `source_url`), and time
  (`observed_at`, `published_at`, `valid_from`/`valid_to`). `NULL` bounds
  mean open-ended; `NULL` uncertainty means exact/unknown.
- `current_prices` — materialized latest observation per trader + material
  (forward-only in `observed_at`, so late backfills never clobber new data).
- `v_current_prices` — pre-joined view for the most common agent question
  ("what does X pay for Y right now?").
- Every table carries `extra_json` headroom plus full timestamps, so the
  schema grows by adding columns, never by breaking old ones.

## Why SQLite and not Postgres?

Single node, zero operations, backups are file copies, and the read/write
split we need maps naturally onto two SQLite files (internal vs. public).
If we ever need multiple writers or horizontal scale, Postgres becomes the
better choice — the `store` crate boundary is where that swap would happen.

## Run

```sh
cargo run -p schrott-mcp-server -- --bind 127.0.0.1:4001 \
    --data-dir ./data --base-url http://localhost:4001
```

Env fallbacks: `BIND`, `DATA_DIR`, `BASE_URL`. The ingestion scheduler runs
inside the same process (first run shortly after boot, then every 6h), and
`POST /api/ingest/run` (logged in) triggers a manual run.

## Deploy

Systemd unit `schrott-mcp.service` runs the release binary as root behind
the existing nginx, which terminates TLS for `schrott.offsite.lol`
(port 4001, data dir `/var/lib/schrott-mcp`). Binaries live in
`/usr/local/bin/` (`schrott-mcp-server` next to
`schrott-mcp-query-worker` — the worker must sit beside the server).
