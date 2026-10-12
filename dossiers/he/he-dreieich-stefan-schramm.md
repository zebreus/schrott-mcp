---
slug: he-dreieich-stefan-schramm
name: Stefan Schramm
trader_type: schrotthaendler
state: HE
city: Dreieich
street: ''
postcode: ''
phone: ''
clear_fields: 'phone'
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Stefan Schramm

## Überblick

Der vorhandene Eintrag zu Herrnroth 11, 63303 Dreieich und 06103 322820 stammt aus tellows/Cylex-Verzeichnissen; ein Betreiber-, Register- oder kommunaler Gewerbenachweis ist weiterhin nicht belegt. Die 2026er Verzeichnisspur allein rechtfertigt weder Kontakt-Fills noch `aktiv`; diese Angaben wurden zurückgenommen und bleiben als Leads in der Timeline.

**Preise:** Ankauf — keine bestätigte Vergütung/Preisliste; Verkauf — keine Liste; Gebühren — keine Tarife belegt.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrotthändler (Verzeichnis)

### Recherche 01.10.2026

- [Recherche 01.10.2026: Kein belastbarer Nachweis — Northdata-Suche nach Stefan Schramm/Dreieich ohne passenden Registertreffer (nur namensferne Treffer); keine Betreiber-Website auffindbar; Quelle(n): keine (nur Seed-Verzeichnis). Frontmatter bleibt leer (Klärfall).]

### Recherche 02.10.2026

- [Recherche 02.10.2026: KORREKTUR der Einschätzung vom 01.10.2026 — doch 2 unabhängige Verzeichnisbelege gefunden: tellows.de (Detailseite direkt abgerufen: „Schrott Schramm, Stefan“, Herrnroth 11, 63303 Dreieich, Tel. 06103-322820, gewerbliche Nummer) + cylex.de („Stefan Schramm, Dreieich, Schrotthändler“, Tel. 06103 322820). street/postcode/phone GEFÜLLT, status pruefung → aktiv, website_status unbekannt. Beleglage offen: nur 2 Aggregatoren (kein Register — Northdata weiter ohne Treffer, keine Betreiber-Website); 11880.com und gelbeseiten.de listen ihn im Dreieich-Umkreis NICHT (nur Dietzenbacher Betriebe) — Restunsicherheit bleibt. Quelle(n): tellows.de/num/06103322820, web2.cylex.de.]

### Korrektur 12.10.2026

- [Korrektur 12.10.2026: Re-Audit nach README-Belegstandard. Die Adresse/Telefonnummer in der Recherche vom 02.10. stammen ausschließlich aus Aggregatoren, nicht aus zwei qualifizierenden Quellen; kein passender Betreiber- oder Registerbeleg gefunden. `street`, `postcode`, `phone` geleert und `status` auf `pruefung` zurückgesetzt; alte Werte und ihre Herkunft bleiben im Verlauf sichtbar. Kein Preisbeleg. Quelle(n): https://tellows.de/num/06103322820; https://web2.cylex.de (frühere Verzeichnisbelege; kein qualifizierter Zweitbeleg)]

### Owner-Gate 12.10.2026

- [Korrektur 12.10.2026: Die Produktionsdatenbank enthält noch die frühere Telefonnummer 06103 322820 und den Status `aktiv`; der Abgleich erfolgte read-only. `clear_fields: phone` ergänzt die bereits dokumentierte Rücknahme, damit der alte Aggregator-Kontakt beim nächsten Seed tatsächlich entfernt wird; `status: pruefung` bleibt die Dossierentscheidung. Vorhandene GEO-Koordinaten werden durch das Leeren der Adresse nicht entfernt und müssen separat geprüft werden. Keine Produktionsdaten geändert. Quelle(n): tellows-/Cylex-Leads und Korrektur 12.10.2026; `docs/trader-seed.md`.]
