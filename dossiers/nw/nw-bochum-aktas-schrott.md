---
slug: nw-bochum-aktas-schrott
name: Aktas Schrott
trader_type: schrotthaendler
state: NW
city: Bochum
street: Hofsteder Straße 135
postcode: '44809'
phone: 0173 - 27 04 293
email: ''
opening_hours: ''
website: https://www.aktas-schrott.de
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Neu, Ankauf unklar (nur Verzeichnis-/Profilbeleg)
provenance_ankauf_raw: mobil/unklar
provenance_origin: table
---

# Aktas Schrott

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Hofsteder Str. 135, 0173-Nr. (Backyard-Typ)
- urspr. Website-Angabe: keine gefunden [Website-Recherche website: services: Abholservice Metalle aller Art (Alu, Kupfer, Edelstahl), Metall-/Elektroschrott, Autoverschrottung, Rohre/Bleche; customer_types: privat; notes: Einzelunternehmen Orhan Aktas; E-Mail nur per JS-Spamschutz lesbar.]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2571 verifiziert, kein Frontmatter-Slot — Betreiber-Site (Abruf 01.10.2026): „Unser Abholservice" (Metalle aller Art, Metall-/Elektroschrott, Autoverschrottung) → Abholung belegt, nur Timeline (pickup_json-Enkodierung Folgeschritt); Zweitnummer gefunden (Footer: Mobil 0152-06105095 neben 0173-2704293) → nur Timeline (kein Zweitnummer-Feld im Schema); Impressum-URL 404 (kein HRB → keine Owner-Ausnahme); Quelle(n): https://www.aktas-schrott.de/ (Homepage + Footer)]

### Recherche 30.10.2026

- [Recherche 30.10.2026: Betreiber-Website am 30.10.2026 erneut live gelesen, Impressum (https://www.aktas-schrott.de/index.php?catid=2&id=21:impressum&view=article) unverändert: "Angaben gemäß § 5 TMG / Orhan Aktas / Aktas Schrott / Hofsteder Straße 135 / 44809 Bochum", "Telefon: 0173 - 27 04 293", "E-Mail: This email address is being protected from spambots." — die E-Mail-Adresse bleibt hinter dem JS-Spam-Schutz, ein Impressum ohne HRB/Registernummer; die Owner-Ausnahme greift weiterhin nicht. Die Startseite nennt weiterhin zwei Mobilnummern ("Mobil: 0173 - 27 04 293", "Mobil: 0152 - 06 10 50 95"), die Leistungsliste ("Metalle aller Art", "Metallschrott / Elektroschrott", "Autos / Autoverschrottung / Keine Teile", "Rohre und Bleche", "Kabelreste", "Demontagen"), den kostenlosen Abholservice ("Wir holen ihren Schrott und ihre Altmetalle kostenlos und schnell in ganz Nordrhein-Westfalen ab"), den Entsorgungs- und Demontageblock sowie "Öffnungszeiten / Mo. - Fr. 8:00 bis 17:00Uhr / Sa. 8.00 bis 13:00Uhr / Andere Termine auf Anfrage". Das 11880-Branchenbuch führt parallel "Orhan Aktas Schrotthändler", "Hofsteder Str. 135 44809 Bochum (Hamme)", "(0173) 2704293"; die Gelbe Seiten führen "Aktas-Schrott Entsorgung", "Hofsteder Str. 135, 44809 Bochum-Hamme" mit abweichenden Öffnungszeiten (Mo–Do 08:00–16:00, Fr 08:00–13:00). Adresse und Haupttelefon sind damit über Betreiberimpressum und zwei Verzeichnisse gedeckt, die Öffnungszeiten bleiben widersprüchlich und werden nicht übernommen. Quelle(n): https://www.aktas-schrott.de/ ; https://www.aktas-schrott.de/index.php?catid=2&id=21:impressum&view=article ; https://www.11880.com/suche/schrotthandel/bochum ; https://www.gelbeseiten.de/gsbiz/14827e48-9a85-4ff2-a3fb-011c71fab922]
- [Recherche 30.10.2026: Live-OSM-Straßenscan Hofsteder Straße, Rasterfenster 7.2018/51.4933 bis 7.2118/51.4993: 61 Adressobjekte mit `addr:housenumber` und `addr:street`="Hofsteder Straße", daraus 57 verschiedene Hausnummern — 76, 78, 80, 81, 82, 83, 85, 86, 87, 88, 89, 90, 91, 94, 99, 116, 118, 120, 122, 122a, 122b, 122c, 122d, 122e, 124, 124a, 124b, 124c, 124d, 124e, 126, 126a, 126b, 126c, 126d, 128, 129, 131, 133, 135, 137, 139, 141, 143, 143a, 145, 147, 149, 166, 168, 170, 171, 172, 174, 176, 178, 182a; die Bereichsschreibungen '170-172' (node/3354962030) wurde in die Einzelwerte 170, 171 und 172 aufgelöst. Das Objekt zur Betriebsadresse ist way/539672665 mit `addr:housenumber`='135', `addr:street`='Hofsteder Straße', `addr:city`='Bochum', `addr:postcode`='44809', `addr:country`='DE', `building`='apartments' — es trägt kein `name=`, `operator=` oder `craft=`. Im Raster tragen 5 von 61 Objekten `name=` oder `operator=`, alle mit fremdem Betriebsbezug: node/1336704562 `name`='Zu den vier Winden' mit `amenity`='pub' (94), node/1568218915 `name`='Haargefühl' mit `shop`='hairdresser' (88), node/3354962030 `name`='tedox' mit `operator`='tedox KG' und `shop`='doityourself' (170-172), node/11529767380 `name`='sterne-handel' (182a), way/148055483 `name`='Speckschweiz' mit `amenity`='pub' (78). Treffer mit Bezug zu Orhan Aktas/Aktas Schrott: 0. Die OSM-Erfassung beschreibt die Anschrift als Wohngebäude (`building`='apartments'), was zum Seed-Bild "Backyard-Typ" passt, aber keinen Betriebsort belegt. Quelle(n): OSM-API-Abfrage api.openstreetmap.org/api/0.6/map.json?bbox=7.2018,51.4933,7.2118,51.4993 (Abruf 30.10.2026)]
- [Recherche 30.10.2026: Preise und Gebühren beim Betreiber nicht veröffentlicht: ANKAUF — die Site nennt "Abholservice", "Schrottabholung" und "Entsorgung" sowie "kostenlos" nur für die Abholung ("Wir holen ihren Schrott und ihre Altmetalle kostenlos und schnell ... ab"), nennt aber keinen Vergütungs- oder Ankaufspreis je Kilogramm; die Preisebasis wird nicht angegeben. VERKAUF — kein Verkauf von Ersatzteilen oder Sekundärrohstoffen angeboten ("Autoverschrottung / Keine Teile"); keine Verkaufspreisliste. GEBÜHREN — kostenlose Abholung wird zugesagt, Demontage- und Entsorgungsentgelte werden nicht beziffert; eine Gebührenliste fehlt. Offen bleibt: die gewerbliche Anlieferung/Abgabe am Standort Hofsteder Straße 135 ist nicht ausdrücklich ausgeschlossen, aber auch nicht beschrieben. Quelle(n): https://www.aktas-schrott.de/ ; https://www.aktas-schrott.de/index.php?catid=2&id=21:impressum&view=article]
