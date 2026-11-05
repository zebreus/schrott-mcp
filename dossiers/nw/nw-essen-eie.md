---
slug: nw-essen-eie
clear_fields: geo
name: EIE
trader_type: schrotthaendler
state: NW
city: Essen
street: Gelsenkirchener Str. 181
postcode: '45309'
phone: 0201 21962114
email: ''
opening_hours: ''
website: ''
website_status: tot
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# EIE

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Owner-Quellenkorrektur 27.10.2026: Zwei-Quellen-Beleg der Adresse

- [Owner-Quellenkorrektur 27.10.2026: Die Adresse Gelsenkirchener Str. 181, 45309 Essen ist inzwischen zweifach unabhängig belegt: Das Örtliche führt EIE Services GmbH an dieser Anschrift mit der Telefonnummer 0201 21962114 und der Branche „Schrott in Essen“, der 11880-Eintrag vom 09.10.2026 wiederholt Adresse und Nummer. Die im Verzeichnis genannte Website `deinschrottplatz.de` ist **kein** EIE-Betreiberportal: Das archivierte Impressum vom 26.02.2021 gehört der ScrapBees GmbH, Platz der Ideen 2, 40476 Düsseldorf, HRB 91074, und die Domain ist heute nicht mehr auflösbar; `website_status: tot` bleibt damit zutreffend. Zugleich fällt die Nominatim-Vorwärtssuche auf Museumsgebäude (Ruhr Museum, Red Dot Design Museum, Schalthaus 2), die Hausnummer 181 trägt also keine erkennbare Schrotthof-Situation; die Adresszeile selbst bleibt deshalb als eigene offene Frage stehen. Die GEO-Zurücknahme vom selben Tag bleibt davon unberührt, eine Ersatzkoordinate wird nicht gesetzt. Quellen: https://www.dasoertliche.de/Themen/EIE-Services-GmbH-Essen-Schonnebeck-Gelsenkirchener-Str ; https://www.11880.com/branchenbuch/essen-ruhr/060440090B106476017/eie-services-gmbh.html ; http://web.archive.org/web/20210226092137/https://www.deinschrottplatz.de/impressum.php ; read-only `public.db` (27.10.2026). **Preise:** Ankauf — keine EIE-Preisliste gefunden, das Verzeichnisprofil nennt nur die Branche „Schrott in Essen“; Verkauf — keine Liste; Gebühren — keine Tarife belegt.]

### Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt zurücknehmen

- [Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt zurücknehmen: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`. Dieser Punkt liegt laut OSM auf der Straße „Freiheit“, 45128 Essen (way 46961889, `highway=service`, `bus=designated`, `vehicle=no`), genauer auf dem Fernbusbahnhof am Essener Hauptbahnhof (Haltestellenknoten 599294635, „Essen, Fernbusbahnhof“). Das ist eine Busspur und kein Gewerbestandort; in den Umweltinspektionsberichten der Bezirksregierung Düsseldorf für die Stadt Essen gibt es keinen Eintrag auf dieser Straße. Das Dossier nennt Gelsenkirchener Str. 181, 45309; die Nominatim-Vorwärtssuche trifft „Ruhr Museum, 181, Gelsenkirchener Straße, Stoppenberg“ (51.485864/7.042187), rund 4,38 km vom gespeicherten Punkt entfernt. Der gespeicherte Punkt widerspricht damit der eigenen Adressangabe dieser Zeile. Offen bleibt die Adresszeile selbst, weil der Vorwärtstreffer auf ein Museum fällt und weder Betreiber-Impressum noch Register- oder Behördenbeleg für EIE Services GmbH vorliegen. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Gelsenkirchener+Str.+181%2C+45309+Essen&format=jsonv2&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/way/46961889/full.json ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 02.10.2026

- [Recherche 02.10.2026: OK (Ausnahmefall, offen dokumentiert — kein Betreiber-/Register-Beleg, aber 4 konsistente Aggregatoren): EIE = EIE Services GmbH, Gelsenkirchener Str. 181, 45309 Essen-Schonnebeck, Tel. 0201 21962114, Kategorie Schrott/Alteisen. Belegt uebereinstimmend durch DasTelefonbuch (Adresse+Tel.+Oeffnungszeiten Mo-Do 8-16/Fr 8-14, Mail eie@deinschrottplatz.de), Gelbe Seiten, Das Oertliche, cylex und city-advisor (alle Gelsenkirchener Str. 181). Eingetragene Homepage www.deinschrottplatz.de ist TOT (DNS-Aufloesung scheitert 02.10.2026, curl exit 6) → website leer, website_status tot. Mail-Adresse wegen toter Domain NICHT ins Frontmatter uebernommen. Expliziter Ankaufbeleg fehlt (nur Branchenkategorie) → Status bleibt pruefung; Koordinaten neu zu geocodieren (Adress-Fill). Quelle(n): DasTelefonbuch-Eintrag + Gelbe Seiten + Das Oertliche + cylex + city-advisor (Abruf 02.10.2026).]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Essen

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Hausnummer **181** existiert auf der
  **„Gelsenkirchener Straße“** zwar in OSM, trägt dort aber **keinen** Bezug zum Seed und **keinen** Schrottbetrieb:
  das am 03.11.2026 nachgeladene Kachelraster über den Stoppenberger/Katernberger Abschnitt (7.0300/51.4800 bis
  7.0500/51.5000, **6 Kacheln von je 0,01 Grad, 0 Kachel-Fehler**) erfasst die Straße mit **82 verschiedenen
  Hausnummern** (55, 66, 67, 68, 70, 71, 72, 73, 74, 76, 77, 78, 79, 80, 81, 81a, 82, 83, 84, 85, 86, 87, 88, 89a, 90, 92, 98,
  100, 122, 124, 128, 130, 138, 138a, 140, 148, 150, 152, 160, 162, 164, 166, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177,
  178, 179, 180, 181, 181a, 182, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 198, 199, 200, 201, 203, 205, 209,
  247, 249, 253, 255) und die Nummer **181** mit **zwölf** Objekten, die
  sämtlich **Museums-, Gastronomie- und Bürozwecke** haben: node/621131210 (`name`=Ruhr Museum),
  node/5614986047 (`name`=Ruhr Museum), node/304633128 (`name`=Red Dot Design Museum),
  node/304633556 (`name`=Casino Zollverein, `amenity`=restaurant), node/692822627 (`amenity`=cafe),
  node/4318457806 (`name`=Zollvereinführung, `tourism`=information), node/13955614137 (`name`=Schmiede Zollverein),
  node/413349095 (`amenity`=fast_food) sowie weitere Objekte desselben Komplexes - die Nummer 181 bezeichnet damit den
  **Zollverein-Museumskomplex** (UNESCO-Welterbe Zollverein) und **nicht** eine Schrotthof-Situation; ein Objekt mit
  Betriebsbezug zur EIE Services GmbH existiert **nicht**. Das bestätigt und verschärft die bereits am 27.10.2026
  dokumentierte Beobachtung, dass die Nominatim-Vorwärtssuche auf Museumsgebäude (Ruhr Museum, Red Dot Design Museum,
  Schalthaus 2) fällt. Quelle 1 (Verzeichnis): Das Örtliche führt **EIE Services GmbH** an Gelsenkirchener Str. 181,
  45309 Essen mit der Telefonnummer 0201 21962114 und der Branche „Schrott in Essen“, der 11880-Eintrag vom 09.10.2026
  wiederholt Adresse und Nummer - die Adresse ist damit **zweifach** belegt, die **Betriebsidentität** an dieser
  Anschrift bleibt jedoch unbestätigt. Quelle 2 (OSM): das Kachelraster sowie die zwölf Objekte der Hausnummer 181
  (live geladen am 03.11.2026, HTTP 200). **Warum keine Koordinate:** Jeder Punkt auf einem der genannten Objekte
  würde das **Museumsgebäude** als Standort eines Schrotthändlers ausweisen und wäre eine Behauptung, die keine Quelle
  trägt; die im Dossier ausdrücklich offene Frage, ob die Verzeichnisadresse überhaupt den Betrieb bezeichnet, wäre
  damit beantwortet, ohne belegt zu sein. **Was zur Auflösung fehlt:** ein Betreiber- oder Registerbeleg, der die
  EIE Services GmbH (oder einen Nachfolger) an der Gelsenkirchener Straße 181 belegt, oder eine andere belegte
  Anschrift; die verwendete Domain `deinschrottplatz.de` ist **tot** und das archivierte Impressum vom 26.02.2021
  gehörte der **ScrapBees GmbH, Platz der Ideen 2, 40476 Düsseldorf** (HRB 91074) - einer **anderen** Gesellschaft, die
  ausdrücklich **nicht** zu diesem Seed gehört. Der bisher gespeicherte Produktionspunkt 51.450284375/7.015053125
  (die Fernbus-Spur „Freiheit“ am Essener Hauptbahnhof, laut read-only-Abgleich vom 27.10.2026) wird durch diese Notiz
  **nicht** ersetzt und bleibt zurückgenommen. `status: pruefung`, `clear_fields: geo`, `website_status: tot` und alle
  übrigen Felder bleiben unverändert. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0300,51.4800,7.0500,51.5000 ;
  https://www.dasoertliche.de/Themen/EIE-Services-GmbH-Essen-Schonnebeck-Gelsenkirchener-Str ;
  https://www.11880.com/branchenbuch/essen-ruhr/060440090B106476017/eie-services-gmbh.html ;
  http://web.archive.org/web/20210226092137/https://www.deinschrottplatz.de/impressum.php (Abruf 03.11.2026).]

### Geokodierung 05.11.2026

- [Geokodierung 05.11.2026: **Klärfall - keine Koordinate gesetzt; die Nachprüfung bestätigt den Vermerk vom
  03.11.2026 mit dem vollständigen Census der Gelsenkirchener Straße.** Die Dossier-Hausnummer **181** existiert zwar
  in OSM, die **Betriebsidentität** an dieser Anschrift aber **nicht**. **Quelle 1 (OSM, live abgefragt am 05.11.2026):**
  das Kachelraster **7.0200/51.4600 bis 7.0700/51.5100**, **25 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**
  (https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0200,51.4600,7.0700,51.5100) **umschließt die Objekt-BBox
  vollständig** und enthält die **„Gelsenkirchener Straße“** nachweislich und vollständig (Verlauf 51.4758599/7.0373576
  bis 51.4986507/7.0575690); sie trägt **216** Adressobjekte; sie weist **216** verschiedene Hausnummern (1, 10, 100, 12, 122, 124, 128, 13, 130, 138, 138a, 14, 140, 148, 15, 150, 152, 16, 160, 162, 164, 166, 168, 169, 17, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 18, 180, 181, 181a, 182, 184, 185, 186, 187, 188, 189, 19, 190, 191, 192, 193, 194, 195, 196, 198, 199, 2, 20, 200, 201, 203, 205, 209, 21, 22, 220, 23, 244, 246, 247, 248, 249, 250, 252, 252a, 253, 255, 258, 259, 26, 260, 262, 263, 264, 264a, 264b, 264c, 265, 266, 267, 268, 268b, 268c, 269, 270, 272, 272a, 274, 275, 277, 278, 279, 28, 281, 282, 284, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 298, 3, 30, 300, 301, 301a, 301b, 305, 307, 308, 309, 31, 313, 315, 318, 319, 32, 320, 322, 324, 325, 326, 328, 33, 337, 339, 34, 341, 343, 345, 347, 349, 35, 351, 353, 355, 357, 359, 36, 361, 363, 365, 367, 369, 37, 371, 373, 375, 377, 379, 38, 381, 39, 4, 40, 41, 42, 43, 45, 47, 48, 49, 5, 5-7, 50, 51, 52, 52-54, 53, 55, 66, 67, 68, 7, 70, 71, 72, 73, 74, 76, 77, 78, 79, 8, 80, 81, 81a, 82, 83, 84, 85, 86, 87, 88, 89a, 90, 92, 98) - die
  **Hausnummer 181 existiert**, sie bezeichnet aber den **Zollverein-Museumskomplex**: **13** Objekte tragen sie
  (node/304633128 „Red Dot Design Museum", `tourism`=museum; node/304633556 „Casino Zollverein", `amenity`=restaurant;
  node/413349095 „Schacht XII, Bistro", `amenity`=fast_food; node/621131210 „Ruhr Museum", `tourism`=museum;
  node/692822627 „Cafe Kohlenwäsche", `amenity`=cafe; node/4318457806 „Zollvereinführung", `tourism`=information;
  node/5614986047 „Ruhr Museum"; node/13955614137 „Schmiede Zollverein"; node/13955631202 „Digital Campus Zollverein";
  node/13955631203 „Orbit", `operator`=Folkwang Universität der Künste, `amenity`=arts_centre; node/413352778 ohne
  Namen zur Nummer 181a; way/32394160 „Schalthaus 2"; way/60100420 „Schalthaus 2.0") - **kein** Objekt mit Bezug zur
  EIE Services GmbH. Eine Suche nach „EIE" über die Felder name, operator, brand, description, website,
  contact:website, note und addr:housename findet im Raster **keinen** Treffer (die Treffer „Gleisdreieck" und
  „Windweier" sind reine Straßennamen) - das ist eine Aussage zum OSM-Bestand, **kein** Negativbeleg über den Betrieb.
  **Quelle 2 (Verzeichnis, im Dossier bereits zitiert):** Das Örtliche führt **EIE Services GmbH** an Gelsenkirchener
  Str. 181, 45309 Essen mit der Telefonnumber 0201 21962114 und der Branche „Schrott in Essen", der 11880-Eintrag vom
  09.10.2026 wiederholt Adresse und Nummer - die Adresse ist damit **zweifach** belegt, die **Betriebsidentität** an
  dieser Anschrift bleibt jedoch unbestätigt; die im Dossier dokumentierte Zuordnung der Domain `deinschrottplatz.de`
  zur ScrapBees GmbH (Platz der Ideen 2, 40476 Düsseldorf) schließt eine Gleichsetzung aus. **Warum keine Koordinate:**
  Jeder Punkt auf einem der genannten Objekte würde das **Museumsgebäude** als Standort eines Schrotthändlers ausweisen
  und wäre eine Behauptung, die keine Quelle trägt; die offene Frage, ob die Verzeichnisadresse überhaupt den Betrieb
  bezeichnet, wäre damit unbeantwortet beantwortet. **Was zur Auflösung fehlt:** ein Betreiber- oder Registerbeleg, der
  die EIE Services GmbH (oder einen Nachfolger) an der Gelsenkirchener Straße 181 belegt, oder eine andere belegte
  Anschrift. `status: pruefung`, `clear_fields: 'geo'`, `website_status: tot` und alle übrigen Felder bleiben
  unverändert. **Reichweitenklausel:** das Fehlen eines OSM-Betriebsbezugs oder einer Hausnummer wäre kein Existenz-
  oder Schließungsbeweis. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0200,51.4600,7.0700,51.5100 ;
  https://www.dasoertliche.de/Themen/EIE-Services-GmbH-Essen-Schonnebeck-Gelsenkirchener-Str ;
  https://www.11880.com/branchenbuch/essen-ruhr/060440090B106476017/eie-services-gmbh.html ;
  http://web.archive.org/web/20210226092137/https://www.deinschrottplatz.de/impressum.php (im Dossier zitiert, Abruf
  05.11.2026).]
