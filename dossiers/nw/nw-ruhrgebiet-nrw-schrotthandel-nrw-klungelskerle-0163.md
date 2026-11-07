---
slug: nw-ruhrgebiet-nrw-schrotthandel-nrw-klungelskerle-0163
clear_fields: 'dropoff_json,geo'
name: Schrotthandel NRW / Klüngelskerle (0163)
trader_type: schrotthaendler
state: NW
city: Ruhrgebiet/NRW
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://schrotthandel.nrw
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Mobile / NRW-weite Händler (ohne geprüften stationären Platz)
provenance_ankauf_raw: mobil/ja
provenance_origin: table
---

# Schrotthandel NRW / Klüngelskerle (0163)

## Überblick

Mobiler NRW-Schrottankauf wird jetzt direkt auf der erreichbaren Zieldomain beworben. Der aktuelle Auftritt nennt 0163 4694319 und info@schrotthandel.nrw; das Impressum nennt „Schrotthandel.nrw / EL-Lahib“ und Vorstadtstraße 65, Bochum, jedoch weder vollständigen Vornamen noch Rechtsform. Die Kontaktseite zeigt 4694319, enthält im HTML aber zusätzlich einen unbeschrifteten `tel:`-Link zu 0163 4630051; auch der frühere 11880-Lead nennt 4630051 und Lohrheidestr. 156. Betreiberbezug und Einsatz-/Betriebsadresse sind ungeklärt. Die separate Domain nrw-schrott.de mit Tarek El-Lahib und Robertstraße 70 wird nicht ohne belegte Betreiberkette zusammengeführt. `website_status: aktiv` beschreibt nur die technische Erreichbarkeit; wegen ungeklärter Betreiber-/Seedidentität bleibt `status: pruefung`. Keine Kontaktdaten oder Adresse in strukturierte Felder übernommen; `clear_fields: 'dropoff_json,geo'` nimmt die nicht belegte frühere Annahmezusage und den Punkt zurück. Für den Zielbetrieb: **Ankauf** ohne verifizierte Kurse; **Verkauf** ohne Preisliste; **Gebühren** ohne überprüfbare Tarifangabe.

## Timeline

### Owner-Audit 26.10.2026: Website wieder direkt erreichbar

