# Rollout-Review 16.10.2026, Check-in 03:00

## Ergebnis und Grenzen

**Kein Rollout durchgeführt; Veröffentlichung noch nicht freigabefähig.** SAXONIA und Hofmann sind im aktuellen Quelltext korrigiert, in Produktion aber weiterhin auf dem alten Fehlerpfad. Zwei konkrete Verifikationsprobleme: Der vorhandene Handler-Prüfer erwartet veraltete Semantik; Hofmann bekommt voraussichtlich einen zusätzlichen alten Zinn-Current-Pointer. Außerdem würde ein Build aus dem schmutzigen Arbeitsbaum ungeprüfte Dossiers in das Serverbinary einbetten.

Diese Prüfung war rein lesend, ausgenommen diese ausdrücklich angeforderte Ergebnisdatei. Keine Builds/Tests mit Artefaktschreibzugriff, Installation, Neustarts, Commits, Backups, Seed-Ausführung oder Ingestion ausgelöst. Produktionszugriffe ausschließlich SQLite-URI `mode=ro` plus `PRAGMA query_only=ON`; HTTPS ausschließlich GET `/health`. Frühere Test-/Live-Nachweise unten sind zitierte Berichte, keine frisch ausgeführten Tests.

## Frisch festgestellter Betriebsstand

- `systemctl cat/show schrott-mcp.service`: aktiv; MainPID 15872, gestartet **09.10.2026 20:04:12 UTC**. `ExecStart=/usr/local/bin/schrott-mcp-server`, root, WorkingDirectory dieses Repo, `DATA_DIR=/var/lib/schrott-mcp`, Bind `127.0.0.1:4001`. Drop-in `20-schrottindex.conf` überschreibt alte Base-URL korrekt mit `https://schrottindex.de`.
- GET `https://schrottindex.de/health`: `{"ok":true,"service":"schrott-mcp"}`. Das beweist weder Workerfunktion noch neue Handler.
- Beide installierten Binaries ausführbar, Zeitstempel 09.10.2026 20:03: Server 83.896.072 Bytes, Worker 20.179.808 Bytes. Daraus allein folgt keine Commitidentität.
- `/usr/bin/prlimit`, `/usr/bin/pdftotext`, `/usr/bin/sqlite3` und `/root/.cargo/bin/cargo` vorhanden. PDF-Voraussetzungen sind hier **kein aktuell fehlender Blocker**. Rund 14,5 GB freier Platz auf `/`; Bedarf für separaten Releasebuild/Backups trotzdem vor Ausführung abschätzen.
- `gh auth status`: aktive GitHub-Anmeldung vorhanden. Die historische Aussage „keine Anmeldung“ ist aktuell überholt. **Kein Pushversuch** in dieser Prüfung; erfolgreiche Veröffentlichung bleibt unbelegt. Keine Tokenwerte in diesen Bericht übernommen.
- Public-Schema `PRAGMA user_version=6`; 3881 Händler, 54 Materialien, 2426 Current-Preise. Internal `user_version=0` ist ein getrennter Schema-Zähler und nicht mit public Version 6 gleichzusetzen.
- SAXONIA letzter gelesener Schritt ID 8318, Run 1815, **16.10. 01:01:31 UTC**, fehlgeschlagen mit HTTP 404 am alten Pfad. Vier Current-Preise: `exact`, Confidence 1.0, alter URL-Pfad, beobachtet 15.10. 06:52:49 UTC, Seitendatum 14.10.
- Hofmann letzter gelesener Schritt ID 8239, Run 1802, **15.10. 21:44:38 UTC**, fehlgeschlagen wegen gemischter Zeilendaten. 23 alte Current-Preise, beobachtet 14.10. 03:24:24 UTC, alle `page_stated`, Datum 30.09. Darunter **Material zinn, Variante leer**, 11,50 EUR/kg.
- HEAD bei Prüfung `81a64832`; zahlreiche geänderte und unversionierte Dossiers sowie fremde Researchdateien. Keine davon verändert.

