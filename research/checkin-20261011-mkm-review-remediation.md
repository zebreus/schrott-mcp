# MKM Fullreview-Findings: Behebung und neue Evidenz

## Status und Freigabegrenze

Alle **zwei P1 und drei P2** aus `/tmp/opencode/mkm-adversarial-review-20261011/REVIEW.md` wurden am unveränderten Kandidaten selbst reproduziert und gezielt behoben. Die Behebung ist durch neue reale Regressionen und den vollständigen Folgelauf belegt. **Keine Selbstfreigabe:** erneutes unabhängiges Fullreview und Owner-Fullreview vor Integration/Produktionsaktivierung bleiben Pflicht. Der ursprüngliche Review samt ursprünglichen Repros/Logs wurde nicht verändert.

Worktree `/tmp/opencode/schrott-mcp-mkm-full-20261010`, unveränderter lokaler Merge-HEAD `ccd4dc79`, Basis `e3fcbbae`, Implementation weiterhin uncommitted. Kein Mainwrite/Push/Deploy/Prodwrite/GEO, keine Delegation, kein Stash/Reset/Discard. Eigener Target `/root/mkm-full-target-20261010`; keine fremden Targets gelöscht.

## Tatsächlich beobachtetes RED

Unverändertes externes Harness vor den Fixes mit tatsächlichen Kandidatenbibliotheken und eigenem Target ausgeführt: `/tmp/opencode/mkm-five-findings-red.log`. F1 First-import-Millberry bleibt current; vollständiger realer 12-Legacy-Cutover nach 0/60 lässt 10 kanonische Quotes stehen. F2 gleiche Historyrow erhält neuen observed_at. F3 hiddenLabel, F4 invalid99.99.2026, F5 unknownTierCondition werden jeweils fälschlich geschrieben. Zusätzlich eigene neue Regressionssuiten vor Implementation RED: `/tmp/opencode/mkm-review-ingestion-red.log` und `/tmp/opencode/mkm-review-store-red.log`.

## Findings einzeln: Behebung belegt

| Finding | Fix | Reale neue Evidenz |
|---|---|---|
| **F1 / P1 — erster Requestimport** | Zehn kanonische Legacyartikel zusätzlich zu beiden Aliasen **explizit** belegt durch separate Read-only-Fixture; nur exakter Material/Variant/Label-EUR/kg-upto-Vertrag, positive finite Quote, passender Maxwert/Confidence und keine fremde Validity. Nur Requestidentitäten autorisieren kanonisches Retirement. Source/Trader/Producer/Late-Schutz der atomaren Storetransaktion bleibt verbindlich. Keine Generalisierung über alle 60 IDs. | `review_f1_first_request_cutover_retires_all_proven_legacy_quotes`: echte 12 Current- plus 5 Acceptance-Zeilen direkt → echtes vollständiges 0/60-Parseroutcome → typed Record → TempDB, **ohne ersten typed Numericimport**, kein eigener belegter Current übrig; alle historischen Rowfelder unverändert. `review_f1_first_request_cutover_preserves_foreign_and_unproven_keys`: fremde Quelle/Producer/unbekanntes Label unverändert. |
| **F2 / P1 — Incomingmarker-only Historyschutz** | Zentraler `record_price_on()`-Guard prüft **bestehenden** MKM-Current-Owner und dessen Source/Source-type/Published-Scope. Unmarked/fremde Publicwrites reject statt historic UPDATE oder Pointerersatz. Same-owner semantic changes bleiben append-only; sonstige Legacy-/MSG-Dedupe unverändert. | `review_f2_real_typed_history_is_protected_from_public_legacy_writers`: echte Fixture → typed Import, danach öffentliche `record_price()` und Snapshot-API mit `{}`/fremdem Producer, späterem Datum, fremden Notes/Published-at; Fehler, gesamte bestehende Row/ID und Historycount unverändert. Storetest `review_f2_unmarked_or_foreign_writer_cannot_mutate_owned_history` prüft zusätzlich gleiche numerische Terms direkt am gemeinsamen Schreibkern. |
| **F3 / P2 — hidden Labelnachfahr** | Explizit versteckte Label- und Artikel-ID-Nachfahren machen die Row nicht aktuell; fehlende bekannte Artikel diagnostiziert, typed Record blockiert. Kein Ausschluss zugänglicher visually-hidden Caption. | `review_f3_f4_f5_invalid_source_contract_cannot_mutate_any_evidence`: direkte/nested hidden Labelteile, inline display:none, Großschreibung/!important; Parsererror oder fehlender bekannter Artikel, echter Record reject. |
| **F4 / P2 — defektes vorhandenes Seitendatum** | Fallible Erfassung vorhandener Pagechange-Anzeige statt `find_map()`-Verwerfen; ungültige/mehrdeutige Anzeige ist Fehler. Genuin fehlende Angabe bleibt Option::None, kein erfundenes PublishedDate. | Derselbe tatsächliche DB-Errorpfad: 99.99.2026, unknown, zweite Pagechange-Anzeige blockiert; genuine fehlende Anzeige wird getrennt als ehrliches None erfolgreich transportiert. |
| **F5 / P2 — verworfene neue Staffelbedingungen** | `Tier` deserialisiert ausschließlich exaktes `{ab,preis}` mit `deny_unknown_fields`; unbekannte Bedingungen sind Vertragsänderung/Fehler, nicht stille Vereinfachung. | Derselbe DB-Errorpfad: `only_customer:contract`, nested `condition.only_customer`, doppelte `ab`-Felder und ab-Objekt statt Zahl; keine Aufnahme. |

