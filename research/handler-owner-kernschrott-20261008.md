# Handler-Owner Kernschrott (#5529)

## Owner-Abnahme auf aktuellem Main (10.10.2026)

Nach Übertragung den gesamten Integrationsdiff und die finale Parserlogik
gelesen; additive Katalog-/Registryänderungen geprüft. Handler und Fixture
entsprechen unverändert den freigegebenen SHA-256-Werten. Im Dossier nur den
neuen Review-Korrekturbullet auf datiertes Korrekturformat mit konkreten
Quellenlinks gebracht; alte Historie und Frontmatter unverändert.

- `cargo fmt --all -- --check`: erfolgreich.
- `cargo test --locked --workspace` auf Main: **606 bestanden, 0 Fehler,
  5 bestehende/Netzwerktests ignoriert**; Ingestion 549. Enthält alle neun
  lokalen Kernschrotttests und Seed-/Persistenzprüfungen.
- Separater echter Main-Livehandler mit `--ignored --nocapture`: **1 bestanden**,
  HTTP 200, 38 Preise / 19 bedingte Materialannahmen / 1 Class-3-Skip;
  Quellenstand 06.10.2026. Kein Produktions-Storezugriff.
- Main-Ausführungslogs: `/root/Documents/kernschrott-main-workspace-20261010.log`
  und `/root/Documents/kernschrott-main-live-20261010.log`.
- Gestagter Diff-Check für Code/Dossier/Dokumentation sauber. Der vollständige
  Check meldet ausschließlich drei originale Whitespace-Zeilen der unverändert
  übernommenen HTML-Fixture und abschließende Leerzeilen der drei Originallogs.
  Diese Evidenzdateien bewusst bytegleich erhalten statt für den Check ändern.
  Keine neue Dependency, kein Rust-Geocoding,
  kein Shared-Record-/Store-Umbau, keine Produktions-DB-Schreiboperation.

Freigabe zum lokalen Main-Commit. Push bleibt wegen fehlender GitHub-
Authentifizierung offen; noch kein Kernschrott-Deployment und kein regulärer
Produktionsnachweis. Die folgenden Vorbereitungs-/Source-Abschnitte sind
historische Schritte, ihre alten Test-/Integrationsgates gelten jetzt nur
noch für den damaligen Stand.

Stündlicher Check-in 10.10.2026 15:00 UTC: Dienst active, keine Warnungen in
den letzten sechs Stunden; read-only Bestand 3881 Händler/54 Materialien/
2426 aktuelle Preise, Feedback unverändert bis ID 6080. Push erneut mit
deaktivierten Prompts versucht und an fehlender HTTPS-Anmeldung gescheitert.

## Main-Integration vorbereitet (10.10.2026) — noch nicht auf Main getestet

Owner-Freigabe des unabhängigen Follow-ups für Handler-SHA-256 `28ed6640fb3026b45167c38bfed77e88d1e6de2c7618f72a4798bf7c7929c47e` übernommen. Ausschließlich geprüfte Kernschrott-Deltas per Patch in `/root/Documents/schrott-mcp` eingebracht, Main-Baseline `a181e3e1937ca183aa1273149b3570c45235f9b7`. **Kein Branch-Merge und kein Ersatz gemeinsamer Dateien durch alte Worktree-Dateistände.** Source-Worktree `/root/Documents/schrott-mcp-kernschrott-20261008` unverändert erhalten. Keine zusätzlichen Semantikänderungen.

Genau diese neun Dateien übertragen/ergänzt:

