# Weitere Rootreserve — begrenzter Audit, 18.10.2026, 23:20 UTC

## Ergebnis und konkreter Ownerumfang

**Zwei weitere alte Incremental-Unterbäume sind vollständig typ-/inhaltshashgeprüft und ergeben zusammen netto 904.052.736 Bytes (0,841965 GiB).** Bei der jüngsten Aufnahme um **23:18:38 UTC** waren **1.096.839.168 Bytes** auf `/` verfügbar; nach einem späteren eigenen Ownereingriff wären bei sonst gleichem Verbrauch **2.000.891.904 Bytes (1,86348 GiB)** verfügbar. Damit ist das Ziel **wiederhergestellte Gesamtreserve ≥1,5 GiB** konkret erreichbar. Diesmal werden nicht zusätzlich 1,5 GiB Freigabe behauptet.

**Empfohlener begrenzter Gesamtumfang, ausschließlich diese beiden vollständigen Pfade:**

| Exakter Pfad | Deduplizierte belegte Bytes | Netto freisetzbar |
|---|---:|---:|
| `/root/Documents/schrott-mcp/target/debug/incremental/schrott_mcp_ingestion-2yt8zpnsu9vr6` | 641.097.728 | **641.097.728** |
| `/root/Documents/schrott-mcp/target/debug/incremental/schrott_mcp_ingestion-24l2naq2rsqsm` | 276.893.696 | **262.955.008** |
| **Union** | **917.991.424** | **904.052.736** |

**Kein Cleanup ausgeführt, nichts gebaut und kein laufender Prozess gestört.** Der erste Pfad allein würde zum letzten Messzeitpunkt knapp reichen; der zweite schafft wegen des aktiven Fullsuite-Laufs zusätzliche Reserve statt eines nur scheinbar stabilen Momentgates. Keine Blanketfreigabe übriger Incrementals. Alle historischen Testexecutables, Quellen, Sourceexports, Deps-/Releaseoutputs, DBs, Reproarchive und Logs bleiben außerhalb des Eingriffs.

## Dynamischer Platz: frisches Gate zwingend

Der freie Rootplatz änderte sich während der laufenden Ownerarbeit deutlich:

| UTC am 18.10. | Verfügbare Rootbytes | Bedeutung |
|---|---:|---|
| 23:16:06 | 1.196.695.552 | erste `df -B1`-Aufnahme |
| 23:17:32, Beginn erster Inhaltsprüfung | 1.082.368.000 | laufende Suite weiterhin aktiv |
| 23:17:34 | 1.076.977.664 | nach erster Inhaltsprüfung |
| 23:18:16 | 1.075.879.936 | nach zweiter Inhaltsprüfung |
| 23:18:38 | **1.096.839.168** | jüngste Reserveplanung |

Das ist **nicht** als Dateiwachstum durch den Audit interpretiert: Dieser hat auf Root nur diesen Bericht als Schreibziel; ausführende Compiler/Testprozesse arbeiten parallel. Die beiden maschinenlesbaren Inventare liegen auf tmpfs. Keine Kapazitätsgarantie für den Zeitpunkt einer späteren Bereinigung.

**Owner-Mindestgate unmittelbar vor einem eigenen Eingriff:**

- Ziel **1,5 GiB = 1.610.612.736 Bytes**.
- Mit **beiden** Kandidaten muss frisch verfügbar sein: **mindestens 706.560.000 Bytes** (`1.610.612.736 − 904.052.736`).
- Mit **nur dem ersten** Kandidaten: mindestens **969.515.008 Bytes**.
- Bei weniger freiem Platz, veränderten Kandidaten oder neuen Haltern reicht dieser Nachweis **nicht**: konkrete neue Platz-/Halterprüfung nötig, kein breiteres Clean.
- Der vom Nutzer genannte Rootwatchdog **600 MiB** liegt **unter** dem obigen 706.560.000-Byte-Gate. Ein nicht ausgelöster Watchdog beweist deshalb noch nicht, dass die 1,5-GiB-Zielreserve nach dem Eingriff erreicht wird. Watchdog und laufende Suite wurden nicht umkonfiguriert.

`/tmp` hatte zu Beginn **88.489.984 Bytes** frei. Rootfreigabe erhöht diesen separaten tmpfs-Platz nicht. `/proc/meminfo` um 23:18:38: **MemAvailable 1.690.736 kB**, **SwapTotal/SwapFree 0**. Kein Swap-/RAM-/`/dev/shm`-Volumen als Release-/Backupreserve angerechnet.

