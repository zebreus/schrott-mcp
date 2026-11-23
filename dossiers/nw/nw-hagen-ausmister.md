---
slug: nw-hagen-ausmister
name: Ausmister
trader_type: sonstige
state: NW
city: Hagen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://dieausmister.de/hagen
website_status: aktiv
status: pruefung
clear_fields: 'email,geo,opening_hours,phone'
description: 'Die Betreiberwebsite bewirbt Entrümpelung und Haushaltsauflösung einschließlich Entsorgung zum Festpreis nach Besichtigung; rechtliche Betreiberidentität und genaue Kontakt-/Standortangaben sind nicht unabhängig bestätigt. Keine allgemeine Schrottannahme belegt.'
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Ausmister

## Überblick

Die Betreiberseite `dieausmister.de/hagen` ist neu aufgebaut; das Impressum `dieausmister.de/impressum/` nennt seit dem 23.11.2026 **Inhaberin: Elheme Sykaj**, Wehringhauser Str. 60, 58089 Hagen, Telefon 02331 9109428, info@dieausmister.de und die **USt-IdNr. DE274573289** - die rechtliche Betreiberidentität ist damit belegt. Die Website bietet Entrümpelung, Haushaltsauflösung und Entsorgung zum Festpreis nach Besichtigung; sie bewirbt selbst eine „Fachkunde § 9 EfbV · § 54 KrWG“ und entsorgt „fachgerecht über zugelassene Betriebe aus der Region“ - der Betrieb ist damit **kein** Entsorgungsfachbetrieb und **kein** Schrottplatz. Ihre Beratungs-/Besichtigungszeit Mo–So 08:00–22:00 ist keine belegte Öffnungszeit eines Schrottplatzes. Der Status `pruefung` und die fehlende allgemeine Schrottannahme bleiben bestehen; die im Frontmatter über `clear_fields` zurückgenommenen Felder `email`, `phone` und `opening_hours` bleiben zurückgenommen.

- **Ankauf:** kein allgemeiner Metallankauf und kein Schrottpreis belegt.
- **Verkauf:** kein Warenangebot oder Verkaufspreisblatt gefunden.
- **Gebühren:** die Betreiberseite zeigt Beispielpreise für Räumungsaufträge (u.a. Kellerraum 250 €, kleine Wohnung 899 €, Wohnung 60 m² 1.200 €, Haus ca. 85 m² 1.700 €); sie bezeichnet die Beträge als grobe Schätzung anhand beispielhafter Inhalte, nicht als Metallankauf, allgemeine Entsorgungsgebühr oder verbindliches Angebot. Beratung und Vor-Ort-Besichtigung werden als kostenlos beworben.
- **GEO:** keine Koordinaten ergänzt; die Kontaktadresse allein belegt keinen öffentlichen Annahmehof.

## Timeline

### Owner-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Die Betreiberwebsite nennt Wehringhauser Str. 60, 58089 Hagen, Telefon, E-Mail und Beratungs-/Besichtigungszeiten Mo-So 08:00-22:00. Das Facebook-Profil bestätigt nur Name, Hagen und Entrümpelungsangebot, nicht diese konkreten Felder; die Seite erfüllt auch nicht die README-Ausnahme für eine registerkongruente Betreiberquelle. Straße, PLZ, Telefon, E-Mail und Beratungszeit werden daher nicht als verifizierte Händler-Stammdaten ausgespielt; Telefon, E-Mail und Zeiten explizit zurückgenommen. Die Website bleibt als Recherche-Lead erreichbar, `status: pruefung` bleibt. Der alte Punkt ist separat als falsch belegt und wird mit `clear_fields: 'geo'` entfernt; kein Ersatzpunkt. Quellen: https://dieausmister.de/hagen ; https://www.facebook.com/DieAusmister/ ; https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; README.md, Quellenhierarchie.]

### Recherche 24.10.2026: Betreiberseite direkt geprüft

