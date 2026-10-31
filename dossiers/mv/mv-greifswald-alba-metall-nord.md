---
slug: mv-greifswald-alba-metall-nord
name: ALBA Metall Nord GmbH
trader_type: schrotthaendler
state: MV
city: Greifswald
street: Gützkower Landstr. 18
postcode: ''
phone: 03834 841059
email: ''
opening_hours: ''
website: https://metall.alba.info/unternehmen/standorte/
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Neue Betriebe, Website verifiziert, Ankauf ja
provenance_ankauf_raw: ja lt. Portal
provenance_origin: table
---

# ALBA Metall Nord GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Stahl/NE (lt. schrottradar MV-Übersicht); gleiche Adresse wie Greifen-Schrott GbR — mögl. Nachfolge/Teilung, nicht deduziert

### Korrektur 01.10.2026 (ALBA-Fix)

- Delisted-Protokoll: Greifswald steht NICHT auf der aktuellen Betreiber-Standortliste (Vollcrawl 01.10.2026: metall.alba.info/unternehmen/standorte = 13 Detailseiten, keine Greifswald; auch alba.info/standorte ohne Greifswald-Treffer). Ältere Liste hatte 17 Treffer inkl. Greifswald.
- Betreiber-Suche/Register/News ohne Schließungs-/Verlagerungsbeleg → kein status geschlossen, Dossier bleibt erhalten; status aktiv → pruefung + Klärfall: Filial-Existenz und Adressbeleg (Gützkower Landstr. 18, Tel. 03834 841059 — bislang nur Aggregator-Leads, kein Betreiber-Beleg) offen. Hinweis: ALBA Nord GmbH führt Betriebsstätte Gützkow (Am Kleinbahnhof 4, 17506 Gützkow) — Namensnähe, anderer Ort, kein Beleg für Zusammenhang.
- website = Domain-Root (aktiv, Impressum via alba.info); keine Detailseite.

### Recherche 01.10.2026

