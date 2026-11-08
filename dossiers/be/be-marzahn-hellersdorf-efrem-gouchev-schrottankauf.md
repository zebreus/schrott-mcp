---
slug: be-marzahn-hellersdorf-efrem-gouchev-schrottankauf
name: Efrem Gouchev Schrottankauf
trader_type: schrotthaendler
state: BE
city: Marzahn-Hellersdorf
street: Bitterfelder Str. 23
postcode: '12681'
phone: 030 99272366
email: info@schrottankauf-bitterfelderstr23.de
opening_hours: Mo-Do 9:00-17:30, Fr 9:00-16:30, Sa 9:00-12:30
website: https://www.schrottankauf-bitterfelderstr23.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Berlin (BE) — Recherche
provenance_ankauf_raw: ja
provenance_origin: table
---

# Efrem Gouchev Schrottankauf

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Generalist inkl. Zinn/Zink/Hartmetall
- Adresse: Bitterfelder Str. 23, 12681 Berlin
- Bezirk: Marzahn-Hellersdorf Adressbeleg: seed/be.json (https://www.schrottankauf-bitterfelderstr23.de)

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Website schrottankauf-bitterfelderstr23.de verifiziert (Start + Impressum): Bitterfelder Str. 23, 12681 Berlin deckungsgleich; Telefon 030 99272366, E-Mail info@schrottankauf-bitterfelderstr23.de (beide DB-gleich) und Zeiten Mo-Do 9:00-17:30, Fr 9:00-16:30, Sa 9:00-12:30 gefüllt; Mobil 0176 62087271 nur Timeline-Vermerk; Quelle(n): https://www.schrottankauf-bitterfelderstr23.de/ + https://www.schrottankauf-bitterfelderstr23.de/impressum]

### Korrektur 08.11.2026 (Preispflege)

- [Korrektur 08.11.2026 (Preispflege): **26 veraltete Current-Preise an der Live-Seite geprüft: 25 Rückzüge und ein Mapping-Gap.** Der Handler läuft (Lauf 08.11.2026 08:33 UTC, 30 Preise übernommen, 3 übersprungen, Status `ok`) und erzeugt die 26 alten Schlüssel heute nicht mehr; die Pipeline zieht Current-Pointer für normale Handler nicht zurück, darum stehen die alten Werte weiter als „aktueller Preis" in der Datenbank.
  Die Seite https://www.schrottankauf-bitterfelderstr23.de/schrottpreise (Abruf 08.11.2026, HTTP 200, 56 010 Bytes) zeigt die Preiskarten heute **ohne** Mengenstufen, nur „ab 1 kg"; Mengenstufen („ab 200 kg", „ab 1000 kg") tragen **nur** die Karten Kupfer, Kupfer Millberry, Messing, Kupferkabel, Kupfer Schwer und Kupfer Kerze. Der Archivstand vom 16.06.2026 trug dagegen auf allen Karten eine Staffel (nur die Aluminiumkabel-Karte nicht), darunter „Kupferkabelschrott mit Stecker" (1,00 / 1,10 / 1,20), „Mischschrott" (0,12 / 0,12 / 0,13), „Altblei" (0,95 / 1,05 / 1,15), „Auswuchtblei" (0,15 / 0,20 / 0,25), „Kabelschälblei" (0,30 / 0,40 / 0,50), „Elektromotoren" (0,35 / 0,40 / 0,45), „Zinnschrott Lötzinn" (5,00 / 6,00 / 7,00) und „Zink" (1,60 / 1,70 / 1,80). Der Betrieb hat die Mengenstufen zwischen dem 04.11. und dem 05.11.2026 von diesen Karten genommen (Seitengröße 56 215 Bytes am 04.11., 53 672 Bytes am 05.11., 53 682 Bytes am 06.11., 55 978 Bytes am 07.11. und 08.11.2026; der Handler übernahm am 04.11. noch 56 Preise, am 05.11. und 06.11. je 31 und am 07.11. und 08.11. je 30).
  **25 Rückzüge**, Karte für Karte mit dem wörtlichen Kartentext vom 08.11.2026 (alle Mengenstufen-Preise mit Stand 04.11.2026, `mischschrott` in EUR/t):
  Karte „Aluminiumschrott mit max. 5% Anhaftung ab 1 kg ➜ 1,00€": `aluminium-gemischt` / „5% Anhaftung, ab 200 kg Überweisung" 1,15 EUR/kg und `aluminium-gemischt` / „5% Anhaftung, ab 1000 kg Überweisung" 1,25 EUR/kg → Rückzug, die Karte trägt heute keine Mengenstufe.
  Karte „Alu Profile 1,90€ Alu Sorte1. 1,10€ Alu Felgen 1,60€ Alu Kupferkühler 3,90€ Alu Guss 1,10€": `aluminium-gemischt` / „ab 200 kg Überweisung" 1,20 EUR/kg und `aluminium-gemischt` / „ab 1000 kg Überweisung" 1,30 EUR/kg → Rückzug, die Karte trägt heute keine Mengenstufe.
  Karte „Altblei ohne Fett ab 1 kg ➜ 1,00 €": `blei` / „ab 200 kg Überweisung" 1,10 EUR/kg und `blei` / „ab 1000 kg Überweisung" 1,20 EUR/kg → Rückzug.
  Karte „Auswuchtblei ab 1 kg ➜ 0,15€": `blei-auswucht` / „Auswuchtblei, ab 200 kg Überweisung" 0,15 EUR/kg und `blei-auswucht` / „Auswuchtblei, ab 1000 kg Überweisung" 0,15 EUR/kg → Rückzug.
  Karte „Kabelschälblei ab 1 kg ➜ 0,30€": `blei-auswucht` / „Kabelschälblei, ab 200 kg Überweisung" 0,40 EUR/kg und `blei-auswucht` / „Kabelschälblei, ab 1000 kg Überweisung" 0,50 EUR/kg → Rückzug.
  Karte „Edelstahlschrott (V2A) ab 1 kg ➜ 0,55€": `edelstahl-v2a` / „ab 200 kg Überweisung" 0,55 EUR/kg und `edelstahl-v2a` / „ab 1000 kg Überweisung" 0,55 EUR/kg → Rückzug.
  Karte „Elektromotoren ohne Anbauteile ab 1 kg ➜ 0,45€": `elektromotoren` / „ab 200 kg Überweisung" 0,60 EUR/kg und `elektromotoren` / „ab 1000 kg Überweisung" 0,70 EUR/kg → Rückzug.
  Karte „Kupferkabelschrott mit Stecker ab 1 kg ➜ 1,00€": `kabel-mit-stecker` / „mit Stecker, ab 200 kg Überweisung" 1,10 EUR/kg und `kabel-mit-stecker` / „mit Stecker, ab 1000 kg Überweisung" 1,20 EUR/kg → Rückzug.
  Karte „Zinnschrott Lötzinn ab 1 kg ➜ 5,00€": `loetzinn` / „Lötzinn, ab 200 kg Überweisung" 6,00 EUR/kg und `loetzinn` / „Lötzinn, ab 1000 kg Überweisung" 7,00 EUR/kg → Rückzug.
  Karte „Mischschrott ab 1 kg ➜ 0,10€": `mischschrott` / „ab 200 kg Überweisung" 100,00 EUR/t und `mischschrott` / „ab 1000 kg Überweisung" 110,00 EUR/t → Rückzug.
  Karte „Zink ab 1 kg ➜ 1,70€": `zink` / „ab 200 kg Überweisung" 1,90 EUR/kg und `zink` / „ab 1000 kg Überweisung" 2,00 EUR/kg → Rückzug.
  Karte „Kupfer verzinnt ab 1 kg ➜ 9,75€ ab 200kg: 9,85€/ 9,75€ Überweisung/ Bar ab 1000 kg: 9,95€/ 9,75€ Überweisung/ Bar": `zinn-geschirr` / „90-95%, ab 200 kg Überweisung" 11,00 EUR/kg und `zinn-geschirr` / „90-95%, ab 1000 kg Überweisung" 12,00 EUR/kg (Stand 04.11.2026) sowie `zinn-geschirr` / „90-95%" 10,00 EUR/kg (Stand 06.11.2026) → Rückzug.
  Zur Herkunft des Paares `zinn-geschirr` / „90-95%" (10,00 EUR/kg, Stand 06.11.2026): es ist kein Fremdkörper, sondern derselbe Kartenbestand. Der Karten-Slot `teller` trug am 16.06.2026 den Titel „Zinnschrott 90-95 % (Teller)" mit „ab 1 kg ➜ 10,00€ ab 200 kg: 11,00€ /10,00€ Überweisung /Bar ab 1000 kg: 12,00€ /10,00€ Überweisung /Bar"; am 06.11.2026 lief der Handler noch mit 31 Preisen und schrieb die Zeile `zinn-geschirr` / „90-95%" 10,00 EUR/kg unter diesem Titel an, als die Karte nur noch den Grundpreis ohne Staffel trug. Seit dem 07.11.2026 heißt derselbe Slot „Kupfer verzinnt" (9,75 / 9,85 / 9,95 EUR/kg), der Titel steht in einem einfachen `<p>` statt in `p.h5`, und der Handler meldet den Block als „(Preisblock ohne Titel, übersprungen)"; der Schlüssel `zinn-geschirr` / „90-95%" entsteht heute nicht mehr → Rückzug.
  **Ein Mapping-Gap, kein Rückzug:** `aluminium-gemischt` / „" 1,10 EUR/kg (Stand 04.11.2026) wird weiterhin angeboten, und zwar als „Alu Sorte1. 1,10€" im Block „Alu Profile 1,90€ Alu Sorte1. 1,10€ Alu Felgen 1,60€ Alu Kupferkühler 3,90€ Alu Guss 1,10€" (der Archivstand vom 16.06.2026 trug denselben Slot noch als „Aluminium ohne Anhaftung Farbe darf drauf sein ab 1 kg ➜ 1,10€"). Der Handler überspringt den Block, weil er den Marker „ab 1 kg ➜" nicht trägt; der Lauf vom 08.11.2026 meldet ihn als „Alu Profile 1,90€ Alu Sorte1. 1,10€ Alu Felgen 1,60€Alu Kupferkühler 3,90€ (kein ab-1-kg-Preis)". Selbst mit dem Marker bliebe der Block ohne Zuordnung, weil `grade_for` in `crates/ingestion/src/traders/handlers/gouchev.rs` für „aluminium" bzw. „5 %" prüft und der Titel weder das eine noch das andere enthält. Die Zuordnung ist eine Entscheidung des Owners und wird hier **nicht** vorgenommen; das Paar steht deshalb **nicht** in der Rückzugsliste.
  Die 25 Rückzüge stehen als `scripts/release/stale_current_prices_12.tsv` (Format wie `crates/store/src/public/stale_current_prices.tsv`, sieben Felder slug|material|variant|preis|einheit|beobachtet|quelle); den Migrationscode der Migration 12 schreibt der Owner. An den Dossierdaten ändert sich nichts, `street`/`postcode`/`lat`/`lon`/`certifications`/`status` bleiben unverändert. Quelle(n): https://www.schrottankauf-bitterfelderstr23.de/schrottpreise (Abruf 08.11.2026); http://web.archive.org/web/20260616175822/https://www.schrottankauf-bitterfelderstr23.de/schrottpreise (Abruf 08.11.2026).]
