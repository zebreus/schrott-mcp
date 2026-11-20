---
slug: be-neukolln-psiorz-inh-marius-maciossek
name: Psiorz (Inh. Marius Maciossek)
trader_type: sonstige
state: BE
city: Neukölln
street: Rotkehlchenweg 2
postcode: '12351'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: unbekannt
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: B. Neu, aber "unsicher" (nur 1 Verzeichnisquelle, keine Website)
provenance_ankauf_raw: ''
provenance_origin: table
---

# Psiorz (Inh. Marius Maciossek)

## Überblick

Neue direkte Webspur: psiorz-dienstleistungen.de nennt Psiorz Dienstleistungen, Kornblumenring 8b, 12357 Berlin, Entrümpelungen/Sperrmüll-/Gartenabfuhr und eine Marius-Maciossek-Mailadresse. Das ist nicht die zuvor allein geprüfte Parkdomain psiorz.de. Die Seite hat kein auswertbares Betreiberimpressum und beweist weder eine Verlagerung von Rotkehlchenweg 2 noch heutigen Schrottankauf; ein neuer Portalname Clemens Maciossek steht zusätzlich im Raum. Keine Kontakte oder Adresse übertragen. **Ankauf / Verkauf / Gebühren:** keine Zahlen; **GEO:** beide Adressspuren erst rechtlich und betrieblich verbinden, kein Punkt-Fill.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 18.10.2026 (18:00-Welle, Shard 1)

