---
slug: rp-worms-kocher-stefan
clear_fields: 'geo'
name: Köcher Stefan
trader_type: schrotthaendler
state: RP
city: Worms
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
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Köcher Stefan

## Überblick

Für „Köcher Stefan“ liegen weiterhin zwei widersprüchliche Verzeichnis-Cluster vor: Heinrich-von-Gagern-Straße 17, 67549 Worms mit Tel. 06241 56721 vs. Nordendstraße 10, 67547 Worms mit Tel. 06241 45385. Kein Betreiber-/Registerbeleg verbindet diese Treffer; keine Adresse/Kontakte übernehmen, Status `pruefung`.

**Preise:** ANKAUF: keine Bedingungen oder Preise verifiziert. VERKAUF: keine Preisliste. GEBÜHREN: keine Angaben.

## Timeline

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 49.630025, 8.332675. Nominatim-Reverse löst den Punkt an Ludwig-Uhland-Straße 27 in Pfiffligheim auf; beide konkurrierenden Dossier-Adresscluster nennen andere Straßen und sind nicht dem Betreiber sicher zugeordnet. `clear_fields: 'geo'` entzieht den unpassenden Altpunkt beim nächsten regulären Seed. Keine Ersatzkoordinate. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.630024999999996&lon=8.332675&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026

