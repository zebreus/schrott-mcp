---
slug: bb-lichtenow-schrottlieb
name: Schrottlieb
trader_type: schrotthaendler
state: BB
city: Lichtenow
street: Chausseestr. 22A
postcode: ''
lat: 52.4802478
lon: 13.9068491
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Schrottlieb, Chausseestr. 22A, 15345 Lichtenow (Ortsteil von Rüdersdorf bei Berlin): Eintrag eines Schrottbetriebs ohne belegten Betrieb. Das amtliche Fachbetrieberegister kennt weder unter dem Namen Schrottlieb (0 Treffer in beiden Routen, Abfrage 18.11.2026) noch im 3-km-Radius um die Anschrift einen anerkannten Betrieb; in OpenStreetMap existiert die Hausnummer 22a als reiner Adressknoten ohne name-, operator- oder Branchentag. Die Domain schrottlieb.de liefert am 18.11.2026 wieder HTTP 200, aber nur mit einer Anubis-Bot-Schutz-Challenge ohne lesbaren Inhalt - also weiterhin kein lesbarer Betreiberauftritt. Ankauf, Verkauf und Gebühren sind nicht belegt; der Status bleibt pruefung.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Schrottlieb

## Überblick

Schrottlieb, Chausseestraße 22A, 15345 Lichtenow: Eintrag eines Schrottbetriebs, für den es
keinen Beleg für einen tatsächlichen Betrieb gibt. Die Domain schrottlieb.de ist seit dem
02.10.2026 tot und am 28.10.2026 nicht auflösbar; das amtliche Fachbetrieberegister kennt für
Lichtenow keinen anerkannten Betrieb, und in OpenStreetMap gibt es dort kein Objekt mit
Betriebsbezug. Die Hausnummer 22a ist zwar als reiner Adressknoten erfasst, ohne Name und
Schrottplatz-Tag, weshalb kein Punkt gesetzt wird. Die Herkunft der Anschrift ist offen: Das
Gemeindemagazin von Rüdersdorf bei Berlin führt Chausseestraße 22a im Jahr 2011 als Anschrift
des Sportvereins Glück Auf. Ankauf, Verkauf und Gebühren sind nicht belegt. Der Status bleibt
pruefung, da weder eine Schließung noch eine Nichtexistenz behauptet wird; offen bleibt die
Gewerbeauskunft der Gemeinde zu Name, Gewerbestand und Tätigkeit.

## Timeline

- [Owner-Korrektur 29.10.2026 (Korrekturvermerk): **Drei Aussagen dieses Dossiers waren falsch und sind berichtigt.** (1) Der Korrekturvermerk vom 28.10.2026 nannte als Seed-Adresse „Juri-Gagarin-Ring 10, 19370 Parchim“ — das ist die Seed-Adresse eines **anderen** Dossiers (`mv-parchim-19370-schrotthandel-parchim`); die Seed-Adresse dieses Dossiers ist **Chausseestr. 22A, 15345 Lichtenow**. (2) Derselbe Vermerk behauptete, eine Hausnummer 22A sei nicht erfasst — sie **ist** erfasst (node 8058385583). Richtig ist: Der Node trägt nur Adress-Tags (`addr:street`, `addr:housenumber`, `addr:postcode`, `addr:suburb`) und **keinen** Betriebsbezug (kein `name`, kein `shop`, kein `industrial=scrap_yard`); deshalb wird für die Seed-Adresse kein Punkt gesetzt. Quellen: node 8058385583 live abgerufen am 29.10.2026. (3) Die Notiz vom 28.10.2026 stützte sich auf einen OSM-Kartenausschnitt `bbox=13.9085,52.4775,13.9135,52.4815`, der die Hausnummer 22a **nicht** umfasste (der Node liegt bei longitude 13.9068491) — die Aussage „nicht einmal als Adressknoten erfasst“ war daher ein Abfrage-Artefakt; die im Kartenausschnitt genannten Objekte sind dort live vorhanden: die Tankstelle „GO“ (node 277249761, `amenity=fuel`) und das Restaurant „Haweli“ (node 7037683717, `amenity=restaurant`) an Chausseestraße 32, der „Kundenparkplatz Eiscafé Pinguin“ (node 4681765012, `amenity=parking` — ein Eiscafé-Objekt ist dort **nicht** erfasst) sowie die „Lichtenower Ziegelei“ (way 609921922, `disused:man_made=kiln`); ein „Busbahnhof Lichtenow“ ist in OSM **nicht** erfasst, dort stehen nur die Haltestellen „Lichtenow, Tankstelle“.]

