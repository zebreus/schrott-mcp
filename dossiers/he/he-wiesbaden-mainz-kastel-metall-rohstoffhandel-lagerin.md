---
slug: he-wiesbaden-mainz-kastel-metall-rohstoffhandel-lagerin
clear_fields: 'geo'
name: Metall- & Rohstoffhandel Lagerin
trader_type: sonstige
state: HE
city: Wiesbaden / Mainz-Kastel
street: Hambuschweg 29
postcode: '55252'
phone: 0176 80439557
email: info@mrl-lagerin.de
opening_hours: ''
website: https://mrl-lagerin.de/
website_status: aktiv
status: pruefung
description: 'Familienbetrieb in dritter Generation im Rhein-Main-Gebiet: Ankauf von Metallen und Rohstoffen zu Tageshöchstpreisen, Abholung mit Kleintransporter oder Absetzkipper, Demontage, Entkernung, Brenn- und Plasmaschneiden sowie kostenfreie Containerstellung (ausdrücklich nur für Metalle, nicht für Müll oder Bauschutt); keine Anlieferung und keine Autoverwertung.'
dropoff_json: ''
pickup_json: ''
provenance_section: Einträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Metall- & Rohstoffhandel Lagerin

## Überblick

Die Betreiber-Primärquelle von MRL Metall & Rohstoffhandel Lagerin ist direkt gelesen und bestätigt die Anschrift, die Kontaktdaten und die Leistungsangaben: Jeffrey Lagerin, MRL Metall & Rohstoffhandel Lagerin, Hambuschweg 29, 55252 Mainz-Kastel, Telefon 0176/80439557, info@mrl-lagerin.de. Die Betreiberseite nennt sich selbst „Entsorgungsfachbetrieb im Rhein-Main-Gebiet" und „zertifizierter Entsorgungsfachbetrieb"; ein amtlicher Nachweis (Zertifikat § 56 KrWG oder Eintrag im Fachbetriebsregister) wurde nicht gefunden, das Impressum nennt weder Rechtsform noch Registernummer. Der Status bleibt deshalb `pruefung`. Der Dossier-Ortsname „Wiesbaden / Mainz-Kastel" ist durch die Betreiberanschrift selbst entschieden: die Adresse liegt in Mainz-Kastel, nicht in Wiesbaden. Der in der Produktionszeile gespeicherte Koordinatenpunkt löste auf die Friedrich-Ebert-Allee in Wiesbaden auf und widersprach damit der belegten Anschrift; `clear_fields: 'geo'` zieht ihn zurück.

**ANKAUF:** „Wir kaufen Ihre Metalle von A-Z zu Tageshöchstpreisen an"; kein numerischer €/kg-Kurs veröffentlicht. **VERKAUF:** keine Metallverkaufspreisliste. **GEBÜHREN:** „Unsere Container können wir kostenfrei zur Verfügung stellen", ausdrücklich begrenzt auf Metalle, nicht Müll oder Bauschutt; kein weiterer Tarif. **Anlieferung:** ausgeschlossen, „Wir können nur Abholung, keine Anlieferung anbieten".

## Timeline

### Recherche 27.10.2026: Betreiberquelle bestätigt, Geo-Punkt zurückgezogen

