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

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

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
