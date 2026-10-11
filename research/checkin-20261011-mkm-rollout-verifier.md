# Owner check-in: MKM persisted rollout gate

Main implementation is committed at `d9fa958f`, not deployed. Current check:
service active, no six-hour warning entries, feedback latest ID 6080 unchanged,
public counts 3,881 traders / 54 materials / 2,426 Current prices. Main clean
before this verifier addition, 23 commits ahead of origin. GitHub authentication
still missing; no matching geocoding logs (not proof of previous success).

## New read-only gate

`scripts/verify-mkm-release.py` checks:

- New typed schema, completed successful post-cutoff 55-price step, canonical
  same-run HTTP-200 fetch and latest manifest timestamp within that step window.
- Scope/trader/source/owner/time and all **60 exact article IDs**, each fixed
  reviewed material/variant mapping, 55 numeric / 5 requests.
- Every same-source Current key equals the manifest numeric partition. Requests
  cannot retain numeric Current quotes; unexpected legacy/unproven same-source
  leftovers are red for review, not silently repaired or deleted.
- Source price, kg threshold and full strict tier contract remain in metadata;
  Store money/max normalize against actual catalog EUR/kg or EUR/t. `upto`,
  confidence 0.5, no publication-date invention or validity, fresh source quotes.
- Each article-aware material acceptance equals the full latest semantic article
  group, with matching owner/source/conditions/timestamp and no foreign validity.

Connections use SQLite URI `mode=ro` and `query_only=ON`; no ingestion. Databases
are independently sampled; this is not a transactional cross-DB snapshot.
Timestamp window is a bookkeeping correlation, **not proof of scheduler trigger
type**. Pair with deployment journal, reviewed primary-source inventory and
parser/store/alias/history regression evidence. Fixed counts require review on
legitimate source changes. This is not a fresh-source semantic oracle, complete
historical immutability auditor, external CSS/browser validator or DB repair tool.
Unknown same-source leftovers intentionally require human/Owner review despite
the producer correctly preserving their unproven provenance.

## Actual gates and controls

Eight new isolated SQLite tests use the real reviewed 60-article source inventory
with synthetic bookkeeping/state. Green fixture remains byte-identical and writes
are rejected. Independent mutations cover numeric/metadata/source/date/freshness,
acceptance ownership, malformed/duplicate manifest, missing schema, bad fetch,
step-window mismatch, normalized bounds and extra Current entries.

All Python script tests: **33 passed** (9 source audit + 24 rollout verifiers).
No Rust production change in this commit.

Additionally built an isolated Owner harness against **actual Main libraries**:
`/root/mkm-owner-unit-review/src/bin/rollout_fixture.rs`. Real saved HTML → real
MKM parser → real `record_articles` → real Main Store, fresh seeded isolated DB
`/root/mkm-owner-unit-review/rollout-fixture-196750`; only its bookkeeping is
synthetic. The new CLI against this actual schema/data exits 0 with
**60 articles / 55 numeric / 5 requests / 55 Current, no failures**. This catches
schema/JSON shape assumptions that solely synthetic Python tables might miss;
it is explicitly **not** successful production scraping or a scheduler run.
The previous Owner unit harness package now resolves Main libraries, preserving
its original F6 source file; no candidate or reviewer artifacts overwritten.

```bash
python3 -B -m unittest discover -s scripts -p 'test_*.py' -v
python3 -B scripts/verify-mkm-release.py --since 2026-10-11T03:00:00Z \
  --data-dir /root/mkm-owner-unit-review/rollout-fixture-196750
python3 -B scripts/verify-mkm-release.py --since 2026-10-11T03:00:00Z
```

Last command is a **pre-rollout negative control**, not actual rollout cutoff:
production exits 1 with `typed MKM schema missing; deployment not proven`.
After authenticated publication/coordinated deployment, use the real cutoff,
wait for regular scheduler recording, rerun read-only and compare source/journal
evidence. No forced ingestion, direct production writes or GEO changes.
