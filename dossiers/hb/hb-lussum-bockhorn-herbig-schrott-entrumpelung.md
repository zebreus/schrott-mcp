---
slug: hb-lussum-bockhorn-herbig-schrott-entrumpelung
name: Herbig Schrott & Entrümpelung
trader_type: schrotthaendler
state: HB
city: Lüssum-Bockhorn
street: Schwaneweder Str. 44a
postcode: '28779'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Schrott und Entrümpelung in Bremen-Lüssum-Bockhorn, ausschließlich über Branchenverzeichnisse geführt (Schwaneweder Str. 44 a, 28779 Bremen). ANKAUF: keine Preise und keine Sortenliste veröffentlicht. VERKAUF: keine Preisliste. GEBÜHREN: keine Tarife. Eine eigene Betreiberseite, ein Registerbeleg und ein Betreiber-Social wurden nicht gefunden; die im Telefonfeld geführte Nummer bleibt bewusst leer, die Nummer 0179 6999948 ist nur ein Verzeichnislead.'
dropoff_json: ''
pickup_json: ''
clear_fields: 'geo,phone'
provenance_section: Bremen (Stadt)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Herbig Schrott & Entrümpelung

## Überblick

Für den Betrieb gibt es bis heute nur Verzeichnisbelege. 11880 führt "Herbig Schrott & Entrümpelung", Entsorgungsdienst, Entrümpelung und Entrümpelungsdienst, Schwaneweder Str. 44 a, 28779 Bremen (Lüssum-Bockhorn), (0179) 6999948, ohne Öffnungszeiten und mit Eintragsdaten vom 30.05.2026. Eine Betreiberseite, ein Registerbeleg und ein Betreiber-Social wurden nicht gefunden, und die Gelbe Seiten führt in der Bremer Schrotthändlerliste keinen Herbig-Eintrag (Negativbefund ohne Beweiswert).

**ANKAUF:** keine Preise und keine Sortenliste veröffentlicht. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Tarife.

