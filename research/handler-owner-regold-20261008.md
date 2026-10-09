# reGOLD Handler #5520 — Owner / Übergabe

## Owner-Nachprüfung am 09.10.2026 nach RAM-Erweiterung

- `cargo test --locked -p schrott-mcp-ingestion regold`: **6 bestanden**.
- Der erste vollständige Lauf deckte eine falsche Testannahme auf: der Store
  dedupliziert unveränderte Kurse bewusst. Nach 20 Erstkursen und genau einer
  Änderung sind 21 Historienzeilen richtig, nicht 40. Die Regression prüft jetzt
  zusätzlich jeden aktuellen Wert sowie zwei Historienzeilen für den geänderten
  und je eine für die 19 unveränderten Feingehalte. Der Store wurde nicht geändert.
- Rust-Livehandler: HTTP 200, 13764 Bytes, **20 Preise**, keine Skips,
  `EUR/g`, `approx`, confidence 0.8, `published_at=None`; Betreiberkontakt
  Klosterstrasse 6-7 / 13581 Berlin korrekt extrahiert.
- `cargo fmt --all` erfolgreich. Workspace-Gate, Main-Integration und
  Deployment mit regulärem Produktions-Scrape bleiben offen. Die folgenden
  OOM-/Testbeschreibungen dokumentieren den früheren Übergabestand.

## Stand

- Worktree: `/root/Documents/schrott-mcp-regold-20261008`, aus Deploy-HEAD `072cf25`.
- Branch: `owner/regold-20261008`.
- Implementierungscommit: `1583137`; Folgecommit korrigiert die Katalogeinheit auf `EUR/g` und ergänzt die lokale DB-/Historienregression.
- **Nicht rollout-ready:** Cargo-Tests und Rust-Livehandlerlauf konnten wegen wiederholter Harness-Neustarts noch nicht abgeschlossen werden. Kein erfolgreicher Test wird behauptet.
- Produktion unverändert; keine DB-Writes, kein Deployment, kein erzwungener Ingest.

## Quelle / Semantik

`https://www.regold.de/vor-ort-ankauf.html` liefert dynamische Kurse bereits im Server-HTML, nicht erst per JS. Selektoren: `#kurse .k-value`, `.k-label`, `.k-price`; Ankauf-Überschrift ist Pflicht. 20 eindeutige Feingehalte: Gold 999/585/900/333/750; Zahngold 750/600; Silber 999/835/700/925/800/625/900; Platin 999/950/750; Palladium 999/950/500. Zahngold bleibt getrennt von Gold. Unbekannte Labels werden gemeldet, nicht geraten. Negative/Null-/Platzhalter-/Bereichspreise, falsche Einheiten und doppelte Material/Feingehalt-Schlüssel schlagen fehl.

Betreiber nennt die Preise unverbindliche Richtwerte: `price_kind=approx`, confidence 0.8, EUR/g ohne Umrechnung. Kein Preisdatum ausgewiesen: `HandlerOutcome.published_at=None`. Die bestehende zentrale Policy darf bei späteren vergleichbaren Preisänderungen ein Datum mit entsprechendem Basis-Marker ableiten; die erste Beobachtung bekommt kein erfundenes Datum. Bestehender append-only Recordpfad bleibt unverändert.

`https://www.regold.de/impressum.html`: reGOLD Edelmetallhandel UG (haftungsbeschränkt), GF Johannes Oduncu, Klosterstrasse 6-7, 13581 Berlin, mail@regold.de. Handler prüft Betreiberzeile und liest nur beschriftete Tabellenfelder; E-Mail-Entities werden dekodiert. Telefon aus Header `a.number_1`, nicht der gebührenfreie 24h-Service. Kein Dossier-Frontmatter geändert.

## Evidenz / Tests

- `cargo fmt --all` und `git diff --check`: erfolgreich.
- Sechs Regressionen implementiert: vollständige Liste/Feingehalte/approx, Redesign und ungültige Werte, unbekannte Labels/Selektor-Scope, Impressum/Betreiber, einmalige Registry/Provenienz sowie echte lokale Katalogaufnahme mit 20 + 20 append-only Beobachtungen und Datumprüfung.
- `cargo test -p schrott-mcp-ingestion regold` mehrfach begonnen, auch `CARGO_BUILD_JOBS=1`; Harness-Restarts brachen alle Versuche ab. Letzte Logs enthalten Dependency-Kompilierung, keinen Test-Pass. Kerneljournal meldete am 09.10.2026 06:00:52 einen OOM-Kill von `opencode`; kein Swap. Build-Ergebnisse daher offen, nicht als bestanden behandeln.
- HTTP-Liveprüfung per Python-stdlib am 09.10.: Preisquelle HTTP 200, 13763 Bytes, genau 20 eindeutige EUR/g-Zeilen; Impressum HTTP 200 mit passendem Betreiber/Adresse. Gold 999 119,81; Zahngold 750 89,95; Silber 999 1,71; Platin 999 47,36; Palladium 500 15,81. Werte unterscheiden sich erwartungsgemäß vom Fixture des 08.10. Das verifiziert Quelle/Struktur, **nicht** den noch ausstehenden Rust-Livehandlerlauf.
- Read-only Prod-Baseline: Trader-ID 10003 für diesen Slug; alle fünf Materialien existieren mit Einheit `EUR/g`; noch 0 Preiszeilen für diesen Trader. Keine DB geändert.
- Code-Review-Skill geladen; beide Review-Subagents wegen Subagent-Tiefenlimit nicht startbar. Manueller Standards-/Spec-Abgleich fand die Einheitenabweichung `g` und korrigierte sie auf `EUR/g`. Operative Tests/Deployment bleiben ausdrücklich offen.

## Genau offene Schritte (Parent seriell koordinieren)

1. Nach Entlastung des Hosts: `PATH=/root/.cargo/bin:$PATH CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/root/Documents/schrott-mcp-deploy/target cargo test -p schrott-mcp-ingestion regold`.
2. `cargo test -p schrott-mcp-ingestion`, dann `cargo test --workspace`; tatsächliche Ergebnisse hier ergänzen. Optional unabhängigen Review im Parent starten.
3. `cargo run -p schrott-mcp-ingestion --example live_handlers be-spandau-regold-edelmetallhandel`: erwartet HTTP 200, 20 EUR/g-approx-Preise, keine Skips, Betreiberkontakt und `published_at=None`; DB-freier Rust-Livetest erforderlich.
4. Erst danach rollout-ready melden. Parent übernimmt Commits in Release, baut und deployt **beide** Binaries koordiniert; kein paralleler Owner-Rollout.
5. Regulären Schedulerlauf abwarten (15-Minuten-Ticks, Handler 6h/hash-stagger). Kein POST-force-run als regulären Nachweis ausgeben.
6. Read-only Prod-Nachweis: erfolgreicher `ingestion_steps`-Eintrag für `trader:be-spandau-regold-edelmetallhandel`, 20 aufgezeichnete Zeilen ohne Skips/Canaries; 20 aktuelle Material/Feingehalt-Schlüssel, EUR/g, approx, source URL korrekt. Erste Beobachtung ohne erfundenes Publikationsdatum. Run/Step-ID, Zeiten und deployed SHA dokumentieren; bestehende Historie erhalten.

Owner bleibt für Fehlerbehebung und diesen Produktionsnachweis zuständig; Auftrag ist bis dahin nicht abgeschlossen.
