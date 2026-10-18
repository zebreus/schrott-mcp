# Goldtrans: isolierter Startup-Regressionsvertrag, noch kein Reparaturfix

## Stand und Grenze

Nur Testsource in `crates/store/src/public/prices.rs` ergänzt. **Kein Migrationfix, kein Testlauf, kein Build, kein Commit und kein Deployment ausgeführt. Keine Produktionsdatenbank geöffnet; keine manuelle Prod-DML.** Der echte Rotlauf ist ausdrücklich Aufgabe des Owners vor einer weiteren Implementierung. Es wird weder PASS noch ein tatsächlich beobachtetes FAIL behauptet.

Exakter Testname:

```text
public::prices::tests::startup_migration_retires_only_14_goldtrans_zero_current_keys_preserving_history
```

Nur als späterer Owner-Aufruf, hier **nicht ausgeführt** (Cargo kann dabei bauen):

```sh
TMPDIR=/tmp/opencode cargo test -p schrott-mcp-store public::prices::tests::startup_migration_retires_only_14_goldtrans_zero_current_keys_preserving_history -- --exact --nocapture
```

## Vereinbarter Seam und Fixture

Seam: normale `PublicDb::open`-Startupmigration, beobachtet über `current_price_for`, `price_history`, die öffentliche Current-View und vollständige Raw-SQL-Zeilenvergleiche. Keine neue Quarantäne-API und kein nachgebauter Repair-SQL im Test. `TempDbDir` isoliert jede Fallgruppe; bei obigem Owner-Aufruf liegt sie unter `/tmp/opencode`. Raw-SQL-Inserts erzeugen ausschließlich dort den bereits persistierten Legacyzustand, unabhängig von künftigen Schreibguards.

Identität aus `research/checkin-20261018-1900-goldtrans-diagnosis.md`:

- Händler: `hh-wandsbek-goldtrans-edelmetallhandel`.
- Quelle: `https://www.goldtrans.de/goldankauf-preise-aktueller-goldpreis-ankauf-in-hamburg.html`.
- Material `gold`, Varianten `999`, `986`, `980`, `965`, `916`, `900`, `875`, `833`, `750`, `585`, `416`, `375`, `333`; Material `zahngold`, Variante leer.
- Betrag `0`, Währung `EUR`, Einheit `EUR/g`.
- Keine produktiven DB-IDs im Test: Händler/Materialien werden erzeugt, Preis-IDs nur aus den Fixture-Inserts übernommen.

Der Hauptfall enthält genau diese 14 Null-Current sowie für jeden Schlüssel einen positiven historischen Vorgänger. Zusätzlich existiert ein älterer, nicht aktueller Gold/916-Nullbeleg. Beide Arten History müssen unverändert bleiben. Erwartung: **0 Current für die 14 betroffenen Schlüssel**, ohne positive Vorgänger als neue aktuelle Angebote zu aktivieren.

Scope-Negativkontrollen:

- Andere Händler, anderes Material, Gold mit leerer oder unbekannter Variante und Zahngold mit nichtleerer Variante behalten ihre Null-Current samt Pointer/Zeitstempel.
- Separate TempDB-Fälle verwenden dieselben 14 bekannten Schlüssel mit positivem Betrag, anderer Source-URL, anderer Währung oder anderer Einheit: jeweils alle 14 Current unverändert.
- Alle gespeicherten Spalten sämtlicher `prices`, `traders`, `materials` und tatsächlich belegter `trader_materials` werden vor/nach Startup verglichen; History wird zusätzlich über die öffentliche API abgefragt.
- Zweites normales Öffnen prüft Idempotenz und verhindert ein späteres Wiederbeleben historischer Vorgänger.

Der erste Nullfall steht absichtlich zuerst. Falls er rot ist, führt dieser einzelne Test seine späteren Guard-Fallgruppen noch nicht aus; deren tatsächliche Ausführung ist erst beim vollständigen Folgelauf belegt.

## Bestehendes Migrationspattern und späterer Scope

`PublicDb::open` in `crates/store/src/public/mod.rs` führt Schemaanlage, `traders::migrate`, **`prices::migrate`**, danach Viewanlage aus. `prices::migrate` verwendet `PRAGMA user_version`; bestehende gezielte Datenreparaturen sind WKG (3), A-Z (4), Schiefer (5), Mappingfehler #4782 (6). Der Test simuliert einen bestehenden Stand mit `user_version=6` und öffnet dann ausschließlich seine TempDB normal erneut.

A-Z/Schiefer selektieren enge Provenienzmerkmale, annotieren History und können ältere Pointer wiederherstellen. **Diese beiden Verhaltensweisen passen nicht zum Goldtrans-Auftrag.** Migration 6 zeigt transaktionale Retirement ohne Ersatzpointer, annotiert aber ebenfalls History. Eine spätere Goldtransmigration darf nur die belegten Null-Current im genannten Scope entfernen, History/Material/Händler/Acceptance nicht verändern und keine historischen Ersatzpreise installieren. Versionierung/Idempotenz sollen über den bestehenden Startupweg erfolgen, nicht über manuelle Prod-DML. Der aktuelle Test fügt keinerlei Migrationscode hinzu und reserviert keine neue Versionsnummer.

