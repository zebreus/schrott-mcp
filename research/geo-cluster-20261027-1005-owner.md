# Essener GEO-Cluster: Owner-Disposition, 27.10.2026, 10:05 UTC

## Systematischer Befund

Der in 17 Händlerzeilen gemeinsam gespeicherte Punkt `51.450284375, 7.015053125` ist über
den OSM-Weg **46961889** eindeutig identifiziert:

- `name=Freiheit`, `highway=service`, `bus=designated`, `coach=yes`, `psv=designated`,
  `vehicle=no`, `oneway=yes`, `maxspeed=30`, `description=Fernbus Linien und Reisebusse`,
  `note=Die stop_position heißt nicht umsonst 'Essen, Fernbusbahnhof'`.
- Die Haltestelle auf diesem Weg (Knoten 599294635) trägt `amenity=bus_station`,
  `name=Essen, Fernbusbahnhof`, `int_name=Essen, long-distance bus station`.

Der Punkt liegt damit auf der **Fernbus-Spur des Essener Hauptbahnhofs** und nicht auf einem
Gewerbestandort. In den 86 Umweltinspektionsberichten der Bezirksregierung Düsseldorf für
die Stadt Essen (alle am 27.10.2026 einzeln abgerufen) existiert kein Eintrag auf der
Straße „Freiheit". Belege: https://api.openstreetmap.org/api/0.6/way/46961889/full.json ,
https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125
&zoom=18&addressdetails=1

## Dispositionsregel

Der Punkt ist für **keine** dieser Zeilen ein belegter Standort. Ob er zurückgenommen
wird, entscheidet die Zeile selbst:

1. **Zurücknehmen (`clear_fields: geo`, ohne Ersatzkoordinate)**, wenn die Zeile eine eigene
   Ortsangabe hat und diese dem Punkt widerspricht: kuratierte Dossier-Adresse oder
   Betreiber-Primärquelle. Der gespeicherte Punkt behauptet dann etwas, das die eigene
   Zeile widerlegt.
2. **Als offener Korrekturauftrag dokumentieren, nicht pauschal zurücknehmen**, wenn die
   Zeile keine eigene, belastbare Ortsangabe hat. Der Punkt ist dann nur ein
   Stadt-Platzhalter ohne Ortsaussage; eine Rücknahme würde eine Behauptung über einen
   Standort ersetzen, den niemand belegt hat. Grundsatz: kein Pauschal-Clear.

Ein Punktreihenfolge- oder Statuswechsel ist in keinem Fall Teil der Korrektur. Eine
Rücknahme ist „kein kuratierter aktueller Punkt“, nicht Schließung, nicht Annahme- oder
Leistungsverweigerung.

## Zurückgenommen (11 Zeilen)

| Slug | Eigene Ortsangabe der Zeile | Forward-Treffer | Abstand | Art des Belegs |
|---|---|---|---|---|
| `nw-essen-mre-metall-recycling-essen` | Bonifaciusring 5, 45309 | Betriebsgelände, Waagen, Anlieferung | 5,81 km | Betreiber-Primärquelle + unabhängiges Verzeichnis |
| `nw-essen-schrott-busters` | Fischweiher 26, 45307 | Wohn-/Mischobjekt | 4,38 km | Betreiber-Impressum + 11880 |
| `nw-essen-schrott-nowak` | Bonnekampstraße 14, 45327 | Wohnobjekt | 5,84 km | Betreiber-Impressum (Archiv) + 11880 + gültige USt-IdNr (VIES) |
| `nw-essen-schrotthandel-missal-metzler` | Kalthofweg 4, 45277 | Wohnobjekt | 5,17 km | Betreiber-Impressum + 11880 |
| `nw-essen-cbr-bracke` | Manderscheidtstr. 91 | Gewerbegebiet Ernestine | 3,57 km | kuratierte Adresse (Verzeichnisprofil) |
| `nw-essen-eie` | Gelsenkirchener Str. 181, 45309 | Ruhr Museum | 4,38 km | kuratierte Adresse |
| `nw-essen-hade-schrott-metallgrohandel-stamm` | Heßlerstr. 77 | Altenessen-Nord | 6,47 km | kuratierte Adresse; Betreiber nennt Alte Bottroper Str. 120 |
| `nw-essen-meinrich` | Keunefeld 9, 45355 | Bochold | 4,41 km | kuratierte Adresse; Register HRB 12559 |
| `nw-essen-seliger` | Vöcklinghauser Str. 1, 45130 | Rüttenscheid | 1,19 km | kuratierte Adresse (Verzeichnisbeleg) |
| `nw-essen-thp-metall-schrott-handel` | Schlenhofstr. 10 | Segeroth | 1,79 km | kuratierte Adresse (city-map) |
| `nw-essen-zjm-demontagen-hausmeisterservice` | Von-Ossietzky-Ring 59, 45279 | Horst | 7,18 km | kuratierte Adresse; Betreiberkandidat nennt Nr. 61 |

Zusätzlich wurde bei `nw-essen-schrott-nowak` die fehlerhafte Frontmatter-Adresse
„Schrott Nowak Essen Marktstr. 14“ geleert: Sie mischt Name und Stadt in das Adressfeld,
und „Marktstraße 14“ existiert in Essen nur in 45355, nicht in 45327.

## Offen und dokumentiert (6 Zeilen, kein Clear)

`nw-essen-bogner-bogner`, `nw-essen-kollner`, `nw-essen-l-g-r`, `nw-essen-schrottladys`,
`nw-essen-schuttler`, `nw-essen-schrotthandler-traut`. Für diese Zeilen gibt es keine
kuratierte Adresse und keinen Betreiber- oder Behördenbeleg, der eine Ortsangabe stützt
(11880-, city-map-, Ruhrpott-Kurier- und Schrottfinder-Leads ohne eigene Betreiberbrücke;
beim L.G.R.-Fall zusätzlich ungeklärter Identitäts- und Branchenwiderspruch). Der Punkt
bleibt als Stadt-Platzhalter stehen und ist in der Dossier-Timeline als **offener
Korrekturauftrag** vermerkt: entscheiden erst mit kuratierter Adresse oder belegter
Annahmestelle.

## Übergreifender Punkt für eine eigene Kampagne

Die Produktion enthält weitere identische Punkt-Cluster, die denselben Charakter haben
(Stadt-Platzhalter statt Händlerstandort): Frankfurt 27 Zeilen, Koblenz 27, Darmstadt 25,
Mannheim 24, Wiesbaden 24, Hanau 21, Bochum 21, Köln 19, Ludwigshafen 18, Trier 18,
Dresden 18, Leipzig 17, Hannover 17, Worms 16 (read-only Abfrage am 27.10.2026). Anders
als der Essener Punkt sind das tatsächliche Stadtzentren; die Dossiers dazu führen teils
selbst eine konkrete Adresse, der Seed-Bestand hält dort aber den Stadtpunkt, weil die
Dossiers keine Koordinaten tragen. Das ist eine eigene, bundeslandweite
Bereinigungsfrage mit je Händler zu klärender Beleglage und **kein** Bestandteil dieser
Korrektur. Bis zu ihrer Entscheidung bleiben diese Punkte unverändert.
