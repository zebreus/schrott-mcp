---
slug: ni-hannover-e-klemens-schrott-u-metalle
lat: 52.4186639
lon: 9.8522529
clear_fields: 'dropoff_json,phone'
name: E. Klemens Schrott u. Metalle
trader_type: schrotthaendler
state: NI
city: Hannover
street: ''
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
provenance_section: Hannover / Region Hannover
provenance_ankauf_raw: ja
provenance_origin: table
---

# E. Klemens Schrott u. Metalle

## Überblick

Der aktuelle Gelbe-Seiten-Eintrag bezeichnet den Händler als „Klemens Enrico“, Moorwaldweg 125c, 30659 Hannover, Telefon 0511 614705 und Mobil 0172 2856532; er behauptet „Abholung von Altmetall frei Haus“. Das ist weiterhin eine einzelne Aggregatorquelle ohne Betreiberwebsite oder unabhängige Register-/Kommunalbestätigung. Stadtbranchenbuch nennt abweichend Hausnummer 125B. Die vorhandene Seed-Adresse 125C bleibt ein Lead, nicht als bestätigte Annahmestelle oder geokodierbare Betriebsstätte zu lesen.

- **Ankauf:** kein vergüteter Ankauf oder Kurs belegt; „Abholung von Altmetall frei Haus“ ist kein Ankaufspreis und beweist keine Auszahlung.
- **Verkauf:** keine Produkt- oder Preisliste.
- **Gebühren:** Gelbe Seiten behauptet freie Abholung, aber ohne Umfang, Mindestmenge oder Bedingungen; keine bestätigte Gebührenordnung.
- **Offen:** Betreiberidentität, Hausnummer 125B/125C und heutige Annahmefunktion. `status: pruefung` bleibt.

## Timeline

### Owner-GEO-/Adress-/Annahme-Korrektur 24.10.2026

- [Owner-GEO-Korrektur 24.10.2026: Read-only-Produktion enthielt Punkt 52.376469, 9.746341, der Nominatim-Reverse zum Fachgerichtszentrum Hannover, Leonhardtstraße 15, auflöst, nicht zu Moorwaldweg 125B/125C. `dropoff_json={"allowed":true}` war ebenfalls gespeichert, obwohl die aktuelle Recherche nur einen unbestätigten Abholhinweis aus einem Verzeichnis und keine öffentliche Hofannahme fand. Straße, PLZ und Telefonnummer wurden wegen zweier widersprüchlicher Verzeichnis-Hausnummern und fehlender Betreiberbestätigung aus den strukturierten Feldern entfernt; `clear_fields: 'dropoff_json,geo,phone'` nimmt die alte Annahme-, GEO- und Telefonanreicherung zurück. Kandidaten und Quellen bleiben in der Timeline; keine Ersatzkoordinate, keine Aussage über Schließung. Keine Produktionsdaten geändert. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=52.376469&lon=9.746341&zoom=18&addressdetails=1 ; die im Rechercheeintrag 24.10.2026 verlinkten Verzeichnisquellen; read-only `public.db`; `docs/trader-seed.md`.]

### Recherche 24.10.2026

