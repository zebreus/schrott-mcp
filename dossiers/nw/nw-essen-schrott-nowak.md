---
slug: nw-essen-schrott-nowak
lat: 51.4914351
lon: 7.0675324
name: Schrott Nowak
trader_type: schrotthaendler
state: NW
city: Essen
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
provenance_section: Neu, Ankauf unklar (nur Verzeichnis-/Profilbeleg)
provenance_ankauf_raw: mobil/unklar
provenance_origin: table
---

# Schrott Nowak

## Überblick

Schrott Nowak ist ein Backyard-Betrieb, dessen Standort und aktuelle Tätigkeit nicht
belegt sind. Ein archiviertes Impressum von 2016 nennt Miroslaw Nowak an der
Bonnekampstraße 14, 45327 Essen, und ein Verzeichniseintrag von 2022 wiederholt Name,
Anschrift und Mobilnummer, beide Quellen sind jedoch keine Betreiber-Primärquelle und
nicht tagesaktuell. Die Domain schrott-nowak.de ist nicht mehr auflösbar, die im Seed
gespeicherte Adresse Marktstraße 14 war doppelt fehlerhaft und wurde zurückgenommen, auch
die Umsatzsteuer-Identnummer gibt ohne Anschriftauflösung keinen Beleg. Offen bleibt damit
die Frage, ob der Betrieb heute noch besteht. Die Koordinate bezeichnet die letzte
dokumentierte Geschäftsanschrift, die OSM-Erfassung beschreibt sie als Wohngebäude ohne
Betriebsbezug; Ankaufspreise sind nicht belegt.

## Timeline

### Owner-Korrektur 27.10.2026: falsches GEO und Fehladresse zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`; der Reverse-Lookup führt auf die Fernbus-Spur „Freiheit“ am Essener Hauptbahnhof und damit auf keinen Betriebsort. Das Betreiber-Impressum im Webarchiv vom 06.02.2016 nennt Miroslaw Nowak, Bonnekampstraße 14, 45327 Essen mit Steuernummer DE260095030, und der unabhängige 11880-Eintrag nennt dieselbe Adresse und Telefonnummer; die USt-IdNr DE260095030 ist über den EU-VIES-Dienst am 27.10.2026 gültig, wobei VIES weder Name noch Anschrift offenlegt. Die bisher gespeicherte Adresse „Schrott Nowak Essen Marktstr. 14“ ist damit doppelt fehlerhaft: Sie mischt Name und Stadt in das Adressfeld, und „Marktstraße 14“ existiert in Essen nur in 45355, nicht in 45327. Die Domain schrott-nowak.de ist nicht mehr auflösbar, die letzte Betreiberfassung stammt von 2017 und der Verzeichniseintrag von 2022, ein aktueller Betrieb ist damit nicht belegt. Deshalb werden das falsche GEO-Paar mit `clear_fields: geo` und die Fehlzeile in `street` zurückgenommen; die belegte Bonnekampstraße 14 bleibt als Lead im Timeline, wird aber nicht als aktueller Standort in die Frontmatter übernommen. Keine Ersatzkoordinate, keine Statusänderung, keine Schließungsbehauptung; `status: pruefung` bleibt. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; http://web.archive.org/web/20160206233754/http://schrott-nowak.de/Impressum.htm ; https://www.11880.com/branchenbuch/essen-ruhr/060440090B102234372/schrott-nowak-de-miroslaw-nowak.html ; https://ec.europa.eu/taxation_customs/vies/rest-api/ms/DE/vat/260095030 ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 03.10.2026

