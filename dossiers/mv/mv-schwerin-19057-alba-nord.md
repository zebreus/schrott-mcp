---
slug: mv-schwerin-19057-alba-nord
name: ALBA Nord GmbH
trader_type: sonstige
state: MV
city: Schwerin
street: Ziegeleiweg 12
postcode: '19057'
phone: +49 385 4811 0
email: nord@alba.info
opening_hours: ''
website: https://nord.alba.info/
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# ALBA Nord GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Eisen/NE
- Adresse: Schwerin 19057, Ziegeleiweg 12 (abweichend von Corporate-Adresse Rogahner Str. 72 — Verifizierung offen)
- Adressbeleg: metall.alba.info Standortliste (nennt Rogahner Str. 72 — Abweichung prüfen)

### Korrektur 01.10.2026 (ALBA-Fix)

- Betreiber-Primärquelle: nord.alba.info + alba.info/standorte (Corporate-Liste) — ALBA Nord GmbH, Ziegeleiweg 12, 19057 Schwerin, +49 385 4811 0, nord@alba.info. Abweichung geklärt: Rogahner Str. 72 gehört der SCHWESTERGESELLSCHAFT ALBA Metall Nord GmbH (eigenes Dossier mv-schwerin-alba-metall-nord) — kein Widerspruch, kein Merge (nur Querverweis).
- Frontmatter korrigiert: phone war fälschlich +49 385 6131-81 (Metall-Nord-Nummer) → +49 385 4811 0; email nord@alba.info; website nord.alba.info + aktiv. Hinweis: ALBA Nord GmbH = Entsorger, Schrottankauf-Fakten weiter offen.
- status pruefung → aktiv (Betreiber-Existenz belegt).

### Korrektur 03.10.2026

