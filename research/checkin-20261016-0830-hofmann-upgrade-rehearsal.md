# Private vollständige Upgradeprobe: Hofmann PASS, Release HOLD

## Bindung und Grenzen

Basis: sauberer Archivexport von `53a2492724db44d8354015aca2b3bf0cab6a81f1`
(4418 Dateien), keine offenen Fremddossiers/Reviewer-Extraquellen.
Während der Probe kam Owner-Commit `9c140e19e92d404aac2c384da6459ce387f79cbe`
hinzu: ausschließlich zwei Researchberichte; Crates, Cargo und Dossiers sind
zwischen beiden Commits identisch. Alle 144 vorgefundenen Fremddateien blieben
hashidentisch; einzige eigene Main-Änderung ist dieser angeforderte Bericht.

Private Evidenzwurzel: `/root/Documents/hofmann-v3-private-upgrade-20261016-0830/`.
Originale `public.db` und `internal.db` sowie `backup-manifest.json` unverändert:

| Original | SHA256 |
|---|---|
| public.db | `3b27bb912cba5dcda0d6bd1becfdb4a379ce35428c282146c5dd3183adf9d24a` |
| internal.db | `37d7f47b13d456b5cd7bb92120815c25f008695a7dba8d8c99e715f245e7ea02` |

Nur private Arbeitskopien geöffnet. Rust-Harness prüft kanonische exakte
Arbeitsverzeichnisse und DB-Pfade, verbietet Symlinks/Hardlinks und benötigt
expliziten Rehearsal-Marker. Drei isolierte Instrumentierungsdateien ausschließlich
per Patch geändert; `instrumentation.patch` ist **kein Releasepatch**.
Bestehendes Target wiederverwendet, Cargo offline/locked, Jobs=2,
TMPDIR=/tmp/opencode; Daten/Export unter /root/Documents.

## Open/Seed: vollständige Wirkung, nicht nur 60er-Auszug

Echte `PublicDb::open`/`InternalDb::open`, danach echter `seed_metadata`-Pfad
einschließlich `seed_traders`, wiederholt und separat vollständig verglichen:

* Open: Ledger leer, user_version weiter 6. Sämtliche vorhandenen Tabellenzeilen
  identisch, insbesondere **20092 Preise / 2426 Current** und gesamte Internal-DB.
* **HOLD – erweiterte DDL:** Zusätzlich zum Hofmann-Ledger entstehen die bisher
  fehlenden Tabellen `source_article_snapshots` und `article_acceptance_owners`
  samt ihren Unique-Indizes. Alle drei neuen Tabellen leer; keine Schemaentfernung.
  Die engere Annahme „nur Hofmann-Tabelle“ ist auf dieser Kopie falsifiziert.
* Vollständiger Korpus **3892**: **715 bestehende Händler geändert, 12 eingefügt**,
  keine gelöscht. Ein nicht mehr im Seed enthaltener historischer Händler
  `st-sangerhausen-mainmetall-grohandelsges` bleibt vollständig identisch.
* **HOLD – gesamte Seedwirkung benötigt Owner-Disposition:** Alle 715 Updates
  ersetzen notes/extra_json/updated_at; unter anderem 83 Statusänderungen und
  Entfernung des historischen `seed_file`-Schlüssels bei **153** Händlern.
  Nichtleere Werte werden bei **28 Straßen / 14 PLZ** leer. Bei Schrott24 setzen
  explizite Seedwerte `{}` beide vorhandenen Servicebeschreibungen zurück
  (Set, nicht Clear). Vollständige Einzelwerte stehen nur in privater Evidenz.
* Letzte Keep/Clear-Entscheidungen über **35028 Feldfälle** geprüft: 0 Abweichungen
  vom tatsächlichen Vertrag einschließlich Hash-Skip. 43 Clear-Direktiven,
  davon 30 tatsächlich nichtleere Feldwerte auf 24 Händlern geleert: 24 Telefon,
  2 E-Mail, 2 Öffnungszeiten, 1 Beschreibung, 1 Website. 3165 Händler werden schon
  beim ersten Seed hashbedingt übersprungen; dadurch bleiben auch 69 abweichende
  Set-Kandidaten als gespeicherte Anreicherungen erhalten, nicht neu angewandt.
* Trader-Idempotenz bewiesen: direkter Nachlauf und zweiter Gesamtboot schreiben
  **0 Trader**, alle Traderzeilen/IDs/first_seen bleiben stabil. Der **Gesamtboot
  ist nicht physisch idempotent**: alle 83 Materialzeitstempel und der Material-
  Sequenzzähler ändern sich erneut (+83). Erster Boot: 29 neue Materialien,
  vorhandene 54 nur mit Zeitstempeländerung. Preise/Current/Internal bleiben
  durch beide Seedläufe unverändert; FTS-/Sequenzwirkungen vollständig erfasst.

