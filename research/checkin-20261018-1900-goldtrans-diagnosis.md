# 19:00 Goldtrans: bestätigte produktive Nullpreis-Veröffentlichung

Stand: 18.10.2026, eigene Prüfungen 19:01–19:03 UTC. **Offener, dringender Datenqualitätsvorfall; nicht geheilt.** Dienstgesundheit/2.683 Current insgesamt widerlegen diesen Fehler nicht.

## Ergebnis und Sofortempfehlung

**Nicht nur gewarnt:** Alle 14 beanstandeten Werte stehen als `0 EUR/g` sowohl in `prices` (History) als auch hinter den 14 Goldtrans-Current-Pointern. Sie sind `published=1`, `price_kind=exact`, `confidence=1.0`, `source_type=haendler_angabe`. Die aktuelle Primärseite liefert ebenfalls 14 explizite `0,00 €/g`; der vorhandene echte Handler akzeptiert sie DB-frei unverändert. Die konkrete Ursache des Nullzustands beim Betreiber (z. B. dessen Kursfeed) ist **nicht** belegt.

Empfohlene, **nicht ausgeführte**, autorisierungspflichtige Sofortmaßnahme:

1. Diese **14 Current-Schlüssel des Händlers 10108** aus der aktuellen Preisauslieferung quarantänisieren/unterdrücken, mit transparenter Nichtverfügbarkeitsanzeige. Historische Fehlerbelege behalten. Den vollständigen Händler oder sämtliche Goldpreise anderer Händler nicht pauschal sperren.
2. Vor erneutem regulärem Goldtrans-Lauf eine **händlerspezifische Fail-closed-Sperre für diese Nullpreissituation vor dem Schreiben** bzw. temporäre Quarantäne dieses einen Preissteps vorsehen. Eine reine Schedulerpause entfernt bereits veröffentlichte Nullpreise nicht; reine Current-Bereinigung ohne Schreibsperre schützt nicht vor Wiederkehr. Konkreter verfügbarer administrativer Quarantänemechanismus wurde hier nicht verifiziert; keine ungetesteten SQL-/Deploymentbefehle empfohlen.
3. Kein Force-Ingest und kein blindes Zurücksetzen auf 12:30-Werte als neue aktuelle Preise. Die letzten positiven Werte sind historischer Referenzbestand, keine frisch bestätigten Betreiberangebote. Wenn sie fachlich angezeigt werden sollen, nur ausdrücklich als zuletzt bekannte/veraltete Preise nach freigegebener Wiederherstellungsentscheidung.
4. Wiederfreigabe erst nach positivem Primärquellen-Capture, Replay durch denselben Parser und geprüftem Nichtschreib-/Quarantäneverhalten bei Nullfixture. Danach Current/History und öffentliche Auslieferung unabhängig prüfen. Das ist Folgearbeit, hier nicht erledigt.

**Canary-Blindstelle:** Der Vergleich prüft `previous.price > 0.0`. Nach dem ersten Nullwrite ist ein weiterer Nullabruf daher kein erneuter Preissprung gegen einen positiven Vorgänger. Auf das Ausbleiben der nächsten Warnung darf keine Entwarnung gestützt werden. Handlerintervall: sechs Stunden; tatsächliche nächste Fälligkeit hier nicht aus Schedulerzustand ausgelesen.

## Konkrete Produktionskette (nur gelesen)

Beide DBs ausschließlich mit `sqlite3.connect('file:/var/lib/schrott-mcp/{public,internal}.db?mode=ro', uri=True)` und unmittelbar `PRAGMA query_only=ON`; nur SELECT/Schema-Lesen. Public-`query_only` zusätzlich mit Ergebnis `1` erfasst. Keine Backups, Migrationen oder Writer geöffnet.

