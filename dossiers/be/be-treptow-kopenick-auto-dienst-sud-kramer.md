---
slug: be-treptow-kopenick-auto-dienst-sud-kramer
name: Auto-Dienst Süd Krämer GmbH
trader_type: autoverwertung
state: BE
city: Treptow-Köpenick
street: Kiefholzstr. 397
postcode: '12435'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: Schrottankauf Berlin (BE) — Recherche
provenance_ankauf_raw: ja — Autoverwertung
provenance_origin: table
---

# Auto-Dienst Süd Krämer GmbH

## Überblick

Das Dossier nennt Auto-Dienst Süd Krämer GmbH an der Kiefholzstraße 397 als Autoverwertung, doch die neu auffindbaren Profile sind Branchenverzeichnisse und ordnen das Unternehmen teils allgemein Kfz/Autohäusern zu. Es fehlt ein aktueller Betreiber-, Register- oder Behördenbeleg speziell für die Autoverwertung; Status deshalb `pruefung`. Der sichtbare Telefonnummer-Platzhalter wurde nicht als Kontakt übernommen. **GEO-Prüfbedarf:** Ein Portal nennt direkt nebenan Kiefholzstr. 394 als „Acit Auto Center in Treptow GmbH“; bei möglicher Adressverifizierung auf Hausnummer 397 und konkreten Hof-/Einfahrtspunkt achten, nicht auf die Nachbarfirma oder einen blockweiten Pin snappen. Keine zuordenbaren ANKAUF-Sätze, VERKAUF-Preise oder Gebühren.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Nachprüfung 26.10.2026

- [Nachprüfung 26.10.2026: Das Örtliche-Profil direkt geöffnet: „Auto-Dienst Süd Krämer GmbH Autoverwertung“, Kiefholzstr. 397, 12435 Berlin-Alt-Treptow, 030 5337676. Das bleibt ein Aggregator-Lead, kein Betreiber-/Registerbeleg und keine aktuelle Annahmebestätigung; Telefonnummer deshalb nicht gefüllt. Das Profil listet als eigenen benachbarten Treffer „Acit Auto Center in Treptow GmbH“, Kiefholzstr. 394; die abweichende Hausnummer spricht gegen ein automatisches Setzen des Krämer-Pins auf den Nachbarpunkt, beweist aber weder Mehrmieterstatus noch Krämer-Betriebsfläche. ANKAUF: keine Fahrzeug-/Schrottpreise; VERKAUF: keine Teileliste; GEBÜHREN: keine Tarife. Vor GEO Gewerbe-/Betreiberbestätigung, genaue Grundstücksgrenze und Kundenzufahrt ermitteln. Quelle (Lead): https://www.dasoertliche.de/Themen/Auto-Dienst-S%C3%BCd-Kr%C3%A4mer-GmbH-Autoverwertung-Berlin-Alt-Treptow-Kiefholzstr .]

### Importiert (Seed-Stand 2026-09-30)

- Autoverwertung
- Adresse: Kiefholzstr. 397, 12435 Berlin
- Bezirk: Treptow-Köpenick Adressbeleg: seed/be.json + recherche/be.md (Verzeichnisbeleg)

### Recherche 07.10.2026

- [Recherche 07.10.2026: Websuche fand Profile für „Auto-Dienst Süd Krämer GmbH“ bei Stadtbranchenbuch (Kiefholzstr. 397, 12435 Berlin, Telefon 030 5337676; Kategorie Autoverwertung) und Business-Branchenbuch (Kfz/Autohäuser, Telefon 030 5337676). Beide sind Branchenverzeichnisse und damit Leads, keine Betreiber-/Register-/Behördenbestätigung; eine Telefonnummer wird deshalb nicht ins Frontmatter übernommen. Der bestehende Strich-Platzhalter im phone-Feld wurde zu leer normalisiert. Status `aktiv` → `pruefung`, da aktuelle Autoverwertungsaktivität nicht unabhängig belegt. ANKAUF — keine Fahrzeug-/Schrottpreise; VERKAUF — keine Teilepreisliste; GEBÜHREN — keine Tarife. Quellen: https://berlin.stadtbranchenbuch.com/A/5431.html ; https://www.business-branchenbuch.de/AutoDienst-Sd-Krmer-GmbH-6162255.html]
