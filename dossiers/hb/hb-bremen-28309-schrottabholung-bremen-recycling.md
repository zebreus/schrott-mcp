---
slug: hb-bremen-28309-schrottabholung-bremen-recycling
name: Schrottabholung Bremen Recycling
trader_type: schrotthaendler
state: HB
city: Bremen
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
clear_fields: 'phone'
provenance_section: (1) Behaltene Kandidaten
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Schrottabholung Bremen Recycling

## Überblick

Historischer Verzeichnis-Kandidat mit ungeklärtem Betreiber; mobile Tätigkeit, öffentliche Annahmestelle, Ankauf und kostenlose Abholung nicht verifiziert. Die am 18.10.2026 gelesene 11880-Detailseite nennt Auf den Roden 2, 28307 Bremen und 0176 22544746, bleibt aber ein Aggregator-Lead. 28309 bei Cylex ist eine historische Dossierangabe; heutige Abrufe liefern 403, der PLZ-Konflikt ist nicht postalisch aufgelöst. Straße und PLZ im Dossier zurückgenommen, ohne Ersatzadresse oder Nichtexistenz zu behaupten; produktive Anwendung erst nach geprüftem Rebuild/Rollout. Andere Bremer Betreiberquellen belegen keine Identitätskette zum Kandidaten. Status `pruefung`, kein Kontakt-/Service-Fill; keine belastbar zuordenbare Preis-/Tarifquelle verifiziert.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „n.e.“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 05.10.2026

- [Recherche 05.10.2026: Cylex-Suchtreffer „Schrottabholung Bremen Kostenlos Recycling“ nennt Auf den Roden 2, 28309 und 0176 22544746; die 11880-Suche Altmetallrecycling/Bremen führt dieselbe Adresse und Nummer unter 28307 Bremen (Arbergen). Beides sind Aggregator-Leads, keine unabhängigen zulässigen Betriebsbelege; Cylex-Detailzugriff blockiert. Postcode-Konflikt deshalb nicht durch Mehrheitsentscheidung aufgelöst und Nummer nicht gefüllt. `city` auf Bremen normalisiert (vorher doppelte PLZ im Ortsfeld), ohne neue Adressbestätigung; bestehendes `postcode: 28309` bleibt ungeklärt. Quellen: https://web2.cylex.de/suche/schrotth%C3%A4ndler/Bremen; https://www.11880.com/suche/altmetallrecycling/bremen.]
- [Abgrenzung 05.10.2026: https://bremerschrotthandel.com/impressum/ ist erreichbar und nennt Kadir Sayli, Syker Straße 6, 28307 Bremen, +49 1522 6349848. https://www.bremer-recycling-service.de/ ist eine JavaScript-Seite; der ausgelieferte Anwendungs-Code https://www.bremer-recycling-service.de/static/js/main.3eadf2b3.js nennt Bremer Recycling Service, Adil Altun, c/o HQ Bremen Airport, Flughafenallee 26, 28199 Bremen, +49 1573 7775391. Keine der beiden Betreiberquellen verknüpft sich mit Auf den Roden 2 oder der Leadnummer 0176 22544746. Deren Ankauf-/Abholwerbung, Preise oder Kontaktdaten daher nicht diesem Dossier zugeordnet. Kein belastbarer numerischer Ankaufspreis, Verkaufspreis oder Gebührentarif für diesen Kandidaten gefunden; weder Schließung noch Umzug daraus ableitbar.]

### Importiert (Seed-Stand 2026-09-30)

- Stahl/Alu/Kupfer/Edelstahl/Blei/Messing, mobil
- Adresse: Bremen 28309, Auf den Roden 2
- Adressbeleg: Seed-notes (Profilbeleg); PLZ Orts-PLZ

### Korrektur 18.10.2026

- [Korrektur 18.10.2026: Street und PLZ im Dossier zurückgenommen: keine zulässige Betreiber-/Adressverknüpfung für den historischen Kandidaten verifiziert. Heute gelesene 11880-Suche und Detailprofil nennen Auf den Roden 2, 28307 Bremen, 0176 22544746; nur Aggregator-Lead ohne benannten Betreiber/Website. Portal-Aktualisierungsdatum kein Betreiber-Lebenszeichen, Cylex-Verweis kein unabhängiger Zweitbeleg. 28309 bei Cylex bleibt historische Dossierangabe; Suche/Detail heute 403, PLZ-Konflikt nicht amtlich aufgelöst. Gelesenes Impressum bremerschrotthandel.com nennt Kadir Sayli, Syker Straße 6 und eigene Telefonnummer, daneben abweichender Footer-tel-Link; keine pauschal konsistente Kontaktlage behauptet. HTML-Strukturdaten und ausgeliefertes JS-Bundle von bremer-recycling-service.de nennen Adil Altun, Flughafenallee 26; keine interaktive Impressumsansicht gerendert. In diesen Quellen keine Verknüpfung zur Leadadresse/-nummer gefunden: keine Identität, Übernahme, Umzug oder definitiv getrennte Betreiber daraus bewiesen. Weitere Suche teilweise zugriffsbeschränkt, kein erschöpfender Negativbefund. Status pruefung, keine Kontakt-/Service-Fills oder GEO. Keine dem Kandidaten belastbar zuordenbare Ankauf-/Verkaufs-/Gebührenquelle verifiziert, kostenlose Abholung unbestätigt. Fehlende Koordinaten behalten gespeichertes GEO; Adressrücknahme bereinigt es nicht. Der uncommittete 12.10.-Entwurf belegt keinen damaligen Abruf; Anwendung erst nach späterem geprüftem Rollout. Quellen: https://www.11880.com/branchenbuch/bremen/060440092B113127592/schrottabholung-bremen-kostenlos-recycling.html ; https://www.11880.com/suche/altmetallrecycling/bremen ; https://bremerschrotthandel.com/impressum/ ; https://www.bremer-recycling-service.de/ ; https://www.bremer-recycling-service.de/static/js/main.3eadf2b3.js ; historischer Lead, heute 403: https://web2.cylex.de/firma-home/schrottabholung-bremen-kostenlos-recycling-16433002.html]
