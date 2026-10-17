# Aktuelle private Upgrade-/Seeddrift: technisch bestanden, kein Deploymentnachweis

Gebunden an `ccd32e29d465795f52efff3f8a72c567c9089714`, ausschließlich sauberer
Gitarchivexport, **3892 committed Dossiers einschließlich der freigegebenen
60er-Welle**. Die fremden Dirtytree-Dossiers und der parallele Releasebuild sind
keine Probeinputs. Gelesen wurden die angeforderten fünf Owner-/Releaseberichte
und die vollständige private Vorprobe vom 16.10.
Beim Abschluss stand Main bereits auf `69ed0ba167de0c628ebf794a1d02c1d059740050`;
der lesende Vergleich zu `ccd32e29` ist für `crates/`, `dossiers/`, Cargo.toml und
Cargo.lock leer. Die Probe bleibt ausdrücklich an den angeforderten Commit gebunden.

**Ergebnis:** aktuelle Kopien integer im SQLite-Sinn; exakt die akzeptierten
24 FK-Altverletzungen, keine neue. Open/Seed ändern keine Preise, Current,
Annahmen oder Internal-Zeilen. Kontakt-Leer+Keep besteht. Der vollständige neue
Seed hat **751 Updates / 12 Inserts / 0 Deletes**. Die alte Freigabe von
715 Updates wird nicht fälschlich als aktueller Diff ausgegeben: die Welle
verursacht zusätzliche technische Effekte, unten ausdrücklich getrennt.
Keine eigenständige neue fachliche Freigabe und kein Release-/Deployment-PASS.

## Frische Sicherungen und Probegrenze

Private Evidenzwurzel, Verzeichniszugriff `0700`:
`/root/Documents/upgrade-drift-20261017-1630/`.

Am **17.10.2026, 16:34:07–16:34:08 UTC** wurden ausschließlich
`/var/lib/schrott-mcp/{public,internal}.db` über URI **`mode=ro` plus
`PRAGMA query_only=ON`** geöffnet und per SQLite-Backup-API auf eigene Dateien
gesichert. Keine bloße Kopie laufender DB-Dateien. Jede DB ist einzeln
WAL-konsistent; **keine atomare datenbankübergreifende Sicherung behauptet**.
Die Originalbackups blieben byteidentisch; separate Arbeitskopien `boot/` und
`baseline/` stammen jeweils aus genau diesen Originalen.

| Originalbackup | SHA256 |
|---|---|
| public.db | `edbe219c3202066271a0e24d75f66064b749fe9e9c4d260fd850ef91bc5329cd` |
| internal.db | `860909f3db4a998cf665f1bdfb6c8a84a808dd5aa0f4c66079c3e4526565f10c` |

Wiederverwendet wurde die vollständige Tabellen-/Schema-/PK-Diffmaschine der
Vorprobe. Deren ausführbarer Harness enthält jedoch alte Seeds; zudem fehlten
seine ursprünglichen `debug/deps`. Deshalb **kein Cargo-/Workspace-Neubuild**:
nur aktuelle committed Core-/Storequellen und ein kleiner synchroner Harness
direkt mit rustc in der eigenen Evidenzwurzel gebaut. Vorhandene Drittbibliotheken
und native Linkinputs aus dem Debugcache wurden ausschließlich gelesen; kein
gemeinsamer Cache/Source geändert, kein Releasecache verwendet.

Die aktuelle `seed_traders.rs` läuft **unverändert**; Materialkatalog,
`seed_metadata` und Timestampfunktion wurden wortgleich aus der aktuellen
`pipeline.rs` extrahiert. Lediglich der generische Fehlerträger ist reduziert.
Echte `PublicDb::open`/`InternalDb::open`, Store-Upserts und Hash-/Keep-/Clearlogik;
kein nachgebauter SQL-Seed. Pfadkanonisierung, Hardlinkprüfung und expliziter
Privatmarker begrenzen die writefähige Probe auf eigene DBs. Dies ist **kein
Smoke des endgültigen Server-/Workerartefakts**, kein Workspace-Testlauf und
keine neue Hofmann-Scrape-/Transitionprobe.

Der tatsächliche unveränderte Codegenerator wurde frisch auf dem Archivexport
und auf den alten akzeptierten Baselineinputs ausgeführt. Alle **16 aktuellen
Codegen-JSONs sind byteidentisch zum finalen Owner-Wellenexport**. Die 3892 Slugs
sind gleich; exakt 60 Payloadänderungen. Quell-/Bibliotheks-/Binaryhashes,
Kommandos, Exitcodes und Analysekorrekturen stehen in den privaten Belegen.

## Open und vollständiger Seed auf dem aktuellen Stand

