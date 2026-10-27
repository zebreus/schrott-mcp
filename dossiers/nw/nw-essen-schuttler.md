---
slug: nw-essen-schuttler
clear_fields: geo
name: Schüttler
trader_type: mobil
state: NW
city: Essen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Schüttler

## Überblick

Identitätskandidat bleibt **Sylvia Schüttler Metallhandel**, Stubbenkamp 6, 45309 Essen-Kray, 0178 4299833, E-Mail-Lead `onvista@hotmail.com` – alles nur Verzeichnisangaben, kein belegter Alias des unspezifischen Seeds „Schüttler“. Die 11880-Seite kategorisiert u. a. Kleintransport, Montage, Altmetallrecycling, Möbeltransport und Schrotthandel, aber nennt keine Betreiberwebsite oder Öffnungszeiten. Creditreform-Suchauszug nennt weitere Handwerks-/Hausmeistertätigkeiten; Detailabruf HTTP 403, daher keine unabhängige Gewerbegegenstandsbestätigung. Keine öffentlich ausgelesene Ankauf-/Verkaufs-/Gebührenliste. Owner: Identität, heutige Tätigkeit und mobil versus öffentliche Annahmestelle klären; erst danach Adresse/GEO qualifizieren.

**Preise:** Ankauf: keine Schüttler-Kurse; Verkauf: keine Warenpreisliste; Gebühren: keine Dienstleistungstarife. Quelle (Verzeichnislead): https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html ; blockierter Lead: https://firmeneintrag.creditreform.de/45309/5110488342/SYLVIA_SCHUETTLER_METALLHANDEL

## Timeline

### Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`. Dieser Punkt liegt laut OSM auf der Straße „Freiheit“, 45128 Essen (way 46961889, `highway=service`, `bus=designated`, `vehicle=no`), genauer auf dem Fernbusbahnhof am Essener Hauptbahnhof (Haltestellenknoten 599294635, „Essen, Fernbusbahnhof“). Das ist eine Busspur und kein Gewerbestandort; in den Umweltinspektionsberichten der Bezirksregierung Düsseldorf für die Stadt Essen gibt es keinen Eintrag auf dieser Straße. Neu gefunden wurde eine bislang nicht erfasste Betreiber-Primärquelle: Das Impressum von „firma-schuettler.de“ (per CNAME identisch mit `xn--schttler-85a.com`) nennt Inhaberin Sylvia Schüttler, Stubbenkamp 6, 45309 Essen, Telefon 01577 1361724, und beschreibt als Tätigkeit Glasfaser-Verlegung und Innenausbau; die Startseite enthält keinen Treffer für „Metall“, „Schrott“ oder „Altmetall“. Die Nominatim-Vorwärtssuche zu Stubbenkamp 6 findet ein Wohnhaus (way 307631343, 51.4732855/7.0775926), rund 5,03 km vom gespeicherten Punkt entfernt; der Umfeldscan dort zeigt nur Textil-Altkleidercontainer, keinen Schrott- oder Metallhof. Damit widerspricht die eigene Betreiberquelle der Zeile dem gespeicherten Punkt, und der Punkt trägt für diese Zeile keine Aussage. `clear_fields: geo` nimmt ihn beim regulären Seed zurück; keine Ersatzkoordinate, weil Stubbenkamp 6 kein belegter Annahmeort ist, und keine Aussage über Betrieb, Annahme oder Fortbestand. Offen bleiben, ob Sylvia Schüttler heute Schrott oder Metall handelt (die Betreiberseite belegt ein anderes Gewerke), die Zuordnung der beiden Telefonnummern und der fehlende Register- oder Behördenbeleg; `status: pruefung` bleibt. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://www.xn--schttler-85a.com/impressum ; https://www.xn--schttler-85a.com/agb ; https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html ; https://nominatim.openstreetmap.org/search?q=Stubbenkamp+6%2C+45309+Essen&format=jsonv2&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/way/46961889/full.json ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 25.10.2026

- [Recherche 25.10.2026: 11880-Detailseite erneut direkt geprüft: „Sylvia Schüttler Metallhandel“, Stubbenkamp 6, 45309 Essen-Kray, 0178 4299833, `onvista@hotmail.com`; Branchen u. a. Kleintransport, Montage, Altmetallrecycling, Möbeltransport und Schrotthandel. Laut Seite stammen die Eintragsdaten vom 13.10.2025, Aktualisierung 04.10.2026. Das ist ein aktualisierter Aggregator-Lead, keine Betreiberbestätigung; keine Öffnungszeiten, aktuelle Annahmestelle oder konkrete Preise/Tarife. Ankauf, Verkauf und Gebühren bleiben ohne Preisangaben. E-Mail neu als unbestätigter Lead dokumentiert, nicht ins Frontmatter übernommen; GEO weiter erst nach Identitäts-/Standortklärung. Quelle: https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html]

### Recherche 11.10.2026

- [Recherche 11.10.2026: Bestehenden 11880-Kandidaten direkt revalidiert: Sylvia Schüttler Metallhandel, Stubbenkamp6/45309 Essen, 0178 4299833, keine Website/Zeiten; Daten vom 13.10.2025 trotz aktueller Anzeige 04.10.2026. Neuer Creditreform-Suchlead vorhanden, Detailabruf 403; Suchsnippet nicht als gelesene Primär-/Registerquelle oder zweite Bestätigung gezählt. Neue Suche identifizierte keinen aktuellen Betreiberauftritt. Keine Namenspräzisierung/Kontakt-/Annahme-Fills; pruefung bleibt. Generischer Preisrange-FAQ ist keine Ankaufspreisliste. Quelle(n): https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html ; https://firmeneintrag.creditreform.de/45309/5110488342/SYLVIA_SCHUETTLER_METALLHANDEL (403)]

### Recherche 01.10.2026

- [Recherche 01.10.2026: miss — nur ein Verzeichnis-Lead: 11880 nennt „Sylvia Schüttler Metallhandel, Stubbenkamp 6, 45309 Essen (Kray), 0178 4299833" (Schrotthandel/Kleintransport/Montage); kein zweiter unabhängiger Beleg, kein Betreiber-Impressum; Frontmatter bleibt leer, status pruefung; Quelle(n): nur Verzeichnis-Lead 11880.com, Stand 01.10.2026]

### Importiert (Seed-Stand 2026-09-30)

- mobil (GS)
- Adresse: Essen

### Recherche 03.10.2026

- [Recherche 03.10.2026: website_status leer → unbekannt; übrige Frontmatter unverändert. Die einzeln abgerufene 11880-Detailseite führt „Sylvia Schüttler Metallhandel“, Stubbenkamp 6, 45309 Essen-Kray, Mobilnummer und E-Mail sowie Altmetallrecycling/Schrotthandel; sie zeigt „Website hinzufügen“, keine Öffnungszeiten und keine zweite unabhängige Betreiber- oder Registerquelle. Der Eintrag ist laut 11880 am 27.06.2026 aktualisiert, die Eintragsdaten stammen vom 13.10.2025. Das reicht nicht für Fills; Name bleibt unpräzisiert „Schüttler“, Straße/Kontakt/Beschreibung bleiben leer, status bleibt pruefung. Quelle(n): https://www.11880.com/branchenbuch/essen-ruhr/190682936B113680307/sylvia-schuettler-metallhandel.html]
