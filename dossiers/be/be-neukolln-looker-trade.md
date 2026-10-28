---
slug: be-neukolln-looker-trade
name: LOOKER TRADE UG
trader_type: sonstige
state: BE
city: Neukölln
street: Hermannstr. 16
postcode: '12049'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: A. Neu (nicht in Hauptliste / Runden 2–3)
provenance_ankauf_raw: unsicher
provenance_origin: table
---

# LOOKER TRADE UG

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 01.10.2026 (Tiefenrecherche-Welle)

- [Recherche 01.10.2026 (Tiefenrecherche-Welle): KEIN neuer Beleg + Frontmatter-Fix — 11880-Suche „Looker/Neukölln" ohne Treffer (nur Foot Locker etc.); Domain looker.de via Direktabruf 2× nicht erreichbar (Transportfehler — tot/blockiert ununterscheidbar, daher website NICHT gefüllt, website_status unbekannt); HRB 136189 (Seed) ohne abrufbaren Registerbeleg (Northdata-Suche JS-geschützt). Regel-Fix: phone „—" → '' (Emdash in phone/email verboten). Alles leer, Klärfall (Handelsregister Berlin). Quelle(n): https://www.11880.com/suche/looker/neukoelln (Negativbeleg), Direktabruf looker.de (Fehler)]

### Importiert (Seed-Stand 2026-09-30)

- nur Das Telefonbuch (Branche Schrott); Domain wieder live (war tot gemeldet) Adresse: https://looker.de
- Bezirk: Neukölln Adressbeleg: handelsregister.ai/northdata HRB 136189 (Websuche 28.09.2026; kein Tel. verifiziert)
