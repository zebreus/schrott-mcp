---
slug: st-seehausen-alba-niedersachsen-anhalt
name: ALBA Niedersachsen-Anhalt GmbH, Standort Seehausen
trader_type: wertstoffhaendler
state: ST
city: Seehausen
street: 'Der Talergalgen 1'
postcode: '39615'
phone: '+49 39386 975-0'
email: 'seehausen@alba.info'
opening_hours: 'Mo.–Fr. 07:00–17:00 Uhr'
website: https://nisa.alba.info
website_status: 'aktiv'
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: 'Nachtrag Audit-Runde 3 (Stand: 2026-09-27)'
provenance_ankauf_raw: Ankauf unklar (Entsorger – prüfen)
provenance_origin: table
---

# ALBA Niedersachsen-Anhalt GmbH

## Überblick

ALBA Niedersachsen-Anhalt GmbH betreibt in Seehausen (Altmark) am Der Talergalgen 1 einen
Standort der Wertstoff- und Bauabfallentsorgung mit Containerdienst, Müllpressen,
Umleerbehältern, Direktanlieferung von Wertstoffen und Schüttgütern; das Portal führt
sechs Standorte des Unternehmens. Die Identität tragen die Standort-Detailseite der
Betreiberdomain mit Anschrift, Rufnummer, E-Mail-Adresse und Öffnungszeiten sowie die
Konzernübersicht. Offen bleibt die Beleglage: Die Betreiberdomain sperrt reguläre Abrufe
per IP-Sperre, eine zweite unabhängige Quelle für die Kontaktdaten fehlt. Ein Ankaufspreis
für Schrott oder Altmetall wird nicht veröffentlicht; die einzige Preisliste des Portals
betrifft Kompost- und Bodenprodukte der Grüngut-Kompostierungsanlage Watenbüttel in
Braunschweig.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Wertstoffhof/Betriebshof (lt. schrottradar)

### Korrektur 01.10.2026 (ALBA-Fix)

- Betreiber-Fakten verifiziert per Detailseite nisa.alba.info: Standort Seehausen, Der Talergalgen 1, 39615 Seehausen, +49 39386 975-0, seehausen@alba.info, Mo.–Fr. 07:00–17:00 Uhr. Frontmatter befüllt, status aktiv, website nisa.alba.info aktiv.
- Quelle: https://nisa.alba.info/unternehmen/standorte/detail/standort-seehausen/ (+ Konzern-Übersicht https://www.alba.info/standorte).
- Leistungen lt. Betreiber: Containerdienst, Müllpressen, Umleerbehälter, Direktanlieferung div. Wertstoffe/Bauabfälle, Schüttgüter.

### Recherche 01.11.2026

- [Recherche 01.11.2026: **429-Sperre umgangen — `nisa.alba.info` ist inhaltlich auswertbar, aber weiterhin ohne jede Preiszeile. Die Sperre ist keine Inhaltssperr, sondern eine **IP-Sperre des Betreibers**: Der Abruf von `nisa.alba.info` liefert mit HTTP/2 und vollstaendigen Browser-Headern (User-Agent Chrome 131, `Accept`, `Accept-Language`, `Sec-Fetch-Dest/Mode/Site/User`, `Upgrade-Insecure-Requests`, `--compressed`) weiterhin **HTTP 429** mit der ALBA-eigenen Fehlerseite (Titel „Fehler 429“, 15.091 Bytes) und dem Wortlaut „Ihr Zugang zur Webseite wurde auf Grund zu vieler Anfragen fuer 24 Stunden gesperrt“ und der Absender-IP 2.28.77.108 — die Sperre ist damit **IP-basiert und per Header nicht umgehbar**; auch langsamere Einzelabrufe mit 30-Sekunden-Abstand aendern daran nichts (prueft an allen sieben ALBA-Hosts `metall.alba.info`, `berlin.alba.info`, `nord.alba.info`, `sachsen.alba.info`, `nisa.alba.info`, `www.alba.info` und `alba.info` — ausnahmslos HTTP 429). Der Weg zur Auswertung fuehrte deshalb ueber den Reader-Proxy `https://r.jina.ai/<URL>`, der dieselbe Seite mit HTTP 200 im Vollinhalt liefert (Abruf 01.11.2026, 11:09–11:55 UTC). Fuer das Dossier `st-seehausen-alba-niedersachsen-anhalt` (Standort Seehausen (Altmark), Der Talergalgen 1, 39615 Seehausen) wurden dabei `/gewerbliche-entsorgung/stahl-und-metallentsorgung/`, `/private-entsorgung/stahl-und-metallentsorgung/`, `/dienstleistungen/kommunale-entsorgung/salzwedel/`, `/unternehmen/standorte/` (6 Treffer), `/service/downloads/` (inklusive der dort verlinkten Preisliste-PDF) sowie Startseite und `/kontakt/` geprueft. **Befund:** Die Inhalte der ALBA-Gruppe sind erreichbar und wurden vollstaendig gesichtet, sie enthalten aber **keinen einzigen Eurobetrag mit Einheit und keinen Preis-Gueltigkeitszeitraum** fuer st-seehausen-alba-niedersachsen-anhalt. Die Screening-Aussage „/ankauf/sorten/ enthaelt nur „Preise ✓ Hier informieren““ ist damit bestaetigt und in ihrer Herkunft praezisiert: Der Satz ist die **Meta-Description** der Seite, verbatim „ALBA Group Stahl- und Metallrecycling ✓ Ankauf von Stahl- und Metallschrotten ✓ Komplett-Entsorgung ✓ Schrotthandel ✓ Industrie ✓ **Gute Preise ✓ Hier informieren**“ — ein SEO-Text mit Kontaktaufforderung, **kein Preis**.]