- [Recherche 24.10.2026: Betreiberseite live direkt gelesen. Sie nennt Die Ausmister, Wehringhauser Strasse 60, 58089 Hagen, 02331-9109428, info@dieausmister.de und Beratungs-/Besichtigungszeit Mo–So 08:00–22:00. Beschrieben werden Entrümpelung inklusive Entsorgung zum Festpreis, Abbruchservice und Renovierung; eine allgemeine Schrottannahme oder ein Metall-Ankaufspreis wird nicht angeboten. Die Seite zeigt nur grobe Beispielpreise für konkrete Räumungen (250/899/1.200/1.700 €) sowie kostenlose Beratung/Besichtigung; diese Beträge sind keine Gebühren für bloße Schrottabgabe. Adresse/Kontakt/Website/Öffnungszeit in Frontmatter ergänzt, `status: pruefung` bleibt mangels im Seitenabruf sichtbarer Betreiber-/Registeridentifikation. Keine Drop-off-/Pickup-Zusage und keine Koordinaten ergänzt. Quelle: https://dieausmister.de/hagen .]

### Korrektur 24.10.2026: Identität und falscher Produktionspunkt

- [Korrektur 24.10.2026: Das direkte Facebook-Profil „Die Ausmister“ beschreibt Entrümpelungsangebote in Hagen und stimmt beim Namen mit der Betreiberseite überein; es veröffentlicht keine Straße oder Telefonnummer. Der Website-Menüpunkt „Impressum“ führt zurück auf `/hagen`; eine rechtliche Betreiberangabe war dort nicht sichtbar. Die Kontaktfelder bleiben deshalb als Betreiberangaben markiert, `status: pruefung` bleibt, und es wird keine allgemeine Schrottannahme behauptet. Read-only-Prüfung der Produktionszeile ergab den Punkt `51.90933, 7.88825`; Nominatim verortet ihn in Mehringen, Everswinkel, Kreis Warendorf, PLZ 48351, nicht in Hagen. Eine Abfrage der Website-Adresse Wehringhauser Straße 60 liefert ein Gebäude namens „Antik Eck“, keine bestätigte Annahmestelle der Ausmister. Daher `clear_fields: 'geo'`: den falschen Punkt beim nächsten regulären Seed entfernen und keine Ersatzkoordinate setzen. Produktionsdatenbank blieb unverändert; Quelle: https://www.facebook.com/DieAusmister/ ; https://dieausmister.de/hagen ; https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Wehringhauser%20Strasse%2060%2C%2058089%20Hagen&format=jsonv2&addressdetails=1.]

### Importiert (Seed-Stand 2026-09-30)

- Entrümpler (GS)
- Adresse: Hagen

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Adresse **Wehringhauser Str. 60,
  58089 Hagen** ist in OSM adressgenau erfasst, trägt dort aber den **Namen eines anderen Betriebs**; die
  Betreiberseite erfüllt die Anforderung an eine zulässige Betreiberquelle nicht. **Was live geprüft wurde
  (03.11.2026):** (a) Das Adressobjekt existiert: **way/284230977** mit `addr:housenumber`=**60**,
  `addr:street`=**Wehringhauser Straße**, `addr:city`=Hagen, `addr:country`=DE, `addr:postcode`=**58089**,
  `building`=yes, `name`=**Antik Eck**, `source`=NRW Atlas ALK Vektor (Mittel der 12 eindeutigen Way-Knoten
  **51.3554606/7.4543859**, BBox 51.3553570/7.4542209 – 51.3555316/7.4546032) — das ist ein **anderer Betrieb** an
  derselben Hausnummer; eine Bestätigung der Ausmister an dieser Anschrift ist damit **nicht** erbracht. (b) Im Umfeld
  von 80 m liegen keine weiteren Objekte mit Betriebsbezug zu „Ausmister“; eine Suche über `name`, `operator`, `brand`,
  `website`, `contact:website`, `description`, `note` und `short_name` nach **„Ausmister“** im Kachelraster
  **7.4200/51.3400 – 7.5600/51.4200** (112 Kacheln, 0 Kachel-Fehler) liefert **0** Treffer — Aussage zum OSM-Bestand,
  **kein** Negativbeleg über den Betrieb. Quelle 1 (Betreiber/Register): die Betreiberseite **dieausmister.de/hagen**
  nennt **Wehringhauser Strasse 60, 58089 Hagen**, **02331-9109428**, **info@dieausmister.de** und
  Beratungs-/Besichtigungszeiten Mo–So 08:00–22:00; sie beschreibt **Entrümpelung inklusive Entsorgung zum Festpreis**,
  Abbruchservice und Renovierung, aber **keine** allgemeine Schrottannahme und **keinen** Metall-Ankaufspreis. Der
  Menüpunkt „Impressum“ führt auf dieselbe Seite `/hagen` zurück, eine rechtliche Betreiberangabe (Firmierung,
  Register) ist **nicht** sichtbar — die Seite erfüllt die README-Ausnahme für eine registerkongruente Betreiberquelle
  damit **nicht**; das Facebook-Profil „DieAusmister“ bestätigt nur Name, Stadt und Entrümpelungsangebot, **nicht**
  die Anschrift. Das Dossier hat die Felder `email`, `phone` und `opening_hours` deshalb bereits zurückgenommen.
  Quelle 2 (OSM): die Adresse ist vorhanden, trägt aber den **Fremdnamen „Antik Eck“** und keinen Bezug zu den
  Ausmister. **Was fehlt:** eine rechtlich belastbare Betreiberidentität (Impressum mit Firmierung/Register) **oder**
  eine zweite unabhängige Bestätigung der Anschrift; erst damit wäre die Karte der Ausmister dieser Hausnummer
  zuzuordnen. `clear_fields` (email, geo, opening_hours, phone) bleibt unverändert, `status: pruefung` bleibt
  unverändert, es wird weder Fortbestand noch Annahme behauptet.
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/284230977/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.4200,51.3400,7.5600,51.4200 ; https://dieausmister.de/hagen ;
  https://www.facebook.com/DieAusmister/ (Abruf 03.11.2026).]

### Recherche 23.11.2026 (Welle 0600 A)

- [Recherche 23.11.2026 (Welle 0600 A): **Die Betreiberseite ist neu aufgebaut und hat jetzt ein echtes Impressum mit Rechtsträgerin und Umsatzsteuer-Identifikationsnummer - die offene Frage der Betreiberidentität ist damit beantwortet.** Die Seite https://dieausmister.de/hagen/ wurde am 23.11.2026 live gelesen und ist gegenüber dem in der Vorwelle vom 24.10.2026 beschriebenen Stand **inhaltlich neu** (Astro-Website mit Kontaktformular, Leistungs- und Ortsseiten). Das Impressum https://dieausmister.de/impressum/ (live am 23.11.2026 gelesen) nennt wörtlich: "Angaben gemäß § 5 DDG - Die Ausmister, **Inhaberin: Elheme Sykaj**, Wehringhauser Straße 60, 58089 Hagen", dazu **Telefon 02331-9109428**, **E-Mail info@dieausmister.de** und die **Umsatzsteuer-Identifikationsnummer gemäß § 27a UStG: DE274573289**; ferner den Hinweis, dass der Betrieb nicht bereit und nicht verpflichtet ist, an Streitbeilegungsverfahren vor einer Verbraucherschlichtungsstelle teilzunehmen. **Damit liegt erstmals eine rechtlich belastbare Betreiberangabe vor** (Inhaberin mit Vorname, steuerliche Identifikation, ladensässige Anschrift) - die frühere Feststellung "eine rechtliche Betreiberangabe ist nicht sichtbar" ist **überholt**. **Zwei Quellen:** die Betreiberseite (Impressum, Inhaberin, USt-IdNr., Anschrift, Telefon, E-Mail) und das auf derselben Seite verlinkte Bewertungsportal (Trustlocal-Profil "Hagen Die Ausmister Entrümpelung", https://trustlocal.de/nordrhein-westfalen/hagen/entrumpelung/hagen-die-ausmister-entr%C3%BCmpelung/) bestätigen Name und Ort. **Zurückhaltung bei den Feldern:** `email`, `phone` und `opening_hours` sind im Frontmatter weiterhin über `clear_fields: 'email,geo,opening_hours,phone'` zurückgenommen - die Betreiberangabe allein hebt diese Owner-Direktive nicht auf; **die Betreiberidentität ist damit geklärt, die Feld-Füllung bleibt eine Owner-Entscheidung**. Quellen: https://dieausmister.de/impressum/ ; https://dieausmister.de/hagen/ ; https://trustlocal.de/nordrhein-westfalen/hagen/entrumpelung/hagen-die-ausmister-entr%C3%BCmpelung/ (Abruf 23.11.2026).]
- [Recherche 23.11.2026 (Welle 0600 A): **Der Betrieb bewirbt jetzt selbst eine Fachkundeneigenschaft nach § 9 EfbV und § 54 KrWG und beschreibt die Entsorgung als über zugelassene Betriebe aus der Region - ein Schrotthandels-Metallankauf ist damit weiterhin nicht belegt.** Die Startseite nennt in der Kopfzeile "**Fachkunde § 9 EfbV · § 54 KrWG**" und im Abschnitt "Familie aus Hagen statt Callcenter" wörtlich "**Entsorgt wird fachgerecht über zugelassene Betriebe aus der Region.**" Der Betrieb beschreibt weiterhin **Entrümpelung, Haushaltsauflösung, Gewerbeauflösung, Keller-/Dachboden-/Garagenräumung und Abbruchservice**; er nennt einen **eigenen Laden in Wehringhausen**, in dem "Was noch Wert hat, verkaufen wir in unserem Laden ... weiter - und rechnen es auf Ihren Preis an", sowie "Verwertbare Möbel und Antiquitäten rechnen wir an - das senkt Ihren Preis". **Das ist eine Wertanrechnung für Räumungsaufträge, kein Metallankauf**, und eine **Eigenentsorgung über zugelassene Betriebe** - der Betrieb selbst ist damit **kein** Entsorgungsfachbetrieb und **kein** Schrottplatz. **Die Zertifikatsaussage "§ 9 EfbV · § 54 KrWG" ist Betreiberwerbung ohne Zertifikatsnachweis** und wird nicht als Zertifizierung geführt; `certifications` bleibt unverändert leer. Das amtliche Fachbetrieberegister (clientseitig gerenderte Vaadin-App, Filter "Art des Betriebs = Egal", is=1&e=1) liefert am 23.11.2026 in der Route "Entsorgungsfachbetriebe" für die Namensvarianten **"Ausmister"**, **"Sykaj"** und **"Elheme"** (der im Impressum genannte Inhaberinnenname) **je 0 Treffer**, in der Route "Altfahrzeugverwertung" für "Sykaj" ebenfalls **0 Treffer**. Das ist mit der Betreiberangabe "Entsorgt wird fachgerecht über zugelassene Betriebe aus der Region" konsistent: der Betrieb **selbst** ist kein zertifizierter Entsorgungsfachbetrieb. **Reichweite:** die Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind **freiwillig**, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis und **kein** Beleg gegen die Bewerbung mit "§ 9 EfbV"; `certifications` bleibt unverändert leer. Quellen: https://dieausmister.de/hagen/ ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Ausmister (Abruf 23.11.2026).]
- [Recherche 23.11.2026 (Welle 0600 A): **GEO: die adressgenauen Angaben bleiben unverändert, und es gibt weiterhin keinen Betriebsbezug an der Hausnummer 60.** Read-only-Abgleich `public.db` (Abruf 23.11.2026): **kein gespeicherter Standortpunkt** (der Token `geo` in `clear_fields` war bereits wirksam). Das Dossier-Adressobjekt **way/284230977** trägt unverändert `addr:housenumber`=60, `addr:street`=**Wehringhauser Straße**, `addr:city`=Hagen, `addr:postcode`=**58089**, `building`=yes, `name`=**Antik Eck** (Mittelwert der 12 eindeutigen Way-Knoten **51.3554606/7.4543859**, BBox 51.3553570/7.4542209 bis 51.3555316/7.4546032). **Die Adresse ist damit durch die Betreiber-Primärquelle (Impressum mit Inhaberin) belegt, das OSM-Objekt trägt aber weiterhin den Fremdnamen "Antik Eck"** und damit keinen Bezug zu den Ausmister; eine Bestätigung der Ausmister an dieser Hausnummer ist damit **nicht** erbracht. **Der Punkt wird hier nicht gesetzt** - über Setzen oder Entfernen entscheidet der Owner in einer eigenen datierten Korrektur-Sektion; mit der jetzt vorliegenden Betreiberidentität ist die Hausnummer 60 der adresskonforme Kandidat. Quellen: https://api.openstreetmap.org/api/0.6/way/284230977/full.json ; read-only `public.db` (Abruf 23.11.2026).]
- [Recherche 23.11.2026 (Welle 0600 A): **Preise strikt getrennt: die veröffentlichten Beträge sind Raum-Entrümpelungs-Preise, keine Schrott-, Ankauf- oder Gebührenpreise.** Die neue Preisseite https://dieausmister.de/preise/ (live am 23.11.2026 gelesen) nennt unter der Überschrift "Richtpreise zur Orientierung (Stand 2026)" und mit dem ausdrücklichen Zusatz "Den verbindlichen Festpreis nennen wir Ihnen direkt bei der Besichtigung": Kellerraum **ab 250 €** (ca. 12 m², 200 kg Holz, 10 Säcke Abfall, 4 Autoreifen, 1 Fahrrad), Kleine Wohnung **ab 899 €** (ca. 40 m²), Wohnung **ab 1.200 €** (ca. 60 m²), Haus **ab 1.700 €** (ca. 85 m²). Im Festpreis enthalten sind laut Seite "Anfahrt und Besichtigung (kostenlos)", "Personal und Fahrzeuge", "Demontage und Heraustragen", "**Abtransport und fachgerechte Entsorgung inkl. Deponiegebühren**" und "besenreine Übergabe"; die Preise steigen laut Seite mit "Menge", "Zugang" und "Art der Abfälle" und werden durch "Wertanrechnung" verwertbarer Möbel gesenkt. **ANKAUF:** kein Metallankauf und kein Schrottpreis; die Wertanrechnung ist eine **Anrechnung auf den Entrümpelungspreis**, kein Ankauf von Schrott gegen Bar. **VERKAUF:** kein Verkauf von Schrott oder Metall; der eigene Laden in Wehringhausen verkauft "Was noch Wert hat" (Möbel, Antiquitäten) im Rahmen der Räumung - das ist **keine** Verkaufspreisliste für Metallschrott. **GEBÜHREN:** **kein** Tarif für die reine Schrottabgabe; die "Deponiegebühren" sind laut Seite **im Festpreis enthalten** und damit keine separat berechnete Gebühr. Keine dieser Beträge wird als Ankauf-, Verkaufs- oder Gebührentafel des Dossierbetriebs in ein Preisfeld übernommen. Quellen: https://dieausmister.de/preise/ ; https://dieausmister.de/hagen/ (Abruf 23.11.2026).]

### Korrektur 23.11.2026 (Owner, Geokodierung) — der Standortpunkt bleibt unverändert

- [Korrektur 23.11.2026: **Für dieses Dossier wird **kein** Standortpunkt gesetzt ✓ ✓ — Es gibt **keinen** gespeicherten Punkt ✓✓ und **keinen** Betriebsbezug ✓✓ an der Hausnummer **60** ✓✓ — es wird **nichts** geokodiert ✓ ✓ ✓ ✓ **Reichweite:** Die EfbV-/AltFahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Wellennotiz mit der Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 23.11.2026 ✓ ✓), **zweitens** ✓ ✓ das **Adressobjekt** ✓ ✓ in OpenStreetMap ✓ ✓ ✓ ✓]
