---
slug: st-kothen-ot-elsdorf-schwind-marko-schrotthandel
lat: 51.7824130
lon: 11.9778684
name: Schwind Marko Schrotthandel
trader_type: schrottplatz
state: ST
city: Köthen OT Elsdorf
street: Ringstr. 4
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Schrotthandel bzw. Entrümpelung in Köthen OT Elsdorf, Ringstr. 4. Die frühere Marko Schwind UG (AG Stendal HRB 8480) wurde liquidiert und am 14.12.2017 im Handelsregister gelöscht; im amtlichen Register der Entsorgungsfachbetriebe (Stand 11.08.2026) ist für Köthen (06366/06369) kein Betrieb Schwind geführt. Eine Fortführung durch eine Einzelperson ist nicht belegt; Annahme, Preise und Gebühren sind nicht belegt.'
dropoff_json: ''
pickup_json: ''
provenance_section: Dessau-Roßlau / Wittenberg / Bitterfeld / Köthen / Bernburg /
  Aschersleben
provenance_ankauf_raw: ja (lt. Verzeichnis, unbestätigt)
provenance_origin: table
---

# Schwind Marko Schrotthandel

## Überblick

Die frühere Marko Schwind UG (AG Stendal HRB 8480) war laut Registerprofil Gegenstand eines Schrotthandels, wurde liquidiert und am 14.12.2017 gelöscht. Ein aktueller 11880-Verzeichniseintrag „Marko Schwind Schrotthandel“ führt Ringstr. 4 und 03496 303294; er beweist weder eine fortgeführte Einzelfirma noch eine Wiederaufnahme nach Löschung der UG. Ein zweiter Eintrag „Schwind Entrümpelungen“ mit abweichender Nummer ist nicht ohne Weiteres derselbe Betrieb. `pruefung` bleibt; keine Schließung des möglichen Einzelbetriebs behauptet. **Ankauf:** keine bestätigte Annahme/Kurse. **Verkauf:** keine Liste. **Gebühren:** keine Tarife. Der bisherige Produktionspunkt lag rund 3,4 km entfernt in Köthen statt in Elsdorf und wurde mit `clear_fields: 'geo'` zurückgezogen, ohne Ersatz.

## Timeline

### Recherche 27.10.2026