- [Recherche 01.10.2026 / Feedback-Triage 2343: Delisting berechtigt — ALBA-Fix-Befund bestätigt (Vollcrawl 01.10.2026: 13 Standorte ohne Greifswald; Gützkower Landstr. 18 + Tel. 03834 841059 ohne Betreiber-Beleg); Dossier bleibt pruefung; Quelle(n): metall.alba.info/unternehmen/standorte/, alba.info/standorte/ (Abruf 01.10.2026)]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Gützkower Landstraße, Greifswald) — ein Objekt nennt den Betrieb selbst.** Straßen-Raster über den **gesamten** Straßenverlauf (Straßenbbox 13.3794776/54.0612893 bis 13.3830883/54.0838611), Scan-Fenster **13.3774776/54.0592893 bis 13.3850883/54.0858611** (**3 Kacheln im 0,01°-Raster des Repo-Werkzeugs `scripts/release/osm_street_scan.py`, 0 Kachel-Fehler**). Zählung getrennt: **109 Adressobjekte / 93 verschiedene Hausnummern** auf der Straße (1, 2, 3, 3a, 3b, 4, 6, 7, 7a, 7b, 7c, 7d, 7e, 7f, 7g, 8, 8a, 8b, 8c, 8d, 8e, 8f, 8g, 8h, 9, 10, 10a, 10b, 10c, 11, 11a, 12, 13, 14, 15, 16, 17, 17a, 18, 19, 20, 21, 31, 32, 32a, 36, 37, 38, 39, 40, 46, 46a, 47, 47a, 54, 55, 63, 63a, 63b, 64, 69, 69b, 69c, 69d, 69e, 70, 71, 72, 79, 80, 81, 81a, 82, 82a, 83, 83a, 83b, 84, 84a, 84b, 84c, 84d, 84e, 85, 85a, 85b, 85c, 85d, 85e, 86, 87, 88, 89) - die Aufzählung nennt exakt die live vorhandenen Nummern; Filter-Key `addr:street=Gützkower Landstraße` (mit `addr:housenumber` als Hausnummer). Es sind **keine Bereichsnotationen** live vorhanden, aber **zahlreiche Buchstaben-Zusätze** (3a, 3b, 7a–7g, 8a–8h, 10a–10c, 11a, 17a, 32a, 46a, 47a, 63a, 63b, 69b–69e, 81a, 82a, 83a, 83b, 84a–84e, 85a–85e) — nach der Referenzzählung des Repo-Werkzeugs zählt **jede** Buchstabennummer als **eigene** Hausnummer. Im gesamten Rastergebiet **187 Adressobjekte mit Hausnummer / 148 verschiedene Hausnummern**; im Raster tragen **214 Objekte `name=`** und **105 Objekte `operator=`**. **Die Dossier-Hausnummer 18 existiert** und trägt **den Betrieb selbst**: **way/421349759** mit `addr:housenumber=18`, `addr:street=Gützkower Landstraße`, `addr:city=Greifswald`, `addr:postcode=17489`, `name=ALBA Metall Nord GmbH`, `operator=ALBA Metall Nord GmbH`, `industrial=scrap_yard`, `landuse=industrial`, `recycling:scrap_metal=yes`, `phone=+49 3834 841059`, `fax=+49 3834 841066`, `email=info@mab-rostock.de`, `website=https://metall-nord.alba.info/unternehmen/standorte/greifswald.html`, `opening_hours=Mo-Fr 08:00-17:00; Sa 09:00-11:30; PH off`. Das ist ein **positiver** Adress- und Identitätsbeleg — das Objekt nennt den **eigenen Betrieb** an der Dossier-Adresse, und sein `phone=+49 3834 841059` **deckt sich exakt** mit der Dossier-Telefonnummer 03834 841059. Weitere Objekte mit Bezug zum Betrieb im Raster: **ein weiteres** Schrott-Objekt mit eigenem Namen — **node/6517975236** mit `name=Altmetallhandel`, `addr:housenumber=6`, `addr:street=Helmshäger Straße`, `addr:postcode=17489`, `addr:city=Greifswald`, `industrial=scrap_yard`, `landuse=industrial`, `recycling:scrap_metal=yes`, `contact:phone=+49 3834 231 04 41` — ein **Fremdbetrieb** (anderer Name, andere Straße, kein Bezug zur ALBA Metall Nord GmbH). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=13.3774776,54.0592893,13.3850883,54.0858611 (Kachelraster, live abgerufen 31.10.2026) ; https://api.openstreetmap.org/api/0.6/way/421349759.json ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Korrektur 31.10.2026 (Korrekturvermerk zum ALBA-Fix vom 01.10.2026): die Adress- und Identitätslage ist jetzt durch OSM belegt, die Filial-Existenz bleibt es nicht.** Die Vorwellen-Notiz „Adressbeleg (Gützkower Landstr. 18, Tel. 03834 841059 — bislang nur Aggregator-Leads, **kein** Betreiber-Beleg) offen“ ist damit **teilweise überholt**: Das OSM-Objekt way/421349759 nennt den Betrieb **an genau dieser Adresse mit genau dieser Telefonnummer**. Das **bleibt** aber ein **OSM-Datenstand**, kein Betreiber-Primärbeleg: Die im Objekt hinterlegte Branchenseite **https://metall-nord.alba.info/unternehmen/standorte/greifswald.html** antwortet am 31.10.2026 mit **HTTP 404** („Page Not Found … No site configuration found“), und die aktuelle Betreiber-Standortliste **metall.alba.info/unternehmen/standorte/** (HTTP 200) nennt weiterhin **„13 Treffer zu Ihrer Auswahl“** — **ohne** Greifswald. Auch die Unter-URL https://metall.alba.info/unternehmen/standorte/greifswald liefert HTTP 404. Der im OSM-Objekt genannte Mailkontakt **info@mab-rostock.de** führt live auf **http://www.mab-rostock.de/** (HTTP 200), das auf die **ALBA-Korporativseite** („ALBA - Wir sind die Zukunft.“) weiterleitet und **keine** Greifswald-Standortseite mehr enthält. **Schlussfolgerung: der Standort war dokumentiert (OSM-Daten mit exakter Adresse und Telefonnummer), die Betreiber-Website führt ihn nicht mehr; ein Schließungs- oder Verlagerungsbeleg fehlt weiterhin.** `status: pruefung` und der Klärfall bleiben daher **unverändert** — ein Delisting ist **kein** Schließungsnachweis. Quelle(n): https://api.openstreetmap.org/api/0.6/way/421349759.json ; https://metall.alba.info/unternehmen/standorte/ ; http://www.mab-rostock.de/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Ankauf, Verkauf und Gebühren: Betreiber-Aussagen ohne Standort-Preisliste.** **ANKAUF:** Die Betreiber-Seite „Ankauf“ (metall.alba.info/ankauf/) sagt wörtlich: „Die Spezialist*innen von ALBA nehmen **auf all unseren Plätzen** alle Arten von Stahl- und Metallschrott **zu Bestpreisen** entgegen“ und der Gewerbe-Teil: „Vom **Ankauf** und der kompletten Entsorgung sämtlicher Eisen- und Nichteisenmetalle … bis hin zur digitalen Materialanalyse“ — die **Sorten** sind damit **betreiberseitig** beschrieben (Produktionsschrotte, Haushaltsschrott, Industrie- und Abbruchschrotte, Kabelschrotte, Mischschrotte, Vormaterial, Schwere Schere, Legierte Schrotte, Karossen, Aluminium, Kupfer, Messing, Brennschrott, Gusseisen, Moniereisen, Scherenschrott, Stahl- und Metallspäne auf der Sorten-Seite), **Zahlenpreise werden nicht veröffentlicht**. **VERKAUF:** die Startseite nennt „Verkauf“ als Leistungsfeld („Rohstoffe für die Industrie“) und die Standortseiten bieten „Nutzmaterial“ an — **ohne** Zahlenpreise. **GEBÜHREN:** **keine** Gebühr beziffert; die Container-Bestellung läuft über das Portal **ALBAclick** („buchen Sie direkt einen Container in unserem Online-Portal ALBAclick“), **ohne** Preisangabe. Quelle(n): https://metall.alba.info/ankauf/ ; https://metall.alba.info/ankauf/sorten/ ; https://metall.alba.info/ ; https://metall.alba.info/unternehmen/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen und Geokodierung.** (a) Der **Betreiber-Standort-Beleg für Greifswald fehlt weiterhin**; neu ist, dass die **Adresse selbst in OSM dokumentiert war** (mit exakter Telefonnummer) — die Frage verschiebt sich damit von „existiert die Adresse“ zu „ist die Filiale noch aktiv und unter welcher Adresse geführt“. (b) Die **PLZ** ist im Frontmatter **leer** und wäre durch das OSM-Objekt mit **17489** belegt; das Frontmatter-Feld wird hier **nicht** geändert, weil die Beleglage eine OSM-Angabe bleibt und die Betreiberquelle fehlt. (c) **Geokodierungsbedarf:** die Dossier-Adresse **Gützkower Landstr. 18, Greifswald** ist in OSM belegt (way/421349759, `addr:postcode=17489`); der im Frontmatter gespeicherte Punkt **54.0871000/13.4132667** stammt aus einer groben Seed-Koordinate und liegt rund **2,7 km** vom belegten Objektpunkt (13.3801372/54.0792998) entfernt — die Koordinate ist **nicht** gesetzt (Auftrag dieser Welle: Adressen benennen, nicht selbst geokodieren). Quelle(n): https://api.openstreetmap.org/api/0.6/way/421349759.json ; https://metall.alba.info/unternehmen/standorte/ (Abruf 31.10.2026).]

- [31.10.2026 (Korrekturvermerk): **Tag-Zitat entfernt**: way/421349759 trägt live **keinen** `addr:country`-Tag — das Dossier zitierte einen nicht vorhandenen Key. Die übrigen Tags des Objekts sind live bestätigt. Quelle(n): https://api.openstreetmap.org/api/0.6/way/421349759.json (Abruf 31.10.2026)]

- [31.10.2026 (Korrekturvermerk): **Fehlende Hausnummer ergänzt**: live trägt way/285737955 `addr:housenumber=64` auf der Gützkower Landstraße (v6 vom 01.06.2025) — die Aufzählung nannte die Nummer 64 nicht und war damit **eine zu kurz**; jetzt nennt sie exakt die **93** live vorhandenen Nummern (Bereichsnotationen „9-10“ und „13-16“ aufgelöst). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=13.3774776,54.0592893,13.3850883,54.0858611 (Abruf 31.10.2026)]
