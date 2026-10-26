---
slug: nw-hagen-ms-metallhandel
name: MS Metallhandel e.K.
trader_type: metallhaendler
state: NW
city: Hagen
street: Werkzeugstr. 28
postcode: '58093'
phone: 02331 3620236
email: ''
opening_hours: 'Mo-Sa 06:00-20:00'
website: https://www.ms-metallhandel-iserlohn.de/schrotthandel
website_status: ''
status: aktiv
clear_fields: 'dropoff_json,geo'
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Dortmund / Hagen / Ennepetal
provenance_ankauf_raw: ja (Gewerbe)
provenance_origin: table
---

# MS Metallhandel e.K.

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Owner-Audit 26.10.2026

- [Owner-Audit 26.10.2026: Der aktuelle Registerabdruck des AG Hagen, HRA 6192 (Abruf 26.10.2026, 10:41; letzter Registereintrag 22.03.2019), bestätigt „MS Metallhandel e.K. Inh. Martina Sinnen“, Geschäftsanschrift Werkzeugstraße 28, 58093 Hagen und Inhaberin Martina Sinnen. Das aktuelle Betreiber-Impressum stimmt damit überein; die aktuelle Schrotthandelsseite beschreibt den Ankauf ausschließlich für Industrie/Gewerbe, eigene Abholung und eine öffentliche LKW-Waage. Die Adresse ist damit stark bestätigt, aber kein Kundeneinfahrtspunkt: `clear_fields: 'geo'` bleibt angemessen, ohne Ersatzkoordinate. Die Website sagt nicht ausdrücklich, dass Kunden Schrott selbst anliefern dürfen; `dropoff_json.allowed=true` bleibt dafür ein offener, separat zu prüfender Claim (die LKW-Waage allein belegt keine Anlieferannahme). Keine Produktionsdaten abgefragt oder geändert. Quellen: https://www.handelsregister.de/rp_web/erweitertesuche/welcome.xhtml (AG Hagen, HRA 6192; aktueller Abdruck); https://www.ms-metallhandel-iserlohn.de/impressum/ ; https://www.ms-metallhandel-iserlohn.de/schrotthandel ; https://www.ms-metallhandel-iserlohn.de/service/ (Abruf 26.10.2026).]

### Owner-Korrektur 26.10.2026: nicht belegte Kundenanlieferung zurücknehmen

- [Owner-Korrektur 26.10.2026: Read-only `public.db` enthält für diesen Slug weiterhin `dropoff_json.allowed=true` für Gewerbekunden. Der direkte Recheck der aktuellen Betreiberseite belegt ausdrücklich B2B-Ankauf, eigene Abholung und eine öffentliche LKW-Waage, aber keine Selbstanlieferung von Kunden am Standort. Die Waage allein ist kein Annahmebeleg. `dropoff_json` bleibt deshalb leer und `clear_fields: 'dropoff_json,geo'` setzt den gespeicherten Claim beim nächsten regulären Seed auf unbekannt `{}` zurück; das behauptet weder `allowed:false` noch eine Schließung. Der falsche Hagen-GEO-Punkt wird weiterhin ohne Ersatz gelöscht. Keine Produktionsdaten geändert. Quellen: Read-only `public.db` (26.10.2026); https://www.ms-metallhandel-iserlohn.de/schrotthandel ; https://www.ms-metallhandel-iserlohn.de/service/ (Abruf 26.10.2026); `docs/trader-seed.md`.]

### Korrektur 24.10.2026: Gemeinsamen falschen Hagen-Punkt zurückziehen

- [Korrektur 24.10.2026: Die Produktionszeile `nw-hagen-ms-metallhandel` enthielt den mit mehreren anderen Hagen-Datensätzen geteilten Punkt `51.90933, 7.88825`. Die read-only-Rückwärtssuche löst ihn in Mehringen, Everswinkel, Kreis Warendorf, PLZ 48351 auf, nicht in Hagen. Die Seed-Adresse Werkzeugstr. 28, 58093 belegt diesen gespeicherten Punkt nicht; `clear_fields: 'geo'` zieht ihn zurück. Mangels bestätigtem Kunden-/Einfahrtspunkt wird keine Ersatzkoordinate gesetzt. Die Produktionsdatenbank blieb unverändert. Quelle: https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; https://www.openstreetmap.org/way/81516036.]

### Importiert (Seed-Stand 2026-09-30)

- NE-Metalle/Edelstahl, LME-Kurse, LKW-Waage (Werkzeugstr. 28)

### Recherche 01.10.2026

- [Feedback-Triage 01.10.2026 (ID 2449): KORREKTUREN mit offener Beleglage — postcode '58093' (Impressum + mehrfach site-intern: Werkzeugstr. 28, 58093 Hagen), phone Lager 02331 3620236 (Büro 02371 8324966), hours 'Mo-Sa 06:00-20:00' (Vorteile-Liste, eine Seite). Impressum: MS Metallhandel e.K., Inhaberin Martina Sinnen, HRA 6192 AG Hagen (registerverankert, aber HRA/e.K. → formal keine Owner-Ausnahme; starke Betreiberquelle + Registerverweis). Koordinaten-Hinweis berechtigt: Domain 'iserlohn' irreführend — einziger belegter Standort ist Hagen (kein Iserlohn-Standort auf der Website) → Timeline-Vermerk 'Koordinaten neu geocodieren (Werkzeugstr. 28, 58093 Hagen)'. B2B-only ('für Industrie und Gewerbe') → dropoff_json gewerbe. E-Mail nur Cloudflare-cloaked → kein Fill. Quelle(n): https://www.ms-metallhandel-iserlohn.de/schrotthandel, https://www.ms-metallhandel-iserlohn.de/impressum/ (Abruf 01.10.2026).]
