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

## Koordinaten-Owner-Follow-up 2026-10-12

- Die vier bestätigten Betreiberadressen wurden direkt erneut auf den Betreiberseiten geprüft und jeweils mit der adressbezogenen OpenStreetMap/Nominatim-Kartierung abgeglichen. Die bisherigen Produktionspunkte lagen 1,1 km (Kaatsch), 2,0 km (Bausch), 2,5 km (HÖHL) bzw. 2,5 km (Schaumaier) vom jeweiligen Gebäude-/Firmenpunkt entfernt. Dossiers enthalten jetzt die WGS84-Punkte aus den OSM-Objekten und dokumentieren Quellen, Abrufdatum und Genauigkeit; die Punkte bezeichnen Gebäude-/Adress- bzw. Firmenlage, nicht verifizierte Eingangstüren.
- Griesbaum bleibt ungeklärt: Der Produktionspunkt `49.6247, 9.6628` liegt rund 3,5 km vom OSM-Gebäudepunkt der nicht belastbar Roland Griesbaum zugeordneten `Bundesstraße 13` entfernt. Die Primärquelle verortet den Betreiber in Boxberg/Unterschüpf, nicht als feste Annahmestelle in Distelhausen. Keine neue Koordinate gesetzt. `docs/trader-seed.md` erlaubt kein explizites Löschen bestehender GEO-Werte; die geleerte Adresse wird die Produktionskoordinate daher nicht entfernen. Identitätsklärung bzw. sichere GEO-Entzugssemantik bleibt Release-Blocker.
- Nominatim-Abfragen: Kaatsch `48.7153089, 9.4058424` (OSM-Weg 254687352); Bausch `47.7928391, 9.6111644` (193316231); HÖHL `49.3008874, 8.4467507` (287560070); Schaumaier `47.8882515, 12.6271388` (365190932). Abruf 12.10.2026; Daten © OpenStreetMap-Mitwirkende, ODbL 1.0. Betreiberseitige Adressnachweise: https://kaatsch.de/oeffnungszeiten ; https://www.bausch-entsorgung.de/standorte/wertstoffhof-ravensburg/ ; https://container-speyer.de/pages/wertstoffhof-in-speyer ; https://www.schaumaier.de/standorte/recyclingzentrum-traunstein .
