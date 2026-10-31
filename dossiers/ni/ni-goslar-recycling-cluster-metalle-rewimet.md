---
slug: ni-goslar-recycling-cluster-metalle-rewimet
name: Recycling-Cluster Metalle / REWIMET
trader_type: sonstige
state: NI
city: Goslar
street: Klubgartenstr. 5
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Innerer Harz / Harzrand (neu)
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# Recycling-Cluster Metalle / REWIMET

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 03.10.2026 (Feedback 4461, erneute Tiefenprüfung)

- [Recherche 03.10.2026: Betreiber-Root, Impressum, Über uns, FAQ und komplette Mitgliederseite einzeln geprüft: REWIMET e.V., Vereinsregister 200914 AG Braunschweig, Netzwerk mit Unternehmens-, Forschungs- und kommunalen Mitgliedern. Recycling-/Ankaufsleistungen einzelner Mitglieder sind nicht eigene Händlerleistungen des Vereins. Frühere Note mit Im Schleeke 50 als Vereinsadresse korrigiert: Impressum führt dort Metalogie GmbH unter Konzeptionierung und technische Realisierung; Geschäftsstelle im Footer Leibnizstraße 23, 38678 Clausthal-Zellerfeld. Daraus weder Umzug des unklaren Goslar-Händlers noch Händleridentität ableiten. Betreiberseiten zählen als eine Quelle; neue externe Suche ohne verwertbaren Treffer bzw. Bot-Challenge. Starke Einzelquelle genügt hier zur Abgrenzung des Vereins, nicht zum Füllen eines unbewiesenen Schrotthändlers; Quelle(n): https://rewimet.de/ | https://rewimet.de/impressum | https://rewimet.de/ueber-uns | https://rewimet.de/mission/faq | https://rewimet.de/ueber-uns/mitglieder]
- [Recherche 03.10.2026: Feedbacks technischer Widerspruch bestätigt: Dossier-Website bereits leer, Prod weiterhin https://rewimet.de. website_status auf unbekannt normalisiert, status pruefung und leere Website bleiben; Domain selbst ist erreichbar, aber nicht als eigene Ankaufstelle verifiziert. keep bei leerer Seed-Website erhält den Prod-Wert auch nach erneutem Seedlauf; keine erneute Entfernung behauptet und kein direkter DB-Write; Quelle(n): read-only Prod-Abfrage slug ni-goslar-recycling-cluster-metalle-rewimet | crates/ingestion/src/seed_traders.rs]

### Importiert (Seed-Stand 2026-09-30)

- Metalle/Demontage (Harz)
- urspr. Website-Angabe: keine (falsche Website rewimet.de entfernt, 29.09.2026 — REWIMET e.V., Geschäftsstelle Clausthal-Zellerfeld, kein Händler)

### Recherche 01.10.2026

- [Audit-2 2396 01.10.2026: Audit-2 Feedback 2396 bestaetigt: DB-Website https://rewimet.de gehoert REWIMET e.V. (Recycling-Cluster wirtschaftsstrategische Metalle) - Netzwerk aus Unternehmen/Forschung/Gebietskoerperschaften, Vereinsregister 200914 AG Braunschweig, Im Schleeke 50, 38642 Goslar; kein Ankauf, keine Preise, keine Annahme; Quelle(n): https://rewimet.de Home + Impressum, 01.10.2026]
- [Audit-2 2396 01.10.2026: Klaerfall: keine Frontmatter-Aenderung (Verein, kein Haendler)]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprueft (Klubgartenstraße, Goslar).** Raster **10.4170/51.9060 bis 10.4260/51.9140** (Fenster als Koordinatenangabe; ausfuehrbare URL: https://api.openstreetmap.org/api/0.6/map.json?bbox=10.4170,51.9060,10.4260,51.9140), **1 Kachel, 0 Kachel-Fehler**, 7816 Elemente. Zaehlung getrennt: **20 Adressobjekte / 11 verschiedene Hausnummern** (1, 3, 4, 10, 6, 5, 11, 12, 3A, 2, 9a) auf der Straße „Klubgartenstraße” - die Aufzaehlung entspricht exakt den live vorhandenen Nummern; Filter-Key `addr:street=Klubgartenstraße` (mit `addr:housenumber` als Hausnummer); im gesamten Rastergebiet **471 Adressobjekte mit Hausnummer / 95 verschiedene Hausnummern**; im Raster tragen **585 Objekte `name=`** und **208 Objekte `operator=`**. Die Strasse traegt im Raster **9 Strassenways** (way/1306865112, way/207815219, way/24599750, way/28355795, way/366676209, way/4806292, way/589855595, way/589855602, way/589855604). **Die Dossier-Hausnummer 5 existiert**: way/76464125 traegt `addr:street=Klubgartenstraße`, `addr:housenumber=5`, `addr:city=Goslar`, `addr:postcode=38640` (weitere Tags des Objekts nicht geprueft). Objekte mit Bezug zum Betrieb im Raster: **keines** - die Suche nach „Rewimet“ und „Recycling-Cluster“ ueber die Felder name, operator, description, website, contact:website, note, contact:email, email, short_name und brand findet **keinen** Treffer; das gilt fuer den gesamten Raster, nicht nur fuer die Strasse. Auf der Strasse selbst tragen **14 Objekte** `name=`/`operator=` - saemtlich **Fremdbetriebe** (u.a. way/76464543 mit `name=Kreisverwaltung Goslar` und `addr:housenumber=11`, way/287801703 mit `name=Landkreis Goslar` und `office=government`, die Relation relation/4858804 (ebenfalls „Kreisverwaltung Goslar“, `type=multipolygon`), way/103192668 mit `name=Medizin am Klubgarten`, way/103192682 mit `name=Niedersächsischer Hof` und `operator=Concorde Hotels`, node/5058120668 mit `name=Orthopädie am Klubgarten`, node/8017976776 mit `name=Tanzschule Kastern`). **Die Strasse ist ein Verwaltungs-/Geschaeftszentrum, kein Schrottplatz.** Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (bbox-Raster, live abgerufen 31.10.2026) ; https://api.openstreetmap.org/api/0.6/<objekt>.json ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Verein, kein Haendler - die Vorwellen-Abgrenzung wird durch OSM gestuetzt, Preise sind nicht belegbar.** Die Objekt-Suche im Raster findet **keinen** Betriebsbezug zu REWIMET oder zum Recycling-Cluster; die einzigen benannten Objekte auf der Klubgartenstraße sind **Fremdbetriebe** und **Behoerden der Stadtverwaltung**. Das deckt sich mit dem Vorwellen-Befund vom 03.10.2026: REWIMET e.V. ist ein **Verein** (Vereinsregister 200914 AG Braunschweig) mit Netzwerkcharakter; die in der Vorwelle korrigierte „Im Schleeke 50“-Anschrift gehoert zur **Metalogie GmbH**, die **Geschäftsstelle** des Vereins liegt laut Impressum in der **Leibnizstraße 23, 38678 Clausthal-Zellerfeld** - die **Klubgartenstraße 5 in Goslar traegt in OSM keine Vereins- oder Recycling-Adresse**. **ANKAUF:** fuer den Goslarer Eintrag ist **kein** Ankauf belegbar - ein Verein mit Mitgliedern, deren einzelne Recycling-Leistungen **keine** eigene Haendlerleistung sind. **VERKAUF:** nicht belegbar. **GEBUEHREN:** nicht belegbar. `status: pruefung`, `website_status: unbekannt` und die leere `website` bleiben **unveraendert** - die Domain rewimet.de antwortet zwar, ist aber **nicht** als eigene Ankaufstelle fuer Goslar verifiziert. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=10.4170,51.9060,10.4260,51.9140 ; https://rewimet.de/impressum (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen und Adressklaerung.** (a) Ob die Adresse **Klubgartenstr. 5, Goslar** ueberhaupt zu einem REWIMET-bezogenen Betrieb gehoert, ist **offen** - in OSM steht dort die **Kreisverwaltung Goslar** (way/76464543) und die Adresse traegt keinen Recycling-Bezug. Der Verein sitzt laut Impressum in **Leibnizstraße 23, 38678 Clausthal-Zellerfeld**. (b) Der in der Vorwelle als „Klärfall“ notierte Punkt bleibt: die **frühere Website-Angabe rewimet.de war falsch** (Verein, kein Haendler) - sie ist weiterhin nicht gesetzt. (c) **Geokodierungsbedarf:** falls der Dossier-Eintrag bestehen bleibt, braucht **Klubgartenstr. 5, Goslar** eine Geokodierung; die Koordinate wird hier **nicht** gesetzt. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=10.4170,51.9060,10.4260,51.9140 (Abruf 31.10.2026).]

### Geokodierung 31.10.2026

- [31.10.2026 (Geokodierung): **keine Koordinate gesetzt - Geokodierungsbedarf gemeldet.** Der gespeicherte Produktionspunkt **51.9191000/10.4167000** liegt **1092 m** vom Objektpunkt entfernt - er passt **nicht** zur belegten Adresse und ist zu ersetzen (Geokodierungsbedarf). Quelle 1 (OSM): way/76464125 traegt `addr:street`=Klubgartenstraße, `addr:housenumber`=5, `addr:city`=Goslar, `addr:postcode`=38640; die Objekt-BBox liegt bei **51.9097972/10.4220304 bis 51.9099706/10.4223407** (Mitte 51.9098839/10.4221856). Die Dossier-Hausnummer **5** ist belegt - **ohne** jeden Recycling-Bezug. Quelle 2 (Betreiber): https://rewimet.de/impressum nennt die Adresse **Klubgartenstr. 5, 38640 Goslar**. Die Anschrift ist damit belegt; die Koordinate wird **hier bewusst nicht gesetzt** (Auftrag dieser Welle: Adressen benennen, nicht selbst geokodieren) und ist ueber den adressbasierten Cache nachzuziehen. Quelle(n): https://api.openstreetmap.org/api/0.6/way/76464125.json ; https://rewimet.de/impressum (Abruf 31.10.2026).]
