---
slug: nw-wuppertal-unterbarmen-halan
lat: 51.2669998
lon: 7.1628562
name: Halan
trader_type: schrotthaendler
state: NW
city: Wuppertal-Unterbarmen
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
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Halan

## Überblick

Das Portal nennt jetzt die konkrete Lead-Adresse **Burgunderstraße 19, 42285 Wuppertal**, Telefon 0202 76713022 und bezeichnet sie als „Adresse des Schrottplatzes“; ein bestätigter Betreiber-/Registerbeleg fehlt. Portalzeiten widersprüchlich: Status „öffnet Montag um 12:00“, Wochenliste Mo–Sa 08:00–20:00. Die dortige Materialliste und Mindestmengen (Eisenmetalle ab 100 kg, Nichteisenmetalle ab 1 kg) sind nicht betreiberspezifisch validiert. Seed-Unterbarmen bleibt unbestätigt; Status `pruefung`, keine Kontakt-/Adressfills.

**Ankauf:** Verzeichnisleistung ohne verifizierte Vergütung/Preise. **Verkauf:** keine Liste. **Gebühren:** keine Tabelle. Burgunderstraße erst nach Betreiber-/Standortbestätigung geocodieren.

## Timeline

### Recherche 26.10.2026 — Portalstatus erneut geprüft

