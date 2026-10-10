# Unabhängiger Review: Owned-Variant-Fundament, 10.10.2026

## Urteil

**Freigabe des eng begrenzten Cow-Migrationsfundaments gegen Worktree-HEAD; kein blockierender Codefehler gefunden.** Keine Freigabe vollständiger MSG-Mengenstaffeln, historischer Current-Reconciliation oder eines Produktionsrollouts. Zwei nicht blockierende Dokumentations-/Nachweisgrenzen stehen unten.

Der Review erfolgte durch eine unabhängige Review-Session, ohne weitere Delegation und ohne Codeänderungen. Standards und fachlicher Vertrag wurden getrennt betrachtet. Einziger geschriebener Repository-Pfad ist dieser Bericht.

## Fixierter Prüfgegenstand

- Worktree: `/tmp/opencode/schrott-mcp-owned-variant-20261010`.
- Verifiziertes HEAD: `e5e32fbdedfa04ca53081c6b52123c9654e757e6`.
- Vergleich: uncommittete Änderungen gegen **dieses HEAD**, nicht gegen das inzwischen weiterentwickelte main; 91 Rust-Dateien, 321 eingefügte / 136 entfernte Zeilen.
- Vollständiges Patch-Artefakt gelesen: `/tmp/opencode/owned-variant-foundation-20261010.patch`. Eigener byteweiser Vergleich mit `git diff HEAD -- '*.rs'`: identisch. SHA-256: `8632aa9296b728ecc1c411d387affed8e066a7a468939f53d1355a2e40b4c80b`.
- Fachlicher Scope aus Review-Auftrag und `research/checkin-20261010-owned-variant-foundation.md`; ergänzend README-Preisvertrag und `research/checkin-20261010-msg-owner-fallback.md`. Dessen größere MSG-Folgeplanung ist **nicht** der Implementierungsumfang dieser Migration.

## Standards

Keine dokumentierte Standardverletzung gefunden. README verlangt getrennte Händlerparser, statischen Materialkatalog und bestehende Preis-/Datumsverträge; diese Grenzen bleiben erhalten. Keine AGENTS.md gefunden. Die breite Konstruktoranpassung ist notwendige Folge des öffentlichen Feldtypwechsels, kein unnötiger fachlicher Umbau. Kein relevanter neuer Architektur-Smell aus dem Review-Baseline-Katalog gefunden.

## Spec / Invarianten

### Legacy-Identität, Current, History, Canary, Datumsinferenz

- `crates/ingestion/src/traders/mod.rs:108`: `Cow<'static, str>` erlaubt statische borrowed und dynamische owned Identitäten. Literal-/Mapping-Konstruktoren bekommen lediglich `.into()`; keine Variante wird umbenannt, normalisiert oder neu kanonisiert.
- `record()` (`mod.rs:308–416`): Snapshot-Schlüssel bleiben `(i64, String)`. Entscheidend ist `p.variant.as_ref().to_owned()`, nicht `Cow::to_owned()`. Snapshot-Lookup, Datumsinferenz und Jump-Canary verwenden weiterhin denselben Textschlüssel.
- `NewPrice.variant` und `current_price_for` erhalten über `as_ref()` unveränderte `&str`. Der Store bleibt unverändert: Current-/Dedup-Schlüssel ist Händler/Material/Variantentext; History-Abfrage filtert denselben Text. Borrowed/Owned ist kein persistiertes Identitätsmerkmal.
- Datum-/Canary-Bedingungen, Mapping-Version, Normalisierung, Zählung und Fehlerschreibpfade sind nicht fachlich verändert. Die bestehenden Canary- und Datumsregressionen bleiben im Diff erhalten. Owned-spezifische Jump-Canary-Auslösung wird allerdings nicht neu getestet.
- Kein Store-/SQL-Schema-/MAP_VERSION-Wechsel; keine neue MSG-Reconciliation. Aus dieser Migration folgt keine Bereinigung existierender Current-Altlasten.

### Drei Leak-Entfernungen

- `handlers/fairkat.rs:38–66`: dieselbe geparste `katnr`, zuvor `Box::leak(katnr.into_boxed_str())`, jetzt `katnr.into()`. Keine Änderung an leerer Kat-Nr, DPF-Ausfilterung, Material, Preis oder Einheit.
- `handlers/katalysator_hai.rs:41–55,85–88`: exakt dasselbe `format!("{marke} {merkmal}")`, nun als `String` statt geleaktem `&'static str`. Aufrufer wandelt in owned Cow; keine neue Whitespace-/Labelbehandlung.
- `handlers/westarp.rs:48–67`: leeres Grade behält den statischen `tier` borrowed; sonst exakt dasselbe `format!("{grade} / {tier}")` owned. Kein Mengenparser-/Mappingwechsel.
- Eigene vollständige Suche unter `crates/**/*.rs`: keine `Box::leak`-Treffer mehr.

### Konstruktoren, weitere Aufrufer, Beispiele

Vollständiger Diff und repositoryweite `ScrapedPrice`-Suche geprüft (98 `ScrapedPrice {`-Vorkommen inklusive Struct-Deklaration). Sämtliche geänderten bestehenden Konstruktoren sind mechanische Typanpassungen; Westarp besitzt bereits einen passend typisierten Cow-Wert. Testprojektionen borgen `as_ref()` statt das nun nicht mehr Copy-fähige Feld aus einer Referenz zu bewegen. Store-Tests in Kernschrott/reGOLD sind passend angepasst. Unveränderte Vergleiche gegen Stringliterale vergleichen Text, nicht Cow-Repräsentation.

