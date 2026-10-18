# Root-Buildreserve: begrenzter Incremental-Audit — 18.10.2026, 19:00 UTC

## Ergebnis: konkret mehr als 1,5 GiB zusätzlich belegbar

**Vier exakt benannte, vollständig typgeprüfte Compiler-Incrementals ergeben netto 1.619.021.824 Bytes = 1,5078316 GiB zusätzlichen freisetzbaren Platz.** Das überschreitet sogar die strengere Auslegung „1,5 GiB neu freigeben“, nicht nur „einschließlich bisher freiem Platz 1,5 GiB erreichen“. Bei unverändertem sonstigem Verbrauch wären danach auf `/` **2.175.229.952 Bytes = 2,02584 GiB verfügbar**.

**Owner-Kandidaten, ausschließlich diese vier vollständigen Unterbäume:**

| Exakter Pfad | Belegte Bytes, Inodes je Pfad dedupliziert | Allein freisetzbare Bytes |
|---|---:|---:|
| `/root/Documents/schrott-mcp/target/debug/incremental/schrott_mcp_ingestion-2vtgyva6ztmwu` | 672.370.688 | **672.370.688** |
| `/root/Documents/schrott-mcp/target/debug/incremental/schrott_mcp_ingestion-3opbkk6fdd0d1` | 342.200.320 | **342.200.320** |
| `/root/Documents/schrott-mcp/target/debug/incremental/schrott_mcp_ingestion-2d2kx83fl5y84` | 340.025.344 | **340.025.344** |
| `/root/Documents/schrott-mcp/target/debug/incremental/schrott_mcp_ingestion-1t8ze0ogvw0js` | 278.507.520 | **264.425.472** |
| **Union, ohne doppelt gezählte Inodes** | **1.633.103.872** | **1.619.021.824** |

**Keine Löschung ausgeführt.** Empfehlung nur für den späteren begrenzten Ownereingriff nach frischer Typ-/Link-/Halterprüfung. Keine Wildcard-, `cargo clean`- oder Gesamt-Targetfreigabe. Die ursprünglichen Quellen/Exports, generierte Seedpayloads, historischen Testexecutables, Cargo-Fingerprints, Repros, Manifeste und Logs bleiben unverändert. Insbesondere **kein Eingriff in `target/debug/build`, `target/debug/deps`, `target/debug/examples` oder `target/release`**.

## Ausgangsplatz und Grenzen des Auftrags

Aufnahme `2026-10-18T19:01:18Z`, `df -B1`:

| Dateisystem | Gesamt | Belegt | Verfügbar |
|---|---:|---:|---:|
| `/` | 80.279.486.464 | 76.429.426.688 | **556.208.128 Bytes (530,4414 MiB)** |
| `/tmp`, separates tmpfs | 4.061.138.944 | 3.964.694.528 | **96.444.416 Bytes (91,9766 MiB)** |

Die gemessenen **556.208.128 Bytes** entsprechen etwa **556 MB**, nicht 556 MiB. `du -x -B1 --max-depth=1 /root/Documents/schrott-mcp/target/debug/incremental` ermittelte **4.622.004.224 Bytes** für den gesamten angefragten Incremental-Baum. Dieser Gesamtwert ist **keine** Freigabe.

Der bereits um 13:31 geprüfte und danach entfernte `schrott_mcp_ingestion-2rwavx3vuzdp7` ist **nicht mehr vorhanden**, nicht Teil der Rechnung und kein erneuter Kandidat. Auch der Tmpaudit wird nicht wiederholt. Keine DB-, Produktiv-, RAM- oder `/dev/shm`-Kapazität als neues Backupvolume angesetzt. RAM und tmpfs/shm sind geteilte Ressourcen; dieser Audit bewertet ausschließlich belegte Rootdateiblöcke.

