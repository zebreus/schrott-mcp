---
slug: nw-bochum-blomeke
clear_fields: 'geo'
name: Blömeke
trader_type: sonstige
state: NW
city: Bochum
street: Seilfahrt 12
postcode: 44809
phone: 0234 57160
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Blömeke

## Überblick

Der Datensatz **Blömeke** passt als Verzeichnislead zu „Blömeke H. Dieter“ an Seilfahrt 12, 44809 Bochum, Telefon 0234 57160. Eine aktuelle 11880-Branchenliste ordnet den Eintrag Altmaterialhandel, Altbausanierung und Hochbau zu; Recyclingpool führt an derselben Adresse/Telefonnummer **Blömeke Bau GmbH** und als Branche Altmaterial-Großhandel. Das stützt eine mögliche Firmen-/Standortspur, ist aber kein Betreiber- oder Registerbeleg und belegt weder eine Schrottannahmestelle noch aktuellen Betrieb. Der bisherige Status `aktiv` beruhte nur auf Verzeichnis-Konsens und wird deshalb auf `pruefung` zurückgenommen.

**Ankauf:** keine bestätigte Annahme oder Preise. **Verkauf:** keine Produkt-/Preisliste. **Gebühren:** keine Tarife oder belastbare Gratiszusage. **GEO:** Adresse und Branchentätigkeit zuerst unabhängig als tatsächlichen Kundenstandort bestätigen; keine Koordinaten aus dem Verzeichnis übernehmen.

## Timeline

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 51.4740889, 7.2192389. Nominatim-Reverse löst den Punkt in der Taubenstraße, Ehrenfeld/Wiemelhausen, auf, nicht an Seilfahrt 12 in Bochum-Hamme. `clear_fields: 'geo'` entzieht den nicht passenden Altpunkt beim nächsten regulären Seed. Die Seilfahrt-Adresse bleibt ein Verzeichnislead; keine Ersatzkoordinate gesetzt. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.474088888888886&lon=7.219238888888889&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026: Firmen- und Leistungslead präzisiert

- [Recherche 25.10.2026: 11880-Suchseite „Altmaterialhändler Bochum“ und Recyclingpool-Profil direkt gelesen. 11880 führt „Blömeke H. Dieter“, Seilfahrt 12, 44809 Bochum-Hamme, Tel. 0234 57160, mit Altmaterialhandel/Altbausanierung/Hochbau. Recyclingpool führt an derselben Adresse und mit derselben Rufnummer „Blömeke Bau GmbH“; dessen ergänzende allgemeine Themenbeschreibung ist ausdrücklich nicht firmenspezifisch. Das ist ein Identitäts-/Branchenlead, keine Betreiberwebsite, kein aktueller Registerauszug und kein Nachweis für Annahme oder Ankauf. Keine Preise: Ankauf ohne Kurs/Bestätigung, Verkauf ohne Liste, Gebühren ohne Tarif. Kein GEO-Fill. Die Beleglage trägt `aktiv` nicht; Status zu `pruefung` korrigiert. Quellen (Verzeichnis-/Branchenbuch-Leads): https://www.11880.com/suche/altmaterialhandel/bochum ; https://www.recyclingpool.de/abfallwirtschaft-blomeke-bau-gmbh-in-bochum-12869 .]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Fill Strasse/PLZ/Tel. (Seilfahrt 12, 44809 Bochum-Hamme, 0234 57160), Status pruefung zu aktiv — Beleglage offengelegt: NUR drei uebereinstimmende Verzeichnisquellen (11880, werkenntdenBESTEN, schrottplatz24-Profil), kein Betreiber-Webauftritt, kein HR-Treffer; Ausnahmefall, Restunsicherheit bleibt; Quelle(n): 11880.com/suche/schrotthandel/bochum, werkenntdenbesten.de/schrotthandel/bochum, schrottplatz24.net/stadt/bochum/profil/bloemeke-h-dieter]

### Importiert (Seed-Stand 2026-09-30)

- Hinterhof-Typ (GS)
- Adresse: Bochum

### Recherche 01.10.2026

- [Recherche 01.10.2026: kein Handelsregister-Firmentreffer (Northdata: nur Personen-Treffer Blömeke, keine Bochumer Firma), keine Betreiber-Website auffindbar; nur Verzeichnis-Leads → keine Frontmatter-Fills; Hinterhof-Typ unbelegt; Klärfall bleibt, Status pruefung]
