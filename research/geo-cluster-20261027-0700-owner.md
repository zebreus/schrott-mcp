# Owner-Abschluss: Essener GEO-Cluster, 27.10.2026

## Ergebnis

Die read-only-Abfrage von `public.db` bestätigt **31** unterschiedliche Händlerzeilen mit demselben Punkt `51.450284375, 7.015053125`. Nominatim-Reverse ordnet den Punkt der Freiheit im Essener Südviertel (45128) zu. Das ist ein starkes Cluster-Signal, aber kein händlerspezifischer Beweis. Die drei disjunkten Shards haben jeden Händler anhand seiner Dossierhistorie und verfügbarer Betreiber-/Behördenbelege einzeln geprüft.

| Entscheidung | Anzahl | Wirkung |
| --- | ---: | --- |
| Standortabweichung bestätigt | 12 | Im jeweiligen Dossier `clear_fields: 'geo'` und datierte Timeline-Korrektur; keine Ersatzkoordinate |
| Zuordnung/Standort unzureichend belegt | 19 | Keine Dossieränderung; bleibt offen |
| **Gesamt geprüft** | **31** | Kein direkter Produktionszugriff außer lesender Abfrage |

## Bestätigt und für den nächsten regulären Seed korrigiert

| Händler | Begründung für Rücknahme des alten Paars | Shard |
| --- | --- | --- |
| Brauckmann | Betreiber und Metallannahme an Krablerstr. 14b, 45326 | [A](geo-cluster-20261027-0700-a.md) |
| C.M.R. Recycling Vertriebs | Betreiber/Schrottplatz Ripshorster Str. 386, 45357 | [A](geo-cluster-20261027-0700-a.md) |
| Carl Herholz | Behördlich dokumentierter Anlagenstandort Wilhelm-Beckmann-Str. 14, 45307; der Beleg bestätigt keine öffentliche Annahme | [A](geo-cluster-20261027-0700-a.md) |
| IST GmbH | Betreiber- und Behördenbelege an Stauderstr. 83–85, 45326, mit Annahmezeiten | [A](geo-cluster-20261027-0700-a.md) |
| Metallion | Betreiberbelege für private/gewerbliche Metallannahme an Heßlerstr. 77, 45329 | [B](geo-cluster-20261027-0700-b.md) |
| P & W Metall | Betreiber-/Registeranschrift Ostuferstr. 8, 45356 und öffentlich zugängliche Waage am Essener Standort; unabhängiger Adressabgleich | [B](geo-cluster-20261027-0700-b.md) |
| Peter Häuser Rohstoffhandel | Betreiber nennt ausdrücklich das Ankauf-Betriebsgelände Ripshorsterstr. 370, 45357 | [B](geo-cluster-20261027-0700-b.md) |
| Ingrid Sackenreuther | Betreiber-/Adressbelege Brüninghofer Weg 17, 45307; Betreiber schließt Annahmestelle und Anlieferung dort ausdrücklich aus | [B](geo-cluster-20261027-0700-b.md) |
| Jürgen Missal | Betreiber- und unabhängiger Adressbeleg Kalthofweg 4, 45277; keine öffentliche Hofannahme behauptet | [B](geo-cluster-20261027-0700-b.md) |
| Schrottplatz Prison (Claudia Prison) | Betreiberstandort Hövelstr. 254, 45356 | [C](geo-cluster-20261027-0700-c.md) |
| TRIMET Aluminium SE | Betreiberstandort Aluminiumallee 1, 45356; B2B-Rücknahme, kein öffentlicher Privatkundeneingang behauptet | [C](geo-cluster-20261027-0700-c.md) |
| WRL GmbH | Betreiberstandort Emscherstr. 27, 45327; Betreiber bestätigt Materialabgabe | [C](geo-cluster-20261027-0700-c.md) |

Der Owner hat die Diffs und die in den Shards dokumentierten Fallbelege geprüft und aktuelle Betreiber-/Behördenquellen stichprobenartig direkt abgerufen. Dabei wurden insbesondere Metallhandel/Anlieferung bei Brauckmann, Betreiber-/Standortangaben bei C.M.R., IST, Prison, TRIMET und WRL sowie die fünf B-Fälle gegengeprüft. Der gemeinsame Produktionspunkt ist jeweils laut der read-only-Abfrage weiterhin der alte Wert; die Seed-Clears sind noch nicht ausgerollt.

## Offen, unverändert

- **Shard A (7):** Bogner, CBR Bracke, EIE, Häde (Essener Seed-Identität/Betreiberwechsel zu Metallion nicht belegt), Köllner, L.G.R., Meinrich.
- **Shard B (5):** MRE, Rainer Ludwig, Schrott Busters, Schrott Nowak, Missal/Metzler.
- **Shard C (7):** Schüttler, Schwarzaugen, Seliger, THP, ZJM, Traut, Schrottladys.

Details zu Namens-/Rechtsträgerabgleich, Quellen und offenen Fragen stehen in den [Shard-Berichten A](geo-cluster-20261027-0700-a.md), [B](geo-cluster-20261027-0700-b.md) und [C](geo-cluster-20261027-0700-c.md). „Offen“ bedeutet nicht, dass der Essener Mittelpunkt als Händlerstandort bestätigt wäre.

## Sicherheits- und Rolloutstatus

- Alle 12 Korrekturen setzen nur `clear_fields: 'geo'` im Dossier; nach [`docs/trader-seed.md`](../docs/trader-seed.md#explicit-enrichment-clears-opt-in) entfernt der nächste reguläre Seed das bestehende Paar. Bis dahin ist die Produktion unverändert.
- Kein direkter Produktions-DB-Schreibzugriff, kein Deployment und kein erzwungener Ingest.
- `cargo test -p schrott-mcp-ingestion seed`: 14/14 bestanden. `cargo fmt --all -- --check` und `git diff --check`: bestanden.
- Die Post-Rollout-Prüfung für ausstehende Korrekturen bleibt wegen des fehlenden erfolgreichen Gutzmann-Schritts rot. Der historische Wave-E-Post-Rollout-Verifier meldet weiterhin Altmanifest-Abweichungen. Kein grünes Pre-Deploy-Gate liegt vor; daher keine Produktionsänderung.
