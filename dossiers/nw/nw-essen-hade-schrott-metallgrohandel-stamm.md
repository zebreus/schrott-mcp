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

### Recherche 24.11.2026 (Welle 2000 A)

- [Recherche 24.11.2026: **Der Seed-Datensatz lässt sich jetzt einem konkreten, im Register belegten Rechtsträger zuordnen: der Häde Recycling GmbH, AG Essen, HRB 33618, Sitz Alte Bottroper Straße 120, 45356 Essen - und genau diese Adresse ist die einzige belegte Betriebsanschrift; die Seed-Zeile „Heßlerstr. 77“ ist nachweislich falsch.** Das amtliche Registerportal (online-handelsregister.de, live am 25.11.2026) liefert für die Suche `Häde Recycling` genau **einen** einschlägigen Treffer: **Häde Recycling GmbH, Essen, HRB 33618**. Der Abzug (http://www.online-handelsregister.de/handelsregisterauszug/nw/Essen/HRB/33618/Haede-Recycling-GmbH) weist aus: `Sie kann schriftlich über die Firmenadresse Alte Bottroper Straße 120, 45356 Essen erreicht werden. Ihr Status wird als aktiv geführt. Die Firma wurde am 14.06.2022 gegründet bzw. in das Handelsregister beim Amtsgericht Essen eingetragen.` mit `Rechtsform Gesellschaft mit beschränkter Haftung`; es liegen **1 Registerbekanntmachung** vor, die letzte Änderung ist vom **14.06.2022**. Die Namensvarianten `Haede Recycling Essen` und `Häde Recycling Essen` liefern **keine** Suchergebnisse, `Häde` allein liefert ausschließlich **Namensvetter ohne Bezug** (u. a. HADEG Recycling GmbH Tostedt, HADEKA Genossenschaft, Hader GmbH Würzburg, Hadena GmbH Mannheim). **Zweite unabhängige Bestätigung der Anschrift:** die Betreiberseite der Häde Recycling GmbH (https://www.haede-recycling.de/impressum, live am 25.11.2026) nennt `Häde Recycling GmbH`, `Alte Bottroper Straße 120, 45356 Essen`, `Telefon: 0201 85893262`, `E-Mail: office@haede-recycling.de`, `Vertreten durch: Geschäftsführer: René Häde`, `Registereintrag Handelsregister: Amtsgericht Essen` - die **HRB-Nummer ist dort weiterhin ein Platzhalter** (`[Hier deine HRB-Nummer eintragen]`) und die **USt-IdNr. ebenfalls** (`[Hier deine USt-IdNr. eintragen]`); das Register ist hier die **einzige** Quelle für die Registernummer, die Betreiberseite die einzige für die USt-Zuordnung. Die Startseite (https://www.haede-recycling.de/) nennt `Häde Recycling, Entsorgungsfachbetrieb für Containerdienst, Schrotthandel & Demontage`, `Rufen Sie uns an: +4920185893262`, `Containerstellung innerhalb 24h`, `Zertifizierter Entsorgungsfachbetrieb`, `Öffnungszeiten: Montags - Freitags: 8:00 - 16:00 Uhr`, `Für Gewerbe, Industrie und Rückbauprojekte`, `B2B-Fokus: Gewerbe, Industrie, Rückbau Deutschlandweite Logistik`. Die Startseite nennt als Sorten `Stahl & Eisen, Edelstahl (V2A / V4A), Aluminium, Kupfer, Messing, Zink, Blei, Kabelschrott, Mischschrott, Maschinen- und Anlagenteile, Produktionsreste aus Metall (Weitere Sorten auf Anfrage)` und zu den Preisen wörtlich `Schrott- & Metallpreise anfragen. Die Preise für Schrott und Metalle richten sich nach Sorte, Qualität, Menge und Marktpreis. So erhalten Sie ein Angebot: Senden Sie uns Ihre Anfrage mit Materialart, Menge und Standort. Gerne nennen wir Ihnen tagesaktuelle Preise oder erstellen ein individuelles Angebot für Abholung, Verladung und Verwertung.` **ANKAUF:** **keine** bezifferte Ankaufspreisliste und **kein** €/kg-Kurs; die Sortenliste ist eine **Annahmebereitschaftsliste**, die Preisaussage ist eine **Angebotsaufforderung**. **VERKAUF:** keine Verkaufspreisliste; die Seite verkauft Schrott und Metalle **nicht**, sondern kauft an und verwertet. **GEBÜHREN:** die **AGB & Datenschutz**-Seite (https://www.haede-recycling.de/agb---datenschutz, live am 25.11.2026) liefert erstmals einen **belastbaren Gebührent Tarifrahmen**: `Alle Preise verstehen sich netto zuzüglich der jeweils geltenden gesetzlichen Umsatzsteuer`, `Entsorgungs-, Transport-, Container-, Standzeit-, Personal- und Maschinenkosten sind, sofern nicht ausdrücklich anders vereinbart, sofort ohne Abzug zur Zahlung fällig`, `Die Preise für Schrott und Metalle richten sich nach Sorte, Qualität, Menge und Marktpreis` (Ziffer 5: `Die Vergütung für Schrott- und Metallmaterialien erfolgt preisabhängig nach Gewicht, Sorte und Qualität. Maßgeblich sind ausschließlich unsere Verwiegung sowie unsere Sorten- und Qualitätsbewertung`), Ziffer 6: `Bei Fehlbefüllung, Vermischung mit nicht vereinbarten Stoffen oder Einwurf von Fremdstoffen sind wir berechtigt, das Material umzusortieren, gesondert zu entsorgen oder abzulehnen. Alle hierdurch entstehenden Mehrkosten trägt der Auftraggeber`, Ziffer 7: `Ergibt sich nach Verwertung, dass der Materialwert (Schrott- oder Metallwert) die anfallenden Transport-, Handling- oder Entsorgungskosten nicht erreicht oder unterschreitet, besteht kein Vergütungsanspruch des Auftraggebers. Entstehende Mehrkosten können dem Auftraggeber in Rechnung gestellt werden`, Ziffer 8: `Die Abrechnung über vergütete Schrott- und Metallmengen erfolgt ausschließlich durch eine von uns erstellte Gutschrift`, Ziffer 9: `Wir behalten uns vor, für bereitgestellte Container eine Containermiete bzw. Standzeitkosten zu berechnen`, Ziffer 10: `Mehrfache Anfahrten, Leerfahrten sowie Warte- und Stillstandzeiten ... können gesondert berechnet werden`. Die AGB gelten ausdrücklich nur gegenüber Unternehmern (`Verbraucher im Sinne des § 13 BGB werden nicht beliefert`) - das ist eine **B2B-Beschränkung** und kein Beleg für eine Privatkundenannahme. **Die Seed-Anschrift ist falsch zugeordnet:** die Betreiberseite nennt `Alte Bottroper Straße 120` und `Alte Bottroper Straße 12` (im Fließtext des Impressums), **nicht** `Heßlerstr. 77`; die 11880-Zeile `Häde Schrotthandel Recycling GmbH` (live am 25.11.2026, http://www.11880.com/branchenbuch/essen-ruhr/120674719B112579630/haede-schrotthandel-recycling-gmbh.html) führt `Alte Bottroper Str. 120, 45356 Essen (Bergeborbeck)`, `(0201) 85893262`, Branchen `Schrotthandel` und `Altmetallrecycling` und **ohne** Öffnungszeiten - eine zweite unabhängige Bestätigung derselben Anschrift. Die Zuordnung `Heßlerstr. 77` zur **Metallion GmbH** (Impressum: `Heßlerstr. 77, 45329 Essen`, `HRB 34136`, `info@metallion.de`) bleibt bestehen, und die Hausnummer 77 fehlt in OSM. **Eine vierte Häde-Spur ist neu zu dokumentieren:** das Verzeichnis schrottplatz-info.de führt `Häde N. SchrottMetallhandel`, **Am Stadthafen 18, 45356 Essen**, Tel. (0201) 80 67 63 39 (live am 25.11.2026); das OSM-Adressobjekt **way/162742930** trägt `addr:housenumber`=18, `addr:street`=Am Stadthafen, `addr:city`=Essen, `addr:postcode`=45356 (Punkt 51.5007419/6.9715967) und **weder** `name=` **noch** `operator=`; die Nummer `0201 8067639` ist eine **andere** als die der Häde Recycling GmbH, die Spur ist also **nicht** zugeordnet. **Amtliche Gegenprobe (live am 25.11.2026, beide Routen, Filter `Art des Betriebs = Egal` is=1 und e=1):** die Namenssuche `Häde` liefert in der Route `Entsorgungsfachbetriebe` **3** Treffer, darunter **Häde Recycling GmbH, Alte Bottroper Straße 120, 45356 Essen, ZZET00700321 / 1, Handeln, Makeln** - die **Efb-Anerkennung der Häde Recycling GmbH an der bestätigten Anschrift ist damit amtlich belegt**; die beiden anderen Treffer sind fremde Rechtsträger (`Wieker Spezialunternehmen für Ölschädenbeseitigung GmbH, An der Autobahn 15, 27798 Hude` und `Fritz Häder KfZ-Meisterbetrieb Bergungs- und Abschleppdienst, Am Schützenplatz 3, 06796 Sandersdorf-Brehna OT Brehna`); die Namenssuche `Haede` liefert **0** Treffer, die Route `Altfahrzeugverwertung` bei `Häde` **0** Treffer. Die Standortabfrage mit **3 km** um `Alte Bottroper Straße 120, 45356 Essen` liefert **12** Efb-Zeilen und **1** AltfahrzeugV-Zeile (`Franz Maag Autoverwertung e.K., Alte Bottroper Straße 46, 45143 Essen, ZAAS00300017 / 1`, 0,7 km), darunter `Entsorgungsbetriebe Essen GmbH` (Pferdebahnstraße 32), `REMEX GmbH Essen Hövelstraße` (Hövelstr. 200), `Evonik Operations GmbH` (Rellinghauser Straße 1-11) und `elorec GmbH` (Alte Bottroper Straße 11-13, ZZET00700015 / 1 bis / 4). **Reichweite:** die Efb-Anerkennung nach § 56 KrWG ist **freiwillig**, ein Register-Nichteintrag wäre kein Nachweis; hier liegt jedoch ein **positiver** Efb-Treffer vor. Die amtliche Anlagenliste der Bezirksregierung Düsseldorf für die Stadt Essen (https://www.brd.nrw.de/umweltinspektionsberichte/Stadt-Essen, live am 25.11.2026) kennt `Alte Bottroper Str. 11-13` mit `Elorec GmbH` (`Schrottplatz, Sortierung und Behandlung von E-Geräten`, Abfallstromkontrolle 21.03.2025) und `Alte Bottroper Str. 124` (Paula Mertzen) - **die Nummer 120 ist dort nicht verzeichnet**; das ist eine Aussage zum amtlichen Anlagenverzeichnis und **kein** Negativbeleg gegen die Häde Recycling GmbH. **GEO:** die Seed-Zeile `Essen Heßlerstr. 77` ist formal fehlerhaft und sachlich falsch; das OSM-Adressobjekt zur belegten Betriebsanschrift existiert adressgenau als **way/27615807** (`addr:housenumber`=120, `addr:street`=Alte Bottroper Straße, `addr:city`=Essen, `addr:postcode`=45356, `addr:suburb`=Bergeborbeck, `building`=yes; Punkt 51.4923050/6.9521912) und trägt **weder** `name=` **noch** `operator=`. Es wird **keine** Koordinate gesetzt, das Frontmatter bleibt unverändert und der Bestandspunkt **nicht** angetastet; die Entscheidung über `street`/`postcode` und einen Standortpunkt liegt beim Owner. **Offen für den Owner:** die Zuordnung des **Seed-Datensatzes** zur Häde Recycling GmbH ist sachlich naheliegend (Name, Stadt, Branche), aber der Seed nennt die **falsche** Straße; vor einem Frontmatter-Fill sollte der Owner entscheiden, ob der Datensatz die **Häde Recycling GmbH (HRB 33618)** beschreiben soll, und dann `street`/`postcode` auf `Alte Bottroper Straße 120`/`45356` setzen; die abweichenden Spuren `Kraienbruch 107` (11880, andere Nummer), `Frillendorfer Straße 148` (Baumarkt) und `Am Stadthafen 18` (andere Nummer) sind zu klären oder zu verwerfen. Quelle(n): https://www.online-handelsregister.de/handelsregisterauszug/nw/Essen/HRB/33618/Haede-Recycling-GmbH ; https://www.online-handelsregister.de/suche/H%C3%A4de+Recycling ; https://www.haede-recycling.de/impressum ; https://www.haede-recycling.de/ ; https://www.haede-recycling.de/agb---datenschutz ; https://www.11880.com/branchenbuch/essen-ruhr/120674719B112579630/haede-schrotthandel-recycling-gmbh.html ; https://www.schrottplatz-info.de/schrottplatz/Essen/Haede-N-SchrottMetallhandel ; https://api.openstreetmap.org/api/0.6/way/27615807/full.json ; https://api.openstreetmap.org/api/0.6/way/162742930/full.json ; https://www.brd.nrw.de/umweltinspektionsberichte/Stadt-Essen ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=H%C3%A4de ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Alte%20Bottroper%20Stra%C3%9Fe%20120%2C%2045356%20Essen%2C%20Deutschland;6.9521912;51.492305;6.9641912;51.504305;6.9401912;51.480305&dst=3&is=1&e=1 (Abruf 25.11.2026).]
