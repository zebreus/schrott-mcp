---
slug: nw-gelsenkirchen-tsr-deutschland-ndl-gelsenkirchen
name: TSR Deutschland GmbH & Co. KG (Ndl. Gelsenkirchen)
trader_type: schrotthaendler
state: NW
city: Gelsenkirchen
street: Grimbergstr. 85
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://www.tsr-recycling.de/standorte/schrotthaendler-gelsenkirchen/
website_status: blockiert
status: aktiv
description: ''
dropoff_json: '{"allowed": true, "customer_types": ["gewerbe"], "conditions": "Keine Annahme von Privatkunden; gewerbliche Anlieferung, Entladung von Containerware (TSR-Standortseite, 30.09.2026)"}'
pickup_json: '{"allowed": true, "conditions": "Abholung vor Ort bei groesseren Mengen mit eigenen Spezialfahrzeugen (TSR-Standortseite, 30.09.2026)"}'
provenance_seed_file: nw
provenance_section: Ruhrgebiet – Großrecycler / Stahlwerksnah (Duisburg, Dortmund,
  Gelsenkirchen, Lünen, Essen)
provenance_ankauf_raw: ja (nur Gewerbe)
provenance_origin: table
---

# TSR Deutschland GmbH & Co. KG (Ndl. Gelsenkirchen)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrottankauf, Kabelzerlegung (Grimbergstr. 85)

### Korrektur 30.09.2026 (Feedback-Triage)

- [Korrektur Feedback fb1857: keine Privatannahme mit Website-Beleg — TSR-Seite: Waage „Anmerkung: Keine Annahme von Privatkunden", „Gewerbliche Anlieferung: Mo–Fr 07:00–15:00 (Entladung von Containerware)" → dropoff_json (gewerbe only); „Abholung vor Ort bei größeren Mengen mit eigenen Spezialfahrzeugen" → pickup_json; Quelle: tsr-recycling.de/standorte/schrotthaendler-gelsenkirchen (30.09.2026, via Jina-Reader)]
- [Korrektur Feedback fb1857: website auf exakte Standortseite präzisiert (vorher generisch /standorte); website_status blockiert (Human-Challenge, verifiziert 30.09.2026); Quelle: Direktabruf]
- (Einzelbeleg, unsicher): TSR-Seite nennt „Grimbergstrasse 85, 45889 Gelsenkirchen, T +49 2093 8420-00" — dafür kein Zweitbeleg (Gelbe Seiten: kein GE-Eintrag; schrottplaetze.org: keine GE-Seite) und Vorwahl 02093 unüblich (möglich: Tippfehler für 0209 ...) → PLZ/Telefon NICHT übernommen, street/Adressbeleg Grimbergstr. 85 (Seed) bleibt; Waage-Zeiten (Mo–Fr 07:00–15:30) ebenfalls nur TSR-Seite → opening_hours leer.
