# Unabhängige Releasewirkungsprüfung der privaten Upgradeprobe

Stand: 16.10.2026; Main `9c140e19e92d404aac2c384da6459ce387f79cbe`.

## Entscheidung

**HOLD für die Gesamtfreigabe bis zu den unten benannten Ownerentscheidungen; kein neu nachgewiesener Hofmann-Codeblocker.** Die drei HOLDs der Probe sind nicht drei bewiesene Upgradefehler:

1. **DDL:** Die zwei zusätzlichen Tabellen gehören zur bereits integrierten MKM-Funktion, nicht zu einer unerklärten Hofmannmigration. Empfehlung: den konkret erweiterten additiven Bootvertrag akzeptieren; die bisherige Freigabe nur der Hofmanntabelle reicht dafür nicht.
2. **Seed:** 715 Updates sind nicht 715 neue Sachverhalte. Ein großer Teil ist bereits geprüft bzw. ausdrücklich disponiert. Neu zu entscheidender Releaseumfang sind insbesondere Katalogerweiterung/Zeitstempelwirkung und die ausdrückliche Übernahme des vollständigen, unten abgegrenzten Seed-Diffs. Frühere Kontakt-HOLDs nicht wieder aufmachen.
3. **FK:** 24 reale, öffentlich abfragbare Altlasten, aber keine durch dieses Upgrade erzeugte Verletzung, keine Preisverwaisung und keine Wiederanbindung an einen neuen Händler. Empfehlung: eng dokumentierte Bestandsausnahme statt ungeprüfter Reparatur; eine Anforderung „FK-Check vollständig leer“ kann nur der Owner ausdrücklich disponieren, nicht dieser Review wegdefinieren.

Kein Deployment, Restore oder Datenreparatur ist damit autorisiert. Die Entscheidung über diese Ausnahmen und den tatsächlichen Release bleibt beim Owner.

## Eigenständig geprüfte Evidenz und Schutz

Vollständig gelesen: `research/checkin-20261016-0830-hofmann-upgrade-rehearsal.md`, der frühere `0300-upgrade-seed-review.md`, `0300-owner.md`, `0730-owner.md`, beide `0300-contact-gates-{a,b}.md`, `0815-hofmann-release-gates.md` sowie `0830-owner.md` (jeweils mit Präfix `checkin-20261016-`).

Evidenzwurzel: `/root/Documents/hofmann-v3-private-upgrade-20261016-0830/`.

* `rehearsal-manifest.json` selbst frisch SHA256-geprüft: `5b2956a7ceb0485a0f7ba44c7bb0845641bbd747a94d75bd7d359c754089d543`. **Alle 279 Einträge** seiner `evidence_sha256`-Map gelesen und gegen ihren gespeicherten Hash geprüft: **0 fehlend, 0 abweichend**. Das ist Dateibindung, keine unabhängige Beglaubigung der ursprünglichen Produktionsentnahme.
* Original `public.db`: `3b27bb912cba5dcda0d6bd1becfdb4a379ce35428c282146c5dd3183adf9d24a`; Original `internal.db`: `37d7f47b13d456b5cd7bb92120815c25f008695a7dba8d8c99e715f245e7ea02`. Backupmanifest: `9e0d255edb1ab78fa88c843f04602666cf39613aaf378d46ba595ef1f5d9ced8`.
* Alle **4418** archivierten Gitdateien von `53a24927` gegen tatsächliche Git-Blob-IDs des privaten `source/` verglichen: keine fehlt; exakt die zwei manifestierten bestehenden Instrumentierungsdateien unterscheiden sich (`seed_traders.rs`, `hofmann_rastatt.rs`). Die zusätzliche Harnessdatei ist separat manifestiert. Keine unerkannte Änderung an Store, Schema, Material-Upsert oder Readpfad.
* `git diff --name-only 53a24927 9c140e19` enthält ausschließlich `0800-amr-http-review.md` und `0830-owner.md`; derselbe produktive Code-/Dossierstand.
* Eigene Auswertung der komprimierten Vollsnapshots `boot-before/open/seed1/seed2.json.gz`, nicht nur Übernahme der Zusammenfassungszahlen. Zusätzlich Original-Public und beide privaten Public-Arbeitsdatenbanken über SQLite-URI **`mode=ro`**, danach **`PRAGMA query_only=ON`** geöffnet und den Wert 1 kontrolliert. Keine DDL/DML, keine Produktion geöffnet, kein Build oder erneuter Ingest.
* Die **143** Dateien aus `/tmp/opencode/checkin-20261016-013049-baseline.json` frisch SHA256-verglichen: keine Abweichung. Nach Abschluss der Read-only-Prüfungen nochmals alle 279 Evidenzhashes und die drei Originalhashes kontrolliert: unverändert. Keine Code-/Dossieränderung; einzige eigene Hauptrepoänderung ist dieser Bericht.

