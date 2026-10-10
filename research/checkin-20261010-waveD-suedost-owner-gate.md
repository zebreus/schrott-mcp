# Owner-Gate Südost, Wave D — 10.10.2026

## Owner-Abnahme nach Übergabe

Alle 20 Main-Diffs vollständig gelesen; drei Pflichtkorrekturen bestätigt.
Anschließend zusätzlich die im Review dokumentierten Gegenbelege für
Schlereth, SMG Gera und den aktuellen WRS-FAQ-Befund als datierte Timeline-
Notizen ergänzt. Dadurch weichen nun sechs statt drei Dossiers von der
unveränderten Deploy-Quelle ab; die nachstehende Agentenübergabe beschreibt
den früheren Stand. Keine weiteren Frontmatter-Fills.

MV-Kontaktseite zusätzlich selbst direkt gelesen: Adresse, Telefon, Mail
und Annahmezeiten stimmen mit neuen Skalaren überein. Statischer Gate für
genau 20 Dateien bestanden: vollständige bisherige Timeline erhalten,
Slugs/GEO unverändert, JSON lesbar, keine Emdash-Platzhalter.
`cargo test --locked -p schrott-mcp-ingestion seed`: 8 bestanden, 0 Fehler;
`git diff --check` sauber. Freigabe dieser eingeschränkten Deltas zum lokalen
Main-Commit, keine Bestätigung ungeklärter Legacyidentitäten.
Push und Produktions-Redeploy bleiben wegen fehlender GitHub-Anmeldung offen.

## Übernahmestand Main — 10.10.2026

**Die genau 20 geprüften Südost-Dossier-Deltas sind in `/root/Documents/schrott-mcp` auf Branch `main` zur Owner-Abnahme vorbereitet. Alle drei angeforderten Korrekturen sind umgesetzt.** Keine Abnahme oder technische Seed-Freigabe vorweggenommen; Gesamtdiff und Tests bleiben beim Owner.

### Baseline-/Konfliktprüfung vor Übernahme

- Main-HEAD: **`48e59e60a91777da8b94fa81a0986ede6028a8bd`**; Quellbaseline: **`072cf25cee749795e104624daacc2608411e84bd`**.
- Vor jedem Edit alle 20 Main-Dateiinhalte gegen die Quellbaseline verglichen: **20/20 textgleich**; auch die committed Main-HEAD-Versionen stimmen mit der Baseline überein. Main hat seit der Baseline andere Arbeit aufgenommen, jedoch **keine eigenen Änderungen an diesen 20 Dateien**. Keine Zielkonflikte, kein Main-Inhalt musste verworfen werden.
- Main war für getrackte Dateien vor Beginn sauber; allein dieser bereits angelegte Bericht war ungetrackt. Quelländerungen als gezielte Frontmatter-/Überblick-/Timeline-Hunks mit `patch` übernommen, kein kompletter alter Repo-/Dossierstand kopiert.
- Quelle `/root/Documents/schrott-mcp-deploy` ausschließlich gelesen, dort keinerlei Edit, Commit oder andere schreibende Aktion durchgeführt. Wellenbericht selbst nicht nach Main kopiert.

### Bewusste Abweichungen von der Quelle

**17 Dossiers sind vollständig textgleich mit ihren aktuellen geprüften Quellversionen. Genau drei unterscheiden sich durch die angeforderten Korrekturen:**

| Main-Dossier | Korrektur / Provenienz |
|---|---|
| `dossiers/sn/sn-mobil-leipzig-schrottjungs-schrottabholung-leipzig.md` | Laufender Preisüberblick: „Hartmetall/Wolframkarbid“ → **„Hartmetall / Wolfram“**. Verkürzte Namen in derselben Liste ebenfalls auf Originalpositionen „Mischschrott schwer“, „Kupfer Millberry“, „Kabel (diverse)“, „Aluminium (div.)“, „Edelstahl V2A“ gebracht; alle Werte unverändert. Neue `### Korrektur 10.10.2026` mit Gewerbequellenbeleg, Großmengen-/Obergrenzen-/Datumsabgrenzung. Kein Frontmatter-/Handlerdelta zusätzlich zur Quelle. |
| `dossiers/sn/sn-heidenau-tsr-deutschland-nl-dresden-heidenau.md` | Aktueller Überblick beschreibt belegte Anlage Dresden-Heidenau, grenzt die auswertbare Dresden-Seite als **Hafen** ab und dokumentiert **Veolia als separaten Betreiber am selben Adresskomplex**, ausdrücklich ohne Nachfolgebehauptung. Keine fremden Zeiten/Kontakte übernommen. Neue `### Korrektur 10.10.2026` mit Zertifikat, Impressum, Hafen- und Veolia-Quellen. Frontmatter/Ort/Adresse/Slug/GEO unverändert. |
| `dossiers/bw/bw-lorrach-mv-lorrach.md` | `dropoff_json.conditions`: fehlende Zahlenpreise/bezifferte Mindestmenge ausschließlich **im geprüften Abrufumfang**. Passende Begrenzung auch im laufenden Überblick; neue `### Korrektur 10.10.2026` mit Betreiberbelegen. Alle übrigen Quell-Stammdaten-/Servicefüllungen unverändert. |

