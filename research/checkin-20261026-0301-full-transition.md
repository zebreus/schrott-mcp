# Private Vollübergangsprobe, 26.10.2026

## Geltungsbereich

Ergänzung zum [43er-Dossieraudit](seed-only-audit-20261026-owner.md), keine
Ersetzung früherer Prüfungen und keine Freigabe des Releases. Ausgangspunkt war
der read-only Online-Backupstand der Produktions-`public.db` um ca. 03:00 UTC,
Schema v6. Die Kopie wurde privat mit dem aktuellen `PublicDb::open` nach v8
migriert; danach wurden aktueller Material- und Händlerseed angewendet. Der
Übergang wurde auf Zeilen- und Spaltenebene verglichen. Eine zweite
Händler-Seedanwendung schrieb null Zeilen.

Die Probe verwendete weder `internal.db` noch Release-Binaries oder einen
regulären Service-/Schedulerlauf. Produktionsdaten wurden nicht beschrieben.
Die temporären Kopien und der nur für diese Probe angelegte Example-Helper
wurden nach Abschluss entfernt.

## Migration v6 -> v8

| Tabelle | Vorher | Nachher | Änderung |
|---|---:|---:|---|
| `traders` | 3.893 | 3.893 | unverändert |
| `materials` | 83 | 83 | unverändert |
| `prices` | 29.547 | 29.547 | zeilengleich |
| `current_prices` | 2.683 | 2.659 | 24 Zeiger entfernt |
| `trader_materials` | 1.401 | 1.401 | 5 Zeilen geändert |

Die 24 entfernten Current-Zeiger und die fünf geänderten Annahmen gehören zum
Heraeus-Goldstore Hanau. Frühere Preiszeilen blieben vollständig erhalten; die
fünf Annahmen stehen danach auf `accepts=0`. Die Seedanwendung änderte keine
Preise, Current-Zeiger oder Annahmezeilen. `user_version` wurde 6 auf 8
gesetzt; die 29 SQLite-Schemaobjekte blieben unverändert.

## Aktueller Seed-Diff auf dem migrierten Stand

| Wirkung Händlerseed | Anzahl |
|---|---:|
| Händlerzeilen vorher / nachher | 3.893 / 3.936 |
| Inserts / Deletes | 43 / 0 |
| Geänderte vorhandene Zeilen | 1.182 |
| Davon mit Änderungen außerhalb `notes`, `extra_json`, `updated_at` | 309 |
| Davon nur Änderungen an diesen drei Spalten | 873 |

Bei allen 1.182 geänderten Zeilen änderten sich `notes`, `extra_json` und
`updated_at`. Die neuen Notizen waren länger als die alten; bei 1.168 Zeilen
blieb der alte Notiztext vollständig als Teiltext erhalten, bei 14 nicht. Das
ist keine pauschale Bestätigung des Inhalts dieser Notizen. In `extra_json`
änderten sich `seed_hash` 1.182-mal, `seed_file` wurde 182-mal entfernt,
`ankauf_raw` änderte sich dreimal und `seed_section` zweimal.

Die weiteren geänderten Spalten, jeweils Anzahl betroffener vorhandener
Händlerzeilen:

| Feld | Zeilen | Richtung, falls relevant |
|---|---:|---|
| `name` | 25 | bestehender zu anderem Wert |
| `trader_type` | 9 | bestehender zu anderem Wert |
| `street` | 93 | 39 leer -> befüllt, 46 befüllt -> leer, 8 ersetzt |
| `postcode` | 73 | 44 leer -> befüllt, 29 befüllt -> leer |
| `city` | 18 | ersetzt |
| `lat` und `lon` | je 156 | 145 Koordinatenpaare entfernt, 11 Paare ersetzt |
| `phone` | 61 | 33 befüllt, 22 geleert, 6 ersetzt |
| `email` | 39 | befüllt |
| `website` | 45 | 43 befüllt, 2 ersetzt |
| `opening_hours` | 29 | 27 befüllt, 1 geleert, 1 ersetzt |
| `status` | 86 | ersetzt |
| `description` | 70 | 63 befüllt, 1 geleert, 6 ersetzt |
| `dropoff_json` | 25 | 6 befüllt, 15 geleert, 4 ersetzt |
| `pickup_json` | 4 | befüllt |
| `website_status` | 54 | ersetzt |

