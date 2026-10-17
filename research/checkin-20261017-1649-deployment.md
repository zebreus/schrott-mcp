# Owner-Rollout 17.10.2026, 16:49 UTC

## Freigabe vor Installation

Gepushter Releaseinput **`ccd32e29d465795f52efff3f8a72c567c9089714`**, erneut direkt über `git ls-remote --heads origin main` bestätigt. Spätere lokale Commits betreffen ausschließlich Ownerberichte, keine neuen Runtime-/Seedinputs.

Owner hat den vollständigen [Artefakt-Bootbericht](checkin-20261017-1645-artifact-boot.md), tatsächliches Bootresultat und Audit gelesen; Manifest-SHA256 `423bc1ff728431fe52c8b32b45ab7e33abcaca9cf8143cc6164442aae56af6cc` sowie alle22 Dateien erneut selbst geprüft. Echter unveränderter Serverboot auf privaten Kopien, erwarteter Erstseed, Health und Quellenidentität zur früheren tatsächlichen Hofmanntransition sind damit als Vorinstallationsevidenz akzeptiert. Kein neuer Hofmanntransitionlauf behauptet. Ergänzend gelten die 746 Rust-/48 Python-Tests, frischen Livechecks, sechs Artefakt-Smokes und die explizite [aktuelle Driftfreigabe](checkin-20261017-1645-owner-drift-disposition.md).

## Tatsächlicher Ablauf

Ausführbares Owner-Skript `/root/Documents/deploy-20261017-reviewed.py`; Evidenz **`/root/Documents/deploy-20261017-reviewed-attempt2/`** (0700), tatsächliches `deployment.json`.

Abschlussmanifest `evidence-manifest.json` bindet zehn Evidenz-/Sicherungs-/Binary-/Skriptdateien, SHA256 `dc5d9c6fe42398bc1648e9825ef78980c38c96f5d2788c0543aafbca0f05ad91`. Alle143 fremden offenen Dateien nach Rollout erneut SHA256-identisch zur geschützten Baseline; nicht gebaut, gestaged oder deployt.

Vorbereitungsversuch1 brach **vor Dienststopp oder Installation** ab: Python-SQLite-Context schloss die eigene lesende Verbindung nicht; die strenge Holderprüfung erkannte deshalb auch den eigenen Prozess. Korrektur auf explizit schließenden Contextmanager, keine Lockerung der Holderprüfung; Erstbelegverzeichnis erhalten.

Versuch2:

1. Archivartefakt-Hashes, Remotecommit und Plattenreserve geprüft. Aktuelle Produktionshändler/-materialdefinitionen ausschließlich `mode=ro/query_only` mit dem akzeptierten Vorzustand verglichen: keine neue fachliche Drift.
2. `/proc`-Scan **aller Prozesse und FDs**, über tatsächliche Device-/Inodeidentität der DB-/WAL-/SHM-Dateien und Executablepfade, zusätzlich PID/Cgroup: vor Stopp ausschließlich MainPID15872 in der Service-Cgroup. Nicht nur Prozessnamensfilter.
3. Dienst geordnet gestoppt; um **16:49:23.438381 UTC** `inactive`, MainPID0 und kein Produktionsdatei-/Binaryholder mehr. Nach Sicherung vor Installation erneut keine Holder. Keine konkurrierenden alten/neuen Writer beobachtet.
4. Beide DBs bei gestopptem Dienst über SQLite-Backup-API aus **read-only/query-only** geöffneten Quellen gesichert; Integrität jeweils `ok`. Kein direktes SQL-DML/DDL gegen Produktion. Beide Altbinaries gesichert und hashidentisch geprüft. Die Sicherung ist keine Restorefreigabe.
5. Beide neuen Binaries während des Stopps über getrennte geprüfte Stagingdateien gemeinsam ersetzt; Installation abgeschlossen **16:49:25.089495 UTC**. Kein alter Binary-/DB-Rollback.
6. Regulären Dienst gestartet, neuer MainPID **1095156**. Lokaler und öffentlicher HTTPS-Healthcheck erfolgreich. Zwei lokale Connection-refused-Antworten während Startup wurden durch begrenzte Curl-Readiness-Retries gefolgt von Erfolg; kein späterer Ausfall verschleiert.
7. Neuer Prozess über `/proc/1095156/exe` exakt hashgebunden; nach Start ausschließlich dieser Prozess in der Service-Cgroup als beobachteter Produktionsholder. Journal belegt normalen Seed mit763 geschriebenen Händlerzeilen und Listener auf127.0.0.1:4001.

