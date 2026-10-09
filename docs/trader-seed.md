# Trader seed and dossier coordinates

`dossiers/<state>/<slug>.md` is compiled by `crates/ingestion/build.rs`
into embedded seed JSON. Rebuild the binary after dossier changes; the
seed applies the embedded data, not live Markdown files.

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
cargo test -p schrott-mcp-ingestion seed
rustc --edition=2021 --test crates/ingestion/build.rs -o /tmp/opencode/seed-build-tests
/tmp/opencode/seed-build-tests
```
