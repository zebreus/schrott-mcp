# MKM: reviewter 61-Artikel-Vertrag / Implementierungsübergabe

## Ergebnis und verbleibendes Gate

Ownerfreigabe für `MKM-MS-01 → messing / Messing 58 blank neu` umgesetzt **in den MKM-/Verifierquellen**. Notwendige Storeänderungen einschließlich Store-Regressionsquellen separat unter **`/tmp/opencode/mkm-store-followup.patch.txt`**, ausdrücklich **nicht angewandt**. `crates/store/src/public/prices.rs` und `goldtrans.rs` wurden hier nicht bearbeitet; Goldtrans-Migration-7-WIP des Owners bleibt unangetastet.

**Nicht integriert und nicht verifiziert:** Der tatsächlich vorhandene Storeguard verlangt weiterhin 60 Artikel. Der neue End-to-end-Test kann deshalb im aktuellen gemischten Arbeitsstand noch nicht als grün gelten. Owner muss den separaten Patch nach seinem Goldtrans-Workspace-Test integrieren und anschließend budgetiert kompilieren/testen. Kein PASS-, Deployment- oder Produktions-Recoverynachweis.

## Belastbares RED vor dem Fix

Gelesen und SHA-256 selbst überprüft:

- `/root/Documents/checkin-20261018-2230-mkm-red.log`
- SHA-256 `eb23cbf7560c0afc94e9ffa138fdde9e6c909f515fa1d6fed67d35782ff9286a`
- Frischer Ownerlauf, genau ein Test, **0 passed / 1 failed**, vom Owner Exit 101 berichtet.
- `traders::handlers::mkm_metals::recording::tests::reviewed_20261018_capture_records_distinct_ms58_variant`
- Fehler am echten Recordingcall: `Parse { url: "https://mkm-metals.de/metallpreise", detail: "incomplete or invalid MKM source contract" }`, nicht Compiler-/Setup-/Capture-Parsefehler.

Dies ist Owner-Evidenz aus seinem isolierten Source-/Buildlauf, kein von mir ausgeführter Test. Diagnose und Phase-1-Bericht wurden vor Änderungen vollständig im relevanten Bereich gelesen.

## Repositorydiff / Vertragsänderungen

| Pfad | Inhalt |
|---|---|
| `crates/ingestion/src/traders/handlers/mkm_metals/overview.rs` | Exakt ein neues Tuple mit Nummer, Label, Detailpfad, `messing`, expliziter eigener Variante und bisheriger Messinggruppe. Reporttext nennt 61. Produktive Missing-/Unknown-/Identity-/Duplicate-/Staffel-/Sichtbarkeitsprüfungen unverändert. |
| `.../mkm_metals/recording.rs` | Keine produktive Guardlockerung; vorhandene `MAPPINGS.len()`-Vollständigkeit und Artikel-/Preis-/Acceptancevalidierung gelten jetzt für 61 Mappings. Neue Sicherheitsregressionsquellen; bestehende aktuelle Positiv-/All-request-/Einheitenregressionen gezielt an die zusätzliche Identität angepasst. |
| `crates/ingestion/src/traders/handlers/mkm_metals.rs` | Test „current overview“ liest jetzt die echte datierte 61er-Capturefixture, erwartet 56 Preisprojektionen/fünf Anfragen. Historische Copperhelpers unverändert. |
| `.../fixtures/mkm-reviewed-mapping-oracle-20261018.json` | Additiver unabhängiger Reviewstand: alle alten 60 Material-/Variantenpaare plus genau MS-01; Reviewgrund und Vorgängerdatei benannt. Nicht aus dem Produktionsmapping generiert. |
| `scripts/verify-mkm-release.py` | Aktiver neuer Oracle, exakte 61er-Identitätsmenge, erfolgreicher 56-Preis-Step, 56/5-Partition. Keine „mindestens“-Zählung, kein frei wählbarer Grün-Schalter; Scope/Freshness/Owner/Current/Acceptance/Normalisierung/Publikationssemantik/Read-only-Verbindungen unverändert. |
| `scripts/audit-mkm-overview.py` | Neuer aktiver Oracle; `data-staffel` unterstützt beide HTML-Attributquotearten, damit auch die sanitizierte Capturefixture korrekt auditierbar ist. JSON-Inhalt wird nicht lockerer validiert. |
| `scripts/test_verify_mkm_release.py` | Historische Inventardatei unverändert; explizit reviewter neuer Artikel nur in der synthetischen Testzustandsprojektion ergänzt, Step 56 und 61/56/5/56 erwartet. Neue Negativ- und Oracle-Subsettests. Bookkeepingzeit auf 18.10. nach dem Captureproduktdatum gesetzt. |
| `scripts/test_audit_mkm_overview.py` | Historische 60er-Audits übergeben explizit den alten Oracle (auch 120 gezielte alte Mappingfehler bleiben 120). Separater Test für neue Captureidentitäten, 56 numerische Angebote und MS58-Staffeln mit einfachen Attributquotes. |

