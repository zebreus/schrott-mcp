# MKM: Diagnose des WARN vom 18.10.2026, 17:32 UTC

Stand: 2026-10-18, Untersuchung ca. 17:58–18:05 UTC. Auftrag strikt diagnostisch; `diagnosing-bugs` geladen. Keine Codeänderung, kein Build, Commit, Deployment oder Force-Ingest. Produktions-DBs ausschließlich SQLite-URI `mode=ro` und `PRAGMA query_only=ON`. Keine AGENTS.md, GLOSSARY.md oder einschlägigen ADR-Dateien gefunden.

## Ergebnis

**Reproduzierbare Quellkontrakt-Erweiterung, korrektes Fail-closed des bekannten MKM-Vertrags.** Die aktuelle Betreiberübersicht enthält **61 Artikel**, darunter neu und nicht freigegeben **`MKM-MS-01`, „Messing 58 blank neu“**. Der feste Parser-/Mapping-Vertrag und der unabhängig reviewte Oracle enthalten 60 Artikel. Der Parser meldet exakt eine unbekannte Identität; die Schreibgrenze verweigert solche Snapshots vor der Transaktion. Kein Hinweis auf HTTP-Ausfall oder ausgefallenen Dienst.

Das ist eine echte Produktions-Ingestionsstörung (seit diesem Lauf keine neue vollständige MKM-Aufnahme), aber **kein Beleg für einen fehlerhaften Guard**. Die Schutzprüfung darf nicht gelockert werden. Der zusätzliche Artikel braucht eine fachlich geprüfte Vertragserweiterung, nicht einen generischen Messing-Fallback oder das stille Ignorieren neuer Zeilen.

**Evidenzgrenze:** Die exakten HTML-Bytes des Produktionsabrufs 17:32 sind nicht als Body in `raw_fetches` gespeichert. Es gibt nur Status/Hash/Länge. Aktuelles HTML ist erfolgreich am echten bestehenden Parser und an der registrierten Fetch-Route reproduziert; der originale 17:32-Body wurde **nicht** bytegenau an `record_articles` erneut ausgeführt. Das vorhandene Inventory-Binary ruft diese Schreibgrenze nicht auf. Der vorhandene Owner-Rollout-Harness liest fest verdrahtete alte Fixtures und erzeugt isolierte DBs; er wurde nicht als vermeintlicher Live-Replay ausgeführt. Neuer Rustbuild war untersagt. Deshalb keine Behauptung einer vollständig erneut ausgeführten historischen Produktionsreproduktion oder eines grünen Rollout-Gates.

## 1. Zuerst roter Signalweg und Quellcapture

Private Artefakte: `/tmp/opencode/mkm-checkin-20261018-1800/` (Verzeichnis mode 0700, nach Entfernung eigener Test-DB-Reste unter 1 MiB). HTTP-Captures enthalten ggf. Session-Cookies; Header nicht in Berichte kopieren. Keine Credentials ausgegeben.

Zwei reine GETs mit begrenztem Timeout/Download:

```text
https://mkm-metals.de/metallpreise  HTTP 200, 256056 Bytes, unveränderte Ziel-URL
https://mkm-metals.de/impressum    HTTP 200,  27826 Bytes, unveränderte Ziel-URL
```

Capture `overview.html` SHA-256:
`cf7fdc8a555194e363ab847e69d91c1f1bf063e5c8f81833d08dd1bbf0aad05e`.

Bereits ausgeführter, schneller, deterministischer Offline-Replay (echter Parser, keine Produktionseinträge):

```sh
target/debug/examples/mkm_inventory \
  /tmp/opencode/mkm-checkin-20261018-1800/overview.html \
  /tmp/opencode/mkm-checkin-20261018-1800/impressum.html
```

Exit **1**, Ausgabe in `inventory.json`, stderr:

```text
Error: "article coverage diagnostics; not complete"
products: 61; numeric: 55; request_only: 5
diagnostics:
MKM-MS-01: unknown article: Messing 58 blank neu | /metallankauf/produkt/messing-58-blank-neu | Ankaufpreise für Messing & Rotguss
```

`numeric: 55` ist hier die bekannte, projizierbare Preispartition, **nicht** die Zahl aller numerischen Quellzeilen. Der neue Artikel bleibt mit `material: null` im vollständigen 61-Artikel-Inventar sichtbar. Der unabhängige HTML-Auditor zählt korrekt **56 numerische Quellzeilen + 5 Anfragen**.

Zusätzlich ausgeführt:

```sh
target/debug/examples/mkm_inventory --registered
```

