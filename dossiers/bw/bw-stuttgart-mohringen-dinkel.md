---
slug: bw-stuttgart-mohringen-dinkel
name: Dinkel
trader_type: sonstige
state: BW
city: Stuttgart-Möhringen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
clear_fields: 'description,dropoff_json,email,geo,opening_hours,phone,website,website_status'
dropoff_json: ''
pickup_json: ''
provenance_section: Neueinträge ohne Website (Register/Verzeichnis, Handeln-Flag)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Dinkel

## Überblick

Der aktuelle Betrieb **Dinkel Recycling GmbH** (HRB 10644) und sein Standort Epplestraße 182, Stuttgart-Degerloch, sind durch Betreiber- und Registerquellen belegt. Die Verbindung dieses Betriebs mit der separaten Seed-Zeile „Dinkel“ in Stuttgart-Möhringen beruht dagegen nur auf einem Branchenverzeichnis-Lead und ist nicht ausreichend bestätigt. Die Dossierzeile bleibt daher auf dem importierten Namen/Ort und `status: pruefung`; Anschrift, Betreiberkontakt, Website, Aktivstatus und Annahmebedingungen der GmbH werden nicht auf die Altzeile übertragen.

**Nicht übertragene Kandidateninformationen:** Die Dinkel-Recycling-Website beschreibt Schrott-/Metallverwertung, Selbstanlieferung und Containerleistungen am Degerlocher Standort; weder diese Leistungen noch dortige Preise sind für die separate Möhringer Seed-Zeile bestätigt. Für den vorliegenden Datensatz werden deshalb keine Annahme-, Ankauf-, Verkaufs- oder Gebührenangaben behauptet. Der alte Produktionspunkt ist als Fehlzuordnung erkannt und wird explizit zurückgenommen; es wird kein Standort geokodiert.

## Timeline

### Owner-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Der Read-only-Produktionsabgleich zeigt für diesen Slug weiterhin die Altzeile `Dinkel` / Stuttgart-Möhringen / `pruefung`, jedoch den Kartenpunkt 48.7258/9.1464 an Hechinger Straße 5. Betreiberwebsite und NorthData HRB 10644 belegen Dinkel Recycling GmbH und deren Standort Epplestraße 182 in Degerloch, aber nicht die Identität dieser Möhringer Seed-Zeile; der einzige Zuordnungsweg ist ein Verzeichnis-Lead. Deshalb werden die zuvor auf die Altzeile übertragenen GmbH-Stammdaten, Aktivstatus und Annahmebehauptung zurückgenommen. `clear_fields: 'description,dropoff_json,email,geo,opening_hours,phone,website,website_status'` löscht auch den gespeicherten `allowed:true`-Claim und den Punkt; keine Ersatzkoordinate. Der ursprüngliche Name/Ort und `pruefung` bleiben erhalten. Quellen: https://dinkel-gmbh.de/stuttgart-degerloch.html ; https://www.northdata.de/Dinkel%20Recycling%20GmbH,%20Stuttgart/HRB%2010644 ; https://stuttgart.city-map.de/en/dinkel-gmbh-entsorgungsfachbetrieb (nur Zuordnungslead) ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=48.7258&lon=9.1464&zoom=18&addressdetails=1 ; README.md, Quellenhierarchie und Betreiberkettenregel.]

### Owner-GEO-Prüfung 21.10.2026

