---
slug: nw-solingen-ayaz-metallhandel
name: Ayaz Metallhandel
trader_type: metallhaendler
state: NW
city: Solingen
street: Junkerstr. 32
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: http://www.ayaz-metallhandel.de
website_status: tot
status: pruefung
description: 'Metallhandel in Solingen-Merscheid (Junkerstr. 32): Annahme von Metallschrott (Kupfer, Aluminium, Edelstahl, Messing, Zinn/Zink, Blei) mit Containerdienst sowie Demontage und Firmenauflösungen. Betrieb und Erreichbarkeit sind derzeit nur ueber Verzeichniseintraege belegt - die eigene Domain ist nicht mehr in Betrieb.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Ayaz Metallhandel

## Überblick

Metallhandel in Solingen-Merscheid, Junkerstr. 32. Leistungen laut Verzeichniseinträgen: Containerdienst, Annahme von Metallschrott, Demontage, Firmenauflösungen; angenommene Sorten: Kupfer, Aluminium, Edelstahl, Messing, Zinn/Zink, Blei. **Beleglage schwach:** die eigene Domain `ayaz-metallhandel.de` ist nicht mehr in Betrieb (DNS zeigt auf den zu `scheideanstalt.de` weiterleitenden Namen), ein aktuelles Betreiber-Impressum existiert nicht; Telefon, E-Mail und Öffnungszeiten stammen ausschließlich aus Verzeichnissen (Aggregatoren) und sind daher nicht in die Daten übernommen. Postleitzahl (42699) und Telefon (0212 2218422) bleiben unbelegt - Nachprüfung offen.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Metallhandel (Junkerstr. 32), nur schrottradar-Profil

### Recherche 29.10.2026

