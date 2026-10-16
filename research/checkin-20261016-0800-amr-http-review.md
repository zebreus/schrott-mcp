# AMR: HTTP-Review 16.10.2026, 08:00 UTC

## Ergebnis

Der Schedulerfehler ist unabhängig reproduzierbar: HTTP 502 bereits vor dem Preisparser. Kein belegter Codebug, keine sichere Ersatz-URL und kein Schließungsbeleg. Die genaue entfernte Gateway-/Upstream-Ursache bleibt unbekannt.

Scope: HEAD `5942a8ad`, anfangs 143 fremde offene Pfade. Nur dieser Bericht geschrieben; kein Dossier-/Code-/Produktionswrite, keine erzwungene Ingestion, kein Dienst-/Deploy-/Commit-Eingriff und keine Hofmann-Bearbeitung.

## Tatsächliche Befunde

- Read-only `journalctl -u schrott-mcp --since '2026-10-16 05:32:00 UTC' --until '2026-10-16 05:35:00 UTC' --no-pager -o cat`: um **05:33:29.743341Z** meldet `schrott_mcp_ingestion::traders::scheduler` für `be-treptow-kopenick-amr-metalle-und-rohstoffe-amr-schrottpla`: `scrape failed: response from http://www.amr-schrottplatz.de was not usable: HTTP 502`. Lauf 1833 endet mit einem Händlerfehler. Dies ist der installierte Scheduler, nicht nur eine alte Dossierangabe.
- Eigene GETs am **16.10.2026, 08:01:14–08:01:46 UTC**, normale TLS-Prüfung, Redirect-Follow mit maximal fünf Hops:

| Ziel | Frisches Ergebnis |
|---|---|
| `http://www.amr-schrottplatz.de/` | zweimal 502, `Server: nginx`, `Content-Length: 150`; keine Weiterleitung |
| `http://amr-schrottplatz.de/` | 502/nginx, 150 Bytes; keine Weiterleitung |
| `http://www.amr-schrottplatz.de/impressum-und-datenschutz/` | 502/nginx, 150 Bytes; keine Weiterleitung |
| `https://www.amr-schrottplatz.de/` | curl 60: `certificate has expired`; kein HTTP-Inhalt |
| `https://www.amr-schrottplatz.de/impressum-und-datenschutz/` | gleicher TLS-Fehler |
| `https://amr-schrottplatz.de/` | gleicher TLS-Fehler |

Der zweite HTTP-www-Abruf nennt `remote_ip=185.30.32.205`. Keine HTTP-/HTTPS-/ALL-/NO_PROXY-Umgebungsvariablen vorhanden; das schließt transparente Zwischenstellen nicht aus. Keine TLS- oder Sicherheitsguard-Umgehung. HTTPS-Redirects sind wegen der gescheiterten TLS-Prüfung nicht untersuchbar.

## Ausgeführtes rotes Repro

```sh
curl --fail --location --silent --show-error --max-redirs 5 \
  --connect-timeout 8 --max-time 20 --dump-header - \
  --write-out '\nstatus=%{http_code} final=%{url_effective} remote=%{remote_ip}\n' \
  http://www.amr-schrottplatz.de
```

Tatsächlich ausgeführt um 08:01:46 UTC: `HTTP/1.1 502 Bad Gateway`, `status=502 final=http://www.amr-schrottplatz.de/ remote=185.30.32.205`, Exit **22**. Der Exit prüft die HTTP-Nutzbarkeit, nicht die inhaltliche Richtigkeit einer späteren 200-Antwort.

Zusätzlich das bereits vorhandene DB-freie Entwicklungsbinary ausgeführt:

```text
target/debug/examples/live_handlers be-treptow-kopenick-amr-metalle-und-rohstoffe-amr-schrottpla
FAILED: response from http://www.amr-schrottplatz.de was not usable: HTTP 502
Exit 1
```

Kein Neubau; Binary-mtime 16.10.2026 03:04 laut `ls`, keine Behauptung einer frisch kompilierten HEAD-Verifikation. Der installierte Journalfehler und unabhängige curl-Abrufe tragen den Befund unabhängig davon.

## Handler, Identität und Grenzen

`crates/ingestion/src/traders/handlers/amr_schrottplatz.rs:20–22,37–39,61–62` nutzt genau HTTP-Startseite und HTTP-Impressum. `fetch_text` in `crates/ingestion/src/traders/mod.rs:184–208` weist Nicht-2xx vor Body-Auswertung zurück. Der Fehler heißt technisch `IngestError::Parse`, ist hier aber ausdrücklich die HTTP-Statusprüfung, kein bewiesener HTML-Parserdefekt. Ein Parserfix oder erfundener Regressionstest wäre nicht belegt.

Das vorhandene AMR-Dossier dokumentiert historisch AMR Metalle und Rohstoffe GmbH, Schnellerstr. 20E, 12439 Berlin, Tel. 030 53013220 und `kontakt@amr-schrottplatz.de`. Frisch gelesen am 16.10.2026 im Zeitfenster 08:01–08:03 UTC: [Umweltbüro Berlin-Brandenburg](https://www.ubb.de/htm/Wertstoffentsorgung/main.php?bl_id=1&w_id=364), Eintrag 18, nennt dieselbe Firma/Adresse/Kontakte und verlinkt `https://www.amr-schrottplatz.de/`. Das stützt die Domainzuordnung, belegt aber weder einen aktuellen Betreiberwechsel noch aktuelle Geschäftstätigkeit; das Verzeichnis ist kein Betreiberimpressum. Suchindex-Inhalte der AMR-Seite wurden nicht als Live-Beleg übernommen. Kein abweichender Betreiber oder neuer Domainumzug bestätigt.

Keine lesbaren aktuellen Betreiberseiten, keine aktuellen Preise, Zeiten oder Gebühren verifiziert. Der im Dossier genannte Preisstand 07.10.2026 wurde hier nicht erneut per DB geprüft. Keine Produktionsdatenbank geöffnet.

## Kleinste belegte Handlungsempfehlung

**Kein Code-/URL-/Statuswechsel.** Abrufstörung offenlassen und nächste reguläre Beobachtung abwarten; falls externe Behebung angestoßen werden soll, über einen zuvor verifizierten Betreiberkontakt HTTP-Gateway und abgelaufenes TLS-Zertifikat melden, ohne Kontaktaufnahme in diesem Auftrag. Erst nach normal validiertem Abruf von Startseite und Impressum eine URL-Änderung und den DB-freien Handler erneut prüfen. Weder TLS lockern noch alte Preise als frisch ausgeben oder aus 502 eine Schließung ableiten.
