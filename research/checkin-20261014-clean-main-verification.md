# Owner-Check-in: isolierte Main-Verifikation, 14.10.2026

## Gesundheits- und Feedbackprüfung

- `schrott-mcp.service`: aktiv; öffentlicher HTTPS-Endpunkt
  `https://schrottindex.de/health`: `{"ok":true,"service":"schrott-mcp"}`.
- Sechs Stunden systemd-Warnings: keine. Separater JSON-WARN-Filter zeigt
  unverändert Münster (verdächtige Preisleiter zurückgewiesen), AMR (HTTP 502)
  und Huth (alte Produktionsversion findet keine Preistabelle).
- Produktionsstatistik ausschließlich read-only: 3881 Händler, 54 Materialien,
  2426 aktuelle Preise. Feedback 7381–7390 unverändert; vorhandene Triageberichte
  bleiben maßgeblich. Keine Feedbackquittierung oder sonstigen Produktionswrites.
- Keine Geo-Logs gefunden. `/`: 75 % belegt, 18 GB frei; `/tmp`: 90 % belegt,
  408 MB frei. Keine ungeprüfte Bereinigung durchgeführt.

## Sauberer committed Stand statt fremder Arbeitsbaum

Commit `ae3a6b05` mit `git archive` nach
`/root/Documents/owner-verification/checkin-ae3a6b05` extrahiert. Kein neuer
Branch/Worktree. Gemeinsamer Cargo-Targetcache benutzt, Quellen und Dossiers
aber ausschließlich aus diesem Snapshot kompiliert.

- `cargo test -p schrott-mcp-ingestion seed`: **14 bestanden, 0 fehlgeschlagen**;
  `/tmp/opencode/checkin-20261014-clean-main-seed.log`.
- `cargo test --workspace`: **719 bestanden, 0 fehlgeschlagen, 7 ignoriert**;
  `/tmp/opencode/checkin-20261014-clean-main-workspace.log`.
- Alle 155 geschützten fremden Datei-Hashes unverändert. Diese Änderungen
  sind nicht Teil des isolierten Prüfsnapshots und werden nicht committed.

## Limekon-Liveprobe: intermittierender Fehler bleibt offen

Mehrere Aufrufe des DB-freien `live_handlers` aus dem sauberen Snapshot
scheiterten beim Impressum mit `error sending request`. Regulärer curl-Abruf
derselben Impressum-URL ergab HTTP 200. Zwischenzeitlich gelangen mehrere
Handleraufrufe mit **HTTP 200, 15 Preisen und 12 Skips**. Somit kein dauerhaft
kaputter Parser belegt, aber auch **keine verlässliche Live-Stabilität** bewiesen.
Keine Ursache (Keepalive, Netz, Quelle) ohne Fehlerkette als bestätigt behauptet.

Die Debug-Ausgabe wurde nur vorübergehend im eigenen Prüfsnapshot erweitert
und danach exakt zurückgenommen; kein Produktivcode geändert. Belege:
`checkin-20261014-clean-main-limekon-live.log`, `*-live-retry.log`,
`checkin-20261014-limekon-debug.log`, `*-debug-second.log`,
`checkin-20261014-clean-main-limekon-final.log` unter `/tmp/opencode`.

Wichtig: Das vorhandene Beispiel meldet `FAILED:` im Text, beendet sich aber
trotzdem mit Exitcode 0. Der Owner verwendete zusätzlich eine Ausgabeassertion
auf HTTP 200, 15 Preise, 12 Skips und Abwesenheit von `FAILED:`; diese schlug
beim letzten unveränderten Snapshot-Aufruf korrekt fehl. Ein grüner CLI-Exit
allein ist deshalb kein Live-Nachweis. Keine TLS-Umgehung, kein erzwungener Ingest.

## Abgeschlossene Aufnahmeprüfungen und Releasegrenze

Beide disjunkten Hintergrundprüfungen abgeschlossen und ihre vollständigen
Berichte vom Owner gelesen:

- [Neckartilda](feedback-20261014-neckartilda-admission.md), Feedback 7380:
  **keine aktive Neuaufnahme**. Alter HRB-/Geschäftsführerstand im Impressum,
  Registerwechsel nach Berlin 2020 und Erlöschen/Löschung 08.01.2025. Owner hat
  das NorthData-Original-HTML separat abgerufen und `title="erloschen"`,
  HRB 31272 sowie das Löschungsdatum in den eingebetteten Historiedaten bestätigt.
  Das beweist keine Platzschließung oder heutige Nachfolge; beide Standorte
  bleiben separat zu klären. Bestehende St.-Ingbert-Zeile nicht verschoben/gelöscht.
- [elorec/Metallcontor](feedback-20261014-elorec-admission.md), Feedback 7378:
  vergüteter Ankauf vorhanden; Owner hat Metall-Leistungsseite und Impressum
  selbst gelesen (HRB 21657, Alte Bottroper Straße 11–13). Metallcontor HRB 19234
  ist ein anderer gelöschter Rechtsträger, kein belegter früherer elorec-Name.
  buyRec ist derselbe Betreiberkanal, kein zweiter Standort. Expliziter aktueller
  Registerstatus und widersprüchliche Annahmezeiten bleiben offen; deshalb
  **noch keine seedwirksame Aufnahme**. Feedback- und Preisgates unverändert.

Nur Researchberichte integriert, keine Dossiers/Enums/Kontaktfelder geändert;
Overwrite-/Deep-Link-/Emdash-Dossiergate ohne neue Feldänderungen. Recherche-
historie und alle Original-Feedbackbezüge erhalten. Keine Aggregatoren als
Aufnahmebeweis gezählt. Beide Berichte sind Prüfbelege, nicht aktive Händlerseeds.

`gh auth status`: keine GitHub-Anmeldung. Push und Redeploy beider Binaries
bleiben offen; die Gesundheitsprüfung belegt nur das bestehende Release.
Dieser Check-in veröffentlicht keine neuen Händler oder Preise in Produktion.
