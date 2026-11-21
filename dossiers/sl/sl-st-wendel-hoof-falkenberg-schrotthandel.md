---
slug: sl-st-wendel-hoof-falkenberg-schrotthandel
lat: 49.4891045
lon: 7.2437521
clear_fields: ''
name: Falkenberg Schrotthandel
trader_type: schrotthaendler
state: SL
city: St. Wendel-Hoof
street: Nebenhügel 2
postcode: 66606
phone: 06856 892882
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: Schrotthandel (Altmaterialhandel) in St. Wendel-Hoof
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (Stand 2026-09-27)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Falkenberg Schrotthandel

## Überblick

Falkenberg Schrotthandel ist nur über Verzeichnisse greifbar: Nebenhügel 2, 66606 St. Wendel-Hoof und 06856 892882 stehen dort übereinstimmend; Das Örtliche listet daneben einen Eintrag ohne Namen mit derselben Anschrift/Fax. Es fehlen Betreiber-, Register- oder kommunaler Gewerbebeleg und eine belastbare aktuelle Annahmebestätigung. Deshalb Status `pruefung` statt `aktiv`; keine Schließung behauptet. **Ankauf:** „Schrott“ ist nur Branchenklassifikation, keine bestätigte Vergütung/Preisliste. **Verkauf/Gebühren:** keine Listen gefunden. **GEO:** Adresse noch Lead; keine Koordinaten freigegeben.

## Timeline

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 49.4663, 7.1681. Nominatim-Reverse löst den Punkt an Parkstraße 5 in St. Wendel auf, nicht am Lead Nebenhügel 2 im Ortsteil Hoof. `clear_fields: 'geo'` entzieht den stadtteilfalschen Altpunkt beim nächsten regulären Seed. Keine Ersatzkoordinate, bis Betreiber und tatsächlicher Standort unabhängig bestätigt sind. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.4663&lon=7.1681&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 20.10.2026 — Statuskorrektur

