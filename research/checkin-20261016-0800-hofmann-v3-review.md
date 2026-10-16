# Unabhängiger Hofmann-V3-Nachreview – 16.10.2026, 08:00

## Gateentscheid

**PASS für den geprüften V3-Code-/Patchumfang, mit verbleibenden Owner-Releasegates. Keine Deploymentfreigabe.** F1 und F2 sind tatsächlich behoben: Meine ursprünglichen vier Tests wurden **byteidentisch**, nicht nur als bearbeitete Nachfolger, gegen V3 ausgeführt und bestehen. Zusätzlich bestehen die acht eingereichten Integrationstests, 17 Hofmann-Tests, 43 Store-Librarytests und zwei neue unabhängige Tests mit unter anderem **48 Zeitordnungsfällen**. Keine neue Implementierungssperre im untersuchten Hofmann-Vertrag gefunden.

Der 745er-Workspacebericht wurde nicht als Ersatz für diese Prüfung verwendet. Kein eigener vollständiger Workspace-/Releasebuild behauptet. Keine Produktionszugriffe, Deployments, Commits oder Delegation; keine Hauptrepo-Code-/Dossieränderung. Alle Cargoaufrufe sequenziell. Die 143 fremden offenen Dateien sind frisch SHA256-identisch zur Schutzbaseline.

## Artefakt-, Basis- und Assertionsbindung

Geprüftes Verzeichnis: `/root/Documents/checkin-20261016-0300-hofmann-upgrade-isolated/`.

- Main frisch **`f476ebd1750e9c28fa94d2c82af1414a40297b2c`**. `crates/`, Root-Cargo.lock/Cargo.toml und rust-toolchain.toml zwischen Main und V3-Basis `81a6483208ade84c592e89a8684f56c9774dcea7` identisch. Der Kandidat ist daher nicht durch eine andere Rustbasis entwertet.
- `manifest-v3.json`: alle acht Quellhashes, beide Patchhashes, sämtliche benannten Loghashes sowie Dispositions-/Auditdateihashes frisch geprüft, alle passend.
- Vollpatch **`hofmann-upgrade-v3.patch`**, SHA256 `e2654e5deef3761455f666720671902298cc385cfeb08ccbd03f87010b7abfe8`: gegen Git-Basis vollständig im Speicher rekonstruiert, alle acht Pfade identisch zur getesteten Quelle.
- Inkrementalpatch **`hofmann-v2-with-reviewer-to-v3.patch`**, SHA256 `3b404f297753ec4bce5ea77fade0a2d54e4a78faffffbcd77bb8d16f14e0f5bb`: gegen gesichertes V2 **einschließlich originaler Reviewerdatei** rekonstruiert, alle fünf Pfade identisch zur finalen Quelle. Nicht auf einen nackten V2-Patch ohne Reviewerdatei anwenden.
- Vollständiger Vergleich der 4.400 Basisdateien: ausschließlich die sechs angegebenen bestehenden Dateien geändert; neue Implementierung `hofmann_tin.rs` und eingereichte Integrationstestdatei gehören zum gebundenen Vollpatch. Meine zwei zusätzlichen Reviewer-Testdateien sind gesonderte Prüfartefakte, **kein** unbemerkt hinzugefügter Bestandteil dieses Patches.
- Ursprüngliche Reviewerdatei unter `revision-2/source/crates/store/tests/review_hofmann_v2.rs` unverändert: SHA256 **`59c074948fe2baa0e084264d1ac5ea23872096f872f7a69d2fc59456ca253db2`**. Als byteidentische Kopie `source/crates/store/tests/reviewer_original_v2_on_v3.rs` frisch ausgeführt: **4/4 grün**. Damit ist die frühere rote Assertion selbst gebunden; Grün hängt nicht von einer gelockerten Nachfolgeassertion ab.
- Bearbeitete eingereichte Reviewerdatei: SHA256 `62ba825ba574a524a9582f4158db925ed009052230704c91c3077e893044af10`; Diff gelesen: ursprüngliche ID-/Kalender-/Bootassertions erhalten, Preiswertassertions ergänzt, vier neue Tests. Alle ursprünglichen V2-Logs und mein rotes V2-Adversariallog unverändert.

