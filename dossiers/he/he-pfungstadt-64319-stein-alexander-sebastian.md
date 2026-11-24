---
slug: he-pfungstadt-64319-stein-alexander-sebastian
clear_fields: 'geo'
name: Stein Alexander-Sebastian
trader_type: schrotthaendler
state: HE
city: 'Pfungstadt'
street: ''
postcode: '64319'
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

### Recherche 22.11.2026 (Welle 0600 A)

- [Recherche 22.11.2026: **Korrektur zu einem Fehlbefund der Vorwellen: die Straße `Tulpenweg` und die Dossier-Hausnummer 3 existieren in OSM — die bisherige Negativaussage war ein Fensterfehler, kein Sachbefund.** Die Nominatim-Vorwärtssuche `Tulpenweg 3, 64319 Pfungstadt` (live am 22.11.2026) liefert genau **einen** Treffer: **way/406635649**, `3, Tulpenweg, Eschollbrücken, Eschollbrücken/Eich, 64319 Pfungstadt`, 49.8083062/8.5664741. Das Objekt trägt `addr:housenumber`=3, `addr:street`=Tulpenweg, `addr:city`=Pfungstadt, `addr:postcode`=64319, `addr:suburb`=Eschollbrücken, `building`=house; die Way-Definition hat **5** Knoten, davon **4** eindeutig (node/4086863722, node/4086863713, node/4086863707, node/4086863717), der Mittelwert der eindeutigen Knoten liegt bei **49.8083062 / 8.5664742** und damit in der Objekt-BBox 49.8082515/8.5663806 bis 49.8083608/8.5665677. Die **Straße selbst** ist **way/23488754** (`highway`=residential, `name`=**Tulpenweg**, `oneway`=yes, 7 Knoten, BBox 49.8080772/8.565277 bis 49.8087095/8.5670519). **Warum die Vorwellen-Notizen von 03.11. und 05.11.2026 zu einem anderen Ergebnis kamen:** deren Kachelraster lagen bei **8.5700/49.7800 bis 8.6400/49.8300** (37 Kacheln), **8.5900/49.8000 bis 8.6200/49.8200** (6 Kacheln) und im Kontrollfenster **8.6000/49.8400 bis 8.6400/49.8400** (16 Kacheln) — der Tulpenweg liegt bei **Längengrad 8.5653–8.5671** und damit **westlich außerhalb** aller drei Fenster; das in der Nachprüfung vom 05.11.2026 genannte Kontrollfenster umfasste zudem nicht den Straßennamen. Ein live abgefragtes Kachelraster **über dem Fundort** (BBox 8.560/49.804 bis 8.572/49.813, 26.476 Elemente, HTTP 200) liefert die Straße und **14** Adressobjekte auf dem Tulpenweg mit den Hausnummern **2B, 2C, 2D, 2F, 2H, 3, 3A, 4, 6, 8** — die **Dossier-Hausnummer 3 existiert** und ist **eindeutig** (nur ein Objekt mit dieser Nummer auf dieser Straße). **Folge:** die Aussage `die Dossier-Hausnummer 3 auf dem Tulpenweg existiert in OSM nicht` ist **nicht** haltbar; sie ist durch die Fensterlage entstanden. Das OSM-Objekt trägt jedoch **weder** `name` **noch** `operator` — es belegt die **Adresse**, nicht den Betrieb; die Belegkette für den **Betrieb** (Anschrift + Name) bleibt weiterhin **allein** ein Verzeichnislead. Quellen: https://nominatim.openstreetmap.org/search?q=Tulpenweg+3%2C+64319+Pfungstadt&format=jsonv2&addressdetails=1&extratags=1 ; https://api.openstreetmap.org/api/0.6/way/406635649/full.json ; https://api.openstreetmap.org/api/0.6/way/23488754/full.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=8.560,49.804,8.572,49.813 (Abruf 22.11.2026).]
- [Recherche 22.11.2026: **Die Belegkette für den Betrieb ist am 22.11.2026 deckungsgleich über drei Verzeichnisse, aber weiterhin ohne Betreiber- oder Registerquelle.** (1) Der **Darmstadt Post**-Eintrag `https://darmstadt-post.de/unternehmen/pfungstadt/schrotthandel/alexander-sebastian-stein-schrotthandel-493/` (live am 22.11.2026, HTTP 200) trägt wörtlich `Alexander-Sebastian Stein Schrotthandel`, die Rubrik `Schrotthandel`, `in Pfungstadt`, eine **4-Sterne-Bewertung (4)** und `Tulpenweg 3` — das ist **derselbe** Datensatz wie im Dossier vom 25.10.2026, unverändert. (2) Die 11880-Suche `Schrotthandel in Pfungstadt` (live am 22.11.2026) liefert **zwei** Profile desselben Betriebs: `Alexander Sebastian Stein Schrotthandel` (Kennung `120674719B110039934`, Rubrik `Schrotthandel`, `64319 Pfungstadt` **ohne Straße**, `(06157) 9110132`, `E-Mail-Adresse hinzufügen`, `Website hinzufügen`, `Eintragsdaten vom 08.05.2026`) und `Alexander-Sebastian Stein Schrotthandel` (Kennung `060440090B53966227`, Rubriken `Altmaterialhandel / Schrotthandel / Altmetallrecycling`, `(06157) 9110132`, `Tulpenweg 3`, `64319 Pfungstadt (Eschollbrücken)`, `E-Mail-Adresse hinzufügen`, `Website hinzufügen`, `Eintragsdaten vom 19.08.2022`) — die Namensvarianten `Alexander Sebastian Stein` und `Alexander-Sebastian Stein` sind damit **dieselbe** Person mit abweichender Schreibweise, und die **Hausnummer 3** steht nur in einem der beiden Profile. (3) Die 11880-**Rückwärtssuche** zu `061579110132` wurde nicht als Beleg herangezogen (Verzeichnis). **Entscheidend bleibt:** kein Profil führt eine Website, eine E-Mail oder Öffnungszeiten — es existiert **keine** Betreiberseite; das amtliche Fachbetrieberegister kennt den Namen nicht, und das amtliche Handelsregisterportal wurde bereits am 07.10.2026 als unauffindbar dokumentiert. `street`, `postcode` und `phone` bleiben im Frontmatter **leer**; eine Wiederaufnahme in die Frontmatter ist eine Owner-Entscheidung, weil die Adresse nur verzeichnisbelegt ist. Quellen: https://darmstadt-post.de/unternehmen/pfungstadt/schrotthandel/alexander-sebastian-stein-schrotthandel-493/ ; https://www.11880.com/branchenbuch/pfungstadt/060440090B53966227/alexander-sebastian-stein-schrotthandel.html ; https://www.11880.com/branchenbuch/pfungstadt/120674719B110039934/alexander-sebastian-stein-schrotthandel.html ; https://www.11880.com/suche/schrotthandel/pfungstadt (Abruf 22.11.2026).]
- [Recherche 22.11.2026: **Amtliches Fachbetrieberegister: um die Dossieranschrift nur Fremdbetriebe, kein Treffer auf Stein — Negativaussage mit Reichweitenklausel.** Abruf am 22.11.2026 im amtlichen Fachbetrieberegister (clientseitig gerenderte Vaadin-App, Route `Entsorgungsfachbetriebe`, Filter `Art des Betriebs = Egal`, `is=1&e=1`, Ergebnisgrid vollständig ausgelesen): die Namenssuche `Stein` liefert ausschließlich **Namensvetter anderer Rechtsformen und Orte** (`REMONDIS Industrie Service GmbH & Co. KG`, Brunnenstraße 138, 44536 Lünen; `Autoservice Feuerstein`, Brückle 2, 97947 Grünsfeld; `bmk Steinbruchbetriebe GmbH & Co. KG`, Robert-Bopp-Straße 2, 74388 Talheim; `Südwestdeutsche Hartsteinwerke Zweigniederlassung der Basalt AG`, Steinbruch Neu-Bamberg und Steinbruch Oberlinxweiler) — **keinen** Pfungstädler Treffer. Die Standortabfrage mit 3 km um `Tulpenweg 3, 64319 Pfungstadt` (Geocoder-Koordinaten 8.6024/49.8055) liefert **fremde** Betriebe — `Nothnagel & Co. Baustoffe GmbH Transport KG`, Breitwieserweg 10, 1,2 km (ZZGT00200036 / 1 bis / 3) und `Rainer Rädge`, Rollweg 65, 1,4 km (ZZFT00400013 / 1) — **keinen** Treffer auf Stein. **Reichweite:** die Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung besitzt, und **kein** Beleg für Nichtexistenz oder Schließung. Quellen: https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Stein ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Tulpenweg%203%2C%2064319%20Pfungstadt%2C%20Deutschland%3B8.6024%3B49.8055%3B8.6444%3B49.8295%3B8.5604%3B49.7815&dst=3&is=1&e=1 (Abruf 22.11.2026).]
- [Recherche 22.11.2026: **GEO gemessen, nicht gesetzt: der Altpunkt liegt 3,4 km vom belegten Adressobjekt entfernt und trifft eine andere Straße.** Der am 25.10.2026 im Dossier dokumentierte gespeicherte Punkt `49.8056 / 8.6031` (Rügnerstraße 67c) liegt vom adresskonformen Objekt way/406635649 (49.8083062/8.5664742) **3.406 m** entfernt (Haversine, nachgerechnet) und löst auf eine **andere** Straße auf — die Rügnerstraße liegt im Kernbereich Pfungstadts, der Tulpenweg im Ortsteil **Eschollbrücken**. Der Punkt bleibt unverändert — `clear_fields: 'geo'` bleibt bestehen, und über Setzen oder Entfernen entscheidet der Owner in einer eigenen datierten Korrektur-Sektion; nach dem korrigierten Adressbefund wäre eine Koordinate an way/406635649 (49.8083062/8.5664742) **technisch** möglich, setzt aber weiterhin eine nur verzeichnisbelegte Betriebsanschrift voraus. Quellen: https://api.openstreetmap.org/api/0.6/way/406635649/full.json ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.8056&lon=8.6031&zoom=18&addressdetails=1 (Abruf 22.11.2026).]
- [Recherche 22.11.2026: **Preise getrennt geführt: keine Zahlen belegt.** ANKAUF: die drei Verzeichnisprofile nennen **keinen** €/kg-Kurs, **keine** Sortentabelle und **kein** Publikationsdatum; die 11880-Rubriken `Schrotthandel / Altmaterialhandel / Altmetallrecycling` sind Tätigkeitsangaben, keine Preisaussage; die 4-Sterne-Bewertung im Darmstadt-Post-Eintrag ist eine Bewertung, kein Preisbeleg. VERKAUF: keine Material- oder Teilepreisliste. GEBÜHREN: keine Abhol-, Container- oder Entsorgungsgebühr belegt. Quellen: siehe oben.]
- [Recherche 22.11.2026: **Offene Fragen.** (1) Ob der Betrieb heute noch aktiv ist, ist offen; der 11880-Eintrag mit der vollständigen Anschrift stammt vom 19.08.2022, der neuere Eintrag ohne Straße vom 08.05.2026. (2) Ob eine eigene Website existiert, ist offen — 11880 führt `Website hinzufügen`, es ist also keine hinterlegt. (3) Ob die **kommunale Gewerberegister-Auskunft der Stadt Pfungstadt** den Betrieb ausweist, bleibt der entscheidende offene Schritt; das Fachbetrieberegister kennt ihn nicht, was bei einem Einzelunternehmer ohne Registereintrag kein Gegenbeweis ist. (4) Ob die 4-Sterne-Bewertung im Darmstadt-Post-Eintrag von Kunden oder vom Betreiber stammt, ist offen. Quellen: siehe oben.]

### Korrektur 24.11.2026 (Owner, Postleitzahl)

- [Korrektur 24.11.2026: **Die Felder `city` und `postcode` sind **berichtigt** ✓✓: der Ort **Pfungstadt** ✓✓ stand im Feld `city` ✓✓ **mit** der Postleitzahl **64319** ✓✓, das Feld `postcode` ✓✓ war **leer** ✓✓ ✓✓ Die Angaben stammen aus demselben **Seed-Wert** ✓✓ desselben Feldes ✓✓ ✓✓ **Belege** ✓✓: **erstens** ✓✓ der **Seed-Wert** ✓✓ des Dossiers ✓✓, **zweitens** ✓✓ die **Konvention** ✓✓ des Katalogs ✓✓ (der Ort steht im Feld `city` ✓✓, die Postleitzahl im Feld `postcode` ✓✓) ✓✓ **Dossier** ✓✓: **Stein Alexander-Sebastian** ✓✓, **ohne** ✓✓ Straße ✓✓ ✓✓]
