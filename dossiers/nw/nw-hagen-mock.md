---
slug: nw-hagen-mock
name: Mock
trader_type: schrotthaendler
state: NW
city: Hagen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
clear_fields: 'geo'
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Mock

## Überblick

Mehrere Branchenverzeichnisse führen **Stefan Mock / Schrotthandel- und Containerdienst Inh. Stefan Mock**, Neue Straße 2, 58135 Hagen-Haspe, Telefon 02331 3779915. Die direkt geöffnete Domain `schrotthagen.de` bewirbt zwar Schrottabholung und Schrottankauf in Hagen und Umgebung, nennt aber weder Stefan Mock noch die Neue Straße oder einen dazu passenden Rechtsträger; ihr Copyright weist 2023 aus. Die Domain bleibt deshalb unverknüpft und beweist weder Mock-Identität noch aktuelle Tätigkeit.

**Preise:** Ankauf: `schrotthagen.de` wirbt allgemein mit „fairen“/„Höchstpreisen“, nennt aber keine Zahlen und ist Mock nicht zugeordnet. Verkauf: keine Mock-Liste. Gebühren: keine Mock-Tarife; „kostenlose Schrottabholung“ auf der fremden/unzugeordneten Domain nicht übertragen. Neue Straße 2 bleibt Lead und kein GEO-Ziel, bis Betreiberbezug und Kundennutzung bestätigt sind.

## Timeline

### Korrektur 24.10.2026: Gemeinsamen falschen Hagen-Punkt zurückziehen