| Beleg | Befund |
|---|---|
| Händler | `hh-wandsbek-goldtrans-edelmetallhandel`, ID **10108** |
| Ingestion | Run **2076**, Step **9759**, `warning`, **14 items_upserted** |
| Stepzeit | Start `18:32:54.367629731+00:00`, Ende `18:32:56.447860858+00:00` |
| Fetch | ID **9755**, HTTP **200**, **93340 Bytes**, `18:32:56.399800153+00:00` |
| Source/fetch URL | `https://www.goldtrans.de/goldankauf-preise-aktueller-goldpreis-ankauf-in-hamburg.html` |
| Produktionsfetch-Metadatenhash | `85a12688a2a15643` — **kein HTML-Bodyhash**, siehe unten |
| Current und History | IDs **22021–22034**, alle Null, alle Current; observed_at und Current-updated_at gleich Fetchzeit |
| Publikationsdatum | `2026-10-18T00:00:00+00:00`, `published_at_basis=page_stated`, `map_v=2` |
| Material | Gold ID **1378**, Zahngold ID **1379**, jeweils `EUR/g` |
| Ownership | Standard-`RecordOutcome::Observation`; **kein snapshot_owner** in Preis-extra_json; für diesen Händler **0** `source_article_snapshots`, **0** `article_acceptance_owners` |

Es existiert kein separater Source-ID-/Article-Owner-Nachweis in dieser beobachteten Preisroute. Technischer Step-/Handlerowner ist der genannte Slug, Quellprovenienz ist die konkrete URL. Keine menschliche Bereitschafts-/Organisationszuständigkeit aus den Daten erfunden.

### Betroffene Schlüssel und letzter positiver Vergleich

Die positiven Vorgänger stammen aus Run **2052**, Step **9623** (`ok`, 14 übernommen), Fetch **9619** (HTTP200, 93359 Bytes), observed_at `2026-10-18T12:30:09.018060442+00:00`. Sie bleiben History, nicht Current.

| Material/Variante | alte History-ID | vorher EUR/g | neue Current-/History-ID | jetzt EUR/g |
|---|---:|---:|---:|---:|
| gold/999 | 19489 | 110.70 | 22021 | 0 |
| gold/986 | 19490 | 109.15 | 22022 | 0 |
| gold/980 | 19491 | 108.49 | 22023 | 0 |
| gold/965 | 19492 | 106.83 | 22024 | 0 |
| gold/916 | 19493 | 101.40 | 22025 | 0 |
| gold/900 | 19494 | 99.63 | 22026 | 0 |
| gold/875 | 19495 | 96.86 | 22027 | 0 |
| gold/833 | 19496 | 92.21 | 22028 | 0 |
| gold/750 | 19497 | 83.03 | 22029 | 0 |
| gold/585 | 19498 | 64.76 | 22030 | 0 |
| gold/416 | 19499 | 46.05 | 22031 | 0 |
| gold/375 | 19500 | 41.51 | 22032 | 0 |
| gold/333 | 19501 | 36.86 | 22033 | 0 |
| zahngold/leer | 19502 | 68.63 | 22034 | 0 |

Abgrenzung: Insgesamt **15** Null-Historyzeilen dieses Händlers, nicht 15 neue Fehlerzeilen. Die zusätzliche ID **7042**, gold/916, stammt schon vom **30.09.2026 17:06:58 UTC**, `map_v=1`, nicht mehr Current. Der aktuelle 14er-Vorfall ist neu; Nullpersistenz ist nicht erstmalig in der gesamten Händlerhistorie.

## Feedbackloop und Primärbeleg

Vorhandenes Binary, **ohne Cargo/Build und ohne DB**:

```sh
python3 - <<'PY'
import subprocess,re
r=subprocess.run(['target/debug/examples/live_handlers',
 'hh-wandsbek-goldtrans-edelmetallhandel'],capture_output=True,text=True,timeout=65)
rows=re.findall(r'^\s+(gold|zahngold)\s+([0-9.]+)\s+EUR/g',r.stdout,re.M)
print('existing_handler_exit',r.returncode,'rows',len(rows),
      'zero_rows',sum(float(x[1])==0 for x in rows))
assert r.returncode==0 and len(rows)==14,'wrong seam or handler failed'
assert all(float(x[1])>0 for x in rows), 'RED: published Goldtrans prices contain zero'
PY
```

