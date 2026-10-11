# MKM F6: Katalogunit am typed Record-Boundary

## Früh persistierter RED-Stand

Owner-Repro `/root/mkm-owner-unit-review` selbst gegen den unveränderten Kandidaten ausgeführt, eigener Target `/root/mkm-full-target-20261010`: **RED**, `/tmp/opencode/mkm-f6-owner-red.log`. Validierter zukünftiger numerischer Requestartikel MKM-SCH-02 speichert 11.13 EUR/kg statt Katalogunit EUR/t und 11130 EUR/t. Keine Behauptung, die heutige Quelle habe einen numerischen Stahlpreis oder Prod sei heute falsch.

F1–F5 bleiben gemäß unabhängigen 143 Harnesscases scope-freigegeben; sie werden nicht ohne nachgewiesene Regression wieder geöffnet. Aufgabe ausschließlich F6: alle MKM-Preisprojektionen vor Storewrite am echten Katalog normalisieren, source article €/kg/Staffeln/Mengen unverändert bewahren, unbelegte Zielunits fail-closed. Echte HTMLfixture→Parser→typedRecord→TempDB-Regressionen für Eisenschrott und Metallschrott/Mischschrott sowie alle Materialunits erforderlich. Noch keine Implementation-/F6-Freigabe.

Implementation bleibt uncommitted im bestehenden isolierten Worktree. Kein Mainwrite/Push/Deploy/Prodwrite/GEO, keine Delegation. Owner arbeitet parallel an anderem Parser-/Auditcode, daher keine dortigen Änderungen ohne notwendige Evidenz. Erneutes unabhängiges F6-Review vor Integration Pflicht.

## Persistierter GREEN-Zwischenstand

- Owner-Harness unverändert erneut selbst ausgeführt: `/tmp/opencode/mkm-f6-owner-green.log`, Exit0, gespeicherter MKM-SCH-02-Quote **11130 EUR/t** bei unverändertem Quellpreis **11.13 EUR/kg**.
- Eigene zwei neue Regressionen waren vor Fix tatsächlich RED (`/tmp/opencode/mkm-f6-regression-red.log`): EUR/t-Projection falsch und unsupported Catalogunit fälschlich akzeptiert. Jetzt drei F6-Tests GREEN (`/tmp/opencode/mkm-f6-regression-green.log`).
- Änderung ausschließlich `recording.rs`: alle 60 Artikelmaterialien vor Schreibbeginn auf tatsächliche Catalogunit EUR/kg oder EUR/t prüfen; Missing/unsupported Unit ist Fehler. Shared `traders::normalize_unit()` **unverändert wiederverwendet** (private Ancestor-Seam ist dem Childmodul bereits zugänglich, keine globale API-/Legacyänderung nötig). Sämtliche NewPricewerte/bothbounds/unit aus normalisierter Projection, nicht mehr direkt vom source Outcome. Nonfinite/abweichende Unit nach Konversion fail-closed.
- Echte HTMLfixture→Parser→typedRecord→TempDB für zwei synthetisch zukünftige Request→Numeric-Artikel **MKM-SCH-01 Eisenschrott und MKM-SCH-02 Mischschrott**: 57/3, jeweils Storeprice/max **11130 EUR/t**; sourceprice weiterhin11.13, alle vier Staffeln unverändert, minimum250kg unverändert, PublishedDate NULL. Keine Behauptung heutiger numerischer Stahlquotes. Der numerische Fixturevertrag wird aus tatsächlichen Millberry-Sourcewerten konstruiert, nicht durch Änderungen am Katalog kaschiert.
- Alle 60 mapped Artikel geprüft gegen festes, reviewtes Unit-Orakel: nur `mischschrott` und `schrott-eisen-ne-gemischt` EUR/t; alle anderen EUR/kg. Jede numerische Currentquote muss tatsächlicher Catalogunit entsprechen. Zusätzlicher Shared-Seam-Test skaliert min10.6/max11.13 →10600/11130; keine heute unbelegte Rangegrammatik aktiviert (aktueller upto-Validator verbietet weiterhin sourceprice_min).
- Mutierte **eigene TempDB-Catalogunits**, nicht sourcecatalog: EUR/Stk, USD/t und empty, darunter request-only Material. Komplette Preise/Current/Acceptances/Manifest/Ownership vor/nach Fehler wertgleich, kein partieller Storewrite/Retirement.
- Source-Snapshot/Outcome/Manifest/extra.article behalten Originalvertragswerte. Vorhandenes unabhängiges Mapping-Orakel und Python-Audit sind **Source-Inventory-Audits**, vergleichen `price_eur_per_kg`, rawpricecell, tiers und kg-thresholds, nicht NewPrice/DBquotes. Deshalb kein Preis-/Unitumschreiben im Source-JSON und keine Änderung am parallel vom Owner geprüften Parser-/Auditcode nötig. Store-Projektion separat mit realen DB-Feldern und unabhängig festen11130/EURt-Assertions geprüft.

