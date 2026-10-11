# Sperling Handler Ownership — 11.10.2026

## Status: independent B1/B2/B3 rereview approved; Owner Main validation passed; publication and production proof remain open

## Owner integration gate — 11.10.2026

The independent rereview in `/root/Documents/sperling-handler-independent-review-20261011.md` closes B1/B2/B3 within the documented HTML/inline-visibility and 47-label contract. Owner verified the exact reviewed handler and both fixture SHA-256 values before transferring only the five candidate files to Main; the candidate branch, index and uncommitted work remain preserved. No catalogue, seed, contact, GEO or shared Store change.

Actual Main validation used the existing root target/TMPDIR under `/root/Documents/sperling-owner-validation-20261011/`:

- `cargo fmt --all -- --check`: passed (`main-fmt.log`).
- `cargo test --offline --locked -p schrott-mcp-ingestion sperling -- --include-ignored --nocapture`: **9 passed, 0 failed**, including the actual DB-free live handler: HTTP 200, 85,290 bytes, **40 mapped / seven skipped**, block publication `2026-10-11T14:35:00+00:00` (`main-targeted-live.log`).
- `cargo test --offline --locked --workspace`: **669 passed, 0 failed, six ignored** (`main-workspace-final.log`). An earlier command hit the execution timeout; its incomplete `main-workspace.log` is not a completion proof.
- `python3 -m unittest discover -s scripts -p 'test_*.py'`: **43 passed** (`main-python.log`).

This authorizes the individual reviewed Main integration, not a production-completion claim. Publication authentication remains unavailable; the previously built pending release does not include this new handler. A new source-linked release build, coordinated deployment, and a naturally scheduled successful production run with read-only evidence remain required. Production databases were not written, and no forced ingestion was invoked. The timestamp represents publication of the entire block, not an individual Zinn recalculation; the postcode discrepancy remains unresolved.

## Independent review correction checkpoint

Read `/root/Documents/sperling-handler-independent-review-20261011.md` fully. The previously green scratch audit logs are vulnerability reproductions, NOT security passes. B1 hidden prices/contradictory direction, B2 hidden/ambiguous provider-register attribution, B3 semantic label tails/masked missing reviewed grades are genuine blockers. Previous local validation below is historical, not current owner approval.

First add explicit failing contract regression tests for all three blockers and retain RED output before implementation changes. Preserve all 47 reviewed grades (including seven unsupported originals); only additive genuinely unknown grades may skip. No shared Store schema/retirement workaround. Fixes remain handler-local with all original source/date/unit/identity and operational owner gates intact.

ENOSPC avoidance: build target `/root/Documents/sperling-owner-validation-20261011/target`, TMPDIR `/root/Documents/sperling-owner-validation-20261011/tmp`, logs alongside. No foreign artifacts deleted and no Main build artifacts used in this correction round. Workspace and actual DB-free live handler must be rerun after fixes. Ownership continues through production proof; no commit/push/deploy/subagents.

**RED reproduced before fixes:** `cargo test -p schrott-mcp-ingestion sperling::tests::b -- --nocapture` exited 101, three explicit rejection-contract tests FAILED (one unrelated existing block-date test passed). `/root/Documents/sperling-owner-validation-20261011/b1-b2-b3-red.log` lists 9 incorrectly accepted B1 mutations, 7 B2 mutations, 4 B3 mutations. These are failing assertions, not green logging-only tests. This is the pre-fix checkpoint; final results follow below.

### Applied handler-local corrections and final evidence

