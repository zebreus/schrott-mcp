# Owner review: private final-artifact short-boot draft (2026-10-18 23:30)

**Original preparation state — boot NOT EXECUTED, no artifact PASS.** No binary started, build run,
production database opened/queried, ingestion triggered, or service changed.
Only this new report and the private script were prepared. Existing untracked
dossiers/research files and worktree code were left untouched.

Script: `/tmp/opencode/checkin-20261018-2330-private-artifact-boot.py` (0700).
Python standard library only. This is an owner-review draft, not a run approval.
At initial preparation even Python syntax/help/test execution was deliberately
not performed under that instruction. The later owner-authorized **synthetic
helper-only selftests** are recorded below; no `main`/boot was executed.

## Inputs that remain deliberately unfilled

The builder must provide the final **unmodified full server artifact**, its
independently pinned SHA256, exact release commit and provenance tying that
artifact to that commit/build inputs. Gold `99dde2c0` and MKM `f58e9e98` are
context, **not** guessed final artifact identities.

The requested ref `refs/owner-releases/goldtrans-mkm-20261018` did not resolve
in this checkout during preparation; the matching-ref listing also returned no
matches. The script defaults to that exact ref and requires it to resolve to the
owner-pinned full release commit. If the builder uses a different namespace,
the owner must explicitly supply `--release-ref`; no fallback to HEAD.

**Chronological follow-up:** after that preparation observation, the owner
confirmed the ref now exists at
`f4bad4dd5b8a3208b952bdb23b84a849900eb6cf`. This is the later owner-confirmed
state, not a correction claiming the ref had always existed. No Git/status
polling was performed by the selftests. The owner also subsequently reported
Cargo exit 0 and artifacts present; builder completion/provenance remain the
owner's separate scope and were not independently exercised by these tests.

Also required: full deployed commit, explicit Public/Internal source paths,
an absent evidence directory under an existing nonsymlink parent, and a
separately SHA256-pinned policy JSON. **No default production/old-backup paths.**
Sources are chosen by the owner at actual run time, not by this preparation.

Invocation shape, **not executed** (replace every placeholder):

```text
python3 /tmp/opencode/checkin-20261018-2330-private-artifact-boot.py \
  --artifact /OWNER/FINAL/schrott-mcp-server \
  --expected-sha OWNER_PINNED_64_HEX_SHA256 \
  --release-ref refs/owner-releases/goldtrans-mkm-20261018 \
  --release-commit OWNER_PINNED_FULL_40_HEX_RELEASE_COMMIT \
  --deployed-commit OWNER_VERIFIED_FULL_40_HEX_DEPLOYED_COMMIT \
  --source-public /OWNER/SELECTED/public.db \
  --source-internal /OWNER/SELECTED/internal.db \
  --evidence-dir /OWNER/NEW/PRIVATE/EVIDENCE_DIRECTORY \
  --policy /OWNER/REVIEWED/policy.json \
  --policy-sha OWNER_PINNED_64_HEX_POLICY_SHA256
```

The policy schema has exactly two keys. A strict illustrative policy is:

```json
{
  "timestamp_exceptions": {"traders": [], "materials": []},
  "sequence_changes": {}
}
```

This strict example is **not a prediction of the actual boot**. `updated_at`
can be listed explicitly for either table only after owner review. No
`first_seen_at`, payload, FTS, or arbitrary timestamp normalization is accepted.
Sequence exceptions require exact integers, for example the *shape*
`"materials": {"before": OWNER_INTEGER, "after": OWNER_INTEGER}` inside
`sequence_changes`; placeholders are not valid JSON. Only `materials` and
`traders` sequence expectations are supported. Unspecified sequences must be
unchanged; rowids and names must remain unchanged even for allowed sequences.
Do not learn an expected value from this run's output and label it an
independent expectation. An unexplained difference means FAILED/investigate.

## Read-only source review and fresh source binding

