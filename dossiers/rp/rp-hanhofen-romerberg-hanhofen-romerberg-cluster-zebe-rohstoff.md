---
slug: rp-hanhofen-romerberg-hanhofen-romerberg-cluster-zebe-rohstoff
clear_fields: geo
name: Hanhofen/Römerberg-Cluster (ZEBE Rohstoffhandel; Wagner R.+B.; Kreischer J.;
  Lehr T.+M./I.+R.; Wagner P.+S.)
trader_type: metallhaendler
state: RP
city: Hanhofen / Römerberg
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# Hanhofen/Römerberg-Cluster (ZEBE Rohstoffhandel; Wagner R.+B.; Kreischer J.; Lehr T.+M./I.+R.; Wagner P.+S.)

## Überblick

Der Cluster bleibt ein Sammel-Klärfall und darf nicht als eine gemeinsame Firma behandelt werden. NorthData führt **ZEBE Rohstoffhandel GmbH Schrott-Metalle Gießerei- und Hüttenbedarf**, HRB 51621 AG Ludwigshafen, Martin-Greif-Str. 31, 67354 Römerberg; der angezeigte Gesellschaftszweck umfasst Handel mit Schrott, Metallen und Nichteisenmetallen sowie Gießerei-/Hüttenbedarf. Sichtbare Jahresabschlüsse reichen bis 2023 (veröffentlicht 24.01.2025); das ist kein Nachweis heutigen Betriebs oder einer öffentlichen Annahmestelle. ZEBE-Adresse und Rechtsträger bleiben nur dieser Cluster-Komponente zugeordnet, nicht in gemeinsame Frontmatter-Felder übertragen. Die übrigen Wagner-, Kreischer- und Lehr-Namen bleiben separate Verzeichnisleads.

**Ankauf:** Registerzweck nennt Handel, aber keine heutigen Annahmebedingungen oder Preise. **Verkauf:** keine Preisliste. **Gebühren:** keine Tarife. **GEO:** Martin-Greif-Str. 31 ist ein Registermirror-Adressbeleg, kein bestätigter öffentlicher Hof; nicht geokodieren, bis Betrieb/Annahmestelle bestätigt sind.

## Timeline

### Owner-Korrektur 27.10.2026: nicht zuordenbaren Clusterpunkt zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `49.3167, 8.3408`; der Reverse-Lookup liegt an Hauptstraße 9b in Hanhofen. Der einzige identifizierte Rechtsträger im Sammeldatensatz, ZEBE Rohstoffhandel GmbH, hat laut dem dokumentierten Registermirror-Lead die Anschrift Martin-Greif-Straße 31 in Römerberg; die übrigen Personen/Betriebe sind unverbundene Leads. Das bestehende Paar kann deshalb keinem Händler dieses Mehr-Orte-Clusters belastbar zugeordnet werden. `clear_fields: geo` nimmt nur diesen ungeprüften Sammelpunkt beim regulären Seed zurück. Kein Ersatzpunkt, keine Aussage, dass es in Hanhofen keinen Betrieb gibt, und kein Schließungsurteil. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.3167&lon=8.3408&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Martin-Greif-Strasse%2031%2C%2067354%20R%C3%B6merberg%2C%20Germany&format=jsonv2&limit=5&addressdetails=1 ; https://www.northdata.de/ZEBE+Rohstoffhandel+GmbH+Schrott-Metalle+Gie%C3%9Ferei-+und+H%C3%BCttenbedarf,+R%C3%B6merberg/Amtsgericht+Ludwigshafen+a.Rhein+HRB+51621 ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 27.10.2026

