---
slug: hb-syke-siegmayer-l
name: Siegmayer L.
trader_type: schrotthaendler
state: NI
city: Syke
street: Sudweyher Str. 34
postcode: '28857'
phone: 04242 931462
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Verzeichnis-Lead: Schrotthandel Siegmayer L., Sudweyher Str. 34, 28857 Syke, Tel. 04242 931462. Kein Betreiberauftritt und kein Registerbeleg; die Hausnummer 34 ist in OSM als Wohngebäude ohne Gewerbe-Tag erfasst, und das offizielle Branchenbuch der Stadt Syke führt an der Sudweyher Straße keinen Schrotthandel. ANKAUF, VERKAUF und GEBÜHREN sind nicht belegt. Status pruefung, keine Schließung behauptet.'
dropoff_json: ''
pickup_json: ''
provenance_section: Neue Funde (nicht in den Tabellen oben / Nachtrag Runde 2)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Siegmayer L.

## Überblick

Die vorhandene Adresse Sudweyher Str. 34, 28857 Syke, und Telefon 04242 931462 stammen bislang nur aus Schrottplatz-/Branchenverzeichnissen. Weitere Verzeichnisse spiegeln diese Daten, sind aber keine unabhängigen Betreiberbelege. Ein Öffnungszeitenbuch führt außerdem einen „Ludwig Siegmayer“ an Zum Walde 7; ob das derselbe Mensch/Betrieb ist, ist offen und wird nicht zusammengeführt. Ein Quercheck zum HE-Dossier `he-ober-ramstadt-64372-siegmayer-markus-schrott` ergab einen anderen Telefonanschluss und ebenfalls widersprüchliche Verzeichnisadressen; gleicher Nachname allein ist kein Merge-Beleg. Die amtliche Stadtzuordnung ist Syke, Niedersachsen; deshalb ist `state` auf NI korrigiert, der unveränderliche Slug/Pfad `hb-...` bleibt wie vorgefunden.

**Preise:** keine Betreiberpreisliste und keine belastbaren Ankaufpreise, Verkaufspreise oder Gebühren. Aktiver Betrieb und Zusammenhang der Namens-/Adressvarianten bleiben ungeklärt.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrott (klein)
- urspr. Website-Angabe: keine (nur schrottplatz-info-Beleg)
- Adresse: Syke (Fern-Umland), Sudweyher Str. 34, 28857, Tel. 04242 931462
- Adressbeleg: Seed-notes (schrottplatz-info)

### Recherche 03.10.2026

