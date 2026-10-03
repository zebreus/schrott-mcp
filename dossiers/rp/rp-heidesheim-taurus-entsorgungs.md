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

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 03.10.2026 (Feedback 4461, erneute Tiefenprüfung)

- [Recherche 03.10.2026: Google-DNS-Liveabfrage taurus-gmbh.de A ergibt Status 3 (NXDOMAIN): keine Website für Unterseiten-/Impressumcrawl verfügbar. Registerspiegel online-handelsregister nennt Taurus Entsorgungs-GmbH, HRB 45865 Mainz, Am Ockenheimer Graben 24, 55411 Bingen, gelöscht am 24.08.2022; North Data zeigt denselben Registerbezug und Erloschen-Markierung. Beide Spiegel beziehen sich auf Registerdaten, nicht als zwei unabhängige Originalregisterprüfungen zählen; kein amtlicher aktueller Auszug eingesehen. DNS-Ausfall beweist für sich keine Betriebsschließung. Nachfolge-/Heidesheimkontinuität nicht geklärt, E&O-Verdacht nicht zum Merge erhoben; Quelle(n): https://dns.google/resolve?name=taurus-gmbh.de&type=A | https://www.online-handelsregister.de/handelsregisterauszug/rp/Mainz/HRB/45865/Taurus-Entsorgungs-GmbH | https://www.northdata.de/Taurus+Entsorgungs-GmbH,+Bingen/Amtsgericht+Mainz+HRB+45865]
- [Recherche 03.10.2026: Prod-Abgleich bestätigt Feedbacks Feldwiderspruch: Website lokal leer, Prod https://taurus-gmbh.de bei website_status tot. Leere Website, tot und pruefung im Dossier unverändert; Vorschlag unbekannt nicht übernommen, da die frühere Domain live NXDOMAIN liefert. keep im Seedimport erhält eine nichtleere Prod-Website bei leerem Dossierfeld, Redeploy allein leert sie nicht. Keine Adressübernahme aus Bingen, kein Lösch-/Merge-Schluss und keine direkten DB-Writes; Quelle(n): read-only Prod-Abfrage slug rp-heidesheim-taurus-entsorgungs | crates/ingestion/src/seed_traders.rs | DNS-Beleg wie oben]

### Importiert (Seed-Stand 2026-09-30)

- Schrott, Altmetall-Ankauf zu Tageshöchstpreisen, Container

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2574 berechtigt — Domain taurus-gmbh.de NXDOMAIN (Feedback-DNS-Beleg) + HR-Spiegel: Taurus Entsorgungs-GmbH, HRB 45865 AG Mainz, Am Ockenheimer Graben 24, 55411 Bingen, Status gelöscht (Löschdatum 24.08.2022, zuvor Ingelheim → Bingen) → Firma erloschen, Website-Feld geleert, Status aktiv → pruefung; Nachfolgeverdacht: E&O Entsorgung GmbH (HRB 51294, gegr. 08.06.2022) sitzt an derselben Adresse (HR-Nachbarschaftsliste) — kein Merge, Dossiers bleiben getrennt; Klärfall: ob Taurus-Betrieb unter E&O fortbesteht; Quelle(n): Feedback-DNS-Check dns.google, online-handelsregister.de HRB 45865, HRB 51294 (E&O)]