### Frische Sicherungen bei gestopptem Dienst

| Datei unter Evidenzwurzel | SHA256 |
|---|---|
| `backup/public.db` | `c461b75813eb0e093c324478830e3600d5e6d6b7003501868dee46572dface28` |
| `backup/internal.db` | `0ff614147525e31f33748082ca77935306ead2227bcb51ff550ed93dd6f2479e` |
| `old-binaries/schrott-mcp-server` | `07fbc7ef44be06b7542c2d0874dcc3af8084523718e418a1ebd14c0f3c50dca4` |
| `old-binaries/schrott-mcp-query-worker` | `b9c7f34c1622f6b0e13986e4583426970476c1a90556ffb380906aefdb4c4b2c` |

### Tatsächlich installiert

| Binary | SHA256 |
|---|---|
| `/usr/local/bin/schrott-mcp-server` | `7de235b153c571d2ed346dbb39fe503003ac20edec0ff9fc9bf0bad85b595c34` |
| `/usr/local/bin/schrott-mcp-query-worker` | `854424a12fb80478731e65367fc7844dca71cab103f2594a2ecd9dafd20786ec` |

## Nachinstallation: tatsächlicher read-only Vollvergleich

Beleg `postboot-readonly-audit.json`, **16:49:59 UTC**:

- **3893 Händler, 83 Materialien, 2426 Current**, Schema6. Alle Händler-/Materialpayloads nach Slug identisch zur freigegebenen privaten Artefaktprobe, abgesehen von ausdrücklich getrennten IDs/Laufzeitfeldern; bestehende IDs sämtlich unverändert. Neue IDs vollständig aufgezeichnet, nicht vorausgesetzt.
- **21159 Preise, 2426 Current und 1315 Annahmen** in allen Spalten und Keys identisch zur tatsächlichen Sicherung unmittelbar vor Installation. Die zwölf zusätzlichen Preise gegenüber der 16:34-Probe entstanden schon vor Stopp durch den alten regulären Betrieb, nicht durch Seed oder eine erzwungene Ingestion.
- Public `integrity_check=ok`, unverändert exakt24 bekannte FK-Altverletzungen bei Rowids1070–1093. Keine Reparatur oder vollständige FK-Integrität behauptet.
- Drei additive Ledger noch **leer**. Daher insbesondere noch keine produktive Hofmann-Cutoverentscheidung oder MKM-Artikelownership als bestanden ausgegeben.
- Installierter Worker führt direkt eine ausschließlich lesende Produktions-SELECT-Abfrage aus: Exit0 und3893/83/2426. Das prüft das neue Binary und die Daten, nicht separat die Server-Confinementgrenzen.

## Noch offene reguläre Abnahme

Rollout-Untergrenze **`2026-10-17T16:49:25.089911+00:00`**. Kein `/api/ingest/run`, keine erzwungene Handleringestion. Die Prüfung muss tatsächliche reguläre Steps/Fetches/Preise seit dieser Grenze verwenden:

```sh
python3 scripts/verify-handler-release.py --since 2026-10-17T16:49:25.089911+00:00
python3 scripts/verify-mkm-release.py --since 2026-10-17T16:49:25.089911+00:00
python3 scripts/verify-pending-corrections.py --since 2026-10-17T16:49:25.089911+00:00
```

Zusätzlich Hofmannledger/ID13725-Historienerhalt und Nachbarerhalt gezielt gegen die neue Sicherung prüfen; der allgemeine Handlerverifier allein deckt diesen Vertrag nicht vollständig ab. Quellen-/Mappingdrift nicht durch feste Sollzahlen kaschieren.

Phasen nach unverändertem Slughash/Scheduler berechnet, **Schätzungen, keine Laufnachweise**: Trapper etwa17:05, MKM17:20, Saxonia17:50, reGOLD18:05, AGH20:05, Hofmann20:35, ELNO22:05 UTC. Handler davor und Laufzeit können nach hinten verschieben. Rohdaten `scheduler-phase-estimates.json`. Vollständige reguläre Abnahme ist deshalb jetzt noch offen; keine wartenden Agenten/erzwungenen Läufe als Ersatz.