Bereits tatsächlich ausgeführt; Ergebnis: `existing_handler_exit 0 rows 14 zero_rows 14`, anschließend **AssertionError / Exit1**. Vorheriger unabhängiger Aufruf desselben Binaries zeigte ebenfalls sämtliche 14 Nullwerte, `conf=Some(1.0)`, HTTP200/93340 Bytes, Datum 18.10. Kein SKIP. Das ist ein echter Parser-/Handler-Repro, keine Python-Nachimplementierung der Zahlenlogik. Die Beispielquelle bestätigt, dass lediglich `scrape_record_outcome` aufgerufen und gedruckt wird, kein `record_outcome`/DBzugriff.

**Capture:** Betreiber-URL am **19:01:48.544478 UTC** ohne Anmeldung/TLS-Ausnahme direkt abgerufen: HTTP200, keine Weiterleitung, `text/html; charset=UTF-8`, 93340 Bytes. SHA256 des tatsächlich gespeicherten HTML: **`4f521430dba3778f990b70c8ef4ff31e5df2bd3fa00780c2a58dc5c5835a2b03`**. Im Preisblock mit Überschrift `Gold Legierung` stehen 13 Goldlegierungen und Zahngold jeweils mit `0,00 &euro;/g`; `Stand: 18.10.2026`. Beispiel:

```html
<td>Gold 999 24 Karat</td>
<td>0,00 &euro;/g</td>
```

**Evidenzgrenze, wichtig:** Der gespeicherte Capture und die zwei echten Parserläufe sind getrennte HTTP-Abrufe. Ein **offline gepinnter Capture→bestehender-Parser-Replay** ist hier **nicht durchgeführt**: vorhandenes `live_handlers` nimmt nur einen Slug, keine Inputdatei an; `goldtrans::parse` ist privat; ein Replay-Adapter würde Build/Codearbeit benötigen, was ausdrücklich untersagt ist. Debugger `gdb`/`lldb` sind ebenfalls nicht vorhanden. Deshalb keine deterministische Offline-Replay-/vollständige Skill-Phase-2-Erfüllung behauptet. Der Live-Loop war zweimal gleich rot, bleibt aber netzabhängig. Keine weitere Spekulation über interne Betreiberursachen aus dieser Lücke abgeleitet. Für exakte historische Byteidentität fehlt zusätzlich der historische Body: `raw_fetches.content_hash` hasht laut `scheduler.rs:219–228` nur `(slug,url,status,bytes)`, **nicht den Inhalt**. Gleiche Länge ist kein Gleichheitsbeweis.

## Hypothesencheck und engste nachgewiesene Ursache

Nach dem roten Handlerloop kommunizierte Rangfolge:

1. **Nullwerte schon in der Quelle.** Vorhersage: direkter Primär-Capture enthält in den ausgewählten Preiszellen Nullen. **Bestätigt für den eigenen frischen Abruf.** Damit keine Behauptung, der historische Produktionsbody sei kryptografisch identifiziert.
2. **Falsche Tabelle/Zahlenwahl.** Vorhersage: die beabsichtigte Goldtabelle zeigt positive Werte, der Handler nimmt andere Zahlen. **Für frischen Abruf widerlegt:** Goldtabelle und bestehender Handler zeigen beide dieselben 14 Nullen. Goldtrans-`parse` selektiert explizit den `Gold Legierung`-Header, `parse_eur` akzeptiert `0,00`, `unit_of` erkennt `/g`.
3. **Erst beim Speichern entstandene Umrechnung/Einheitenfehler.** Vorhersage: DB-freier Handler liefert positive Werte. **Widerlegt:** schon der echte Handler liefert Null, und DB-Einheit bleibt EUR/g. Kein kg/g-Faktor nötig, um das Symptom zu erklären.

Nachgewiesene Fehlerkette: Betreiber liefert aktuell syntaktisch gültige Nullwerte → Parser akzeptiert sie ohne positive Preisplausibilität → Goldtrans-Scrape erzeugt `exact`/confidence1.0 → Standardrecord schreibt → Canary vergleicht **erst nach erfolgreichem Write** und meldet nur Warnungen. Belege: `goldtrans.rs:34–75,160–178`; `traders/mod.rs:550–592`; `scheduler.rs:184–199`. Der Code dokumentiert ausdrücklich „warnings, not failures: the data is recorded“. **Kein wirkender Schutzguard**, keine bloße Dry-run-Warnung.

