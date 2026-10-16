# Unabhängiger Verifier-Review – 16.10.2026, Folgeauftrag 03:00

## Ergebnis und Empfehlung

**Die enge Handler-Verifierkorrektur ist sachlich richtig und freigabefähig. Die acht Wave-E-Fehler sind unabhängig davon im sauberen HEAD reproduzierbar.** Ursache ist die Vermischung eines eingefrorenen Veröffentlichungsmanifests vom 11.10. mit veränderlichen aktuellen Dossiers in der Testfixture. Als Gate für den Release `81a64832` ist dieses historische Manifest tatsächlich veraltet – nicht nur bei einer SHA, sondern auch bei drei strukturierten Zielwerten.

**Empfehlung:** Handlerkorrektur separat übernehmen. Historisches Wave-E-Manifest unverändert als historischen Vertrag erhalten; dessen Tests auf überprüfte, revisionsgebundene historische Fixtures umstellen. Für die aktuelle Veröffentlichung ein explizites neues, an den Release gebundenes Erwartungsmanifest mit geprüftem skalarem Delta und tatsächlichem isoliertem Seednachweis verwenden. **Keine pauschale SHA-Ersetzung, kein Löschen der Hashprüfung, keine Rücknahme neuer belegter Dossierwerte, nur um das alte Gate grün zu bekommen.** Bis diese Trennung umgesetzt ist, keinen grünen Python-Gesamtlauf und keine bestandene aktuelle Wave-E-Produktionsabnahme behaupten.

Die vom Owner inzwischen gemeldeten 726 Rusttests, zwei Liveprüfungen, grüner Releasebuild und Remote-Hashgleichheit sind der neue Ausgangsstand. Die früheren Auth-/Buildvorbehalte dieses Reviewers werden hier **nicht** als fortbestehende Blocker wiederholt. Der vorliegende Befund betrifft ein Prüfwerkzeug und dessen Fixtures, nicht einen nachgewiesenen Defekt des neuen Rustartefakts.

## Frische Reproduktion ohne fremde Dossiers

Alle folgenden Befehle wurden tatsächlich ausgeführt; keine Builds oder Produktionszugriffe. Die Python-Suites schreiben ausschließlich ihre vorgesehenen temporären Fixture-Datenbanken unter `/tmp/opencode`. `-B` und `sys.dont_write_bytecode` verhindern Bytecode-Dateien. Im sauberen Archiv wurden keine Quelldateien editiert.

Vor der Auswertung **sämtliche 4.400 versionierten Dateien** des vorhandenen Archivs `/root/Documents/checkin-20261016-0300-release-1wz2obd5` bytegenau gegen `git show 81a64832:<Pfad>` verglichen: **null Abweichungen**. Die dortigen unveränderten Skripte enthalten bewusst noch nicht die zwei offenen Handlerkorrekturen aus dem Hauptrepo.

| Ausführung | Tatsächliches Ergebnis |
| --- | --- |
| Hauptrepo: `python3 -B -m unittest discover -s scripts -p 'test_verify_handler_release.py'` | **8 Tests bestanden**, Exit 0 |
| Hauptrepo: `python3 -B -m unittest discover -s scripts -p 'test_verify_*.py'` | **36 Tests: 28 bestanden, 8 fehlgeschlagen**, Exit 1 |
| Sauberes HEAD-Archiv: derselbe Gesamtlauf | **34 Tests: 26 bestanden, 8 fehlgeschlagen**, Exit 1 |

Die Differenz von zwei Tests entsteht durch die neu hinzugefügten negativen Saxonia-/Hofmannfälle. In beiden Gesamtläufen scheitern dieselben acht Wave-E-Testmethoden bereits in `test_verify_wave_e_postrollout.py:53`, also in `setUp`, **bevor deren eigentliche Testkörper ausgeführt werden**. Das sind acht Wiederholungen derselben ersten Fixture-Inkonsistenz, nicht acht nachgewiesene Produktionsfehler.

Erster betroffener Datensatz im Manifest: `by-peienberg-seiler` (zweiter Eintrag). Gelesene Timeline-SHA:

```text
aktuelle/saubere Notes: 039c1b5410862a4840c21f6b86c2e272274c49002fc3442f52258e25cdd098d4
Manifest erwartet:     f702a19891c05a70a007eb98e39af3eec5baecf362b02d6ac9ffcb47f62d7d1e
```

Damit ist eine Ursache durch die 143 fremden offenen Dateien ausgeschlossen.

## Review der zwei offenen Handlerdateien