1. `crates/ingestion/src/traders/handlers/kernschrott.rs` — neues Handlerfile, bytegleich zum freigegebenen Hash.
2. `crates/ingestion/tests/fixtures/kernschrott-elektro-20261008.html` — neue unveränderte Fixture, bytegleich zum Source-Worktree.
3. `crates/ingestion/src/pipeline.rs` — ausschließlich die 13 geprüften additiven Katalogeinträge; sämtliche bisherigen Main-Inhalte erhalten.
4. `crates/ingestion/src/traders/handlers/mod.rs` — ausschließlich Moduldeklaration und Registry-Zeile für Kernschrott.
5. `dossiers/by/by-regensburg-kernschrott-recycling.md` — ausschließlich die zwei geprüften Timeline-Ergänzungen vom 08./10.10.2026. Main-Dossier war vor Übertragung bytegleich zur Source-HEAD-Baseline; keine zusätzliche Main-Timeline vorhanden, bestehende Metadaten/Timeline vollständig erhalten.
6. `research/handler-owner-kernschrott-20261008.md` — Source-Bericht übernommen, nur dieser Main-Übergabeabschnitt und Hinweise zu historischen Source-Logs ergänzt.
7. `research/kernschrott-targeted-review-fixes-20261010.log` — finales gezieltes **Source-Worktree**-Testlog, bytegleich übertragen.
8. `research/kernschrott-workspace-review-fixes-20261010.log` — finales **Source-Worktree**-Workspace-Testlog, bytegleich übertragen.
9. `research/kernschrott-live-review-fixes-20261010.log` — finales **Source-Worktree**-Livehandlerlog, bytegleich übertragen.

Statische Übertragungsprüfung: Entfernen genau der 13 eingefügten Katalogblöcke bzw. der zwei Modul-/Registry-Zeilen ergibt bytegleich den jeweiligen aktuellen Main-HEAD-Dateistand. Damit bleiben alle neuen Main-Katalogeinträge und sämtliche Main-Handler erhalten, einschließlich der gegenüber Source zusätzlich vorhandenen Module `agh_altgoldhandel`, `beller`, `elno`, `hofmann_rastatt`, `regold`, `saxonia`, `trapper` (sieben zusätzliche Moduldeklarationen). Handler, Fixture und die drei finalen Logs wurden auf Bytegleichheit geprüft. Vorhandener Main-Reviewbericht `research/checkin-20261010-kernschrott-independent-review.md` nicht verändert.

**In diesem Main-Vorbereitungsschritt keine Tests, Builds, Rustfmt-Ausführung, Commits, Pushes, Deploys oder Produktions-DB-Zugriffe.** Sämtliche grünen Test-/Live-Ergebnisse weiter unten stammen ausschließlich aus dem Source-Worktree, nicht aus dem inzwischen erweiterten Main.

Noch offene Gates nach Vorbereitung:

- Owner prüft den aktuellen Main-Diff einschließlich aller neuen Dateien und dieser übertragenen Belege.
- Owner führt gezielte Kernschrott-/Persistenztests, vollständige aktuelle Main-Workspace-Tests, Format-/Diff-Prüfung und Livehandler-Verifikation auf Main durch; die Source-Logs ersetzen diese Main-Verifikation nicht.
- Erst anschließend entscheidet der Owner über Commit/Push, Main-Integration und koordinierten Rollout; GitHub-Authentifizierung gegebenenfalls extern beheben.
- Regulären Produktions-Scheduler-Lauf und gespeicherte Bedingungen/Preise anschließend read-only prüfen; Produktionsgate bleibt offen. Dokumentierte Dedupe-/Current-Staleness-/Querysichtbarkeitsgrenzen bleiben unverändert.

