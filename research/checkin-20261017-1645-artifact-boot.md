# Finales Serverartefakt: echter privater Kurzboot bestanden

**PASS für normalen Boot, Health und vollständigen Seedvergleich des finalen
Artefakts**, nicht nur des reduzierten Fehler-Harness. Kein Schedulerlauf-,
Transition- oder Deployment-PASS. Keine Produktion geöffnet oder verändert,
kein Service geändert, keine forced ingestion, kein Commit/Push.

## Artefakt, frische Arbeitskopien und Isolation

Unverändert direkt gestartet:
`/root/Documents/checkin-20261017-1630-release-a_8a8jlo/artifacts/schrott-mcp-server`

SHA256 vor Start, über `/proc/<eigener PID>/exe` und nach Ende bestätigt:
`7de235b153c571d2ed346dbb39fe503003ac20edec0ff9fc9bf0bad85b595c34`.
Releaseinput laut gebundenem Artefaktmanifest:
`ccd32e29d465795f52efff3f8a72c567c9089714`. Kein Rebuild, keine Instrumentierung
des Binaries und kein Überschreiben bestehender Belege.

Neue Evidenzwurzel: `/root/Documents/artifact-boot-20261017-1645/`, Modus **0700**.
Am **17.10.2026 16:42:34 UTC** neue Arbeitskopien per SQLite-Backup-API aus
`/root/Documents/upgrade-drift-20261017-1630/original/{public,internal}.db`,
Quellen ausschließlich `mode=ro` und `query_only=ON`. Beide Kopien vor Start
byteidentisch zu den Originalbackups und vollständig tabellen-/schemaidentisch
zum Vorzustand der 16:30-Probe. Das sind neue Arbeitskopien der ursprünglichen
Sicherung mit Produktionsdatenstand **16:34:07–08 UTC**, keine neue spätere
Produktionssicherung. In diesem Folgegate wurde keine ProdDB geöffnet.

Explizite CLI:

```text
--bind 127.0.0.1:34493
--data-dir /root/Documents/artifact-boot-20261017-1645/boot
--base-url http://127.0.0.1:34493
```

Port vorher auf Loopback als frei ermittelt. `cwd`, `HOME`, `TMPDIR` und alle
drei XDG-Verzeichnisse liegen ausschließlich unter der neuen Privatwurzel.
**Keine Environment-Vererbung**: nur diese expliziten Verzeichnisse,
`PATH=/usr/bin:/bin` und `RUST_LOG=info`; tatsächliches `/proc`-Environment und
`cwd` mit aufgezeichnet. Keine Produktions-ENV-/Proxy-/Credential-/Datadirconfig.
Healthclient ohne Proxy, nur Loopback-GET `/health`.

## Tatsächlicher Prozessablauf und Schedulergrenze

* Start **16:42:41.470631 UTC**, eigener PID **1094286**.
* Echter Serverlog: `ingestion: trader seed up to date (763 rows written)`,
  danach `schrott-mcp listening on 127.0.0.1:34493`.
* `/health` um **16:42:42.546128 UTC**: **HTTP 200**,
  `{"ok":true,"service":"schrott-mcp"}`. 37 vorherige Connection-refused-Proben
  während des Starts sind erhalten, kein endgültiger Bind-/Timeoutfehler.
  Health selbst ist ein statischer Liveness-Endpunkt; der nachfolgende vollständige
  DB-Vergleich belegt die Seedwirkung, nicht die Healthantwort allein.
* Danach ausschließlich **SIGTERM an den eigenen Popen-PID**, reaped mit Exit
  **-15**. Ende **16:42:42.550462 UTC**, Gesamtlaufzeit **1.079732 Sekunden**.
  Kein SIGKILL/Watchdogeingriff; eigener Prozess nicht mehr vorhanden und Port
  danach geschlossen. Kein behaupteter graceful-shutdown-Test.
