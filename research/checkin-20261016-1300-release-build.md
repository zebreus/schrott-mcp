# Gebundener Releasebuild – technisches Build-/Smoke-Gate PASS

Build und private Smokes tatsächlich am **16.10.2026**, Abschluss nach Unterbrechung am **17.10.2026, 16:01 UTC**. **Kein Deployment und keine Produktions-/Restorefreigabe.** Owner-Katalogentscheid `checkin-20261016-1300-catalog-owner-release.md` vollständig berücksichtigt; ELNO-Folgearbeit nicht untersucht oder verändert.

## Releaseinput und Schutzgrenzen

- Exakter Buildcommit und auch HEAD bei Export/Wiederaufnahmeprüfung: `e8438e24fd407c4359da7e6a61a3ffdb8a2c4c64`. Während des Berichtsabschlusses erfolgte extern der reine Owner-Reportcommit `8d0bd4932a887dc64cdb613d381ebb565628be36` (nur `research/checkin-20261017-1600-owner.md`); kein Code-/Dossierunterschied zum Buildcommit. Releasebindung bleibt ausdrücklich **e8438e24**, kein stilles Nachziehen des HEAD.
- Neue Releasewurzel **`/root/Documents/checkin-20261016-1300-release-b7w9y36u`** (nachfolgend `R`); `R/source` ausschließlich aus `git archive --format=tar <Commit>`. Alle **4.428 getrackten Blobs** gegen Git-Blob-ID und SHA256 geprüft; Dateimenge exakt, nach Build und bei Wiederaufnahme unverändert. Keine Probeinstrumentierung, keine Arbeitsbaumoverlays.
- `source-audit.json` enthält jeden Pfad/Modus/Git-Blob/SHA256/Größe. Die 143 Baseline-Dateien und alle 194 vor Build erfassten offenen Fremddateien sind weiterhin hashidentisch. Parallel hinzugekommene Wellenänderungen stehen in Statusaufnahmen, nicht im Export. Keine Dossiers geändert.
- `crates/`, `scripts/`, `dossiers/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` sind per Gitvergleich vollständig identisch zu `53a2492724db44d8354015aca2b3bf0cab6a81f1`. Zusätzlich **4.125 Dateien** dieser Bereiche byteidentisch zur tatsächlich getesteten sauberen Integrationsquelle von 08:24–08:26 UTC; `test-evidence-reuse.json` bindet deren Logs.
- Daher Workspace **745 bestanden, 7 ignoriert** und Python **48 bestanden** aus jener Integration übernommen, **nicht frisch wiederholt**. Ebenso keine neuen Live-Scrapes behauptet. Alte 03:00-Release-Smokes wurden nicht übernommen.

## Tatsächlicher Build

Sequenziell im neuen Export, `TMPDIR=/tmp/opencode`, `CARGO_BUILD_JOBS=2`, `CARGO_TARGET_DIR=/root/Documents/schrott-mcp/target`, Cargo-PATH `/root/.cargo/bin`. Vorhandenes Release-Target (782 MiB) wiederverwendet; keine neuen großen Targets, keine Bereinigung. Vor Export 5,8 GiB root/164 MiB tmp frei; nach Build rund 5,0 GiB, bei Wiederaufnahme 5.291.999.232 Bytes root frei.

```sh
cargo fmt --all -- --check
cargo build --release --locked --offline -p schrott-mcp-server -p schrott-mcp-query-worker
```

| Schritt | Tatsächliche UTC-Zeit am 16.10. | Exit |
|---|---|---:|
| fmt check | 13:07:02–13:07:03 | 0 |
| gemeinsamer Releasebuild | 13:07:03–13:08:04 | 0 |

`logs/release-build.log` belegt Neukompilation aller sechs Workspace-Pakete aus `R/source`, einschließlich Seed-Codegen aus den archivierten Dossiers; kein bloßes Umkopieren alter Artefakte. Toolchain: rustc `1.100.0-nightly (5ceaf6608 2026-09-25)`, cargo `1.100.0-nightly (3d7cf6e93 2026-09-25)`; Releaseprofil optimized + debuginfo.

## Gemeinsam gebundene Artefakte

Beide in `R/artifacts/`, Kopien byteidentisch zum jeweils frisch gebauten Targetbinary; Größen in Bytes:

| Binary | Größe | SHA256 |
|---|---:|---|
| `schrott-mcp-server` | 88408744 | `c7c4d5f7495f78d1317c9217d2a06093e35b7a8ddac75ac3202333c535ad0795` |
| `schrott-mcp-query-worker` | 20820528 | `8eb7571352f9ba722eb12a514f8b59ffc7de483cf1b2c2dfb6d4ec577b360695` |

## Private Artefakt-Smokes

**Frisch am 16.10., 13:08:17 UTC direkt mit den kopierten Binaries ausgeführt**, danach am 17.10. Hashbindungen geprüft, nicht erneut gebaut/ausgeführt. Reproduzierbarer Harness `R/artifact-smoke.py`; jeder Aufruf mit UTC-Zeit, vollständigem Kommando, Exit, Requesthash und stdout/stderr in `artifact-smoke.json` und `logs/`.

