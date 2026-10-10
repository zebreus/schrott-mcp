# Kernschrott: unabhängiger Read-only-Review, 10.10.2026

## Ergebnis

**Follow-up: Freigabe zur Main-Integration des erneut geprüften Stands.** S1/F1 sind behoben; F4 und der Quellenprüfungsanteil von F3 ebenfalls. S2/F2/F3 sind mit den ausdrücklich dokumentierten Owner-Entscheidungen für diesen handlerlokalen Umfang akzeptiert, nicht durch eine Store-/View-Migration gelöst. Kein verbleibender Code-Review-Blocker gefunden. **Keine Produktions-/Auftragsabschlussfreigabe:** koordinierter Rollout und regulärer Scheduler-/Speichernachweis bleiben offen.

## Follow-up der Überarbeitung — 10.10.2026

Dieser Abschnitt ersetzt die ursprüngliche Freigabeentscheidung und deren offenen Integrationsgates. Der vollständige Erstbericht darunter bleibt als historische Befundgrundlage erhalten; seine alten Handlerzeilennummern und Annahmezahlen beziehen sich ausdrücklich auf den ersten Stand.

### Tatsächlich erneut geprüfter Stand und Belege

- Tracked Diff gegen Worktree-HEAD erneut gelesen: unverändert additive 13 Katalogeinträge und Modul-/Registry-Einbindung; Dossier um Review-Korrektur ergänzt, alte Timeline erhalten. Gesamtes weiterhin untracked Handlerfile samt allen Tests gelesen, nicht nur `git diff --stat` (dieses erfasst den neuen Handler nicht). Git-Status zeigt keinen Shared-Record-/Store-/View-/Dependency-Diff.
- Aktueller Handler-SHA-256 unabhängig nachgerechnet: `28ed6640fb3026b45167c38bfed77e88d1e6de2c7618f72a4798bf7c7929c47e`, identisch mit dem final getesteten Stand im überarbeiteten Owner-Bericht:94. Fixture unverändert: `b1a313c9cfc2d3dea5845984f9b98190eac7ba07daedddc61252e466a36db532`. Ein Dateihash plus Owner-Ausführungsbericht ist keine kryptografische Bindung der Logs an ein Buildartefakt; keine solche Garantie behauptet.
- Preisseite unabhängig nochmals live read-only abgerufen: HTTP 200, `/elektro`, 40824 Bytes, SHA-256 `c958c911c85c9b634ed5d32ed98316c0d65b793a4e5e4b8681a9849fb2d5f753`. Alle 42 vollständigen Name/Beschreibung/Preis-Datensätze weiterhin identisch zur Fixture und damit zur untenstehenden Volltabelle.
- Globale Reservation im tatsächlichen `.sec-head p` und Ausschluss im tatsächlichen `.note > div` nochmals gegen Handlerkonstanten geprüft: jeweils genau ein Text, beide exakt gleich nach Whitespace-Normalisierung. Der aktuelle Livezustand trägt die Prüfannahmen.
- Neue Logs vollständig beziehungsweise die lange Debug-Ergebniszeile programmatisch gelesen: `kernschrott-targeted-review-fixes-20261010.log` nennt die 9 erfolgreichen lokalen Tests einzeln, einschließlich Drift, vollständigem Record, Dedupe und blockiertem Folgesnapshot; 1 Livetest dort bewusst ignoriert. `kernschrott-live-review-fixes-20261010.log` zeigt separat echten Handlerlauf, 38 Preise / 19 Aggregate / 1 Class-3-Skip. `kernschrott-workspace-review-fixes-20261010.log` meldet 515 bestandene Ingestiontests, insgesamt 572; das ersetzt nicht die fachlichen Einzelprüfungen.
- **Keine eigenen Builds, Rust-Tests, DB-Zugriffe, Commits oder Worktree-Änderungen.** Eigene zusätzliche Ausführung ausschließlich lesendes Git, Dateilesen/Hashing/HTML- und Logauswertung sowie HTTP-GET. Nur dieser Main-Bericht aktualisiert.

### S1 — geschlossen für den erfolgreichen Record-Pfad

`kernschrott.rs:235–236,297–310,345–353` sammelt pro Material alle zugehörigen Varianten **mit vollständigen tatsächlichen Beschreibungen**. Ein BTreeMap-Eintrag je Material verhindert last-variant-wins. Die 19 expliziten Annahmen enthalten Variante-für-Variante-Bedingungen und beide validierten globalen Texte; Platinen erhalten zusätzlich eine ausdrücklich als Quarantäne erkennbare Class-3-Abgrenzung, keine angebliche Quell-Negativannahme.

