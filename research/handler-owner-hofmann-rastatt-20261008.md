# Handler-Owner Hofmann Rastatt — Feedback #5530

## Owner-Nachprüfung nach RAM-Erweiterung am 09.10.2026

`cargo test --locked -p schrott-mcp-ingestion hofmann`: zehn bestanden,
darunter alle vier Rastatt-Tests. Kompilierter Rust-Livehandler: HTTP 200,
255841 Bytes, 22 Preise, ein Batterie-Skip, Quellenstand 30.09.2026.
Damit sind die früheren OOM-bedingt offenen lokalen Prüfungen nachgeholt.

Im nächsten Schritt werden Starterbatterien explizit auf `batterien-blei`
gemappt (150 EUR/t, nicht elementares Blei). Dieser Katalogeintrag kommt mit
Trapper in den gemeinsamen Release. Die geänderte Regression besteht alle
vier Rastatt-Tests; erneuter Rust-Livehandler liefert HTTP 200 und **23 Preise
ohne Skips**, darunter 150 EUR/t Starterbatterien. Nullpreise werden ebenfalls
abgelehnt. Workspace-Gate, Deployment und regulärer Produktionsnachweis offen.

## Stand / Übergabe (09.10.2026)

Implementiert, **noch nicht rollout-ready**: erfolgreiche abschließende Rust-Regressionen und Live-Ausführung des kompilierten Handlers sowie regulärer Produktionsscrape fehlen. Kein Deployment und keine Produktionsdatenbank-Schreiboperation vorgenommen. Ownership bleibt offen.

- Branch: `handler/hofmann-rastatt-20261008`, Basis `072cf25` (eigener Worktree aus HEAD).
- Aktiver Worktree: `/root/Documents/hofmann-rastatt-20261008`. Wegen vollem `/tmp` aus `/tmp/opencode/hofmann-rastatt-20261008` kopiert und `git worktree repair` ausgeführt. Die alte Kopie ist kein zweiter Handler/Branch und wurde nicht als aktive Arbeitskopie benutzt.
- Handler: `crates/ingestion/src/traders/handlers/hofmann_rastatt.rs`; genau eine Registry-Zeile in `handlers/mod.rs`.
- Regression: reduzierte vollständige Live-DOM-Fixture `handlers/fixtures/hofmann_rastatt_20260930.html`, vier Rust-Tests direkt beim Handler.
- Commit: der Implementierungscommit ist über `git log -1 --format='%H %s' handler/hofmann-rastatt-20261008 -- crates/ingestion/src/traders/handlers/hofmann_rastatt.rs` eindeutig auflösbar (Commit-ID kann nicht in denselben Commit eingebettet werden).

## Beleg / Betreiber / Preisrichtung

Am 08.10.2026 live erfolgreich abgerufen:

- https://hofmann-entsorgung.de/preise-und-verguetungen/ (HTTP 200; Roh-HTML `/tmp/opencode/hofmann-live.html`, 255845 Bytes).
- https://hofmann-entsorgung.de/impressum/: HOFMANN GmbH, Werkstraße 6a, 76437 Rastatt; HRB 521673 AG Mannheim; Ralf Hofmann. Passt zu Dossier und vorhandener Betreiberprüfung vom 01.10.; nicht Hofmann Metall GmbH in Sachsen und nicht Schrott Hofmann Mannheim.

Tabelle `table.hmp-table`, `Sorte / Preis pro t`, Abschnitt `Preise Schrott & Metalle / Für Privatkunden`. Ausweispflicht bei Auszahlung, Barauszahlung bis 1.999,99 EUR, darüber Gutschrift aufs Kundenkonto: Händler bezahlt Anlieferer. Verlinkte Abfall-PDF enthält hingegen Entsorgungskosten und ist ausdrücklich keine Preisquelle des Handlers. Vor-Ort-Einstufung und mögliche kurzfristige Preisänderungen bleiben Betreiber-Vorbehalt, keine erfundene Garantie.

23 Quellsorten, jeweils Stand 30.09.2026 16:22 Uhr; **sämtliche Quellenwerte EUR/t, auch NE-Metalle**. Handler behält Quellenunit und Zahl (Millberry 10.550 EUR/t, Zinn 11.500 EUR/t; keine falsche Multiplikation). 22 gemappte Preiszeilen plus ein lauter Skip: `Bleibatterien / Starterbatterien` (150 EUR/t) hat keinen Katalogeintrag und darf nicht als Blei eingelesen werden.

Alle weiteren Sorten abgedeckt: Altblech; Mischschrott leicht/schwer; Handelsguss; Blei alt; Zinkblech alt; Zinn; Alu-Geschirr; Alu bunt eisenfrei; saubere Alu-Felgen; V2A; Messing raff/schwer; Bronze; Elektromotoren; Kupferkabel mit/ohne Stecker; Kupfer leicht/schwer/Draht/Berry/Millberry. Gleichartige Sorten behalten getrennte Varianten. Unqualifizierter Kupfer Draht wird nicht als blanker Berry oder Millberry erfunden.