- [Recherche 29.10.2026: **Die Domain ist weiterhin tot, jetzt mit klarem DNS-Beleg** - `ayaz-metallhandel.de` ist im DNS ein CNAME auf `schrott-koenig.de` (35.156.129.40), und diese Domain liefert HTTP 301 auf `https://www.scheideanstalt.de/` (fremde Gesellschaft). `http://www.ayaz-metallhandel.de/` und `https://www.ayaz-metallhandel.de/` antworten deshalb beide mit **keinem Inhalt** (curl Exit 000), `website_status: tot` ist damit bestaetigt und bleibt. **Verzeichnis-Angaben (Aggregatoren = Leads, keine Belege)**: „Ayaz Metallhandel, Junkerstr. 32, 42699 Solingen (Merscheid)", Telefon 0212-2218422, Fax 0212-2218467, E-Mail `info@ayaz-metallhandel.de` (Cylex/11880/branchen-info) bzw. `ayaz-metallhandel@gmx.de` (ein Places-Eintrag) - **zwei sich widersprechende E-Mail-Adressen**, beide unbelegt; Oeffnungszeiten „Mo-Fr 09:00-17:00, Sa nach Terminabsprache" (Cylex, Finde-Offen, branchen-info) gegen „Mo 09:00-16:00 ... Sa 09:00-12:00" (metallbauer.biz) - **widerspruechlich**. **Konsequenz**: Telefon, E-Mail, Postleitzahl und Oeffnungszeiten werden **nicht** in die Frontmatter uebernommen; `postcode` bleibt leer (42699 ist nur Aggregator-Angabe) - **Geokodierungsbedarf: „Junkerstraße 32, 42699 Solingen" muss am Objekt geprueft werden**. Das Dossier-`street` „Junkerstr. 32" bleibt unveraendert, weil es aus dem Import stammt und die Strassenform bestaetigt ist, der Ortsteil Merscheid folgt aus den Aggregatoren und wird nicht behauptet. **ANKAUF**: „Kupfer, Aluminium, Edelstahl, Messing, Zinn und Zink, Blei" (Verzeichnisangabe, Lead); ein Nutzer-Review nennt, dass fuer Altschrott „etwas Geld" bekommen wurde - **keine Preisangabe, kein Preisbeleg**. **VERKAUF**: kein Beleg. **GEBUEHREN**: kein Beleg (Containerdienst nur als Leistungsname). **Offen**: ob der Betrieb 2026 noch existiert (die letzte belastbare Spur ist der Verzeichniseintrag mit Aktualisierung 17.09.2026 bzw. 29.07.2026; eine eigene, lesbare Website gibt es nicht, und das Web-Archiv war am 29.10.2026 nur eingeschraenkt abfragbar), sowie die belastbare Bestaetigung von Adresse, Telefon und Sorten. Quelle(n): http://www.ayaz-metallhandel.de/ und https://www.ayaz-metallhandel.de/ (Abruf 29.10.2026, kein Inhalt); DNS-Abfrage ayaz-metallhandel.de -> schrott-koenig.de (Abruf 29.10.2026); https://web2.cylex.de/firma-home/ayaz-metallhandel-6525592.html ; https://solingen.branchen-info.net/fp_3951495.php ; https://www.11880.com/branchenbuch/solingen/ (Abruf 29.10.2026, Aggregator-Lead).]

- [Recherche 29.10.2026 (OSM-Nachtrag): **Die Adresse ist als Industrieadresse in OSM erfasst, der Betrieb aber nicht** — Objekt **way 133633639**: `addr:housenumber=32`, `addr:street=Junkerstraße`, `addr:city=Solingen`, `addr:postcode=42699`, **`building=industrial`**, ohne `operator`-, `name`-, `industrial`- oder `landuse`-Tag. Der Hausnummernraster über die Ausdehnung der Junkerstraße (3 Strassen-Ways, Ausdehnung lon 7.019825..7.021868 / lat 51.160438..51.163494, Rasterfenster 7.019025/51.159938/7.022668/51.163994, Ortsteile Ohligs/Aufderhöhe/Merscheid, **42699 Solingen**) zählt **45 Objekte mit `addr:housenumber`**, darunter die Nummern 30, **32**, 34, 35, 36, 37, 38, 39 (u. a.). **Bewertung**: Die Hausnummer 32 existiert in OSM und ist als **Industriegebäude** getaggt — das ist mit einem Schrottbetrieb vereinbar, belegt den Betrieb aber **nicht** (kein Betreiber- oder Branchen-Tag). Die PLZ **42699** ist damit OSM-seitig belegt, bleibt im Frontmatter aber **leer**: OSM und die Verzeichniseintraege sind keine zwei unabhaengigen Belege (Aggregatoren sind Leads), und der Betrieb selbst ist ohne eigene, lesbare Website nicht verifizierbar. **Geokodierungsbedarf (Owner-Gate)**: „Junkerstraße 32, 42699 Solingen" — der Kandidat ist der OSM-Way **133633639** (`building=industrial`); vor einer Geokodierung ist zu klaeren, ob der Betrieb 2026 noch existiert und ob er genau dort sitzt. Quelle(n): https://api.openstreetmap.org/api/0.6/way/133633639/full (Abruf 29.10.2026); OSM-API-Abfrage `api.openstreetmap.org/api/0.6/map?bbox=` über die Ausdehnung der Junkerstraße (Abruf 29.10.2026); https://photon.komoot.io/api/?q=Junkerstra%C3%9Fe+Solingen (Abruf 29.10.2026).]

- [Recherche 29.10.2026 (GEO-Abgleich): **Der in der Produktionsdatenbank gespeicherte Punkt stimmt nicht mit der belegten Adresse ueberein** — gespeichert `51.1686125 / 7.0578875`; Rueckwaertsgeokodierung dieses Punktes (Photon-Reverse ueber OSM) ergibt `Untengönrather Straße, 42655 Solingen (Solingen-Mitte), OSM way 26848749`. Das in OSM gefuehrte Adressobjekt der belegten Anschrift liegt `2.622 m` vom gespeicherten Punkt entfernt: OSM way 133633639 (`addr:housenumber=32`, `addr:street=Junkerstraße`, `addr:city=Solingen`, `addr:postcode=42699`, `building=industrial`). **Konsequenz**: der gespeicherte Punkt wird hier **nicht** automatisch zurueckgezogen — die Auswahl der `clear_fields`-Direktive bzw. einer Ersatzkoordinate ist eine Owner-Entscheidung, und fuer eine **neue** Koordinate liegen in diesem Dossier nur **eine** Positionsquelle (OSM) vor, waehrend die Regel eine zweite unabhaengige Quelle verlangt. **Keine Koordinate gesetzt.** Der gespeicherte Punkt liegt zudem im **falschen Postleitzahlbereich** (42655 statt 42699). Quelle(n): Photon-Reverse (https://photon.komoot.io/reverse, Abruf 29.10.2026); OSM-API-Abfrage `api.openstreetmap.org/api/0.6/way/<id>` bzw. `/node/<id>` (Abruf 29.10.2026); Haversine-Berechnung (R = 6371 km).]
