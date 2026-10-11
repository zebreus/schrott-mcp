# MSG Mengenstaffeln und Current-Reconciliation — Fortschritt 10.10.2026

## Status

**Review-Korrekturen implementiert, unabhängiger Re-Review ausstehend:** R1/P1 und R2/P2 sind durch Verhaltensregressionen abgesichert; 12 MSG- und 5 Store-Snapshot-Tests grün, Workspace **623 bestanden, 0 Fehler, 5 ignoriert**. Die ursprünglichen unabhängigen Findings bleiben erhalten, der dort angehängte Behebungsabschnitt ist ausdrücklich Implementierungsagent-Evidenz, keine neue unabhängige Freigabe.

Begonnen auf sauberem main `6d5ef762`, neuer separater Worktree `/tmp/opencode/schrott-mcp-msg-tiers-20261010`, Branch `handler/msg-tiers-reconciliation-20261010`. Foundation-Worktree bleibt unverändert. Keine Commits/Pushes/Deployments, kein forced ingest, keine Produktionswrites oder GEO. Eigener Targetdir `/tmp/opencode/target-msg-tiers-20261010`.

## Auftrag / Abnahmekriterien

- Alle sichtbaren positiven dynamischen Staffeln, vollständige grade+quantity-Identität, gleiche Beträge getrennt, Duplicate-Konflikte fail closed.
- Preismaß aus expliziter Betreiberquelle; Mengenmaß separat, keine EUR/t-Inferenz aus Mengen-t.
- Echte gespeicherte Fixture, Test-first Parser→record→TempDB und historieerhaltende Reconciliation; fremde Quellen/Händler bewahren.
- Bei Fetch-/Parser-/Mapping-/Schreibfehlern keine Current-Retirement; keine Kataloglücken durch behelfsmapping verdecken.
- Live-Inventur, gezielte und Workspace-Tests, Fmt/Diff, präzise Vollständigkeitsgrenzen.

Erster Stand vor unabhängiger Review: 9 MSG-, 3 Store-Snapshot-Tests, Workspace 618 bestanden. Diese Evidenz erkannte R1/R2 nicht und ist keine Freigabe; aktuelle Behebungsevidenz unten. Keine Owner-Abnahme/Produktionskorrektur beansprucht.

## Tatsächlicher Quell-/Produktionsbefund

- Primärquelle `https://www.msg-metallrecycling.de/preisliste`, heute live geladen und unverändert als `crates/ingestion/src/traders/handlers/fixtures/msg-preisliste-20261010.html` gespeichert (238280 Bytes). Sichtbarer `.hinweis`: „freibleibende Preise pro Kilogramm.“ Mengen-kg/t ist davon unabhängig.
- 66 Produktkarten sind auf `section#preiseSchrott` **und zwei nachfolgende `section#info.section_product4`** verteilt. Exklusive Auswahl von `#preiseSchrott` würde vier Besteck-Kataloglücken verbergen. Unabhängige Python-HTMLParser-Inventur: 38×3, 6×2, 7×1, 15×0 positive sichtbare Staffeln = 133.
- 124 Angebote auf 45 gemappten Sorten; neun positive Staffeln auf sechs bekannten Kataloglücken werden explizit diagnostiziert: Alu Cu Kühler (3), Stahl- und Gussspäne (2), Messer versilbert (1), Silberbesteck 800 (1), Versilbertes Besteck 60/40/20 (1), Versilbertes Besteck 80/90/100 (1). Keine Ersatzmaterialien eingeführt.
- Produktion nur `file:/var/lib/schrott-mcp/public.db?mode=ro` + `PRAGMA query_only=ON` gelesen: 46 Current-Keys, alle exact-source MSG-URL und Rohlabels mit `(ab N kg:)`; davon `messing/Wasseruhren` noch vom 29.09.2026, neben korrektem `messing-leicht/Wasseruhren`. Keine Produktionsänderung oder forced ingest.

## Implementierter Vertrag

