# Limekon-Handler — Implementierung 14.10.2026

> Historischer Implementiererstand unten: 13 Preise / 14 Skips. Der anschließende
> Owner-Review korrigiert zwei unbegründete Auslassungen auf **15 Preise / 12 Skips**.
> Maßgeblicher Integrationsnachweis: `checkin-20261014-limekon-handler-owner.md`.

## Ergebnis und Umfang

`nw-doerentrup-limekon` ist genau einmal registriert. Eigener Handler
`crates/ingestion/src/traders/handlers/limekon.rs`, zwei eigene HTML-Fixtures
unter `crates/ingestion/src/traders/handlers/fixtures/limekon-*-20261014.html`,
zwei erforderliche Registryzeilen in `handlers/mod.rs` und dieser Bericht.
Keine Dossiers, Katalogdefinitionen oder fremden Änderungen bearbeitet.
Owner-Integration und unabhängiger Materialreview bleiben gesonderte Gates.

Quelle ausschließlich https://limekon.de/pages/preisliste.php; zusätzlich
https://limekon.de/pages/impressum1.php für Kontaktanreicherung gemäß
Handlerstil. Kein Abruf/Merge der Startseite, keine zusätzliche 835-Silberquote.
Normales HTTPS ohne TLS-Ausnahme. Keine neuen Abhängigkeiten.

## Abgebildete Richtpreise

13 Zeilen: `zinn` / 99er; `zinn-geschirr` / 85–97%; `silber` / 999,
800, 925; `gold` / 999, 750, 585, 333; `zahngold` / gelb, weiß;
`kupfer-gemischt` / raff; `messing` / schwer.
Raff ist in der bestehenden Katalogdefinition von `kupfer-gemischt`
ausdrücklich enthalten; kein neues `kupfer-raff` und kein Millberry erfunden.
Händlergrade bleiben Varianten, Originalzeile samt ursprünglicher Preiseinheit
bleibt im Label. Gold 333 unverändert 34,65 EUR/g, keine Formelkorrektur.

Alle Preise `price_kind=approx`, numerische `confidence=0.5` gemäß bestehendem
Richtpreis-Präzedenzfall; keine erfundenen Preisgrenzen/Steuerannahmen.
Die Fachspec-Bezeichnung `source_type=published` ist im vorhandenen Handler-Typ
nicht als Feld verfügbar: der gemeinsame Recorder setzt `published=true` und
`source_type=haendler_angabe` (öffentliche Händlerangabe). Keine Änderung am
Recorder/Katalog und keine DB zum Prüfen oder Aufzeichnen geöffnet. Silber fein
1.000,00 EUR/kg wird durch Faktor 0,001 zu
1,00 EUR/g, alle anderen Einheiten entsprechen dem vorhandenen Katalog.

`published_at=2026-10-14T00:00:00+00:00` über vorhandenen `parse_de_date`:
Kalendertagskodierung des expliziten „Preisliste ab 14.10.2026“, kein Beleg für
eine Mitternachtsveröffentlichung. Recorder-Basis für explizite Daten ist
`page_stated`. Keine Uhrzeit 08:00 übernommen, kein `valid_to` erfunden.

## 14 sichtbare Skips / offene fachliche Abhängigkeiten

| Zeile(n) | Grund |
| --- | --- |
| Zinnlegierungen ca. 50% | Konservativ ausgelassen: kein Lötzweck/Materialform belegt. Der vorhandene generische Katalog `zinn-legierungen` wurde nicht neu erfunden, ist aber nicht in dieser engen Allowlist; eine spätere explizite Freigabe kann das erweitern. Nicht `loetzinn` oder Reinzinn. |
| Besteck versilbert ≥90 / <90, Messer ≥90 / <90, Korpus versilbert (5 Zeilen) | Versilberte Objektklassen/Auflagengrade, keine massiven Silberlegierungen. |
| 5 DEM Silber | EUR/Stück, keine Gewichtsquote. |
| andere Legierungen | Silberanteil × fein ist eine Formel, keine beobachtete numerische Quote. |
| Münzen / Barren (auf fein) | 107 EUR/g **Feingewicht**. `ScrapedPrice` trägt keine explizite Gewichtsbasis; eine nur textuelle Variante schützt nachgelagerte Gewichtsmultiplikation nicht verlässlich. Deshalb bewusst kein Bruttogewichtspreis, Originalwert/Basis bleiben im Skip. |
| Double | Vergoldete Ware; kein vorhandener passender enger Grade für diese Zuordnung. |
| Eisen | Keine belegte konkrete Katalogsorte; weder Mischschrott noch Gussbruch/Scherenklasse aus bloßem „Eisen“ ableiten. |
| Modeschmuck, Weltmünzen (2 Zeilen) | Unspezifizierte Objekt-/Mischfraktionen, keine Gold-/Silberquote. |
| Blei/Zink | Kombinierter unscharfer Eintrag, keine Aufspaltung. |

