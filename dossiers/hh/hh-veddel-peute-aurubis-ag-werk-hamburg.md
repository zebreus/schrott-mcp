---
slug: hh-veddel-peute-aurubis-ag-werk-hamburg
lat: 53.5215966
lon: 10.0332899
name: Aurubis AG, Werk Hamburg
trader_type: schrotthaendler
state: HH
city: Veddel/Peute
street: Hovestr. 50
postcode: '20539'
phone: 040 / 7883-0
email: ''
opening_hours: ''
website: https://www.aurubis.com
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Hamburg (HH) — Recherche für Schrott MCP
provenance_ankauf_raw: ja (nur Gewerbe/große Mengen, Zentraleinkauf)
provenance_origin: table
---

# Aurubis AG, Werk Hamburg

## Überblick

Aurubis AG, Werk Hamburg, Hovestraße 50 in 20539 Hamburg-Veddel, ist die Geschäftsstelle der
Kupferhütte mit HRB 1775 am Amtsgericht Hamburg, die Kupfer- und Legierungsschrott,
Elektronikschrott, Shredderfraktionen sowie Schlacken und Rückstände aufbereitet. Belegt sind
die Anschrift und die Telefonnummer 040 7883-0 durch das Konzern-Impressum, in OpenStreetMap
steht an der Hausnummer 50 ein Bürogebäude ohne Betriebsnamen, während die betrieblich
benannten Aurubis-Flächen Zentrale und Werk Ost als Industrieflächen geführt sind. Offen bleibt
die Frage der Annahme, denn die Dossier-Adresse ist die Büroseite, das Gebäude trägt keinen
Annahme- oder Recycling-Tag, und ob und wo Material angeliefert werden kann, ist nicht belegt.
Ein Standortkurs je Sorte wird nicht veröffentlicht, die Preisbildung läuft laut Konzernangabe
transparent über das Kundenportal, Gebühren für Annahme oder Container fehlen.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Kupferhütte: Kupfer-/E-Schrott, komplexe Recyclingrohstoffe

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2180 teilweise berechtigt — PLZ 20539 + Telefon 040 / 7883-0 + website_status aktiv per Konzern-Impressum (Geschäftsstelle Aurubis AG, Hovestrasse 50, 20539 Hamburg, AG Hamburg 66 HRB 1775); Telefon = Zentrale Geschäftsstelle, keine Schrottannahme; private Schrottanlieferung (dropoff) auf Betreiberseite unbelegt → kein dropoff-Fill; Quelle(n): https://www.aurubis.com/impressum]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Hovestraße, Hamburg-Veddel/Peute).** Kachelraster über den **gesamten** Straßenverlauf: OSM-Way-BBox aller Ways mit `name=Hovestraße` (13 Ways) = **53.5163255/10.02331 bis 53.5250171/10.0500878**, **plus 0,010 Grad Rand** → Raster-BBox **bbox=10.0133255,53.5063255,10.0600878,53.5350171** (live über `https://api.openstreetmap.org/api/0.6/map.json?bbox=10.0133255,53.5063255,10.0600878,53.5350171`, Kacheln à 0,01 Grad). Zählung getrennt: **37 Adressobjekte / 37 verschiedene Hausnummern** (16, 20, 21, 23, 27, 28, 28a, 30, 32, 33, 35, 39, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 57, 59, 61, 63, 65, 66, 67, 68, 70, 71, 72, 74, 76) auf der Straße, Filter-Key `addr:street` gleich „Hovestraße“ (Filter-Key `addr:street=Hovestraße` **mit** `addr:housenumber`; keine Bereichs- oder Buchstabennotation auf dieser Straße). **Die Dossier-Hausnummer 50 existiert in OSM**: way/218366110 mit `addr:housenumber=50`, `addr:street=Hovestraße`, `addr:city=Hamburg`, `addr:country=DE`, `addr:postcode=20539`, `building=office`, `building:levels=9` — das Objekt trägt **weder `name=` noch `operator=`**, ist also ein reiner Adress-/Gebäudepunkt **ohne** Betriebsbezug. **Positiver Betriebsbeleg für Aurubis im Raster** (ein Objekt, das **den eigenen Betrieb** nennt): way/665691214 mit `name=Aurubis - Zentrale`, `operator=Aurubis AG`, `landuse=industrial`, `phone=+49 40 78830`, `website=https://www.aurubis.com/de`; ferner way/666212155 mit `name=Aurubis - Werk Ost`, `operator=Aurubis AG`, `landuse=industrial`, `website=https://www.aurubis.com/de`; dazu die Nebenanlagen way/36368370 mit `name=Aurubis`, `operator=Aurubis AG`, `amenity=parking`, `access=private`, `opening_hours=24/7`; way/665708071 mit `name=Aurubis`, `operator=Aurubis AG`; way/665708072 mit `name=Aurubis Besucher`, `operator=Aurubis AG`, `fee=no`; way/666212159 mit `name=Truck-Parking`, `operator=Aurubis AG`; way/666212165 mit `operator=Aurubis AG`; way/36696100 mit `name=Aurubis - Mitarbeiterparkplatz`, `operator=Aurubis AG`; node/1707397575 mit `name=Esse Kontaktanlage`, `operator=Aurubis AG`. **Auf der Straße selbst tragen genau 4 Objekte `name=`/`operator=`**: way/282471499 mit `name=Peter Stolz Edelstahlhandelsgesellschaft`, `operator=Peter Stolz Edelstahlhandelsgesellschaft mbH`, `addr:housenumber=32`, `addr:street=Hovestraße` (**anderer Betrieb**); way/230359635 mit `name=NT Logistic GmbH`, `addr:housenumber=20`; way/110413625 mit `name=Wiwa WILKO WAGNER`, `addr:housenumber=21`; way/310627446 mit `name=Abwasserpumpwerk P20`, `operator=Hamburg Wasser`, `addr:housenumber=16`. Im Raster tragen **1547 Objekte** `name=` oder `operator=`, **davon 20 mit Aurubis-Bezug**; **auf einer anderen Straße** liegt node/13110660436 mit `name=Peute Baustoff GmbH`, `description=100%-ige Tochtergesellschaft der Aurubis AG`, `addr:housenumber=79`, `addr:street=Peutestraße` und way/1426087853 mit `name=Druckwärmespeicher "Thermoskanne"`, `description=Abwärme von Aurubis: 4 Mill. Liter Warmwasserspeicher für Fernwärme`, `addr:housenumber=16`, `addr:street=Georgswerder Damm`. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=10.0133255,53.5063255,10.0600878,53.5350171 (bbox-Raster, live abgerufen) ; https://api.openstreetmap.org/api/0.6/way/218366110.json ; https://api.openstreetmap.org/api/0.6/way/665691214.json ; https://api.openstreetmap.org/api/0.6/way/666212155.json ; https://api.openstreetmap.org/api/0.6/way/282471499.json ; https://api.openstreetmap.org/api/0.6/node/13110660436.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Anschrift und Lage des Werks bestätigt, aber die Dossier-Adresse ist die *Büro*-Seite, nicht die Annahme.** Das Konzern-Impressum nennt die Geschäftsstelle der Aurubis AG mit **Hovestrasse 50, 20539 Hamburg**, **AG Hamburg HRB 1775** — das ist deckungsgleich mit dem Dossier. In OSM trägt das Gebäude an dieser Nummer (way/218366110) `building=office`, während die **betrieblich** benannten Aurubis-Flächen (`landuse=industrial`) way/665691214 (`name=Aurubis - Zentrale`) und way/666212155 (`name=Aurubis - Werk Ost`) tragen; die Telefonnummer der Zentrale ist dort als `phone=+49 40 78830` hinterlegt — das entspricht der Dossier-Angabe **040 / 7883-0** (Schreibweise der Konzernseite). Das Dossier vermerkt bereits zutreffend, dass es sich um die **Telefonzentrale der Geschäftsstelle** handelt und **keine** Schrottannahme belegt ist; dieser Befund wird durch den OSM-Abgleich **gestützt**: Das Objekt an der Dossier-Adresse trägt **keinen** Annahme- oder Recycling-Tag, und ein Objekt mit Annahmebezug auf der Hovestraße ist **nicht** vorhanden. Quelle(n): https://www.aurubis.com/impressum (Abruf 31.10.2026) ; https://api.openstreetmap.org/api/0.6/way/218366110.json ; https://api.openstreetmap.org/api/0.6/way/665691214.json.]
- [Recherche 31.10.2026: **Preise strikt getrennt: Ankauf, Verkauf, Gebühren.** ANKAUF: die Konzernseite „Recycling at Aurubis“ beschreibt das Geschäft als Abnehmer von Recyclingrohstoffen („Aurubis maximizes metal recovery from recycling materials … We match your materials to the most suitable facility based on type and volume“; „Transparent and fair pricing tailored for your materials“; „We process a variety of recycling materials, from copper scrap to printed circuit boards and precious metals“) und nennt die **Sortenliste** (Copper & alloy scrap: Millberry/direct melt scrap, Copper scrap, Copper granules, Insulated copper wire (ICW), Copper tinned, Gunmetal, Tin bronze, Aluminum bronze, Brass, Copper-nickel alloys, Lead scrap, Copper-lead granules, Copper graphite foils; Printed circuit board & electronic scrap: Printed circuit boards, Electronic waste (WEEE); Mixed heavy metal fractions & shredder: Automotive recycling shredder, E-scrap shredder, Heavy metal fraction from waste incineration, Shredders from the recycling of end-of-life vehicles, E-scrap shredder; Residues, slimes & sludges: Slags, Drosses, Fines) sowie die **Verarbeitungskapazität** „> 1000000 t of recycling raw materials processed annually“. **Zahlenpreise je Sorte werden nicht veröffentlicht** — die Preisbildung verläuft laut Seite über „Transparent and fair pricing“ und das Kundenportal „myAurubis Portal: Simplify and enhance your recycling experience“ (Verträge, Lieferdetails, Materialanalysen). **Ein Standortkurs für Hamburg ist damit nicht belegt**. VERKAUF: Aurubis tritt als **Abnehmer** und Verarbeiter auf; als Verkäufer von Sekundärrohstoffen (Stahl-/Gussschrotten) wird der Standort Hamburg **nicht** als Anbieter geführt — die Produktseiten nennen Kupferkathoden, Kupferprodukte, Gießwalzdraht, Stangen/Profile, Stranggussformate, Industrie-Walzprodukte und Architektur-Walzprodukte (das sind **Erzeugnisse des Werks**, keine Schrott-Verkaufspreise). GEBÜHREN: **keine** öffentliche Annahme-, Container- oder Entsorgungsgebühr für den Standort Hamburg gefunden; die Seite nennt lediglich das myAurubis-Portal als „single point of contact“ für „contracts, delivery details, and material assays“. **Keine** Erschließung von Preisen aus Börsen-/Marktkursen. Quelle(n): https://www.aurubis.com/recycling (Abruf 31.10.2026) ; https://www.aurubis.com/impressum (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Hausnummern-Gegenrechnung abgeschlossen — die Aufzählung ist bestätigt.** Nachgerechnet über die Zähllogik von `scripts/release/osm_street_scan.py` (Filter-Key `addr:street=Hovestraße` **mit** `addr:housenumber`, Kachelraster 0,01 Grad über das **vollständige** Raster-BBox **bbox=10.0133255,53.5063255,10.0600878,53.5350171**, 15/15 Teilfenster vollständig verfügbar): **37 Adressobjekte / 37 verschiedene Hausnummern** — **deckungsgleich** mit der Angabe im Recherche-Vermerk oben: jede dort genannte Nummer existiert live, und jede live vorhandene Nummer ist dort genannt. Die **Dossier-Hausnummer 50** ist dabei durch way/218366110 (`addr:housenumber=50`, `addr:postcode=20539`, `addr:city=Hamburg`, **weder `name=` noch `operator=`**) belegt. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=10.0133255,53.5063255,10.0600878,53.5350171 (Raster vollständig, live abgerufen) (Abruf 31.10.2026).]

### Geokodierung 01.11.2026

- [Geokodierung 01.11.2026: **Koordinate korrigiert**: `lat`/`lon` = **53.5215966/10.0332899** (Mittel der eindeutigen
  Knoten von way/218366110, innerhalb der Objekt-BBox). Quelle 1 (Betreiber): Das Konzern-Impressum nennt
  **„Geschäftsstelle Aurubis AG, Hovestrasse 50, 20539 Hamburg / Germany“**, Telefon **040 / 7883-0**, Amtsgericht
  Hamburg **HR B 1775**, USt-IdNr. DE 11 85 14 155 (live abgerufen 01.11.2026, https://www.aurubis.com/impressum) —
  die Dossier-Adresse, -Telefonnummer und die Registerlage sind damit über die Betreiberquelle belegt. Quelle 2
  (OSM): way/218366110 (live geladen 01.11.2026, https://api.openstreetmap.org/api/0.6/way/218366110/full.json) trägt
  `addr:housenumber`=50, `addr:street`=Hovestraße, `addr:city`=Hamburg, `addr:country`=DE, `addr:postcode`=20539,
  `building`=office, `building:levels`=9 — die Dossier-Hausnummer **50** und die Dossier-Straße sind objektgenau
  belegt; es hat 6 Knotenreferenzen, davon 6 eindeutige, die BBox der eindeutigen Knoten reicht von
  53.5214970/10.0331625 bis 53.5216801/10.0334673 und umschließt den Mittelwert. Zusatzbeleg: way/665691214
  (`name`=Aurubis - Zentrale, `operator`=Aurubis AG, `landuse`=industrial, `phone`=+49 40 78830,
  `website`=https://www.aurubis.com/de; live abgerufen 01.11.2026) liegt unmittelbar nördlich desselben Standorts und
  bestätigt den Betriebsbezug; die Dossier-Telefonnummer **040 / 7883-0** ist dort als `phone`=+49 40 78830
  hinterlegt. **Qualifizierung:** das Objekt an der Dossier-Adresse trägt **weder `name` noch `operator` und kein
  Annahme-/Recycling-Tag** — es ist der **Büro-/Geschäftsstellen**-Standort, nicht eine Schrottannahme-Fläche; die
  betrieblich benannten Aurubis-Flächen (way/665691214 „Aurubis - Zentrale“, way/666212155 „Aurubis - Werk Ost“)
  tragen `landuse`=industrial. Die Koordinate bezeichnet daher die belegte Geschäftsstellen-Anschrift und **keine**
  verifizierte Annahme- oder Waagenstelle; `phone` bleibt die Zentrale, `dropoff`/`pickup` bleiben leer, `status`
  **aktiv** bleibt unverändert.]
