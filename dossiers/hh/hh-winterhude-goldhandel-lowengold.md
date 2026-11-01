---
slug: hh-winterhude-goldhandel-lowengold
name: Goldhandel Löwengold
trader_type: sonstige
state: HH
city: Winterhude
street: Grasweg 8
postcode: '22299'
phone: 040/ 53024720
email: info@loewengold.eu
opening_hours: Mo-Fr 12:00-17:00
website: https://goldwert-ankauf.de/loewengold/
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Goldhandel Löwengold

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Edelmetall (Zahngold/Altgold/Silber/Münzen)
- Adresse: Winterhude, Grasweg 8, 22299
- Adressbeleg: goldwert-ankauf.de/loewengold

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Grasweg, Hamburg-Winterhude).** Kachelraster über den **gesamten** Straßenverlauf: OSM-Way-BBox aller Ways mit `name=Grasweg` (6 Ways) = **53.5925947/10.002191 bis 53.5935332/10.0110317**, **plus 0,010 Grad Rand** → Raster-BBox **bbox=9.9921910,53.5825947,10.0210317,53.6035332** (live über `https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9921910,53.5825947,10.0210317,53.6035332`, Kacheln à 0,01 Grad). Zählung getrennt: **41 Adressobjekte / 46 verschiedene Hausnummern** (1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13, 14, 15, 19, 21, 30, 32, 34, 36, 38, 38A, 38b, 42, 44, 46, 48, 50, 52, 54, 54a, 54b, 56, 58, 60, 62, 63, 64, 65, 66, 70, 72, 73, 74, 75, 76, 17a,b) auf der Straße, Filter-Key `addr:street` gleich „Grasweg“ (Filter-Key `addr:street=Grasweg` **mit** `addr:housenumber`; Bereichs- und Sammelnotationen mit der Repo-Referenzregel `osm_street_scan.expand` aufgelöst, u.a. `13-15` → 13/14/15, `62-66` → 62/63/64/65/66, `72-76` → 72/73/74/75/76; die **Sammelnotation `17a,b`** wird von dieser Regel **nicht** aufgeteilt und steht deshalb **als ein Wert** (`17a,b`) **am Ende** der Liste, ebenso `38A` und `38b` als getrennte Werte). **Die Dossier-Hausnummer 8 existiert in OSM**: way/100495283 mit `addr:housenumber=8`, `addr:street=Grasweg`, `addr:city=Hamburg`, `addr:postcode=22299`, `building=apartments`, `building:levels=5`, `roof:levels=1`, `roof:shape=saltbox` — das Objekt trägt **weder `name=` noch `operator=`** und ist ein **Wohngebäude** (`building=apartments`), also **kein** Betriebsobjekt; die Dossier-Adresse ist damit **adresslich belegt, aber nicht betrieblich**. Auf der Straße tragen **3 Objekte** `name=`/`operator=` — **keines** davon nennt Löwengold, Lev Gehlmann oder einen Goldhandel: way/59136151 mit `name=Kita Grasweg`, `operator=Elbkinder Vereinigung Hamburger Kitas gGmbH`, `addr:housenumber=21`; node/4424769742 mit `name=Praxis für ganzheitliche Medizin`, `addr:housenumber=3`; way/100495259 mit `name=Headshop Grasweg`, `addr:housenumber=5`, `shop=gift`. Im Raster tragen **1712 Objekte** `name=` oder `operator=`; ein Objekt mit `name=Gold und Silber Ankauf Nord`, `shop=gold_buyer` (node/10788225888) liegt im Raster, trägt aber **keine** `addr:`-Tags und ist damit **nicht** der Dossier-Standort Grasweg 8. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9921910,53.5825947,10.0210317,53.6035332 (bbox-Raster, live abgerufen) ; https://api.openstreetmap.org/api/0.6/way/100495283.json ; https://api.openstreetmap.org/api/0.6/way/59136151.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Betreiberidentität und Kontaktdaten jetzt zweifach belegt.** Das **Impressum der Betreiberseite** (https://goldwert-ankauf.de/loewengold-impressum/, HTTP 200) nennt „Angaben gemäß § 5 DDG: **Lev Gehlmann e.K. (GOLDHANDEL LÖWENGOLD), Grasweg 8, 22299 Hamburg**, Geschäftsleitung: Lev Gehlmann, Eingetragener Kaufmann, **Handelsregister A des Amtsgerichts Hamburg HRA 133093**, USt-IdNr. **DE455637967**“ und als Telefon **+49 (0) 40 53024720** — deckungsgleich mit dem Dossier-Telefon **040/ 53024720**. Die **E-Mail** ist auf der Seite per Cloudflare-E-Mail-Schutz kodiert; die Dekodierung des `data-cfemail`-Attributs ergibt **info@loewengold.eu**, identisch auf Filial- und Impressumsseite. **Zweiter, unabhängiger Beleg**: das **Branchenbuch der Stadt Hamburg** (Eintragsnummer 101306036, „Letzte Aktualisierung: 27.07.2026“) führt **Grasweg 8, 22299 Hamburg (Winterhude)**, Telefon **040 / 530 247 20**, E-Mail **info@loewengold.eu** und die Zeiten **Mo-Fr 12:00-17:00, Sa/So geschlossen** — deckungsgleich mit den Dossier-Öffnungszeiten **Mo-Fr 12:00-17:00**. Damit sind `email` (**info@loewengold.eu**), `opening_hours` (**Mo-Fr 12:00-17:00**) und die Rechtsform (**e.K. des Einzelunternehmens Lev Gehlmann, HRA 133093 AG Hamburg**) erstmals **zweifach** belegt; die Felder werden entsprechend gefüllt. Quelle(n): https://goldwert-ankauf.de/loewengold-impressum/ ; https://www.hamburg.de/branchenbuch/hamburg/eintrag/101306036/ ; https://goldwert-ankauf.de/loewengold/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Preise strikt getrennt: Ankauf, Verkauf, Gebühren.** ANKAUF: die Filialseite wirbt mit „**faire, aktuelle Ankaufspreise**“ und „Auf Basis des **tagesaktuellen Edelmetallkurses** erhalten Sie sofort ein faires Angebot“; die verkauften Sorten sind **Gold, Silber, Platin, Palladium, Altgold, Zahngold, Schmuck, Barren und Münzen** (teils unterteilt in Schmuck/Barren/Münzen). **Zahlen-Ankaufpreise werden nicht veröffentlicht** — es wird der Tageskurs des Metalls zugrunde gelegt, ein Kurs je Legierung/Feingehalt wird **nicht** genannt. VERKAUF: **kein** Verkauf von Edelmetallen durch den Betrieb belegt; die Seite beschreibt ausschließlich den **Ankauf** durch den Kunden („Sie möchten Gold zu Geld machen“, „Wir kaufen Ihre Edelmetalle … an“). GEBÜHREN: **ausdrücklich gebührenfrei** ist die **Bewertung**: „Die Bewertung ist **kostenlos** und völlig unverbindlich: Sie entscheiden in Ruhe, ob Sie verkaufen möchten.“ Ebenfalls kostenfrei ist die **Sofortauszahlung** („Sofort-Auszahlung: Überweisung oder Bar“); **Gebühren** für Bewertung, Transport oder Auszahlung werden nicht genannt. Das Parken wird ausdrücklich als teilweise **gebührenpflichtig** beschrieben („Im Grasweg und den umliegenden Nebenstraßen … gibt es kostenlose und gebührenpflichtige Parkplätze“; Parkhaus CONTIPARK Tiefgarage Forum Winterhude) — das ist eine **Parkplatz-Situation am Standort**, **keine** Gebühr des Betriebes. Quelle(n): https://goldwert-ankauf.de/loewengold/ (Abruf 31.10.2026).]

