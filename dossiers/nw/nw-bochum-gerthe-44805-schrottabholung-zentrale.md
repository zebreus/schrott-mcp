---
slug: nw-bochum-gerthe-44805-schrottabholung-zentrale
name: Schrottabholung Zentrale
trader_type: schrotthaendler
state: NW
city: Bochum-Gerthe 44805
street: Dieselstr. 88
postcode: 44805
phone: 01525 9084206
email: ''
opening_hours: ''
website: https://schrottabholung-zentrale.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
certifications: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: ja
provenance_origin: table
---

# Schrottabholung Zentrale

## Überblick

Schrottabholung Zentrale ist ein mobiler Schrott-, Autoverwertungs- und Containerdienst in
der Dieselstraße 88, 44805 Bochum-Gerthe. Das Impressum der Betreiberseite weist Rameh
Schakif als Einzelunternehmer mit derselben Anschrift und der Telefonnummer aus, zwei
Verzeichnisleads stützen Adresse und Nummer, eine Mailadresse ist nur betreiberseitig
belegt. Da kein Registereintrag vorliegt, stützt sich die Identität allein auf die
Betreiberquelle, ein zweiter unabhängiger Beleg fehlt. Offen bleiben damit die Beleglage
der Kontaktdaten sowie eine belastbare Preisangabe, die der Betrieb nicht veröffentlicht.
Im amtlichen Fachbetrieberegister ist weder ein Eintrag als Entsorgungsfachbetrieb noch
nach AltfahrzeugV für den Standort gefunden worden; die Anerkennungen sind freiwillig, ein
Nichteintrag belegt ihren Nichtbestand nicht.

## Timeline

### Recherche 01.10.2026 (Feedback-Triage 2939)

- [Recherche 01.10.2026: Feedback 2939 („PLZ im city-Feld, Abholservice nicht erfasst") Klärfall — keine Frontmatter-Änderung: city-Feld mit PLZ („Bochum-Gerthe 44805") bleibt unverändert (historisch mit PLZ, Slug-fixiert, vgl. RHH-Präzedenz); mobiler Schrott-/AV-/Container-Abholservice bereits in Seed-Timeline dokumentiert (s. unten), kein zusätzlicher Fill. Quelle(n): Seed-Beleg + Betreiber-Seite schrottabholung-zentrale.de (Abruf 01.10.2026)]

### Importiert (Seed-Stand 2026-09-30)

- Schrott/AV/Container, mobil; Preisseite ohne €-Festpreise; mögl. Verbund s. Schrotthändler Dortmund
- Adresse: Bochum-Gerthe 44805 (lt. Vorlage)

### Recherche 02.10.2026

- [Recherche 02.10.2026: Prod-Drift geheilt - Prod trug street Dieselstr. 88 ohne Dossier-Beleg (keep()-Artefakt). Live re-verifiziert: BETREIBER-Impressum (Rameh Schakif, Einzelunternehmer, Dieselstr. 88 Gewerbegebiet, 44805 Bochum, Tel +49 (0) 15259084206, info@schrottabholung-zentrale.de) + Startseite (Dieselstrasse 88, 44805 Bochum, 01525 9084206) + Zweitbelege firmania (Dieselstr. 88, 44805 Bochum-Gerthe) + Cylex (Dieselstr., Gerthe, 01525 9084...) + lokaleschrottplatz (+49 1525 9084206). Fill street/postcode/phone als begruendeter Ausnahmefall (Betreiber-Impressum + Verzeichnis-Konsens; Einzelunternehmen ohne HRB -> Owner-Ausnahme greift formal nicht), offen dokumentiert. E-Mail nur Impressum-Einzelbeleg -> leer. Website live (HTTPS 200) -> website_status aktiv. status aktiv bleibt; Quelle(n): https://schrottabholung-zentrale.de/impressum + https://schrottabholung-zentrale.de/ + firmania.de/bochum/schrottabholung-zentrale-4258382 + Cylex]

### Recherche 02.11.2026

- [Recherche 02.11.2026: **Zertifikats-Recherche ohne Treffer.** Für `Schrottabholung Zentrale` (Dieselstr. 88, 44805 Bochum) wurde im amtlichen Fachbetrieberegister (GESA) weder ein Efb-Zertifikat noch ein AltfahrzeugV-Eintrag gefunden; `certifications` bleibt leer. Suchen und Befunde (live am 02.11.2026, clientseitig gerenderte Vaadin-App, Ergebnisgrid vollständig durchscrollt): (1) Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrottabholung Zentrale` -> **0 Treffer**, ebenso `…/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schrottabholung Zentrale` -> **0 Treffer** (zusätzlich mit dem Filter `Art des Betriebs = Egal` geprüft, ebenfalls 0); (2) Radiusabfrage mit 3 km um die Dossieradresse `…/Entsorgungsfachbetriebe/adr=Dieselstraße 88, 44805 Bochum, Deutschland;…&dst=3&is=1&e=1` -> **24 Treffer**, u. a. `Betriebsstätte Bochum (Logistik)`, `Dieselstraße 3, 44805 Bochum`, `ZZET00700106 / 1`, 0,2 km, `COREBO GmbH`, `Dieselstraße 22, 44805 Bochum`, `ZZKT00100199 / 1`, 0,3 km, `Betriebsstandort`, `Daimlerstr. 18-20, 44805 Bochum`, `ZZET00800018 / 1`, 0,3 km - **kein Treffer auf `Schrottabholung Zentrale` und keiner auf Dieselstr. 88**; der Eintrag `ZZET00800018` gehört nach Detaildialog und Zertifikat zur fremden `H.R.B. Rohstoffhandel Bochum GmbH` und wurde **nicht** übernommen; (3) dieselbe Abfrage in der Route `…/Altfahrzeugverwertung/adr=…&dst=3&is=1&e=1` -> **0 Treffer**; (4) Positivkontrolle mit 10 km Radius um dieselbe Adresse -> 217 Efb-Treffer und 4 Altfahrzeug-Treffer, die Abfragetechnik funktioniert also.
  Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung besitzt.
  Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrottabholung%20Zentrale ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schrottabholung%20Zentrale ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Dieselstra%C3%9Fe%2088%2C%2044805%20Bochum%2C%20Deutschland;7.2990895;51.5224373;7.3005895;51.5239373;7.2975895;51.5209373&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Dieselstra%C3%9Fe%2088%2C%2044805%20Bochum%2C%20Deutschland;7.2990895;51.5224373;7.3005895;51.5239373;7.2975895;51.5209373&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Dieselstra%C3%9Fe%2088%2C%2044805%20Bochum%2C%20Deutschland;7.2990895;51.5224373;7.3040895;51.5274373;7.2940895;51.5174373&dst=10&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Dieselstra%C3%9Fe%2088%2C%2044805%20Bochum%2C%20Deutschland;7.2990895;51.5224373;7.3040895;51.5274373;7.2940895;51.5174373&dst=10&is=1&e=1 (Abruf 02.11.2026).]
