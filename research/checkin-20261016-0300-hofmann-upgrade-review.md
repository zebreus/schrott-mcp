# Adversarial Review: isolierter Hofmann-Zinn-Upgrade

## Gateentscheid

**HOLD für Integration in der vorliegenden Form.** Der Preis-/Retirement-Schreibvorgang ist tatsächlich transaktional und der direkte Produktions-Upgradepfad plausibel eng. Es fehlen aber Guards für ausdrücklich weiterhin angebotene, unbepreiste Zinn-Sorten und für die vollständige Legacy-Beweisprüfung innerhalb der Transaktion. Außerdem ist die zugesagte Zeitabsicherung nur einseitig: Ein später eintreffender alter ungradierter Snapshot kann den bereits retired Key wiederbeleben.

Das sind konkrete Kontrollflussbefunde, keine aus Testzahlen abgeleiteten Vermutungen. Die unten genannten zusätzlichen Szenarien wurden **nicht als Rusttests ausgeführt**; sie sind präzise Gegenbeispiele für die vom Owner nach Review geplanten gezielten Tests. Keine Builds, Testläufe, Produktionsabfragen/-writes oder Änderungen an Fixdateien durchgeführt. Die tatsächliche Produktionszeile prüft der Owner separat; dieser Bericht behauptet deren Inhalt nicht.

## Geprüfte Identität und vorhandene Evidenz

Verzeichnis `/root/Documents/checkin-20261016-0300-hofmann-upgrade-isolated`:

- Basis `81a6483208ade84c592e89a8684f56c9774dcea7`; alle 4.400 Basisdateien gegen `source/` verglichen: **ausschließlich** `crates/ingestion/src/traders/handlers/hofmann_rastatt.rs` und `crates/ingestion/src/traders/mod.rs` verschieden.
- Patch-SHA256 **399d05c23d3245e81b39e1f1eb125a2dd898b23f441aa74b0d828d11624992fd**, stimmt mit `manifest.json` überein. Patch in Speicher gegen die Git-Basis angewendet, sämtliche Kontext-/Entfernungszeilen geprüft: Ergebnis für beide Dateien byteinhaltlich gleich `source/`. Keine Patchanwendung im Hauptrepo.
- README, vollständiger Patch, relevanter Recorder-/Storecode und Testlogs gelesen. `green-hofmann.log` weist 12 bestandene Fälle aus; Workspace-Log summiert 729 bestandene Tests, sieben ignoriert, darunter bestehende Snapshot-Rollbacktests. Diese Logs sind Vorbefunde, keine hier frisch ausgeführten Tests oder Nachweise der unten fehlenden Sequenzen.
- 143 geschützte offene Dateien gegen `/tmp/opencode/checkin-20261016-013049-baseline.json` erneut SHA256-geprüft: unverändert.

Alle folgenden Codezeilen beziehen sich auf `source/`.

## Befund 1 – P1: explizite zweite Zinn-Sorte mit „auf Anfrage“ wird nicht als Ambiguität erkannt

**Ort:** `traders/mod.rs:550–563`, `handlers/hofmann_rastatt.rs:201–220`.

Der neue Guard zählt ausschließlich `outcome.prices` mit Material `zinn`. Der Parser legt bekannte Sorten ohne Zahlenpreis dagegen nur in `skipped_labels` ab und überspringt sie, z. B. **`Zinn: auf Anfrage`**. Eine gültige Tabelle mit

```text
Zinn       | auf Anfrage  [gültiger eigener Stand]
Zinn alt   | 11500 €/t    [gültiger eigener Stand]
```

liefert deshalb genau eine gezählte Zinn-Quote. `record_zinn_alt` wird aufgerufen und kann den alten `zinn/''`-Pointer retire'n, obwohl der Betreiber **beide** Sorten weiterhin ausdrücklich unterscheidet. Das ist keine eindeutige Umbenennung von Zinn zu Zinn alt. README:52 verspricht gerade, explizit gleichzeitig angebotene Sorten nicht zusammenzulegen; der Test `both_grades` deckt ausschließlich **zwei numerische Quotes** ab.

