# Owner-Check-in 14.10.2026 — AMR und gezielte Folgeprüfungen

## Gesundheits- und Feedbackprüfung

Anleitung `prompts/hourly-checkin.md` erneut gelesen und ausgeführt.
Produktionsdatenbanken ausschließlich SQLite `mode=ro`/`query_only` geöffnet.
Zehn neueste Feedbackeinträge IDs 7381–7390 unverändert; keine neue
Händlerbehauptung ungeprüft übernommen. Service aktiv, HTTPS-Health ok;
3881 Händler / 54 Materialien / 2426 aktuelle Preise. Keine systemd-
Prioritätswarnungen oder Geo-Logs. App-JSON separat geprüft: Hofmann
gemischte Zeilendaten (noch alte Produktion), Münster Preisleiter,
AMR HTTP502. Ein registrierter Worktree, ausschließlich main.

## AMR: tatsächliche Reproduktion statt vermuteter URL-Reparatur

DB-freies Kommando tatsächlich ausgeführt:
`cargo run -p schrott-mcp-ingestion --example live_handlers -- be-treptow-kopenick-amr-metalle-und-rohstoffe-amr-schrottpla`
(Cargo über `/root/.cargo/bin/cargo`). Exit 1, HTTP502 von
`http://www.amr-schrottplatz.de`.

Gegenproben für Quellstörung versus HTTPS-Wechsel: unabhängiger HTTP-
Abruf erneut 502; HTTPS-www-Startseite und echter Impressumspfad
`/impressum-und-datenschutz/` mit Zertifikatsprüfung beide
`certificate has expired`. Kein lesbarer Betreiberinhalt; präzisere
Ursache des entfernten 502 nicht festgestellt. HTTPS ist damit keine
freigegebene Ersatzquelle. Historischer Befund vom 08.10. bestätigt,
nicht als neue erfolgreiche Quellenprüfung ausgegeben.

Read-only JOIN current_prices/prices/traders ergibt vier AMR-Zeilen,
letzte Beobachtung `2026-10-07T10:29:14.898857953+00:00`. Diese können
veraltet sein. Neue Dossiernote und kuratierter Überblick kennzeichnen
diesen Unterschied ausdrücklich. Keine Rust-/URL-Änderung ohne nutzbaren
Beleg, keine TLS-Umgehung, Cachepreise oder Betriebsschließung behauptet.
Die entfernte Server-/Zertifikatsreparatur liegt nicht im Zugriff dieses Repos.

## Unabhängige offene Arbeit fortgeführt

Zwei konkrete Händler aus der aktuellen read-only traders-Abfrage ausgewählt;
Dossiers zuvor sauber, disjunkte report-only Rechercheaufträge gestartet:

- TSR Neumünster `sh-neumunster-tsr-recycling-nl-neumunster`: rechtliche
  Betreiberkette TSR Recycling versus TSR Deutschland, standortbezogene
  Zertifikate, Website-Status. Session `ses_ec57ec3e8ffeBujC3qk7QBHqGo`,
  Ergebnisziel `checkin-20261014-tsr-neumuenster-followup.md`.
- BERK Kevelaer `nw-kevelaer-berk-recycling`: Live-Register HRB18672,
  aktuelle branchbezogene Quellen/Zertifikate, offene Kontakt-/Adressfreigabe.
  Session `ses_ec57e9a0dffez6Aia3hqynGpqR`,
  Ergebnisziel `checkin-20261014-berk-kevelaer-followup.md`.

Recherchemethoden wählen die Agenten selbst. Kein DB-Zugriff, Dossier-/Rust-
oder GEO-Edit, Commit/Push/Deploy durch die Agenten. Ergebnisse und
Ownerabnahme noch ausstehend; nicht als erledigte Identitätsprüfung gewertet.
Diese gezielten Fortsetzungen ersetzen keine neue vollständige 60er-Welle.

## Gate und Veröffentlichung

AMR-Frontmatter bytegleich, alle ursprünglichen Timeline-Zeilen erhalten;
keine Enum-/GEO-/Kontaktüberschreibung. Neue Detail-URL ist der tatsächlich
abgerufene Impressumspfad, aber ausdrücklich Abrufgrenze, kein Sachbeleg.
Owner-Gate tatsächlich bestanden: Frontmatter bytegleich gegen HEAD,
komplette Alttimeline unverändert enthalten, neue Bracket-Note korrekt,
Scoped Diffcheck sauber und ursprüngliche 155 Schutzhashes unverändert.
Seedtests tatsächlich ausgeführt: 14 bestanden, 0 Fehler, 647 gefiltert.
Keine fremden Dossiers oder ungeprüften Agentenreports einschließen.

Hofmann-Korrektur bereits lokal in b875e1b0, noch nicht deployed; sein
erfolgreicher lokaler Live-Test ist kein Produktionsnachweis. Git-Push
zuletzt tatsächlich an fehlender HTTPS-Authentifizierung gescheitert;
Veröffentlichung nach diesem Commit erneut prüfen, keinen Redeploy vorwegnehmen.
