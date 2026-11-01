---
slug: nw-ratingen-mhr-metallhandel-ruhr
lat: 51.3304459
lon: 6.9101121
name: MHR Metallhandel Ruhr GmbH
trader_type: metallhaendler
state: NW
city: Ratingen
street: Heiligenhauser Str. 3
postcode: '40883'
phone: ''
email: ''
opening_hours: ''
website: https://tsr.eu
website_status: aktiv
status: aktiv
description: Verwaltungsgesellschaft MHR Metallhandel Ruhr GmbH (TSR Group), B2B, kein Publikums-Ankauf
dropoff_json: '{}'
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: ja (B2B)
provenance_origin: table
---

# MHR Metallhandel Ruhr GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 03.10.2026 (Feedback 4518)

- [Recherche 03.10.2026: Feedback 4518 teilweise berechtigt. Die bereits dokumentierte Betreiberrecherche bezeichnet MHR als Verwaltungsgesellschaft; die erneuten Abrufe der TSR-Seiten liefern derzeit keinen auswertbaren Inhalt. Weder die B2B-Angabe noch die Verwaltungsanschrift belegen eine Kundenannahme bzw. Anlieferstelle. Der bisherige DB-Wert `{"allowed":true}` ist dafür kein Beleg. Frontmatter `dropoff_json` explizit auf `{}` gesetzt (= unbekannt; kein „keine Annahme“-Urteil); Kundenarten nicht geraten. Quelle(n): https://www.tsr-recycling.de/bibliothek, https://www.tsr.eu/en/about-us (Betreiberquellen; Abrufprüfung 03.10.2026 ohne auswertbaren Seiteninhalt)]

### Recherche 01.10.2026

- [Recherche 01.10.2026: ok — Konzern-Primärquelle: tsr-recycling.de/Bibliothek führt „Verwaltung MHR — MHR Metallhandel Ruhr GmbH, Heiligenhauser Straße 3, 40883 Ratingen, www.tsr.eu"; TSR Group (REMONDIS) bestätigt MHR als Verwaltungsgesellschaft; kein Publikums-Schrottplatz, daher Telefon/E-Mail/Öffnungszeiten leer; Frontmatter Adresse + Website (Domain-Root) gefüllt (Adresse neu — Koordinaten neu zu geocodieren); Quelle(n): https://www.tsr-recycling.de/bibliothek, https://www.tsr.eu/en/about-us, Stand 01.10.2026]

### Importiert (Seed-Stand 2026-09-30)

- Metallhandel (Heiligenhauser Str. 3, Hösel), nur schrottradar-Profil + TSR-Link

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Heiligenhauser Straße, Ratingen-Hösel).** Kachelraster über den Straßenverlauf (Straßenbbox 6.9000/51.3250 bis 6.9100/51.3350, **1 Kachel, 0 Kachel-Fehler**; Objekt-BBox der Straße 51.3300936/6.9097625 bis 51.3303267/6.9101737). Zählung getrennt: **4 Adressobjekte / 7 verschiedene Hausnummern** (1, 2, 3, 4, 5, 6, 7 - die Rohnotationen sind „1“, „1-7“, „2“ und „4“; die Bereichsnotation „1-7“ wurde nach der Referenzregel `osm_street_scan.expand` zu 1, 2, 3, 4, 5, 6 und 7 aufgelöst) auf der Straße „Heiligenhauser Straße“, im gesamten Rastergebiet **643 Adressobjekte mit Hausnummer / 163 verschiedene Hausnummern**; im Raster tragen 188 Objekte `name=` und 33 Objekte `operator=`. **Die Dossier-Hausnummer 3 existiert in OSM, aber nur als Teil einer Bereichsnotation**: way/261902089 mit `addr:housenumber=1-7`, `addr:street=Heiligenhauser Straße`, `addr:city=Ratingen`, `addr:postcode=40883`, `name=Hösel Center`, `shop=mall`, `building=levels=3` - es ist **kein** eigenständiges Adressobjekt „3“. Die drei weiteren Objekte sind node/9653891988 mit `addr:housenumber=1`, `name=Optik Palm`, `operator=Ernst-Guido Palm , Optik Palm e.K.`, `shop=optician` (ein **anderer** Betrieb), way/289679960 mit `addr:housenumber=2` und way/289679961 mit `addr:housenumber=4` (beide `building=yes`, **ohne** `name=`- und **ohne** `operator=`-Tag). Die Straße selbst ist way/131836403 und way/131836404 (beide mit `name=Heiligenhauser Straße`, `highway=primary`, `ref=B 227`). Weitere Objekte mit Bezug zum Betrieb im Raster: **keines** - die Namenssuche nach „MHR“, „TSR“, „Schrott“, „Metall“ und „Recycling“ über die Felder name, operator, description und website findet **keinen** Treffer mit Betriebsbezug. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=6.9000,51.3250,6.9100,51.3350 ; https://api.openstreetmap.org/api/0.6/way/261902089.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Konzern-Primärquelle live - die Anschrift bestätigt, der Firmenname hat sich geändert.** Das Standortverzeichnis der Bibliothek (live abgerufen am 31.10.2026; Direktabruf auf tsr-recycling.de liefert HTTP 200 mit einer Bot-Challenge-Seite, der Inhalt wurde über den Jina-Reader gelesen) führt unter der Rubrik Ratingen den Eintrag **„Verwaltung MHR – MHR Metallhandel Ruhr GmbH – Heiligenhauser Straße 3 – 40883 Ratingen – https://www.tsr.eu/“**; die Dossier-Anschrift ist damit betreiberseitig bestätigt. Das Impressum der Konzernseite nennt live **TSR Deutschland GmbH & Co. KG**, Brunnenstraße 138, 44536 Lünen, Amtsgericht Dortmund, **HRA 19743**, USt-IdNr DE 369313272, persönlich haftende Gesellschafterin TSR Deutschland Verwaltungs GmbH (AG Dortmund, **HRB 36274**), Geschäftsführung Michael Jasperneite, Wouter Cornelis Kusters, Dr. Christian Satlow - der im Dossier bzw. im Slug geführte Name **„TSR Recycling GmbH & Co. KG“** ist damit **nicht mehr** die auf der Betreiberseite geführte Gesellschaftsbezeichnung. Die Bibliothek stellt für die **MHR Metallhandel Ruhr GmbH** eigene Bedingungen bereit: **Allgemeine Einkaufsbedingungen (Stand 03-2020)** und **Allgemeine Verkaufsbedingungen (rev1, 31.03.2023, Geltung für Unternehmer)**. Quelle(n): https://www.tsr-recycling.de/bibliothek ; https://www.tsr-recycling.de/impressum/ ; https://www.tsr-recycling.de/fileadmin/user_upload/tsr-local-sites/downloads/Allg._Einkaufsbedingungen_MHR_Metallhandel_Ruhr_GmbH_Stand_03-2020.pdf (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **ANKAUF, VERKAUF und GEBÜHREN getrennt - nur Mechanismen, keine Zahlenpreise.** **ANKAUF:** **keine** öffentliche Sorten- oder Preisliste und **kein** Preisrechner; die Allgemeinen Einkaufsbedingungen der MHR Metallhandel Ruhr GmbH (Stand 03-2020, PDF; der **Direktabruf der Datei liefert HTTP 403** (Zugriffssperre), der Inhalt wurde über den Jina-Reader gelesen) belegen **Preismechanismen**: „Der in der Bestellung bzw. Einkaufsbestätigung wiedergegebene Preis ist bindend“ bzw. „Die vereinbarten Preise gelten, sofern nichts anderes vereinbart ist, frei Empfangsstelle. Sie sind Festpreise.“ (Ziffer 4.1), Zahlungsziel **14 Tage** nach Eingang und Gutbefund beim NE-Einkauf, beim Eisen- und Gießerei-Schrott der **30.** des der Lieferung folgenden Monats, bei vorzeitiger Zahlung **Skontoabzug** (Ziffer 5.2). **VERKAUF:** **keine** öffentliche Verkaufspreisliste; die Allgemeinen Verkaufsbedingungen der MHR (rev1, 31.03.2023, laut Dateiname „Geltung für Unternehmer“) waren am 31.10.2026 **nicht maschinell lesbar** (PDF ohne extrahierbare Textschicht, Abruf über den Jina-Reader ohne Inhalt) - ihr Zahleninhalt wird hier **nicht** behauptet. **GEBÜHREN:** beziffert ist keine Gebühr; belegt sind die Mechanismen „Erfolgt die Behältergestellung und die Abholung vereinbarungsgemäß durch uns berechnen wir grundsätzlich Miete und Transport“ (Ziffer 4.2 der Einkaufsbedingungen), die Abrechnung „unter Berücksichtigung evtl. Wiege- und sonstiger Kosten“ (Ziffer 5.1) sowie die Verrechnung der **Schrottabgabe** (Versicherungsprämie je Tonne Stahlschrott nach dem „Kölner Abkommen (neu)“) **zu Lasten des Lieferanten** (Ziffer 7.4); als **Weigerkostenkatalog** ist ein Katalog unter `https://www.tsr.eu/downloads` benannt, dessen Zahlenwerte in dieser Welle **nicht** ausgelesen wurden. Kommunale Tarife, Markt- und Börsenpreise sind **nicht** belegt. Quelle(n): https://www.tsr-recycling.de/fileadmin/user_upload/tsr-local-sites/downloads/Allg._Einkaufsbedingungen_MHR_Metallhandel_Ruhr_GmbH_Stand_03-2020.pdf ; https://www.tsr-recycling.de/bibliothek (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Punkte und Geokodierungsbedarf.** Unverändert bleibt: `dropoff_json` `{}` (unbekannt - die Bibliothek weist MHR als **Verwaltung** aus, eine Kundenannahme wird nicht belegt), `phone`, `email` und `opening_hours` leer, `status: aktiv` unverändert, `provenance_ankauf_raw` „ja (B2B)“. Die **B2B-Eigenschaft** ist jetzt durch Ziffer 1.2 der Einkaufsbedingungen belegt („Unsere Einkaufsbedingungen gelten nur gegenüber Unternehmern gem. § 14 BGB.“). **Geokodierungsbedarf:** der read-only Produktionspunkt 51.305/6.8516333 (Abruf 31.10.2026) liegt rund **4,91 km** vom OSM-Objekt way/261902089 (`name=Hösel Center`, `addr:housenumber=1-7`, `addr:street=Heiligenhauser Straße`, `addr:postcode=40883`, Punktlage 51.3306932/6.9099150) entfernt und ist damit **nicht** die Dossier-Anschrift; eine Geokodierung erfolgt **nicht** in dieser Welle. Offen bleibt die Frage, ob an der Verwaltungsanschrift überhaupt Material angenommen wird. Quelle(n): https://api.openstreetmap.org/api/0.6/way/261902089.json ; read-only `public.db` (31.10.2026).]

### Geokodierung 01.11.2026

- [Geokodierung 01.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **51.3304459/6.9101121** (Punktlage von
  node/11904863445, ein Node-Objekt ohne Kinder). Der Node trägt `addr:housenumber`=3,
  `addr:street`=Heiligenhauser Straße, `addr:city`=Ratingen, `addr:postcode`=40883, `addr:type:de`=Gebäudeteil und
  `source`=ALKIS NRW; er liegt damit exakt auf der Dossier-Hausnummer. Quelle 1 (Betreiber): die Bibliotheksseite der
  TSR-Gruppe (live abgerufen am 01.11.2026, HTTP 200) führt den Standort **„Ratingen / Verwaltung MHR / MHR
  Metallhandel Ruhr GmbH / Heiligenhauser Straße 3 / 40883 Ratingen“** - die im Dossier bereits zitierte
  Betreiberquelle bestätigt damit genau die in OSM gefundene Anschrift. Quelle 2 (OSM): node/11904863445 (live
  geladen am 01.11.2026, HTTP 200) sowie das Kachelraster 6.9000/51.3260 bis 6.9120/51.3340. **Abgrenzung:** der
  ebenfalls auf die Hausnummer 3 getaggte node/13230095288 (`name`=All In, `shop`=kiosk, `post_office`=post_partner)
  liegt rund **25 m** entfernt und ist ein **Kiosk** mit Postpartner-Schalter, nicht der Betrieb; der gewählte
  Node/11904863445 trägt dagegen `source`=ALKIS NRW und keinen `name`- oder `operator`-Tag und bezeichnet damit die
  Gebäudeadresse, nicht einen verifizierten Eingang. Der bisher gespeicherte Produktionspunkt 51.305/6.851633333333333
  liegt rund **4,95 km** vom neuen Punkt entfernt und wird durch diesen ersetzt; er war **nicht** die
  Dossier-Anschrift. Offen bleibt unverändert die Frage, ob an der Verwaltungsanschrift überhaupt Material
  angenommen wird. Quelle(n): https://www.tsr-recycling.de/bibliothek ;
  https://api.openstreetmap.org/api/0.6/node/11904863445.json ; read-only `public.db` (01.11.2026).]