**Erforderlich:** ursprüngliche Sortenpräsenz aus dem typisierten Parse transportieren oder mindestens bekannte unbepreiste Zinn-Sorten als Retirement-Ambiguität behandeln. Preisimport von `alt` darf weiter möglich bleiben; kein daraus abgeleitetes Retirement der anderen Sorte. Test über echtes Zwei-Zeilen-HTML/`parse_dated`, nicht nur nachträgliche Manipulation von `prices`, muss den alten Pointer erhalten.

## Befund 2 – P2: enger Legacy-Prädikatnachweis liegt außerhalb der Transaktion

**Ort:** `hofmann_rastatt.rs:20–47`, Store `public/prices.rs:694–725` und `809–850`.

`current_price_for` liest den alten Datensatz vor Beginn von `record_price_snapshot`. Aus Label, Währung/Einheit, Unsicherheitsvertrag, Validität, Mappingversion und Datumsbasis wird nur die **ID** weitergegeben. Die spätere Storetransaktion prüft Current-Kandidaten zwar erneut auf Händler/Quelle, published/Quellenklasse, Producer und Beobachtungszeit; die übrigen Legacy-Merkmale werden **nicht erneut geprüft**.

Konkrete zulässige Interleaving-Sequenz:

1. Helper liest ID L als genau belegtes `Zinn` und gibt den DB-Lock frei.
2. Ein anderer Recorder schreibt denselben numerischen Key/Vertrag, gleiche Quelle, aber Label `Andere Zinnfraktion`, mit Zeitpunkt zwischen Altbeobachtung und Upgrade.
3. Generische Deduplizierung findet L: die SQL-Dedupe ignoriert für Nicht-MKM unter anderem das Label; `UPDATE prices` ändert Label/Metadaten/Beobachtungszeit **unter derselben ID L**.
4. Upgrade-Transaktion findet L weiterhin Current, gleiche Quelle, kein fremder Producer und nicht neuer als Upgrade. Weil `legacy_ids.contains(L)`, wird die mittlerweile nicht mehr passend belegte Beobachtung retired.

Die Preis-/Deleteoperation ist zwar atomar; die **Beweisbindung** ist es nicht. Mehrere Threads/DB-Verbindungen können zwischen den beiden PublicDb-Aufrufen schreiben. Dass der reguläre Scheduler sequenziell läuft, ist keine vom Store erzwungene Exklusivität. Die vorhandenen Fremdproducer-/Neuere-ID-Tests widerlegen dieses Same-ID-Szenario nicht.

**Erforderlich:** Legacy-Auswahl mit vollständigem Prädikat oder unveränderlichem Zeilenfingerprint in dieselbe Storetransaktion ziehen; alternativ eine tatsächlich durchgesetzte Exklusivitätsgarantie begründen. Deterministischer Interleavingtest muss bei geändertem Same-ID-Beleg fail-closed bleiben. Nicht pauschal den gesamten Store umbauen; der zusätzliche Vertrag darf auf diesen Upgradefall begrenzt sein.

## Befund 3 – P2: Späterlauf-/Late-Replay-Schutz endet am neuen `alt`-Zweig

**Ort:** `traders/mod.rs:550–565`, Store `public/prices.rs:888–917`.

Nach erfolgreichem Upgrade `zinn/'' @ T1 → zinn/alt @ T3` ist der alte Current-Key leer. Ein danach ausgeführtes typisiertes Outcome mit altem Label `Zinn`, beobachtet bei **T2 < T3**, fällt nicht in den neuen Snapshotzweig, sondern in `record_price`. Dieser prüft Current nur für **denselben Key**. Für `zinn/''` existiert nun kein Pointer: der Store setzt ihn wieder Current (`None => true`). Ergebnis: wieder zwei Current-Preise; der neue Producer-/Zeitguard wird gar nicht erreicht.

Auch eine spätere echte Neubeobachtung von `Zinn` ist von einem Legacy-Kandidaten nicht dauerhaft unterschieden: Das gleiche `map_v:2`-/Label-/Quellenprofil kann nach dem Upgrade neu entstehen und beim nächsten alleinigen `alt`-Lauf erneut als „historischer“ Key retired werden. Der Patch implementiert damit einen wiederkehrenden Ersatzvertrag, nicht technisch eine einmalig abgegrenzte Altbestandsmigration.

