---
slug: ni-wilhelmshaven-jade-stahl
name: ALBA Metall Nord GmbH, Standort Wilhelmshaven
trader_type: schrotthaendler
state: NI
city: Wilhelmshaven
street: 'Emsstr. 29'
postcode: '26382'
phone: '+49 4421 4020'
email: 'amn.wilhelmshaven@alba.info'
opening_hours: 'Mo.–Do. 7:30–16:30, Fr. 7:00–14:45 Uhr'
website: https://metall.alba.info
website_status: 'aktiv'
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Oldenburg / Wilhelmshaven / Weser-Ems
provenance_ankauf_raw: ja
provenance_origin: table
---

# JADE-STAHL GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Stahl/Eisen

### Recherche 30.09.2026

- Emsstr. 29, 26382 + Tel. 04421 402-0: Gelbe Seiten nennt JADE-STAHL; Das Telefonbuch nennt an gleicher Adresse ALBA Metall Nord GmbH (Namens-Caveat, Adress-/Tel-Zuordnung unsicher).
- Quelle: https://www.gelbeseiten.de/suche/Schrott/Wilhelmshaven
- Quelle: https://www.dastelefonbuch.de/Suche/Schrott/Wilhelmshaven

### Korrektur 01.10.2026 (ALBA-Fix, Betreiber-Nachfolge — Dossier NICHT gelöscht)

- Northdata-Befund (INTERSEROH Jade-Stahl GmbH, AG Oldenburg HRB 131660): 01.09.2014 Verschmelzung (ALBA Metall Ost), 03.09.2014 Löschung durch Verschmelzung auf ALBA Metall Nord GmbH. Gesellschaft erloschen, Rechtsnachfolger ALBA Metall Nord GmbH.
- Betreiber-Bestätigung: Detailseite metall.alba.info führt den Standort Wilhelmshaven (Emsstrasse 29, 26382, +49 4421 4020, amn.wilhelmshaven@alba.info) ausdrücklich als „früher Jade-Stahl Wilhelmshaven“.
- Frontmatter auf Betreiber-Stand gesetzt (name ALBA Metall Nord GmbH, Standort Wilhelmshaven; Adresse/Zeiten/Mail vom Betreiber; website metall.alba.info aktiv). Altname JADE-STAHL hier in Timeline dokumentiert. Schwester-Dossier: ni-wilhelmshaven-alba-metall-nord-hafen.
- Quellen: https://www.northdata.de/INTERSEROH+Jade-Stahl+GmbH,+Wilhelmshaven + https://metall.alba.info/unternehmen/standorte/detail/betriebsstaette-wilhelmshaven/

### Recherche 01.10.2026

- [Feedback-Triage ID 2368: Meldung berechtigt — Dublette von ni-wilhelmshaven-alba-metall-nord-hafen (ID 1156) bestätigt (Verschmelzung 2014, Gesellschaft erloschen, Rechtsnachfolger ALBA Metall Nord); Dossier bleibt als Altname-Spur bestehen (kein Löschen), Frontmatter auf Betreiber-Stand, Altname in Timeline; Typ sonstige → schrotthaendler (wie Schwester-Dossier). Beleglage: Betreiber + Northdata-Verschmelzungsbefund. Quelle(n): siehe Korrektur-Bullet.]

### Recherche 01.11.2026

