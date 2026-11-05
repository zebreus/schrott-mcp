---
slug: he-linden-burns-schrotthandel
clear_fields: 'geo'
name: Burns Schrotthandel
trader_type: schrotthaendler
state: HE
city: Linden
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
provenance_section: Einträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Burns Schrotthandel

## Überblick

Ein aktueller Gelbe-Seiten-Eintrag führt „Burns Schrotthandel“, Hüttenberger Str. 23, 35440 Linden, Tel. 06403 778468, Rubrik Schrott. Das ist ein einzelner Branchenverzeichnisbeleg, kein Betreiber-/Registerbeleg; Annahmestelle, heutiger Betrieb und Zuordnung zum Seed sind nicht bestätigt. Keine Kontaktdaten oder Betriebsmerkmale ins Frontmatter übernommen.

**Preise:** ANKAUF – keine Kurse oder Bedingungen bestätigt. VERKAUF – keine Liste. GEBÜHREN – keine belegten Tarife. GEO erst nach Bestätigung, dass Adresse und tatsächliche Annahmestelle zum Betrieb gehören.

## Timeline

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält für den Linden-Lead den Punkt 51.18435, 8.33325. Nominatim-Reverse verortet ihn in Huxel bei Schmallenberg, Hochsauerlandkreis, nicht in Linden (Hessen). `clear_fields: 'geo'` entzieht das falsche Paar beim nächsten regulären Seed. Die Hüttenberger-Str.-Adresse bleibt ein einzelner Verzeichnislead; keine Ersatzkoordinate gesetzt. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.18435&lon=8.33325&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026

