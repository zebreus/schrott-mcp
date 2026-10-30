---
slug: bb-cottbus-autoverwertung-cottbus
name: Autoverwertung Cottbus
trader_type: autoverwertung
state: BB
city: Cottbus
street: Laubenhof 9
postcode: ''
phone: 01632337268
email: info@autoverwertung-cottbus.de
opening_hours: Mo-Sa 8-21, So zu
website: https://autoverwertung-cottbus.de
website_status: aktiv
status: aktiv
description: 'Autoverwertung Joseph Dina, Laubenhof 9, 45326 Essen (Altenessen): Autoverwertung und Autoentsorgung mit kostenloser Abholung, kostenloser Abmeldung und Verwertungsnachweis, Auszahlung bar oder per Echtzeitüberweisung bei Übergabe. Die Website autoverwertung-cottbus.de ist eine Orts-Marke desselben Inhabers (Impressum Joseph Dina, Telefon 01632337268); ihr Impressum nennt dieselbe Anschrift "Laubenhof 9, 45326 Cottbus", wobei 45326 und die Straße Laubenhof in OSM zu Essen-Altenessen gehören. In Cottbus selbst ist keine Laubenhof-Straße in OSM erfasst. Ankaufpreise werden nach Fahrzeugzustand ermittelt, nicht als Zahlenliste.'
dropoff_json: ''
pickup_json: '{"allowed": true, "conditions": "Kostenlose Abholung (Betreiber-Website, Feedback-Triage 01.10.2026)"}'
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Autoverwertung Cottbus

## Überblick

Autoverwertung Joseph Dina, Laubenhof 9, 45326 (Impressum der Betreiber-Website), Telefon
01632337268. Die im Dossier geführte Ortsangabe Cottbus ist offen: Die Postleitzahl 45326
und die Straße Laubenhof gehören nach OSM zu Essen-Altenessen, nicht zu Cottbus, und in
Cottbus ist keine Laubenhof-Straße erfasst. Betrieben werden eine Autoverwertung mit
kostenloser Abholung, kostenloser Abmeldung, Verwertungsnachweis und Auszahlung bar oder
per Echtzeitüberweisung bei Übergabe. Ankaufpreise werden nach Substanz, Laufleistung,
Antrieb, Getriebe, Ausstattung, Teilewert, Demontageaufwand, Fahrzeugpapieren, VIN,
Schlüsseln und Materialmix ermittelt; eine Zahlenliste wird nicht veröffentlicht.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Unfall-/Schrottautos, kostenlose Abholung, Online-Bewertung
- Adresse: Cottbus, Laubenhof 9
- Adressbeleg: Seed-notes + autoverwertung-cottbus.de
- Adressbeleg: https://autoverwertung-cottbus.de, https://autoverwertung-cottbus.de/impressum

### Recherche 01.10.2026

