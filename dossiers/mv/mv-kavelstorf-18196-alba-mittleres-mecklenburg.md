---
slug: mv-kavelstorf-18196-alba-mittleres-mecklenburg
name: ALBA Nord GmbH
trader_type: sonstige
state: MV
city: Kavelstorf
street: Silder Moor 10
postcode: '18196'
phone: +49 38208-8390
email: ''
opening_hours: ''
website: https://nord.alba.info/
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# ALBA Mittleres Mecklenburg GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Eisen/NE/Container
- Adresse: Kavelstorf 18196, Silder Moor 10

### Korrektur 01.10.2026 (ALBA-Fix)

- Betreiber-Primärquelle: alba.info/standorte (Corporate-Standortliste, Impressum via alba.info) führt „ALBA Nord GmbH, Betriebsstätte Kavelstorf, Silder Moor 10, 18196 Kavelstorf, +49 38208-8390“ — KEINE „ALBA Mittleres Mecklenburg GmbH“ als Betreiber bekannt.
- name auf Betreiber-Stand korrigiert (ALBA Nord GmbH); Altname „ALBA Mittleres Mecklenburg GmbH“ hier in Timeline archiviert. Slug unverändert.
- Frontmatter: street/postcode/city/phone aus Corporate-Liste; website auf nord.alba.info (ALBA Nord GmbH, verifiziert erreichbar 01.10.2026) + aktiv. Hinweis: ALBA Nord GmbH = Entsorger (Gewerbe/Siedlungsabfälle), nicht ALBA Metall Nord — Schrottankauf-Fakten weiter offen, Leistungen Eisen/NE/Container aus Seed-Provenienz.
- status pruefung → aktiv (Betreiber-Existenz belegt).

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Silder Moor, Kavelstorf) — die Dossier-Hausnummer 10 existiert in OSM nicht.** Straßen-Raster über den **gesamten** Straßenverlauf (Straßenbbox 12.2057483/54.0122572 bis 12.2091187/54.0149217), Scan-Fenster **12.2037483/54.0102572 bis 12.2111187/54.0169217** (**1 Kachel im 0,01°-Raster des Repo-Werkzeugs `scripts/release/osm_street_scan.py`, 0 Kachel-Fehler**). Zählung getrennt: **7 Adressobjekte / 7 verschiedene Hausnummern** auf der Straße (2, 3, 4, 5, 6, 7, 9) - die Aufzählung nennt exakt die live vorhandenen Nummern; Filter-Key `addr:street=Silder Moor` (mit `addr:housenumber` als Hausnummer). Keine Bereichs- oder Buchstabennotation auf dieser Straße. Im gesamten Rastergebiet **11 Adressobjekte mit Hausnummer / 11 verschiedene Hausnummern**; im Raster tragen **20 Objekte `name=`** und **10 Objekte `operator=`**. **Die Dossier-Hausnummer 10 existiert auf dem Silder Moor in OSM NICHT** - das ist zu schreiben, nicht zu übergehen: im geprüften Fenster gibt es **kein** Objekt mit `addr:street=Silder Moor` und `addr:housenumber=10`, und **kein** Objekt des Fensters trägt überhaupt die Hausnummer 10. Objekte mit Bezug zum Betrieb im Raster: **keines** - die Suche nach „alba“, „kavelstorf“, „schrott“ und „nord“ über die Felder name, operator, description, website, contact:website, note, contact:email, email, short_name, brand und operator:name findet **keinen** Treffer mit Betriebsbezug zur ALBA Nord GmbH; **kein** Objekt des Rasters trägt einen `name=`- oder `operator=`-Wert mit Bezug zu „ALBA“. Die `name=`-Objekte der Straße sind **Fremdbetriebe**: **way/41646690** mit `addr:housenumber=9`, `addr:street=Silder Moor`, `addr:city=Dummerstorf`, `addr:postcode=18196`, `addr:suburb=Kavelstorf`, `name=Hermann Stitz & Co ROSTOCK KG`, `building=industrial`, `phone=+49 38208 660`; **way/41646691** mit `addr:housenumber=5`, `name=Johannes J. Matthies Gmb`, `building=commercial`; **way/98989990** mit `addr:housenumber=7`, `name=Meesenburg Großhandel`, `building=commercial`, `building:levels=1`; dazu way/30791745 mit `name=Gewerbegebiet Kavelstorf Ost`, `landuse=commercial` (ohne Hausnummer). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=12.2037483,54.0102572,12.2111187,54.0169217 (Kachelraster, live abgerufen 31.10.2026) ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Betreiber-Primärquelle bestätigt die Betriebsstätte Kavelstorf weiterhin wörtlich — mit unveränderter Telefonnummer.** Die **Corporate-Standortliste** alba.info/standorte (live am 31.10.2026, HTTP 200) führt unter **„ALBA Nord GmbH“** den Eintrag **„Betriebsstätte Kavelstorf / Silder Moor 10 / 18196 Kavelstorf / +49 38208-8390 / Auf Karte anzeigen“** — identisch zur Vorwellen-Notiz vom 01.10.2026 (Adresse und Telefonnummer unverändert). Die **eigene Standortseite** der ALBA Nord GmbH (nord.alba.info) beschreibt das Geschäft: „**Seit 70 Jahren: Gut aufgestellt in Mecklenburg-Vorpommern** — In der gesamten Region Mecklenburg-Vorpommern kümmern wir uns um die **Entsorgung und Verwertung von Gewerbe- und Siedlungsabfällen**“ und listet als Leistungsfeld **„Stahl- und Metallrecycling“** neben Gewerbe, Privat, Kommunale Dienstleistungen, Abbruch- und Tiefbauarbeiten; als Kontakt nennt sie **+49 385 4811 0** und **nord (at) alba.info**. Die **Rechtsform-Kette** aus der Vorwelle (keine „ALBA Mittleres Mecklenburg GmbH“ als Betreiber bekannt; ALBA Nord GmbH ist der Entsorger, **nicht** ALBA Metall Nord) bleibt damit **unverändert bestätigt**. Quelle(n): https://alba.info/standorte/ ; https://nord.alba.info/ ; https://nord.alba.info/standorte/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Ankauf, Verkauf und Gebühren getrennt.** **ANKAUF:** die ALBA-Gruppe nennt den Schrott-Ankauf als Standardleistung: „Die Spezialist*innen von ALBA nehmen **auf all unseren Plätzen** alle Arten von **Stahl- und Metallschrott zu Bestpreisen** entgegen und sorgen für deren fachgerechte Verwertung“ (metall.alba.info/ankauf/); für Gewerbe gilt: „Vom **Ankauf** und der kompletten Entsorgung sämtlicher **Eisen- und Nichteisenmetalle** über Demontage-, Rückbau- und Abbrucharbeiten bis hin zur **digitalen Materialanalyse** und transparenten Dokumentation“ (metall.alba.info/ankauf/gewerbe/). Die **Sorten** sind auf der Sorten-Seite (metall.alba.info/ankauf/sorten/) benannt: **Produktionsschrotte, Haushaltsschrott, Industrie- und Abbruchschrotte, Kabelschrotte, Mischschrotte, Vormaterial, Schwere Schere, Legierte Schrotte, Karossen, Aluminium, Kupfer, Messing, Brennschrott, Gusseisen, Moniereisen, Scherenschrott, Stahl- und Metallspäne** — das deckt die Seed-Angabe „Eisen/NE“ inhaltlich. **Zahlenpreise werden nicht veröffentlicht.** **VERKAUF:** „Verkauf von Nutzmaterial“ ist als ALBA-Leistungsfeld benannt, **ohne** Zahlenpreise. **GEBÜHREN:** **keine** Gebühr beziffert; die **Container-Bestellung** läuft über **ALBAclick** („buchen Sie direkt einen Container in unserem Online-Portal ALBAclick“), **ohne** Preisangabe; die Abfallkalender/Behältergrößen sind im **ALBAclick-Kundenportal** hinterlegt. Quelle(n): https://metall.alba.info/ankauf/ ; https://metall.alba.info/ankauf/gewerbe/ ; https://metall.alba.info/ankauf/sorten/ ; https://nord.alba.info/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen und Geokodierung.** (a) Die **Hausnummer 10** ist in OSM **nicht** belegt, das Nummernband des Silder Moor läuft live von 2 bis 9 — **ob** die Betriebsstätte unter dieser Nummer oder unter einer im OSM nicht erfassten Nummer geführt wird, ist **offen**; die Adresslage stützt sich ausschließlich auf die **Betreiber-Standortliste**. (b) Ob die **ALBA Nord GmbH** an dieser Adresse tatsächlich **Schrott ankauft** (und nicht nur Gewerbe-/Siedlungsabfälle entsorgt), ist über die Gruppe **nicht** standortscharf belegt — die Seite nennt „Stahl- und Metallrecycling“ als Leistungsfeld **der Gesellschaft**, die **Standortliste** führt Kavelstorf als „Betriebsstätte“ **ohne** Leistungsangabe. Das ist die **entscheidende offene Frage** für den Schrott-MCP. (c) Der Dossier-**Slug** und der **Altname** „ALBA Mittleres Mecklenburg GmbH“ bleiben unverändert (DB-Schlüssel, Archivierung in der Timeline). (d) **Geokodierungsbedarf:** die Dossier-Adresse **Silder Moor 10, 18196 Kavelstorf** ist in OSM **nicht** belegt — es existiert **kein** Objekt mit dieser Hausnummer, das als Koordinate dienen könnte; der gespeicherte Produktionspunkt **54.0060000/12.1908000** stammt aus einer groben Seed-Koordinate und ist **nicht** durch ein OSM-Objekt gedeckt. Die Koordinate wird hier **nicht** gesetzt (Auftrag dieser Welle: Adressen benennen, nicht selbst geokodieren). Quelle(n): https://alba.info/standorte/ ; https://api.openstreetmap.org/api/0.6/map.json?bbox=12.2037483,54.0102572,12.2111187,54.0169217 (Abruf 31.10.2026).]