Exit **1**, `Error: "registered route coverage diagnostics"`. Ausgabe `registered.stdout`: dieselbe einzelne Diagnose; sämtliche Artikelobjekte stimmen mit dem Offline-Capture-Replay überein. Dies ist die echte registrierte Fetch-/Parse-Route, **nicht** `record_articles` und keine Ingestion.

## 2. Minimierung und falsifizierbare Trennlinien

Nach dem roten Replay geprüfte Hypothesen:

1. **Ein neuer unbekannter Artikel ist hinreichender Auslöser:** Entfernen genau dieser Zeile im privaten Capture muss die Diagnose beseitigen.
2. **Weitere Layout-/Staffelabweichungen:** Nach dieser Entfernung bleiben andere Diagnosen oder Auditabweichungen bestehen.
3. **Persistenz-/Produktionszustandsfehler statt Quelländerung:** Der letzte erhaltene Bestand zeigt unabhängig vom fehlgeschlagenen neuesten Lauf Identitäts-/Mappingabweichungen. Der Release-Verifier allein kann das nicht entscheiden, weil er absichtlich den neuesten Lauf bewertet.

Kontrolle: Per `<tr>…</tr>` genau eine Zeile mit `MKM-MS-01` identifiziert (Assertion: genau 1) und nur diese aus einer **privaten Kopie** entfernt. Originalcapture unverändert.

```sh
target/debug/examples/mkm_inventory \
  /tmp/opencode/mkm-checkin-20261018-1800/overview-without-new-row.html \
  /tmp/opencode/mkm-checkin-20261018-1800/impressum.html
```

Exit **0**: `products=60, numeric=55, request_only=5, diagnostics=[]`.
Damit ist für die beobachtete Parserdiagnose diese einzelne zusätzliche Identität hinreichend und keine zweite notwendige Layoutänderung erkennbar. **Keine Handlungsempfehlung, diese Zeile produktiv zu entfernen.**

Unabhängiger bestehender Auditor:

```sh
python3 -B scripts/audit-mkm-overview.py \
  /tmp/opencode/mkm-checkin-20261018-1800/overview.html \
  --candidate-json /tmp/opencode/mkm-checkin-20261018-1800/inventory.json
```

Exit **1**, 8 Gruppen / 61 Produkte / 56 numerisch / 5 Anfragen. Ausschließlich drei Lücken zu `MKM-MS-01`: unbekannter Artikel, explizites Materialmapping fehlt, kein reviewter Oracle-Eintrag. Keine anderen Artikelvertragsabweichungen.

## 3. Neue Primärquelle unabhängig bestätigt

GET `https://mkm-metals.de/metallankauf/produkt/messing-58-blank-neu`: HTTP **200**, 56926 Bytes, keine Zieländerung. Capture `mkm-detail-MKM-MS-01.html`, SHA-256:
`8f6f584c0d98915d9a441128fea0ffb67d80e938f2983f1d192008ebe85880d0`.

Bestehendes `audit-mkm-overview.py::compare_details` auf genau diesen Artikel angewandt:

- Detail-Artikelnummer **MKM-MS-01**, stimmt mit Übersicht überein.
- Sichtbare Detailstaffeln stimmen vollständig mit den Übersichtsdaten überein: **ab 0 kg 6,20 €/kg; ab 5 kg 6,32 €/kg; ab 500 kg 6,51 €/kg**.
- `detail_contract_gaps=[]`.
- Übersichtsdatum `2026-10-18`, Text `heute`, title `18.10.2026, 15:13 Uhr`. Keine Zeitzone der Titel-Uhrzeit belegt; nicht zu einem UTC-Publikationszeitpunkt umgedeutet.

Das bestätigt Identität und Preisstaffel, **noch nicht** die richtige Katalog-Material-/Variantenentscheidung. Keine Browser-/externe-CSS-Sichtbarkeitsprüfung vorgenommen.

## 4. Produktionsbeleg und tatsächlicher Guard

Journal, exakt eingegrenzt auf 17:30–17:35 UTC:

```text
2026-10-18T17:32:32.162690Z WARN
slug=rp-ludwigshafen-mkm-metals
record failed: response from https://mkm-metals.de/metallpreise was not usable: incomplete or invalid MKM source contract
```

`systemctl show schrott-mcp`: **active/running**, Start `2026-10-17 16:49:25 UTC`.

Read-only Produktionsabfrage:

| Beleg | Wert |
|---|---|
| Fehlgeschlagener Step | id **9732**, run **2072**, `failed`, **0** items_upserted |
| Zeitfenster | 17:32:31.850264313–17:32:32.162726961 UTC |
| Zugehöriger Fetch | id **9728**, HTTP **200**, **256100** Bytes, content_hash `d4681600af2898dd` |
| Letzter erfolgreicher Step | id **9596**, run **2048**, 11:29:42 UTC, **55** Preise / **5** Annahmen |
| Vorheriger Fetch | HTTP 200, **253490** Bytes, hash `83ce4ec384ecffec` |
| Neuester Artikel-Ledger | id **4**, `2026-10-18T11:29:42.503024610+00:00`, owner `mkm-article-v1` |
| Erhaltener Ledger | **60** Identitäten, exakt Oracle-Identitätsmenge; keine Material-/Variantenabweichung |
| Aktuelle gleichquellige Preise | **55**; beobachtet zwischen 17.10. 17:19:54 und 18.10. 11:29:42 UTC |

Aktueller Capture und Produktionsfetch sind wegen anderer Länge **kein byteidentischer Beleg**; der Produktionshash ist auch nicht als SHA-256 ausgewiesen. Keine Gleichsetzung oder unbelegte Erklärung der Längendifferenz.

Codepfad:

- `crates/ingestion/src/traders/handlers/mkm_metals.rs:45–50`: typisierter Fetch/Parse, Diagnosen werden zur Schreibgrenze transportiert.
- `.../mkm_metals/overview.rs:868–890`: unbekannte Artikelidentität erzeugt Diagnose, kein erratenes Mapping; `:955` überträgt Diagnosen nach `skipped_labels`.
- `.../mkm_metals/recording.rs:41–58`: `record_articles` verwirft nichtleere Diagnosen/Skipped-Labels **oder** eine von `MAPPINGS.len()` abweichende Artikelanzahl mit genau der Produktionsfehlermeldung. Dies passiert **vor** Traderlookup und Store-Transaktion. Die aktuelle 61-Artikel-Quelle verletzt beide Bedingungen.

## 5. Bestehende Gates/Tests: reale Ergebnisse, nicht grün umgedeutet

```sh
python3 -B scripts/verify-mkm-release.py --since 2026-10-17T16:49:25Z
```

Exit **1**: 60 Artikel / 55 numerisch / 5 Anfragen / 55 Current, aber:

```text
manifest is not from latest completed step window
successful completed post-rollout 55-price step missing
```

Erwartbar: Neuester Step ist gescheitert und hat keinen neuen Manifeststand. **Das Produktionsgate bleibt rot.** Engere Cutoffs `2026-10-18T17:30:00Z` und `11:29:00Z` ebenfalls rot, zusätzlich Freshness-Meldungen; sie sind keine isolierten Korruptionsbelege für den zuvor gespeicherten Bestand. Es wurden weder Verifierlogik geändert noch neuester fehlgeschlagener Step weggefiltert.

```sh
python3 -B -m unittest discover -s scripts -p 'test_*mkm*.py' -v
```

**17 Tests bestanden** (9 Source-Audit, 8 MKM-Release-Verifier), ausschließlich isolierte Testdaten.

Vorhandenes Testbinary, **kein cargo/Build**, einzeln ausgeführt mit privatem TMPDIR:

```text
target/debug/deps/schrott_mcp_ingestion-1308358f6f77fcd7
  traders::handlers::mkm_metals::overview::tests::same_count_unknown_and_changed_identity_are_not_complete --exact --nocapture
  -> 1 passed, 0 failed (0,54 s)

target/debug/deps/schrott_mcp_ingestion-1308358f6f77fcd7
  traders::handlers::mkm_metals::recording::tests::incomplete_or_corrupt_typed_contract_cannot_write_or_retire --exact --nocapture
  -> 1 passed, 0 failed (3,43 s)
```

Letzterer prüft echte isolierte Recording-Aufrufe und erhaltene Preise/History bei ungültigen Verträgen. Das ist bestehende Guard-Regressionsabdeckung, **kein Replay des neuen Live-Artikels am Recording-Einstieg**. Ein anfänglicher breiter MKM-Testlauf wurde nach 30 s abgebrochen; kein Gesamterfolg behauptet. Seine ausschließlich selbst erzeugten DB-Reste wurden gezielt entfernt; die beiden oben genannten Einzeltests wurden danach separat erfolgreich ausgeführt.

Binary-Provenienz SHA-256:

- laufendes Produktionsbinary `/proc/1095156/exe`: `7de235b153c571d2ed346dbb39fe503003ac20edec0ff9fc9bf0bad85b595c34`
- vorhandenes Inventory-Binary (mtime 17.10. 16:16 UTC): `1feebc41a2ab6d768c2da9d8ab8d9eac5c968c7056e555c931411ef15ba9bc19`
- vorhandenes Rust-Testbinary: `763f12ea611e8f58954784ebad98739274d9a1f01c18abb4dddfcef3db7ee3ac`