- Owner-Branch: `owner/kernschrott-20261008`, eigener Worktree `/root/Documents/schrott-mcp-kernschrott-20261008`, Ausgangspunkt `072cf25`.
- Auftrag bleibt bis Deployment und erfolgreichem **regulären** Prod-Scrape offen. Rollout beider Binaries ausschließlich durch koordinierenden Owner; keine Produktionsschreibzugriffe durch diesen Handler-Owner.
- Quellen am 08.10.2026: Legacy `http://www.kernschrott-recycling.de/?seite=elektro` leitet auf `https://www.kernschrott-recycling.de/elektro`; Impressum `https://www.kernschrott-recycling.de/impressum` bestätigt Kernschrott-Recycling UG (haftungsbeschränkt), HRB 12051, Dieselstr. 7, 93053 Regensburg, Michael Bitterbier. Keine Betreiberumbenennung oder Änderung vorhandener Stammdaten erforderlich.
- Preisliste: Stand **06.10.2026**, **EUR/kg**, 42 Positionen. Erwartet: **38 Preise**, **3 Anfragefraktionen ohne Preis**, **1 bewusster Skip**. Seit Review-Korrektur liefert der Handler **19 explizite aggregierte Materialannahmen**, davon 3 Anfragefraktionen; nicht mehr nur 3 Annahmeobjekte. Lüfter 0,00 ist ein tatsächlich veröffentlichter Nullpreis, keine erfundene Vergütung. Class-3-Platinen enthalten laut validierter Beschreibung gefährliche Stoffe, die der validierte globale Ausschluss ausschließt: Widerspruch wird quarantänisiert; keine bestätigte Class-3-Annahme.
- Handler `crates/ingestion/src/traders/handlers/kernschrott.rs`: Tabellenzeilen dynamisch auslesen, exakte semantische Sorten-Mappings, Original-Sortenname als Variante; Beschreibungen und validierte globale Vorbehalte in Preisnotizen und aggregierten Annahmebedingungen. Zusätzliche unbekannte Sorten werden laut übersprungen, sofern alle bekannten Sorten erhalten bleiben und die Positionszahl stimmt. Nicht unterstützte Preisangaben bekannter Sorten, fehlende bekannte Sorten, verlorene Pflichtbeschreibungen und veränderte globale Bedingungen blockieren den gesamten Snapshot. Weitere konkrete Guards/Nachweise unten; keine pauschale Garantie für beliebige semantische Inhaltsänderungen. Fehlender Seitenstand bleibt unbekannt (Shared-Inferenz-/Dedupe-Regeln unverändert).
- Katalog additiv: `prozessoren`, `integrierte-schaltungen`, `elektronikstecker`, `notebooks`, `computer`, `netzteile`, `laufwerke`, `optische-datentraeger`, `ablenkeinheiten`, `panzersicherungen`, `elektrobauteile`, `luefter`, `kabel-ohne-stecker`. Keine Laufwerk-/CPU-/Gerätepreise unter Platinen; Handyplatinen sind echte `platinen`, ganze Handys echte `handys`. Bei „Kabel ohne Stecker“ ist das Leitermetall nicht ausdrücklich belegt: eigene generische Kabelannahme statt erfundener Kupfer-Zuordnung. Keine Migration/Umklassifizierung vorhandener Katalog-/Preisgeschichte; **keine Immutable-History-Garantie** für zukünftige Record-Ausführungen (siehe S2).
- Fixture: unveränderte HTTP-Seite `crates/ingestion/tests/fixtures/kernschrott-elektro-20261008.html`, Abruf 08.10.2026. Tests decken gesamte Liste, getrennte Sorten, Anfrage/Nullpreis, Änderungen, neue unbekannte Sorten, Modellnummern vs. Preise, Struktur/Datum/Einheit, Identitätsprüfung und echte Seed→Record→History-Persistenz ab.

## Verifikation / Übergabe

Historischer Stand vor Wiederaufnahme: vollständige Testausführung und echter Livehandler waren noch ausstehend. Mehrere OpenCode-Neustarts hatten laufende Builds abgebrochen; diese abgebrochenen Läufe sind kein erfolgreicher Testnachweis. **Aktueller Auftrag vom 10.10.2026: keine Commits, kein Push, kein Deploy; Main-Integration/Rollout ausschließlich durch koordinierenden Owner nach Review.** GitHub-Authentifizierung bleibt extern blockiert und wurde hier nicht benötigt.

Build-Umgebung: `/root/.cargo/bin/cargo` (Cargo fehlt im Harness-PATH). Die erste komplette Verifikation wurde am 09.10.2026 nachweislich durch den Kernel-OOM-Killer beendet (`journalctl -u kernschrott-validation-20261009`: `oom-kill`, 1 GB Peak). Wiederholung daher mit `CARGO_BUILD_JOBS=1 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0`, eigenem Worktree-Target und temporärem lokalen systemd-Testjob, ohne Produktionsservice-/DB-Änderung. Keine neue Regex-Abhängigkeit: streng validierte handlerlokale Betrags-/Datumsparser.

### Historische Erstverifikation 10.10.2026 — vor unabhängigem Review

