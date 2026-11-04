---
slug: st-allstedt-wolferstedt-schneider-marko
clear_fields: 'geo'
name: Schneider Marko
trader_type: schrottplatz
state: ST
city: Allstedt / Wolferstedt
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
provenance_section: Halle / Saalekreis / Burgenlandkreis / Merseburg / Weißenfels
  / Naumburg / Zeitz
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# Schneider Marko

## Überblick

Kandidat „Marko Schneider Schrottentsorgung“: Creditreform führt einen aktiven Gewerbebetrieb; Branchenverzeichnisse nennen Nienstedter Str. 1, 06542 Allstedt-Wolferstedt, Tel. 034652 245. Ein eigener Betreiberauftritt oder eine unabhängige aktuelle Standortbestätigung wurde nicht gefunden. Anschrift und Telefon bleiben deshalb Prüfhinweise, keine bestätigten Stammdaten. Keine belastbare Ankaufspreisliste, Verkaufspreisliste oder Gebührenordnung; vor Verifizierung nicht geocodieren.

## Timeline

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 51.4038, 11.3869. Nominatim-Reverse löst den Punkt an Gerstenstraße 14 in Klosternaundorf auf, nicht an der Lead-Adresse Nienstedter Str. 1 in Allstedt-Wolferstedt. `clear_fields: 'geo'` entzieht den falsch zugeordneten Altpunkt beim nächsten regulären Seed. Keine Ersatzkoordinate, da die Geschäfts-/Annahmestellenadresse nur Verzeichnislead ist. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.4038&lon=11.3869&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Quellenprüfung 24.10.2026

