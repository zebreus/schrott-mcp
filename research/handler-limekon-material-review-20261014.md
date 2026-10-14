# Limekon — unabhängiger fachlicher Materialreview

Stand: **14.10.2026**. Händler: `nw-doerentrup-limekon`. Gegenstand ist die Sollzuordnung der realen Preisliste, **kein Review eines Implementierungsdiffs**. Handler, Registry und Fixtures des parallel arbeitenden Implementierers wurden nicht bearbeitet. Ausschließlich dieses Dokument wird geschrieben; keine Dossiers, Produktionszugriffe, TLS-Ausnahmen, Commits, Pushes oder Deployments.

## Urteil

Die dedizierte Preisliste enthält **27 nichtleere Originalzeilen**, darunter eine Formel. Mit dem vorhandenen Katalog sind **15 numerische Richtpreisbeobachtungen fachlich eindeutig abbildbar**, **12 Zeilen bewusst zu skippen**. Die 50%-Zinnlegierung, Eisen, Kupfer raff und schweres Messing sollen nicht aufgrund eines übervorsichtigen Edelmetall-only-Mappings verlorengehen. Der Münzen-/Barrenpreis auf Feingewicht wird für den üblichen `ScrapedPrice`-Pfad hingegen zunächst nicht freigegeben: eine EUR/g-Bruttoquote daraus zu machen wäre falsch. Ein späterer, nachweislich feinbasisfähiger Pfad kann diese eine Zeile zusätzlich abbilden.

Alle freigegebenen Werte: **`price_kind="approx"`, `confidence=Some(0.5)`, `price_min=None`, `price_max=None`**, EUR, Ankauf. Die Quelle kennzeichnet die Tabelle ausdrücklich als **„Richtpreise“**; „ab“ im Datumsheading ist kein Preisuntergrenzenoperator. Weder `exact` noch `upto` oder ein erfundener Mindestpreis sind daraus abzuleiten. [Q1, R2, R3]

## Quellen und Repo-Vertrag

### Primärquellen, am 14.10.2026 direkt abgerufen