## Startpfade und tatsächliche Schreibwirkung

`crates/server/src/main.rs:78–112` öffnet zuerst `InternalDb::open`, danach **schreibend** `PublicDb::open`, ruft `seed_metadata` auf und startet den Scheduler. Erst danach wird der Listener gebunden. Ein Probe-Server mit anderem Port und produktivem `--data-dir` ist deshalb **kein read-only Smoke-Test**; sogar ein späterer Bindfehler kommt erst nach möglichen DB-Änderungen.

`crates/store/src/public/mod.rs:268–281` aktiviert WAL, führt Schema-DDL, Legacy-DDL, Trader-/Preismigrationen und View-DDL aus. `prices.rs` besitzt Migrationen bis Version 6, teils mit History-Annotation und Pointer-Retirement. Aktuelle DB hat Version 6; das ist kein Beweis, dass alle anderen Open-DDL/Traderpfade wirkungslos wären. Hofmann/SAXONIA-Fixes selbst benötigen laut aktuellem Code/Ownerbericht keine neue Storemigration.

`pipeline.rs:543–566`: Materialkatalog-Upserts und vollständiger eingebetteter Traderseed; Seedfehler beendet den Server vor Listen. Kein globaler atomarer Start über beide DBs und gesamten Katalog/Seed erkennbar; Teilwirkung bei späterem Fehler nicht ausschließen. `pipeline.rs:683–686`: erster normaler Schedulerlauf nach 15 Sekunden, dann Schlaf 15 Minuten nach Laufende; auch diese Läufe seeden erneut. Kein vorhandener CLI-Schalter für `--no-seed`, `--no-scheduler`, Migration-only oder Dry-run. Nicht erfinden.

`build.rs` kompiliert die **lokalen** Dossiers in `$OUT_DIR`; `seed_traders.rs` bettet sie ein. Git-Tracking schützt nicht vor Aufnahme unversionierter Dossiers. Deshalb kein Releasebuild aus diesem Arbeitsbaum. Leere Seedfelder löschen nicht allgemein Enrichment; `clear_fields` und bestehende GEO-Paare gesondert nach `docs/trader-seed.md` prüfen. Die dokumentierten ungeklärten GEO-Punkte bei Bamberg/Florian und Roding/Jansen sind Beispiele notwendiger Owner-Entscheidungen, falls diese Dossiers zum Releaseumfang gehören.

Worker: `crates/server/src/mcp/worker.rs:139–146` sucht `schrott-mcp-query-worker` direkt neben `current_exe()`. Der Worker nimmt `<data-dir>` als Positionsargument, JSON über stdin und öffnet ausschließlich `PublicDb::open_read_only` (`crates/query-worker/src/main.rs:48–69`). Server setzt OS-Limits und bei root UID/GID 65534; Verzeichnis-, DB-, WAL-/SHM-Lesbarkeit ist Teil des Smoke-Gates. Aktuell Verzeichnis 0755 und public DB/WAL/SHM 0644. Direkter Root-Worker-Test ersetzt den echten MCP-Test unter Confinement nicht.

## Reale Blocker und Verifikationslücken

