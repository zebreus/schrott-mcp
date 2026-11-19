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

### Recherche 19.11.2026 (Welle 1800 A)

- [Recherche 19.11.2026: **Der Joa im amtlichen Fachbetrieberegister ist der Wölfersheimer, nicht der Altenglaner; die Verbindung des Seed-Falls zu diesem Betrieb ist damit ausgeschlossen.** Das amtliche Fachbetrieberegister (live am 19.11.2026, Filter Art des Betriebs = Egal, Ergebnisgrid vollständig gelesen) liefert für die Namenssuche `Joa` in der Route "Entsorgungsfachbetriebe" 23 Treffer, von denen die ersten drei dem Betrieb `Stefan Joa Containerdienst - Schrotthandel - Entrümpelung - Stahldemontage Inhaber Andre Joa Standort / Hauptsitz`, Im Leituch 8, 61200 Wölfersheim, `ZZFT01100159 / 1`, `/ 2` und `/ 3` mit den Tätigkeiten Sammeln und Befördern, Lagern sowie Behandeln gehören; die übrigen 20 Zeilen sind Namensvetter mit den Namensbestandteilen Joachim oder Hans-Joachim, darunter `Joachim Schrotthandel GmbH`, Leutersgrube 2, 97616 Bad Neustadt, und `Joachim Alpers GmbH`, Dinghorner Str. 125, 21717 Fredenbeck. In der Route "Altfahrzeugverwertung" liefert `Joa` 2 Treffer, beide ortsfremd. **Der Wölfersheimer Betrieb ist amtlich zertifiziert; das Altenglaner Seed-Fragment ist es nicht.** Das Originalzertifikat (Dateiname `ZZFT011001599.pdf`, live aus dem Register geladen) nennt in Feld 3.3 die Vorgangsnummer `ZZFT011001590009`, in Feld 3.1 die Zertifikatsnummer `074/01/04`, die Zertifizierungsorganisation `SVG Zertifizierungsdienst GmbH`, Breitenbachstraße 1, 60487 Frankfurt am Main, 3 Anlagen, gültig bis 31.12.2026, Prüfungsdatum 16.07.2025 und Ausstellungsdatum 28.07.2025, in Feld 4 den Betriebsnamen `Stefan Joa Containerdienst - Schrotthandel - Entrümpelung - Stahldemontage Inhaber Andre Joa` mit `Registernummer: Nicht zutreffend`, also als Einzelunternehmen. Die Standortabfrage mit 3 km um `Industriestraße 9, 66885 Altenglan` liefert in der Route "Entsorgungsfachbetriebe" 0 Zeilen und in der Route "Altfahrzeugverwertung" 2 Zeilen, beide `AVR GmbH`, Industriestraße 9, 66885 Altenglan, `ZAGS00300052 / 1` und `ZAGS00300052 / 2`; die im Dossier dokumentierte Ortsangabe der AVR GmbH ist damit amtlich bestätigt, ein Joa steht dort nicht. Das amtliche Handelsregisterportal (online-handelsregister.de, Abruf 19.11.2026) liefert für `Joa Altenglan` weder Unternehmen noch Personen. **Reichweite:** die Efb- und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein Nichtexistenzbeweis; im Handelsregisterportal erscheinen Einzelunternehmer ohne Registereintrag nicht. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Joa ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Joa ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Industriestra%C3%9Fe%209%2C%2066885%20Altenglan%2C%20Deutschland%3B7.4628815%3B49.5577823%3B7.5063918%3B49.5851062%3B7.4193712%3B49.5304584&dst=3&is=1&e=1 ; https://www.online-handelsregister.de/suche/Joa%20Altenglan (Abruf 19.11.2026).]
- [Recherche 19.11.2026: **Der Wölfersheimer Auftritt ist heute neu gebrandet und nennt einen anderen Ansprechpartner; der Namensvetter bleibt damit auch aktuell getrennt.** Die Startseite von `stefan-joa.de` (live am 19.11.2026, HTTP 200) trägt den Titel `André Joa | Containerdienst Wölfersheim`, nennt `Seit Mai 2005 zertifizierter Entsorgungsfachbetrieb`, `Gegründet 1972, drei Generationen`, `5.000 m² Betriebsgelände` im Gewerbegebiet Wölfersheim und `Privatanlieferung jederzeit während der Geschäftszeiten`; der Schrottankauf wird mit `Ankauf zu Tagespreisen` beworben, ohne Beträge. Der Namensvetter wird **nicht** übernommen. **ANKAUF:** für den Dossier-Betrieb in Altenglan sind weiterhin keine Preise oder Annahmebedingungen belegt. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Tarife. Quelle(n): https://www.stefan-joa.de/ (Abruf 19.11.2026).]
- [Recherche 19.11.2026: **GEO: der gespeicherte Produktionspunkt trifft ein Industriegebäude in Patersbach und liegt 0,92 km von der im Register belegten Industriestraße.** Der in der Produktionsdatenbank gespeicherte Punkt 49.55/7.4667 löst per Nominatim-Rückabfrage auf `3, Austraße, 66885 Altenglan-Patersbach` auf, also auf das Objekt `way/384946478` mit `building`=manufacture; die einzige im Register belegte Altenglaner Straße trägt bei Hausnummer 9 das Objekt `way/845903943` (mit `addr:housenumber`=9, `addr:street`=Industriestraße, `addr:postcode`=66885, `addr:city`=Altenglan, `building`=yes) bei 49.5578766/7.4627987, die Luftlinienabweichung beträgt 0,92 km. Das Seed-Fragment `Streithühle 4` bleibt davon unberührt in keiner Straße des geprüften Fensters auffindbar. Das Frontmatter führt keine `clear_fields`-Anweisung, der Punkt bleibt damit in der Produktion stehen; er wurde gemessen und nicht eigenmächtig geändert. Quelle(n): https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.55&lon=7.4667&zoom=18&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/way/384946478.json ; https://api.openstreetmap.org/api/0.6/way/845903943.json (Abruf 19.11.2026).]
