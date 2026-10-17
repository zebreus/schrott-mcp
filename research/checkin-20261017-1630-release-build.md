# Hashgebundener Releasebuild – Build-/private Smoke-Gates PASS

Tatsächlich ausgeführt am **17.10.2026, 16:31–16:38 UTC**. **Kein Deployment und keine Produktions-/Restorefreigabe.** Gelesene Vorgaben: [alter Releasebericht](checkin-20261016-1300-release-build.md), [Owner-Abschluss](checkin-20261017-1618-owner-final.md) und [verbleibende Releasegates](checkin-20261016-0815-hofmann-release-gates.md).

## Exakter Input und Schutzgrenzen

- Buildcommit ausschließlich **`ccd32e29d465795f52efff3f8a72c567c9089714`**, Git-Tree `f5d11433b11900c972e37d9926cf77f8e177753b`. Kein Nachziehen eines späteren HEAD.
- Releasewurzel **`/root/Documents/checkin-20261017-1630-release-a_8a8jlo`**, nachfolgend `R`. `R/source` stammt ausschließlich aus `git archive --format=tar <Commit>`, ohne Worktree oder Arbeitsbaumoverlay. Das originale Archiv liegt als `R/source.tar` vor.
- **4.440 Dateien**: vollständige Dateimenge, Git-Blob-ID, Modus, Größe und SHA256 geprüft. Export nach Build und Smokes unverändert. `source-audit.json` enthält die vollständige Liste.
- Alle **143 fremden Baselinepfade** gegen `/tmp/opencode/checkin-current-baseline.json` vor und nach Ausführung SHA256-identisch. `protected-before.json`, `protected-after.json` und Statuslogs binden den Schutzbeleg. Keine dieser Änderungen in den Export übernommen.
- Buildrelevante Bereiche `crates/`, `scripts/`, `dossiers/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` gegenüber Ownercommit `8b6e4770` ohne Gitdiff. Zusätzlich **4.127 Dateien** dieser Bereiche byteidentisch zur tatsächlich getesteten Owner-Integrationsquelle `/root/Documents/checkin-20261017-combined`.
- Deren vorhandene Workspace-/Python-/Live-Logs stimmen mit den drei Owner-SHA256 überein und sind in `integration-evidence.json` gebunden. **746 Tests bestanden, 7 ignoriert; Python 48 bestanden; ELNO-Liveprobe 36 Preise** sind übernommene Integrationsbelege, **hier nicht erneut ausgeführt**. Kein alter Release-Smoke übernommen.

## Tatsächlicher Build

Cargo absolut `/root/.cargo/bin/cargo`, `CARGO_BUILD_JOBS=1`, `CARGO_INCREMENTAL=0`, `TMPDIR=/tmp/opencode`, `CARGO_TARGET_DIR=/root/Documents/schrott-mcp/target`. Ausschließlich bestehendes Target als Cache, kein neues großes Target und kein Cleanup. Vorher root etwa 3,7 GiB/tmp 113 MiB frei; nach Abschluss root **2.266.972.160 Bytes** frei.

Bewusster Toolchain-Override **`RUSTUP_TOOLCHAIN=stable` wie Owner-Integration**, obwohl die unveränderte archivierte `rust-toolchain.toml` nightly vorgibt: rustc `1.98.1 (48a229cea 2026-09-01)`, cargo `1.98.1 (797e8a9bc 2026-08-05)`. Kein separater Nightly-Releasebuild behauptet.

```sh
/root/.cargo/bin/cargo fmt --all -- --check
/root/.cargo/bin/cargo build --release --locked --offline -p schrott-mcp-server -p schrott-mcp-query-worker
```

| Gate | Tatsächliche UTC-Zeit | Exit |
|---|---|---:|
| rustfmt check | 16:31:25–16:31:26 | 0 |
| gemeinsamer Releasebuild | 16:31:26–16:38:24 | 0 |

Build mit 2400-s-Limit im Hintergrund regulär beendet, kein Timeout und kein Polling. `logs/release-build.log` belegt Neukompilation aller sechs Workspacepakete aus `R/source`, inklusive Seed-Codegen aus den archivierten Dossiers. Fertig nach 6m58s, Profil optimized + debuginfo; nicht bloß alte Binaries kopiert.

## Gemeinsam gebundene Artefakte

Um **16:38:29 UTC** nach `R/artifacts/` kopiert, byteidentisch zum jeweiligen frisch gebauten Targetbinary:

| Binary | Bytes | SHA256 |
|---|---:|---|
| `schrott-mcp-server` | 87990736 | `7de235b153c571d2ed346dbb39fe503003ac20edec0ff9fc9bf0bad85b595c34` |
| `schrott-mcp-query-worker` | 20780656 | `854424a12fb80478731e65367fc7844dca71cab103f2594a2ecd9dafd20786ec` |

## Frische private Artefakt-Smokes

Alle **16:38:29 UTC** direkt mit diesen kopierten Binaries ausgeführt. Reproduzierbarer Harness `R/artifact-smoke.py`; Kommandos, UTC-Zeiten, Exits, Requesthashes und stdout/stderr in `artifact-smoke.json` und `logs/`.

Server ausschließlich `--help`, im leeren privaten cwd mit bereinigter Umgebung. Der geprüfte archivierte Code beendet diesen Pfad mit Exit 2 (`crates/server/src/main.rs:41,69–72`), **vor** Konfiguration (73), DB-Open (78/82), Seed (86) und Scheduler (112). Kein Listener-/Dienststart und keine dort angelegte Datenbank.

Worker ausschließlich mit `R/smoke-data/worker`, einer **neu erzeugten minimalen Wegwerf-SQLite-Datei** mit `smoke_marker(7)`, keine Produktionskopie. `PublicDb::open_read_only` verwendet `SQLITE_OPEN_READ_ONLY` (`crates/store/src/public/mod.rs:258–266`).

