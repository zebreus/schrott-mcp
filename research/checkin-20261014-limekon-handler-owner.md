# Owner-Check-in: Limekon-Handler, 14.10.2026

## Ergebnisprüfung und Korrektur

Implementierungsdiff, beide HTML-Fixtures und unabhängigen Materialreview vollständig
gelesen. Freigabe nach Korrektur: **15 Richtpreise + 12 sichtbare Skips = 27
nichtleere Quellzeilen**. Die drei leeren Tabellenzeilen sind Layout.

Der ursprüngliche Implementiererstand ließ zwei bestehende Katalogmaterialien
unbegründet aus. Owner ergänzt `zinn-legierungen` / `ca. 50%` mit **17,50 EUR/kg**
und `mischschrott` / `Eisen` mit **100 EUR/t** aus **0,10 EUR/kg**. Keine Lötzinn-,
Guss- oder Scherenklasse erfunden. `pipeline.rs` belegt beide Katalogslugs;
`normalize_unit` in `traders/mod.rs` belässt bereits passende EUR/t unverändert,
also keine doppelte Skalierung. Silber fein wird lokal von 1000 EUR/kg auf
1 EUR/g skaliert. Beide Originalwerte bleiben im Preislabel.

## Reviewer-Gates

- Genau eine Registryaufnahme `nw-doerentrup-limekon`, 6h-Schedule; ausschließlich
  [dedizierte Preisliste](https://limekon.de/pages/preisliste.php) als Preisquelle,
  [Impressum](https://limekon.de/pages/impressum1.php) für tatsächlich extrahierte Kontakte.
- Alle 15 Material-/Varianten-/Wert-/Einheitenkombinationen einzeln im Test geprüft;
  Richtpreise immer `approx`, Konfidenz 0,5, EUR, ohne erfundene Preisgrenzen.
- Alle zwölf fachlichen Skips sichtbar: fünf versilberte Objektklassen, Stückmünze,
  Formel, Feinbasis-Münzen/Barren, Double, Modeschmuck, Weltmünzen, Blei/Zink.
  Kein Homepagefallback, kein 835er Silber, kein Blei/Zink-Fan-out.
- Vollständige Einheit statt Teilstring; Mutation EUR/g fein auf regulärem Gold
  wird sichtbar abgelehnt. Unbekannte Grade/Units, Null/negative/Range-/Anfragewerte
  geskippt; fehlende Anker/Datum/Richtpreise, ungültige Kalenderdaten, mehrere
  scoped Tabellen und doppelte gültige Varianten schlagen fehl.
- Datum aus Q1 als Kalendertagscontainer, keine behauptete Mitternachtsveröffentlichung.
  Gemeinsamer Recorder unverändert (`haendler_angabe`, published, page_stated);
  keine neue Recording-End-to-End-Behauptung aus einem Parser-Test.
- Kein Dossier geändert: Overwrite-/Enum-/Deep-Link-/Emdash-Dossiergate daher
  ohne neue Dossieränderungen. Seed-/Registrytests im Workspace erfolgreich.
  Beide neue Reports erhalten die ursprüngliche Recherche/Implementierungshistorie.
- Keine neuen Abhängigkeiten, kein Rust-GEO, kein TLS-Bypass, kein Produktionsingest.

## Verifizierte Tests

- Aktualisierte Solltests zunächst **rot**: drei scheiterten konkret an 13 statt
  15 Preisen (`/tmp/opencode/limekon-owner-red.log`). Danach vier Limekon-Tests
  erfolgreich (`limekon-owner-green.log`).
- `cargo fmt --all -- --check`: erfolgreich nach rustfmt des eigenen Handlers.
- `cargo test --workspace`: **719 bestanden, 0 fehlgeschlagen, 7 ignoriert**,
  einschließlich Seedtests (`limekon-owner-workspace.log`).
- DB-freier `live_handlers nw-doerentrup-limekon`: **HTTP 200, 13680 Bytes,
  15 Preise, 12 Skips**, explizites Datum und vollständige Impressumkontakte
  (`limekon-owner-live.log`).
- SHA-256 aller **155** geschützten fremden Dateien aus
  `/tmp/opencode/wave-20261014-selection.json`: **0 Abweichungen**.

## Check-in und Releasegrenze

Feedback-IDs 7381–7390 unverändert, bereits in den vorhandenen Triageberichten
bearbeitet. Service aktiv; keine systemd-Warnungen in sechs Stunden. Separater
JSON-WARN-Check: Münster verdächtige Preisleiter korrekt zurückgewiesen, AMR
HTTP 502, Huth alte Produktionsversion ohne Preistabelle. Produktionszählung
read-only: **3881 Händler / 54 Materialien / 2426 aktuelle Preise**.
Keine Geo-Logs gefunden. Keine Produktionsdatenbank geändert.

Lokal für main freigegeben. **Push und Redeploy bleiben blockiert**:
`gh auth status` bestätigt fehlende GitHub-Anmeldung. Keine Zugangsdaten gesucht,
keine TLS-/SSH-Sicherheitsumgehung und kein vorgezogener Produktionsdeploy.
Die Service-/DB-Prüfung belegt Gesundheit des bestehenden Releases, ausdrücklich
nicht die Veröffentlichung dieses neuen Handlers.