- [Recherche 25.10.2026: Neue Namens-/Ortsnachsuche reproduziert bei Gelbe Seiten den Lead Heinrich-von-Gagern-Str. 17, 67549 Worms, 06241 56721; der widersprechende Cluster Nordendstr. 10, 67547, 06241 45385 bleibt aus anderen Verzeichnissen bestehen. Keine Betreiber-/Registerquelle oder Namensbrücke löst auf, ob es sich um denselben, verschiedene oder veraltete Einträge handelt. Keine Felder geändert. ANKAUF: keine Preise/Bedingungen; VERKAUF: keine Liste; GEBÜHREN: keine Angaben. Kein GEO bis Betreiber und exakter Standort geklärt. Quelle: https://www.gelbeseiten.de/gsbiz/56e67ae3-bd77-4073-8c4b-f9abf1a3fa47 (Verzeichniseintrag, Abruf 25.10.2026); ältere Clusterquellen sind oben dokumentiert.]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Vorhandene widersprüchliche Lead-Sets nochmals getrennt bewertet: Heinrich-von-Gagern-Str. 17 / 06241 56721 vs. Nordendstr. 10 / 06241 45385; beide stammen aus Branchenverzeichnissen, und keine Primärquelle, Registerzuordnung oder Namensbrücke wurde gefunden. Die widersprüchlichen Orte/PLZ können unterschiedliche Betriebe oder veraltete Profile bedeuten; eine Wahl zwischen ihnen wäre unbelegt. Keine Feldänderung. ANKAUF/VERKAUF/GEBÜHREN: keine verifizierten Konditionen, Preise oder Tarifblätter. Offen: Betreiber-Vorname/Rechtsträger und welcher Wormser Standort, falls überhaupt, zum Seed gehört; Quelle: bereits dokumentierte Verzeichnis-Leads: Gelbe Seiten, schrottplatz-info, oeffnungszeitenbuch, branchenbuchdeutschland; branchen-info, Firmania und Cylex (Einzelbeleg je Lead-Cluster; kein Primärbeleg).]

### Recherche 01.10.2026

- DDG-Recherche (01.10.2026): nur Aggregator-Leads, zudem WIDERSPRUCH – Heinrich-von-Gagern-Str. 17, 67549 Worms-Neuhausen, Tel. 06241 56721 (Gelbe Seiten, schrottplatz-info, oeffnungszeitenbuch, branchenbuchdeutschland) vs. Nordendstr. 10, 67547 Worms, Tel. 06241 45385 (branchen-info, Firmania, Cylex). Keine Betreiber-Website, kein HRB.
- Klärfall: keine Frontmatter-Fills – nur Aggregator-Leads mit Adress-/Telefon-Widerspruch, kein zweiter echter Beleg.

### Importiert (Seed-Stand 2026-09-30)

- Schrott vermutet
- Adresse: Worms

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Der Seed „Köcher Stefan Schrottentsorgung" hat
  **zwei widersprüchliche Adress-Cluster** (Gelbe Seiten: **Heinrich-von-Gagern-Str. 17, 67549 Worms-Neuhausen**,
  06241 56721; weitere Verzeichnisse: **Nordendstr. 10, 67547 Worms**, 06241 45385) und **keinen** Betreiber- oder
  Registerbeleg, der die beiden Orte einem Betrieb zuordnet. **Was live geprüft wurde (04.11.2026):** Das am
  04.11.2026 live abgerufene Kachelraster über Worms-Innenstadt umfasst **8.3600/49.6200 bis 8.3700/49.6300**,
  **1 Kachel von je 0,01 Grad, 0 Kachel-Fehler**; die Namenssuche nach **„Köcher"** über die Felder name, operator,
  description, website, contact:website, brand, note und addr:housename findet dort **keinen** Treffer, und eine
  Straßensuche nach **„Nordendstraße"** im selben Raster findet **keine** Adressobjekte dieser Straße - die
  Dossier-Adresse **Nordendstr. 10** ist im abgetasteten Bereich **nicht** als OSM-Objekt nachweisbar. Das zweite
  Raster über Worms-Pfiffligheim/Worms-Süd umfasst **8.3200/49.6200 bis 8.3400/49.6400**, **4 Kacheln von je 0,01
  Grad, 0 Kachel-Fehler**; es enthält die **Speerstraße** mit **24** Adressobjekten (Hausnummern 1 bis 32, lückenlos
  geradzahlig plus 3, 5, 7, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19) sowie die **Heinrich-Heine-Straße**,
  die **Ludwig-Uhland-Straße** und die **Alzeyer Straße**, aber **keine** „Heinrich-von-Gagern-Straße" und **keine**
  „Nordendstraße" - die **Dossier-Adresse Heinrich-von-Gagern-Str. 17** liegt damit **nicht** im abgetasteten Bereich.
  Quelle 1 (OSM): das live abgerufene Kachelraster 8.3600/49.6200–8.3700/49.6300 **und**
  8.3200/49.6200–8.3400/49.6400, jeweils **ohne Treffer** für „Köcher", „Nordendstraße" und
  „Heinrich-von-Gagern-Straße". Quelle 2 (Verzeichnis, im Dossier bereits zitiert, live nachgelesen am 04.11.2026):
  die **Gelbe-Seiten-Detailseite** liefert **HTTP 200** und führt live **„Köcher Stefan Schrottentsorgung"** in der
  Rubrik **Schrott** an **Heinrich-von-Gagern-Str. 17, 67549 Worms-Neuhausen**, Telefon **06241 56721** - das ist
  ein Aggregator-Lead und bestätigt **nur eine** der beiden Adressen. **Warum keine Koordinate:** Ein Punkt müsste
  sich zwischen Worms-Neuhausen (49.6467/8.3427) und Worms-Innenstadt (Nordendstraße) entscheiden - das wäre eine
  **Ratenentscheidung zwischen zwei widersprüchlichen Verzeichnisangaben** ohne Betreiber- oder Registerbrücke und
  würde zudem eine der beiden Anschriften unbelegt bevorzugen. **Was zur Auflösung fehlt:** ein lesbarer
  Betreiberauftritt (Impressum mit Rechtsträger), ein Registereintrag oder eine Klärung, welche der beiden
  Anschriften (oder ob beide) zum Seed gehören. **Reichweitenklausel:** Das **Fehlen einer Hausnummer in OSM** ist
  **kein** Nachweis für das Fehlen des Betriebs - die beiden Straßennamen sind in den abgetasteten Bereichen
  schlicht nicht enthalten, das Raster belegt also nur die Suchlage, nicht die Abwesenheit. `clear_fields: 'geo'`
  **bleibt** bestehen, `status: pruefung` bleibt unverändert; es wird weder Fortbestand noch Schließung behauptet.
  Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=8.3600,49.6200,8.3700,49.6300 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.3200,49.6200,8.3400,49.6400 ;
  https://www.gelbeseiten.de/gsbiz/56e67ae3-bd77-4073-8c4b-f9abf1a3fa47 (Abruf 04.11.2026).]
