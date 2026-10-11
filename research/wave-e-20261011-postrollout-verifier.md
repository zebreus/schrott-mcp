# Wave E — begrenzter Postrollout-Gate (11.10.2026)

Owner-Handoff, **keine Veröffentlichungsfreigabe**. Isolierter Worktree
`/root/Documents/schrott-mcp-wave-e-postrollout-20261011`, Branch
`verify/wave-e-postrollout-20261011`, Basis `db6c81e0`. Keine Mainwrites,
Commits, Pushes, Builds, Deployments, Ingestion oder Rust-/GEO-Änderungen.

## Artefakte und Benutzung

- `scripts/verify-wave-e-postrollout.py`: nur `public.db`, URI `mode=ro`,
  zusätzlich `PRAGMA query_only=ON`; fehlende DB wird nicht angelegt.
- `scripts/wave-e-20261011-expectations.json`: exakt60 Manifestslugs,
  157 Zielskalare, neun explizite Enrichment-Clearsets,60 SHA256 der vollständigen
  tatsächlich geseedeten UTF-8-Timeline-Notes inklusive Originalhistorie.
- `scripts/test_verify_wave_e_postrollout.py`: neue lokale TempDBs unter
  `/tmp/opencode`, keine Produktions-/Rehearsalwrites.

Erst **nach** koordiniertem Rollout mit dessen tatsächlichem UTC-Start ausführen:

```sh
python3 scripts/verify-wave-e-postrollout.py --since '<rollout-start-UTC>'
python3 -m unittest discover -s scripts -p 'test_verify_wave_e_postrollout.py' -v
```

Exit0 nur bei vollständig passendem Gate, sonst1 (fehlendes Pflichtargument2).
`--data-dir` erlaubt einen anderen DB-Ordner. Kein optionaler Erwartungsmanifest-
Override im CLI: Prüfung bleibt an die eingecheckte begrenzte Auswahl gebunden.
Keine internen Scheduler-/Fetchdaten erforderlich, kein forced scraping.

## Erwartungsbasis / Quellenprovenienz

Gelesen: `research/wave-e-20261011-owner.md`, `docs/trader-seed.md` sowie der
tatsächliche private Main-Seedvergleich
`/root/Documents/wave-e-main-seed-rehearsal-audit-20261011.json` (SHA256
`366d20b31c6ba60bfb8c3b6c0ad3f5fea3ca3031ca15d79c9b15bfac574a02ea`). Auswahl
`/root/Documents/wave-e-20261011-selection.json` (SHA256
`c1260bbc4c4ba1cc7c7f7e3e6fb09227a9a7ce9e935bbe75bb26dff533646f03`).
Auswahl-, Audit- und neue Manifest-Slugmengen tatsächlich gleich geprüft.

Zielwerte: alle60 `street`/`postcode`-Paare plus die freigegebenen tatsächlichen
Skalardeltas des Audits. Diese Werte aus dem privaten Seedklon gelesen und für
jeden Auditdelta zusätzlich gegen `private_seed` geprüft. Noteshashes stammen
ebenfalls aus dessen DB, nicht aus einer SQL-Seedsimulation. Die Offlinefixtures
extrahieren Timeline-Bullets entsprechend dem dokumentierten Format in
`crates/ingestion/build.rs` und prüfen deren Hash gegen die festen Erwartungen.
Das testet zusätzlich die Zuordnung zum eingebetteten Dossierstand, ersetzt
aber keinen echten Seedtest. Private Seedzeit aller60:
`2026-10-11T09:15:42.678078740+00:00`.

Dauerhafte Quellenprovenienz: Manifest verweist auf Ownerbericht und den
unveränderlichen Dossierstand `db6c81e0`. Die eigentlichen geprüften URLs,
Abrufgrenzen und Reviewer-/Ownerkorrekturen stehen in den vollständigen
Timelines unter `dossiers/<slug-prefix>/<slug>.md`; Stuhr liegt in `hb`, obwohl
Produktionsstate NI ist. Keine erneute Quellenrecherche, Vollquellengültigkeit
oder aktuelle Betriebs-/Vergütungsbestätigung behauptet.

## Vertrag und Grenzen

- Blank `street`/`postcode` überschreibt beim geänderten normalen Seed auch
  gefüllte Legacy-Adressen. Die neun Süd-Withdrawals sind deshalb echte leere
  Adressziele, nicht `clear_fields` und kein behaupteter Umzug/Schließungsbeleg.
- Blank Enrichment bleibt **außerhalb** expliziter `clear_fields` ein Keep.
  Solche Altwerte werden nicht pauschal auf Leerheit geprüft. Die neun
  Clearsets sind leere freigegebene Telefon-/Mail-/Zeit-/Beschreibungsskalare.
  Insbesondere darf aus leerer Reinhardt-/Seiler-Website kein genereller
  Website-Clearvertrag abgeleitet werden; keine neue Domain ist hier erwartet.
