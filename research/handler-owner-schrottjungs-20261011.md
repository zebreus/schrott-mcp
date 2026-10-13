# SchrottJungs — strikte Handler-Gates, 11.10.2026

## Ergebnis / Ownership

### B1–B3 Nacharbeit nach unabhängigem Review

Vollständiges unabhängiges Review `/root/Documents/schrottjungs-handler-independent-review-20261011.md` gelesen. **Keine Main-/Deploymentfreigabe:** B1 (unqualifizierte automatische Materialannahmen), B2 (unverbrauchte Quellbedingungen/zusätzliche eigene Karten), B3 (Hidden-Anker/Stylesheetklassen) sind reproduzierte Blocker. Owner autorisiert ausschließlich einen default-false Outcome-Opt-in zum Unterdrücken automatischer direkter UND Fallback-Preisannahmen; explizite Annahmen bleiben unverändert. SchrottJungs bekommt zehn explizite, vollständig konditionierte Materialannahmen. Keine Händler-Slug-Sonderlogik, Schema-/GEO-/Katalogänderung oder Löschung bestehender/fremder Annahmen.

Bestätigte Testseams: echter `record()`→TempDB→Annahmematrix/Standard-View einschließlich fremder Baseline und Write-Delta; Default-Legacy-Recorder; händlereigener SourceWindow-Parser gegen die konkreten B2/B3-Mutationen. Zuerst roter B1-Regressionstest, dann enger Seamfix; B2/B3 ebenfalls negative Regressionen. Lokale Quell-/Live-/Testbelege werden fortgeschrieben. Ownership/Auth/Publication/Prodnachweis bleiben offen.

#### Lokale Nacharbeit / Handoff zum erneuten unabhängigen Review