Rekursive Dateilesungen erfolgten ausschließlich im angefragten Incremental-Baum. Ergänzend wurden gezielte bereits vorhandene Repo-Researchberichte, lokale Gitobjekt-Metadaten zur Quellenzuordnung und `/proc`-Haltermetadaten gelesen. **Keine** fremden Build-/Deps-/Releasebäume, Quellarchive oder Reprodatenbanken rekursiv geöffnet. Keine Sourceänderung, kein Cargo/Compilerlauf, kein Test, keine Secrets/Environments gelesen. Einziger eigener Schreibpfad: dieser Bericht.

## Vollständige Typprüfung statt Namensvermutung

Der gesamte vorhandene Incremental-Baum wurde per `lstat`/`os.walk(followlinks=False)` inventarisiert. Jede vorhandene `.o` wurde am 20-Byte-ELF-Header geprüft: **ELF ET_REL=1**. In keinem Unterbaum trat eine abweichende Objektdatei oder ein unerwarteter Dateityp auf. Gefunden wurden ausschließlich `.o`, die bekannten Rust-Cachedateien `dep-graph.bin`, `query-cache.bin`, `work-products.bin`, `.rmeta` und leere Session-`.lock`-Dateien. **Keine Quellen, Logs, DBrepros, Symlinks oder Spezialdateien** im inventarisierten Incremental-Inhalt; keine ET_EXEC-/ET_DYN-Testexecutables.

Für die vier vorgeschlagenen Pfade zusätzlich jede Datei erneut geprüft:

- zusammen **1.301 reguläre Dateien**: **1.280 ET_REL-Objekte**, **15 `.bin`**, **eine `.rmeta`**, **fünf leere `.lock`**;
- alle `.bin` tragen im gelesenen Header **`RSIC`**, gefolgt von Rustversionsmetadaten; keine bloß nach Endung als Cache eingestuften Dateien;
- die einzige `.rmeta` beginnt mit **`rust`**, Header hex `727573740000000a08d9d600`;
- null Symlinks und null unbekannte Dateien in den vier Kandidaten;
- jeder Kandidat ist Compilerzustand, **kein archiviertes Testexecutable und keine nur dort abgelegte Testquelle**. Compilerobjekte können natürlich den damals kompilierten Testcode enthalten; sie sind nicht dessen Quell-/Ergebnisarchiv.

Die übrigen Incremental-Unterbäume wurden zwar typinventarisiert, aber **nicht** pauschal zur Entfernung empfohlen. Der seit 18:12 frisch geänderte `schrott_mcp_ingestion-3h1aol3qhguxm` ist insbesondere **nicht** Teil der Auswahl. Die angekündigten Goldtrans-/MKM-Diagnose- und Testquellarbeiten werden nicht verändert oder als Freigabe ihrer Artefakte interpretiert.

## Harte Linkrechnung und Alter

Rechnung pro Inode `(st_dev, st_ino)`, nicht je Dateiname: `st_blocks × 512` nur einmal summiert. Freisetzbar nur, wenn die Zahl aller Links innerhalb der vorgesehenen Entfernung dem `st_nlink` entspricht. Verzeichnisblöcke mitgerechnet. Die Union der vier Kandidaten wurde separat dedupliziert und auf Links geprüft.

| Kandidat, Suffix nach `schrott_mcp_ingestion-` | Datei-mtime UTC | Linkbefund |
|---|---|---|
| `2vtgyva6ztmwu` | 18.10. 12:15:34.354818–12:35:25.218294 | alle regulären Dateien nlink=1 |
| `3opbkk6fdd0d1` | 16.10. 03:02:07.270244–03:02:42.964729 | alle regulären Dateien nlink=1 |
| `2d2kx83fl5y84` | 16.10. 07:35:27.611984–07:35:52.630324 | alle regulären Dateien nlink=1 |
| `1t8ze0ogvw0js` | 16.10. 03:02:06.213229–03:02:32.079581 | eine `.rmeta` nlink=2, alle anderen regulären Dateien nlink=1 |

