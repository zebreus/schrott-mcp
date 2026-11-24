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

### Recherche 24.11.2026 (Welle 2000 B)

- [Recherche 24.11.2026: **Die Verzeichnisspur wird durch ein drittes, unabhängiges Verzeichnis gestützt, das die Anschrift, die Rufnummer und die Tageszeiten nennt, aber weiterhin kein Betreiber- oder Registerbeleg ist; das amtliche Fachbetrieberegister kennt den Namen "Schramm" in keiner Route, und ein Vermerk "amtlich anerkannt" in einem Branchenportal erweist sich als nicht belegbar.** Das Branchenportal `gavabiz.de` führt `Schramm Stefan in Dreieich Hessen` im Abschnitt Schrott mit `Herrnroth 11 Dreieich, Dreieich 63303`, `06103 32 28 20`, den Koordinaten `50.0279900, 8.7128400` und den Öffnungszeiten `Mo, Tu, We, Th, Fr 08:30 - 20:00`, `Sa 08:30 - 19:00`; das Verzeichnis `recyclinghof-wertstoffhof.de` (live am 25.11.2026 gelesen) führt `Stefan Schramm`, `Herrnroth 11, 63303 Dreieich`, `Telefon: 06103322820 (+496103322820)` und vermerkt `Angaben laut Google-Eintrag und Webseite des Betriebs (Stand 18.10.2026); "amtlich anerkannt" laut GESA-Fachbetrieberegister der Länder.` **Dieser Vermerk ist nach dem Live-Abgleich nicht haltbar:** die Namenssuchen `Schramm` und `Stefan Schramm` liefern im amtlichen Fachbetrieberegister (live am 25.11.2026, beide Routen, Filter "Art des Betriebs = Egal" (is=1&e=1), Namenssuchen dst=10) in der Route `Entsorgungsfachbetriebe` **je 0 Treffer** und in der Route `Altfahrzeugverwertung` **je 0 Treffer**; die Ortsabfrage um Herrnroth 11, 63303 Dreieich im 3-km-Radius liefert **14** Efb-Zeilen, darunter `Container Dienst Dreieich GmbH`, Daimlerstr. 7, 63303 Dreieich, `ZZFE00100067 / 1`, `/ 2`, `/ 3`, `GSA Gesellschaft zur Sanierung von Altlasten mbH`, Martin-Behaim-Straße 11, 63263 Neu Isenburg, `ZZPT00600033 / 1`, `/ 3`, `Compass Wertstoffmanagement GmbH`, Werner-Heisenberg-Straße 12, 63263 Neu-Isenburg, `ZZHT00300401 / 1` sowie `Knettenbrech + Gurdulic Umweltservice GmbH`, Werner-Heisenberg-Straße 13, 63263 Neu-Isenburg, `ZZHT00300057 / 1` und `/ 2`, die Route `Altfahrzeugverwertung` liefert **0** Treffer - **kein** Treffer nennt `Schramm` oder `Herrnroth 11`. Der Portalvermerk `amtlich anerkannt` ist damit eine **unzutreffende Portalangabe**, kein amtlicher Nachweis; die Efb- und die AltfahrzeugV-Anerkennung sind ohnehin freiwillig, ein Register-Nichteintrag ist **kein** Negativbeweis. Das Suchmaschinenprofil `exa.ai` nennt für `Stefan Schramm` keine gesonderten Daten; die Namenssuche `Stefan Schramm` beim Betreiber-Namensregister ist ergebnislos, die Namenssuche im **amtlichen Registerportal** (live am 25.11.2026) für `Stefan Schramm` liefert **keinen** Treffer zu einem Schrotthandel in Dreieich - Einzelunternehmer werden nicht ins HRA/HRB eingetragen, ein 0-Treffer-Befund ist daher **kein** Nichtexistenzbeweis. **Die Anschrift ist belegt:** Herrnroth 11, 63303 Dreieich steht bei drei voneinander unabhängigen Verzeichnissen (tellows, Cylex, gavabiz) mit identischer Schreibweise, und im OSM existiert das Adressobjekt **node/9948827180** (50.0279878/8.7127966) mit `addr:housenumber`=11, `addr:street`=Herrnroth, `addr:postcode`=63303, `addr:city`=Dreieich, `addr:country`=DE, **ohne** `name=` **ohne** `operator=` **ohne** `shop=` - es ist also **kein** Betriebsobjekt, die Hausnummer 11 ist aber als Adresse real. ANKAUF: keine Kurse und keine Sortenliste; die Öffnungszeiten `08:30 bis 20:00` sind eine Verzeichnisangabe, keine Kondition. Das Mindestgewicht `Annahme von Eisenmetallen ab 100 kg, Nichteisenmetallen ab 1 kg` auf dem Portalprofil ist eine **identische Textbausteine** (Krieger Fulda, J.E. Schrott Hanau, R&W Schrotthändler Wiesbaden und Schrotthändler Sabri Dietzenbach tragen denselben Wortlaut) und wird **nicht** als Ankaufbedingung geführt. VERKAUF: keine Preisliste. GEBÜHREN: keine Tarife. **GEO:** das Adressobjekt node/9948827180 existiert und trägt die Hausnummer, aber kein Betriebs-Tag; es wird **keine** Koordinate gesetzt und das Frontmatter bleibt unverändert (`street` und `postcode` leer, `clear_fields: 'phone'` gesetzt, `status: pruefung`). Die in der Produktionsdatenbank gespeicherte Koordinate 50.02/8.6961 wurde in dieser Recherche **nicht** verändert. **Offen für den Owner:** die belegte Anschrift Herrnroth 11, 63303 Dreieich und die Nummer 06103 322820 sind Kandidaten für eine Frontmatter-Übernahme, und der gespeicherte Produktionspunkt 50.02/8.6961 sollte gegen node/9948827180 geprüft werden; der Portalvermerk `amtlich anerkannt` ist zu korrigieren. Quelle(n): https://dreieich.gavabiz.de/company/schramm-stefan-dreieich.html ; https://recyclinghof-wertstoffhof.de/schrottplatz/hessen/dreieich/ ; https://www.tellows.de/num/06103322820 ; https://web2.cylex.de/firma-home/stefan-schramm-16751609.html (Direktabruf 25.11.2026: HTTP 403) ; https://api.openstreetmap.org/api/0.6/node/9948827180.json ; https://www.online-handelsregister.de/suche/Stefan%20Schramm ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schramm ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Stefan%20Schramm ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Herrnroth%2011%2C%2063303%20Dreieich%2C%20Deutschland%3B8.7127966%3B50.0279878%3B8.724797%3B50.039988%3B8.700797%3B50.015988&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Herrnroth%2011%2C%2063303%20Dreieich%2C%20Deutschland%3B8.7127966%3B50.0279878%3B8.724797%3B50.039988%3B8.700797%3B50.015988&dst=3&is=1&o=5&e=1 (Abruf 25.11.2026).]