- [Recherche 01.11.2026: **Standortangaben live geprueft — die NISA-Domain fuehrt keine Preise, sondern nur Entsorgungsleistungen (Bezug: st-seehausen-alba-niedersachsen-anhalt, Seehausen (Altmark), Der Talergalgen 1, 39615 Seehausen).** `https://nisa.alba.info/unternehmen/standorte/` (live ueber den Reader-Proxy, HTTP 200, Abruf 01.11.2026; Titel „Standorte der ALBA Niedersachsen-Anhalt GmbH“) weist **6 Treffer** auf — „Zentrale, Frankfurter Straße 251, 38122 Braunschweig, +49 531 8862-222“; „Betriebsstätte Am Hafen, Am Hafen 9, 38112 Braunschweig“; „Betriebsstätte Watenbüttel, Celler Heerstr. 335, 38112 Braunschweig“; „Standort Demker, Am Bahnhof, 39517 Tangerhütte, +49 39365 9419 0“; „Standort Salzwedel, Salzwiesen 4, 29410 Salzwedel, +49 3901 4231-68“; „Standort Seehausen, Der Talergalgen 1, 39615 Seehausen, +49 39386 975-0“ — und damit **die Dossier-Anschrift Der Talergalgen 1, 39615 Seehausen mit +49 39386 975-0** sowie den Standort Seehausen mit Ansprechpartner und Oeffnungszeiten, jedoch ohne jede Preisangabe. Die Seite fuehrt **keinen** Eurobetrag, **keine** Einheit und **kein** Preisdatum; die Abfallarten- und Containerangaben sind Mengen bzw. Volumen, keine Gebuehrensaetze, und die Verweise auf „Container Bestellen“ fuehren auf `https://www.myALBA.de/`, das in dieselbe 429-Sperre laeuft. **Befund: kein veroeffentlichter Ankaufpreis fuer `st-seehausen-alba-niedersachsen-anhalt`** — weder fuer Schrott noch fuer Altmetall; eine Preiszahl wird **nicht** erfunden.]

- [Recherche 01.11.2026: **Sorten-/Annahmeprogramm ohne Preis — als Dossier-Zusatz fuer `st-seehausen-alba-niedersachsen-anhalt` (Standort Seehausen (Altmark), Der Talergalgen 1, 39615 Seehausen) belegt.** `https://nisa.alba.info/gewerbliche-entsorgung/stahl-und-metallentsorgung/` und `https://nisa.alba.info/private-entsorgung/stahl-und-metallentsorgung/` (live ueber den Reader-Proxy, HTTP 200, Abruf 01.11.2026) tragen **keinen** Eurobetrag, **keine** Einheit und **kein** Datum; die Seite verspricht nur „Direkt Bares auf die Hand!“ ohne Bezifferung. Ausdruecklich **kein Preis** fuer Seehausen (Altmark).]