Gemeinsamen Record-Pfad nochmals gelesen: `traders/mod.rs:416–450` schreibt weiterhin zunächst implizite Blank-/RAM→Platinen-Fallback-Annahmen. **Erst danach** schreibt `463–499` die expliziten Aggregate. Da `platinen` selbst ein Aggregateintrag ist, bleibt kein nachlaufender RAM-Fallback, der seine Ausschlüsse wieder überschreibt. Alle 19 Materialien sind im additiven/bestehenden Katalog vorhanden; kein unaufgelöster Mappingkey gefunden.

Konkrete Nachweise statt einer Zählprüfung:

- `seeded_catalog_records_all_conditions_and_amount_change_history` (`kernschrott.rs:808–915`) vergleicht jede tatsächliche gespeicherte Annahmerow mit dem vollständigen Aggregate, prüft alle 41 gemappten Variantennamen und globale Texte sowie explizit Festplatten, Notebooks, Handys, Stecker, Netzteile und Platinen. Nach einem zweiten Preisrecord wird jede der 19 gespeicherten Annahmen erneut verglichen. Notizen aller 38 gespeicherten Preise werden ebenfalls vollständig mit den Parserlabels verglichen.
- Zusätzlich unabhängig die komplette Livehandler-Debugausgabe aus dem neuen Log extrahiert: **alle 41 vollständigen Originalnamen-plus-Beschreibungen** sind in ihrem jeweils richtigen Materialaggregate enthalten; die 38 numerischen Fraktionen zusätzlich in ihrem Preislabel. Alle 19 Aggregate enthalten beide vollständigen globalen Texte. Keine verlorene Beschreibung und keine zusammengefallene Geräte-/CPU-/Platinenfraktion gefunden.
- Mehrere Netzteil-/Handy-/CPU-/Platinenvarianten stehen nebeneinander mit jeweils zugehörigen Bedingungen; „Nur mit Kupferkern“ wird nicht pauschal allen Netzteilen zugewiesen. Nullpreis-Lüfter bleibt Preis plus bedingte Annahme; drei Anfragefraktionen bleiben ohne erfundenen Preis.

**Grenze der Freigabe:** Der unveränderte Shared-Record ist nicht atomar. Während eines laufenden Records können implizite Zwischenstände sichtbar sein; Schreibfehler/Abbruch zwischen Blank-Annahme und finalem Aggregate können Bedingungen unvollständig hinterlassen. Der erfolgreiche Endzustand ist nachgewiesen, keine Transaktions-/Crash-Garantie. Das ist kein neuer Kernschrott-Blocker im ausdrücklich handlerlokalen Scope; das Produktionsgate muss tatsächliche Endrows prüfen, nicht nur `accepted=19` (dieser Zähler allein beweist keine erfolgreichen Writes).

### F1 — geschlossen mit ausdrücklich konservativem Snapshot-Vertrag

`kernschrott.rs:184–187,239–250,287–291,329–344` prüft deklarierte Positionszahl gegen alle Nicht-Kategoriezeilen, vollständige Anwesenheit sämtlicher 41 bekannten Mappings plus Class 3, echte Kategorieform und nichtleere Pflichtbeschreibungen. Entfernte Zeile blockiert auch bei auf 41 korrigiertem Footer. Als `cat` maskierte Preiszeile scheitert. Die sechs erlaubten beschreibungslosen Varianten stimmen mit dem vollständigen Live-/Fixture-Befund überein; leere/umbenannte Festplatten-/Notebookbeschreibungen scheitern.

`blocked_source_mutations()` (`372–390`) enthält diese konkreten Mutationen; Drift-Test (`393–397`) prüft ihre Fehler. Der Record-Folgetest (`421–455`) speichert zunächst echte Current-/History-/Annahmezeilen und vergleicht deren vollständige Debug-Snapshots nach den abgewiesenen Kandidaten unverändert. Dieser Test simuliert die Parse→Record-Grenze, nicht den gesamten Scheduler; die tatsächliche Scrape-Funktion (`144–151`) propagiert den Parsefehler mit `?`, weshalb ein Fehler kein erfolgreiches HandlerOutcome zum Record liefert.

Zusätzliche unbekannte Sorten mit korrekt erhöhter Zahl bleiben erlaubte laute Skips, solange der bekannte Bestand vollständig ist (`658–687`). „Vollständiger Snapshot“ meint daher **vollständig für alle bekannten geprüften Varianten**, nicht die automatische Interpretation unbekannter Ergänzungen. Geänderte nichtleere Variantenbeschreibungen werden quellengetreu übernommen, nicht umfassend semantisch klassifiziert. Der Owner-Bericht benennt diese Grenzen jetzt korrekt.

### F3/F4 — Quellenbehauptungen abgesichert, Querygrenze akzeptiert

