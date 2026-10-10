# Unabhängiger Review: `clear_fields`

## Ergänzende dynamische Owner-Abnahme

Nach dem unabhängigen statischen Review hat der Owner den vollständigen Drei-Dateien-Diff gelesen und die Tests auf Main ausgeführt: **13 seed-gefilterte Tests**, **9 Buildscript-Tests**, anschließend **610 Workspace-Tests bestanden, 0 fehlgeschlagen, 5 ignoriert**. Workspace-Log: `/root/Documents/seed-clear-main-workspace-20261010.log`; Seed-Log: `/root/Documents/seed-clear-main-20261010.log`. `cargo fmt --all -- --check` und `git diff --check` bestanden. Damit sind auch die vier neuen realen TempDB-Clear-Regressionstests ausgeführt; keine Produktionsdatenbank beschrieben. Die folgenden Aussagen „nicht ausgeführt“ beziehen sich weiterhin ausschließlich auf den unabhängigen Reviewer.

Lokale Integration freigegeben. Veröffentlichung/Redeploy und tatsächliche Produktions-Clears bleiben wegen fehlender GitHub-Authentifizierung offen; kein Erfolgsnachweis für Produktionsbereinigung aus lokalen Tests abgeleitet.

Datum: 2026-10-10. Basis: `HEAD` = `2dbf69f48dcba2257791a037d1a0b12938d79d5a`.
Vergleich: `git diff HEAD -- crates/ingestion/build.rs crates/ingestion/src/seed_traders.rs docs/trader-seed.md`.
Prüfumfang ausschließlich diese drei Dateien; Referenz ist der im Auftrag formulierte Vertrag.

## Ergebnis

**Keine konkreten funktionalen Findings im geprüften Diff.** Die opt-in-Löschsemantik entspricht nach statischer Prüfung dem Vertrag. Dies ist keine Bestätigung erfolgreicher Builds oder Tests: Auftragsgemäß wurden keine Builds, Tests oder DB-Zugriffe ausgeführt. Store-Implementierung und `TempDbDir`-Implementierung liegen außerhalb des Prüfumfangs.

## Spec / Löschvertrag

- **Build und Runtime sind fail-closed:** `build.rs:235–266` und `seed_traders.rs:88–126` erlauben genau die neun vorgesehenen Enrichmentfelder. Trimmen, Duplikaterkennung, unbekannte Namen und leere Tokens innerhalb nichtleerer Listen sind abgedeckt. Konflikte prüfen `is_empty()`, nicht `trim().is_empty()`: Auch ein gefülltes Whitespace-Feld sowie explizite JSON-Defaults widersprechen dem Clear. Eine insgesamt leere/Whitespace-Directive ist absichtlich ein No-op.
- **Validierung vor Writes:** `seed_traders.rs:338–341` validiert den gesamten Batch vor Eintritt in die Verarbeitungsschleife und vor `existing_seed_state`/`upsert_trader`. Deserialisierungsfehler beim Laden werden ebenfalls vor Anwendung des Corpus zurückgegeben. Keine neue Forderung nach transaktionaler Atomizität bei späteren Store-Fehlern wird daraus abgeleitet.
- **Normales Upsert statt Lösch-SQL:** `seed_traders.rs:352–383` bildet Clear-Felder auf `''`, `unbekannt`, `[]` beziehungsweise `{}` ab und übergibt sie an den vorhandenen `NewTrader`-Upsert (`413–442`). Neues produktives SQL ist nicht enthalten. `certifications` wird bei einem Clear zusätzlich in die Hash-Eigentümerschaft aufgenommen.
- **Drift trotz identischem Hash:** `clear_drift` vergleicht jeden markierten gespeicherten Wert mit seinem Zieldefault (`355–357`) und verhindert den Hash-Skip (`400–405`). Dadurch bleibt die Directive auch bei späterem Enrichment mit erhaltenem `seed_hash` autoritativ. Bei bereits passenden Defaults greift der normale Skip.
- **Unmarkiertes Enrichment und GEO:** Für die übrigen neun Enrichmentfelder wird unverändert `keep` verwendet. `website_checked_at` wird weiterhin aus dem gespeicherten Zustand übernommen (`430`). Beide GEO-Felder sind verboten; der bisherige Koordinatenpfad ist unverändert (`384–387`, `424–426`). `first_seen_at` erhält keine neue Zuweisung im Anwendungspfad; seine konkrete Erhaltung durch den Store wird in den neuen DB-Tests ausdrücklich behauptet.
- **Dokumentation:** `docs/trader-seed.md:7–54` beschreibt die tatsächliche Semantik einschließlich unabhängiger Website-/Status-Clears, fortdauernder Autorität, Directive-Entfernung und nicht-negativer fachlicher Interpretation. „YAML null“ bezieht sich auf den Dossierparser: Direktes JSON-null für das Runtime-Stringfeld ist nicht als akzeptierte Eingabe dokumentiert.

