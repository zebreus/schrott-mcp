# Begrenzter tmpfs-Cacheaudit — 18.10.2026, 17:30 UTC

## Entscheidung in einem Satz

**Nur den kleinsten vollständigen, eindeutig zugeordneten Incremental-Unterbaum als Owner-Kandidaten empfehlen:** `/tmp/opencode/sperling-target/debug/incremental/schrott_mcp_core-3e0rkw74qjmz1` — **1.634.304 Bytes (1,5586 MiB) freisetzbare Dateiblöcke**, bei unveränderten Links und ohne Halter. **Kein gesamtes Target, kein `debug/build`, kein Testexecutable freigegeben; nichts gelöscht.** Der kleine Gewinn ist ausdrücklich nur minimale Entlastung, keine Lösung für große Builds oder Backups.

„Kleinster“ meint hier einen **vollständigen cratebezogenen Incremental-Cache**, nicht eine willkürlich herausgegriffene einzelne `.o`-Datei. Zwei Core-Caches sind beim Nettogewinn gleich klein; empfohlen wird der ältere von 14:03 statt 14:04 UTC am 11.10. Für einen größeren Eingriff braucht es eine gesonderte Owner-Entscheidung; aus dieser Tabelle folgt kein Sammellöschauftrag.

## Umfang und Messung

Neuer begrenzter Folgeauftrag, kein Neustart des Rootaudits. Rekursiv inventarisiert wurde ausschließlich `/tmp/opencode/sperling-target`. Zusätzlich nur gezielte Repo-Researchreferenzen, die beiden ausdrücklich zugehörigen Sperling-Testlogs, Git-Metadaten für `crates/core`, `/proc`-Haltermetadaten und die vom Auftrag benannte vorherige Cleanup-Evidenz gelesen. Keine DB geöffnet, keine Test-/Buildprogramme gestartet, keine Secrets oder Prozessumgebungen gelesen. Einziger eigener Schreibpfad ist dieser Bericht.

Aufnahme `2026-10-18T17:31:13Z`, `df -B1 /tmp`:

| tmpfs gesamt | Belegt | Verfügbar | Anzeige |
|---:|---:|---:|---:|
| 4.061.138.944 | 3.959.758.848 | **101.380.096 Bytes (96,6836 MiB)** | 98 % |

`du -x -B1` des begrenzten Baums, belegte Blöcke mit Hardlink-Deduplizierung:

| Pfad relativ zu `/tmp/opencode/sperling-target/` | Bytes |
|---|---:|
| gesamter Baum | 2.649.153.536 |
| `debug` | 2.649.145.344 |
| `debug/build` | 1.657.798.656 |
| `debug/incremental` | 990.355.456 |
| `debug/examples` | 991.232 |

Die kleine `examples`-Zahl ist **kein** kleiner Binarybestand: GNU `du` hatte die zwei Binary-Hardlinks bereits unter `build` gezählt. Nicht Eltern und Kinder addieren, keine logischen Dateigrößen mit belegten Blöcken verwechseln.

Die bereits erfolgte **Root**entlastung ist abgeschlossen: `/root/Documents/checkin-20261018-1331-cache-cleanup.json` dokumentiert ausschließlich den damaligen Root-Incremental-Unterbaum und einen tatsächlich beobachteten Gewinn von **250.032.128 Bytes**. Dieser Vorgang wird hier weder wiederholt noch verändert und hat das separate `/tmp`-tmpfs nicht entlastet.

## Inhaltstypen, Links und Alter

Vollständige Metadateninventur: **4.204 reguläre Dateien, 1.076 Unterverzeichnisse, null Symlinks, null Spezialdateien**. Root-Device/Inode bei Aufnahme: **33 / 80920**. Datei-mtime-Spanne des gesamten Targets: **11.10.2026 14:02:40.284899 bis 14:08:26.237642 UTC**. mtime ist Altersindiz, kein Eigentums- oder Reproduktionsbeweis.

Suffixinventur: 1.998 `.o`, 203 `.rlib`, 209 `.rmeta`, 12 `.so`, 3 `.a`, 251 `.d`, 291 `.json`, 274 `.timestamp`, 48 `.bin`, 16 `.lock`, acht `.rs`, zwei `.c`, ein `.TAG` und 888 Dateien ohne Suffix. Keine nach Dateinamen erkennbare `.db`/`.sqlite`/`.sqlite3`-Datei, keine `.log`/Markdown-/Python-/Archivdatei im untersuchten Target. Das allein wäre **kein** Freigabebeweis; die ausgewählten Incrementals wurden zusätzlich vollständig typgeprüft.

