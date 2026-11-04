---
slug: sl-saarbrucken-malstatt-thomanek-schrott-und-metall
lat: 49.2398497
lon: 6.9774239
clear_fields: 'dropoff_json'
name: Thomanek Schrott und Metall
trader_type: schrotthaendler
state: SL
city: Saarbrücken-Malstatt
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf im Saarland (SL) — Recherche
provenance_ankauf_raw: ja (lt. Verzeichnis)
provenance_origin: table
---

# Thomanek Schrott und Metall

## Überblick

Ein Gelbe-Seiten-Eintrag führt „Thomanek Schrott und Metall“ in der Ludwigstr. 58, 66115 Saarbrücken-Malstatt, mit 0176 84739707 und den Leistungen Metallrecycling, Schrott, Containerservice und Kabelrecycling. Das bleibt ein einzelner Verzeichnis-Lead; Betreiber-, Register- oder kommunaler Zweitbeleg fehlt. Ein möglicher Bezug zu Grün Thomanek in Völklingen-Fürstenhausen ist nicht nachgewiesen. Status daher `pruefung`; Adresse, Telefon und Leistungen nicht als verifizierte Stammdaten behandelt. **Ankauf:** keine bestätigten Kurse; **Verkauf:** keine Preisliste; **Gebühren:** kein Tarif. Offen: Betreiberidentität, aktuelle Annahme und mögliche Verbindung zum Völklinger Betrieb; GEO erst danach.

## Timeline

### Owner-Abgleich 26.10.2026

