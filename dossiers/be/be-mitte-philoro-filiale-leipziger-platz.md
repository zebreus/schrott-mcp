---
slug: be-mitte-philoro-filiale-leipziger-platz
name: philoro (Filiale Leipziger Platz)
trader_type: sonstige
state: BE
city: Mitte
street: Mitte, Leipziger Platz 1
postcode: '10117'
phone: ''
email: berlin@philoro.de
opening_hours: Mo-Fr 10-18, Sa 10-15
website: https://philoro.de
website_status: ''
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: A. Kept (Tabelle)
provenance_ankauf_raw: ja
provenance_origin: table
---

# philoro (Filiale Leipziger Platz)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

- [Owner-Korrektur 28.10.2026 (Korrekturvermerk): **Die vorstehende Notiz trifft für diese Zeile nicht zu und wird hiermit berichtigt.** Sie behauptet, in der Produktion habe der Platzhalter „—“ gestanden - tatsächlich steht dort die **heute von einem 6h-Handler gepflegte** Rufnummer (+49 30 206 33 99 50, Betreiber philoro); der `updated_at`-Zeitpunkt der Zeile fällt exakt auf den Handlerlauf. Der `clear_fields: 'phone'`-Direktive ist deshalb für diese Zeile **entfernt**: Er hätte die belegte Nummer gelöscht und der nächste Handlerlauf hätte sie zurückgeschrieben (Oszillation). Das Dossier-Telefonfeld bleibt **leer** (unbekannt) - der Seed überschreibt die Handler-Nummer damit nicht, die belegte Nummer bleibt im Bestand erhalten. Für die 47 Zeilen, in denen die Produktion tatsächlich einen Platzhalter führte, bleibt der Clear unverändert - dort schreibt kein Handler zurück, der Clear ist also stabil. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive) ; `crates/store/src/public/traders.rs` (`set_trader_info` überschreibt vorhandene Werte).]
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Im Dossier stand „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe. Das Feld ist jetzt **leer** (unbekannt) und `clear_fields: 'phone'` nimmt den in der Produktion gespeicherten Platzhalter beim nächsten regulären Seed zurück; es wird **keine** Nummer erfunden. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Importiert (Seed-Stand 2026-09-30)

- Edelmetall, Dental-/Altgold, Altgold-Rechner
- (Filiale Leipziger Platz)
- Adresse: Mitte, Leipziger Platz 1, 10117 Berlin
- Bezirk: Mitte Adressbeleg: seed/be.json (https://philoro.de)
- Adressbeleg: https://philoro.de/filialen/berlin
