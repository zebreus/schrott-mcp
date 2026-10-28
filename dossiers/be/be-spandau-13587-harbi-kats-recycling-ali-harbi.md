---
slug: be-spandau-13587-harbi-kats-recycling-ali-harbi
name: Harbi Kats Recycling (Ali Harbi)
trader_type: sonstige
state: BE
city: Spandau 13587
street: Rauchstraße 43
postcode: '13587'
phone: ''
email: info@harbi-kats.de
opening_hours: ''
website: https://harbi-kats.de
website_status: ''
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Wide-Net (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Harbi Kats Recycling (Ali Harbi)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

- [Owner-Korrektur 28.10.2026 (Korrekturvermerk): **Die Straße ist auf die Schreibweise des Betreibers normalisiert.** Das Dossier trug `Rauchstr. 43`, die Produktionszeile führt `Rauchstraße 43` (von dem 6h-Handler aus dem Betreiberauftritt geschrieben). Beide Schreibweisen meinen dieselbe Anschrift, aber der Seed schreibt das Dossier-Feld direkt in die Zeile und der Handler schreibt es zurück: ohne Normalisierung wechselt die Straße nach jedem Seed-Lauf zwischen beiden Formen. Der Betreiberauftritt nennt die Anschrift heute als `Rauchstraße 43` (live gelesen am 28.10.2026), das Feld `city` bleibt unverändert. Quelle(n): https://harbi-kats.de/impressum/ ; read-only `public.db` (28.10.2026).]

- [Owner-Korrektur 28.10.2026 (Korrekturvermerk): **Die nachstehende Notiz trifft für diese Zeile nicht zu und wird hiermit berichtigt.** Sie behauptet, in der Produktion habe der Platzhalter „—“ gestanden - tatsächlich steht dort die **heute von einem 6h-Handler gepflegte** Rufnummer (01795950837, Betreiber Harbi-Kats); der `updated_at`-Zeitpunkt der Zeile fällt exakt auf den Handlerlauf. Der `clear_fields: 'phone'`-Direktive ist deshalb für diese Zeile **entfernt**: Er hätte die belegte Nummer gelöscht und der nächste Handlerlauf hätte sie zurückgeschrieben (Oszillation). Das Dossier-Telefonfeld bleibt **leer** (unbekannt) - der Seed überschreibt die Handler-Nummer damit nicht, die belegte Nummer bleibt im Bestand erhalten. Für die 47 Zeilen, in denen die Produktion tatsächlich einen Platzhalter führte, bleibt der Clear unverändert - dort schreibt kein Handler zurück, der Clear ist also stabil. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive) ; `crates/store/src/public/traders.rs` (`set_trader_info` überschreibt vorhandene Werte).]
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Im Dossier stand „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe. Das Feld ist jetzt **leer** (unbekannt) und `clear_fields: 'phone'` nimmt den in der Produktion gespeicherten Platzhalter beim nächsten regulären Seed zurück; es wird **keine** Nummer erfunden. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Importiert (Seed-Stand 2026-09-30)

- Kat/DPF-Ankauf — Impressum + Katalog + 4 Kleinanzeigen + Facebook; Barzahlung (NEU → HANDLER-KANDIDAT prüfen)
- Adresse: Spandau 13587, Rauchstr. 43
- Bezirk: Spandau Adressbeleg: harbi-kats.de Impressum + Kleinanzeigen (be.md Wide-Net; Kat-Ankauf)
- Adressbeleg: https://harbi-kats.de
