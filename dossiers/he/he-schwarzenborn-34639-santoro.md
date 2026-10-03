---
slug: he-schwarzenborn-34639-santoro
name: Santoro
trader_type: schrotthaendler
state: HE
city: Schwarzenborn
street: ''
postcode: '34639'
phone: ''
email: ''
opening_hours: ''
website: 'http://www.schrotthandel-santoro.de/'
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4b – Kandidaten-Sweep (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Santoro

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 03.10.2026

- [Korrektur 03.10.2026 (Audit-Feedback #4524): die im importierten Ortsfeld enthaltene PLZ `34639` in das separate `postcode`-Feld verschoben und `city` auf Schwarzenborn normalisiert. Straßenadresse bleibt wegen des Widerspruchs zwischen Betreiberseite und Stadtverzeichnis ungeklärt; Feldaufteilung bestätigt keine der beiden Straßen. Quelle(n): bisheriger Dossier-Seedwert; Betreiber-/Kommunal-Gegenprüfung im folgenden Recherchevermerk.]

- [Recherche 03.10.2026: Betreiber-Vollcrawl einzeln: http://www.schrotthandel-santoro.de/, http://www.schrotthandel-santoro.de/ueber-uns, http://www.schrotthandel-santoro.de/leistungen, http://www.schrotthandel-santoro.de/schrottabholung, http://www.schrotthandel-santoro.de/ankauf, http://www.schrotthandel-santoro.de/containerdienst, http://www.schrotthandel-santoro.de/haushaltsaufloesungen und http://www.schrotthandel-santoro.de/impressum. Betreiberseiten bieten ausdrücklich Schrottankauf/Abholung; die städtische Gewerbedatenbank listet unabhängig „Schrotthandel Santoro“, Franco Santoro, Schwarzenborn → Status pruefung → aktiv. Identität daher plausibel; keine zweite Firma/kein Merge belegt. Adresse bleibt Klärfall: Betreiber-Kontaktseite nennt Hauptstraße 2, 34639, Impressum dagegen Neue Straße 29; die Stadt listet ebenfalls Franco Santoro, Neue Straße 29. Telefonnummer ebenfalls widersprüchlich: Betreiberseite +49 173 730 26 68 vs. Stadt 0173/7302683; Festnetz +49 5686 930 555, E-Mail und Zeiten stehen nur auf der Betreiberseite. Deshalb Straße, Telefon, E-Mail und Öffnungszeiten nicht gefüllt; kein vollständiger koordinatenreifer Straßenbeleg. HTTP-Seiten abrufbar, HTTPS-Abrufe weiter mit Transportfehlern; website_status aktiv bleibt. Quelle(n): http://www.schrotthandel-santoro.de/, http://www.schrotthandel-santoro.de/ankauf, http://www.schrotthandel-santoro.de/schrottabholung, http://www.schrotthandel-santoro.de/containerdienst, http://www.schrotthandel-santoro.de/impressum, https://www.schwarzenborn.de/bauen-gewerbe/gewerbebetriebe]

### Importiert (Seed-Stand 2026-09-30)

- Kleinst-Schrotthändler
- Adresse: Schwarzenborn 34639

### Recherche 01.10.2026 (Feedback-Triage 2920)

- [Recherche 01.10.2026: Feedback 2920 berechtigt, aber Klärfall — keine Frontmatter-Änderung: Kontaktfakten nur einzelbelegt (Betreiber-Website http only, s. Recherche 01.10.2026 unten) + Straßen-WIDERSPRUCH innerhalb der Betreiberquelle (Kontaktseite Hauptstraße 2 vs Impressum Neue Straße 29, 34639 Schwarzenborn) — kein Zweitbeleg für eine der beiden Straßen (Verzeichnisse gespalten: 11880 Hauptstr. 2 / GS-Örtliches Neue Str. 29). Telefon/E-Mail/Zeiten ebenfalls nur Einzelbeleg → nach Zwei-Quellen-Regel kein Fill. city-Feld unverändert (historisch mit PLZ). Klärfall: Vor-Ort-/Telefoncheck aktuelle Straße + https-Defekt. Quelle(n): s. Recherche-Eintrag 01.10.2026 unten, Re-Verifizierung 01.10.2026]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Website live verifiziert (nur http, https scheitert — TLS-Fehler 01.10.2026): Schrotthandel Santoro, Kontaktseite: Hauptstraße 2, 34639 Schwarzenborn, +49 5686 930 555, +49 173 730 26 68, info@schrotthandel-santoro.de, Mo–Fr 08:00–17:00, Sa 08:00–13:00, Leistungen Abholung/Ankauf/Container/Haushaltsauflösungen; Impressum dagegen: Inhaber Franco Santoro, Neue Straße 29, 34639 Schwarzenborn (Tel. identisch) — WIDERSPRUCH innerhalb der Betreiberquelle → Straße NICHT in Frontmatter. Creditreform-Beleg: Franco Santoro Schrotthandel, Schwarzenborn, Gewerbebetrieb, wirtschaftsaktiv (ohne Straßenangabe). Kein HRB: Owner-Direktive greift nicht; Telefon/E-Mail/Öffnungszeiten Einzelbeleg NUR hier. Klärfall: aktuelle Straße (Hauptstr. 2 vs. Neue Str. 29) + https-Defekt; Quelle(n): schrotthandel-santoro.de (http) + /impressum, Creditreform-Firmeneintrag, Aggregator-Leads (11880: Hauptstr. 2; GS/Örtliches: Neue Str. 29)]
