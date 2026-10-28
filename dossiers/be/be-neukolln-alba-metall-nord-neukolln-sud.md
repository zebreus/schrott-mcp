---
slug: be-neukolln-alba-metall-nord-neukolln-sud
name: ALBA Metall Nord (Neukölln-Süd)
trader_type: sonstige
state: BE
city: Neukölln
street: Ziegrastr. 2-46
postcode: '12057'
phone: ''
email: ''
opening_hours: ''
website: https://metall.alba.info
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: A. Kept (Tabelle)
provenance_ankauf_raw: ja (Gewerbe)
provenance_origin: table
---

# ALBA Metall Nord (Neukölln-Süd)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Importiert (Seed-Stand 2026-09-30)

- Fe/NE, Kette/Filiale (B2B ok)
- Adresse: Neukölln, Ziegrastr. 2-46, 12057 Berlin (Ex-Interseroh = ALBA, kein Doppel)
- Bezirk: Neukölln Adressbeleg: seed/be.json (https://metall.alba.info)
- Adressbeleg: https://metall.alba.info

### Korrektur 01.10.2026 (ALBA-Fix)

- Delisted-Protokoll: Neukölln (Ziegrastr.) steht auf KEINER aktuellen Betreiber-Liste (Vollcrawl 01.10.2026: metall.alba.info = 13 Detailseiten; alba.info/standorte: weder Metall Nord noch ALBA Berlin GmbH mit Neukölln-Adresse).
- Betreiber-Suche/Register/News ohne Schließungs-/Verlagerungsbeleg → kein status geschlossen, Dossier bleibt erhalten; status aktiv → pruefung + Klärfall: Filial-Existenz und Seed-These „Ex-Interseroh = ALBA, kein Doppel“ (aus street-Feld hierher archiviert) weiter ohne Betreiber-Beleg.
- street normalisiert (Ziegrastr. 2-46, 12057 Berlin); website = Domain-Root (aktiv) ohne Detailseite.

### Korrektur 01.10.2026 (Owner-Gate)

- [Owner-Gate 01.10.2026: Gedankenstrich-Datenmüll → leer (Shard hat Street bereinigt, Phone übersehen).]

### Recherche 01.10.2026 (Feedback-Triage 2858)

- [Recherche 01.10.2026: Re-Verifikation Betreiber-Standortliste (13 Treffer — kein Neukölln/Ziegrastraße); Befund aus Korrektur 01.10.2026 bestätigt, kein Schließungs-/Verlagerungsbeleg; Quellen: https://metall.alba.info/unternehmen/standorte/]
- Feedback 2858 (Standort nicht gelistet) berechtigt; Klärfall Filial-Existenz bleibt.
