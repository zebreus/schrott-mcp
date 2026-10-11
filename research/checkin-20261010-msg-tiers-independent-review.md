# MSG Mengenstaffeln — unabhängiger Read-only-Review, 10.10.2026

## Ergebnis

**Keine Freigabe der automatischen Current-Reconciliation: ein unabhängig reproduzierter P1-Blocker.** Die SQL-Atomarität und die heutige Staffelabbildung sind weitgehend überzeugend; der Parser liefert jedoch auch bei einer beschädigten Karte einen vermeintlich vollständigen Snapshot. Das neue Retirement macht daraus den Verlust gültiger Current-Zeiger. Zusätzlich besteht eine P2-Grenze des behaupteten fail-closed Mappingvertrags.

Keine Produktionszugriffe, Netzwerkabrufe, Commits, Staging oder Änderungen an Implementierung/Fixture. Im Worktree ausschließlich dieser Bericht geschrieben. Owner-Review und erneute Main-Tests bleiben erforderlich. Keine Freigabe anhand eines Counts oder des Implementierungsagentenberichts.

## Prüfstand und Evidenztrennung

- Worktree `/tmp/opencode/schrott-mcp-msg-tiers-20261010`, HEAD `6d5ef762ac2d9b87386491322e4aed1f340b5569` plus uncommitted Patch. Alle drei Rust-Diffs vollständig gelesen: MSG-Handler, MSG-Dispatch in `crates/ingestion/src/traders/mod.rs`, Store-Seam in `crates/store/src/public/prices.rs`; relevante unveränderte Store-, Scheduler- und README-Verträge ebenfalls geprüft.
- Die drei Rust-Diffs sind **byte-identisch zum Präfix** von `/tmp/opencode/msg-tiers-reconciliation-20261010.patch`. Fixture-SHA256 unabhängig berechnet: `51e9d62c01112fc29ef23e1a64c0cc400ec65b9083df2502a93c64c3e79b7212`. Der mitgeteilte zwischenzeitliche Main-Stand `725b126` ist nicht Grundlage dieses Diff-Reviews; MKM-Scripts gehören nicht zum Scope.
- Bestehende Logs tatsächlich gelesen/ausgewertet: `/tmp/opencode/msg-tiers-targeted-final.log` (9 MSG-Tests), `/tmp/opencode/msg-snapshot-targeted-final.log` (3 Store-Tests), `/tmp/opencode/msg-tiers-workspace-final.log` (618 bestanden, 5 ignoriert). **Dies bleibt Implementierungsagent-Evidenz**, kein unabhängig neu gebauter Workspace-Lauf.
- Eigenständig ausgeführter Fixture-Audit: `research/msg_tier_inventory.py` gegen die gespeicherte Fixture und das bestehende Live-Log; zusätzliche explizite, manuell festgelegte 45-Titel→Material-Tabelle gegen sämtliche 124 Logzeilen als Multimenge `(material, vollständiges Rohlabel/Schwelle, Decimal-Preis)`. 124/124 stimmen überein; bei den gemappten Labels entsteht keine 48-Zeichen-Trunkierungskollision. Der HTTP-Live-Lauf selbst bleibt Agent-Evidenz, kein eigener heutiger Fetch.
- Eigene Rust-Reproduktionen ausschließlich unter **separatem** `/tmp/opencode/target-msg-independent-review-20261010`: `parser-review`, `end-to-end-review`, isolierte SQLite-Dateien in `db-repro/`. Parserfunktionen wurden unverändert aus dem aktuellen Source in einen stdin-Harness übernommen. Der zweite Harness verwendet die echten `HandlerOutcome`-/`ScrapedPrice`-Typen und den **realen kompilierten öffentlichen `traders::record`-Pfad**, `PublicDb`, `InternalDb`, Katalog-/Händlerseed und Tokio. Bereits vorhandene Agenten-Rustartefakte wurden nur als Linkabhängigkeiten gelesen, nicht umgebaut. Deshalb: eigene Verhaltensreproduktion, **kein vollständiger unabhängiger Cargo-Neubuild**.
- Erste Harness-Versuche scheiterten an PATH, gesplitteten Rust-Metadaten bzw. einem HTML-Serialisierungsvergleich; diese sind keine Verhaltensevidenz. Die schließlich ausgeführten Harnesses waren erfolgreich. Kein zusätzlichen Subagenten eingesetzt.

## Findings

### R1 — P1 / Blocker: fehlende Preiszeilen werden als belegtes Nicht-Angebot behandelt

**Source:** `crates/ingestion/src/traders/handlers/msg_metallrecycling_gotha.rs:716`, `:769–771`; nachgelagert `:253–263`. Retirement in `crates/store/src/public/prices.rs:557–565` und `:579–583`.

`parse()` iteriert nur erkannte `div.pricedisplay`. Wenn eine sichtbare, zuvor bepreiste Karte keine solchen Elemente mehr liefert, bleibt `kept=false` und `zero_skipped=false`; ohne weitere Prüfung wird „kein Preis: Preis auf Anfrage“ diagnostiziert. Das ist kein Beleg aus der Quelle: die Karte kann weiterhin alle positiven Beträge zeigen, nur mit geänderter CSS-Klasse. Es wird weder mindestens eine vorhandene Staffelstruktur verlangt noch ein **sichtbares tatsächliches** „Preis auf Anfrage“ als Ersatzbeleg geprüft.

**Unabhängig ausgeführte Reproduktion:**

