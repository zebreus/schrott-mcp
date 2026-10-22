# Isolated Goldtrans/MKM release build

**Final status: PASS for the bounded build, artifacts and separate post-build isolation audit.** Cargo exited **0**, elapsed **130.067 seconds**, no watchdog stop and no retry. The original controller's whole-Git-status guard **FAILED**, not PASS; its unchanged blocker and the separate evidence-based disposition are below. This is not deployment/boot approval.

## Identity and isolation

- Deployed parent: `ccd32e29d465795f52efff3f8a72c567c9089714`.
- Goldtrans: `99dde2c0913999ddb323f0adcad75b56a403b08b`.
- MKM resolved fully: `f58e9e98cab74b72173da1e6126cf11b1b9e4d51`.
- Isolated release commit: `f4bad4dd5b8a3208b952bdb23b84a849900eb6cf`.
- Input tree: `34c7dd348da90d4a175453d0494ae537b99e3b29`.
- Dedicated non-branch ref: `refs/owner-releases/goldtrans-mkm-20261018`; created with expected old zero OID, never overwriting a preexisting ref.
- Evidence root: `/root/Documents/checkin-20261018-2330-handler-release-rh4GhX5x`.
- Exact source archive: evidence root `/source`; private temporary index: `/release.index`.

Constructed using `read-tree` CCD, `update-index --index-info` for the 18 selected committed final blobs, `write-tree`, and `commit-tree` with CCD as sole parent. No branch/worktree switch, cherry-pick, reset, or real-index staging. All other paths remain CCD, including unreleased research. The report itself is intentionally new untracked documentation, not part of the release tree.

The selected path set equals both the union of runtime-scoped paths touched by the two fixes and the complete CCD→MKM diff under `crates`, `Cargo.toml`, `Cargo.lock`, and `scripts`: exactly **18** paths. CCD→Goldtrans-parent is empty in these scopes. Release→MKM is empty in these scopes. Release→CCD has exactly those 18 paths and no others. Thus no intervening runtime changes enter the release.

The full dossier tree is identical to CCD: `2f446c6fc263b72f38dc5bb485f222c5525f7ecc`. The original 34 foreign untracked paths are outside the archive and must remain untouched.

## Audit evidence

`input-manifest.json` records every release path, blob OID and Git mode, plus commit/tree identity. Both `audit-before.json` and `audit-after.json` report **4,446 files verified** against the exact commit, including complete path-set equality, regular/symlink type and executable Git mode. A fresh separate post-completion audit rederived the Git tree, checked the complete manifest against it, and rehashed every exported file/mode/path: **4,446 PASS**. Dossier identity, runtime identity against tested MKM, exact 18-path delta and release ref were checked again.

## Bounded build

Started `2026-10-18T23:31:41Z`; own shell-completion notification received before finalization. Cargo **Exit 0**; build log ends `Finished release profile [optimized + debuginfo] ... in 2m 09s`. Log confirms all six local workspace crates compiled from this isolated source, including dossier seed code generation. No missing offline dependency or compilation blocker.

```sh
/root/.cargo/bin/cargo +stable build --release --locked --offline \
  -p schrott-mcp-server -p schrott-mcp-query-worker -j 1
```

Working directory is the isolated source. `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=1`, shared `CARGO_TARGET_DIR=/root/Documents/schrott-mcp/target`; root-owned `TMPDIR` is evidence root `/tmp`, not the nearly full system `/tmp`. Toolchain: cargo `1.98.1 (797e8a9bc 2026-08-05)`, rustc `1.98.1 (48a229cea 2026-09-01)`.

Preflight: **1,860,517,888 bytes root free**, **1,801,814,016 bytes MemAvailable**; both exceed mandatory 1,400/1,200 MiB start gates. A one-second active watchdog samples `statvfs(/)` and `/proc/meminfo`; below 600 MiB root, below 320 MiB MemAvailable, or at 1,200 seconds it terminates only the owned build process group, escalating after five seconds. No retry. Full build output: `build.log`; resources: `resources.jsonl`; command, environment and result: `result.json`; reproducible controller retained as `run.py`. No cache/evidence deletion.

Actual watchdog: **131 samples**, minimum root available **1,845,235,712 bytes (1,759.754 MiB)**, minimum MemAvailable **964,292,608 bytes (919.621 MiB)**, maximum sampled elapsed **130.067263 seconds**. No threshold crossed and no process-group kill. Controller elapsed **130.067382 seconds**. After copying both private artifacts and completing original audits, final recorded root free **1,751,445,504 bytes (1,670.309 MiB)** and MemAvailable **1,750,282,240 bytes (1,669.199 MiB)**. Watchdog minima cover the Cargo interval; final resources are separately identified, not falsely included in that interval's minimum. No swap/system-TMP reliance or concurrent Cargo initiated by this agent.

## Private artifacts: freshly copied and independently rehashed

Directory: `/root/Documents/checkin-20261018-2330-handler-release-rh4GhX5x/artifacts` (private mode 0700). Both files remain unbooted and undeployed.