- [Recherche 24.10.2026: Exakte aktuelle Namens-/Orts-Suche ergab keinen Betreiberauftritt oder neuen unabhängigen Adressbeleg. Gelbe Seiten führt „Klemens Enrico“, Moorwaldweg 125c, 30659 Hannover und 0511 614705; Stadtbranchenbuch nennt 125B. Beide Quellen bleiben Verzeichnis-Leads. „Abholung von Altmetall frei Haus“ ist keine Bestätigung von vergütetem Ankauf und keine Aussage über Annahme am Hof. Keine Telefon-/Adressänderung, keine Preise für Ankauf/Verkauf oder Gebühren und keine Koordinaten. Moorwaldweg 125B/125C bleibt ein strittiger Geokodierungskandidat; öffentliche Anlieferung nicht belegt. Quellen: https://www.gelbeseiten.de/gsbiz/4e4e6229-dff0-4b91-a124-2c89237d2a02 ; https://hannover.stadtbranchenbuch.com/105406.html .]

### Recherche 22.10.2026: Einzelaussage zu Gratisabholung, Hausnummer weiter strittig

- [Recherche 22.10.2026: Gelbe-Seiten-Detail direkt gelesen: „Klemens Enrico“, Moorwaldweg 125c, 30659 Hannover, 0511 614705 und 0172 2856532; unter „Über das Unternehmen“ steht „Abholung von Altmetall frei Haus“, weitere Bedingungen oder Preise fehlen. Das andere Verzeichnis Stadtbranchenbuch führt „Schrott u. Metalle Enrico Klemens“ an 125B, bei gleicher Festnetznummer. Beide sind Aggregator-Leads, kein Betreiber-/Registerbeleg; die 125C-Angabe wird nicht als gesichert ausgewählt. Keine neuen Frontmatter-Fills oder Koordinaten. Ankauf/Auszahlung und Materialpreise offen; „frei Haus“ nur als unbestätigte Abhol-/Gebührenaussage, keine Verkaufs- oder Preisliste. Quellen: https://www.gelbeseiten.de/gsbiz/4e4e6229-dff0-4b91-a124-2c89237d2a02 ; https://hannover.stadtbranchenbuch.com/105406.html .]

### Recherche 03.10.2026

- [Recherche 03.10.2026: Status aktiv auf pruefung gesetzt, nicht geschlossen: 11880 zeigt einen aktualisierten Eintrag (06.09.2026) für Moorwaldweg 125 C und 0511 614705; Gelbe Seiten stimmt dabei überein, Tellows nennt jedoch 125 B. Das sind Verzeichnis-/Aggregator-Leads, keine zwei belastbaren unabhängigen Betreiberbelege; die vorherige Mehrheitswahl 125C ist deshalb keine bestätigte Hausnummer. Straße, PLZ und Telefon bleiben als bestehende, unbestätigte Leads unverändert; keine Geokodierung/Koordinaten, Öffnungszeiten oder Leistungen ergänzt. Keine Betreiber-Website oder belastbare Register-/Kommunalbestätigung gefunden; Statusprüfung bedeutet keine Schließungsbehauptung. Offen: tatsächliche Hausnummer/Betreiberidentität und ob aktuell Anlieferung oder Abholung angeboten wird. Quelle(n): https://www.11880.com/branchenbuch/hannover/060440092B113979576/schrott-u-metalle-e-klemens.html; https://www.gelbeseiten.de/suche/schrott-u.-metalle/hannover; https://www.tellows.de/num/0511614705]

### Importiert (Seed-Stand 2026-09-30)

- Schrott/Metalle

### Recherche 02.10.2026

- [Recherche 02.10.2026: Moorwaldweg 125C, 30659 Hannover (Lahe) + Tel. 0511 614705 in Frontmatter (11880-Liveabruf Stand 06.09.2026 + Gelbe Seiten + Stadtbranchenbuch + Tellows konsistent); ABER Rufnummern-Variante 0511 614732 (Cylex, älter) vs 0511 614705 (Mehrheit, aktuell) → 614705 gefüllt, Variante dokumentiert; Adress-Variante 125B (Tellows/Stadtbranchenbuch) vs 125C (11880/Gelbe Seiten) → 125C gefüllt (aktuellerer Beleg); keine Betreiber-Website → website_status unbekannt; Status bleibt aktiv; Quelle(n): 11880.com/schrott-u-metalle-e-klemens, gelbeseiten.de, hannover.stadtbranchenbuch.com, tellows.de, cylex.de]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **52.4186639/9.8522529** (Mittel der **11**
  eindeutigen Knoten von way/516068818; die BBox der eindeutigen Knoten reicht von 52.4185871/9.8521381 bis
  52.4187350/9.8523679 und **umschließt den Mittelwert**). Quelle 1 (OSM, live geladen am 04.11.2026, HTTP 200):
  **way/516068818** trägt `addr:street`=Moorwaldweg, `addr:housenumber`=**125c**, `addr:postcode`=30659,
  `addr:city`=Hannover, `addr:country`=DE, `building`=detached - die Dossier-Hausnummer **125c** ist damit in OSM
  **eindeutig** vorhanden. **Fensterprüfung (kein Fensterfehler):** das am 04.11.2026 live abgerufene Kachelraster
  umfasst **9.8500/52.4100 bis 9.8700/52.4300**, **4 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**; das Fenster
  enthält die gesuchte Straße **nachweislich** mit **13** Straßenobjekten (way/23896421, way/33127014,
  way/34700410, way/277256512, way/277256515, way/407535749, way/951931297, way/951931298, way/1166292819,
  way/1182081683, way/1182081684, way/1182081685, way/1295601974) sowie **10** Adressobjekten auf „Moorwaldweg" mit
  **10** verschiedenen Hausnummern (120, 123, **125b**, **125c**, 255, 255a, 312, 314, 355, 356) - die beiden
  strittigen Nummern **125b** und **125c** sind **beide** erfasst (way/516068820 bzw. way/516068818) und liegen
  **36 m** auseinander, die Zuordnung ist also nicht durch OSM, sondern durch die Verzeichnisquellen zu entscheiden.
  Quelle 2 (Verzeichnis, im Dossier bereits zitiert, live nachgelesen am 04.11.2026): der **Gelbe-Seiten-Eintrag**
  (https://www.gelbeseiten.de/gsbiz/4e4e6229-dff0-4b91-a124-2c89237d2a02) liefert **HTTP 200** und nennt live **„Klemens
  Enrico, Schrott, Moorwaldweg 125c, 30659 Hannover, 0511 61 47 05"** mit dem Betriebstext „Abholung von Altmetall frei
  Haus"; die in den **JSON-LD-Angaben** derselben Seite ausgewiesene Standortkoordinate **52.418636/9.852251** fällt auf
  das OSM-Objekt **125c** (Abweichung **3,1 m**) und **nicht** auf 125b (Abweichung 36,3 m) - die Verzeichnisangabe
  „125c" ist damit geometrisch **bestätigt**. Der **11880-Eintrag** nennt live **„Schrott u. Metalle E. Klemens,
  Moorwaldweg 125 C, 30659 Hannover (Lahe)"** und bestätigt die Nummer unabhängig vom Gelbe-Seiten-Eintrag. Die
  abweichende Angabe **125B** (Stadtbranchenbuch/Tellows) bleibt als **Gegenbeleg** ungelöst dokumentiert, sie ist
  jedoch durch die 3,1-m-Lage der Verzeichnis-Koordinate **nicht** gestützt. **Zurücknahme des `geo`-Tokens
  (Begründung):** `clear_fields` wurde von `'dropoff_json,geo,phone'` auf **`'dropoff_json,phone'`** gekürzt - nur das
  Token **geo** wurde entfernt, weil es den am 24.10.2026 zurückgenommenen und falsch liegenden Produktionspunkt
  (52.376469/9.746341, Fachgerichtszentrum Hannover, Leonhardtstraße 15) fernhalten sollte; der hier gesetzte Punkt
  liegt **exakt** auf der von zwei Verzeichnissen belegten Anschrift, während `clear_fields` nicht gleichzeitig `geo`
  neben belegtem `lat`/`lon` stehen darf. **Qualifizierung (unverändert übernommen):** Der Punkt bezeichnet die
  **verzeichnisgestützte Anschrift** eines **mobilen** Kleinbetriebs („Abholung von Altmetall frei Haus"), **nicht**
  einen öffentlichen Annahmehof, eine Waage oder eine Schrottfläche - ein Gewerbe-/Recycling-Tag existiert an beiden
  Gebäuden nicht (beide sind als `building=detached` erfasst); `street`, `postcode`, `phone`, `email`,
  `opening_hours`, `website`, `website_status` und `description` bleiben **leer** wie vom Dossier entschieden, und
  `status: pruefung` bleibt unverändert; die Aussage „Abholung von Altmetall frei Haus" wird **nicht** als
  vergüteter Ankauf, als Auszahlung oder als Gebührenfreiheit übernommen. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/516068818/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=9.8500,52.4100,9.8700,52.4300 ;
  https://www.gelbeseiten.de/gsbiz/4e4e6229-dff0-4b91-a124-2c89237d2a02 ;
  https://www.11880.com/branchenbuch/hannover/060440092B113979576/schrott-u-metalle-e-klemens.html (Abruf 04.11.2026).]
