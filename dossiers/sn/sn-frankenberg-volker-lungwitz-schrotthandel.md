---
slug: sn-frankenberg-volker-lungwitz-schrotthandel
name: Volker Lungwitz Schrotthandel e.K.
trader_type: schrotthaendler
state: SN
city: Frankenberg
street: Mühlenstr. 7
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://vlschrott.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: 4) Chemnitz und Mittelsachsen
provenance_ankauf_raw: ja
provenance_origin: table
---

# Volker Lungwitz Schrotthandel e.K.

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Fe/Bunt/Kabel, Tagespreisliste netto
- Di/Do bis 18 Uhr

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2656 geprueft — Betreiberseite nennt Muehlenstr. 7, 09669 Frankenberg, 037206 82055, info@vlschrott.de, Zeiten Mo/Mi/Fr 7-16, Di/Do 7-18 — Einzelunternehmen ohne HRB, Einzelbeleg ohne Zweitquelle, daher Timeline-Vermerk ohne Frontmatter-Fill, website live aktiv; Quelle(n): https://vlschrott.de/, https://vlschrott.de/oeffnungszeiten/]

### Korrektur 07.11.2026 (Preispflege)

- [Korrektur 07.11.2026 (Preispflege): **Drei veraltete Current-Preise an der Live-Seite geprüft — alle drei sind Rückzug.** Der Handler läuft (Lauf 07.11.2026 09:07 UTC, 47 Preise übernommen, 6 übersprungen, Status `ok`) und erzeugt die drei alten Schlüssel heute nicht mehr — die Pipeline zieht Current-Pointer für normale Handler nicht zurück, darum stehen die alten Werte weiterhin als „aktueller Preis“ in der Datenbank. Betroffen sind `blei` / „Kabelblei“ — 0,40 EUR/kg, `edelstahl-gemischt` / „“ — 0,15 EUR/kg und `kabel-kupfer` / „Cu-Pb Papier“ — 1,55 EUR/kg (alle Stand 29.09.2026).
  Die Seite https://vlschrott.de/einkaufspreise/ (Abruf 07.11.2026 13:02 UTC) trägt die Tabellenüberschrift „Preisliste vom | 05.11.26 09:47“ und führt die drei betroffenen Zeilen als „Kabelblei“ mit der Preiszelle **„0,40 €/kg“**, „Chromstahl“ mit **„0,15 €/kg“** und „Cu-Pb Kabel m. Papier ab 5 cm“ mit **„1,50 €/kg“**.
  1) `blei` / „Kabelblei“ — der Betrieb bietet die Sorte unverändert an, der Handler führt sie seit dem Kabel-Split (Commit 8258caf4) als `kabel-blei` / „Kabelblei“ (0,40 EUR/kg, Stand 07.11.2026 09:07 UTC) → **Rückzug** (Fremdkörper). 2) `edelstahl-gemischt` / „“ — der Betrieb bietet die Sorte unverändert an, der Handler führt sie heute aber mit der Variante „Chromstahl“: `edelstahl-gemischt` / „Chromstahl“ (0,15 EUR/kg, Stand 07.11.2026 09:07 UTC) → **Rückzug** (Fremdkörper, andere Bezeichnung). 3) `kabel-kupfer` / „Cu-Pb Papier“ — der Betrieb bietet die Sorte an, heute mit **„1,50 €/kg“** (Datenbankwert 1,55 EUR/kg), der Handler führt sie als `kabel-blei` / „Cu-Pb Papier“ (1,50 EUR/kg, Stand 07.11.2026 09:07 UTC) → **Rückzug** (Fremdkörper).
  Alle drei Preise bleiben in der Historie; zurückgezogen werden soll nur der jeweils alte Current-Pointer. Im Dossier ist keine OSM-/Objekt-ID hinterlegt; die Dossier-Adresse Mühlenstr. 7, 09669 Frankenberg ist in OSM als way/377904593 geführt (`addr:housenumber`=7, `addr:street`=Mühlenstraße, `addr:postcode`=09669, `addr:city`=Frankenberg/Sa., `building`=residential), daneben way/288471321 (Hausnummer 7, `building`=apartments; live abgerufen 07.11.2026). An den Dossierdaten ändert sich nichts, `street`/`postcode`/`lat`/`lon`/`certifications`/`status` bleiben unverändert. Quelle(n): https://vlschrott.de/einkaufspreise/ (Abruf 07.11.2026 13:02 UTC).]
