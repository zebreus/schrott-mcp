# SchrottJungs — current-Main integration gate, 13.10.2026

## Scope / current decision

Port of the existing uncommitted `owner/schrottjungs-20261011` candidate to
`integration/schrottjungs-20261013`, based on Main `733b6e5f` including Heraeus.
The historical source qualification, owner decisions and earlier failed reviews
remain in `handler-owner-schrottjungs-20261011.md`. User explicitly requested
completion, fresh testing, correct integration and removal of the worktree.

**Approved for local Main integration after final gates below.** Independent
current-source reviews found and then closed additional operator and embedded-CSS
defects. The earlier failures remain documented as history, not open blockers.
Publication, deployment and regular production evidence remain separate gates.

## Owner port audit

- Exactly one canonical nationwide BY handler registration, no sibling or Entel
  yard feed. Four original candidate fixtures and historical report preserved.
- Generic `HandlerOutcome.skip_price_acceptances` is default false. Only
  automatic direct and fallback acceptance writes are guarded; explicit qualified
  acceptances remain outside. No slug-specific shared policy, schema, catalogue,
  source-date inference, contact or GEO changes.
- Independent mechanical port audit: 120 existing Rust files, 122 exclusively
  `skip_price_acceptances: false` additions, no removed lines. This includes the
  freshly integrated Heraeus literal; shared recorder/registry are separately
  audited semantic changes, not counted as mechanical edits.
- All 155 preexisting foreign Main dossier/research files remain byte-identical
  to their saved pre-Heraeus fingerprints; none staged or transferred here.
- First fresh current-Main targeted baseline: 11 SchrottJungs tests passed.
  This precedes the new review corrections and is not final approval.

## Fresh review requirements

1. Operator identity must come from a unique visible actual legal block, not
   root-level script/style/JSON-LD, hidden, historical or negated name fragments.
   A newly conflicting provider statement must not be silently ignored.
2. Preserve CSS numeric argument boundaries: `scale(1 0)` must not become
   `scale(10)`; comma-alpha aliases `rgb()`/`hsl()` must detect zero alpha.
3. Escaped custom-property definitions and `var()` references must resolve to
   the same decoded identifier; embedded transparency is not an external-CSS
   exemption.
4. Repeated nesting `&` with parent selector lists must preserve cross-parent
   semantics or be conservatively rejected before expansion.
5. CSS selector expansion must be bounded before allocation/appending, not only
   after a potentially enormous intermediate string has already been built.

Final source snapshot was independently rereviewed after all writers finished.
Both Standards and SPEC passed. Targeted, Workspace, Python, rustfmt and real
DB-free live validation were repeated successfully before local integration.

### Additional rereview gates / unsuccessful Workspace evidence

The first fixes closed the five original findings, but a fresh independent
rereview found two additional gaps: globally collected opaque CSS variables
could suppress an explicit transparent fallback from an unrelated selector;
CSS-generated conflicting provider text outside the legal block escaped the
DOM-text digest. Both need genuine negative regressions and a final rereview.
Standards also require the operator fixture to preserve the unchanged original
legal-section HTML and the surrounding digest to exclude technical subtrees.
One designated writer is completing these changes; the intermediate handler
SHA `ec15f9f0bae41bf06c09caaa2075d7a49996db5a0a39e2b94c2675f12a8ab7af`
is **not** the final approved source. These findings were subsequently fixed;
the final approved source and successful replacement gates follow below.

`owner-workspace-audit-unique.log` exited **101**: Ingestion 639 passed,
one failed, seven ignored. The failed local redirect test's mock-server thread
panicked on a `BrokenPipe` while delivering the 302 response, then its join
panicked. This is not a successful Workspace gate; the test must handle only
expected early-client-close transport cases without weakening status/redirect/
private-target assertions, then the full suite must pass. A prior shared output
file was concurrently overwritten and is not used as count/completion evidence.
The separately rerun Python suite passed 43 tests, and the DB-free live example
returned HTTP 200 / 11 prices / eight skips / ten acceptances; neither overrides
these open gates. Original 129 candidate paths were hash-verified into
`/root/Documents/worktree-archive-20261013/schrott-mcp-schrottjungs-original-20261013.tar.gz`,
but the original worktree remains until correct Main integration is proven.

## Source / runtime prerequisites and limits

Fresh original HTML is retained under the original worktree's
`target/owner-evidence-20261013/`: Gewerbe 117684 bytes, SHA-256
`955a98a501523ae3ce4d27602719d20f4acd690b32fe1bdac75fc3a39e7f91c0`;
Impressum 105601 bytes, SHA-256
`fe253416d3c7127f0bacb5f969e7a2fa5ad5be293cd7aff3524c8b4db0a95eec`.

Only new direct dependency is `cssparser = 0.35`, already in the Workspace lock
graph; no new package versions. Existing `sha2` edge from Heraeus is retained.
Runtime needs trusted outbound HTTPS to `schrottjungs.de`; handler-local
no-redirect client, 30-second timeout, 4 MB response bound. No external CSS or
browser rendering, no PDF tool or geocoder required. Conservative text/CSS
qualification intentionally fails on unsupported/drifting relevant evidence.

