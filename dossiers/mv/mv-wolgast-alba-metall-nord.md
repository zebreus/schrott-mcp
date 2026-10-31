---
slug: mv-wolgast-alba-metall-nord
name: ALBA Metall Nord GmbH
trader_type: sonstige
state: MV
city: Wolgast
street: Kröslinerstr. 5
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://metall.alba.info/unternehmen/standorte/
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Betriebe (Tabelle)
provenance_ankauf_raw: ja
provenance_origin: table
---

# ALBA Metall Nord GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Stahl/NE, Container, Demontage

### Korrektur 01.10.2026 (ALBA-Fix)

- Delisted-Protokoll: Wolgast steht NICHT auf der aktuellen Betreiber-Standortliste (Vollcrawl 01.10.2026: metall.alba.info = 13 Detailseiten ohne Wolgast; alba.info/standorte ohne Treffer). Ältere Liste hatte 17 Treffer inkl. Wolgast.
- Betreiber-Suche/Register/News ohne Schließungs-/Verlagerungsbeleg → kein status geschlossen, Dossier bleibt erhalten; status aktiv → pruefung + Klärfall: Filial-Existenz und Adressbeleg (Kröslinerstr. 5 — bislang ohne Betreiber-Beleg) offen.
- website = Domain-Root (aktiv, Impressum via alba.info); keine Detailseite.

### Recherche 01.10.2026

