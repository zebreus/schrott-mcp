# Owned Preisvarianten — Fundament für MSG, 10.10.2026

## Fortschritt

- Ausgangspunkt: aktuelles main `e5e32fb`, sauber verifiziert; separater Worktree `/tmp/opencode/schrott-mcp-owned-variant-20261010`, Branch `foundation/owned-price-variant-20261010`.
- Auftrag: ausschließlich `ScrapedPrice.variant` auf `Cow<'static, str>` migrieren, bestehende Identitäten erhalten und dynamische owned Varianten über reale temporäre Datenbanken testen.
- README gelesen; keine AGENTS.md im Repository gefunden. Keine Produktionsdatenbank geöffnet, kein GEO, keine Pushes/Deployments.
- Der bereits in Baseline-HEAD enthaltene MSG-Sichtbarkeitsfix bleibt unverändert. Mengenstaffeln werden NICHT aktiviert; historische Current-Reconciliation bleibt Folgeauftrag.
- Migration in sämtlichen Preis-Konstruktoren erfolgt; Record-Lookupkeys bleiben `(material_id, String)` via `variant.as_ref().to_owned()`, Store-Aufrufe erhalten unveränderten `&str`.
- TempDB-Test `owned_and_borrowed_variants_keep_current_identity` bestanden: vier verschiedene Keys bei gleichen Beträgen, leere/statische Variante, owned 137/250-kg-Varianten, borrowed/owned gleicher Text dedupliziert, A→B→A-Historie und korrekte Datumsinferenz, lesbar nach Drop des Outcomes.
- Der TempDB-Test konstruiert dynamische Varianten mit `format!` aus Testzahlen und schreibt durch den echten `record()`-/Store-Pfad. Er ist kein Parser→Store-Nachweis und kein vollständiger Handler→DB-Integrationstest.
- Kompilierung sämtlicher Ingestion-Tests grün. Workspace und gezielte Tests abgeschlossen, siehe Nachweise unten. Builds erhalten eigenen `CARGO_TARGET_DIR=/tmp/opencode/target-owned-variant-20261010`.

## Bestehende Leaks (vor Änderung)

`handlers/fairkat.rs`, `handlers/westarp.rs` und `handlers/katalysator_hai.rs` verwendeten bereits `Box::leak` für dynamische Varianten. Jetzt ohne Leak: Fair-Kat übernimmt `katnr` als owned Cow; Westarp behält `tier` borrowed bzw. `format!("{grade} / {tier}")` owned; Katalysator-Hai liefert weiterhin exakt `format!("{marke} {merkmal}")`, nun als String→Cow. Keine Material-/Mappingänderungen.

## Abgrenzung und Integration

Cow ist gegenüber String gewählt, damit statische Mappingtabellen keine neuen Allokationen brauchen. `.into()` bedeutet für `&'static str` borrowed und für `String` owned. Cow ist nicht Copy; Testprojektionen/Store-Lookups borgen ausdrücklich `as_ref()`. Kein Datenbankschema-/MAP_VERSION-Wechsel nötig, da persistierte Varianten bytegleich bleiben.

Andere Sessions entwickeln main unabhängig weiter (bei Zwischenprüfung bereits `725b126`). Dieser Worktree bleibt auf dem geprüften Ausgangspunkt; keine fremden Änderungen übernommen oder überschrieben. Gutzmann erhält hier ausschließlich die unvermeidliche Konstruktor-Typanpassung, keine Parser-/Mappingänderung. Bei späterer Integration gleichartige `.into()`-Konflikte mechanisch prüfen.

## Verifizierte Ergebnisse

Alle folgenden Befehle wurden vom Implementierungsagenten in dieser Session im eigenen Worktree mit dem oben genannten Agent-Targetdir ausgeführt; die Testlogs sind Agent-Evidenz, keine vom Owner ausgeführten Tests:

| Prüfung | Ergebnis | Artefakt |
|---|---|---|
| `cargo test -p schrott-mcp-ingestion --no-run` | grün | `/tmp/opencode/owned-variant-compile.log` |
| Neuer TempDB-Test einzeln | 1 bestanden | Toolausgabe dieser Session |
| `cargo test -p schrott-mcp-ingestion traders::tests` | 15 bestanden (inklusive `seed_traders::tests`, nicht ausschließlich Trader-Modultests) | `/tmp/opencode/owned-variant-targeted.log` |
| Handlerfilter `msg_metallrecycling_gotha`, `fairkat`, `westarp`, `katalysator_hai` | 6 / 3 / 5 / 4 bestanden | gleicher Targeted-Log |
| `cargo test --workspace` | **612 bestanden, 0 Fehler, 5 ignoriert** | `/tmp/opencode/owned-variant-workspace.log` |
| `cargo fmt --all -- --check` | grün | Toolausgabe |
| `git diff --check` | grün | Toolausgabe |
| Suche `Box::leak` unter `crates/` | keine Treffer | Toolausgabe |

