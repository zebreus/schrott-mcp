---
slug: nw-essen-wrl
clear_fields: 'geo'
name: WRL GmbH
trader_type: schrotthaendler
state: NW
city: Essen
street: Emscherstr. 27
postcode: '45327'
phone: 0201 2899134
email: info@wrl-essen.de
opening_hours: Mo-Fr 7:00-17:00, Sa 7:00-16:00
website: https://www.wrl-essen.de
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Neu, Ankauf unklar (nur Verzeichnis-/Profilbeleg)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# WRL GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Owner-GEO-Korrektur 27.10.2026

- [Owner-GEO-Korrektur 27.10.2026: Die ausschließlich lesend abgefragte Produktionszeile enthält `51.450284375, 7.015053125`; Nominatim-Reverse ordnet den Punkt der Freiheit im Essener Südviertel zu. Die Betreiber-Startseite und das Impressum identifizieren die WRL GmbH an Emscherstr. 27, 45327 Essen; die Startseite verortet die Betriebe in Katernberg und lädt ausdrücklich zur Materialabgabe an den veröffentlichten Öffnungszeiten ein. Das widerspricht dem Südviertel-Pin; eine genaue Zufahrt/Einfahrt ist damit nicht festgelegt. `clear_fields: 'geo'` entzieht den Altpunkt beim nächsten regulären Seed. Keine Ersatzkoordinate. Quellen: https://www.wrl-essen.de/ ; https://www.wrl-essen.de/impressum/ ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?format=jsonv2&street=Emscherstrasse%2027&city=Essen&postalcode=45327&country=Germany&addressdetails=1 ; read-only `/var/lib/schrott-mcp/public.db` (27.10.2026); `docs/trader-seed.md`.]

### Importiert (Seed-Stand 2026-09-30)

- Emscherstr. 27, nur schrottradar-Profil
- urspr. Website-Angabe: keine gefunden [Recherche 30.09.2026: HR-Fund via Northdata, PLZ bestätigt (Straße beibehalten); Quelle: northdata.de/WRL]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Frontmatter gefüllt (phone, email, website, website_status aktiv, opening_hours) per Owner-Ausnahme — Betreiber-Impressum (https://www.wrl-essen.de: WRL GmbH, Emscherstr. 27, HRB 19378 AG Essen) HR-kongruent, HR-Spiegel aktiv-konform; Öffnungszeiten Mo-Fr 07:00-17:00 (Annahme bis 16:30), Sa 07:00-16:00 (Annahme bis 15:30); Straße/PLZ NICHT angefasst — HR-Spiegel nennt PLZ 45326 abweichend zum Impressum (PLZ-Konflikt, Klärfall); Quelle(n): Betreiber-Impressum wrl-essen.de, HR-Spiegel online-handelsregister.de]

### Recherche 01.10.2026 (Triage Feedback 2564)

- [Recherche 01.10.2026: Feedback 2564 berechtigt — Betreiber-Footer (aktuell, © 2026): „Emscherstrasse 27 | 45327 Essen" → PLZ-Konflikt zugunsten der Betreiber-Primärquelle aufgelöst (postcode 45326 → 45327); street-Fix (Firmen-/Ortspräfix entfernt → „Emscherstr. 27"); HR-Spiegel-45326 als überholter Registerstand vermerkt; Quelle(n): https://www.wrl-essen.de/ (Footer)]
