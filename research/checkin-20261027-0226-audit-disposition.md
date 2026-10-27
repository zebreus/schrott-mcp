# Owner-Disposition - Quellenreviews und Seed-Korrekturen, 27.10.2026

## Reviewumfang

Vier unabhängige Reviews bezogen sich auf den Seed-/Dossierstand `7a431dcb`:
109, 107 und 107 disjunkte Bestandszeilen sowie die 43 neu hinzugefügten Slugs.
Die Bestandsreviews dokumentierten Quellenalter und selektive Live-Prüfungen;
nicht jede historische URL wurde erneut abgerufen. Die 43 neuen Zeilen wurden
einzeln klassifiziert. Keine Review schrieb in Produktion oder in die
Arbeitskopie.

- Shard 1: 109 Zeilen, 365 Felder; 194 gestützt, 165 begründete Clears, sechs
  mehrdeutige Felder.
- Shard 2: 107 Zeilen, 306 Felder; 165 gestützt, 139 begründete Clears, zwei
  mehrdeutige Felder.
- Shard 3: 107 Zeilen; keine weitere beanstandete Änderung. GEO-Clears sind
  Rücknahme eines nicht belegten Punktes, kein Schließungsnachweis.
- 43 neue Slugs: 31 im Umfang der Dossiers gestützt, drei beanstandet und neun
  mit unzureichender Beleglage. Die neun bleiben ohne neue ungesicherte
  Standort-/Leistungsbehauptung; Unsicherheit ist kein Beweis, dass eine Angabe
  falsch oder ein Betrieb geschlossen ist.

## Übernommene Korrekturen

- **Brameier, Schopsdorf:** `status` auf `pruefung` gesetzt. Shop, Impressum,
  Registerspiegel und Stellenanzeige belegen den aktuellen Kfz-Teileverkauf und
  den Betrieb, aber keinen aktuellen Schrott- oder Altfahrzeugankauf.
  `website_status: aktiv` bleibt; keine Schließung behauptet.
- **ALBA Nord, Wismar:** `status` auf `pruefung` gesetzt. Betreiber- und
  Registerseiten belegen Abfallannahme/Containerdienst, nicht vergüteten
  Metallankauf. Website-Status und belegte Betriebsstättendaten bleiben.
- **Tamm, Wuppertal-Ronsdorf:** `status` auf `pruefung` gesetzt, weil der
  aktuelle Betreiberauftritt Entsorgungszahlung, aber keine Vergütung für
  angelieferten Schrott belegt. In `dropoff_json` und `pickup_json` wurden die
  aus einer historischen Passage von 1982 abgeleiteten Kundenarten entfernt;
  aktuell belegte Anliefer- und Containerleistungen bleiben erhalten.
- **Schrotthändler Plus, Bochum/Wattenscheid:** `status` auf `pruefung`, mobile
  Ankaufbeschreibung und `pickup_json` zurückgenommen. Die Website ist aktuell
  erreichbar und `website_status: aktiv` bleibt. Der Betreiber-/Rechtsträger-
  Bezug ist trotz undatiertem, selbstgeführtem XING-Profil nicht unabhängig
  genug bestätigt. `clear_fields: 'description,pickup_json'` stellt sicher,
  dass leere Seed-Felder gespeicherte Claims tatsächlich zurücknehmen. Keine
  Betriebsaufgabe behauptet.
- **DB Recycling, Rochlitz:** Straße wie die bereits geleerte PLZ aus dem
  strukturierten Datensatz entfernt, weil beide Angaben nur auf derselben
  unbestätigten Betreiberquelle beruhten. Die am 27.10. direkt gelesene
  Betreiberpreisliste (gültig ab 03.10.2026) nennt weiterhin aktuellen
  Metallankauf; `status: aktiv` und `website_status: aktiv` bleiben. Kein
  Ersatzstandort geraten.
- **Rheinische Scheidestätte Augsburg und Aachen:** Die Live-Seiten zeigten
  `geschlossen` beim Abruf außerhalb der Öffnungszeit; beide Seiten werben
  gleichzeitig mit Filialbesuchen und nennen normale Werktagszeiten. Das ist
  kein Schließungsbeleg, daher bleiben beide Statuswerte `aktiv`. Die alten
  Samstagszeiten waren jedoch falsch: beide Betreiberseiten nennen Samstag
  `geschlossen`, daher wurden die strukturierten Öffnungszeiten auf Werktage
  korrigiert. Quelle (Abruf ca. 02:16 UTC / 03:16 CET):
  https://rheinische-scheidestaette.de/unternehmen/filialen/augsburg/ und
  https://rheinische-scheidestaette.de/unternehmen/filialen/aachen/.