**Erforderlich vor Abnahme:** gewünschte Späterlaufpolitik explizit entscheiden und testen: (a) Late-Replay eines alten `Zinn` nach erfolgreichem Upgrade, (b) spätere echte gleichzeitige Sortenangebote, (c) nachfolgend nur `alt`. Für die zugesagte Zeitordnung muss der retired Key eine wirksame Übergangs-/Zeitgrenze behalten oder der Rückweg entsprechend geschützt sein; keine pauschale Blockierung legitimer neuer Sorten. Soll ausschließlich der einmalige Vorwärtslauf garantiert werden, müssen Anspruch und akzeptiertes Restrisiko ausdrücklich enger abgenommen werden. Der bisherige zwölfteilige Guardtest prüft „Alt-Pointer ist schon neuer als Upgrade“, **nicht** diese umgekehrte Reihenfolge.

## Was am Patch trägt

- **Gezielter Einstieg:** exakter Händler-Slug, typed Row-Dates, exakte Quell-URL, `zinn/alt`, Quelllabel `Zinn alt`. Der typisierte Snapshot besitzt private Preise/Datumsfelder; Row-Dates werden vor Preiswrites vollständig validiert. Keine beliebige Händler- oder URLübernahme.
- **Legacy-Vorauswahl:** ausschließlich derselbe Händler/Material und leere Variante; genaue Herkunft, published, EUR/kg, exact/1.0, fehlende Grenzen, map_v 2, page_stated/row_stated und kein fremder Producer. `page_stated` für den alten untypisierten Recorder ist richtig mitgedacht. Die tatsächliche Übereinstimmung der Produktion bleibt bis zum Ownerbefund offen.
- **Atomare Schreibgrenze:** Store `record_snapshot_transaction` beginnt eine Transaktion, prüft neue Keykonflikte und Retirementkandidaten, schreibt Ersatz und löscht anschließend nur Current-Pointer; Commit am Ende. Bei Fehler Rückabwicklung beider Teile. Grenze ist ein Zinn-Key, **nicht** die gesamte 23-Zeilen-Tabelle oder Acceptancewrites. Das ist im README korrekt abgegrenzt.
- **Producer:** eigener Marker `hofmann-zinn-alt-v1`; fremde Current-Producer/Quellen werden im Snapshotzweig abgewiesen. Der neue Owner umfasst im aktuellen Code nur den `alt`-Key; kein pauschaler Hofmann-Tabellensweep.
- **History:** Retirement selbst löscht keine `prices`-Zeile. Volltabellentest vergleicht sämtliche Spalten des arrangierten alten page_stated-Datensatzes; Nachbarzeilen und individuelle Veröffentlichungsdaten werden kontrolliert. Wiederholung des gleichen `alt`-Outcomes dedupliziert erwartungsgemäß. Normales Refreshen neu beobachteter Quotes kann weiter Beobachtungs-/Metadaten ändern; der Patch verspricht sinnvollerweise keine generelle Unveränderlichkeit aller Preiszeilen.
- **Fehlertransparenz:** fehlgeschlagener Einzelwrite wird nicht als `recorded` gezählt, sondern in `skipped` gemeldet; Nachbarwrites können weiterhin erfolgen. Kein vorgezogener unbedingter Delete vor erfolgreicher Preisveröffentlichung gefunden.

## Abnahmeempfehlung

Owner sollte zuerst den realen Legacy-Preis vollständig gegen den engen Prädikatvertrag prüfen, danach gezielte **neue Gegenbeispiele** zu Befund 1–3 abnehmen. Für einen breiten Snapshot-/Zeitguard-Abschluss sind diese Fälle derzeit offen; 729 grüne Tests ersetzen sie nicht. Befund 1 und die atomare Beweisbindung sollten vor Integration behoben werden. Für Befund 3 ist mindestens eine explizite, getestete Scope-/Restrisikoentscheidung nötig.

**Kein Freigabevotum für unveränderte Integration.** Keine Hauptrepo-Fixdatei, isolierte Quelle, Patch oder Log wurde durch diesen Review verändert; Integrationsumfang dieses Auftrags ausschließlich dieser Bericht.
