# Owner check-in: Kernschrott rollout gate, 2026-10-11

Service active, no warning entries in the preceding six hours. Read-only counts
remain 3,881 traders / 54 materials / 2,426 Current prices. Feedback latest ID
6080 is unchanged. No matching `/tmp/opencode/*geo*.log` files found; this is
not proof of successful previous geocoding. `gh auth status` remains unauthenticated.
Main was clean and 18 commits ahead of origin before this verifier change.

MKM remediation is handled by its already-resumed implementation session;
no duplicate agent, status polling, candidate edits or premature integration.
Its independent review findings remain blocking until fixed and re-reviewed.

## Missing production gate implemented

`scripts/verify-kernschrott-release.py` checks the documented pending release:

- Completed post-rollout successful step with 38 prices, explicit single
  Class-3 exclusion and no canary; canonical same-run HTTP-200 source fetch.
- 38 Current rows, exact confidence 1, EUR/kg and catalog-unit agreement,
  published trader source and fresh observations; reviewed 06.10.2026 page
  date with `page_stated` metadata.
- Preserves the genuine Lüfter zero-price, rejects other zero/negative or
  invalid quotes, and rejects request-only/Class-3 Current quotes.
- Both full reviewed global conditions appear in base-row notes. The
  notes-free Current view alone is not a safe conditions surface.
- 19 explicit fresh accepted aggregates with the reviewed material set and
  both conditions; correct three request materials and Class-3 quarantine text.
  Additional implicit fallback acceptances are not falsely counted as the
  handler's 19 explicit aggregates.

This is a **bounded persisted-contract gate**, not an independent 38-variant
source oracle, historical integrity guarantee or scheduler-trigger proof.
Pair with the existing source/parser/TempDB review and deployment journal.
The fixed count/date/zero contracts require source re-review on legitimate
changes. Read-only connections use URI `mode=ro` plus `query_only=ON`; no
ingestion or production changes. Databases are sampled independently.

Four new isolated SQLite tests cover green/zero-price, byte-identical files
and rejected writes, independently mutated quote semantics/source/date/
freshness/conditions/request/quarantine, lost acceptances, bad fetches and
canaries/stale steps. Together with prior verifier tests: **16 passing**.
These fixtures are not production evidence.

```bash
python3 -B -m unittest discover -s scripts -p 'test_verify*py' -v
python3 -B scripts/verify-kernschrott-release.py --since 2026-10-11T00:00:00Z
```

The second command is a deliberate **pre-rollout negative control**, not the
actual release cutoff. It exits 1: no completed post-cutoff step, zero Current
quotes and zero aggregates. Kernschrott is still not deployed. After publication
and coordinated deployment, use the actual rollout timestamp and wait for a
normal scheduler phase. Do not substitute manual force ingestion or DB repairs.

No Rust, dossier, catalog, geocoding, production or other worktree changes.
