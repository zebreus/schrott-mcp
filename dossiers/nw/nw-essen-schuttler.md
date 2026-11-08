---
slug: nw-essen-schuttler
lat: 51.4732823
lon: 7.0776219
name: Schüttler
trader_type: mobil
state: NW
city: Essen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Schüttler

## Überblick

Identitätskandidat bleibt **Sylvia Schüttler Metallhandel**, Stubbenkamp 6, 45309 Essen-Kray, 0178 4299833, E-Mail-Lead `onvista@hotmail.com` – alles nur Verzeichnisangaben, kein belegter Alias des unspezifischen Seeds „Schüttler“. Die 11880-Seite kategorisiert u. a. Kleintransport, Montage, Altmetallrecycling, Möbeltransport und Schrotthandel, aber nennt keine Betreiberwebsite oder Öffnungszeiten. Creditreform-Suchauszug nennt weitere Handwerks-/Hausmeistertätigkeiten; Detailabruf HTTP 403, daher keine unabhängige Gewerbegegenstandsbestätigung. Keine öffentlich ausgelesene Ankauf-/Verkaufs-/Gebührenliste. Owner: Identität, heutige Tätigkeit und mobil versus öffentliche Annahmestelle klären; erst danach Adresse/GEO qualifizieren.

**Preise:** Ankauf: keine Schüttler-Kurse; Verkauf: keine Warenpreisliste; Gebühren: keine Dienstleistungstarife. Quelle (Verzeichnislead): https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html ; blockierter Lead: https://firmeneintrag.creditreform.de/45309/5110488342/SYLVIA_SCHUETTLER_METALLHANDEL

## Timeline

### Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`. Dieser Punkt liegt laut OSM auf der Straße „Freiheit“, 45128 Essen (way 46961889, `highway=service`, `bus=designated`, `vehicle=no`), genauer auf dem Fernbusbahnhof am Essener Hauptbahnhof (Haltestellenknoten 599294635, „Essen, Fernbusbahnhof“). Das ist eine Busspur und kein Gewerbestandort; in den Umweltinspektionsberichten der Bezirksregierung Düsseldorf für die Stadt Essen gibt es keinen Eintrag auf dieser Straße. Neu gefunden wurde eine bislang nicht erfasste Betreiber-Primärquelle: Das Impressum von „firma-schuettler.de“ (per CNAME identisch mit `xn--schttler-85a.com`) nennt Inhaberin Sylvia Schüttler, Stubbenkamp 6, 45309 Essen, Telefon 01577 1361724, und beschreibt als Tätigkeit Glasfaser-Verlegung und Innenausbau; die Startseite enthält keinen Treffer für „Metall“, „Schrott“ oder „Altmetall“. Die Nominatim-Vorwärtssuche zu Stubbenkamp 6 findet ein Wohnhaus (way 307631343, 51.4732855/7.0775926), rund 5,03 km vom gespeicherten Punkt entfernt; der Umfeldscan dort zeigt nur Textil-Altkleidercontainer, keinen Schrott- oder Metallhof. Damit widerspricht die eigene Betreiberquelle der Zeile dem gespeicherten Punkt, und der Punkt trägt für diese Zeile keine Aussage. `clear_fields: geo` nimmt ihn beim regulären Seed zurück; keine Ersatzkoordinate, weil Stubbenkamp 6 kein belegter Annahmeort ist, und keine Aussage über Betrieb, Annahme oder Fortbestand. Offen bleiben, ob Sylvia Schüttler heute Schrott oder Metall handelt (die Betreiberseite belegt ein anderes Gewerke), die Zuordnung der beiden Telefonnummern und der fehlende Register- oder Behördenbeleg; `status: pruefung` bleibt. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://www.xn--schttler-85a.com/impressum ; https://www.xn--schttler-85a.com/agb ; https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html ; https://nominatim.openstreetmap.org/search?q=Stubbenkamp+6%2C+45309+Essen&format=jsonv2&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/way/46961889/full.json ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 25.10.2026

