# Tiefenrecherche-Welle — Owner-Gate, 22.10.2026

Die doppelt eingereichte 6h-Aufforderung wurde als eine Welle behandelt.
Drei 20er-Shards sind abgeschlossen; Scope und Slugs stehen in
[`wave-20261022-0600-scope.md`](wave-20261022-0600-scope.md). Die agentische
Recherche ist in den Dossiers dokumentiert. Der Owner-Gate hat die Änderungen
auf Quellhierarchie, Feld-Overwrites, Preis-/Gebührentrennung, GEO-Folgen,
Frontmatter, Timeline-/Quellenformat und Enum-/Seed-Validierung geprüft.

## Owner-Dispositionen

- `nw-essen-autoverwertung-chruscz`: neu vorgeschlagene Frontmatter-Kontakte
  zurückgehalten. Das Örtliche ist nach README ein Branchenbuch-Lead, kein
  zweiter qualifizierter Beleg; außerdem enthält dessen E-Mail einen
  Schreibfehler gegenüber dem Betreiber-Impressum. Beleglage, Büro-/Hof-
  Abweichung und offene Annahme bleiben nachvollziehbar in der Timeline.
- `st-halle-seik-automobil-recycling`: die Nissan-Partnerliste nennt
  Grenzstraße 43c, aber diese OEM-Liste ist keine der im README zugelassenen
  Quellen für einen Frontmatter-Overwrite. Wegen des Konflikts mit 43 und der
  nicht regulär TLS-authentisierten Betreiberwebsite bleibt 43 bis zur
  qualifizierten Gegenprüfung bestehen; kein neuer GEO-Wert.
- `st-aschersleben-schrott-und-metallhandelsgesellschaft-as`: aktuelle
  Betreiberanschrift Majoranweg 4, Kontakt und Website sind durch
  Betreiberimpressum und den HRB-kongruenten Registerspiegel belegt. Die
  Produktionszeile führt jedoch noch Walter-Kersten-Str. 15 mit GEO
  51.7574, 11.4608. Nominatim löste Majoranweg 4 nur auf die Straßenlinie,
  nicht auf eine Hausnummer; keine Ersatzkoordinate geraten. Eine Seed-
  Veröffentlichung dieser Adressänderung bleibt daher gesperrt, solange
  der alte Produktions-GEO-Wert nicht belastbar ersetzt werden kann.
- `th-groengottern-scholz-recycling-standort-groengottern`: die aktuelle
  Altfahrzeug-Bescheinigung und Betreiberangaben bestätigen die Anlage und
  Selbstanlieferung. Der Produktionspunkt lag laut Reverse-Abfrage an einem
  unbenannten Parkplatz statt auf der Anlage. Die Dossierkoordinaten wurden
  auf den OSM-Adresspunkt 51.1426713, 10.5574896 gesetzt; der Punkt liegt
  innerhalb der kartierten Scholz-Anlage. Die Timeline präzisiert die
  Genauigkeit: Adresspunkt innerhalb des Betriebsgeländes, kein bestätigter
  Eingang/Gate.
- `bw-mannheim-schrotthandel-wittmann`: `status` von `aktiv` auf `pruefung`
  gesetzt, da Betreiberidentität und Namens-/Kontaktkonflikt nicht unabhängig
  aufgelöst sind. Dies ist keine Schließungsbehauptung.
- `he-wettenberg-krofdorf-blum-marco-schrottentsorgung`: die leere
  Frontmatter-Straße war bereits vor dieser Welle vorhanden. Produktion hält
  weiterhin Höhenstr. 27A und GEO 50.6167, 8.65. Laut Seed-Vertrag würde ein
  späterer Seed die Straße leeren, aber das GEO behalten; das ist ein weiterer
  Grund, den gesamten Seed-Rollout nicht ungeprüft auszuführen.

Bei den übrigen Änderungen wurden keine unbelegten numerischen Ankaufspreise
als Kurse übernommen; Verkaufspreise und Gebühren wurden getrennt behandelt.
Keine Produktionsschreibzugriffe.

## Verifikation

- `CARGO_BUILD_JOBS=1 /root/.cargo/bin/cargo test -p schrott-mcp-ingestion seed`:
  **14 passed, 0 failed** (675 gefiltert). Der Seed-Codegen hat alle Dossiers
  erfolgreich verarbeitet.
- `git diff --check`: sauber.
- Statischer Scan der 60 Slugs: keine Website-Deep-Links, keine Gedankenstriche
  in Telefon-/E-Mail-Frontmatter, keine datumslose neue Timeline-Überschrift
  und keine neu hinzugefügte Recherchezeile ohne Quellenlink.
- Read-only SQL: 60/60 Auswahlzeilen weiterhin vorhanden und weiter
  recherchierbar; Produktionszählung unverändert bei 3.893 Händlern,
  83 Materialien und 2.683 aktuellen Preisen.

## Veröffentlichung ausgesetzt

Kein Commit, Push, Deploy oder Cleanup: `main` liegt 32 Commits vor
`origin/main`; der Arbeitsbaum umfasst aktuell 497 geänderte und 163
ungetrackte Einträge. Er enthält fremde bzw. bereits laufende Dossierarbeiten,
ungetrackte Rechercheartefakte sowie unabhängige Handleränderungen. Eine
selektive Wellenveröffentlichung lässt sich daraus nicht sicher abgrenzen.
Zusätzlich ist die Aschersleben-Adressänderung mit dem weiter gespeicherten
alten GEO nicht seed-sicher; auch der bereits vor dieser Welle geleerte
Wettenberg-Straßenwert würde beim Seed den Datenbanktext entfernen, das GEO
aber erhalten. Der Produktionsstand bleibt unangetastet. Nächster sicherer
Schritt: Arbeitsbaum fachlich abgrenzen und Aschersleben-/Wettenberg-GEO vor
jedem Seed separat auflösen; erst danach einen isolierten Rollout prüfen.
