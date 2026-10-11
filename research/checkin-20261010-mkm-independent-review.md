# MKM — unabhängiger Read-only-Review, 10.10.2026

## Ergebnis / Freigabegrenze

**Keine uneingeschränkte Kandidatenfreigabe und ausdrücklich keine Produktionsfreigabe.** Der gespeicherte Tages-Snapshot ist artikelbewusst vollständig inventarisiert: 8 Gruppen, 60 Artikel, 55 numerische Angebote, 5 Anfrageannahmen. Vier konkrete P2-Befunde bleiben: unsichtbare Angebote werden als aktuell akzeptiert, Preis-/Einheitengrammatik ist nicht fail-closed, die Betreiberprüfung akzeptiert veränderte Identitäten, und das neue Zinn-99-Mapping widerspricht einer expliziten Katalogabgrenzung. Dazu besteht eine bekannte, hier nicht zu implementierende P1-Aktivierungssperre am Shared Record-/Retirement-Vertrag.

**Begrenzte Freigabe:** gespeicherte Quelleninventur und Übergabe als nicht aktivierter Forschungs-/Mappingkandidat; nicht als vollständig fachlich freigegebenes Mapping, zukunftssicherer Parser oder abgeschlossene Behebung. Registrierung muss Legacy bleiben. Keine Änderungen an Shared Store/Record-Seam durch diesen Review. Der parallele MSG-Agent bearbeitet Retirement; dieser Report dupliziert keine Implementierung.

## Gegenstand und Integrität

- Worktree `/tmp/opencode/schrott-mcp-mkm-full-20261010`; HEAD und Referenz beide `6d5ef762ac2d9b87386491322e4aed1f340b5569`. Deshalb Vergleich der **Arbeitskopie einschließlich ungetrackter Dateien**, nicht ein leerer `base...HEAD`-Vergleich.
- Vollpatch `/tmp/opencode/mkm-full-20261010.patch`, 10.456 Zeilen, SHA256 `608ec348d5078a74da5bfff9bc128b76e5b34639455712b7b9b06066c231b1a5`. `git apply --reverse --check` erfolgreich: Patch entspricht der geprüften Arbeitskopie.
- Parser `crates/ingestion/src/traders/handlers/mkm_metals/overview.rs`, SHA256 `345cc5be4624d232f7e2af08783fa30c5b3da074b36c03a95872e433c1d0bc50`.
- Geprüft: Parser, Legacy-Handler, additive Katalogdefinitionen, Example, beide Python-Dateien, beide Fixtures, Kandidaten-/Audit-JSON, Check-in und die 60 vorhandenen Detailantworten samt Manifest.
- `git diff 6d5ef762 -- crates/store crates/ingestion/src/traders/mod.rs` leer. Handlerfunktion weiterhin `scrape: |c| Box::pin(scrape(c))`, `mkm_metals.rs:35–41`; `scrape()` läuft weiterhin über den Legacy-Kupferparser, nicht `overview::parse`.
- Keine Produktionszugriffe, Netzabrufe, Commits, Staging, Deploys oder Main-Codeänderungen dieses Reviewers. Einzige neue dauerhafte Worktree-Datei ist dieser Report. Temporäre Mutationsfixtures in eigenen `TemporaryDirectory`-Verzeichnissen wurden nur dort automatisch entfernt; keine fremden Cache-Löschungen.

## Spec — konkrete Befunde

### P2-1: Verborgene Produktzeile/-gruppe bleibt ein diagnoseloser Preis

**Ort:** `overview.rs:521–527, 612–647, 790–803`.

Der DOM-Walk prüft weder `hidden` noch inline `display:none` am Produkt oder dessen Vorfahren. `text()` nimmt sämtliche DOM-Texte. Eigene Mutationen der echten Fixture:

```text
MET-KU-MILL: <tr> -> <tr hidden>                 EXIT 0; 60/55/5; diagnostics=[]
MET-KU-MILL: <tr> -> <tr style="display:none">   EXIT 0; 60/55/5; diagnostics=[]
<section id="kupfer"> -> <section hidden ...>  EXIT 0; 60/55/5; diagnostics=[]
```

Eine zurückgezogene, noch im DOM versteckte Zeile ist damit weiterhin ein aktuelles numerisches Angebot. Die bekannte Captionklasse `visually-hidden` ist dagegen legitime barrierefreie Tabellenbeschriftung und darf nicht pauschal als verstecktes Produkt behandelt werden. Erforderlich: eindeutig verborgene Produkt-/Tabellencontainer blockieren bzw. ausfiltern **mit fehlendem-Artikel-Diagnostik**, Tests auf versteckte Vorfahren; unbekannte Sichtbarkeitsformen nicht als bewiesene Sichtbarkeit ausgeben. Vollständige CSS-/Browserauswertung ist hier nicht behauptet.

### P2-2: Einheit und Zahl werden nur teilweise validiert

**Ort:** `overview.rs:647–699`, insbesondere `has_quote = ...contains("/kg")`; Verwendung des absichtlich toleranten Legacy-Helfers `traders/mod.rs:210–227`.

```text
erste echte Millberry-Zelle /kg -> /kgCO2       EXIT 0; 60/55/5; diagnostics=[]
/kg -> /kg nur Sondervertrag                   EXIT 0; 60/55/5; diagnostics=[]
11,13 € -> 1.1,13 €                            EXIT 0; 60/55/5; diagnostics=[]
```

Der Kandidat erzeugt trotzdem `unit="EUR/kg"` und einen normalen `upto`-Preis. Der Zusatzvertrag bleibt zwar in `Article.price_cell` erhalten, wird aber nicht als unbekannte Preisbedingung diagnostiziert. `1.1,13` ist keine korrekte deutsche Tausendergruppierung, wird jedoch zu 11,13 normalisiert und besteht die Staffelprüfung. Dagegen schlagen `/t`, negative Preise und leere Zellen korrekt fehl. Erforderlich: vollständige, verankerte Grammatik der sichtbaren Preiszelle einschließlich exakt `/kg`, Zahlengruppierung und genau einer Mengenbedingung; zusätzliche unbekannte Bedingungen blockieren. Kein globaler Umbau des Legacy-Helfers nötig.

### P2-3: Betreiberidentität ist eine Substring-, keine Identitätsprüfung

**Ort:** `overview.rs:561–575`.

```text
Impressum Industriestraße 4 -> Industriestraße 40  EXIT 0; diagnostics=[]
Impressum HRA 61664 -> HRA 616640                   EXIT 0; diagnostics=[]
```

Beide geänderten Identitäten erfüllen `contains(expected)`. Bei Hausnummer 40 liefert `extract_info` zudem tatsächlich die veränderte Adresse weiter. Die im Check-in behauptete Ablehnung geänderter Betreiberidentität ist deshalb zu weitgehend. Erforderlich: abgegrenzte/exakte strukturierte Adress- und Registernummernprüfung; keine tolerante Präfixprüfung. Die gesicherte Fixture selbst passt zum erwarteten Betreiber, und ein neuer amtlicher Registerbeleg wird zu Recht nicht behauptet.

### P2-4: Neues `SW10040 -> zinn` erfasst ausdrücklich ausgeschlossene Geschirrwaren

