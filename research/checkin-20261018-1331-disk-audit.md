# Plattenaudit 18.10.2026, 13:31–13:34 UTC — nur Befund, keine Löschung

## Ergebnis / Owner-Entscheidung

**Der große eindeutig generierte Bestand ist der gemeinsame Cargo-Cache:** `/root/Documents/schrott-mcp/target` belegt **13.710.766.080 Bytes (12,77 GiB)**, davon **4.889.407.488 Bytes Incremental-Cache**. Die 14 angefragten Owner-Seedexporte zusammen belegen dagegen nur **422.707.200 Bytes (403,13 MiB)** und enthalten kleine, zu erhaltende Testinputmanifeste. Keine pauschale Export-/Target-/tmp-Bereinigung empfehlen.

**Kleinster hier konkret untersuchter, sinnvoller Notfallkandidat:** ausschließlich der alte Rust-Incremental-Unterbaum

`/root/Documents/schrott-mcp/target/debug/incremental/schrott_mcp_ingestion-2rwavx3vuzdp7`

Er belegt **262.959.104 Bytes (250,78 MiB)**, ausschließlich 256 `.o`, eine `.rmeta`, drei `.bin` und eine `.lock` (keine Symlinks). Objektdateien stammen vom 12.10.; jüngster mtime im Unterbaum ebenfalls 12.10. Dieser Unterbaum ist nur rekonstruierbarer Compilerzustand, kein Quellen-/Testergebnisarchiv. **Achtung Hardlink:** die `.rmeta` hat zwei Links, ihr Blockanteil von **12.869.632 Bytes** wird beim Entfernen dieses einen Unterbaums voraussichtlich nicht frei. Konservativer Gewinn daher etwa **250.089.472 Bytes (238,50 MiB)** einschließlich Verzeichnisblöcken, nicht die vollen 250,78 MiB. Erwartet danach nur etwa **564 MiB auf `/`** frei, sofern sonst nichts wächst: Notfallluft, ausdrücklich **kein Release-/Backup-Go**.

Dies ist eine **Freigabeempfehlung an den Owner, keine ausgeführte Bereinigung**. Vor einem Eingriff erneut laufende Cargo-/Rustc-Arbeit ausschließen, Eigentümerschaft/Abhängigkeiten bestätigen und unmittelbar danach `df` messen. Bei der Prozessaufnahme dieses Audits kein `cargo` oder `rustc`; das ist keine dauerhafte Exklusivitätsgarantie. Ohne Zustimmung/Exklusivität bleibt Platz ein Blocker. Weitere Eingriffe sind für diese Notfallentlastung nicht nötig.

## Gemessener Platz

Alle Größen mit `du -x -B1` sind belegte Blöcke, keine aufsummierten logischen Dateigrößen. GNU `du` zählt Hardlinks innerhalb eines Aufrufs nur einmal; Untertabellen nicht zu bereits enthaltenen Eltern addieren. Größen sind Momentaufnahmen eines weiterlaufenden Systems.

| `df -B1` | Dateisystemgröße | Belegt | Verfügbar | Anzeige |
|---|---:|---:|---:|---:|
| `/`, erste Aufnahme | 80.279.486.464 | 76.643.618.816 | **342.016.000 (326,17 MiB)** | 100 % |
| `/`, Schlussaufnahme | 80.279.486.464 | 76.643.926.016 | **341.708.800 (325,88 MiB)** | 100 % |
| `/tmp`, erste Aufnahme, eigenes tmpfs | 4.061.138.944 | 3.959.697.408 | **101.441.536 (96,74 MiB)** | 98 % |
| `/tmp`, Schlussaufnahme | 4.061.138.944 | 3.959.750.656 | **101.388.288 (96,69 MiB)** | 98 % |

`/tmp` ist **nicht** dieselbe Kapazität wie `/`: Rootcache-Freigabe vergrößert nicht den freien tmpfs-Platz. Die Differenz zwischen Dateisystemgröße und Used+Available ist nicht als frei nutzbare Reserve anzusetzen.

| Exakter Pfad unter `/root/Documents/schrott-mcp/` | Belegte Bytes | Einordnung |
|---|---:|---|
| `target` | 13.710.766.080 | gemeinsamer Buildcache, kein versionierter Quellbaum |
| `target/debug` | 11.721.949.184 | enthält folgende fünf Teilbestände |
| `target/debug/incremental` | 4.889.407.488 | regenerierbarer Rust-Compilerzustand |
| `target/debug/build` | 4.580.392.960 | nicht nur Buildskripte: auch Cargo-Artefaktablage |
| `target/debug/deps` | 1.824.374.784 | Bibliotheken/Testexecutables, teilweise Hardlinks |
| `target/debug/examples` | 421.584.896 | gebaute Beispiele, keine Testquellen |
| `target/debug/.fingerprint` | 6.184.960 | Cargo-Metadaten |
| `target/release` | 1.988.800.512 | Releasecache, **nicht** pauschal freigeben |
| `target/release/build` | 1.207.025.664 | Teil des Releasebestands |

