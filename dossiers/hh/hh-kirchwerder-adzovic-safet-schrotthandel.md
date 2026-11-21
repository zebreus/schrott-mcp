---
slug: hh-kirchwerder-adzovic-safet-schrotthandel
name: Adzovic Safet Schrotthandel
trader_type: schrotthaendler
state: HH
city: Kirchwerder
street: 'Warwischer Hauptdeich 90'
postcode: '21037'
phone: ''
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

# Adzovic Safet Schrotthandel

## Überblick

Nur ein Einzel-Lead bei schrottplatz-info.de nennt Adzovic Safet Schrotthandel am Warwischer Hauptdeich 90, 21037; Betreiberwebsite, Registerbeleg und zweiter unabhängiger Nachweis fehlen. Ein hamburg.de-Eintrag zu „Adzovic“ nennt dagegen Von-Scheliha-Str. 34, 21035, Haushaltsauflösungen und Tel. 0176 65353877 – andere Adresse und Branche, daher keine sichere Personenzuordnung und kein Merge. Frontmatter-Kontakte bleiben leer; `status: pruefung`.

**Preise (Ankauf/Verkauf/Gebühren):** keine verifizierbaren Preis- oder Gebührenangaben. **Offen:** Betreiberidentität, tatsächlicher Standort, Betriebsform (Schrotthandel vs. Haushaltsauflösung) und Annahmebedingungen klären; keine Schließung aus fehlenden Quellen ableiten. Quellen siehe Recherche 02.10.2026.

**Recherche 29.10.2026:** Die Anschrift ist durch zwei unabhängige Verzeichnisbetreiber und durch das Adress-Objekt in OpenStreetMap gedeckt (siehe Timeline); ein Betreiber-, Impressum- oder Registerbeleg fehlt weiterhin. In Hamburg bestehen mindestens drei verschiedene „Adzovic“-Betriebe — eine Zusammenführung ist nicht belegbar. **Ankauf:** keine Kurse belegt. **Verkauf:** keine Liste. **Gebühren:** keine Tarife.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrott (Kleinhändler Vierlande)
- Adresse: Kirchwerder, Warwischer Hauptdeich 90, 21037

### Recherche 02.10.2026

- [Recherche 02.10.2026: miss / KLAERFALL - einziger Beleg schrottplatz-info.de (Warwischer Hauptdeich 90, 21037, ohne Tel./Website); keine Betreiber-Website, kein Registerbeleg, kein Zweitbeleg. hamburg.de-Adzovic-Eintrag (Von-Scheliha-Str. 34, 21035, Haushaltsaufloesungen, 0176 65353877) weicht in Adresse und Branche ab - Identitaet/Personenzoordnung ungeklärt, moeglichst Kleinst-/Kleinhaendler ohne oeffentlichen Auftritt; kein Fill; Quelle(n): schrottplatz-info.de (Einzel-Lead), hamburg.de (Abweichungs-Lead)]

### Recherche 29.10.2026