`kernschrott.rs:162–188` liest Vorbehalte aus genau einem Preisabschnitt, fordert jeweils genau einen passenden Knoten und vergleicht den **gelesenen** normalisierten Text mit dem überprüften Wortlaut. Fehlender, mehrdeutiger oder geänderter Text erzeugt einen Parsefehler. `global_conditions` wird aus den gelesenen Variablen `reservation` und `exclusion` aufgebaut, **nicht** trotz fehlender Quelle aus den Konstanten als angeblich aktueller Quelltext ergänzt. Preislabels und Materialaggregate erhalten diese gelesenen Texte. Die synthetische Aggregations-/Quarantäneerklärung ist erkennbar unsere konservative Einordnung, kein erfundenes Betreiberzitat.

Class 3 (`272–281`) verlangt zusätzlich exakte gefährliche Beschreibung und lesbaren EUR-Betrag vor Ausgabe der Widerspruchsbegründung. Änderung der Beschreibung/des globalen Ausschlusses oder unlesbarer Preis blockieren. Ein anderer numerischer Class-3-Betrag allein hebt die Quarantäne nicht auf; das ist sachlich angemessen. F4 geschlossen.

F3-Viewanteil: `v_current_prices` bleibt notes-frei. Überarbeiteter Owner-Bericht:56–71 dokumentiert `exact` als abgelesenen Betrag, keine Verbindlichkeit, und einen korrekten Join **über `current_prices.price_id`** auf Preisnotizen plus Materialannahmen. Diese bewusst gewählte Querygrenze ist für Main-Integration akzeptiert; eine reine Viewabfrage ist weiterhin kein Nachweis bedingungsfreier aktueller Ankaufzusage. Tatsächliche Sichtbarkeit beim konsumierenden Querypfad bleibt Produktionsgate.

### F2/S2 — dokumentierte Entscheidungen akzeptiert, keine versteckte Migration

- **F2:** Bekannter numerischer Preis→Anfrage und Anfrage→numerisch blockieren (`293–305`), ebenso Wegfall/Umbenennung/ungültiger bekannter Preis. Alte Current-/Historyrows bleiben ohne erneute Bestätigung bestehen; kein Withdrawal, kein künstliches Fortschreiben des Beobachtungsdatums durch einen fehlgeschlagenen Parse. Preis→Anfrage/Wegfall sind in den neuen Mutation-/Persistenztests enthalten. Anfrage→numerisch hat im gelesenen Testvektor noch keinen eigenen gezielten Mutationsfall, der Codeguard ist jedoch eindeutig; ergänzende Regression sinnvoll, kein Freigabeblocker. Owner-Bericht:50 legt Pflicht zur Beachtung von Beobachtungsalter und Scrape-Fehlern offen. Für den vereinbarten konservativen Umfang akzeptiert, keine allgemeine Lösung veralteter Current-Preise behauptet.
- **S2:** Neuer echter Record-Test (`457–513`) erwartet bei unverändertem Festplattenbetrag und geänderter Bedingung/neuem Datum dieselbe ID, genau eine Historyrow, neue Notizen/Beobachtung/Ingestion/Datum sowie aktualisierte Materialbedingungen. Der separate CPU-Betragsänderungstest erwartet weiterhin zwei Preisrows. Damit stimmen Dokumentation und tatsächliche Dedupe-Semantik überein. Owner-Bericht:52–54 und neue Dossier-Timeline behaupten keine immutable Bedingungshistorie. Keine Store-Migration erforderlich für diesen Auftrag; der allgemeine README-Append-only-Begriff bleibt weiter gefasst als der vorhandene Store und ist nicht durch diesen Handler „repariert“.

### Aktuelle Freigabe und verbleibende Gates

**Explizite unabhängige Review-Freigabe zur Main-Integration für Handlerhash `28ed6640…929c47e` und den oben gelesenen Begleitdiff. Keine verbleibenden blockierenden Befunde aus S1/S2/F1/F2/F3/F4 im vereinbarten Scope.** Neue Quellen-/Mapping-/Codeänderungen nach diesem Stand benötigen erneute Prüfung; diese Freigabe autorisiert selbst keinen Commit/Deploy.

Offen bleiben ausschließlich die bereits vereinbarten Integrations-/Betriebsschritte: koordinierte Integration/Rollout, erforderliche externe GitHub-Authentifizierung, und **regulärer** Produktions-Scheduler-Nachweis. Bei unveränderter Livequelle jetzt korrekt **38 Preise / 19 aggregierte bedingte Materialannahmen, davon 3 Anfragefraktionen / 1 Class-3-Skip** prüfen. Sämtliche 19 Endrows einschließlich aller Bedingungen, 38 Current-Varianten/Notizen/Quelle/EUR/kg/Seitenstand/Basis, angemessene History und Fehler-/Stalenessbehandlung über die tatsächliche Queryfläche nachweisen. Kein Summenzähler oder lokaler Livehandler ersetzt diese Read-only-Produktionsprüfung; hier keine DB geöffnet und kein Produktionsgate geschlossen.