| Artifact | Bytes | SHA256 |
| --- | ---: | --- |
| `schrott-mcp-server` | 88005832 | `b9b5c9e01e6695585a239d2d80ba4d2022bc562ff7b7eaa9995ff389d0ecf490` |
| `schrott-mcp-query-worker` | 20780648 | `ee1bca37befb70f32009194b236447d16e492ccf44fc6b875f13059b6fc00955` |

## Original guard failure and separate disposition

Original `result.json` remains **unchanged**: blocker **`AssertionError('worktree status changed')`**. Its whole-status check ran after Cargo Exit 0, both artifact copies/hashes, and the 4,446-file post-build source audit. It must not be represented as a successful original guard, and no controller/runtime data were edited or rerun.

Fresh separate audit (`finalize.py`, `separate-final-audit.json`) finds exactly this additional untracked report relative to the original 34 foreign paths:

- `research/checkin-20261018-2330-owner.md`

No private-boot-plan report was present in that audit; none is assumed. This build report is deliberately added only after the separate audit and is an additional authorized documentation path, not a source/index change.

Separate evidence: HEAD still `f58e9e98cab74b72173da1e6126cf11b1b9e4d51`; `git diff HEAD` empty for tracked paths; actual index SHA256 still **`c4be7672165a9745293042d631410761b2b7fb47f34b691c0a0324b09a9caf4f`**; all **34 foreign file SHA256 values PASS**, all remain untracked. Boundary: this agent's foreign-content snapshot was recorded during the build, not before controller start; the agent never edited those paths. Initial status was separately recorded before preparation. New owner Markdown report explains the original whole-status change; all protected content/index checks independently pass. Disposition: **original statusguard FAILED, reconciled by separate audit**, not a rewritten PASS. No second build or new Git ref.

## Retained evidence SHA256

All paths below are under the evidence root; full list is also in `separate-final-audit.json`.

| Evidence | SHA256 |
| --- | --- |
| `run.py` (original controller) | `846706f526665c1e684254e2b5053e43034155ff59e0acb9753645734efb7c79` |
| `result.json` (original result, blocker retained) | `89d7259080e524ebbb55c59871a703c62fb13370d82842275d8e196c77638a6c` |
| `build.log` | `83358ddb64b59c7cb8b16c62b8deb2d4bf1f57d5413c11c74917b8098040908a` |
| `resources.jsonl` | `2077541163c7dd69a79d08aa646d012909265f86cf6e267353877b65445ac9ad` |
| `input-manifest.json` | `81f0cef4af25a9789611e766e7334bc7ab165e34aa91e37dd21385de6e66d4ff` |
| `audit-before.json` and `audit-after.json` (identical) | `9e335bf38618b3a3a91fc850495341e020aac768801b505456c78083d3eda71a` |
| `foreign-untracked-sha256.json` | `6572b16d6a90ba347cb012f01664189a2c03235b4d4f80ec9bcb7dc81e32ed8a` |

## Existing gates and review boundaries

No suite rerun during this bounded release build. The owner's [handler follow-up](checkin-20261018-2310-handler-followup.md) records 761 Rust PASS / 7 ignored, 52 Python PASS, Goldtrans controlled live rejection and MKM HTTP 200 with 56 numerical offers / 5 requests. This agent independently rehashed the existing logs, matching that report:

- `/root/Documents/checkin-20261018-2320-mkm-workspace.log`: `78cf4bb1a1b788bc7e30c6731c2b2f77f316b66b37f95364d410edb5db356960`.
- `/root/Documents/checkin-20261018-2310-python-suite.log`: `2f3b18d77a2d72fe02c2ad60f1f5c7c73bc46b255f56c6cb5c94e68cd26c605f`.
- `/root/Documents/checkin-20261018-2325-mkm-live.log`: `5607c07c7ce7831da91b74f881724595ebd7592b4f57a47a88da6f0fa291c811`.

These are prior gates, not newly executed tests or production recovery evidence. Separate owner agents handle production-clone review. This agent performs no service stop, deployment, private boot, production DB query/write, or push. Existing actual push attempt failed with exit 128 for missing authentication; no push or deployment success is claimed. A successful build alone does not approve rollout or prove production recovery.

## Final owner-requested confirmation (2026-10-19)

Fresh read-only checks reconfirm original result SHA256 unchanged (`89d7259080e524ebbb55c59871a703c62fb13370d82842275d8e196c77638a6c`), original blocker retained, Cargo Exit 0, exact original HEAD/index SHA256, empty tracked `git diff HEAD`, and all 34 protected foreign-content hashes unchanged. At this later check the complete additional untracked path set is exactly:

- `research/checkin-20261018-2330-handler-release-build.md` (this authorized report).
- `research/checkin-20261018-2330-owner.md`.
- `research/checkin-20261018-2330-private-boot-plan.md` (added after the earlier separate audit).

The earlier audit's one-report list remains historically correct and was not rewritten. These documentation additions do not change the release input tree or compiled artifacts. No rebuild, binary invocation, ref change or original-result edit was performed. RAM evidence is **system MemAvailable minimum**, not process peak RSS: the watchdog did not measure a compiler/process peak RSS, so none is claimed. Actual minimum available RAM and root reserve, exact source binding and artifact SHA256 values are recorded above. Final disposition remains **build/artifacts/source isolation PASS; original whole-status guard FAILED, separately reconciled**, with no deployment/production recovery claim.
