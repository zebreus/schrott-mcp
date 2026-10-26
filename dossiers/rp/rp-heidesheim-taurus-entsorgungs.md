---
slug: rp-heidesheim-taurus-entsorgungs
name: Taurus Entsorgungs GmbH
trader_type: schrotthaendler
state: RP
city: Heidesheim
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: 'tot'
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Rheinland-Pfalz (RP) — Recherche
provenance_ankauf_raw: Ankauf Ja
provenance_origin: table
---

# Taurus Entsorgungs GmbH

## Überblick

Die Registerspiegel führen Taurus Entsorgungs-GmbH, HRB 45865 Mainz, als 2022 gelöscht; die frühere Domain taurus-gmbh.de löst aktuell nicht auf. Ein möglicher Fortführungshinweis ist jetzt besser eingrenzbar: Die Betreiberwebsite von E&O Entsorgung GmbH nennt HRB 51294, Dirk Sauer und den Binger Standort Am Ockenheimer Graben 24, 55411 Bingen. E&O bewirbt dort heute Schrott- und Metallannahme. Das belegt einen anderen aktuellen Rechtsträger am historischen Taurus-Registerstandort, aber keine Nachfolge, Übernahme oder Beziehung zum Heidesheim-Seed. Angaben und Preise von E&O werden deshalb nicht auf Taurus übertragen.

**Preise:** Für Taurus ist keine aktuelle Ankaufsliste belegt; keine Verkaufspreise oder Tarife. E&O nennt nur für seinen eigenen Betrieb Tageshöchstpreise und Annahmeschwellen, keine numerischen Materialpreise. `status: pruefung` bleibt; weder Bingen-Adresse noch E&O-Leistungen werden ins Frontmatter übernommen. Kein GEO-Pin ohne Identitäts- und Standortbeleg für den Seed.

## Timeline

### Recherche 26.10.2026: E&O-Annahmebedingungen abgegrenzt