Einziger Abzug:

`schrott_mcp_ingestion-1t8ze0ogvw0js/s-hn94fptsr8-1de4we6-3ukw2e7vgyi3ulfbcv0vqk6d8/metadata.rmeta`

- Inode **1168213**, nlink **2**, **14.082.048 belegte Bytes**.
- Nur ein Link befindet sich im empfohlenen Viererumfang. Der zweite liegt außerhalb dieses Umfangs und wird **nicht** gesucht, geöffnet oder angefasst: keine Ausweitung auf `build/deps` erforderlich.
- Deshalb die gesamten **14.082.048 Bytes abgezogen**; kein behaupteter Gewinn aus dem erhaltenen zweiten Link.

mtime ist Altersbeleg, kein alleiniger Löschgrund. Die freisetzbaren Bytes sind eine exakte Blockrechnung der Aufnahme, nicht die Zusage desselben späteren globalen `df`-Deltas unter Parallelwachstum.

## Quellenbezug je Kandidat, einschließlich historischer Referenzen

Für **jede der fünf Sessions** wurde ein tatsächliches ET_REL-Objekt mit `readelf --debug-dump=info` gelesen. Alle nennen `crates/ingestion/src/lib.rs` als Compilation Unit. Die Root-Cargo-Fingerprints liegen außerhalb des erlaubten Incremental-Scopes und wurden **nicht** zusätzlich gelesen. Statt erfundener Fingerprintbindung werden hier die konkreten DWARF-Quellpfade und unabhängigen historischen Researchbindungen angegeben.

### 1. `2vtgyva6ztmwu`: zwei ausdrücklich dokumentierte isolierte Seedinputs

| Session | Gelesenes Objekt | `DW_AT_comp_dir` |
|---|---|---|
| `s-hnbr5wu0tb-19sb4aa-az6nonj3x4t8k857q264fvoun` | `5u3s3gvahtrqld2wph4ylpsde.o` | `/root/Documents/wave-20261018-1202-scoped-seed` |
| `s-hnbrm0flza-07he2y4-65ewon4x5hoiunigzzhp4csup` | `5u3s3gvahtrqld2wph4ylpsde.o` | `/root/Documents/checkin-20261018-1230-scoped-seed` |

Producer beider Sessions: **rustc 1.98.1 (48a229cea 2026-09-01)**.

- `research/wave-20261018-1202-owner.md:49` bindet den ersten Input an **`d6498036694bb9d12ffa42438efce71d15b557a5` plus genau 60 finale Dossiers**, 14 Seedtests. Erhaltene Evidenzpfade: `/root/Documents/wave-20261018-1202-scoped-seed.log`, `/tmp/opencode/wave-20261018-1202-seed-result.json` und `...-compiled-notes-audit.json`.
- `research/checkin-20261018-1230-owner.md:44` bindet den zweiten Input an **`789f792dc1cb54b4b6ec6f55942a91e3bd6ca705` plus zwei finale Dossiers**, 14 Seedtests. Original-Log `/root/Documents/checkin-20261018-1230-scoped-seed.log`, Resultat `/tmp/opencode/checkin-20261018-1230-seed-result.json`.
- Der frühere `research/checkin-20261018-1331-disk-audit.md` dokumentiert zusätzlich die damalige vollständige Input-/Overlay-/Loghashprüfung dieses 12:30-Exports und dessen historische Marienbergfassung. **Keine neue Quellenhashprüfung oder Testwiederholung hier behauptet.**

Die Originalexporte, Dossierinputs, Payloads und sämtliche zugehörigen Logs/Manifeste bleiben erhalten. Entfernt würde ausschließlich der zusätzliche Compilerzustand zu diesen dokumentierten Inputs.

### 2. `3opbkk6fdd0d1`: historisches Workspace-Test-Incremental des sauberen Releaseexports

