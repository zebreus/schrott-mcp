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
