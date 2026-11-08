---
slug: nw-essen-hade-schrott-metallgrohandel-stamm
clear_fields: geo
name: Häde Schrott-/Metallgroßhandel (Stamm)
trader_type: schrotthaendler
state: NW
city: Essen
street: Essen Heßlerstr. 77
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
provenance_section: Neu, Ankauf unklar (nur Verzeichnis-/Profilbeleg)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Häde Schrott-/Metallgroßhandel (Stamm)

## Überblick

Häde Schrott- und Metallgroßhandel ist eine Seed-Zeile, deren Betrieb und Anschrift nicht
geklärt sind; die Adresszeile Essen Heßlerstr. 77 ist formal fehlerhaft und die
Postleitzahl fehlt. Zu trennen sind mindestens drei Entitäten: die Häde Recycling GmbH mit
René Häde an der Alten Bottroper Straße 120 in Essen, deren Register- und
Umsatzsteuerfelder nur Platzhalter tragen, ein Verzeichniseintrag mit derselben Rufnummer
an der Kraienbruch 107 in Essen und ein zweiter Eintrag an der Frillendorfer Straße 148,
wo heute ein Baumarkt sitzt. Die Recherche vom 02.10.2026 belegt, dass die Heßlerstraße 77
aktuell von der Metallion GmbH betrieben wird, ohne Häde-Bezug, und die Hausnummer 77
fehlt zudem in OSM. Offen bleibt die Zuordnung, welche Entität der Seed-Datensatz
abbildet; eine Koordinate wurde nicht gesetzt, Ankaufspreise sind nicht belegt.

## Timeline

### Owner-Quellenkorrektur 27.10.2026: mehrere konkurrierende Häde-Spuren

