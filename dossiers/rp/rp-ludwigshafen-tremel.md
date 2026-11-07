---
slug: rp-ludwigshafen-tremel
lat: 49.4554044
lon: 8.4194830
name: Tremel
trader_type: sonstige
state: RP
city: Ludwigshafen
street: Wachtenburgstr. 9
postcode: '67065'
phone: 0621 5297052
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

# Tremel

## Überblick

Tremel ist ein als Schrott Tremel geführter Schrottbetrieb in der Wachtenburgstraße 9, 67065
Ludwigshafen am Rhein, im Stadtteil Mundenheim, mit der Rufnummer 0621 5297052. Die
Verzeichniseinträge nennen als Anschrift Flurstraße 9H, das Ratsinformationssystem der Stadt
Ludwigshafen belegt die Umbenennung der Flurstraße in Wachtenburgstraße im Jahr 2022, und in
OpenStreetMap tragen die Straßenways den alten Namen als "old_name". Die Hausnummer 9 ist dort
belegt, ein Objekt, das den Betrieb selbst nennt, existiert jedoch nicht; ob die
Verzeichnisangabe 9H dieselbe Liegenschaft wie die belegte Nummer 9 bezeichnet, ist nicht
entscheidbar. Offen bleibt die Beleglage insgesamt dünn: ein Betriebsauftritt, ein Impressum,
ein Registereintrag und ein Zertifikat fehlen, Website, Öffnungszeiten und E-Mail bleiben leer,
und für Ankauf, Verkauf und Gebühren ist nichts belegt.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Wachtenburgstr. 9, Mikro
- urspr. Website-Angabe: keine
- Adresse: Ludwigshafen

### Recherche 31.10.2026

