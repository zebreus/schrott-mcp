---
slug: nw-essen-schrotthandler-traut
name: Schrotthändler Traut (Essen)
trader_type: schrotthaendler
state: NW
city: Essen
street: ''
postcode: ''
phone: 0162 9135397
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Zweiter Verzeichniseintrag desselben mobilen Schrotthaendlers Traut fuer die Stadt Essen: Das Schrottfinder-Profil nennt dieselbe Mobilnummer +49 162 9135397, dieselbe Leistung "Abtransport von Schrott" und dieselben Metallarten (Kupfer, Aluminium, Edelstahl, Messing, Zink, Blei), laesst die Anschrift aber leer und setzt den Kartenmarker auf 92, Witteringstraße, 45130 Essen (Wohnobjekt). Es gibt keine belegte eigene Betriebsanschrift in Essen. Die Zeile ist deshalb die Dublette zur Leitzeile nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger (Anschrift Kurt-Schumacher-Straße 80, 45881 Gelsenkirchen) und bleibt als Querverweis bestehen; nichts wird geloescht oder zusammengefuehrt. ANKAUF: keine Traut zuordenbaren Kurse. VERKAUF: keine Liste. GEBUEHREN: keine Tarife.'
dropoff_json: ''
pickup_json: ''
clear_fields: geo
provenance_section: Neu, Ankauf unklar (nur Verzeichnis-/Profilbeleg)
provenance_ankauf_raw: mobil/unklar
provenance_origin: table
---

# Schrotthändler Traut

## Überblick

Dublette. Das Essen-Profil des Schrottfinder ist eine Stadtvariante desselben
Portal-Eintrags wie die Gelsenkirchen-Zeile: gleiche Mobilnummer +49 162 9135397, gleiche
Leistung, gleiche Metallarten, aber **keine Anschrift** und ein Marker auf einem
Wohnobjekt in der Witteringstraße. Es gibt keine belegte eigene Betriebsanschrift in
Essen.

Die Zeile bleibt als sichtbarer Verzeichniseintrag bestehen (keine Loeschung, kein
Merge) und verweist auf die Leitzeile
`nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger`. Der Produktionspunkt
51.450284375/7.015053125 lag auf der Busspur des Essener Fernbusbahnhofs und wird mit
`clear_fields: geo` zurueckgenommen; ein Ersatzpunkt wird nicht geraten.

**Ankauf:** keine Traut zuordenbaren Kurse. **Verkauf:** keine Liste.
**Gebuehren:** keine Tarife.

## Timeline

### Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt als offenen Korrekturauftrag dokumentieren

