---
slug: nw-duisburg-tsr-deutschland-ndl-duisburg
name: TSR Deutschland GmbH & Co. KG (Ndl. Duisburg)
trader_type: schrotthaendler
state: NW
city: Duisburg
street: Rohstoffinsel 2-10
postcode: "47138"
phone: 02034 5007-0
email: ""
opening_hours: ""
website: https://www.tsr-recycling.de/standorte/schrotthaendler-duisburg/
website_status: blockiert
status: aktiv
description: ""
dropoff_json: '{"allowed": true, "customer_types": ["gewerbe"], "days": ["Mo", "Di", "Mi", "Do", "Fr"], "conditions": "Keine Annahme von Privatkunden; Anlaufstelle fuer Gewerbe-/Industriekunden (TSR-Standortseite, 30.09.2026)"}'
pickup_json: ""
provenance_section: Ruhrgebiet – Großrecycler / Stahlwerksnah (Duisburg, Dortmund,
  Gelsenkirchen, Lünen, Essen)
provenance_ankauf_raw: ja (nur Gewerbe, keine Privatkunden)
provenance_origin: table
---

# TSR Deutschland GmbH & Co. KG (Ndl. Duisburg)

## Überblick

Großrecycler auf der Rohstoffinsel Duisburg (Rohstoffinsel 2-10, 47138 Duisburg);
Stahl-/NE-Schrott mit Shredder und TSR40-Anlage. Nur Gewerbe, keine Privatkunden.
Zentrale: 02306 1063800 (Lünen).

## Timeline

### Importiert (Seed-Stand 30.09.2026)

- Stahl-/NE-Schrott, Shredder, TSR40-Anlage (Rohstoffinsel 2-10) [Quelle: seed/nw.json,
  Section "Ruhrgebiet – Großrecycler / Stahlwerksnah"]
- Adressbeleg: tsr-recycling.de/standorte + Bibliothek-Zertifikat [Quelle: seed-Notes]

### Korrektur 30.09.2026 (Feedback-Triage)

- [Korrektur Feedback fb1845: Telefon war Zentrale Lünen (02306 1063800 = TSR-HQ Brunnenstr. 138, bestätigt via wer-zu-wem.de + TSR-Impressum) → Standort-Nummer 02034 5007-0 (TSR-Standortseite via Jina-Reader: T +49 2034 5007-0, F ...5007-68); Verzeichnis-Angabe „(0203) 45 00 70" (golocal, lokaleschrottplatz.de) weicht ab und ist gegen die Primärquelle unterlegen; Quelle: tsr-recycling.de/standorte/schrotthaendler-duisburg]
- [Korrektur Feedback fb1845: keine Privatannahme mit Website-Beleg — TSR-Seite: „Anmerkung: Keine Annahme von Privatkunden", „Anlaufstelle für Gewerbe- und Industriekunden" → dropoff_json (gewerbe only); Quelle: TSR-Standortseite (30.09.2026)]
- [Korrektur Feedback fb1845: website_status blockiert (Human-Challenge auf tsr-recycling.de, verifiziert 30.09.2026); Quelle: Direktabruf]
- (Klärfall): Öffnungszeiten widersprüchlich — TSR-Seite Mo–Fr 06:00–17:30 vs. lokaleschrottplatz.de Mo–Fr 06:00–17:00 vs. schrottplaetze.org Mo–Do 06:00–16:30/Fr 15:00 (dort zudem alte Straßenschreibung „Schrottinsel") → opening_hours leer gelassen.