### GEO-Abgleich

Alle 145 entfernten alten Koordinatenpaare entsprechen einer Quelle mit
`clear_fields: geo`; die Menge der Quell-Direktiven und der entfernten
Produktionspaare stimmte exakt überein. Davon waren 127 Händler im Status
`pruefung`, 18 im Status `aktiv`. Das Löschen eines nicht belegten Kartenpunkts
behauptet weder Schließung noch fehlende Annahme.

Die elf geänderten, nichtleeren Punkte sind bereits mit GEO-Begründung und
Quellenverweisen in den jeweiligen Dossiers dokumentiert. Dieser Check hat
diese Dossierbelege gelesen, die externen URLs aber nicht erneut abgerufen.

| Dossier | Altpunkt -> Seedpunkt |
|---|---|
| [Albert Gogel, Backnang](../dossiers/bw/bw-backnang-albert-gogel-schrott-gogel.md) | 48.9474, 9.4372 -> 48.9477513, 9.4248526 |
| [MS-Brenner, Karlsruhe](../dossiers/bw/bw-karlsruhe-ms-brenner.md) | 49.0061143, 8.396978 -> 49.0643497, 8.3702234 |
| [Lusch, Wuppertal](../dossiers/nw/nw-wuppertal-lusch-heinz.md) | 51.2595130, 7.1649087 -> 51.2960878, 7.2474527 |
| [Tamm, Wuppertal-Ronsdorf](../dossiers/nw/nw-wuppertal-ronsdorf-tamm.md) | 51.2595130, 7.1649087 -> 51.2247867, 7.2141755 |
| [TSR Chemnitz](../dossiers/sn/sn-chemnitz-tsr-deutschland-nl-chemnitz.md) | 50.8207333, 12.9104667 -> 50.860415, 12.941881 |
| [Scholz Marienberg](../dossiers/sn/sn-marienberg-scholz-recycling-betrieb-marienberg.md) | 50.6132, 13.1883 -> 50.64031, 13.15493 |
| [TSR Ströbeck](../dossiers/st/st-strobeck-tsr-recycling-standort-strobeck-halberst.md) | 51.9141, 10.9444 -> 51.9051449, 10.9728386 |
| [Scholz Großengottern](../dossiers/th/th-groengottern-scholz-recycling-standort-groengottern.md) | 51.1482, 10.5623 -> 51.1426713, 10.5574896 |
| [Altfeld, Gevelsberg](../dossiers/nw/nw-gevelsberg-altfeld.md) | 51.3197, 7.3392 -> 51.3262186, 7.3502388 |
| [Fritsch, Hagen](../dossiers/nw/nw-hagen-fritsch.md) | 51.90933, 7.88825 -> 51.34597, 7.5837325 |
| [Indyka, Adorf](../dossiers/sn/sn-adorf-08626-indyka-thomas-av.md) | 50.3201, 12.2599 -> 50.3173097, 12.2605506 |

## Materialien, Integrität und Grenzen

Alle 83 Materialzeilen blieben inhaltlich gleich; der Materialseed änderte nur
`updated_at` aller 83 Zeilen. Der wiederholte Händlerseed schrieb null Zeilen.
`PRAGMA integrity_check` ergab für alle drei privaten Stände `ok`; der
FTS5-Integritätscheck des finalen privaten Standes war ebenfalls erfolgreich.
Die Current-Price-Zeiger stimmten nach Migration und Seed vollständig mit ihren
Preiszeilen überein.

Die 24 bereits im Ausgangsbestand vorhandenen FK-Verletzungen in
`trader_materials` (Rowids 1070-1093, fehlender Händler 15757) blieben erhalten;
es kamen keine hinzu. Das ist keine Reparaturfreigabe.

