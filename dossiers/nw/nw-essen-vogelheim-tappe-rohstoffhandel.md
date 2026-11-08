---
slug: nw-essen-vogelheim-tappe-rohstoffhandel
name: Tappe Rohstoffhandel GmbH
trader_type: schrotthaendler
state: NW
city: Essen-Vogelheim
street: 'Am Stadthafen 18'
postcode: '45356'
lat: 51.5007419
lon: 6.9715967
phone: 0201 6144122
email: 'tappe-recycling@t-online.de'
opening_hours: 'Mo-Fr 08:00-16:00'
website: https://www.tappe-recycling.de/
website_status: 'aktiv'
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: R4d Neu mit Ankauf-Beleg (ja)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Tappe Rohstoffhandel GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Efb, Tagespreisliste, "Ankauf von Metallen (auch Kleinstmengen)", Container, Abholung (Am Stadthafen 18, 0201 6144122, FB schrottimpott)

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2568 berechtigt (starker Betreiber-Einzelbeleg, offen dokumentiert) — Betreiber-Impressum (Tappe Rohstoffhandel GmbH, GF Detlef Tappe, Am Stadthafen 18, 45356 Essen, HRB 21945 AG Essen, Tel. 0201/6144122, tappe-recycling@t-online.de) + Footer-Detail (Zeiten Mo-Fr 08:00-16:00, Sa/So geschlossen) → street/postcode/email/hours/website_status gefüllt; HR-Spiegel ohne Eintrag unter naheliegender URL (Live-Check gescheitert → keine volle Owner-Ausnahme, Beleglage offen); kein Namensvetter-Hinweis; Koordinaten neu zu geocodieren; Quelle(n): https://www.tappe-recycling.de/impressum/, https://www.tappe-recycling.de/ (Footer)]

### Korrektur 07.11.2026 (Preispflege)

- [Korrektur 07.11.2026 (Preispflege): **Zwei veraltete Current-Preise an der Live-Seite geprüft — beide sind Rückzug (Fremdkörper).** Der Handler läuft (Lauf 07.11.2026 06:36 UTC, 10 Preise übernommen, Status `ok`) und erzeugt die zwei alten Schlüssel heute nicht mehr — die Pipeline zieht Current-Pointer für normale Handler nicht zurück, darum stehen die alten Werte weiterhin als „aktueller Preis“ in der Datenbank. Betroffen sind `aluminium-profile` / „“ — 1,10 EUR/kg und `kupfer-berry` / „“ — 10,10 EUR/kg (beide Stand 27.09.2026).
  Die Seite https://www.tappe-recycling.de/ (Abruf 07.11.2026 13:02 UTC) führt den Block „Aktuelle Schrottpreise“ mit dem Datum **„06.11.2026“** und der Spaltenüberschrift „€ pro kg“; die Zeilen lauten „Mischschrott | 0,170 €“, „Schredderschrott | 0,150 €“, „Kabelschrott (Basis 40% Kupfer) | 3,30 €“, „Aluminium | 1,00 €“, „Blei | 1,00 €“, „Edelstahl | 0,70 €“, „Kupferschrott 1 | ECU/Milb. | 10,90 €“, „Kupferschrott 2 | 9,90 €“, „Messing | 6,20 €“ und „Zink | 1,90 €“.
  1) `aluminium-profile` / „“ — die Zeile **„Aluminium | 1,00 €“** bietet der Betrieb an (Datenbankwert 1,10 EUR/kg), der Handler führt die Sorte heute aber als `aluminium-gemischt` / „“ (1,00 EUR/kg, Stand 07.11.2026 06:36 UTC) → **Rückzug** (Fremdkörper). 2) `kupfer-berry` / „“ — die Zeile **„Kupferschrott 2 | 9,90 €“** bietet der Betrieb an (Datenbankwert 10,10 EUR/kg), der Handler führt sie heute aber als `kupfer-gemischt` / „“ (9,90 EUR/kg, Stand 07.11.2026 06:36 UTC), während „Kupferschrott 1 | ECU/Milb. | 10,90 €“ auf `kupfer-millberry` / „“ läuft → **Rückzug** (Fremdkörper).
  Beide Preise bleiben in der Historie; zurückgezogen werden soll nur der jeweils alte Current-Pointer. Im Dossier ist keine OSM-/Objekt-ID hinterlegt; die Dossier-Adresse Am Stadthafen 18, 45356 Essen ist in OSM als way/162742930 geführt (`addr:housenumber`=18, `addr:street`=Am Stadthafen, `addr:postcode`=45356, `addr:city`=Essen; live abgerufen 07.11.2026). An den Dossierdaten ändert sich nichts, `street`/`postcode`/`lat`/`lon`/`certifications`/`status` bleiben unverändert. Quelle(n): https://www.tappe-recycling.de/ (Abruf 07.11.2026 13:02 UTC).]

### Geokodierung 08.11.2026

- [Recherche 08.11.2026: **Koordinate der Dossier-Adresse belegt** (`lat`/`lon` = **51.5007419/6.9715967**). Punktgüte: **Hausnummer-Treffer**; der Dossierbetrieb „Tappe Rohstoffhandel GmbH“ ist über die Adresse **'Am Stadthafen 18', 45356 Essen-Vogelheim** adressiert; das OSM-Adressobjekt **w/162742930** trägt `addr:housenumber`=18, `addr:street`=Am Stadthafen, `addr:postcode`=45356, `addr:city`=Essen; der Punkt ist der Mittelwert der eindeutigen Way-Knoten; das Objekt trägt **weder `name=` noch `operator=`** - die Koordinate belegt die **Anschrift**, nicht die Betreiberzuordnung; die Nominatim-Vorwärtssuche nach „Am Stadthafen 18, 45356 Essen-Vogelheim“ liefert denselben Objektpunkt. Quelle(n): https://nominatim.openstreetmap.org/search?format=jsonv2&extratags=1&namedetails=1&addressdetails=1&countrycodes=de&limit=20&q=Am%20Stadthafen%2018%2C%2045356%20Essen-Vogelheim ; https://api.openstreetmap.org/api/0.6/way/162742930.json (Abruf 08.11.2026).]
