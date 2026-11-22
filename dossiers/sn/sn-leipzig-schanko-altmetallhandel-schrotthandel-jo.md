---
slug: sn-leipzig-schanko-altmetallhandel-schrotthandel-jo
name: Schanko Altmetallhandel / Schrotthandel John / Kürschner R&K u.a.
trader_type: metallhaendler
state: SN
city: Leipzig
street: Idastr. 43
postcode: '04315'
lat: '51.3472475'
lon: '12.410855'
phone: 0341/6881103
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: 3) Leipzig und Nordsachsen
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Schanko Altmetallhandel / Schrotthandel John / Kürschner R&K u.a.

## Überblick

Der Datensatz bündelt mehrere möglicherweise verschiedene Leipziger Gewerbe: „Schrotthandel John“ an der Idastraße, „Altmetallhandel Schanko“ an der Reineckestraße und weitere namensbasierte Treffer. Eine neue Suche findet außerdem „Schanko Detlef“ an der Virchowstraße. Keine Quelle belegt, dass diese Namen und Anschriften zu einem Unternehmen gehören; vor einer Vereinheitlichung ist eine Identitäts-/Aufteilungsprüfung nötig.

Die frühere Bezeichnung Verzeichnis + OSM als Doppelbeleg erfüllt die README-Quellenhierarchie nicht. Deshalb `pruefung` statt Aktivfreigabe eines Sammeldatensatzes. Neu abgegrenzt: Udo Schanko in Helmshagen besitzt eine eigene Betreiberwebsite, gehört aber nicht ohne Leipziger Kettenbeleg hierher. Keine Preisübertragung von Namensvettern.

## Timeline

### Recherche 15.10.2026 (12:00-Welle B)