- [Recherche 26.10.2026: Halan-Profil auf LokaleSchrottplatz direkt erneut gelesen. Die Seite nennt weiterhin Burgunderstraße 19, 42285 Wuppertal und 0202 76713022, behauptet aber keine Betreiberidentität. Der Status „geschlossen/öffnet morgen 12“ widerspricht der Wochenliste Mo–Sa 08–20; Materialliste und 100-kg-/1-kg-Mindestmengen sind Portalvorlage, keine bestätigten Konditionen. Daher weder Adresse noch Service/Preise als Betreiberfakten übernommen; Ankauf, Verkauf und Gebühren nicht verifiziert. `clear_fields: 'geo'` bleibt bestehen; keine Ersatzkoordinate aus dem Portal-Lead. Quelle: https://lokaleschrottplatz.de/halan-schrottabholung-wuppertal/ .]

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 51.259513, 7.1649087. Nominatim-Reverse löst den Punkt an den Gewerblichen Schulen I am Hardtufer auf, nicht an der als Portal-Lead geführten Burgunderstraße 19. Der Portalstatus und die Annahmefunktion sind widersprüchlich; `clear_fields: 'geo'` entzieht den nicht adressgenauen Altpunkt beim nächsten regulären Seed. Keine Ersatzkoordinate. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.259513043478265&lon=7.164908695652174&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026: Portalprofil und Mindestmengen gegengeprüft

- [Recherche 25.10.2026: Halan-Profil auf LokaleSchrottplatz direkt gelesen. Das Portal bezeichnet Burgunderstraße 19, 42285 Wuppertal nun ausdrücklich als „Adresse des Schrottplatzes“, führt 0202 76713022, Wochenliste Mo–Sa 08:00–20:00 und Mindestmengen Eisenmetalle ab 100 kg/NE-Metalle ab 1 kg. Zugleich steht im Status „Geschlossen, öffnet Montag um 12:00“, was mit der Wochenliste nicht zusammenpasst. Sämtliche Angaben bleiben ein einzelner Branchenportal-Lead; die standardisierte Material-/Mindestmengenliste ist keine verifizierte Halan-Annahmebedingung. **Ankauf:** keine Vergütung/Preise bestätigt; **Verkauf:** keine Liste; **Gebühren:** keine Tarife. Keine GEO-Koordinate oder Frontmatter-Änderung; Betreiber und Annahmefunktion vor Pin bestätigen. Quelle: https://lokaleschrottplatz.de/halan-schrottabholung-wuppertal/ .]

### Recherche 13.10.2026

- [Recherche 13.10.2026: LokaleSchrottplatz-Profil erneut live geprüft: Burgunderstraße 19, 42285 Wuppertal, 0202 76713022 und Standardmaterial-/Mindestmengenangaben bleiben ausschließlich ein Portal-Lead. Seine Anzeige „öffnet morgen 12:00“ steht weiterhin im Konflikt mit der Wochenliste Mo–Sa 08–20 Uhr; die Mindestmengen-Eisen/NE-Formulierungen gleichen den MAR-/Mario-Deutsch-Profilen und sind nicht als betreiberspezifische Kondition belastbar. Kein Primärbeleg für Halan/Annahmeadresse. Ankauf: keine bestätigte Vergütung oder Preise; Verkauf: keine Liste; Gebühren: keine Tarife. Keine GEO-/Frontmatter-Änderung. Quelle: https://lokaleschrottplatz.de/halan-schrottabholung-wuppertal/ ; Abruf 13.10.2026.]

### Vertiefung 08.10.2026

- [Recherche 08.10.2026: Bisherige Aussage Lead ohne Adresse aktualisiert: LokaleSchrottplatz direkt gelesen,Burgunderstraße19,42285Wuppertal,020276713022. WochenlisteMo-Sa08-20 widerspricht Status öffnet morgen12:00; MindestmengenEisen100kg/NE1kg und breite Materialliste identisch bei TuS/HartmutStefan. Keine Betreiberbestätigung/Materialbedingungen daraus übernehmen, keine Frontmatter-Fills. Suche nach Name/Telefon liefert keinen zuordenbaren Primärbeleg. Keine numerischen Ankauf-/Verkaufs-/Gebührenwerte, Verzeichnislead als konkreter Gewerberegister-Suchschlüssel erhalten. Quelle(n): https://lokaleschrottplatz.de/halan-schrottabholung-wuppertal/ ; https://lokaleschrottplatz.de/tus-schrott-metalle/ ; https://lokaleschrottplatz.de/hartmut-stefan/]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Wuppertal-Unterbarmen

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall — nur 1 Verzeichnis-Lead ohne Adresse: Halan Schrottabholung Wuppertal, Tel. +49 202 76713022 (lokaleschrottplatz.de-Lead); 11880-Eintrag existiert, aber Abruf blockiert (403); kein Betreiber, kein Register; kein Frontmatter-Fill; Quelle(n): nur Lead — lokaleschrottplatz.de/halan-schrottabholung-wuppertal/]
- [Recherche 01.10.2026: Klärfall bestätigt — Nachrecherche ohne Zweitbeleg (kein Betreiber, kein Register, keine Adresse), Telefon-Lead unbestätigt — kein Fill, Status pruefung; Quelle(n): keine belegfähige Quelle 01.10.2026]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt: 51.2669998, 7.1628562** (Mittelwert der eindeutigen Knoten des
  Objekts **way/294012794**, zehn eindeutige Knoten; der Punkt liegt innerhalb der Objekt-BBox
  7.1627227/51.2669539 bis 7.1629955/51.2670813). **Quelle 1 (OSM, live geladen am 04.11.2026, HTTP 200):**
  **way/294012794** trägt `addr:street`=Burgunderstraße, `addr:housenumber`=**19**, `addr:city`=Wuppertal,
  `addr:postcode`=42285, `addr:country`=DE und `building`=yes - Straße, Hausnummer, PLZ und Ort stimmen mit der im
  Dossier belegten Seed-Adresse **Burgunderstraße 19, 42285 Wuppertal (Unterbarmen)** überein. **Fensterprüfung (kein
  Fensterfehler):** das am 04.11.2026 live abgerufene Kachelraster umfasst **7.1600/51.2600 bis 7.1700/51.2700**,
  **1 Kachel von je 0,01 Grad**, **0 Kachel-Fehler**; das Fenster enthält die gesuchte Straße **nachweislich** mit
  den Straßenobjekten **way/38659746** und **way/38659747** (`name`=Burgunderstraße) sowie **24** Adressobjekten auf
  „Burgunderstraße" mit den Hausnummern 1, 3, 5, 12, **19**, 21, 27, 29, 31, 33, 35, 37, 39, 39a, 39b, 41, 41a, 41b,
  41c, 47, 49 und 50 (50 doppelt) - die **Dossier-Hausnummer 19 existiert** und ist im Raster das einzige Objekt mit
  dieser Nummer auf dieser Straße. Eine Namenssuche nach **„Halan"** über die Felder name, operator, description,
  website, contact:website, brand, note und addr:housename findet im Raster **keinen** Treffer - es gibt also
  **keine** OSM-Betriebszuordnung, nur die Adressangabe. Quelle 2 (Betreiber/Verzeichnis, im Dossier bereits zitiert,
  live nachgelesen am 04.11.2026): das **Schrottplatz-Portal LokaleSchrottplatz** liefert **HTTP 200** und führt live
  **„Halan Schrottabholung Wuppertal"** mit der Angabe **„Adresse des Schrottplatzes: Burgunderstraße 19, 42285
  Wuppertal"**, Telefon **+49 202 76713022**. **Zurücknahme des `geo`-Tokens (Begründung):**
  `clear_fields: 'geo'` **entfällt ersatzlos**, weil der Seed jetzt eine belegte Koordinate führt; der Seed lehnt ein
  `geo`-Clear neben belegtem `lat`/`lon` ab, und der Punkt liegt exakt auf der durch zwei unabhängige Quellen
  gedeckten Anschrift. **Qualifizierung (unverändert übernommen):** Der Punkt bezeichnet die
  **portalgestützte Anschrift**, keinen verifizierten Annahmehof; das im Dossier dokumentierte **Fehlen einer
  Betreiberidentität** (das Portal behauptet keine Betreiberfirma, kein Registerbeleg), der **widersprüchliche
  Portalstatus** („Geschlossen, öffnet morgen um 13:00" gegen die Wochenliste Mo–Sa 08–20) und die
  **Portal-Mindestmengen** (100 kg Eisen / 1 kg NE, identisch mit anderen Portalprofilen) bleiben unbestätigte
  Portalangaben - eine Ankaufzusage, ein Annahmehof oder ein Öffnungsnachweis wird durch die Koordinate **nicht**
  belegt. `status: pruefung`, `city`, `street` (leer), `postcode` (leer), `phone` (leer), `email`, `opening_hours`,
  `website` (leer), `website_status` (leer) und `description` (leer) bleiben unverändert. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/294012794/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.1600,51.2600,7.1700,51.2700 ;
  https://lokaleschrottplatz.de/halan-schrottabholung-wuppertal/ (Abruf 04.11.2026).]