## Historischer Erstbericht — Befunde vor der Überarbeitung

## Umfang und Evidenzgrenzen

- Geprüfter Worktree: `/root/Documents/schrott-mcp-kernschrott-20261008`, uncommittete Änderungen gegen dessen `HEAD`: `crates/ingestion/src/pipeline.rs`, `crates/ingestion/src/traders/handlers/mod.rs`, `dossiers/by/by-regensburg-kernschrott-recycling.md`; zusätzlich untracked `kernschrott.rs`, HTML-Fixture und Owner-Bericht einschließlich vorhandener Logs.
- README vollständig gelesen. Maßgebliche Regeln: handlerlokales Parsing und explizites Mapping (README:26–35); fester Materialkatalog, Annahmen mit Bedingungen, getrennte Preisvarianten, Herkunft/Datum und forward-only Current (README:73–90); erhaltene Dossier-Timeline und evidenzbasierte Änderungen (README:98–121, 163–182, 216–220). Keine `AGENTS.md` oder separate Coding-Standards-Datei im geprüften Worktree gefunden. Keine externe Issue-Spec verifiziert; fachlicher Maßstab sind Auftrag, README, Owner-Bericht und Betreiberquelle, nicht eine behauptete Vollprüfung eines GitHub-Issues.
- Am 10.10.2026 unabhängig direkt abgerufen: <https://www.kernschrott-recycling.de/elektro> und <https://www.kernschrott-recycling.de/impressum>. Preisseite zusätzlich mit Python-Standardbibliothek HTTP-read-only ausgelesen; Name, Beschreibung und Preis **aller 42 Positionen** mit Fixture verglichen: vollständig identisch. Kein Rust-Handler durch diesen Reviewer ausgeführt.
- Live-Preisabruf: HTTP 200, finale URL `/elektro`, 40824 Bytes, SHA-256 `c958c911c85c9b634ed5d32ed98316c0d65b793a4e5e4b8681a9849fb2d5f753`. Fixture: 38191 Bytes, SHA-256 `b1a313c9cfc2d3dea5845984f9b98190eac7ba07daedddc61252e466a36db532`. Unterschiedliche Gesamtseiten, aber identische geprüfte Preiszeilen; keine Behauptung bytegleicher Live-Fixture.
- Gemeinsamen `record()`- und Store-Code ausschließlich gelesen. **Keine Builds, Tests, Commits, Datenbankzugriffe, Deployment- oder Worktree-Schreibzugriffe.** Einzige geschriebene Datei ist dieser Bericht im Main-Research-Verzeichnis. Vorhandene Testlogs sind fremde Ausführungsevidenz, kein eigener Testnachweis.

## Standards-Achse

### S1 — Hoch: Annahmebedingungen gehen im tatsächlichen Record-Pfad verloren

`kernschrott.rs:230–240` bewahrt Bedingungen nur in Preislabels. `traders/mod.rs:416–426` speichert für jeden numerischen Preis dagegen `accepts=true, conditions=""`; `store/src/public/prices.rs:658–665` überschreibt bestehende Bedingungen damit. Das ist eine konkrete Integrationslücke gegenüber README:75–76, nicht bloß ein Stilgeruch und nicht ausschließlich neu eingeführter Shared-Code.

Belege aus der Livequelle: Festplatten „Platine muss vorhanden sein, ansonsten erfolgt kein Ankauf“; Notebooks „unberaubt, ohne Akku“; Handyvarianten „Akku muss entfernt sein“; Steckernetzteile „Nur mit Kupferkern“; Computerstecker nur vergoldet, abgetrennt und ohne Ferritring. Die gespeicherte Materialannahme sagt dennoch uneingeschränkt ja. Besonders kritisch: generisches `platinen=true` enthält keinen seitenweiten Gefahrstoffausschluss und keine Class-3-Abgrenzung. Der Class-3-Skip verhindert deren Preis, **nicht** eine irreführend unbedingte generische Platinenannahme.

**Gate:** Materialannahmen müssen quellengetreue Bedingungen einschließlich globaler Ausschlüsse erhalten; bei mehreren Varianten sinnvoll aggregieren, nicht die letzte Variante als einzige Bedingung speichern. Tests müssen die tatsächlichen Annahmerows für Festplatten, Geräte, Steckernetzteile und Platinen prüfen. Falls Shared-Code bewusst unverändert bleiben soll, braucht es einen expliziten, fachlich tragfähigen handlerlokalen Weg; die drei vorhandenen Anfrageannahmen allein lösen das nicht.

### S2 — Mittel: „Append-only“ und History-Nachweis müssen enger beschrieben werden