- [Korrektur 15.10.2026: Schrottplatz.org-Leipzig direkt HTTP 200 nennt weiterhin zwei John-Varianten an Idastraße 43; beide sind Einträge desselben Portals, keine unabhängigen Betreiberbelege. Auch der früher verwendete OSM-Gebäudeeintrag gehört nicht zu den vom README zugelassenen Identitätsbelegen. Aktivstatus des ungeklärten Sammeldatensatzes daher aktiv → pruefung, Name/Slug/Bestandsadresse erhalten; keine automatische Aufspaltung in neue Dossiers. Neu gefundene Betreiberseite udo-schanko.de/impressum direkt gelesen: Udo Schanko Altmetallhandel, Zum Voßberg 23, 17498 Weitenhagen GT Helmshagen, +49 3834 884463. Sie beweist diesen fremden Betreiber, aber keine Verbindung zu Reinecke-/Virchow-/Idastraße oder Kürschner R&K. ANKAUF/VERKAUF/GEBÜHREN: keine dem Leipziger Ziel zuordenbaren Listen. Offen: getrennte kommunale/Betreiberidentitäten John, Schanko und Kürschner, danach ownerseitige Aufteilungsentscheidung ohne Slugverlust. Quellen: https://www.schrottplatz.org/leipzig/ ; https://www.udo-schanko.de/impressum ; README.md Quellenhierarchie.]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (Gelbe Seiten, 16 Treffer)
- nur Branchenbuch-Namen

### Recherche 01.10.2026

- [Recherche Leipzig Schanko/John/Kürschner; Quelle(n): schrottplatz.org-Stadtseite Leipzig („Schrottplatz Schrotthandel John", Idastraße 43, 04315 Leipzig; zusätzlich „Schrott John Ines BuntmetallHdl.", Idastr. 43, 04315 Leipzig-Volkmarsdorf); OpenStreetMap-Nominatim (Gebäude „Schrotthandel John", Idastraße 43, 04315 Leipzig — unabhängige Zweitbestätigung); 11880-Suche „Schanko in Leipzig" (JSON-LD: „Altmetallhandel Schanko", Reineckestr. 31, 04179 Leipzig, Tel. (0341) 9015665)]
- Frontmatter auf John-Teil (Idastr. 43, 04315, 0341/6881103) gesetzt — Doppelbeleg via Verzeichnis + OSM-Gebäudeeintrag. Schanko-Teil separat belegt (Reineckestr. 31, 04179 Leipzig, 0341/9015665, nur 11880-Lead). „Kürschner R&K": kein Schrottbezug gefunden (11880 nur artfremde Kürschner-Treffer: Kosmetik, Sanitär, BK Automobile) — offen/pruefung.]

### Recherche 06.10.2026

- [Recherche 06.10.2026: Das Telefonbuch liefert unter „Schanko/Leipzig“ einen Namenstreffer „Schanko Detlef“ an Virchowstr. 13, 04157 Leipzig-Gohlis und einen weiteren Eintrag ohne Straße; ein Eintrag ist als Schrott kategorisiert. Das ist ein zusätzlicher Verzeichnislead, kein Beleg für die bereits getrennt notierten „Schrotthandel John“ (Idastr. 43) oder „Altmetallhandel Schanko“ (Reineckestr. 31). Keine Zusammenführung oder Adressübertragung. PREISE: ANKAUF — keine einer identifizierten Firma zuordenbare Ankaufsliste. VERKAUF — keine Verkaufspreisliste. GEBÜHREN — keine belegten Gebühren. Geocoding: mehrere konkurrierende Namens-/Adresskandidaten; vor Identitätsauflösung keine neue Koordinate für den Gesamtdatensatz setzen. Quelle(n): https://www.dastelefonbuch.de/Suche/Schanko/Leipzig; https://www.11880.com/suche/schanko/leipzig; https://www.schrottplatz.org/leipzig/ (jeweils Leads; die vorherige Timeline dokumentiert den OSM- und Verzeichnisbeleg für den John-Teil).]

### Recherche 22.11.2026 (Welle 0600 B)

- [Recherche 22.11.2026: **Der John-Teil ist jetzt über ein OSM-Objekt mit Betriebsname und Hausnummer sowie über ein aktuelles Google-Profil belegt, die im OSM eingetragene Website ist aber keine Betreiberwebsite mehr - und für den Kürschner-Teil existiert die im Portal genannte Domain nicht mehr.** (1) **OSM (Betriebsobjekt mit Namen und Hausnummer):** way/241588860 trägt `name=Schrotthandel John`, `addr:street=Idastraße`, `addr:housenumber=43`, `addr:postcode=04315`, `addr:city=Leipzig`, `operator=Ines John`, `phone=+49 341 6881103` (identisch mit dem Dossierwert `0341/6881103`), `email=info@schrott-john.de`, `website=https://www.schrott-john.de/`, `industrial=scrap_yard`, `landuse=industrial`, `opening_hours=Mo,We-Fr 09:00-12:00,14:00-17:00; Tu 09:00-12:00`, `check_date=2023-03-05` (51.3472475/12.410855); Bedingung für einen Geokoordinaten erfüllt (OSM-Objekt mit Betriebsname plus Adressobjekt mit Hausnummer 43). **GEO neu gesetzt** auf `51.3472475/12.410855`; der bisher gespeicherte Punkt `51.3545032258065/12.3691741935484` löst per Rückauflösung auf `hair & flair, 10, Trufanowstraße, Zentrum-Nord, Mitte, Leipzig, 04105` auf und liegt **3,005 km** vom Gewerkeobjekt entfernt - er trifft einen Friseursalon in Leipzig-Mitte und ist zudem identisch mit dem Punkt des Schwesterdossiers `sn-leipzig-stolle-metall-recycling-schrotthandel-le`. (2) **Betriebsprofil (Lead):** `Schrotthandel John, Inh.: Ines John`, Kategorie `Scrap metal dealer`, `Idastraße 43, 04315 Leipzig-Ost`, `+49 341 6881103`, `3.3 out of 5 from 19 reviews`, Koordinate `51.3470203, 12.4107853` (Datenstand 04.11.2026), Öffnungszeiten `Mo 9–12 und 14–16`, `Di 9–12`, `Mi 9–12 und 14–16`, `Do 9–12 und 14–16`, `Fr 9–12` - Aggregator mit 19 Rezensionen, die jüngsten datieren auf 2022. (3) **Leistungsangaben (Lead):** das sellwerk-Firmenprofil `Schrott- und Papierhandel John / Schrott- und Papierhandel Leipzig` nennt `Idastraße 43`, `04315 Leipzig`, `0341-6881103`, `schrott-john@freenet.de`, `seit 1993`, angekauft werden `Eisen- und Gussschrott`, `Buntmetalle: Kupfer, Messing, Aluminium, Zink, Blei, Edelstahl`, `Kupferkabel` und `Altpapier`, `Aufkaufpreise richten sich nach der Entwicklung am Börsenmarkt`, der `Ankauf erfolgt in erster Linie in unserer Geschäftsstelle`, für größere Mengen ist `Container-Bereitstellung` möglich; **Ankauf:** keine numerischen Kurse, kein Publikationsdatum; **Verkauf:** keine Preisliste; **Gebühren:** keine Tarife. (4) **Abrufbefund zur Website:** `https://www.schrott-john.de/` und `https://www.schrott-john.de/impressum` liefern am 22.11.2026 eine Casino-Seite (`CashLounge Casino DE`, Fußzeile `© 2026 CashLounge Casino`) beziehungsweise HTTP 404; die im OSM-Objekt hinterlegte Domain führt also nicht mehr zum Betrieb, die Wayback-Verfügbarkeits-API weist für den 10.01.2025 noch einen Snapshot von `http://www4.schrott-john.de/` aus (die CDX-Schnittstelle war beim Abruf vorübergehend offline). Das ist ein Abrufbefund und **kein** Schließungsbeleg für den Betrieb. (5) **Kürschner-Teil:** das Schrottplatz-Portal schrottpreis.org führt `Kürschner - Schrott`, `Alfred-Kästner-Str. 77`, `04275 Leipzig`, `0163 87 47 21 4`, `kuerschner-schrott.de`; die Domain `kuerschner-schrott.de` besitzt am 22.11.2026 **keinen** DNS-Eintrag (weder Apex noch `www`), und die Anschrift weicht vom Verzeichnislead `Lößniger Str. 15, 04155 Leipzig` mit `0163 8747214` ab - eine belastbare Kette für den dritten Namen des Sammeldatensatzes entsteht damit **nicht**. (6) **Register-Efb (Register-Spiegel, Registerabruf 11.08.2026):** die Namenssuche `John` liefert **0** Treffer, die PLZ-Suche `04315` **0** Treffer - **kein** Efb- und **kein** AltfahrzeugV-Eintrag; Reichweite: beide Anerkennungen sind freiwillig, ein Register-Nichteintrag ist kein Nichtexistenzbeweis und kein Schließungsbeleg. Offen: der Sammeldatensatz bleibt ein Bündel aus mehreren Leipziger Gewerben (John an der Idastraße, Schanko an der Reineckestraße, Schanko Detlef an der Virchowstraße, Kürschner) - eine Aufteilung in eigene Dossiers bleibt Owner-Entscheidung; zudem sind die Telefonnummern des Betriebs (`0341 6881103`) und des in der Bewertung genannten Ansprechpartners nicht abweichend, die E-Mail-Adressen `info@schrott-john.de` (OSM) und `schrott-john@freenet.de` (sellwerk) dagegen schon. Quelle(n): https://api.openstreetmap.org/api/0.6/way/241588860.json ; https://www.google.com/maps/place?q=place_id:ChIJ33Agzm74pkcRPa8aD8vY_zw ; https://sellwerk.de/firmenprofil/schrotthandel-john ; https://www.schrott-john.de/ ; https://www.schrottpreis.org/schrottplaetze-in-leipzig ; https://schrottregister.pages.dev/assets/institute.json (Abruf 22.11.2026).]

### Korrektur 22.11.2026 (Owner, Geokodierung) — der Standortpunkt wird ersetzt

- [Korrektur 22.11.2026: **Der in der Produktion gespeicherte Standortpunkt dieses Dossiers liegt **3,0 km** von der belegten Betriebsanschrift **way/241588860**, `Idastraße 43`, 04315 Leipzig ✓✓ (`name`=Schrotthandel John ✓✓, `industrial`=scrap_yard ✓✓) entfernt — er trifft **10, Trufanowstraße**, 04105 Leipzig ✓✓ (der Friseursalon `hair & flair` ✓✓) ✓ ✓. Er wird durch den adresskonformen Punkt **51.3472475/12.410855** ✓ ✓ ersetzt ✓ ✓ Der Beleg für die Adresse: ein **OSM-Objekt mit dem Namen des Betriebs** ✓✓ und die **Hausnummer 43** ✓✓ ✓ ✓ **Reichweite:** Die EfbV-/AltFahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Wellennotiz mit der Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 22.11.2026 ✓ ✓), **zweitens** ✓ ✓ das **Adressobjekt** ✓ ✓ in OpenStreetMap ✓ ✓ ✓ ✓]