- [Recherche 01.10.2026 / Feedback-Triage 2305: Delisting berechtigt — ALBA-Fix-Befund bestätigt (Vollcrawl 01.10.2026: metall.alba.info 13 Standorte ohne Wolgast; alba.info/standorte ohne Treffer); Dossier bleibt pruefung; Quelle(n): metall.alba.info/unternehmen/standorte/, alba.info/standorte/ (Abruf 01.10.2026)]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Krösliner Straße, Wolgast) — die Dossier-Hausnummer 5 existiert auf dieser Straße nicht.** Straßen-Raster über den **gesamten** Straßenverlauf (Straßenbbox 13.7568043/54.0703159 bis 13.7574797/54.0707564), Scan-Fenster **13.7548043/54.0683159 bis 13.7594797/54.0727564** (**1 Kachel im 0,01°-Raster des Repo-Werkzeugs `scripts/release/osm_street_scan.py`, 0 Kachel-Fehler**). Zählung getrennt: **4 Adressobjekte / 4 verschiedene Hausnummern** auf der Straße (8, 9, 10, 11) - die Aufzählung nennt exakt die live vorhandenen Nummern; Filter-Key `addr:street=Krösliner Straße` (mit `addr:housenumber` als Hausnummer). Keine Bereichs- oder Buchstabennotation auf dieser Straße. Im gesamten Rastergebiet **4 Adressobjekte mit Hausnummer / 4 verschiedene Hausnummern**; im Raster tragen **23 Objekte `name=`** und **3 Objekte `operator=`**. **Die Dossier-Hausnummer 5 existiert auf der Krösliner Straße in OSM NICHT** - das ist zu schreiben, nicht zu übergehen: im geprüften Fenster gibt es **kein** Objekt mit `addr:street=Krösliner Straße` und `addr:housenumber=5`; das Nummernband der Straße beginnt live bei **8** (node/7217215070 mit `addr:housenumber=8`, `addr:street=Krösliner Straße`, `addr:city=Wolgast`; ferner node/7217215072 mit `addr:housenumber=9`, node/7217215069 mit `addr:housenumber=10` und node/7217215071 mit `addr:housenumber=11`) - alle vier **ohne** `name=` und **ohne** `operator=`-Tag. Objekte mit Bezug zum Betrieb im Raster: **keines** - die Suche nach „alba“, „metall“, „schrott“ und „nord“ über die Felder name, operator, description, website, contact:website, note, contact:email, email, short_name, brand und operator:name findet **keinen** Treffer mit Betriebsbezug zur ALBA Metall Nord GmbH; **kein** Objekt des Rasters trägt einen `name=`- oder `operator=`-Wert mit Bezug zu „ALBA“. Die `name=`-Objekte im Raster sind ausschließlich **Verkehrs-, Orts- und Landschaftsobjekte** (node/10745265394 mit `name=Wolgast`, `traffic_sign=city_limit`; way/23152576, way/23152577, way/1155438694 mit `name=Krösliner Straße`; way/36873970 mit `name=Am Poppelberg`, `landuse=commercial`; way/376486668 mit `name=Greifswalder Straße`, `ref=L 262`; relation/68798 mit `operator=Land Mecklenburg-Vorpommern`; relation/148653 mit `operator=Ostseebus GmbH`), **kein** Betrieb. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=13.7548043,54.0683159,13.7594797,54.0727564 (Kachelraster, live abgerufen 31.10.2026) ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Korrektur 31.10.2026 (Korrekturvermerk zum ALBA-Fix vom 01.10.2026): die Adresslage ist nun geprüft — ohne Treffer.** Die Vorwellen-Notiz „Adressbeleg (Kröslinerstr. 5 — bislang ohne Betreiber-Beleg) offen“ bleibt **wortgleich** bestehen. Neu ist: In OSM existiert auf der Krösliner Straße **weder die Hausnummer 5** noch ein Objekt mit Betriebsbezug zur ALBA Metall Nord GmbH; die **Betreiber-Standortliste** metall.alba.info/unternehmen/standorte/ (live am 31.10.2026, HTTP 200) nennt weiterhin **„13 Treffer zu Ihrer Auswahl“** — **ohne** Wolgast; die Unter-URL https://metall.alba.info/unternehmen/standorte/wolgast liefert HTTP 404. Ein **Schließungs- oder Verlagerungsbeleg** fehlt weiterhin → `status: pruefung` und der Klärfall bleiben **unverändert**; ein Delisting ist **kein** Schließungsnachweis. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=13.7548043,54.0683159,13.7594797,54.0727564 ; https://metall.alba.info/unternehmen/standorte/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Ankauf, Verkauf und Gebühren: Betreiber-Aussagen der ALBA-Gruppe, keine standortscharfen Preise.** **ANKAUF:** „Die Spezialist*innen von ALBA nehmen **auf all unseren Plätzen** alle Arten von **Stahl- und Metallschrott zu Bestpreisen** entgegen und sorgen für deren fachgerechte Verwertung“ (metall.alba.info/ankauf/); die **Sorten** der Gruppe sind auf metall.alba.info/ankauf/sorten/ benannt (**Produktionsschrotte, Haushaltsschrott, Industrie- und Abbruchschrotte, Kabelschrotte, Mischschrotte, Vormaterial, Schwere Schere, Legierte Schrotte, Karossen, Aluminium, Kupfer, Messing, Brennschrott, Gusseisen, Moniereisen, Scherenschrott, Stahl- und Metallspäne**) — das deckt die Seed-Angabe „Stahl/NE“ inhaltlich. **Zahlenpreise werden nicht veröffentlicht.** **VERKAUF:** „Verkauf von Nutzmaterial“ ist als ALBA-Leistungsfeld benannt, **ohne** Zahlenpreise. **GEBÜHREN:** **keine** Gebühr beziffert; die Container-Bestellung läuft über **ALBAclick**, **ohne** Preisangabe. Quelle(n): https://metall.alba.info/ankauf/ ; https://metall.alba.info/ankauf/sorten/ ; https://metall.alba.info/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen und Geokodierung.** (a) Die **Dossier-Adresse Kröslinerstr. 5** ist in OSM **nicht** belegt (die Straße beginnt bei 8) — ob der Betrieb unter einer anderen Nummer geführt wird oder die Seed-Adresse falsch ist, ist **offen**; die Adresslage stützt sich ausschließlich auf den **Seed-Eintrag**, es gibt **keinen** Aggregator- oder Betreiberbeleg für diese Nummer. (b) Die Seed-Angaben **„Container, Demontage“** sind durch die Website **nicht** standortscharf belegt — die ALBA-Gruppe bietet beide Leistungen an, aber **keinem** Wolgaster Standort zugeordnet. (c) **Geokodierungsbedarf:** die Dossier-Adresse **Kröslinerstr. 5, Wolgast** ist in OSM **nicht** belegt — es existiert **kein** Objekt mit dieser Hausnummer, das als Koordinate dienen könnte; der gespeicherte Produktionspunkt **54.0528000/13.7720000** stammt aus einer groben Seed-Koordinate und ist **nicht** durch ein OSM-Objekt mit der Dossier-Hausnummer gedeckt. Die Koordinate wird hier **nicht** gesetzt (Auftrag dieser Welle: Adressen benennen, nicht selbst geokodieren). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=13.7548043,54.0683159,13.7594797,54.0727564 ; https://metall.alba.info/unternehmen/standorte/ (Abruf 31.10.2026).]