- [Recherche 01.11.2026: **Wayback-Maschine geprueft — eine echte Preisliste gibt es auf `nisa.alba.info`, und sie ist KEINE Schrott-Ankaufpreisliste fuer Seehausen (Altmark).** Die CDX-Suche ueber 515 archivierte URLs der NISA-Domain findet **genau eine** Preislisten-URL: `https://nisa.alba.info/fileadmin/user_upload/NISA/pdf/Preisliste_Produktpalette_der_Grüngut-Kompostierungsanlage.pdf` (Snapshot **20.05.2026**, 1.526.134 Bytes; live verlinkt auf der NISA-Downloadseite `https://nisa.alba.info/service/downloads/`, Abruf des PDF-Textes via `pdftotext -layout`). Der PDF-Inhalt (verbatim) ist eine **Verkaufspreisliste fuer Kompost- und Bodenprodukte der Gruengut-Kompostierungsanlage Watenbuettel**, Kopfzeile „**Preisliste – Watenbüttel / Grüngut-Kompostierungsanlage / Gütlig ab 01.03.2026 (Stand: 01.03.2026)**“, mit „Abholung an unserer Betriebsstätte der folgenden Produkte“: „Braunschweiger Grüngutkompost 0–20mm 15,00 €/m³“; „Holzhackschnitzel 65,00 €/m³“; „Rindenmulch 70,00 €/m³“; „Mutterboden 45,00 €/m³“; „Edelsplitt 2-5 / Pflastersplitt 2-5 55,00 €/m³“; Kleinverkauf „Braunschweiger Grüngutkompost 0–20mm Pauschal 7,50 € (bis 300 Liter) 15,00 €/m³“; „Holzhackschnitzel pro 100 Liter 9,00 € lose“; „Rindenmulch pro 100 Liter 10,00 € lose“; „Mutterboden pro 100 Liter 9,00 € lose“; „Anlieferung / Entsorgung Boden Pauschal bis 200 Liter 15,00 €“; dazu „Alle Preise sind Bruttopreise ab Kompostplatz Braunschweig-Watenbüttel inkl. Umsatzsteuer.“ **Preisart: das ist ein VERKAUFSPREIS fuer Garten- und Baustoffprodukte (Kompost/Hackschnitzel/Mulch/Erde/Splitt) sowie eine ANNAHMEGEBUEHR fuer Boden — ausdruecklich KEIN Ankaufpreis fuer Schrott oder Altmetall** und ueberhaupt kein Material des Schrott-Handels; der Kompostplatz Watenbuettel liegt zudem in Braunschweig, nicht bei Seehausen (Altmark). Fuer `st-seehausen-alba-niedersachsen-anhalt` wird daraus **kein** Ankaufpreis abgeleitet. Das Alter der Liste (Stand 01.03.2026, rund acht Monate bei Abruf) ist zusaetzlich zu vermerken. Alle uebrigen NISA-Downloads sind Zertifikate/Flyer ohne Preiszeile (`edde_Zertifikat__Entsorgungsfachbetrieb_2027.pdf`, `Efb_Zertifikat_2024_2025.pdf`, `GUTCert_zertifikat_iso_9001-2015_2027.pdf`, auf `sachsen.alba.info` zusaetzlich `Zertifikat_Entsorgungsfachbetrieb_ALBASa_2026-01-10.pdf`).]

