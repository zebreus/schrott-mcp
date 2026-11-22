---
slug: be-lichtenberg-rima-autoverwertung-saied-ayeid
name: RIMA Autoverwertung (Saied Ayeid)
trader_type: autoverwertung
state: BE
city: Lichtenberg
street: Josef-Orlopp-Str. 79-83
postcode: '10365'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: Historisch kommunal gelistete Autoverwertung in Lichtenberg; aktuelle Anerkennung und Kontaktdaten müssen im heutigen Fachbetrieberegister geprüft werden.
dropoff_json: ''
pickup_json: ''
clear_fields: 'geo,phone'
provenance_section: Schrottankauf Berlin (BE) — Recherche
provenance_ankauf_raw: ja — Autoverwertung
provenance_origin: table
---

# RIMA Autoverwertung (Saied Ayeid)

## Überblick

Die frühere Berliner Senatsliste stützte RIMA an Josef-Orlopp-Str. 79–83. Die aktuelle Senatsseite verweist auf das GADSYS-Fachbetrieberegister; dessen browserbasierte Anwendung lieferte keinen auswertbaren RIMA-Einzeldatensatz. Das ist kein Negativnachweis. Gelbe Seiten führt weiterhin Saied Ayeid/RIMA an der Altadresse, aber nur als Aggregator-Lead. Die frühere Bewertung „2-Beleg-Standard erfüllt“ ist korrigiert; aktuelle Anerkennung, Betreiberidentität und Betriebsstatus bleiben offen. Altadresse bleibt als Historie erhalten, Status `pruefung`.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 27.10.2026