1. Gespeicherte reale Fixture unverändert parsen: 124 katalogisierte Angebote.
2. Nur innerhalb der sichtbaren Millberry-Textkarte die drei Klassen `class="pricedisplay"` durch `class="changed-pricedisplay"` ersetzen. Titel, Schwellen, positive Werte und € bleiben unverändert. Kein HTML in Dateien verändert; Mutation nur im Arbeitsspeicher.
3. Exakter Source-Parser akzeptiert 121 Angebote und erfindet den Skip `Millberry (kein Preis: Preis auf Anfrage)`.
4. Echte isolierte Public-/InternalDB seeden; ursprünglichen 124er-Outcome über den realen MSG-`record`-Dispatch schreiben; einen erfolgreichen 124er-Step als Baseline hinterlegen.
5. Mutierten 121er-Outcome eine Stunde später über denselben realen Dispatch schreiben: **Erfolg**, `recorded=121`.
6. Alle drei Current-Keys `Millberry / ab 1 kg`, `/ ab 100 kg`, `/ ab 1000 kg` sind anschließend weg. Jeweils eine historische Zeile bleibt erhalten. 121/124 unterschreitet das 50%-Gate nicht.

Eigener Harness-Output:

```text
CONFIRMED: exact source parser -> REAL record_snapshot -> isolated DB; broken Millberry accepted121, all3 Current pointers retired, history kept, baseline124 gate passed
```

**Auswirkung:** Partieller Selector-/CMS-Strukturfehler wird als vollständige Sortimentsänderung veröffentlicht. Die Transaktion arbeitet korrekt, aber atomar mit einem falschen Snapshot. Das ist neu sicherheitsrelevant durch das eingeführte Retirement; der alte observation-only Pfad hätte die nicht wiedergefundenen Preise stehen gelassen.

**Erforderliche Abhilfe:** Ein nachvollziehbarer Nicht-Angebotsvertrag pro sichtbarer Karte: unerwartet fehlende Staffelstruktur/noch sichtbare uninterpretierte Preisfelder fail closed; explizite request-only/hidden/empty/zero-Zustände weiterhin getrennt behandeln. Keine starre 124er-Zahl als Ersatz. Regression **echte Fixture → Parser → Outcome → record → TempDB**, mit vorhandener Baseline und Vergleich aller Current- sowie History-Zeilen vor/nach dem Fehler. Eine legitime Quellumstellung auf sichtbares „Preis auf Anfrage“ muss separat positiv getestet werden.

### R2 — P2: „neue unbekannte bepreiste Sorte fail closed“ gilt nicht bei Keyword-Überlappung

**Source:** `crates/ingestion/src/traders/handlers/msg_metallrecycling_gotha.rs:51–52`, `:91–95`, `:365–374`, entsprechend weitere `contains`-Äste `:414–445`; Retirement erhöht das Risiko dieses unveränderten Mappings.

Der neue Fehlerzweig wird nur nach `grade_for(None)` erreicht. `grade_for` ist keine endliche Liste geprüfter Titel/Aliasse, sondern enthält breite Substring-Fallbacks. Eigener exakter Source-Parser-Harness: Nur den Titel `Alu Kabel` in der realen Fixture zu **`Alu Kabel mit Kupfer`** geändert; der unbekannte Mischsortentitel wird weiterhin ohne Fehler als `kabel-alu` veröffentlicht. Eine Kombination wie `V2A/V4A` würde ebenfalls vom ersten passenden Ast kategorisiert. Der Test `Unbekannte neue Sorte` trifft ausschließlich einen Titel ohne bekannte Keywords und beweist die behauptete allgemeine Fail-closed-Grenze daher nicht.

Die Substring-Tabelle ist bereits Baseline, **nicht neu eingeführt**. Neu sind jedoch der Anspruch der Vollständigkeitsvalidierung und das Retirement alter eigener Keys bei einer so „erkannten“ Umbenennung. Der aktuelle 45-Titel-Bestand ist davon nicht falsch gemappt; das Finding betrifft die Annahme künftig neuer/komponierter Sorten.

**Abhilfe/Owner-Entscheidung:** Explizit verifizierte Titel/Aliasse oder bewusst begrenzte erlaubte Umbenennungsformen; unbekannte Zusätze mit fachlichem Bedeutungswechsel nicht allein durch Keywords akzeptieren. Tests für unbekannte bepreiste Titel mit bekannten Keywords und Materialkonflikten. Wenn breite Matches bewusst beibehalten werden, den behaupteten Mapping-Fail-closed-Vertrag entsprechend einschränken; das ist dann keine allgemeine Sicherheit gegen Behelfsmappings.

## Geprüfte Sicherheitsgrenzen — kein weiterer bestätigter Transaktionsblocker

