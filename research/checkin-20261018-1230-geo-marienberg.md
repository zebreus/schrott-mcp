# GEO-Prüfung Marienberg — 18.10.2026

## Entscheidung

**Neues Paar belegbar: `lat: 50.64031`, `lon: 13.15493` (WGS84). GEO-Freigabe als Betreiber-Standortmarker empfohlen, nicht als vermessener Geländemittelpunkt oder bestätigte Einfahrt.** Der offizielle Standortfinder liefert genau dieses Paar im selben Datensatz wie Scholz Recycling GmbH, Reitzenhainer Straße 35 c, 09496 Marienberg, Telefon und Standortmail. OSM bestätigt dort unabhängig die konkrete Hausnummer und den Namen am Gebäude; kein bloßer Orts-/Straßentreffer.

**Altpunkt `50.6132 / 13.1883` für diese Betriebsstätte nicht haltbar.** Er liegt rund **3,82 km südöstlich** des Betreiberpunkts, im OSM-Waldpolygon statt am identifizierten Betrieb. Keine Evidenz für eine Scholz-Betriebsstätte am Altpunkt gefunden. Das ist keine Aussage über historische Verlagerungen: Eine solche Kette wurde nicht untersucht oder behauptet.

Nur dieser Bericht wurde geschrieben. Keine Dossieränderung, keine Rust-Geokodierung, kein Produktions-/DB-Zugriff mit Schreibwirkung, kein Commit/Deploy. Das Produktionsaltpaar ist Auftragsvorgabe, nicht durch einen neuen DB-Abruf verifiziert.

## Quellen und direkte Befunde

Alle folgenden externen Quellen am **18.10.2026** live abgerufen. Koordinaten durchgehend Breite/Länge.

### 1. Betreiber — ausschlaggebendes Koordinatenpaar

- Standortfinder: https://www.scholz-recycling.com/standorte/
- Öffentlicher JSON-Endpunkt: https://www.scholz-recycling.com/standorte-json-nicht-loeschen/
- Eindeutige Datensatzselektion: `ort == "Marienberg"`, `strasse == "Reitzenhainer Straße"`, `hausnummer == "35 c"`, `plz == "09496"`. Kein generischer Konzernmarker und kein gleichnamiger fremder Betrieb.
- Relevanter Originalauszug:

```json
{
  "Name": "Scholz Recycling GmbH",
  "Unternehmen": "Scholz Recycling GmbH",
  "lat": "50.64031",
  "lng": "13.15493",
  "strasse": "Reitzenhainer Straße",
  "hausnummer": "35 c",
  "plz": "09496",
  "ort": "Marienberg",
  "telefon": "+49 3735 22495",
  "email": "marienberg@scholz-recycling.de"
}
```

Der Datensatz enthält außerdem standortspezifische Öffnungszeiten und Feiertagshinweise 2026. Das stützt die aktuelle Standortzuordnung, ist aber kein Vermessungsdatum der Koordinaten. Kein Genauigkeitsradius und keine Kennzeichnung als Tor, Zufahrt oder Flächenzentrum im gelesenen Datensatz.

### 2. Ein einzelner, gezielter Nominatim-Adressabgleich

Abgerufene URL (ein Request, keine Variantenserie/Reverse-Bulkabfrage):

https://nominatim.openstreetmap.org/search?street=Reitzenhainer%20Stra%C3%9Fe%2035%20c&city=Marienberg&postalcode=09496&country=Deutschland&format=jsonv2&addressdetails=1&limit=3

Antwort: genau ein Treffer, `osm_type: way`, **`osm_id: 587548042`**, `category: building`, `type: yes`, `name: Scholz Recycling GmbH`, Hausnummer `35C`, Straße Reitzenhainer Straße, Ort Marienberg, PLZ 09496; zusätzlich Ortsteilzuordnung Wüstenschlette. Repräsentationspunkt **`50.6404444 / 13.1549269`**. Boundingbox Breite 50.6403886–50.6405002, Länge 13.1547627–13.1550911.

Dieser Punkt liegt etwa **15 m nördlich** des Betreiberpunkts und repräsentiert das Gebäude, nicht das gesamte Schrottgelände. Nominatim und OSM sind hier **keine zwei unabhängigen Belege**, sondern dieselbe Kartendatenbasis.

### 3. OSM-Objekt direkt gegengeprüft

- Objektseite: https://www.openstreetmap.org/way/587548042
- Vollständige Objektgeometrie und Tags: https://www.openstreetmap.org/api/0.6/way/587548042/full
- Way **587548042**, Version **2**, letzter ausgewiesener Objektzeitstempel **2018-11-18T15:59:08Z**, Changeset **64624063**.
- Tags: `name=Scholz Recycling GmbH`, `building=yes`, `addr:city=Marienberg`, `addr:street=Reitzenhainer Straße`, `addr:housenumber=35C`, `addr:postcode=09496`.
- Eckknoten **5612177572–5612177575**. Das kleine Gebäude ist ungefähr 23 × 12 m entlang der Boundingbox; keine flächendeckende Scholz-Betriebsumgrenzung.

**Genauigkeitsfolgerung:** Der Betreiberpunkt liegt südlich außerhalb dieses Gebäudeumrisses, aber in unmittelbarer Standortnähe. Deshalb nicht als Gebäudezentrum bezeichnen. Die Übereinstimmung ist auf Betriebs-/Adressniveau stark; die 15-m-Differenz ist kein relevanter Standortwiderspruch. Die fünf Nachkommastellen sind Quellpräzision, kein Beleg für Meter-Vermessungsgenauigkeit.

