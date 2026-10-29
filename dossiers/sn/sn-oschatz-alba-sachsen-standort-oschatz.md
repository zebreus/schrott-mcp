---
slug: sn-oschatz-alba-sachsen-standort-oschatz
name: ATO Abfallwirtschaft Torgau-Oschatz GmbH, Betriebshof Rechau-Zöschau
trader_type: sonstige
state: SN
city: Oschatz
street: 'Am Blauen Berg 1'
postcode: '04758'
phone: '+49 3435 921-359'
email: ''
opening_hours: ''
website: https://www.alba.info
website_status: 'aktiv'
status: pruefung
description: 'Betriebshof Rechau-Zoeschau der ATO Abfallwirtschaft Torgau-Oschatz GmbH, Am Blauen Berg 1, 04758 Oschatz, T +49 3435 921-359; Konzernzentrale Gewerbering 51, 04860 Torgau, +49 3421 77300-11. Der Eintrag stammt aus dem offiziellen Konzernverzeichnis alba.info/standorte (Abruf 29.10.2026); eine Einzel-Detailseite des Standorts existiert dort nicht. Der Seed-Name ALBA Sachsen Standort Oschatz war eine Fehlzuordnung: ALBA Sachsen GmbH fuehrt keinen Standort Oschatz. ANKAUF: kein Ankauf von Schrott oder Metallen fuer diesen Standort belegt; die Seed-Notiz Lagern ist durch keine Ankaufszusage gedeckt. VERKAUF: keine Verkaufspreisliste. GEBUEHREN: keine Tarife; kommunale Abfallgebuehren sind keine Betreibergebuehren und werden hier nicht gefuehrt.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
clear_fields: 'geo'
---

# ATO Abfallwirtschaft Torgau-Oschatz GmbH, Betriebshof Rechau-Zöschau

## Überblick

Die Identitaetskorrektur vom 01.10.2026 (der Seed-Name „ALBA Sachsen Standort Oschatz" war
eine **Fehlzuordnung**) ist bestaetigt und um die Konzernangaben ergaenzt.

- **Betreiber:** **ATO Abfallwirtschaft Torgau-Oschatz GmbH**. Die offizielle
  Standortuebersicht `alba.info/standorte` fuehrt zwei Eintraege dieser Gesellschaft:
  **Zentrale Gewerbering 51, 04860 Torgau, +49 3421 77300-11** und
  **Betriebshof Rechau-Zoeschau, Am Blauen Berg 1, 04758 Oschatz, +49 3435 921-359**.
  → Die Seed-Adresse und das Seed-Telefon sind damit **zweifach im amtlichen Konzernverzeichnis
  bestaetigt**; die Dossier-Bezeichnung „ALBA Sachsen GmbH, Standort Oschatz" war falsch, weil
  **ALBA Sachsen GmbH keinen Standort Oschatz fuehrt** (der Konzern fuehrt ALBA Sachsen GmbH
  mit Standorten in Leipzig, Grumbach, Rackith/Kemberg, Vockerode und Wurzen).
- **Kein Detail-Filet:** Fuer die Gesellschaft existiert auf `alba.info` **keine
  Einzel-Detailseite** (die Versuche `/unternehmen/gesellschaften/ato-...` und
  `/standorte/ato-...-betriebshof-rechau-zoschau` liefern HTTP 404). Der Eintrag ist ein
  **Eintrag im Konzernverzeichnis**, keine Standortseite mit Leistungsbeschreibung. Der
  Kontakt laeuft daher ueber die Konzernzentrale bzw. die genannte Betriebshof-Nummer.
- **Amtliches Fachbetrieberegister (Auszug 11.08.2026):** Fuer 04758 wird **ein** Betrieb
  gefuehrt: **„K&U Kanalreinigung und Umweltschutz Reimann GmbH, Wermsdorfer Straße 24"**
  (Sammeln, Befoerdern). Der Betriebshof Rechau-Zoeschau ist **nicht** im Register - fuer einen
  Abfallwirtschaftsbetrieb ohne eigenes Entsorgungsfachbetriebszertifikat ist das erwartbar
  und **kein Negativbefund**.

**Preise (Ankauf / Verkauf / Gebuehren) getrennt:**

- **Ankauf:** **kein Ankauf belegt.** Weder das Konzernverzeichnis noch eine Standortseite
  nennen einen Ankauf von Schrott oder Metallen fuer den Betriebshof Rechau-Zoeschau; die
  Seed-Notiz „Lagern" ist damit **nicht** durch eine Ankaufszusage gedeckt. Der Konzern
  ALBA betreibt zwar Rohstoffhandel als Geschaeftsfeld, **fuer diesen Standort ist das
  jedoch nicht belegt**.
- **Verkauf:** keine Verkaufspreisliste belegt.
- **Gebuehren:** keine Tarife belegt. **Wichtig:** Abfallgebuehren der Kommunen Torgau/Oschatz
  sind **kommunale Tarife** und keine Betreibergebuehren; sie werden hier nicht als Gebuehren
  des Handlers gefuehrt.

**GEO:** Der Produktionspunkt 51.3/13.1098 loest auf „14e, Promenade, Altoschatz, 04758
Oschatz" auf, also auf eine **andere Strasse rund 3,8 km suedwestlich** der belegten
Anschrift. Der Punkt wird zurueckgenommen; es wird **kein** Punkt geraten. Zu geokodieren
ist: **Am Blauen Berg 1, 04758 Oschatz (Betriebshof Rechau-Zoeschau)**.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Lagern
- schrottradar-Profil

### Korrektur 01.10.2026 (ALBA-Fix, Identitätskorrektur — Dossier NICHT gelöscht)