## F1 – Zeitordnung bis zur tatsächlichen Current-ID: PASS

**Echter Code:** `hofmann_tin.rs::guard_write_on` liefert den normalisierten eingehenden UTC-Schlüssel zurück. `prices.rs::record_price_on` verwendet ihn beim abschließenden Pointervergleich und normalisiert dort auch den bereits gespeicherten Current-Zeitstring **zum Vergleich im Speicher**. Es bleibt nicht mehr bei einem korrekten Guard mit anschließend falschem Rohstringvergleich.

Das gilt auch **vor** dem ersten Ledger für exakt identifizierte Hofmann-Zinn-Quellkeys: Händler-Slug, Material `zinn`, Varianten leer/alt, exakte URL. Der erste typisierte Write fällt deshalb nicht mehr wegen eines noch fehlenden Ledgers in die alte Rohstringordnung zurück. Nach einer Entscheidung schützt der Guard die beiden betroffenen Keys auch bei generischen Writes. Andere Händler/Materialien/Varianten werden dadurch nicht auf eine neue globale Zeitpolitik umgestellt.

Frische Nachweise:

- Original: generisch `.1Z → .100000001Z`, Preis 12 → 13, tatsächliche neue ID wird Current.
- Original: generischer Current `...01Z`, danach typisiert `...01.000000001Z`, Preis 12 → 13, neue ID wird Current.
- Eingereichte Erweiterungen: beide Varianten, beide zweiten Writer, unterschiedliche UTC-/Fractionformen, Same-instant-Ties über höhere Preis-ID, Erstwrite ohne Ledger.
- **Eigene zusätzliche Matrix: 48 Kombinationen** aus Ledger vorhanden/nicht vorhanden, generischem/typisiertem zweiten Writer, beiden Varianten und sechs Zeitpaaren. Einschließlich kürzerer/längerer Fraction, `Z`/`+00:00`, gleicher Zeit in anderer Darstellung, Schalttag-zu-März- und Jahreswechsel.
- Jeder Matrixfall prüft **Current-ID und Preis 13**, nicht nur ein erfolgreiches Return oder Historycount. Bei strikt späterer Zeit wird anschließend ein älterer generischer Preiswechsel versucht: abgewiesen, Preise/Current/Ledger unverändert.

**Historygrenze korrekt:** Für jeden Matrixfall sämtliche Spalten der alten Preiszeile vor/nach Preiswechsel identisch. Bestehende Zeitstrings werden nicht per Migration umgeschrieben. Die bisherige Dedupe-/Refreshsemantik einer wirklich neu beobachteten numerisch gleichen Quote bleibt getrennt davon bestehen; kein allgemeiner Append-only-Vertrag für alle Nicht-MKM-Zeilen behauptet.

## F2 – strikter Kalender und Dependencykante: PASS

`utc_key` verwendet nun tatsächlich `chrono::DateTime::parse_from_rfc3339` statt SQLite-Datumsakzeptanz und erzeugt einen festen UTC-Nanosekundenschlüssel. Strukturelle UTC-/Fractiongrenzen bleiben davor bestehen; Leap Seconds werden ausdrücklich abgewiesen. Unterstützter Vertrag: gewöhnliche Sekunden, `Z` beziehungsweise `+00:00`, bis neun Fractionstellen. Kein Anspruch auf beliebige Offset-/Schreibvarianten von RFC3339.

