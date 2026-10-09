# Handler-Owner Trapper (#5528)

- Owner-Branch: `owner/trapper-20261008`; Basis `072cf25`.
- Worktree: `/tmp/opencode/owner-trapper`.
- Implementation-Commits: `42b9c08` (Handler/Katalog/Dossier/Fixture), `2aa4fb9` (begrenzter Download, dynamische Monatsupdates, Kategorienbedingung).
- Status: **ROLLOUT-READY** (09.10.2026). Tests und Live-Rust-Scrape bestanden; Deployment und regulärer Prod-Scrape bleiben offen und werden vom Owner nach dem seriellen Parent-Rollout geprüft.
- Gemeinsamen Rollout beider Binaries koordiniert der Parent seriell. Produktions-DBs bleiben für diesen Owner read-only.

## Primärquellen / Preissemantik

Am 09.10.2026 neu geprüft: Betreiber-Impressum Trapper GmbH, HRB 3736 Bayreuth, Am Goldenen Feld 31, Kulmbach kongruent mit erhaltenem Dossier. `https://www.trapper-kulmbach.de/service/preise` verlinkt weiterhin `/upload/preisliste-trappergmbh-anlieferung-august-2026.pdf` (SHA256 `15382e62e0ef36312f6df939133f99f738b148983f790912985267dbd4f9383d`). PDF: **Anlieferung / Ankauf**, gültig ab 04.08.2026; keine aktuellen Oktober-Tagespreise. 21 Sorten: 8 €/t, 13 €/kg. Freibleibende, mengen-/zusammensetzungsabhängige Richtwerte. Keine USt-Basis ausgewiesen, keine Verkaufspreise/Gebühren.

100 kg ist ausschließlich Vergütungsgrenze für Stahlschrott/Guss, nicht Annahmemindestmenge. Unterqualitäten von Kupferblech/Raff, Guss/Bremsscheiben, Alublech und Zink bleiben unterschiedliche Varianten. VA-Abfälle wird nicht als V2A/V4A behauptet; isolierte Rohre nicht als blankes Kupfer. Katalog additiv um expliziten Trägerschrott und Bleibatterien ergänzt, statt diese als scherengerecht oder Weichblei zu erfinden.

## Implementierung / Deployment-Gates

`trapper.rs`: HTML-Link bei jedem Lauf neu entdecken; eindeutiger HTTPS-Betreiber-PDF-Link, keine fest verdrahtete Monatsdatei. PDF über Poppler `pdftotext -layout - -` extrahieren, stdin/stdout ohne temporäre Dateien. Kindprozess wird bei Scheduler-Abbruch beendet. PDF-Identität, Anlieferungsrichtung, explizites Datum und 100-kg-Bedingung prüfen; geänderte Verträge/Einheiten/duplizierte Grades scheitern laut, unbekannte Sorten werden protokolliert. PDF-URL ist Preis-Provenienz; `record()` normalisiert €/t zu Katalog-€/kg für Motoren/Batterien/Platinen.

**Deployment benötigt `/usr/bin/pdftotext` (poppler-utils) und
`/usr/bin/prlimit` (util-linux)**, beide mit absoluten Pfaden. systemd
`schrott-mcp.service` läuft als root ohne Prozess-Sandbox. Vor Rollout beide
Executables im Service-Dateisystem prüfen. Nach Review: 256 MiB Adressraum,
15 Sekunden CPU, 20 Sekunden Laufzeit, stdout maximal 1 MB, stderr 64 KiB;
Kindprozess wird auch bei Abbruch beendet. Richtwerte werden strukturiert als
`approx` mit confidence 0.8 erfasst. Keine zusätzlichen Rust-Abhängigkeiten
und keine DB-Migration.

## Verifikation / Übergabe

Build-Artefakte ausschließlich `/var/tmp/opencode-trapper-target` (debug=0, jobs=1 wegen Speicherlimit). Extraktionsbelege `/var/tmp/trapper-owner-live.pdf`, `/var/tmp/trapper-owner-live.txt`.