- [Recherche 01.10.2026: berechtigt eingearbeitet — street auf Laubenhof 9 bereinigt (Impressum Joseph Dina), pickup_json kostenlose Abholung, website_status aktiv; PLZ 45326 anomal (Essen-PLZ statt 030xx) daher kein PLZ-Fill, Straßburger Str. 8 nur Timeline (abweichende Angabe, Einzelbeleg), kein E-Mail-Beleg; Quelle(n): https://autoverwertung-cottbus.de + Impressum (Abruf 01.10.2026)]

### Recherche 30.10.2026

- [Recherche 30.10.2026: Der Orts-Klärfall vom 01.10.2026 (PLZ 45326 als "anomal" eingestuft) ist aufgelöst, und zwar zugunsten von Essen, nicht von Cottbus. Das Impressum von autoverwertung-cottbus.de (live abgerufen) nennt wörtlich "Angaben gemäß § 5 TMG Joseph Dina Laubenhof 9 45326 Cottbus Telefon: 01632337268"; dieselbe Adresse Laubenhof 9 in derselben PLZ steht im Impressum der Parallelseite erkelenz-autoverwertung.de mit "Joseph Dina Laubenhof 9 45326 Erkelenz", und die Seiten teilen sich Telefon 01632337268, Layout und Impressumstext. Ein Verzeichniseintrag "Joseph Dina Autoverwertung" (11880) führt denselben Betrieb unter "Laubenhof 9, 45326 Essen, Ruhr". Die Postleitzahl 45326 ist die von Essen-Altenessen: OSM erfasst die Laubenhof in Essen mit `addr:city=Essen` und `addr:postcode=45326`, unter anderem way/159635617 und way/159635631 mit `addr:housenumber=9` und `addr:street=Laubenhof`; `addr:suburb=Altenessen` tragen die Objekte node/8018125412 und node/14271358313 an den Hausnummern 25 und 27. Eine Nominatim-Suche nach "Laubenhof" in Cottbus liefert keinen Treffer in Cottbus; die einzigen deutschen Laubenhof-Treffer liegen in Essen, Braunschweig, Osnabrück, Auerbach und Schwarzhofen. Das Dossier-Feld city bleibt "Cottbus" (Slug und Dateipfad bleiben aus Stabilitätsgründen unverändert), die Anschrift bleibt wie belegt, aber **keine** PLZ wird ergänzt: Die PLZ 45326 ist belegt, gehört aber zu Essen; der Betrieb selbst schreibt "Cottbus" an dieselbe Anschrift. Ein Beleg für einen tatsächlich in Cottbus betriebenen Standort existiert nicht. Der Klärfall vom 01.10.2026 wird damit als "Adresse liegt in Essen, die Betreiber-Website vermarktet den Standort als Cottbus" präzisiert, nicht als Fehler der Vorwelle. Die zweite auf der Website genannte Anschrift "Straßburger Straße 8, 77694 Kehl am Rhein" (im Impressum als Zuständigkeit der "Universalschlichtungsstelle des Zentrums für Schlichtung e.V." genannt) ist keine Betriebsanschrift; sie bleibt als Einzelbeleg nur in der Timeline. E-Mail-Fill: Das strukturierte Datenformat der Startseite (JSON-LD, `application/ld+json`) nennt im contactPoint wörtlich `"email":"info@autoverwertung-cottbus.de"` und `"telephone":"01632337268"`; das Feld email wird deshalb gesetzt. Zu Ankauf, Verkauf und Gebühren getrennt: ANKAUF - die Betreiber-Website nennt als Wertfaktoren "Substanz, Laufleistung, Antrieb/Abgasnorm, Getriebe und Ausstattung des Fahrzeugs", dazu "Teilewert und Demontageaufwand", "Vollständige Fahrzeugpapiere, eine stimmige VIN und komplette Schlüssel" sowie "Materialmix und Verwertungsquote"; es wird ein "unverbindliches, marktgerechtes Angebot" nach Online-Anfrage erstellt, aber **kein** Zahlenpreis und keine Preisliste veröffentlicht. VERKAUF - kein Verkauf von Ersatzteilen: die Website hebt "KEINE ERSATZTEILE" ausdrücklich hervor, es werden Fahrzeuge angekauft, nicht Teile verkauft. GEBÜHREN - die Abholung ist kostenlos ("Kostenlose Abholung"), die Abmeldung bei der Zulassungsstelle erfolgt kostenfrei ("Die Abmeldung bei der Zulassungsstelle übernehmen wir standardmäßig für Sie"), eine Gebührentafel oder Bearbeitungsgebühr wird nicht genannt; die Auszahlung erfolgt "bei der Übergabe per Echtzeitüberweisung oder bar auf die Hand", eine Zahlungsgebühr ist nicht angegeben. OSM-Abgleich live für die tatsächlich belegte Anschrift (Rasterfenster um den Adresspunkt 51.4834153/6.9919648, bbox 6.9879648,51.4794153 bis 6.9959648,51.4874153, Straße "Laubenhof" in Essen): 28 Adressobjekte mit `addr:housenumber` und `addr:street=Laubenhof`, daraus **24 verschiedene Hausnummern** (1, 2, 2a, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 15a, 16, 18, 19, 21, 23, 25, 27); die Nummer 9 ist durch zwei Objekte belegt (way/159635617 mit `building=commercial` und way/159635631 mit `building=yes`), die Nummern 25 und 27 tragen zusätzlich `addr:suburb=Altenessen` (node/8018125412 und node/14271358313). Im Raster tragen 93 Objekte `name=` oder `operator=`; **0** davon nennen Joseph Dina, den Betrieb oder eine Autoverwertung; ein Objekt trägt `name=Zollamt Essen` mit `office=government` (node/13571113389), ein weiteres `name=WJM Witra Spedition` mit `office=logistics` (node/8018122367), node/13571113389 liegt an der Nachbarhausnummer 8. Das ist eine Aussage zum OSM-Bestand und kein Negativbeleg gegen die Betreiberanschrift. Ein OSM-Abgleich für "Laubenhof" in Cottbus war mangels auffindbarer Straße nicht möglich (kein Treffer). Quelle(n): https://autoverwertung-cottbus.de/impressum ; https://autoverwertung-cottbus.de/ ; https://www.erkelenz-autoverwertung.de/impressum ; https://www.erkelenz-autoverwertung.de/ ; https://www.11880.com/branchenbuch/essen-ruhr/060440459B113959553/joseph-dina-autoverwertung.html ; OSM-API-Abfrage api.openstreetmap.org/api/0.6/map.json?bbox=6.9879648,51.4794153,6.9959648,51.4874153 (Abruf 30.10.2026)]

- [30.10.2026 (Korrekturvermerk): **Elementtyp berichtigt**: die zitierte Objekt-ID 13571113389 existiert in OSM als **node** (`addr:housenumber=8`, `addr:street=Laubenhof`, `name=Zollamt Essen`, `office=government`); ein gleichnamiger **way** existiert nicht (live HTTP 404). Quelle(n): https://api.openstreetmap.org/api/0.6/node/13571113389.json (Abruf 30.10.2026)]