README:77 und `traders/mod.rs:25` nennen append-only. Tatsächlich dedupliziert `store/src/public/prices.rs:489–527` gegen den aktuellen Preis und verändert dessen `observed_at`, Notizen, Quelle sowie gegebenenfalls Publikationsdatum/Basis. Gleichbleibender Betrag bei geänderter Sortierbedingung oder neuem Seitenstand ergibt **keine neue historische Beobachtung**. Das ist eine bestehende Store-Eigenschaft, keine neue Migration des Kernschrott-Diffs.

Der neue Test `kernschrott.rs:540–599` belegt im Code die beabsichtigte Prüfung aller initialen Preisrows und einer CPU-Betragsänderung (308 → 321,50), nicht unveränderliche Beobachtungs-/Bedingungs-/Datumshistorie. „Vorhandene Katalog-/Preisgeschichte bleibt unverändert“ im Owner-Bericht:8 ist als Aussage über den additiven Diff plausibel, nicht als Garantie jeder zukünftigen Record-Ausführung.

**Gate:** Owner-Dokumentation auf tatsächlich belegte Preisänderungshistorie begrenzen; gewünschte History-Semantik für unveränderten Betrag/geänderte Bedingungen explizit entscheiden und gezielt testen. Nicht ohne gesonderten Auftrag den gesamten Store umbauen.

### Sonstige Standards

- Ein Handlerfile, eine Modul-/Registry-Ergänzung, gemeinsame HTTP-/Zahl-/Datumshilfen und handlerlokale strenge Betragsprüfung entsprechen dem Repo-Muster. Keine neue Dependency; Katalogdiff nur additiv (13 Slugs), keine Änderung bestehender Materialdefinitionen und keine History-Migration.
- Dossier bleibt beim stabilen Slug und ergänzt die Timeline statt alte Recherche zu löschen. Identität und Anschrift stimmen mit dem Live-Impressum überein. Die historische Register-/HR-Kongruenz aus der Recherche vom 01.10. wurde hier nicht erneut extern geprüft; keine neue Umbenennungs-/Zusammenführungsentscheidung im Diff.
- Keine belastbare zusätzliche Fowler-Smell-Beanstandung: explizite Mappingliste und vollständiger Erwartungsvektor im Test sind hier fachlich nützliche Redundanz, keine zu abstrahierende Parser-Duplizierung.

## Fach-/Spec-Achse

### F1 — Hoch: Fehlende Listenzeilen und Teile der Strukturänderungen sind nicht fail-closed

`kernschrott.rs:181–250` iteriert nur vorhandene `tbody tr`, ignoriert jede Zeile mit Klasse `cat` bedingungslos und fordert am Ende nur `count > 0`. Es gibt keinen Abgleich zwischen erkannter Zeilenzahl und der auf der Seite angegebenen Positionszahl, keine Prüfung fehlender bekannter Varianten und keine Pflichtbeschreibungen für konditionierte Fraktionen.

Konkrete durch den Kontrollfluss belegte Gegenbeispiele (statisch geprüft, keine Rust-Ausführung):

1. Eine komplette bekannte Preiszeile entfernen, Footer weiter „42 Positionen“: Parse liefert erfolgreich 37 Preise / 3 Annahmen / 1 Skip; für die fehlende Sorte entsteht kein Skip.
2. Einer echten Zeile zusätzlich `class="cat"` geben: wird vor Namens-/Preisvalidierung still verworfen.
3. `.ds` von Festplatten oder Notebooks entfernen/umbenennen: gültige Preiszeile ohne Ankaufsvoraussetzung, weil null Beschreibungen ausdrücklich zulässig sind.

Die Shared-Canary greift erst bei weniger als der Hälfte der bisherigen Preisanzahl (`traders/mod.rs:538–542`), erst nach Speicherung und ohne Kenntnis einzelner Varianten. Ein Verlust von 1 aus 38 löst sie nicht aus. Vorhandene Strukturtests prüfen einzelne Feld-/Einheitsfehler, nicht diese Fälle. Owner-Bericht:7,20,28 muss „fail-closed“ entsprechend einschränken.

**Gate:** Vollständigkeit gegen Seiten-Positionszahl und bekannte Sorten/Drift gezielt absichern; echte Kategoriezeilen von versehentlich maskierten Positionszeilen unterscheiden; Verlust semantisch nötiger Beschreibungen melden/blockieren. Tests für ganze entfernte Zeile, falsche `cat`-Klassifikation und verschwundene Ankaufsvoraussetzung ergänzen. Keine starre Einbettung aktueller Preise; legitime Listenänderungen sollen weiterhin überprüfbar sein.

### F2 — Mittel: Preis → Anfrage/Entfernung lässt einen alten Current-Preis bestehen