| Bereich | Befund |
|---|---|
| Dispatch | Nur exakter MSG-Slug in `traders/mod.rs:289–297`; andere Handler behalten den alten Record-Pfad. |
| Händler/Quelle | Store validiert einheitlichen Trader, exact Source-URL, Zeitpunkt, published/haendler_angabe; Current-Candidates sind exact-source und trader-scoped. Kein globaler Rebuild. |
| Producer/Legacy | Retirement nur Marker `msg-visible-tiers-v1` oder explizit übergebene Legacy-IDs. Handler Legacy-Erkennung verlangt parsebares Rohlabel/Staffel plus historische Material-/Variantkombination; Wasseruhren unter `messing` ist ausdrücklich begrenzt auf diesen exakten Titel und Variant. Unbewiesene same-source Keys werden nicht gelöscht. |
| Fremde Keykollision | Vor Schreibbeginn in derselben Transaktion blockiert ein Current-Key mit anderer Quelle, source_type, published-Status oder neuerem observed_at den ganzen Snapshot. Store-Test prüft kollidierende fremde Quelle und erhaltenen Legacy-Pointer. |
| Atomarität | Alle `record_price_on`-Writes, Same-price-Refresh/JSON-Merge und Current-DELETE in einer SQLite-Transaktion. Rückgabe über `?` vor Commit rollt zurück. Trigger-Test bei zweiter Preiszeile und beim DELETE liest erhaltene Legacy-Pointer/fehlende neue Pointer; kein beobachteter Autocommit-Leak im Preisbereich. |
| Ancillary | Annahme-/Fallback-/Kontaktwrites laufen fallibel **vor** der Preistransaktion. Ein solcher Fehler verhindert Retirement. Bereits erfolgreiche Enrichment-Writes bleiben bei späterem Fehler absichtlich bestehen; sie sind nicht atomar mit Preisen. Kein Finding allein wegen dieses ausdrücklich akzeptierten Scopes. |
| Late snapshots | Neuere managed/legacy Candidate-Zeit oder neuerer kollidierender Current-Key führt vor Preiswrites zum Fehler. Unbekannte same-source Keys außerhalb des managed/legacy Scopes sind kein globaler Snapshot-Watermark. Zeitvergleiche sind lexikalisch, passend zu hier von Utc erzeugten RFC3339-Strings; die generische Store-API validiert/normalisiert keine beliebigen Offset-Strings. |
| Volumengate | Fallibler Zugriff auf `last_ok_step_items`; bei Baseline ≥5 blockieren <50% oder >3×. Übergang 45 bisherige Angebote→124 liegt unter 3×. Keine allgemeine Vollständigkeitsprüfung: kleine Teilverluste passieren, ohne Baseline kein Gate. R1 beweist den praktischen Teilverlustfall. |
| Fetch-/Parsing-/Catalogfehler | `?` bei beiden Fetches/Impressum-Parsing verhindert Outcome; Scheduler ruft Record nur für erfolgreichen Outcome auf. Mapping-/Material-/Einheitenprüfung vor Preiswrites. Preisstatus wird zusätzlich im Record validiert. Bekannte sechs Kataloglücken sind Diagnosen, nicht Fehler und nicht Ersatzmaterialien. |
| Same-price-Refresh | Bestehender Preisrefresh-Vertrag bleibt: unveränderte aktuelle Zeile wird refreshed statt dupliziert. Snapshot-Seam stempelt danach den Marker via `json_patch`; Test beweist Markerübernahme bei vorher unmarkierter, gleichwertiger Zeile, erhaltene `published_at_basis`, späteres Retirement und erhaltene History. Merge gehört zur Transaktion. |
| Historische Felder | Retirement selbst löscht nur Current-Pointer. Legacy-Test vergleicht komplette historische Zeilen vorher/nachher. Refresh einer noch aktuellen gleichen Preiszeile ist bewusst keine immutable History; die generische bestehende Refresh-Semantik ist nicht als Fehler dieses Patches gewertet. |

**Producer-Grenze ausdrücklich:** Der Kollisionscheck ist source-, nicht producer-exklusiv. Ein gleichquelliger kollidierender Key wird auch dann nicht abgewiesen, wenn sein `extra_json.snapshot_owner` einen anderen Wert trägt; der Snapshot übernimmt den Marker. Das erklärt die bewusst erlaubte Markerübernahme, ist aber keine allgemeine Garantie „alle fremden Producer-Zeilen unberührt“. Aktuell nur ein MSG-Produzent; Owner sollte diese Seam nicht ohne zusätzliche Producer-Kollisionsentscheidung für mehrere Produzenten derselben URL verwenden. Nicht als heute unabhängig nachgewiesener Fremdproducer-Produktionsfall behauptet.

## Staffel-/Fixture- und Testabdeckung

