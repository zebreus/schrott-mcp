# Unabhängiger Release-Review – 16.10.2026, Check-in 03:00

## Entscheidung

**Inhaltlicher Umfang freigabefähig; noch keine Installations-/Rolloutfreigabe.** Gegenüber dem vorbereiteten Release `fbfd06c0ff1635e58f9f46f5ed39c62d8a6cb3f7` enthält der geprüfte HEAD `81a6483208ade84c592e89a8684f56c9774dcea7` keine neue Rust-/Schema-/Handleränderung. Die zusätzlichen Dossierkorrekturen rechtfertigen keinen weiteren Code-Fix als Voraussetzung. **Die vorhandenen 22:30-Releasebinaries dürfen aber nicht als Release dieses HEAD installiert werden:** Ihr eingebetteter Seed ist veraltet. Ein neuer isolierter, an diesen HEAD gebundener Build und dessen Produktionsabnahme fehlen. Der zuletzt dokumentierte GitHub-Authentifizierungsblocker ist ebenfalls nicht als aufgehoben nachgewiesen.

Dieser Review hat weder gebaut noch Tests wiederholt, keinen Server gestartet, keine Datenbank beschrieben, nichts gestaged/gecommittet und keine Installation vorgenommen. Ein versuchter zusätzlicher Read-only-Reviewagent wurde vom Harness wegen der Subagent-Tiefengrenze abgewiesen; daraus wird kein zweiter erfolgreicher Review behauptet.

## Tatsächlich geprüfter Diff und Seed

- `fbfd06c0` ist Vorfahr von HEAD; Zwei-Punkt-Diff und Merge-Base-Vergleich beziehen sich daher auf denselben Ausgangsstand. Acht zusätzliche Commits, **77 Dateien: 60 bestehende Dossiers und 17 Researchberichte**. Keine hinzugefügten/entfernten Dossiers, keine Slugwechsel. `git diff --check fbfd06c0 HEAD` ohne Befund.
- Die **143 geschützten offenen Dateien** wurden gegen `/tmp/opencode/checkin-20261016-013049-baseline.json` erneut per SHA256 geprüft: alle unverändert; keine überschneidet sich mit dem committeten Release-Diff. Ein daneben offener `research/checkin-20261016-0300-owner.md` gehört ebenfalls nicht zum geprüften HEAD. Kein Arbeitsbaum-Overlay verwendet.
- Die jeweils **3.892 Dossierdateien** der vorhandenen isolierten Archive wurden frisch bytegenau gegen `git show fbfd06c0:<Pfad>` beziehungsweise `git show HEAD:<Pfad>` verglichen: jeweils null Abweichungen. Archive: `/root/Documents/checkin-20261015-2200-head-cgdwaw96` und `/root/Documents/checkin-20261016-0130-isolated-7qa14e0l`.
- Danach vorhandene echte Codegen-Ausgaben `audit-out/seed_traders/*.json` und `owner-after/seed_traders/*.json` verglichen: **3.892 → 3.892**, keine hinzugefügten/entfernten Schlüssel, exakt **60 geänderte Seeds**. Das ist ein frischer Vergleich vorhandener Generatorartefakte mit frisch bestätigten Eingaben, **keine neue Generatorausführung**.

| Änderung im kompilierten Seed | Anzahl |
| --- | ---: |
| ausschließlich `notes` | 50 |
| `status` und `notes`: H&G, Haucke, DAS Recycling | 3 |
| TSR Ginsheim und NRH Edersleben: acht Stammdatenfelder plus `notes` | 2 |
| `city/postcode/notes`: Holthusen, Göttingen/Koch, Mainz/Arnold, Zedlitz/Zauritz | 4 |
| Heine: `street/status/notes` | 1 |

In allen 60 Ausgaben ist der vollständige vorherige `notes`-String weiterhin enthalten. Die vier Orts-/PLZ-Trennungen sind syntaktisch, keine neuen Standortbeweise. Kein neuer Preisdatensatz, keine Koordinatenänderung, kein neu eingeführtes `clear_fields` und keine neue Annahme-JSON-Freigabe.

## Standards und fachliche Freigabe

Maßstab: README „Source hierarchy“, autoritative Betreiber-Ausnahme und Betreiberketten; tatsächliche Seedsemantik statt bloßer Markdownansicht. Ein gesondertes Ursprungsticket wurde nicht vorausgesetzt: Auftrag ist die Releaseprüfung des vorhandenen Dossierdiffs.

**Keine neue harte Standardverletzung in den strukturierten Änderungen festgestellt.** Die kritischen Aktivierungen sind standortbezogen belegt:

- **TSR Ginsheim-Gustavsburg:** Betreiberdetail frisch per normalem HTTPS-Abruf gelesen: lokale Nummer `+49 6134 750225`, Zeiten einschließlich Pause 09:00–09:30, Buntmetalle ab 1 kg/Schrotte ab 100 kg, Barvergütung bei Selbstanlieferung. Webfetch allein lieferte eine Challenge, der direkte HTTPS-Abruf dagegen lesbaren Inhalt; keine Sperrseite als Standortbeleg gewertet. Originalzertifikat frisch im Speicher geladen und mit `pdftotext -layout - -` gelesen: HRA 19743 Dortmund, Gültigkeit bis 12.01.2028, mehrfach ausdrücklich NL Ginsheim-Gustavsburg/Landdammstraße 15/65462. SHA256 **c57e18c89a3918f8ca8fe73464f6996eb545dad98f141a7f99b30f9915748bce**, deckungsgleich mit Ownerbeleg. Kein frischer amtlicher Registerauszug behauptet; der ältere Registerspiegel bleibt eine dokumentierte Grenze.
- **NRH Edersleben:** Betreiber-Anfahrtsseite frisch gelesen: eigener Standort Der Hutdeckel/06528, 03464-671217, Mo/Di/Do bis 16 Uhr, Mi bis 18 Uhr, Fr bis 14 Uhr, Pausen 09–09:30 und 12–12:30. Originalzertifikat frisch wie oben gelesen: HRB 403819 Jena, Edersleben/Hutdeckel in mehreren Anlagen, gültig bis 30.09.2027; SHA256 **7a3ac9f882e2baaee75adfd757767475114fb17e01961521d7336f55f4dfe521**, ebenfalls identisch. Keine pauschale Privat-/Gefahrstoffannahme aus Abfallkatalogen abgeleitet.
- **H&G/Haucke/Heine:** `aktiv → pruefung` zieht ungenügend belegte Aktivität zurück; weder Schließung noch Zusammenführung. Heines `Schachtstr.` wird bewusst geleert, nicht ungeprüft durch Friedenshaller Ring ersetzt.
- **DAS Recycling:** `geschlossen → pruefung` ist nachvollziehbare Rücknahme einer unbelegten Negativfeststellung, keine bestätigte Wiedereröffnung.
- **MRJ, Gertloff, RCH und Wiese:** neu dokumentierte kommunale/alte Register-/Zertifikatsfunde führen gerade nicht zu ungesicherten Betreibermerges oder Kontakttransplantationen. Verbleibende Identitätsketten sind keine Voraussetzung für eine reine Notizveröffentlichung bei unverändertem `pruefung`.

Die übrigen Notizänderungen qualifizieren überwiegend Leads und Abrufgrenzen. Sie sind keine neue Händler-, Preis- oder GEO-Verifikation. Insbesondere historische H&G-Kontakte/Zeiten bleiben im strukturierten Bestand erhalten; `pruefung` und korrigierte Timeline verhindern keine isolierte Fehlinterpretation einzelner Felder durch SQL-Nutzer. Das ist eine ausdrücklich dokumentierte Bestandsgrenze, keine frisch freigegebene Verifikation.

Quellen der eigenständigen Direktprüfung:

- https://www.tsr-recycling.de/standorte/schrotthaendler-ginsheim-gustavsburg/
- https://www.tsr-recycling.de/uploads/tx_3slocations/Efb_Zertifikat_TSR%20SW%20II%20(Frankfurt-Darmstadt_Ginnsheim-Gustavsburg)_12012028.pdf
- https://www.nrh-recycling.de/anfahrt.html
- https://www.nrh-recycling.de/uploads/pdf/2026_ZZEE001002207010_EFB-Zertifikat_NHR_10220.pdf

## Konkrete Seed-/Deploymentrisiken

### 1. Veraltetes Releaseartefakt – tatsächlicher Rolloutblocker

Der 22:30-Server mit SHA256 `989c4a16313bc4d35cc2e1d4a080e9e4305af1bee323f3d0cd5e93d06494c977` wurde laut zugehörigem committetem Manifestbericht aus dem `fbfd06c0`-Archiv gebaut. Ein Neustart dieses Artefakts liest **nicht** die inzwischen geänderten Dossiers: `build.rs:364–416` kompiliert sie, `seed_traders.rs:164` bindet sie mit `include_str!` ein. Dass Rust unverändert ist, macht diesen Server deshalb nicht HEAD-kongruent. Worker-Code unverändert; beide Deploymentbinaries sollten dennoch als nachvollziehbares gemeinsames Releasepaar bereitgestellt werden.

**Freigabebedingung:** ausschließlich sauberer HEAD-Export, kein Build aus dem Hauptarbeitsbaum; Eingabeinventar, Commit und neue Artefakthashes festhalten. Die vorhandenen technischen Testnachweise dürfen als Vorbefund dienen, nicht als Build-/Rolloutnachweis des neuen Servers.

### 2. Notizen lösen vollständige Row-Upserts aus – nicht nur 10 skalare Änderungen

