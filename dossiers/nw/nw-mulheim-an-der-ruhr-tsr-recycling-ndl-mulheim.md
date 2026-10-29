---
slug: nw-mulheim-an-der-ruhr-tsr-recycling-ndl-mulheim
name: TSR Recycling (Ndl. Mülheim)
trader_type: sonstige
state: NW
city: Mülheim an der Ruhr
street: Steineshoffweg 21
postcode: '45479'
phone: 0208 999540
email: ''
opening_hours: 'Mo-Do 06:00-14:00; Fr 06:00-13:30'
website: https://www.tsr-recycling.de/standorte/schrotthaendler-muelheim/
website_status: blockiert
status: aktiv
certifications: '["Vom Betreiber auf der Standortseite veroeffentlichte Zertifikatsdokumente (PDF-Links, Abruf 29.10.2026): Entsorgungsfachbetrieb (EfB; gemeinsame Zertifikatsdatei laut Dateiname ‚EfB_Zertifikat_RheinRuhrII_Duisburg_Mülheim‚), Qualitätsmanagement DIN EN ISO 9001:2015, Energiemanagementsystem DIN EN ISO 50001:2018. Die verlinkten PDF-Dateien lieferten am 29.10.2026 beim Direktabruf HTTP 403 (Bot-Schutz der Seite), deshalb sind Zertifikatsnummern und Gueltigkeitsdauer hier NICHT belegt - die Liste ist die Zertifikatsbekanntgabe des Betreibers fuer diesen Standort."]'
description: 'TSR-Niederlassung Muelheim an der Ruhr (Steineshoffweg 21): industrieller Versorgungsschwerpunkt (Stahlwerke und Giessereien), Einkauf von Giesserei- und Schwerschrotten; kein Bareinkauf, keine Privatkunden, keine NE-Metalle.'
dropoff_json: '{"allowed": true, "customer_types": ["gewerbe"], "days": ["Mo", "Di", "Mi", "Do", "Fr"], "conditions": "Keine Privatkundenannahme; kein An-/Verkauf von Autoteilen, Altfahrzeugen, NE-Metallen; kein Bareinkauf (TSR-Standortseite, 29.10.2026)"}'
pickup_json: ''
provenance_section: Neu, Ankauf verifiziert (ja)
provenance_ankauf_raw: ja (B2B)
provenance_origin: table
---

# TSR Recycling (Ndl. Mülheim)

## Überblick

TSR-Niederlassung in Mülheim an der Ruhr (Steineshoffweg 21, 45479). Industrieller Versorgungsschwerpunkt: Einkauf von Gießerei- und Schwerschrotten für Stahlwerke und Gießereien in ganz Europa, dazu Lohnzerkleinerung nach Vorgabe (ohne Preisangabe). Keine Privatkundenannahme, keine NE-Metalle, kein Bareinkauf. Öffnungszeiten Mo-Do 06:00-14:00, Fr 06:00-13:30.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Steineshoffweg 21 — weiterer TSR-Standort

### Korrektur 30.09.2026 (Feedback-Triage)

- [Korrektur Feedback fb1846: street bereinigt („Mülheim an der Ruhr Steineshoffweg 21" enthielt Stadt im Straßenfeld) → Steineshoffweg 21, 45479 Mülheim an der Ruhr + Tel. 0208 999540 vierquellig (TSR-Standortseite via Jina-Reader: T +49 208 999540 + golocal + schrottplaetze.org + Gelbe Seiten „Steineshoffweg 21, 45479, 0208 99954-0"); Quelle: tsr-recycling.de/standorte/schrotthaendler-muelheim + golocal.de/muelheim/schrotthandel/tsr-recycling-niederlassung-muelheim-gCCU + schrottplaetze.org/schrottplatz-muelheim-ruhr-tsr-recycling]
- [Korrektur Feedback fb1846: Annahmebedingungen mit Website-Beleg — TSR-Seite: „Keine Privatkundenannahme. Kein An- und Verkauf von Autoteilen, Altfahrzeugen, NE-Metallen." Fokus industrielle Versorgung (Stahlwerke/Gießereien) → dropoff_json (gewerbe only); „kein Bareinkauf" steht dort NICHT und wird nicht behauptet; Quelle: TSR-Standortseite (30.09.2026)]
- [Korrektur Feedback fb1846: website_status blockiert (tsr.eu + tsr-recycling.de mit Human-Challenge, verifiziert 30.09.2026); Quelle: Direktabruf]
- (Klärfall): Öffnungszeiten widersprüchlich — TSR-Seite Mo–Do 06:00–14:00/Fr 06:00–13:30 vs. schrottplaetze.org Mo–Do 06:00–16:30/Fr 15:00 → opening_hours leer gelassen.

### Recherche 29.10.2026