**Alle 16 Incremental-Unterbäume** enthalten ausschließlich:

- `.o` mit tatsächlich gelesenem ELF-Header **ET_REL = 1**, keine ET_EXEC-/ET_DYN-Testexecutables;
- jeweils exakt `dep-graph.bin`, `query-cache.bin`, `work-products.bin`;
- gegebenenfalls `metadata.rmeta`;
- je eine leere Session-`.lock`.

Keine abweichenden Dateien, Quellen, Logs, DBrepros oder ausführbaren Testarchive innerhalb dieser Incrementals gefunden. Die Headerprüfung aller dortigen `.o` las nur 20 Bytes pro Datei. **Compilerzustand ist nicht der originale Testnachweis:** Neu erzeugbarer Cache bedeutet weder historische Testwiederholung noch Garantie byteidentischer Outputs mit heute verfügbarer Toolchain.

### Vollständige Incremental-Tabelle

Präfix jeweils `/tmp/opencode/sperling-target/debug/incremental/`. „Allein freisetzbar“ zählt jeden Inode einmal und nur dann, wenn sämtliche Hardlinks innerhalb genau dieses Unterbaums liegen. Bei sechs `.rmeta` bleibt ein zweiter Link außerhalb des Unterbaums bestehen; dessen Blöcke sind konsequent abgezogen. Verzeichnisblöcke im Kandidaten sind null. Keine FD-/Mappinghalter beobachtet, siehe unten.

| Unterbaum | Eigene belegte Bytes vor Hardlinkabschlag | Allein freisetzbare Bytes |
|---|---:|---:|
| `schrott_mcp_core-3e0rkw74qjmz1` **Empfehlung** | 1.683.456 | **1.634.304** |
| `schrott_mcp_core-2250o6gkcs1an` | 1.683.456 | 1.634.304 |
| `schrott_mcp_core-2bp4j43rapcb5` | 1.650.688 | 1.650.688 |
| `schrott_mcp_query_worker-18uq92ui8t9ex` | 2.064.384 | 2.064.384 |
| `schrott_mcp_auth-0p1meqm1dhaqa` | 2.703.360 | 2.682.880 |
| `schrott_mcp_auth-142sk5ahb0zb9` | 3.166.208 | 3.166.208 |
| `build_script_build-1j3qu7feqr27t` | 3.604.480 | 3.604.480 |
| `live_handlers-2btt8gti3s6o2` | 7.966.720 | 7.966.720 |
| `mkm_inventory-1mf0aqqdthoqp` | 10.272.768 | 10.272.768 |
| `schrott_mcp_store-24r6qfb4z26kt` | 15.720.448 | 15.400.960 |
| `schrott_mcp_store-223b6xvaizy3t` | 15.802.368 | 15.478.784 |
| `schrott_mcp_store-0bgdyl9v0cqya` | 19.099.648 | 19.099.648 |
| `schrott_mcp_server-0kqzs9zshzryo` | 39.596.032 | 39.596.032 |
| `schrott_mcp_ingestion-0bofyoc2fkrpy` | 260.870.144 | 247.992.320 |
| `schrott_mcp_ingestion-1yw2b1z0d0qyp` | 307.949.568 | 307.949.568 |
| `schrott_mcp_ingestion-0g9lg0n436a0e` | 310.161.408 | 310.161.408 |

Dies ist eine **Typ-/Platzinventur**, keine pauschale Freigabe der übrigen 15 Unterbäume: deren historische Quellversionen wurden hier nicht je einzeln bis zum Commit rekonstruiert. Insbesondere die großen Ingestion-Caches stammen aus der damaligen Zwischenfassung; daraus keine Freigabe historischer Testexecutables ableiten.

## Exakter kleinster Kandidat und Quellbezug

Vollständiger Pfad:

`/tmp/opencode/sperling-target/debug/incremental/schrott_mcp_core-3e0rkw74qjmz1`

Enthält **45 reguläre Dateien**: 40 ET_REL-Objekte, drei Compiler-`.bin`, eine `.rmeta`, eine leere Lockdatei; keine weiteren Dateitypen. Datei-mtime **11.10.2026 14:03:03.437216–14:03:03.631219 UTC**. Session-Unterverzeichnis:

`s-hn44732hd2-14hkho5-5g847m3d7q9dh58qzxav0eh4s`

