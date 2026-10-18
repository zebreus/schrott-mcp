# MKM: echte historische typed-60→61-Regression als Anwenderpatch

## Ergebnis / Koordination

**Zusätzliche Regression vorbereitet, nicht integriert und nicht ausgeführt.**
Patch: `/tmp/opencode/mkm-real-transition-test.patch.txt`, OpenCode-Patchformat,
relative Pfade ab Repositoryroot, 210068 Bytes, SHA-256:
`8996961650751fc3e9755f9a3714259e39c93ec9935ed26ea029501f48df4d87`.

`recording.rs`, Store, Goldtrans, Python-Skripte, Cargo-Dateien und bestehende
Fixtures wurden hier **nicht bearbeitet**. Damit kein konkurrierender Eingriff
in die Owner-/Implementerarbeiten. Die einzige neue Repositorydatei dieses
Auftrags ist dieser Bericht. Patch erst nach deren Abschluss integrieren;
bei verändertem Kontext den einzelnen Include-Hunk bewusst übernehmen.

## Historischer Seed statt nachgebautem Writer

Gelesen: ursprünglicher `record_articles`-/Storevertrag, aktueller Parser,
unveränderte alte Capture-/Oraclepfade, Testhelpers und vorhandener externer
`/root/mkm-owner-unit-review/src/bin/rollout_fixture.rs`.
Der generische `record_observation`-Legacyhelper reicht ausdrücklich nicht:
er verliert Artikel-/Staffelmetadaten und erzeugt keine typed Ownership.

Stattdessen vorhandene **isolierte** Harness-DB ausschließlich read-only
(`mode=ro`, `query_only=ON`, Lesetransaktion) logisch exportiert:
`/root/mkm-owner-unit-review/rollout-fixture-196750/public.db`.
Kein Zugriff auf Produktions-DBs. Enthalten und beim Export geprüft:

| Tabelle | Zeilen |
|---|---:|
| prices | 55 |
| current_prices | 55 |
| trader_materials | 40 |
| source_article_snapshots | 1 |
| article_acceptance_owners | 40 |

Ledgerdatum `2026-10-11T03:00:00.000000000+00:00`, Owner `mkm-article-v1`,
kanonische MKM-URL, exakt die alten 60 Oracleidentitäten/-Mappingpaare und
55 numerische Artikel. Manifeststring-SHA-256:
`720224db2af390c6abfaf2633cce7d57e5aebf107936040e12c99a9d946b8f55`.
Das ist vorhandene isolierte Persistenzevidenz, **keine Produktionskopie**.
Die Provenienz des damals erzeugenden Binaries wurde nicht neu unabhängig
etabliert; keine Behauptung eines frisch ausgeführten historischen Writers.

Patch enthält diesen vollständigen logischen Fünf-Tabellen-Export als neue
Fixture `mkm-historical-typed60-harness-20261011.json`, 190744 Bytes,
SHA-256 `65129560699cd83e102d10d0a13ab5735af1b3f0a7b21141822164cf785d90d2`.
Nur öffentliche Artikel-/Preisdaten und synthetische Harnessprovenienz;
keine HTTP-Header, Cookies oder Credentials exportiert. Alte HTML-/Oracledatei
haben weiterhin keinen Diff gegen `4a61556f`.

## Patchumfang und beabsichtigte Aussage

- Test-only `rusqlite 0.32` mit `bundled` als Devdependency; gleiche bereits
  verwendete Storeversion. Zugehörige direkte Dependencykante in `Cargo.lock`.
- Ein Include innerhalb des bestehenden `#[cfg(test)]`-Moduls in `recording.rs`.
- Neue benachbarte Datei `recording_historical_transition_test.rs` und obige Fixture.
- Keine produktive 60/61-Ausnahme, kein neuer Produktionshook, kein nachgebauter
  Recording-/Normalisierungs-/Retirementalgorithmus und keine Trigger/Fake-Erfolge.

Testname:
`traders::handlers::mkm_metals::recording::tests::historical_typed_60_to_real_61_preserves_evidence_and_ownership`.

Der Test lädt die historischen Tabellen in eine frische `TempDbDir`, mit
DB-lokaler Material-/Trader-ID-Zuordnung. Nur bei anderem Trader-ID wird dessen
Feld im importierten Manifest angepasst; ansonsten bleibt der originale String
erhalten. Ab Import muss der gesamte alte Ledger einschließlich Manifeststring
bytegleich erhalten bleiben. Vor dem Übergang prüft der Test alle alten
Artikelmetadaten gegen den echten alten Parser/Oracle, 55 Currentpreise,
fünf Messingmitglieder und alle 40 Acceptance-/Ownershippaare.

Zwei unabhängig vorbereitete TempDB-Szenarien:

1. Echter neuer datierter Capture → Parser → vorhandenes `commit` →
   `record_outcome` → **reales `record_articles` / Store**. Erwartet 56/5,
   einen zusätzlich angehängten 61er-Ledger, eigene MS58-Variante, weiterhin
   getrennte fünf alte Messingidentitäten und sechs Messing-Acceptancemitglieder.
   Sämtliche alten Preisfelder bleiben als vollständige Zeilen erhalten;
   neue Beobachtungen dürfen angehängt werden. Alle aktuellen numerischen/
   Anfragekeys werden gegen den neuen Ledger geprüft; alle historischen
   Material-Acceptances samt Ownership gegen seine semantischen Artikel und
   Beobachtungszeit. Zusätzliches gleichtraderiges, unabhängig besessenes
   Gold-Current/History-/Acceptance-/Owner-Evidenz bleibt vollständig gleich.
2. Bereits vor dem Aufruf fremd besessene Messing-Acceptance: derselbe echte
   61er-Pfad muss exakt `foreign acceptance producer` erreichen, nicht schon
   am veralteten Storecount scheitern. Vorher/nachher vollständiger, sortierter
   Feldvergleich aller fünf Tabellen. Kein gezielter Fehlertrigger.

Nach erfolgreichem Übergang wird die echte alte 60er-Quelle erneut angeboten:
heutiger Recordingguard muss sie ablehnen, wieder ohne irgendeine Änderung
in diesen fünf Tabellen. Erfolgsfall und spezifischer Konfliktfehler verhindern
eine bloß wegen Vorvalidierung scheinbar grüne Negativregression.

## Evidenzgrenze / Ownerfolge

Ausgeführt wurden ausschließlich Quellenlektüre und read-only Export/
Konsistenzprüfungen der vorhandenen isolierten Fixture-DB; ein privater
Serialisierungsschritt erzeugte das Patchartefakt. **Kein Rustbuild, kein
Rust-/Python-Testlauf, keine Patchanwendung, kein Commit, keine Produktionquery
oder -schreiboperation.** Keine Behauptung eines Compiler- oder PASS-Nachweises.

Voraussetzung für den späteren positiven Lauf bleibt die separat vorbereitete
Store-61-Integration. Owner danach budgetiert diesen Test frisch kompilieren/
ausführen lassen; Cargo-Devdependency/Lockfile und Include-Kontext dabei prüfen.
Diese Regression belegt bei Erfolg einen Übergang aus real vorhandener alter
typed Persistenzevidenz am heutigen Recordingseam, nicht die erneute Ausführung
des historischen Writers oder einen Live-/Deployment-/Recoverynachweis.