- Variantentext = eigene normalisierte Quelltitelzeichenfolge + ` / ab <kanonische Menge> <kg|t>`. Keine begrenzte Schwellentabelle, keine Leaks, gleiche Beträge an verschiedenen Schwellen getrennt. Deutschen Mengenzahlenvertrag (Tausenderpunkt/Dezimalkomma) von englischen Dezimal-Preisfeldern trennen; ungültige/mehrdeutige Formen verhindern den Snapshot. Kanonische Mengenzeichenfolge wird ohne f64-Rundung gebildet; exakter Wert steht zusätzlich in `extra_json.quantity_min_text`, numerischer Abfragewert in `quantity_min`, Einheit in `quantity_unit`. Menge wird nicht mit EUR/kg↔EUR/t normalisiert.
- Sichtbarkeit gilt für Zeilen/Zellen/Vorfahren und deren Textkinder. Positive sichtbare Felder werden vollständig übernommen; leere CMS-Felder/Request-only und Nullwerte diagnostiziert, Null blockiert spätere positive Staffel nicht. Kein Preisdatum aus „tagesaktuell“ erfunden.
- Konfligierende Duplicate-Keys sind Parserfehler, identische Wiederholungen werden dedupliziert. Neue unbekannte bepreiste Sorten sind Mappingfehler, bekannte belegte Kataloglücken eigene Diagnostics.
- Nur MSG wird aus `record()` auf fail-closed `record_snapshot` geroutet. Bestehende Beobachtungssemantik aller anderen Handler bleibt erhalten. MAP_VERSION und SQL-Schema unverändert.
- Enge Store-Seam `record_price_snapshot` validiert einheitlichen Händler/Quelle/Zeitpunkt, eindeutige Keys und Producer-Marker. Die bestehenden Preis-Schreiboperationen laufen zusammen mit Current-Retirement in **einer Transaktion**. Schreib-/Retirementfehler rollen beide zurück. Fremdquellen-Kollisionen und verspätete Snapshots werden ohne Current-Änderung abgewiesen.
- Retirement nur exact-source/published/haendler_angabe MSG-Current: nachweisbare Legacy-Rohlabel+alte Grade/Materialkombination (Wasseruhren expliziter Fehlmappingfall) oder eigener `snapshot_owner=msg-visible-tiers-v1`-Marker und im neuen Snapshot fehlender Key. Kein globaler Current-Rebuild, kein DELETE/Remap historischer Preise, keine Änderungen fremder Händler/Quellen. Unbewiesene gleichquellige Keys bleiben stehen.
- Unveränderte aktuelle Preise werden gemäß bestehendem Store-Vertrag refreshed statt als neue Preisänderung historisiert. Snapshot-Metadaten werden dabei per JSON-Merge gestempelt, auch wenn eine gleichwertige Current-Zeile bisher keinen Producer-Marker hatte; vorhandene Datumsbasis bleibt erhalten. Ein eigener Regressionstest beweist anschließendes Retirement ohne Legacy-Liste und erhaltene History.
- Katalog-/Baselinefehler und verdächtige Volumenänderung (<50 % oder >3× ab Baseline 5) verhindern Retirement. Annahme-/Kontaktwrites werden vor der abschließenden Preistransaktion fallibel ausgeführt: auch deren Fehler verhindern Retirement; bereits erfolgreiche Enrichment-Writes sind bei späterem Preisfehler nicht Teil des Price-Rollbacks. Sie verändern keine Current-Preise oder historische Preiszeilen.

## Test-first und Infrastrukturgrenze

