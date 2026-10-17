# ELNO: freigegebene Aliasintegration, 17.10.2026

**Gebundener Handoff:** Alias, echte Fixture und Regression in Main integriert, uncommittet. Echter Cargo-Rot/Grün-Nachweis, Post-Fmt-ELNO-Gate und frischer DB-freier Liveabruf bestanden. **Workspace-Gate unvollständig: Harness-Timeout nach 120 s, kein PASS.** Keine laufenden Pflichtprozesse mehr, kein Deployment. Nachweise gelten für `8d0bd493` plus drei ELNO-Overlays, nicht für den inzwischen fortgeschriebenen Main-Commit `a147b164` einschließlich 60er-Welle.

## Auftrag und Isolation

Owner-Freigabe: ausschließlich zusätzliches exaktes `Cu-Kerze, sauber/ neu`-Alias derselben Händlerzeile, echte neue PDF mit Herkunft und Regression. Keine globale Kupferharmonisierung, keine Änderung an `kupfer-berry`, Variantenidentität, 36er-Gate oder Preis-/Einheitenschutz. Kein Commit/Push, kein Produktionszugriff, kein Deploy.

Vor Beginn waren alle vier vorgesehenen Code-/Fixture-/Bericht-Zielpfade sauber bzw. nicht vorhanden. Ausgangs-HEAD: `8d0bd4932a887dc64cdb613d381ebb565628be36`. Main-Handler vor Änderung SHA-256: `01414d537a5ad5f2f35d0644a064e29a12d7035cb2b792061b20b17dfadd34cb` (identisch zum Diagnosebericht).

## Primärfixture

Die neue Binärfixture wurde erneut direkt vom Betreiber heruntergeladen, vor dem Schreiben auf Größe/Hash geprüft, nicht synthetisch erzeugt:

- <https://www.elno-container.de/wp-content/uploads/2026/10/Preisliste_16.10.pdf>
- HTTP 200, 210166 Bytes, SHA-256 `74ba81979f1d8a1b7c83a71798bcb57b09e0597a2bd9c5f82d3dd97944cb76c6`.
- Pfad: `crates/ingestion/src/traders/handlers/fixtures/elno-20261017.pdf`; Herkunftsmetadata daneben in `elno-20261017.md`.
- Vollständige Zeilen-/Einheiten-/Vorzeichenprüfung: [Diagnosebericht](checkin-20261017-1600-elno-pdf-review.md). Kein veröffentlichtes Datum erfunden.

## CodegenScope: gebundener Altstand, keine aktuelle Main-Welle kompiliert

Alle Cargo-Befehle laufen in `/root/Documents/schrott-mcp-elno-alias-20261017`, einem mit `git archive HEAD` beim damaligen HEAD **`8d0bd4932a887dc64cdb613d381ebb565628be36`** erstellten sauberen Export. Diese feste Basis bleibt auch nach Änderungen von Main erhalten. Ausschließlich diese drei Quelldateien wurden darübergelegt:

1. `crates/ingestion/src/traders/handlers/elno.rs` (zunächst **nur Test**, Alias erst nach echtem Rotgate).
2. `crates/ingestion/src/traders/handlers/fixtures/elno-20261017.pdf`.
3. `crates/ingestion/src/traders/handlers/fixtures/elno-20261017.md`.

Vor dem Rotgate alle **4429 Blobs der festen Altbasis** gegen Git-Objekthashes geprüft: einzige bestehende Quelldatei mit Abweichung ist `elno.rs`; alle **3892 Dossiers identisch zur Altbasis**. Nach Main-Fortschreibung erneut explizit gegen `8d0bd4932a887dc64cdb613d381ebb565628be36` verifiziert, unverändertes Ergebnis; alle drei Main-/Export-Zielquellen weiterhin byteidentisch. Die fremden uncommitteten Altdateien und neuen Wellen sind damit nicht im Buildinput. Kein `cargo` im Main-Workspace ausgeführt.

Main steht bei Wiederaufnahme inzwischen auf **`a147b164b7a325a5f68da6b652138c408913081c`** und enthält laut Owner die geprüfte 60er-Welle. **Dieser Auftrag testet diese neuere Kombination nicht** und behauptet weder einen integrierten Wellenbuild noch Workspacegrün für `a147b164`. Sämtliche hier ausgewiesenen Cargo-/Codegen-/Live-Ergebnisse gelten ausschließlich für die feste Altbasis plus drei ELNO-Overlays. Die geschützten 143 Altdateien wurden nicht bearbeitet.

Unverändertes `crates/ingestion/build.rs`, SHA-256 `3302f9589572dd8108aaded9a9ee702876bcc18110e18cb8efdc7700057e4888`, liest laut Zeilen 363–364 `CARGO_MANIFEST_DIR/../../dossiers`: also den Export, nicht Main. Vorhandenes `/root/Documents/schrott-mcp/target` dient ausschließlich als Cargo-Artefaktcache. Kein eigener Releasebuild, keine breite Löschung.

