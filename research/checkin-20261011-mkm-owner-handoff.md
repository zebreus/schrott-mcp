# MKM owner handoff verification — 2026-10-11

Status at initial handoff: candidate received, **not integrated or production-approved**.
See the final Main-integration section below for the current state; historical
checkpoints are retained and are not retroactive approvals.

Candidate worktree: `/tmp/opencode/schrott-mcp-mkm-full-20261010`.
Baseline for the complete candidate diff: `e3fcbbae`.
Owner Main at handoff: `ceb31277`.
Patch: `/tmp/opencode/mkm-production-seam-20261011.patch`.
Verified SHA256: `4420d866ff4e258e4858e6802a992b88ac529876ea37030feba4dee42190d0d2`.

## Direct owner evidence (not implementation approval)

- `git apply --check` against current Main succeeds; no patch applied.
- Complete diff contains 27 files, including typed pipeline/recording,
  scheduler, Store, fixtures, inventory/audits and earlier review history.
- Actual workspace log `/tmp/opencode/mkm-workspace-seam-20261011.log`
  matches reported SHA256
  `61de7eee44e08855d08b9169b10b7e0459e38f70aaa6c01d973e005c1d67949d`.
  Parsed actual test result lines total **651 passed, 0 failed, 5 ignored**.
- Actual `/tmp/opencode/mkm-msg-regression-20261011.log` records 13 passing
  MSG tests and zero failures.
- Saved overview source artifact is 253,811 bytes, hash
  `0bf08f47395c3436e97c8c002b150f4bb89a38d08425599a4f33610e3a0baa7e`.
- Saved imprint artifact is 28,398 bytes, hash
  `77ca243d58b3e3127eccfb6edaf5bc17e36d0cbd5063ef166f56370e7f153f6d`.
  Both hashes and lengths independently recomputed. This verifies saved
  artifact integrity, not a new live request or all material semantics.

## Pending gates before integration

- Independent adversarial full review launched in session
  `ses_ed7951badffenPbKygXhf4hZK6`: Store/scheduler/typed recording, parser
  write boundaries, immutable history, source/producer/trader ownership,
  atomic acceptance/alias reconciliation, chronology and additive DDL.
- Owner full implementation review remains required; log/hash checks alone
  cannot substitute for it or expand the old parser-only review approval.
- Resolve findings, rerun relevant gates on the final candidate, inspect
  exact diff and raw-fixture whitespace exceptions, then integrate.
- Run Main workspace/MSG/fmt gates again before committing the implementation.
- Publication still needs working GitHub authentication. Deployment requires
  separately reviewed migration/catalog prerequisites, normal scheduler proof
  of the 60-article contract and read-only production verification afterward.

Main code and production databases are untouched by these handoff checks.
No force ingestion, GEO changes, or modifications to candidate/other worktrees.

## Independent review received: integration blocked

Reviewer report: `/tmp/opencode/mkm-adversarial-review-20261011/REVIEW.md`.
Owner read the complete report; these findings are supported by the reviewer's
own actual-library/TempDB reproductions, not yet independently rerun by Owner.
Reported green Store/MKM/MSG tests do not cover or negate these cases.

1. **P1:** First typed import of the real 12 legacy Current rows directly to
   a complete 0/60 request snapshot leaves 10 numeric Current quotes. Existing
   transition tests first import typed numeric rows and miss the cutover case.
2. **P1:** Public legacy `record_price()` with unmarked metadata can update an
   existing typed MKM history row's timestamp. Immutable protection considers
   incoming metadata but not the existing row owner.
3. **P2:** Hidden label descendants can supply expected identity text despite
   a changed visible article title.
4. **P2:** Present but invalid page-change date becomes successful NULL metadata
   rather than a parse error.
5. **P2:** Unknown tier fields can be accepted and silently discarded instead
   of rejecting an unreviewed source contract.

Implementation session `ses_ed87d0283ffetNWpWACsi5NOwD` resumed with all five
findings, mandatory red-to-green regression tests, full final gates, and an
explicit prohibition on Main edits, deployment or self-approval. Fixes must
preserve foreign/unproven provenance and legacy/MSG behavior. A new independent
review and Owner full review remain required after the fixes. The candidate
has **no integration or production approval**; no patch applied to Main.

## Owner continuation after reviewer usage-limit interruption

The F1–F5 follow-up session ended with a usage-limit error, not a completed
review or an additional code finding. The same session was resumed. Work is
split: independent reviewer owns Store/recording/overview remediation checks;
Owner inspected the full pipeline/typed dispatch/scheduler diffs and complete
scheduler control flow separately. No candidate edits or patch application.

Owner code observations:

- Only the exact MKM slug takes the typed fetch path. Non-MKM handlers retain
  their function-pointer scrape path. Typed MKM under another slug is rejected;
  untyped MKM recording is rejected. MSG still takes its existing snapshot path.
- Scheduler retains sequential timeout/error/step handling and journals the
  observation projection, but passes the complete typed outcome to recording.
  No loss of article metadata at this dispatch boundary found in code review.
- Full pipeline diff consists of 16 additive catalog entries. A local static
  slug extraction found 66 baseline / 82 candidate entries, no duplicate slugs,
  no removed old slugs; full diff showed no other pipeline changes. This is a
  catalog-structure check, not source-to-material approval.
- Candidate DDL adds `source_article_snapshots` and `article_acceptance_owners`
  via `CREATE TABLE IF NOT EXISTS`. A read-only `sqlite_master` query against
  production found neither table, verifying the stated first-creation premise
  **at this check**, not authorizing any direct agent migration. Recheck before
  coordinated deployment; normal application startup owns gated schema setup.

Owner executed the actual candidate scheduler unit-test subset with locked
dependencies and isolated existing target `/root/mkm-full-target-20261010`:
**5 passed / 0 failed** (phase stability, stagger, Berlin daily times, failing
handler isolation and legacy fallback acceptance). These are not a real MKM
HTTP→scheduler→production proof; that rollout gate remains open.

Check-in service active, warning journal empty for six hours, feedback unchanged
at 6080, counts 3,881 / 54 / 2,426. No matching geocoding logs; no prior geocoding
success inferred. GitHub authentication still absent. Main code and production
databases unchanged; MKM integration remains blocked pending completed reviews.

## Final F1–F5 review received; additional Owner F6 blocks integration

Independent review completed: `RE-REVIEW.md` now includes the successful final
143-case run, scoped F1–F5 closure, protected chronology/legacy neighbors and
complete rollback evidence. DiskFull failure and subsequent successful run
are retained separately; no failure is relabelled as a pass. The approval is
limited to reviewed Store/recording/overview fixes, not the overall rollout.

Owner subsequently read the full Store change diff, transaction/retirement
control flow, complete typed recording implementation and registered fetch
path. A new **P2 F6: missing catalog-unit normalization** was reproduced before
integration. `recording.rs` constructs `NewPrice` directly with source price,
max and unit, unlike normal recording's catalog normalization. The catalog
contains `mischschrott` and `schrott-eisen-ne-gemischt` in EUR/t; parser quotes
are EUR/kg. These two original articles are currently request-only, so this
is **not a claim of wrong numeric quotes in today's source or production**.

Owner scratch harness `/root/mkm-owner-unit-review/src/main.rs` uses actual
candidate libraries, original parsed fixture, a complete otherwise unchanged
typed snapshot, and a valid numeric MKM-SCH-02 article/price/tier contract in
place of its request. With freshly seeded isolated TempDB, `record_articles`
succeeds with 56 prices, storing **11.13 EUR/kg** against catalog **EUR/t**.
The assertion requiring normalized **11,130 EUR/t** fails (exit 101). This is
a typed-boundary/recording repro, not a claim that mutated HTML was fetched.

Implementation session resumed for F6: normalize all numeric MKM prices and
bounds into actual catalog units before transaction, preserve original source
EUR/kg prices, quantity thresholds and all tier metadata, reject unsupported
unit contracts without mutation. Required regression additionally uses mutated
fixture HTML → real parser → typed Record → TempDB for both iron articles;
remaining F1–F5/MSG/legacy and full gates must pass. No catalog-unit edits as a
workaround. Fresh patch and independent F6 review remain mandatory. No patch
has been applied to Main, and no deployment or production write occurred.

## Continued Owner parser/source review while F6 is being fixed

Owner read the complete `overview.rs` production parser (types, all 60 exact
article mappings, caption/group/row partition, visibility guards, price grammar,
date/tier parsing, numeric/request projection and missing/duplicate diagnostics),
the fixed per-article mapping oracle and complete independent Python audit.
No additional demonstrated functional blocker found in this reading. Parser
comments still describing the candidate as proof-only/without transport are
stale; update during final integration without changing historical reports.

Fresh Owner read-only HTTP requests for overview and imprint both returned
200 at their exact canonical final URLs. Saved outside the repo:

- `/root/Documents/mkm-owner-overview-20261011.html`, 253,811 bytes,
  SHA256 `0bf08f47395c3436e97c8c002b150f4bb89a38d08425599a4f33610e3a0baa7e`.
- `/root/Documents/mkm-owner-impressum-20261011.html`, 28,398 bytes,
  SHA256 `77ca243d58b3e3127eccfb6edaf5bc17e36d0cbd5063ef166f56370e7f153f6d`.
