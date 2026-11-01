---
slug: hh-altstadt-norddeutsche-edelmetall-scheideanstalt-n
lat: 53.5506822
lon: 9.9873944
name: Norddeutsche Edelmetall Scheideanstalt GmbH (NES GROUP), Niederlassung Hamburg
trader_type: sonstige
state: HH
city: Altstadt
street: Neuer Wall 80
postcode: '20354'
phone: '040 60926890'
email: ''
opening_hours: Mo-Fr 10:00-17:30, nur mit Termin
website: https://scheideanstalt-hamburg.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 3
provenance_ankauf_raw: ja (HH nur mit Termin; Werk Norderstedt)
provenance_origin: table
---

# Norddeutsche Edelmetall Scheideanstalt GmbH (NES GROUP), Niederlassung Hamburg

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Au/Ag/Pt/Pd-Ankauf zu Schmelzkursen, Scheidung, Barren/Halbzeuge, Tagespreis-Tabelle

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2202 berechtigt — lat/lon (Raum Moers/NRW) falsch; Hamburg-Büro per Betreiber-Standortseite: COLLECTION Servicebüro Neuer Wall 80, 20354 Hamburg, Mo-Fr 10:00-17:30 nur mit Anmeldung, Tel. +49(0)40 60926890 → street/postcode/phone/hours korrigiert + website_status aktiv; Werk Norderstedt (Oststraße 128, 22844 Norderstedt, HRB 6761 AG Kiel, register-kongruent per Northdata) in Timeline dokumentiert; Adresse neu → Koordinaten neu geocodieren; Quelle(n): https://scheideanstalt-hamburg.de/impressum/ + Northdata HRB 6761]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Neuer Wall, Hamburg-Altstadt).** Kachelraster über den **gesamten** Straßenverlauf: OSM-Way-BBox aller Ways mit `name=Neuer Wall` (7 Ways) = **53.5497944/9.9861173 bis 53.5528181/9.9932505**, **plus 0,010 Grad Rand** → Raster-BBox **bbox=9.9761173,53.5397944,10.0032505,53.5628181** (13 Kacheln à 0,01 Grad, live über `https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9761173,53.5397944,10.0032505,53.5628181` abgerufen). Zählung getrennt: **92 Adressobjekte / 55 verschiedene Hausnummern** (2, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 18, 19, 20, 22, 24, 25, 26, 30, 31, 32, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 46, 48, 50, 52, 54, 55, 57, 59, 61, 63, 64, 69, 71, 72, 73, 74, 75, 77, 80, 84, 86, 88) auf der Straße, Filter-Key `addr:street` gleich „Neuer Wall“ (Filter-Key `addr:street=Neuer Wall` **mit** `addr:housenumber`; Bereichs-/Sammelnotationen mit der Repo-Referenzregel `osm_street_scan.expand` aufgelöst, u.a. `73-75` → 73/74/75). **Die Dossier-Hausnummer 80 existiert in OSM**: way/57019298 mit `addr:housenumber=80`, `addr:street=Neuer Wall`, `addr:city=Hamburg`, `addr:country=DE`, `addr:postcode=20354`, `building=yes`, `building:levels=7` — das Objekt trägt **weder `name=` noch `operator=`**, ist also ein reiner Adress-/Gebäudepunkt ohne Betriebsbezug. Auch Hausnummer **13** ist belegt (u.a. node/687784842 mit `addr:housenumber=13`, `addr:street=Neuer Wall`, `name=Felix Jud & Co.`, `shop=books`; way/39878473 mit `addr:housenumber=13`, `building=retail`). Im Raster tragen **6181 Objekte** `name=` oder `operator=`; **davon 40 auf der Straße selbst**, **keines** davon nennt die NES GROUP, die Norddeutsche Edelmetall Scheideanstalt oder die COLLECTION. **Positiver Beleg im Raster für ein anderes Edelmetall-Ankaufgeschäft**: node/9283838181 mit `name=philoro Edelmetalle - Filiale Hamburg`, `operator=philoro EDELMETALLE`, `shop=gold_buyer`, `addr:housenumber=77`, `addr:street=Neuer Wall`, `addr:postcode=20354`, `opening_hours=Mo-Fr 10:00-18:00; Sa 10:00-15:00`, `phone=+49 40 181000300`; node/9283838181 trägt ferner `website=https://philoro.de`. Ferner node/11035248065 mit `name=Rheinische Scheidestätte`, `shop=gold_buyer`, `phone=+49 40 248278787`, `email=mailto:info-hamburg@rheinische-scheidestaette.de`; node/11035248065 trägt ferner `website=https://rheinische-scheidestaette.de/unternehmen/filialen/hamburg/` (**beide sind andere Betriebe, nicht die NES**). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9761173,53.5397944,10.0032505,53.5628181 (bbox-Raster, Kacheln live abgerufen) ; https://api.openstreetmap.org/api/0.6/way/57019298.json ; https://api.openstreetmap.org/api/0.6/node/9283838181.json ; https://api.openstreetmap.org/api/0.6/node/11035248065.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Adresskonflikt Neuer Wall 80 gegen Neuer Wall 13 — beide werden aktuell genannt.** Die Betreiberseite nennt an mehreren Stellen **zwei** Hamburger Adressen nebeneinander: die Fußzeile aller Seiten (Impressum, Startseite, „Besuch und Versand“, „Goldankauf Hamburg“) führt als Hamburger Servicebüro **„Neuer Wall 13, in der Passage Alsterarkaden/Neuer Wall, 20354 Hamburg“** mit „Nur mit Terminvereinbarung“ und **Mo-Fr 10:00-17:30**; die Seite „Besuch und Versand“ benennt dagegen ausdrücklich „**Im Service-Büro in der Hamburger City am Neuer Wall 80 bei COLLECTION**“, und die Seite „Goldankauf Hamburg“ schreibt „An unserer bekannten Serviceadresse COLLECTION am Neuen Wall 80“. Die Startseite nennt die Niederlassung „in der Passage Alsterarkaden / Neuer Wall 13“ und im Fließtext „**Neuer Wall Nr. 80**“. Das Dossier-Feld `street: Neuer Wall 80` ist damit **weiterhin eine von zwei auf der Betreiberseite belegten Hamburger Anschriften** und wird **nicht** geändert; die Zuordnung, welche der beiden Nummern die aktuellere bzw. richtige ist, ist aus den Seiten **nicht** entscheidbar. Die **Öffnungszeiten Mo-Fr 10:00-17:30** im Frontmatter stammen aus der Fußzeile zum Standort **Neuer Wall 13**; für **Neuer Wall 80** wird keine eigene Zeit genannt. Das Impressum selbst führt **nur** den Hauptsitz: „Norddeutsche Edelmetall Scheideanstalt GmbH, Gold- und Silberscheideanstalt, Hauptsitz: Materialannahme und Edelmetallwerk, D-22844 Norderstedt – Oststrasse 128, Telefon: +49 (0)40 609 26 89-0, Handelsregister/Registergericht: Amtsgericht Kiel, Registernummer: HRB 6761 KI, USt-IdNr. DE814192085“ — die Hamburger Niederlassung ist **kein eigener Registereintrag**. Das Impressum nennt als E-Mail **`kontakt@norddeutsche-es.de`** und zusätzlich die Domains `norddeutsche-edelmetall.de` und `nes-silbershop.de`; die E-Mail ist auf der Seite per JavaScript maskiert eingebettet, das Feld `email` bleibt deshalb leer. Quelle(n): https://scheideanstalt-hamburg.de/impressum/ ; https://scheideanstalt-hamburg.de/besuch-und-versand/ ; https://scheideanstalt-hamburg.de/goldankauf-hamburg/ ; https://scheideanstalt-hamburg.de/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Preise strikt getrennt: Ankauf, Verkauf, Gebühren.** ANKAUF: die Betreiberseite veröffentlicht eine **Schmelzkurs-Tabelle** („Goldankauf Hamburg – Ankaufspreise für Schmelzmetalle“) mit **Gold 105,17 Euro/g**, **Silber 1.243,80 Euro/kg**, **Platin 34,57 Euro/g**, **Palladium 23,23 Euro/g**; ausdrücklich als **Schmelzkurse (Zahlkurse)** bezeichnet, in denen die **Verarbeitungskosten bereits enthalten** sind und die die **kostenfreie 4-fach Analytik** (Gold/Platin/Palladium/Silber) einschließen. Der **Rechner** auf derselben Seite arbeitet mit den Legierungen 333er/8k, 375er/9k, 416e/10k, 585er/14k, 750er/18k, 833er/20k, 875er/21k, 900er/21,6k, 916,6er/22k, 995er, 999,9/24k und weist ausdrücklich darauf hin, dass Nebenwerte wie Silber, Platin oder Palladium **erst bei der Analytik** auftauchen und **zusätzlich vergütet** werden können. VERKAUF: „In unseren Online-Shops können Sie 24 Std. viele Edelmetalle und Edelmetallprodukte kaufen und sie nur bei Abholung bar zahlen“ — **Barren, Münzen, Halbzeuge, Drähte, Bleche, Granalien**; eine öffentliche **Preisliste für Verkäufe** wird nicht veröffentlicht. Die Feinmetall-Tabelle auf der Seite „Edelmetallpreise“ (VK/AK-Spalten: Gold VK >999 uv **116,92**, AK >999 hf **108,86**, AK >999 nh **106,63**; Silber **1.492,56** / **1.340,54** / **1.312,90** Euro/kg; Platin **43,72** / **38,64** / **36,60** Euro/g; Palladium **29,38** / **25,96** / **24,60** Euro/g) ist nach Betreiberangabe eine **statistische Auflistung der Börsenkurse** („Nur die mit unserer Auftragsannahme bestätigten Ankaufs- oder Verkaufspreise sind … verbindlich“) und damit **kein Standortpreis** — sie wird als **Markt-/Börsenpreis**, nicht als Hamburger Standortkurs geführt. GEBÜHREN: **ausdrücklich gebührenfrei** sind laut Betreiber die **Terminvereinbarung** („Terminvereinbarungen sind kostenfrei“), die **4-fach Analytik** (kostenfrei, in den Schmelzkursen enthalten), die **Beratung/Analysen im Servicebüro** („Analysen und Beratungen … erfolgen kostenfrei und unverbindlich“) sowie die **Versandbox** („Eine sichere kostenfreie Versandbox / tasche bestellen“) und die **Parkmöglichkeit am Werksgelände Norderstedt** („barriere- und kostenfrei“). Für das **Werk Norderstedt** nennt die Seite die Zeiten **Mo 09:00-17:30, Di 09:00-17:30, Mi 09:00-18:30, Do 09:00-17:30, Fr 09:00-16:30** (Terminvereinbarung erbeten). Quelle(n): https://scheideanstalt-hamburg.de/goldankauf-hamburg/ ; https://scheideanstalt-hamburg.de/preisrechner/ ; https://scheideanstalt-hamburg.de/edelmetallpreise/ ; https://scheideanstalt-hamburg.de/besuch-und-versand/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Mindestmengen für die Umformung zu Barren (Bedingung, kein Preis).** Die Seite „Gold Silber Recycling“ nennt für die Umwandlung kundeneigener Edelmetalle zu Barren Mindestmengen: „Scheidegutmengen an Gold müssen dafür ein Mindestfeingoldgewicht von **450g** erreichen. Scheidegutmengen an Silber müssen dafür ein Mindestfeinsilbergewicht von **10Kg** erreichen und den Gesamtfeingehalt von **70%** nicht unterschreiten. Wenn Sie diese Gewichte nicht erreichen, können Sie natürlich auch Ihre Edelmetallwerte in neue Barren verrechnen lassen.“ Quelle(n): https://scheideanstalt-hamburg.de/gold-silber-recycling/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Hausnummern-Gegenrechnung abgeschlossen — die Aufzählung ist bestätigt.** Nachgerechnet über die Zähllogik von `scripts/release/osm_street_scan.py` (Filter-Key `addr:street=Neuer Wall` **mit** `addr:housenumber`, Kachelraster 0,01 Grad über das **vollständige** Raster-BBox **bbox=9.9761173,53.5397944,10.0032505,53.5628181**, 9/9 Teilfenster vollständig verfügbar): **92 Adressobjekte / 55 verschiedene Hausnummern** — **deckungsgleich** mit der Angabe im Recherche-Vermerk oben: jede dort genannte Nummer existiert live, und jede live vorhandene Nummer ist dort genannt. Die **Dossier-Hausnummer 80** ist dabei durch way/57019298 (`addr:housenumber=80`, `addr:postcode=20354`, `addr:city=Hamburg`, **weder `name=` noch `operator=`**) belegt. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9761173,53.5397944,10.0032505,53.5628181 (Raster vollständig, live abgerufen) (Abruf 31.10.2026).]