Session **`s-hn94fqggfr-1fsp3q8-eq9m1xrb3ui4tdyyxsi8drkua`**, Objekt **`4pmr6oa4r78tsln1t4j95bycc.o`**:

- Compdir **`/root/Documents/checkin-20261016-0300-release-1wz2obd5`**;
- Producer **rustc 1.100.0-nightly (5ceaf6608 2026-09-25)**;
- `research/checkin-20261016-0300-owner.md:58` dokumentiert für genau diesen Export u. a. 726 bestandene Workspace-Tests und einen Releasebuild. **Diese Ergebnisse werden hier nur als historische Quellenzuordnung zitiert.**
- `research/checkin-20261016-0300-verifier-review.md:15` dokumentiert Vergleich aller **4.400 versionierten Exportdateien** mit **`81a6483208ade84c592e89a8684f56c9774dcea7`**, null Abweichungen. Lokale Existenz dieses Commits und seines `crates/ingestion/src/lib.rs`-Blobs frisch mit Git geprüft.

Das historische Testbinary, die Releaseartefakte und deren Logs bleiben ausdrücklich erhalten; ein ET_REL-Incremental ist keines dieser ausführbaren Ergebnisartefakte.

### 3. `2d2kx83fl5y84`: begrenzter Kontakt-Seedtest im separaten Export

Session **`s-hn9byysp3g-0lhyv5z-4f53wi4lsdkj8684rmtoqhktv`**, Objekt **`3z7jh8ukwaid3ad1hbl9i32sy.o`**:

- Compdir **`/root/Documents/checkin-20261016-contact-a-bzgybe2u`**;
- Producer dieselbe **1.100.0-nightly**;
- `research/checkin-20261016-0730-owner.md:20` nennt genau diesen **sauberen Export von `03071ea6` plus vier Dossieroverlays**, 14 bestandene echte Seedtests, unveränderten Codegenerator und die erhaltenen Belege **`owner-gate.json`, `seed-test.log`, `codegen-diff.json`**.
- Commit vollständig lokal aufgelöst: **`03071ea6c9a91b1d2ed280df02c6daa4e8a238c0`**, Ingestion-Quellblob vorhanden. Keine nur im Cache befindliche Source-/Reprodatei gefunden.

Auch hier bleiben Quell-/Dossierexport, Testexecutable, Generatorausgaben und Evidenz unangetastet.

### 4. `1t8ze0ogvw0js`: Bibliotheks-Incremental desselben gebundenen Releaseexports

Session **`s-hn94fptsr8-1de4we6-3ukw2e7vgyi3ulfbcv0vqk6d8`**, Objekt **`3ik75g697vkz6frkv25twhbkf.o`**:

- Compdir ebenfalls **`/root/Documents/checkin-20261016-0300-release-1wz2obd5`**;
- Producer dieselbe **1.100.0-nightly**;
- Quellenbindung an den unveränderten archivierten **`81a64832`**-Input wie unter 2, aber eigener Bibliothekscompilerzustand mit einer hardverlinkten `.rmeta`.

Der außerhalb des Entfernungsumfangs existierende `.rmeta`-Link bleibt erhalten; sein Blockvolumen ist bereits vollständig abgezogen. Keine Bibliotheks-/Testbinarydatei unter `build` oder `deps` Teil dieser Empfehlung.

### Aussagegrenze zur Reproduktion

DWARF + dokumentierter isolierter Input + lokale Gitobjekte belegen den **konkreten Quellenbezug und den generierten Cachecharakter**. Sie sind kein neuer bytegenauer Vollvergleich der ursprünglichen Quellarchive und keine Zusage, dass ein heutiger Neucompile mit anderer Toolchain dieselben Bytes erzeugt. Für die Platzfreigabe wird **kein** historisches Testexecutable als neu reproduziert ausgegeben oder entfernt. Die originalen Repro-/Source-/Ergebnisartefakte müssen gerade nicht durch diesen Audit ersetzt werden.

## Halter und frische Owner-Gates

