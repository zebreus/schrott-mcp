---
slug: sl-losheim-bachem-schrotthandel-bachem-name-unsicher-evtl
clear_fields: 'dropoff_json,geo'
name: Schrotthandel Bachem (Name unsicher, evtl. Kolbe)
trader_type: schrotthaendler
state: SL
city: Losheim-Bachem
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
provenance_section: Schrottankauf im Saarland (SL) — Recherche
provenance_ankauf_raw: ja (lt. Verzeichnis)
provenance_origin: table
---

# Schrotthandel Bachem (Name unsicher, evtl. Kolbe)

## Überblick

Seedname ausdrücklich unsicher („evtl. Kolbe“), Ankauf bisher nur Verzeichnisbehauptung. Neuer Lead: namenloser Schrotthandel, Bachemer Str. 53, Losheim-Bachem, 06872 9221121. Keine qualifizierte Verbindung zu Kolbe, kein Betreiber-/Registerbeleg oder Aktivitätsnachweis; aktiven Händlerstatus auf pruefung zurückgenommen. Ankauf-/Verkaufs-/Gebührenkonditionen offen.

## Timeline

### Owner-Korrektur 27.10.2026: falsches GEO und unbelegte Anlieferung zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `49.5181, 6.735` und `dropoff_json={"allowed":true}`. Der Punkt liegt laut Reverse-Lookup an der B 268; die einzige dokumentierte Adressspur ist dagegen ein anonymer Verzeichnis-Lead zu Bachemer Straße 53, dessen Nominatim-Vorwärtssuche ein Gebäude rund fünf Kilometer entfernt findet. Weder diese Verzeichnisangabe noch die alten, nicht zuordenbaren Annahmeclaims verifizieren Betreiberidentität, aktuellen Betrieb oder Kundenzugang. Deshalb zieht `clear_fields: 'dropoff_json,geo'` sowohl die nicht passende Koordinate als auch die allgemeine gespeicherte Anlieferungszusage beim regulären Seed zurück. Keine Ersatzkoordinate, keine Behauptung eines Umzugs oder einer Schließung; `status: pruefung` bleibt. Quellen/Leads: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.5181&lon=6.735&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Bachemer%20Strasse%2053%2C%2066679%20Losheim%20am%20See%2C%20Germany&format=jsonv2&limit=5&addressdetails=1 ; https://www.gelbeseiten.de/gsbiz/d57b9c59-6a9b-44e0-81b0-3ede451d858e ; read-only `public.db` (27.10.2026); README.md, Quellenhierarchie; docs/trader-seed.md.]

### Recherche 27.10.2026: namenlosen Portal-Lead erneut geprüft

- [Recherche 27.10.2026: Gelbe-Seiten-Detail erneut gelesen: weiterhin anonymer Eintrag „Schrotthandel“, Bachemer Str. 53, 66679 Losheim am See-Bachem, 06872 9221121; eingebundene GoLocal-Bewertung datiert 14.10.2015. Das Portal nennt Containerdienst/Altgeräteentsorgung, aber weder Betreiber/„Kolbe“ noch Betreiberseite. Einzelner Aggregator, kein Aktivitäts- oder Annahmebeleg; keine Namens-/Kontaktfüllung, kein GEO. **Ankauf:** keine Konditionen oder Kurse; **Verkauf:** keine Liste; **Gebühren:** keine Tarife/kostenlose Entsorgung bestätigt. Quelle (Lead): https://www.gelbeseiten.de/gsbiz/d57b9c59-6a9b-44e0-81b0-3ede451d858e .]

### Recherche 21.10.2026

- [Recherche 21.10.2026: Gelbe-Seiten-Detail direkt erneut gelesen: weiterhin nur der namenlose Eintrag „Schrotthandel“, Bachemer Str. 53, 66679 Losheim am See-Bachem, 06872 9221121; der einzige eingebundene Bewertungsbeitrag stammt vom 14.10.2015. Eintrag nennt als Leistungsumfang Containerdienst/Altgeräteentsorgung, aber keinen Betreiber, kein Kolbe und keine Betreiber-Website. Das bleibt ein einzelner Aggregator-Lead; keine Identitäts-/Adressänderung und kein Schließungsurteil. ANKAUF: keine Kurse oder vergüteten Bedingungen; VERKAUF: keine Liste; GEBÜHREN: keine Tarife. Quelle: https://www.gelbeseiten.de/gsbiz/d57b9c59-6a9b-44e0-81b0-3ede451d858e .]

### Recherche 18.10.2026 (Welle 1202 West)

- [Recherche 18.10.2026: Das am 14.10. blockierte Gelbe-Seiten-Detail heute direkt vollständig HTTP 200 gelesen: weiterhin nur Schrotthandel/Bachemer Str. 53/66679 Losheim-Bachem/06872 9221121, kein Inhaber Kolbe/Betreiberlink. Leistungsumfang Containerdienst/Altgeräte-Entsorgung ist Portalinhalt; eingebundene GoLocal-Bewertung vom 14.10.2015 kein aktueller Tätigkeitsnachweis. Kein Namens-/Kontaktfill und keine Gleichsetzung des unsicheren Seednamens mit Kolbe. Ankauf: weder Auszahlungsmodell noch Zahlen belegt; Verkauf: keine Liste; Gebühren: kein Tarif oder kostenlose Entsorgung belegt. Betreibername, aktuelle Betriebsstätte und Annahmebedingungen vor GEO offen; Quelle: https://www.gelbeseiten.de/gsbiz/d57b9c59-6a9b-44e0-81b0-3ede451d858e .]

### Recherche 14.10.2026

- [Korrektur 14.10.2026: Gelbe-Seiten-Ortssuche live gelesen (200): namenloser Schrotthandel, Bachemer Str. 53, 66679 Losheim am See–Bachem, 06872 9221121; daneben andere Betriebe, nicht als Kolbe zugeschrieben. Zugehöriges Detail HTTP 403/Challenge; keine vollständige Inhaberidentität oder Betreiberquelle. Google Kolbe/Bachem nur Consent, DuckDuckGo Bot-Challenge. Ein Verzeichnislead und der unsichere Seedname tragen den bisherigen status aktiv nicht; auf pruefung zurückgenommen, keine Kontakte oder Branche erweitert, kein Schließungsbeleg. ANKAUF: nur alte Portalbehauptung, keine Preise; VERKAUF: keine Liste; GEBÜHREN: keine Tarife. Offen: Name/Inhaber am Bachemer Standort, Kolbe-Zuordnung und tatsächlicher Ankauf. Quelle: https://www.gelbeseiten.de/suche/schrotthandel/losheim%20am%20see ; https://www.gelbeseiten.de/gsbiz/d57b9c59-6a9b-44e0-81b0-3ede451d858e (403); https://www.google.com/search?q=%22Kolbe%22+%22Bachem%22+Schrott (Consent).]

### Importiert (Seed-Stand 2026-09-30)

- Schrott