- Der erste real-fixture Red-Lauf wurde vor Parserimplementierung gestartet, scheiterte aber am Linker bei vollem `/tmp`; keine behauptete Verhaltens-Red-Evidenz daraus. Store-Seam-Test wurde vor API-Implementierung ausgeführt und war erwartbar wegen fehlender Methode rot (`/tmp/opencode/msg-snapshot-red.log`).
- Beim ersten Parserlauf nach Implementierung war der echte Fixturetest zusätzlich **rot (129 statt 133)**: er deckte die unerwarteten `#info`-Karten auf. Danach Scope korrigiert und alle neun MSG-Tests grün. Logs `/tmp/opencode/msg-tiers-first-green.log` (Name historisch, enthält den Red-Fund) und `/tmp/opencode/msg-tiers-targeted.log`.
- `/tmp` war durch beide eigenen Cargo-Caches voll. Nur eigener abgeschlossener Foundation-Buildcache nach `/root/Documents/target-owned-variant-20261010-cache` verschoben; keine fremden Dateien entfernt. Abgeschnittenen eigenen Store-Patch wiederhergestellt und vollständige Tests/Quell-Diffprüfung erforderlich. Kein fremder Worktree verändert.
- Der neue eigene MSG-Targetcache wurde anschließend nach `/root/Documents/target-msg-tiers-20261010-cache` verschoben; `/tmp/opencode/target-msg-tiers-20261010` ist nun ein Symlink darauf. Targetdir und Buildlogs bleiben eindeutig diesem Agenten zugeordnet; keine weiteren Infrastrukturabbrüche.

## Bisherige Testabdeckung

- Reale gespeicherte Quelle Parser→Outcome→`record`→TempDB: alle 124 gemappten Angebote pro Key/Preis/Rohlabel/Quelle geprüft, alle 46 unabhängig erfassten Legacy-Keys retired und Originalgeschichte unverändert vorhanden. Fremdhändler/-quelle und unbewiesener MSG-Key geschützt, Repeat ohne neue History, neue 137,5-t-Schwelle/enge Schreibweisenänderung retired nur frühere eigene Keys, FE 0,19 EUR/kg→190 EUR/t behält Menge 100 kg.
- Sichtbare kg/t-Alternative, hidden row/wrapper/dummies; drei Millberry-Staffeln, einpreisige Bronze, erste leere FE-Staffel, gleiche Beträge verschiedenartige Schwellen, Null-first→positive spätere Staffeln, widersprüchliche/identische Duplicate-Keys, unbekannte Sorten/Einheiten/Zahlen.
- Mapping-/HTTP-Status-/Parser-/verspätete Snapshot-/Volumeneinbruchfehler ohne Current-Retirement. Store-Trigger injizieren Fehler bei zweiter Preiszeile und beim DELETE: keine neuen Current-/History-Preise verbleiben, Legacy bleibt. Fremdquellen-Keykollision ebenso ohne Retirement.
- TempDB read-only geöffnet: echte Enrichment-Schreibverweigerung verhindert Retirement; weitere Regressionen für owned-spezifische Jump-Canary, Kalenderdatumsinferenz und A→B→A-Historie durch den MSG-Snapshot-Pfad.

## Live-Vollständigkeitscheck, nicht nur Count

Echter Handlerlauf ohne DB: HTTP 200, 238280 Bytes, 124 Preisangebote, kein Seitendatum, Betreiber-Kontakt Gotha kongruent (`/tmp/opencode/msg-tiers-live.log`). Die 124 erwarteten gemappten Quellzeilen wurden **einzeln als Multimenge aus Rohlabel einschließlich Schwelle und EUR/kg-Betrag** mit dem Live-Handleroutput verglichen; keine fehlenden/unerwarteten Angebote. Alle neun Katalogdiagnosen sind ebenfalls gegen konkrete Quellzeilen geprüft. Der Beispieloutput verkürzt Labels auf 48 Zeichen und zeigt keine Varianten; Variantentext und vollständige Rohlabels werden daher zusätzlich im Rust-Parser→Store-Test pro Angebot geprüft, nicht aus dem Live-Count behauptet.

Reproduzierbarer unabhängiger stdlib-HTMLParser-Audit: `research/msg_tier_inventory.py`. Aufruf:

```sh
python3 research/msg_tier_inventory.py \
  /tmp/opencode/msg-tier-live-audit-source-20261010.html \
  /tmp/opencode/msg-tiers-live.log
```

Vollständiger Inventuroutput (66 Karten mit allen sichtbaren Staffelbeträgen, Null-/Hidden-Anteilen, neun Kataloglücken): `/tmp/opencode/msg-tiers-live-inventory.json`. SHA-256 des erneut geladenen Live-HTML und gespeicherten Fixtures identisch: `51e9d62c01112fc29ef23e1a64c0cc400ec65b9083df2502a93c64c3e79b7212`. Das ist heutige Quellvergleichsevidenz, keine Garantie einer künftigen Webseite.