Keine Behauptung, dass diese unterschiedlichen Programme byteidentisch oder frisch gegen den jetzigen HEAD gebaut wurden. Befund beruht auf konsistenten Live-/Capture-Replays, eng passender Codeprüfung und unabhängig beobachtetem Produktionsguard.

## Nächste sichere Maßnahme

1. **Guard unverändert lassen, kein Force-Ingest.** Owner soll die neue primäre Produktidentität fachlich prüfen und ein explizites Material-/Variantenmapping samt Oracle- und Vollständigkeitsvertragserweiterung planen. Nicht lediglich `60` auf `61` erhöhen oder neues Material blind auf `messing` abbilden.
2. In einer freigegebenen Entwicklungsumgebung mit Platz das gesicherte neue HTML durch **Parser → `record_articles` → isolierten Store** testen: zunächst exakt rot, dann nach reviewter Erweiterung vollständig (Quelle derzeit **56 numerisch / 5 Anfragen**). Atomarität, bestehende 60 Identitäten, Staffel-/Einheitenmetadaten und History müssen erhalten bleiben. Diesen fehlenden neuen End-to-end-Recording-Replay hier ausdrücklich als offen behandeln.
3. Erst nach separater Freigabe/Deployment regulären Schedulerlauf abwarten und read-only Verifier mit geprüft angepasstem Oracle/Partition und realem Deployment-Cutoff erneut ausführen. Aktuell **kein grüner Release-/Freshnessnachweis**.

Produktionsschaden begrenzt auf ausgebliebene frische vollständige MKM-Aufnahme; der zuvor gültige Bestand bleibt sichtbar. Weitere Zeit ohne Vertragsreview bedeutet weitere ausgelassene MKM-Aktualisierungen, nicht automatisches Recovery durch Neustart.

## Nachtrag: Mappingentscheidung und begrenzter Umsetzungsplan

Folgeauftrag des Owners, 18.10.2026: **nur Planung**, keine Umsetzung. Owner hat die Produktdetailseite selbst live gelesen. Zusätzlich deren bereits gesicherten Body erneut gelesen und heutigen Katalog/MKM-Current ausschließlich read-only geprüft. Die folgenden Tests und Änderungen sind **geplant, nicht ausgeführt/implementiert**.

### A. Entscheidung: `messing` + eigene stabile Variante

**Empfehlung: `MKM-MS-01 → (messing, "Messing 58 blank neu")`; kein neues Katalogmaterial.**

- `crates/ingestion/src/pipeline.rs:168–172` definiert `messing`, Name „Messing“, Kategorie `nichteisen`, Einheit `EUR/kg`, Beschreibung „Messing aus Armaturen, Schrauben und Drehspänen.“ Produktionskatalog bestätigt diese Werte (Material-id 7). Das ist eine übergreifende Messingkategorie, keine Definition ausschließlich gebrauchter oder ausschließlich einer Legierungszusammensetzung zugehöriger Ware. Die Beschreibung nennt Beispiele, nicht einen numerischen Cu-Gehalt oder ein Verbot von Fertigungsabschnitten.
- `overview.rs:218–265` und heutige MKM-Current-Daten enthalten bereits fünf getrennte Varianten unter `messing`: `SW10020 / Messing Hülsen`, `SW10019 / Messing 63 Abfälle`, `SW10018 / Messing 58 Abfälle`, `SW10021 / Messing schwer`, `SW10023 / Messing Späne`. `SW10022 / Messing leicht` ist dagegen ausdrücklich `messing-leicht`. Keine der sechs Identitäten wird ersetzt.
- Detailquelle: Automatenmessing im Neuzustand, blanke Dreh-/Frästeile, Stangen-/Profilabschnitte und Fehlteile ohne Anhaftungen; klar abgegrenzt von gebrauchten 58-Abfällen, 63-Abfällen und Spänen. Sie nennt neben CuZn39Pb3 auch CuZn39Pb2 und CuZn40Pb2. Ein neues Material nur namens `CuZn39Pb3` würde sogar enger als der angebotene Artikel sein. Nicht „neues Metall“, sondern eigene handelbare Sorte innerhalb Messing.
- Diese Unterschiede **müssen** im Variantenschlüssel und vollständigen Artikel-/Staffelmetadaten erhalten bleiben. Gemeinsam unter `messing` heißt nicht gleicher Preis oder Austauschbarkeit. Weder Variante `""` noch `"Messing 58 Abfälle"`, `"Messing schwer"` oder `"Messing Späne"` ist zulässig. Kein globales Mapping oder Katalog-/Seed-/Dossier-Update nötig. Neues Katalogmaterial erst bei einer separat begründeten systemweiten Taxonomieentscheidung, nicht zur Behebung dieses einzelnen Warns.

