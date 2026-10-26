---
slug: bw-stuttgart-mohringen-dinkel
name: Dinkel
trader_type: sonstige
state: BW
city: Stuttgart-Möhringen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
clear_fields: 'description,dropoff_json,email,geo,opening_hours,phone,website,website_status'
dropoff_json: ''
pickup_json: ''
provenance_section: Neueinträge ohne Website (Register/Verzeichnis, Handeln-Flag)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Dinkel

## Überblick

Der aktuelle Betrieb **Dinkel Recycling GmbH** (HRB 10644) und sein Standort Epplestraße 182, Stuttgart-Degerloch, sind durch Betreiber- und Registerquellen belegt. Die Verbindung dieses Betriebs mit der separaten Seed-Zeile „Dinkel“ in Stuttgart-Möhringen beruht dagegen nur auf einem Branchenverzeichnis-Lead und ist nicht ausreichend bestätigt. Die Dossierzeile bleibt daher auf dem importierten Namen/Ort und `status: pruefung`; Anschrift, Betreiberkontakt, Website, Aktivstatus und Annahmebedingungen der GmbH werden nicht auf die Altzeile übertragen.

**Nicht übertragene Kandidateninformationen:** Die Dinkel-Recycling-Website beschreibt Schrott-/Metallverwertung, Selbstanlieferung und Containerleistungen am Degerlocher Standort; weder diese Leistungen noch dortige Preise sind für die separate Möhringer Seed-Zeile bestätigt. Für den vorliegenden Datensatz werden deshalb keine Annahme-, Ankauf-, Verkaufs- oder Gebührenangaben behauptet. Der alte Produktionspunkt ist als Fehlzuordnung erkannt und wird explizit zurückgenommen; es wird kein Standort geokodiert.

## Timeline

### Owner-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Der Read-only-Produktionsabgleich zeigt für diesen Slug weiterhin die Altzeile `Dinkel` / Stuttgart-Möhringen / `pruefung`, jedoch den Kartenpunkt 48.7258/9.1464 an Hechinger Straße 5. Betreiberwebsite und NorthData HRB 10644 belegen Dinkel Recycling GmbH und deren Standort Epplestraße 182 in Degerloch, aber nicht die Identität dieser Möhringer Seed-Zeile; der einzige Zuordnungsweg ist ein Verzeichnis-Lead. Deshalb werden die zuvor auf die Altzeile übertragenen GmbH-Stammdaten, Aktivstatus und Annahmebehauptung zurückgenommen. `clear_fields: 'description,dropoff_json,email,geo,opening_hours,phone,website,website_status'` löscht auch den gespeicherten `allowed:true`-Claim und den Punkt; keine Ersatzkoordinate. Der ursprüngliche Name/Ort und `pruefung` bleiben erhalten. Quellen: https://dinkel-gmbh.de/stuttgart-degerloch.html ; https://www.northdata.de/Dinkel%20Recycling%20GmbH,%20Stuttgart/HRB%2010644 ; https://stuttgart.city-map.de/en/dinkel-gmbh-entsorgungsfachbetrieb (nur Zuordnungslead) ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=48.7258&lon=9.1464&zoom=18&addressdetails=1 ; README.md, Quellenhierarchie und Betreiberkettenregel.]

### Owner-GEO-Prüfung 21.10.2026

- [Owner-GEO-Prüfung 21.10.2026: Die Betreiberseite bestätigt Selbstanlieferung von Schrott und Metall an Epplestraße 182, 70567 Stuttgart. Der vorhandene Produktionspunkt löst dagegen auf Hechinger Straße 5 auf. Epplestraße 182 bleibt ein belastbarer Standortkandidat, aber der genaue Eingang ist nicht punktgenau belegt; kein Ersatz-GEO ergänzt. Quellen: [Betreiberseite](https://dinkel-gmbh.de/stuttgart-degerloch.html), [Reverse-Abfrage](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=48.7258&lon=9.1464&zoom=18&addressdetails=1), [OSM-Objekt am Produktionspunkt](https://www.openstreetmap.org/node/3127571980), [Owner-Bericht](../../research/wave-20261021-0900-geo-bw.md).]

### Recherche 21.10.2026

- [Recherche 21.10.2026: Den seednahen Eintrag „Dinkel“ mit der aktuellen Dinkel Recycling GmbH abgeglichen. Die Betreiberseiten nennen Epplestraße 182, 70567 Stuttgart-Degerloch, Telefon 0711 453218, info@dinkel-gmbh.de, Anlieferung Mo-Fr 07:00-16:30 und Sa 08:00-12:00 sowie Selbstanlieferung von Schrott/Metall durch Privat- und Gewerbekunden. NorthData führt Amtsgericht Stuttgart HRB 10644, Adresse Epplestr. 182 und die Registerpublikation vom 26.03.2026 zur Umbenennung aus Dinkel GmbH Müll- und Schuttabfuhr; damit sind Firmenname, Betreiberadresse und aktuelle Existenz unabhängig vom Betreiberauftritt gestützt. City-map ordnet den Dinkel-Eintrag dem Suchbereich Möhringen zu, bleibt aber ein Verzeichnislead; für die strukturierte Ortsangabe folgt die konkrete Betreiberbezeichnung Stuttgart-Degerloch. Ankauf: keine Auszahlung/Kurse auf den geprüften Seiten belegt; Verkauf: keine Metallverkaufspreise; Gebühren: Container-Shop benötigt eine PLZ und verweist bei nicht abgedeckten Gebieten auf individuelles Angebot; AGB verweisen auf vereinbarte Preise/Preisliste, ohne dort Zahlen zu veröffentlichen. Quellen: https://dinkel-gmbh.de/impressum.html ; https://dinkel-gmbh.de/kontakt.html ; https://dinkel-gmbh.de/stuttgart-degerloch.html ; https://shop.dinkel-gmbh.de ; https://dinkel-gmbh.de/dl/2022-DINKEL-AGB.pdf ; https://www.northdata.de/Dinkel%20Recycling%20GmbH,%20Stuttgart/HRB%2010644 ; Zuordnungslead: https://stuttgart.city-map.de/en/dinkel-gmbh-entsorgungsfachbetrieb .]

### Importiert (Seed-Stand 2026-09-30)

- Tätigkeit unklar
- Adresse: Stuttgart-Möhringen