**B1:** Every consumed heading/table/header/date/title/priced row/cell is checked for `hidden`, `aria-hidden=true` and inline `display:none` through its ancestors; text descendants cannot hide quoted values. Inline `visibility:hidden/collapse` and `content-visibility:hidden` also reject. Only the six known non-price presentation/category rows may retain their source `aria-hidden=true`. The exact visible, once-occurring source Ankauf paragraph must be local to the bounded `#1_0` material section; all section roots must be visible, the consumed panel must have one visible exact `Preise` title, and section-local Verkauf/Nichtzahlen contradictions reject. Tests include all nine review mutations plus 35 hiding/node combinations (ancestors, tables, tbody, rows, cells, headers, dates). An intermediate post-fix test exposed an ineffective hidden-paragraph mutation after the fixture paragraph was expanded to the full actual source wording; corrected that test and added explicit `assert_ne!` mutation-effectiveness checks rather than accepting a false green.

**B2:** One wholly visible identity block, one exact Anbieter heading, one exact register paragraph, and an exact contiguous provider+register segment before Kontakt. Count provider/register/HRB/company markers to reject extra/duplicate/conflicting identity within that same block. Hidden fields/ancestors fail. All seven original identity mutations now reject, plus hiding the company name, HRB and Anbieter heading separately. `Andere GmbH / HRB 999999` cannot pass alongside the genuine block. No PLZ correction or enrichment.

**B3:** Explicit 47-original-label snapshot contract, not a minimum row count. All 47 labels must occur exactly once, including the seven unsupported originals. Modified known-label semantic tails fail before emitting an outcome, even when all original rows remain and the tailed row is additive. Masking **each of the 47** original grades with unknown `998er Altgold` fails; the specific removed `Zinn unter 90%` + new unknown reproduction fails. Genuine additive `998er Altgold` with all 47 originals intact succeeds with **40 prices + 8 loud skips**, no mapping guessed. Legitimate grade removals/condition changes require a newly reviewed handler contract; no shared Store/schema/retirement changes were made or used as substitutes.

Visibility is a bounded HTML/inline-style check, **not** a browser computed-CSS proof. External stylesheet changes are not inferred. Exact source wording/provider layout is intentionally conservative: changes require review rather than plausible inference.

Final commands from the existing worktree with `PATH=/root/.cargo/bin:$PATH`, `TMPDIR=/root/Documents/sperling-owner-validation-20261011/tmp`, `CARGO_TARGET_DIR=/root/Documents/sperling-owner-validation-20261011/target`:

- `cargo test -p schrott-mcp-ingestion sperling -- --include-ignored --nocapture`: **9 passed, 0 failed, 0 ignored**, including all three asserted blocker contracts, original source/unit/date/identity tests, genuine additive-unknown test, real local catalogue/record test (40 recorded, 7 skipped, zero canaries), and actual DB-free handler. Log `targeted-live-green.log`.
- Actual live handler: **HTTP 200, 85,290 bytes, 40 prices + 7 original skips**, price-block publication `2026-10-11T14:25:00+00:00` (16:25 Berlin). No DB initialized by that live test, no production or scheduler invocation.
- `cargo test --workspace`: **669 passed, 0 failed, 6 ignored** (ingestion 602 + other crates 67). Log `workspace-green.log`.
- `cargo fmt --all -- --check` and `git diff --check`: passed. Main read-only status check empty, HEAD still `d06f601d6de0bfe2c41fb76d4097c5722ec33759`.

All named correction-round logs live under `/root/Documents/sperling-owner-validation-20261011/`. Independent reviewer scratch files were not modified; no foreign artifacts deleted, no delegation, Main/Prod writes, commit/push or deployment. **Request Owner ReReview of B1/B2/B3, not Main integration authorization.** Deployment auth and genuine naturally scheduled successful production evidence remain blocked/open; ownership is not closed.

- Owner worktree: `/root/Documents/schrott-mcp-sperling-20261011`
- Branch: `handler/sperling-20261011`, base `d06f601d6de0bfe2c41fb76d4097c5722ec33759`.
- Slug: `hh-hohenfelde-trauschmuck-sperling`.
- No commit, push, deployment, production writes, forced ingestion, Rust GEO or delegation. Main tracked files untouched. One initial test invocation used Main's existing build target directory (build artifacts only); subsequent builds use a separate target directory.
- Ownership continues through reviewed Main rollout and a successful **real production scheduler scrape**. Local success is not completion.

