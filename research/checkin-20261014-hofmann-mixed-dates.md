# Check-in: Hofmann Rastatt – gemischte Zeilendaten

## Frische Prüfung

Service aktiv, HTTPS `https://schrottindex.de/health` meldet `ok:true`.
Produktionsabfragen ausschließlich `sqlite3 -readonly`: 3881 Händler,
54 Materialien, 2426 aktuelle Preise. Feedback-IDs 7381–7390 unverändert.
Keine Geo-Logs oder systemd-Prioritätswarnungen. JSON-WARNs separat gelesen:
Münster Preisleiter, AMR HTTP502, Huth fehlende Preistabelle und neu
Hofmann/Rastatt um 09:27:23 UTC mit gemischten Zeilendaten.

## Reproduktion und Quelle

Tatsächlich ausgeführt:

```sh
/root/.cargo/bin/cargo run -p schrott-mcp-ingestion --example live_handlers -- bw-rastatt-hofmann
```

DB-freies Ergebnis: Exitcode **1**, `mixed row publication dates: row-date
support required`. Keine Ingestion oder Produktionsänderung ausgelöst.

Quelle vollständig gelesen und als HTML erfasst:
https://hofmann-entsorgung.de/preise-und-verguetungen/
(`/tmp/opencode/hofmann-rastatt-20261014.html`). 23 Zeilen mit EUR/t.
20 Zeilen nennen 14.10.2026, drei (Starterbatterien, V2A, Elektromotoren)
weiterhin 30.09.2026. Zinn heißt jetzt ausdrücklich **Zinn alt**.
Gebühren-PDF nicht als Ankaufquelle verwendet.

Prüfhypothesen: (1) fehlende zeilenbezogene Datumsunterstützung, (2) neue
Sortenbezeichnung, (3) Struktur-/Einheitenänderung. Ein minimierter
Zweizeilen-Test isoliert (1): beide Einzelzeilen sind parsebar, gemeinsam
scheitern sie gezielt an der Datumsgrenze. Kontext/Header/Einheiten sind
unverändert. Die Ablehnung schützt vor falscher Veröffentlichung mit
pauschalem neuen Datum; sie darf nicht einfach entfernt werden.

## Fortschritt, nicht vollständige Fehlerbehebung

Separaten Mappingfehler (2) test-first nachgewiesen: neue Testzeile `Zinn alt`
ergab zunächst nur 22 statt 23 Preise. Explizites Mapping auf Material `zinn`,
Variante `alt` ergänzt; alte unspezifizierte `Zinn`-Variante bleibt bestehen.
Regression prüft Anzahl, Originalbezeichnung, Variante und Betrag. Rustfmt
ausgeführt. Der gemischte Datumsfall ist weiterhin bewusst fail-closed.

Vor dem zusätzlichen minimierten Datumsregressionstest: fünf Hofmann-Tests
bestanden; Workspace **720 bestanden, 0 fehlgeschlagen, 7 ignoriert**
(650 Ingestion plus 70 übrige Tests). Workspace enthält aktuelle lokale
Dossiers, kein Nachweis einer separaten sauberen Release-/Seed-Auswahl.
Nach Ergänzung des minimierten Datumsregressionstests: **6 Hofmann-Tests
bestanden, 0 fehlgeschlagen**; rustfmt erneut ausgeführt. Der neue Test
bestätigt die notwendige Schutzgrenze, nicht die noch fehlende Datumsreparatur.

Offene notwendige Lösung: zeilenbezogene explizite Veröffentlichungsdaten
bis durch den tatsächlichen `record()`-/Store-Pfad transportieren, einschließlich
Provenienz und Vorrang gegenüber Seiten-/beobachtetem Änderungsdatum.
Aktuell besitzt `ScrapedPrice` kein Datumsfeld und `record()` verwendet nur
`HandlerOutcome.published_at`. Zahlreiche vollständige Outcome-/Price-Literale
müssen bei einer Interface-Erweiterung geprüft werden. End-to-End-TempDB-Test
muss alte und neue Datumszeilen unabhängig nachweisen; erst dann Live-Scrape,
Workspace-Gate und Veröffentlichung. Kein Datumsraten, keine Teilimportlösung
als Abschluss und keine erfolgreiche Live-Reparatur behauptet.

## Parallelität und Veröffentlichung

Restliche Welle 4 konkret delegiert: A 16, B 20, C 18 noch offene Dossiers,
je eigener Bericht `research/wave-20261014-4-{a,b,c}.md`. Bereits sechs
Owner-geprüfte Fälle ausgeschlossen; keine Agentencommits/-deployments.
155 fremde geschützte Hashes zu Check-in-Beginn unverändert.
`gh auth status` weiterhin nicht angemeldet. Handleränderung bleibt
uncommitted, da vollständiger Live-/Datums-Gate noch nicht erfüllt.
Keine Produktionswrites, GEO-Änderung oder Schwächung von Sicherheitsgates.

## Nachtrag: vollständige lokale Zeilendatums-Korrektur

Der oben dokumentierte Teilstand ist überholt; Historie bewusst erhalten.
Erneuter DB-freier Live-Lauf zunächst tatsächlich Exit 1 mit gemischten
Zeilendatierungen. Regression
`mixed_dates_can_be_transportable_without_relabelling_prices` vor der
Korrektur ausgeführt: FAILED mit genau derselben Parserfehlermeldung.