- Eigenständiger Vergleich nicht nur von Counts: alle 124 gemappten Quellstaffeln mit **explizitem Grade-Oracle** gegen die vorhandenen Handler-Logzeilen geprüft. 45 Titel, EUR/kg-Beträge und Rohschwellen stimmen für die gespeicherte Quelle überein. 133 positive Quellstaffeln minus neun Diagnosen auf sechs Kataloglücken. Keine neue Ersatzmap für Alu-Cu-Kühler, Stahl-/Gussspäne oder Silber-/versilbertes Besteck.
- Materialabbildung manuell gegen alle 45 expliziten Mapping-Testfälle geprüft. Grade-/Quantity-Key wird aus normalisiertem Quelltitel plus kanonischer Schwelle gebildet, nicht aus Betrag; gleiche Beträge bei anderen Schwellen bleiben getrennt. Heute enthalten alle gemappten Quellschwellen ganzzahlige kg; neue t-/Dezimalschwellen sind Mutationsfälle, kein aktueller Live-Befund.
- Expliziter sichtbarer Kilogrammhinweis liegt in der Fixture. Mengen-t wird nicht zu Preis-EUR/t umgedeutet. FE-Normalisierung wird im echten TempDB-Test als 0,19 EUR/kg→190 EUR/t bei `quantity_unit=kg` geprüft. Fehlender/widersprechender Hinweis fail closed.
- Deutsche Mengenzahlen: Tausendergruppen und Dezimalkomma streng geprüft; Identität ohne f64-Rundung. Eigener Harness bestätigt kanonische Gleichheit `001.000,500` und `1000,50`. Test trennt 9007199254740992/3. Numerischer Extra-Abfragewert darf runden, Text-Key nicht. Konfligierende/identische Duplikate werden getestet, allerdings nicht als kombinierter deutscher Schreibweisen-Duplikatfall durch die gesamte DB-Kette.
- Sichtbarkeit: Klassenfilter für Element und Vorfahren; visible_text filtert auch versteckte Textdescendants. Eigener Harness mit `Mill<span class="w-condition-invisible">WRONG</span>berry` liefert korrekt Millberry. Kein allgemeiner CSS/JS-Browservertrag. Zusätzliche `#info.section_product4`-Karten werden erfasst; alle vier Bestecklücken sind im Inventar. Template-/Selektoränderungen bleiben die R1-Grenze.
- Tatsächlich vorhandener vollständiger Parser→Outcome→Record→TempDB-Test rekonstruiert **46 Legacy-Keys**, darunter falsche und korrekte Wasseruhren-Zuordnung; kontrolliert alle retired IDs und komplette alte History-Zeilen. Fremdhändler, Fremdquelle, unbewiesener MSG-Key geschützt; Repeat, Umbenennung, 137,5-t-Staffel, Price-Jump, Datumsinferenz und A→B→A durch realen Record-Pfad enthalten.
- **Abgrenzung der 46er-Evidenz:** 42 zusätzliche Legacy-Keys werden aus dem neuen Parser und derselben `grade_for`-Tabelle erzeugt (`:933–956`), nicht aus einer eingecheckten unabhängigen 46-Key-Produktionsliste. Dadurch stark als Reconciliation-Test, aber nicht unabhängig gegen einen Mappingfehler im gemeinsamen Oracle. Wasseruhren-Historie wird erhalten, jedoch synthetisch zum 09.10. und Preis 3,0 angelegt, nicht als forensischer Replay der tatsächlichen Zeile vom 29.09. Produktionskongruenz der 46 konkreten Original-Keys bleibt Agenten-/Owner-Evidenz; hier keine Produktion geöffnet.
- Fehlerabdeckung vorhanden: forged Material/HTTP-Status, späterer/älterer Snapshot, echter fehlender Katalogeintrag, read-only Ancillary-Schreibfehler, Parserfehler und Count-Einbruch. Preis-/DELETE-Triggerfehler und Foreign-Keycollision im Store-Test. Nicht vorhanden: R1-Strukturverlust bei ansonsten gültiger Karte, keywordüberlappende unbekannte Sorte, explizite Growth-/Baseline-Readerror-/Kontakttrigger-Regression. Letztere sind Testlücken, kein separat bewiesener Rollbackdefekt.

## Owner-Gate

1. **R1 beheben und echte Parser→Record→DB-Regression aufnehmen, bevor Retirement regulär eingesetzt wird.** Ein grüner vollständiger Workspace-Lauf ersetzt diese Regression nicht.
2. R2-Mappingvertrag fachlich eingrenzen oder bewusst als Einschränkung entscheiden; keine pauschale Behauptung unbekannte Sorten seien fail closed.
3. Owner prüft die 46 Legacy-Originalkeys einschließlich Wasseruhren und den Ancillary-Scope; Main-Tests auf integriertem Stand folgen unabhängig.

Eng begrenzte positive Aussage: **Die heutige gespeicherte Quelle wird in 124 katalogisierte Grade-/Mengenangebote korrekt abgebildet, und die Preis-/Pointer-Transaktion ist gegen die getesteten Schreib-/Retirementfehler atomar. Das reicht wegen R1 nicht zur Freigabe eines vermeintlich vollständigen automatischen Snapshots.**

---

## Unabhängiger Re-Review nach R1/R2 — 11.10.2026

### Ergebnis des aktualisierten Audits

**R2 behoben; ursprüngliche R1-Reproduktion behoben, R1-Sicherheitsvertrag aber noch nicht vollständig erfüllt. Weiterhin keine Freigabe des automatischen Retirement wegen untenstehendem P1/R1b.** Producer-Schutz und unabhängig gespeicherter Legacy-Oracle sind substanzielle, erfolgreich geprüfte Verbesserungen. Die vorherigen Abschnitte bleiben als historischer Review unverändert erhalten; deren Aussagen zu alter Substring-Map, synthetischer Legacy-Liste und fehlendem Producer-Kollisionsschutz beschreiben nicht mehr den aktuellen Stand.

Der gemeldete Implementer-Usageerror wurde nicht als Codefehler behandelt. Entscheidend waren actual `git status`, `git diff 6d5ef762` und alle aktuellen/untracked Dateien, **nicht der alte Review-Patch**. Weiterhin drei geänderte Rust-Dateien; neu außerdem `fixtures/msg-legacy-current-20261010.json`. Keine Code-/Fixture-/Main-/Produktionsänderungen durch diesen Review, kein Produktionszugriff, Commit oder Löschen von Buildcaches. Ausschließlich diesen Bericht im Worktree ergänzt.

Aktueller Rust-Diff-SHA256: `c2d42ffe82afd766725a071be6e90bee45bbf6d8b117d2ef7c77f5df5f464361` (`git diff 6d5ef762 -- crates/`). Aktueller Handler-SHA256: `1bae3ac32be19e911ebbc439d520d6fc1fd648e03cd2f14d584e631e211347eb`; Store-Preise-SHA256: `8db05aba88149b539b4f8ee5519e8f17787301bf3e63ec4ddea37566b343ca1e`. Vollständigen aktuellen Handler inklusive Tests/Mapping, Store-Diff und Dispatch erneut gelesen.

