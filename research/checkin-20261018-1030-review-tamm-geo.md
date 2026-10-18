# Unabhängiges GEO-/Adresspunktreview: Tamm, Check-in 18.10.2026 10:30

## Entscheidung

**FREIGABE des vorhandenen Paars `lat: 51.2247867`, `lon: 7.2141755` ausschließlich als OSM-gestützter Gebäude-/Adresspunkt für Gasstraße 11, 42369 Wuppertal.** Der aktuelle Nominatim-Treffer liefert genau dieses Paar und den behaupteten OSM-Way; die OSM-Primärdaten bestätigen Hausnummer, Straße, Postleitzahl, Stadt und Industriegebäude. **Keine Freigabe als Einfahrt, Annahmestelle oder genauer Navigations-/Lkw-Zufahrtspunkt.**

**Dokumentationsblocker für eine unveränderte Gesamtfreigabe des Entwurfs:** Der Überblick behauptet weiterhin, es seien keine Koordinaten ergänzt und Geocoding sei noch nötig. Das widerspricht dem Frontmatter und dem späteren GEO-Eintrag. Die auf 12.10.2026 datierte Abrufbehauptung wird durch dieses Review nicht historisch bestätigt. Das Koordinatenpaar selbst ist aktuell belegt; die widerspruchsfreie Herkunftsdokumentation muss der Owner separat herstellen.

## Gegenstand und Vergleich mit HEAD

- Geprüft: Arbeitsbaumfassung `dossiers/nw/nw-wuppertal-ronsdorf-tamm.md`, insbesondere Zeilen 16–17, 29, 42 und 44–46, gegen HEAD `a61dcc944d2db7094bbcf429f5f794a4fe6eb959` (`git show HEAD:…`, `git diff -- …`).
- HEAD enthält keine Koordinaten, keine GEO-Timeline und nur die unnormalisierte Seed-Adresse `Tamm GmbH Wuppertal-Ronsdorf Gasstr. 11` bei leerer PLZ. Das Paar und beide 12.-Oktober-Einträge sind Änderungen im Arbeitsbaum. Aus diesem Diff lässt sich ihr tatsächlicher Entstehungs-/Abrufzeitpunkt nicht ableiten.
- Der im Entwurf genannte alte Produktionspunkt `51.259513043478265, 7.164908695652174` wurde nicht gegen eine Datenbank geprüft. Weder dessen Produktionsstatus noch ein erfolgter Austausch werden hier bestätigt.
- Betreiberidentität, Betriebsstatus, Dienstleistungen und Anlieferzeiten sind ausdrücklich nicht Gegenstand dieses Reviews.

## Tatsächlich abgerufene Primärquellen

Die folgenden zwei gezielten Webabrufe lieferten in dieser Sitzung verwertbare Antworten; unmittelbar anschließende UTC-Systemzeit: **2026-10-18T10:30:42Z**. Dies ist das aktuelle Abruffenster, kein Nachweis eines Abrufs am 12.10.2026. HTTP-Header und exakter serverseitiger Antwortzeitpunkt wurden vom Abrufwerkzeug nicht ausgegeben.

### 1. Nominatim: strukturierte Adresssuche

Quelle: <https://nominatim.openstreetmap.org/search?street=Gasstra%C3%9Fe%2011&city=Wuppertal&postalcode=42369&format=jsonv2&addressdetails=1&limit=5>

Die Antwort enthält **einen Treffer**:

```json
{
  "osm_type": "way",
  "osm_id": 481426457,
  "lat": "51.2247867",
  "lon": "7.2141755",
  "category": "building",
  "type": "industrial",
  "addresstype": "building",
  "name": "",
  "address": {
    "house_number": "11",
    "road": "Gasstraße",
    "neighbourhood": "Blaffertsberg",
    "hamlet": "Blombach",
    "suburb": "Gemarkung Ronsdorf",
    "city": "Wuppertal",
    "state": "North Rhine-Westphalia",
    "postcode": "42369",
    "country_code": "de"
  },
  "boundingbox": ["51.2247338", "51.2248396", "7.2140632", "7.2142878"]
}
```

Dies bestätigt einen adressierten Gebäude-Treffer, keinen benannten Tamm-POI. Die Ortsgliederung ist die Nominatim-Zuordnung, kein zusätzlicher amtlicher Adressnachweis.

### 2. OSM: Way einschließlich Geometrie

Tatsächlich abgerufene API: <https://www.openstreetmap.org/api/0.6/way/481426457/full>

