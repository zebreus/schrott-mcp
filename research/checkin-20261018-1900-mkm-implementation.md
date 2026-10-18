# MKM Phase 1: Capturefixture und Recording-Regressionsquellcode

## Status / Umfang

**Quellcode für den vor-Fix-Regressionsschritt bereit; noch kein ausgeführter Red-Nachweis.** Fachliche Ownerfreigabe: `MKM-MS-01 → messing / Messing 58 blank neu`. Keine Mapping-, Store-Guard-, Oracle-, Verifier- oder Dossieränderung. Kein Build, keine Testausführung, kein Commit/Push/Deployment, kein Netzwerkabruf und kein Zugriff auf Produktions-DBs in dieser Phase. Bestehende Änderungen nach der Unterbrechung übernommen und eng geprüft, nicht neu begonnen.

Alle Repositoryänderungen mit dem Patchtool. Lieferumfang genau:

1. `crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261018-sanitized.html` — 21.293 Bytes, datierter minimierter HTML-Capture mit sämtlichen 61 Quellzeilen.
2. `crates/ingestion/src/traders/handlers/fixtures/mkm-impressum-20261018-sanitized.html` — 636 Bytes, zugehörige Betreiberidentität/Kontakt/Register.
3. `crates/ingestion/src/traders/handlers/mkm_metals/recording.rs` — ausschließlich ein zusätzlicher `#[tokio::test]` im vorhandenen Testmodul (103 Zeilen). Produktivfunktionen und vorhandene Tests unverändert.
4. Dieser Bericht. Der frühere Diagnose-/Planbericht bleibt unverändert.

Paralleländerung erkannt und bewahrt: `crates/store/src/public/prices.rs` enthält inzwischen den disjunkten neuen Goldtrans-Startup-Regressionsquellcode eines anderen Auftrags; dessen Diff ist kein MKM-Storepatch und wurde hier nicht bearbeitet. Insbesondere bleibt der MKM-60er-Manifestguard unverändert.

## Captureherkunft und Sanitizing

Quelle: bereits gesicherte GET-Bodies aus `/tmp/opencode/mkm-checkin-20261018-1800/`; URLs `https://mkm-metals.de/metallpreise` und `https://mkm-metals.de/impressum`, damals HTTP 200. Kein neuer Liveabruf. Identität und Staffeln des neuen Artikels wurden vom Owner unabhängig an der Betreiberdetailseite bestätigt. Nicht als bytegenauer Body des historischen 17:32-Produktionsabrufs ausgeben.

| Artefakt | SHA-256 |
|---|---|
| Original `overview.html`, 256.056 Bytes | `cf7fdc8a555194e363ab847e69d91c1f1bf063e5c8f81833d08dd1bbf0aad05e` |
| Original `impressum.html`, 27.826 Bytes | `3e797e66e174e331be33c50f20b2ccbc9d87f1c4634ac11994757e0ef3750616` |
| Neue `mkm-overview-20261018-sanitized.html` | `1aef5a3e780cdbcce5300b1f6409a4bd07f6b7ed419b2f0eeec2911bcd2eae75` |
| Neue `mkm-impressum-20261018-sanitized.html` | `8cb8d1d1d11243be89ab0ef3584d9cddd820219bccf75397b9b5f4f1ac7ca33c` |

Die Fixture ist eine **aus dem echten Roh-HTML extrahierte/minimierte HTML-Projektion**, nicht der komplette unveränderte Seitenbody und nicht eine aus Parser-Snapshots oder dem alten 60er-Fixture synthetisierte Tabelle:

- Alle acht Ankaufstabellen und alle 61 Produktzeilen aus dem Original in Originalreihenfolge übernommen: 56 numerische Quellangebote und fünf `auf Anfrage`.
- Erhalten: Gruppencaptions, Artikelnummern, exakte Labels und Detailpfade, vollständige sichtbare Preiszellentexte einschließlich Mengen, `strong`-Preisauszeichnung, maschinenlesbare Produktdaten und relative Datumstexte, `data-price`, sämtliche `data-staffel`-Werte und Seitendatum 16.10.2026.
- Weißraum normalisiert; HTML-Entities dekodiert und für den jeweiligen Text-/Attributkontext erneut kodiert. Staffelattribute verwenden einfache äußere Anführungszeichen mit dem unveränderten dekodierten JSON-Inhalt. Keine Preise, Schwellen, Artikelidentitäten oder Datumswerte fachlich umgeschrieben. Eine beim Patchübertrag statisch bemerkte Abweichung der alten Widia-Schwelle wurde vor Übergabe wieder exakt auf den Rohcapturewert 40 kg korrigiert; finaler SHA stimmt mit dem privat aus Roh-HTML erzeugten Entwurf überein.
- Entfernt: Seitenkopf/-fuß, Navigation, Scripte/JSON-LD, CSS/SVG/Bilder, Buttons/Formaktionen, Produkt-id-/Mengenfelder, UI-Ausgabetexte, dekorative Attribute und redundante Wrapper. Nur das Staffelinput bleibt als nicht ausführbarer Datenträger. Keine Header/Cookies, Authdaten oder sonstigen Sessionwerte ins Repository übernommen; keine solchen Geheimnisfelder im erhaltenen Datensatz benötigt. Öffentliche Betreiber-/Kontaktdaten bleiben als Identitätsevidenz stehen.
- Impressum: ausschließlich die drei aus dem Capture übernommenen `h2`/`p`-Blöcke „Angaben gemäß § 5 DDG“, „Kontakt“, „Registereintrag“, mit normalisiertem Weißraum. Keine erfundene Adresse oder Registeridentität.
- Die Reduktion bewahrt den hier betroffenen Daten-/Artikelkontrakt, **nicht** sämtliche DOM-/Ancestor-/CSS-/Browser-Sichtbarkeitseigenschaften der Originalseite. Sie ist keine neue Sichtbarkeits- oder umfassende Layoutregression. Das private unveränderte Original bleibt für einen später separat freigegebenen Vollbody-Replay erhalten.

Sanitizing erfolgte durch kleine private Extraktion aus Roh-HTML, ohne Anwendungscode oder Tests auszuführen; endgültige Repositorydateien wurden per Patchtool angelegt. Private Entwürfe liegen nur im bestehenden privaten Captureverzeichnis.

## Neuer Test: genaue Schnittstelle und erwartetes Red

Datei: `crates/ingestion/src/traders/handlers/mkm_metals/recording.rs`.

Vollqualifizierter Name:

```text
traders::handlers::mkm_metals::recording::tests::reviewed_20261018_capture_records_distinct_ms58_variant
```

Der Test verwendet bestehendes `setup` für frisch gesäte **isolierte** PublicDb/InternalDb und festes Beobachtungsdatum 18.10.2026. Kein Produktionspfad, keine Fixture-DB-Kopie. Tatsächliche Aufrufkette im Testquellcode:

```text
include_str!(beide neue datierte HTML-Fixtures)
→ overview::parse(..., 200)
→ RecordOutcome::MkmArticles(snapshot)
→ crate::traders::record_outcome(...)
→ vorhandenes record_articles
→ vorhandener atomarer Storepfad (erst nach späterer Vertragsfreigabe erreichbar)
```

Vor dem Recordingcall werden nur Captureumfang (61 Artikel/fünf Anfragen) geprüft. **Keine** vorgezogene Assertion auf leere Diagnosen oder neues Materialmapping; kein manuell gebauter Snapshot, kein Mapping-Override, kein Mock und kein `#[ignore]`/`#[should_panic]`. So muss der Owner den echten Recording-Guard beobachten können, statt schon an einer behaupteten Parserfreigabe zu scheitern.