- [Recherche 20.10.2026: Das-Örtliche-Recheck zeigt weiterhin zwei Einträge an Nebenhügel 2, 66606 St. Wendel-Hoof; nur „Falkenberg Schrotthandel“ führt 06856 892882, der zweite Eintrag nennt Fax 06856 892255. Die Verzeichnisübereinstimmung bestätigt keinen Betreiber und ist nach README weiterhin Lead. Daher `status: aktiv` auf `pruefung` korrigiert, nicht geschlossen; Kontaktdaten/Adresse und GEO unverändert. Keine Ankauf-, Verkaufs- oder Gebührenliste gefunden. Quelle: https://www.dasoertliche.de/Themen/Schrotthaendler/St-Wendel.html (Abruf 20.10.2026); Gegenprüfung/älterer Lead: 11880-Eintrag, Recherche 02.10.2026.]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Adresse Nebenhügel 2, 66606 St. Wendel (Hoof) + Tel. (06856) 892882 in zwei Verzeichnissen konsistent (11880 + Das Örtliche, dort zusätzlich Fax 06856 892255) — als begründete Ausnahme übernommen (beide Aggregatoren, offen dokumentiert). Kein Betreiber-Webauftritt, keine E-Mail/Website belegbar. Koordinaten-Neugeocodierung läuft automatisch. Quelle(n): 11880-Eintrag Nebenhügel 2 + https://www.dasoertliche.de/Themen/Schrotthändler/St-Wendel.html]

### Importiert (Seed-Stand 2026-09-30)

- Schrott

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **49.4891045/7.2437521** (Mittel der 8 eindeutigen Knoten von
  way/227934248; der Punkt liegt innerhalb der Objekt-BBox
  49.4890397/7.2436126–49.4891750/7.2439582). Quelle 1 (OSM, live geladen am 04.11.2026):
  **way/227934248** trägt `name`=**Falkenberg Schrotthandel**, `addr:street`=Nebenhügel,
  `addr:housenumber`=**2**, `addr:postcode`=66606, `addr:city`=Sankt Wendel,
  `addr:suburb`=**Hoof**, `building`=yes - Straße, Hausnummer, PLZ, Ort **und der Geschäftsname**
  stimmen mit der Dossier-Adresse **Nebenhügel 2, 66606 St. Wendel-Hoof** überein; das Objekt trägt
  den **Betriebsbezug** (`name`). Quelle 2 (im Dossier bereits zitiert, live nachgelesen am
  04.11.2026): Das Telefonbuch
  https://www.dasoertliche.de/Themen/Schrotthaendler/St-Wendel.html liefert HTTP 200 und führt den
  Eintrag **„Falkenberg Schrotthandel, Nebenhügel 2, 66606 St. Wendel, Hoof, Telefon 06856 89 28
  82“** (zweiter Eintrag „Falkenberg“ an derselben Anschrift mit Fax 06856 89 22 55) mit der
  Kartenposition **49.4892290/7.2437017** - die **rund 4 m** vom gesetzten OSM-Punkt entfernt liegt
  und die Anschrift damit unabhängig bestätigt. **Fensterprüfung (kein Fensterfehler):** das am
  04.11.2026 live abgerufene Kachelraster umfasst **7.2400/49.4800 bis 7.2500/49.5000**, **2 Kacheln
  von je 0,01 Grad**, **0 Kachel-Fehler**, **9.266 Elemente**; das Fenster enthält die gesuchte
  Straße **„Nebenhügel“ nachweislich** mit den drei Straßenobjekten **way/222311470** und
  **way/227917947** (residential) sowie **way/227917948** (service) sowie **21 Adressobjekten** auf
  „Nebenhügel“ mit **21 verschiedenen Hausnummern** (1–7, 9–12, 14, 16–20, 24, 28 sowie 1a, 7a) - die
  **Dossier-Hausnummer 2 existiert** und ist im Raster das einzige Objekt mit dieser Nummer auf dieser
  Straße. **Negativaussage, qualifiziert:** im Raster tragen **87 Objekte** `name=` oder `operator=`;
  **mit Bezug zu „Falkenberg“ oder „Schrott“** trifft es auf **genau 1 Objekt**
  (way/227934248, `name`=Falkenberg Schrotthandel) - es gibt **kein** zweites Objekt dieser Adresse
  und **keinen** konkurrierenden Schrottbetrieb auf der Straße. **Zurücknahme des `geo`-Tokens
  (Begründung):** `clear_fields: 'geo'` wurde **entfernt** (das Feld steht jetzt als
  `clear_fields: ''`), weil es den am 25.10.2026 zurückgenommenen und stadtteilfalschen
  Produktionspunkt (49.4663/7.1681 = Parkstraße 5 in St. Wendel) fernhalten sollte; der hier gesetzte
  Punkt liegt **exakt** auf der verzeichnisbestätigten Betriebsadresse im Ortsteil **Hoof**, und
  `clear_fields` darf nicht gleichzeitig `geo` neben belegtem `lat`/`lon` stehen. **Qualifizierung
  (unverändert übernommen):** Der Punkt bezeichnet die **betrieblich benannte Anschrift**, aber
  **nicht** einen verifizierten Annahmehof, eine Waage oder eine öffentliche Schrottannahme; das
  Branchenlabel „Schrotthandel“ ist **keine** bestätigte Vergütung, es wird **kein** Ankaufkurs,
  **keine** Materialverkaufspreisliste und **keine** Gebühr behauptet. `status` bleibt `pruefung`,
  `street`, `postcode`, `phone`, `email`, `opening_hours`, `website`, `website_status` und
  `description` bleiben unverändert. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/227934248/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.2400,49.4800,7.2500,49.5000 ;
  https://www.dasoertliche.de/Themen/Schrotthaendler/St-Wendel.html (Abruf 04.11.2026).]

### Recherche 21.11.2026 (Welle 1800 C)

- [Recherche 21.11.2026: **Kein Register- und kein Insolvenztreffer unter diesem Namen; der OSM-Betriebsname bleibt der einzige Beleg fuer die Anschrift.** Die Namenssuche im amtlichen Handelsregisterportal (online-handelsregister.de, Abruf 21.11.2026, Suchbegriff `Falkenberg Schrott`) liefert `Keine Suchergebnisse!`; im amtlichen Insolvenzbekanntmachungen-Portal (Abruf 21.11.2026, Suche `Falkenberg`) erscheinen nur Privatpersonen (`Falkenberg, Waltraud`, Limburg a.d. Lahn; `Falkenberg, Samira`, Fürth), keine Gesellschaft. Die Namenssuche im amtlichen Fachbetrieberegister (live am 21.11.2026) liefert fuer `Falkenberg` in der Route `Entsorgungsfachbetriebe` **0 Treffer** und in der Route `Altfahrzeugverwertung` **1 Treffer** (`Autoverwertung Falkenberg Sandro Domke`, `Eberswalder Straße / Gewerbegebiet / Haus Nr. 7`, `16259 Falkenberg/Mark`, `ZAPS00100030 / 1`, ein Namensvetter in Brandenburg); die Standortabfrage mit 3 km um `Nebenhügel 2, 66606 St. Wendel` liefert **0 Efb- und 0 AfV-Zeilen**, die Abfrage mit 10 km **8 Efb-Zeilen** (u.a. `Südwestdeutsche Hartsteinwerke ... Steinbruch Oberlinxweiler - Recyclinganlage`, `Spiemontstraße 100`, 66606 St. Wendel/Oberlinxweiler, 1,5 km, `ZZLT00500064 / 8` und `/ 9`). **Keine Zeile nennt Falkenberg.** Reichweitenklausel: die Efb- und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein Nichtexistenz- und kein Schliessungsbeweis. Quelle(n): https://www.online-handelsregister.de/suche/Falkenberg%20Schrott ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Nebenh%C3%BCgel%202%2C%2066606%20St.%20Wendel%2C%20Deutschland%3B7.2387851%3B49.4891015%3B7.2487851%3B49.4962588%3B7.2287851%3B49.4819442&dst=10&is=1&e=1 ; https://neu.insolvenzbekanntmachungen.de/ap/suche.jsf (Abruf 21.11.2026).]
- [Recherche 21.11.2026: **GEO gemessen: der gespeicherte Punkt trifft das einzige Objekt, das den Betriebsnamen traegt, mit 2,4 m Abstand.** Der Punkt 49.4891045/7.2437521 loest rueckaufloesend auf `way/227934248` auf, das Objekt mit `name`=`Falkenberg Schrotthandel`, `addr:housenumber`=2, `addr:street`=Nebenhügel, `addr:postcode`=66606, `addr:city`=Sankt Wendel, `addr:suburb`=Hoof, `building`=yes; Abstand zum Objektmittelpunkt **2,4 m**, also keine Abweichung, das Frontmatter-Paar bleibt unveraendert. Nach der Geokodierungsregel dieser Repo-Welle wird ein gespeicherter Punkt nicht entfernt und nicht ersetzt; die Entscheidung ueber Setzen oder Zuruecknehmen trifft der Owner in einer eigenen datierten Korrektur-Sektion. Der Punkt bezeichnet die **betrieblich benannte Anschrift**, nicht einen verifizierten Annahmehof oder eine Waage; der amtliche Efb-Nachweis fehlt weiterhin. Quelle(n): https://api.openstreetmap.org/api/0.6/way/227934248/full.json (Abruf 21.11.2026).]
- [Recherche 21.11.2026: **Verzeichnisstand bestaetigt, ein neuer Eintrag datiert, keine Preise.** Das Oertliche fuehrt am 21.11.2026 weiterhin **zwei** Eintraege an `Nebenhügel 2, 66606 St. Wendel, Hoof`: `Falkenberg` (`Schrott`, `Fax 06856 89 22 55`) und `Falkenberg Schrotthandel` (`Tel. 06856 89 28 82`); die Gelbe-Seiten-, Bundes-Telefonbuch-, meinestadt- und Stadtbranchenbuch-Eintraege nennen dieselbe Nummer und Adresse, der Bundes-Telefonbuch-Eintrag ist mit `Eingetragen seit: 12.03.2026` datiert, und der Weblocator-/Kundenportal-Eintrag verweist als `Website` auf `https://www.gebr-ritscher.de/` (siehe Nachbar-Dossier `hh-altstadt-walter-ritscher`), also auf einen **Kachelofenbetrieb im Oberlausitz** und nicht auf einen Schrottbetrieb in St. Wendel. Saemtliche Eintraege bleiben Verzeichnisleads ohne Betreiber- oder Registerbeleg. **ANKAUF:** nicht belegt, `Schrott` ist eine Branchenklassifikation ohne bestaetigte Verguetung. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife. `status` bleibt `pruefung`, `street`, `postcode`, `phone` und das Koordinatenpaar bleiben unveraendert. Quelle(n): https://www.dasoertliche.de/Themen/Schrotthaendler/St-Wendel.html ; https://www.bundes-telefonbuch.de/st-wendel/firma/falkenberg-schrotthandel-bvg080604085 ; https://www.gelbeseiten.de/gsbiz/ee60e099-977e-40da-863f-fc67305f31d7 (Abruf 21.11.2026).]
