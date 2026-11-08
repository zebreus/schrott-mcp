---
slug: ni-marxen-metallankauf24-andre-owsianski-ne-spezia
lat: 53.3162375
lon: 10.0052057
name: Metallankauf24 Andre Owsianski (NE-Spezialist mit Preisliste)
trader_type: metallhaendler
state: NI
city: Marxen
street: 'Hinter der Bahn 23'
postcode: '21439'
phone: '04185 8094617'
email: 'info@metallankauf24.de'
opening_hours: 'Schrottplatz Do–Fr 09:00–14:00 o. n. V.; Büro Mo–Fr (Mo/Di/Do/Fr 08:00–14:00, Mi 08:00–18:00)'
website: https://metallankauf24.de
website_status: 'aktiv'
status: aktiv
description: ''
dropoff_json: ''
pickup_json: '{"allowed": true, "conditions": "Abholung deutschlandweit + Grossraum Hamburg, Paketversand, Anlieferung Marxen (Betreiber-Website, Feedback-Triage 01.10.2026)"}'
provenance_section: Celle / Lüneburg / Heide / Uelzen / Harburg
provenance_ankauf_raw: ja
provenance_origin: table
---

# Metallankauf24 Andre Owsianski (NE-Spezialist mit Preisliste)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Impressum verifiziert — Andre Owsianski e.K., Hinter der Bahn 23, 21439 Marxen, HRA 201768, Tel. 04185-8094617, Fax 04185-8094618, info@metallankauf24.de; Footer/Home konsistent (gleiche Adresse/Tel/Mail); Zeiten-Seite: Schrottplatz Do–Fr 09:00–14:00 o. n. V., Büro Mo/Di/Do/Fr 08:00–14:00, Mi 08:00–18:00; Leistungen Anlieferung Marxen + Abholung deutschlandweit/Großraum Hamburg + Paketversand, Tagespreise, VDM-Mitglied. HRA als register-kongruente Betreiber-Primärquelle (HRA 201768 aus Impressum). Quelle(n): https://metallankauf24.de/ + https://metallankauf24.de/impressum + https://metallankauf24.de/kontakt/oeffnungszeiten (eine Quelle, HRA-kongruent)]
- [Feedback-Triage ID 2379: Meldung berechtigt — Adresse/Telefon/E-Mail/Zeiten aus Betreiber-Impressum + Zeiten-Seite (HRA-kongruent, starke Einzelquelle) übernommen; pickup_json:true (Abholung deutschlandweit/HH + Versand + Anlieferung belegt); Typ metallhaendler bleibt (NE-Spezialist mit Preisliste zutreffend); website_status → aktiv. Beleglage: starke Betreiber-Einzelquelle (HRA-kongruent, offengelegt). Quelle(n): siehe Vorbullet.]

### Importiert (Seed-Stand 2026-09-30)

- NE-Metalle, Kabel, Zinn/Nickel/Rotguss

### Korrektur 07.11.2026 (Preispflege)