Exakter neuer Mappingeintrag in der bestehenden Tuple-Struktur (Plan):

```text
("MKM-MS-01", "Messing 58 blank neu", "messing-58-blank-neu",
 "messing", Some("Messing 58 blank neu"), 1)
```

Die Artikelnummer ist die stabile Quellidentität; Label, Detailpfad und Gruppe bleiben gemeinsam streng validiert. Der explizite Variantenwert entspricht dem Betreiberlabel und ist ein persistenter Schlüssel, kein aus Preis, Datum oder Legierungsanalyse generierter Text. Ein späteres Rebranding muss wieder reviewt werden, nicht still einen zweiten Schlüssel erzeugen. Store-Current wird über Trader/Material/Variante adressiert, die Artikelnummer zusätzlich im Manifest und `extra_json.article.article` bewahrt. Owner `mkm-article-v1` und Quell-URL bleiben unverändert: kein neues Speicherformat, kein Wechsel des Produzenten und keine Aliasmigration. Neues Messingmaterial oder ein Owner-Wechsel würden unnötige Acceptance-/Historymigrationen verursachen.

Sollwert dieser **konkreten** Quelle: 61 Identitäten, 56 numerische Angebote, 5 Anfragen, 8 Gruppen. Neuer Preis `6.51 EUR/kg`, `upto`, `price_max=6.51`, `price_min=NULL`, Konfidenz `0.5`; Staffeln `(0,6.20),(5,6.32),(500,6.51)`, obere Schwelle **500 kg**. Kein universeller Kleinmengenpreis von 6,51 und kein erfundenes Veröffentlichungsdatum. Die bestehende Messing-Acceptance enthält anschließend **sechs** Artikel statt fünf; `RecordSummary.accepted=5` zählt dagegen weiterhin die fünf Anfrageartikel aller Materialien, nicht Messingsorten oder Acceptance-Gruppen.

### B. Vollständigkeitsannahmen: konkrete Änderungsstellen

**Wichtiger Zusatz gegenüber der Erstdiagnose:** Der Store hat einen eigenen festen 60er-Guard. Nur das Parsermapping zu erweitern reicht nicht; spätestens die Store-Transaktion würde weiter scheitern. Kein globales Suchen/Ersetzen von `60`, `55` oder `5`.

