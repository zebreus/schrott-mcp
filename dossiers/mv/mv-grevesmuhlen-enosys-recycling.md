---
slug: mv-grevesmuhlen-enosys-recycling
name: enosys Recycling
trader_type: sonstige
state: MV
city: Grevesmühlen
street: 'Am Baarssee 3'
postcode: '23936'
phone: '03881-7590600'
email: ''
opening_hours: 'Mo-Fr 8:00-16:00'
website: https://enosys-recycling.de
website_status: ''
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Neue Betriebe, Website verifiziert, Ankauf ja
provenance_ankauf_raw: ja
provenance_origin: table
---

# enosys Recycling

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Entsorgungsfachbetrieb (EfbV), Eisen/NE/Kabel, Tagesbestpreise, bar/Gutschrift, Container

### Recherche 01.10.2026

- [Recherche 01.10.2026: Fill Betreiber-Primärquelle (enosys Recycling, HRB 12601): Am Baarssee 3, 23936 Grevesmühlen, Tel. +49 3881-7590600, Mo–Fr 08:00–16:00 (Mär–Sep Do bis 18:00 Schrottplatz, Winter n. V.); E-Mail divergent (info@enosys.de vs. info@enosys-recycling.de) → leer; Typ sonstige bleibt (Option wertstoffhaendler dokumentiert — Schrottannahme + Container + Filterstaubaufbereitung); Quelle(n): enosys-recycling.de/kontakt/, /impressum/ (Feedback 2335)]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Am Baarssee, Grevesmühlen) — die Straße trägt in OSM keine einzige Adressobjekt-Adresse.** Straßen-Raster über den **gesamten** Straßenverlauf (Straßenbbox 11.1573897/53.8670672 bis 11.1620144/53.8701235), Scan-Fenster **11.1553897/53.8650672 bis 11.1640144/53.8721235** (**1 Kachel im 0,01°-Raster des Repo-Werkzeugs `scripts/release/osm_street_scan.py`, 0 Kachel-Fehler**). Zählung getrennt: **0 Adressobjekte / 0 verschiedene Hausnummern** auf der Straße - die Aufzählung ist damit **leer**, weil live **kein** Objekt mit `addr:street=Am Baarssee` und `addr:housenumber` existiert; Filter-Key `addr:street=Am Baarssee` (mit `addr:housenumber` als Hausnummer). Die Straße ist in OSM zwar als **Verkehrsweg** erfasst (way/127047502, way/208428702 und way/208428703 mit `highway=unclassified` und `name=Am Baarssee`), aber **ohne** Adressobjekte. Im gesamten Rastergebiet **0 Adressobjekte mit Hausnummer / 0 verschiedene Hausnummern**; im Raster tragen **13 Objekte `name=`** und **7 Objekte `operator=`** — die `name=`-Objekte sind ausschließlich Verkehrs- und Stadtgrenz-Objekte (node/11390013716 mit `name=Grevesmühlen`, `traffic_sign=city_limit`; way/315960009 mit `name=Lübecker Chaussee`, `ref=B 105`; relation/63309 mit `name=Bundesstraße 105`; relation/1267768 mit `name=Bus 335: Grevesmühlen => Lübeck`, `operator=NAHBUS Nordwestmecklenburg GmbH`; relation/2910741 mit `operator=E.DIS Netz`), **kein** Objekt mit Bezug zum Betrieb enosys Recycling. Objekte mit Bezug zum Betrieb im Raster: **keines** - die Suche nach „enosys“, „recycling“, „schrott“, „metall“ und „container“ über die Felder name, operator, description, website, contact:website, note, contact:email, email, short_name, brand und operator:name findet **keinen** Treffer mit Betriebsbezug; im Raster existiert **überhaupt kein** Objekt mit `landuse=`, `industrial=`, `man_made=`, `craft=`, `shop=`, `office=` oder `building=` außer **way/361199990** (`building=yes`) und **way/361199991** (`landuse=farmland`, `produce=flowers`, `fee=yes`, `self_service=yes`) — beides **ohne** Adressbezug. **Die Dossier-Hausnummer 3 existiert auf der Am Baarssee in OSM nicht** - das ist zu schreiben, nicht zu übergehen. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=11.1553897,53.8650672,11.1640144,53.8721235 (Kachelraster, live abgerufen 31.10.2026) ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Betreiber-Primärquelle bestätigt die Dossier-Adresse wörtlich — inklusive Register und Standort-Hinweisen.** Das **Impressum** nennt **„Am Baarssee 3 / 23936 Grevesmühlen / Handelsregister: HRB 12601 / Telefon: +49 3881-7590600“**; die **Kontaktseite** nennt dieselbe Anschrift **„Am Baarssee 3, 23936 Grevesmühlen“** und die Zeiten **„Mo-Fr. 08:00 - 16:00 Uhr / Do 08:00 - 18:00 Uhr (Schrottplatz)“** — die Vorwellen-Angabe „Mär–Sep Do bis 18:00 Schrottplatz, Winter n. V.“ ist damit in der Kernzeit (Mo–Fr 08:00–16:00) **bestätigt**; die **Zusatzangabe zum Schrottplatz-Donnerstag** ist auf der Seite **ohne Jahreszeit** genannt. Die Startseite belegt den Geschäftszweig wörtlich: **„Metallankauf von enosys Recycling: Zu tagesaktuellen Preisen kaufen wir: Eisen, Kupfer, Messing, Aluminium, Blei, Zink, und Kabel.“** Quelle(n): https://enosys-recycling.de/impressum/ ; https://enosys-recycling.de/kontakt/ ; https://enosys-recycling.de/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Ankauf, Verkauf und Gebühren getrennt.** **ANKAUF:** die **Sortenliste** ist betreiberseitig belegt — **Eisen, Kupfer, Messing, Aluminium, Blei, Zink und Kabel** „zu tagesaktuellen Preisen“; die Startseite beschreibt zusätzlich die **Filterstaub-Aufbereitung** („Filterstaub mit Alumium, Kupfer, Messing oder anderen Metallen“ — Schreibweise „Alumium“ **so im Original**) und nennt als Sonderfraktionen „Filterstäube, feinkörnige Rückstände und technische Granulate“. **Zahlenpreise werden nicht veröffentlicht** — „tagesaktuelle Preise“ ohne Betrag. **VERKAUF:** die Website beschreibt **keinen Verkauf** von Nutzmaterial; die Leistungsübersicht nennt **Entsorgung, Containerdienst und Aufbereitung** — Ankauf/Entsorgung, kein Verkauf. **GEBÜHREN:** der **Containerdienst** ist die zentrale Gebührenleistung, aber **ohne Preise**: „Wir liefern Container in **Größen von 3 m³ bis 30 m³**“, „Unser Containerdienst ist im Umkreis von **bis zu 70 Kilometern** rund um Grevesmühlen für Sie im Einsatz“ und „Wir stellen Ihnen schnell und zuverlässig den passenden Container … direkt zu Ihrem Aufstellstandort“; die Anfrage läuft über das Formular („Stellen Sie Ihre Anfrage“) — **kein** Mietpreis, **keine** Transportpauschale und **keine** Mindestmengenentschädigung genannt. Quelle(n): https://enosys-recycling.de/ ; https://enosys-recycling.de/leistungen/ ; https://enosys-recycling.de/anfrage/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen und Geokodierung.** (a) Die **Dossier-Adresse Am Baarssee 3, 23936 Grevesmühlen** ist in OSM **nicht** belegt (die Straße existiert, aber ohne Adressobjekte); die Beleglage für die Adresse bleibt damit die **Betreiber-Quelle** (Impressum + Kontaktseite) — die Aussage der Vorwelle bleibt gültig. (b) Die **E-Mail** bleibt **leer**: die Kontaktseite nennt **nur** die Telefonnummer, das Impressum **keine** E-Mail — die Vorwellen-Notiz zur Divergenz (info@enosys.de vs. info@enosys-recycling.de) bleibt gültig und wird hier **nicht** aufgelöst. (c) Die Seed-Angaben **„bar/Gutschrift“** und **„Tagesbestpreise“** sind durch die Website **nicht** gedeckt: die Seite nennt „Zu tagesaktuellen Preisen“ (ohne „best“), aber **keine** Auszahlungsart; die Vorwellen-Notiz zur Seed-Provenienz bleibt unverändert bestehen. (d) **Geokodierungsbedarf:** die Dossier-Adresse **Am Baarssee 3, 23936 Grevesmühlen** braucht eine Geokodierung; in OSM existiert **kein** Objekt mit dieser Hausnummer, das als Koordinate dienen könnte. Der gespeicherte Produktionspunkt **53.8634000/11.1916000** stammt aus einer groben Seed-Koordinate und ist **nicht** durch ein OSM-Objekt gedeckt; die Koordinate wird hier **nicht** gesetzt (Auftrag dieser Welle: Adressen benennen, nicht selbst geokodieren). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=11.1553897,53.8650672,11.1640144,53.8721235 ; https://enosys-recycling.de/impressum/ (Abruf 31.10.2026).]
- [31.10.2026 (Korrekturvermerk): **Erreichbarkeit der zitierten Quellen geprüft**: `enosys-recycling.de/leistungen/`; `enosys-recycling.de/anfrage/` — die genannte(n) Adresse(n) war(en) am 31.10.2026 **nicht abrufbar** (TCP-Timeout bzw. HTTP 404). Sie wird deshalb **nicht** als Beleg geführt; die übrigen Quellenangaben dieses Dossiers bleiben unverändert und sind live geprüft.]
