---
slug: sn-niederwurschnitz-schrott-und-altpapierannahme
lat: 50.7181428
lon: 12.7534167
name: Schrott- und Altpapierannahme
trader_type: schrotthaendler
state: SN
city: Niederwürschnitz
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://schrottannahme-stollberg.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Schrott- und Altpapierannahme

## Überblick

Die Betreiberwebsite bezeichnet den Standort Niederwürschnitz als Annahmestelle für Altpapier, Schrott und Buntmetalle und nennt Stollberger Str. 34, 09399 Niederwürschnitz, Telefon 037296/928604 sowie konkrete Wochenzeiten. Das Impressum nennt Kevin Kühn an derselben Adresse. Die Betreiberwebsite ist derzeit der einzige qualifizierte Standortbeleg; daher bleiben Anschrift, Telefon, Mail und Öffnungszeiten aus dem Frontmatter leer. Zusätzlich hat der sichtbare E-Mail-Text `info@altpapier-stollberg.de` einen abweichenden `mailto:`-Zielwert `info@testzentrum-stollberg-west.de`; keine E-Mail übernehmen, bis dies geklärt ist.

**Preise:** **Ankauf** – Annahme von Schrott/Buntmetall belegt, aber keine Aussage zur Vergütung oder Zahlenkurse. **Verkauf** – keine Liste. **Gebühren** – keine Entsorgungs-/Annahmegebühren belegt; weder kostenlos noch bezahlt unterstellen. **GEO:** genaue Betreiberadresse und Kartenlink machen Geocoding prüfbar; Punkt vor Übernahme durch Owner/Standortprüfung bestätigen.

## Timeline

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 50.725, 12.75. Nominatim-Reverse löst den Punkt an einem Wirtschaftsweg in Niederwürschnitz auf, nicht an der Betreiberwebsite/Impressum genannten Stollberger Str. 34. `clear_fields: 'geo'` entzieht das nicht adressgenaue Paar beim nächsten regulären Seed. Der Betreiberstandort ist nur durch dieselbe Website-Quellfamilie belegt; kein Ersatzpunkt ohne Owner-Prüfung des Kartenpins. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.725&lon=12.75&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026