## 1. Erweiterte DDL: tatsächlicher Vertrag statt pauschaler Migrationsangst

### Befund und Wirkung

`crates/store/src/public/mod.rs:270–283` führt beim schreibenden Open nacheinander die Traders-, Materials-, Prices- und Hofmann-Schemata aus. Die zwei Tabellen stehen in `crates/store/src/public/prices.rs:79–90` und stammen aus **`d9fa958f19b840d01bdaf516e29e4c6b65873b2b`, 11.10.2026, „integrate reviewed MKM article ingestion and atomic reconciliation“**. Sie sind auf dem alten Datenbankstand schlicht noch nicht vorhanden. Die Hofmannintegration hat sie nicht nebenbei neu erfunden.

Die Vollsnapshots bestätigen: vorhandene Schemaeinträge bleiben erhalten, drei neue Tabellen samt ihren Unique-Indizes kommen hinzu; alle drei sind unmittelbar nach Open leer. Bestehende Tabellenzeilen bleiben gleich. `user_version` bleibt 6. Nach beiden vollständigen Seeds sind die beiden Artikeltabellen weiterhin leer; der Hofmannlauf benötigt sie nicht.

Das bedeutet aber **nicht**, dass die Artikeltabellen ungenutzte Dekoration sind:

* `mkm_metals/recording.rs` verwendet `OWNER="mkm-article-v1"` und ruft `record_article_price_snapshot` auf.
* `prices.rs:604–680,744–755` liest frühere Artikelmanifeste und Acceptance-Ownership und schreibt Manifest, Ownership, Annahmen und Preisabgleich in derselben Transaktion. Der Vertrag ist dort auf MKM-Händler, exakte Quell-URL, Producer und 60 Artikel eingegrenzt.
* `source_article_snapshots` hält Beobachtung und Manifest mit Unique-Schlüssel Händler/Quelle/Owner/Zeit. `article_acceptance_owners` hält aktuellen Eigentumsnachweis je Händler/Material. Beide referenzieren Händler, letztere zusätzlich Material. Beim regulären MKM-Lauf dürfen also neue Daten und Änderungen an `trader_materials` entstehen; die reine Hofmannprobe beweist diesen weiteren Schedulerlauf nicht.

**Tatsächlicher Readpfad:** kein ORM, das unbekannte Tabellen generell verbietet. Der MCP-Weg ist `tools.rs` → isolierter SQL-Worker → `PublicDb::open_read_only` → validiertes `SELECT`. Read-only-Open führt keine Schemaanlage aus. Der SQL-Weg hat keine Tabellen-Allowlist: neue Tabellen und ihre späteren Inhalte sind grundsätzlich öffentlich abfragbar, auch wenn `SCHEMA_DOC` sie noch nicht aufzählt. Das sind öffentliche Quellen-/Annahmebelege, kein internes Nutzerledger; trotzdem muss ihre Öffentlichkeit zum akzeptierten Vertrag gehören. Die bestehenden View-/Abfragen erhalten durch die reine Anlage keine neuen Preise oder Annahmen.

### Empfohlener Ownerentscheid