- [Recherche 18.10.2026: Eigene Namenssuche entdeckt andere Domain http://www.psiorz-dienstleistungen.de/; vollständig direkt gelesen, HTTP 200: Psiorz Dienstleistungen, Entrümpelungen/Sperrmüllabfuhr/Gartenabfälle, Kornblumenring 8b/12357 Berlin, 0177 3250539, 030 66509077, Fax 030 66509078, marius.maciossek@freenet.de. Keine weiteren Inhalts-/Impressumlinks im HTML. Mail und mit altem Portal kongruente Festnetznummer sind Zuordnungsindizien, kein unabhängiger Nachweis einer Inhaber- oder Standortkette. Cylex-Suchlead nennt stattdessen Psiorz Inhaber Clemens Maciossek; nicht als bewiesene Nachfolge behandeln. Register-Suggest Marius Maciossek HTTP 429, keine negative Registersuche; Berliner Gewerbeauskunft technisch Timeout, kein Ergebnisdatensatz. Keine Umgehung der Sperren. Keine Preisangaben auf der gelesenen Einseitenpräsenz: Ankauf nicht beworben, Verkauf keine Liste, Gebühren der Abfuhr nicht beziffert. status unbekannt und Frontmatter bleiben, keine GEO-/Umzugsbehauptung. Quellen: http://www.psiorz-dienstleistungen.de/ ; https://web2.cylex.de/firma-home/psiorz-inhaber-clemens-maciossek-15257303.html (nur Suchlead) ; https://www.northdata.de/suggest.json?query=Marius%20Maciossek&countries=DE (429) ; https://gewerbeauskunft.berlin.de/ (Timeout).]

### Importiert (Seed-Stand 2026-09-30)

- nur schrottplatz-info (alt) Adresse: Rotkehlchenweg 2, 12351 Berlin
- Bezirk: Neukölln Adressbeleg: nur schrottplatz-info alt (be.md Runde 2); psiorz.de tot (Runde 4)

### Recherche 01.10.2026

- [Recherche 01.10.2026: psiorz.de liefert nur Plesk-Parkseite (http 200 mit Refresh auf defaultsite, https ohne Antwort), daher website_status tot nicht belegbar ohne Zweitbeleg und kein Website-Fill; phone Platzhalter bereinigt; Quelle(n): Direktabruf http://psiorz.de und https://psiorz.de 01.10.2026]
- [Recherche 01.10.2026: Kein HR-Treffer zu Psiorz Maciossek; Seed-Adresse nur Einzelbeleg aus schrottplatz-info, kein Zweitbeleg, daher kein Fill und Klaerfall; Quelle(n): Northdata-Pfadsuche 01.10.2026]

### Recherche 04.10.2026

- [Recherche 04.10.2026: Der Direktabruf von http://psiorz.de/ liefert weiterhin nur eine leere Weiterleitung auf „defaultsite“ (Last-Modified 2012); HTTPS scheitert an der Zertifikatsprüfung. Aktuelle Treffer für Psiorz/Marius Maciossek/Rotkehlchenweg stammen weiterhin nur aus Branchen- und Schrottplatzverzeichnissen, nicht aus Betreiber-, Register- oder kommunalen Quellen. Website-Status und Inhaberschaft bleiben deshalb ungeklärt; keine Frontmatter-Füllung, vorhandene Adresse unverändert; Quelle(n): http://psiorz.de/; https://www.schrottplatz.org/berlin/schrott-psiorz-inh-marius-maciossek-aYEPRF.html; https://www.schrottplatz-info.de/schrottplatz/Berlin/Psiorz-Inh-Marius-Maciossek; https://bazaaar.de/anbieter/firma-psiorz-inh-marius-maciossek/; https://www.upa-online.de/berlin-4fc1cdf700.html (Aggregator-Leads)]

### Recherche 20.11.2026 (Welle 0000 C)

- [Recherche 20.11.2026: **Das amtliche Fachbetrieberegister kennt weder Psiorz noch Maciossek; die Nichttreffer sind ein Hinweis ohne Beweiswert.** Abfragen am 20.11.2026 über das **gesamte** Register: Route `Entsorgungsfachbetriebe` mit `n=Psiorz` → **0 Treffer**, mit `n=Maciossek` → **0 Treffer**; Standortsuche 3 km um Rotkehlchenweg 2, 12351 Berlin (52.4395881/13.4536838) mit `n=Psiorz` → **0 Treffer** und mit `n=Maciossek` → **0 Treffer**; Standortsuche 3 km um Kornblumenring 8b, 12357 Berlin (52.4253294/13.4923976) mit `n=Psiorz` → **0 Treffer**. **Reichweite:** Efb- und Afv-Anerkennung sind freiwillig; ein Register-Nichteintrag ist **kein** Nichtexistenz- und **kein** Schließungsbeleg. Quellen: https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Psiorz ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Maciossek (Abruf 20.11.2026).]
- [Recherche 20.11.2026: **Die eigene Betreiberseite `psiorz-dienstleistungen.de` ist live und nennt die Anschrift Kornblumenring 8b, 12357 Berlin — die Dossier-Adresse Rotkehlchenweg 2 bleibt davon getrennt.** Die Seite wurde am 20.11.2026 vollständig gelesen (HTTP 200, 1376 Bytes) und zeigt `Psiorz Dienstleistungen`, `Entrümpelungen / Sperrmüllabfuhr / Gartenabfälle`, Kornblumenring 8b, 12357 Berlin, `+49 (0)177 3250539`, `+49 (0)30 66509077`, `marius.maciossek@freenet.de`; ein Impressum und ein Betreiber-Register fehlen weiterhin. OSM belegt **beide** Adressen hausnummergenau: way/311251625 (`addr:street`=Rotkehlchenweg, `addr:housenumber`=2, `addr:postcode`=12351) und node/2821391787 (`addr:street`=Kornblumenring, `addr:housenumber`=8B, `addr:postcode`=12357) — beide ohne `name=` und ohne `operator=`, also **Adressobjekte, keine Betriebsobjekte**. Es wird **keine** der beiden Adressen in das Frontmatter übernommen und **kein** Punkt gesetzt: die beiden Spuren sind weiterhin nicht rechtlich und betrieblich verbunden. Quellen: http://www.psiorz-dienstleistungen.de/ ; https://api.openstreetmap.org/api/0.6/node/2821391787.json ; https://api.openstreetmap.org/api/0.6/way/311251625/full.json (Abruf 20.11.2026).]
- [Recherche 20.11.2026: **GEO gemessen, nicht geändert.** Die read-only Produktionszeile (20.11.2026) führt 52.4395881/13.4536838; die Rückauflösung liefert `2, Rotkehlchenweg, Buckow 2, Buckow, Neukölln, Berlin, 12351` — das ist **exakt** die Seed-Adresse, der Punkt ist damit **richtig** und wird nicht angetastet. **ANKAUF:** die gelesene Betreiberseite bewirbt Entrümpelung, Sperrmüllabfuhr und Gartenabfälle, aber **keinen** Schrottankauf, **keine** Sorten und **keine** Kurse; **VERKAUF:** keine Verkaufsliste; **GEBÜHREN:** keine Abhol- oder Entsorgungstarife und keine Zusage einer kostenlosen Annahme. Quellen: read-only `public.db` (20.11.2026); https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=52.4395881&lon=13.4536838&zoom=18&addressdetails=1 (Abruf 20.11.2026).]