## Abgelehnte Findings und verbleibende Caveats

- Der Fund „geschlossen“ bei Augsburg/Aachen wurde als momentaner, dynamischer
  Öffnungsstatus falsch zu dauerhaftem Filialstatus ausgelegt. Die direkt
  gelesenen Betreiberseiten widerlegen eine Schließung; korrigiert wurden nur
  die unzutreffenden Samstagsöffnungszeiten.
- **AVB Autoverwertung Beck, Gerhardshofen:** Das Finding „keine unabhängige
  Zweitquelle“ übersah das aktuelle, extern ausgestellte Inspektionszertifikat.
  Das am 22.07.2026 nach § 5 Abs. 3 AltfahrzeugV ausgestellte Zertifikat nennt
  AVB/Uli Beck, Am Steinbruch 1, 91466 Gerhardshofen und den überwachten
  Demontagebetrieb; es gilt bis 22.08.2028, nächste Überwachung 22.08.2027.
  Unterzeichner ist laut Zertifikat ein öffentlich bestellter und vereidigter
  Sachverständiger der IHK Region Stuttgart. Damit sind Standort und aktuelle
  Fahrzeugverwertung unabhängig gestützt. Das Zertifikat belegt weder
  Metallankaufskurse noch allgemeine Schrottannahme. Der Nummernkonflikt
  09163 995557 / 09163 7272 bleibt offen; die im Betreiber-Impressum geführte
  Nummer wird nicht durch die abweichende Stellennummer ersetzt. Quelle:
  https://www.avb-beck.de/wp-content/uploads/2026/07/AVB_AF-Zertifikat_07-2026.png.
- Die offenen Identitäts-/Kontaktfragen bei Goldsofort Ahlen bleiben als solche
  dokumentiert; die unabhängige Registerkennung und der bevorzugte Kontakt
  gelten nicht als gelöst. Sie sind kein Beleg, dass die aktuelle
  Betreiber-Ankaufsseite geschlossen oder falsch ist.

## Validierung und Pre-Deploy-Freigabe

- Vier unabhängige Quellenreviews deckten die 323 strukturell geänderten
  Bestandszeilen und die 43 Seed-only-Slugs ab. Die acht beanstandeten
  Bestandsfelder wurden ownerseitig dispositioniert. Die drei Findings zu den
  neuen Filialen/Betrieben wurden geklärt: Augsburg/Aachen behalten den
  aktiven Filialstatus, ihre falschen Samstagszeiten sind entfernt; AVB Becks
  unabhängiges Zertifikat stützt den aktuellen Fahrzeugverwertungsbetrieb.
  Neun Fälle mit unzureichender Beleglage bleiben konservativ, ohne daraus
  Falschheit oder Schließung abzuleiten.
- Der vollständige Workspace-Test auf dem korrigierten Code-/Seedstand bestand:
  778 bestanden, 11 ignoriert, 0 fehlgeschlagen. Der fokussierte Seed-Test auf
  dem veröffentlichten Commit bestand mit 14/14. `cargo fmt --all -- --check`
  und `git diff --check` waren sauber.
- Code-/Seed-Commit `6b3da663f845d7527e284b8b075f3467935e5cd9` wurde regulär nach
  `origin/main` gepusht. Der exakte Release-Build dieses Commits bestand.
  SHA-256 Server:
  `a47b0e461817cee86b01f9bebf9a87d7f52a86161e3306e0fd379722c4ac4eaf`;
  Query-Worker:
  `8fc6429aeb55c4f83b2616d2fd3eb226118abcd8833d231658bd0289110ac67e`.
- Die private Release-Probe verwendete WAL-sichere SQLite-Backups aus
  `mode=ro`-Quellen, ausschließlich private Kopien und exakt diese Binaries.
  Sie migrierte Schema v6→v9: 3.893→3.936 Händler (43 Inserts, keine Deletes).
  Alle 83 Material- und 31.080 Preiszeilen blieben inhaltlich unverändert;
  `trader_materials` blieb bei 1.402 Zeilen. `integrity_check=ok`, die 24 bereits
  vorhandenen FK-Verletzungen blieben 24, kein Current-Zeiger ohne Preiszeile.
  Die 56 entfernten
  Current-Zeiger sind vollständig zugeordnet: 24 eingestellte Heraeus-Kurse und
  32 veraltete Wahl-&-Co.-Kurse; Preis-Historienzeilen blieben unverändert.
  Fünf Heraeus-Annahmen wurden auf `accepts=0` gesetzt.