Ein minimierter Offline-Regressionstest und ein Fix wurden dem Arbeitsauftrag entsprechend nicht erstellt. Für künftige Reparatur ist eine Capture-Nullfixture am echten Handler→Record-Seam erforderlich: muss zeigen, dass weder neue History noch Current mit diesen 14 exakten Nullangeboten entstehen; ein bloßer Test von `parse_eur("0,00")` wäre der falsche Seam. Keine globale Nullpreisregel für alle Händler ohne Prüfung von Gratisannahme-/Entsorgungssemantik ableiten.

## Private Belege / Abschlussstatus

Kleiner privater Captureordner `/tmp/opencode/goldtrans-1900/` (0700), HTML/DB-/Parserbelege 0600, keine Authheader oder Secrets:

- `db.json`: 50 gezielte Händler-Historyzeilen, Current-Markierung, Gesamtzahl, query_only-Nachweis.
- `internal.json`: vier letzte Steps und Fetchmetadaten dieses Slugs.
- `final-db.json`: Current-Schlüssel und Ownership-Abgrenzung, Stand 19:03 UTC.
- `source.html`, `source-meta.json`: Primär-Capture und sichere HTTPmetadaten.
- `parser-live.txt`: echter zweiter Handlerlauf vor roter Assertion.

**Nur Bericht und kleine private Belege geschrieben.** Keine Codeänderung, Prod-DML, Force-Ingest, Deployment, Build, DBkopie oder Dienständerung. Andere MKM-/Cachedateien nicht bearbeitet. Keine Reparatur oder tatsächliche Unterdrückung der öffentlichen Preise behauptet; öffentliche API-Auslieferung wurde nicht separat aufgerufen. Letzte direkte DBprüfung: weiterhin 14 Null-Current. Höchste Priorität für das neue Datenqualitätsproblem; Eingriff und offline Capture-Replay bleiben offen.

## Nachtrag: begrenzt autorisierter Regressionstest-Quellcode (noch ungebaut)

Owner-Folgeauftrag: ausschließlich Goldtrans-Datei, kleine datierte Fixture und dieser Nachtrag; **noch kein Fix und keine Build-/Test-/Cargoausführung**. Zu Beginn `git diff -- crates/ingestion/src/traders/handlers/goldtrans.rs` geprüft: keine vorbestehende halbfertige Änderung an dieser Datei. Produktionsdatenbanken in diesem Folgeauftrag nicht geöffnet.

### Bereitgestellte Dateien und exakte Tests

`crates/ingestion/src/traders/handlers/goldtrans.rs`, Modul `traders::handlers::goldtrans::tests`:

- `captured_zero_snapshot_rejected_before_recording`: gepinnter echter 14er-Null-Capture → derselbe Preisparser und dieselbe `HandlerOutcome`-Erzeugung wie im Livehandler → echtes `traders::record` ausschließlich auf privaten Testdatenbanken. Erwartet unveränderte DB und einen Goldtrans-`IngestError::Parse`, nicht bloß einen Canary, leere Erfolge oder Skips.
- `partial_zero_snapshot_rejected_atomically_before_recording`: zwei Mutationen desselben Captures nach positiver Vorbelegung: einzelner Nullwert spät bei Gold/333 bzw. Zahngold; zugleich erster gültiger Goldwert verändert. Verlangt Zurückweisung des **ganzen** Snapshots, ohne vorherige positive Teilwrites. Die beiden Fälle erhalten jeweils eigene Test-DBs.
- `positive_snapshots_preserve_prices_variants_and_recording`: bestehende positive Dreizeilen-`FIXTURE` weiterhin echt recorden; danach 14er-Positivkontrolle mit festen, unabhängig belegten 12:30-Vorgängerwerten. Prüft 14 Material-/Varianten-/Preis-Tupel, EUR/g, exact, confidence1.0, Publikationsdatum, keine Skips sowie erfolgreiche Current-Besetzung. Die 14er-Positivkontrolle ist ausdrücklich **synthetisch mutiert**, kein erfundener positiver Livecapture.