| Stelle | Heute / eng begrenzte geplante Anpassung |
|---|---|
| `mkm_metals/overview.rs:89ff,868–955` | Aktives `MAPPINGS` um exakt den reviewten Eintrag ergänzen; exakte Artikelmenge, Label/Pfad/Gruppe, Doppelschlüssel-, Missing- und Unknown-Prüfungen erhalten. Fehlendes MS-01 muss künftig rot sein. Reporttext `:55` nennt dann 61 statt 60; keine inhaltliche Freigabe durch `production_ready`-Flag. |
| `mkm_metals/recording.rs:47–57,75–190` | Kardinalität ist bereits `MAPPINGS.len()`, Partition bereits `seen.len()-request_count`; **keine** fest kodierte 56/5-Produktionsbedingung hinzufügen. Neue Identität durch dieselbe vollständige Validierung und Gruppierung führen. `PROVEN_LEGACY_ARTICLES` mit zehn historischen Kupferidentitäten **unverändert**. |
| `crates/store/src/public/prices.rs:611–633` | MKM-spezifische Manifestkardinalität `a.len()!=60` bewusst auf aktuell reviewte **61** umstellen (benannter lokaler Vertragswert statt breit geteilter freier Größenparameter). Scope, Timestamp, Eindeutigkeit, Besitz-/Konflikt-/Rollbackprüfungen erhalten. Kein `>=60` oder `60\|61`: alte unvollständige Quelle darf neue Current-Pointer nicht still retire'n. Exakte semantische Artikelmenge prüft weiterhin der Handler; Store ist hier struktureller zweiter Guard, kein unabhängiger Mappingoracle. |
| Oracle | Alten `mkm-reviewed-mapping-oracle-20261011.json` **unverändert** als 60er-Evidenz bewahren. Neuen datierten `mkm-reviewed-mapping-oracle-20261018.json` mit genau derselben alten 60er-Menge plus MS-01 und Reviewgrund anlegen; Identitätsmengendifferenz muss exakt `{MKM-MS-01}` sein, alle alten Material-/Variantenpaare identisch. Kein Oracle automatisch aus `MAPPINGS` generieren. |
| `scripts/verify-mkm-release.py:16,45,68–70,138–139` | Aktiven Oracle auf neuen datierten Stand stellen; Step-Erwartung 56 statt 55, exakte neue Menge 61 und Quellpartition 56/5 samt Fehlermeldungen prüfen. Request/Quote-/Current-/Acceptance-/Einheiten-/Freshnessprüfungen unverändert. Keine Anzahl aus beliebigem beobachtetem Manifest ableiten, keine frei wählbare „erwartete Anzahl“ als Grün-Schalter. |
| `scripts/audit-mkm-overview.py:18–19` | Default des **aktuellen** Audits auf neuen Oracle; Zähllogik ist bereits dynamisch. Historische Aufrufer/Tests übergeben explizit alten Oracle. |
| `overview.rs` Tests `:968–970,1024–1027,1135–1244,1305–1343,1393–1429` | Alte Counts/Datums-/Identitätsassertions an historische Quelle gebunden lassen; aktueller vollständiger Positivtest auf neuem Capture 61/56/5. Missing-/Same-count-/Visibility-/Decoy-Tests müssen weiterhin spezifische Mutation treffen, nicht nur am zusätzlich fehlenden MS-01 scheitern. |
| `recording.rs` Tests `:328ff,472,609–614,1025` | Positive Recording-Fixture muss vollständigen aktuellen Vertrag erfüllen. 55 Current wird gezielt 56; vollständiger All-request-Fall gezielt 61 statt 60. Historische Replays `12` Altpreise, `5` alte Acceptances und zehn proven Legacy-Artikel bleiben historische Zahlen. Keine Änderung an 0.5-Konfidenz, 60%-Kupferkabel/Zinn oder Produktstaffeln. |
| `mkm_metals.rs:421–430` | Als „current“ bezeichneter Test darf nicht mehr alten 60er-Body mit leeren Diagnosen erwarten; auf echten 61er-Capture wechseln. Historische test-only Copper-/Mappinghelpers nicht global erweitern. |
| Store-Tests `prices.rs:1269–1328` | Synthetisches `(0..60)`-Manifest gezielt auf zulässige aktuelle Struktur bringen. **Wichtig:** Triggerfehler nachweislich erreichen; sonst würde ein Test wegen falscher Anzahl vor der Transaktion rot und täuschte Rollbackabdeckung vor. |
| `scripts/test_verify_mkm_release.py:51,82,98` | Bisherige synthetische Buchhaltung um neue aktuelle Artikelprojektion erweitern, Step 56/Artikel 61; die alte 60er-Inventardatei nicht überschreiben. Ergänzend alter 60er-Zustand als bewusst rotes aktuelles Gate, nicht als weiterhin aktuelle Positivfixture. |
| `scripts/test_audit_mkm_overview.py:26–45` | Historische Vorher-/Nachher-Tests explizit mit altem Oracle; **120** gezielte Material-/Variantenfehler für die historischen 60 Artikel bleiben 120. Separater neuer 61er-Test; nicht 120 blind auf 122 ändern. |

Datierte Forschungsinventare/alte Vollaudits bleiben Belege ihres damaligen Vertrags und werden nicht umgeschrieben. `examples/mkm_inventory.rs` selbst hat keine feste 60/55/5-Prüfung; seine Defaultfixture bleibt historisch, vollständiger aktueller Replay erfolgt mit expliziten Capturepfaden. Den externen `/root/mkm-owner-unit-review/.../rollout_fixture.rs` nicht als grüne aktuelle Evidenz weiterverwenden: fest verdrahtete alte HTMLs, 55/5 und alte Cutoffzeit. Keine neuen allgemeinen Versions-/Konfigurationsmodule für diese einzelne Änderung.

### C. Altes Fixture bleibt Beleg, nicht Produktions-Ausnahmeregel

`mkm-overview-20261010.html` und alter Oracle bleiben byteidentisch. Unter erweitertem aktiven Mapping kann dieses HTML weiterhin **60 Artikel / 55 Preisprojektionen / 5 Anfragen** und sämtliche alten Mappings/Staffeln belegen; zusätzlich erwartet der Test jetzt **genau** `MKM-MS-01: missing known article; retirement NOT authorized`. An der aktuellen Recording-Schnittstelle muss es scheitern, ohne DB-Effekt. Ein historisch gültiger Vertrag ist nicht automatisch eine heute vollständige Quelle.