* Unveränderter normaler Code **startet den Scheduler**, dieser wartet vor dem
  ersten `run_once` jedoch **15 Sekunden** (`pipeline.rs:667–689`). Prozessende
  nach 1.08 Sekunden lag davor; zusätzlich privater 8-Sekunden-Hardstop abgesichert.
  Kein Trigger von `/api/ingest/run`, kein manueller Scrape oder Ingest.
* **Gesamte Internal-DB einschließlich Schema und aller Tabellen unverändert**:
  keine neuen Runs, Steps oder Fetches, keine anderen internen Bootwrites.
  Serverlog enthält genau die beiden genannten Zeilen, stderr leer.
  **Kein regulärer Schedulerlauf getestet**; dieses Betriebsgate bleibt offen.

## Vollständiger Vergleich, keine weg-normalisierten Überraschungen

Alle Tabellen/Spalten/PKs beziehungsweise Rowids und Schemas vor/nach echtem
Serverboot erfasst; zusätzlich kompletter Rohdiff gegen
`upgrade-drift-20261017-1630/snapshots/seeded.json.gz`, also den erwarteten
**ersten** Vollseed, nicht den zweiten Materialrefresh.

| Gegen erwarteten ersten Seed | Tatsächlicher Unterschied |
|---|---|
| Materialien | 83 Zeilen, ausschließlich `updated_at` |
| Händler | 763 Zeilen `updated_at`; davon bei 12 neuen Händlern zusätzlich `first_seen_at` |
| FTS einschließlich Schatten-/Layouttabellen | **0 Unterschiede** |
| Sämtliche `sqlite_sequence`-Zeilen | **0 Unterschiede** |
| Alle anderen Tabellen und Schemas, einschließlich Internal | **0 Unterschiede** |

Damit stimmen **IDs, Payloads, Hashes, Keep/Clear-Ergebnisse, FTS-Layout und
Sequenzstände exakt** mit dem erwarteten ersten Seed überein. Nur die real
verschiedenen Ausführungszeiten unterscheiden sich; keine Normalisierung von
FTS oder Sequenzen war nötig. Händler **3881→3893**, dieselben **751 Updates /
12 Inserts / 0 Deletes**; Materialien **54→83**. Bestehende Händler-IDs und
first_seen identisch zur Vorprobe. Tradersequenz **26621**, Materialsequenz
**107785**, wie im erwarteten ersten Seed. Keine zweite Seedrunde hinzugefügt.

**21147 Preise, 2426 Current, 1315 Annahmezeilen**: gegenüber den neuen
Vorstartkopien und dem erwarteten Seed in **allen Spalten und Keys identisch**.
Alle drei akzeptierten additiven Tabellen leer, `user_version=6`.
`integrity_check=ok` für beide DBs; Public weiterhin exakt die bekannten
24 FK-Altverletzungen, `trader_materials` Rowids1070–1093 / fehlender Trader15757;
Internal keine. Keine vollständige FK-Integrität oder Datenreparatur behauptet.

## Hofmann: spezifische Quellenidentität, keine neue Transition

Der bestehende kompilierte V3-Harness und seine Instrumentierung sind auf alte
DB-/HTML-/Logpfade und Marker fest verdrahtet; seine ursprünglichen
`debug/deps` fehlen. Unverändertes Ausführen würde alte Evidenzpfade benutzen,
nicht diese neuen Kopien. Deshalb **keine Wiederholung der Transition** und
kein neuer Transitionbuild/Livefetch; stattdessen die ausdrücklich erlaubte
spezifische Quellenidentität nachgewiesen.

Die alte exakt manifestierte `instrumentation.patch` wurde ausschließlich in
einer **neuen Kopie** rückwärts angewandt. Danach **27 relevante Dateien**
byteidentisch zum finalen Releaseexport und zusätzlich gegen die Git-Blob-IDs
von `ccd32e29` geprüft: gesamtes Core-/Storemodul einschließlich V3-DDL,
UTC-/Legacyguard und Transaktion, Hofmannhandler, typisierter Recorder
`traders/mod.rs`, Scheduler, Pipeline/Seedpfad, Cargo-/Lockinputs.
**279 alte Evidenzhashes** plus damalige Instrumentierungsquellhashes erneut
verifiziert; keine alten Quellen/Probedaten geändert.

