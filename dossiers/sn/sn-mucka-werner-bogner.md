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
