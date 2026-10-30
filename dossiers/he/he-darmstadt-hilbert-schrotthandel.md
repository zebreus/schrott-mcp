---
slug: he-darmstadt-hilbert-schrotthandel
name: Hilbert Schrotthandel
trader_type: metallhaendler
state: HE
city: Darmstadt
street: Kirschenallee 176
postcode: '64293'
phone: ''
email: ''
opening_hours: ''
website: https://schrotthandel-hilbert.de/
website_status: aktiv
status: pruefung
description: 'Schrotthandel und Metallservice aus Darmstadt mit Buntmetall-Ankauf zu Tageshöchstpreisen, Ladekran-LKW und kostenfreiem Container fuer die Abholung; ausserdem Demontage- und Abbrucharbeiten sowie Entruempelung und Haushaltsauflösung. Zwei Anschriften im Betreiberauftritt: Kirschenallee 176 (Fusszeile) und Akazienweg 7 (Anfahrt-Link), beide 64293 Darmstadt - die Strasse bleibt deshalb offen.'
dropoff_json: ''
pickup_json: '{"allowed": true, "conditions": "Abholung mit Ladekran-LKW; fuer die Abholung stellt der Betrieb einen kostenfreien Container bereit; Demontage- und Abbrucharbeiten sowie Entruempelung und Haushaltsauflösung werden angeboten (schrotthandel-hilbert.de, Abruf 30.10.2026)."}'
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Hilbert Schrotthandel

## Überblick

Schrotthandel und Metallservice aus Darmstadt, betrieben von **Damiano Hilbert** und **Horst Hilbert**
(Einzelunternehmen ohne HRB). Der Betreiberauftritt `schrotthandel-hilbert.de` ist am 30.10.2026 auf
**eine einzige Seite reduziert**: alle Unterpfade (`/schrott`, `/demontage`, `/entruempelung`,
`/uber-uns`, `/contact`, `/impressum`) liefern **HTTP 404**, es gibt also **kein Impressum** und keine
Rechtsform prüfbar.

**Adresslage weiter ungeklärt:** Die Fußzeile nennt **Kirschenallee 176, 64293 Darmstadt**, der
"Anfahrt"-Link im Menü und in der Fußzeile führt dagegen per Google-Maps-Kurzlink auf die Suche
**"Akazienweg 7, 64293 Darmstadt"** (302 auf `https://www.google.com/search?q=Akazienweg+7,+64293+Darmstadt`).
Beide Hausnummern existieren in OSM. Die Dossier-Straße bleibt deshalb unverändert, `postcode` 64293
ist dagegen belegt, weil beide Betreiberanschriften dieselbe PLZ tragen.

**Ankauf:** Ankauf von Buntmetallen im Raum Darmstadt "zu Tageshöchstpreisen", Abholung per
Ladekran-LKW, für die Abholung wird ein **kostenfreier Container** gestellt. **Keine Zahlenpreise.**
**Verkauf/Dienstleistung:** Demontage und Abbruch, Entrümpelung und Haushaltsauflösung "zu fairen
Preisen", ebenfalls ohne Zahlen. **Gebühren:** keine Gebührentafel; "kostenfreier Container" und die
Übernahme des Abtransports sind Leistungszusagen, kein Tarif.

**Offen:** Straße, Kontaktdaten und eine belastbare Rechtsform bleiben Klärfall; die Seite enthält
zusätzlich ungepflegte englische Demo-Blöcke einer Webflow-Vorlage ("Cagoworks", "Lorem ipsum",
`hello@cagoworx.com`, `+1 (555) 000-0000`), was die Pflegequalität der Auftritt betrifft.

## Timeline

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback-Triage 2261 KLAERFALL — Adress-Widerspruch Betreiber-Site (Footer Kirschenallee 176 vs. Kontaktseite Akazienweg 7, beide 64293 Darmstadt) + Impressum 404 (keine Rechtsform/HRB pruefbar, vermutl. Einzel Damiano/Horst Hilbert) → keine Owner-Ausnahme, nur Einzelbeleg → PLZ/Telefon/E-Mail nicht in Frontmatter (publiziert: 64293, Mobil 01590-1232867 / 0160-97744177 mit Link-Ziel-Widerspruch, info@schrotthandel-hilbert.de); Dossier-Strasse Kirschenallee 176 bleibt (ungeklaert); website aktiv; Quelle(n): https://schrotthandel-hilbert.de/ + https://schrotthandel-hilbert.de/contact]

### Importiert (Seed-Stand 2026-09-30)

- Buntmetall-Ankauf Tageshöchstpreise, Ladekran-Abholung, Container, Demontage

### Recherche 30.10.2026