**Ort:** `overview.rs:390`; bestehender Katalog `pipeline.rs:255–259`.

Katalog `zinn`: „Reinzinn und hochprozentiges Zinn (**ohne Löt- und Geschirrwaren**)“. Die Detailquelle `/tmp/opencode/mkm-detail-SW10040.html`, Canonical `/metallankauf/produkt/zinn-99`, nennt ausdrücklich „**hochreines Geschirrzinn**“, „**Zinngeschirr und Zinnfiguren aus Nachlässen**“ neben Bangka-Zinn, Anoden, Barren und Blöcken. Der Gesamtartikel wird ohne Einschränkung auf den ausschließenden Slug gemappt. Ein vorhandener Slug und grüne TempDB-Auflösung beweisen keine fachliche Deckung.

Erforderlich: Owner-Entscheidung für eine passende ganze Quellenfraktion oder ausdrücklich belegte Einschränkung; bestehende Katalogdefinition nicht still überschreiben und keinen Artikel automatisch in mehrere identische Preisangebote duplizieren. `zinn-geschirr` allein wäre für den Gesamtartikel ebenfalls zu eng. Die additive `zinn-legierungen`-Fraktion für **90 %** löst diesen **99-%**-Konflikt nicht automatisch.

### P1 — bekannte Aktivierungssperre, kein neuer Implementierungsauftrag

`overview.rs:46–61, 598–605, 1105–1240`; `traders/mod.rs:347–398`: Der vollständige Artikelvertrag bleibt außerhalb `HandlerOutcome`. `record()` erhält nur Overview-URL, Rohlabel und generierte `extra_json={map_v,...}`, nicht Artikel-ID/Deep-Link, Mindestmenge, Staffel oder Produktdatum. Falscher alter Plugkey und weggefallener Current-Key bleiben bestehen. **Nicht auf vollständigen Handler umregistrieren**, bevor Shared-Owner Metadaten und atomare quellenspezifische Reconciliation nachgewiesen hat. Keine zweite Seam in diesem Review implementiert.

## Standards

Keine `AGENTS.md`, kein `docs/agents/issue-tracker.md` in diesem Worktree gefunden; keine zusätzlichen projektspezifischen Reviewstandards hergeleitet. `git diff --check 6d5ef762` erfolgreich. Die Scope-Trennung ist sinnvoll: Kandidatenmodul, read-only Example und explizites `production_ready=false`; reservierte Shared-Pfade unverändert.

**Heuristischer, nicht blockierender Smell:** `Mapping` als sechsteiliges Tupel mit `m.0/m.1/m.2/m.3/m.4/m.5` und numerischen Gruppenindizes (`overview.rs:69–76, 767–789`) erschwert semantische Reviews einer 60-Zeilen-Tabelle. Benannte Felder/Gruppe würden die Vertragsprüfung lesbarer machen. Kein Architekturumbau als Voraussetzung für die isolierte Übergabe verlangt. Die fachlichen/Validierungsbefunde oben sind keine kosmetischen Smells.

## Vollständige artikelbezogene Prüfung

Alle 60 Detaildateien vorhanden. Ihre SHA256 stimmen **einzeln** mit `/tmp/opencode/mkm-detail-audit-20261010.json` überein, und alle 60 Canonical-URLs stimmen mit der jeweiligen Kandidaten-URL überein. Python-Audit erneut ausgeführt: 60 Detailidentitäten, sämtliche sichtbaren Staffelzeilen gegen Overview-`data-staffel`, **keine Vertragslücken**. Titel, Deep-Link und Tabellenzugehörigkeit des aktuellen Overview sind artikelweise mit dem Kandidaten abgeglichen, nicht allein 60/55/5 verglichen. Die Materialbewertung unten erfolgte zusätzlich anhand der Detailbeschreibungen und Katalogtexte, nicht aus dem Audit-Count.

Tabelle: numerisch = **Höchstwert EUR/kg**, „ab“ = Mindestgesamtmenge für diesen Höchstwert; Anfrage = kein Preis. Datum ist **Produktänderungsdatum**, kein freigegebenes Veröffentlichungsdatum. Für neue Artikel ist die Variant der exakte Overview-Rohsortentitel; die elf Legacy-Ausnahmen sind unten separat fixiert. „OK“ bedeutet fachlich passend für den gespeicherten Artikel innerhalb dieser Grenze, nicht produktiv persistiert.

