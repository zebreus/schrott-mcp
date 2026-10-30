---
slug: rp-altenglan-joa-altenglan
name: Joa Altenglan
trader_type: sonstige
state: RP
city: Altenglan
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
provenance_section: Nachtrag Wide-Net PROSE-Cluster (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Joa Altenglan

## Überblick

Der Eintrag „Joa“ mit Seed-Fragment „Streithühle 4“ bleibt ungeklärt. Der bereits dokumentierte Ortsregister-Auszug führt dort keinen Joa, sondern AVR GmbH an der Industriestraße 9; die Adresse darf nicht auf Joa übertragen werden. Ein zusätzlicher Webtreffer zu Stefan Joa betrifft Wölfersheim (Hessen) und ist ausdrücklich kein Identitätsbeleg für Altenglan. Status `pruefung`.

**Preise:** ANKAUF: für Altenglan keine Preise/Annahmebedingungen belegt. VERKAUF: keine Preisliste. GEBÜHREN: keine Angaben. Der getrennte Wölfersheim-Auftritt wirbt mit Tagespreisen, aber ohne Beträge und ist nicht diesem Dossier zuzurechnen.

OSM-Abgleich (30.10.2026): Die einzige registerlich belegte Altenglaner Straße, die Industriestraße, trägt in OSM 4 Adressobjekte mit den Nummern 1, 2, 7a und 9; die Nummer 4 kommt dort nicht vor, und kein Objekt trägt einen Bezug zu "Joa". Für die Seed-Schreibweise "Streithühle" ist in dem geprüften OSM-Fenster keine Straße dieses Namens erfasst.

## Timeline

### Recherche 07.10.2026

- [Recherche 07.10.2026: Suchergebnis-Gegenprobe fand einen Betreiberauftritt „Stefan Joa Containerdienst“ mit Sitz in Wölfersheim, Hessen, Tel. 06036 988872; der Betreiberauftritt ordnet den Betrieb ausdrücklich Wölfersheim/Wetterau zu, nicht Altenglan. Dieser gleichnamige, räumlich klar abweichende Treffer wird nicht mit dem Seed verbunden. Der frühere Register-/EfB-Abgleich für Altenglan bleibt maßgeblich: nur AVR GmbH, Industriestraße 9, kein Joa. Keine Feldänderung. ANKAUF: die Wölfersheimer Seite nennt nur Tagespreise, keine Zahlen; für Altenglan nichts belegt. VERKAUF: keine Liste. GEBÜHREN: keine Tarife; Quelle: https://www.stefan-joa.de/ ; schrottregister ort-altenglan (GSA/EfB-Extrakt, Stand 11.08.2026, bereits dokumentiert)]

### Importiert (Seed-Stand 2026-09-30)

- Streitühle 4, 2. Händler
- urspr. Website-Angabe: keine
- Adresse: Altenglan

### Recherche 02.10.2026

- [Recherche 02.10.2026: Klärfall — "Streitühle 4" ohne Beleg (wohl Wohnplatz Streitmühle-Fehlparse); schrottregister ort-altenglan führt nur AVR GmbH, Industriestraße 9 (Lagern/Behandeln), kein "Joa" → kein Fill, status weiter pruefung. Quelle(n): schrottregister ort-altenglan (GSA/EfB-Extrakt, Stand 2026-08-11, Abruf 02.10.2026)]

### Recherche 30.10.2026

- [Recherche 30.10.2026: OSM-Abgleich der einzigen durch das amtliche Fachbetrieberegister belegten Altenglaner Straße; Ergebnis bestätigt die Vorwellen-Notiz, erweitert sie aber um die tatsächliche Nummernfolge. Im Rasterfenster **7.4607188/49.5526083 bis 7.4680323/49.5594794** (Straßen-BBox der Ways way/53559104 und way/1137609571 plus 0,002 Grad Rand) liegen auf der **Industriestraße** in Altenglan **4 Adressobjekte / 4 verschiedene Hausnummern**: 1, 2, 7a und 9. Für die Nummer 9 existiert genau ein Objekt, **way/845903943** mit den Tags `addr:street=Industriestraße`, `addr:housenumber=9`, `addr:city=Altenglan`, `addr:country=DE`, `addr:postcode=66885`, `building=yes`. Präzise Negativaussage: **die Hausnummer 4 ist auf der Industriestraße in OSM nicht erfasst**, die dort vorkommenden Nummern sind ausschließlich 1, 2, 7a und 9. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (bbox-Raster, Abruf 30.10.2026); https://api.openstreetmap.org/api/0.6/way/845903943.json]
- [Recherche 30.10.2026: Der Seed-Fragment "Streithühle 4" lässt sich keiner Straße in Altenglan zuordnen. In einem OSM-Fenster nordwestlich von Altenglan (**lon 7.4200 bis 7.5000, lat 49.5900 bis 49.6200**) sind im Fenster 7.42/49.59 bis 7.50/49.62 **34 verschiedene Straßennamen** erfasst (highway-Objekte mit `name=`; ohne diese Filterung liegt die Elementzahl bei 197); darunter "Christophelsmühle", "Hohlweg", "Mühlweg" und "Schrammenmühle", aber **keine Straße mit "streit" im Namen**. Das engere Fenster um den Ortskern (lon 7.4200 bis 7.5000, lat 49.5200 bis 49.5900) wurde vom OSM-Server mit HTTP 400 abgewiesen und ist daher **nicht** geprüft; die Negativaussage gilt folglich nur für das geprüfte Fenster und nicht für den gesamten Ort. Eine Adresssuche (Nominatim, Suchindex und **kein** OSM-Tag) liefert für die Schreibweisen "Streithühle" und "Streitühle" in Altenglan keinen Treffer. ANKAUF: keine Preise, keine Ankaufsbedingungen belegt. VERKAUF: keine Preisliste. GEBÜHREN: keine Tarife. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (bbox-Raster, Abruf 30.10.2026); https://nominatim.openstreetmap.org/search?q=Streith%C3%BChle%2C+Altenglan%2C+Germany&format=jsonv2]

- [30.10.2026 (Korrekturvermerk): **Zahl präzisiert**: die Angabe „44 verschiedene Straßennamen“ war unter keiner reproduzierbaren Regel erfüllbar. Live liefert das genannte Fenster (7.42/49.59 bis 7.50/49.62) **34** verschiedene Straßennamen, wenn — wie hier angegeben — `highway`-Objekte mit `name=` gezählt werden (alle benannten Objekte einschließlich Bauwerken: 197). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=<Raster laut Dossier> (Nachmessung 30.10.2026)]
