# GEO-Owner-Prüfung: TSR Chemnitz, Frankenberger Straße 155a

**Abrufdatum aller nachfolgend tatsächlich abgerufenen Webquellen: 18.10.2026.**
**Umfang:** ausschließlich Recherchebericht zu `dossiers/sn/sn-chemnitz-tsr-deutschland-nl-chemnitz.md`; keine Dossier-, Rust-, Produktions- oder DB-Änderung, kein Commit/Deploy.

## Ergebnis / Freigabevorschlag

- **Altpunkt `50.82073333333334 / 12.910466666666668` ist für den belegten Standort nicht haltbar.** Er liegt rund **4,93 km südwestlich** des adresskongruenten OSM-Gebäudes in einem kartierten Wohngebiet bei Parkstraße/Herderstraße (Kapellenberg), nicht an der Frankenberger Straße 155a.
- **Neues Paar ist als Gebäudereferenzpunkt belegbar:** **`lat: 50.860415`, `lon: 12.941881` (WGS84)**, geometrisch aus dem explizit `TSR Recycling` und vollständig `Frankenberger Straße 155a, 09131 Chemnitz` getaggten Gebäude **OSM way 520190347** abgeleitet.
- **Punktart ausdrücklich Gebäudemitte, weder nachgewiesenes Gesamtgeländezentrum noch Zufahrt/Tor/Annahmeplatz.** GEO-Freigabe sinnvoll, sofern ein adress-/betriebsbezogener Gebäudepunkt dem Produktstandard genügt. Für navigationsgenaue Kundenanfahrt bleibt die Freigabe offen. Keine Vermessungs- oder Meterpräzision behaupten.
- Die aktuelle Betreiberzuordnung wird hier **vorausgesetzt gemäß gesonderter Owner-Prüfung im Dossier**, nicht aus OSM abgeleitet. OSM lokalisiert den bereits identifizierten Betrieb; der dort ältere Name `TSR Recycling` beweist weder aktuelle Rechtsform noch Firmen-/Nachfolgekette.

## 1. Betreiberkarte: Abrufgrenze und Adressanker

Kanonische Betreiber-Detailseite:

<https://www.tsr-recycling.de/standorte/schrotthaendler-chemnitz/>

Direkter HTML-Abruf lieferte **„Challenge Validation“**, keinen auswertbaren Seiteninhalt und keine Kartenkoordinaten. Keine Challenge gelöst, keine Sperre umgangen. Der Suchindex zeigte zur selben URL `TSR Deutschland GmbH & Co. KG`, `Frankenberger Straße 155a`, `09131 Chemnitz`, Telefon `+49 371 3563998-0` und den Linktext „Anfahrt“; das ist **nur Suchindexsicht**, kein live gelesener Kartenmarker. Daher wird hier **kein Paar als Betreiberkartenkoordinate ausgegeben**. Die detaillierte Betreiber-/Registerprüfung im bereits vorhandenen Dossier wurde gelesen, aber nicht als eigene neue Registerprüfung ausgegeben.

## 2. Konkretes OSM-Objekt und reproduzierbare Koordinate

**Objekt:** <https://www.openstreetmap.org/way/520190347>

**Direkt gelesene Primärdaten inklusive Geometrie:**
<https://www.openstreetmap.org/api/0.6/way/520190347/full.json>

Aktueller ausgelieferter Objektstand:

- way **520190347**, **Version 5**, letzter Objektzeitstempel **2019-11-16T20:59:33Z**, changeset **77175091**.
- Tags: `name=TSR Recycling`, `building=commercial`, `addr:street=Frankenberger Straße`, `addr:housenumber=155a`, `addr:postcode=09131`, `addr:city=Chemnitz`.
- Kein `operator`-Tag und kein eigenständiger Eingangspunkt am Gebäude in den gelesenen Geometriedaten.
- Vier Gebäudeecken, Reihenfolge des geschlossenen Ways:

| OSM node | Breite | Länge |
| --- | --- | --- |
| 5070711907 | 50.8605632 | 12.9414510 |
| 5070711909 | 50.8606248 | 12.9422421 |
| 5070711910 | 50.8602677 | 12.9423119 |
| 5070711908 | 50.8602061 | 12.9415208 |