Serverhilfe sicher vor Datenzugriff: archiviertes `crates/server/src/main.rs:41,69–72` beendet `--help` mit Exit 2, **vor** Konfiguration (73), DB-Open (78/82), Seed (86) und Scheduler (112). Nur dieser Hilfeprozess lief, kein Dienst-/Listenerstart. Leeres privates cwd/HOME, keine geerbte Produktionskonfiguration; keinerlei DB im Hilfeverzeichnis angelegt.

Workerkommando jeweils `R/artifacts/schrott-mcp-query-worker R/smoke-data/worker`, Request über stdin. Ausschließlich eine neu erzeugte minimale Wegwerf-SQLite-Datei mit `smoke_marker(7)`, keine Kopie/Öffnung produktiver Daten. `PublicDb::open_read_only` nutzt `SQLITE_OPEN_READ_ONLY` (`crates/store/src/public/mod.rs:258–266`).

| Request/Prüfung | Ergebnis | Exit |
|---|---|---:|
| Server `--help` | erwartete Usage, vor DB-Open | 2 |
| `{"sql":"SELECT 42 AS marker"}` | integer `[[42]]` | 0 |
| `{"sql":"UPDATE smoke_marker SET value=99"}` | only SELECT … allowed | 1 |
| `{"sql":"SELECT 42; UPDATE smoke_marker SET value=99"}` | multiple statements … not allowed | 1 |
| `{nope` | bad request JSON | 1 |
| 65.537 ASCII-Bytes | query text too long | 1 |

DB-SHA256 vor, nach **jedem** Prozess und bei Wiederaufnahme: **`4e98e0881ca50e0fbd6e11093da18c93a0c7f685728326019647d9bf0352a8a8`**. Gesamter Smoke-Dateibaum unverändert, keine WAL-/SHM-/Journaldatei oder zusätzliche Datenbank entstanden. Die Ablehnungen belegen die getesteten Validierungsgrenzen, nicht einen hier separat erzwungenen Validator-Bypass oder die Produktions-Sandboxinstallation.

## Manifest und Nachprüfung

Alle folgenden Pfade relativ zu `R`; SHA256:

| Evidenz | SHA256 |
|---|---|
| `release-manifest.json` – Commit, Toolchain, Bindungen aller Build-/Smoke-Logs | `3445eaa11d94595cb2a6f2a39df4b8c81a6f86315b123dde34bfc074d26ff030` |
| `source-audit.json` – vollständige Blobauditliste | `74f0b9272078b721633fa933890ef63cdbb41f16c38e39b18fae47957957ee53` |
| `artifact-manifest.json` – beide Binaryhashes/Größen/Commit/Kopierzeit | `32f411417923426cbb3d1cce85ef2817e1d5d27dfed0ea59a2238999c1f3b9c5` |
| `build-results.json` – Kommandos/Zeiten/Exits/Buildumgebung | `2f4177ac86f63b9134a13009cb8778a6740a1dd619f0bf5fe5bc81135ce91c3c` |
| `logs/release-build.log` | `8adef553e7cf5523224f5a3cb6b986012d8b06a59a55cb0b87c6103df7463359` |
| `artifact-smoke.json` | `35e2a11c6aace09699583e0b5ef095784ddc9c60ec2874391e8bb346c551d51b` |
| `test-evidence-reuse.json` – keine neuen Testläufe | `8e304b86049e2082e543874267468776c95aaa63e9c3b7bbe9b01a44c8c596d3` |

`resume-audit.json`/`resume-processes.log` belegen die Wiederaufnahme am 17.10.: keine überlebenden Cargo-/Rust-Buildprozesse, alle obigen Manifest-/Log-/Artefakthashes gültig, kompletter Export unverändert, 143/194 Schutzdateien unverändert. Vorhandener Produktionsdienst nur in Prozessliste gesehen, nicht kontaktiert oder verändert. Keine Duplikatbuilds, keine Cleanupaktion. `final-audit.json` und Statusaufnahmen dokumentieren den Abschluss.

## Grenzen / verbleibende Betriebsentscheidung

**PASS ausschließlich für dieses Build-/private Smoke-Gate.** Kein Commit, Push, Installieren, Serviceeingriff, Deployment, Produktionsdatenbanköffnen oder erzwungener Ingest. Kein Nachweis erfolgreicher Remoteveröffentlichung, exklusiven Writers, Driftfreiheit oder regulärer Schedulerabnahme. Owner entscheidet das weitere Releaseverfahren; neue 60er-Welle und ELNO-Arbeit gehören nicht zu diesem Input.

Die Owner-Disposition aus dem Katalogentscheid ersetzt keine operative Backup-/Rollbackprüfung: regulärer Start kann additive DDL und Seed-/Katalogänderungen ausführen. **Alter Binarytausch ist kein Datenrollback und kann den Hofmann-Ledgervertrag umgehen.** Keine Ledgerlöschung, Downmigration, Bestandsreparatur oder Restorefreigabe; Schema-/Datenzustand und beide Binaries müssen gemeinsam betrachtet werden.