- **B1 RED tatsächlich ausgeführt:** `cargo test -p schrott-mcp-ingestion b1_conditioned_acceptances -- --nocapture` scheiterte vor dem Fix (Exit 101), weil `stahlschrott-scheren` und `edelstahl-gemischt` aus den drei explizit gesetzten Fremd-Baselinezeilen überschrieben wurden. Danach enger `HandlerOutcome.skip_price_acceptances: bool`-Opt-in, Default false. `record()` guardet nur automatische direkte und Fallback-Preisannahmen; explizite `outcome.acceptances` bleiben außerhalb des Guards. Kein Handler-Slugtest, keine Änderung an Preis-/Datums-/Fallbacktabellen oder DB-Schema.
- Die neue Struct-Spalte erforderte **122 mechanische `false`-Initialisierungen** für vollständig ausgeschriebene Legacy-Literale. Diffaudit: 119 sonstige tracked Dateien enthalten ausschließlich 121 solche Einzeilen-Ergänzungen (einschließlich zwei Scheduler-Testliteralen); die weitere Initialisierung steht im gemeinsamen Modul-Test. Default-basierte Literale brauchen keine Änderung. Diese große Dateianzahl ist keine Erweiterung von Handlerlogik. Zwei reale Legacy-Händleridentitäten Beller/Vedder erhalten im Test synthetische Recorder-Quotes, ausdrücklich keine behaupteten Livepreise: direkte Guss-/V2A-Annahmen bleiben leer konditioniert, beide bisherigen Fallbacks bleiben mit bisheriger Begründung aktiv, explizite Alu-Annahme bleibt qualifiziert.
- SchrottJungs optiert als einziger Handler ein und liefert **10 deduplizierte explizite Materialannahmen** für die 11 Preisvarianten. Alle vollständigen Gewerbe-/Großmengen-/Mengen-/Qualitäts-/Sortierungs-/Markt-/Anfragegrenzen kommen in `trader_materials.conditions` an, zusätzlich die Beleggrenze „Betreiberreferenz, keine Zusage einer Annahme durch einen lokalen Partnerhof“. Originalsorten werden im Annahmetext eingeschlossen (beide Mischschrottsorten zusammen, Kupfer gemischt ausschließlich Quelle Kupfer Raff), nicht die gesamte breite Katalogfraktion versprochen. Die zusätzliche Originalsorten-Regression war ebenfalls zuerst rot und wurde danach umgesetzt.
- Echte TempDB mit leerer Annahmebasis: exakt 10 bedingte Materialien, keine neuen `stahlschrott-scheren`-/`edelstahl-gemischt`-Fallbacks, alle 11 Standardpreise weiterhin `upto`, gleiche Varianten, obere Grenze gleich Preis, untere Grenze NULL. Separate TempDB mit drei Fremdannahmen: alle drei Zeilen inklusive `accepts`, Conditions und Zeitstempeln unverändert; danach 13 Gesamtzeilen, also **+10 neue Zeilen**, nicht eine behauptete 10er-Gesamtmatrix. Wiederholung: `accepted=10` zählt aufgelöste explizite Einträge, **Write-Delta 0**, sämtliche Annahmezeilen und Zeitstempel identisch. Nichts gepurgt; bereits vorhandene Fremdannahmen bleiben bewusst sichtbar.
- **B2 RED tatsächlich ausgeführt**, Exit 101 vor Windowfix: die zusätzlichen `<p>`-Bedingungen wurden akzeptiert. Jetzt ist das eigene FE-/NE-Fenster textuell geschlossen: alle nichtleeren Textnodes müssen unter tatsächlich konsumierten Feldern liegen; extra Quote-/Condition-Elemente, unbekannte Daten-/Tooltipattribute und abweichende Bildbeschreibungen werden abgewiesen. Alle Karten im Abschnitt werden inventarisiert, nicht nur Grid-Karten; neue zwanzigste Karte außerhalb Grid scheitert. Regressionen für 10-t/netto/keine-Abholung-Kartenparagraph, Privatkundenparagraph vor Grid und Extrakarte plus zusätzliche Text-/Data-/Bildvarianten; nichts davon wird als echte Quellkondition ausgegeben. Navigation/Gebühren außerhalb der zwei Fenster bleiben unberührt.
- **B3 RED tatsächlich ausgeführt**, Exit 101 vor Hiddenfix: versteckter semantischer Anker wurde akzeptiert. Jetzt werden beide Header/Sub-Anker und der FE-Ankauf-Overline individuell (erste und letzte Vorkommen), Karten, Label-/Preisfelder und Banner samt Vorfahren/Descendants geprüft. `hidden`, `aria-hidden`, Inline-Hiding, explizite Hiddenklassen und passende eingebettete stylesheetbasierte Hiding-Selektoren blockieren. Regresstests für `.review-hidden {display:none}` und `@media print`/`visibility:hidden!important` bestehen; fremde Navigations-Hiddenklasse ohne Preisbezug blockiert nicht.
- **Sichtbarkeitsgrenze:** serverseitiger HTML-/eingebetteter CSS-Vertrag, kein Browserclaim. Media-/Supportsregeln werden konservativ behandelt; CSS-Cascade, externe Stylesheets, JS/Dynamic DOM und vollständige Computed-Style-/Layoutsichtbarkeit werden weder gefetcht noch simuliert. Eindeutige im eingebetteten CSS definierte Custom Properties werden aufgelöst; Farben mit explizitem Alpha 0 werden erkannt. Ein nur in nicht geladenen externen Stylesheets aufgelöster `color:var(...)`-Wert wird nicht als transparent klassifiziert. Nicht auswertbare Selektoren und unbekannte Sichtbarkeitsdeklarationen scheitern konservativ. Keine vollständige Browser-/CSS-Engine behauptet.
- Eigener Test-TMP unter `/root/Documents/schrott-mcp-schrottjungs-20261011/target/schrottjungs-test-tmp`, per `TMPDIR` für Cargo gesetzt. Keine fremden Temp-/Targetdateien gelöscht; keine ENOSPC-Umgehung durch Fremdcleanup.
- Finale targeted-Prüfung nach Originalsortenpräzisierung: `TMPDIR=.../target/schrottjungs-test-tmp /root/.cargo/bin/cargo test -p schrott-mcp-ingestion schrottjungs`: **8 bestanden, 0 fehlgeschlagen**, einschließlich B1–B3 und Preisänderungsdatum bis Standard-View. Ein vorheriger Aufruf mit irrtümlichem Paketnamen `schrott-ingestion` scheiterte an der Paketauswahl (Exit 101), nicht an einem Test; obiger korrekter Aufruf wurde tatsächlich ausgeführt.
- Finaler tatsächlicher DB-freier Livehandler nach Originalsortenpräzisierung: `/root/.cargo/bin/cargo run -p schrott-mcp-ingestion --example live_handlers -- by-u-a-munchen-schrottjungs-mobil-bundesweit`, Exit 0, HTTP 200, 117684 Bytes, **11/19 Preise + 8 Skips**, `acceptances=10`, `published_at=None`, leeres TraderInfo. Alle zehn ausgegebenen Annahmen enthalten vollständige Bedingungen, Partnerhofgrenze und konkrete Originalsorten. Kein Produktionsscrape. Alte `acceptances=0`-Belege weiter unten sind ausschließlich Vor-Review-Historie.
- Finaler Rustfmt-Check und `git diff --check` bestanden. Erneuter false-only-Audit bestätigt 119 Legacy-Dateien / 121 ausschließlich `false`-Ergänzungen außerhalb Shared-Modul/Registry. Worktree weiterhin Branch `owner/schrottjungs-20261011`, HEAD `d06f601d6de0bfe2c41fb76d4097c5722ec33759`, drei neue untracked Handler-/Fixture-/Berichtsdateien. Main read-only geprüft: inzwischen fremde Sperling-Handler-/Fixture-/Registry-/Berichtsarbeit vorhanden, unverändert belassen; nicht als eigener sauberer Main behauptet. Keine eigenen Mainwrites.
- Finale Workspaceprüfung nach Originalsortenpräzisierung: erster Lauf `sh_12b6898f2001mNcBCzgDdBYWMk` vom Tool nach 120 Sekunden abgebrochen, ohne gemeldeten Testfehler; dieser Lauf ist kein erfolgreicher Gesamtbeleg. **Wiederholung mit 600 Sekunden Limit erfolgreich abgeschlossen**, `sh_12b6a868e0014lWus8nIFS1sdP`: **669 bestanden, 0 fehlgeschlagen, 5 ignoriert**, Auth 4, Core 3, Ingestion 602, Queryworker 4, Server 16, Store 40; alle Doc-Tests erfolgreich. Vollständiges Abschlusslog nachgelesen: `/root/.local/share/opencode/shell/42aa29071d6c1a29b482e0269f73393202254764/sh_12b6a868e0014lWus8nIFS1sdP.out`. Ignorierte Live-Tests sind kein Livebeleg; SchrottJungs wurde separat DB-frei tatsächlich ausgeführt. Damit sind die lokalen Pflichtprüfungen abgeschlossen, nicht das unabhängige Review oder die Publication-/Prodgates.

