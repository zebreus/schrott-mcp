---
slug: bb-schoneiche-hein-schrotthandel
name: HEIN Schrotthandel GmbH
trader_type: schrotthaendler
state: BB
city: Schöneiche
street: Werner-von-Siemens-Str. 12
postcode: '15566'
phone: 030 64387710
email: info@hein-schrotthandel.de
opening_hours: Mo-Fr 8:00-16:00, Sa 9:00-13:00
website: https://www.hein-schrotthandel.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: 'Nachtrag Audit-Runde 3 (Stand: 2026-09-27)'
provenance_ankauf_raw: ja
provenance_origin: table
---

# HEIN Schrotthandel GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrotthandel
- Adressbeleg: hein-schrotthandel.de

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Website hein-schrotthandel.de verifiziert (Kontakt): Werner-von-Siemens-Str. 12, 15566 Schöneiche und Tel. 030 64387710 deckungsgleich (DB-Schreibweise 030 64 38 77 10 identisch); E-Mail info@hein-schrotthandel.de (DB-gleich) und Zeiten Mo-Fr 8:00-16:00, Sa 9:00-13:00 gefüllt; Mobil 0152 34326391 nur Timeline-Vermerk; Quelle(n): https://www.hein-schrotthandel.de/kontakt/]

### Preispflege 07.11.2026

- [Preispflege 07.11.2026: **Zwei veraltete Current-Preise an der Live-Seite geprüft — beide Fälle sind Rückzug (Fremdkörper), keine Preisänderung am Betrieb.** Der Handler läuft (Lauf 07.11.2026 02:19 UTC, 95 Preise übernommen, Status `ok`), er erzeugt die beiden alten Schlüssel (material, variant) heute aber nicht mehr — die Pipeline zieht Current-Pointer für normale Handler nicht zurück, darum stehen die alten Werte weiterhin als „aktueller Preis" in der Datenbank. 1) `kabel-kupfer` / „Blei" — 0,45 EUR/kg, Stand 29.09.2026: Die Seite https://www.hein-schrotthandel.de/schrottpreise/ (Abruf 07.11.2026 13:01 UTC) führt die Sorte in der Tabelle „Material | Preis (pro Tonne)" als Zeile „Kupfer-Blei-Kabel ohne Teer (fettfrei)" mit der Preiszelle **„450,00 €"** (pro Tonne, also 0,45 EUR/kg). Die Sorte wird heute also noch angeboten — aber nicht mehr als Kupferkabel: der Handler führt sie (Remap nach Feedback #133) als eigenes Material `kabel-blei` / „Blei" (heute 0,45 EUR/kg, Stand 07.11.2026). Der Schlüssel (kabel-kupfer, „Blei") wird heute nicht mehr erzeugt → **Rückzug** (der Preis bleibt in der Historie). 2) `messing` / „Wasseruhren" — 1,10 EUR/kg, Stand 29.09.2026: dieselbe Seite, Zeile „Messing Wasseruhren", Preiszelle **„1.100,00 €"** (pro Tonne, also 1,10 EUR/kg). Die Sorte wird heute angeboten, aber als leichtes Messing: der Handler führt sie als `messing-leicht` / „Wasseruhren" (heute 1,10 EUR/kg, Stand 07.11.2026). Der alte Schlüssel (messing, „Wasseruhren") ist damit ein Fremdkörper → **Rückzug**. Beide aktuellen Zeiger existieren bereits und sind frisch (Stand 07.11.2026); zurückgezogen werden soll nur der jeweils alte Current-Pointer. Im Dossier ist keine OSM-/Objekt-ID hinterlegt; an der Preiszelle ändert sich nichts, `street`/`postcode`/`lat`/`lon`/`certifications`/`status` bleiben unberührt. Quelle(n): https://www.hein-schrotthandel.de/schrottpreise/ (Abruf 07.11.2026 13:01 UTC).]
