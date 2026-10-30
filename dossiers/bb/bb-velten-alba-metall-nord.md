---
slug: bb-velten-alba-metall-nord
name: ALBA Metall Nord GmbH
trader_type: schrotthaendler
state: BB
city: Velten
street: Breite Str. 47b
postcode: '16727'
phone: ''
email: ''
opening_hours: ''
website: https://metall.alba.info/
website_status: ''
status: pruefung
description: 'ALBA Metall Nord GmbH, Breite Str. 47b, 16727 Velten: Die Anschrift ist im GHG-Verifizierungszertifikat der ALBA plc & Co. KG (Nr. C-24-25940, 29.11.2024) als Standort 89 "ALBA plc & Co. KG, Breite Str. 47B, 16727 Velten" geführt, im Abfallwirtschaftsplan des Landes Brandenburg (Fortschreibung 2023, Teilplan Gefährliche Abfälle) jedoch als "AWU Abfallwirtschafts-Union Oberhavel GmbH, Breite Straße 47 a, 16727 Velten" mit einem Elektronikschrottzwischenlager. Die Standortseite der ALBA Metall Nord (13 Treffer) enthält Velten nicht. Ob ein ALBA-Metallbetrieb an Breite Str. 47b in Velten tatsächlich besteht, ist offen; die Hausnummern 47a und 47B sind in OSM getrennt erfasst.'
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: 'Nachtrag Audit-Runde 3 (Stand: 2026-09-27)'
provenance_ankauf_raw: ja
provenance_origin: table
---

# ALBA Metall Nord GmbH

## Überblick

ALBA Metall Nord GmbH, Breite Str. 47b, 16727 Velten. Der Klärfall vom 01.10.2026 (kein
Velten auf der ALBA-Metall-Standortseite) ist durch einen neuen amtlichen Beleg nur
teilweise aufgelöst: Das GHG-Verifizierungszertifikat der ALBA plc & Co. KG führt
"Breite Str. 47B, 16727 Velten" als Standort der Konzernmutter, während der Abfallwirtschaftsplan
des Landes Brandenburg an dieser Adresse die AWU Abfallwirtschafts-Union Oberhavel GmbH
ausweist. Die Standortseite der ALBA Metall Nord enthält Velten weiterhin nicht.
Telefon, E-Mail und Öffnungszeiten sind nicht belegt. Ankauf, Verkauf und Gebühren werden
für diese Anschrift nicht mit Zahlen veröffentlicht.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „n.e.“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Importiert (Seed-Stand 2026-09-30)

- Stahl-/Metallschrott
- Adressbeleg: schrottradar-Beleg via Runde 3

### Korrektur 01.10.2026 (ALBA-Fix)

- [Korrektur 01.10.2026 (ALBA-Fix): Filiale NICHT auf Betreiber-Seite (Vollcrawl 13 Detailseiten: kein Velten/Breite Str.); status aktiv → pruefung + Klärfall. KEINE Schließungs-/Verlagerungsbelege (Betreiber-Suche, Websuche ergebnislos), KEINE Frontmatter-Füllung, nichts gelöscht. Klärfall: Existenz/Fortbestand Breite Str. 47b, 16727 Velten (Register/HR prüfen). Quelle(n): metall.alba.info/unternehmen/standorte (live, Vollcrawl 01.10.2026)]

### Recherche 01.10.2026 (Feedback-Triage 2139)