1. **Releaseumfang/Seed ungeprüft:** Release aus sauberem, explizit freigegebenem Commit bauen, nicht aus dem aktuellen Repozustand. Seit dem installierten Stand akkumulierte Änderungen mitprüfen; ein „nur zwei Handler“-Redeploy ist sonst in Wahrheit ein umfassender Seed-/Code-Rollout.
2. **Bestehender Handler-Prüfer veraltet:** `scripts/verify-handler-release.py:18` fordert SAXONIA `exact`, obwohl der neue Handler `approx`/0.8 liefert. Zeilen 82–84 fordern bei Hofmann `page_stated`, korrekt ist jetzt **row_stated**. Exit 1 nach richtigem Rollout wäre daher erwartbar, kein Anlass zur Datenmanipulation. Ein unverändertes Grün wäre gerade kein Nachweis der neuen Semantik.
3. **Hofmann Zinn-Keywechsel:** neues Mapping `Zinn alt` → `zinn/alt`, altes Current ist `zinn/''`. `traders/mod.rs` zeichnet neue Preise je Material/Variante auf; im gelesenen Hofmann-/generischen Recorderpfad kein Retirement des alten Schlüssels. Bei unveränderter Quelle daher **23 neue Beobachtungen plus 1 alter Current = 24** zu erwarten. Vor Abnahme in Wegwerf-DB aus aktuellem Snapshot reproduzieren und explizit entscheiden: bestehender Current bleibt als Altlast sichtbar oder eng begrenztes, getestetes Retirement über regulären Anwendungspfad nötig. **Keine Produktions-SQL-Löschung.** Der bisherige TempDB-Test mit leerer DB beweist dieses Upgrade-Szenario nicht. Exakt-23-Gate keinesfalls einfach auf 24 ändern und alte Quote ignorieren.
4. **Keine frischen Release-/Live-Gates:** frühere Berichte liefern lokale Tests, aber nicht für den noch auszuwählenden sauberen Release. Auth wieder verfügbar, Push/Remoteidentität und Buildidentität dennoch offen.
5. **Rollback nicht nur Binarywechsel:** Bootseed/Migrationen und normale Ingestion sind schreibend. Alte Binaries zurückzustellen stellt Daten nicht zurück. Wiederherstellung aus Backup wäre eine gesondert freizugebende Produktionsmutation mit Verlust neuerer Daten; nicht Bestandteil dieser read-only Prüfung.

## Vorhandene Prüfer: geeigneter Einsatz

Alle fünf Release-/Postrollout-Prüfer öffnen DBs mit `mode=ro`/`query_only`; sie lösen keine Ingestion aus. Die `test_verify_*.py` verwenden dagegen temporäre Fixture-DBs und sind keine Produktionsbelege.

| Kommando | Reichweite / Grenze |
|---|---|
| `python3 -B scripts/verify-handler-release.py --since "$ROLLOUT_SINCE"` | Sechs Handler; derzeit **nicht als SAXONIA-/Hofmann-Abnahme geeignet**, siehe oben. |
| `python3 -B scripts/verify-mkm-release.py --since "$ROLLOUT_SINCE"` | Persistierte MKM-Contracts, feste 55/5 quellenabhängig; nur bei betroffenem Releaseumfang. |
| `python3 -B scripts/verify-kernschrott-release.py --since "$ROLLOUT_SINCE"` | Begrenztes Kernschrott-Gate; kein allgemeiner Source-Audit. |
| `python3 -B scripts/verify-pending-corrections.py --since "$ROLLOUT_SINCE"` | MSG, explizite Seed-Clears/Gotha; kein vollständiger Seed-/GEO-Audit. |
| `python3 -B scripts/verify-wave-e-postrollout.py --since "$ROLLOUT_SINCE"` | Wave-E-Erwartungsdatei/Timeline-Publikation; kein Handlernachweis. |

`--since` muss der tatsächliche Rolloutbeginn sein, nicht ein Datum, das alte Schritte grün macht. Zwei unabhängig gelesene DBs sind kein gemeinsamer Snapshot. `ingestion_runs` enthält keinen Trigger-Typ; Journal/Operationsnachweis „kein Force-run“ ergänzen. Kurz laufende Read-Transaktionen nach abgeschlossenen regulären Schritten verwenden.

## Sicherer Rolloutplan für den ausführenden Owner (hier NICHT ausgeführt)

