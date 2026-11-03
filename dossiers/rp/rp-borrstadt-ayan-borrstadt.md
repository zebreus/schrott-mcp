---
slug: rp-borrstadt-ayan-borrstadt
lat: 49.5837123
lon: 7.9416515
name: Ayan Börrstadt
trader_type: sonstige
state: RP
city: Börrstadt
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
provenance_section: Nachtrag Wide-Net PROSE-Cluster (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Ayan Börrstadt

## Überblick

Die Betreiberwebsite und zwei dort hochgeladene SMU-Zertifikate nennen **R. Ayan Autorecycling/Schrotthandel** an der Kaiserstraße 5 in Börrstadt. Die Zertifikate wurden nicht bei der ausstellenden Stelle verifiziert; weder sie noch die Betreiberwebsite sind eine unabhängige zweite Quelle zur Zuordnung des bestehenden Seed-Datensatzes `Ayan Börrstadt`. Deshalb bleiben Name, Typ, Anschrift, Aktivstatus und Kundenannahme ungeändert bzw. ungefüllt. `status: pruefung`; keine aktive Annahme oder Auszahlung behauptet. Die genannten Preis- und Serviceaussagen bleiben Recherche-Leads.

**Ankauf:** Betreiber wirbt mit Tagespreisen auf Anfrage für u. a. Kupfer, Kabel, Messing, Zink, Aluminium, Mischschrott, Eisen, V2A, Batterien und Elektromotoren; keine Zahlenkurse. **Verkauf:** allgemeiner An-/Verkauf von Schrott genannt, aber keine Materialpreisliste; die Website sagt ausdrücklich, dass keine gebrauchten Kfz-Teile verkauft werden. **Gebühren:** Containerdienst und Abholung werden beworben, jedoch ohne konkrete Tarife; keine pauschal kostenlose Abholung behaupten.

## Timeline

### Owner-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Seed-Diff-Review bestätigt, dass die aktuellen Betreiberangaben und beide SMU-Zertifikate auf der Betreiberwebsite selbst gehostet werden; ein unabhängiger zulässiger Zweitbeleg für die Verbindung zur bisherigen Zeile `Ayan Börrstadt` fehlt. Die vorgeschlagenen Änderungen an Name, Typ, Anschrift, Website, Status und Drop-off werden daher zurückgestellt; die unvollständige importierte Straßenzeichenfolge wird nicht als Adresse übernommen. Read-only Produktionspunkt 49.5777/7.9425 löst auf Gebäude Gartenweg 1, nicht auf Kaiserstraße 5; `clear_fields: 'geo'` entfernt den falschen Punkt ohne Ersatzkoordinate. Quellen: https://www.schrotthandel-ayan.de/ ; https://www.schrotthandel-ayan.de/impressum ; https://www.schrotthandel-ayan.de/materialankauf ; https://www.schrotthandel-ayan.de/uploads/2ULsB1WR/Entsorgungsfachbetrieb.pdf ; https://www.schrotthandel-ayan.de/uploads/PPUnQXUS/anerkannter-Demontagebetrieb.pdf ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.5777&lon=7.9425&zoom=18&addressdetails=1 ; README.md, Quellenhierarchie.]

### Recherche 26.10.2026

- [Recherche 26.10.2026: Die zwei auf der Betreiberwebsite veröffentlichten SMU-Zertifikate visuell geprüft: Zertifikat Entsorgungsfachbetrieb SMU/1195/001, Kontrolle 24.02.2026, ausgestellt 17.03.2026, gültig bis 23.08.2027; das Zertifikat führt „R. Ayan Schrotthandel, Autorecycling, Containerdienst“, Kaiserstraße 5, 67725 Börrstadt und Tätigkeiten Sammlung/Transport. Das Demontagebetriebs-Zertifikat nennt ebenfalls R. Ayan Autorecycling an dieser Anschrift, Prüfung 24.01.2026, ausgestellt 09.03.2026, nächste Prüfung Januar 2027 (maximale Gültigkeit 23.07.2027). Zusammen mit Betreiber-Impressum und Leistungsseiten stützen die Ausstellerdokumente die Seed-Zuordnung, Anschrift und Aktivität; die PDFs sind Betreiber-Uploads, keine eigenständige Registerabfrage beim Aussteller. Die aktuelle Materialankaufseite lädt zum direkten Besuch am Standort ein und nennt die Annahme kleiner und großer Mengen; `dropoff_json.allowed` ist entsprechend gesetzt. Das ist keine Koordinatenbestätigung. ANKAUF: Tagespreise auf Anfrage, keine EUR-Kurse. VERKAUF: keine Preise; keine gebrauchten Kfz-Teile. GEBÜHREN: Container/Abholung ohne numerischen Tarif. Frontmatter-Kontaktfelder bleiben mangels unabhängiger Bestätigung leer. Quellen: https://www.schrotthandel-ayan.de/impressum ; https://www.schrotthandel-ayan.de/materialankauf ; https://www.schrotthandel-ayan.de/containerdienst ; https://www.schrotthandel-ayan.de/an-und-verkauf-von-schrott ; https://www.schrotthandel-ayan.de/autoverwertung ; https://www.schrotthandel-ayan.de/uploads/2ULsB1WR/Entsorgungsfachbetrieb.pdf ; https://www.schrotthandel-ayan.de/uploads/PPUnQXUS/anerkannter-Demontagebetrieb.pdf .]

### Recherche 13.10.2026

- [Recherche 13.10.2026: Betreiberwebsite erstmals umfassend für den tatsächlich im Seed genannten Börrstadter Händler gelesen. Impressum/Kontakt nennen Resul Ayan, Kaiserstraße 5, 67725 Börrstadt, 06357 498, 0176 68 222 826, schrotthandel-ayan@t-online.de; Kontaktseite nennt Mo–Fr 08:00–16:30, Sa 08:00–12:00. Leistungsseiten belegen Schrott-/Altmetallankauf zu „Tagespreisen“ ohne Zahlen, Materialgruppen u. a. Kupfer, Kabel, Messing, Zink, Aluminium, Mischschrott, Eisen, V2A, Batterien und E-Motoren, Containerdienst/Demontage sowie Autoverwertung. Startseite erklärt, dass keine gebrauchten Autoteile verkauft werden. Containerpreise/Abholgebühren nicht beziffert. Da Impressum keine Handelsregisterkennung hat und keine unabhängige zweite Quelle die Identität des Seed-Eintrags bestätigt, Frontmatter unverändert belassen; die derzeit fehlerhafte Straßenangabe nicht als valide Adresse behandeln. Kaiserstraße 5 ist GEO-Kandidat, kein bestätigter öffentlicher Annahmepunkt. Quellen: https://www.schrotthandel-ayan.de/ ; https://www.schrotthandel-ayan.de/impressum ; https://www.schrotthandel-ayan.de/kontakt ; https://www.schrotthandel-ayan.de/materialankauf ; https://www.schrotthandel-ayan.de/schrotthandel ; https://www.schrotthandel-ayan.de/an-und-verkauf-von-schrott ; https://www.schrotthandel-ayan.de/containerdienst ; https://www.schrotthandel-ayan.de/autoverwertung]

### Importiert (Seed-Stand 2026-09-30)

- Kaiserstr. 5, 82★
- urspr. Website-Angabe: keine
- Adresse: Börrstadt

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **49.5837123/7.9416515** (Mittel der eindeutigen
  Knoten von way/329824713, innerhalb der Objekt-BBox). Das Objekt trägt `addr:housenumber`=**5**,
  `addr:street`=**Kaiserstraße**, `addr:city`=Börrstadt, `addr:postcode`=67725, `addr:country`=DE und
  `building`=yes - die Dossier-Adresse (Kaiserstraße 5, 67725 Börrstadt) ist damit **adresslich vollständig
  belegt**; Objekt-BBox 49.5836552/7.9415512 bis 49.5837694/7.9417517, der Punkt liegt **innerhalb**; der Way hat
  4 eindeutige Knoten. Quelle 1 (Betreiber): Das **Impressum der Betreiber-Website** (live abgerufen 03.11.2026,
  HTTP 200, https://www.schrotthandel-ayan.de/impressum) nennt „**Resul Ayan, Autorecycling, Schrotthandel und
  Containerdienst, Kaiserstraße 5, 67725 Börrstadt**“, Telefon 06357 498, Mobil 0176 68 222 826,
  E-Mail schrotthandel-ayan@t-online.de, Inhaber Resul Ayan; die Seitenfußzeile wiederholt „R. Ayan Autorecycling |
  Schrotthandel | Containerdienst, **Kaiserstraße 5 | 67725 Börrstadt**“ - Straße, Hausnummer, PLZ und Ort sind
  damit über eine **Betreiber-Primärquelle** belegt. Das zweite Zertifikat des Dossiers (**anerkannter
  Demontagebetrieb**, https://www.schrotthandel-ayan.de/uploads/PPUnQXUS/anerkannter-Demontagebetrieb.pdf) nennt
  nach der Recherche vom 26.10.2026 denselben Rechtsträger und dieselbe Anschrift „**R. Ayan Schrotthandel,
  Autorecycling, Containerdienst, Kaiserstraße 5, 67725 Börrstadt**“ (Prüfung 24.01.2026, ausgestellt 09.03.2026) -
  es stützt die Anschrift, bleibt aber ein Betreiber-Upload und **keine** eigenständige Registerabfrage.
  Quelle 2 (OSM): way/329824713 (live abgerufen 03.11.2026,
  https://api.openstreetmap.org/api/0.6/way/329824713/full.json) sowie das Kachelraster über
  **bbox=7.9300,49.5700,7.9600,49.6000** (live abgerufen 03.11.2026) - auf der Kaiserstraße in Börrstadt ist die
  **Hausnummer 5 genau einmal** belegt und damit **eindeutig** (gefundene Nummern u. a. 1, 1a, 2, 2a, 4, 4a, 5, 6,
  6a, 7, 8, 9, 11, 11a, 12, 13, 13a, 14, 15, 19, 20, 20a, 22, 24, 26, 34, 36). **Qualifizierung:** das Objekt trägt
  **weder `name=` noch `operator=`** und keinen Betriebsbezug - der Betrieb ist in OSM also **nicht namentlich
  kartiert**; gesetzt wird deshalb der **adressgenau belegte** Objektpunkt, nicht ein Betriebspolygon. Der
  previously gespeicherte Produktionspunkt 49.5777/7.9425 (Gartenweg 1) ist mit dieser Koordinate **ersetzt**;
  `clear_fields: 'geo'` wurde deshalb aus dem Frontmatter **entfernt** (ein belegtes lat/lon-Paar und das
  gleichzeitig gesetzte `geo`-Clear sind laut Seed-Vertrag unzulässig). **Was offen bleibt:** die seit 26.10.2026
  dokumentierte **Identitäts-Zuordnung** der Seed-Zeile `Ayan Börrstadt` zu „R. Ayan Autorecycling/Schrotthandel“
  bleibt **unverändert** offen - die Zertifikate wurden weiterhin **nicht** bei der ausstellenden Stelle
  verifiziert; `name`, `trader_type`, `street`, `postcode`, `website`, `status` und `dropoff_json` bleiben daher
  **unverändert bzw. ungefüllt**, `status` bleibt **pruefung**. Die gesetzte Koordinate belegt den
  **Adressstandort**, nicht die Aktivität, die Kundenannahme oder die Preise. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/329824713/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.9300,49.5700,7.9600,49.6000 (Kachelraster, live abgerufen
  03.11.2026) ; https://www.schrotthandel-ayan.de/impressum (live abgerufen 03.11.2026).]
