---
slug: th-bad-salzungen-fallenstein-rolf-schrotthandel
name: Fallenstein Rolf Schrotthandel
trader_type: schrotthaendler
state: TH
city: Bad Salzungen
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
provenance_section: Schrottankauf Thüringen (TH) — Recherche
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# Fallenstein Rolf Schrotthandel

## Überblick

Mehrere Verzeichnis-Leads wiederholen Schulweg 22, 36433 Moorgrund-Witzelroda und Telefon 03695 84624, aber weder Betreiberwebsite noch Register- oder Gewerbebeleg bestätigen Identität und aktuelle Anschrift. Diese nicht unabhängig belegten Werte werden deshalb nicht im Frontmatter geführt; „Bad Salzungen“ bleibt als übergeordneter Ortswert. Ankauf, Verkauf und Gebühren sind nicht belegt. Status `pruefung`; kein Geocoding als bestätigter Annahmepunkt.

## Timeline

### Owner-GEO-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Read-only-Abgleich von `public.db` zeigt weiterhin den Punkt 50.8134, 10.2361. Die frische Nominatim-Reverse-Abfrage löst ihn auf ein Wohngebäude, Michaelisstraße 2, Wildprechtroda auf; ein Betreiberbezug ist nicht belegt. Der Produktions-Straßenwert Schulweg 22 stammt nur aus Verzeichnis-Leads; das Dossier führt Straße und PLZ bereits leer. Die geänderte Seed-Direktive `clear_fields: 'geo'` veranlasst beim nächsten regulären Seed die Rücknahme des falschen GEO-Punkts und aufgrund der leeren Kernadresse auch das Entfernen des alten Straßenwerts. Kein Ersatzpunkt abgeleitet; keine Aussage über Schließung. Quellen: https://nominatim.openstreetmap.org/reverse?lat=50.8134&lon=10.2361&format=jsonv2&zoom=18&addressdetails=1&extratags=1 ; https://www.openstreetmap.org/way/217747859 ; [Owner-GEO-Bericht](../../research/wave-20261021-0900-geo-th.md).]

### Owner-GEO-Prüfung 21.10.2026

- [Owner-GEO-Prüfung 21.10.2026: Der Produktionspunkt löst auf Michaelisstraße 2 in Wildprechtroda auf. Schulweg 22 in Moorgrund bleibt ein Verzeichnis-Lead ohne Betreiber- oder amtliche Standortbestätigung; OSM bildet dort nur ein Gebäude ab. Keine belastbare Zuordnung zu Fallenstein oder einer Annahmestelle, daher kein GEO-Kandidat. Quellen: [Verzeichnis-Lead](https://www.schrottplatz-info.de/schrottplatz/Bad-Salzungen/Fallenstein-Rolf-Schrotthandel), [Reverse-Abfrage](https://nominatim.openstreetmap.org/reverse?lat=50.8134&lon=10.2361&format=jsonv2&zoom=18&addressdetails=1&extratags=1), [Owner-Bericht](../../research/wave-20261021-0900-geo-th.md).]

### Unabhängige Nachprüfung 21.10.2026

- [Recherche 21.10.2026: Mehrere Verzeichnis-Leads stimmen auf Schulweg 22, 36433 Moorgrund (Ortsteil Witzelroda), 03695 84624 überein; darunter Schrottplatz-Info und 11880. PLZ und Telefon wurden als nicht primär verifizierte Kontakt-Leads ergänzt, nicht als Betreiberbestätigung. Der Datensatz erfasst keine konkreten Leistungen/Öffnungszeiten und keine Preise. Ankauf, Verkauf und Gebühren ungeklärt; keine Geokodierung als bestätigter Recyclinghof, bis Betreiber und aktuelle Anschrift geprüft sind. Quellen: https://www.schrottplatz-info.de/schrottplatz/Bad-Salzungen/Fallenstein-Rolf-Schrotthandel ; https://www.11880.com/suche/schrotthandel/wutha-farnroda ; https://www.schrottplatz.org/bad-salzungen/schrott-fallenstein-rolf-schrotthandel-aYEPSi.html]

### Owner-Korrektur 21.10.2026

- [Korrektur 21.10.2026: Owner-Gate: Die genannten Belege sind ausschließlich Branchen-/Schrottplatz-Verzeichnisse; nach der Quellenhierarchie in `README.md` sind sie Leads, keine Feldbelege. Die zuvor ergänzte PLZ und Rufnummer wurden deshalb wieder entfernt; auch die bereits vorhandene, nur durch diese Lead-Spur gestützte Straße wurde aus dem Frontmatter zurückgenommen. Die konkrete Anschrift bleibt im Rechercheverlauf nachvollziehbar, aber ungeprüft. Kein Betreiber-/Registerbeleg, keine bestätigte Annahme und kein Geocoding. Quellen/Lead-Spuren: https://www.schrottplatz-info.de/schrottplatz/Bad-Salzungen/Fallenstein-Rolf-Schrotthandel ; https://www.11880.com/suche/schrotthandel/wutha-farnroda ; https://www.schrottplatz.org/bad-salzungen/schrott-fallenstein-rolf-schrotthandel-aYEPSi.html]

### Importiert (Seed-Stand 2026-09-30)

- Schrotthandel, ländlich (Wartburgkreis/Rhön-nah)