- [Recherche 27.10.2026: NorthData-Unternehmensseite zu ZEBE direkt gelesen: **ZEBE Rohstoffhandel GmbH Schrott-Metalle Gießerei- und Hüttenbedarf**, AG Ludwigshafen HRB 51621, Martin-Greif-Str. 31, 67354 Römerberg; Gegenstand nennt Handel mit Schrott, Metallen und Nichteisenmetallen sowie Gießerei-/Hüttenbedarf. Sichtbarster Jahresabschluss 2023 am 24.01.2025 veröffentlicht. Dies ist ein Registermirror-Identitäts-/Adressbefund, aber weder aktuelle Betriebsbestätigung noch öffentliche Annahme-/Anlieferbestätigung. Keine ZEBE-Felder auf den Sammelcluster übertragen; die übrigen Namen bleiben getrennte Leads. **ANKAUF:** Geschäftszweck belegt keinen tatsächlichen aktuellen Ankauf/Kurs. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife. Kein GEO. Quelle: https://www.northdata.de/ZEBE+Rohstoffhandel+GmbH+Schrott-Metalle+Gie%C3%9Ferei-+und+H%C3%BCttenbedarf,+R%C3%B6merberg/Amtsgericht+Ludwigshafen+a.Rhein+HRB+51621 .]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Klärfall — einziger HR-Beleg im Cluster ist ZEBE Rohstoffhandel GmbH (Martin-Greif-Str. 31, 67354 Römerberg, HR AG Ludwigshafen, Verzeichnis-/Snippet-Hinweis, nicht unabhängig verifiziert); die übrigen Cluster-Kleinhändler (Wagner R.+B., Kreischer J., Lehr T.+M./I.+R., Wagner P.+S.) nur Gelbe-Seiten-Leads ohne Betreiber-Beleg → keine Frontmatter-Fills (Cluster-Adresse nicht mit ZEBE-Adresse überschreiben). Quelle(n): YellowMap-Eintrag + HR-Snippet ZEBE Rohstoffhandel GmbH + Gelbe Seiten]

### Importiert (Seed-Stand 2026-09-30)

- Mehrere Kleinhändler Schrott-/Metallhandel (nur Gelbe Seiten)

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt, weil die Zeile ein Sammel-Cluster über mehrere
  nicht zugeordnete Kleinbetriebe ist.** Das Dossier enthält **fünf** eigenständige Namen (ZEBE Rohstoffhandel GmbH;
  Wagner R.+B.; Kreischer J.; Lehr T.+M./I.+R.; Wagner P.+S.) und hält ausdrücklich fest, dass die
  **ZEBE-Adresse und deren Rechtsträger nur dieser Cluster-Komponente zugeordnet** bleiben und **nicht** in
  gemeinsame Frontmatter-Felder übertragen werden dürfen - ein Punkt für den Sammel-Datensatz müsste sich also
  willkürlich für eine der Komponenten entscheiden. **Was geprüft wurde (live am 03.11.2026):** (a) Die
  Dossier-Adresse „**Martin-Greif-Str. 31, 67354 Römerberg**“ ist in OSM **eindeutig belegt**: **way/133500286** mit
  `addr:housenumber`=**31**, `addr:street`=**Martin-Greif-Straße**, `addr:city`=**Römerberg**,
  `addr:postcode`=**67354**, `addr:suburb`=**Berghausen**, `addr:country`=DE und `building`=residential (live geladen
  03.11.2026, https://api.openstreetmap.org/api/0.6/way/133500286/full.json); Objekt-BBox
  49.2879978/8.4066219 bis 49.2881284/8.4068373, der Way hat 7 Knotenreferenzen, davon **6 eindeutige** (ein
  doppelter Knoten), der Mittelwert der eindeutigen Knoten ist **49.2880619/8.4067252**. Die **Hausnummer 31** ist auf
  der Martin-Greif-Straße **eindeutig** (Kachelraster über **bbox=8.3750,49.2600,8.4250,49.3000**, live abgerufen
  03.11.2026; die belegten Nummern sind 1 bis 36, 38, 40, 42, 44, 44A, 44B, 46, 48, 50, 52, 54, 56, 58, 60, 62 sowie
  2a). (b) Eine Suche über **alle** Tag-Werte nach „ZEBE“, „Kreischer“, „Lehr“ bzw. einem Schrott-/Metallbetrieb im
  Raster liefert **0 Objekte**, die **den Betrieb** nennen - die Betriebe sind in OSM **nicht namentlich kartiert**; das
  Objekt trägt **weder `name=` noch `operator=`** und ist als `building`=**residential** kartiert. (c) Die **zweite**
  im Dossier zitierte **Martin-Greif-Straße** in OSM (way/33440561, way/34309401 und die Adressobjekte
  way/169943830, way/169943837, way/169943856, way/169943868 mit `addr:city`=**Ludwigshafen am Rhein**,
  `addr:postcode`=**67065**) liegt in **Ludwigshafen** und **nicht** in Römerberg - sie ist damit **keine** Alternative
  für die Dossier-Adresse und wurde **nicht** übernommen. **Quelle 1 (Register):** Der **Handelsregisterauszug über
  online-handelsregister.de** (live abgerufen 03.11.2026, HTTP 200,
  https://www.online-handelsregister.de/handelsregisterauszug/rp/Ludwigshafen-am-Rhein/HRB/51621/ZEBE-Rohstoffhandel-GmbH-Schrott-Metalle-Giesserei-und-Huettenbedarf)
  nennt „**ZEBE Rohstoffhandel GmbH Schrott-Metalle Gießerei- und Hüttenbedarf**“, aktiv, **HRB 51621**,
  **Amtsgericht Ludwigshafen a.Rhein**, mit „**Martin-Greif-Str. 31 • 67354 Römerberg**“ und dem Satz „Sie können
  schriftlich über die Firmenadresse **Martin-Greif-Str. 31, 67354 Römerberg** erreicht werden“; der **NorthData-
  Eintrag** (im Dossier bereits zitiert) ist deckungsgleich. **Quelle 2 (OSM):** way/133500286 (live geladen
  03.11.2026) sowie das Kachelraster (live abgerufen 03.11.2026 über
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.3750,49.2600,8.4250,49.3000). **Was fehlt:** die
  **Entscheidung des Owners**, ob der Sammel-Cluster in **Einzeldossiers** aufgeteilt wird und welche Komponente
  welche Koordinate erhält - erst dann wäre die jeweils zugehörige OSM-Koordinate eindeutig zuzuordnen. **Unverändert
  bleibt:** Die Adresse ist ein **Register-Adressbeleg** und **kein** bestätigter öffentlicher Hof; der sichtbare
  Jahresabschluss reicht bis 2023 (veröffentlicht 24.01.2025) und belegt **weder** heutigen Betrieb **noch** eine
  Kundenannahme; die **Ankaufskurse**, **Preislisten** und **Gebührentarien** bleiben unbelegt. `status` bleibt
  **pruefung**, `street`/`postcode` bleiben leer, `clear_fields: geo` bleibt bestehen. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/133500286/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.3750,49.2600,8.4250,49.3000 (Kachelraster, live abgerufen
  03.11.2026) ;
  https://www.online-handelsregister.de/handelsregisterauszug/rp/Ludwigshafen-am-Rhein/HRB/51621/ZEBE-Rohstoffhandel-GmbH-Schrott-Metalle-Giesserei-und-Huettenbedarf
  (live abgerufen 03.11.2026).]

### Geokodierung 05.11.2026

- [Geokodierung 05.11.2026: **Klärfall - weiterhin keine Koordinate, obwohl die Adresse in OSM inzwischen live
  bestätigt ist.** Anders als bei den Negativbefunden dieser Welle ist die Dossier-Adresse hier **nicht** unauffindbar:
  Das OSM-Objekt existiert, die Adresse ist **zweifach** belegt. **Was fehlt, sind die beiden anderen Bedingungen des
  Dossiers - die Zuordnung zu einer **Komponente** des Sammel-Clusters und die Bestätigung einer **Annahmestelle** -
  und beides kann eine Geokodierungsprüfung nicht ersetzen. **Was geprüft wurde (live am 05.11.2026):** (a)
  **Suchfenster (0,01°-Kacheln):** **bbox=8.3900,49.2750,8.4200,49.2950** = 6 Kacheln à 0,01° (8.3900/49.2750/8.4000/
  49.2850, 8.3900/49.2850/8.4000/49.2950, 8.4000/49.2750/8.4100/49.2850, 8.4000/49.2850/8.4100/49.2950, 8.4100/
  49.2750/8.4200/49.2850, 8.4100/49.2850/8.4200/49.2950), live abgerufen am 05.11.2026; **Zensus:** **50** Adressobjekte
  mit `addr:street`=Martin-Greif-Straße (Nummern 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
  21, 22, 23, 24, 25, 26, 27, 28, 29, **31**, 32, 33, 34, 35, 36, 38, 40, 42, 44, 44A, 44B, 46, 48, 50, 52, 54, 56,
  58, 60, 62 sowie 2a) und **67** namentliche Straßen im Fenster; eine Suche über **alle** Tag-Werte nach „**ZEBE**",
  „**Schrott**", „**Rohstoff**" oder „**Gießerei**" liefert **0 Treffer** - die Betriebe sind in OSM **nicht namentlich
  kartiert**. (b) **Objekt (live geladen):** **way/133500286** mit `addr:housenumber`=**31**, `addr:street`=**Martin-Greif-
  Straße**, `addr:city`=**Römerberg**, `addr:postcode`=**67354**, `addr:suburb`=**Berghausen**, `addr:country`=DE,
  `building`=**residential** (live am 05.11.2026 über
  https://api.openstreetmap.org/api/0.6/way/133500286/full.json); 7 Knotenreferenzen, davon **6 eindeutige**;
  Objekt-BBox 49.2879978/8.4066219 bis 49.2881284/8.4068373; Mittelwert der eindeutigen Knoten **49.2880714/8.4067411**,
  liegt **innerhalb** der Objekt-BBox; die **Objekt-BBox liegt vollständig im Suchfenster** (Prüfung ✓). (c) **Quelle 1
  (Register, live):** Der **Handelsregisterauszug** über online-handelsregister.de (live abgerufen 05.11.2026, HTTP 200,
  https://www.online-handelsregister.de/handelsregisterauszug/rp/Ludwigshafen-am-Rhein/HRB/51621/ZEBE-Rohstoffhandel-GmbH-Schrott-Metalle-Giesserei-und-Huettenbedarf)
  nennt „**ZEBE Rohstoffhandel GmbH Schrott-Metalle Gießerei- und Hüttenbedarf**", **aktiv**, **HRB 51621**,
  **Amtsgericht Ludwigshafen a.Rhein**, Anschrift „**Martin-Greif-Str. 31 • 67354 Römerberg**" - die Dossier-Adresse ist
  damit **registerbelegt**. **Quelle 2 (OSM):** way/133500286 (live abgerufen 05.11.2026) sowie das 0,01°-Kachelraster.
  **Was trotzdem für die Nichtsetzung spricht (unverändert übernommen):** (1) Die Zeile ist ein **Sammel-Cluster** über
  **fünf** eigenständige Namen (ZEBE Rohstoffhandel GmbH; Wagner R.+B.; Kreischer J.; Lehr T.+M./I.+R.; Wagner P.+S.);
  das Dossier hält ausdrücklich fest, dass die **ZEBE-Adresse und deren Rechtsträger nur dieser Cluster-Komponente
  zugeordnet** bleiben und **nicht** in gemeinsame Frontmatter-Felder übertragen werden dürfen - ein Punkt für den
  Sammel-Datensatz müsste sich **willkürlich** für eine der Komponenten entscheiden. (2) Die Adresse ist ein
  **Register-Adressbeleg** und **kein** bestätigter öffentlicher Hof: das OSM-Objekt ist als `building`=**residential**
  kartiert und trägt **weder `name=` noch `operator=`**, und der **amtliche Registerabzug** des Fachbetrieberegisters der
  Gemeinsamen Stelle Altfahrzeuge und des Registers der Entsorgungsfachbetriebe (Stand 11.08.2026, über den Spiegel
  schrottregister.pages.dev) kennt für **67354 Römerberg/Hanhofen** **keinen** Betrieb und **keinen** Eintrag „ZEBE" -
  eine Efb-/AltfahrzeugV-Anerkennung und damit eine **Annahmestelle** sind **nicht belegt**. Der sichtbare
  Jahresabschluss reicht bis 2023 (veröffentlicht 24.01.2025) und belegt **weder** heutigen Betrieb **noch** eine
  Kundenannahme. **Damit wäre der Kandidatenpunkt 49.2880714/8.4067411 - er wird hier bewusst nicht gesetzt.**
  `status` bleibt **pruefung**, `street`/`postcode` bleiben leer, `clear_fields: geo` bleibt bestehen. **Reichweiten-
  Klausel:** Das Fehlen einer Efb-Anerkennung oder eines Betriebsnamens in OSM ist **kein** Existenz- oder
  Schließungsbeweis; die offene Owner-Entscheidung zur **Aufteilung des Sammel-Clusters in Einzeldossiers** bleibt
  Voraussetzung für eine Koordinate. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/133500286/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.3900,49.2750,8.4000,49.2850 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.3900,49.2850,8.4000,49.2950 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4000,49.2750,8.4100,49.2850 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4000,49.2850,8.4100,49.2950 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4100,49.2750,8.4200,49.2850 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4100,49.2850,8.4200,49.2950 ;
  https://www.online-handelsregister.de/handelsregisterauszug/rp/Ludwigshafen-am-Rhein/HRB/51621/ZEBE-Rohstoffhandel-GmbH-Schrott-Metalle-Giesserei-und-Huettenbedarf ;
  https://schrottregister.pages.dev/assets/institute.json (live abgerufen 05.11.2026).]