Der Mutationstest `kernschrott.rs:418–426` prüft nur `HandlerOutcome` bei CPU-Preis → Anfrage. Im echten Record-Pfad wird dann eine materialweite Annahme gespeichert (`traders/mod.rs:463–499`), aber der bereits gespeicherte CPU-Variantenpreis weder beendet noch aus `current_prices` entfernt. Vollständig verschwundene oder umbenannte Zeilen haben dasselbe Problem. Preis- und Annahmenhistorie sind zu unterscheiden: `trader_materials` ist eine mutable Tabelle ohne Variantenkey (`prices.rs:67–77,658–665`). Mehrere Anfrage-CPU-Sorten würden außerdem jeweils dieselbe Annahmebedingung überschreiben.

Das ist eine bestehende Pipeline-Grenze, die der neue dynamische Handler und sein Anfragewechsel-Test sichtbar machen. Initiale drei Anfragefraktionen haben im geprüften Snapshot keinen numerischen Preis; deren jetzige Darstellung ist richtig. Ein späterer Anfragewechsel ist aber nicht vollständig integriert nachgewiesen.

**Gate:** Übergang numerischer Preis → Anfrage und Wegfall einer Variante durch gezielten Record/Current/History-Test nachweisen; History behalten, Current-/Gültigkeitsverhalten explizit entscheiden. Bis zur Lösung keine Aussage, dass der Handler solche Änderungen vollständig semantisch verarbeitet.

### F3 — Mittel: Unverbindlichkeit und globaler Ausschluss sind nur teilweise konsumierbar

Für die aktuelle Quelle korrekt: alle numerischen Preislabels und Anfragebedingungen erhalten „unverbindlich, Rücksprache vor Verkauf“. `exact`/`confidence=1.0` ist gemäß `ScrapedPrice`-Kommentar (`traders/mod.rs:110–115`) ein exakt abgelesener Listenbetrag, **keine** Zusage eines verbindlichen Ankaufspreises; daher kein isolierter Klassifikationsfehler.

Aber `v_current_prices` enthält keine Preisnotizen (`store/src/public/prices.rs:88–97`). Die häufigste Queryfläche zeigt somit `exact`/1.0 ohne diesen Vorbehalt. Ein Join auf `prices` kann ihn abrufen, eine reine View-Abfrage nicht. Der globale Gefahrstoffausschluss ist nur als Lüfterbeschreibung mitgenommen und im Class-3-Skip erwähnt; bei anderen Preislabels fehlt er. Der Parser überprüft den tatsächlichen Vorbehalt/Ausschluss nicht und fügt seinen fest codierten Unverbindlichkeitstext auch bei einer später geänderten Seite unverändert hinzu (`kernschrott.rs:224–240`).

**Gate:** Koordinierender Owner muss entscheiden, wie Vorbehalte über die tatsächliche Queryfläche sichtbar werden und wie geänderte globale Bedingungen erkannt werden. Mindestens globalen Ausschluss in den gespeicherten Annahmebedingungen erhalten (S1) und dokumentieren, dass `exact` keine Verbindlichkeit meint. Ein Wegfall/geänderter Ausschlusstext darf nicht weiterhin als geprüftes Quellfaktum gelten.

### F4 — Niedrig: Class-3-Quarantäne ist konservativ richtig, Begründung nicht dynamisch

Die Livequelle enthält gleichzeitig Class 3 mit 1,60 EUR/kg und gefährlichen Stoffen sowie „Kein Ankauf“ solcher Materialien. Nicht als Preis/Annahme zu übernehmen ist richtig. Der Skip (`kernschrott.rs:210–215`) ist aber allein vom Namen abhängig und behauptet den Widerspruch auch dann, wenn Beschreibung oder globaler Ausschluss künftig verändert werden. Die vorsichtige Quarantäne kann bleiben; ihre Begründung sollte Quellenzustand und „erneut prüfen“ unterscheiden. Ein Skip ist auch keine explizit gespeicherte Class-3-Negativannahme, da Annahmen keinen Variantenkey haben.

## Vollständiger Live-/Fixture-Abgleich

Alle Namen, Beschreibungen und Beträge waren zwischen Livequelle und Fixture identisch. Nachstehend die vollständige geprüfte Zuordnung; numerische Werte sind EUR/kg, Varianten bleiben die Originalnamen. Die 38 Preis-Tripel entsprechen dem expliziten Erwartungsvektor `kernschrott.rs:326–373`; kein Modellnummernpreis und keine Fraktionsverwechslung gefunden.

