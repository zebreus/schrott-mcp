---
slug: bb-furstenwalde-graunke-m-segeth
name: Autoverwertung L.S. Graunke (Autoteilezentrum L.S. Graunke)
trader_type: autoverwertung
state: BB
city: Fürstenwalde/Spree
street: Karl-Liebknecht-Str. 30
postcode: '15517'
phone: 03361 50348
email: info@graunke24.de
opening_hours: Mo-Fr 08:00-12:00, 13:00-17:00
website: https://graunke24.de
website_status: aktiv
status: aktiv
description: 'Autoverwertung L.S. Graunke / Autoteilezentrum L.S. Graunke, Karl-Liebknecht-Straße 30, 15517 Fürstenwalde/Spree, Telefon 03361 50348. Der Abfallwirtschaftsplan des Landes Brandenburg führt an dieser Anschrift eine "PKW-Abstellfläche/Zwischenlager für max. 850 PKW" mit dem Anlagentitel "Autoverwertung L.S. Graunke" und dem Betreiber "Autoteilezentrum L.S. Graunke". Das OSM-Objekt an der Dossier-Hausnummer trägt name=Autoteilezentrum L.S.Graunke, operator=L.S. Graunke, shop=car_parts, email=info@graunke24.de und opening_hours=Mo-Fr 08:00-12:00, 13:00-17:00. Ankaufpreise für Fahrzeuge und Ankaufpreise für Schrott werden nicht mit Zahlen veröffentlicht.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Graunke (M. Segeth)

## Überblick

