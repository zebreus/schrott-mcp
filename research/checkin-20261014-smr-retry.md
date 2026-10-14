# Owner-Check-in 14.10.2026 — SMR-Nachprüfung

## Frische Bestandsprüfung

- `prompts/hourly-checkin.md` gelesen. Feedback read-only: jüngste zehn
  IDs 7381–7390 unverändert, keine neue Meldung seit dem vorigen Check-in.
- Public DB read-only/query_only: 3881 Händler, 54 Materialien, 2426
  aktuelle Preisdatensätze. Keine Produktionswrites/erzwungene Ingestion.
- systemd aktiv, keine Prioritätswarnungen binnen sechs Stunden.
  Öffentlicher HTTPS-Health tatsächlich `{"ok":true,"service":"schrott-mcp"}`.
- Nur main als Worktree, HEAD zu Beginn 0623093d. Keine Geo-Logs gefunden.
- Bekannte App-WARNs: Hofmann gemischte Zeilendaten (lokale Korrektur noch
  unveröffentlicht), Münster verdächtige Preisleiter fail-closed, AMR HTTP502.

## Neuer WARN und tatsächlicher Reproduktionsversuch

13:59:53.985898Z: SMR `bb-lubben-neuendorf-smr`,
https://smr-luebben.de/smr-preise/, `keine Preistabelle`.

Zweimal frisch den echten Handler ohne Datenbank ausführen:

```sh
/root/.cargo/bin/cargo run -p schrott-mcp-ingestion --example live_handlers -- bb-lubben-neuendorf-smr
target/debug/examples/live_handlers bb-lubben-neuendorf-smr
```

Beide Exit0, HTTP200, 19 EUR/kg-Preise, Altpapier ausdrücklich ausgelassen,
Kontakt aus Impressum gelesen. Antwortgrößen 848074/848131 Bytes. Kein
Veröffentlichungsdatum vorhanden (`published_at=None`), nicht erfunden.
Diese CLI gibt bei Scrape-Fehler Exit1 und ist ein direkter Fehlerindikator;
der konkrete Produktionsfehler ließ sich damit jetzt nicht reproduzieren.
Keine Ursachenfestlegung auf Timeout, Tunnel, HTML-Änderung oder Sperre ohne
die fehlgeschlagene Originalantwort. Kein spekulativer Parser-Fix, keine
unterdrückte Warnung, keine Altpreise als frische Beobachtung ausgeben.
Bei Wiederholung fehlgeschlagene Antwort diagnostisch erfassen; erst dann
gezielten deterministischen Repro/Regression bauen.

`cargo test -p schrott-mcp-ingestion traders::handlers::smr`:
5 bestanden, 0 fehlgeschlagen, darunter Ablehnung falscher Tabelle/Einheit.
Kein vollständiger Workspace-Lauf behauptet. Bestehende Current-DB zeigt
19 SMR-Zeilen; `current_prices.updated_at` reicht 28.09.–08.10.2026.
Dies ist der Current-Zeilenzeitstempel, kein Beweis des letzten erfolgreichen
Abrufs oder eines aktuellen Produktions-Scrapes. Live-Tests schreiben nichts.

## Owner-Gate und Publikation

Nur dieser technische Bericht geändert, keine Dossier-/Frontmatter-/Slug-/
GEO-Overrides; historische Dossiers und fremde Arbeit unberührt. Abschluss:
geschützte Hashes, leerer Index, diff-check und Seedtests prüfen; exakt diesen
Bericht committen und tatsächlichen Push versuchen. Kein Redeploy, bevor
geprüfter Stand erfolgreich veröffentlicht ist. Authentifizierung war zuletzt
mit fehlendem Git-Passwort blockiert, nicht durch Umgehung ersetzt.

Gates ausgeführt: 155 geschützte Pfade SHA256-unverändert, Index leer;
14 Seedtests bestanden, 0 fehlgeschlagen. Bericht ohne Frontmatter-Eingriff.