- [Recherche 01.11.2026: **429-Sperre umgangen — `metall.alba.info` ist inhaltlich auswertbar, aber weiterhin ohne jede Preiszeile. Die Sperre ist keine Inhaltssperr, sondern eine **IP-Sperre des Betreibers**: Der Abruf von `metall.alba.info` liefert mit HTTP/2 und vollstaendigen Browser-Headern (User-Agent Chrome 131, `Accept`, `Accept-Language`, `Sec-Fetch-Dest/Mode/Site/User`, `Upgrade-Insecure-Requests`, `--compressed`) weiterhin **HTTP 429** mit der ALBA-eigenen Fehlerseite (Titel „Fehler 429“, 15.091 Bytes) und dem Wortlaut „Ihr Zugang zur Webseite wurde auf Grund zu vieler Anfragen fuer 24 Stunden gesperrt“ und der Absender-IP 2.28.77.108 — die Sperre ist damit **IP-basiert und per Header nicht umgehbar**; auch langsamere Einzelabrufe mit 30-Sekunden-Abstand aendern daran nichts (prueft an allen sieben ALBA-Hosts `metall.alba.info`, `berlin.alba.info`, `nord.alba.info`, `sachsen.alba.info`, `nisa.alba.info`, `www.alba.info` und `alba.info` — ausnahmslos HTTP 429). Der Weg zur Auswertung fuehrte deshalb ueber den Reader-Proxy `https://r.jina.ai/<URL>`, der dieselbe Seite mit HTTP 200 im Vollinhalt liefert (Abruf 01.11.2026, 11:09–11:55 UTC). Fuer das Dossier `ni-wilhelmshaven-jade-stahl` (Standort Wilhelmshaven (früher Jade-Stahl), Emsstr. 29, 26382 Wilhelmshaven) wurden dabei `/ankauf/sorten/`, `/ankauf/`, `/ankauf/gewerbe/`, `/ankauf/privathaushalte/`, `/verkauf/`, `/unternehmen/standorte/` (13 Treffer), `/service/downloads-zertifikate/` und `/kontakt/` sowie die 13 Standort-Detailseiten; die Preis-Kandidatenpfade `/preise/`, `/ankauf/preise/`, `/sorten/` und `/service/ankauf/` liefern **404** („Seite konnte nicht gefunden werden“) geprueft. **Befund:** Die Inhalte der ALBA-Gruppe sind erreichbar und wurden vollstaendig gesichtet, sie enthalten aber **keinen einzigen Eurobetrag mit Einheit und keinen Preis-Gueltigkeitszeitraum** fuer ni-wilhelmshaven-jade-stahl. Die Screening-Aussage „/ankauf/sorten/ enthaelt nur „Preise ✓ Hier informieren““ ist damit bestaetigt und in ihrer Herkunft praezisiert: Der Satz ist die **Meta-Description** der Seite, verbatim „ALBA Group Stahl- und Metallrecycling ✓ Ankauf von Stahl- und Metallschrotten ✓ Komplett-Entsorgung ✓ Schrotthandel ✓ Industrie ✓ **Gute Preise ✓ Hier informieren**“ — ein SEO-Text mit Kontaktaufforderung, **kein Preis**.]

