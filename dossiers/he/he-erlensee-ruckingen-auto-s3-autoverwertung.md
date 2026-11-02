---
slug: he-erlensee-ruckingen-auto-s3-autoverwertung
name: AUTO-S3 Autoverwertung
trader_type: autoverwertung
state: HE
city: Erlensee-Rückingen
street: ''
postcode: ''
phone: ''
clear_fields: 'phone'
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Einträge
provenance_ankauf_raw: AV / unklar
provenance_origin: table
certifications: ''
---

# AUTO-S3 Autoverwertung

## Überblick

AUTO-S3 bleibt ein Verzeichnislead für Schubertstr. 2, Erlensee-Rückingen. Die aktuelle Gelbe-Seiten-Suche nennt dieselbe Nummer und einen Website-Hinweis, aber der Detailabruf ist blockiert; kein Impressum oder Betreiber-/Registerbeleg gelesen. Unqualifizierte Straße/Telefon aus Frontmatter zurückgenommen, Historie erhalten. Ankauf, Teileverkauf und Gebühren nicht primär bestätigt.

## Timeline

### Recherche 26.10.2026

- [Recherche 26.10.2026: Die Gelbe-Seiten-Suche nach AUTO-S3 in Erlensee wurde direkt erneut abgerufen. Sie zeigt weiterhin einen passenden Eintrag für Schubertstr. 2, 63526 Erlensee-Rückingen, 06181 2998169 und einen Website-Hinweis; der Suchtreffer blendet dynamisch „Geöffnet – schließt um 19:00“ ein. Das ist eine Portalstatusanzeige, keine bestätigte Betreiber-Öffnungszeit. Der Detailabruf war bereits am 14.10. durch 403/Challenge blockiert; Betreiber und Website-Ziel sind weiterhin nicht lesbar bestätigt. ANKAUF: kein belegter Fahrzeugankauf oder Kurs; VERKAUF: keine Teilepreisliste; GEBÜHREN: keine Tarife. Keine Felder ergänzt, `clear_fields: phone` und `status: pruefung` bleiben. Quelle: https://www.gelbeseiten.de/suche/auto-s3/erlensee (Verzeichnis-Suche, direkt geprüft); https://www.gelbeseiten.de/gsbiz/c2faf292-856c-42d3-ac39-ffae2062a7f8 (Detail-403, Prüfung 14.10.2026).]

### Recherche 14.10.2026

- [Korrektur 14.10.2026: Owner zieht den historischen Telefonwert 061812998169 ausdrücklich mit clear_fields phone zurück: ausschließlich Verzeichnislead, keine qualifizierte aktuelle Betreiberzuordnung. Straße Schubertstr. 2 bleibt als ungeprüfter historischer Lead in der Timeline, nicht als kuratierte Annahmeadresse. Read-only Produktionsprüfung: Straße bislang Schubertstr. 2, GEO 50.163/8.9782. Ein geänderter Seed mit leerer Straße entfernt die Adresse, erhält aber diese ungeprüfte GEO; keine Standortvalidierung oder Koordinatenlöschung behauptet. Prüfstatus beibehalten, separater GEO-Klärfall vor einer späteren Standortfreigabe; kein direkter Produktionswrite. Quelle: Dossier-Historie 01.10.2026 und Recherche 14.10.2026; docs/trader-seed.md; read-only traders-Abfrage 14.10.2026.]

- [Korrektur 14.10.2026: Gelbe-Seiten-Suche live gelesen (200): AUTO-S3 Autoverwertung, Schubertstr. 2, 63526 Erlensee-Rückingen, 06181 2998169, Website-Hinweis. Verlinkter Detailabruf HTTP 403/Challenge; Website-Ziel und Impressum deshalb nicht verifiziert. DuckDuckGo ebenfalls Bot-Challenge. Straße und Telefon waren ausschließlich Verzeichnisangaben (Historie 01.10.); aus Frontmatter geleert, nicht durch ungeprüfte PLZ ergänzt. Leeres phone allein leert vorhandene DB-Anreicherung nicht: Owner muss eine ausdrückliche phone-clear_fields-Entscheidung prüfen; leere Straße wirkt nach docs/trader-seed.md anders, alte GEO werden dadurch nicht validiert. Kein DB-Zugriff/GEO-Eingriff. ANKAUF: Kfz-Ankauf aus Seed nicht primär bestätigt; VERKAUF: keine Teilepreisliste; GEBÜHREN: keine Tarife. Offen: Betreiber, Website-Ziel, aktuelle Annahmestelle. Quelle: https://www.gelbeseiten.de/suche/auto-s3/erlensee ; https://www.gelbeseiten.de/gsbiz/c2faf292-856c-42d3-ac39-ffae2062a7f8 (403); docs/trader-seed.md.]

### Importiert (Seed-Stand 2026-09-30)

- AV – Autoverwertung [Website-Recherche verzeichnis: services: Autoverwertung, Kfz-Ankauf; notes: Gelbe Seiten, 11880 und Seed-Adresse übereinstimmend (Rückingen).]

### Recherche 01.10.2026

- [Recherche 01.10.2026: miss — kein Betreiber-Webauftritt zu AUTO-S3 (Schubertstr. 2, Erlensee-Rückingen) verifiziert; Seed-Tel. 061812998169 und Strasse nur Verzeichnis-Leads ohne Zweitbeleg, unangetastet; Statusfeld unveraendert; Quelle(n): keine Betreiberquelle]

### Recherche 02.11.2026

- [Recherche 02.11.2026: **Zertifikats-Recherche ohne Treffer.** Für `AUTO-S3 Autoverwertung` (Schubertstr. 2, 63526 Erlensee-Rückingen) wurde im amtlichen Fachbetrieberegister (GESA) weder ein Efb-Zertifikat noch ein AltfahrzeugV-Eintrag gefunden; `certifications` bleibt leer. Suchen und Befunde (live am 02.11.2026, clientseitig gerenderte Vaadin-App, Ergebnisgrid vollständig durchscrollt): (1) Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=AUTO-S3` -> **0 Treffer** (EfB); (2) Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Auto%20S3` -> **0 Treffer** (EfB); (3) Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=AUTO%20S3` -> **0 Treffer** (EfB); (4) Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=S3%20Autoverwertung` -> **0 Treffer** (EfB); (5) Namenssuche `…/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=AUTO-S3` -> **0 Treffer** (Altfahrzeug); (6) Namenssuche `…/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=Auto%20S3` -> **0 Treffer** (Altfahrzeug); (7) Namenssuche `…/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=AUTO%20S3` -> **0 Treffer** (Altfahrzeug); (8) Namenssuche `…/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=S3%20Autoverwertung` -> **0 Treffer** (Altfahrzeug). Die Radiusabfrage mit 3 km um `Schubertstr. 2, 63526 Erlensee-Rückingen` (`…/Entsorgungsfachbetriebe/adr=Schubertstr. 2, 63526 Erlensee-Rückingen, Deutschland;…&dst=3&is=1&e=1`) liefert **2 Treffer**: `Hilmar Broich GmbH`, `Dieselstraße 6, 63526 Erlensee`, `ZZFE00100194 / 1`, und `TSW Taunus Saugwagen GmbH`, `Zum Fliegerhorst 1313, 63526 Erlensee`, `ZZFT00300807 / 1` - **kein Treffer auf `AUTO-S3` und keiner auf `Schubertstr. 2`**. Dieselbe 3-km-Abfrage in der Route `…/Altfahrzeugverwertung/adr=…&dst=3&is=1&o=5&e=1` liefert **0 Treffer** - **kein AltfahrzeugV-Vermerk**. Als Positivkontrolle liefert die 10-km-Efb-Abfrage um dieselbe Adresse **32 Treffer** (u. a. `Erich Depping & Sohn GmbH`, `Katharina-Paulus-Straße 1, 65760 Eschborn`, `ZZFE00100298 / 1, 2`, `TSR Deutschland GmbH & Co. KG, Region Südwest I`, `Otto-Röhm-Straße 57, 64293 Darmstadt`, `ZZEE00200162 / 17, 18, 19`), die 10-km-Abfrage in der Altfahrzeugverwertung-Route **0 Treffer** - die Abfragetechnik funktioniert also.
  Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung besitzt.
  Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=AUTO-S3 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=AUTO-S3 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Auto%20S3 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=Auto%20S3 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=AUTO%20S3 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=AUTO%20S3 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=S3%20Autoverwertung ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=S3%20Autoverwertung ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Schubertstr.%202%2C%2063526%20Erlensee-R%C3%BCckingen%2C%20Deutschland%3B8.9859631%3B50.1572817%3B8.997963%3B50.169282%3B8.973963%3B50.145282&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Schubertstr.%202%2C%2063526%20Erlensee-R%C3%BCckingen%2C%20Deutschland%3B8.9859631%3B50.1572817%3B8.997963%3B50.169282%3B8.973963%3B50.145282&dst=3&is=1&o=5&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Schubertstr.%202%2C%2063526%20Erlensee-R%C3%BCckingen%2C%20Deutschland%3B8.9859631%3B50.1572817%3B8.997963%3B50.169282%3B8.973963%3B50.145282&dst=10&is=1&a=1&gl=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Schubertstr.%202%2C%2063526%20Erlensee-R%C3%BCckingen%2C%20Deutschland%3B8.9859631%3B50.1572817%3B8.997963%3B50.169282%3B8.973963%3B50.145282&dst=10&is=1&o=5&e=1 (Abruf 02.11.2026).]