### Vollständigkeitscheckliste

- [x] Drei Millberry-Staffeln (9,90/10,10/11,00), Bronze nur ein Angebot, FE erste echte Schwelle 100 kg, fallende Rotguss-Stücke (7,20→6,50→6,70); Live-Audit umfasst sämtliche Quellzeilen.
- [x] Beliebige neue positive Schwellen, deutsche Dezimal-/Gruppenzahlen, kg/t bei unverändertem EUR/kg-Quellpreis, gleichpreisige unterschiedliche Schwellen getrennt.
- [x] Erkannte Titelumbenennung bekommt eigene Identität; vorherige eigene Current-Angebote entfernt, History erhalten. Unbekannte bepreiste neue Sorte verhindert Snapshot statt Behelfsmapping.
- [x] Leere/Null/Request-only/hidden Dummywerte keine Phantompreise; Zero-first unterdrückt spätere positive Angebote nicht.
- [x] Konfligierende Doppelidentitäten fail closed, identische Wiederholungen dedupliziert.
- [x] Alle 46 tatsächlich belegten Legacy-Keys nachgebaut und entfernt, alte History weiter vorhanden; Wasseruhren falsch UND korrekt staffellos erfasst.
- [x] Fremdquelle, Fremdhändler, unbewiesener MSG-Key geschützt; Wiederholung idempotent und spätere Source-Snapshots reconciled.
- [x] Parser-/Mapping-/Status-/Katalog-/Baseline-/Enrichment-/Preis-/Retirementfehler und verspätete Snapshots ohne Current-Retirement.
- [ ] Owner-Review/Integration, erneute Main-Tests, gesondert freigegebener regulärer Produktionslauf und read-only Nachprüfung. **Nicht Teil dieses Agentenauftrags ausgeführt.**

## Risiken / Abgrenzung

- Betreiber-Preis-/Kontaktseite sind Primärquellen für die Angebote. Es gibt hier keine neue Betreiberidentitäts-/Dossierentscheidung und keine Behauptung zweier unabhängiger Identitätsbelege.
- Bewusst fail-closed bei Änderung der expliziten Einheitshinweis-/Webflow-Zellenstruktur oder neuer unbekannter bepreister Sorte; eine echte massive Sortimentsänderung unter 50 % Baseline erfordert Review statt automatischem Retirement.
- Sichtbarkeitsvertrag betrifft Webflows belegte `w-condition-invisible`-Klasse; keine allgemeine Browser-CSS-/JavaScript-Sichtbarkeitssimulation.
- Historische Preisbeobachtungen unter falschem Wasseruhren-Material werden **nicht umgeschrieben**. Der falsche Current-Zeiger verschwindet; alte Rohlabels/Materialzuordnung bleiben forensisch erhalten.
- Kein direktes Reparatur-SQL, keine Migration am Boot, kein forced ingest: erste erfolgreiche normale MSG-Ingestion mit diesem Code führt die eng begrenzte Transaktion aus. Bei Ausfall bleibt der bisherige Current-Stand stehen und der Schritt ist failed.
- Atomarität gilt Preisbeobachtungen + Current-Zeiger. Ancillary Annahme-/Kontakt-Enrichment kann bei einem späteren Preisfehler schon aktualisiert sein, aber kein Retirement auslösen (siehe Implementierungsvertrag).
- Es bleiben neun positive Quellstaffeln wegen fachlicher Kataloglücken ohne Current-Preis; vollständige Abdeckung **der heutigen katalogisierten Angebote**, nicht aller 133 Sorten-/Staffelangebote als DB-Preise.

## Historische Agenten-Evidenz vor unabhängiger Review