Für bestehende Positiv-/Mutationsregressionen, die alte Datums-/Preisbesonderheiten benötigen, ein gemeinsamer **test-only** Fixturehelper: aus altem unverändertem HTML plus exakt einer unveränderten, reviewten MS-01-Zeile des neuen Captures eine als synthetisch beschriftete 61er-Kombination erzeugen (Assertion: alter Input enthält MS-01 nicht; Ausgabe einmal). So bleiben z.B. 09.10.-Pagedatum, alte Kupferstaffeln und vorhandene Mutationstreffer erhalten. Diese Kombination ist **kein** Ersatz für den separaten echten 18.10.-Capture-End-to-end-Test und keine Live-Evidenz. Kein test-only `MAPPINGS`-Override, kein produktiver „historical mode“ und kein Entfernen der neuen Missing-Diagnose, nur um alte Tests grün zu halten.

### D. Exakter Red→Green- und Sicherheitsplan

1. **Vor Änderungen, echter neuer Regressionstest:** gesicherten 61er-Body samt Impressum in einer freigegebenen Entwicklungsumgebung als datierte Fixtures verfügbar machen (vorher SHA-256 des Originalcaptures prüfen; Cookies/CSRF-/Sessiondaten nicht ungeprüft committen, Body ggf. gezielt und dokumentiert sanitizen). In `recording.rs` neuen Async-Test `reviewed_20261018_capture_records_distinct_ms58_variant` über vorhandenes `setup`/`commit` schreiben: **echtes `overview::parse` → `RecordOutcome::MkmArticles` → `record_outcome` → `record_articles` → echter isolierter PublicDb/Store**. Keine vorgefertigte Snapshot-Struktur als Ersatz. Erwarteter Erfolg `(56,5)` muss vor Mappingpatch **rot** mit dem bekannten `incomplete or invalid MKM source contract` werden. Startet der Test nicht oder scheitert er nur an Compiler/Platz/Setup, ist dies **kein** reproduzierter fachlicher Red.
2. **Minimalpatch in zwei diagnostischen Stufen:** erst exakt neues Mapping + Oracle/aktuelle Testprojektion; der End-to-end-Test muss nun den noch bestehenden Store-60er-Guard erreichen und dort scheitern. Danach eng benannten Storevertrag auf 61 anheben und aktuelle Verifiererwartungen ergänzen. Erst dann derselbe originale Capture-Test grün. Keine Lockerung einer anderen Schutzprüfung zur Umgehung dieser Stufen.
3. **Grün ist mehr als Anzahl:** Manifest genau neue Oracle-Menge, alle 60 alten Material-/Variantenpaare unverändert, MS-01 genau einmal. Current `(messing,"Messing 58 blank neu")`, Quelle/Owner/Artikelmetadaten, drei Staffeln, 500-kg-Schwelle und Preissemantik exakt prüfen. Alte fünf `messing`-Varianten bleiben getrennt vorhanden; Messing-Acceptance enthält sortiert genau sechs semantische Artikel. Kein neues Material, keine erfundene Validität/Publikation, kein globaler Alias.
4. **Unbekannte 62 bleibt rot:** echten vollständigen 61er-Body um eine strukturell gültige numerische Zeile mit neuer Nummer `MKM-MS-02`, eigenem Label/Pfad und gültigen Staffeln ergänzen. Parser muss spezifisch `unknown article` melden, Recording dieselbe Schutzmeldung; jeweils davor/danach sortierte Zeilen aller `prices`, `current_prices`, `source_article_snapshots`, `trader_materials`, `article_acceptance_owners` vergleichen: **kein Unterschied**. Zusätzlich MS-01 gegen unbekannte Nummer austauschen (weiterhin 61 Zeilen): Unknown **und** Missing müssen greifen; weder Anzahl noch Wort „Messing“ darf freigeben. Fehlende/duplizierte MS-01, geänderter Pfad/Gruppe/Label und falsche Staffel ebenfalls rote Kontrollen.
5. **Historischer 60→61-Persistenzübergang:** isolierten 60er-Vorbestand mit unverändertem alten Manifest und seinen alten Besitz-/Current-/Preiszeilen als eng begrenzte **test-only** Seed-Evidenz laden (logische Testdaten, keine Produktions-DB-Kopie; erforderlichenfalls einmal mit dem unveränderten Vor-Patch-Harness in der Entwicklungsumgebung erzeugen). Nicht über eine produktive 60/61-Ausnahmeregel „seeden“. Danach neuen echten Capture durch gleiche Recording-Schnittstelle führen: alter 60er-Ledger bleibt byteidentisch, neuer 61er-Ledger wird angehängt; Messing-Acceptance bekommt einen zusätzlichen Artikel, keine Änderung fremder Besitzverhältnisse. Alte History-Zeilen inkl. Metadaten bleiben unverändert; neue Beobachtungen dürfen angehängt werden, auch für alte Artikel bei geänderten Datums-/Preisevidenzen. **Nicht** pauschal nur eine neue Preiszeile verlangen.
6. **Retire/History/Idempotenz:** vollständiges neues Snapshot an gleicher Zeit wiederholen: keine Duplikate/Mutation. Später gleiche semantische Evidenz: nach vorhandener Dedupe-Semantik prüfen, nicht neues Current-Beobachtungsdatum erzwingen. MS-01 in einer weiterhin vollständigen 61er-Quelle auf `auf Anfrage` umstellen (Preiszelle und Staffeln konsistent): `(55,6)`, genau sein eigener numerischer Current-Pointer weg, History erhalten, seine Acceptance bleibt; zurück numerisch `(56,5)`. Unvollständige 60er-Quelle danach darf gerade **nicht** als autorisierte Retirementquelle wirken. Zusätzlich gleichzeitige foreign-source/-producer-/newer-Current- und Acceptance-Konflikte vorbesetzen: Gesamttransaktion muss unverändert scheitern bzw. fremde unabhängige Schlüssel unangetastet lassen.
7. **Rollback wirklich erreichen:** am 61er-Storepfad Failpoints/isolierte SQLite-Trigger nacheinander bei Retirement, Acceptance und Manifest einrichten; konkrete Trigger-Fehlermeldung prüfen und alle fünf Evidenztabellen vorher/nachher vergleichen. Zusätzlich vorhandene Alias-/Foreign-/History-/Unit-/All-request-Tests auf vollständiger neuer Positivfixture erneut ausführen. Kein partieller neuer Messingpreis, kein verwaister Ledger/Owner und keine rückwirkend bearbeitete alte History bei Fehlern.
8. **Verifier getrennt abnehmen:** neuer echter Parser-/Store-Fixturebestand plus explizit synthetische Step-/Fetch-Buchhaltung kann den aktualisierten read-only Verifier grün machen, bleibt aber **isolierte**, keine Produktionsfreigabe. Bestehende Negativmutationen erhalten; neuer unbekannter Artikel, alter 60er-Manifeststand, fehlender MS-01-Current, fälschlich wiederverwendete Altvariante und Acceptance mit nur fünf Messingartikeln müssen rot sein. Historische 60er-Evidenz bleibt überprüfbar, aber darf nicht das aktuelle Gate bestehen. Bytegleichheits-/Schreibverbotstest des Verifiers beibehalten.

