# Owner-Gate: Recherchewelle 2026-10-12 (Shards A–C)

## Umfang und Review

- Die drei disjunkten Länder-Shards enthalten 20 Slugs je Shard: 60 eindeutige Dossiers insgesamt. Davon wurden 59 geändert; das bereits aktuelle Oberderdingen-Dossier ist der unveränderte 60. Ein read-only-Abgleich bestätigt für alle 60 vorhandene Produktionszeilen und die aktuelle Auswahlbedingung `website='' OR street=''`.
- Owner-Prüfung: alle Frontmatter-YAMLs, Slug-/Bundesland-Zuordnung, Trader-/Status-/Website-Enums, JSON-Felder, `clear_fields`, Koordinatenbereiche und Timeline-Historien geprüft. Alle bisherigen Timeline-Zeilen bleiben in Reihenfolge erhalten. Keine `lat`-/`lon`-Änderung. Die aktuellen Rechercheeinträge entsprechen dem README-Format; Webers `website` zeigt auf den Domain-Root, das Schroll-Profil bleibt Quellenlink; kein Em-Dash-Platzhalter in strukturierten Frontmatter-Feldern.
- Die zusammengeführte Owner-Gate-Kopie entspricht bytegenau den drei Shard-Änderungen. Die 60 ausgewählten Dossiers überschneiden sich nicht mit den 57 bereits im Root-`main` veränderten Dossiers.
- Owner-Korrekturen: Bausch Ravensburg erhält die standortspezifische Telefonnummer `+49 751 36322-9` (Zentrale bleibt als solche dokumentiert); Gold Inn widerruft den ungültigen gespeicherten Telefonwert `—` explizit via `clear_fields: 'phone'`; vier aktuelle 12.10.-Timeline-Marker wurden auf das README-Format normalisiert. Frumento-Kurse wurden um den direkt geprüften Snapshot 12.10.2026, 20:09:20 ergänzt; der ältere Snapshot bleibt Historie.

## Test und Produktionsgrenzen

- `git diff --check`: bestanden.
- `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/root/Documents/schrott-mcp/target /root/.cargo/bin/cargo test --offline --locked -p schrott-mcp-ingestion seed`: **14 bestanden, 0 fehlgeschlagen, 594 gefiltert**.
- Derselbe Build-Target mit `cargo test --offline --locked -p schrott-mcp-ingestion`: **602 bestanden, 0 fehlgeschlagen, 6 ignoriert**; ELNO-, AGH-, Saxonia-, Hofmann-, reGOLD- und Trapper-Regressionen alle grün.
- `cargo test --offline --locked --workspace`: **662 bestanden, 0 fehlgeschlagen, 6 ignoriert**. `cargo fmt --all -- --check`: bestanden. In dieser Welle wurde kein Rust-Code geändert.
- Produktionsdatenbanken wurden ausschließlich read-only abgefragt. Der Gold-Inn-Telefonwert ist in Produktion tatsächlich das ungültige `—`; der nächste Seed würde nur dieses Telefon leeren und den Status auf `pruefung` setzen.
- **Kein Seed-Rollout:** Bei Griesbaum wird die unbelegte Straße `Bundesstr. 13` aus dem Dossier entfernt. Produktion enthält derzeit noch diese Straße und GEO `49.6247, 9.6628`; laut Seed-Vertrag bleibt das GEO ohne neue Koordinate bestehen. Identität/Annahmestelle und Punkt sind ungeklärt. Auch die neu bestätigten Straßen bei Kaatsch, Bausch, HÖHL und Schaumaier würden vorhandene, in dieser Welle nicht verifizierte Produktionspunkte behalten. Keine DB-Schreiboperation, kein Geocode in Rust und keine Koordinaten erfunden. Vor Veröffentlichung sind diese Punkte unabhängig zu prüfen.

## Check-in / Release

- `schrott-mcp.service`: aktiv, `NRestarts=0`, `MemoryCurrent` rund 81 MB, Peak rund 109 MB; keine Journal-Warnungen der letzten sechs Stunden.
- Letztes Feedback weiterhin #6080–#6071 (09.10.2026); Produktionsbestand read-only: 3.881 Händler, 54 Materialien, 2.426 aktuelle Preise. Der Log-Stichworttreffer zu Geocoding betrifft vorhandene Workspace-Testlogs; kein aktiver Geocoding-Prozess oder Lauf gefunden.
- Reguläre Produktionsläufe belegen die fünf Handler: ELNO 36 Preise (16:38 UTC), AGH 8 (14:36), Hofmann Rastatt 23 (15:06), Saxonia 4 (18:23; sechs nicht explizite Verkaufs-/Verarbeitungskurse korrekt übersprungen), reGOLD 20 (18:38). Ingestion #1504 endete 18:38:54 UTC mit 156 Preisen und 0 Händlerfehlern. Die isolierten Branch-Commits sind nicht Vorfahren von `main`; alle fünf Handler sind dort registriert und durch anders gehashte spätere Commits vertreten. Die vier Nicht-ELNO-Diffs sind überwiegend `ScrapedPrice`-Borrowed-Variant-Anpassungen. Das ELNO-Worktree enthält noch `spawn_blocking`/`pdf-extract`; `main` und `origin/main` verwenden die begrenzte `prlimit`/`pdftotext`-Ausführung. Nicht den veralteten Worktree blind mergen.
- `main` lag vor dieser Wave 38 Commits vor `origin/main` und hat 57 fremde, bereits vorhandene Dossieränderungen. Der Owner integriert diese Wave nur lokal per Fast-Forward über die disjunkten, geprüften Pfade; Push/Redeploy bleiben gesperrt, da der Push die 38 Commits mitveröffentlichen würde und der Seed-Redeploy ungeprüfte GEO-Punkte übernähme. Keine fremden Änderungen verändert.

## Nächste Schritte

1. Die fünf übernommenen bzw. bestehenden Produktions-GEO-Punkte für Griesbaum, Kaatsch, Bausch, HÖHL und Schaumaier unabhängig gegen die belegten Standorte prüfen; keine DB-Manipulation.
2. Die fünf isolierten Branch-Tips einzeln als superseded/integriert gegen `main` dokumentieren; nur noch nicht enthaltene Semantik übernehmen. Die aktuelle Main-Implementierung ist durch grüne Workspace-Tests und Live-Logs abgesichert. Für ELNO sind `/usr/bin/prlimit` und `/usr/bin/pdftotext` vorhanden; der dokumentierte Prozess-/Zeit-/Output-Limit-Vertrag bleibt Deployment-Voraussetzung.
3. Remote-Ancestry und Release-Arbeitsbaum bereinigen, bevor irgendein Push oder Redeploy erfolgt. Produktionsdaten bleiben bis dahin read-only.