| Prüfung | Ergebnis | Artefakt |
|---|---|---|
| MSG-Modultests, finale Fassung | 9 bestanden | `/tmp/opencode/msg-tiers-targeted-final.log` |
| Store-Snapshot-Tests, finale Fassung | 3 bestanden | `/tmp/opencode/msg-snapshot-targeted-final.log` |
| Workspace, finale Fassung | **618 bestanden, 0 Fehler, 5 ignoriert** (4+3+558+4+16+33) | `/tmp/opencode/msg-tiers-workspace-final.log` |
| `cargo fmt --all -- --check`, `git diff --check` | grün | Session-Toolausgabe |
| Vollständige `Box::leak`-Suche unter `crates/` | keine Treffer | Session-Toolausgabe |
| Letzter echter Handlerlauf + unabhängiger Audit | HTTP 200; 124/124 gemappte Quellangebote übereinstimmend, neun Katalogdiagnosen | `/tmp/opencode/msg-tiers-live.log`, `/tmp/opencode/msg-tiers-live-inventory.json` |
| main-Zustand nach Arbeit | sauber, weiterhin `6d5ef762` | Session-Toolausgabe |

Testlogs sind **Implementierungsagent-Evidenz**, keine vom Owner unabhängig ausgeführten Tests. Keine AGENTS.md im Worktree gefunden; README-Preis-/Datums-/Quellenvertrag erneut gelesen. Kein allgemeiner Material-/Parsingrefactor und kein Dossier-/Katalogwechsel.

Manueller finaler Diff-Audit: nur drei Rust-Dateien geändert: MSG-Handler inklusive Tests; Record-Dispatch nur für exakten MSG-Slug; Store-Seam + Tests. Der bisherige `record_price`-Körper ist bis auf Connection-Übergabe unverändert, Schema-/Alt-Migrationen und alle bestehenden Tests unberührt. Der beim ENOSPC zunächst abgeschnittene Dateischluss ist vollständig zur Baseline identisch wiederhergestellt, im finalen Diff keine dortigen Löschungen. Alle anderen Handler, auch Gutzmann, unverändert. Neue Dateien: echte Fixture, unabhängiges Inventurskript, dieser Bericht.

Owner-Review bleibt erforderlich, insbesondere für producer-/legacy-genauen Retirement-Vertrag und die bewusst vorgezogenen Ancillary-Writes. Ein eigenes unabhängiges Review wird nicht behauptet (diese Agentensession erlaubt keine weitere Subagent-Tiefe). Keine Commits/Staging/Pushes/Deployments. Vollständiger Review-Patch einschließlich neuer Dateien: `/tmp/opencode/msg-tiers-reconciliation-20261010.patch`.

## Grundlagen

`README.md`, `research/checkin-20261010-msg-owner-fallback.md`, aktuelle MSG-Handler-/Record-/Store-Quellen werden erneut geprüft. Der Sichtbarkeitsfix und Cow-Fundament gehören bereits zur Baseline.

## Behebung R1/P1 und R2/P2 — aktueller Patchstand

### R1: vollständiger sichtbarer Kartenvertrag, kein Count-Pflaster

- Jeder sichtbare Kartenchild muss Titel, bekannte `pricedisplay`-Zeile, textlose Dekoration oder **explizit sichtbares** `Preis auf Anfrage` sein. Uninterpretierbare sichtbare Felder führen zu Parse-Fehler, auch wenn zusätzlich Anfrage sichtbar ist. Keine automatische Anfrage-Inferenz aus null erkannten Zeilen; keine feste 124-Produktionscountregel.
- Echte Millberry-Karte mit allen drei Klassen `changed-pricedisplay`: nach realem Fixture→Parser→real `record`-Routing bleiben **sämtliche Spalten aller Current- und History-Zeilen** gegenüber voraufgezeichneter TempDB identisch, Fehler erforderlich. Vor Fix tatsächlich Verhaltens-RED, nicht Compilerfehler: `/tmp/opencode/msg-review-red-damaged_card_structure_cannot_publish_or_retire_any_current_or_history.log`.
- Zusätzliche Negativfälle: einzelne beschädigte Klasse; alle Preiszeilen hidden ohne sichtbare Anfrage; beschädigte Preiszeilen plus sichtbare Anfrage; unmarkiert leere Beträge. Alle fail closed ohne Current-/History-Änderung.
- Positivfälle mit realem Record: bekannte Preiszeilen hidden **und explizite sichtbare Anfrage**; `w-dyn-bind-empty`-markierte leere CMS-Beträge; explizite Nullbeträge; explizit verborgene ganze Karte. Diese vier verifizierten Nicht-Angebotszustände haben getrennte Diagnostik, keine Phantompreise; legitimes Retirement entfernt nur betroffene eigene Current-Zeiger und erhält deren komplette History. Hidden Preiszeilen allein sind dagegen kein nachgewiesener Nicht-Angebotszustand.

