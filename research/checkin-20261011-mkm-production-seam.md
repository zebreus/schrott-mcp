# MKM typed Article-/Record-Snapshot — laufender Stand

**Neuester F6-Stand:** katalog-normalisierte Storeprojektion für alle MKM-Quotes mit unverändertem Sourcevertrag; Owner-RED-Repro nun GREEN, HTML→Parser→typedRecord→TempDB für beide EUR/t-Stahlartikel und unsupported-unit-Errorinvarianz belegt. Workspace660/5ignored, MKM36, Store40, MSG13, Python9, fmt und frischer registrierter Read-only-Audit8/60/55/5 ohne Gaps. Einzelheiten `research/checkin-20261011-mkm-f6-catalog-units.md`. F1–F5 sind unabhängig scope-freigegeben (143 Fälle, RE-REVIEW.md); **F6 unabhängiges Review und Owner-Integration bleiben offen, keine Selbstfreigabe**. Ältere Zwischenstände unten bleiben als Verlauf erhalten.

**Aktueller Remediation-Endstand:** alle fünf adversarialen Findings mit eigenen RED→GREEN-Repros behoben; Details einzeln in `research/checkin-20261011-mkm-review-remediation.md`. Final Workspace **657 passed / 5 ignored**, Store40, MKM33, MSG13, Python9, fmt und frischer registrierter Read-only-Livepfad 8/60/55/5 samt unabhängiger Article-/Mappingprüfung ohne Gaps. F2 zusätzlich mit atomarer öffentlicher Single-write-Transaktion und echtem Trigger-Rollbackrepro gehärtet. **Erneutes unabhängiges Fullreview plus Owner-Fullreview weiterhin ausstehend; keinerlei Selbst-/Produktionsfreigabe.** Die nachfolgenden älteren End- und Zwischenstände bleiben als Geschichte erhalten.

## Adversarial-Fullreview: Freigabe widerrufen, fünf REDs selbst bestätigt

Der neue Auftrag bezieht sich auf `/tmp/opencode/mkm-adversarial-review-20261011/REVIEW.md`. Alle fünf Repros am unveränderten Kandidaten selbst ausgeführt (Harness, eigener Target `/root/mkm-full-target-20261010`, Log `/tmp/opencode/mkm-five-findings-red.log`): F1 erster Requestimport lässt Millberry und beim realen 12-Zeilen-Cutover 10 kanonische Quotes stehen; F2 unmarked öffentlicher Legacywrite überschreibt dieselbe immutable Historyrow; F3 hidden Labelnachfahr, F4 `99.99.2026` und F5 `only_customer` werden fälschlich aufgenommen. Die vorherigen grünen Tests deckten diese Fälle nicht ab. **Keine Freigabe**, alle fünf Findings offen bis neue Regressionen tatsächlich grün sind. Main710233c6 bleibt unberührt; keine Delegation/Prodwrites/Push/Deploy/GEO.

### Persistierter Fix-Zwischenstand (noch keine erneute unabhängige Freigabe)

Eigene hinzugefügte Regressionssuiten vor Implementation tatsächlich RED: `/tmp/opencode/mkm-review-{ingestion,store}-red.log`. Nach gezielten Änderungen tatsächlich GREEN: `/tmp/opencode/mkm-review-{ingestion,store}-green.log` (4 Ingestion-/1 Storetest).