Neuer Preis bleibt `6.51 EUR/kg`, `upto`, `price_max=6.51`, kein `price_min`, Konfidenz 0.5; Staffeln `(0,6.20),(5,6.32),(500,6.51)` und 500-kg-Schwelle bleiben unverändert im Quell-/Preis-/Acceptancevertrag. Keine erfundene Veröffentlichung oder Gültigkeit. Keine neue Materialkatalogzeile, keine Umbenennung alter Messingvarianten und keine globale Aliasmigration. Owner bleibt `mkm-article-v1`; die zehn Proven-Legacy-Kupferidentitäten werden nicht erweitert.

Die reguläre produktive Partition wird weiterhin aus gültigen Artikeln ermittelt, nicht hart auf 56/5 begrenzt. Der externe Release-Verifier prüft bewusst den freigegebenen beobachteten 56/5-Stand; ein legitimer weiterer Anfragewechsel braucht dort erneuten Review, keinen stillen Grünentscheid.

## Historische Evidenz erhalten, aktuelle Regressionen vollständig

SHA-256 nach Änderung erneut kontrolliert, unverändert gegenüber Phase 1:

- `crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261010.html`: `dd69560a9f34b226995869ca47eca09d7ffb9d3aa00383d12d337a6ea9c7de1d`
- `crates/ingestion/src/traders/handlers/fixtures/mkm-reviewed-mapping-oracle-20261011.json`: `bf44a6fefef1dd9a7c30d9bd0e98358356d156b69370756a7b5ce8703c3507e5`

Die beiden neuen sanitizierten Capturefixtures aus Phase 1 bleiben unverändert. Keine historische Forschungsinventardatei umgeschrieben.

Ein expliziter neuer Parsertest liest das unveränderte alte HTML **ohne** Ergänzungshelper: weiterhin 60/55/5 und alle alten Mappings, jetzt exakt eine Missing-Diagnose für MS-01. Das alte HTML wird nicht durch einen produktiven 60/61-Ausnahmemodus akzeptiert. Der historische private Legacy-Writer-Beleg verwendet weiterhin direkt diesen 60er-Snapshot und seine alten 55er-Assertions.

Andere bestehende Positiv-/Mutationsregressionen benötigen einen vollständigen Vertrag, sollen aber ihre alten Datums-/HTMLmutationen weiter treffen. Dafür gibt es `#[cfg(test)] augment_legacy_test_html`: übernimmt genau die neue MS-01-Zeile aus dem reviewten Capture und fügt sie der alten Messingtabelle hinzu, ohne alte Fixturebytes zu ändern. Nur dessen Testmarkupattribute werden an bestehende Row-Mutationshelper angepasst. Die Kombination ist **synthetisch**, kein Livecapture. Echte 61er-Parser-/Recordingtests rufen den Parser direkt auf den datierten Capturefixtures auf und umgehen diesen Helper. Invalid-Source-Regressionsbasen werden ebenfalls vervollständigt, damit sie nicht bloß am fehlenden neuen Artikel statt an ihrer eigentlichen Mutation scheitern.

Gezielt geänderte Zählungen: aktuelle vollständige Eingabe 61/56/5; Kupfer→Anfrage 55/6; zwei bisherige Stahl-Anfragen numerisch 58/3; vollständiges All-request 0/61. Historische 12 Altpreise/fünf Altacceptances, 60%-Materialnamen, Konfidenz 0.5 und Quellstaffeln sind keine neuen globalen Ersetzungsziele.

## Neue/erweiterte Regres­sionsquellen, sämtlich ungetestet in dieser Phase

Rust-Ingestion, unter `traders::handlers::mkm_metals::`:

1. `recording::tests::reviewed_20261018_capture_records_distinct_ms58_variant` — unveränderter ursprünglicher Red-Test als Erfolgsgate; echter Capture→Parse→`record_outcome`→Store, 61er-Ledger, 56 Current, eigene Messingvariante, sechs Messing-Acceptancemitglieder, alle alten 60 Paare und Quell-/Staffel-/Publikationssemantik.
2. `overview::tests::historical_60_article_capture_is_preserved_but_no_longer_complete` — alte echte Quelle/Oracle erhalten und als aktuell unvollständig ausgewiesen; alter Oracle exakte Teilmenge des neuen.
3. `overview::tests::reviewed_61_article_capture_matches_additive_independent_oracle` — echte neue Capturequelle, exakte neue Menge und alle 61 unabhängigen Mappingpaare.
4. `recording::tests::reviewed_capture_unknown_62_and_same_count_replacement_cannot_mutate_evidence` — echte aktuelle Quelle zunächst erfolgreich aufnehmen, danach unbekannten 62. Artikel und gleichzähligen Identitätstausch ablehnen; zusätzlich alte 60er-Quelle, geändertes MS58-Label/Detailpfad, falsche obere Preis-/Staffelprojektion und doppelte MS58-Zeile. Bei Recordingablehnung Vergleich **aller gespeicherten Felder** in `prices`, `current_prices`, `trader_materials`, `source_article_snapshots`, `article_acceptance_owners`.
5. `recording::tests::reviewed_ms58_request_transition_preserves_immutable_history_and_other_brass_keys` — Wiederholung an gleicher Zeit ohne neue Preis-/Ledgerzeilen; MS58 numerisch→Anfrage (55/6)→numerisch (56/5), nur sein Current fällt weg, History und alte Ledgerfelder bleiben erhalten, gebrauchte 58-Abfälle behalten ihren eigenen Currentschlüssel.

Bestehende Alias-/Foreign-/Unit-/All-request-/Atomicitytests bleiben erhalten und werden auf vollständigen Testeingaben geführt. Das ist **Quellcodeabdeckung**, kein verifizierter Testlauf.

Python, genaue neue Namen:

- `InventoryTests.test_reviewed_61_capture_preserves_ms58_identity_and_single_quoted_tiers`
- `MkmReleaseTests.test_old_oracle_is_an_exact_subset_of_new_review`
- `MkmReleaseTests.test_historical_60_and_unknown_62_or_same_count_article_are_red`
- `MkmReleaseTests.test_ms58_missing_quote_reused_variant_or_missing_acceptance_member_are_red`

Bestehende Verifier-Negativmutationen und bytegleiche Read-only-Prüfung bleiben erhalten. Die Pythonzustände sind synthetische Buchhaltung mit historischer Artikelprojektion plus explizitem neuen Artikel, **kein** echter isolierter Store-/Live-Schedulerlauf.

## Separater Store-Anwenderpatch: Ownerintegration notwendig

Datei: `/tmp/opencode/mkm-store-followup.patch.txt`.

SHA-256 bei Übergabe: `7bb352b91ad4e260374d4ef30492c6755649b9c336ecdbce89f7dcd10767affc`.

Format: **OpenCode/`apply_patch`-Envelope** (`*** Begin Patch`, absoluter Zielpfad), **kein** Unified-Git-Diff; Owner liest/anwendet den Text nach Ende seines konkurrierenden `prices.rs`-Workflows. Nicht auf die Produktions-DB anwenden. Goldtrans-/Migrationscode ist in diesem Patch nicht enthalten.

Patchinhalt:

- Lokaler `MKM_ARTICLE_COUNT=61` im Snapshotwriter; ausschließlich den MKM-Manifestkardinalitätsguard daran binden. Keine Zulassung alter 60er-Neuschreibungen, keine Datenmigration, keine Löschung historischer Ledger und keine Änderung der übrigen Scope-/Zeit-/Owner-/Konflikt-/Transaktionsprüfungen.
- Bestehenden `article_transaction_rolls_back_after_acceptance_or_manifest_failure` mit zulässigem 61er-Testmanifest betreiben und **explizit** auf `test rollback` als Fehler prüfen, damit er nachweislich den injizierten Acceptance-/Manifest-Trigger erreicht statt vorher an einer falschen Anzahl zu scheitern.
- Neuer `reviewed_mkm_61_contract_preserves_prior_60_ledger_and_rejects_partial_retirement`: strukturellen historischen 60er-Ledger nur in isolierter Test-DB direkt seeden; 60-/62-Neuschreibungen müssen alle fünf Evidenztabellen unverändert lassen; zulässiger 61er-Storecommit hängt neuen Ledger an, bewahrt den alten Manifeststring und alle alten Preisfelder, retired nur autorisierte Currentpointer, keine History. Synthetische Store-Artikel-IDs sind ausdrücklich kein Ersatz für die echten Ingestion-/Capture-/Oracle-Tests.

Der separate Patch wurde nur als Text geprüft, **nicht** probeweise am laufend bearbeiteten Ownerfile angewandt. Bei zwischenzeitlicher Format-/Kontextänderung muss der Owner die eng benannten MKM-Hunks gegen seinen aktuellen Stand übernehmen. Vollständige Integration ist Voraussetzung für den ersten Greenversuch.

## Prüfstatus und nächste sichere Handlung

Hier durchgeführt: Berichte/RED-Log/Code/Diffs gelesen, Log- und alte Evidenzhashes kontrolliert; `git diff --check` auf den eigenen MKM-/Skriptänderungen ohne Whitespacebefund. Das ist keine Rust-/Python-Ausführungsprüfung.

