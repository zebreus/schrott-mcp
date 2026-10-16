# Wave-E-Fixturekorrektur – 16.10.2026, Folgeauftrag 03:00

**Umgesetzt, zur Owner-Diffprüfung bereit.** Historische Tests verwenden jetzt eine statische, revisionsgebundene Offline-Fixture statt aktueller Dossiers. Historisches Manifest und Produktionsverifier unverändert. Keine Commits, Builds oder Produktionszugriffe.

## Enger Änderungsumfang

1. `scripts/test_verify_wave_e_postrollout.py`: aktuelle Dossierextraktion entfernt; validierender Fixtureloader und drei zusätzliche Integritäts-/Scope-/Offlineprüfungen. Die acht bisherigen Verifiertests bleiben erhalten und verwenden nun historische Notes.
2. `scripts/fixtures/wave-e-20261011-notes.json`: 60 vollständige historische Timeline-Strings mit Slug, Formatversion und Vollrevision **`db6c81e048458b12a0d9fd1aefde6fbcb7320a40`**. Kein Produktionssnapshot und keine aktuellen Dossierinhalte.
3. Dieser angeforderte Bericht.

Die bereits offenen Owneränderungen an `scripts/verify-handler-release.py` und `scripts/test_verify_handler_release.py` wurden nicht bearbeitet. Kein Staging. Alle 143 geschützten offenen Dateien frisch gegen `/tmp/opencode/checkin-20261016-013049-baseline.json` geprüft: **keine Hashabweichung**.

## Herkunft und Integrität

Die neue statische JSON-Datei wurde einmalig aus `git show <Vollrevision>:dossiers/<Bundesland>/<Slug>.md` für exakt die 60 Slugs des unveränderten historischen Manifests exportiert. Normalisierung wie dokumentiert: Timeline-Bullets, Fortsetzungszeilen anhängen, Whitespace kollabieren, mit ` | ` verbinden. **Vor Ausgabe wurde jeder der 60 Strings gegen seinen vorhandenen Manifest-SHA256 geprüft; alle stimmen überein.** Keine Erwartungs-SHA wurde geändert oder aus aktuellen Dossiers ersetzt.

Das einmalige Exporthilfsprogramm liegt außerhalb des Repos unter `/tmp/opencode/generate-wave-e-historical-notes.py`; es ist weder Testlaufzeitabhängigkeit noch Integrationsumfang. Die Fixture ist lesbares JSON, nicht komprimiert oder ausführbar.

Fixture: **145.354 Bytes**, SHA256:

```text
d6e92dce31ea7e4c61728aaeee9daf73f55711333c4268742e2338a9713e42e3
```

Der Loader verlangt exakt 60 eindeutige, mit dem Manifest identische Slugs, passende Vollrevision/Manifestrevision und gültige Notes-Digests für alle Einträge. Zusätzliche Felder je Händler werden nicht still akzeptiert. Negative Tests decken falsche Version/Revision, fehlende/zusätzliche/doppelte/fremde Slugs, Zusatzfelder sowie NULL/leere/manipulierte Notes ab. Eine weitere Prüfung erlaubt beim Laden nur Fixture und Manifest als Dateien und verbietet Subprozessaufrufe.

## Tatsächliche Tests

Vorherige rote Reproduktion ist im unmittelbar vorausgehenden `research/checkin-20261016-0300-verifier-review.md` dokumentiert: Hauptrepo **28 bestanden/8 fehlgeschlagen**, sauberes HEAD-Archiv **26 bestanden/8 fehlgeschlagen**. Alle acht Fehler entstanden vor den Testkörpern beim historischen Seiler-Hashvergleich. Keine neue rote Wiederholung oder neue Ursache behauptet.

Nach Änderung frisch ausgeführt:

| Prüfung | Ergebnis |
| --- | --- |
| `python3 -B -m unittest discover -s scripts -p 'test_verify_wave_e_postrollout.py'` | **11 bestanden**, Exit 0 |
| `python3 -B -m unittest discover -s scripts -p 'test_verify_*.py'` | **39 bestanden**, Exit 0 |
| Wave-E-Suite in temporärem Verzeichnis mit ausschließlich vier kopierten Dateien: Test, Produktionsverifier, historisches Manifest, statische Fixture; **ohne `.git` und ohne `dossiers/`** | **11 bestanden**, Exit 0 |
| `git diff --check` | ohne Befund |

Die 39 Tests umfassen die acht unverändert übernommenen Owner-Handlerverifiertests. Fixture-Datenbanken und Negativmutationen lagen ausschließlich in temporären Testverzeichnissen. Keine Git-Abhängigkeit bei der Testausführung; Git wurde nur beim einmaligen historischen Export genutzt.

Unveränderte historische Dateien, nach Abschluss erfasst:

```text
scripts/wave-e-20261011-expectations.json
871453ff9f2b54e262a93212c83e42788b29a82a6e2b05b1876300d74768787f
scripts/verify-wave-e-postrollout.py
d2e902cb457bbcae347b0e767303f5ac0ef720507d37a1f5cfa83fae07b9ae54
```

`git diff --name-only` für genau diese beiden Dateien ist leer. Die Historien-/Integritätskontrolle ist nicht durch Abschwächen der Assertions grün geworden.

## Abnahmegrenze

**Die historische Testsuite ist repariert; das historische Produktionsmanifest wird dadurch nicht aktuell.** Die zuvor gefundenen 23 Timeline- und drei skalaren Abweichungen gegenüber HEAD bleiben eine separate aktuelle Releasegate-Aufgabe. Dieser Fix importiert keine neuen Erwartungen, ändert keine Händlerdaten und behauptet keine neue Produktionsabnahme. Owner soll den tatsächlichen Testdiff und die neue Fixture vor Integration prüfen.