## Umfang: alte Cleanups ausgeschlossen, aktuelle Arbeit geschützt

Zu Beginn nur `du -x -B1 --max-depth=1 target/debug/incremental` als Überblick: verbleibender Gesamtbaum **2.988.900.352 Bytes**, keine Freigabe dieses Gesamtbestands. Vertiefte Datei-/Inhaltsprüfung ausschließlich der beiden Kandidaten.

Explizit frisch als **nicht mehr vorhanden** geprüft und nicht erneut angefordert:

- der frühere 13:31-Pfad `schrott_mcp_ingestion-2rwavx3vuzdp7`;
- die vier bereits bereinigten 19:00-Pfade `2vtgyva6ztmwu`, `3opbkk6fdd0d1`, `2d2kx83fl5y84`, `1t8ze0ogvw0js` (jeweils mit Präfix `schrott_mcp_ingestion-`).

Die aktuellen Gold-/MKM-Sourceevidence- und Testquellarbeiten, der aktive Sourceexport `/root/Documents/checkin-20261018-2320-mkm-workspace-source` sowie insbesondere der jüngere Cache `schrott_mcp_ingestion-3h1aol3qhguxm` sind **keine** Kandidaten. Keine Dateien dort geöffnet oder editiert. Kein Zugriff auf Produktionsdatenbanken. Kein Cargo-/Compiler-/Testprozess durch diesen Audit gestartet oder beendet.

## Vollständige Kandidateninventur und SHA256

Beide Kandidaten mit `lstat` und nicht symlinkfolgender Inventur geprüft; Wurzel und Kandidaten sind keine Symlinks. **Jede reguläre Datei** wurde typgeprüft und gestreamt gehasht, maximal **65.536 Bytes pro Lesechunk**, kein großer Inhaltsbuffer und kein Hashen fremder Source-/DB-/Releasebäume. Identische Inodes nur einmal inhaltlich gelesen; alle Pfadnamen dennoch einzeln inventarisiert. Vor/nach dem Scan Dateimenge und Statidentität inklusive mtime/ctime verglichen: **unverändert**. Erster Kandidat nach Prüfung des zweiten nochmals statbasiert unverändert.

| Inventur | `2yt8zpnsu9vr6` | `24l2naq2rsqsm` |
|---|---:|---:|
| Reguläre Dateien | 520 | 261 |
| Tatsächlich geprüfte ELF **ET_REL=1** `.o` | 512 | 256 |
| `dep-graph.bin` / `query-cache.bin` / `work-products.bin` | je 2 | je 1 |
| `.rmeta` mit tatsächlichem `rust`-Header | 0 | 1 |
| Leere Session-`.lock` | 2 | 1 |
| Verzeichnisse einschließlich Kandidatenwurzel | 3 | 2 |
| Unbekannte Typen / Symlinks / Spezialdateien | **0** | **0** |
| Quellen / Logs / DBrepros / ET_EXEC-/ET_DYN-Testexecutables | **0** | **0** |

Alle neun `.bin` tragen den gelesenen **`RSIC`**-Header mit Rustversionsmetadaten. Kein Endungs-/Ordnernamenbeweis allein. Die `.o` sind relocatable Compilerobjekte, **keine archivierten ausführbaren Testergebnisse**. Dass Objekte den damals kompilierten Testcode enthalten können, macht sie nicht zu einer Testquelldatei oder einem Testlog.

### Maschineninventare — unveränderte neue Evidenz, kein Löschscript

Beide neu exklusiv angelegt, Mode **0600**, keine vorhandene Datei überschrieben. Enthalten pro Pfad Typ, Device/Inode, Modus, logische/belegte Bytes, nlink, mtime/ctime, Header und **vollständigen Datei-SHA256**; zusätzlich Linkgruppen, Nettorechnung, Vor-/Nach-Halteraufnahmen und Platzwerte.

1. **`/tmp/opencode/checkin-20261018-2320-build-reserve-inventory.json`**
   - 278.351 Bytes, Aufnahme **23:17:32.545093–23:17:34.342337 UTC**;
   - Datei-SHA256 **`529847c48bad139e322d4fbd2982e47a944926c5f64896ac97ef2f6fd21255e4`**;
   - kanonischer `entries`-SHA256 **`b3f98c90904d2ec6304c6d6dcc11de2ce57163232c43c430a33fc8820964f1ee`**.
