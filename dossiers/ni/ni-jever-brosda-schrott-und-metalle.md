---
slug: ni-jever-brosda-schrott-und-metalle
name: Brosda Schrott und Metalle
trader_type: schrotthaendler
state: NI
city: Jever
street: ''
postcode: ''
phone: ''
clear_fields: phone
email: ''
opening_hours: ''
website: ''
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Oldenburg / Wilhelmshaven / Weser-Ems
provenance_ankauf_raw: ja
provenance_origin: table
---

# Brosda Schrott und Metalle

## Überblick

Offener Orts-/Identitätsklärfall: Die Betreiberwebsite von Thomas Brosda nennt Altmetallankauf in Wangerland/Wiefels, nicht Jever. Der bisherige Verzeichnisbeleg verbindet diese Orte nicht ausreichend. Keine Schließung oder Verlegung bewiesen; Kontaktdaten und Leistungen bleiben als Betreiber-Einzelbeleg in der Timeline.

## Timeline

### Seed-Wirkungsprüfung 16.10.2026

- [Korrektur 16.10.2026: Die am 14.10. begründete Rücknahme des Jever-Telefonfelds war im Seed unwirksam: leere Enrichmentfelder erhalten gespeicherte Werte. Echte seed_metadata-Probe auf privater Produktionskopie bestätigte, dass 04461 965331 ohne Direktive bestehen bleibt. Deshalb clear_fields: phone ausdrücklich gesetzt; beim nächsten regulären Seed wird nur das Telefon zurückgenommen, Historie und andere Felder bleiben erhalten. Keine neue Identitäts-/Umzugsentscheidung, keine Behauptung entfernter Koordinaten oder Annahmezusagen. Belegbasis bleibt die Quellenbewertung vom 14.10.; Wirkungsvertrag docs/trader-seed.md und research/checkin-20261016-0300-upgrade-seed-review.md. Keine Produktions-SQL-Änderung.]

### Recherche 14.10.2026

