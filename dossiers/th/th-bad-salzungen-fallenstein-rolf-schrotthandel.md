---
slug: th-bad-salzungen-fallenstein-rolf-schrotthandel
lat: 50.8254135
lon: 10.2881622
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
clear_fields: ''
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

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **50.8254135/10.2881622** (Mittel der 7 eindeutigen Knoten von
  way/1050790878; der Punkt liegt innerhalb der Objekt-BBox
  50.8253734/10.2880570–50.8254835/10.2883087). Quelle 1 (OSM, live geladen am 04.11.2026):
  **way/1050790878** trägt `addr:street`=Schulweg, `addr:housenumber`=**22**,
  `addr:postcode`=36433, `addr:city`=Bad Salzungen, `addr:country`=DE und `building`=yes - Straße,
  Hausnummer, PLZ und Ort stimmen mit der im Dossier dokumentierten Adresse **Schulweg 22, 36433
  Moorgrund-Witzelroda** überein (PLZ und Ort trägt das Objekt als Bad Salzungen, der Ortsteil
  Moorgrund/Witzelroda ist die Bezeichnung des Verzeichnis-Leads). Das Objekt trägt **weder `name=`
  noch `operator=`**; es ist eine **Adressangabe** eines Gebäudes, kein Betriebs-POI. Quelle 2 (im
  Dossier bereits zitiert, live nachgelesen am 04.11.2026): das Schrottplatz-Verzeichnis
  https://www.schrottplatz-info.de/schrottplatz/Bad-Salzungen/Fallenstein-Rolf-Schrotthandel liefert
  HTTP 200 mit **„Schrottplatz Fallenstein Rolf Schrotthandel, Adresse: Schulweg 22, 36433 Moorgrund
  Witzelroda“** und der Kartenposition **50.8254555/10.2879650** - die **rund 15 m** vom gesetzten
  OSM-Punkt entfernt liegt und die Anschrift damit unabhängig bestätigt. **Fensterprüfung (kein
  Fensterfehler):** das am 04.11.2026 live abgerufene Kachelraster umfasst **10.2800/50.8200 bis
  10.2900/50.8300**, **1 Kachel von je 0,01 Grad**, **0 Kachel-Fehler**, **2.711 Elemente**; das
  Fenster enthält die gesuchte Straße **„Schulweg“ nachweislich** mit den beiden Straßenobjekten
  **way/110996550** und **way/110996558** (`highway`=living_street) sowie **12 Adressobjekten** auf
  „Schulweg“ mit **12 verschiedenen Hausnummern** (1, 2, 3, 8, 10, 12, 14, 16, 18, 20, **22**, 10a) - die
  **Dossier-Hausnummer 22 existiert** und ist im Raster das einzige Objekt mit dieser Nummer auf
  dieser Straße. **Negativaussage, qualifiziert:** im Raster tragen **55 Objekte** `name=` oder
  `operator=`; **mit Bezug zu „Fallenstein“, „Schrott“ oder „Recycling“** trifft es auf **kein**
  Objekt dieser Adresse - der Betrieb ist in OSM **nicht** kartiert; der Punkt bezeichnet
  ausschließlich die verzeichnisgestützte Anschrift. **Zurücknahme des `geo`-Tokens (Begründung):**
  `clear_fields: 'geo'` wurde **entfernt** (das Feld steht jetzt als `clear_fields: ''`), weil es den
  am 26.10.2026 zurückgenommenen und nachweislich falschen Produktionspunkt (50.8134/10.2361 =
  Wohngebäude Michaelisstraße 2, Wildprechtroda) fernhalten sollte; der hier gesetzte Punkt liegt
  **exakt** auf der verzeichnisbestätigten Anschrift, und `clear_fields` darf nicht gleichzeitig
  `geo` neben belegtem `lat`/`lon` stehen. **Qualifizierung (unverändert übernommen):** Der Punkt
  bezeichnet die **Adresse eines Verzeichnis-Leads**, **nicht** einen verifizierten Annahmehof,
  eine Waage oder eine öffentliche Schrottannahme; das im Dossier offene Problem der fehlenden
  Betreiber-/Registerquelle und der abweichenden Ortsangaben (Bad Salzungen gegen Moorgrund-
  Witzelroda) bleibt unverändert, ebenso `status: pruefung`, `city` (weiterhin der übergeordnete
  Ortswert „Bad Salzungen“), `street`, `postcode`, `phone`, `email`, `opening_hours`, `website`,
  `website_status` und `description`. Es wird kein Ankaufkurs, keine Preisliste und keine Gebühr
  behauptet. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/1050790878/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=10.2800,50.8200,10.2900,50.8300 ;
  https://www.schrottplatz-info.de/schrottplatz/Bad-Salzungen/Fallenstein-Rolf-Schrotthandel
  (Abruf 04.11.2026).]