2. **`/tmp/opencode/checkin-20261018-2320-build-reserve-inventory-extra.json`**
   - 134.928 Bytes, Aufnahme **23:18:15.735547–23:18:16.577367 UTC**;
   - Datei-SHA256 **`a4a14216c7671c1e76fdb32b0c8feca6bf0cba337d1db00ad530058d9a670b08`**;
   - kanonischer `entries`-SHA256 **`b191cbf7c6289619a62e4a6d201ab0bfc9d7a85e6574f261ba01d3dbf596b2e5`**;
   - bindet außerdem erstes Inventar samt Dateihash, seine unveränderte Metadaten-Nachprüfung und die kombinierte Rechnung.

`entries`-Hashschema: UTF-8-JSON mit `sort_keys=True, separators=(',', ':')` über das jeweilige vollständige `entries`-Array. Beide Hashes anschließend erneut validiert. Diese Inventare sind neue Auditevidenz auf tmpfs, nicht Bestandteil der freizugebenden Caches; **erhalten**, nicht als späteren Cleanupkandidaten behandeln.

## Hardlinks und exakte Nettorechnung

### `2yt8zpnsu9vr6`: interne Hardlinks kosten keinen Nettogewinn

- Zwei Sessions, **520 Pfadnamen, 488 reguläre Inodes**.
- **32 Objekt-Inodes** mit je zwei Links; **sämtliche Links liegen innerhalb genau dieses Kandidaten**. Die genauen beiden Pfade je Inode stehen vollständig im ersten Inventar.
- Kein regulärer Inode mit unbekanntem/externem verbleibendem Link.
- Verzeichnisblöcke **53.248 Bytes**, pro Inode deduplizierte Gesamtblöcke **641.097.728 Bytes**. Deshalb sind exakt diese Bytes bei Entfernung beider Sessions freisetzbar; nicht jede Session einzeln mit dem Gesamtgewinn verwechseln.

### `24l2naq2rsqsm`: externe `.rmeta` vollständig abgezogen

- Session **`s-hn94hctkit-1b75poq-82p66q7j0vjds6917qx804bkq`**, 261 reguläre Dateien.
- Einziger externer Link: deren **`metadata.rmeta`**, Device **2049**, Inode **1168265**, nlink **2**, nur **ein** Link innerhalb des Kandidaten, **13.938.688 belegte Bytes**.
- Zweiter Link außerhalb des Kandidaten **nicht entfernt**. Kein Scan durch `build/deps` nötig; dieser Blockanteil wird vollständig abgezogen.
- **276.893.696 − 13.938.688 = 262.955.008 Bytes** netto, inklusive **20.480 Bytes Verzeichnisblöcken**.

Zwischen den beiden Kandidaten **null gemeinsame reguläre Inodes**, frisch durch Inventarvergleich geprüft. Kombinierter Gewinn daher ohne Doppelzählung **904.052.736 Bytes**. Kein behauptetes bytegenau gleiches späteres `df`-Delta während der gleichzeitig laufenden Suite.

## Quellen-/Historienbezug und ehrliche Reproduktionsgrenze

### Alter stabiler Main-Compilerzustand: `2yt8zpnsu9vr6`

Datei-mtime gesamt **14.10.2026 00:08:00.173904–18:09:45.760581 UTC**. In jeder Session ein tatsächlich vorhandenes Objekt mit `readelf --debug-dump=info` untersucht:

| Session | Gelesenes Objekt |
|---|---|
| `s-hn6s95v787-1luo43l-dbct87pwex1cqlknt5ab4lax2` | `9dxaanzwm7gf66s1m9hwt4n8r.o` |
| `s-hn7m2c0q8h-19n2nvf-b57gkje9rsc1tbeyodzwai29s` | `ei4vjdd55pzzbt6c89xff2wxo.o` |

Beide nennen Compilation Unit **`crates/ingestion/src/lib.rs`**, Compdir **`/root/Documents/schrott-mcp`**, Producer **rustc 1.98.1 (48a229cea 2026-09-01)**. Frühere Größen-/Typinventur dieses exakten Pfads ist in `research/checkin-20261018-1331-disk-audit.md` dokumentiert; die aktuelle Prüfung ersetzt deren damalige bloße Reserveplanung durch vollständige Datei-/Inhalts-/Linkprüfung.

