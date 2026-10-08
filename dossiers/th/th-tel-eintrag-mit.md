---
slug: th-tel-eintrag-mit
name: Eintrag mit
trader_type: sonstige
state: TH
city: unbekannt
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (2026-09-27)
provenance_ankauf_raw: unklar (Register-Prosa)
provenance_origin: prose
---

# Eintrag mit

## Überblick

Das Seed-Fragment ist keinem Betrieb zuzuordnen: „Eintrag mit“ ist kein identifizierbarer Firmenname, `Tel.` wirkt wie ein verschobenes Feldlabel, und der Seed-Wert `03641` stimmt mit der Telefonvorwahl von Jena überein (kein Beleg für eine Firmenanschrift). Stadt und PLZ daher aus dem Frontmatter entfernt, nicht durch Jena ersetzt. Kein Betreiber-/Registertreffer oder belastbarer Hinweis auf Schrottankauf, Annahme oder Entsorgungsbetrieb; Herkunft des Fragments bleibt offen. Keine Preisangaben oder Leistung belegt.

## Timeline

### Provenienzprüfung 08.10.2026

- [Recherche 08.10.2026: Repo-Gegenprüfung statt weiterer Suche nach einem künstlichen Firmennamen: Die exakte Fragmentkombination ist nur in diesem Dossier erhalten; das in der Importhistorie vorausgesetzte ursprüngliche Register-/Tabellensubjekt ist hier nicht rekonstruierbar. Dieses Ergebnis gilt für den geprüften Markdown-Korpus, nicht für alle früheren Datenstände. `provenance_origin: prose` und `name: Eintrag mit` bleiben aus Gründen der Slug-/Historienstabilität erhalten, sind aber kein Betriebsbeleg; city unbekannt ist ausschließlich Schema-Platzhalter. Nicht mit dem Jenaer Heiko-Schmidt-Dossier zusammenführen, nur weil Vorwahl 03641 passt. ANKAUF/VERKAUF/GEBÜHREN: mangels identifizierter Firma nicht zuordenbar, nicht als preislose reale Annahmestelle darstellen. Offener Primärschritt: Originalimport/Registernummer beschaffen. Quelle: dieses Dossier, Importiert (Seed-Stand 2026-09-30), repo-interne Textsuche 08.10.2026; Validierungsbezug crates/ingestion/src/seed_traders.rs .]

### Importiert (Seed-Stand 2026-09-30)

- Registerfund ohne geprüfte Website (PLZ 03641)

### Quellen-/Feldprüfung 07.10.2026

- [Quellen-/Feldprüfung 07.10.2026: Die Suche nach der exakten Kombination „Eintrag mit“ + „03641“ + Schrott/Jena lieferte keine identifizierbare Firma. Das Telefonbuch ordnet 03641 ausdrücklich der Telefonvorwahl Jena zu; daraus folgt aber weder, dass das Fragment einen Jenaer Schrotthändler bezeichnet, noch ein tatsächlicher Betriebsstandort. Die fehlerverdächtigen Werte `city: Tel.` und `postcode: 03641` deshalb geleert statt als Anschrift übernommen oder eine Firma zu erraten. Offen bleibt die Herkunft des Register-Prosa-Schnipsels und sein eigentliches Subjekt. Keine bestätigte Tätigkeit, Ankauf-/Verkaufspreise oder Gebühren. Quelle(n): https://www.dastelefonbuch.de/Vorwahlen/03641--Jena ; https://html.duckduckgo.com/html/?q=%22Eintrag+mit%22+%2203641%22+Schrott+Jena]

- [Owner-Korrektur 07.10.2026: `city` muss laut `seed_traders::validate_seeds` nichtleer sein. Da keine Stadt identifiziert ist, steht dort jetzt `unbekannt` als Schema-Platzhalter, nicht als geografische Behauptung; `postcode` bleibt leer. Quelle: `crates/ingestion/src/seed_traders.rs` (Validierung verlangt nichtleere `city`).]
