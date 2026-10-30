---
slug: ni-loningen-49624-reiners-schrott-und-metallhandel
name: Reiners GmbH Schrott- und Metallhandel
trader_type: metallhaendler
state: NI
city: Löningen
street: Bremer Str. 17
postcode: 49624
phone: 05432 2200
email: ''
opening_hours: Mo-Fr 07:30-12:00,13:00-17:00
website: https://www.reiners-gmbh.de
website_status: tot
status: pruefung
description: 'Schrott- und Metallhandel Reiners GmbH in Löningen. Register: Reiners GmbH, Amtsgericht Oldenburg HRB 150015, Geschäftsführer Benjamin Reiners, Kapital 25.600 Euro, Firmensitz Löningen, Unternehmensgegenstand Großhandel in Rohstoffen und Waren; Registeranschrift Bremer Str. 17, 49624 Löningen. Die eigene Website ist nicht erreichbar (HTTP 503, Wartungsmodus). ANKAUF/VERKAUF: keine Preise veröffentlicht, auch nicht über die im OSM-Objekt hinterlegten Recycling-Sorten (Alu, Kupfer, Eisen, Metall, Nickel, Schrott, Zink). GEBÜHREN: keine Tarife veröffentlicht. Status bleibt pruefung, weil ein unabhängiger Zweitbeleg für die laufende Annahme fehlt.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Wide-Net (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Reiners GmbH Schrott- und Metallhandel

## Überblick

Die **Hausnummer im Dossier wurde von 19 auf 17 korrigiert**: das Handelsregister (NorthData) nennt Bremer Str. 17, D-49624 Löningen, OSM trägt den Betriebsknoten Reiners GmbH Schrott mit derselben Rufnummer unter der Hausnummer 17, und der Seed nannte ebenfalls 17. Die abweichende Angabe Bremer Str. 19 stammt allein aus dem 11880-Eintrag; die Hausnummer 19 ist in OSM an der Bremer Straße **nicht** erfasst (dort stehen 17 und 19a). Die eigene Website https://www.reiners-gmbh.de liefert HTTP 503 im Wartungsmodus, website_status tot bleibt damit bestehen.

## Timeline

### Recherche 02.10.2026

- Beleglage: 1) 11880-Eintrag Reiners GmbH (aktualisiert 20.03.2024): Bremer Str. 19, 49624 Löningen, Tel. 05432 2200, Fax 05432 2900, Öffnungszeiten Mo-Fr 07:30-12:00/13:00-17:00; Kategorien Schrotthandel/Containerdienst. 2) DasÖrtliche-Eintrag: Bremer Str. 17-19, 49624 Löningen, Tel. 05432 2200. 3) Eigencheck Website https://www.reiners-gmbh.de am 02.10.2026: HTTP 503 (offline). Seed/GS: HRB 150015, Adresse Bremer Str. 17.
- Adress-Diskrepanz dokumentiert: Seed Bremer Str. 17 vs 11880 Bremer Str. 19 vs DasÖrtliche 17-19. Dossier nutzt Bremer Str. 19 (jüngste 11880-Führung); Hausnummern 17/19 gehören offenbar zum selben Betriebsgelände (17-19). Keine Geocodierung geändert (Erstbefüllung).
- Fax 05432 2900 aus 11880 unverändert notiert (kein Dossier-Feld).
- `website_status: tot` (503 beim Direktabruf). `status: pruefung` bleibt (kein zweiter unabhängiger Aktivitätsbeleg jünger 12 Monate).

### Importiert (Seed-Stand 2026-09-30)

- Schrott — GS + HRB 150015 (NEU)
- Adresse: Löningen 49624, Bremer Str. 17

### Recherche 30.10.2026