**Erneutes unabhängiges Ownerreview erforderlich; dieser Implementierungsbericht erteilt keine Main-/Deployfreigabe.** Originalreviewdatei unverändert belassen. Review sollte insbesondere den Shared-Guard, false-only-Diff, genaue Write-Deltas/Foreign-Baseline sowie jede ursprüngliche B2/B3-Mutation erneut gegen den Kandidaten ausführen. Auth/Publication/Mainintegration/Deployment/normaler Prod-Schedulernachweis weiterhin offen; Ownership nicht abgeschlossen.

### Ownerentscheidung / Fortsetzung

Owner hat kanonischen BY-Slug, sichtbares Konditionssuffix in jedem `variant`, unveränderte Repo-Inferenz `observed_price_change` (ausdrücklich kein Quellpublikationsbeleg) und diagnostiziertes Teilmapping ratifiziert. Damit ist der nachstehende ursprüngliche Zeit-Gateblock aufgehoben. **Im isolierten Worktree implementiert und genau einmal registriert: 11 von 19 Originalpositionen als konditionierte Obergrenzen, 8 diagnostizierte Skips.** Schema und Datumsrecorder unverändert; inzwischen ausdrücklich autorisierter Annahme-Recorder-Opt-in siehe B1 oben. Jeder Variant trägt zusätzlich „Quelldatum unbekannt; ggf. Datum der Preisänderungsbeobachtung“. Handler setzt `published_at=None`. Publication/Deployment/Prodnachweis bleiben offen; kein vollständiger 19er-Feed behauptet.

Baseline nachträglich auditiert: `cargo test --workspace` erfolgreich (Exit 0; Ingestion 594 bestanden, 5 ignoriert, Store 40 bestanden, Server 16, Queryworker 4; alle weiteren ausgeführten Tests ebenfalls erfolgreich). Rustfmt-Check und gezielter Datumstest bestanden. Diese Ergebnisse betreffen den unveränderten Baselinecode, noch keinen neuen Handler.

### Implementierungsbelege vor unabhängigem Review (historisch)

- Handler: `crates/ingestion/src/traders/handlers/schrottjungs.rs`; Registry: `handlers/mod.rs`, ein Modul und ein `handler()`-Aufruf. Keine Geschwisterregistrierung.
- Fixture: `handlers/fixtures/schrottjungs-gewerbe-20261011.html`, Ausschnitt aus direktem Live-GET; Original-Selektoren/Sorten/Zahlen/Einheiten/Bedingungen erhalten, dekorative Bilder/SVG/Wrapper ausgelassen und diese Kürzung im Fixture dokumentiert.
- Vier gezielte Tests bestanden, 0 fehlgeschlagen: vollständige 19er-Partition/11 genaue Werte/Units und sichtbare Qualifizierung; Fail-closed gegen Betrags-, Unit-, Preisrichtungs-, Bedingungs-, Hidden-/Doppelzeilen- und Abschnittsdrift; Betreiber/HRB/HTTP-/Redirectvertrag und einmalige Registry; echte TempDB-Aufzeichnung in den bestehenden Katalog und Standard-View mit Preisänderung.
- TempDB-Beleg: alle 11 `v_current_prices`-Zeilen zeigen vollständiges Konditionssuffix, `upto`, `price_max=price`, `price_min=NULL`, korrekte Quellunits und URL; erster Abruf hat `published_at=NULL`. Mutation Mischschrott leicht 170→180 EUR/t ergibt weiterhin dieselbe qualifizierte Variante, `price_max=180`, Untergrenze NULL und `published_at=2026-10-12T00:00:00+00:00`; Preis-Metadaten weisen **`published_at_basis=observed_price_change`**, nicht `page_stated`, aus. Das Datum ist eine Preisänderungsbeobachtung, kein Quellgültigkeits-/Publikationsbeleg. Keine zweite Händlerzeile erhält Preise derselben Quelle.
- DB-freier tatsächlicher Livehandler zweimal erfolgreich ausgeführt (auch nach letztem Codeaudit): `/root/.cargo/bin/cargo run -p schrott-mcp-ingestion --example live_handlers -- by-u-a-munchen-schrottjungs-mobil-bundesweit`. Endstatus `200`, `117684` HTML-Bytes, `published_at=None`, `acceptances=0`, `alive=true`, komplett leeres `TraderInfo`; **11 Preispositionen + 8 laute SKIPs**, übereinstimmend mit Quellmomentaufnahme. Beide eigenen Quellen ohne Redirect/Statuswechsel gelesen und Betreiberanker geprüft. Dies ist ein Live-Handlerbeleg ohne `record()`/DB, **kein Produktionsscrape**.
- Rustfmt ausgeführt, anschließender `cargo fmt --all -- --check` bestanden. `git diff --check` bestanden. **Finale vollständige Workspaceprüfung bestanden, Exit 0: 665 bestanden, 0 fehlgeschlagen, 5 Live-Tests ignoriert** (Auth 4, Core 3, Ingestion 598, Queryworker 4, Server 16, Store 40; Doc-Tests ebenfalls erfolgreich). Die Schrottjungs-Liveprüfung wurde separat tatsächlich ausgeführt, nicht aus ignorierten Tests abgeleitet.