- [Owner-GEO-Prüfung 21.10.2026: Die Betreiberseite bestätigt Selbstanlieferung von Schrott und Metall an Epplestraße 182, 70567 Stuttgart. Der vorhandene Produktionspunkt löst dagegen auf Hechinger Straße 5 auf. Epplestraße 182 bleibt ein belastbarer Standortkandidat, aber der genaue Eingang ist nicht punktgenau belegt; kein Ersatz-GEO ergänzt. Quellen: [Betreiberseite](https://dinkel-gmbh.de/stuttgart-degerloch.html), [Reverse-Abfrage](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=48.7258&lon=9.1464&zoom=18&addressdetails=1), [OSM-Objekt am Produktionspunkt](https://www.openstreetmap.org/node/3127571980), [Owner-Bericht](../../research/wave-20261021-0900-geo-bw.md).]

### Recherche 21.10.2026

- [Recherche 21.10.2026: Den seednahen Eintrag „Dinkel“ mit der aktuellen Dinkel Recycling GmbH abgeglichen. Die Betreiberseiten nennen Epplestraße 182, 70567 Stuttgart-Degerloch, Telefon 0711 453218, info@dinkel-gmbh.de, Anlieferung Mo-Fr 07:00-16:30 und Sa 08:00-12:00 sowie Selbstanlieferung von Schrott/Metall durch Privat- und Gewerbekunden. NorthData führt Amtsgericht Stuttgart HRB 10644, Adresse Epplestr. 182 und die Registerpublikation vom 26.03.2026 zur Umbenennung aus Dinkel GmbH Müll- und Schuttabfuhr; damit sind Firmenname, Betreiberadresse und aktuelle Existenz unabhängig vom Betreiberauftritt gestützt. City-map ordnet den Dinkel-Eintrag dem Suchbereich Möhringen zu, bleibt aber ein Verzeichnislead; für die strukturierte Ortsangabe folgt die konkrete Betreiberbezeichnung Stuttgart-Degerloch. Ankauf: keine Auszahlung/Kurse auf den geprüften Seiten belegt; Verkauf: keine Metallverkaufspreise; Gebühren: Container-Shop benötigt eine PLZ und verweist bei nicht abgedeckten Gebieten auf individuelles Angebot; AGB verweisen auf vereinbarte Preise/Preisliste, ohne dort Zahlen zu veröffentlichen. Quellen: https://dinkel-gmbh.de/impressum.html ; https://dinkel-gmbh.de/kontakt.html ; https://dinkel-gmbh.de/stuttgart-degerloch.html ; https://shop.dinkel-gmbh.de ; https://dinkel-gmbh.de/dl/2022-DINKEL-AGB.pdf ; https://www.northdata.de/Dinkel%20Recycling%20GmbH,%20Stuttgart/HRB%2010644 ; Zuordnungslead: https://stuttgart.city-map.de/en/dinkel-gmbh-entsorgungsfachbetrieb .]

### Importiert (Seed-Stand 2026-09-30)

- Tätigkeit unklar
- Adresse: Stuttgart-Möhringen

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Zwei voneinander unabhängige Gründe: (a) die
  **Identität der Dossierzeile** ist ungeklärt, (b) die **Hausnummer 182** fehlt in OSM. Quelle 1 (OSM, live geprüft am
  04.11.2026): das am 04.11.2026 live abgerufene Kachelraster umfasst **9.1500/48.7300 bis 9.1700/48.7700**, **8 Kacheln
  von je 0,01 Grad**, **0 Kachel-Fehler**; das Fenster enthält die gesuchte Straße **nachweislich** mit **30**
  Straßenobjekten auf dem „Epplestraße" (u. a. way/158936268, way/28566787, way/46610723, way/155576221, way/34619378,
  way/100468020, way/100468024, way/100468029, way/563805817, way/469935863, way/557625072; Verlauf 48.72965/9.16711 bis
  48.74931/9.17065) sowie **88** Adressobjekten auf „Epplestraße" mit **72** verschiedenen Hausnummern (2 bis 91, 151,
  178, 180, 181, 185, 192, 200 sowie die Zusatznummern 1A, 1B, 1a, 20A, 32A, 39A, 39B, 39C, 40-44, 48A, 56A, 56B, 90-96)
  - **die Dossier-Hausnummer 182 existiert dort nicht**, obwohl die unmittelbaren Nachbarnummern 178 (**way/246643394**
  „Häckselplatz Möhringen"), 180 (**way/233903194**), 181 (**way/30098179** Hockeyhalle HTC), 185 (**way/246666403**),
  192 (**way/233903195** „Felix Renner Transporte GmbH") und 200 (**way/233903196**) erfasst sind; eine Suche nach
  **„Dinkel"** über die Felder name, operator, brand, description, website, contact:website und addr:housename findet im
  Fenster **keinen** Treffer. Der bisher gespeicherte Produktionspunkt 48.7258/9.1464 löst live auf **node/3127571980**
  (48.7258683/9.1466502) auf, ein Objekt mit `name`=**foto-butik**, `shop`=photo - **kein** Schrottbetrieb. Quelle 2
  (Betreiber/Register, im Dossier bereits zitiert, live nachgelesen am 04.11.2026): die **Betreiberseite**
  https://dinkel-gmbh.de/stuttgart-degerloch.html liefert **HTTP 200** und nennt live „Dinkel Recycling GmbH",
  **Epplestraße 182, 70567 Stuttgart**, Telefon 0711 453218, info@dinkel-gmbh.de und die Selbstanlieferung von
  Schrott/Metall durch Privat- und Gewerbekunden; das im Dossier zitierte **NorthData**-Profil (Amtsgericht Stuttgart
  **HRB 10644**) bestätigt Rechtsträger und heutige Existenz, war am 04.11.2026 aber wegen **HTTP 429** nicht erneut
  auslesbar und wird deshalb **nicht** als eigener Live-Beleg gezählt. **Warum keine Koordinate:** die Dossierzeile
  führt den Namen **„Dinkel"** mit dem Ort **Stuttgart-Möhringen**, die belegte Betriebsanschrift liegt aber in
  **Stuttgart-Degerloch**; die Verknüpfung der Möhringer Zeile mit der Degerlocher Gesellschaft stützt sich allein auf
  einen **Verzeichnislead** (stuttgart.city-map.de, Suchbereich Möhringen) und ist keine Betreiber- oder
  Registerfeststellung - die Übernahme des Degerlocher Punktes würde den Ort der Zeile ändern und ist eine
  **Owner-Entscheidung (Merge)**, keine Geokodierung. Zusätzlich fehlt die Hausnummer 182 in OSM, sodass kein
  adressgenaues Objekt vorliegt. `clear_fields:
  'description,dropoff_json,email,geo,opening_hours,phone,website,website_status'` **bleibt** unverändert bestehen (es
  gibt keinen Ersatzpunkt), `status: pruefung` bleibt, es wird keine Schließung und keine Betriebsverlegung behauptet.
  **Reichweitenklausel:** das Fehlen der Hausnummer 182 in OSM ist **kein** Nachweis für das Fehlen oder die Verlegung
  des Betriebs; die Operatorquelle belegt die Anschrift unverändert. Offen bleiben die Zuordnung der Möhringer Seedzeile
  zum Betreiber in Degerloch und eine adressgenaue OSM-Erfassung. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=9.15,48.73,9.17,48.77 ;
  https://api.openstreetmap.org/api/0.6/node/3127571980.json ; https://dinkel-gmbh.de/stuttgart-degerloch.html (Abruf
  04.11.2026) ; https://www.northdata.de/Dinkel%20Recycling%20GmbH,%20Stuttgart/HRB%2010644 (im Dossier zitiert, am
  04.11.2026 HTTP 429).]

