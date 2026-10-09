# Handler-Release — Owner-Gates 09.10.2026

## Ziel und Prüfmatrix

ELNO, AGH Altgoldhandel, SAXONIA, Hofmann Rastatt und reGOLD fertigstellen,
einzeln in main integrieren und gemeinsam deployen; Trapper nur nach separater
Prüfung übernehmen. Kein Produktions-SQL-Write, keine Rust-Geocodierung,
laufende Dossierarbeit bleibt erhalten.

| Anforderung | Evidenz / noch offenes Gate |
| --- | --- |
| Vorhandene Arbeit erhalten | Original-Worktrees unter `/root/Documents/` erhalten; Release isoliert auf `owner/handler-release-20261009` |
| Kein Rollback bestehender Produktion | Bereits deployter Stand `072cf25` mit main `53d647f` konfliktfrei zusammengeführt; Merge-Commit wartet auf Workspace-Gate |
| ELNO PDF | Final 5 Regressionen, echte PDF-Fixture; Live HTTP 200, 36 Preise, 3 explizite Skips; Streaming-/Extraktionslimits und strikte Mengen-/Tokenprüfung |
| AGH | 4 Tests bestanden; Rust live HTTP 200, 8 Preise, Quellenstand 09.10.2026 |
| SAXONIA | 5 Tests bestanden; Rust live HTTP 200, 4 Ankaufspreise; 6 Nicht-Ankaufskurse explizit ausgeschlossen |
| Hofmann Rastatt | 4 eigene Tests bestanden; final Rust live HTTP 200, 23 Preise inklusive Starterbatterien, keine Skips |
| reGOLD | 6 Tests bestanden; Rust live HTTP 200, 20 indikative Preise, kein erfundenes Publikationsdatum; Historienregression `1d7a3f3` |
| Trapper | Isolierte Commits übernommen; final 7 Regressionen und HTTP 200, 21 Preise ohne Skips, approx/0.8, Quellenstand 04.08.2026 |
| PDF-Voraussetzungen | Beide: `/usr/bin/pdftotext` (poppler-utils) und `/usr/bin/prlimit` (util-linux), absolute Pfade; pdf-extract entfernt; Prozess-/Outputlimits dokumentiert |
| Rustfmt | Vor jedem Codecommit und finalen Release `cargo fmt --all -- --check` |
| Vollständige Tests | Final `cargo test --locked --workspace`: 597 bestanden, 0 fehlgeschlagen, 4 bestehende Tests ignoriert; Ingestion 540 bestanden |
| Review | Beide unabhängigen Reviews abgeschlossen; ELNO-Token-/Zeilenverlust und Prozessisolation sowie Trapper-Richtwertsemantik behoben; Follow-up bestätigt Code, Prerequisite-Doku korrigiert |
| Main / Push | Einzelne Handler-Commits übernehmen, geprüften Release in main integrieren; Push-Ergebnis dokumentieren |
| Deployment | Beide Binaries koordiniert ersetzen, Dienstgesundheit prüfen; noch nicht erfolgt |
| Produktions-Scrape | Reguläre Scheduler-Steps plus aktuelle Preise/Quellsemantik read-only nachweisen; alle sechs Trader haben vor Release 0 aktuelle Preise |

## Ressourcen und Ausgangslage

Nach Server-Erweiterung: 7.6 GiB RAM, bei Kontrolle 6.2 GiB verfügbar;
Root-Dateisystem 75 GiB, 42 GiB frei. Produktionsdienst aktiv.
Direkte SQL-Schreiboperationen in `/var/lib/schrott-mcp/*.db` bleiben verboten.
Neue Preise dürfen nur durch den regulären Ingestionpfad entstehen.

Finale Logs: `/root/Documents/handler-release-reviewed-workspace.log` und
`/root/Documents/handler-release-reviewed-live.log`. Alle sechs kompilierten
Handler liefern HTTP 200. Read-only Service-Prerequisite-Gate: root, kein
RootDirectory/RootImage, keine InaccessiblePaths/NoExecPaths; beide absolute
Executables ausführbar, prlimit-beschränkter Poppler-Versionstest erfolgreich.

Lokale erfolgreiche Livehandler sind **kein** Produktionsnachweis. Die offenen
Gates oben werden ausschließlich mit tatsächlichen Ergebnissen geschlossen.