- [Recherche 03.10.2026: Die neu gefundenen Treffer bei SchrottFinder, SchrottRadar und SchrottplatzMetall sind Branchen-/Schrottverzeichnisse und keine unabhängigen Betreiberbelege; sie wiederholen Marktstraße 14 und dieselbe Mobilnummer, teils mit PLZ-Angabe 45327. StadtBranche nennt 45327, dieselbe Telefonnummer und verweist auf eine frühere Betreiber-Domain, aber keine konkrete Straße; es belegt weder aktuelle Betreiberidentität noch Anschrift. Betreiber-Startseite sowie Impressum, Kontakt, Leistungen und Galerie der Domain waren nicht abrufbar (HTTP 422/Transportfehler); kein Register-/Kommunalbeleg und kein belastbarer Zweitbeleg für die Identität, Anschrift oder Tätigkeit. Deshalb keine Frontmatter-Felder ergänzt/korrigiert; die Seed-Straßenangabe bleibt ungeklärt, Status pruefung, kein Schließungs- oder Verlagerungsbeleg. Quelle(n): https://schrottfinder.de/schrott-nowak-essen/, https://schrottradar.de/schrott-nowak/, https://schrottplatzmetall.de/schrott-nowak-essen/, http://stadtbranche.de/w-schrott-nowak.de, https://schrott-nowak.de/, https://schrott-nowak.de/impressum, https://schrott-nowak.de/kontakt]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Negativbefund — GS „schrott essen“ (30 Treffer, Komplettliste geprueft) enthaelt kein Nowak, keine Marktstr. 14. Backyard-Typ (0176-Nr.) ohne Betreiber-/Verzeichnisbeleg. Kein Fill, status bleibt pruefung (miss-nahe).; Quelle(n): https://www.gelbeseiten.de/suche/schrott/essen (Abruf 01.10.2026)]

### Importiert (Seed-Stand 2026-09-30)

- Marktstr. 14, 0176-Nr. (Backyard-Typ)
- urspr. Website-Angabe: keine gefunden

### Recherche 30.10.2026

- [Recherche 30.10.2026: Der bislang nur als Lead notierte 11880-Eintrag ist am 30.10.2026 vollständig gelesen: "Schrott-Nowak.de Miroslaw Nowak", Branche "Altmaterialhandel / Schrotthandel / Altmetallrecycling", "Bonnekampstr. 14", "45327 Essen (Katernberg)", "(0176) 65336367"; die Seite bietet ausdrücklich "E-Mail-Adresse hinzufügen" und "Website hinzufügen" an, führt also weder eine E-Mail-Adresse noch eine Website und keine Öffnungszeiten ("keine Öffnungszeiten"). Eintragsdaten 30.12.2022, Aktualisierung 03.10.2026; das eingestellte Logo trägt die Aufschrift "Schrott-Nowak.de" (hochgeladen 13.12.2020). Damit ist die Bonnekampstraße 14 durch ein zweites, vom Webarchiv-Impressum unabhängiges Verzeichnis gestützt — beide Quellen sind weiterhin keine Betreiber-Primärquelle und keine tagesaktuelle Tätigkeitsbestätigung. Quelle(n): https://www.11880.com/branchenbuch/essen-ruhr/060440090B102234372/schrott-nowak-de-miroslaw-nowak.html]
- [Recherche 30.10.2026: Live-OSM-Straßenscan Bonnekampstraße, Rasterfenster 7.0625/51.4884 bis 7.0725/51.4944: 39 Adressobjekte mit `addr:housenumber` und `addr:street`="Bonnekampstraße", daraus 38 verschiedene Hausnummern — 1, 7, 9, 11, 11a, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 49, 51, 53, 55. Das Objekt zur Kandidatenadresse ist way/276495449 mit `addr:housenumber`='14', `addr:street`='Bonnekampstraße', `addr:city`='Essen', `addr:postcode`='45327', `addr:country`='DE', `building`='apartments', `building:levels`='3', `roof:shape`='gabled' — kein `name=`, kein `operator=`; OSM beschreibt die Anschrift als dreigeschossiges Wohngebäude, passend zum Seed-Typ "Backyard". Im Raster trägt nur 1 von 39 Objekten `name=` oder `operator=`: way/245169554 `name`='Verkaufshalle' mit `building`='container' und `shop`='kiosk' (15) — ein fremder Betrieb, Treffer mit Bezug zu Schrott Nowak: 0. Quelle(n): OSM-API-Abfrage api.openstreetmap.org/api/0.6/map.json?bbox=7.0625,51.4884,7.0725,51.4944 (Abruf 30.10.2026)]
- [Recherche 30.10.2026: Preise: ANKAUF — der 11880-Eintrag nennt "Altmetallrecycling" als Branche, weist aber selbst darauf hin, dass die Preisspanne nur mit Euro-Symbolen angezeigt würde, "sofern uns diese Information vorliegt"; eine Spanne oder ein €/kg-Kurs wird nicht ausgegeben, es gibt keine Preisliste. VERKAUF — kein Verkauf von Sekundärrohstoffen oder Ersatzteilen belegt. GEBÜHREN — keine Entrümpelungs- oder Entsorgungsgebühren belegt; eine Abholung ist im Portal nicht beschrieben. Die Domain schrott-nowak.de bleibt unaufösbar, die letzte Betreiberfassung im Webarchiv stammt von 2017/2016; die Bonnekampstraße 14 bleibt damit eine belegte Kandidatenadresse ohne aktuellen Betriebsnachweis, `status: pruefung` und die zurückgenommene Fehlzeile bleiben unverändert. Quelle(n): https://www.11880.com/branchenbuch/essen-ruhr/060440090B102234372/schrott-nowak-de-miroslaw-nowak.html ; http://web.archive.org/web/20160206233754/http://schrott-nowak.de/Impressum.htm]


### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **51.4914351/7.0675324** (Mittel der 4 eindeutigen
  Way-Knoten von way/276495449; der Punkt liegt innerhalb der Objekt-BBox **51.4913625/7.0674270 – 51.4915076/7.0676376**).
  Quelle 1 (OSM): way/276495449 trägt `addr:housenumber`=**14**, `addr:street`=**Bonnekampstraße**, `addr:city`=Essen,
  `addr:country`=DE, `addr:postcode`=**45327**, `building`=apartments, `building:levels`=3 — Dossier-Straße,
  -Hausnummer und -PLZ stimmen objektgenau überein. Die Bonnekampstraße trägt live **60** verschiedene Hausnummern
  (1, 7, 9, 11, 11a, 13–45, 46, 48, 49, 51–59, 61–85 mit Lücken); im Umfeld trägt nur ein Objekt `name=`/`operator=`
  (way/245169554 `name`=Verkaufshalle, `shop`=kiosk) — **kein** Bezug zum Betrieb; Aussage zum OSM-Bestand, **kein**
  Negativbeleg. Quelle 2 (Betreiber/Register): das archivierte Betreiber-Impressum (Webarchiv-Fassung vom
  **06.02.2016**) nennt **Miroslaw Nowak, Bonnekampstraße 14, 45327 Essen** mit Steuernummer **DE260095030**; der
  unabhängige 11880-Eintrag (Eintrag 30.12.2022, Aktualisierung 03.10.2026) führt denselben Namen und dieselbe
  Anschrift mit **Bonnekampstr. 14, 45327 Essen (Katernberg)** und **(0176) 65336367**. **Abgrenzung:** die
  Betreiberquelle ist ein **Archivstand von 2016**, die Domain `schrott-nowak.de` ist nicht mehr auflösbar; ein
  **heutiger** Betrieb ist damit **nicht** belegt. Die Koordinate bezeichnet die **letzte dokumentierte
  Geschäftsanschrift** (Status `pruefung` bleibt, `status` wird nicht geändert); die im Seed geführte Fehlzeile
  „Marktstr. 14“ bleibt unangetastet. Der zuvor im Produktionsbestand gespeicherte Punkt **51.450284375/7.015053125**
  (Straße „Freiheit“, 45128 Essen) lag **5844 m** entfernt und war für diese Zeile falsch.
  **Hinweis zum Frontmatter:** der Clear-Token `geo` wurde aus `clear_fields` entfernt, weil die Zeile jetzt eine
  kuratierte Koordinate trägt — ein gleichzeitig gesetztes `clear_fields: 'geo'` wird vom Seed-Validator als
  Konflikt abgewiesen (`clear_fields conflicts with populated geo`).
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/276495449/full.json ;
  http://web.archive.org/web/20160206233754/http://schrott-nowak.de/Impressum.htm ;
  https://www.11880.com/branchenbuch/essen-ruhr/060440090B102234372/schrott-nowak-de-miroslaw-nowak.html (Abruf 03.11.2026).]

### Recherche 07.11.2026

- [Recherche 07.11.2026: **Das amtliche Fachbetrieberegister kennt in 3 km um die Bonnekampstraße 14 keinen Betrieb „Nowak“ — die Namenssuche liefert genau 2 Efb-Treffer, beide ein Fremdbetrieb in Falkensee, und die Afv-Route 0.** Live am 07.11.2026 über die clientseitig gerenderte Vaadin-App (Route „Entsorgungsfachbetriebe“, Filter „Art des Betriebs = Egal“, `is=1&e=1`): Namenssuche `n=Nowak` -> **2 Treffer**, nämlich „**Rohrreinigung für Haus und Industrie Firma Wolfgang Nowak**“, Münchener Straße 18, 14612 Falkensee, ZZHT00700106 / 1 (Sammeln, Befördern, Handeln, Makeln) und „**U. Nowak - Lohnunternehmen**“, Bistard 4c, 41751 Viersen, ZZPT00300027 / 1 (Sammeln, Befördern) — **zwei andere Rechtsträger an zwei anderen Orten** (Rohrreinigung bzw. Lohnunternehmen, nicht Schrott); Route „Altfahrzeugverwertung“ `n=Nowak` -> **0 Treffer**. Standortsuche um **Bonnekampstraße 14, 45327 Essen** (OSM/Nominatim-Punktlage 51.4914352/7.0675323, Ortsteil Katernberg): Efb-Route mit 3 km -> **17 Treffer**, mit 5 km -> **48 Treffer**; Afv-Route mit 3 km -> **4 Treffer**, mit 5 km -> **4 Treffer**. Die nächstliegenden Einträge sind **Fremdbetriebe**: „**Schönmackers Umweltdienste GmbH & Co. KG Betrieb Essen**“, Bonifaciusstraße 160, 45309 Essen, ZZET01700101 / 1-7 (2,3 km); „**Mill Container- und Entsorgungsdienst GmbH**“, Manderscheidtstraße 91-95, 45141 Essen, ZZEE00200119 / 1-2 (2,4 km); in der Afv-Route „**WRL GmbH**“, Emscherstrasse 27, 45327 Essen, ZZBT00300361 / 1 und / 3 (2,7 km) — **weder** die Bonnekampstraße 14 **noch** eine andere Anschrift trägt den Namen „Nowak“. **Reichweite:** die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung besitzt, und **kein** Existenz- oder Schließungsbeweis. **Das ändert nichts an der Kernlage:** die Dossier-Adresse stammt aus einem **Archiv-Impressum von 2016** und einem **11880-Eintrag von 2022**; die Domain `schrott-nowak.de` bleibt unaufösbar, ein heutiger Betrieb ist **weiterhin nicht belegt**. `certifications` bleibt leer, `status: pruefung` bleibt unverändert, die Koordinate bleibt als „letzte dokumentierte Geschäftsanschrift“ stehen (sie wurde am 03.11.2026 gesetzt). Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Nowak ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&o=5&n=Nowak ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Bonnekampstra%C3%9Fe%2014%2C%2045327%20Essen…&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Bonnekampstra%C3%9Fe%2014%2C%2045327%20Essen…&dst=3&is=1&e=1&o=5 (Abruf 07.11.2026).]
- [Recherche 07.11.2026: **Die USt-ID DE 260095030 ist über den amtlichen EU-Dienst VIES weiterhin gültig; VIES gibt weder Namen noch Anschrift preis — der Betrieb bleibt in der Identität ungeklärt, die Preise bleiben unbelegt.** Die am 07.11.2026 über `https://ec.europa.eu/taxation_customs/vies/rest-api/ms/DE/vat/260095030` geprüfte Umsatzsteuer-Identifikationsnummer **DE 260095030** liefert `isValid: true`, `userError: VALID` (Anfrage-Datum 2026-11-07); `name` und `address` werden mit „---“ maskiert ausgegeben. Das bestätigt die **Gültigkeit der USt-Identität** (unverändert gegenüber dem Befund vom 27.10.2026), **nicht** deren Inhaber oder Anschrift. **Preise:** für den Dossierbetrieb ist am 07.11.2026 **kein** Ankauf-, Verkaufs- oder Gebührenwert belegt worden — der 11880-Eintrag „Schrott-Nowak.de Miroslaw Nowak“ (Branche „Altmaterialhandel / Schrotthandel / Altmetallrecycling“) weist selbst darauf hin, dass die Preisspanne nur mit Euro-Symbolen angezeigt würde „sofern uns diese Information vorliegt“, und weist sie **nicht** aus; eine Abholung ist dort nicht beschrieben, es gibt **keine** Entrümpelungs- oder Entsorgungsgebühr. Die **Archiv**-Fassung des Betreiber-Impressums (06.02.2016) und der **Webarchiv**-Stand bleiben die jüngsten Betreiberbelege. **`street`/`postcode` bleiben leer** (die Bonnekampstraße 14 bleibt ein historisches Lead, kein bestätigter aktueller Standort), `status: pruefung` bleibt unverändert. Quelle(n): https://ec.europa.eu/taxation_customs/vies/rest-api/ms/DE/vat/260095030 ; https://www.11880.com/branchenbuch/essen-ruhr/060440090B102234372/schrott-nowak-de-miroslaw-nowak.html ; http://web.archive.org/web/20160206233754/http://schrott-nowak.de/Impressum.htm (Abruf 07.11.2026).]

### Recherche 08.11.2026

- [Recherche 08.11.2026: **Die Archivlage ist präzisiert: das letzte Impressum im Webarchiv stammt vom 06.11.2016, die letzte erfolgreich gespeicherte Startseite vom 24.04.2018; danach gibt es nur noch Weiterleitungen und einen Serverfehler.** Die CDX-Snapshotliste für schrott-nowak.de (abgerufen am 08.11.2026 über http://web.archive.org/cdx/search/cdx?url=schrott-nowak.de&output=text&fl=timestamp,original,statuscode&collapse=timestamp:6) umfasst 15 Zeitpunkte von 10.06.2013 bis 13.11.2018; Status 200 haben die Fänge vom 10.06.2013, 29.04.2014, 01.05.2014, 18.12.2014, 10.01.2016, 10.03.2016, 07.10.2016, 14.10.2017 und 24.04.2018, die Fänge ab 12.06.2017 sind 301-Weiterleitungen, der letzte vom 13.11.2018 trägt den Status 502. Die archivierte Impressumsseite http://web.archive.org/web/20161106192730/http://schrott-nowak.de/Impressum.htm (live gelesen am 08.11.2026) nennt unverändert „Inhaber und Verantwortlich für den Inhalt: Miroslaw Nowak, Bonnekampstrasse 14, 45327 Essen“; für das Impressum existieren im Archiv nur die beiden Fänge vom 06.02.2016 und vom 06.11.2016. Die Startseitenfassung vom 24.04.2018 läuft noch auf einem „kostenlose Homepage“-Anbieter und zeigt den Hinweis „Urlaub!!!“. Die im Dossier formulierte Angabe, die letzte Betreiberfassung stamme von 2017, ist damit auf **Impressum 06.11.2016** und **Startseite 24.04.2018** zu verfeinern; die Domain schrott-nowak.de bleibt nicht mehr auflösbar. Quelle(n): http://web.archive.org/cdx/search/cdx?url=schrott-nowak.de&output=text&fl=timestamp,original,statuscode&collapse=timestamp:6 ; http://web.archive.org/web/20161106192730/http://schrott-nowak.de/Impressum.htm ; http://web.archive.org/web/20180424222250/http://www.schrott-nowak.de/ (Abruf 08.11.2026).]
- [Recherche 08.11.2026: **Die strukturierten Portaldaten bestätigen die Bonnekampstraße 14 objektgenau; die am 03.11.2026 gesetzte Koordinate entspricht der Portalangabe, und die Nummer steht nur in diesem einen Verzeichnis.** Der 11880-Eintrag „Schrott-Nowak.de Miroslaw Nowak“ wurde am 08.11.2026 live gelesen; die strukturierten Daten (schema.org, @type RecyclingCenter) nennen postalCode 45327, addressLocality Essen, streetAddress „Bonnekampstr. 14“, die Koordinate 51.491435/7.067532, die Branche „Altmaterialhandel“, Eintragsdaten 30.12.2022 und Portalaktualisierung 03.10.2026; der Eintragskopf bietet „E-Mail-Adresse hinzufügen“ und „Website hinzufügen“ an. Die Portal-Koordinate deckt sich mit der Koordinate des OSM-Objekts way/276495449, dessen Tags addr:housenumber=14, addr:street=Bonnekampstraße, addr:city=Essen, addr:postcode=45327 und building=apartments lauten und das damit als Wohngebäude ohne Betriebsbezug ausweisen. Die Felder `street` und `postcode` bleiben leer, weil die belegte Anschrift ein Archivstand von 2016 ist und ein heutiger Betrieb nicht belegt ist. Quelle(n): https://www.11880.com/branchenbuch/essen-ruhr/060440090B102234372/schrott-nowak-de-miroslaw-nowak.html ; https://api.openstreetmap.org/api/0.6/way/276495449/full.json (Abruf 08.11.2026).]
- [Recherche 08.11.2026: **Preise getrennt geführt: für Schrott Nowak ist am 08.11.2026 kein Ankaufskurs, keine Verkaufsliste und keine Gebührenangabe belegt.** Der live gelesene 11880-Eintrag nennt die Branchen „Altmaterialhandel / Schrotthandel / Altmetallrecycling“, weist aber selbst darauf hin, dass eine Preisspanne nur mit Euro-Symbolen angezeigt würde, „sofern uns diese Information vorliegt“, und zeigt **keine** an. **Ankauf:** kein €/kg-Kurs und keine Materialpreisliste. **Verkauf:** kein Verkauf von Sekundärrohstoffen oder Ersatzteilen belegt. **Gebühren:** keine Entrümpelungs-, Abhol- oder Entsorgungsgebühr beziffert; eine Abholung ist im Verzeichnis nicht beschrieben. `status` bleibt `pruefung`; die Koordinate bezeichnet weiterhin nur die letzte dokumentierte Geschäftsanschrift. Quelle(n): https://www.11880.com/branchenbuch/essen-ruhr/060440090B102234372/schrott-nowak-de-miroslaw-nowak.html (Abruf 08.11.2026).]