## Direct evidence, not inherited assertions

Read backlog P1 in full, README source hierarchy/identity rules and dossier before implementation. Independently fetched on 11.10.2026, all HTTP 200:

1. https://www.goldankauf-ge.de/preise — 85,290 bytes, material block `Aktualisiert: 11.10.2026 13:05`, 47 original priced rows. Genuine Ankauf (`Wir zahlen ...`), EUR/g and EUR/kg. Excludes short repeated Barren table, coin/product tables and comparison table. Editorial `Zuletzt aktualisiert: 10.09.2026` is not the price date.
2. https://www.goldankauf-ge.de/impressum — 38,957 bytes, Trauschmuck Sperling GmbH, Lübecker Straße 95, 22087 Hamburg, HRB 119447 Registergericht Hamburg, Geschäftsführer Patrick Marcel Bächle.
3. https://www.northdata.de/Trauschmuck%20Sperling%20GmbH,%20Hamburg/HRB%20119447 — 179,226 bytes, same company/HRB/street/house number and Geschäftsführer; 2025 annual accounts publication 29.07.2026. Register address **22089**, operator/dossier **22087**. No address correction/enrichment emitted.
4. https://www.goldankauf-ge.de/agb — 46,817 bytes. Postankauf basis is Eingangstag; shop offer immediate, postal offer 24h; Unternehmer separately disclosed. Rücksendungsgebühr 12 EUR for unpurchased positions is not a material quote or a universal fee-free condition.

Fetch artifacts retained in `/tmp/opencode/sperling/{rates,impressum,register,agb}.html`; fixtures contain bounded source excerpts with original labels and values.

## Mapping/unit/date contract

