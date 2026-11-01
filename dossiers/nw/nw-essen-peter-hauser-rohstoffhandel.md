---
slug: nw-essen-peter-hauser-rohstoffhandel
name: Peter Häuser Rohstoffhandel
trader_type: sonstige
state: NW
city: Essen
street: Ripshorsterstr. 370
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: http://altmetall-haeuser.de
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'geo'
provenance_section: Neu, Ankauf unklar (nur Verzeichnis-/Profilbeleg)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Peter Häuser Rohstoffhandel

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Owner-Korrektur 27.10.2026: GEO-Rücknahme

- [Owner-Korrektur 27.10.2026: Der read-only Produktionspunkt 51.450284375, 7.015053125 wird von Nominatim als Freiheit, Essen 45128 aufgelöst. Die Betreiber-Startseite für den Schrott- und Metallhandel Häuser nennt dagegen ausdrücklich das „Betriebsgelände“ Ripshorsterstr. 370, 45357 Essen, lädt zum Besuch ein und belegt Schrott-/Metallankauf während der Öffnungszeiten; das Impressum nennt Peter Häuser als Betreiber. Die abweichende Einbleckstr. 3, Oberhausen, ist auf derselben Website als Impressums-/Rechtsanschrift und nicht als Betriebsgelände bezeichnet. Der zentrale Produktionspunkt ist daher nicht der für genau diesen Händler benannte Ankaufstandort. `clear_fields: 'geo'` zieht das alte Paar beim nächsten regulären Seed ein; keine Ersatzkoordinate gesetzt. Die abweichende Rechtsanschrift bleibt dokumentiert und wird nicht in die Frontmatter übertragen. Quellen/Abruf 27.10.2026: http://altmetall-haeuser.de/startseite.html ; http://altmetall-haeuser.de/impressum.html ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; read-only `public.db`; `docs/trader-seed.md`.]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Website altmetall-haeuser.de live (Metallhandel + Impressum): Peter Häuser Rohstoffhandel, Anfahrt Ripshorsterstr. 370, 45357 Essen, Tel. 0208 6206845 / 0171 9264875, info@altmetall-haeuser.de, Öffnungszeiten Mo–Fr 08–17 Uhr, Sa nach Absprache, Schrott-/Metallhandel, Container, Entrümpelung, Kleinabbrüche; Impressum-Sitz abweichend Einbleckstr. 3, 46117 Oberhausen (Einzelunternehmen, Steuernr., kein HRB) — Sitz-Divergenz ungeklärt, City Essen unverändert; Adresse/Tel/Mail/Zeiten EINZELBELEG (kein HR-Eintrag, keine Owner-Ausnahme, kein Zweitbeleg) nur Timeline, Frontmatter bis auf website/website_status unangetastet; Quelle(n): http://altmetall-haeuser.de/metallhandel.html + http://altmetall-haeuser.de/impressum.html (eine Quelle)]

### Recherche 01.10.2026 (Triage Feedback 2565)

- [Recherche 01.10.2026: Feedback 2565 teilweise berechtigt — Impressum (Abruf 01.10.2026: Einbleckstr. 3, 46117 Oberhausen, Tel. 0208-6206845 / 0171-9264875) bestätigt Sitz/Betrieb-Divergenz → PLZ/Tel/Mail/Zeiten weiter ohne Zweitbeleg (kein Fill); street-Fix (Firmen-/Ortspräfix entfernt → „Ripshorsterstr. 370", operator-belegte Anfahrt) als belegter Korrektur-Fill mit offener Dokumentation; city bleibt Essen; Quelle(n): http://altmetall-haeuser.de/impressum.html]

### Importiert (Seed-Stand 2026-09-30)

- Ripshorsterstr. 370 (s. Merge-Notiz C4)
- urspr. Website-Angabe: keine gefunden

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Ripshorster Straße, Essen/Oberhausen).** Kachelraster über den Straßenverlauf (Straßenbbox 6.9000/51.4800 bis 6.9300/51.4900, **3 Kacheln, 0 Kachel-Fehler**; Objekt-BBox der Straße 51.4835952/6.9000612 bis 51.4907344/6.9263931 - der nördlichste Straßenknoten liegt damit rund 80 m außerhalb der Fensternordkante). Zählung getrennt: **91 Adressobjekte / 91 verschiedene Hausnummern** (126, 130, 136, 136a, 136b, 138, 140, 142, 148, 150, 156, 158, 160, 162, 164, 172, 174, 182, 184, 186, 188, 190, 192, 248, 249, 255, 280, 282, 284, 285, 288, 289, 290, 291, 292, 293, 294, 296, 298, 300, 301, 302, 303, 304, 306, 307, 308, 309, 310, 311, 312, 312a, 313, 314, 315, 316, 318, 318a, 319, 320, 321, 322, 323, 328, 330, 331, 333, 334, 335, 336, 337, 338, 340, 342, 344, 356, 364, 366, 367, 368, 375, 377, 379, 380, 386, 460, 460a, 464, 470, 479, 495) auf der Straße „Ripshorster Straße“, im gesamten Rastergebiet **1336 Adressobjekte mit Hausnummer / 373 verschiedene Hausnummern**; im Raster tragen 380 Objekte `name=` und 248 Objekte `operator=`. **Die Dossier-Hausnummer 370 existiert in OSM auf der Ripshorster Straße nicht** - die Nachbarnummern 368 (way/198663926) und 375 (way/198663912) sowie 379 (way/198663921) und 380 (node/13096393837) sind erfasst, **370** nicht. Die Straße selbst ist way/4244539, way/25450467, way/25450471, way/28194870, way/31264469, way/31264470, way/31989890, way/150239970, way/215066633, way/808403170, way/986320830, way/986320831, way/1186484813, way/4244547, way/4244548, way/161417187, way/999689902, way/999689903, way/26142420, way/26607098, way/27667555, way/27667557, way/28194872, way/305039158, way/305039159, way/406643363 und way/1035163011 (drei davon mit `postal_code=46117`, der Rest `postal_code=45357` - die Straße läuft über die Stadtgrenze). Weitere Objekte mit Bezug zum Betrieb im Raster: **keines** - die Namenssuche nach „Häuser“, „Schrott“, „Metall“, „Recycling“ und „Rohstoff“ über die Felder name, operator, description, website und brand findet **keinen** Treffer mit Betriebsbezug; die beiden benannten Betriebe auf der Straße sind **andere**: way/275469470 mit `name=Sprenker + Gravius GmbH & Co. KG (SG)` (`addr:housenumber=366`) und way/1426254970 mit `name=Jakobs.eu`, `operator=Sprenker + Gravius GmbH & Co. KG (SG)`, `addr:housenumber=364`, `website=https://jakobs.eu/`. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=6.9000,51.4800,6.9300,51.4900 ; https://api.openstreetmap.org/api/0.6/way/198663926.json ; https://api.openstreetmap.org/api/0.6/way/275469470.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Die Impressumsanschrift ist in OSM belegt - und das ist eine andere Straße.** Im selben Raster liegen **10 Adressobjekte / 10 verschiedene Hausnummern** (1, 3, 5, 7, 9, 11, 13, 15, 17, 19) auf der Straße „Einbleckstraße“. **Die Hausnummer 3 existiert**: way/226838734 mit `addr:housenumber=3`, `addr:street=Einbleckstraße`, `addr:city=Oberhausen`, `addr:postcode=46117`, `building=house` - **ohne** `name=`- und **ohne** `operator=`-Tag (Punktlage 51.4895162/6.9112727). Das entspricht der Impressumsanschrift des Betriebs („Einbleckstr. 3, 46117 Oberhausen“), während die Betreiberseite als **Betriebsgelände** die Ripshorsterstr. 370 in 45357 Essen nennt; die im Anfahrts-Kartenembedder der Startseite hinterlegte Punktlage des Betriebsgeländes (`ll=51.488681,6.919324`) und das Objekt way/226838734 liegen rund **565 m** auseinander. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=6.9000,51.4800,6.9300,51.4900 ; https://api.openstreetmap.org/api/0.6/way/226838734.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Betreiber-Primärquelle live - Preise und Gebühren getrennt, keine Zahlenwerte.** Startseite, Schrotthandel-Seite, Metallhandel-Seite und Impressum wurden am 31.10.2026 erneut live abgerufen (alle HTTP 200). Die Startseite nennt unverändert das „Betriebsgelände“ **Ripshorsterstr. 370, 45357 Essen**, „Ankauf von Schrott und Metall während der Öffnungszeiten“, die Öffnungszeiten **Mo.–Fr. 08–17 Uhr, Sa. nach Absprache**, die Telefonnummern **0208 / 620 68 45** und **0171 / 926 48 75**, `info@altmetall-haeuser.de` und das Angebot eines „unverbindlichen Angebots“; das Impressum nennt **Peter Häuser, Rohstoffhandel, Einbleckstr. 3, 46117 Oberhausen**, Steuernummer 110 / 5056 / 1026, Berufsbezeichnung „Rohstoffhandel“ und die Berufshaftpflicht der R+V Versicherungsgruppe. Die eingebettete Anfahrtskarte der Startseite verweist mit `ll=51.488681,6.919324` auf den Kartenausschnitt - das ist der **Kartenmittelpunkt der Einbettung**, **keine** geokodierte Adresse. **ANKAUF:** **keine** Ankaufspreisliste und **keine** Sorten- oder Mengenpreise; die Seiten `schrotthandel.html` und `metallhandel.html` bestehen ausschließlich aus **Bildgalerien** ohne jeden Zahlenwert. **VERKAUF:** **keine** Preise und **keine** Verkaufsliste. **GEBÜHREN:** **keine** Entsorgungsgebühr, **keine** Abholpauschale, **keine** Containergebühr; genannt wird nur das „unverbindliche Angebot“. Kommunale Tarife, Markt- und Börsenpreise sind **nicht** belegt. Quelle(n): http://altmetall-haeuser.de/startseite.html ; http://altmetall-haeuser.de/schrotthandel.html ; http://altmetall-haeuser.de/metallhandel.html ; http://altmetall-haeuser.de/impressum.html (HTTP 200, Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Punkte und Geokodierungsbedarf.** Die Beleglage bleibt eine **Betreiber-Einzelquelle**: die Website belegt die Betriebsstätte, aber weder ein Impressum-Registereintrag (Einzelunternehmen, nur Steuernummer, **kein** HRB) noch eine zweite unabhängige Quelle. `status: pruefung`, `website` (http://altmetall-haeuser.de) und `website_status: aktiv` bleiben unverändert; `phone`, `email` und `opening_hours` bleiben **leer** (nur Betreiber-Einzelbeleg), die Sitz-Divergenz Einbleckstr. 3 (Oberhausen) gegenüber Betriebsgelände Ripshorsterstr. 370 (Essen) bleibt und wird **nicht** aufgelöst. **Geokodierungsbedarf:** der read-only Produktionsabruf am 31.10.2026 zeigt `lat`/`lon` = **NULL** (`clear_fields: geo` greift); die Betriebsstätte ist in OSM **nicht** als Hausnummer 370 erfasst - als Bezugspunkte bieten sich way/198663926 (Ripshorster Straße 368) und node/13096393837 (Ripshorster Straße 380) an, die um den Kartenmittelpunkt der Anfahrtskarte liegen; eine Geokodierung erfolgt **nicht** in dieser Welle. Quelle(n): https://api.openstreetmap.org/api/0.6/way/198663926.json ; https://api.openstreetmap.org/api/0.6/way/226838734.json ; read-only `public.db` (31.10.2026).]

### Geokodierung 01.11.2026

- [Geokodierung 01.11.2026: **Klärfall - keine Koordinate gesetzt, `clear_fields: 'geo'` bleibt bestehen.** Die
  Dossier-Hausnummer **370** existiert auf der Ripshorster Straße in OSM **nicht**: das am 01.11.2026 nachgeladene
  Kachelraster 6.8950/51.4740 bis 6.9450/51.5000 (**6 Kacheln, 0 Kachel-Fehler**) erfasst die Straße
  Ripshorster Straße mit **100 Adressobjekten / 93 verschiedenen Hausnummern** (u. a. 356, 364, 366, 367, 368, 375,
  377, 379, 380, 386, 460, 460a, 464, 470, 479, 495) - die **Nachbarnummern 368 (way/198663926) und 375
  (way/198663912) sind erfasst, 370 nicht**; eine Suche über alle Adressobjekte des Rasters nach der Hausnummer 370
  (sowie 370a/370b) ergibt **null Treffer**. Die Aussage vom 31.10.2026, dass die Hausnummer 370 auf der Straße
  fehlt, ist damit **unverändert bestätigt und diesmal mit einem weiteren, nach Westen erweiterten Raster
  abgesichert**. **Die Betriebsstätte lässt sich damit nicht auf ein OSM-Objekt abbilden.** Quelle 1 (Betreiber):
  die Startseite des Betriebs (live abgerufen am 01.11.2026, HTTP 200) nennt unverändert das **„Betriebsgelände“
  Ripshorsterstr. 370, 45357 Essen** mit „Ankauf von Schrott und Metall während der Öffnungszeiten“, die Öffnungszeiten
  Mo.–Fr. 08–17 Uhr, Sa. nach Absprache, die Telefonnummern 0208 620 68 45 und 0171 926 48 75 sowie
  `info@altmetall-haeuser.de`; das Impressum (live, HTTP 200) nennt **Peter Häuser, Rohstoffhandel, Einbleckstr. 3,
  46117 Oberhausen**, Steuernummer 110/5056/1026, Berufsbezeichnung „Rohstoffhandel“ und die Berufshaftpflicht der
  R+V Versicherungsgruppe. Quelle 2 (OSM): das Kachelraster sowie way/226838734 (`addr:housenumber`=3,
  `addr:street`=Einbleckstraße, `addr:city`=Oberhausen, `addr:postcode`=46117, `building`=house, ohne `name` und ohne
  `operator`; Mittelwert der eindeutigen Knoten 51.4895061/6.9112873) - die **Impressumsanschrift** ist damit
  zwar in OSM belegt, sie liegt aber rund **564 m** vom Mittelpunkt der eingebetteten Anfahrtskarte des
  Betriebsgeländes (`ll=51.488681,6.919324`) und ist eine **andere Straße in einer anderen Stadt**. Die
  Sitz-/Betriebsdivergenz (Betriebsgelände Essen vs. Rechtsanschrift Oberhausen) bleibt damit **ungelöst**; es wird
  **keine Ersatzkoordinate** aus einem Nachbarobjekt gesetzt. Der read-only Produktionsabruf am 01.11.2026 zeigt
  `lat`/`lon` = **NULL** (`clear_fields: 'geo'` greift), es ist also kein Altpunkt zu ersetzen. Die bereits
  dokumentierte Owner-Entscheidung (kein Ersatzpunkt bei nachweislich abweichendem Altpunkt) bleibt damit
  wirksam. Quelle(n): http://altmetall-haeuser.de/startseite.html ;
  http://altmetall-haeuser.de/impressum.html ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=6.8950,51.4740,6.9450,51.5000 ;
  https://api.openstreetmap.org/api/0.6/way/226838734/full.json ; read-only `public.db` (01.11.2026).]