Keine zusätzlichen neuen Recherche-/Kontaktfüllungen für Schlereth, SMG, WRS oder andere Prüffälle vorgenommen; die im ursprünglichen Audit dokumentierten weiterführenden Hinweise bleiben im Bericht. Keine andere Tranche, kein Handler, keine Geodaten, keine Slugs bearbeitet.

### Read-only Abschlussaudit des vorbereiteten Ergebnisses

- `git diff --name-only` entspricht **exakt der 20-Pfade-Menge** aus der untenstehenden Einzelentscheidungstabelle; keine weiteren getrackten Dateiänderungen. Dieser ungetrackte Bericht ist zusätzlich und muss beim Owner-Gesamtreview ausdrücklich mitgelesen werden.
- Dossier-Diff gegen Main-HEAD: **20 Dateien, 126 hinzugefügte / 33 entfernte Zeilen**. Entfernte Zeilen betreffen ersetzte Frontmatterwerte/Überblicke, nicht historische Timeline.
- **20/20 Baseline-Timelines** und **20/20 vollständige aktuelle Quell-Timelines** sind als unveränderte zusammenhängende Zeichenfolgen im Main-Ergebnis enthalten. Die drei Korrekturen vom 10.10. sind vorangestellt; keine alte Recherche oder Gegenbelege umgeschrieben.
- **20/20 Slugs und GEO-Feldzustände** gegenüber Baseline und Quelle unverändert. Frontmatter zwischen Quelle und Main nur beim MV-Bedingungstext unterschiedlich.
- Sämtliche nichtleeren Bedingungs-JSONs mit `json.loads` lesbar. `git diff --check` ohne Befund. Dies sind Text-/Integritätsprüfungen, **kein Build, kein Rust-/Seedtest und keine DB-Wirkungsprüfung**.
- Keine Commits, Builds, Deployments oder Datenbankzugriffe ausgeführt. Owner liest den Gesamtdiff und führt `cargo test -p schrott-mcp-ingestion seed` aus; Veröffentlichung/Produktionsprüfung bleiben spätere, gesondert autorisierte Schritte.
- Beim abschließenden `git status` zusätzlich sichtbare fremde, ungetrackte Datei `research/checkin-20261010-kernschrott-independent-review.md`: nicht von dieser Übernahme erstellt, nicht gelesen oder bearbeitet und nicht Teil des 20-Dossier-Pakets. Parallele Arbeit bleibt erhalten.

## Ursprüngliches Recherche-Ergebnis vor Main-Übernahme

**Fachlich weitgehend integrationsfähig, aber keine uneingeschränkte Gesamtfreigabe.** Die drei Rückstufungen auf `pruefung`, Website-Status-Normalisierungen und neuen MV-Lörrach-Stammdaten entsprechen dem README-Standard. Vor Gesamtintegration müssen der weiterhin falsche Schrottjungs-Sortenname im kuratierten Überblick und der überholte TSR-Überblick korrigiert werden. Bei MV sollte die negative Preis-/Mindestmengenaussage auf den tatsächlichen Abrufumfang begrenzt werden. Offene Identitäten bleiben ausdrücklich offen; keine zusätzlichen Kontakte, Betreiberketten, Koordinaten oder Händlerpreise freigeben.

**Kein Deployment-/CI-Gate:** Builds, Tests, Commits und Deployments wurden auftragsgemäß nicht ausgeführt. Keine Produktionsdatenbank geöffnet. Dossiers und Deploy-Worktree ausschließlich gelesen; allein diese Reportdatei wurde geschrieben. Ein nach Skill vorgesehenes weiteres Recherche-Subagent ließ sich wegen des Subagent-Tiefenlimits nicht starten; sämtliche nachfolgend als direkt geprüft bezeichneten Quellen wurden selbst abgerufen.

## Prüfgrundlage und Dateiaudit

- Worktree: `/root/Documents/schrott-mcp-deploy`, Vergleich gegen `HEAD` **`072cf25cee749795e104624daacc2608411e84bd`**. Prüfobjekt: `research/wave-20261009d-suedost.md` und ausschließlich die darin genannten 20 Dossiers.
- Alle 20 aktuellen Frontmatter-/Überblickblöcke und ihre vollständigen Dossier-Diffs gegen HEAD gelesen. Für **20/20** ist der komplette bisherige Text nach `## Timeline` als unveränderte zusammenhängende Zeichenfolge im neuen Timeline-Inhalt enthalten. Das bestätigt Erhalt der bisherigen Historie, nicht die historische Richtigkeit aller darin enthaltenen Behauptungen.
- 20 eindeutige Pfade; keine Slugänderung, kein neuer GEO-Wert, keine neue Umbenennungs-/Mergekette. Alle nichtleeren `dropoff_json`/`pickup_json` dieser 20 Dateien mit Python `json.loads` erfolgreich gelesen. Das ist ausdrücklich kein Seed-Compiler-/Schema-/Idempotenztest.
- Tatsächliche Frontmatter-Deltas: **acht** `website_status`-Änderungen (sieben leer → `unbekannt`, Denk Zeil `tot` → `unbekannt`), **drei** `status: aktiv` → `pruefung`; **acht** bisher leere MV-Felder gefüllt. Insgesamt elf Dossiers mit Frontmatteränderungen (Spindler hat zwei); die übrigen neun haben keinen Frontmatter-Delta. Schrottjungs und TSR ändern aktuell nur Timeline-Text.
- README maßgeblich: Z. 98–121 (Frontmatter/Timeline als Buildinput, stabile Slugs), 148–182 (zweiseitige Identitätsprüfung, Aggregatoren nur Leads, zwei unabhängige zugelassene Quellen beziehungsweise qualifizierte Betreiber-Primärquellen-Ausnahme), 194–220 (Betreiberketten und Owner-Gate). Betreiber-Impressum allein beweist keine Dossieridentität; Registerlöschung beweist keinen geschlossenen Nachfolgerhof.