- F1: exakt zehn unabhängig belegte kanonische Legacyartikelidentitäten zusätzlich zu beiden Aliasen für **Requestretirement beim ersten typed Import** freigegeben, nur exakter Material/Variant/Label-Kaufvertrag und richtige Source/Trader; Store schützt fremde Producer/neuere Evidenz. Vollständige reale 12-Current-/5-Acceptancefixture direkt nach 0/60, ohne vorherigen typed Numericimport, lässt keinen eigenen belegten Quote zurück. Fremde Quellen/Producer und unbekannte Labels bleiben unverändert, vollständige historische Rows erhalten.
- F2: bestehender Current-Row-MKM-Owner wird zentral in `record_price_on()` geprüft, unabhängig vom Incomingmarker; unmarked/fremde Publicwrites werden abgelehnt, ebenso andere Source/Source-type/Published-Scope. Echte Parser→typedRecord→TempDB plus öffentliche `record_price()`-/Snapshot-Schreibversuche bewahren sämtliche History-/Currentfelder. Legacy/MSG bleiben außerhalb dieser MKM-only-Schutznorm.
- F3: explizit versteckte Label-/Artikel-ID-Nachfahren schließen die Row aus; fehlende bekannte Artikel erzeugen Diagnostik und blockieren den echten Record. `hidden`, inline `display:none`, nested hidden ancestor und !important geprüft; zugängliche visually-hidden Caption unverändert gültig.
- F4: vorhandenes Seitenänderungsdatum streng fallibel geparst, ungültige/mehrdeutige vorhandene Daten Fehler; tatsächlich fehlendes Datum bleibt ehrlich NULL und legitim. 99.99.2026/unknown/zweite Anzeige getestet.
- F5: jedes Tierobjekt hat strenges Serde-Schema (`deny_unknown_fields`), unbekannte direkte/nested Conditionfelder Fehler, nicht still wegdeserialisiert. Validiertes typed Tier bleibt exakt `{ab,preis}`.
- F3–F5 realer Errorpfad vergleicht komplette Preise, Currentpointer, Acceptances und beide Ledger vor/nach dem Versuch und beweist keine Mutation. Vollständiger Workspace-/MSG-/Python-/fmt-/Live-Folgelauf noch ausstehend zu diesem Zwischenstand.

Expliziter Folgeauftrag: geprüften MKM-Kandidaten auf aktuellem Main mit MSG-`record_price_snapshot` synchronisieren und die fehlende typed Artikel-/Metadaten-/Alias-Seam implementieren. Store und `traders/mod.rs` sind hierfür nun erlaubt, bestehender MSG-Vertrag bleibt verbindlich. Keine Mainänderung, kein Push/Deploy/Produktionswrite/Forced-Ingest/Rust-GEO. Owner-Full-Review vor Cherry-pick erforderlich.

## Persistierter Start

Worktree `/tmp/opencode/schrott-mcp-mkm-full-20261010`, vorhandener Kandidat auf `6d5ef762`, Main `e3fcbbae90bfb9afe45cad8d2f67b055814c55e1`. Vorhandene uncommittete Arbeit vollständig vorhanden; Baseline-Commit im eigenen Branch ausdrücklich autorisiert, nicht gepusht. `/tmp` 1,6 GiB frei; eigenes Target bleibt, keine fremden Cachelöschungen.

## Vereinbarte Test-Seams und Freigabesperren

- Gespeicherte echte Fixture → Artikelparser → typed Outcome → echter `record()` → echte isolierte TempDB: unabhängige 60-Artikel-/Material-/Variant-Oracle, 55 Höchstpreise/5 Anfrageartikel, keine Count-only-Freigabe.
- Öffentliche Store-`record_price_snapshot` als Basis für atomare quote-/retirement-Operation mit trader/source/producer-Schutz, immutable History, vollständigem validiertem Artikelvertrag, verifizierten Aliasen und Request-Transitions.
- Unbekannte/fehlende/geänderte Artikel, Parse-/Preis-/Einheiten-/Betreiberfehler und unvollständige Snapshots autorisieren kein Retirement.
- Mindestmengen, vollständige Staffeln, Artikel-ID/Produkt-URL, `product_change_date` und `page_change_date` getrennt persistieren; keine Published-at-Erfindung. Semantic-Metadataänderung bei gleichem Preis ist neue immutable Beobachtung.
- Kerze-/Steckerkabel-Altschlüssel quellengeschützt retireden, fremde Quellen/Händler/Producer/Historie unverändert; Numeric→Request ausdrücklich verifiziert, atomar mit Annahmen und fehlschlagenden Requests.
- Late-/Duplicate-/Conflict-/Rollback-/Idempotenztests und unveränderte MSG-Tests erforderlich. Handlerregistrierung bleibt bis nach vollständiger Seam-TempDB-Evidenz Legacy.

Noch keine Implementierungs-/Produktionsfreigabe oder neue Testerfolge behauptet. Fortschritt und konkrete rote/grüne Evidenz werden hier fortgeschrieben.

## Persistierter Implementierungsstand