- [Owner-Audit 26.10.2026: Root, Impressum, Kontakt und Schrottankauf von schrotthandel.nrw lieferten jetzt direkt lesbare Betreiberseiten; die frühere Einstufung `website_status: blockiert` war als technische Erreichbarkeitsangabe überholt und wird zu `aktiv` korrigiert. Der Auftritt nennt mobil NRW-weite Leistungen, 0163 4694319, info@schrotthandel.nrw und im Impressum „Schrotthandel.nrw / EL-Lahib“, Vorstadtstraße 65, 44866 Bochum; ein voller Vorname bzw. eine Rechtsform fehlen. Zusätzlich zur sichtbaren 0163 4694319 enthält die aktuelle Kontaktseite einen unbeschrifteten HTML-`tel:`-Link auf 0163 4630051; die Nummer steht auch im bisherigen 11880-Lead, der außerdem Lohrheidestr. 156 nennt. Das ist ein ungelöster Kontakt-/Adresskonflikt, keine Bestätigung der 11880-Adresse. Die separate nrw-schrott.de-Spur mit Tarek El-Lahib/Robertstraße 70 wird mangels Betreiberkette nicht zusammengeführt. `status: pruefung`, leere Adresse/Kontakte und `clear_fields: 'geo'` bleiben angemessen: die Website belegt kein stationäres Annahmegrundstück und keinen Punkt; keine Nummer übernommen. Keine Produktionsdaten abgefragt oder geändert. Quellen: https://schrotthandel.nrw/ ; https://schrotthandel.nrw/impressum ; https://schrotthandel.nrw/contact ; https://schrotthandel.nrw/schrottankauf ; Abgrenzung: https://nrw-schrott.de/impressum.html (Abruf 26.10.2026).]

### Owner-Korrektur 26.10.2026: alte Annahmezusage zurücknehmen

- [Owner-Korrektur 26.10.2026: Der read-only Produktionsdatensatz enthielt neben dem Punkt `51.58945/7.2035` noch `dropoff_json={"allowed":true}`. Die aktuelle Betreiberseite bewirbt einen mobilen NRW-weiten Abholservice und nennt Partner-Wertstoffhöfe als Ziel der abgeholten Materialien; sie belegt keinen eigenen öffentlichen Annahmestandort. Zusammen mit der weiterhin ungeklärten Betreiber-/Seedidentität trägt das die gespeicherte allgemeine Selbstanlieferungszusage nicht. Der Owner ergänzt `clear_fields: 'dropoff_json,geo'`; der nächste reguläre Seed setzt die Zusage auf unbekannt `{}` und entfernt den falschen GEO-Punkt, ohne `allowed:false`, Schließung oder eine Partnerannahme dem Seed-Händler zuzuschreiben. `website_status: aktiv` bleibt allein technische Erreichbarkeit, `status: pruefung` bleibt wegen der offenen Identität. Keine Produktionsdaten geändert. Quellen: Read-only `public.db` (26.10.2026); https://schrotthandel.nrw/ ; https://schrotthandel.nrw/schrottankauf ; https://schrotthandel.nrw/contact ; `docs/trader-seed.md`.]

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 51.58945, 7.2035. Nominatim-Reverse löst den Punkt in der Tiroler Straße in Recklinghausen auf. Der Seed beschreibt einen unbestätigten mobilen NRW-Service, keine dortige Betriebs- oder Annahmestelle; der Straßenpunkt ist irreführend. `clear_fields: 'geo'` entzieht ihn beim nächsten regulären Seed. Keine Ersatzkoordinate für ein regionales Einsatzgebiet geraten. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.58945&lon=7.2035&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026: neue konkrete Anschrift als Lead

- [Recherche 25.10.2026: 11880-Detailseite „Schrotthandel.nrw“ direkt gelesen: Lohrheidestr. 156, 44866 Bochum-Wattenscheid, 0163 4630051, info@schrotthandel.nrw, Mo–Sa 07:00–19:00; Eintragsdaten 21.09.2026, Portalaktualisierung 09.10.2026. Die Seite verlinkt die Zieldomain schrotthandel.nrw. Das ist ein neuer, passender Lead für eine der bisherigen konkurrierenden 0163-Nummern (die 0163 4694319 bleibt unbestätigt), aber kein Betreiber-Impressum oder unabhängiger Zweitbeleg. Die Direktabrufe vom 18.10. hatten Root 403 und /impressum, /schrottankauf, /datenschutz 404; der Portalstatus „lädt normal“ ersetzt keinen Primärquellenabruf. **Ankauf:** keine Zahlenkurse; **Verkauf:** keine Liste; **Gebühren:** keine belastbaren Tarife. Keine Felder geändert. Lohrheidestr. 156 ist erst nach Betreiber-/Annahmebestätigung geocodierbar; nicht mit der separaten Robertstraße-70-Spur zusammenführen. Quelle (Lead): https://www.11880.com/branchenbuch/bochum/120674719B112437141/schrotthandel-nrw.html ; Direktgegenprobe: https://schrotthandel.nrw/ .]

### Importiert (Seed-Stand 2026-09-30)

- Altmetall (Cu/Messing/Alu/Edelstahl), Autoteile

### Recherche 01.10.2026

- [Recherche 01.10.2026: Homepage-Abruf 403, /impressum-Abruf 404 → website_status blockiert gesetzt; Impressum-Inhalt (EL-Lahib, Vorstadtstr. 65, 44866 Bochum) nur als Such-Snippet greifbar (Lead, keine zweite Quelle); Vorgänger-Domain nrw-schrott.de nennt abweichend Robertstr. 70, 44809 Bochum → Identitäts-/Adressklärung offen, Klärfall; Quelle(n): eigene Fetch-Probe 01.10.2026, Websuche (Leads)]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Retry /schrottabholung-koeln → 404 (Suchindex listet die Seite mit Inhalt + Tel. 0163/4694319, Zeiten Mo-Fr 7-21/Sa 9-21 - Bot-Sperre wahrscheinlich, kein Browser-Rendering moeglich). website_status bleibt blockiert; kein Fill; Klaerfall Adressen (Vorstadtstr. 65 vs. Robertstr. 70) weiter offen; Quelle(n): eigene Fetch-Probe 02.10.2026, Websuche (Leads)]

### Recherche 03.10.2026

- [Recherche 03.10.2026: website auf Domain-Root normalisiert. Betreiberseiten /schrottankauf und /schrottabholung-solingen sind im Suchindex mit Schrottankauf/Abholung in NRW, Kontaktangaben und aktuellen Copyright-Jahren auffindbar, konnten einzeln direkt jedoch nicht abgerufen werden (Root 403, relevante Unterseiten 404); daher keine Kontakt-, Öffnungszeit- oder Adressdaten daraus übernommen und website_status bleibt blockiert. Die direkt abrufbare ältere Domain nrw-schrott.de nennt Tarek El-Lahib, Robertstraße 70, 44809 Bochum, Kontakt 01525-2376589 und Inhalte mit RSS-Daten aus 2015; eine belastbare Betreiber-/Identitätsverknüpfung zur aktuellen Domain mit abweichenden indexierten Kontaktdaten fehlt. Frühere Such-Snippets zu Vorstadtstr. 65 und die abweichende Alt-Domain-Adresse bleiben Leads; keine Merge- oder Adressentscheidung. Quelle(n): https://schrotthandel.nrw/, https://schrotthandel.nrw/schrottankauf, https://schrotthandel.nrw/schrottabholung-solingen, https://schrotthandel.nrw/impressum, https://nrw-schrott.de/impressum.html, https://nrw-schrott.de/rss.xml]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Erneute Direktprobe bestätigt: https://schrotthandel.nrw/ weiterhin 403, /schrottankauf und /impressum weiterhin 404. Der Suchindex führt zur Ankaufseite nun Telefon 0163 4630051, während frühere Snippets 0163 4694319 nennen; beide Nummern bleiben unbestätigte Leads und werden nicht ins Frontmatter übernommen. Die ältere, direkt erreichbare Domain https://nrw-schrott.de/impressum.html nennt Tarek El-Lahib, Robertstraße 70, Bochum, aber keine belegte Verbindung zum aktuellen Domainauftritt. ANKAUF: nur Werbeaussage/Indexhinweis, keine numerische Preisliste; VERKAUF: keine Liste gefunden; GEBÜHREN: keine überprüfbare Tarifangabe. Offen: aktuelle Betreiberidentität, korrekter Kontakt, Rechts-/Impressumsseite und ob tatsächlich Schrott angenommen wird. Quelle(n): https://schrotthandel.nrw/; https://schrotthandel.nrw/schrottankauf; https://schrotthandel.nrw/impressum; https://nrw-schrott.de/impressum.html; Suchindex-Abfrage als Lead: https://www.google.com/search?q=site%3Aschrotthandel.nrw+0163+Schrottankauf]

### Korrektur 18.10.2026

- [Korrektur 18.10.2026: Status aktiv → pruefung wegen ungeklärter Betreiber-/Seedidentität und lediglich indexgestützter Ziel-Leistungen, nicht allein wegen HTTP 403. Im heutigen Review Root mit/ohne www jeweils 403, /impressum, /schrottankauf und /datenschutz jeweils 404; kein Betreiberinhalt dieser Zieldomain gelesen. Indexangaben 0163 4630051 versus 0163 4694319 bleiben Konflikt-Leads, keine bestätigten Kontakte. Startseite/Impressum der separaten Domain nrw-schrott.de gelesen: Tarek El-Lahib, Robertstraße 70, Bochum, 01525-2376589; Kopf mit zusätzlichem WhatsApp-Kontakt. Dort beworbene kostenlose Abholung ist termin-/standort-/mengenabhängig; 100–150 kg nur FAQ-Beispiel, keine allgemeine feste Mindestmenge. Ankauf nach Menge/Qualität/Aufwand, Wiederverkauf von Fahrzeugteilen und möglicher Demontagemehraufwand, im gelesenen Text keine numerischen Tarife. Keine dieser Angaben in strukturierte Zielfelder übertragen. Gemeinsamer Nachname beweist keine Betreiberkette; separate Domain nicht als bewiesener Vorgänger oder anderer Betreiber behandeln. Historische RSS-Daten beweisen nicht heutiges Inhaltsalter; der uncommittete 12.10.-Entwurf ist kein belegter damaliger Abruf. Für den Zielbetrieb Preis-/Gebührenbeträge offen. Quellen: https://schrotthandel.nrw/ ; https://www.schrotthandel.nrw/ ; https://schrotthandel.nrw/impressum ; https://schrotthandel.nrw/schrottankauf ; https://schrotthandel.nrw/datenschutz ; https://nrw-schrott.de/impressum.html ; https://nrw-schrott.de/]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Für **Schrotthandel NRW / Klüngelskerle (0163)**
  sind die **Einsatz-/Betriebsadresse** und der **Betreiberbezug** ungeklärt: das Betreiber-Impressum und das
  Verzeichnisprofil nennen **zwei verschiedene** Anschriften in Bochum, und die Website belegt **kein** stationäres
  Annahmegrundstück. **Was live geprüft wurde (04.11.2026):** (a) **Vorstadtstraße** (Kachelraster 7.1200/51.4700 bis
  7.1500/51.5000, **9 Kacheln von je 0,01 Grad, 0 Kachel-Fehler**, live abgerufen) trägt **55** Adressobjekte mit **54**
  verschiedenen Hausnummern (1, 2, 3, 5, 6, 7, 7a, 8, 9, 10, 11, 12, 13, 14, 15, 15a, 16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 48, 49, 50, 51, 52, 53, 55, 56, 57, 58, 59, 60, 61, 62, 63, 65 und
  67); die **Hausnummer 65 existiert** als **way/325148295** mit `addr:housenumber`=65, `addr:street`=Vorstadtstraße,
  `addr:city`=Bochum, `addr:postcode`=44866 und `building`=yes - **ohne** `name=` **noch** `operator=`. (b)
  **Lohrheidestraße** (Kachelraster 7.1000/51.4700 bis 7.1300/51.5000, **9 Kacheln von je 0,01 Grad, 0 Kachel-Fehler**,
  live abgerufen) trägt **71** Adressobjekte mit **64** verschiedenen Hausnummern (1, 2, 3, 5, 7, 8, 8a, 9, 10, 12, 14,
  18, 49, 51, 53, 55, 56, 57, 58, 59, 60, 61, 61a, 62, 63, 64, 66, 68, 69, 70, 71, 71a, 72, 82, 83, 85, 87, 89, 91, 121,
  123, 125, 135, 137, 138, 139 und 140–157); die **Hausnummer 156 existiert** als **way/249160779** mit
  `addr:housenumber`=156, `addr:street`=Lohrheidestraße, `addr:city`=Bochum, `addr:postcode`=44866 und
  `building`=**apartments** - also ein **Wohnhaus**, ebenfalls **ohne** Betriebsbezug. (c) Namenssuchen nach **„Lahib“**
  über die Felder name, operator, description, website, contact:website, brand, note und addr:housename finden in
  **beiden** Rastern **keinen** Treffer. Quelle 1 (Betreiber/Verzeichnis): das **Impressum** der Betreiberseite
  https://schrotthandel.nrw/impressum nennt live (Abruf 04.11.2026) **„Schrotthandel.nrw EL-Lahib, Vorstadtstraße 65,
  44866 Bochum“** (ohne vollständigen Vornamen und ohne Rechtsform); die **Kontaktseite**
  https://schrotthandel.nrw/contact zeigt die Nummer **4694319** und enthält zusätzlich einen **unbeschrifteten**
  `tel:`-Link auf **0163 4630051**; das im Dossier bereits zitierte **11880**-Profil „Schrotthandel.nrw“ nennt dagegen
  **Lohrheidestr. 156, 44866 Bochum-Wattenscheid, 0163 4630051, info@schrotthandel.nrw, Mo–Sa 07:00–19:00**
  (Eintragsdaten 21.09.2026, Portalaktualisierung 09.10.2026) - die beiden Anschriften stehen also **unvereinbar**
  nebeneinander, und die Domain **nrw-schrott.de** (Tarek El-Lahib, Robertstraße 70) wird mangels belegter
  Betreiberkette **nicht** zusammengeführt. Quelle 2 (OSM): die beiden live abgerufenen Kachelraster
  7.1200/51.4700–7.1500/51.5000 und 7.1000/51.4700–7.1300/51.5000 mit way/325148295 und way/249160779 **ohne**
  Betriebsbezug. **Warum keine Koordinate:** Der Seed ist ein **mobiler NRW-Dienst**; die Website belegt **kein**
  stationäres Annahmegrundstück, und die beiden einzigen Adressangaben sind **widersprüchlich** (Impressum
  Vorstadtstraße 65 gegen 11880 Lohrheidestraße 156) - jeder Punkt müsste **eine** davon willkürlich auswählen und würde
  zudem einen Abholhof suggerieren, den keine Quelle trägt; der zudem nicht zum Seed passende Firmenname (Impressum
  „EL-Lahib“ gegen Seed-Nennung „Klüngelskerle“) bleibt ungeklärt. **Was zur Auflösung fehlt:** eine Bestätigung, welche
  der beiden Anschriften die Betriebsadresse ist, und ein Beleg für eine **öffentliche Annahme** (statt eines mobilen
  Abholservices). `clear_fields: 'dropoff_json,geo'` **bleibt** bestehen (die nicht belegte frühere Annahmezusage und
  der falsche Punkt bleiben zurückgenommen), `status: pruefung` bleibt unverändert; es wird weder Fortbestand noch
  Schließung behauptet. Quelle(n): https://api.openstreetmap.org/api/0.6/way/325148295/full.json ;
  https://api.openstreetmap.org/api/0.6/way/249160779/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.1200,51.4700,7.1500,51.5000 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.1000,51.4700,7.1300,51.5000 ;
  https://schrotthandel.nrw/impressum ;
  https://www.11880.com/branchenbuch/bochum/120674719B112437141/schrotthandel-nrw.html (Abruf 04.11.2026).]

### Recherche 07.11.2026: „Klüngelskerle" ist die Eigenbezeichnung der Betreiberseite; Adresskonflikt bleibt

- [Recherche 07.11.2026: **Der im Dossier vom 26.10.2026 als ungeklärter Widerspruch notierte Namens-/Adresskonflikt ist am 07.11.2026 teilweise aufgelöst: „Klüngelskerle" ist die Eigenbezeichnung der Betreiberseite, und die drei 0163-Nummern lassen sich genauer zuordnen.** Die Seiten https://schrotthandel.nrw/schrottankauf und https://schrotthandel.nrw/ wurden am 07.11.2026 live gelesen: die Angebotsseite trägt die Überschrift **„Schrottankauf durch die Klüngelskerle in NRW"** und die Signatur **„-Ihre Klüngelskerle aus dem Ruhrgebiet"** — der Seed-Name „Klüngelskerle" ist damit **nicht** mehr nur eine Seed-Ableitung, sondern die **selbst gewählte Marke der Betreiberseite**. Das Impressum (https://schrotthandel.nrw/impressum, live gelesen) nennt weiterhin **„Schrotthandel.nrw / EL-Lahib, Vorstadtstraße 65, 44866 Bochum"** ohne vollständigen Vornamen und ohne Rechtsform; die **Startseite** nennt als Kontakt **0163 4694319** mit den Öffnungszeiten **Mo–Fr 07:00–21:00** und die Angebotsseite zusätzlich **Sa 09:00–21:00** sowie **info@schrotthandel.nrw** und einen WhatsApp-Kontakt. **Die drei Nummern im Dossier lassen sich damit so zuordnen:** **0163 4694319** ist die auf der **Startseite und der Angebotsseite** sichtbare Hotline (E-Mail- und WhatsApp-Kontakt), **0163 4630051** ist die im 11880-Eintrag genannte Nummer mit der **Anschrift Lohrheidestr. 156** (die im Dossier vom 25.10.2026 und 04.11.2026 dokumentierte unbeschriftete `tel:`-Verlinkung auf der Kontaktseite), und **0163 4694319 / 0163 4630051** sind die beiden Kontaktvarianten. **Der Adresskonflikt Vorstadtstraße 65 (Impressum) gegen Lohrheidestraße 156 (11880) bleibt offen** — die Betreiberseite nennt **kein** stationäres Annahmegrundstück, sondern bewirbt einen mobilen NRW-Service; die 11880-Adresse ist Aggregator-Klasse. Der Verweis der Website auf „Partner-Wertstoffhöfe" als Ziel der abgeholten Materialien bestätigt den mobilen Charakter und schließt einen eigenständigen Lagerhof aus. **Rechtsform bleibt offen:** eine e.K., GbR oder Einzelfirma ist aus dem Impressum nicht ableitbar; „EL-Lahib" ist eine Namensangabe ohne Vornamen und ohne Rechtsform.]
- [Recherche 07.11.2026: **Amtliches Fachbetrieberegister (E-15) am 07.11.2026 clientseitig gerendert abgefragt (Chromium/Playwright, da die Anwendung clientseitig rendert): Route „Entsorgungsfachbetriebe" und Route „Altfahrzeugverwertung", durchgängig mit dem Filter „Art des Betriebs = Egal" (is=1&e=1). Namenssuche „El-Lahib" → 0 Treffer, Namenssuche „Klüngelskerle" → 0 Treffer (Route „Entsorgungsfachbetriebe"). Standortabfrage im 3-km-Radius um „Vorstadtstraße 65, 44866 Bochum": Route „Entsorgungsfachbetriebe" → **12 Treffer**, sämtlich ortsfremd; Route „Altfahrzeugverwertung" → **0 Treffer** — **kein El-Lahib- oder Klüngelskerle-Eintrag** am Impressum-Standort. **Reichweite der Negativaussagen:** geprüft wurden die Namensvarianten „El-Lahib" und „Klüngelskerle" in beiden Registerrouten sowie die Standortsuche mit validiertem adr-Parameter im 3-km-Radius um die Impressumsanschrift; für die 11880-Anschrift Lohrheidestraße 156 wurde am 07.11.2026 **keine eigene Radiusabfrage** gestellt. Abrufdatum 07.11.2026. Die Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind **freiwillig**, ein Register-Nichteintrag beweist **nicht** die Abwesenheit eines Betriebes und ist kein Nichtexistenzbeweis; der Dritt-Spiegel schrottregister.pages.dev wurde nicht als Beleg herangezogen.]
- [Recherche 07.11.2026: **Preise, strikt getrennt (live am 07.11.2026 gelesen): ANKAUF —** die Angebotsseite nennt als angekaufte Waren **Elektrogeräte, Metallreste, kaputte Maschinen, Metallschrott, Messing und Altmetalle** sowie besonders lukrativ **Kupfer, Messing, Aluminium und Edelstahl**, verspricht „attraktive Preise" und nennt als Preisermittlungsgrundlage **Menge, Beschaffenheit und Reinheitsgrad** sowie **tagesaktuelle Marktpreise**; **es wird kein €/kg- oder €/t-Wert genannt** — eine Preisliste existiert nicht, das Angebot ist ein **individuelles Angebot nach Anfrage** („verbindliches Angebot" nach dem Gespräch, Zahlung **bar oder per Überweisung**). **VERKAUF —** die Angebotsseite nennt als **Ankauf von Autoteilen** ausdrücklich ein Interesse: „wir sind auch am Ankauf von Autoteilen interessiert", und beschreibt die Demontage von Fahrzeugen, um Teile einzeln zu verkaufen — das ist ein **Ankaufsangebot des Händlers an den Kunden**, kein Verkauf von Teilen an den Kunden; eine **Verkaufspreisliste** (Ersatzteilpreise für Endkunden) wird **nicht** veröffentlicht. **GEBÜHREN —** die Angebotsseite nennt eine **kostenlose Mitnahme bei geringer Schrottmenge**: „Falls die Menge Ihres Schrotts für einen Schrottankauf zu gering ist, bieten wir Ihnen alternativ eine kostenlose Mitnahme an" — eine **bedingte** Freizeitnahme, **kein** generell kostenloser Abholservice und **ohne** Mengenschwelle, Preis oder Einheit. Die Startseite nennt eine **kostenfreie Beratung** und ein **unverbindliches Angebot**; ein Tarif, eine Pauschale oder eine Gebührenordnung wird **nicht** veröffentlicht. Der Dritt-Spiegel nrw-schrott.de (anderer Betrieb, Tarek El-Lahib) bleibt getrennt. Keine Frontmatter-Änderung, `status: pruefung` bleibt, `clear_fields: 'dropoff_json,geo'` bleibt bestehen. Quellen: https://schrotthandel.nrw/schrottankauf ; https://schrotthandel.nrw/ ; https://schrotthandel.nrw/impressum ; https://schrotthandel.nrw/contact ; https://www.11880.com/branchenbuch/bochum/120674719B112437141/schrotthandel-nrw.html ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=El-Lahib ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=El-Lahib ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Kl%C3%BCngelskerle ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Vorstadtstra%C3%9Fe%2065%2C%2044866%20Bochum%2C%20Deutschland%3B7.108700%3B51.500950%3B7.194100%3B51.464250%3B7.108700%3B51.464250&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Vorstadtstra%C3%9Fe%2065%2C%2044866%20Bochum%2C%20Deutschland%3B7.108700%3B51.500950%3B7.194100%3B51.464250%3B7.108700%3B51.464250&dst=3&is=1&e=1 (Abruf 07.11.2026).]