Diese Probe misst den aktuellen Diff, disponiert ihn aber nicht fachlich
vollständig. Insbesondere sind die 309 Zeilen mit strukturellen Änderungen,
die 14 nicht als Teiltext erhaltenen Notizen, alle Katalogdefinitionen und die
Bindung an exakt geprüfte Release-Binaries nicht durch eine einzige
Diff-Zählung freigegeben. Die ältere Prüfung vom 16.10. hatte einen anderen
Produktions- und Seedstand und ihre Zahlen sind nicht auf diese Probe
übertragbar. Kein Deployment und keine Produktionsänderung sind freigegeben.

## Owner-Nachprüfung: Notiz- und Händlerzählung, 26.10.2026 08:30 UTC

- Die 14 zuvor als „nicht zusammenhängender Teiltext“ markierten Notizfelder
  wurden erneut verglichen: Produktions-`traders.notes` ausschließlich
  `mode=ro` gegen die aktuellen generierten Seed-JSONs. Für alle 14 lieferte
  `difflib.SequenceMatcher(a=old, b=new, autojunk=False)` ausschließlich
  `equal`- und `insert`-Operationen, keine `delete`-/`replace`-Operationen.
  Somit bleibt jeder alte Notiztext exakt und in derselben Reihenfolge als
  Teilfolge erhalten; neuere Timeline-Einträge sind lediglich dazwischen
  eingefügt. Das belegt technische Erhaltung, nicht die fachliche Richtigkeit
  der alten oder neuen Aussagen. Betroffene Slugs: `be-treptow-kopenick-also-recommerce-ag`,
  `he-heuchelheim-blum-harald-schrotthandel`,
  `rp-trier-pfalzel-sascha-kaiser-schrott-metallhandel`,
  `sl-lebach-autoverwertung-volz-renate-volz`,
  `st-halle-schrott24-metaloop-online-ankauf-partner`,
  `st-schonebeck-fuhlrott-g`, `st-arneburg-bohlke-andreas-schrottverwertung`,
  `st-groningen-manfred-leiner`, `bw-ehningen-kurt-rie-nachfolger`,
  `he-korbach-34497-heidel-eckhardt`,
  `st-radegast-eks-ankauf-von-katalysatoren-und-batteri`,
  `st-halle-seik-automobil-recycling`, `st-badingen-bismark-hinze`,
  `st-aschersleben-lorenz`.
- Die aktuellen generierten Seeds enthalten **3.935 eindeutige Slugs**:
  **3.892** stimmen mit Produktionsslugs überein, **43** Seed-Slugs sind neu.
  Ein Produktionsslug ohne Seed-Dossier bleibt erhalten:
  `st-sangerhausen-mainmetall-grohandelsges`. Mit diesem Bestandsrest ergibt
  sich die im Übergang erwartete Anzahl **3.936** nach dem Seed. Der bekannte
  Sonderfall ist bereits in der [Prüfung vom 16.10.](checkin-20261016-0300-upgrade-seed-review.md)
  festgehalten; kein zusätzlicher Händler oder Delete wird daraus abgeleitet.
- Diese Nachprüfung schließt die technische Notiz-Erhaltungsfrage und erklärt
  die Händlerzählung, **nicht** aber die fachliche Disposition der **309**
  strukturell geänderten Bestandszeilen. Produktionsdaten wurden nicht
  verändert; die Pre-Deploy-Freigabe bleibt offen.

## Owner-Nachprüfung: frische Übergangsprobe, 26.10.2026 09:00 UTC

- Erneute konsistente SQLite-Sicherung von `/var/lib/schrott-mcp/public.db`,
  Quelle ausschließlich `mode=ro`, User-Version 6. Zwei private Kopien wurden
  verwendet: die erste als Baseline, die zweite mit aktuellem `PublicDb::open`
  auf v8 migriert und mit dem Seed von HEAD `c01504f0` befüllt. Der aktuelle
  Seed schrieb **1.243 Zeilen** in der privaten Kopie; Produktion wurde nicht
  beschrieben. Der vorherige Übergangstest beruhte auf einem älteren Snapshot
  und Seedstand, daher gelten dessen 309 Strukturzeilen nicht als aktueller
  Zähler.