- Bestehende Änderungen an Handler-Registrierung, additivem Materialkatalog, Dossier und unveränderter HTML-Fixture erhalten. Kein Rust-Geocoding ergänzt oder externer Geocodinglauf ausgeführt (die Workspace-Suite enthält vorhandene reine lokale Geocode-Cache-Unit-Tests). Keine Produktions-DB geöffnet oder geändert; Persistenztests verwenden ausschließlich neue temporäre lokale Test-DBs.
- Weiter gehärtet: exakte Tabellenüberschriften `Position` / `Preis / kg` statt Substring-Einheitsprüfung; genau ein nichtleerer Sortenname und ein Preisfeld sowie höchstens eine Beschreibung je Zeile (auch unbekannte Sorten), rein numerisches Datum, keine nicht-endlichen Beträge. Unbekannte Sorten bleiben laute Skips; geänderte Preise werden weiter dynamisch übernommen.
- Snapshottest prüft nun sämtliche 38 `(Originalvariante, Material, Preis)`-Tripel und alle 3 Anfragefraktionen einzeln. Persistenztest prüft zusätzlich jeden gespeicherten Preis inklusive EUR/kg, Quelle, Seitenstand, `page_stated` und Originalnotizen sowie Annahmebedingungen; CPU-Preisänderung bleibt als getrennte Historie nachgewiesen.
- Eigener Buildpfad: `/root/Documents/kernschrott-cargo-target-20261010` (nicht Main-Target); alle Cargo-Testläufe mit `CARGO_BUILD_JOBS=1 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0`. Kein neuer systemd-Job/Produktionsservice-Eingriff bei dieser Wiederaufnahme.
- Erster Handler-Testlauf: `cargo test -p schrott-mcp-ingestion kernschrott -- --nocapture`: **6 bestanden, 0 fehlgeschlagen, 1 expliziter Livetest ignoriert**; vollständiger Neuaufbau in 1m52s erfolgreich. Aktuelle Review-Fix-Verifikation folgt unten.
- **Echter Livehandler erfolgreich:** `cargo test -p schrott-mcp-ingestion live_handler_verifies_identity_prices_and_date -- --ignored --nocapture`: **1 bestanden**, direkte Ausführung von `(handler().scrape)(&client)` einschließlich Live-Impressumsprüfung, ohne Store/`record()`. Ergebnis HTTP **200**, **40824 Bytes**, `fetch_url=https://www.kernschrott-recycling.de/elektro`, `published_at=2026-10-06T00:00:00+00:00`, **38 Preise / 3 Annahmen / 1 quarantänisierter Class-3-Skip**; Lüfter **0,00 EUR/kg**, Intel 286/386/486 **308,00 EUR/kg**. Dies ist ein Netzwerktest, **kein** regulärer Produktions-Scheduler-Nachweis.
- **Abschließende historische Workspace-Suite vor Review-Fixes:** `cargo test --workspace --quiet`, Exit **0**, **569 bestanden / 0 fehlgeschlagen / 5 explizite Netzwerktests ignoriert**, Doc-Tests ebenfalls erfolgreich. Davon Ingestion **512 bestanden**. Altes Log ausschließlich im erhaltenen Source-Worktree: `/root/Documents/schrott-mcp-kernschrott-20261008/research/kernschrott-workspace-tests-20261010.log`; bewusst nicht zusätzlich nach Main übertragen.
- **Historischer Livehandler vor Review-Fixes erneut erfolgreich:** Exit **0**, **1 bestanden**. Altes Log ausschließlich im erhaltenen Source-Worktree: `/root/Documents/schrott-mcp-kernschrott-20261008/research/kernschrott-live-handler-20261010.log`; bewusst nicht zusätzlich nach Main übertragen. Fixture wurde nicht mit neuen Liveinhalten überschrieben.
- `cargo fmt --all -- --check` und `git diff --check` erfolgreich. Keine neuen Dependencies oder Änderungen an `Cargo.lock`.
- Diese Erstverifikation belegte Werte/Materialzuordnung, aber **nicht** die später in S1/F1 gefundenen Integrations-/Vollständigkeitslücken. Ihr pauschaler Fail-closed-/History-Anspruch ist durch den unabhängigen Review korrigiert. Die alten Logs bleiben als historische Evidenz erhalten, nicht als Nachweis der Review-Fixes.

## Review-Korrekturen S1/S2/F1–F4 (10.10.2026)

Unabhängigen Review `/root/Documents/schrott-mcp/research/checkin-20261010-kernschrott-independent-review.md` vollständig gelesen; Owner-Entscheidungen umgesetzt, ausschließlich handlerlokal, kein Shared-Record-/Store-/View-/History-Umbau.

### S1 — tatsächliche bedingte Materialannahmen