- Unveränderte Adressziele sind Reproduktionsziele des freigegebenen Seeds,
  keine neuen unabhängigen Adressfreigaben. Die übrige unveränderte Legacy-
  Stammdaten-/Enrichmentinventur liegt ausdrücklich außerhalb des Gates.
- Exakter Noteshash schützt die gesamte publizierte Timeline, auch historische
  und nachträglich eingeordnete Aussagen. Ein später absichtlich geänderter
  Dossierstand benötigt ein neues Review/Manifest; nicht durch Regex oder
  lockere Recherche-Datumsprüfung als dieselbe Welle akzeptieren.
- `updated_at >= --since` ist notwendig, aber allein kein Seed-/Buildnachweis:
  andere Writes können den Zeitstempel erhöhen, idempotente Seeds bewegen ihn
  nicht. Falscher/früher `--since` liefert keine Vorrolloutfreigabe. Deployment-
  journal und Ownerfreigabe bleiben separat erforderlich.
- GEO/first_seen wurden im Owner-Rehearsal unverändert beobachtet, hier weder
  gelesen noch neu validiert. Besonders withdrawn Adressen und TSRs neue
  Adresse bescheinigen keine passende Koordinate. Keine Price-/Source-/Handler-
  Vollabdeckung, kein Produktionssnapshot über zwei Datenbanken.

## Tatsächlich ausgeführte Checks

Read-only Läufe mit **Testschwelle** `2026-10-11T09:00:00Z` (kein tatsächlicher
Rolloutstart, keine Veröffentlichungsbehauptung):

| DB | Exit | Geprüft | Ergebnis |
|---|---:|---:|---|
| `/var/lib/schrott-mcp/public.db` |1|60|176 begrenzte Fehler:60 Zeitstempel,60 Notes,56 Zielskalare |
| `/root/schrott-mcp-wave-e-seed-rehearsal-20261011/public.db` |0|60|keine Abweichung |

Produktion somit erwartungsgemäß **vor** Veröffentlichung rot; tatsächlicher
privater Seed passend grün. Kein Produktionswrite. Acht UnitTests grün:
alle157 Zielwerte einzeln mutiert, leere Clears/Adresswithdrawals, Notes inkl.
Historie, fehlende/duplizierte Händler, since/fehlendes CLI-Argument, ungültiges
Manifest, ignorierte fremde Händler/GEO/Keep-Enrichment und fehlende DB.
Green-CLI lässt TempDB-Bytes unverändert; Writes scheitern auch nach Ausschalten
von `query_only` am URI-Read-only-Modus. Owner übernimmt Review/Integration.

Gesamte Python-Regression anschließend ausgeführt:
`python3 -B -m unittest discover -s scripts -p 'test_*.py' -v` — **41 Tests
bestanden** (33 bestehende +8 neue). `git diff --check` grün; Worktree enthält
nur die vier oben benannten neuen Dateien, keine Änderung bestehender Dateien.

## Owner-Gate und tatsächliche Main-Prüfung

Owner las Prüfer, vollständige Tests und diesen Bericht. Erwartungsmanifest
unabhängig gegen Auswahl, Audit und tatsächliche private Seed-DB geprüft:
exakte60-Slugmenge,157 Zielskalare,9 Clearsets; jedes Ziel stimmt zum gespeicherten
Wert, jeder Noteshash zur vollständigen echten Timeline. Erwartungsfelder sind
genau street/postcode plus freigegebene Auditdeltas, nicht beliebige zusätzliche
Legacy-Kontaktleersetzungen. Source-/GEO-/first_seen-/Build- und Schedulergates
bleiben ausdrücklich außerhalb dieses Prüfers.

Owner führte8 neue TempDB-Tests tatsächlich erfolgreich aus und wiederholte
beide read-only Kontrollen: Prod exit1/60geprüft/176Abweichungen; privater echter
Main-Seedklon exit0/60geprüft/keineAbweichung. Testschwelle09:00UTC ist weiterhin
**kein realer Rolloutstart**. Damit kein falscher Produktionsabschlussclaim.

Vier Dateien mittels getrenntem temporärem Git-Index als Reviewed-Tree-Artefakt
`45bd781f` zusammengeführt, Main `cherry-pick --no-commit`; Kandidatenbranch und
Index nicht verändert, bestehende Main-Read-only-Härtung bleibt erhalten.
Nach Zusammenführung gesamte tatsächliche Main-Python-Suite **43 bestanden**
(35 bestehende einschließlich Read-only-Härtung +8 neue), vor Commit geprüft.
Keine Rust-/Dossier-/Binaryänderung, kein neuer Build nötig. Authentifizierter
Push/Deployment und tatsächliche Produktionsprüfung nach echtem Cutoff bleiben
offen; lokale Integration ist kein Rolloutnachweis.