**„Akzeptiert: Beim ersten Boot dieses Gesamtstands entstehen zusätzlich zum Hofmannledger die beiden MKM-Belegtabellen samt Indizes, leer und ohne Datenrewrite; spätere MKM-Nutzung und öffentliche SQL-Lesbarkeit sind Teil des bereits integrierten Releaseumfangs.“**

Das erweitert den expliziten Entscheid aus `0815-hofmann-release-gates.md`, der nur `hofmann_tin_cutovers` akzeptiert. Kein Anlass für eine spontane Downmigration oder künstliches Auslassen der MKM-Tabellen: der vorhandene Producer benötigt sie. Wenn dieser MKM-Vertrag nicht im Release enthalten sein darf, ist das eine echte Scopeentscheidung vor Release, nicht mit einem Hofmann-only-Test erledigt.

**Restunsicherheit:** Diese Probe hat keinen vollständigen regulären MKM-Schedulerlauf auf dem Seedstand nachgewiesen. Nach alter/neuer Writervermischung bzw. Binary-Rollback werden weder Artikelownership noch Hofmannledger zuverlässig respektiert. Die unveränderte `user_version=6` ist kein Kompatibilitätsbeweis für alte Writer. Additive Tabellen allein wären rückwärtslesbar; die mit ihnen verbundenen späteren Datenentscheidungen sind nicht durch Binarytausch rückgängig gemacht.

## 2. Vollseed: bekannte Dispositionen und wirklich zusätzlicher Umfang

### Unabhängig nachgezählte Wirkung

Aus den vollständigen Traderzeilen, nach `slug` verglichen:

| Wirkung erster Seed | Eigener Befund |
|---|---:|
| Händler vorher/nachher | 3881 / 3893 |
| Updates / Inserts / Deletes | 715 / 12 / 0 |
| Geänderte Zeilen mit mindestens einem Feld außer notes/extra_json/updated_at | 170 |
| Nur diese drei Metafelder geändert | 545 |
| notes / extra_json / updated_at ersetzt | je 715 |
| Statusänderungen | 83 |
| Straße / PLZ geändert insgesamt | 61 / 54 |
| Davon nichtleer → leer | 28 / 14 |
| Telefon / E-Mail / Öffnungszeiten geändert | 45 / 16 / 19 |

Die früheren höheren Kontaktzahlen aus dem älteren Seedreview sind **nicht** der finale Diff. Weitere jetzige Spaltenzahlen: Beschreibung 39, Website-Status 39, Website 32, Name 19, Ort 17, Lat/Lon je 9, Händlertyp 4, dropoff_json 3, pickup_json 2, Bundesland 1. Bei `extra_json` wird als bisheriger Schlüssel ausschließlich `seed_file` entfernt, genau 153-mal. Kein Trader verschwindet; der nicht mehr geseedete historische Sangerhausen-Mainmetall-Datensatz bleibt erhalten.

Preise, Current und `trader_materials` sind in den beiden Seedstufen zum Vorzustand zeilengleich. Sämtliche Internal-Snapshotdaten bleiben gleich. Das ist **keine** Zusage unveränderter öffentlicher Antworten: `v_current_prices` joint Händler/Material, daher können etwa Orts-/PLZ-/Namensänderungen schon vorhandene Preisresultate anders beschriften. Status-/Adressfilter über `traders` liefern bewusst andere Ergebnisse.

### Bereits entschieden bzw. schon sachlich geprüft — nicht als neue Blocker verkaufen

