---
slug: bb-templin-benzin-paul-schrotthandel
clear_fields: 'dropoff_json,geo,phone'
name: Benzin Paul Schrotthandel
trader_type: schrotthaendler
state: BB
city: Templin
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
provenance_section: Schrottankauf Brandenburg (BB) – Recherche
provenance_ankauf_raw: ja
provenance_origin: table
---

# Benzin Paul Schrotthandel

## Überblick

Der Seed nennt „Benzin Paul Schrotthandel“, führt die Stadt aber als Templin und die ungeklärte Adresse Brüssower Allee 90 in Prenzlau. Schrottplatz-Info listet diesen Namen dort sogar mit zwei unterschiedlichen Telefonnummern. Die Betreiberseite an derselben Hausnummer gehört jedoch ausdrücklich der Prenzlauer Schrott-Recycling GmbH (PSR), HRB 10921, und nennt deren eigene Kontaktdaten. Eine Beziehung, Untermiete oder Identität Benzin = PSR ist nicht belegt; die konkurrierende Adresse und PLZ wurden deshalb aus den Stammdaten entfernt, Stadt Templin bleibt ungeklärter Seed-Kontext, `status: pruefung`.

**Preise:** ANKAUF — für Benzin keine verifizierten Preise. PSR hat an der konkurrierenden Adresse eine „Schrottpreise“-Seite, nennt dort aber nur telefonische Tagespreisauskunft; das ist PSR und keine Benzin-Preisliste. VERKAUF — keine Liste. GEBÜHREN — keine Benzin-Tarife. **Offen:** ob Benzin ein eigener Betrieb ist, korrekte Stadt/Adresse, Betreiber und aktueller Betrieb. **GEO:** keine Koordinaten, solange die Zielidentität und Annahmeadresse ungeklärt sind.

## Timeline

### Owner-GEO-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Die aktuelle Gegenprobe findet bei Gelbe Seiten und Das Örtliche „Benzin Paul Schrotthandel“ an Gottlieb-Daimler-Str. 1, 17268 Templin, jeweils mit 03987 40 92 81; Das Örtliche nennt außerdem paul.benzin@t-online.de. Beide Verzeichnisse zeigen werktägliche Öffnungszeiten und Schrottankauf, sind aber keine unabhängigen Betreiber-/Registerbelege. Schrottplatz-Info listet in Templin denselben Namen an Gottlieb-Daimler-Str. 1 mit 03987 5 28 21 sowie zweimal an Zehdenicker Str. 33 mit 03987 40 92 80 bzw. 03987 40 92 81; Angebot und Öffnungszeiten sind dort nicht erfasst. Die widersprüchlichen Standort-/Telefon-Leads lösen damit weder Identität noch aktuellen Kundenbetrieb auf; Adresse, Telefon, E-Mail und Zeiten werden nicht übernommen. Nominatim/OSM führt an Gottlieb-Daimler-Str. 1 einen gleichnamigen Recycling-POI; das ist nur ein Kartenlead, keine Betreiberbestätigung und kein GEO-Fill. Der im Read-only-Ownerabgleich vom 20.10. dokumentierte Produktionspunkt 53.3131636, 13.9321636 reverse-geokodiert heute zu einer unklassifizierten Straße in Grünow/Gramzow, Uckerfelde, nicht nach Templin; die OSM-Adresssuche für die frühere Brüssower Allee 90 ergibt einen anderen Punkt in Prenzlau. Eine heutige read-only-Abfrage der Produktionszeile fand zusätzlich den Platzhalter `phone=n.e.` und `dropoff_json={"allowed":true}`. Da keine Betreiberquelle den Altpunkt, die Telefonnummer oder die öffentliche Annahme bestätigt, werden genau diese drei gespeicherten Werte mit `clear_fields: 'dropoff_json,geo,phone'` beim nächsten regulären Seed zurückgenommen. Das bedeutet weder `allowed:false` noch Schließung. Stadt Templin bleibt ungeklärter Seed-Kontext; PSR-Daten werden nicht übertragen. Quellen (Abruf 26.10.2026): https://www.gelbeseiten.de/gsbiz/5a3c203b-b1be-42b8-8570-18ab944682f8 ; https://www.dasoertliche.de/Themen/Benzin-Paul-Schrotthandel-Templin-Gottlieb-Daimler-Str ; https://www.schrottplatz-info.de/schrottplatz/Templin/Benzin-Paul-Schrotthandel ; https://nominatim.openstreetmap.org/search?q=Gottlieb-Daimler-Str.%201%2C%2017268%20Templin%2C%20Germany&format=jsonv2&addressdetails=1 ; https://nominatim.openstreetmap.org/reverse?lat=53.3131636363636&lon=13.9321636363636&format=jsonv2&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Br%C3%BCssower%20Allee%2090%2C%2017291%20Prenzlau%2C%20Germany&format=jsonv2&addressdetails=1 ; Read-only-Snapshot dokumentiert in `research/wave-20261020-1801-owner-review.md`. OSM-Daten © OpenStreetMap-Mitwirkende, ODbL 1.0.]

### Recherche 20.10.2026

- [Recherche 20.10.2026: Schrottplatz-Info führt „Benzin Paul Schrotthandel“ an Brüssower Allee 90, 17291 Prenzlau und zeigt auf derselben Detailseite zwei Einträge mit verschiedenen Nummern (03984 83 27 19 und 03984 83 36 13); Angebot/Öffnungszeiten seien nicht erfasst. Die aktuelle Betreiberwebsite und ihr Impressum weisen Brüssower Allee 90 dagegen der Prenzlauer Schrott-Recycling GmbH, HRB 10921, zu; deren „Schrottpreise“-Seite veröffentlicht keine Zahlen, sondern verweist auf telefonische Tagespreisauskunft. Dies belegt weder eine Verbindung zu Benzin noch einen separaten Kundenbetrieb dort. Straße/PLZ entfernt, Status aktiv → pruefung; Stadt Templin bleibt als Seed-Angabe ungeklärt. PSR-Kontakte und Preise nicht übertragen. GEO gesperrt, keine Koordinaten gesetzt. Quellen (Abruf 20.10.2026): https://www.schrottplatz-info.de/schrottplatz/Prenzlau/Benzin-Paul-Schrotthandel ; https://www.psr-recycling.de/datenschutz--impressum ; https://www.psr-recycling.de/schrott-ankauf/schrottpreise .]

### Importiert (Seed-Stand 2026-09-30)

- Schrott
- Adressbeleg: recherche/bb.md Runde 2 (schrottplatz-info)