Die vier Ecken bilden ein Parallelogramm. Dessen Flächenschwerpunkt beträgt **50.86041545 / 12.94188145**; hier sind auch Bounding-Box-Mitte und Mittelwert der vier verschiedenen Ecken identisch. Vorgeschlagenes gerundetes Paar: **50.860415 / 12.941881**. Berechnung lokal aus den gelesenen OSM-Punkten, **kein Geocoderergebnis und keine Rust-Geokodierung**.

**Genauigkeit:** Der Punkt liegt innerhalb des ausdrücklich benannten und adressierten Gebäudes. Ausdehnung des Gebäudeumrisses rund 47 m Nord–Süd und 60 m Ost–West; das beschreibt den Umriss, **keinen gemessenen Fehlerkreis**. Sechs Nachkommastellen sind Speicher-/Reproduktionspräzision, keine Bestätigung einer Submeter-Lage. Ein amtlicher Katasterabgleich, aktuelles Orthofoto, Vor-Ort-Abgleich oder Betreiberbestätigung zur konkreten Annahmestelle erfolgte nicht. Alter des OSM-Tags (2019) bleibt ein Aktualitätsvorbehalt; die heutige exakte Adresskongruenz ist der entscheidende Anschluss an den separat belegten Betrieb.

## 3. Zufahrt und Umgebung: keine freie Torinterpretation

Zwei gezielte öffentliche OSM-Kartenausschnitte wurden gelesen (keine flächendeckende Abfrage):

- <https://www.openstreetmap.org/api/0.6/map?bbox=12.949,50.856,12.965,50.866>
- <https://www.openstreetmap.org/api/0.6/map?bbox=12.940,50.857,12.949,50.863>

Der erste Ausschnitt entlang der Frankenberger Straße ergab zunächst nur andere Hausnummern; der angrenzende zweite Ausschnitt enthält das deutlich weiter westlich hinter der Straßenfront liegende Gebäude 155a. **Nicht von der Hausnummer aus einen Punkt an der Hauptstraße interpolieren.** Das Umfeld ist dicht mit Bahn-/ehemaligen Bahnanlagen und mehreren Gewerbegebäuden kartiert. Weder ein beliebiges Gleis noch ein allgemeines Industriepolygon ist deshalb automatisch TSR-Gelände.

**Adressnachbarn / Abgrenzung:**

- **way 520299026:** Frankenberger Straße **155**, Gebäude bei ungefähr 50.8599 / 12.9433 — nicht 155a.
- **way 520299699:** Frankenberger Straße **155c**, ebenfalls eigener Gebäudeumriss.
- **way 982295917:** Frankenberger Straße **155d**, `name=WMS Werkzeugmaschinen-Service GmbH`, `website=https://wms-chemnitz.com/`; eigener Gebäudeumriss südlich von TSR. Nicht dessen Mittelpunkt übernehmen.

**Zufahrtsindiz, kein verifiziertes Tor:**

<https://www.openstreetmap.org/way/520299019>

<https://www.openstreetmap.org/api/0.6/way/520299019.json>

way **520299019**, Version **5**, Zeitstempel **2022-07-25T13:36:28Z**, changeset **124053264**: `highway=service`, `service=driveway`, `access=customers`. Der Weg läuft von östlichen Anschlusswegen bis an die Südseite des benannten Gebäudes. Er enthält unter anderem node **5760741192**, **50.8602206 / 12.9424751**, nahe der Südostecke, und node **6680166378**, **50.8601407 / 12.9414252**, südwestlich des Gebäudes. Das sind **Straßen-/Wegstützpunkte**, keine als `entrance`/TSR-Tor belegten Kundenannahmepunkte. Der im Kartenausschnitt enthaltene umgebende driveway **way 710508126** führt um das Gebäude; ebenfalls keine ausreichende Grundlage, einen einzigen Punkt als amtliche/bestätigte Zufahrt auszugeben.

**Folgerung:** Das neue Referenzpaar ist gerade **nicht** auf die mutmaßliche Zufahrt verschoben. Gesamtgeländegrenzen und genaue Torposition bleiben offen.

## 4. Altpunkt: konkreter Gegenbeleg

Gelesener kleiner Vergleichsausschnitt:

<https://www.openstreetmap.org/api/0.6/map?bbox=12.9094,50.8200,12.9115,50.8214>

Lokaler Punkt-in-Polygon-Abgleich mit den ausgelieferten Way-Koordinaten: Der Altpunkt liegt innerhalb **OSM way 1347024488**, `landuse=residential` (`source=bing`). Das ist ein tatsächlicher Flächengegenbeleg, nicht nur das Ausbleiben eines TSR-Suchtreffers.

- Nahe Kartierungsanker: **node 1191282856**, **50.8209573 / 12.9105397**, `addr:street=Parkstraße`, `addr:housenumber=9a`, `addr:postcode=09120`, `addr:suburb=Kapellenberg`, `entrance=staircase`.
- **way 303961249** trägt `name=Parkstraße`, `highway=residential`, `maxspeed=30`.
- Nachbarschaft außerdem Herderstraße, ebenfalls Wohnadressen. Der Altpunkt ist somit weder eine unpräzise Mitte des neuen TSR-Grundstücks noch nur ein abweichender Eingang desselben Betriebs.
- Haversine-Distanz Altpunkt → neuer Gebäudeschwerpunkt, Erdradius 6371 km: **4,933 km**. Keine Aussage zur Fahrstrecke.

Objektlinks: <https://www.openstreetmap.org/way/1347024488>, <https://www.openstreetmap.org/node/1191282856>, <https://www.openstreetmap.org/way/303961249>.

## 5. Quellenkritik und Fehlspuren

- Eine kleine Overpass-Abfrage nach `name~TSR` im Chemnitzer Bereich endete mit **HTTP 406**; einmaliger normaler Kontrollabruf per Standard-HTTP ebenfalls 406. Danach keine weiteren Versuche, keine Ausweichinstanz und keine Umgehung. Die reguläre OSM-Karten-/Objekt-API war öffentlich erreichbar; sie wurde für die oben dokumentierten kleinen Ausschnitte und benannten Objekte verwendet.
- Suchergebnisse lieferten zunächst OSM nodes **13593152258** und **155542944**. Direkte Objektabfragen zeigten **42.5906023 / -75.5818069** bzw. **37.3407772 / -81.1646292**, also offenkundig keine Chemnitzer Treffer. **Verworfen**, nicht als GEO-Belege verwendet. Auch das suchseitig angebotene changeset **110843641** betraf nach direktem Abruf einen anderen Ort, kein TSR-Chemnitz-Beleg.
- Ausbildungsportal <https://www.perspektiven-finden.com/unternehmen/trs-recycling-gmbh-co-kg> live gelesen: alter Name `TSR Recycling GmbH & Co. KG`, Adresse Frankenberger Str. 155a. Das Portal warnt selbst, dass seine Google-Anfahrtskarte automatisch erstellt werde und Korrektheit/Aktualität nicht garantiert seien. **Nur Hinweis, kein Betreiberkettenbeleg und kein Koordinatenfreigabebeleg.** Kein dortiger Maps-Schlüssel verwendet.
- Cylex-Abfrage scheiterte mit HTTP 403; nicht umgangen und nicht als Beleg gerechnet.
- **Keine Nominatim-Anfrage** ausgeführt; vom Suchdienst angebotene Nominatim-URLs nicht abgerufen. Keine Bulkrequests, kein Geocodinglauf.
- **Scholz/Uhlestraße 20 wurde nicht als Ersatzbetrieb oder GEO-Kandidat verwendet.** Die Zuordnung des Altpunktes zu irgendeinem anderen Recycler wird ausdrücklich nicht behauptet.

## Owner-Entscheidung

**Empfehlung: Altpunkt verwerfen; `50.860415 / 12.941881` ausschließlich mit Timeline-Hinweis „OSM way 520190347, adresskongruentes TSR-Gebäude; abgeleitete Gebäudemitte, nicht Tor/Geländezentrum; Abruf 18.10.2026“ zur Übernahme freigeben.** Falls das Datenprodukt zwingend eine verifizierte Kundenzufahrt verlangt: GEO offen lassen, Betreiberbestätigung oder belastbare aktuelle Anfahrtskarte nachfordern. Dieser Bericht führt die Übernahme selbst nicht aus.
