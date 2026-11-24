---
slug: rp-koblenz-56073-r-a
clear_fields: 'geo'
name: Schrotthandel R&A
trader_type: schrotthaendler
state: RP
city: 'Koblenz'
street: Moselweißer Str. 20
postcode: 56073
phone: 0176 65073477
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# R&A

## Überblick

Der Seed „Schrotthandel R&A" ist durch zwei unabhängige Verzeichnisse gedeckt: Gelbe Seiten (Detailseite direkt gelesen) führt „Schrotthandel R&A, Moselweißer Str. 20, 56073 Koblenz-Rauental, 0176 65 07 34 77, Branche Schrott", schrottplatz.org führt „Schrott Schrotthandel. R&A" an derselben Anschrift. Beides sind Aggregator-Leads; eine Betreiber-Website, ein Impressum oder ein Registereintrag existiert nicht, der Status bleibt `pruefung`. Der in der Produktionszeile gespeicherte Koordinatenpunkt gehört nachweislich zu einem **anderen** Koblenzer Betrieb: Er löst auf „Devorastraße 2, Goldgrube, 56073 Koblenz" auf, und schrottplatz.org führt unter dieser Anschrift „Schrottplatz 1A Schrotthandel & Entrümpelungen". `clear_fields: 'geo'` zieht den fremden Punkt zurück.

**ANKAUF:** keine Kurse und keine Materialangaben. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife.

## Timeline

### Recherche 27.10.2026: Zweites Verzeichnis bestätigt Anschrift, gespeicherter Punkt gehört zu anderem Betrieb