## Konkrete Freigaben

### 1. MV Lörrach: Stammdaten übernehmen

Datei: `dossiers/bw/bw-lorrach-mv-lorrach.md`.

Direkt am 10.10.2026 bestätigt:

| Feld/Befund | Verifizierter Wert / Beleg |
|---|---|
| Betreiber | Metall- und Rohstoffverwertung Lörrach GmbH; Impressum Freiburg HRB 412919 [M1] |
| Registerkongruenz | Northdata gleicher Rechtsträger/HRB, Schwarzwaldstr. 48–50, Gegenstand Metallhandel/-verwertung; Veröffentlichung Jahresabschluss 2025 am 17.08.2026 [M7] |
| Kundenkontakt | Schwarzwaldstr. 44–50, 79539 Lörrach; +49 7621 43940; info@mv-loerrach.de [M2] |
| Annahmezeiten | Mo–Fr 07:30–11:40, Mo–Do 13:00–16:40, Fr 13:00–16:00 [M2] |
| Privatanlieferung | Metall zu Tagespreisen; größere Mengen bei Bedarf abholbar [M3] |
| Altautoabholung | Nicht fahrtüchtige Fahrzeuge mit Spezialfahrzeug abholbar [M4] |
| Container | Absetzwannen ab 5 cbm, Abroller bis 40 cbm, individuelles Konzept [M5] |
| Aktualität/Zertifikat | EfB 2026-0191, ZN ZertifizierungsNetzwerk GmbH, Prüfung 14.04.2026, Ausstellung 08.07.2026, gültig bis 13.10.2027; HRB 412919 und Standort Schwarzwaldstraße 46–50 [M6] |

**README-Ausnahme erfüllt:** registerkongruentes Impressum, aktueller Betreiber/Registerbefund, lokale Kontakt-/Zeitenseite und aktuelles Zertifikat. Die drei Hausnummernbereiche sind transparent dokumentierte Varianten, kein Anlass für erfundene Tor-Koordinaten. Der doppelte `.de.de`-Mailtext im Impressum ist tatsächlich vorhanden; korrektes `mailto` und Kontaktseite rechtfertigen die übernommene einfache `.de`-Adresse.

Die acht neuen Frontmatterwerte sind fachlich tragfähig. `pickup_json.allowed: true` ist durch die ausdrücklich begrenzten Anwendungsfälle im Bedingungstext eingehegt; weder pauschal kostenlose Abholung noch allgemeine jederzeitige Annahme ergänzen. Kundenzeiten werden zutreffend als **Annahmezeiten**, nicht Bürozeiten geführt.

**Kleine Korrektur vor Gesamtintegration:** In `dropoff_json.conditions` ist „keine veröffentlichte Zahlenpreisliste oder Mindestmenge“ zu absolut. Integrationstauglicher Ersatz bei unverändertem JSON-Aufbau:

```yaml
dropoff_json: '{"allowed":true,"customer_types":["privat","gewerbe"],"conditions":"Metallanlieferung zu Tagespreisen während der Annahmezeiten; im geprüften Abrufumfang keine Zahlenpreisliste oder bezifferte Mindestmenge gefunden. Material und Sonderabfälle vorab klären."}'
```

Ankauf: Tagespreise ohne Zahlenfund auf den geprüften Seiten. Verkauf und Gebühren: dort kein verifizierter Zahlentarif; fehlender Fund bedeutet weder erwiesene Nichtveröffentlichung noch kostenlosen Service.

### 2. Becke, Anas, Spindler: Rückstufungen unverändert übernehmen

- `dossiers/sn/sn-chemnitz-becke-jens-buntmetallhandel.md`: `aktiv` → `pruefung`. Creditreform-Direktabruf auch am 10.10. HTTP 403 [B1]; Suchergebnis bestätigt lediglich die bestehende Creditreform-Spur. Zusätzliche Telefonbücher sind keine zweite qualifizierte Quelle. Keine neuen Zeiten aus widersprüchlichen Portalen.
- `dossiers/sn/sn-leipzig-anas-metall-leipzig.md`: `aktiv` → `pruefung`. Eigene Namens-/Adresssuche ergab wieder Portal-/Kartenleads, aber keinen zurechenbaren Betreiber. Keine Übernahme von Portalzeiten, 100-kg-Eisen-/1-kg-NE-Schwellen oder Kontakten. Automatisches „geschlossen“ darf nicht mit Geschäftsaufgabe verwechselt werden.
- `dossiers/bw/bw-albstadt-spindler-karl-schrotthdl.md`: `aktiv` → `pruefung`, leerer Website-Status → `unbekannt`. Die dokumentierte Aggregator-Aktivierung reicht nicht; bestehende Adresse/Telefon bleiben als unbestätigte Legacywerte erhalten, nicht als aktuell bestätigt etikettieren.

