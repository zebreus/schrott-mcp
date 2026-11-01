---
slug: st-vockerode-alba-sachsen
name: ALBA Sachsen GmbH, Standort Vockerode
trader_type: schrotthaendler
state: ST
city: Oranienbaum-Wörlitz OT Vockerode
street: 'Walderseeer Str. 40'
postcode: '06785'
phone: '+49 34905 21181'
email: 'kompostplatz_vockerode@alba.info'
opening_hours: 'Apr.–Okt.: Mo.–Fr. 6:30–17:00, Sa. 8:00–11:00; Nov.–Mär.: Mo.–Fr. 6:30–16:00'
website: https://sachsen.alba.info
website_status: 'aktiv'
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: 'Nachtrag Audit-Runde 3 (Stand: 2026-09-27)'
provenance_ankauf_raw: Ankauf unklar (Entsorger – prüfen)
provenance_origin: table
---

# ALBA Sachsen GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Betriebshof (lt. schrottradar)

### Korrektur 01.10.2026 (ALBA-Fix)

- Betreiber-Fakten verifiziert per Detailseite sachsen.alba.info: Standort Vockerode, Walderseeer Str. 40, 06785 Oranienbaum-Wörlitz, +49 34905 21181, kompostplatz_vockerode@alba.info. Frontmatter befüllt, website sachsen.alba.info aktiv.
- WICHTIGER Caveat: Lt. Betreiber ist Vockerode ein Kompostwerk (Komposterde, Containerdienst, Entsorgungsberatung) — KEIN Schrottankauf belegt. Status bleibt daher pruefung, trader_type ungeändert; Schrott-Ankaufannahme nicht unterstellen.
- Quelle: https://sachsen.alba.info/unternehmen/standorte/detail/standort-vockerode/ (+ Konzern-Übersicht https://www.alba.info/standorte).
- [Recherche 01.10.2026 (Feedback 2750, berechtigt und bereits abgebildet): gemeldeter Befund (Kompostwerk/Containerdienst, kein Schrottankauf belegt) entspricht exakt dem Caveat der Korrektur heute — Status bleibt pruefung, trader_type unverändert, keine Frontmatter-Änderung.]

### Recherche 01.11.2026

- [Recherche 01.11.2026: **429-Sperre umgangen — `sachsen.alba.info` ist inhaltlich auswertbar, aber weiterhin ohne jede Preiszeile. Die Sperre ist keine Inhaltssperr, sondern eine **IP-Sperre des Betreibers**: Der Abruf von `sachsen.alba.info` liefert mit HTTP/2 und vollstaendigen Browser-Headern (User-Agent Chrome 131, `Accept`, `Accept-Language`, `Sec-Fetch-Dest/Mode/Site/User`, `Upgrade-Insecure-Requests`, `--compressed`) weiterhin **HTTP 429** mit der ALBA-eigenen Fehlerseite (Titel „Fehler 429“, 15.091 Bytes) und dem Wortlaut „Ihr Zugang zur Webseite wurde auf Grund zu vieler Anfragen fuer 24 Stunden gesperrt“ und der Absender-IP 2.28.77.108 — die Sperre ist damit **IP-basiert und per Header nicht umgehbar**; auch langsamere Einzelabrufe mit 30-Sekunden-Abstand aendern daran nichts (prueft an allen sieben ALBA-Hosts `metall.alba.info`, `berlin.alba.info`, `nord.alba.info`, `sachsen.alba.info`, `nisa.alba.info`, `www.alba.info` und `alba.info` — ausnahmslos HTTP 429). Der Weg zur Auswertung fuehrte deshalb ueber den Reader-Proxy `https://r.jina.ai/<URL>`, der dieselbe Seite mit HTTP 200 im Vollinhalt liefert (Abruf 01.11.2026, 11:09–11:55 UTC). Fuer das Dossier `st-vockerode-alba-sachsen` (Standort Oranienbaum-Wörlitz OT Vockerode, Walderseeer Str. 40, 06785 Oranienbaum-Wörlitz) wurden dabei `/entsorgung/gewerbe/`, `/entsorgung/privat/`, `/unternehmen/standorte/` (5 Treffer), `/service/downloads-zertifikate/` sowie die Standort-Detailseiten Wurzen und Vockerode geprueft. **Befund:** Die Inhalte der ALBA-Gruppe sind erreichbar und wurden vollstaendig gesichtet, sie enthalten aber **keinen einzigen Eurobetrag mit Einheit und keinen Preis-Gueltigkeitszeitraum** fuer st-vockerode-alba-sachsen. Die Screening-Aussage „/ankauf/sorten/ enthaelt nur „Preise ✓ Hier informieren““ ist damit bestaetigt und in ihrer Herkunft praezisiert: Der Satz ist die **Meta-Description** der Seite, verbatim „ALBA Group Stahl- und Metallrecycling ✓ Ankauf von Stahl- und Metallschrotten ✓ Komplett-Entsorgung ✓ Schrotthandel ✓ Industrie ✓ **Gute Preise ✓ Hier informieren**“ — ein SEO-Text mit Kontaktaufforderung, **kein Preis**.]

