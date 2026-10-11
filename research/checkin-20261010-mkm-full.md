# MKM Vollabdeckung — laufender isolierter Check-in

## Persistierter Wiederanlauf / P2-Behebung (11.10.2026)

Vorhandene Arbeit fortgesetzt, nicht neu implementiert. Vier neue Offline-Regressionen `p2_1` bis `p2_4` wurden vor den Korrekturen tatsächlich ausgeführt: **alle vier rot**, Exit 101 (hidden Zeile wird Quote; `/kgCO2` akzeptiert; Straße 40 akzeptiert; SW10040 fälschlich `zinn`). Danach bereits im Worktree umgesetzt, aber beim Wiederanlauf **noch nicht grün nachgewiesen**: explizite DOM-Sichtbarkeit einschließlich Vorfahren; separate verankerte Zell-/deutsche Zahlengrammatik; exakter Impressumsadressblock und Registerwert; additive ganze Fraktion `zinn-rein-formgemischt`. Kerze-Kandidat bereits auf additive `kupfer-stanzabfaelle / Kerze` korrigiert, Legacy-Registrierung unverändert und Alias-Migration weiterhin offen.

Noch zu erledigen: vorhandene Korrekturen testen, unabhängige 60-Artikel-Oracle hinzufügen und prüfen, Legacy-Stabilitätstest um dokumentierte Kerze-Ausnahme berichtigen, Live-/Python-/Ingestion-Evidenz und Patch aktualisieren; Behebung an unveränderte Reviewhistorie anhängen. **Kein P2-Abschluss oder Re-Review-Freigabe bis dahin.** Reservierte Store-/mod-Pfade weiterhin unberührt. Eigenes Target weiterhin `/tmp/opencode/mkm-full-target-20261010`, freier `/tmp`-Platz beim Wiederanlauf 1,8 GiB; kein Fremd-Delete.

### Persistierter Zwischenstand nach Korrekturtests (11.10.)

Die vier P2 sind jetzt über echte gespeicherte Quellenmutationen grün; insgesamt **17 MKM-Tests bestanden** einschließlich echter TempDB. Die neue Sichtbarkeitstest-Erweiterung traf zunächst wegen falscher `section`-/erster-Tabellen-Ersetzung nicht die Produktfixture; das wurde ausdrücklich korrigiert und jede Mutation muss nun die echte Produktzeile erreichen. Keine grüne Behauptung aus einer No-op-Mutation.

Zusätzliche echte rote Python-Reproduktion: `test_real_snapshot_all_battery_wrong_is_not_semantically_green` scheiterte vor Auditkorrektur mit `[] is not true`. Separate feste 60-Artikel-Oracle jetzt ergänzt, aus dem unabhängigen Detail-/Katalogreview und den zwei expliziten Fachkorrekturen, nicht aus `MAPPINGS` erzeugt. Python vergleicht jetzt sowohl Material als auch Variant; fehlende Oracle-Einträge sind `mapping semantics NOT CHECKED`, nie stilles Grün. Historisches Kandidaten-JSON vom 10.10. bleibt unverändert und zeigt mit neuer Oracle genau die beiden Materialkonflikte Zinn 99 und Kerze.

Noch offen in diesem Zwischenstand: finale Ingestion/Python/Format-/Live-Gates, aktualisierter Patch und unabhängiger Re-Review. P1-Shared-Seam-/Alias-Aktivierungssperre unverändert.

### P2-Behebungsevidenz und Aliasvertrag (11.10., vor Re-Review)

Die älteren nachfolgenden Berichtsteile dokumentieren den Stand vom 10.10.; deren damalige Vollständigkeits-/Legacyformulierungen werden durch den unabhängigen Review und diesen Nachtrag eingeschränkt, nicht rückwirkend umgeschrieben.