Für die Tests notwendige kleine **verhaltenserhaltende Seam-Extraktion**, kein Guard/Fix: `prices_from_html(status, html)` erzeugt den bislang inline in `scrape` aufgebauten Preis-Outcome. `scrape` ruft diesen auf und ergänzt danach wie bisher das separat gefetchte Impressum. Reihenfolge Preisparse → Impressumsfetch/-parse sowie Ergebnisfelder bleiben erhalten. Damit testet der Replay keine nachgebaute Mapping-/Filterlogik. `parse_eur` und globale Recorder-/Store-/APIregeln wurden nicht geändert; Nullwerte werden im bereitgestellten Produktionspfad weiterhin nicht neu gesperrt.

Bestehende Setupbausteine: `crate::test_support::TempDbDir`, `PublicDb::open`, `InternalDb::open`, `crate::seed_metadata`. Reale Recording-Schreibzugriffe sind im **Quellcode der späteren Tests** ausschließlich auf diesen frischen Testverzeichnissen vorgesehen; diese Tests wurden jetzt nicht gestartet. `TMPDIR=/tmp/opencode` beim später freizugebenden Lauf setzen. Vorher/Nachher-Snapshot prüft alle Current-/History-/Annahmezeilen, den betroffenen Händler und SQLite-Sequenzen. Ein erfolgreicher leerer/geskippter Outcome wird trotz unveränderter DB durch die separate Parsefehler-Assertion ausdrücklich nicht grün.

### Capturepin und Sanitizing

Fixture: `crates/ingestion/src/traders/handlers/fixtures/goldtrans-zero-20261018.html`, **1825 Bytes**, SHA256 **`b029d1ec12c2b0dcf1496df6dc28fab985f83e62b2bc66df797f02f011372682`**; dieser Hash wird auch im Capturetest fest geprüft. Original `/tmp/opencode/goldtrans-1900/source.html` unangetastet, Originalhash weiterhin **`4f521430dba3778f990b70c8ef4ff31e5df2bd3fa00780c2a58dc5c5835a2b03`**; Capturezeit und Originalhash stehen im Fixturekommentar.

Retain: Introanker, vollständige zweispaltige Tabelle mit 14 Preiszeilen und Stand-Datum. Entfernt: Seitengerüst, CSS und übrige Inhalte; Zeilenenden und unwesentliche Zeilenend-Leerzeichen normalisiert. Keine Zahl-, Material-, Legierungs- oder Datumsänderung im Capture. Statische Prüfung per Python-HTMLParser (kein Rust-/Regressionstestlauf): **30 normalisierte Header-/Datenzellen identisch** zur ursprünglichen Goldtabelle, **14 Nullpreiszellen**, kein Script-/Iframe-/Form-/Input-/Eventhandler-/Auth-/Cookie-Markup im Auszug. Nur öffentliches Preistabellenfragment, keine Kontakt-/Authdaten übernommen.

### Status und noch gesperrter nächster Schritt

**Quellcode bereit, nicht kompiliert und nicht ausgeführt. Kein gemessenes Rot/Grün, keine Behauptung einer Kompilierbarkeit oder eines bestandenen Tests.** Aufgrund unveränderter fehlender Sperre ist das erwartete Verhalten der beiden negativen Tests ein Fehler an der DB-Unverändertheitsassertion; das ist eine Vorhersage, keine Rotevidenz. Die vorherigen Live-Repro-Ausgaben sind keine Ausführung dieser neuen Tests.

Nach separater Budgetfreigabe: zuerst die neuen Tests im vorhandenen Target und ohne Produktionszugriff laufen lassen und exakte Rotevidenz sichern; Positivkontrolle prüfen. **Erst danach und nur mit Fixfreigabe** den vollständigen Goldtrans-Snapshot vor jeglichem Recording bei nicht endlichem oder nicht positivem Gold-/Zahngoldpreis mit Parsefehler abweisen; keine Zeilen still entfernen und keine globalen Nullpreisregeln. Nonfinite-/Negativfälle bei dieser Folgestufe gezielt ergänzen; die drei jetzt geschriebenen Tests behaupten dafür noch keine vollständige Abdeckung. Anschließend derselbe Replay und Positivkontrolle erneut. Keine Storequery-/APIänderung; bestehende produktive Null-Current bleiben separate Owner-Quarantänearbeit.

## Nachtrag: gezielt freigegebener Goldtrans-Fix nach echtem Owner-Rot

