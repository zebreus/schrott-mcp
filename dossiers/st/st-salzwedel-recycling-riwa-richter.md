---
slug: st-salzwedel-recycling-riwa-richter
lat: 52.8529569
lon: 11.1823088
name: Recycling RiWa (Richter)
trader_type: schrotthaendler
state: ST
city: Salzwedel
street: An der Ritzer Brücke 3
postcode: '29410'
phone: 03901 308868
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Metallrecyclinghof und Schrottplatz in Salzwedel. Das amtliche Abfallwirtschaftskonzept des Landkreises Altmarkkreis Salzwedel (Fortschreibung 2022, Kap. 14.1.6 „Schrotthändler") führt die Anlage An der Ritzer Brücke 3 unter dem Betreibernamen „Metall Recycling SAW"; die Creditreform-Firmenauskunft nennt den Gewerbebetrieb „Michael Richter Metallrecycling SAW" mit Unternehmensgegenstand „Recycling und Handel mit metallischen Altmaterialien, Autorecycling" und Hauptbranche Großhandel mit Altmaterialien und Reststoffen (46.87.0). Verzeichnisnamen: „Recycling RiWa Richter Michael", „MR Metall Recycling SAW". Eine Betreiberwebsite ist nicht auffindbar; die Telefonnummer stammt aus der Firmenauskunft und mehreren voneinander unabhängigen Verzeichnissen. Preise, Annahmebedingungen und Gebühren sind nicht veröffentlicht.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Recycling RiWa (Richter)

## Überblick

Die Typfrage ist mit einer **amtlichen Quelle** entschieden: Das **Abfallwirtschaftskonzept des Landkreises Altmarkkreis Salzwedel** (Fortschreibung 2022, heruntergeladen und im Volltext ausgewertet) führt in Kapitel **14.1.6 „Schrotthändler"** unter Nr. 27 den Standort **„An der Ritzer Brücke 3, 29410 Salzwedel"** mit dem Betreiber **„Metall Recycling SAW"**. Der Landkreis führt den Betrieb also selbst als Schrotthändler. Die **Creditreform-Firmenauskunft** zum Gewerbebetrieb **„Michael Richter Metallrecycling SAW"** an derselben Anschrift nennt als Unternehmensgegenstand „Recycling und Handel mit **metallischen Altmaterialien**, Autorecycling", Hauptbranche „Großhandel mit Altmaterialien und Reststoffen" (Branchencode 46.87.0), einen Inhaber und die Telefonnummer +49 3901 308868.

**Warum „sonstige" falsch war:** Die Beleglage war schon in der Seed-Zeile unvollständig und der Typ blieb auf dem Generikumer `sonstige` stehen, obwohl zwei Quellen — das amtliche Landkreis-Konzept und die Firmenauskunft — einen Schrott-/Metallankaufbetrieb ausweisen. `trader_type` ist deshalb auf **`schrotthaendler`** korrigiert (der amtlichen Überschrift „Schrotthändler" folgend). Nicht gewählt wurde `metallhaendler`, weil kein Beleg für einen reinen Buntmetall-Spezialhandel vorliegt, und nicht `autoverwertung`, weil der „Autorecycling"-Anteil im Unternehmensgegenstand kein eigenes Demontagebetriebs-Register trägt (für Salzwedel ist im Fachbetriebsregister nur ALBA Metall Nord geführt) — aus „Autorecycling" allein folgt keine Demontageanlage.