- [Recherche 25.10.2026: Betreiberhomepage und Impressum direkt gelesen: „Schrott- und Altpapierannahme“ in Niederwürschnitz, Stollberger Str. 34, 09399, Telefon 037296/928604, Inhaber laut Impressum Kevin Kühn. Homepage nennt Annahme von Altpapier, Schrott und Buntmetallen; Öffnungszeiten Mo geschlossen, Di–Do 13–18, Fr 13–17, Sa 09–12; Anlieferung/Kartenlink vorhanden. Kein zweiter unabhängiger Beleg für den Standort gefunden, daher keine Frontmatter-Fills oder Zeitenübernahme. Qualitätsmangel am Kontakt: sichtbarer Text `info@altpapier-stollberg.de`, Linkziel aber `mailto:info@testzentrum-stollberg-west.de`; E-Mail nicht übernommen. ANKAUF: keine Aussage zur Bezahlung/keine Kurse; VERKAUF: keine Preisliste; GEBÜHREN: keine Tarife. GEO: exakte Betreiberadresse jetzt recherchierbar; Owner soll Adresse/öffentlichen Zugang und Kartenpunkt prüfen. Quelle: https://schrottannahme-stollberg.de/ ; https://schrottannahme-stollberg.de/impressum.html .]

### Importiert (Seed-Stand 2026-09-30)

- Annahmestelle Altpapier/Schrott/Bunt
- Di–Sa geöffnet

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Seite schrottannahme-stollberg.de live verifiziert (Impressum: Kevin Kühn, Stollberger Str. 34, 09399 Niederwürschnitz, Tel. 037296/928604) — aber Einzelquelle ohne Zweitbeleg → nur website_status aktiv, Adress-/Kontaktfelder bleiben leer; Quelle(n): schrottannahme-stollberg.de Impressum (Live-Fetch 01.10.2026, Einzelbeleg)]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt: 50.7181428, 12.7534167** (Mittelwert der eindeutigen Knoten des
  Objekts **way/199773029**, sechs eindeutige Knoten; der Punkt liegt innerhalb der Objekt-BBox
  12.7529991/50.7180476 bis 12.7536852/50.7183008). **Quelle 1 (OSM, live geladen am 04.11.2026, HTTP 200):**
  **way/199773029** trägt `addr:street`=Stollberger Straße, `addr:housenumber`=**34a**, `addr:city`=Niederwürschnitz,
  `addr:postcode`=09399 und `building`=industrial - Straße, Hausnummer, PLZ und Ort stimmen mit der
  Betreiber-Adresse **Stollberger Str. 34, 09399 Niederwürschnitz** überein (die Dossier-Adresse nennt die
  Hausnummer **34**; in OSM trägt das Objekt **34a**, das Nachbarobjekt **way/199773034** die Nummer 40 und
  **way/200046332** die Nummer 40a - die Nummernfolge 34/34a ist im Raster nachvollziehbar). **Fensterprüfung (kein
  Fensterfehler):** das am 04.11.2026 live abgerufene Kachelraster über Niederwürschnitz umfasst
  **12.7400/50.7050 bis 12.7700/50.7300**, **9 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**; das Fenster enthält
  die gesuchte Straße **nachweislich** mit **50** Adressobjekten auf „Stollberger Straße" und den Hausnummern 2, 4, 6,
  7, 11, 12, 12a, 13, 14, 15, 16, 17, 18, 18a, 18b, 18c, 19, 20, 21, 21a, 21c, 22, 23, 25, 26, 28, 30, 31, 32, 33,
  **34a**, 35, 37, 39, 40, 40a, 41, 42, 43, 46, 47, 49, 50, 51, 56, 56a, 58, 60 und 62 - die
  **Dossier-Hausnummer 34 existiert** in OSM als **34a** und ist das einzige Objekt dieser Nummer in der
  Nummerierung; eine separate **34** ist im Raster **nicht** abgebildet. Eine Namenssuche nach **„Kühn"** über die
  Felder name, operator, description, website, contact:website, brand, note und addr:housename findet im Raster
  **keinen** Treffer (die Namensobjekte dort sind „Lidl", „Pension Eberle", „Schreinerei & Metallbau Wagner" und
  weitere Fremdbetriebe) - es gibt also **keine** OSM-Betriebszuordnung, nur die Adressangabe. Quelle 2 (Betreiber, im
  Dossier bereits zitiert, live nachgelesen am 04.11.2026): die **Betreiberwebsite**
  https://schrottannahme-stollberg.de/ liefert **HTTP 200** und nennt live **„Standortadresse Stollberger Str. 34,
  09399 Niederwürschnitz"** mit Telefon **037296 / 928604** und `info@altpapier-stollberg.de`; das **Impressum**
  nennt live **„Kevin Kühn, Stollberger Str. 34, 09399 Niederwürschnitz"** - identisch mit der Dossier-Adresse.
  **Zurücknahme des `geo`-Tokens (Begründung):** `clear_fields: 'geo'` **entfällt ersatzlos**, weil der Seed jetzt eine
  belegte Koordinate führt; der Seed lehnt ein `geo`-Clear neben belegtem `lat`/`lon` ab, und der Punkt liegt exakt
  auf der durch zwei unabhängige Quellen gedeckten Anschrift. **Qualifizierung (unverändert übernommen):** Der Punkt
  bezeichnet die **betreiberseitig genannte Anschrift**; das im Dossier dokumentierte **Fehlen einer zweiten
  unabhängigen Standortquelle** (nur dieselbe Website-Familie, kein Betreiberregister) bleibt bestehen, ebenso der
  im Dossier festgehaltene **Mail-Mismatch** (sichtbarer Text `info@altpapier-stollberg.de`, Linkziel
  `mailto:info@testzentrum-stollberg-west.de`) - die E-Mail-Adresse wird deshalb **nicht** übernommen. Eine
  Annahmezusage, ein Ankaufskurs oder eine Gebühr wird durch die Koordinate **nicht** belegt. `status: aktiv`,
  `city`, `street` (leer), `postcode` (leer), `phone` (leer), `email` (leer), `opening_hours` (leer), `website`,
  `website_status`, `description` (leer), `dropoff_json` und `pickup_json` bleiben unverändert. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/199773029/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=12.7400,50.7050,12.7700,50.7300 ;
  https://schrottannahme-stollberg.de/ ; https://schrottannahme-stollberg.de/impressum.html (Abruf 04.11.2026).]
