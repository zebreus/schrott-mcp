---
slug: nw-essen-wrl
lat: 51.5016982
lon: 7.0321982
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

WRL GmbH, Emscherstr. 27, 45327 Essen, HRB 19378 Amtsgericht Essen: Schrott- und Metallhandel
mit Containerdienst; die Betreiberseite verortet die Betriebe in Katernberg und lädt zur
Materialabgabe in den veröffentlichten Öffnungszeiten ein. Die Identität tragen das
Betreiberimpressum, der Footer mit derselben Anschrift und ein Registerspiegel, dessen
älterer PLZ-Stand 45326 lautet. In OpenStreetMap ist die Gesellschaft als eigenes
Büroobjekt ohne Hausnummer geführt; die Hausnummer 27 fehlt dort, der gesetzte Punkt
bezeichnet das benannte Gebäude gegenüber der Anschrift 28. Offen bleiben die
ungeklärte Hausnummer, der Widerspruch der beiden PLZ-Angaben und der nicht belegte
Ankaufspreis.

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




### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **51.5016982/7.0321982** (Mittel der 6 eindeutigen
  Way-Knoten von way/1057464915; der Punkt liegt innerhalb der Objekt-BBox **51.5016335/7.0321296 – 51.5017580/7.0322667**).
  Quelle 1 (OSM): way/1057464915 trägt `name`=**WRL GmbH**, `office`=company, `website`=**https://www.wrl-essen.de/**,
  `building`=yes — Name und Website stimmen mit dem Dossier-Betrieb (WRL GmbH, `website` https://www.wrl-essen.de)
  objektgenau überein, der Betriebsbezug ist also unmittelbar am Objekt belegt. **Hausnummer:** für die
  Dossier-Adresse **Emscherstraße 27** existiert in OSM **kein** Adressobjekt; die live erfassten Hausnummern der
  Emscherstraße (Kachelraster **7.0200/51.4900 – 7.0400/51.5200**) lauten u. a. 2, 4, 10, 12, 28, 32, 34, 35, 36, 38
  und weiter 80–212 — **die 27 fehlt**. Das benannte Betriebsgebäude liegt unmittelbar an der Emscherstraße gegenüber
  dem Adressobjekt **Emscherstraße 28** (way/369120064) und ist der einzige Objektkörper mit Betriebsbezug in diesem
  Umfeld; gesetzt wird deshalb das Objekt mit dem **Betriebsbezug**, nicht eine geratene Hausnummer. Quelle 2
  (Betreiber/Register): das Impressum der Betreiber-Website nennt **WRL GmbH, Emscherstr. 27, HRB 19378 AG Essen**, der
  Website-Footer nennt **„Emscherstrasse 27 | 45327 Essen“**, und die Startseite lädt ausdrücklich zur Materialabgabe an
  den veröffentlichten Öffnungszeiten ein. Der zuvor im Produktionsbestand gespeicherte Punkt **51.450284375/7.015053125**
  (Straße „Freiheit“, 45128 Essen) lag **5839 m** entfernt und war für diese Zeile falsch.
  **Hinweis zum Frontmatter:** der Clear-Token `geo` wurde aus `clear_fields` entfernt, weil die Zeile jetzt eine
  kuratierte Koordinate trägt — ein gleichzeitig gesetztes `clear_fields: 'geo'` wird vom Seed-Validator als
  Konflikt abgewiesen (`clear_fields conflicts with populated geo`).
  Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/1057464915/full.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0200,51.4900,7.0400,51.5200 ;
  https://www.wrl-essen.de/impressum/ ; https://www.wrl-essen.de/ (Abruf 03.11.2026).]
