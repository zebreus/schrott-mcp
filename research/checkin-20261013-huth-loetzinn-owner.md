# Owner-Check-in: Huth-Lötzinn, 13. Oktober 2026

## Konkreter Fortschritt

Huths deutsche Quellenzeile „Zinnschrott, Lötzinn“ wurde bisher unter `zinn`
veröffentlicht. Der Katalog in `crates/ingestion/src/pipeline.rs` schließt
Lötwaren aus `zinn` ausdrücklich aus und führt dafür `loetzinn` (EUR/kg).
Der materialabhängige Zweig verwendet jetzt `loetzinn`; Preise, Quelllabel,
Mengenvarianten und Einheit bleiben unverändert.

Quelle: https://www.huth-recycling.de/einkaufspreise

## Diagnose und Tests

- Die deutschen und englischen Mapping-Assertions zuerst auf den richtigen
  Katalogschlüssel geändert. `cargo test -p schrott-mcp-ingestion
  huth_aschaffenburg -- --nocapture` scheiterte reproduzierbar in genau diesen
  zwei Assertions: tatsächlich `zinn`, erwartet `loetzinn`.
- Hypothesen: falscher Schlüssel im Mapping-Zweig; Verlust durch Übersetzung;
  Zeilenverschiebung im Tabellenparser. Beide Sprachvarianten scheiterten am
  gleichen Schlüssel, während das Live-Quelllabel korrekt erhalten war.
  Bestätigte Ursache: falscher Schlüssel im Lötzinn-Zweig.
- Nach Korrektur: `cargo test --workspace` erfolgreich, 715 Tests bestanden,
  7 ignoriert, 0 fehlgeschlagen. Enthält alle sieben Huth-Tests und die
  14 Seed-Tests. Log: `/tmp/opencode/checkin-huth-solder-workspace.log`.
- `cargo fmt --all -- --check` und `git diff --check` erfolgreich.
- Datenbankfreier Live-Scrape: HTTP 200, 220 Preiszeilen, davon vier Lötzinn-
  Mengenstufen korrekt unter `loetzinn`: 4,53 / 6,20 / 7,15 / 9,06 EUR/kg.
  Reinzinn bleibt separat unter `zinn`; Quellenpreisdatum 13. Oktober 2026.
  Log: `/tmp/opencode/checkin-huth-solder-live.log`.
- Keine temporäre Debug-Instrumentierung hinzugefügt.

## Feedback, Health und Schutz fremder Arbeit

- Neueste zehn Feedback-Einträge: 7390 bis 7381; gegenüber dem vorherigen
  Check-in unverändert. Ältere offene Leads werden dadurch nicht als erledigt
  gewertet.
- Dienst `active`; keine systemd-Warnungen innerhalb der letzten sechs
  Stunden. Separater App-JSON-Filter: bekannte Huth-Tabellenwarnung,
  Münster-Preisleiter-Ablehnung und erneutes AMR HTTP 502.
- Produktionsbestand read-only: 3881 Händler, 54 Materialien, 2426 Preise.
- Keine Geocoding-Logs unter `/tmp/opencode/*geo*.log` gefunden.
- Alle 155 fremden Dossier-/Recherchedateien anhand des bestehenden
  SHA-256-Manifests unverändert; nur eigener Handler-Patch und dieser Bericht
  gehören zum Commit. Kein Dossier-Overwrite. Kein Händler-Enum-, Deep-Link-
  oder Kontakt-Emdash-Feld wird geändert; Materialschlüssel existiert im
  Katalog. Mengenvarianten und Forschungshistorie bleiben erhalten.

## Veröffentlichung bleibt offen

`gh auth status` bestätigt weiterhin fehlende GitHub-Anmeldung. Der lokale
Fix ist nicht als gepusht oder deployt zu betrachten. Keine Produktions-
Schreiboperation, kein erzwungener Ingest und kein Rust-Geocode ausgeführt.
Eine spätere Produktions-Spot-Verifikation muss die vier Lötzinn-Stufen nach
autorisierter Veröffentlichung prüfen; der lokale Live-Scrape ersetzt das
nicht.
