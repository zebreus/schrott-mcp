---
slug: be-mitte-rheinische-scheidestatte-berlin-mitte
name: Rheinische Scheidestätte GmbH (Berlin-Mitte)
trader_type: sonstige
state: BE
city: Mitte
street: Friedrichstraße 172
postcode: '10117'
phone: ''
email: info-berlin-mitte@rheinische-scheidestaette.de
opening_hours: Mo-Fr 9:30-18:00, Sa geschlossen
website: https://rheinische-scheidestaette.de
website_status: ''
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: A. Kept (Tabelle)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Rheinische Scheidestätte GmbH (Berlin-Mitte)

## Überblick

Die Rheinische Scheidestätte GmbH betreibt am Friedrichstraße 172, 10117 Berlin-Mitte, eine
Filiale für Scheideanstalt, Dentalgold und Tageskurse. Der Betreiberauftritt
rheinische-scheidestaette.de/unternehmen/filialen/ bestätigt die Anschrift in der
Schreibweise "Friedrichstraße 172"; Mail info-berlin-mitte@rheinische-scheidestaette.de und
die Zeiten Mo-Fr 9:30-18:00 bei geschlossenem Samstag stammen aus der Seed-Unterlage. Die
Seed-Notiz verweist zusätzlich auf eine Seite mit aktuellen Edelmetallkursen, ohne dass Kurse
ausgewiesen wären. Die Rufnummer 030-2593200 wird laut Korrekturvermerk vom 28.10.2026 von
einem 6h-Handler gepflegt, das Telefonfeld bleibt leer. Offen bleibt, dass eine zweite
unabhängige Quelle fehlt: Rechtsform, Registereintrag und Impressum sind nicht belegt.

## Timeline

- [Owner-Korrektur 28.10.2026 (Korrekturvermerk): **Die Straße ist auf die Schreibweise des Betreibers normalisiert.** Das Dossier trug `Mitte, Friedrichstr. 172`, die Produktionszeile führt `Friedrichstraße 172` (von dem 6h-Handler aus dem Betreiberauftritt geschrieben). Beide Schreibweisen meinen dieselbe Anschrift, aber der Seed schreibt das Dossier-Feld direkt in die Zeile und der Handler schreibt es zurück: ohne Normalisierung wechselt die Straße nach jedem Seed-Lauf zwischen beiden Formen. Der Betreiberauftritt nennt die Anschrift heute als `Friedrichstraße 172` (live gelesen am 28.10.2026), das Feld `city` bleibt unverändert. Quelle(n): https://rheinische-scheidestaette.de/unternehmen/filialen/ ; read-only `public.db` (28.10.2026).]

- [Owner-Korrektur 28.10.2026 (Korrekturvermerk): **Die nachstehende Notiz trifft für diese Zeile nicht zu und wird hiermit berichtigt.** Sie behauptet, in der Produktion habe der Platzhalter „—“ gestanden - tatsächlich steht dort die **heute von einem 6h-Handler gepflegte** Rufnummer (030-2593200, Betreiber Rheinische Scheidestätte Berlin-Mitte); der `updated_at`-Zeitpunkt der Zeile fällt exakt auf den Handlerlauf. Der `clear_fields: 'phone'`-Direktive ist deshalb für diese Zeile **entfernt**: Er hätte die belegte Nummer gelöscht und der nächste Handlerlauf hätte sie zurückgeschrieben (Oszillation). Das Dossier-Telefonfeld bleibt **leer** (unbekannt) - der Seed überschreibt die Handler-Nummer damit nicht, die belegte Nummer bleibt im Bestand erhalten. Für die 47 Zeilen, in denen die Produktion tatsächlich einen Platzhalter führte, bleibt der Clear unverändert - dort schreibt kein Handler zurück, der Clear ist also stabil. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive) ; `crates/store/src/public/traders.rs` (`set_trader_info` überschreibt vorhandene Werte).]
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Im Dossier stand „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe. Das Feld ist jetzt **leer** (unbekannt) und `clear_fields: 'phone'` nimmt den in der Produktion gespeicherten Platzhalter beim nächsten regulären Seed zurück; es wird **keine** Nummer erfunden. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Importiert (Seed-Stand 2026-09-30)

- Scheideanstalt, Dentalgold, Tageskurse. PREISLISTE: https://rheinische-scheidestaette.de/infothek/aktuelle-edelmetallkurse/
- Adresse: Mitte, Friedrichstr. 172, 10117 Berlin
- Bezirk: Mitte Adressbeleg: seed/be.json (https://rheinische-scheidestaette.de)
- Adressbeleg: https://rheinische-scheidestaette.de/unternehmen/filialen/