| # | Originalposition | Material | Preis/Behandlung |
|---|---|---|---|
| 1 | Leiterplatten Kl. 1A | platinen | 15,00 |
| 2 | Leiterplatten Kl. 1B | platinen | 4,00 |
| 3 | Leiterplatten Kl. 2 | platinen | 8,00 |
| 4 | Leiterplatten Kl. 3 | — | 1,60 veröffentlicht, quarantänisiert |
| 5 | Steckkarten | platinen | 15,00 |
| 6 | Steckkarten mit Blende | platinen | 10,00 |
| 7 | Steckkarten mit Blende und Kühler | platinen | 6,50 |
| 8 | Laptopplatinen | platinen | 15,00 |
| 9 | IC's | integrierte-schaltungen | 18,00 |
| 10 | Handyplatinen | platinen | 28,00 |
| 11 | Handys komplett ohne Akku | handys | 18,00 |
| 12 | Smartphones komplett ohne Akku | handys | 5,00 |
| 13 | Prozessor Keramik Intel 286, 386, 486 | prozessoren | 308,00 |
| 14 | Prozessor Keramik Goldcap | prozessoren | 198,00 |
| 15 | Prozessor Keramik | prozessoren | 160,00 |
| 16 | Prozessor Kunststoff Schwarz | prozessoren | 110,00 |
| 17 | Prozessor Keramik mit Aludeckel | prozessoren | 90,00 |
| 18 | Prozessor Slot | prozessoren | 78,00 |
| 19 | Prozessor Kunststoff | prozessoren | 60,00 |
| 20 | Prozessor Kupferkern 90 % Pin | prozessoren | 10,50 |
| 21 | Prozessor Kupferkern 50 % Pin | prozessoren | 9,50 |
| 22 | Prozessor Kupferkern ohne Pin | prozessoren | 6,00 |
| 23 | Arbeitsspeicher Goldkante | ram | 68,00 |
| 24 | Arbeitsspeicher Silberkante | ram | 28,00 |
| 25 | Arbeitsspeicher Goldkante mit Aluleiste | ram | 28,00 |
| 26 | Computerstecker | elektronikstecker | 1,50 |
| 27 | Flachbandkabelstecker | elektronikstecker | 1,50 |
| 28 | Kabel ohne Stecker | kabel-ohne-stecker | Anfrage, nur Annahme |
| 29 | Kabel mit Stecker | kabel-mit-stecker | Anfrage, nur Annahme |
| 30 | Laptops, Notebooks | notebooks | 2,80 |
| 31 | Festplatten | festplatten | 2,20 |
| 32 | Netzteile mit Kabel | netzteile | 1,10 |
| 33 | Computer komplett unberaubt | computer | 0,90 |
| 34 | Laufwerke | laufwerke | 0,85 |
| 35 | Netzteile ohne Kabel | netzteile | 0,90 |
| 36 | CDs | optische-datentraeger | 0,10 |
| 37 | Ablenkeinheiten | ablenkeinheiten | 2,80 |
| 38 | Panzersicherungen | panzersicherungen | 1,00 |
| 39 | Steckernetzteile | netzteile | 0,25 |
| 40 | Elektrobauteile | elektrobauteile | 0,20 |
| 41 | Alu-Cu-Kühler ohne Eisen | alu-cu-kuehler | Anfrage, nur Annahme |
| 42 | Lüfter | luefter | 0,00, echter veröffentlichter Nullpreis |

**Nullpreis:** `quoted_amount()` akzeptiert 0,00; Preis wird nicht anhand von `> 0` verworfen. Der gemeinsame Record-Pfad speichert ihn als Preis und implizite Annahme. Hier sachlich richtig: „Kein Ankauf“ in der Lüfterbeschreibung bezieht sich auf enthaltene Gefahrstoffe, nicht auf jedes Material oder jeden Lüfter. Nullpreis bedeutet keine Vergütung, nicht automatisch keine Annahme.

**Materialien:** Ganze Geräte, CPUs, ICs und Stecker werden nicht als Platinenpreis gespeichert. Generisches Kabel ohne Stecker trägt keinen unbelegten Kupfer-/Aluminiumanteil. CDs werden nicht als Laufwerke und Lüfter nicht als lose Motoren zugeordnet. Katalogänderungen nachvollziehbar und vollständig für die neuen Mappings.

**Datum:** Live-/Fixture-Stand 06.10.2026 → `2026-10-06T00:00:00+00:00`, als Datumskonvention des vorhandenen Helpers, nicht erfundene Uhrzeit der Veröffentlichung. `published_at_basis=page_stated` wird im gemeinsamen Record-Pfad erzeugt. Ungültige/mehrdeutige `Stand:`-Angaben scheitern; fehlendes `Stand:` wird `None`, danach gelten dokumentierte Shared-Inferenz-/Dedupe-Regeln. Datum wird allerdings aus dem gesamten Dokumenttext gelesen, nicht spezifisch aus dem Tabellenfooter; anderer „Stand:“ kann den Scrape blockieren. Tests decken fehlendes Datum im Parser und Shared-Inferenz separat ab, nicht alle Kernschrott-Persistenzkombinationen.

