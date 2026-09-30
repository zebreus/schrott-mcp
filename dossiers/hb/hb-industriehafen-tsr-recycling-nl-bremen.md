---
slug: hb-industriehafen-tsr-recycling-nl-bremen
name: TSR Recycling GmbH & Co. KG, NL Bremen
trader_type: metallhaendler
state: HB
city: Industriehäfen
street: Waterbergstr. 6
postcode: '28237'
phone: 0421 64903-0
email: ''
opening_hours: Mo–Do 06:00–16:15, Fr 06:00–15:15
website: https://www.tsr-recycling.de/standorte/schrotthaendler-bremen/
website_status: blockiert
status: aktiv
description: ''
dropoff_json: '{"allowed": true, "customer_types": ["gewerbe"], "days": ["Mo", "Di", "Mi", "Do", "Fr"], "time_windows": ["06:00-16:15"], "conditions": "Keine Privatkundenannahme; Fr Annahmeschluss 15:15 (TSR-Standortseite, 30.09.2026)"}'
pickup_json: ''
provenance_seed_file: hb
provenance_section: Bremen (Stadt)
provenance_ankauf_raw: ja (Gewerbe)
provenance_origin: table
---

# TSR Recycling GmbH & Co. KG, NL Bremen

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Stahlschrott/NE-Metalle, Großmengen, v. a. Gewerbe
- Adresse: Industriehäfen, Waterbergstr. 6

### Korrektur 30.09.2026 (Feedback-Triage)

- [Korrektur Feedback fb1808: PLZ 28237 + Tel. 0421 64903-0 + Zeiten Mo–Do 06:00–16:15 / Fr 06:00–15:15 vierquellig (TSR-Standortseite via Jina-Reader + dasoertliche.de + golocal.de, alle „Waterbergstr. 6, 28237 Bremen", Tel. 0421 64903-0); Quelle: tsr-recycling.de/standorte/schrotthaendler-bremen + dasoertliche.de/Themen/TSR-Recycling-Niederlassung-Bremen + golocal.de/bremen/schrotthandel/tsr-deutschland-niederlassung-bremen-Tsr]
- [Korrektur Feedback fb1808: Privatanlieferung NICHT möglich mit Website-Beleg — TSR-Seite: „Anmerkung: Keine Privatkundenannahme möglich", Kundenkreis Gewerbe/Handwerk/Bau → dropoff_json (gewerbe only); Quelle: tsr-recycling.de/standorte/schrotthaendler-bremen (30.09.2026)]
- [Korrektur Feedback fb1808: website_status blockiert persists — tsr-recycling.de antwortet automatisiert weiter nur mit Human-Challenge (curl 30.09.2026); TSR-Inhalte nur via Reader-Proxy prüfbar; Quelle: Direktabruf]
- (Klärfall): schrottplaetze.org nennt abweichende Zeiten (Mo–Do 07:00–15:45, Fr 07:00–12:15) — gegen 3 übereinstimmende Quellen (TSR-Seite, dasoertliche, golocal) unterlegen, daher nicht übernommen.
