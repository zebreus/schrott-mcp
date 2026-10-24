---
slug: ni-hannover-e-klemens-schrott-u-metalle
clear_fields: 'dropoff_json,geo,phone'
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
