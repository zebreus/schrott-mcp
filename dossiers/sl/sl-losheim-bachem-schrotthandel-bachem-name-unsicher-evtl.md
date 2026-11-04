---
slug: sl-losheim-bachem-schrotthandel-bachem-name-unsicher-evtl
lat: 49.4753962
lon: 6.7092501
clear_fields: 'dropoff_json'
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

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **49.4753962/6.7092501** (Mittel der 4 eindeutigen Knoten von
  way/343673544; der Punkt liegt innerhalb der Objekt-BBox
  49.4753097/6.7091307–49.4754826/6.7093696). Quelle 1 (OSM, live geladen am 04.11.2026):
  **way/343673544** trägt `addr:street`=Bachemer Straße, `addr:housenumber`=**53**,
  `addr:postcode`=66679, `addr:city`=Losheim am See, `addr:country`=DE und `building`=yes - Straße,
  Hausnummer, PLZ und Ort stimmen mit der Dossier-Adresse **Bachemer Str. 53, 66679 Losheim-Bachem**
  überein. Das Objekt trägt **weder `name=` noch `operator=`**; es ist eine **Adressangabe** eines
  Gebäudes, kein Betriebs-POI. Quelle 2 (im Dossier bereits zitiert, live nachgelesen am 04.11.2026):
  der Gelbe-Seiten-Eintrag
  https://www.gelbeseiten.de/gsbiz/d57b9c59-6a9b-44e0-81b0-3ede451d858e liefert HTTP 200 und führt im
  LocalBusiness-Block **„Schrotthandel“, streetAddress „Bachemer Str. 53“, postalCode „66679“,
  addressLocality „Losheim am See“, Telefon „06872 9 22 11 21“, keywords „Containerdienst,
  Altgeräte Entsorgung“** mit der Kartenposition **49.475384/6.7091746** - die **rund 6 m** vom
  gesetzten OSM-Punkt entfernt liegt und die Anschrift damit unabhängig bestätigt. **Fensterprüfung
  (kein Fensterfehler):** das am 04.11.2026 live abgerufene Kachelraster umfasst
  **6.7000/49.4700 bis 6.7100/49.4800**, **1 Kachel von je 0,01 Grad**, **0 Kachel-Fehler**,
  **5.502 Elemente**; das Fenster enthält die gesuchte Straße **„Bachemer Straße“ nachweislich** mit
  den vier Straßenobjekten **way/28606568** und **way/565148956** (`highway`=unclassified) sowie
  **way/30292855** und **way/31456050** (residential) und **64 Adressobjekten** auf „Bachemer Straße“
  mit **64 verschiedenen Hausnummern** (1–57 ohne 54 und 56, dazu 6a, 8a, 14a, 14b, 15a, 39a, 40a,
  40b, 47a) - die **Dossier-Hausnummer 53 existiert** und ist im Raster das einzige Objekt mit dieser
  Nummer auf dieser Straße. **Negativaussage, qualifiziert:** im Raster tragen **88 Objekte** `name=`
  oder `operator=`; **mit Bezug zu „Kolbe“, „Schrott“, „Recycling“ oder „Containerdienst“** trifft es
  auf **kein** Objekt dieser Adresse - der Betrieb ist in OSM **nicht** kartiert und trägt in der
  Verzeichnisquelle **keinen Inhabernamen** (der Dossier-Name „Bachem“ bzw. „evtl. Kolbe“ bleibt
  ungeklärt). **Zurücknahme des `geo`-Tokens (Begründung):** `clear_fields: 'dropoff_json,geo'` wurde
  auf **`'dropoff_json'`** gekürzt - das Token `geo` ist **entfernt**, weil es den am 27.10.2026
  zurückgenommenen und an der B 268 liegenden Produktionspunkt (49.5181/6.735) fernhalten sollte; das
  Token `dropoff_json` bleibt **unverändert** bestehen, weil die allgemeine Annahmezusage weiterhin
  nicht belegt ist. `clear_fields` darf nicht gleichzeitig `geo` neben belegtem `lat`/`lon` stehen.
  **Qualifizierung (unverändert übernommen):** Der Punkt bezeichnet die **Adresse eines
  namenlosen Verzeichnis-Leads**, **nicht** einen verifizierten Annahmehof, eine Waage oder eine
  öffentliche Schrottannahme; die im Dossier offenen Punkte (Inhaber/Kolbe-Zuordnung, aktueller
  Betrieb, Annahmebedingungen, Mindestmengen) bleiben unverändert, ebenso `status: pruefung`,
  `street`, `postcode`, `phone`, `email`, `opening_hours`, `website`, `website_status` und
  `description`. Es wird kein Ankaufkurs, keine Mindestmenge, keine Preisliste und keine Gebühr
  behauptet. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/343673544/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=6.7000,49.4700,6.7100,49.4800 ;
  https://www.gelbeseiten.de/gsbiz/d57b9c59-6a9b-44e0-81b0-3ede451d858e (Abruf 04.11.2026).]
