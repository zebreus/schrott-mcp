---
slug: rp-breitscheid-frohlich-breitscheid
clear_fields: 'geo'
name: Fröhlich Breitscheid
trader_type: sonstige
state: RP
city: Breitscheid
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

# Fröhlich Breitscheid

## Überblick

Der Creditreform-Einzelbeleg „Michael Fröhlich, Breitscheid, Gewerbebetrieb, wirtschaftsaktiv" bleibt die einzige Evidenzklasse für die Betreiberidentität; eine Straße, eine Telefonnummer oder eine Betreiber-Website wurden nicht belegt. Der Verzeichnislead aus dem Vorverlauf (Josef-Reuschenbach-Str. 2, Breitscheid, 0172 2041425) bleibt ein Aggregator-Lead ohne eigenständige Bestätigung. Der in der Produktionszeile gespeicherte Koordinatenpunkt lag rund 35 Kilometer vom Ort Breitscheid (Landkreis Altenkirchen) entfernt auf der Gemarkung Horressen (56410, Westerwaldkreis) und widersprach damit der eigenen Ortsangabe des Dossiers; `clear_fields: 'geo'` zieht ihn zurück.

**ANKAUF:** keine Kurse. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife. Offen bleiben Straße, Telefon und die Frage, ob der Gewerbebetrieb heute noch besteht.

## Timeline

### Recherche 27.10.2026: Geo-Punkt zurückgezogen, Identität weiter offen

- [Recherche 27.10.2026: GEO-Korrektur: Der gespeicherte Punkt 50.4388666666667, 7.77833333333333 löst per Nominatim-Reverse auf „Brillenweg, Horressen, Montabaur, Westerwaldkreis, 56410" auf. Die exakte Nominatim-Abfrage „Breitscheid, Landkreis Altenkirchen, Rheinland-Pfalz" löst dagegen auf 50.7499806, 7.6620905 auf (Breitscheid Bahnhofstraße, Bahnhofstraße, Pfannenschoppen); die Differenz beträgt rund 35 Kilometer und der Punkt liegt in einem anderen Landkreis und in einer anderen Gemeinde als die Ortsangabe des Dossiers. `clear_fields: 'geo'` entzieht das Alt-Paar beim regulären Seed; das bedeutet nur „keine kuratierte aktuelle Koordinate", keine Schließung und keine Annahmeverweigerung. Eine Ersatzkoordinate wurde nicht geraten, weil für Breitscheid selbst keine belegte Hausnummer vorliegt. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.4388666666667&lon=7.77833333333333&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Breitscheid%2C%20Landkreis%20Altenkirchen%2C%20Rheinland-Pfalz&format=jsonv2&addressdetails=1&limit=3 ; read-only-Abgleich `public.db` (27.10.2026) ; `docs/trader-seed.md`.]
- [Recherche 27.10.2026: Identitätslage unverändert: Die Betreiber-Domains aus dem Vorverlauf liefern weiterhin NXDOMAIN bzw. kein DNS, ein Registereintrag oder ein Impressum für Michael Fröhlich in Breitscheid (RP) wurde nicht gefunden. Der Creditreform-Einzelbeleg (Gewerbebetrieb, wirtschaftsaktiv, ohne Straßenangabe) bleibt Aktivitäts-Einzelbeleg; der Verzeichnislead „Josef-Reuschenbach-Str. 2, 0172 2041425" ist Aggregator und wurde nicht ins Frontmatter übernommen. Straße, Telefon, E-Mail und Website bleiben leer, Status bleibt `pruefung`. Quellen: https://www.11880.com/suche/schrotthandel/breitscheid ; https://web2.cylex.de/suche/schrotthandel/Breitscheid (Suchindex) ; Creditreform-Firmeneintrag 01.10.2026 laut Vorverlauf.]
- [Recherche 27.10.2026: **ANKAUF:** keine Kurse und keine Materialangaben. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife. Der Portal-FAQ-Hinweis auf einen möglichen Euro-Preisbereich ohne Material- und Mengenbezug ist keine Preisliste und wurde nicht übernommen. Offen bleibt die Gewerbeauskunft der Ortsgemeinde Breitscheid bzw. der Verbandsgemeinde Altenkirchen-Flammersfeld zur Straße und zum aktuellen Inhaber. Quellen: https://www.11880.com/suche/schrotthandel/breitscheid .]

### Importiert (Seed-Stand 2026-09-30)

- urspr. Website-Angabe: keine
- Adresse: Breitscheid

### Recherche 01.10.2026

- [Recherche 01.10.2026: Creditreform-Beleg: Michael Fröhlich, Breitscheid, Gewerbebetrieb, wirtschaftsaktiv (ohne Straßenangabe) — als Aktivitäts-Einzelbeleg nur hier. Betreiber-Domains tot (NXDOMAIN/kein DNS). Aggregator-Leads ohne Belegwert: Josef-Reuschenbach-Str. 2, Breitscheid, Tel. 0172 2041425. Kein HRB → Owner-Direktive greift nicht; kein Fill, Status pruefung bleibt, Klärfall: Straße/Telefon via Gewerbe-/Melderegister Breitscheid klären; Quelle(n): Creditreform-Firmeneintrag 01.10.2026, DNS-/curl-Checks, Leads 11880/Cylex]
