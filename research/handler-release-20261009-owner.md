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
| Main / Push | Einzelne Handler-Commits übernommen; Release `373414e` per Fast-forward in main integriert; `git push origin main` erfolgreich (`a6a0dbd..373414e`) |
| Deployment | Beide Release-Binaries am 09.10.2026 20:04:12 UTC koordiniert ersetzt; Dienst active, öffentlicher Health ok, Query-Worker SELECT erfolgreich |
| Produktions-Scrape | Vor Release alle sechs mit 0 aktuellen Preisen; ereignisbasierte Journal-Beobachtung und abschließender read-only Verifier gestartet, reguläre Fälligkeiten noch offen |

## Ressourcen und Ausgangslage

Nach Server-Erweiterung: 7.6 GiB RAM, bei Kontrolle 6.2 GiB verfügbar;
Root-Dateisystem 75 GiB, 42 GiB frei. Produktionsdienst aktiv.
Direkte SQL-Schreiboperationen in `/var/lib/schrott-mcp/*.db` bleiben verboten.
Neue Preise dürfen nur durch den regulären Ingestionpfad entstehen.

`scripts/verify-handler-release.py --since <rollout-RFC3339>` prüft beide DBs
ausschließlich mit `mode=ro`: Stepstatus/-anzahl, explizite Skips, HTTP-Fetch,
aktuelle Preise, normierte Einheiten, Unsicherheit, Quellen-URL und Datum-Basis.
`python3 scripts/test_verify_handler_release.py`: vier lokale Fixturetests
bestanden (grüner Vollbestand ohne DB-Dateiänderung sowie rote fehlende Steps,
falsche Semantik/Einheiten/Provenienz/Freshness und Datum-Basis). Diese Tests
sind kein Produktionsnachweis. Die DB speichert den Trigger-Typ nicht;
reguläre Fälligkeit/Journal und das Nichtausführen eines Force-Runs müssen
zusätzlich dokumentiert werden.

Finale Logs: `/root/Documents/handler-release-reviewed-workspace.log` und
`/root/Documents/handler-release-reviewed-live.log`. Alle sechs kompilierten
Handler liefern HTTP 200. Read-only Service-Prerequisite-Gate: root, kein
RootDirectory/RootImage, keine InaccessiblePaths/NoExecPaths; beide absolute
Executables ausführbar, prlimit-beschränkter Poppler-Versionstest erfolgreich.

Reguläre UTC-Hashphasen für den Produktionsnachweis (zuzüglich höchstens
15 Minuten Tick-Raster und sequentieller Vorgänger): AGH 20:01:06,
Hofmann 20:28:15, ELNO 22:02:05, Trapper 22:59:20, SAXONIA 23:45:15,
reGOLD 23:53:54 am 09.10.2026. Wenn der Rollout eine Phase verpasst, folgt
der jeweilige reguläre Lauf sechs Stunden später. Kein Force-Run als Ersatz.

Lokale erfolgreiche Livehandler sind **kein** Produktionsnachweis. Die offenen
Gates oben werden ausschließlich mit tatsächlichen Ergebnissen geschlossen.

## Tatsächliches Deployment

Release-Build erfolgreich nach 6m27s, Code-Stand `373414e`; spätere Commits
ändern nur Verifier/Tests und Dokumentation, keine Crates oder Dossiers im
Build-Worktree. Binaries am 09.10.2026 20:04:11–20:04:12 UTC ausgetauscht,
mit vorheriger Sicherung und gestopptem Dienst während beider atomarer
Dateiersetzungen. Backup/Manifest:
`/root/Documents/schrott-mcp-deployment-backups/20261009T200411Z-373414e/`.

- Server SHA256: `07fbc7ef44be06b7542c2d0874dcc3af8084523718e418a1ebd14c0f3c50dca4`
- Worker SHA256: `b9c7f34c1622f6b0e13986e4583426970476c1a90556ffb380906aefdb4c4b2c`
- Dienst active seit 20:04:12 UTC, MainPID 15872; keine neuen Warnungen.
- `https://schrottindex.de/health`: `{"ok":true,"service":"schrott-mcp"}`.
- Installierter Query-Worker führt read-only `SELECT count(*) FROM materials`
  erfolgreich aus: 54. Boot-Seed ergänzt die beiden Katalogmaterialien; 3881
  Händler und zunächst weiterhin 2283 aktuelle Preise. Uncommittete Recherche-
  Dossiers sind nicht eingebaut und wurden nicht angetastet.

Rollout verpasst die AGH-Phase 20:01:06 UTC: nächster regulärer Lauf
**10.10.2026 02:01:06 UTC** plus Tick/Vorgänger. Die übrigen fünf nächsten
Phasen stehen oben. Abschlussbeobachtung folgt Journalereignissen, kein
Sleep-/Statuspolling und kein Force-Ingest; danach wird
`scripts/verify-handler-release.py --since 2026-10-09T20:04:11.553005+00:00`
ausgeführt. Produktions-Evidenzdatei:
`/root/Documents/handler-release-production-evidence.json` (erst nach Abschluss).