1. Releasecommit und **alle** enthaltenen Dossiers/Seedänderungen freigeben; Remoteveröffentlichung überprüfen. Saubere isolierte Releasequelle verwenden, fremde Änderungen weder stashen noch verwerfen. Build-/Commit-/Cargo.lock-Identität dokumentieren. Veralteten Prüfer separat korrigieren und offline testen; Hofmann-Upgradealtlast entscheiden.
2. In dieser sauberen Quelle passende bestehende Gates ausführen:

   ```sh
   cargo fmt --all -- --check
   cargo test -p schrott-mcp-ingestion saxonia
   cargo test -p schrott-mcp-ingestion hofmann_rastatt
   cargo test -p schrott-mcp-ingestion seed
   cargo test --workspace
   cargo test -p schrott-mcp-ingestion --example live_handlers
   cargo run -p schrott-mcp-ingestion --example live_handlers -- sn-halsbrucke-09633-saxonia-edelmetalle
   cargo run -p schrott-mcp-ingestion --example live_handlers -- bw-rastatt-hofmann
   cargo build --release --locked -p schrott-mcp-server -p schrott-mcp-query-worker
   ```

   Die letzten beiden Handlerloops sind DB-frei, aber Cargo schreibt Buildartefakte und HTTP erfolgt live; deshalb in dieser Prüfung nicht gestartet. Tatsächliche Testanzahl/Exitstatus lesen, kein Nulltest als Erfolg. Keine produktive Serverinstanz zum Test starten. Frühere Befunde: SAXONIA sieben Tests/4 Preise/6 Skips (`checkin-20261015-1300-owner.md`); Hofmann neun Tests/23 Preise, zeilenbezogene Dates (`checkin-20261014-hofmann-mixed-dates.md`, Nachtrag). Tageskurse können seither wechseln.
3. Freigegebene WAL-sichere Backups **beider** DBs und Restoreverfahren vorbereiten. Bestehender dokumentierter Weg ist `sqlite3 -readonly <quell-db> ".backup '<separates-backupziel>'"`, niemals `cp` der laufenden DB. Backup schreibt nur ins neue Ziel; kein Backup hier ausgeführt. Für gemeinsamen konsistenten Rücksetzpunkt Schreibquieszenz im genehmigten Wartungsfenster organisieren. Migration/Seed auf Wegwerfkopien proben; kein Netzwerk-Serverstart als vermeintlich isolierter Migrationslauf.
4. Beide aus demselben Releasebuild stammenden Binaries außerhalb der Livepfade bereitstellen, Prüfsummen/Architektur/Abhängigkeiten/Modi prüfen. Im ausdrücklich genehmigten Wartungsfenster Dienst anhalten, alte Binarypaarung sichern, beide Binaries austauschen und danach Dienst starten. **Kein Überschreiben laufender Executables, kein gemischtes Server-/Workerpaar während laufender Requests.** Einzelne Dateiumbenennungen sind kein atomarer Paarwechsel; die Dienstpause bildet die Grenze. Exakte Rolloutzeit erfassen. Diese Installations-/Dienstaktionen sind Vorschlag für spätere Autorisierung, nicht hier erteilte oder genutzte Berechtigung.
5. Sofort danach Dienst, Journal, `/health`, OAuth-Discovery und einen vorhandenen authentifizierten MCP-Client mit `schrott_query_sql`/`SELECT 1` prüfen. Keine neuen Tokens/Benutzer nur fürs Smoke-Gate anlegen. Als zusätzlicher DB-read-only Workercheck möglich:

   ```sh
   /usr/local/bin/schrott-mcp-query-worker /var/lib/schrott-mcp <<'JSON'
   {"sql":"SELECT 1 AS rollout_probe"}
   JSON
   ```

   Das prüft nicht den Server-Confinementpfad. `journalctl -u schrott-mcp.service --since "$ROLLOUT_SINCE" -o cat` auch auf JSON-WARN/ERROR lesen; ausschließlich `-p warning` übersieht normale stderr-JSON-Logprioritäten.
6. Normale Schedulerphasen abwarten, kein `POST /api/ingest/run`, kein `run_once(force=true)`. Beide Handler nutzen Every-6h; nach Neustart stabile Slugphase, 15-Minuten-Polling **plus** sequenzielle Handlerlaufzeiten. Bis ungefähr eine volle Periode plus Poll-/Laufpuffer einplanen, nicht sofort einen fehlenden neuen Schritt als Parserfehler werten. Restart ist kein Force-run und auch kein Sofortabruf aller Handler.
7. Erst nach abgeschlossenen regulären Schritten Spotprüfung unten und passende freigegebene Prüfer ausführen. Installationserfolg und Datenveröffentlichung getrennt melden. Fehler führt zu Stopp/Eskalation gemäß Rollbackplan, nicht zu direktem SQL-Reparieren oder erzwungener Ingestion.