### Lösung und Transportgrenze

- `hofmann_rastatt::DatedSnapshot` bindet akzeptierte Preise und explizite
  Zeilendaten in gleicher Reihenfolge; private Felder, keine unabhängig
  editierbare öffentliche Projektion. Datum wird erst zusammen mit einem
  tatsächlich übernommenen Preis angehängt, übersprungene Sorten verschieben
  keine Zuordnung. Zinn alt als Material zinn/Variante alt unterstützt.
- Der bestehende typisierte `RecordOutcome`-Pfad erhält eine Hofmann-Variante;
  Scheduler verwendet bereits `scrape_record_outcome`/`record_outcome`.
  Keine neue optionale Schnittstelle in rund 130 anderen Handlerkonstruktoren.
  MKM/MSG- und Legacy-Contracts erhalten, keine neue Storemigration/Abhängigkeit.
- Recorder prüft sämtliche Zeilendaten und Anzahl vor Writes, bindet die
  Variante an den Hofmann-Slug und verwendet **row > page > inferred**.
  Provenienz `extra_json.published_at_basis = row_stated`. Explizite alte
  Zeilendaten bleiben auch bei Preisänderung alt, statt das Beobachtungsdatum
  zu erfinden. Date-only-Updates bei unverändertem Betrag ebenfalls geprüft.
- Datumslos projiziertes Hofmann-Outcome kann nicht über generisches
  `record()` gespeichert werden. Der Legacy-Scraper lehnt weiterhin gemischte
  Datierungen ab; Scheduler und Live-CLI verwenden den vollständigen typisierten
  Pfad. Keine Schutzgrenze entfernt, kein Teilimport als Erfolg ausgegeben.
- DB-freies Beispiel `live_handlers` zeigt nun explizite Daten pro Preis und
  nutzt denselben Transport wie der Scheduler.

### Verifizierte Gates

| Gate | Tatsächlicher Nachweis |
|---|---|
| Red-capable Schleife | Live-CLI Exit 1 und neuer deterministischer Mischdatums-Test zunächst FAILED |
| Parser/Transport | Finale neun Hofmann-Tests in Workspace erfolgreich, einschließlich übersprungener unbekannter/auf-Anfrage-Zeilen zwischen akzeptierten Zeilen |
| Reale Speicherung | TempDB-Test `typed_recording_keeps_each_date_and_rejects_lossy_transport`: 23 Preise, je Originaldatum/row_stated/Quell-URL, Preisänderungs-Historie, Row-Vorrang vor widersprechendem Page-Datum und beobachteter Änderung, Date-only-Update, fehlerhafte/unvollständige Datierungen ohne Writes, falscher Slug/Legacy-Projektion blockiert |
| Finaler Workspace | cargo test --workspace: **724 bestanden, 0 Fehler, 7 ignoriert**, Log `/tmp/opencode/checkin-hofmann-rowdates-workspace.log`; davon Ingestion 654/0/7 |
| Beispiel-Regression | cargo test -p schrott-mcp-ingestion --example live_handlers: 1 bestanden, 0 Fehler |
| Aktueller Live-Scrape | cargo run -p schrott-mcp-ingestion --example live_handlers -- bw-rastatt-hofmann: Exit 0, HTTP200, 257165 Bytes, **23 Preise, keine Skips**, 20 Zeilen 14.10.2026, Starterbatterien/V2A/Elektromotoren weiter 30.09.2026; page published_at bewusst None |
| Format/Scope | rustfmt auf den drei Rustdateien, scoped diff check; keine Dossier- oder GEO-Änderung; ursprüngliche 155 geschützte Fremdpfade SHA-256 unverändert |
| Unabhängiges Review | Session ses_ec5965d54ffebIlkrlaQtsh2mb: keine Blocker; optionale Skip-/Datum-Zuordnungsregression und Dokumentation ergänzt, finale Tests danach erneut grün |

Finale Gesundheits-/Feedbackprüfung dieses Check-ins: Service aktiv,
HTTPS-Health ok, read-only 3881 Händler / 54 Materialien / 2426 Preise,
Feedback-IDs 7381–7390 unverändert. Keine systemd-Prioritätswarnungen oder
Geo-Logs; JSON-WARNs bestehende Hofmann-Datierungsablehnung, Münster-Preisleiter,
AMR HTTP502. Genau ein Worktree registriert (`main`), keine Nebenworktrees.

### Veröffentlichung bleibt separat

Nur drei Rustdateien und dieser Bericht für den lokalen Korrekturcommit
freigegeben. Aktuelle lokale Dossiers beeinflussen zwar den Test-Build,
werden aber weder mitcommittet noch als geprüfter Release-Seed ausgegeben.
Git-Authentifizierung war im vorherigen tatsächlichen Push nicht verfügbar;
nach Commit erneuten Push tatsächlich prüfen. Kein Redeploy und kein
Produktionsnachweis dieser Korrektur vor erfolgreicher Veröffentlichung.
Die Produktionswarnung ist bis zum tatsächlichen Rollout erwartbar.
Keine erzwungene Ingestion, keine Produktionsmutation, keine Rust-GEO-
Erweiterung oder temporäre Debug-Logs.