| Gruppe | Artikel | Materialslug | Höchstwert | ab kg | Produktdatum | Bewertung aus Detailquelle |
|---|---|---|---:|---:|---|---|
| Kupfer | MET-KU-MILL | kupfer-millberry | 11,13 | 250 | 2026-09-28 | OK, blanker Draht |
| Kupfer | MKM-CU-01 | kupfer-berry | 10,92 | 250 | 2026-09-28 | Legacy-Ausnahme: blanker Stanzabfall, kein flexibler Draht |
| Kupfer | MKM-CU-03 | kupfer-gemischt | 10,92 | 250 | 2026-09-28 | Legacy, Variante Sammelschienen bleibt |
| Kupfer | MKM-CU-02 | kupfer-granulat | 10,50 | 250 | 2026-09-28 | OK, Kabelgranulat ohne Isolierung |
| Kupfer | SW10013 | kupfer-gemischt | 10,40 | 250 | 2026-09-28 | Legacy, massive Teile Basis 95 % |
| Kupfer | SW10014 | kupfer-berry | 10,29 | 250 | 2026-09-28 | OK/Legacy, blanker Draht 98 % |
| Kupfer | MET-KU-RAFF | kupfer-gemischt | 10,29 | 250 | 2026-09-28 | OK/Legacy, Raffinationskupfer |
| Kupfer | MKM-CU-04 | kupfer-gemischt | 6,93 | 250 | 2026-09-28 | Legacy, dünn/beschichtet/verzinnt |
| Kupfer | MET-KA-70 | kabel-kupfer | 5,99 | 250 | 2026-09-09 | OK/Legacy, 70 % Leiteranteil |
| Kupfer | SW10017 | cu-ms-kuehler | 5,10 | 250 | 2026-07-27 | OK, Cu-Netz/Messingkästen, eisenfrei |
| Kupfer | MET-KA-60 | kabel-kupfer | 4,94 | 250 | 2026-09-09 | OK/Legacy, NYM/60 % |
| Kupfer | SW10015 | kabel-kupfer | 3,89 | 250 | 2026-10-09 | OK/Legacy, 38–40 % |
| Kupfer | SW10016 | kabel-mit-stecker | 1,94 | 250 | 2026-10-09 | OK/Legacy, Stecker dürfen bleiben/30 % |
| Kupfer | SW10035 | kabel-alu-cu | 1,04 | 250 | 2026-07-26 | OK, beide Leitermetalle, keine Kühlerfraktion |
| Messing/Rotguss | SW10024 | bronze-rotguss | 9,14 | 250 | 2026-10-09 | OK, Stücke separat von Spänen |
| Messing/Rotguss | SW10025 | bronze-rotguss | 7,88 | 250 | 2026-10-09 | OK, Cu-Sn-Zn-Späne |
| Messing/Rotguss | SW10020 | messing | 7,35 | 250 | 2026-10-02 | OK, leere Hülsen/MS72; keine Munition annehmen |
| Messing/Rotguss | SW10019 | messing | 7,09 | 250 | 2026-08-23 | OK, MS63 |
| Messing/Rotguss | SW10018 | messing | 6,62 | 500 | 2026-10-09 | OK, MS58 |
| Messing/Rotguss | SW10021 | messing | 6,20 | 500 | 2026-10-09 | OK, massive Messingteile, nicht Kupfer schwer |
| Messing/Rotguss | SW10023 | messing | 5,57 | 500 | 2026-10-09 | OK, trockene Zerspanungsspäne |
| Messing/Rotguss | SW10022 | messing-leicht | 4,52 | 500 | 2026-07-26 | OK, dünnwandiges Messing, nicht Kupfer leicht |
| Aluminium | SW10011 | alu-cu-kuehler | 2,94 | 500 | 2026-10-09 | OK, Cu-Rohre/Alulamellen, eisenfrei |
| Aluminium | MKM-AL-06 | aluminium-profile | 2,34 | 250 | 2026-09-29 | OK, blanke neue Profile |
| Aluminium | SW10001 | aluminium-profile | 2,10 | 500 | 2026-10-09 | OK mit Rohvariant, maximal 10 % Eloxal, kein Lack |
| Aluminium | SW10002 | aluminium-felgen | 1,89 | 500 | 2026-10-09 | OK, sauber; eigene Variante |
| Aluminium | SW10000 | aluminium-isoprofile | 1,89 | 500 | 2026-08-26 | OK, Kunststoff-Isoliersteg bleibt, Glas/Beschläge ab |
| Aluminium | MKM-AL-01 | aluminium-profile-beschichtet | 1,89 | 500 | 2026-09-24 | OK, lackiert/pulverbeschichtet/foliert |
| Aluminium | SW10003 | aluminium-felgen | 2,03 | 10000 | 2026-10-02 | OK, Reifen/Fremdmaterial; vergüteter Metallanteil beachten |
| Aluminium | SW10004 | aluminium-gemischt | 1,47 | 500 | 2026-08-25 | OK mit Rohvariant, Geschirr/max. 10 % Anhaftung |
| Aluminium | MKM-AL-02 | aluminium-blech | 1,37 | 500 | 2026-09-24 | OK, neu/blank, ohne Folie |
| Aluminium | MKM-AL-03 | aluminium-gemischt | 1,37 | 500 | 2026-09-24 | OK, Form-/Legierungsmischung, max. 2 % Anhaftung im Detail |
| Aluminium | MKM-AL-04 | aluminium-guss | 1,30 | 10 | 2026-09-24 | OK, sauberer Guss |
| Aluminium | MKM-AL-05 | kabel-alu | 0,90 | 10 | 2026-09-24 | OK, isolierte Aluleiter ohne Cu |
| Aluminium | SW10012 | aluminium-spaene | 0,63 | 500 | 2026-10-09 | OK, Basis 5 % Feuchte/Öl, Detail nennt Legierung 226 |
| Edelstahl | SW10028 | edelstahl-v4a | 1,66 | 250 | 2026-10-09 | OK, Mo-haltig; 1,5 m echte Annahmegrenze |
| Edelstahl | SW10026 | edelstahl-v2a | 0,84 | 500 | 2026-10-09 | OK, 1.4301; 1,5 m echte Annahmegrenze |
| Edelstahl | SW10029 | edelstahl-v2a | 0,53 | 500 | 2026-10-09 | OK, Späne getrennte Variante |
| NE | MKM-NE-06 | wolfram | 49,35 | 50 | 2026-09-30 | OK, metallisch ab 99 %, kein WC-Hartmetall |
| NE | SW10040 | zinn | 31,50 | 100 | 2026-09-09 | **P2-4**, Gesamtangebot umfasst Geschirrzinn |
| NE | SW10039 | zinn-legierungen | 25,73 | 100 | 2026-09-09 | OK, Feinlot und Zinngegenstände/rund 90 % |
| NE | SW10042 | loetzinn | 15,75 | 100 | 2026-09-09 | OK, Sn60/Pb40 |
| NE | MKM-NE-05 | nickel | 11,90 | 10 | 2026-09-24 | OK, Reinnickel ab 99 %, eigene Variante |
| NE | MKM-NE-03 | loetzinn | 8,50 | 10 | 2026-09-24 | OK, Zinn-Blei-Lot/Badreste |
| NE | SW10041 | loetzinn | 7,35 | 250 | 2026-09-09 | OK, Zinn-Blei-Lot/rund 30 % |
| NE | MKM-NE-04 | nickel | 6,90 | 10 | 2026-09-24 | OK, Ni-Legierung 60–99 %, eigene Variante |
| NE | MKM-NE-01 | platinen | Anfrage | — | — | OK, Begutachtung statt erfundenem Kilopreis |
| NE | MKM-NE-02 | neodym-magnete | Anfrage | — | — | OK, NdFeB-Magnete, kein Neodymmetallpreis |
| Blei/Zink/Motoren | SW10031 | zink | 2,10 | 500 | 2026-10-09 | OK als breite Zinkfraktion, Detail auch Druckguss/Anoden |
| Blei/Zink/Motoren | SW10030 | blei-gemischt | 1,26 | 500 | 2026-09-09 | OK, Rohr-/Walzblei **und** Auswuchtgewichte |
| Blei/Zink/Motoren | SW10033 | elektromotoren | 0,53 | 500 | 2026-10-09 | OK, max. 500 kg **je Motor**, nicht Lieferhöchstmenge |
| Blei/Zink/Motoren | MKM-BL-01 | blei-geschoss | 0,42 | 500 | 2026-09-24 | OK, Kugelfang/Bleischrot, Feuchte/Sand beachten |
| Hartmetall | MKM-HM-01 | hartmetall | 54,20 | 50 | 2026-10-08 | OK, gemischte Hartmetallformen, Analyse |
| Hartmetall | SW10037 | hartmetall | 54,20 | 30 | 2026-10-08 | OK, Vollhartmetall/WC-Co |
| Hartmetall | SW10036 | hartmetall | 54,20 | 30 | 2026-10-08 | OK, Widia-Wendeplatten/WC-Co |
| Hartmetall | MKM-HM-02 | hartmetall | 49,20 | 30 | 2026-10-07 | OK, massive Verschleiß-/Ziehteile |
| Hartmetall | SW10038 | hss-werkzeuge | 1,70 | 10 | 2026-07-27 | OK, hochlegierter Werkzeugstahl, kein Hartmetall |
| Schrott | MKM-SCH-01 | mischschrott | Anfrage | — | — | OK, Eisen/Stahl ohne nennenswerten NE-Anteil |
| Schrott | MKM-SCH-03 | metallschrott-gemischt | Anfrage | — | — | OK, überwiegender Buntmetallanteil |
| Schrott | MKM-SCH-02 | schrott-eisen-ne-gemischt | Anfrage | — | — | OK, Eisen und NE ungetrennt/Begutachtung |

### 14 additive Katalogfraktionen / Dubletten

