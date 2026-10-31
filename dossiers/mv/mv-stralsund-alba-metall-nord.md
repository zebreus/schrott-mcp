---
slug: mv-stralsund-alba-metall-nord
name: ALBA Metall Nord GmbH
trader_type: schrotthaendler
state: MV
city: Stralsund
street: Dähnholmstr. 13
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

- Stahl/NE, Container (lt. schrottplaetze.org)

### Korrektur 01.10.2026 (ALBA-Fix)

- Delisted-Protokoll: Stralsund steht NICHT auf der aktuellen Betreiber-Standortliste (Vollcrawl 01.10.2026: metall.alba.info = 13 Detailseiten ohne Stralsund; alba.info/standorte ohne Treffer). Ältere Liste hatte 17 Treffer inkl. Stralsund.
- Betreiber-Suche/Register/News ohne Schließungs-/Verlagerungsbeleg → kein status geschlossen, Dossier bleibt erhalten; status aktiv → pruefung + Klärfall: Filial-Existenz und Adressbeleg (Dähnholmstr. 13 — bislang nur Aggregator-Lead schrottplaetze.org, kein Betreiber-Beleg) offen.
- website = Domain-Root (aktiv, Impressum via alba.info); keine Detailseite.

### Recherche 01.10.2026

- [Recherche 01.10.2026 / Feedback-Triage 2304: Delisting berechtigt — ALBA-Fix-Befund bestätigt (Vollcrawl 01.10.2026: metall.alba.info 13 Standorte ohne Stralsund; alba.info/standorte ohne Treffer); Dossier bleibt pruefung; Quelle(n): metall.alba.info/unternehmen/standorte/, alba.info/standorte/ (Abruf 01.10.2026)]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Dänholmstraße, Stralsund) — Straßenschrift und Hausnummer weichen vom Dossier ab.** Die OSM-Straße heißt **„Dänholmstraße“** (mit **ä**), nicht „Dähnholmstraße“ wie im Dossier/Seed; die Straßen-Objekte way/28340606 und way/46258879 tragen `name=Dänholmstraße`; way/117765084 trägt nur `landuse=industrial`. Straßen-Raster über den **gesamten** Straßenverlauf (Straßenbbox 13.1033702/54.3062660 bis 13.1071456/54.3087237), Scan-Fenster **13.1013702/54.3042660 bis 13.1091456/54.3107237** (**1 Kachel im 0,01°-Raster des Repo-Werkzeugs `scripts/release/osm_street_scan.py`, 0 Kachel-Fehler**). Zählung getrennt: **4 Adressobjekte / 4 verschiedene Hausnummern** auf der Straße (7, 9, 11, 13a) - die Aufzählung nennt exakt die live vorhandenen Nummern; Filter-Key `addr:street=Dänholmstraße` (mit `addr:housenumber` als Hausnummer). Keine Bereichsnotation auf dieser Straße; die **Buchstaben-Notation „13a“** ist live vorhanden (way/588794640) und zählt nach der Referenzzählung des Repo-Werkzeugs als **eigene** Hausnummer neben 13. Im gesamten Rastergebiet **60 Adressobjekte mit Hausnummer / 56 verschiedene Hausnummern**; im Raster tragen **156 Objekte `name=`** und **81 Objekte `operator=`**. **Die Dossier-Hausnummer 13 existiert auf der Dänholmstraße in OSM NICHT** - das ist zu schreiben, nicht zu übergehen: im geprüften Fenster gibt es **kein** Objekt mit `addr:street=Dänholmstraße` und `addr:housenumber=13`; das nächstliegende Objekt ist **way/588794640** mit `addr:housenumber=13a`, `addr:street=Dänholmstraße`, `addr:city=Stralsund`, `addr:country=DE`, `addr:postcode=18439`, `building=yes` (Objekt-BBox 13.1034626/54.3087584 bis 13.1038536/54.3088793, Mitte 13.1036575/54.3088174) **ohne** `name=` und **ohne** `operator=`. Objekte mit Bezug zum Betrieb im Raster: **keines** - die Suche nach „alba“, „metall“, „schrott“ und „nord“ über die Felder name, operator, description, website, contact:website, note, contact:email, email, short_name, brand und operator:name findet **keinen** Treffer mit Betriebsbezug zur ALBA Metall Nord GmbH; **kein** Objekt des Rasters trägt einen `name=`- oder `operator=`-Wert mit Bezug zu „ALBA“. Die industriell geprägten Objekte im Raster sind **Fremdbetriebe**: way/116845984 mit `name=Volkswerft Stralsund`, `official_name=Volkswerft Stralsund - Maritimer Industrie- und Gewerbepark`, `industrial=shipyard`, `landuse=industrial`, `website=https://www.stralsund.de/investoren/volkswerft/`, `wikidata=Q882348` sowie way/117765084 mit `landuse=industrial` (ohne `name=`). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=13.1013702,54.3042660,13.1091456,54.3107237 (Kachelraster, live abgerufen 31.10.2026) ; https://api.openstreetmap.org/api/0.6/way/588794640.json ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Korrektur 31.10.2026 (Korrekturvermerk zum ALBA-Fix vom 01.10.2026): Straßenschreibweise korrigiert, Adresslage neu bewertet.** Die Vorwellen-Notiz „Adressbeleg (Dähnholmstr. 13 — bislang nur Aggregator-Lead schrottplaetze.org, **kein** Betreiber-Beleg) offen“ bleibt **wortgleich** bestehen. Ergänzt ist: die **Straßenschreibweise** lautet nach OSM **„Dänholmstraße“** (nicht „Dähnholmstraße“) — der Dossier-Eintrag `street` bleibt dennoch **unverändert**, weil ein Schreibweisen-Wechsel ohne Beleg des Betreibers eine Frontmatter-Änderung wäre und der Slug unverändert bleiben muss; die Abweichung ist hier dokumentiert. Die **Hausnummer 13** existiert in OSM **nicht** (nur **13a**), es gibt in OSM **kein** Objekt mit Betriebsbezug zur ALBA Metall Nord, und die Betreiber-Standortliste **metall.alba.info/unternehmen/standorte/** (live am 31.10.2026, HTTP 200) nennt weiterhin **„13 Treffer zu Ihrer Auswahl“** — **ohne** Stralsund; die Unter-URL https://metall.alba.info/unternehmen/standorte/stralsund liefert HTTP 404. Ein **Schließungs- oder Verlagerungsbeleg** fehlt weiterhin → `status: pruefung` und der Klärfall bleiben **unverändert**; ein Delisting ist **kein** Schließungsnachweis. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=13.1013702,54.3042660,13.1091456,54.3107237 ; https://metall.alba.info/unternehmen/standorte/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Ankauf, Verkauf und Gebühren: Betreiber-Aussagen der ALBA-Gruppe, keine standortscharfen Preise.** **ANKAUF:** „Die Spezialist*innen von ALBA nehmen **auf all unseren Plätzen** alle Arten von **Stahl- und Metallschrott zu Bestpreisen** entgegen und sorgen für deren fachgerechte Verwertung“ (metall.alba.info/ankauf/); die **Sorten** der Gruppe sind auf metall.alba.info/ankauf/sorten/ benannt (**Produktionsschrotte, Haushaltsschrott, Industrie- und Abbruchschrotte, Kabelschrotte, Mischschrotte, Vormaterial, Schwere Schere, Legierte Schrotte, Karossen, Aluminium, Kupfer, Messing, Brennschrott, Gusseisen, Moniereisen, Scherenschrott, Stahl- und Metallspäne**) — das deckt die Seed-Angabe „Stahl/NE“ inhaltlich. **Zahlenpreise werden nicht veröffentlicht.** **VERKAUF:** „Verkauf von Nutzmaterial“ ist als ALBA-Leistungsfeld benannt, **ohne** Zahlenpreise. **GEBÜHREN:** **keine** Gebühr beziffert; die Container-Bestellung läuft über **ALBAclick**, **ohne** Preisangabe. Quelle(n): https://metall.alba.info/ankauf/ ; https://metall.alba.info/ankauf/sorten/ ; https://metall.alba.info/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen und Geokodierung.** (a) Die **entscheidende offene Frage** bleibt die Filial-Existenz: Die Betreiber-Standortliste führt **kein** Stralsund, und in OSM gibt es **kein** Objekt, das die ALBA Metall Nord an der Dänholmstraße nennt — der Aggregator-Lead (schrottplaetze.org) aus dem Seed bleibt der **einzige** Hinweis auf die Existenz. (b) Die **Hausnummer 13** fehlt in OSM (nur 13a); welche Nummer der Betrieb tatsächlich führt, ist **offen**. (c) **Geokodierungsbedarf:** die Dossier-Adresse **Dähnholmstr. 13, Stralsund** ist in OSM **nicht** belegt; das nächstliegende Objekt way/588794640 (13a) ist **kein** Ersatz für die Dossier-Adresse. Der gespeicherte Produktionspunkt **54.3112333/13.0676667** stammt aus einer groben Seed-Koordinate und ist **nicht** durch ein Objekt mit der Dossier-Hausnummer gedeckt; die Koordinate wird hier **nicht** gesetzt (Auftrag dieser Welle: Adressen benennen, nicht selbst geokodieren). Quelle(n): https://api.openstreetmap.org/api/0.6/way/588794640.json ; https://metall.alba.info/unternehmen/standorte/ (Abruf 31.10.2026).]

- [31.10.2026 (Korrekturvermerk): **Objekt-IDs und Tag-Zitate berichtigt**: way/117765084 trägt live **nur** `landuse=industrial` und **keinen** `name`-Tag; `name=Dänholmstraße` steht an **way/28340606** und **way/46258879**. Quelle(n): https://api.openstreetmap.org/api/0.6/way/117765084.json ; https://api.openstreetmap.org/api/0.6/way/28340606.json (Abruf 31.10.2026)]
