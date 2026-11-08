---
slug: ni-saterland-nfh-zickura
name: NFH Zickura GmbH
trader_type: 'schrotthaendler'
state: NI
city: Saterland
street: 'Rudolf-Diesel-Str. 11'
postcode: '26683'
phone: '04498-9232777'
email: 'info-nfh-zickura@t-online.de'
opening_hours: 'Mo-Fr 08:00-17:00 (Annahme bis 16:30), Sa 09:00-13:00'
website: https://nfh-zickuragmbh.de
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Weitere Verzeichnis-Funde (Nienburg/Diepholz/Verden, Osnabrück-Nord,
  Vechta/Cloppenburg, Peine, Stade, Gifhorn, Lüneburg
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# NFH Zickura GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Alu/Kühler/Blei/VA/E-Motoren/Kupfer

### Recherche 01.10.2026

- [Audit-2 2418 01.10.2026: Audit-2 Feedback 2418 (Zeiten fehlten): Zeiten + Stammdaten auf Betreiber-Website belegt - NFH-Zickura GmbH, Struecklingen, Rudolf-Diesel-Str. 11, 26683 Saterland, Tel. 04498-9232777, info-nfh-zickura@t-online.de; Schrottankauf Eisen/Stahl/Kupfer/Alu/Zink/Blei/Messing, Direktabrechnung (DB-Werte damit im Dossier synchronisiert); Quelle(n): https://nfh-zickuragmbh.de Home + Impressum, 01.10.2026]
- [Audit-2 2418 01.10.2026: Eingearbeitet: street/postcode/phone/email/opening_hours + trader_type sonstige -> schrotthaendler (reiner Schrottankauf)]

### Korrektur 07.11.2026 (Preispflege)

- [Korrektur 07.11.2026 (Preispflege): **Zwei veraltete Current-Preise an der Live-Seite geprüft — beide sind Rückzug (Fremdkörper).** Der Handler läuft (Lauf 07.11.2026 12:09 UTC, 40 Preise übernommen, 7 übersprungen, Status `ok`) und erzeugt die zwei alten Schlüssel heute nicht mehr — die Pipeline zieht Current-Pointer für normale Handler nicht zurück, darum stehen die alten Werte weiterhin als „aktueller Preis“ in der Datenbank. Betroffen sind `mischschrott` / „Blech“ — 80,00 EUR/t und `mischschrott` / „Schwer“ — 200,00 EUR/t (beide Stand 29.09.2026).
  Die Seite https://www.nfh-zickuragmbh.de/preisliste-1 (Abruf 07.11.2026 13:02 UTC; die Handler-URL https://nfh-zickuragmbh.de/preisliste-1 leitet per 301 auf die www-Form um) trägt die Tabellenüberschrift „Material 30.08.2026“ mit der Spalte „€/kg“ und führt die Schrottsorten als Zeilen „Schrott-Misch | 0,180 kg“, „Schrott-Schwer | 0,205 kg“, „Schrott-Neu | 0,215 kg“, „Schrott-Blech | 0,085 kg“ und „Schrott-Späne | 0,110 kg“.
  1) `mischschrott` / „Blech“ — die Zeile **„Schrott-Blech | 0,085 kg“** bietet der Betrieb weiterhin an (0,085 EUR/kg = 85,00 EUR/t), der Handler führt sie aber seit der FE/NE-Auditwelle (Commit c0a19292) als Shredder-Input: heute `stahlschrott-shredder` / „Blech“ (85,00 EUR/t, Stand 07.11.2026 12:09 UTC) → **Rückzug** (Fremdkörper). 2) `mischschrott` / „Schwer“ — die Zeile **„Schrott-Schwer | 0,205 kg“** bietet der Betrieb weiterhin an (0,205 EUR/kg = 205,00 EUR/t), der Handler führt sie aber als Scherenschrott: heute `stahlschrott-scheren` / „Schwer“ (205,00 EUR/t, Stand 07.11.2026 12:09 UTC) → **Rückzug** (Fremdkörper).
  Beide Preise bleiben in der Historie; zurückgezogen werden soll nur der jeweils alte Current-Pointer. Im Dossier ist keine OSM-/Objekt-ID hinterlegt; für die Dossier-Adresse Rudolf-Diesel-Str. 11, 26683 Saterland liefert die OSM-Live-Abfrage (Nominatim, 07.11.2026) kein Objekt mit dieser Hausnummer → **kein Adressobjekt in OSM**. An den Dossierdaten ändert sich nichts, `street`/`postcode`/`lat`/`lon`/`certifications`/`status` bleiben unverändert. Quelle(n): https://www.nfh-zickuragmbh.de/preisliste-1 (Abruf 07.11.2026 13:02 UTC).]

### Geokodierung 08.11.2026

- [Geokodierung 08.11.2026: **Klärfall - keine Koordinate gesetzt.** Die **Rudolf-Diesel-Straße** (26683 Saterland, Gewerbegebiet Bollingen) trägt in OSM **kein** Adressobjekt; die Dossier-Hausnummer **11** existiert dort nicht. **Quelle 1 (OSM, live abgefragt am 08.11.2026):** das Kachelraster **7.6750/53.1260 bis 7.6880/53.1350** (35 Kacheln von je 0,002 Grad, 0 Kachel-Fehler, 2.664 Elemente) enthält auf der Rudolf-Diesel-Straße **kein** Objekt mit `addr:housenumber`; die Straße besteht dort aus einem einzigen Way (**way/85525891**, 9 Knoten, BBox 7.6794118/53.1297828 bis 7.6835298/53.1317825) ohne `addr:*`-Tags und liegt im **Gewerbegebiet Bollingen** (way/85525917, landuse=commercial). Das erweiterte Fenster **7.6700/53.1220 bis 7.6950/53.1400** (20 Kacheln von je 0,005 Grad, 0 Kachel-Fehler, 8.787 Elemente) zeigt adressierte Nachbarstraßen: Am Bollinger Kanal (6 bis 41), Bollinger Ring (1 bis 22), Kanalstraße (1 bis 28), Bahnhofstraße (12 bis 28), In der Lee (1 bis 22), Eschstraße (1 bis 8) und Bollinger Straße (2 bis 12) - das Viertel ist kartiert, die Adressebene der Rudolf-Diesel-Straße fehlt. Eine Namenssuche nach "zickura" und "nfh" über die Felder name, operator, brand, description, website, contact:website und addr:housename findet im Fenster **keinen** Treffer; die einzigen Treffer auf "saterland" sind zwei Radweg-Relationen (relation/16568375, relation/16568376). **Quelle 2 (Nominatim, live abgefragt am 08.11.2026):** die Vorwärtssuche nach "Rudolf-Diesel-Str. 11, 26683 Saterland" liefert **nur** das Straßenobjekt way/85525891, **kein** Adressobjekt. **Warum keine Koordinate:** es existiert kein adressgenaues OSM-Objekt; ein Punkt auf der Straßenmitte wäre eine Ersatzverortung und keine Geokodierung der Betriebsadresse. **Reichweitenklausel:** das Fehlen der Hausnummer 11 in OSM ist **kein** Nachweis für das Fehlen der Betriebsstätte; das Betreiber-Impressum nennt im Dossier dieselbe Anschrift Rudolf-Diesel-Str. 11. `lat`/`lon` bleiben unberührt, alle übrigen Felder bleiben unverändert.]
  Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=7.6750000,53.1260000,7.6880000,53.1350000 ; https://api.openstreetmap.org/api/0.6/map.json?bbox=7.6700000,53.1220000,7.6950000,53.1400000 ; https://nominatim.openstreetmap.org/search?format=jsonv2&extratags=1&namedetails=1&addressdetails=1&countrycodes=de&limit=20&q=Rudolf-Diesel-Str.%2011%2C%2026683%20Saterland ; https://api.openstreetmap.org/api/0.6/way/85525891/full.json (Abruf 08.11.2026).]