- [Recherche 24.10.2026: Das Örtliche-Detailseite direkt gelesen; sie führt Schneider Marko, Nienstedter Str. 1, 06542 Allstedt-Wolferstedt und 034652 245. Creditreform beschreibt einen aktiven Gewerbebetrieb „Marko Schneider Schrottentsorgung“, belegt aber nicht unabhängig die Verzeichnisadresse, öffentliche Annahme oder Kundenzugang. Die konvergierenden Angaben bleiben deshalb Recherchehinweise und keine neuen Frontmatter-Fills. ANKAUF: keine Betreiberbestätigung/Preise. VERKAUF: keine Liste. GEBÜHREN: keine Tarife. GEO: Nienstedter Str. 1 ist Verzeichnisadresse ohne bestätigten Hofstatus; geokodieren erst nach Betreiber-/Standortbestätigung. Quellen: https://www.dasoertliche.de/Themen/Schneider-Marko-Allstedt-Wolferstedt-Nienstedter-Str ; https://firmeneintrag.creditreform.de/06542/3130174407/MARKO_SCHNEIDER_SCHROTTENTSORGUNG ; https://www.schrottplatz-info.de/schrottplatz/Allstedt/Schneider-Marko .]

### Importiert (Seed-Stand 2026-09-30)

- Schrottplatz (lt. schrottplatz-info-Umkreis)

### Recherche 06.10.2026

- [Recherche 06.10.2026: Creditreform führt „Marko Schneider Schrottentsorgung“ als Gewerbebetrieb/wirtschaftlich aktiv; Verzeichnisse nennen Nienstedter Str. 1, 06542 Allstedt-Wolferstedt und Tel. 034652 245. Kein Betreiberbeleg für genau diese Anschrift/Kontaktdaten gefunden; daher keine Frontmatter-Fills und Status pruefung. Quelle(n): https://firmeneintrag.creditreform.de/06542/3130174407/MARKO_SCHNEIDER_SCHROTTENTSORGUNG ; https://www.schrottplatz-info.de/schrottplatz/Allstedt/Schneider-Marko ; https://www.dasoertliche.de/Themen/Schneider-Marko-Allstedt-Wolferstedt-Nienstedter-Str]
- [Recherche 06.10.2026: Preise und Geocoding: keine numerischen Ankauf-/Verkaufspreise oder Gebühren gefunden. Nienstedter Str. 1 ist ein Verzeichnislead und erst nach Bestätigung durch Betreiber/Owner zu geocodieren.]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Hausnummer **1** der Nienstedter Straße
  ist in OSM **nicht** erfasst; das abgetastete Fenster enthält die gesuchte Straße **nachweislich** selbst (der
  Fensterfehler der letzten Welle - ein Raster mehrere Kilometer vom Objekt entfernt - liegt hier **nicht** vor).
  **Quelle 1 (OSM, live abgerufen am 04.11.2026):** Kachelraster über **Allstedt-Wolferstedt**
  (**11.4100/51.4200 bis 11.4400/51.4400**, **6 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**, live über
  `https://api.openstreetmap.org/api/0.6/map.json?bbox=11.4100,51.4200,11.4400,51.4400`): die **Nienstedter Straße**
  ist darin mit **neun Straßenobjekten** enthalten (way/24998939, way/279415189, way/388707773, way/1213051612,
  way/1215975007, way/1510576595, way/1512454317, way/1510576598, way/1510576619) und führt **genau ein**
  Adressobjekt mit `addr:street`=Nienstedter Straße, nämlich **way/1301527957** mit `addr:housenumber`=**197**,
  `addr:city`=Allstedt, `addr:postcode`=06542, `building`=yes - die **Hausnummer 1 existiert im Raster nicht**. Ein
  vergrößertes Orientierungsraster über **11.3000/51.4000 bis 11.5000/51.5000** (ebenso live abgerufen) bestätigt
  denselben Befund für den **gesamten** Straßenverlauf der Nienstedter Straße zwischen Wolferstedt und Nienstedt
  (Straßenverlauf 51.4262440/11.4203485 bis 51.4287921/11.4322785): auch dort ist **197** die **einzige** belegte
  Hausnummer dieser Straße, und eine Namenssuche nach **„Schneider“**, **„Marko“** und **„Schrott“** in den Feldern
  name, operator, description, website, contact:website, brand, note und addr:housename findet **keinen** Treffer.
  Quelle 2 (Betreiber/Verzeichnis, im Dossier bereits zitiert): die im Dossier dokumentierten Verzeichnis-Links -
  **Das Örtliche** („Schneider Marko, Nienstedter Str. 1, 06542 Allstedt-Wolferstedt, Tel. 034652 245“), das
  **Creditreform-Firmenprofil** („MARKO SCHNEIDER SCHROTTENTSORGUNG“, wirtschaftlich aktiv) und **schrottplatz-info.de**
  („Schneider Marko“, Allstedt) - belegen die Anschrift und den aktiven Gewerbebetrieb, aber **nur** als
  Verzeichnis-/Registereintrag; ein **Betreiberauftritt** (Impressum, Website, Schrottplatz-Schild) existiert nicht.
  **Warum keine Koordinate:** OSM kennt an dieser Adresse **kein** Objekt; der einzige Ankerpunkt der Straße wäre
  way/1301527957 mit der **falschen** Hausnummer 197 und wäre ein geratener Punkt; der zurückgenommene Produktionspunkt
  **51.4038/11.3869** (Gerstenstraße 14, Klosternaundorf) bleibt mit `clear_fields: 'geo'` zurückgenommen. **Was zur
  Auflösung fehlt:** ein OSM-Objekt mit `addr:housenumber`=**1** und `addr:street`=Nienstedter Straße (oder ein
  benannter Betriebspunkt „Marko Schneider Schrottentsorgung“) - vor allem aber ein **lesbarer Betreiber-/Registerbeleg
  mit vollständiger Anschrift**, der bestätigt, dass die Verzeichnisadresse tatsächlich die Betriebs- und Annahmestelle
  ist; `street`, `postcode`, `phone`, `website`, `website_status`, `description` und `status: pruefung` bleiben
  unverändert. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=11.4100,51.4200,11.4400,51.4400 ;
  https://api.openstreetmap.org/api/0.6/way/1301527957/full.json ;
  https://www.dasoertliche.de/Themen/Schneider-Marko-Allstedt-Wolferstedt-Nienstedter-Str ;
  https://firmeneintrag.creditreform.de/06542/3130174407/MARKO_SCHNEIDER_SCHROTTENTSORGUNG ;
  https://www.schrottplatz-info.de/schrottplatz/Allstedt/Schneider-Marko (Abruf 04.11.2026).]