### Eigene neue Verhaltens-Evidenz

Separater Targetdir **`/tmp/opencode/target-msg-independent-review-20261011`**, inklusive separatem `TMPDIR=.../tmp`. Nichtlöschend angelegt; keine fremden Artefakte umgebaut.

- **Eigenständig frisch kompilierter Source-Test-Harness:** aktuelle Parser-/Mapping-/Kontaktfunktionen und sämtliche zwölf aktuellen MSG-Tests unverändert aus dem Source; zwei eigene Tests ergänzt ausschließlich im stdin-Compilerinput. Echte `HandlerOutcome`-Typen und realer öffentlicher `traders::record`-Pfad aus den vorhandenen aktuellen Agenten-Linkbibliotheken. Parser-/Testcode somit unabhängig neu kompiliert, Record-Abhängigkeit nicht nochmals vollständig als Cargo-Workspace gebaut. `rereview-tests`: **13 bestanden, 1 echte Verhaltens-RED** (P1/R1b). Log `.../rereview-tests.log`, Compilerlog `.../compile.log`.
- **Store eigenständig aus aktuellem Source neu kompiliert:** kompletter Store-Crateroot mit seinen tatsächlichen Modulen, Linkabhängigkeiten aus bestehendem Cache. `store-rereview-tests snapshot_`: **5 bestanden**, inklusive Preisfehler-/DELETE-Rollback, foreign-source/late Snapshot, foreign-producer, leere Producer-Adoption und Ownership-/Datumsbasisrefresh. Log `.../store-rereview-tests.log`, Compilerlog `.../store-compile.log`.
- Alle **zwölf vorhandenen MSG-Tests** waren im eigenen Lauf grün, einschließlich der ursprünglichen `changed-pricedisplay`-Negativprobe, strikter komponierter Titel plus direkt gefälschtem Outcome, deutscher kanonischer Duplikate und echtem 46-Fixture→Record→DB-Historienschutz.
- Eigenes zusätzliches Positivtestverfahren startet für **jeden** legitimen Nicht-Angebotszustand erneut mit vorhandenen drei Millberry-Current-Pointern: sichtbare Anfrage bei hidden Staffeln, markierte CMS-Leerfelder, explizite Nullpreise, hidden ganze Karte und vollständig entfernte Karte. Alle fünf separat erfolgreich reconciled, Millberry-History jeweils vollständig identisch erhalten. Das verstärkt den vorhandenen sequenziellen Positivtest, der nach dem ersten Zustand bereits keine Millberry-Pointer mehr hat.
- Vorhandene Agentenlogs neu gelesen: `/tmp/opencode/msg-review-targeted.log`, `msg-review-store.log`, `msg-review-workspace.log`: 12/5 gezielte und **623 Workspace-Tests, 5 ignoriert**. Die 623 bleiben ausdrücklich **Agenten-Evidenz**, nicht mein eigener vollständiger Workspace-Lauf.

### P1 / R1b — uninterpretierte sichtbare Felder innerhalb erkannter Staffelzeilen bleiben unsicher

**Source:** `crates/ingestion/src/traders/handlers/msg_metallrecycling_gotha.rs:662–675`, `:702–706`, `:715–726`; Current-Retirement weiterhin `crates/store/src/public/prices.rs:564–567`, `:578–584`.

Die neue vollständige Sichtbarkeitsprüfung gilt nur für **direkte Kartenkinder**. Ein erkanntes `div.pricedisplay` wird dort pauschal akzeptiert. Innerhalb dieser Zeile werden anschließend ausschließlich `div`-Descendants gesammelt. Sichtbare direkte Text-/`span`-/andere Nicht-div-Felder der Zeile werden weder konsumiert noch abgewiesen. Ein markiertes leeres div genügt daher weiterhin als Nicht-Angebotsbeleg, obwohl daneben der echte positive Preis sichtbar steht.

**Eigene negative Mutation, nur in der echten Millberry-Karte/im RAM:**

```html
<!-- bisher -->
<div class="text-size-medium">9.90</div>
<!-- mutierter sichtbarer Wertbereich innerhalb unverändertem pricedisplay -->
<div class="text-size-medium w-dyn-bind-empty"></div>
<span class="changed-price-value">9.90</span>
```

Die Menge `ab 1 kg`, Eurozelle, übrigen beiden Millberry-Staffeln und sämtliche anderen Karten bleiben unverändert. Das ist **nicht** die legitime Mutation „Preis entfernen und CMS-empty markieren“: 9,90 steht weiterhin sichtbar in der Preiszeile, jetzt in einem nicht konsumierten Feld.

Eigener Test `independent_uninterpreted_visible_span_must_block_retirement`: Public-/InternalDB mit echtem 124er-Snapshot und erfolgreicher 124er-Baseline vorbefüllt; modifizierte Fixture→aktueller exakter Parser→realer Record→isolierte DB. Statt Parse-Fehler entsteht ein 123er-Outcome; Record gelingt. `Millberry / ab 1 kg` verschwindet aus Current, obwohl sein Preis weiterhin sichtbar ist. Die vollständigen Current-/History-Tabellen sind verändert (andere aktuelle Zeilen werden außerdem refreshed). Eigener Lauf meldet:

```text
SPAN FIELD result_error=false current_ab1_exists=false tables_unchanged=false
FAILED: uninterpreted positive span cannot be interpreted as CMS-empty price
```

**Blocker bleibt:** Der geforderte Vertrag „keine uninterpretierbaren sichtbaren Fields“ ist nicht erfüllt. Ein Count-Gate schützt 123/124 ebenso wenig wie zuvor 121/124. Der ursprüngliche spezifische R1-Fall ist korrekt repariert; die gleiche Retirement-Risikoklasse bleibt innerhalb erkannter Zeilen reproduzierbar.

**Erforderliche Abhilfe:** Auch innerhalb jeder sichtbaren Staffelzeile jeden sichtbaren Inhalt erfassen oder fail closed abweisen — insbesondere unzugeordnete direkte Text-/Elementkinder. Versteckte Alternativen bleiben ausgeschlossen; Textdescendants innerhalb eines tatsächlich konsumierten Wertfeldes dürfen weiter funktionieren. Die obige zusätzliche echte Parser→Outcome→Record→TempDB-Negativprobe muss grün werden, mit kompletten unveränderten Current-/History-Tabellen. Bestehende legitime Empty-/Zero-/Request-/Hidden-/Removed-Fälle erhalten; kein fester Count oder zusätzlicher Mengenfilter als Pflaster.

### R2, Producer und Legacy — aktualisierte Bewertung

- **R2 geschlossen:** `grade_for:350–409` ist eine exakte Liste belegter Quelltitel, normalisiert nur Case/Whitespace und validierte Mengenannotation. Kein `contains`-Behelfsmapping mehr. Eigener Source-Testlauf weist `Alu Kabel mit Kupfer`, `V2A/V4A Edelstahl` und weitere komponierte Sorten ab; manipulierte Outcomes werden im realen Record bei `:186–196` ebenfalls abgewiesen, gesamte Preis-/Current-Tabellen unverändert. Allgemeines `NEU` nicht mehr akzeptiert; Case-Änderung erlaubt und separat reconciled. Die sechs bekannten Kataloglücken bleiben ohne Ersatzmaps.
- **Producer-Grenze verbessert und selbst getestet:** Store-Kollisionsquery `:527–534` blockiert abweichenden nichtleeren `snapshot_owner` vor Preiswrites, auch bei identischer SourceURL. Candidate-Filter `:548–558` verhindert Retirement eines fremden Producers selbst bei explizit übergebener Legacy-ID. Der neue Store-Test kontrolliert vollständige Current-/History-Tabellen beim Kollisionsfehler und anschließend geschützten Fremdproducer bei legitimer anderer Publikation. Alle fünf Store-Snapshottests im eigenen frisch kompilierten Store-Lauf grün. Keine zusätzliche Producer-/Rollback-Lücke bestätigt.
- **Unabhängiger 46-Key-Oracle tatsächlich im aktualisierten Test:** JSON-File vollständig gelesen; 46 eindeutige `(material_slug,variant)`-Keys, nicht mehr aus `parse`/`grade_for` generiert. Eigener Hash `9c7b1fe9deb2a6ae3a674fd806bd88039ecd9d91bb823d81a435e1e126f802bd` stimmt mit dokumentiertem Hash überein. Reale Preis-/EURkg-EURt-Werte, Rohlabels, Produktionspreis-IDs, map_v/Datumsdefaults und Wasseruhren-Sonderzeile enthalten. Falsches `messing/Wasseruhren`: Produktions-ID 4895, `2026-09-29T18:45:21.638632402+00:00`, `{}`; korrektes `messing-leicht/Wasseruhren`: ID 7184, 10.10.-Defaultzeit, `map_v1`.
- Der aktuelle vollständige TempDB-Test (`:1217–1267`, `:1341–1361`) lädt diese Fixture ohne neue Parserableitung, replayt ihre Beobachtungswerte und prüft Retirement aller 46 IDs plus vollständige erhaltene alte Zeilen. In meinem eigenen Source-Harnesslauf grün. Lokale IDs ersetzen natürlich Produktions-IDs; Fixture-Defaults behaupten verlustfrei `ingested_at=observed_at` und enthalten gemeinsame Date-/Bounds-Felder.
- **Provenienz sauber abgegrenzt:** Fixture nennt readonly URI/`PRAGMA query_only=ON` und Capturezeit `2026-10-10T20:46:30.758868+00:00`; Implementierungsbericht nennt erneuten Vergleich aller 20 SELECT-Spalten. Diese Produktions-Capture-/Read-only-Nachprüfung ist Implementer-Evidenz. Ich habe weder Produktion geöffnet noch einen unabhängigen zweiten Original-SELECT geprüft. Unabhängig bestätigt sind die gespeicherten konkreten Daten, ihr Hash und ihr tatsächlicher Replay-/Historienvertrag, nicht ein selbst ausgeführter Produktionsabzug.
- Quellinventur erneut selbst gegen `/tmp/opencode/msg-review-live.log` ausgeführt: 133 positive sichtbare Staffeln, 124 gemappte, neun Gap-Diagnosen; gespeicherte HTML-Fixture und neues Agenten-Live-HTML byteidentisch. Die heute unveränderten 45 Grade-Zuordnungen/124 Schwellen behalten den zuvor explizit geprüften Oracle. Agenten-Live-Fetch bleibt Agenten-Evidenz.

### Freigabescope / nächster Owner-Schritt

