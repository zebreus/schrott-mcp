---
slug: sl-st-ingbert-66386-rohstoffhandel-neckartilda-nl-st-ingbert
name: Rohstoffhandel Neckartilda GmbH, NL St. Ingbert
trader_type: sonstige
state: SL
city: St. Ingbert 66386
street: Schlackenbergstr. 23
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: http://www.neckartilda-rohstoffhandel.de
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Wide-Net (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Rohstoffhandel Neckartilda GmbH, NL St. Ingbert

## Überblick

Historischer Standortkandidat an der Schlackenbergstraße 23, getrennt vom
Bruchmühlbach-Miesauer Platz Industriestraße 1. Die weiterhin abrufbare
Betreiberseite nennt beide Plätze, aber ein veraltetes Impressum. Der zugehörige
Rechtsträger wurde nach Berlin übertragen und wird im Registerdatenprofil als
erloschen geführt. Heutiger Betreiber und Fortführung des St.-Ingbert-Platzes
bleiben offen; `pruefung` bleibt bestehen. Kein belegtes Platzschließungsdatum,
keine aktuell freigegebenen Annahme-, Kontakt- oder Öffnungszeitangaben.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Rohstoffhandel/Recycling — STAMMHAUS Bruchmühlbach-Miesau RLP (HRB Zweibrücken) — HQ-Caveat
- Adresse: St. Ingbert 66386, Schlackenbergstr. 23

### Nachprüfung 14.10.2026

- [Korrektur 14.10.2026: Die Betreiberseite nennt weiter Schlackenbergstraße 23, 66386 St. Ingbert und separat Industriestraße 1, 66892 Bruchmühlbach-Miesau. Das Impressum führt noch HRB 31272 Zweibrücken und Pascal Keßler. Die Registerkette führt seit 2020 nach Berlin, HRB 217698 B; NorthData kennzeichnet den Rechtsträger als erloschen und nennt 08.01.2025 als Löschungsereignis. Website-Erreichbarkeit ist kein aktueller Fortbestandsbeweis. Quelle: https://wk0s76uib.hier-im-netz.de/Impressum ; https://www.northdata.de/NECKARTILDA%20Rohstoffhandel%20GmbH,%20Berlin/Amtsgericht%20Charlottenburg%20(Berlin)%20HRB%20217698%20B]
- [Recherche 14.10.2026: Registerlöschung belegt keine Schließung beider Recyclingplätze; heutiger Betreiber bzw. Nachfolge und tatsächliche Standortfortführung sind gesondert zu klären. Status pruefung und stabiler Slug bleiben unverändert, keine Zusammenführung mit dem RP-Platz, keine aktiven Kontakt-/Annahmezusagen aus der alten Website übernommen. Feedback 7380 betrifft den separaten RP-Aufnahmekandidaten; historische Sammelmeldungen 1415/1423 bleiben erhalten. Vollständiger Prüfbericht: research/feedback-20261014-neckartilda-admission.md; Quelle: https://wk0s76uib.hier-im-netz.de/Impressum ; https://www.northdata.de/NECKARTILDA%20Rohstoffhandel%20GmbH,%20Berlin/Amtsgericht%20Charlottenburg%20(Berlin)%20HRB%20217698%20B]

### Recherche 31.10.2026

- [Recherche 31.10.2026: OSM-Adressprüfung live am Objekt, Raster bbox=7.0804023,49.2658596,7.1101498,49.2884712 (OSM-Way-BBox des gesamten Straßenverlaufs "Schlackenbergstraße", way/24404098 + way/24404094, plus 0,010 Grad Rand; in Kacheln abgetastet), Filter addr:street=Schlackenbergstraße mit addr:housenumber: **33 Objekte** | **29 verschiedene Hausnummern** (roh 1, 3, 5, 7, 13, 15, 19, 20, 20a, 21, 23, 25, 27, 29, 29a, 29b, 29c, 31, 33, 35, 35a, 37, 39, 39a, 41, 41a, 41b, 43, 43b — keine Bereichsnotation, daher unverändert aufgelöst). **Die Dossier-Hausnummer 23 ist erfasst, trägt aber einen fremden Firmennamen:** way/398009637 mit `addr:housenumber=23`, `addr:street=Schlackenbergstraße`, `addr:city=Sankt Ingbert`, `addr:postcode=66386`, `addr:country=DE`, `building=yes`, `name=Süd Müll` — das ist die Firmierung eines **anderen** Betriebs an der Dossier-Adresse, nicht „Rohstoffhandel Neckartilda": der Firmierungsname der Dossier-Firma ist an dieser Nummer in OSM **nicht** eingetragen. Die Nummernfolge auf der Schlackenbergstraße lässt **22** und **24** aus; belegt sind u.a. **21** in way/398009633 (ohne `name=`-Tag) und **25** in way/398009608 mit `name=Getränke Kohler`; die Nummern **20** und **20a** sind ebenfalls belegt (way/292832074 mit `name=Peter Gross Bau` und way/1320356115 mit `name=Green Campus`). Von den 33 Adressobjekten der Straße tragen 23 ein `name=`- oder `operator=`-Tag, **keines** davon mit Bezug zum Dossier (u.a. way/398009598 `name=Autocentrum Schlackenberg` an 1, way/292830664 `name=Kartonagen Schmidt` an 39, way/292830662 `name=ABEX Holzauer` an 27). Im gesamten Raster tragen 699 Objekte `name=` oder `operator=`; Objekte mit Bezug zu **Schrott/Metall/Recycling** sind dort **1** (Kriterium: `shop=gold_buyer` | `amenity=recycling` | `industrial=` | `landuse=industrial` | `recycling:` oder ein Name/Operator mit Schrott-/Recycling-Bezug): node/12758318001 mit `amenity=recycling`, `name=Wertstoffzentrum St. Ingbert`, `addr:street=Dudweilerstraße`, `addr:housenumber=19`, `addr:postcode=66386`, `opening_hours=Tu-We, Fr 12:00-17:00; Th 12:00-18:00; Sa 08:00-16:...` — ein **kommunales Wertstoffzentrum** in St. Ingbert, nicht der Dossier-Betrieb. **Kein Objekt im Raster trägt `name=Neckartilda` oder `name=Rohstoffhandel Neckartilda`:** der Betrieb ist in OSM an der Dossier-Adresse **nicht** kartiert; der an der Nummer 23 kartierte Name „Süd Müll" ist als **Fremdfirma** zu führen und **nicht** mit dem Dossier gleichzusetzen. Quelle(n): https://api.openstreetmap.org/api/0.6/way/398009637.json ; https://api.openstreetmap.org/api/0.6/way/398009608.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0804023,49.2658596,7.1101498,49.2884712 (Abruf 31.10.2026)]
- [Recherche 31.10.2026: Preise getrennt nach Ankauf, Verkauf und Gebühren. **Ankauf:** Die Betreiberseite nennt eine **Vergütung**, aber **keinen** Euro-Betrag: „Wir machen aus Abfall Geld … Wir nehmen Rohstoffe von Privathaushalte an, sortieren und verwerten sie. Durch diese Handlung haben die Bürger der umliegenden Verbandsgemeinden die Möglichkeit Ihren Abfall umweltgerecht zu entsorgen. **Sie bringen uns die unten aufgeführten Rohstoffe und wir zahlen dafür**; Sie liefern: **Altmetalle und Schrott, Altpapier und Kartonagen, Altkleider und Schuhe, Elektroschrott (außer Kühlgeräte und Fernseher)**. Wir wiegen bei uns vor Ort und zahlen **bar**." Die **Sortenliste** (Altmetalle/Schrott, Altpapier/Kartonagen, Altkleider/Schuhe, Elektroschrott ohne Kühlgeräte und Fernseher) und die **Zahlungsart bar** sind damit belegt, der **Preis je Kilogramm** nicht; die Seite „Über Uns" nennt zusätzlich die Öffnungszeiten Mo–Sa 08:00–12:00 und Mo–Do 13:00–16:00, die Seite „Öffnungszeiten St. Ingbert" nennt für den St.-Ingbert-Platz **Mo–Fr 8.00–12.30 und 13.00–16.00** — die beiden Angaben widersprechen sich in den Vormittagszeiten (08:00–12:00 vs. 08:00–12:30) und im Wochentag (Mo–Sa vs. Mo–Fr); die Diskrepanz bleibt offen und die Angaben werden nicht in die Frontmatter übernommen. Eine **Anmeldung ist für Gewerbetreibende verpflichtend**: „Nach den Richtlinien des KrWG dürfen nur Gewerbetreibende Elektroschrott abgeben" — die Elektroschrott-Annahme ist damit gewerblichen Anlieferern vorbehalten, Privathaushalte dürfen Elektroschrott **nicht** abgeben. **Verkauf:** Kein Verkauf von Sekundärrohstoffen ausgewiesen. **Gebühren:** Keine Gebühren-, Pauschal- oder Abholpreisangabe auffindbar; die im Impressum genannten Rufnummern (06894-892 52 70 und Fax 06894-892 52 69 für St. Ingbert, 06372-806 446 0 und Fax 06372-806 446 1 für Bruchmühlbach-Miesau) sind **Kontakt-, keine Preisangaben**; das Impressum führt weiterhin **HRB 31272 Amtsgericht Zweibrücken** und **Pascal Keßler** als Geschäftsführung — ein veralteter Stand, der zum erloschenen Rechtsträger der Vorwelle passt und den **heutigen Betreiber nicht belegt**. Quelle(n): https://wk0s76uib.hier-im-netz.de/Ueber-Uns ; https://wk0s76uib.hier-im-netz.de/Oeffnungszeiten-St-Ingbert ; https://wk0s76uib.hier-im-netz.de/Impressum (Abruf 31.10.2026)]