- Meine unveränderte Ablehnungsassertion für **31.02.2026** besteht.
- Eingereichte Tests prüfen elf ungültige Kalender-/Uhrzeitinputs über observed_at, typisiertes ingested_at und Koexistenzentscheidung; jeweils alle drei persistierten Flächen unverändert. Generischer geschützter Writer verwirft einen unmöglichen **zukünftigen** Tag, nicht bloß einen bereits alten Replay.
- Echte Schalttage 2000/2024 und gültige Monatsenden bleiben akzeptiert.
- Eigener Zusatztest verwirft auch Jahrhundert-Nichtschalttag 2100-02-29, 30.02., mehr als neun Fractionstellen, leere Fraction, fehlende UTC-Kennung sowie außerhalb des erklärten Vertrags liegende Offsetformen; über typisierten **und generischen Vor-Ledger-Writer**, ohne persistierte Teilentscheidung.

**Dependency frisch strukturell geprüft:** Store-Cargo.toml ergänzt nur `chrono = { version = "0.4", default-features = false, features = ["std"] }`. Cargo.lock hat keine hinzugefügten/entfernten Packages, keine geänderten Versionen/Checksummen. Einziger geänderter Packageeintrag: `schrott-mcp-store@0.1.0`, Dependencyliste plus `chrono`. Die bereits gelockte Chrono-Version wird genutzt; kein neues Netzwerk-/Zeitzonen-/Clock-Erfordernis aus dieser Storekante. Tests liefen durchgehend `--locked --offline`.

## Frühere Retirementbefunde und Persistenzvertrag bleiben intakt

Handler-/Recorderdateien sind hashidentisch zur geprüften V2-Fassung; V3 ändert deren Präsenz-/Transportsemantik nicht. Frisch grün sind weiterhin:

- ausdrückliche Sortenkoexistenz einschließlich Anfragezeilen, keine Verwechslung mit fehlenden numerischen Preisen;
- Belegauswahl unter `BEGIN IMMEDIATE`, Same-ID-Interleaving entwertet den alten Beleg vor der transaktionalen Auswahl;
- ausschließlich belegtes altes `zinn/''`, präzise Current-Löschung, unveränderte retired Historienzeile, Nachbarn und individuelle Row-Dates;
- Rollback bei Preisinsert-, Pointerdelete- und Ledgerinsertfehler, keine Teiländerung der drei Flächen;
- dauerhafte einmalige Entscheidung, Reopen-/Replaygrenzen, erlaubte spätere echte Sorten und kein erneuter Sweep bei folgendem nur-alt.

Bewusst weiterhin keine Atomarität über die gesamte Hofmann-Tabelle, Acceptances oder beide Datenbanken. Eine validierte konservative Koexistenzentscheidung ist eigene Evidenz und darf nachfolgenden best-effort Preisfehler überleben. Fremde/zweifelhafte Altbelege werden nicht heuristisch gelöscht. Ein solcher erhaltener Current ist kein Beweis einer aktuellen Quellquote.

## Frische Testläufe und neue Evidenz

Arbeitsverzeichnis `source/`; `TMPDIR=/tmp/opencode`, `CARGO_BUILD_JOBS=2`, `CARGO_TARGET_DIR` auf isoliertes `target/`. Alle Läufe sequenziell; ausschließlich Wegwerf-DBs. Neue Logs unter `logs/review-3/`, ursprüngliche Evidenz nicht überschrieben.

| `cargo test --locked --offline …` | Ergebnis | Neuer Log |
| --- | --- | --- |
| `-p schrott-mcp-store --test reviewer_original_v2_on_v3 -- --test-threads=1` | **4 bestanden** | `reviewer-original-byteexact-on-v3.log` |
| `-p schrott-mcp-store --test review_hofmann_v2 -- --test-threads=1` | **8 bestanden** | `reviewer-v3-original-and-extended.log` |
| `-p schrott-mcp-store --test reviewer_v3_extra -- --test-threads=1` | **2 bestanden**, darunter 48 Matrixfälle | `reviewer-independent-v3-extra.log` |
| `-p schrott-mcp-ingestion hofmann_rastatt` | **17 bestanden** | `reviewer-fresh-v3-hofmann.log` |
| `-p schrott-mcp-store --lib` | **43 bestanden** | `reviewer-fresh-v3-store.log` |

