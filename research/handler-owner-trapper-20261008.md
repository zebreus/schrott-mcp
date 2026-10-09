# Handler-Owner Trapper (#5528)

- Owner-Branch: `owner/trapper-20261008`; Basis `072cf25`.
- Worktree: `/tmp/opencode/owner-trapper`.
- Status: Implementation vorhanden; Tests/Review laufen, noch kein Rollout.
- Gemeinsamen Rollout beider Binaries koordiniert der Parent seriell. Produktions-DBs bleiben für diesen Owner read-only.

## Primärquellen / Preissemantik

Am 09.10.2026 neu geprüft: Betreiber-Impressum Trapper GmbH, HRB 3736 Bayreuth, Am Goldenen Feld 31, Kulmbach kongruent mit erhaltenem Dossier. `https://www.trapper-kulmbach.de/service/preise` verlinkt weiterhin `/upload/preisliste-trappergmbh-anlieferung-august-2026.pdf` (SHA256 `15382e62e0ef36312f6df939133f99f738b148983f790912985267dbd4f9383d`). PDF: **Anlieferung / Ankauf**, gültig ab 04.08.2026; keine aktuellen Oktober-Tagespreise. 21 Sorten: 8 €/t, 13 €/kg. Freibleibende, mengen-/zusammensetzungsabhängige Richtwerte. Keine USt-Basis ausgewiesen, keine Verkaufspreise/Gebühren.

100 kg ist ausschließlich Vergütungsgrenze für Stahlschrott/Guss, nicht Annahmemindestmenge. Unterqualitäten von Kupferblech/Raff, Guss/Bremsscheiben, Alublech und Zink bleiben unterschiedliche Varianten. VA-Abfälle wird nicht als V2A/V4A behauptet; isolierte Rohre nicht als blankes Kupfer. Katalog additiv um expliziten Trägerschrott und Bleibatterien ergänzt, statt diese als scherengerecht oder Weichblei zu erfinden.

## Implementierung / Deployment-Gates

`trapper.rs`: HTML-Link bei jedem Lauf neu entdecken; eindeutiger HTTPS-Betreiber-PDF-Link, keine fest verdrahtete Monatsdatei. PDF über Poppler `pdftotext -layout - -` extrahieren, stdin/stdout ohne temporäre Dateien. Kindprozess wird bei Scheduler-Abbruch beendet. PDF-Identität, Anlieferungsrichtung, explizites Datum und 100-kg-Bedingung prüfen; geänderte Verträge/Einheiten/duplizierte Grades scheitern laut, unbekannte Sorten werden protokolliert. PDF-URL ist Preis-Provenienz; `record()` normalisiert €/t zu Katalog-€/kg für Motoren/Batterien/Platinen.

**Deployment benötigt `pdftotext` (Poppler)**. Hier `/usr/bin/pdftotext` vorhanden; systemd `schrott-mcp.service` läuft als root ohne Prozess-Sandbox. Vor Rollout sicherstellen, dass es auch im Service-PATH erreichbar bleibt. Keine zusätzlichen Rust-Abhängigkeiten und keine DB-Migration.

## Verifikation / Übergabe

Tests und Live-Rust-Scrape werden nach Abschluss hier ergänzt. Build-Artefakte ausschließlich `/var/tmp/opencode-trapper-target` (debug=0, jobs=1 wegen Speicherlimit). Extraktionsbelege `/var/tmp/trapper-owner-live.pdf`, `/var/tmp/trapper-owner-live.txt`.

Read-only Produktionsbaseline am 09.10.2026: Slug löst **ID 237** auf (nicht Issue-/Recherche-Nummer 5528!), 0 Preiszeilen, keine `ingestion_steps` für den Slug. Nach Rollout ausschließlich über Slug/Join verifizieren: regulären Scheduler-Step `ok`, 21 Preise, 0 Skips/Canaries, PDF-Provenienz/Datum sowie normierte Motoren 0,40 €/kg, Starterbatterien/Platinen 0,30 €/kg und getrennte Varianten. Kein manuelles Force-Ingest oder DB-Schreiben durch den Owner. Owner bleibt bis zum erfolgreichen regulären Produktionsscrape zuständig.