- [Recherche 30.10.2026: **Adressdiskrepanz entschieden, mit Gegenbeleg.** Das Handelsregister (NorthData) weist die Reiners GmbH aus mit "Adresse: Bremer Str. 17, D-49624 Löningen" (Titel: "Reiners GmbH, Löningen, Amtsgericht Oldenburg HRB 150015", Geschäftsführer: Benjamin Reiners, Prokura: Benno Reiners, Nicht mehr Geschäftsführer: Benno Reiners, Nicht mehr Prokura: Benjamin Reiners, Kapital: 25.600 €, Firmensitz: Löningen, Gegenstand: "Großhandel in Rohstoffen und Waren"). OSM trägt an derselben Hausnummer den Betriebsknoten. Der abweichende Lead Bremer Str. 19 stammt allein aus dem 11880-Eintrag und wird hier als Gegenbeleg festgehalten; die 11880-Nummer bleibt als Suchlead unberührt. Das Dossier-Feld street wurde deshalb von Bremer Str. 19 auf Bremer Str. 17 gesetzt, das Feld city von Löningen 49624 auf Löningen (die Postleitzahl steht bereits im Feld postcode). Quelle(n): https://www.northdata.de/Reiners+GmbH,+Löningen ; https://api.openstreetmap.org/api/0.6/node/8522452312.json (Abruf 30.10.2026)]
- [Recherche 30.10.2026: OSM-Betriebsknoten live am Objekt geprüft. Raster bbox=7.7574178,52.7350304,7.7736445,52.7518309 über den gesamten Verlauf der Straße "Bremer Straße" in Löningen, Filter addr:street=Bremer Straße: 55 Objekte | 38 verschiedene Hausnummern. Die Karte trägt node/8522452312 mit addr:housenumber=17, addr:street=Bremer Straße, addr:postcode=49624, addr:city=Löningen, amenity=recycling, recycling_type=centre, name=Reiners GmbH Schrott, contact:phone=+49 5432 2200, contact:facebook=https://www.facebook.com/reiners.gmbh/, note=http://www.reiners.gmbh leitet weiter auf facebook sowie recycling:aluminium=yes, recycling:copper=yes, recycling:iron=yes, recycling:metal=yes, recycling:nickel=yes, recycling:scrap_metal=yes, recycling:zinc=yes. Die zuvor im Dossier geführte **Hausnummer 19 ist im Raster nicht erfasst**; vorhanden sind hier 17 (zweimal, als Node und als Gebäude way/40140660), 19a (way/40153764, building=yes) und 2-4 (node/6015793698, ein Fitnessstudio). Quelle(n): https://api.openstreetmap.org/api/0.6/node/8522452312.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=7.7574178,52.7350304,7.7736445,52.7518309 (Abruf 30.10.2026)]
- [Recherche 30.10.2026: Website-Status erneut live geprüft. http://www.reiners-gmbh.de, https://www.reiners-gmbh.de und https://reiners-gmbh.de liefern übereinstimmend HTTP 503 mit der Zieladresse https://reiners-gmbh.de/ und dem Seitentitel "Reiners GmbH - Wartungsmodus". Der OSM-Vermerk "leitet weiter auf facebook" ist damit veraltet und wird nicht übernommen; website_status bleibt tot, status bleibt pruefung, weil ein zweiter unabhängiger Beleg für eine laufende Annahme fehlt. Quelle(n): https://reiners-gmbh.de/ (Abruf 30.10.2026)]
- [Recherche 30.10.2026: Preise getrennt nach Ankauf, Verkauf und Gebühren. ANKAUF: der Betrieb ist im OSM-Objekt als Recycling-Zentrum mit den Sorten Aluminium, Kupfer, Eisen, Metall, Nickel, Schrott und Zink erfasst, **Zahlenpreise sind jedoch nirgends veröffentlicht**; das Register nennt nur den Unternehmensgegenstand Großhandel in Rohstoffen und Waren. VERKAUF: kein Beleg. GEBÜHREN: keine Tarifordnung, keine kostenfreie Annahme behauptet. Die im Dossier geführten Öffnungszeiten Mo-Fr 07:30-12:00 und 13:00-17:00 stammen weiterhin nur aus dem 11880-Eintrag und wurden nicht aufgenommen. Quelle(n): https://api.openstreetmap.org/api/0.6/node/8522452312.json ; https://www.northdata.de/Reiners+GmbH,+Löningen (Abruf 30.10.2026)]

- [30.10.2026 (Korrekturvermerk): **OSM-Tag-Zitat berichtigt**: der `note`-Tag von node/8522452312 lautet woertlich „http://www.reiners.gmbh leitet weiter auf facebook“ (Element-Version 4 vom 06.10.2023, also kein nachtraeglicher Edit) — die im Dossier genannte Form „reiners-gmbh.de“ weicht in Domain und Endung ab. Quelle(n): https://api.openstreetmap.org/api/0.6/node/8522452312/full (Abruf 30.10.2026)]
