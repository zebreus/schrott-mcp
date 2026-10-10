# Gutzmann: Owner-Fallback und Standortfix

Die beiden Agenten für Gutzmann-Review und MSG-Staffelplan scheiterten am Nutzungslimit. Keine erfolgreiche unabhängige Abnahme daraus abgeleitet; MSG bleibt offen. Der Owner hat den vorhandenen Gutzmann-Diff im Deploy-Worktree selbst gelesen und den eng begrenzten Standortfix in Main umgesetzt. Quellworktree unverändert.

## Reproduktion und Fix

`cargo test --locked -p schrott-mcp-ingestion traders::handlers::gutzmann::tests::impressum_extracts_contact` zuerst rot: `assertion failed: info.street.is_empty()`. Die echte Parser-Fixture liefert Tottlebener Hauptsitzdaten für den Gotha-Handler. Frühere Diagnose (`checkin-20261008-sdm-price-coverage.md` im Deploy-Worktree) hatte den Extraktionspfad und Store-Überschreibpfad bereits eingegrenzt; deshalb keine erneute breite Hypothesensuche.

Fix: Impressum liefert nur Telefon/E-Mail. Straße/PLZ/Ort bleiben leer und überschreiben damit nicht die Dossieranschrift. Fehlende Anker und fehlende Telefon-/E-Mail-Daten bleiben Fehler; zusätzliche Fax-/Adress-only-Regression. Kein Mapping-, Preis-, Store- oder GEO-Code geändert. Bestehende Quellenidentität nicht durch neue Firmen-Merges ersetzt.

Kontaktseite erneut direkt gelesen: https://www.sdm-gutzmann.de/kontakt/ nennt Tottleben als Büro/Buchhaltung, Hauptstraße 46, 99947; Gotha als Recyclinghof, An der Ostbahn 4, 99867, Telefon 03621 733 4660/4661. Diese Betreiber-Einzelquelle bestätigt Standortabgrenzung, keine neue unabhängige Rechtsträgerfreigabe. Dossier-Gotha-Anschrift bereits vorhanden, keine neuen strukturierten Kontakt-/Standort-Fills. Alte Timeline vollständig erhalten, neuer Korrekturtext ändert den hashrelevanten Notizpayload für normalen Seed. Tottlebener Dossier unangetastet, keine Preisduplikation.

## Tatsächlich ausgeführte Gates

- Gutzmann-Tests nach Fix: **4 bestanden**.
- Seed-gefilterte Tests: **13 bestanden**, Log `/root/Documents/gutzmann-owner-seed-20261010.log`.
- Workspace: **610 bestanden, 0 Fehler, 5 ignoriert**, Log `/root/Documents/gutzmann-owner-workspace-20261010.log`.
- `cargo fmt --all -- --check`, `git diff --check`: grün.
- Echter Live-Handler: HTTP **200**, 19 Preise, `published_at=None`, Standortfelder leer, Telefon/E-Mail vorhanden; Log `/root/Documents/gutzmann-owner-live-20261010.log`. Weiterhin bewusste Auslassungen S3/Batterien: keine vollständige Preisabdeckung behauptet.

## Produktion und Veröffentlichung

Read-only Prüfung zeigt weiterhin `th-gotha-sdm-gutzmann | Hauptstraße 46 | 99947 | Gotha`, Tottleben unverändert `Hauptstr. 46 | 99947 | Tottleben`. **Noch nicht repariert/deployed.** Push scheitert weiterhin an fehlender GitHub-Anmeldung. Nach Veröffentlichung koordinierter Redeploy, Seed-Adresse read-only prüfen und nächsten regulären Gotha-Scrape beobachten: Adresse muss An der Ostbahn 4 / 99867 / Gotha bleiben. Keine direkten Produktionswrites, kein erzwungener Ingest.

Stündlicher Check: Service aktiv, keine Warnungen letzte sechs Stunden; 3881 Händler, 54 Materialien, 2426 Current-Preise. Feedback weiterhin maximal ID 6080, kein neues Feedback. Keine Geocoding-Logs unter `/tmp/opencode` im vorigen Check gefunden; daraus kein Erfolg eines früheren Laufs abgeleitet.
