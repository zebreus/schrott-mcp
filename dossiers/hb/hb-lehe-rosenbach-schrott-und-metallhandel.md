---
slug: hb-lehe-rosenbach-schrott-und-metallhandel
name: Rosenbach Schrott und Metallhandel
trader_type: metallhaendler
state: HB
city: Lehe
street: Lange Str. 30
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
clear_fields: 'phone'
provenance_section: Bremerhaven
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Rosenbach Schrott und Metallhandel

## Überblick

Für „Rosenbach Schrott und Metallhandel“ liegen weiterhin nur zwei widersprüchliche Verzeichnis-Leads vor: **Lange Str. 30** (Lehe) und **Spadener Str. 53** (Bremerhaven). Keine Betreiber- oder Registerquelle bestätigt Inhaber, Annahmestelle oder ob beide Adressen überhaupt denselben Betrieb betreffen.

**Preise:** keine belegte Ankaufpreisliste, Verkaufspreisliste oder Gebührenordnung. **Geocoding:** beide Straßen sind getrennte Kandidaten; weder auswählen noch geocodieren, bis Betreiber und Geschäftsadresse geklärt sind.

**Stand 29.10.2026:** Der Adresskonflikt bleibt und ist jetzt beidseitig geprüft. Die Schrotthandel-Branchenliste von Das Telefonbuch für Bremerhaven führt den Betrieb als "Rosenbach Schrott und Metallhandel, Spadener Str. 53, 27578 Bremerhaven, Tel. 0471 30 71 38 74", das Örtliche und Gelbe Seiten nennen dagegen Lange Str. 30 in Lehe. In OSM existiert die Hausnummer 30 auf der Lange Straße (Bremerhaven-Lehe) als Gebäude (way/258362969, `building=house`, `addr:postcode=27580`), die Hausnummer 53 auf der Spadener Straße dagegen **nicht**: die Spadener Straße trägt dort 26 Adressobjekte mit 26 verschiedenen Hausnummern, darunter 1, 1a, 2, 5, 6, 7, 9, 10, 12, 14, 17 und 18, aber keine 53. Ein Objekt mit Bezug zu Rosenbach oder Schrott existiert in keinem der beiden Kartenausschnitte. Zusatzkonflikt: die Verzeichnisse nennen für beide Adressen die PLZ 27578, die OSM-Adressobjekte beider Straßen tragen 27580. Der gespeicherte GEO-Punkt 53.6707/8.1785 löst per Nominatim auf die Landesgrenze zu Niedersachsen auf (relation/62771) und liegt damit nicht in Bremerhaven. **Ankauf / Verkauf / Gebühren:** keine Preise und keine Tarife belegt.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „n.e.“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 06.10.2026

- [Recherche 06.10.2026: Erneute Prüfung ergab keinen unabhängigen Betreiber-/Registerbeleg; bestehender Adresskonflikt bleibt: Gelbe Seiten führt Lange Str. 30, 27578 Lehe, Tel. 01575 3648535; Das Telefonbuch Spadener Str. 53, 27578 Bremerhaven, Tel. 0471 30713874. Beide Angaben sind nur Leads und können nicht zu einem gemeinsamen Betrieb zusammengeführt werden. ANKAUF/VERKAUF/GEBÜHREN: keine verifizierten Zahlen oder Tarife. Geocoding für beide Kandidaten ausgesetzt. Quelle(n): https://www.gelbeseiten.de/ ; https://www.dastelefonbuch.de/ (Verzeichnis-Leads)]

### Importiert (Seed-Stand 2026-09-30)

- Schrott-/Metallhandel (klein)
- Adresse: Lehe, Lange Str. 30
- Adressbeleg: Seed-notes (Verzeichnis); PLZ n. verifiziert

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall – kein belastbarer Betreiber-/Register-Doppelbeleg; Northdata-Titelcheck "Rosenbach, Bremerhaven" nur Suche ohne HR-Treffer; Seed-Adresse Lange Str. 30 (Lehe) bleibt Einzelbeleg; Quelle(n): keine belegfähige Quelle]
- [Recherche 01.10.2026: Felder leer gelassen; Statusfeld unverändert; Quelle(n): siehe Vorbullet]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Klärfall mit Adresskonflikt — zwei Aggregator-Leads mit verschiedenen Adressen: Gelbe Seiten (Lange Str. 30, 27578 Bremerhaven-Lehe, 01575 3648535) vs. Das Telefonbuch (Spadener Str. 53, 27578 Bremerhaven, 0471 30713874). Aggregatoren sind Leads, kein Beleg; Adresskonflikt verhindert jeden Fill. Kein Frontmatter-Fill; Folgewelle: Vor-Ort-/Betreiber-Quelle. Quelle(n): gelbeseiten.de + dastelefonbuch.de (Leads, Abruf 02.10.2026)]

### Recherche 29.10.2026

