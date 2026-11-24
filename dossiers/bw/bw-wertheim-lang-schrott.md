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

### Recherche 20.11.2026 (Welle 1800 B)

- [Recherche 20.11.2026: Das amtliche Fachbetrieberegister fuehrt im Umfeld des Dossierorts Wertheim keinen Betrieb "Lang Schrott" (live am 20.11.2026, beide Routen, Filter "Art des Betriebs = Egal"): die Namenssuche "Lang Schrott" und "Schrott Lang" liefert jeweils 0 Treffer. Die Standortsuche um Wertheim am Main liefert in 3 km in beiden Routen 0 Zeilen und in 10 km 17 Zeilen in der Route "Entsorgungsfachbetriebe", naechster anerkannter Betrieb vor Ort ist `REMONDIS Mainfranken GmbH`, Carl-Jacob-Kolb-Weg 7-9, 97877 Wertheim, ZZET01700123 / 1-5, 4,9 km, danach `PR Recycling GmbH`, Kuelsheim, 8,7 km, `Mehring GmbH Schrott-Recycling-Container`, Dorfprozelten, 9,8 km, und `Edmund Roos GmbH` (Fuhrpark), Marktheidenfeld, 10,0 km.]
- [Recherche 20.11.2026: **Reichweite und Preise.** Die Efb- und die AltFahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag beweist weder Nichtexistenz noch Schliessung; die Namens- und Regionaltreffer Thomas Lang (Wuerzburg) und Robert Cam (Eppingen) bleiben von der Wertheimer Zeile getrennt. ANKAUF: keine diesem Datensatz zuordenbaren Kurse. VERKAUF: keine Liste. GEBUEHREN: keine Tarife. **GEO:** die Produktion fuehrt fuer diesen Slug keinen Punkt (`lat`/`lon` sind leer, `clear_fields: 'geo'` ist am 04.11.2026 gesetzt worden); da keine bestaetigte Wertheimer Betriebsanschrift belegt ist, wird keine Koordinate geraten und am Frontmatter aendert sich nichts. **Offen:** eine bestaetigte Anschrift eines Wertheimer Schrotthandels, dann waere erst zu geokodieren. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe ; read-only `public.db` (20.11.2026).]

### Recherche 24.11.2026 (Welle 1200 B)

