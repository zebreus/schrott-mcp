---
slug: sl-saarbrucken-malstatt-thomanek-schrott-und-metall
clear_fields: 'dropoff_json,geo'
name: Thomanek Schrott und Metall
trader_type: schrotthaendler
state: SL
city: Saarbrücken-Malstatt
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
provenance_section: Schrottankauf im Saarland (SL) — Recherche
provenance_ankauf_raw: ja (lt. Verzeichnis)
provenance_origin: table
---

# Thomanek Schrott und Metall

## Überblick

Ein Gelbe-Seiten-Eintrag führt „Thomanek Schrott und Metall“ in der Ludwigstr. 58, 66115 Saarbrücken-Malstatt, mit 0176 84739707 und den Leistungen Metallrecycling, Schrott, Containerservice und Kabelrecycling. Das bleibt ein einzelner Verzeichnis-Lead; Betreiber-, Register- oder kommunaler Zweitbeleg fehlt. Ein möglicher Bezug zu Grün Thomanek in Völklingen-Fürstenhausen ist nicht nachgewiesen. Status daher `pruefung`; Adresse, Telefon und Leistungen nicht als verifizierte Stammdaten behandelt. **Ankauf:** keine bestätigten Kurse; **Verkauf:** keine Preisliste; **Gebühren:** kein Tarif. Offen: Betreiberidentität, aktuelle Annahme und mögliche Verbindung zum Völklinger Betrieb; GEO erst danach.

## Timeline

### Owner-Abgleich 26.10.2026