## Qualifizierter Übergang auf separater privater Kopie: PASS

Zusätzlich zur unabhängigen Direktprobe wurde der **vollständig geseedete Bootstand**
über SQLite-Backup-API in eine getrennte private Transitionkopie übernommen;
alle Tabellen/Schema vor dem Scrape identisch. Frischer echter HTTP-200-Scrape
über `scrape_record_outcome`, anschließend echter typisierter `record_outcome`:

* 23 Preise übernommen, keine Skips/Canaries. Das vollständige enge Legacyprädikat
  und die Zeitbedingung passen. **Nur Key (137,14,'') mit Current-ID13725 entfernt**;
  genau Key (137,14,'alt') hinzugekommen, neue Preis-ID **20457**.
* Historienzeile **13725 in allen Spalten identisch**, kanonischer Zeilenhash
  `363f4676fedff62722878d66aa5c502bf7cd8260da2b03fc69f84b7751161e85`.
  Alt: **16.5 EUR/kg**, exact/confidence1, row_stated,
  published_at `2026-10-14T00:00:00+00:00`.
* Ledger persistent: retired_price_id13725, decided_at
  `2026-10-16T08:45:44.163459291+00:00`. Alle 23 gespeicherten Row-Dates stimmen
  mit dem Liveparser überein. Keine Preiszeile gelöscht, 20 neue Preiszeilen;
  reguläres Same-ID-Dedupe aktualisiert bei 13722/13729/13733 ausschließlich
  observed_at/ingested_at/extra_json. Keine anderen Händlerpreise/-Pointer verändert;
  alle Hofmann-Nachbarkeys erhalten. Internal, Trader und Materialien unverändert.
* Gleichzeitiger Retry, neuer Prozess/Reopen und typisiertes Late-Replay lassen
  sämtliche Tabellen/Schema identisch. Auch späterer Retry **+60 Sekunden**
  bewahrt IDs, Ledger und alte Historie; anschließende Reopen/Late-Probe identisch.
  Late-Replay ist ausdrücklich eine synthetische historische Zinn-HTML-Zeile
  durch echten Parser/Recorder zum 15.10., kein behaupteter historischer Livefetch.

## Zusätzlicher realer Befund: HOLD

`integrity_check=ok` bei allen Arbeitsdatenbanken, aber Public enthält **24
Foreign-Key-Verletzungen** in trader_materials (fehlender Händler15757).
Auf einer weiteren unveränderten privaten Baselinekopie identisch nachgewiesen:
bereits vorhanden, **0 neu durch Upgrade/Seed/Übergang**. Keine Reparatur vorgenommen.
SQL-Integrität darf deshalb nicht mit vollständiger Referenzintegrität gleichgesetzt
werden; Befund benötigt Owner-Disposition (`baseline-foreign-keys-HOLD.json`).

## Nachweise und Entscheidung

`rehearsal-manifest.json` SHA256:
`5b2956a7ceb0485a0f7ba44c7bb0845641bbd747a94d75bd7d359c754089d543`.
Es bindet Originale, Instrumentierung, sämtliche JSON/Logs/Snapshots per SHA256.
19 echte Harness-Aufrufe mit Exit0, Kommandos/UTC-Zeiten und jeweiligen drei
Quellhashes unter `logs/*.command.json`; Rustfmt-Check Exit0. Kein neuer kompletter
Workspace-Testlauf behauptet. Vollständige Tabellen-/Schema-Snapshots unter
`snapshots/`, vollständige PK-/Rowid- und Allspalten-Diffs in `diff-*.json`,
`seed-field-decisions.json`, `seed-historical-impact.json`, `boot-ddl-HOLD.json`,
`transition-proof.json`, `seeded-transition-proof.json`, `final-integrity.json`.
Frische Seeded-Live-HTML SHA256:
`5891ec5c3c9df1c8e28cc29d589894ad0473a86b3f39f5018fcf2a6ced09a55d`.
Frühe Instrumentierungsversuche erhalten; Korrekturen in `instrumentation-notes.json`.

**Kein Release-PASS:** Erweiterte DDL, vollständige Seed-/Historienwirkung und
vorbestehende FK-Schuld bleiben HOLD bis ausdrücklicher Owner-Disposition.
Kein Produktionspfad geöffnet, kein Commit/Push/Deploy/Service/Produktionsingest.
Keine Restorefreigabe: alter Binarytausch umgeht das Ledger und ist kein sicherer
Datenrollback; keine Ledgerlöschung/Downmigration oder Datenreparatur autorisiert.