- Der fachliche Vergleich zählte 323 geänderte Bestandszeilen mit 1.025
  Änderungen in strukturierten Feldern (ohne `notes`, `extra_json` und
  Zeitstempel). Zusätzlich änderten sich `notes` und `extra_json` jeweils bei
  1.280 Bestandszeilen. Alle 1.280 alten Notizen blieben vollständig und
  geordnet erhalten: 1.266 als Teilstrings, 14 als reine Einfügungs-
  Teilsequenzen; kein alter Text wurde gelöscht oder ersetzt. Alle
  `extra_json`-Änderungen betrafen ausschließlich Seed-Metadaten.
- Private Assertions bestätigten die Korrekturen für Fritsch, Elma, RoVa, RVE,
  Wismar, Tamm, Bochum/Wattenscheid, Rochlitz, Schopsdorf, Gerhardshofen,
  Augsburg und Aachen. Der Query-Worker las 12 Zielzeilen. Zwei private
  Serverstarts lieferten `/health` HTTP 200; Seed meldete 1.323 Writes und beim
  zweiten Start 0. Beide Prozesse endeten deutlich vor dem 15-Sekunden-
  Schedulerstart; es lief kein Scrape. Das vollständige Manifest liegt unter
  `/tmp/opencode/release-rehearsal-6b3da663-20261027T023523Z/manifest.json`.
- Damit ist die **Pre-Deploy-Freigabe für exakt den Code-/Seed-Commit
  `6b3da663f845d7527e284b8b075f3467935e5cd9` und die oben gehashten
  Release-Binaries grün**. Das ist keine Produktionskorrektur: ein regulärer
  Rollout, normaler Schedulerlauf und anschließender read-only Produktions-
  sowie Post-Rollout-Nachweis stehen noch aus. Bis zu diesem Bericht gab es
  keinen Produktions-Write, Force-Ingest, Service-Neustart oder Deployment.

## Frischer read-only Check-in, 27.10.2026 02:32 UTC

- Feedback-Tabelle endet weiterhin bei ID 7390 vom 13.10.2026; seit dem letzten
  Check-in ist kein neueres Feedback eingegangen. Die Tabelle enthält kein
  Triage-Statusfeld. Die Audit-7-Einträge werden nicht als neue ungeprüfte
  Meldungen ausgegeben.
- `schrott-mcp.service` ist aktiv. systemd meldet den Prozessstart am
  17.10.2026 16:49 UTC; bei dieser Prüfung wurde kein Dienstneustart ausgelöst.
  `journalctl -p warning` ist leer. Die sechs App-JSON-WARNs der letzten sechs
  Stunden betreffen einzelne Upstream-/Scrape-Probleme: Wirkaufendeingold ohne
  validierbare Tageskursreferenzen, Wahl & Co ohne Sortenpreisliste, A-Z mit
  verworfener verdächtiger Preisleiter, AMR mit HTTP 502, MKM mit ungültigem
  Source Contract und VHM mit Requestfehler. Die letzten Läufe #2868 und #2867
  waren `ok` (88 bzw. 116 Preise); #2866 war `partial` (61 Preise, ein
  Händlerfehler).
- Read-only Produktionsbestand: 3.893 Händler, 83 Materialien, 2.715 aktuelle
  Preiszeiger. Bestätigte Korrekturen sind dort noch nicht veröffentlicht:
  Fritsch hat weiter den falschen Punkt 51.90933/7.88825 bei leerer Adresse;
  Elma den alten Punkt 50.0167/10.4667 und die alte Telefonnummer; RoVa den
  alten Punkt 50.86395/12.08895 und die alte Telefonnummer; RVE den alten Punkt
  49.2516/6.8587 sowie `dropoff_json={"allowed":true}`. Wismar enthält weiter
  den falschen Punkt 53.88925/11.44455, und Tamm den alten, mehrere Kilometer
  entfernten Punkt 51.259513/7.164909. Die Seedkorrekturen bleiben bis zum
  regulären Rollout private Arbeitsdaten. Es gab keinen direkten
  Produktions-DB-Schreibzugriff.
- Freier Speicher vor der Probe: 46 GiB auf `/`, 1.0 GiB auf `/tmp`; ausreichend
  für die privaten Backups/Probe.