* `integrity_check=ok` für Public und Internal vor/nach der Probe. Public hat
  weiterhin exakt `trader_materials`-Rowids **1070–1093**, jeweils fehlender
  Trader **15757**; alle 24 Zeilen auch gegenüber der alten Kopie identisch.
  Materialeltern vorhanden, keine Preise/Current für 15757, keine Wiederanlage.
  Internal ohne FK-Verletzungen. **Keine Behauptung vollständiger FK-Integrität.**
* Open erzeugt exakt die bereits akzeptierten drei leeren Tabellen
  `hofmann_tin_cutovers`, `source_article_snapshots`, `article_acceptance_owners`
  samt ihren drei Unique-Indizes. Alle bestehenden Tabellenzeilen identisch,
  kein Schemaeintrag entfernt, `user_version=6`. Alle drei bleiben durch Seed leer.
* Vollseed: Händler **3881→3893**, **751 bestehende Updates / 12 Inserts**;
  **172** Bestandszeilen mit Skalaränderungen, **579** nur Notes/Metadaten/Zeit.
  Bestehende IDs und `first_seen_at` bleiben erhalten, kein historischer Händler
  gelöscht. Unter anderem weiterhin exakt **83 Statusänderungen**, dieselben
  Adress-/Kontakt-/Service-Rücknahmen und neun kuratierte GEO-Paare; keine
  geografische Recherche, Geocodierung oder Rust-GEO-Ausführung.
* Materialkatalog **54→83**, dieselben 29 akzeptierten Definitionen. Alle
  bestehenden IDs/Definitionen bleiben bis auf `updated_at` identisch.
* **21147 Preise / 2426 Current / 1315 Annahmezeilen**, alle Spalten und Keys
  durch Open, ersten und zweiten Seed identisch. Gesamte Internal-DB einschließlich
  Schema und aller Tabellen ebenfalls identisch. Kein Seed-Preis-/Annahmebeleg
  aus neuen Materialdefinitionen abgeleitet.
* Direkter Tradernachlauf nach beiden Vollseeds jeweils **0 Writes**; zweiter
  Vollseed ändert keine Trader oder FTS-Zeilen. Wie bereits akzeptiert werden
  erneut alle **83 Materialzeitstempel** und die Materialsequenz um **83** erhöht.
  Gesamtseed daher weiterhin nicht physisch idempotent; kein neuer Blocker.

## Neue Wellenwirkung separat vom akzeptierten Basiseffekt

Kontrollkopie derselben frischen Sicherung mit altem akzeptiertem Seed gegen
aktuellen Seed: exakt 60 logische Traderdifferenzen nach expliziter Separierung
der Laufzeitstempel und neuer Insert-IDs. Rohdiff bleibt vollständig erhalten.
Der Kontrollstand stimmt in sämtlichen anderen Traderinhalten mit der alten
akzeptierten Probe überein; IDs/Zeiten werden dabei nicht als Inhaltsgleichheit
ausgegeben und bestehende IDs separat geprüft.

* **58 nur Notes**, dazu DKE nur Beschreibung+Notes und Edelmetallkontor
  Idar-Oberstein Name/Website/Website-Status/Beschreibung+Notes, exakt gemäß
  `wave-20261016-1300-owner.md`. Keine neuen Kontakte, Adressen, Statuswerte,
  Service-JSONs, Clears oder Koordinaten aus der Welle.
* 24 der 60 Händler waren bereits im alten Updatepaket; **36 weitere Updates**
  erhöhen den Gesamtstand von 715 auf **751**. Zwei zusätzliche Skalarfälle
  erklären **170→172**, reine Metadatenfälle **545→579**.
* Dadurch weitere **6 `seed_file`-Entfernungen**, insgesamt **153→159**:
  Ruft/Großlangheim, Rissel/Höchstadt, Kupferdantler/München,
  Sindlhauser/Roding, Menche/Marburg und Edelmetallkontor/Idar-Oberstein.
  Keine anderen zusätzlichen verlorenen Metadatenschlüssel. Dies sind neue
  Einzelwirkungen desselben bekannten Ersetzungsvertrags, nicht still durch die
  alte eng gebundene 153er-Disposition als schon einzeln akzeptiert ausgegeben.
* SQLite-Upserts verbrauchen auch bei Updates Autoincrementwerte: auf identischem
  Ausgangsstand endet Tradersequenz mit altem Seed bei **26585**, aktuell bei
  **26621** (**+36**). Bei **11 der 12 neuen Händler** unterscheiden sich deshalb
  die neu vergebenen IDs vom Gegenlauf; sämtliche bestehenden IDs unverändert.
  Exakte Slugs/IDs unter `wave-new-insert-id-effects.json`. FTS-Zeilen und
  Schattenindizes/Sequenzen sind im Rohdiff sichtbar, nicht wegbehauptet.

