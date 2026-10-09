# Schrott MCP

One Rust process serving both the marketing/auth website and a remote MCP
server at `https://schrottindex.de`, backed by two SQLite databases.

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
  Beller Berlin's [handler](crates/ingestion/src/traders/handlers/beller.rs)
  reads the HTML modal `PREISLISTE ANKAUF`, retaining grades and quantity
  conditions; see the [owner verification](research/handler-owner-beller-20261008.md).
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
  `valid_from`/`valid_to`). For handlers without a page-stated date,
  `published_at` uses the German calendar day only when a comparable scrape
  detects a price change; `extra_json.published_at_basis` distinguishes this
  inference (`observed_price_change`) from a page-stated date (`page_stated`).
  `NULL` means unknown; `NULL` bounds mean open-ended; `NULL` uncertainty
  means exact/unknown.
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
free-form working notes, `## Timeline` = dated research context) AND the
direct build input — no JSON detour:

- `crates/ingestion/build.rs` — the single compiler (frontmatter →
  scalars, Timeline bullets → `notes`, zero third-party deps so the build
  stays offline-capable). It runs on every build, re-runs on any dossier
  change (`cargo:rerun-if-changed`), and emits one JSON file per state
  into `$OUT_DIR/seed_traders/` (under `target/`, never committed), which
  `seed_traders.rs` embeds via `include_str!`. Bad dossiers fail the
  build LOUDLY (unknown/duplicate keys, block scalars, missing slug).
  Slugs (`<state>-<city>-<name>`) are derived once and never hand-edited.
- `crates/ingestion/src/seed_traders.rs` — parses/validates the embedded
  seeds and applies changes via a payload hash in `extra_json.seed_hash`, so `updated_at` keeps
  meaning "last real change" and `first_seen_at` survives.
- Evidence-backed `lat`/`lon` frontmatter values are imported as a WGS84
  pair and override existing DB coordinates, including on later dossier
  changes. Both absent/empty preserve existing coordinates; one-sided,
  non-finite or out-of-range pairs fail validation. Rust does not geocode
  addresses. Sources and point accuracy belong in the dossier Timeline;
  removing a pair does not clear DB GEO. See [seed semantics](docs/trader-seed.md).
- `cargo test` validates the whole corpus (unique slugs, enum values,
  state codes, URL shapes, idempotency) — the CI gate for seed changes.

## Keeping the data fresh

1. **Seed updates (quarterly):** re-audit agents edit the dossiers
   (`dossiers/<state>/<slug>.md`: frontmatter scalars, Timeline bullets
   with `[Recherche DD.MM.YYYY: ...; Quelle: ...]`), `cargo test` must
   pass, merge → rebuild/redeploy → the boot seed applies the diff
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

## Feedback triage (schrott_feedback)

User reports are leads, not verdicts — they are often wrong or half-right
and ALWAYS need deep verification, never binary thinking (postmortem
30.09.2026: a "website shows a different city" report was correct but
one-sided — the seed row had merged TWO same-name traders, and unlinking
the website alone left a phantom entity behind):

1. **Two-sided check.** A mismatch claim confirms only one side. Always
   verify the other side too: search the claimed address — it may host a
   *different, real* trader.
2. **Impressum alone is not enough.** Name + Ort in the impressum proves
   the website's owner, not that the dossier row IS that owner.
3. **Namesake/merge check.** Family names (Kaiser, Schmidt, Müller, …)
   attract conflated rows: search for same-name traders at the claimed
   address before attributing anything.
4. **No phantom leftovers.** After unlinking a wrong attribute, re-check
   that the remaining dossier (name + place) still describes a real
   entity — otherwise split the row or downgrade to `pruefung` with a
   Timeline note instead of leaving a plausible-looking fiction.
5. Every triage decision lands in the dossier Timeline with
   `[Korrektur DD.MM.YYYY: …; Quelle: …]` including its
   counter-evidence.
6. **Source hierarchy (30.09.2026):** aggregators (Das Örtliche, Gelbe
   Seiten, 11880, GoLocal, Cylex, city-map, schrottplatz-/schrottradar
   portals and the like) are LEADS, never evidence — routinely outdated
   or wrong. Evidence is only: operator website (Impressum/Kontakt),
   Handelsregister/Northdata/Creditreform, municipal trade register,
   operator-run social profile. Beleg-Standard = 2 independent sources
   from this list; aggregator + evidence still counts as a single
   source (`Einzelbeleg`). When triage touches a dossier, re-verify any
   legacy aggregator-backed fields instead of trusting them.
7. **Authoritative operator primary source (01.10.2026):** a verified
   operator site (impressum with name + HRB + Ort, HR-congruent per
   Northdata, current branch pages, audit certificates) suffices ALONE
   for that operator's own branch facts (address, phone, mail, hours,
   services, branch existence) — no second source needed. Recognition
   criteria (all must hold): impressum names the operating company with
   register number; the entity is live in the register; branch pages
   are per-site (address/phone/hours, not a generic contact form);
   content is current (certificates, dates, news). Anything failing a
   criterion falls back to the 2-source standard. Second source still
   required for identity questions (rename/merge chains, see below).

## Operator rename/merge chains (Betreiber-Ketten)

Large operators (ALBA, INTERSEROH, TSR, REMONDIS, …) rename, merge and
relocate companies while branches keep operating — leaving dossiers
with dead names, stale addresses and phantom rows. Case study
01.10.2026: `bb-wittenberge-interseroh-metallaufbereitung-ost`
(INTERSEROH Metallaufbereitung Ost, merged away 2011) is actually the
live ALBA Metall Nord branch Quitzow (Buchholzer Chaussee 5, Perleberg)
— found only by crawling the operator's branch detail pages.

Recognition signals: dossier name is a former legal entity (check the
name history on Northdata — HRB pages list prior names); branch
address exists only as aggregator lead; operator runs a branch finder
with per-site detail pages; EFB/environmental certificates name the
operator at the branch address.

Handling protocol: (1) prove the chain — Northdata/HR name history
(old → new entity, merger publications); (2) crawl EVERY linked branch
detail page individually, never stop at the overview (Quitzow lesson —
the overview lists names, the detail page carries the facts);
(3) correct `name`/address/city per the operator primary source,
keeping the old name in the Timeline; (4) slugs NEVER change
(DB keys, feedback refs — stability over cosmetics);
(5) delisted branches are checked for closure/relocation (operator
search, register, news) and never deleted — `geschlossen` only with
evidence, otherwise `pruefung` + Klärfall; (6) cross-reference sibling
dossiers (same operator, same city) instead of merging.

Research delegation: use the short, outcome-oriented assignment in
`prompts/tiefenrecherche-welle.md`. Agents choose their research methods,
tools and depth; the source and operator-chain standards above remain
the reference for assessing results, rather than a checklist copied into
each assignment. The owner gate before commit checks identity, sources,
preserved history and overwrites (non-empty field changes need a documented
2-source or authoritative-primary-source basis), runs
`cargo test -p schrott-mcp-ingestion seed`, and verifies production after
redeploy. Agent freedom changes the workflow, not the evidence standard.

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
the existing nginx, which terminates TLS for `schrottindex.de`
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
