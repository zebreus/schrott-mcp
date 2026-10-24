---
slug: nw-hagen-zimahmo
name: ZIMAHMO
trader_type: mobil
state: NW
city: Hagen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
clear_fields: 'geo'
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# ZIMAHMO

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Korrektur 24.10.2026: Gemeinsamen falschen Hagen-Punkt zurückziehen

- [Korrektur 24.10.2026: Die Produktionszeile `nw-hagen-zimahmo` enthielt den mit mehreren anderen Hagen-Datensätzen geteilten Punkt `51.90933, 7.88825`. Die read-only-Rückwärtssuche löst ihn in Mehringen, Everswinkel, Kreis Warendorf, PLZ 48351 auf, nicht in Hagen. `clear_fields: 'geo'` zieht den falschen Punkt zurück; mangels belegter Händleradresse wird keine Ersatzkoordinate gesetzt. Die Produktionsdatenbank blieb unverändert. Quelle: https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; https://www.openstreetmap.org/way/81516036.]

### Importiert (Seed-Stand 2026-09-30)

- mobil (GS)
- Adresse: Hagen

### Recherche 03.10.2026

- [Recherche 03.10.2026: Kein eindeutig zuordenbarer Betreiberauftritt, Impressum, Register- oder Kommunalbeleg zu „ZIMAHMO“ in Hagen verifiziert; die Websuche lieferte keine belastbare Identitäts-/Standortquelle. NorthData-Namenssuche zeigte nur unscharfe, unpassende „Mahmoud“-Treffer ohne Hagen-Bezug (keine negative Registerauskunft). Deshalb keine Frontmatter-Fills, keine Telefonnummer, Adresse oder Koordinaten geraten; status bleibt pruefung, ohne Schließungsbehauptung. Quelle(n): keine belegfähige Quelle; Suchprotokolle (kein Sachbeleg): https://www.bing.com/search?q=%22ZIMAHMO%22+Hagen; https://www.northdata.de/?query=ZIMAHMO]