## Legacy-Codegen / Hash / Keep

- Ohne nichtleere Directive wird kein neues JSON-Paar emittiert (`build.rs:327–330`); Reihenfolge und Emissionslogik der alten Paare bleiben im Diff unangetastet. Daher bleibt die generierte Ausgabe für unveränderte alte Eingaben statisch nachvollziehbar bytegleich.
- Bei leerer Directive ruft jeder `effective`-Zweig den alten `keep` mit denselben Parametern auf. `keep` selbst wurde nur kommentiert, nicht funktional geändert.
- Alte Hash-Eingabereihenfolge, Zertifikatsbedingung und GEO-Hash bleiben erhalten. Die zusätzliche Hash-Domain wird nur für nichtleere Clears eingespeist (`301–305`); die neue Zertifikatsbedingung ist ohne Directive identisch mit HEAD. Damit bleibt die Legacy-Hashberechnung auch bei vorhandenen Koordinaten/Zertifikaten statisch unverändert.

## Testprüfung: reale TempDB-Semantik und Grenzen

- `seed_clear_each_field_restores_defaults_and_preserves_other_enrichment` (`504–585`) nutzt `PublicDb::open(dir.path())`, den realen `upsert_trader` und SELECT-Abfragen, keine Store-Mocks. Für jedes Feld werden Default, alle anderen Enrichmentwerte, GEO, `website_checked_at`, `first_seen_at` und `updated_at` geprüft. Ein weiterer realer Enrichment-Upsert behält den zuvor gelesenen Hash bei; anschließende Wiederherstellung und No-op werden geprüft. Das ist ein aussagekräftiger Regressionstest für die gefährliche Same-Hash-Drift.
- `seed_clear_validation_rejects_whole_batch_before_any_write` (`588–645`) stellt einen gültigen Seed vor den ungültigen Seed und prüft den weiterhin leeren Traderbestand. Erfasst sind verbotene/unbekannte Felder, Duplikate, Leertokens, sämtliche gefüllten Konflikte und falsche JSON-Directive-Typen.
- `seed_clear_all_fields_on_insert_is_idempotent` (`703–728`) prüft alle Defaults auf Insert, Wiederholung sowie Zeitstempel.
- Der Hash-Test (`648–700`) enthält einen festen Legacy-Hash, prüft fehlende/leere/Whitespace-Directive, kanonische Reihenfolge sowie Änderung und Entfernung. Der Zahlenwert `870c82e3dd902952` wurde in diesem Read-only-Review nicht unabhängig berechnet oder ausgeführt; die unveränderte Legacy-Berechnung wurde gegen HEAD gelesen.
- Die Build-Tests (`433–505`) prüfen canonical/opt-in-Ausgabe und ungültige Directives auf Parser-Ebene. Sie vergleichen geordnete Rows, nicht komplette emittierte JSON-Dateien mit einem HEAD-Golden. Bytekompatibilität folgt hier aus dem unveränderten Emissionspfad, nicht aus einem ausgeführten Golden-Test. Ebenso fehlt ein expliziter Legacy-Golden mit Koordinaten und Zertifikaten; kein nachgewiesener Defekt, lediglich begrenzte zusätzliche Testabsicherung.
- Ob `TempDbDir` tatsächlich eindeutige isolierte Pfade erzeugt und wie der Store intern upsertet, wurde wegen des Drei-Dateien-Scope nicht separat untersucht. Die sichtbaren Tests sind als echte TempDB-Integrationstests angelegt, ihr erfolgreicher Lauf bleibt unbestätigt.

## Standards

Keine belegte Standardverletzung innerhalb des eingeschränkten Prüfumfangs. Die doppelte Clear-Validierung in dependency-freiem Buildscript und Runtime ist ein möglicher Wartungsrisiko-Heuristikpunkt, kein aktueller Vertragsfehler: Die neun Felder und Regeln stimmen überein und werden auf beiden Seiten getestet. Keine zusätzliche Abstraktion oder Scope-Erweiterung erforderlich.

Zusammenfassung: Standards **0 konkrete Findings**, Spec **0 konkrete Findings**; statisch konsistent, dynamische Verifikation ausdrücklich nicht durchgeführt. Einzige Schreiboperation dieses Reviews: dieses Dokument.