- **Q1 — maßgebliche Preisquelle:** [Limekon Preisliste](https://limekon.de/pages/preisliste.php), sowohl Markdown/Text als auch Original-HTML gelesen. `#beginContent`, Heading **„Preisliste ab 14.10.2026“**, zweispaltige `tbody`-Tabelle ohne `thead`, danach sichtbares **„Richtpreise“**. HTML enthält `&gt;=90`, `&lt;90`, `&nbsp;` und bei „Messer <90“ ein zusätzliches `p` im Preis-`td`.
- **Q2 — Kontext/Quellenkonflikte, nicht zweite Preisfeedquelle:** [Limekon Homepage](https://limekon.de/). Erklärt Ankauf, Gewichtsmultiplikation und Versandtag-/Eingangstag-Mindestpreisgarantie. Preisbox **„Preisliste ab 14.10.2026 08:00“**, abweichende Grade und zusätzliche Silberzeile. Allgemeine Homepage-Aussage „€ je kg“ überschreibt keine konkreten €/g-/Stück-Zellen aus Q1.
- **Q3 — Objektklassen:** [Limekon Definitionen](https://limekon.de/pages/home/definitionen.php). Modeschmuck ist Restware nach Aussortierung wert-/edelmetallhaltiger Gegenstände, muss nicht einmal aus Metall bestehen. Weltmünzen können Münzen, **Noten**, Gedenkmünzen und Medaillen umfassen; sortenreine Metall-/Nominalbestände sind nicht pauschal eingeschlossen. Daher keine generische Metall-/Edelmetallquote für diese Labels.

### Tatsächlich gelesene Codequellen

- **R1 — Katalog:** [`crates/ingestion/src/pipeline.rs`](../crates/ingestion/src/pipeline.rs), `MATERIAL_CATALOG`, insbesondere Z. 21–54, 98–116, 168–172, 238–277, 489–522. `zinn-legierungen` ist bereits vorhanden; `loetzinn` ist ausdrücklich Zinn-Blei-Lötware. `silber`/`gold`/`zahngold` sind EUR/g, `mischschrott` EUR/t; kein Katalogmaterial für versilberte Objektklassen, Double, Weltmünzen oder kombinierte Blei/Zink-Fraktion.
- **R2 — Preis-/Varianten-/Recordingvertrag:** [`crates/ingestion/src/traders/mod.rs`](../crates/ingestion/src/traders/mod.rs), `ScrapedPrice` Z. 100–120, `HandlerOutcome` Z. 156–180, `parse_eur`/`parse_de_date` Z. 211–257, `record` Z. 390–480 und `normalize_unit`. Varianten sind Händleruntergrade und Bestandteil der Current-Identität `(trader, material, variant)`; kein eigenes typisiertes Feingewichtsfeld. Das Recording bewahrt `label` als `notes`, schreibt `source_type="haendler_angabe"`, `published=true` und explizites Datum mit `published_at_basis="page_stated"`. Zentrale Einheitenkonversion betrifft kg↔t, **nicht kg↔g**.
- **R3 — Richtpreispräzedenz:** [`crates/ingestion/src/traders/handlers/gerwischer.rs`](../crates/ingestion/src/traders/handlers/gerwischer.rs), Z. 1–9, 41–61: Richtwerte → `approx`/0.5, ohne künstliche Grenzen.
- **R4 — Variantenpräzedenz:** [`crates/ingestion/src/traders/handlers/scheideanstalt_ka.rs`](../crates/ingestion/src/traders/handlers/scheideanstalt_ka.rs), Z. 1–10, 96–129: Feingehalte als Varianten, Münzen-/Barrenklassen getrennt von gewöhnlichen Goldlegierungen. Das ist **kein** Nachweis, dass jede EUR/g-fein-Quote in eine EUR/g-Bruttoquote umgeschrieben werden darf.
- **R5 — Warnender Altpräzedenzfall:** [`crates/ingestion/src/traders/handlers/nes_scheideanstalt.rs`](../crates/ingestion/src/traders/handlers/nes_scheideanstalt.rs), Z. 1–11, 107–119: dort Silber/kg bewusst ohne Umrechnung geskippt. Für Limekon ist die explizite lineare kg→g-Konversion beweisbar und lokal möglich; dieser alte Scope-Verzicht ist **kein fachliches Verbot**, Silber fein abzudecken.

Die Repo-Suche nach `SourceUnit`/`MaterialVariant` ergab in den gelesenen Rustquellen keine bestehenden gleichnamigen Typen. „SourceUnit“ unten bezeichnet deshalb den **empfohlenen lokalen Parservertrag**, nicht eine behauptete vorhandene Infrastruktur. Maßgeblich ist der tatsächliche `ScrapedPrice`-Vertrag R2.

## Vollständige Sollzuordnung der Originalzeilen

Originallabels aus Q1; Schreibweisen der Varianten sind konkrete Empfehlungen. Äquivalente stabile Schreibweisen sind zulässig, wenn sämtliche Qualifikationen erhalten bleiben. Jede freigegebene Zeile ist `approx`/0.5. Preise sind die **beobachteten** Werte, nicht aus Feingehalten nachgerechnet. [Q1, R1, R2]

| Nr. | Originalzeile | Quellwert / SourceUnit | Sollslug / Variante | Zielwert und Entscheidung |
| --- | --- | --- | --- | --- |
| 1 | Zinn fein / 99er | 37,00 €/kg | `zinn` / `99er` | **37,00 EUR/kg**, aufnehmen. Konkrete hochprozentige Zinnquote; keine ausdrückliche gemeinsame Geschirr-/Formenfraktion, daher nicht automatisch `zinn-rein-formgemischt`. |
| 2 | Geschirrzinn 85-97% | 28,50 €/kg | `zinn-geschirr` / `85-97%` | **28,50 EUR/kg**, aufnehmen. Legierter Geschirrgrad exakt erhalten, nicht generisches `zinn` oder Homepagegrad. |
| 3 | Zinnlegierungen ca. 50% | 17,50 €/kg | `zinn-legierungen` / `ca. 50%` | **17,50 EUR/kg**, aufnehmen. Katalog für gemischte legierte Zinnwaren vorhanden. Sn-Anteil allein beweist weder Lötverwendung noch Blei als Legierungspartner; **nicht `loetzinn`**, nicht Reinzinn. „ca.“ gehört zum Materialgehalt, nicht zu einem erfundenen Preisintervall. |
| 4 | Besteck versilbert >=90 | 55,00 €/kg | **SKIP — kein passender Slug** | Versilberte Objektklasse, keine massive Silberlegierung. `>=90` nicht als 90%/900er Ag lesen. |
| 5 | Besteck versilbert <90 | 26,00 €/kg | **SKIP — kein passender Slug** | Wie Nr. 4, eigener Auflagegrad; nicht mit >=90 zusammenführen. |
| 6 | Messer >=90 | 18,50 €/kg | **SKIP — kein passender Slug** | Messer im versilberten Tabellenblock, eigene Objektklasse/Preis; weder Silberfeingehalt noch Besteckpreis erfinden. |
| 7 | Messer <90 | 8,00 €/kg | **SKIP — kein passender Slug** | Wie Nr. 6, eigener Grad. Verschachteltes `p` verhindert nicht den vollständigen Zelltext. |
| 8 | Korpus versilbert | 20,00 €/kg | **SKIP — kein passender Slug** | Versilberte Hohlkörper, kein massives `silber`; keine Umrechnung zu 0,020 EUR/g als Silberquote. |
| 9 | Silber fein / 999 | 1.000,00 €/kg | `silber` / `999` | **1,00 EUR/g**, aufnehmen: 1000 EUR/kg **÷1000**. Kein Skip nur wegen Katalogeinheit. Keine zusätzliche Division durch 0,999 oder Metallgehalt. |
| 10 | Silber 800er | 0,80 €/g | `silber` / `800` | **0,80 EUR/g**, aufnehmen. Preis der angegebenen Legierung, nicht pro Gramm Feinsilber. |
| 11 | Silber 925er | 0,93 €/g | `silber` / `925` | **0,93 EUR/g**, aufnehmen. Nicht 0,93/0,925 normalisieren. |
| 12 | 5 DEM Silber | 7,00 €/Stück | **SKIP — Katalog-/Einheitenbasis passt nicht** | Eigenständiger Münzstückpreis, kein Grammgewicht angegeben. Weder pauschales Münzgewicht/Ag-Gehalt recherchieren und unterstellen noch EUR/Stück in `silber` EUR/g verstecken. |
| 13 | andere Legierungen | Silberanteil * fein | **SKIP — Formel, keine numerische Quote** | Silberblock liefert Kontext, aber kein bestimmter Feingehalt/Preis/Unit. Keine hypothetischen 835er-/900er- oder sonstigen Preise erzeugen. |
| 14 | Gold 999 / fein | 105,00 €/g | `gold` / `999` | **105,00 EUR/g**, aufnehmen. Spezifizierter 999er Grad; nicht dieselbe Basis/Objektklasse wie Nr. 18. |
| 15 | Gold 750er | 78,75 €/g | `gold` / `750` | **78,75 EUR/g**, aufnehmen. Keine Feinpreisdivision. |
| 16 | Gold 585er | 61,42 €/g | `gold` / `585` | **61,42 EUR/g**, aufnehmen. Keine Neuberechnung aus 105 × 0,585. |
| 17 | Gold 333er | 34,65 €/g | `gold` / `333` | **34,65 EUR/g**, aufnehmen. Der beobachtete Wert bleibt 34,65, auch wenn 105 × 0,333 = 34,965 wäre. |
| 18 | Münzen / Barren (auf fein) | 107,00 €/g fein | **SKIP im Standardpfad — Feinbasis nicht verlustfrei abgesichert** | Fachlich **Gold**, aus Goldblock; wäre `gold` / `Münzen/Barren (auf fein)` mit **107 EUR/g Feingold**, ausdrücklich **keine behauptete 999er Legierung**. Nicht als 107 EUR/g Bruttomaterial ausgeben und nicht mit Nr. 14 kollabieren. Siehe Feinbasisgate unten. |
| 19 | Zahngold gelb | 69,00 €/g | `zahngold` / `gelb` | **69,00 EUR/g**, aufnehmen. Eigenständige Dentalfraktion; keine unbekannte Feinheit/Karatzahl ergänzen. |
| 20 | Zahngold weiß | 25,00 €/g | `zahngold` / `weiß` | **25,00 EUR/g**, aufnehmen. Stabil getrennt von gelb; keine Zuordnung zu Platin/Palladium aus Farbe. |
| 21 | Double | 250,00€/kg | **SKIP — kein passender Slug** | Double als beschichtete/vergoldete Ware ist keine definierte massive Goldlegierung. Zwar rechnerisch 0,25 EUR/g Ware, aber das macht daraus keinen `gold`-Preis. Auch nicht 250 EUR/g. |
| 22 | Eisen | 0,10 €/kg | `mischschrott` / `Eisen` | **100,00 EUR/t**, aufnehmen: ×1000. Breiter Eisen-/Stahlschrott-Katalog, ohne konkrete Sorte. Nicht Neuschrott, Schere, Gussbruch oder Stahlträger behaupten. Variante hält das unspezifische Händlerlabel sichtbar. |
| 23 | Modeschmuck | 6,00 €/kg | **SKIP — kein passender Slug** | Gemischte Objekt-/Restfraktion, ausdrücklich möglicherweise nichtmetallisch (Q3). Nicht `gold`, `silber` oder „überwiegend NE“-Mischmetall. |
| 24 | Weltmünzen | 3,00 €/kg | **SKIP — kein passender Slug** | Gewichtsquote auf heterogenem Sammelgut einschließlich Noten (Q3), keine konkrete Silber-/Gold-/Kupferfraktion. Nicht mit 5-DEM-Stückpreis verwechseln. |
| 25 | Kupfer raff | 9,00 €/kg | `kupfer-gemischt` / `Raff` | **9,00 EUR/kg**, aufnehmen. Katalog nennt Raff ausdrücklich; nicht Millberry, Berry oder blankes Rohrkupfer. |
| 26 | Messing schwer | 5,00 €/kg | `messing` / `schwer` | **5,00 EUR/kg**, aufnehmen. Schweres Messing liegt im breiten Messingkatalog; Variante statt erfundenem `messing-schwer`-Katalogslug. Nicht `messing-leicht`. |
| 27 | Blei/Zink | 1,00 €/kg | **SKIP — kombinierter mehrdeutiger Grad** | Keine eindeutige Entscheidung zwischen getrenntem Altblei, Zink oder Blei/Zink-Verbund. Weder zwei Preiszeilen fan-outen noch `blei-gemischt` verwenden (gemischte **Bleiformen**, kein Blei-Zink-Gemisch). |

Die drei vollständig leeren Trennzeilen aus Q1 sind Layout, keine zusätzlichen Originalmaterialzeilen oder Skips. Nr. 13 muss als Formel-Skip sichtbar bleiben, obwohl keine Zahl erkannt wird. **15 Price-Zeilen + 12 begründete Skips = 27 fachliche Zeilen.**

## Quellepriorität und Datum

1. **Q1 bleibt alleinige Preisquelle** und `source_url`/`fetch_url`. Homepage Q2 weder als stiller Fallback noch zum Vervollständigen der Grades verwenden. [Q1, Q2]
2. Konkrete Konflikte: Q1 `Geschirrzinn 85-97%`, **28,50**; Q2 `Zinn 80-99%`, **28,550**. Letzteres darf weder den Grad überschreiben noch als 28,50 gerundet eingemischt werden. Q2 hat zusätzlich `Silber 835` zu **0,84 €/g**, das in Q1 **nicht** vorhanden ist. Q1 hat dagegen Stückmünze, Formel und Feinbasis-Münzen/Barren. Q1 spezifiziert `Kupfer raff`/`Messing schwer`, Q2 nur `Kupfer`/`Messing`. Gleiche Tagesüberschrift beweist nicht gleiche Grades. [Q1, Q2]
3. `published_at` für Q1: Datum **14.10.2026**, im vorhandenen `parse_de_date`-Speicherformat **`2026-10-14T00:00:00+00:00`**, mit `page_stated`. UTC-Mitternacht ist hier der Repo-Datumscontainer, **kein Beleg für tatsächliche Veröffentlichung um 00:00 UTC**. Beobachtungszeit getrennt erfassen. [Q1, R2]
4. **08:00 nicht von Q2 übernehmen** und ohne Zeitzonenbeleg keine sekundengenaue UTC-Veröffentlichung behaupten. „ab“ heißt geltende Liste seit genanntem Tag; keine erfundene `valid_to`, `valid_from` oder Ablaufdauer. Ungültige Kalenderdaten dürfen nicht akzeptiert werden. [Q1, Q2, R2]

## SourceUnit- und Feinbasisgate

### Quellbasis vor Zielkonversion erkennen

Ein lokaler `SourceUnit`-Begriff muss mindestens **€/kg**, **€/g**, **€/Stück**, **€/g fein** unterscheiden. Die Formel hat keine numerische Preiseinheit. **`€/g fein` darf nicht durch `contains("/g")`, Präfixmatching oder Strippen von `fein` zu gewöhnlichem €/g werden.** Kein Default auf Katalogeinheit bei fehlender/unverständlicher Quellunit. Preis ausschließlich aus der rechten Zelle lesen: andernfalls gewinnt z.B. `999`, `800`, `50` oder `5 DEM` gegen den tatsächlichen Geldbetrag. Deutsche Gruppierung `1.000,00` bedeutet 1000, nicht 1 oder 100000. [Q1, R2]

Zwei freigegebene Skalierungen sind transparent: **Silber/kg ÷1000 → Silber/g**, **Eisen/kg ×1000 → Eisen/t**. Erstere muss im Händlerpfad erfolgen, denn R2 normalisiert zentral nur kg↔t. Letztere kann entweder dort oder vorab stattfinden, aber niemals doppelt. Im Parser Quellpreis und Quellunit zusammenhalten; bei Silberkonversion Rohtext `1.000,00 €/kg` zusätzlich in der zulässigen Preisprovenienz/`notes` erhalten, wenn das übliche Label allein ihn sonst verliert. Keine zentrale Recordingänderung für diesen Review gefordert. [Q1, R2]

### Warum Nr. 18 nicht einfach „999 Münzen/Barren“ ist

Ein Feinpreis berechnet den Ankauf aus **Feingoldgewicht**, nicht zwingend aus Waren-/Bruttogewicht. Beispiel ausschließlich zur Risikoerklärung: 10 g einer 750er Goldmünze enthalten 7,5 g fein; bei 107 €/g fein ergibt das 802,50 EUR und nicht 1070 EUR. Die Quelle gibt für Nr. 18 **keinen festen Feingehalt** an, also kann der Handler keine allgemeine Bruttoquote konstruieren. Die 999er Quote Nr. 14 ist dagegen ein separat veröffentlichter Legierungsgradpreis. [Q1]

R2 erlaubt zwar freie Varianten und Unit-Strings und bewahrt Rohlabels, aber besitzt keinen eigenen Gewichtsbasistyp. Nur ein Label/Variantenzusatz bei Unit **`EUR/g`** belegt noch nicht, dass nachgelagerte Preisvergleichs-/Mengenrechner feinbasisrichtig arbeiten. Auch der R4-Münzenpräzedenzfall rechtfertigt keine ungeprüfte Umdeutung. **Für den initialen Standardhandler ist der begründete Skip die Freigabeempfehlung.** Die übrigen 15 Preise sind davon unabhängig vollständig zu übernehmen. [R2, R4]

Eine alternative Aufnahme von Nr. 18 wäre nur nach einem separat nachgewiesenen End-to-End-Gate zulässig: eigenständige Variante `Münzen/Barren (auf fein)`, tatsächlich bewahrte Basis **EUR/g fein**, keine Feinheitsannahme, keine automatische Normierung auf Brutto-EUR/g, keine unqualifizierte Vergleichs-/Berechnungsintegration. Das Erfinden eines neuen Unitschemas oder einer Infrastrukturänderung gehört nicht zu diesem Auftrag. Der Review fordert daher **keine** Katalogerweiterung, um diese Zeile um jeden Preis einzuspeisen.

## Konkrete Reviewer-Checkliste

- [ ] Genau `nw-doerentrup-limekon`, Preisprovenienz Q1; kein Homepagepreisfeed/Fallback oder unbemerktes Zusammenlegen zweier Tabellen.
- [ ] Echtes HTML als Fixture: Tabelle ohne `thead`, leere Trennzeilen, Entities/NBSP, `<p>` in Messerpreis und sichtbare Richtpreisnotiz funktionieren. Anker auf tatsächlichen Preisblock statt „erste Tabelle“/Ganzseitenzahlen.
- [ ] **15** Preise nach obiger Standardfreigabe, **12** nichtleere fachliche Skips mit nachvollziehbarer Begründung; Formel nicht lautlos verloren. Ein abweichender Zähler muss konkret erläutert werden, besonders bei Nr. 18.
- [ ] `zinn-legierungen`/`ca. 50%` enthalten; weder `zinn` noch `loetzinn`, keine künstliche 50%-Preisformel. `zinn-geschirr` trägt 85–97%, nicht Homepage 80–99%.
- [ ] `Eisen` → `mischschrott`/`Eisen`, **100 EUR/t**; `Kupfer raff` → `kupfer-gemischt`/`Raff`, **9 EUR/kg**; `Messing schwer` → `messing`/`schwer`, **5 EUR/kg**. Keine nicht existierenden Slugs oder unnötigen Skips dieser Katalogabdeckung.
- [ ] Silber fein aus **1000 EUR/kg** wird genau **1 EUR/g**, kein 1000×-Fehler oder zusätzlicher 999er Reinheitsfaktor. 800/925 unverändert **0,80/0,93 EUR/g**; **keine 835er Preiszeile**.
- [ ] Goldvarianten 999/750/585/333 bleiben getrennt; **105/78,75/61,42/34,65 EUR/g** unverändert. Keine rechnerische „Korrektur“ von 333 oder 585. Zahngold gelb/weiß separat und nicht reguläres Gold.
- [ ] Versilbertes Besteck, beide Messergrade und Korpus vollständig **aus Preisaufnahme ausgeschlossen**; keine 900er/90%-Ag-Varianten, keine Silberkonversion der Objektpreise.
- [ ] Double, Modeschmuck und Weltmünzen nicht durch Gold-/Silber-/Mischmetall-Substringfallback abbilden. Nr. 12 bleibt Stückpreis-Skip, Blei/Zink ohne Fan-out oder falsches `blei-gemischt`.
- [ ] SourceUnit-Mutationstest: `/g fein` bleibt von `/g` unterschieden; unverständliche/fehlende/abweichende Units führen zu sichtbarem Skip/Fehler, nicht Default. Änderungen von Stück-/Feinbasiszeilen dürfen nie normale Silber-/Gold-Grammpreise erzeugen.
- [ ] Nr. 18 ist im Standardpfad sichtbar geskippt. Falls Implementierung dennoch aufnimmt: oben genanntes **End-to-End-Feinbasisgate konkret belegt**, eigener Variantenschlüssel und keine behauptete 999er Feinheit; ein Parserunit-Test allein genügt nicht.
- [ ] Alle 15 Preise **approx/0.5**, Grenzen leer. Mutation „Richtpreise entfernt“ führt zu bewusster Quellenprüfung/Fehler, nicht stiller Hochstufung auf exact. Die Garantie Q2 hebt den Richtpreischarakter Q1 nicht auf.
- [ ] Datum aus Q1 ist kalendarisch validiert; explizite `page_stated`-Provenienz und getrennte `observed_at`. Keine Q2-Uhrzeit, kein Copyright-/Abrufdatum, keine erfundene Zeitzone/Gültigkeitsfrist.
- [ ] Änderungen/Skips unbekannter Labels und duplicate `(material, variant)` werden sichtbar; keine last-row-wins-Kollision. Rohlabel bleibt in Provenienz, skalierte Quellwerte sind für Prüfung nachvollziehbar.
- [ ] Testnachweise sind lokale Parser-/Recordingtests; kein realer Ingest/Produktionswrite, kein TLS-bypass. Dieser Bericht ist **keine** Behauptung, dass die parallele Implementierung bereits alle Gates bestanden hat.

## Abgrenzung zur Vorrecherche

Die vorhandene Aufnahmegate-Notiz `research/feedback-20261014-limekon-admission.md` wurde nur als Kontext gelesen, nicht als Preisautorität. Ihre zunächst offene 50%-Zinnzuordnung wird durch den **real vorhandenen** Katalogslug `zinn-legierungen` aufgelöst. Dass dort `source_type=published`/`confidence=approx` sprachlich genannt wird, darf nicht in falsche Codefelder übernommen werden: gemäß R2 sind **`source_type="haendler_angabe"`, `published=true`, `price_kind="approx"`, `confidence=0.5`** die tatsächlichen Vertragswerte. Die fachliche Zuordnung oben wurde gegen Q1 und R1 unabhängig hergeleitet.