| Spezifischer V3-Pfad | Gemeinsamer SHA256 |
|---|---|
| `crates/store/src/public/hofmann_tin.rs` | `4167aca01c504cdeda1e1b241a34f748dd01c1491ffb9b681bbe11127d5f09ac` |
| `crates/ingestion/src/traders/handlers/hofmann_rastatt.rs` ohne alte Instrumentierung | `636181147a39c2c351ec7fcfa1d25ee9305c9662f9b49936a70af7738707d58d` |
| `crates/ingestion/src/traders/mod.rs` | `35b72f9a14113168b28bd1e4a5fd068d2b3197489ac037d98f88bc77b025b724` |

Alte Transitionbelege weiterhin unter
`/root/Documents/hofmann-v3-private-upgrade-20261016-0830/`, gebundenes Manifest
`5b2956a7ceb0485a0f7ba44c7bb0845641bbd747a94d75bd7d359c754089d543`.
Diese Quellenidentität ist **kein neuer Lauf des finalen Binaries durch den
Cutover**. Hier nicht erneut ausgeführt: qualifizierter HTTP-Scrape, Entfernung
des Legacy-Current bei unveränderter Historienzeile, alt-Zeile/Ledgerentscheidung,
Retry einschließlich späterem Retry, Reopen und synthetisches Late-Replay auf
den neuen Kopien. Der reale Kurzboot lässt den Ledger weiterhin leer.

## Ownerbelege und Erhalt der bisherigen 4524 Dateien

Alle folgenden Pfade relativ zur **neuen** Evidenzwurzel:

* `run.py`, `copy-manifest.json`, `boot-result.json`, `logs/server.{stdout,stderr}`:
  unverändertes Artefakt, vollständige CLI/ENV/PID-/Health-/Stopzeiten und Logs.
* `snapshots/{before,after}.json.gz`, `diff-actual-boot.json`,
  `diff-harness-expected-vs-artifact.json`: vollständige Alltabellen-/Allspalten-
  und Schema-/PK-/Rowiddiffs, auch Internal/FTS/Sequenzen.
* `integrity.json`, `audit.py`, `audit.json`: konkrete Assertions, FK-Einzelzeilen
  und Summen; nicht bloß gefilterter Händlervergleich.
* `identity.py`, `deinstrument-command.json`, `old-probe-deinstrumented/`,
  `v3-source-identity.json`: Rücknahme nur der alten Probeinstrumentierung,
  alle 27 Quellenhashpaare und Liste der 279 erneut geprüften Altbelege.
* `prior-manifest-binding.json`, `prior-evidence-preservation.json`: altes
  Manifest und **alle 4524 manifestierten Dateien nach dieser Probe erneut
  hashidentisch verifiziert**. Alte volatile WAL/SHM-Sidecars waren bereits
  ausdrücklich nicht Teil dieses Manifests; deren Unverändertheit wird nicht behauptet.

Ein neues `manifest.json` bindet die neuen stabilen Privatdateien, volatile
SQLite-WAL/SHM-Dateien ausgenommen: **22 Dateien**, alle Hashes nachgeschrieben
und nochmals geprüft, `finalize.py` erhalten. Manifest-SHA256:
`423bc1ff728431fe52c8b32b45ab7e33abcaca9cf8143cc6164442aae56af6cc`.
Alte Berichte/Manifestdateien nicht editiert.
Keine Installations-/Restorefreigabe, kein Betriebsnachweis exklusiver Writer
oder regulärer Scheduler-/MKM-/Public-Readabnahme aus diesem Kurzboot abgeleitet.