### R2: belegte exakte Titel und enge Schreibweisen

- `grade_for` enthält ausschließlich die 45 heute bepreisten katalogisierten Quelltitel plus belegtes `Hartmetall Schlamm`. Vergleich normalisiert nur Groß-/Kleinschreibung und Whitespace; validierte Legacy-Staffelannotation wird separat abgespalten. Keine `contains`-Materialheuristiken, erfundene NEU-Aliasse oder neue Materialien.
- `Alu Kabel mit Kupfer`, `V2A/V4A Edelstahl`, `Millberry mit Messing`, `Kupfer Schwer mit Alu`, `Alu Bleche blank mit Kupfer` werden im echten Parser→Record-Pfad abgewiesen; zusätzlich wird ein manipuliertes Outcome direkt an real `record` gegeben und dort ebenfalls abgewiesen. Komplette Current-/History-Tabellen bleiben unverändert. Verhaltens-RED vor Fix: `/tmp/opencode/msg-review-red-composed_unknown_material_titles_cannot_publish_or_retire.log`.
- Eng bedeutungserhaltende Änderung `Millberry`→`MILLBERRY` sowie `Alu Bleche blank`→`ALU BLECHE BLANK` bleibt erkannt, erhält neue Variante und reconciled vorherige eigene Keys. Allgemeine Umbenennungen bleiben absichtlich mappingpflichtig.
- Deutsche äquivalente Mengen `1000,50` und `001.000,500` ergeben genau `ab 1000.5 kg`: unterschiedliche Preise verhindern Record; gleiche Preise deduplizieren im realen TempDB-Record zu einer Identität/Historybeobachtung.

### Unabhängiger 46-Key-Oracle und Producer-Grenze

- Neue gespeicherte Fixture `crates/ingestion/src/traders/handlers/fixtures/msg-legacy-current-20261010.json`, read-only Produktions-SELECT vom **2026-10-10T20:46:30.758868+00:00**. Nicht aus Parser/`grade_for` erzeugt. Enthält exakt 46 Key-/Beobachtungszeilen mit realen Preisen, Einheiten, Material-/Variantenstrings, SourceURL, Rohlabels, Mappingmetadaten, Datumsfeldern und Produktionspreis-ID. Wiederholte Felder verlustfrei als Defaults gespeichert; alle 20 SELECT-Spalten unabhängig gegen expandierte Fixture erneut identisch verifiziert.
- Oracle-SHA256 `9c7b1fe9deb2a6ae3a674fd806bd88039ecd9d91bb823d81a435e1e126f802bd`. Falsches `messing/Wasseruhren` behält echten 29.09.-Zeitpunkt und leeres Mappingobjekt, korrektes `messing-leicht/Wasseruhren` tatsächlichen 10.10.-Zeitpunkt/`map_v1`. TempDB testet alle 46 erfassten Keys und vollständige historische Zeilen vor/nach Retirement inklusive SourceURL; lokale IDs sind naturgemäß nicht Produktions-IDs.
- Store verweigert Current-Key-Kollision mit anderem **nichtleerem** `snapshot_owner`, selbst bei gleicher SourceURL. Regression vor Fix Verhaltens-RED (`/tmp/opencode/msg-producer-collision-red.log`), danach vollständige Current-/History-Tabellen unverändert. Fremder Producer wird auch bei explizit übergebenem Legacy-ID nicht retired. Same-source-Adoption ohne Marker **und mit explizit leerem Marker** weiterhin grün; ursprüngliche Datumsbasis bleibt erhalten.

### Neue Agenten-Evidenz / Vollständigkeit