Reviewed the prior reference report
`research/checkin-20261017-1645-artifact-boot.md` and
`/root/Documents/artifact-boot-20261017-1645/{run.py,audit.py}`.
The new script does not import their harnesses, reuse their paths, assume their
old seed counts, or execute their migration SQL.

Current checkout source observations (not yet final-release proof):

* `crates/server/src/main.rs:78–89` opens both databases and calls
  `seed_metadata`; line 112 starts the normal scheduler.
* `crates/ingestion/src/pipeline.rs:671–687` places an initial
  `sleep(Duration::from_secs(15))` before the scheduler's first `run_once`.
* `crates/ingestion/src/seed_traders.rs:388–405` skips a trader whose effective
  seed hash, coordinates and clear-field state match. Therefore **no trader
  timestamp exception is assumed**. Unchanged source does not prove a live DB
  is already seed-converged; actual payload/ID/hash comparisons remain required.
* `crates/ingestion/build.rs` generates embedded trader JSON from dossiers.
  Comparing only `seed_traders.rs` would miss the actual compiled seed input.
* `crates/store/src/public/materials.rs:55–68` uses an AUTOINCREMENT insert with
  conflict update, unconditionally assigning material `updated_at`. Conflicting
  inserts may also advance `sqlite_sequence`. Exact catalog size/sequence
  increments for the final release/source baseline remain owner work; no 83,
  763, or previous-run sequence constants are imported.
* `crates/store/src/public/prices.rs:433–459` migration 7 deletes only matching
  Goldtrans zero Current pointers and sets version 7; it does not delete History
  or reconstruct older positive pointers.

At actual run time the script resolves **pinned Git objects**, not mutable
worktree files, and records deployed/release object IDs for the whole dossiers
tree, codegen, pipeline/catalog, seed_traders, trader store and material store.
It stops if any differ. This is intentionally conservative: even an unrelated
pipeline/store edit can block it and requires owner review, not silent bypass.
It also checks the release pipeline/main text for the normal scheduler call and
15-second initial delay before `run_once`, recording their hashes. This lexical
check is **not** a control-flow proof; the owner must review the final release
pipeline and all startup paths and confirm artifact/source provenance before
launch. Neither the deployed commit nor release build provenance was established
by this preparation.

## Planned isolation, capture and lifecycle

1. New evidence root and subdirectories 0700; umask 077; existing root rejected.
   Fresh private cwd, HOME, TMPDIR/TMP/TEMP and four XDG directories. Script and
   reviewed policy copied into the evidence root, with hashes recorded.
2. No inherited server environment: only explicit private paths,
   `PATH=/usr/bin:/bin`, `RUST_LOG=info`. No proxy/config/credential/service ENV.
   `/proc/<own PID>/{exe,cwd,cmdline,environ}` verifies exact executable SHA,
   path, arguments, cwd and exact allowlist. Unexpected environment values are
   **not dumped**. No HTTP headers or credentials are collected.
3. Initial and immediately prelaunch minimum: **1200 MiB root/evidence free**
   and **1200 MiB effective available RAM** (minimum of host MemAvailable and
   readable finite cgroup-v2 limit minus current usage). Additional copy-size/WAL estimate must leave
   600 MiB headroom. No swap is relied on. Runtime samples every nominal 50 ms
   enforce **600 MiB disk / 320 MiB RAM**; intervention fails the run.
4. Each selected source is opened by SQLite URI **`mode=ro`**, with
   **`query_only=ON`** checked. No source migration, journal-mode/checkpoint,
   write SQL or repair. Backup API only, bounded progress timeout; source and
   destination connections are closed in `finally`.
5. Separate fresh `before/{public,internal}.db` are preserved. Working copies
   are separate SQLite backups from those read-only before copies and must be
   fully snapshot-identical before launch. A running artifact never receives
   the source or before paths.
6. Execute the full owner artifact directly with loopback bind on a freshly
   probed free port, explicit private `--data-dir`, private cwd and loopback
   `--base-url`. No instrumentation, rebuild, force flag or ingest endpoint.
   Health client has proxies and redirects disabled, only GET `/health`.
   Validate expected 200 JSON and correlate the loopback LISTEN socket inode
   with **the own server PID's `/proc` fds**, not an unrelated raced listener.