| Übernommene Originalposition | Bestehendes Katalogmaterial |
|---|---|
| Mischschrott leicht; Mischschrott schwer (getrennte Varianten) | `mischschrott` |
| Trägerschrott | `stahlschrott-traeger` |
| Bremsscheiben | `eisenschrott-gussbruch` |
| Kupfer Raff | `kupfer-gemischt` |
| Kupfer Millberry | `kupfer-millberry` |
| Messing gemischt | `messing` |
| Rotguss | `bronze-rotguss` |
| Aluminium (div.) | `aluminium-gemischt` |
| Edelstahl V2A | `edelstahl-v2a` |
| Zink | `zink` |

**Acht Originalpositionen bleiben ohne Preisimport:** Scherenvormaterial (nicht belegter aufbereiteter Scherenschrott), Sorte 3 (keine exakte Katalogposition), Zerlegematerial (Zusammensetzung unbekannt), Kupferschiene (keine exakte Katalogposition/Qualität), Kabel (diverse) (Leitermetall/Stecker/Gehalt unbekannt), Blei (Weichblei-/Auswucht-/Formzuordnung offen), Zinn (keine belegte Reinzinnqualität/Warenform), Hartmetall / Wolfram (zwei getrennte Katalogmaterialien). Jeder Skip enthält Originalname, Originalobergrenze/-einheit, Grund und Bedingungen. Keine erfundene Wolframkarbid-/Kabel-/Reinzinnspezifikation; kein neuer Katalog.

### Ursprünglicher Gatebefund (historisch, vor Ownerentscheidung)

**Zurückgestellt, nicht implementiert oder registriert.** Die Betreiberzuordnung und die numerische Obergrenzensemantik sind belegbar. Der unveränderte gemeinsame Recorder erfüllt jedoch die streng ausgelegte Auftragsbedingung „kein Tagesdatum“ nicht dauerhaft: bei vergleichbarer Preisänderung wird auch ohne Quellendatum ein `published_at`-Tagesdatum ergänzt. Zudem reicht die Ablage von Bedingungen ausschließlich in Preisnotizen nicht für die Standard-View. Keine stille Recorder-/Schemaänderung und kein unqualifizierter Punktpreisfeed.

Dies ist ein offener Gatebericht, kein fertiger Handler und keine Produktionsfreigabe. Individuelle Ownership endet weder an diesem Bericht noch an einem später fertiggestellten Parser. Owner muss die unten genannten Entscheidungen koordinieren, anschließend Integration, Publication, Deployment und echten Scheduler-/Prod-Scrape nachweisen. Bekannter fehlender Git-Auth bleibt ein externer Publication-/Deploymentblocker; hier wurde weder Auth geprüft noch Deployment versucht.

- Basis: `d06f601d6de0bfe2c41fb76d4097c5722ec33759`.
- Isolierter Worktree: `/root/Documents/schrott-mcp-schrottjungs-20261011`.
- Branch: `owner/schrottjungs-20261011`; keine Commits/Pushes.
- Änderungen im ursprünglichen Gatecheck: nur dieser Bericht. Aktuelle Worktreeänderungen zusätzlich Handler/Fixture und zwei Registryzeilen; weiterhin keine Main-, Dossier-, Geo-, Schema- oder Prodänderung.
- Keine Subdelegation, kein Produktionszugriff, keine erzwungene Ingestion.

## FIRST: genau eine bestehende Händlerzeile

**Fachlich zulässige und inzwischen Owner-ratifizierte kanonische Zeile: `by-u-a-munchen-schrottjungs-mobil-bundesweit`.** Das bestehende Dossier nennt ausdrücklich `SchrottJungs (mobil/bundesweit)`, `trader_type: mobil`, leere eigene Straßen-/PLZ-Felder und die bundesweite Gewerbeliste mit Großmengen-/bis-zu-Grenzen. Es ist damit keine erfundene neue Betreiberzeile. `state: BY` / `city: u. a. München` bleiben historische Seedklassifikation, kein Nachweis einer Münchner Niederlassung oder lokaler Preisgeltung.

Direkt gelesen am 11.10.2026:

1. https://schrottjungs.de/impressum-datenschutz/ — Schrottjungs UG, Billwerder Steindamm 15a, 20537 Hamburg, Magnus Ditz, AG Hamburg HRB 196586.
2. https://www.northdata.de/Schrottjungs%20UG,%20Hamburg/HRB%20196586 — gleicher Name, Anschrift, HRB/EUID `DEK1101R.HRB196586`, Groß-/Schrotthandel und Eintragung 16.01.2026 mit Magnus Ditz. Direkt gelesene Registerseite, kein Suchsnippet oder Aggregatorbeleg; aus fehlendem Statusfeld keinen zusätzlichen Registerstatus abgeleitet.
3. https://schrottjungs.de/gewerbe/ — „Deutschlandweit“, gewerblicher Metallankauf, Stadtauswahl einschließlich München und die unten vollständig dokumentierte Liste.
4. https://schrottjungs.de/schrottabholung-muenchen/ — mobile Partnerabholung und getrennte Anlieferadresse **Entel GmbH, Lerchenstraße 19, 80995 München**. Keine Gewerbewerte als Entel-Hofpreise zuordnen; keine Hamburger Sitzadresse in Münchner Stationsfelder schreiben.