Außerhalb `crates/ingestion/src/` wurden keine Rust-`ScrapedPrice`-Referenzen gefunden. `crates/ingestion/examples/live_handlers.rs` separat gelesen: iteriert borrowed über Outcomes, konstruiert keine Preise und benötigt keine Variantentyp-Anpassung. Keine übersehene notwendige Example-Änderung gefunden. Öffentliche externe Rust-Aufrufer außerhalb dieses Repositorys sind nicht geprüft; der API-Typwechsel bleibt bewusst source-breaking.

### Reale TempDB-Abdeckung

`owned_and_borrowed_variants_keep_current_identity` (`mod.rs:774–940`) öffnet echte `PublicDb`/`InternalDb` in `TempDbDir`, schreibt durch `record()` und liest echte Current-/History-Daten. Es ist kein gemockter Store-Test.

Belegt werden vier getrennte Current-Keys bei identischen Beträgen (leer, statisch, owned 137/250 kg), gleiche Textidentität trotz Owned→Borrowed-Wechsel, Dedup ohne zusätzlichen History-Eintrag, isolierter Preiswechsel mit deutscher Tagesinferenz und `observed_price_change`, A→B→A mit neuer History sowie Lesbarkeit nach Drop des Outcomes. Die abschließende Current-Anzahl ist vier.

## Nicht blockierende Hinweise / Grenzen

1. **TempDB-Test ist dynamische Konstruktion, kein Parser→Store-Integrationstest.** `mod.rs:842` erzeugt die owned Variante mit `format!` aus Testzahlen. Es wird keine Händler-Fixture geparst und kein vollständiger Scrape-Outcome der drei Leak-Handler in diesen TempDB-Test eingespeist. Die vorhandenen Handler-Tests prüfen Parser/Mapping separat. Das genügt als Nachweis des Cow-/Store-Fundaments, darf aber nicht als realer MSG-Schwellenparser- oder vollständiger Handler→DB-Nachweis beschrieben werden. Auch ein vorab mit Legacy-`NewPrice` befüllter DB-Key wird nicht eigens getestet; Textgleichheit ist hier durch Diff-/Store-Audit und Owned/Borrowed-Dedup belegt.
2. **Scope-Formulierung präzisieren:** `research/checkin-20261010-owned-variant-foundation.md:8` sagt „MSG-Sichtbarkeitsfix ist enthalten“. Gegen das geprüfte HEAD enthält `handlers/msg_metallrecycling_gotha.rs` ausschließlich `variant: variant.into()`; der Sichtbarkeitsfix ist bereits Baseline, keine Leistung dieses Diffs. Zeile 44 grenzt das korrekt ein. Empfohlene Formulierung: „Der bereits in HEAD enthaltene MSG-Sichtbarkeitsfix bleibt unverändert.“ Keine Staffelaktivierung oder Reconciliation ist im Rust-Diff enthalten; beides bleibt offen.

## Evidenz: eigene Prüfungen vs. vorhandene Tests

**Eigene Prüfungen:** vollständiger Diff-/Quelltext-Audit, HEAD/Status/Stat, byteweiser Patchvergleich, repositoryweite Konstruktor-/Caller-/Leak-Suche, eigener `git diff --check` ohne Fehler. Kein eigener Cargo-Build, Testlauf oder Live-HTTP-Aufruf; damit keine neuen Build-/DB-Artefakte und kein konkurrierender Targetdir.

**Vorhandene Implementierungsagent-Evidenz, hier gelesen, nicht selbst ausgeführt (keine Owner-Testläufe):**

- `/tmp/opencode/owned-variant-workspace.log`: 612 bestanden, 0 Fehler, 5 ignoriert (4 + 3 + 555 + 4 + 16 + 30); der neue Cow-TempDB-Test ist explizit `ok`.
- `/tmp/opencode/owned-variant-targeted.log`: 15 beim Filter `traders::tests` bestanden; dieser Filter trifft auch `seed_traders::tests`, also nicht 15 ausschließlich Trader-Modultests. Zusätzlich MSG 6, Fairkat 3, Westarp 5, Katalysator-Hai 4 bestanden. Canary und Datumsinferenz explizit `ok`.
- Logs nennen den Agent-Targetdir `/tmp/opencode/target-owned-variant-20261010`. Logsichtung ist keine unabhängige Wiederholung und kein kryptografisch gebundener Nachweis des Build-Inputs. Die Patchgleichheit bezieht sich auf den aktuellen Review-Diff.
- Formatierung ist laut Implementierungsagent grün; nicht unabhängig erneut ausgeführt.

## Scopegrenzen / Integration

Keine Commits, Änderungen an main, Produktions-/Secrets-/Netzzugriffe oder bestehende DB-Dateien. Keine komplette MSG-Staffelumsetzung, Duplicate-Konfliktbehandlung, neue Preiseinheitenlogik, Nullpreisregel oder Current-Retirement abgenommen. Keine Live-Seitenvollständigkeit behauptet. Änderungen auf main nach `e5e32fb` sind nicht Gegenstand dieser Freigabe; nach Integration sind Konflikte und Tests erneut zu prüfen.

**Zusammenfassung:** Standards: 0 Findings. Spec/Codekorrektheit des Cow-Fundaments: 0 blockierende Findings; 2 nicht blockierende Nachweis-/Dokumentationshinweise. Freigabe ausschließlich für den fixierten Migrationsdiff.