Der kombinierte F3–F5-Test vergleicht **sämtliche Rows** in prices/current_prices/trader_materials/source_article_snapshots/article_acceptance_owners vor und nach jedem Fehler. Keine Mutation von History, Current, Acceptance oder Ledger. Bestehende hidden-price-ancestor-, strikte Preisgrammatik-, Betreiber-/Registernummer- und Mapping-Oracletests bleiben grün.

Zusätzliche F2-Härtung: Eigentümerprüfung und Single-public-write sind jetzt in derselben SQLite-Transaktion, auch bei mehreren unabhängig geöffneten DB-Connections. Kein Check/Write-Race und kein partieller Historyinsert bei fehlgeschlagenem Currentupdate. Der neue Trigger-Repro `review_f2_public_single_write_keeps_owner_check_and_pointer_atomic` war tatsächlich RED (2 statt 1 Historyrow, `/tmp/opencode/mkm-review-public-write-red.log`), jetzt GREEN mit kompletter Store-Suite (`/tmp/opencode/mkm-review-store-final.log`, 40 passed). Normale Legacy-/MSG-Dedupesemantik unverändert.

## Finaler vollständiger Folgelauf

- Workspace: **657 passed / 0 failed / 5 ignored** (Ingestion 590 passed, Store 40 passed); `/tmp/opencode/mkm-review-workspace-20261011.log`.
- MKM: **33 passed**, MSG separat **13 passed**; `/tmp/opencode/mkm-review-{mkm,msg}-20261011.log`. MSG-Handlerfile unverändert; 46-Artikelvertrag unverändert.
- Python unabhängiger Audit: **9 passed**; `/tmp/opencode/mkm-review-python-20261011.log`.
- `cargo fmt --all -- --check` erfolgreich. Code-/Dokumentdiffcheck erfolgreich unter ausdrücklich dokumentiertem Ausschluss der zwei bytegetreuen ursprünglichen Raw-HTML-Fixtures und deren Quellwhitespace.
- Frischer **registrierter Read-only-Fetch**: **8 Tabellen / 60 Artikel / 55 numerisch / 5 Anfrage**, Diagnostics leer. `research/mkm-reviewed-typed-registered-live-inventory-20261011.json`; `production_ready=false` bleibt.
- Separat erneut erfasste echte HTML-Primärquelle, unabhängiger per-Artikel-Preis/Mengen/Staffel/Datum- und fixed Material/Variant-Oraclematch: `research/mkm-reviewed-typed-contract-audit-20261011.json`, **candidate_article_contract_gaps=[]**, `production_approval=false` bleibt. HTTP-Header/Hashes und Testsummary: `research/mkm-reviewed-remediation-evidence-20261011.json`.

## Übergabe und offene Grenzen

Vollständiger aktualisierter Patch gegen MSG-Basis `e3fcbbae`: `/tmp/opencode/mkm-production-seam-20261011.patch`. Implementation nicht committed; nur bereits autorisierte alte Baseline-/Mergecommits. Main ist während des Laufs vom Owner auf `2ca3028c` fortgeschrieben worden (seit beauftragtem710233c6 ausschließlich Owner-Handoffdoc); nicht verändert/synchronisiert/überschrieben. Patch auf aktuellem Main lediglich `git apply --check` prüfen, nicht anwenden.

Die dokumentierte Store/Validator-Vertrauensgrenze bleibt: Store ist kein eigener Fachmapping-Parser, die öffentliche MKM-Manifestseam setzt vollständige Validierung durch `record_articles()` voraus. Der registrierte Scheduler tut das; zukünftige Aufrufer müssen dies ebenfalls tun. Das im Review ausdrücklich **nicht als Finding** gezählte Direkt-Store-Forge ist nicht als geschlossener sechster Befund oder eigenständige Store-Komplettheitsprüfung ausgegeben. Unbekannte/fremde Legacykeys werden weiterhin nicht retired, nur bekannte proven Keys und eigene verwaltete Beobachtungen.

Offen: erneutes unabhängiges Review des gesamten Kandidaten samt diesen Fixes, Owner-Fullreview, spätere Integration und produktiver End-to-End-Abgleich durch Owner. Kein Produktionserfolg und keine Freigabe aus Count-/Testgrün abgeleitet.
