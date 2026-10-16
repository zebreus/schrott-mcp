# Herkunft der 24 verwaisten Annahmen

## Ergebnis und Entscheidungsvorschlag

**Belastbare Herkunftskette zum alten Hein-Schöneiche-Handler, keine aus der Materialliste geratene Identität.** Die 24 Zeilen passen zeitlich exakt zum journalisierten Heinlauf vom 29.09.2026 und zum damaligen Recordervertrag. Git belegt anschließend die Bereinigung von Hein-Rename-Race-Dubletten. **Nicht direkt überliefert** sind die frühere Elternzeile `id=15757` und das konkrete Löschstatement; die genaue Löschtransaktion bleibt daher rekonstruiert, nicht bewiesen.

**Empfehlung zur gesonderten Ownerfreigabe: die 24 verwaisten Altannahmen begrenzt löschen, nicht auf Händler434 umhängen und 15757 nicht wiederanlegen.** Auf der privaten Baseline sind sämtliche 24 Schlüssel bei dem kanonischen Hein-Händler434 bereits mit positiven, jeweils neueren Annahmen vorhanden. Eine Zuordnung würde keine fehlende Annahme retten, könnte aber neuere Zeit-/Bedingungsbelege überschreiben. Kein Reparaturauftrag wird hier ausgeführt oder als autorisiert behauptet; die Ausnahme aus `0900-owner-disposition.md` gilt bis zu einer separaten Entscheidung fort.

## Beweiskette

Private Wurzel: `/root/Documents/hofmann-v3-private-upgrade-20261016-0830/`. Nur Original-Backups `public.db` und `internal.db` über `mode=ro`, `uri=True`, danach `PRAGMA query_only=ON` gelesen; Query-only jeweils 1. Keine Produktion, keine Datenänderung, kein Build/Archiv/Restore.

1. **Public: identischer letzter Schreibzeitpunkt.** Rowids1070–1093 tragen ausschließlich `trader_id=15757`, `accepts=1`, leere Bedingungen, NULL-Gültigkeitsgrenzen und identische `observed_at=updated_at`:
   `2026-09-29T13:16:25.493609987+00:00`.
   Das beweist ihren gespeicherten Beobachtungs-/Schreibstand, nicht für sich allein den ursprünglichen Insertzeitpunkt.
2. **Internal: genau ein Fetch mit exakt diesem Nanosekundenzeitpunkt.** `raw_fetches.id=641`, `run_id=214`, `scraper=be-sitz-schoneiche-hein-schrotthandel`, URL `https://www.hein-schrotthandel.de/schrottpreise/`, HTTP200, 198173Bytes. `ingestion_steps.id=641` desselben Runs beginnt `13:16:24.017369658+00:00`, endet `13:16:25.677549409+00:00`, Status `ok`, 95 Preise übernommen/36 übersprungen. Run214 endet `ok`. Die 24 Annahmen sind nicht als separate Zahl journalisiert.
3. **Historischer Code erklärt die Zeit-/Slugbindung.** Im Stand `44227892^`, `crates/ingestion/src/traders/scheduler.rs:123–154`, wird ein einziger `stamped=Utc::now()` sowohl an Fetchjournal als auch Recorder übergeben. `traders/mod.rs:254–268` löst den Händler über `find_trader_id(handler_slug)` auf; `:348–358` schreibt zu übernommenen Preisen automatisch positive Annahmen mit leerer Bedingung und genau diesem Timestamp. Daher ist der exakte Fetchzeitpunkt ein konkreter Herkunftsanker, nicht nur eine ungefähre zeitliche Nachbarschaft. Welches tatsächlich laufende Binary diesen Lauf ausführte, ist nicht separat hashgebunden; der gelesene historische Vertrag erklärt die gespeicherte Form.
4. **Git belegt den Identitäts-/Slugwechsel ausdrücklich:**
   * `442278923471ae63795e871b2d063ffc751286fd`, **29.09.07:31:16UTC**, „Hein + Daiko BE→BB (Slug-Umzug mit Preis-Erhalt)“: Handler-SLUG von `be-sitz-schoneiche-hein-schrotthandel` auf **`bb-schoneiche-hein-schrotthandel`**; alte Berliner Seedzeile entfernt, Brandenburg-Seed mit derselben Hein-Domain ergänzt. Alter Seed nennt Hein Schrotthandel GmbH, Werner-von-Siemens-Str.12,15566, Sitz Schöneiche. Das ist ein dokumentierter Umzug derselben Standortzuordnung, kein Namensähnlichkeitsschluss.
   * `e4fa145a159e3c8fac4e31935c761ebada1cec4b`, **29.09.13:40:30UTC**, Commitnachricht ausdrücklich **„Rename-Race-Duplikate Hein gelöscht“**. Der tatsächliche Gitdiff enthält Koordinatenerhalt und Geocodetool, **kein SQL-Löschprotokoll und keine ID15757**. Die Nachricht stützt die Ursache, beweist aber weder genaue Row-ID noch FK-Modus/ausführenden Prozess.
