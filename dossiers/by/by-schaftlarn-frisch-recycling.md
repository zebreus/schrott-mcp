---
slug: by-schaftlarn-frisch-recycling
lat: 47.977364
lon: 11.4674728
name: Frisch Recycling GmbH
trader_type: schrotthaendler
state: BY
city: Schäftlarn
street: Kloster Schäftlarn 3 a
postcode: '82067'
phone: 08178/9550-03
email: frisch-recycling@t-online.de
opening_hours: Büro Mo-Do 8-12 und 13-17, Fr 8-12 und 13-16.30; Anlieferung Mo-Do 8-12 und 13-16.30, Fr 8-12 und 13-16
website: http://frisch-recycling.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Betriebe mit geprüfter Website
provenance_ankauf_raw: ja
provenance_origin: table
---

# Frisch Recycling GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback-ID 1963 berechtigt – Adresse/PLZ/Telefon/E-Mail/Öffnungszeiten eingearbeitet; Quelle(n): Betreiber-Website frisch-recycling.de Footer + /impressum (Owner-Ausnahme: Impressum Name+HRB+Ort HR-kongruent)]
- Frisch Recycling GmbH, GF Josef Frisch, Kloster Schäftlarn 3 a, 82067 Schäftlarn; Tel. 08178/9550-03, E-Mail frisch-recycling@t-online.de; HRB 115545 AG München, USt-IdNr. DE 185743708.
- Öffnungszeiten getrennt übernommen: Büro Mo-Do 8–12 + 13–17, Fr 8–12 + 13–16.30; Anlieferung Mo-Do 8–12 + 13–16.30, Fr 8–12 + 13–16.
- Website auf Domain-Root normiert.

### Importiert (Seed-Stand 2026-09-30)

- E-Schrott (Leiterplatten, CPUs), Metalle, Kabel

