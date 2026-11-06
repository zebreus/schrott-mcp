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

Metallion GmbH ist ein Metallhändler in der Heßlerstraße 77, 45329 Essen, der NE-Metalle
und Aluminium ankauft und die Annahme von privaten und gewerblichen Kunden von Montag bis
Samstag zwischen 08:00 und 20:00 Uhr ausweist, bei kostenloser Abholung ab 500 Kilogramm
mit mobiler Waage. Das Impressum der Betreiberseite nennt die Gesellschaft mit HRB 34136
Amtsgericht Essen und den Geschäftsführer Ahmad Merhi, die Ankaufseite bestätigt die
Annahme von privat und gewerblich. Offen bleibt der Standortnachweis, weil die Hausnummer
77 auf der Heßlerstraße in OSM fehlt, die belegten Nummern springen von 65 auf 79 und kein
Objekt den Betrieb benennt, weshalb keine Koordinate gesetzt wurde.

## Timeline

### Owner-Korrektur 27.10.2026: GEO-Rücknahme

- [Owner-Korrektur 27.10.2026: Die read-only-Abfrage von `public.db` ergab für Metallion den gespeicherten Punkt 51.450284375, 7.015053125. Die Nominatim-Rückwärtssuche löst genau diesen Punkt als Freiheit, Essen 45128 (Straßenobjekt, kein Metallion-Betrieb) auf. Die aktuelle Betreiberseite identifiziert Metallion GmbH, Heßlerstr. 77, 45329 Essen; die Ankaufseite belegt ausdrücklich die Annahme auch von privat und gewerblich sowie Anlieferung, und das Impressum nennt HRB 34136 AG Essen. Damit liegt der Produktionspunkt nicht an der belegten Annahmestelle. `clear_fields: 'geo'` zieht nur das alte Paar beim nächsten regulären Seed ein; keine Ersatzkoordinate gesetzt, Historie erhalten. Quellen/Abruf 27.10.2026: https://metallion.de/impressum/ ; https://metallion.de/aluminum-ankauf ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; read-only `public.db`; `docs/trader-seed.md`.]

### Importiert (Seed-Stand 2026-09-30)

- NE-Metalle, Alu-Spezialist (Heßlerstr. 77)

### Recherche 01.10.2026

- [Feedback-Triage 01.10.2026 (ID 2450): BEWIESENE KORREKTUREN — postcode '45329', phone 0201 344344, email info@metallion.de, hours 'Mo-Sa 08:00-20:00 (Annahmeschluss 19:00)'. Impressum nennt GmbH + HRB 34136 AG Essen, GF Ahmad Merhi (Owner-Primärquelle); Header/Footer/Ankaufseiten bestätigen. Privatannahme belegt ('Ob privat oder gewerblich') → dropoff_json privat/gewerbe; Abholung ab 500 kg → pickup_json. Quelle(n): https://metallion.de/aluminum-ankauf, https://metallion.de/impressum/ (Abruf 01.10.2026).]


### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Hausnummer **77** ist auf der
  **Heßlerstraße** in OSM **nicht** erfasst; jeder andere Punkt wäre geraten. **Was live geprüft wurde (03.11.2026):**
  (a) Die Straße ist erfasst (Straßenname node/277057769 bei 51.5093023/7.0072847; der Verlauf liegt im Kachelraster
  **7.0000/51.5000 – 7.0500/51.5200**, live abgerufen über `https://api.openstreetmap.org/api/0.6/map.json?bbox=…`,
  Kachelraster **6.9400/51.3900 – 7.1600/51.5300** vollständig abgedeckt, 0 Kachel-Fehler im Zielbereich). (b) Die
  Heßlerstraße trägt live **213** verschiedene Hausnummern (3–67, 79–121, 122–361 mit Lücken sowie 275a, 275b, 277a,
  277b, 301a); die Nummerierung springt **von 65 auf 79** - **die 77 fehlt**. (c) Eine Suche nach
  `addr:housenumber`=77 mit `addr:postcode`=45329 im gesamten Essener Raster liefert **13** Objekte, **alle auf anderen
  Straßen** (Arenbergstraße, Wolbeckstraße, Hohendahlstraße, Rahmdörne, Batenbrocker Straße, Boshamerweg, Karnaper
  Straße, Böhmerheide, Basunestraße, Bischoffstraße, I. Schichtstraße, Heisterholz, Stinnesstraße) und **kein** Objekt
  auf der Heßlerstraße. (d) Eine Suche über `name`, `operator`, `brand`, `website`, `contact:website`, `description`
  und `note` nach **„Metallion“** im gesamten Raster liefert **0** Treffer. Quelle 1 (Betreiber/Register): das Impressum
  der Betreiber-Website nennt **Metallion GmbH, Heßlerstr. 77, 45329 Essen**, **HRB 34136 AG Essen**, GF Ahmad Merhi; die
  Ankaufseite bestätigt die Annahme von privat und gewerblich sowie die Anlieferung Mo–Sa 08:00–20:00. Die Anschrift ist
  damit betreiberseitig belegt. Quelle 2 (OSM): das Fehlen der Dossier-Hausnummer - **kein** Objekt mit
  `addr:housenumber`=77 und `addr:street`=Heßlerstraße existiert, und **kein** Objekt nennt den Betrieb. **Was fehlt:**
  ein adressgenauer OSM-Punkt für Heßlerstraße 77; die Nachbarnummern 65 und 79 liegen an anderen Stellen der Straße
  und wären als Ersatz geraten. `clear_fields: 'geo'` bleibt unverändert bestehen, der zuvor gespeicherte
  Produktionspunkt **51.450284375/7.015053125** (Straße „Freiheit“, 45128 Essen) bleibt zurückgenommen.
  Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=6.9400,51.3900,7.1600,51.5300 ;
  https://api.openstreetmap.org/api/0.6/node/277057769.json ; https://metallion.de/impressum/ ;
  https://metallion.de/aluminum-ankauf (Abruf 03.11.2026).]

### Korrektur 04.11.2026

- [Korrektur 04.11.2026: **Mengenangabe des Vermerks vom 03.11.2026 berichtigt: 16 statt 13 Objekte mit
  `addr:housenumber`=77 und `addr:postcode`=45329 im Essener Kachelraster.** Die drei bislang nicht
  mitgezählten Objekte sind **Hegerkamp** (way/276260967, `addr:street`=Hegerkamp, `addr:city`=Essen,
  `building`=house), **Holzschragen** (way/163016191, `addr:street`=Holzschragen, `building`=yes) und
  **Lattenkamp** (way/276276480, `addr:street`=Lattenkamp, `building`=house); Hegerkamp und Lattenkamp
  wurden zuletzt am **30.05.2025** (Changeset **166981379**), Holzschragen zuletzt am **15.05.2014**
  (Changeset **22346723**) bearbeitet. Alle drei waren also **schon vor** der Welle vom 03.11.2026 in
  OSM erfasst: die Zahl 13 war eine **unvollständige Zählung**, keine Änderung des Datenbestands.
  **Live nachgezählt (04.11.2026):** Das im Vermerk vom 03.11.2026 dokumentierte Kachelraster
  **6.9400-7.1600 / 51.3900-51.5300** (0,01-Grad-Kacheln, api.openstreetmap.org/api/0.6/map.json,
  **308 von 308 Kacheln**, **0 Kachel-Fehler**) liefert **16** Objekte mit `addr:housenumber`=77 und
  `addr:postcode`=45329 auf **16 verschiedenen Straßen**: Arenbergstraße, Batenbrocker Straße,
  Basunestraße, Bischoffstraße, Böhmerheide, Boshamerweg, Hegerkamp, Heisterholz, Hohendahlstraße,
  Holzschragen, I. Schichtstraße, Karnaper Straße, Lattenkamp, Rahmdörne, Stinnesstraße, Wolbeckstraße;
  ohne PLZ-Filter trägt das Raster insgesamt **383** Objekte mit `addr:housenumber`=77.
  **Die tragende Aussage des Vermerks vom 03.11.2026 bleibt damit bestätigt:** Die Dossier-Hausnummer
  **77** fehlt auf der **Heßlerstraße** nach wie vor, **0 von 16** der so belegten Objekte liegt dort;
  im Korridorfenster der Heßlerstraße (**7.0000-7.0500 / 51.5000-51.5200**, 10 Kacheln zu 0,01 Grad,
  vollständig abgerufen, 0 Kachel-Fehler) liegt **kein** Objekt mit `addr:housenumber`=77 und
  `addr:street`=Heßlerstraße, und die dort belegten Nummern springen von **65** auf **79**.
  **Unverändert:** `clear_fields: 'geo'` bleibt bestehen, es wird **kein** Punkt geraten; der
  zurückgenommene Produktionspunkt **51.450284375/7.015053125** (Straße „Freiheit", 45128 Essen)
  bleibt zurückgenommen; `status: aktiv`, `street`/`postcode`, Annahmezeiten, Ankaufspreise,
  Verkaufspreise und Gebühren bleiben unverändert; der Vermerk vom 03.11.2026 bleibt **wortgleich**
  erhalten und wird durch diesen Vermerk **berichtigt**. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=6.9400,51.3900,7.1600,51.5300 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0000,51.5000,7.0500,51.5200 ;
  https://api.openstreetmap.org/api/0.6/way/276260967/full.json ;
  https://api.openstreetmap.org/api/0.6/way/163016191/full.json ;
  https://api.openstreetmap.org/api/0.6/way/276276480/full.json (Abruf 04.11.2026).]