Alle Exit 0. Die 4/8 Läufe enthalten Überschneidungen und werden nicht als unabhängige zusätzliche Produktfälle aufsummiert. Der 745er-Workspace bleibt ein hashgeprüfter Vorbefund; hier nicht erneut vollständig ausgeführt.

Eigene neue Testdatei: `source/crates/store/tests/reviewer_v3_extra.rs`, SHA256 `aee835c5b5ea3cbf6ae431bc902a8485397e1be180c7892d9f949fbdf9ce3a63`.

```text
reviewer-original-byteexact-on-v3.log
cbfbff125deac9da3fb5d1a3c96f632fd26e89cfe320dada2b4f90403869849a
reviewer-v3-original-and-extended.log
8a17ba1b42fb382a7c7b7edd88f16260e0714800965b2673f9880521e4880831
reviewer-independent-v3-extra.log
4b611f1d759222fe8576a4aea564a6cefdadd157afba7ab3e92c1d0ce8e5b217
reviewer-fresh-v3-hofmann.log
a4bf32a65ac22639ebc4836e49f6c2323389b846c8265e104ba4330f4a9c287c
reviewer-fresh-v3-store.log
14d7045618d535fc8f5399eda60463c40948a6d8a122aeebb94ce39946ef93af
```

## Verbleibende Owner-Releasegates – nicht durch PASS aufgehoben

1. **Genau gebundenen Vollpatch integrieren**, nicht nur das inkrementelle Delta ohne seine V2-/Reviewerbasis. Meine zwei weiteren Reviewer-Prüfdateien nicht als schon manifestierten Integrationsumfang ausgeben. Owner prüft tatsächlichen Diff und finalen Quellstand.
2. **Additive Schemaänderung ausdrücklich abnehmen:** `hofmann_tin_cutovers`, weiterhin user_version 6, keine Downmigration. Neue Tabelle bei schreibendem `PublicDb::open`, nicht beim Read-only-Worker. Kein Retirement/Historyrewrite beim Boot. Der v6-Boot-ohne-Ledger-Fall ist frisch grün; keine echte Produktionssicherung als Upgradeprobe verwendet.
3. **Reale Legacyform bestätigen:** konkrete Produktionszeile muss zum engen Label-/Quelle-/Mapping-/Preis-/Zeitvertrag passen. Hier keine Produktionsabfrage oder erneute Händlerquellenprüfung. Unpassender Altbeleg bleibt konservativ erhalten, nicht durch erweiterte Matcher „passend machen“.
4. **Exklusive neue Writer und belastbarer Rollback:** Alte Binaries ignorieren die Ledgergrenze; V2 stellt zusätzlich F1/F2 wieder her. Kein Mischbetrieb. Binarytausch allein ist kein Datenrollback. WAL-sichere Sicherung, genehmigter Ingestionstopp beziehungsweise geprüfter Binary-/DB-Restoreplan bleiben erforderlich; keine automatische Ledgerentfernung.
5. **Sauberer Releaseinput und Produktionsabnahme:** 143 fremde offene Dateien weiterhin ausschließen, normalen Boot-/Seedumfang separat prüfen, Artefakte binden und nach genehmigtem Rollout regulären Schedulerlauf mit Current-IDs, Historien-/Nachbarerhalt und Ledgerentscheidung read-only belegen. Keine erzwungene Ingestion oder Veröffentlichung durch dieses Review autorisiert.

**Endurteil: PASS im beschriebenen Implementierungsumfang; keine offenen F1/F2-Codeblocker. Owner-Integration und Produktionsrelease bleiben eigenständige, bedingte Gates.**