- [Offene Punkte 28.10.2026: (1) Gewerbeauskunft der Gemeinde **Rüdersdorf bei Berlin** (Ortsteil Lichtenow) zu **Chausseestraße 22A** — Name, Gewerbestand, Tätigkeit Schrott/Metall. (2) Ob der Eintrag „Schrottlieb“ überhaupt je ein Gewerbebetrieb war oder eine Verzeichnis-/Portalmarke ohne Betrieb ist, ist ungeklärt; das Dossier bleibt deshalb `pruefung` (keine Schließungsbehauptung, keine Nichtexistenzbehauptung). (3) Für den Fall einer Bestätigung wäre **Chausseestraße 22A, 15345 Lichtenow** neu zu geokodieren (die Hausnummer 22A **ist** in OSM erfasst — node 8058385583, Fassung 2 vom 31.01.2025).]

- [Owner-Korrektur 28.10.2026 (zweiter Korrekturvermerk): **Die Angabe zur OSM-Erfassung war von Anfang an falsch und ist berichtigt.** Die Hausnummer **22a** der Chausseestraße in 15345 Lichtenow ist in OSM erfasst — node 8058385583 (`addr:street=Chausseestraße`, `addr:housenumber=22a`, `addr:postcode=15345`, `addr:suburb=Lichtenow`, Fassung 2 vom 31.01.2025, also lange vor der Welle). Die frühere Angabe, OSM kenne dort keine Hausnummer, war falsch; für die Seed-Adresse (Chausseestr. 22A in Lichtenow; PLZ 15345 laut OSM) bleibt es bei „kein Geokodierpunkt“, weil der Node nur Adress-Tags trägt und keinen Betriebsbezug aufweist. Quellen: node 8058385583 live abgerufen am 28.10.2026.]

- [Owner-Korrektur 28.10.2026 (Korrekturvermerk): **Eine Formulierung war ungenau und ist präzisiert.** die Formulierung „OSM kennt dort derzeit keine Hausnummer“ war ungenau: Die Straße Chausseestraße ist in OSM erfasst (Nominatim-Reverse auf „Chausseestraße, 15345 Lichtenow“), eine **Hausnummer 22a** ist jedoch erfasst (node 8058385583, Fassung 2 vom 31.01.2025). Die Aussage „kein Geokodierpunkt für die Seed-Adresse“ bleibt richtig, weil der Node keinen Betriebsbezug trägt. Quellen: Nominatim-Reverse auf die genannte Adresse, live abgerufen am 28.10.2026.]

### Recherche 28.10.2026 (Shard C, 6h-Welle): Domain weiterhin nicht auflösbar, Register ohne Treffer

