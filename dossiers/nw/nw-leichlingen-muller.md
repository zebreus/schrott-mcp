---
slug: nw-leichlingen-muller
name: Müller
trader_type: schrotthaendler
state: NW
city: Leichlingen
street: ''
postcode: ''
phone: ''
clear_fields: 'phone'
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Ungeklärter Leichlinger Seed-Eintrag; Registerkandidat Müller Metallankauf GmbH, HRB Köln 48784. Aktueller Betreiberkontakt, Standort-/Annahmebedingungen und Inhaberzuordnung nicht ausreichend belegt; keine bestätigte öffentliche Annahmestelle.'
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Müller

## Überblick

Registerkandidat Müller Metallankauf GmbH, AG Köln HRB 48784, Metzholz 68, 42799 Leichlingen. Northdata ist ein zulässiger Einzelbeleg, Verzeichnisse kein zweiter. Alte strukturierte Adresse/PLZ/Telefon zurückgenommen und „Inh. Axel Müller“-Description durch ausdrücklichen Prüfvorbehalt ersetzt, Werte historisch erhalten. Keine aktuelle Annahme-/Kontaktbestätigung oder Preise. Müller Containerdienst GmbH/Leverkusen HRB 99637 nicht zuordnen; status pruefung statt unbelegtem aktiv.

## Timeline

- [Owner-Korrektur 10.10.2026: Explizite Seed-Direktive clear_fields für phone ergänzt, damit die zurückgenommene unbestätigte Nummer nach geprüfter Veröffentlichung durch den normalen Seed auch im Bestand geleert wird. Kein leerer Dossierwert als vermeintlicher DB-Löschnachweis. Neue nichtleere Beschreibung bewahrt den Prüfvorbehalt; alte Werte unten erhalten. Keine direkte Produktionsmutation, GEO-Löschung oder Schließungsbehauptung. Grundlage: docs/trader-seed.md; research/checkin-20261010-waveD-nordwest-owner-gate.md.]

### Recherche 09.10.2026 / Korrektur 10.10.2026

- [Korrektur 10.10.2026: Die unten zurückgenommene „Inh. Axel Müller“-Description durch nichtleeren Prüfvorbehalt ersetzt, statt leer zu lassen: keep() der Seed-Logik würde sonst eine alte gespeicherte Inhaberbehauptung erhalten. Kein zusätzlicher positiver Standort-/Leistungs-Fill. Leeres phone nimmt den Dossierwert zurück, löscht aber vorhandene DB-Kontakte nicht; gesonderte Owner-Bereinigung vor Veröffentlichung erforderlich. Quelle: crates/ingestion/src/seed_traders.rs, keep()/apply_seed_traders]

- [Recherche 10.10.2026: Positiver Northdata-Einzelfund aus Wave D bestätigt: Müller Metallankauf GmbH, AG Köln HRB 48784, Metzholz 68, 42799 Leichlingen; GF-Publikation Axel Müller 03.06.2003, Gegenstandsänderung 05.09.2024 (Altmetallhandel, Containerdienst, Baustoffe/Schüttgüter, Demontage). GS nennt 02174 6718860, 02174 39350 und 0151 14788231, 2025-Bewertung mit Inhaberantwort; kein unabhängiger Betreiberauftritt. Die per Verzeichnis verknüpfte Domain container-mueller.de nennt im eigenen Impressum Müller Containerdienst GmbH, HRB Köln 99637, Manforter Straße 213/Leverkusen, Betrieb Dieselstraße 12, 02171 582820; keine bewiesene Kette zu HRB 48784. Keine fremden Website-/Kontakt-Fills, kein Ankauf-/Verkaufs-/Gebührentarif. Quelle(n): https://www.northdata.de/M%C3%BCller%20Metallankauf%20GmbH,%20Leichlingen/Amtsgericht%20K%C3%B6ln%20HRB%2048784 ; https://www.gelbeseiten.de/gsbiz/010d092f-035b-4887-a12e-e07ee1118781 ; https://www.container-mueller.de/impressum/]
- [Korrektur 10.10.2026: Unbelegten Aktivstatus auf pruefung zurückgenommen, keine Schließung. Legacywerte street Metzholz 68, postcode 42799, phone 02174 6718860 und description „Müller Metallankauf GmbH (Inh. Axel Müller) — Metallankauf/Schrotthandel in Leichlingen“ aus aktueller Frontmatter geleert und hier erhalten. Adresse nur Register-Einzelbeleg, Telefon/Description Verzeichnis; zweite zulässige Quelle fehlt. Historischer GF-Eintrag bestätigt keine heutige Eigentümer-/Inhaberangabe. Slug/GEO/Typ/Seed-Ort unverändert; Quelle: README.md, Source hierarchy; obige Quellen und Recherche 02.10.2026]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Leichlingen

### Recherche 02.10.2026

- [Recherche 02.10.2026: ok — Adresse/Telefon per Verzeichnis-Konsens gefuellt (Ausnahmefall, offen dokumentiert): Mueller Metallankauf GmbH Inh. Axel Mueller, Metzholz 68, 42799 Leichlingen, Tel. 02174 6718860 (cylex nennt abweichend Mobil 01511 4788... — nicht uebernommen); kein Betreiber-Auftritt, keine Website, kein Registerbeleg gefunden; Adresse neu/gefuellt — Koordinaten neu zu geocodieren; Status aktiv; Quelle(n): dasoertliche.de (Adresse), gelbeseiten.de/gsbiz/010d092f (Adresse/Tel.), dastelefonbuch.de (Tel. 02174 6718860), ortsdienst.de (Adresse/Tel.), lokaleschrottplatz.de (Adresse/Tel.)]