40 mapped rows: gold 15 (two separate Barren weight bands plus 13 Altgold finenesses), zahngold 4 (colour and ceramic condition preserved), silber 11 (stamped Barren/Münzen separate from Altsilber, vergoldetes Silber retains its own label without invented fineness), platin 5, palladium 3 (999/950/**900**), zinn 2 (no invented >=90 condition for plain Zinn).

Seven unsupported originals are loudly returned as `skipped_labels`: `Ab 90er Besteck`, `Ab 90er Besteck 18/10 Edelstahl`, `Ab 90er Messer`, `Ab 90er Messer 18/10 Edelstahl`, `Unter 90er Besteck`, `Unter 90er Messer`, `Korpusware versilbert`. No silver-plated item becomes solid silver or stainless-steel scrap. No catalogue expansion guessed.

Units: all mapped precious metals EUR/g, Zinn EUR/kg, unchanged source amounts. Exact normalized original labels are variants and raw labels, preventing collapse. Global price-block timestamp is publication evidence only: source expressly excludes Zinn/Versilbertes from its five-minute refresh; never claim individual recalculation of those fixed prices. HandlerOutcome has only a global published_at field. The printed timestamp is interpreted in Europe/Berlin, converted to RFC3339 UTC; ambiguous/nonexistent DST times fail closed. Fixture `11.10.2026 13:05` becomes `2026-10-11T11:05:00+00:00`, not an invented midnight or editorial date.

Bounded UTF-8 fetch <=512 KiB, exact final source URL, validated HTTP success, isolated `#1_0` section, exact table headers and one date, positive strict decimal/unit validation, duplicate rejection, known operator identity/HRB required. No contact/address enrichment.

## Historical validation checkpoint (before independent B1/B2/B3 review)

- Initial rustfmt completed.
- First targeted run: 2 passed, 1 failed, 1 ignored. Failure is a test expectation mismatch: shared parse_de_date returns RFC3339 midnight, not `YYYY-MM-DD`; implementation will preserve actual block time with Europe/Berlin semantics instead of inventing midnight.
- Corrected the initial expectation/implementation to retain the actual block time; targeted run **5 passed, 0 failed, 1 explicitly ignored network test**.
- Local ephemeral DB/catalogue test: **40 recorded, 7 skipped, zero canaries**, all stored variants, amounts, units and publication timestamps checked. No production DB involved.
- `cargo test --workspace`: **666 passed, 0 failed, 6 ignored** (ingestion 599, other crates 67). Full output `/tmp/opencode/sperling-workspace-tests.log`.
- Actual handler function `(handler().scrape)(&client)` executed via `cargo test -p schrott-mcp-ingestion sperling::tests::live_db_free_handler -- --ignored --nocapture`: **1 passed**, HTTP 200, 85,290 bytes, **40 prices + 7 unsupported labels**, live block publication `2026-10-11T14:00:00+00:00` (printed 16:00 Berlin). No DB initialized in this test, no `record()`/pipeline/scheduler invocation. This is **not** a production scheduler scrape.
- `cargo fmt --all -- --check` and `git diff --check`: passed; Main `git status --short` empty, HEAD unchanged `d06f601d6de0bfe2c41fb76d4097c5722ec33759`.
- Source-fixture audit using Python HTMLParser: all **47 original (label, price+unit) pairs exactly equal**, including 900er Palladium and all unsupported variants. The product/control tables remain outside the bounded material section.
- Self-review completed without delegation: selector boundary excludes repeated short list/products/comparison; source and operator gates fail closed; unknown rows skip explicitly; no fineness or condition invented, no GEO/seed changes. Runtime registry contains the slug exactly once. Independent owner review remains mandatory.
- All subsequent cargo runs use `/tmp/opencode/sperling-target` (isolated artifacts).
- Final rerun after strict date-digit hardening: `cargo test -p schrott-mcp-ingestion sperling -- --include-ignored --nocapture` **6 passed, zero failures/ignored** including the actual live handler; block `2026-10-11T14:05:00+00:00` (16:05 Berlin), still HTTP 200 / 85,290 bytes / 40+7. Log `/tmp/opencode/sperling-targeted-live-tests.log`. Workspace rerun again **666 passed, 0 failed, 6 ignored**; final rustfmt and diff whitespace checks passed.

Original downloaded evidence SHA-256:

| File | SHA-256 |
|---|---|
| rates.html | `b6cd3a94dfcc1a481e5ec2ed4e5e7083c0cb1024844db4c1b8a9b83bef356066` |
| impressum.html | `ef2c462a62733813980586e6491801c1f489027b49f99181148685b15075fe41` |
| register.html | `1d2116efb3d9f5a22c9fc967b1748b2578bfd1fb273a168a4a3b6f52f95fb40e` |
| agb.html | `d02506fa9e724a475193e47c851a5dc3bd9976de0ac3d29cb22e98b8baa176be` |

## Review handoff (uncommitted worktree)

Five files only: `crates/ingestion/src/traders/handlers/mod.rs` (module + registry), `crates/ingestion/src/traders/handlers/sperling.rs`, fixtures `sperling-rates.html` / `sperling-impressum.html`, and this report. Owner can inspect the worktree before authorizing integration; no commit or push is implied by this handoff.

Reproduce with `PATH=/root/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/opencode/sperling-target` set and the worktree as working directory: `cargo test -p schrott-mcp-ingestion sperling -- --nocapture`, explicit ignored live test as above, `cargo test --workspace`, `cargo fmt --all -- --check`.

## Open mandatory owner gates

1. Owner review mapping/unit/date/identity/source evidence and isolated diff.
2. Owner integrates reviewed code into Main and publishes coordinated release (no agent authorization inferred).
3. Owner supplies authorized deployment/auth context; missing production auth is a known blocker. No deployment claimed or attempted.
4. Read-only prove deployed revision includes handler and registry once.
5. Wait for a **naturally scheduled** successful production scrape for the exact slug; prove scheduler run identifier/time/status, actual mapped/skipped counts and price source/unit/variants/date provenance. No forced production ingestion.
6. Only then close ownership; until then `reviewedMainRollout` and `erfolgreicher echterProdSchedulerScrape` remain unverified.
