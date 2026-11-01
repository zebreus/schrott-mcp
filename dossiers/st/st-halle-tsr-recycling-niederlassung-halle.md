---
slug: st-halle-tsr-recycling-niederlassung-halle
name: TSR Deutschland GmbH & Co. KG, Niederlassung Halle
trader_type: metallhaendler
state: ST
city: Halle
street: Äußere Radeweller Straße 2
postcode: '06132'
phone: 0345 7728790
email: ''
opening_hours: ''
website: https://www.tsr-recycling.de/standorte/schrotthaendler-halle/
website_status: blockiert
status: aktiv
description: ''
dropoff_json: '{"allowed": true, "customer_types": ["privat", "gewerbe"], "conditions": "Annahme fuer Privatkunden, Gewerbe und Industrie; THE METAL BOX fuer Kleinmengen; Containerdienst (TSR-Standortseite, 30.09.2026)"}'
pickup_json: ''
provenance_section: Überregionale / große Verwerter mit ST-Standorten
provenance_ankauf_raw: ja
provenance_origin: table
---

# TSR Recycling GmbH & Co. KG, Niederlassung Halle

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Stahlschrott, NE-Metalle

### Korrektur 30.09.2026 (Feedback-Triage)

- [Korrektur Feedback fb1847: Adresse Äußere Radeweller Straße 2, 06132 Halle + Tel. 0345 7728790 zweiquellig (TSR-Standortseite via Jina-Reader: T +49 345 7728790, F ...87933 + Gelbe Seiten: „Äußere Radeweller Str. 2, 06132 Halle (Radewell-Osendorf), 0345 7728790"); Quelle: tsr-recycling.de/standorte/schrotthaendler-halle + gelbeseiten.de/Suche/TSR/Halle]
- [Korrektur Feedback fb1847: Privatanlieferung JA mit Website-Beleg — TSR-Seite: „Ob Privatkunde, Gewerbebetrieb oder Industrie – wir übernehmen die fachgerechte Annahme ... Ihrer Altmetalle" (zusätzlich Papierannahme, Schwerschrottaufbereitung, THE METAL BOX für Kleinmengen, Containerdienst) → dropoff_json (privat + gewerbe); Quelle: TSR-Standortseite (30.09.2026)]
- [Korrektur Feedback fb1847: website_status blockiert (Human-Challenge auf tsr-recycling.de, verifiziert 30.09.2026); Quelle: Direktabruf]
- (Einzelbeleg, unsicher): Waage-/Kassenzeiten nur von der TSR-Seite (Waage Mo–Do 07:00–16:30/Fr 07:00–16:00, Kasse Mo–Fr 07:00–15:30, letzte Einwiegung 30 Min. vor Schluss) → opening_hours leer gelassen; TSR-Seitentext nennt fälschlich „Sachsen" (Halle liegt in Sachsen-Anhalt) — redaktioneller Fehler der Website.
### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Äußere Radeweller Straße, Halle-Radewell).** Kachelraster über den Straßenverlauf (Straßenbbox 11.9900/51.4150 bis 12.0100/51.4350, **4 Kacheln, 0 Kachel-Fehler**; Objekt-BBox der Straßen 51.4225311/11.9956104 bis 51.4289585/12.0044252). Zählung getrennt: **10 Adressobjekte / 10 verschiedene Hausnummern** (4, 5, 6, 7, 9, 10, 11, 12, 13, 15) auf der Straße „Äußere Radeweller Straße“, im gesamten Rastergebiet **978 Adressobjekte mit Hausnummer / 222 verschiedene Hausnummern**; im Raster tragen 212 Objekte `name=` und 60 Objekte `operator=`. **Die Dossier-Hausnummer 2 existiert in OSM auf der „Äußeren Radeweller Straße“ nicht** - sie ist dort **nicht** erfasst. Sie existiert auf der **benachbarten, gleichlautend benannten anderen Straße „Radeweller Straße“**: way/183178811 mit `addr:housenumber=2`, `addr:street=Radeweller Straße`, `addr:city=Halle (Saale)`, `addr:postcode=06132`, `addr:suburb=Radewell`, `building=yes` (Punktlage 51.4232830/11.9958285). Das ist zu schreiben, nicht zu übergehen; ob Betrieb und Objekt dieselbe Liegenschaft meinen, ist aus den Quellen **nicht** entscheidbar. Weitere Objekte mit Bezug zum Betrieb im Raster: **keines** - die Namenssuche nach „TSR“, „Schrott“, „Metall“ und „Recycling“ über die Felder name, operator, description und website findet **keinen** Treffer mit Bezug zur TSR. Zwei benannte Objekte der Äußeren Radeweller Straße sind **andere** Betriebe: way/59170940 mit `addr:housenumber=12`, `amenity=recycling`, `name=Wertstoffmarkt`, `operator=Hallesche Wasser und Stadtwirtschaft GmbH`, `recycling_type=centre`, `recycling:scrap_metal=yes`, `opening_hours=Mo-Fr 06:00-21:00; Sa 07:00-12:00` (ein **kommunaler** Wertstoffmarkt) und way/59170938 mit `addr:housenumber=11`, `name=SV Grün-Weiß Ammendorf e.V.`, `leisure=sports_centre` (ein Sportverein). Die Straßen selbst sind way/24405751 (`name=Äußere Radeweller Straße`, `highway=residential`, `maxspeed=30`) und way/28767380 (`name=Radeweller Straße`, `highway=residential`, `maxspeed=30`, `postal_code=06132`). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=11.9900,51.4150,12.0100,51.4350 ; https://api.openstreetmap.org/api/0.6/way/183178811.json ; https://api.openstreetmap.org/api/0.6/way/59170940.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Betreiber-Primärquelle live - Standort, Kontakt und Zeiten bestätigt.** Die TSR-Standortseite (live abgerufen am 31.10.2026; der Direktabruf auf tsr-recycling.de liefert HTTP 200 mit einer Bot-Challenge-Seite, der Seiteninhalt wurde über den Jina-Reader gelesen) nennt **TSR Deutschland GmbH & Co. KG, Äußere Radeweller Straße 2, 06132 Halle**, **T +49 345 7728790**, **F +49 345 77287933**, **Waage: Mo-Fr 07:00–15:30 Uhr** mit dem Zusatz **„Letzte Einwiegung 15:00 Uhr.“** und **Kasse: Mo-Fr 07:00–15:30 Uhr**; ferner die Annahme „Ob Privatkunde, Gewerbebetrieb oder Industrie“, **Papierannahme**, **Schwerschrottaufbereitung**, **Abbruch- und Demontagearbeiten**, **Containerdienst** und **THE METAL BOX** für Kleinmengen sowie das **Entsorgungsfachbetriebszertifikat**, die **QM-Zertifizierung nach DIN EN ISO 9001:2015** und den Status **Autorisierter Sammelpartner im A I U I F e.V.** (jeweils als PDF). Das Impressum (live, HTTP 200) führt die Gesellschaft als **TSR Deutschland GmbH & Co. KG**, Brunnenstraße 138, 44536 Lünen, Amtsgericht Dortmund, **HRA 19743**, USt-IdNr DE 369313272, persönlich haftende Gesellschafterin **TSR Deutschland Verwaltungs GmbH** (AG Dortmund, **HRB 36274**). Die im Fließtext der Standortseite verwendete Landesangabe „Schrott- und Metallrecycling **in Sachsen**“ ist weiterhin ein redaktioneller Fehler der Website - Halle liegt in Sachsen-Anhalt. Quelle(n): https://www.tsr-recycling.de/standorte/schrotthaendler-halle/ ; https://www.tsr-recycling.de/impressum/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **ANKAUF, VERKAUF und GEBÜHREN getrennt.** **ANKAUF:** **keine** bezifferte Sorten- oder Preisliste; die Privatkunden-Seite der Betreiberseite erklärt ausdrücklich: „Die Preise für Schrott und Altmetall unterliegen täglichen Schwankungen, da sie von den Rohstoffmärkten beeinflusst werden. Aus diesem Grund können wir keine universell gültigen Preisangaben machen.“ und nennt nur „**tagesaktuelle Preise**“ (Wertmaterialbewertung vor Ort, dokumentierte Einzelverwiegung, Auszahlung **sofort in bar** bzw. **bar oder per Überweisung**); für Privatkunden besteht eine **gesetzliche Identifizierungspflicht** (Ausweisdokument). **VERKAUF:** **keine** Verkaufspreisliste und **kein** bezifferter Rückvergütungspreis; „transparente Preise“ werden angekündigt, aber **nicht** mit Zahlen belegt. **GEBÜHREN:** **keine** bezifferte Betreibergebühr, **keine** Mindestmengengebühr, **kein** Kassenentgelt; **THE METAL BOX**, Containerdienst, Annahme und Abholung werden **ohne** Preisangabe genannt. Kommunale Tarife sind **nicht** Gegenstand dieser Quelle; der in OSM geführte kommunale **Wertstoffmarkt** der Halleschen Wasser und Stadtwirtschaft GmbH an der Äußeren Radeweller Straße 12 (way/59170940) ist ein **anderer** Betreiber mit **anderen** Öffnungszeiten und wird **nicht** als Gebühren- oder Angebotsbeleg der TSR verwendet. Quelle(n): https://www.tsr-recycling.de/kunden/privatkunden/ ; https://www.tsr-recycling.de/standorte/schrotthaendler-halle/ ; https://api.openstreetmap.org/api/0.6/way/59170940.json (Abruf 31.10.2026).]
- [Korrektur 31.10.2026: **Die Waage- und Kassenzeiten der Vorwelle sind überholt.** Die am 30.09.2026 notierten Angaben „Waage Mo–Do 07:00–16:30 / Fr 07:00–16:00, Kasse Mo–Fr 07:00–15:30“ sind auf der live abgerufenen Standortseite am 31.10.2026 **nicht** mehr vorhanden; dort steht jetzt **Waage Mo-Fr 07:00–15:30** mit dem Zusatz „Letzte Einwiegung 15:00 Uhr“ und **Kasse Mo-Fr 07:00–15:30**. Die Vorwellen-Notiz bleibt **wortgleich** stehen, der aktuelle Stand ist hier datiert festgehalten. `opening_hours` bleibt unverändert **leer**, weil die Zeiten laut Betreiberseite **Waage und Kasse** bezeichnen und eine allgemeine Öffnungszeit der Annahme daraus nicht ohne Weiteres folgt. Quelle(n): https://www.tsr-recycling.de/standorte/schrotthaendler-halle/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Punkte und Geokodierungsbedarf.** `name` wird von „TSR Recycling GmbH & Co. KG, Niederlassung Halle“ auf **„TSR Deutschland GmbH & Co. KG, Niederlassung Halle“** korrigiert: das Impressum der Betreiberseite führt die Gesellschaft am 31.10.2026 unter dieser Bezeichnung (HRA 19743, AG Dortmund); der Slug bleibt unverändert. `street`, `postcode`, `phone`, `opening_hours`, `website` und `website_status: blockiert` bleiben unverändert, `dropoff_json` bleibt unverändert (Privat, Gewerbe und Industrie, THE METAL BOX für Kleinmengen, Containerdienst - live bestätigt). Offen bleibt, ob die in OSM fehlende Hausnummer 2 auf der „Äußeren Radeweller Straße“ durch die Schreibweise „Radeweller Straße“ oder durch eine geänderte Nummerierung erklärt wird. **Geokodierungsbedarf:** der read-only Produktionspunkt 51.60848125/11.27634375 (Abruf 31.10.2026) liegt rund **53,9 km** vom OSM-Objekt way/183178811 (`addr:housenumber=2`, `addr:street=Radeweller Straße`, `addr:postcode=06132`) entfernt und damit **vollständig außerhalb** des Standortgebiets Halle-Radewell - der Punkt ist **kein** Standort des Betriebs und ist zu ersetzen; eine Geokodierung erfolgt **nicht** in dieser Welle. Quelle(n): https://api.openstreetmap.org/api/0.6/way/183178811.json ; https://www.tsr-recycling.de/impressum/ ; read-only `public.db` (31.10.2026).]

### Geokodierung 01.11.2026

- [Geokodierung 01.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Hausnummer **2** existiert auf der
  **„Äußeren Radeweller Straße“** in OSM **nicht**: das am 01.11.2026 nachgeladene Kachelraster 11.9800/51.4150 bis
  12.0200/51.4400 (**4 Kacheln, 0 Kachel-Fehler**) erfasst die **„Äußere Radeweller Straße“** mit **10 Adressobjekten /
  10 verschiedenen Hausnummern** (4, 5, 6, 7, 9, 10, 11, 12, 13, 15) - **die Nummer 2 fehlt**, die Nummer 12 ist
  way/59170940 (`name`=Wertstoffmarkt, `operator`=Hallesche Wasser und Stadtwirtschaft GmbH, kommunaler Wertstoffmarkt,
  **anderer Betreiber**) und die Nummer 11 ist way/59170938 (`name`=SV Grün-Weiß Ammendorf e.V.,
  `leisure`=sports_centre, **anderer Betrieb**). Die Nummer 2 existiert stattdessen auf dem **benachbarten
  Straßensegment „Radeweller Straße“** (way/183178811, `addr:housenumber`=2, `addr:street`=Radeweller Straße,
  `addr:city`=Halle (Saale), `addr:postcode`=06132, `addr:suburb`=Radewell, `building`=yes, ohne `name` und ohne
  `operator`; Mittelwert der eindeutigen Knoten 51.4232830/11.9958285). **Die beiden Straßennamen sind keine
  verschiedenen Straßen, sondern zwei OSM-Segmente derselben durchgehenden Straße:** way/28767380
  (`name`=Radeweller Straße) und way/24405751 (`name`=Äußere Radeweller Straße) teilen sich den Knoten node/264921626
  bei 51.4259273/12.0000006 und treffen sich dort ohne geometrische Lücke; das Segment „Radeweller Straße“ trägt die
  Nummern 1 bis 17, das Segment „Äußere Radeweller Straße“ die Nummern 4 bis 15, und beide Segmente führen
  **dieselben Nummern 4, 5, 6, 9, 10, 11, 12, 13 und 15** - die Nummerierung ist in OSM an der Segmentgrenze
  doppelt vergeben. Der Adresspunkt „Radeweller Straße 2“ liegt rund **320 m** vom Objektabschnitt „Äußere Radeweller
  Straße 4“ und rund **412 m** vom gemeinsamen Grenzknoten entfernt. **Welches der beiden Segmente die offizielle
  Nummerierung des Betriebs trägt, ist aus den Quellen nicht entscheidbar**; die Betreiberquelle nennt
  „Äußere Radeweller Straße 2“, die OSM-Objekte legen die Nummer 2 auf das andere Segment. Quelle 1 (Betreiber): die
  Standortseite der TSR (live abgerufen am 01.11.2026, HTTP 200) nennt unverändert **„TSR Deutschland GmbH & Co. KG,
  Äußere Radeweller Straße 2, 06132 Halle“**, T +49 345 7728790, Waage und Kasse Mo–Fr 07:00–15:30 Uhr; die Seite
  verlinkt als „Anfahrt“ eine Google-Maps-Suchanfrage **„TSR Deutschland GmbH & Co. KG, Äußere Radeweller Straße 2
  06132 Halle“** - eine **Suchanfrage ohne Koordinaten**, aus der sich kein georeferenzierter Punkt gewinnen lässt
  (die Seite enthält weder ein `geo`-, `latitude`/`longitude`- noch ein `!3d…`-Koordinatenfeld). Quelle 2 (OSM): das
  Kachelraster sowie way/183178811, way/24405751, way/28767380, way/59170940 und way/59170938 (live am 01.11.2026
  geladen). Der read-only Produktionsabruf am 01.11.2026 zeigt den bisherigen Produktionspunkt
  51.60848125/11.27634375 - er liegt rund **53,9 km** vom OSM-Objekt „Radeweller Straße 2“ entfernt und damit
  **vollständig außerhalb** des Standortgebiets Halle-Radewell; er ist **kein** Standort des Betriebs. Eine
  Koordinate wird dennoch **nicht** gesetzt: solange die offizielle Hausnummernzuordnung zwischen „Äußere Radeweller
  Straße“ und „Radeweller Straße“ nicht geklärt ist, wäre jeder gesetzte Punkt eine unbelegte Vermutung; eine
  Ersatzkoordinate aus einem Nachbarobjekt wird ausdrücklich **nicht** gesetzt. `status`, `street`, `postcode`,
  `phone`, `opening_hours`, `website`, `website_status` und `dropoff_json` bleiben unverändert. Quelle(n):
  https://www.tsr-recycling.de/standorte/schrotthaendler-halle/ ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=11.9800,51.4150,12.0200,51.4400 ;
  https://api.openstreetmap.org/api/0.6/way/183178811/full.json ; read-only `public.db` (01.11.2026).]