7. Health deadline 5.5 seconds; independent watchdog requests own-PID TERM at
   7 seconds, then own-PID KILL after 0.3 seconds if needed. Normal success
   immediately sends TERM after health. Main cleanup also escalates after
   0.35 seconds. SIGINT/TERM/HUP to the harness trigger cleanup. No process
   groups, pkill, other PIDs, or service operations. Require reaping, exit -15,
   closed loopback port, no watchdog/resource events, and **total observed
   lifetime <8 seconds**, below the reviewed 15-second first scheduler delay.
8. Only after reaping, read-only backup the stopped working DBs into separate
   `after/` copies. Audit the snapshots; recheck artifact SHA and pristine
   before logical snapshots and database byte hashes. Retain all failure
   logs/copies, without cleanup/deletion or automatic retry.

## Exact comparison gates, not just counts

Every table, including `sqlite_sequence`, FTS virtual tables and their shadow
tables, is enumerated from `sqlite_schema`. Schemas include SQL, indexes,
triggers, views, rootpages and per-table `table_xinfo`. Rowid tables hash physical
rowids plus every visible/generated data column; WITHOUT ROWID tables use their
complete PK ordering. SQLite storage values are type-tagged; blobs are encoded
exactly, floats use `float.hex`, and rows stream into SHA256 without copying
entire production rowsets into JSON. FTS hidden command/rank pseudocolumns are
not stored-data columns; their definitions remain in xinfo and all FTS shadow
storage columns are compared. Unknown/shadowed rowid layouts fail closed.

* Public must start at **6** and finish at **7**. Schema/table set/application ID
  must be identical. Both integrity checks must be `ok`.
* Before boot, independently select the precisely guarded Goldtrans zero Current
  rows (trader/source/currency/unit/value/material/variant and matching history
  FK/key). Require exactly the 13 gold grades plus empty-variant zahngold: **14**.
  Save those small candidate records in private evidence.
* After Current must hash exactly as **before minus those 14 rows**, in **all
  columns and rowids**. None of those keys may remain, even pointing at an old
  positive price. Every other Current row must match exactly.
* Entire `prices` History, `trader_materials` acceptance rows, and all three
  `hofmann_tin_cutovers`, `source_article_snapshots`,
  `article_acceptance_owners` ledgers must hash identically. This covers MKM and
  every other trader's historical ledger data, not just Goldtrans.
* Traders/materials require exact identities, rowids and payload hashes, except
  only explicitly owner-approved `updated_at` columns. Raw hashes and raw
  changed-column counts are still recorded; no timestamp surprise is erased.
* All other Public tables, including FTS storage, must be exact. Sequence
  before/after values are recorded and checked against the explicit policy,
  preserving sequence row identities. No old-boot FTS/sequence assumptions.
* **Entire Internal snapshot must be equal**, all tables/columns/schema/version
  and FK findings: no added runs, steps, fetches or other startup mutations.
* Known historical Public FK findings must be exactly
  `trader_materials` rowids **1070–1093**, parent `traders`, FK index 1, both
  before and after. Internal FK findings must stay empty. **Preserve, not fix**
  those 24; a different baseline blocks the run, not a repair opportunity.

`snapshot-{before,after}.json` retains full schema/layout, counts and exact
table hashes. `diff-{public,internal}.json` streams changed tables and retains
complete positional mismatch/column counts with at most 100 hash-only samples.
Samples are **positional**, not a claimed key-joined row edit script after
insertions. Exact acceptance rests on the full hashes and explicit candidate
filter, not those samples. No entire Internal row contents or secret/header
dump. `manifest.json` hashes stable evidence files; retained volatile WAL/SHM
sidecars are explicitly excluded.

## Limits / owner decisions before any run

* No artifact exists in these newly prepared inputs yet. This report establishes
  no startup, migration, resource, integrity or comparison PASS.