## Fehlerverhalten / Historie

- Bespoke Parser, keine neue Dependency, keine gemeinsame Parsing-Abstraktion; vorhandene HTTP/Zahlen/Datum-Helfer.
- Unbekannte Sorten und Anfragepreise laut in `skipped_labels`; keine statischen Ersatzpreise.
- Struktur-, Einheiten-, Preisrichtungs-, negative-/Range-Preis-, Duplikat- und Datumwechsel werden abgelehnt, nicht geraten.
- `HandlerOutcome` hat nur ein gemeinsames `published_at`: abweichende Veröffentlichungs-Tage zwischen gemappten Zeilen scheitern ehrlich mit `row-date support required`. Kein Maximaldatum einer Sorte für alle anderen erfunden. Uhrzeit dient Validierung; Datum wird wie bestehende Handler als UTC-Mitternacht gespeichert.
- HTTP 429 scheitert sichtbar im normalen HTTP-Helfer; keine Request-Schleife/Blockumgehung, kein Cache-Erfolg. Regulärer Scheduler versucht später erneut.
- Bestehender `record()`-Pfad bleibt unverändert und append-only. Dossier-Frontmatter unverändert, frühere Timeline erhalten. README Abschnitte Queryable data model, Feedback triage, Deploy/Logging sind Referenz; kein manueller Prod-Preiswrite.

## Tatsächliche Testresultate / Ressourcenblockade

1. Erster `cargo test -p schrott-mcp-ingestion hofmann_rastatt --lib`: echter Compilerfehler wegen nicht vorhandener `regex`-Dependency. **Behoben**, Parser jetzt mit lokalen String-Prüfungen ohne neue Dependency.
2. Wiederholte Cargo-Test-/Check-Läufe (auch ein Buildjob, ohne Incremental/Debug) wurden durch OpenCode-Serverneustarts abgebrochen; kein erfolgreicher Cargo-Testabschluss behauptet.
3. Isolierter `rustc --test`-Harness gegen existierende echte Workspace-rlib/rmeta und die unveränderte Handlerdatei ausführbar: **3/4 Tests bestanden**; fehlgeschlagener Test war eine Fixture-Mutation, die global `40 €/t` ersetzte und dadurch auch `140 €/t` zu `1auf Anfrage` machte. Auf `replacen(..., 1)` korrigiert. Finale Neuausführung bislang durch Ressourcen/Neustarts blockiert; nicht als grün gemeldet.
4. Rust-Liveharness gestartet, bislang kein erfolgreicher kompilierten Handler-Liveausgabebeleg. HTTP-Livequellenbeleg und Fixture sind vorhanden; beides ersetzt keinen Handler-Livetest.

Konkrete Umgebungsbefunde: `/tmp` zunächst 1,9 GB/100%, Handler-Schreiben zweimal `ENOSPC`; Root-Dateisystem später 38 GB/100%; Host 3814 MB RAM, kein Swap, etwa 429 MB verfügbar, mehrere konkurrierende Rust-Builds aus anderen Owner-Worktrees. `git worktree move` über Dateisystemgrenze scheiterte mit `Invalid cross-device link`; eigene Kopie/Repair löste nur den Worktree-Speicherort. Keine fremden Targets, Prozesse oder Daten gelöscht/gestoppt.

## Genau offene Schritte für Parent-Koordination

1. Ressourcen/Builds seriell koordinieren; dann `cargo test -p schrott-mcp-ingestion hofmann_rastatt --lib`, `cargo test --workspace`, `cargo check --workspace` und `cargo run -p schrott-mcp-ingestion --example live_handlers bw-rastatt-hofmann` im Owner-Worktree. Erwartung: 22 Zeilen EUR/t, ein Batterie-Skip, `published_at=2026-09-30T00:00:00+00:00` (bei unveränderter Quelle).
2. Review des isolierten Commits gegen Basis `072cf25` und Auftrag #5530; erst nach Test-/Livegate rollout-ready erklären.
3. Parent integriert und rollt **beide Binaries gemeinsam seriell** gemäß README aus; kein eigenständiger Owner-Rollout. Keine Preis-/Historienreparatur per SQL.
4. Regulären Scheduler-Tick abwarten (15-min-Tick, gestaffelte 6h-Fälligkeit), nicht `/api/ingest/run` zum Schein eines regulären Nachweises verwenden.
5. Read-only Nachweis nach Rollout: `ingestion_steps` für `trader:bw-rastatt-hofmann` auf erfolgreichen regulären Step prüfen, `raw_fetches` HTTP 200 prüfen; `prices`/`current_prices`/`v_current_prices` für den Slug auf 22 unterscheidbare Sorten/Varianten, EUR/t, Datum, Quellen-URL und `haendler_angabe/published` prüfen. Tatsächliche Run-/Step-IDs, Timestamp, Binary-Commit und Ergebnis hier ergänzen. HTTP 429 bleibt ein offener realer Fehler bis ein regulärer Versuch erfolgreich ist.