Eng begrenzt positiv bestätigt: **R2-Mappingfix, ursprünglicher R1-`changed-pricedisplay`-Fix, legitime Nicht-Angebotszustände, 46-Legacy-Fixture-Replay samt Wasseruhren-Historie und neue Producer-Kollisions-/Retirementgrenze.** Ancillary-Writes bleiben bewusst außerhalb der Preis-/Pointer-Transaktion; bei späterem Preisfehler dürfen bereits erfolgreiche Enrichments bestehen bleiben. Kein neuer Befund gegen diesen vereinbarten Scope.

**Keine Retirement-Freigabe, bevor P1/R1b behoben und die eigene zusätzliche Negativprobe erfolgreich ist.** Anschließend Owner liest den tatsächlichen vollständigen Diff und führt Main-Tests aus. Ein 623er-Agentenlog oder Usageerror allein entscheidet weder für noch gegen Freigabe.

---

## Dritter unabhängiger Review: Owner-Fix R1b — 11.10.2026

### Ergebnis: eng begrenzte technische Retirement-Freigabe

**R1, R1b und R2 sind im jetzt geprüften Stand geschlossen. Kein weiterer bestätigter Blocker innerhalb des vereinbarten MSG-Snapshot-/Legacy-/Producer-Sicherheitsvertrags.** Die zuvor rote eigene R1b-Reproduktion und zusätzliche direkte Text-/verschachtelte Konfliktproben sind jetzt erfolgreich fail closed. Damit **technische Review-Freigabe für das eng begrenzte MSG-Current-Retirement dieses konkreten Stands**, vorbehaltlich des noch ausstehenden vollständigen Owner-Diffgates und der Main-Tests. Keine Deployment-/Produktionsfreigabe, keine Behauptung bereits erfolgreicher Main-Tests.

### Actual State und exakt abgegrenzter Owner-Delta

- `git status`/aktuelles `git diff 6d5ef762` und untracked Fixtures geprüft; der exportierte alte Patch wurde nicht als aktueller Stand verwendet. Die historischen Reviewabschnitte bleiben erhalten und werden durch dieses neue Ergebnis nicht rückwirkend umgeschrieben.
- Aktueller Handler-SHA256 **`f3f0de52f1419973c94286dc18f308491b5713f576ec580eb303263ab8c69d7b`**. Aktueller kompletter Rust-Diff-SHA256 (`git diff 6d5ef762 -- crates/`): **`d774b2d2251dc5d301d6c4790a82f0c4d0f7006d0cb2853e63366955868bc34a`**.
- Der Vergleich zum vorher tatsächlich geprüften Source ist **hashgenau**, nicht nur nach Owner-Bericht: Im RAM ausschließlich die neue `tier_el.children()`-Prüfung auf den bisherigen div-Descendant-Selector zurückgesetzt, dessen entfernte Selector-Deklaration wiederhergestellt und den neuen Owner-Test entfernt. Der resultierende Handler hat exakt den vorher dokumentierten SHA256 **`1bae3ac32be19e911ebbc439d520d6fc1fd648e03cd2f14d584e631e211347eb`**. Daraus folgt: Der Owner-Delta besteht tatsächlich nur aus Row-Knotenprüfung, Entfernung des unbenutzten div-Selectors und dem neuen R1b-Test. Keine Dateien dafür geschrieben/editiert.
- Store-Preisdatei weiterhin `8db05aba88149b539b4f8ee5519e8f17787301bf3e63ec4ddea37566b343ca1e`; Dispatch weiterhin `84312fd1b96640a825959d6058a27ad50624f2515333ad1541cf47d1d8aa4c3c`. Legacy-JSON weiterhin `9c7b1fe9deb2a6ae3a674fd806bd88039ecd9d91bb823d81a435e1e126f802bd`; HTML-Fixture weiterhin `51e9d62c01112fc29ef23e1a64c0cc400ec65b9083df2502a93c64c3e79b7212`. Somit keine unerwartete Änderung an R2, Producer-Kollisionsschutz oder unabhängigem 46-Key-Oracle.

### Prüfung des Row-Fixes und eigene neue Tests

**Source:** `crates/ingestion/src/traders/handlers/msg_metallrecycling_gotha.rs:701–726`, `:727–747`; neuer Owner-Test `:862–909`.

Der Parser prüft jetzt alle direkten Knoten der sichtbaren Staffelzeile: hidden Elemente werden ausgeschlossen; direkte sichtbare divs werden als die drei Zellen konsumiert; unbekannte sichtbare Nicht-div-Inhalte sowie nichtleere direkte Textknoten werden abgewiesen. `visible_text` konsumiert weiterhin Textdescendants eines zugeordneten Zell-divs. Es werden nicht mehr alle verschachtelten divs fälschlich als zusätzliche Zellen gezählt. Mengen-, Preis-, CMS-empty- und Request-Konfliktvalidierung bleibt erhalten, kein Count-Pflaster und kein weiterer Retirementwechsel.

Eigenständiger Targetdir **`/tmp/opencode/target-msg-independent-review-r1b-final-20261011`**, mit separatem `TMPDIR=.../tmp`. Source-Harness und Store-Testbinary frisch kompiliert; vorhandene Owner-/Agenten-Linkartefakte ausschließlich gelesen, kein Fremdcache verändert/gelöscht. Keine Code-/Fixture-/Main-/Produktionsänderungen; im Worktree ausschließlich diesen Bericht angehängt.

