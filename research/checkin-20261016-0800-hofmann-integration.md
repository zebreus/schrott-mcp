# Hofmann V3: begrenzte Main-Integration, keine Deployfreigabe

## Umfang und Bindung

Am 2026-10-16 integriert, ohne Commit/Push/Deployment. Unveränderter HEAD:
`f476ebd1750e9c28fa94d2c82af1414a40297b2c`.
Grundlage: vollständig gelesener V3-PASS-Bericht
`research/checkin-20261016-0800-hofmann-v3-review.md` und Vollpatch
`/root/Documents/checkin-20261016-0300-hofmann-upgrade-isolated/hofmann-upgrade-v3.patch`,
SHA256 `e2654e5deef3761455f666720671902298cc385cfeb08ccbd03f87010b7abfe8`.

Genau sechs bestehende und zwei neue Quell-/Lockdateien ausschließlich per
Patch-Tool übernommen. Alle acht Zielpfade waren vorher sauber bzw. abwesend;
ihre Main-Bytes sind nach Übernahme und nach Tests identisch mit manifest-v3:

| Pfad | SHA256 |
|---|---|
| `Cargo.lock` | `81fcb73b11e924fb6782426be3b2b55cfb1b2c46c92fe3049aadd09219fe4d9b` |
| `crates/ingestion/src/traders/handlers/hofmann_rastatt.rs` | `636181147a39c2c351ec7fcfa1d25ee9305c9662f9b49936a70af7738707d58d` |
| `crates/ingestion/src/traders/mod.rs` | `35b72f9a14113168b28bd1e4a5fd068d2b3197489ac037d98f88bc77b025b724` |
| `crates/store/Cargo.toml` | `83e3b2c4991faafbf48b699d9c6a1ef6f8724d9577f3bfe894c90db43b79e87e` |
| `crates/store/src/public/hofmann_tin.rs` (neu) | `4167aca01c504cdeda1e1b241a34f748dd01c1491ffb9b681bbe11127d5f09ac` |
| `crates/store/src/public/mod.rs` | `e7090aebe9afdc73ffaf5929a75e1e8aba4fd7a1874fae535fabd2155fcb565f` |
| `crates/store/src/public/prices.rs` | `4375c5357a3935f85e876f1b3bb2cbcd953eda61ee2178f810ed2fa532293363` |
| `crates/store/tests/review_hofmann_v2.rs` (neu) | `62ba825ba574a524a9582f4158db925ed009052230704c91c3077e893044af10` |

Keine unabhängigen zusätzlichen Reviewer-Dateien übernommen. Die 143
Baseline-Dateien sowie sämtliche 146 vorgefundenen fremden Status-Dateien
blieben hashidentisch. Keine Dossiers oder sonstigen Docs geändert; einzig
zusätzliche Main-Datei neben den acht Zielen ist dieser autorisierte Bericht.

## Frische integrierte Verifikation

Evidenzwurzel:
`/root/Documents/checkin-20261016-0800-hofmann-integration-evidence/`.
`source/` ist ein frischer Git-Archivexport des obigen HEAD plus ausschließlich
acht Overlays aus Main; keine offenen Fremddossiers. `export-audit.json` bindet
alle 4415 Dateien und weist genau acht Abweichungen vom HEAD nach. Sämtliche
Exportdateien blieben durch die Prüfungen unverändert.

Sequenziell 08:24–08:26 UTC ausgeführt, alle Exitcodes 0:

| Kommando im Export | Ergebnis | Log unter `logs/` |
|---|---|---|
| `cargo fmt --all -- --check` | PASS | `rustfmt.log` |
| `cargo test --workspace --locked --offline` | **745 bestanden, 0 fehlgeschlagen, 7 ignoriert**; Store 43, integrierte Reviewer 8 | `workspace.log` |
| `cargo run --locked --offline -p schrott-mcp-ingestion --example live_handlers -- bw-rastatt-hofmann` | HTTP 200, 23 Preise, Zinn alt 16500 EUR/t; Zeilendaten 2026-10-14 und 2026-09-30 | `live-hofmann.log` |
| `cargo run --locked --offline -p schrott-mcp-ingestion --example live_handlers -- sn-halsbrucke-09633-saxonia-edelmetalle` | HTTP 200, 4 Ankaufspreise, Confidence 0.8, Datum 2026-10-16; 6 nicht explizite Ankaufskurse ausgelassen | `live-saxonia.log` |
| `python3 -B -m unittest discover -s scripts -p 'test_*.py'` | **48 bestanden** (aktueller HEAD enthält mehr als die früher erwarteten 39) | `python-verifiers.log` |

Umgebung: `TMPDIR=/tmp/opencode`, `CARGO_BUILD_JOBS=2`,
`CARGO_TARGET_DIR=/root/Documents/checkin-20261016-0300-hofmann-upgrade-isolated/target`,
`PYTHONDONTWRITEBYTECODE=1`, Cargo-Pfad `/root/.cargo/bin` ergänzt.
Ein erster Aufruf scheiterte vor Prozessstart an fehlendem Cargo im PATH;
danach vollständige reale Prüfserie wie oben, keine Installation.
Das geprüfte Beispiel ruft ausschließlich den Scrape-Pfad auf und öffnet keine
Datenbank; es beweist keine Produktions-Pointerbewegung. Tests verwenden
Wegwerfdatenbanken. Keine Produktionsdatenbank geöffnet, keine erzwungene
Ingestion, kein Serverstart/Neustart und kein Build fremder laufender Prozesse.

Nachweise: `preflight.json`, `main-status-before.txt`, `main-status-after.txt`,
`export-audit.json`, `test-results.json` (Kommandos/UTC-Zeiten/Exitcodes),
`final-audit.json` (Hashes, Schutzprüfung, `git diff --check` Exit 0).
Die dort kopierte `manifest-v3.json` hat SHA256
`1ed6531a3cf7df9eaded4507bd93d604edf4ab4ee3d77e162b4c2a709989eb05`.
Log-SHA256 stehen vollständig in `final-audit.json`.

## Persistenz-, DDL- und Rollbackgrenze

Beim späteren schreibfähigen PublicDb-Öffnen entsteht additiv die Tabelle
`hofmann_tin_cutovers`; `user_version` bleibt 6. Öffnen allein retiret nichts
und schreibt keine historischen Preiszeilen um. Einmalige, transaktional
gebundene Legacy-Prüfung entfernt ausschließlich den bewiesenen alten
Current-Pointer; Historie bleibt erhalten. Explizite Koexistenz einschließlich
Anfragezeilen wird dauerhaft konservativ gespeichert. Spätere echte Sorten
werden nicht pauschal gelöscht. Guard und Pointer verwenden dieselbe strikt
kalendarisch validierte UTC-Ordnung, auch gegen vorhandene andere Schreibweisen.

**Binary-Rollback ist kein Datenrollback.** Alte Writer umgehen das Ledger;
V2 bringt zusätzlich die Zeitordnungsfehler zurück. Kein gemischter Writerbetrieb.
DDL-/Backup-/Restore- und konsistenter Binarywechselplan sind vor jedem Rollout
separat vom Owner festzulegen. Hier weder Produktionsmigration noch Rollout
ausgeführt oder freigegeben. Owner entscheidet Commit erst nach Diff und Tests.
