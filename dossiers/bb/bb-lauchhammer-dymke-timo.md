---
slug: bb-lauchhammer-dymke-timo
name: Dymke Timo e.K.
trader_type: schrotthaendler
state: BB
city: Lauchhammer
street: Dolsthaidaer Str. 45
postcode: 01979
lat: 51.4693927
lon: 13.7510774
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

### Geokodierung 08.11.2026

- [Geokodierung 08.11.2026: **Koordinate der Dossier-Adresse belegt** (`lat`/`lon` = **51.4693927/13.7510774**). Punktgüte: **Hausnummer-Treffer**; der Dossierbetrieb „Dymke Timo e.K.“ ist über die Adresse **Dolsthaidaer Str. 45, 01979 Lauchhammer** adressiert; das OSM-Adressobjekt **w/659282776** trägt `addr:housenumber`=45, `addr:street`=Dolsthaidaer Straße, `addr:postcode`=01979, `addr:city`=Lauchhammer; das Objekt trägt **weder `name=` noch `operator=`** - die Koordinate belegt die **Anschrift**, nicht die Betreiberzuordnung; der Punkt ist der Mittelwert der 6 eindeutigen Way-Knoten; die Nominatim-Vorwärtssuche nach „Dolsthaidaer Str. 45, 01979 Lauchhammer“ liefert unter den 1 Treffern dasselbe Objekt **w/659282776**; das Dossier nennt neben Dolsthaidaer Straße 45 eine zweite Straßenangabe; das OSM-Objekt trägt `addr:street`=Dolsthaidaer Straße und löst damit die Schreibweise der Dossier-Adresse auf.]
  Quelle(n): https://nominatim.openstreetmap.org/search?format=jsonv2&extratags=1&namedetails=1&addressdetails=1&countrycodes=de&limit=20&q=Dolsthaidaer+Str.+45%2C+01979+Lauchhammer ; https://api.openstreetmap.org/api/0.6/way/659282776.json (Abruf 08.11.2026).]
