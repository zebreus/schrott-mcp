# Adversarial Nachreview Hofmann V2 – 16.10.2026, 07:30

## Gateentscheid

**HOLD.** Die drei ursprünglichen Gegenbeispiele sind in V2 substanziell korrigiert und ihre Regressionstests frisch grün. Die persistente Übergangsentscheidung und der atomare Belegnachweis sind keine bloßen Berichtbehauptungen. Neue eigene Tests zeigen jedoch: **Ein zeitlich neuerer, akzeptierter Preis kann weiterhin nicht Current werden**, sowohl über den generischen Writer als auch über den typisierten Hofmann-Writer nach einem generischen Write. Die normalisierte Guard-Zeit und die weiterhin rohe Pointer-Zeitordnung widersprechen sich. Zusätzlich akzeptiert die neue UTC-Validierung einen unmöglichen Kalendertag.

737 grüne Tests begründen daher keine Freigabe. Frisch ausgeführt: **17 Hofmann-Tests grün, 43 Store-Librarytests grün; vier eigene adversariale Tests: einer grün, drei rot.** Keine Produktionszugriffe, Deployments, Commits oder Implementierungsdelegation. Im Hauptrepo ausschließlich dieser Bericht; keine Code-/Dossieränderung.

## Artefaktbindung und Scope

Geprüft: `/root/Documents/checkin-20261016-0300-hofmann-upgrade-isolated/`, `hofmann-upgrade-v2.patch`, `source/`, Disposition, Manifest und relevante Logs.

- Main frisch bestätigt: **`5942a8ad3177ffcaf42ff27a02c6054724f72060`**. Sämtliche `crates/`-Blobs zwischen V2-Basis `81a6483208ade84c592e89a8684f56c9774dcea7` und diesem Main identisch.
- V2-Patch SHA256: **`1a2cf708f677401c4c5c9e48822f0c598e4ac6bc74bfceb5a928d8671397613e`**. Manifest-Hashes aller fünf Quellpfade, drei Patchartefakte und sechs benannten Logs frisch bestätigt.
- Alle 4.400 Basisdateien gegen isolierte Quelle verglichen: genau die vier manifestierten bestehenden Rustdateien abweichend; neues Modul `crates/store/src/public/hofmann_tin.rs`. V2-Patch einschließlich Kontexten in Speicher gegen Basis angewendet: alle fünf Ergebnisse identisch mit `source/`.
- Meine neuen Tests liegen zusätzlich unter `source/crates/store/tests/review_hofmann_v2.rs`, **nicht im V2-Patch**. Keine bestehende V2-Implementierung wurde dafür geändert. Neue Reviewlogs liegen nur im isolierten Verzeichnis. Die manifestierten Quell-/Patch-/Originallogdateien bleiben unverändert.
- Alle **143** geschützten offenen Dateien gegen `/tmp/opencode/checkin-20261016-013049-baseline.json` erneut geprüft: keine SHA256-Abweichung.

## Ursprüngliche Befunde: tatsächliche Disposition

### R1: unbepreiste Sortenpräsenz – korrigiert

`hofmann_rastatt.rs:183–190` setzt private Präsenzbits aus den tatsächlichen Tabellenlabels **vor** der numerischen Preisprojektion. Struktur/Datum werden weiter validiert; ein Parsefehler liefert keinen veröffentlichbaren Snapshot. `Zinn: auf Anfrage` verschwindet daher nicht mehr als semantische Präsenz. `traders/mod.rs:451ff` persistiert ausdrücklich erkannte Koexistenz vor den best-effort Preiswrites.

Frisch grün: echtes Zwei-Zeilen-HTML mit unbepreistem Zinn und numerischem alt; ebenso numerisches Zinn/unbepreistes alt und beide auf Anfrage neben einem anderen Zahlenpreis. Folgender nur-alt-Lauf löscht den erhaltenen Zinn-Key nicht. Die frühere ausschließliche Zählung von `prices` ist entfernt.

**Bewusste Grenze:** Eine konservative Koexistenzentscheidung darf bestehen bleiben, auch wenn anschließende Preise scheitern. Das ist sachlich nachvollziehbar, weil sie nach vollständig erfolgreichem Parse reale Sortenpräsenz belegt und selbst nichts löscht; sie ist kein Preisimportnachweis. Ausschließlich zwei unbepreiste Zeilen ohne irgendeinen Zahlenpreis scheitern weiterhin am bestehenden Nichtleer-Parservertrag.