- [Recherche 29.10.2026: Standortseite Muelheim am 29.10.2026 ueber den Jina-Reader erneut live gelesen (Direktabruf von tsr.eu/tsr-recycling.de = Bot-Challenge, `website_status: blockiert` bestaetigt). **Website-Feld praezisiert** auf die standortspezifische Branchenseite (analog Gelsenkirchen/Duisburg/Dortmund); `tsr.eu` ist die Gruppen-Seite, nicht der Standort. Woertlich belegt: „TSR Deutschland GmbH & Co. KG, Steineshoffweg 21, 45479 Muelheim a. d. Ruhr", „T +49 208 999540", „F +49 208 9995444", „Oeffnungszeiten: Mo-Do: 06:00 - 14:00 Fr: 06:00 - 13:30". **Anmerkung woertlich neu belegt**: „Bitte beachten Sie: Am Standort Muelheim werden kein Schrott von Privatkunden und keine NE-Metalle angenommen. Es erfolgt **kein Bareinkauf**." - die Aussage „kein Bareinkauf" stand am 30.09.2026 noch **nicht** auf der Seite und wurde deshalb nicht behauptet; am 29.10.2026 steht sie dort woertlich, das `dropoff_json` ist entsprechend ergaenzt (die fruehere Klaerfall-Notiz bleibt unveraendert stehen). **Oeffnungszeiten** jetzt aus der Betreiber-Primärquelle gefuellt; die Aggregator-Abweichung (schrottplaetze.org Mo-Do 06:00-16:30/Fr 15:00) ist ein Lead und unterliegt dem Betreiberwert. **Standortprofil**: „industrielle Versorgung ... Wir beliefern Stahlwerke und Giessereien in ganz Europa mit Recyclingrohstoffen und kaufen die entsprechenden Qualitaeten europaweit ein. Im Mittelpunkt stehen Giessereischrotte und Schwerschrotte, zum Beispiel Guss, Walzen, Baeren sowie weitere schwere Eisenqualitaeten"; Leistungen: „Europaweites Netzwerk", „Definierte Stueckgroessen", „Lohnzerkleinerung" („Wir bieten Lohnzerkleinerung nach Vorgabe, inkl. Sortierung und Siebung (je nach Material) an" - **kostenpflichtige Dienstleistung, aber ohne Preisangabe; kein Tarif belegt**). **ANKAUF**: Giesserei- und Schwerschrotte, NE-Metalle ausdruecklich nicht; **VERKAUF**: aufbereitete Recyclingrohstoffe europaweit; **GEBUEHREN**: Lohnzerkleinerung/Containerdienst ohne Preis, kein Tariffund. Quelle(n): https://www.tsr-recycling.de/standorte/schrotthaendler-muelheim/ (Abruf 29.10.2026); https://www.tsr-recycling.de/kunden/schrottlieferanten/ (Abruf 29.10.2026).]

- [Recherche 29.10.2026 (Zertifikats-Nachtrag): Die Standortseite Muelheim veroeffentlicht **drei** Zertifikatsdokumente mit PDF-Link: „Entsorgungsfachbetrieb", „Qualitaetsmanagement gemaess DIN EN ISO9001:2015", „Energiemanagementsystem gemaess DIN EN ISO 50001:2018". Der EfB-Dateiname lautet „EfB_Zertifikat_RheinRuhrII_Duisburg_Mülheim_11052027.pdf" — das Zertifikat deckt **Duisburg und Muelheim** gemeinsam ab. Die PDF-Dateien lieferten am 29.10.2026 beim Direktabruf HTTP 403; Zertifikatsnummern und Gueltigkeitsdauer sind daher **nicht** belegt und werden nicht behauptet. Quelle(n): https://www.tsr-recycling.de/standorte/schrotthaendler-muelheim/ (Abruf 29.10.2026).]

- [Recherche 29.10.2026 (GEO-Abgleich): **Der in der Produktionsdatenbank gespeicherte Punkt stimmt nicht mit der belegten Adresse ueberein** — gespeichert `51.43217 / 6.88176`; Rueckwaertsgeokodierung dieses Punktes (Photon-Reverse ueber OSM) ergibt `Amtsgericht / Tourainer Ring, 45468 Mülheim an der Ruhr (Altstadt II), OSM way 1201409439`. Das in OSM gefuehrte Adressobjekt der belegten Anschrift liegt `1.303 m` vom gespeicherten Punkt entfernt: OSM way 102214851 (`addr:housenumber=21`, `addr:street=Steineshoffweg`, `addr:city=Mülheim an der Ruhr`, `addr:postcode=45479`, `building=yes`). **Konsequenz**: der gespeicherte Punkt wird hier **nicht** automatisch zurueckgezogen — die Auswahl der `clear_fields`-Direktive bzw. einer Ersatzkoordinate ist eine Owner-Entscheidung, und fuer eine **neue** Koordinate liegen in diesem Dossier nur **eine** Positionsquelle (OSM) vor, waehrend die Regel eine zweite unabhaengige Quelle verlangt. **Keine Koordinate gesetzt.** Der gespeicherte Punkt liegt zudem im **falschen Postleitzahlbereich** (45468 statt 45479). Quelle(n): Photon-Reverse (https://photon.komoot.io/reverse, Abruf 29.10.2026); OSM-API-Abfrage `api.openstreetmap.org/api/0.6/way/<id>` bzw. `/node/<id>` (Abruf 29.10.2026); Haversine-Berechnung (R = 6371 km).]
