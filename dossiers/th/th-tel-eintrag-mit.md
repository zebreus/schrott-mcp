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
status: geschlossen
description: 'Kein Betrieb - Importartefakt: der Seed-Name "Eintrag mit" ist kein Firmenname, "Tel." ist ein verrutschtes Feldlabel, die PLZ 03641 ist die Jenaer Vorwahl und keine Anschrift. Es existiert kein identifizierbarer Rechtsträger, keine Adresse, kein Kontakt und kein Registereintrag zu diesem Eintrag. Status "geschlossen" bedeutet hier: kein existierender Betrieb unter diesem Eintrag - nicht die Schließung einer Firma.'
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

### Recherche 28.10.2026: Abschließende Bewertung als Importartefakt

- [Recherche 28.10.2026: Der Eintrag wurde abschließend gegen das amtliche Register geprüft. Für Jena führt der Auszug vom 11.08.2026 (Fachbetrieberegister der Gemeinsamen Stelle Altfahrzeuge und Register der Entsorgungsfachbetriebe, gespiegelt über https://schrottregister.pages.dev) genau zwei Einträge: „Kommunalservice Jena (Wertstoffhof), Löbstedter Straße 56“ und „Scholz Recycling GmbH, Löbstedter Straße 113“ - kein Eintrag, kein Teilname und keine Trägerbetriebsbezeichnung, die zu „Eintrag mit“ passen könnte. Eine eigene Suche nach dem Fragment findet keinen Betrieb; das Dossier bleibt der einzige Fundort der Zeichenfolge im Korpus. **Bewertung: Das ist kein Händler.** Der Name „Eintrag mit“ ist ein fragmentiertes Feldlabel aus einer Prosa-/Register-Importzeile („Eintrag mit …“, PLZ 03641 = Jenaer Vorwahl, „Tel.“ als verrutschtes Präfix), kein Rechtsname; es gibt keine Adresse, keinen Kontakt, keine Tätigkeit und keine Preis- oder Gebührenangabe, die sich einem Betrieb zuordnen ließen. Eine Namenssuche anhand einer Vorwahl kann keinen künstlichen Firmennamen heilen, und der Jenaer Heiko-Schmidt-Datensatz (`th-jena-07751-altstoffhandel-heiko-schmidt`) wird ausdrücklich NICHT zusammengeführt. Um zu verhindern, dass der Eintrag als prüfenswerter Betrieb im Umlauf bleibt, wurde `status` von `pruefung` auf `geschlossen` gesetzt und die description ausformuliert: „geschlossen“ bedeutet hier ausdrücklich „kein existierender Betrieb unter diesem Eintrag“, nicht „eine Firma hat aufgehört“ - eine Firmenschließung wird hier nicht behauptet, weil es keine Firma gibt. Der Schema-Platzhalter `city: unbekannt` bleibt (Validierung verlangt ein nichtleeres city), `postcode` bleibt leer, es wurde keine Koordinate und kein Kontakt gesetzt. Für den Owner bleibt als offene Punkt die Herkunft des Originalimports (welche Register-/Tabellenzeile das Fragment erzeugt hat) und die Frage, ob der Eintrag aus dem öffentlichen Datenbestand entfernt werden soll - ein Löschen ist über die Seed-API nicht vorgesehen. ANKAUF/VERKAUF/GEBÜHREN: mangels identifizierter Firma nicht zuordenbar und nicht als preislose reale Annahmestelle darzustellen. Quelle(n): https://schrottregister.pages.dev/ort-jena ; https://schrottregister.pages.dev/betrieb-kommunalservice-jena-wertstoffhof-jena ; https://schrottregister.pages.dev/betrieb-scholz-recycling-gmbh-jena ; Provenienz- und Feldprüfvermerke dieses Dossiers vom 07.10.2026 und 08.10.2026.]

### Recherche 16.10.2026 (13:00-Welle C)

- [Recherche 16.10.2026: Eigenständige exakte Websuche nach „Eintrag mit“/„03641“/Schrott/Jena ohne Treffer in der benutzten Suchintegration. Das ist nur ein begrenzter Suchbefund und kein Register-Negativnachweis. Die Primärlücke bleibt das fehlende Subjekt des Prosaimports; weitere Händlersuche anhand einer Vorwahl kann keinen künstlichen Firmennamen heilen. Keine Umbenennung in einen Jenaer Betrieb, kein Merge, keine Frontmatter-Fills. ANKAUF/VERKAUF/GEBÜHREN mangels identifizierter Firma nicht zuordenbar; GEO ausgeschlossen, bis Originalimport einen realen Rechtsträger und Standort belegt. Quellenbasis: dokumentierter Seed-/Provenienzbefund in diesem Dossier; Websuche 16.10.2026 (keine qualifizierende Zielquelle).]

### Provenienzprüfung 08.10.2026

- [Recherche 08.10.2026: Repo-Gegenprüfung statt weiterer Suche nach einem künstlichen Firmennamen: Die exakte Fragmentkombination ist nur in diesem Dossier erhalten; das in der Importhistorie vorausgesetzte ursprüngliche Register-/Tabellensubjekt ist hier nicht rekonstruierbar. Dieses Ergebnis gilt für den geprüften Markdown-Korpus, nicht für alle früheren Datenstände. `provenance_origin: prose` und `name: Eintrag mit` bleiben aus Gründen der Slug-/Historienstabilität erhalten, sind aber kein Betriebsbeleg; city unbekannt ist ausschließlich Schema-Platzhalter. Nicht mit dem Jenaer Heiko-Schmidt-Dossier zusammenführen, nur weil Vorwahl 03641 passt. ANKAUF/VERKAUF/GEBÜHREN: mangels identifizierter Firma nicht zuordenbar, nicht als preislose reale Annahmestelle darstellen. Offener Primärschritt: Originalimport/Registernummer beschaffen. Quelle: dieses Dossier, Importiert (Seed-Stand 2026-09-30), repo-interne Textsuche 08.10.2026; Validierungsbezug crates/ingestion/src/seed_traders.rs .]

### Importiert (Seed-Stand 2026-09-30)

- Registerfund ohne geprüfte Website (PLZ 03641)

### Quellen-/Feldprüfung 07.10.2026

- [Quellen-/Feldprüfung 07.10.2026: Die Suche nach der exakten Kombination „Eintrag mit“ + „03641“ + Schrott/Jena lieferte keine identifizierbare Firma. Das Telefonbuch ordnet 03641 ausdrücklich der Telefonvorwahl Jena zu; daraus folgt aber weder, dass das Fragment einen Jenaer Schrotthändler bezeichnet, noch ein tatsächlicher Betriebsstandort. Die fehlerverdächtigen Werte `city: Tel.` und `postcode: 03641` deshalb geleert statt als Anschrift übernommen oder eine Firma zu erraten. Offen bleibt die Herkunft des Register-Prosa-Schnipsels und sein eigentliches Subjekt. Keine bestätigte Tätigkeit, Ankauf-/Verkaufspreise oder Gebühren. Quelle(n): https://www.dastelefonbuch.de/Vorwahlen/03641--Jena ; https://html.duckduckgo.com/html/?q=%22Eintrag+mit%22+%2203641%22+Schrott+Jena]

- [Owner-Korrektur 07.10.2026: `city` muss laut `seed_traders::validate_seeds` nichtleer sein. Da keine Stadt identifiziert ist, steht dort jetzt `unbekannt` als Schema-Platzhalter, nicht als geografische Behauptung; `postcode` bleibt leer. Quelle: `crates/ingestion/src/seed_traders.rs` (Validierung verlangt nichtleere `city`).]
