# Owner-Disposition der Upgrade-Restwirkungen

16.10.2026. `research/checkin-20261016-0900-upgrade-impact-review.md` vollständig gelesen. Die folgende Entscheidung betrifft eng den gebundenen privaten Diff aus Commit53a24927, nicht beliebige zukünftige Seed-/Schemaänderungen. Gesamtdeployment bleibt bis zu den weiteren Gates gesperrt.

## Akzeptierte additive DDL

Zusätzlich zum Hofmannledger werden `source_article_snapshots` und `article_acceptance_owners` samt ihren Unique-Indizes als Bestandteil der bereits integrierten MKM-Funktion akzeptiert. Auf der privaten Kopie entstehen alle drei Tabellen leer; keine bestehende Tabellenzeile oder Schemaeintrag wird durch Open entfernt/umgeschrieben, user_version bleibt6. Spätere MKM-Nutzung und öffentliche SQL-Lesbarkeit der Quellen-/Annahmebelege gehören zum Releaseumfang. Das sind keine internen Feedback-/Nutzerdaten. Die Hofmannprobe beweist keinen regulären MKM-Produktionslauf; dieser bleibt nach Rollout separat zu verifizieren. Alte Writer sind für beide Ownershipverträge nicht rollbackkompatibel.

## Akzeptierter genauer Traderseed-Diff

Der vollständig gebundene private Trader-Diff wird akzeptiert:715 Updates/12 Inserts/0 Deletes, davon170 Zeilen mit fachlichen Skalaränderungen und545 nur notes/extra_json/updated_at. Die83 Statusänderungen und28 Straßen-/14 PLZ-Rücknahmen sind bereits einzeln geprüfter Umfang; sie werden ausdrücklich mit übernommen. Keine bestätigte Wiedereröffnung aus geschlossen→pruefung, kein Einfahrtsnachweis aus erhaltenen GEOs. Traderzeilen/IDs/first_seen bleiben nach dem ersten Seed idempotent; Preise, Current, Annahmezeilen und Internal sind durch Seed unverändert. Geänderte Händlerbeschriftungen/Filter wirken trotzdem auf öffentliche Preisviews.

Die finalen Kontaktentscheidungen gelten unverändert: AGOSI nur Zentralkontakte; Fernau/Graf/Willeführ leer+Keep, deshalb vor Deployment gegen aktuelle Drift prüfen; Bausch/SM Metals/Limekon/Brosda begrenzte explizite Clears.153 seed_file-Entfernungen sind akzeptierter Metadatenwechsel, nicht Erhalt dieser Schlüssel in der aktuellen DB. Legacybooleans sind keine Servicefacts.

Schrott24 (`nw-nrw-weit-schrott24-metaloop-online-portal`): Rücknahme der beiden früher positiven Service-JSONs auf explizites `{}` **akzeptiert**. Dies ist Set, nicht Keep oder Clear. Ungeklärte Betreiber-/Adress-/Insolvenzzuordnung trägt keine aktuelle positive Leistungszusage. Kein Schließungsurteil und keine Übertragung auf andere Schrott24-Partnerdatensätze.

## Begrenzte FK-Bestandsausnahme

Exakt24 vorbestehende `trader_materials`-Zeilen (Rowids1070–1093) ohne Elternhändler15757 werden als getrennte Datenqualitätsschuld akzeptiert, nicht als repariert oder harmlos. Alle Materialeltern existieren; kein Preis/Current gehört zu15757, keine ID-Wiedervergabe im Seed. Alle24 bleiben durch Boot/Seed/Hofmanntransition identisch, keine neue Verletzung. Direkte SQL-Abfragen können die positiven verwaisten Annahmen weiterhin sehen; Inner-Joins verbergen sie lediglich. Monitoring darf daher `integrity_check=ok`, aber **nicht vollständige FK-Integrität** behaupten.

Gesonderter offener Klärungsauftrag: Herkunft und frühere Identität von15757 anhand belastbarer Import-/Git-/Auditbelege rekonstruieren; anschließend begründete Lösch-/Zuordnungsentscheidung zur ausdrücklichen Freigabe vorlegen. Keine Identität aus Materialliste erraten, keinen historischen Händler wiederanlegen, keine Produktionszeilen in diesem Release löschen oder umhängen. Diese dokumentierte Ausnahme ermächtigt nicht zu direkter Datenreparatur.

## Noch offenes Kataloggate und Betriebsbedingungen

29 zusätzliche Materialdefinitionen sowie wiederholte Materialzeitstempel-/Sequenzwirkungen sind noch **nicht pauschal fachlich freigegeben**. Enger Auftrag an dieselbe Implementierungssitzung: alle29 Definitionen/Einheiten/Producerbelege und tatsächlichen updated_at-Verbrauch prüfen, Ergebnis `research/checkin-20261016-0900-material-catalog-gate.md`. Keine erneute Upgradeprobe oder ungezielte Recherchewelle.

Weiter erforderlich: Katalogentscheidung, erfolgreiche reguläre Remoteveröffentlichung, neuer hashgebundener Build aus sauberem Commit, aktuelle Sicherung/Driftprüfung, nachweislich exklusive neue Writer und regulärer Scheduler-/Public-Read-Nachweis inklusive MKM. Fail-safe bleibt Ingestionstopp; kein automatischer Datenrestore oder bloßer Rücktausch alter Binaries. Keine Produktionsänderung durch diese Disposition.