- [Recherche 29.10.2026: Adresskonflikt beidseitig live geprüft. Das Telefonbuch führt in seiner Schrotthandel-Branchenliste für Bremerhaven den Eintrag "Rosenbach Schrott und Metallhandel, Spadener Str. 53, 27578 Bremerhaven, Tel. 0471 30 71 38 74"; das Örtliche und Gelbe Seiten nennen Lange Str. 30 in Lehe. Beides bleibt Aggregator-Lead ohne Betreiber- oder Registerbeleg, eine Zusammenführung ist damit weiterhin nicht belegbar. OSM-Prüfung an beiden Kandidaten: Auf der Lange Straße in Bremerhaven-Lehe existiert die Hausnummer 30 als Gebäude (way/258362969, `building=house`, `addr:housenumber=30`, `addr:street=Lange Straße`, `addr:postcode=27580`, `addr:city=Bremerhaven`); die Lange Straße trägt dort 89 Adressobjekte mit 84 verschiedenen Hausnummern. Auf der Spadener Straße existiert die Hausnummer 53 **nicht**: dort stehen 26 Adressobjekte mit 26 verschiedenen Hausnummern (1, 1a, 2, 5, 6, 7, 9, 10, 12, 14, 17, 18 und weitere), die 53 fehlt. In beiden Kartenausschnitten existiert kein Objekt mit Bezug zu Rosenbach oder Schrott. PLZ-Konflikt: die Verzeichnisse nennen 27578, die OSM-Adressobjekte beider Straßen tragen 27580; das Frontmatter-Postcode bleibt deshalb leer. GEO: der gespeicherte Punkt 53.6707/8.1785 löst per Nominatim auf Niedersachsen (administrative Grenze relation/62771) auf und liegt nicht in Bremerhaven; er wird hier nicht ersetzt, weil die Anschrift ungeklärt ist. ANKAUF: keine Preise belegt. VERKAUF: keine Preisliste. GEBÜHREN: keine Tarife. Quelle(n): https://www.dastelefonbuch.de/Branchen/Schrotthandel/Bremerhaven (Abruf 29.10.2026); OSM-API way/258362969 und Kartenausschnitt Spadener Straße (Abruf 29.10.2026); Nominatim-Reverse (Abruf 29.10.2026)]

- [30.10.2026 (Korrekturvermerk): **Zählung der Lange Straße berichtigt**: die Nachmessung am 30.10.2026 in PLZ 27580 Bremerhaven ergibt **89** statt 41 Adressobjekte und **84** statt 39 verschiedene Hausnummern (die Werte 141, 142, 143, 144 und 145 entstammen der Bereichsangabe „141-145“ (node/302934606); löst man die Bereichsnotation auf, sind es 85 Einzelwerte). Raster: bbox 8.5868674,53.5636599,8.5959295,53.5715419 (Ausdehnung der Adressobjekte der Lange Straße in PLZ 27580 Bremerhaven), Strasse „Lange Straße“, Filter `addr:postcode=27580`. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=8.5868674,53.5636599,8.5959295,53.5715419 (Abruf 30.10.2026)]

### Recherche 20.11.2026 (Welle 1800 C)

- [Recherche 20.11.2026: **Das amtliche Fachbetrieberegister kennt den Betrieb nicht, und das amtliche Handelsregister führt unter dem Suchbegriff keinen passenden Rechtsträger - die beiden Verzeichnis-Adressen bleiben ungeklärt.** Die Namenssuche am 20.11.2026 liefert für „Rosenbach“ in der Route „Entsorgungsfachbetriebe“ (`.../Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Rosenbach`) **0 Treffer** und in der Route „Altfahrzeugverwertung“ **0 Treffer**; die Suche im amtlichen Handelsregister unter „Rosenbach Schrott“ liefert **keine Unternehmen und keine Personen**. Die Radiusabfrage mit 3 km um die Kandidatenadresse Lange Straße 30, 27580 Bremerhaven (53.5700017/8.5945512) liefert in der Route „Entsorgungsfachbetriebe“ **17 Zeilen**, darunter Dapprich & Gerlach GmbH (Werkstraße 2, 27568 Bremerhaven, 0,5 km), REDUX Recycling GmbH (Batteriestraße 94, 27568 Bremerhaven), Werner Eulig GmbH (Langmirjen 21, 27578 Bremerhaven), Karl Nehlsen GmbH & Co. KG (Franziusstraße 96 und Brückenstraße 25, 27568 Bremerhaven), UTG Unabhängige Tanklogistik GmbH (Barkhausenstr. 37, 27568 Bremerhaven), BEG logistics GmbH und Richard Bauer Rohstoff-Handels GmbH & Co. KG (Zur Hexenbrücke 16, 27570 Bremerhaven), Bremerhavener Entsorgungsgesellschaft mbH (Zur Hexenbrücke 16, 27570 Bremerhaven) - **keine Zeile nennt Rosenbach**. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Lange%20Stra%C3%9Fe%2030%2C%2027580%20Bremerhaven%2C%20Deutschland;8.5945512;53.5700017;8.6145512;53.5900017;8.5745512;53.5500017&dst=3&is=1&e=1 ; https://www.online-handelsregister.de/suche/Rosenbach%20Schrott (Abruf 20.11.2026).]
- [Recherche 20.11.2026: **Die Nachmessung der Hausnummer 30 bestätigt die frühere Erfassung und schließt den Adresskonflikt nicht.** Das Gebäude **way/258362969** trägt `addr:housenumber=30`, `addr:street=Lange Straße`, `addr:postcode=27580`, `addr:city=Bremerhaven`, `building=house`; es trägt **kein** `name=`, **kein** `operator=` und **keinen** Recycling- oder Schrott-Tag, und die Hausnummer 53 auf der Spadener Straße existiert in OSM nach wie vor nicht. Der Adresskonflikt zwischen Lange Str. 30 (Lehe) und Spadener Str. 53 (Bremerhaven) bleibt damit unverändert: beide Angaben stammen aus Aggregatoren, und keine Betreiber- oder Registerquelle bestätigt Inhaber, Annahmestelle oder die Zuordnung beider Adressen zu einem Betrieb. **ANKAUF:** keine Preise belegt. **VERKAUF:** keine Preisliste belegt. **GEBÜHREN:** keine Tarife belegt. `status` bleibt `pruefung`, `street` und `postcode` bleiben unverändert, und der gemessene Punkt 53.6707/8.1785 (außerhalb Bremerhavens) wird nicht eigenmächtig ersetzt. Quelle(n): https://api.openstreetmap.org/api/0.6/way/258362969/full.json (Abruf 20.11.2026).]