Auch tatsächlich ausgeführtes Buildskript geprüft: `target/debug/build/schrott-mcp-ingestion-87a0409a2c948ddb/output` nennt den Export-Dossierpfad und **3892 einzelne Dossierdateien**, sämtlich unter `/root/Documents/schrott-mcp-elno-alias-20261017/crates/ingestion/../../dossiers/`, danach `seed codegen ok`. Damit nicht nur vermuteter, sondern nachgewiesener Codegen-Inputscope.

Cargo-Umgebung für sämtliche nachstehenden Builds/Tests:

```sh
env PATH=/root/.cargo/bin:/usr/local/bin:/usr/bin:/bin \
  RUSTUP_TOOLCHAIN=stable \
  CARGO_TARGET_DIR=/root/Documents/schrott-mcp/target \
  CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 \
  cargo ...
```

## Regression und Verifikationsstand

Neuer Test im bestehenden ELNO-Modul: `actual_october_pdf_preserves_kerze_grades_and_quote_guards`. Er extrahiert beide echten PDFs über `extract()`, erwartet jeweils 38 numerische Quotes/36 zugeordnete Preise, identische eindeutige Material-/Variantenschlüssel und dieselben drei bekannten Skips. Neue Kerze: 10,30 EUR/kg, ursprüngliches neues Label; kein `published_at`. Mutationen prüfen Minus/Plus/Null, Range, €/t, €/kg, €/g, Fremdwährung, Anfrage, fehlende Zeile und Duplikate einschließlich beider Aliase derselben Sorte.

### Echtes Cargo-Rotgate, vor Aliasänderung

```sh
cargo test --locked --offline -p schrott-mcp-ingestion elno -- --nocapture
```

Im sauberen Export mit nur neuem Test/Fixtures: **Exit 101, 5 bestanden / 1 fehlgeschlagen**, neuer Test an `parse_text(&text, URL).unwrap()` rot:

```text
incomplete ELNO price list: only 35 mapped prices
test traders::handlers::elno::tests::actual_october_pdf_preserves_kerze_grades_and_quote_guards ... FAILED
```

Cargo kompilierte echte Workspace-Crates (`core`, `ingestion`, `store`) aus dem Export, meldete erfolgreiches Seed-Codegen und führte das Testbinary `target/debug/deps/schrott_mcp_ingestion-abcc22196fa6703b` aus. Build 34,01 s, Tests 0,10 s. **Kein Scratch-Typschalenbeweis.**

Erst nach diesem Rotgate einzeilig ergänzt:

```diff
-        "u-Kerze, sauber/ neu" => ("kupfer-berry", "Kerze, sauber/neu"),
+        "u-Kerze, sauber/ neu" | "Cu-Kerze, sauber/ neu" => ("kupfer-berry", "Kerze, sauber/neu"),
```

Zusätzliche Quelltextprüfung: Vor `#[cfg(test)]` ist der gesamte Produktionscode nach Rückersetzung nur dieses Aliases **byteidentisch zur Altbasis `8d0bd493`**. Fetch-/Extraktionsgrenzen, alle Katalogidentitäten, Count- und Quote-Guards wurden somit nicht verändert.

### Cargo-Grüngate und Formatierung

Derselbe echte Cargo-Befehl nach ausschließlich dem Einzeilenalias: **Exit 0, alle 6 ELNO-Tests bestanden**, 0,10 s Testlauf. Anschließend `cargo fmt --all` im Export und `cargo fmt --all -- --check` erfolgreich. Eine erneute vollständige Blobprüfung gegen die feste Altbasis nach Formatierung bestätigte weiterhin ausschließlich `elno.rs` als geänderte bestehende Datei. Formatierte Zielquelle in Main und Export ist byteidentisch.

Finale Ziel-SHA-256:

| Datei | SHA-256 |
|---|---|
| `elno.rs` | `836e848db78bb0493f23a6893e79ac81c0c03f92d895463161edca032de88717` |
| `fixtures/elno-20261017.pdf` | `74ba81979f1d8a1b7c83a71798bcb57b09e0597a2bd9c5f82d3dd97944cb76c6` |
| `fixtures/elno-20261017.md` | `9734c6d9cdaa9252e8a75f61959663b53614b5f84bcc5bf926bf91e7265b8cbd` |

Export-`Cargo.lock` SHA-256: `81fcb73b11e924fb6782426be3b2b55cfb1b2c46c92fe3049aadd09219fe4d9b`. Zum Handoff alle drei Quellen erneut Main gegen Export byteidentisch geprüft; Hashes unverändert. Main dabei weiterhin `a147b164b7a325a5f68da6b652138c408913081c`.

Post-Fmt derselbe gezielte Cargo-Test nochmals: **Exit 0, 6/6 bestanden**, 0,09 s (Build 30,66 s). `git diff --check` für die gebundenen geänderten Main-Dateien ebenfalls erfolgreich.

### Vorhandenes Workspace-Gate: Timeout, ausdrücklich kein PASS

Einziger gestarteter Workspace-Lauf im festen Export:

```sh
cargo test --workspace --locked --offline
```

Shell-ID `sh_14a9fd2fb001b0Ty78i9J4qu1H`. Log: `/root/.local/share/opencode/shell/42aa29071d6c1a29b482e0269f73393202254764/sh_14a9fd2fb001b0Ty78i9J4qu1H.out`.

Der gesamte Testbuild wurde nach **1m 44s erfolgreich beendet**. Auth meldete **4 bestanden**, Core **3 bestanden**. Die Ingestion-Suite begann mit **672 Tests** und protokollierte unter anderem alle sechs ELNO-Fälle erfolgreich; ein abschließendes Ingestion-/Workspace-Ergebnis fehlt. Danach meldete die automatische Shell-Benachrichtigung:

```text
Command exceeded timeout of 120000 ms.
Timed out before completion
```

**Exitstatus: Harness-Timeout/Abbruch, kein regulärer Cargo-Exitcode geliefert** (weder 0 noch einen numerischen Code erfinden). Die 120-s-Ausführungsgrenze betraf Build und Tests zusammen. Das ist ein externer Laufzeitabbruch, kein belegter Assertionfehler; dennoch kein vollständiger Workspace-Erfolg. Original-Shell-/Cargo-PIDs `1083081`/`1083082` sind anschließend nicht mehr vorhanden. Auf ausdrückliche Owner-Vorgabe **kein zweiter Workspace-Start**, kein Polling und kein Ersatz einzelner Tests als vermeintliches Workspacegrün.

### Frischer DB-freier Liveabruf: bestanden

Erst nach dem realen Workspace-Abbruch, unverändert aus demselben festen Export und mit derselben Cargo-Umgebung:

```sh
cargo run --locked --offline -p schrott-mcp-ingestion --example live_handlers -- be-spandau-elno-container-und-dienstleistungs
```

**Exit 0.** Cargo kompilierte das Example frisch (20,98 s), anschließend echte Betreiber-HTTP-Abfragen; `--offline` betrifft Cargo-Abhängigkeiten, nicht den Handlerabruf. Relevante Ausgabe:

```text
status=200 bytes=210166 published_at=None acceptances=0 alive=true
kupfer-berry 10.300 EUR/kg conf=Some(1.0) published_at=None [Cu-Kerze, sauber/ neu; Privatkunden; ...]
SKIP: Cu-MS Kühler (no exact catalog mapping)
SKIP: Sorte 3 (no exact catalog mapping)
SKIP: Elektronikschrott (Platinen, Netzteile, Rechner etc.): auf Anfrage
```

Ausgabe vollständig kontrolliert: **36 Preise**, alle EUR/kg und `published_at=None`, darunter beide getrennten `kupfer-berry`-Varianten Haardraht/Kerze; keine Entsorgungsgebühren als Ankaufpreis. Das Example ruft ausschließlich den Handler-/Beobachtungspfad auf, öffnet keine DB und führt keine Ingestion-Aufzeichnung aus.

Frisches `target/debug/examples/live_handlers` SHA-256: `91e80a34b1ef5ee1b47c91846e628ba3fa40f7ccd0ad5028ca9d316197d9f47a`. Dies ist ein lokales Debug-Artefakt, kein installierter Release.

### Abschlussmatrix / Owner-Restgate

| Gate | Ergebnis / Exit |
|---|---|
| Neuer echter Cargo-Test ohne Alias | Erwartet ROT, Exit 101; 5 alt grün / 1 neu rot |
| Derselbe Cargo-Test nach Alias | GRÜN, Exit 0; 6/6 |
| `cargo fmt --all` und Formatcheck | Erfolgreich, jeweils Exit 0 |
| Gezieltes ELNO-Gate nach Fmt | GRÜN, Exit 0; 6/6 |
| Einziger Workspace-Lauf | **UNVOLLSTÄNDIG**, externer 120-s-Timeout, kein regulärer Exitcode |
| Frisch kompiliertes DB-freies Live-Example | GRÜN, Exit 0; HTTP 200, 36 Preise |
| Zielpfad-/Hash-/Codegen-/Diff-Audit | Erfolgreich; nur gebundene ELNO-Zielquellen und dieser Bericht bearbeitet |

**Verbleibendes Owner-Gate:** Falls vollständiges Workspacegrün Freigabevoraussetzung ist, muss Owner einen ausreichend langen weiteren Lauf explizit freigeben. Ein späterer Build auf `a147b164` plus ELNO-Fix wäre zudem ein **anderer Integrationsscope** und müsste gesondert geprüft werden. Hier keine Behauptung, dass aktuelle 60er-Welle mit dem Fix gebaut oder vollständig getestet wurde. Keine Pflichtprüfung wird als noch laufend oder als erfolgreich verschleiert.

## Releasegrenze

Der frühere Release `e8438e24` bleibt uninstalliert und enthält diesen Fix nicht. Dieser Auftrag installiert nichts. Lokales Testgrün ist weder Deployment noch Nachweis eines erfolgreichen regulären Produktions-Schedulerlaufs.