Diese zusätzlichen technischen Wirkungen sind dem Owneraudit ausdrücklich
vorgelegt; kein unerklärter zusätzlicher fachlicher Payloadeffekt gefunden.
Die akzeptierten drei DDL-Tabellen, 29 Katalogdefinitionen, wiederholten
Materialrefreshes und 24 FK-Altschulden werden **nicht erneut zu Blockern erklärt**.

## Kontaktgate und tatsächliche Produktionsdrift seit der Vorprobe

Alle **sieben gesperrten Felder** vor Open, danach und nach beiden Seeds leer:
Fernau `phone/email/opening_hours`, Graf/Kiel `phone/email`, **Willeführ/Aschaffenburg**
`phone/email` (nicht andere Willeführ-Dossiers). Aktuelle Seedwerte leer,
kein entsprechendes `clear_fields`, somit **Keep**, keine dauerhafte Löschungszusage.
Keine Drift in diesen Feldern; Einzelwerte/Stages unter `contact-audit.json`.

Die Produktion ist gegenüber dem Backup vom 16.10. tatsächlich weitergelaufen:
Preise **20092→21147** (+1055), 1804 vorhandene Preiszeilen geändert;
**498** Currentpointer anders, Anzahl weiterhin 2426. **977** Annahmezeilen
haben ausschließlich Beobachtungs-/Updatezeiten geändert; Anzahl weiterhin 1315.
**125** Trader unterscheiden sich ausschließlich in `website_checked_at/updated_at`,
alle 54 Materialien nur in Refreshzeit. Internal enthält unter anderem 128
zusätzliche Runs und 694 zusätzliche Steps/Fetches sowie weitere Authzustandsdrift.
Vollständige Einzelwerte bleiben privat. Diese Vorher-Drift wird **nicht dem
Upgrade/Seed zugerechnet**; durch die Probe selbst sind alle genannten Preis-,
Current-, Annahme- und Internalzeilen unverändert.

## Vollständige Belege für Owneraudit

Alle folgenden Pfade relativ zur privaten Evidenzwurzel:

* `backup-manifest.json`, `original/`, `integrity-before.json`,
  `integrity-after.json`: Backupzeiten/-hashes, exakte FK-Zeilen.
* `prepare.py`, `probe.rs`, `metadata.rs`, `source/`, `commands.jsonl`,
  `preparation.json`, `archive-input-audit.json`, `seed-path-source-binding.json`, `library-binding.json`,
  `probe-notes.json`, `logs/*.json`: ausführbare Probe, Inputs, Grenzen,
  frühere fehlgeschlagene Vorbereitungs-/Analyseannahmen transparent erhalten;
  **vier tatsächliche DB-Harnessaufrufe, alle Exit0**.
* `current/seed_traders/`, `baseline/seed_traders/`,
  `current-codegen-owner-equality.json`, `codegen-wave-diff.json`:
  vollständige 3892er-Payloads und Ownergleichheit.
* `snapshots/{before,opened,seeded,twice,baseline-seeded}.json.gz`,
  `diff-{open,seed1,seed2,production-drift,wave-control-raw}.json`:
  vollständige Schemas, alle Tabellenzeilen, PK-/Rowid-/Allspaltendiffs.
* `wave-control-normalized.json`, `wave-new-insert-id-effects.json`,
  `seed-impact-and-wave-increment.json`, `accepted-baseline-equivalence.json`,
  `trader-production-drift.json`, `contacts.json`, `contact-audit.json`,
  `summary.json`, `audit.json`: reproduzierbare Einzelprüfungen und Summen.
* `prove.py`, `supplement.py`, `audit.py`: Datenerhebung und Assertions;
  `manifest.json` bindet sämtliche stabilen privaten Dateien per SHA256
  (volatile SQLite-WAL/SHM-Dateien ausdrücklich ausgenommen).

Abschlussbindung: **4440 von 4440 Exportdateien** gegen die Git-Blob-IDs des
Zielcommits geprüft, alle exakt gleich; **4524 private Dateien** manifestiert.
`manifest.json` SHA256:
`4286d438e1decd86e1af73f7224f1b587874179e778c0fd524d3e84ba4df35f1`.

Keine Produktionsmutation, kein Service-/Deployment-/Restorevorgang, kein
Commit/Push. Writerexklusivität, endgültige Releaseartefakte, Installation,
regulärer Scheduler/Public-Read einschließlich MKM und späterer Hofmannübergang
werden hier **nicht** als neu nachgewiesen behauptet. Sicherungen/Drift sind an
die angegebenen Zeiten gebunden, keine Zusage eines später unveränderten Zustands.