- [Recherche 27.10.2026: Amtliches Fachbetrieberegister neu ausgewertet (Fachbetrieberegister der Gemeinsamen Stelle Altfahrzeuge der Länder nach § 7 Abs. 2a AltfahrzeugV und Register der Entsorgungsfachbetriebe nach § 56 KrWG, Auszug mit Stand 11.08.2026, gespiegelt über SchrottRegister): Für **Köthen (06366)** ist **„KER Kaiser Entsorgung & Recycling GmbH“, Am Güterbahnhof 3** geführt, für **Köthen (Anhalt) (06366/06369)** die drei Einträge **„Kremer GmbH“, Ringstraße 1**, **„PreZero Service Köthen GmbH, Betriebshof Logistik“, Pfriemsdorfer Weg 10** und **„PreZero Service Köthen GmbH, Umschlaganlage und Sonderabfallzwischenlager“, Maxdorfer Straße 40**. Ein **„Marko Schwind“/„Schwind“-Eintrag fehlt**; die PLZ der Seed-Adresse Ringstr. 4 (06369) liegt innerhalb der abgedeckten PLZ. Zusammen mit der bereits dokumentierten Löschung der Marko Schwind UG (HRB 8480) am 14.12.2017 und der Tatsache, dass die Register-Suchfunktion kein „Marko Schwind“ mehr kennt, ist damit **kein fortführender Betrieb im amtlichen Register belegt** — das ist kein Nachweis, dass der mögliche Einzelbetrieb nicht mehr arbeitet.]
- [Recherche 27.10.2026: Verzeichnis-Leads bleiben widersprüchlich und sind keine Belege: der 11880-Eintrag „Marko Schwind Schrotthandel“ (Ringstr. 4, 03496 303294, zusätzlich 0177 4036272) und der Örtliche-Eintrag „Schwind Entrümpelungen“ (Ringstr. 4, 0177 3361329) sind Aggregatoren; die unterschiedlichen Mobilnummern bleiben ungeklärt. Firmensuche (firmenschau.com, 27.10.2026) nach „Marko Schwind Köthen“ ohne Treffer.]
- [Recherche 27.10.2026: GEO-Korrektur. Die read-only-Produktionszeile enthielt den Punkt **51.7519/11.9709**; die OSM/Nominatim-Rückabfrage löst ihn auf **„Wallstraße 58/59, 06366 Köthen (Anhalt)“** auf — also **Köthen proper**, während die Zeile ausdrücklich **„Köthen OT Elsdorf“, Ringstr. 4 (06369 Elsdorf)** führt; der Adresspunkt „Ringstraße 4, 06369 Elsdorf“ liegt bei **51.7824130/11.9778684**, rund **3,4 km Luftlinie** entfernt. Da die Zeile eine eigene Ortsangabe trägt, die dem gespeicherten Punkt widerspricht, wird der Punkt mit `clear_fields: 'geo'` **zurückgezogen**; eine Ersatzkoordinate wird **nicht** gesetzt, weil die Hausnummer nur über Aggregator-Leads belegt ist (die Registerpublikation zur UG nennt „06369 Köthen-Elsdorf“, redigiert die Straße jedoch) und die Zwei-Quellen-Regel für die Anschrift damit nicht erfüllt ist.]
- [Preise/Ankauf/Verkauf/Gebühren 27.10.2026: **ANKAUF:** keine bestätigte Annahme und keine Kurse. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Tarife. Status bleibt `pruefung`; das ist **kein** festgestellter Schließungsstatus des möglichen Nachfolgebetriebs. Quellen: https://schrottregister.pages.dev/ort-kothen ; https://schrottregister.pages.dev/ort-kothen-anhalt ; https://www.firmenschau.com/suche?q=Marko+Schwind+K%C3%B6then ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&zoom=18&lat=51.7519&lon=11.9709&addressdetails=1 ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=Ringstra%C3%9Fe+4%2C+06369+Elsdorf&addressdetails=1 ; read-only `public.db` (27.10.2026).]

### Recherche 24.10.2026 - Welle 06:00 / Shard C

- [Recherche 24.10.2026: Neue exakte Suche brachte keine Betreiberquelle; der 11880-Eintrag nennt weiter Schrotthandel/Altmetall und 03496 303294 an Ringstr. 4, bleibt aber ein Aggregator-Lead. Die Registerhistorie zur Marko Schwind UG endet mit Löschung 14.12.2017; der Suchtreffer beweist keine Fortführung durch eine Einzelperson. Abweichende „Schwind Entrümpelungen“-Kontakte nicht zusammengeführt. ANKAUF: keine bestätigten Kurse; VERKAUF: keine Preisliste; GEBÜHREN: keine Tarife. Keine Fills/GEO; Status pruefung ist kein festgestellter Schließungsstatus des möglichen Nachfolgebetriebs. Quellen/Leads: https://www.11880.com/branchenbuch/elsdorf-bei-koethen/060440090B48384962/marko-schwind-schrotthandel.html ; https://www.northdata.de/Marko+Schwind+UG,+K%C3%B6then/Amtsgericht+Stendal+HRB+8480 ; https://www.dasoertliche.de/Themen/Schwind-Entr%C3%BCmpelungen-Elsdorf-Elsdorf-Ringstr .]

### Recherche 05.10.2026

- [Recherche 05.10.2026: Neue qualifizierende Registerspur statt Wiederholung der Verzeichnisrecherche vom 03.10.: Northdata-Direktseite HTTP 200 führt Marko Schwind UG, AG Stendal HRB 8480, Köthen, Gegenstand Schrotthandel/Hausmeisterdienste/Arbeitsvermittlung; Liquidation 25.06.2012, Insolvenz 2014, eingestelltes Verfahren 02.05.2017 und Löschung 14.12.2017. Eintragungspublikation einzeln live lesbar, 06369 Köthen-Elsdorf, Straße dort redigiert; Löschungspublikation nur Premium-Hinweis, keine Volltexteinsicht behauptet. Das ist keine automatisch bewiesene Identität mit dem möglicherweise einzeln fortgeführten Seed-Betrieb; Quelle(n): https://www.northdata.de/Marko+Schwind+UG,+K%C3%B6then/Amtsgericht+Stendal+HRB+8480 + https://www.northdata.de/?id=7808767 + https://www.northdata.de/?id=4267203614]
- [Recherche 05.10.2026: Firmenschau-Direktabruf nennt die UG mit HRB 8480, Ringstraße 4, 06369 Köthen-Elsdorf und 03496 303294; nur Aggregator-Lead, kein unabhängiger Nachfolgebeweis und kein aktueller Aktivbeleg. Verbindung zur Schrotthandel-/Entrümpelungs-Doppelspur mit unterschiedlichen Mobilnummern aus 03.10. bleibt offen. Kein aktueller Betreiber-Auftritt/Impressum für den tiefen Unterseitenabruf verifiziert; Suchdienst teilweise ohne passende Ergebnisse, DuckDuckGo CAPTCHA. Keine Preise gefunden, keine Kontaktdaten/Postleitzahl ergänzt, status pruefung beibehalten. Owner benötigt aktuellen Gewerbe-/Nachfolgerbeleg, nicht bloß Gleichheit von Adresse und Name; Quelle(n): https://firmenschau.com/firmen/czo3OiIyNzgxMzcyIjs=/ + https://www.northdata.de/Marko+Schwind+UG,+K%C3%B6then/Amtsgericht+Stendal+HRB+8480 + https://html.duckduckgo.com/html/?q=%22Marko%20Schwind%22%20Schrotthandel (CAPTCHA am 05.10.2026)]

### Importiert (Seed-Stand 2026-09-30)

- Schrotthandel (lt. schrottplatz-info)

### Recherche 03.10.2026

- [Recherche 03.10.2026: 11880-Detailseite führt „Marko Schwind Schrotthandel“ mit Altmetallrecycling/Altmaterialhandel, nennt in der FAQ Ringstr. 4, 06369 Elsdorf, und führt Tel. 03496 303294 sowie zusätzlich 0177 4036272; E-Mail/Website/Öffnungszeiten fehlen. Das Örtliche führt am selben Haus „Schwind Entrümpelungen“, Ringstr. 4, 06369, Tel. 0177 3361329. Beides sind Aggregator-Leads, keine Betreiber-Primärquelle und trotz gleicher Adresse/Namensbestandteil kein unabhängiger Identitäts-/Merge-Beleg; Kontaktzuordnung und Verhältnis beider Einträge bleiben offen. Keine Postleitzahl, Telefonnummer oder Leistung ergänzt/korrigiert; website_status mangels belegter Betreiber-Domain auf unbekannt gesetzt, status pruefung bleibt; keine Schließung belegt. Quelle(n): https://www.11880.com/branchenbuch/elsdorf-bei-koethen/060440090B48384962/marko-schwind-schrotthandel.html + https://www.dasoertliche.de/Themen/Schwind-Entr%C3%BCmpelungen-Elsdorf-Elsdorf-Ringstr (beide Verzeichnis-Leads, kein Identitätsbeleg).]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **51.7824130/11.9778684** (Mittelwert der **1**
  eindeutigen Knoten des Objekts **node/5121795516** - ein Objektknoten, der Mittelwert ist damit deckungsgleich mit dem
  Knoten und liegt innerhalb der Objekt-BBox 11.9778684/51.7824130 bis 11.9778684/51.7824130). Quelle 1 (OSM):
  **node/5121795516** trägt live (abgerufen am 04.11.2026 über
  `https://api.openstreetmap.org/api/0.6/node/5121795516.json`, HTTP 200) `addr:street`=Ringstraße,
  `addr:housenumber`=**4**, `addr:city`=**Elsdorf**, `addr:postcode`=**06369**, `addr:country`=DE - die Dossier-Adresse
  existiert damit **unabhängig** in OSM und **straßengenau** (PLZ 06369 = Elsdorf bei Köthen). Das Objekt trägt **weder**
  `name=` **noch** `operator=` und **keinen** Branchentag. **Fensterdisziplin:** das am 04.11.2026 live abgerufene
  Kachelraster über **Köthen-Elsdorf** (**11.9600/51.7700 bis 12.0000/51.8000**, **12 Kacheln von je 0,01 Grad**,
  **0 Kachel-Fehler**) enthält die gesuchte Straße tatsächlich: die **Ringstraße in 06369 Elsdorf** ist im Raster mit
  **sieben Straßenobjekten** (way/124722540, way/352172025, way/352172805, way/353289758, way/1545410345,
  way/1545410346, way/1545410354) und den Hausnummern **1** (node/5121795518), **3** (node/5121795517), **4**
  (node/5121795516), **5**
  (node/5121795515), **6** (node/5121795514), **7** (node/5121795513), **8** (node/5121795512), **9**
  (node/5121795511), **10** (node/5121795510), **11** (node/5121795509), **12** (node/5121795508) und **13**
  (node/5120128902) vertreten - die **Dossier-Hausnummer 4 existiert** und ist im Raster das einzige Objekt dieser
  Nummer auf dieser Straße; die gleichnamige Ringstraße in **Aken (Elbe) 06385** (u. a. node/4776512942 „4“) und eine
  weitere Ringstraße bei 12.016/51.799 liegen außerhalb der Ortsgrenze und sind **andere** Straßen - die Filterung
  über `addr:city`=Elsdorf bzw. `addr:postcode`=06369 trennt die drei sauber. Quelle 2 (Betreiber/Verzeichnis, im
  Dossier bereits zitiert, live nachgelesen am
  04.11.2026): die **11880**-Detailseite führt **„Marko Schwind Schrotthandel“** mit den Sparten Altmetallrecycling
  und Altmaterialhandel an **Ringstr. 4, 06369 Elsdorf bei Köthen**, Telefon **(03496) 303294** (sowie 0177
  4036272); die dort hinterlegte Geokoordinate **11.977838/51.782424** liegt rund **5 m** vom OSM-Objekt entfernt und
  bestätigt denselben Standort. Das im Dossier zitierte **Das Örtliche** führt am selben Haus **„Schwind
  Entrümpelungen“, Ringstr. 4, 06369** (Tel. 0177 3361329) und das im Dossier zitierte **Creditreform-/NorthData-Profil**
  der früheren **Marko Schwind UG (AG Stendal HRB 8480)** nennt ebenfalls „06369 Köthen-Elsdorf“ und Ringstraße 4
  (Straße in der Registerpublikation redigiert). **Qualifizierung (wichtig):** die Koordinate markiert die
  **Anschrift, die die Verzeichnisse diesem Seed-Eintrag nennen**; die **Marko Schwind UG ist laut Register am
  14.12.2017 gelöscht**, eine Fortführung durch eine Einzelperson ist **nicht** belegt, ein **Betreiberauftritt**
  existiert nicht und im amtlichen Fachbetrieberegister ist für Köthen **kein** Betrieb „Schwind“ geführt. Die
  Koordinate ist deshalb **kein** Beleg für einen aktiven Schrottplatz und **kein** Beleg für Fortbestand oder
  Schließung; `street`, `postcode`, `phone`, `website`, `website_status`, `description` und `status: pruefung` bleiben
  unverändert. `clear_fields: 'geo'` **entfällt**, weil die Koordinate nun belegt vorliegt und der Seed `geo` neben
  belegtem `lat`/`lon` ablehnt. Quelle(n): https://api.openstreetmap.org/api/0.6/node/5121795516.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=11.9600,51.7700,12.0000,51.8000 ;
  https://www.11880.com/branchenbuch/elsdorf-bei-koethen/060440090B48384962/marko-schwind-schrotthandel.html ;
  https://www.dasoertliche.de/Themen/Schwind-Entr%C3%BCmpelungen-Elsdorf-Elsdorf-Ringstr (Abruf 04.11.2026).]