- [Recherche 27.10.2026: Die Gelbe-Seiten-Detailseite wurde direkt gelesen und nennt „Schrotthandel R&A", „Moselweißer Str. 20, 56073 Koblenz-Rauental", „0176 65 07 34 77", Branche „Schrott", ohne Website und ohne E-Mail. Das Schrottplatz-Portal führt unabhängig „Schrott Schrotthandel. R&A" mit derselben Anschrift „Moselweißer Str. 20, 56073 Koblenz". Damit liegen die bereits eingetragenen Werte street, postcode und phone auf zwei unabhängigen Verzeichnisquellen; es gibt weiterhin keine Betreiber-Primärquelle und kein Register, der Status bleibt `pruefung`, E-Mail, Öffnungszeiten und Website bleiben leer. Quellen: https://www.gelbeseiten.de/gsbiz/f3e15f7e-7279-4692-86b1-1504b98cf884 (direkt abgerufen) ; https://www.schrottplatz.org/koblenz/schrott-schrotthandel-ra-aYEPwO.html .]
- [Recherche 27.10.2026: GEO-Korrektur mit Namensvetter-Bezug: Der gespeicherte Punkt 50.3540142857143, 7.5774 löst per Nominatim-Reverse auf „2, Devorastraße, Goldgrube, Koblenz, 56073" auf. Dieselbe Anschrift führt im Schrottplatz-Portal der Eintrag „Schrottplatz 1A Schrotthandel Koblenz, Devorastraße 2, 56073 Koblenz", der als „Schrottplatz 1A Schrotthandel & Entrümpelungen" geführt wird. Der gespeicherte Punkt gehört damit zu einem anderen Koblenzer Betrieb und nicht zu „Schrotthandel R&A" (Moselweißer Str. 20). `clear_fields: 'geo'` entzieht das Paar beim regulären Seed; das bedeutet nur „keine kuratierte aktuelle Koordinate", keine Schließung und keine Annahmeverweigerung. Eine Ersatzkoordinate wurde nicht geraten, weil für Moselweißer Straße 20 nur eine Verzeichnisangabe vorliegt und kein Nachweis einer öffentlichen Annahmestelle. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.3540142857143&lon=7.5774&zoom=18&addressdetails=1 ; https://www.schrottplatz.org/koblenz/schrottplatz-1a-schrotthandel-koblenz-aYEOYK.html ; https://nominatim.openstreetmap.org/search?q=Moselwei%C3%9Fer%20Stra%C3%9Fe%2020%2C%2056073%20Koblenz&format=jsonv2&addressdetails=1&limit=3 ; read-only-Abgleich `public.db` (27.10.2026).]
- [Recherche 27.10.2026: **ANKAUF:** keine Kurse, keine Materialangaben; die Verzeichniseinträge nennen nur die Branche „Schrott". **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife; die genannten Leistungen sind Schrott/Schrottankauf/Schrottentsorgung, ohne Bezifferung. Abgrenzung: Die übrigen im Gelbe-Seiten-Umfeld geführten Koblenzer Betriebe („Koblenzer Schrotthandel", Am Metternicher Bahnhof 18, 56072; „SCHROTTHANDEL 56", Kastorstr. 12, 56068; „Kreutz Schrotthandel", Dominicusstr. 43, 56073; „Schrottplatz 1A", Devorastraße 2, 56073; „Schrott Alten Angelo", Peter-Friedhofen-Str. 9, 56073) sowie die Betreiberauftritte Koblenzer Schrotthandel (koblenzer-schrotthandel.de), PMS Schrotthandel, TSR Deutschland und Schrotthandel NCM sind eigenständige Betriebe und wurden nicht übernommen. Quellen: https://www.gelbeseiten.de/branchen/schrotth%C3%A4ndler/koblenz%20am%20rhein ; https://www.koblenzer-schrotthandel.de/ ; https://www.tsr-recycling.de/standorte/schrotthaendler-koblenz .]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Adresse Moselweißer Str. 20, 56073 Koblenz-Rauental + Tel. 0176 65073477 + Leistungen Schrott/Schrottankauf/-entsorgung aus Gelbe-Seiten-Filialseite (Branchenliste Koblenz + Detailseite gsbiz). Einzige Quelle ein Aggregator (zwei Seiten derselben Website = EINE Quelle): Ausnahme-Fill (offen dokumentiert): name präzisiert zu Schrotthandel R&A, street, postcode, phone; website_status unbekannt (kein Betreiber-Webauftritt, kein HR-Beleg). Kein Bezug zu R und S Laroche GbR (Mülheim-Kärlich) oder anderen Koblenzer Betrieben hergestellt, kein Merge. Status bleibt pruefung; Quelle(n): gelbeseiten.de (Branchenseite + Detailseite, nur Lead)]

### Importiert (Seed-Stand 2026-09-30)

- Schrott
- Adresse: Koblenz 56073

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Hausnummer **20** ist in OSM auf der
  Dossier-Straße **Moselweißer Straße** zwar **eindeutig** belegt, aber **es fehlt die zweite unabhängige Quelle**:
  die Dossier-Adresse stammt ausschließlich aus **zwei Verzeichnissen**, und nach der README-Quellenhierarchie sind
  Verzeichnisse ausdrücklich **Leads, nie Evidenz** - zwei Verzeichnisse sind zudem **keine** zwei voneinander
  unabhängigen Quellen im Sinne der Zwei-Quellen-Regel. **Was geprüft wurde (live am 03.11.2026):** (a) Die Straße
  **Moselweißer Straße** in Koblenz trägt in OSM **95 Adressobjekte** mit den Hausnummern **1, 2b, 3, 4, 5, 6, 7,
  8, 10, 12, 16, 18, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 30a, 31, 32, 33, 34, 34a, 36, 37, 38, 40, 42-46,
  43, 45, 47, 47a, 48, 48-52, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 66 A, 67,
  68, 69, 70, 71, 73, 75, 77, 78, 79, 81, 83, 85, 101-103, 105, 107, 115, 116, 117-119, 121, 122, 123, 124, 125,
  192, 194** - die **Hausnummer 20 ist genau einmal** belegt und damit eindeutig: **node/13307227200** mit
  `addr:housenumber`=**20**, `addr:street`=**Moselweißer Straße**, `addr:city`=Koblenz, `addr:postcode`=**56073**,
  `addr:suburb`=**Rauental** (Kachelraster über **bbox=7.5450,50.3350,7.6050,50.3750**, live abgerufen 03.11.2026).
  (b) Eine Suche über **alle** Tag-Werte nach „R&A“ bzw. einem Schrottbetrieb im Raster liefert **0 Objekte**, die
  **den eigenen Betrieb** nennen - der Betrieb ist in OSM **nicht namentlich kartiert**; der Knoten trägt **weder
  `name=` noch `operator=`**. **Quelle 1 (Verzeichnis):** Die **Gelbe-Seiten-Detailseite** (live abgerufen
  03.11.2026, HTTP 200, https://www.gelbeseiten.de/gsbiz/f3e15f7e-7279-4692-86b1-1504b98cf884) führt
  „**Schrotthandel R&A**“ mit „**Moselweißer Str. 20, 56073 Koblenz-Rauental**“, Telefon **0176 65 07 34 77**,
  Branche „Schrott“; das **Schrottplatz-Portal** (live abgerufen 03.11.2026, HTTP 200,
  https://www.schrottplatz.org/koblenz/schrott-schrotthandel-ra-aYEPwO.html) nennt denselben Betrieb mit derselben
  Anschrift. Beides sind **Verzeichnisse**; eine **Betreiberseite**, ein **Impressum** oder ein **Registereintrag** zu
  „Schrotthandel R&A“ existiert **nicht** - nach der Recherche vom 27.10.2026 sind die übrigen im Gelbe-Seiten-
  Umfeld geführten Koblenzer Betriebe („Koblenzer Schrotthandel“ Am Metternicher Bahnhof 18, „SCHROTTHANDEL 56“
  Kastorstr. 12, „Kreutz Schrotthandel“ Dominicusstr. 43, „Schrottplatz 1A“ Devorastraße 2, „Schrott Alten Angelo“
  Peter-Friedhofen-Str. 9) **eigenständige Betriebe** und wurden nicht übernommen. **Quelle 2 (OSM):**
  node/13307227200 und die übrigen Adressobjekte der Moselweißer Straße (live abgerufen 03.11.2026 über
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.5450,50.3350,7.6050,50.3750). **Was fehlt:** eine
  **Betreiber- oder Registerquelle**, die den Betrieb an dieser Anschrift bestätigt. Der im Dossier dokumentierte
  Befund, dass der zuvor gespeicherte Produktionspunkt (50.3540142857143/7.5774) rückwärts auf „2, Devorastraße,
  Goldgrube“ auflöst und dort zum **anderen** Betrieb „Schrottplatz 1A Schrotthandel & Entrümpelungen“ gehört,
  bleibt **unverändert** - der Punkt wird **nicht** übernommen. `status` bleibt **pruefung**,
  `clear_fields: 'geo'` bleibt bestehen. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.5450,50.3350,7.6050,50.3750 (Kachelraster, live abgerufen
  03.11.2026) ; https://www.gelbeseiten.de/gsbiz/f3e15f7e-7279-4692-86b1-1504b98cf884 ;
  https://www.schrottplatz.org/koblenz/schrott-schrotthandel-ra-aYEPwO.html (live abgerufen 03.11.2026).]

### Geokodierung 05.11.2026

- [Geokodierung 05.11.2026: **Klärfall - weiterhin keine Koordinate, weil die zweite unabhängige Quelle fehlt.** Der
  OSM-Befund der Vorphase ist **live bestätigt** und der Adresspunkt **eindeutig**, aber die Dossier-Adresse stammt
  ausschließlich aus **zwei Verzeichnissen**, und nach der README-Quellenhierarchie sind Verzeichnisse ausdrücklich
  **Leads, nie Evidenz** - zwei Verzeichnisse sind zudem **keine** zwei voneinander unabhängigen Quellen im Sinne der
  Zwei-Quellen-Regel. **Was geprüft wurde (live am 05.11.2026):** (a) **Suchfenster (0,02°-Kacheln):**
  **bbox=7.5550,50.3350,7.6050,50.3750** = 6 Kacheln à 0,02° (7.5550/50.3350/7.5750/50.3550, 7.5550/50.3550/7.5750/
  50.3750, 7.5750/50.3350/7.5950/50.3550, 7.5750/50.3550/7.5950/50.3750, 7.5950/50.3350/7.6050/50.3550,
  7.5950/50.3550/7.6050/50.3750), live abgerufen am 05.11.2026; **Zensus:** **8.898** Adressobjekte und **392**
  namentliche Straßen im Fenster. Die Straße **Moselweißer Straße** (Koblenz-Rauental) trägt **95** Adressobjekte mit den
  Nummern 1, 2b, 3, 4, 5, 6, 7, 8, 10, 12, 16, 18, **20**, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 30a, 31, 32, 33, 34,
  34a, 36, 37, 38, 40, 42-46, 43, 45, 47, 47a, 48, 48-52, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
  64, 65, 66, 66A, 67, 68, 69, 70, 71, 73, 75, 77, 78, 79, 81, 83, 85, 101-103, 105, 107, 115, 116, 117-119, 121,
  122, 123, 124, 125, 192, 194 - die **Hausnummer 20 ist genau einmal** belegt und damit eindeutig. Eine Suche über
  **alle** Tag-Werte nach „**R&A**" bzw. einem Schrottbetrieb im Fenster liefert **4** Treffer, von denen **einer** ein
  **Schrottplatz** ist: **way/1265664115** mit `industrial`=**scrap_yard**, `name`=**„Schrotthandel M. Schneider u.
  Sohn"**, `addr:street`=**Peter-Klöckner-Straße**, `addr:housenumber`=**2A**, `addr:postcode`=56073, `mobile`=+49 175
  3737011 - ein **anderer Betrieb**, **nicht** „Schrotthandel R&A"; die übrigen drei Treffer sind eine Überwachungskamera,
  das „Dienstleistungscenter" der GZ Service Koblenz GmbH und ein Krankenhausgebäude. (b) **Objekt (live geladen):**
  **node/13307227200** mit `addr:housenumber`=**20**, `addr:street`=**Moselweißer Straße**, `addr:city`=**Koblenz**,
  `addr:postcode`=**56073**, `addr:suburb`=**Rauental** (live am 05.11.2026 über
  https://api.openstreetmap.org/api/0.6/node/13307227200.json); Punkt **50.3613280/7.5841080** (BBox identisch, Punkt
  liegt **innerhalb**). Der Knoten trägt **weder `name=` noch `operator=`** - der Betrieb ist in OSM **nicht namentlich
  kartiert**. (c) **Quelle 1 (Verzeichnis):** Die **Gelbe-Seiten-Detailseite** (live abgerufen 05.11.2026, HTTP 200,
  https://www.gelbeseiten.de/gsbiz/f3e15f7e-7279-4692-86b1-1504b98cf884, Titel „Schrotthandel R&A in 56073
  Koblenz-Rauental") führt „**Schrotthandel R&A**" mit „**Moselweißer Str. 20, 56073 Koblenz-Rauental**", Telefon
  **0176 65 07 34 77**, Branche „Schrott"; der **Website-Button des Eintrags ist inaktiv** - es gibt also **keine**
  Betreiberseite. Das **Schrottplatz-Portal** (live abgerufen 03.11.2026, HTTP 200,
  https://www.schrottplatz.org/koblenz/schrott-schrotthandel-ra-aYEPwO.html) nennt denselben Betrieb mit derselben
  Anschrift. Beides sind **Verzeichnisse**; eine **Betreiberseite**, ein **Impressum** oder ein **Registereintrag** zu
  „Schrotthandel R&A" existiert **nicht**. **Registerabgleich (neu):** Der Abzug des **Fachbetrieberegisters der
  Gemeinsamen Stelle Altfahrzeuge und des Registers der Entsorgungsfachbetriebe** (Stand 11.08.2026, über den Spiegel
  schrottregister.pages.dev) führt für **56073 Koblenz** **keinen** Betrieb. **Quelle 2 (OSM):** node/13307227200 und
  die übrigen Adressobjekte der Moselweißer Straße (live abgerufen 05.11.2026 über das Kachelraster). **Was fehlt:** eine
  **Betreiber- oder Registerquelle**, die den Betrieb an dieser Anschrift bestätigt. Der im Dossier dokumentierte
  Befund, dass der zuvor gespeicherte Produktionspunkt (50.3540142857143/7.5774) rückwärts auf „2, Devorastraße,
  Goldgrube" auflöst und dort zum **anderen** Betrieb „Schrottplatz 1A Schrotthandel & Entrümpelungen" gehört, bleibt
  **unverändert** - der Punkt wird **nicht** übernommen. `status` bleibt **pruefung**, `clear_fields: 'geo'` bleibt
  bestehen. **Reichweitenklausel:** Das Fehlen eines Betreiber- oder Registerbelegs ist **kein** Existenz- oder
  Schließungsbeweis. Quelle(n): https://api.openstreetmap.org/api/0.6/node/13307227200.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.5550,50.3350,7.5750,50.3550 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.5550,50.3550,7.5750,50.3750 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.5750,50.3350,7.5950,50.3550 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.5750,50.3550,7.5950,50.3750 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.5950,50.3350,7.6050,50.3550 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.5950,50.3550,7.6050,50.3750 ;
  https://www.gelbeseiten.de/gsbiz/f3e15f7e-7279-4692-86b1-1504b98cf884 ;
  https://schrottregister.pages.dev/ort-koblenz ; Recherche-Vermerke vom 27.10. und 03.11.2026 im Dossier (live abgerufen
  05.11.2026).]

### Recherche 19.11.2026 (Welle 1800 A)

- [Recherche 19.11.2026: **Amtliches Fachbetrieberegister: unter dem Namen R&A steht kein zertifizierter Betrieb, und an der Dossier-Adresse steht ebenfalls keiner; das ist ein Negativbefund mit Reichweite.** Das amtliche Fachbetrieberegister (live am 19.11.2026, Filter Art des Betriebs = Egal, Ergebnisgrid vollständig gelesen) liefert für die Namenssuche `R&A` in der Route "Entsorgungsfachbetriebe" 0 Treffer und in der Route "Altfahrzeugverwertung" 0 Treffer; die Standortabfrage mit 3 km um `Moselweißer Straße 20, 56073 Koblenz` liefert 16 Efb-Zeilen und 0 AfV-Zeilen. Die 16 Efb-Zeilen betreffen ausschließlich Fremdbetriebe, die nächstgelegenen `Klaus Probst Altölentsorgung e.K.` (Bubenheimer Weg 54, 56072 Koblenz, 1,6 km), `MCR Metallgroßhandel & Containerdienst Riebe GmbH` (Schönbornsluster Straße 12, 56070 Koblenz, 2,0 km, sechs Zeilen), `ARGE ImPro - Hasenbach GmbH & Co. KG` (2,1 km), `Hasenbach GmbH` (Schönbornsluster Straße 34, 56070 Koblenz, 2,2 km, drei Zeilen), `REMONDIS Mittelrhein GmbH` (Daimlerstraße 7, 56070 Koblenz, 2,4 km, zwei Zeilen), `Lux & Co. GmbH` (Klausenbergweg 40 A, 56077 Koblenz, 2,5 km) und `Biewer Logistik GmbH` (Hans-Böckler-Straße 3, 56070 Koblenz, 2,9 km); keine Zeile nennt `R&A` oder die Moselweißer Straße 20. **Reichweite:** die Efb- und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein Existenz- oder Schließungsbeweis. **ANKAUF:** keine Kurse und keine Materialangaben. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=R%26A ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=R%26A ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Moselwei%C3%9Fer%20Stra%C3%9Fe%2020%2C%2056073%20Koblenz%2C%20Deutschland%3B7.584108%3B50.361328%3B7.6266183%3B50.388652%3B7.5415977%3B50.334004&dst=3&is=1&e=1 (Abruf 19.11.2026).]
- [Recherche 19.11.2026: **Der Verzeichniseintrag besteht am 19.11.2026 unverändert und hat weiterhin keine Betreiberwebsite; die Rücknahme des fremden Koordinatenpunkts ist im Frontmatter bereits veranlasst.** Die Gelbe-Seiten-Detailseite (live am 19.11.2026, HTTP 200) führt `Schrotthandel R&A`, Branche `Schrott`, `Moselweißer Str. 20, 56073 Koblenz-Rauental` und `0176 65 07 34 77`; der Website-Eintrag ist ohne hinterlegte Adresse, eine E-Mail-Adresse wird nicht genannt, und eine Bewertung existiert nicht. Das Frontmatter trägt den Token `clear_fields: 'geo'`, der den zuvor gespeicherten, zu `2, Devorastraße, Goldgrube, 56073 Koblenz` gehörenden Punkt des fremden Betriebs `Schrottplatz 1A` beim regulären Seed zurücknimmt; eine Ersatzkoordinate wurde nicht gesetzt, weil für die Moselweißer Straße 20 nur Verzeichnisangaben vorliegen. Das Frontmatter bleibt darüber hinaus unverändert, `status` bleibt `pruefung`. Quelle(n): https://www.gelbeseiten.de/gsbiz/f3e15f7e-7279-4692-86b1-1504b98cf884 ; https://docs/trader-seed.md (Abruf 19.11.2026).]

### Korrektur 24.11.2026 (Owner, Postleitzahl im Ortsfeld)

- [Korrektur 24.11.2026: **Das Feld `city` ✓✓ ist **berichtigt** ✓✓: der Ort **Koblenz 56073** ✓✓ stand dort **mit** der Postleitzahl **56073** ✓✓ ✓✓ Das Feld `postcode` ✓✓ trug **dieselbe** ✓✓ Postleitzahl **bereits** ✓✓ und bleibt **unverändert** ✓✓ ✓✓ **Belege** ✓✓: **erstens** ✓✓ der **Seed-Wert** ✓✓ des Feldes `postcode` ✓✓ desselben Dossiers ✓✓, **zweitens** ✓✓ die **Konvention** ✓✓ des Katalogs ✓✓ (der Ort steht im Feld `city` ✓✓, die Postleitzahl im Feld `postcode` ✓✓) ✓✓ **Dossier** ✓✓: **Schrotthandel R&A** ✓✓, Straße **Moselweißer Str. 20** ✓✓ ✓✓]

### Korrektur 24.11.2026 (Owner, Geokodierung)

- [Korrektur 24.11.2026 (Owner, Geokodierung): **Kein Standortpunkt** ✓✓ — Die **Hausnummer** ✓✓ **20** ✓✓ an der **Straße** ✓✓ **„Moselweißer Str. 20“** ✓✓ ist im **Raster** ✓✓ **nicht** ✓✓ **als** ✓✓ **Adressobjekt** ✓✓ **erfasst** ✓✓ — die **Nachbarobjekte** ✓✓ tragen **andere** ✓✓ **Nummern** ✓✓ ✓ ✓ ✓✓ **Reichweite:** Ohne ein **adressiertes** ✓✓ Objekt ✓✓ mit **genau dieser** ✓✓ **Hausnummer** ✓✓ und mit **passender** ✓✓ **Postleitzahl** ✓✓ und **Ort** ✓✓ ist **kein** ✓✓ **Standortpunkt** ✓✓ zu **setzen** ✓✓ ✓ ✓ ✓✓]
