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

**Schrotthandel Santoro**, Inhaber **Franco Santoro**, 34639 Schwarzenborn. Der Betreiberauftritt ist
nur über **http** erreichbar (HTTPS scheitert an einem selbstsignierten Zertifikat); die Seite wirbt
Ankauf und Abholung von Schrott und Altmetall, Containerdienst sowie Haushaltsauflösungen im Raum
Schwalm-Eder, Hersfeld-Rotenburg, Vogelsberg, Waldeck-Frankenberg und Kassel.

**Adresslage ungeklärt:** Start- und Kontaktseite nennen **Hauptstraße 2**, das Impressum nennt
**Neue Straße 29**; die eingebettete Karte ist auf "Neue Str. 29" zentriert, während der gesetzte
Marker im Bereich Hauptstraße 2 liegt. **Beide Hausnummern existieren in OSM** (Hauptstraße 2 als
way/185796011, Neue Straße 29 als way/848171827), beide ohne Betreiber- oder Namenstag - die
Wahl zwischen den beiden Anschriften ist aus OSM nicht ableitbar.

**Telefon:** Der Betreiber nennt **+49 5686 930 555** und **+49 173 730 26 68**; die städtische
Gewerbedatenbank nennt abweichend 0173/7302683 (Ziffernverdreher). Öffnungszeiten laut Betreiber
Mo-Fr 08:00-17:00, Sa 08:00-13:00.

**Beleglage:** Die kommunale Gewerbedatenbank der Gemeinde Schwarzenborn führt den Betrieb als
"Schrotthandel Santoro", Franco Santoro - das ist die Bestätigung, mit der der Status auf aktiv
gesetzt wurde. Straße, Telefon, E-Mail und Öffnungszeiten bleiben wegen der fehlenden zweiten Quelle
im Frontmatter leer.

**Ankauf/Verkauf/Gebühren:** keine Zahlenpreise und kein Sortenverzeichnis, kein Warenverkauf,
kein Gebührentarif.

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

### Recherche 30.10.2026

- [Recherche 30.10.2026: **Beide Adressen des Betreibers existieren in OSM; der Widerspruch bleibt ungelöst, die Telefonnummer ist geklärt.** Der Betreiberauftritt `http://www.schrotthandel-santoro.de/` ist am 30.10.2026 erreichbar (HTTPS scheitert weiterhin an einem selbstsignierten Zertifikat); **Startseite und Kontaktseite nennen "Schrotthandel Santoro, Hauptstraße 2, 34639 Schwarzenborn"**, das **Impressum nennt "Inhaber: Franco Santoro, Neue Straße 29, 34639 Schwarzenborn"** - der Widerspruch aus dem Vermerk vom 01.10.2026 besteht also unverändert. Neu ist, dass die in die Seite eingebettete Karten-Konfiguration die **Karte auf "Neue Str. 29, 34639 Schwarzenborn" zentriert** (lat 50.9127738, lon 9.4410472), während der gesetzte Marker bei **lat 50.9159018, lon 9.4756855** liegt, also im Bereich Hauptstraße 2 - die Karte zeigt damit **zwei** Adressen zugleich. **OSM-Raster über den gesamten Straßenverlauf (Hauptstraße, Schwarzenborn-Grebenhagen, Raster 9.4692,50.9099,9.4848,50.9221; 4 Kacheln, 0 Kachel-Fehler): 21 Adressobjekte / 21 verschiedene Hausnummern** (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 17, 19, 21, 23, 25, 27, 29) - **Hauptstraße 2 existiert** als **way/185796011** mit `addr:housenumber=2`, `addr:street=Hauptstraße`, `addr:city=Schwarzenborn`, `addr:postcode=34639`, `building=yes`; **OSM-Raster über den gesamten Straßenverlauf (Neue Straße, Schwarzenborn, Raster 9.4365,50.9052,9.4505,50.9179; 4 Kacheln, 0 Kachel-Fehler): 25 Adressobjekte / 25 verschiedene Hausnummern** (1, 3, 4, 6, 7, 14, 16, 17, 18, 20, 22, 25, 26, 27, **27A**, 28, **29**, 30, 31, 32, 33, 34, 36, 38, 40) - **Neue Straße 29 existiert** als **way/848171827** mit `addr:housenumber=29`, `addr:street=Neue Straße`, `addr:city=Schwarzenborn`, `addr:postcode=34639`, `building=yes`; **keines** der beiden Objekte trägt einen `name`- oder `operator`-Tag, beide sind einfache `building=yes`-Objekte ohne Geschäftsbezug. Die Hausnummern sind also beide **plausibel kartiert**, eine Entscheidung zwischen den beiden Anschriften ist aus OSM nicht ableitbar. **Telefonklärung:** Der Betreiber nennt auf Start- und Kontaktseite **+49 5686 930 555** und **+49 173 730 26 68**, das Impressum zusätzlich **+49 5686 / 930 555**; die städtische Gewerbedatenbank nennt abweichend **0173/7302683** (Ziffernverdreher gegenüber +49 173 730 26 68) - die **beiden Betreiberangaben sind in sich stimmig**, nur die städtische Angabe weicht ab. **Betreiberleistungen (http, live):** Ankauf von Schrott und Altmetall, Abholung, Containerdienst, Haushaltsauflösungen, Einsatzgebiete Schwalm-Eder, Hersfeld-Rotenburg, Vogelsberg, Waldeck-Frankenberg und Kassel; Öffnungszeiten Mo-Fr 08:00-17:00, Sa 08:00-13:00; E-Mail info@schrotthandel-santoro.de; außerdem ein Betriebsferien-Hinweis "vom 10. - 22. August" (ohne Jahr). **Beleglage:** Es gibt weiterhin **keinen Registereintrag** und keine zweite unabhängige Quelle; die städtische Gewerbedatenbank der Gemeinde Schwarzenborn (Gewerbebetriebe) führt den Betrieb nach dem Vermerk vom 03.10.2026 als "Schrotthandel Santoro", Franco Santoro - das ist die kommunale Bestätigung, mit der der Status 2026 auf aktiv gesetzt wurde. Straße, Telefon, E-Mail und Öffnungszeiten bleiben deshalb nach der Zwei-Quellen-Regel **nicht** im Frontmatter. **ANKAUF:** keine Zahlenpreise und kein Sortenverzeichnis veröffentlicht; **VERKAUF:** kein Warenverkauf; **GEBÜHREN:** kein Gebührentarif. Quelle(n): http://www.schrotthandel-santoro.de/ ; http://www.schrotthandel-santoro.de/impressum ; http://www.schrotthandel-santoro.de/ankauf ; https://api.openstreetmap.org/api/0.6/map.json ; https://www.schwarzenborn.de/bauen-gewerbe/gewerbebetriebe]
