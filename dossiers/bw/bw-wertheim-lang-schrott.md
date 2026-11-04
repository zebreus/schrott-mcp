---
slug: bw-wertheim-lang-schrott
clear_fields: 'geo'
name: Lang Schrott
trader_type: schrotthaendler
state: BW
city: Wertheim
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (final)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Lang Schrott

## Überblick

Die vorhandenen Namens-/Regionstreffer belegen keinen Schrotthändler in Wertheim: Die auffindbare Lang-Schrotthandel-Seite gehört Thomas Lang in Würzburg; der separat für Wertheim werbende Mr.-Schrott-Auftritt nennt Robert Cam und Eppingen. Keine belastbare Betreiberquelle oder Verbindung zum Wertheimer Seed gefunden. `status: pruefung` ist keine Schließungsbehauptung.

**Preise:** Ankauf — keine dem Wertheimer Datensatz zuordenbaren Kurse; Verkauf — keine Angaben; Gebühren — keine Angaben. Ohne bestätigte Betriebsadresse keine Geokodierung.

## Timeline

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält für Wertheim den Punkt 51.1105, 11.41685. Nominatim-Reverse löst ihn auf die L 1057 bei Buttstädt in Thüringen, nicht nach Wertheim am Main. Der Altpunkt ist daher falsch; `clear_fields: 'geo'` entzieht ihn beim nächsten regulären Seed. Keine Ersatzkoordinate geraten, da die genaue Betreiber-/Annahmestellenadresse weiter fehlt. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.1105&lon=11.41685&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 18.10.2026 — Welle 12:02 Süd