Der Handler aggregiert **eine** explizite Annahme pro Material (19): alle Originalvarianten mit ihren jeweiligen Beschreibungen, die tatsächliche validierte Unverbindlichkeit/Rücksprache und den tatsächlichen validierten globalen Gefahrstoffausschluss. Die Zusammenstellung bezeichnet Variantenbedingungen ausdrücklich als jeweils zugehörig, nicht als Bedingungen der letzten Variante. `record()` schreibt zuerst Preis-/Fallback-Annahmen und **danach** diese Aggregate; dadurch werden implizite leere Bedingungen einschließlich des RAM→Platinen-Fallbacks überschrieben. Platinenbedingungen grenzen Class 3 ausdrücklich als quarantänisierte, nicht bestätigte Annahme ab (keine variantenspezifische Negativannahme im Store behauptet).

Test `seeded_catalog_records_all_conditions_and_amount_change_history` verwendet echte lokale Seed→Record→Store-Aufrufe. Er prüft gespeicherte Annahmerows für **alle 41 gemappten Varianten**, globale Vorbehalte und gezielt Festplatten/Notebooks/Handys/Elektronikstecker/Netzteile/Platinen. Netzteile enthält alle drei Varianten, Handys beide Varianten, Platinen alle gültigen Klassen/Sorten. Nach einem zweiten Preisrecord werden sämtliche Aggregate erneut geprüft; kein letzte-Variante-gewinnt und kein Verlust durch implizite Annahmen.

### F1/F2 — Vollständigkeit und Drift statt erfolgreicher Teilsnapshots

- Abgleich tatsächlicher Nicht-Kategoriezeilen mit genau einer `#pcount`-Angabe im Preisabschnitt; zusätzlich müssen **alle 41 bekannten gemappten Varianten plus Class 3** vorkommen. Entfernen/Umbenennen bleibt auch bei passend reduzierter Seitenzahl ein Parsefehler.
- Kategoriezeilen müssen echte einspaltige `colspan=2`-Zeilen ohne Sorten-/Preis-/Beschreibungsfelder sein. Maskierte `cat`-Preiszeilen blockieren, nicht still überspringen.
- Alle bekannten Sorten mit zuvor vorhandener Beschreibung benötigen weiterhin eine nichtleere `.ds`; nur sechs explizit überprüfte, ursprünglich beschreibungslose Sorten dürfen ohne Beschreibung vorkommen. Entfernen/Umbenennen/Leeren einer Pflichtbeschreibung blockiert. **Geänderte nichtleere Variantenbedingungen werden quellengetreu übernommen**, nicht automatisch als fachlich identisch behauptet.
- Numerisch→Anfrage, Anfrage→numerisch und unlesbare/nicht unterstützte Preisangaben bekannter Sorten blockieren den gesamten Snapshot bis Neubewertung. Keine Current-Withdrawal-Implementierung hinzugefügt. Numerische Betragsänderungen bleiben dynamisch erlaubt. Zusätzliche unbekannte Sorten bei vollständigem Bestand und stimmiger Positionszahl bleiben laute Skips, keine geratenen Zuordnungen.
- Globale Reservation und Gefahrstoffausschluss müssen im **Preisabschnitt** jeweils genau einmal und mit dem geprüften Wortlaut vorhanden sein. Verlust/Änderung führt zu Parsefehler/Neubewertung, nicht zu erfundenen unveränderten Quellfakten. Die validierten Texte werden in sämtliche Preisnotizen und Materialannahmen übernommen.

Tests `source_drift_blocks_instead_of_producing_partial_or_unconditional_snapshots` und `failed_parse_after_record_leaves_all_current_history_and_acceptances_unconfirmed` decken entfernte Zeilen mit unveränderter/reduzierter Zahl, verschwundene Class 3, maskierte Kategorie, fehlende/leere/umbenannte Pflichtbeschreibung, Zahlendrift, Preis→Anfrage, ungültige bekannte Preisangabe, Sortenumbenennung, veränderte/fehlende globale Vorbehalte und geänderte Class-3-Beschreibung ab. Der zweite Test speichert zuerst den vollständigen Snapshot mit echtem `record()`, versucht dann dieselben Mutationen an **Kopien** der Fixture und vergleicht alle Current-Preisrows (inkl. IDs/Beobachtungen/Notizen), sämtliche Variantenhistorien und Annahmen vor/nach fehlgeschlagenem Parse: unverändert; der fehlgeschlagene Parse erreicht `record()` nicht.