- [Korrektur 24.10.2026: Die Produktionszeile `nw-hagen-mock` enthielt den mit mehreren anderen Hagen-Datensätzen geteilten Punkt `51.90933, 7.88825`. Die read-only-Rückwärtssuche löst ihn in Mehringen, Everswinkel, Kreis Warendorf, PLZ 48351 auf, nicht in Hagen. `clear_fields: 'geo'` zieht den falschen Punkt zurück; mangels belegter Händleradresse wird keine Ersatzkoordinate gesetzt. Die Produktionsdatenbank blieb unverändert. Quelle: https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; https://www.openstreetmap.org/way/81516036.]

### Recherche 13.10.2026

- [Recherche 13.10.2026: Neue/aktuelle Aggregator-Treffer zu Stefan Mock nennen Schrotthandel- und Containerdienst, Neue Str. 2, 58135 Hagen-Haspe, 02331 3779915. Diese Treffer sind keine Betreiberquelle oder unabhängige Registerbestätigung. `schrotthagen.de` wurde direkt geöffnet; die allgemeine Hagen-Schrottseite weist keinen Namens-/Adress-/Impressumsbezug zu Stefan Mock auf und bleibt unverknüpft. Ankauf: durch Verzeichnisse behauptet, keine Preise; Verkauf: keine Liste; Gebühren: keine Tarife. Keine Kontaktdatenfüllung und kein GEO auf Basis der Aggregatoren. Quellen: https://www.11880.com/suche/schrotthandel/hagen ; https://www.schrottplatz-info.de/schrottplatz/Hagen ; https://schrotthagen.de/ ; Abruf 13.10.2026.]

### Importiert (Seed-Stand 2026-09-30)

- Schrott, Ankauf fraglich (GS)
- Adresse: Hagen

### Recherche 22.10.2026

- [Recherche 22.10.2026: `schrotthagen.de` direkt geöffnet. Die Seite bewirbt Schrottankauf/-abholung für Hagen und Umland, nennt aber weder Stefan Mock, Neue Straße 2 noch einen zuordenbaren Rechtsträger; Copyright 2023. Das ist kein Beleg für eine Verbindung oder aktuelle Leistung von Mock. Verzeichnisangaben bleiben Leads; keine strukturierten Felder oder GEO übernommen. ANKAUF: keine Mock-spezifischen Kurse; VERKAUF: keine Liste; GEBÜHREN: keine Mock-spezifischen Tarife. Quelle: https://schrotthagen.de/]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Lead-Adresse **Neue Str. 2,
  58135 Hagen-Haspe** ist im live abgerufenen OSM-Bestand **nicht** erfasst, und die Adresse stützt sich allein auf
  **Aggregator-Einträge**. **Was live geprüft wurde (03.11.2026):** (a) Eine Suche über alle Tag-Werte nach
  `addr:street`=**Neue Straße** im Kachelraster **7.4200/51.3400 – 7.5600/51.4200** (112 Kacheln à 0,01 Grad, live
  abgerufen über `https://api.openstreetmap.org/api/0.6/map.json?bbox=…`, **0 Kachel-Fehler**) und in den zusätzlich
  abgerufenen Haspe-Kacheln **7.4000/51.3400 – 7.4400/51.3600** liefert **0** Adressobjekte und
  **0** Hausnummern; auch ein Objekt mit `name`=Neue Straße existiert im Raster nicht — in Hagen-Haspe sind statt
  dessen u. a. die Straßen Tillmannsstraße, Hestertstraße, Kölner Straße, Berliner Straße, Voerder Straße, Haenelstraße,
  Swolinzkystraße, Steinstraße und Am Romberg erfasst; die einzige Adressobjekt-Treffer mit „Neue“ im Raster betrifft
  **Neuer Mühlenweg** (1, 2, 3, 4, 5, 57), also eine **andere** Straße. (b) Eine Suche über `name`, `operator`, `brand`,
  `website`, `contact:website`, `description`, `note` und `short_name` nach **„Mock“** und **„Stefan Mock“** im
  Kachelraster **7.4200/51.3400 – 7.5600/51.4200** (112 Kacheln, 0 Kachel-Fehler) liefert **0** Treffer mit
  Betriebsbezug — Aussage zum OSM-Bestand, **kein** Negativbeleg über den Betrieb. Quelle 1 (Aggregator, **keine**
  Betreiber-/Registerquelle): die aktuellen Verzeichnistreffer zu **Stefan Mock** nennen **Schrotthandel- und
  Containerdienst**, **Neue Str. 2, 58135 Hagen-Haspe**, **02331 3779915** — das sind Aggregator-Treffer und keine
  Betreiberquelle; `schrotthagen.de` nennt weder Stefan Mock noch die Neue Straße 2 noch einen zuordenbaren
  Rechtsträger (Copyright 2023) und bleibt unverknüpft. Quelle 2 (OSM): die Dossier-Straße und ihre Hausnummer
  existieren im live abgerufenen Raster **nicht**, ein Objekt mit Betriebsbezug ebenfalls nicht. **Was fehlt:** ein
  adressgenauer OSM-Punkt **und** eine Betreiber- oder Registerquelle, die „Mock“ an die Neue Straße 2 bindet; erst
  damit wäre die Zeile geokodierbar. `clear_fields: 'geo'` bleibt bestehen, `status: pruefung` bleibt unverändert, es
  wird weder Fortbestand noch Schließung behauptet.
  Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=7.4200,51.3400,7.5600,51.4200 ;
  https://www.11880.com/suche/schrotthandel/hagen ; https://www.schrottplatz-info.de/schrottplatz/Hagen ;
  https://schrotthagen.de/ (Abruf 03.11.2026).]

### Recherche 08.11.2026

- [Recherche 08.11.2026: **Das Impressum von schrotthagen.de ist erstmals lesbar und benennt einen anderen Betreiber: Inhaber Mohammed El-lahib, Ahlmannshof 42, 45889 Gelsenkirchen; damit ist die Domain jetzt positiv von „Stefan Mock“ getrennt und nicht bloß nicht zugeordnet.** Die Seite https://www.schrotthagen.de/impressum.html wurde am 08.11.2026 live gelesen (der Pfad /impressum antwortet mit HTTP 404, die Seite ist unter /impressum.html verlinkt); sie nennt unter „Pflichtangaben nach § 5 TMG“: „schrotthagen.de, Inhaber: Mohammed El-lahib, Ahlmannshof 42, 45889 Gelsenkirchen, Internet: https://www.schrotthagen.de, E-Mail: info@schrotthagen.de, Telefon: 0152 / 13 65 38 54“ sowie „© Copyright 2023 schrotthagen.de“. Die Startseite nennt weder „Stefan Mock“ noch die Neue Straße 2 und wirbt mit „Kostenlose Schrottabholung“, „Schrottankauf bei größeren Mengen“ und „Barzahlung und Höchstpreise“; die Domain bleibt damit ein **anderer** Betrieb und wird **nicht** als Quelle für diesen Seed geführt. Beobachtung ohne Wertung: der Nachname „El-lahib“ taucht auch bei den Betreibernamen der Dossiers nw-nrw-weit-schrott-ankauf-nrw-0163 (Mahmoud El-Lahib, Bochum) und nw-nrw-weit-01522-schrottankauf-netzwerk-hamm-hagen (A. Lahib, Hamm/Bonn/Krefeld) auf; ein Betreiberverbund ist damit **nicht** belegt, die Rufnummern (0152 / 13 65 38 54 gegenüber 0163 / 20 49 842 und 01522-422-4277) unterscheiden sich. Quelle(n): https://www.schrotthagen.de/impressum.html ; https://www.schrotthagen.de/ ; https://www.schrotthagen.de/impressum (Abruf 08.11.2026).]
- [Recherche 08.11.2026: **Der Negativbefund vom 03.11.2026 zur Straße „Neue Straße“ in Hagen war ein Fensterfehler: die Anschrift existiert in OSM, liegt aber im Stadtteil Westerbauer und nicht in Haspe.** Das am 08.11.2026 live abgerufene OSM-Objekt way/291621547 trägt addr:housenumber=2, addr:street=Neue Straße, addr:city=Hagen, addr:postcode=58135 und building=yes; die Objekt-BBox reicht von 51.3390596/7.4012495 bis 51.3392592/7.4016969, der Nominatim-Treffer weist den Ortsteil **Westerbauer** aus. Das am 03.11.2026 abgerufene Kachelraster 7.4200/51.3400 bis 7.5600/51.4200 liegt sowohl in der Länge (7.4012 liegt westlich von 7.4200) als auch in der Breite (51.3391 liegt südlich von 51.3400) **außerhalb** dieses Objekts; die dortige Aussage, es existiere keine Adresse „Neue Straße“ in Hagen, galt damit nur für das abgetastete Fenster. Das Objekt trägt weder `name=` noch `operator=`, es belegt also die Anschrift, **nicht** einen Schrottbetrieb und **nicht** die Zuordnung des Verzeichnisleads „Neue Str. 2, 58135 Hagen-Haspe“; die Frage, ob der Lead die Hasper oder die Westerauer Adresse meint, bleibt offen, und `clear_fields: 'geo'` bleibt bestehen. Das amtliche Fachbetrieberegister kennt den Namen nicht: die Namenssuche n=Mock liefert in beiden Routen (Filter „Art des Betriebs = Egal“, is=1&e=1) 0 Treffer, die Standortabfrage um Neue Straße 2, 58135 Hagen (Punktlage 51.3391562/7.4014796) im 3-km-Radius 0 Treffer in der Route „Entsorgungsfachbetriebe“ und 0 Treffer in der Route „Altfahrzeugverwertung“. Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein Nachweis fehlender Anerkennung und kein Existenz- oder Schließungsbeweis. Quelle(n): https://api.openstreetmap.org/api/0.6/way/291621547/full.json ; https://nominatim.openstreetmap.org/search?q=Neue+Stra%C3%9Fe+2%2C+58135+Hagen&format=jsonv2&addressdetails=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Mock ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Mock ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=3&is=1&e=1&adr=Neue%20Stra%C3%9Fe%202%2C%2058135%20Hagen%3B7.341480%3B51.399156%3B7.461480%3B51.279156%3B7.341480%3B51.279156 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=3&is=1&e=1&adr=Neue%20Stra%C3%9Fe%202%2C%2058135%20Hagen%3B7.341480%3B51.399156%3B7.461480%3B51.279156%3B7.341480%3B51.279156 (Abruf 08.11.2026).]
- [Recherche 08.11.2026: **Preise getrennt geführt: für Stefan Mock ist am 08.11.2026 kein Ankaufskurs, keine Verkaufsliste und keine Gebührenangabe belegt.** Die Werbung „Kostenlose Schrottabholung“, „Schrottankauf bei größeren Mengen“, „Barzahlung und Höchstpreise“ und „fairer Schrottankauf“ auf schrotthagen.de steht ohne jeden Betrag und gehört nach dem Impressumsbefund zu einem **anderen** Betrieb; sie wird diesem Seed **nicht** zugerechnet. **Ankauf:** kein €/kg-Kurs für Mock belegt. **Verkauf:** keine Verkaufsliste. **Gebühren:** keine Tarife und keine belegte Kostenfreiheit für Mock. `status` bleibt `pruefung`. Quelle(n): https://www.schrotthagen.de/ ; https://www.schrotthagen.de/impressum.html (Abruf 08.11.2026).]

### Recherche 17.11.2026

- [Recherche 17.11.2026: **Die Standortfrage ist jetzt abschließend geklärt: Es gibt in Hagen nur EINE „Neue Straße" — sie liegt im Stadtteil Westerbauer (nicht Haspe) — und die Hausnummer 2 ist in OSM als Wohngebäude erfasst; ein Schrottbetrieb ist dort nicht kartiert.** Live am 17.11.2026 geprüft: der OSM-Kachelraster **bbox=7.3950/51.3320 bis 7.4150/51.3470** (live abgerufen über https://api.openstreetmap.org/api/0.6/map.json?bbox=7.3950,51.3320,7.4150,51.3470) enthält **30 Adressobjekte** der „Neue Straße" mit den Hausnummern 2, 4, 6, 7, 8, 10, 11, 11a, 12, 14, 15, 17, 17a, 19, 21, 23, 24, 26, 27, 27a, 27b, 28, 28a, 28b, 28d, 29, 30, 30a, 31, 34, alle mit `addr:street`=Neue Straße, `addr:postcode`=58135, `addr:city`=Hagen. Die eingetragenen Betriebsnamen an dieser Straße sind **Lemmer GmbH & Co. KG** (Nr. 34, `building`=industrial), **Comhan Holland BV** (Nr. 23, industrial), **AWO Treff Westerbauer** (Nr. 17a, industrial) und **FWS GmbH & Co. KG** (Nr. 28, industrial) — **kein** Schrott- oder Recyclingbetrieb und **kein** „Mock". Die beiden weiteren live geprüften Kachelraster **bbox=7.4150/51.3470 bis 7.4350/51.3600** (Haspe) und **bbox=7.4000/51.3450 bis 7.4400/51.3700** sowie **bbox=7.3800/51.3400 bis 7.4100/51.3600** enthalten **kein** Adressobjekt und **keine** Straße mit „Neue" im Namen; die Nominatim-Suche „Neue Straße, 58135 Hagen" liefert ausschließlich Treffer in **Westerbauer** (Neue Straße Westerbauer; Neue Straße Quambusch/Westerbauer; Neue Straße Martinstraße/Quambusch). **Das Objekt way/291621547 (Hausnummer 2)** liegt bei **51.3391945/7.4015370**, trägt `building`=**yes** und **weder `name` noch `operator`** — es belegt die **Anschrift**, **nicht** einen Betrieb und **nicht** eine Kundenannahme. **Folgerung für die Zuordnung:** der Verzeichnislead „Neue Str. 2, 58135 Hagen-**Haspe**" nennt eine **falsche oder veraltete Stadtteilangabe**; die Hausnummer existiert nur in Westerbauer. Die im Vermerk vom 08.11.2026 offene Frage „Hasper oder Westerauer Adresse" ist damit **beantwortet: es gibt nur eine „Neue Straße" in Hagen, in Westerbauer** — damit ist die Anschrift als solche plausibel, ihre **Betreiberbindung** an „Schrotthandel- und Containerdienst Inh. Stefan Mock" bleibt aber **unbelegt**, da die Adresse nur aus **Aggregator-Einträgen** stammt. `clear_fields: 'geo'` bleibt bestehen, `status` bleibt `pruefung`, es wird **kein** Ersatzpunkt gesetzt. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=7.3950,51.3320,7.4150,51.3470 ; https://api.openstreetmap.org/api/0.6/map.json?bbox=7.4150,51.3470,7.4350,51.3600 ; https://nominatim.openstreetmap.org/search?format=jsonv2&limit=10&q=Neue+Stra%C3%9Fe%2C+58135+Hagen ; https://api.openstreetmap.org/api/0.6/way/291621547/full.json (Abruf 17.11.2026).]
- [Recherche 17.11.2026: **Die Negativaussage zum amtlichen Fachbetrieberegister ist am Primärregister geprüft: die Namenssuche „Mock" liefert 0 Treffer in beiden Routen, und die Standortabfrage um die Westerauer Adresse liefert 14 Efb- und 1 AfV-Treffer im 3-km-Radius, keinen mit dem Namen.** Live-Abfrage am 17.11.2026 (clientseitig gerenderte Vaadin-App, Filter „Art des Betriebs = Egal", is=1&e=1, Ergebnisgrid vollständig durchscrollt): die **Namenssuche „Mock"** liefert in **beiden** Routen (Entsorgungsfachbetriebe, Altfahrzeugverwertung) **0 Treffer**. Die **Standortabfrage um Neue Straße 2, 58135 Hagen** (Punktlage 51.3391562/7.4014796, die Karte der Westerauer Adresse) liefert im **3-km-Radius 14** Efb-Treffer, darunter **TOI TOI & DIXI Sanitärsysteme GmbH Betriebsstätte Hagen**, Neue Straße 19-21, 58135 Hagen, 0,3 km (ZZET02000072 / 6 Sammeln, Befördern, Lagern) und **Grün & Mücher GmbH & Co. KG**, Asker Straße 6, 58285 Gevelsberg, 0,8 km (ZZEE00300193 / 1 bis / 3) — **andere Rechtsträger**, **nicht** übernommen; die Route „Altfahrzeugverwertung" liefert **1 Treffer** (**Iyawe Autoteilehandel**, Am Sinnerhoop 77, 58285 Gevelsberg, 0,8 km, ZAES01100078 / 1), ebenfalls fremd. **Reichweite:** die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**, ein Register-Nichteintrag beweist **weder** die fehlende Anerkennung **noch** eine Nichtexistenz; geprüft wurden die Namensvariante „Mock" sowie die Standortsuche im 3-km-Radius um die Dossier-Adresse. Ein Zertifikat für Stefan Mock ist damit **nicht ausgewiesen**; `certifications` bleibt unverändert. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Mock ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Mock ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Neue%20Stra%C3%9Fe%202%2C%2058135%20Hagen%2C%20Deutschland%3B7.4014796%3B51.3391562%3B7.4780000%3B51.3689000%3B7.3250000%3B51.3094000&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Neue%20Stra%C3%9Fe%202%2C%2058135%20Hagen%2C%20Deutschland%3B7.4014796%3B51.3391562%3B7.4780000%3B51.3689000%3B7.3250000%3B51.3094000&dst=3&is=1&e=1 (Abruf 17.11.2026).]
- [Recherche 17.11.2026: **Preise getrennt geführt: für Stefan Mock ist kein Ankaufskurs, keine Verkaufsliste und keine Gebührenangabe belegt; die Werbung der Fremddomain schrotthagen.de wird nicht übertragen.** Die Seite schrotthagen.de (live abgerufen am 17.11.2026) wirbt mit „Kostenloser Schrottabholung", „Schrottankauf bei größeren Mengen", „Barzahlung und Höchstpreise" und „fairer Schrottankauf" — **ohne jeden Betrag**; nach dem Impressumsbefund vom 08.11.2026 gehört sie zum **anderen** Betrieb (Inhaber Mohammed El-lahib, Ahlmannshof 42, 45889 Gelsenkirchen) und wird diesem Seed **nicht** zugerechnet. **Ankauf:** kein €/kg-Kurs für Mock belegt. **Verkauf:** keine Verkaufsliste. **Gebühren:** keine Tarife und keine belegte Kostenfreiheit für Mock. Die beiden Verzeichnisleads (Schrotthandel- und Containerdienst Inh. Stefan Mock, Neue Str. 2, 58135 Hagen-Haspe, 02331 3779915) bleiben Aggregator-Leads ohne Betreiber- oder Registerbestätigung. `status` bleibt `pruefung`, `clear_fields: 'geo'` bleibt bestehen. Quelle(n): https://www.schrott-hagen.de/ ; https://www.schrott-hagen.de/impressum.html ; https://www.11880.com/suche/schrotthandel/hagen ; https://www.schrottplatz-info.de/schrottplatz/Hagen (Abruf 17.11.2026).]