- **P2-1:** `explicitly_hidden` prüft Produktzeile, Preisinhalt und sämtliche Elementvorfahren auf HTML-`hidden` und inline `display:none`, einschließlich Schreibweise/Whitespace/`!important`. Solche Produkte werden nicht als aktuelle Preise ausgegeben; bekannte fehlende Artikel bleiben `missing known article; retirement NOT authorized`. `visually-hidden`-Captions bleiben gültig. Offline-Tests mutieren die echte Zeile, Kupfer-Section, Preiszelle und `strong`-Preisinhalt. Keine Aussage über externe Stylesheets, responsive CSS oder vollständige Browser-Sichtbarkeit; Scope steht ausdrücklich im Kandidaten-JSON.
- **P2-2:** separate Kandidatengrammatik für die **gesamte** normalisierte Preiszelle: deutsche Zahl mit genau zwei Nachkommastellen, `€ /kg ab <deutsche ganze kg-Menge> kg`. Ganze Zahlen ohne führende Null und mit korrekt dreistellig gruppierten Tausenderblöcken; keine Legacy-Helferänderung. `/kgCO2`, Sondervertrag/netto, mehrfaches `ab`, falsche Gruppierung `1.1,13`, Dezimalpunktpreis, negatives/Nullangebot und zusätzliche Tokens blockieren. Gültig getestet: `1.111,13 € /kg ab 10.000 kg`. Sichtbarer `strong`-Preis muss zudem mit kompletter Zelle übereinstimmen.
- **P2-3:** exakter, verankerter normalisierter DDG-Betreiber-/Adressabsatz; Registerwert wird aus seinem eigenen `Registereintrag`-Absatz vollständig als `HRA 61664` gelesen. Mutierte echte Impressumsfixture: Straße 40, HRA 616640, PLZ 670630, Firmenzusatz und doppelte Registernummer abgelehnt. Kein neuer amtlicher Registerbeleg behauptet.
- **P2-4:** `SW10040` genau einmal als `zinn-rein-formgemischt / Zinn 99%`: nahezu unlegiertes Zinn, ganze Quellenfraktion aus Bangka-Zinn, Anoden, Barren/Blöcken sowie hochreinen Geschirr-/Figurenwaren. Nicht niedrigprozentiges Lot, nicht ausschließlich Geschirr. Bestehendes `zinn` mit explizitem Geschirrausschluss und `zinn-geschirr` unverändert. Echter TempDB-Seed testet die neue breite Definition und den unveränderten alten Ausschlusstext. Neuabruf `/metallankauf/produkt/zinn-99` am 11.10. bestätigt erneut „hochreines Geschirrzinn“ und „Zinngeschirr und Zinnfiguren“; Exzerpt/Hash in `research/mkm-p2-primary-evidence-20261011.json`.
- **Oracle:** `fixtures/mkm-reviewed-mapping-oracle-20261011.json` ist eine separate feste 60-Artikel→Material/Variant-Tabelle aus dem unabhängigen Review, mit den begründeten Korrekturen für SW10040 und MKM-CU-01. Nicht aus Implementierungs-`MAPPINGS` erzeugt. Rust prüft jede Kandidatenzeile dagegen, Python jede Material- und Variantangabe. Alle-Batterien-/WRONG-Mutation war zuvor real rot und ist jetzt diagnostiziert; unbekannte Oracleartikel ausdrücklich „NOT CHECKED“. Audit benennt Snapshot-/Fachoracle-/Sichtbarkeitsgrenzen und bleibt `production_approval=false`.

#### Kerze: fachliche Korrektur im Kandidaten, zwingende künftige Migration

Neu abgerufene Detailquelle `/metallankauf/produkt/kupfer-kerze` beschreibt blanken Stanzabfall aus Kupferband (Gitter/Butzen/Randstreifen), **keinen flexiblen Berry-Draht**. Der Kandidat verwendet deshalb die additive Fraktion `kupfer-stanzabfaelle / Kerze`. Der registrierte Legacy-Handler bleibt technisch unverändert, seine `kupfer-berry / Kerze`-Zuordnung wird **nicht fachlich freigegeben** und darf bei künftiger Aktivierung nicht stillschweigend übernommen werden.

Exakte erforderliche Alias-/Retirementmigration:

1. Scope ausschließlich Händler `rp-ludwigshafen-mkm-metals`, Overviewquelle `https://mkm-metals.de/metallpreise`, belegter Artikel `MKM-CU-01`, Deep-Link `https://mkm-metals.de/metallankauf/produkt/kupfer-kerze` und Rohlabel `Kupfer Kerze`.
2. Altkey `(kupfer-berry, Kerze)` → Nachfolger `(kupfer-stanzabfaelle, Kerze)`. Erst gemeinsamer verifizierter vollständiger Artikel-Snapshot darf in **derselben atomaren Operation** den Nachfolger aufzeichnen und das quellenbelegte Alt-Current retireden; Preis-/Mengen-/Datumsevidenz muss im neuen Vertrag bestehen bleiben.
3. Append-only alte Preisbeobachtungen nicht löschen, nicht Material-ID oder historische Quellendaten umschreiben. Alias-/Lineageevidenz dokumentieren, damit alte Historie auffindbar bleibt. Unknown-/NULL-/fremde Provenienz am Altkey ist eine manuell zu klärende Sperre, kein Freibrief für Retirement.
4. Keine materialweite Berry-Neuzuordnung, keine globalen Variant-Aliase, keine anderen Händler oder Quellen verändern. Selbst bei gleicher Variant `Kerze` und gleichem Betrag Fremdquelle strikt schützen. Shared-TempDB-Tests müssen das samt Rollback/Idempotenz beweisen.
5. Diese Migration und die bereits bekannte falsche Steckerkabel-Migration sind hier **nur spezifiziert**, nicht implementiert/aktiviert. Die zehn übrigen bisherigen Kandidatenidentitäten bleiben stabil. `required_alias_migrations` ist nun explizit im Kandidaten-JSON enthalten.

#### Finale Gates dieses Behebungsstands

- Ingestion: **567 bestanden, 5 ignoriert**; insgesamt 572 Tests. MKM umfasst 17 bestandene Tests einschließlich aller vier P2 und echter TempDB. Python: **9 bestanden**, einschließlich historisch fehlerhafter Kerze-/Zinn-Oracles und der All-Batterien-/WRONG-Mutation.
- Geänderte Rustdateien `rustfmt --check` erfolgreich; `git diff --check` erfolgreich; reservierte Store-/mod-Diffs leer. Kein neues globales Clippy-Grün behauptet (historischer Repo-Lintblocker bleibt).
- Livequellen am 11.10.00:06 UTC: Overview HTTP 200, 253811 Bytes, SHA256 `0bf08f47395c3436e97c8c002b150f4bb89a38d08425599a4f33610e3a0baa7e`; Impressum HTTP 200, 28398 Bytes, SHA256 `77ca243d58b3e3127eccfb6edaf5bc17e36d0cbd5063ef166f56370e7f153f6d`. Neu gebauter echter Parser auf exakt diesen frischen Abrufen: **8/60/55/5**, Diagnostik leer, `production_ready=false`.
- Neue, nicht rückwirkend überschriebene Evidenz: `research/mkm-full-live-inventory-20261011.json`, `research/mkm-full-contract-audit-20261011.json`. Alle 60 Artikelverträge und fixed-oracle-Mappings stimmen; die 60 **gespeicherten Detailabrufe vom 10.10.** stimmen weiterhin in sichtbaren Staffeln/Identität. Nicht alle 60 Details am 11.10. erneut abgerufen behauptet; Zinn99/Kerze wurden am 11.10. neu abgerufen und dokumentiert.
- Originalreview bleibt erhalten; Behebung wird daran angehängt. Re-Review und finaler Übergabepatch folgen. Kein Main-/Prodwrite, keine Commits/Push/Deploy; P1-Shared-Seam und beide Aliasmigrationen blockieren weiterhin jede Produktionsaktivierung.

### Unabhängiger Re-Review technisch blockiert / Übergabe (11.10.)

Zusätzlicher selbst ausgeführter Blackbox-Nachweis mit dem **neu gebauten echten Example-Binary**: `research/mkm-p2-offline-repros-20261011.json`. Baseline Exit 0/55; hidden Zeile, inline display:none, verborgener Preisvorfahr und verborgene Kupfersection jeweils Exit 1, Artikel fehlt explizit und Retirement nicht autorisiert. `/kgCO2`, Sondervertrag, `1.1,13`, Straße 40 und HRA 616640 jeweils Exit 1 mit konkretem Parsefehler. Zinn99-/Kerze-Zielmappings und `production_ready=false` im Baselineoutput separat geprüft. Dies ist ein anderer Testpfad (CLI statt Unit), **kein unabhängiger Agentenreview**.

