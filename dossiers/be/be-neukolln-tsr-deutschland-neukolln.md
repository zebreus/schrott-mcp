---
slug: be-neukolln-tsr-deutschland-neukolln
name: TSR Deutschland GmbH & Co. KG (Neukölln)
trader_type: sonstige
state: BE
city: Neukölln
street: Lahnstr. 1-9
postcode: '12055'
phone: 030 63225780
email: ''
opening_hours: Mo–Do 07:00–11:30, 12:30–15:30, Fr 07:00–11:30, 12:30–14:30
website: https://www.tsr-recycling.de
website_status: blockiert
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Berlin (BE) — Recherche
provenance_ankauf_raw: ja
provenance_origin: table
---

# TSR Deutschland GmbH & Co. KG (Neukölln)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Großrecycler Fe + NE, Privat + Gewerbe
- Adresse: Lahnstr. 1-9, 12055 Berlin
- Bezirk: Neukölln Adressbeleg: Yelp/MapQuest Lahnstr. 3-5 (Websuche 28.09.2026, mit Vorbehalt)

### Korrektur 30.09.2026 (Feedback-Triage)

- [Korrektur Feedback fb1825: Telefon 030 632257-6 → 030 63225780 vierquellig (TSR-Standortseite via Jina-Reader: T +49 30 63225780, F ...88 + golocal + schrottplatz-info.de + Öffnungszeitenbuch übereinstimmend „(030) 63 22 57 80"); alte „-6"-Nummer unvollständig/falsch; Quelle: tsr-recycling.de/standorte/schrotthaendler-berlin-neukoelln + golocal.de/berlin/schrotthandel/tsr-deutschland-niederlassung-berlin-neukoelln-11CbeJ + schrottplatz-info.de/schrottplatz/Berlin/TSR-Recycling + oeffnungszeitenbuch.de Berlin-Neukoelln-678251R]
- [Korrektur Feedback fb1825: Zeiten Mo–Do 07:00–11:30 + 12:30–15:30, Fr bis 14:30 dreiquellig (TSR-Seite + golocal + Öffnungszeitenbuch); Quelle: wie oben]
- [Korrektur Feedback fb1825: website_status blockiert (Human-Challenge auf tsr-recycling.de, verifiziert 30.09.2026); Quelle: Direktabruf]
- Hinweis: TSR-Seite meldete befristet „29./30.09.2026 NE-Annahme ganztägig geschlossen (Jahresinventur), ab 01.10.26 normal" — temporär, nicht in opening_hours übernommen; TSR Berlin hat 2. Standort Westhafenstr. 1, 13353 (Moabit, Tel. 030 39881990, eigener Slug nötig falls gewünscht).

### Recherche 31.10.2026

- [Recherche 31.10.2026: Die von der Betreiber-Kette selbst betriebene Standortseite war am 31.10.2026 erneut nicht abrufbar (Bot-Challenge, HTTP 200 mit der Seite "Challenge Validation" statt Inhalt), der Betreiber-Beleg kam daher aus einer zweiten, unabhaengigen Firmen-Profilquelle mit demselben Datenstand. Die Profilseite weist den Betrieb als "TSR Deutschland GmbH & Co. KG // Niederlassung Berlin Neukölln" mit "+493063225780", "tsr-recycling.de" und der Mail "info@tsr.eu" aus und bestaetigt die Oeffnungszeiten wortgleich: "Monday 7:00 AM - 11:30 AM, 12:30 PM - 3:30 PM", "Tuesday 7:00 AM - 11:30 AM, 12:30 PM - 3:30 PM", "Wednesday 7:00 AM - 11:30 AM, 12:30 PM - 3:30 PM", "Thursday 7:00 AM - 11:30 AM, 12:30 PM - 3:30 PM", "Friday 7:00 AM - 11:30 AM, 12:30 PM - 2:30 PM", "Saturday Closed", "Sunday Closed"; die Mail "info@tsr.eu" ist damit erstmals belegt und wurde in die Timeline aufgenommen (die Frontmatter bleibt unveraendert, dort ist email leer). Ebenfalls neu belegt ist die Standortbeschreibung: "TSR zaehlt zu den europaweit fuehrenden Unternehmen fuer das Recycling von Metallen. Als modernes Unternehmen decken unsere zuverlaessigen Mitarbeiter an unseren europaeweiten Standorten alle Aufgaben ab, die beim Thema Schrott anfallen. Neben Ankauf, Verkauf und Aufbereitung von Altmetallen gehoeren auch die Bereiche Downstream und Elektrorecycling zu unserem Leistungsspektrum. So produzieren wir hochwertige Recyclingrohstoffe, die in der Produktion wieder eingesetzt werden koennen." Das stuetzt die Einordnung "Grossrecycler Fe + NE, Privat + Gewerbe". OSM-Abgleich (Rasterfenster 13.4416080/52.4671687 bis 13.4562532/52.4695854, Lahnstrasse, 77 Adressobjekte mit addr:housenumber und addr:street=Lahnstrasse, daraus 64 verschiedene Hausnummern; die Bereichsnotation "36-40" ist einzeln aufgeloest): Die Dossier-Hausnummer 1-9 ist im OSM als Einzelobjekte erfasst, node/2820306557 mit `addr:housenumber=1`, `addr:street="Lahnstraße"`, `addr:city=Berlin`, `addr:postcode=12055`, `addr:suburb="Neukölln"`, `addr:inclusion=potential`, `source=Geoportal Berlin / Hauskoordinaten`, node/2820306600 mit `addr:housenumber=3`, node/2820307640 mit `addr:housenumber=5`, `addr:inclusion=potential` und node/2820307678 mit `addr:housenumber=9` - die Nummern 2, 4, 6, 7 und 8 sind im Raster nicht belegt, eine Adressangabe "1-9" existiert in OSM nicht. Im Raster tragen 310 Objekte `name=` und/oder `operator=` (240 mit `name=`, 134 mit `operator=`); 0 Objekte nennen "TSR", einen Recycling- oder Schrottbetrieb dieser Kette. Der einzige Objekt mit Recycling-Bezug gehoert zu einem anderen Betreiber: node/2820307602 mit `addr:housenumber=31`, `addr:street="Lahnstraße"`, `amenity=recycling`, `recycling_type=container`, `operator=REMONDIS`, `operator:wikidata=Q884975` sowie way/456816396 mit `landuse=commercial`, `name=Remondis Industriepark`. Das ist ein starker Hinweis darauf, dass der Standort in OSM nicht einem TSR-Betrieb zugeordnet ist, aber kein Negativbeleg ueber den Betrieb. Zu Ankauf, Verkauf und Gebuehren getrennt: ANKAUF - die Profilseite nennt "Neben Ankauf, Verkauf und Aufbereitung von Altmetallen" das Leistungsspektrum, nennt aber keinen Ankaufskurs und keine Preisliste; eine Sortenliste fuer den Standort Neukoelln ist nicht veroeffentlicht. VERKAUF - die Profilseite nennt den Verkauf von Recyclingrohstoffen ("So produzieren wir hochwertige Recyclingrohstoffe, die in der Produktion wieder eingesetzt werden koennen"), nennt aber keine Verkaufskurse. GEBUEHREN - keine Gebuehrentafel und keine Anlieferungsgebuehr fuer Privatpersonen veroeffentlicht. Offene Fragen: Die Betreiber-Seite tsr-recycling.de bleibt hinter der Challenge und liefert keine Inhalte; die Herkunft der Seed-Adresse "Lahnstr. 1-9" (Yelp/MapQuest laut Seed "Lahnstr. 3-5", mit Vorbehalt) ist damit weiterhin nur ein Verzeichnislead, der im OSM in dieser Form nicht existiert; eine zweite unabhaengige Bestaetigung der Hausnummer steht aus. Quelle(n): https://fr.nextdoor.com/pages/tsr-recycling-gmbh-co-kg-berlin-berlin/ ; https://www.tsr-recycling.de/standorte/schrotthaendler-berlin-neukoelln (Abruf 31.10.2026: Bot-Challenge, kein Inhalt) ; OSM-API-Abfrage api.openstreetmap.org/api/0.6/map.json?bbox=13.4416080,52.4671687,13.4562532,52.4695854 (Abruf 31.10.2026) ; https://www.openstreetmap.org/node/2820307678 ; https://www.openstreetmap.org/node/2820307602]

- [31.10.2026 (Korrekturvermerk): **OSM-Tag-Zitat berichtigt**: der zitierte Wert `addr:street="Lahnstrasse"` existiert in OSM nicht — live trägt das Objekt `addr:street="Lahnstraße"`. Die **Zählungen** im Dossier bleiben unverändert (die Zählung lief auf der richtigen, umgelauteten Straße). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (Raster laut Dossier, Abruf 31.10.2026)]

- [31.10.2026 (Korrekturvermerk): **OSM-Tag-Zitat berichtigt**: der zitierte Wert `addr:suburb="Neukoelln"` existiert in OSM nicht — live trägt das Objekt `addr:suburb="Neukölln"`. Dieser Vermerk betrifft den **TSR-Standort Neukölln**. Die **Zählungen** im Dossier bleiben unverändert (die Zählung lief auf der richtigen, umgelauteten Straße). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (Raster laut Dossier, Abruf 31.10.2026)]
