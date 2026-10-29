---
slug: ni-luchow-wilfried-bleck
name: Wilfried Bleck
trader_type: autoverwertung
state: NI
city: Lüchow
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Autoverwertung mit Autorecycling und Gebrauchtteilehandel in Lüchow (Wendland). Verzeichnisquellen nennen den Betrieb unter wechselnden Vornamen („Wilfried Bleck" und „Autoverwertung Bleck, Marcel Bleck") mit der Anschrift Albrecht-Thaer-Str. 16, 29439 Lüchow, Telefon 05841 709300, Fax 05841709302 und E-Mail autoverwertung-bleck@web.de. Dieselbe Hausnummer trägt im amtlichen Register den anerkannten Demontagebetrieb „Autoverwertung Krause" (Zertifikat ZAFS00100011, Registerstand 11.08.2026) und in OSM eine als „Autoverwertung Boick" erfasste Schrottplatz-Fläche (industrial=scrap_yard) — es handelt sich also um eine gemeinsam genutzte bzw. mehrfach belegte Hausnummer, nicht um einen exklusiv dem Seed-Eintrag zugeordneten Standort. ANKAUF: keine Fahrzeugvergütung, kein Schrottkurs und keine Sortenliste belegt. VERKAUF: Gebrauchtteilehandel ist in den Verzeichnisangaben genannt, eine Verkaufspreisliste existiert nicht. GEBÜHREN: keine Verwertungs-, Abhol- oder Entsorgungstarife belegt; die Öffnungszeiten Mo–Fr 08:15–17:00 sind eine Verzeichnisangabe.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# Wilfried Bleck

## Überblick

Der Seed-Eintrag nennt einen Vornamen und eine Branche, aber keine Adresse. Die Recherche führt zu einer **durch drei unabhängige Quellen deckungsgleich belegten Hausnummer — die jedoch nachweislich nicht exklusiv ist**:

1. **Verzeichnisquellen** (unabhängig voneinander): `ortsdienst.de` führt „Wilfried Bleck in Lüchow (Schrottplatz)", Albrecht-Thaer-Str. 16, 29439 Lüchow, Telefon 05841709300, Fax 05841709302, E-Mail autoverwertung-bleck@web.de, Öffnungszeiten Montag–Freitag 8:15–17:00, Bewertung 2,6 Sterne bei 17 Bewertungen, Tätigkeitsangabe „Autoverwertung, Autorecycling und Gebrauchtteile Handel". `autoverwerter.de` (Verzeichnis der Autoverwerter im Landkreis Lüchow-Dannenberg) führt „Autoverwertung-Bleck [Autoteile Handel, Autoverwertung]", Albrecht-Thaer-Str. 16, 29439 Lüchow, Telefon 05841 709300. `dasoertliche.de` führt „Bleck Wilfried Autoverwertung" **und** „Autoverwertung Bleck, Marcel Bleck" an derselben Anschrift mit derselben Telefonnummer.
2. **Amtliches Register** (Fachbetrieberegister, Registerstand 11.08.2026): Die **Hausnummer 16** ist dort dem anerkannten **Demontagebetrieb „Autoverwertung Krause"** zugeordnet (Albrecht-Thaer-Str. 16, 29432 Lüchow, Zertifikat ZAFS00100011) — die Nachbardossiers `ni-luchow-autoverwertung-krause` und `ni-luchow-piskorski-entsorgungsfachbetrieb` dokumentieren denselben Ort (Piskorski: Albrecht-Thaer-Straße 13). Der Registereintrag trägt **keinen Eintrag „Bleck"**.
3. **OSM**: Dieselbe Fläche ist als `industrial=scrap_yard`, `landuse=industrial`, **`name=Autoverwertung Boick`** erfasst (way 963347299, 8 Stützpunkte, Fläche mit rund 13,5 m Abstand zum Gebäude-Way 213905139 „16, Albrecht-Thaer-Straße").

Damit sind an **einer** Hausnummer drei Gewerbetreibende bzw. Schrottplatz-Bezeichnungen belegt (Krause, Boick, Bleck) und zusätzlich zwei Vornamen (Wilfried/Marcel) für den Verzeichniseintrag. Der Seed-Eintrag „Wilfried Bleck" ist damit **nicht exklusiv zuordenbar**; die Frontmatter-Felder bleiben deshalb leer und `status: pruefung` bleibt bestehen.

**GEO:** Der bisherige Produktionspunkt (53.3257/10.84365) löst nach OSM/Nominatim auf „11, Butterberg, Sprengelshof, Teldau, Boizenburg-Land, **Mecklenburg-Vorpommern (19273)**" auf und liegt rund **43,71 km** vom belegten Lüchower Ort entfernt — ein **falscher Punkt im falschen Bundesland**. Ein Ersatz wird **nicht** gesetzt: Das OSM-Adressgebäude „16, Albrecht-Thaer-Straße" (way 213905139, `addr:postcode=29439`, 52.9838646/11.1672339) trägt den Betrieb **nicht** — es ist ein reines Adressgebäude ohne Namens- oder Betreiber-Tag, und dieselbe Adresse ist amtlich Krause zugeordnet. Der Nachbar-Way 963347299 („Autoverwertung Boick") ist die einzige Scrap-Yard-Fläche, gehört aber zu einem **anderen** Namen.

**Preise (Ankauf / Verkauf / Gebühren getrennt):**

- **Ankauf:** **Keine Fahrzeugvergütung, kein Schrottkurs, keine Sortenliste belegt.** Das amtliche Register führt ausdrücklich keine Preise; die Verzeichnisangaben nennen nur die Tätigkeitsfelder „Autoverwertung, Autorecycling und Gebrauchtteile Handel".
- **Verkauf:** Gebrauchtteilehandel ist als Tätigkeit genannt; **eine Verkaufspreisliste existiert nicht**.
- **Gebühren:** **Keine Verwertungs-, Abhol- oder Entsorgungstarife belegt.** Die Öffnungszeiten Montag–Freitag 8:15–17:00 sind eine Verzeichnisangabe, kein Gebührenbeleg.

**Offen:** Ob „Wilfried" und „Marcel" dieselbe Person bezeichnen oder zwei Betriebe; ob der Eintrag „Bleck" eine veraltete Namensvariante von „Boick" oder „Krause" ist; und welche Gesellschaft die Hausnummer tatsächlich betreibt.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Autoverwertung, Albrecht-Thaer-Str. 16 (nur Verzeichnisbeleg) (Flag)
- (Autoverwertung Bleck)
### Recherche 29.10.2026: Hausnummer belegt, aber drei Gewerbetreibende an derselben Adresse — kein exklusiver Standort

- [Recherche 29.10.2026: Die Recherche führt zu einer **durch drei unabhängige Quellen deckungsgleich belegten Hausnummer**, die jedoch **nicht exklusiv** dem Seed-Eintrag gehört. (1) **Verzeichnisquellen**, unabhängig voneinander: `ortsdienst.de` — „Wilfried Bleck in Lüchow (Schrottplatz)", Albrecht-Thaer-Str. 16, 29439 Lüchow, Telefon 05841709300, Fax 05841709302, E-Mail autoverwertung-bleck@web.de, Öffnungszeiten Montag–Freitag 8:15–17:00, 2,6 Sterne bei 17 Bewertungen, Tätigkeitsangabe „Autoverwertung, Autorecycling und Gebrauchtteile Handel"; `autoverwerter.de` (Verzeichnis der Autoverwerter im Landkreis Lüchow-Dannenberg) — „Autoverwertung-Bleck [Autoteile Handel, Autoverwertung]", Albrecht-Thaer-Str. 16, 29439 Lüchow, Telefon 05841 709300; `dasoertliche.de` — „Bleck Wilfried Autoverwertung" **und** „Autoverwertung Bleck, Marcel Bleck", beide an derselben Anschrift mit derselben Nummer. (2) **Amtliches Register** (Fachbetrieberegister, Registerstand 11.08.2026): Die **Hausnummer 16 ist dort dem anerkannten Demontagebetrieb „Autoverwertung Krause" zugeordnet** (Zertifikat ZAFS00100011, Anschrift im Register „Albrecht-Thaer-Str. 16, 29432 Lüchow"); ein Eintrag „Bleck" existiert dort **nicht**. Die Ortsseite Lüchow führt nur drei Betriebe (Autoverwertung Krause, Piskorski GmbH & Co. KG, Albrecht-Thaer-Straße 13, sowie einen Hauptsitz Bergstraße 10 mit Handeln/Makeln) — **„Bleck" fehlt im Register**. (3) **OSM**: dieselbe Fläche ist als `industrial=scrap_yard`, `landuse=industrial`, **`name=Autoverwertung Boick`** erfasst (way 963347299, 8 Stützpunkte, rund **13,5 m** vom Gebäude-Way 213905139 „16, Albrecht-Thaer-Straße" entfernt). An **einer** Hausnummer stehen damit drei Bezeichnungen (Krause amtlich, Boick in OSM, Bleck in Verzeichnissen) und zwei Vornamen (Wilfried/Marcel) — der Seed-Eintrag ist **nicht exklusiv zuordenbar**. Frontmatter daher unverändert: Straße, PLZ, Telefon, E-Mail und Öffnungszeiten bleiben **leer**, `status: pruefung` bleibt; `website_status` von leer auf `unbekannt` normalisiert und `description` gesetzt. **GEO:** der Produktionspunkt 53.3257/10.84365 löst nach OSM/Nominatim auf „11, Butterberg, Sprengelshof, Teldau, Boizenburg-Land, **Mecklenburg-Vorpommern (19273)**" auf und liegt **43,71 km** vom belegten Lüchower Ort entfernt (Haversine R=6371 km) — ein **falscher Punkt im falschen Bundesland**. Ein Ersatz wird **nicht** gesetzt: Das OSM-Adressgebäude way 213905139 („16, Albrecht-Thaer-Straße", `addr:postcode=29439`, 52.9838646/11.1672339) trägt **keinen** Namen und keinen Betreiber-Tag, und dieselbe Adresse ist amtlich Krause zugeordnet; der Nachbar-Way 963347299 („Autoverwertung Boick") ist die einzige Scrap-Yard-Fläche, gehört aber zu einem **anderen** Namen. **ANKAUF:** **keine Fahrzeugvergütung, kein Schrottkurs, keine Sortenliste belegt**; das amtliche Register führt ausdrücklich keine Preise, die Verzeichnisangaben nennen nur Tätigkeitsfelder. **VERKAUF:** Gebrauchtteilehandel als Tätigkeit genannt, **keine Verkaufspreisliste**. **GEBÜHREN:** **keine Verwertungs-, Abhol- oder Entsorgungstarife belegt**; die Öffnungszeiten Mo–Fr 8:15–17:00 sind eine Verzeichnisangabe, kein Gebührenbeleg. **Offen:** ob „Wilfried" und „Marcel" dieselbe Person bezeichnen; ob „Bleck" eine veraltete Namensvariante von „Boick" oder „Krause" ist; welcher Betrieb die Hausnummer tatsächlich betreibt. Quellen: https://www.ortsdienst.de/niedersachsen/luechow-dannenberg/luechow/schrottplatz/wilfried-bleck-inst556755/ ; https://autoverwerter.de/autoverwertungen/Niedersachsen/L%C3%BCchow-Dannenberg-3354.html ; https://www.dasoertliche.de/Themen/Kfz-Dienstleistungen/L%C3%BCchow-Wendland.html ; https://schrottregister.pages.dev/betrieb-autoverwertung-krause-luchow ; https://schrottregister.pages.dev/ort-luchow ; https://api.openstreetmap.org/api/0.6/way/963347299/full ; https://api.openstreetmap.org/api/0.6/way/213905139/full ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.3257&lon=10.84365&zoom=18&addressdetails=1 ; read-only `public.db` (29.10.2026).]
