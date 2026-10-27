# Owner-Follow-up - Release-Probe und Produktionsabgleich, 27.10.2026, 02:10 UTC

## Aktueller Stand

- Geprüfter Stand: `7a431dcb570c865f7bd56262b64e41341f712c26`, Tree
  `4ae7e2fc05a286fd660bca5b76d17a5dd5ac19dc`; `main` entspricht `origin/main`,
  Arbeitskopie war sauber.
- Der stündliche Produktionsabgleich blieb read-only. Es gab weder DB-Schreibzugriffe
  noch Deploy, Service-Neustart oder Force-Ingest.
- Neueste Feedback-IDs bleiben 7390 bis 7381 vom 13.10.2026; kein neueres
  Feedback.

## Live-Health und noch falsche Produktionsdaten

- `schrott-mcp.service` ist aktiv. `journalctl -p warning` meldete in den letzten
  sechs Stunden nichts; die App-JSON-Logs enthielten sechs Scrape-WARNs:
  WirKaufenDeinGold ohne validierbare Tageskursreferenzen (21:12), Wahl & Co ohne
  Sortenpreisliste (22:58), A-Z mit verworfener verdächtig gleichmäßiger
  Preisleiter (00:13), AMR HTTP 502 (00:29), MKM mit ungültigem Source Contract
  (01:29) und VHM mit Request-Fehler (01:59).
- Produktionsbestand: 3.893 Händler, 83 Materialien, 31.056 Preise und 2.715
  aktuelle Preiszeiger. Lauf #2866 war `partial` (61 Preise, ein Händlerfehler);
  #2865 war `ok` (24 Preise, kein Händlerfehler).
- Read-only bestätigt weiterhin offene, falsche Werte: Fritsch Hagen hat leere
  Straße/PLZ und den falschen Punkt 51.90933/7.88825; Elma führt weiterhin den
  alten Punkt 50.0167/10.4667 und die alte Telefonnummer; RoVa den alten Punkt
  50.86395/12.08895 und die alte Telefonnummer; RVE den alten Punkt
  49.2516/6.8587 sowie `dropoff_json={"allowed":true}`. Diese Fehler bleiben
  Korrekturaufträge, nicht als behoben gemeldet.
- Die korrigierten Seedwerte wurden in der privaten Probe bestätigt. Für Fritsch
  sind dort Oeger Str. 63, 58119, 51.34597/7.5837325 und 02334 53030 enthalten;
  Elma hat keinen GEO-/Telefonwert und Prüfstatus; RoVa behält Stadtrodaer Str. 1
  und 07548, aber keinen GEO-/Telefonwert und Prüfstatus; RVE hat keinen GEO- oder
  Annahmezusagewert und Prüfstatus.

## Release-Binaries und private Upgrade-Probe

- Exakt-HEAD-Build mit `cargo build --release --locked --offline -p
  schrott-mcp-server -p schrott-mcp-query-worker` erfolgreich.
- SHA-256 Server: `07fe7b0716b0e5e004765ed71979e5b24bda34366d0c92a77d0f60ec78c46943`.
  SHA-256 Query-Worker:
  `8fc6429aeb55c4f83b2616d2fd3eb226118abcd8833d231658bd0289110ac67e`.
- WAL-sichere read-only Backups wurden am 02:05:28 UTC aufgenommen und nur in
  privaten SQLite-Kopien verwendet. Ausgang: Schema v6, 3.893 Händler, 83
  Materialien, 31.056 Preise, 2.715 aktuelle Zeiger, 1.402
  `trader_materials`; Integritätsprüfung `ok`, 24 vorhandene FK-Verletzungen.
- Der Release-Server meldete auf der privaten Kopie `/health` HTTP 200 und
  migrierte bis v9. Der Seedlauf schrieb 1.323 Zeilen. Danach: 3.936 Händler,
  83 Materialien, 31.056 Preise, 2.659 aktuelle Zeiger, 1.402
  `trader_materials`; 323 bestehende strukturelle Zeilen geändert (1.030
  Feldänderungen), 43 Slugs neu, keine entfernt. Integritätsprüfung blieb `ok`,
  FK-Verletzungen blieben bei 24, kein aktueller Preiszeiger ohne Preiszeile.
- Der Query-Worker las die private migrierte DB erfolgreich. Ein zweiter
  Serverstart meldete `/health` HTTP 200 und schrieb null Seedzeilen. Beide
  Starts wurden vor dem Scheduler-Intervall beendet; es lief kein privater
  Scrape-Lauf. Der Beleg liegt unter
  `/tmp/opencode/release-rehearsal-7a431dcb-20261027T0206Z/manifest.json`.

## Tests, Quellenreview und Freigabegrenze

- Vollständiger Workspace-Lauf auf diesem Stand: 778 Tests bestanden, 11
  ignoriert, keiner fehlgeschlagen. Separater Seed-Lauf: 14 bestanden. `cargo
  fmt --all -- --check` und `git diff --check` waren sauber.
- Drei disjunkte unabhängige Quellenreviews der geänderten Bestandszeilen und ein
  separater Review der 43 neuen Seed-Slugs laufen noch. Owner-Dispositionen und
  endgültige Freigabe folgen nach den Ergebnissen; bis dahin ist das
  Pre-Deploy-Gate ausdrücklich offen.
- Der Post-Rollout-Verifier wurde nicht vorab ausgeführt. Er ist erst nach einem
  regulären Rollout mit dessen tatsächlichem Zeitpunkt aussagekräftig.
- GitHub-Zugriff wurde mit dem dokumentierten `gh auth setup-git`-Dry-Run
  getestet; Ergebnis `Everything up-to-date`. Es wurden keine Credentials
  angezeigt oder protokolliert.

Die notwendigen Korrekturen sind in den geprüften Seeddaten enthalten, aber noch
nicht in Produktion. Sie gelten erst nach grünem Pre-Deploy-Gate, regulärem
Rollout, normalem Scheduler-Lauf und anschließendem read-only Produktionsnachweis
als behoben.
