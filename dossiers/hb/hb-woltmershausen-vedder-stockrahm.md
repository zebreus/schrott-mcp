---
slug: hb-woltmershausen-vedder-stockrahm
lat: 53.0767514
lon: 8.7535607
name: Vedder & Stockrahm GmbH & Co. KG
trader_type: containerdienst
state: HB
city: Woltmershausen
street: Senator-Bömers-Str. 10
postcode: '28197'
phone: 0421 542554
email: info@vedder-stockrahm.de
opening_hours: Mo-Fr 07:00-16:00 Uhr
website: https://www.vedder-stockrahm.de/
website_status: ''
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Bremen (Stadt)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Vedder & Stockrahm GmbH & Co. KG

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Altmetallrecycling, Containerdienst, Rohstoffhandel; tagesaktuelle Preisliste online, Barzahlung
- Adresse: Woltmershausen, Senator-Bömers-Str. 10
- Adressbeleg: vedder-stockrahm.de

### Recherche 01.10.2026 (Feedback-Triage 2149)

- [Recherche 01.10.2026: Feedback #2149 (Öffnungszeiten/Konditionen fehlen) berechtigt — E-Mail info@vedder-stockrahm.de + Zeiten Mo-Fr 07:00-16:00 Uhr (letzte Verwiegung 15:45, Anmeldung vor Auffahrt) aus Betreiber-Seite (Kontaktbereich Homepage). Konditionen (Barzahlung, Verwiegung 1 kg-30 t, Abholung ab ca. 1,5 t, tagesaktuelle Ankaufspreise /ankauf) als Prosa-Leistungsmerkmale hier dokumentiert (kein Frontmatter-Feld). Beleglage: Einzelbeleg (Betreiber-Seite = EINE Quelle; GmbH & Co. KG, HR-Abruf offen). Adresse Senator-Bömers-Str. 10, 28197 Bremen bestätigt. Quelle(n): vedder-stockrahm.de (Betreiber, live 01.10.2026)]

### Korrektur 07.11.2026 (Preispflege)

- [Korrektur 07.11.2026 (Preispflege): **Drei veraltete Current-Preise an der Live-Seite geprüft — alle drei sind Rückzug (Fremdkörper).** Der Handler läuft (Lauf 07.11.2026 12:54 UTC, 19 Preise übernommen, 2 übersprungen — „Versilbertes Besteck 90 / 100", „Versilberte Messer", Status `ok`), er erzeugt die drei alten Schlüssel heute aber nicht mehr — die Pipeline zieht Current-Pointer für normale Handler nicht zurück, darum stehen die alten Werte weiterhin als „aktueller Preis" in der Datenbank. Betroffen sind `zinn` / „50-59%" — 5,90 EUR/kg, Stand 29.09.2026, `zinn` / „60-69%" — 9,00 EUR/kg, Stand 29.09.2026 und `zinn` / „70-79%" — 11,00 EUR/kg, Stand 29.09.2026. Die Seite https://www.vedder-stockrahm.de/ankauf/ (Abruf 07.11.2026 13:02 UTC) trägt die Überschrift „Unverbindliche Ankaufspreise07.11.2026" und die Tabelle „Materialbezeichnung | Preis | Einheit"; sie führt die Zinnsorten als Zeilen „Zinn 80% - 98% (Geschirr)" mit der Preiszelle **„14,00"** (EUR / KG), „Zinn 70% - 79%" mit **„11,00"**, „Zinn 60% - 69%" mit **„9,00"** und „Zinn 50% - 59%" mit **„5,90"** (jeweils EUR / KG). Der Betrieb bietet die Sorten also weiterhin mit unverändertem Preis an — aber der Handler führt sie seit der Zinn-/Blei-Trennung (Feedback 136/137) als `zinn-geschirr` (Zinngeschirr) und nicht mehr als `zinn` (Zinn / Reinzinn): die drei aktuellen Zeiger `zinn-geschirr` / „50-59%", `zinn-geschirr` / „60-69%", `zinn-geschirr` / „70-79%" mit 5,90 / 9,00 / 11,00 EUR/kg sind heute frisch (Stand 07.11.2026). Die alten Schlüssel (`zinn`, …) werden heute nicht mehr erzeugt; sie behaupten für „Zinn / Reinzinn" einen Zinngeschirr-Preis und sind damit Fremdkörper → **alle drei Rückzug** (die Preise bleiben in der Historie). Im Dossier ist keine OSM-/Objekt-ID hinterlegt; an der Preiszelle ändert sich nichts, `street`/`postcode`/`lat`/`lon`/`certifications`/`status` bleiben unberührt. Quelle(n): https://www.vedder-stockrahm.de/ankauf/ (Abruf 07.11.2026 13:02 UTC).]

### Geokodierung 08.11.2026

- [Recherche 08.11.2026: **Koordinate belegt: `lat`/`lon` = 53.0767514/8.7535607.** Für den Dossier-Eintrag **Vedder & Stockrahm GmbH & Co. KG** trägt das OSM-Objekt way/71459092 (`addr:housenumber`=10, `addr:street`=Senator-Bömers-Straße, `addr:postcode`=28197, `addr:city`=Bremen) die Dossier-Adresse **Senator-Bömers-Str. 10, 28197 Woltmershausen** mit der passenden Hausnummer; das Objekt führt keinen Betriebsnamen, der Punkt ist deshalb der **Adressmittelpunkt** (der Punkt ist das Mittel der eindeutigen Knoten, BBox 53.0766289/8.7534054 bis 53.0768739/8.753716). Quelle(n): https://api.openstreetmap.org/api/0.6/way/71459092/full.json ; https://nominatim.openstreetmap.org/search?q=Senator-B%C3%B6mers-Str.+10%2C+28197+Woltmershausen&format=jsonv2&extratags=1&namedetails=1&limit=10&addressdetails=1&countrycodes=de ; https://nominatim.openstreetmap.org/search?street=Senator-B%C3%B6mers-Str.+10&postalcode=28197&city=Woltmershausen&country=Germany&format=jsonv2&extratags=1&namedetails=1&limit=10&addressdetails=1 (Abruf 08.11.2026).]
