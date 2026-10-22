# Recherchewelle 22.10.2026 18:00 UTC: Owner-Gate

## Ergebnis

Die drei Shards sind fachlich und strukturell lokal abgenommen. Das ist keine
Produktionsveröffentlichung. Die Produktionsdatenbanken blieben read-only; es
gab keinen Seed-Lauf gegen Produktion und kein Deployment.

## Umfang und Quellenprüfung

- Das Scope-Manifest enthält 60 frisch aus der Produktionsauswahl gezogene
  Händler, aufgeteilt in drei bundesland-disjunkte Gruppen zu je 20. Alle
  geänderten Dossierpfade gehören zu diesem Manifest: 47 Dossiers wurden
  bearbeitet, 13 blieben mangels neuer belastbarer Evidenz unverändert.
- Berichte und Dossier-Diffs der Shards A, B und C wurden geprüft. Identitäts-,
  Standort- und Betreiberkonflikte bleiben offen, wo nur Leads oder
  Namensähnlichkeiten vorliegen. Keine Übernahme eines fremden Betriebs,
  keine Schließungsbehauptung aus einer Suchlücke und keine Koordinate aus einer
  ungeklärten Adresse.
- Die drei Frontmatter-Änderungsfälle wurden einzeln geprüft:
  - MCA Müritzer Container & Abbruch: aktueller Betreiberauftritt und Impressum
    nennen GmbH, Anschrift und Kontakte; das North-Data-Profil bestätigt den
    Rechtsträger und dieselbe Anschrift. Schrott/Metalle bleiben als
    Container-Entsorgung, nicht als vergüteter Ankauf, klassifiziert.
  - Danache Autoverwertung: Betreiberseite nennt den vorhandenen Namen und die
    Seedadresse. Nur `website` und `website_status` wurden ergänzt; Kontakt,
    Öffnungszeiten, Aktivstatus und GEO bleiben unverändert. Samstagzeiten
    widersprechen sich.
  - SEBO Dienstleistungen: die vorhandene Domain und ihr Impressum sind live;
    nur `website_status` wurde von `tot` auf `aktiv` geändert. Das belegt keine
    aktuelle Schrottannahme oder Auszahlung.
- Sonstige Frontmatter-Felder, Slugs, Statuswerte, Händlerarten, Koordinaten,
  `clear_fields`, Annahme- und Abhol-JSON blieben unverändert. Die frühere
  Recherchehistorie ist in allen geänderten Dossiers vollständig und geordnet
  erhalten.
- Alle 47 neuen Rechercheeinträge haben ein Datumspräfix und eine nachvoll-
  ziehbare Quellenangabe oder benennen ausdrücklich, dass kein neuer
  verifizierbarer Beleg vorlag. Ankauf, Verkauf und Gebühren wurden getrennt.
  Neue Emdashes wurden entfernt. Relative Links der drei Shard-Berichte sind
  gültig; neu hinzugefügte Quellen-URLs sind syntaktisch HTTP(S)-URLs.
- Gezielt erneut gelesene Primärseiten bestätigen die MCA-Felder, die
  Danache-Website und die wieder erreichbare SEBO-Domain. Der MCA-Kontaktpfad
  heißt `/kontakt`; eine abweichende `/contact`-URL lieferte 404 und wird nicht
  als Quelle verwendet. Der lokale Seed-Codegenerator akzeptierte den gesamten
  Dossierbestand.

## Technische und betriebliche Checks

- `CARGO_BUILD_JOBS=1 /root/.cargo/bin/cargo test -p schrott-mcp-ingestion
  seed`: 14 bestanden, 0 fehlgeschlagen, 678 gefiltert.
- Scope- und Historienprüfung: 60 ausgewählte Slugs, 47 geänderte Dossiers,
  13 unveränderte Dossiers, keine Änderungen außerhalb der Auswahl; alte
  Timeline-Bullets blieben in Reihenfolge erhalten. `git diff --check` ist
  sauber.
- Feedback-IDs 7390 bis 7381 sind weiterhin die neuesten; alle stammen vom
  13.10.2026. Kein neues Feedback zur Triage.
- `schrott-mcp.service` ist aktiv. Lokales und öffentliches `/health` liefern
  `{"ok":true}`. Keine systemd-Warnungen in den letzten sechs Stunden.
  Read-only Produktionszählung: 3.893 Händler, 83 Materialien und 2.683
  aktuelle Preise.
- Lauf #2454 (18:04 UTC) war `ok`, 77 Preise und 0 Händlerfehler. Die sechs
  App-WARN-Ereignisse im Prüfzeitraum betreffen MKM-Quellvertrag, Goldkurs-
  Referenzen, Cederbaum HTTP 503, fehlende Wahl-Sortenpreisliste, die bei
  Münster fail-closed abgewiesene verdächtige Preisleiter und AMR HTTP 502.
  Das sind bekannte sichere Ablehnungen bzw. externe Fehler; keine Änderung
  oder Produktionsbehebung wurde vorgetäuscht.
- Unter `/tmp/opencode/*.log` wurde kein Geocoding-Log gefunden.

## Veröffentlichung

Ein Redeploy bleibt gesperrt: `origin/main` ist 38 Commits vom deployten Stand
`ccd32e29` entfernt. Bereits vor dieser Welle umfasste der Deploy-to-main-Diff
1.088 Dateien und rund 25.969 zusätzliche Zeilen. Dieser breite
Produktions-Releaseumfang ist nicht durch das Gate dieser 60 Dossiers
abgedeckt. `gh auth status` meldet außerdem, dass kein GitHub-Host angemeldet
ist. Zum Zeitpunkt dieses Owner-Gates gab es keinen Push oder Redeploy; der
reguläre Git-Push wird separat nach dem lokalen Commit versucht. Die neuen
Dossiers sind nicht als produktiv behauptet. Der bestehende lokale
Reinwarth-Korrekturcommit `770c2a87` bleibt separat erhalten.