* Source DBs are captured sequentially: SQLite backup gives each DB a consistent
  view, **not an atomic cross-database production snapshot**. Owner must choose
  an appropriate capture boundary and attest what those sources represent.
  Live sources may change due to their own writers; this script cannot attest
  their whole-file byte stability across capture.
* `mode=ro/query_only` forbids source database/WAL writes by this SQLite
  connection. SQLite may participate in existing WAL shared-memory read-mark
  bookkeeping (or create/access sidecars if filesystem permissions allow).
  If “read-only” must include absolutely no source-directory/SHM mutation,
  owner must arrange an existing OS read-only snapshot/mount or equivalent
  externally before choosing sources. `immutable=1` is deliberately **not**
  used on potentially live/WAL-backed databases; it could lose committed WAL
  data. No such mount/snapshot/service change is performed by this script.
* Timing is a userspace watchdog, **not a hard real-time guarantee** against
  uninterruptible I/O, OS scheduling stalls, harness SIGKILL, or host failure.
  An observed >=8s lifetime fails; a process that cannot be reaped blocks
  after-backup/audit and requires owner intervention. Do not claim the 15s
  scheduler boundary was preserved after such failure. There is no cgroup,
  network namespace or filesystem sandbox: startup-source review is essential.
* RAM thresholds use the minimum of `/proc/meminfo` MemAvailable and discovered
  finite cgroup-v2 remaining budgets (mount root plus accessible membership
  ancestors). A cgroup-v1 memory controller blocks the gate pending review.
  Hidden/unmounted ancestor limits cannot be inferred; owner must confirm the
  exposed hierarchy represents the actual budget. Disk reservation is an
  estimate, not a quota; other processes can consume resources.
* Loopback health is liveness, not a complete Public read-API test. Absence of
  fallback here is proven for persisted Current keys, not every HTTP/MCP
  projection or a second boot. This script does **one** short boot, not the
  second-open/idempotence, scheduler, live fetch, Hofmann transition, MKM
  replay, deployment, or exclusive-writer gates.
* Python's SQLite must support the copied schema/FTS version. Missing features
  fail and retain evidence; the script does not rebuild SQLite or normalize
  away incompatibilities. After the selftest fix below, layout detection
  explicitly requires SQLite >=3.37 (`PRAGMA main.table_list`).
* Artifact SHA checks before/process/after do not independently prove build
  provenance or make the supplied path immutable against concurrent replacement.
  Owner must use a private stable artifact and verify the builder's manifest.
* Original server stdout/stderr are retained privately, not echoed or parsed
  into a secret dump. Owner should inspect them locally; this draft does not
  assert exact log-line counts from the old artifact.

**Next gate: owner reviews script, final-release scheduler/startup source,
artifact provenance, deployed seed binding, exact policy, source capture
semantics and resource headroom. Only then can the owner authorize an actual
private run. No production deployment/service approval follows from this plan.**

## Owner-authorized synthetic verifier selftests — 2026-10-19 follow-up

**PASS for 26 synthetic verifier test methods only: 0 failures, 0 errors,
0 skips. No artifact boot/startup/deployment PASS.** All fixtures were freshly
created synthetic databases on `/root`'s filesystem; no production DB, private
production clone, build, server, service, live HTTP or status polling involved.
The helper module was loaded through `importlib`, never through its `main`.
The test harness blocks `subprocess.Popen` and `socket.socket` during import
and tests. AST parsing and `py_compile` of the verifier were included; bytecode
is retained under the private test root, not in the repository.

Reviewable test script:
`/tmp/opencode/checkin-20261018-2330-verifier-tests.py`.

Actual command, twice (same test script; fixed verifier between runs):

```text
python3 -B /tmp/opencode/checkin-20261018-2330-verifier-tests.py
```

| Run | Concrete result | Retained evidence root |
|---|---|---|
| Before fixes | Exit 1; 26 methods, 1 failure and 4 parameter-subcase errors, 0 skips; 0.378s | `/root/checkin-2330-verifier-tests-pqkbdqcm` |
| After fixes | Exit 0; 26 methods, 0 failures/errors/skips; 0.393s | `/root/checkin-2330-verifier-tests-xa10jriv` |