- Kandidaten-Baseline `209854b7` im eigenen Branch erstellt, Main `e3fcbbae` konfliktfrei hineingemergt. Keine Mainänderung/Push. Unveränderte rohe HTML-Fixtures besitzen Quell-Whitespace; Baseline-Diffcheck meldete deren trailing whitespace. Bytes/Hashes bewusst bewahrt, Code-/Dokumentdiff ohne die zwei Raw-Fixtures sauber geprüft. Kein pauschales Whitespace-Grün über rohe Quellen behauptet.
- Tatsächlich rote Store-Schleife: `cargo test -p schrott-mcp-store article_semantic_metadata_change`: gleiche 11,13, Menge 250→500 ergibt **1 statt 2** Historybeobachtungen. MKM-Marker-spezifische immutable Dedupesemantik nun grün; MSG-/Legacy-Dedupesemantik unverändert.
- Tatsächlich roter echter Record-Roundtrip: `typed_article_outcome_roundtrips`: nach Fixture→Outcome→Record fehlt Artikel-ID (`Null` statt `MET-KU-MILL`). Typed `RecordOutcome::MkmArticles(Snapshot)` und vollständige Revalidierung am Schreibboundary nun grün; Menge, alle 4 Staffeln, Produktdatum 28.09., Seitendatum 09.10. persistieren getrennt, Published-at bleibt NULL.
- Kein neues optionales Feld in 130 fremde Handlerkonstruktoren eingebaut. Stattdessen expliziter typed Pipeline-Outcome und MKM-Record-Bridge; spätere Schedulerintegration muss diesen Pfad verwenden, untypisierter Legacy-MKM darf dann nicht mehr schreiben.
- Store erweitert die bestehende atomare Snapshottransaktion um MKM-eigenen immutable Artikelmanifest-/Acceptance-Commit und scope-/producer-begrenzte Acceptanceownership. Vollständige Parser-/Material-/Variant-/Preis-/Mengen-/Staffel-/Datum-/Anfragepartition wird nochmals am typed Record-Boundary geprüft, nicht über `complete=true` behauptet.
- Noch offen: unabhängiger Produktions-Readonly-Aliasfixture-Replay, Request-Transitions, Konditionsänderung ohne Betrag, Late/Duplicate/Conflict/SQL-Rollbacktests, Fremdquellen-/Producer-/Händlerschutz, alle MSG-Tests und Gates, Scheduler/Registrierung erst danach. Gegenwärtig **keine Aktivierung** und **kein Abschlussclaim**.
# Persistierter Zwischenstand: zusätzliche Write-Gates und Transaktionsevidenz

- `cargo test -p schrott-mcp-ingestion mkm_metals`: **24 passed**, einschließlich echtem Record→TempDB, unabhängiger 60-Artikel-Mappingtabelle und Replay der **12 separat read-only aus Prod erfassten Current-Zeilen**. Kein Schreiben nach Prod.
- Alias-Replay entfernt nur die belegten falschen Current-Schlüssel (Steckerkabel und Kerze), erhält sämtliche erfassten historischen Felder und schreibt Kerze zu `kupfer-stanzabfaelle`, nicht Berry.
- Mengenänderung 250→500 kg bei gleichem Betrag erzeugt neue immutable Preisobservation; gleicher semantischer Vertrag bleibt idempotent. Numeric→request (54/6) entfernt den eigenen Current-Quote, ohne History zu löschen; spätere Wiederholung, late snapshot und timestamp conflict explizit getestet.
- Fremdquellen-/Fremdproducer-Aliasse bleiben bestehen; Fremdquellenkollision am eigentlichen Artikelkey blockiert den ganzen Snapshot ohne Aliasretirement.
- Echter RED-Repro: die alte `record_price_snapshot`-API akzeptierte den MKM-Producer ohne Manifest. Jetzt explizit abgelehnt, MSG-API unverändert. Echter RED-Repro: plain MKM-HandlerOutcome wurde lossy geschrieben; jetzt öffentliches `record()` fail-closed, nur typed `record_outcome()` erlaubt. Ursprünglicher Gap-Repro bleibt als historischer Test gegen private Legacy-Implementation erhalten.
- `cargo test -p schrott-mcp-store`: **38 passed**. SQL-Trigger erzwingen Fehler erst bei Acceptance bzw. Manifestinsert; Quotes, Aliasretirement, Acceptances und beide Ledger rollen zusammen zurück.
- Noch **keine Registrierung umgestellt**, noch kein vollständiger Workspace-/Live-Audit für diesen neuen Seam-Stand. Fremdhändler-/all-request-/Datumsänderungsfälle und typed Schedulertransport benötigen weitere Evidenz; Owner-Full-Review bleibt Pflicht vor Cherry-pick.