`git ls-files target` liefert keine Dateien; `git check-ignore target/debug/incremental target/debug/examples` bestätigt beide Ignoreregeln. Ignoriert allein wäre kein Löschbeweis; für den empfohlenen Einzelunterbaum wurde zusätzlich die vollständige Dateitypen-/Linkinventur durchgeführt.

Beachtlich: `target/debug/build/schrott-mcp-ingestion` allein **2.998.214.656 Bytes**; darunter z. B. `bf1ad43e3403c8be/out/schrott_mcp_ingestion-bf1ad43e3403c8be` mit **153.262.080 logischen Bytes** und `6041257bb44dffc1/out/schrott_mcp_ingestion-6041257bb44dffc1` mit **152.501.776 logischen Bytes**, jeweils Testexecutables neben Fingerprints/Depfiles. Der ungewöhnliche Aufbau ist kein zusätzlicher Quellbaum. Diese größeren Artefakte wurden hier **nicht** einzeln auf alle Referenzen/ersetzte Testquellen auditiert und sind nicht der vorgeschlagene erste Eingriff.

Weitere exakt vermessene Incremental-Bestände als Reserveplanung, **nicht zusätzliche Löschaufträge**:

| Pfad unter `target/debug/incremental/` | Belegte Bytes |
|---|---:|
| `schrott_mcp_ingestion-2vtgyva6ztmwu` | 672.370.688 |
| `schrott_mcp_ingestion-2yt8zpnsu9vr6` | 641.097.728 |
| `schrott_mcp_ingestion-3h1aol3qhguxm` | 344.637.440 |

Der erste dieser Reservepfade enthält noch Objekte vom 18.10., 12:35; daher den älteren kleineren Einzelkandidaten vorziehen. Cacheabbau kann den nächsten Compile verteuern. `CARGO_INCREMENTAL=0` vermeidet neuen Incremental-Zuwachs, beseitigt aber weder vorhandene Caches noch neue Executables/Codegen-/Linkerdateien.

## Die 14 gezielt geprüften Owner-Exporte

Alle folgenden Pfade liegen unmittelbar unter `/root/Documents/`. `du -x -B1 -s /root/Documents/checkin-20261018-*-scoped-seed`:

| Ordnername | Belegte Bytes |
|---|---:|
| `checkin-20261018-0230-scoped-seed` | 29.868.032 |
| `checkin-20261018-0300-scoped-seed` | 29.913.088 |
| `checkin-20261018-0330-scoped-seed` | 29.949.952 |
| `checkin-20261018-0700-scoped-seed` | 30.015.488 |
| `checkin-20261018-0730-scoped-seed` | 30.056.448 |
| `checkin-20261018-0800-scoped-seed` | 30.101.504 |
| `checkin-20261018-0830-scoped-seed` | 30.142.464 |
| `checkin-20261018-0900-scoped-seed` | 30.187.520 |
| `checkin-20261018-0930-scoped-seed` | 30.253.056 |
| `checkin-20261018-1000-scoped-seed` | 30.289.920 |
| `checkin-20261018-1030-scoped-seed` | 30.326.784 |
| `checkin-20261018-1100-scoped-seed` | 30.384.128 |
| `checkin-20261018-1230-scoped-seed` | 30.584.832 |
| `checkin-20261018-1300-scoped-seed` | 30.633.984 |
| **Summe** | **422.707.200** |

Dies sind **Quell-/Dossierexporte**, keine separaten Gigabyte-Targets. Typisch etwa 19 MiB Dossiers, 5 MiB Crates und 4–5 MiB Research je Export. Sie verwenden den gemeinsamen Projekt-Targetcache. Das gelesene `checkin-20261018-0230-seed.py` erzeugt Export aus `git archive <head>`, kopiert genau den Scope darüber, schreibt `scoped-inputs.json` und startet Seedtests mit `CARGO_TARGET_DIR=/root/Documents/schrott-mcp/target`.

### Tatsächlicher Hash-/Quellvergleich

Für **jeden** der 14 Exporte:

1. Ergebnismanifest gelesen, Ausgangscommit mit lokalem `git ls-tree -rz <head>` aufgelöst (kein Checkout/Netzwerk).
2. SHA256 jedes darin benannten Overlay-Dossiers gegen die Exportdatei geprüft: **alle gültig**.
3. SHA256 des danebenliegenden Testlogs gegen `log_sha256` im Ergebnismanifest geprüft: **alle gültig**, dokumentierter Exit überall 0. Dies ist Log-/Inputprüfung, **keine neu ausgeführten Tests**.
4. Alle vorhandenen Exportdateien mit Git-Blobhash `SHA1("blob " + Länge + NUL + Bytes)` gegen ihren jeweiligen Basiscommit verglichen: **keine unerklärte Änderung außerhalb des deklarierten Overlays**. Zusätzliche Dateien nur die zwölf `scoped-inputs.json` und bei 11:00 die zwei ausdrücklich als Overlay deklarierten neuen Dossiers. Insbesondere keine nur dort vorhandene, vom Basiscommit abweichende Rust-Testregression gefunden. Kein Vollständigkeits-/Dateimodus-Audit aller Git-Einträge behauptet.
5. Overlay-Dossiers sind alle im aktuellen Repository getrackt. Alle außer einem entsprechen auch den aktuellen Arbeitsbytes. Ausnahme: **12:30 Marienberg** wurde um 13:00 erneut geändert; der historische 12:30-Hash stimmt separat mit dem lokalen Commit **`b84a166f644d92135ed78d1934f62e3c142ea50a`** überein. Nicht durch den heutigen Arbeitsstand ersetzen. 13:00-Overlay separat gegen HEAD `4f412ad866384784fcd32af031645afd0b62ca73` geprüft: gültig.

Damit sind die Quell-/Overlaybytes dieser Exporte lokal rekonstruierbar, solange die gebundenen Gitobjekte und Overlay-Versionen bleiben. Das macht **nicht** ihre originalen Manifeste/Logs entbehrlich und ist keine Zusicherung byteidentischer neuer Compileroutputs. Für Exportabbau zuerst die Manifeste außerhalb des betroffenen Ordners erhalten, exakten Reproduktionsinput dokumentieren und erneut hashen. Bei nur ~29 MiB Gewinn pro Export ist das gegenüber einem reinen Compiler-Cache unnötig riskanter; hier **keinen Export freigeben**.

### Exakt zu erhaltende Evidenz

- Für die zwölf Zeitpunkte **0230, 0300, 0330, 0700, 0730, 0800, 0830, 0900, 0930, 1000, 1030, 1100** jeweils die Originaldateien:
  - `/root/Documents/checkin-20261018-<Zeit>-scoped-seed/scoped-inputs.json` (191–1.849 logische Bytes, innerhalb des Exports);
  - `/root/Documents/checkin-20261018-<Zeit>-scoped-seed-result.json` (außerhalb);
  - `/root/Documents/checkin-20261018-<Zeit>-scoped-seed.log` (je 1.909 logische Bytes, außerhalb).
- Für **1230 und 1300** gibt es gerade **kein** `scoped-inputs.json` im Export und kein danebenliegendes `-result.json`. Maßgebliche Inputs/Exits/Loghashes liegen in **`/tmp/opencode/checkin-20261018-1230-seed-result.json`** und **`/tmp/opencode/checkin-20261018-1300-seed-result.json`**, vollständige Logs unter `/root/Documents/checkin-20261018-1230-scoped-seed.log` und `/root/Documents/checkin-20261018-1300-scoped-seed.log`. tmpfs-Pfade sind derzeit kein dauerhaft gesichertes Archiv; nicht als Cache behandeln. Keine dauerhafte Zweitkopie dieser Dateien wurde hier angelegt.
- Die Scope-/Schutz-/Formal-Auditdateien in `/tmp/opencode/checkin-20261018-*-scope.json`, `*-final-scope.json`, `*-protected-final.json`, `*-formal-audit.json` sowie gebundene Quellen-/GEO-Rohbelege bleiben Evidenz, nicht allein wegen `.json`/tmp freigeben. Die Muster beschreiben Erhaltung, keine pauschale Aussage, dass jede Kombination existiert.
- Originale uncommitteter Entwürfe in den vorhandenen `/root/Documents/checkin-20261018-*-precorrection-backup` und den beiden `checkin-20261018-1230-prechange-backup` / `checkin-20261018-1300-prechange-backup` unbedingt erhalten. Gemessen nur **28.672–122.880 Bytes pro Ordner**. Diese ursprünglichen Entwürfe sind **nicht** durch spätere korrigierte Gitfassungen ersetzt. Kein sinnvoller Platzhebel.
- Fremde/ungetrackte Dossiers und insbesondere `research/checkin-20261016-1300-release-build.md` nicht verändern. Anfangsstatus: 33 ungetrackte Dossiers plus dieser fremde Bericht; kein getrackter Änderungseintrag.
- Release-Evidenzwurzel **`/root/Documents/checkin-20261017-1630-release-a_8a8jlo`**, gemessen **161.796.096 Bytes**, bleibt erhalten: `source.tar`, `source-audit.json`, `release-manifest.json`, `artifact-manifest.json`, kopierte Binaries, Logs, private Smoke-/Integrationsbelege. Der Bericht `research/checkin-20261017-1630-release-build.md` bindet diese ausdrücklich. `target/release/schrott-mcp-server` und sein `deps`-Pendant sind außerdem Hardlinks (je 87.990.736 logische Bytes, zwei Links); eine einzelne Namenslöschung schafft dort keinen vollständigen Binarygewinn. Keinen Release-/Rollbackbestand für die Notfallentlastung anfassen.