Die tatsächliche `MATERIAL_CATALOG`-Konstante hat 81 Einträge und **keine doppelten Slugs** (67 bestehende + 14 additive); Diff verändert keine bestehende Definition. Gleiches Material mit unterschiedlicher Rohvariant ist keine doppelte Preisidentität. Der Parser prüft die 55 `(material, variant)`-Keys separat (`overview.rs:845–852`). Keine kollabierten Stück-/Späne-, Nickelgehalts- oder Felgenzustandsvarianten gefunden.

| Neuer Slug | Detailartikel | Fachliche Abgrenzung zu bestehenden Fraktionen |
|---|---|---|
| kupfer-granulat | MKM-CU-02 | rieselfähig nach Kabelaufbereitung, kein Millberry/isoliertes Kabel |
| cu-ms-kuehler | SW10017 | Kupfer/Messing statt Alu/Cu |
| kabel-alu-cu | SW10035 | beide Leitermetalle, nicht Kabel-Cu/Kabel-Alu |
| aluminium-spaene | SW10012 | lose Zerspanungsspäne mit Feuchte-/Ölbasis |
| aluminium-profile-beschichtet | MKM-AL-01 | Lack/Pulver/Folie statt blanker Profile |
| aluminium-isoprofile | SW10000 | thermische Kunststoffstege statt blanker Vollprofile |
| nickel | MKM-NE-04/05 | nickelreiche Legierung/Reinnickel, keine Edelstahlfraktion |
| wolfram | MKM-NE-06 | Metall statt Wolframcarbid oder HSS |
| neodym-magnete | MKM-NE-02 | NdFeB statt Reinmetall; nur Anfrage |
| zinn-legierungen | SW10039 | breite 90-%-Warenfraktion inkl. bleifreiem Lot und Gegenständen; weder nur Sn-Pb-Lot noch nur Geschirr |
| blei-gemischt | SW10030 | Gesamtangebot inkl. Auswuchtblei, bestehendes `blei` schließt dieses aus; keine Akkuannahme |
| blei-geschoss | MKM-BL-01 | Kugelfang/Schrot statt sauberes Rohrblei/Auswuchtblei |
| metallschrott-gemischt | MKM-SCH-03 | NE-Anteil überwiegt; nicht bestehender Eisenmischschrott |
| schrott-eisen-ne-gemischt | MKM-SCH-02 | Eisen/NE ungetrennt, nicht bestehender reiner Eisen-/Stahlmix |

Diese 14 Abgrenzungen sind plausibel und belegbar. Union-Fraktionen `blei-gemischt`/`zinn-legierungen` überschneiden Teilmengen vorhandener Sorten bewusst, ersetzen sie aber nicht. Es wird kein zweiter MKM-Preis auf den Teilmengen-Slugs erzeugt. Anfrageartikel erhalten keine fingierte kg-/t-Quote; `EUR/t` im Eisenkatalog ist nur Katalogdefault, kein belegter MKM-Anfragepreis.

### Elf stabile Legacy-Identitäten und fachliche Grenze

Unverändert gegenüber aktuellem registriertem Mapping: `kupfer-millberry/""`, `kupfer-berry/Kerze`, `kupfer-gemischt/Sammelschienen`, `kupfer-gemischt/schwer`, `kupfer-berry/Berry 98%`, `kupfer-gemischt/Raff`, `kupfer-gemischt/leicht`, `kabel-kupfer/70%`, `kabel-kupfer/60%`, `kabel-kupfer/38-40%`, `kabel-mit-stecker/Stecker 30%`. Keine Mengen in Legacy-Variants hineinbenannt. Der historische falsche `kabel-kupfer/Stecker 30%` ist **nicht** eine zu stabilisierende zwölfte Identität.

**Explizite Legacy-Ausnahme:** MKM-CU-01 ist gemäß Detailquelle Stanzabfall; `kupfer-berry` wird im Katalog als flexibler Draht/Litze beschrieben (`pipeline.rs:115–119`). Der Kandidat übernimmt diese bereits bestehende fachliche Unschärfe wegen der verlangten Legacy-Stabilität. Deshalb kein neuer Change-Regression-Befund und keine eigenmächtige Migration, aber auch keine Behauptung, alle elf Legacy-Identitäten seien durch Stabilität fachlich neu bewiesen. Ebenso bleiben die bereits breit verwendeten `kupfer-gemischt`-Varianten erkennbar und getrennt.

## Audit-Oracle: was unabhängig geprüft wird und was nicht

`scripts/audit-mkm-overview.py:104–137` vergleicht Artikelmengen **und** Gruppe, Titel, URL, sichtbare Preiszelle, Preis, Mindestmenge, Datum, Anfragezustand und komplette Staffel artikelweise. Das ist wesentlich stärker als Count-Gleichheit. `compare_details:140–176` gewinnt Staffeln aus sichtbaren Detailtabellen statt aus deren Overview-JSON; alle 60 Detailidentitäten und Hashes sind vorhanden. Betreiberseiten sind allerdings dieselbe Primärquelle, keine unabhängigen Register- oder Marktquellen.

**Direkt geprüfte Oracle-Grenze:** Alle 60 `material`-Werte zu `batterien-blei` und sämtliche `variant`-Werte zu `WRONG` geändert, ansonsten unverändertes Kandidaten-JSON: `compare_candidate(...) == []`. Das Audit fordert nur nichtleeres Material und prüft keine Variantsemantik. `every_article_partitioned...` gewinnt seine Erwartungsartikel wiederum aus der Implementierungs-`MAPPINGS`-Tabelle. Diese Tests sind Vertrags-/Partitionsprüfungen, **keine unabhängige Fachmapping-Oracle**. Der Check-in beschreibt diese Grenze überwiegend ehrlich; dessen Satz „vollständiger Quellen-/Mappingkandidat“ darf nicht als uneingeschränkte semantische Freigabe gelesen werden. Dieser Review ergänzt gerade die Detail-/Katalogentscheidung und findet P2-4.

Auch versteckte DOM-Zeilen prüft der Python-Audit nicht auf Sichtbarkeit. Ein übereinstimmender Parser/Audit kann deshalb P2-1 gemeinsam übersehen. Änderung der Betreiberadresse/Registernummer wird überhaupt nicht von diesem Overview-Audit geprüft. Keine zusätzliche grüne Count-Gate als Ersatz verlangen.

## Tests, Logs und Herkunft der Evidenz

### Selbst ausgeführt

1. `python3 -B -m unittest discover -s scripts -p test_audit_mkm_overview.py -v`: **7/7 bestanden**, Exit 0. Kein Bytecode im Worktree erzeugt.
2. `python3 -B scripts/audit-mkm-overview.py crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261010.html --candidate-json research/mkm-full-live-inventory-20261010.json --detail-dir /tmp/opencode`: Exit 0, `candidate_article_contract_gaps=[]`, `detail_contract_gaps=[]`, `production_approval=false`. Voller Tool-Rohoutput: `/root/.local/share/opencode/shell/42aa29071d6c1a29b482e0269f73393202254764/sh_12790d484001cuy6apfSHphE3X.out`.
3. Eigene offline Mutationsläufe gegen bereitgestelltes `/tmp/opencode/mkm-full-target-20261010/debug/examples/mkm_inventory`; SHA256 `bb0c9f9a4703adfc2bceb92001f22ee09c3c784e023880bc877a148377e8fd0b`. Invocation jeweils `mkm_inventory <eigene-mutierte-overview.html> <fixture-impressum.html>` bzw. eigenes mutiertes Impressum. **Vorhandenes Agent-Binary, kein unabhängiger Neu-Build behauptet.** Quellcode bestätigt die beobachteten Validierungslücken.
4. Weitere negative Kontrollen derselben echten Fixture:

```text
Baseline                         EXIT 0; 60/55/5; diagnostics=[]
bekannte Zeile entfernt           EXIT 1; 59/54/5; missing known article; retirement NOT authorized
Artikel-ID UNKNOWN statt bekannt  EXIT 1; 60/54/5; unknown article + missing known article
echte Zeile dupliziert            EXIT 1; duplicate article MET-KU-MILL
Datum entfernt                   EXIT 1; numeric article without machine-readable product change date
Preiszelle leer                  EXIT 1; ambiguous quote/request or unit
Preis + auf Anfrage              EXIT 1; ambiguous quote/request or unit
Preis negativ                    EXIT 1; malformed price number
```

5. Manifest-/Canonical-Check: 60/60 Hashes gleich, 60/60 Canonical-URLs gleich. Slug-Duplikatcheck: 81 Einträge, keine Dubletten. `git diff --check` sowie Patch-Reverse-Check erfolgreich.

### Agent-Evidenz, nicht als eigene Testausführung ausgegeben

Der Check-in meldet 13 Rust-MKM-Tests, 563 Ingestion-Tests und 5 ignorierte Live-Tests sowie echte TempDB-Aufzeichnung. Deren Code wurde gelesen; **diese Rust-Suites und TempDB wurden hier nicht erneut ausgeführt**. Der vorhandene Target enthält kein `debug/deps`-Testexecutable mehr. Bei nur ca. 1,8 GiB freiem `/tmp` wurde weder ein riskanter neuer voller Target-Build gestartet noch ein fremder Cache gelöscht. Keine Workspace-/Clippy-Freigabe aus Agentzahlen abgeleitet. Der Check-in nennt das fehlgeschlagene strikte Clippy-Gate ausdrücklich.

Der TempDB-Test `real_tempdb_record_path_proves_catalog_coverage_and_shared_seam_gaps` benutzt echte `PublicDb`/`InternalDb`, `seed_metadata`, `record` und einen Disposable-Pfad, keine Produktions-DB. Er prüft alle Materialauflösungen, 55 gespeicherte `upto`-Preise, 5 Annahmen und explizit zurückbleibende falsche/entfernte Schlüssel plus append-only Historie. Er prüft fehlende `article`, `tiers`, `product_change_date` in realem `extra_json`, sowie Dedupe beim unveränderten Preis. Das ist ein glaubwürdiger, codebezogen nachvollziehbarer **Verlustnachweis**, keine Lösung des Verlusts.

Grenzen: Der Artikelvertrag wird bereits beim Übergang von `Snapshot.articles` zu `HandlerOutcome` nicht transportiert; der Test zeigt den resultierenden DB-Verlust, nicht einen vollständigen Metadatenroundtrip. URL, vollständige Mengen-/Detailbedingungen, Metadatenänderung bei gleichem Preis, Numeric→Anfrage-Retirement, atomarer Rollback, legitime Artikelentfernung und andere Quellen/Händler sind keine bewiesenen neuen Shared-Seam-Features. `recorded=55` zählt Verarbeitung, nicht 55 neue Historyeinträge; das wird durch History-Dedupe korrekt relativiert. Die erste TempDB-Aufzeichnung ohne Publikationsdatum beweist nicht, dass `record()` niemals ein Datum inferiert: unveränderte Shared-Logik kann bei späterer beobachteter Preisänderung `observed_price_change` setzen.

## Preis-/Datumsvertrag und Abschluss

- Im Kandidaten überall `upto`, `price_max=price`, `price_min=None`, EUR/kg und Confidence 0,5; keine Nullpreise für die fünf Anfragen. Alle sichtbaren Staffeln bleiben im Artikelvertrag erhalten. Millberry 11,13 gilt ab 250 kg, unsaubere Felgen 2,03 erst ab **10.000 kg**; Kleinmengen haben niedrigere Staffelwerte.
- `published_at=None` im Kandidaten ist korrekt vorsichtig. Seitenänderung 09.10.2026 und Produktdaten 26.07.–09.10.2026 sind verschiedene Evidenz. Kein Produktdatum wird pauschal zum Veröffentlichungsdatum umetikettiert.
- Detailbedingungen sind nicht sämtlich maschinenlesbar im Kandidaten: beispielsweise Vergütung nur des Metallanteils bei unsauberen Felgen, Anhaftungsgrenzen im Detail und die Motor-Einzelgewichtsgrenze. Vor produktiver Lossless-Freigabe muss klar sein, welche Bedingungen der neue gemeinsame Vertrag trägt bzw. nur über die verlinkte Detailquelle dokumentiert; ein nackter Höchstpreis plus `upto` ist kein universeller Auszahlungsanspruch.
- Fehlende/unbekannte/geänderte Identitäten blockieren die Example-Freigabe über Diagnostics; allerdings bleiben gültige Teilpreise im `outcome`. Das ist für ein proof-only Inventar vertretbar, **kein atomarer Fail-closed-Recordvertrag**. `production_ready` bleibt auch beim grünen Snapshot immer false.

**Zusammenfassung:** Standards: 0 nachgewiesene harte Repo-Regelverletzungen, 1 nicht blockierender Tupel-Smell. Spec: 4 neue P2-Kandidatenbefunde; bekannte P1-Shared-Seam-/Aktivierungssperre bleibt. Der aktuelle Quellenbestand ist vollständig nachgewiesen, nicht die vollständige fachliche/produktive Behebung. Nach Parser-/Identitätskorrekturen und Owner-Entscheidung zu SW10040 erneut prüfen; Registrierung bis zu separatem Shared-Owner-End-to-End-Nachweis unverändert Legacy lassen.

---

## Angehängter Behebungsstand des Implementierers — 11.10.2026

**Die vorstehenden unabhängigen Befunde bleiben unverändert als Reviewhistorie erhalten.** Folgendes ist neue Implementiererevidenz, noch keine unabhängige Nachprüfung:

- Vier offline P2-Regressionen wurden vor Korrektur alle rot ausgeführt (Exit 101), danach alle grün. Versteckte Zeilen/Section-/Preisvorfahren erzeugen keinen aktuellen Millberry-Preis, sondern fehlende bekannte Artikel mit ausdrücklich gesperrtem Retirement; legitime `visually-hidden`-Captions bleiben.
- Gesamte Preiszelle jetzt verankert mit exaktem `€ /kg ab … kg`-Vertrag und eigener strenger deutscher Zahlengruppierung. `/kgCO2`, Sondervertrag, `1.1,13`, mehrfaches `ab` und unbekannte Zusätze abgelehnt. Legacy-Zahlenhelper unberührt.
- DDG-Betreiber-/Adressabsatz exakt geprüft; volle Registernummer aus Registerabsatz, keine Präfixprüfung. Echte Impressumsmutationen Straße 40/HRA 616640/PLZ/Firmenzusatz/doppelte Nummer scheitern.
- SW10040 jetzt einzige ganze Quellenfraktion `zinn-rein-formgemischt / Zinn 99%`, einschließlich hochreiner Geschirr-/Figurenwaren neben Anoden/Barren/Bangka-Zinn; enger bestehender `zinn`-Ausschluss und `zinn-geschirr` bleiben unverändert. TempDB überprüft tatsächliche Katalogdefinition.
- Separate feste 60-Artikel-Material/Variant-Oracle aus diesem Review und zwei expliziten Fachkorrekturen; nicht aus `MAPPINGS` gewonnen. All-Batterien-/WRONG-Audit war real rot und wird jetzt erkannt, unbekannte Oracleartikel bleiben ausdrücklich nicht geprüft. Historisches Kandidaten-JSON vom 10.10. bleibt erhalten und wird mit neuer Oracle an genau Kerze/Zinn99 als falsch diagnostiziert.
- Kerze-Kandidat fachlich korrigiert auf `kupfer-stanzabfaelle / Kerze`, registrierter Handler weiterhin Legacy. Exakte künftige quellenspezifische Alias-/Retirementmigration mit Historien- und Fremdquellenschutz ist in `research/checkin-20261010-mkm-full.md` spezifiziert und als `required_alias_migrations` im neuen Kandidaten-JSON markiert. Keine fachliche Freigabe aus bloßer Legacy-Stabilität.
- Gates: 17 MKM-Tests, 567 Ingestion-Tests bestanden/5 ignoriert; 9 Python-Tests; Rustfmt-/Diffcheck erfolgreich. Neu abgerufene Overview-/Impressumsquellen vom 11.10. mit neu gebautem Parser: 8/60/55/5, keine Diagnostik, keine Vertrags-/Oraclelücken. Artefakte mit Datum 11.10. neu angelegt, alte Artefakte nicht überschrieben.

**Weiterhin P1 gesperrt:** kein vollständiger Shared-Metadaten-/Retirementvertrag, keine Aliasmigration aktiviert, keine produktive Registrierung geändert. Reservierte `crates/store` und `traders/mod.rs` unberührt. Kein Commit/Push/Deploy/Produktionswrite. Unabhängiger Re-Review wird separat protokolliert; vollständige Details und Quelle/Hashes im Behebungsbericht.

### Re-Review-Ausführungsgrenze (Implementierer, 11.10.)

Beide nach Behebung angeforderten unabhängigen Reviewer (Standards/Spec) wurden vom Harness **vor Start** wegen `Subagent depth limit reached (1)` abgelehnt. Deshalb **keine unabhängige Behebungsfreigabe** und kein neuer Reviewerbericht behauptet. Owner muss außerhalb dieses Subagenten einen separaten Re-Review durchführen. Die vorstehenden P2-Behebungen sind eigene nachgewiesene Tests, kein Ersatz für den angeforderten unabhängigen Review; P1 bleibt zusätzlich offen.

---

## Unabhängiger Re-Review — 11.10.2026

### Ergebnis

**Die vier ursprünglichen P2 sind für den dokumentierten Kandidatenscope behoben. Begrenzte Kandidatenfreigabe; keine Produktionsfreigabe.** Die eigene Nachprüfung reproduziert jetzt die erwarteten Fehler für versteckte Produkt-/Preisvorfahren, `/kgCO2`, unbekannte Vertragszusätze, `1.1,13`, Straße 40 und HRA 616640. Die Zinn99-Gesamtfraktion ist passend korrigiert. Kerze ist ebenfalls fachlich korrigiert, aber die zwingende Legacy-Alias-/Retirementmigration bleibt offen. Keine neuen blockierenden Change-Befunde in diesem Re-Review.

Die historischen Findings, die damalige Ergebnistabelle und der Implementierernachtrag oben bleiben unverändert. Dieser Abschnitt aktualisiert deren Status, nicht ihre damalige Evidenz. Die 14 ursprünglich additiven Fraktionen sind jetzt um **zinn-rein-formgemischt** und **kupfer-stanzabfaelle** ergänzt; insgesamt 16 additive Fraktionen, 83 Katalogeinträge, keine Slug-Dubletten.

### Geprüfter Stand / Scope

- Tatsächlicher `git diff 6d5ef762` und sämtliche relevanten ungetrackten Dateien geprüft, einschließlich neuer separater Oracle und neuer Evidenz vom 11.10.; nicht bloß Implementiererbericht übernommen.
- Vollpatch `/tmp/opencode/mkm-full-p2-20261011.patch`: SHA256 `03926531a812b7e227704fe0f80b6c786158356d11d6899a51d568d4cc5adc7c`. **Vor** diesem Reviewappend war `git apply --reverse --check` erfolgreich. Der Patch enthält den Implementiererstand, noch nicht diesen neuen Reviewabschnitt.
- Parser SHA256 `fb97cf5a532aa521856dac92fa883278027b504626da467295385ff59c9805f2`.
- Historischer Vollpatch vom 10.10. weiterhin SHA256 `608ec348d5078a74da5bfff9bc128b76e5b34639455712b7b9b06066c231b1a5`, also unverändert.
- Reserved-Pfadvergleich `git diff 6d5ef762 -- crates/store crates/ingestion/src/traders/mod.rs` weiterhin leer. `mkm_metals.rs:35–81` registriert weiterhin den Legacy-`scrape()` und dessen Kupferparser. Keine versteckte Aktivierung des vollständigen Kandidaten oder der Kerze-Korrektur.
- Dieser Reviewer hat nur den bestehenden Reviewbericht ergänzt. Keine Code-/Mainänderungen, Commits, Produktionszugriffe oder -writes, neuen HTTP-Abrufe oder Shared-Seam-/Sourcegate-Doppelarbeit. Owner/MSG behält diesen Bereich.

### P2-Nachprüfung mit eigenen Mutationen

Verwendet wurde das am 11.10. neu gebaute bereitgestellte echte Example-Binary `/tmp/opencode/mkm-full-target-20261010/debug/examples/mkm_inventory`, SHA256 `68a17dccf86a6bca5e9a5468f6468173fcd9446f618275f8338e8c03f97dd53c`. **Eigene Blackbox-Läufe, kein eigener Rust-Neu-Build behauptet.** Alle Mutationen sind aus der echten 10.10.-Overview-/Impressumsfixture erzeugt. Die erste echte `MET-KU-MILL`-Produktzeile wurde per Artikel-ID ausgewählt, nicht eine Header-/Navigationserwähnung. Für jede Mutation wurde zusätzlich geprüft, dass die Eingabe wirklich von der Basis abweicht. Eigene temporäre Dateien lagen ausschließlich in `TemporaryDirectory(prefix="mkm-independent-rereview-", dir="/tmp/opencode")`.

Invocation: `mkm_inventory <eigene-overview> <eigenes-impressum>`; die selbst beobachteten Logs:

| Mutation | Exit | Eigene Beobachtung |
|---|---:|---|
| unveränderte Basis | 0 | 60 Artikel, 55 numerisch, 5 Anfrage, Diagnostics leer, `production_ready=false` |
| Produktzeile `hidden` | 1 | 59/54/5; Millberry fehlt, `retirement NOT authorized` |
| Produktzeile `style="display:none"` | 1 | 59/54/5; gleiche Sperre |
| Kupfersection-Vorfahr `hidden` | 1 | 46/41/5; alle 14 Kupferartikel ausdrücklich fehlen, kein Retirement |
| Kupfersection `DISPLAY : none !important` | 1 | 46/41/5; alle 14 Kupferartikel ausdrücklich fehlen |
| Preiszellen-Vorfahr `hidden` | 1 | 59/54/5; Millberry fehlt, kein Preis aus verstecktem Preisinhalt |
| Einheit `/kgCO2` | 1 | `unknown complete price-cell grammar: 11,13 € /kgCO2 ab 250 kg` |
| Zusatz `/kg nur Sondervertrag` | 1 | `unknown complete price-cell grammar` |
| Zahl `1.1,13 €` | 1 | `unknown complete price-cell grammar` |
| Impressum `Industriestraße 40` | 1 | `operator identity/address changed` |
| Impressum `HRA 616640` | 1 | `operator register identity changed` |

**P2-1 geschlossen im expliziten DOM-/Inline-Scope:** `overview.rs:553–575, 707–734` prüft versteckte Vorfahren und Preisnachfahren, bekannte ausgelassene Artikel werden am Ende diagnostiziert. `visually-hidden`-Captions bleiben legitim. Externe Stylesheets, responsive Regeln und vollständige Browser-/CSS-Sichtbarkeit werden nicht bewiesen; Kandidaten-JSON erklärt diese Grenze ausdrücklich. Keine weitergehende Sichtbarkeitsfreigabe daraus.

**P2-2 geschlossen:** `overview.rs:577–623, 749–778` validiert die gesamte normalisierte Preiszelle, exaktes `€ /kg ab … kg`, korrekt gruppierte deutsche ganze Mengen und Preise mit zwei Centstellen sowie Übereinstimmung mit dem sichtbaren `strong`. Unbekannte Suffixe werden nicht mehr still als EUR/kg normalisiert. Der Legacy-Zahlenhelper bleibt unberührt.

**P2-3 geschlossen:** `overview.rs:631–664` prüft den normalisierten Betreiber-/Adressabsatz exakt und den vollständigen Registerwert aus dessen eigenem Absatz. Die zwei ursprünglichen Präfixrepros sind selbst nachgewiesen gesperrt. Keine amtliche Registerprüfung oder allgemeine Betreiberverifikation über diese gespeicherte Identität hinaus freigegeben.

**P2-4 geschlossen:** `overview.rs:399–406` ordnet SW10040 genau einmal `zinn-rein-formgemischt / Zinn 99%` zu. `pipeline.rs:21–22` beschreibt die **ganze** nahezu unlegierte Quellenfraktion, einschließlich Bangka-Zinn, Anoden, Barren, Blöcken und hochreinen Geschirr-/Figurenwaren; nicht nur Geschirr und kein niedrigprozentiges Lot. Die ursprünglichen Definitionen von `zinn` mit Geschirrausschluss und `zinn-geschirr` werden im Diff nicht geändert. Die zwei frisch gespeicherten Detailquellen für SW10040/MKM-CU-01 wurden selbst gegen ihre Manifest-SHA256 geprüft; beide stimmen. Der neue Zinn99-Detailtext bestätigt weiterhin die zuvor fehlende breite Warenabdeckung. Keine Preisduplizierung auf Teilmengen-Slugs gefunden.

### Artikelpartition und unabhängige Oracle — kein Count-Ersatz

Die separate feste `fixtures/mkm-reviewed-mapping-oracle-20261011.json` enthält **60 Artikel → jeweils Material und Variant**. Eigener Gegencheck: alle 60 Materialsollwerte stimmen mit der zuvor unabhängig erstellten Detail-/Katalogtabelle dieses Reports überein, außer den beiden explizit erneut begründeten Korrekturen Zinn99/Kerze. Sämtliche Variants bleiben gegenüber dem historischen Kandidaten unverändert. Die Erwartungen werden im geprüften Testcode aus der separaten Datei gelesen, **nicht aus `MAPPINGS` zur Laufzeit abgeleitet** (`overview.rs:1111–1149`). Gemeinsame Primärquellen bleiben gemeinsame Primärquellen; diese Oracle ist ein fachlich überprüftes Sollartefakt, keine zweite unabhängige Markt-/Registerquelle.

Eigene direkte Aufrufe von `compare_candidate` auf dem echten Snapshot:

```text
Baseline gegen neue Oracle                    []
alle 60 Materialien -> batterien-blei
und alle 60 Variants -> WRONG                 genau 120 Fehler
                                             60 Materialfehler + 60 Variantfehler,
                                             ausdrücklich 2 je Artikel geprüft
historischer Kandidat vom 10.10.              genau 2 Materialfehler:
                                             MKM-CU-01: Berry -> Stanzabfälle
                                             SW10040: zinn -> zinn-rein-formgemischt
Oracle-Eintrag SW10040 entfernt               mapping semantics NOT CHECKED
```

Damit ist die alte Oracle-Lücke im geprüften Fixed-Oracle-Vertrag geschlossen: nichtleerer Materialtext oder korrekter Count allein genügen nicht mehr. `scripts/audit-mkm-overview.py:113–156` prüft Material und Variant jeweils; fehlende fachliche Erwartungen erzeugen ausdrücklich eine Lücke. Ein Zukunftsartikel erfordert weiterhin eine neue fachliche Reviewentscheidung statt automatischer Mappingfreigabe.

Zusätzliche **eigene** Parserkontrollen, alle Exit 1: bekannte Zeile entfernt; Artikel zu `UNKNOWN` geändert bei weiterhin 60 Zeilen; echte Zeile dupliziert; Titel geändert; URL geändert; numerischen Preis gelöscht; Preis plus `auf Anfrage`. Unknown/Missing/Changed liefern explizite Diagnostics, Duplikat/broken/ambiguous liefern Parsefehler. Ein defekter Preis wird nicht als Anfrageannahme umgedeutet. Das belegt mehr als unveränderte 55/5-Zahlen.

### Frischer gespeicherter Snapshot / fachlicher Delta

Das echte Binary wurde zusätzlich **selbst offline** auf `/tmp/opencode/mkm-p2-overview-20261011.html` und `/tmp/opencode/mkm-p2-impressum-20261011.html` ausgeführt. Ergebnis stimmt **vollständig** mit `research/mkm-full-live-inventory-20261011.json` überein: 8/60/55/5, leere Diagnostics, keine Produktionsbereitschaft. Quellenhashes selbst bestätigt:

- Overview: `0bf08f47395c3436e97c8c002b150f4bb89a38d08425599a4f33610e3a0baa7e`.
- Impressum: `77ca243d58b3e3127eccfb6edaf5bc17e36d0cbd5063ef166f56370e7f153f6d`.

Eigener erneuter Auditaufruf auf genau dieser gespeicherten 11.10.-Overview, neuem Kandidaten-JSON und `/tmp/opencode`-Detailinventar: Exit 0, `candidate_article_contract_gaps=[]`, `detail_contract_gaps=[]`, 60 Detailartikel, `production_approval=false`. Die Detailstaffeln sind weiterhin die gespeicherten 60 Detailabrufe vom 10.10.; keine Behauptung 60 neuer Detailabrufe durch diesen Reviewer.

