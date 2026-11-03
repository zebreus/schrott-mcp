---
slug: nw-essen-schrott-busters
lat: 51.4660509
lon: 7.0729867
name: Schrott Busters
trader_type: schrotthaendler
state: NW
city: Essen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Neu, Ankauf unklar (nur Verzeichnis-/Profilbeleg)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Schrott Busters

## Überblick

Betreiberwebsite identifiziert sich als Schrott Busters Oliver Wojke, Fischweiher 26, 45307 Essen. Mobile Schrottabholung und Metallankauf für Privat- und Industriekunden werden ausdrücklich angeboten; keine öffentliche Hofannahme aus der Kontaktadresse ableiten. Die Website ist ein Einzelbeleg ohne unabhängige Betreiberbestätigung. Ankauf ohne Zahlenliste; Verkauf von Elektronikschrott erwähnt, ebenfalls ohne Liste. Kostenfreie Schrott-/Buntmetallabholung ist eine konkrete Selbstangabe, keine allgemeine Gebührenfreiheit für Demontage oder Entrümpelung. Status `pruefung`, Stammdaten vorerst unverändert.

## Timeline

### Owner-Korrektur 27.10.2026: falsches GEO zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`; der Reverse-Lookup führt auf die Fernbus-Spur „Freiheit“ am Essener Hauptbahnhof (OSM way 46961889, `vehicle=no`), also nicht auf einen Betriebsort. Die dokumentierte Geschäftsanschrift Fischweiher 26, 45307 Essen (Betreiber-Impressum mit Steuernummer 111/5329/3605 plus 11880-Eintrag vom 15.10.2026 mit identischer Telefonnummer) liegt rund 4,4 km entfernt und ist ein Wohn-/Mischobjekt; eine öffentliche Hofannahme ist nicht belegt, die Betreiberseite beschreibt mobile Abholung. Das gespeicherte Paar ist damit für genau diese Zeile falsch und wird mit `clear_fields: geo` zurückgenommen, ohne Ersatzkoordinate und ohne Annahmestellen-Behauptung; `status: pruefung` bleibt. Offen bleiben Register- oder Behördenbeleg, Abholgrenzen und Vergütung. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://schrott-busters.de/kontakt/impressum/ ; https://www.11880.com/branchenbuch/essen-ruhr/060440092B112411338/schrott-busters-oliver-wojke.html ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 14.10.2026 (Welle 5 A)

- [Recherche 14.10.2026: Startseite, Impressum und Ankauf-Unterseite direkt gelesen (HTTP 200). Impressum nennt Oliver Wojke, Fischweiher 26, D – 45307 Essen, kontakt@schrott-busters.de, 0177 6105096; das D ist Länderpräfix, nicht belegter Hausnummerzusatz. Die Marken-/Adresskongruenz zum Seed ist stark, aber Website-Unterseiten sind derselbe Quellenursprung; kein HRB im Impressum und keine zweite zulässige unabhängige Quelle verifiziert. Daher keine Frontmatter-Fills. Startseite nennt Metallankauf von Privat und Industrie, kostenfreie Schrott-/Buntmetallabholung und Ankauf/Verkauf von Elektronikschrott. Ankaufseite datiert 26.03.2024; keine numerischen Ankaufskurse, Verkaufswerte oder Gebühren für sonstige Leistungen auf den gelesenen Seiten. Genehmigungen nur pauschale Selbstwerbung, kein gelesenes Zertifikat. Offen: unabhängige Inhaber-/Gewerbebestätigung, aktuelle Abholgrenzen und Vergütung, keine Hofannahme unterstellen; Quelle(n): https://schrott-busters.de/ ; https://schrott-busters.de/kontakt/impressum/ ; https://schrott-busters.de/schrottankauf/]

### Importiert (Seed-Stand 2026-09-30)

- Fischweiher 26, nur schrottradar-Profil
- urspr. Website-Angabe: keine gefunden



### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **51.4660509/7.0729867** (Mittel der 4 eindeutigen
  Way-Knoten von way/100180825; der Punkt liegt innerhalb der Objekt-BBox **51.4659682/7.0729014 – 51.4661335/7.0730720**).
  Quelle 1 (OSM): way/100180825 trägt `addr:housenumber`=**26**, `addr:street`=**Fischweiher**, `addr:city`=Essen,
  `addr:postcode`=**45307**, `building`=residential — Dossier-Straße, -Hausnummer und -PLZ stimmen objektgenau
  überein; die Straße trägt live **40** verschiedene Hausnummern (1–39, 46, 48, 50, 52, 54). Das Objekt ist in OSM als
  **Wohnhaus** kartiert und trägt **weder** `name=` **noch** `operator=`; ein Objekt mit Betriebsbezug zu „Schrott
  Busters“ ist auf der Fischweiher nicht vorhanden — Aussage zum OSM-Bestand, **kein** Negativbeleg über den Betrieb.
  Quelle 2 (Betreiber/Register): das Impressum der Betreiber-Website nennt **Oliver Wojke, Fischweiher 26, D – 45307
  Essen**, **kontakt@schrott-busters.de** und **0177 6105096** mit Steuernummer **111/5329/3605**; der unabhängige
  11880-Eintrag (15.10.2026) führt denselben Betrieb mit derselben
  Telefonnummer und derselben Anschrift. **Abgrenzung:** die Betreiberseite beschreibt **mobile Abholung**; eine
  öffentliche Hofannahme an der Anschrift ist **nicht** belegt, die Koordinate bezeichnet daher die dokumentierte
  Geschäftsanschrift. Der zuvor im Produktionsbestand gespeicherte Punkt **51.450284375/7.015053125** (Straße „Freiheit“,
  45128 Essen) lag **4380 m** entfernt und war für diese Zeile falsch.
  **Hinweis zum Frontmatter:** der Clear-Token `geo` wurde aus `clear_fields` entfernt, weil die Zeile jetzt eine
  kuratierte Koordinate trägt — ein gleichzeitig gesetztes `clear_fields: 'geo'` wird vom Seed-Validator als
  Konflikt abgewiesen (`clear_fields conflicts with populated geo`).
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/100180825/full.json ; https://schrott-busters.de/kontakt/impressum/ ;
  https://www.11880.com/branchenbuch/essen-ruhr/060440092B112411338/schrott-busters-oliver-wojke.html (Abruf 03.11.2026).]
