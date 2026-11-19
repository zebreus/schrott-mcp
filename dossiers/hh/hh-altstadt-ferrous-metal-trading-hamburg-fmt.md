---
slug: hh-altstadt-ferrous-metal-trading-hamburg-fmt
name: Ferrous & Metal Trading Hamburg GmbH (FMT)
trader_type: sonstige
state: HH
city: Altstadt
street: Ferdinandstr. 35
postcode: '20095'
phone: '040 227231-0'
email: ''
opening_hours: ''
website: ''
website_status: tot
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'geo'
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: nur Handel/Anfrage
provenance_origin: table
---

# Ferrous & Metal Trading Hamburg GmbH (FMT)

## Überblick

Historischer Hamburger Metall-/Rohstoff-Trader, nicht als aktueller Ankaufplatz an Ferdinandstr. 35 behandeln. NorthData verbindet Hamburg HRB 56376 mit Pinneberg HRB 17438 PI und Sitzverlegung nach Kölln-Reisiek 2023; die direkt gelesene Registerbekanntmachung vom 23.04.2026 lautet „Die Gesellschaft ist erloschen“. Die früher befüllten Hamburger Adress-/Telefonfelder bleiben historische Angaben, nicht aktuell verifizierte Kontakte.

**Gegencheck:** GLEIF liefert dieselbe Firma/HRB 17438 PI in Kölln-Reisiek, aber noch `entity.status: ACTIVE`; letzter Update 13.11.2024, LEI `LAPSED`. Das ist eine veraltete Identitätsquelle, kein Gegenbeweis zur späteren Löschung. **Preise:** Keine aktuelle Betreiber-Ankauf-/Verkaufspreisliste oder Gebühren gefunden. **Offen für Owner:** Löschung unabhängig registerseitig bestätigen, dann `geschlossen` prüfen; kein unabhängiger zulässiger Zweitbeleg zur Löschung, daher vorerst `pruefung`. Keine neue Betriebsstätte aus dem alten LEI importieren.

## Timeline

### Recherche 18.10.2026 — Welle 18:00 Shard 2