Nach Owner-Rotlauf und gesonderter Fixfreigabe erforderlicher Prüfbereich:

1. Den oben genannten exakten Test tatsächlich rot beobachten; danach gegen den gesondert reviewten Fix vollständig laufen lassen, einschließlich aller fünf Fallgruppen und beider Startups.
2. `public::prices::tests` insgesamt: insbesondere bestehende WKG-, A-Z-, Schiefer- und #4782-Migrationstests, History-/Current-/Snapshot- und Ownershiptests.
3. `schrott-mcp-store` insgesamt: Startup, Trader-Migration, Hofmann-Guards und sonstige Storetests dürfen nicht regressieren.
4. Die bisherigen Tests `migration_retires_a_z_ladder_from_current_prices` und `migration_retires_schiefer_sale_quotes_from_current_prices` erwarten momentan ausdrücklich Endversion **6**. Bei einer späteren neuen Migration ist deren Endversionsassertion anzupassen; **jetzt unverändert**, weil noch kein Fix existiert. Trader-Migrationstests direkt gegen `traders::migrate` sind davon zu unterscheiden.
5. Negative/nichtendliche künftige Goldtrans-Writes gehören zum separaten Handlerauftrag `handlers/goldtrans.rs`; dieser Storetest ersetzt weder dessen echten Handler-/Record-Replay noch seine Writeguards. Ein späterer Rollout braucht beide Schutzteile und unabhängige Current-/History-Nachkontrolle.

Keine Fremdänderungen oder MKM-Storelogik bearbeitet. Raw-SQL-History bleibt als unveränderter Beleg zugänglich; öffentliche Current-Unterdrückung ist hier nur als noch auszuführender Testvertrag formuliert, nicht als bereits erreichte Produktionswirkung.

## Nachtrag: Owner-Rotlauf bestätigt, Migration 7 zur Prüfung bereit

Die Abschnitte oben dokumentieren den vorherigen Test-only-Stand. Danach hat der Owner den **echten fachlichen Rotlauf** bestätigt: Log `/root/Documents/checkin-20261018-2230-goldtrans-current-red.log`, SHA256 `8f54a6555b204d44595f23f881d19d5cd5bc68cb1cee2a89279ddef0437c9103`, Exit **101**, ein ausgeführter Test mit Fehler **remaining 14 statt 0**, keine Buildprobleme. Das ist ein vom Owner gemeldeter Nachweis; hier wurde weder das Log erneut geprüft noch der Lauf wiederholt. Auf dieser Basis erfolgte die ausdrückliche Fixfreigabe.

Nun ausschließlich in `prices.rs` ergänzt:

- Migration **7**, nach bestehendem `user_version`-Muster: transaktionales `DELETE` nur aus `current_prices` mit belegtem Pointer, übereinstimmender Händler-/Material-/Variantenidentität, exaktem Goldtrans-Slug, exakter URL, `EUR`, `EUR/g`, `price=0` und den genannten 14 Schlüsseln. Versionsfortschreibung und Retirement committen zusammen.
- **Kein History-UPDATE**, keine Annotation, keine Änderung an Händler/Material/Acceptance, keine Ersatzpointer und keine Änderung am MKM-Storeguard. Die neue Migration schreibt außer Current-Retirement nur ihre Schema-Versionsmarke.
- Die zwei vorhandenen A-Z-/Schiefer-Endversionsassertions gezielt von **6 auf 7** angepasst. Der bestehende Startup-Regressionsvertrag prüft nun bei beiden Öffnungen zusätzlich Version 7; sein Kommentar beschreibt den bestätigten Rotlauf historisch.
- Neuer Test `public::prices::tests::migration_7_goldtrans_rolls_back_partial_retirement_and_allows_retry`: SQLite-`AFTER DELETE`-Trigger mit `RAISE(FAIL)` erzeugt absichtlich eine bereits teilweise ausgeführte Löschung. Erwartet werden vollständige Pointer-/History-Erhaltung, beendete Transaktion und Version 6 nach Fehler; nach Triggerentfernung erfolgreicher Retry auf Version 7 bei unveränderter History. **Testsource ergänzt, nicht ausgeführt.**

**Kein Green-Nachweis und keine Erfolgszahlen:** Auch nach Fixfreigabe hier keine Builds, Tests, ProdDB-Zugriffe, Prodwrites, Force-Ingests, Commits oder Deployments ausgeführt. Owner übernimmt die oben beschriebenen vollständigen Prüfungen inklusive neuem Rollbacktest unter Ressourcenwächter und anschließendem Snapshot. Produktionswirkung weiterhin nicht behauptet; vorgesehen ist allein die normale Appmigration beim gesondert reviewten Rollout.