- [Recherche 25.10.2026: 11880-Detailseite erneut direkt geprüft: „Sylvia Schüttler Metallhandel“, Stubbenkamp 6, 45309 Essen-Kray, 0178 4299833, `onvista@hotmail.com`; Branchen u. a. Kleintransport, Montage, Altmetallrecycling, Möbeltransport und Schrotthandel. Laut Seite stammen die Eintragsdaten vom 13.10.2025, Aktualisierung 04.10.2026. Das ist ein aktualisierter Aggregator-Lead, keine Betreiberbestätigung; keine Öffnungszeiten, aktuelle Annahmestelle oder konkrete Preise/Tarife. Ankauf, Verkauf und Gebühren bleiben ohne Preisangaben. E-Mail neu als unbestätigter Lead dokumentiert, nicht ins Frontmatter übernommen; GEO weiter erst nach Identitäts-/Standortklärung. Quelle: https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html]

### Recherche 11.10.2026

- [Recherche 11.10.2026: Bestehenden 11880-Kandidaten direkt revalidiert: Sylvia Schüttler Metallhandel, Stubbenkamp6/45309 Essen, 0178 4299833, keine Website/Zeiten; Daten vom 13.10.2025 trotz aktueller Anzeige 04.10.2026. Neuer Creditreform-Suchlead vorhanden, Detailabruf 403; Suchsnippet nicht als gelesene Primär-/Registerquelle oder zweite Bestätigung gezählt. Neue Suche identifizierte keinen aktuellen Betreiberauftritt. Keine Namenspräzisierung/Kontakt-/Annahme-Fills; pruefung bleibt. Generischer Preisrange-FAQ ist keine Ankaufspreisliste. Quelle(n): https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html ; https://firmeneintrag.creditreform.de/45309/5110488342/SYLVIA_SCHUETTLER_METALLHANDEL (403)]

### Recherche 01.10.2026

- [Recherche 01.10.2026: miss — nur ein Verzeichnis-Lead: 11880 nennt „Sylvia Schüttler Metallhandel, Stubbenkamp 6, 45309 Essen (Kray), 0178 4299833" (Schrotthandel/Kleintransport/Montage); kein zweiter unabhängiger Beleg, kein Betreiber-Impressum; Frontmatter bleibt leer, status pruefung; Quelle(n): nur Verzeichnis-Lead 11880.com, Stand 01.10.2026]

### Importiert (Seed-Stand 2026-09-30)

- mobil (GS)
- Adresse: Essen

### Recherche 03.10.2026

- [Recherche 03.10.2026: website_status leer → unbekannt; übrige Frontmatter unverändert. Die einzeln abgerufene 11880-Detailseite führt „Sylvia Schüttler Metallhandel“, Stubbenkamp 6, 45309 Essen-Kray, Mobilnummer und E-Mail sowie Altmetallrecycling/Schrotthandel; sie zeigt „Website hinzufügen“, keine Öffnungszeiten und keine zweite unabhängige Betreiber- oder Registerquelle. Der Eintrag ist laut 11880 am 27.06.2026 aktualisiert, die Eintragsdaten stammen vom 13.10.2025. Das reicht nicht für Fills; Name bleibt unpräzisiert „Schüttler“, Straße/Kontakt/Beschreibung bleiben leer, status bleibt pruefung. Quelle(n): https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html]


### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **51.4732823/7.0776219** (Mittel der 10 eindeutigen
  Way-Knoten von way/307631343; der Punkt liegt innerhalb der Objekt-BBox **51.4732306/7.0775334 – 51.4733340/7.0777194**).
  Quelle 1 (OSM): way/307631343 trägt `addr:housenumber`=**6**, `addr:street`=**Stubbenkamp**, `addr:city`=Essen,
  `addr:country`=DE, `addr:postcode`=**45309**, `building`=house — Dossier-Straße, -Hausnummer und -PLZ stimmen
  objektgenau überein; der Stubbenkamp trägt live **11** verschiedene Hausnummern (2, 4, 6, 8, 10, 1a, 1b, 3a, 3b, 5a,
  5b). Das Objekt ist in OSM als **Wohnhaus** kartiert und trägt **weder** `name=` **noch** `operator=`; ein Objekt mit
  Betriebsbezug zu „Schüttler“ ist nicht vorhanden — Aussage zum OSM-Bestand, **kein** Negativbeleg über den Betrieb.
  Quelle 2 (Betreiber/Register): das Impressum von `firma-schuettler.de` (CNAME-identisch mit `xn--schttler-85a.com`)
  nennt **Inhaberin Sylvia Schüttler, Stubbenkamp 6, 45309 Essen**, Telefon **01577 1361724**; das unabhängige
  11880-Profil „Sylvia Schüttler Metallhandel“ (Aktualisierung 04.10.2026) führt dieselbe Person und dieselbe Anschrift
  **Stubbenkamp 6, 45309 Essen (Kray)** mit **0178 4299833** und den Branchen Altmetallrecycling/Schrotthandel.
  **Abgrenzung (offen, bleibt offen):** die Betreiberseite beschreibt als Tätigkeit **Glasfaser-Verlegung und
  Innenausbau** und nennt weder „Metall“ noch „Schrott“; ob Frau Schüttler heute Schrott oder Metall handelt, ist
  damit **nicht** geklärt, ebenso die Zuordnung der beiden Telefonnummern. Die Koordinate bezeichnet die von beiden
  Quellen übereinstimmend genannte **Anschrift der Inhaberin**, **nicht** einen belegten Annahmehof (der Umfeldscan
  zeigt an der Anschrift nur Textil-Altkleidercontainer); `status: pruefung` bleibt unverändert. Der zuvor im
  Produktionsbestand gespeicherte Punkt **51.450284375/7.015053125** (Straße „Freiheit“, 45128 Essen) lag
  **5.033 m** entfernt und war für diese Zeile falsch.
  **Hinweis zum Frontmatter:** der Clear-Token `geo` wurde aus `clear_fields` entfernt, weil die Zeile jetzt eine
  kuratierte Koordinate trägt — ein gleichzeitig gesetztes `clear_fields: 'geo'` wird vom Seed-Validator als
  Konflikt abgewiesen (`clear_fields conflicts with populated geo`).
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/307631343/full.json ; https://www.xn--schttler-85a.com/impressum ;
  https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html (Abruf 03.11.2026).]

