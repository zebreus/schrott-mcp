---
slug: nw-essen-schrotthandel-missal-metzler
clear_fields: geo
name: Schrotthandel Missal / Metzler
trader_type: schrotthaendler
state: NW
city: Essen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://schrotthandel-metzler-essen.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: '{"allowed": true, "conditions": "Kostenlose Schrott-/Altmetallabholung im Ruhrgebiet/NRW laut Betreiber; Demontage, Brennarbeiten, Sortierung und mögliche Entsorgungskosten vorab vereinbaren, nicht pauschal kostenlos. Ankaufpreis abhängig von Art, Menge und Aufwand."}'
provenance_section: Essen / Mülheim / Oberhausen / Bottrop
provenance_ankauf_raw: ja (mobil)
provenance_origin: table
---

# Schrotthandel Missal / Metzler

## Überblick

Mobile Betreiber-Sphäre Metzler/Jürgen Missal mit belegter kostenloser Altmetallabholung im Ruhrgebiet/NRW. Impressum nennt Jürgen Missal, Kalthofweg 4, zusätzlich „Vertretungsberechtigt: Markus Metzler“. Historische Zum-Oberhof-Adresse und rechtliche Rollen bleiben ungeklärt, daher keine Kontakt-/Adressfills. **Ankauf:** ausdrücklich nur „je nach Art und Umfang“; telefonische Preisvereinbarung, keine EUR/kg-Liste. **Verkauf:** keine separate Liste. **Gebühren:** Abholung wird kostenlos beworben, Preisberechnung berücksichtigt aber Demontage, Sortierung und mögliche Entsorgungskosten; keine pauschale kostenlose Demontage/Entrümpelung zusagen.

**GEO:** Kalthofweg 4 ist Impressums-/Kontaktadresse, Zum Oberhof 32 ein widersprüchlicher Portal-Lead; weder ist als Annahmestelle oder Abholstützpunkt bestätigt. Kein Betriebs-Pin bis Betreiberrollen, Adresse und Kundenannahme geklärt sind.

## Timeline

### Owner-Korrektur 27.10.2026: falsches GEO zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`; der Reverse-Lookup führt auf die Fernbus-Spur „Freiheit“ am Essener Hauptbahnhof und damit auf keinen Betriebsort. Betreiber-Impressum und 11880 belegen zwei Adressen derselben Betreiber-Sphäre: Kalthofweg 4, 45277 Essen (Jürgen Missal, Vertretungsberechtigt Markus Metzler) und Zum Oberhof 32, 45307 Essen (11880-Eintrag mit der im Dossier gespeicherten Domain, während die Betreiberseiten „Zum Oberhof“ nicht erwähnen). Beide liegen 4,07 bis 5,17 km vom gespeicherten Punkt entfernt und sind Wohnobjekte ohne belegten Annahmehof; Rollen- und Standortkonflikt zwischen Missal und Metzler bleiben offen, ebenso Registerstatus und Preislisten. Das gespeicherte Paar ist damit für genau diese Zeile falsch und wird mit `clear_fields: geo` zurückgenommen, ohne Ersatzkoordinate und ohne Annahmestellen-Behauptung; `status: aktiv` bleibt unverändert. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://schrotthandel-metzler-essen.de/impressum/ ; https://www.11880.com/branchenbuch/essen-ruhr/060440090B101936723/schrotthandel-juergen-missal.html ; https://www.11880.com/branchenbuch/essen-ruhr/120672194B37573047/markus-metzler-schrotthandel.html ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Vertiefung Nordwest 08.10.2026