Noch ausstehend zu diesem Zwischenstand: vollständige F1–F5/MSG/Legacy-/Workspace-/fmt-/Python-/frische Livegates, Patchneugenerierung und unabhängiges F6-Review. Kein Selbstapproval.

## Finaler F6-Kandidatenstand / Handoff

**F6-Behebung durch eigene RED→GREEN-Evidenz belegt, unabhängiges F6-Review noch offen.** Owner-Repro unverändert GREEN; vollständiger Folgelauf:

- Workspace **660 passed / 0 failed / 5 ignored**, einschließlich Legacy-/F1–F5-Regressionspfaden; Ingestion593, Store40. Log `/tmp/opencode/mkm-f6-workspace-20261011.log`.
- MKM **36 passed**, MSG separat **13 passed**, Python-Audit **9 passed**; `/tmp/opencode/mkm-f6-{mkm,msg,python}-20261011.log`.
- `cargo fmt --all -- --check` erfolgreich. Code-/Dokumentdiffcheck mit explizitem Ausschluss der zwei unveränderten bytegetreuen ursprünglichen Raw-HTML-Fixtures erfolgreich; kein pauschales Whitespace-Grün über deren Quellbytes.
- Frischer tatsächlicher registrierter **read-only** Sourcepfad **8 Tabellen / 60 Artikel / 55 numerisch / 5 Anfrage**, Diagnostics leer. Heutige betroffene Stahlartikel weiterhin Anfrage, keine synthetischen Testpreise in Sourceinventur.
- Frisch separat erfasste Übersicht/Impressum mit HTTP-Headern/Hashes; unabhängiger per-Artikel-Source-/Material-/Variant-Audit **candidate_article_contract_gaps=[]**. Sourceaudit weiterhin ausdrücklich kein normalized-DB-Audit; neue Store-Unit-/Moneybound-Regressionen liefern diesen separaten Beleg.
- Artefakte: `research/mkm-f6-registered-live-inventory-20261011.json`, `research/mkm-f6-source-contract-audit-20261011.json`, `research/mkm-f6-normalization-evidence-20261011.json`. Source-/Auditreports behalten `production_ready=false` / `production_approval=false`; keine Selbstfreigabe.

F1–F5-Freigabe bleibt genau der unabhängig überprüfte Scope von `/tmp/opencode/mkm-adversarial-review-20261011/RE-REVIEW.md` (abschließend143 erfolgreich geprüfte Harnesscases); keine neue Regression gefunden. Dieser Agent hat den unabhängigen Harness-/Reviewbestand nicht überschrieben und stellt seine eigenen aktuellen Tests nicht als neues unabhängiges Review aus.

F6-Codeänderung ausschließlich `crates/ingestion/src/traders/handlers/mkm_metals/recording.rs`. `traders/mod.rs`-Normalizer, Store, Parser, Katalogdefinitionen, bestehender Mappingoracle und Python-Audit gegenüber dem F1–F5-Stand **nicht** dafür geändert. Keine Katalogunitänderung als Bugmaskierung; Unitmutationen nur als fail-closed-Tests in eigenen TempDBs.

Vollständiger aktualisierter MKM-Kandidatenpatch gegen synchronisierte MSG-Basis `e3fcbbae`: `/tmp/opencode/mkm-production-seam-20261011.patch`. Uncommitted Implementation bleibt erhalten. Main inzwischen durch Owner auf `41b08cbf` fortgeschrieben (seit MSG-Basis ausschließlich Verifier/Docs, keine MKM-Codeüberschneidung); nicht geschrieben/merged/rebased/stashed/reset. Nur `git apply --check` auf aktuellem Main, kein Apply.

**Offen:** erneutes unabhängiges F6-Review und Owner-Gesamt-/Integrationsfreigabe, späterer produktiver End-to-End-Abgleich/Rollout durch Owner. Keine Push-/Deploy-/Authversuche, keinerlei Prod-DB-/GEO-Aufrufe in diesem Auftrag.