- [Recherche 31.10.2026: **Adresskonflikt geklärt: „Wachtenburgstraße 9“ und „Flurstr. 9H“ sind dieselbe Straße — Flurstraße wurde 2022 in Wachtenburgstraße umbenannt.** Das **Ratsinformationssystem der Stadt Ludwigshafen am Rhein** führt die **Ausschussvorlage 20225462** „**Straßenumbenennung im Stadtteil Mundenheim - Flurstraße in Wachtenburgstraße**“ (Art.: Vorlage der Verwaltung) und im Sitzungsteil „**TOP Ö 15: Straßenumbenennung im Stadtteil Mundenheim - Flurstraße in Wachtenburgstraße**, Sitzung: **05.09.2022, Bau- und Grundstücksausschuss**, **Beschluss: Mit Stimmenmehrheit bei Gegenstimmen und Enthaltungen angenommen**, Abstimmung: Ja: 14, Nein: 1, Enthaltung: 1, Befangen: 0“. Die **Dossier-Adresse „Wachtenburgstr. 9“** und die **Verzeichnis-Adresse „Flurstr. 9H“** bezeichnen damit **dieselbe Straße in Mundenheim**; die Schreibweisen sind **nicht** zwei verschiedene Betriebsadressen. **OSM-Befund**: die Straße ist in OSM **nicht** unter `name=Wachtenburgstraße` erfasst — in einem Fenster um die Dossier-Koordinate (live über `https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4000,49.4600,8.4600,49.5000`; Filter-Key `addr:street` gleich „Flurstraße“ bzw. „Wachtenburgstraße“) (BBox um 49,4866/8,4181) gibt es **keine** Straße mit diesem Namen; der Namensbestand enthält dort u.a. `Industriestraße`, `Frankenthaler Straße`, `Erzbergerstraße`, `Goerdelerplatz`. **Konsequenz**: `street` bleibt **„Tremel Ludwigshafen Wachtenburgstr. 9“** (unverändert, weil der Seed-Wert die aktuelle Schreibweise trägt und die Umbenennung belegt ist); `city` **Ludwigshafen**, `postcode` **67065** werden ergänzt, weil die Verzeichnisquelle „Flurstr. 9H, **67065** Ludwigshafen am Rhein“ nennt (Mundenheim gehört zu 67065). Quelle(n): https://ludwigshafen.de/ratsinformationssystem/bi/vo0050.php?__kvonr=12047750 ; https://ludwigshafen.de/ratsinformationssystem/bi/to0050.php?__ktonr=52545 ; https://www.schrottplatz-info.de/schrottplatz/Ludwigshafen-am-Rhein/Tremel-I- ; https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4000,49.4600,8.4600,49.5000 (Namensbestand im Fenster live abgerufen) (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **OSM live geprüft (Flurstraße/Wachtenburgstraße, Ludwigshafen-Mundenheim).** OSM kennt in einem Fenster um die Dossier-Koordinate (**bbox=8.4000,49.4600 bis 8.4600,49.5000**) **keine** Straße mit `name=Wachtenburgstraße` und **keine** Straße mit `name=Flurstraße` in Ludwigshafen — der Namensbestand des Fensters umfasst u.a. `Industriestraße`, `Frankenthaler Straße`, `Erzbergerstraße`, `Goerdelerplatz`, `Bruchwiesenstraße`, `Weiherstraße`, `Rheinuferstraße`, `Teichgartenweg`. Der **einzige** Way mit `name=Flurstraße` in den abgefragten Kacheln ist **way/141548477** mit `highway=residential` und Extent **49.4236615/7.4957107 bis 49.4247856/7.4964785** — das liegt in **Hütschenhausen** (RP), **nicht** in Ludwigshafen. Die **Dossier-Hausnummer 9** (bzw. **9H**) ist damit in OSM **nicht** als Adressobjekt belegt; ein Objekt, das **den eigenen Betrieb** nennt, existiert im Fenster **nicht**. **Konsequenz**: die Dossier-Adresse ist durch die Verzeichnisquelle und die amtliche Umbenennung belegt, aber **nicht** durch OSM; die OSM-Lage ist als **Klärfall** dokumentiert und im Bericht als **Adresse mit Geokodierungsbedarf** zu nennen. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4000,49.4600,8.4600,49.5000 (live abgerufen) ; https://api.openstreetmap.org/api/0.6/way/141548477.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Der Betrieb selbst ist über die genannten Verzeichnisse nicht eindeutig belegt — zwei Quellen nennen zwei unterschiedliche Adressen.** **Schrottplatz-Info** führt den Eintrag „**Schrott Tremel I.**“ mit „Adresse: **Flurstr. 9H, 67065 Ludwigshafen am Rhein**“, „Telefon: Tel: **(0621) 5 29 70 52**“ und den ausdrücklichen Negativbefunden „**Für diesen Schrottplatz wurde bisher noch keine vollständige Beschreibung erfasst**“, „**Die Öffnungszeiten für diesen Anbieter wurden noch nicht vollständig erfasst**“ und „**Das Leistungsangebot für diesen Anbieter ist noch nicht vorhanden**“; der Link „zur Homepage Schrott Tremel I.“ führt **auf die Portalseite selbst** (`href="/"`) und damit **auf keinen eigenen Betriebsauftritt**. Ein zweiter Verzeichniseintrag nennt **„Ilona Tremel, Schrott, Flurstr. 9H, Stadtteil Mundenheim, 67065 Ludwigshafen am Rhein“** (Quelle: Stadtbranchenbuch `ludwigshafen-am-rhein.stadtbranchenbuch.com/1041914.html` — beim Abruf am 31.10.2026 **HTTP 403**, Inhalt daher nur über den Suchindex belegt, **nicht** am Portal verifiziert). **Widersprüchlich** bleibt: Das Örtliche-ähnliche Cylex-Verzeichnis nennt „**Tremel i., Wachtenburgstraße 9, 67065 Ludwigshafen am Rhein, Mundenheim**“ (beim Abruf **HTTP 403**), während Schrottplatz-Info **Flurstr. 9H** nennt — ob es sich um **Hausnummer 9** oder **9H** handelt, ist aus den verifizierbaren Quellen **nicht** entscheidbar. **Ein Betriebsauftritt, ein Impressum, ein Registereintrag oder ein Zertifikat liegt nicht vor**; `website` bleibt deshalb **leer**, `website_status` **unbekannt**, `opening_hours` und `email` bleiben **leer**, und `status` bleibt **pruefung**. **Wichtiger Abgrenzungspunkt**: Im selben Ort gibt es **mindestens zwei weitere Schrottbetriebe**, die **nicht** mit Tremel zu verwechseln sind — „**Schmelzer Günther GmbH**“ (REC-METAL, Erbachstraße 23, 67065) und „**Opposite Michael SchrottHdl.**“ (beide von Schrottplatz-Info als Nachbarn genannt). Quelle(n): https://www.schrottplatz-info.de/schrottplatz/Ludwigshafen-am-Rhein/Tremel-I- ; https://www.rec-metal.de/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Preise: für Tremel ist nichts belegt.** ANKAUF: **keine** Preise, **keine** Sortenliste, **kein** Leistungsangebot belegt (Schrottplatz-Info: „Das Leistungsangebot für diesen Anbieter ist noch nicht vorhanden“). VERKAUF: **keine** Verkaufsangaben. GEBÜHREN: **keine** Annahme-, Abhol- oder Containergebühr belegt; „Die Öffnungszeiten … wurden noch nicht vollständig erfasst“ heißt ausdrücklich **nicht**, dass keine Öffnungszeiten bestehen. **Nicht** übernommen werden Preisangaben **anderer** Betriebe desselben Ortes (REC-METAL/Schmelzer, Opp, STEIL-Gruppe, Wetzel) — das wären **fremde** Kurse. Die Dossier-Provenienz „**Ankauf unklar**“ bleibt damit zutreffend und wird **nicht** geändert. Quelle(n): https://www.schrottplatz-info.de/schrottplatz/Ludwigshafen-am-Rhein/Tremel-I- (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Hausnummern-Gegenrechnung abgeschlossen — die Aufzählung ist bestätigt.** Nachgerechnet über die Zähllogik von `scripts/release/osm_street_scan.py` (Filter-Key `addr:street=Wachtenburgstraße` **mit** `addr:housenumber`, Kachelraster 0,01 Grad über das **vollständige** Raster-BBox **bbox=8.4000,49.4600,8.4600,49.5000**, 24/24 Teilfenster vollständig verfügbar): **0 Adressobjekte / 0 verschiedene Hausnummern** — **deckungsgleich** mit der Angabe im Recherche-Vermerk oben: jede dort genannte Nummer existiert live, und jede live vorhandene Nummer ist dort genannt. Die **Dossier-Hausnummer 9** ist auf dieser Straße auch in der Gegenrechnung **nicht** vorhanden — das deckt sich mit dem Recherche-Vermerk oben. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4000,49.4600,8.4600,49.5000 (Raster vollständig, live abgerufen) (Abruf 31.10.2026).]
- [Korrektur 31.10.2026: **Frontmatter bereinigt und ergänzt — die Angabe `street` ist doch geändert worden; die Aussage im Recherche-Vermerk oben, `street` bleibe unverändert, ist damit berichtigt.** `street` stand als „Tremel Ludwigshafen Wachtenburgstr. 9“ im Straßennfeld (Betriebsname und Ort waren dort mitgeführt) und ist jetzt auf die reine Straßenangabe **„Wachtenburgstr. 9“** gesetzt; `city` trägt unverändert „Ludwigshafen“. `postcode` ist von leer auf **67065** und `phone` von leer auf **0621 5297052** gesetzt (Verzeichniseintrag „Schrott Tremel I.“: „Flurstr. 9H, 67065 Ludwigshafen am Rhein“, „Tel: (0621) 5 29 70 52“). Die Straßenschreibweise **Wachtenburgstraße** und die Hausnummer **9** bleiben durch das Ratsinformationssystem der Stadt Ludwigshafen am Rhein belegt (Vorlage 20225462, „Straßenumbenennung im Stadtteil Mundenheim - Flurstraße in Wachtenburgstraße“, Sitzung 05.09.2022). `website`, `website_status`, `opening_hours` und `email` bleiben leer, `status` bleibt **pruefung** — ein Betriebsauftritt, ein Impressum oder ein Registereintrag liegt nicht vor. Quelle(n): https://www.schrottplatz-info.de/schrottplatz/Ludwigshafen-am-Rhein/Tremel-I- ; https://ludwigshafen.de/ratsinformationssystem/bi/vo0050.php?__kvonr=12047750 (Abruf 31.10.2026).]

### Geokodierung 01.11.2026

- [Geokodierung 01.11.2026: **Koordinate korrigiert**: `lat`/`lon` = **49.4554044/8.4194830** (Mittel der eindeutigen
  Knoten von node/12828823015, innerhalb der Objekt-BBox). **Der Klärfall vom 31.10.2026 ist damit erledigt: das
  Suchfenster der Vorwelle (bbox=8.4000,49.4600 bis 8.4600,49.5000) lag zu weit nördlich - die Straße liegt bei
  49.4554/8.4195 und war deshalb außerhalb des Fensters.** **Was neu geprüft wurde (live am 01.11.2026):** (a) Die
  Straße **Wachtenburgstraße** ist in OSM mit **zwei** Ways kartiert (way/33715402, `highway`=residential,
  `name`=Wachtenburgstraße, **`old_name`=Flurstraße**, `maxspeed`=30, `sidewalk`=both; way/33715405, `highway`=residential,
  `name`=Wachtenburgstraße, **`old_name`=Flurstraße**, `maxspeed`=30) - der **`old_name`=Flurstraße** bestätigt die 2022
  durch das Ratsinformationssystem belegte Umbenennung **unabhängig in OSM**. (b) Ein **frisches** Kachelraster über
  **bbox=8.3800,49.4500,8.4700,49.5100** (live abgerufen 01.11.2026) findet auf der Wachtenburgstraße **17
  Adressobjekte** mit den Hausnummern **2a, 3, 3a, 4, 6, 7, 8, 9, 10, 11, 12, 13, 15, 17, 19, 21** sowie **4**
  Wohngebäude ohne Hausnummer - die **Dossier-Hausnummer 9 ist also belegt**: node/12828823015 mit
  `addr:housenumber`=9, `addr:street`=**Wachtenburgstraße**, `addr:city`=Ludwigshafen am Rhein, `addr:postcode`=67065,
  `addr:country`=DE und `entrance`=main. Die **Hausnummer 9H** existiert in OSM **nicht**; die belegte Nummer **9** ist
  die einzige 9 auf der Straße und die **niedrigste** der beiden Schreibweisen aus den Verzeichnissen. (c) Eine Suche
  über **alle** Tag-Werte nach „tremel" im Raster liefert **0 Objekte** - ein Objekt, das **den eigenen Betrieb** nennt,
  existiert in OSM **nicht**; node/12828823015 trägt **weder `name=` noch `operator=`**. Quelle 1 (Betreiber): Das
  Verzeichnis **Schrottplatz-Info** (https://www.schrottplatz-info.de/schrottplatz/Ludwigshafen-am-Rhein/Tremel-I-,
  live abgerufen 01.11.2026) führt den Eintrag „**Schrott Tremel I.**“ mit „Adresse: **Flurstr. 9H, 67065 Ludwigshafen
  am Rhein**“ und „Telefon: **(0621) 5 29 70 52**“ - die Dossier-Adresse und -Telefonnummer sind damit belegt; das
  **Ratsinformationssystem der Stadt Ludwigshafen am Rhein** belegt die amtliche Umbenennung „**Flurstraße in
  Wachtenburgstraße**“ (Vorlage 20225462, Bau- und Grundstücksausschuss, Sitzung 05.09.2022) und damit die
  Identität beider Schreibweisen (live abgerufen 01.11.2026,
  https://ludwigshafen.de/ratsinformationssystem/bi/vo0050.php?__kvonr=12047750). Quelle 2 (OSM): node/12828823015
  (live abgerufen 01.11.2026, https://api.openstreetmap.org/api/0.6/node/12828823015.json) sowie way/33715402 und
  way/33715405 (live abgerufen 01.11.2026, https://api.openstreetmap.org/api/0.6/way/33715402/full.json ;
  https://api.openstreetmap.org/api/0.6/way/33715405/full.json) - Objekt-BBox 49.4554044/8.4194830, der Punkt liegt
  innerhalb. **Was offen bleibt:** Ob die Verzeichnisangabe **9H** dieselbe Liegenschaft wie die OSM-Nummer **9**
  bezeichnet, ist aus den verifizierbaren Quellen **nicht** entscheidbar; ein **Betriebsauftritt, ein Impressum, ein
  Registereintrag oder ein Zertifikat** liegt für Tremel weiterhin **nicht** vor, `website`, `opening_hours` und
  `email` bleiben leer und `status` bleibt **pruefung**. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.3800,49.4500,8.4700,49.5100 (Kachelraster, live abgerufen
  01.11.2026) ; https://api.openstreetmap.org/api/0.6/node/12828823015.json ;
  https://www.schrottplatz-info.de/schrottplatz/Ludwigshafen-am-Rhein/Tremel-I- (live abgerufen 01.11.2026).]

### Recherche 07.11.2026

- [Recherche 07.11.2026: **Das amtliche Fachbetrieberegister kennt in Mundenheim sechs Altfahrzeugverwertungs-Betriebe - aber keinen Tremel; die Namenssuche „Tremel“ führt ausschließlich zu einem bayerischen Namensvetter. Der Seed-Name ist damit amtlich nicht belegt.** (1) **Namenssuche (live abgefragt 07.11.2026):** Die Route „Altfahrzeugverwertung“ liefert für **„Tremel“** **0 Treffer**; die Route „Entsorgungsfachbetriebe“ liefert **5 Treffer**, sämtlich die **Tremel Entsorgungs- und Transport GmbH** mit den Standorten **Rohrbach 2, 91572 Bechhofen** (ZZIT00400046/1 bis /3: Lagern, Lagern/Behandeln, Lagern), **Waizendorf 5, 91572 Bechhofen** (/4: Fuhrpark, Sammeln/Befördern) und **Am Sägewerk 7, 91601 Dombühl** (/5: Lagern/Behandeln) - sämtlich in **Bayern** (Landkreis Ansbach/Neustadt an der Aisch), also **kein Bezug** zu Ludwigshafen-Mundenheim. Die beiden sind **Namensvetter** und werden nicht übernommen. (2) **Standortabfrage um die Dossier-Adresse** (Photon-Geokodierung: **Wachtenburgstraße 9, 67065 Ludwigshafen am Rhein** -> **49.4554044 / 8.4194830**, OSM way/174459035) in der Route „Altfahrzeugverwertung“ ergibt im **3-km-Radius 6 Treffer**, im 5-km-Radius ebenfalls 6, im 10-km-Radius 9: **„Horst Lehr“, Erbachstraße 20, 67065 Ludwigshafen am Rhein, ZAHS00500003 / 1** (0,4 km); **„Containerdienst und Autoverwertung Thomas Lehr“, Rampenweg 24, 67065 Ludwigshafen, ZAGS00300029 / 1** (0,4 km); **„Auto Daud Auto-Recycling e. K.“, Rampenweg 18, 67065 Ludwigshafen, ZAGS00300007 / 1** (0,5 km); **„Abschleppdienst Krajcovic“, Erbachstraße 15, 67065 Ludwigshafen, ZAGS00300015 / 1** (0,5 km); **„Autoverwertung und Recycling Waldemar Gurr“, Am Sandloch 8, 67065 Ludwigshafen, ZAGS00300010 / 1** (0,9 km); **„Wagner & Wagner GmbH & Co. KG“, Hoheneckenstraße 82, 67065 Ludwigshafen, ZAGS00300024 / 1** (1,2 km). **Kein Treffer nennt Tremel**, und **kein Treffer liegt an der Wachtenburgstraße 9** - die sechs Nachbarn belegen zugleich, dass **67065 Mundenheim ein dichtes Autoverwertungs-Gebiet** ist, in dem ein nicht im Register geführter Kleinbetrieb existieren kann. Die Efb-Route liefert im 3-km-Radius **18 Treffer** (5 km: 31; 10 km: 238), **keiner** mit Tremel im Namen. **Konsequenz:** Die Beleglage aus der Vorwelle (nur Verzeichnislead „Schrottplatz-Info“ mit „Schrott Tremel I.“, keine Betreiberseite, kein Impressum, kein Registereintrag) bleibt bestehen; die Identität **Tremel** ist amtlich **nicht** belegt. **Keine Frontmatter-Änderung**, `status` bleibt `pruefung`, `website_status` bleibt `unbekannt`. **ANKAUF:** keine Kurse und keine Sortenliste; **VERKAUF:** keine Liste; **GEBÜHREN:** keine Tarife. **GEO:** die Dossier-Koordinate 49.4554044/8.4194830 entspricht dem OSM-Adressobjekt **node/12828823015** (`addr:housenumber`=9, `addr:street`=Wachtenburgstraße, `addr:postcode`=67065) und bleibt unverändert; ein von der Straße benanntes Betriebsobjekt existiert in OSM **nicht**. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Tremel ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Tremel ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Wachtenburgstra%C3%9Fe%209%2C%2067065%20Ludwigshafen%20am%20Rhein%2C%20Deutschland%3B8.419483%3B49.4554044%3B8.431452%3B49.467375%3B8.407514%3B49.443434&dst=3&is=1&e=1 (Abruf 07.11.2026).]
