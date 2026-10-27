---
slug: nw-essen-schrott-busters
clear_fields: geo
name: Schrott Busters
trader_type: schrotthaendler
state: NW
city: Essen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Neu, Ankauf unklar (nur Verzeichnis-/Profilbeleg)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Schrott Busters

## Überblick

Betreiberwebsite identifiziert sich als Schrott Busters Oliver Wojke, Fischweiher 26, 45307 Essen. Mobile Schrottabholung und Metallankauf für Privat- und Industriekunden werden ausdrücklich angeboten; keine öffentliche Hofannahme aus der Kontaktadresse ableiten. Die Website ist ein Einzelbeleg ohne unabhängige Betreiberbestätigung. Ankauf ohne Zahlenliste; Verkauf von Elektronikschrott erwähnt, ebenfalls ohne Liste. Kostenfreie Schrott-/Buntmetallabholung ist eine konkrete Selbstangabe, keine allgemeine Gebührenfreiheit für Demontage oder Entrümpelung. Status `pruefung`, Stammdaten vorerst unverändert.

## Timeline

### Owner-Korrektur 27.10.2026: falsches GEO zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`; der Reverse-Lookup führt auf die Fernbus-Spur „Freiheit“ am Essener Hauptbahnhof (OSM way 46961889, `vehicle=no`), also nicht auf einen Betriebsort. Die dokumentierte Geschäftsanschrift Fischweiher 26, 45307 Essen (Betreiber-Impressum mit Steuernummer 111/5329/3605 plus 11880-Eintrag vom 15.10.2026 mit identischer Telefonnummer) liegt rund 4,4 km entfernt und ist ein Wohn-/Mischobjekt; eine öffentliche Hofannahme ist nicht belegt, die Betreiberseite beschreibt mobile Abholung. Das gespeicherte Paar ist damit für genau diese Zeile falsch und wird mit `clear_fields: geo` zurückgenommen, ohne Ersatzkoordinate und ohne Annahmestellen-Behauptung; `status: pruefung` bleibt. Offen bleiben Register- oder Behördenbeleg, Abholgrenzen und Vergütung. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://schrott-busters.de/kontakt/impressum/ ; https://www.11880.com/branchenbuch/essen-ruhr/060440092B112411338/schrott-busters-oliver-wojke.html ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 14.10.2026 (Welle 5 A)

- [Recherche 14.10.2026: Startseite, Impressum und Ankauf-Unterseite direkt gelesen (HTTP 200). Impressum nennt Oliver Wojke, Fischweiher 26, D – 45307 Essen, kontakt@schrott-busters.de, 0177 6105096; das D ist Länderpräfix, nicht belegter Hausnummerzusatz. Die Marken-/Adresskongruenz zum Seed ist stark, aber Website-Unterseiten sind derselbe Quellenursprung; kein HRB im Impressum und keine zweite zulässige unabhängige Quelle verifiziert. Daher keine Frontmatter-Fills. Startseite nennt Metallankauf von Privat und Industrie, kostenfreie Schrott-/Buntmetallabholung und Ankauf/Verkauf von Elektronikschrott. Ankaufseite datiert 26.03.2024; keine numerischen Ankaufskurse, Verkaufswerte oder Gebühren für sonstige Leistungen auf den gelesenen Seiten. Genehmigungen nur pauschale Selbstwerbung, kein gelesenes Zertifikat. Offen: unabhängige Inhaber-/Gewerbebestätigung, aktuelle Abholgrenzen und Vergütung, keine Hofannahme unterstellen; Quelle(n): https://schrott-busters.de/ ; https://schrott-busters.de/kontakt/impressum/ ; https://schrott-busters.de/schrottankauf/]

### Importiert (Seed-Stand 2026-09-30)

- Fischweiher 26, nur schrottradar-Profil
- urspr. Website-Angabe: keine gefunden
