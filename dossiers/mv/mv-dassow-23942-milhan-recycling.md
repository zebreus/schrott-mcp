---
slug: mv-dassow-23942-milhan-recycling
name: Milhan Recycling
trader_type: sonstige
state: MV
city: Dassow 23942
street: Dassower Str. 5
postcode: 23942
phone: 038826 88360
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Milhan Recycling

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Eisen
- Adresse: Dassow 23942

### Recherche 01.10.2026

- [Recherche 01.10.2026: keine Betreiber-Website, kein HR-Beleg; nur Verzeichnis-Leads (kein Beleg); Quelle(n): schrottplatz.org, misterwhat.de, adressennet.de]
- Lead-Fakten (unsicher, NICHT in Frontmatter): Milhan Recycling, Inh. Torsten Milhan, Dassower Str. 5, 23942 Dassow; Tel. 038826 88360; Abbrucharbeiten/Recycling.
- HR-Spur: Torsten Klaus Milhan (Dassow) war GF einer ZBR Recycling UG und ist GF einer S und K UG (Northdata/Registercheck-Leads, Zusammenhang mit Milhan Recycling ungeklärt).
- Klärfall Folgewelle: Betreiber-Quelle oder HR-Beleg suchen.

### Recherche 02.10.2026

- [Recherche 02.10.2026: Adresse/PLZ/Telefon/Inhaber in 6 Verzeichnissen identisch belegt (Ausnahmefall-Fill mit offener Dokumentation, baut auf 01.10-Leads auf): Milhan Recycling Inh. Torsten Milhan, Dassower Str. 5, 23942 Dassow, Tel. 038826/88360; Branchen: Abbruch/Recycling/Abfallwirtschaft; Zusatzspur: Milhan Recycling UG (haftungsbeschränkt), selbe Adresse (bizarchiv/bauenwohnen) — Registerzusammenhang weiter ungeklärt; keine Betreiber-Website → website_status unbekannt; bleibt pruefung (nur Aggregator-Belege); Quelle(n): misterwhat.de, business-branchenbuch.de, unternehmensauskunft.com, schrottplatz.org, tellows.de, germanlisting.com (02.10.2026)]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Dassower Straße, Dassow) — die Dossier-Hausnummer 5 existiert, aber an zwei verschiedenen Orten.** Straßen-Raster über den **gesamten** Straßenverlauf der Dassower Straße (Straßenbbox 10.9604825/53.8865860 bis 11.0103702/53.9556800 - die Straße durchzieht Dassow über **mehrere Ortsteile**), Scan-Fenster **10.9584825/53.8845860 bis 11.0123702/53.9576800** (**48 Kacheln im 0,01°-Raster des Repo-Werkzeugs `scripts/release/osm_street_scan.py`, 0 Kachel-Fehler**). Zählung getrennt: **61 Adressobjekte / 43 verschiedene Hausnummern** auf der Straße (1, 1a, 1b, 2, 2a, 3, 3a, 4, 4a, 4c, 4d, 4e, 5, 5a, 6, 7, 8, 8a, 9, 9a, 9b, 9c, 10, 10a, 10b, 11, 12, 12a, 12b, 13, 14, 14a, 15, 15a, 16, 17, 18, 19, 20, 21, 26, 27, 28) - die Aufzählung nennt exakt die live vorhandenen Nummern; Filter-Key `addr:street=Dassower Straße` (mit `addr:housenumber` als Hausnummer). Es sind **keine Bereichsnotationen** live vorhanden, aber **zahlreiche Buchstaben-Zusätze** (1a, 1b, 2a, 3a, 4a, 4c, 4d, 4e, 5a, 8a, 9a–9c, 10a, 10b, 12a, 12b, 14a, 15a) — nach der Referenzzählung des Repo-Werkzeugs zählt **jede** Buchstabennummer als **eigene** Hausnummer. Im gesamten Rastergebiet **1172 Adressobjekte mit Hausnummer / 1143 verschiedene Hausnummern**; im Raster tragen **498 Objekte `name=`** und **85 Objekte `operator=`**. **Die Dossier-Hausnummer 5 ist live zweifach auf der Dassower Straße erfasst** — das ist zu schreiben, nicht zu übergehen: **way/446649357** mit `addr:housenumber=5`, `addr:street=Dassower Straße`, `addr:city=Dassow`, `addr:country=DE`, `addr:postcode=23942`, `addr:suburb=Harkensee`, `building=yes` (Objekt-BBox 10.9743157/53.9539829 bis 10.9747600/53.9542671, Mitte 10.9745621/53.9541534) **und** **way/238278054** mit `addr:housenumber=5`, `addr:street=Dassower Straße`, `addr:city=Dassow`, `addr:country=DE`, `addr:postcode=23942`, `addr:suburb=Prieschendorf`, `building=yes` (Objekt-BBox 10.9859149/53.8894977 bis 10.9861456/53.8895694, Mitte 10.9860087/53.8895406). Die beiden Objekte liegen rund **7,3 km** auseinander in **verschiedenen Ortsteilen** — welche der beiden Adressen die Milhan Recycling-Adresse ist, ist aus OSM **nicht** entscheidbar. Objekte mit Bezug zum Betrieb im Raster: **keines** - die Suche nach „milhan“, „recycling“, „abbruch“ und „dassow“ über die Felder name, operator, description, website, contact:website, note, contact:email, email, short_name, brand und operator:name findet **keinen** Treffer mit Betriebsbezug zur Milhan Recycling; **kein** Objekt des Rasters trägt einen `name=`- oder `operator=`-Wert mit Bezug zu „Milhan“. Der einzige Betrieb mit `name=` auf der Dassower Straße ist node/805624351 mit `name=Opa's Bauernhof`, `addr:housenumber=26`, `addr:street=Dassower Straße`, `addr:city=Kalkhorst`, `addr:postcode=23942`, `addr:suburb=Neuenhagen`, `shop=farm`, `tourism=chalet` - ein **Fremdbetrieb** (Ferienanlage), **kein** Bezug zur Milhan Recycling. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=10.9584825,53.8845860,11.0123702,53.9576800 (Kachelraster, live abgerufen 31.10.2026) ; https://api.openstreetmap.org/api/0.6/way/446649357.json ; https://api.openstreetmap.org/api/0.6/way/238278054.json ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen und Geokodierung — die Adresslage ist durch OSM nicht auflösbar.** (a) Die **Dossier-Adresse Dassower Str. 5** ist in OSM an **zwei** Stellen vorhanden (Harkensee und Prieschendorf); der in der Produktionsdatenbank gespeicherte Punkt **53.9110000/10.9755000** liegt **zwischen** beiden, rund **4,8 km** von der Harkenseer-Objektmitte (way/446649357) und rund **2,5 km** von der Prieschendorfer-Objektmitte (way/238278054) entfernt - er ist **keinem** der beiden Objekte zugeordnet. **Beide** Kandidaten liegen in **Ortsteilen**, die nicht dem Kernort „Dassow“ entsprechen (der Kernort Dassow liegt weiter südlich); die Verzeichnis-Leads der Vorwellen nennen nur „Dassow“ ohne Ortsteil. **Welche Adresse stimmt, bleibt offen** - das ist ein Klärfall für die nächste Welle (Betreiber-Quelle oder Registerbeleg mit Ortsteil). (b) Die Vorwellen-Notizen bleiben **wortgleich** gültig: weiterhin **keine** Betreiber-Website, weiterhin **kein** HR-Beleg, weiterhin nur Aggregator-Leads; `status: pruefung` und `website_status: unbekannt` bleiben unverändert. (c) **Preise/Gebühren:** für die Milhan Recycling ist **keine** Preisquelle auffindbar - weder eine Betreiber-Website noch ein Sorten-/Preisverzeichnis; die Seed-Angabe „Eisen“ bleibt eine Aggregator-Angabe ohne Bestätigung durch den Betreiber. **Ankaufpreise, Verkaufserlöse und Gebühren sind für diesen Betrieb nicht belegt**. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=10.9584825,53.8845860,11.0123702,53.9576800 ; https://api.openstreetmap.org/api/0.6/way/238278054.json (Abruf 31.10.2026).]