Nach den finalen Behebungsgates wurden getrennte Standards- und Spec-Re-Reviewer angefordert. **Beide Aufrufe scheiterten vor dem Start** mit `Subagent depth limit reached (1). Increase experimental.subagent_depth to allow nested subagents.` Keine Reviewer-Session oder unabhängige Freigabe daraus abgeleitet, keine Plugin-/Harnesskonfiguration geändert und nicht erneut versucht. Der Owner muss auf übergeordneter Sessionebene den unabhängigen Re-Review starten. Behebung aller vier ursprünglichen P2 ist durch eigene Red/Green-/Live-/Oracle-Evidenz belegt, **unabhängiger Re-Review bleibt offen**.

Aktualisierter Vollpatch einschließlich neuer Oracle/Behebungsevidenz und append-only Reviewnachtrag: `/tmp/opencode/mkm-full-p2-20261011.patch`. Der ursprüngliche `/tmp/opencode/mkm-full-20261010.patch` bleibt als historischer Reviewstand unverändert erhalten. Die neue lokale Quellen-/Mappingarbeit ist weiterhin ausschließlich **proof-only**, keine produktive Freigabe. Trackerstandardsdatei `docs/agents/issue-tracker.md` fehlt laut vorigem Review; für spätere trackerbasierte Reviews `/setup-matt-pocock-skills` verwenden. Das lokale Originalreview und Userbriefing waren hier die maßgebliche Spec, keine Tracker-/Setupänderung vorgenommen.

Basis `6d5ef762`, Worktree `/tmp/opencode/schrott-mcp-mkm-full-20261010`, Branch `handler/mkm-full-20261010`. Main und Deploy-Worktree unverändert. Eigenes `CARGO_TARGET_DIR=/tmp/opencode/mkm-full-target-20261010`.

Auftrag: vollständige artikelbewusste Partition aller echten Ankaufproduktzeilen, Bedingungen und Quellenidentität erhalten, keine erfundenen Preise. Test-first über gespeicherte aktuelle Betreiberquelle und echten Parser/Outcome, anschließend isolierte TempDB. Noch keine Vollständigkeitsfreigabe, keine Produktionswrites, kein Commit/Push/Deploy.

## Aktueller Stand

- Alte Diagnosen gelesen: 8 Tabellen / 60 Artikel / 55 numerisch / 5 Anfrage, bisher 11 numerische Handlerpreise. Kupfer-only und unsichere generische Labelarme bestätigt.
- Quellen werden neu abgerufen; Artikelidentität, Beschaffenheit, Mengen und Datum müssen geprüft werden.
- Store und `crates/ingestion/src/traders/mod.rs` sind reserviert für den parallelen MSG-/Shared-Seam-Agenten und werden hier nicht editiert.

## Freigabesperren / benötigte Shared Seam

1. Vollständiger Snapshot muss alle Artikelidentitäten partitionieren und unbekannte/geänderte/duplizierte Artikel ausdrücklich melden; reine Count-Gleichheit genügt nicht.
2. Historieerhaltendes Retirement ausschließlich für Händler `rp-ludwigshafen-mkm-metals` und Quelle `https://mkm-metals.de/metallpreise`, atomar mit akzeptiertem vollständigem Snapshot. Keine Retiremententscheidung bei Mapping-/Parsegaps oder unvollständigem HTTP-Ergebnis.
3. Alter Current-Schlüssel `kabel-kupfer / Stecker 30%` darf retired werden, append-only Historie bleibt erhalten; Nachfolger `kabel-mit-stecker / Stecker 30%`. Entfernte Artikel/Schlüssel ebenfalls quellenspezifisch retired, nie globale/materialweite Löschung.
4. Artikel-ID/URL und sortenspezifisches Änderungsdatum benötigen Transport neben Preis/Variant. `HandlerOutcome.published_at` allein ist ein Seitenwert und darf heterogene Produktdaten nicht überschreiben. Page-last-change ist kein Beleg für Veröffentlichung jeder Preiszeile.
5. Mengen/Staffeln müssen ohne stille Identitätsänderung oder bedingungslose Kleinmengen-Höchstpreise erhalten bleiben. Keine Staffel-/Reconciliationaktivierung ohne gemeinsame Seam und Owner-Freigabe.

Fortsetzung und konkrete Test-/Live-Evidenz werden hier ergänzt. GitHub-Auth/Deployment beim Owner offen.

## Neu abgerufene Primärquellen (10.10.2026)