- [Korrektur 31.10.2026: `email` erstmals gefüllt (**info@loewengold.eu**) und `website_status` auf **aktiv** gesetzt. Die E-Mail ist **zweifach** belegt: (1) Impressum der Betreiberseite `goldwert-ankauf.de/loewengold-impressum/` — dort per Cloudflare-E-Mail-Schutz kodiert, Dekodierung des `data-cfemail`-Attributs ergibt **info@loewengold.eu**; (2) Branchenbuch der Stadt Hamburg, Eintragsnummer 101306036 (Letzte Aktualisierung 27.07.2026) — nennt E-Mail **info@loewengold.eu**, Telefon **040 / 530 247 20**, Grasweg 8, 22299 Hamburg (Winterhude) und die Zeiten **Mo-Fr 12:00-17:00**. `website_status` war im Seed leer; die Filialseite und das Impressum liefern beide HTTP 200, das Impressum nennt Betreibername **Lev Gehlmann e.K. (GOLDHANDEL LÖWENGOLD)**, **HRA 133093 AG Hamburg**, USt-IdNr. DE455637967 — Website **aktiv**. `opening_hours` (Mo-Fr 12:00-17:00) und `phone` (040/ 53024720) bleiben unverändert, beide sind jetzt ebenfalls zweifach belegt. Quelle(n): https://goldwert-ankauf.de/loewengold-impressum/ ; https://goldwert-ankauf.de/loewengold/ ; https://www.hamburg.de/branchenbuch/hamburg/eintrag/101306036/ (Abruf 31.10.2026)]
- [Recherche 31.10.2026: **Hausnummern-Gegenrechnung abgeschlossen — die Aufzählung ist bestätigt.** Nachgerechnet über die Zähllogik von `scripts/release/osm_street_scan.py` (Filter-Key `addr:street=Grasweg` **mit** `addr:housenumber`, Kachelraster 0,01 Grad über das **vollständige** Raster-BBox **bbox=9.9921910,53.5825947,10.0210317,53.6035332**, 9/9 Teilfenster vollständig verfügbar): **41 Adressobjekte / 46 verschiedene Hausnummern** — **deckungsgleich** mit der Angabe im Recherche-Vermerk oben: jede dort genannte Nummer existiert live, und jede live vorhandene Nummer ist dort genannt. Die **Dossier-Hausnummer 8** ist dabei durch way/100495283 (`addr:housenumber=8`, `addr:postcode=22299`, `addr:city=Hamburg`, **weder `name=` noch `operator=`**) belegt. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9921910,53.5825947,10.0210317,53.6035332 (Raster vollständig, live abgerufen) (Abruf 31.10.2026).]