- [Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt als offenen Korrekturauftrag dokumentieren: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`. Dieser Punkt liegt laut OSM auf der Straße „Freiheit“, 45128 Essen (way 46961889, `highway=service`, `bus=designated`, `vehicle=no`), genauer auf dem Fernbusbahnhof am Essener Hauptbahnhof (Haltestellenknoten 599294635, „Essen, Fernbusbahnhof“). Das ist eine Busspur und kein Gewerbestandort; in den Umweltinspektionsberichten der Bezirksregierung Düsseldorf für die Stadt Essen gibt es keinen Eintrag auf dieser Straße. Die direkt gelesene Schrottfinder-Detailseite nennt Traut, eine Telefonnummer und eine Essener „Annahmestelle“, lässt deren Anschrift aber leer; das Portal erklärt selbst, dass Unternehmensangaben aus öffentlichen Quellen stammen. Eine kuratierte Adresse oder ein Betreiber- oder Behördenbeleg fehlt. Der Punkt wird deshalb nicht pauschal zurückgenommen (kein Pauschal-Clear), sondern als reiner Stadt-Platzhalter dokumentiert: Er ist **ein offener Korrekturauftrag** und erst mit einer kuratierten Adresse oder einer belegten Annahmestelle zu entscheiden. Bis dahin kein Ersatzpunkt und keine Statusänderung. Neu gefunden wurde ein Zwillingsprofil desselben Aggregators für dieselbe Telefonnummer +49 162 9135397: „Schrotthändler Traut – Gelsenkirchen“, Kurt-Schumacher-Straße 80, 45881 Gelsenkirchen (Marker 51.5217378/7.090119, rund 9,5 km entfernt). Der Marker des Essener Profils selbst liegt bei 51.4409073/7.0160182 (92, Witteringstraße, 45130 Essen, Wohnobjekt, rund 1,02 km entfernt); das Portal führt also für diese Zeile keine brauchbare Essener Anschrift. Die Essener Zeile ist damit mit hoher Wahrscheinlichkeit eine Ortsdublette desselben Mobilbetriebs; eine belegte eigene Betriebsanschrift in Essen gibt es weiterhin nicht, und die Gelsenkirchen-Adresse wird bewusst **nicht** in diese Zeile übernommen. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://schrottfinder.de/schrotthandler-traut-essen/ ; https://schrottfinder.de/schrotthandler-traut-gelsenkirchen-gelsenkirchen/ ; https://nominatim.openstreetmap.org/search?q=Kurt-Schumacher-Stra%C3%9Fe+80%2C+45881+Gelsenkirchen&format=jsonv2&addressdetails=1 ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 15.10.2026 Welle18A

- [Recherche 15.10.2026 Welle18A: Gezielte Suche und direkt gelesene Essener Schrottfinder-Übersicht ergeben konkreten Traut-Lead mit 491629135397 (= +49 162 9135397), Ortsangabe nur Essen und Abholung vor Ort. Korrigiert den pauschalen Vorwellenbefund keine konkrete Spur, bestätigt aber weder Betreiber noch aktuellen Ankauf. Profil wurde nicht als eigengesteuertes Socialprofil belegt; Telefon/Adresse/Services bleiben im Frontmatter leer, status pruefung. Preisabgrenzung: Zahlenblock mit Stand 08.01.2025 gehört im gelesenen Layout zum WRL-Profil; allgemeine Footerpreise sind Portalorientierung und ausdrücklich kein öffentliches Angebot, keine Traut-Ankaufskurse. Verkauf und Gebühren nicht belegt. GeoBedarf: gewerbliche Identität und stationärer/mobiler Betrieb zuerst klären. Quelle (Lead, Volltext direkt gelesen): https://schrottfinder.de/essen ; verlinkter, nicht separat gelesener Zielpfad: https://schrottfinder.de/schrotthandler-traut-essen/ .]

### Importiert (Seed-Stand 2026-09-30)

- 0162-Nr., ohne Adresse
- urspr. Website-Angabe: keine gefunden

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall — kein Treffer: kein Betreiber, keine Adresse, kein Register zu Schrotthändler Traut Essen gefunden (nur generische Essener Schrott-Verzeichnisse); kein Frontmatter-Fill; Quelle(n): keine]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Für die Zeile existiert **keine** belegte
  Betriebsanschrift in Essen; der Kartenmarker des verwendeten Aggregatorportals liegt in einem **Wohnobjekt** und ist
  kein Betriebsort. **Was live geprüft wurde (03.11.2026):** (a) Das Portal-Profil „Schrotthändler Traut“ lässt die
  Anschrift **leer** und nennt nur eine Mobilnummer (**+49 162 9135397**), die Leistung „Abtransport von Schrott“ und
  die Metallarten Kupfer, Aluminium, Edelstahl, Messing, Zink, Blei; das Portal schreibt selbst, dass alle
  Unternehmensangaben aus öffentlichen Quellen stammen. (b) Der Marker dieses Portals liegt bei
  **51.4409073/7.0160182**; die dort genannte Anschrift **92, Witteringstraße, 45130 Essen** ist in OSM nur durch
  **zwei** Objekte belegt: **node/3453735410** (`amenity`=car_sharing, `name`=**Stadtmobil**,
  `brand`=Stadtmobil Rhein-Ruhr) und **node/4548791823** (`amenity`=restaurant, `name`=**Tío Pepe**,
  `website`=https://tio-pepe-essen.de/) — **kein** Schrottbezug; der Marker liegt **230 m** von der Nummer 92 entfernt.
  (c) Eine Suche über `name`, `operator`, `brand`, `website`, `contact:website`, `description`, `note` und `short_name`
  nach **„Traut“** im Kachelraster **6.9400/51.4100 – 7.1000/51.4900** liefert **0** Treffer mit Betriebsbezug (nur
  „Trautes Heim“ und „Waldtrautstraße“ als Straßennamen). (d) Die Zwillingszeile
  `nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger` führt dieselbe Mobilnummer mit der Anschrift
  **Kurt-Schumacher-Straße 80, 45881 Gelsenkirchen**; eine belegte eigene Essener Betriebsanschrift gibt es weiterhin
  nicht, und die Gelsenkirchen-Adresse wird **nicht** in diese Zeile übernommen. Quelle 1 (Betreiber/Register): für
  Traut in Essen ist **weder** eine Betreiberseite **noch** ein Registereintrag gefunden; 11880 führt die Nummer nicht,
  Gelbe Seiten und Das Örtliche kennen keinen Traut-Eintrag in Essen oder Gelsenkirchen, NorthData-Suchen „Schrotthandel
  Traut“/„Traut Gelsenkirchen“ zeigen kein passendes Registerunternehmen (bei Einzelunternehmern ohne HRB ist das ein
  Negativbefund ohne Beweiswert). Quelle 2 (OSM): **kein** Objekt mit Betriebsbezug zu Traut in Essen, das Adressobjekt
  an der Portal-Markeradresse gehört einem Carsharing-Punkt und einem Restaurant. **Was fehlt:** eine kuratierte
  Anschrift **oder** ein belegter Annahmehof für den mobilen Betrieb in Essen; der Klärfall bleibt offen, der
  Stadt-Platzhalter wird nicht durch einen geratenen Punkt ersetzt.
  Quelle(n): https://schrottfinder.de/schrotthandler-traut-essen/ ;
  https://api.openstreetmap.org/api/0.6/node/3453735410.json ;
  https://api.openstreetmap.org/api/0.6/node/4548791823.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=6.9400,51.4100,7.1000,51.4900 (Abruf 03.11.2026).]

### Geokodierung 05.11.2026

- [Geokodierung 05.11.2026: **Klärfall - keine Koordinate gesetzt; die Nachprüfung bestätigt den Vermerk vom
  03.11.2026 mit dem erweiterten Fenster.** Für die Zeile existiert **keine** belegte Betriebsanschrift in Essen; der
  Kartenmarker des verwendeten Aggregatorportals liegt in einem **Wohnobjekt** und ist kein Betriebsort.
  **Quelle 1 (OSM, live abgefragt am 05.11.2026):** das Kachelraster **7.0000/51.4200 bis 7.0300/51.4600**, **12
  Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**
  (https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0000,51.4200,7.0300,51.4600) enthält die **Witteringstraße**
  nachweislich und vollständig (Verlauf 51.4402509/7.0062821 bis 51.4424725/7.0181751). Die Markeradresse **92,
  Witteringstraße, 45130 Essen** ist in OSM nur durch **zwei** Objekte belegt: **node/3453735410** (`amenity`=car_sharing,
  `name`=**Stadtmobil**, `brand`=Stadtmobil Rhein-Ruhr; **39,9 m** vom Marker) und **node/4548791823**
  (`amenity`=restaurant, `name`=**Tío Pepe**, `website`=https://tio-pepe-essen.de/; **24,4 m** vom Marker) - **kein**
  Schrottbezug. Eine Suche nach „Traut" über die Felder name, operator, brand, description, website, contact:website,
  note und addr:housename findet im Kachelausschnitt **keinen** Treffer mit Betriebsbezug - Aussage zum OSM-Bestand,
  **kein** Negativbeleg über den Betrieb. **Quelle 2 (Betreiber/Register, im Dossier bereits zitiert):** das
  Portal-Profil „Schrotthändler Traut" lässt die Anschrift **leer** und nennt nur eine Mobilnummer (**+49 162
  9135397**), die Leistung „Abtransport von Schrott" und die Metallarten Kupfer, Aluminium, Edelstahl, Messing, Zink,
  Blei; das Portal schreibt selbst, dass alle Unternehmensangaben aus öffentlichen Quellen stammen. Für Traut in Essen ist
  **weder** eine Betreiberseite **noch** ein Registereintrag gefunden; 11880 führt die Nummer nicht, Gelbe Seiten und Das
  Örtliche kennen keinen Traut-Eintrag in Essen oder Gelsenkirchen, NorthData-Suchen „Schrotthandel Traut" und „Traut
  Gelsenkirchen" zeigen kein passendes Registerunternehmen (bei Einzelunternehmern ohne HRB ist das ein Negativbefund
  ohne Beweiswert). Die Zwillingszeile `nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger` führt dieselbe
  Mobilnummer mit der Anschrift **Kurt-Schumacher-Straße 80, 45881 Gelsenkirchen**; eine belegte eigene Essener
  Betriebsanschrift gibt es weiterhin nicht, und die Gelsenkirchen-Adresse wird **nicht** in diese Zeile übernommen.
  **Warum keine Koordinate:** Es fehlt eine belegte Anschrift; jeder Punkt wäre eine Ableitung aus einem fremden Objekt
  oder einer Straßenmitte. **Was zur Auflösung fehlt:** eine kuratierte Anschrift **oder** ein belegter Annahmehof für den
  mobilen Betrieb in Essen; der Klärfall bleibt offen, der Portal-Platzhalter wird nicht durch einen geratenen Punkt
  ersetzt. `status: pruefung`, `clear_fields: 'geo'` und alle übrigen Felder bleiben unverändert. **Reichweitenklausel:**
  das Fehlen eines OSM-Betriebsbezugs oder einer Hausnummer wäre kein Existenz- oder Schließungsbeweis. Quelle(n):
  https://schrottfinder.de/schrotthandler-traut-essen/ ;
  https://api.openstreetmap.org/api/0.6/node/3453735410.json ;
  https://api.openstreetmap.org/api/0.6/node/4548791823.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0000,51.4200,7.0300,51.4600 (Abruf 05.11.2026).]


### Geokodierung 05.11.2026 (Nachprüfung)

- [**Klärfall - keine Koordinate gesetzt; die Nachprüfung bestätigt den Vermerk vom 05.11.2026 mit dem enger gefassten
  Fenster.** Für die Zeile existiert **keine** belegte Betriebsanschrift in Essen; der Kartenmarker des verwendeten
  Aggregatorportals liegt in einem **Wohnobjekt** und ist kein Betriebsort. **Quelle 1 (OSM, live geladen am 05.11.2026,
  HTTP 200):** das Kachelraster **7.0000/51.4400 bis 7.0200/51.4600**, **4 Kacheln von je 0,01 Grad**, **0
  Kachel-Fehler** (https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0000,51.4400,7.0200,51.4600) enthält die
  **Witteringstraße** nachweislich und vollständig mit **57** Straßenobjekten (Verlauf 51.4402509/7.0062821 bis
  51.4424725/7.0181751); die Straße trägt im Fenster **137** Adressobjekte (90 Ways, 47 Nodes) und **93** verschiedene
  Hausnummern. Die Markeradresse **92, Witteringstraße, 45130 Essen** ist in OSM durch **drei** Objekte belegt:
  **way/354508084** (`addr:housenumber`=92, `addr:street`=Witteringstraße, `addr:city`=Essen, `addr:country`=DE,
  `addr:postcode`=45130, `building`=apartments; Mittel der **9** eindeutigen Knoten **51.4410768/7.0158151**, BBox
  51.4410206/7.0155783 bis 51.4411675/7.0160754), **node/3453735410** (`amenity`=car_sharing, `name`=**Stadtmobil**,
  `brand`=Stadtmobil Rhein-Ruhr, `operator`=Stadtmobil Rhein-Ruhr GmbH, `website`=https://rhein-ruhr.stadtmobil.de;
  Punktlage **51.4410560/7.0154956**, rund **40 m** vom Portal-Marker entfernt) und **node/4548791823**
  (`amenity`=restaurant, `name`=**Tío Pepe**, `website`=https://tio-pepe-essen.de/, `opening_hours`=Mo-Sa,PH
  18:00-24:00; Punktlage **51.4410972/7.0158434**, rund **24 m** vom Portal-Marker entfernt) - **kein** Schrottbezug.
  Eine Suche nach „Traut" über die Felder name, operator, brand, description, website, contact:website, contact:phone,
  note und addr:housename findet im Kachelausschnitt **keinen** Treffer - Aussage zum OSM-Bestand, **kein** Negativbeleg
  über den Betrieb. **Quelle 2 (Aggregator, live am 05.11.2026 nachgelesen, HTTP 200; keine
  Betreiber-/Registerquelle):** das Portal-Profil „Schrotthändler Traut Essen“ lässt die Anschrift **leer** („nimmt
  Schrott unter der Adresse [leer] an“) und nennt nur die Mobilnummer **+49 162 9135397**, die Leistung „Abtransport von
  Schrott“ und die Metallarten Kupfer, Aluminium, Edelstahl, Messing, Zink und Blei; das Portal schreibt selbst, dass
  alle Unternehmensinformationen aus öffentlichen Quellen stammen, und die dort gezeigten Schrottmetallpreise sind
  ausdrücklich **Portal-Durchschnitte** und **kein** öffentliches Angebot. Für Traut in Essen ist **weder** eine
  Betreiberseite **noch** ein Registereintrag gefunden; 11880 führt die Nummer nicht, Gelbe Seiten und Das Örtliche
  kennen keinen Traut-Eintrag in Essen oder Gelsenkirchen, NorthData-Suchen „Schrotthandel Traut“ und „Traut
  Gelsenkirchen“ zeigen kein passendes Registerunternehmen (bei Einzelunternehmern ohne HRB ist das ein Negativbefund
  ohne Beweiswert). Die Zwillingszeile `nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger` führt dieselbe
  Mobilnummer mit der Anschrift **Kurt-Schumacher-Straße 80, 45881 Gelsenkirchen**; eine belegte eigene Essener
  Betriebsanschrift gibt es weiterhin nicht, und die Gelsenkirchen-Adresse wird **nicht** in diese Zeile übernommen.
  **Warum keine Koordinate:** Es fehlt eine belegte Anschrift; jeder Punkt wäre eine Ableitung aus einem fremden Objekt
  oder einer Straßenmitte. **Was zur Auflösung fehlt:** eine kuratierte Anschrift **oder** ein belegter Annahmehof für
  den mobilen Betrieb in Essen; der Klärfall bleibt offen, der Portal-Platzhalter wird nicht durch einen geratenen Punkt
  ersetzt. `status: pruefung`, `clear_fields: 'geo'` und alle übrigen Felder bleiben unverändert.
  **Reichweitenklausel:** das Fehlen eines OSM-Betriebsbezugs oder einer Hausnummer wäre kein Existenz- oder
  Schließungsbeweis. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0000,51.4400,7.0200,51.4600 ;
  https://api.openstreetmap.org/api/0.6/node/3453735410.json ;
  https://api.openstreetmap.org/api/0.6/node/4548791823.json ;
  https://api.openstreetmap.org/api/0.6/way/354508084/full.json ; https://schrottfinder.de/schrotthandler-traut-essen/ ;
  https://www.northdata.de/?query=Traut+Gelsenkirchen (im Dossier zitiert, Abruf 05.11.2026).]

### Recherche 09.11.2026

- [Recherche 09.11.2026: **Der Dubletten-Charakter dieser Zeile ist am 09.11.2026 bestätigt: das Essener und das Gelsenkirchener Portalprofil tragen dieselbe Mobilnummer, dieselbe Leistung und dieselben Metallarten; eine eigene Essener Betriebsanschrift gibt es weiterhin nicht, und das amtliche Fachbetrieberegister kennt in Essen keinen Traut-Betrieb.** Verzeichnisabgleich (live 09.11.2026, HTTP 200): das Profil „Schrotthändler Traut – Essen“ (https://schrottfinder.de/schrotthandler-traut-essen/) lässt die Anschrift **leer** („nimmt Schrott unter der Adresse an“ ohne Adresse), nennt die Mobilnummer **+49 162 9135397**, die Leistung „**Abtransport von Schrott**“ und die Metallarten **Kupfer, Aluminium, Edelstahl, Messing, Zink, Blei** und verweist auf **0 Bewertungen**; das Zwillingsprofil „Schrotthändler Traut – Gelsenkirchen“ (live 09.11.2026) nennt die **Adresse Kurt-Schumacher-Straße 80, Gelsenkirchen** und **dieselbe** Mobilnummer +49 162 9135397, **dieselbe** Leistung und **dieselben** Metallarten. Die Dublette ist damit am 09.11.2026 unverändert; die Gelsenkirchener Anschrift wird bewusst **nicht** in diese Zeile übernommen. Die Gelbe-Seiten-Umkreissuche „Schrott in Essen“ (live 09.11.2026, 29 Treffer) führt **keinen** Eintrag Traut — der Portalbetrieb ist in Essen nur über das Aggregatorprofil sichtbar. **Amtliches Fachbetrieberegister** (https://fachbetrieberegister.gadsys.de/fachbetrieberegister/, clientseitig gerenderte Vaadin-App, beide Routen, Filter „Art des Betriebs = Egal“ is=1&e=1, Abruf 09.11.2026): die Namenssuchen „Traut“ und „Schrotthändler Traut“ liefern **0** Treffer in beiden Routen; die Standortsuche mit validiertem adr-Parameter (Witteringstraße 92, 45130 Essen; PLZ-Abgleich 45130, Punkt 51.4410768/7.0158151) im **3-km-Radius** liefert in der Route „Entsorgungsfachbetriebe“ **13** Anlagen, davon als nächstgelegener die Evonik Operations GmbH (Rellinghauser Straße 1-11, 45128 Essen, 0,9 km, ZZET01200002 / 2, Handeln, Makeln) — alle übrigen gehören fremden Rechtsträgern; kein Treffer nennt Traut, und die Route „Altfahrzeugverwertung“ liefert dort **0** Treffer. **Amtliches Handelsregisterportal** (online-handelsregister.de, Abruf 09.11.2026): die Suche nach „Schrotthändler Traut“ liefert **keine** Rechtseinheit; bei einem nicht registerpflichtigen Einzelbetrieb ist das ein Negativbefund ohne Beweiswert. **Preise (getrennt geführt):** **Ankauf** — kein Traut zuordenbarer €/kg-Kurs; die auf dem Portal gezeigten Schrottmetallpreise sind ausdrücklich **Portal-Durchschnitte** und **kein** öffentliches Angebot des Betriebs; **Verkauf** — keine Verkaufsliste; **Gebühren** — keine Tarife. **GEO:** nicht gesetzt; `clear_fields: 'geo'` **bleibt** unverändert bestehen und der Produktionspunkt 51.450284375/7.015053125 (Fernbus-Spur am Essener Hauptbahnhof) bleibt zurückgenommen. Der Marker des Portals liegt bei 51.4409073/7.0160182, die von ihm genannte Anschrift **92, Witteringstraße** ist in OSM durch **way/354508084** (`addr:housenumber`=92, `addr:street`=Witteringstraße, `addr:postcode`=45130, `addr:city`=Essen, `building`=apartments) sowie durch **node/3453735410** (Stadtmobil-Carsharing) und **node/4548791823** (Restaurant Tío Pepe) belegt — **Wohnobjekt und Gastronomie, kein Schrottstandort**; jeder Punkt wäre eine Ableitung aus einem fremden Objekt und wird nicht gesetzt. **Reichweite der Aussage:** geprüft wurden die zwei Namensvarianten in beiden Registerrouten, die Standortsuche im 3-km-Radius in beiden Routen, das amtliche Handelsregisterportal sowie die beiden Portalprofile und die städtische Gelbe-Seiten-Umkreissuche; die Trefferzahlen sind oben wörtlich genannt. Ein Register-Nichteintrag ist **kein** Beweis dafür, dass ein Betrieb keine Anerkennung besitzt: die Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind freiwillig, und ein mobiler Einzelbetrieb ohne Efb-Anerkennung wäre dort ohnehin nicht sichtbar. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Traut ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrotth%C3%A4ndler%20Traut ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Witteringstra%C3%9Fe%2092%2C%2045130%20Essen%2C%20Deutschland;7.0158151;51.4410768;7.0393859;51.4646476;6.9922443;51.417506&dst=3&is=1&e=1 ; https://www.online-handelsregister.de/suche/Schrotth%C3%A4ndler+Traut ; https://schrottfinder.de/schrotthandler-traut-essen/ ; https://schrottfinder.de/schrotthandler-traut-gelsenkirchen-gelsenkirchen/ ; https://www.gelbeseiten.de/suche/schrott/essen ; https://api.openstreetmap.org/api/0.6/way/354508084/full.json (Abruf 09.11.2026).]