- Übersicht HTTP 200, 253885 Bytes, SHA256 `dd69560a9f34b226995869ca47eca09d7ffb9d3aa00383d12d337a6ea9c7de1d`. Unverändertes HTML als versionierte Fixture `crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261010.html` gespeichert. Seitenänderung lautet **09.10.2026**, nicht 08.10.; Produktdaten liegen zwischen 26.07. und 09.10.2026, fünf Anfrageprodukte ohne eigenes Datum.
- Impressum HTTP 200, 28536 Bytes, SHA256 `e85bbb6e4065d7fc7960bf43135991d0e2c797cd282e7c1ff639663904dc8733`; ebenfalls unveränderte Fixture. MKM Metals e.K., Mubarik Ahmad, Industriestraße 4, 67063 Ludwigshafen am Rhein, HRA 61664, Telefon 0176 220 168 40, contact@mkm-metals.de.
- Standort Ludwigshafen HTTP 200, 40630 Bytes, SHA256 `8f16b1e10833536ff7a1210b494f7fd3591e40a1722409687d0dc84a5846a847`. Betriebsstätte/Domain/Stammdaten kongruent, keine Übertragung auf Footer-Standorte. Betreiberidentität primärseitig bestätigt; **kein neuer amtlicher Registerauszug**.
- **Alle 60 Produkt-Deep-Links HTTP 200**, jeweilige Artikelnummer im Produkttext bestätigt. Alle 55 Detail-Mengenstaffeln unabhängig aus sichtbarer Detailtabelle extrahiert und mit Überblick-`data-staffel` verglichen: **0 Abweichungen**. Fünf Anfrageprodukte besitzen keine numerische Staffel. Rohabrufe `/tmp/opencode/mkm-detail-<ARTIKEL>.html`, Manifest `/tmp/opencode/mkm-detail-audit-20261010.json`. Die Dokumente sind Betreiberquellen, keine unabhängigen Quellen.

### Fachlich begründete additive Katalogfraktionen

Vorhandene geeignete Materialien werden wiederverwendet (Messing/Rotguss, Aluminiumguss/-blech/-felgen, Alu-Cu-Kühler, Edelstahl, Kabel-Alu, Zinn 99, Lötzinn 60/30, Platinen, Motoren, Hartmetall, HSS, eisenhaltiger Mischschrott).

| Neue Fraktion | Konkrete Betreiber-Produktbelege / Abgrenzung |
|---|---|
| `kupfer-granulat` | MKM-CU-02 `/metallankauf/produkt/kupfergranulat`: getrenntes rieselfähiges Kupfer aus Kabelgranulierung, weder Kabel noch Millberry |
| `cu-ms-kuehler` | SW10017 `/metallankauf/produkt/cu-ms-kuehler-ohne-fe`: Kupfernetz/Messingkästen, keine Aluminium-Kupfer-Verbindung |
| `kabel-alu-cu` | SW10035 `/metallankauf/produkt/alu-kupfer-kabel`: Verbundleitungen mit beiden Leitermetallen |
| `aluminium-spaene` | SW10012 `/metallankauf/produkt/alu-spaene-basis-5`: Dreh-/Fräs-/Bohrspäne, Basis 5 % Feuchte/Öl |
| `aluminium-profile-beschichtet` | MKM-AL-01 `/metallankauf/produkt/alu-profile-farbe`: lackiert/pulverbeschichtet; bestehende Profildefinition verlangt blank |
| `aluminium-isoprofile` | SW10000 `/metallankauf/produkt/alu-iso-profile-ausbau`: thermisch getrennt mit Kunststoffstegen, ohne Glas/Beschläge |
| `nickel` | MKM-NE-04/05 `/metallankauf/produkt/nickel-60-99`, `/nickel-rein`: Legierung bzw. Reinnickel, kein Edelstahl/Hartmetall |
| `wolfram` | MKM-NE-06 `/metallankauf/produkt/wolfram`: metallisches Wolfram ab 99 %, ausdrücklich kein Wolframcarbid |
| `neodym-magnete` | MKM-NE-02 `/metallankauf/produkt/neodym-magnete`: NdFeB-Magnete, nicht Neodymmetall |
| `zinn-legierungen` | SW10039 `/metallankauf/produkt/zinn-90`: Feinlot **und** Zinngegenstände; Reinzinnkatalog schließt Lot/Geschirr aus, Lötzinnkatalog ist Sn-Pb-Lot |
| `blei-gemischt` | SW10030 `/metallankauf/produkt/blei`: umfasst Auswuchtgewichte; bestehendes `blei` schließt diese ausdrücklich aus. Keine Akku-Annahme ableiten |
| `blei-geschoss` | MKM-BL-01 `/metallankauf/produkt/geschossblei`: Kugelfang/Bleischrot, Sand und Feuchte entscheidend; kein Auswuchtblei |
| `metallschrott-gemischt` | MKM-SCH-03 `/metallankauf/produkt/metallschrott`: überwiegender Buntmetallanteil, nicht reine Eisenmischung |
| `schrott-eisen-ne-gemischt` | MKM-SCH-02 `/metallankauf/produkt/mischschrott`: Eisen und NE-Metalle ungetrennt. Bestehendes `mischschrott` bleibt unverändert auf Eisen/Stahl begrenzt |

