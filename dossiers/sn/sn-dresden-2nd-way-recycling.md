---
slug: sn-dresden-2nd-way-recycling
name: 2nd way Recycling GmbH
trader_type: metallhaendler
state: SN
city: Dresden
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://www.second-way.de
website_status: 'aktiv'
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: 1) Dresden, Freital, Heidenau, Pirna
provenance_ankauf_raw: ja
provenance_origin: table
---

# 2nd way Recycling GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrott, Buntmetall
- kleiner Händler

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2623 berechtigt-dokumentiert (Klaerfall) — Betreiber-Impressum + Startseite live verifiziert (eine Website = eine Quelle): 2nd way Recycling GmbH, Fritz-Reuter-Str.41 (Impressum-Schreibweise OHNE Leerzeichen — DB-Wert Fritz-Reuter-Str.41 stimmt, Feedback-Vorschlag mit Leerzeichen widerlegt), 01097 Dresden, Tel. 0173 5963719, info@second-way.de, HRB 42903 AG Dresden; Zeiten Mo-Fr 09:30-17:00 / Sa 09:30-12:00 (Startseite); HR-Live steht aus → Adress-/Kontakt-Fakten nur Timeline, kein Frontmatter-Fill; nur website_status → aktiv; Quelle(n): https://www.second-way.de/impressum + https://www.second-way.de/ (Abruf 01.10.2026)]

### Korrektur 07.11.2026 (Preispflege)

- [Korrektur 07.11.2026 (Preispflege): **Ein veralteter Current-Preis an der Live-Seite geprüft — Rückzug (Fremdkörper).** Der Handler läuft (Lauf 07.11.2026 13:40 UTC, 43 Preise übernommen, 8 übersprungen, Status `ok`) und erzeugt den alten Schlüssel heute nicht mehr — die Pipeline zieht Current-Pointer für normale Handler nicht zurück, darum steht der alte Wert weiterhin als „aktueller Preis“ in der Datenbank. Betroffen ist `kabel-kupfer` / „Cu-Pb Papier“ — 0,90 EUR/kg, Stand 29.09.2026.
  Die Seite https://www.second-way.de/ankaufspreise/ (Abruf 07.11.2026 13:02 UTC) trägt über der Tabelle den Vermerk „letzte Aktualisierung 06.11.26 um 09:05 Uhr“ und führt die betroffene Zeile als „CU-Kabel m. PB u.Papier, fettfrei, min 5cm“ mit der Preiszelle **„0,90 EUR/kg“**.
  Der Betrieb bietet die Sorte also weiterhin mit unverändertem Preis an, der Handler führt sie seit dem Kabel-Split (Commit 8258caf4) aber als eigenes Material `kabel-blei` / „Cu-Pb Papier“ (0,90 EUR/kg, Stand 07.11.2026 13:40 UTC) → der Schlüssel `kabel-kupfer` / „Cu-Pb Papier“ wird heute nicht mehr erzeugt → **Rückzug** (Fremdkörper; der Preis bleibt in der Historie).
  Im Dossier ist keine OSM-/Objekt-ID hinterlegt; die Dossier-Adresse Fritz-Reuter-Str. 41, 01097 Dresden ist in OSM als way/43762150 geführt (`addr:housenumber`=41, `addr:street`=Fritz-Reuter-Straße, `addr:postcode`=01097, `addr:city`=Dresden, `building`=office, `phone`=+49 173 5963719; live abgerufen 07.11.2026). An den Dossierdaten ändert sich nichts, `street`/`postcode`/`lat`/`lon`/`certifications`/`status` bleiben unverändert. Quelle(n): https://www.second-way.de/ankaufspreise/ (Abruf 07.11.2026 13:02 UTC).]
