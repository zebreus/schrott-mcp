# Owner-Check-in: ELNO-Integrationsgate, 17.10.2026

## Umfang und tatsächliche Prüfung

Basis `a147b164b7a325a5f68da6b652138c408913081c` enthält die einzeln geprüfte 60er-Recherche-Welle. Neuer sauberer `git archive`-Export: `/root/Documents/checkin-20261017-combined`; darüber ausschließlich `elno.rs` und die beiden neuen ELNO-Fixturedateien. Kein Build aus dem schmutzigen Main-Verzeichnis. Die drei Overlays sind byteidentisch mit den im [Integrationsbericht](checkin-20261017-1600-elno-alias-integration.md) dokumentierten Hashes. Der Owner hat die vollständige Änderung gelesen: ein exakt begrenzter zusätzlicher Händleralias und eine Regression mit echten PDFs; keine globale Kupfer-Neuzuordnung, keine abgeschwächten Mindestanzahl-, Vorzeichen- oder Einheitenschutzregeln. Herkunft und vollständige Quellenprüfung stehen im [Diagnosebericht](checkin-20261017-1600-elno-pdf-review.md).

Der frühere externe 120-s-Abbruch wird nicht als Erfolg gewertet. Ein neuer vollständiger Lauf mit 600-s-Limit auf der jetzt zusammengeführten Basis ist regulär mit Exit 0 abgeschlossen:

- `cargo test --workspace --locked --offline`: **746 bestanden, 0 fehlgeschlagen, 7 ignoriert**; alle Suite-Ergebnisse einschließlich Doc-Tests vorhanden.
- `cargo fmt --all -- --check`: Exit 0.
- `python3 -m unittest discover -s scripts -p 'test_*.py'`: **48 bestanden**, Exit 0.
- Frisch kompiliertes `cargo run --locked --offline -p schrott-mcp-ingestion --example live_handlers -- be-spandau-elno-container-und-dienstleistungs`: Exit 0, HTTP 200, 210166 Bytes, **36 Preise**, EUR/kg, darunter Kerze 10,30; `published_at=None`, drei bekannte Skips. Keine Datenbank geöffnet, keine Produktionsingestion erzwungen.

Cargo-Umgebung: `PATH=/root/.cargo/bin:/usr/local/bin:/usr/bin:/bin`, `RUSTUP_TOOLCHAIN=stable`, `CARGO_TARGET_DIR=/root/Documents/schrott-mcp/target`, `CARGO_BUILD_JOBS=1`, `CARGO_INCREMENTAL=0`. Vorhandenes Target dient nur als Buildcache. PDF-Auswertung benötigt weiterhin `prlimit` und `pdftotext`; keine neuen Abhängigkeiten oder Rust-Geocodierung.

Logs außerhalb des Repos unter `/root/Documents/checkin-20261017-combined-`:

| Suffix | SHA-256 |
|---|---|
| `workspace.log` | `ed8383a358fb2ed65a24487ed30dc97472b11d18cc29fa7421b1c84f89db67f2` |
| `python.log` | `159056ac8bcef62923b5a0c221ea2423f9cf43c3cbae5d08d62e0d0ae1eff3a7` |
| `live.log` | `32077a1675732a71e87f0a4a6c4193fb6b45cd84eca78cc0e216ec7f0b480af2` |

## Aktueller Check-in, 16:18 UTC

- Einziger registrierter Worktree: `/root/Documents/schrott-mcp`; einziger lokaler Branch `main`, Remote-Tracking nur `origin/main` und dessen HEAD. Alte zusätzliche Remote-Branches wurden zuvor nach beidseitiger Erreichbarkeitsprüfung gelöscht. Der obige Testexport ist kein Git-Worktree.
- Alle **143** fremden offenen Pfade gegen `/tmp/opencode/checkin-current-baseline.json` erneut SHA-256-geprüft: unverändert, nicht Teil dieses Commits.
- Dienst aktiv; systemd-Warnungsabfrage ohne Einträge. App-JSON zeigt fünf Warnungen: Münster verweigert verdächtige Preisleiter, ELNO alte 35er-Zuordnung, AMR HTTP 502, Saxonia alte URL 404 und Hofmann fehlende Zeilendatumsunterstützung. Lokales Testgrün ist ausdrücklich kein Produktionsfix dieser Warnungen.
- Read-only/query-only: **3881 Händler, 54 Materialien, 2426 aktuelle Preise**. Die zehn neuesten Feedbackeinträge sind weiterhin 7390 bis 7381 vom 13.10.; keine neuen Leads seit der bisherigen Triage. Vollständiger Lesebeleg `/tmp/opencode/checkin-20261017-final-health.json`.

## Freigabe und verbleibende Releasegrenze

ELNO-Alias und Fixtures sind für den gesonderten Main-Commit freigegeben. Die 60er-Welle ist bereits `a147b164`. Der frühere Releasebuild `e8438e24` enthält weder diese Welle noch den Alias und darf nicht als neuer Stand installiert werden.

Push und Deployment sind separate Gates: normaler Push und tatsächliche Remote-Bestätigung, neuer hashgebundener Releasebuild aus genau diesem geprüften Stand, aktuelle Drift-/Backup-/Seedprüfung und nachgewiesene Writer-Exklusivität gemäß [Releaseplan](checkin-20261016-0815-hofmann-release-gates.md) bleiben vor Installation erforderlich. Kein direkter Schreibzugriff auf Produktionsdatenbanken, kein automatischer Datenbank-Restore und kein Deployment mit alten Artefakten.
