---
slug: mv-neubrandenburg-17036-alba-metall-nord-nl-neubrandenburg
name: ALBA Metall Nord NL Neubrandenburg
trader_type: sonstige
state: MV
city: Neubrandenburg
street: Kruseshofer Str. 21
postcode: '17036'
phone: 0395 3698315
email: ''
opening_hours: ''
website: https://metall.alba.info/unternehmen/standorte/
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# ALBA Metall Nord NL Neubrandenburg

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Stahl/NE/Container
- Adresse: Neubrandenburg 17036, Kruseshofer Str. 21, Tel. 0395 3698315

### Recherche 01.10.2026 (Feedback-Triage 2943)

- [Recherche 01.10.2026: Feedback 2943 bestätigt, aber kein neuer Fill — kein Frontmatter-Change: ALBA-Fix 01.10.2026 (s. unten) dokumentiert bereits vollständig, dass Neubrandenburg NICHT auf der aktuellen Betreiber-Standortliste steht (Vollcrawl metall.alba.info ohne Treffer); status pruefung + Klärfall bleiben. Feedback liefert keinen neuen Beleg über den Fix hinaus. Quelle(n): s. ALBA-Fix-Eintrag unten]

### Korrektur 01.10.2026 (ALBA-Fix)

- Delisted-Protokoll: Neubrandenburg steht NICHT auf der aktuellen Betreiber-Standortliste (Vollcrawl 01.10.2026: metall.alba.info = 13 Detailseiten ohne Neubrandenburg; alba.info/standorte ohne Treffer). Ältere Liste hatte 17 Treffer inkl. Neubrandenburg.
- Betreiber-Suche ohne Treffer; Register/News ohne Schließungs-/Verlagerungsbeleg (Verzeichnis-Leads wie Gelbe Seiten/dasoertliche nennen Kruseshofer Str. 21 weiterhin — Aggregatoren nur Leads, kein Beleg) → kein status geschlossen, Dossier bleibt erhalten; status pruefung + Klärfall.
- Frontmatter normalisiert (city/postcode getrennt, street aus Timeline übernommen — Adresse weiter ohne Betreiber-Beleg); website = Domain-Root (aktiv) ohne Detailseite.

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Kruseshofer Straße, Neubrandenburg) — die Dossier-Hausnummer 21 existiert, ist aber nach OSM einem anderen Betrieb zugeordnet.** Straßen-Raster über den **gesamten** Straßenverlauf (Straßenbbox 13.2961809/53.5575495 bis 13.3075578/53.5586319), Scan-Fenster **13.2941809/53.5555495 bis 13.3095578/53.5606319** (**2 Kacheln im 0,01°-Raster des Repo-Werkzeugs `scripts/release/osm_street_scan.py`, 0 Kachel-Fehler**). Zählung getrennt: **24 Adressobjekte / 18 verschiedene Hausnummern** auf der Straße (1, 3, 7, 8, 9, 10, 11, 12, 14, 15, 16, 18, 20, 21, 22, 24, 28, 30) - die Aufzählung nennt exakt die live vorhandenen Nummern; Filter-Key `addr:street=Kruseshofer Straße` (mit `addr:housenumber` als Hausnummer). Keine Bereichs- oder Buchstabennotation auf dieser Straße. Im gesamten Rastergebiet **86 Adressobjekte mit Hausnummer / 71 verschiedene Hausnummern**; im Raster tragen **107 Objekte `name=`** und **89 Objekte `operator=`**. **Die Dossier-Hausnummer 21 existiert** und ist **zweifach** belegt: **way/587299372** mit `addr:housenumber=21`, `addr:street=Kruseshofer Straße`, `addr:city=Neubrandenburg`, `addr:postcode=17036`, `name=SMW Spezialmaschinen und Werkzeugbau GmbH & Co. KG`, `industrial=factory`, `landuse=industrial`, `phone=+49 395 36709 0`, `fax=+49 395 36709 14`, `email=smw@smw-nb.de`, `website=https://www.smw-spezialmaschinen.de` (Objekt-BBox 13.2956864/53.5573311 bis 13.3001848/53.5585894, Mitte 13.2985277/53.5581092) **und** **way/42094726** mit `addr:housenumber=21`, `addr:street=Kruseshofer Straße`, `addr:city=Neubrandenburg`, `addr:postcode=17036`, `building=industrial` (ohne `name=`/`operator=`, Objekt-BBox 13.2961082/53.5574564 bis 13.2994053/53.5583323, Mitte 13.2976119/53.5580012). Das **einzige** Objekt mit Betriebsbezug an der Dossier-Hausnummer nennt einen **anderen Betrieb**: die **SMW Spezialmaschinen und Werkzeugbau GmbH & Co. KG** (Maschinenbau) — **nicht** die ALBA Metall Nord NL Neubrandenburg. Objekte mit Bezug zum Betrieb im Raster: **keines** - die Suche nach „alba“, „metall“, „schrott“ und „nord“ über die Felder name, operator, description, website, contact:website, note, contact:email, email, short_name, brand und operator:name findet **keinen** Treffer mit Betriebsbezug zur ALBA Metall Nord; **kein** Objekt des Rasters trägt einen `name=`- oder `operator=`-Wert mit Bezug zu „ALBA“. Die `name=`-Objekte der Straße sind **Fremdbetriebe**: node/10880990872 mit `name=Jesus Punkt` (`amenity=place_of_worship`, `denomination=pentecostal`, `addr:housenumber=20`), node/9077320999 mit `contact:website=https://www.reifenhelm.de` (`addr:housenumber=8`), node/9077321082 mit `contact:email=info@kfz-meisterbetrieb-peter-dahl.de` (`addr:housenumber=18`), way/95507199 mit `name=Mandt Büroorganistation` (`addr:housenumber=7`), way/117848710 mit `brand=Lidl` (`addr:housenumber=1`), way/1013859629 mit `brand=Boels` (`addr:housenumber=3`) und way/1012894899 mit `brand=Audi` (`addr:housenumber=9`). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=13.2941809,53.5555495,13.3095578,53.5606319 (Kachelraster, live abgerufen 31.10.2026) ; https://api.openstreetmap.org/api/0.6/way/587299372.json ; https://api.openstreetmap.org/api/0.6/way/42094726.json ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Korrektur 31.10.2026 (Korrekturvermerk zum ALBA-Fix vom 01.10.2026): die Adresslage ist geklärt, die Filial-Existenz bleibt offen.** Die Vorwellen-Notiz „Adresse weiter ohne Betreiber-Beleg“ bleibt **wortgleich** bestehen. Neu ist: In OSM ist die **Hausnummer 21 der Kruseshofer Straße** zwar erfasst, das dort benannte Objekt nennt aber die **SMW Spezialmaschinen und Werkzeugbau GmbH & Co. KG** — die **Dossier-Telefonnummer 0395 3698315** gehört **nicht** zu diesem Objekt (dort `phone=+49 395 36709 0`). Ob die ALBA Metall Nord NL Neubrandenburg **einmal** unter dieser Nummer ansässig war und die Nummer inzwischen neu belegt ist, oder ob die Dossier-Adresse von Anfang an auf einen anderen Betrieb zielte, ist **offen**. Die Betreiber-Standortliste **metall.alba.info/unternehmen/standorte/** (live am 31.10.2026, HTTP 200) nennt weiterhin **„13 Treffer zu Ihrer Auswahl“** — **ohne** Neubrandenburg; die Unter-URL https://metall.alba.info/unternehmen/standorte/neubrandenburg liefert HTTP 404. Ein **Schließungs- oder Verlagerungsbeleg** fehlt weiterhin → `status: pruefung` und der Klärfall bleiben **unverändert**. Quelle(n): https://api.openstreetmap.org/api/0.6/way/587299372.json ; https://metall.alba.info/unternehmen/standorte/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Ankauf, Verkauf und Gebühren: Betreiber-Aussagen der ALBA-Gruppe, keine standortscharfen Preise.** **ANKAUF:** „Die Spezialist*innen von ALBA nehmen **auf all unseren Plätzen** alle Arten von **Stahl- und Metallschrott zu Bestpreisen** entgegen und sorgen für deren fachgerechte Verwertung“ (metall.alba.info/ankauf/); die **Sorten** der ALBA-Gruppe sind auf metall.alba.info/ankauf/sorten/ benannt (**Produktionsschrotte, Haushaltsschrott, Industrie- und Abbruchschrotte, Kabelschrotte, Mischschrotte, Vormaterial, Schwere Schere, Legierte Schrotte, Karossen, Aluminium, Kupfer, Messing, Brennschrott, Gusseisen, Moniereisen, Scherenschrott, Stahl- und Metallspäne**) — das deckt die Seed-Angabe „Stahl/NE“ inhaltlich. **Zahlenpreise werden nicht veröffentlicht.** **VERKAUF:** „Verkauf von Nutzmaterial“ ist als ALBA-Leistungsfeld benannt, **ohne** Zahlenpreise. **GEBÜHREN:** **keine** Gebühr beziffert; die Container-Bestellung läuft über **ALBAclick**, **ohne** Preisangabe. Quelle(n): https://metall.alba.info/ankauf/ ; https://metall.alba.info/ankauf/sorten/ ; https://metall.alba.info/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen und Geokodierung.** (a) Die **entscheidende offene Frage** bleibt: Existiert die **ALBA Metall Nord NL Neubrandenburg** an der Kruseshofer Str. 21 noch, und wenn ja, unter welcher Adresse/Telefonnummer? Die **Dossier-Telefonnummer 0395 3698315** ist in OSM **keinem** Objekt zugeordnet; die **Betreiberquelle** führt Neubrandenburg **nicht**. (b) Die **PLZ 17036** und die **Straße Kruseshofer Straße** sind in OSM konsistent zur Dossier-Adresse. (c) **Geokodierungsbedarf:** die Dossier-Adresse **Kruseshofer Str. 21, 17036 Neubrandenburg** ist in OSM belegt (way/42094726 ohne Namens-Tag, way/587299372 mit dem SMW-Betriebsnamen) — es existiert **kein** Objekt, das den **ALBA-Betrieb** nennt; die Koordinate wird hier **nicht** gesetzt (Auftrag dieser Welle: Adressen benennen, nicht selbst geokodieren), der gespeicherte Produktionspunkt **53.5591333/13.2773667** stammt aus einer groben Seed-Koordinate und liegt rund **1,6 km** vom belegten Objektraum entfernt. Quelle(n): https://api.openstreetmap.org/api/0.6/way/42094726.json ; https://metall.alba.info/unternehmen/standorte/ (Abruf 31.10.2026).]
