---
slug: be-neukolln-alba-metall-nord-neukolln-sud
name: ALBA Metall Nord (Neukölln-Süd)
trader_type: sonstige
state: BE
city: Neukölln
street: Ziegrastr. 2-46
postcode: '12057'
phone: ''
email: ''
opening_hours: ''
website: https://metall.alba.info
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: A. Kept (Tabelle)
provenance_ankauf_raw: ja (Gewerbe)
provenance_origin: table
---

# ALBA Metall Nord (Neukölln-Süd)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Importiert (Seed-Stand 2026-09-30)

- Fe/NE, Kette/Filiale (B2B ok)
- Adresse: Neukölln, Ziegrastr. 2-46, 12057 Berlin (Ex-Interseroh = ALBA, kein Doppel)
- Bezirk: Neukölln Adressbeleg: seed/be.json (https://metall.alba.info)
- Adressbeleg: https://metall.alba.info

### Korrektur 01.10.2026 (ALBA-Fix)

- Delisted-Protokoll: Neukölln (Ziegrastr.) steht auf KEINER aktuellen Betreiber-Liste (Vollcrawl 01.10.2026: metall.alba.info = 13 Detailseiten; alba.info/standorte: weder Metall Nord noch ALBA Berlin GmbH mit Neukölln-Adresse).
- Betreiber-Suche/Register/News ohne Schließungs-/Verlagerungsbeleg → kein status geschlossen, Dossier bleibt erhalten; status aktiv → pruefung + Klärfall: Filial-Existenz und Seed-These „Ex-Interseroh = ALBA, kein Doppel“ (aus street-Feld hierher archiviert) weiter ohne Betreiber-Beleg.
- street normalisiert (Ziegrastr. 2-46, 12057 Berlin); website = Domain-Root (aktiv) ohne Detailseite.

### Korrektur 01.10.2026 (Owner-Gate)

- [Owner-Gate 01.10.2026: Gedankenstrich-Datenmüll → leer (Shard hat Street bereinigt, Phone übersehen).]

### Recherche 01.10.2026 (Feedback-Triage 2858)

- [Recherche 01.10.2026: Re-Verifikation Betreiber-Standortliste (13 Treffer — kein Neukölln/Ziegrastraße); Befund aus Korrektur 01.10.2026 bestätigt, kein Schließungs-/Verlagerungsbeleg; Quellen: https://metall.alba.info/unternehmen/standorte/]
- Feedback 2858 (Standort nicht gelistet) berechtigt; Klärfall Filial-Existenz bleibt.

### Recherche 31.10.2026

- [Recherche 31.10.2026: Die Standortliste des Betreibers wurde am 31.10.2026 erneut vollstaendig gelesen; der Befund der Vorwellen ist unveraendert, der Klaerfall bleibt offen, und die Adresslage der Ziegrastrasse wurde jetzt im OSM geprueft. Die Seite https://metall.alba.info/unternehmen/standorte/ nennt weiterhin "13 Treffer zu Ihrer Auswahl" und listet: "Standort Rostock - Zentrale Nord, ALBA Metall Nord GmbH, Werkstrasse 1, 18069 Rostock, +49 381 809060", "Standort Berlin-Spandau, ALBA Metall Nord GmbH, Nonnendammallee 28, 13599 Berlin, +49 30 351 82 843", "Standort Doeberitz, ALBA Metall Nord GmbH, Am Hafen 22, 14727 Premnitz/ OT Doeberitz, +49 3386 200110", "Standort Friedland, ALBA Metall Nord GmbH, An der Kleinbahn 13a, 17098 Friedland, +49 39601 2760", "Standort Hoppegarten, ALBA Metall Nord GmbH, Industriestr. 16, 15366 Hoppegarten, +49 30 35182 422", "Standort Lichtenberg, ALBA Metall Nord GmbH, Herzbergstr. 46-48, 10365 Berlin, +49 30 35182 9150", "Standort Luebbenau, ALBA Metall Nord GmbH, Nordstrasse 4, 03222 Luebbenau, +49 3542 2567", "Standort Luckenwalde, ALBA Metall Nord GmbH, Daemmchenweg 16, 14943 Luckenwalde, +49 3371 689 84 50", "Standort Pretzier, ALBA Metall Nord GmbH, Ritzlebener Str. 29, 29410 Salzwedel/ OT Pretzier, +49 39037 222", "Standort Quitzow, ALBA Metall Nord GmbH, Buchholzer Chaussee 5, 19348 Quitzow, +49 3876 786602", "Standort Schwerin, ALBA Metall Nord GmbH, Rogahner Strasse 72, 19061 Schwerin, +49 385 6131-81", "Standort Wilhelmshaven, ALBA Metall Nord GmbH, Emsstrasse 29, 26382 Wilhelmshaven, +49 4421 4020" und "Standort Stuttgart - Zentrale Sued, ALBA Metall Sued GmbH, Am Mittelkai 21, 70327 Stuttgart, +49 711 918920-0" - eine Neukoellner Ziegrastrasse steht weiterhin nicht darunter, ebenso wenig ein Berliner Standort in Reinickendorf. OSM-Abgleich (Rasterfenster 13.4578128/52.4719255 bis 13.4597207/52.4782104, Ziegrastrasse, 11 Adressobjekte mit addr:housenumber und addr:street=Ziegrastrasse, daraus 11 verschiedene Hausnummern; die Bereichsnotation "15-19" ist einzeln aufgeloest): Die Dossier-Hausnummer 2-46 existiert im OSM nicht als Adressangabe - die Nummernliste des Rasters lautet 1, 11, 13, 15, 16, 17, 18, 19, 21, 48, 225, wobei die Nummern 15, 16, 17, 18 und 19 aus der Bereichsangabe "15-19" stammen (node/2820273415 mit `addr:housenumber=15-19`, `addr:street="Ziegrastraße"`, `addr:postcode=12057`, `source=Geoportal Berlin / Hauskoordinaten`); die Nummern **15, 21 und 48** stehen zusaetzlich als eigene Objekte mit `addr:suburb="Neukölln"` (node/2820272799, node/2820273425, node/2820273544); auf der Sonnenallee tragen die Objekte zu 225, 226 und 227 denselben Stadtteil (node/255336108, node/2820273430, node/3774793898, node/2820273431, node/1923316601, node/2820273432); die Objekte zu Hausnummer 1 (node/2820272741, node/10836623615, node/11066768896, way/167793134) und 13 (node/6512193993) tragen **keinen** `addr:suburb`-Tag. Die Nummern 2 bis 46 sind also nicht durchgaengig belegt, weder die Nummern 2, 3, 4 usw. noch eine Angabe "2-46"; eine Zusammenfassung wie im Dossier-Seed existiert im OSM nicht. Im Raster tragen 85 Objekte `name=` und/oder `operator=` (78 mit `name=`, 23 mit `operator=`); 0 Objekte nennen "ALBA", "Metall Nord", "Interseroh" oder einen Schrottbetrieb. Das Raster gehoert heute dem Neukoellner Speicher und der Eventflaeche Estrel: way/365569349 mit `addr:housenumber=225`, `addr:street="Ziegrastraße"`, `building=hotel`, `name="Flügel 3"`, `alt_name=Estrel`, node/10836623615 mit `addr:housenumber=1`, `name="Neukölln Speicher"`, `operator=Estrel`, `amenity=events_venue`, `website=https://www.estrel.com/`, node/11907609694 mit `operator=Canoa Berlin GmbH`, `name=Canoa Berlin` sowie **vier** Objekte mit `operator=Berliner Stadtreinigung` (node/7494347939, node/7566282241, node/10312777343, node/10312777385). Das ist ein starker OSM-Hinweis darauf, dass die Strasse heute nicht als Schrott-Standort gefuehrt wird, aber kein Negativbeleg ueber den Betrieb (der Betrieb koennte unter einer anderen Nummer oder ausserhalb des Rasters liegen). Zu Ankauf, Verkauf und Gebuehren getrennt: ANKAUF - die Betreiber-Seite https://metall.alba.info/ankauf/sorten/ belegt das Sortenprogramm des Konzerns, nicht dieses Dossier: "Produktionsschrotte", "Haushaltsschrott", "Industrie- und Abbruchschrotte", "Kabelschrotte", "Mischschrotte", "Vormaterial", "Schwere Schere", "Legierte Schrotte", "Karossen", "Aluminium", "Kupfer", "Messing", "Brennschrott", "Gusseisen", "Moniereisen", "Scherenschrott", "Stahl- und Metallspaene", mit dem Angebot "Die Spezialist*innen von ALBA nehmen auf all unseren Plaetzen saemtliche Sorten von Stahl- und Metallschrotten an" und "zu Bestpreisen"; eine Preisliste oder ein Ankaufskurs ist nicht veroeffentlicht, die Seite verweist auf "Jetzt kontaktieren". VERKAUF - die Navigaton der Konzernseite fuehrt einen Bereich "Verkauf" (https://metall.alba.info/verkauf/), auf dieser Welle nicht inhaltlich ausgewertet; Verkaufskurse fuer Sekundaerrohstoffe sind nicht belegt. GEBUEHREN - keine Gebuehrentafel, keine Container-Tarife und keine Anlieferungsgebuehr veroeffentlicht; die Seite wirbt "Schnell, rechtssicher und zu Bestpreisen", nennt aber keine Kosten. Offene Fragen: Die Seed-These "Ex-Interseroh = ALBA, kein Doppel" bleibt unbelegt, da die Ziegrastrasse in OSM heute durch die Estrel-Eventflaeche gepraeegt ist und die Hausnummer 2-46 dort nicht existiert; der Klaerfall Filial-Existenz ist damit eher zu einer Frage der ueberholten Seed-Hausnummer als der ueberholten Strasse geworden, bleibt aber offen. Ein Beleg fuer eine Aufnahme in die aktuelle ALBA-Standortliste fehlt weiterhin. Quelle(n): https://metall.alba.info/unternehmen/standorte/ ; https://metall.alba.info/ankauf/sorten/ ; https://metall.alba.info/ ; OSM-API-Abfrage api.openstreetmap.org/api/0.6/map.json?bbox=13.4578128,52.4719255,13.4597207,52.4782104 (Abruf 31.10.2026) ; https://www.openstreetmap.org/way/365569349 ; https://www.openstreetmap.org/node/2820273415]

- [31.10.2026 (Korrekturvermerk): **OSM-Tag-Zitat berichtigt**: der zitierte Wert `addr:street="Ziegrastrasse"` existiert in OSM nicht — live trägt das Objekt `addr:street="Ziegrastraße"`. Die **Zählungen** im Dossier bleiben unverändert (die Zählung lief auf der richtigen, umgelauteten Straße). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (Raster laut Dossier, Abruf 31.10.2026)]

- [31.10.2026 (Korrekturvermerk): **OSM-Tag-Zitat berichtigt**: der zitierte Wert `addr:suburb="Neukoelln"` existiert in OSM nicht — live trägt das Objekt `addr:suburb="Neukölln"`. Dieser Vermerk betrifft den **ALBA-Metall-Nord-Standort Neukölln-Süd**. Die **Zählungen** im Dossier bleiben unverändert (die Zählung lief auf der richtigen, umgelauteten Straße). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (Raster laut Dossier, Abruf 31.10.2026)]

- [31.10.2026 (Korrekturvermerk): **OSM-Tag-Zitat berichtigt**: der zitierte Wert `name="Fluegel 3"` existiert in OSM nicht — live trägt das Objekt `name="Flügel 3"`. Die **Zählungen** im Dossier bleiben unverändert (die Zählung lief auf der richtigen, umgelauteten Straße). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (Raster laut Dossier, Abruf 31.10.2026)]

- [31.10.2026 (Korrekturvermerk): **OSM-Tag-Zitat berichtigt**: der zitierte Wert `name="Neukoelln Speicher"` existiert in OSM nicht — live trägt das Objekt `name="Neukölln Speicher"`. Die **Zählungen** im Dossier bleiben unverändert (die Zählung lief auf der richtigen, umgelauteten Straße). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (Raster laut Dossier, Abruf 31.10.2026)]

- [31.10.2026 (Korrekturvermerk): **Objektanker ergänzt**: der Vermerk nannte den korrigierten Wert ohne Objekt-ID und war damit objektgenau nicht nachprüfbar; die Notiz nennt die betroffenen Objekte nun ausdrücklich. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (Raster laut Dossier, Abruf 31.10.2026)]

- [31.10.2026 (Korrekturvermerk): **Aufzählung präzisiert**: nur die Hausnummern **15** und **21** tragen live `addr:suburb=Neukölln` (node/2820272799, node/2820273425); die Objekte zu Hausnummer 1 (node/2820272741, node/10836623615, node/11066768896, way/167793134) und 13 (node/6512193993) tragen **keinen** `addr:suburb`-Tag. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=13.4578128,52.4719255,13.4597207,52.4782104 (Abruf 31.10.2026)]

- [31.10.2026 (Korrekturvermerk): **Liste vollständig**: live tragen **neun** Objekte `addr:suburb=Neukölln` — auf der Ziegrastraße die Hausnummern **15, 21 und 48** (node/2820272799, node/2820273425, node/2820273544), auf der Sonnenallee die Nummern 225, 226 und 227 (node/255336108, node/2820273430, node/3774793898, node/2820273431, node/1923316601, node/2820273432). Die Objekte zu Hausnummer 1 und 13 tragen **keinen** `addr:suburb`-Tag. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=13.4578128,52.4719255,13.4597207,52.4782104 (Abruf 31.10.2026)]

- [31.10.2026 (Korrekturvermerk): **Zahl berichtigt**: im genannten Raster tragen live **vier** Objekte `operator=Berliner Stadtreinigung` (node/7494347939, node/7566282241, node/10312777343, node/10312777385) — nicht drei. Vier unabhängige Messungen (OSM-API und Overpass, 31.10.2026) ergeben jeweils vier; die letzte Objektänderung erfolgte am 07.02.2024, es ist also keine Datenänderung. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=13.4578128,52.4719255,13.4597207,52.4782104 (Abruf 31.10.2026)]