- [Korrektur 14.10.2026: Impressum erneut gelesen: Thomas Brosda/Brosda Altmetallhandel, Fuhlrieger Allee 1, 26434 Wangerland/Wiefels, 04461-965331, vsbrosda@aol.com. Der Telefonwert aus der Recherche 30.09.2026 ist zwar betreiberkongruent, aber Gelbe Seiten, Das Örtliche und 11880 sind nach README nur Leads, keine unabhängigen Belege. Deshalb strukturiertes Telefon geleert; der Wert bleibt hier und in der unveränderten Alttimeline erhalten. Jever-Zuordnung weiterhin nicht geklärt: status von aktiv auf pruefung gesetzt, ohne Slug-/City-Wechsel, Zusammenlegung, Umzugs- oder Schließungsbehauptung. Leere Adress-/Website-/Konditionsfelder bleiben leer; Quelle: http://www.brosda.com/impressum/ ; Abruf 14.10.2026]
- [Korrektur 14.10.2026: Die ältere Aussage „kein HRB“ beweist keine Registerfreiheit. Das Impressum behauptet einen Handelsregistereintrag mit „DT01-14 90027“, nennt aber kein Registergericht und keine konventionelle HRA-/HRB-Kennung. Diese Angabe ist nicht unabhängig geprüft und wird weder als bestätigter Registereintrag noch als Zertifikat interpretiert; Betreiber-Ausnahme daher nicht belegt. Gezielte Register-/Kommunalsuchen nach Thomas Brosda und Brosda/Fuhlrieger Allee in Wangerland/Friesland ergaben keinen verwertbaren Zweitbeleg. Fehlende Treffer beweisen keine Nichtexistenz; Quelle: http://www.brosda.com/impressum/ ; Abruf 14.10.2026]
- [Recherche 14.10.2026: Betreiber nennt Ankauf von Privat und Gewerbe, Barauszahlung gegen gültigen Personalausweis; Preise nur auf Anfrage. Genannt werden Alteisen, Schwerschrott, Aluminium/Alukabel, Blei, Elektromotoren, Kupfer/Kupferkabel, Messing, Rotguss, VA-Stahl, Zink, Zinn sowie Pb-Batterien/Akkumulatoren. Ausdrücklich keine Kühlgeräte und Bildschirme. LKW-/Kranmuldenabholung und Container sind Eigenangaben ohne numerischen Tarif oder pauschales Gratisversprechen. Autoteile-/Reifenverkauf ist kein Schrottankaufpreis; Bild-Cache-Zeitstempel sind kein datierter Fortbestandsbeleg. Keine Preise oder Bedingungen importiert; Quelle(n): http://www.brosda.com/wir-kaufen/ , http://www.brosda.com/leistungen/ , http://www.brosda.com/autoteile-zubeh%C3%B6r/ ; Abruf 14.10.2026]

### Importiert (Seed-Stand 2026-09-30)

- Schrott/Metalle

### Recherche 30.09.2026

- Telefon 04461 965331 doppelt belegt (Gelbe Seiten + Das Örtliche).
- Verzeichnis-Ort Wangerland-Wiefels (Fuhlrieger Allee 1) vs Dossier-City Jever → Straße/PLZ nicht übernommen (Stadt-Mismatch).
- Quelle: https://www.gelbeseiten.de/suche/Schrott/Jever
- Quelle: https://www.dasoertliche.de/?form_name=search_nat&kw=Brosda&ci=Jever

### Recherche 02.10.2026

- [Recherche 02.10.2026: Betreiber re-verifiziert (Einzelunternehmen Thomas Brosda, kein HRB, keine Owner-Ausnahme): Brosda Altmetallhandel, Fuhlrieger Allee 1, 26434 Wangerland/Wiefels, Tel. 04461-965331, Fax 04461-965332, vsbrosda@aol.com; Ankauf Alteisen/Schwerschrott/Alu/Blei/Elektromotoren/Kupfer/Messing/Rotguss/VA-Stahl/Zink/Zinn/Batterien, Abholung per LKW. 11880 Wangerland-Treffer kongruent (Fuhlrieger Allee 1, 04461 965331) als Zweitbeleg für Betreiber-Ort. City-Mismatch Dossier-City Jever vs Betreiber-Ort Wangerland bleibt → street/postcode/email nicht übernommen (City-Mismatch-Regel), Telefon 04461 965331 bleibt (betreiber-kongruent, doppelt belegt). Klärfall Folgewelle: Slug-Split Jever/Wangerland oder Umzug prüfen. Quelle(n): http://www.brosda.com/, http://www.brosda.com/impressum/, http://www.brosda.com/wir-kaufen/, https://www.11880.com/suche/schrott/wangerland.]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Website lebt (Einzelunternehmen Thomas Brosda, kein HRB, keine Owner-Ausnahme); City-Mismatch Jever vs Wangerland bleibt; Quelle(n): http://www.brosda.com/, http://www.brosda.com/impressum/, http://www.brosda.com/wir-kaufen/]
- Betreiber-Fakten (Einzelbeleg, NICHT in Frontmatter): Brosda Altmetallhandel, Inhaber Thomas Brosda, Fuhlrieger Allee 1, 26434 Wangerland/Wiefels, Tel. 04461-965331, Fax 04461-965332, vsbrosda@aol.com; Ankauf Alteisen, Schwerschrott, Alu, Blei, Elektromotoren, Kupfer, Messing, Rotguss, VA-Stahl, Zink, Zinn, Batterien; Abholung per LKW.
- Dossier-City Jever vs Betreiber-Ort Wangerland: Straße/PLZ nicht übernommen (City-Mismatch-Regel); Telefon 04461 965331 war bereits gesetzt und ist betreiber-kongruent.
- Klärfall Folgewelle: Dossier-City Jever vs Wangerland klären (Slug-Split oder Umzug prüfen).