## Umgebung, Zugang und Gegenbelege

### Umfeld des neuen Punkts

Gezielter OSM-Kartenausschnitt:

https://www.openstreetmap.org/api/0.6/map?bbox=13.1515,50.6380,13.1590,50.6425

- **Way 330009090**, `landuse=industrial`, ist die größere Gewerbegebietsfläche. Sie umfasst auch andere Gebäude/Betriebe und darf **nicht** als Scholz-Geländegrenze oder dessen Mittelpunkt verwendet werden.
- **Way 61687884**, Reitzenhainer Straße **35**, `building=retail`, zusätzlich `railway=workshop`, ist ein anderes Gebäude nordöstlich. Nicht mit **35C** gleichsetzen.
- **Way 587548045**, Hausnummer **35a**, liegt weiter östlich. Kein Ersatz für den punktgenauen 35C-Treffer.
- **Way 587544623**, Reitzenhainer Straße **33**, trägt `name=Fink Farben GmbH`; **Way 239220327**, **31B**, trägt `name=Bach GmbH`. Die gemeinsame Straße und Gewerbegebietsfläche allein würden keine Betreiberzuordnung rechtfertigen.
- **Way 61687893**, `highway=service`, `surface=asphalt`, führt von der Reitzenhainer Straße westwärts bis in die Nähe des Scholz-Gebäudes und verzweigt/umschließt dort Flächen. Der Anschluss an die öffentliche Straße liegt am Knoten **768858555**, **50.6410058 / 13.1577910**; öffentliche Straße dort **Way 364002956**. Das ist eine kartierte Gebietserschließung, **keine bestätigte separate Scholz-Kundenzufahrt**. Den Straßenanschluss nicht als neues Händlerpaar übernehmen.
- Im gelesenen Ausschnitt kein als Scholz-Tor eindeutig benanntes Objekt. Die tatsächlich zu nutzende Annahmezufahrt, Waage und der genaue Mittelpunkt des gesamten Betriebsgeländes bleiben offen. Keine Luftbild- oder Vor-Ort-Verifikation durchgeführt.

### Umfeld des Altpunktes

Separater kleiner Ausschnitt, nicht aus dem neuen Punkt abgeleitet:

https://www.openstreetmap.org/api/0.6/map?bbox=13.1858,50.6115,13.1908,50.6149

- Altpunkt liegt bei geometrischer Punkt-in-Polygon-Prüfung der gelieferten Way-Knoten **innerhalb Way 488647765**, `landuse=forest`, Version **6**, Zeitstempel **2026-10-09T20:11:29Z**, Changeset **190271841**: https://www.openstreetmap.org/way/488647765 .
- Nahe kartierte Umgebung: **Försterwiesen** (`landuse=meadow`, Way **112876605**), **Halbmeilenbachweg** (Wegabschnitte **334374543**, **334374545**) und **Rote Pockau**, `alt_name=Halbmeilenbach` (u. a. Way **853229113**).
- Im geprüften Ausschnitt kein Scholz-/35C-Objekt als Gegenbeleg zum neuen Standort. Fehlende OSM-Eintragung allein wäre kein Nicht-Existenzbeweis; ausschlaggebend ist die Kombination aus deutlicher räumlicher Abweichung, Waldgeometrie und positivem Betreiber-/Adressbeleg am neuen Punkt.
- Distanz Altpunkt → Betreiberpunkt selbst aus den beiden WGS84-Paaren mit Haversine (Erdradius 6.371 km) berechnet: rund **3.825 m Luftlinie**, kein Routingwert.

## Freigabegrenze für den Owner

1. Für einen normalen Händler-Standortpin ist **`50.64031 / 13.15493` freigabefähig**, mit Provenienz Betreiber-JSON und unterstützendem adress-/namenskongruentem OSM-Gebäude. Keine manuelle Mittelung der beiden Paare nötig.
2. Punktart dokumentieren als **„Betreiber-Standortmarker unmittelbar beim adresskongruenten Betriebsgebäude; kein bestätigter Zufahrtspunkt, kein vermessenes Geländezentrum“**. Ein numerischer Genauigkeitsradius ist nicht seriös belegt.
3. Falls das Freigabegate ausdrücklich ein bewiesenes Tor oder echtes Flächenzentrum verlangt: **diese höhere Punktgenauigkeit bleibt offen**; nicht aus dem allgemeinen Gewerbepolygon oder dessen Erschließung erfinden.
4. Karten lokalisieren ausschließlich den bereits identifizierten Betrieb. Sie beweisen keine Firmen-, Rename-/Merge- oder Umzugskette. Der im Dossier dokumentierte Betreiber-/Register-Gate wird hier nicht durch OSM ersetzt.
5. Ein späteres Entfernen leerer/fehlender Dossierkoordinaten würde laut README bestehende DB-GEO nicht löschen. Dieser Bericht nimmt weder eine solche Änderung noch die Übernahme des neuen Paars vor.

Recherchegrenzen: Websuche war technisch nicht verfügbar (HTTP 401); die entscheidenden Betreiber- und OSM/Nominatim-Primärabrufe waren erfolgreich. Keine Nominatim-Beschränkung umgangen; nach dem einen erfolgreichen Adressrequest keine weiteren Nominatim-Abfragen.