Eigener artikelweiser Vergleich 10.10. → 11.10. ergibt: exakt dieselben 60 Artikelidentitäten; **keine Änderungen** an Gruppe, Titel, Produkt-URL, Preiszelltext, Preis, Mindestmenge, vollständigen Staffeln, Produktdatum oder Anfragepartition. Genau **zwei** Materialänderungen (Kerze/Zinn99); **keine** Variantänderung. Preis-/Datumsevidenz bleibt erhalten, Höchstwerte werden nicht zu bedingungslosen Kleinmengenpreisen und Produktdaten nicht zu Veröffentlichungsdaten.

Kerze ist jetzt fachlich korrekt `kupfer-stanzabfaelle / Kerze` (`overview.rs:95–101`; additive Beschreibung `pipeline.rs:23–24`). Der registrierte Legacykey `(kupfer-berry, Kerze)` wird weiterhin technisch verwendet, aber nicht fachlich neu freigegeben. Die künftige quellenspezifische, atomare, historienerhaltende Migration steht in `research/checkin-20261010-mkm-full.md:27–37` und als `required_alias_migrations` im tatsächlich geprüften JSON (`overview.rs:55–60`). Keine Aliasmigration implementiert, kein historischer Preis umgeschrieben. Die zehn anderen bisherigen Material/Variant-Identitäten bleiben stabil; die bekannte Steckerkabelmigration bleibt ebenfalls offen.

### Eigene Gates / nicht übernommene Claims

- Selbst ausgeführt: 18 eigene Blackbox-Fälle (Baseline + 17 Mutation-/Partitionkontrollen), Fixed-Oracle-Mutation mit exakt 120 Fehlern, fehlender Oracleeintrag, historische Oraclekonflikte, artikelweiser Snapshotvergleich, frischer gespeicherter Parser-/Auditlauf, Detailquellenhashes und 83-Einträge-Slug-Duplikatcheck.
- Selbst ausgeführt: `python3 -B -m unittest discover -s scripts -p test_audit_mkm_overview.py -v`, **9/9 bestanden**, Exit 0; `git diff --check 6d5ef762` erfolgreich.
- Implementiererclaims **17 MKM-/567 Ingestion-Tests plus 5 ignorierte Tests** sind nicht als eigene Rust-Suite-/TempDB-Ausführung ausgegeben. Zugehöriger Testcode und Änderungen geprüft. Bei ca. 1,6 GiB freiem `/tmp` kein zusätzlicher voller Rust-Target-Build und keine fremden Cache-Deletes; die eigene Verhaltensnachprüfung nutzt das oben gehashte Binary. Kein globales Clippy-/Workspace-Grün behauptet.
- Standards: keine neue harte Regelverletzung gefunden; der historische nicht blockierende Tupel-Smell bleibt. Exact-Source-Grammatik und präzise dokumentierte Sichtbarkeitsgrenze sind für diesen isolierten Proofkandidaten vertretbar.

### Endgültige Freigabegrenze dieses Re-Reviews

**Freigegeben:** nicht aktivierter MKM-Quellen-/Mappingkandidat für die geprüften gespeicherten Snapshots und expliziten Sichtbarkeits-/Grammatik-/Identitätsverträge; vier alte P2 geschlossen, zwei Fachkorrekturen begründet, vollständige 60-Artikel-Partition samt 55 Preisen/5 Anfragen und separater Material-/Variant-Oracle nachgewiesen.

**Nicht freigegeben / P1 bleibt:** produktive Registrierung, Lossless-Record-Roundtrip von Artikel-ID/URL/Menge/Staffel/Produktdatum, atomare source-/trader-spezifische Reconciliation, Kerze-/Steckerkabelmigration, legitim entfernte Artikel, Numeric→Anfrage-Retirement, Metadatenupdates bei gleichem Preis, Rollback/Idempotenz/Fremdquellenschutz und Produktions-End-to-End-Abgleich. Die im Erstreview belegten Shared-Record-Verluste sind nicht durch Parser-P2-Fixes geheilt. Teilpreise im diagnostischen `Outcome` sind weiterhin kein atomarer akzeptierter Vollsnapshot. Sourcegate-/Shared-Seam-Arbeit bleibt beim Owner/MSG; keine parallele zweite Implementierung oder Produktionsfreigabe durch diesen Report.

## Implementierer-Nachtrag 2026-10-11 — neuer Seam-Kandidat, kein unabhängiges Re-Review

Der Owner hat anschließend MKM-beschränkte Store-/Scheduler-Seam-Arbeit ausdrücklich erlaubt; MSG-Basis `e3fcbbae` ist im isolierten MKM-Branch integriert. Typed Article→Record→TempDB, beide quellengeschützten Aliasretirements, immutable Konditions-/Datumsevidenz, 55/5 und 54/6 sowie vollständiges 0/60-Requestretirement, Duplicate/Late/Conflict, SQL-Rollback und Foreign-source/producer/trader-Schutz sind jetzt implementiert und durch konkrete Tests belegt. Workspace 651 passed / 5 ignored, MSG separat 13 passed; registrierter Read-only-Livepfad 8/60/55/5, unabhängiger Artikel-/Mappingaudit ohne Gaps. Vollständige Evidenz und bewusst erhaltene Fehl-/Zwischenstände: `research/checkin-20261011-mkm-production-seam.md` und `research/mkm-typed-seam-evidence-20261011.json`.

Dieser Nachtrag ändert **nicht rückwirkend** die obenstehende Reviewerfreigabe. Neuer Store-/Transport-/Registrierungskandidat muss vollständig durch den Owner reviewed werden, bevor Integration/Produktionsaktivierung erlaubt ist. Kein Push, Deploy oder Produktionswrite erfolgt; kein selbst ausgestelltes unabhängiges Review. Die historische Legacy-Kerze-Registrierung ist im neuen Kandidaten test-only, nicht mehr der aktive Parserpfad.

### Weiterer Implementierer-Nachtrag: adversarialer Fullreview / fünf behobene Findings

Das unabhängige Fullreview `/tmp/opencode/mkm-adversarial-review-20261011/REVIEW.md` hat Freigabe verweigert (2P1/3P2). Alle fünf Repros selbst bestätigt, danach F1 first-request Legacycutover, F2 bestehender Owner-/öffentlicher Historyschutz, F3 hidden Labelnachfahren, F4 defektes vorhandenes Seitendatum und F5 neue Staffelbedingungen gezielt behoben. Beleg mit realen Parser→typedRecord→TempDB-Regressionen, vollständiger History/Current/Acceptance/Manifest-Errorprüfung, RED-/GREEN-Logs und finalen Gates in `research/checkin-20261011-mkm-review-remediation.md`. Workspace657/5ignored, MKM33, Store40, MSG13, Python9 und frischer registrierter Liveaudit8/60/55/5 ohne Artikel-/Mappinggaps. **Kein unabhängiges Re-Review hiermit ersetzt**; erneutes unabhängiges und Owner-Fullreview bleiben erforderlich, Implementation uncommitted und Prod unangetastet.