Hardlinkrechnung vollständig lokal aufgelöst:

- `.../s-hn44732hd2-14hkho5-5g847m3d7q9dh58qzxav0eh4s/metadata.rmeta`: **45.540 logische Bytes, 49.152 belegte Bytes, nlink=2, Inode 84828 auf Device 33**.
- Genau derselbe Inode liegt zusätzlich unter **`/tmp/opencode/sperling-target/debug/build/schrott-mcp-core/fd1c40e2f46b8d05/out/libschrott_mcp_core-fd1c40e2f46b8d05.rmeta`**. Dieser zweite Link bleibt erhalten; keine externe unbekannte Linkposition mehr offen.
- Alle übrigen 44 Dateien nlink=1. Gesamtblöcke **1.683.456 − 49.152 = 1.634.304 Bytes**. Das ist die exakte freisetzbare Blockmenge dieser Momentaufnahme, nicht eine Garantie des späteren globalen `df`-Deltas bei Parallelwachstum.

Quell-/Fingerprintbelege, nicht nur Ableitung aus dem Ordnernamen:

1. Cargo-Fingerprint `debug/build/schrott-mcp-core/fd1c40e2f46b8d05/fingerprint/lib-schrott_mcp_core.json` benennt Bibliotheksprofil, Features `[]`, `rustflags: []`, `compile_kind: 0`, Depinfo relativ zu diesem Buildpfad. Kein zusätzliches Repro-/Harnessprofil.
2. Zugehöriges `.d` referenziert ausschließlich **`crates/core/src/lib.rs`** als Quelle für `.rlib`/`.rmeta`. Binäres Cargo-Depinfo nennt `src/lib.rs`.
3. Ein Objekt dieses konkreten Incrementals (`e4z9y4pusdpaqcdg47xzeg5k9.o`) mit `readelf --debug-dump=info` untersucht: `DW_AT_comp_dir` **`/root/Documents/schrott-mcp-sperling-20261011`**, Producer **rustc 1.100.0-nightly (5ceaf6608 2026-09-25)**. Kein Programm ausgeführt.
4. `research/handler-owner-sperling-20261011.md` nennt denselben Worktree, Ausgangscommit **`d06f601d6de0bfe2c41fb76d4097c5722ec33759`** und genau fünf damalige Kandidatdateien; **kein Core-Quellfile** gehörte zu diesen Änderungen. Derselbe Bericht nennt ausdrücklich das isolierte `sperling-target`.
5. Lokal lesender `git diff --name-only d06f601d6de0bfe2c41fb76d4097c5722ec33759 HEAD -- crates/core` ist leer. Quellblob `crates/core/src/lib.rs`: **`f7742178e48b76a3e11cacbd3ef128886356cd6a`**, `crates/core/Cargo.toml`: **`e794e266a54d43ea81d3cd9ec05f80eeb0ec7446`**. Die Corequelle ist damit vorhanden und seit dem gebundenen Ausgangscommit unverändert; keine nur im Cache liegende Testquelle festgestellt. DWARF/Depinfo allein sind kein kryptographischer Sourcehash des damaligen Compilers; die dokumentierte enge Änderungsmenge ergänzt den Bezug.

Originalcache-Metadaten zur Owner-Nachprüfung, SHA256:

| Datei innerhalb der oben genannten Session | SHA256 |
|---|---|
| `dep-graph.bin` | `7f1d917bf7f7ed1799b824020e42bad028e3f6e884c5ff5d3ddc36b313a560d3` |
| `query-cache.bin` | `2e525dbd300594f0f0186d6e1de40bf2061a0480862cfd4010a67baf4c408d04` |
| `work-products.bin` | `da278a81747a4cc6312e494367f6921ee4a879e4c702ad18d0f7445468fb41d1` |
| `metadata.rmeta` | `56e8e30f5b1dc239c6f5c7b26d974e76b4e333346291048301e2821d2f839b32` |

## Was ausdrücklich erhalten bleiben muss

**`debug/build` ist kein reiner entbehrlicher Incremental-Baum.** Darin liegen acht generierte `.rs` (z. B. SQLite-Bindings und Web-Atoms), zwei `.c`, Cargo-Fingerprints/Compilerdiagnostik sowie historische Test-/Beispielexecutables. Deren Archiv-/Ersatzstatus ist nicht vollständig geklärt, also **keine Freigabe**. Konkrete erhaltene Executables unter `/tmp/opencode/sperling-target/`:

| Pfad | Logische Bytes | Evidenzbezug |
|---|---:|---|
| `debug/build/schrott-mcp-ingestion/a60a6b13ac59140f/out/schrott_mcp_ingestion-a60a6b13ac59140f` | 144.016.952 | explizit im Targeted-Live-Log ausgeführt |
| `debug/build/schrott-mcp-ingestion/ad989841b5e3a728/out/schrott_mcp_ingestion-ad989841b5e3a728` | 144.788.864 | explizit im Workspace-Log ausgeführt |
| `debug/build/schrott-mcp-core/2fa0d4220368f1e9/out/schrott_mcp_core-2fa0d4220368f1e9` | 7.584.168 | drei Coretests im Workspace-Log |
| `debug/build/schrott-mcp-ingestion/b0b8a99b0ee664a5/out/mkm_inventory` | 111.273.280 | Hardlink zum gleichnamigen `debug/examples`-Binary |
| `debug/build/schrott-mcp-ingestion/2c7245e3c60d2074/out/live_handlers` | 121.973.584 | Hardlink zum gleichnamigen `debug/examples`-Binary |

Die **außerhalb** des Targets liegenden, gezielt geprüften Original-Testlogs bleiben ebenfalls erhalten:

- `/tmp/opencode/sperling-targeted-live-tests.log`, 1.556 Bytes, SHA256 **`4b8e18bc32d481d6ca1fe72293a341b3ec7aa1d9ed858a552eb2f85355c36489`**: sechs Tests bestanden, ausdrücklich obiges historisches Testbinary.
- `/tmp/opencode/sperling-workspace-tests.log`, 60.408 Bytes, SHA256 **`e11407a485cabf63ba1ef6f12d4ae6d552e73e2716187cb84340e6b945961f41`**: nennt die tatsächlichen Workspace-Testbinarypfade und Ergebnisse, einschließlich drei Coretests.

Gezielte Suche `sperling-target|sperling-independent-target` in Repo-Research ergab nur den Sperling-Ownerbericht, den vorherigen Audit, dessen Ownerabschluss und die frühere Größenaufnahme. Keine dort gefundene Referenz verlangt die **Incremental-Coreobjekte** als Archiv; historische Testbinaries sind dagegen ausdrücklich in Original-Logs referenziert. Keine Behauptung einer vollständigen Suche über alle fremden Dateien/externen Archive. Die Sperling-Rohquellen unter `/tmp/opencode/sperling` und spätere Owner-/Independent-Reviewverzeichnisse sind **nicht Teil dieses Eingriffs** und wurden nicht rekursiv auditiert.

## Compiler, offene Dateien und verbleibende Grenzen

Lesende `/proc`-Aufnahme während des Audits:

- keine Prozesse mit `comm` Cargo, rustc, rust-analyzer, cc, ld, lld oder clang;
- für alle zugänglichen numerischen PIDs `cwd`, `exe`, `root`, FD-Symlinkziele und `maps` auf das **exakte Targetpräfix** geprüft: **null Halter/Mappingtreffer**;
- **null Permission-denied-PIDs**; kurzlebige verschwundene Prozesse wurden toleriert. Keine Environments/Credentials gelesen und keine Locks erworben oder verändert.

Diese Aufnahme ist kein Lock und keine zukünftige Exklusivitätsgarantie. Owner muss vor einem eventuellen eigenen Eingriff dieselben Halter-/Compilerbedingungen und unveränderte Dateimenge/Links erneut prüfen. Bei neuem Halter, unbekannter Datei, verändertem Linkstatus oder ungelöstem Artefaktbezug **keine Freigabe**. Danach `df -B1 /tmp` neu messen; keine Freisetzung wurde hier ausgeführt oder als erfolgt behauptet.

Bei ansonsten unverändertem tmpfs würde der minimale Kandidat von **101.380.096 auf 103.014.400 Bytes** frei führen, also nur von **96,68 auf 98,24 MiB**. Das entspricht dem ausdrücklich kleinsten vollständigen Eingriff, schafft aber **keine belastbare Buildreserve**. Größere Mengen nicht stillschweigend durch Ausbau auf das gesamte Incremental-/Targetverzeichnis erreichen. Historische Executables, Fingerprints/Diagnostik, Logs und einzigartige Quellen-/Reproevidenz bleiben erhalten; im Zweifelsfall ist fehlende sichere Freigabe der Blocker.
