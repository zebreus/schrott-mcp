---
slug: he-hanau-kulzer-scheidgut-service
name: Kulzer GmbH (Scheidgut-Service)
trader_type: sonstige
state: HE
city: Hanau
street: Leipziger Straße 2
postcode: '63450'
phone: 0800 4372522
email: ''
opening_hours: ''
website: https://www.kulzer.de/
website_status: aktiv
status: aktiv
description: 'Scheidgut-Service der Kulzer GmbH (Dentalhersteller, HRB 91228 AG Hanau) in Leipziger Straße 2, 63450 Hanau. Ankauf von Altgold, Zahngold, Gekrätz, Feilung und Tafelsilber gegen tagesaktuelle Vergütung; Einsendung per Post oder Abholung durch den Außendienstberater, Auszahlung per Überweisung oder Verrechnungsscheck innerhalb von 7 Werktagen. Der Betreiber veröffentlicht über den Altgoldrechner eine maschinenlesbare, mit Datum versehene Ankauf-Preistabelle je Gramm in Euro.'
dropoff_json: '{"allowed": true, "conditions": "Einsendung von Scheidgut per Post (adressierter Briefumschlag, Plastiktuetechen, Datenblatt) oder Uebergabe an den Aussendienstberater; private Anlieferung sowie Zahnarztpraxen und Labore; Auszahlung der Verguetung per Ueberweisung oder Verrechnungsscheck innerhalb von 7 Werktagen (kulzer.de, Abruf 29.10.2026)."}'
pickup_json: '{"allowed": true, "conditions": "Abholung vor Ort durch den Aussendienstberater, der das Scheidgut beim Kunden wiegt und zur Analyse mitnimmt; Alternativ Einsendung per Post; gebuehrenfreie Hotline 0800 4372 522 (kulzer.de, Abruf 29.10.2026)."}'
certifications: '["Scheidgut-Aufbereitung: Altgold, Gekraetz und Feilung werden im eigenen Haus analysiert und aufbereitet (Labor-Analyse, Scheidetechnik); der Altgoldrechner weist den Ankaufspreis ausdruecklich als unverbindlichen Richtwert aus (kulzer.de, Abruf 29.10.2026). Eine Eintragung als Entsorgungsfachbetrieb nach § 56 KrWG war zum Abrufzeitpunkt nicht belegbar: das amtliche Fachbetrieberegister (Spiegel schrottregister.pages.dev, Auszug 11.08.2026) fuehrt in Hanau 12 Betriebe, darunter keinen der Kulzer GmbH."]'
provenance_section: Nachtrag Audit-Runde 3b (Spezialisten) (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Kulzer GmbH (Scheidgut-Service)

## Überblick

Scheidgut-Service der **Kulzer GmbH**, Leipziger Straße 2, 63450 Hanau (AG Hanau, HRB 91228,
USt-IdNr. DE 812593096). Der Dentalhersteller betreibt neben dem Verkauf von Zahntechnik-Produkten
einen **Ankauf von Altgold, Zahngold, Gekrätz, Feilung und Tafelsilber**; die Vergütung erfolgt zum
tagesaktuellen Ankaufspreis, die Auszahlung per Überweisung oder Verrechnungsscheck innerhalb von
7 Werktagen. Private Anlieferer, Zahnarztpraxen und Labore werden über eine **gebührenfreie Hotline
(0800 4372 522)** und eine **kostenlose Scheidgut-Sammeldose** angesprochen.

**Neuer Fund vom 29.10.2026 - öffentliche, datierte Ankauf-Preistabelle.** Der Altgoldrechner der
Betreiberseite lädt seine Preise aus einem **öffentlichen JSON-Endpunkt ohne Anmeldung**:
`https://cloud-service.kulzer.com/api/OldGoldCalculator/GetData?Language=de` (HTTP 200, JSON). Der
Endpunkt liefert `priceDate` sowie eine Tabelle mit vier Mengenstaffeln (`lte10`, `lte20`, `lte50`,
`gt50` Gramm) je Legierung. Der Rechner-JS (`script-min.js`) bestätigt die Semantik: Die Eingabe ist
das **Gewicht in Gramm**, die Werte sind **Euro je Gramm**, und die Staffelung richtet sich nach der
Eingabemenge. Abrufstand 29.10.2026, `priceDate` **29.10.2026**:

| Legierung | bis 10 g | bis 20 g | bis 50 g | über 50 g |
|---|---|---|---|---|
| Gold 333 (8 Karat) | 30,80 EUR/g | 31,25 EUR/g | 31,80 EUR/g | 32,30 EUR/g |
| Gold 585 (14 Karat) | 55,25 EUR/g | 55,70 EUR/g | 56,25 EUR/g | 56,75 EUR/g |
| Gold 750 (18 Karat) | 71,25 EUR/g | 71,70 EUR/g | 72,25 EUR/g | 72,75 EUR/g |
| Feinsilber 999,9 | 1,19 EUR/g | 1,19 EUR/g | 1,19 EUR/g | 1,19 EUR/g |
| Silber 800 (Besteck) | 0,89 EUR/g | 0,89 EUR/g | 0,89 EUR/g | 0,89 EUR/g |
| Silber 925 (Schmuck) | 1,03 EUR/g | 1,03 EUR/g | 1,03 EUR/g | 1,03 EUR/g |
| Zahngold II | 42,44 EUR/g | 42,89 EUR/g | 43,44 EUR/g | 43,94 EUR/g |
| Zahngold III | 48,03 EUR/g | 48,48 EUR/g | 49,03 EUR/g | 49,53 EUR/g |

Zusätzlich nennt derselbe Endpunkt `goldBasePrice` 114,12 und `silverBasePrice` 1485,34 (Rohstoff-
Referenz des Betreibers). Der begleitende Filter-Endpunkt
`https://cloud-service.kulzer.com/api/OldGoldCalculator/GetFilter?Language=de` liefert die Legierungs-
bezeichnungen und ihre Karatangaben (Label) sowie die Kategorien Schmuckgold, Silberlegierungen und
Zahngold.

**Wichtige Abgrenzung (Ankauf, Verkauf, Gebühren):**
- **Ankauf:** Die Tabelle oben ist ein **Ankaufspreis des Betreibers für Altgold/Zahngold/Silber**, kein
  Sortenpreis für Schrott und kein Verkaufskurs. Der Betreiber bezeichnet den Rechnerwert selbst als
  **unverbindlichen Richtwert**: der genaue Edelmetallgehalt wird erst im Labor analysiert, „der vom
  Altgoldrechner ermittelte ‚Aktuelle Ankaufspreis' ist demnach unverbindlich, es besteht kein Anspruch
  auf eine exakte Vergütung dieses Betrages".
- **Verkauf:** Kulzer verkauft Zahntechnikprodukte (Online-Shop mit Preislisten). Das ist **nicht** Teil
  des Scheidgut-Geschäfts und wurde hier nicht als Schrotthandels-Leistung übernommen.
- **Gebühren:** Die Hotline ist ausdrücklich **gebührenfrei**, die Scheidgut-Sammeldose und die
  Scheidgutbroschüre sind kostenlos. Die Scheidgut-Aktion nennt **Aktionspreise für Reinigung, Schmelze
  und Analyse** des eingesendeten Materials als **Aufbereitungs-Abzüge** von der Vergütung; konkrete
  Beträge werden dafür nicht veröffentlicht. Bei Rücksendung lose gewordener Steine können laut
  Altgold-Seite **Rückversandkosten** entstehen. **Abzüge** entstehen außerdem für Anhaftungen an
  Kronen und Brücken (Zähne, Wurzel, Keramik, Kunststoff, Stahlklammern) sowie für Steine, Uhrwerke und
  Hohlräume im Schmuck.
- Der vom Betreiber gewürdigte Besuch beim Zahnarzt mit anschließender Donation an eine gemeinnützige
  Einrichtung ist eine **Spendenempfehlung**, kein Ankaufskurs.

**Einordnung der Belegkette:** Die Standort- und Sachverhaltsangaben stammen aus der Betreiber-Primärquelle
(`kulzer.de`); das Impressum nennt Kulzer GmbH mit Registernummer, das Dossier ist HR-kongruent
(Northdata). Der **Altgold-Preisendpunkt ist eine Betreiberquelle**, kein Fremdmarktplatz und kein
Drittmarkt: er gehört in die Kategorie „Ankaufspreis des Betreibers für Edelmetall", nicht in eine
Schrott-Sortenpreisliste. Ob die Filiale in Hanau die Werte des Online-Rechners 1:1 anwendet, ist nicht
ausdrücklich bestätigt und bleibt offen.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Dental-Hersteller mit Scheidgut-Ankauf: Zahngold (klein/groß), Altgold, Gekrätz, Feilung; privat + Zahnärzte/Labore; persönlich/Post, Auszahlung per Überweisung

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Impressum verifiziert (Kulzer GmbH, Leipziger Straße 2, 63450 Hanau, AG Hanau HRB 91228, USt-ID DE 812593096) + Northdata HR-kongruent (Kulzer GmbH, Hanau, AG Hanau HRB 91228) → Owner-Ausnahme greift; Scheidgut-Service bestätigt (Altgold/Gekrätz/Feilung, tagesaktuelle Vergütung, Sammeldose, gebührenfreie Hotline 0800 4372522, Techn. Hotline 06181 413 9999); /de/de/scheidgut/ (ohne .html) per Bot-Schutz 403, .html-Detailseiten abrufbar; Quelle(n): https://www.kulzer.de/de/de/impressum.html, https://www.kulzer.de/de/de/scheidgut/scheidgutaufbereitung/scheidgut.html, https://www.kulzer.de/de/de/scheidgut/scheidgut.html, https://www.kulzer.de/de/de/scheidgut/altgold-von-privatpersonen/altgold-von-privatpersonen.html, https://www.kulzer.de/]

### Korrektur 01.10.2026 (Owner-Gate)

- [Owner-Gate 01.10.2026: Shard-Deep-Link→Domain-Root normiert (Website-Regel); Scheidgut-Detail-URLs bleiben in Recherche-Note.]

### Recherche 29.10.2026

- [Recherche 29.10.2026: Kulzer Altgold-Preisquelle live geprueft - oeffentlicher JSON-Endpunkt ohne Anmeldung, HTTP 200 am 29.10.2026, `priceDate` 29.10.2026: ANKAUF in Euro je Gramm mit Mengenstaffeln (bis 10 g / bis 20 g / bis 50 g / ueber 50 g): Gold 333 30,80/31,25/31,80/32,30; Gold 585 55,25/55,70/56,25/56,75; Gold 750 71,25/71,70/72,25/72,75; Feinsilber 999,9 1,19 (alle Staffeln); Silber 800 0,89 (alle Staffeln); Silber 925 1,03 (alle Staffeln); Zahngold II 42,44/42,89/43,44/43,94; Zahngold III 48,03/48,48/49,03/49,53. Zusatzfelder `goldBasePrice` 114,12 und `silverBasePrice` 1485,34 (Rohstoff-Referenz). Der Rechner-JS in script-min.js belegt die Einheitensemantik (Eingabe Gramm, Ergebnis Euro, Staffelung nach Eingabemenge) und den Ausgabe-Preis als unverbindlichen Richtwert; Abzuege fuer Reinigung, Schmelze und Analyse werden als Aktionspreise ohne Betraege genannt. GEBUEHREN: Hotline 0800 4372 522 gebuehrenfrei, Sammeldose und Scheidgutbroschuere kostenlos, Ruecksendkosten bei Verlust loser Steine moeglich. VERKAUF: Zahntechnik-Shop separat, kein Bestandteil des Scheidgut-Geschaefts. Fachbetrieberegister (Spiegel schrottregister.pages.dev, Auszug 11.08.2026): Hanau 12 Betriebe, kein Kulzer - daher keine EFB-Behauptung. OSM-Raster ueber den gesamten Verlauf der Leipziger Strasse (4 benannte Ways, bbox 8.9230257,50.1241620,8.9460847,50.1315571): 28 Adressobjekte / 25 verschiedene Hausnummern; Hausnummer 2 existiert als zwei Objekte (node/4874455690 mit `name=Kulzer`, `office=company`, `addr:city=Hanau`, `addr:postcode=63450`, `addr:street=Leipziger Straße`, `wheelchair=yes`, `check_date=2025-05-20`; way/221106760 mit `building=office`, `building:levels=8`). Quelle(n): https://cloud-service.kulzer.com/api/OldGoldCalculator/GetData?Language=de ; https://cloud-service.kulzer.com/api/OldGoldCalculator/GetFilter?Language=de ; https://www.kulzer.de/de/de/scheidgut/altgoldrechner/altgoldrechner.html ; https://www.kulzer.de/de/de/scheidgut/altgold-von-privatpersonen/altgold-von-privatpersonen.html ; https://www.kulzer.de/de/de/scheidgut/scheidgut.html ; https://www.kulzer.de/de/de/scheidgut/scheidgutaufbereitung/scheidgut.html ; https://www.kulzer.de/media/system-files/js/script-min.js ; https://schrottregister.pages.dev/ort-hanau ; https://api.openstreetmap.org/api/0.6/map.json ; Data (c) OpenStreetMap contributors, ODbL 1.0]

- [30.10.2026 (Korrekturvermerk): **Objektzahl der Leipziger Straße berichtigt**: die Nachmessung am 30.10.2026 mit demselben Raster (bbox 8.9230257,50.1241620,8.9460847,50.1315571) ergibt **28** Adressobjekte mit `addr:street=Leipziger Straße` (bisher 31); die 25 verschiedenen Hausnummern sind unveraendert. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=<Raster laut Dossier> (Nachmessung 30.10.2026)]

- [30.10.2026 (Korrekturvermerk):  **OSM-Tag-Zitat berichtigt**: der zitierte Adressobjekt-Filter war `addr:housenumber=<Straßenname>`; das ist kein OSM-Tag, sondern ein Fehlzitat — gefiltert wurde nach `addr:street=<Straßenname>` (mit `addr:housenumber` als Hausnummer). Die Objekt- und Hausnummernzahlen selbst sind live bestätigt.]