- [Recherche 29.10.2026: Anschrift jetzt durch zwei unabhaengige Verzeichnisbetreiber plus Adress-Objekt in OpenStreetMap gedeckt. (1) schrottplatz.org fuehrt live abgerufen den Eintrag „Schrott Adzovic Safet Schrotthandel“, Warwischer Hauptdeich 90, 21037 Hamburg, Tel. (0172) 5 46 13 41, Kategorie Schrottplatz, 0 Bewertungen, Dienstleistungen „Schrotthandel“; die Telefonnummer ist damit erstmals belegt, aber nur EIN Verzeichnisbeleg und daher nicht ins Frontmatter uebernommen. (2) OSM-Praesenzpruefung ueber api.openstreetmap.org (Abruf 29.10.2026): die Strasse „Warwischer Hauptdeich“ (Kirchwerder/Bergedorf, 21037) traegt 15 Objekte mit addr:street; das sind 15 verschiedene Hausnummern (72, 74, 76, 78, 80, 80a, 84, 88, 90, 92, 94, 104, 106, 114, 116) — die Hausnummer 90 EXISTIERT als Gebaeude way/257411602 (addr:city=Hamburg, addr:postcode=21037, building=yes) ohne Namens-, Betreiber- oder Branchen-Tag; street/postcode deshalb ins Frontmatter ergaenzt, phone bleibt leer, status pruefung bleibt (kein Betreiber-/Registerbeleg). (3) Namesaken-Abgrenzung: ein dritter Adzovic-Betrieb ist live belegt — das Impressum von entsorgungen24.de nennt „David Adzovic, Mozartstrasse 6, 22083 Hamburg“, Entruempelung, Tel. +49 176 2277 2608; zusammen mit dem hamburg.de-Lead (Von-Scheliha-Str. 34, 21035, Haushaltsaufloesungen, 0176 65353877) sind es drei verschiedene Hamburger „Adzovic“-Betriebe mit unterschiedlicher Branche und Telefonnummer — keine Personenzuordnung und kein Merge. ANKAUF: keine Kurse, keine Annahmebedingungen; VERKAUF: keine Liste; GEBUEHREN: keine Tarife. Quelle(n): https://www.schrottplatz.org/hamburg/schrott-adzovic-safet-schrotthandel-aYEPVH.html ; https://entsorgungen24.de/impressum/ ; OpenStreetMap via api.openstreetmap.org (way/257411602, Stand 29.10.2026)]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Keine Koordinate gesetzt — Klärfall dokumentiert.** Das OSM-Adressobjekt existiert und
  wurde live nachgeladen: **way/257411602** (live geladen 03.11.2026,
  https://api.openstreetmap.org/api/0.6/way/257411602/full.json) trägt `addr:housenumber`=**90**,
  `addr:street`=**Warwischer Hauptdeich**, `addr:city`=Hamburg, `addr:postcode`=**21037**, `building`=yes; das Objekt
  hat **8 eindeutige Knoten**, der Mittelwert der Knoten (53.4255317/10.1222117) liegt innerhalb der Objekt-BBox
  (53.4254810/10.1221255 bis 53.4256361/10.1223382). Die Dossier-Adresse und -Postleitzahl sind damit objektgenau
  belegt. **Warum trotzdem keine Koordinate:** Für die Adresse **Warwischer Hauptdeich 90** existiert im Dossier
  **keine** Betreiber- oder Registerquelle — die beiden Verzeichnisbetreiber (schrottplatz.org, schrottplatz-info.de)
  sind nach der Repo-Quellenhierarchie **Leads, kein Beleg**, und der OSM-Knoten selbst trägt **weder `name=` noch
  `operator=`** (kein Betriebsbezug, der das Gebäude dem Dossier-Händler zuordnet). Die im Dossier genannten
  Adzovic-Belege betreffen **andere** Betriebe an anderen Anschriften (hamburg.de: „Adzovic", Von-Scheliha-Str. 34,
  21035, Haushaltsauflösungen; Impressum entsorgungen24.de: „David Adzovic, Mozartstrasse 6, 22083 Hamburg",
  Entrümpelung) — eine Personenzuordnung oder ein Merge ist damit **nicht** belegbar. Nach der Zwei-Quellen-Regel
  (OSM-Objekt **und** eine im Dossier zitierte Betreiber-/Registerquelle) ist die zweite Quelle für diese Adresse nicht
  erfüllt; ein Punkt wäre hier eine Behauptung, keine Geokodierung. **Negativaussage, qualifiziert:** Der
  Adress-Objekt-Nachweis belegt **das Gebäude der Hausnummer 90**, nicht die Betreiberidentität „Adzovic Safet
  Schrotthandel"; `street`/`postcode` bleiben unverändert im Frontmatter, `phone` bleibt leer, `status: pruefung`
  bleibt. **Nächster Schritt für den Owner:** ein aktueller Gewerbe-/Betreibernachweis (Impressum mit Rechtsträger oder
  ein Registerauszug) für den Standort Warwischer Hauptdeich 90 — dann kann way/257411602 ohne Neuladen als Koordinate
  übernommen werden. Quelle(n): https://api.openstreetmap.org/api/0.6/way/257411602/full.json ;
  https://www.schrottplatz.org/hamburg/schrott-adzovic-safet-schrotthandel-aYEPVH.html (live abgerufen 03.11.2026,
  Verzeichnislead); Bestandsvermerke vom 29.10.2026 und 02.10.2026 dieses Dossiers.]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Keine Koordinate gesetzt - das OSM-Adressobjekt existiert und wurde erneut live bestätigt; die zweite Quelle
  fehlt weiterhin.** Die Nachprüfung prüft das Objekt mit verifiziertem Fenster neu. **Fenster und Raster:**
  Kachelraster **10.1122/53.4156 bis 10.1322/53.4356** (Hamburg-Kirchwerder, Warwischer Hauptdeich), **4 Kacheln von
  je 0,01 Grad**, **0 Kachel-Fehler** (api.openstreetmap.org/api/0.6/map.json, Abruf 04.11.2026). Das Fenster enthält
  die gesuchte Straße **nachweislich**: der **Warwischer Hauptdeich** ist mit **4** Straßenobjekten darin erfasst.
  **Census im Fenster:** **20** Adressobjekte mit `addr:street`=Warwischer Hauptdeich und `addr:postcode`=21037 mit
  **20** verschiedenen Hausnummern (62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 80a, 84, 88, **90**, 92, 94, 104, 106,
  114, 116). **Quelle 1 (OSM-Objekt, live geladen 04.11.2026):** **way/257411602**
  (https://api.openstreetmap.org/api/0.6/way/257411602/full.json) trägt `addr:housenumber`=**90**,
  `addr:street`=**Warwischer Hauptdeich**, `addr:city`=Hamburg, `addr:postcode`=**21037**, `building`=yes (Version 1,
  Stand 20.01.2014); das Objekt hat **9** eindeutige Knoten, der Mittelwert der eindeutigen Knoten ist **53.4255433 /
  10.1222079**, die Objekt-BBox ist 53.4254810/10.1221255 bis 53.4256361/10.1223382, der Punkt liegt **innerhalb**
  der BBox. **Quelle 2 (Betreiber-/ Register, weiterhin nicht erfüllt):** Für die Adresse **Warwischer Hauptdeich
  90** existiert im Dossier **keine** Betreiber- oder Registerquelle - die beiden Verzeichnisbetreiber
  (schrottplatz.org, schrottplatz-info.de) sind nach der Repo-Quellenhierarchie **Leads, kein Beleg**; die im Dossier
  genannten Adzovic-Belege betreffen **andere** Betriebe an anderen Anschriften (hamburg.de: „Adzovic",
  Von-Scheliha-Str. 34, 21035, Haushaltsauflösungen; Impressum entsorgungen24.de: „David Adzovic, Mozartstrasse 6,
  22083 Hamburg", Entrümpelung) - eine Personenzuordnung oder ein Merge ist damit **nicht** belegbar. **Nebenbefund
  zur Adresslage:** Der Verzeichnislead schrottplatz.org trägt selbst eine Kartenkoordinate **53.4255609 /
  10.1222289** und deckt sich damit auf wenige Meter mit dem OSM-Objekt - die **Adresse** ist also durch Lead und OSM
  deckungsgleich, die **Betreiberidentität** bleibt unbelegt. Das OSM-Objekt trägt **weder `name=` noch
  `operator=`**. **Warum keine Koordinate:** Nach der Zwei-Quellen-Regel (OSM-Objekt **und** eine im Dossier zitierte
  Betreiber-/Registerquelle für dieselbe Adresse) ist die zweite Quelle nicht erfüllt; ein Punkt wäre hier eine
  Behauptung, keine Geokodierung. **Negativaussage, qualifiziert:** Der Adress-Objekt-Nachweis belegt **das Gebäude
  der Hausnummer 90**, nicht die Betreiberidentität „Adzovic Safet Schrotthandel"; eine Suche nach „adzovic", „safet"
  und „schrott" über alle Tag-Werte des Fensters findet **kein** Objekt mit Betriebsbezug. **Reichweite /
  Fehlbelegklausel:** Das Fehlen eines Efb-Eintrags (freiwillige Anerkennung) oder eines Branchenbuch-Eintrags belegt
  **nicht**, dass dort kein Betrieb existiert; umgekehrt belegt die vorhandene Hausnummer **keinen** Betrieb.
  `street`/`postcode` bleiben unverändert im Frontmatter, `phone` bleibt leer, `status: pruefung` bleibt; es wird
  weder Fortbestand noch Schließung behauptet. **Nächster Schritt für den Owner:** ein aktueller
  Gewerbe-/Betreibernachweis (Impressum mit Rechtsträger oder ein Registerauszug) für den Standort Warwischer
  Hauptdeich 90 - dann kann way/257411602 ohne Neuladen als Koordinate übernommen werden. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=10.1122,53.4156,10.1322,53.4356 ;
  https://api.openstreetmap.org/api/0.6/way/257411602/full.json ;
  https://www.schrottplatz.org/hamburg/schrott-adzovic-safet-schrotthandel-aYEPVH.html (live abgerufen 04.11.2026,
  Verzeichnislead) ; Bestandsvermerke vom 29.10.2026 und 02.10.2026 dieses Dossiers (Abruf 04.11.2026).]