- `cargo test -p schrott-mcp-ingestion trapper` auf `2aa4fb9`: **5 bestanden, 0 fehlgeschlagen**, einschließlich Aufnahme aller 21 Katalogpreise und Tonnen→kg-Normalisierung im isolierten Test-DB-Verzeichnis sowie neue PDF-Daten/Preise.
- `cargo test --workspace`: **568 bestanden, 0 fehlgeschlagen, 4 bestehende Tests ignoriert**; Ingestion allein 511 bestanden / 4 ignoriert. Vollständige Dossier-/Seed-Prüfung enthalten. Log `/var/tmp/trapper-workspace.log`; abgeschlossener Verifikations-Service `trapper-owner-verification.service`, `Result=success`, `ExecMainStatus=0`.
- `cargo run -p schrott-mcp-ingestion --example live_handlers by-kulmbach-trapper`: **HTTP 200, 325238 PDF-Bytes, 21 Preise, 0 Skips**, `published_at=2026-08-04T00:00:00+00:00`, Website alive. Alle 21 Live-Preise stimmen mit neu gelesenem Betreiber-PDF überein. End-to-end HTML-Discovery → HTTP-PDF → Poppler → Rust-Parser erfolgreich, ohne DB-Zugriff. Log `/var/tmp/trapper-live.log`.
- Standards-Review gegen `072cf25`: README-Standards erfüllt (eigener Parser/Labelmapping, statischer Katalog, keine geteilte Preisparsinglogik, Historie erhalten). Beim Eigenreview Download vor vollständigem Einlesen auf 5 MB begrenzt und Zuordnung des Fußnoten-Sterns zu Stahl/Guss abgesichert; in `2aa4fb9` behoben.
- Spec-Review gegen expliziten Ownerauftrag: echter dynamischer PDF-Abruf, alle 21 Sorten, gemischte Einheiten, Preisrichtung/Betreiber/Datum, Regressionen und isolierte Commits vorhanden. Deployment und regulärer Prod-Step ausdrücklich noch offen. Kein Scope-Creep außer zwei notwendigen, additiven Katalogmaterialien.
- Die Code-Review-Skill verlangte parallele Review-Subagents; Harness verweigerte diese konkret mit `Subagent depth limit reached (1)`. Beide Achsen daher als Eigenreview durchgeführt, nicht als unabhängige Fremdprüfung dargestellt. Kein `docs/agents/issue-tracker.md` vorhanden; verwendete Spec ist der explizite Auftrag der Parent-Session.
- Scheduler-Hashphase: 17.960 s nach jeder UTC-6h-Grenze, also **04:59:20 / 10:59:20 / 16:59:20 / 22:59:20 UTC**, nächster Tick typischerweise binnen 15 Minuten (sequentielle Vorgänger können zusätzlich verzögern). Keine manuelle Änderung an due-Map/Cadence erforderlich.

Read-only Produktionsbaseline am 09.10.2026: Slug löst **ID 237** auf (nicht Issue-/Recherche-Nummer 5528!), 0 Preiszeilen, keine `ingestion_steps` für den Slug. Nach Rollout ausschließlich über Slug/Join verifizieren: regulären Scheduler-Step `ok`, 21 Preise, 0 Skips/Canaries, PDF-Provenienz/Datum sowie normierte Motoren 0,40 €/kg, Starterbatterien/Platinen 0,30 €/kg und getrennte Varianten. Kein manuelles Force-Ingest oder DB-Schreiben durch den Owner. Owner bleibt bis zum erfolgreichen regulären Produktionsscrape zuständig.

## Parent-Rollout

Branch ab Basis `072cf25` vollständig übernehmen (zwei Code-Commits oben plus abschließender Evidence-Commit); `handlers/mod.rs` und `pipeline.rs` können mit parallelen Handler-Ownern kollidieren: beide neuen Registry-/Katalogeinträge erhalten. Beide Release-Binaries gemeinsam auf dem integrierten Stand bauen/deployen. Kein separater Rollout durch diesen Owner. Nach Parent-Bestätigung den nächsten regulären Trapper-Step abwarten (Hashphase oben), read-only Evidenz im Dossier/Übergabedokument ergänzen und erst dann Ownership als erfüllt melden.