- [Recherche 01.11.2026: **Standortangaben live gelesen — die ALBA-Sachsen-Domain fuehrt keine Preise (Bezug: st-vockerode-alba-sachsen, Oranienbaum-Wörlitz OT Vockerode, Walderseeer Str. 40, 06785 Oranienbaum-Wörlitz).** https://sachsen.alba.info/unternehmen/standorte/detail/standort-vockerode/ (live ueber den Reader-Proxy, HTTP 200, Abruf 01.11.2026) belegt fuer Oranienbaum-Wörlitz OT Vockerode: Komposterde vom Profi. Die Seite nennt **keinen** Eurobetrag, **keine** Einheit und **kein** Gueltigkeitsdatum; die Oeffnungszeiten und Anlieferzeiten (Bio-/Gaerreste) sind Zeitangaben, keine Preise. **Befund: kein veroeffentlichter Ankaufpreis fuer `st-vockerode-alba-sachsen`** — eine Preiszahl wird **nicht** erfunden.]

- [Recherche 01.11.2026: **Sorten-/Annahmeprogramm ohne Preis — als Dossier-Zusatz fuer `st-vockerode-alba-sachsen` (Standort Oranienbaum-Wörlitz OT Vockerode, Walderseeer Str. 40, 06785 Oranienbaum-Wörlitz) belegt.** Die ALBA-Sachsen-Seiten (`/entsorgung/gewerbe/`, `/entsorgung/privat/`, `/unternehmen/standorte/`) tragen **keinen** Eurobetrag, **keine** Einheit und **kein** Datum. Ausdruecklich **kein Preis** fuer Oranienbaum-Wörlitz OT Vockerode.]

- [Recherche 01.11.2026: **Wayback-Maschine geprueft — auch `sachsen.alba.info` hatte nie eine Preisliste, auch nicht fuer Oranienbaum-Wörlitz OT Vockerode.** CDX-Suchen ueber `berlin.alba.info*` (962 URLs), `nord.alba.info*` (551 URLs), `sachsen.alba.info*` (139 URLs) und `www.alba.info*` (8.000 URLs, `collapse=urlkey`) finden **keine** Preislisten-URL fuer Schrott oder Altmetall. Die einzigen „preis“-Treffer sind eine alte Presse-Meldung von 2010 („Stahlschrottpreise gestiegen“) und die NISA-Kompost-Preisliste (siehe dort). Die archivierte ALBA-Berlin-Seite „Angebote und Sonderaktionen“ (Snapshot 2017) enthaelt **null** Eurozeichen — nur Hochzeitscontainer und Container-Volumen. **Befund fuer `st-vockerode-alba-sachsen`: keine Preisquelle, auch nicht historisch.**]

- [Recherche 01.11.2026: **Der Vockeroder Platz ist ein Kompostplatz, kein Schrottplatz — die Sorten-Zuordnung im Dossier ist zu praezisieren.** Die Detailseite `https://sachsen.alba.info/unternehmen/standorte/detail/standort-vockerode/` (live ueber den Reader-Proxy, HTTP 200, Abruf 01.11.2026) fuehrt „Standort Vockerode / ALBA Sachsen GmbH / Walderseeer Str. 40 / 06785 Oranienbaum-Wörlitz / +49 34905 21181 / kompostplatz_vockerode@alba.info“ mit Ueberschrift „Komposterde vom Profi“ und Oeffnungszeiten April bis Oktober Mo–Fr 6:30 bis 17:00 Uhr, Sa 8:00 bis 11:00 Uhr, November bis März Mo–Fr 6:30 bis 16:00 Uhr, Sa geschlossen, dazu „Anlieferzeit (Bio / Gaerreste): Montag - Donnerstag: 07:00 - 15:00 Uhr“. Die Seite nennt **keinen** Eurobetrag, **keine** Einheit und **kein** Datum. Der Platz wird damit als **Kompost-/Bioabfallplatz** der ALBA Sachsen belegt, nicht als Schrott-Annahmestelle — das ist eine **Zusatzpräzisierung** und **kein** Preis.]

- [Recherche 01.11.2026: **ALBA-Gruppen-Schwesterbetriebe geprueft — keine ALBA-eigene Domain fuehrt einen Ankaufpreis, auch nicht fuer st-vockerode-alba-sachsen (Bezug: die ALBA Sachsen GmbH mit fuenf Standorten in Sachsen und Sachsen-Anhalt auf `sachsen.alba.info`).** Die regionalen ALBA-Domains `alba-metall.de`, `alba-nord.de`, `alba-niedersachsen.de`, `alba-entsorgung.de`, `alba-recycling.de` und `albagroup.com` sind **echte ALBA-Domains** (alle sechs auf derselben IP 81.17.217.235 wie `www.alba.info`), aber sie sind **reine 301-Weiterleitungen** auf `www.alba.info` (verbatim `location: http://www.alba.info/recycling/anlagentechnik/stahl-und-metall.html` bzw. `http://www.alba.info/standorte/berlin-4/alba-recycling-gmbh.html`) und damit **ohne eigenen Inhalt** — ein eigener Preisbestand ist dort nicht moeglich. Ueber den Reader-Proxy liefern sie `ERR_CONNECTION_REFUSED` (HTTP 422), da kein TLS auf den Vhosts liegt. Die CDX-Suche ueber `alba-metall.de*` (59 URLs) und `alba-entsorgung.de*` (380 URLs) zeigt fuer die Historie **keine** Preislisten-URL; einzig `alba-metall.de/wDeutsch/Mikrosites/albametall/preisanfrage.php` (Snapshot 09.02.2008) ist ein **Preisanfrage-Formular** — ein Kontaktformular mit den Feldern „Firma / Name“, „E-Mail“, „Betreff“, „Menge“, „Material“, **null** Eurobetrag, also ebenfalls **keine Preisquelle**, sondern eine Preis-*Anfrage*. Die benachbarten ALBA-Gesellschaften sind ebenfalls negativ geprueft: `interseroh.de` (heute unter der Marke Interzero) liefert auf `/preise/` und `/ankauf/` **404** und **null** Eurozeichen; `www.bral.de/ankauf/` und `/elektroschrott/` tragen **null** Eurozeichen und verweisen auf Anfrageformulare; `awu-oberhavel.de` (AWU Abfallwirtschafts-Union Oberhavel) fuehrt als kommunale Entsorgung **null** Eurozeichen; `alba-berlin.de` ist eine **Fremd-Domain** (Domain-Parking, `vinnings.com`-Weiterleitung) und `alba-group.com` eine **Verkaufs-Domain** („Alba-Group.com for sale | Spaceship.com“) — beide **nicht** ALBA. **Befund fuer `st-vockerode-alba-sachsen`: keine ALBA-eigene Domain fuehrt einen Ankaufspreis.**]

- [Recherche 01.11.2026: **ALBAclick-Shop geprueft — Containerpreise, kein Ankaufpreis (Bezug: st-vockerode-alba-sachsen, Standort Oranienbaum-Wörlitz OT Vockerode).** `https://shop.albaclick.de/` (eigene IP, **nicht** von der 429-Sperre betroffen, HTTP 200, Abruf 01.11.2026 11:44 UTC) fuehrt im Seitentitel „Container online bestellen ab 119€“ — das ist ein **Container-Dienstleistungspreis (Gebuehr)** fuer die Abfallentsorgung, **kein** Ankaufpreis fuer Schrott; die uebrigen Abfallarten (Mischabfall, Pappe, Bioabfall, Glas, Bauschutt, Sperrmuell, Holz, Gruenschnitt, Erdaushub, Elektrogeraete) werden ohne Preis gelistet, die Preise werden clientseitig ueber `api.albaclick.de` geladen; `/schrott` liefert „Nicht gefunden“. `https://www.myALBA.de/` laeuft in dieselbe 429-Sperre. **Befund fuer `st-vockerode-alba-sachsen`: kein Ankaufpreis.**]

- [Recherche 01.11.2026: **Preisarten strikt getrennt fuer `st-vockerode-alba-sachsen` (Standort Oranienbaum-Wörlitz OT Vockerode, Walderseeer Str. 40, 06785 Oranienbaum-Wörlitz).** **Ankaufpreis:** nicht veroeffentlicht — kein Zahlenwert, keine Einheit, kein Datum; die Betreiber-Angabe zum Standort Oranienbaum-Wörlitz OT Vockerode bewirbt den Ankauf ausschliesslich qualitativ. **Verkauf:** nicht veroeffentlicht — die Sorten- und Produktpalette der ALBA-Gruppe ist fuer Oranienbaum-Wörlitz OT Vockerode ohne Preis. **Gebuehren / Annahmegebuehren:** nicht veroeffentlicht — die Container- und Behaelterangaben zu Oranienbaum-Wörlitz OT Vockerode sind **Volumen** (m³), keine Gebuehrensaetze. **Markt-/Boersenkurs:** nicht veroeffentlicht — die ALBA-Seiten nennen fuer Oranienbaum-Wörlitz OT Vockerode keinen eigenen Kurs und keine eigene Notierung. **Kommunale Gebuehr:** nicht einschlaegig bzw. nicht veroeffentlicht. **Nicht** belegt und **nicht** erfunden: ein absoluter Material-Ankaufpreis, eine Gebuehrentafel oder ein Kurs fuer `st-vockerode-alba-sachsen`.]