- [Recherche 26.10.2026: E&O-Schrott-/Metallseite erneut direkt ausgewertet. Sie nennt für den eigenen Binger Betrieb Tageshöchstpreise, aber keine numerischen EUR-Kurse. Angegebene Schwellen: Eisenankauf ab 100 kg; sortierte NE-Metalle bis 10 kg kostenfreie Annahme, Ankauf ab 11 kg; Edelstahl/VA, E-Motoren und Bleibatterien bis 20 kg kostenfrei, Ankauf ab 21 kg; gemischter Elektroschrott bis 100 kg sowie E-Schrott-Bauteile bis 20 kg kostenfrei, Ankauf darüber (Bauteile wie Platinen/RAM/CPUs nach Einzelfall). Das sind Annahme-/Mindestmengenregeln und keine Euro-Preisliste. Diese Konditionen, Tageshöchstpreise und kostenfreie Kleinmengenannahme gehören ausschließlich zu E&O; sie werden nicht Taurus zugerechnet. Für Taurus bleiben ANKAUF ohne aktuelle Kurse, VERKAUF ohne Preisliste und GEBÜHREN ohne bestätigte Tarife. Quelle: https://eo-entsorgung.de/schrott-metalle/ (E&O-Betreiberquelle, nicht Taurus).]

### Recherche 25.10.2026: E&O am früheren Taurus-Registerstandort

- [Recherche 25.10.2026: Die Betreiberwebsite von E&O Entsorgung GmbH nennt im Impressum HRB 51294 AG Mainz, Vertretung Dirk Sauer und Am Ockenheimer Graben 24, 55411 Bingen. Die Unternehmensseite bezeichnet Bingen als Hauptstandort/Wertstoffhof; die eigene Schrott-und-Metalle-Seite bestätigt dort Annahme/Vergütung von Metallschrott und bietet Container an. Sie beschreibt E&O als aus E&O Recycling GmbH und HKV Elektroschrott Recycling GmbH hervorgegangen, nennt Taurus aber nicht. Damit ist ein aktueller, eigenständiger E&O-Betrieb am gleichen Ort wie die frühere Taurus-Registeranschrift belegt, nicht aber eine Nachfolge oder Betriebsfortführung aus Taurus. Keine Adresse/Leistung ins Taurus-Frontmatter übertragen und kein Merge. E&O veröffentlicht keine konkreten Euro-Kurse; die dort genannten Annahmeschwellen bzw. kostenlose Kleinmengenannahme sind E&O-Bedingungen, keine Taurus-Preise oder -Gebühren. Kein GEO für Taurus. Quellen: https://eo-entsorgung.de/impressum/ ; https://eo-entsorgung.de/unternehmen/ ; https://eo-entsorgung.de/schrott-metalle/ (direkt abgerufen 25.10.2026; Betreiberquellen von E&O, nicht Taurus).]

### Recherche 03.10.2026 (Feedback 4461, erneute Tiefenprüfung)

- [Recherche 03.10.2026: Google-DNS-Liveabfrage taurus-gmbh.de A ergibt Status 3 (NXDOMAIN): keine Website für Unterseiten-/Impressumcrawl verfügbar. Registerspiegel online-handelsregister nennt Taurus Entsorgungs-GmbH, HRB 45865 Mainz, Am Ockenheimer Graben 24, 55411 Bingen, gelöscht am 24.08.2022; North Data zeigt denselben Registerbezug und Erloschen-Markierung. Beide Spiegel beziehen sich auf Registerdaten, nicht als zwei unabhängige Originalregisterprüfungen zählen; kein amtlicher aktueller Auszug eingesehen. DNS-Ausfall beweist für sich keine Betriebsschließung. Nachfolge-/Heidesheimkontinuität nicht geklärt, E&O-Verdacht nicht zum Merge erhoben; Quelle(n): https://dns.google/resolve?name=taurus-gmbh.de&type=A | https://www.online-handelsregister.de/handelsregisterauszug/rp/Mainz/HRB/45865/Taurus-Entsorgungs-GmbH | https://www.northdata.de/Taurus+Entsorgungs-GmbH,+Bingen/Amtsgericht+Mainz+HRB+45865]
- [Recherche 03.10.2026: Prod-Abgleich bestätigt Feedbacks Feldwiderspruch: Website lokal leer, Prod https://taurus-gmbh.de bei website_status tot. Leere Website, tot und pruefung im Dossier unverändert; Vorschlag unbekannt nicht übernommen, da die frühere Domain live NXDOMAIN liefert. keep im Seedimport erhält eine nichtleere Prod-Website bei leerem Dossierfeld, Redeploy allein leert sie nicht. Keine Adressübernahme aus Bingen, kein Lösch-/Merge-Schluss und keine direkten DB-Writes; Quelle(n): read-only Prod-Abfrage slug rp-heidesheim-taurus-entsorgungs | crates/ingestion/src/seed_traders.rs | DNS-Beleg wie oben]

### Importiert (Seed-Stand 2026-09-30)

- Schrott, Altmetall-Ankauf zu Tageshöchstpreisen, Container

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2574 berechtigt — Domain taurus-gmbh.de NXDOMAIN (Feedback-DNS-Beleg) + HR-Spiegel: Taurus Entsorgungs-GmbH, HRB 45865 AG Mainz, Am Ockenheimer Graben 24, 55411 Bingen, Status gelöscht (Löschdatum 24.08.2022, zuvor Ingelheim → Bingen) → Firma erloschen, Website-Feld geleert, Status aktiv → pruefung; Nachfolgeverdacht: E&O Entsorgung GmbH (HRB 51294, gegr. 08.06.2022) sitzt an derselben Adresse (HR-Nachbarschaftsliste) — kein Merge, Dossiers bleiben getrennt; Klärfall: ob Taurus-Betrieb unter E&O fortbesteht; Quelle(n): Feedback-DNS-Check dns.google, online-handelsregister.de HRB 45865, HRB 51294 (E&O)]