Geprüft wurde der tatsächliche `git diff` für `scripts/verify-handler-release.py` und `scripts/test_verify_handler_release.py`; beide wurden nicht verändert.

- **Saxonia:** Der Verifier erwartet jetzt `(4, "approx", True, 6)` statt `exact`. Seine unveränderte zentrale Unsicherheitsprüfung fordert damit auch **confidence 0.8**. Das entspricht dem committeten Handler `saxonia.rs:184–187`, der die nicht bindenden Richtwerte ausdrücklich als `approx`/`Some(0.8)` ausgibt. Kein sachwidriges Herabsetzen eines Gates.
- **Hofmann:** Datumsbasis jetzt für diesen Slug zwingend `row_stated`, für andere datierte Handler weiter `page_stated`. Das passt zur bestehenden Rust-Recordingprüfung in `hofmann_rastatt.rs:405–421`, die je Preis dessen eigenes Datum und den Marker `row_stated` prüft. Weder `observed_price_change` noch ein beliebiger Marker wird als Ersatz akzeptiert.
- **Tests:** Fixture stellt Saxonia ausdrücklich auf `approx` und Hofmann auf `row_stated`; neue negative Tests setzen Saxonia auf altes `exact/1.0` beziehungsweise Hofmann auf `page_stated` und verlangen die konkreten Fehler. Alle acht Fälle inklusive Read-only-/Nichtanlegeprüfungen frisch grün. Die übrigen Fixtureparameter bleiben teilweise aus `HANDLERS` abgeleitet; das ist keine unabhängige Vollprüfung aller Handlerverträge, aber kein Blocker der beiden eng begründeten Korrekturen.
- Keine Erweiterung des DB-Schreibumfangs oder der Produktionsberechtigungen. Der Verifier bleibt URI-`mode=ro` plus `query_only=ON`.

## Historischer Nachweis statt bloßer Hash-Neuberechnung

`scripts/wave-e-20261011-expectations.json` benennt ausdrücklich:

- `dossier_revision: db6c81e0`;
- den Ownerreview `research/wave-e-20261011-owner.md`;
- Erwartungsbasis: tatsächliches `seed_metadata` in einer privaten Produktionssicherung vom **11.10.2026, 09:15:42 UTC**, mit Auditdigest;
- vollständige historische Timeline-Hashes und damals geprüfte skalare Zielwerte für 60 Slugs.

Frisch alle **60** historischen Dossiers mit `git show db6c81e0:<Pfad>` gelesen und mit der dokumentierten Timeline-Normalisierung ausgewertet: **alle 60 alten Notes-Hashes stimmen exakt mit dem Manifest überein**. Keine historischen Inhalte fehlen als zusammenhängender Notes-String in den heutigen Timelines. Der alte Manifestinhalt ist damit nicht willkürlich falsch; die damals behauptete private Seedrehearsal wurde hier allerdings nicht erneut durchgeführt.

Für Seiler ist die Entstehung konkret belegbar:

| Gitstand | Timeline-SHA |
| --- | --- |
| `db6c81e0` | `f702a198…f62d7d1e` – exakt Manifest |
| `fbfd06c0` | `039c1b54…cdd098d4` |
| `81a64832` | `039c1b54…cdd098d4` |

`git log` ordnet die Ergänzung dem Commit **`3a930675`**, „research: verify 60 fresh trader dossiers for October 14 wave“, zu. `git diff db6c81e0 HEAD -- dossiers/by/by-peienberg-seiler.md` zeigt eine zusätzliche Recherche vom 14.10. über den neuen Creditreform-Domainhinweis, mit ausdrücklich erhaltenem 403-/Identitätsvorbehalt; keine Frontmatteränderung. `git diff fbfd06c0 HEAD` ist für dieses Dossier leer. **Der erste Fehler existiert also schon im zuvor vorbereiteten Release und entstand nicht durch die jüngste 60er-Welle.**

## Umfang der Veralterung: nicht nur Seiler

Vergleich aller 60 Manifestzeilen mit den vorhandenen echten HEAD-Codegen-Ausgaben unter `/root/Documents/checkin-20261016-0130-isolated-7qa14e0l/owner-after/seed_traders`: **23 Notes-Hashes abweichend, drei skalare Zielabweichungen in zwei Dossiers**. Die Timeline-Extraktion aus dem sauberen aktuellen Archiv stimmt für alle 60 Zeilen mit diesen Generator-Notes überein. Es wurde kein Generator neu gebaut/ausgeführt.