| Prüfung | Ergebnis | Artefakt |
|---|---|---|
| MSG, inklusive R1/R2/Non-offer/Duplicate | 12 bestanden | `/tmp/opencode/msg-review-targeted.log` |
| Store Snapshot/Producer/Adoption | 5 bestanden | `/tmp/opencode/msg-review-store.log` |
| Workspace | **623 bestanden, 0 Fehler, 5 ignoriert** (4+3+561+4+16+35) | `/tmp/opencode/msg-review-workspace.log` |
| Echter Livehandler ohne DB + unabhängiger Einzelzeilenvergleich | HTTP 200, 124/124 Quellangebote vollständig übereinstimmend, 9 belegte Gap-Zeilen, 66 Karten/133 positive Staffeln | `/tmp/opencode/msg-review-live.log`, `/tmp/opencode/msg-review-live-inventory.json` |

Live-HTML `/tmp/opencode/msg-review-live-source.html` ist zur gespeicherten echten Fixture byteidentisch (SHA256 `51e9d62c01112fc29ef23e1a64c0cc400ec65b9083df2502a93c64c3e79b7212`). Nicht aus dem Count allein behauptet: Audit vergleicht Label inklusive Schwelle und Betrag als Multimenge; vollständige Varianten/Store-Keys, Maße und Historienschutz zusätzlich in Rust getestet. Neun fachliche Kataloglücken unverändert explizit diagnostiziert, nicht behelfsgemapped.

Neue Fixture und Behebungsabschnitt ergänzen den ursprünglichen Diff; keine weiteren Rust-Dateien, keine MKM-/Pipeline-/Mainänderungen durch diesen Agenten. Fmt/Diff- und Patchprüfung werden vor Übergabe erneut ausgeführt. Kein Commit/Staging/Push/Deploy/forced ingest/Produktionswrite. **Owner-Re-Review und Integration bleiben ausstehend.**

## Owner-Behebung P1/R1b — 11.10.2026

Nach dem zweiten unabhängigen Review übernimmt der Owner den verbliebenen konkreten Fehler selbst. Neuer echter Fixture→Parser→Record→TempDB-Test `unconsumed_visible_tier_fields_cannot_retire_current` zuerst ausgeführt und **Verhaltens-RED**: `visible unconsumed price must block snapshot`. Die Millberry-Mutation ersetzt den Preisdiv durch einen markierten leeren div plus weiterhin sichtbaren 9,90-Preis in einem direkten span; zusätzlich direkte Textmutation. Vorbefüllte 124er-Current-/History-Tabellen und erfolgreiche Baseline vorhanden.

Fix: Jede Preiszeile konsumiert jetzt ausschließlich ihre direkten sichtbaren div-Zellen, prüft aber **alle** direkten Knoten. Unzugeordnete sichtbare Elementinhalte oder direkte Textknoten führen vor Outcome/Record zum Fehler. Versteckte Alternativen bleiben ausgeschlossen. Textdescendants innerhalb eines konsumierten Wertdivs bleiben erlaubt und werden positiv getestet; keine doppelte Erfassung verschachtelter divs durch Descendant-Selector mehr. Kein Count-Pflaster, kein zusätzlicher Store-/Retirementwechsel.

Owner-Tests auf genau diesem Worktree: **13 MSG-Tests bestanden**, einschließlich beiden neuen Negativproben mit kompletter Current-/History-Gleichheit und positivem inline-span im konsumierten Preisfeld. **624 Workspace-Tests bestanden, 0 Fehler, 5 ignoriert**, `/root/Documents/msg-owner-r1b-workspace-20261011.log`. Rustfmt angewandt, anschließender Format-/Diffcheck grün. Diese Läufe sind tatsächliche Owner-Evidenz auf dem Worktree, noch keine Main- oder Produktionsevidenz.

Die historischen Reviewberichte bleiben unverändert erhalten. Der alte exportierte Patch ist nach diesem Owner-Fix nicht mehr maßgeblich; für den nächsten Review tatsächliches `git diff` und untracked Dateien lesen. **Erneute unabhängige R1b-Abnahme, vollständiges Owner-Diffgate, Main-Tests und Veröffentlichung/Deployment weiterhin offen.** Keine Produktionswrites, kein Commit/Push/Redeploy.