- Händlerzeilen **3.893 → 3.936** (43 Inserts, 0 Deletes). **1.200** vorhandene
  Zeilen änderten sich; `notes`, `extra_json` und `updated_at` änderten sich bei
  allen. Bei **313** dieser Zeilen änderten sich zusätzlich strukturelle
  Händlerfelder; **887** änderten sich nur in den drei Seed-Metadatenfeldern.
  Die 313 Strukturzeilen sind noch nicht vollständig fachlich disponiert.
- Aktuelle strukturelle Feldzählung betroffener Zeilen: `name` 25,
  `trader_type` 9, `street` 93, `postcode` 73, `city` 18, `lat`/`lon` je 162,
  `phone` 61, `email` 38, `website` 46, `opening_hours` 30, `status` 86,
  `description` 70, `dropoff_json` 26, `pickup_json` 4 und `website_status` 56.
  Alle **151** `clear_fields: geo`-Direktiven entsprechen genau 151 in der
  privaten Anwendung entfernten Koordinatenpaaren; weitere **11** Paare wurden
  ersetzt. Die zuvor auffälligen 14 Notizen behalten ihren alten Text weiterhin
  vollständig und geordnet als Teilfolge.
- Tabellen vorher/nachher: Materialien **83/83**, Preise **29.832/29.832**,
  `current_prices` **2.683/2.659**, `trader_materials` **1.401/1.401**. Der
  Übergang entfernt 24 Current-Preiszeiger, sämtlich für den Heraeus-Goldstore
  Hanau; fünf vorhandene Annahmezeilen desselben Händlers stehen danach auf
  `accepts=0`. `integrity_check` und FTS5-Integritätsprüfung sind `ok`,
  Current-Preiszeiger-Mismatches bleiben null; die 24 schon vorher vorhandenen
  FK-Verletzungen bleiben unverändert. Dies ist eine private
  Quellcode-/Datenprobe, keine Release-Binary-/Serviceprobe und keine
  fachliche Freigabe. Keine Produktionsänderung oder Deploymentfreigabe.

## Owner-Nachprüfung: aktueller Seed und Acht-Fälle-Audit, 26.10.2026 09:18 UTC

- **Diese Probe ersetzt die Zähler im 09:00-Abschnitt für den nun geprüften
  Arbeitsbaum.** Eine neue konsistente Sicherung von Produktions-`public.db`
  wurde ausschließlich lesend von Schema v6 erstellt und privat mit aktuellem
  `PublicDb::open` nach v8 migriert. Der aktuelle Dossierseed schrieb 1.243
  Zeilen; eine zweite Anwendung schrieb null. Produktion blieb unangetastet.
- Händlerzeilen **3.893 → 3.936** (43 Inserts, 0 Deletes), **1.200** geänderte
  Bestandszeilen: 887 nur in `notes`/`extra_json`/`updated_at`, **313** mit
  zusätzlichen strukturellen Händlerfeldänderungen. Von diesen 313 Zeilen sind
  acht der nachfolgend verlinkten Hochrisikofälle im Owner-Gate geprüft; **305**
  strukturelle Zeilen bleiben für die vollständige Pre-Deploy-Disposition offen.
  Die 14 zuvor auffälligen alten Notiztexte bleiben alle vollständig und
  geordnet als Teilfolge erhalten.
- Aktuelle Feldzählung betroffener Bestandszeilen: `name` 25, `trader_type` 9,
  `street` 94, `postcode` 74, `city` 18, `lat`/`lon` je 164, `phone` 63,
  `email` 38, `website` 46, `opening_hours` 30, `status` 86, `description` 70,
  `dropoff_json` 28, `pickup_json` 4 und `website_status` 56. Alle **153**
  dossierseitigen `clear_fields: geo`-Direktiven entsprechen exakt den 153 in
  der privaten Seedanwendung entfernten Koordinatenpaaren; weitere **11** Paare
  wurden ersetzt. Kein Rust-Geocoding und kein Ersatzpunkt.
