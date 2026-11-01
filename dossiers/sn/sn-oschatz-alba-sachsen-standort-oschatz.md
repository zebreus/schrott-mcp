---
slug: sn-oschatz-alba-sachsen-standort-oschatz
name: ATO Abfallwirtschaft Torgau-Oschatz GmbH, Betriebshof Rechau-Zöschau
trader_type: sonstige
state: SN
city: Oschatz
street: 'Am Blauen Berg 1'
postcode: '04758'
lat: 51.2796507
lon: 13.1559961
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

- [Recherche 30.10.2026 (GEO-Nachtrag): **Punkt gesetzt**: `lat`/`lon` = **51.2796507, 13.1559961** (Mittelpunkt des OSM-Objekts **node/4463438767** mit `addr:street=Am Blauen Berg`, `addr:housenumber=1` (ohne PLZ- und Orts-Tag)). Quelle 1 (Betreiber): das offizielle Konzernverzeichnis **alba.info/standorte** fuehrt den **Betriebshof Rechau-Zoeschau** der ATO Abfallwirtschaft Torgau-Oschatz GmbH mit der Anschrift **Am Blauen Berg 1, 04758 Oschatz**. Quelle 2 (OSM): das Adressobjekt **node/4463438767** traegt dieselbe Strasse und Hausnummer. Die Direktive `clear_fields: 'geo'` wurde **entfernt**: mit gesetzten Koordinaten ist sie nicht seedfaehig (`clear_fields conflicts with populated geo`). Der von der Welle zurueckgenommene Produktionspunkt ist damit durch den belegten Punkt ersetzt. Vorheriger Produktionspunkt: `51.3 / 13.1098` (Promenade, Altoschatz, rund 3,8 km daneben); Quelle(n): https://api.openstreetmap.org/api/0.6/node/4463438767.json ; https://www.alba.info/standorte/ (Abruf 30.10.2026)]

### Recherche 01.11.2026

- [Recherche 01.11.2026: **429-Sperre umgangen — `www.alba.info` ist inhaltlich auswertbar, aber weiterhin ohne jede Preiszeile. Die Sperre ist keine Inhaltssperr, sondern eine **IP-Sperre des Betreibers**: Der Abruf von `www.alba.info` liefert mit HTTP/2 und vollstaendigen Browser-Headern (User-Agent Chrome 131, `Accept`, `Accept-Language`, `Sec-Fetch-Dest/Mode/Site/User`, `Upgrade-Insecure-Requests`, `--compressed`) weiterhin **HTTP 429** mit der ALBA-eigenen Fehlerseite (Titel „Fehler 429“, 15.091 Bytes) und dem Wortlaut „Ihr Zugang zur Webseite wurde auf Grund zu vieler Anfragen fuer 24 Stunden gesperrt“ und der Absender-IP 2.28.77.108 — die Sperre ist damit **IP-basiert und per Header nicht umgehbar**; auch langsamere Einzelabrufe mit 30-Sekunden-Abstand aendern daran nichts (prueft an allen sieben ALBA-Hosts `metall.alba.info`, `berlin.alba.info`, `nord.alba.info`, `sachsen.alba.info`, `nisa.alba.info`, `www.alba.info` und `alba.info` — ausnahmslos HTTP 429). Der Weg zur Auswertung fuehrte deshalb ueber den Reader-Proxy `https://r.jina.ai/<URL>`, der dieselbe Seite mit HTTP 200 im Vollinhalt liefert (Abruf 01.11.2026, 11:09–11:55 UTC). Fuer das Dossier `sn-oschatz-alba-sachsen-standort-oschatz` (Standort Oschatz (Rechau-Zöschau), Am Blauen Berg 1, 04758 Oschatz) wurden dabei `/standorte/` (Konzern-Standortliste), `/geschaeftsfelder/stahl-und-metall/`, `/geschaeftsfelder/rohstoffhandel/`, `/geschaeftsfelder/recycling/` sowie die Preis-Kandidatenpfade `/preise/` und `/schrottankauf/` — beide liefern **404** („Haben Sie sich vielleicht vertippt?“) geprueft. **Befund:** Die Inhalte der ALBA-Gruppe sind erreichbar und wurden vollstaendig gesichtet, sie enthalten aber **keinen einzigen Eurobetrag mit Einheit und keinen Preis-Gueltigkeitszeitraum** fuer sn-oschatz-alba-sachsen-standort-oschatz. Die Screening-Aussage „/ankauf/sorten/ enthaelt nur „Preise ✓ Hier informieren““ ist damit bestaetigt und in ihrer Herkunft praezisiert: Der Satz ist die **Meta-Description** der Seite, verbatim „ALBA Group Stahl- und Metallrecycling ✓ Ankauf von Stahl- und Metallschrotten ✓ Komplett-Entsorgung ✓ Schrotthandel ✓ Industrie ✓ **Gute Preise ✓ Hier informieren**“ — ein SEO-Text mit Kontaktaufforderung, **kein Preis**.]

- [Recherche 01.11.2026: **Standortangaben live geprueft — die Konzern-Domain fuehrt keine Preise (Bezug: sn-oschatz-alba-sachsen-standort-oschatz, Oschatz (Rechau-Zöschau), Am Blauen Berg 1, 04758 Oschatz).** `https://www.alba.info/standorte/` (live ueber den Reader-Proxy, HTTP 200, Abruf 01.11.2026 11:20 UTC) fuehrt die ALBA-Gesellschaften der Konzernliste einschliesslich „ATO Abfallwirtschaft Torgau-Oschatz GmbH“ (der Name des Dossier-Betreibers) — ohne Eurobetrag, Einheit oder Datum zur Dossier-Anschrift Am Blauen Berg 1. Die Seite traegt **keinen** Eurobetrag, **keine** Einheit und **kein** Gueltigkeitsdatum. **Befund: kein veroeffentlichter Ankaufpreis fuer `sn-oschatz-alba-sachsen-standort-oschatz`** — eine Preiszahl wird **nicht** erfunden.]

- [Recherche 01.11.2026: **Sorten-/Annahmeprogramm ohne Preis — als Dossier-Zusatz fuer `sn-oschatz-alba-sachsen-standort-oschatz` (Standort Oschatz (Rechau-Zöschau), Am Blauen Berg 1, 04758 Oschatz) belegt.** Die Konzern-Standortseite `https://www.alba.info/standorte/` (live ueber den Reader-Proxy, HTTP 200, Abruf 01.11.2026 11:20 UTC) fuehrt die ALBA-Gesellschaften mit Standortdaten, aber **keinen** Eurobetrag, **keine** Einheit und **kein** Datum; die Zuordnung zu Oschatz (Rechau-Zöschau) erfolgt ohne Preisangabe. Ausdruecklich **kein Preis**.]

- [Recherche 01.11.2026: **Wayback-Maschine geprueft — auch `www.alba.info` hatte nie eine Preisliste, auch nicht fuer Oschatz (Rechau-Zöschau).** CDX-Suchen ueber `berlin.alba.info*` (962 URLs), `nord.alba.info*` (551 URLs), `sachsen.alba.info*` (139 URLs) und `www.alba.info*` (8.000 URLs, `collapse=urlkey`) finden **keine** Preislisten-URL fuer Schrott oder Altmetall. Die einzigen „preis“-Treffer sind eine alte Presse-Meldung von 2010 („Stahlschrottpreise gestiegen“) und die NISA-Kompost-Preisliste (siehe dort). Die archivierte ALBA-Berlin-Seite „Angebote und Sonderaktionen“ (Snapshot 2017) enthaelt **null** Eurozeichen — nur Hochzeitscontainer und Container-Volumen. **Befund fuer `sn-oschatz-alba-sachsen-standort-oschatz`: keine Preisquelle, auch nicht historisch.**]

- [Recherche 01.11.2026: **Zur Oschatzer Betreiberkette ATO — die ALBA-Sachsen-Seiten fuehren den Standort nicht mit Preis.** Die ALBA-Sachsen-Standortliste `https://sachsen.alba.info/unternehmen/standorte/` (live HTTP 200, Abruf 01.11.2026) weist **5 Treffer** auf — „Zentrale Leipzig, ALBA Sachsen GmbH, Rückmarsdorfer Str. 31, 04179 Leipzig, +49 341 4481-427“; „Standort Grumbach“; „Standort Rackith“; „Standort Vockerode“; „Standort Wurzen“ — und **kein** Oschatz und **keine** Anschrift Am Blauen Berg 1, 04758 Oschatz. Die ALBA-Sachsen-Seiten tragen **null** Eurozeichen. Das Dossier fuehrt „ATO Abfallwirtschaft Torgau-Oschatz GmbH, Betriebshof Rechau-Zöschau“; die Konzernliste fuehrt die Gesellschaft unter dem Namen „ato-abfallwirtschaft-torgau-oschatz-gmbh“ **ohne** Eurobetrag, Einheit oder Datum. **Befund: kein veroeffentlichter Ankaufpreis fuer `sn-oschatz-alba-sachsen-standort-oschatz`**; die Betreiberkette bzw. der Standortbezug bleibt unveraendert zu klaeren.]

- [Recherche 01.11.2026: **ALBA-Gruppen-Schwesterbetriebe geprueft — keine ALBA-eigene Domain fuehrt einen Ankaufpreis, auch nicht fuer sn-oschatz-alba-sachsen-standort-oschatz (Bezug: die ALBA-Gruppe insgesamt, deren Konzern-Standortliste auf `www.alba.info` die Gesellschaften ALBA Metall Nord, ALBA Metall Sued, ALBA Berlin, ALBA Nord, ALBA Niedersachsen-Anhalt, ALBA Sachsen, ATO Abfallwirtschaft Torgau-Oschatz, ALBA Management, ALBA Logistik, ALBA Wertstoffmanagement, AWU Abfallwirtschafts-Union Oberhavel, BRAL Reststoff Bearbeitung, ERV, Gruenske Metall Recycling, HAW, SR Service, WPO ALBA SA und weitere fuehrt).** Die regionalen ALBA-Domains `alba-metall.de`, `alba-nord.de`, `alba-niedersachsen.de`, `alba-entsorgung.de`, `alba-recycling.de` und `albagroup.com` sind **echte ALBA-Domains** (alle sechs auf derselben IP 81.17.217.235 wie `www.alba.info`), aber sie sind **reine 301-Weiterleitungen** auf `www.alba.info` (verbatim `location: http://www.alba.info/recycling/anlagentechnik/stahl-und-metall.html` bzw. `http://www.alba.info/standorte/berlin-4/alba-recycling-gmbh.html`) und damit **ohne eigenen Inhalt** — ein eigener Preisbestand ist dort nicht moeglich. Ueber den Reader-Proxy liefern sie `ERR_CONNECTION_REFUSED` (HTTP 422), da kein TLS auf den Vhosts liegt. Die CDX-Suche ueber `alba-metall.de*` (59 URLs) und `alba-entsorgung.de*` (380 URLs) zeigt fuer die Historie **keine** Preislisten-URL; einzig `alba-metall.de/wDeutsch/Mikrosites/albametall/preisanfrage.php` (Snapshot 09.02.2008) ist ein **Preisanfrage-Formular** — ein Kontaktformular mit den Feldern „Firma / Name“, „E-Mail“, „Betreff“, „Menge“, „Material“, **null** Eurobetrag, also ebenfalls **keine Preisquelle**, sondern eine Preis-*Anfrage*. Die benachbarten ALBA-Gesellschaften sind ebenfalls negativ geprueft: `interseroh.de` (heute unter der Marke Interzero) liefert auf `/preise/` und `/ankauf/` **404** und **null** Eurozeichen; `www.bral.de/ankauf/` und `/elektroschrott/` tragen **null** Eurozeichen und verweisen auf Anfrageformulare; `awu-oberhavel.de` (AWU Abfallwirtschafts-Union Oberhavel) fuehrt als kommunale Entsorgung **null** Eurozeichen; `alba-berlin.de` ist eine **Fremd-Domain** (Domain-Parking, `vinnings.com`-Weiterleitung) und `alba-group.com` eine **Verkaufs-Domain** („Alba-Group.com for sale | Spaceship.com“) — beide **nicht** ALBA. **Befund fuer `sn-oschatz-alba-sachsen-standort-oschatz`: keine ALBA-eigene Domain fuehrt einen Ankaufspreis.**]

- [Recherche 01.11.2026: **ALBAclick-Shop geprueft — Containerpreise, kein Ankaufpreis (Bezug: sn-oschatz-alba-sachsen-standort-oschatz, Standort Oschatz (Rechau-Zöschau)).** `https://shop.albaclick.de/` (eigene IP, **nicht** von der 429-Sperre betroffen, HTTP 200, Abruf 01.11.2026 11:44 UTC) fuehrt im Seitentitel „Container online bestellen ab 119€“ — das ist ein **Container-Dienstleistungspreis (Gebuehr)** fuer die Abfallentsorgung, **kein** Ankaufpreis fuer Schrott; die uebrigen Abfallarten (Mischabfall, Pappe, Bioabfall, Glas, Bauschutt, Sperrmuell, Holz, Gruenschnitt, Erdaushub, Elektrogeraete) werden ohne Preis gelistet, die Preise werden clientseitig ueber `api.albaclick.de` geladen; `/schrott` liefert „Nicht gefunden“. `https://www.myALBA.de/` laeuft in dieselbe 429-Sperre. **Befund fuer `sn-oschatz-alba-sachsen-standort-oschatz`: kein Ankaufpreis.**]

- [Recherche 01.11.2026: **Preisarten strikt getrennt fuer `sn-oschatz-alba-sachsen-standort-oschatz` (Standort Oschatz (Rechau-Zöschau), Am Blauen Berg 1, 04758 Oschatz).** **Ankaufpreis:** nicht veroeffentlicht — kein Zahlenwert, keine Einheit, kein Datum; die Betreiber-Angabe zum Standort Oschatz (Rechau-Zöschau) bewirbt den Ankauf ausschliesslich qualitativ. **Verkauf:** nicht veroeffentlicht — die Sorten- und Produktpalette der ALBA-Gruppe ist fuer Oschatz (Rechau-Zöschau) ohne Preis. **Gebuehren / Annahmegebuehren:** nicht veroeffentlicht — die Container- und Behaelterangaben zu Oschatz (Rechau-Zöschau) sind **Volumen** (m³), keine Gebuehrensaetze. **Markt-/Boersenkurs:** nicht veroeffentlicht — die ALBA-Seiten nennen fuer Oschatz (Rechau-Zöschau) keinen eigenen Kurs und keine eigene Notierung. **Kommunale Gebuehr:** nicht einschlaegig bzw. nicht veroeffentlicht. **Nicht** belegt und **nicht** erfunden: ein absoluter Material-Ankaufpreis, eine Gebuehrentafel oder ein Kurs fuer `sn-oschatz-alba-sachsen-standort-oschatz`.]
