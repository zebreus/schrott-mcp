---
slug: nw-essen-metallion
name: Metallion GmbH
trader_type: metallhaendler
state: NW
city: Essen
street: Heßlerstr. 77
postcode: '45329'
phone: 0201 344344
email: info@metallion.de
opening_hours: 'Mo-Sa 08:00-20:00 (Annahmeschluss 19:00)'
website: https://metallion.de/aluminum-ankauf
website_status: ''
status: aktiv
description: ''
dropoff_json: '{"allowed": true, "customer_types": ["privat", "gewerbe"], "conditions": "Anlieferung Mo-Sa 08:00-20:00, Annahmeschluss 19:00; kostenlose Abholung ab 500 kg (Betreiber-Website, Feedback-Triage 01.10.2026)"}'
pickup_json: '{"allowed": true, "conditions": "Kostenlose Abholung ab 500 kg inkl. mobiler Waage (Betreiber-Website, Feedback-Triage 01.10.2026)"}'
clear_fields: 'geo'
provenance_section: Essen / Mülheim / Oberhausen / Bottrop
provenance_ankauf_raw: ja
provenance_origin: table
---

# Metallion GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Owner-Korrektur 27.10.2026: GEO-Rücknahme

- [Owner-Korrektur 27.10.2026: Die read-only-Abfrage von `public.db` ergab für Metallion den gespeicherten Punkt 51.450284375, 7.015053125. Die Nominatim-Rückwärtssuche löst genau diesen Punkt als Freiheit, Essen 45128 (Straßenobjekt, kein Metallion-Betrieb) auf. Die aktuelle Betreiberseite identifiziert Metallion GmbH, Heßlerstr. 77, 45329 Essen; die Ankaufseite belegt ausdrücklich die Annahme auch von privat und gewerblich sowie Anlieferung, und das Impressum nennt HRB 34136 AG Essen. Damit liegt der Produktionspunkt nicht an der belegten Annahmestelle. `clear_fields: 'geo'` zieht nur das alte Paar beim nächsten regulären Seed ein; keine Ersatzkoordinate gesetzt, Historie erhalten. Quellen/Abruf 27.10.2026: https://metallion.de/impressum/ ; https://metallion.de/aluminum-ankauf ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; read-only `public.db`; `docs/trader-seed.md`.]

### Importiert (Seed-Stand 2026-09-30)

- NE-Metalle, Alu-Spezialist (Heßlerstr. 77)

### Recherche 01.10.2026

- [Feedback-Triage 01.10.2026 (ID 2450): BEWIESENE KORREKTUREN — postcode '45329', phone 0201 344344, email info@metallion.de, hours 'Mo-Sa 08:00-20:00 (Annahmeschluss 19:00)'. Impressum nennt GmbH + HRB 34136 AG Essen, GF Ahmad Merhi (Owner-Primärquelle); Header/Footer/Ankaufseiten bestätigen. Privatannahme belegt ('Ob privat oder gewerblich') → dropoff_json privat/gewerbe; Abholung ab 500 kg → pickup_json. Quelle(n): https://metallion.de/aluminum-ankauf, https://metallion.de/impressum/ (Abruf 01.10.2026).]