Objektlink zur Nachprüfung: <https://www.openstreetmap.org/way/481426457> (nicht als zusätzlicher unabhängiger Beleg gezählt).

Die XML-Antwort enthält Way **481426457, Version 4**, sichtbar, Änderungszeit **2023-05-12T14:33:50Z**, Changeset **136021839**, mit sämtlichen Tags:

```text
addr:city=Wuppertal
addr:housenumber=11
addr:postcode=42369
addr:street=Gasstraße
building=industrial
```

Geschlossener Umring, Reihenfolge aus der Antwort:

| Node | Breitengrad | Längengrad |
| --- | --- | --- |
| 3174769284 | 51.2247928 | 7.2140632 |
| 4743632337 | 51.2247338 | 7.2141009 |
| 4743632338 | 51.2247805 | 7.2142878 |
| 4743632339 | 51.2248396 | 7.2142501 |
| 3174769284 | 51.2247928 | 7.2140632 |

Die gelieferten Node-Versionen tragen jeweils den Änderungszeitpunkt 2021-10-22T23:16:25Z. Diese Objektzeitstempel sind **keine** Abrufzeitstempel und keine Bestätigung aktueller Vor-Ort-Verhältnisse.

## Schwerpunkt und Aussagegrenzen

- Lokaler, rein lesender Rechentest mit den vier gelieferten Eckpunkten: Das Prüfkoordinatenpaar liegt innerhalb des Polygons. Der planare Flächenschwerpunkt beträgt ungefähr `lat 51.22478668213847`, `lon 7.214175521818456`; auf sieben Nachkommastellen gerundet exakt das vorhandene Paar. Die Bezeichnung Gebäude-Schwerpunkt ist damit für diesen Umring plausibel und rechnerisch gestützt, nicht nur aus einer Kartenansicht geschätzt.
- Nominatim und OSM sind **keine zwei unabhängigen Vermessungsquellen**: Nominatim verarbeitet OSM-Daten. Belegt ist die Übereinstimmung zwischen Adressindex, adressiertem OSM-Gebäude und Punkt, nicht eine amtliche Vermessungsgenauigkeit. Sieben Nachkommastellen begründen keine entsprechende reale Genauigkeit.
- Der Way trägt weder `name=Tamm` noch ein `entrance`-/`access`-Tag. Die vollständige Way-Antwort belegt keine Zufahrtsroute, Torlage, Kundenannahme, Grundstücksgrenze oder Nutzungsberechtigung. Aus fehlenden Tags folgt auch nicht, dass es keine Einfahrt gibt.
- Keine amtlichen Adresspunktdaten, Luftbilder, Vor-Ort-Aufnahmen oder Eingangsknoten geprüft. Für die begrenzte Gebäude-/Adresspunktfreigabe besteht nach den aktuellen Antworten kein GEO-Blocker; für Einfahrtpräzision fehlt ein eigenständiger Beleg.

## Umgang mit der widersprüchlichen Historie

1. Der Überblick ist als Beschreibung des aktuellen Zustands falsch/veraltet und sollte durch den Owner an die ergänzten Koordinaten angepasst werden.
2. „Keine Koordinaten gesetzt“ im ersten 12.-Oktober-Timeline-Eintrag kann eine frühere Arbeitsphase beschreiben; ein nachfolgender GEO-Schritt am selben Datum wäre logisch möglich. **Die tatsächliche zeitliche Abfolge und der damalige Nominatim-Abruf sind aber hier nicht belegt.** Aktuelle Treffer dürfen nicht nachträglich als Abrufbestätigung für den 12.10. ausgegeben werden.
3. Zulässige aktuelle Aussage: „Am 18.10.2026 erneut unabhängig gegen Nominatim und OSM geprüft; Gebäudepunkt bestätigt, keine verifizierte Einfahrt.“ Die ursprüngliche Datierung nur mit separatem zeitgenössischem Nachweis als verifiziert behandeln; andernfalls transparent als vorhandene, historisch ungeprüfte Owner-Angabe kennzeichnen.

## Umfang des Eingriffs

Nur diese Reviewdatei erstellt. Dossier unverändert gelassen; keine Datenbankzugriffe/-schreibvorgänge, kein RustGEO, kein Commit. Zwei gezielte öffentliche Quellenabrufe, keine Massenabfragen. OSM-Daten: © OpenStreetMap-Mitwirkende, ODbL 1.0 (<https://www.openstreetmap.org/copyright>).
