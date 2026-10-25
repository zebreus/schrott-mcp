---
slug: he-pfungstadt-64319-stein-alexander-sebastian
clear_fields: 'geo'
name: Stein Alexander-Sebastian
trader_type: schrotthaendler
state: HE
city: Pfungstadt 64319
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
provenance_section: Nachtrag Audit-Runde 4b – Kandidaten-Sweep (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Stein Alexander-Sebastian

## Überblick

Das Dossier beruht auf Verzeichnisangaben für „Alexander-Sebastian Stein“ in Pfungstadt-Eschollbrücken (früher gemeldet: Tulpenweg 3, Telefon 06157 9110132). Es gibt keinen Betreiberauftritt, Register-/Impressumsbeleg oder sonstige aktuelle Primärquelle, die einen aktiven Schrottbetrieb bestätigt. Adresse und Telefonnummer wurden aus dem Frontmatter entfernt; sie bleiben als unbestätigte Leads in der Recherchehistorie.

**Preise:** keine Ankaufspreise, Verkaufspreise oder Entsorgungsgebühren veröffentlicht oder belastbar gefunden. Status `pruefung` bleibt; weder Betriebsaufgabe noch aktueller Ankauf ist belegt.

## Timeline

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 49.8056, 8.6031. Nominatim-Reverse löst den Punkt an der Rügnerstraße 67c auf, nicht am historischen Verzeichnislead Tulpenweg 3. Der Dossierstand belegt keinen Betreiberstandort. `clear_fields: 'geo'` entzieht das nicht passende Paar beim nächsten regulären Seed; keine Ersatzkoordinate gesetzt. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.8056&lon=8.6031&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026

- [Recherche 25.10.2026: Darmstadt Post führt in einem aktuellen Unternehmensverzeichniseintrag „Alexander-Sebastian Stein Schrotthandel“, Tulpenweg 3, 64319 Pfungstadt, Tel. 06157 9110132. Das ergänzt die älteren 11880-/Stadtbranchenbuch-Leads, ist aber ebenfalls ein Verzeichnis und keine Betreiber-/Registerbestätigung; die dort gezeigte Bewertung ist kein Nachweis laufender Annahme. Kein Frontmatter-Fill. ANKAUF: keine bestätigten Kurse; VERKAUF: keine Liste; GEBÜHREN: keine Angaben. GEO erst nach Identitäts- und Standortbestätigung. Quelle: https://darmstadt-post.de/unternehmen/pfungstadt/schrotthandel/alexander-sebastian-stein-schrotthandel-493/ (Verzeichnis, Abruf 25.10.2026).]

### Korrektur 17.10.2026 (Keep/Clear-Grenze)

- [Korrektur 17.10.2026: Frühere Feldrücknahmen bereits am 07.10. integriert, nicht als neue Wellenbereinigung zählen. Weiter nur Portal-Leads, keine Primärbestätigung/Schließungsbehauptung. street/postcode leer ersetzen bei geändertem Seed die gespeicherte Kernadresse; phone leer hält ohne clear_fields dagegen gespeicherte Anreicherung. Kein zusätzlicher Telefon-Clear autorisiert, daher keine DB-wirksame Telefonrücknahme behaupten. Servicefelder bleiben unbekannt/Keep, keine GEO-Freigabe; ohne Paar bleiben vorhandene Koordinaten erhalten, kein automatisches Geocoding. Quelle: research/checkin-20261017-1700-wave13-b-review.md; docs/trader-seed.md; Portal-Leads der alten Recherche]

### Korrektur/Recherche 07.10.2026

- [Korrektur 07.10.2026: Die bisherigen Werte Tulpenweg 3, 64319 und 06157 9110132 beruhten auf 11880 und Stadtbranchenbuch, also ausschließlich Branchenverzeichnisquellen; ein Betreiber-, Register- oder Kommunalbeleg zur aktiven Schrottfirma fehlt. Deshalb street/postcode/phone aus dem Frontmatter entfernt, Werte und damalige Doppelbeleg-Entscheidung bleiben in der Historie nachvollziehbar. NorthData-Suche nach „Stein Alexander-Sebastian Pfungstadt“ ergab keinen eindeutig passenden Rechtsträger; das beweist keine Nichtexistenz. ANKAUF: nicht bestätigt/keine Zahlenpreise; VERKAUF: keine Liste; GEBÜHREN: keine Angaben in den ausgewerteten Quellen. Offen: Identität, Adresse und aktueller Betrieb, ggf. Gewerberegister/Vor-Ort-Bestätigung; Quelle: https://www.northdata.de/?query=Stein%20Alexander-Sebastian%20Pfungstadt; https://www.11880.com/; https://pfungstadt.stadtbranchenbuch.com/]

### Importiert (Seed-Stand 2026-09-30)

- Kleinst-Schrotthändler
- Adresse: Pfungstadt 64319

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Gegenrecherche per Bing-Websuche (Direktabruf; Suchterme: Stein/Schrott/Pfungstadt): keine Betreiber-Website, kein Impressum, kein Handelsregister-/Northdata-Treffer, kein Betreiber-Social; Treffer nur namensgleiche Fremdfirmen und Orte; Vorbefund 'keine Website verifiziert' bestaetigt; kein Frontmatter-Fill (2-Beleg-Standard nicht erfuellbar); Klaerfall: kommunales Gewerberegister Pfungstadt oder Vor-Ort-Check; Quelle(n): Bing-SERP 01.10.2026, recherche/he.md]

### Recherche 01.10.2026 (Tiefenrecherche-Welle, Nachtrag)

- [Recherche 01.10.2026: Adress-/Telefon-Fund in ZWEI konsistenten Verzeichnissen — 11880 („Alexander-Sebastian Stein Schrotthandel“, Tulpenweg 3, 64319 Pfungstadt-Eschollbrücken, (06157) 9110132, Rubrik Schrotthandel/Altmetallrecycling) + stadtbranchenbuch Pfungstadt („Alexander-Sebastian Stein, Tulpenweg 3, 64319 Pfungstadt, 06157 9110132“): street/postcode/phone daher GEFÜLLT, aber Beleglage offen dokumentiert (beide Aggregatoren, KEIN Betreiber-Beleg, kein Register) — status bleibt pruefung. Keine Betreiber-Website, kein Impressum, kein Social. Namensvariante „Alexander Sebastian Stein“ (11880-Suche) vs. „Alexander-Sebastian Stein“ (11880-Profil) dokumentiert. Folgewelle: Gewerberegister Pfungstadt / Vor-Ort Eschollbrücken. Quelle(n): 11880.com, pfungstadt.stadtbranchenbuch.com.]

### Recherche 13.10.2026

- [Recherche 13.10.2026: Erneute Suche nach exaktem Namen, Pfungstadt und „Schrott“ ergab keine neue Betreiber-, Register- oder kommunale Quelle. Der frühere Tulpenweg-3-/64319-/06157-9110132-Treffer bleibt ein Verzeichnislead; die bereits dokumentierte Entfernung dieser Felder aus Frontmatter wird bestätigt. Negativbefund ist kein Nichtexistenz-/Schließungsnachweis. ANKAUF: nicht bestätigt/keine Zahlenpreise; VERKAUF/GEBÜHREN: keine Listen. Kein verifizierter Standort für GEO. Quelle/Suchweg: https://www.northdata.de/?query=Stein%20Alexander-Sebastian%20Pfungstadt; https://www.11880.com/; https://pfungstadt.stadtbranchenbuch.com/]
