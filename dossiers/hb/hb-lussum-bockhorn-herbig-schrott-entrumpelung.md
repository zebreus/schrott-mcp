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
website_status: ''
status: pruefung
description: 'Schrott und Entrümpelung in Bremen-Lüssum-Bockhorn, ausschließlich über Branchenverzeichnisse geführt (Schwaneweder Str. 44 a, 28779 Bremen). ANKAUF: keine Preise und keine Sortenliste veröffentlicht. VERKAUF: keine Preisliste. GEBÜHREN: keine Tarife. Eine eigene Betreiberseite, ein Registerbeleg und ein Betreiber-Social wurden nicht gefunden; die im Telefonfeld geführte Nummer bleibt bewusst leer, die Nummer 0179 6999948 ist nur ein Verzeichnislead.'
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: Bremen (Stadt)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Herbig Schrott & Entrümpelung

## Überblick

Für den Betrieb gibt es bis heute nur Verzeichnisbelege. 11880 führt "Herbig Schrott & Entrümpelung", Entsorgungsdienst, Entrümpelung und Entrümpelungsdienst, Schwaneweder Str. 44 a, 28779 Bremen (Lüssum-Bockhorn), (0179) 6999948, ohne Öffnungszeiten und mit Eintragsdaten vom 30.05.2026. Eine Betreiberseite, ein Registerbeleg und ein Betreiber-Social wurden nicht gefunden, und die Gelbe Seiten führt in der Bremer Schrotthändlerliste keinen Herbig-Eintrag (Negativbefund ohne Beweiswert).

**ANKAUF:** keine Preise und keine Sortenliste veröffentlicht. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Tarife.

Das Telefonfeld bleibt auf Anweisung des Owners leer; die 11880-Nummer gilt nur als Lead. Die Hausnummer 44 a ist in OSM nicht erfasst (nur 44 b), für die Straße Schwaneweder Straße existiert kein Adresspunkt zur Hauskennummer 44 a, deshalb bleibt die Geokodierung offen. Der gespeicherte Punkt liegt auf der Schwaneweder Straße selbst und bleibt unangetastet.

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
- [Recherche 28.10.2026: GEO bleibt offen: OSM kennt in der Schwaneweder Straße nur die Hausnummer 44 b (Way 305866258, addr:housenumber 44 b, addr:street Schwaneweder Straße, addr:postcode 28779, 53.2002032/8.5744241), nicht die Seed-Adresse 44 a; die beiden Hausnummern liegen rund 1,23 km auseinander (Haversine, R = 6371 km), es wurde deshalb keine Koordinate geraten und Geocoding für Schwaneweder Str. 44 a steht aus. Der gespeicherte Punkt 53.2108611/8.5795163 löst auf die Schwaneweder Straße in Bremen-Lüssum-Bockhorn auf und ist damit straßengleitend, wird deshalb nicht angetastet. Quelle(n): https://api.openstreetmap.org/api/0.6/way/305866258/full.json]
- [Recherche 28.10.2026: ANKAUF, VERKAUF und GEBÜHREN: Für den Betrieb sind keine Preise, keine Sortenliste, kein Verkaufsangebot und keine Gebühren veröffentlicht; das Verzeichnis vermerkt lediglich Entrümpelung, Schrotthandel und Haushaltsauflösung als Leistungsumfang, ohne Konditionen. Damit bleibt auch die Abhol- oder Anlieferfrage offen, weshalb dropoff_json und pickup_json leer bleiben und keine Servicezusage aus dem Verzeichnis abgeleitet wird. Quelle(n): https://www.11880.com/suche/herbig/bremen]
