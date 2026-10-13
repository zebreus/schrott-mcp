# Tiefenrecherche-Welle 2 — Owner-Gate, 13.10.2026

## Auswahl und Scope

Alle drei Recherche-Shards sind abgeschlossen: 20/20/20 Händler aus fünfzehn
Bundesländern, nach Bundesland disjunkt. Die Auswahl stammt aus `public.db`
mit `mode=ro` und dem Kriterium `website='' OR street=''`; Ausgangspunkt war
Main `0a58fa0d`. Poolgröße: 1.968; 79 Slugs aus der vorherigen Welle
ausgeschlossen. Die vollständige Auswahl und Ausgangs-SHA256s stehen außerhalb
des Repos in
`/root/Documents/worktree-archive-20261013/wave-20261013-wave2-selection.json`.

Der erneute Read-only-Abgleich fand alle 60 Slugs in der Produktionsdatenbank;
alle erfüllen weiterhin das Auswahlkriterium. Die Agenten bearbeiteten ihre
jeweils 20 Dossiers und Berichte. Es wurden keine Produktionsdatenbanken
beschrieben und keine Koordinaten ergänzt.

## Owner-Gate

- **Overwrite / Scope:** 155 vorbestehende fremde Arbeitsdateien stimmen
  weiterhin bytegenau mit ihren Manifest-SHA256s überein. Geänderte Pfade sind
  genau diese 155 geschützten Pfade, die 60 ausgewählten Dossiers und drei
  Shard-Berichte; keine unerwarteten oder fehlenden Pfade.
- **Dossiers:** 60/60 ausgewählte Dossiers wurden fortgeschrieben. Alle Slugs
  und bisherigen Timeline-Einträge blieben erhalten; jedes Dossier hat eine
  datierte Notiz vom 13.10.2026. Es gibt keine Koordinatenänderungen,
  Emdash-Platzhalter in Telefon/E-Mail oder verbliebene Website-Deep-Links in
  den 60 Frontmatter-Feldern.
- **Quellen / Identität:** Preise sind in den Berichten nach Ankauf, Verkauf
  und Gebühren getrennt. Platform- und unzugeordnete Preise werden nicht als
  lokale Händlerkurse ausgegeben. Owner hat Resebecks aktuelle Northeimer
  Standort-/Annahmedaten und die historische Gröschel-Verknüpfung,
  MOHRs Betreiber-Startseite sowie Willeführ-WHG und den HRA-3535-Beleg direkt
  gegengeprüft. Die offenen Identitäts-, Betriebs- und Annahmefragen bleiben
  ausdrücklich offen; keine GEO-Pins wurden abgeleitet.
- **Form / Tests:** `git diff --check` und ein Zeilenend-Whitespace- und
  Notenformat-Audit über alle 60 Dossiers und drei Shard-Berichte sind sauber.
  `/root/.cargo/bin/cargo fmt --all -- --check` ist sauber.
  `/root/.cargo/bin/cargo test -p schrott-mcp-ingestion seed`: **14 bestanden,
  0 fehlgeschlagen**. Die Tests validieren Seed-Parsing/Enums und
  Idempotenzverhalten; es wurden keine Rust-Dateien geändert.

## Check-in und Veröffentlichungsgrenze

Stündlicher Check-in: Service `active`; keine systemd-Warnungen der letzten
sechs Stunden. App-WARNs: Münster-Preisleiter fail-closed, AMR HTTP 502 und
Huth ohne Preistabelle. Produktionsdatenbank ausschließlich read-only geprüft:
3.881 Händler, 54 Materialien, 2.426 aktuelle Preise. Feedback IDs 7381–7390
sind Audit-7-Leads; sie wurden hier nicht verändert, da der vorherige
Owner-Check-in-Bericht eine parallele Feedback-Triage dokumentiert und diese
Arbeit nicht gestört werden soll.

Die Dossieränderungen sind Seed-Eingaben. Laut `README.md` erfordert ihre
Übernahme in Produktion einen Neubau und ein Redeployment; der Boot-Seed
schreibt die Änderungen in die Produktionsdatenbank. Wegen der verbindlichen
Read-only-Grenze wurden **weder Redeployment noch Seed-Schreibvorgang**
ausgeführt. Aktuelle Health-/Zähl-Spotchecks belegen daher nicht, dass die
neuen Dossierwerte bereits in Produktion liegen.

Der aktuelle lokale Main enthält vor dieser Welle vier bereits vorhandene,
nicht zu dieser Welle gehörende Commits. GitHub-CLI meldet keine Anmeldung;
Push-Authentifizierung ist damit aktuell nicht nachgewiesen. Vorhandene
Commits wurden nicht umgeschrieben. Dieser Owner-Gate-Bericht und die drei
Shard-Berichte gehören zum einzelnen Wave-Commit; Push-Dry-Run und
Produktionsrollout bleiben getrennte Schritte.
