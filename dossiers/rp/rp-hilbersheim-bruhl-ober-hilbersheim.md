---
slug: rp-hilbersheim-bruhl-ober-hilbersheim
name: Brühl Ober-Hilbersheim
trader_type: sonstige
state: RP
city: Hilbersheim
street: Brühl Ober-Hilbersheim Hilbersheim Obergasse 12
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

# Brühl Ober-Hilbersheim

## Überblick

Die Seed-Zeile ist offenbar aus Name und Ort zusammengesetzt; der Straßenwert enthält beides zusätzlich. Konkreter Lead ist „schrottverwertung Brühl“, Obergasse 12, **55437 Ober-Hilbersheim**, Rufnummer 06728 7349963. Das stammt allein aus Cylex, nicht aus einem Betreiber-/Registerbeleg; daher weder Stadt/Straße normalisiert noch PLZ/Telefon übernommen.

Brühl ist hier mutmaßlich Firmen-/Personenname, nicht automatisch die NRW-Stadt Brühl. **Ankauf / Verkauf / Gebühren:** offen, kein Preisblatt verifiziert. Erst Rechtsinhaber und Ober-Hilbersheimer Anschrift unabhängig bestätigen, dann Annahme und Geocodierung prüfen.

## Timeline

### Recherche 15.10.2026 (00:00-Welle A)

- [Recherche 15.10.2026: Eigenständige Adresssuche Brühl + Obergasse 12 + Schrott liefert Cylex-Lead schrottverwertung Brühl, Obergasse 12, 55437 Ober-Hilbersheim, 06728 7349963. Folgesuche nach Betreiber/Register ohne passenden zulässigen Treffer; zahlreiche Ergebnisse für Brühl (NRW) verworfen. Aggregator ist ausdrücklich nur Suchansatz: keine Reparatur des zusammengesetzten Straßenfelds und keine PLZ-/Telefon-Fills ohne Gegenbeleg. status pruefung bleibt; Ankauf/Verkauf/Gebühren nicht verifiziert, GEO zurückgestellt. Quellen/Suchspuren: https://web2.cylex.de/firma-home/schrottverwertung-bruehl-16581904.html (Suchindex); https://www.google.com/search?q=%22Br%C3%BChl%22+%22Obergasse+12%22+Schrott ; https://www.google.com/search?q=%22Br%C3%BChl%22+%22Ober-Hilbersheim%22+Schrott+Register .]

### Importiert (Seed-Stand 2026-09-30)

- Obergasse 12
- urspr. Website-Angabe: keine
- Adresse: Hilbersheim

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Hausnummer **12** findet sich in OSM auf
  der Dossier-Straße **Obergasse** in **Ober-Hilbersheim** **nicht**; zusätzlich ist das `street`-Feld ein
  **Importartefakt** („Brühl Ober-Hilbersheim Hilbersheim Obergasse 12“ - Name, Ort und Straße in einem Feld) und die
  Beleglage beruht allein auf einem **Cylex-Lead**. **Was geprüft wurde (live am 03.11.2026):** (a) Die Straße
  **Obergasse** in **Ober-Hilbersheim (55437)** ist in OSM mit **zwei** Ways kartiert (way/25504403 und
  way/48299186, `highway`=living_street, `name`=Obergasse, `surface`=paving_stones) und trägt **15 Adressobjekte** mit
  den Hausnummern **1, 2, 3, 4, 6, 7, 8, 9, 10, 15, 17, 19, 21, 23, 25** - die **Hausnummer 12 existiert dort
  nicht**; das Kachelraster über **bbox=8.0100,49.8800,8.0500,49.9150** (live abgerufen 03.11.2026) umfasst den
  gesamten Ortskern und ist vollständig verfügbar. (b) Eine Suche über **alle** Tag-Werte nach „brühl“ bzw. einem
  Schrott-/Verwertungsbetrieb im Raster liefert **0 Objekte**, die **den eigenen Betrieb** nennen - der Betrieb ist in
  OSM **nicht namentlich kartiert**; die Objekte tragen **weder `name=` noch `operator=`**. **Quelle 1 (Verzeichnis):**
  der **Cylex-Lead** (im Dossier vom 15.10.2026 zitiert,
  https://web2.cylex.de/firma-home/schrottverwertung-bruehl-16581904.html) nennt „**schrottverwertung Brühl**“ mit
  „**Obergasse 12, 55437 Ober-Hilbersheim**“, Telefon 06728 7349963 - das ist ein **Verzeichnislead**; eine
  **Betreiberseite**, ein **Impressum** oder ein **Registereintrag** existiert **nicht** (das Dossier hat die
  Folgesuche nach Betreiber/Register am 15.10.2026 ohne zulässigen Treffer dokumentiert). **Quelle 2 (OSM):** die
  Straßen- und Adressobjekte der Obergasse (live abgerufen 03.11.2026 über
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.0100,49.8800,8.0500,49.9150). **Was fehlt:** ein
  OSM-Objekt mit `addr:street`=Obergasse **und** `addr:housenumber`=12 (oder ein benanntes Betriebsobjekt an dieser
  Anschrift) **sowie** eine Bestätigung der **Rechtsinhaberschaft**; nach der Recherche vom 15.10.2026 ist
  außerdem zu klären, ob **„Brühl“** hier Firmen-/Personenname ist und **nicht** die NRW-Stadt **Brühl** - eine
  Verwechslung mit dieser ist auszuschließen. `status` bleibt **pruefung**, `street`/`postcode`/`phone`/`website`
  bleiben unverändert bzw. leer, `clear_fields: 'geo'` bleibt bestehen. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.0100,49.8800,8.0500,49.9150 (Kachelraster, live abgerufen
  03.11.2026) ; Recherche-Vermerk vom 15.10.2026 im Dossier.]