Die drei skalaren Widersprüche:

| Slug | historisches Manifest | committeter aktueller Seed | fachliche Änderungsbasis im Diff |
| --- | --- | --- | --- |
| `th-zedlitz-07557-entsorgung-und-recycling-bernd-zauritz` | `postcode: ""` | `07557` | `62f42ce7`: vorhandenes `Zedlitz 07557` syntaktisch in Ort/PLZ getrennt, keine neue Standortbehauptung |
| `sn-kirchberg-08107-containerdienst-kaminski-sohn` | `postcode: ""` | `08107` | `8e07dda7`: Betreiberadresse gegen kommunalen Gewerbeeintrag geprüft |
| derselbe Kirchberg-Slug | `street: ""` | `Auerbacher Straße 70A` | derselbe dokumentierte Betreiber-/Kommunalabgleich; Kontakt-/Inhaberwidersprüche nicht harmonisiert, `pruefung` bleibt |

Auch ein Update aller 23 SHA-Werte ließe deshalb ein aktuelles Seed-Gate noch an diesen drei Vorgaben scheitern. Das historische Manifest verlangt heute unter anderem eine nicht mehr beabsichtigte Adressleere. Die alte Erwartung darf die später belegte Adresse nicht ohne erneute fachliche Entscheidung zurücksetzen.

Zusätzliche **In-Memory-Kontrollprüfung**, keine Produktionsdaten: 60 aktuelle Generatorzeilen in eine ausschließlich im Speicher angelegte Tabelle geladen, `updated_at` einheitlich auf den ebenfalls verwendeten synthetischen Rolloutzeitpunkt `2026-10-16T03:00:00Z` gesetzt, unverändertes archiviertes `verify()` aufgerufen. Ergebnis: `ok:false`, `checked_count:60`, genau **26 Meldungen = 23 Timeline- plus drei Zielwertabweichungen**. Dies beweist den Vertragskonflikt auch jenseits des früh abbrechenden `setUp`; es ist ausdrücklich **kein** tatsächlicher Seedlauf oder Produktions-Rolloutnachweis.

Weiteres begrenztes Experiment ohne Dateiedits: nur im Python-Prozess `dossier_notes()` der archivierten Tests durch die zuvor hashgeprüften Notes aus `db6c81e0` ersetzt. **Alle acht unveränderten Wave-E-Testkörper bestanden.** Weder Manifest noch Produktcode wurden dafür geändert. Das unterstützt die Diagnose „historische Vertragsfixture benötigt historische Eingaben“, nicht das Überschreiben alter Auditdaten.

## Konkrete nächste Änderung, getrennt nach Zweck

1. **Handlerkorrektur abnehmen:** zwei geprüfte Skripte samt acht grünen Tests; keine erneute Ruständerung daraus ableiten.
2. **Historische Tests reparieren:** stabile, nachvollziehbar aus `db6c81e0` und dem damaligen Manifest abgenommene Fixture verwenden; nicht weiter beliebige aktuelle Dossiers für den historischen Vertrag lesen. Revision/Fixtureintegrität separat testen. Der normale Offline-Test sollte keine unkontrollierte Git-Historienverfügbarkeit voraussetzen.
3. **Aktuelles Releasegate ausdrücklich versionieren:** eigenes Manifest oder explizite Manifestauswahl mit Release-/Dossierbindung; alle 60 beziehungsweise den tatsächlich gewünschten aktuellen Releasescope gegen echte isolierte Seedanwendung prüfen. Die 23 Notes-Deltas und drei strukturierten Konflikte fachlich disponieren, Quellen-/Historiennachweise erhalten und erst danach neue Digests veröffentlichen. Kein automatisches „Erwartung = gerade beobachteter Wert“.
4. **Fehlanwendung verhindern:** `load_manifest()` validiert derzeit Form/Scope, erzwingt aber die deklarierte `dossier_revision` nicht. CLI/Report sollten deutlich historischen versus aktuellen Zielstand benennen und eine unpassende Auswahl diagnostizieren. Das historische Gate darf nicht still als universelles aktuelles Rolloutgate erscheinen.

**Abnahmegrenze:** Es gibt einen echten offenen Verifier-/Fixturepflegefehler im sauberen HEAD. Er blockiert die Behauptung eines grünen Python-Gesamtgates und einer erfolgreichen aktuellen Wave-E-Verifikation. Er ist kein Beleg, dass das neue Releasebinary falsch gebaut ist, und kein Grund, die überholten Auth-/Buildblocker zu reaktivieren.