Hier **nicht** ausgeführt: Build, `cargo check`, Rust-/Python-Tests, Verifier-/Parserbinaries, Produktionsqueries/-writes, Commit, Push, Deployment, GEO oder Dossierbearbeitung. Owner-Goldtrans- und andere fremde Änderungen weder zurückgesetzt noch angepasst.

Ownerfolge nach Ressourcenfreigabe: (1) Storepatch im eigenen fertigen WIP integrieren, (2) ersten echten Capture-End-to-end-Test und neue Unknown-/Historytests frisch ausführen, (3) vollständige MKM-/betroffene Storetests einschließlich Trigger-Nachweis sowie Python-MKM-Tests ausführen, (4) Compiler-/Testbefunde beheben oder ausdrücklich als offen melden. Solange Patchintegration und frische Ausführung fehlen, lautet der Status **Implementierungsdiff vorbereitet, Storeintegration und Green-Verifikation offen**, nicht „fertig deploybar“.

## Nachtrag: frischer Python-Fehler und eng begrenzte Oraclekorrektur

Nach dem Ownerbefund ausdrücklich nur leichte Python-Tests freigegeben. Den Fehler vor jeder Änderung selbst identisch reproduziert:

```sh
python3 -B -m unittest discover -s scripts -p 'test_*mkm*.py' -v
```

Erster tatsächlicher Lauf: **21 Tests in 1.083 s, drei FAIL / ein ERROR, Exit 1**. `test_old_oracle_is_an_exact_subset_of_new_review` warf `KeyError: 'SW10000'`; der neue Capture-Audit meldete dieselbe fehlende Identität; beide Verifier-Positivtests scheiterten mit Mappingabweichung, 55/5 statt 56/5 und unerwartetem Current.

Ursache ist mein Übertragungsfehler beim Anlegen des additiven Oracles: Der neue Stand hatte vor der Korrektur nur **60** Einträge, weil zwar MS-01 hinzugefügt, aber der alte Artikel **SW10000** ausgelassen war. Direkter JSON-Vergleich: fehlende Altidentitäten genau `[SW10000]`, neue Identitäten genau `[MKM-MS-01]`, **keine** sonstigen geänderten Altpaare. Das ist weder eine neue Betreiberänderung noch ein Fehler der Guards.

Identitätsabgleich vor dem Fix:

- Alter unabhängiger Oracle: `SW10000 → ["aluminium-isoprofile", "Alu Iso Profile Ausbau"]`.
- Unverändertes Produktionsparsermapping: exakt dieselbe Nummer, dasselbe Label und Material, Pfad `/metallankauf/produkt/alu-iso-profile-ausbau`, Gruppe Aluminium; Variante ist das Label.
- Echter neuer Capture: dieselbe Identität mit 1,89 EUR/kg ab 500 kg. Keine Verwechslung mit `SW10001` (Alu Profile blank/max. 10%eloxiert) oder `MKM-AL-01` (Alu Profile mit Farbe).

**Einziger Sourcefix dieses Folgeauftrags:** in `crates/ingestion/src/traders/handlers/fixtures/mkm-reviewed-mapping-oracle-20261018.json` genau die fehlende Zeile ergänzt:

```json
"SW10000": ["aluminium-isoprofile", "Alu Iso Profile Ausbau"]
```

Keine Assertion, Erwartungszahl oder Guard abgeschwächt; keine sonstige Rust-/Pythonquelle geändert. Danach denselben Befehl frisch erneut ausgeführt: **21 Tests in 1.160 s, OK, Exit 0**. Damit sind die Python-MKM-Tests einschließlich exakter Oracle-Teilmenge, 61er-Capturemenge, Verifier-Positivfixture/Einheitennormalisierung sowie vorhandener Negativkontrollen tatsächlich grün. Neuer Oracle-SHA-256: `2eebe2b44bdb9d33c2e936903ecd67175c4cf90841e478525762c0b3f54678d8`.

Alter Oracle und altes HTML erneut per SHA-256 kontrolliert, unverändert: `bf44a6fefef1dd9a7c30d9bd0e98358356d156b69370756a7b5ce8703c3507e5` bzw. `dd69560a9f34b226995869ca47eca09d7ffb9d3aa00383d12d337a6ea9c7de1d`. Der separate Storepatch ist ebenfalls unverändert (`7bb352b91ad4e260374d4ef30492c6755649b9c336ecdbce89f7dcd10767affc`) und wurde weiterhin **nicht angewandt**. Goldtrans-/Owner-WIP unberührt.

Diese frische Evidenz gilt **nur für die Python-MKM-Tests dieses Arbeitsstands**. Keine Rustbuilds/-tests, Produktionszugriffe, Commits oder Deployments ausgeführt; kein Gesamt-PASS und keine Übertragung des Ergebnisses auf den unabhängigen eingefrorenen Rustexport des Owners.