Während des Audits alle zugänglichen numerischen `/proc`-PIDs geprüft: `comm`, `cwd`, `exe`, `root`, FD-Symlinkziele und `maps`, jeweils auf das exakte Incrementalpräfix.

- **Keine Cargo-/rustc-/rust-analyzer-/cc-/ld-/lld-/clang-Prozesse beobachtet**.
- **Null FD-/cwd-/Executable-/Mappinghalter** innerhalb des geprüften Incremental-Baums.
- **Null Permission-denied-PIDs**; verschwundene kurzlebige Prozesse toleriert. Keine Prozesse gestoppt und keine Locks verändert.

Das ist eine Aufnahme, keine Exklusivität. Owner vor eigenem Eingriff:

1. Exakt dieselben vier Pfade sowie unveränderte Typ-/Linkinventur erneut prüfen; keine parallel neu gebauten Sessions mitnehmen.
2. Compiler/FD-/Mappinghalter erneut ausschließen, einschließlich etwaiger inzwischen gestarteter Goldtrans-/MKM-Läufe. Bei Unklarheit **kein Eingriff**.
3. Nur den begrenzten bestätigten Umfang freigeben, nie die übrigen Incrementals oder `build/deps/release`. Unter keinen Umständen die archivierten Originalinputs/Logs/Reprodaten mitentfernen.
4. Danach Root-`df` messen und reale Freigabe dokumentieren. `CARGO_INCREMENTAL=0` beim später gesondert genehmigten Build kann Neuzuwachs begrenzen, verhindert aber weder Codegen-/Linkerpeak noch neue Test-/Releasebinaries.

Inventar-SHA256 zur Erkennung von Abweichungen, **keine Vollinhaltshashes**: je Kandidat alle Pfade inklusive Wurzel sortiert; Zeile `[relpath, st_dev, st_ino, st_mode, st_size, st_blocks, st_nlink, st_mtime_ns]`; JSON `separators=(',', ':')`, SHA256:

| Kandidat | Inventar-SHA256 |
|---|---|
| `2vtgyva6ztmwu` | `9f5514571c36b258eaf6fb9c66e61725703f3a27a4860663cef2bbddaa72e8dc` |
| `3opbkk6fdd0d1` | `9c52590ec5c35aa8a47e1eb227a8e13dde208c8e93052f929652007b094f2823` |
| `2d2kx83fl5y84` | `a79535619e4d75e2b52f1554521e92b290af4499d1bf8a6a7d834c98616f45c7` |
| `1t8ze0ogvw0js` | `849c77bef6bf9dc54f8fd923a9d87fb3a775f7494f6b3601b41dc68afd1327c8` |

## Reserveplanung, keine Betriebsfreigabe

**1,5 GiB = 1.610.612.736 Bytes**. Die vier geprüften Caches liegen netto **8.409.088 Bytes darüber**. Für die Gesamtreserve ergibt die Aufnahme **2,02584 GiB**, also rund **0,526 GiB oberhalb** der verlangten 1,5-GiB-Reserve. Parallelwachstum und neue Buildoutputs können diese Reserve aufzehren; bei frischer Prüfung neu rechnen, nicht die alten Zahlen als Istzustand behandeln.

Das konkrete Root-Platzziel ist mit diesem begrenzten Umfang **belegbar**, nicht länger nur ein symbolischer 1-MiB-Kandidat. **Kein Build-/Backup-/Rollout-Go:** `/tmp` bleibt separat eng; der kommende Build braucht sein eigenes Peak-/TMPDIR-/Artefaktbudget, und sichere Produktionssicherung/Writer-/Rollback-/fachliche Gates sind andere Aufgaben. RAMavailable oder freier `/dev/shm`-Adressraum ersetzen weder persistente Rootreserve noch ein genehmigtes Backupvolume. In diesem Audit wurde nichts gebaut, gelöscht, committed, deployed oder an einer DB geöffnet.