## Aktueller Kandidatenstand nach vollständigem Lauf

Die vorstehenden Zwischenstände bleiben historisch erhalten. **Im isolierten Kandidaten** verwendet der Scheduler jetzt `scrape_record_outcome()` → `RecordOutcome::MkmArticles` → `record_outcome()` → MKM-Validator → atomare Storetransaktion. Die registrierte Read-only-Projektion verwendet denselben vollständigen Parser; die alte Kupfer-only-/Berry-Kerze-Implementation ist ausschließlich `#[cfg(test)]` historische Regressionsevidenz. Öffentliches plain MKM-`record()` bleibt abgelehnt. Main/Prod verwenden diesen Kandidaten nicht.

### Verifizierte Checkliste

- [x] Main `e3fcbbae` sicher in eigenen Branch integriert, MKM-Baseline `209854b7` nur lokal; keine Folgeimplementation committed/gepusht.
- [x] Unabhängiges 60-Artikel→Material/Variant-Oracle gegen tatsächliche TempDB-Current-Keys plus separater Live-HTML-Audit; nicht bloß 55 Quotes gezählt.
- [x] Reale separat read-only erfasste **12 Current- und 5 Acceptance-Prodzeilen** in neue TempDB replayed. Datenbank-IDs werden über Slugs neu aufgelöst. Belegte Alias-Current-Pointer entfernt, sämtliche alten Preisrow-Felder byte-/wertgleich bewahrt.
- [x] Steckerkabel → `kabel-mit-stecker`; Kerze → `kupfer-stanzabfaelle`. Foreign source, producer und identischer Legacykey unter anderem Händler bleiben unverändert. Foreign target-key collision rollt den ganzen Snapshot zurück.
- [x] 55/5 baseline; 54/6 Numeric→Request; **0/60 all-request** ohne Phantomprices, trotzdem kompletter Artikelmanifest und geschützte Retirementtransaktion.
- [x] Gleicher Betrag bei 250→500 kg appendet neue Beobachtung. Semantic duplicate bleibt Preisrow-idempotent. Produktdatum und Seitendatumänderung appendet Evidenz, alte History unverändert; keine Published-at-Erfindung.
- [x] Mindestmenge, vollständige Staffeln, Produkt-ID/-URL, Rohlabel/normalisierter Preiszellentext und Produktdatum im Preisextra; komplette inklusive raw relativer Produktdatumsanzeige im immutable Snapshotledger. Anfrageartikel bleiben ohne numerischen Preis; Evidenz auch im Manifest und artikelbewussten Acceptance-JSON.
- [x] Seitendatum ausdrücklich **calendar_day**, UTC-Mitternacht nur normalisierte Repräsentation, nicht tatsächlicher Quellzeitpunkt. Produktdatum getrennt, unbelegte Request-Produktdaten bleiben NULL.
- [x] Late/duplicate/same-timestamp conflicting snapshots fail-safe. SQL-Triggerfehler bei Acceptance und Manifestinsert rollen Quoteinsert, Currentretirement, Acceptances und Ownershipledger gemeinsam zurück.
- [x] Neuer tatsächlich roter Repro für neuere/unabhängig überschriebene Acceptance: davor akzeptiert, jetzt abgelehnt. Ownership speichert zuletzt verifizierte Conditions/Observed-at; fremde Änderungen und Validityverträge werden nicht überschrieben. Alte positive leere, nicht neuere Acceptances dürfen adopted werden; negative/abweichend konditionierte/validitätsbeschränkte unowned Acceptances blockieren.
- [x] Response-final-URL exakt Operatorübersicht/Impressum, HTTP exakt200; Redirect zu anderer Identität oder status201/204/301/404/500 abgelehnt, ohne globale Legacyfetch-Änderung.
- [x] Workspace **651 passed / 0 failed / 5 ignored**; Ingestion **586 passed / 5 ignored**, Store **38 passed**; MSG separat **13 passed**, dessen 46-Artikelvertrag und bestehende Store-MSG-Dedupesemantik unverändert. `cargo fmt --all -- --check` und Python-Auditunittests **9 passed**.
- [x] Read-only registrierter Livepfad: **8/60/55/5**, keine Diagnostik; independent `candidate_article_contract_gaps=[]`. Quellen mit HTTP-Headers/Hashes erneut erfasst, siehe `mkm-typed-seam-evidence-20261011.json`.