Vollständig gelesen: beide externen Backlogberichte, README sowie BY-, HH-, HB-, NI-, beide SN- und SL-Schrottjungs-Dossiers. Zusätzlich alle Schrottjungs-Dossierpfade per Volltextsuche geprüft; BE ist ebenfalls kein eigenständiger Feed.

| Bestehende Geschwisterzeile | Grenze |
|---|---|
| `hh-hamburgweit-mobile-abholdienste-schrottjungs-schrott` | Sammeldossier mehrerer Marken; nicht als reine kanonische Schrottjungs-Zeile verwenden |
| `hb-bremen-schrottjungs-bremen` | mobiles Einsatzgebiet, kein eigenständiger Bremer Hof-/Tagespreisfeed |
| `ni-hannover-schrottjungs-abholung` | mobiles Einsatzgebiet, keine lokale numerische Standardliste |
| `sn-mobil-leipzig-schrottjungs-schrottabholung-leipzig` | derselbe Gewerbe-Referenzfeed, kein Leipziger Partnerpreis |
| `sn-markkleeberg-taucha-schkeudi-schrottjungs-schrottabholung-leipzig` | überlappendes Servicegebiet; kein zweiter Feed und kein Merge in diesem Auftrag |
| `sl-saarbrucken-umland-schrottjungs-mobil` | derselbe mobile Betreiber, kein separater Saarbrücker Tarif |
| `be-berlinweit-mobil-schrottjungs-mobile-abholung` | ebenfalls Servicegebiet, nicht dieselbe Liste nochmals registrieren |

## Direkt bestätigte 19 Originalpositionen

Alle Werte sind ausdrücklich **bis zu**, nicht garantierte Auszahlung, nicht Börse/Verkauf/Gebühr. Abrufmomentaufnahme, kein Preisdatum.

| Originalbezeichnung | Obergrenze | Originaleinheit |
|---|---:|---|
| Scherenvormaterial | 200 | €/Tonne |
| Mischschrott leicht | 170 | €/Tonne |
| Mischschrott schwer | 210 | €/Tonne |
| Sorte 3 | 240 | €/Tonne |
| Trägerschrott | 230 | €/Tonne |
| Bremsscheiben | 240 | €/Tonne |
| Zerlegematerial | 540 | €/Tonne |
| Kupfer Raff | 8,50 | €/kg |
| Kupfer Millberry | 9,50 | €/kg |
| Kupferschiene | 9,20 | €/kg |
| Messing gemischt | 4,50 | €/kg |
| Rotguss | 8 | €/kg |
| Kabel (diverse) | 5,20 | €/kg |
| Aluminium (div.) | 2,50 | €/kg |
| Edelstahl V2A | 1,10 | €/kg |
| Zink | 1,90 | €/kg |
| Blei | 1,40 | €/kg |
| Zinn | 22 | €/kg |
| Hartmetall / Wolfram | 55 | €/kg |

Quellenhinweis: „Alle genannten Preise sind unverbindliche Richtpreise für Metallankauf in Großmengen.“ Tagesaktuelle Preise auf Anfrage; Menge, Qualität, Sortierung und aktuelle Marktlage beeinflussen Auszahlung; kleinere Mengen andere Konditionen. Keine numerische Großmengenschwelle, USt-Basis, Untergrenze oder Preisgültigkeit genannt. Eisen-Abholgrenze 300 kg und 40–80 EUR Gebühren sind keine Preisliste-Mengenschwelle und keine Ankaufwerte. Container mietfrei **bei Schrottbefüllung**, nicht universell kostenfrei.

## Modell-/Darstellungsgates gegen echten Maincode

### Bestanden: numerische Obergrenzen

- `crates/ingestion/src/traders/mod.rs:102–120`: `ScrapedPrice` hat `price_kind`, `price_min`, `price_max`, Original-Label und Varianten.
- `crates/store/src/public/prices.rs:112–114`: `upto` ist bereits erlaubte Semantik.
- Ehrliche Darstellung wäre `price_kind="upto"`, `price=price_max=Quellobergrenze`, `price_min=None`; `price` ist hier der Transportwert der Obergrenze, keine garantierte Auszahlung. Keine erfundene Nulluntergrenze und kein `approx`-Punkt als Ersatz.
- Der Recorder normalisiert Preis **und beide Grenzen** über `normalize_unit` (`traders/mod.rs:416–423,670ff`). Originalsorten/-werte/-einheiten müssten zusätzlich im Label/Notiztext erhalten bleiben. Für vorhandene EUR/t-Katalogsorten ist €/Tonne bereits dieselbe Basiseinheit; keine unnötige kg-Konvertierung.

### Ursprünglich offen, inzwischen per sichtbarem Variantensuffix umgesetzt: Bedingungen