**Kein exakter damaliger Main-Commit aus mtime geraten.** Die ursprüngliche Main-Zwischenfassung wird hier nicht als vollständig rekonstruiert behauptet. Freigegeben wird gerade **nicht** ein historisches Testbinary oder dessen Quellen, sondern ausschließlich nachgewiesener Rust-Incrementalzustand ohne Quellen-/Log-/Archivdateien. Ein Neucompile kann für seinen vorhandenen Sourceinput neuen Cache erzeugen; historische byteidentische Outputs oder identisches Timing werden nicht zugesichert. Originalquellen, Gitobjekte, Testbinaries und Reproevidenz bleiben unverändert erhalten. Ist für den Owner zusätzlich die Bewahrung exakt dieser Compiler-Incrementalzustände als Archivziel erforderlich, ist dieser erste Pfad **nicht** freizugeben; der zweite allein würde die Zielreserve derzeit nicht herstellen.

### Gebundener sauberer Releaseexport: `24l2naq2rsqsm`

Datei-mtime **16.10.2026 03:03:45.300577–03:04:02.686814 UTC**. Objekt **`d1f4e6l1xkdmye25kimt5n389.o`** in obiger Session nennt:

- Compilation Unit **`crates/ingestion/src/lib.rs`**;
- Compdir **`/root/Documents/checkin-20261016-0300-release-1wz2obd5`**;
- Producer **rustc 1.100.0-nightly (5ceaf6608 2026-09-25)**.

`research/checkin-20261016-0300-verifier-review.md:15` dokumentiert für exakt diesen Export den damaligen Vergleich aller **4.400 versionierten Dateien** gegen **`81a6483208ade84c592e89a8684f56c9774dcea7`**, null Abweichungen. Lokale Existenz dieses Commits und seines Ingestion-Quellblobs frisch lesend mit `git cat-file -e` bestätigt. `research/checkin-20261016-0300-owner.md` bindet die damaligen Workspace-/Releasebelege an diesen Export. Kein neuer Sourcearchiv-Vollvergleich, keine alten Tests erneut ausgeführt. Die Originalarchive und damaligen Test-/Releaseartefakte sind ausdrücklich **kein** Entfernungsumfang.

## Aktive Fullsuite: beobachten, nicht stören

Um **23:16:55 UTC** tatsächlich beobachtet: **Cargo PID 1233676 und rustc PID 1234031**, cwd jeweils `/root/Documents/checkin-20261018-2320-mkm-workspace-source`. Während beider Inhaltsprüfungen war weiterhin **Cargo PID 1233676** beobachtbar. Das temporäre Nichtauftauchen von rustc in einzelnen Aufnahmen ist **kein** Nachweis eines beendeten Ownerlaufs.

Vor/nach beiden Scans alle zugänglichen numerischen `/proc`-PIDs auf `cwd`, `exe`, `root`, FD-Symlinkziele und `maps` mit dem exakten Root-Incrementalpräfix geprüft:

- **Null Halter/Mappingtreffer** im Incremental-Baum, damit auch null auf beiden Kandidaten.
- **Null Permission-denied-PIDs**; verschwundene kurzlebige Prozesse toleriert.
- Kein Prozesssignal, keine Pause, kein Kill, kein Lockmanöver, kein Lesen von Secrets oder Environments.
- `CARGO_INCREMENTAL=0`, ein Compilerjob, zwei Testthreads und Watchdoggrenzen sind die **vom Nutzer mitgeteilte Laufkonfiguration**, nicht durch Eingriff in den Ownerprozess geändert. Es wird **nicht** behauptet, dass generell kein Compiler aktiv sei.

Owner unmittelbar vor dem späteren Eingriff Dateimenge/Typen/SHA/Linkgruppen und Halter **erneut** prüfen. Bei einem neuen Halter, unerwarteter Datei oder verändertem Source-/Archivzweck keine Freigabe. Diese Aufnahme ist keine Exklusivitätsgarantie während einer Fullsuite.

## Abschlussgrenzen

Der begrenzte Zweierumfang kann die Root-Gesamtreserve auf **≥1,5 GiB** bringen, solange das frische **706.560.000-Byte-Gate** eingehalten wird. Damit liegt ein praktischer Platzvorschlag statt eines symbolischen Kleinstcaches vor. **Keine tatsächliche Reservewiederherstellung behauptet**, solange Owner nicht selbst begrenzt bereinigt und `df` gemessen hat.

Keine Freigabe eines Releasebuilds allein durch diesen Audit: Fullsuite-Ergebnis, Compiler-/Linkerpeak, isolierter endgültiger Input, tmpfs/RAM und sichere Backup-/Deploymentgates bleiben unabhängig. Keine weiteren Dateien außer **diesem Bericht und den beiden ausdrücklich angeforderten neuen Maschineninventaren** geschrieben. Insbesondere keine parallele Gold-/MKM-Sourceevidence verändert.