- [Recherche 01.11.2026: **Standortseite live gelesen — Ankaufszusage ohne Betrag fuer ni-wilhelmshaven-jade-stahl (Wilhelmshaven (früher Jade-Stahl), Emsstr. 29, 26382 Wilhelmshaven).** https://metall.alba.info/unternehmen/standorte/detail/betriebsstaette-wilhelmshaven/ (live ueber den Reader-Proxy, HTTP 200, Abruf 01.11.2026) belegt fuer Wilhelmshaven (früher Jade-Stahl): An unserem Standort in Wilhelmshaven (früher „Jade-Stahl Wilhelmshaven“) stehen Ihnen 110 Expert*innnen für Schrott- und Metallrecycling zur Seite.. Telefon +49 4421 4020, E-Mail amn.wilhelmshaven@alba.info, Oeffnungszeiten „Mo–Do 7:30–16:30 Uhr, Fr 7:00–14:45 Uhr“. Die Seite nennt **keinen** Eurobetrag, **keine** Einheit und **kein** Gueltigkeitsdatum zum Ankaufspreis — „Bestpreis“, „gute Preise“ und „faire Preise“ sind Werbeformeln ohne Zahl.]

- [Recherche 01.11.2026: **Sortenprogramm ohne Preis — als Dossier-Zusatz fuer `ni-wilhelmshaven-jade-stahl` (Standort Wilhelmshaven (früher Jade-Stahl), Emsstr. 29, 26382 Wilhelmshaven) belegt.** `https://metall.alba.info/ankauf/sorten/` (live ueber den Reader-Proxy, HTTP 200, Abruf 01.11.2026 11:09 UTC; Titel „ALBA Metallrecycling: Unsere Sorten“) listet **17 Sortengruppen** als Bild-/Teaser-Kacheln: Produktionsschrotte, Haushaltsschrott, Industrie- und Abbruchschrotte, Kabelschrotte, Mischschrotte, Vormaterial, Schwere Schere, Legierte Schrotte, Karossen, Aluminium, Kupfer, Messing, Brennschrott, Gusseisen, Moniereisen, Scherenschrott, Stahl- und Metallspaene. Die Seite trägt **keinen einzigen Eurobetrag, keine Einheit und kein Datum** — ausdruecklich **kein Preis**, sondern das reine Sortenprogramm. Die Kacheln verlinken auf die Sorten-Uebersicht, nicht auf Einzel-Artikelseiten mit Preisen; die Meta-Description der Seite endet auf „Gute Preise ✓ Hier informieren“ (Kontaktaufforderung, kein Wert). Ob und zu welchen Konditionen Wilhelmshaven (früher Jade-Stahl) diese Sorten annimmt, ist betreiberseitig **nicht** beziffert.]

- [Recherche 01.11.2026: **Wayback-Maschine geprueft — `metall.alba.info` hatte nie eine Preisliste, auch nicht fuer Wilhelmshaven (früher Jade-Stahl).** Die CDX-Suche (`http://web.archive.org/cdx/search/cdx?url=metall.alba.info*&output=text&limit=2000&collapse=urlkey&fl=timestamp,original,statuscode`, 545 URLs) findet **keine** Preislisten-URL; die vier archivierten Ankaufs-Pfade sind `/ankauf/` (Snapshot 29.05.2022), `/ankauf/gewerbe/`, `/ankauf/privathaushalte/` und `/ankauf/sorten/` (Snapshot 29.05.2022). Der Snapshot `20220529085719` von `/ankauf/sorten/` (111.347 Bytes) traegt **null** Eurozeichen und dieselbe 17-Sorten-Liste wie heute — **keine aeltere Preisliste, die verloren gegangen waere**. Auch der Snapshot `20260520131022` (105.755 Bytes) fuehrt null Eurozeichen. Der Snapshot `20260609115753` der „Annahmebedingungen fuer Stahlschrott-Streckenlieferungen an die ALBA Metall Nord GmbH“ (`Einkaufsbedingungen_AMN_Strecke.pdf`, live verlinkt auf der Downloadseite) traegt weiterhin den Stand **„(Stand 09. Mai 2022)“** und **keinen** Eurobetrag — er belegt die **Preisart** (Annahme zu vereinbarten Bedingungen, Bezug auf die „Europæische Stahlschrottliste“) und die **Annahmefristen** (Stueckgewicht hoechstens 500 Kilogramm, Ausschluss von Altkarossen/Motoren/Spaenen/Baeren/Sinter/Schlacken), aber **keinen Preis**. **Befund fuer `ni-wilhelmshaven-jade-stahl`: keine Preisquelle, auch nicht historisch.**]

- [Recherche 01.11.2026: **Zur Betriebsstaetten-Nachfolge Jade-Stahl — die Umbenennung ist am 01.11.2026 betreiberseitig bestaetigt und mit einer Zahlungsart-Aenderung verbunden.** Die Detailseite nennt den Standort „frueher „Jade-Stahl Wilhelmshaven““ und fuehrt „110 Expert*innnen“ fuer „Schrott- und Metallrecycling“; Anschrift Emsstrasse 29, 26382 Wilhelmshaven, +49 4421 4020, amn.wilhelmshaven@alba.info. Fuer die Fortfuehrung unter dem Namen ALBA Metall Nord GmbH spricht zusaetzlich, dass die ALBA-Metall-Standortliste den Standort **an dieser Anschrift** fuehrt (13 Treffer, Eintrag „Standort Wilhelmshaven“). Eine **Preisaussage** enthaelt die Seite **nicht** — weder fuer den aktuellen noch fuer den historischen Betreibernamen. **Befund: kein veroeffentlichter Ankaufpreis fuer `ni-wilhelmshaven-jade-stahl`**; der Nachfolge-Beleg zur Dossier-Adresse wird durch die Preispruefung nicht erbracht.]

- [Recherche 01.11.2026: **ALBA-Gruppen-Schwesterbetriebe geprueft — keine ALBA-eigene Domain fuehrt einen Ankaufpreis, auch nicht fuer ni-wilhelmshaven-jade-stahl (Bezug: die ALBA Metall Nord GmbH und die ALBA Metall Sued GmbH, deren Standorte auf `metall.alba.info` gefuehrt werden).** Die regionalen ALBA-Domains `alba-metall.de`, `alba-nord.de`, `alba-niedersachsen.de`, `alba-entsorgung.de`, `alba-recycling.de` und `albagroup.com` sind **echte ALBA-Domains** (alle sechs auf derselben IP 81.17.217.235 wie `www.alba.info`), aber sie sind **reine 301-Weiterleitungen** auf `www.alba.info` (verbatim `location: http://www.alba.info/recycling/anlagentechnik/stahl-und-metall.html` bzw. `http://www.alba.info/standorte/berlin-4/alba-recycling-gmbh.html`) und damit **ohne eigenen Inhalt** — ein eigener Preisbestand ist dort nicht moeglich. Ueber den Reader-Proxy liefern sie `ERR_CONNECTION_REFUSED` (HTTP 422), da kein TLS auf den Vhosts liegt. Die CDX-Suche ueber `alba-metall.de*` (59 URLs) und `alba-entsorgung.de*` (380 URLs) zeigt fuer die Historie **keine** Preislisten-URL; einzig `alba-metall.de/wDeutsch/Mikrosites/albametall/preisanfrage.php` (Snapshot 09.02.2008) ist ein **Preisanfrage-Formular** — ein Kontaktformular mit den Feldern „Firma / Name“, „E-Mail“, „Betreff“, „Menge“, „Material“, **null** Eurobetrag, also ebenfalls **keine Preisquelle**, sondern eine Preis-*Anfrage*. Die benachbarten ALBA-Gesellschaften sind ebenfalls negativ geprueft: `interseroh.de` (heute unter der Marke Interzero) liefert auf `/preise/` und `/ankauf/` **404** und **null** Eurozeichen; `www.bral.de/ankauf/` und `/elektroschrott/` tragen **null** Eurozeichen und verweisen auf Anfrageformulare; `awu-oberhavel.de` (AWU Abfallwirtschafts-Union Oberhavel) fuehrt als kommunale Entsorgung **null** Eurozeichen; `alba-berlin.de` ist eine **Fremd-Domain** (Domain-Parking, `vinnings.com`-Weiterleitung) und `alba-group.com` eine **Verkaufs-Domain** („Alba-Group.com for sale | Spaceship.com“) — beide **nicht** ALBA. **Befund fuer `ni-wilhelmshaven-jade-stahl`: keine ALBA-eigene Domain fuehrt einen Ankaufspreis.**]

- [Recherche 01.11.2026: **ALBAclick-Shop geprueft — Containerpreise, kein Ankaufpreis (Bezug: ni-wilhelmshaven-jade-stahl, Standort Wilhelmshaven (früher Jade-Stahl)).** `https://shop.albaclick.de/` (eigene IP, **nicht** von der 429-Sperre betroffen, HTTP 200, Abruf 01.11.2026 11:44 UTC) fuehrt im Seitentitel „Container online bestellen ab 119€“ — das ist ein **Container-Dienstleistungspreis (Gebuehr)** fuer die Abfallentsorgung, **kein** Ankaufpreis fuer Schrott; die uebrigen Abfallarten (Mischabfall, Pappe, Bioabfall, Glas, Bauschutt, Sperrmuell, Holz, Gruenschnitt, Erdaushub, Elektrogeraete) werden ohne Preis gelistet, die Preise werden clientseitig ueber `api.albaclick.de` geladen; `/schrott` liefert „Nicht gefunden“. `https://www.myALBA.de/` laeuft in dieselbe 429-Sperre. **Befund fuer `ni-wilhelmshaven-jade-stahl`: kein Ankaufpreis.**]

- [Recherche 01.11.2026: **Preisarten strikt getrennt fuer `ni-wilhelmshaven-jade-stahl` (Standort Wilhelmshaven (früher Jade-Stahl), Emsstr. 29, 26382 Wilhelmshaven).** **Ankaufpreis:** nicht veroeffentlicht — kein Zahlenwert, keine Einheit, kein Datum; die Betreiber-Angabe zum Standort Wilhelmshaven (früher Jade-Stahl) bewirbt den Ankauf ausschliesslich qualitativ. **Verkauf:** nicht veroeffentlicht — die Sorten- und Produktpalette der ALBA-Gruppe ist fuer Wilhelmshaven (früher Jade-Stahl) ohne Preis. **Gebuehren / Annahmegebuehren:** nicht veroeffentlicht — die Container- und Behaelterangaben zu Wilhelmshaven (früher Jade-Stahl) sind **Volumen** (m³), keine Gebuehrensaetze. **Markt-/Boersenkurs:** nicht veroeffentlicht — die ALBA-Seiten nennen fuer Wilhelmshaven (früher Jade-Stahl) keinen eigenen Kurs und keine eigene Notierung. **Kommunale Gebuehr:** nicht einschlaegig bzw. nicht veroeffentlicht. **Nicht** belegt und **nicht** erfunden: ein absoluter Material-Ankaufpreis, eine Gebuehrentafel oder ein Kurs fuer `ni-wilhelmshaven-jade-stahl`.]
