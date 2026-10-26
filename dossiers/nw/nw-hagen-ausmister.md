---
slug: nw-hagen-ausmister
name: Ausmister
trader_type: sonstige
state: NW
city: Hagen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://dieausmister.de/hagen
website_status: aktiv
status: pruefung
clear_fields: 'email,geo,opening_hours,phone'
description: 'Die Betreiberwebsite bewirbt Entrümpelung und Haushaltsauflösung einschließlich Entsorgung zum Festpreis nach Besichtigung; rechtliche Betreiberidentität und genaue Kontakt-/Standortangaben sind nicht unabhängig bestätigt. Keine allgemeine Schrottannahme belegt.'
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Ausmister

## Überblick

Die Betreiberseite `dieausmister.de/hagen` nennt „Die Ausmister“, Wehringhauser Str. 60, 58089 Hagen, 02331 9109428 und info@dieausmister.de. Das Facebook-Profil „Die Ausmister“ bestätigt den Namen, Hagen und Entrümpelungsangebote, nennt aber keine Straße oder Telefonnummer. Die Website bietet Entrümpelung, Haushaltsauflösung und Entsorgung zum Festpreis nach Besichtigung. Ihre Beratungs-/Besichtigungszeit Mo–So 08:00–22:00 ist keine belegte Öffnungszeit eines Schrottplatzes. Die rechtliche Betreiberidentität bleibt offen; der Status `pruefung` und die fehlende allgemeine Schrottannahme bleiben bestehen.

- **Ankauf:** kein allgemeiner Metallankauf und kein Schrottpreis belegt.
- **Verkauf:** kein Warenangebot oder Verkaufspreisblatt gefunden.
- **Gebühren:** die Betreiberseite zeigt Beispielpreise für Räumungsaufträge (u.a. Kellerraum 250 €, kleine Wohnung 899 €, Wohnung 60 m² 1.200 €, Haus ca. 85 m² 1.700 €); sie bezeichnet die Beträge als grobe Schätzung anhand beispielhafter Inhalte, nicht als Metallankauf, allgemeine Entsorgungsgebühr oder verbindliches Angebot. Beratung und Vor-Ort-Besichtigung werden als kostenlos beworben.
- **GEO:** keine Koordinaten ergänzt; die Kontaktadresse allein belegt keinen öffentlichen Annahmehof.

## Timeline

### Owner-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Die Betreiberwebsite nennt Wehringhauser Str. 60, 58089 Hagen, Telefon, E-Mail und Beratungs-/Besichtigungszeiten Mo-So 08:00-22:00. Das Facebook-Profil bestätigt nur Name, Hagen und Entrümpelungsangebot, nicht diese konkreten Felder; die Seite erfüllt auch nicht die README-Ausnahme für eine registerkongruente Betreiberquelle. Straße, PLZ, Telefon, E-Mail und Beratungszeit werden daher nicht als verifizierte Händler-Stammdaten ausgespielt; Telefon, E-Mail und Zeiten explizit zurückgenommen. Die Website bleibt als Recherche-Lead erreichbar, `status: pruefung` bleibt. Der alte Punkt ist separat als falsch belegt und wird mit `clear_fields: 'geo'` entfernt; kein Ersatzpunkt. Quellen: https://dieausmister.de/hagen ; https://www.facebook.com/DieAusmister/ ; https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; README.md, Quellenhierarchie.]

### Recherche 24.10.2026: Betreiberseite direkt geprüft

- [Recherche 24.10.2026: Betreiberseite live direkt gelesen. Sie nennt Die Ausmister, Wehringhauser Strasse 60, 58089 Hagen, 02331-9109428, info@dieausmister.de und Beratungs-/Besichtigungszeit Mo–So 08:00–22:00. Beschrieben werden Entrümpelung inklusive Entsorgung zum Festpreis, Abbruchservice und Renovierung; eine allgemeine Schrottannahme oder ein Metall-Ankaufspreis wird nicht angeboten. Die Seite zeigt nur grobe Beispielpreise für konkrete Räumungen (250/899/1.200/1.700 €) sowie kostenlose Beratung/Besichtigung; diese Beträge sind keine Gebühren für bloße Schrottabgabe. Adresse/Kontakt/Website/Öffnungszeit in Frontmatter ergänzt, `status: pruefung` bleibt mangels im Seitenabruf sichtbarer Betreiber-/Registeridentifikation. Keine Drop-off-/Pickup-Zusage und keine Koordinaten ergänzt. Quelle: https://dieausmister.de/hagen .]

### Korrektur 24.10.2026: Identität und falscher Produktionspunkt

- [Korrektur 24.10.2026: Das direkte Facebook-Profil „Die Ausmister“ beschreibt Entrümpelungsangebote in Hagen und stimmt beim Namen mit der Betreiberseite überein; es veröffentlicht keine Straße oder Telefonnummer. Der Website-Menüpunkt „Impressum“ führt zurück auf `/hagen`; eine rechtliche Betreiberangabe war dort nicht sichtbar. Die Kontaktfelder bleiben deshalb als Betreiberangaben markiert, `status: pruefung` bleibt, und es wird keine allgemeine Schrottannahme behauptet. Read-only-Prüfung der Produktionszeile ergab den Punkt `51.90933, 7.88825`; Nominatim verortet ihn in Mehringen, Everswinkel, Kreis Warendorf, PLZ 48351, nicht in Hagen. Eine Abfrage der Website-Adresse Wehringhauser Straße 60 liefert ein Gebäude namens „Antik Eck“, keine bestätigte Annahmestelle der Ausmister. Daher `clear_fields: 'geo'`: den falschen Punkt beim nächsten regulären Seed entfernen und keine Ersatzkoordinate setzen. Produktionsdatenbank blieb unverändert; Quelle: https://www.facebook.com/DieAusmister/ ; https://dieausmister.de/hagen ; https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Wehringhauser%20Strasse%2060%2C%2058089%20Hagen&format=jsonv2&addressdetails=1.]

### Importiert (Seed-Stand 2026-09-30)

- Entrümpler (GS)
- Adresse: Hagen