- [Owner-Quellenkorrektur 27.10.2026: Die Identität dieser Zeile ist breiter aufgeklärt als bisher angenommen. Die Betreiberseite der Häde Recycling GmbH (GF René Häde, Alte Bottroper Straße 120, 45356 Essen) trägt als HRB- und USt-IdNr-Feld nur Platzhalter; der 11880-Eintrag „Schrotthandel + Containerdienst Häde GmbH Rene Häde“ nennt dieselbe Telefonnummer 0201 85893262, aber die Adresse Kraienbruch 107, 45357 Essen, und die dort genannte Website `schrotthandel-containerdienst.de` ist eine Domain-Parking-Seite ohne Impressum. Ein dritter 11880-Eintrag führt „Schrottgroßhandel Haede Schrottabholung Düsseldorf-Ratingen“ an Frillendorfer Straße 148, 45139 Essen, wo heute ein Baumarkt sitzt; die Nominatim-Vorwärtssuche zu Heßlerstr. 77 findet nur den Straßennamen ohne auflösbare Hausnummer. Die amtliche Anlagenliste der Bezirksregierung kennt Alte Bottroper Straße nur mit den Nummern 11-13 (Elorec GmbH, Schrottplatz und E-Geräte-Behandlung) und 124 (Paula Mertzen GmbH), nicht 120. Damit sind mindestens drei Entitäten und vier Adressen zu trennen; welche Spur zur Seed-Zeile gehört, ist offen. Der gespeicherte Punkt auf der Fernbus-Spur bleibt zurückgenommen, eine Ersatzkoordinate wird nicht gesetzt, und die formal fehlerhafte Adresszeile „Essen Heßlerstr. 77“ (Stadtname im Adressfeld, PLZ leer) bleibt als eigene offene Adressfrage stehen. Quellen: https://www.haede-recycling.de/impressum ; https://www.11880.com/branchenbuch/essen-ruhr/120674719B102132131/schrotthandel-containerdienst-haede-gmbh-rene-haede.html ; https://www.11880.com/branchenbuch/essen-ruhr/120674719B101734422/schrottgrosshandel-haede-schrottabholung-duesseldorf-ratingen.html ; https://www.brd.nrw.de/umweltinspektionsberichte/Stadt-Essen ; read-only `public.db` (27.10.2026). **Preise:** Ankauf — für keine der drei belegten Häde-Spuren ist eine Ankaufspreisliste gefunden, die Betreiberseite nennt keine Kurse; Verkauf — keine Verkaufsliste; Gebühren — keine Abhol- oder Entsorgungstarife belegt.]

### Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt zurücknehmen

- [Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt zurücknehmen: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`. Dieser Punkt liegt laut OSM auf der Straße „Freiheit“, 45128 Essen (way 46961889, `highway=service`, `bus=designated`, `vehicle=no`), genauer auf dem Fernbusbahnhof am Essener Hauptbahnhof (Haltestellenknoten 599294635, „Essen, Fernbusbahnhof“). Das ist eine Busspur und kein Gewerbestandort; in den Umweltinspektionsberichten der Bezirksregierung Düsseldorf für die Stadt Essen gibt es keinen Eintrag auf dieser Straße. Das Dossier nennt „Essen Heßlerstr. 77“; die Nominatim-Vorwärtssuche findet „Heßlerstraße, Altenessen-Nord“ (51.508429/7.011867), rund 6,47 km vom gespeicherten Punkt entfernt. Die Betreiberseite der Häde Recycling GmbH nennt dagegen Alte Bottroper Straße 120, 45356 Essen, also einen dritten Ort. Der gespeicherte Punkt widerspricht damit beiden Adressangaben zu dieser Zeile und trägt für sie keine Aussage. Offen bleiben die Identität des Seed-Datensatzes und die Frage, welche Adresse überhaupt zu ihm gehört; der HRB-Eintrag auf der Betreiberseite ist nur ein Platzhalter. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=He%C3%9Flerstra%C3%9Fe+77%2C+Essen&format=jsonv2&addressdetails=1 ; https://www.haede-recycling.de/impressum ; https://api.openstreetmap.org/api/0.6/way/46961889/full.json ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Importiert (Seed-Stand 2026-09-30)

- Heßlerstr. 77 (s. Notiz C6)

### Korrektur 30.09.2026 (Feedback-Triage)

- [Korrektur 30.09.2026 (Feedback-Triage): Feedback 1724 bestaetigt — Hesslerstr. 77 gehoert zu Roswitha Haede/Entsorgungsfachbetrieb (eigene Domains), nicht zur Ratinger Marcus-Haede-Firma; Website entfernt, status pruefung (Split-Kandidat). Quelle: haede-recycling.de.]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Split bestätigt — Heßlerstr. 77, 45329 Essen wird aktuell von Metallion GmbH betrieben (metallion.de: Tel. 0201 344344, info@metallion.de, Mo-Sa 08-20 Uhr, Schrott-/NE-Metall-Ankauf mit Abholung ab 500 kg); kein Häde-Bezug mehr an dieser Adresse; Ratinger Marcus-Häde-Firma sitzt auf Kalkumer Str. Ratingen-Lintorf (schrotthandel-haede.de, eigene Firma). Dossier bleibt pruefung als Split-Kandidat (unklar, ob Essener Betrieb je zu Häde gehörte oder Seed-Fehlzuordnung); kein Frontmatter-Fill. Quelle(n): metallion.de (Abruf 02.10.2026); schrotthandel-haede.de]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Für diese Zeile ist **keine** Adresse
  zuzuordnen, und die Dossier-Hausnummer existiert in OSM **nicht**: Das am 03.11.2026 nachgeladene Kachelraster über
  die Heßlerstraße in Altenessen-Nord (7.0000/51.5000 bis 7.0200/51.5200, **4 Kacheln von je 0,01 Grad,
  0 Kachel-Fehler**) erfasst die Straße mit **46 verschiedenen Hausnummern** (4, 6, 7, 8, 9, 10, 12, 18, 20, 23, 28, 32,
  33, 34, 36, 37, 38, 39, 40, 42, 43, 44, 45, 48, 50, 51, 52, 53, 58, 60, 63, 65, 79, 81, 94, 96, 100, 102, 104, 105, 106,
  107, 109, 111, 115, 121) - die
  **Hausnummer 77 fehlt**, und eine Suche nach der Nummer 77 in Verbindung mit „Heßlerstraße“ findet **keinen**
  Treffer; das bereits am 27.10.2026 dokumentierte Ergebnis der Nominatim-Vorwärtssuche (nur der **Straßenname** ohne
  auflösbare Hausnummer) ist damit bestätigt. Hinzu kommt, dass die Adresszeile **„Essen Heßlerstr. 77“** den
  Stadtnamen **im Adressfeld** enthält (PLZ leer) und damit formal fehlerhaft ist. **Warum keine Koordinate:** Es sind
  mindestens **drei** Entitäten und **vier** Adressen zu trennen - die Betreiberseite der **Häde Recycling GmbH** (GF
  René Häde, **Alte Bottroper Straße 120, 45356 Essen**) mit HRB- und USt-Feld nur als Platzhalter; der
  11880-Eintrag „Schrotthandel + Containerdienst Häde GmbH Rene Häde“ mit derselben Telefonnummer 0201 85893262, aber
  der Adresse **Kraienbruch 107, 45357 Essen** und der Website `schrotthandel-containerdienst.de`, die eine
  **Domain-Parking-Seite ohne Impressum** ist; und der 11880-Eintrag „Schrottgroßhandel Haede Schrottabholung
  Düsseldorf-Ratingen“ mit der Adresse **Frillendorfer Straße 148, 45139 Essen**, wo heute ein **Baumarkt** sitzt. Die
  amtliche Anlagenliste der Bezirksregierung Düsseldorf kennt **Alte Bottroper Straße** nur mit den Nummern **11-13**
  (Elorec GmbH, Schrottplatz und E-Geräte-Behandlung) und **124** (Paula Mertzen GmbH), **nicht 120**. Und die Recherche
  vom 02.10.2026 belegt, dass **Heßlerstr. 77, 45329 Essen aktuell von der Metallion GmbH** betrieben wird (metallion.de:
  Tel. 0201 344344, info@metallion.de, Mo-Sa 08-20 Uhr, Schrott-/NE-Metall-Ankauf mit Abholung ab 500 kg) - **ohne
  Häde-Bezug**; der Dossier-Standort ist damit als **Split-Kandidat** dokumentiert. Quelle 1 (Betreiber/Verzeichnis):
  https://www.haede-recycling.de/impressum ; die beiden 11880-Einträge ; https://metallion.de ;
  https://www.brd.nrw.de/umweltinspektionsberichte/Stadt-Essen . Quelle 2 (OSM): das Kachelraster sowie die
  gefundenen Adressobjekte der Heßlerstraße (live geladen am 03.11.2026, HTTP 200). **Warum keine Koordinate:** Der
  einzige zu diesem Seed notierte Adresskandidat **Heßlerstr. 77** existiert in OSM **nicht** und gehört nach der
  Recherche vom 02.10.2026 einem **anderen** Betrieb (Metallion GmbH); die übrigen Adressen (Alte Bottroper Straße 120,
  Kraienbruch 107, Frillendorfer Straße 148) gehören zu **anderen** Häde-Spuren und sind nicht diesem Seed zugeordnet.
  Jeder Punkt wäre eine Behauptung über die Lage einer nicht geklärten Entität. **Was zur Auflösung fehlt:** die
  Festlegung, **welche** der drei Häde-Entitäten der Seed-Datensatz abbildet, und deren belegte Anschrift; erst dann ist
  ein OSM-Objekt zu bestimmen. Der bisher gespeicherte Produktionspunkt 51.450284375/7.015053125 (die Fernbus-Spur
  „Freiheit“ am Essener Hauptbahnhof, laut read-only-Abgleich vom 27.10.2026) wird durch diese Notiz **nicht**
  ersetzt und bleibt zurückgenommen. `status: pruefung`, `clear_fields: geo` und alle übrigen Felder bleiben
  unverändert; eine Schließung oder Auflösung wird **nicht** behauptet. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0000,51.5000,7.0200,51.5200 ;
  https://www.haede-recycling.de/impressum ;
  https://www.11880.com/branchenbuch/essen-ruhr/120674719B102132131/schrotthandel-containerdienst-haede-gmbh-rene-haede.html ;
  https://www.11880.com/branchenbuch/essen-ruhr/120674719B101734422/schrottgrosshandel-haede-schrottabholung-duesseldorf-ratingen.html ;
  https://metallion.de ; https://www.brd.nrw.de/umweltinspektionsberichte/Stadt-Essen (Abruf 03.11.2026).]

### Geokodierung 05.11.2026

- [Geokodierung 05.11.2026: **Klärfall - keine Koordinate gesetzt; die Nachprüfung bestätigt den Vermerk vom
  03.11.2026 mit einem deutlich erweiterten Fenster und dem vollständigen Census der Heßlerstraße.** Für diese Zeile
  ist **keine** Adresse eindeutig zuzuordnen, und die Dossier-Hausnummer **77** existiert in OSM **nicht**. **Quelle 1
  (OSM, live abgefragt am 05.11.2026):** das Kachelraster **6.9800/51.4800 bis 7.0500/51.5300**, **35 Kacheln von je
  0,01 Grad**, **0 Kachel-Fehler**
  (https://api.openstreetmap.org/api/0.6/map.json?bbox=6.9800,51.4800,7.0500,51.5300) **umschließt die Objekt-BBox
  vollständig**, enthält die **„Heßlerstraße“** nachweislich und vollständig (Verlauf 51.5082321/7.0072953 bis
  51.5135547/7.0408199) und erfasst sie mit **199** Adressobjekten; sie weist **199** verschiedene Hausnummern (10, 100, 102, 104, 105, 106, 107, 109, 111, 112, 114, 115, 12, 121, 122, 125, 132, 134, 135, 136, 137, 138, 139, 140, 141, 146, 147, 148, 149, 151, 152, 154, 155, 156, 157, 158, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 18, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 196, 197, 198, 199, 20, 201, 203, 205, 207, 209, 210, 211, 212, 213, 215, 217, 219, 221, 226, 227, 229, 23, 230, 231, 232, 233, 234, 235, 236, 238, 239, 242, 243, 244, 245, 246, 247, 248, 250, 254, 261, 263, 265, 267, 273, 275a, 275b, 276, 277a, 277b, 279, 28, 280, 281, 282, 283, 284, 285, 286, 287, 289, 290, 292, 293, 294, 295, 296, 297, 298, 299, 301, 301a, 302, 303, 304, 305, 307, 309, 314, 32, 325, 327, 329, 33, 331, 333, 334, 335, 337, 339, 34, 341, 343, 345, 347, 349, 351, 353, 356, 357, 36, 361, 37, 38, 39, 4, 40, 42, 43, 44, 45, 48, 50, 51, 52, 53, 58, 6, 60, 63, 65, 7, 79, 8, 81, 9, 94, 96) - die **Hausnummer 77 fehlt**, die belegten Nummern springen von **65**
  direkt auf **79**; eine Suche nach der Nummer 77 in Verbindung mit „Heßler" findet **keinen** Treffer. Das erweiterte
  Raster bestätigt damit den Befund vom 03.11.2026 (dort 46 Nummern in einem deutlich kleineren Fenster) und schließt
  einen Fensterfehler der letzten Welle **aus**. Hinzu kommt, dass die Adresszeile **„Essen Heßlerstr. 77"** den
  Stadtnamen **im Adressfeld** enthält (PLZ leer) und damit formal fehlerhaft ist. **Quelle 2 (Betreiber/Register, im
  Dossier bereits zitiert):** es sind mindestens **drei** Entitäten und **vier** Adressen zu trennen - die
  Betreiberseite der **Häde Recycling GmbH** (GF René Häde, **Alte Bottroper Straße 120, 45356 Essen**) mit
  HRB- und USt-Feld nur als Platzhalter; der 11880-Eintrag „Schrotthandel + Containerdienst Häde GmbH Rene Häde" mit
  derselben Telefonnummer 0201 85893262, aber der Adresse **Kraienbruch 107, 45357 Essen** und der Website
  `schrotthandel-containerdienst.de`, die eine **Domain-Parking-Seite ohne Impressum** ist; und der 11880-Eintrag
  „Schrottgroßhandel Haede Schrottabholung Düsseldorf-Ratingen" mit der Adresse **Frillendorfer Straße 148, 45139
  Essen**, wo heute ein **Baumarkt** sitzt. Die amtliche Anlagenliste der Bezirksregierung Düsseldorf kennt **Alte
  Bottroper Straße** nur mit den Nummern **11-13** (Elorec GmbH, Schrottplatz und E-Geräte-Behandlung) und **124**
  (Paula Mertzen GmbH), **nicht 120**. Die Recherche vom 02.10.2026 belegt, dass **Heßlerstr. 77, 45329 Essen aktuell
  von der Metallion GmbH** betrieben wird (metallion.de: Tel. 0201 344344, info@metallion.de, Mo-Sa 08-20 Uhr,
  Schrott-/NE-Metall-Ankauf mit Abholung ab 500 kg) - **ohne Häde-Bezug**. **Warum keine Koordinate:** Der einzige zu
  diesem Seed notierte Adresskandidat **Heßlerstr. 77** existiert in OSM **nicht** und gehört nach der Recherche vom
  02.10.2026 einem **anderen** Betrieb (Metallion GmbH); die übrigen Adressen (Alte Bottroper Straße 120, Kraienbruch
  107, Frillendorfer Straße 148) gehören zu **anderen** Häde-Spuren und sind nicht diesem Seed zugeordnet. Jeder Punkt
  wäre eine Behauptung über die Lage einer nicht geklärten Entität. **Was zur Auflösung fehlt:** die Festlegung,
  **welche** der drei Häde-Entitäten der Seed-Datensatz abbildet, und deren belegte Anschrift; erst dann ist ein
  OSM-Objekt zu bestimmen. `status: pruefung`, `clear_fields: 'geo'` und alle übrigen Felder bleiben unverändert; eine
  Schließung oder Auflösung wird **nicht** behauptet. **Reichweitenklausel:** das Fehlen einer Hausnummer in OSM ist
  eine Datenlücke und **kein** Existenz- oder Schließungsbeweis. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=6.9800,51.4800,7.0500,51.5300 ;
  https://www.haede-recycling.de/impressum ; https://www.11880.com/branchenbuch/essen-ruhr/120674719B102132131/schrotthandel-containerdienst-haede-gmbh-rene-haede.html ;
  https://www.11880.com/branchenbuch/essen-ruhr/120674719B101734422/schrottgrosshandel-haede-schrottabholung-duesseldorf-ratingen.html ;
  https://metallion.de ; https://www.brd.nrw.de/umweltinspektionsberichte/Stadt-Essen (im Dossier zitiert, Abruf
  05.11.2026).]

### Recherche 08.11.2026

- [Recherche 08.11.2026: **Die Seed-Straßenzeile „Essen Heßlerstr. 77“ bleibt unverändert: die Anschrift ist nachweislich einem anderen Betrieb zugeordnet, und die Zuordnung des Seed-Datensatzes zu einer der drei Häde-Spuren bleibt offen.** (1) Verzeichniseintrag, live am 08.11.2026: die Essener Stadtseite des Schrottfachportals schrottradar.de führt „Häde Schrott- und Metallgroßhandel Containerdienst“ mit „Heßlerstraße 77, 45329 Stadtbezirke V Essen“ und der Rufnummer +49 201 344344. (2) Die zweite unabhängige Quelle widerspricht dem Betriebsnamen: das Impressum der metallion GmbH nennt wörtlich „Heßlerstr. 77, 45329 Essen“, Telefon 0201 344 344, info@metallion.de, HRB 34136, USt-IdNr. DE339072538. Die Nummer des Verzeichniseintrags ist also die Nummer der metallion GmbH und nicht die der Häde Recycling GmbH, deren Betreiberseite haede-recycling.de „Alte Bottroper Straße 120, 45356 Essen“ mit der Nummer 0201 85893262 nennt; der 11880-Eintrag „Häde Schrotthandel Recycling GmbH“ führt dieselbe Anschrift mit derselben Nummer. **Falschzuordnung, ausdrücklich benannt:** die Seed-Anschrift Heßlerstr. 77 gehört nach dem Betreiberimpressum der metallion GmbH und trägt keinen Häde-Bezug, wie bereits der Vermerk vom 02.10.2026 festhält; die Hausnummer 77 fehlt zudem in der OSM-Hausnummernliste der Heßlerstraße (Vermerk vom 03.11.2026). **Offen bleibt:** welche der drei Häde-Entitäten mit den vier Adressen Alte Bottroper Straße 120, Kraienbruch 107, Frillendorfer Straße 148 und Heßlerstr. 77 der Seed-Datensatz abbildet; eine belegte Anschrift für diesen Datensatz gibt es damit nicht, `street` und `postcode` bleiben unverändert, `clear_fields` mit dem Token geo und `status: pruefung` bleiben unverändert, es wird keine Koordinate gesetzt. Quelle(n): https://schrottradar.de/nordrhein-westfalen/essen/ ; https://metallion.de/impressum ; https://www.haede-recycling.de/impressum ; https://www.11880.com/branchenbuch/essen-ruhr/120674719B112579630/haede-schrotthandel-recycling-gmbh.html (Abruf 08.11.2026).]
