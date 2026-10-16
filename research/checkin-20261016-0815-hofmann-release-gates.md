# Hofmann V3: Owner-Schemaentscheid und verbleibender Releaseplan

16.10.2026,08:15UTC. Code-PASS aus `research/checkin-20261016-0800-hofmann-v3-review.md` vollständig gelesen. Owner hat zusätzlich die produktive Implementierung des neuen Storemoduls, einschließlich Schema, UTC-Guard, vollständigem Legacyprädikat und Transaktionsgrenze, selbst gelesen. Dies ist keine Deploymentfreigabe.

## Konkreter Schemaentscheid

Die additive Tabelle `hofmann_tin_cutovers` ist für die **Integration** akzeptiert: dauerhafte einmalige Entscheidung statt wiederholter Abwesenheitslöschung; Schlüssel Händler/Material, Quellenbindung, Entscheidungszeit und optionale retired Preis-ID. Bestehende user_version6 bleibt unverändert. `PublicDb::open` erstellt lediglich die fehlende Tabelle, kein Retirement oder Timestamp-/Historyrewrite beim Boot. Tatsächliche Entscheidung erst durch qualifizierte Beobachtung. Read-only-Worker erzeugt diese Tabelle nicht. Frische Reviewer-Boot-/Reopen-/Rollbacktests stützen diesen begrenzten Vertrag, ersetzen aber noch keine Probe auf einer aktuellen Produktionskopie samt vollständigem Seed.

## Reale Legacyform frisch bestätigt

08:14:42UTC ausschließlich `mode=ro`/`query_only` auf public.db: user_version6, Ledger noch nicht vorhanden. Genau ein Hofmann-Zinn-Current: Preis-ID13725, Händler137, Material14, Variante leer,11.5EUR/kg,exact/confidence1.0,haendler_angabe/published, genaue Hofmann-Quell-URL, Label Zinn, map_v2/page_stated, keine Min-/Max-/Gültigkeitsgrenzen und kein snapshot_owner. observed_at14.10.2026 03:24:24.104857250+00:00. Dies passt zum gelesenen engen Legacyprädikat; zukünftige Beobachtung muss zusätzlich dessen Zeitbedingung erfüllen. Rohbeleg `/tmp/opencode/hofmann-v3-owner-production-shape.json`. Keine Preise oder Pointer verändert. Einheit active/MainPID15872; dies allein ist **kein** Nachweis exklusiver Writer.

## Verpflichtende Gates vor Installation

1. Genau den manifestierten V3-Vollpatch übernehmen und acht Quellhashes bestätigen. Sauberer aktueller Main-Export plus ausschließlich geprüfte Änderungen, keine143 fremden Dossiers. rustfmt, Workspace, Python-Verifier und frische DB-freie Live-Scrapes auf diesem Stand. Integration/Commit sind nicht gleich Deployment.
2. Aktuelle WAL-konsistente SQLite-Sicherung über Backup-API aus read-only geöffneten Quellen in ein privates Verzeichnis außerhalb Produktion; Integrität prüfen. Keine bloßen Kopien laufender db-Dateien ohne WAL. Beide Datenbanken und Altbinaries mit Hashes/Zeiten binden.
3. Neuer normaler Boot-/Seedpfad auf dieser privaten Kopie prüfen: leere additive Tabelle direkt nach Open, vollständige Seedwirkung separat diffen und akzeptieren, keine unbeabsichtigten Preis-/Currentänderungen. Dann qualifizierten Hofmannübergang ausschließlich auf privater Kopie proben: genaue ID13725 aus Current entfernt, gesamte alte Historienzeile identisch, korrekte alt-Zeile/Ledger, Nachbarquotes erhalten, Retry/Reopen und Replayguard. Das ist keine erzwungene Produktionsingestion.
4. Erst nach grünen Gates Releaseartefakte aus exakt freigegebenem Commit neu bauen und hashbinden. Regulärer Push muss erfolgreich und Remote-Commit bestätigt sein; derzeit fehlt HTTPS-Authentifizierung. Nicht mit einem alten81a-Artefakt oder einem ungeprüften Dirtytree installieren.
5. Vor Installation alle tatsächlich schreibenden Prozesse identifizieren, regulären Service geordnet stoppen und Stopp bestätigen. Kein Mischbetrieb alter/neuer Writer. Eine einzelne MainPID oder ein erfolgloser Prozessnamensfilter genügt nicht als Exklusivitätsbeweis.
6. Nach genehmigter Installation Dienst/Health und **regulären** Schedulerlauf abwarten; ausschließlich lesend IDs, individuelle Row-Dates, Einheiten, Preisarten, Historien-/Nachbarerhalt und Ledgerentscheidung nachweisen. Kein manuelles Nachhelfen an Daten oder Ledger.

## Fail-safe/Rollback

Ein alter Binarytausch ist nach erfolgter Ledgerentscheidung **kein sicherer Rollback**: alte Writer ignorieren die Grenze. Bei Fehlern bleibt Ingestion gestoppt, bis entweder der neue Stand sicher korrigiert oder ein ausdrücklich geprüfter konsistenter Binary-/DB-Restoreplan freigegeben ist. Keine automatische Ledgerlöschung/Downmigration. Die allgemeine Erlaubnis zum Redeploy ist keine Erlaubnis, Produktionsdatenbanken direkt zurückzuschreiben; ein eventueller Restore benötigt gesonderte ausdrückliche Freigabe. Sicherer Standard ist daher Stopp statt unbewiesener Datenrücknahme. Backup/Restore und Writerexklusivität sind hier beschrieben, noch nicht durchgeführt oder als getestet ausgegeben.