Expected honest outcome remains 11 mapped `upto` prices from 19 original
positions, eight diagnosed catalogue skips, ten explicitly conditioned material
acceptances, no page-stated publication date. Existing observation-change date
inference remains visibly qualified in variants, not represented as source date.
Production databases stay read-only; no forced ingestion. Local Main integration
will not prove authenticated publication, deployment or regular production
scheduler success; those require separate evidence.

## Final owner gate — 13.10.2026

Approved handler SHA-256:
`4db25ee0fab4a5aa349d4e4432ff992687363aee92921a0e8ab97832635ddc10`.
Approved new Impressum fixture SHA-256:
`859c8919cc70b32b749ca300decf9f45f89baaff6c0598411022690f33ac388e`.
Its complete 2854-byte legal section is byte-for-byte present in retained fresh
source, including Bricks IDs/classes/image attributes; only the fixture's outer
main wrapper supplies synthetic test scope. Technical subtrees are not operator
proof or privacy text. CSS-generated main/ancestor context is checked. Explicit
color-variable fallbacks conservatively fail closed regardless of unscoped opaque
variable definitions; no selector applicability/browser-CSS claim is made.

Actual final commands used the existing original candidate target and private
TMPDIR with one Cargo build job, in the isolated current-Main integration tree:

- `cargo test --offline --locked -p schrott-mcp-ingestion schrottjungs::tests -- --include-ignored --nocapture`, with `SCHROTTJUNGS_EVIDENCE_DIR` at the retained fresh source directory: **25 passed, zero failed, zero ignored** (`owner-final-targeted25.log` / `owner-final25-targeted.log`).
- `cargo test --offline --locked --workspace`: **713 passed, zero failed, seven existing/explicitly source-dependent ignored**; Auth 4, Core 3, Ingestion 643, Query Worker 4, Server 19, Store 40; all four Doc-test suites complete. Clean dedicated evidence `owner-workspace-source4db25-final.log`; additionally repeated in `owner-final-4db25ee-workspace.log`. The retained-source test is exercised separately above, not assumed from an ignored result.
- `python3 -B -m unittest discover -s scripts -p 'test_*.py'`: **43 passed**.
- `cargo fmt --all -- --check`: passed. Staged source/code/docs diff check passes excluding the single unchanged original full-page HTML fixture. Its original trailing whitespace/CRLF is deliberately preserved, as required by the real-source fixture standard; the full staged whitespace check therefore reports those original lines, not a fully clean whole-tree result.
- Redirect stress: **24/24 repetitions passed across four concurrent processes** (`sole-owner-finalhash-redirect24-clean-20261013.log`); only expected BrokenPipe/ConnectionReset during mock response delivery is tolerated, with actual 302/source-identity/private-target assertions retained. Prior failed Workspace result is superseded, not hidden.
- Actual final DB-free live example: **HTTP 200, 117684 bytes, 11 prices, eight explicit skips, ten fully conditioned acceptances, `published_at=None`, empty trader info** (`owner-approved-final-live.log`). Full output inspected; example exit alone is not a gate. An initial ad-hoc count regex omitted `edelstahl-v2a` because it excluded digits; the output contains all 11 rows and the handler tests confirm the exact partition.
- Independent final SPEC and Standards rereviews: **PASS**, exact final source/fixture hashes checked, no remaining concrete blockers within the stated HTML/embedded-CSS contract. Optional display-string grade recovery remains nonblocking. Reviews performed no writes or production access and do not claim deployment approval.

No candidate code changed after this approved hash. Local integration commit
`ca654bfc` fast-forwarded Main from `733b6e5f`; all 155 foreign files remained
hash-identical. Actual `cargo test --offline --locked --workspace` directly in
Main then passed **713 tests / zero failures / seven ignored**, all Doc-tests
complete (`main-workspace-ca654bfc.log`, separately repeated in
`owner-main-ca654bfc-workspace.log`). These runs include the actual foreign
dossier inputs, without staging or discarding them.

Some `GIT_TERMINAL_PROMPT=0 git push origin main` attempts failed with
`could not read Username for 'https://github.com': terminal prompts disabled`.
A subsequent actual push succeeded (`b9e1a4d4..ca654bfc main -> main`), and a
direct `git ls-remote origin refs/heads/main` confirmed remote
`ca654bfc215765bb6837c06720741b5dcaa576b1`, matching local Main/origin at that
point. Code publication is therefore proven; persistent authentication cannot
be assumed from these mixed results. No deployment of the newer local Main was
performed or claimed. Original candidate and final
source/test/live evidence are archived with verified file hashes before removing
the completed worktrees; all original branch/commit references remain preserved.

Hourly check-in during this gate: production service active; 3881 traders,
54 materials, 2426 current prices; feedback unchanged through ID7390. Latest app
WARN is the existing Münster suspicious-price-ladder fail-closed rejection at
10:46 UTC, not a SchrottJungs production run. No production write/forced scrape.