Autoverwertung L.S. Graunke / Autoteilezentrum L.S. Graunke, Karl-Liebknecht-Straße 30,
15517 Fürstenwalde/Spree, Telefon 03361 50348, E-Mail info@graunke24.de. Der Abfallwirtschaftsplan
des Landes Brandenburg führt an dieser Anschrift eine "PKW-Abstellfläche/Zwischenlager für max.
850 PKW" unter dem Anlagentitel "Autoverwertung L.S. Graunke" mit dem Betreiber "Autoteilezentrum
L.S. Graunke". Das OSM-Objekt an der Dossier-Hausnummer 30 trägt `name=Autoteilezentrum
L.S.Graunke`, `operator=L.S. Graunke` und `shop=car_parts`. Die im Dossier geführte Domain
autoverwertung-graunke.de antwortet mit HTTP 403 und wird nicht als belegte Website geführt;
belegt ist graunke24.de. Ankauf- und Verkaufspreise werden nicht mit Zahlen veröffentlicht.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „n.e.“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Importiert (Seed-Stand 2026-09-30)

- AV/Ersatzteile, nur eBay-Präsenz
- Adresse: Fürstenwalde, Karl-Liebknecht-Str. 30
- Adressbeleg: Seed-notes; Website nur eBay

### Recherche 30.10.2026

- [Recherche 30.10.2026: Die Identität der Zeile wird erstmals amtlich und objektbezogen belegt; zugleich weicht der Seed-Name von der belegten Firmierung ab. Der Abfallwirtschaftsplan des Landes Brandenburg, Fortschreibung 2023, Teilplan Gefährliche Abfälle, führt für Oder-Spree den Eintrag "254. Oder-Spree PKW-Abstellfläche/Zwischenlager für max. 850 PKW, Autoverwertung L.S. Graunke, Karl-Liebknecht-Straße 30, 15517 Fürstenwalde/Spree" mit der Betreiberanschrift "Autoteilezentrum L.S. Graunke, Karl-Liebknecht-Straße 30, 15517 Fürstenwalde/Spree"; dieselbe Anlage erscheint im Teilplan Gefährliche Abfälle zusätzlich unter Nr. 227 als "Lagerung von Schrott GRB Gesellschaft für Rohstoffrückgewinnung Brandenburg mbH Berliner Straße 24, 15890 Eisenhüttenstadt" (anderer Betrieb, anderer Ort). Zweite Primärquelle ist OSM: Das Objekt way/179160055 trägt an der Dossier-Hausnummer 30 wörtlich `name=Autoteilezentrum L.S.Graunke`, `operator=L.S. Graunke`, `addr:housenumber=30`, `addr:street=Karl-Liebknecht-Straße`, `addr:postcode=15517`, `addr:city=Fürstenwalde/Spree`, `building=yes`, `shop=car_parts`, `email=info@graunke24.de`, `website=https://graunke24.de`, `opening_hours=Mo-Fr 08:00-12:00, 13:00-17:00`; das Nachbarobjekt way/617981168 trägt `landuse=commercial` und `name=Autoteilezentrum L.S.Graunke`. Beide Objekte nennen also den Betrieb selbst und sind **positive** Belege. Ein dritter Beleg: Das Branchenverzeichnis auto-werkstatt.de führt "Autoteilezentrum L.S. Graunke" mit "Kontakt 03361 50348, Karl-Liebknecht-Straße 30, 15517 Fürstenwalde/Spree, Domain graunke24.de (im Verzeichnis ohne Protokoll notiert)". Daraus wurden der Name auf "Autoverwertung L.S. Graunke (Autoteilezentrum L.S. Graunke)", city von "Fürstenwalde" auf "Fürstenwalde/Spree", phone auf "03361 50348", email auf "info@graunke24.de", opening_hours auf "Mo-Fr 08:00-12:00, 13:00-17:00" und website von "https://autoverwertung-graunke.de" auf "https://graunke24.de" gesetzt, website_status auf aktiv angehoben und status von pruefung auf **aktiv** gesetzt; PLZ und Straße bleiben unverändert, der Slug bleibt aus Stabilitätsgründen unverändert. Begründung der Namensänderung: Der Seed-Name "Graunke (M. Segeth)" ist in keiner der Quellen auffindbar; amtlicher Plan, OSM-Objekt und Branchenverzeichnis nennen übereinstimmend "L.S. Graunke" beziehungsweise "Autoverwertung L.S. Graunke" und "Autoteilezentrum L.S. Graunke"; ein Bezug auf "M. Segeth" ist nicht belegt. Der Seed-Name bleibt in der Historie erhalten. Der Klärfall "nur eBay-Präsenz" vom Seed ist damit ausgeräumt: die eBay-Angebote tragen dieselbe Anschrift und dieselbe Domain-Familie (graunke24.de). Zur Domain: autoverwertung-graunke.de antwortet live mit HTTP 403 und ist daher als Website nicht belegt; atz-graunke.de leitet per JavaScript-Weiterleitung auf graunke24.de um. Zu Ankauf, Verkauf und Gebühren getrennt: ANKAUF - die amtliche Anlagenbezeichnung "Autoverwertung L.S. Graunke" und die Einstufung als "PKW-Abstellfläche/Zwischenlager für max. 850 PKW" belegen das Ankaufs- und Lagergeschäft für Fahrzeuge; ein Zahlenpreis für Fahrzeugankauf wird nicht veröffentlicht. VERKAUF - der Betrieb tritt am Markt als Autoteilezentrum mit `shop=car_parts` und als Anbieter von Autoteilen und Ersatzteilen auf; ein Verkaufspreis für Ersatzteile oder ein Preissortiment wird nicht veröffentlicht. GEBÜHREN - keine Gebührentafel und keine Annahmegebühr veröffentlicht; die Mittagspause 12:00-13:00 in den Öffnungszeiten ist eine Öffnungszeit-Angabe und keine Gebühr. OSM-Abgleich live (Rasterfenster um den Adresspunkt 52.3784414/14.0759769, bbox 14.0719769,52.3744414 bis 14.0799769,52.3824414; Straße "Karl-Liebknecht-Straße"): 5 Adressobjekte mit `addr:housenumber` und `addr:street` gleich Karl-Liebknecht-Straße, daraus **5 verschiedene Hausnummern** (23, 25, 27, 30, 36); die Dossier-Hausnummer 30 ist darunter. Im Raster tragen 74 Objekte `name=` oder `operator=`; **2** davon nennen den Betrieb (way/179160055 und way/617981168, beide mit `name=Autoteilezentrum L.S.Graunke`). Das ist eine Aussage zum OSM-Bestand und kein Negativbeleg gegen die Betreiberanschrift. Quelle(n): https://bravors.brandenburg.de/verwaltungsvorschriften/awp_gefaehrliche_abfaelle_2024 ; https://www.auto-werkstatt.de/fuerstenwalde-spree/autoteilezentrum-l-s-graunke-aUPmpQ ; https://graunke24.de ; https://autoverwertung-graunke.de ; https://www.atz-graunke.de/ ; OSM-API-Abfrage api.openstreetmap.org/api/0.6/map.json?bbox=14.0719769,52.3744414,14.0799769,52.3824414 (Abruf 30.10.2026)]

- [30.10.2026 (Korrekturvermerk zur Telefon-Direktive): Die `clear_fields: 'phone'`-Direktive aus der Owner-Korrektur vom 28.10.2026 ist mit dem in dieser Recherche belegten Telefonwert nicht mehr vereinbar und wurde deshalb aus dem Frontmatter entfernt. Der Seed-Codegen bricht den Build ab, wenn ein Clear-Token und ein belegter Wert im selben Feld stehen ("clear_fields conflicts with populated phone"), und `docs/trader-seed.md` verlangt für jeden Clear-Token ein leeres Feld. Die Direktive war auf den Import-Platzhalter "n.e." gerichtet; jetzt steht stattdessen eine belegte Nummer im Feld. Die betroffene Owner-Notiz vom 28.10.2026 bleibt **wortgleich und unverändert** in der Historie stehen. Quelle(n): docs/trader-seed.md (Semantik der `clear_fields`-Direktive); crates/ingestion/build.rs (Konfliktprüfung clear_fields gegen belegte Felder, Abruf 30.10.2026)]