| Request/Prüfung | Ergebnis | Exit |
|---|---|---:|
| Server `--help` | erwartete Usage, Abbruch vor DB-Open | 2 |
| `{"sql":"SELECT 42 AS marker"}` | integer `[[42]]` | 0 |
| `{"sql":"UPDATE smoke_marker SET value=99"}` | only SELECT … allowed | 1 |
| `{"sql":"SELECT 42; UPDATE smoke_marker SET value=99"}` | multiple statements … not allowed | 1 |
| `{nope` | bad request JSON | 1 |
| 65.537 ASCII-Bytes | query text too long | 1 |

DB-SHA256 vor, nach **jedem** Prozess und im Abschlussaudit: **`4e98e0881ca50e0fbd6e11093da18c93a0c7f685728326019647d9bf0352a8a8`**. Gesamter privater Smoke-Dateibaum unverändert; keine WAL-/SHM-/Journaldatei oder zusätzliche Datenbank. Die Ablehnungen belegen diese Validatorgrenzen, keinen separat erzwungenen Validator-Bypass oder installierte Produktionssandbox.

## Manifest und Abschlussaudit

Pfade relativ zu `R`; SHA256:

| Evidenz | SHA256 |
|---|---|
| `release-manifest.json` – Commit/Tree, Archiv, Artefakte, Harness, sämtliche Logs und private DB | `6e2b5ea04a4ff7d3cccfc1f1cb94583c5cde27d7ef217d33e0311c728b20a113` |
| `source.tar` | `98c6009e13891ec5a0ec166f2edca3335c7c8da637e62698c4daa522e0e84387` |
| `source-audit.json` | `81cda43ac979bbc6a123161d873daf6b96aabe6c06b7dcd51fe4a5a5966b3615` |
| `artifact-manifest.json` | `1b4fd8d1cf0eae7828a71e16471c5e934375b85dd6a6f7781c6230cbe537d90e` |
| `build-results.json` | `de6fbb4ab7dfb8c5a7ce247d90d90a839412b1d362026bfa8c50c53f4972451c` |
| `logs/release-build.log` | `a57b30f0669b3a9a4e757e00d82ed60dd2178953d468c2d30b925cf004f82bb7` |
| `artifact-smoke.json` | `11cb16a99a6359f7679bc766e644fae86f75c4d7dcf0fde9ac4922c98e3aece0` |
| `integration-evidence.json` | `86c30348673daf4b327322591f46a3e3103598b07c0292ff1dcd4da6d3854d14` |
| `final-audit.json` | `143b41b86c39780a03b9687f887bd1e80480f204b28735cbb4d38b0a3854d362` |

Abschlussaudit **16:38:30 UTC**: 4.440 Quelldateien, 143 Schutzdateien und privater Smoke-Dateibaum unverändert, sechs Smokes bestanden. Anschließend alle im Manifest gebundenen Dateien erneut SHA256-geprüft: gültig. Driver/Finalizer sind unter `R` mitgebunden. Früher erstellter Unterbrechungshinweis `/root/Documents/checkin-20261017-1630-release-HANDOFF.md` ist durch diesen erfolgreichen Abschluss überholt: **nicht erneut bauen oder Finalizer wiederholen**.

## Verbleibende Blocker / keine Betriebsfreigabe

**PASS ausschließlich für diesen Build und die privaten Artefakt-Smokes.** Keine Produktionsdatenbank geöffnet, kein Serverdienst gestartet, kein Service geändert, keine Installation, kein Commit/Push. Im Repo ausschließlich dieser Bericht geschrieben; Buildcache und eigene Dateien liegen außerhalb des Quellinputs.

- Der Buildagent hat weder Push noch Remoteprüfung ausgeführt. **Nachtrag Owner:** Der frühere Push mit gh-Credential-Helper war erfolgreich; nach Eingang dieses Berichts bestätigt ein neues direktes `git ls-remote --heads origin main` exakt den Buildcommit `ccd32e29d465795f52efff3f8a72c567c9089714`. Die Remoteveröffentlichung dieses Releaseinputs ist damit belegt. Der spätere reine Check-in-Bericht `69ed0ba1` ist noch nicht gepusht; eine zwischenzeitlich fehlende gh-Anmeldung betrifft diesen Bericht, nicht die Erreichbarkeit des gebauten Commits.
- Aktuelle Driftprüfung, WAL-konsistente Sicherung beider Produktionsdatenbanken samt Altbinaries sowie vollständiger Boot-/Seed-/Hofmannübergang auf akzeptierter privater Kopie sind **hier nicht durchgeführt**. Diese minimalen Smokes ersetzen sie nicht.
- Writer-Exklusivität, genehmigte Installation, Health und regulärer Schedulerlauf sind **nicht nachgewiesen**. Kein lokales Testgrün als Produktionsfix ausgegeben.
- Alter Binarytausch ist kein sicherer Datenrollback und kann den Hofmann-Ledgervertrag umgehen. Keine Ledgerlöschung, Downmigration, Bestandsreparatur oder Restorefreigabe; Schema-/Datenzustand und beide Binaries bleiben gemeinsam zu betrachten.

Owner-Nachprüfung nach Eingang: Manifest-SHA256 gegen die obige Bindung geprüft, sämtliche **37** darin gelisteten Dateien einschließlich beider Releasebinaries erneut auf Größe und SHA256 verifiziert. Tatsächliche `build-results.json` und vollständige `artifact-smoke.json` gelesen: alle ausgewiesenen Exits und privaten DB-Erhalt bestätigt. Kein neuer Build/Smoke gestartet. Private Driftprüfung läuft unabhängig weiter; noch keine Installationsfreigabe.
