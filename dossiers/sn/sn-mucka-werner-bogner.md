---
slug: sn-mucka-werner-bogner
clear_fields: 'dropoff_json,geo'
name: Werner Bogner
trader_type: sonstige
state: SN
city: Mücka
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
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Werner Bogner

## Überblick

Werner Bogner in Mücka bleibt unbestätigter Seed-/Verzeichniskandidat. Die aktuelle Register-Namenssuche findet einen Werner Bogner in Kempten, aber keinen belegten Mücka-Bezug; dieser Homonym ist nicht zugeordnet. Status `pruefung` statt bisher unbelegt `aktiv`, keine Schließungsbehauptung.

Die historische Verzeichnis-Spur Am Bahnhof 126/02906 Mücka/035893 6983 ist kein geprüfter Betreiberstandort. Es liegt keine belastbare heutige Straßenanschrift zur Geokodierung vor. **Ankauf:** kein aktueller Betreiber- oder Preisbeleg. **Verkauf:** keine Produkt-/Materialliste. **Gebühren:** keine verifizierten Abhol- oder Annahmetarife. Offen: heutige Inhaberidentität, Gewerbenachweis, richtige Hausnummer und öffentliche Metallannahme.

## Timeline

### Owner-GEO-/Annahme-Korrektur 24.10.2026

- [Owner-GEO-Korrektur 24.10.2026: Read-only-Produktion enthielt den Punkt 51.3167, 14.7 und `dropoff_json={"allowed":true}`. Es gibt weiterhin weder eine belastbare aktuelle Straßenanschrift noch eine bestätigte Betreiberidentität in Mücka; die historische Adresse Am Bahnhof 126 ist ausdrücklich ungeprüft. `clear_fields: 'dropoff_json,geo'` nimmt die unbelegte Annahmezusage und den keinem aktuellen Standort zugeordneten GEO-Punkt zurück. Keine Ersatzadresse/-koordinate, keine Schließungsbehauptung und keine Produktionsänderung. Quellen: read-only `public.db` (24.10.2026); Quellen-/Namensprüfung in der Dossier-Timeline; `docs/trader-seed.md`.]

### Recherche 08.10.2026