Für E-Motoren ist `max. 500KG` gemäß Produktdetail **500 kg je Motor**, nicht Höchstmenge der Lieferung. Höchstpreisstaffel separat ab 500 kg. Transportkosten, PayPal, Mindestauszahlung sind keine Ankaufkurse und werden nicht in Metallpreise verrechnet.

## Implementierter Nachweis (nicht aktiviert)

- `mkm_metals/overview.rs`: DOM-Parser ausschließlich tatsächlich captionierter Ankauftabellen. Explizite Artikel-/Titel-/Deep-Link-/Gruppenprüfung für alle 60 Artikel; unbekannte oder geänderte Artikel bleiben im Inventar, aber erzeugen **keinen Preis**, sondern Diagnostik. Bekannte fehlende Artikel ebenfalls explizit, keine automatische Retirementfreigabe. Leere/duplizierte Tabellen oder Artikel, unverständliche Einheit/Menge/Preis, negative/Nullpreise, inkonsistente sichtbare/verborgene Staffelwerte und geänderte Betreiberidentität schlagen fehl.
- Mengenbedingungen und Rohlabel werden **nicht** in einer erfundenen Fixpreisinterpretation verschluckt: Source-Article besitzt Preiszelltext, Mindestmenge für den Höchstwert, komplette Staffel, rohe Datumsspur, maschinenlesbares Produktänderungsdatum und URL. Kandidatenpreis bleibt `upto`, `price_max=price`, `price_min=None`, confidence 0,5; Anfrageartikel sind Annahmen, nie Nullpreise.
- Bestehende elf Material/Variant-Identitäten unverändert. Neue Varianten verwenden eigene Rohsortentitel, damit Rotguss-Stücke/Späne, Messinggehalte und Alufelgenzustände nicht kollabieren. **Keine Staffelpreise aktiviert**, keine Mengen in bestehende Preisidentitäten hineinumbenannt.
- `pipeline.rs`: 14 additive, fachlich belegte Materialien mit Abgrenzungsbeschreibungen; bestehende Katalogdefinitionen nicht überschrieben. Echte Seed-/Record-TempDB bestätigt die Auflösung sämtlicher Artikel auf den tatsächlichen Katalog.
- `examples/mkm_inventory.rs`: read-only Kandidateninventur, Fixture oder `--live`, keine DB, kein Scheduler/Geocoding. `production_ready=false` ist unverhandelbare Ausgabe auch bei lückenfreiem Kandidateninventar.
- Produktänderungsdatum wird **nicht als Preisveröffentlichungsdatum ausgegeben**. Kandidaten-`HandlerOutcome.published_at=None`; getrenntes `page_change_date=2026-10-09T00:00:00+00:00`. Ob „Geändert“ im konkreten System Preis- oder Produktänderung bedeutet, darf eine neue gemeinsame Seam nicht stillschweigend entscheiden; aktuell ehrlich als `product_change_date` bezeichnet.
- `scripts/audit-mkm-overview.py`: zusätzlich Artikel-für-Artikel-Abgleich gegen Kandidaten-JSON und unabhängige Detailseitenprüfung. Kein bloßer Count-Gate, keine semantische Mapping-/Produktionsfreigabe daraus.

## Ausgeführte Tests / Live-Inventur