- `ScrapedPrice` hat kein eigenes Preisbedingungenfeld. `record()` schreibt `label` nach `prices.notes` (`traders/mod.rs:474`), aber `v_current_prices` selektiert **weder `notes` noch `extra_json`** (`public/prices.rs:100–109`). Nur die Bedingungen in Notizen abzulegen und dann eine sichere konditionierte Standardsicht zu behaupten, wäre falsch.
- Bestehende `variant` wird dagegen in die Standardsicht übernommen. Eine mögliche schemafreie Lösung ist ein mit Owner abgestimmtes, pro Originalsortenvariante sichtbares Konditionssuffix: bundesweiter Gewerbe-/Großmengenrichtwert, unverbindlich, Mengen-/Qualitäts-/Sortierungs-/Marktabhängigkeit, Tagesangebot auf Anfrage, kein Privat-/Entel-Tarif. Das ist **noch keine implementierte oder getestete Darstellungsentscheidung**. Eine ausschließlich versteckte Preisnotiz wird nicht freigegeben.
- Automatische Annahme aus Preisbeobachtung wird im Recorder mit leeren Bedingungen gesetzt (`traders/mod.rs:496–505`). Keine Aussage, dass dieser Pfad selbst bereits alle Großmengenbedingungen transportiert.

### Ursprünglicher Zeitblock, durch explizite Ownerentscheidung aufgehoben

- Ein Handler kann `HandlerOutcome.published_at=None` setzen, aber damit nicht dauerhaft unbekannte Veröffentlichung erzwingen.
- `traders/mod.rs:424–454` ergänzt bei vergleichbarer Preisänderung `german_calendar_date(now)` und kennzeichnet es als `observed_price_change` in `extra_json`.
- `comparable_price_changed` (`646–660`) gilt auch für Änderungen von `price_max`/`price_kind`; kein Ausschluss für `upto`. `HandlerOutcome` hat keinen vorhandenen Opt-out.
- README:93–95 dokumentiert diese generische Inferenz bewusst. Sie ist **nicht dasselbe wie ein erfundenes `page_stated`-Datum**, kann aber die strikte Vorgabe „kein Tagesdatum“ verletzen. In der Standard-View wäre das Datum sichtbar, seine Inferenzbasis dagegen nicht.
- Deshalb vor Umsetzung ausdrückliche Owner-Klärung: Ist nur ein erfundenes Quell-/Tageskursdatum verboten, während klar offengelegte Änderungsbeobachtung zulässig ist? Oder muss `published_at` dauerhaft NULL bleiben? Bei letzterem ist ein separat autorisierter Recordervertrag nötig. Keine heimliche allgemeine Recorderänderung, keine Dummy-Datierung und keine variierenden Varianten zur Umgehung der Vergleichbarkeit.

### Durch konservatives Teilmapping behandelt: kein vollständiges 19er-Mapping

`crates/ingestion/src/pipeline.rs` enthält getrennt `wolfram` (metallisches Wolfram, ausdrücklich kein Carbid-Hartmetall) und `hartmetall` (Werkzeug-Hartmetall). **Hartmetall / Wolfram** ist ohne weitere Quellspezifikation kein sicherer gemeinsamer Katalogpreis. **Kabel (diverse)** belegt weder Leitermetall noch Steckerfreiheit: `kabel-kupfer`, `kabel-alu` oder `kabel-ohne-stecker` wären unbelegte Präzisierungen. Diese Originalpositionen müssten laut diagnostiziert werden; keine neue Katalog-/Schemaerweiterung autorisiert. Ebenso Scherenvormaterial nicht unbesehen auf bereits zerkleinerten Scherenschrott abbilden, Zerlegematerial nicht aus dem Preis zu einer erfundenen Fraktion machen. Keine Reinheit für Zinn oder Kupferschiene ergänzen.

## Ursprüngliche Prüfungen / nicht beanspruchte Ergebnisse (vor Fortsetzung)

- Live-HTTP-Quellen und Register wurden read-only direkt gelesen; 19 sichtbare Obergrenzen bestätigt. **Kein DB-freier tatsächlicher Rusthandlerlauf**, denn kein Handler wurde vor den offenen Gates angelegt/registriert. Web-Fetch ist kein Handlerlauf.
- `/root/.cargo/bin/cargo test -p schrott-mcp-ingestion published_date_is_inferred_only_for_comparable_price_changes`: **1 bestanden, 0 fehlgeschlagen**, 598 herausgefiltert. Der unveränderte bestehende Test bestätigt ein inferiertes deutsches Tagesdatum nach undatierter Preisänderung; `upto`-Anwendbarkeit folgt aus der nicht nach Kind eingeschränkten Vergleichsfunktion. Er benutzt isolierte Testdatenbanken, keine Produktion. Der erste Aufruf `cargo ...` scheiterte lediglich an fehlendem PATH-Eintrag; Wiederholung mit absolutem Toolpfad erfolgreich.
- Kein neuer Parser, keine Fixture-, Mapping-, Registrierungs- oder Record-Testabdeckung behauptet.
- `/root/.cargo/bin/cargo fmt --all -- --check`: **bestanden**, keine Formatierungswrites.
- `/root/.cargo/bin/cargo test --workspace`: Baselineprüfung erfolgreich, Exit 0 (siehe Fortsetzung oben). Diese Baseline ist keine Testabdeckung eines nicht existierenden Schrottjungs-Handlers. Nach Gatefreigabe bleiben neue targeted Tests, Workspace-Tests, echter DB-freier Handlerliveaufruf und Ownerreview verpflichtend.
- Registry-/Handler-Volltextsuche nach `schrottjungs`: kein Treffer. Main-`git status --short` nach Prüfung leer; Worktree-Status zeigt ausschließlich den neuen Bericht als untracked.

