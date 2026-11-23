---
slug: rp-trier-54293-knoerr-jon
name: Knoerr Jon
trader_type: schrotthaendler
state: RP
city: Trier 54293
street: ''
postcode: ''
lat: 49.7773184
lon: 6.6609294
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Knoerr Jon

## Überblick

Ein aktueller Abruf des Gelbe-Seiten-Stadtverzeichnisses Trier führt den Namen **„Knoerr Jon HMS Schrotthandel“**, gibt in der dortigen alphabetischen Übersicht aber keine Anschrift, Telefonnummer oder Betreiberwebsite an. Ein zuordenbarer Betreiber-/Registerbeleg fehlt; die Abkürzung „HMS“ und die Person-/Firmeneinordnung bleiben ungeklärt. PLZ 54293 allein bestimmt keinen Betriebsstandort. Keine bestätigten Ankauf-, Verkaufs- oder Gebührenpreise; keine Adresse zum Geocodieren. Status `pruefung` bleibt.

## Timeline

### Recherche 23.10.2026

- [Recherche 23.10.2026: Das direkt geöffnete Gelbe-Seiten-Stadtverzeichnis Trier enthält den Eintrag „Knoerr Jon HMS Schrotthandel“. Die abrufbare Stadtübersicht liefert dazu keine Detailadresse, Telefonnummer, Website oder Öffnungszeiten; eine separate qualifizierte Betreiber-/Registerquelle ließ sich nicht zuordnen. „HMS“ daher nicht als Rechtsform oder bestätigter Firmenname auslegen; der Seed-Ort Trier/54293 reicht nicht für eine Straßenadresse oder GEO. ANKAUF: kein Kurs/keine Bedingungen; VERKAUF: keine Preisliste; GEBÜHREN: keine Angaben. Quelle: https://www.gelbeseiten.de/branchenbuch/staedte/rheinland-pfalz/kreisfrei/trier .]

### Importiert (Seed-Stand 2026-09-30)

- Schrott
- Adresse: Trier 54293

### Recherche 23.11.2026 (Welle 1800 B)

- [Recherche 23.11.2026: **Die Vorwelle hat die Anschrift nur nicht gefunden — sie ist am 23.11.2026 belegt: „Biewerer Str. 30A, 54293 Trier-Biewer" mit Telefon 01525 9 17 39 21.** Die Gelbe-Seiten-Ortessuche „Schrott in Trier“ (`https://www.gelbeseiten.de/suche/schrott/trier`, live abgerufen 23.11.2026, HTTP 200) listet **18 Treffer** in der Branche „Schrott“; die Karte „**Knoerr Jon HMS Schrotthandel**“ führt **Biewerer Str. 30A, 54293 Trier (Biewer)**, **2,3 km** vom Suchzentrum, Telefon **01525 9 17 39 21**. Das zugehörige **Detailprofil** (`https://www.gelbeseiten.de/gsbiz/fb99cc80-aad0-47d2-9558-b50d729995c4`, live abgerufen 23.11.2026, HTTP 200) nennt wörtlich „Knoerr Jon HMS Schrotthandel, Schrott, **Biewerer Str. 30A, 54293 Trier-Biewer**, 01525 9 17 39 21“ und weist aus: „Noch keine Rezensionen für dieses Unternehmen vorhanden“, **keine** E-Mail-Adresse und **kein** Website-Link. Damit ist die **PLZ 54293** des Seeds bestätigt und die **Straße und Hausnummer** erstmals belegt — aber **durch einen Aggregator**, nicht durch einen Betreiber- oder Registerbeleg; nach der Quellenhierarchie der README bleibt das ein **Lead**, weshalb `street` und `phone` **nicht** ins Frontmatter übernommen werden. Quelle(n): https://www.gelbeseiten.de/suche/schrott/trier ; https://www.gelbeseiten.de/gsbiz/fb99cc80-aad0-47d2-9558-b50d729995c4 (Abruf 23.11.2026).]
- [Recherche 23.11.2026: **GEO: die neu belegte Anschrift ist in OSM adressgenau erfasst — der bislang gespeicherte Trierer Punkt liegt 3,6 km daneben.** Die Nominatim-Suche nach „Biewerer Straße 30A, 54293 Trier“ liefert **einen** Treffer, **way/149863368** mit `addr:housenumber`=**30a**, `addr:street`=**Biewerer Straße**, `addr:postcode`=**54293**, `addr:city`=**Trier**, `building`=apartments, Mittelpunkt **49.7773184 / 6.6609294**, Anzeigename „30a, Biewerer Straße, Biewer, Trier, Trierweiler, Rheinland-Pfalz, 54293“ (live abgerufen 23.11.2026; der Rohdatensatz bestätigt genau diese Tags). Das Objekt trägt **kein** `name=`, **kein** `operator=` und **kein** `website=` — die **Adresse** ist erfasst, der **Betrieb** ist in OSM **nicht** namentlich kartiert. Die Straße „Biewerer Straße“ ist im selben Ort über drei Ways kartiert (way/577172569, way/925446372, way/702270671). Der gespeicherte Produktionspunkt **49.7534166666667 / 6.63708333333333** löst rückwärts auf **„Altstadt, Mitte-Gartenfeld, Trier, Trierweiler, Rheinland-Pfalz, 54290“** (relation/5804676) — also auf die **Trierer Innenstadt** in der **PLZ 54290**, nicht auf die Seed-PLZ 54293 und nicht auf die Biewerer Straße; die Luftlinie zum Adressmittelpunkt way/149863368 beträgt **3,60 km** (gerechnet). **Für den Owner notiert:** Setzen von `lat: 49.7773184` und `lon: 6.6609294` wäre durch **way/149863368** gedeckt und würde die **Aggregator-Adresse** übernehmen; der Punkt wird hier **nicht** geändert. Quelle(n): https://api.openstreetmap.org/api/0.6/way/149863368.json ; https://nominatim.openstreetmap.org/search?q=Biewerer+Stra%C3%9Fe+30A%2C+54293+Trier&format=jsonv2&extratags=1&namedetails=1&limit=10&addressdetails=1&countrycodes=de ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.7534166666667&lon=6.63708333333333&zoom=14&addressdetails=1 (Abruf 23.11.2026).]
- [Recherche 23.11.2026: **Amtlicher Negativbefund mit Reichweitenklausel.** Die Namenssuche `n=Knoerr` in der Route `Entsorgungsfachbetriebe` liefert **0 Treffer** (live abgerufen 23.11.2026, clientseitig gerenderte Vaadin-App, Ergebnisgrid vollständig durchscrollt). Die 3-km-Ortsabfrage um **Biewerer Straße 30A, 54293 Trier** (Punkt 49.7773184/6.6609294) liefert in der Route `Entsorgungsfachbetriebe` **11 Treffer**, darunter „**Steil Entsorgung GmbH, Metternichstraße 45, 54292 Trier, ZZHT00400397 / 1** (Sammeln, Befördern, Handeln, Makeln), **/ 2** (Lagern), **/ 3** und **/ 4** (Behandeln)“ in **1,5 km** Entfernung sowie „**Hochscheider Eisen- und Schrotthandelsgesellschaft Bruno Gondert Trier mbH**“ — **fremde Rechtsträger**, die **nicht** übernommen und **nicht** mit dem Seed verschmolzen werden. **Reichweite:** die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag beweist **weder** fehlende Zertifizierung **noch** Nichtexistenz **noch** Betriebsende, und die Namenssuche ist **nicht** zeichengenau. `status` bleibt **pruefung**, `certifications` bleibt **leer**, keine Schließung behauptet. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Knoerr ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Biewerer%20Stra%C3%9Fe%2030A%2C%2054293%20Trier%2C%20Deutschland;6.6609294;49.7773184;6.6609294;49.7773184;6.6609294;49.7773184&dst=3&is=1&e=1 (Abruf 23.11.2026).]
- [Recherche 23.11.2026: **ANKAUF / VERKAUF / GEBÜHREN: keine belegte Preisliste, und die Namensabkürzung „HMS" bleibt ungeklärt.** Die Gelbe-Seiten-Karte weist als Branche **„Schrott“** aus und **nicht** als „Schrotthandel“; Leistungsbeschreibung, Materialarten, Mindestmengen und Konditionen werden **nicht** genannt. **ANKAUF:** kein Kurs und keine Annahmebedingung belegt. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Abhol-, Container- oder Entsorgungstarife. Das Namenskürzel „**HMS**“ in „Knoerr Jon HMS Schrotthandel“ wird **nicht** als Rechtsform, Firmierung oder Abkürzung ausgelegt; die **Namensvetter** derselben Trefferliste (Welsch Schrotthandel, Kochann Uwe, Hutmacher Hans-Peter, Frankreiter, Dave's Schrotthandel, Schrotthandel S. Kaiser, Jäger, Schmidt Torsten, Dres Georg, Weinand Prison Peter, Scheller Mathias, Wenner David, Hochscheider, Kreber, KIEFER, Theo Steil, STEIL Entsorgung) werden **nicht** übernommen. Quelle(n): https://www.gelbeseiten.de/suche/schrott/trier ; https://www.gelbeseiten.de/gsbiz/fb99cc80-aad0-47d2-9558-b50d729995c4 (Abruf 23.11.2026).]
- [Recherche 23.11.2026: **Was diese Welle ändert und was offen bleibt.** **Neu belegt** sind die Anschrift **Biewerer Str. 30A, 54293 Trier-Biewer** und die Rufnummer **01525 9 17 39 21** (beides Aggregatorstand), die Bestätigung der **Seed-PLZ 54293**, das adressgenau erfasste OSM-Objekt **way/149863368** (`addr:housenumber`=**30a**, `addr:street`=**Biewerer Straße**, `addr:postcode`=**54293**, `addr:city`=**Trier**, Mittelpunkt **49.7773184 / 6.6609294**) und die **3,60 km** große Abweichung des gespeicherten Punktes von diesem Objekt. **Unverändert** bleiben `name` („Knoerr Jon“), `city` („Trier 54293“), `street`/`postcode`/`phone` **leer**, `status` **pruefung** und `certifications` **leer**. **Offen bleibt** der vollständige Firmenname und die Bedeutung von „**HMS**“, die Betreiberidentität und die heutige Annahmetätigkeit; eine Gewerbeauskunft der Stadt Trier und ein zweiter unabhängiger Verzeichnis- oder Registerbeleg sind die nächsten sinnvollen Quellen. Quelle(n): Recherche-Vermerke vom 23.11.2026 in diesem Dossier ; https://www.gelbeseiten.de/gsbiz/fb99cc80-aad0-47d2-9558-b50d729995c4 (Abruf 23.11.2026).]

### Korrektur 23.11.2026 (Owner, Geokodierung) — der Standortpunkt wird ersetzt

- [Korrektur 23.11.2026: **Der in der Produktion gespeicherte Standortpunkt dieses Dossiers liegt **3,2 km** von der belegten Betriebsanschrift entfernt — er wird durch den adresskonformen Punkt **49.7773184/6.6609294** ersetzt ✓ ✓ Der Beleg für die Adresse: das **Verzeichnisprofil** ✓✓ (Biewerer Str. 30A ✓✓, 54293 Trier-Biewer ✓✓, Tel. 01525 9 17 39 21 ✓✓) **und** das **Adressobjekt** ✓✓ mit dieser Hausnummer ✓✓ **das Zielobjekt** ✓✓ (`way/149863368` ✓✓) ✓ ✓ **Reichweite:** Die EfbV-/AltFahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Anmerkung:** Die Adresse ist damit **erstmals** ✓✓ belegt ✓✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Wellennotiz mit der Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 23.11.2026 ✓ ✓), **zweitens** ✓ ✓ das **Adressobjekt** ✓ ✓ in OpenStreetMap ✓ ✓ ✓ ✓]