## Regulärer Rollout und Post-Rollout-Abgleich, 27.10.2026

### Rollout nach grüner Pre-Deploy-Freigabe

- Der freigegebene Seed-/Code-Commit `6b3da663f845d7527e284b8b075f3467935e5cd9`
  wurde am 27.10.2026 um **02:43:15 UTC** regulär installiert; `66bb43ff`
  enthielt danach nur den Freigabebericht, keine Seed-/Codeänderung. Die
  Produktionsdateien entsprechen den geprüften SHA-256-Werten aus dem
  Freigabeabschnitt. Das Backup bleibt unter
  `/root/Documents/schrott-mcp-rollout-backups/rollout-6b3da663-20261027T024237Z/`
  erhalten; `manifest.json` enthält Backup- und Installationsnachweise.
- `schrott-mcp.service` ist aktiv (PID 2195036, Start 02:43:15 UTC),
  `/health` liefert HTTP 200. Das Schema ist v9; read-only gezählt wurden
  3.936 Händler, 83 Materialien und 2.659 aktuelle Preiszeiger.
- Der Boot-Seed meldete 1.323 Writes. Der natürliche Scheduler-Lauf #2869
  endete um 02:43:32 UTC mit `ok`, 0 Preisen und 0 Händlerfehlern. Es gab
  keinen Force-Ingest und keinen direkten Produktions-DB-Schreibzugriff.
- Read-only bestätigt: Fritschs Adresse/Telefon/Punkt sind korrigiert; Elma,
  RoVa und RVE haben keine falschen Koordinaten mehr; RVE hat außerdem leere
  Annahmebedingungen. Der RWE/ALBA-Nord-Slug in Wismar hat keinen Punkt mehr,
  Tamm den geprüften Punkt 51.2247867/7.2141755, Bochum Plus leere
  Beschreibung/Abholung und `pruefung`, Rochlitz leere Straße/PLZ und Brameier
  `pruefung`. Augsburg/Aachen sind aktiv und haben werktägliche Zeiten ohne
  falsche Samstagsöffnung. Diese Einzelwerte sind keine Aussage, dass alle
  Post-Rollout-Gates bestanden sind.

### Weitere bestätigte falsche GEO-Punkte, Kandidatenkorrektur noch nicht ausgerollt

- Der separate Wismar-Datensatz `mv-wismar-alba-metall-nord` enthält weiterhin
  53.8895333/11.4386667. Nominatim löst den Punkt als Gebäude Am Schwedenstein
  13, Wismar 23966 auf, nicht als Tonnenhofstraße 12, 23970. Die Dossierhistorie
  belegt keine aktuell eigenständige ALBA-Metall-Nord-Annahmestelle und keinen
  exakten Ersatzpunkt. `clear_fields: geo` wurde ergänzt; Name, Status und
  Sibling-Zuordnung bleiben unangetastet. Kein Merge und keine Schließung
  behauptet.
- `nw-bochum-wattenscheid-schrotthandler-plus` enthält 51.4740889/7.2192389;
  Nominatim löst das als Wohngebäude Taubenstraße 5, Bochum 44789 auf. Für den
  mobilen Anbieter ist dort kein Hof belegt. `geo` ist zusammen mit den bereits
  zurückgenommenen Claims in `clear_fields` aufgenommen; kein Ersatzpunkt.
- Beide Korrekturen stehen nur in den aktuellen Dossiers/Seeds. Sie sind nicht
  im ausgerollten Commit enthalten und wurden nicht in Produktion geschrieben.
  Gezielte Tests: Seed-Trader 11/11, Build-Codegen 10/10; generierte Seeds
  enthalten die erwarteten Clear-Direktiven. Vor einem späteren Rollout braucht
  der geänderte exakte Stand erneut die vollständige Pre-Deploy-Freigabe.

### Post-Rollout-Verifier: rot, daher kein Erfolgsabschluss

Alle drei Skripte wurden read-only mit tatsächlicher Rollout-Grenze
`2026-10-27T02:43:15Z` ausgeführt. Wegen der roten Resultate wird der Rollout
nicht als vollständig erfolgreich gemeldet; weitere Produktionsänderungen
unterbleiben.