5. **Private Journale passen zum zurückgebliebenen alten Writer:** alter Hein-Slug vier gespeicherte Steps, letzter der oben genannte13:16-Lauf; neuer Slug60 Steps, erster gespeicherter am29.09.16:44:23UTC. Damit ist die Gitänderung07:31 nicht gleichbedeutend mit einem bereits erfolgten Writerwechsel. Die Lücke bis16:44 beweist keinen exakten Deployment-/Löschzeitpunkt.
6. **Kanonischer heutiger Datensatz im Backup:** Händler **434**, `bb-schoneiche-hein-schrotthandel`, HEIN Schrotthandel GmbH, Werner-von-Siemens-Str.12,15566Schöneiche, `first_seen_at=2026-09-27T12:34:13.488753536+00:00`. Alle582 Preiszeilen der Hein-Quell-URL liegen bei434; keine Preis-/Currentzeile bei15757. Eine Preiszeile mit exakt dem alten13:16-Zeitpunkt ist nicht mehr vorhanden: spätere Dedupe-/Refreshschritte können Beobachtungszeiten überschreiben; daraus **keinen** direkten Preis-ID-Transferbeweis ableiten.

**Einordnung:** Nanosekunden-Fetchbindung + historischer Slug-Lookup + ausdrücklich dokumentierte Rename-/Dublettenbereinigung ergeben hohe Sicherheit für die Herkunft **Hein-Schöneiche-Rename-Altbestand**. Die frühere numerische Elternidentität15757→alterSlug ist mangels Vor-Löschsnapshot kein direktes Stammdatenzitat. Es wäre überzogen, exakte Löschzeit, ausgeführtes SQL oder FK-Deaktivierung als erwiesen auszugeben.

## Warum eher Löschen als Zuordnen?

Eigener Join der orphan-Materialschlüssel auf `trader_materials` bei434: **24 von24 vorhanden,24 positiv,24 mit neuerem observed_at**. Bei23 stammt der neuere Beleg vom16.10.03:17:36UTC, bei Material21 vom29.09.16:44:25UTC. Teilweise existieren inzwischen fachlich präzisere Bedingungen, z.B. bei Mischschrott/Messing; diese nicht mit leerem Alttext überschreiben. Das ist ausschließlich ein Vergleich vorhandener Acceptance-Belege, **keine parallele Katalogprüfung** und kein Beweis für aktuelle Annahme aller Materialien außerhalb dieses Datenstands.

Eine künftige begrenzte Bereinigung sollte nur nach ausdrücklicher Freigabe und frischer Prüfung erfolgen: genau24 Zeilen, weiterhin fehlender Elternhändler15757, vollständige alte Zeilen unverändert, keine zugehörigen Preise/Current, keine neu hinzugekommenen Daten oder Ownershipbindungen; Sicherungs-/Auditnachweis vorher. **Rowids allein reichen als Löschprädikat nicht.** Falls der Owner für die historische Identität einen direkten Vor-Löschbeleg verlangt, bleibt die Bestandsschuld offen, bis dieser vorliegt; keine erfundene Zuordnung als Abkürzung.

## Reproduzierbare lesende Prüfungen

Verbindungen ausschließlich so öffnen; `db` auf eines der beiden genannten privaten Originale setzen:

```python
c = sqlite3.connect(db.as_uri() + '?mode=ro', uri=True)
c.execute('PRAGMA query_only=ON')
assert c.execute('PRAGMA query_only').fetchone() == (1,)
```

Public:

```sql
SELECT rowid,* FROM trader_materials WHERE trader_id=15757 ORDER BY rowid;
SELECT id,slug,name,street,postcode,city,first_seen_at
FROM traders WHERE slug='bb-schoneiche-hein-schrotthandel';
SELECT count(*) AS orphan_count,
       count(n.trader_id) AS covered,
       sum(n.accepts=1) AS positive,
       sum(n.observed_at>o.observed_at) AS newer
FROM trader_materials o LEFT JOIN trader_materials n
  ON n.trader_id=434 AND n.material_id=o.material_id
WHERE o.trader_id=15757;
```

Internal:

```sql
SELECT id,run_id,scraper,url,status_code,byte_len,content_hash,fetched_at
FROM raw_fetches WHERE fetched_at='2026-09-29T13:16:25.493609987+00:00';
SELECT * FROM ingestion_steps WHERE id=641;
SELECT * FROM ingestion_runs WHERE id=214;
SELECT scraper,count(*),min(started_at),max(started_at)
FROM ingestion_steps WHERE scraper IN
 ('be-sitz-schoneiche-hein-schrotthandel','bb-schoneiche-hein-schrotthandel')
GROUP BY scraper;
```

Git: `git show 44227892`, `git show e4fa145a`, `git show '44227892^:crates/ingestion/src/traders/mod.rs'` und entsprechend `scheduler.rs`. Achtung: historisches `raw_fetches.content_hash=950e98425af8f1bb` ist laut Scheduler nur Rust-Hash über Slug/URL/Status/Bytelänge, **kein Hash des HTML-Inhalts**; kein behauptetes rekonstruierbares Original-HTML.

## Suchgrenzen und Originalschutz

Gesucht: gegenwärtiger Repo-Inhalt nach15757, altem Hein-Slug und Lösch-/Mergehinweisen; alle erreichbaren Git-Commitnachrichten nach Hein/Rename-Race; Git-Diffhistorie nach15757/Rename-Race unter `research/tools/scripts/recherche`; betreffende Seed-/Handlerdiffs; private Public- und Import-/Fetch-/Step-/Feedbackbelege. Feedback289 belegt bereits am28.09. den alten Hein-Slug und die falsche BE-Zuordnung, enthält aber ebenfalls keine numerische15757. Kein eigenständiges ID15757-Löschprotokoll gefunden. Nicht untersucht: unzugängliche/unbekannte Vor-Löschbackups, Systemjournal/Shellhistory, nicht erreichbare Gitobjekte, Produktionsdaten oder externe Quellen. Kein solches Artefakt neu erzeugt.

Die Originalsicherungen nach den Abfragen unverändert: Public SHA256 `3b27bb912cba5dcda0d6bd1becfdb4a379ce35428c282146c5dd3183adf9d24a`, Internal `37d7f47b13d456b5cd7bb92120815c25f008695a7dba8d8c99e715f245e7ea02`, Backupmanifest `9e0d255edb1ab78fa88c843f04602666cf39613aaf378d46ba595ef1f5d9ced8`. Reproduzierbarer Zeilensatz-Fingerprint: `sha256(json.dumps(fetchall(),separators=(',',':'),ensure_ascii=False).encode())` für obiges `SELECT rowid,* ... ORDER BY rowid` = **`46d926ee52be0571775a9cbfbf6a7d0dde62e37e315fa0940baa99c4b68c3d29`**. Dieser neue Analysefingerprint ersetzt keinen bestehenden Manifesthash.

Alle143 Fremddateien gegen `/tmp/opencode/checkin-20261016-013049-baseline.json` geprüft: unverändert. Main während des Reviews ownerseitig bis `e024d5ba` weitergelaufen; keine fremden Änderungen angefasst. Einzige eigene Änderung dieses Auftrags: dieser Bericht. **Vorschlag vorgelegt, nichts repariert oder zur Produktion freigegeben.**