- [Recherche 08.10.2026: Schrottpreise-Seite separat erneut direkt gelesen: ausdrücklich telefonische Angebote, Kalkulation abhängig von Menge/Zusammensetzung, Demontage/Sortierung und möglichen Entsorgungskosten. Keine EUR/kg-Zahlen, keine datierte Liste. Bereits heute präzisiertes pickup_json sachgerecht und unverändert; keine kostenlose Zusatzarbeit aus Werbebanner ableiten. Drei Betreiberdomains weiterhin nicht drei unabhängige Belege, Rollen-/Adressklärung offen. Verkaufspreisliste/Gebührentarife auf der geprüften Seite nicht veröffentlicht. Quelle: https://schrotthandel-metzler-essen.de/schrottpreise/]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Bestaetigung + neue Zweitspur, Klaerfall bleibt: alle drei Betreiber-Domains live re-verifiziert (schrotthandel-metzler-essen.de HTTP 200; schrotthandel-missal-essen.de HTTP 301; schrott-in-essen.de live) + schrottradar-Portal „Schrott und Metallhandel J. Missal" Essen. NEU: cylex-Eintrag „Schrotthandel Metzler, Zum Oberhof, Essen-Kray, 0175 9254..." stuetzt die Seed-Adresse Zum Oberhof 32 gegen das Impressum (Kalthofweg 4, Juergen Missal) — Widerspruch damit weiter unaufgeloest (Aggregator vs. Impressum); Feedback-Triage-Entscheid 01.10.2026 (Felder leer lassen) bleibt gueltig, kein Fill. Quelle(n): Live-Abrufe + cylex (02.10.2026).]

### Recherche 01.10.2026

- [Feedback-Triage 01.10.2026 (ID 2453): Adress-/Kontakt-Felder BLEIBEN LEER (Klaerfall bestaetigt) — Betreiber-Impressum (Einzelunternehmen Juergen Missal, Kalthofweg 4, 45277 Essen, Tel. 0170 3547895, schrotthandel-missal@web.de, ohne HRB) widerspricht Seed-Adresse 'Zum Oberhof 32'; Zweit-Domains schrotthandel-missal-essen.de + schrott-in-essen.de bestaetigen Kalthofweg 4 (alle eine Betreiber-Sphaere = formal EIN Beleg; keine Owner-Ausnahme ohne HRB). Firmania-Lead nennt Zum Oberhof 32, 45307 Essen als Alternativadresse — Widerspruch unaufgeloest, daher kein Adress-/Kontakt-Fill (Owner-Gate-Praezedenz). Schwerpunkt Abholung zweifelsfrei belegt ('Wir holen Schrott bei Ihnen kostenlos ab, Ruhrgebiet/NRW', Demontage/Brennarbeiten) → pickup_json gesetzt. Quelle(n): https://schrotthandel-metzler-essen.de/, https://schrotthandel-metzler-essen.de/impressum/ (Abruf 01.10.2026).]

- [Recherche 01.10.2026: Betreiber-Website live und tiefgecrawlt (Start, Schrottabholung, Kabelschrott, Metallhandel, Altmetall-Ankauf, Impressum einzeln abgerufen). Angebot: kostenlose mobile Schrott-/Altmetallabholung im Ruhrgebiet/NRW, Demontage/Brennarbeiten. Website per Regel auf Domain-Root normalisiert; website_status aktiv (eigener Live-Abruf 01.10.2026). Impressum (Einzelbeleg, unsicher — Einzelunternehmen Jürgen Missal, keine HRB, Owner-Direktive nicht anwendbar): Kalthofweg 4, 45277 Essen, Tel. 0170 3547895, schrotthandel-missal@web.de — weicht von Seed-Adresse „Zum Oberhof 32" ab; Straße/PLZ NICHT überschrieben (Klärfall Adresswiderspruch für Folgewelle). Kontaktfelder daher leer gelassen. Quelle(n): https://schrotthandel-metzler-essen.de/schrottabholung/, https://schrotthandel-metzler-essen.de/impressum/.]

### Importiert (Seed-Stand 2026-09-30)

- Abholung, Demontage, Brennarbeiten (Zum Oberhof 32)

### Recherche 08.10.2026

- [Korrektur 08.10.2026: Owner hat Schrottpreise-Seite erneut gelesen und die bisherige pauschale inkl.-Formulierung in pickup_json präzisiert. Kostenlose Abholung bleibt Betreiberwerbung; Demontage-/Sortieraufwand und mögliche Entsorgungskosten fließen laut derselben Betreiberseite in die Preiskalkulation ein. Keine Zusage kostenloser Zusatzarbeiten. Quelle: https://schrotthandel-metzler-essen.de/schrottpreise/]

- [Recherche 08.10.2026: Impressum, tatsächliche Schrottpreis-Seite und Ankaufseite einzeln live gelesen. Impressum enthält neben Jürgen Missal auch Vertretungsberechtigt Markus Metzler; keine HRB, USt-ID derzeit nicht erteilt. Beide Preis-/Ankaufseiten nennen 0170 3547895 UND 0175 9254651; letztere passt zur früheren Metzler-Verzeichnisspur, beweist aber keine Umzugs-/Rechtsträgerkette. Owner-Gate von 01.10. bleibt: keine Kontakt-/Adressfills, alle Domains derselben Sphäre kein unabhängiger Zweitbeleg. Quelle(n): https://schrotthandel-metzler-essen.de/impressum/ ; https://schrotthandel-metzler-essen.de/schrottpreise/ ; https://schrotthandel-metzler-essen.de/altmetall-alteisen-ankauf-essen-ruhrgebiet-nrw-fa-metzler/]
- [Recherche 08.10.2026: PREISPRÄZISIERUNG ANKAUF: je nach Art und Umfang möglich, tagesaktuelle telefonische Angebote, keine numerische öffentliche Liste. Schrottpreise-Seite nennt Zusammensetzung/Menge, Demontage-/Sortieraufwand und mögliche Entsorgungskosten als Kalkulationsfaktoren. VERKAUF: keine separate Preisliste in den geprüften Seiten. GEBÜHREN: kostenlose Schrottabholung beworben, nicht automatisch kostenlose Zusatzarbeiten; keine Tariftabelle. Vorhandenes pickup_json bleibt erhalten, aber dessen Formulierung inkl. Demontage/Brennarbeiten darf nicht als Zusage kostenloser Zusatzarbeiten ausgelegt werden. Quelle(n): https://schrotthandel-metzler-essen.de/schrottpreise/ ; https://schrotthandel-metzler-essen.de/altmetall-alteisen-ankauf-essen-ruhrgebiet-nrw-fa-metzler/]

### Recherche 20.10.2026

- [Recherche 20.10.2026: Schrottpreise-Seite direkt erneut abgerufen. Sie verlangt für ein konkretes Angebot den Anruf und nennt keine EUR/kg-Werte oder datierte Tabelle; Zusammensetzung/Menge, Demontage-/Sortieraufwand und mögliche Entsorgungskosten wirken laut Text auf die Preisberechnung. Seite wirbt zugleich mit kostenloser Abholung im Ruhrgebiet/NRW. ANKAUF: telefonische Einzelfallpreise, keine numerische Preisliste. VERKAUF: keine separate Produkt-/Verkaufspreisliste. GEBÜHREN: kostenlose Abholung behauptet; Zusatzarbeiten/Entsorgungskosten können anfallen und sind nicht pauschal kostenfrei, aber keine Tarife ausgewiesen. Identitäts-/Adresskonflikt aus bisherigen Einträgen bleibt offen; Frontmatter nicht geändert. GEO offen: keine Zuordnung von Kalthofweg 4 oder Zum Oberhof 32 als Schrottannahme; mobiler Dienst ist kein bestätigter Hof. Quelle: https://schrotthandel-metzler-essen.de/schrottpreise/]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Für diese Zeile sind **zwei** Betriebsadressen
  im Umlauf, und der Adresskonflikt ist im Dossier **ausdrücklich offen** (Owner-Gate vom 01.10.2026); eine
  Festlegung wäre eine Ermessensentscheidung und keine geprüfte Geokodierung. **Was live geprüft wurde (03.11.2026):**
  (a) **Beide** Kandidaten sind in OSM als Adressobjekte erfasst: **way/318494685** mit `addr:housenumber`=**4**,
  `addr:street`=**Kalthofweg**, `addr:postcode`=**45277**, `addr:suburb`=Überruhr-Hinsel, `building`=yes (Mittel der 4
  eindeutigen Way-Knoten **51.4257355/7.0783742**) und **way/88583555** mit `addr:housenumber`=**32**,
  `addr:street`=**Zum Oberhof**, `addr:postcode`=**45307**, `building`=apartments (Mittel der 4 eindeutigen Way-Knoten
  **51.4568962/7.0728580**); die beiden Objekte liegen **3.486 m** auseinander. (b) Beide Objekte tragen **weder** `name=`
  **noch** `operator=`; eine Suche über `name`, `operator`, `brand`, `website`, `contact:website`, `description`, `note`
  und `short_name` nach **„Missal“** und **„Metzler“** im Essener Kachelraster **6.9400/51.3900 – 7.1600/51.5300**
  liefert **0** Treffer. (c) Die **Schwesterzeile** `nw-essen-schrott-und-metallhandel-j-missal` führt für die
  Betreiberanschrift **Kalthofweg 4** dieselbe Quellenlage; dort ist die Anschrift gesetzt. Quelle 1 (Betreiber/
  Register): das Impressum der Betreiber-Website **schrotthandel-metzler-essen.de/impressum** nennt **Jürgen Missal**
  **und** den Vertretungsberechtigten **Markus Metzler** an **Kalthofweg 4, 45277 Essen** (Telefon **0170 3547895**,
  keine HRB, USt-ID nicht erteilt) — die Betreiberquelle spricht damit für Kalthofweg 4. Dagegen steht der
  Verzeichniseintrag „Markus Metzler Schrotthandel“, **Zum Oberhof 32, 45307 Essen** (11880), der die im Dossier
  geführte Seed-Adresse trägt und die von den Betreiberseiten **nicht** erwähnt wird; beide Adressen sind in OSM
  **Wohnobjekte ohne Betriebsbezug**. Quelle 2 (OSM): beide Kandidaten sind adressgenau erfasst, eine Auswahl ist
  ohne Betreiberentscheidung nicht begründbar. **Was fehlt:** die Klärung, ob der Betrieb Missal oder der Betrieb Metzler
  an der jeweils anderen Anschrift sitzt (Betreiberbestätigung oder Registerauszug); erst dann ist ein Punkt für diese
  Zeile setzbar. `street`/`postcode` bleiben leer, `clear_fields: 'geo'` bleibt bestehen, `status: aktiv` bleibt
  unverändert, es wird **keine** Annahme- oder Standortbehauptung aufgestellt.
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/318494685/full.json ;
  https://api.openstreetmap.org/api/0.6/way/88583555/full.json ; https://schrotthandel-metzler-essen.de/impressum/ ;
  https://www.11880.com/branchenbuch/essen-ruhr/120672194B37573047/markus-metzler-schrotthandel.html (Abruf 03.11.2026).]

### Geokodierung 05.11.2026

- [Geokodierung 05.11.2026: **Klärfall - keine Koordinate gesetzt; die Nachprüfung bestätigt den Vermerk vom
  03.11.2026.** Für diese Zeile sind **zwei** Betriebsadressen im Umlauf, und der Adresskonflikt ist im Dossier
  **ausdrücklich offen** (Owner-Gate vom 01.10.2026); eine Festlegung wäre eine Ermessensentscheidung und keine
  geprüfte Geokodierung. **Quelle 1 (OSM, live geladen am 05.11.2026, HTTP 200):** (a) **beide** Kandidaten sind in
  OSM als Adressobjekte erfasst: **way/318494685** mit `addr:housenumber`=**4**, `addr:street`=Kalthofweg,
  `addr:postcode`=**45277**, `addr:suburb`=Überruhr-Hinsel, `building`=yes (Mittel der **4** eindeutigen Way-Knoten
  **51.4257355/7.0783742**, BBox 51.4256888/7.0783024 bis 51.4257824/7.0784476) und **way/88583555** mit
  `addr:housenumber`=**32**, `addr:street`=Zum Oberhof, `addr:postcode`=**45307**, `building`=apartments (Mittel der
  **4** eindeutigen Way-Knoten **51.4568962/7.0728580**, BBox 51.4568163/7.0727197 bis 51.4569761/7.0729962); die
  beiden Objekte liegen rund **3,49 km** auseinander. (b) das Kachelraster **7.0500/51.4100 bis 7.1000/51.4800**, **35
  Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**
  (https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0500,51.4100,7.1000,51.4800) **umschließt beide Objekt-BBoxes
  vollständig** und enthält die **„Kalthofweg“** (Verlauf 51.4257925/7.0779563 bis 51.4262281/7.0796087) mit **18** Adressobjekten; sie weist **18** verschiedene Hausnummern (1, 10, 11, 12, 13, 14, 15, 16, 18, 2, 20, 3, 4, 5, 6, 7, 8, 9) sowie den **Zum
  Oberhof** (Verlauf 51.4555093/7.0687763 bis 51.4574307/7.0778756) mit **36** Adressobjekten; sie weist **36** verschiedene Hausnummern (1, 10, 11, 12, 13, 14, 15, 16, 17, 19, 2, 20, 21, 22, 23, 24, 26, 28, 3, 30, 32, 34, 36, 38, 4, 5, 50, 52, 6, 62, 64, 66, 68, 7, 8, 9) - **beide** Dossier-Hausnummern existieren in OSM. (c) beide
  Objekte tragen **weder** `name=` **noch** `operator=`; eine Suche nach „Missal" und „Metzler" über die Felder name,
  operator, brand, description, website, contact:website, note und addr:housename findet im Kachelausschnitt **keinen**
  Treffer - Aussage zum OSM-Bestand, **kein** Negativbeleg über den Betrieb. **Quelle 2 (Betreiber/Register, im Dossier
  bereits zitiert):** das Impressum der Betreiber-Website **schrotthandel-metzler-essen.de/impressum** nennt **Jürgen
  Missal** **und** den Vertretungsberechtigten **Markus Metzler** an **Kalthofweg 4, 45277 Essen** (Telefon **0170
  3547895**, keine HRB, USt-ID nicht erteilt) - die Betreiberquelle spricht damit für Kalthofweg 4. Dagegen steht der
  Verzeichniseintrag „Markus Metzler Schrotthandel", **Zum Oberhof 32, 45307 Essen** (11880), der die im Dossier
  geführte Seed-Adresse trägt und der von den Betreiberseiten **nicht** erwähnt wird; beide Adressen sind in OSM
  **Wohnobjekte ohne Betriebsbezug**. Die **Schwesterzeile** `nw-essen-schrott-und-metallhandel-j-missal` führt für die
  Betreiberanschrift **Kalthofweg 4** dieselbe Quellenlage; dort ist die Anschrift gesetzt. **Warum keine Koordinate:**
  Die Wahl zwischen den beiden Adressen bleibt eine Ermessensentscheidung, solange das Owner-Gate vom 01.10.2026 offen
  ist; jeder Punkt würde eine Rollen- und Anschriftenkette behaupten, die keine Quelle trägt. **Was zur Auflösung
  fehlt:** die Klärung, ob der Betrieb Missal oder der Betrieb Metzler an der jeweils anderen Anschrift sitzt
  (Betreiberbestätigung oder Registerauszug); erst dann ist ein Punkt für diese Zeile setzbar. `street`/`postcode`
  bleiben leer, `clear_fields: 'geo'` bleibt bestehen, `status: aktiv` bleibt unverändert, es wird **keine** Annahme-
  oder Standortbehauptung aufgestellt. **Reichweitenklausel:** das Fehlen eines OSM-Betriebsbezugs oder einer
  Hausnummer wäre kein Existenz- oder Schließungsbeweis. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/318494685/full.json ;
  https://api.openstreetmap.org/api/0.6/way/88583555/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0500,51.4100,7.1000,51.4800 ;
  https://schrotthandel-metzler-essen.de/impressum/ ;
  https://www.11880.com/branchenbuch/essen-ruhr/120672194B37573047/markus-metzler-schrotthandel.html (im Dossier
  zitiert, Abruf 05.11.2026).]

