---
slug: nw-essen-schrottplatz-prison-claudia-prison
lat: 51.4785696
lon: 6.9835565
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




### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **51.4785696/6.9835565** (Mittel der 4 eindeutigen
  Way-Knoten von way/49219240; der Punkt liegt innerhalb der Objekt-BBox **51.4784998/6.9834696 – 51.4786395/6.9836433**).
  Quelle 1 (OSM): way/49219240 trägt `addr:housenumber`=**254**, `addr:street`=**Hövelstraße**, `addr:city`=Essen,
  `addr:country`=DE, `addr:postcode`=**45356**, `building`=yes — Dossier-Straße, -Hausnummer und -PLZ stimmen
  objektgenau überein. Der Betriebsbezug ist im OSM zusätzlich am selben Standort belegt: node/5612444503
  (`name`=**Schrotthandel Prison**, `industrial`=**scrap_yard**, `landuse`=industrial) liegt **31 m** entfernt bei
  **51.4787665/6.9832361** — Betrieb und Anschrift sind damit auf demselben Grundstück verortet. Quelle 2
  (Betreiber/Register): das Impressum der Betreiber-Website nennt **Schrott- und Metallgroßhandel Prison-Kramer,
  Hövelstr. 254, 45356 Essen** (Einzelunternehmen Claudia Prison-Kramer, ohne HRB), die Kontaktseite bestätigt
  Anschrift, Telefon **0201 660081** und E-Mail **buero@schrottplatz-essen.de** sowie Anlieferung, Containerstellung
  und Entsorgung am Standort; die kongruenten Verzeichnisleads (YellowMap/golocal/city-map/schrottplatz-info) nennen
  dieselbe Anschrift. Der zuvor im Produktionsbestand gespeicherte Punkt **51.450284375/7.015053125** (Straße „Freiheit“,
  45128 Essen) lag **3828 m** entfernt und war für diese Zeile falsch.
  **Hinweis zum Frontmatter:** der Clear-Token `geo` wurde aus `clear_fields` entfernt, weil die Zeile jetzt eine
  kuratierte Koordinate trägt — ein gleichzeitig gesetztes `clear_fields: 'geo'` wird vom Seed-Validator als
  Konflikt abgewiesen (`clear_fields conflicts with populated geo`).
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/49219240/full.json ; https://api.openstreetmap.org/api/0.6/node/5612444503.json ;
  https://schrottankauf-in.de/impressum.php ; https://schrottankauf-in.de/kontakt-infos-schrottankauf-zu-spitzen-preisen-in-nrw.html.php
  (Abruf 03.11.2026).]