- [Recherche 27.10.2026: Betreiber-Primärquelle direkt gelesen (Startseite, Impressum, Datenschutzerklärung; Abruf über die WordPress-JSON-Schnittstelle `wp-json/wp/v2/pages`, weil die Seiten selbst nur ein Div-Konstrukt liefern): Impressum „Angaben gemäß § 5 TMG: Jeffrey Lagerin, MRL Metall & Rohstoffhandel Lagerin, Hambuschweg 29, 55252 Mainz – Kastel; Telefon: 0176/80439557; E-Mail: info@mrl-lagerin.de". Startseite: „MRL-Metall & Rohstoffhandel Lagerin, Ihr Entsorgungsfachbetrieb im Rhein-Main-Gebiet", „MRL – Ihr zertifizierter Entsorgungsfachbetrieb Lagerin im Rhein-Main-Gebiet", „Als Familienbetrieb in dritter Generation". Die Seiten tragen als letzte Änderung 2021-02-20 (Impressum) und 2020-06-04 (Startseite), das Impressum ist aber aktuell abrufbar. Quellen: https://mrl-lagerin.de/ ; https://mrl-lagerin.de/impressum/ ; https://mrl-lagerin.de/wp-json/wp/v2/pages .]
- [Recherche 27.10.2026: Frontmatter gefüllt: postcode „55252" (Betreiber-Impressum; die exakte Nominatim-Adressabfrage löst Hambuschweg 29 nach Mainz-Kastel auf, Hausnummer und PLZ passen), phone „0176 80439557", email „info@mrl-lagerin.de", website „https://mrl-lagerin.de/" mit website_status „aktiv", description ergänzt. `street` bleibt „Hambuschweg 29" (unverändert, jetzt durch die Betreiberquelle belegt). `opening_hours` bleibt leer: die Betreiberseite veröffentlicht keine Öffnungszeiten. status bleibt `pruefung`, weil das Impressum keine Rechtsform oder Registernummer nennt und für den Claim „zertifizierter Entsorgungsfachbetrieb" kein amtlicher Nachweis vorliegt. Quellen: https://mrl-lagerin.de/impressum/ ; https://nominatim.openstreetmap.org/search?q=Hambuschweg%2029%2C%2055252%20Mainz-Kastel&format=jsonv2&addressdetails=1&limit=3 .]
- [Recherche 27.10.2026: GEO-Korrektur: Der gespeicherte Punkt 50.0778461538462, 8.24530769230769 löst per Nominatim-Reverse auf die Friedrich-Ebert-Allee im Wiesbadener Stadtteil Südost (65185) auf, während die belegte Betreiberanschrift in Mainz-Kastel (55252) liegt; der Punkt widerspricht damit der eigenen belegten Ortsangabe des Dossiers. `clear_fields: 'geo'` entzieht das Paar beim regulären Seed; das bedeutet nur „keine kuratierte aktuelle Koordinate", keine Schließung und keine Annahmeverweigerung. Eine Ersatzkoordinate wurde nicht geraten: die Nominatim-Abfrage löst nur die Straße „Hambuschweg" in Mainz-Kastel auf, nicht die Hausnummer 29. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.0778461538462&lon=8.24530769230769&zoom=18&addressdetails=1 ; read-only-Abgleich `public.db` (27.10.2026) ; `docs/trader-seed.md`.]
- [Recherche 27.10.2026: **ANKAUF:** „Ankauf von Rohstoffen: Wir kaufen Ihre Metalle von A-Z zu Tageshöchstpreisen an" und „Wir zahlen Tageshöchstpreise"; eine Preisliste mit €/kg-Angaben ist auf der Betreiberseite nicht vorhanden, der Seed-Hinweis „keine Preise" bleibt damit korrekt. **VERKAUF:** keine Metallverkaufspreisliste; die Seite nennt nur Ankauf, Demontage und Verwertung. **GEBÜHREN:** „Demontage: Unsere Container können wir kostenfrei zur Verfügung stellen" mit der ausdrücklichen Einschränkung „Unser kostenfreier Containerdienst gilt nur für Metalle, nicht für Müll oder Bauschutt"; das ist eine bedingte Kostenfreiheit für die Containerstellung, keine allgemeine kostenlose Entsorgungszusage. Keine Demontage-, Abhol- oder Verwertungstarife beziffert. **Anlieferung:** „Die Abholung erfolgt mit einem Kleintransporter (3,5t) sowie optional mit einem Absetzkipper mit passenden Mulden. Wir können nur Abholung, keine Anlieferung anbieten"; Autoverwertung wird ausdrücklich nicht angeboten. Quelle: https://mrl-lagerin.de/ .]
- [Recherche 27.10.2026: Beleglage offen: Ein amtliches Fachbetriebsregister für Hessen bzw. ein § 56 KrWG-Zertifikat für MRL wurde nicht gefunden; der Betreiber nennt den Status „zertifizierter Entsorgungsfachbetrieb" nur auf der eigenen Seite. Ein zweiter Beleg aus einer unabhängigen Evidenzklasse (Register, Behörde, Betreiber-Social) wurde nicht gefunden; der Gelbe-Seiten-Lead nennt dieselbe Anschrift, bleibt aber Verzeichnislead. Status bleibt deshalb `pruefung` mit Klärfall. Quellen: https://mrl-lagerin.de/ ; https://www.gelbeseiten.de/gsbiz/41716368-e307-4750-963a-408852500857 (Verzeichnislead, keine Evidenz).]

### Recherche 23.10.2026

- [Recherche 23.10.2026: Betreiberstartseite und Impressum erneut direkt gelesen: Jeffrey Lagerin/MRL, Hambuschweg 29, 55252 Mainz-Kastel, Telefon 0176/80439557 und info@mrl-lagerin.de. Leistungsangaben: Ankauf aller Metalle zu „Tageshöchstpreisen“, Abholung, ausdrücklich keine Anlieferung, Demontage/Entkernung, kostenfreie Containerstellung; die Website begrenzt den kostenfreien Containerdienst auf Metalle (nicht Müll/Bauschutt) und erklärt, keine Autoverwertung anzubieten. Impressum weist keine HRB-Nummer aus; daher trotz Betreiberquelle keine Skalare oder JSON-Konditionen ins Frontmatter übernommen. **ANKAUF:** keine Euro-/kg-Preisliste, nur Tagespreiswerbung. **VERKAUF:** keine Metallverkaufspreisliste. **GEBÜHREN:** kostenlose Containerstellung ist keine veröffentlichte kostenlose Entsorgungs-/Abholzusage; weitere Tarife fehlen. Quellen: https://mrl-lagerin.de/ ; https://mrl-lagerin.de/impressum/ .]

### Recherche 04.10.2026

- [Recherche 04.10.2026: Die Seiten des MRL-Auftritts (Startseite und Impressum) nennen Jeffrey Lagerin, Hambuschweg 29, 55252 Mainz-Kastel, Telefon 0176/80439557 und info@mrl-lagerin.de; die Startseite beschreibt Metallankauf/Abholung und ausdrücklich „keine Anlieferung“. Das Impressum enthält keine HRB-Angabe; einzige externe Adress-/Telefonspur ist Gelbe Seiten und zählt nur als Aggregator-Lead, keine zweite unabhängige Quelle. Daher Straße/PLZ/Kontakte/Website und JSON-Felder nicht gefüllt; eine Betreiberquelle allein reicht hier nicht; Quelle(n): http://mrl-lagerin.de/ ; http://mrl-lagerin.de/impressum ; https://www.gelbeseiten.de/gsbiz/41716368-e307-4750-963a-408852500857 (Lead)]

### Importiert (Seed-Stand 2026-09-30)

- Metall-/Rohstoffhandel