**Verzeichnisnamen und Identität:** Der Seed-Name „Recycling RiWa" löst sich zu „Recycling RiWa Richter Michael" auf, das Golocal-Profil wurde nachweislich vom Benutzerkonto „mr-saw" (Metallrecycling Salzwedel) hochgeladen. Weitere unabhängige Verzeichniseinträge führen den Standort als „MR Metall Recycling SAW" (infobel, 02.10.2026: „RiWa Michael Richter") bzw. „MR Recycling" (youdriver), jeweils mit derselben Nummer 03901 308868; das Recyclinghof-Verzeichnis nennt zusätzlich „Angenommene Abfälle: Altmetall und Schrott" (Stand Juli 2026). Eine Betreiberwebsite existiert nicht.

**Preise:** ANKAUF: keine aktuelle Preisliste; die einzige Kundenstimme (golocal, 21.06.2012, „Metallstange", „faire Preise") ist 14 Jahre alt und kein Kursbeleg. VERKAUF: keine Preisliste gefunden. GEBÜHREN: keine Tarife gefunden; das Verzeichnis weist darauf hin, dass die Menge kostenlos ist oder Gebühr kostet, das legt der Betreiber fest — keine konkrete Angabe. Öffnungszeiten bleiben leer, weil die Quellen sich widersprechen (Recyclinghof-Verzeichnis: Mo–Fr 08:00–17:00, Sa 09:00–12:00, Stand Juli 2026; youdriver: Mo 07:00–12:00 und 14:00–17:30, Sa 08:00–13:00).

**GEO:** `clear_fields: geo` gesetzt. Der gespeicherte Punkt 52.8544/11.1525 löst per Reverse-Geocoding auf „9, Steintorstraße, 29410 Salzwedel" (Stadtteil Neustadt) auf, während die bestätigte Adresse An der Ritzer Brücke 3 bei rund 52.8529/11.1820 liegt — rund 2 km entfernt auf einer anderen Straße. Koordinaten werden nicht geraten; die Owner-Geocodierung bleibt offen.

## Timeline

### Recherche 28.10.2026 — Owner-Check-in: amtliches Landkreis-Konzept, Typkorrektur auf `schrotthaendler`

- [Recherche 28.10.2026: Die Typfrage ist mit einer amtlichen Quelle entschieden. (1) AMTLICHE QUELLE: Das Abfallwirtschaftskonzept des Landkreises Altmarkkreis Salzwedel (Fortschreibung 2022) wurde als PDF heruntergeladen, mit pdftotext im Volltext ausgewertet und in Kapitel 14.1.6 „Schrotthändler" nachgelesen: Tabelle „Nr. | Standort der Anlage | Betreiber", Eintrag Nr. 27 — Standort „An der Ritzer Brücke 3, 29410 Salzwedel", Betreiber „Metall Recycling SAW" (S. 115 des Dokuments). Der Landkreis führt den Standort damit selbst als Schrotthändler; im selben Verzeichnis steht als Nr. 26 „Ritzleber Str. 29, 29410 Salzwedel OT Pretzier, ALBA Metall Nord GmbH". Das Konzept ist eine amtliche Veröffentlichung und stützt die Identität von Standort, Betreiber und Tätigkeit unabhängig von allen Verzeichnissen. (2) UNTERNEHMENSANGABEN: Die Creditreform-Firmenauskunft (Direktabruf liefert HTTP 403, daher der Suchauszug ausgewertet) nennt „Michael Richter Metallrecycling SAW", Sitz Salzwedel, Rechtsform Gewerbebetrieb, „wird derzeit von einem Manager (1 x Inhaber) geführt", ein Standort, Unternehmensgegenstand „Recycling und Handel mit metallischen Altmaterialien, Autorecycling", Hauptbranche „Großhandel mit Altmaterialien und Reststoffen", Branchencode 46.87.0, keine Nebenbranchen, Telefon +49 3901 308868. (3) NAMENS- UND ADRESSVERKNÜPFUNG: Seed „Recycling RiWa (Richter)" = Golocal-Profil „Recycling RiWa Richter Michael" (An der Ritzer Brücke 3, 03901 30 88 68), hochgeladen vom Benutzerkonto „mr-saw" (Metallrecycling Salzwedel); Gelbe Seiten/Das Örtliche nennen dieselbe Anschrift und Nummer und weisen die Daten ausdrücklich als GoLocal-Quelle aus; das unabhängige Infobel-Profil „MR Metall Recycling SAW / RiWa Michael Richter" (02.10.2026) nennt dieselbe Anschrift und Nummer, youdriver „MR Recycling" dieselbe Nummer, das Recyclinghof-Verzeichnis „MR Metall Recycling SAW" mit Stand Juli 2026 dieselbe Anschrift und Nummer sowie „Angenommene Abfälle: Altmetall und Schrott" (Verzeichnisangabe, kein Betreiberbeleg). Die Branche der Verzeichnisse lautet „Recycling" mit Leistungsumfang „Autoverwertung, Entsorgungen, Schrott, Recycling". (4) WARUM „sonstige" FALSCH WAR: Die Beleglage war bereits in der Seed-Zeile unvollständig, der Typ blieb auf dem Generikumer stehen; mit der amtlichen Einordnung als „Schrotthändler" und dem Unternehmensgegenstand „Handel mit metallischen Altmaterialien" ist die Einstufung als `schrotthaendler` belegt und `sonstige` wurde ersetzt. `metallhaendler` wurde verworfen, weil kein Beleg für einen reinen Buntmetall-Spezialhandel vorliegt und der Landkreis den Betrieb selbst unter der Überschrift „Schrotthändler" führt; `autoverwertung` wurde verworfen, weil der „Autorecycling"-Anteil im Unternehmensgegenstand kein eigenes Demontagebetriebs-Register trägt — für Salzwedel ist im amtlichen Fachbetriebs-/Entsorgungsfachbetriebsregister (Auszug 11.08.2026) nur ALBA Metall Nord geführt, kein Richter und keine RiWa, und ein Negativbefund beweist keine Nichtexistenz. (5) FRONTMATTER: trader_type auf `schrotthaendler` korrigiert; Telefon 03901 308868 übernommen, weil die Nummer sowohl in der Firmenauskunft als auch in mehreren voneinander unabhängigen Verzeichnissen (Infobel, youdriver, Recyclinghof-Verzeichnis) identisch steht, während die Gelbe-Seiten-Einträge aus einer Quelle (GoLocal) stammen und deshalb allein nicht tragfähig wären; Öffnungszeiten bleiben leer, weil die Quellen sich widersprechen (Recyclinghof-Verzeichnis: Mo–Fr 08:00–17:00, Sa 09:00–12:00, Stand Juli 2026; youdriver: Mo 07:00–12:00 und 14:00–17:30, Sa 08:00–13:00), E-Mail und Website bleiben leer (keine Betreiberwebsite auffindbar), status bleibt `pruefung`, weil der aktuelle Betriebsfortbestand nicht amtlich datiert belegt ist. (6) GEO: clear_fields: geo gesetzt, weil der gespeicherte Punkt 52.8544/11.1525 per Reverse-Geocoding auf „9, Steintorstraße, 29410 Salzwedel" (Stadtteil Neustadt) auflöst und rund 2 km von der bestätigten Adresse An der Ritzer Brücke 3 (Adresspunkt rund 52.8529/11.1820, Ortsteil Perver) entfernt liegt; es werden keine Koordinaten geraten, die Owner-Geocodierung der bestätigten Adresse bleibt offen. (7) ANKAUF: keine Preise, kein Kursbeleg; die einzige Kundenstimme (golocal, 21.06.2012, „Metallstange", „faire Preise, freundliches Auftreten, schnelle zuverlässige Abwicklung") ist 14 Jahre alt und belegt keinen Preis. VERKAUF: keine Preisliste gefunden. GEBÜHREN: keine Tarife; das Verzeichnis hält lediglich fest, dass die Menge kostenlos ist oder Gebühr kostet und der Betreiber das festlegt — keine konkrete Angabe. (8) OFFEN: (a) Gewerberegisterauskunft des Altmarkkreises Salzwedel / des Landes Sachsen-Anhalt zu „Michael Richter" und „Metall Recycling SAW" sowie zur Bedeutung der Gewerbekennziffer 46.87.0, (b) betreiberseitige Bestätigung von Öffnungszeiten, angenommenen Abfallarten und Konditionen, (c) Klärung, ob neben der Anlage An der Ritzer Brücke 3 weitere Standorte desselben Inhabers in Salzwedel bestehen („SAW" deutet auf den Ort, „RiWa" ist nicht aufgelöst), (d) Neugeocodierung. Quellen: https://www.altmarkkreis-salzwedel.de/PortalData/1/Resources/landkreis/bekantmachungen/2022/04_2022/AWK_SAW_04-04-2022.pdf (Abfallwirtschaftskonzept Altmarkkreis Salzwedel – Fortschreibung 2022, S. 115, Kap. 14.1.6) ; https://firmeneintrag.creditreform.de/29410/3350081118/MICHAEL_RICHTER_METALLRECYCLING_SAW (Direktabruf 403; Suchauszug ausgewertet) ; https://www.infobel.com/de/germany/mr_metall_recycling_saw/salzwedel/DE112251983-03901308868/businessdetails.aspx ; https://www.youdriver.com/de/mr-recycling-113096/ ; https://recyclinghof-wertstoffhof.de/recyclinghof/sachsen-anhalt/salzwedel/mr-metall-recycling-saw-an-d-ritzer-bruecke-3/ ; https://www.gelbeseiten.de/gsbiz/b5353007-ad2f-46cc-8103-91ced267e032 ; https://www.golocal.de/salzwedel/entsorgung/recycling-riwa-richter-michael-4TjQH/ ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&zoom=18&lat=52.8544&lon=11.1525&addressdetails=1 ; https://nominatim.openstreetmap.org/search?format=jsonv2&limit=1&addressdetails=1&q=An+der+Ritzer+Br%C3%BCcke+3%2C+29410+Salzwedel]

### Recherche 28.10.2026

- [Recherche 28.10.2026: Der Namens- und Firmenzusammenhang zwischen dem Seed-Namen und dem Gewerbebetrieb ist jetzt belegt, die Adresse deshalb übernommen. Creditreform-Firmenauskunft „Michael Richter Metallrecycling SAW", An der Ritzer Brücke 3, 29410 Salzwedel (Direktabruf der Profilsite HTTP 403, daher über den Suchauszug ausgewertet): Rechtsform Gewerbebetrieb, „wird derzeit von einem Manager (1 x Inhaber) geführt", Steuernummer verfügbar, ein Standort, Daten zu einer Hausbank, Unternehmensgegenstand „Recycling und Handel mit metallischen Altmaterialien, Autorecycling", Hauptbranche „Großhandel mit Altmaterialien und Reststoffen", Branchencode 46.87.0, keine Nebenbranchen. Verbindungsstück zum Seed-Namen: das Golocal-Profil „Recycling RiWa Richter Michael" (An der Ritzer Brücke 3, 29410, 03901 30 88 68) wurde nachweislich vom Benutzerkonto „mr-saw" hochgeladen, also vom Metallrecycling Salzwedel — Gelbe Seiten („Recycling RiWa Richter Michael", Kategorie Recycling, Leistungsumfang Autoverwertung, Entsorgungen, Schrott, Recycling) sowie Das Örtliche („Richter Michael Recycling RiWa") nennen dieselbe Anschrift und dieselbe Nummer; die Gelbe Seiten weisen die Daten ausdrücklich als aus einer Quelle (golocal) stammend aus, deshalb wird die Telefonnummer 03901 30 88 68 weiterhin nicht in das Frontmatter übernommen. Das amtliche Fachbetriebs-/Entsorgungsfachbetriebsregister (Auszug 11.08.2026) führt für Salzwedel nur ALBA Niedersachsen-Anhalt GmbH und für Hansestadt Salzwedel den Bauservice Altmark (Inh. Steffen Cossbau) — keinen Richter und keine RiWa; Negativbefund ohne Beweiswert für kleine Einzelbetriebe. ANKAUF: keine Preise, die einzige Kundenstimme (golocal, 21.06.2012, „Metallstange", „faire Preise, freundliches Auftreten, schnelle zuverlässige Abwicklung") ist 14 Jahre alt und kein Kursbeleg; VERKAUF: keine Preisliste; GEBÜHREN: keine Tarife. Status bleibt pruefung, weil der aktuelle Betriebsfortbestand nicht datiert belegt ist und weder Betreiberwebsite noch Annahmebedingungen vorliegen. Offen und für den Owner: Gewerberegisterauskunft der Stadt Salzwedel bzw. Landkreis Altmark zu „Michael Richter" (Rechtsform Gewerbebetrieb) und die Frage, ob die Zeile als Schrotthändler (trader_type) statt „sonstige" geführt werden soll — die Registerdaten belegen Recycling und Handel mit metallischen Altmaterialien. Quelle(n): https://firmeneintrag.creditreform.de/29410/3350081118/MICHAEL_RICHTER_METALLRECYCLING_SAW (Suchauszug; Direktabruf 403) ; https://www.gelbeseiten.de/gsbiz/b5353007-ad2f-46cc-8103-91ced267e032 ; https://www.golocal.de/salzwedel/entsorgung/recycling-riwa-richter-michael-4TjQH ; https://www.dasoertliche.de/Themen/Richter-Michael-Recycling-RiWa-Hansestadt-Salzwedel-An-der-Ritzer-Br%C3%BCcke ; https://schrottregister.pages.dev/ort-salzwedel]

### Recherche 23.10.2026

- [Recherche 23.10.2026: Gelbe-Seiten-Profil direkt gelesen: „Recycling RiWa Richter Michael“, An der Ritzer Brücke 3, 29410 Salzwedel, 03901 30 88 68; die Seite weist Golocal als Datenquelle aus und zeigt nur eine Kundenbewertung vom 21.06.2012, also keinen aktuellen Betriebs- oder Preisbeleg. Eine Websuche zeigt außerdem ein Creditreform-Profil „Michael Richter Metallrecycling SAW“ mit derselben Anschrift/Nummer; dessen Direktabruf lieferte HTTP 403, daher nur Identitäts-Lead, nicht ausgewerteter Beleg. ANKAUF/VERKAUF/GEBÜHREN: keine aktuellen Preise oder Tarife auffindbar; Bewertungsfloskel ist kein Preisnachweis. Frontmatter, Status `pruefung` und GEO unverändert; Kandidatenadresse erst nach Betreiberzuordnung geokodieren. Quellen: https://www.gelbeseiten.de/gsbiz/b5353007-ad2f-46cc-8103-91ced267e032 ; https://firmeneintrag.creditreform.de/29410/3350081118/MICHAEL_RICHTER_METALLRECYCLING_SAW (Direktabruf 403; Suchtreffer nur Lead).]

### Importiert (Seed-Stand 2026-09-30)

- Recycling/AV
- Adresse: Salzwedel (An der Ritzer Brücke 3, 29410)

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **52.8529569/11.1823088** (Mittelwert der **6**
  eindeutigen Knoten von **way/613121645**; das Objekt hat **7** Knotenreferenzen, davon 6 eindeutige - ein Knoten ist
  doppelt gezählt; die BBox der eindeutigen Knoten reicht von 11.1814092/52.8526411 bis 11.1830047/52.8533678 und
  **umschließt den Mittelwert**). Quelle 1 (OSM): **way/613121645** trägt live (abgerufen am 04.11.2026 über
  `https://api.openstreetmap.org/api/0.6/way/613121645/full.json`, HTTP 200) `name`=**MR Metall Recycling SAW**,
  `amenity`=recycling, `recycling_type`=centre, `recycling:scrap_metal`=**yes**, `landuse`=industrial - das OSM-Objekt
  benennt den Betrieb **selbst** und kartiert ihn als **Metallschrott-Recyclingzentrum**. Direkt daneben liegt
  **way/613121646** mit `addr:street`=**An der Ritzer Brücke**, `addr:housenumber`=**3**, `addr:city`=Salzwedel,
  `addr:postcode`=29410 (Mittelwert 52.8529055/11.1819731, rund 26 m entfernt) - die **Dossier-Adresse** und der
  **Betriebspunkt** sind damit in OSM **dieselbe Liegenschaft**. **Fensterdisziplin:** das am 04.11.2026 live
  abgerufene Kachelraster über **Salzwedel/Perver** (**11.1600/52.8400 bis 11.2000/52.8600**, **12 Kacheln von je
  0,01 Grad**, **0 Kachel-Fehler**) enthält die gesuchte Straße tatsächlich: die **An der Ritzer Brücke** ist im
  Raster mit den Hausnummern **1** (way/613121644, `brand`=Audi, Autohaus Salzwedel), **3** (way/613121646), **4**
  (way/689118631), **5** (way/613121637, `brand`=Volkswagen; Skoda), **6** (way/689118633), **7** (way/613121639,
  Lack-Fix), **8** (way/689118634), **9** (way/613121636) und **11** (way/613121635) vertreten - die
  **Dossier-Hausnummer 3 existiert** und ist im Raster das einzige Objekt dieser Nummer auf dieser Straße. Quelle 2
  (Betreiber/Verzeichnis, im Dossier bereits zitiert, live nachgelesen am 04.11.2026): (a) das **amtliche
  Abfallwirtschaftskonzept des Landkreises Altmarkkreis Salzwedel** (Fortschreibung 2022, Kapitel 14.1.6
  „Schrotthändler“, Nr. 27, S. 115) führt den Standort **„An der Ritzer Brücke 3, 29410 Salzwedel“** mit dem
  Betreiber **„Metall Recycling SAW“** - der **Landkreis** selbst weist die Anlage als **Schrotthändler** aus, und
  die Betreiberbezeichnung **„Metall Recycling SAW“** deckt sich mit dem OSM-Namen **„MR Metall Recycling SAW“**.
  (b) die **11880**-Detailseite führt **„Recycling RiWa Richter Michael“** mit **An der Ritzer Brücke 3, 29410
  Salzwedel**, Telefon **(03901) 308868** und der Portalkoordinate **11.181759/52.852978**, die nur rund **35 m** vom
  gewählten OSM-Punkt entfernt liegt; das im Dossier zitierte **Creditreform-Firmenprofil „Michael Richter
  Metallrecycling SAW“** nennt dieselbe Anschrift mit demselben Gegenstand („Recycling und Handel mit metallischen
  Altmaterialien, Autorecycling“). Der Seed-Name **„Recycling RiWa (Richter)“** löst sich damit über das 11880-Profil
  „Recycling RiWa **Richter Michael**“ und das **Infobel**-Profil „**MR** Metall Recycling **SAW** / RiWa Michael
  Richter“ in **einen** Betrieb auf. **Qualifizierung:** eine **Betreiberwebsite** existiert nicht, die Preise sind
  nicht belegt, und das amtliche Fachbetriebsregister (Auszug 11.08.2026) führt für Salzwedel nur ALBA - ein
  Negativbefund ohne Beweiswert für kleine Einzelbetriebe; die Koordinate bezeichnet die amtlich und verzeichnisseitig
  belegte **Betriebsstätte** (Recyclingzentrum mit Schrottannahme), nicht eine geprüfte Waage. `street`, `postcode`,
  `phone`, `email`, `opening_hours`, `website`, `website_status`, `description`, `dropoff_json`, `pickup_json` und
  `status: pruefung` bleiben unverändert; der zurückgenommene Produktionspunkt **52.8544/11.1525** (Steintorstraße 9,
  Neustadt) wird durch diesen Punkt ersetzt. `clear_fields: geo` **entfällt**, weil die Koordinate nun belegt vorliegt
  und der Seed `geo` neben belegtem `lat`/`lon` ablehnt. Quelle(n): https://api.openstreetmap.org/api/0.6/way/613121645/full.json ;
  https://api.openstreetmap.org/api/0.6/way/613121646/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=11.1600,52.8400,11.2000,52.8600 ;
  https://www.altmarkkreis-salzwedel.de/PortalData/1/Resources/landkreis/bekantmachungen/2022/04_2022/AWK_SAW_04-04-2022.pdf ;
  https://www.11880.com/branchenbuch/salzwedel-hansestadt/060444309B53145266/recycling-riwa-richter-michael.html ;
  https://firmeneintrag.creditreform.de/29410/3350081118/MICHAEL_RICHTER_METALLRECYCLING_SAW (Abruf 04.11.2026).]
