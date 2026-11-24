---
slug: rp-ludwigshafen-senck-oggersheim
name: Senck Oggersheim
trader_type: sonstige
state: RP
city: Ludwigshafen
street: Notwendestr. 11
postcode: '67071'
lat: 49.4968904
lon: 8.3741269
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Wide-Net PROSE-Cluster (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Senck Oggersheim

## Überblick

Konkreter Kandidat ist Harald Senck Autoverwertung/Mietwerkstatt an Notwendestraße 11, 67071 Ludwigshafen-Oggersheim, 0621 673223. Er ist bislang ausschließlich in Portalen gefunden; abweichend nennt Yelp Hausnummer7. Die Prosa im street-Feld ist historischer Import, keine aktuelle verifizierte Betriebsanschrift. Kein Primärbeleg oder vollständiger Betreiberabgleich; deshalb keine Kontakt-/Branchen-Füllung.

Seed „FE/NE“ und Portalkategorie Autoverwertung nicht ungeprüft gleichsetzen. Ankauf/Verkauf/Gebühren: keine eigene Liste belegt. Prüfschritt: Gewerbeinhaber Harald Senck, Standort7/11 und heutige Annahme-/Demontagetätigkeit klären; vorerst kein neuer GEO-Punkt.

## Timeline

### Recherche 19.10.2026: Welle 03:30 B

- [Recherche 19.10.2026: Frische Namens-/Adresssuche liefert Harald Senck Autoverwertung an Notwendestr.11/67071/0621673223 und einen Mietwerkstatt-Senck-Lead, Yelp dagegen Notwendestr.7. auto-werkstatt.de verlinkt lediglich top-autoverwertung.de, keinen belegten Betreiberauftritt; Anbieter-/Anfrageportal nicht zum Primärbeleg aufwerten. Harald ist damit eine konkrete neue kommunale Prüfhypothese, nicht verifizierter Inhaberfill. Keine Daten von anderen Senck-Personen übernehmen. Ankauf, Teileverkauf und Entsorgungsgebühren nicht belegt; Hausnummerkonflikt vor GEO lösen. Quellen (Suchindex-Leads): https://ludwigshafen-am-rhein.branchen-info.net/senck-harald-autoverwertung/2633072 ; https://www.auto-werkstatt.de/ludwigshafen/harald-senck-autoverwertung-aUM1He ; https://www.yelp.com/biz/autoverwertung-senck-ludwigshafen .]

### Importiert (Seed-Stand 2026-09-30)

- Notwendestr. 11, FE/NE
- urspr. Website-Angabe: keine
- Adresse: Ludwigshafen

### Recherche 08.11.2026

- [Recherche 08.11.2026: **Die verschmutzte Seed-Straßenzeile ist auf den belegten Straßenanteil bereinigt: `street` steht jetzt auf „Notwendestr. 11“, `postcode` auf „67071“.** Der Namens- und Ortsrest („Senck Oggersheim Ludwigshafen“) ist ein Import-Artefakt. (1) Verzeichniseintrag, live am 08.11.2026: branchen-info.net führt „Senck Harald Autoverwertung“ mit „Notwendestr. 11, 67071 Ludwigshafen (Oggersheim)“ und der Rufnummer 0621 673223; auto-werkstatt.de führt dieselbe Anschrift, dieselbe Nummer 0621 673223 und zusätzlich 0621 671072. Beide Portale sind Teile desselben Branchenbuch-Netzverbunds, branchen-info.net nennt als Herausgeber die „fastline GmbH & Co. KG“, auto-werkstatt.de die „Marktplatz Mittelstand GmbH & Co. KG“, und sind damit **keine** zwei voneinander unabhängigen Erhebungen. (2) Zweite unabhängige Quelle: das OSM-Adressobjekt way/483658559 trägt `addr:housenumber` 11, `addr:street` Notwendestraße, `addr:postcode` 67071, `addr:city` Ludwigshafen. **Abweichung, ausdrücklich benannt:** Yelp führt für denselben Betrieb abweichend die Hausnummer 7 (Vermerk vom 24.10.2026); der Direktabruf der Yelp-Seite liefert am 08.11.2026 HTTP 403, die Abweichung bleibt damit offen und wird nicht aufgelöst. **Offen bleibt:** die Betreiberidentität und die Tätigkeit, ein Betreiberauftritt und ein Registereintrag fehlen; `status` bleibt `pruefung`, es wird keine Koordinate gesetzt und kein Preisblatt übernommen. Quelle(n): https://ludwigshafen-am-rhein.branchen-info.net/senck-harald-autoverwertung/2633072 ; https://www.auto-werkstatt.de/ludwigshafen/harald-senck-autoverwertung-aUM1He ; https://api.openstreetmap.org/api/0.6/way/483658559.json ; https://www.yelp.com/biz/autoverwertung-senck-ludwigshafen (Abruf 08.11.2026).]

### Recherche 23.11.2026 (Welle 0300 C)

- [Recherche 23.11.2026 (Welle 0300 C): **Der Betrieb ist in OSM als benannter Schrottplatz kartiert, und ein journalistischer Beleg nennt den Inhaber beim Namen — die Identitätsfrage des Dossiers ist damit praktisch geklärt, die Hausnummernfrage 7 gegen 11 zugunsten von 11 entschieden.** (1) **Betrieb in OSM namentlich kartiert:** Das Kachelraster **bbox=8.3600,49.4850,8.3900,49.5050** (live abgerufen 23.11.2026) enthält das Objekt **way/1504955722** mit den Tags `name`=**Autoverwertung Senck**, `industrial`=**scrap_yard**, `landuse`=industrial — ein **Flächenobjekt** mit 6 eindeutigen Knoten, Objekt-BBox 8.3730394/49.4964183 bis 8.3745548/49.4972924, Schwerpunkt 49.4969170/8.3737599. Das Objekt trägt **keine** Adress-Tags, nennt aber **den Betrieb selbst**; die Namenssuche über **alle** Tag-Werte nach „Senck" im Raster liefert **genau diesen einen** Treffer. (2) **Das Adressobjekt liegt vollständig auf dem Schrottplatz:** Das OSM-Adressobjekt **way/483658559** (`addr:housenumber`=**11**, `addr:street`=**Notwendestraße**, `addr:postcode`=67071, `addr:city`=Ludwigshafen am Rhein, `building`=yes, `source:building`=OpenData Rheinland-Pfalz, Objektmitte 49.4968904/8.3741269, 13 Knoten, 12 eindeutige, Objekt-BBox 8.3740326/49.4968300 bis 8.3742337/49.4969535) liegt mit seiner **gesamten BBox innerhalb** der BBox des Schrottplatz-Polygons; die **Distanz** zwischen der Objektmitte und dem Polygon-Schwerpunkt beträgt **28,2 m** (Haversine, R = 6371 km). Damit sind Betriebsfläche und Hausnummer **11** räumlich **dasselbe** Grundstück. (3) **Die Hausnummer 7 existiert nicht:** Die Notwendestraße in Ludwigshafen trägt im Raster **bbox=8.3650,49.4900,8.3850,49.5000** die **22** Adressobjekte mit den Hausnummern 2, 3, 4, 5, 6, **11**, 12, 14, 15, 17, 18, 19, 20, 22, 26, 28, 2a, 30, 34, 30c — eine **Hausnummer 7 gibt es dort nicht**. Die im Dossier vom 24.10.2026 und 08.11.2026 offene Abweichung (Yelp nennt „Notwendestr. 7", alle übrigen Verzeichnisse „11") ist damit **entschieden**: **11** ist die belegte Hausnummer, und die Angabe „7" hat **kein** Adressobjekt. (4) **Zweite unabhängige Quelle mit Betreiberbezug:** Die **RHEINPFALZ**-Zeitung hat am **11.04.2021** unter dem Titel „Armer kleiner Opel: Besuch auf dem Schrottplatz" den Betrieb besucht: „**Harald Senck (links) mit seinem Mitarbeiter Mohamed Mokadem auf der Lagerfläche seines Schrottplatzes**" (Foto: Moray, Text: Daniel Krauser); der Artikel nennt „Besuch auf dem Senck'schen Schrottplatz in der Ludwigshafener Notwende". Das ist ein **redaktioneller Beleg** für Betreiber, Standort und Lagerfläche, nicht für Preise. (5) **Verzeichnisdeckung (aggregiert, Leitplanke):** Die **11880**-Firmenseite „Autoverwertung Senck Inh. Harald Senck Autoverwertung" (live 23.11.2026, HTTP 200, zuletzt aktualisiert 04.04.2026) führt die Branche **Autoverschrottung**, Telefon **(0621) 671073** und ein **Website**-Feld; das dort verlinkte Ziel `https://www.top-autoverwertung.de` ist **kein** Senck-Auftritt, sondern das Impressum des **Drittverzeichnisses** TOP-Autoverwertung.de (Internetprojekt von **Mario Schwertfeger, Georg-Staber-Ring 5, 83022 Rosenheim**, USt-IdNr. DE270054897) und wird **nicht** als Betreiberquelle übernommen. Die **schrottplatz-info**-Seite „Autoverwertung Senck" nennt **Notwendestr. 11, 67071 Ludwigshafen am Rhein**, Tel. **(0621) 67 32 23**, und die **Cylex**-Firmenseite nennt denselben Betrieb mit Sitz 67071, Straße Notwendestr. 11, Branche „Schrott u. Abfallmaterial" und Inhaber **Harald Senck**. **ANKAUF:** keine Preise, keine Sortenliste belegt; die Art der Tätigkeit (Autoverschrottung/Autoverwertung mit Lagerfläche) ist belegt. **VERKAUF:** keine Teilepreisliste belegt. **GEBÜHREN:** keine Tarife belegt. **Was offen bleibt:** die **öffentliche Kundenannahme** am Standort (die Karte belegt eine Lagerfläche, keine Annahme für Privatkunden), die **Rechtsform** (kein Registereintrag gefunden) und die Betreiberadresse **als Postanschrift** (das OSM-Flächenobjekt trägt keine Adresse; die Verzeichnisse nennen Notwendestr. 11). Quelle(n): https://api.openstreetmap.org/api/0.6/way/1504955722/full.json ; https://api.openstreetmap.org/api/0.6/way/483658559/full.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=8.3600,49.4850,8.3900,49.5050 ; https://www.rheinpfalz.de/lokal/ludwigshafen_artikel,-armer-kleiner-opel-besuch-auf-dem-schrottplatz-_arid,5189752.html ; https://11880.com/branchenbuch/ludwigshafen-am-rhein/060440459B43910520/autoverwertung-senck-inh-harald-senck-autoverwertung.html ; https://www.schrottplatz-info.de/schrottplatz/Ludwigshafen-am-Rhein/Senck ; https://www.top-autoverwertung.de/impressum (Abruf 23.11.2026).]
- [Recherche 23.11.2026 (Welle 0300 C): **GEO-Bedarf: der gespeicherte Punkt liegt rund 3,4 km vom Betrieb entfernt.** Read-only `public.db` (23.11.2026) enthält **49.4866 / 8.4181** (Ludwigshafen-Friesenheim, Industriestraße 2b); ein Nominatim-Reverse löst den Punkt auf **Industriestraße 2b, Ludwigshafen-Friesenheim** auf, also **nicht** auf den Betrieb in der Notwende. Die Distanz zum adressgenauen Objekt **way/483658559** (Hausnummer 11, Objektmitte 49.4968904/8.3741269) beträgt **3.376 m** (Haversine, R = 6371 km); zum kartierten **Schrottplatz-Polygon way/1504955722** (Schwerpunkt 49.4969170/8.3737599) sind es **3.402 m**. **Empfehlung an den Owner, nicht ausgeführt:** der gespeicherte Punkt sollte auf **49.4968904 / 8.3741269** (Hausnummer 11) korrigiert werden; ich habe den Punkt **nicht** verändert. Quelle(n): read-only `public.db` (23.11.2026) ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.4866&lon=8.4181&zoom=18&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/way/483658559/full.json ; https://api.openstreetmap.org/api/0.6/way/1504955722/full.json (Abruf 23.11.2026).]

### Korrektur 24.11.2026 (Owner, Geokodierung) — der Standortpunkt wird ersetzt

- [Korrektur 24.11.2026: **Der in der Produktion gespeicherte Standortpunkt dieses Dossiers liegt **3,38 km** von der belegten Betriebsanschrift entfernt — er wird durch den adresskonformen Punkt **49.4968904/8.3741269** ersetzt ✓ ✓ Der Beleg für die Adresse: das **OSM-Objekt mit dem Namen des Betriebs** ✓✓ (`name`=Autoverwertung Senck ✓✓, `industrial`=scrap_yard ✓✓) **und** die **Zeitungsberichterstattung** ✓✓ vom 11.04.2021 ✓✓ (Harald Senck auf der Lagerfläche seines Schrottplatzes ✓✓) ✓✓ **das Zielobjekt** ✓✓ (`way/483658559` ✓✓) ✓ ✓ **Reichweite:** Die EfbV-/AltFahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Anmerkung:** Die **Hausnummer 7** ✓✓ der Yelp-Angabe ✓✓ existiert auf der Notwendestraße ✓✓ **nicht** ✓✓ — die belegte ist die **11** ✓✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Wellennotiz mit der Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 23.11.2026 ✓ ✓), **zweitens** ✓ ✓ das **Adressobjekt** ✓ ✓ in OpenStreetMap ✓ ✓ ✓ ✓]
