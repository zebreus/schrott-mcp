# Heraeus Goldstore Hanau — Owner integration gate, 13.10.2026

## Scope and decision

Ported the existing uncommitted candidate from `handler-owner-heraeus-20261011`
to current Main `12f77ce0315fd8c35d4211bd978e928c1db96882` in isolated branch
`integration/heraeus-20261013`. The original source qualification and historical
review are preserved in `handler-owner-heraeus-20261011.md`; its earlier lack of
integration authorization is superseded by the user's explicit worktree
completion/integration request. No dossier, GEO, scheduler, schema or shared
HTTP-policy changes. Main's 155 foreign dossier/research changes are excluded.

## Independent review and actual correction

Standards review: no blockers. The expanded semantic qualification fixture is
not an unmodified original HTML excerpt; this is a documented nonblocking
fixture-standard discrepancy, not an assertion of full original-source coverage.
Fresh original HTML was retained separately and exercised below. Optional
diagnostic improvement (`one()` attributes missing elements to the rates URL)
does not affect fail-closed behavior.

Correctness review found a P1: Impressum text in a newly inserted
`.cms-element-product-listing` subtree escaped the rates-page exemption.
The new regression genuinely failed before the fix (exit 101). Listing
exclusion now applies only to the rates page, whose unique listing is fully
validated afterward; Impressum compares all nontechnical main text. The common
exclusion predicate prevents drift between text and visibility checks. Regression
covers visible, `hidden` and inline `display:none` contradictory insertions.
Independent rereview PASS: P1 closed, no additional actionable identity bypass
or rates regression within the documented HTML/CSS evidence boundary.

## Fresh validation after the final code correction

Commands ran in the isolated current-Main worktree with `CARGO_BUILD_JOBS=1`,
`CARGO_TARGET_DIR=/root/Documents/schrott-mcp-heraeus-20261011/target`, and
`TMPDIR` at that target's `tmp` directory:

- `cargo test --offline --locked -p schrott-mcp-ingestion heraeus_goldstore_hanau::tests -- --nocapture`: **17 passed**.
- `cargo test --offline --locked --workspace`: **689 passed, 0 failed, 6 existing ignored** (Auth 4, Core 3, Ingestion 619, Query Worker 4, Server 19, Store 40).
- Workspace additionally used `HERAEUS_TEST_SOURCE_DIR` pointing to freshly
  downloaded original rates and Impressum HTML; the adversarial actual-source
  test passed. Rates SHA-256 `794f645ca105fb8675973641db9308a3fa4426fdb57cc56dd97bb7165bee1c9c`;
  Impressum SHA-256 `8647ca55813b953869b56a4aff75b3c357ef50314ca5f386558960a6e8cd94ed`.
- `cargo fmt --all -- --check`, `git diff --check`: passed.
- `python3 -B -m unittest discover -s scripts -p 'test_*.py'`: **43 passed**.
- `cargo run --offline --locked -p schrott-mcp-ingestion --example live_handlers -- he-hanau-heraeus-goldstore-hanau-argor-heraeus-de`: actual DB-free live fetch, **HTTP 200, 388224 bytes, 24 EUR/g prices, 5 conditional acceptances, `published_at=None`, alive=true**. Output explicitly checked because this example's exit code alone is insufficient.

Evidence lives in `target/heraeus-evidence/{workspace-final-20261013.log,live-final-20261013.log,source-20261013/}` until archival during worktree removal. Earlier pre-fix green results are not the final gate.

## Dependencies, deployment and limits

Only direct dependency added is `sha2 = 0.10`; already locked in the workspace,
no new package versions. Runtime needs outbound HTTPS to
`store.heraeus-gold.com` and `azweust1ieh606.blob.core.windows.net`, normal trusted
TLS roots, no redirects, 30-second per-request timeout and at most 2 MB/source.
No PDF parser or `pdftotext` runtime dependency. Reviewed PDF hashes and full
non-price text are deliberately conservative: even PDF metadata or unrelated
main-copy changes require requalification. Existing HTML/CSS visibility limits
remain; this is not browser rendering. A failed scrape creates no fresh outcome
but does not automatically retire existing old prices.

Local integration is authorized after these gates, not production publication.
Deployment requires a fresh source-linked release build including this handler,
authenticated publication, and coordinated restart. Await the regular six-hour
scheduler and verify production read-only afterward; no forced ingestion or
production DB writes. Shop operator HMG/HRA 93163 remains distinct from buyer
AHD/HRB 505728; no invented source publication date or contact/GEO enrichment.
