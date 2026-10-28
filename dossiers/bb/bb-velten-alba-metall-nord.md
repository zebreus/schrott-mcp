---
slug: bb-velten-alba-metall-nord
name: ALBA Metall Nord GmbH
trader_type: schrotthaendler
state: BB
city: Velten
street: Breite Str. 47b
postcode: '16727'
phone: ''
email: ''
opening_hours: ''
website: https://metall.alba.info/
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: 'Nachtrag Audit-Runde 3 (Stand: 2026-09-27)'
provenance_ankauf_raw: ja
provenance_origin: table
---

# ALBA Metall Nord GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „n.e.“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Importiert (Seed-Stand 2026-09-30)

- Stahl-/Metallschrott
- Adressbeleg: schrottradar-Beleg via Runde 3

### Korrektur 01.10.2026 (ALBA-Fix)

- [Korrektur 01.10.2026 (ALBA-Fix): Filiale NICHT auf Betreiber-Seite (Vollcrawl 13 Detailseiten: kein Velten/Breite Str.); status aktiv → pruefung + Klärfall. KEINE Schließungs-/Verlagerungsbelege (Betreiber-Suche, Websuche ergebnislos), KEINE Frontmatter-Füllung, nichts gelöscht. Klärfall: Existenz/Fortbestand Breite Str. 47b, 16727 Velten (Register/HR prüfen). Quelle(n): metall.alba.info/unternehmen/standorte (live, Vollcrawl 01.10.2026)]

### Recherche 01.10.2026 (Feedback-Triage 2139)

- [Recherche 01.10.2026 (Feedback-Triage 2139): Re-Meldung triagiert — Standort Velten (Breite Str. 47b, 16727) weiterhin nicht auf ALBA-Metall-Standortliste (13 Standorte, kein Velten); ALBA-Fix vom 01.10. (status pruefung, Klärfall) bestätigt und bleibt; keine Änderung, keine Fills (phone/email/hours leer, kein Schließungsbeleg). Quelle(n): https://metall.alba.info/unternehmen/standorte/]
