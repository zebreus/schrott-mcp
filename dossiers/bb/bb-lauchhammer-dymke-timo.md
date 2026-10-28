---
slug: bb-lauchhammer-dymke-timo
name: Dymke Timo e.K.
trader_type: schrotthaendler
state: BB
city: Lauchhammer
street: Dolsthaidaer Str. 45
postcode: 01979
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
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Dymke Timo e.K.

## Überblick

Der Lauchhammer-Schrotthandel Timo Dymke e.K. ist durch einen Registerdatenbanktreffer als Rechtsträger plausibilisiert; der dort genannte Inhaber Sascha Schwarzenberger und die Bockwitzer Straße 97 weichen von der Seed-Adresse Dolsthaidaer Straße 45 ab. Gelbe-Seiten- und Stadtportal-Angaben zur Dolsthaidaer Straße sind keine Betreiberbestätigung und widersprechen sich teils sogar bei der Hausnummer. Identität und aktuelle Kundenannahme an einer konkreten Adresse bleiben ungeklärt; `status: pruefung`. Keine belastbaren **Ankaufspreise**, **Verkaufspreise** oder **Gebühren**; nicht geokodierbar, bis die Adresse aufgelöst ist.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „n.e.“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 20.10.2026

- [Recherche 20.10.2026: Exakte Namenssuche nach „Schrotthandel Timo Dymke“ und Inhaber Sascha Schwarzenberger brachte keine zusätzliche Betreiberseite oder unabhängig aktuelle Annahmestellenbestätigung. Der bestehende Creditreform-Treffer bleibt ein einzelner Registerdatenbankbeleg für den Rechtsträger und die abweichende Bockwitzer-Str.-97-Anschrift; die Seed-/Verzeichnisadresse Dolsthaidaer Str. 45 sowie der zusätzliche Verzeichnislead Dolsthaidaer Str. 40 bleiben ungeklärt. Keine Zusammenführung oder Frontmatter-Korrektur; Status pruefung. ANKAUF/VERKAUF/GEBÜHREN: keine zuordenbare numerische Liste gefunden. GEO: keine Koordinaten bei widersprüchlichen Betriebsadressen. Quelle: https://firmeneintrag.creditreform.de/01979/3050246840/SCHROTTHANDEL_TIMO_DYMKE_E_K_INHABER_SASCHA_SCHWARZENBERGER (abgerufen 20.10.2026); ergänzend bestehende Registerspiegel in den Rechercheeinträgen vom 01./13.10.2026.]

### Recherche 13.10.2026

- [Recherche 13.10.2026: Creditreform führt „Schrotthandel Timo Dymke e.K. Inhaber Sascha Schwarzenberger“ und nennt eine Registerbekanntmachung vom 15.09.2021; Anschrift Bockwitzer Str. 97, 01979 Lauchhammer. Die Seed-/Gelbe-Seiten-Adresse Dolsthaidaer Str. 45 bleibt davon abweichend; ein Stadtportal nennt zusätzlich Dolsthaidaer Str. 40. Diese Registerdatenbank- und Verzeichnisangaben lösen weder aktuelle Betriebsstätte noch Annahmepunkt auf, daher weder Straße noch Status geändert und kein Merge mit einem anderen Betrieb. ANKAUF/VERKAUF/GEBÜHREN: keine verifizierte Zahlenliste. GEO: beide Straßenangaben ungeklärt. Quelle: https://firmeneintrag.creditreform.de/01979/3050246840/SCHROTTHANDEL_TIMO_DYMKE_E_K_INHABER_SASCHA_SCHWARZENBERGER ; weitere Adress-Leads siehe bestehende Recherche 01.10.2026.]

### Importiert (Seed-Stand 2026-09-30)

- Schrott/Altmetall (GS-verifiziert; 11880-Zweitadresse Bockwitzer Str. 97 überholt — ein Betrieb)
- Adresse: Lauchhammer, Dolsthaidaer Str. 45
- Adressbeleg: Seed-notes (GS-verifiziert); PLZ Orts-PLZ

### Korrektur 30.09.2026

- [Korrektur 30.09.2026: status aktiv → pruefung — Existenz nur durch Verzeichnisbeleg, kein unabhaengiger Zweitbeleg (Feedback 1464/1465); Re-Pruefung per Folgewelle.]

### Recherche 01.10.2026

- [Recherche bb-lauchhammer-dymke-timo: HR-Eintrag belegt Existenz: Schrotthandel Timo Dymke e.K., Inhaber Sascha Schwarzenberger, HRA 3929 CB AG Cottbus — ABER HR-Adresse Bockwitzer Straße 97, 01979 Lauchhammer weicht von Dossier-Straße Dolsthaidaer Str. 45 ab (Einzelbeleg HR, kein Zweitbeleg → KEINE Korrektur, City-Mismatch-Regel; street bleibt); keine Betreiber-Website, keine Verzeichnistreffer über Seed hinaus; status bleibt pruefung; Quelle(n): https://www.online-handelsregister.de/handelsregisterauszug/br/Cottbus/HRA/3929CB/Schrotthandel-Timo-Dymke-eK-Inhaber-Sascha-Schwarzenberger]