- [Owner-Abgleich 26.10.2026: Gelbe Seiten führt weiterhin nur einen nicht inhaberbestätigten Eintrag „Thomanek Schrott und Metall“, Ludwigstr. 58, 0176 84739707, ohne Website oder Bewertungen. YellowMap führt dieselbe Mobilnummer zusätzlich für „A & W Recycling“ an Klausenerstr. 18 in Saarbrücken; das ist ein konkurrierender Verzeichnisdatensatz, kein Beleg für gemeinsame Inhaberschaft oder dafür, welcher Eintrag aktuell ist. Der getrennte Gelbe-Seiten-Eintrag „Grün Thomanek Metallrecycling UG“ nennt Saarwiesenweg 2 in Völklingen und eine andere Festnetznummer; eine Verbindung nach Malstatt ist nicht belegt. Daher kein Kontakt-/Adress-Fill und `status: pruefung` bleibt angemessen. Nominatim reverse-geocodiert den Altpunkt erneut auf Ufergasse/St. Johann; die Suche nach Ludwigstr. 58 findet nur einen Gebäude-/Wohnadresspunkt und einen anderen OSM-Ortseintrag, keine Thomanek-Annahme. `clear_fields: geo` bleibt richtig; kein Ersatzpunkt. Seed-Hinweis: leere Straße/PLZ überschreiben die alten Kernadresswerte, leeres Telefon oder leere Annahme-JSON löschen gespeicherte Enrichmentwerte dagegen nicht. Da keine Produktion abgefragt wurde, ist ein möglicher Alt-Telefon-/Annahmeclaim nicht verifiziert; nur bei einem konkret vorhandenen, falschen Altwert wäre ein eigener Clear zu prüfen. Quellen: https://www.gelbeseiten.de/gsbiz/c7fef7ea-8eed-46eb-a47a-ff182a9c98d1 ; https://www.yellowmap.de/Details/4q2hykSllPeZaUWb5ujCfQ==.aspx ; https://www.gelbeseiten.de/gsbiz/210cc1d9-2789-4d87-8a06-28fe99c2e2c2 ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.2354125&lon=6.9932063&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=Ludwigstra%C3%9Fe%2058%2C%2066115%20Saarbr%C3%BCcken%2C%20Germany&addressdetails=1&limit=5 ; `docs/trader-seed.md`.]

### Owner-Korrektur 26.10.2026

- [Owner-Korrektur 26.10.2026: Die im vorherigen Owner-Abgleich offene
  Produktionsfrage wurde read-only geklärt: `public.db` enthält für diesen
  Slug weiterhin `dropoff_json={"allowed":true}` und den inzwischen
  zurückgenommenen Aktivstatus. Der aktuelle Seed lässt das leere
  `dropoff_json` ohne Clear bestehen. Die Betreiber-/Register-/Kommunalprüfung
  findet aber nur nicht inhaberbestätigte Verzeichnis-Leads; weder der
  Malstatter Eintrag noch die widersprüchliche Nummernzuordnung zu A & W oder
  der getrennte Völklinger Rechtsträger belegt öffentliche Annahme in Malstatt.
  Deshalb ergänzt der Owner `clear_fields: 'dropoff_json,geo'`: der nächste
  reguläre Seed setzt die gespeicherte Zusage auf unbekannt `{}` und entfernt
  den falschen Punkt, nicht auf `allowed:false`; das behauptet weder
  Annahmeverweigerung noch Schließung. Status `pruefung` bleibt unabhängig
  davon bestehen. Keine Produktionsdaten geändert. Quellen und Grenzen:
  Read-only `public.db` (26.10.2026), die im Owner-Abgleich direkt geprüften
  Verzeichnis-Leads und [`docs/trader-seed.md`](../../docs/trader-seed.md).]

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 49.2354125, 6.9932063. Nominatim-Reverse löst den Punkt in der Ufergasse in Saarbrücken-St. Johann auf, nicht an der Ludwigstraße 58 in Malstatt. `clear_fields: 'geo'` entzieht den stadtteilfalschen Altpunkt beim nächsten regulären Seed. Betreiber- und Annahmestelle bleiben ungeklärt; keine Ersatzkoordinate. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.2354125&lon=6.99320625&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026: Statuskorrektur

- [Recherche 25.10.2026: Die direkt gelesene Gelbe-Seiten-Seite führt Thomanek Schrott und Metall, Ludwigstr. 58, 66115 Saarbrücken-Malstatt, Tel. 0176 84739707; als Leistungen nennt sie Metallrecycling, Schrott, Containerservice und Kabelrecycling. Das ist ein Einzelbeleg eines Aggregators und bestätigt weder Betreiber noch aktuelle Kundenannahme. Der zuvor auf `aktiv` gesetzte Status ist deshalb auf `pruefung` korrigiert; keine Schließung abgeleitet und keine Kontakt-/Adress-Fills vorgenommen. Mögliche Verbindung zu Grün Thomanek in Völklingen bleibt offen. ANKAUF: keine belegten Kurse; VERKAUF: keine Liste; GEBÜHREN: keine Tarife. GEO erst nach Betreiber-/Standortbestätigung. Quelle: https://www.gelbeseiten.de/gsbiz/c7fef7ea-8eed-46eb-a47a-ff182a9c98d1 .]

### Importiert (Seed-Stand 2026-09-30)

- Schrotthandel (evtl. verwandt mit Nr. 10)

### Recherche 02.10.2026

- [Recherche 02.10.2026: Klärfall ohne Fill (nur Timeline) — nur GelbeSeiten-Einzelbeleg (Ludwigstr. 58, 66115 Saarbrücken-Malstatt, Tel 0176 84739707); keine Betreiber-Website/HR-Zweitquelle; mögl. Bezug Grün Thomanek (Völklingen-Fürstenhausen) ungeklärt; kein Fill; Quelle(n): gelbeseiten.de (eine Quelle).]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall — Lead Saarbrücken-Malstatt, Ludwigstr. 58 nur Verzeichnisbeleg (Einzelbeleg, unsicher); keine Website gefunden; mögl. verwandt mit Grün Thomanek (Völklingen-Fürstenhausen, Saarwiesenweg 2); keine Frontmatter-Fills; Quelle(n): recherche/sl.md]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **49.2398497/6.9774239** (Mittel der 26 eindeutigen Knoten von
  way/631153200; der Punkt liegt innerhalb der Objekt-BBox
  49.2397211/6.9772222–49.2399304/6.9776023). Quelle 1 (OSM, live geladen am 04.11.2026):
  **way/631153200** trägt `addr:street`=Ludwigstraße, `addr:housenumber`=**58**,
  `addr:postcode`=66115, `addr:city`=Saarbrücken und `building`=apartments - Straße, Hausnummer,
  PLZ und Ort stimmen mit der Dossier-Adresse **Ludwigstr. 58, 66115 Saarbrücken-Malstatt** überein.
  Das Objekt trägt **weder `name=` noch `operator=`**; es ist die **Adressangabe** eines
  Mehrparteienhauses, kein Betriebs-POI. Quelle 2 (im Dossier bereits zitiert, live nachgelesen am
  04.11.2026): der Gelbe-Seiten-Eintrag
  https://www.gelbeseiten.de/gsbiz/c7fef7ea-8eed-46eb-a47a-ff182a9c98d1 liefert HTTP 200 und führt im
  LocalBusiness-Block **„Thomanek Schrott und Metall“, streetAddress „Ludwigstr. 58“, postalCode
  „66115“, addressLocality „Saarbrücken“, Telefon „0176 84 73 97 07“, keywords „Metallrecycling,
  Schrott, Containerservice, Kabelrecycling“** mit der Kartenposition **49.239857/6.977358** - die
  **rund 5 m** vom gesetzten OSM-Punkt entfernt liegt und die Anschrift damit unabhängig bestätigt.
  **Fensterprüfung (kein Fensterfehler):** das am 04.11.2026 live abgerufene Kachelraster umfasst
  **6.9700/49.2300 bis 6.9800/49.2500**, **2 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**,
  **35.742 Elemente**; das Fenster enthält die gesuchte Straße **„Ludwigstraße“ nachweislich** mit
  **11 Straßenobjekten** (way/15701142, way/117335403, way/218647746, way/652569857 bis
  way/652569861, way/870818315, way/1039640491, way/1040623881, way/1490834012; residential bzw.
  primary_link) sowie **42 Adressobjekten** auf „Ludwigstraße“ mit **42 verschiedenen Hausnummern**
  (3–36 sowie 54, 55, 57, **58**, 59, 60, 61, 65, 67–73) - die **Dossier-Hausnummer 58 existiert** und
  ist im Raster von **zwei** Objekten belegt: **way/631153200** (Gebäude, nur Adresse) und
  **node/6105056583** mit `name`=**Tanzstudio Fortuna**, `leisure`=dance, `operator`=HBS GmbH und
  `contact:website`=https://www.tanzstudio-fortuna.de/ - das ist ein **anderer Betrieb in derselben
  Hausnummer** (34 m entfernt); der gesetzte Punkt ist das **Gebäudeobjekt mit der Anschrift**.
  **Negativaussage, qualifiziert:** im Raster tragen **1.672 Objekte** `name=` oder `operator=`,
  **mit Bezug zu „Thomanek“, „Schrott“, „Metallrecycling“, „Containerdienst“ oder
  „Kabelrecycling“** trifft es auf **kein** Objekt dieser Adresse - der Betrieb ist in OSM **nicht**
  kartiert. **Zurücknahme des `geo`-Tokens (Begründung):** `clear_fields: 'dropoff_json,geo'` wurde auf
  **`'dropoff_json'`** gekürzt - das Token `geo` ist **entfernt**, weil es den am 26.10.2026
  zurückgenommenen und nachweislich falschen Produktionspunkt (49.2354125/6.9932063 = Ufergasse,
  Saarbrücken-St. Johann) fernhalten sollte; das Token `dropoff_json` bleibt **unverändert** bestehen,
  weil die allgemeine Annahmezusage weiterhin nicht belegt ist. `clear_fields` darf nicht
  gleichzeitig `geo` neben belegtem `lat`/`lon` stehen. **Qualifizierung (unverändert übernommen):**
  Der Punkt bezeichnet die **Adresse eines einzelnen Verzeichnis-Leads**, **nicht** einen verifizierten
  Annahmehof, eine Waage oder eine öffentliche Schrottannahme; der im Dossier offene Punkt der
  fehlenden zweiten unabhängigen Quelle, die widersprüchliche Zuordnung derselben Mobilnummer zu
  „A & W Recycling“ (Klausenerstr. 18) und der mögliche Bezug zum getrennten „Grün Thomanek
  Metallrecycling UG“ (Saarwiesenweg 2, Völklingen) bleiben unverändert, ebenso `status: pruefung`,
  `street`, `postcode`, `phone`, `email`, `opening_hours`, `website`, `website_status` und
  `description`. Es wird kein Ankaufkurs, keine Preisliste und keine Gebühr behauptet. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/631153200/full.json ;
  https://api.openstreetmap.org/api/0.6/node/6105056583.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=6.9700,49.2300,6.9800,49.2500 ;
  https://www.gelbeseiten.de/gsbiz/c7fef7ea-8eed-46eb-a47a-ff182a9c98d1 (Abruf 04.11.2026).]
