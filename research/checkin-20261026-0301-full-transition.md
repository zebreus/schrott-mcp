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
