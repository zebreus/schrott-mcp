# Preisquellen-Screening: mittelgroße Schrottbetreiber (Abruf 27.10.2026)

Rechercheauftrag: Für mittelgroße Schrottbetreiber klären, **wo und in welchem Format**
sie Preise veröffentlichen. Nicht die Frage, ob eine Webseite erreichbar ist, sondern die
Frage nach einer nutzbaren, maschinenlesbaren Preisquelle.

Grundlage ist das Screening in
[`research/handler-backlog-20261027.md`](handler-backlog-20261027.md), das für
`proaurum.de, schrott24.de, steil.de, isr-recycling.de, ziems-recycling.de,
jakob-becker.de, kunze-group.de, riwald.de, goetz-neu-ulm.de, rmr-recycling.de` an den
üblichen Pfaden keine Preisseite mit Preisangaben gefunden hatte. Dieses Screening war eine
**Pfad-Suche**. Die hier vorliegende Recherche geht tiefer: robots.txt, Sitemap-Baum,
zweite Pfad-Runde, Produktdetailseiten, JSON-Endpunkte, PDF-Downloadcenter, Marktberichte
und maschinenlesbare Zusatzdateien.

## Abgrenzung der Methode

- Alle Abrufe am **27.10.2026** mit identischem, klar erkennbarem User-Agent:
  `Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko)
  Chrome/126.0.0.0 Safari/537.36 (compatible; SchrottIndex price-source research;
  contact: research@schrottindex.de)`.
- **Kein** Umgehen von Bot-Challenges, **kein** Betreten von Login-Bereichen, **kein**
  Umgehen von robots.txt-Sperren. Wo eine Sperre oder eine Anmeldung stand, ist das als
  solches vermerkt und die Quelle wurde nicht abgerufen.
- Kein Betreiber-Code, keine Produktionsdatenbank, kein Dossier wurde angefasst. Reines
  Lesen und Schreiben dieser Notiz.
- Preisangaben wurden **nur** aus Betreiber-Content übernommen. Fremde Marktplätze
  (LME, Fastmarkets, Metalprice, ScrapMonster) sind ausdrücklich als solche markiert und
  **nicht** als Händler-`haendler_angabe` geführt.

## Gesamtergebnis

Von 17 geprüften Domains haben **eine** eine betreibereigene, ohne Anmeldung maschinenlesbare
Preisquelle mit Materialbezeichnung, Preis, Einheit und Datum: **mcschrott.de**.
Eine weitere (**proaurum.de**) veröffentlicht eine vollständig maschinenlesbare
Preisliste, aber für **Edelmetall-Handel**, nicht für Schrottsorten. Eine dritte
(**schrott24.de**) hat einen JSON-Preisendpunkt, publiziert aber **Plattform-Maximalpreise**,
aus denen kein Händlerpreis für ein lokales Dossier wird.

Die übrigen 14 Domains haben **keine** öffentliche Preisquelle. Sie veröffentlichen
Sorten, Annahmekriterien, Zertifikate und AGB, aber keine Kurse.

| Domain | Preisquelle öffentlich? | Format | Einheit/Datum | Realistische Folge |
|---|---|---|---|---|
| **mcschrott.de** | **Ja**, 6 Annahmestellen | HTML-Tabelle + eingebettetes JSON | €/kg, Stand 23.10.2026 | **Handler** |
| proaurum.de | Ja, aber Edelmetall-Shop | HTML-Tabelle + `data-price-amount` | EUR/USD, Zeitstempel | Marktinfo-Pipeline (Gold/Silber) |
| schrott24.de | Ja, aber Plattform-Maximalpreis | HTML + **JSON** `page-data.json` | €/kg, **ohne Datum** | nur Markt-/Plattformkontext |
| steil.de | Nein | Sortenbeschreibung | - | nichts |
| isr-recycling.de | Nein (Kundenkarte) | Sorten + AGB | - | nichts |
| ziems-recycling.de | Nein | Sorten | - | nichts |
| jakob-becker.de | Nein | Zertifikate, AGB | - | nichts |
| kunze-group.de | Nein | Sortenbeschreibung | - | nichts |
| riwald.de | Nein | Zertifikate, AGB | - | nichts |
| goetz-neu-ulm.de | Nein (Marktbericht-PDF) | PDF (BDSV/Fastmarkets) | $/t, datiert | Marktinfo-Pipeline |
| rmr-recycling.de | Nein | - | - | nichts |
| kuehl-entsorgung.de | Nein | Zertifikate, AGB | - | nichts |
| fegert-recycling.de | Nein | Sorten | - | nichts |
| foell-rohstoffhandel.de | Nein | Annahmekriterien | - | nichts |
| schrott-mohr.de | Nein | Sorten + `llms.txt` | - | nichts |
| eberhard-mayr.de | Nein | - | - | nichts |
| wmr-online.de | Nein (Preisformel) | Formel ohne Werte | - | nichts |

---

## 1. mcschrott.de (MC Schrott / Mohr Rohstoff, 6 Annahmestellen) - **Treffer**

Dossiers im Repo: `mv/mv-rostock-mc-schrott-rostock`,
`bb/bb-brandenburg-an-der-havel-mc-schrott-annahmestelle`,
`bb/bb-werder-ot-plotzin-mc-schrott-annahmestelle`,
`bb/bb-kloster-lehnin-ot-reckahn-mc-schrott-annahmestelle`,
`bb/bb-nauen-mc-schrott-annahmestelle`, `bb/bb-rathenow-mc-schrott-annahmestelle`.
Das Auftrags-Screening nannte nur einen Treffer (Homepage mit 9-mal `/kg`), tatsächlich
sind es sechs Dossiers und die Homepage verlinkt auf eine echte Preisseite.

### (a) Welche Preisquellen es gibt und wo

| Quelle | URL | Befund |
|---|---|---|
| Ankaufspreisliste | <https://www.mcschrott.de/preise/> | HTTP 200, 51.579 Bytes, statisches HTML, **keine** Bot-Challenge |
| Standortseiten (Filialfinder) | <https://www.mcschrott.de/standorte/> und `/standorte/<slug>/` | jede Standortseite verlinkt auf `../../preise/?standort=<slug>` |
| Preisrechner | auf `/preise/` eingebaut | reines Client-JS, rechnet aus derselben Liste |
| Preisdokumente (PDF) | `/_preislisten/` | Verzeichnis existiert, **per robots.txt gesperrt** (`Disallow: /_preislisten/`), nicht abgerufen |
| `robots.txt` | <https://www.mcschrott.de/robots.txt> | `Allow: /`, zusätzlich `Disallow: /_src/`, `Disallow: /_preislisten/` |