- [Recherche 30.10.2026: **Der Adress-Widerspruch ist nicht aufgelöst, sondern hat eine neue Form bekommen.** Der Betreiberauftritt `https://schrotthandel-hilbert.de/` (HTTP 200) ist auf **eine Seite reduziert**: die Unterpfade `/schrott`, `/demontage`, `/entruempelung`, `/uber-uns`, `/contact` und `/impressum` liefern am 30.10.2026 alle **HTTP 404** (je 1.154 Bytes Fehlerseite). Damit gibt es **kein Impressum** und keine prüfbare Rechtsform/Registerangabe; es bleibt ein Einzelunternehmen (Damiano Hilbert und Horst Hilbert), die Owner-Ausnahme greift nicht. Die früher als Akazienweg-7-Quelle zitierte Kontaktseite ist nicht mehr erreichbar, **aber der Adresswiderspruch besteht fort**: Die Fußzeile schreibt "Adresse: Kirschenallee 176, 64293 Darmstadt", während der Menü- und Fußzeilen-Link **"Anfahrt"** über den Google-Maps-Kurzlink `https://g.co/kgs/m3HUvva` per HTTP 302 auf die Suche **"Akazienweg 7, 64293 Darmstadt"** führt. **Beide Hausnummern existieren in OSM**, live gemessen: Kirschenallee, Raster 8.6298,49.8737,8.6434,49.8882 (4 Kacheln, 0 Fehler) - **46 Adressobjekte / 44 verschiedene Hausnummern**, darunter **node/2911775037** mit `addr:housenumber=176`, `addr:street=Kirschenallee`, `addr:city=Darmstadt`, `addr:postcode=64293`, `addr:country=DE` (ohne `name`- und ohne `operator`-Tag); Akazienweg, Raster 8.6167,49.8777,8.6302,49.8917 (4 Kacheln, 0 Fehler) - **30 Adressobjekte / 30 verschiedene Hausnummern**, darunter **node/2940158790** mit `addr:housenumber=7`, `addr:street=Akazienweg`, `addr:city=Darmstadt`, `addr:postcode=64293`, `addr:suburb=Darmstadt-Nord` und `fixme=place node at building entry`. Kein der beiden Objekte trägt einen Betreiber- oder Namens-Tag; eine Zuordnung ist aus OSM nicht möglich. Frontmatter: `postcode` 64293 gefüllt (beide Betreiberanschriften tragen dieselbe PLZ), `description` und `pickup_json` neu gesetzt; **Telefon und E-Mail bleiben leer** (Einzelbeleg ohne Impressum/HRB nach der Entscheidung vom 01.10.2026), publiziert sind weiterhin die beiden Mobilnummern 01590 1232 867 und 0160 977 44 177 sowie info@schrotthandel-hilbert.de. **Pflegemangel am Auftritt:** der Link der zweiten Mobilnummer zeigt auf `tel:+4915901232867`, also auf die Nummer von Damiano Hilbert und nicht auf 0160 977 44 177; außerdem stehen ungepflegte englische Demo-Blöcke einer Webflow-Vorlage auf der Seite ("Cagoworks", "Reliable delivery network", "Lorem ipsum", `hello@cagoworx.com`, `+1 (555) 000-0000`, "123 Sample St, 74RK, London UK"). **ANKAUF:** "Wir kaufen Buntmetalle im gesamten Raum Darmstadt zu Tageshöchstpreisen an", Abholung per Ladekran-LKW, "für die Abholung stellen wir einen kostenfreien Container bereit" - **keine Zahlenpreise**, keine Sortenliste mit Kursen. **VERKAUF/Dienstleistung:** Demontage und Abbrucharbeiten, Entrümpelung und Haushaltsauflösung "zu fairen Preisen" - ohne Zahlen; "ungekompliziertes Angebot" für Demontageprojekte. **GEBÜHREN:** keine Gebührentafel und kein Tarif; "kostenfreier Container" und die Übernahme des Abtransports sind Leistungszusagen, keine Preisposition. **Amtliches Fachbetrieberegister (Auszug 11.08.2026):** Darmstadt führt 8 Betriebe (Darmstädter Recycling Zentrum GmbH, ENTEGA AG, Eigenbetrieb für kommunale Aufgaben und Dienstleistungen, Merck KGaA (zweimal), NL Büdingen/BS Darmstadt Landwehrstraße 88, Niederlassung Rhein-Main Dolivostraße 11, uniroh Betriebsstätte Darmstadt Pallaswiesenstraße 206) - **kein Eintrag zu Hilbert**; das ist ein Negativbefund zum Registerstand und kein Beleg gegen eine nicht zertifizierungspflichtige Annahme. Quelle(n): https://schrotthandel-hilbert.de/ ; https://schrotthandel-hilbert.de/contact ; https://schrotthandel-hilbert.de/impressum ; https://g.co/kgs/m3HUvva ; https://api.openstreetmap.org/api/0.6/map.json ; https://schrottregister.pages.dev/ort-darmstadt]
