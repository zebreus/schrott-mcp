---
slug: nw-essen-eie
clear_fields: geo
name: EIE
trader_type: schrotthaendler
state: NW
city: Essen
street: Gelsenkirchener Str. 181
postcode: '45309'
phone: 0201 21962114
email: ''
opening_hours: ''
website: ''
website_status: tot
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# EIE

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Owner-Quellenkorrektur 27.10.2026: Zwei-Quellen-Beleg der Adresse

- [Owner-Quellenkorrektur 27.10.2026: Die Adresse Gelsenkirchener Str. 181, 45309 Essen ist inzwischen zweifach unabhängig belegt: Das Örtliche führt EIE Services GmbH an dieser Anschrift mit der Telefonnummer 0201 21962114 und der Branche „Schrott in Essen“, der 11880-Eintrag vom 09.10.2026 wiederholt Adresse und Nummer. Die im Verzeichnis genannte Website `deinschrottplatz.de` ist **kein** EIE-Betreiberportal: Das archivierte Impressum vom 26.02.2021 gehört der ScrapBees GmbH, Platz der Ideen 2, 40476 Düsseldorf, HRB 91074, und die Domain ist heute nicht mehr auflösbar; `website_status: tot` bleibt damit zutreffend. Zugleich fällt die Nominatim-Vorwärtssuche auf Museumsgebäude (Ruhr Museum, Red Dot Design Museum, Schalthaus 2), die Hausnummer 181 trägt also keine erkennbare Schrotthof-Situation; die Adresszeile selbst bleibt deshalb als eigene offene Frage stehen. Die GEO-Zurücknahme vom selben Tag bleibt davon unberührt, eine Ersatzkoordinate wird nicht gesetzt. Quellen: https://www.dasoertliche.de/Themen/EIE-Services-GmbH-Essen-Schonnebeck-Gelsenkirchener-Str ; https://www.11880.com/branchenbuch/essen-ruhr/060440090B106476017/eie-services-gmbh.html ; http://web.archive.org/web/20210226092137/https://www.deinschrottplatz.de/impressum.php ; read-only `public.db` (27.10.2026). **Preise:** Ankauf — keine EIE-Preisliste gefunden, das Verzeichnisprofil nennt nur die Branche „Schrott in Essen“; Verkauf — keine Liste; Gebühren — keine Tarife belegt.]

### Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt zurücknehmen

- [Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt zurücknehmen: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`. Dieser Punkt liegt laut OSM auf der Straße „Freiheit“, 45128 Essen (way 46961889, `highway=service`, `bus=designated`, `vehicle=no`), genauer auf dem Fernbusbahnhof am Essener Hauptbahnhof (Haltestellenknoten 599294635, „Essen, Fernbusbahnhof“). Das ist eine Busspur und kein Gewerbestandort; in den Umweltinspektionsberichten der Bezirksregierung Düsseldorf für die Stadt Essen gibt es keinen Eintrag auf dieser Straße. Das Dossier nennt Gelsenkirchener Str. 181, 45309; die Nominatim-Vorwärtssuche trifft „Ruhr Museum, 181, Gelsenkirchener Straße, Stoppenberg“ (51.485864/7.042187), rund 4,38 km vom gespeicherten Punkt entfernt. Der gespeicherte Punkt widerspricht damit der eigenen Adressangabe dieser Zeile. Offen bleibt die Adresszeile selbst, weil der Vorwärtstreffer auf ein Museum fällt und weder Betreiber-Impressum noch Register- oder Behördenbeleg für EIE Services GmbH vorliegen. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Gelsenkirchener+Str.+181%2C+45309+Essen&format=jsonv2&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/way/46961889/full.json ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 02.10.2026

- [Recherche 02.10.2026: OK (Ausnahmefall, offen dokumentiert — kein Betreiber-/Register-Beleg, aber 4 konsistente Aggregatoren): EIE = EIE Services GmbH, Gelsenkirchener Str. 181, 45309 Essen-Schonnebeck, Tel. 0201 21962114, Kategorie Schrott/Alteisen. Belegt uebereinstimmend durch DasTelefonbuch (Adresse+Tel.+Oeffnungszeiten Mo-Do 8-16/Fr 8-14, Mail eie@deinschrottplatz.de), Gelbe Seiten, Das Oertliche, cylex und city-advisor (alle Gelsenkirchener Str. 181). Eingetragene Homepage www.deinschrottplatz.de ist TOT (DNS-Aufloesung scheitert 02.10.2026, curl exit 6) → website leer, website_status tot. Mail-Adresse wegen toter Domain NICHT ins Frontmatter uebernommen. Expliziter Ankaufbeleg fehlt (nur Branchenkategorie) → Status bleibt pruefung; Koordinaten neu zu geocodieren (Adress-Fill). Quelle(n): DasTelefonbuch-Eintrag + Gelbe Seiten + Das Oertliche + cylex + city-advisor (Abruf 02.10.2026).]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Essen
