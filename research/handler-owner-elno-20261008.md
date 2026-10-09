# ELNO Handler-Ownership (#5519)

## Auftrag und isolierter Stand

- Branch: `owner/elno-20261008`, Basis: `072cf25`.
- Worktree: `/root/Documents/schrott-mcp-elno-20261008` (aus eigenem HEAD-Worktree verschoben, weil `/tmp` voll war).
- Ziel: dynamischen PDF-Handler für `be-spandau-elno-container-und-dienstleistungs` implementieren, Regressionen und echten Live-Abruf prüfen; Parent rollt beide Binaries seriell aus. Produktionsdatenbanken bleiben read-only.
- Rollout und regulärer Prod-Scrape sind **offen**, nicht durch Buildgrün ersetzt.

## Belege

- Betreiber: https://www.elno-container.de/impressum/ — ELNO Container- und Dienstleistungs GmbH, HRB 141261 B, Tiefwerderweg 13, 13597 Berlin. Kongruent mit erhaltener Dossierhistorie zu Register und aktuellem EFB-Zertifikat.
- Preisquelle: Startseite https://www.elno-container.de/ , verlinktes zweiseitiges PDF https://www.elno-container.de/wp-content/uploads/2026/09/Preisliste_11.09.pdf . Keine Datumsaussage im Dokument; kein Datum aus URL ableiten.
- Privatkunden, freibleibend/tagesabhängig, Einstufung vor Ort; Gewerbe telefonisch anfragen. Metallpreise €/kg, Abfallgebühren separat netto €/cbm bzw. €/to, niemals Ankaufpreise.
- PDF-Fixture SHA-256: `f406df3eb717c230c488010fb073f37eba0e6e0184a3176658edd7fe4dcde6e9`.
- Read-only Prod-Baseline: Trader-ID `411`, richtiger Betreiber/Adresse, `0` current_prices (vor Handlerdeployment).

## Infrastruktur während Entwicklung

Mehrfache Harness-Neustarts unterbrachen Builds. Gemeinsamer `/tmp`-Datenträger und anschließend Root-Datenträger liefen durch parallele Builds voll. Ausschließlich eigene Buildartefakte wurden entfernt; kompakter Ein-Job-Build ohne Debuginfo/Incremental wird verwendet. Keine fremden Änderungen verworfen.

## Übergabe / noch offener Nachweis

Owner-Nachprüfung am 09.10. nach RAM-Erweiterung:
`cargo test --locked -p schrott-mcp-ingestion elno`: drei Tests bestanden,
einschließlich tatsächlicher PDF-Binärfixture und Betreiberprüfung.
Erneuter Rust-Livehandler nach Streaming-Härtung: HTTP 200, 210199 Bytes,
36 Preiszeilen, EUR/kg, kein erfundenes Preisdatum. Zwei numerische Sorten
(`Sorte 3`, `Cu-MS Kühler`) sind ausdrücklich ohne exakte Katalogzuordnung
gemeldet; Elektronikschrott wird als Anfragepreis ausgeschlossen.
Autobatterien nutzen `batterien-blei`, dessen Katalogeintrag mit Trapper in
den gemeinsamen Release kommt. PDF-Downloads werden bereits beim Lesen
auf 5 MB begrenzt. Nach Review ersetzt ein abbrechbarer Poppler-Prozess die
ursprüngliche Rust-PDF-Auswertung; `pdf-extract` und das Debug-Example wurden
entfernt. Deployment benötigt `/usr/bin/prlimit` (util-linux) und
`/usr/bin/pdftotext` (poppler-utils). Auswertung: 256 MiB Adressraum,
15 Sekunden CPU, 20 Sekunden Laufzeit, 1 MB stdout und 64 KiB stderr.
Rustfmt erfolgreich; Workspace-/Integrations-/Produktionsgate noch offen.

Nach Commit integriert der Parent den isolierten Stand, baut und rollt **Server und Query-Worker** gemeinsam seriell aus. Danach regulären Schedulerlauf abwarten (kein manueller DB-Write und kein Force-Run als regulären Nachweis ausgeben). Read-only verifizieren: `ingestion_steps` erfolgreich mit Preisanzahl, `raw_fetches` tatsächliche PDF-URL/HTTP 200, `current_prices` Varianten/Einheiten und fehlendes erfundenes published_at. Ownership bleibt bis zu diesem Nachweis offen.
