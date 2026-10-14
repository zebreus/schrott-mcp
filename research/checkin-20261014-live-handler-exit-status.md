# Owner-Check-in: verlässlicher Livehandler-Exitstatus, 14.10.2026

## Fehler und Korrektur

Der vorherige Check-in zeigte einen echten Prüftoolfehler: `live_handlers`
druckte `FAILED:` bei Scrape-Fehlern, gab aber Prozessstatus 0 zurück. Auch ein
unbekannter Slug wurde als leerer Erfolg behandelt. Das kann Shell-/CI-Gates
fälschlich grün machen, obwohl der Abruf scheitert.

`crates/ingestion/examples/live_handlers.rs` gibt jetzt `ExitCode` zurück:

- Status **1**, sobald ein ausgewählter Handler fehlschlägt;
- Status **1**, wenn kein Handler ausgewählt/gefunden wird;
- Status **0**, wenn mindestens ein Handler ausgewählt wurde und alle Scrapes
  erfolgreich sind. Die vorhandene Ausgabe erfolgreicher Ergebnisse bleibt.

Alle ausgewählten Handler werden weiterhin ausgeführt: ein früher Fehler
verhindert nicht die Prüfung der restlichen Handler. Ein nicht ausgewählter
fehlerhafter Handler beeinflusst den Status nicht. Fehlerausgaben gehen nach
stderr. Keine Material-/Parser-/Recording-/Scheduleränderung, keine neuen
Abhängigkeiten, keine Veränderung des Produktionstransports. Ein Handler-`Ok`
ist weiterhin sein bestehender Erfolgskontrakt, keine neue Mindestpreiszahl.

## Red/Green und reale CLI-Nachweise

Die eigentliche Ausführungsschleife wurde als `run_handlers` testbar gemacht;
die Tests rufen denselben Pfad wie `main` auf, keine parallel erfundene Statuslogik.
Ein einziger deterministischer Async-Test nutzt kontrollierte Handler ohne
HTTP oder Datenbank und prüft Fehler, unbekannten Slug, leere Registry,
Fortsetzung nach Fehler sowie Filterung und Erfolg mit Aufrufzähler.

- Red: `cargo test -p schrott-mcp-ingestion --example live_handlers` scheiterte
  nach Extraktion an **ExitCode 0 statt 1** bei kontrolliertem Scrape-Fehler
  (`/tmp/opencode/live-handlers-exit-red.log`).
- Green: derselbe explizite Beispieltest bestanden
  (`live-handlers-exit-green.log`). Standard-Workspace-Tests allein führen
  diesen Beispieltest nicht aus; er wird deshalb gesondert als Gate genannt.
- Echter Prozess: `cargo run -p schrott-mcp-ingestion --example live_handlers --
  __unknown_owner_probe__` endet nach Fehlermeldung mit Status **1**; über Shell
  ausdrücklich auf diesen Wert geprüft (`live-handlers-unknown-slug.log`).
- Echter DB-freier Limekon-Prozess: **HTTP 200, 15 Preise, 12 Skips, Status 0**;
  Zahlen und Abwesenheit von `FAILED:` zusätzlich per Ausgabeassertion geprüft
  (`checkin-live-cli-limekon.log`). Der zuvor beobachtete intermittierende
  Impressum-Verbindungsfehler ist damit nicht als behoben erklärt.
- `cargo test --workspace`: **719 bestanden, 0 fehlgeschlagen, 7 ignoriert**,
  inklusive Seedtests (`checkin-live-cli-workspace.log`), plus der oben separat
  bestandene Beispieltest. rustfmt und `cargo fmt --all -- --check` erfolgreich.

## Check-in / Owner-Gate / Releasegrenze

Feedback 7381–7390 unverändert und bereits recherchiert. Service aktiv,
öffentlicher HTTPS-`/health` erfolgreich. Keine systemd-Warnings in sechs
Stunden; separater JSON-WARN-Check enthält weiterhin Münster-Preisleiter,
AMR HTTP 502 und Huths alte Produktionsversion ohne Preistabelle. Statistik
ausschließlich read-only: **3881 Händler / 54 Materialien / 2426 aktuelle Preise**.
Keine Geo-Logs gefunden; keine Produktionswrites oder erzwungene Ingestion.

Diff-Gate: nur eigenes Beispiel plus dieser neue Bericht; keine Dossiers
geändert, daher kein neues Overwrite-/Enum-/Deep-Link-/Emdash-Feldproblem.
Alle **155** geschützten fremden SHA-256-Dateihashes unverändert. Keine Debug-
Instrumentation oder TLS-Ausnahmen. Kleiner isolierter Fix ohne nötige
parallele Dossieränderungen; bestehende Research-Agenten nicht gestört.

`gh auth status` weiterhin ohne GitHub-Anmeldung. Lokaler Commit ist keine
Veröffentlichung: Push/Redeploy bleiben offen. Das Dev-Beispiel benötigt selbst
keinen Produktionsdeploy; ausstehende Händler-/Handler-Releases bleiben separat.
