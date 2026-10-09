# Beller handler owner — #5518

## Scope and identity

- Branch: `handler/beller-20261008`, based on `072cf25`.
- Implementation commit: `dcb07e7` (`feat(ingestion): add verified Beller Berlin purchase handler (#5518)`); five Beller-owned paths only.
- Worktree: `/root/Documents/schrott-mcp-beller-20261008` (moved off the full `/tmp` tmpfs; no foreign artifacts deleted).
- Operator: BELLER Demontagen Altmetall Schrott GmbH, Geschäftsführer Jörg Beller, HRB60757 Amtsgericht Charlottenburg. Homepage contact, actual private-customer purchase offer and Impressum all agree on Späthstraße 145, 12359 Berlin-Britz / Neukölln and 030 33 44 889; `info@beller-das.de`.
- Source: <https://www.beller-das.de/>, checked 09.10.2026. Dossier history is appended, not replaced; no scalar/address reinterpretation required.

## Parsing and coverage

`#modal-altmetall` contains the server-rendered, dynamically maintained PREISLISTE ANKAUF: no JavaScript execution or separate API required. Two columns Bezeichnung / Preis €/kg are mandatory. Contact is restricted to `#modal-impressum`. Structure/unit/operator changes fail closed; unknown labels are reported, never guessed.

33 live rows → 31 prices (30 exact, one bounded range), one acceptance-only open minimum quote, one non-material surcharge. All price units remain EUR/kg, including ferrous scrap; no tonne scaling. Exact source sub-grade labels become distinct variants, including purity, contamination, exclusions and quantity conditions.

- Kupfer Apparate: 2.40–4.40 EUR/kg, `range`, min/max retained and midpoint 3.40 only as the representative schema price.
- Thick copper peeling cable: `ab 3,40 € /kg` retained as acceptance condition and explicit price skip. Existing schema supports only exact/upto/range/approx, not an open lower-bound quote; no fake exact price or invented ceiling.
- Mischschrott: minimum **100 kg**, in variant/raw label; E-Motore: **up to 300 kg**. These are material-specific, not a global minimum/maximum for the trader.
- Zuschlag (Material zum Zerlegen): not independently attributed to any material.
- No price publication date stated: `published_at=None`. WordPress modification/certificate dates are not price dates.

## Verification

- Initial focused tests exposed only a floating-point comparison in the new range regression; corrected to tolerance, without changing parsing.
- Read-only live example: `cargo run -p schrott-mcp-ingestion --example live_handlers -- be-neukolln-beller-demontagen-altmetall-schrott`: HTTP 200, 93,873 bytes, 31 prices / one acceptance / two explicit skips, correct Berlin contact, no publication date. Millberry 10.10, Candy 9.70, Mischschrott 0.10 EUR/kg.
- Regression tests cover every mapped live grade, key uniqueness, ranges, quantities, open minimum quotes, dynamic changes, independently quoted live rows, entity/whitespace decoding, unknown rows, unrelated tables, unit/owner drift and malformed prices.
- `git diff --check`: clean.
- `cargo test --workspace`: **566 passed, 0 failed, 4 ignored** (including all three Beller regressions); log `/root/beller-workspace-tests.log`.
- Integrated dirty `main`: `cargo test -p schrott-mcp-ingestion beller`: **3 passed, 0 failed**; log `/root/beller-main-tests.log`. Foreign-file preservation checked again after compilation.
- Integrated dirty `main` live example: **HTTP 200; 31 prices / one acceptance / two intended skips**, same contact and prices as isolated worktree; log `/root/beller-main-live.log`. Status: rollout-ready; regular production scrape still pending coordinator rollout.
- Parallel sub-agent review unavailable: harness nesting depth limit (this is already a sub-agent). Local review against repository handler conventions performed instead.

## Integration / production

User steering 09.10.2026: integrate only Beller into dirty `main` in `/root/Documents/schrott-mcp`; preserve all foreign changes. **No deployment authorized in this phase.** Production has not been written to; a regular production scheduler scrape remains unverified until the coordinator deploys.

`dcb07e7` applied successfully to `/root/Documents/schrott-mcp` on `main` as **uncommitted Beller-only changes**, not a cherry-pick of foreign edits. Before/after SHA-256 checks confirm all 35 previously dirty non-registry files byte-for-byte unchanged; removing only `pub mod beller;` and `beller::handler()` reproduces the entire prior dirty registry exactly. No commit made in dirty main and no deployment performed.
