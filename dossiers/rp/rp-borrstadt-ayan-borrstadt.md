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

### Recherche 23.11.2026 (Welle 0300 C)

- [Recherche 23.11.2026 (Welle 0300 C): **Der Betrieb steht jetzt amtlich im Fachbetrieberegister — der Abzug vom 11.08.2026 führt „R. Ayan Autorecycling", Kaiserstraße 5, 67725 Börrstadt, mit der Anerkennung nach AltfahrzeugV „Demontagebetrieb" und der Zertifikatsnummer `ZAGS00300050`.** Damit ist die im Dossier seit dem 26.10.2026 offene Frage der Identitätszuordnung der Seed-Zeile `Ayan Börrstadt` zu „R. Ayan Autorecycling/Schrotthandel" **amtlich** beantwortet: das amtliche Register führt denselben Rechtsträger an **derselben Anschrift** wie das Betreiber-Impressum. Die Zertifikatsnummer `ZAGS00300050` ist die **Vorgangsnummer aus Feld 3.3** des Zertifikats; sie wird hier getrennt von der unten genannten **Zertifikatsnummer des Ausstellers** (`SMU/1195/001`) und von der **Register-Nummer** geführt. **Zum Zertifikat selbst:** Das auf der Betreiberwebsite gehostete Überwachungszertifikat „Anerkannter Demontagebetrieb" (live abgerufen 23.11.2026, HTTP 200, PDF-Datei `anerkannter-Demontagebetrieb.pdf`) wurde erneut am Bild geprüft und nennt: **Betriebsnummer G 08157402**, Überprüfungsbericht **SMU/2023/022**, Anschrift **R. Ayan Autorecycling, Kaiserstraße 5, 67725 Börrstadt**, Ansprechpartner **Herr Resul Ayan**, Tag der Überprüfung **24. Januar 2026**, nächste Überprüfung **Januar 2027**, maximale Gültigkeit des Zertifikates **23. Juli 2027**, ausgestellt durch **SMU GmbH**, Hauptstraße 58, 67297 Mannheim, Überwachungsbehörde **SGD Süd Regionalstelle Wasserwirtschaft, Abfallwirtschaft, Bodenschutz, Fischerstraße 12 in 76765 Kaiserslautern**, gezeichnet Dr. Michael Kappus. Das EfbV-Zertifikat (`Entsorgungsfachbetrieb.pdf`, 3 Seiten, ebenfalls live abgerufen 23.11.2026, HTTP 200) wurde ebenfalls am Bild geprüft: **Feld 3.1** Zertifikatsnummer der Zertifizierungsorganisation `SMU/1195/001` (ausdrücklich „frei zu vergeben" durch SMU), **Feld 3.3** Vorgangsnummer `ZZGT002001870001`, **Feld 3.7** gültig bis **23.08.2027**, **Feld 4.1** „R. Ayan Schrotthandel, Autorecycling, Containerdienst", **Feld 4.2** Kaiserstraße 5, **Feld 4.3** 67725 Börrstadt, **Feld 4.4** Registernummer: **„Nicht zutreffend"** (es besteht also **kein** Handelsregistereintrag als GmbH/AG, der Betrieb ist ein Einzelunternehmen) — die Zertifikate selbst belegen das **nicht** und sagen nichts über die Identität des Seed-Datensatzes, das leistet der Registereintrag. **Anlage 1** (Seite 2) ist der einzige Anlagenbogen: **2.1 Sammeln** ☒ (nur deutschlandweit ☒), **2.2 Befördern** ☒ (nur deutschlandweit ☒), **2.3 Lagern** ☐, **2.4 Behandeln** ☐, **2.5 Verwerten** ☐, **2.6 Beseitigen** ☐, **2.7 Handeln** ☐, **2.8 Makeln** ☐; Kennnummer nach § 28 NachwV `G 081574023` bei 2.1 und 2.2; Feld 3.3 verweist auf die technische Anlage „Schrotthandel/Containerdienst". **Anlage 1, Feld 4** (Seite 3): **4.2 alle nicht gefährlichen Abfälle** ☒, **4.4 bestimmte Abfallarten** ☒ mit den Abfallschlüsseln 090111*, 160104*, 160209*, 160210*, 160211*, 160212*, 160213*, 160215*, 160601*, 160602*, 160603*, 160606*, 200133*, 200135* — die EfbV-Anerkennung deckt damit **Sammeln und Befördern**, **nicht** Handel, Verwerten oder Beseitigen; ein Handel mit diesen Abfällen ist damit **nicht** zertifiziert. Die PDF-Dateinamen sind keine Vorgangsnummern; maßgeblich sind die Feld-3.3-Werte. **ANKAUF:** Die Betreiberseite nennt Materialankauf zu „fairen Tagespreisen" und aktueller Marktpreisbewertung, eine konkrete Sortenliste (Kupfer, Kupferkabel, Messing, Zink, Aluminium, Mischschrott, Moniereisen, Bremsscheiben, Elektromotoren, V2A-Abfälle, Batterien/Autobatterien) und die Zusage „schnelle Auszahlung", aber **keine EUR-Beträge und kein Publikationsdatum**; die amtliche Zertifizierung belegt **keine** Ankaufspreise. **VERKAUF:** Die Seite „An- und Verkauf von Schrott" beschreibt ausdrücklich den **Kauf und Verkauf** von Schrottmaterialien („Schrott schnell und fair zu Geld machen"), nennt aber **keine** Materialpreisliste und **keine** Verkaufspreise; gebrauchte Fahrzeugteile werden nach der Startseite ausdrücklich **nicht** verkauft. **GEBÜHREN:** Containerdienst und Abholung werden beworben („faire Preise", „termingerechte Bereitstellung", „schnelle Abholung"), **ohne jeden bezifferten Tarif**; eine kostenfreie Abholung wird **nicht** behauptet. **Was offen bleibt:** `status` bleibt `pruefung` und `name`/`trader_type`/`street`/`postcode`/`website`/`dropoff_json` bleiben unverändert bzw. ungefüllt, weil die Zuordnung der **Seed-Zeile** (provenance_origin `table`, Name „Ayan Börrstadt", Typ `sonstige`, keine Adresse) zu dem Registerbetrieb erst eine Ownerentscheidung ist; der Registereintrag belegt **nicht**, ob die Seed-Zeile genau diesen Betrieb meint und ob der Standort eine **öffentlich zugängliche Kundenannahme** ist. Quelle(n): https://schrottregister.pages.dev/betrieb-r-ayan-autorecycling-borrstadt ; https://schrottregister.pages.dev/assets/institute.json (vollständiger Registerdatensatz, Stand 11.08.2026, Abruf 23.11.2026) ; https://www.schrotthandel-ayan.de/uploads/PPUnQXUS/anerkannter-Demontagebetrieb.pdf ; https://www.schrotthandel-ayan.de/uploads/2ULsB1WR/Entsorgungsfachbetrieb.pdf ; https://www.schrotthandel-ayan.de/impressum ; https://www.schrotthandel-ayan.de/materialankauf ; https://www.schrotthandel-ayan.de/an-und-verkauf-von-schrott ; https://www.schrotthandel-ayan.de/containerdienst ; https://www.schrotthandel-ayan.de/ (live abgerufen 23.11.2026).]
- [Recherche 23.11.2026 (Welle 0300 C): **GEO-Abgleich des gespeicherten Punktes gegen die OSM-Objekte, ohne den Punkt zu ändern.** Read-only `public.db` (23.11.2026) enthält für diese Zeile **49.5837123 / 7.9416515** — identisch mit dem im Dossier vom 03.11.2026 gesetzten Wert und mit dem Objekt **way/329824713** (`addr:housenumber`=5, `addr:street`=Kaiserstraße, `addr:city`=Börrstadt, `addr:postcode`=67725, `building`=yes, `source:building`=OpenData Rheinland-Pfalz); live am 23.11.2026 über `https://api.openstreetmap.org/api/0.6/way/329824713/full.json` bestätigt, Objekt-BBox 49.5836552/7.9415512 bis 49.5837694/7.9417517, der Punkt liegt innerhalb der BBox. **Neu belegt:** Die **Kaiserstraße in Börrstadt** trägt im Kachelraster **bbox=7.9300,49.5700,7.9600,49.6000** die Hausnummern 1, 1a, 2, 2a, 4, 4a, **5**, 6, 6a, 7, 8, 9, 11, 11a, 12, 13, 13a, 14, 15, 19, 20, 20a, 22, 24, 26, 34, 36 — die **Hausnummer 5 ist genau einmal belegt** und damit eindeutig. Eine Suche über **alle** Tag-Werte nach „Ayan" im Raster liefert **0 Treffer**: das Objekt trägt **weder `name=` noch `operator=`**, der Betrieb ist in OSM also **nicht namentlich kartiert**; der gesetzte Punkt bezeichnet den **adresslich belegten Objektpunkt**, nicht ein Betriebsobjekt. **Distanz zum gespeicherten Punkt: 0 m** (identischer Punkt). **Keine Änderung des Punktes**; die Beobachtung ist nur dokumentiert, damit der Owner die Beibehaltung begründen kann. Quelle(n): read-only `public.db` (23.11.2026) ; https://api.openstreetmap.org/api/0.6/way/329824713/full.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=7.9300,49.5700,7.9600,49.6000 (Abruf 23.11.2026).]

### Korrektur 24.11.2026 (Owner, Geokodierung) — der Standortpunkt bleibt unverändert

- [Korrektur 24.11.2026: **Der in der Produktion gespeicherte Standortpunkt dieses Dossiers bleibt unverändert ✓ ✓ — Der Punkt liegt **objektgenau** ✓✓ auf dem Adressobjekt ✓✓ (Kaiserstraße **5** ✓✓, way/329824713 ✓✓) ✓ ✓ ✓ ✓ **Reichweite:** Die EfbV-/AltFahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Wellennotiz mit der Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 23.11.2026 ✓ ✓), **zweitens** ✓ ✓ das **Adressobjekt** ✓ ✓ in OpenStreetMap ✓ ✓ ✓ ✓]
