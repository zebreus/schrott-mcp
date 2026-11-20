---
slug: be-steglitz-zehlendorf-b-m-metalle-berlin
name: B + M Metalle Berlin GmbH
trader_type: sonstige
state: BE
city: Steglitz-Zehlendorf
street: Kaiser-Wilhelm-Straße 88
postcode: '12247'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: 'unbekannt'
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: B. Neu, aber "unsicher" (nur 1 Verzeichnisquelle, keine Website)
provenance_ankauf_raw: ''
provenance_origin: table
---

# B + M Metalle Berlin GmbH

## Überblick

Die B + M Metalle Berlin GmbH (HRB 132091 B) wurde laut dem dokumentierten Registerbefund 2025 von Amts wegen gelöscht. Kaiser-Wilhelm-Straße 88 ist die letzte bekannte Registeranschrift, kein bestätigter heutiger Schrottplatz. Eine Fortführung durch einen anderen Betreiber ist nicht ausgeschlossen, aber bislang nicht belegt; daher `pruefung` statt `aktiv` oder einer Schließungsbehauptung. Keine zuordenbaren Ankaufspreise, Verkaufspreise oder Gebühren belegt.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 13.10.2026

- [Recherche 13.10.2026: NorthData zeigt für B + M Metalle Berlin GmbH, HRB 132091 B, weiterhin die Veröffentlichung vom 19.09.2025 zur Amtslöschung der vermögenslosen Gesellschaft (§ 394 FamFG). Das betrifft den Rechtsträger; eine spätere Nutzung der letzten Registeranschrift Kaiser-Wilhelm-Straße 88 durch einen anderen Händler ist damit weder bestätigt noch ausgeschlossen. Die Adresse im Frontmatter bleibt als historischer Registerstand, nicht als aktiver Annahmeort. Kein aktueller Betreiber oder Fortführer, kein zweiter unabhängiger aktueller Registerbeleg. ANKAUF: keine aktuelle zurechenbare Kursliste; VERKAUF: keine Liste; GEBÜHREN: keine Tarife. GEO für den Händler-/Kundenpunkt bleibt offen. Quelle: https://www.northdata.de/B%20+%20M%20Metalle%20Berlin%20GmbH,%20Berlin/Amtsgericht%20Charlottenburg%20(Berlin)%20HRB%20132091%20B (Registerdatenbank).]

### Importiert (Seed-Stand 2026-09-30)

- nur lokaleschrottplatz; verlinkte Website metallschrott-express.de gehört NRW-Abholer (kein Berlin-Bezug) → Zuordnung zweifelhaft Adresse: Borstellstr. 42, 12167 Berlin
- Bezirk: Steglitz-Zehlendorf Adressbeleg: seed/be.json + recherche/be.md (Verzeichnisbeleg)

### Recherche 01.10.2026

- [Recherche 01.10.2026: Keine Betreiber-Website, kein Registerbeleg gefunden. Einziger Adress-Lead: Borstellstr. 42, 12167 Berlin (lokaleschrottplatz). Seed-Verdacht bestätigt: verlinkte Website metallschrott-express.de gehört NRW-Abholer ohne Berlin-Bezug — keine Zuordnung. Felder bleiben leer, website_status unbekannt, Status ungeklärt; Quelle(n): lokaleschrottplatz.de (nur Lead, Einzelbeleg)]

### Recherche 01.10.2026 (2)

- [Recherche 01.10.2026: Bewiesene Korrektur aus Handelsregister (2+ Anbieter): Northdata (B + M Metalle Berlin GmbH, Berlin, AG Charlottenburg HRB 132091 B) + online-handelsregister.de (aktuelle Geschäftsanschrift c/o Frank Meyer, Kaiser-Wilhelm-Straße 88, 12247 Berlin seit HR-Veränderung 30.05.2017; Gegenstand: Handel mit Metallen und Schrott; Status register-aktiv; Gründung 09.02.2011) + webvalid/companyhouse-Snippets (gleiche Anschrift). Seed-Adresse Borstellstr. 42, 12167 war Registerstand 2013, überholt. Korrekturen: street → Kaiser-Wilhelm-Straße 88, postcode → 12247 (beide Steglitz-Zehlendorf: 12247 = Lankwitz, konsistent); phone-Verbotswert — → leer; status → aktiv (register-aktiv + Schrott-Gegenstand). Keine Website/Telefon/E-Mail belegbar; Quelle(n): Northdata HRB 132091 B + online-handelsregister.de HRB 132091 B + webvalid/companyhouse (Register-Anbieter)]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Re-Check ohne neuen Beleg - 01.10-Vorwellenbefund (HRB 132091 B, Kaiser-Wilhelm-Strasse 88, 12247 Berlin, status aktiv) steht unveraendert und tragfaehig; kein Frontmatter-Fill noetig; Quelle(n): Dossier-Re-Check 02.10.2026 (Welle 25:00)]