### Standards-Review

Manueller Diff-Audit: keine dokumentierte Standardverletzung gefunden. Händlerparser bleiben getrennt; keine gemeinsame Mengen-/Materialheuristik eingeführt. Die breite Konstruktoranpassung (91 Rust-Dateien) ist unmittelbar durch den bewusst geänderten öffentlichen Feldtyp erforderlich, kein fachlicher Umbau. Testprojektionen erhalten dieselben Stringwerte über `as_ref()`. Keine Store-/Server-/Dossier-/Katalogdatei verändert.

### Spec-Review

Manueller Diff-Audit: keine fehlende Anforderung dieses Fundament-Auftrags gefunden. Alle bisherigen literalen/statischen Varianten bleiben borrowed, drei bislang geleakte dynamische Identitäten owned und textgleich; Lookupkeys, Store-Aufrufe und Tests korrekt angepasst. Der reale TempDB-Test prüft getrennte Current-Identitäten, unveränderte statische Keys, Dedup und Preisgeschichte. MSG-Diff enthält ausschließlich `variant: variant.into()`; `kept` und Sichtbarkeitsfix sind unverändert. Keine Reconciliation oder vollständige Mengenstaffelabdeckung behauptet.

Die zunächst versuchten Review-Subagenten wurden vom Harness wegen Subagent-Tiefenlimit 1 abgewiesen. Anschließend erfolgte ein unabhängiger Review in einer separaten Session: `research/checkin-20261010-owned-variant-independent-review.md`. Dieser gibt ausschließlich den fixierten Cow-Migrationsdiff frei; der Owner hat Rust-Diff und Review gelesen und die scoped Freigabe bestätigt. Die zwei nicht blockierenden Dokumentationshinweise sind hier präzisiert. `docs/agents/issue-tracker.md` fehlt; `/setup-matt-pocock-skills` wäre für einen späteren trackerbasierten Skill-Workflow nötig, wird hier nicht ungefragt ausgeführt.

## Restarbeit und Risiken

- Vollständige MSG-Staffeln sind **weiter offen**: strikter dynamischer Schwellenparser, belegte Preiseinheit unabhängig von Mengenmaß, Duplicate-Konfliktprüfung, Nullpreisvertrag sowie source-/slug-genaue historische Current-Reconciliation inklusive Wasseruhren-Mismapping. Dieses Fundament allein behebt keine Produktions-Current-Altlast.
- Rust-API ist absichtlich nicht mehr Copy-kompatibel; neue Aufrufer müssen `.into()` beim Bauen und `as_ref()` beim Borgen verwenden. Persistierte SQL-Identität/Schema und bestehende Preise ändern sich nicht.
- Kein echter Live-HTTP-Scrape in diesem Teilschritt: deterministische Fixtures und echter temporärer Store-Pfad sind belegt, heutige Webseitenvollständigkeit nicht erneut behauptet.
- Weitere Commits anderer Sessions auf main müssen bei Integration separat berücksichtigt und Workspace-Tests erneut ausgeführt werden.
- Owner hat den Commit ausschließlich der 91 geprüften Rust-Dateien und dieser beiden Foundation-/Reviewberichte im eigenen Branch/Worktree beauftragt. Kein Push, Deployment, Produktionszugriff oder Secrets-Zugriff. Patch-Artefakt: `/tmp/opencode/owned-variant-foundation-20261010.patch` (nur Rust-Diff; Berichte liegen im Worktree). Owner-Main-Tests folgen nach Integration.

## Quellen

- `README.md`, insbesondere Händler-spezifische Parser und Preis-/Current-Vertrag.
- `research/checkin-20261010-msg-owner-fallback.md`: Eigentümerplan; vollständige MSG-Staffeln und Reconciliation explizit offen.
- `crates/ingestion/src/traders/mod.rs` und `crates/store/src/public/prices.rs`: tatsächliche Variante-/Aufzeichnungs-/Current-Verträge.