- `/root/Documents/mkm-owner-fresh-source-audit-20261011.json`: independent
  inventory of the freshly fetched overview is 8 groups / 60 articles /
  55 numeric / 5 requests; comparison of every source article contract and
  fixed material/variant oracle against prior candidate projection has no gaps.

This audit does not test catalog-normalized stored prices (F6 remains open),
the page-date parser, browser visibility or production rollout. Saved sources
are byte-identical to the previously reviewed source hashes. No re-fetch of
all 60 detail pages claimed; their semantic review evidence remains separate.

Latest check-in: service active, six-hour warning journal empty, feedback still
6080, public counts 3,881 / 54 / 2,426. Main clean before this document update;
GitHub auth absent, no matching geocoding logs. No candidate edits, duplicated
implementation work, production DB writes, forced ingestion or deployment.

## Final Owner integration gate — Main verified, production still pending

Independent `F6-REVIEW.md` received and read completely: F6 closed with actual
HTML→parser→typed-record→TempDB normalization/source-preservation checks,
unsupported/missing catalog and overflow fail-closed, immutable/ownership/
request-retirement/chronology and Legacy/MSG neighbors. This complements the
previous completed 143-case F1–F5 approval; not a production approval.

Owner read the final F6 implementation as well as full changed Store code,
typed recording and source/dispatch/parser/pipeline/scheduler paths. Existing
and new tests plus independent failure-path evidence cover all six findings.
No remaining demonstrated integration blocker. Normalized Storeprojection
is distinct from original source article prices, tiers and kg thresholds.

Reviewed final patch SHA256:
`4e399dfe83c842a270b52a76c06cdd0d3b11225b2b1cd0b2a72a8ec66a870ddd`.
Candidate index tree `a74cec01cb1bae03873b9ffa83094388978e62f9` was verified
byte-for-byte against that exact full patch relative to `e3fcbbae`. Created
isolated reviewed integration commit `0ac5bf83b0dc668c6dd8446832fb0a0579e0c076`
on `owner/mkm-reviewed-integration-20261011`, without moving the candidate
branch or consuming/changing its staged/uncommitted work. Applied that delta
to current Main with `git cherry-pick --no-commit`; no replacement of shared
files with an old Worktree snapshot, no discarded Main additions.

Only Owner follow-ups beyond reviewed functional code: correct stale parser
comments to describe the actual typed transport and include `--registered`
in example usage help, formatted as required. Core transferred files match
reviewed hashes: recording `05216f11027bd1b9209b54d9107ce3099be60f9153feff95e400bc8e84fc6736`,
Store `dedfda0e3e5cfc15935c0e8bd4dad36b9181f853e1f0bb97944794b4414d9de2`.

### Actual Main gates

- First full Main command exceeded its 120-second tool budget after building
  Main libraries and starting tests; **not counted as a successful suite**.
  Original log `/root/Documents/mkm-main-workspace-20261011.log` retained.
- Repeated foreground command with 900-second budget completed exit 0:
  `cargo test --locked --workspace`, **660 passed, 0 failed, 5 ignored**;
  `/root/Documents/mkm-main-workspace-final-20261011.log`, actual result lines
  independently totalled. Reused isolated root target, no production store.
- Separate Main MKM, MSG, seed subsets and fmt all exit 0:
  `/root/Documents/mkm-main-{mkm,msg,seed,fmt}-20261011.log`.
- All Python script tests **25 passed** (9 source-audit plus 16 rollout-verifier
  tests); this includes previously committed Owner verifier additions absent
  from the candidate baseline.
- Actual Main `mkm_inventory --registered` exit 0:
  `/root/Documents/mkm-main-registered-live-20261011.{json,log}`. Typed
  transport returns 8 groups / 60 articles / 55 prices / 5 requests, no
  diagnostics. Every article contract and fixed material/variant oracle matches
  independently fetched Owner source; this is read-only source proof, not
  production ingestion or normalized-production-DB proof.
- Code/document diffcheck green excluding only the two byte-preserved original
  raw HTML fixtures. Their original whitespace remains an explicit exception;
  no blanket full-diff whitespace claim or source-byte mutation.

Owner permits the scoped Main implementation commit after these gates.
Production is **not deployed or approved by this integration**. Remaining:
authenticated publication, coordinated both-binary deployment, recheck new
table absence/schema prerequisites and backups, then regular scheduler
recording and read-only verification of all 60 article contracts/55 prices/
5 requests, provenance/normalization/conditions/date policy and source-scoped
legacy alias retirement with history preserved. No force-ingest or direct DB
repair as a substitute; no Rust GEO and no other agent/worktree disruption.
