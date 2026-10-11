# Owner check-in: pending-rollout gate, 2026-10-11

## Production observations

- Service active; warning journal for the preceding six hours empty.
- Public counts: 3,881 traders, 54 materials, 2,426 Current prices.
- Latest feedback ID remains 6080 (2026-10-09); no new feedback in this check.
- Existing six-handler verifier, run with the actual release cutoff
  `2026-10-09T20:04:12Z`, passes ELNO (36), Trapper (21), AGH (8),
  Saxonia (4), Hofmann Rastatt (23), reGOLD (20). All recorded source
  fetches HTTP 200; no contract failures. Saxonia and reGOLD now also have
  successful steps on 2026-10-11, at 00:03 and 00:18 UTC respectively.
- `gh auth status` reports no authenticated GitHub host. Publication remains
  blocked; no deployment is claimed for the pending commits.

## New bounded gate

`scripts/verify-pending-corrections.py` checks the five explicit contact
clears, Schrott24's empty service objects, the Gotha branch address and fresh
trader timestamps. It requires completed successful post-cutoff ingestion
steps and canonical HTTP-200 source fetches for Gutzmann (19) and MSG (124).
MSG's same-source Current rows must have the reviewed producer marker,
quantity/variant identity, exact normalized positive EUR quotes, fresh
observations, source classification and supported publication-date policy.
Unmanaged same-source leftovers are red for review, not silently deleted.

This gate is intentionally **not** an independent source-to-material oracle,
proof of scheduler trigger type, all-handler coverage, all dossier facts or
Gutzmann's full quote semantics. It complements the reviewed MSG source
inventory, Rust parser/store tests, and a deployment journal. The two databases
are read independently; do not treat the result as a cross-database snapshot.
Future legitimate price-list changes require review of the fixed 124/19 gates.

Connections use SQLite URI `mode=ro` and `PRAGMA query_only=ON`. The script
does not run ingestion or change production data. Temporary-database tests
prove a green fixture leaves both database files byte-identical, attempted
writes fail, and stale observations/seed/steps, incorrect contacts/address,
portal arrays, fetch errors/provenance, canaries, incomplete/wrong-count steps,
legacy/invalid metadata, bad quantity identity, bad quote semantics, unsupported
dates and missing Current rows fail. Fixtures do not prove deployment.

Commands:

```bash
python3 -B -m unittest discover -s scripts -p 'test_verify*py' -v
python3 -B scripts/verify-handler-release.py --since 2026-10-09T20:04:12Z
python3 -B scripts/verify-pending-corrections.py --since 2026-10-11T00:00:00Z
```

All twelve verifier tests pass (four existing release tests, eight new tests).
The last command is deliberately a **pre-rollout negative control**, not an
actual rollout timestamp: production still has legacy contacts and Gotha
address, zero managed MSG tier rows, legacy MSG Current keys and no completed
post-cutoff 124/19 steps. It exits 1 as expected. After real publication and
deployment, rerun with the actual rollout timestamp and wait for regular
scheduler phases; do not force-ingest or manually repair production databases.

Only Python verifier/tests and this evidence document change here. No Rust,
dossier, catalog, geocoding or production data changes; the independent MKM
implementation worktree is untouched.
