---
slug: ni-friesoythe-h-henken-friesoythe-auch-cloppenburg-pap
name: H. Henken GmbH
trader_type: schrotthaendler
state: NI
city: Friesoythe
street: 'Jadestraße 8'
postcode: '26169'
phone: '04491 2191'
email: 'info@henkengmbh.de'
opening_hours: 'Mo–Fr 08:00–17:00 (Annahme bis 16:30); 1. Sa/Monat 08:00–13:00'
website: https://henkengmbh.de
website_status: 'aktiv'
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Osnabrück / Emsland / Grafschaft Bentheim
provenance_ankauf_raw: ja
provenance_origin: table
---

# H. Henken GmbH (Friesoythe; auch Cloppenburg/Papenburg/Leer/Westerstede)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrott/Container/Autoverwertung

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Impressum verifiziert – H. Henken GmbH, Jadestraße 8, 26169 Friesoythe, Annahmestelle Elbestraße 17, HRB 150697 AG Oldenburg, GF Helmut Henken, Tel. 04491 2191, auftrag@henkengmbh.de / info@henkengmbh.de; Zeiten ab 01.04.2026 Mo–Fr 08:00–17:00 (Annahme bis 16:30), 1. Sa/Monat 08:00–13:00, Pausen 09:00–09:15 + 12:00–12:30. Zweitbeleg: Northdata HRB 150697 AG Oldenburg, Adresse Jadestr. 8 D-26169 Friesoythe, Gegenstand An-/Verkauf gebrauchte KFZ + Autoverwertung + Schrott-/Metallhandel + Baustoffhandel, JA bis 2024. Doppelbeleg. Quelle(n): https://henkengmbh.de, https://henkengmbh.de/kontakt/, https://henkengmbh.de/impressum, Northdata HRB 150697 (Abruf 01.10.2026)]
- [Feedback-Triage ID 2359: Meldung berechtigt — (a) Typ: autoverwertung → schrotthaendler (Mischbetrieb per Register-Gegenstand: Schrott-/Metallhandel + Container + Autoverwertung; Schrott/Container = Schwerpunkt laut Startseite, Altautoverwertung als Nebenleistung in Timeline; Altname mit Filial-Zusatz auf Register-Namen gekürzt, Filialorte Cloppenburg/Papenburg/Leer/Westerstede unbelegt → aus name entfernt, in Timeline erhalten). (b) Annahmestelle Elbestraße 17 (Betreiber-Footer/Kontakt) in Timeline vermerkt (kein separates Feld). (c) Adresse/Telefon/E-Mail/Zeiten aus Betreiber-Impressum (register-kongruent, Doppelbeleg) übernommen; Mail info@ (Footer/Startseite) statt auftrag@ (Impressum) — beide Betreiber, info@ als Hauptadresse. Beleglage: Doppelbeleg. Quelle(n): siehe Vorbullet.]

### Preispflege 07.11.2026

- [Preispflege 07.11.2026: **Vier veraltete Current-Preise an der Live-Seite geprüft — alle vier sind Rückzug.** Der Handler läuft (Lauf 07.11.2026 13:09 UTC, 46 Preise übernommen, 8 übersprungen, darunter „Motoren u. Getriebe“, Status `ok`) und erzeugt die vier alten Schlüssel heute nicht mehr — die Pipeline zieht Current-Pointer für normale Handler nicht zurück, darum stehen die alten Werte weiterhin als „aktueller Preis“ in der Datenbank. Betroffen sind `mischschrott` / „Blech 1“ — 85,00 EUR/t, `mischschrott` / „Blech 2“ — 75,00 EUR/t, `mischschrott` / „Motoren u. Getriebe“ — 240,00 EUR/t und `mischschrott` / „Stahl“ — 210,00 EUR/t (alle Stand 29.09.2026).
  Die Seite https://henkengmbh.de/preise-schrott-und-buntmetalle/ (Abruf 07.11.2026 13:02 UTC) trägt die Überschrift „Unsere aktuellen Schrottpreise“ mit dem Stand „Stand: 28.09.2026“ und die Tabelle „Bezeichnung | Preis“; die vier betroffenen Zeilen lauten „Blech 1 - Waschmaschine, Trockner, Fahrrad 20-30 % Müllanteil“ mit der Preiszelle **„85.00 €“**, „Blech 2 - Kühltheken oh. Gefahrstoffe, Draht mit Anhaftung - 30-50 % Müllanteil“ mit **„75.00 €“**, „Stahl (Stärke von 0,6 cm)“ mit **„210.00 €“** und „Motoren u. Getriebe“ mit **„240.00 €“** (Einheit jeweils Tonne).
  1) `mischschrott` / „Blech 1“ und 2) `mischschrott` / „Blech 2“ — der Betrieb bietet beide Sorten weiterhin mit unverändertem Preis an, der Handler führt sie aber seit der FE/NE-Auditwelle (Commit c0a19292) als Shredder-Input: heute `stahlschrott-shredder` / „Blech 1“ (85,00 EUR/t) und `stahlschrott-shredder` / „Blech 2“ (75,00 EUR/t), beide Stand 07.11.2026 13:09 UTC → **Rückzug** (Fremdkörper). 3) `mischschrott` / „Stahl“ — heute `stahlschrott-scheren` / „Stahl“ (210,00 EUR/t, Stand 07.11.2026 13:09 UTC), denn Stahl ab 6 mm Stärke ist Scherenschrott und kein Misch → **Rückzug** (Fremdkörper). 4) `mischschrott` / „Motoren u. Getriebe“ — der Betrieb bietet die Sorte weiterhin mit unverändertem Preis an, der Handler ordnet sie aber keinem Katalogmaterial zu (Verbrenner-Motoren und Getriebe: weder E-Motor noch Misch) und legt dafür keinen Preis an; das Label steht im Lauf 07.11.2026 13:09 UTC unter den acht übersprungenen → der alte Schlüssel `mischschrott` / „Motoren u. Getriebe“ behauptet einen Mischschrott-Preis → **Rückzug** (Fremdkörper).
  Alle vier Preise bleiben in der Historie; zurückgezogen werden soll nur der jeweils alte Current-Pointer. Im Dossier ist keine OSM-/Objekt-ID hinterlegt; für die Dossier-Adresse Jadestraße 8, 26169 Friesoythe liefert die OSM-Live-Abfrage (Nominatim, 07.11.2026) kein Objekt mit dieser Hausnummer → **kein Adressobjekt in OSM**. An den Dossierdaten ändert sich nichts, `street`/`postcode`/`lat`/`lon`/`certifications`/`status` bleiben unverändert. Quelle(n): https://henkengmbh.de/preise-schrott-und-buntmetalle/ (Abruf 07.11.2026 13:02 UTC).]
