---
slug: hh-heimfeld-harburg-isr-recycling-itzehoer
name: ISR Recycling GmbH & Co. KG (Itzehoer)
trader_type: autoverwertung
state: HH
city: Heimfeld/Harburg
street: Wilhelm-Weber-Str. 3
postcode: '21079'
phone: 040 771441
email: ''
opening_hours: ''
website: https://www.isr-recycling.de
website_status: aktiv
status: aktiv
description: Schrotthandel und Recycling von Wertstoffen in Hamburg-Harburg
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: ja
provenance_origin: table
---

# ISR Recycling GmbH & Co. KG (Itzehoer)

## Überblick

ISR Recycling GmbH & Co. KG mit Sitz Hafenstraße 35, 25524 Itzehoe, Amtsgericht Pinneberg HRA
1317 IZ, betreibt in Hamburg-Harburg einen Schrott- und Metallgroßhandel an der
Wilhelm-Weber-Straße 3, 21079 Hamburg, wo nach Angabe der Standortseite Edelstahl, Aluminium,
Kupfer und Altmetalle angekauft werden. Die Anschrift tragen die Betreiber-Standortseite und
das Branchenverzeichnis des Hamburger Hafens, die dieselbe Rechtsperson unter derselben Adresse
und derselben Rufnummer 040 771441 führen; das Impressum nennt keine Registernummer.
Ankaufpreise werden nicht beziffert, sondern als Höchst- beziehungsweise Tageshöchstpreis
bezeichnet, eine Verkaufspreisliste und eine bezifferte Gebühr fehlen. Offen bleiben der Typ
des Standorts, da die Autoverwertung für Harburg ausdrücklich ausgeschlossen ist, und der
Standortnachweis, da die Wilhelm-Weber-Straße in OSM ohne Adressierung geführt wird.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- FE/NE, Container, Demontage, Autoverwertung

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Identität register-kongruent (Northdata: ISR Recycling GmbH & Co. KG, Itzehoe, AG Pinneberg HRA 1317 IZ; online-handelsregister.de + webvalid.de als 2. Register-Anbieter, u.a. Hafenstraße 35, 25524 Itzehoe). Filiale Hamburg-Harburg per Betreiber-Standortseite: Wilhelm-Weber-Str. 3, 21079 Hamburg, Tel. 040 771441, Fax 040 774094, Öffnungszeiten Mo-Fr 07:00-16:00 Uhr (Einzelbeleg, unsicher — Impressum ohne Registernummer, Owner-Ausnahme greift nicht), daher KEINE Adress-/Kontakt-Fills; nur website_status aktiv (Website live, Identität registerbestätigt); Quelle(n): https://www.isr-recycling.de/index.php/standorte-isr + /impressum (eine Quelle) + Northdata HRA 1317 IZ + online-handelsregister.de (Register-Kongruenz)]
- [Recherche 01.10.2026: Feedback 2197 bestätigt Vorbefund — Standort Hamburg-Harburg auf Betreiber-Standortseite gelistet (Wilhelm-Weber-Str. 3, 21079 Hamburg, Tel. 040 771441, Mo-Fr 07:00-16:00 Uhr), aber Impressum ohne Registernummer → weiterhin keine Adress-/Kontakt-Fills (Einzelbeleg, Owner-Ausnahme greift nicht); trader_type autoverwertung bleibt vorerst (Schrott-/Metallgroßhandel auf Standortseite belegt, Typwechsel braucht Zweitbeleg — Klärfall); Identität registerbestätigt (ISR Recycling GmbH & Co. KG, Sitz Itzehoe HRA 1317 IZ); Quelle(n): https://www.isr-recycling.de/index.php/standorte-isr + Northdata HRA 1317 IZ]

### Recherche 04.10.2026

- [Recherche 04.10.2026: Fill street Wilhelm-Weber-Str. 3, postcode 21079 und phone 040 771441 sowie die knappe Beschreibung Schrotthandel/Recycling von Wertstoffen: Betreiber-Standortseite nennt diese aktuelle Harburger Filiale; Hafen Hamburg führt unabhängig dieselbe Rechtsperson am selben Standort und dieselbe Telefonnummer (zwei unabhängige Quellen). Impressum nennt keine HR-Nummer, daher greift die Owner-Ausnahme nicht; die ergänzten Standortangaben stützen sich ausdrücklich auf den unabhängigen Hafenbeleg. opening_hours bleibt leer (nur Betreiberquelle nennt Mo-Fr 07:00-16:00); email bleibt leer, da Betreiber-Website und Hafenverzeichnis unterschiedliche zentrale Adressen ausweisen und keine lokale Mail bestätigt ist. Qualitätslücke: trader_type autoverwertung bleibt unverändert, obwohl die ISR-Autoverwertungsseite Hamburg nicht unter den Altauto-Annahmestellen aufführt; Harburg ist als Schrotthandel/Metallgroßhandel ausgewiesen. Keine neue Koordinatenangabe; durch die neue Adresse stößt der adressbasierte geocode_cache automatisch eine Neu-Geocodierung an; Quelle(n): https://www.isr-recycling.de/index.php/standorte-isr; https://www.hafen-hamburg.de/de/adressen/isr-itzehoer-schrott-und-recycling-gmbh-und-co-kg-7803/; https://www.isr-recycling.de/index.php/impressum; https://www.isr-recycling.de/index.php/autoverwertung-isr]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprüft (Wilhelm-Weber-Straße, Hamburg-Harburg/Sand).** Kachelraster über den **gesamten** Straßenverlauf: OSM-Way-BBox aller Ways mit `name=Wilhelm-Weber-Straße` (2 Ways: way/5836066 und way/607682372, beide `highway=residential`) = **53.4700016/9.9743159 bis 53.4739983/9.9790130**, **plus 0,010 Grad Rand** → Raster-BBox **bbox=9.9643159,53.4600016,9.9890130,53.4839983** (live über `https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9643159,53.4600016,9.9890130,53.4839983`, Kacheln à 0,01 Grad). Zählung getrennt: **0 Adressobjekte / 0 verschiedene Hausnummern** auf der Straße (Filter-Key `addr:street` gleich „Wilhelm-Weber-Straße“ **mit** `addr:housenumber` — im ganzen Raster findet sich **kein einziges** Objekt mit `addr:street=Wilhelm-Weber-Straße`). **Die Dossier-Hausnummer 3 existiert in OSM auf dieser Straße nicht** — die Straße ist zwar als Route kartiert, aber **ohne jede Adressierung**. Im selben Raster-BBox tragen **1330 Objekte** `name=` oder `operator=`; **keines** davon nennt ISR, die ISR Recycling GmbH & Co. KG oder einen Schrottbetrieb an dieser Straße — die nächstgelegenen Recycling-Objekte sind **kommunale Sammelstellen** (node/1656097066 mit `operator=Stadtreinigung Hamburg`, `amenity=recycling`, `recycling:paper=yes`; node/393842954 mit `operator=Stadtreinigung Hamburg`; node/6651740362 mit `operator=Falke Recycling`, `recycling:clothes=yes`; node/7623197796 mit `operator=Loop`, `recycling:clothes=yes`) sowie **Straßenobjekte** der Umgebung (way/4981040, way/488200381, way/488200382 mit `name=2. Hafenstraße`; way/4069180 mit `name=Seehafenstraße`). **Die einzige Karte an der 2. Hafenstraße** trägt way/23495935 mit `addr:housenumber=15`, `addr:street=2. Hafenstraße`, `building=yes` — **ohne** `name=`/`operator=`. Ein Objekt, das **den eigenen Betrieb** nennt, ist im Raster **nicht** vorhanden. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9643159,53.4600016,9.9890130,53.4839983 (bbox-Raster, live abgerufen) ; https://api.openstreetmap.org/api/0.6/way/5836066.json ; https://api.openstreetmap.org/api/0.6/way/23495935.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Die geocodierte Koordinate im Produktionsbestand liegt nicht auf der Dossier-Straße.** Die Produktionsdatenbank führt für diesen Dossier **lat 53.4537727 / lon 9.9812636** — das ist der Bereich **Baererstraße/Maretstraße** in Hamburg-Heimfeld, rund **2 km nördlich** der OSM-Wilhelm-Weber-Straße (53.470/9.9766). Der Dossier-Wert selbst wird **nicht** geändert (die Betreiberseite nennt Wilhelm-Weber-Str. 3, 21079 Hamburg); die Koordinate ist aber **als falsch zu behandeln** und der Datensatz braucht eine **Neu-Geocodierung** an der Betriebsadresse. Der **Widerspruch zur OSM-Lage** ist oben benannt: die OSM-Wilhelm-Weber-Straße führt **keine Hausnummern**, und die nächstgelegenen Adressobjekte im Raster liegen an **Konsul-Ritter-Straße** (way/23557598, `addr:housenumber=15/17`, `addr:postcode=21079`), Lauenbrucher Deich und Seehafenstraße. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9643159,53.4600016,9.9890130,53.4839983 (bbox-Raster, live abgerufen) ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Betreiberangaben und Leistungen am 31.10.2026 bestätigt — die Standortseite nennt die Hamburger Filiale ausdrücklich als Schrott-/Metallgroßhandel.** Die **Standortseite** (https://www.isr-recycling.de/index.php/standorte-isr, HTTP 200) führt „**Schrotthandel Hamburg Harburg**, ISR Recycling GmbH & Co. KG, **Wilhelm-Weber-Str. 3, 21079 Hamburg**, **0 40 - 77 14 41**, **0 40 - 77 40 94**, Öffnungszeiten **Mo. - Fr. 7.00Uhr - 16.00Uhr**“ und beschreibt: „**Wir sind Ihr Schrotthandel in Hamburg! Unser Schrott- und Metallgroßhandel befindet sich im Süden von Hamburg direkt am Wasser. Für eine Preisauskunft oder die Abholung von Mengen setzen Sie sich gerne mit uns in Verbindung. Edelstahl, Alu, Kupfer, Gold und Altmetalle, wir kaufen Ihren Metallschrott.**“ Das **Impressum** (https://www.isr-recycling.de/index.php/impressum, HTTP 200) nennt als Dienstanbieter „**ISR - Recycling GmbH & Co. KG, Hafenstraße 35, 25524 Itzehoe**“, Geschäftsführung „**SMS Recycling Verwaltungs GmbH**“, Geschäftsführer **Kevin Hopp**, USt-Id **DE 187203567** — **ohne Registernummer**; die Impressumsseite listet zusätzlich **LSH Lübecker Schrotthandel GmbH** (Dampfpfeife 12-20, 23569 Lübeck), **Warnsholz GmbH & Co. KG** (Robert-Bosch-Straße 8, 25335 Elmshorn) und **Peter Stolz Edelstahlhandelsges. mbH** (Hovestraße 32, 20539 Hamburg) als **eigene Gesellschaften** — diese sind **nicht** mit der Hamburger Filiale zu vermischen. **Die Sorte „Gold“** in der Standortbeschreibung ist eine **Metall-Angabe im Fließtext**; das Angebot der Filiale bleibt **Schrott/Metallgroßhandel**, **nicht** Edelmetallscheideanstalt. Quelle(n): https://www.isr-recycling.de/index.php/standorte-isr ; https://www.isr-recycling.de/index.php/impressum (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Preise strikt getrennt: Ankauf, Verkauf, Gebühren.** ANKAUF: die Standortseite nennt **keine** Zahlen, sondern nur die Zusage „vergüten zu **Höchstpreisen**“ (Itzehoe) bzw. „für eine **Preisauskunft** … setzen Sie sich gerne mit uns in Verbindung“ (Hamburg) und die Sorten **Edelstahl, Alu, Kupfer, Gold und Altmetalle**; die Leistungsseite formuliert „**Wir kaufen Ihren Schrott zum aktuellen Tageshöchstpreis**“. Ein **Kurs je Sorte für Hamburg** wird **nicht** veröffentlicht — die Angabe ist eine **Tagespreis-Zusage ohne Zahl**. VERKAUF: **keine** Verkaufspreisliste; die Leistungsseite beschreibt **Recycling und Metallhandel** (Rückführung in den Produktionskreislauf), **Containerverleih**, **Private Entsorgung**, **Autoverwertung** („**Kein Verkauf von Kfz-Ersatzteilen**“) und **Demontage u. Abbrucharbeiten**. GEBÜHREN: **keine** bezifferte Annahme-, Container- oder Abholgebühr; die Container-Seite nennt **Abrollcontainergrößen 4 m³ bis 55 m³** und **Mulden 1 m³ bis 12 m³ (auch mit Deckel)** sowie die **Abholung zum vereinbarten Termin** — das sind **Leistungsangaben ohne Betrag**. **Ausdrücklich** ist nur die **kostenfreie** Wertstoffhof-Annahme der Kreis Steinburg benannt („Steinburgerinnen und Steinburger können ihre privaten Abfälle auf dem größten Wertstoffhof im Kreis Steinburg **unter Berücksichtigung der derzeit gültigen Entgeltordnung** anliefern“) — das betrifft **Itzehoe**, nicht die Hamburger Filiale. Die **Kundenkarte** ist eine **Leistung** („für Stammlieferanten … Vorzugspreise“), **keine** Gebühr. Quelle(n): https://www.isr-recycling.de/index.php/standorte-isr ; https://www.isr-recycling.de/index.php/leistungen-isr ; https://www.isr-recycling.de/index.php/containerverleih-isr ; https://www.isr-recycling.de/index.php/private-entsorgung-isr (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Hinweis zur Typisierung bleibt offen — die Autoverwertung ist für Harburg ausdrücklich ausgeschlossen.** Die Seite **„Autoverwertung“** (https://www.isr-recycling.de/index.php/autoverwertung-isr, HTTP 200) sagt wörtlich: „Autoverwertung ist jedoch **nicht an allen unserer Standorte** möglich. Sie können Ihr Auto auf den folgenden Schrottplätzen abwracken: **Berlin, Elmshorn, Itzehoe, Lübeck, Wahlstedt**“ — **Hamburg-Harburg steht nicht in dieser Liste**. Die Vorwellen-Notiz, dass `trader_type: autoverwertung` für diesen Dossier **nicht** belegt ist, bleibt damit **wortgleich** bestehen und wird **nicht** korrigiert: der Dossier beschreibt einen **Schrotthandel/Metallgroßhandel**, und `trader_type` wird **nicht** geändert, weil die Typentscheidung eine eigene Klärung mit Vorbeleg braucht. Ergänzend: das **SchrottRegister** (Fachbetrieberegister/Register der Entsorgungsfachbetriebe) führt „**ISR Recycling GmbH & Co. KG**“ mit **Zertifikatsnummer ZZHT00300405** und den Tätigkeiten **Lagern** und **Behandeln** (Abfallschlüssel 160212*, 160215*, 200123*) sowie eine gesonderte Eintragung „**ISR Recycling GmbH & Co. KG Zentrale Hafenstraße**“ in Itzehoe — **eine Eintragung speziell für den Standort Hamburg-Harburg ist nicht vorhanden**. Quelle(n): https://www.isr-recycling.de/index.php/autoverwertung-isr ; https://schrottregister.pages.dev/betrieb-isr-recycling-gmbh-co-kg-itzehoe ; https://schrottregister.pages.dev/ort-itzehoe (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Hausnummern-Gegenrechnung abgeschlossen — die Aufzählung ist bestätigt.** Nachgerechnet über die Zähllogik von `scripts/release/osm_street_scan.py` (Filter-Key `addr:street=Wilhelm-Weber-Straße` **mit** `addr:housenumber`, Kachelraster 0,01 Grad über das **vollständige** Raster-BBox **bbox=9.9643159,53.4600016,9.9890130,53.4839983**, 9/9 Teilfenster vollständig verfügbar): **0 Adressobjekte / 0 verschiedene Hausnummern** — **deckungsgleich** mit der Angabe im Recherche-Vermerk oben: jede dort genannte Nummer existiert live, und jede live vorhandene Nummer ist dort genannt. Die **Dossier-Hausnummer 3** ist auf dieser Straße auch in der Gegenrechnung **nicht** vorhanden — das deckt sich mit dem Recherche-Vermerk oben. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9643159,53.4600016,9.9890130,53.4839983 (Raster vollständig, live abgerufen) (Abruf 31.10.2026).]

### Geokodierung 01.11.2026

- [Geokodierung 01.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Hausnummer **3** findet sich in OSM
  auf der Dossier-Straße **nicht**, deshalb wird bewusst **keine** Koordinate gesetzt. **Was geprüft wurde (live am
  01.11.2026):** (a) Die Straße ist als Route erfasst, aber **ohne jede Adressierung**: way/5836066 und
  way/607682372 tragen `name`=Wilhelm-Weber-Straße, `highway`=residential, **keine** `addr:*`-Tags; im
  Kachelraster **bbox=9.9643159,53.4600016,9.9890130,53.4839983** (9 Kacheln, 0 Kachel-Fehler, 34 717 Elemente)
  ist die Zahl der Adressobjekte mit `addr:housenumber` **0**, es gibt **kein** Objekt mit
  `addr:street`=Wilhelm-Weber-Straße. (b) Im erweiterten Raster **bbox=9.9550,53.4650,10.0050,53.4900** (56 183
  Elemente) sind die **einzigen** Treffer zu „Wilhelm-Weber“ die beiden Straßenways, die beiden
  Haltestellenpaare (node/2030476775, node/5758449262, node/5758449263, node/5758449264) und die
  Haltestellenrelation 8446116 — **kein** Adressobjekt und **kein** Betriebsobjekt. (c) Die Suche nach den
  Adressobjekten mit `addr:housenumber`=3 und `addr:postcode`=21079 im erweiterten Raster liefert **sechs**
  Objekte auf **anderen** Straßen (Veritaskai node/308908629 und way/131615718, Dampfschiffsweg way/128839698,
  Hafenbezirk way/128913246, Oberer Traunweg way/132401406, Moorburger Straße way/229651046) — **keines** davon
  trägt den Betriebsbezug. (d) Eine Suche nach „isr“, „scrap“, „schrott“, „recycl“, „metall“ und „abbruch“ über
  **alle** Tag-Werte im erweiterten Raster findet **kein** Objekt, das ISR oder einen Schrott-/Metallbetrieb an
  dieser Straße benennt; die einzigen Recycling-Treffer sind kommunale Sammelstellen (node/393842954 und
  node/1656097066 mit `operator`=Stadtreinigung Hamburg, node/6651740362 mit `operator`=Falke Recycling,
  node/7623197796 mit `operator`=Loop, node/31255112, node/8481044621) sowie way/23495935 mit
  `addr:housenumber`=15, `addr:street`=2. Hafenstraße (`building`=yes, **ohne** `name`/`operator`). Quelle 1
  (Betreiber): Die Standortseite der Betreiber-Website nennt die Hamburger Filiale **„Schrotthandel Hamburg
  Harburg“, ISR Recycling GmbH & Co. KG, Wilhelm-Weber-Str. 3, 21079 Hamburg, 0 40 - 77 14 41** (live abgerufen
  01.11.2026, https://www.isr-recycling.de/index.php/standorte-isr); das Hafenbranchenbuch nennt dieselbe
  Rechtsperson unter derselben Anschrift und derselben Telefonnummer
  (https://www.hafen-hamburg.de/de/adressen/isr-itzehoer-schrott-und-recycling-gmbh-und-co-kg-7803/, live abgerufen
  01.11.2026) — die Anschrift ist damit **betreiberseitig doppelt belegt**, aber in OSM **nicht** auffindbar.
  Quelle 2 (OSM): das Fehlen der Dossier-Hausnummer — kein Objekt mit `addr:housenumber`=3 und
  `addr:street`=Wilhelm-Weber-Straße existiert. **Was fehlt:** ein adressgenauer OSM-Punkt für
  Wilhelm-Weber-Str. 3; ohne ihn wäre jede Koordinate geraten. Der im Produktionsbestand (read-only) geführte
  Punkt **53.4537727/9.9812636** (Baererstraße/Maretstraße, Hamburg-Heimfeld) bleibt als **falscher**
  Fremdpunkt dokumentiert (rund 2 km nördlich der OSM-Wilhelm-Weber-Straße) und ist nicht in die Dossierseite
  übernommen worden; `street`, `postcode`, `phone`, `website` und `status` **aktiv** bleiben unverändert.]
