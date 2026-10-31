---
slug: hb-stuhr-edwins-autorecycling
name: Edwins Autorecycling
trader_type: autoverwertung
state: NI
city: Stuhr
street: Gutenbergstr. 10
postcode: '28816'
phone: 0421/2008877
email: ''
opening_hours: ''
website: http://edwinsautorecycling.de
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Neue Funde (nicht in den Tabellen oben)
provenance_ankauf_raw: unklar (nur Altautos anzunehmen)
provenance_origin: table
---

# Edwins Autorecycling

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- FLAG Autoverwertung
- Adresse: Stuhr (Umland), Gutenbergstr. 10, 28816 [Website-Recherche website: services: Autorecycling, Autoteile, Altautos; customer_types: privat; notes: Schwacher Beleg: nur einseitige Homepage mit Adressangabe, kein formelles Impressum; Mobil 0175/3374205.]

### Recherche 01.10.2026

- [Recherche/Korrektur 01.10.2026: state HB → NI korrigiert (Stuhr, 28816 liegt in Niedersachsen, Lkr. Diepholz; Slug unverändert); Adresse Gutenbergstr. 10, 28816 Stuhr und Tel. 0421/2008877 bestätigt, Mobil 0175/3374205 aus Betreiber-Homepage nur Timeline (Einzelbeleg, Einseiten-Visitenkarte ohne Impressum — Restunsicherheit offengelegt); Öffnungszeiten nirgends genannt → Feld bleibt leer; Quelle(n): http://edwinsautorecycling.de/]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprueft (Gutenbergstraße, Stuhr).** Raster **8.7570/53.0160 bis 8.7760/53.0270** (Fenster als Koordinatenangabe; ausfuehrbare URL: https://api.openstreetmap.org/api/0.6/map.json?bbox=8.7570,53.0160,8.7760,53.0270), **4 Kacheln, 0 Kachel-Fehler**, 4877 Elemente. Zaehlung getrennt: **2 Adressobjekte / 2 verschiedene Hausnummern** (2, 23) auf der Straße „Gutenbergstraße” - die Aufzaehlung entspricht exakt den live vorhandenen Nummern; Filter-Key `addr:street=Gutenbergstraße` (mit `addr:housenumber` als Hausnummer); im gesamten Rastergebiet **134 Adressobjekte mit Hausnummer / 100 verschiedene Hausnummern**; im Raster tragen **125 Objekte `name=`** und **35 Objekte `operator=`**. Die Strasse traegt im Raster **1 Strassenway** (way/55682693). **Die Dossier-Hausnummer 10 existiert auf der Gutenbergstraße in OSM NICHT.** Die live vorhandenen Nummern der Strasse sind **nur 2 und 23** (zwei Objekte: node/7600882547 mit `addr:housenumber=2` und way/92416870 mit `addr:housenumber=23`). Das ist zu schreiben, nicht zu uebergehen: die Dossier-Adresse **Gutenbergstr. 10** ist in OSM **nicht** belegt; eine Suche nach `addr:housenumber=10` im erweiterten Fenster findet in Stuhr nur way/92412462 mit `addr:street=Werner-von-Siemens-Straße`, `addr:housenumber=10`, `name=Strohback + Krey` - eine **andere** Strasse. Objekte mit Bezug zum Betrieb im Raster: **keines** - die Suche nach „Edwin“ und „Autorecycling“ ueber die Felder name, operator, description, website, contact:website, note, contact:email, email, short_name und brand findet **keinen** Treffer; das gilt fuer den gesamten Raster, nicht nur fuer die Strasse. Auf der Strasse selbst traegt **1 Objekt** `name=` - ein **Fremdbetrieb**: node/7600882547 mit `name=Aral ADS`, `brand=Aral`, `amenity=vending_machine`. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (bbox-Raster, live abgerufen 31.10.2026) ; https://api.openstreetmap.org/api/0.6/<objekt>.json ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Betreiberseite live, aber eine Einseiten-Visitenkarte - Ankaufspreise nicht belegbar.** Die Seite **http://edwinsautorecycling.de/** antwortet am 31.10.2026 mit HTTP 200, ist aber ein **minimales HTML-Dokument** (860 Bytes) ohne Impressum, ohne Angebotsseiten und ohne Preise. Ihr vollstaendiger Textinhalt lautet: „Edwins Autorecycling / Gutenbergstrasse 10 / 28816 Stuhr / Telefon: 0421/2008877 / Mobil: 0175/3374205“; die `<meta name="keywords">`-Zeile nennt „Autoteile, Autoverwertung; Autorecycling, Altmetall, Altautos, Teile, “. **ANKAUF:** „Altmetall“ und „Altautos“ sind damit als Angebotsfelder belegt, **Zahlenpreise gibt es nicht**. **VERKAUF:** „Autoteile“ sind als Angebot belegt - **ohne** Preise. **GEBUEHREN:** **keine** Gebuehr genannt. Quelle(n): http://edwinsautorecycling.de/ (Live-Fetch 31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen und Adressklaerung.** (a) **Die Dossier-Adresse Gutenbergstr. 10 ist in OSM nicht belegt** - die Strasse traegt in OSM nur die Nummern 2 und 23; die Nummer 10 kommt in Stuhr auf der **Werner-von-Siemens-Straße** vor (way/92412462, `name=Strohback + Krey`). **Ob der Betrieb unter einer abweichenden Nummer erfasst ist, ist offen**; die Angabe „Gutenbergstrasse 10“ stammt allein von der Betreiberseite. (b) Die **Betriebsadresse traegt in OSM kein Objekt mit `name=`/`operator=`**. (c) `phone` ist belegt (0421/2008877, Betreiber-Einzelbeleg), `email` und `opening_hours` bleiben **leer**; `status` bleibt `pruefung`, `website_status: aktiv` bleibt (Seite live, HTTP 200). (d) **Geokodierungsbedarf:** die Dossier-Adresse **Gutenbergstr. 10, 28816 Stuhr** braucht eine Geokodierung; die Koordinate wird hier **nicht** gesetzt. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=8.7570,53.0160,8.7760,53.0270 ; http://edwinsautorecycling.de/ (Abruf 31.10.2026).]

### Geokodierung 31.10.2026

- [31.10.2026 (Geokodierung): **keine Koordinate gesetzt - Geokodierungsbedarf gemeldet.** Der gespeicherte Produktionspunkt **53.0215855/8.7662873** liegt **366 m** vom Objektpunkt entfernt - er passt **nicht** zur belegten Adresse und ist zu ersetzen (Geokodierungsbedarf). Quelle 1 (OSM): way/92416870 traegt `addr:street`=Gutenbergstraße, `addr:housenumber`=23, `building`=yes; die Objekt-BBox liegt bei **53.0232449/8.7614307 bis 53.0234603/8.7619013** (Mitte 53.0233526/8.7616660). Die Dossier-Hausnummer **10** existiert auf der Gutenbergstraße in OSM **nicht** (live: nur 2 und 23); way/92416870 traegt `addr:housenumber=23` und ist damit **nicht** die Dossier-Adresse - der Punkt ist **kein** Ersatz fuer die Dossier-Adresse. Quelle 2 (Betreiber): http://edwinsautorecycling.de/ nennt die Adresse **Gutenbergstr. 10, 28816 Stuhr**. Die Anschrift ist damit belegt; die Koordinate wird **hier bewusst nicht gesetzt** (Auftrag dieser Welle: Adressen benennen, nicht selbst geokodieren) und ist ueber den adressbasierten Cache nachzuziehen. Quelle(n): https://api.openstreetmap.org/api/0.6/way/92416870.json ; http://edwinsautorecycling.de/ (Abruf 31.10.2026).]