* **153 `seed_file`-Entfernungen:** in `0300-owner.md` ausdrücklich akzeptierter Provenienzwechsel zum Dossiercompiler. Hier bestätigt, keine zweite Freigabe desselben Einzelthemas nötig. Datenbankmetadaten werden tatsächlich ersetzt; Git-/Auditbelege sind nicht gleich einem Erhalt dieser alten Schlüssel in der aktuellen DB.
* **Legacybooleans:** der Owner hat ausdrücklich akzeptiert, dass sie keine Servicefacts sind; aktueller dokumentierter Readvertrag ist Konditions-JSON, fehlendes `allowed` bedeutet unbekannt. Kein automatisches Umdeuten alter `accepts_dropoff=1` und keine heimliche SQL-Bereinigung. Beliebiges Kompatibilitäts-SQL kann Altwerte weiterhin sehen.
* **AGOSI:** finale zwei Ergänzungen `+49 7231 960-0` / `info@agosi.de` entsprechen dem freigegebenen zentralen Kontakt, nicht einer Behauptung zum separaten NE-Hof. **Fernau/Graf/Willeführ:** Detailkontakte weiterhin zurückgehalten, leere Seedwerte mit **Keep**, keine neue Clear-Zusage. Die damalige Quellen-/Registergrenze bleibt bestehen, ist aber keine offene Freigabe der ausdrücklich nicht publizierten Werte.
* **Bausch/SM Metals/Limekon:** Owner hat die nicht freigegebenen Kontakt-/Öffnungszeitenwerte auf explizites Clear umgestellt; bei Limekon zusätzlich positive Service-JSONs. Brosdas frühere Telefonnummer wird tatsächlich geleert. Nicht mit der veralteten Empfehlung verwechseln, diese Angaben erst noch zu entfernen. Ein Clear auf bereits leerem Ausgangsfeld erzeugt zurecht keinen nichtleer→leer-Diff.
* **83 Statusänderungen:** kein neu entdecktes Paket. Die alte Vollseedprüfung deckt exakt dieselbe Verteilung ab: 70 aktiv→pruefung, 10 pruefung→aktiv, je 1 unbekannt→aktiv, unbekannt→pruefung und geschlossen→pruefung. Letzteres bleibt konkret `sn-lauta-das-recycling-inh-raphael-duffort`; `pruefung` ist keine bestätigte Wiedereröffnung. Frühere Kontakt-HOLDs rechtfertigen nach den korrigierten Kontaktdispositionen nicht automatisch eine Rücknahme der bereits separat belegten Identitäts-/Aktivitätsentscheidung.
* **28 Straßen-/14 PLZ-Leerungen:** dieselben im früheren Einzelreview aufgeführten Quellenrücknahmen, nicht neu durch Hofmann erzeugt. Sie entfernen unbelegte Präzision, lassen aber teilweise frühere GEOs stehen. Damit wird kein Tor-/Einfahrtsnachweis behauptet. Der vollständige Seedentscheid muss diese sichtbare Informationsrücknahme einschließen; eine pauschale Aussage „Seed ändert nur Notizen“ wäre falsch.

### Schrott24: konkrete noch ausdrücklich zu übernehmende Rücknahme

Bei **`nw-nrw-weit-schrott24-metaloop-online-portal`** werden beide bislang positiven JSON-Objekte auf **`{}` gesetzt**. Das ist ein nichtleerer expliziter Seedwert (**Set**, nicht Clear und nicht Keep). Die Anlieferwerbung mit 80+ Stellen/Versand und die Partnerabholung verlieren damit ihren bisherigen positiven `allowed`-Nachweis. Die neue Beschreibung nennt Betreiber-/Adress-/Insolvenzkonflikt und bestätigt gerade keine aktuelle Anliefer-/Abholzusage.

Das war bereits im `0300-upgrade-seed-review.md` als konkrete Wirkung benannt, ist also **kein neu aufgetauchter Parserfehler**. In den gelesenen Ownerdispositionen finde ich aber keinen ebenso expliziten Einzelentscheid wie zur `seed_file`-Entfernung. **Empfehlung: diese Rücknahme ausdrücklich akzeptieren**, weil ungeklärte Betreiberwerbung keine aktuelle Leistungszusage trägt; nicht als Geschäftsaufgabe ausgeben. Sie wirkt nur auf diesen Datensatz. Andere regionale Schrott24-Partner-/Portalbeschreibungen werden damit nicht automatisch widerrufen; keine unbelegte globale Harmonisierung vornehmen.

### Wirklich zusätzlich zu bewertende Dimension: 29 Materialien und Zeitstempel

Erster Seed: **54 → 83** Katalogzeilen. Bei allen 54 bestehenden Materialien bleibt jede Spalte außer `updated_at` gleich, insbesondere ID/Slug/Einheit/Beschreibung. Kein Remapping vorhandener Preise oder Annahmen. Die 29 neuen Slugs sind:

```text
ablenkeinheiten, aluminium-isoprofile, aluminium-profile-beschichtet,
aluminium-spaene, blei-gemischt, blei-geschoss, computer, cu-ms-kuehler,
elektrobauteile, elektronikstecker, integrierte-schaltungen, kabel-alu-cu,
kabel-ohne-stecker, kupfer-granulat, kupfer-stanzabfaelle, laufwerke,
luefter, metallschrott-gemischt, neodym-magnete, netzteile, nickel,
notebooks, optische-datentraeger, panzersicherungen, prozessoren,
schrott-eisen-ne-gemischt, wolfram, zinn-legierungen, zinn-rein-formgemischt
```

28 sind `EUR/kg`, `schrott-eisen-ne-gemischt` ist `EUR/t`. Die neuen Katalogzeilen allein behaupten weder Händlerannahme noch einen Preis. Sie werden aber unmittelbar durch `list_materials`, Corpuszähler und beliebiges MCP-SQL sichtbar; spätere Handler dürfen sie verwenden. Ihre fachliche Trennung muss deshalb Teil der Gesamtfreigabe sein, nicht nur die des Zinnfixes. Dieser Review bestätigt die Additivität und Einheiten, **nicht** eine erneute Primärquellenprüfung aller 29 Materialdefinitionen und aller Producerzuordnungen.

`pipeline.rs:543–565` ruft bei **jedem** Seed alle Material-Upserts auf; `materials.rs:52–75` aktualisiert auch bei unverändertem Inhalt `updated_at`. Frisch bestätigt: erster Lauf 54 Timestampupdates + 29 Inserts; zweiter Lauf **83 Timestampupdates**, keine weitere Materialzeile, keine Inhaltsänderung. SQLite-AUTOINCREMENT steigt jeweils um 83 auch bei Konflikt-Upserts: **100790 → 100873 → 100956**. Das ist Katalog-Refresh-/Sequenzchurn, kein Nachweis neuer Recherche oder 83 neuer Materialien. Der Seed läuft auch im regulären Pipelinepfad, nicht nur beim Boot. Die Traderseite bleibt dagegen nach Erstanwendung zeilengleich/hash-idempotent.

### Empfohlener Ownerentscheid zum Seed-HOLD

**„Akzeptiert: exakt der vollständige private 715/12-Diff einschließlich der bereits geprüften 83 Status-, 28 Straßen-/14 PLZ-Rücknahmen und der expliziten Schrott24-JSON-Rücknahme; bestätigte Kontakt-Clear/Keep-Grenzen bleiben unverändert. Zusätzlich werden die 29 additiven Materialdefinitionen als Katalogumfang freigegeben; Materialzeitstempel sind Refreshzeiten und ihre wiederholten Writes/Sequenzsprünge akzeptierter technischer Effekt, kein Freshnessbeleg.“**

Kein Grund, die 153 Provenienzentscheidungen oder erledigte Kontaktkorrekturen als neue Pflichtreparatur aufzutürmen. **Noch nicht abgedeckt** sind eine explizite Übernahme dieses finalen Gesamtumfangs und gegebenenfalls die fachliche Freigabe der 29 Katalogdefinitionen durch den zuständigen Owner. Wenn ein Verbraucher `materials.updated_at` als fachliche Neubewertung interpretiert, ist diese konkrete Semantik vor Release zu klären, nicht durch diesen Bericht umzudefinieren.

**Restunsicherheit:** private Baseline ist vom 08:28:41UTC; zwischenzeitliche Produktionsanreicherungen sind nicht geprüft. Insbesondere Blank+Keep kann dann bewusst etwas anderes erhalten; Clear kann etwas inzwischen neu Befülltes entfernen. Die Seedroutine ist keine eineinzige Gesamttransaktion über alle Händler/Materialien; Erfolg in der Probe garantiert bei späterem I/O-Fehler keinen atomaren Rollback des kompletten Bootseeds. Regulärer Fehler-/Stoppplan bleibt erforderlich. 0 Feldvertragsabweichungen im Prüfartefakt belegen korrekte Anwendung, nicht automatisch die fachliche Richtigkeit jedes Sollwertes.

