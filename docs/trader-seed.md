# Trader seed and dossier coordinates

`dossiers/<state>/<slug>.md` is compiled by `crates/ingestion/build.rs`
into embedded seed JSON. Rebuild the binary after dossier changes; the
seed applies the embedded data, not live Markdown files.

## Explicit enrichment clears (opt-in)

Blank dossier fields normally **keep stored enrichment**, not delete it.
To withdraw a specific stored value, use an optional flat frontmatter scalar:

```yaml
phone: ''
website: ''
clear_fields: 'phone,website'
```

Only these fields are allowed:

| Clear field | Stored empty default |
|---|---|
| `phone`, `email`, `website`, `opening_hours`, `description` | `''` |
| `website_status` | `unbekannt` |
| `certifications` | `[]` |
| `dropoff_json`, `pickup_json` | `{}` |

- Missing/empty `clear_fields` (also whitespace-only or YAML null) retains
  the previous keep and hash contract. No corpus-wide rewrite is needed.
- Tokens are trimmed and sorted lexicographically for canonical hashing.
  Unknown/forbidden names, duplicates and empty tokens in a nonempty list
  fail closed. For example `phone,`, `phone,,email` and `phone,phone` fail.
  YAML lists/maps are not accepted: this directive is a scalar.
- A clear field must be absent or empty in the same dossier. Any populated
  value conflicts, including whitespace strings, `unbekannt`, `[]` or `{}`.
  Build codegen rejects this; runtime validates the **entire batch before
  any DB write**, including seeds not produced by codegen.
- Clear defaults are applied through the normal seed upsert, without a
  special SQL mutation. Other enrichment fields retain their normal keep
  behavior. Clearing `website` does not implicitly clear `website_status`;
  select both explicitly when appropriate. `website_checked_at` is retained.
- GEO, slug, name, state, city, street/postcode, trader type and trader status
  cannot be cleared by this API. Existing address and GEO semantics are
  unchanged. This is not a general record-deletion API.
- A clear means “no curated current fact”, **not** proof of closure,
  rejection of customers/materials, free service or negative certification.
  Document the withdrawn claim, evidence and date in Timeline; preserve
  historical notes. Directive selection remains an owner review decision.

A directive remains authoritative while present: later enrichment of a
clear-marked field is removed on the next seed, even if `seed_hash` survived
unchanged. Repeated seeds with already-empty defaults do not write or move
`updated_at`; changes preserve `first_seen_at`. Changing/removing a directive
changes its hash ownership; removal resumes ordinary keep behavior and does
not restore old values. Rebuild before applying a changed directive.

## GEO contract

- `lat` and `lon` are evidence-backed WGS84 decimal scalars (quoted or
  plain). Document source, retrieval date and point accuracy in Timeline.
- Both supplied: finite latitude −90…90 and longitude −180…180 required.
  The pair replaces stored coordinates on insert or dossier change.
- Both missing/empty (including YAML null): preserve the existing DB pair.
  New traders without a pair retain NULL coordinates.
- Only one supplied, non-numeric, non-finite or out-of-range: build fails.
  Seed validation also rejects invalid pairs before any DB writes.
- No Rust geocoding, address lookup or inferred coordinates. An address
  change without a supplied pair does **not** invalidate or correct old GEO.
  Removing dossier coordinates does **not** delete stored coordinates.

Coordinate evidence is reviewed by humans; numeric validation cannot prove
that a point belongs to the trader or identifies an entrance rather than
an address/building centroid. Existing dossier pairs will become authoritative
on the first run of the rebuilt binary and may replace old DB GEO.

## Idempotency

Supplied coordinates participate numerically in `extra_json.seed_hash`.
Equivalent decimal spellings do not trigger updates. Dossiers without a
pair keep the historical hash contract (no corpus-wide GEO migration).
Unchanged effective payloads skip writes and preserve `updated_at`;
`first_seen_at` survives changes. If DB GEO differs from a supplied dossier
pair even with the same hash, the next seed restores the authoritative pair.

Regression checks use temporary local databases only:

```sh
CARGO_BUILD_JOBS=1 cargo test -p schrott-mcp-ingestion seed_traders::tests --lib
rustc --edition=2021 --test crates/ingestion/build.rs -o /tmp/opencode/seed-build-tests
/tmp/opencode/seed-build-tests
```
