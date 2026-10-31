---
slug: sl-saarlouis-roden-autoverwertung-both
name: Autoverwertung Both GmbH
trader_type: autoverwertung
state: SL
city: Saarlouis-Roden
street: Güterbahnhofstraße 18
postcode: '66740'
phone: ''
email: ''
opening_hours: ''
website: https://www.autoverwertung-both.de/
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf im Saarland (SL) — Recherche
provenance_ankauf_raw: nur Kfz-Entsorgung
provenance_origin: table
---

# Autoverwertung Both GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Website live (Direktabruf): Autoverwertung Both GmbH, Gueterbahnhofstr. 18, 66740 Saarlouis-Roden, Tel 06831-81 88 5, info@autoverwertung-both.de, Oeffnungszeiten Mo-Fr 09:00-17:00 (Sa n.V.), Leistungen Autoverwertung/Abschleppdienst/Teileshop (eBay-Shop verlinkt, kein unabhaengiger Beleg); Impressum-Routen (/Impressum + /impressum) beide 404 (Impressum-Abruf GESCHEITERT - Owner-Ausnahme nicht anwendbar); Quelle(n): https://www.autoverwertung-both.de/ (eine Quelle)]
- [Recherche 01.10.2026: HR-Gegenbeleg northdata: Autoverwertung Both GmbH, AG Saarbruecken HRB 109799, Adresse Gueterbahnhofstr. 18, D-66740 Saarlouis, GF Daniel Both, Eintragung 17.11.2023, Gegenstand Autoverwertung/Abschleppdienst/Schrott-/Rohstoffhandel/Ersatzteilhandel (aktiv); Quelle(n): https://www.northdata.de/Autoverwertung Both GmbH, Saarlouis (HRB 109799)]
- [Recherche 01.10.2026: Frontmatter-Fills aus Doppelbeleg (Name+Adresse deckungsgleich): street + postcode + website_status aktiv (website aus Seed); Telefon/E-Mail/Zeiten nur Einzelbeleg (Betreiber-Website) -> kein Fill. Quelle(n): siehe beide Quellen oben.]

### Importiert (Seed-Stand 2026-09-30)

- Autoverwertung (kostenlos), Abschleppdienst, Teileshop

### Recherche 31.10.2026

- [Recherche 31.10.2026: OSM-Adressprüfung live am Objekt, Raster bbox=6.7418496,49.3155013,6.7726134,49.3377105 (OSM-Way-BBox des gesamten Straßenverlaufs "Güterbahnhofstraße", way/65621813 + way/183747600, plus 0,010 Grad Rand; in Kacheln abgetastet), Filter addr:street=Güterbahnhofstraße mit addr:housenumber: **13 Objekte** | **13 verschiedene Hausnummern** (roh 1, 3, 11, 12, 13, 14, 15, 15a, 17, 17a, 17b, 18, 21 — keine Bereichsnotation, daher unverändert aufgelöst). Die Dossier-Hausnummer **18 ist erfasst und trägt den Betriebsnamen**: way/354133535 mit `addr:housenumber=18`, `addr:street=Güterbahnhofstraße`, `addr:city=Saarlouis`, `addr:postcode=66740`, `addr:country=DE`, `name=Auto Both`, `shop=car`, `building=yes` — das ist ein **zweiter, vom Betreiber unabhängiger Beleg** für Name und Hausnummer. Der Name ist allerdings als „**Auto Both**" (nicht „Autoverwertung Both GmbH") erfasst. Von den 13 Adressobjekten der Straße trägt genau **1** ein `name=`-Tag — das genannte Objekt. Im gesamten Raster tragen 1305 Objekte `name=` oder `operator=`; Objekte mit Bezug zu **Schrott/Metall/Recycling/Verwertung** sind dort **2** (Kriterium: `shop=gold_buyer` | `amenity=recycling` | `industrial=` | `landuse=industrial` | `recycling:` oder ein Name/Operator mit Schrott-/Recycling-/Verwert-Bezug): way/354133535 `name=Auto Both` (das Dossier-Objekt) und node/10119845258 mit `amenity=recycling`, `operator=EVS`, `recycling:cardboard=yes`, `recycling:paper=yes`, `recycling_type=container`, `opening_hours=Mo-Sa 08:00-20:00` — ein **Wertstoffcontainer** der EVS, nicht der Dossier-Betrieb. Die weiteren im Raster liegenden Wertstoffcontainer (node/10119845260 und node/2187851870 mit `operator=AWO`, node/10117362991 mit `operator=Deutsches Rotes Kreuz`) tragen **keinen** Recycling-Tag und sind **nicht** zu zählen. Quelle(n): https://api.openstreetmap.org/api/0.6/way/354133535.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=6.7418496,49.3155013,6.7726134,49.3377105 (Abruf 31.10.2026)]
- [Recherche 31.10.2026: **Das Impressum ist erreichbar — die 404-Angabe der Vorwelle ist nicht mehr gültig.** Die Seiten liegen unter dem Präfix `/de/`: https://www.autoverwertung-both.de/de/Impressum liefert HTTP 200 und nennt „**Autoverwertung Both GmbH**, Anschrift: **Güterbahnhofstr. 18, 66740 Saarlouis-Roden**, Telefon: 06831-81 88 5, E-Mail: info@autoverwertung-both.de, Internet: www.autoverwertung-both.de, **Inhaber: Herr Daniel Both**, Rechtsform: GmbH, **Kammer: IHK Saarland**, Franz-Josef-Röder-Straße 9, 66119 Saarbrücken, **Umsatzsteuer-ID: DE364337991**, **Handelsregister: Amtsgericht Saarbrücken HRB 109799**" — der HRB-Beleg der Vorwelle (NorthData HRB 109799, GF Daniel Both) ist damit **über die Betreiberseite selbst** bestätigt und die **USt-IdNr.** ist erstmals belegt. Die unter `/AGB` (ohne Präfix) aufgerufene Seite liefert HTTP 302 auf die 404-Seite; die AGB liegen unter **https://www.autoverwertung-both.de/de/AGB**. Die Startseite nennt als Gründungsangabe **1972** („Im Jahr 1972 gründete Hans Jürgen Both die Firma … geführt von den Söhnen Daniel und Benjamin Both in zweiter Generation"), die Startseite nennt dagegen „Mehr als 50 Jahre Erfahrung" — die beiden Angaben sind **nicht** widerspruchsfrei (1972 bis 2026 wären 54 Jahre, die Startseite ist also plausibel; die Formulierung „mehr als 50 Jahre" deckt sich). Die Startseite nennt ferner: „Sie finden uns im **Gewerbegebiet Güterbahnhofstraße** direkt hinter dem Bahnhof Saarlouis." Quelle(n): https://www.autoverwertung-both.de/de/Impressum ; https://www.autoverwertung-both.de/ (Abruf 31.10.2026)]
- [Recherche 31.10.2026: Preise getrennt nach Ankauf, Verkauf und Gebühren. **Ankauf:** Die Seite „Autoverwertung" (https://www.autoverwertung-both.de/de/Autoverwertung) belegt eine **Vergütung nur für Unfallfahrzeuge**: „**Ankauf von Unfallfahrzeugen jeglicher Art** — Handelt es sich um ein jüngeres Unfallfahrzeug mit einem Alter von **maximal 10 Jahren** ist eine **Vergütung** möglich. Das ist jedoch **von Fall zu Fall unterschiedlich**." Ein Euro-Betrag wird **nicht** genannt; die Vergütung ist damit **individuell** und eine Preisliste existiert nicht. Für die reguläre Privatannahme gilt dagegen ausdrücklich **Gebührenfreiheit**: „**Kostenlose Verwertung der Altfahrzeuge** inkl. gesetzlich geforderten Verwertungsnachweis" und „**Kostenloser Abmeldeservice** für Schrottfahrzeuge"; auf der Startseite: „Das Beste daran: Dieser Service ist **kostenlos** für Sie." **Verkauf:** Der Betrieb **verkauft** gebrauchte Kfz-Originalersatzteile über den **eBay-Teileshop** (`ebay.de/str/autoverwertungbothgmbh`); die AGB (https://www.autoverwertung-both.de/de/AGB) nennen die Preisregeln des Teile-Shops: „Unsere Preise verstehen sich **inklusive Umsatzsteuer** und zuzüglich der Kosten für den Versand", die **Versandkosten** sind in einer Tabelle „Versandkosten" unter „Leistungen" einzusehen, für Paketversand gilt Zahlung per **Vorkasse** oder **PayPal** mit Überweisung „**innerhalb von zehn Tagen** nach Vertragsschluss", bei Bar Abholung ist „die Zahlung des Kaufpreises **unmittelbar mit Vertragsschluss** fällig"; „Wir bieten **keine Zahlung per Nachnahme** an"; die gesetzliche Gewährleistung beträgt **12 Monate**, bei Gebrauchtwagen gegenüber Kaufleuten ist sie **ausgeschlossen** (bei Verbrauchern **ein Jahr**); Eigentumsvorbehalt bis zur vollständigen Bezahlung. Die AGB nennen **keine** konkreten Teilpreise — diese stehen nur im eBay-Shop. **Gebühren:** Die **Entgeltefreiheit** der Fahrzeugannahme ist die zentrale Gebührenaussage (siehe oben); ergänzend nennen die AGB, dass die Wiederverwendung von Schrottfahrzeugen „**zu 90 %**" möglich sei („Durch die sortenreine Auftrennung der Rohstoffe können wir bis zu **90 %** wiederverwenden oder recyceln") — eine **Rückgewinnungsquote**, keine Gebühr. Für gewerbliche Kunden werden „Kooperationen zur **Rücknahmestelle** für Altfahrzeugen mit entsprechendem **Vertrag**" angeboten, ohne Preis. Offen bleibt, ob die Entsorgung eines Fahrzeugs ohne Hauptkomponenten (Motor, Getriebe, Katalysator, Batterie, Räder) gegen Gebühr erfolgt — die Betreiberseite sagt dazu **nichts**. Quelle(n): https://www.autoverwertung-both.de/de/Autoverwertung ; https://www.autoverwertung-both.de/de/AGB (Abruf 31.10.2026)]