## 3. FK-Schuld: konkret sichtbare Reichweite, keine automatische Reparatur

Eigene `PRAGMA foreign_key_check`-Abfrage auf Original, `boot/public.db` und `transition/public.db`: jeweils exakt **24** Verletzungen, `trader_materials`-Rowids **1070–1093**, Eltern `traders`, FK-ID **1**. Die Schema-FK-Reihenfolge und der Join bestätigen: fehlender **Händler 15757**, **alle 24 Materialien existieren**. Alle 24 Zeilen haben `accepts=1`, leere Bedingungen. Insgesamt existieren 1315 Annahmezeilen; sie bleiben durch Boot/Seed/Hofmanntransition zeilengleich.

Betroffene Materialien: blei, aluminium-profile, aluminium-blech, aluminium-guss, kabel-alu, aluminium-gemischt, eisenschrott-gussbruch, elektromotoren, platinen, hartmetall, hss-werkzeuge, kabel-kupfer, katalysatoren, stahlschrott-shredder, kupfer-gemischt, kupfer-millberry, kabel-blei, kabel-mit-stecker, messing, mischschrott, edelstahl-v2a, edelstahl-v4a, zink, zinn.

**Scope am tatsächlichen Read-/Writepfad:**

* `SELECT count(*) FROM prices WHERE trader_id=15757` und entsprechend `current_prices`: in allen drei geprüften DBs **0**. Keine Preis-/Current-FK-Schuld dieses Händlers; die vorhandene Preisview kann dadurch keinen verwaisten Preis präsentieren.
* Ein Inner-Join von `trader_materials` auf `traders` verwirft alle 24. **Aber sie sind nicht generell unsichtbar:** das tatsächliche MCP bietet freies validiertes SQL; direkte Abfragen/Annahmezählungen oder ein Materialjoin ohne Händlerjoin können die 24 positiven Annahmen liefern. `PublicDb::existing_acceptance` (`prices.rs:1018–1032`) liest ebenfalls direkt nach Händler-/Material-ID, ohne Elternjoin. Ein Caller mit der alten ID kann also `Some(true, "")` erhalten. Kein ORM filtert diese Schuld magisch heraus.
* `PublicDb::open` setzt `foreign_keys=ON`; das macht bestehende Verletzungen weder sauber noch verhindert es generell Open/zulässige Writes an anderen Händlern. `set_acceptance` benutzt echte FK-gebundene Inserts/Upserts. Keine FK-Deaktivierung als angebliche Lösung nötig oder akzeptabel.
* Keine stille Reidentifizierung: Trader-Sequenz vorher **25858**, neue zwölf IDs liegen **25861–26355**, danach Sequenz **26585**. Der Seed vergibt keine 15757 neu. Alle bestehenden IDs bleiben erhalten; fehlender 15757 bleibt fehlend. Eine spätere ausdrücklich gesetzte historische ID wäre eine andere, hier nicht autorisierte Operation.

### Empfohlener Ownerentscheid

**„Akzeptiert als begrenzte vorbestehende Datenqualitätsschuld: genau diese 24 Annahmen ohne Händler15757, keine neuen FK-Verletzungen durch das Upgrade. Separates Klärungsticket mit Herkunfts-/Löschentscheidung; keine Wiederanlage eines mutmaßlichen Händlers und keine Löschung im Release. Monitoring darf weiter nur integrity_check=ok, nicht vollständige FK-Integrität behaupten.“**

Das ist vertretbar, weil der Release den Schaden nicht erweitert und keine betroffenen Preise bzw. ID-Neubindung erzeugt. Es ist **keine** Behauptung, positive Annahmen ohne Anbieter seien fachlich harmlos. Wenn der geltende Releasevertrag null FK-Verletzungen verlangt oder direkte Annahmezählungen korrekt sein müssen, bleibt dieser Punkt ein **echter Owner-HOLD** bis autorisierter Behandlung oder ausdrücklich dokumentierter Ausnahme. Der Reviewer erteilt weder die Ausnahme selbst noch einen Reparaturauftrag.

