---
slug: he-sinn-35764-risse-hildebrand
name: Risse-Hildebrand
trader_type: schrotthaendler
state: HE
city: Sinn 35764
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4b – Kandidaten-Sweep (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Risse-Hildebrand

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Kleinst-Schrotthändler
- Adresse: Sinn 35764

### Recherche 01.10.2026

- [Recherche 01.10.2026: Keine Betreiber-Website, kein Registerbeleg gefunden. Nur Aggregator-Leads (keine Belege): In der Großen Wiese 8, 35764 Sinn; Telefon WIDERSPRÜCHLICH (06449) 6432 (schrottplatz-info) vs. 06449-717976 (tellows); „Risse & Hildebrand GbR" (unternehmensauskunft, Aggregator). Kein Fill, Status pruefung bleibt, Klärfall: Existenz/Adresse/Telefon vor Ort bzw. Gewerberegister Sinn klären; Quelle(n): DDG-Recherche 01.10.2026, Leads schrottplatz-info.de, 11880, tellows, unternehmensauskunft.com]

### Recherche 29.10.2026

- [Recherche 29.10.2026: **OSM-Prüfung der konkreten Hausnummer: „In der Großen Wiese 8“ existiert, trägt aber einen ganz anderen Betrieb.** Raster über den gesamten Straßenverlauf „In der Großen Wiese“ in Sinn-Edingen (way/217621275 mit 17 Nodes, bbox 50.633462/8.323341 bis 50.636071/8.324702, Kachelraster 1x2, 0 Kachel-Fehler) ergibt **8 Adressobjekte und 7 verschiedene Hausnummern** (2, 4, 6, 8, 10, 12, 14). Die gesuchte **Hausnummer 8 existiert**: `way/162421838` mit `addr:housenumber=8`, `addr:street=In der Großen Wiese`, `addr:city=Sinn`, `addr:postcode=35764`, `addr:suburb=Edingen`, `building=commercial`, `building:levels=1` und `name=ABG Abschleppdienst & Baggervermietung`. Das Objekt beschreibt damit einen **Abschleppdienst mit Baggervermietung**, nicht einen Schrotthandel; ein **Risse- oder Hildebrand-Objekt existiert an dieser Anschrift nicht**. Quelle(n): https://api.openstreetmap.org/api/0.6/map?bbox=8.323341,50.633462,8.324702,50.636071 ; https://api.openstreetmap.org/api/0.6/way/162421838 ; Abruf 29.10.2026.]
- [Recherche 29.10.2026: **Amtliche Gegenproben mit exaktem Umfang; kein Betriebsbeleg für Risse-Hildebrand.** Die Namensrecherche im gemeinsamen Registerportal der Länder (HRA/HRB, alle Registerarten, Suche „alle Wörter müssen vorkommen“) nach „Risse Hildebrand“ liefert **0 Treffer**; die Suche nach „Hildebrand“ allein liefert **einen** Treffer, Hamburg, Amtsgericht Hamburg, HRB 38411, also **keinen** Registereintrag in Hessen. Einzelunternehmer werden nicht ins HRA/HRB eingetragen, ein 0-Treffer-Befund beweist daher **kein Nichtbestehen**. Die Ortsseite Sinn des amtlichen Fachbetrieberegisters (Auszug 11.08.2026, Postleitzahl 35764) führt **genau einen** anerkannten Betrieb, „PreZero Service Mitte-West GmbH & Co. KG NL Dillenburg, BS Sinn“, Herborner Straße 7-9, Tätigkeit Lagern und Behandeln, und **keinen** „Risse“ oder „Hildebrand“; ein anerkannter Demontagebetrieb ist in Sinn nicht verzeichnet. Die Bereichsangabe 7-9 ist einzeln aufzulösen und umfasst die Nummern 7, 8 und 9; ob diese drei Nummern einzeln in OSM erfasst sind, ist in dieser Welle **nicht** geprüft worden, weil der Straßenzusammenhang nicht Gegenstand der Seed-Adresse ist. Das Fehlen im Register ist ein **Hinweis ohne Beweiswert**, da die Efb-Erkennung freiwillig ist. Quelle(n): https://www.handelsregister.de/rp_web/normalesuche/welcome.xhtml ; https://schrottregister.pages.dev/ort-sinn ; Abruf 29.10.2026.]
- [Recherche 29.10.2026: **GEO und Frontmatter unverändert; die Adresslage ist als belastbarer Kandidaten bestätigt, die Identität nicht.** Der in Produktion gespeicherte Punkt 50.65/8.3333 löst per Reverse-Geocoding auf **„12, Gutenbergstraße, Sinn, 35764“** auf, also eine **andere Straße** als die belegte Adresse In der Großen Wiese 8; er wird **nicht** eigenmächtig zurückgezogen, weil für den Eintrag keine belegte Betriebsadresse vorliegt, sondern dem Owner als offener Punkt gemeldet. `street`, `postcode` und `phone` bleiben unverändert: die Adresse stammt aus Aggregator-Leads und ist erst jetzt durch OSM als **existierende Hausnummer** bestätigt, nicht als Betriebsstandort; der Telefonwiderspruch (06449 6432 gegen 06449 717976) aus der Vorwelle bleibt bestehen, und ein Unternehmen „Risse & Hildebrand GbR“ ist im Register **nicht** auffindbar. `status` bleibt `pruefung`. ANKAUF/VERKAUF/GEBÜHREN: keine belegten Kurse, keine Verkaufsliste, keine Tarife; der am OSM-Objekt genannte Abschleppdienst begründet für diesen Eintrag **keinen** Ankauf. Quelle(n): https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.65&lon=8.3333&zoom=18&addressdetails=1 ; Abruf 29.10.2026.]

- [30.10.2026 (Korrekturvermerk): **Korridorzählung berichtigt**: die Nachmessung am 30.10.2026 mit demselben Raster (bbox 8.323341,50.633462,8.324702,50.636071) ergibt **8** Adressobjekte mit `addr:housenumber` (bisher 31) und **7** verschiedene Hausnummern (bisher 17). **Ein Nummernwert „14,16“ kommt im OSM nicht vor** — die entsprechende Behauptung eines Datenartefakts ist damit hinfällig. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=8.323341,50.633462,8.324702,50.636071 (Nachmessung 30.10.2026)]