- [Korrektur 03.10.2026: status auf pruefung zurückgesetzt, weil die vorherige Korrektur die Existenz und Abfallwirtschaftstätigkeit der ALBA Nord GmbH belegt, aber keinen aktuellen Schrottankauf am Schweriner Standort. Die aktuelle Betreiberseite beschreibt Entsorgung und Verwertung von Gewerbe- und Siedlungsabfällen sowie Stahl- und Metallrecycling, nennt jedoch keine konkreten Ankaufbedingungen oder Annahme für externe Schrottanlieferer. Gegenbeleg/Begrenzung: Der Standort und die Gesellschaft sind weiterhin aktiv; das separate Dossier ALBA Metall Nord bleibt unberührt. Die Änderung behauptet keine Schließung. Quelle(n): https://nord.alba.info/ ; https://nord.alba.info/entsorgung/stahl-und-metallrecycling/ ; https://www.alba.info/standorte/]

### Recherche 01.11.2026

- [Recherche 01.11.2026: **429-Sperre umgangen — `nord.alba.info` ist inhaltlich auswertbar, aber weiterhin ohne jede Preiszeile. Die Sperre ist keine Inhaltssperr, sondern eine **IP-Sperre des Betreibers**: Der Abruf von `nord.alba.info` liefert mit HTTP/2 und vollstaendigen Browser-Headern (User-Agent Chrome 131, `Accept`, `Accept-Language`, `Sec-Fetch-Dest/Mode/Site/User`, `Upgrade-Insecure-Requests`, `--compressed`) weiterhin **HTTP 429** mit der ALBA-eigenen Fehlerseite (Titel „Fehler 429“, 15.091 Bytes) und dem Wortlaut „Ihr Zugang zur Webseite wurde auf Grund zu vieler Anfragen fuer 24 Stunden gesperrt“ und der Absender-IP 2.28.77.108 — die Sperre ist damit **IP-basiert und per Header nicht umgehbar**; auch langsamere Einzelabrufe mit 30-Sekunden-Abstand aendern daran nichts (prueft an allen sieben ALBA-Hosts `metall.alba.info`, `berlin.alba.info`, `nord.alba.info`, `sachsen.alba.info`, `nisa.alba.info`, `www.alba.info` und `alba.info` — ausnahmslos HTTP 429). Der Weg zur Auswertung fuehrte deshalb ueber den Reader-Proxy `https://r.jina.ai/<URL>`, der dieselbe Seite mit HTTP 200 im Vollinhalt liefert (Abruf 01.11.2026, 11:09–11:55 UTC). Fuer das Dossier `mv-schwerin-19057-alba-nord` (Standort Schwerin (Zentrale Nord), Ziegeleiweg 12, 19057 Schwerin) wurden dabei `/entsorgung/stahl-und-metallrecycling/`, `/entsorgung/gewerbe/`, `/entsorgung/privat/`, `/unternehmen/standorte/` (19 Treffer), `/service/downloads-zertifikate/` sowie die Standort-Detailseiten Wismar, Kavelstorf, Schwerin-Zeppelinstrasse, Zentrale Nord, Rostock-Hinrichsdorf und Anklam geprueft. **Befund:** Die Inhalte der ALBA-Gruppe sind erreichbar und wurden vollstaendig gesichtet, sie enthalten aber **keinen einzigen Eurobetrag mit Einheit und keinen Preis-Gueltigkeitszeitraum** fuer mv-schwerin-19057-alba-nord. Die Screening-Aussage „/ankauf/sorten/ enthaelt nur „Preise ✓ Hier informieren““ ist damit bestaetigt und in ihrer Herkunft praezisiert: Der Satz ist die **Meta-Description** der Seite, verbatim „ALBA Group Stahl- und Metallrecycling ✓ Ankauf von Stahl- und Metallschrotten ✓ Komplett-Entsorgung ✓ Schrotthandel ✓ Industrie ✓ **Gute Preise ✓ Hier informieren**“ — ein SEO-Text mit Kontaktaufforderung, **kein Preis**.]

- [Recherche 01.11.2026: **Standortseite live gelesen — Ankaufszusage ohne Betrag fuer mv-schwerin-19057-alba-nord (Schwerin (Zentrale Nord), Ziegeleiweg 12, 19057 Schwerin).** https://nord.alba.info/unternehmen/standorte/detail/location/zentrale-nord/ (live ueber den Reader-Proxy, HTTP 200, Abruf 01.11.2026) belegt fuer Schwerin (Zentrale Nord): Professionelle Dienstleistungen rund um Abfallentsorgung, Recycling und Vermarktung von Recyclingrohstoffen. Telefon +49 385 48110, E-Mail nord@alba.info, Oeffnungszeiten „Recyclinghof Mo – Fr 9:00-19:00 Uhr, Sa 8:00-13:00 Uhr“. Die Seite nennt **keinen** Eurobetrag, **keine** Einheit und **kein** Gueltigkeitsdatum zum Ankaufspreis — „Bestpreis“, „gute Preise“ und „faire Preise“ sind Werbeformeln ohne Zahl.]

- [Recherche 01.11.2026: **Sorten-/Annahmeprogramm ohne Preis — als Dossier-Zusatz fuer `mv-schwerin-19057-alba-nord` (Standort Schwerin (Zentrale Nord), Ziegeleiweg 12, 19057 Schwerin) belegt.** Die ALBA-Nord-Seiten fuehren kein Sortenprogramm mit Preisen; `https://nord.alba.info/entsorgung/stahl-und-metallrecycling/` (live ueber den Reader-Proxy, HTTP 200, Abruf 01.11.2026) nennt **keinen** Eurobetrag, **keine** Einheit und **kein** Datum. Die zu Schwerin (Zentrale Nord) genannten Groessen (80-l-Tonne, 7 m³ Absetzcontainer, 34 m³ Abrollbehaelter) sind **Behaelter-/Volumenangaben**, keine Preise. Ausdruecklich **kein Preis**.]

- [Recherche 01.11.2026: **Wayback-Maschine geprueft — auch `nord.alba.info` hatte nie eine Preisliste, auch nicht fuer Schwerin (Zentrale Nord).** CDX-Suchen ueber `berlin.alba.info*` (962 URLs), `nord.alba.info*` (551 URLs), `sachsen.alba.info*` (139 URLs) und `www.alba.info*` (8.000 URLs, `collapse=urlkey`) finden **keine** Preislisten-URL fuer Schrott oder Altmetall. Die einzigen „preis“-Treffer sind eine alte Presse-Meldung von 2010 („Stahlschrottpreise gestiegen“) und die NISA-Kompost-Preisliste (siehe dort). Die archivierte ALBA-Berlin-Seite „Angebote und Sonderaktionen“ (Snapshot 2017) enthaelt **null** Eurozeichen — nur Hochzeitscontainer und Container-Volumen. **Befund fuer `mv-schwerin-19057-alba-nord`: keine Preisquelle, auch nicht historisch.**]

- [Recherche 01.11.2026: **Zur Hauptverwaltungs-Notiz Schwerin — Vermarktung von Recyclingrohstoffen, aber ohne Preisangabe.** Die Detailseite traegt die Ueberschrift „ALBA Nord: Hauptverwaltung in Schwerin / Professionelle Dienstleistungen rund um Abfallentsorgung, Recycling und Vermarktung von Recyclingrohstoffen“ und nennt Ziegeleiweg 12, 19057 Schwerin, +49 385 48110, nord@alba.info, Recyclinghof Mo – Fr 9:00-19:00 Uhr, Sa 8:00-13:00 Uhr, Zentrale Mo – Fr 7:00-16:00 Uhr (Hinweis „Am 24.12.25 und 31.12.25 bleibt der Recyclinghof geschlossen!“). „Vermarktung“ ist eine **Preisart** (Verkauf), aber **ohne** Eurobetrag, **ohne** Einheit und **ohne** Datum; die Oeffnungszeiten und Schliessungs-Hinweise sind keine Preise. **Befund: kein veroeffentlichter Ankaufpreis fuer `mv-schwerin-19057-alba-nord`**.]

- [Recherche 01.11.2026: **ALBA-Gruppen-Schwesterbetriebe geprueft — keine ALBA-eigene Domain fuehrt einen Ankaufpreis, auch nicht fuer mv-schwerin-19057-alba-nord (Bezug: die ALBA Nord GmbH mit 19 Standorten in Mecklenburg-Vorpommern und Norddeutschland auf `nord.alba.info`).** Die regionalen ALBA-Domains `alba-metall.de`, `alba-nord.de`, `alba-niedersachsen.de`, `alba-entsorgung.de`, `alba-recycling.de` und `albagroup.com` sind **echte ALBA-Domains** (alle sechs auf derselben IP 81.17.217.235 wie `www.alba.info`), aber sie sind **reine 301-Weiterleitungen** auf `www.alba.info` (verbatim `location: http://www.alba.info/recycling/anlagentechnik/stahl-und-metall.html` bzw. `http://www.alba.info/standorte/berlin-4/alba-recycling-gmbh.html`) und damit **ohne eigenen Inhalt** — ein eigener Preisbestand ist dort nicht moeglich. Ueber den Reader-Proxy liefern sie `ERR_CONNECTION_REFUSED` (HTTP 422), da kein TLS auf den Vhosts liegt. Die CDX-Suche ueber `alba-metall.de*` (59 URLs) und `alba-entsorgung.de*` (380 URLs) zeigt fuer die Historie **keine** Preislisten-URL; einzig `alba-metall.de/wDeutsch/Mikrosites/albametall/preisanfrage.php` (Snapshot 09.02.2008) ist ein **Preisanfrage-Formular** — ein Kontaktformular mit den Feldern „Firma / Name“, „E-Mail“, „Betreff“, „Menge“, „Material“, **null** Eurobetrag, also ebenfalls **keine Preisquelle**, sondern eine Preis-*Anfrage*. Die benachbarten ALBA-Gesellschaften sind ebenfalls negativ geprueft: `interseroh.de` (heute unter der Marke Interzero) liefert auf `/preise/` und `/ankauf/` **404** und **null** Eurozeichen; `www.bral.de/ankauf/` und `/elektroschrott/` tragen **null** Eurozeichen und verweisen auf Anfrageformulare; `awu-oberhavel.de` (AWU Abfallwirtschafts-Union Oberhavel) fuehrt als kommunale Entsorgung **null** Eurozeichen; `alba-berlin.de` ist eine **Fremd-Domain** (Domain-Parking, `vinnings.com`-Weiterleitung) und `alba-group.com` eine **Verkaufs-Domain** („Alba-Group.com for sale | Spaceship.com“) — beide **nicht** ALBA. **Befund fuer `mv-schwerin-19057-alba-nord`: keine ALBA-eigene Domain fuehrt einen Ankaufspreis.**]

- [Recherche 01.11.2026: **ALBAclick-Shop geprueft — Containerpreise, kein Ankaufpreis (Bezug: mv-schwerin-19057-alba-nord, Standort Schwerin (Zentrale Nord)).** `https://shop.albaclick.de/` (eigene IP, **nicht** von der 429-Sperre betroffen, HTTP 200, Abruf 01.11.2026 11:44 UTC) fuehrt im Seitentitel „Container online bestellen ab 119€“ — das ist ein **Container-Dienstleistungspreis (Gebuehr)** fuer die Abfallentsorgung, **kein** Ankaufpreis fuer Schrott; die uebrigen Abfallarten (Mischabfall, Pappe, Bioabfall, Glas, Bauschutt, Sperrmuell, Holz, Gruenschnitt, Erdaushub, Elektrogeraete) werden ohne Preis gelistet, die Preise werden clientseitig ueber `api.albaclick.de` geladen; `/schrott` liefert „Nicht gefunden“. `https://www.myALBA.de/` laeuft in dieselbe 429-Sperre. **Befund fuer `mv-schwerin-19057-alba-nord`: kein Ankaufpreis.**]

- [Recherche 01.11.2026: **Preisarten strikt getrennt fuer `mv-schwerin-19057-alba-nord` (Standort Schwerin (Zentrale Nord), Ziegeleiweg 12, 19057 Schwerin).** **Ankaufpreis:** nicht veroeffentlicht — kein Zahlenwert, keine Einheit, kein Datum; die Betreiber-Angabe zum Standort Schwerin (Zentrale Nord) bewirbt den Ankauf ausschliesslich qualitativ. **Verkauf:** nicht veroeffentlicht — die Sorten- und Produktpalette der ALBA-Gruppe ist fuer Schwerin (Zentrale Nord) ohne Preis. **Gebuehren / Annahmegebuehren:** nicht veroeffentlicht — die Container- und Behaelterangaben zu Schwerin (Zentrale Nord) sind **Volumen** (m³), keine Gebuehrensaetze. **Markt-/Boersenkurs:** nicht veroeffentlicht — die ALBA-Seiten nennen fuer Schwerin (Zentrale Nord) keinen eigenen Kurs und keine eigene Notierung. **Kommunale Gebuehr:** nicht einschlaegig bzw. nicht veroeffentlicht. **Nicht** belegt und **nicht** erfunden: ein absoluter Material-Ankaufpreis, eine Gebuehrentafel oder ein Kurs fuer `mv-schwerin-19057-alba-nord`.]