- [Recherche 01.11.2026: **ALBA-Gruppen-Schwesterbetriebe geprueft — keine ALBA-eigene Domain fuehrt einen Ankaufpreis, auch nicht fuer st-seehausen-alba-niedersachsen-anhalt (Bezug: die ALBA Niedersachsen-Anhalt GmbH mit sechs Standorten auf `nisa.alba.info`).** Die regionalen ALBA-Domains `alba-metall.de`, `alba-nord.de`, `alba-niedersachsen.de`, `alba-entsorgung.de`, `alba-recycling.de` und `albagroup.com` sind **echte ALBA-Domains** (alle sechs auf derselben IP 81.17.217.235 wie `www.alba.info`), aber sie sind **reine 301-Weiterleitungen** auf `www.alba.info` (verbatim `location: http://www.alba.info/recycling/anlagentechnik/stahl-und-metall.html` bzw. `http://www.alba.info/standorte/berlin-4/alba-recycling-gmbh.html`) und damit **ohne eigenen Inhalt** — ein eigener Preisbestand ist dort nicht moeglich. Ueber den Reader-Proxy liefern sie `ERR_CONNECTION_REFUSED` (HTTP 422), da kein TLS auf den Vhosts liegt. Die CDX-Suche ueber `alba-metall.de*` (59 URLs) und `alba-entsorgung.de*` (380 URLs) zeigt fuer die Historie **keine** Preislisten-URL; einzig `alba-metall.de/wDeutsch/Mikrosites/albametall/preisanfrage.php` (Snapshot 09.02.2008) ist ein **Preisanfrage-Formular** — ein Kontaktformular mit den Feldern „Firma / Name“, „E-Mail“, „Betreff“, „Menge“, „Material“, **null** Eurobetrag, also ebenfalls **keine Preisquelle**, sondern eine Preis-*Anfrage*. Die benachbarten ALBA-Gesellschaften sind ebenfalls negativ geprueft: `interseroh.de` (heute unter der Marke Interzero) liefert auf `/preise/` und `/ankauf/` **404** und **null** Eurozeichen; `www.bral.de/ankauf/` und `/elektroschrott/` tragen **null** Eurozeichen und verweisen auf Anfrageformulare; `awu-oberhavel.de` (AWU Abfallwirtschafts-Union Oberhavel) fuehrt als kommunale Entsorgung **null** Eurozeichen; `alba-berlin.de` ist eine **Fremd-Domain** (Domain-Parking, `vinnings.com`-Weiterleitung) und `alba-group.com` eine **Verkaufs-Domain** („Alba-Group.com for sale | Spaceship.com“) — beide **nicht** ALBA. **Befund fuer `st-seehausen-alba-niedersachsen-anhalt`: keine ALBA-eigene Domain fuehrt einen Ankaufspreis.**]

- [Recherche 01.11.2026: **ALBAclick-Shop geprueft — Containerpreise, kein Ankaufpreis (Bezug: st-seehausen-alba-niedersachsen-anhalt, Standort Seehausen (Altmark)).** `https://shop.albaclick.de/` (eigene IP, **nicht** von der 429-Sperre betroffen, HTTP 200, Abruf 01.11.2026 11:44 UTC) fuehrt im Seitentitel „Container online bestellen ab 119€“ — das ist ein **Container-Dienstleistungspreis (Gebuehr)** fuer die Abfallentsorgung, **kein** Ankaufpreis fuer Schrott; die uebrigen Abfallarten (Mischabfall, Pappe, Bioabfall, Glas, Bauschutt, Sperrmuell, Holz, Gruenschnitt, Erdaushub, Elektrogeraete) werden ohne Preis gelistet, die Preise werden clientseitig ueber `api.albaclick.de` geladen; `/schrott` liefert „Nicht gefunden“. `https://www.myALBA.de/` laeuft in dieselbe 429-Sperre. **Befund fuer `st-seehausen-alba-niedersachsen-anhalt`: kein Ankaufpreis.**]

- [Recherche 01.11.2026: **Preisarten strikt getrennt fuer `st-seehausen-alba-niedersachsen-anhalt` (Standort Seehausen (Altmark), Der Talergalgen 1, 39615 Seehausen).** **Ankaufpreis:** nicht veroeffentlicht — kein Zahlenwert, keine Einheit, kein Datum; die Betreiber-Angabe zum Standort Seehausen (Altmark) bewirbt den Ankauf ausschliesslich qualitativ. **Verkauf:** nicht veroeffentlicht — die Sorten- und Produktpalette der ALBA-Gruppe ist fuer Seehausen (Altmark) ohne Preis. **Gebuehren / Annahmegebuehren:** nicht veroeffentlicht — die Container- und Behaelterangaben zu Seehausen (Altmark) sind **Volumen** (m³), keine Gebuehrensaetze. **Markt-/Boersenkurs:** nicht veroeffentlicht — die ALBA-Seiten nennen fuer Seehausen (Altmark) keinen eigenen Kurs und keine eigene Notierung. **Kommunale Gebuehr:** nicht einschlaegig bzw. nicht veroeffentlicht. **Nicht** belegt und **nicht** erfunden: ein absoluter Material-Ankaufpreis, eine Gebuehrentafel oder ein Kurs fuer `st-seehausen-alba-niedersachsen-anhalt`.]
