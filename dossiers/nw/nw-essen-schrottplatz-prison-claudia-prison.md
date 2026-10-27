---
slug: nw-essen-schrottplatz-prison-claudia-prison
clear_fields: 'geo'
name: Schrottplatz Prison (Claudia Prison)
trader_type: schrottplatz
state: NW
city: Essen
street: Hövelstr. 254
postcode: '45356'
phone: 0201 660081
email: buero@schrottplatz-essen.de
opening_hours: 'Mo-Fr 08:00-17:00, Sa 08:00-14:00'
website: https://schrottankauf-in.de/
website_status: aktiv
status: aktiv
description: ''
dropoff_json: '{"allowed": true, "conditions": "Anlieferung/Containerstellung/Entsorgung am Standort Hoeverlstr. 254 Essen; Preisauskunft im Essener Buero (Betreiber-Kontaktseite, Feedback-Triage 01.10.2026)"}'
pickup_json: '{"allowed": true, "conditions": "Schrottabholung in Essen und ganzem Ruhrgebiet (Betreiber-Startseite, Feedback-Triage 01.10.2026)"}'
provenance_section: Essen / Mülheim / Oberhausen / Bottrop
provenance_ankauf_raw: ja
provenance_origin: table
---

# Schrottplatz Prison (Claudia Prison)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Owner-GEO-Korrektur 27.10.2026

- [Owner-GEO-Korrektur 27.10.2026: Die ausschließlich lesend abgefragte Produktionszeile enthält `51.450284375, 7.015053125`; Nominatim-Reverse ordnet den Punkt der Freiheit im Sternviertel/Südviertel (45128 Essen) zu. Betreiber-Impressum und Kontaktseite nennen dagegen den Schrott- und Metallgroßhandel Prison-Kramer, Hövelstr. 254, 45356 Essen; der Betreiberauftritt beschreibt den Standort und Anlieferung. Nominatim adressiert Hövelstraße 254 separat in Essen-Bergeborbeck. Der Produktionspunkt passt daher nicht zum belegten Standort. `clear_fields: 'geo'` entzieht ihn beim nächsten regulären Seed; keine Ersatzkoordinate, da kein genauer Eingangspunkt geprüft wurde. Quellen: https://schrottankauf-in.de/impressum.php ; https://schrottankauf-in.de/kontakt-infos-schrottankauf-zu-spitzen-preisen-in-nrw.html.php ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=H%C3%B6velstrasse%20254%2C%2045356%20Essen%2C%20Germany ; read-only `/var/lib/schrott-mcp/public.db` (27.10.2026); `docs/trader-seed.md`.]

### Recherche 01.10.2026

- [Feedback-Triage 01.10.2026 (ID 2452): KORREKTUREN mit offener Beleglage — postcode '45356', phone 0201 660081, email buero@schrottplatz-essen.de, hours 'Mo-Fr 08:00-17:00, Sa 08:00-14:00' von Betreiber-Kontaktseite + Impressum + Startseite uebernommen (eine Website = formal EIN Beleg; Impressum nennt Einzelunternehmen Claudia Prison-Kramer ohne HRB → keine Owner-Ausnahme). Zweitbelegung via kongruente Leads: YellowMap/golocal/city-map/schrottplatz-info bestaetigen Adresse 45356 + Telefon + E-Mail. Anlieferung + Abholung belegt ('Containerstellung, Entsorgung, Anlieferung', 'Schrottabholung in Essen und im ganzen Ruhrgebiet', 'Oder besuchen Sie uns') → dropoff/pickup_json gesetzt (ohne Kundentyp, privat/gewerbe nicht explizit belegt). Alternativ-Domain www.schrottplatz-essen.de in Leads genannt, nicht verifiziert. Quelle(n): https://schrottankauf-in.de/, https://schrottankauf-in.de/kontakt-infos-schrottankauf-zu-spitzen-preisen-in-nrw.html.php, https://schrottankauf-in.de/impressum.php (Abruf 01.10.2026).]

### Importiert (Seed-Stand 2026-09-30)

- Kabelschrott, Katalysatoren, NE-Metalle (Hövelstr. 254)