## SAXONIA/Hofmann: präzise read-only Spotabnahme

Bestehende einfache Kommandos für die Persistenzsicht (hier als Wiederholungsanleitung; `$ROLLOUT_SINCE` zusätzlich bei Auswertung tatsächlich anwenden):

```sh
sqlite3 -readonly /var/lib/schrott-mcp/internal.db \
  "PRAGMA query_only=ON; SELECT id,run_id,scraper,status,items_upserted,message,started_at,finished_at FROM ingestion_steps WHERE scraper IN ('sn-halsbrucke-09633-saxonia-edelmetalle','bw-rastatt-hofmann') ORDER BY id DESC LIMIT 8;"
sqlite3 -readonly /var/lib/schrott-mcp/public.db \
  "PRAGMA query_only=ON; SELECT t.slug,m.slug,p.variant,p.price,p.unit,p.price_kind,p.confidence,p.source_url,p.observed_at,p.published_at,json_extract(p.extra_json,'$.published_at_basis'),p.notes FROM traders t JOIN current_prices c ON c.trader_id=t.id JOIN prices p ON p.id=c.price_id JOIN materials m ON m.id=p.material_id WHERE t.slug IN ('sn-halsbrucke-09633-saxonia-edelmetalle','bw-rastatt-hofmann') ORDER BY t.slug,m.slug,p.variant;"
```

- Für jeden Handler Schritt `started_at >= ROLLOUT_SINCE`, Status ok, finished_at gesetzt, keine CANARY-/unerwarteten Skipmeldungen; zugehörigen `raw_fetches`-Datensatz über **run_id und scraper** zuordnen: HTTP200, Bytes > 0, korrekte URL. Preisbeobachtungen ebenfalls nach Rollout; aktuelle Schlüsselmenge gegen denselben Quellenstand prüfen. Keine historische feste Tageszahl/Beträge als aktuelle Wahrheit ausgeben.
- **SAXONIA:** genau Gold/Silber/Platin/Palladium, EUR/g aus EUR/kg÷1000, `approx`, Confidence 0.8, `haendler_angabe`, published wahr, URL `https://saxonia.de/tageskursanzeiger/`. Explizites Seitendatum/`page_stated`; sechs verarbeitet/unverarbeitet-Buttons ausgeschlossen. Keine frischen Ankaufpreise aus Chart oder anderen Rubriken. Fehlende Kontakte aus neuem Impressum nicht als Löschauftrag behandeln.
- **Hofmann:** erwartete 23 neue, vollständig normalisierte Preiszeilen bei unveränderter Quellstruktur, exact/1.0, URL `https://hofmann-entsorgung.de/preise-und-verguetungen/`, **jede neue Zeile row_stated mit ihrem eigenen Datum**. EUR/t bei NE-Fraktionen korrekt in Katalog-EUR/kg umrechnen. Starterbatterien/V2A/Elektromotoren hatten bei letzter Owner-Liveprüfung weiter 30.09., übrige 14.10.; frisch gegen heutige Quelle prüfen, nicht pauschal „heutiges Datum“ verlangen. `Zinn alt` muss `zinn/alt` sein; zusätzlichen alten `zinn/''`-Pointer ausdrücklich ausweisen und nicht als neue Zeile zählen oder verschweigen.

**Freigabestatus:** read-only Bestandsaufnahme abgeschlossen. Saubere Releaseauswahl, belastbarer Seed-/Upgrade-Probelauf, korrigiertes Verifikationscontract und Entscheidung zum Hofmann-Altkey fehlen vor einer vollständigen Rolloutabnahme. Kein Produktionsfix behauptet.