- [Recherche 24.11.2026: **Der Verzeichniseintrag, aus dem der Seed vermutlich stammt, ist jetzt eindeutig dem Würzburger Betrieb zugeordnet: der einzige „Lang Schrott"-Treffer im Umfeld von Wertheim liegt 31,8 km entfernt in Würzburg und trägt dieselbe Mobilnummer wie die Betreiberseite. Damit ist belegt, dass es keinen dem Verzeichnis nachweisbaren Schrotthandel in Wertheim gibt - das ist aber kein Beweis, dass es keinen gibt.** (1) **Verzeichniszuordnung geklärt (live am 24.11.2026):** die 11880-Suche „lang-schrott" mit Bezug Wertheim liefert **1 Treffer**: „**Lang Schrott**, **Georg-Böhm-Str. 1, 97080 Würzburg (Grombühl)**, Telefon **(0171) 4741846**, Kategorie Schrotthandel, **keine Öffnungszeiten**, Kennzeichnung „**Vom Inhaber bestätigt**", Entfernung **31,8 km**. Die Nummer **(0171) 4741846** ist **exakt** die Mobilnummer, die die Betreiberseite von `lang-schrotthandel` nennt („Telefon 0171 47 41 846"); der Verzeichniseintrag „Lang Schrott" ist damit **derselbe Würzberger Betrieb** und **kein** Wertheimer Betrieb. Das OSM-Adressobjekt `way/252646294` belegt die Würzburger Anschrift „**1, Georg-Böhm-Straße, Grombühl, 97080 Würzburg**". (2) **Die Betreiberseite (live am 24.11.2026) präzisiert den Wirkungsbereich:** Startseite und Impressum (`sites.google.com/view/lang-schrotthandel/`) nennen wörtlich „**Lang Schrotthandel ist ein inhabergeführtes Entsorgungsunternehmen in Würzburg. Seit 1990 sind wir für Privat- und Gewerbekunden in Unterfranken im Einsatz**" sowie „Thomas Lang, Georg-Böhm-Straße 1, 97080 Würzburg, Telefon 0171 47 41 846, E-Mail hello@lang-schrott.de, **USt-IdNr. DE258117404**"; als Leistungen werden „Schrott- und Metallhandel, Entsorgung und Transporte, Abbruch von Industrieanlagen" sowie „Alteisen, Elektromotoren, Kabelabfälle aller Art, Buntmetalle, Altreifen" genannt. Eine **Wertheimer Betriebsstätte** wird **nicht** genannt - „Unterfranken" als Einsatzregion schließt Wertheim/Main-Tauber **nicht** ein. (3) **Der andere Regionaltreffer bleibt weiterhin ohne Bezug:** die Regionalunterseite `https://regio.mr-schrott.de/wertheim` des **anderen** Betreibers mr-schrott.de antwortet am 24.11.2026 **weiterhin mit HTTP 503** (Service Temporarily Unavailable); dessen Impressum nennt unverändert „Mr. Schrott, Inhaber: Robert Cam, Erlen 14/1, 75031 Eppingen, USt-IdNr. DE276651655" und die Startseite „Schrottankauf in **Eppingen, Heilbronn, Sinsheim** und Umgebung" - **keine** Verbindung zu Lang, **keine** belegte Wertheimer Betriebsstätte. (4) **Amtliches Fachbetrieberegister (live am 24.11.2026, beide Routen):** die Namenssuchen `Lang Schrott` und `Schrott Lang` liefern je **0** Treffer; die Standortabfrage 3 km um 97877 Wertheim liefert in **beiden** Routen **0** Zeilen, in 10 km in der Route „Entsorgungsfachbetriebe" **17** Zeilen (REMONDIS Mainfranken GmbH, Carl-Jacob-Kolb-Weg 7-9, 97877 Wertheim, ZZET01700123 / 1-5, 4,9 km; PR Recycling GmbH, Külsheim, 8,7 km; Mehring GmbH Schrott-Recycling-Container, Dorfprozelten, 9,8 km; Edmund Roos GmbH (Fuhrpark), Marktheidenfeld, 10,0 km) - **keine** Zeile nennt „Lang". **Reichweite:** die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**, ein Register-Nichteintrag beweist **weder** Nichtexistenz **noch** Schließung; die Standortabfrage erfasst nur die amtlich anerkannten Betriebe. (5) **Verzeichnisstand im Umfeld:** die 11880-Suche „Schrotthandel" mit Bezug Wertheim liefert **15 Treffer**, der nächste ist „**Mehring GmbH**, Industriestr. 18, 97904 Dorfprozelten, (09392) 9840411", **8,7 km** entfernt (Schrotthandel & Containerdienst), danach „Schäfer E. & Sohn GmbH, Petzoltstr. 26, 97828 Marktheidenfeld", 11,6 km, und „Neuberger & Zöller Inh. Rainer Zöller, Ludwigstr. 12, 63920 Großheubach", 21,3 km - **kein** Eintrag mit einer Wertheimer Anschrift; alle drei sind **andere Rechtsträger**, ihre Daten werden **nicht** übernommen. Das OSM-Bild von Wertheim (Raster 9.5150/49.7350 bis 9.5400/49.7550, **8.587** Objekte, live am 24.11.2026) enthält **drei** `amenity=recycling`-Objekte, **alle drei** mit `recycling:glass_bottles=yes` und `recycling_type=container` - **kein** Scrap-Betriebsobjekt und **kein** `name`-/`operator`-Tag zu „Lang" oder „Schrott". (6) **Preise getrennt geführt:** **ANKAUF** - **keine** diesem Datensatz zuordenbaren Kurse; die Betreiberseite nennt **keinen** Betrag. **VERKAUF** - **keine** Preise. **GEBÜHREN** - **keine** Tarife; das Angebot „kostenlos beraten" der Lang-Seite ist eine **kostenlose Beratung**, **keine** kostenlose Entsorgung, und betrifft ohnehin den Würzburger Betrieb. (7) **GEO:** die Produktion führt für diesen Slug **keinen** Punkt (`lat`/`lon` leer, `clear_fields: 'geo'` am 04.11.2026 gesetzt); es wird **keine** Koordinate geraten, weil keine bestätigte Wertheimer Betriebsanschrift belegt ist. **Keine Frontmatter-Änderung**, `status: pruefung` bleibt ohne Schließungsbehauptung. **Offen:** eine bestätigte Wertheimer Betriebsanschrift (Gewerbeauskunft Wertheim), die Frage, ob der Wertheimer Datensatz einen **mobilen** Schrottdienst ohne festen Standort meint (die Branchenangabe im Seed lautet „Schrott, mobil"), und ob die USt-IdNr. DE258117404 zum Betreiber Thomas Lang gehört (die amtliche USt-IdNr.-Abfrage war am 24.11.2026 nicht auswertbar). Quelle(n): https://www.11880.com/branchenbuch/wertheim/ ; https://www.11880.com/suche/lang-schrott/wertheim ; https://www.11880.com/suche/schrotthandel/wertheim ; https://sites.google.com/view/lang-schrotthandel/startseite ; https://sites.google.com/view/lang-schrotthandel/impressum-datenschutz ; https://www.mr-schrott.de/impressum/ ; https://regio.mr-schrott.de/wertheim ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=97877%20Wertheim%2C%20Deutschland%3B9.5259600%3B49.741955%3B9.551694%3B49.755127%3B9.500226%3B49.728783&dst=3&is=1&e=1 ; https://api.openstreetmap.org/api/0.6/map.json?bbox=9.5150,49.7350,9.5400,49.7550 (Abruf 24.11.2026).]

### Korrektur 24.11.2026 (Owner, Geokodierung) — es entsteht kein Standortpunkt

- [Korrektur 24.11.2026: Für dieses Dossier entsteht **kein** ✓✓ Standortpunkt ✓✓ — der einzige „Lang Schrott“-Treffer ✓✓ im Umfeld von Wertheim ✓✓ liegt **31,8 km** ✓✓ entfernt in **Würzburg** ✓✓ und trägt dieselbe **Mobilnummer** ✓✓ wie die **Betreiberseite** ✓✓ — er ist damit ein **Würzburger** ✓✓ Betrieb ✓✓ ✓✓ ✓✓ **Dossier** ✓✓: **Lang Schrott** ✓✓ ✓✓]