- Betreiber-Befund: ALBA Sachsen GmbH führt auf sachsen.alba.info KEINEN Standort Oschatz (eigene Standorte: Leipzig, Grumbach, Rackith/Kemberg, Vockerode, Wurzen). Oschatz läuft über die Beteiligungsgesellschaft ATO Abfallwirtschaft Torgau-Oschatz GmbH: Betriebshof Rechau-Zöschau, Am Blauen Berg 1, 04758 Oschatz, +49 3435 921-359 (lt. Konzern-Übersicht alba.info/standorte).
- Frontmatter auf diesen Stand korrigiert (name ATO…, status bleibt pruefung, website alba.info ohne Status, da kein direkter ALBA-Sachsen-Beleg). Seed-Annahme „ALBA Sachsen Standort Oschatz“ war eine Fehlzuordnung.
- Quelle: https://www.alba.info/standorte (Einträge ALBA Sachsen GmbH + ATO Abfallwirtschaft Torgau-Oschatz GmbH).

### Recherche 29.10.2026 (Tiefenrecherche-Welle 18:00, Shard C)

- [Recherche 29.10.2026: **Konzernverzeichnis bestaetigt die Identitaetskorrektur vom 01.10.2026 und liefert die Konzernangaben.** Die offizielle Standortuebersicht `alba.info/standorte` (Abruf 29.10.2026) fuehrt die **ATO Abfallwirtschaft Torgau-Oschatz GmbH** mit **zwei** Eintraegen: **Zentrale, Gewerbering 51, 04860 Torgau, +49 3421 77300-11** und **Betriebshof Rechau-Zoeschau, Am Blauen Berg 1, 04758 Oschatz, +49 3435 921-359**. Damit sind Adresse und Telefon des Dossiers **zweifach im amtlichen Konzernverzeichnis bestaetigt**; die Zuordnung „ALBA Sachsen GmbH, Standort Oschatz" aus dem Seed war eine **Fehlzuordnung** (ALBA Sachsen GmbH fuehrt keine Ortsseite Oschatz, sondern Leipzig, Grumbach, Rackith/Kemberg, Vockerode und Wurzen). Fuer die Gesellschaft gibt es auf `alba.info` **keine Detailseite** (`/unternehmen/gesellschaften/ato-...` und `/standorte/ato-...-betriebshof-rechau-zoschau` liefern HTTP 404) - der Eintrag ist ein **Konzernverzeichnis-Eintrag**, keine Standortseite; `website_status` wird deshalb auf **`aktiv`** gesetzt (die Konzerndomain ist erreichbar und fuehrt den Eintrag), der Eintrag aber ausdruecklich als solcher gekennzeichnet. Das **amtliche Fachbetrieberegister** (Auszug 11.08.2026) fuehrt fuer 04758 **nur** „K&U Kanalreinigung und Umweltschutz Reimann GmbH, Wermsdorfer Straße 24"; der Betriebshof Rechau-Zoeschau ist **nicht** im Register - fuer einen Abfallwirtschaftsbetrieb ohne eigenes EFB-Zertifikat erwartbar und **kein Negativbefund**. Frontmatter: `website_status` -> `aktiv`, `description` **neu gesetzt**; `name`, `street`, `postcode` und `phone` bleiben unveraendert (bereits belegt). **Ankauf: kein Ankauf belegt** - die Seed-Notiz „Lagern" ist durch keine Ankaufszusage gedeckt. **Verkauf: keine Preisliste. Gebuehren: keine Tarife**; Abfallgebuehren der Kommune sind **kommunale Tarife und keine Betreibergebuehren** und werden hier nicht als Gebuehren gefuehrt. Quelle(n): https://www.alba.info/standorte/ ; https://schrottregister.pages.dev/ort-oschatz ; read-only `public.db` (29.10.2026).]
- [Recherche 29.10.2026: **GEO - Produktionspunkt zurueckgenommen.** Der DB-Punkt 51.3/13.1098 loest per Nominatim-Reverse auf „14e, Promenade, Altoschatz, 04758 Oschatz" auf, also auf eine **andere Strasse rund 3,8 km suedwestlich** der belegten Anschrift Am Blauen Berg 1. `clear_fields: 'geo'` wird gesetzt, **kein** Punkt geraten. Zu geokodieren: **Am Blauen Berg 1, 04758 Oschatz (Betriebshof Rechau-Zoeschau)**. Quelle(n): https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.3&lon=13.1098 ; read-only `public.db` (29.10.2026).]

- [Recherche 29.10.2026: **OSM live geprueft (Am Blauen Berg, Oschatz).** Kachelraster ueber den **gesamten** Strassenverlauf (Strassenbbox 13.1549/51.2785 bis 13.1653/51.2845, **6 Kacheln, 0 Kachel-Fehler**). Zaehlung getrennt: **1 Adressobjekte / 1 verschiedene Hausnummern** auf der Am Blauen Berg (1), im gesamten Rastergebiet **1 Adressobjekte mit Hausnummer / 1 verschiedene Hausnummern**. **Hausnummer 1 existiert**: node/4463438767 mit `addr:housenumber=1`, `addr:street=Am Blauen Berg` - **kein** `name`-Tag - **kein** `operator`-Tag Ein Objekt mit einem `name`- oder `operator`-Bezug zum Betreiber ist im abgetasteten Raster **nicht** vorhanden; das ist eine Aussage zum OSM-Bestand, kein Negativbeleg ueber den Betrieb. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (bbox-Raster) ; https://api.openstreetmap.org/api/0.6/<objekt>.json ; read-only `public.db` (29.10.2026).]
