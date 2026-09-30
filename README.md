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
- `crates/ingestion` — trader price ingestion (`traders/`): one file per
  Händler in `handlers/` (slug, schedule, scrape fn with its own
  selectors + label→material table — parsing is never shared, only HTTP,
  German-number and date helpers). The `scheduler` runs what's due in one
  sequential loop: staggered 6 h cadence (slug-hash offset) or per-trader
  `DailyAt` times (Europe/Berlin), per-handler timeouts, failure
  isolation, run/step/fetch bookkeeping. 5 handlers live (Vedder,
  Lausitz, Tappe, Kupferhelden, Metallankauf24); trader #6..#500 = one
  new file + one registry line. `examples/live_handlers.rs` runs handlers
  against real pages without touching the DB (handler dev loop).
- `crates/server` — axum web app: German marketing page, signup/login
  (username + password + "professional data-user" checkbox, nothing else),
  dashboard, OAuth 2.0 authorization server with dynamic client registration
  and Client ID Metadata Documents (CIMD)
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

- `traders` — one row per Händler (slug, name, type, curated German
  `description`, address, geo, contact, `website` + `website_status`
  [`aktiv`/`tot`/`blockiert`/`unbekannt`] + check timestamp, opening
  hours, service conditions, min/max quantity, certifications, status).
  `traders_fts` (FTS5) makes name/city/postcode search instant.
  Drop-off/pickup are condition objects (`dropoff_json`/`pickup_json`:
  `allowed`, `customer_types` [`privat`/`gewerbe`], `days`, `time_windows`,
  quantities, free `conditions`) — never plain yes/no. Structured facts
  live in typed columns; `extra_json` carries only provenance
  (`seed_*`), review notes, aliases and other URLs; `notes` stays free
  prose. Migrations are additive (`migrate()` backfills new columns in
  existing files, e.g. booleans → condition JSON).
- `materials` — the static price catalog (slug, German name, category, unit).
  Seeded by ingestion; scrapers never invent materials.
- `trader_materials` — which trader accepts which material, with conditions
  and validity window.
- `prices` — append-only observations: price + currency/unit, the trader's
  own `variant` sub-grade (`''` = standard; two grades never collapse into
  one current price), uncertainty (`price_min`/`price_max`, `confidence`),
  provenance (`source_type`, `published` = trader published it themselves,
  `source_url`), and time (`observed_at`, `published_at`, per-material
  `valid_from`/`valid_to`). `NULL` bounds mean open-ended; `NULL`
  uncertainty means exact/unknown.
- `current_prices` — materialized latest observation per trader + material
  + variant (forward-only in `observed_at`, so late backfills never clobber
  new data).
- `v_current_prices` — pre-joined view for the most common agent question
  ("what does X pay for Y right now?").
- Every table carries `extra_json` headroom plus full timestamps, so the
  schema grows by adding columns, never by breaking old ones.

## Trader seed: from research to database

The `dossiers/<state>/<slug>.md` files are the human-readable source (one
dossier per trader: YAML frontmatter = all scalars, `## Überblick` =
free-form working notes, `## Timeline` = dated research context); the
database is seeded from versioned JSON compiled from them:

- `seed/traders/<state>.json` — GENERATED, never hand-edited. Rebuild
  after any dossier change: `python3 tools/dossiers2seed.py --write`
  (dry-run default: without `--write` it only reports diffs).
  Slugs (`<state>-<city>-<name>`) are derived once and never hand-edited,
  so re-compiles update instead of duplicating.
- `tools/dossiers2seed.py` — the single compiler (frontmatter → scalars,
  Timeline bullets → `notes`). Retired: `tools/seed2dossiers.py`
  (one-shot migration) and `tools/md2seed.py` (legacy
  `recherche/*.md` pipeline); the `recherche/*.md` reports remain as
  read-only archive.
- `crates/ingestion/src/seed_traders.rs` — parses/validates the embedded
  JSON and upserts it on every ingestion run. Unchanged rows are skipped
  via a payload hash in `extra_json.seed_hash`, so `updated_at` keeps
  meaning "last real change" and `first_seen_at` survives.
- `cargo test` validates the whole corpus (unique slugs, enum values,
  state codes, URL shapes, idempotency) — the CI gate for seed changes.

## Keeping the data fresh

1. **Seed updates (quarterly):** re-audit agents edit the dossiers
   (`dossiers/<state>/<slug>.md`: frontmatter scalars, Timeline bullets
   with `[Recherche DD.MM.YYYY: ...; Quelle: ...]`), recompile
   (`python3 tools/dossiers2seed.py --write`), `cargo test` must pass,
   merge → rebuild/redeploy → the boot seed applies the diff
   automatically. The seed never deletes: closures arrive as
   `status: geschlossen`.
2. **Continuous (between audits):** trader-website monitoring by the
   ingestion scrapers (next milestone — refreshes `updated_at`, records
   price observations, flags dead sites), `schrott_feedback` user
   reports, and manual PRs for corrections.
3. **Review flags:** `status: pruefung` (~1.000 rows) marks entries whose
   buying status still needs a phone/website check; `origin: prose`
   marks heuristic register-cluster parses. Both are queryable and
   shrink with every review pass.

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

Env fallbacks: `BIND`, `DATA_DIR`, `BASE_URL`. The scheduler ticks every
15 minutes and runs whatever trader handlers are due (each staggered to
~every 6 h, or fixed daily times); `POST /api/ingest/run` (logged in)
force-runs every handler immediately.

## Deploy

Systemd unit `schrott-mcp.service` runs the release binary as root behind
the existing nginx, which terminates TLS for `schrott.offsite.lol`
(port 4001, data dir `/var/lib/schrott-mcp`). Binaries live in
`/usr/local/bin/` (`schrott-mcp-server` next to
`schrott-mcp-query-worker` — the worker must sit beside the server).
Backups are WAL-safe only via `sqlite3 <db> ".backup '<dest>'"` —
never plain `cp` (recent rows live in the `-wal` file).

## Logging (systemd journal)

The server logs JSON lines to stderr; systemd journals them as
structured records. Every ingestion event carries `slug` (and errors)
as top-level fields; failures are also visible in `ingestion_runs`
(`partial`), per-trader `ingestion_steps` (`failed` + message) and the
`raw_fetches` journal (failed attempts: `status_code` 0).

```sh
journalctl -u schrott-mcp.service -f                    # follow
journalctl -u schrott-mcp.service -p warning --since today  # failures
journalctl -u schrott-mcp.service -o json-pretty | jq 'select(.slug)'  # per-trader
sqlite3 /var/lib/schrott-mcp/internal.db \
  "SELECT scraper, status, message FROM ingestion_steps ORDER BY id DESC LIMIT 10;"
```