- [Korrektur 07.11.2026 (Preispflege): **Vier veraltete Current-Preise an der Live-Seite geprüft — alle vier sind Rückzug.** Der Handler läuft (Lauf 07.11.2026 07:06 UTC, 3 Preise übernommen, 5 übersprungen — „Edelstahl / Nickel“, „Messing / Rotguss“, „VHM / HSS / WOLFRAM“, „Zink / Blei“, „Kabel / E-Motoren“, Status `ok`) und erzeugt die vier alten Schlüssel heute nicht mehr — die Pipeline zieht Current-Pointer für normale Handler nicht zurück, darum stehen die alten Werte weiterhin als „aktueller Preis“ in der Datenbank. Betroffen sind `aluminium-profile` / „“ — 2,90 EUR/kg, `kupfer-millberry` / „“ — 10,80 EUR/kg, `kabel-kupfer` / „“ — 4,50 EUR/kg und `messing` / „“ — 8,80 EUR/kg (alle Stand 27.09.2026).
  Die Seite https://www.metallankauf24.de/ (Abruf 07.11.2026 13:02 UTC; die Handler-URL https://metallankauf24.de/ leitet per 301 auf die www-Form um) führt unter der Überschrift „Aktuelle Schrottpreise“ je Kategorie eine Preiszelle: „Kupfer“ mit **„bis zu € 10,60 erhalten“**, „Aluminium“ mit **„bis zu € 2,90 erhalten“**, „Edelstahl / Nickel“ mit **„bis zu € 12,00 erhalten“**, „Messing / Rotguss“ mit **„bis zu € 8,90 erhalten“**, „VHM / HSS / WOLFRAM“ mit **„bis zu € 45,00 erhalten“**, „Zink / Blei“ mit **„bis zu € 2,00 erhalten“**, „Kabel / E-Motoren“ mit **„bis zu € 4,60 erhalten“**, „Zinn“ mit **„bis zu € 26,00 erhalten“** und „Schrott“ mit **„Preis auf Anfrage“**.
  1) `aluminium-profile` / „“ — die Kategorie „Aluminium“ bietet der Betrieb mit unverändertem Preis an, der Handler führt sie heute aber als `aluminium-gemischt` / „“ (2,90 EUR/kg, Stand 07.11.2026 07:06 UTC) → **Rückzug** (Fremdkörper). 2) `kupfer-millberry` / „“ — die Kategorie „Kupfer“ nennt der Betrieb heute mit **„bis zu € 10,60 erhalten“** (Datenbankwert 10,80 EUR/kg), der Handler führt sie als `kupfer-gemischt` / „“ (10,60 EUR/kg, Stand 07.11.2026 07:06 UTC) → **Rückzug** (Fremdkörper). 3) `kabel-kupfer` / „“ — die Kategorie „Kabel / E-Motoren“ nennt der Betrieb mit **„bis zu € 4,60 erhalten“** (Datenbankwert 4,50 EUR/kg), der Handler ordnet sie aber keinem Katalogmaterial zu (Kabel und E-Motoren gemischt) und legt dafür keinen Preis an; das Label steht im Lauf 07.11.2026 07:06 UTC unter den fünf übersprungenen → **Rückzug** (Fremdkörper). 4) `messing` / „“ — die Kategorie „Messing / Rotguss“ nennt der Betrieb mit **„bis zu € 8,90 erhalten“** (Datenbankwert 8,80 EUR/kg), der Handler ordnet sie keinem Katalogmaterial zu (Messing und Rotguss gemischt, die Zelle nennt den Höchstwert der Kategorie) → **Rückzug** (Fremdkörper).
  Alle vier Preise bleiben in der Historie; zurückgezogen werden soll nur der jeweils alte Current-Pointer. Im Dossier ist keine OSM-/Objekt-ID hinterlegt; die Dossier-Adresse Hinter der Bahn 23, 21439 Marxen ist in OSM als way/884011623 geführt (`addr:housenumber`=23, `addr:street`=Hinter der Bahn, `addr:postcode`=21439, `addr:city`=Marxen; live abgerufen 07.11.2026), das benachbarte Objekt way/884011622 trägt `addr:housenumber`=21 und `name`=metallankauf 24. An den Dossierdaten ändert sich nichts, `street`/`postcode`/`lat`/`lon`/`certifications`/`status` bleiben unverändert. Quelle(n): https://www.metallankauf24.de/ (Abruf 07.11.2026 13:02 UTC).]

### Recherche 08.11.2026

- [Recherche 08.11.2026: **Koordinate belegt: `lat`/`lon` = 53.3162375/10.0052057.** Für den Dossier-Eintrag **Metallankauf24 Andre Owsianski (NE-Spezialist mit Preisliste)** trägt das OSM-Objekt way/884011623 (`addr:housenumber`=23, `addr:street`=Hinter der Bahn, `addr:postcode`=21439, `addr:city`=Marxen) die Dossier-Adresse **Hinter der Bahn 23, 21439 Marxen** mit der passenden Hausnummer; das Objekt führt keinen Betriebsnamen, der Punkt ist deshalb der **Adressmittelpunkt** (der Punkt ist das Mittel der eindeutigen Knoten, BBox 53.3161692/10.0050904 bis 53.3163059/10.0053209). Quelle(n): https://api.openstreetmap.org/api/0.6/way/884011623/full.json ; https://nominatim.openstreetmap.org/search?q=Hinter+der+Bahn+23%2C+21439+Marxen&format=jsonv2&extratags=1&namedetails=1&limit=10&addressdetails=1&countrycodes=de ; https://nominatim.openstreetmap.org/search?street=Hinter+der+Bahn+23&postalcode=21439&city=Marxen&country=Germany&format=jsonv2&extratags=1&namedetails=1&limit=10&addressdetails=1 (Abruf 08.11.2026).]