Both roots are 0700. Each retains `tests.log`, `summary.json`, a SHA256
`manifest.json`, tiny synthetic SQLite fixtures and the explicitly directed
compiled verifier bytecode. Summary pre-summary/manifest file totals were
881257 and 877084 bytes respectively; combined evidence remains under 2 MiB.
Neither failed evidence nor existing user changes were discarded. SQLite used
by these tests: **3.46.1**. Parameter subcases are not inflated into extra test
method counts.

Coverage: tagged NULL/int/real/text/blob/Unicode encodings including signed zero;
repeatable hashes/snapshots and unchanged source bytes; type/payload/rowid
mutation detection; exact explicit timestamp omission without hiding payload
changes; row filtering; composite-PK WITHOUT ROWID and binary ordering; rowid
alias shadowing/fail-closed behavior; stored/virtual generated columns;
FTS5 plus all shadow tables and mutation detection; query-only plus mode=ro
write rejection (including after disabling query_only); context closure on
normal/error paths; SQLite backups, existing-destination rejection, source main
file preservation and committed-WAL inclusion/source WAL byte preservation;
minimal diff; valid/invalid/malformed policies; exact 14 candidates, guarded
mismatch rejection, missing/duplicate candidates, exact Current filtering and
old-pointer reintroduction detection with History unchanged.

### Review delta to the owner-read 611-line draft

1. **Actual layout bug fixed:** a regex over CREATE SQL incorrectly classified
   a normal rowid table whose text default contained `'WITHOUT ROWID'`. Regression
   test 11 failed. Layout now uses SQLite's `PRAGMA main.table_list` `wr` flag
   and fails closed when that metadata is unavailable. No SQL-string heuristic.
   Minimum SQLite 3.37 is now explicit.
2. **Policy validation extracted:** the existing inline checks now live in
   `validate_policy(policy)` and `main` calls it. This allows isolated tests
   without running `main`. Malformed JSON shapes previously produced TypeError
   (still caught as FAILED by main, not an acceptance bypass); checks now verify
   dict/list structure first and produce explicit RuntimeError rejection. No
   new timestamp/sequence exceptions or automatic expectations were added.
3. **No boot/runtime expansion:** server start, resource/watchdog, signal,
   scheduler, source-selection and actual DB capture logic were not changed.
   Material `updated_at` exceptions and exact catalog/sequence expectations
   remain the owner's decision from independently reviewed release inputs and
   the fresh read-only source baseline at actual run time. The synthetic
   policy example's numbers are fixture values, not production expectations.

Exact SHA256 bindings:

```text
owner-read original verifier:
e67b54aa396280003aa176107f645a2d0aa8549db2764908e541f8675f8e06eb
first selftest verifier (validation extraction, before fixes):
826d5c57de1b78299637495f1be1cfaea4208029eba1e19fea1b74b13fbfc38d
final fixed verifier:
e90f5e6149d937f9d2ebd1746895ef8487afee423ff34dc6e99984b92737ed53
unchanged test script (both runs):
31293db739e7fc2642e3cd61a06787af21a949e9dccdb415bfc293361f44a53c
passing tests.log:
e2c4ad0720bc41de36e6c7f1e3d3679437c3663174a60c06dce6f1b1c2bf8d13
passing manifest.json:
f967aa8b40bf99690cde90d97790716a7dbc55a96bcd227ccbc167fbfcf017ae
```

**Limits unchanged:** these tests do not exercise the full Public/Internal
`audit` as an integrated production-baseline gate, process cleanup/watchdog,
resource gates, Git/provenance binding, HTTP, server startup or scheduler timing.
The no-fallback selftest exercises persisted Current hash/filter rejection,
not an HTTP/MCP API. No observed actual-boot delta was used to learn expectations.
Owner must review the above small code delta before the separate actual run.