Owner meldet den echten Offline-Capture→Outcome→Recording-Rotlauf: `/root/Documents/checkin-20261018-2230-goldtrans-write-red.log`, Exit101, ein ausgeführter Test, DB-Unverändertheitsassertion schlägt wegen 14 neuer Nullwrites fehl. Logdatei hier ausschließlich gelesen; SHA256 frisch bestätigt: **`febc6de672c412a7a8ee76eac437083c14beb546c490942a8c97b8f7b3c4c697`**. Logzusammenfassung bestätigt 1 failed / 0 passed, keine neue Testausführung durch diesen Agenten. Dies ersetzt die zuvor offene Rotevidenz für den Null-Capture, nicht für neu hinzugefügte Fälle.

**Fixquellcode bereit, Grünprüfung offen.** Ausschließlich in `goldtrans.rs`: `positive_gold_quote` validiert bei als Gold/Zahngold erkannten Tabellenzeilen die vollständige Preiszelle, bevor daraus ein `HandlerOutcome` entstehen kann. Erforderlich: nichtleerer vorzeichenloser deutscher Zahlenbetrag (ASCII-Ziffern, Punkt/Komma), explizites `€/g` oder `EUR/g`, anschließend `parse_eur`-Ergebnis **endlich und >0**. Sonst Goldtrans-`IngestError::Parse`, gesamter Snapshot verworfen — einschließlich früherer gültiger Zeilen. Auch fehlende Preiszelle wird für diese Materialien Fehler, nicht Skip. Livehandler bricht somit vor Impressumsenrichment und vor jeglichem Recorderaufruf ab. CSS/Tabellenwahl, andere Handler, Shared-`parse_eur`, Recorder, Store und API unverändert.

Warum vollständige Betragssyntax nötig ist: Der bestehende Sharedparser überliest ein führendes Minus und könnte `-1,00` als +1 akzeptieren; `NaN`/`inf` ohne Ziffern liefern `None` und wurden bisher übersprungen; `1e309` könnte als 1 abgeschnitten werden. Die Goldtrans-lokale Syntaxprüfung verhindert diese Umdeutungen. Ein Betrag aus 400 Neunen prüft zusätzlich echten numerischen Overflow am Finite-Guard; nichtnumerische Preise/Anfragetext dürfen ebenfalls kein Teilgrün erzeugen. Keine globale Regel gegen Gratisannahmen eingeführt.

Neue Tests im selben bestehenden Modul `traders::handlers::goldtrans::tests`:

- **`negative_nonfinite_and_unparseable_prices_reject_before_recording`**: je eine späte Gold/333- und Zahngoldmutation mit `-1,00`, Unicode-Minus, `-0,00`, `NaN`, `inf`, `-inf`, `Infinity`, `1e309`, 400-stelligem Overflowbetrag, Anfragetext oder leerem Betrag; gleichzeitig erster gültiger Preis verändert. Echter Capture→Outcome→Record-Seam, Parsefehler und unveränderte Current/History/Annahmen/Händler/Sequenzen erforderlich. Eine gemeinsame isolierte positive Baseline für die 22 Varianten spart wiederholtes Catalogsetup.
- **`missing_gold_price_cell_rejects_before_recording`**: komplette späte Zahngold-Preiszelle aus dem Capture entfernt, früher gültiger Preis verändert; gleicher atomarer Nichtschreibvertrag. Keine bloße Tabellenentfernung oder Parserstub.

Bestehende Capture-/Teilnull-/Positivtests unverändert im fachlichen Vertrag erhalten. Fixture unverändert und weiterhin hashgepinnt. Nur Testhelper zur Wiederverwendung der Nichtschreibassertion extrahiert; kein testseitiger Preisguard. **Keine Builds, Tests, Cargoaufrufe, Prod-DBzugriffe, Commits oder Deployments durchgeführt.** Nur statisches `git diff --check` geprüft; kein Grün-/Kompilierbarkeitsnachweis behauptet. Owner führt die Grünprüfung mit eigenem Budgetwächter aus. Separate Current-Retire-/Quarantänemigration ausdrücklich nicht bearbeitet; kein Produktionsheilungsnachweis aus diesem lokalen Patch.