1. **Vor Fix rot:**
   `CARGO_TARGET_DIR=/tmp/opencode/mkm-full-target-20261010 /root/.cargo/bin/cargo test --locked -p schrott-mcp-ingestion current_overview_all_numeric_articles_reach_real_mapping -- --nocapture`
   Echte aktuelle Fixture → alter Parser/Mapping: `left: 11, right: 55`, Exit 101. Der Test verwendet jetzt den vollständigen Kandidaten, nicht einen Count-Stummel.
2. MKM-Tests nach Fix: **13 bestanden**, einschließlich echter TempDB, Legacy-Identitäten, Anfragepartition, Artikelmutationen bei unverändertem Count, fehlende/duplizierte Artikel, Quellen-/Einheiten-/Mengen-/Preis-/Staffelmutationen, Gebühren-/Verkaufstabellen-Decoys.
3. Gesamte Ingestion-Suite: **563 bestanden, 5 bewusst ignorierte Live-Tests**, keine Fehler; Doc-Tests erfolgreich. Keine vollständige Workspace-Suite behauptet.
4. Python-Audit: **7 Tests bestanden**; Artikel-/Mengen-/Datum-/Staffeländerungen trotz gleicher Anzahl, Phantom-/Duplikatartikel und Detailidentitäts-/Staffelabweichungen werden erkannt.
5. Neu gebauter Kandidat live: `cargo run --locked -q -p schrott-mcp-ingestion --example mkm_inventory -- --live`: **8 / 60 / 55 / 5**, Diagnostik leer, `production_ready=false`. Versionierte Ausgabe `research/mkm-full-live-inventory-20261010.json`.
6. Unabhängiger Audit:
   `python3 scripts/audit-mkm-overview.py crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261010.html --candidate-json research/mkm-full-live-inventory-20261010.json --detail-dir /tmp/opencode`
   Exit 0: **keine Artikelvertragslücken, keine Detailvertragslücken**, 60 Detailidentitäten/Hashes und sichtbare Staffeln. Versionierte Ausgabe `research/mkm-full-contract-audit-20261010.json`. Dies bestätigt den gezeigten Snapshot, nicht zukünftige Quellen oder produktives Record-Verhalten.
7. Registrierter Handler ebenfalls neu gebaut/live gelesen: weiterhin **11**, Diagnose **44 numerische Lücken**, Audit Exit 1. Rohoutput `/tmp/opencode/mkm-full-registered-live-20261010.log`; Auswertung `/tmp/opencode/mkm-full-registered-audit-20261010.json`. Kein grüner Main-/Produktions-Coverageclaim.
8. Nur `sqlite3 -readonly` auf Produktion: weiterhin 12 MKM-Current-Zeilen. Alte `kabel-kupfer / Stecker 30%` 2,29 seit 29.09. steht neben aktueller `kabel-mit-stecker / Stecker 30%` 1,94. Aktuelle Beobachtung übriger Zeilen 10.10.2026 17:30:03 UTC. **Dieser Agent hat keine Produktionspreise geschrieben.**

| Gruppe | Artikel | numerisch | Anfrage |
|---|---:|---:|---:|
| Kupfer | 14 | 14 | 0 |
| Messing/Rotguss | 8 | 8 | 0 |
| Aluminium | 13 | 13 | 0 |
| Edelstahl | 3 | 3 | 0 |
| NE-Metalle | 10 | 8 | 2 |
| Blei/Zink/E-Motoren | 4 | 4 | 0 |
| Hartmetall | 5 | 5 | 0 |
| Schrott | 3 | 0 | 3 |

## Exakte Owner-Seam / noch offene Arbeit

**Der Auftrag ist produktiv noch nicht abgeschlossen.** Der echte TempDB-Test beweist nicht nur 55 katalogaufgelöste `upto`-Preise und 5 Annahmen, sondern auch ausdrücklich, dass unverändertes `record()` alte falsche/entfernte Schlüssel behält und Artikel-ID, Staffel und Produktdatum aus dem Legacy-Outcome verliert. Testname `real_tempdb_record_path_proves_catalog_coverage_and_shared_seam_gaps`.

Benötigte gemeinsame API (Name vom MSG-/Store-Owner festzulegen): zusätzlich zum Legacy-Outcome ein typisierter **quellenspezifischer Artikel-Snapshot**, nicht freie Labelheuristik und nicht 55 als alleiniger Vollständigkeitsindikator:

- Scope `{trader_slug, source_url}` und Mapping-/Snapshotversion; Ergebnisstatus `verified_complete` nur nach expliziter Artikelpartition, Betreiberprüfung und fehlender Diagnostik. Unbekannte/geänderte/weggefallene bekannte Artikel zunächst `blocked`, nicht automatisch Retirement. Entfernte Artikel erst nach verifizierter Owner-/Mappingentscheidung als legitime Entfernung behandeln.
- Artikel-ID → Produkt-URL → Preis-/Annahmeidentität `(material_slug, variant)`, inklusive Rohlabel, Preiszelltext, kompletter Staffel, Höchstpreis-Mindestmenge und Zeilenänderungsdatum. Gemeinsamer Record darf Mengen nicht stillschweigend verlieren oder in stabile Legacy-Variants umbenennen. Sidecar/`extra_json` muss Daten **auch bei unverändertem Preis** aktualisieren können; historische Metadaten dürfen nicht nachträglich überschrieben werden.
- Datumsevidenz ausdrücklich typisieren: `page_change_date`, `product_change_date` und `observed_price_change` sind verschiedene Angaben. Kein pauschales Published-at aus Page-last-change; kein Produktänderungsdatum als Preisdatum ohne Quellenbeleg.
- Atomare Aufzeichnung + source-/trader-begrenztes Retirement von fehlenden bestätigten Artikelpreis-Identitäten. Falscher Legacy-Plugkey explizite Alias-/Retirementmigration zu richtigem Plugkey. Numeric→Anfrage muss alten Current-Preis retireden und Annahme erhalten. Andere Händler/Quellen sowie append-only Preisgeschichte dürfen nicht betroffen sein.
- Teilfehler/Schreibfehler/Mappinggaps/unvollständige Quelle müssen Reconciliation vollständig verhindern. Idempotenz, unveränderte Quellenmetadatenänderungen, wegfallende Artikel, Materialwechsel und Numeric→Anfrage brauchen echte Store-Integrationstests.
- Erst danach registriertes `scrape()` von Legacy auf den vollständigen Kandidaten umstellen. Derzeit absichtlich unveränderte produktive Registrierung; diese Arbeit verändert weder `crates/store` noch `traders/mod.rs`.
- Nach Owner-Integration: Gates erneut auf gemeinsamer Basis, GitHub-Auth herstellen, normale Veröffentlichung/Deployment, regulären Produktionsscrape abwarten und read-only **artikelbewussten** Current-/Historienabgleich durchführen. Keine Forced-Ingestion/GEO durch diesen Agenten, kein Feedbackabschluss vor diesem Nachweis.

Offene Semantik ist somit nicht versteckt: persistierbare Mengen-/Datumsevidenz und historieerhaltendes Retirement fehlen in der gemeinsamen Schnittstelle; Produktdatum als Veröffentlichungsdatum nicht freigegeben. Betreiber-/Snapshot-/Katalogbelege sind vorhanden, aber **keine End-to-End-Vollständigkeitsfreigabe**.

## Abschlusscheck dieses isolierten Arbeitsstands

- `git diff --check` erfolgreich. Main unverändert und sauber bei `6d5ef762`; Diff für reservierte Pfade `crates/store` und `crates/ingestion/src/traders/mod.rs` leer.
- Striktes Clippy-Gate **nicht bestanden**: erster Lauf stoppt im unveränderten Store bei `set_acceptance` (`too_many_arguments`). `--no-deps -- -D warnings` zeigt bestehenden breitflächigen Lintbestand (143 Lib-/154 Test-Fehler, u.a. Legacy-Parser-Typecomplexity und Dokumentation anderer Handler); kein Befundpfad im neuen `overview.rs` oder `mkm_inventory.rs`. Keine fremden Codebereiche für ein kosmetisches grünes Gate verändert. Rohlog `/root/.local/share/opencode/shell/42aa29071d6c1a29b482e0269f73393202254764/sh_1278dac9d0013vSuQghTCWM0y5.out`.
- Übergabediff einschließlich neuer Dateien: `/tmp/opencode/mkm-full-20261010.patch` (nach finaler Validierung erzeugt). Keine Staging-/Commit-/Push-/Deployoperation.
- Weitere Ausführung am gemeinsamen Store-/Aktivierungsbereich liegt beim Owner. Dies ist ein nachweisbarer vollständiger **Quellen-/Mappingkandidat**, keine Behauptung abgeschlossener Produktionsbehebung.
