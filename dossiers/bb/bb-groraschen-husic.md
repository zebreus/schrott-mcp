---
slug: bb-groraschen-husic
name: Husic
trader_type: schrotthaendler
state: BB
city: Großräschen
street: Karl-Marx-Str. 18
postcode: 01983
lat: 51.5957230
lon: 14.0001210
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Husic

## Überblick

„Schrotthandel Husic“ an Karl-Marx-Str. 18 erscheint in 11880, bislang ohne unabhängige Betreiber- oder Registerquelle. Das städtische Verzeichnis nennt separat „Dienstleistung Transporte und Schrotthandel Hedt“ an Neu Bückgen 28; kein Beleg für eine Verbindung. Seed-Adresse bleibt ein Lead, nicht geokodierungs- oder kontaktreif. Keine veröffentlichten Ankaufspreise, Verkaufspreise oder Gebühren belegt.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „n.e.“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 14.10.2026 (18:00-Welle C)

- [Recherche 14.10.2026: 11880-Detail neu direkt gelesen: Schrotthandel Husic, Karl-Marx-Str. 18, 01983 Großräschen, 0155 61332975, Kategorien Entrümpelung/Schrotthandel, keine Zeiten, E-Mail/Website nur zum Hinzufügen; Datenstand 01.06.2026/Update 04.10.2026. FAQ-Verweis auf das allgemeine Label vom Inhaber bestätigt ist kein nachgewiesener Betreiber-Nachweis für diesen Eintrag. Kommunale Hedt-Detailkarte ebenfalls direkt gelesen: Monika Hedt, Neu Bückgen 28, eigene Festnetz-/Mobilnummern, Eintritt 01.06.2007. Sie belegt einen anderen Betrieb, nicht Husic. Keine Kontakte aus Leads gefüllt, pruefung unverändert. ANKAUF keine Zahlenkurse; VERKAUF/GEBÜHREN keine Listen. Offen: voller Name, Gewerbezuordnung und heutige Annahme/Abholung; Karl-Marx-Straße erst dann geokodieren; Quelle(n): https://www.11880.com/branchenbuch/grossraeschen/060441363B113521775/schrotthandel-husic.html (Lead); https://www.grossraeschen.de/verzeichnis/visitenkarte.php?mandat=21796 (kommunale Fremdbetriebsquelle).]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Erneute Namenssuche ergab keinen Betreiberauftritt oder unabhängigen Nachweis zusätzlich zum 11880-Treffer; das kommunale Verzeichnis führt weiterhin den anderen Betrieb „Hedt“ an Neu Bückgen 28. Keine Verbindung konstruiert und keine Frontmatter-Fills. In den geprüften Quellen keine konkrete Ankaufpreisliste, Verkaufspreise oder Gebühren; Status pruefung bleibt. Quelle(n): https://www.11880.com/branchenbuch/grossraeschen/060441363B113521775/schrotthandel-husic.html (Aggregator); https://www.grossraeschen.de/verzeichnis/visitenkarte.php?mandat=21796 (kommunaler Eintrag für Hedt, kein Husic-Beleg).]

### Importiert (Seed-Stand 2026-09-30)

- Schrott
- Adresse: Großräschen, Karl-Marx-Str. 18
- Adressbeleg: Seed-notes (Verzeichnis); PLZ Orts-PLZ

### Recherche 04.10.2026

- [Recherche 04.10.2026: Klärfall, keine verifizierten Kontakt-Fills. 11880 führt „Schrotthandel Husic“ mit Karl-Marx-Str. 18, 01983 Großräschen, Telefon 0155 61332975 und den Kategorien Entrümpelung/Schrotthandel (Datensatz 01.06.2026, aktualisiert 04.10.2026); das bleibt ein einzelner Aggregator-Beleg, kein unabhängiger Zweitbeleg und keine Betreiber-Primärquelle. Die Seed-Adresse stimmt damit überein, ist aber nicht unabhängig bestätigt und bleibt unverändert. Kein zurechenbarer Betreiberauftritt gefunden; website bleibt leer, website_status auf unbekannt gesetzt. Telefon, E-Mail, Zeiten, Beschreibung sowie Anlieferung/Abholung bleiben mangels belastbarer Belege leer. Das städtische Verzeichnis listet separat „Dienstleistung Transporte und Schrotthandel Hedt“ an Neu Bückgen 28; kein Beleg für Identität mit Husic, daher nicht vermischt.; Quelle(n): https://www.11880.com/branchenbuch/grossraeschen/060441363B113521775/schrotthandel-husic.html; https://www.grossraeschen.de/verzeichnis/visitenkarte.php?mandat=21796]

### Geokodierung 08.11.2026

- [Geokodierung 08.11.2026: **Koordinate der Dossier-Adresse belegt** (`lat`/`lon` = **51.5957230/14.0001210**). Punktgüte: **Hausnummer-Treffer**; der Dossierbetrieb „Husic“ ist über die Adresse **Karl-Marx-Str. 18, 01983 Großräschen** adressiert; das OSM-Adressobjekt **n/9483559671** trägt `addr:housenumber`=18, `addr:street`=Karl-Marx-Straße, `addr:postcode`=01983, `addr:city`=Großräschen; das Objekt trägt **weder `name=` noch `operator=`** - die Koordinate belegt die **Anschrift**, nicht die Betreiberzuordnung; der Punkt ist die Knotenkoordinate; die Nominatim-Vorwärtssuche nach „Karl-Marx-Str. 18, 01983 Großräschen“ liefert unter den 1 Treffern dasselbe Objekt **n/9483559671**.]
  Quelle(n): https://nominatim.openstreetmap.org/search?format=jsonv2&extratags=1&namedetails=1&addressdetails=1&countrycodes=de&limit=20&q=Karl-Marx-Str.+18%2C+01983+Gro%C3%9Fr%C3%A4schen ; https://api.openstreetmap.org/api/0.6/node/9483559671.json (Abruf 08.11.2026).]

### Recherche 19.11.2026 (Welle 1800 B)

- [Recherche 19.11.2026: **Der 11880-Lead besteht am 19.11.2026 unverändert fort, ist aber weiterhin der einzige Beleg — das amtliche Register kennt in Großräschen keinen zertifizierten Betrieb an der Seed-Adresse.** Die 11880-Detailseite wurde erneut direkt gelesen (HTTP 200) und führt weiterhin **`Schrotthandel Husic Großräschen | Oktober 2026 aktualisiert`**, **Karl-Marx-Str. 18**, **01983**, Telefon **(0155) 61332975**; die Seite weist **Eintragsdaten vom 01.06.2026** und **Aktualisierung am 04.10.2026** aus — die im Dossier dokumentierten Daten sind damit **unverändert**. Wichtig ist die Datierung: der Eintrag ist ein **Verzeichnisdatensatz von Juni 2026**, kein tagesaktueller Betreiberbeleg; ein Betreiberauftritt, ein Registerbeleg oder ein kommunaler Beleg existiert weiterhin nicht. **NEGATIVBEFUND mit Reichweite:** die Namenssuche in der Route „Entsorgungsfachbetriebe“ nach „Husic“ liefert am 19.11.2026 **0 Treffer**; die Standortabfrage mit 3 km um „Karl-Marx-Str. 18, 01983 Großräschen“ in derselben Route liefert **18 Treffer**, sämtlich **fremde** Rechtsträger (Kockro Logistik und Speditions GmbH, Woschkower Weg 14, 1,5 km, ZZHT00300707 / 1; EEW Energy from Waste Großräschen GmbH, Bergmannstraße 29, 2,1 km, ZZPT00400064 / 1 Verwerten; Abfallentsorgungsverband Schwarze Elster, Bergmannstraße 44, 01983 Großräschen OT Freienhufen, 2,2 km, ZZPT00300181 / 1 bis / 4; Becker Umweltdienste GmbH, Bergmannstraße 13, 2,4 km, ZZEE00200124 / 46 bis / 48; LCM Lausitzer Container & Metall GmbH, Gewerbestraße 6a, 2,9 km, ZZHT01000327 / 1 Befördern) — **keine** Zeile nennt Husic oder die Karl-Marx-Straße 18. Reichweite: die Efb-Anerkennung ist **freiwillig**, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis und **keine** Schließungsaussage — er belegt nur, dass **kein zertifizierter Betrieb** „Husic“ an der Seed-Adresse geführt wird. **Der entscheidende Abgleich bleibt die Abgrenzung zum Nachbarbetrieb:** das kommunale Verzeichnis Großräschen führt weiterhin separat **`Dienstleistung Transporte und Schrotthandel Hedt`, Neu Bückgen 28** (Monika Hedt, Eintritt 01.06.2007, eigene Festnetz-/Mobilnummern) — dieser andere Betrieb wird **nicht** mit dem Seed gleichgesetzt; eine Verbindung zwischen beiden ist weiterhin **nicht** belegt. **Quelle 2 (unabhängig, live):** das adressgenaueste OSM-Objekt der Seed-Adresse ist **node/9483559671** (`addr:housenumber`=**18**, `addr:street`=Karl-Marx-Straße, `addr:postcode`=**01983**, `addr:city`=Großräschen, `addr:country`=DE) bei 51.5957230/14.0001210 — es trägt **weder `name=` noch `operator=`** und belegt die **Anschrift**, nicht den Betrieb; ein Betriebs-POI für Husic ist in OSM **nicht** vorhanden. **Frontmatter unverändert:** `street`/`postcode` bleiben als **Verzeichnisangaben** stehen, `phone` bleibt **leer** (der `clear_fields`-Token `phone` bleibt bestehen — der 11880-Lead allein rechtfertigt keine Übernahme), `website` bleibt **leer**, `status` bleibt **pruefung**. **ANKAUF:** keine Ankaufspreise, keine Mengen- oder Annahmebedingung belegt. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Abhol- oder Entsorgungstarife. Quelle(n): https://www.11880.com/branchenbuch/grossraeschen/060441363B113521775/schrotthandel-husic.html ; https://api.openstreetmap.org/api/0.6/node/9483559671.json ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Husic ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Karl-Marx-Str.%2018%2C%2001983%20Grossr%C3%A4schen%2C%20Deutschland%3B14.000121%3B51.595723%3B14.0337498%3B51.6293518&dst=3&is=1&e=1 (Abruf 19.11.2026).]
- [Recherche 19.11.2026: **Der gespeicherte Standortpunkt liegt exakt auf der belegten Anschrift — bestätigt, aber weiterhin kein Betriebsnachweis.** Der read-only Produktionspunkt (51.595723/14.000121) ist **deckungsgleich** (Abweichung **0,0 m**) mit dem OSM-Adressobjekt **node/9483559671** und löst sich in der Nominatim-Rückauflösung auf **`18, Karl-Marx-Straße, Großräschen-Nord, Großräschen, 01983, Deutschland`** auf. Der Punkt belegt damit die **Anschrift** (Hausnummer-Treffer), **nicht** den Betrieb — das Objekt trägt weder `name=` noch `operator=`. `lat`/`lon` bleiben **unverändert**; `clear_fields` enthält **kein** `geo`-Token und bleibt unverändert. **Offen:** Inhaberschaft, Gewerbezuordnung, heutige Annahme- und Abholbedingungen; die Vorwellen-Notiz zu einem „FAQ-Verweis auf das allgemeine Label vom Inhaber“ bleibt ungeklärt. Quelle(n): https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.595723&lon=14.000121&zoom=18&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/node/9483559671.json ; read-only `public.db` (19.11.2026).]
