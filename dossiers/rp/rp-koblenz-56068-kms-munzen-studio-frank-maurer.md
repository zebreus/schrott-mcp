---
slug: rp-koblenz-56068-kms-munzen-studio-frank-maurer
name: KMS Münzen Studio (Frank Maurer)
trader_type: sonstige
state: RP
city: Koblenz
street: Rathauspassage 20
postcode: '56068'
lat: 50.3601177
lon: 7.5971797
phone: 0176 29576381
email: ''
opening_hours: Mo-Fr 10:00-13:00 und 14:00-18:00, Sa 10:00-16:00
website: https://www.koblenzer-muenzen-studio.de
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# KMS Münzen Studio (Frank Maurer)

## Überblick

KMS Koblenzer Münzen Studio von Frank Maurer, einem freien Sachverständigen, ist ein
numismatischer Ankauf und Verkauf in der Rathauspassage 20, 56068 Koblenz. Angekauft werden
Münzen, Orden, Briefmarken und Fachliteratur, verkauft Münzen, Zubehör von Lindner und Safe
sowie numismatische Literatur. Die Identität trägt allein die Betreiber-Kontaktseite mit
Adresse, Rufnummer und Öffnungszeiten, eine zweite unabhängige Quelle fehlt. In OpenStreetMap
existiert die Hausnummer 20, das Objekt trägt aber weder Namen noch Betreiberbezug. Belegte
Gebühr: Die mündliche Angebotserstellung ist kostenfrei, eine schriftliche kostet mindestens 25
Euro inklusive Umsatzsteuer, für Verpackungsmaterial werden 2 Euro Pauschale berechnet. Offen
bleiben die E-Mail-Adresse, die nur durch Cloudflare maskiert sichtbar ist, und die Frage, ob
die Angebotsgebühr je Angebot oder je Vorgang anfällt.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Münzen-/Goldankauf
- Adresse: Koblenz 56068

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Seite live verifiziert (koblenzer-muenzen-studio.de, Jimdo, abgerufen 01.10.2026): KMS Koblenzer Münzen Studio, Frank Maurer (freier Sachverständiger), Rathauspassage 20, D-56068 Koblenz, Tel. 0176-29576381, frank-maurer1@web.de, Öffnungszeiten Mo–Fr 10–13 & 14–18, Sa 10–16; Betreiber-Facebook-Seite existent (Snippet, Inhalte Cloudflare-blockiert) → Adress-Zweitbeleg fehlt → Adresse/Telefon/E-Mail/Zeiten nur Betreiber-Einzelbeleg → (Einzelbeleg, unsicher), NICHT in Frontmatter. website/website_status aktiv gefüllt (Domain-Root, live verifiziert). PERSONEN-LINK: derselbe Frank Maurer betreibt Golden Gate Goldankauf (Entenpfuhl 12) — getrennte Dossiers beibehalten. Klärfall: Koblenz-Fakten requires Zweitbeleg; Quelle(n): https://www.koblenzer-muenzen-studio.de/, https://www.koblenzer-muenzen-studio.de/kontakt-anfahrt/]
- [Korrektur 01.10.2026: Feedback #2984 berechtigt eingearbeitet — city „Koblenz 56068" zu „Koblenz" (PLZ-im-city-Formatfehler), street Rathauspassage 20, postcode 56068, phone 0176 29576381, opening_hours Mo-Fr 10:00-13:00 und 14:00-18:00, Sa 10:00-16:00 per Betreiber-Kontaktseite (Frank Maurer, freier Sachverständiger); E-Mail bleibt begründet leer (Cloudflare-maskiert, frank-maurer1@web.de nicht lesbar verifizierbar — nur Timeline); PERSONEN-LINK Frank Maurer (KMS Rathauspassage 20 vs. Golden Gate Entenpfuhl 12) — getrennte Dossiers beibehalten, kein Merge; Beleglage: Betreiber-Website als starke Einzelquelle (Einzelunternehmen ohne HRB — keine Owner-Ausnahme), offengelegt; Koordinaten neu zu geocodieren; Quelle(n): https://www.koblenzer-muenzen-studio.de/kontakt-anfahrt/ (Abruf 01.10.2026)]

### Recherche 31.10.2026

- [31.10.2026 (Korrekturvermerk): **Abruflimit der OSM-API dokumentiert**: das zitierte Fenster überschreitet live die Grenze von 50 000 Knoten der `map.json`-Schnittstelle (HTTP 400 „too many nodes“); die genannten **13 Objekte / 12 verschiedenen Hausnummern** der Rathauspassage wurden deshalb über die **Kachelmethode** (`scripts/release/osm_street_scan.py`, 0,01°-Kacheln) gemessen und exakt reproduziert. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=7.5868271,50.3496509,7.6070772,50.3701771 (Fenster laut Dossier, Abruf 31.10.2026)]
- [Recherche 31.10.2026: OSM-Adressprüfung live am Objekt, Raster bbox=7.5868271,50.3496509,7.6070772,50.3701771 (OSM-Way-BBox des gesamten Straßenverlaufs "Rathauspassage", way/38986411, plus 0,010 Grad Rand; in Kacheln abgetastet), Filter addr:street=Rathauspassage mit addr:housenumber: **13 Objekte** | **12 verschiedene Hausnummern** (roh 2, 4, 6, 8, 9, 10, 12, 14, 16, 18, 20, 22 — keine Bereichs- oder Buchstabennotation, daher unverändert aufgelöst). Die Dossier-Hausnummer **20 ist erfasst**: node/13258111292 trägt `addr:housenumber=20`, `addr:street=Rathauspassage`, `addr:city=Koblenz`, `addr:postcode=56068` — dieses Objekt trägt **kein** `name=` und **kein** `operator=`. Von den 13 Adressobjekten der Straße trägt genau **1** ein `name=`/`operator=`-Tag: node/10743121894 mit `addr:housenumber=10`, `addr:street=Rathauspassage`, `name=Fahrschule Clauser`, `amenity=driving_school` — **kein** Bezug zur Dossier-Firma. Im gesamten Raster tragen 3544 Objekte `name=` oder `operator=`; Objekte mit Bezug zu **Schrott/Metall/Recycling/Edelmetall** sind dort **7** (Kriterium: `shop=gold_buyer` | `amenity=recycling` | `industrial=` | `landuse=industrial` | `recycling:` oder ein Name/Operator mit Schrott-/Recycling-Bezug, zusätzlich ein Name mit Goldankauf-/Münz-Bezug): node/13648283754 `name=Koblenzer Goldankauf` mit `shop=gold_buyer` (Position 7.5938883/50.3570522, also auf der Schloßstraße, **ohne** `addr:housenumber`), node/13587502944 `name=Goldankauf Ziehl` mit `shop=gold_buyer` (Position 7.5923054/50.3570760, ebenfalls **ohne** `addr:housenumber`), node/4982836415 `name=Schmuckkontor Krick GmbH` (Löhrstraße 2, `shop=jewelry`), node/3346137446 mit `operator=DRK-Kreisverband Koblenz-Stadt e.V.` und `amenity=recycling`, node/13954035216 mit `operator=Deutsches Rotes Kreuz` und `amenity=recycling`, node/12534758498 mit `operator=Wittmann` und `amenity=recycling` sowie way/34792238 `name=expert klein` (Am Saarplatz 1, `shop=electronics`). **Kein** Objekt im Raster trägt `name=KMS`, `name=Münzen Studio` oder einen `shop=gold_buyer`-Eintrag an der Rathauspassage. Das KMS ist damit in OSM **nicht** kartiert; die Anschrift Rathauspassage 20 existiert dort, ein Münzankaufsbetrieb ist an dieser Nummer nicht erfasst. Quelle(n): https://api.openstreetmap.org/api/0.6/node/13258111292.json ; https://api.openstreetmap.org/api/0.6/node/10743121894.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=7.5868271,50.3496509,7.6070772,50.3701771 (Abruf 31.10.2026)]
- [Recherche 31.10.2026: Preise getrennt nach Ankauf, Verkauf und Gebühren. **Ankauf:** Die Leistungsseite „Ankauf" (https://www.koblenzer-muenzen-studio.de/leistungen/ankauf/) verspricht den Ankauf von Münzen (Byzantiner Gold, Byzanz Bronze, Mittelalter, Altdeutsch, Kaiserreich Gold/Silber, Weimarer Republik, Drittes Reich, BRD Gold/Silber, Euro, DDR in ST und PP, Habsburg/Österreich, Schweiz ab 17. Jh., Weltsilber, Anlagemünzen China/Zarenreich), Orden aus dem 1. und 2. Weltkrieg sowie Briefmarken („1 Buch oder größte Sammlungen sofort gegen Bar") und Fachliteratur — **ohne jede Preisangabe**; Bezahlung „bar oder per Überweisung Ihren Wünschen entsprechend". **Verkauf:** Die Leistungsseite „Verkauf" listet Münzen, Zubehör der Firmen Lindner und Safe (Münzboxen, Münzalben, Reinigungsmittel, Vitrinen für Orden), Briefmarken sowie Numismatik-/Phaleristik-Fachliteratur — ebenfalls **ohne** Preisliste. **Gebühren:** Hier liegt die erste belegte Zahl des Dossiers: die Seite „Angebotserstellung" nennt „Meine Angebotserstellung mündlich ist für Sie kostenfrei" und „Für eine Angebotserstellung schriftlich berechne ich min. 25,-Euro inkl. gesetzl. MwSt." — eine **Gebühr** für das schriftliche Angebot, die mündliche Bewertung ist ausdrücklich kostenfrei. Die AGB (https://www.koblenzer-muenzen-studio.de/agb-s/) ergänzen: „Für Verpackungsmaterial berechne ich 2 Euro pauschal", Portokosten „zu hundert Prozent nach Deutsche Post AG", Zahlung sofort ohne Abzug, Widerrufsrecht 14 Tage, Gutscheine verfallen nach 3 Jahren, Anzahlungen nach 1 Jahr. Ferner gilt: „Der Ankauf von Ware und der Ankauf und Verkauf von Artikeln des 3 Reiches sind bei mir erst ab dem 21. Lebensjahr möglich." Die im Dossier als E-Mail geführte Adresse frank-maurer1@web.de ist auf der Kontaktseite **nur** durch Cloudflare-Maskierung sichtbar und bleibt deshalb nicht verifizierbar — die E-Mail bleibt begründet leer. Offen: ob die 25-Euro-Gebühr für schriftliche Angebote je Angebot oder je Vorgang anfällt und ob für Gutachten/Echtheitsprüfung (https://www.koblenzer-muenzen-studio.de/leistungen/gutachten-schätzungen/) eigene Gebühren gelten — dort ist **keine** Preisangabe veröffentlicht. Quelle(n): https://www.koblenzer-muenzen-studio.de/leistungen/ankauf/ ; https://www.koblenzer-muenzen-studio.de/leistungen/verkauf/ ; https://www.koblenzer-muenzen-studio.de/leistungen/angebotserstellung/ ; https://www.koblenzer-muenzen-studio.de/agb-s/ (Abruf 31.10.2026)]

### Geokodierung 31.10.2026

- [31.10.2026 (Geokodierung): **Koordinate korrigiert**: `lat`/`lon` = **50.3601177/7.5971797** (Mittel des Knotenpunkts von node/13258111292 (1 eindeutiger Knoten); Punkt liegt innerhalb der Objekt-BBox). Alter Punkt 50.3540143/7.5774000 lag **1559 m** entfernt. Quelle 1 (OSM): node/13258111292 trägt `addr:housenumber`=20, `addr:street`=Rathauspassage, `addr:city`=Koblenz, `addr:postcode`=56068. Das Adressobjekt trägt keinen `name`/`operator`-Tag; in OSM existiert an der Rathauspassage kein Objekt mit `name`=KMS oder `shop`=gold_buyer - die Adresse ist nur über die Betreiberquelle belegt. Quelle 2 (Betreiber): die Betreiber-Kontaktseite nennt „KMS Koblenzer Münzen Studio, Frank Maurer, Rathauspassage 20, D-56068 Koblenz“. Quelle(n): https://api.openstreetmap.org/api/0.6/node/13258111292.json ; https://www.koblenzer-muenzen-studio.de/ (Abruf 31.10.2026)]