- [Recherche 25.10.2026: Gelbe-Seiten-Detailseite direkt geprüft: „Burns Schrotthandel“, Hüttenberger Str. 23, 35440 Linden, Tel. 06403 778468, Rubrik Schrott. Der Datensatz bietet keinen Betreiberauftritt, keine Preise und keine unabhängig belegte Annahmefunktion. Deshalb bleibt er ein einzelner Verzeichnislead; kein Kontakt-/Adressfill und kein Schluss auf aktuellen Betrieb. ANKAUF: unbekannt; VERKAUF: keine Liste; GEBÜHREN: keine Angaben. GEO nicht vor Bestätigung der Betreiberidentität und Annahmestelle. Quelle: https://www.gelbeseiten.de/gsbiz/a2affcc7-5875-4371-9970-9fa8cc8feabe (Verzeichnis, Abruf 25.10.2026).]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Klärfall/miss bleibt — erneut kein Betreiber-Beleg zu Burns Schrotthandel in Linden (keine Betreiber-Website, kein HRB, keine kommunale Nennung, nur Aggregator-Leads); keine 2 unabhängigen Belege → weiter keine Frontmatter-Fills. Quelle(n): Websuche ohne Betreiber-Treffer]

### Importiert (Seed-Stand 2026-09-30)

- Schrotthändler

### Recherche 01.10.2026

- [Recherche Burns Linden: kein 2. unabhängiger Beleg, nur Aggregator-Leads, daher kein Frontmatter-Fill, Klärfall; Quelle(n): Suche ohne Betreiber-Beleg]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt, `clear_fields: 'geo'` bleibt bestehen.** Die
  Dossier-Hausnummer **23** auf der "Hüttenberger Straße" in Linden existiert in OSM: das am 03.11.2026 live
  abgefragte Kachelraster ueber den Lindener Ortsbereich (Rasterfenster 8.5900/50.5100 bis 8.6900/50.5550,
  0,01-Grad-Kacheln ueber `scripts/release/osm_street_scan.py`, 50 Kacheln, 0 Fehler) weist auf der Strasse
  **36 Adressobjekte** mit **35 verschiedenen Hausnummern** aus (2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17,
  19, 21, 22, **23**, 24, 25, 26, 27, 28, 29, 30, 32, 34, 36, 38, 40, 42, 43, 30a, 17 A, 27 A) - die Nummer 23
  ist als `way/609560812` erfasst (`addr:housenumber`=23, `addr:street`=Hüttenberger Straße,
  `addr:postcode`=35440, `addr:city`=Linden, `building`=yes; 4 eindeutige Knoten, BBox 8.6450431/50.5299585 bis
  8.6451703/50.5300505, Mittelwert der eindeutigen Knoten 50.5300045/8.6451067); die Strasse selbst ist als
  way/27194110 und way/38097770 (`highway=secondary`, `ref`=L 3129, `postal_code`=35440) kartiert. Das Objekt
  traegt **weder** `name` **noch** `operator`; die einzigen benannten Objekte der Strasse sind `name=Nau,
  Daniel` (Nummer 29) und `name=Autovermietung Thorald Schäfer GmbH` (Nummer 43) - ein Objekt mit Bezug zu
  "Burns", "Schrott" oder "Metall" existiert im Raster **nicht** (die Treffer zu `amenity=recycling` sind
  kommunale Sammelstellen und der kommunale "Wertstoffhof Linden"). Quelle 1 (Betreiber/Register): **fehlt** -
  die Gelbe-Seiten-Detailseite (live gelesen am 25.10.2026) fuehrt "Burns Schrotthandel", Hüttenberger Str. 23,
  35440 Linden, Tel. 06403 778468, Rubrik Schrott, bietet aber **keinen** Betreiberauftritt, **keine** Preise und
  **keine** unabhaengig belegte Annahmefunktion; eine Websuche vom 02.10.2026 fand keinen Betreiber-, HRB- oder
  kommunalen Beleg zu "Burns Schrotthandel" in Linden, nur Aggregator-Leads. Ein Verzeichniseintrag ist nach dem
  Quellenstandard kein Betreiberbeleg. Quelle 2 (OSM): way/609560812 sowie das Kachelraster (live, oben).
  Damit liegt eine belegte **Anschrift** vor, aber **kein** belegter **Betrieb**; ein Punkt auf der
  Hüttenberger Straße 23 wuerde einen Standort suggerieren, den weder OSM noch eine Betreiber- oder
  Registerquelle dem Seed zuordnet. Es wird **keine** Koordinate erfunden. Der gespeicherte Altpunkt
  51.18435/8.33325 (Huxel bei Schmallenberg, Hochsauerlandkreis) liegt rund 300 km entfernt und wird mit
  `clear_fields: 'geo'` ohnehin zurueckgenommen; `status` bleibt `pruefung`, fehlende Belege beweisen keine
  Nichtexistenz oder Schliessung.
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/609560812/full.json ;
  https://api.openstreetmap.org/api/0.6/way/27194110/full.json ;
  `scripts/release/osm_street_scan.py` (Kachelraster) ;
  https://www.gelbeseiten.de/gsbiz/a2affcc7-5875-4371-9970-9fa8cc8feabe ;
  https://schrottregister.pages.dev/ort-linden ; read-only `public.db` (Abruf 03.11.2026)]

### Geokodierung 05.11.2026

- [Geokodierung 05.11.2026: **Klärfall - keine Koordinate gesetzt, `clear_fields: 'geo'` bleibt bestehen.** Die
  Dossier-Hausnummer **23** auf der "Hüttenberger Straße" in Linden **existiert in OSM**; nicht belegt ist der
  **Betrieb**. Quelle 1 (OSM, live geprüft am 05.11.2026): **Fensterprüfung (kein Fensterfehler):** das live abgerufene
  Kachelraster **8.6350/50.5200 bis 8.6550/50.5400** umfasst **4 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler** nach
  Wiederholung der zunächst mit HTTP 429 abgewiesenen zwei Kacheln, 14342 Elemente; die Straße "Hüttenberger Straße" ist
  dort mit **45** Adressobjekten und **35** verschiedenen Hausnummern erfasst (2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
  17, 17 A, 19, 21, 22, **23**, 24, 25, 26, 27, 27 A, 28, 29, 30, 30a, 32, 34, 36, 38, 40, 42, 43) - die Dossier-Hausnummer
  23 ist im Raster das **einzige** Objekt mit dieser Nummer. way/609560812, live geladen am 05.11.2026 (HTTP 200,
  https://api.openstreetmap.org/api/0.6/way/609560812/full.json), trägt `addr:housenumber`=23, `addr:street`=Hüttenberger
  Straße, `addr:postcode`=35440, `addr:city`=Linden, `building`=yes; **5** Knoten, davon **4** eindeutig, Objekt-BBox
  8.6450431/50.5299585 bis 8.6451703/50.5300505, der Punkt läge innerhalb. **Betriebsbezug in OSM:** keiner - das Objekt
  trägt **weder** `name` **noch** `operator`; die Suche nach "Schrott|Burns|Metall" über die Felder name, operator, brand,
  description, website, contact:website, shop, craft und addr:housename findet im Kachelausschnitt **keinen** Treffer; die
  einzigen benannten Objekte der Straße sind `name`=Nau, Daniel (Nummer 29) und `name`=Autovermietung Thorald Schäfer GmbH
  (Nummer 43). Quelle 2 (Betreiber/Register): **fehlt** - die Gelbe-Seiten-Detailseite (live gelesen am 25.10.2026 und hier
  zitiert) führt "Burns Schrotthandel", Hüttenberger Str. 23, 35440 Linden, Tel. 06403 778468, Rubrik Schrott, bietet aber
  **keinen** Betreiberauftritt, **keine** Preise und **keine** unabhängig belegte Annahmefunktion; eine Websuche vom
  02.10.2026 fand keinen Betreiber-, HRB- oder kommunalen Beleg zu "Burns Schrotthandel" in Linden, nur Aggregator-Leads.
  **Warum keine Koordinate:** es liegt eine belegte **Anschrift** vor, aber **kein** belegter **Betrieb**; ein Punkt auf der
  Hüttenberger Straße 23 würde einen Standort suggerieren, den weder OSM noch eine Betreiber- oder Registerquelle dem Seed
  zuordnet. Der gespeicherte Altpunkt 51.18435/8.33325 (Huxel bei Schmallenberg, rund 300 km entfernt) wird mit
  `clear_fields: 'geo'` ohnehin zurückgenommen; `status` bleibt `pruefung`. **Reichweitenklausel:** das Fehlen eines
  Betreiber- oder Registerbelegs ist **kein** Nichtexistenz- oder Schließungsbeweis; die Anschrift ist in OSM belegt.
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/609560812/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.635,50.52,8.655,50.54 ;
  https://www.gelbeseiten.de/gsbiz/a2affcc7-5875-4371-9970-9fa8cc8feabe ;
  https://schrottregister.pages.dev/ort-linden (Abruf 05.11.2026)]
