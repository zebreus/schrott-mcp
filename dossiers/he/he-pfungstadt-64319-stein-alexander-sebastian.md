---
slug: he-pfungstadt-64319-stein-alexander-sebastian
clear_fields: 'geo'
name: Stein Alexander-Sebastian
trader_type: schrotthaendler
state: HE
city: Pfungstadt 64319
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
provenance_section: Nachtrag Audit-Runde 4b – Kandidaten-Sweep (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Stein Alexander-Sebastian

## Überblick

Das Dossier beruht auf Verzeichnisangaben für „Alexander-Sebastian Stein“ in Pfungstadt-Eschollbrücken (früher gemeldet: Tulpenweg 3, Telefon 06157 9110132). Es gibt keinen Betreiberauftritt, Register-/Impressumsbeleg oder sonstige aktuelle Primärquelle, die einen aktiven Schrottbetrieb bestätigt. Adresse und Telefonnummer wurden aus dem Frontmatter entfernt; sie bleiben als unbestätigte Leads in der Recherchehistorie.

**Preise:** keine Ankaufspreise, Verkaufspreise oder Entsorgungsgebühren veröffentlicht oder belastbar gefunden. Status `pruefung` bleibt; weder Betriebsaufgabe noch aktueller Ankauf ist belegt.

## Timeline

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 49.8056, 8.6031. Nominatim-Reverse löst den Punkt an der Rügnerstraße 67c auf, nicht am historischen Verzeichnislead Tulpenweg 3. Der Dossierstand belegt keinen Betreiberstandort. `clear_fields: 'geo'` entzieht das nicht passende Paar beim nächsten regulären Seed; keine Ersatzkoordinate gesetzt. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.8056&lon=8.6031&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026

- [Recherche 25.10.2026: Darmstadt Post führt in einem aktuellen Unternehmensverzeichniseintrag „Alexander-Sebastian Stein Schrotthandel“, Tulpenweg 3, 64319 Pfungstadt, Tel. 06157 9110132. Das ergänzt die älteren 11880-/Stadtbranchenbuch-Leads, ist aber ebenfalls ein Verzeichnis und keine Betreiber-/Registerbestätigung; die dort gezeigte Bewertung ist kein Nachweis laufender Annahme. Kein Frontmatter-Fill. ANKAUF: keine bestätigten Kurse; VERKAUF: keine Liste; GEBÜHREN: keine Angaben. GEO erst nach Identitäts- und Standortbestätigung. Quelle: https://darmstadt-post.de/unternehmen/pfungstadt/schrotthandel/alexander-sebastian-stein-schrotthandel-493/ (Verzeichnis, Abruf 25.10.2026).]

### Korrektur 17.10.2026 (Keep/Clear-Grenze)

- [Korrektur 17.10.2026: Frühere Feldrücknahmen bereits am 07.10. integriert, nicht als neue Wellenbereinigung zählen. Weiter nur Portal-Leads, keine Primärbestätigung/Schließungsbehauptung. street/postcode leer ersetzen bei geändertem Seed die gespeicherte Kernadresse; phone leer hält ohne clear_fields dagegen gespeicherte Anreicherung. Kein zusätzlicher Telefon-Clear autorisiert, daher keine DB-wirksame Telefonrücknahme behaupten. Servicefelder bleiben unbekannt/Keep, keine GEO-Freigabe; ohne Paar bleiben vorhandene Koordinaten erhalten, kein automatisches Geocoding. Quelle: research/checkin-20261017-1700-wave13-b-review.md; docs/trader-seed.md; Portal-Leads der alten Recherche]

### Korrektur/Recherche 07.10.2026

- [Korrektur 07.10.2026: Die bisherigen Werte Tulpenweg 3, 64319 und 06157 9110132 beruhten auf 11880 und Stadtbranchenbuch, also ausschließlich Branchenverzeichnisquellen; ein Betreiber-, Register- oder Kommunalbeleg zur aktiven Schrottfirma fehlt. Deshalb street/postcode/phone aus dem Frontmatter entfernt, Werte und damalige Doppelbeleg-Entscheidung bleiben in der Historie nachvollziehbar. NorthData-Suche nach „Stein Alexander-Sebastian Pfungstadt“ ergab keinen eindeutig passenden Rechtsträger; das beweist keine Nichtexistenz. ANKAUF: nicht bestätigt/keine Zahlenpreise; VERKAUF: keine Liste; GEBÜHREN: keine Angaben in den ausgewerteten Quellen. Offen: Identität, Adresse und aktueller Betrieb, ggf. Gewerberegister/Vor-Ort-Bestätigung; Quelle: https://www.northdata.de/?query=Stein%20Alexander-Sebastian%20Pfungstadt; https://www.11880.com/; https://pfungstadt.stadtbranchenbuch.com/]

### Importiert (Seed-Stand 2026-09-30)

- Kleinst-Schrotthändler
- Adresse: Pfungstadt 64319

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Gegenrecherche per Bing-Websuche (Direktabruf; Suchterme: Stein/Schrott/Pfungstadt): keine Betreiber-Website, kein Impressum, kein Handelsregister-/Northdata-Treffer, kein Betreiber-Social; Treffer nur namensgleiche Fremdfirmen und Orte; Vorbefund 'keine Website verifiziert' bestaetigt; kein Frontmatter-Fill (2-Beleg-Standard nicht erfuellbar); Klaerfall: kommunales Gewerberegister Pfungstadt oder Vor-Ort-Check; Quelle(n): Bing-SERP 01.10.2026, recherche/he.md]

### Recherche 01.10.2026 (Tiefenrecherche-Welle, Nachtrag)

- [Recherche 01.10.2026: Adress-/Telefon-Fund in ZWEI konsistenten Verzeichnissen — 11880 („Alexander-Sebastian Stein Schrotthandel“, Tulpenweg 3, 64319 Pfungstadt-Eschollbrücken, (06157) 9110132, Rubrik Schrotthandel/Altmetallrecycling) + stadtbranchenbuch Pfungstadt („Alexander-Sebastian Stein, Tulpenweg 3, 64319 Pfungstadt, 06157 9110132“): street/postcode/phone daher GEFÜLLT, aber Beleglage offen dokumentiert (beide Aggregatoren, KEIN Betreiber-Beleg, kein Register) — status bleibt pruefung. Keine Betreiber-Website, kein Impressum, kein Social. Namensvariante „Alexander Sebastian Stein“ (11880-Suche) vs. „Alexander-Sebastian Stein“ (11880-Profil) dokumentiert. Folgewelle: Gewerberegister Pfungstadt / Vor-Ort Eschollbrücken. Quelle(n): 11880.com, pfungstadt.stadtbranchenbuch.com.]

### Recherche 13.10.2026

- [Recherche 13.10.2026: Erneute Suche nach exaktem Namen, Pfungstadt und „Schrott“ ergab keine neue Betreiber-, Register- oder kommunale Quelle. Der frühere Tulpenweg-3-/64319-/06157-9110132-Treffer bleibt ein Verzeichnislead; die bereits dokumentierte Entfernung dieser Felder aus Frontmatter wird bestätigt. Negativbefund ist kein Nichtexistenz-/Schließungsnachweis. ANKAUF: nicht bestätigt/keine Zahlenpreise; VERKAUF/GEBÜHREN: keine Listen. Kein verifizierter Standort für GEO. Quelle/Suchweg: https://www.northdata.de/?query=Stein%20Alexander-Sebastian%20Pfungstadt; https://www.11880.com/; https://pfungstadt.stadtbranchenbuch.com/]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt, `clear_fields: 'geo'` bleibt bestehen.** Die
  Dossier-Hausnummer **3** auf dem "Tulpenweg" in Pfungstadt existiert in OSM **nicht**: das am 03.11.2026 live
  abgefragte Kachelraster ueber den Pfungstaedter Ortsbereich (Rasterfenster 8.5700/49.7850 bis 8.6400/49.8250,
  0,01-Grad-Kacheln ueber `scripts/release/osm_street_scan.py`, 40 Kacheln, 0 Fehler) liefert **0 Objekte**
  mit dem Strassennamen "Tulpenweg" und **kein** Adressobjekt mit der Hausnummer 3 - die Strasse selbst ist
  in OSM an dieser Stelle nicht kartiert. Quelle 1 (Betreiber/Register): **fehlt** - fuer "Stein
  Alexander-Sebastian" existiert weder eine Betreiberseite noch ein Impressum noch ein Register- oder
  Northdata-Treffer; der Dossier-Beleg ist ausschliesslich ein **Verzeichnislead** (Darmstadt Post:
  "Alexander-Sebastian Stein Schrotthandel, Tulpenweg 3, 64319 Pfungstadt, Tel. 06157 9110132"; dazu die
  aelteren 11880- und Stadtbranchenbuch-Eintraege), der nach dem Quellenstandard kein Beleg ist; die dort
  gezeigte Bewertung ist kein Nachweis laufender Annahme, und das Dossier fuehrt street/postcode/phone
  deshalb ausdruecklich **nicht** als Frontmatter-Werte. Quelle 2 (OSM): das Kachelraster (live, oben); ohne
  Adressobjekt kann kein Punkt gebildet werden. Der gespeicherte Altpunkt 49.8056/8.6031 (Rügnerstraße 67c)
  wird mit `clear_fields: 'geo'` ohnehin zurueckgenommen; er loest zudem auf eine **andere** Strasse als
  die Verzeichnisangabe "Tulpenweg 3" auf. Es wird **keine** Koordinate erfunden; `status` bleibt `pruefung`,
  und die offene Frage der kommunalen Gewerberegister-Auskunft bleibt offen.
  Quelle(n): `scripts/release/osm_street_scan.py` (Kachelraster) ;
  https://darmstadt-post.de/unternehmen/pfungstadt/schrotthandel/alexander-sebastian-stein-schrotthandel-493/ ;
  https://pfungstadt.stadtbranchenbuch.com/ ;
  read-only `public.db` (Abruf 03.11.2026)]

### Geokodierung 05.11.2026

- [Geokodierung 05.11.2026: **Klärfall - keine Koordinate gesetzt, `clear_fields: 'geo'` bleibt bestehen.** Die
  Dossier-Hausnummer **3** auf dem "Tulpenweg" in Pfungstadt-Eschollbrücken existiert in OSM **nicht** - die Straße ist
  dort nicht kartiert. Quelle 1 (OSM, live geprüft am 05.11.2026): **Fensterprüfung (kein Fensterfehler):** das live
  abgerufene Kachelraster über den Pfungstädter Ortsbereich umfasst **8.5700/49.7800 bis 8.6400/49.8300**, **37 Kacheln
  von je 0,01 Grad**, **0 Kachel-Fehler** (zwei zunächst mit HTTP 429 abgewiesene Kacheln wurden einzeln nachgeladen und
  ohne Fehler übernommen), 87224 Elemente; es liefert **0** Objekte mit dem Straßennamen "Tulpenweg" und **kein**
  Adressobjekt mit der Hausnummer 3. Quelle 2 (Betreiber/Register): **fehlt** - für "Stein Alexander-Sebastian" existiert
  weder eine Betreiberseite noch ein Impressum noch ein Register- oder Northdata-Treffer; der Dossier-Beleg ist
  ausschließlich ein **Verzeichnislead** (Darmstadt Post: "Alexander-Sebastian Stein Schrotthandel, Tulpenweg 3, 64319
  Pfungstadt, Tel. 06157 9110132"; dazu die älteren 11880- und Stadtbranchenbuch-Einträge), der nach dem Quellenstandard
  kein Beleg ist; die dort gezeigte Bewertung ist kein Nachweis laufender Annahme. **Warum keine Koordinate:** ohne
  adressgenaues OSM-Objekt kann kein Punkt gebildet werden, und die Adresse selbst ist nur lead-belegt; der
  `clear_fields: 'geo'`-Eintrag bleibt bestehen, der gespeicherte Altpunkt 49.8056/8.6031 (Rügnerstraße 67c) wird damit
  ohnehin zurückgenommen - er löst zudem auf eine **andere** Straße als die Verzeichnisangabe "Tulpenweg 3" auf. `status`
  bleibt `pruefung`. **Reichweitenklausel:** das Fehlen einer Hausnummer in OSM ist **kein** Nachweis für das Fehlen des
  Betriebs; die offene Frage der kommunalen Gewerberegister-Auskunft bleibt offen. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.57,49.78,8.64,49.83 ;
  https://darmstadt-post.de/unternehmen/pfungstadt/schrotthandel/alexander-sebastian-stein-schrotthandel-493/ ;
  https://pfungstadt.stadtbranchenbuch.com/ (Abruf 05.11.2026)]

### Geokodierung 05.11.2026 (Nachprüfung)

- [Geokodierung 05.11.2026: **Klärfall - keine Koordinate gesetzt, `clear_fields: 'geo'` bleibt bestehen.** Die
  Dossier-Hausnummer **3** auf dem "Tulpenweg" in 64319 Pfungstadt-Eschollbrücken existiert in OSM **nicht**; die
  Nachprüfung mit eng am Fundort liegendem Fenster bestätigt den Negativbefund der Vorwellen-Notiz vom 05.11.2026.
  Quelle 1 (OSM, live geprüft am 05.11.2026): **Fensterprüfung (kein Fensterfehler):** das live abgerufene
  Kachelraster **8.5900/49.8000 bis 8.6200/49.8200** umfasst **6 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**,
  35012 Elemente, 3078 Adressobjekte auf 114 Straßen; die Straßenliste enthält **keine** Straße "Tulpenweg", und eine
  Suche nach "Tulpen|Tulpenweg" über die Felder name, operator, brand, description, website, contact:website, shop,
  craft und addr:housename findet im Raster **0 Treffer**. Das **Kontrollfenster 8.6000/49.8000 bis 8.6400/49.8400**
  (**16 Kacheln**, **0 Kachel-Fehler**, 41973 Elemente, 3090 Adressobjekte auf 118 Straßen) bestätigt das für den
  gesamten Pfungstädter Ortsbereich einschließlich Eschollbrücken: auch dort **0** Treffer auf "Tulpen|Tulpenweg". Das
  Fenster liegt am Fundort: der Ortsteil
  **Eschollbrücken** (way/150959725, `name`=Eschollbrücker Straße, BBox 8.5898058/49.8012697 bis
  8.5931821/49.8025529) liegt darin ebenso wie der bisher gespeicherte Punkt (Rügnerstraße 67c = way/332647042 bei
  8.6027920/49.8053659, Objekt-BBox 8.6027920/49.8053659 bis 8.6031167/49.8055362) - der gespeicherte Punkt löst auf
  eine **andere** Straße als die Verzeichnisangabe "Tulpenweg 3" auf. Quelle 2 (Betreiber/Register): **fehlt** - für
  "Stein Alexander-Sebastian" existiert weder eine Betreiberseite noch ein Impressum noch ein Register- oder
  Northdata-Treffer; der Dossier-Beleg ist ausschließlich ein **Verzeichnislead** (Darmstadt Post: "Alexander-Sebastian
  Stein Schrotthandel, Tulpenweg 3, 64319 Pfungstadt, Tel. 06157 9110132"; dazu die älteren 11880- und
  Stadtbranchenbuch-Einträge), der nach dem Quellenstandard kein Beleg ist; die dort gezeigte Bewertung ist kein
  Nachweis laufender Annahme, und das Dossier führt street/postcode/phone deshalb ausdrücklich **nicht** als
  Frontmatter-Werte. **Warum keine Koordinate:** ohne adressgenaues OSM-Objekt kann kein Punkt gebildet werden, und
  die Adresse selbst ist nur lead-belegt; der `clear_fields: 'geo'`-Eintrag bleibt bestehen, der gespeicherte
  Altpunkt 49.8056/8.6031 (Rügnerstraße 67c) wird damit ohnehin zurückgenommen; `status` bleibt `pruefung`, und die
  offene Frage der kommunalen Gewerberegister-Auskunft der Stadt Pfungstadt bleibt offen. **Reichweitenklausel:**
  das Fehlen einer Hausnummer in OSM ist **kein** Nachweis für das Fehlen des Betriebs. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.59,49.8,8.62,49.82 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.6,49.8,8.64,49.84 ;
  https://api.openstreetmap.org/api/0.6/way/150959725/full.json ;
  https://api.openstreetmap.org/api/0.6/way/332647042/full.json ;
  https://darmstadt-post.de/unternehmen/pfungstadt/schrotthandel/alexander-sebastian-stein-schrotthandel-493/ ;
  https://pfungstadt.stadtbranchenbuch.com/ (Abruf 05.11.2026)]