- Der achtteilige Owner-Gegencheck: [Benzin Paul / Templin](../dossiers/bb/bb-templin-benzin-paul-schrotthandel.md)
  hat konkurrierende Verzeichnisleads, keinen Betreiberbeleg und einen Altpunkt
  außerhalb Templins; die read-only Produktion enthielt zusätzlich `phone=n.e.`
  und `dropoff_json={"allowed":true}`. GEO, Platzhaltertelefon und nicht
  bestätigte öffentliche Annahme werden nun ausdrücklich zurückgenommen.
  [Weiss / Jüterbog](../dossiers/bb/bb-juterbog-weiss-rolf-schrotthandel-u-transporte.md)
  behält weder Verzeichnisadresse, Telefonnummer, alten Pin noch unbestätigte
  Annahme als aktuelle Fakten. Bei [Schmidt / Kiel](../dossiers/sh/sh-kiel-schmidt-boris-ne-metallhandel-und-schrot.md)
  belegt Ehrichs eigene Chronik eine Übernahme 2020, aber die unabhängige
  Zuordnungskette zum alten Rechtsträger fehlt; keine Verschmelzung oder
  Übertragung der heutigen Ehrich-Annahme. [Dorozala](../dossiers/nw/nw-lippstadt-dorozala.md)
  bleibt mangels zweiten Löschungsbelegs auf `pruefung`; aktuelle Adresse und
  Telefon sind geleert, ohne Nachfolgerannahme. Bei [Pfleging](../dossiers/nw/nw-rheda-wiedenbruck-pfleging.md)
  bleiben gelöschte GmbH, unbekannter heutiger Betriebsort und `pruefung`
  getrennt; nicht aktuelle Kontaktfelder und der fehlzugeordnete Punkt werden
  aus dem strukturierten Seed entfernt. [Reinhardt / Ulm](../dossiers/bw/bw-ulm-eselsberg-reinhardt-schrotthandel.md)
  hat einen auf der B 10 liegenden Punkt und eine unbestätigte Annahmezusage;
  beide erhalten jetzt explizite Clears. [ACS Jäger](../dossiers/mv/mv-ribnitz-damgarten-acs-jager-schrotthandel-abbruch-containe.md)
  und [Reinwarth BDE](../dossiers/sn/sn-frankenberg-rainer-reinwarth-bde.md)
  bleiben `pruefung`; ihre fehlzugeordneten GEO-/Annahmeclaims werden
  zurückgenommen, nicht in Ablehnung oder Schließung umgedeutet. Alte
  Recherchehistorie blieb erhalten.
- Tabellen vorher/nachher: Materialien **83/83**, Preise **29.865/29.865**,
  `current_prices` **2.683/2.659**, `trader_materials` **1.401/1.401**. Die 24
  entfernten Current-Zeiger und fünf geänderten Annahmezeilen betreffen weiter
  Heraeus Hanau. SQLite- und FTS5-Integritätschecks sind `ok`; Current-Price-
  Zeigermismatches **0/0**, die 24 vorher bestehenden FK-Verletzungen bleiben
  unverändert. Dies ist weiterhin nur eine private Daten-/Quellprobe, keine
  Release-Binary-/Serviceprobe und keine fachliche Freigabe der übrigen 305
  Zeilen. Kein Deployment und keine Produktionsänderung freigegeben.

## Owner-Nachprüfung: Fritsch Hagen, 26.10.2026 09:36 UTC

- Eine frische Read-only-Abfrage bestätigt, dass Produktion für
  `nw-hagen-fritsch` noch den Punkt **51.90933/7.88825**, leere Straße/PLZ und
  `status=pruefung` enthält. Der aktuelle Seed enthält dagegen **Ernst Jürgen
  Fritsch GmbH**, Oeger Str. 63, 58119 Hagen, **51.3459700/7.5837325** und
  `status=aktiv`. Vergleich der privaten Seed-JSON mit der Read-only-Zeile
  ergibt 12 strukturell geänderte Felder: Name, Beschreibung, Straße, PLZ,
  beide Koordinaten, Telefon, E-Mail, Öffnungszeiten, Website,
  Website-Status und Händlerstatus.