## Ergänzung: CSS-/Live-Regressionen nach unabhängiger Prüfung (11.10.2026)

- Das unabhängige Review `ses_ed36a0644ffebJWnR14pk3e5br` meldete **FAIL**: ein Self-Append bei animierten Regeln blockierte den Build; String-basierte CSS-Prüfungen ließen escaped `var()`-Deklarationen passieren und übersahen `clip: rect(...)`; die Kontextabgrenzung schloss fremde Elemente allein wegen einer kollidierenden Price-Section-Klasse aus; der Kommentarentferner beschädigte CSS-Strings wie `content: "/*...*/"`.
- Alle vier Befunde sind im isolierten Kandidaten behoben. Das fehlerhafte Self-Append entfällt. CSS-Deklarationen laufen jetzt über den direkten `cssparser = "0.35"`-Dependency und tokenbewertete Regeln; escaped CSS-Property-/Funktionsnamen werden erkannt, riskante/unklare Sichtbarkeit bleibt fail-closed und `clip: rect(...)` wird berücksichtigt. Der CSS-Kommentar-/Klammer-Scanner respektiert Strings und Escapes. Kontext wird nur für die tatsächlich erkannten Preisabschnitte samt deren Subtrees ausgeschlossen, nicht anhand einer kollidierenden Klasse.
- Der anschließende echte Livehandlerlauf deckte zusätzlich Font-Awesome-Selektoren mit `:not(:focus, :active)` auf. Stateful Pseudoklassen werden jetzt als vollständiger Selektorzustand sicher verbreitert, ohne verschachtelte Kommalisten oder zitierte Attributwerte zu beschädigen; Regressionen für `:not(:focus, :active)`, `:hover`, normale `:not(...)` und `":focus"` in Attributwerten sind enthalten.
- Nach diesen Änderungen: gezielte Tests `cargo test -p schrott-mcp-ingestion schrottjungs::tests -- --nocapture`: **11 bestanden, 0 fehlgeschlagen**. `cargo test --workspace`: **672 bestanden, 0 fehlgeschlagen, 5 ignoriert**, alle Doctests erfolgreich (Auth 4, Core 3, Ingestion 605, Query 4, Server 16, Store 40). `cargo fmt --all -- --check` und `git diff --check` bestanden.
- Erneuter DB-freier Livehandler: HTTP 200, 117684 Bytes, `published_at=None`, `alive=true`, 10 Annahmen, 11 Preise und 8 diagnostizierte Skips. Das bestätigt Parserlauf gegen die aktuelle Livequelle, **nicht** Produktion/Scheduler/Deployment.
- **Status:** frühere unabhängige Review ist nach Fixes weiterhin ein FAIL-Bericht; eine neue unabhängige Prüfung des aktuellen vollständigen Diffs ist erforderlich. Kein Main-Integrations-/Commit-/Push-/Deploymentnachweis und keine Produktionsdatenbank wurde für diese Handlerprüfung verwendet.

## Offene Owner-/Publication-/Deployment-/Scheduler-Gates

1. **Ratifiziert und lokal getestet:** eine kanonische BY-Zeile, keine Geschwister-/Entel-Preise; sichtbares dauerndes Variantensuffix; unveränderte Inferenzpolitik mit offenem Quellendatum; transparentes Teilmapping 11/19 plus 8 diagnostizierte Skips.
2. **Lokal umgesetzt:** Handler/Fixture/Registry, acht gezielte Tests einschließlich B1–B3, autorisierter default-false Annahme-Recorder-Opt-in, Rustfmt und tatsächlicher DB-freier Livehandler; keine Mainfacts/GEO/Schema-/Datumsinferenzänderung. Finale Workspaceprüfung siehe aktuelle Belege.
3. **Offen:** Ownerreview der uncommitteten Dateien einschließlich Mappingkonservatismus, langlebigen Varianten und strict fail-closed Sourcevertrag; koordinierte Integration in Main. Owner darf Diagnose-Skips nicht als 19-von-19-Coverage ausgeben.
4. **Offen / bekannter externer Blocker:** Git-Auth/Publication; hier weder Credentials benutzt/geprüft noch Commit/Push durchgeführt. **Keine Mainintegration, Veröffentlichung oder Deployfreigabe durch diesen Bericht.**
5. **Offen:** autorisiertes Ownerdeployment, danach normaler Schedulerlauf und read-only Prodbelege für genau diese kanonische Zeile, Quelle, Grenzen, Bedingungen, Zeitbasis und materialgerechte Units. Kein `POST /api/ingest/run`, kein Forced-Prod-Ingest. Erwartbar wegen 8 Katalogskips ein diagnostizierter Teilfeed; `partial` nicht als Parserausfall oder vollständige Coverage umdeuten.
6. **Offen:** tatsächlicher Produktionsscrape/Publicationnachweis; nicht durch lokalen HTTP-200, Parserfertigkeit oder Registryeintrag ersetzt. Ownership endet erst nach diesem Nachweis, aktuell ausdrücklich nicht abgeschlossen.

### Nacharbeit aus Code-/Spec-Reviews und Vollseiten-Livefixture (11.10.2026)

