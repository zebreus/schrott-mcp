---
slug: rp-koblenz-56068-golden-gate
name: Golden Gate
trader_type: sonstige
state: RP
city: Koblenz
street: Entenpfuhl 12
postcode: '56068'
phone: 0261 5501139
email: ''
opening_hours: ''
website: https://www.goldengate-koblenz.de
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Golden Gate

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Goldankauf
- Adresse: Koblenz 56068

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Seite live verifiziert (goldengate-koblenz.de, Jimdo, abgerufen 01.10.2026): Golden Gate Goldankauf – Souvenirs/Geschenkartikel/Kleinantiquitäten, Inh. Frank Maurer, Entenpfuhl 12, 56068 Koblenz, Fon 0261-5501139, info@goldengate-koblenz.de; kein HRB-Impressum (Einzelunternehmen) → kein Owner-Ausnahme-Fall; Facebook-Seite (goldengates.goldankauf, abweichende Schreibweise) + YouTube-Imagefilm (Frank H. Maurer) stützen Existenz, liefern aber keine Adress-Zweitbelege → Adresse/Telefon/E-Mail nur Betreiber-Einzelbeleg → (Einzelbeleg, unsicher), NICHT in Frontmatter. website/website_status aktiv gefüllt (Domain-Root, live verifiziert). PERSONEN-LINK: derselbe Frank Maurer betreibt KMS Koblenzer Münzen Studio (Rathauspassage 20) — getrennte Dossiers beibehalten. Klärfall: Koblenz-Fakten requires Zweitbeleg (Register/Social/kommunal); Quelle(n): https://www.goldengate-koblenz.de/, https://www.goldengate-koblenz.de/kontakt/]
- [Korrektur 01.10.2026: Feedback #2982 berechtigt eingearbeitet — city „Koblenz 56068" zu „Koblenz" (PLZ-im-city-Formatfehler), street Entenpfuhl 12, postcode 56068, phone 0261 5501139 per Betreiber-Kontaktseite (Inh. Frank Maurer); E-Mail info@goldengate-koblenz.de nur Timeline (Feedback-Scope Straße/PLZ/Telefon, kein E-Mail-Fill); PERSONEN-LINK Frank Maurer (Golden Gate Entenpfuhl 12 vs. KMS Rathauspassage 20) — getrennte Dossiers beibehalten, kein Merge; Beleglage: Betreiber-Website als starke Einzelquelle (Einzelunternehmen ohne HRB — keine Owner-Ausnahme), offengelegt; Koordinaten neu zu geocodieren; Quelle(n): https://www.goldengate-koblenz.de/kontakt/ (Abruf 01.10.2026)]

### Recherche 31.10.2026

- [Recherche 31.10.2026: OSM-Adressprüfung live am Objekt, Raster bbox=7.5859663,50.3501251,7.6075255,50.3708549 (OSM-Way-BBox des gesamten Straßenverlaufs "Entenpfuhl", way/1248899606, plus 0,010 Grad Rand; in Kacheln abgetastet), Filter addr:street=Entenpfuhl mit addr:housenumber: **53 Objekte** | **33 verschiedene Hausnummern** (roh 1, 2-4, 5, 6, 8, 9, 10, 11, 11-13, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 23, 23-25, 24, 25, 26, 27, 28, 29, 30, 32, 33, 33-35, 37 — Bereichsnotationen mit der `expand()`-Funktion aus `scripts/release/osm_street_scan.py` einzeln aufgelöst; aufgelöst: 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 23, 24, 25, 26, 27, 28, 29, 30, 32, 33, 34, 35, 37). Die Dossier-Hausnummer **12 ist erfasst**: node/12046479898 trägt `addr:housenumber=12`, `addr:street=Entenpfuhl`, `name=Golden Gate`, `shop=jewelry`, `opening_hours=Mo-Sa 10:15-17:30; Su,PH off` — das ist ein **zweiter, vom Betreiber unabhängiger Beleg** für Name und Hausnummer; das Objekt trägt selbst `name=`, die Negativaussage der Vorwelle ist damit widerlegt. Im selben Haus sitzt außerdem node/12046479900 mit `addr:housenumber=12`, `addr:street=Entenpfuhl`, `name=Goldschmiede Faust`, `shop=jewelry` — die zweite Ladenzeile am selben Hausnummern-Eingang ist damit **nicht** der Dossier-Firma zuzuordnen; welche der beiden Ladentüren Golden Gate und welche Goldschmiede Faust belegt, ist offen. Von den 53 Adressobjekten der Straße tragen 22 ein `name=`- oder `operator=`-Tag; im gesamten Raster tragen 3577 Objekte `name=` oder `operator=`; Objekte mit Bezug zu **Schrott/Metall/Recycling/Edelmetall** sind dort **9** (Kriterium: `shop=gold_buyer` | `amenity=recycling` | `industrial=` | `landuse=industrial` | `recycling:` oder ein Name/Operator mit Schrott-/Recycling-Bezug): node/13648283754 `name=Koblenzer Goldankauf` mit `shop=gold_buyer`, node/13587502944 `name=Goldankauf Ziehl` mit `shop=gold_buyer` (beide **ohne** `addr:housenumber`, also nicht ortsbezogen), node/4982836415 `name=Schmuckkontor Krick GmbH` (Löhrstraße 2, `shop=jewelry`), node/12046479898 `name=Golden Gate` (Entenpfuhl 12, `shop=jewelry`), node/12046479900 `name=Goldschmiede Faust` (Entenpfuhl 12, `shop=jewelry`), node/3346137446 mit `operator=DRK-Kreisverband Koblenz-Stadt e.V.` und `amenity=recycling`, node/13954035216 mit `operator=Deutsches Rotes Kreuz` und `amenity=recycling`, node/12534758498 mit `operator=Wittmann` und `amenity=recycling` sowie way/34792238 `name=expert klein` (Am Saarplatz 1, `shop=electronics`). **Kein** Objekt im Raster trägt `name=Golden Gate` außer node/12046479898. Quelle(n): https://api.openstreetmap.org/api/0.6/node/12046479898.json ; https://api.openstreetmap.org/api/0.6/node/12046479900.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=7.5859663,50.3501251,7.6075255,50.3708549 (Abruf 31.10.2026)]
- [Recherche 31.10.2026: Preise getrennt nach Ankauf, Verkauf und Gebühren. **Ankauf:** Für Golden Gate ist **keine** Ankaufspreisliste und **kein** Euro-Betrag je Gramm oder Gramm Gold öffentlich einsehbar; die Leistungsseite „Goldankauf" (https://www.goldengate-koblenz.de/leistungen/goldankauf/) trägt **nur** Lorem-Ipsum-Fließtext und **keine** Preise. Auch die Startseite nennt nur „Wir zahlen faire Preise" ohne Zahl. **Verkauf:** Das Angebot ist „Souvenirs & Geschenkartikel" und „Kleinantiquitäten"; eine Preisliste des Onlineshops (Jimdo-Shop) existiert, die öffentliche Shop-Startseite zeigte jedoch keine Positionen mit Preisen. **Gebühren:** Die Liefer- und Zahlungsbedingungen des Jimdo-Shops nennen „Standardversandkosten Deutschland 0,00 €" und weisen darauf hin, dass bei Lieferungen ins Ausland Zölle, Steuern und Gebühren anfallen können, die im angezeigten Gesamtpreis nicht enthalten sind; die deutsche Lieferung ist damit gebührenfrei. Das Impressum (https://www.goldengate-koblenz.de/about/) bestätigt Inh. Frank Maurer und die E-Mail info@goldengate-koblenz.de; die Seite „Wir über uns" ist ausdrücklich noch im Aufbau. Offen bleibt, ob Frank Maurer die beiden Koblenzer Ladengeschäfte (Entenpfuhl 12 und Rathauspassage 20) gemeinsam oder getrennt betreibt und ob die Goldschmiede Faust am selben Eingang ein Nachbarbetrieb ist. Quelle(n): https://www.goldengate-koblenz.de/leistungen/goldankauf/ ; https://www.goldengate-koblenz.de/j/shop/deliveryinfo ; https://www.goldengate-koblenz.de/about/ (Abruf 31.10.2026)]