Die Sitemap (<https://www.mcschrott.de/sitemap.xml>) führt `/preise/` als reguläre Seite.

### (b) Ohne Anmeldung maschinenlesbar?

Ja, vollständig. Es gibt drei maskenlesbare Ebenen auf derselben Seite:

1. **HTML-Tabelle**: 59 Zeilen in sieben Gruppen (Kupfer 15, Aluminium 19, Messing 2,
   Edelstahl & Sonderstähle 6, Blei & Zink 4, Eisen & Mischschrott 8, Sonstiges 5).
   Spalten: `Bezeichnung` (inkl. `span.item-desc` mit der Sorte) und `Preis €/kg`.
2. **Eingebettetes JSON**: `<script id="mcs-prices">` mit **54** Objekten
   `{name, price, groupLabel, priceRos}`. Die 5 fehlenden Positionen sind die
   „auf Anfrage"-Zeilen (Chromstahl, Widiaplättchen, Bohrer, Katalysator klein/groß).
3. **HTML-Attribut je Zeile**: `data-price-ros` trägt den Regionalpreis.

### (c) Format

- Einheit: **EUR/kg**, ausdrücklich **Nettopreise**.
- Datum: **„Stand: 23.10.2026"** im Hero und „unverbindliche Richtpreise auf Basis unserer
  Annahmepreisliste vom 23.10.2026" im Einleitungstext. Das ist ein **Seitendatum**
  (`page_stated`), 4 Tage vor dem Abruf.
- **Preise je Filiale**: Die Seite hat einen Standortumschalter mit sechs Buttons
  (Brandenburg an der Havel, Rathenow, Nauen, Kloster Lehnin/OT Reckahn,
  Werder (Havel)/OT Plötzin, Rostock). Fünf Höfe tragen `data-region="brb"`, Rostock
  `data-region="ros"`. Der Regionspreis steht **nur** im Attribut `data-price-ros`; die
  sichtbare Zelle `<td class="col-price">` zeigt den Standardwert (Brandenburg).
  Sieben Sorten weichen in Rostock ab:

  | Sorte | Brandenburg €/kg | Rostock €/kg |
  |---|---|---|
  | Altschrott Sorte 3 | 0,17 | 0,14 |
  | Mischschrott | 0,15 | 0,11 |
  | Gussschrott | 0,145 | 0,11 |
  | Bremsscheiben | 0,145 | 0,11 |
  | Vorbrennermaterial | 0,15 | 0,12 |
  | Moniereisen fertig | 0,15 | 0,13 |
  | Moniereisen | 0,13 | 0,11 |

  Eine Zeile trägt zusätzlich `data-only="brandenburg rathenow nauen"` (Papier, 0,10 €/kg):
  **Papier wird nur an drei von sechs Standorten angenommen**. Das ist eine reale
  Standortannahme-Aussage, kein Preis.
- Der Tiefenlink `?standort=<slug>` liefert **byte-identisches** HTML. Die Standortauswahl
  wirkt ausschließlich clientseitig. Ein Scraper muss deshalb `data-price-ros` lesen, nicht
  die sichtbare Zelle.
- Zwei Abzüge stehen **nicht** in der Preisliste, sondern werden an der Waage verrechnet:
  „nicht metallische Materialien in einer Lieferung werden mit 0,25 € je Kilogramm
  berechnet", Reifen „30,00 € je PKW-Reifen, 50,00 € je LKW-Reifen".
- Der Betreiber stuft die Werte selbst als **unverbindliche Richtpreise** ein: „maßgeblich
  ist immer die Bewertung und Verwiegung an der Annahmestelle". Der
  <https://www.mcschrott.de/faq/> bestätigt: „Nein. Die Preisliste ist eine unverbindliche
  Richtpreisliste."

### (d) Was realistisch folgt

**Handler.** Das ist der einzige Betreiber der ganzen Gruppe mit einer
handler-tauglichen Quelle: Materialbezeichnung, Preis, Einheit, Datum, maschinenlesbar,
ohne Anmeldung, ohne Challenge, ohne Wechsel der Quelle pro Material.

Zu beachten für den Datenvertrag:

- Der Preis ist ein **Richtpreis**, keine Zusage. Der Katalog kennt dafür `price_kind:
  "approx"` („rounded/guide price“) neben `exact` („Listenpreis“); die Doku
  [`docs/adding-a-trader-handler.md`](../docs/adding-a-trader-handler.md) regelt den
  „exact"-Fall nur als `confidence: Some(1.0)` und sagt **nicht** ausdrücklich, dass
  „unverbindlich“ immer `approx` bedeutet. Der Repo-Schwester-Handler `kernschrott` bucht
  bei wortgleichem „unverbindlich“ als `exact`/1.0, neun weitere Handler nutzen `approx`.
  Für `mc_schrott` (Werte je Sorte, Standort und Stand-Datum veröffentlicht) wurde
  deshalb `exact`/1.0 **einheitlich für alle sechs Zeilen** gewählt, mit der
  Betreiberformulierung im Modul-Doc; eine Umstellung aller sechs Zeilen auf `approx`
  würde die Preise aus dem direkten Vergleich (`PREISVERGLEICHE` wertet nur
  `price_kind='exact'`) nehmen und bleibt eine bewusst offene Entscheidung.
- `published_at` = 23.10.2026 (`page_stated`). Das **Aktualisierungsintervall** ist
  unbekannt: die Liste trägt ein Stand-Datum, aber keinen Änderungsindikator. Ob sie
  wöchentlich, täglich oder nur bei Bedarf neu datiert wird, muss beobachtet werden.
- **Standortdimension**: sechs Dossiers x 59 Sorten. Der Preis ist nicht für alle Höfe
  gleich (siehe Rostock-Spalte). Der Defaultwert im HTML ist die Brandenburg-Gruppe; ein
  Handler, der nur die sichtbare Zelle liest, würde Rostock falsch bedienen.
- Fünf Positionen sind „auf Anfrage". Sie sind als **Acceptance-Beleg** zu behandeln,
  nicht als Preis.
- Der Betreiber sperrt `/_preislisten/` per robots.txt. Es gibt also vermutlich eine
  **PDF-Preisliste je Standort**, die der Betreiber bewusst nicht crawlen lassen will.
  Diese Quelle wurde hier **nicht** abgerufen. Sie ist für einen Handler nicht nutzbar,
  solange der Betreiber die Freigabe nicht erteilt.

### (e) Was zu klären wäre

1. Aktualisierungsintervall der Liste (täglich? wöchentlich?) und ob das Stand-Datum bei
   jeder Änderung mitgeführt wird.
2. Ob die Richtpreise als verbindliche Annahme bei der Waage gelten oder ob ein
   Handshake nötig ist (Ankunftszeitpunkt, Verwiegung).
3. Ob die sechs Höfe eine gemeinsame Preisliste teilen oder je Hof eine eigene führen
   (aktuell teilen sie eine Liste mit einer Regionsabweichung).
4. Freigabe für `/_preislisten/` (PDF) oder für einen offenen API-Endpunkt.
5. Abzugsregeln (0,25 €/kg Nichtmetall, Reifen) als Gebühr in den Datenbestand oder nur
   als Notiz.

---

## 2. proaurum.de (pro aurum GmbH, 11 Dossiers) - **Preisliste vorhanden, aber Edelmetall**

Dossiers: `sn/sn-dresden-pro-aurum-filiale`, `bw/bw-ulm-pro-aurum-filiale`,
`bw/bw-stuttgart-pro-aurum-filiale`, `nw/nw-duesseldorf-pro-aurum-filiale`,
`nw/nw-bielefeld-pro-aurum-filiale`, `be/be-charlottenburg-wilmersdorf-pro-aurum-filiale-berlin`,
`sl/sl-saarbruecken-pro-aurum-filiale`, `he/he-bad-homburg-pro-aurum-filiale`,
`hh/hh-altstadt-pro-aurum-vertrieb-filiale-hamburg`,
`by/by-muenchen-schwabing-pro-aurum-filiale`, `by/by-muenchen-riem-pro-aurum-goldhaus`.

### (a) Welche Preisquellen es gibt und wo

| Quelle | URL | Befund |
|---|---|---|
| Preisliste Onlineshop | <https://www.proaurum.de/preisliste/> | HTTP 200, 1,6 MB, Magento-Route `pricelist`, **vollständig serverseitig gerendert** |
| Preisradar | <https://www.proaurum.de/leistungen/online-tools/preisradar/> | **Registrierung und Login erforderlich**, nicht abgerufen |
| Altgoldservice | <https://www.proaurum.de/leistungen/altgoldservice-und-goldankauf/> | erläutert den Rechenweg, keine Preistabelle |
| Chartanalyse / Goldreport | <https://www.proaurum.de/aktuelles/chartanalyse/>, `/aktuelles/goldreport/` | Newsletter-Abo-Seiten mit Analystenkommentar (2018), keine Preistabelle |
| Marktberichte | `/aktuelles/marktberichte/` | **HTTP 404** |
| Sitemap | <https://www.proaurum.de/sitemap.xml>, 778 URLs | Preis-Treffer: `/leistungen/online-tools/preisradar/`, `/aktuelles/chartanalyse/` |

### (b) Ohne Anmeldung maschinenlesbar?

Die **Preisliste ja**, ohne jedes Login. Sie enthält:

- `div.cs-price-list__timestamp`: **„Zuletzt aktualisiert am 27.10.2026 um 18:15:42"**.
- Tabelle `cs-price-list__print-table`, Kopf **„Basiskurse ab 2026-10-27 17:15:21"**,
  Zeilen `Euro in USD | 1,1139 $`, `Gold | 4.059,10 $ | 4.059,50 $`,
  `Silber | 54,41 $ | 54,43 $`, `Palladium | 1.034,20 $ | 1.034,60 $`,
  `Platin | 1.535,50 $ | 1.537,00 $` (Spalten `Verkaufen` und `Kaufen`).
- Produkttabelle mit Spalten `Bezeichnung | Gewicht | Verkaufen | Kaufen`, Werte je Zeile
  im Attribut `data-price-amount="114.00"` etc. Beispiel: „Goldbarren 1 Gramm Umicore",
  Gewicht `1,00 g`, Verkaufen `114,00 €`, Kaufen `133,50 €`.

Der **Preisradar** ist der eigentliche „Preisüberwachungs"-Dienst, verlangt aber
ausdrücklich „Registrieren Sie sich mit Ihrer E-Mail Adresse und Ihrem Passwort", also
**Login-Bereich, nicht abgerufen**.

### (c) Format

- HTML-Tabelle, Werte zusätzlich als `data-price-amount` maschinenlesbar.
- Einheit: Die Produkttabelle nennt **EUR pro Produkt**; das Gewicht steht in der
  Nachbarspalte (`1,00 g`). Die Basiskurse stehen in **USD je Feinunze**. Die Umrechnung
  in EUR/Gramm ist eine Ableitung, keine Seitenangabe.
- **Keine Standortdimension**: eine zentrale Shop-Preisliste für alle Filialen.
- Inhalt ist **Edelmetall-Handel** (Barren, Münzen), nicht Schrottsorten. Die Altgoldseite
  nennt als Rechenbeispiel 1.300 USD je Feinunze zu 37,97 EUR je Gramm und warnt vor
  10 bis 15 Prozent Abschlag für die Scheidung, aber **nennt keinen eigenen Ankaufpreis**.

### (d) Was realistisch folgt

**Kein Schrott-Handler.** Die veröffentlichten Preise sind für Barren und Münzen, die
Materialbezeichnungen passen nicht in den Schrott-Katalog. Der einzige Ansatzpunkt ist eine
**Marktinformations-Pipeline für Edelmetalle**: die Basiskurse mit Zeitstempel sind
maschinenlesbar, aber in USD/Feinunze und müssen für einen EUR/g-Vergleich umgerechnet
werden. Das ist eine andere Quelle als ein Händler-Sortenpreis und gehöre in einen
eigenen Datenbereich, nicht in `haendler_angabe`.

### (e) Was zu klären wäre

1. Kaufen die Filialen (Bad Homburg, Bielefeld, München …) Altgold zu den Konditionen der
   Shop-Preisliste oder zu eigenen Filialkonditionen?
2. Gibt es eine öffentliche (nicht angemeldete) Preisquelle für **Altgold-Sorten**
   (Bruchgold, Goldschrott, Zahngold)? Die Altgoldseite verweist auf die Basiskurse,
   aber ohne Tabelle.
3. Ist die Basiskurs-Zeitreihe (Verlauf) öffentlich abrufbar? Das wäre der eigentliche
   Wert für eine Markt-Pipeline.
4. Wie ist die Währungsumschaltung (EUR/USD) zu behandeln, damit ein Kurs nicht mit dem
   falschen EURUSD-Stand in die Datenbank wandert.

---

## 3. schrott24.de (Metaloop / Schrott24, 10 Dossiers) - **Plattformpreis, kein Händlerpreis**

Dossiers: `hb/hb-online-bremen-schrott24-metaloop-online-ankauf-abholun`,
`sn/sn-online-schrott24-metaloop-liz-altmetallhandler`,
`nw/nw-nrw-weit-schrott24-metaloop-online-portal`, `nw/nw-dusseldorf-hartmetallschrott24`,
`sh/sh-online-kropp-schrott24-metaloop-online-portal-partner`,
`be/be-online-schrott24-online-ankauf-partnernetz`,
`sl/sl-online-homburg-schrott24-metaloop-online-portal-partner`,
`st/st-halle-schrott24-metaloop-online-ankauf-partner`,
`he/he-hessenweit-schrott24-metaloop-online-ankauf-partner`,
`ni/ni-uberregional-schrott24-online-ankauf-90-anlieferstell`.

### (a) Welche Preisquellen es gibt und wo

| Quelle | URL | Befund |
|---|---|---|
| Produktdetailseiten | `/altmetall-ankauf/<kategorie>/<produkt>/` | tragen `Ankaufpreise bis zu*, €/kg: … ab N Kg` |
| **JSON-Endpunkt** | `https://www.schrott24.de/page-data/altmetall-ankauf/kupfer/kupfer-blank-i-millberry/page-data.json` | **HTTP 200, `application/json`, 859 KB**, ohne Anmeldung |
| Schrottpreise (SEO) | <https://www.schrott24.de/schrottpreise/> | erklärende Seite, **keine** Tabelle, Datum „Tagesaktuelle Schrottpreise vom 14.11.2025" |
| Blog-Archiv | `/preise/` | „Preis-Updates", LME-Kommentar, **kein Preisblatt** |
| Marktdaten | <https://www.schrott24.de/marktdaten/> | LME-Charts, Quelle lme.com → **fremder Marktplatz** |
| Sitemap | <https://www.schrott24.de/wp-sitemap-posts-page-1.xml> | **225** URLs unter `/altmetall-ankauf/`, davon **170 auf Produkt-Ebene** (53 Kategorie-Seiten, 2 Wurzel) |

Das Auftrags-Screening notierte zu `/schrottpreise/`: „Tagesaktuelle Schrottpreise vom
14.11.2025". Der Abruf am 27.10.2026 bestätigt dieses Datum **unverändert**. Die Seite ist
ein Erklärstück mit Kategorie-/Mengen-Wahl, ohne Kurstabelle.

### (b) Ohne Anmeldung maschinenlesbar?

Ja, auf zwei Wegen. Das JSON ist der interessantere:

```json
"result": { "pageContext": { "product": {
    "slug": "kupfer-blank-i-millberry",
    "name": "Kupfer, blank I, Millberry",
    "price": [ {"start": 3, "price": 8.2}, {"start": 100, "price": 8.49},
               {"start": 500, "price": 8.79}, {"start": 10000, "price": 9.04} ],
    "price_low": [], "parent_categories": [ {"name": "Kupfer", "slug": "kupfer"} ] } } }
```

Weitere Stichproben aus dem gleichen Muster:

| Produkt | `price` (kg-Stufen) |
|---|---|
| `Eisen und Stahl, Mischschrott` | 100 kg 0,16 / 1.000 kg 0,20 / 10.000 kg 0,22 / 20.000 kg 0,24 |
| `Aluminium, gemischt` | 5 kg 1,25 / 100 kg 1,30 / 500 kg 1,40 / 10.000 kg 1,79 |
| `Messing-Patronenhülsen` | 3 kg 5,29 / 100 kg 5,49 / 500 kg 5,69 / 10.000 kg 6,14 |
| `Zink, alt` | 5 kg 1,50 / 100 kg 1,60 / 500 kg 1,77 / 10.000 kg 2,94 |

Die HTML-Produktseite zeigt dieselben Werte: „Ankaufpreise bis zu*, €/kg: 8.20 ab 3 Kg /
8.49 ab 100 Kg / 8.79 ab 500 Kg". Die Werte sind am 26.10.2026 und am 27.10.2026
identisch, was für eine tagesaktuelle Plattform bemerkenswert ist.

### (c) Format

- Einheit: **EUR/kg**, gestaffelt nach Mengen (kg-Schwellen).
- **Kein Datum im JSON und kein Datum auf der Produktseite.** Das einzige Datum der Domain
  steht auf `/schrottpreise/` (14.11.2025) und ist offenbar nicht an die Produktpreise
  gekoppelt. Für `published_at` gäbe es hier faktisch nur `observed_at`.
- Semantik: **„Ankaufpreise bis zu"** = Obergrenze. Die Seite sagt ausdrücklich:
  „Preise sind abhängig von Ihrem Standort und der gewählten Transportart. Angezeigt werden
  **Maximalpreise** aus Deutschland und Österreich. Bei größeren Mengen können ggf. auch
  höhere Preise erzielt werden."

### (d) Was realistisch folgt

**Kein Händler-`haendler_angabe`-Preis.** Das ist der zentrale Punkt dieses Betreibers.
Die Plattform selbst grenzt ab, <https://www.schrott24.de/standorte/>:

> „Wenn Sie ohne Online-Verkauf das Material anliefern, erhalten Sie nicht den fixierten
> Preis von der Website sondern den **lokalen Preis des Schrotthändlers**."

Der Plattformwert ist also ein **Angebotsrahmen der Plattform**, kein Kurs eines
Schrott24-Partnerbetriebs vor Ort. Die zehn Dossiers sind Metaloop-Partner- bzw.
Online-Partner-Einträge mit ortsabhängiger Partneradresse, die erst nach dem Onlineverkauf
mitgeteilt wird. Ein Handler auf diesen Plattformpreis würde in den Dossiers Preise
ausweisen, die der Betreiber vor Ort nicht bezahlt. Das ist genau der Fall, den die
Fremdmarktplatz-Regel verhindern soll.

Der JSON-Endpunkt ist trotzdem ein **technischer Fund** und hat einen echten Nutzen:
als **Plattform-/Marktkontext** (max. Angebotspreis je Sorte, Mengenstaffel) und als
technische Referenz, wie eine JSON-Preisquelle überhaupt aussehen muss. Er ist **kein**
Ersatz für einen Händlerpreis.

### (e) Was zu klären wäre

1. Gibt es für einen konkreten Partnerhof eine öffentliche Standort-Preisliste? Bisher nein.
2. Wie ist der Abzug zwischen Plattform-Maximalpreis und lokalem Händlerpreis im
   Vertrag geregelt? (Das wäre die einzige zulässige Brücke zu einem Händlerpreis.)
3. Gibt es ein Partnerportal mit Kursen für eingeloggte Partner? (Nicht abgerufen.)
4. Warum trägt `/schrottpreise/` seit dem 14.11.2025 dasselbe Datum, während die
   Produktpreise sich ändern können? Wird das Datum gepflegt?

---

## 4. steil.de (Theo Steil GmbH & Co. KG, 10 Dossiers) - **keine Preisquelle**

### (a)-(c) Preisquellen, Anmeldung, Format

- Sitemap ist ein TYPO3-Generator:
  `https://www.steil.de/?sitemap=pages&type=1533906435&cHash=…` → **143 URLs, alle geprüft**.
  Keine Preis-URL.
- `robots.txt` (HTTP 200) sperrt nichts relevant (`Disallow:` nur für Standardpfade).
- Gelesen und ohne Zahlenbefund: `/handel/schrotthandel`, `/handel/metallhandel`,
  `/handel/int-export` sowie die vier Unterseiten
  `/handel/schrotthandel/ankauf-das-nehmen-wir-ihnen-ab`,
  `/handel/schrotthandel/verkauf-das-bieten-wir-ihnen-an`,
  `/handel/metallhandel/ankauf-das-nehmen-wir-ihnen-ab`,
  `/handel/metallhandel/verkauf-das-bieten-wir-ihnen-an`.
  Die Unterseiten tragen nur Überschriften („Ankauf - Das nehmen wir Ihnen ab"), keine Werte.
- **„Studie Schrottbonus"** (`/bdsv-video`, verlinkt in der Navigation): die
  BDSV-Studie zum Klimabeitrag von Schrott (CO2-Bilanz, Einschmelzmengen). Enthält
  **Mio.-Tonnen- und Prozentangaben zum Rohstoffeinsatz**, aber **keine Preise**.
- Maschinenlesbare EFB-Zertifikate existieren
  (`/gruppe/zertifikate-agb-1/maschinenlesbare-efb-zertifikate`). Das zeigt technische
  Bereitschaft, ist aber keine Preisquelle.
- Alle geprüften Pfade (`/preise/`, `/metallpreise/`, `/tagespreise/`, `/ankauf/preise/`,
  `/preisliste/`, `/preise/metall/`, `/schrottpreise/`, `/aktuelle-preise/`, `/de/preise/`,
  `/tageskurs/`, `/goldpreis/`, `/ankaufspreise/`, `/sortenpreise/`, `/preistabelle/`,
  `/marktplatz/`, `/portal/`, `/b2b/`, `/login/`, `/api/preise`, `/api/prices`) liefern
  keine Preisdaten.

### (d) Was realistisch folgt

**Nichts.** Theo Steil ist Großhandel: die Kunden sind Stahlwerke, Gießereien und
Schmelzwerke, nicht Privatanlieferer. Eine öffentliche Sortenpreisliste ist hier weder
erwartbar noch nötig. Die Preise entstehen bilateral im Handel.

Nebenbefund mit Nutzen für andere Betreiber dieser Gruppe: Steil führt
**Rhein-Main-Rohstoffe (rmr-recycling.de)** und **MHF Metallhandel Frankfurt** als
verbundene Unternehmen. Ein gemeinsamer „Gruppen-Handler" über diese Domains wäre die
natürliche Erweiterung, sobald eine dieser Domains eine Preisquelle bekommt.

### (e) Was zu klären wäre

1. Gibt es ein **Kundenportal / Preisinformationssystem**, das Stammlieferanten sehen?
   Das ist die realistischste Form einer nutzbaren Preisquelle bei einem Großhändler.
2. Gibt es eine Sortenliste mit **Preisindizes** (z. B. „Preis = LME-Faktor x Index")?
   Die Agentur-Gruppe WMR beschreibt genau eine solche Formel (Abschnitt WMR).

---

## 5. isr-recycling.de (ISR Recycling + LSH, Warnsholz, Peter Stolz, 11 Dossiers) - **Preisquelle existiert, ist aber kundengebunden**

### (a)-(c)

- Sitemap <https://www.isr-recycling.de/sitemap.xml>: **96 URLs**, vollständig geprüft.
  Preis-Treffer sind ausschließlich **PDFs**: `agb-isr.pdf`, `agb-lsh.pdf`,
  `agb-warnsholz.pdf`, `agb-peterstolz.pdf`, `Allgemeine-An--und-Verkaufsbedingungen-ISR.pdf`,
  `Stammdaten-Erklaerung-Gewerblich/Privat.pdf`, `business-account-registration-ISR.pdf`
  sowie 30 Zertifikats-PDFs.
- `/index.php/downloads-isr` (und -lsh, -warnsholz, -peter-stolz): Abschnitt **„AGB - Hier
  finden Sie unsere aktuell gültigen An- und Verkaufsbedingungen"** plus Zertifikate.
  **Keine Preisliste.**
- `/index.php/leistungen-isr`: „Wir kaufen Ihren Schrott zum aktuellen **Tageshöchstpreis**"
  (keine Zahlen). Und die entscheidende Passage:

  > „Kundenkarte - Nutzen Sie unsere Kundenkarte bei der Anlieferung von Schrott und Metallen.
  > Für Stammlieferanten stellen wir eine **personalisierte Karte** aus, mit der Sie an
  > allen Standorten **Vorzugspreise** erhalten."

- Standortseiten (`/index.php/standorte-isr`, 11 Standorte): „Für eine **Preisauskunft**
  oder die Abholung von Mengen setzen Sie sich gerne …", „Für **Preisanfragen** oder die
  Gestellung von Containern sprechen Sie uns …"
- Alle Preispfade (zweite Runde) ohne Zahlenbefund.

### (d) Was realistisch folgt

**Nichts maschinenlesbar.** Die Preisquelle existiert, aber sie ist an ein
**Kundenkonto** gebunden: Kundenkarte mit Vorzugspreisen, An-/Verkaufsbedingungen,
Stammdatenerklärung. Das ist eine **B2B-Preissource als Geschäftsbeziehung**, nicht eine
Webquelle. Ein Handler wäre hier nicht nur unzulässig, sondern auch sinnlos: die
Vorzugspreise unterscheiden sich je Stammlieferant.

### (e) Was zu klären wäre

1. Gibt es ein Kundenportal mit Kursliste für Stammlieferanten, und ist es für
   Stammkunden ohne Login erreichbar?
2. Gibt es eine **Standard- oder Höchstpreistabelle** für Gelegenheitsanlieferer, die
   nicht in der Kundenkartenlogik steckt? Das wäre der einzige Weg zu öffentlichen Preisen.

---

## 6. ziems-recycling.de (6 Dossiers) - **keine Preisquelle**

### (a)-(c)

- Sitemap <https://www.ziems-recycling.de/sitemap.xml>: **12 URLs**
  (`/`, `/standorte.html`, `/malchow.html`, `/malchin.html`, `/gnoien.html`,
  `/stralsund.html`, `/waren-mueritz.html`, `/unternehmen.html`, `/abbruch.html`,
  `/zertifizierungen.html`, `/kontakt.html`, `/datenschutz.html`).
- `robots.txt` **HTTP 404** (WordPress-Fehlerseite).
- Alle geprüften Preispfade liefern die WordPress-Fehlerseite
  („REQUEST URL: www.ziems-recycling.de/tagespreise/"), also **kein Soft-404-Falschtreffer**.
- Startseite, `/unternehmen.html`, `/standorte.html`, `/malchow.html`: kein einziger
  Eurobetrag, kein €/kg, kein Kurs.

### (d) Was realistisch folgt

**Nichts.** Eine Domain, fünf Standorte, kein Filialfinder mit Preistabelle, keine
Preisquelle. Die sechs Dossiers teilen sich eine Domain; ein Sammel-Handler ohne
Preisquelle hätte keinen Gegenstand.

### (e) Was zu klären wäre

Ob die fünf Standorte (Malchow, Malchin, Güstrow/Neuenkirchen, Stralsund, Waren/Müritz)
unterschiedliche Tarife haben. Falls ja, wäre eine Preisliste je Hof die Voraussetzung
für einen Sammel-Handler.

---

## 7. jakob-becker.de (7 Dossiers) - **keine Preisquelle**

### (a)-(c)

- Yoast-Sitemap-Index <https://jakob-becker.de/sitemap_index.xml> mit 5 Teilsitemaps
  (`post-`, `page-`, `standort-`, `ceo-`, `download-sitemap.xml`), zusammen geprüft.
- `/downloads/` („Alle wichtigen Informationen an einem Ort") enthält **82 verschiedene
  PDF-Links**, ausschließlich: AGB (Stand März 2026), Merkblatt Containerdienstleistungen,
  Zertifikate (EFB, ISO 9001/14000/50001, ElektroG, GewAbfV, WHG, RAL-Gütezeichen,
  SURE, BUC AMS), Erklärungen (Mindestlohn, Grundsatzerklärung, Verhaltenskodex für
  Zulieferer), Organsigramm sowie Kommunalblätter (Abfuhrpläne von Kreisen und Städten).
  **Keine Preisliste, kein Preis-PDF.**
- `/download/` ist ein WP-Archiv mit 24 eigenen Unterseiten, ebenfalls ohne Preise.
- Alle Preispfade ohne Zahlenbefund.

### (d) Was realistisch folgt

**Nichts.** Der Downloadcenter ist umfangreich und gut gepflegt, aber er enthält
Zertifikate und Bedingungen, keine Kurse.

### (e) Was zu klären wäre

Ob die Becker-Maurer-Gruppe (BeckerArmbrust, Rhenus Becker, Semper Becker, LCM) eine
zentrale Preislogik für die Standorte führt, die öffentlich abrufbar wäre. Der
Organigramm-Download deutet auf eine Holding-Struktur hin, aber ohne Kursseite.

---

## 8. kunze-group.de (Walter Kunze Schrott- und Metallhandels GmbH, 5 Dossiers) - **keine Preisquelle**

### (a)-(c)

- **Kein robots-Sitemap**, `robots.txt` (Joomla) sperrt nur Standardpfade.
- `/index.php/leistungen/schrott-und-metallhandel`: „Schrott- und Metallannahme **zu
  aktuellen Tageshöchstpreisen**" (keine Zahlen).
- Die vier Produkt-Modalartikel (`index.php?option=com_content&view=article&id=19` bis
  `22`: Abfälle, Gießereischrott, Metalle legiert, Stahlschrotte) wurden einzeln
  abgerufen und gelesen: reine **Sortenbeschreibungen** nach EN 1010 (Mischschrott,
  Stahlschrott E 2, E 8, Shreddervormaterial, Stärke-/Abmessungsgrenzen). **Keine Preise.**
- Fünf Standorte: Langenau, Herbrechtingen, Freital, Großenhain, Dresden.
- Alle Preispfade ohne Zahlenbefund.

### (d) Was realistisch folgt

**Nichts.** Wie Steil: reines B2H-Geschäft mit hoher Sortenkompetenz, aber ohne
öffentliche Preisliste.

### (e) Was zu klären wäre

Ob es eine Kundensparte (Wiederverkäufer) mit Preisliste gibt. „Tageshöchstpreise" ist die
einzige Angabe; ob ein Index-System dahintersteht, wäre zu erfragen.

---

## 9. riwald.de (Riwald Recycling, 6 Dossiers) - **keine Preisquelle**

### (a)-(c)

- WordPress-Sitemap-Index mit 4 Teilsitemaps, zusammen **123 URLs** (18 Beiträge,
  105 Seiten), alle geprüft. Preis-Treffer: ausschließlich die sechs
  `downloads-*-riwald-*`-Seiten.
- `/downloads-de-riwald-electronics-recycling/` (und Rhein-Main, Franken, englische
  Varianten): **AEB, AVB, GTS, EFB, ElektroG, ISO 9001/14000/50000** als PDFs auf
  `c89f9f6bd94de186-endpoint.azureedge.net`. **Keine Preise.**
- `/sell-to-us/` (Anlieferung an Riwald) und
  `/materialien-und-leistungen-de-riwald-recycling-franken/`: kein Eurobetrag, kein Kurs.
- Alle Preispfade ohne Zahlenbefund.

### (d) Was realistisch folgt

**Nichts.** Riwald arbeitet als Elektronik-Recycling-Spezialist mit Anliefererkriterien;
Preise für Buntmetalle und Edelstahl sind nicht öffentlich.

### (e) Was zu klären wäre

Ob die Standorte Eppingen, Lustadt und Wiedergeltingen (drei EFB-Standorte) je eine
Anlieferer-Preisliste führen. Wenn ja, wäre das ein **Sammel-Handler über drei Dossiers**
mit Standortdimension, wie bei MC Schrott.

---

## 10. goetz-neu-ulm.de (Götz GmbH Schrott + Metalle, 6 Dossiers) - **kein Händlerpreis, aber Marktbericht**

### (a)-(c)

- `robots.txt` sperrt WP-Admin und Suche; Sitemap-Index mit 5 Teilsitemaps
  (63 Seiten, Posts, Produkte, Taxonomien), geprüft. Preis-Treffer:
  `/schrottsorten-fuer-stahlwerke/`, `/download/`.
- `/schrott-und-metalle/`: „Alle Anlieferungen werden bei uns transparent gewogen und **zu
  tagesaktuellen Preisen vergütet** … basierend auf den aktuellen Börsenpreisen."
  FAQ: „Bekomme ich bei Götz den besten Preis für meinen Schrott? Ja, wir vergüten alle
  angelieferten Metalle transparent und fair auf Basis der aktuellen Börsenpreise."
  **Keine Zahlen.**
- `/download/`: Container-Info, Annahmekriterien Vereinssammlungen, Altauto-Hinweise,
  AGB Einkauf/Entsorgung/Verkauf sowie **„Bedingungen für Lieferung"** für
  Gussbruch/Gießereistahlschrott (Fassung **1. August 1996**) und unlegierten Stahlschrott
  (Fassung **1. Mai 2002**). Die beiden PDFs wurden gelesen: reine
  Liefer-/Zahlungsbedingungen (Eigentumsvorbehalt, Zahlungstermin, Sistierung, Versand),
  **keine Preise**.
- **Der eigentliche Fund**: <https://www.goetz-neu-ulm.de/news/> publiziert monatlich
  **„Aktuelle Marktsituation Schrottmarkt"** als PDF, Ausgabe **#86 vom 21. Oktober 2026**
  (`https://www.goetz-neu-ulm.de/wp-content/uploads/2026/10/2026_10_21_SMarkt_86_Marktkommentar.pdf`).
  Serie **45 Monatsausgaben** von 01/2023 bis 10/2026 (nur 05/2024 fehlt); die Ausgabe
  01/2025 liegt sogar direkt bei `cdn.bdsv.org`. Inhalt ist der **BDSV-Marktkommentar**
  (Redaktion David Kämper), Götz ist Republisher.
- **WAF-Hinweis**: `goetz-neu-ulm.de` antwortet auf Teile der Pfadproben mit **HTTP 403**
  und einem WordPress-WAF-Body
  (`{"code":"wp_die","message":"You are not allowed to visit this website.","IP":…}`).
  Für einen Handler ist das ein reales Betriebsrisiko.

### (d) Was realistisch folgt

**Kein Händlerpreis, aber ein Marktinformations-Dokument.** Der Marktkommentar enthält:

- **Relative** deutsche Preisveränderungen je Region, **ohne absolutes Preisniveau**:
  „Norden: … zu unveränderten Preisen; Nachholeffekte brachten vereinzelt Aufschläge von
  5 €/t", „Westen: Die Preise wurden um bis zu 10 €/t angehoben", „Südwesten/Süden/Saar/
  Osten: … zu unveränderten Preisen", „Italien: Aufschläge von 10 €/t für Neuschrott und
  5 €/t für Altschrott", „Schweiz: 5 bis 10 €/t", „Polen: 5 €/t",
  „Gießereimarkt: Aufschläge von 5 €/t".
- **Absolute internationale Indexstände in $/t mit Datum** (Fastmarkets, **fremder
  Marktplatz**): HMS 1&2 (80:20) CFR Türkei 19.10.: **390,00 $/t**; FOB New York 14.10.:
  **344,00 $/t**; Schredderschrott FOB New York 14.10.: **364,00 $/t**; FOB Los Angeles
  14.10.: **315,50 $/t** und **325,50 $/t**; FM-HRC-Index FOB US-Midwest 19.10.:
  **1.285,00 $/st**; FM-Betonstahl Export FOB Türkei 15.10.: 615-620 $/t; FM-Stahlknüppel
  CFR Türkei 15.10.: 510-520 $/t.

Das ist eine **dokumentierte Preisquelle anderer Art**: ein monatlicher, datierter
Marktbericht eines Branchenverbands, den der Betreiber selbst mitverteilt. Er taugt als
**Marktinformations-Pipeline** (Marktniveau und -richtung für den deutschen Schrottmarkt,
in $/t und als regionale Veränderung in €/t). Er taugt **nicht** als Händler-`haendler_angabe`
für Götz: Es gibt keinen Götz-Ankaufspreis, und die Indexwerte betreffen türkische und
US-Fob-Märkte, nicht den deutschen Binnenmarkt.

### (e) Was zu klären wäre

1. Gibt es eine **eigene Götz-Preisliste für gewerbliche Anlieferer** (parallel zum
   Kundenbereich)? Die Trennung „Privat / Vereinssammlung / Gewerbe" auf der Downloadseite
   legt eine solche Staffelung nahe.
2. Wie ist der **BEHG-Aufschlag** geregelt? Die News-Seite führt einen eigenen Beitrag
   „BEHG: Kurzfristige Anpassung für die Berechnung des BEGH-Aufschlags"
   (`goetz-kundenin…pdf`) - eine **ver öffentlichte Kalkulationsregel**. Das ist der
   interessanteste Ansatzpunkt für eine Preisformel.
3. Lässt der Betreiber die Nutzung des monatlichen Marktkommentars als Datenquelle zu?

---

## 11. rmr-recycling.de (Rhein-Main-Rohstoffe GmbH, 4 Dossiers) - **keine Preisquelle**

### (a)-(c)

- Sitemap-Index mit 3 Teilsitemaps (`page-`, `ditty_news_ticker-`, `category-sitemap.xml`),
  geprüft; kein Preis-Treffer.
- Startseite: „Handel mit Sekundärrohstoffen", „Vermarktungsmöglichkeiten im In- und
  Ausland", „Labor und Analysegeräte", „rd. 1.000 Container". **Kein Eurobetrag, kein Kurs.**
- Ein einziges PDF: EFB-Zertifikat 2025.
- Alle Preispfade ohne Zahlenbefund.

### (d) Was realistisch folgt

**Nichts.** RMR ist ein Großhandel (Steil-Gruppe) und vermittelt Sekundärrohstoffe
weiter; Endkundenpreise gibt es nicht. Zusammen mit MHF Metallhandel Frankfurt und den
übrigen Steil-Standorten ist RMR der passende Adressat für die Frage nach einem
Kundenportal (siehe Steil).

### (e) Was zu klären wäre

Dasselbe wie bei Steil: B2B-Kurssystem für Stammlieferanten?

---

## 12. kuehl-entsorgung.de (Kühl Entsorgung & Recycling, 5 Dossiers) - **keine Preisquelle**

### (a)-(c)

- Sitemap-Index mit 13 Teilsitemaps, darunter `container-sitemap.xml`,
  `standort-sitemap.xml`, `faq_kategorie-sitemap.xml`, `lexikon-sitemap.xml`,
  `awsm_job_openings-sitemap.xml`; zusammen geprüft. Preis-Treffer: nur
  `/service/downloadcenter/`.
- Der Downloadcenter <https://kuehl-entsorgung.de/service/downloadcenter/> listet rund
  40 PDFs: EFB-/EBA-/QM-/ENMS-Zertifikate je Standort, AGB (11.12.2025),
  **„Besondere Geschäftsbedingungen für den Verkauf von Sekundärrohstoffen"** (10/2025),
  AGB-WP (09/2025) sowie **je Standort ein PDF** (Augsburg, Berlin, Heidenau, Karlsruhe,
  Waldshut-Tiengen, Kerpen, Lauda-Königshofen, Efringen-Kirchen, Thermbad-Wiesenbad,
  Großrückenwalde, OK-Oudegem, Ebersbach, Berlin-Hoppegarten, Diedorf) sowie
  `Annahmebedingungen.pdf` (10 Seiten, 402 KB) und `elektroschrott-flyer.pdf`.
- Die **standortbezogenen PDFs wurden gelesen**: z. B. `Augsburg.pdf` (2 Seiten) und
  `Berlin.pdf` (2 Seiten) sind **Datenschutzerklärungen** der jeweiligen Gesellschaft
  (Kühl Entsorgung & Recycling Süd GmbH Augsburg / GmbH & Co. KG Berlin). **Keine Preise.**
- `Annahmebedingungen.pdf` enthält nur Formulierungen wie „können kostenpflichtig nach
  entsprechenden Fraktionen [berechnet werden]" - **keine Zahlen**.
- `robots.txt` (Yoast) sperrt nichts; `kuehl-gruppe.de` zeigt ebenfalls keine Preise.

### (d) Was realistisch folgt

**Nichts.** Der Downloadcenter ist gut ausgebaut, aber Zertifikate, AGB und
Annahmebedingungen. Die Standort-PDFs sind ein interessanter Dateinamen-Bestand, aber kein
Preisbestand - sie zu zitieren, als seien sie Preisblätter, wäre falsch.

### (e) Was zu klären wäre

Ob die „Besonderen Geschäftsbedingungen für den Verkauf von Sekundärrohstoffen" eine
**Preisformel** enthalten (Index minus Abschlag). Dieses PDF ist als Vertragsbestandteil
das einzige Dokument der Gruppe, das überhaupt eine Preislogik verspricht.

---

## 13. fegert-recycling.de (Fegert Recycling GmbH, 4 Dossiers) - **keine Preisquelle**

### (a)-(c)

- **Joomla-Soft-404**: jede geprüfte Preis-URL liefert HTTP 200 mit **exakt 20.999
  Bytes** und dem Inhalt der Startseite („Fegert Recycling GmbH - Das Unternehmen",
  `h1: Über uns`). Geprüft wurden **38** Pfade in zwei Runden:
  `/preise/`, `/metallpreise/`, `/tagespreise/`, `/aktuelle-preise/`, `/ankauf/preise/`,
  `/preise/metall/`, `/schrottpreise/`, `/de/preise/`, `/preisliste/`, `/preislisten/`,
  `/ankauf/`, `/downloads/`, `/download/`, `/aktuelles/`, `/sorten/`, `/ankaufspreise/`,
  `/tarife/`, `/gebuehren/`, `/services/`, `/materialpreise/`, `/tageskurs/`, `/goldpreis/`,
  `/silberpreis/`, `/sortenpreise/`, `/preistabelle/`, `/api/preise`, `/api/prices`,
  `/marktplatz/`, `/portal/`, `/b2b/`, `/login/`, `/aktuelle-preise-anmeldung/`,
  `/abonnieren/`, `/newsletter/`, `/preise-und-kurse/`, `/notierungen/`.
  Das ist kein Treffer, sondern die Joomla-Fallbackseite.
- `/sitemap.xml`: **12 URLs** (`/`, `/leistungen.html`, fünf `/standorte/standort-*.html`,
  `/kontakt.html`, `/zertifizierung.html`, `/impressum.html`, `/datenschutz.html`,
  `/agb.html`). Keine Preis-URL.
- Startseite und `/leistungen.html`: Sorten und Leistungen ohne Zahlen.

### (d) Was realistisch folgt

**Nichts.** Auf dieser Domain existiert kein Preispfad. Wichtig für das Screening-Verfahren:
Ein reiner HTTP-200-Check erzeugt hier 18 Falschtreffer. Jede künftige Screening-Liste
muss den Seitentitel bzw. den `h1` gegen den erwarteten Pfad prüfen.

### (e) Was zu klären wäre

Ob Fegert überhaupt Preise vergibt (Containerdienst, Annahmegebühren) und ob die fünf
Standorte (Großpetersstraße, Glindenberger Weg, Gatersleben, Schönebeck/Elbe, Staßfurt)
unterschiedliche Tarife haben.

---

## 14. foell-rohstoffhandel.de (Föll Rohstoffhandel, 4 Dossiers) - **keine Preisquelle**

### (a)-(c)

- TYPO3, `robots.txt` sperrt nur Systempfade; Sitemap `sitemap.xml` plus
  `sitemapimage.xml`, geprüft.
- `/was-machen-wir/schrott-metall`: Sortenübersicht (FE-Schrotte, NE-Metalle, legierte
  Schrotte) mit Beispielen, **keine Preise**. Der Preis kommt nur als Kontaktthema vor:
  „(Erster Ansprechpartner für Kundenkonto, E-Mail-Adresse, **Preise**)".
- `/service/downloads`: Formulare (Stammdatenblatt Privat/Gewerblich, Gesetze,
  Erklärung) und ein Link auf `gesetze-im-internet.de/gewabfv_2017/GewAbfV.pdf`.
  **Kein Preis-PDF.**
- `annahmekriterien-schrott-und-metalle.pdf` (AVV 17 04 05 / AVV 17 04 07, Stand
  01.08.2025) wurde gelesen: reine **Annahmekriterien** (Ausschlussliste, Altfahrzeugregel
  mit max. 4 Reifen, „Jeder weitere Reifen wird extra berechnet", Elektronikschrott
  kostenlos, Aussortierung auf Kosten des Anlieferers). **Keine Preise.**

### (d) Was realistisch folgt

**Nichts.** Föll ist ein reiner Vermittler: „Gemeinsam ermitteln wir, was für eine
**Angebotserstellung** benötigt wird". Preise entstehen je Ausschreibung.

### (e) Was zu klären wäre

Ob es einen Onlineshop mit Preisanzeige für Container gibt (die Startseite verweist auf
„bestellt einfach online" Container). Containerpreise wären Gebühren, keine Sortenpreise,
wären aber für den Marktplatz relevant.

---

## 15. schrott-mohr.de (Mohr Rohstoff GmbH, Wiesbaden, 3 Dossiers) - **keine Preisquelle, aber `llms.txt`**

### (a)-(c)

- Sitemap-Index mit 6 Teilsitemaps (`post-`, `page-`, `glossar-`, `abholgebiete-`,
  `schrottarten-`, `local-sitemap.xml`), geprüft. Preis-Treffer: nur die Glossar-URLs und
  `/schrottankauf`.
- `/schrottankauf`: Titel „Die besten Schrottpreise im Schrottankauf", Text: „… unter
  börsenähnlichen Bedingungen werden **Tagespreise ausgehandelt**. Wir sind in der Lage,
  diese aktuellen Preise jederzeit einzusehen", und „Unser System berechnet Ihren
  Schrottwert nach dem **tagesaktuellen Kurs** und Sie erhalten Ihr Geld direkt Bar an der
  Kasse." Darunter eine **Sortenliste mit rund 60 Materialien** (Altschrott, Alu-Fe 2/50 %,
  Alu-Felgen, Blei, Bremsscheiben, E-Motoren, Edelstahl, Hartmetall, Kabel 38/50/70 %,
  Kupfer Berry/Millberry/Raff, Messing 58, Mischschrott, V2A, V4A, Zink, Zinn …) -
  **vollständig ohne Preise**.
- **`<https://www.schrott-mohr.de/llms.txt>`** (28 KB) ist vorhanden: eine
  maschinenlesbare Beschreibung von Unternehmen, Leistungen, Sorten und Abläufen. Sie
  enthält **keine Preise**, aber die Aussage: „Mohr zahlt tagesaktuelle Preise, die sich an
  den internationalen Rohstoffbörsen orientieren."
- Blogbeitrag „Schrottpreise - wovon hängen diese ab?" (31.07.2020): erklärt die
  Einflussgrößen (Rohstoffbörse, Menge, Reinheit, Qualität, Anhaftungen). **Keine Zahlen.**
- Ein einziger Zahlenwert im ganzen Bestand: die Autoverwertung zahlt „bis zu **100 Euro**
  für Ihr komplettes Altfahrzeug". Das ist eine Vergütung für ein Fahrzeug, **kein
  Sortenpreis**, und keine Preisliste.
- Alle Preispfade (inkl. `/tageskurs/`, `/goldpreis/`, `/ankaufspreise/`, `/sortenpreise/`,
  `/preistabelle/`, `/api/preise`, `/api/prices`, `/marktplatz/`, `/portal/`, `/b2b/`)
  ohne Zahlenbefund.

### (d) Was realistisch folgt

**Nichts.** Der Sortenbestand ist gut strukturiert und wäre nach idealer Sortenliste, aber
ohne Preise wertlos für einen Preis-Handler. Der WERT dieser Domain liegt anderswo: Die
`llms.txt` zeigt, dass der Betreiber maschinenlesbare Selbstdarstellung betreibt. Das ist
ein **Ansprachekanal** für die Frage nach einer Preisquelle, kein Preisbeleg.

### (e) Was zu klären wäre

1. Gibt es eine Preisliste, die im Haus-System existiert und nur nicht online steht? Der
   Text „Unser System berechnet Ihren Schrottwert nach dem tagesaktuellen Kurs" deutet
   darauf hin, dass es eine interne Kursliste gibt.
2. Wird die Sortenliste aus `/schrottankauf` gepflegt? Sie ist die bislang beste
   Materialbasis im Bestand und wäre der natürliche Anker für eine spätere Preisliste.
3. Gibt es eine Preisauskunft per WhatsApp/Mail, die strukturiert abrufbar wäre?

---

## 16. eberhard-mayr.de (Eberhard Mayr + Konzern-Domains, 4 Dossiers) - **keine Preisquelle**

### (a)-(c)

- `robots.txt` (Yoast) nennt **sechs** Sitemaps und damit die Konzernstruktur:
  `eberhard-mayr.de`, `hettinger-schrott.de`, `muellex-entsorgung.de`,
  `eberhard-mayr.de/kaempf-umweltservice/`, `eberhard-mayr.de/kaempf-recycling/`,
  `hmh-metallhandel.de`.
- Alle fünf Domains wurden abgerufen (Startseiten, `page-sitemap.xml`): **kein einziger
  Eurobetrag, kein €/kg, kein Kurs, kein Preiswort mit Zahl.**
- `eberhard-mayr.de` nennt auf der Startseite: „… 350.000 Tonnen CO2 einsparen. Wir
  berechnen auf Wunsch individuell …" - eine Individualangebots-Zusage, kein Preisblatt.
- Alle Preispfade auf allen fünf Domains ohne Zahlenbefund.

### (d) Was realistisch folgt

**Nichts.** Sechs Domains, ein Konzern, keine Preisquelle. Bemerkenswert ist nur die
Konzernbreite: Die Domains sind ein Multi-Marken-Auftritt derselben Holding.

### (e) Was zu klären wäre

Ob eine der Konzernmarken (Hettinger Schrott, MüllEx, Kämpf Recycling, Kämpf
Umweltservice, HMH Metallhandel) eine Preisliste führt. Eine Sammel-Recherche über alle
sechs Domains wäre hier die richtige Form, nicht eine Recherche über eine Domain.

---

## 17. wmr-online.de (Wesermarsch Metallrecycling GmbH, 3 Dossiers) - **Preisformel benannt, Werte fehlen**

### (a)-(c)

- `robots.txt` (WordPress) sperrt nur `wp-admin`; Sitemap
  `wp-sitemap-posts-page-1.xml` geprüft, kein Preis-Treffer.
- `/gewerbekunden/metallrecycling/` enthält die entscheidende Formel:

  > „Die einzelnen Fraktionen werden dabei nach **Tagespreis** oder auf Wunsch mit einem
  > **festen Abschlag zu bestimmten Indizes** vergütet. Durch eine genaue Analyse der
  > Materialien mit Hilfe eines Spektralanalysegerätes garantieren wir Ihnen bestmögliche
  > …"

- Das „Abfall-ABC" (<https://www.wmr-online.de/privatkunden/abfall-abc/>) listet
  Materialien für Privatkunden - **ohne Preise**.
- `/preise/metall/` leitet serverseitig auf `/gewerbekunden/metallrecycling/` um.
- Alle Preispfade ohne Zahlenbefund.

### (d) Was realistisch folgt

**Nichts direkt, aber die Formel ist der beste Ansatzpunkt der ganzen Gruppe.** WMR
beschreibt eine **Indexrechnung**: Fraktionspreis = Index minus fester Abschlag. Das ist
eine Preislogik, die sich technisch umsetzen ließe, **wenn** Index und Abschlag bekannt
wären. Beides ist es nicht: der Index (vermutlich LME) und der Abschlag sind nicht
veröffentlicht.

Der praktische Weg ist deshalb: **Index aus einer offenen Quelle (LME) + Abschlag beim
Betreiber erfragen**. Der Betreiber hat den Abschlag vermutlich als interne Konstante.

### (e) Was zu klären wäre

1. Welcher Index genau (LME-Kupfer? ein House-Index? Monatsmittel?) und welcher
   Abschlagssatz je Fraktion.
2. Ob es eine schriftliche Preisliste für Gewerbekunden gibt.
3. Ob die Spektralanalyse-Ergebnisse (Schrottsortierung) mit einem Preisblatt
   verknüpft werden können.

---

# Fremde Marktplätze und Marktinformation (ausdrücklich **kein** `haendler_angabe`)

Die folgenden Belege enthalten Preisangaben, die **nicht vom Betreiber selbst stammen**.
Sie dürfen nicht als Händlerangabe verbucht werden und taugen nur als
Marktinformations- oder Kontextquelle.

| Fund | Operator | Art | Warum kein Händlerpreis |
|---|---|---|---|
| LME-Charts auf `/marktdaten/` mit Quellenlink auf lme.com | schrott24.de | Fremde Börse (LME), EUR/t, Tagesnotierung | Weltmarkt-Index, nicht Angebot des Betreibers |
| BDSV-Marktkommentar #86 (21.10.2026) mit Fastmarkets-Indizes 390,00 $/t (HMS 1&2 CFR Türkei), 344,00 $/t (HMS FOB New York), 364,00 $/t (Schredder FOB NY), 315,50/325,50 $/t (FOB LA), 1.285,00 $/st (HRC FOB US-Midwest), 615-620 $/t (Betonstahl FOB Türkei), 510-520 $/t (Knüppel CFR Türkei) | goetz-neu-ulm.de (BDSV-Republisher) | Fremder Marktplatz (Fastmarkets), Exportmärkte, datiert | Indexwerte für türkische/US-Märkte; keine deutschen Preisniveaus, keine Götz-Kurse |
| Regionale Veränderungen „+5 €/t", „+10 €/t", „unveränderte Preise" im selben Marktkommentar | goetz-neu-ulm.de | **Relative** Marktbewegung, deutsches Inland | Kein absoluter Kurs, nur Bewegungsrichtung |
| LME-Kommentar-Blog `/preise/` | schrott24.de | Marktblog | Kommentar, keine Liste |
| Gold-/Silberbasiskurse 4.059,10 $ / 54,41 $ je Feinunze | proaurum.de | Markt- bzw. Weiterverkaufskurs des Händlers | Zwar Händler-Angabe, aber **Durchreichungskurs eines Weltmarkts**, und in USD je Feinunze, nicht ein Sortenpreis |
| „Studie Schrottbonus" (BDSV) | steil.de | Klimastudie | Mio.-t- und Prozentangaben zu Rohstoffeinsatz, keine Kurse |
| Pressemitteilungen zu Kursbewegungen (u. a. „Nickel wieder fester, Kupfer unter 6.000 USD") | schrott24.de | Marktbericht | Journalistische Marktbeobachtung |

**Metallprice und ScrapMonster** wurden in dieser Recherche als Kandidaten für
Fremdpreise geprüft, aber **kein einziger dieser beiden Marktplätze** erscheint in den
geprüften Betreiber-Seiten dieser Gruppe. Die einzige echte Fremdpreis-Bezugnahme im
gesamten Bestand ist die **LME** (schrott24.de) und die **Fastmarkets**-Indizes im
BDSV-Marktkommentar (goetz-neu-ulm.de).

---

# Konsequenzen

1. **Ein Handler-Kandidat: `mcschrott.de`.** Die Quelle erfüllt alle vier Bedingungen
   aus [`docs/adding-a-trader-handler.md`](../docs/adding-a-trader-handler.md):
   erreichbare URL, Materialbezeichnung, Preis, Einheit und Datum maschinenlesbar.
   Vor dem Bau sind drei Punkte zu klären: Aktualisierungsintervall, die
   Standortdimension (Rostock-Override in `data-price-ros`, `data-only` bei Papier) und
   die Frage `exact` vs. `approx` (Betreiber nennt es ausdrücklich „unverbindliche
   Richtpreise").
2. **Ein technischer Referenzfall: schrott24.de.** Das `page-data.json` zeigt, wie eine
   JSON-Preisquelle aussehen muss. Inhaltlich bleibt es ein Plattform-Maximalpreis ohne
   Datum und ohne Standortbezug; die Plattform selbst weist darauf hin, dass vor Ort der
   lokale Händlerpreis gilt. **Kein `haendler_angabe`-Preis für die zehn
   Schrott24-Dossiers.**
3. **Ein Marktinformations-Kandidat: goetz-neu-ulm.de** (monatlicher BDSV-Marktkommentar
   mit datierten Fastmarkets-Indizes) und **proaurum.de** (Zeitstempel-Basiskurse für
   Edelmetalle). Beides sind andere Quellenarten und gehören in einen eigenen
   Marktinformations-Datenbereich, nicht in `haendler_angabe`.
4. **Kein Handler für die übrigen 14 Domains.** Bei 13 davon ist die Preisquelle nicht
   öffentlich; bei `fegert-recycling.de` existiert kein Preispfad überhaupt. Der
   Screening-Befund aus [`research/handler-backlog-20261027.md`](handler-backlog-20261027.md)
   bestätigt sich damit in der Tiefe: **der Hebel ist die Preisquelle, nicht der Handler.**
5. **Screening-Regel nachziehen.** `fegert-recycling.de` liefert auf **38** geprüften
   Preispfaden HTTP 200 mit der Startseite. Jede Screening-Liste muss neben dem
   Statuscode den Seitentitel prüfen, sonst entstehen Falschtreffer. Außerdem ist über
   **alle** Domains hinaus ein zweiter Pfad-Durchgang nötig: `proaurum.de` und
   `schrott24.de` haben die Preise nicht unter den üblichen Pfaden, sondern unter
   `/preisliste/` bzw. `/altmetall-ankauf/<kategorie>/<produkt>/`.
6. **Nächster Schritt ist nicht der Handler, sondern die Ansprache.** Für die Betreiber
   ohne Preisquelle ist der konkrete Hebel eine Kontaktaufnahme mit einer konkreten
   Frage: die in der Recherche ermittelte Preislogik (Index plus Abschlag bei WMR,
   BEHG-Aufschlag bei Götz, Kundenkarte bei ISR, Kundenportal bei Steil/RMR, internes
   Kurssystem bei Mohr). Die Formulierungen sind bewusst so gewählt, dass sie eine
   vorhandene Quelle aufdecken statt eine neue zu behaupten.

# Abgrenzung

- Diese Notiz ist **kein** Freigabe-, Release- oder Deploy-Dokument und **kein**
  Dossier-Eintrag. Es wurde keine Dossierdatei geändert, kein Code angefasst und die
  Produktionsdatenbank nicht geöffnet.
- Der Befund „keine Preisquelle" ist ein **negativer Befund zum Abrufzeitpunkt** für die
  geprüften Pfade und Sitemaps, **keine** Behauptung über das gesamte Web. Bei
  `proaurum.de` und `schrott24.de` hat die zweite Pfad-Runde gezeigt, dass ein negativer
  Erstbefund bei diesen Domains nicht belastbar wäre.
- `robots.txt`-Sperren wurden respektiert: `mcschrott.de` `/_preislisten/` wurde nicht
  abgerufen; `proaurum.de` `/rest/`, `/customer/`, `/checkout/` wurden nicht abgerufen.
- Login-Bereiche wurden nicht betreten: der proaurum-Preisradar und jedes Kundenportal
  bleiben ungeprüft.
- Der Abruf von `mcschrott.de` zeigt für die URL `/preise/` einen **Stand vom 23.10.2026**
  bei Abruf am 27.10.2026. Ob die Liste zwischenzeitlich datiert wird, ist offen.
- Die in der Recherche genannten Einzelwerte (Kupfer Millberry 10,50 €/kg, Rostock-Overrides,
  Fastmarkets-Indizes, proaurum-Basiskurse) sind **Abrufbefunde vom 27.10.2026** und keine
  Empfehlung zur Übernahme in den Datenbestand.