**Bewusste F2-Grenze:** Bereits vorhandene Preise bleiben bei blockierter Quelle im Current-Pointer und in der History mit ihrer **alten Beobachtung**, ohne neue Bestätigung. Ein Parsefehler beendet/withdrawt keinen alten Preis und macht die notes-freie Current-View nicht automatisch aktuell gültig. Separate Gültigkeits-/Withdrawal-/Staleness-Lösung bleibt erforderlich; bis dahin müssen Konsumenten Beobachtungsalter und Scrape-Fehler beachten. Keine erfolgreiche Teilspeicherung und kein Refresh alter Current-Preise durch diesen fehlgeschlagenen Scrape.

### S2 — reale Dedupe-Semantik, keine Immutable-History

Test `unchanged_amount_changed_conditions_refreshes_existing_row_not_immutable_history` belegt mit echtem `record()` und Store: gleichbleibender Festplattenbetrag, geänderte nichtleere Bedingungen und neuer Seitenstand ergeben **dieselbe Preis-ID und genau eine Historyrow**; Beobachtung/Ingestion, Notizen und Publikationsdatum werden aktualisiert. Materialannahmen enthalten danach die neuen Bedingungen. Die alte Bedingungs-/Datumsbeobachtung bleibt dabei **nicht unveränderlich archiviert**. Der separate 308→321,50-CPU-Test belegt zwei Betragshistorien, keine generelle Immutable-Observation-History. Das ist bewusst bestehende Store-Semantik, hier nicht umgebaut.

### F3 — Query-Vertrag und Vorbehalte

`exact` / `confidence=1.0` bedeutet **exakt abgelesener Listenbetrag**, nicht verbindliche Ankaufzusage. `v_current_prices` enthält weiterhin keine `notes` und bietet alleine **keine Verbindlichkeits-/Bedingungs-/Aktualitätsgarantie**. Für variantenspezifische Preisnotizen muss auf die zum Current-Pointer gehörende `prices`-Row gejoint werden; für alle materialweiten Variantenbedingungen zusätzlich `trader_materials`. Beispiel ausschließlich zur späteren read-only Owner-Prüfung, hier auf keiner Produktions-DB ausgeführt:

```sql
SELECT t.slug, m.slug AS material_slug, p.variant, p.price, p.unit,
       p.price_kind, p.confidence, p.notes, tm.conditions,
       p.observed_at, p.published_at, p.extra_json
FROM current_prices c
JOIN prices p ON p.id = c.price_id
JOIN traders t ON t.id = c.trader_id
JOIN materials m ON m.id = c.material_id
LEFT JOIN trader_materials tm
  ON tm.trader_id = c.trader_id AND tm.material_id = c.material_id
WHERE t.slug = 'by-regensburg-kernschrott-recycling';
```

### F4 — quellenzustandsgetreue Class-3-Begründung

Quarantäne setzt nun die **exakt validierte gefährliche Class-3-Beschreibung**, einen lesbaren veröffentlichten Betrag und den **validierten globalen Ausschluss** voraus. Erst dann behauptet der Skip den tatsächlichen Widerspruch. Beschreibung/Ausschluss/Preisform geändert: gesamter Parse blockiert mit Neubewertungsaufforderung, keine veraltete Widerspruchsbegründung als geprüftes Faktum. Kein Class-3-Preis und keine bestätigte Class-3-Annahme.

### Aktuelle Verifikation nach Review-Fixes

Eigener bestehender Target und Speicherlimits unverändert wie oben. Keine Produktions-DB geöffnet, keine externen Geocodingläufe, keine Commits/Push/Deploy. Fixture unverändert; Mutationen ausschließlich in Teststrings. Aktuelle Test-/Live-Logs separat von Erstverifikation, nach sämtlichen Code-/Test-/Dossieränderungen ausgeführt:

- `cargo test -p schrott-mcp-ingestion kernschrott -- --nocapture`: Exit **0**, **9 bestanden / 0 fehlgeschlagen / 1 expliziter Livetest ignoriert**. [Gezielte Review-Fix-Tests](kernschrott-targeted-review-fixes-20261010.log) nennen alle drei echten Record-/Persistenztests.
- `cargo test --workspace --quiet`: Exit **0**, **572 bestanden / 0 fehlgeschlagen / 5 explizite Netzwerktests ignoriert**, davon **515 Ingestiontests**; Doc-Tests ebenfalls erfolgreich. [Workspace-Review-Fix-Log](kernschrott-workspace-review-fixes-20261010.log).
- `cargo test -p schrott-mcp-ingestion live_handler_verifies_identity_prices_and_date -- --ignored --nocapture`: Exit **0**, **1 bestanden**, tatsächlicher Handler einschließlich Live-Impressumsprüfung. HTTP **200**, **40824 Bytes**, `/elektro`, `published_at=2026-10-06T00:00:00+00:00`, **38 Preise / 19 bedingte Materialannahmen / 1 Class-3-Skip**, davon **3 Anfragefraktionen**. Test prüft validierte globale Vorbehalte in allen Annahmen und Preisnotizen, echten Lüfter-Nullpreis und EUR/kg. [Vollständiges Live-Review-Fix-Ergebnis](kernschrott-live-review-fixes-20261010.log). Kein Store-/Produktionszugriff im Live-Test.
- `cargo fmt --all -- --check` / `git diff --check`: erfolgreich. Keine neue Dependency/Änderung `Cargo.lock`; kein Store-/Shared-Record-Diff.

Ausführungsumgebung für sämtliche Cargo-Tests:

```sh
CARGO_TARGET_DIR=/root/Documents/kernschrott-cargo-target-20261010 \
CARGO_BUILD_JOBS=1 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 \
CARGO_INCREMENTAL=0 /root/.cargo/bin/cargo test -p schrott-mcp-ingestion kernschrott -- --nocapture
```

SHA-256 des final getesteten Handlerfiles: `28ed6640fb3026b45167c38bfed77e88d1e6de2c7618f72a4798bf7c7929c47e`. SHA-256 der unveränderten Fixture: `b1a313c9cfc2d3dea5845984f9b98190eac7ba07daedddc61252e466a36db532` (identisch mit unabhängigem Review). Kein unveränderlicher Commitnachweis behauptet; Arbeit bleibt bewusst uncommittet.

### Offene Übergabegates

1. Koordinierender Owner: erneuter unabhängiger Review der S1/F1-Fixes und dokumentierten S2/F2/F3/F4-Grenzen, anschließend Main-Integration der uncommitteten Worktree-Arbeit; **hier bewusst kein Commit/Push/Deploy**.
2. GitHub-Authentifizierung extern beheben, soweit für Owner-Integration benötigt; lokale Verifikation davon unabhängig.
3. Koordinierter Rollout beider Binaries und anschließend erfolgreicher **regulärer** Produktions-Scheduler-Scrape mit den unten genannten read-only Nachweisen. Der lokale Livehandler-Test schließt dieses Produktionsgate ausdrücklich nicht.

## Produktionsgate

Nach koordiniertem Rollout read-only prüfen: Händler #5529 / Slug, regulärer Scheduler-Schritt mit `items_upserted=38` (bei unveränderter Quelle), **19 aggregierte bedingte Materialannahmen (davon 3 Anfragefraktionen)** und dokumentierter Class-3-Skip, keine Canary-Warnungen; aktuelle und historische Preise getrennt nach Material/Variante, EUR/kg, Quelle /elektro und `published_at=2026-10-06`, Basis `page_stated`. Vorbehalte über die konsumierte Queryfläche einschließlich oben dokumentierter Joins sichtbar machen; Scrape-Fehler und alte Beobachtungen dürfen keine aktuelle Bestätigung suggerieren. Kein manueller Prod-Scrape als Ersatz für regulären Scheduler-Nachweis.

Read-only Nachweisabfrage gegen `internal.db` (Zeit muss nach Deployment liegen):

```sql
SELECT id, run_id, status, items_upserted, message, started_at, finished_at
FROM ingestion_steps
WHERE scraper = 'by-regensburg-kernschrott-recycling'
ORDER BY id DESC LIMIT 3;
SELECT run_id, url, status_code, byte_len, fetched_at
FROM raw_fetches
WHERE scraper = 'by-regensburg-kernschrott-recycling'
ORDER BY id DESC LIMIT 3;
```