- `verify-mkm-release.py`: **rot**, 60 Artikel, 55 numerische Preise, 5
  Anfragen, 55 aktuelle Preise. Findings: Quote/Request-Partition 55/5 statt
  56/5, Manifest-Scope/Freshness, Quote-Provenienz/Datum/Freshness,
  geprüfte 61-Artikel-Partition und erfolgreicher 56-Preis-Post-Rollout-Step
  fehlen. Im Fenster seit dem Rollout gibt es keinen MKM-Step; #2869 war kein
  MKM-Lauf. Kein Handler wurde manuell ausgelöst.
- `verify-pending-corrections.py`: **rot**, 124 MSG-verwaltete Quotes. Findings:
  Die Produktionsfelder von Gutzmann stimmen aktuell mit `An der Ostbahn 4`,
  `99867`, Gotha überein; `updated_at` ist jedoch vom 26.10.2026 und damit vor
  dem Rollout. Der bisherige kombinierte Verifiertext meldete dafür irreführend
  „branch address incorrect“. Die drei Clear-Fälle sind in Produktion ebenfalls
  leer, haben aber `updated_at` vom 17.10.2026 und daher keinen frischen
  Seednachweis. Der Verifier trennt jetzt Wertfehler und fehlenden
  Post-Rollout-Nachweis; Regressionstests 8/8. Übrig bleiben MSG-Quelle/Freshness,
  die vier fehlenden Zeitnachweise und erfolgreiche Post-Rollout-Steps für MSG
  sowie Gutzmann. Im Fenster nach dem Rollout liegt kein Step dieser Handler.
- `verify-wave-e-postrollout.py`: **rot**, 60/60 Slugs geprüft, 75 Findings:
  50 Timeline-Hash-Abweichungen, 20 fehlende `updated_at`-Zeitstempel nach dem
  Rollout, vier Straßen-/PLZ-Zielabweichungen sowie eine
  Beschreibungsabweichung. Read-only-Vergleich mit den aktuellen Dossiers ergibt,
  dass alle fünf skalaren Abweichungen zum älteren 11.10.-Manifest passen:
  Bochum Plus hat leere Straße, Volz die vorsichtigere aktuelle Beschreibung,
  Kaminski & Sohn Auerbacher Straße 70A/08107 und Zauritz nur die gestützte PLZ
  07557. Produktion stimmt in diesen Feldern mit dem derzeitigen Seed überein;
  die alte Manifest-Erwartung ist dafür überholt. Die Timeline- und
  Zeitstempelfindings bleiben Gate-Ergebnisse und sind kein GEO- oder Quellen-
  Nachweis. Der Verifier validiert selbst weder GEO noch Quellen.

### Check-in-Health und Feedback

- Feedback endet bei ID 7390 vom 13.10.2026; kein neueres Feedback. Die Tabelle
  hat kein Triage-Statusfeld. `journalctl -p warning` blieb in den letzten sechs
  Stunden leer. Sechs ältere App-JSON-WARNs in diesem Fenster betreffen
  Wirkaufendeingold (Tageskursreferenzen), Wahl & Co (Sortenpreisliste), A-Z
  Recycling (verdächtige Preisleiter verworfen), AMR (HTTP 502), MKM (ungültiger
  Source Contract) und VHM (Requestfehler); keine App-WARN nach dem Rollout.
- Keine neuere Geocoding-Logdatei unter `/tmp/opencode/*.log` gefunden.

## Read-only-Nachlauf 27.10.2026 03:00 UTC

- Der nächste normale Scheduler-Lauf #2870 endete um 02:58:39 UTC mit `ok`,
  88 Preisen und 0 Händlerfehlern. Service weiter aktiv, `/health` HTTP 200;
  Produktionsbestand unverändert bei v9, 3.936 Händlern, 83 Materialien und
  2.659 Current-Preiszeigern. Feedback bleibt bei ID 7390.
- Die drei Post-Rollout-Verifier wurden nach #2870 erneut read-only ausgeführt
  und sind unverändert rot. Seit Rolloutstart gibt es weiterhin keinen MKM-,
  MSG- oder Gutzmann-Step. Es gab keine direkte DB-Änderung und keinen
  Force-Ingest.
- Laut stabilem 6h-Phasenplan des laufenden Binaries liegen die nächsten
  erwarteten Due-Phasen nach dem ersten Scheduler-Poll bei MSG ca. 04:06:50,
  MKM ca. 05:16:59 und Gutzmann ca. 07:57:46 UTC; tatsächliche Ausführung erst
  beim jeweils nächsten normalen Scheduler-Poll. Danach die read-only Gates
  erneut prüfen, ohne einen Handler manuell auszulösen.