`payload_hash` enthält `notes` (`seed_traders.rs:269–285`). Jeder der 60 geänderten Seeds kann deshalb einen vollständigen Upsert auslösen. `street/postcode/city/status` sind seedführend, auch bei Leerwerten; Kontakt-/Beschreibungsfelder werden bei gewöhnlichen Leerwerten dagegen erhalten. Fehlendes GEO erhält das gespeicherte Paar (`384–387`). Der SQL-Upsert ersetzt zudem `extra_json` und setzt vom Seed nicht belegte min/max-Mengen auf NULL; bei den 60 aktuell gelesenen Produktionszeilen waren beide Mengen bereits NULL.

Frische Produktionsgegenprüfung ausschließlich über `file:/var/lib/schrott-mcp/public.db?mode=ro` und `PRAGMA query_only=ON`, 3.881 Händler. Neben den bewusst neuen Änderungen treten gegenüber der Produktion auch **bereits vor fbfd06c0 vorbereitete**, noch nicht ausgerollte Korrekturen zutage:

| Seed | Produktionswirkung beim fälligen Upsert |
| --- | --- |
| Pfeifer Rainer/Würzburg | `Brunnfloßgasse 12` und `97078` werden leer; bestehendes `clear_fields: phone` löscht die derzeit noch gespeicherte Nummer |
| Schrottankauf Klar/Bochum | zusammengesetztes Straßenfeld `Schrottankauf Klar Bochum Robertstr. 1` wird leer |
| Klass/Rellingen | `Adlerstr. 36` wird leer, Aktivstatus wird `pruefung` |
| Gröschel/Northeim | vorbereiteter Name Resebeck GmbH (Standort Northeim), Am Mühlenanger 46/37154 wird erstmals gegenüber diesem Produktionsstand wirksam |
| Lenhart/Kitzingen | vorbereitete Statusrücknahme `aktiv → pruefung` |

Diese Effekte sind **keine neuen Regressionen des Vergleichsdiffs** und durch erhaltene Korrekturhistorie grundsätzlich erklärbar. Sie dürfen aber beim nächsten Deploy nicht als „nur neue Notizen“ übersehen werden. Die Pfeifer-Altformulierung „DB-Adresse/GEO ... nicht verändert“ beschreibt nicht die Wirkung des gesamten späteren Seed-Upserts: Adresse wird geleert, GEO bleibt. Post-Rollout-Abnahme muss diese Fälle zusammen mit den neuen Stammdaten prüfen; unveränderte Warenpreise allein reichen nicht.

### 3. Alte GEO-Paare bleiben trotz geänderter Adresse oder Aktivierung bestehen

Read-only frisch bestätigt: TSR **49.9711/8.3453**, NRH **51.4167/11.2833**, Heine **51.7946/11.7401**. Keine dieser Koordinaten wird durch diesen Release geprüft oder korrigiert. Auch Adressleerungen in obiger Tabelle entfernen keine Pins. **Kein Blocker für Stammdaten-/Notizrelease**, solange keine verifizierten Hof-/Einfahrtspunkte oder abgeschlossene GEO-Bereinigung versprochen werden. Für ein solches Kartenversprechen wäre es ein echter fachlicher Blocker.

### 4. Installation, Start und Rollback sind schreibende Vorgänge

`server/main.rs:86` führt Seedmetadata schon vor HTTP-Start aus; Gesundheit des alten Dienstes beweist keine neue Veröffentlichung. Der Seed läuft zeilenweise ohne äußere Gesamttransaktion. Ein Fehler kann deshalb einen Teilstand hinterlassen. Alte Binärdateien wieder einzuspielen ist kein vollständiges Datenrollback: Seed löscht keine Trader, und Erhaltungsregeln verhindern insbesondere die automatische Rücknahme vorher gefüllter Kontaktfelder. WAL-sichere Sicherung und nachvollziehbarer Wiederherstellungsplan müssen vor dem genehmigten Rollout vorliegen; hier weder Sicherung noch Restore ausgeführt.

## Freigabeumfang und nächste Gates

**Freigabefähig:** genau die 60 committeten Seedänderungen samt 17 Berichten als Nachfolger des vorbereiteten Releases; bestehende Handlerfixes bleiben dadurch unberührt. Kein neuer eigenständiger Handler-/Preis-/GEO-Abschluss.

**Vor Installation offen:** reguläre Veröffentlichung/Authentifizierung klären (keinen Bypass aus diesem Review ableiten), HEAD-isolierten Server bauen, Releasepaar/Hashes nachweisen, Sicherung und Rollback festlegen. **Nach Installation:** tatsächlichen Prozess-/Binarywechsel und erfolgreichen Bootseed belegen; TSR/NRH, vier Ortsnormalisierungen, vier Statuskorrekturen einschließlich Heine-Straßenleerung sowie obige Alt-Pendenzen gezielt read-only abnehmen. Ein grünes `/health` oder alte 14 Seedtests ersetzt diese Abnahme nicht.

**Endurteil:** keine zusätzliche inhaltliche Sperre im neuen Diff gefunden; **HOLD für Rollout mit alten Artefakten oder schmutzigem Buildinput**. Die 143 fremden Dateien sind unverändert und ausdrücklich nicht freigegeben.