### Recherche 18.11.2026

- [Recherche 18.11.2026 (Welle 0600 B): **Das amtliche Fachbetrieberegister bestätigt die Degerlocher Gesellschaft erstmals amtlich — und das amtliche Handelsregister führt für HRB 10644 noch den ALTEN Namen. Damit ist die Firmierung „Dinkel Recycling GmbH" durch zwei unabhängige amtliche Quellen gestützt, die Zuordnung zur Möhringer Seedzeile bleibt es nicht.** (a) Route „Entsorgungsfachbetriebe", Namenssuche „Dinkel": **7 Treffer**, davon „**Dinkel Recycling GmbH, Epplestraße 182, 70567 Stuttgart, ZZHT00400873 / 1 Sammeln, Befördern, Handeln, Makeln**"; die übrigen **6 Zeilen** betreffen ausschließlich die **Pfahler Müllabfuhr GmbH Dinkelsbühl** (zwei Anlagenstandorte in Dinkelsbühl mit je drei Anlagen) — also einen Namensvetter. **Zertifikat ZZHT004008731002 im Original gelesen** (PDF-Permalink, wörtlich mit `pdftotext -layout`, Ankreuzfelder mit `scripts/release/pdf_checkboxes.py`): Zertifizierungsorganisation **PÜG Prüf- und Überwachungsgesellschaft mbH**, Hämmerlestraße 14 + 16; **Zertifikatsnummer K15040-20260703-Efb-de**; Vorgangsnummer **ZZHT004008731002**; **1 Anlage**; gültig bis **19.11.2027**; Prüfungsdatum **20.05.2026**, Ausstellungsdatum **03.07.2026**; Betreiber (Feld 4): „**Dinkel Recycling GmbH**, Epplestraße 182, 70567 Stuttgart", **HRB 10644, Registergericht Stuttgart**; Anlage 1: Sammeln, Befördern, Handeln und Makeln (Kennnummer H011859101). (b) Das amtliche Handelsregisterportal (online-handelsregister.de, Abruf 18.11.2026) führt zu HRB 10644 **nicht** „Dinkel Recycling GmbH", sondern „**Dinkel GmbH Müll- und Schuttabfuhr**, Stuttgart • HRB 10644"; die Suche nach „Dinkel Recycling" bzw. „Dinkel Recycling GmbH Stuttgart" liefert **keine** Rechtseinheit, die Suche nach „Dinkel" nur die 100-Ergebnis-Grenze mit Dinkelacker/Dinkelsbühl-Treffern. Der Suchindex des Registerportals bildet die Umbenennung also noch nicht ab, während Betreiber-Impressum und amtliches Zertifikat den neuen Namen führen. Quellen: https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Dinkel ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&p=ZZHT00400873&fd=1 ; https://www.online-handelsregister.de/suche/Dinkel%20M%C3%BCll-%20und%20Schuttabfuhr ; https://www.online-handelsregister.de/suche/Dinkel%20Recycling (Abruf 18.11.2026).]
- [Recherche 18.11.2026 (Welle 0600 B): **Die Adresslage an der Epplestraße ist präzisiert: das amtliche Register führt den Standort als „Epplestraße 182 - 192".** Die Standortabfrage mit 3 km um den Suchpunkt **48.7419/9.1694** (Epplestraße im Degerlocher Abschnitt; die Hausnummer 182 selbst ist nicht adressgenau in OSM auffindbar, siehe unten) liefert **5 Efb-Treffer**, deren Entfernungsangaben das Register selbst ausweist: „Dinkel Recycling GmbH, Epplestraße 182, 70567 Stuttgart, ZZHT00400873 / 1" (2,3 km), „**REDI Abfallsortieranlage GbR, Epplestraße 182 - 192, 70567 Stuttgart-Degerloch, ZZHT01000227 / 1 Lagern, Verwerten, Beseitigen** und **/ 2 Behandeln, Verwerten, Beseitigen**" (2,3 km), „Felix Renner Transporte GmbH, Roßhaustraße 76, 70597 Stuttgart" (0,5 km) und „STRABAG GmbH Direktion Baden-Württemberg, Vaihinger Straße 131, 70567 Stuttgart" (2,9 km). **OSM-Gegenprüfung (live 18.11.2026):** im Kachelraster **9.1600/48.7350 bis 9.1750/48.7480** trägt die Epplestraße **78 Adressobjekte mit 78 verschiedenen Hausnummern** (u. a. 78, 80, 81, 82, 87, 90-96, 151) — **die Hausnummer 182 existiert dort weiterhin nicht**; der Dossiervermerk vom 04.11.2026 bleibt damit bestätigt. Quellen: https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Dinkel ; https://api.openstreetmap.org/api/0.6/map.json?bbox=9.16,48.735,9.175,48.748 (Abruf 18.11.2026).]
- [Recherche 18.11.2026 (Welle 0600 B): **Der Zuordnungskern bleibt offen — und die Möglichkeiten sind jetzt klar benannt.** Das Dossier führt die Seedzeile „**Dinkel**" mit dem Ort **Stuttgart-Möhringen**; die belegte Betriebsanschrift liegt in **Stuttgart-Degerloch**. Der einzige Zuordnungsweg ist weiterhin der Verzeichnislead stuttgart.city-map.de (Zuordnung zum Suchbereich Möhringen), also **kein** Betreiber- oder Registerkriterium nach README. Neu belegt ist, dass es **mindestens zwei** in Stuttgart registerlich geführte „Dinkel"-Linien gibt, die zu unterscheiden sind: (1) die Degerlocher **Dinkel Recycling GmbH** (HRB 10644, Register noch unter „Dinkel GmbH Müll- und Schuttabfuhr") und (2) die gleichnamige **REDI Abfallsortieranlage GbR** an „Epplestraße 182 - 192" (andere Gesellschaft, andere Tätigkeiten). Ein **eigenständiger, in Möhringen ansässiger Schrottbetrieb** ist weder im amtlichen Fachbetrieberegister (Namenssuche „Dinkel": alle 7 Treffer außerhalb Möhringens) noch im amtlichen Handelsregister nachweisbar. **Reichweite:** die Efb-Anerkennung ist freiwillig — ein Register-Nichteintrag in Möhringen beweist **nicht**, dass es dort keinen Betrieb gibt; die Namenssuche „Dinkel" ist zudem ein Präfix-/Teilzeichenabgleich. **Konsequenz für das Frontmatter:** **keine Änderung.** `status: pruefung`, `clear_fields: 'description,dropoff_json,email,geo,opening_hours,phone,website,website_status'` und der leere `certifications`-Stand bleiben bestehen; die Degerlocher Zertifikats- und Kontaktdaten werden **nicht** auf die Möhringer Zeile übertragen. Der Merge (Möhringer Zeile = Dinkel Recycling GmbH) bleibt eine **Owner-Entscheidung**, keine Rechercheableitung. Quellen: siehe oben (Abruf 18.11.2026).]
- [Recherche 18.11.2026 (Welle 0600 B): **Preise (Ankauf / Verkauf / Gebühren) getrennt.** **ANKAUF:** kein veröffentlichter Zahlenkurs; das geprüfte Zertifikat ist ein Anerkennungs-, kein Preisdokument. **VERKAUF:** keine Verkaufspreisliste. **GEBÜHREN:** Die Allgemeinen Geschäftsbedingungen der Dinkel Recycling GmbH (PDF `2022-DINKEL-AGB.pdf`, HTTP 200, live abgerufen am 18.11.2026) belegen in **§ 5 Preise** die **Preisarten**, aber **keine Eurobeträge**: Absatz 1 „Es gelten die vereinbarten Preise. Ergänzend gilt die **Dinkel-Preisliste** zzgl. der jeweils gesetzlichen Umsatzsteuer"; Absatz 2 nennt **Nebenleistungen** wie „Analysen, Erstellung von Entsorgungsnachweisen, **Wiegekosten**, Begleit- und Übernahmescheinbearbeitung u. ä." sowie „**Verwaltungsgebühren**, die bei der Bearbeitung von Entsorgungs- bzw. Verwertungsnachweisen nach der Nachweisverordnung anfallen"; Absatz 3: „Dinkel berechnet dem AG **vergebliche An- und Abfahrten** … gemäß der Dinkel-Preisliste" und die mit der Lagerung verbundenen Kosten; Absatz 4: Weiterberechnung erhöhter Entsorgungskosten bei >10 % Annahmepreiserhöhung. Die **Preisliste selbst ist nicht veröffentlicht** — die genannten Positionen sind damit **Gebührenarten ohne Betrag**, keine belegten Tarife, und werden nicht als Zahlenwerte übernommen. Für die **Seedzeile „Dinkel" (Möhringen)** selbst bleibt es bei: **ANKAUF** keine Kurse, **VERKAUF** keine Liste, **GEBÜHREN** keine Tarife. Quellen: https://dinkel-gmbh.de/dl/2022-DINKEL-AGB.pdf ; https://dinkel-gmbh.de/impressum.html (Abruf 18.11.2026).]