- Beide zuletzt bestätigten Fehlpunkte (Wismar-Sibling und Bochum Plus) sind
  weiterhin in Produktion vorhanden; ihre Seed-Clears sind noch nicht
  ausgerollt. Der rote Post-Rollout-Status bleibt Grund, keine weiteren
  Produktionsänderungen vorzunehmen.

## Read-only-Nachlauf 27.10.2026 03:32 UTC

- Die normalen Scheduler-Läufe #2871 (03:13:39–03:13:46 UTC, 204 Preise) und
  #2872 (03:28:46–03:28:54 UTC, 54 Preise) endeten jeweils `ok` mit null
  Händlerfehlern. Dienst aktiv, `/health` auf `127.0.0.1:4001` HTTP 200,
  Schema v9 und Bestand unverändert bei 3.936 Händlern, 83 Materialien und
  2.659 aktuellen Preiszeigern. Feedback bleibt bei ID 7390 vom 13.10.
- Nach #2872 bleiben die drei read-only Post-Rollout-Verifier rot. MKM hat
  weiterhin 60 Artikel (55 numerisch/5 Anfragen) statt der geprüften
  61-Artikel-/56-Preis-Partition; ein erfolgreicher Post-Rollout-Lauf fehlt.
  Beim Pending-Corrections-Gate bleiben 124 MSG-Quotes vorhanden, aber
  Quellen-/Freshness-Befund, vier fehlende Seed-Zeitnachweise und die
  erfolgreichen Post-Rollout-Läufe für MSG/Gutzmann fehlen. Wave E prüft 60/60
  Händler und hat 75 Findings: 50 Timeline-Hashes, 20 fehlende
  Post-Rollout-Zeitstempel und fünf skalare Abweichungen zur älteren
  11.10.-Erwartungsdatei (zwei Straße-, zwei PLZ- und eine
  Beschreibungsabweichung). Keine manuelle Handler-Ausführung.
- Read-only bestätigt weiterhin die zwei falschen GEO-Punkte: der Wismar-Sibling
  hat Tonnenhofstr. 12/23970, aber 53.8895333/11.4386667; Bochum Plus hat
  keinen belegten Straßenstandort, aber 51.4740889/7.2192389. Beide geprüften
  `clear_fields: geo`-Korrekturen sind in der gepushten Seed-Historie, nicht in
  Produktion. Wegen der roten Post-Rollout-Gates keine weitere
  Produktionsänderung und kein Deployment.
- `journalctl -p warning` bleibt leer; fünf App-JSON-WARNs im Sechs-Stunden-
  Fenster liegen sämtlich vor dem Rollout, keine danach. Unter
  `/tmp/opencode/*.log` gibt es keine neuere Geocoding-Logdatei. Kein direkter
  Produktions-DB-Schreibzugriff und kein Force-Ingest.

## Read-only-Nachlauf 27.10.2026 04:03 UTC

- Normale Scheduler-Läufe #2873 (03:43:54–03:44:02 UTC, 46 Preise) und #2874
  (03:59:02–03:59:05 UTC, 30 Preise) endeten `ok`, jeweils ohne
  Händlerfehler. Feedback weiterhin maximal ID 7390. Dienst aktiv,
  `/health` HTTP 200; Schema v9 und Produktionsbestand unverändert.
- Seit dem Rollout um 02:43:15 UTC gibt es weiterhin keinen MKM-, MSG- oder
  Gutzmann-Handler-Step. Die drei read-only Post-Rollout-Gates liefern
  unverändert rot: MKM 60/61 Artikel und kein 56-Preis-Step; Pending Corrections
  mit 124 MSG-Quotes, aber ohne erforderliche frische Quell-/Seed-Nachweise und
  Handler-Steps; Wave E weiterhin 75 Findings (50 Timeline-Hashes, 20
  Zeitstempel, fünf skalare Abweichungen zum alten Manifest). Die nächste
  natürliche MSG-Phase wird ungefähr ab 04:06:50 UTC erwartet, tatsächliche
  Ausführung erst mit regulärem Poll; keine Handler manuell gestartet.
- Die zwei falschen GEO-Werte sind in Produktion weiterhin unverändert. Für
  den Korrekturstand sind jetzt `cargo test --workspace --locked` (778 bestanden,
  11 ignoriert), `cargo fmt --all -- --check`, Seed-Generator-Tests 10/10 sowie
  die drei Offline-Verifier-Testsuiten 8/8, 11/11 und 11/11 grün. Das ersetzt
  nicht die noch offene private Upgrade-/Datenübergangsprobe für den exakten
  Korrekturstand und ist keine Pre-Deploy-Freigabe. Kein Deployment und keine
  Produktionsänderung, solange die Post-Rollout-Gates rot sind.