- [Owner-Abgleich 26.10.2026: Gelbe Seiten führt weiterhin nur einen nicht inhaberbestätigten Eintrag „Thomanek Schrott und Metall“, Ludwigstr. 58, 0176 84739707, ohne Website oder Bewertungen. YellowMap führt dieselbe Mobilnummer zusätzlich für „A & W Recycling“ an Klausenerstr. 18 in Saarbrücken; das ist ein konkurrierender Verzeichnisdatensatz, kein Beleg für gemeinsame Inhaberschaft oder dafür, welcher Eintrag aktuell ist. Der getrennte Gelbe-Seiten-Eintrag „Grün Thomanek Metallrecycling UG“ nennt Saarwiesenweg 2 in Völklingen und eine andere Festnetznummer; eine Verbindung nach Malstatt ist nicht belegt. Daher kein Kontakt-/Adress-Fill und `status: pruefung` bleibt angemessen. Nominatim reverse-geocodiert den Altpunkt erneut auf Ufergasse/St. Johann; die Suche nach Ludwigstr. 58 findet nur einen Gebäude-/Wohnadresspunkt und einen anderen OSM-Ortseintrag, keine Thomanek-Annahme. `clear_fields: geo` bleibt richtig; kein Ersatzpunkt. Seed-Hinweis: leere Straße/PLZ überschreiben die alten Kernadresswerte, leeres Telefon oder leere Annahme-JSON löschen gespeicherte Enrichmentwerte dagegen nicht. Da keine Produktion abgefragt wurde, ist ein möglicher Alt-Telefon-/Annahmeclaim nicht verifiziert; nur bei einem konkret vorhandenen, falschen Altwert wäre ein eigener Clear zu prüfen. Quellen: https://www.gelbeseiten.de/gsbiz/c7fef7ea-8eed-46eb-a47a-ff182a9c98d1 ; https://www.yellowmap.de/Details/4q2hykSllPeZaUWb5ujCfQ==.aspx ; https://www.gelbeseiten.de/gsbiz/210cc1d9-2789-4d87-8a06-28fe99c2e2c2 ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.2354125&lon=6.9932063&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=Ludwigstra%C3%9Fe%2058%2C%2066115%20Saarbr%C3%BCcken%2C%20Germany&addressdetails=1&limit=5 ; `docs/trader-seed.md`.]

### Owner-Korrektur 26.10.2026

- [Owner-Korrektur 26.10.2026: Die im vorherigen Owner-Abgleich offene
  Produktionsfrage wurde read-only geklärt: `public.db` enthält für diesen
  Slug weiterhin `dropoff_json={"allowed":true}` und den inzwischen
  zurückgenommenen Aktivstatus. Der aktuelle Seed lässt das leere
  `dropoff_json` ohne Clear bestehen. Die Betreiber-/Register-/Kommunalprüfung
  findet aber nur nicht inhaberbestätigte Verzeichnis-Leads; weder der
  Malstatter Eintrag noch die widersprüchliche Nummernzuordnung zu A & W oder
  der getrennte Völklinger Rechtsträger belegt öffentliche Annahme in Malstatt.
  Deshalb ergänzt der Owner `clear_fields: 'dropoff_json,geo'`: der nächste
  reguläre Seed setzt die gespeicherte Zusage auf unbekannt `{}` und entfernt
  den falschen Punkt, nicht auf `allowed:false`; das behauptet weder
  Annahmeverweigerung noch Schließung. Status `pruefung` bleibt unabhängig
  davon bestehen. Keine Produktionsdaten geändert. Quellen und Grenzen:
  Read-only `public.db` (26.10.2026), die im Owner-Abgleich direkt geprüften
  Verzeichnis-Leads und [`docs/trader-seed.md`](../../docs/trader-seed.md).]

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 49.2354125, 6.9932063. Nominatim-Reverse löst den Punkt in der Ufergasse in Saarbrücken-St. Johann auf, nicht an der Ludwigstraße 58 in Malstatt. `clear_fields: 'geo'` entzieht den stadtteilfalschen Altpunkt beim nächsten regulären Seed. Betreiber- und Annahmestelle bleiben ungeklärt; keine Ersatzkoordinate. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.2354125&lon=6.99320625&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026: Statuskorrektur

- [Recherche 25.10.2026: Die direkt gelesene Gelbe-Seiten-Seite führt Thomanek Schrott und Metall, Ludwigstr. 58, 66115 Saarbrücken-Malstatt, Tel. 0176 84739707; als Leistungen nennt sie Metallrecycling, Schrott, Containerservice und Kabelrecycling. Das ist ein Einzelbeleg eines Aggregators und bestätigt weder Betreiber noch aktuelle Kundenannahme. Der zuvor auf `aktiv` gesetzte Status ist deshalb auf `pruefung` korrigiert; keine Schließung abgeleitet und keine Kontakt-/Adress-Fills vorgenommen. Mögliche Verbindung zu Grün Thomanek in Völklingen bleibt offen. ANKAUF: keine belegten Kurse; VERKAUF: keine Liste; GEBÜHREN: keine Tarife. GEO erst nach Betreiber-/Standortbestätigung. Quelle: https://www.gelbeseiten.de/gsbiz/c7fef7ea-8eed-46eb-a47a-ff182a9c98d1 .]

### Importiert (Seed-Stand 2026-09-30)

- Schrotthandel (evtl. verwandt mit Nr. 10)

### Recherche 02.10.2026

- [Recherche 02.10.2026: Klärfall ohne Fill (nur Timeline) — nur GelbeSeiten-Einzelbeleg (Ludwigstr. 58, 66115 Saarbrücken-Malstatt, Tel 0176 84739707); keine Betreiber-Website/HR-Zweitquelle; mögl. Bezug Grün Thomanek (Völklingen-Fürstenhausen) ungeklärt; kein Fill; Quelle(n): gelbeseiten.de (eine Quelle).]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall — Lead Saarbrücken-Malstatt, Ludwigstr. 58 nur Verzeichnisbeleg (Einzelbeleg, unsicher); keine Website gefunden; mögl. verwandt mit Grün Thomanek (Völklingen-Fürstenhausen, Saarwiesenweg 2); keine Frontmatter-Fills; Quelle(n): recherche/sl.md]