Das sind Korrekturen der Belegqualität, keine Schließungsbefunde.

### 3. Website-Status und WRS

Alle acht Website-Statusänderungen sind integrationsfähig. Insbesondere `dossiers/by/by-zeil-am-main-97475-denk-av.md`: ohne zugeordnete Domain ist `tot` unbegründet; `unbekannt` ist korrekt. Die Kandidatenseite ist tatsächlich per HTTP lesbar und nennt Brigitte Denk/Wildgarten 5/Kontakte [D1], beweist aber weder die unspezifische Seedzeile noch ihre Gleichheit mit Elfriede-/Fränklin-/Haßfurt-Spuren. Daher **keine** Website-/Kontaktfüllung oder Dublettenentscheidung.

`dossiers/th/th-gotha-wrs-wertstoff-recycling-service.md`: bisherigen `geschlossen`-Status des exakten Rechtsträgers erhalten. Northdata bestätigt direkt Liquidation 09.02.2011 und Löschung 31.07.2014 [W1]; der zweite Registerspiegel nennt gelöscht/HRB 500985 und die historische Schäferstraße 36 [W2]. Sein aktuell gelesener FAQ-Text sagt ebenfalls „Nein … gelöscht“, nicht mehr „seit 19 Jahren aktiv“. Die Notiz vom 09.10. kann als datierte damalige Beobachtung bleiben; daraus keinen heute noch vorhandenen Quellenwiderspruch behaupten. Spiegel stammen aus Registerdaten und sind keine zwei unabhängigen Beobachtungen des heutigen Hofbetriebs. Nachfolge bleibt offen.

## Nötige Korrekturen und neue Gegenbelege

### Schrottjungs: Preisfund ja, lokaler Standardpreis nein

Datei: `dossiers/sn/sn-mobil-leipzig-schrottjungs-schrottabholung-leipzig.md`.

Gewerbeseite [S1] am 10.10. selbst direkt gelesen: **alle 19 Obergrenzen** stimmen mit dem Wellenbericht überein:

| FE-Originalposition | bis zu EUR/t |
|---|---:|
| Scherenvormaterial | 200 |
| Mischschrott leicht | 170 |
| Mischschrott schwer | 210 |
| Sorte 3 | 240 |
| Trägerschrott | 230 |
| Bremsscheiben | 240 |
| Zerlegematerial | 540 |

| NE-Originalposition | bis zu EUR/kg |
|---|---:|
| Kupfer Raff | 8,50 |
| Kupfer Millberry | 9,50 |
| Kupferschiene | 9,20 |
| Messing gemischt | 4,50 |
| Rotguss | 8,00 |
| Kabel (diverse) | 5,20 |
| Aluminium (div.) | 2,50 |
| Edelstahl V2A | 1,10 |
| Zink | 1,90 |
| Blei | 1,40 |
| Zinn | 22,00 |
| Hartmetall / Wolfram | 55,00 |

**Pflichtkorrektur:** Der aktuelle kuratierte `## Überblick` enthält immer noch **„Hartmetall/Wolframkarbid 55,00“**. Die späteren Warnungen vom 08./09.10. machen diese widersprüchliche laufende Sortenbezeichnung nicht richtig. Im Überblick auf **„Hartmetall / Wolfram 55,00 €/kg“** ändern, außerdem die dort verkürzten Sammelpositionen nach Möglichkeit als „Kabel (diverse)“ und „Aluminium (div.)“ wiedergeben. Historische Timeline nicht umschreiben; neue Korrektur mit Quelle [S1] ergänzen.

Fachlich sichere neue Timeline-Notiz zur Integration:

> [Korrektur 10.10.2026: Gewerbeseite direkt erneut gelesen; Originalposition heißt Hartmetall / Wolfram, nicht Wolframkarbid. Kuratierten Überblick entsprechend korrigieren; 55,00 EUR/kg ist ausschließlich eine unverbindliche Großmengen-Obergrenze, keine belegte Reinheits-/Wolframkarbidqualität. Alle 19 Obergrenzen stimmen mit 09.10. überein. Kein Preis-Gültigkeitsdatum, keine numerische Großmengenschwelle/USt-Basis gefunden. Quelle: https://schrottjungs.de/gewerbe/ .]

Weitere eigene Primärprüfung:

- Impressum [S2]: Schrottjungs UG, Billwerder Steindamm 15a, 20537 Hamburg; Magnus Ditz, HRB 196586 Hamburg; 0173-8705566, info@schrottjungs.de. Das klärt den **Websiteherausgeber**, nicht den ausführenden Leipziger Partner oder den Zahlungspflichtigen jedes Einzelangebots. Ein neues Register-Live-Gate für die Gesellschaft wurde in diesem Audit nicht abschließend durchgeführt.
- Leipzig [S3] und Startseite [S4] am 10.10. **neu direkt gelesen**, nicht nur Befunde vom 08.10. übernommen. Leipzig nennt 40–80 EUR bei kleinen Mengen/Einzelgeräten und zugleich „Wir holen NICHT ab … Eisenschrott < 300kg“. Ortsreste Lüneburg/Oedeme/Berlin bestehen fort.
- Startseite [S4] enthält **innerhalb derselben Seite** sowohl 40–80 EUR in Leistungs-/FAQ-Texten als auch 20–50 EUR in einer weiteren FAQ-Passage. Der Konflikt ist nicht lediglich „Leipzig gegen Startseite“. Startseiten-FAQ: Abholung durch lokale Partnerbetriebe; kein namentlicher Leipziger Partner belegt.
- Gewerbecontainer [S1]: mietfrei **bei Schrottbefüllung**, Lieferung/Abholung inklusive. Partner sind laut Seite geprüfte Entsorgungsfachbetriebe; das ist kein eigener Zertifizierungsnachweis für die Schrottjungs UG.

**Integration:** Recherchepreisfund und ausdrücklich bedingte mobile Dienstleistung behalten. Ankauf-Obergrenzen sind keine Gebühren und keine Verkaufspreise. Keine Preise auf sämtliche Stadtzeilen verteilen; kein pauschaler Leipziger Tages-/Privattarif, kein 300-kg-Mindestwert für sämtliche Materialien. Kanonische Betreiberzeile, Partner-/Abrechnungszuordnung, sichtbare Großmengenkondition und Materialmapping bleiben Handler-Gates. Keine Untergrenze oder Veröffentlichungsdatum aus dem Abrufdatum erfinden. Die Welle führt selbst keinen neuen Preisdatensatz/Handler ein; Preisnennungen in Timeline sind Dokumentation, nicht automatisch Ingestion-Preise.

### TSR Dresden-Heidenau: Zertifikat tragfähig, Überblick aktualisieren

Datei: `dossiers/sn/sn-heidenau-tsr-deutschland-nl-dresden-heidenau.md`.

EfB-PDF am 10.10. erneut direkt über HTTP geladen und ohne Dateiablage per `pdftotext` ausgewertet [T1]:

- Zertifikat **801.0237/25**, Vorgang **ZZST002000211011**, Ingenieurbüro Ulbricht GmbH; Prüfung 23.10.2025, Ausstellung 24.10.2025, gültig bis 22.04.2027.
- Betreiber **TSR Deutschland GmbH & Co. KG**, Dortmund **HRA 19743**; aktuelles Betreiber-Impressum bestätigt HRA 19743/Sitz Lünen [T3].
- Anlagen **4–6**: ausdrücklich **Niederlassung Dresden-Heidenau**, **Am Lugaer Graben 20, 01259 Dresden**.
- Anlagen **1–3**: **Dresden-Hafen, Magdeburger Str. 58**. Das sind unterschiedliche zertifizierte Standorte; Hafen nicht auf Heidenau übertragen.

**Neuer Abrufbefund:** `https://www.tsr-recycling.de/standorte/schrotthaendler-dresden/` ist am 10.10. per direktem HTML-Abruf auswertbar [T2], bestätigt jedoch **nur Magdeburger Straße 58, 01067 Dresden**, +49 351 2635100 und dortige Öffnungszeiten. Kein „Lugaer“-/„Heidenau“-Standortblock im ausgelesenen Text. Die damalige Abrufblockade war methoden-/zeitbezogen, nicht Nichtexistenz einer Webseite. Keine Hafenzeiten/Telefonnummer ins Heidenau-Dossier kopieren.

**Pflichtkorrektur im laufenden Überblick:** „Der Betreiber-Webauftritt bestätigt derzeit keine auswertbare Heidenau-Filialseite“ präzisieren. Integrationsfähiger Ersatz:

> Das aktuelle Betreiberzertifikat belegt die Niederlassung Dresden-Heidenau an Am Lugaer Graben 20, 01259 Dresden. Die am 10.10.2026 direkt auswertbare Dresden-Detailseite beschreibt ausschließlich den getrennten Hafenstandort Magdeburger Straße 58. Kundenzeiten und Annahmebedingungen am Lugaer Graben sind weiterhin unbestätigt; keine HMV- oder Nachfolgekette belegt.

**Neuer zweiseitiger Adressgegencheck:** Die Betreiberseite von **Veolia Umweltservice Ost GmbH** bestätigt ebenfalls **Am Lugaer Graben 20, 01259 Dresden**, Wertstoffhof/Sortier-/Aufbereitungsanlage, eigene Zeiten und eigenes Telefon [T4]. Das ist ein realer weiterer Betreiber am selben Adresskomplex, **kein Nachweis einer TSR→Veolia-Nachfolge** und kein Beweis gegen das aktuelle TSR-Zertifikat. Veolia-Zeiten, Kartenzahlungsregel und Kontakte bleiben strikt getrennt.

Die Welle darf den zertifizierten Standort dokumentieren; sie muss keinen zweiten Kundenzeitenbeleg erfinden. Der bestehende `status: aktiv` hat nun einen aktuellen standortspezifischen Zertifikatsbeleg, ist aber keine Garantie öffentlich zugänglicher Privatannahme. **Nicht blind `city: Heidenau` → Dresden oder neue Frontmatteradresse setzen:** Das wäre ein zusätzlicher, im jetzigen Delta nicht enthaltener Identity-/Standortkorrekturschritt. Nach Owner-Abgleich der Geschwisterzeilen und genauer Zuordnung ist Dresden/01259/Am Lugaer Graben 20 der belegte sachliche Standortkandidat; stabilen Slug erhalten. Noch offen: amtlicher Stadt-/Standortabgleich, konkrete Kundenannahme, HMV-/frühere Betreiberidentität und gegebenenfalls Betreiberbestätigung der geteilten Adresse.

## Einzelentscheidungen für alle 20 Dossiers

Alle Pfade beziehen sich auf `/root/Documents/schrott-mcp-deploy/dossiers/`. „Freigabe“ bedeutet fachliche Freigabe **des vorliegenden Deltas**, nicht Bestätigung sämtlicher Legacyfelder.

| Dossier | Entscheidung / verbleibendes Gate |
|---|---|
| `by/by-regensburg-bitterbier.md` | Timeline freigeben. Kernschrott HRB 12051/Dieselstr. 7/Michael Bitterbier selbst direkt registerseitig bestätigt [R1]; keine Verbindung zu Vilshofener Str. 3. Gewerbe-/Betreiberidentität und Telefonendung 76/78 offen. |
| `sn/sn-netzschkau-08491-schrott-dietrich-frank-matthias-fam-1-be.md` | Prüfnotiz/Normalisierung freigeben. Kommunaler Stadtanzeiger selbst direkt gelesen [N1], belegt nur Personennennung, keine Handelsidentität. Frontmattername behauptet weiter „Fam., 1 Betrieb“; als expliziter ungeklärter Legacy-Sammelimport behalten, nicht als bestätigte Händlerfamilie publizieren. Gewerbe-/Einzelidentitäten nötig, kein vorschneller Split. |
| `by/by-karlstadt-gambach-97753-schrott-und-metallhandel-christian-schle.md` | Vorsichtige Notiz freigeben; neues konkretes Domain-Gegenbeleg unten. Keine Stammham-Kontakte übernehmen. Betreiber-/Gewerbeidentität Karlstadt bleibt offen. |
| `sn/sn-durrrohrsdorf-dittersbach-01-neumann-wilfried-av.md` | Notiz/Normalisierung freigeben. Eigene Suche wieder nur Radeberger-Str.-17-Verzeichnisleads; Gemminger Namensvetter nicht zuordnen. Gewerbe- und individuelles GESA-/EfB-Gate offen. |
| `sn/sn-zwickau-gottfried-wunderlich.md` | Notiz/Normalisierung freigeben; Negativvorschläge sind kein Negativregister. Gewerbeauskunft für Jacobstr. 4 nötig; keine Kontakte/GEO aus Leads. |
| `th/th-gera-smg-schrott-metall.md` | Notiz/Normalisierung freigeben. Eigene Suche ergänzt Scholz-/SMH-Leads und getrennten SMH-Betreiberstandort unten; keine Kette bewiesen. Exakte historische Firmierung/Gessentalstr. 5A amtlich klären. |
| `by/by-auerbach-ark.md` | Prüfnotiz freigeben. HRB 4617 und Mischadresse selbst direkt bestätigt [R2]; aktueller amtlicher Registerauszug, Sitz/Betriebsstätte und Kontaktnachweis fehlen. Keine Leipelt-Zuordnung. |
| `by/by-hafurt-denk.md` | Notiz/Normalisierung freigeben. Andreas-Denk-Adressen Kilianstr. 23/Galgenanger 2 bleiben Leads, keine Umzugskette nach Zeil. |
| `by/by-kummersbruck-schrott-christian.md` | Notiz/Normalisierung freigeben. Personen-/Vereinsprofil ist kein Gewerbebeleg; Familienname Schrott nicht als Branche auslegen. Amtlicher Namens-/Gewerbeabgleich nötig. |
| `sn/sn-chemnitz-becke-jens-buntmetallhandel.md` | Rückstufung/Notiz freigeben; Creditreform weiterhin 403. Zweiter qualifizierter Identitäts-/Betriebsbeleg fehlt. |
| `by/by-konigsberg-luczkow.md` | Prüfnotiz freigeben. Eigene Suche bestätigt nur Maßbach-Portalcluster, keinen Betreiber. Beide Orte und Telefonnummer gemeinsam abgleichen; keine Mailübernahme. |
| `by/by-obertraubling-kaiser-recycling.md` | Prüfnotiz freigeben. Creditreform selbst erneut 403 [K1]; exakte Jutta-Kaiser-/GESA-Identität und Telefonkonflikt offen. |
| `sn/sn-leipzig-anas-metall-leipzig.md` | Rückstufung/Notiz freigeben. Vollständiger Betreibername/Annahmestelle und Primärbeleg fehlen. |
| `th/th-gotha-wrs-wertstoff-recycling-service.md` | Timeline freigeben, bestehenden GmbH-Schließungsstatus erhalten. Heutiger FAQ-Befund siehe oben; Nachfolger nicht bestätigt. |
| `by/by-zeil-am-main-97475-denk-av.md` | `tot` → `unbekannt` und Notiz freigeben; Brigitte-/Elfriede-/Fränklin-/Geschwisterdossieridentität offen. |
| `by/by-pilsting-94431-kroi-av.md` | Prüfnotiz freigeben. Gemeinde-HTML selbst bestätigt Horst Kroiß, Landauer Weg 103, 09953 521, autokroiss@t-online.de [P1]. Kommunales Firmenverzeichnis ist kein amtlicher Gewerberegisterauszug und allein kein Zwei-Quellen-Nachweis. Hausnummer 106/103 und Betreiberannahme offen. |
| `bw/bw-albstadt-spindler-karl-schrotthdl.md` | Rückstufung/Normalisierung/Notiz freigeben; aktueller Gewerbe-/Kundenannahmebeleg fehlt. |
| `bw/bw-lorrach-mv-lorrach.md` | Acht Stammdaten-/Servicefüllungen freigeben; Abrufumfang in negativer Konditionsaussage präzisieren. |
| `sn/sn-mobil-leipzig-schrottjungs-schrottabholung-leipzig.md` | Timeline-Preisfund freigeben **nach Überblickkorrektur Wolframkarbid → Wolfram**. Kein lokaler Preis-/Partner-/Handler-Gate geschlossen. |
| `sn/sn-heidenau-tsr-deutschland-nl-dresden-heidenau.md` | Zertifikatsnotiz freigeben **nach aktualisiertem Überblick**. Hafen-/Veolia-Fakten strikt getrennt lassen; aktuelle Kundenannahme am Lugaer Graben offen. |

### Neue Rechercheansätze ohne Feldfreigabe

**Schlereth Karlstadt:** Eigene Suche findet die Facebook-Seite `SchrotthandelChristianSchlereth` mit Suchauszug „Gambach“ und Domain `schrotthandel-schlereth.de` [C1]. Facebook-Direktabruf lieferte keinen auswertbaren Inhalt; der Auszug ist hier ein Lead, keine authentifizierte Betreiber-Socialquelle. Die Domain einschließlich Impressum dagegen direkt gelesen [C2]: **Ewald Schlereth, ES-Schrott-Containerdienst Schlereth, Jahnstr. 28, 85134 Stammham**, Telefon **08405 279** beziehungsweise **0172 9068749**. Somit konkret belegter fremder Domainbetreiber; nicht Christian/Frühlingstr. 43/Karlstadt. Kein Beweis, dass der zuvor blockierte Gelbe-Seiten-Weblink genau auf diese Domain zeigt. In neuer Timeline den Kandidaten und Gegenbeleg dokumentieren, nicht eine aufgelöste Verwandtschaft-/Zweigkette behaupten. Der Wellenbericht hat diese prüfbare Domainspur noch nicht ausgeschöpft.

**SMG Gera:** Portal-Suchauszug [G1] nennt für Gessentalstr. 5A `scholz-recycling.de`; das ist lediglich ein Betreiberketten-Lead. Scholz-Seite `/standort/gera` [G2] ist direkt lesbar, trägt 25.09.2017, enthält im extrahierten Inhalt aber keine zuordnungsfähige lokale Adresse/Kontaktzeile. Direkte aktuelle SMH-Standortseite [G3] nennt **SMH**, **Fasaneninsel 10, 07548 Gera**, nicht SMG/Gessentalstraße. Keine Gessental-/SMG-Nachfolgekette bewiesen, keine Übernahme fremder Öffnungszeiten. Nächster sinnvoller Schritt: historische exakte SMG-Firmierung/HR-Daten plus Scholz-Kette prüfen, statt weitere unspezifische Ortsplanung zu sammeln.

## Owner-Schritte aus dem ursprünglichen Audit

**Aktualisierung nach Main-Vorbereitung:** Schritte 1 und 2 unten sind für genau die 20 Dossiers umgesetzt; offen bleiben Owner-Abnahme, technische Tests und gegebenenfalls spätere autorisierte Veröffentlichung/Produktionsprüfung. Die ursprünglichen Recherchebefunde und empfohlenen Ersatztexte bleiben als Audit-Historie erhalten.

1. Zwei Pflichtkorrekturen im kuratierten Überblick durchführen; MV-Konditionssatz auf den Abrufumfang begrenzen. Neue Gegenchecks gegebenenfalls als datierte Timeline ergänzen; bestehende Historie nicht umschreiben.
2. Nur die 20 freigegebenen Südost-Deltas zur Integration auswählen; der Worktree enthält andere ausstehende Tranchen und einen Handler-Delta, die dieses Audit ausdrücklich nicht bewertet oder bearbeitet.
3. Beim Owner das vorgeschriebene **`cargo test -p schrott-mcp-ingestion seed`** ausführen. Technische Import-/Update-Semantik, komplette Korpusvalidierung und insbesondere tatsächliche DB-Wirkung der Timelineänderungen sind noch nicht nachgewiesen.
4. Erst nach späterer autorisierter Integration/Veröffentlichung Produktionswerte read-only gegen Seed prüfen. Hier keine Aussage „produktiv korrigiert“ oder „Tests bestanden“ ableiten.

## Quellenverzeichnis — eigener Abruf 10.10.2026

Direkt bedeutet eigener HTTP-/Webfetch-Inhaltsabruf; PDFs per direktem HTTP und `pdftotext` im Speicher. Suchauszüge/Blockaden ausdrücklich separat ausgewiesen. Northdata und Registerspiegel sind README-zugelassene Registerbelege, keine eigenen Beobachtungen von Kundenbetrieb.

- [M1] https://www.mv-loerrach.de/impressum.html — direkt.
- [M2] https://www.mv-loerrach.de/kontakt.html — direkt.
- [M3] https://www.mv-loerrach.de/leistungen-privatanlieferung.html — direkt.
- [M4] https://www.mv-loerrach.de/leistungen-komplettentsorgung.html — direkt.
- [M5] https://www.mv-loerrach.de/leistungen.html — direkt.
- [M6] https://www.mv-loerrach.de/pdf/Zertifikat_Entsorgungf_Betrieb.pdf — direktes PDF, Text gelesen.
- [M7] https://www.northdata.de/Metall-%20und%20Rohstoffverwertung%20L%C3%B6rrach%20GmbH,%20L%C3%B6rrach/Amtsgericht%20Freiburg%20HRB%20412919 — direkt HTTP 200.
- [S1] https://schrottjungs.de/gewerbe/ — direkt.
- [S2] https://schrottjungs.de/impressum-datenschutz/ — direkt.
- [S3] https://schrottjungs.de/schrottabholung-leipzig/ — direkt.
- [S4] https://schrottjungs.de/ — direkt, beide Gebührenbereiche enthalten.
- [T1] https://www.tsr-recycling.de/uploads/tx_3slocations/EfB_Dresden%20%28Hafen%2CHeidenau%2920270422.pdf — direktes PDF, Text aller Anlagen gelesen.
- [T2] https://www.tsr-recycling.de/standorte/schrotthaendler-dresden/ — direkt HTTP 200, HTML-Text auswertbar.
- [T3] https://www.tsr-recycling.de/impressum/ — direkt HTTP 200.
- [T4] https://www.veolia.de/ueber-uns/geschaeftsfelder/standorte-und-dienstleistungen/dresden-ost — direkt.
- [D1] http://www.autoverwertung-denk.de/pg003.html — direktes Impressum.
- [W1] https://www.northdata.de/WRS-Wertstoff%20Recycling%20Service%20GmbH,%20Gotha/Amtsgericht%20Jena%20HRB%20500985 — direkt HTTP 200.
- [W2] https://www.online-handelsregister.de/handelsregisterauszug/th/Jena/HRB/500985/WRS-Wertstoff-Recycling-Service-GmbH — direkt HTTP 200.
- [P1] https://www.markt-pilsting.de/in-pilsting-zuhause/wirtschaft-und-gewerbe/gewerbe-am-ort — direkt, HTML mit sonst ausgeblendeten Kontaktfeldern zusätzlich gelesen.
- [R1] https://www.northdata.de/Kernschrott-Recycling+UG,+Regensburg — direkt HTTP 200.
- [R2] https://www.northdata.de/ARK+Schrott+und+Recycling+UG,+Oberpferdt/Amtsgericht+Amberg+HRB+4617 — direkt HTTP 200.
- [N1] https://www.netzschkau.de/fileadmin/stadtanzeiger/2015/09/files/basic-html/page5.html — direkt HTTP 200; Personennennung, keine Betriebsbelege.
- [B1] https://firmeneintrag.creditreform.de/09113/3030231164/JENS_BECKE_BUNTMETALLHANDEL — eigener Direktabruf HTTP 403, zusätzlich Suchlead.
- [K1] https://firmeneintrag.creditreform.de/93083/8230781699/JUTTA_KAISER_AUTORECYCLING — eigener Direktabruf HTTP 403.
- [C1] https://www.facebook.com/SchrotthandelChristianSchlereth — Suchauszug; Direktabruf ohne auswertbaren Inhalt.
- [C2] https://www.schrotthandel-schlereth.de/impressum — direkt HTTP 200; zusätzlich https://schrotthandel-schlereth.de/ direkt gelesen.
- [G1] https://www.auskunft.de/firma/s8krqw7k-smg-schrott-und-metall-gmbh-gera — Suchlead, kein Primärbeleg.
- [G2] https://www.scholz-recycling.com/standort/gera — direkt, lokale Fakten im extrahierten Inhalt nicht enthalten.
- [G3] https://www.smh-recycling.de/unternehmen/standorte — direkt, anderer Standort/anderer Betreibername.

Nicht sämtliche negativen Such-/Suggestbefunde vom 09.10. erneut ausgeführt. Keine damalige erfolglose Suche als Nichtexistenzbestätigung gezählt. Bei den übrigen offenen Dossiers gilt die Freigabe der vorsichtigen Dokumentation, nicht eine unabhängig neu bewiesene Identität oder universelle Nichtveröffentlichung von Preisen.