### Recherche 08.11.2026

- [Recherche 08.11.2026: **Amtliches Fachbetrieberegister: für den Essener Metallhandel „Sylvia Schüttler Metallhandel“ (Stubbenkamp 6, 45309 Essen-Kray) weder eine Efb- noch eine AltfahrzeugV-Anerkennung ausgewiesen; das Adressobjekt ist ein Wohnhaus ohne Betriebsbezug.** Geprüft wurden beide Routen des amtlichen Fachbetrieberegisters (https://fachbetrieberegister.gadsys.de/fachbetrieberegister/), Route „Entsorgungsfachbetriebe“ und Route „Altfahrzeugverwertung“, durchgängig mit dem Filter „Art des Betriebs = Egal“ (`is=1&e=1`); die Standortabfragen liefen mit validiertem `adr`-Parameter um die Dossieradresse (Nominatim-Abgleich 08.11.2026: 51.4732855/7.0775926) in den Radien 3 km und 10 km. **Route „Entsorgungsfachbetriebe“:** die Namenssuche `n=Schüttler` **0 Treffer**, `n=Schuettler` (ASCII-Schreibweise) **0 Treffer**; die Standortsuche 3 km um die Dossieradresse → **9 Treffer**, 10 km um 45309 Essen-Kray → **182 Treffer** — u. a. „WRL GmbH“, Emscherstrasse 27, 45327 Essen (ZZBT00300361 / 3, 4,5 km), „IST GmbH“, Stauderstraße 83-85, 45326 Essen (ZZBT00300413 / 2 und ZAET01500469 / 1, 4,8 km), „Franz Maag Autoverwertung EK“, Alte Bottroper Straße 46, 45143 Essen (ZAAS00300017 / 1, 7,0 km), „P & W Metall GmbH“, Ostuferstraße 8, 45356 Essen (ZZEE00300286 / 3, 7,6 km); **kein** Treffer trägt den Namen Schüttler, **keiner** liegt am Stubbenkamp 6, und keiner wurde übernommen. **Route „Altfahrzeugverwertung“:** `n=Schüttler` **0 Treffer**, `n=Schuettler` **0 Treffer**; 3 km um die Dossieradresse → **0 Treffer**, 10 km um 45309 Essen-Kray → **10 Treffer** (u. a. „Franz Maag Autoverwertung EK“, Alte Bottroper Straße 46, 45143 Essen, ZAAS00300017 / 1; „M+R Autoverwertung“, Vogelsanger Str. 385b, 50827 Köln) — sämtlich **fremd**. **Adressobjekt (neu):** OSM **way/307631343** trägt `addr:street=Stubbenkamp`, `addr:housenumber=6`, `addr:postcode=45309`, `addr:city=Essen`, `addr:suburb=Kray`, `building=house`, `building:levels=1` — **ohne** `name=`, `operator=`, `shop=` oder `craft=`-Tag; die Dossier-Adresse trägt damit **keinen** Betriebsbezug im Kartendatensatz. Die im Geokodierungsvermerk vom 03.11.2026 gesetzte Koordinate **51.4732823 / 7.0776219** deckt sich mit dem OSM-Objekt und der Nominatim-Auflösung (51.4732855/7.0775926) und bleibt damit **belegt**; die Seed-Anschrift selbst ist weiterhin **nur** ein Verzeichnis-Lead. **Zweitbeleg zur Lead-Existenz (Verzeichnis, nur Lead):** die 11880-Detailseite „Sylvia Schüttler Metallhandel“ (live gelesen) nennt unverändert Stubbenkamp 6, 45309 Essen (Kray), (0178) 4299833, `onvista@hotmail.com` und die Branchen „Kleintransport, Montagearbeiten, Altmetallrecycling, Möbeltransport, Schrotthandel“ — **keine** Öffnungszeiten, **keine** eigene Website; das sind **Plattformrubriken** und **keine** Betreiber- oder Leistungsbelege. Die im Dossier vom 01.10.2026 dokumentierte Sperrung des Creditreform-Eintrags (HTTP 403) verhindert weiterhin eine unabhängige Gewerbegegenstandsbestätigung; die dort genannten Tätigkeiten bleiben **Hinweise, kein Beleg**. **Ergebnis:** Zertifikatsnummer, Vorgangsnummer, Zertifizierungsorganisation, zertifizierte Tätigkeit und Standort sind für Sylvia Schüttler Metallhandel am Stubbenkamp 6 nicht ausgewiesen; `certifications` bleibt leer. **Preise (getrennt geführt):** **Ankauf** — keine belegten Kurse; **Verkauf** — keine Warenpreisliste; **Gebühren** — keine Dienstleistungstarife; die Portalkategorien begründen **weder** eine Vergütung **noch** eine Gebühr. **Reichweite der Aussage:** geprüft wurden zwei Namensvarianten in beiden Registerrouten, die Standortsuche in den Radien 3 km und 10 km um die Dossieradresse sowie 10 km um 45143 Essen; Abrufdatum 08.11.2026. **Ein Register-Nichteintrag ist kein Nachweis**, dass der Betrieb keine Anerkennung besitzt: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**. **Offen bleibt:** die Betreiberidentität, Rechtsform, heutige Tätigkeit und die Abgrenzung „mobil versus öffentliche Annahmestelle“ — der Seed bleibt `status: pruefung`, die Kontaktfelder bleiben leer.
  Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Sch%C3%BCttler ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Sch%C3%BCttler ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Stubbenkamp%206%2C%2045309%20Essen%2C%20Deutschland;7.0775926;51.4732855;7.098388;51.495106;7.056797;51.451465&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=45309%20Essen-Kray%2C%20Deutschland;7.0775926;51.4732855;7.177160;51.565727;6.978029;51.380844&dst=10&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Stubbenkamp%206%2C%2045309%20Essen%2C%20Deutschland;7.0775926;51.4732855;7.098388;51.495106;7.056797;51.451465&dst=3&is=1&e=1 ; https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html ; https://api.openstreetmap.org/api/0.6/way/307631343/full.json (Abruf 08.11.2026).]