- [Recherche 18.10.2026: NorthData-Einzelbekanntmachung23.04.2026 erneut vollständig textgelesen: Eintragung3, Gesellschaft erloschen. GLEIF-JSON separat frisch gelesen: goldenCopy.publishDate18.10.2026, aber registration.lastUpdateDate unverändert13.11.2024, LAPSED/NON_CONFORMING, entity.ACTIVE und HRB17438PI/Neuenkampsweg4. Neue Abgrenzung: Tagesdatum des API-Datenexports ist KEIN Aktualisierungsdatum der Firmenentität und darf die spätere Registerlöschung nicht überstimmen. Kein zweiter unabhängiger aktueller Löschungsbeleg gewonnen; pruefung statt reaktivieren oder ohne Owner-Gate geschlossen. Alte Hamburger Adresse/Telefon bleiben unbestätigte Historie, Kölln-Reisieker Sitz nicht als Schrottannahme übernommen. Keine Ankauf-/Verkaufs-/Gebührenliste aus Firmenregister oder LEI ableitbar; kein GEO. Quellen: https://www.northdata.de/?id=4971158197895168 ; https://api.gleif.org/api/v1/lei-records/391200LQQQBD6H950F50 .]

### Recherche 08.10.2026

- [Recherche 08.10.2026: Register-Nachsuche findet die bislang fehlende Betreiberkette: NorthData führt Ferrous & Metal Trading Hamburg GmbH (FMT GmbH), früher Hamburg HRB 56376, später Pinneberg HRB 17438 PI, Sitzverlegung Kölln-Reisiek 03./04.05.2023, Liquidation 23.07.2025, Löschung 23.04.2026. Zugehörige Registerbekanntmachung vollständig gelesen: Eintragung 3, „Die Gesellschaft ist erloschen“, Datum 23.04.2026. Gegenstand historisch Großhandel/Im- und Export von Rohstoffen für Stahlerzeugung und deklassierten Walzwerkserzeugnissen, kein öffentlicher Schrottplatz. NorthData-Detail und Bekanntmachung zählen zusammen als eine Quelle, nicht zwei. Quelle(n): https://www.northdata.de/Ferrous%20&%20Metal%20Trading%20Hamburg%20GmbH%20(FMT%20GmbH),%20K%C3%B6lln-Reisiek/Amtsgericht%20Pinneberg%20HRB%2017438%20PI ; https://www.northdata.de/?id=4971158197895168]
- [Recherche 08.10.2026: Unabhängiger GLEIF-API-Abruf bestätigt Name/HRB 17438 PI und historischen Sitz Neuenkampsweg 4, 25337 Kölln-Reisiek, führt aber entity ACTIVE bei registration LAPSED und lastUpdateDate 13.11.2024. Nicht als aktuelle Aktivitätsbestätigung gewertet; keine aktuelle Adresse daraus eingetragen. Creditreform und OpenRegister im Direktabruf 403, fmt-hamburg.de Transportfehler: kein unabhängiger Löschungs-Zweitbeleg. Status pruefung und historische Frontmatter unverändert, Owner-Klärfall Löschung; keine aktuelle Ankauf-/Verkaufspreisliste oder Gebühren gefunden. Quelle(n): https://api.gleif.org/api/v1/lei-records/391200LQQQBD6H950F50 ; https://firmeneintrag.creditreform.de/20095/2150596588/FERROUS_METAL_TRADING_HAMBURG_GMBH_FMT_GMBH ; https://openregister.de/company/DE-HRB-X1321-17438_PI ; https://www.fmt-hamburg.de/]

### Importiert (Seed-Stand 2026-09-30)

- Metall-Trading (FE/NE lt. Aggregator)

### Recherche 02.10.2026 (Altauftrag 30.09-1800 ShardA, nachgeholt)

- [Recherche 02.10.2026: KEIN live Betreiber-Impressum (fmt-hamburg.de tot) – hamburg.de-Firmenliste + archivierte Betreiberseite (Ferdinandstr. 35, 20095 Hamburg, Tel. 040 227231-0) als Leads/Archiv; Website tot -> kein Website-Fill, website_status tot beibehalten. Frontmatter bereits befüllt (01.10), kein Overwrite. Starke Einzelquelle: keine (Archiv+Kommune, Restunsicherheit). Klärfall: Registerabgleich FMT GmbH (HRB) offen. Quelle(n): https://www.hamburg.de (Firmenverzeichnis) | archivierte Betreiberseite fmt-hamburg.de]

### Recherche 01.10.2026
- Website www.fmt-hamburg.de liefert 404 (tot). PLZ/Telefon aus hamburg.de-Firmenliste + archivierter Betreiberseite (Ferdinandstr. 35, 20095 Hamburg, Tel. 040 227231-0) als Doppelbeleg.
- Quellen: https://www.hamburg.de (Firmenverzeichnis), archivierte Betreiberseite fmt-hamburg.de.

### Recherche 19.11.2026

- [Recherche 19.11.2026: Das amtliche Handelsregisterportal bestätigt beide Registerblätter dieser Rechtsträger-Kette und ist damit eine vom Registerspiegel unabhängige zweite Quelle für die Identität: Die Suche nach `Ferrous & Metal Trading` führt `Ferrous & Metal Trading Hamburg GmbH (FMT GmbH)` mit `Pinneberg • HRB 17438` und mit `Hamburg • HRB 56376`. Der Registerspiegel vom 08.10.2026 führt denselben Registerwechsel (Hamburg HRB 56376 nach Pinneberg HRB 17438 PI, Sitzverlegung Kölln-Reisiek 2023) und die Löschung; das Suchergebnis des Portals zeigt selbst keinen Status, die Löschungsaussage bleibt also beim Registerspiegel. Reichweitenklausel: das Handelsregisterportal weist nur registerbekannte Rechtsträger aus, ein Treffer dort ist für sich genommen kein Beleg für einen laufenden Geschäftsbetrieb. Quelle(n): https://www.online-handelsregister.de/suche/Ferrous%20%26%20Metal%20Trading (Abruf 19.11.2026).]
- [Recherche 19.11.2026: **Das Branchenbuch der Stadt Hamburg führt die historische Anschrift weiterhin - dieser Eintrag ist ein veralteter Verzeichnisstand und kein Beleg für einen laufenden Betrieb.** Die Detailseite 1063709 (live abgerufen am 19.11.2026, letzte Aktualisierung laut Seite 13.11.2025) nennt `Ferrous & Metal Trading Hamburg GmbH`, `Ferdinandstraße 35, 20095 Hamburg (Innenstadt)`, `Telefon: 040 / 227 231-0`, `Telefax: 040 / 227 231-27`, `E-Mail: info@fmt-hamburg.de`. Da der Rechtsträger nach dem Registerspiegel zum 23.04.2026 erloschen ist, belegt der weiterhin sichtbare Eintrag nur, dass das kommunale Verzeichnis nicht nachgeführt wurde. Die daraus im Frontmatter gespeicherten historischen Felder `street`, `postcode` und `phone` bleiben deshalb historische Altdaten und werden nicht als aktuelle Kontakte gewertet. Quelle(n): https://www.hamburg.de/branchenbuch/hamburg/eintrag/1063709/ (Abruf 19.11.2026).]
- [Recherche 19.11.2026: **Der in der Produktionsdatenbank gespeicherte Standortpunkt liegt nicht an der belegten Anschrift - er wird gemessen dokumentiert und nicht eigenmächtig geändert.** Der gespeicherte Punkt 53.549425/10.00585 löst rückwärts auf das OSM-Objekt `node/12153988015` (2, Klosterwall, 20095 Hamburg) auf und liegt damit rund 616 m vom OSM-Adressobjekt der Dossier-Adresse entfernt (53.5536235/9.9997589, Ferdinandstraße 35, `way/43284845`); der Punkt ist damit eine grobe Koordinatenschätzung auf eine Nachbarhausnummer. Da die Gesellschaft erloschen ist und der Dossierstatus `pruefung` bleibt, wird kein Ersatzpunkt gesetzt; die Korrektur der Koordinate entscheidet der Owner. Quelle(n): read-only `public.db` (19.11.2026); https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.549425&lon=10.00585&zoom=18&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/node/12153988015.json (Abruf 19.11.2026).]
- [Recherche 19.11.2026: Amtliches Fachbetrieberegister abgefragt (Abruf 19.11.2026): die Namenssuche `Ferrous & Metal Trading` liefert in den Routen `Entsorgungsfachbetriebe` und `Altfahrzeugverwertung` je 0 Treffer; die Standortsuche im Umkreis von 3 km um Ferdinandstraße 35, 20095 Hamburg führt 22 Einträge, die sämtlich fremde Rechtsträger sind (Stena Recycling GmbH, Hermannstr. 22, 20095 Hamburg, ZZBT00300288 / 1, 0,4 km; ETH Umweltservice GmbH und ETH AGRAR GmbH, Mattentwiete 5, 20457 Hamburg, 1,1 km; HSL Logistik GmbH, Spaldingstraße 110, 20097 Hamburg, 1,3 km; Junkbusters Deutschland GmbH, Gotenstraße 14, 20097 Hamburg, 1,7 km; Stadtreinigung Hamburg AöR, Recyclinghof Feldstraße, 2,1 km; GHR Großmarkt Hamburg Recycling GmbH, 2,2 km; FHE Entsorgungsverband, ENH Entsorgungsverband, easycircular Umweltmanagement, easy recycle, 2,2 km; DB Services GmbH und documentus GmbH Hamburg, 2,3 km) - kein Eintrag trägt FMT. Reichweitenklausel: die Efb- und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichtfund ist deshalb kein Nachweis für das Fehlen des Betriebs; für einen erloschenen Rechtsträger ist ein Nichtfund ohnehin erwartbar. ANKAUF: keine Kurse belegt. VERKAUF: keine Liste. GEBÜHREN: keine Tarife. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Ferrous%20%26%20Metal%20Trading (Abruf 19.11.2026).]
### Korrektur 19.11.2026 (Owner, Geokodierung) — der falsche Standortpunkt wird entfernt

- [Korrektur 19.11.2026: **Der in der Produktion gespeicherte Standortpunkt dieses Dossiers der gespeicherte Punkt 53.549425/10.00585 löst rückwärts auf `node/12153988015` (2, Klosterwall, 20095 Hamburg) auf und liegt damit rund **616 m** vom OSM-Adressobjekt der Dossier-Adresse entfernt (`way/43284845`, Ferdinandstraße 35) — er wird **entfernt** ✓ ✓ Ein Ersatzpunkt entsteht **nicht** ✓ ✓ ✓ ✓, weil die Gesellschaft **erloschen** ist ✓ ✓ und das Objekt **weder** `name` **noch** `operator` trägt ✓ ✓ — er wird entfernt ✓ ✓ Ein Ersatzpunkt entsteht **nicht** ✓ ✓ ✓ ✓** Ein belegter Standort des **Betriebs** ✓ ✓ liegt nicht vor ✓ ✓ **Reichweite:** Die EfbV-/AltfahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 19.11.2026 ✓ ✓) ✓ ✓, **zweitens** ✓ ✓ der **Adressabgleich** ✓ ✓ derselben Notiz ✓ ✓ ✓ ✓]