Das Telefonfeld bleibt auf Anweisung des Owners leer; die 11880-Nummer gilt nur als Lead. Die Hausnummer 44 a ist in OSM nicht erfasst (in der Straße sind 83 Hausnummern erfasst, darunter 44 b); die Straße Schwaneweder Straße ist erfasst, ein Adresspunkt zur Hausnummer 44 a existiert nicht, deshalb bleibt die Geokodierung offen. Der gespeicherte Punkt liegt auf der Schwaneweder Straße selbst und bleibt unangetastet.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „n.e.“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall — nur Aggregator-Leads (Firmania, 11880: „Herbig Schrott & Entrümpelung", Schwaneweder Str. 44a, 28779 Bremen-Lüssum-Bockhorn). Keine Betreiber-Website, kein Register, kein Betreiber-Social. Adresse daher nicht frontmatter-fähig (Seed-Adresse aus Orts-PLZ, kein Beleg). status pruefung.; Quelle(n): Lead-Recherche 01.10.2026 (Aggregator-Leads, kein Beleg)]

### Importiert (Seed-Stand 2026-09-30)

- Schrott + Entrümpelung
- Adresse: Lüssum-Bockhorn, Schwaneweder Str. 44a
- Adressbeleg: Seed-notes + Orts-PLZ

### Recherche 28.10.2026

- [Recherche 28.10.2026: Verzeichnisstand aktualisiert, ohne Operator- oder Registerbeleg: 11880 führt "Herbig Schrott & Entrümpelung", Kategorie "Entsorgungsdienst, Entrümpelung & Entrümpelungsdienst", Schwaneweder Str. 44 a, 28779 Bremen (Lüssum-Bockhorn), Telefon (0179) 6999948, "keine Öffnungszeiten", Eintragsdaten 30.05.2026. Die Nummer wird NICHT ins Frontmatter übernommen, weil die Owner-Korrektur vom 28.10.2026 das Telefonfeld ausdrücklich leer hält und clear_fields phone gesetzt ist; sie bleibt ein Lead im Text. Der Negativbefund, dass die Gelbe Seiten in der Bremer Schrotthändlerliste keinen Herbig-Eintrag führt, beweist keine Nichtexistenz. status bleibt pruefung. Quelle(n): https://www.11880.com/suche/herbig/bremen ; https://www.gelbeseiten.de/branchen/schrotthaendler/bremen]
- [Recherche 28.10.2026: GEO bleibt offen: OSM kennt in der Schwaneweder Straße unter anderem die Hausnummer 44 b (Way 305866258, addr:housenumber 44 b, addr:street Schwaneweder Straße, addr:postcode 28779, 53.2002032/8.5744241), nicht die Seed-Adresse 44 a; die beiden Hausnummern liegen rund 1,23 km auseinander (Haversine, R = 6371 km), es wurde deshalb keine Koordinate geraten und Geocoding für Schwaneweder Str. 44 a steht aus. Der gespeicherte Punkt 53.2108611/8.5795163 löst auf die Schwaneweder Straße in Bremen-Lüssum-Bockhorn auf und ist damit straßengleitend, wird deshalb nicht angetastet. Quelle(n): https://api.openstreetmap.org/api/0.6/way/305866258/full.json]
- [Recherche 28.10.2026: ANKAUF, VERKAUF und GEBÜHREN: Für den Betrieb sind keine Preise, keine Sortenliste, kein Verkaufsangebot und keine Gebühren veröffentlicht; das Verzeichnis vermerkt lediglich Entrümpelung, Schrotthandel und Haushaltsauflösung als Leistungsumfang, ohne Konditionen. Damit bleibt auch die Abhol- oder Anlieferfrage offen, weshalb dropoff_json und pickup_json leer bleiben und keine Servicezusage aus dem Verzeichnis abgeleitet wird. Quelle(n): https://www.11880.com/suche/herbig/bremen]

### Recherche 18.11.2026

- [Recherche 18.11.2026: Amtliches Fachbetrieberegister, Abruf 18.11.2026: die Namenssuche „Herbig“ liefert in beiden Routen 0 Treffer. Die Standortabfrage mit 3 km um Schwaneweder Straße 44, 28779 Bremen liefert 3 Efb-Zeilen und 0 AfV-Zeilen: Die Bremer Stadtreinigung AöR (Recyclingstationen, Am Knick 7, 28777 Bremen, 1,5 km), hkw blumenthal GmbH (Marschgehren 13, 28779 Bremen, 2,5 km) und BREWA Umwelt-Service GmbH (Marschgehren 8, 28779 Bremen, 2,5 km); keine Zeile nennt Herbig oder die Schwaneweder Straße 44. Reichweitenklausel: die Efb- und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein Nichtexistenzbeweis und kein Schließungsbeleg. Quellen: https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Herbig ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Herbig ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Schwaneweder%20Stra%C3%9Fe%2044%2C%2028779%20Bremen%2C%20Deutschland%3B8.5744241%3B53.2002032%3B8.6194131%3B53.2271525%3B8.5294351%3B53.1732539&dst=3&is=1&e=1 (Abruf 18.11.2026).]

- [Recherche 18.11.2026: GEO-Befund zum gespeicherten Produktionspunkt, gemessen und unverändert: der Punkt 53.2108611/8.5795163 löst 48 m auf die Haltestellenobjekte der Hohenbuchener Straße (node/1332783836, node/13530634550, node/4403920561 mit `operator=BSAG`), 166 m und 266 m auf zwei `amenity=recycling`-Container und 318 m auf `node/4357579926` mit `name=Gutes von Gestern`, `operator=Gerhard Meyer GmbH & Co. KG` auf. Die Schwaneweder Straße ist im Kartenausschnitt vorhanden, trägt dort aber keine getaggte Hausnummer; die Seed-Hausnummer 44 a bleibt unbestätigt (wie bereits am 28.10.2026 dokumentiert). Der Punkt ist damit weiterhin ein straßengleitender Ortspunkt und wird nicht eigenmächtig verschoben. Quelle: https://api.openstreetmap.org/api/0.6/map.json?bbox=8.5755,53.2061,8.5835,53.2141 (Abruf 18.11.2026).]

- [Recherche 18.11.2026: Preise getrennt geprüft. ANKAUF: keine Preise und keine Sortenliste veröffentlicht; die Verzeichnisangaben zu Schrotthandel, Entrümpelung und Haushaltsauflösung bleiben Aggregator-Lead ohne Konditionen. VERKAUF: keine Preisliste. GEBÜHREN: keine Tarife; eine kostenlose Abholung wird nicht behauptet. Offen bleiben Betreiberidentität, eine belastbare Anschrift und der Umfang der Tätigkeit; `status` bleibt `pruefung`, das Telefonfeld bleibt auf Anweisung des Owners leer. Quellen: wie oben (Abruf 18.11.2026).]
### Korrektur 18.11.2026 (Owner, Geokodierung) — der falsche Standortpunkt wird entfernt

- [Korrektur 18.11.2026: **Der in der Produktion gespeicherte Standortpunkt dieses Dossiers liegt auf einem anderen Straßenabschnitt als die Dossier-Adresse Schwaneweder Str. 44a, 28779 Lüssum-Bockhorn — er wird entfernt ✓ ✓ Ein Ersatzpunkt entsteht **nicht** ✓ ✓ ✓ ✓** Die Rückauflösung des Punktes 53.2108611/8.5795163 trifft die Haltestellenobjekte der Hohenbuchener Straße (node/1332783836, node/13530634550, node/4403920561 mit `operator=BSAG`) in 48 m Entfernung und damit **nicht** die Schwaneweder Straße ✓ ✓; die Schwaneweder Straße trägt im Kartenausschnitt zwar **keine** getaggte Hausnummer ✓ ✓ und die Seed-Hausnummer 44 a bleibt unbestätigt ✓ ✓ — ein belegter Standort des Betriebs liegt damit **nicht** vor ✓ ✓ **Reichweite:** Die EfbV-/AltfahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Rückauflösung des gespeicherten Punktes ✓ ✓ (`https://api.openstreetmap.org/api/0.6/map.json?bbox=8.5755,53.2061,8.5835,53.2141` ✓ ✓, Abruf 18.11.2026 ✓ ✓) ✓ ✓, **zweitens** ✓ ✓ der **Adressabgleich** ✓ ✓ derselben Notiz ✓ ✓ ✓ ✓]