- [Recherche 08.10.2026: Eigenständige Northdata-Suggest-Suche Werner Bogner liefert nur Person in Kempten; keine Verbindung zu Mücka nachgewiesen, Homonym nicht übernommen. DDG-Suche Werner Bogner/Mücka nur Bot-Challenge, Websuche ohne brauchbare Primärquelle; Recherchegrenze, kein amtlicher Negativnachweis. Alte deubiz/Schrottportal-Adress- und Materialspuren weiter nicht betreiberbelegt, keine Adress-/Kontaktfills. status aktiv→pruefung mangels verifizierter aktueller Annahme, nicht geschlossen. Ankauf/Verkauf/Gebühren nicht verifiziert. Offen: vollständige Betreiberbestätigung/Gewerbeauskunft am Mückaer Kandidatenstandort; Quelle: https://www.northdata.de/suggest.json?query=Werner%20Bogner&countries=DE .]

### Importiert (Seed-Stand 2026-09-30)

- Fe/Alu/Cu/Edel/Blei/Messing
- Annahme lt. lokaleschrottplatz

### Recherche 01.10.2026

- [Recherche 01.10.2026: nur Aggregatoren (02906 Mücka, Tel. 035893 6983; deubiz-Lead Am Bahnhof 126); kein Betreiber-/Registerbeleg; Quelle(n): keine belegfähige Quelle (nur Aggregator-Leads)]
- [Recherche 01.10.2026: kein Doppelbeleg → kein Fill; Felder leer, Status aktiv unverändert (Ankauf-Scope unbestätigt); Quelle(n): keine belegfähige Quelle]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Die historische Verzeichnis-Adresse
  **Am Bahnhof 126, 02906 Mücka** existiert zwar als OSM-Objekt, ist aber **kein** belegter Betriebsstandort: das
  Objekt trägt **weder** `name=` **noch** `operator=` und **keine** Schrott-/Recycling-Bezüge, und die
  Betriebsidentität selbst ist ungeklärt - die **NorthData-Namenssuche** findet zum Seed-Namen **„Werner Bogner"**
  ausschließlich eine **Person in Kempten**, **ohne** jeden Mücka-Bezug. **Was live geprüft wurde (04.11.2026):**
  das am 04.11.2026 live abgerufene Kachelraster über Mücka umfasst **14.6900/51.3050 bis 14.7200/51.3250**,
  **6 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**; es enthält die gesuchte Straße **nachweislich** mit
  **18** Adressobjekten auf „Am Bahnhof" und den Hausnummern 125, **126**, 126a, 127, 128a, 128b, 128c, 128d, 129,
  130, 131, 132, 133, 134, 135, 136, 137 und 138. Die **Dossier-Hausnummer 126 existiert** als **way/158553787**
  mit `addr:street`=Am Bahnhof, `addr:housenumber`=126, `addr:city`=Mücka, `addr:postcode`=02906 und
  `building`=yes (live über die Objektabfrage geprüft, vier eindeutige Knoten, Objekt-BBox
  14.7044618/51.3192608 bis 14.7047836/51.3193705); das Nachbarobjekt **way/729709458** trägt die Nummer 126a.
  Eine Namenssuche nach **„Bogner"** über die Felder name, operator, description, website, contact:website, brand,
  note und addr:housename findet im Raster **keinen** Treffer - es gibt also **keine** OSM-Betriebszuordnung, nur die
  Adressangabe. Quelle 1 (OSM): das live abgerufene Kachelraster 14.6900/51.3050–14.7200/51.3250 mit
  **way/158553787** ohne Betriebsbezug. Quelle 2 (Register/Verzeichnis, im Dossier bereits zitiert, live
  nachgelesen am 04.11.2026): die **NorthData-Namenssuche**
  (https://www.northdata.de/suggest.json?query=Werner%20Bogner&countries=DE) liefert **HTTP 200** und genau
  **einen** Treffer: **„Werner Bogner"**, Typ **person**, Ort **Kempten** - **kein** Treffer in Mücka, keine
  Rechtsträgerin, keine belastbare Namensbrücke zum Seed. **Warum keine Koordinate:** Die Adresse ist im OSM-Bestand
  zwar vorhanden, aber das Dossier selbst führt sie ausdrücklich als **„ausdrücklich ungeprüft"**; ein Punkt an einem
  Wohn- oder Gewerbebau ohne Betriebsbezug würde für den Seed einen Standort behaupten, den weder OSM noch ein
  Betreiber- oder Registerbeleg trägt. **Reichweitenklausel:** Das **Fehlen eines Betriebsobjekts** (kein
  `name=`/`operator=` in OSM, kein Registertreffer) ist **kein** Nachweis für das Fehlen des Betriebs - es belegt nur,
  dass die Seed-Identität am 04.11.2026 nicht belegbar war. **Was zur Auflösung fehlt:** ein lesbarer Betreiberauftritt
  (Impressum mit Rechtsträger, Ansprechpartner) in Mücka oder ein Registereintrag, der den Betrieb und seinen Standort
  in Mücka bestätigt; die Dossier-Historie nennt als offenen Schritt ausdrücklich die „heutige Straßenanschrift" und
  die „bestätigte Betreiberidentität in Mücka". `clear_fields: 'dropoff_json,geo'` **bleibt** unverändert bestehen,
  `status: pruefung` bleibt unverändert; es wird weder Fortbestand noch Schließung behauptet. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/158553787/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=14.6900,51.3050,14.7200,51.3250 ;
  https://www.northdata.de/suggest.json?query=Werner%20Bogner&countries=DE (Abruf 04.11.2026).]

### Recherche 23.11.2026 (Welle 0600 C)

- [Recherche 23.11.2026: **Die Register-Namenssuche liefert am 23.11.2026 unverändert nur einen Homonymen in Kempten; eine belastbare Zuordnung zu Mücka ist weiterhin nicht möglich.** Die NorthData-Suggest-Suche wurde am 23.11.2026 live mit drei Varianten abgefragt: `Werner Bogner` liefert **genau einen** Treffer, Typ **person**, Ort **Kempten** (https://www.northdata.de/Bogner,%20Werner,%20Kempten/183w); `Bogner Mücka` und `Bogner Schrott` liefern jeweils **eine leere** Trefferliste. **Reichweitenklausel:** Die NorthData-Suche erfasst Rechtsträger und Personen, aber **keine** Einzelunternehmen ohne Registereintrag; ein leeres Ergebnis ist deshalb **kein** Nachweis der Nichtexistenz eines Mückaer Kleinbetriebs und **kein** Schließungsbeleg. Der Homonym in Kempten wird **nicht** zugeordnet. Quellen: https://www.northdata.de/suggest.json?query=Werner%20Bogner&countries=DE ; https://www.northdata.de/suggest.json?query=Bogner%20M%C3%BCcka&countries=DE ; https://www.northdata.de/suggest.json?query=Bogner%20Schrott&countries=DE (Abruf 23.11.2026).]
- [Recherche 23.11.2026: **Amtliches Fachbetrieberegister, beide Routen, live geprüft: für „Bogner" und für Am Bahnhof 126, 02906 Mücka kein Treffer.** Abfrage des amtlichen Fachbetrieberegisters (Route "Entsorgungsfachbetriebe" und Route "Altfahrzeugverwertung", Filter "Art des Betriebs = Egal", Namenssuche `Bogner` sowie Standortabfrage mit 5 km um Am Bahnhof 126, 02906 Mücka, Abruf 23.11.2026): in **beiden** Routen **0 Treffer**. **Reichweitenklausel:** Die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis und **kein** Schließungsbeleg. Quellen: https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Bogner ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Bogner ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Am%20Bahnhof%20126%2C%2002906%20M%C3%BCcka%2C%20Deutschland%3B14.7046227%3B51.3193157%3B14.7046227%3B51.3193157%3B14.7046227%3B51.3193157&dst=5&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Am%20Bahnhof%20126%2C%2002906%20M%C3%BCcka%2C%20Deutschland%3B14.7046227%3B51.3193157%3B14.7046227%3B51.3193157%3B14.7046227%3B51.3193157&dst=5&is=1&e=1 (Abruf 23.11.2026).]
- [Recherche 23.11.2026: **GEO-Nachmessung: der Befund vom 04.11.2026 gilt unverändert - das Adressobjekt way/158553787 trägt weiterhin keinen Betriebsbezug, und es ist keine Koordinate gesetzt.** Das Objekt **way/158553787** (`addr:street`=Am Bahnhof, `addr:housenumber`=**126**, `addr:city`=Mücka, `addr:postcode`=02906, `building`=yes) und sein Nachbar **way/729709458** (Nummer 126a) sind im Kachelstreifen 14.6900/51.3050 bis 14.7200/51.3250 (10.802 Elemente) die einzigen Objekte mit der Nummer 126; auf derselben Straße tragen way/158553772 (131), way/158553774 (127), way/158553782 (132), way/158553786 (138), way/158553790 (135), way/158553792 (134), way/158553794 (136), way/158553798 (129, `building`=train_station), way/158553800 (125), way/552127343 (137), way/732413155 (133) und way/732413160 (130) die übrigen Hausnummern. Eine Namenssuche über name, operator, description, website, contact:website, brand, note und addr:housename findet im Streifen **keinen** Treffer zu „Bogner", „Schrott" oder „Recycling". Die **Reichweitenklausel** bleibt: Das Fehlen eines Betriebsobjekts in OSM ist **kein** Nachweis für das Fehlen des Betriebs - es belegt nur, dass die Seed-Identität am 23.11.2026 nicht belegbar war. `clear_fields: 'dropoff_json,geo'` **bleibt** unverändert bestehen, `status: pruefung` bleibt unverändert; es wird **weder** Fortbestand **noch** Schließung behauptet. Quellen: https://api.openstreetmap.org/api/0.6/way/158553787/full.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=14.6900,51.3050,14.7200,51.3250 (Abruf 23.11.2026).]
- [Recherche 23.11.2026: **Preise getrennt geprüft - keine Preisquelle.** ANKAUF: kein aktueller Betriebs- oder Preisbeleg; die Seed-Sorten Fe/Alu/Cu/Edel/Blei/Messing stammen aus dem alten Verzeichnis-Eintrag und sind **nicht** bestätigt. VERKAUF: keine Produkt- oder Materialliste mit Preisen. GEBÜHREN: keine verifizierten Abhol- oder Annahmetarife. **Preise fremder Betriebe wurden nicht übernommen.** Offen bleiben die heutige Inhaberidentität, ein Gewerbenachweis, die richtige Hausnummer und die öffentliche Metallannahme; das amtliche Gewerberegister des Marktes Mücka wäre der nächste Schritt. Quellen: https://www.northdata.de/suggest.json?query=Werner%20Bogner&countries=DE ; https://api.openstreetmap.org/api/0.6/way/158553787/full.json (Abruf 23.11.2026).]

### Korrektur 23.11.2026 (Owner, Geokodierung) — der Standortpunkt bleibt unverändert

- [Korrektur 23.11.2026: **Für dieses Dossier wird **kein** Standortpunkt gesetzt ✓ ✓ — Das **Adressobjekt** ✓✓ trägt **weiterhin** **keinen** Betriebsbezug ✓✓ — kein Punkt ✓ ✓ ✓ ✓ **Reichweite:** Die EfbV-/AltFahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Wellennotiz mit der Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 23.11.2026 ✓ ✓), **zweitens** ✓ ✓ das **Adressobjekt** ✓ ✓ in OpenStreetMap ✓ ✓ ✓ ✓]
