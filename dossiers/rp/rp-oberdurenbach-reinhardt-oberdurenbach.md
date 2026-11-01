---
slug: rp-oberdurenbach-reinhardt-oberdurenbach
name: Reinhardt Oberdürenbach
trader_type: sonstige
state: RP
city: Oberdürenbach
street: Königsseeweg 2
postcode: '56651'
phone: 01775385349
email: ''
opening_hours: ''
website: https://www.schrottmetallhandelreinhardt.com
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: '{"allowed": true, "customer_types": ["privat", "gewerbe"], "conditions": "Kostenlose Schrott-Abholung (24-48h), Demontage, Ladekran (Betreiber-Website, Feedback-Triage 01.10.2026)"}'
provenance_section: Nachtrag Wide-Net PROSE-Cluster (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Reinhardt Oberdürenbach

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- , Königseeweg 2 — ungleich Seed Oberzissen
- Adresse: Oberdürenbach
- Adressbeleg: https://www.schrottmetallhandelreinhardt.com

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 3122 berechtigt, eingearbeitet — pickup_json (kostenlose Schrott-Abholung, Demontage, Ladekran) gesetzt; street 'Königseeweg 2' → 'Königsseeweg 2' (Einheitsschreibung Betreiber-Website, Footer + Standortblock); Standort 56651 Oberdürenbach bestätigt; phone 01775385349 steht im Kontaktblock (Dossier-Wert bestätigt, keine Änderung); website_status aktiv, status bleibt pruefung (dünne Betreiber-Seite ohne Impressum/HR, keine 2. Quelle). Korrektur zur Feedback-Details-Aussage 'kein Telefon': Kontaktbereich nennt Tel 01775385349. Adress-Hinweis: Koordinaten neu zu geocodieren (Straßenschreibung geändert). Quelle(n): https://www.schrottmetallhandelreinhardt.com/ (Abruf 01.10.2026)]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Königsseeweg, Oberdürenbach).** Kachelraster über den **gesamten** Straßenverlauf: OSM-Way-BBox aller Ways mit `name=Königsseeweg` (4 Ways: way/128908977, way/134114698, way/276141318, way/276141392) = **50.469097/7.1555209 bis 50.4824351/7.1728872**, **plus 0,010 Grad Rand** → Raster-BBox **bbox=7.1455209,50.4590970,7.1828872,50.4924351** (live über `https://api.openstreetmap.org/api/0.6/map.json?bbox=7.1455209,50.4590970,7.1828872,50.4924351`, Kacheln à 0,01 Grad). Zählung getrennt: **16 Adressobjekte / 16 verschiedene Hausnummern** (1, 2, 3, 4, 7, 8, 9, 10, 11, 12, 15, 16, 17, 18, 21, 27) auf der Straße, Filter-Key `addr:street` gleich „Königsseeweg“ (Filter-Key `addr:street=Königsseeweg` **mit** `addr:housenumber`; keine Bereichs- oder Buchstabennotation). **Die Dossier-Hausnummer 2 existiert in OSM**: way/659781993 mit `addr:housenumber=2`, `addr:street=Königsseeweg`, `addr:city=Oberdürenbach`, `addr:country=DE`, `addr:postcode=56651`, `building=yes` — das Objekt trägt **weder `name=` noch `operator=`**; die **Dossier-Adresse ist adresslich vollständig belegt** (Straße, Hausnummer, PLZ und Ort stimmen überein), aber **nicht betrieblich**. **Alle 16 Adressobjekte** tragen `addr:street=Königsseeweg`, `addr:city=Oberdürenbach`, `addr:country=DE`, `addr:postcode=56651` und `building=yes` — **keines** mit `name=`/`operator=`. Im Scan-BBox tragen **190 Objekte** `name=` oder `operator=`; **keines** davon nennt Reinhardt, Schrott oder Metallrecycling — **ein Objekt, das den eigenen Betrieb nennt, existiert im OSM nicht**. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=7.1455209,50.4590970,7.1828872,50.4924351 (bbox-Raster, live abgerufen) ; https://api.openstreetmap.org/api/0.6/way/659781993.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Die Betreiberseite widerspricht sich in der Straßenschreibweise und setzt eine andere Kartenposition als die Produktionsdatenbank.** Der **Standortblock** der Kontaktseite (https://www.schrottmetallhandelreinhardt.com/kontakt, HTTP 200) nennt „Schrott&metalhandel Reinhardt, **Königsseeweg 2**, 56651 Oberdürenbach“; der **Karten-Widget** derselben Seite trägt dagegen `data-location="**Königseeweg 2** 56651 Oberdürenbach, Deutschland"` (also **ohne** das „s“ nach „König“) mit `data-location-lat="50.4700891"` und `data-location-lng="7.156843"`. Die **Startseite** nennt im Fließtext **„Königsseeweg 2“**. Die **in der Produktionsdatenbank gespeicherte Koordinate** (lat 50,4486833 / lon 7,1919) liegt **rund 3 km südöstlich** vom Karten-Widget des Betreibers und **nicht** in Oberdürenbach, sondern im Bereich **Galenberg/Oberzissen** — die dortigen OSM-Adressobjekte tragen `addr:city=Galenberg` bzw. `addr:city=Oberzissen`. **Die Dossier-Adresse wird nicht geändert**: `street` bleibt **„Königsseeweg 2“** (Einheitsschreibung der Betreiberseite und **exakt** die OSM-Schreibweise `addr:street=Königsseeweg`), die **Koordinate ist aber neu zu geocodieren**; die Variante „Königseeweg“ aus dem Karten-Widget ist als **Uneinheitlichkeit der Betreiberseite** dokumentiert (in OSM existiert **keine** Straße dieses Namens). Quelle(n): https://www.schrottmetallhandelreinhardt.com/ ; https://www.schrottmetallhandelreinhardt.com/kontakt (Abruf 31.10.2026) ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Leistungen bestätigt; Preise und Gebühren getrennt.** Die Startseite (https://www.schrottmetallhandelreinhardt.com/, HTTP 200) nennt die Leistungen: **Schrottabholung**, **Entsorgung vor Ort** („Unsere Firma ist im Besitz von einem Gewerbe und über eine **extra Genehmigung für das fachgerechte Metallrecycling**. Auch ein **Ladekran** ist vorhanden für richtig schwere Sachen“), **Metallrecycling**, **umweltgerechte Entsorgung**; die angenommene Sortenliste umfasst **Fahrräder, Heizkörper, Edelstahl, Mofas/Mopeds, Kabel, Alles aus Eisen, Landwirtschaftliche Geräte, Messing, Aluminium, Kupfer, UVM, Au** sowie **Zerlegung von Industriemaschinen und Landwirtschaftlichen Geräten inklusive der Entsorgung**, **Demontage von Öltanks** und **Entkernungen**. ANKAUF: die Seite behauptet einen **Ankauf** („Wir nehmen alle Arten von Schrott an“) und nennt zu den **Preisen** ausdrücklich: „**Ja, wir bieten kostenlose Preisangebote basierend auf Ihrer Schrottmenge und Materialart an.**“ — **kein Zahlenpreis**, sondern ein **kostenloses Angebot nach Menge und Materialart**. VERKAUF: **keine** Verkaufsangaben. GEBÜHREN: **kostenlose Schrottabholung** („Wir bieten an **kostenlose Schrott Abholung**, Zerlegung von Industriemaschinen und Landwirtschaftlichen Geräten inklusive der Entsorgung, Demontage von Öltanks, Entsorgung von Schrott und Altmetall“) und **kostenlose Preisangebote**; die Abholung erfolgt „in der Regel innerhalb von **24-48 Stunden** nach Ihrer Anfrage“ (Frist, **keine** Gebühr). Quelle(n): https://www.schrottmetallhandelreinhardt.com/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Die Beleglage bleibt dünn — `status` bleibt `pruefung`.** Die Betreiberseite hat **kein Impressum**, **keine** HR-Nummer, **keine** Datenschutzerklärung mit Kontaktdaten und **keine** E-Mail-Adresse (nur ein Kontaktformular); die Navigation umfasst nur **Home / Kontakt / Bilder**. Ein **unabhängiger Zweitbeleg** für Identität, Adresse oder Telefonnummer ist **nicht** gefunden; der Dossier-Eintrag stammt aus der **PROSE-Cluster-Saat** („Adressbeleg: https://www.schrottmetallhandelreinhardt.com“). `email` und `opening_hours` bleiben daher **leer**, `website_status` bleibt **aktiv** (Startseite und Kontaktseite liefern HTTP 200). Das im Dossier geführte `pickup_json` („Kostenlose Schrott-Abholung (24-48h), Demontage, Ladekran“) ist durch die Betreiberseite **bestätigt** und bleibt unverändert. Quelle(n): https://www.schrottmetallhandelreinhardt.com/ ; https://www.schrottmetallhandelreinhardt.com/kontakt ; https://www.schrottmetallhandelreinhardt.com/bilder (Abruf 31.10.2026).]

- [Korrektur 31.10.2026: Koordinaten-Hinweis. Die in der Produktionsdatenbank gespeicherte Position (lat 50,4486833 / lon 7,1919) liegt **nicht** am Betriebsstandort, sondern im Ortsteil **Galenberg/Oberzissen** (dortige OSM-Adressobjekte tragen `addr:city=Galenberg` bzw. `addr:city=Oberzissen`). Das **Karten-Widget der Betreiberseite** setzt den Standort auf **lat 50.4700891 / lon 7.156843** — das ist der Ort **Oberdürenbach** und deckt sich mit der OSM-Adresse `addr:street=Königsseeweg`, `addr:city=Oberdürenbach`, `addr:postcode=56651`. Die Dossier-Adresse (Straße/Hausnummer/PLZ) ist damit **adresslich belegt**; der **GEO-Punkt des Datensatzes ist falsch und neu zu setzen**. `street` bleibt unverändert. Quelle(n): https://www.schrottmetallhandelreinhardt.com/kontakt (data-location-lat/-lng im Karten-Widget) ; https://api.openstreetmap.org/api/0.6/way/659781993.json ; read-only `public.db` (Abruf 31.10.2026)]
- [Recherche 31.10.2026: **Hausnummern-Gegenrechnung abgeschlossen — die Aufzählung ist bestätigt.** Nachgerechnet über die Zähllogik von `scripts/release/osm_street_scan.py` (Filter-Key `addr:street=Königsseeweg` **mit** `addr:housenumber`, Kachelraster 0,01 Grad über das **vollständige** Raster-BBox **bbox=7.1455209,50.4590970,7.1828872,50.4924351**, 16/16 Teilfenster vollständig verfügbar): **16 Adressobjekte / 16 verschiedene Hausnummern** — **deckungsgleich** mit der Angabe im Recherche-Vermerk oben: jede dort genannte Nummer existiert live, und jede live vorhandene Nummer ist dort genannt. Die **Dossier-Hausnummer 2** ist dabei durch way/659781993 (`addr:housenumber=2`, `addr:postcode=56651`, `addr:city=Oberdürenbach`, **weder `name=` noch `operator=`**) belegt. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=7.1455209,50.4590970,7.1828872,50.4924351 (Raster vollständig, live abgerufen) (Abruf 31.10.2026).]