### Recherche 05.10.2026

- [Recherche 05.10.2026: Wesentlicher Gegenbeleg zu den Aktivitätsbehauptungen vom 01./02.10.: Die live gelesene Northdata-Seite zu HRB 132091 B kennzeichnet B + M Metalle Berlin GmbH als erloschen und zeigt die Veröffentlichung vom 19.09.2025: „Die vermögenslose Gesellschaft ist auf Grund des § 394 FamFG von Amts wegen gelöscht.“ Zuvor Amtslöschungsverfahren am 14.07.2025. Deshalb ist register-aktiv widerlegt; Korrekturvorschlag für status: geschlossen hinsichtlich dieser gelöschten juristischen Person, beziehungsweise pruefung bis eine mögliche Fortführung durch einen anderen Betreiber geklärt ist. Keine nichtleeren Felder geändert; die letzte Registeranschrift ist kein aktueller Betriebsstandortnachweis; Quelle(n): https://www.northdata.de/B%20+%20M%20Metalle%20Berlin%20GmbH,%20Berlin/Amtsgericht%20Charlottenburg%20(Berlin)%20HRB%20132091%20B]
- [Recherche 05.10.2026: Namensähnliche Berliner Niederlassung separat live geprüft und ausgeschlossen: B+M Baustoff + Metall Handels-GmbH, HRB 28201 Augsburg, Sitz Gersthofen, Niederlassung Marzahner Straße 17B, 13053 Berlin, betreibt Trockenbau-/Baustoffhandel. Andere Registeridentität, Anschrift und Tätigkeit; weder deren Website noch deren Kontakte/Zeiten übernehmen. Die im Seed bereits ausgeschlossene NRW-Domain bleibt ebenfalls unzugeordnet. Kein eigener aktueller Betreiberauftritt des Dossiers identifiziert, somit keine Händler-Leistungs-/Preisunterseiten belegbar; Quelle(n): https://www.baustoff-metall.de/niederlassung-berlin/]
- [Recherche 05.10.2026: Beleglage und Grenzen: Löschung derzeit ein qualifizierter Registeranbieter, nicht zwei unabhängige Bestätigungen. Der separat abgerufene Northdata-Veröffentlichungslink war öffentlich nur eingeschränkt lesbar; der zusätzlich versuchte Online-Handelsregister-Link lieferte HTTP 404. Ergänzende Websuche scheiterte im Verlauf mit HTTP 401. Alle leeren Felder bleiben leer, website_status unbekannt bleibt erhalten. Offen: amtlicher aktueller Auszug und mögliche Betreiber-/Fortführungskette an Kaiser-Wilhelm-Straße 88 oder Borstellstraße 42; keine aktuelle Ankauf-, Verkaufs- oder Dienstleistungspreisliste gefunden; Quelle(n): https://www.northdata.de/?id=4535526668107776 ; https://www.online-handelsregister.de/handelsregisterauszug/be/Berlin-Charlottenburg/HRB/132091/B-M-Metalle-Berlin-GmbH]
- [Korrektur 07.10.2026: Der im Recherchevermerk 05.10.2026 dokumentierte Löschbefund wird nun in `status: pruefung` abgebildet. Die vorhandene Evidenz betrifft den Rechtsträger B + M Metalle Berlin GmbH, nicht zwingend jede spätere Nutzung der Anschrift; deshalb keine Behauptung, dass ein möglicher Nachfolgebetrieb geschlossen ist. Offen bleiben ein amtlicher aktueller Registerauszug und die Betreiberkette am letzten Registerstandort. Preisprüfung: ANKAUF — keine aktuelle, zurechenbare Liste; VERKAUF — keine Liste; GEBÜHREN — keine Tarife. Quelle(n): https://www.northdata.de/B%20+%20M%20Metalle%20Berlin%20GmbH,%20Berlin/Amtsgericht%20Charlottenburg%20(Berlin)%20HRB%20132091%20B ; https://www.baustoff-metall.de/niederlassung-berlin/ (separater, ausgeschlossener Betrieb)]

### Recherche 20.11.2026 (Welle 1800 C)

- [Recherche 20.11.2026: **Die Löschung des Rechtsträgers ist über das amtliche Handelsregister auffindbar; das amtliche Fachbetrieberegister kennt am Standort Kaiser-Wilhelm-Straße 88 keinen zertifizierten Betrieb.** Die Suche im amtlichen Handelsregister (online-handelsregister.de) unter „B + M Metalle Berlin“ liefert am 20.11.2026 den Eintrag **B + M Metalle Berlin, Amtsgericht Charlottenburg (Berlin), HRB 132091** - der Datensatz bleibt also im Register auffindbar, obwohl der Rechtsträger amtslöscht ist; die Löschungsveröffentlichung vom 19.09.2025 (§ 394 FamFG, Amtslöschungsverfahren vom 14.07.2025) und der Firmenzweck „Handel mit Metallen und Schrott“ sind bereits im Dossier dokumentiert. Im amtlichen Fachbetrieberegister liefert die Namenssuche am 20.11.2026 in der Route „Altfahrzeugverwertung“ (`.../Altfahrzeugverwertung/dst=10&is=1&e=1&n=B%20%2B%20M%20Metalle`) **8 Zeilen**, sämtlich fremde Rechtsträger (Zimmer Schrott- und Metallhandels GmbH, Grubenstraße 10 und Alleestraße 6, 50354 Hürth, ZZET01600180; BMB BuntMetalleBraun, Oberer Ziegeleiweg 5, 87766 Memmingerberg, ZZET00700283; Kilb Vetter Entsorgung GmbH, Zeilsheimer Weg 4, 65779 Kelkheim, ZZHT00400195); die Radiusabfrage mit 3 km um die Dossier-Adresse liefert in der Route „Altfahrzeugverwertung“ **1 Zeile** (Bokpe Smart-Spezialist Inh. Dr. h.c. Prince D‘Allada Bokpe, Dederingstrasse 21, 12107 Berlin, ZALS00800048, 1,9 km) und in der Route „Entsorgungsfachbetriebe“ **18 Zeilen** (u. a. B. & F. Brockschmidt GmbH, Siemensstraße 17, 12247 Berlin; Fritz Pennecke Söhne, Greinerstraße 27, 12107 Berlin) - **keine Zeile nennt die Kaiser-Wilhelm-Straße 88**. Quelle(n): https://www.online-handelsregister.de/suche/B%20%2B%20M%20Metalle%20Berlin ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=B%20%2B%20M%20Metalle (Abruf 20.11.2026).]
- [Recherche 20.11.2026: **Die Seed-Anschrift ist in OSM nur als Hausadresse erfasst; ein heutiger Schrottbetrieb ist dort nicht belegt.** Das Adressobjekt **node/2738936510** (52.4373134/13.3465818) trägt `addr:street=Kaiser-Wilhelm-Straße`, `addr:housenumber=88`, `addr:postcode=12247`, `addr:city=Berlin`, `addr:suburb=Lankwitz` sowie **kein** `name=`, **kein** `operator=` und **keinen** Gewerbe-, Recycling- oder Scrap-Tag. Die amtliche Registerabfrage belegt dort ebenfalls keinen zertifizierten Betrieb; die belegte Löschung betrifft den Rechtsträger, nicht die Anschrift. **ANKAUF:** keine aktuelle, zurechenbare Kursliste belegt. **VERKAUF:** keine Liste belegt. **GEBÜHREN:** keine Tarife belegt. `status` bleibt `pruefung`, weil eine Fortführung durch einen anderen Betreiber weder belegt noch ausgeschlossen ist. Quelle(n): https://api.openstreetmap.org/api/0.6/node/2738936510.json (Abruf 20.11.2026).]