Der Test erwartet bewusst **Erfolg** des vollständigen neuen Vertrags. Mit unverändertem Code ist bei der `.expect("reviewed 61-article capture must record through the real MKM Store transaction")` nach Codeprüfung der Fehler `incomplete or invalid MKM source contract` zu erwarten. **Das wurde hier nicht ausgeführt.** Ein Compiler-, Setup- oder vorgelagerter Parsefehler zählt nicht als bestätigtes fachliches Red. Nach einem später gezielten Mappingpatch bleibt der separate feste Store-60er-Guard bestehen; auch dessen notwendige Anpassung ist hier ausdrücklich nicht vorweggenommen.

Nach einem später separat freigegebenen Fix prüft derselbe Test:

- Summary 56 Preise/fünf Anfragen, 56 gleichquellige Currentpreise.
- Neuer eigener Schlüssel `messing / Messing 58 blank neu`; 6,51 EUR/kg als `upto`, obere Preisgrenze 6,51, keine untere, Konfidenz 0,5, keine erfundene Publikation/Validität.
- Quell-/Owner-/Artikelmetadaten, Detailpfad, Produktdatum 18.10. und vollständige Staffeln `(0,6.20),(5,6.32),(500,6.51)` samt 500-kg-Schwelle.
- Alle fünf bisherigen Messingvarianten behalten eigene Currentzeilen; Messing-Acceptance umfasst sechs Artikel, MS-01 genau einmal und mit identischer semantischer Artikelmetadatenprojektion.
- Genau ein 61-Artikel-Ledger; jede der 60 alten Identitäten genau einmal mit unverändertem Material-/Variantenpaar aus dem **alten unabhängigen Oracle**, zusätzlich MS-01 genau einmal.

Dieser einzelne Tracer ist noch **nicht** die Umsetzung aller geplanten Unknown-62-, History-/Retirement-/Rollback-/60→61-Übergangsregressionen; jene bleiben Phase 2 und werden nicht als bereits abgedeckt behauptet.

## Alte Evidenz bytegleich / tatsächlich vorgenommene Prüfung

SHA-256 jeweils vor und nach Änderung identisch:

- `crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261010.html`: `dd69560a9f34b226995869ca47eca09d7ffb9d3aa00383d12d337a6ea9c7de1d`
- `crates/ingestion/src/traders/handlers/fixtures/mkm-reviewed-mapping-oracle-20261011.json`: `bf44a6fefef1dd9a7c30d9bd0e98358356d156b69370756a7b5ce8703c3507e5`

Durchgeführt ausschließlich: Quell-/Difflektüre, Captureextraktion/-review, SHA-256-/Größenkontrolle und `git diff --check` auf der geänderten Rustdatei (ohne Befund). Das ist **keine** Compiler-, Parser-, Recording- oder Testverifikation.

Nicht ausgeführt: `cargo build`, `cargo check`, `cargo test`, bestehende Rustbinaries, Inventory-/Verifier-/Python-Tests und neuer Capture-Replay. Budgetfreigabe bleibt Voraussetzung; selbst zwischenzeitlich angezeigte 2,1 GiB root/92 MiB tmp gelten nicht als Ownerfreigabe oder nachgewiesener Buildspitzenpuffer.

## Übergabe an Owner

Erst **nach separater Budgetfreigabe**, geeignetem Target/TMPDIR und überprüfter Reserve den neuen Test allein kompilieren/ausführen, beispielsweise:

```sh
cargo test -p schrott-mcp-ingestion --lib \
  traders::handlers::mkm_metals::recording::tests::reviewed_20261018_capture_records_distinct_ms58_variant \
  -- --exact --nocapture
```

Dieser Befehl wurde **nicht** ausgeführt und enthält bewusst keine ungeprüften Pfad-/Budgetvorgaben. Vor Phase 2 tatsächliche Ausgabe und roten Fehlerort sichern. Erst danach gezielte weitere Beauftragung für Mapping/Storevertrag/Oracle/Verifier und restliche Sicherheitsregressionen. Lieferstatus: **Regressionquellcode bereit, Build/Red noch unbestätigt, Produktionsproblem unverändert.**
