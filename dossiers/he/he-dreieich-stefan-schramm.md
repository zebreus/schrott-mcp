---
slug: he-dreieich-stefan-schramm
name: Stefan Schramm
trader_type: schrotthaendler
state: HE
city: Dreieich
street: ''
postcode: ''
phone: ''
clear_fields: 'phone'
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
certifications: ''
---

# Stefan Schramm

## Überblick

Der vorhandene Eintrag zu Herrnroth 11, 63303 Dreieich und 06103 322820 stammt aus tellows/Cylex-Verzeichnissen; ein Betreiber-, Register- oder kommunaler Gewerbenachweis ist weiterhin nicht belegt. Die 2026er Verzeichnisspur allein rechtfertigt weder Kontakt-Fills noch `aktiv`; diese Angaben wurden zurückgenommen und bleiben als Leads in der Timeline.

**Preise:** Ankauf — keine bestätigte Vergütung/Preisliste; Verkauf — keine Liste; Gebühren — keine Tarife belegt.

## Timeline

### Recherche 26.10.2026

- [Recherche 26.10.2026: tellows-Profil zur 06103 322820 direkt erneut gelesen: es wiederholt „Schrott Schramm, Stefan“, Herrnroth 11, 63303 Dreieich und bewertet die Nummer als gewerblich. tellows bleibt ein Verzeichnis-/Community-Eintrag, kein Betreiber-, Register- oder kommunaler Gewerbenachweis; die parallele Cylex-Spur ist derselben nicht qualifizierten Quellenklasse zuzurechnen. Keine neuen Identitäts-/Annahmebelege, kein Preis- oder Tarifblatt; frühere Rücknahme von Straße, PLZ und Telefon sowie `status: pruefung` bleiben bestehen. Quelle(n): https://www.tellows.de/num/06103322820 ; https://web2.cylex.de/firma-home/stefan-schramm-16751609.html (Direktabruf 403).]

### Importiert (Seed-Stand 2026-09-30)

- Schrotthändler (Verzeichnis)

### Recherche 01.10.2026

- [Recherche 01.10.2026: Kein belastbarer Nachweis — Northdata-Suche nach Stefan Schramm/Dreieich ohne passenden Registertreffer (nur namensferne Treffer); keine Betreiber-Website auffindbar; Quelle(n): keine (nur Seed-Verzeichnis). Frontmatter bleibt leer (Klärfall).]

### Recherche 02.10.2026

- [Recherche 02.10.2026: KORREKTUR der Einschätzung vom 01.10.2026 — doch 2 unabhängige Verzeichnisbelege gefunden: tellows.de (Detailseite direkt abgerufen: „Schrott Schramm, Stefan“, Herrnroth 11, 63303 Dreieich, Tel. 06103-322820, gewerbliche Nummer) + cylex.de („Stefan Schramm, Dreieich, Schrotthändler“, Tel. 06103 322820). street/postcode/phone GEFÜLLT, status pruefung → aktiv, website_status unbekannt. Beleglage offen: nur 2 Aggregatoren (kein Register — Northdata weiter ohne Treffer, keine Betreiber-Website); 11880.com und gelbeseiten.de listen ihn im Dreieich-Umkreis NICHT (nur Dietzenbacher Betriebe) — Restunsicherheit bleibt. Quelle(n): tellows.de/num/06103322820, web2.cylex.de.]

### Korrektur 12.10.2026

- [Korrektur 12.10.2026: Re-Audit nach README-Belegstandard. Die Adresse/Telefonnummer in der Recherche vom 02.10. stammen ausschließlich aus Aggregatoren, nicht aus zwei qualifizierenden Quellen; kein passender Betreiber- oder Registerbeleg gefunden. `street`, `postcode`, `phone` geleert und `status` auf `pruefung` zurückgesetzt; alte Werte und ihre Herkunft bleiben im Verlauf sichtbar. Kein Preisbeleg. Quelle(n): https://tellows.de/num/06103322820; https://web2.cylex.de (frühere Verzeichnisbelege; kein qualifizierter Zweitbeleg)]

### Owner-Gate 12.10.2026

- [Korrektur 12.10.2026: Die Produktionsdatenbank enthält noch die frühere Telefonnummer 06103 322820 und den Status `aktiv`; der Abgleich erfolgte read-only. `clear_fields: phone` ergänzt die bereits dokumentierte Rücknahme, damit der alte Aggregator-Kontakt beim nächsten Seed tatsächlich entfernt wird; `status: pruefung` bleibt die Dossierentscheidung. Vorhandene GEO-Koordinaten werden durch das Leeren der Adresse nicht entfernt und müssen separat geprüft werden. Keine Produktionsdaten geändert. Quelle(n): tellows-/Cylex-Leads und Korrektur 12.10.2026; `docs/trader-seed.md`.]

### Recherche 02.11.2026

- [Recherche 02.11.2026: **Zertifikats-Recherche ohne Treffer.** Für `Stefan Schramm (Schrott Schramm)` (Herrnroth 11, 63303 Dreieich) wurde im amtlichen Fachbetrieberegister (GESA) weder ein Efb-Zertifikat noch ein AltfahrzeugV-Eintrag gefunden; `certifications` bleibt leer. Suchen und Befunde (live am 02.11.2026, clientseitig gerenderte Vaadin-App, Ergebnisgrid vollständig durchscrollt): (1) Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Stefan%20Schramm` -> **0 Treffer** (EfB); (2) Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schramm` -> **0 Treffer** (EfB); (3) Namenssuche `…/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=Stefan%20Schramm` -> **0 Treffer** (Altfahrzeug); (4) Namenssuche `…/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=Schramm` -> **0 Treffer** (Altfahrzeug). Die Radiusabfrage mit 3 km um `Herrnroth 11, 63303 Dreieich` (`…/Entsorgungsfachbetriebe/adr=Herrnroth 11, 63303 Dreieich, Deutschland;…&dst=3&is=1&e=1`) liefert **14 Treffer**, u. a. `Container Dienst Dreieich GmbH`, `Daimlerstr. 7, 63303 Dreieich`, `ZZFE00100067 / 1, 2, 3`, `GSA Gesellschaft zur Sanierung von Altlasten mbH`, `Martin-Behaim-Straße 11, 63263 Neu Isenburg`, `ZZPT00600033 / 1, 3`, `Compass Wertstoffmanagement GmbH`, `Werner-Heisenberg-Straße 12, 63263 Neu-Isenburg`, `ZZHT00300401 / 1` - **kein Treffer auf `Schramm` und keiner auf `Herrnroth 11`**. Dieselbe 3-km-Abfrage in der Route `…/Altfahrzeugverwertung/adr=…&dst=3&is=1&o=5&e=1` liefert **0 Treffer** - **kein AltfahrzeugV-Vermerk**. Als Positivkontrolle liefert die 10-km-Efb-Abfrage um dieselbe Adresse **41 Treffer** (u. a. `Container-Service Schiedeck, Inh. Jens Schiedeck`, `Langener Straße 9-11, 63303 Dreieich-Götzenhain`, `ZZFE00100197 / 1`, `Wilms Recycling GmbH`, `Ober-Rodener Straße 56, 63322 Rödermark`, `ZZFT01200052 / 1, 2, 3`), die 10-km-Abfrage in der Altfahrzeugverwertung-Route **0 Treffer** - die Abfragetechnik funktioniert also.
  Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung besitzt.
  Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Stefan%20Schramm ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=Stefan%20Schramm ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schramm ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=Schramm ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Herrnroth%2011%2C%2063303%20Dreieich%2C%20Deutschland%3B8.7127966%3B50.0279878%3B8.724797%3B50.039988%3B8.700797%3B50.015988&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Herrnroth%2011%2C%2063303%20Dreieich%2C%20Deutschland%3B8.7127966%3B50.0279878%3B8.724797%3B50.039988%3B8.700797%3B50.015988&dst=3&is=1&o=5&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Herrnroth%2011%2C%2063303%20Dreieich%2C%20Deutschland%3B8.7127966%3B50.0279878%3B8.724797%3B50.039988%3B8.700797%3B50.015988&dst=10&is=1&a=1&gl=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Herrnroth%2011%2C%2063303%20Dreieich%2C%20Deutschland%3B8.7127966%3B50.0279878%3B8.724797%3B50.039988%3B8.700797%3B50.015988&dst=10&is=1&o=5&e=1 (Abruf 02.11.2026).]