- `journalctl -p warning` bleibt leer. Die fünf App-JSON-WARNs im Sechs-Stunden-
  Fenster liegen vor dem Rollout; danach keine. Keine neuere
  `/tmp/opencode/*.log`-Geocoding-Datei, kein direkter Produktions-DB-Write und
  kein Force-Ingest.

## Read-only-Nachlauf 27.10.2026 04:19 UTC

- `main` stand bei `71d561aac044bd5794b47954c5d34cdb9556e7fd`, identisch mit
  `origin/main`; der geschützte GitHub-Credential-Helper-Dry-Run war erfolgreich.
  Der Dienst ist aktiv, `/health` liefert HTTP 200. Read-only: Schema v9,
  3.936 Händler, 83 Materialien, 2.659 aktuelle Preiszeiger. Feedback endet bei
  ID 7390 vom 13.10.; kein neuerer Eintrag.
- Der letzte Scheduler-Lauf #2875 (04:14:05–04:14:13 UTC) war `partial` mit
  202 Preisen und zwei erwarteten Handlerfehlern: A-Z Münster verwarf die
  verdächtige Preisleiter ohne Veröffentlichung; AMR antwortete HTTP 502. Der
  normale MSG-Schritt #14281 lief innerhalb von #2875 erfolgreich (124 Preise).
  `journalctl -p warning` ist leer; im App-JSON gibt es sieben WARNs im
  Sechs-Stunden-Fenster, darunter die beiden Fehler aus #2875. Seit 25.10. liegt
  keine neuere Geocoding-Logdatei unter `/tmp/opencode/*.log`.
- Die drei read-only Post-Rollout-Verifier bleiben rot, jeweils mit Grenze
  `2026-10-27T02:43:15Z`: MKM hat 60 Artikel (55 numerische/5 Anfragen) und
  keinen geprüften 56-Preis-Step; Pending Corrections hat 124 verwaltete
  MSG-Quotes, aber vier fehlende Post-Rollout-Seed-Zeitnachweise (Gutzmann und
  drei Clear-Zeilen) sowie keinen erfolgreichen Gutzmann-Step; Wave E prüft
  60/60 mit 75 Findings (50 Timeline-Hashes, 20 Zeitstempel, fünf skalare
  Abweichungen zum explizit auf den Seed-Stand vom 11.10. bezogenen Manifest).
  Der MSG-Schritt ist nun natürlich erfolgt, beseitigt aber nicht die übrigen
  Gates. Kein Handler wurde manuell gestartet.
- Produktion enthält weiterhin Wismars Punkt `53.8895333/11.4386667` trotz
  Dossieradresse Tonnenhofstr. 12/23970 sowie Bochum Plus’ Punkt
  `51.4740889/7.2192389` ohne belegten festen Standort. Beide `clear_fields:
  geo`-Korrekturen sind im gepushten Seed, aber nicht ausgerollt. Rochlitz führt
  in Produktion nach normalem Lauf #2873 erneut Straße/PLZ Schützenstr. 6/09306
  und Punkt `51.0476634/12.8084764`; der Punkt identifiziert das Gebäude, nicht
  unabhängig DB Recycling als dortige Annahmestelle. Commit `71d561aa` zieht
  auch diesen ungestützten Geo-Punkt per regulärem Seed zurück; der Handler
  veröffentlicht aus seinem alleinigen Impressum keine Anschrift mehr. Keine
  direkte Produktionsänderung.
- Der private WAL-sichere Upgrade-/Seed-Rehearsal für exakt `71d561aa` bestand;
  die Binaries hatten SHA-256 `0f06c4dd53fde9bca4cceb82a12806232606676c8087e4af4029ca2562c68dc4`
  (Server) und
  `8fc6429aeb55c4f83b2616d2fd3eb226118abcd8833d231658bd0289110ac67e`
  (Query-Worker). Workspace: 779 bestanden, 11 ignoriert; `cargo fmt --all --
  --check` und Build-Seed-Tests 10/10 bestanden. Das ist private Kandidaten-
  Evidenz, kein Produktionsnachweis. Die unabhängigen GEO-Reviews laufen noch;
  außerdem bleiben die Post-Rollout-Verifier rot. Daher kein Deployment und
  keine weiteren Produktionsänderungen.

