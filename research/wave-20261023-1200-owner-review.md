# Owner-Gate — Recherchewelle 23.10.2026, 12:00 UTC

**Status:** Scope, Inhalte und Seed-Validierung geprüft. Lokal commitfähig; Push
und Redeploy sind derzeit blockiert (Details unten).

## Auswahl und Umfang

`wave-20261023-1200-scope.md` protokolliert 60 frisch und read-only aus dem
Produktionspool gezogene Slugs (`website='' OR street=''`). Der Pool umfasste
bei der Zählung um 12:08 UTC 1.933 qualifizierte Händler. Drei nach
Bundesland getrennte Shards bearbeiteten je 20 Slugs. 39 Dossiers wurden
geändert; alle liegen innerhalb der Auswahl. Keine Dossieränderung außerhalb
des Scopes. Produktionsdatenbanken blieben read-only.

## Inhaltlicher Owner-Gate

- Bestehende Recherchehistorien blieben erhalten. Alle 39 geänderten Dossiers
  haben einen datierten Eintrag; negative Suchbefunde sind als solche markiert.
- Ein vorgeschlagener Namens-/Adresswechsel im MRJ-Dossier wurde verworfen:
  APM führt aktuell Metall-Recycling-Jaretzke an derselben Adresse/Telefonspur,
  aber die Betreiber-/Nachfolgekette zum MRJ-Seed ist nicht belegt. Seed-Name,
  Adresse und leere Beschreibung bleiben unverändert; Jaretzke ist als
  Kandidat dokumentiert.
- `Marske` und `Petermann` wurden von `aktiv` auf `pruefung` gesetzt, ohne eine
  Schließung zu behaupten: die aktuelle Kundenannahme ist nicht durch eine
  qualifizierte Betreiberquelle bestätigt.
- `Lohmann` wurde anhand der Betreiberseiten (HRB 4029) präzisiert; die EGST
  bestätigt unabhängig eine Lohmann-Wertstoffhofpräsenz an derselben Adresse,
  jedoch als Containerdienst-Rechtseinheit und nicht als Metallannahme. Die
  abweichenden Öffnungszeiten bleiben offengelegt; keine Koordinate wurde
  ergänzt.
- Keine GEO-/`clear_fields`-Änderung. Ungeklärte Anbieter, Zweitstandorte und
  Kandidatenpreise bleiben ausdrücklich getrennt.
- Added-note scan: 39 datierte Notizen, alle mit schließender Klammer und
  Quellenhinweis (bei einer reinen Negativsuche ausdrücklich keine qualifizierte
  Fundstelle); kein neu hinzugefügtes U+2014-Em-Dash.
- Geprüfte Primärseiten: APM-Annahmestellenliste sowie Lohmann-Impressum und
  Wertstoffhofseite; beide bestätigen die in den Dossiers jeweils begrenzt
  zugeschriebenen Fakten. Der GLEIF-Link war beim direkten Abruf nicht
  auswertbar; die HRB-Angabe stützt sich auf das Lohmann-Impressum.

## Verifikation

- `git diff --check`: erfolgreich.
- Scope-Prüfung: 60 eindeutige Slugs; 39 geänderte Dossiers; keine Änderungen
  außerhalb des Manifests; alle ausgewählten Slugs haben ein Dossier.
- `CARGO_BUILD_JOBS=1 /root/.cargo/bin/cargo test -p schrott-mcp-ingestion seed`:
  **14 bestanden, 0 fehlgeschlagen** (683 gefiltert).

## Owner-Check-in / Produktion

Read-only-Prüfung um 12:00 UTC am 23.10.2026:

- `schrott-mcp.service` aktiv; keine systemd-Warnungen der Priorität `warning`.
- Öffentliche Datenbank: 3.893 Händler, 83 Materialien, 2.683 aktuelle Preise.
- Neuester Ingest-Lauf #2525: `ok`, 115 Preise, 0 Fehler; #2524 und #2523
  waren `partial` mit je einem Handlerfehler.
- Feedback weiterhin unverändert bis ID 7390; jüngste Einträge vom 13.10.2026.
- App-Logs enthalten u. a. Quellenvertrags-/Listenwarnungen für MKM,
  WirKaufenDeinGold, Böhner, Wahl und A-Z sowie HTTP 502 für AMR. Lokale
  DB-freie Checks für MKM, WirKaufenDeinGold und Wahl waren erfolgreich;
  AMR bleibt wegen HTTP 502/abgelaufenem Zertifikat offen. TLS wurde nicht
  umgangen. Keine GEO-Logs gefunden.

## Veröffentlichung

Ein vorhandener Release-Gate hält den Redeploy an: Der Produktionsstand
`ccd32e29` liegt weit hinter dem aktuellen `main`-Stand (43 Commits und 1.167
Dateipfade laut Check-in); der Rückstand soll zuerst als koordinierter Release
geprüft werden. Daher kein Redeploy und kein Produktions-Schreibzugriff.

Die README dokumentiert die geschützte GitHub-PAT-Injection. In dieser
Ausführungsumgebung waren `GH_TOKEN` und `GITHUB_TOKEN` nicht gesetzt,
`gh auth status` meldete keine lokale Anmeldung und `git push --dry-run origin
main` scheiterte mit „could not read Username“. Der PAT wurde weder gesucht noch
ausgegeben. Ein Commit kann lokal erstellt werden, aber Veröffentlichung ist
bis verfügbarer geschützter Credential-Injection blockiert.
