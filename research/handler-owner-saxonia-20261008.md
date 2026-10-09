# SAXONIA handler ownership — #5555

- Branch: `owner/saxonia-20261008`; isolated worktree from `072cf25`.
- Owner remains responsible until a successful regular production scrape.
- Production databases were not modified. Parent coordinates the serial joint rollout of server and query-worker binaries.

## Sources and semantics

Verified operator: SAXONIA Edelmetalle GmbH, Chemnitz HRB 31481, Erzstraße 9, 09633 Halsbrücke/Sachsen. Primary source: https://saxonia.de/impressum/ (also fetched and identity-checked by every scrape).

https://saxonia.de/edelmetallhandel/tageskurse/ renders current values in `.metal-buttons .metal-flex`. Use visible metal headings, not misleading button classes or historical Chart.js data. Visible `Aktueller Tageskurs vom 08.10.2026` is the publication date; WordPress modification metadata is unrelated.

Four explicit Ankauf quotes are retained (Gold, Silber, Platin, Palladium). Six verarbeitet/unverarbeitet quotes are excluded and reported, not interpreted as customer payouts. https://saxonia.de/edelmetallhandel/ describes both purchase and sale transactions on agreed prices; no guaranteed scrap/dental settlement or invented purity is claimed.

The widget uses dot-decimal EUR/kg. Divide by 1000 for catalog EUR/g. Captured 08.10.2026: Gold 117.3879, Silber 1.68499, Platin 46.41538, Palladium 30.56969 EUR/g. Raw labels retain original EUR/kg values. Empty variant, exact list quote, confidence 1.0, page-stated date.

Missing dates, incomplete four-metal purchase widgets, invalid/nonpositive values, unit changes, contradictory duplicates or changed operator/location fail closed. Unknown metals and non-Ankauf directions are skipped loudly. Standard record path preserves append-only history; no historical observations are deleted or rewritten.

## Verification and rollout

Owner recheck after the RAM expansion on 09.10.2026:
`cargo test --locked -p schrott-mcp-ingestion saxonia` passed all five tests.
The compiled Rust live handler returned HTTP 200, 217941 bytes, four purchase
prices, EUR/g, source-stated date 09.10.2026 and the verified Halsbrücke contact.
Gold 119.1458, silver 1.71390, platinum 46.89109, palladium 31.24882 EUR/g.
The six non-purchase quotes were reported as excluded. Evidence log:
`/root/Documents/saxonia-owner-recheck.log`. Workspace and rollout remain open.

Regression, full workspace and live-check results will be recorded below before handoff. Repeated harness restarts interrupted earlier compilation; they were not test failures.

Open production step: parent integrates isolated commit, rebuilds and deploys both binaries serially, then waits for the regular scheduler step for `sn-halsbrucke-09633-saxonia-edelmetalle`. Owner verifies successful step and four corresponding price/current-price observations read-only (source URL, EUR/g, page-stated publication date, raw labels, preserved history). A local DB-free live scrape does not constitute production proof.