### R2: Same-ID-Beleg außerhalb der Transaktion – korrigiert

`hofmann_tin.rs:172–230`: `BEGIN IMMEDIATE` **vor** Current-Auswahl; keine extern vorselektierte Legacy-ID mehr. Der vollständige enge Vertrag wird auf dem unter Schreibreservierung gelesenen Datensatz geprüft: exakter Händler/Materialscope, leere Variante, URL/Quellenklasse/published, Label `Zinn`, positiver endlicher EUR/kg-exact-Preis mit confidence 1, keine Grenzen, map_v 2, page_stated/row_stated, kein fremder Producer, nicht neuer als Upgrade.

Ersatzpreis, Delete genau dieses Current-Pointers und Ledgerinsert liegen in derselben Transaktion. Frisch grün: der Same-ID-Gegenbeispieltest sowie Storetriggerfehler bei Preisinsert, Pointerdelete und Ledgerinsert mit anschließend unveränderten drei Tabellen und erfolgreichem Retry. Die entfernte alte Preis-Historienzeile wird nicht aktualisiert oder gelöscht. Scope-/Fremdproducerprüfungen sind echte ausführbare Tests, nicht nur Kommentare.

### R3: einmalige Grenze, Replay und spätere legitime Sorten – Kernsequenz korrigiert, Zeitrepräsentation noch fehlerhaft

`hofmann_tin_cutovers` speichert einmalig Händler/Material/Quelle, Entscheidungszeit und optional retired ID. Nach einer Entscheidung gibt es **keinen erneuten Legacy-Sweep**. Der zentrale `record_price_on`-Einstieg ruft den Guard vor Dedupe/Refresh auf; ein alter generischer Write kann die entfernte leere Variante nicht mehr einfach wegen fehlendem Current wiederbeleben.

Frisch grün: Reopen, Replay vor/an der Retirementgrenze, echte neue Sorte +1 ns, Rückdatierung gegen deren Current, später nur alt ohne erneutes Löschen sowie spätere beide-Sorten-Sequenz. Diese konkreten V1-Gegenbeispiele sind erledigt. Das nachfolgende neue Gegenbeispiel betrifft die **Weitergabe akzeptierter Zeitordnung bis zum Current-Update**, nicht das Fehlen eines Ledgers.

## Neuer Blocker F1 – P2: normalisierter Guard, aber lexikografischer Current-Write mit anderen Zeitstrings

**Code:** `hofmann_tin.rs:21–37, 99–109, 185–203`; `prices.rs:record_price_on`, abschließender Currentvergleich `(p.observed_at, price_id) > (old_observed.as_str(), old_id)`.

`utc_key()` normalisiert zum Vergleich auf neun Nachkommastellen. Der typisierte Writer normalisiert zudem seine eingehenden Zeitstrings. Der generische Writer **speichert jedoch weiterhin die erlaubte ursprüngliche Schreibweise**, und der abschließende Pointervergleich normalisiert weder eingehende noch bestehende Zeit. UTC-`Z` mit unterschiedlichen Fractionbreiten ist somit im Guard zeitlich richtig, beim Pointervergleich aber falsch geordnet.

### Gegenbeispiel A: zwei generische Writes nach Cutover

Echter PublicDb-/Storepfad auf Wegwerf-DB:

1. `zinn/''` bei 14.10. erzeugen; typisierter alt-Cutover am 16.10., 03:00:00Z.
2. Echte neue ungradierte Sorte über `record_price`, **03:00:01.1Z**, Preis **12** → ID 3, Current.
3. Noch neuere Beobachtung über denselben Writer, **03:00:01.100000001Z**, Preis **13** → ID 4, Write erfolgreich.
4. Erwartet Current ID 4. **Tatsächlich weiter ID 3/Preis 12.**

Der Guard lässt den korrekten +1-ns-Fortschritt zu. Beim rohen Stringvergleich sortiert `.1Z` aber nach `.100000001Z`. Die neue Beobachtung wird appendiert, ohne den Current-Pointer zu aktualisieren.

### Gegenbeispiel B: typisierter Writer nach generischem Current

Nach demselben Cutover:

1. Generischer alt-Write **03:00:01Z**, Preis **12** → ID 3.
2. Typisierter alt-Write **03:00:01.000000001Z**, Preis **13** → ID 4, Write erfolgreich.
3. Erwartet Current ID 4. **Tatsächlich weiter ID 3/Preis 12.**

Der typisierte Writer erkennt den bestehenden Current über `utc_key` korrekt als älter und speichert den neuen Input normalisiert. Der Store-Pointervergleich vergleicht danach trotzdem gegen den bestehenden rohen `...01Z`-String: `.` sortiert vor `Z`.

**Bedeutung:** Der neue Replayguard schützt nicht hinreichend die gesamte zugesagte Zeitordnung aller öffentlichen Writer. Ein erfolgreich gezählter Hofmannimport kann einen älteren Current stehenlassen. Das ist nicht bloß ein ungültiger Timestamp-Test: beide Gegenbeispiele benutzen gültige UTC-RFC3339-Eingaben, die V2 ausdrücklich akzeptiert. Die rohe Vergleichsschwäche ist im generischen Store älter; V2 übernimmt sie aber in seinen neuen präzisen Cutover-/Writervertrag.

**Abnahmeanforderung:** Zeitordnung für guarded Hofmann-Keys bis zur tatsächlichen Pointerbewegung konsistent machen, einschließlich bereits gespeicherter anderer UTC-Schreibweisen. Nur den Guard zu ändern oder künftige typisierte Inputs zu normalisieren reicht nicht. Keine pauschale Umschreibung historischer Preiszeilen. Beide Tests müssen mit Preiswechsel und tatsächlicher Current-ID-/Wertprüfung grün werden.

## Neuer Befund F2 – P3: SQLite-Datumsakzeptanz ist keine strikte UTC-Kalendervalidierung

`utc_key()` prüft `julianday(raw) IS NOT NULL`, Stringlänge und Fraction; SQLite akzeptiert dabei **`2026-02-31T03:00:00Z`**. Eigener Test reicht diesen unmöglichen Tag über die neue öffentliche `record_hofmann_tin_price`-API ein und erwartet Ablehnung. **Tatsächlich wird der Write einschließlich Cutoverentscheidung akzeptiert.** Der Kalender wird nicht strikt validiert; der zurückgegebene Schlüssel enthält weiterhin den ursprünglichen unmöglichen Tag.

Der normale Scheduler erzeugt `observed_at` aus Chrono und ist von diesem Input nicht direkt betroffen. Daher P3/Vertragshärtung, kein zweiter unabhängig behaupteter Produktionsfehler. Vor einer Zusage „ungültige UTC-Formate werden abgewiesen“ ist strikte Kalenderprüfung beziehungsweise ein explizit enger Eingabevertrag nötig; SQLite-`julianday` allein genügt nicht.

## Eigene ausführbare Evidenz

Alle Cargoaufrufe **sequenziell**, `--locked --offline`, `TMPDIR=/tmp/opencode`, `CARGO_BUILD_JOBS=2`, `CARGO_TARGET_DIR=/root/Documents/checkin-20261016-0300-hofmann-upgrade-isolated/target`, Arbeitsverzeichnis `source/`.

| Frischer Lauf | Ergebnis | Neuer Reviewlog unter `logs/review-2/` |
| --- | --- | --- |
| `cargo test --locked --offline -p schrott-mcp-ingestion hofmann_rastatt` | **17 bestanden**, Exit 0 | `reviewer-green-hofmann.log` |
| `cargo test --locked --offline -p schrott-mcp-store --lib` | **43 bestanden**, Exit 0 | `reviewer-green-store.log` |
| `cargo test --locked --offline -p schrott-mcp-store --test review_hofmann_v2 -- --test-threads=1` | **1 bestanden, 3 fehlgeschlagen**, Exit 101 | `adversarial-reviewer-v2.log` |

Eigene Tests in `source/crates/store/tests/review_hofmann_v2.rs`: zwei gültige Zeit-/Current-Gegenbeispiele, ungültiger Kalendertag, positiver additiver v6-Bootfall. Sie verwenden die echten öffentlichen Store-APIs; direkte Fixture-SQL-Inserts/DDL ausschließlich in neu angelegten Wegwerf-Datenbanken unter `/tmp/opencode/hofmann-v2-review-*`. Keine Testassertion der V2-Suite geändert. Gesamter 737er-Workspace wurde hier nicht erneut gelaufen.