- [30.10.2026 (Korrekturvermerk): **URL berichtigt**: die gespeicherte `https://`-Adresse ist **nicht abrufbar** — das TLS-Zertifikat der Domain deckt den Hostnamen nicht ab (`CERTIFICATE_VERIFY_FAILED: Hostname mismatch`, geprüft am 30.10.2026); auch `https://www.frisch-recycling.de/` scheitert daran. Über **http** liefert die Seite HTTP 200 (Startseite 94 376 Bytes, `/impressum/` 67 037 Bytes). Die Domain ist damit erreichbar, aber nur unverschlüsselt — die Website-Feld wurde auf die funktionsfähige Form gesetzt, der `website_status` bleibt `aktiv`. Quelle(n): http://frisch-recycling.de/ ; http://frisch-recycling.de/impressum/ (Abruf 30.10.2026)]

### Korrektur 07.11.2026 (Preispflege)

- [Korrektur 07.11.2026 (Preispflege): **Fünf veraltete Current-Preise an der Live-Seite geprüft — alle fünf sind Rückzug.** Der Handler läuft (Lauf 07.11.2026 07:37 UTC, 85 Preise übernommen, 2 Annahmen, 51 übersprungen, Status `ok`) und erzeugt die fünf alten Schlüssel heute nicht mehr — die Pipeline zieht Current-Pointer für normale Handler nicht zurück, darum stehen die alten Werte weiterhin als „aktueller Preis" in der Datenbank. Die Seite https://frisch-recycling.de/ankauf/ (Abruf 07.11.2026 13:01 UTC, http führt per Weiterleitung auf dieselbe Seite) trägt die Preiskarten unter „Unsere Preise" mit dem Einheitsanker „Alle angegebenen Preise sind Kilogramm-Preise (€/kg)". 1) `kabel-kupfer` / „mit Stecker" — 2,50 EUR/kg, Stand 29.09.2026: Preiskarte „Kabel mit Stecker" mit der Preiszelle **„2,50 €"**. Der Betrieb bietet die Sorte mit Preis an, der Handler führt sie aber als eigenes Material `kabel-mit-stecker` / „mit Stecker" (heute 2,50 EUR/kg, Stand 07.11.2026) → **Rückzug** (Fremdkörper). 2) `messing` / „Wasserzähler" — 2,30 EUR/kg, Stand 29.09.2026: Preiskarte „Messing Wasserzähler" mit der Preiszelle **„2,30 €"**; heute `messing-leicht` / „Wasserzähler" (2,30 EUR/kg, Stand 07.11.2026) → **Rückzug** (Fremdkörper). 3) `zinn` / „60/40" — 14,00 EUR/kg, Stand 29.09.2026: Preiskarte „Zinn 60/40" mit der Preiszelle **„14 €"**; heute `loetzinn` / „60/40" (14,00 EUR/kg, Stand 07.11.2026) → **Rückzug** (Fremdkörper, Zinn/Lötzinn-Trennung). 4) `kupfer-gemischt` / „Pellets" — 9,00 EUR/kg, Stand 01.10.2026: **die Sorte fehlt heute** — in den 136 sichtbaren Preiskarten der Seite gibt es keine Karte „Kupfer Pellets"; die Bezeichnung erscheint nur in der versteckten Legacy-Tabelle `table.pricetable`, die auf jedem Viewport ausgeblendet ist und vom Handler ausdrücklich nie gelesen wird (dort steht „Kupfer Pellets | kg | 7,20 €") → **Rückzug** (Sorte fehlt). 5) `kupfer-gemischt` / „Targets" — 8,80 EUR/kg, Stand 01.10.2026: **die Sorte fehlt heute** — sichtbar ist nur die Karte „Alu Cu Targets" mit der Preiszelle **„1,30 €"** (Mischprodukt ohne Katalogmaterial, vom Handler laut übersprungen); „Kupfer Targets" steht nur noch in der versteckten Legacy-Tabelle (`table.pricetable`, dort „Kupfer Targets | kg | 6,90 €") → **Rückzug** (Sorte fehlt). Alle fünf Preise bleiben in der Historie; die aktuellen Zeiger für die heute noch angebotenen Sorten sind frisch (Stand 07.11.2026). Im Dossier ist keine OSM-/Objekt-ID hinterlegt; an der Preiszelle ändert sich nichts, `street`/`postcode`/`lat`/`lon`/`certifications`/`status` bleiben unberührt. Quelle(n): https://frisch-recycling.de/ankauf/ (Abruf 07.11.2026 13:01 UTC).]

### Recherche 08.11.2026

- [Recherche 08.11.2026: **Koordinate belegt: `lat`/`lon` = 47.977364/11.4674728.** Für den Dossier-Eintrag **Frisch Recycling GmbH** trägt das OSM-Objekt node/3731250471 (`addr:housenumber`=3, `addr:postcode`=82067, `addr:city`=Schäftlarn, `addr:place`=Kloster Schäftlarn) die Dossier-Adresse **Kloster Schäftlarn 3 a, 82067 Schäftlarn** mit der passenden Hausnummer; das Objekt führt keinen Betriebsnamen, der Punkt ist deshalb der **Adressmittelpunkt** (der Punkt ist der Adressknoten, BBox 47.977364/11.4674728 bis 47.977364/11.4674728). OSM führt die Adresse mit `addr:place`=Kloster Schäftlarn und der Hausnummer **3** ohne den Zusatz **a**; der Adressabruf löst die Dossier-Adresse auf genau diesen Knoten auf. Quelle(n): https://api.openstreetmap.org/api/0.6/node/3731250471.json ; https://nominatim.openstreetmap.org/search?q=Kloster+Sch%C3%A4ftlarn+3+a%2C+82067+Sch%C3%A4ftlarn&format=jsonv2&extratags=1&namedetails=1&limit=10&addressdetails=1&countrycodes=de ; https://nominatim.openstreetmap.org/search?street=Kloster+Sch%C3%A4ftlarn+3+a&postalcode=82067&city=Sch%C3%A4ftlarn&country=Germany&format=jsonv2&extratags=1&namedetails=1&limit=10&addressdetails=1 (Abruf 08.11.2026).]