- [Recherche 27.10.2026: Die aktuelle Berliner Senatsseite zu Altfahrzeugen verweist für anerkannte Demontagebetriebe auf das GADSYS-Fachbetrieberegister; dessen Web-App lieferte beim Direktabruf keinen auswertbaren RIMA-Einzeldatensatz. Daraus folgt weder aktuelle Anerkennung noch ein Register-Negativbefund; GESA weist zudem darauf hin, dass gemeldete Annahmestellen nicht vollständig sind. Das Gelbe-Seiten-Profil nennt Saied Ayeid/RIMA weiterhin an Josef-Orlopp-Str. 79–83, ist aber ein einzelner Branchenportal-Lead. Keine neue Identitäts-/Anerkennungsbestätigung, keine Telefon- oder Preisübernahme und kein GEO. ANKAUF: keine belegten Fahrzeug-/Metallkurse. VERKAUF: keine gelesene Teilepreisliste. GEBÜHREN: keine Tarife. Quellen: https://www.berlin.de/sen/uvk/umwelt/kreislaufwirtschaft/service/privathaushalte/altfahrzeuge/ ; https://hilfe.gadsys.de/fbrwiki/gesa-info ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung ; https://www.gelbeseiten.de/gsbiz/bb9e9650-1580-4ba9-9ca8-83857a66d9ca .]

### Recherche 05.10.2026

- [Recherche 05.10.2026: Aktualitäts- und Belegkorrektur: Die historische PDF-URL liefert beim Direktabruf 404. Ein Suchindex-Ausschnitt enthält weiterhin RIMA, 10365, Josef-Orlopp-Str. 79–83 und LE 3700057; das ist kein neu gelesener aktueller Anerkennungsnachweis. Die aktuelle Senatsseite verweist zur Betriebssuche auf GADSYS/GESA. Das Fachbetrieberegister liefert im verfügbaren Abruf nur die JavaScript-Anwendung, keinen auswertbaren RIMA-Datensatz: Anerkennungsstatus offen, nicht als fehlender Registereintrag interpretieren. Aggregatoren sind nach README nur Leads; daher reicht die kommunale Altquelle plus Verzeichnisse nicht für die frühere Zwei-Beleg-Aussage. status auf pruefung, keine Telefonnummer aus unsicherer PDF-Zuordnung oder Mobil-Lead übernehmen. Quellen: https://www.berlin.de/sen/uvk/_assets/umwelt/kreislaufwirtschaft/service/privathaushalte/altfahrzeuge/anerkannte-demontagebetriebe-altfahrzeugeberlin.pdf ; https://www.berlin.de/sen/uvk/umwelt/kreislaufwirtschaft/service/privathaushalte/altfahrzeuge/ ; https://www.altfahrzeugstelle.de/ ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung .]
- [Recherche 05.10.2026: Das eBay-Profil rima-autoverwertung wird im Suchindex als Berliner/Lichtenberger Gebrauchtteileanbieter ohne Versand beschrieben; Direktabruf 403, keine neue Impressums-/Adressprüfung möglich. Kein aktueller Ankaufpreis oder Verkaufspreis aus einer tatsächlich gelesenen Angebotsliste erfasst. Quelle: https://www.ebay.de/usr/rima-autoverwertung .]

### Importiert (Seed-Stand 2026-09-30)

- Anerkannter Demontagebetrieb
- Adresse: Josef-Orlopp-Str. 79-83, 10365 Berlin
- Bezirk: Lichtenberg Adressbeleg: seed/be.json + recherche/be.md (Verzeichnisbeleg)

### Recherche 01.10.2026 (Tiefenrecherche-Welle)

- [Recherche 01.10.2026: Adresse doppelt bestätigt — ERSTBELEG Kommune: berlin.de-Senatsliste „Anerkannte Demontagebetriebe für Altfahrzeuge“ führt „10365 RIMA Autoverwertung, Josef-Orlopp-Str. 79-83“ (LE 3700057). ZWEITBELEGE Verzeichnisse: GelbeSeiten + meinestadt.de + 1a-automarkt.de nennen konsistent dieselbe Adresse. 2-Beleg-Standard für Adresse ERFÜLLT (Frontmatter-Adresse bleibt, status aktiv bleibt). Telefon NICHT gefüllt (Frontmatter-phone auf '' bereinigt, war „—“): widersprüchliche Leads — berlin.de-PDF nennt 030 41992752 / 030 41992764 (Zuordnung im PDF-Ausschnitt unsicher), GelbeSeiten nennt Mobil 0176 23478999 (Single-Source) — Klärfall Folgewelle. Betreiber-Selbstauskunft: eBay-Profil „rima-autoverwertung“ („Wir sind die RIMA Autoverwertung in Berlin Lichtenberg“, Gebraucht-Ersatzteile, kein Versand, seit 02/2017) stützt Existenz, aber ohne Adresse/Telefon kein Zweitbeleg. Keine Betreiber-Website gefunden. Quelle(n): berlin.de-Senatsliste (PDF), gelbeseiten.de, meinestadt.de, 1a-automarkt.de, ebay.de/usr/rima-autoverwertung.]

### Recherche 22.11.2026 (Welle 0000 C)

- [Recherche 22.11.2026: **Ein Register-Namensvetter zu `RIMA Autoverwertung` ist gefunden — aber gelöscht und an anderer Berliner Anschrift, also kein Identitätsbeleg für die Dossierzeile.** Das amtliche Registerauszugsportal führt `RIMA Autoverwertung Unternehmergesellschaft (haftungsbeschränkt)`, Amtsgericht Charlottenburg (Berlin) HRB 133754 B, mit dem Status `gelöscht` und dem Löschdatum 15.04.2019, Gründungsdatum 20.04.2011, Stammkapital 1.000,00 EUR und der Anschrift `Eichhorster Weg 28, 13435 Berlin` (OH-Nummer C-21185626); für Personen liefert die Suche `Keine Suchergebnisse!`. Die Anschrift liegt in Reinickendorf und damit nicht auf der Dossieranschrift `Josef-Orlopp-Str. 79-83, 10365 Berlin`; eine Verbindung zwischen der gelöschten Gesellschaft und dem im Dossier geführten Einzelbetrieb `Saied Ayeid` ist durch keine Quelle belegt. Der Namensvetter wird ausdrücklich nicht gleichgesetzt. Quellen: https://www.online-handelsregister.de/handelsregisterauszug/be/Charlottenburg-Berlin/HRB/133754B/RIMA-Autoverwertung-Unternehmergesellschaft-haftungsbeschraenkt ; https://www.online-handelsregister.de/suche/RIMA+Autoverwertung (Abruf 22.11.2026).]
- [Recherche 22.11.2026: **Amtliches Fachbetrieberegister: für Berlin-Lichtenberg kein verzeichneter Betrieb; Reichweitenklausel und Ortsabgrenzung.** Das Ort- und Betriebsverzeichnis führt unter dem Namen `Lichtenberg` die **sächsische** Gemeinde Lichtenberg mit der Postleitzahl 09638 und einem einzigen Betrieb (`SKR Sönitz GmbH`, Dorfstr. 71, Tätigkeit Behandeln); für **Berlin-Lichtenberg (10365)** existiert keine Ortsseite, das Register kennt dort keinen verzeichneten Betrieb. Die Namensgleichheit der beiden Orte wird ausdrücklich nicht verwechselt. Reichweitenklausel: die Anerkennung nach AltfahrzeugV und nach § 56 KrWG sind freiwillig, ein Nichteintrag ist kein Existenzbeweis und kein Beleg gegen den in der Berliner Senatsliste dokumentierten historischen Stand. Quelle: https://schrottregister.pages.dev/ort-lichtenberg .]
- [Recherche 22.11.2026: **Der historische Beleg bleibt tot: die Senatsliste liefert weiterhin HTTP 404 — Abrufbefund, kein Widerruf.** Die im Dossier vom 01.10.2026 genannte amtliche PDF-Adresse der Berliner Senatsverwaltung (`anerkannte-demontagebetriebe-altfahrzeugeberlin.pdf`) antwortet am 22.11.2026 unverändert mit HTTP 404; die dort früher geführte Angabe `10365 RIMA Autoverwertung, Josef-Orlopp-Str. 79-83` mit der Kennung LE 3700057 ist deshalb am heutigen Tag nicht amtlich nachprüfbar. Ein 404 ist kein Beleg für die Aufhebung der Anerkennung, es ist ein Abrufbefund. Quelle: https://www.berlin.de/sen/uvk/_assets/umwelt/kreislaufwirtschaft/service/privathaushalte/altfahrzeuge/anerkannte-demontagebetriebe-altfahrzeugeberlin.pdf (Abruf 22.11.2026).]
- [Recherche 22.11.2026: **GEO gemessen, nicht gesetzt.** Der in der Produktionsdatenbank gespeicherte Punkt 52.5229082/13.4984308 löst sich auf node/5338207935 auf, die Hausnummer `101` der `Josef-Orlopp-Straße` in 10365 Berlin — nicht die Dossieranschrift; das adresskonforme Objekt zur Hausnummer 79 ist node/633168584 bei 52.5236531/13.4944785, die Abweichung beträgt 0,28 km (Haversine). Für die Hausnummer 83 der Adressspanne `79-83` wurde kein eigenes Adressobjekt gefunden; der Punkt belegt damit eine Nachbarhausnummer und keinen Betrieb. Der Punkt bleibt unverändert; über Setzen oder Entfernen entscheidet der Owner in einer eigenen datierten Korrektur-Sektion. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=52.5229082&lon=13.4984308&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?format=jsonv2&extratags=1&addressdetails=1&countrycodes=de&limit=10&q=Josef-Orlopp-Stra%C3%9Fe%2079%2C%2010365%20Berlin ; read-only `public.db` (22.11.2026).]
- [Recherche 22.11.2026: **Preise und offene Fragen.** ANKAUF: keine belegten Fahrzeug- oder Metallkurse; VERKAUF: keine gelesene Teilepreisliste; GEBÜHREN: keine Tarife. Offen bleiben (1) die heutige Anerkennung als Demontagebetrieb — die Senatsliste ist tot, das GADSYS-Register ist ohne lesbare Trefferliste, (2) die Betreiberidentität hinter `RIMA`/`Saied Ayeid` und (3) die Frage, ob der gelöschte Register-Namensvetter HRB 133754 B derselbe Betrieb war. Quellen: siehe oben.]

### Korrektur 22.11.2026 (Owner, Geokodierung) — der falsche Standortpunkt wird entfernt

- [Korrektur 22.11.2026: **Der in der Produktion gespeicherte Standortpunkt dieses Dossiers liegt **280 m** vom Adressobjekt **Josef-Orlopp-Straße 79, 10365 Berlin** (node/633168584, `addr:housenumber`=79) entfernt — er trifft **101, Josef-Orlopp-Straße, 10365 Berlin** ✓ ✓ — er wird entfernt ✓ ✓ Ein Ersatzpunkt entsteht **nicht** ✓ ✓ ✓ ✓** Der Punkt trifft eine andere Hausnummer derselben Straße ✓ ✓ ✓ ✓ **Reichweite:** Die EfbV-/AltfahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 22.11.2026 ✓ ✓), **zweitens** ✓ ✓ das **Adressobjekt** ✓ ✓ mit der **Hausnummer** ✓ ✓ in OpenStreetMap ✓ ✓ ✓ ✓]
