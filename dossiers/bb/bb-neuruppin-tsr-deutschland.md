---
slug: bb-neuruppin-tsr-deutschland
name: TSR Deutschland GmbH & Co. KG
trader_type: metallhaendler
state: BB
city: Neuruppin
street: Philipp-Oehmigke-Str. 13
postcode: '16816'
phone: 03391 404839
email: ''
opening_hours: 'Mo-Do 07:30-12:00, 12:30-16:30, Fr 07:30-12:00, 12:30-14:00'
website: https://www.tsr-recycling.de/
website_status: blockiert
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Brandenburg (BB) – Recherche
provenance_ankauf_raw: ja
provenance_origin: table
---

# TSR Deutschland GmbH & Co. KG

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Fe-/NE-Metalle, Container, Demontage
- Adressbeleg: tsr-recycling.de/standorte

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Philipp-Oehmigke-Straße, Neuruppin).** Der Streetname wurde in **41 Suchkacheln** à 0,01 Grad um den Ort gefunden (Way way/113750910); Straßenbbox **12.7933022/52.8757648 bis 12.7954769/52.8785918**, Raster mit 0,010 Grad Rand: **12.7833022,52.8657648,12.8054769,52.8885918**, abgetastet in **9 Kacheln à 0,01 Grad, 0 Kachel-Fehler** (1 der 41 Suchkacheln wurde vom Server mit HTTP 400 abgewiesen und ist in der Suche unberücksichtigt). Zählung getrennt: **4 Adressobjekte / 2 verschiedene Hausnummern** auf der Straße (**4** und **13**; die Nummer 4 ist von **drei** Objekten belegt, node/9330356979, node/10732991283 und way/180928804); im gesamten Rastergebiet **80 Adressobjekte mit Hausnummer / 37 verschiedene Hausnummern**, im Raster tragen **67 Objekte `name=`** und **20 Objekte `operator=`**. Filter-Key war durchgehend **`addr:street=Philipp-Oehmigke-Straße`** mit `addr:housenumber`; Bereichs- und Buchstabennotationen wurden mit `osm_street_scan.expand` aufgelöst (hier traten keine auf). Die **Dossier-Hausnummer 13 liegt auf dieser Straße**. Quelle(n): https://api.openstreetmap.org/api/0.6/map?bbox=12.7833022,52.8657648,12.8054769,52.8885918 (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Positiver Beleg: zwei OSM-Objekte nennen den Betrieb, eines davon trägt exakt die Dossier-Adresse und die Dossier-Telefonnummer.** node/14270350334 mit `addr:housenumber=13`, `addr:street=Philipp-Oehmigke-Straße`, `addr:city=Neuruppin`, `addr:postcode=16816`, `name=TSR Recycling`, `operator=TSR Deutschland GmbH & Co. KG`, `landuse=industrial`, `industrial=scrap_yard`, `phone=+49 3391 404839`, `opening_hours=Mo-Th 07:30-12:00, 12:30-16:30; Fr 07:30-12:00, 12:30-14:00`; dazu node/14270350958 mit `amenity=recycling`, `recycling:scrap_metal=yes`, `recycling_type=container`, `opening_hours=Mo-Su 00:00-23:59`, `operator=TSR Deutschland GmbH & Co. KG`. **Negativaussage, qualifiziert:** von den 4 Adressobjekten der Straße tragen **3 Objekte `name=`** (node/9330356979 mit `name=Simon`, node/14270350334 mit `name=TSR Recycling`, way/180928804 mit `name=Dreistern-Konserven`) und **3 Objekte `operator=`** (node/9330356979 mit `operator=Werner Simon GmbH & Co. KG`, node/14270350334 mit `operator=TSR Deutschland GmbH & Co. KG`, way/180928804 mit `operator=Dreistern-Konserven GmbH & Co KG`); **mit Bezug zu TSR** trifft es auf der Straße **1 Objekt** (node/14270350334) und im gesamten Raster **2 Objekte** (node/14270350334 und node/14270350958). Der Betrieb wird damit **nicht verleugnet**, sondern an genau zwei Objekten positiv belegt. Quelle(n): https://api.openstreetmap.org/api/0.6/node/14270350334.json ; https://api.openstreetmap.org/api/0.6/node/14270350958.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Betreiber-Angaben live bestätigt - die Standortseite nennt Adresse, Telefon und Öffnungszeiten deckungsgleich zum Dossier.** Die Standortseite „Schrotthändler Neuruppin" nennt **TSR Deutschland GmbH & Co. KG, Philipp-Oehmigke-Straße 13, 16816 Neuruppin**, Telefon **+49 3391 404839** (passend zum Dossier `03391 404839`), Fax **+49 3391 404843** und die Öffnungszeiten **Mo-Do 07:30 - 12:00 und 12:30 - 16:30, Fr 07:30 - 12:00 und 12:30 - 14:00**; für 21.11.2026 und 28.11.2026 werden 09:00 - 12:00 Uhr, für 30.11.2026 geschlossen genannt. Die `opening_hours` im Frontmatter ist bisher leer und wird mit diesen Werten **gefüllt**. Leistungsangebot der Seite: Altmetalle und Metallschrott, **Altpapier**, **Bleibatterien**, weitere Wertstoffe, **Reifen-Demontage** (vor Ort oder im Betrieb), **Container in verschiedenen Größen**, **THE METAL BOX** zur Zwischenlagerung und **Brennarbeiten**; die Efb-Bescheinigung (Region Nordost, Regionalbereich Nordost II) und das QM-Zertifikat nach DIN EN ISO 9001:2015 liegen als PDF vor. **Die Website ist nur über einen Reader-Proxy lesbar:** der Direktabruf von https://www.tsr-recycling.de/ liefert HTTP 200 mit einer **„Challenge Validation"**-Bot-Wall (1840 Bytes, kein Inhalt); `website_status` wird deshalb von leer auf **blockiert** gesetzt. Das Impressum nennt als Gesellschaft **TSR Deutschland GmbH & Co. KG**, Sitz **Brunnenstraße 138, 44536 Lünen**, Registergericht **Amtsgericht Dortmund**, Registernummer **HRA 19743**, persönlich haftende Gesellschafterin **TSR Deutschland Verwaltungs GmbH** (AG Dortmund, HRB 36274), Geschäftsführung **Michael Jasperneite, Wouter Cornelis Kusters, Dr. Christian Satlow**; TSR Deutschland ist ein Unternehmen der **REMONDIS-Gruppe**. Das Feld `email` bleibt **leer**, weil die Standortseite nur ein Kontaktformular verlinkt und die Impressumsadresse info@tsr.eu eine konzernweite Zentrale ist. Quelle(n): https://www.tsr-recycling.de/standorte/schrotthaendler-neuruppin/ ; https://www.tsr-recycling.de/impressum/ ; https://www.tsr-recycling.de/ (Direktabruf, Bot-Wall) (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Preise und Gebühren getrennt.** ANKAUF: **keine** öffentliche Ankaufspreisliste; die Seite „Schrottlieferanten" nennt **tagesaktuelle Preise**, die sich nach den Rohstoffmärkten richten, sowie die Regel, dass **sortierte Materialien** höher vergütet werden als unsortierter Mischschrott, und verweist für konkrete Preise auf die telefonische Anfrage („Schrottpreise sind tagesaktuell und marktgerecht") - **kein Zahlenwert** genannt. VERKAUF: **keine** öffentliche Verkaufspreisliste. GEBÜHREN: **keine** bezifferte Betreibergebühr; der Containerservice und die Abholung werden ohne Preis genannt, die Preisfindung für „Aktuelle Preisinformationen" verweist auf die Kontaktaufnahme. **Markt- bzw. Börsenpreis ist kein Standortpreis:** die genannten „tagesaktuellen Preise" sind Rohstoffmarkt-Notierungen und damit **kein** Standortpreis für Neuruppin. Quelle(n): https://www.tsr-recycling.de/kunden/schrottlieferanten/ ; https://www.tsr-recycling.de/standorte/schrotthaendler-neuruppin/ (Abruf 31.10.2026).]