| Eigene Prüfung | Reale Evidenz / Ergebnis |
|---|---|
| Aktuelle 13 MSG-Tests plus drei eigene zusätzliche Source-Tests | **16 bestanden, 0 Fehler**; `.../r1b-final-tests.log`, Compilerlog `.../compile.log` |
| Store frisch aus aktuellen tatsächlichen Modulen kompiliert, fünf Snapshottests | **5 bestanden, 0 Fehler**; `.../store-tests.log`, Compilerlog `.../store-compile.log` |
| Originale R1b-Spanmutation und direkte Textmutation | Beide erzeugen **keinen Outcome**, vollständige Current-/History-Tabellen des vorbefüllten 124er-Snapshots bleiben identisch |
| Zusätzliche nicht konsumierte sichtbare span-Zelle und vierte direkte div-Zelle | Parse-Fehler, keine Current-/History-Änderung |
| Konfligierende verschachtelte Werte | Wert-div mit `div(9.90)+div(10.10)` bzw. `span(9.90)+div(10.10)` wird abgewiesen; keine Tabellenänderung |
| Gewrappte echte Werte | `span(9.90)` **und** einzelner verschachtelter `div(9.90)` innerhalb des konsumierten Wert-divs behalten 124 Angebote und 9,90; anschließend echter Record→DB erfolgreich, ursprüngliche Preis-ID behalten, keine neue Historyzeile |
| Hidden Konflikttext innerhalb einer konsumierten Zelle und direkt neben ihr | Wird korrekt ausgeschlossen; unveränderter positiver Preis durch echten Record, keine Phantomzeile |
| Fünf legitime Nicht-Angebotszustände | Jeweils **frisch mit allen drei bestehenden Millberry-Pointern** begonnen: sichtbare Anfrage bei hidden Zeilen, markiert leer, explizit zero, hidden ganze Karte, entfernte Karte. Parser→realer Record→TempDB reconciled jeweils ausschließlich die nicht mehr angebotenen Millberry-Keys; komplette Millberry-History identisch erhalten |
| R2/forged Outcome, 46-Key-Replay, Producer-/Foreignsource-/Late-/Rollback-/Adoptionverträge | Vorhandene gezielte Source-Tests im eigenen Lauf grün; unveränderte Quellen/Fixtures nach Hashvergleich |

Die eigenen negativen Proben werden mit erfolgreicher 124er-Baseline und vollständig vorbefüllter Public-/InternalDB ausgeführt. Sie erwarten ausdrücklich Parse-Fehler, nicht lediglich „Record lehnt später ab“. Die Positivproben für Wrapped-/Hidden-Text laufen zusätzlich durch den echten Record-Pfad. Das schließt genau die zuvor unabhängig bewiesene R1b-Lücke, ohne die legitimen Quellenzustände zu blockieren.

**Evidenzgrenzen:** Der MSG-Harness kompiliert aktuelle Parser-/Mapping-/Kontaktfunktionen und alle aktuellen MSG-Tests aus dem Source, verwendet aber den echten öffentlichen Record-Pfad aus der aktuellen vorhandenen Linkbibliothek; kein eigener vollständiger Cargo-Workspace-Neubuild. Der Store wurde erneut selbst aus seinen aktuellen Modulen kompiliert. `/root/Documents/msg-owner-r1b-workspace-20261011.log` selbst gelesen: **624 bestanden, 5 ignoriert**. Das sind tatsächliche **Owner-Worktree-Läufe**, nicht Reviewer-Workspace-/Main-/Produktionsläufe. Der historische Owner-RED ist als Owner-Evidenz dokumentiert; meine eigene vorherige RED und heutige erfolgreiche Nachprobe bilden die unabhängige Verhaltensevidenz.

### Weitergeltender Freigabescope

- Freigegeben ist ausschließlich der geprüfte **MSG**-Pfad: exact trader/source, published/haendler_angabe, eigene Producer-Keys oder nachgewiesene Legacy-Kombinationen inklusive historisch falsch zugeordneten Wasseruhren; keine generische Current-Neuberechnung, keine historischen Remaps/Deletes.
- Preis-/Refresh-/Markermerge-/Current-Retirement bleiben in einer SQLite-Transaktion. Fremdquellen-/nichtleere Fremdproducer-Kollision, spät beobachteter managed/legacy Snapshot und getestete SQL-Fehler verhindern Preis-/Pointeränderungen. Vorab fallible Ancillary-Writes bleiben ausdrücklich außerhalb dieses Rollbacks; bereits erfolgreiche Enrichments sind zulässig.
- Die konkrete 46-Key-Fixture, ihr Replay und Historienschutz sind unabhängig geprüft. Ihr Produktions-Capture/Read-only-Abzug bleibt dokumentierte Implementer-Evidenz; hier kein neuer Produktionszugriff. Die heutige gespeicherte 133/124/9-Quellinventur nochmals selbst gegen vorhandenes Agenten-Live-Log geprüft. Keine Garantie allgemeiner Browser-CSS/JS-Sichtbarkeit oder beliebiger künftiger Seitenstrukturen.
- Das Volumengate ist weiterhin eine zusätzliche Grobgrenze, keine universelle Vollständigkeitsgarantie. Die Freigabe beruht auf geprüfter Struktur-/Mapping-/Nicht-Angebotsvalidierung und realen Negativ-/Rollbackproben, nicht allein auf Counts.

**Nächste Gates verbleiben beim Owner:** vollständigen tatsächlichen Code-Diff lesen, Integration/Main-Tests durchführen und etwaige Veröffentlichung/Deployment separat entscheiden. Die technische Reviewer-Freigabe ersetzt keines dieser Gates.
