---
slug: nw-essen-schrott-nowak
clear_fields: geo
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

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

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