Unbekannte Labels/Grade, Einheiten, negative/Null-/Anfrage-/Range-Werte und
abweichende Tabellenstruktur werden sichtbar geskippt. Zahlen werden vor dem
permissiven gemeinsamen Zahlenparser als vollständige Zelle mit exakt erwarteter
Einheit und deutschem Dezimalformat validiert. Fehlender Preisbereich, Datum,
Richtpreise-Marker, mehrere Tabellen, doppelte Materialvariante oder ausschließlich
Skips führen zum Parsefehler, nicht zu leerem Erfolg.

## Tests und reale Quellenbelege

Reale HTML-Abrufe liegen vollständig außerhalb des Repos:
`/tmp/opencode/limekon-preisliste-20261014.html` und
`/tmp/opencode/limekon-impressum-20261014.html`. Eigene Repo-Fixtures halten die
realen Inhalts-/Kontaktblöcke fest; Navigation/Scripts/Footer entfallen,
Tabellenzeilen sind lediglich whitespace-kompakt dargestellt, Labels/Werte/
Entities/Elemente bleiben erhalten.

Angeforderte Seams: Parser und Handlerregistrierung/HTTP-Scrape. Vorhandene
Handler besitzen lokale TCP-Tests einzelner Transporthelfer, aber kein gefundenes
wiederverwendbares Muster für einen **registrierten HTTPS-Scrape-Mock**. Daher
kein TLS-Mock/URL-Testsonderpfad eingeführt: Registry separat auf genau einen
Handler mit exakter URL getestet, dessen echte Scrape-Funktion anschließend über
das vorhandene DB-freie `live_handlers` ausgeführt.

- Red: `/root/.cargo/bin/cargo test -p schrott-mcp-ingestion limekon`:
  erstes Fixture-Verhalten scheitert am unimplementierten Parser
  (`/tmp/opencode/limekon-red-1.log`); zweiter Kontakt-Test-Zyklus zunächst mit
  fehlendem `extract_info` (`limekon-red-2.log`).
- Green: **4 Limekon-Tests bestanden** (`limekon-green-2.log`): alle 13
  beobachteten Quotes/Varianten/Einheiten/Approx-/Datumsfelder, alle 14 Skips,
  adversariale Labels/Einheiten/Zahlen, Scope/Anker/Datum/Duplikate, Registry und
  echtes Impressum-Kontaktfixture.
- `/root/.cargo/bin/rustfmt --edition 2021 crates/ingestion/src/traders/handlers/limekon.rs`
  und `/root/.cargo/bin/cargo fmt --all -- --check`: erfolgreich
  (`/tmp/opencode/limekon-rustfmt-check.log`).
- `/root/.cargo/bin/cargo test --workspace`: **719 bestanden, 0 fehlgeschlagen,
  7 ignoriert** (Ingestion 649 bestanden / 7 ignoriert), einschließlich
  Seed-/Registryprüfungen; `/tmp/opencode/limekon-workspace-tests.log`.
- `/root/.cargo/bin/cargo run -p schrott-mcp-ingestion --example live_handlers -- nw-doerentrup-limekon`:
  **HTTP 200, 13.680 Bytes, 13 Preise, 14 Skips**, explizites Datum, vollständiger
  Kontakt aus Impressum. `/tmp/opencode/limekon-live-scrape.log`.

## Sicherheit / Übergabe

Alle **155** `protected_hashes` aus
`/tmp/opencode/wave-20261014-selection.json` gegen SHA-256 geprüft:
**0 Abweichungen**, `/tmp/opencode/limekon-protected-hashes.log`.
Die Produktions-DB wurde gar nicht geöffnet. Nur Workspace-Tests verwenden ihre
üblichen lokalen Testdatenbanken; Livehandler und neue Tests sind DB-frei.
Keine GEO-Anreicherung, Kontaktaufnahme, TLS-Ausnahme, Commit/Push/Deploy oder
Produktionswrites. Bestehende fremde Änderungen nicht zurückgesetzt.
Owner prüft den Integrationsdiff und den separat erstellten Materialreview.
