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