## Abschließendes Owner-Worktree-Gate 11.10.2026

Dritter unabhängiger Review erteilt eng begrenzte technische MSG-Retirement-Freigabe; historische R1/R1b/R2-Blocker durch reale Negativ-/Positivproben geschlossen. Owner hat den tatsächlichen Store-/Dispatch-Diff, gesamten MSG-Handler einschließlich Tests, Legacy-JSON und Auditcode gelesen. Source-/Producergrenzen, Price-Transaktion, bewusst getrennte Ancillary-Writes und neun diagnostizierte Kataloglücken akzeptiert; keine generische Snapshot-Freigabe für MKM oder andere Handler daraus abgeleitet. Kein Schema-/Mappingversions-/GEO-/Dossierwechsel. Die unveränderte HTML-Fixture bleibt Originalquellartefakt.

Frischer **Owner-Liveabruf** mit tatsächlichem Handler HTTP 200: `/root/Documents/msg-owner-live-20261011.log`. Separat neu geladenes HTML `/root/Documents/msg-owner-source-20261011.html`, SHA256 `51e9d62c01112fc29ef23e1a64c0cc400ec65b9083df2502a93c64c3e79b7212`; stdlib-Audit `/root/Documents/msg-owner-inventory-20261011.json` bestätigt **66 Karten, 133 positive Quellstaffeln, 124/124 katalogisierte Angebote einzeln übereinstimmend, neun konkrete Gap-Zeilen**. Variant-/Normalisierungs-/Retirementbeweis zusätzlich in echten Rust-DB-Tests, nicht aus verkürztem CLI-Label abgeleitet.

Zusammen mit 624 Owner-Workspace-Tests und grünem Format-/Diffgate ist dieser genaue Worktree-Stand zur Main-Integration freigegeben. **Main-Tests nach Integration und Veröffentlichung/Deployment samt regulärem Produktions-Scrape bleiben offen.** Kein Erfolg für Produktionsbereinigung behauptet; GitHub-Authentifizierung zuletzt weiterhin fehlend.

Beim zusätzlichen **staged** Diffcheck der nun ebenfalls erfassten neuen Dateien wurden 14 ursprüngliche Trailing-Whitespace-Zeilen ausschließlich im unveränderten HTML-Quellartefakt sichtbar (CSS-Zeilen 91/92/152/156/177/207/213/228/235–239/244). Bewusste Originalartefakt-Ausnahme: keine Byteänderung zur kosmetischen Gate-Beruhigung; Hash-/Livevergleich bleibt identisch. Staged Diffcheck aller übrigen Code-/Test-/Berichtsdateien ist separat erforderlich und grün. Frühere unstaged Diffchecks erfassten diese untracked Fixture noch nicht; keine Behauptung eines uneingeschränkt grünen Checks sämtlicher Originalquellbytes.

## Main-Abnahme 11.10.2026

Integriert als **`1eb5198a`** (separater Quellcommit `a946f04d`). Danach tatsächlicher Owner-Main-Lauf `cargo test --locked --workspace`: **624 bestanden, 0 Fehler, 5 ignoriert**, `/root/Documents/msg-main-workspace-20261011.log`; Seed-Gate `cargo test --locked -p schrott-mcp-ingestion seed`: **13 bestanden**, `/root/Documents/msg-main-seed-20261011.log`. Rustfmt-Check grün, Main sauber nach Integration. Die registrierte Main-Implementierung erfasst damit Staffelangebote; **installierte Produktion weiterhin alter Code**, keine Current-Reconciliation dort erfolgt.

`GIT_TERMINAL_PROMPT=0 git push origin main` scheitert erneut an fehlendem HTTPS-GitHub-Benutzernamen. Push, koordinierter Server-/Worker-Redeploy und regulärer MSG-Produktionslauf mit read-only artikel-/staffelbezogener Current-/Historienspotprüfung bleiben erforderlich. Produktion nicht manuell beschrieben und keine erzwungene Ingestion.