- Die live gelesenen Betreiberseiten bestätigen die einzelnen Änderungen:
  [Kontakt](https://www.fritsch-hagen.de/index.php?pa=kontakt) nennt die
  Gesellschaft, das ausdrücklich als Betriebsgelände/Büro bezeichnete
  Grundstück Oeger Str. 63, Öffnungszeiten und Besuch; die
  [Schrotthandelsseite](https://www.fritsch-hagen.de/index.php?pa=schrotthandel)
  bestätigt Schrottankauf und Anlieferung mit geeichter Fahrzeugwaage; das
  [Impressum](https://www.fritsch-hagen.de/index.php?pa=impressum) nennt HRB
  7553, Gesellschaft, Adresse und Kontaktdaten. Der direkt gelesene
  [NorthData-Eintrag](https://www.northdata.de/Ernst+J%C3%BCrgen+Fritsch+GmbH,+Hagen/Amtsgericht+Hagen+HRB+7553)
  stimmt bei Rechtsträger, Register und Anschrift überein.
- Das [OpenStreetMap-Gebäudeobjekt](https://www.openstreetmap.org/way/305239745)
  ist als Oeger Straße 63, 58119 Hagen adressiert. Der aktuelle Nominatim-Punkt
  **51.3459700/7.5837325** löst auf genau dieses Gebäude auf; der alte Punkt
  wird auf Mehringen in Everswinkel aufgelöst. Das Ersatzpaar ist ein
  Gebäudeadresspunkt, kein vermessener Eingang. Geocoder und OSM sind
  Standortzuordnung, keine unabhängigen Betreiberbelege.
- Owner-Disposition: Die Seed-Änderungen für diese eine strukturelle Zeile
  tragen einschließlich des Status `aktiv`; das ist kein Produktionsrollout.
  Damit verbleiben **304** der zuvor offenen 305 strukturellen Zeilen. Die
  Produktionszeile bleibt unverändert und falsch; es gab weder direkten
  Datenbankzugriff mit Schreibrechten noch Deploy.

## Owner-Nachprüfung: fünf weitere GEO-Rücknahmen, 26.10.2026 09:40 UTC

- Fünf weitere strukturelle Übergangszeilen wurden anhand der Dossier-Diffs,
  der neuesten privaten Seed-JSON und read-only Produktionswerten gegengeprüft;
  die fünf Timelines wurden einzeln owner-reviewed. Die **GEO-Clears tragen**:
  SchrottJungs München verweist für die belegte Anlieferung ausdrücklich auf
  den getrennten Partnerhof Entel, nicht auf eine eigene Anschrift;
  AK Containerdienst belegt Aschaffenburg als Liefergebiet, den Metallplatz
  aber nur in Rödermark; ISR ist mit Annahme an der Wilslebener Straße 6
  belegt; die Schrott- und Metallhandelsgesellschaft ist ein separater
  Rechtsträger am Majoranweg 4. In Aschersleben werden die beiden Einträge
  trotz eines geteilten falschen Alt-Pins nicht zusammengeführt. Für keinen
  der fünf Fälle wurde ein Ersatzpunkt geraten.
- Beim Malstatter Thomanek-Datensatz stand in Produktion zusätzlich zum falschen
  GEO noch `dropoff_json={"allowed":true}`. Verifiziert sind nur widersprüchliche
  Aggregator-Leads, kein aktueller Betreiber-/Annahmebeleg. Der Owner ergänzte
  deshalb `clear_fields: 'dropoff_json,geo'`; der nächste reguläre Seed soll
  `dropoff_json` auf unbekannt `{}` setzen und den Punkt löschen, ohne
  `allowed:false` oder Schließung abzuleiten. Die vorangegangene
  Timeline-Unsicherheit bleibt als Recherchehistorie erhalten.
- Für AK und die Ascherslebener GmbH war kein amtlicher aktueller
  Registerauszug zugänglich; Registerangaben beruhen auf Betreiber-Impressum
  bzw. Registerspiegel. Die städtische Suche lieferte für ISR keinen
  unabhängigen kommunalen Betreiberbeleg; der alte SMS-Partnerlisteneintrag
  ist kein Nachweis einer zweiten heutigen Gesellschaft oder einer Verschmelzung.
  Alle fünf GEO-Rücknahmen bleiben Clears ohne Ersatzkoordinaten.
- Vorbehaltlich der noch ausstehenden vollständigen privaten Re-Probe sind damit
  **299** der 313 strukturellen Zeilen weiterhin offen (304 nach Fritsch minus
  diese fünf Owner-disponierten Zeilen). Diese Zählung wird nach aktuellem
  Seed-/Upgrade-Abgleich erneut validiert. Keine Produktionsdaten geändert,
  kein Deployment.

## Owner-Nachprüfung: Lorenz Aschersleben, 26.10.2026 09:43 UTC

- Der vom Aschersleben-Audit benachbarte Datensatz `st-aschersleben-lorenz`
  wurde separat geprüft: Produktion enthält im Status `pruefung` weiter nur den
  alten Punkt **51.7574/11.4608**; die aktuelle Rückwärtssuche löst ihn auf das
  als Wohngebäude (`apartments`) erfasste OSM-Objekt **13b Herrenbreite**.
  Dossier, Betreiberidentität und Betriebsadresse bleiben ungeklärt; die
  dokumentierten Westdorf-/Telefonbuch-Leads belegen keine Verbindung zum
  Punkt. Der vorhandene GEO-Clear trägt, ohne Ersatzpunkt und ohne eine
  Zusammenführung mit ISR oder der separaten Schrott- und Metallhandelsgesellschaft.
- Damit sind **298** der 313 strukturellen Zeilen weiterhin offen; der Zähler
  ist vorläufig und wird nach dem aktuellen privaten Seed-/Upgrade-Abgleich
  bestätigt. Keine Produktionsdaten geändert, kein Deployment.

## Owner-Nachprüfung: vier NRW/RP-Dossiers und frische Übergangsprobe, 26.10.2026 09:56 UTC

- Die vier zuletzt bearbeiteten Dossiers wurden owner-reviewed und mit
  Read-only-Produktionswerten abgeglichen. **MS Metallhandel Hagen:** die
  Betreiberseite belegt B2B-Ankauf, eigene Abholung und eine öffentliche
  Fahrzeugwaage, aber keine Kunden-Selbstanlieferung; der gespeicherte
  `dropoff_json={"allowed":true,"customer_types":["gewerbe"],...}` wird
  deshalb zusätzlich zum falschen GEO mit `clear_fields` neutral auf `{}`
  zurückgenommen. **Putzke Hagen:** Impressum und Metallankaufsseite der
  Betreiberwebsite bestätigen FP Putzke Recycling GmbH, die Hagen-Adresse und
  ausdrückliche Privat-/Gewerbeanlieferung; der Seedname ist auf den
  Betreibernamen präzisiert, aber kein Ersatz-GEO aus der Straßenadresse
  geraten. **Schrotthandel NRW/Klüngelskerle:** Root, Impressum, Kontakt und
  Leistungsseite sind wieder direkt erreichbar; `website_status` wird daher
  `aktiv`, während die unklare Betreiberidentität `status: pruefung` behält.
  Der mobile Auftritt verweist auf Partnerhöfe und belegt keinen eigenen
  Annahmestandort; das Produktions-`{"allowed":true}` wird gemeinsam mit dem
  falschen GEO neutral gelöscht. **Südpfalz Metall Rohrbach:** die Website nennt
  im Impressum Janneck Knecht und HRB 33727, ohne den Rechtsträger zu nennen;
  der geprüfte Registerstand ordnet die Nummer der MBK Mietservice GmbH mit
  abweichendem Unternehmensgegenstand zu. Daher `status: pruefung`, kein
  rechtlicher Betreiber-Fill; vorhandene GEO-/Annahme-Clears bleiben. Es wurde
  weder eine Schließung noch `allowed:false` abgeleitet.
- Der aktuelle private Übergangsabgleich wurde nach diesen Änderungen frisch
  auf zwei SQLite-Backupkopien aus einer ausschließlich lesend geöffneten
  Produktionsquelle ausgeführt: Schema **v6 → v8**, Händler **3.893 → 3.936**,
  **43 Inserts / 0 Deletes**, **313** vorhandene strukturell geänderte Zeilen.
  Neue Feldzählung: `name` 26, `trader_type` 9, `street` 94, `postcode` 74,
  `city` 18, `lat`/`lon` je 164, `phone` 63, `email` 38, `website` 46,
  `opening_hours` 30, `status` 87, `description` 70, `dropoff_json` 31,
  `pickup_json` 4 und `website_status` 57. Von den 313 strukturellen Zeilen
  sind jetzt 19 einzeln owner-disponiert; **294** bleiben für das vollständige
  fachliche Pre-Deploy-Gate offen. Alle **153** Geo-Clears stimmen mit
  entfernten Koordinatenpaaren überein; weitere **11** Paare wurden ersetzt.
  Der zweite Händlerseed schrieb **0** Zeilen.
- Die private Probe ergab `integrity_check=ok` vor und nach Migration/Seed,
  FTS5-Integritätsprüfung erfolgreich und **24** bereits vorhandene
  `trader_materials`-FK-Verletzungen vor wie nach der Probe. Materialien
  **83/83**, Preise **29.880/29.880**, aktuelle Preise **2.683/2.659** und
  `trader_materials` **1.401/1.401**. Der bekannte Verlust von 24
  Current-Preiszeigern betrifft die fünf Heraeus-Annahmen und autorisiert keine
  Reparatur. Backupkopien und temporärer Probehelper wurden gelöscht; die
  Produktionsdatenbank blieb unverändert.
- Keine Deploymentfreigabe: die 294 strukturellen Zeilen sowie der vollständige
  fachliche Source-/Seed-Audit und die Bindung an exakt geprüfte
  Release-Binaries bleiben offen. Der erneut read-only ausgeführte
  `verify-mkm-release.py` bleibt **post-rollout-rot** (60 Artikel, 55 numerisch,
  5 Anfragen, 55 current; Partition 55/5 statt 56/5, Manifest nicht aus dem
  neuesten Step-Fenster, geprüfte 61-Artikel-Partition weicht ab, kein
  erfolgreicher 56-Preis-Lauf nach Rollout). Das ist weder Pre-Deploy-Gate noch
  Deployment-Autorisierung. Kein Deployment, Force-Ingest, Produktionswrite
  oder Dienstneustart.

## Owner-Nachprüfung: Balzer Alfeld GEO-Rücknahme, 26.10.2026 19:32 UTC

- Read-only Produktion enthält für `ni-alfeld-balzer-metallverwertung` weiter
  `49.4333/11.55`; Dossier und Produktionszeile stimmen bei Name, Ort, Staat,
  Straße, PLZ und Status überein. Eine aktuelle Nominatim-Rückwärtssuche löst
  den Punkt an der Hersbrucker Straße in Pollanden, Alfeld, Landkreis
  Nürnberger Land, Bayern, 91236 auf. Das ist nicht Alfeld (Leine),
  Niedersachsen, 31061.
- Das [Dossier](../dossiers/ni/ni-alfeld-balzer-metallverwertung.md) markiert
  `clear_fields: 'geo'` und setzt bewusst keinen Ersatz:
  weder die mögliche Registergesellschaft noch eine öffentliche Annahmestelle
  oder konkrete Zufahrt ist als Betreiberstandort ausreichend verifiziert.
  Die Seed-Implementierung setzt bei diesem expliziten Clear beide Koordinaten
  auf `NULL` (`seed_traders.rs`, `coordinates`-Verarbeitung); gewöhnliche leere
  Koordinaten würden einen bestehenden Punkt dagegen erhalten. Seed-Tests
  liefen nach dieser Prüfung erneut.
- Owner-Disposition: Die Geo-Rücknahme ist fachlich bestätigt; der falsche
  Pin darf nicht erhalten bleiben und es wird kein Ersatzpunkt geraten. Dies
  ist keine Freigabe für Produktionsänderung oder Rollout. Der aggregierte
  Zähler von 294 offenen strukturellen Zeilen wird bis zur erneuten vollständigen
  privaten Übergangsprobe nicht neu berechnet; die Gesamtfreigabe bleibt offen.
