---
slug: he-erlensee-ruckingen-auto-s3-autoverwertung
name: AUTO-S3 Autoverwertung
trader_type: autoverwertung
state: HE
city: Erlensee-Rückingen
street: ''
postcode: ''
phone: ''
clear_fields: 'phone'
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Einträge
provenance_ankauf_raw: AV / unklar
provenance_origin: table
---

# AUTO-S3 Autoverwertung

## Überblick

AUTO-S3 bleibt ein Verzeichnislead für Schubertstr. 2, Erlensee-Rückingen. Die aktuelle Gelbe-Seiten-Suche nennt dieselbe Nummer und einen Website-Hinweis, aber der Detailabruf ist blockiert; kein Impressum oder Betreiber-/Registerbeleg gelesen. Unqualifizierte Straße/Telefon aus Frontmatter zurückgenommen, Historie erhalten. Ankauf, Teileverkauf und Gebühren nicht primär bestätigt.

## Timeline

### Recherche 14.10.2026

- [Korrektur 14.10.2026: Owner zieht den historischen Telefonwert 061812998169 ausdrücklich mit clear_fields phone zurück: ausschließlich Verzeichnislead, keine qualifizierte aktuelle Betreiberzuordnung. Straße Schubertstr. 2 bleibt als ungeprüfter historischer Lead in der Timeline, nicht als kuratierte Annahmeadresse. Read-only Produktionsprüfung: Straße bislang Schubertstr. 2, GEO 50.163/8.9782. Ein geänderter Seed mit leerer Straße entfernt die Adresse, erhält aber diese ungeprüfte GEO; keine Standortvalidierung oder Koordinatenlöschung behauptet. Prüfstatus beibehalten, separater GEO-Klärfall vor einer späteren Standortfreigabe; kein direkter Produktionswrite. Quelle: Dossier-Historie 01.10.2026 und Recherche 14.10.2026; docs/trader-seed.md; read-only traders-Abfrage 14.10.2026.]

- [Korrektur 14.10.2026: Gelbe-Seiten-Suche live gelesen (200): AUTO-S3 Autoverwertung, Schubertstr. 2, 63526 Erlensee-Rückingen, 06181 2998169, Website-Hinweis. Verlinkter Detailabruf HTTP 403/Challenge; Website-Ziel und Impressum deshalb nicht verifiziert. DuckDuckGo ebenfalls Bot-Challenge. Straße und Telefon waren ausschließlich Verzeichnisangaben (Historie 01.10.); aus Frontmatter geleert, nicht durch ungeprüfte PLZ ergänzt. Leeres phone allein leert vorhandene DB-Anreicherung nicht: Owner muss eine ausdrückliche phone-clear_fields-Entscheidung prüfen; leere Straße wirkt nach docs/trader-seed.md anders, alte GEO werden dadurch nicht validiert. Kein DB-Zugriff/GEO-Eingriff. ANKAUF: Kfz-Ankauf aus Seed nicht primär bestätigt; VERKAUF: keine Teilepreisliste; GEBÜHREN: keine Tarife. Offen: Betreiber, Website-Ziel, aktuelle Annahmestelle. Quelle: https://www.gelbeseiten.de/suche/auto-s3/erlensee ; https://www.gelbeseiten.de/gsbiz/c2faf292-856c-42d3-ac39-ffae2062a7f8 (403); docs/trader-seed.md.]

### Importiert (Seed-Stand 2026-09-30)

- AV – Autoverwertung [Website-Recherche verzeichnis: services: Autoverwertung, Kfz-Ankauf; notes: Gelbe Seiten, 11880 und Seed-Adresse übereinstimmend (Rückingen).]

### Recherche 01.10.2026

- [Recherche 01.10.2026: miss — kein Betreiber-Webauftritt zu AUTO-S3 (Schubertstr. 2, Erlensee-Rückingen) verifiziert; Seed-Tel. 061812998169 und Strasse nur Verzeichnis-Leads ohne Zweitbeleg, unangetastet; Statusfeld unveraendert; Quelle(n): keine Betreiberquelle]