### Geokodierung 01.11.2026

- [Geokodierung 01.11.2026: **Koordinate korrigiert**: `lat`/`lon` = **53.5506822/9.9873944** (Mittel der eindeutigen
  Knoten von way/57019298, innerhalb der Objekt-BBox). Quelle 1 (Betreiber): Die Betreiber-Website nennt die
  Hamburger Serviceadresse **„Neuer Wall 80"** ausdrücklich als Büro der Scheideanstalt: „Zentral und nah gelegen,
  unser COLLECTION Büro, **Neuer Wall 80**" sowie „An unserer bekannten Serviceadresse **COLLECTION am Neuen Wall
  80**" und „Im Service-Büro in der Hamburger City am **Neuer Wall 80** bei COLLECTION" (live abgerufen 01.11.2026,
  https://scheideanstalt-hamburg.de/goldankauf-hamburg/ ; https://scheideanstalt-hamburg.de/besuch-und-versand/ ;
  https://scheideanstalt-hamburg.de/) — die Dossier-Adresse ist damit über die Betreiberquelle belegt, die
  Dossier-Telefonnummer +49(0)40 60926890 ebenfalls (Zentrale +49 (0)40 609 26 89-0). Quelle 2 (OSM): way/57019298
  (live abgerufen 01.11.2026, https://api.openstreetmap.org/api/0.6/way/57019298/full.json) trägt `addr:housenumber`=80,
  `addr:street`=Neuer Wall, `addr:city`=Hamburg, `addr:postcode`=20354, `building`=yes, `building:levels`=7 — die
  Dossier-Hausnummer **80** und die Dossier-Straße sind objektgenau belegt. Objekt-BBox:
  53.5503632/9.9869158 bis 53.5509259/9.9878774, der Punkt liegt innerhalb. **Qualifizierung (bleibt offen):**
  das Gebäude trägt **weder `name=` noch `operator=`**; die Betreiberseite nennt **neben** Neuer Wall 80 in der
  Fußzeile aller Seiten den Servicebüro-Standort **Neuer Wall 13** (in der Passage Alsterarkaden/Neuer Wall), und ein
  OSM-Raster über Neuer Wall (bbox=9.9761173,53.5397944,10.0032505,53.5628181, live abgerufen 01.11.2026) enthält
  **kein** Objekt mit Bezug zu NES, Norddeutsche Edelmetall Scheideanstalt oder COLLECTION. Gesetzt wird deshalb
  der in OSM belegte **Adressstandort der Dossier-Adresse Neuer Wall 80** — die Wahl zwischen 80 und 13 ist aus den
  Quellen **nicht** entscheidbar und bleibt dokumentiert; das Dossier-Feld `street` bleibt unverändert „Neuer Wall 80",
  `status` **aktiv** bleibt unverändert.]

### Korrektur 01.11.2026

- [Korrektur 01.11.2026: **Telefonnummer in die wählbare Inlandform überführt**: Das Frontmatter führte die Nummer in der gedruckten Auslands-Notation `+49(0)40 60926890`; die Notation `(0)` ist international **nicht wählbar** (die Null ist bereits in der Vorwahl enthalten). Der Wert ist jetzt `040 60926890` — Vorwahl und Teilnehmer durch ein Leerzeichen getrennt, ohne Länderpräfix, mit der inlandüblichen führenden Null. Die Ziffernfolge selbst ist **unverändert**; die Belegquelle steht im Dossier.]