- [Recherche 03.10.2026: Offizielle Stadt-Syke-Angaben ordnen Syke dem Bundesland Niedersachsen, Landkreis Diepholz, zu; das widerspricht dem bestehenden Pfad/Slug hb- und state HB. State/Pfad/Slug nicht geändert, weil Slugs unveränderlich sind und eine Umordnung in einen anderen State außerhalb dieses Shards liegt — Owner-Klärfall. Die Händleradresse/Telefonnummer bleiben nur Seed-/Schrottplatz-Info-Leads, kein unabhängiger Betreiber-/Registerbeleg; website_status leer→unbekannt, status pruefung bleibt, keine Schließung behauptet.; Quelle(n): https://www.syke.de/leben/ueber-syke/, https://www.syke.de/portal/startseite.html]

### Recherche 06.10.2026

- [Korrektur 06.10.2026: geografische Zuordnung von Syke erneut anhand der offiziellen Stadtseite geprüft; `state` von HB auf NI gesetzt und Dossier unter `dossiers/ni/` abgelegt. Der unveränderliche Slug bleibt `hb-syke-siegmayer-l`. Die Verzeichnisdaten zu Sudweyher Str. 34/04242 931462 wurden in einer frischen Suche erneut gefunden, aber nur auf Aggregatoren; eine andere Verzeichnisangabe zu Ludwig Siegmayer, Zum Walde 7, beweist keine Identität und wird nicht zusammengeführt. Quercheck mit dem separaten HE-Siegmayer-Dossier: anderer Telefonanschluss, kein Beleg für dieselbe Person/Firma; kein Merge. Kein Betreiberbeleg und keine Preise für Ankauf, Verkauf oder Gebühren gefunden. Quelle(n): https://www.syke.de/leben/ueber-syke/ ; https://www.schrottplatz-info.de/schrottplatz/Syke/Siegmayer-L- ; https://www.schrottplatz.org/syke/schrott-siegmayer-l-aYEPvj.html ; https://www.oeffnungszeitenbuch.de/filiale/Syke-Ludwig%20Siegmayer-2720779D.html (Verzeichnisleads); Dossiervergleich: `dossiers/he/he-ober-ramstadt-64372-siegmayer-markus-schrott.md`]

### Recherche 30.10.2026

- [Recherche 30.10.2026: Amtliches Fachbetrieberegister (Spiegel schrottregister.pages.dev, Registerabruf 11.08.2026): für Syke existiert **keine Ortsseite** (HTTP 404, Seite nicht gefunden). Das ist **kein Negativbefund**, weil das Verzeichnis nur Orte mit Registereinträgen führt und die Efb-Anerkennung freiwillig ist. Quelle(n): https://schrottregister.pages.dev/ort-syke (Abruf 30.10.2026)]
- [Recherche 30.10.2026: OSM-Adressprüfung live am Objekt. Raster bbox=8.8236414,52.9390717,8.8529780,52.9608815 über den gesamten Verlauf der Straße "Sudweyher Straße" in Syke (Ortsteil Barrien), Filter addr:street=Sudweyher Straße: 45 Objekte | 43 verschiedene Hausnummern. Die Seed-Hausnummer 34 ist erfasst: way/260924504 mit addr:housenumber=34, addr:street=Sudweyher Straße, addr:postcode=28857, addr:city=Syke, addr:country=DE, addr:suburb=Barrien, **building=residential**. Ein name-, operator-, shop- oder industrial-Tag ist nicht gesetzt; präzise: die Adresse existiert in OSM als Wohnhaus, ein Schrottbetrieb ist dort nicht kartiert. Hinweis zur Zählung: zwei Rohwerte der Karte sind zusammengesetzte Notationen ("12,12a" und "2,2b") und wurden vom Scanwerkzeug als je ein Eintrag gezählt. Quelle(n): https://api.openstreetmap.org/api/0.6/way/260924504/full.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=8.8236414,52.9390717,8.8529780,52.9608815 (Abruf 30.10.2026)]
- [Recherche 30.10.2026: Amtliches Branchenbuch der Stadt Syke gegengeprüft. Das von der Stadt veröffentlichte Branchenbuch führt für die Sudweyher Straße die Einträge "EDEKA Stellmann" (7), "Falken-Apotheke Barrien" (11), "Nienaber Elektrotechnik GmbH" (12), "Charisma" (22), "Meyer Raumausstattung" (26), "Finance Consult Maklerpool GmbH & Co. KG" (37), "Avanti Malerfachbetrieb" (31) und "Elsner Klaus, Sanitär-, Heizungs- und Gasinstallationen" (48). Für die Hausnummer **34 ist kein Gewerbeeintrag** der Stadt Syke hinterlegt und **kein Schrotthandel** an dieser Straße genannt; die zweite Verzeichnisangabe "Ludwig Siegmayer, Zum Walde 7" aus der Vorwelle bleibt weiterhin ohne Zuordnung. Quelle(n): https://www.syke.de/regional/branchenbuch/gesamt/uebersicht.html?brotID=0&gewerblich=1&id=0&naviID=0&no_kreisweit=0&showall=1&vereine=0 (Abruf 30.10.2026)]
- [Recherche 30.10.2026: Preise getrennt nach Ankauf, Verkauf und Gebühren. ANKAUF: die Schrottkategorie stammt aus dem Verzeichnislead; ein Sortenangebot und **Zahlenpreise** sind nicht belegt. VERKAUF: kein Beleg. GEBÜHREN: keine Tarifordnung und keine Gebührenangabe. Der Status `pruefung` bleibt unverändert; eine Schließung wird nicht behauptet. Quelle(n): https://www.syke.de/regional/branchenbuch/gesamt/uebersicht.html (Abruf 30.10.2026)]
