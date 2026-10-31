---
slug: hh-hafencity-stena-recycling
name: Stena Recycling GmbH
trader_type: sonstige
state: HH
city: Hamburg-Altstadt
street: Hermannstrasse 22
postcode: '20095'
phone: ''
email: ''
opening_hours: ''
website: https://stenarecycling.com
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: Ankauf anzufragen
provenance_origin: table
---

# Stena Recycling GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- FE/NE, Logistik (lt. Aggregator)

### Korrektur 30.09.2026 (Feedback-Triage)

- [Korrektur 30.09.2026 (Feedback-Triage): Feedback 1727 bestaetigt — Hermannstrasse 22, 20095 = Hamburg-Altstadt (nicht HafenCity 20457); Stadt/Strasse/PLZ korrigiert, Website bleibt aktiv. Quelle: https://stenarecycling.com (Filialfinder).]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Hermannstraße, Hamburg-Altstadt).** Straßen-Raster über den **gesamten** Straßenverlauf (Straßenbbox 9.9960541/53.5517041 bis 9.9973350/53.5524459), Scan-Fenster **9.9940541/53.5497041 bis 9.9993350/53.5544459** (**1 Kachel im 0,01°-Raster des Repo-Werkzeugs `scripts/release/osm_street_scan.py`, 0 Kachel-Fehler**). Zählung getrennt: **17 Adressobjekte / 13 verschiedene Hausnummern** auf der Straße (7, 9, 10, 11, 12, 13, 14, 15, 16, 18, 22, 40, 46) - die Aufzählung nennt exakt die live vorhandenen Nummern; Filter-Key `addr:street=Hermannstraße` (mit `addr:housenumber` als Hausnummer). Eine **Bereichsnotation** ist live vorhanden: node/5639506521 mit `addr:housenumber=10-14` und `name=Riccardo Retail GmbH, Store Hamburg` löst sich nach der Referenzzählung des Repo-Werkzeugs in **10, 11, 12, 13, 14** auf. Im gesamten Rastergebiet **216 Adressobjekte mit Hausnummer / 145 verschiedene Hausnummern**; im Raster tragen **582 Objekte `name=`** und **189 Objekte `operator=`**. **Die Dossier-Hausnummer 22 existiert**: **node/732351390** mit `addr:housenumber=22`, `addr:street=Hermannstraße`, `addr:city=Hamburg`, `addr:country=DE`, `addr:postcode=20095`, `entrance=staircase` - **ohne** `name=`- und **ohne** `operator=`-Tag. Objekte mit Bezug zum Betrieb im Raster: **keines** - die Suche nach „stena“, „recycling“, „metall“ und „schrott“ über die Felder name, operator, description, website, contact:website, note, contact:email, email, short_name, brand und operator:name findet **keinen** Treffer mit Betriebsbezug zu Stena Recycling; **in keinem** Objekt des Rasters trägt ein `name=`- oder `operator=`-Wert den Bestandteil „Stena“. Drei Objekte tragen zwar `addr:housenumber=22`, liegen aber auf **anderen Straßen** und haben **keinen** Bezug zur Dossier-Adresse: node/8636604444 mit `name=Generalkonsulat der Republik Argentinien in Hamburg` und way/39859197 mit `name=Rathaushörn` (beide `addr:street=Mönckebergstraße`) sowie way/982084024 mit `name=Perle Hamburg` (`addr:street=Spitalerstraße`). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9940541,53.5497041,9.9993350,53.5544459 (Kachelraster, live abgerufen 31.10.2026) ; https://api.openstreetmap.org/api/0.6/node/732351390.json ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Betreiber-Primärquelle bestätigt die Adresse wörtlich und liefert Kontaktdaten.** Der **Filialfinder** der deutschen Stena-Recycling-Website (Seite „Eine Niederlassung suchen“) listet: **„Stena Recycling GmbH / Deutschland / Hermannstrasse 22, 20095 Hamburg / Auf Karte anzeigen / +49 160 9796 5289 / info@stenametall.de / Zur Website“**. Damit sind **Adresse, Telefon und E-Mail** belegt; die Bestätigung kommt aus der **Niederlassungsliste des Betreibers selbst**, nicht aus einem Aggregator. Die Einträge der übrigen Länder derselben Liste (Stena Recycling AS Norwegen, Stena Recycling Oy Finnland, Stena Recycling SP z o o Polen, Stena Recycling srl Italien, Stena Metal International Schweden) bestätigen, dass die deutsche Gesellschaft **Stena Recycling GmbH** heißt (nicht „Stena Recycling Deutschland GmbH“). Das **Impressum** der deutschen Seite (Seite „Impressum“) weist darauf hin, dass die EFB-Zertifikate für den Standort **DE-06188 Landsberg** gelten — das ist ein **anderer** Standort und darf **nicht** auf Hermannstraße 22 bezogen werden. Quelle(n): https://www.stenarecycling.com/de/kontaktieren-sie-uns/eine-niederlassung-suchen/ ; https://www.stenarecycling.com/de/ueber-uns/impressum/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Preise und Gebühren: keine öffentliche Preisliste auffindbar.** Die deutsche Stena-Website ist eine **Unternehmens-/Leistungsseite** (Was wir bieten: Wertstoffrecycling, Branchenlösungen, Recycling-Services) **ohne** Ankaufs- oder Verkaufspreisliste; eine Suchabfrage nach Zahlenpreisen auf den Seiten „Wertstoffrecycling“, „Branchenlösungen“, „Recycling-Services“ und „Zirkuläre Elektronik“ ergab **keine** bezifferte Preisposition. **ANKAUF:** die Leistungsbeschreibung „Material recycling“ / „Recycling services“ belegt das **Ankaufgeschäft als Leistungsfeld des Konzerns**, aber **keine** Sortenliste und **keinen** Preis. **VERKAUF:** **keine** Preise, **keine** Produktliste mit Preisen. **GEBÜHREN:** **keine** Gebühr, kein Mindestmenge-, Container- oder Waagengeld genannt. Die EFB-Zertifikate sind hinterlegt, aber ohne Preisbezug. Quelle(n): https://www.stenarecycling.com/de/ ; https://www.stenarecycling.com/de/was-wir-bieten/recycling-services/ ; https://www.stenarecycling.com/de/ueber-uns/impressum/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Korrektur 31.10.2026 (Korrekturvermerk zu den Vorwellen-Notizen).** Die Vorwellen-Notiz zu `provenance_ankauf_raw: Ankauf anzufragen` bleibt wortgleich bestehen; der **Filialfinder** belegt inzwischen die **Adresse** (Hermannstrasse 22, 20095 Hamburg) für die Gesellschaft **Stena Recycling GmbH** und macht die Zeile „Ankauf anzufragen“ für die **Existenz- und Adressfrage** überholt — sie bleibt aber als **Leistungsfrage** offen, weil die Website **keine** Sortenliste und **keine** Preise nennt. Die Vorwellen-Korrektur „Hermannstrasse 22, 20095 = Hamburg-Altstadt (nicht HafenCity 20457)“ ist durch den Filialfinder **bestätigt**. Die im Frontmatter leeren Felder `phone` und `email` sind mit dem Filialfinder **zweifach** belegt (Betreiber-Niederlassungsliste + Filialfinder), `opening_hours` bleibt **leer** (keine Standortzeiten veröffentlicht). Quelle(n): https://www.stenarecycling.com/de/kontaktieren-sie-uns/eine-niederlassung-suchen/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen und Geokodierung.** (a) Der Dossiername „Stena Recycling GmbH“ ist durch den Filialfinder **bestätigt**; die Seed-Provenienz „FE/NE, Logistik (lt. Aggregator)“ ist durch die Website **nicht** konkret belegt — die Leistungsseiten nennen **Wertstoffrecycling / Branchenlösungen / Recycling-Services**, aber **keine** Sorten. (b) **Geokodierungsbedarf:** die Dossier-Adresse **Hermannstrasse 22, 20095 Hamburg** ist in OSM belegt (node/732351390, `addr:housenumber=22`, `addr:street=Hermannstraße`, `addr:postcode=20095`, `entrance=staircase`); der gespeicherte Produktionspunkt **53.5494250/10.0058500** stammt aus einer generischen Altstadt-Koordinate und weicht **721 m** vom belegten Objektpunkt (9.9955021/53.5514865) ab. Quelle(n): https://api.openstreetmap.org/api/0.6/node/732351390.json ; https://www.stenarecycling.com/de/kontaktieren-sie-uns/eine-niederlassung-suchen/ (Abruf 31.10.2026).]