- [Recherche 01.10.2026 (Feedback-Triage 2139): Re-Meldung triagiert — Standort Velten (Breite Str. 47b, 16727) weiterhin nicht auf ALBA-Metall-Standortliste (13 Standorte, kein Velten); ALBA-Fix vom 01.10. (status pruefung, Klärfall) bestätigt und bleibt; keine Änderung, keine Fills (phone/email/hours leer, kein Schließungsbeleg). Quelle(n): https://metall.alba.info/unternehmen/standorte/]

### Recherche 30.10.2026

- [Recherche 30.10.2026: Neue Belege zur Anschrift, der Klärfall bleibt aber offen und wird präzisiert. Das GHG-Verifizierungszertifikat der ALBA plc & Co. KG (Nr. C-24-25940 vom 29.11.2024, Berichtszeitraum 01.01.2023 bis 31.12.2023, Grundlage GHG Protocol, reasonable assurance) listet in seinem Anhang unter den berichtspflichtigen Standorten den Eintrag "89 ALBA plc & Co. KG Breite Str. 47B, 16727 Velten" - die Dossier-Adresse ist damit in einem Originaldokument des Konzerns als Konzernstandort geführt, allerdings unter der Konzernmutter und nicht unter "ALBA Metall Nord GmbH". Gegenläufig weist der Abfallwirtschaftsplan des Landes Brandenburg, Fortschreibung 2023, Teilplan Gefährliche Abfälle, an derselben Anschrift den Betreiber "AWU Abfallwirtschafts-Union Oberhavel GmbH" aus: Nr. 161 "Elektronikschrottzwischenlager, AWU Abfallwirtschafts-Union Oberhavel GmbH, Breite Straße 47 a, 16727 Velten" und Nr. 180 "Zwischenlager für herrenlose gefährliche Abfälle, AWU Abfallwirtschafts-Union Oberhavel GmbH, Breite Straße 47 a, 16727 Velten". Der Abfallwirtschaftsplan 2007 (Teilplan Siedlungsabfälle) führt dieselbe Fläche als "182 Oberhavel PPK/LVP-Sortieranlage Breite Straße 47 b, 16727 Velten" mit dem Betreiber "Abfallwirtschafts-Union Oranienburg GmbH" und zusätzlich als "201 Oberhavel Glasaufbereitungsanlage Breite Straße 47 b, 16727 Velten" mit "Recycling Zentrum Oberhavel GmbH". Das ISO-50001-Zertifikat B-24-13185 vom 20.06.2024 der ALBA plc & Co. KG führt für Velten dagegen "Franz-Josef-Schweitzer-Platz 1, 16727 Velten" als Standort 28 mit der Standortbezeichnung "ALBA plc & Co. KG ℅ ALBA Management GmbH"; die GHG-Anlage nennt für dieselbe Adresse "ALBA Management GmbH". Die Standortseite der ALBA Metall Nord (live abgerufen) enthält weiterhin 13 Treffer und **kein** Velten. Bewertung: Für die Anschrift "Breite Str. 47b, 16727 Velten" ist kein ALBA-Metallbetrieb der ALBA Metall Nord GmbH belegt; belegt sind die Konzernmutter im GHG-Bericht und die AWU bzw. deren Vorgänger im Abfallwirtschaftsplan. Der Klärfall bleibt offen, es wird **keine** Schließung und **keine** Übernahme behauptet; status bleibt pruefung, Name, Anschrift und PLZ bleiben unverändert, das Telefonfeld bleibt leer (der `clear_fields: 'phone'`-Verweis der Owner-Korrektur vom 28.10.2026 bleibt unberührt, eine Nummer wird nicht erfunden). Zu Ankauf, Verkauf und Gebühren getrennt: ANKAUF - kein Ankauf von Stahl- oder NE-Metallschrott durch ALBA an dieser Anschrift belegt; die Betreiber-Seite bewirbt den Ankauf pauschal für "all unsere Plätze", nennt aber gerade keinen Standort Velten. VERKAUF - kein Verkaufspreis und kein Vormaterialangebot veröffentlicht. GEBÜHREN - keine Gebührentafel veröffentlicht. OSM-Abgleich live (Rasterfenster um den Adresspunkt 52.6834871/13.1826383, bbox 13.1786383,52.6794871 bis 13.1866383,52.6874871): 32 Adressobjekte mit `addr:housenumber` und `addr:street=Breite Straße`, daraus **31 verschiedene Hausnummern** (32, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 44A, 45, 47A, 47B, 47a, 48, 50, 51, 53A, 54, 55, 56, 57, 57A, 58, 59, 61, 62, 63, 64); die Sammelnotation "55;56" (way/363906581) ist dabei **zwei** Nummern (55 und 56), und die Nummern 32 (way/200762681, way/494405071) und 57 (way/363906584, way/363906591) tragen je zwei Objekte, die 32 Objekte also 30 verschiedene Roh-Notationen. "47a" (way/494710090) und "47A" (way/1081215522) sind zwei getrennte Objekte mit identischer Schreibweise in unterschiedlicher Groß-/Kleinschreibung, "47B" (way/494711234) ist ein drittes. Die Dossier-Hausnummer 47b ist als "47B" erfasst: way/494711234 trägt `addr:housenumber=47B`, `addr:street=Breite Straße`, `addr:postcode=16727`, `addr:city=Velten`, `building=yes` und **kein** `name=`/`operator=`. Das Nachbarobjekt way/1081215522 an "47A" trägt `name=AWU Abfallwirtschafts-Union Oberhavel GmbH` mit `building=yes` und `addr:city=Velten`. Im Raster tragen 88 Objekte `name=` oder `operator=`; **1** davon nennt die AWU an der Nachbarhausnummer 47A, **0** nennen ALBA. Das ist eine Aussage zum OSM-Bestand und kein Negativbeleg gegen die Betreiberanschrift. Quelle(n): https://metall.alba.info/fileadmin/user_upload/ALBA_Metall_Nord/PDF/2024/Urkunde_Ver_CCF_ALBA_plc_und_Co._KG_GJ2023_final.pdf ; https://www.alba.info/fileadmin/user_upload/ALBA_Sachsen/PDf/Zertifikat_50001_ALBA_2024-2027_20.06.2024.pdf ; https://bravors.brandenburg.de/verwaltungsvorschriften/awp_gefaehrliche_abfaelle_2024 ; https://bravors.brandenburg.de/verwaltungsvorschriften/awp_siedlungsabfaelle_2007 ; https://metall.alba.info/unternehmen/standorte/ ; OSM-API-Abfrage api.openstreetmap.org/api/0.6/map.json?bbox=13.1786383,52.6794871,13.1866383,52.6874871 (Abruf 30.10.2026)]