- [Recherche 28.10.2026: Die Domain **schrottlieb.de** ist am 28.10.2026 **nicht mehr auflösbar** (DNS-Fehler „getaddrinfo ENOTFOUND“ für die Domain sowie für ihre üblichen Hostnamen-Varianten mit und ohne die Www-Präfix-Sform und mit Schema) — der Befund vom 02.10.2026 (Domain tot) besteht unverändert fort; es gibt keine Inhalte, keine Kontaktdaten und keine Preise mehr abzurufen. Quellen: DNS-Abfragen für die Domain und ihre Hostnamen-Varianten (Abruf 28.10.2026).]
- [Recherche 28.10.2026: Amtliches Fachbetrieberegister (Auszug 11.08.2026, Spiegel SchrottRegister) hat **keine Ortsseite für Lichtenow** (Ortsteil von Rüdersdorf bei Berlin, 15345) — dort ist kein anerkannter Entsorgungsfachbetrieb verzeichnet. Negativbefund **ohne** Beweiswert (Efb-Anerkennung freiwillig). Quellen: https://schrottregister.pages.dev/ (Abruf 28.10.2026).]
- [Recherche 28.10.2026: OpenStreetMap-Abgleich um **Chausseestraße 22, 15345 Lichtenow** (`api.openstreetmap.org/api/0.6/map`): **kein** Objekt „Schrottlieb“ und kein `industrial=scrap_yard`; benannt sind dort die Tankstelle „GO“ (Chausseestraße 32), das Restaurant „Haweli“, das Eiscafé Pinguin, der Busbahnhof Lichtenow und das als ehemalige Ziegelei benannte Gebäude „Lichtenower Ziegelei“. Die Hausnummer **22A** ist in OSM erfasst (node 8058385583, `addr:housenumber=22a`, `addr:street=Chausseestraße`, `addr:postcode=15345`, Fassung 2 vom 31.01.2025) — der oben zitierte Kartenausschnitt umfasste diese Hausnummer jedoch nicht, weshalb die frühere Angabe „nicht einmal als Adressknoten erfasst“ ein Abfrage-Artefakt war; der Node trägt nur Adress-Tags und **keinen** Betriebsbezug, die Seed-Adresse „Chausseestr. 22A“ ist damit durch OpenStreetMap **nicht** als Betriebsstandort gedeckt. Quelle: https://api.openstreetmap.org/api/0.6/map?bbox=13.9085,52.4775,13.9135,52.4815 (Abruf 28.10.2026).]
- [Recherche 28.10.2026: Adress-Hinweis (kein Beleg, nur Plausibilität): Das Gemeindemagazin „Rüdersdorf 2011“ der Gemeinde Rüdersdorf bei Berlin nennt für **Chausseestraße 22a, 15345 Rüdersdorf bei Berlin OT Lichtenow** den **Sportverein „Glück Auf“ Rüdersdorf e.V.** (Ansprechpartner Karl-Heinz Eichhorn). Das ist ein Hinweis darauf, dass die Hausnummer im Jahr 2011 **nicht** von einem Schrottbetrieb genutzt wurde; für 2026 ist daraus nichts abzuleiten. Quelle: http://www.stadtmagazinverlag.de/orte/ruedersdorf7/rudersdorf7.pdf (Abruf 28.10.2026).]
- [Preise/Ankauf/Verkauf/Gebühren 28.10.2026: **ANKAUF** — keine Kurse, kein Sortiment, keine Annahmebedingungen belegt. **VERKAUF** — keine Verkaufspreise belegt. **GEBÜHREN** — keine Tarife belegt. Die tote Domain trägt keine Preisinformation mehr; es gibt keinen Betreiberauftritt, aus dem Preise, Konditionen oder Gebühren abgeleitet werden könnten. Quellen: siehe oben.]
- [Offene Punkte 28.10.2026: (1) Gewerbeauskunft der Gemeinde **Rüdersdorf bei Berlin** (Ortsteil Lichtenow) zu **Chausseestraße 22A** — Name, Gewerbestand, Tätigkeit Schrott/Metall. (2) Ob der Eintrag „Schrottlieb“ überhaupt je ein Gewerbebetrieb war oder eine Verzeichnis-/Portalmarke ohne Betrieb ist, ist ungeklärt; das Dossier bleibt deshalb `pruefung` (keine Schließungsbehauptung, keine Nichtexistenzbehauptung). (3) Für den Fall einer Bestätigung wäre **Chausseestraße 22A, 15345 Lichtenow** neu zu geokodieren (die Hausnummer 22A **ist** in OSM erfasst — node 8058385583, `addr:street=Chausseestraße`, `addr:housenumber=22a`, `addr:postcode=15345`, `addr:suburb=Lichtenow`, Fassung 2 vom 31.01.2025 — die frühere Angabe, OSM kenne dort keine Hausnummer, war falsch).]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (dünner Beleg, GS 0 — behalten mit Abgleich-Vermerk)
- Adresse: Lichtenow, Chausseestr. 22A

### Recherche 01.10.2026

- [Recherche 01.10.2026: miss — keine Treffer zu Betreiber-Website/Register; nur duenner Adress-Lead; Statusfeld unveraendert; Quelle(n): Verzeichnis-Lead]

### Recherche 02.10.2026

- [Recherche 02.10.2026: miss bestaetigt - Domain schrottlieb.de TOT (curl 000, 02.10.2026), Das Oertliche 0 Treffer Schrottlieb/Lichtenow UND Schrott/Lichtenow (naechster Treffer Letschin); keine Fills, status pruefung; Quelle(n): curl-Statuscheck schrottlieb.de + Das-Oertliche-Negativsuche 02.10.2026]

### Geokodierung 08.11.2026

- [Geokodierung 08.11.2026: **Koordinate der Dossier-Adresse belegt** (`lat`/`lon` = **52.4802478/13.9068491**). Punktgüte: **Hausnummer-Treffer**; der Dossierbetrieb „Schrottlieb“ ist über die Adresse **Chausseestr. 22A, Lichtenow** adressiert; das OSM-Adressobjekt **n/8058385583** trägt `addr:housenumber`=22a, `addr:street`=Chausseestraße, `addr:postcode`=15345, `addr:city`=Rüdersdorf bei Berlin; das Objekt trägt **weder `name=` noch `operator=`** - die Koordinate belegt die **Anschrift**, nicht die Betreiberzuordnung; der Punkt ist die Knotenkoordinate; die Nominatim-Vorwärtssuche nach „Chausseestr. 22A,  Lichtenow“ liefert unter den 1 Treffern dasselbe Objekt **n/8058385583**; die OSM-Adresse führt `addr:city`=Rüdersdorf bei Berlin und `addr:postcode`=15345 - Lichtenow ist ein Ortsteil von Rüdersdorf bei Berlin, das Frontmatter führt den Ortsteil, aber keine Postleitzahl.]
  Quelle(n): https://nominatim.openstreetmap.org/search?format=jsonv2&extratags=1&namedetails=1&addressdetails=1&countrycodes=de&limit=20&q=Chausseestr.+22A%2C++Lichtenow ; https://api.openstreetmap.org/api/0.6/node/8058385583.json (Abruf 08.11.2026).]

### Recherche 18.11.2026

- [Recherche 18.11.2026: **Die Domain-Befunde vom 28.10.2026 haben sich geändert und werden hier durch eine eigene datierte Sektion korrigiert, ohne den alten Vermerk zu überschreiben.** Die Feststellung vom 28.10.2026 lautete, schrottlieb.de sei am 28.10.2026 nicht mehr auflösbar (DNS-Fehler). **Am 18.11.2026 löst die Domain wieder auf** (DNS-Eintrag vorhanden) und antwortet auf https://schrottlieb.de/ mit **HTTP 200**, liefert aber nur die Anubis-Bot-Schutz-Seite (Set-Cookie: techaro.lol-anubis-cookie-verification-200cedf8) und den Text Loading... Anubis could not load its JavaScript - **kein lesbarer Betreiberinhalt, kein Impressum, keine Preise**. Die Domain ist damit nicht mehr als nicht auflösbar zu führen, bleibt aber ein **nicht lesbarer Betreiberauftritt**; es wird weder ein Betrieb angenommen noch eine Schließung behauptet. **Amtliches Fachbetrieberegister (live am 18.11.2026):** die Namenssuche Schrottlieb liefert in der Route **Entsorgungsfachbetriebe 0 Treffer** und in der Route **Altfahrzeugverwertung 0 Treffer**; die Ortsabfrage mit 3 km Radius um Chausseestraße 22A, 15345 Lichtenow liefert in **beiden** Routen **0 Treffer**. **Reichweite:** die Efb-Anerkennung nach § 56 KrWG und die Altfahrzeugverwertungsanerkennung nach § 7 Abs. 2a AltfahrzeugV sind freiwillig; ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis und **keine** Schließungsaussage, zumal ein nicht anerkannter Einzelbetrieb in diesen Registern gar nicht erscheinen muss. **OpenStreetMap:** der Node n/8058385583 (addr:housenumber 22a, addr:street Chausseestraße, addr:postcode 15345, addr:city Rüdersdorf bei Berlin) ist unverändert nur ein Adressknoten ohne name-, operator- oder Branchentag; die bestehende Koordinate 52.4802478/13.9068491 bleibt damit eine Anschrifts-, keine Betriebsangabe. **Frontmatter:** `description` neu gefüllt (bisher leer) mit dem belegten Sachstand; `website_status` von leer auf **unbekannt** normalisiert; `status` bleibt pruefung, `street`, `postcode`, `phone` bleiben unverändert. **ANKAUF:** keine Kurse, kein Sortiment, keine Annahmebedingungen belegt; **VERKAUF:** keine Verkaufspreise; **GEBÜHREN:** keine Tarife. **Offen bleibt** die Gewerbeauskunft der Gemeinde Rüdersdorf bei Berlin zu Name, Gewerbestand und Tätigkeit Schrott/Metall an der Chausseestraße 22A sowie die Frage, ob der Eintrag Schrottlieb überhaupt je ein Gewerbebetrieb war. Quelle(n): https://schrottlieb.de/ (HTTP 200 mit Anubis-Challenge, Abruf 18.11.2026) ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrottlieb ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schrottlieb ; https://api.openstreetmap.org/api/0.6/node/8058385583.json (Abruf 18.11.2026)]
