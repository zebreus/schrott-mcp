---
slug: nw-hagen-matthies
name: Matthies
trader_type: schrotthaendler
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

# Matthies

## Überblick

Ein aktueller 11880-Verzeichniseintrag nennt **Rosalie Matthies Metall- u. Schrottankauf**, Zur-Nieden-Str. 20, 58095 Hagen, 02331 16452; er ist ein Suchlead, kein Betreiber-/Registerbeleg und keine bewiesene Verbindung zum Seednamen „Matthies“. Daher bleiben Adresse, Telefon und Betrieb ungefüllt; der frühere gemeinsame GEO-Punkt ist ausdrücklich zurückgezogen und wird nicht durch diesen unverbundenen Lead ersetzt.

**Ankauf:** nur qualitative Verzeichnis-Kategorie, keine Kurse. **Verkauf:** keine Liste. **Gebühren:** keine Abhol-/Annahmetarife. **GEO:** Zur-Nieden-Str. 20 erst nach Betreiber-/Seed-Zuordnung und Bestätigung einer Kundenannahmestelle neu geokodieren.

## Timeline

### Recherche 27.10.2026: neuer Hagen-Verzeichnislead, Identität offen

- [Recherche 27.10.2026: 11880 führt „Rosalie Matthies Metall- u. Schrottankauf“, Zur-Nieden-Str. 20, 58095 Hagen, 02331 16452; Metadaten nennen Eintrag vom 29.06.2021 und Aktualisierung 01.10.2026. Das bleibt ein einzelner Verzeichnislead und belegt weder die Verbindung zur Seedzeile „Matthies“ noch aktuelle Annahme. Keine Stammdatenübernahme und kein GEO; `clear_fields: 'geo'` bleibt wichtig, weil der alte gemeinsame Punkt nach Mehringen aufgelöst wurde. **Ankauf:** keine Preise; **Verkauf:** keine Liste; **Gebühren:** keine Tarife. Quelle (Lead): https://www.11880.com/branchenbuch/hagen-westfalen/120674719B1730010/rosalie-matthies-metall-u-schrottankauf.html .]

### Korrektur 24.10.2026: Gemeinsamen falschen Hagen-Punkt zurückziehen

- [Korrektur 24.10.2026: Die Produktionszeile `nw-hagen-matthies` enthielt den mit mehreren anderen Hagen-Datensätzen geteilten Punkt `51.90933, 7.88825`. Die read-only-Rückwärtssuche löst ihn in Mehringen, Everswinkel, Kreis Warendorf, PLZ 48351 auf, nicht in Hagen. `clear_fields: 'geo'` zieht den falschen Punkt zurück; mangels belegter Händleradresse wird keine Ersatzkoordinate gesetzt. Die Produktionsdatenbank blieb unverändert. Quelle: https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; https://www.openstreetmap.org/way/81516036.]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Kein belastbarer Treffer — keine Betreiber-Website, kein Register-, kein Kommunalbeleg gefunden (Websuche ohne Ergebnis). Felder leer gelassen, Klärfall für Folgewelle (Existenz/Adresse ungeklärt). Quelle(n): keine.]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Hagen

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Lead-Adresse **Zur-Nieden-Str. 20,
  58095 Hagen** ist in OSM adressgenau erfasst, stützt sich aber allein auf **einen** Verzeichniseintrag; der im
  Dossier festgehaltene Einzel-Lead-Charakter bleibt maßgeblich. **Was live geprüft wurde (03.11.2026):** (a) Das
  Adressobjekt existiert: **way/214233262** mit `addr:housenumber`=**20**, `addr:street`=**Zur-Nieden-Straße**,
  `addr:city`=Hagen, `addr:country`=DE, `addr:postcode`=**58095**, `building`=yes (Mittel der 4 eindeutigen Way-Knoten
  **51.3530197/7.4752698**, BBox 51.3529432/7.4751490 – 51.3530961/7.4753907) — Dossier-Straße, -Hausnummer und -PLZ
  stimmen objektgenau überein. (b) Das Objekt trägt **weder** `name=` **noch** `operator=`; im Umfeld von 80 m liegen
  nur die Straßenobjekte der Zur-Nieden-Straße und der Bergstraße sowie node/12124409764 `name`=Goldbergquelle — **kein**
  Bezug zum Betrieb. Eine Suche über `name`, `operator`, `brand`, `website`, `contact:website`, `description`, `note`
  und `short_name` nach **„Matthies“** im Kachelraster **7.4200/51.3400 – 7.5600/51.4200** (112 Kacheln, 0 Kachel-Fehler)
  liefert **0** Treffer — Aussage zum OSM-Bestand, **kein** Negativbeleg über den Betrieb. Quelle 1 (Verzeichnis,
  **keine** Betreiber-/Registerquelle): der 11880-Eintrag „Rosalie Matthies Metall- u. Schrottankauf“, **Zur-Nieden-Str. 20,
  58095 Hagen**, **02331 16452** (Eintrag 29.06.2021, Aktualisierung 01.10.2026 — Portalmetadaten); das Dossier hält
  selbst fest, dass dies „ein einzelner Verzeichnislead“ ist, der „weder die Verbindung zur Seedzeile ‚Matthies‘ noch
  aktuelle Annahme“ belegt, und dass „keine Stammdatenübernahme und kein GEO“ erfolgt. Quelle 2 (OSM): die Adresse ist
  vorhanden, der **Betriebsbezug** fehlt. **Was fehlt:** eine zweite unabhängige Quelle — Betreiberauftritt,
  Gewerbeauskunft oder ein weiterer, eigenständig erhobener Registereintrag —, die „Matthies“ an die Zur-Nieden-Straße 20
  bindet; erst damit wäre der Standort gesichert. `clear_fields: 'geo'` bleibt bestehen, `status: pruefung` bleibt
  unverändert, es wird weder Fortbestand noch Annahme behauptet.
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/214233262/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.4200,51.3400,7.5600,51.4200 ;
  https://www.11880.com/branchenbuch/hagen-westfalen/120674719B1730010/rosalie-matthies-metall-u-schrottankauf.html
  (Abruf 03.11.2026).]