### Designgrenzen und noch offene Freigaben

**Kein Produktionsclaim:** `production_ready=false` / `production_approval=false` in beiden Inventur-/Auditreports bleiben absichtlich bestehen. Owner muss den gesamten Kandidaten einschließlich Store-/Schedulerdiff vor Cherry-pick reviewen. Auth/Push/Deploy/Prodactivation bleiben außerhalb dieses Worktrees und wurden nicht versucht. Ein neuer unabhängiger Review durch einen weiteren Subagenten ist auf dieser Sessionebene nicht möglich (bekanntes Subagent-Depthlimit); Owner-Review ist nicht durch Selbstprüfung ersetzt.

Der **MKM-Recordvalidator** prüft exakte 60 bekannte Artikel-/Mapping-/Label-/Link-/Gruppenidentitäten und sämtliche Quotes/Requests/Mengen/Staffeln. **Store** prüft Transaktionsscope, Producer, Trader, Vollständigkeitsstruktur/unique IDs, Chronologie und Ownership; er ist ausdrücklich kein zweiter Fachmapping-Parser. MKM-Marker durch generische Snapshot-API wird abgelehnt. Nichtbelegte fremde oder unmarked unbekannte Legacykeys werden nicht auf bloßen Quellcount retired. Keine anderen Traderkontakte/Geodaten werden durch diesen Seam neu geschrieben.

Neue Tabellen `source_article_snapshots` und `article_acceptance_owners` sind additive DDL, beim Öffnen bestehender DBs angelegt; bislang nicht produktiv existierende MKM-Kandidatentabellen haben daher keine vorgängige Spaltenmigration. Owner sollte diese Annahme beim Review gegen seinen Integrationsstand prüfen.

Eigener Target musste beim Workspace-Serverlink von `/tmp/opencode/mkm-full-target-20261010` nach **`/root/mkm-full-target-20261010`** verschoben werden: `/tmp` war voll. Ausschließlich eigener Target bewegt, keinerlei fremde Artefakte gelöscht. Initialer Library-Build deckte außerdem einen zu eng `cfg(test)`-geschützten `ElementRef`-Import auf; behoben, danach kompletter Workspace grün. Keine verschwiegenen fehlgeschlagenen Läufe.

Artefakte: `research/mkm-typed-registered-live-inventory-20261011.json`, `research/mkm-typed-contract-audit-20261011.json`, `research/mkm-typed-seam-evidence-20261011.json`; unabhängige Prod-Fixtures in `crates/ingestion/src/traders/handlers/fixtures/mkm-legacy-{current,acceptances}-readonly-20261011.json`. Vollständige Workspace-/MSG-Logs `/tmp/opencode/mkm-workspace-seam-20261011.log` und `/tmp/opencode/mkm-msg-regression-20261011.log`.

### Übergabediff / aktueller Mainstand

Vollständiger binärer Git-Patch gegen die vereinbarte MSG-Basis `e3fcbbae`: **`/tmp/opencode/mkm-production-seam-20261011.patch`**, einschließlich staged neuer Dateien, Oracle, Fixtures und historischer Reviewevidenz. `git diff --check e3fcbbae` sauber **mit explizitem Ausschluss der beiden bytegetreu rohen ursprünglichen HTML-Fixtures**, nicht pauschal über deren Quellwhitespace. Kein Implementation-Commit erstellt.

Während dieses Laufs hat der Owner Main weiter auf **`ceb312777c30cccb3c4070c681c481b217693fdc`** gebracht; aktueller Main ist sauber. Das Delta seit `e3fcbbae` enthält ausschließlich den Pending-rollout-Verifier, dessen Tests und dessen Bericht (3 Dateien), keine Überschneidung mit MKM. Nicht von diesem Agenten geschrieben und nicht überschrieben. `git apply --check /tmp/opencode/mkm-production-seam-20261011.patch` auf diesem aktuellen Main ist erfolgreich (reine Prüfung, kein Apply/Write). Kandidatenworktree bleibt auf synchronisierter MSG-Basis plus eigener uncommitteter Implementation; kein versteckter Rebase/Stash/Reset.
