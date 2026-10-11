# Pending release: private production-snapshot rehearsal

Build source commit: `1c8326e7` (Main clean before preparation).
Production remains the deployed six-handler release `373414e`; this document
does not authorize publication, installation or a restart.

## Preflight observations

- Service active/running, PID 15872; ExecStart `/usr/local/bin/schrott-mcp-server`,
  DATA_DIR `/var/lib/schrott-mcp`, BIND `127.0.0.1:4001`.
- Installed server hash remains
  `07fbc7ef44be06b7542c2d0874dcc3af8084523718e418a1ebd14c0f3c50dca4`;
  adjacent query-worker
  `b9c7f34c1622f6b0e13986e4583426970476c1a90556ffb380906aefdb4c4b2c`.
  Matches the existing deployment manifest, not a new release.
- PDF requirements available: `/usr/bin/pdftotext` 26.01.0,
  `/usr/bin/prlimit` util-linux 2.41.3.
- Production still lacks both new MKM ledger tables (read-only sqlite_master
  check). First-creation premise confirmed now; recheck immediately at rollout.
- No six-hour warning entries; latest feedback 6080 unchanged; public counts
  3,881 / 54 / 2,426. GitHub authentication remains missing.

## Actual isolated rehearsal

Created new private directory `/root/schrott-mcp-release-rehearsal-20261011`
(0700, snapshots 0600). Both databases captured with `sqlite3 -readonly ...
.backup`, **not ordinary file copy**. Each source is WAL-safe; the two snapshots
are not a cross-database transactional snapshot. They contain private production
data and stay outside git/public artifacts. No overwrite of an existing directory.

Owner harness `/root/mkm-owner-unit-review/src/bin/rehearsal.rs` is hardcoded
to this private directory, not production. Actual Main libraries open/migrate
the clone, run normal metadata/dossier seed and parse the saved freshly fetched
Owner primary HTML. Real MKM typed recording succeeds with 55 prices/5 requests;
bookkeeping is explicitly synthetic rehearsal evidence, not a scheduler run.

`verify-mkm-release.py` against the cloned actual schema/state with cutoff
`2026-10-11T05:31:43.357115711+00:00` passes:
**60 articles / 55 numeric / 5 requests / 55 Current / no failures**. Normal
recording therefore works on the existing production-shaped data, not only
empty TempDB fixtures. Alias/partition checks are part of this gate.

Additional read-only clone queries confirm all five explicit phone clears,
Iserbrook website clear, Schrott24 service JSON `{}`/`{}`, and Gotha branch
`An der Ostbahn 4 / 99867 / Gotha` after normal seed. No direct corrections used.
All **20 pre-typed MKM historical price rows** retained in the clone compare
field-for-field equal to the current production rows read through mode=ro.
This is concrete retained-history evidence for this rehearsal, not a guarantee
for arbitrary future contracts or unrelated handler history semantics.

No genuine production database, binary or service state changed. No forced
production ingestion, Rust GEO, or candidate/reviewer worktree changes.

## Release build in progress

Started a background locked release build of **both** server and query-worker:

```bash
CARGO_TARGET_DIR=/root/schrott-mcp-pending-release-20261011 \
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 \
  /root/.cargo/bin/cargo build --locked --release \
  -p schrott-mcp-server -p schrott-mcp-query-worker
```

Log `/root/Documents/pending-release-build-20261011.log`; completion notification
not yet received at this checkpoint. **No successful release build claimed.**
After actual completion, record binary hashes/link prerequisites and retain
the build source commit even if later documentation-only commits are added.
Do not install or restart before authenticated publication/coordinated gates.

At real rollout: fresh WAL-safe backups, both verified binaries together,
normal application schema/seed, regular scheduler phases, read-only release,
Kernschrott, MSG/correction and MKM verifiers with real rollout cutoff plus
source/journal checks. This private rehearsal does not replace those gates.