**Identität:** Live-Impressum nennt Kernschrott-Recycling UG (haftungsbeschränkt), Dieselstr. 7, 93053 Regensburg, HRB 12051, Geschäftsführer Michael Bitterbier. Passt zu Dossier/Handler. Vier erwartete Identitätsstrings werden je Scrape geprüft; kein neues Stammdaten-Enrichment, keine Slugänderung. Das beweist erwartete Seitenidentität, nicht einen neuen unabhängigen Registeraudit.

## Tests: konkrete Aussage statt grüner Gesamtzahl

- Fixture-Test prüft alle 38 Varianten/Material/Betrags-Tripel, die drei Anfrage-Materialien, Sortentrennung und ausgewählte Bedingungen. Gut; nicht jede Beschreibung wird als exakte unabhängige Erwartung asserted. Der eigene Live-/Fixture-Abgleich dieses Reviews umfasst dagegen alle Beschreibungen.
- Guard-Tests belegen im Code Unit/Datum/Duplikat/fehlendes Feld/ungültigen Betrag und unbekannte Sorte; Grenzen unter F1 bleiben offen.
- Seed→Record→History-Test prüft alle initialen Preise einschließlich Quelle, EUR/kg, Datum, Notizen und Basis sowie drei Anfrageannahmen und eine CPU-Preisänderung. Keine Prüfung der Annahmebedingungen preisbehafteter Fraktionen, von Current-Pointern, Anfragewechseln nach Vorpreis, Wegfall oder unverändertem Betrag bei geänderten Bedingungen.
- Live-Test im Owner-Log ist tatsächlich der Handler einschließlich Impressumsfetch, nicht nur ein Webseitenabruf. Er prüft grobe Verteilung und ausgewählte Werte, nicht die vollständige spätere Produktionsspeicherung. Das vorhandene Log unterstützt den Owner-Nachweis; nicht unabhängig hier neu ausgeführt.
- Workspace-Log berichtet erfolgreiche Suite, bindet die einzelnen Testergebnisse aber nicht selbst an einen unveränderlichen Commit/Dateihash. **569 grüne Tests sind kein Ersatz für die genannten semantischen Nachweise.**

## Freigabegates für den koordinierenden Owner

1. **Vor Main-Integration blockierend:** S1 (gespeicherte bedingte Annahmen/globaler Ausschluss) und F1 (vollständige Liste/Strukturdrift) beheben und gezielte Regressionen nachweisen. Kein „vollständig fail-closed“ ohne diese Nachweise.
2. **Vor Freigabe explizit entscheiden:** F2 (Current bei Anfrage/Wegfall), F3 (Querysichtbarkeit/Änderung globaler Vorbehalte), S2 (reale History-Semantik). Shared-Grenzen nicht als bestandene Handlergarantien verkaufen; Änderungen oder bewusst dokumentierte Einschränkungen mit zuständigem Owner abstimmen. F4-Begründung auf tatsächlichen/prüfbedürftigen Quellenzustand begrenzen.
3. Owner-Bericht und gegebenenfalls Dossier bei Umsetzung berichtigen: keine pauschale Fail-closed-/History-Garantie; aktuelle 42er-Liste und konservative Quarantäne sind gut belegt. Fixture nicht zur Anpassung an einen gewünschten Teststand manipulieren.
4. Nach finalen Änderungen gezielte Testresultate für die obigen Fälle sowie Format/Diff/Seed-/Workspace-Gates durch den implementierenden Owner neu erfassen. Dieser Reviewer hat sie ausdrücklich nicht ausgeführt.
5. **Gesondertes Produktionsgate, hier offen:** koordinierter Rollout beider Binaries, danach erfolgreicher regulärer Scheduler-Schritt für den Slug (bei unveränderter Quelle 38 Preise, 3 explizite Anfrageannahmen, Class-3-Skip, keine Canary). Read-only verifizieren: echte gespeicherte Annahmebedingungen, alle Current-Varianten samt EUR/kg/Quelle/Seitenstand/Basis und angemessene Preisgeschichte; Vorbehalte über die konsumierte Queryfläche sichtbar. Lokaler Livehandler oder manueller Prod-Scrape ersetzen den regulären Nachweis nicht. Keine Produktionsabfrage durch diesen Reviewer vorgenommen.

**Achsenfazit:** Standards: 2 konkrete Befunde, schwerster S1 (Annahmebedingungen). Fach/Spec: 4 Befunde, schwerster F1 (unbemerkter Zeilen-/Bedingungsverlust). Aktuelle Werte und Materialzuordnung sind freigabefähig belegt; Integrations-/Drift-/Produktionsgarantien noch nicht.