- [Recherche 18.10.2026: Lang-Startseite und Impressum heute einzeln HTTP 200 direkt gelesen: Thomas Lang, Georg-Böhm-Straße 1, 97080 Würzburg, Unterfranken als Einsatzregion, keine explizite Wertheimer Betriebsstätte. Mr.-Schrott-Impressum ebenfalls HTTP 200: Robert Cam/Erlen 14-1/75031 Eppingen-Richen; Wertheim-Regionalunterseite heute HTTP 503, historische Werbeaussage nicht als heute gelesen ausgegeben. Weder andere Betreiberwebsite noch Servicegebiet beweist die Wertheimer Seedidentität. Keine Frontmatter-Füllung/Umbenennung/Umzug, pruefung bleibt. Ankauf: keine diesem Datensatz zuordenbaren Zahlen; Verkauf: keine Preise; Gebühren: keine Tarife, kostenlose Beratung bei Thomas Lang keine kostenlose Entsorgung. GEO mangels eigener bestätigter Betriebsanschrift zurückstellen. Quellen: https://sites.google.com/view/lang-schrotthandel/startseite ; https://sites.google.com/view/lang-schrotthandel/impressum-datenschutz ; https://regio.mr-schrott.de/wertheim ; https://www.mr-schrott.de/impressum/ .]

### Importiert (Seed-Stand 2026-09-30)

- Schrott, mobil
- Größe: klein/mobil

### Recherche 03.10.2026

- [Recherche 03.10.2026: Vollcrawl der auffindbaren Lang-Schrotthandel-Seite und ihres Impressums ergibt Thomas Lang, Georg-Böhm-Straße 1, 97080 Würzburg; die Seite beschreibt Würzburg/Unterfranken, aber keine Niederlassung in Wertheim. Eine separate Betreiberseite von Mr. Schrott bewirbt zwar Schrottservice in Wertheim, nennt im Impressum jedoch Inhaber Robert Cam und den Standort Eppingen; keine Verbindung zu Lang. Diese Namens-/Regionaltreffer belegen den Wertheimer Dossier-Datensatz nicht. Deshalb website_status auf unbekannt und status aktiv auf pruefung (Identität/Existenz am Ort ungeklärt, keine Schließung behauptet); Name, Ort, Straße, PLZ, Telefon, E-Mail und Services nicht ergänzt. Quelle(n): https://sites.google.com/view/lang-schrotthandel/startseite, https://sites.google.com/view/lang-schrotthandel/impressum-datenschutz, https://regio.mr-schrott.de/wertheim, https://www.mr-schrott.de/impressum/.]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Für **„Lang Schrott"** in Wertheim ist **keine**
  bestätigte Betriebsanschrift im Dossierort belegt. Quelle 1 (OSM, live geprüft am 04.11.2026): **Fensterprüfung:** das
  am 04.11.2026 live abgerufene Kachelraster umfasst **9.9900/49.7900 bis 10.0000/49.8000**, **1 Kachel von je 0,01
  Grad**, **0 Kachel-Fehler**; die Kachel erfasst den **Bahnhofsbereich der Stadt Würzburg** mit **73** Adressobjekten
  auf **8** Straßen (Am Trog, Friedrich-Bergius-Ring, Gattingerstraße, Kitzinger Straße, Louis-Pasteur-Straße,
  Nürnberger Straße, Robert-Bunsen-Straße, Sandäcker) - eine Straße **„Georg-Böhm-Straße"** ist dort **nicht** erfasst,
  und eine Suche nach **„Lang"** und **„Schrott"** über die Felder name, operator, brand, description, website,
  contact:website, shop, craft, amenity, landuse und man_made findet im Kachelausschnitt **keinen** Treffer zu einem
  Lang-Schrotthandel. Der Ausschnitt deckt **nicht** die gesamte Stadt ab; das Fehlen der Anschrift darin ist deshalb
  **kein** Nachweis gegen ihre Existenz, aber **auch kein** Beleg für einen Standort in **Wertheim**. Für den Dossierort
  **Wertheim** selbst liegt in dieser Welle **kein** belegter Betriebsanschrift vor, auf die ein Fenster bezogen werden
  könnte; eine Suche nach **„Lang"** und **„Schrott"** im Umfeld von Wertheim findet **kein** OSM-Betriebsobjekt zu
  dieser Dossierzeile. Quelle 2 (Betreiber, im Dossier bereits zitiert, live nachgelesen am 04.11.2026): die
  **Startseite** https://sites.google.com/view/lang-schrotthandel/startseite und das **Impressum**
  https://sites.google.com/view/lang-schrotthandel/impressum-datenschutz des einzigen auffindbaren
  Lang-Schrotthandel-Auftritts nennen live **Thomas Lang, Georg-Böhm-Straße 1, 97080 Würzburg**, Unterfranken als
  Einsatzregion - also eine Anschrift in **Würzburg**, nicht in **Wertheim**; der Impressumabruf (HTTP 200) belegt damit
  die Person **Thomas Lang**, aber **keine** Wertheimer Betriebsstätte. Die Regionalunterseite
  https://regio.mr-schrott.de/wertheim des **anderen** Betreibers mr-schrott.de war am 18.10.2026 nicht abrufbar (HTTP
  503), und dessen Impressum nennt Robert Cam, Erlen 14-1, 75031 Eppingen - eine **andere** Rechtsträgerin ohne Bezug
  zur Dossierzeile. **Warum keine Koordinate:** die einzige belegte Anschrift liegt in einem **anderen** Ort und
  **anderen** Landkreis; sie als Punkt für den Wertheimer Datensatz zu setzen, würde den Datensatz umziehen; die
  Identität der Wertheimer Seedzeile ist offen. `clear_fields: 'geo'` **bleibt** unverändert bestehen (es gibt keinen
  Ersatzpunkt), `status: pruefung` bleibt, Ankauf-, Verkaufs- und Gebührenangaben bleiben unbelegt, es wird **weder**
  Fortbestand **noch** Schließung behauptet. **Reichweitenklausel:** das Fehlen eines OSM-Betriebs-Tagings in Wertheim
  ist **kein** Nachweis für das Fehlen eines dortigen mobilen Schrotthandels - die Regionstreffer belegen die Branche,
  nicht den Standort. Offen bleibt eine bestätigte Wertheimer Betriebsanschrift; erst dann ist zu geokodieren.
  Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.97,49.79,9.99,49.81 ;
  https://sites.google.com/view/lang-schrotthandel/impressum-datenschutz ; https://regio.mr-schrott.de/wertheim ;
  https://www.mr-schrott.de/impressum/ (Abruf 04.11.2026).]