- Standards-Review `ses_ed34a886effe7c2rUJlk4y5eNW` bemängelte, dass vereinfachte Fixture-Markups den dokumentierten HTML-Auszugstandard verletzen. `schrottjungs-gewerbe-20261011.html` ist jetzt der unveränderte 7.419-Byte-Ausschnitt der direkt gelesenen Originalseite; zusätzlich pinnt `schrottjungs-gewerbe-page-20261011.html` die komplette unveränderte HTTP-200-Antwort (117.684 Byte). Die Parse-/Kontextregression läuft jetzt gegen die volle Seite, nicht gegen rekonstruiertes Markup.
- Spec-Review `ses_ed34a880dffexU6IuVz3T4bDwg` fand (a) Kommas in `:not(:focus, :active)` wurden beim tatsächlichen CSS-Selektorlauf zerschnitten, (b) das Nachladen externer Stylesheets lag außerhalb des vereinbarten Quellvertrags und (c) moderne `rgb(... / 0)`-Transparenz wurde nicht erkannt. Selektorlisten werden nun klammer-/stringbewusst getrennt und im tatsächlichen Parsepfad getestet; externe Stylesheet-Auflösung/-Fetch ist entfernt; RGB-/RGBA-/Hex-Nullalpha plus eindeutig inline definierte Custom Properties haben Regressionen.
- Der vollständige Originalseiten-Test deckte danach den tatsächlich eingebetteten Bricks-CSS-Nesting-Stil im `<head>` auf (`.popup-sj__form { & .options-wrapper { ... }}`). Einmaliges Flat-Parsing verlor die übergeordneten Selektoren und ließ den Handler trotz gültiger Seite fail-closed abbrechen. CSS-Nesting wird jetzt begrenzt rekursiv aufgelöst (max. Tiefe 64, max. 512 kombinierte Selektoren, max. 8 MB Ausgabe); direkte Parent-Deklarationen und verschachtelte Regeln bleiben getrennt. Neue Regressionen beweisen: relevanter verschachtelter Nullalpha-Selektor blockiert; die reale, fremde Popupregel blockiert nicht die Preisfenster.
- **Bewusst dokumentierte Restgrenze:** Der Body der realen Seite verwendet `color:var(--neutral-trans-80)`, dessen Definition ausschließlich aus nicht gefetchtem externem CSS kommt. Dieser Wert ist im HTML nicht auflösbar. Unaufgelöste externe Farbvariablen werden nicht als transparent ausgegeben; explizite Alpha-0-Werte und eindeutig inline definierte Variablen scheitern weiterhin. Widersprüchliche Inline-Definitionen desselben Variablennamens scheitern fail-closed. Der Handler simuliert weiterhin weder externe Stylesheets noch Browser-Cascade.
- Nach diesen Änderungen: gezielte SchrottJungs-Suite **11/11 bestanden**; Workspace **672 bestanden, 0 fehlgeschlagen, 5 ignoriert** (Auth 4, Core 3, Ingestion 605, Query 4, Server 16, Store 40; Doctests erfolgreich); `cargo fmt --all -- --check` und `git diff --check` bestanden.
- Neuer DB-freier echter Handlerlauf via `cargo run -p schrott-mcp-ingestion --example live_handlers -- by-u-a-munchen-schrottjungs-mobil-bundesweit`: HTTP 200, 117.684 Bytes, 10 Annahmen, 11 Preise und 8 diagnostizierte Skips, `published_at=None`; keine Datenbank geöffnet. Beim anschließenden normalen Beispielbuild fiel auf, dass die 117-KB-Testfixture als ungenutztes Library-Konstantum in den Nicht-Testbuild gelangte; sie ist jetzt mit `#[cfg(test)]` abgegrenzt. Danach erneut bestätigt: gezielte Suite 11/11, `cargo fmt --all -- --check`, `git diff --check` und derselbe DB-freie HTTP-200-Livelauf erfolgreich; der Buildwarnhinweis ist beseitigt.
- Nach zusätzlicher Regression für widersprüchliche Custom-Property-Definitionen erneut bestätigt: **672 Workspace-Tests bestanden, 0 fehlgeschlagen, 5 ignoriert**; gezielte Suite **11/11**, Rustfmt-/Diff-Checks sauber, DB-freier Livehandler wieder HTTP 200 mit 11 Preisen, 8 diagnostizierten Skips und 10 Annahmen. Ambige Inline-CSS-Farbvariablen scheitern nun fail-closed.
- Weiterhin keine Mainänderung, kein Commit/Push/Deploy und kein Produktionsscrape dieses Kandidaten; keine Produktionswrites. Der separate Owner-Check liest den laufenden Prodstand ausschließlich read-only. Die Workspace-/Live-Ergebnisse belegen nur diesen isolierten Kandidaten; das erneute unabhängige Review des jetzigen CSS-Parsers sowie Publication-/Scheduler-/Produktionsgates sind offen.

Scope-Audit vor B1–B3 (historisch): Main `git status --short` leer. Worktree damals ausschließlich Registryänderung und drei neue uncommittete Dateien. Aktuelle autorisierte Seam-/false-only-Ergänzungen siehe B1–B3-Handoff oben. Weiterhin keine Commits, Pushes, Deploys oder Prodwrites.
