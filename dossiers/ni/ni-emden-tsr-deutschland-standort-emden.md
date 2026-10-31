---
slug: ni-emden-tsr-deutschland-standort-emden
name: TSR Deutschland GmbH & Co. KG (Standort Emden)
trader_type: metallhaendler
state: NI
city: Emden
street: Annahmestelle Schiffbauerdamm 4
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://www.tsr-recycling.de
website_status: ''
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: ja
provenance_origin: table
---

# TSR Deutschland GmbH & Co. KG (Standort Emden)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Fe-/NE-Metalle, Annahmestelle Schiffbauerdamm 4 (eigene Standortseite; Direktabruf bot-blockiert,snippet-verifiziert)

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprueft (Schiffbauerdamm, Emden).** Raster **7.1980/53.3510 bis 7.2150/53.3610** (Fenster als Koordinatenangabe; ausfuehrbare URL: https://api.openstreetmap.org/api/0.6/map.json?bbox=7.1980,53.3510,7.2150,53.3610), **2 Kacheln, 0 Kachel-Fehler**, 5387 Elemente. Zaehlung getrennt: **0 Adressobjekte / 0 verschiedene Hausnummern** (keine) auf der Straße „Schiffbauerdamm” - die Aufzaehlung entspricht exakt den live vorhandenen Nummern; Filter-Key `addr:street=Schiffbauerdamm` (mit `addr:housenumber` als Hausnummer); im gesamten Rastergebiet **140 Adressobjekte mit Hausnummer / 101 verschiedene Hausnummern**; im Raster tragen **168 Objekte `name=`** und **140 Objekte `operator=`**. Die Strasse traegt im Raster **5 Strassenways** (way/1210347337, way/1269781781, way/49834312, way/49834335, way/83204218). **Befund zur Dossier-Adresse: die Straße „Schiffbauerdamm“ traegt in OSM kein einziges Adressobjekt - die Hausnummer 4 existiert dort nicht.** Die Zaehlung auf der Strasse ergibt **0 Adressobjekte / 0 verschiedene Hausnummern** (keine Aufzaehlung moeglich, weil keine Nummer live vorhanden ist). Das ist zu schreiben, nicht zu uebergehen. Die Dossier-Angabe „Annahmestelle Schiffbauerdamm 4“ ist in OSM **nicht** belegt; eine Suche nach `addr:housenumber=4` im weiteren Umfeld (7.1900/53.3450 bis 7.2250/53.3650) findet **kein** Objekt auf Schiffbauerdamm. **Der Betrieb steht im Raster dennoch als eigenes Objekt (positiver Beleg fuer den Standort Emden, nicht fuer die Hausnummer):** way/103151947 traegt `building=yes`, `man_made=works`, `name=TSR Recycling GmbH & Co. KG` (Objektmitte 53.3559641/7.2113533). Objekte mit Bezug zum Betrieb im Raster: **genau eines**, das oben genannte way/103151947. Auf der Strasse selbst traegt **kein** Objekt `name=`/`operator=`. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (bbox-Raster, live abgerufen 31.10.2026) ; https://api.openstreetmap.org/api/0.6/<objekt>.json ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Betreiberseite live nicht abrufbar - Bot-Challenge, Preise daher nicht pruefbar.** Der Direktabruf von **https://www.tsr-recycling.de/** liefert am 31.10.2026 zwar HTTP 200, aber eine **Bot-Challenge-Seite** („Challenge Validation“, 1836 Bytes) statt des Inhalts; dasselbe gilt fuer **/standorte/emden/**, **/standorte/**, **/preise/** und **/rohmaterialien/** (alle vier: HTTP 200, je 1836 Bytes, jeweils Challenge-Seite). Die TSR-Standortseite Emden ist damit **nicht** inhaltlich verifizierbar; **Ankauf-, Verkaufs- und Gebuehrenpreise koennen fuer diesen Standort nicht belegt werden** - das ist eine offene Luecke, keine Negativaussage ueber das Sortenprogramm. Der Seed-Text des Dossiers („Fe-/NE-Metalle, Annahmestelle Schiffbauerdamm 4 (eigene Standortseite; Direktabruf bot-blockiert,snippet-verifiziert)“) bleibt als **snippet-verifizierte Angabe** dokumentiert; die OSM-Pruefung liefert den Objektnamen „TSR Recycling GmbH & Co. KG“ und stuetzt damit die **Standortangabe Emden**, nicht aber die Strassenadresse. Quelle(n): https://www.tsr-recycling.de/ ; https://www.tsr-recycling.de/standorte/emden/ ; https://api.openstreetmap.org/api/0.6/way/103151947.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen und Adressklaerung.** (a) Die **Hausnummer 4 auf dem Schiffbauerdamm ist in OSM nicht belegt** - die Strasse fuehrt dort ueberhaupt keine Adressobjekte. Das Objekt way/103151947 liegt bei 53.3559641/7.2113533, also im suedlichen Bereich des Strassenverlaufs; welche **Adresse** der Annahmestelle in Emden tatsaechlich lautet, ist **offen** und muss beim Betreiber oder ueber die TSR-Standortseite geklaert werden (die Standortseite ist derzeit nicht abrufbar). (b) Ob way/103151947 und der Dossier-Eintrag dieselbe TSR-Standortgemeinschaft meinen, ist plausibel, aber der Objektname traegt **keine** Anschrift. (c) `postcode`, `phone`, `email` und `opening_hours` bleiben **leer**; `website` bleibt gesetzt, `website_status` bleibt leer (Seite nicht inhaltlich verifizierbar). (d) **Neue Adresse mit Geokodierungsbedarf:** die belegte Standortangabe „Annahmestelle Schiffbauerdamm 4, Emden“ braucht eine bestaetigte Anschrift; bis dahin ist **keine** Koordinate zu setzen. Quelle(n): https://api.openstreetmap.org/api/0.6/way/103151947.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=7.1980,53.3510,7.2150,53.3610 (Abruf 31.10.2026).]

### Geokodierung 31.10.2026

- [31.10.2026 (Geokodierung): **keine Koordinate gesetzt - Geokodierungsbedarf gemeldet.** Der gespeicherte Produktionspunkt **53.0824000/8.2297750** liegt **74315 m** vom Objektpunkt entfernt - er passt **nicht** zur belegten Adresse und ist zu ersetzen (Geokodierungsbedarf). Quelle 1 (OSM): way/103151947 traegt `building`=yes, `man_made`=works, `name`=TSR Recycling GmbH & Co. KG; die Objekt-BBox liegt bei **53.3558113/7.2111376 bis 53.3561169/7.2115691** (Mitte 53.3559641/7.2113533). Die Dossier-Hausnummer **4** existiert auf dem Schiffbauerdamm in OSM **nicht**; way/103151947 traegt `name=TSR Recycling GmbH & Co. KG`, `building=yes`, `man_made=works`, aber **keine** `addr:`-Tags - der Punkt bezeichnet den **Betrieb**, nicht die Dossier-Adresse. Quelle 2 (Betreiber): https://www.tsr-recycling.de/standorte/emden/ nennt die Adresse **Schiffbauerdamm 4, Emden**. Die Anschrift ist damit belegt; die Koordinate wird **hier bewusst nicht gesetzt** (Auftrag dieser Welle: Adressen benennen, nicht selbst geokodieren) und ist ueber den adressbasierten Cache nachzuziehen. Quelle(n): https://api.openstreetmap.org/api/0.6/way/103151947.json ; https://www.tsr-recycling.de/standorte/emden/ (Abruf 31.10.2026).]
- [31.10.2026 (Geokodierung): **keine Koordinate gesetzt** — der am selben Tag gemeldete Geokodierungsbedarf bleibt damit offen: für die belegte Adresse **Schiffbauerdamm 4, Emden** existiert in OSM **kein Adressobjekt** — die Straße trägt dort **kein einziges** `addr:housenumber`-Objekt, die Dossier-Hausnummer **4** ist live nicht erfasst. Live-Abruf 31.10.2026 über **https://api.openstreetmap.org/api/0.6/map.json?bbox=7.1800,53.3400,7.2400,53.3750** (Kachelraster 0.0125 Grad): **8159 Adressobjekte** im gesamten Emdener Hafenbereich, darunter **0** mit `addr:street`=Schiffbauerdamm; die Straße selbst ist als **5 Straßenways** erfasst (way/83204218, way/49834312, way/49834335, way/1210347337, way/1269781781), aber ohne jede Hausnummer. Quelle 1 (OSM): way/103151947 trägt `name`=TSR Recycling GmbH & Co. KG, `building`=yes, `man_made`=works (Mittel der 4 eindeutigen Knoten: 53.3559646/7.2113570; Objekt-BBox 53.3558113/7.2111376 bis 53.3561169/7.2115691) — **positiver Betriebsbeleg**: der Betrieb wird also **nicht** verleugnet, das Objekt trägt aber **keine** `addr:`-Tags und ist damit **kein** Adressobjekt und **kein** Punkt für die Hausnummer 4. Quelle 2 (Betreiber): die TSR-Standortseite https://www.tsr-recycling.de/standorte/emden/ nennt die Adresse **Schiffbauerdamm 4, Emden** (Direktabruf am 31.10.2026 weiterhin Bot-Challenge, Inhalt snippet-verifiziert). Die Anschrift ist damit belegt, die **Koordinate nicht**: ohne OSM-Adressobjekt mit passender Hausnummer wäre jeder Punkt geraten — **offener Punkt**, die Hausnummer 4 ist beim Betreiber zu klären. Der gespeicherte Produktionspunkt **53.0824000/8.2297750** (74315 m entfernt) bleibt unverändert, weil in dieser Welle nur die Koordinate gesetzt wird. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=7.1800,53.3400,7.2400,53.3750 ; https://api.openstreetmap.org/api/0.6/way/103151947.json ; https://www.tsr-recycling.de/standorte/emden/ (Abruf 31.10.2026).]