**Restunsicherheit:** fehlende Händleridentität und Entstehungsursache sind mit diesen Belegen nicht rekonstruiert. Kein belastbarer Schluss auf geschlossen/gelöscht/zusammengeführt; deshalb keine Reparatur aus der Materialliste erraten. Ein vollständiger Livezustand nach Probenzeitpunkt wurde nicht abgefragt.

## Reproduzierbarkeit der unabhängigen Checks

Alle folgenden Checks benötigen weder schreibendes DB-Open noch Build oder Ingest:

1. Python: `json.load(rehearsal-manifest.json)`; für jeden Eintrag in `evidence_sha256` Datei unter Evidenzwurzel per `hashlib.file_digest(..., 'sha256')` vergleichen. Keine Manifest-/Hashneuberechnung als Ersatz für einen Unterschied verwendet.
2. Python: `gzip.decompress`/`json.loads` der vier Bootsnapshots; `public.tables.traders.rows` nach `slug` und jede Spalte vergleichen; Materialien entsprechend; `prices/current_prices/trader_materials` vollständig gleichsetzen. Ergebniszahlen stehen oben, einschließlich zweitem Materialseed.
3. Je private DB: `sqlite3.connect(path.as_uri()+'?mode=ro', uri=True)` und `PRAGMA query_only=ON`; dann `PRAGMA foreign_key_check`, Abfragen nach `trader_id=15757`, `LEFT JOIN` auf beide Eltern und `sqlite_sequence`. Read-only-URI und Query-only waren gesetzt; beim abschließenden Wiederholungscheck mit explizitem `uri=True` erneut Query-only=1 und genau 24 FK-Verletzungen in allen drei DBs. Keine Writeprobe erforderlich oder durchgeführt.
4. `git ls-tree -r -z 53a24927`; Dateiinhalt des privaten Exports als Gitblob (`sha1(b'blob '+len+NUL+Inhalt)`) vergleichen. Erklärte zwei Instrumentierungsdifferenzen plus zusätzliche manifestierte Harnessdatei, keine Freigabe des Harness als Releasecode.

## Verbleibende Owner-Releasegates

* Drei eng formulierte Entscheidungen oben protokollieren; insbesondere nicht „alles geprüft“ aus einer Gesamtzahl ableiten. Falls MKM-/Katalogumfang nicht gewollt ist, Releaseumfang vor Installation klären, nicht Produktionsdaten ad hoc anpassen.
* Anforderungsgemäße aktuelle Sicherung und exklusiver Writerwechsel; die privaten Backups sind kein getesteter, datenbankübergreifend atomarer Restore. Kein alter Writer nach persistierter Hofmann-/Artikelentscheidung; Fail-safe bleibt Ingestionstopp, kein bloßer Binary-Rollback.
* Exakten freigegebenen Commit/sauberen Export und Releaseartefakte/Remote binden, ohne die 143 fremden Dateien. Keine Aussage über inzwischen erfolgte Auth-/Buildfortschritte aus älteren Berichten als neuen Blocker übernehmen; dieser Review hat keinen Build oder Push ausgeführt.
* Nach genehmigtem Release tatsächlichen regulären Schedulerlauf und öffentliche Readwirkung einschließlich Hofmann-Historie/Current, MKM-Vertrag und begrenzter FK-Ausnahme rein lesend nachweisen. Die private Hofmann-PASS-Evidenz ersetzt diesen Betriebsnachweis nicht.

**Fazit:** Technisch empfehlenswert ist eine explizite, begrenzte Annahme der zusätzlichen MKM-DDL und der vorhandenen FK-Schuld, plus die bewusste Gesamtseed-/Katalogfreigabe. Bis dieser Ownerentscheid samt Betriebs-/Rollbackgates vorliegt: **HOLD, nicht wegen erneut behaupteter Hofmannfehler oder bereits erledigter Kontakt-HOLDs.**