Reproduzierbare Artefakthashes:

```text
review_hofmann_v2.rs
59c074948fe2baa0e084264d1ac5ea23872096f872f7a69d2fc59456ca253db2
adversarial-reviewer-v2.log
4cd0572f8935e2d42458b9d36f4d6dfd9b34aafa25a3d5fbabd80fb1586d38d5
reviewer-green-hofmann.log
72d2e9e642708f53f92395dfbedd3ad1322a252e29375b379090136a7e62e5ba
reviewer-green-store.log
5d71f798dc5ae083e1a1b9a8bc9dfb9ebc116be011e8a44c15ed0258d8623814
```

## Upgrade-, Boot- und Rollbackvertrag

- **DDL ist neu, nicht nur Logik:** `PublicDb::open` erstellt `hofmann_tin_cutovers` additiv vor den vorhandenen Migrationen. `user_version` bleibt 6. Kein automatisches Retirement beim Boot; die erste echte typisierte Beobachtung beziehungsweise validierte Koexistenz entscheidet. Read-only-Öffnung des Queryworkers führt diese DDL nicht aus.
- **Eigenständig geprüft:** Wegwerf-v6-Datenbank mit bestehender Preis-/Currentzeile, ohne Ledger-Tabelle, geschlossen und per neuem `PublicDb::open` geöffnet: Tabelle leer erstellt, sämtliche Preis-/Currentzeilen identisch, `user_version=6`. Test frisch grün. Zusätzlich V2-Reopen-/Rollbacktriggerfälle frisch grün. Das ist kein Test einer echten Produktionssicherung oder sämtlicher historischer DB-Versionen.
- **Historien-/Nachbarschutz:** Einmaliges Retirement löscht nur den belegten leeren Current-Pointer; die alte `prices`-Zeile bleibt. Später erlaubte Neubeobachtungen behalten die bestehende generische Refreshsemantik. Keine ganze Tabelle oder beide Datenbanken umfassende Transaktion behauptet. Preise anderer Händler/Materialien ohne passenden Ledger bleiben im generischen Guard unberührt; vorhandene Storetests frisch grün.
- **Rollback ist nicht allein ein Binarytausch:** Alte Binaries kennen den Guard nicht. Die additive Tabelle bleibt zwar erhalten, verhindert alte Writer aber nicht durch Trigger/DB-Constraints. Die Disposition benennt das korrekt. Ein Releaseplan muss Mischbetrieb ausschließen und beim Rollback entweder Ingestion stoppen oder eine ausdrücklich abgenommene sichere Wiederherstellung von Binary **und** DB-Zustand festlegen. Keine Downmigration/Entfernung des Ledgers aus diesem Review ableiten.
- **Produktionsshape:** Die enge page_stated-/row_stated-Matchingform und vollständiger Historienvergleich sind in den Tests vorhanden. Die aktuelle echte Legacyzeile habe ich nicht abgefragt; der separate Ownerbeleg muss vor Integration/Release exakt zu dieser V2-Auswahl passen. Kein Produktivbereinigungs-/GEO-/Seedabschluss aus den Tests ableiten.

## Restbedingungen für ein PASS

1. **F1 beheben und beide eigenen Gegenbeispiele grün nachweisen**, einschließlich konsistenter Pointerbewegung für gültige unterschiedliche UTC-Schreibweisen; ursprüngliche 17/43-Gates unverändert erhalten.
2. F2 entweder strikt beheben oder ausdrücklich auf vertrauenswürdig vorvalidierte Eingaben begrenzen und die derzeit weitergehende Validierungsbehauptung korrigieren; keine bloße Grünfärbung des Tests.
3. Neuer Patch-/Quellhash und erneute gezielte Verifikation des finalen Standes; Reviewer-Testdatei nicht ungeprüft als Teil des bisherigen V2-Manifests ausgeben.
4. Additive DDL, tatsächliches Legacy-Matching, exklusiver neuer Writer und DB-/Binary-Rollbackplan ownerseitig abnehmen. Das ist anschließend eine Releaseentscheidung, kein durch diesen Review genehmigter Produktionslauf.

**Endurteil: ursprüngliche drei Befunde korrekt adressiert; neue Zeitrepräsentationslücke mit echter roter Evidenz verhindert PASS.**
