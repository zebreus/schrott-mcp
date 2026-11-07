---
slug: bw-balingen-schlaich-kurt-schrotthdl
lat: 48.2537941
lon: 8.8093351
name: Schlaich Kurt SchrottHdl.
trader_type: schrotthaendler
state: BW
city: Balingen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
clear_fields: 'dropoff_json,phone'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: ja
provenance_origin: table
---

# Schlaich Kurt SchrottHdl.

## Überblick

Kurt Schlaich in Balingen-Erzingen ist derzeit nur durch Branchenverzeichnisse greifbar. Erlenstraße 58, 72336 und 07433 35407 bleiben historische Verzeichnisangaben und wurden aus den strukturierten Feldern entfernt; kein Betreiber- oder Registerbeleg bestätigt die aktuelle Annahme. Der frühere Ausnahme-Fill bleibt in der Historie erhalten, rechtfertigt aber keine heutige Aktivität. `pruefung` statt `aktiv`, keine geprüften Zeiten oder Annahmebedingungen. **Ankauf:** keine belegten Kurse. **Verkauf:** keine Preisliste. **Gebühren:** keine Container-/Abholungstarife.

## Timeline

### Recherche 27.10.2026: amtlicher Registerabgleich Balingen

- [Recherche 27.10.2026: Amtlicher Fachbetrieberegister als neue Gegenquelle herangezogen (GESA/eEFBV-Auszug, Stand 11.08.2026): Für Balingen (72336) ist genau ein zertifizierter Betrieb geführt, „Transportbetonwerk" in der Langen Straße 65 (Lagern, Behandeln); ein anerkannter Demontagebetrieb ist nicht verzeichnet und „Kurt Schlaich" bzw. die Erlenstraße 58 sind nicht enthalten. Das Register enthält nur zertifizierte Metall- und Fahrzeugbetriebe; das Fehlen belegt weder Nichtexistenz noch Geschäftsaufgabe, es existiert aber kein EFB-/Demontage-Nachweis für den Verzeichnislead; Quelle(n): https://schrottregister.pages.dev/ort-balingen]
- [Recherche 27.10.2026: Beleglage und Clear-Regel unverändert: Die Adresse Erlenstr. 58, 72336 Balingen-Erzingen und die Nummer 07433 35407 stammen weiterhin nur aus Branchenverzeichnissen (11880, Gelbe Seiten, Das Telefonbuch) und damit aus einer Belegklasse; ein Betreiberauftritt, ein Registerbeleg oder ein kommunaler Nachweis ist nicht auffindbar, die Verzeichniszeiten Mo–Fr 08:30–17:00 sind keine Betreiberzeiten. **Ankauf:** kein bezifferter Metallkurs; **Verkauf:** keine Liste; **Gebühren:** keine Container-/Abholungstarife. `status: pruefung` und die Clear-Regel `clear_fields: 'dropoff_json,geo,phone'` bleiben unverändert; es wurde weder eine Adresse noch eine Koordinate neu gesetzt — die read-only Produktionszeile enthält weiterhin die nur über ein Verzeichnis geführte Telefonnummer, `dropoff_json={"allowed":true}` und den GEO-Punkt 48.2753/8.8546, der auf Hirschbergstraße 2 auflöst; Quelle(n): https://www.11880.com/branchenbuch/balingen/130733657B27323519/kurt-schlaich-schrotthdl.html ; https://www.gelbeseiten.de/gsbiz/41228816-a168-4580-b316-c60ae05d93b4 ; read-only `public.db` (27.10.2026)]

### Owner-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Read-only Produktion enthält weiter die nur über ein Branchenverzeichnis geführte Telefonnummer 07433 35407, `dropoff_json={"allowed":true}` und GEO 48.2753/8.8546. Der Punkt löst an Hirschbergstraße 2 auf, nicht an der nur als Verzeichnis-Lead bekannten Erlenstraße 58; weder Adresse ist als aktuelle Annahmestelle des Händlers belegt. `clear_fields: 'dropoff_json,geo,phone'` zieht Annahmeclaim, Telefonnummer und Fehlpunkt aus dem nächsten Seed zurück. Status `pruefung`, keine Schließung und kein Ersatzpunkt. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=48.2753&lon=8.8546&zoom=18&addressdetails=1 ; https://www.openstreetmap.org/way/169826526 ; https://www.11880.com/branchenbuch/balingen/130733657B27323519/kurt-schlaich-schrotthdl.html (Lead) ; README.md, Quellenhierarchie.]

### Recherche 26.10.2026

- [Recherche 26.10.2026: 11880-Profil für Kurt Schlaich direkt erneut gelesen: Erlenstraße 58, 72336 Balingen-Erzingen, 07433 35407; werktags 08:30–17:00 laut Verzeichnis. Das Profil nennt keine Betreiberwebsite oder E-Mail; sein Änderungsdatum/Verzeichnistext ersetzt keine Betreiberbestätigung. Eine ältere Nutzerbewertung ist subjektiv und keine Kurs- oder Abrechnungsquelle. Adresse, Telefon und Zeiten bleiben Leads; kein GEO-Fill. ANKAUF: kein bezifferter Metallkurs; VERKAUF: keine Liste; GEBÜHREN: keine Tarife. Quelle (Lead): https://www.11880.com/branchenbuch/balingen/130733657B27323519/kurt-schlaich-schrotthdl.html .]

### Owner-GEO-Prüfung 21.10.2026

- [Owner-GEO-Prüfung 21.10.2026: Die Read-only-Kartenprüfung ordnet den vorhandenen Produktionspunkt einem Gebäude an der Hirschbergstraße 2 zu; Erlenstr. 58 ist ein getrenntes OSM-Adressobjekt und bleibt ein historischer Verzeichnis-Lead. Kein Betreiber- oder Annahmebeleg verbindet Schlaich mit einem der Punkte. GEO bleibt ungeklärt; keine Adresse geokodiert. Quellen: [Reverse-Abfrage](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=48.2753&lon=8.8546&zoom=18&addressdetails=1), [OSM-Gebäude am Produktionspunkt](https://www.openstreetmap.org/way/169826526), [OSM-Adresssuche Erlenstraße](https://nominatim.openstreetmap.org/search?street=Erlenstr.%2058&postalcode=72336&city=Balingen&country=Deutschland&format=jsonv2&limit=3&addressdetails=1), [Owner-Bericht](../../research/wave-20261021-0900-geo-bw.md).]

### Recherche 21.10.2026

- [Recherche 21.10.2026: Erneute unabhängige Suche findet die Adresse Erlenstr. 58, 72336 Balingen-Erzingen und Telefon 07433 35407 weiterhin nur in Verzeichnissen; kein Betreiberauftritt, kein lesbarer Registerbeleg, kein kommunaler Nachweis. Die Felder street/postcode/phone wurden deshalb geleert; die Lead-Werte und bisherigen Korrekturen bleiben in der Timeline erhalten. Status pruefung, keine Schließungsbehauptung. Ankauf/Verkauf/Gebühren ohne belastbare Preislisten. GEO erst nach bestätigter Betreiberadresse. Quellen (Leads): https://www.gelbeseiten.de/gsbiz/41228816-a168-4580-b316-c60ae05d93b4 ; https://www.11880.com/branchenbuch/balingen/130733657B27323519/kurt-schlaich-schrotthdl.html .]

### Recherche 08.10.2026 (Registergegenprüfung)

- [Recherche 08.10.2026: Gegenrecherche über Namens-/Ortswebsuche und Northdata-Suggest Kurt Schlaich: Suggest HTTP200 mit results leer, Webtreffer nur Verzeichnisprofile zu Erlenstr. 58. Das ist keine vollständige Handelsregister-/Gewerbeprüfung und beweist keine Schließung, zumal Einzelunternehmen nicht zwingend im Handelsregister stehen. Vorhandene Adresse/Nummer bleiben historische Verzeichnis-Leads; aggregierte Zeiten nicht als Betreiberzeiten übernehmen. ANKAUF: aktuelle Auszahlung und numerische Preise ungeklärt. VERKAUF: keine Liste. GEBÜHREN: keine Container-/Abholungstarife. Nächster sinnvoller Belegweg Gewerbeauskunft Balingen bzw. direkt dokumentierte Betreiberbestätigung, nicht weitere kongruente Aggregatoren. Quelle: https://www.northdata.de/suggest.json?query=Kurt%20Schlaich&countries=DE ; Verzeichnis-Ausgangsquelle: https://www.11880.com/branchenbuch/balingen/130733657B27323519/kurt-schlaich-schrotthdl.html .]

### Recherche 05.10.2026

- [Recherche 05.10.2026: Evidenzkorrektur zur Runde 02.10.: drei kongruente Aggregatoren sind laut README keine zulässigen Betreiberbelege. 11880 vollständig gelesen, kongruent Erlenstr. 58, Balingen-Erzingen, 07433 35407; dort Mo–Fr 08:30–17 nur Verzeichnisangabe, Eintragsdaten 01.12.2022 und Aktualisierung 03.10.2026 belegen keine heutige Betreiberbestätigung. Alte Bewertung von 2015 keine aktuelle Preisquelle. Kein lesbarer Betreiber-/Register-/Kommunalbeleg gefunden; status aktiv → pruefung, bestehende Adress-/Telefonfelder als historische Leads erhalten, keine neuen Fills. Nicht als geschlossen bewertet.; Lead-Quelle: https://www.11880.com/branchenbuch/balingen/130733657B27323519/kurt-schlaich-schrotthdl.html]

### Recherche 02.10.2026

- [Recherche 02.10.2026: street/postcode/phone gefüllt (begründeter Ausnahmefall, offengelegt): 3 konkordante Verzeichnisse — Gelbe Seiten (Erlenstr. 58, 72336 Balingen-Erzingen, Tel 07433 35407), Das Telefonbuch (Erlenstr. 58, 72336 Balingen-Erzingen, Tel 07433 35…07 maskiert, konsistent), 11880-JSON-LD (Erlenstr. 58, 72336 Balingen, Tel (07433) 35407). KEINE Betreiber-Website, kein Register-/Kommunalbeleg gefunden. Hinweis Adress-Fill aus leerem Feld: Koordinaten neu zu geocodieren. Keine Zeiten/Leistungen verifiziert; Quelle(n): gelbeseiten.de/gsbiz/41228816… + adresse.dastelefonbuch.de/Balingen/1-Schrott-Kurt-Schlaich… + 11880-Branchenbuch Balingen (Abruf 02.10.2026)]

### Importiert (Seed-Stand 2026-09-30)

- Schrott
- Größe: klein

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **48.2537941/8.8093351** (Mittel der **4**
  eindeutigen Knoten von way/270461034; die BBox der eindeutigen Knoten reicht von 48.2537446/8.8092328 bis
  48.2538436/8.8094374 und **umschließt den Mittelwert**). Quelle 1 (OSM, live geladen am 04.11.2026, HTTP 200):
  **way/270461034** trägt `addr:street`=Erlenstraße, `addr:housenumber`=**58**, `addr:postcode`=72336,
  `addr:city`=Balingen, `addr:suburb`=Erzingen, `addr:country`=DE sowie `building`=yes; Straße, Hausnummer, PLZ und Ort
  stimmen mit der Dossier-Adresse **Erlenstr. 58, 72336 Balingen-Erzingen** überein. **Fensterprüfung (kein
  Fensterfehler):** die am 04.11.2026 live abgerufenen Kachelraster umfassen **8.8100/48.2500 bis 8.8700/48.2900** (**24
  Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**) und **8.8000/48.2500 bis 8.8200/48.2700** (**4 Kacheln**, **0
  Kachel-Fehler**) - zusammen **26 verschiedene Kacheln**; das Fenster enthält die gesuchte Straße **nachweislich und
  vollständig**, das einzige Straßenobjekt **way/144174509** „Erlenstraße" (BBox 48.25341/8.80783 bis 48.25483/8.81615)
  liegt **vollständig** darin. Im Fenster liegen **64** Adressobjekte auf „Erlenstraße" mit **32** verschiedenen
  Hausnummern (1, 5, 9, 10, 11, 12, 14, 16, 18, 19, 21, 22, 24, 26, 27, 30, 31, 33, 34, 35, 36, 37, 38, 41, 43, 44, 45,
  49, 51, 53, 55 und **58**) - **die 58 existiert** und ist das einzige Objekt mit dieser Nummer auf dieser Straße. Eine
  Suche nach **„Schlaich"** über die Felder name, operator, brand, description, note und addr:housename findet im
  Fenster **keinen** Treffer - es gibt also **keine** OSM-Betriebszuordnung, nur die Adressangabe. Quelle 2
  (Verzeichnis, im Dossier bereits zitiert, live nachgelesen am 04.11.2026): das **11880-Profil**
  https://www.11880.com/branchenbuch/balingen/130733657B27323519/kurt-schlaich-schrotthdl.html liefert **HTTP 200**
  (Seitentitel „Kurt Schlaich SchrottHdl. Balingen | Oktober 2026") und nennt live „Erlenstr. 58", „72336" und „07433
  35407"; das im Dossier dokumentierte Gelbe-Seiten-Profil nennt dieselbe Anschrift. **Beleggrenze (unverändert
  übernommen):** Quelle 2 ist ein **Verzeichnislead** (README, Quellenhierarchie) und **kein** Betreiber-, Register-
  oder Kommunalbeleg; das amtliche Fachbetrieberegister (Auszug 11.08.2026) führt für Balingen nur „Transportbetonwerk",
  Langen Straße 65, und weder „Kurt Schlaich" noch die Erlenstraße 58 - das ist wegen der freiwilligen EFB-Anerkennung
  **kein** Nichtexistenzbeweis. Der Punkt bezeichnet damit die **doppelt belegte Anschrift** (OSM-Adressobjekt plus
  Verzeichnislead), **nicht** eine verifizierte Annahmestelle oder Waage; `status: pruefung` bleibt, Ankauf-, Verkaufs-
  und Gebührenangaben bleiben unbelegt. **Zurücknahme des `geo`-Tokens (Begründung):** `clear_fields:
  'dropoff_json,geo,phone'` wurde um das Token **geo** gekürzt (die Tokens `dropoff_json` und `phone` bleiben
  **unverändert** bestehen), weil es den falschen Altpunkt 48.2753/8.8546 fernhalten sollte; dieser löst live auf
  **way/169826526** (Hirschbergstraße 2, 72336 Balingen, Mittel 48.2748422/8.8546901) auf und damit **nicht** auf die
  Dossier-Adresse - `clear_fields` darf außerdem nicht gleichzeitig `geo` neben belegtem `lat`/`lon` stehen. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/270461034/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.81,48.25,8.87,48.29 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.80,48.25,8.82,48.27 ;
  https://api.openstreetmap.org/api/0.6/way/169826526/full.json ;
  https://www.11880.com/branchenbuch/balingen/130733657B27323519/kurt-schlaich-schrotthdl.html (Abruf 04.11.2026).]

### Recherche 07.11.2026

- [Recherche 07.11.2026: **Das amtliche Fachbetrieberegister bestätigt den Negativbefund vom 27.10.2026 am Originalregister und nicht nur über den Spiegel: für „Kurt Schlaich“ existiert weder ein EfbV- noch ein AltfahrzeugV-Eintrag.** Die Namenssuche in der Route „Entsorgungsfachbetriebe“ (`…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schlaich`, Abruf 07.11.2026) liefert **0 Treffer**, die Namenssuche in der Route „Altfahrzeugverwertung“ (`…/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=Schlaich`) ebenfalls **0 Treffer**. Die Radiusabfrage mit 3 km um die Lead-Adresse Erlenstraße 58, 72336 Balingen (`…/Entsorgungsfachbetriebe/adr=72336 Balingen, Deutschland;8.8093351;48.2537941;…&dst=3&is=1&e=1`) liefert **1 Treffer**, nämlich den **fremden** NK-Verwertung Inh. Naim Krasniqi (Alte-Balinger-Straße 38, 72336 Balingen-Endingen, ZZHT00400372 / 1, Sammeln und Befördern, 2,8 km); die Radiusabfrage in der Route „Altfahrzeugverwertung“ mit demselben Radius liefert **0 Treffer**. Damit ist am Originalregister weder für die Lead-Adresse noch für den Namen ein zertifizierter Betrieb oder eine AltfahrzeugV-Anerkennung eingetragen. **Reichweite:** die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**, ein Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung besitzt, und **kein** Nachweis einer Geschäftsaufgabe. Der seit dem 02.10.2026 geführte Aggregatorlead bleibt damit unverändert: keine eigene Betreiberwebsite, kein Registerbeleg, keine kommunalen oder gewerberechtlichen Belege. `status` bleibt **pruefung**, die Contact- und Adressfelder bleiben leer, die Koordinate 48.2537941/8.8093351 (OSM-Adressobjekt way/270461034, Erlenstraße 58) bleibt als **Adresspunkt** stehen und ist **keine** belegte Annahmestelle oder Waage. **ANKAUF:** keine bezifferten Metallkurse; **VERKAUF:** keine Preisliste; **GEBÜHREN:** keine Container- oder Abholungstarife. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schlaich ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=Schlaich ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=72336%20Balingen%2C%20Deutschland%3B8.8093351%3B48.2537941%3B8.8093351%3B48.2537941%3B8.8093351%3B48.2537941&dst=3&is=1&e=1 .]