### E. Reproduktions-/Buildplatzgate und Übergabe

Bei diesem Folgeauftrag gemessen: **root 559 MiB frei, `/tmp` 94 MiB**. Kein Build gestartet, keine Tests neu kompiliert, keine Produktionsschreiboperation und keine neuen Capture-/DB-Exporte. Das vorhandene Inventory-Binary kann den bisherigen Quell-Red wiederholen, aber weder geänderten Rustcode noch einen neu geschriebenen End-to-end-Test ausführen. Vorhandenes Testbinary enthält diesen neuen Test nicht. **Geplanter Red/Green-Recording-Nachweis bleibt daher hier blockiert/offen**, nicht als grün anzurechnen. Das originale 17:32-Produktions-HTML bleibt weiterhin nicht verfügbar; neuer Regressionstest reproduziert den gesicherten aktuellen Quellfehler, nicht bytegenau den historischen Abruf.

Buildfreigabe erst nach eigenem Gate: zugelassene Entwicklungsmaschine/-Volume mit anhand der vorhandenen Artefaktgrößen und erwarteten inkrementellen Rust-/Link-/Tmp-/Test-DB-Spitzen **nachgewiesen ausreichender Reserve**, explizites Target-/TMPDIR und Abbruchgrenze. Keine Reinigung fremder Artefakte, kein Build auf dieser fast vollen Produktionspartition, kein Vertrauen auf „es ist nur ein kleiner Patch“. Auf geeigneter Umgebung zuerst den einzelnen neuen End-to-end-Test, dann eng gefilterte MKM-/betroffene Storetests und Python-MKM-Tests; Gesamtlauf mit eigener Platz-/Laufzeitfreigabe. Erst separater Ownerentscheid erlaubt Implementierung, anschließend gegebenenfalls Deployment und regulären Schedulerlauf. Dieser Nachtrag erteilt keine solche Ausführungsfreigabe.