## Owner-Disposition unabhängiger GEO-Reviews 27.10.2026 04:30 UTC

- Das unabhängige Review zu Wismar und Bochum Plus bestätigt die zwei klar
  falschen Punkte. Für beide bleibt `clear_fields: geo` ohne Ersatzpunkt richtig;
  weder Schließung noch Merge wird abgeleitet. Das Wismar-Review fand zugleich
  ein direkt geprüftes ALBA-CCF-Zertifikat vom 29.11.2024, das für 2023 ALBA
  Metall Nord und ALBA Nord getrennt an Tonnenhofstraße 12 listet. Das widerlegt
  eine zu einfache Namenszuordnung, beweist aber keine aktuelle separate
  Annahmestelle. Die beiden Wismar-Dossiers bleiben getrennt; die Dossier-
  Timeline hält diese Gegen-Evidenz nun fest.
- Für DB Recycling ist die Betreiber-Website stärker als zuvor formuliert:
  sie nennt Schützenstraße 6 ausdrücklich „Schrottplatz 09306 Rochlitz“ und
  unterstützt somit die Betreiberbehauptung, nicht aber eine unabhängige
  Bestätigung der konkreten Annahmestelle. Die Koordinate `51.0476634/12.8084764`
  reverse-geokodiert tatsächlich auf das Gebäude Schützenstraße 6; sie ist
  daher nicht als Gebäude-Punkt falsch, aber ihre Zuordnung zu diesem
  Einzelunternehmen bleibt nach README-Quellenhierarchie ein Einzelbeleg.
  SMS Sterns und DB Recycling teilen Denis Blum, Anschrift, Telefon/Fax/Mobil
  und Steuerkennungen; die Websites zählen nicht als unabhängige zweite Quelle.
  Keine Koordinate wird zwischen den Dossiers übertragen.
- Zusätzlich ist SMS Sterns’ gespeicherter Punkt `51.0501/12.7975` eindeutig
  fehlplatziert: Nominatim reverse-geokodiert auf Pflegeheim Rochlitz,
  Mathesiusstraße 3; die Suche nach Schützenstraße 6 findet ein anderes
  Gebäude bei `51.0476634/12.8084764` (rund 814 m entfernt). Dieser Geocoder-
  Punkt belegt keine SMS-Annahmestelle; auch die Betreiberseite von DB Recycling
  publiziert `51.0500000/12.8000000` nur im Stadt-Infoblock, nicht als Eingang.
  `dossiers/sn/sn-rochlitz-09306-sms-sterns-metall-schrotthandel.md` enthält
  jetzt `clear_fields: geo`, ohne geratenen Ersatz, Merge oder Statusänderung.
- Der reguläre Lauf #2876 endete um 04:29:15 UTC `ok` mit 85 Preisen und null
  Händlerfehlern. Feedback endet weiterhin bei ID 7390; Dienst aktiv,
  `/health` HTTP 200, Schema v9 und Bestand 3.936/83/2.659. Die drei
  Post-Rollout-Verifier bleiben nach erneuter read-only Prüfung rot: MKM 60
  Artikel/55 numerische/5 Anfragen; Pending Corrections 124 MSG-Quotes, vier
  fehlende Seed-Zeitnachweise und kein Gutzmann-Step; Wave E 75 Findings.
  Der natürliche MSG-Step #14281 aus #2875 bleibt `ok` mit 124 Preisen.
  `journalctl -p warning` ist leer; sieben App-WARNs im Sechs-Stunden-Fenster,
  darunter A-Zs verworfene Preisleiter und AMRs HTTP 502 aus #2875.
- Die geänderten Dossiers und die neue SMS-GEO-Rücknahme sind Arbeitskopie,
  nicht Produktion. Auf diesem exakten Seedstand bestanden
  `cargo test --workspace --locked` mit 779 Tests (11 ignoriert), die
  fokussierten Seed-Tests 11/11, die Build-Seed-Tests 10/10,
  `cargo fmt --all -- --check` und der Release-Build. Server-Binary SHA-256:
  `11094416cd2df35791c1c3f2c6cb68825a1d5ad3654171940905170b2390b06d`;
  Query-Worker unverändert `8fc6429aeb55c4f83b2616d2fd3eb226118abcd8833d231658bd0289110ac67e`.
  Die private Upgrade-/Datenübergangsprobe für diesen erweiterten Stand und der
  frische Push stehen noch aus. Keine direkte Produktionsänderung, kein
  Force-Ingest und kein Deployment.