## Separates `/tmp`-Problem — begrenzte Metadatenaufnahme

Nur `du` innerhalb des bekannten `/tmp/opencode` ausgeführt, keine Datenbank geöffnet oder Inhalte daraus gelesen. Insgesamt dort rund **3,5 GiB**; Hauptkandidaten:

| Exakter Pfad | Belegte Bytes | Aussagegrenze |
|---|---:|---|
| `/tmp/opencode/sperling-target` | 2.649.153.536 | davon `debug` 2.649.145.344; sehr großer mutmaßlicher Cargo-Cache, Owner/Quellbezug nicht vollständig auditiert |
| `/tmp/opencode/heraeus-independent-review-20261011` | 238.723.072 | davon `target` 237.838.336; **umgebende Reviewquellen nicht mitfreigeben** |
| `/tmp/opencode/target-msg-independent-review-20261010` | 131.596.288 | enthält **`db-repro` 9.592.832 Bytes**; Name „target“ ist hier gerade kein Beweis für reinen Buildcache |

Zusätzlich `du -h` jeweils etwa 122 MiB für `/tmp/opencode/target-msg-independent-review-20261011` und `/tmp/opencode/target-msg-independent-review-r1b-final-20261011`. Diese fremden Review-/Reprobestände wurden nicht auf Ersatzkopien geprüft. **Keine Freigabe** allein auf Basis dieser Metadaten. Ein separater Owneraudit des größten `sperling-target/debug` könnte tmpfs entlasten; bis dahin bleibt dessen knappe Reserve eigenständig blockierend. Keine Root-Vollinventur; nicht der gesamte 72-GiB-Verbrauch erklärt.

## Vorsichtige Platzplanung

1. **Jetzt kein Build/Backup zur Verifikation starten.** Reines Quellen-/Hashlesen und dieser Bericht brauchen wenig Platz; ein neuer Testexport kostet bereits etwa 29 MiB, zusätzlich können trotz gemeinsamen Caches neue Testbinaries von ~140–155 MB und Codegen/Linkerdateien entstehen. Der letzte 13:00-Ownerbericht dokumentiert selbst 31.330.304 Bytes Nettowachstum bei einem begrenzten Seedtest — das ist keine allgemeine Obergrenze.
2. Owner kann nach Freigabe den genau genannten alten Incremental-Unterbaum als **einzigen ersten Eingriff** wählen; Hardlinkabschlag und parallel wachsende Daten beachten. Keine `cargo clean`-/Wildcard-/pauschale `rm`-Anweisung aus diesem Audit ableiten.
3. Für späteren Release separat Budget festlegen: benötigter Build-/Linkerpeak + Export/Artefaktkopien + Logs + Sicherheitsreserve; **beide** Dateisysteme beachten. Der belegte frühere Releasebericht nennt Start rund 3,7 GiB frei auf `/` und Ende 2.266.972.160 Bytes; die jetzigen ~326 MiB liegen weit darunter, ein Unterschied im Nettowachstum ist kein verlässlicher Spitzenbedarf. Incremental-Notfallfreigabe allein reicht nicht als Nachweis.
4. Produktionsbackup erfordert darüber hinaus vorherige **reine Größenplanung** der genehmigten DB-/WAL-/Binarysicherungen und ihrer Ziele, ohne sie hier zu öffnen oder zu kopieren. Kein solcher Platzbedarf wurde in diesem Audit ermittelt; daher **Backup-/Rolloutblocker unverändert**. Platz schaffen ersetzt auch keine Authentifizierungs-/Input-/Fach-/Deploymentgates.

## Nachvollziehbare lesende Prüfkommandos

```sh
df -B1 / /tmp
du -x -B1 --max-depth=1 target
du -x -B1 --max-depth=1 target/debug
du -x -B1 --max-depth=1 target/debug/incremental
du -x -B1 -s /root/Documents/checkin-20261018-*-scoped-seed
git ls-files target
git check-ignore target/debug/incremental target/debug/examples
ps -eo pid,comm
```

Manifest-/Logprüfung ergänzend mit Python `json`, `hashlib`, `pathlib` und ausschließlich lesenden `git ls-tree` / `git show`; kein Scriptimport/-start der Seeddriver, kein Cargo, keine DB-Verbindung, kein Commit/Deploy. Einziger eigener Schreibpfad: **dieser Bericht**. Keine Löschung, keine Sicherung verschoben, keine Nutzer-/Fremdarbeit überschrieben.
