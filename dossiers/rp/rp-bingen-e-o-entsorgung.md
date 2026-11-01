---
slug: rp-bingen-e-o-entsorgung
name: E&O Entsorgung GmbH
trader_type: schrotthaendler
state: RP
city: Bingen
street: 'Am Ockenheimer Graben 24'
postcode: '55411'
phone: '06721 984050'
email: 'info@eo-entsorgung.de'
opening_hours: 'Mo-Fr 07:00-16:00'
website: https://eo-entsorgung.de/
website_status: 'aktiv'
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Rheinland-Pfalz (RP) — Recherche
provenance_ankauf_raw: Ankauf Ja
provenance_origin: table
---

# E&O Entsorgung GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Alu, Kupfer, Messing, Kabel, Edelstahl, E-Schrott; Mindestmengen!

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2575 berechtigt — Betreiber-Impressum (E&O Entsorgung GmbH, Am Ockenheimer Graben 24, 55411 Bingen am Rhein, HRB 51294 AG Mainz, GF Dirk Sauer) + HR-Spiegel (aktiv, HRB 51294, selbe Adresse, gegr. 08.06.2022, Gegenstand Recycling/Handel insb. Elektroschrottkomponenten) adressgleich → Owner-Ausnahme erfüllt → street/postcode/phone/email gefüllt; Zeiten Mo-Fr 07:00-16:00, Sa geschlossen (Impressum-Footer, gilt für Betriebe Bingen/Kelkheim/Maintal); DB-lat/lon (48.11/9.27, Raum Riedlingen, >130 km falsch) durch Adress-Fill überholt — Koordinaten neu zu geocodieren; Website von Deep-URL auf Domain-Root normiert; Quelle(n): https://eo-entsorgung.de/impressum/, https://eo-entsorgung.de/schrott-metalle/, online-handelsregister.de HRB 51294]

### Recherche 01.11.2026

- [Recherche 01.11.2026: **ANKAUFPREIS des Betreibers belegt — datierte Gruppen-Preisliste.** Live abgerufen am 01.11.2026 07:09 UTC: `https://eo-entsorgung.de/wp-content/themes/eo-entsorgung/altmetall-pdf.php` → HTTP 200, `application/pdf`, 40.635 Bytes, 2 Seiten, per `pdftotext -layout` ohne OCR lesbar (Dateilink: https://eo-entsorgung.de/wp-content/themes/eo-entsorgung/altmetall-pdf.php). Wörtliche Preisfragmente: Kopfzeile jeder Seite „(Stand: 07.12.2023)", Titel „ALTMETALL ANKAUF-PREISLISTE (Preis je Tonne)"; Werte „SCHREDDERVORMATERIAL / MISCHSCHROTT LEICHT € 160,00", „MISCHSCHROTT SCHWER € 200,00", „CU MILBERRY € 6.400,00", „CU WICU RAFF € 2.849,00", „CU KABEL MIT STECKER € 1.048,00", „MESSING SCHWER € 3.700,00", „MS 58 ABFÄLLE € 4.050,00", „ALU NEU € 1.000,00", „ALTZINK € 1.200,00", „ALTBLEI € 1.300,00", „E-MOTOREN € 500,00" (39 Sorten). Fußzeile jeder Seite: „E&O Entsorgung GmbH | Am Ockenheimer Graben 24 | 55411 Bingen / Kelkheim (Zweigstelle) | Zeilsheimer Weg 4 | 65779 Kelkheim / Maintal (Zweigstelle) | Gutenbergstraße 8 | 63477 Maintal | Telefon: 06721 / 98 405-0 | E-Mail: info@eo-entsorgung.de" — der Sitz **Bingen** wird darin zuerst und ohne Vorbehalt genannt. PREISART: **Ankauf** (der Betreiber zahlt dem Verkäufer), ausdrücklich Ankauf-Preisliste; keine Gebührenliste, kein Verkaufskurs des Betreibers. EINHEIT: **Euro je Tonne**. GÜLTIGKEIT: „(Stand: 07.12.2023)" — rund 2 Jahre und 11 Monate alt, also historische Richtwerte und keine Tageskurse; die PDF-Metadaten (`CreationDate`/`ModDate` = Erzeugungszeitpunkt des Abrufs) sind ausdrücklich kein Preisdatum. Quelle(n): https://eo-entsorgung.de/wp-content/themes/eo-entsorgung/altmetall-pdf.php (Abruf 01.11.2026 07:09 UTC)]
- [Recherche 01.11.2026: **Standortbezug Bingen und Grenzen der Preisaussage.** Die maschinenlesbare Zwillingsquelle `https://eo-entsorgung.de/wp-json/wp/v2/pages?slug=ankauf-altmetall_offenbach` (HTTP 200, `application/json`, 82.439 Bytes, Abruf 01.11.2026 07:09 UTC) trägt dieselben 39 Werte in €/Tonne, aber **kein Preis-Datum** (WordPress-Metadaten `date 2023-11-17T11:45:46`, `modified 2025-07-17T07:33:36`); die vier Schwester-Seiten `ankauf-altmetall_wiesbaden|mainz|frankfurt|darmstadt` führen identische Wertelisten, einen Slug für Bingen gibt es nicht (`/bingen/` liefert HTTP 301 auf `/containerdienst/bingen/`). „Bingen" kommt in **keiner** der fünf Preis-Seiten vor; deren „Wertstoffhöfe:"-Block nennt ausschließlich „Wertstoffhof Maintal, Gutenbergstraße 8, 63477 Maintal", die Seitenüberschrift lautet jeweils „Ankauf von Altmetall im Raum …". Die Preisliste ist damit eine **Konzern-Preisliste des Sitzes Bingen** (PDF-Fußzeile) mit publiziertem Standortbezug über /schrott-metalle/ („Mit den Standorten Bingen, Kelkheim und Maintal finden Sie gleich drei Anlaufstellen in der Metropolregion Rhein-Main") — aber **keine** Binger-spezifische Preisliste und keine Binger Mengenstaffel. Der Abweichungsvorbehalt „Info: In Kelkheim erfolgt der Schrottankauf über die Kilb Vetter Entsorgung GmbH, die Schrottpreise können abweichen!" betrifft Bingen nicht. MINDESTMENGEN (standortübergreifend, /schrott-metalle/): „Ankauf von Nichteisenmetallen sortenrein, wie z.B. Alu, Kupfer, Messing, Zink, Blei. Bis 10 kg kostenlose Annahme. Ankauf erst ab 11 kg."; „Ankauf von Edelstahl; VA, E-Motore, Bleibatterien. Bis 20 kg kostenlose Annahme. Ankauf erst ab 21 kg."; „Ankauf von Elektronikschrott gemischt ... Bis 100 kg kostenlose Annahme. Ankauf erst ab 101 kg."; „Beim Ankauf von Eisenschrotten (FE-Schrott) erfolgt eine Vergütung ab einer Mindestmenge von 100 kg!"; aus der Preislisten-Seite zusätzlich „Mindestankaufsmenge Mischschrott / Schreddervormaterial 200 kg, ansonsten nur kostenfreie Annahme". Quelle(n): https://eo-entsorgung.de/wp-json/wp/v2/pages?slug=ankauf-altmetall_offenbach ; https://eo-entsorgung.de/schrott-metalle/ (Abruf 01.11.2026 07:09 UTC)]
