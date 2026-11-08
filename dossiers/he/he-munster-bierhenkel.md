---
slug: he-munster-bierhenkel
clear_fields: 'geo'
name: Bierhenkel
trader_type: schrotthaendler
state: HE
city: Münster
street: Kärcherstr. 22
postcode: '64839'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Bierhenkel

## Überblick

Kärcherstraße 22 gehört zur Münster/Hessen-Altheim-Spur, nicht Münster/Westfalen. Neu ist ein Creditreform-Suchauszug „Marvin Waldemar Bierhenkel Schrott, Textilien, Maschinen“. Die Zuordnung zum Seed bleibt mangels lesbarem Vollprofil und zweiter unabhängiger Belegquelle offen. Bestehende Straße ist Legacywert, keine bestätigte Publikumsannahme. Ankauf, Verkauf und Gebühren ohne verifizierte Zahlen; vor GEO vollständige Betreiberidentität und Betriebsadresse klären.

## Timeline

### Recherche 18.10.2026 (Welle 1202 West)

- [Recherche 18.10.2026: Creditreform-Kandidatenprofil Marvin Waldemar Bierhenkel Schrott, Textilien, Maschinen erneut direkt angefordert, HTTP 403 ohne lesbaren Firmeninhalt. Kein heutiger Vollprofil-/Adressnachweis und keine aktuelle unabhängige Gegenprüfung; historische Suchauszüge aus 15.10. nicht als heute neu gelesen ausgegeben. Google nur Consent, DDG Challenge. Kärcherstr. 22 bleibt Legacyspur für Münster/Hessen, keine verifizierte Publikumsannahme; keine Übertragung auf Münster/Westfalen. Ankauf: keine belegbare Zahlenliste; Verkauf: keine verifizierte Liste; Gebühren: kein zurechenbarer Tarif. Betreiber-/Gewerbeauskunft und Betriebsadresse vor GEO klären, Frontmatter unverändert; Quelle(n): https://firmeneintrag.creditreform.de/64839/6050303837/MARVIN_WALDEMAR_BIERHENKEL_SCHROTT_TEXTILIEN_MASCHINEN ; https://html.duckduckgo.com/html/?q=%22Bierhenkel%22%20Schrott%20M%C3%BCnster .]

### Recherche 15.10.2026 Welle18B

- [Recherche 15.10.2026: Eigene Namens-/Ortssuche findet erstmals Creditreform-ID 6050303837, Titel Marvin Waldemar Bierhenkel Schrott, Textilien, Maschinen / Münster, URL-PLZ 64839. Nur Suchauszug gelesen; Vollabruf HTTP 403, daher weder rechtliche Identität noch Adresse daraus als vollverifiziert behandeln. Das Örtliche/Schrottplatz-Leads nennen Kärcherstr. 22, 64839 Münster/Hessen-Altheim, 06071 33719; diese bleiben Aggregatoren, nicht Zweitbelege. Keine Frontmatter-Fills. ANKAUF/VERKAUF/GEBÜHREN: keine zuordenbare numerische Liste; GEO erst nach Primärabgleich der Betriebsstätte. Quellen: https://firmeneintrag.creditreform.de/64839/6050303837/MARVIN_WALDEMAR_BIERHENKEL_SCHROTT_TEXTILIEN_MASCHINEN (Suchauszug; direkt 403); https://www.dasoertliche.de/Themen/Bierhenkel-Schrotthandel-Schrott-und-Metalle-M%C3%BCnster-Hessen-Altheim-K%C3%A4rcherstr (Suchlead).]

### Importiert (Seed-Stand 2026-09-30)

- Schrotthändler (Verzeichnis)

### Recherche 08.11.2026

- [Recherche 08.11.2026: **Die Seed-Anschrift ist in OSM als Hausnummer belegt (Kärcherstraße 22, 64839 Münster OT Altheim) und `postcode` wurde auf 64839 gesetzt; zugleich zeigt der read-only Abgleich der Produktionszeile, dass dort ein völlig anderer Punkt gespeichert ist (Lippetal, Nordrhein-Westfalen), und das amtliche Fachbetrieberegister führt den Betrieb nicht.** Anschrift: das OSM-Adressobjekt way/326299783 trägt `addr:housenumber`=22, `addr:street`=Kärcherstraße, `addr:postcode`=64839, `addr:city`=Münster (Hessen), `addr:suburb`=Altheim und `building`=residential (5 Knoten, 4 eindeutig, Mittelwert 49.91513554/8.89044112); der unabhängige Geokoder Photon löst "Kärcherstraße 22, 64839 Münster" auf dasselbe Objekt auf, und das Verzeichnis Das Örtliche nennt dieselbe Anschrift mit der Ortsangabe Münster/Hessen-Altheim und der Telefonnummer 06071 33719. `postcode` wurde deshalb auf 64839 gesetzt, was den Ort gegen Münster in Westfalen abgrenzt; `street` bleibt unverändert Kärcherstr. 22, `phone` bleibt leer (nur Verzeichnislead) und `website` bleibt leer. **Produktionsbefund für den Owner:** der in der Produktionsdatenbank gespeicherte Punkt dieser Zeile (read-only Abgleich public.db am 08.11.2026) lautet 51.6992941/8.0388823 und löst per Photon-Rückwärtssuche auf "Dalmer Weg 40, 59510 Lippetal (Kreis Soest), Nordrhein-Westfalen" auf, also **nicht** auf die belegte Anschrift und **nicht** auf Münster/Hessen. Das Dossier führt selbst keine Koordinate, und hier wird **keine** gesetzt, weil die Betreiberidentität (Marvin Waldemar Bierhenkel) nur durch einen Creditreform-Suchauszug mit HTTP 403 und einen Verzeichnislead gedeckt ist, während die Anschrift selbst belegt ist; die **Owner-Entscheidung** ist: der falsch gesetzte Produktionspunkt wird mit `clear_fields: 'geo'` **zurückgenommen** ✓ — er gehört zu einem anderen Betrieb in einem anderen Bundesland ✓ ✓ — und es wird **keine** neue Koordinate gesetzt ✓ ✓, weil die Betreiberidentität nicht belegt ist ✓ ✓. **Zertifizierung:** das amtliche Fachbetrieberegister (clientseitig gerenderte Vaadin-App, Ergebnisgrid vollständig durchscrollt) wurde am 08.11.2026 live in beiden Routen mit dem Filter "Art des Betriebs = Egal" (is=1&e=1) abgefragt: die Namenssuche "Bierhenkel" liefert in beiden Routen 0 Treffer; die Standortabfrage um Kärcherstraße 22, 64839 Münster liefert mit 3 km in der Route "Entsorgungsfachbetriebe" 3 Anlagenzeilen (Knettenbrech und Gurdulic Umweltservice GmbH, Benzstraße 14, 64807 Dieburg, ZZHT00300057 / 7, 2,5 km; BB Transporte GmbH Unternehmenssitz, Lagerstraße 31, 64807 Dieburg, ZZFT00300288 / 1, 2,6 km) und in der Route "Altfahrzeugverwertung" 0 Zeilen, mit 10 km in der Route "Entsorgungsfachbetriebe" 21 Zeilen und in der Route "Altfahrzeugverwertung" 1 (Höreth Autoverwertung, Ostheimer Weg 13, 64832 Babenhausen, ZAGS00300011 / 1, 6,7 km). **Kein Treffer nennt Bierhenkel.** Reichweite: geprüft wurden die Namenssuche, die Standortradien 3 km und 10 km und beide Routen; die Efb- und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist **kein** Nachweis, dass ein Betrieb keine Anerkennung besitzt. Das Feld `certifications` bleibt leer, `status` bleibt `pruefung`. ANKAUF: keine belegbare Kursliste; VERKAUF: keine verifizierte Liste; GEBÜHREN: kein zurechenbarer Tarif.
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/326299783/full.json ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Bierhenkel ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Bierhenkel ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=K%C3%A4rcherstra%C3%9Fe%2022%2C%2064839%20M%C3%BCnster%2C%20Deutschland%3B8.890439%3B49.9151466%3B8.9322909%3B49.9420959%3B8.8485871%3B49.8881973&dst=3&is=1&e=1 ; https://photon.komoot.io/reverse?lat=51.6992941&lon=8.0388823 (Abruf 08.11.2026).]