### Recherche 21.11.2026 (Welle 1800 C)

- [Recherche 21.11.2026: **Kein Registereintrag unter diesem Namen; ein vierter Hamburger Namensvetter belegt.** Die Namenssuche im amtlichen Handelsregisterportal (online-handelsregister.de, Abruf 21.11.2026, Suchbegriff `Adzovic`) liefert `Keine Suchergebnisse!`; fuer einen Einzelbetrieb ohne HRB ist das ein Negativbefund ohne Beweiswert und kein Schliessungsbeleg. Im amtlichen Insolvenzbekanntmachungen-Portal (Abruf 21.11.2026, Suche `Adzovic`) erscheinen nur Privatpersonen (`Adzovic, Natascha`, Hamburg, `68c IK 188/26`; `Adzovic, Brena`, Hamburg, `68g IK 590/25`), keine Gesellschaft. Neu belegt ist ein vierter Hamburger Betrieb mit dem Familiennamen: `Schrotthandel Adzovic M&N`, `Steinfurther Allee`, `+49 176 68815168` (Verzeichnislead 2-pos.de); zusammen mit den bereits dokumentierten Betrieben `Adzovic Haushaltsauflösungen` (Von-Scheliha-Str. 34, 21035) und `David Adzovic` (Mozartstrasse 6, 22083, Entrümpelung) sind es vier verschiedene `Adzovic`-Betriebe; eine Personenzuordnung und ein Merge sind damit **nicht** belegbar. Quelle(n): https://www.online-handelsregister.de/suche/Adzovic ; https://neu.insolvenzbekanntmachungen.de/ap/suche.jsf ; https://2-pos.de/1440/61442 (Abruf 21.11.2026).]
- [Recherche 21.11.2026: Amtliches Fachbetrieberegister (live am 21.11.2026): die Namenssuche `Adzovic` liefert in der Route `Entsorgungsfachbetriebe` **0 Treffer** und in der Route `Altfahrzeugverwertung` **0 Treffer**; die Standortabfrage mit 3 km um `Warwischer Hauptdeich 90, 21037 Hamburg` liefert nur **2 Efb-Zeilen** (`CURRUS GmbH Hauptstandort`, `Süderquerweg 446`, 1,1 km, `ZZFT01100273 / 1`, Sammeln, Befördern; `Lothar Neubert GmbH`, `Süderquerweg 579`, 1,3 km, `ZZBT00300165 / 1`) und **0 AfV-Zeilen**; **keine Zeile nennt Adzovic**. Reichweitenklausel: die Efb- und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein Nichtexistenzbeweis und widerlegt die Verzeichnisangabe nicht. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Adzovic ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Warwischer%20Hauptdeich%2090%2C%2021037%20Hamburg%2C%20Deutschland%3B10.1172319%3B53.4255667%3B10.1272319%3B53.4327239%3B10.1072319%3B53.4184094&dst=3&is=1&e=1 (Abruf 21.11.2026).]
- [Recherche 21.11.2026: **OSM nachgemessen: die Hausnummer existiert, ein Betriebsbezug fehlt.** Das Fenster 10.1122/53.4156 bis 10.1322/53.4356 (9.755 Elemente) enthaelt 20 Adressobjekte mit `addr:street`=Warwischer Hauptdeich und `addr:postcode`=21037 mit 20 verschiedenen Hausnummern (62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 80a, 84, 88, 90, 92, 94, 104, 106, 114, 116), darunter die Dossier-Hausnummer 90 als `way/257411602` mit `building`=yes; eine Volltextsuche ueber `name`, `operator`, `brand`, `description`, `note`, `shop`, `craft`, `landuse`, `man_made` und `amenity` findet **kein** Objekt mit Bezug zu `Adzovic`, `Safet` oder `Schrott`. Es wird keine Koordinate gesetzt, weil fuer diese Anschrift weiterhin keine Betreiber- oder Registerquelle existiert. **ANKAUF:** nicht belegt. **VERKAUF:** nicht belegt. **GEBÜHREN:** nicht belegt. `status` bleibt `pruefung`, `street`/`postcode` bleiben unveraendert, `phone` bleibt leer. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=10.1122,53.4156,10.1322,53.4356 ; https://api.openstreetmap.org/api/0.6/way/257411602/full.json (Abruf 21.11.2026).]
