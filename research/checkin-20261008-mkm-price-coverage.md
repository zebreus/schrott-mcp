# Feedback #5557 — MKM Metals: Preisabdeckung, 08.10.2026

## Ergebnis / Freigabegrenze

**Coverage-Fehler bestätigt; Mengen und Aktualität des Feedbacks präzisieren.** Aktuelle Betreiberquelle veröffentlicht **60 Sorten in 8 Gruppen**, davon **55 numerische Ankaufpreise und 5 „auf Anfrage“**. Der unveränderte echte Handler liefert **11 Preise**, 3 explizite Kupfer-Skips und **0 acceptances**. Somit fehlen **44 numerische Preise (41 außerhalb Kupfer + 3 innerhalb Kupfer)**; weitere 5 Anfrage-Sorten sind keine Zahlenpreise. Produktionsbestand hat tatsächlich 12 Current-Zeilen, davon eine historische doppelte Steckerkabel-Zuordnung. Keine Codefixes, kein Commit/Deployment, keine DB-Schreiboperationen; Owner übernimmt gemeinsame Umsetzung und Veröffentlichung.

Untersucht bei Repo-HEAD `072cf25cee749795e104624daacc2608411e84bd`; Arbeitsbaum-Handler live ausgeführt. Quellen am 08.10.2026 abgerufen. Webseiten können sich weiter ändern; nachfolgende Zahlen sind eine Beobachtung, keine harte zukünftige Sollzahl.

## Quellen und Betreiberzuordnung

- **[P1: vollständige Preisübersicht](https://mkm-metals.de/metallpreise)**: „Preisliste · zuletzt geändert 08.10.2026“, „Alle 60 Sorten“, 8 Tabellen mit Caption „Ankaufpreise für …“. Gezählt wurden ausschließlich Produktzeilen dieser Tabellen, nicht wiederholte Sortiment-Karten, Navigation oder FAQs.
- **[P2: Impressum](https://mkm-metals.de/impressum)**: MKM Metals e.K., Inhaber Mubarik Ahmad, Industriestraße 4, 67063 Ludwigshafen am Rhein, HRA 61664 Amtsgericht Ludwigshafen am Rhein; Telefon 0176 220 168 40, contact@mkm-metals.de, Montag–Samstag 09:00–18:00. Ankaufpreise ausdrücklich Richtwerte für sortenreines Material; verbindlich sind Begutachtung und geeichte Verwiegung.
- **[P3: konkrete Betriebsstätte](https://mkm-metals.de/standort-ludwigshafen)**: tatsächlicher Schrott-/Metallankauf an Industriestraße 4, geeichte Fahrzeugwaage, Selbstanlieferung/Versand/Abholung ab 500 kg. Dossier und Produktionshändler stimmen in Name, Domain, Straße, PLZ und Stadt damit überein. Kein Befund einer Betreiberverwechslung; Preise nicht pauschal auf die zusätzlich im Footer verlinkten Standorte Goslar/Braunschweig übertragen.
- **[P4: Kupfer Millberry / Produktstaffel](https://mkm-metals.de/metallankauf/produkt/kupfer-millberry)**: Artikel MET-KU-MILL, 10,60–11,13 EUR/kg; Mengenstaffel bis 40 kg 10,60 / ab 40 kg 10,81 / ab 100 kg 10,97 / ab 250 kg 11,13. Übersicht ist der Höchstpreis, kein universeller Kleinmengenpreis.
- **[P5: „Metallverkauf“ aus Kundensicht](https://mkm-metals.de/leistungen/metallverkauf)**: Kunden verkaufen Altmetall an MKM; nennt ebenfalls 60 Sorten in 8 Gruppen, Abrechnung/Gutschrift und Gebühren. Der URL-Name ist kein Beleg für Händler-Verkaufspreise.
- **[P6: Metalleinkauf für Industriekunden](https://mkm-metals.de/leistungen/metalleinkauf)**: Beschaffung von Sekundärrohstoffen für Kundenproduktion; keine öffentliche numerische Händler-Verkaufspreisliste auf der geprüften Seite.

Registerkongruenz wurde im erhaltenen Dossierverlauf am 01.10.2026 recherchiert; diese Recherche hat **keinen neuen amtlichen Registerabruf** vorgenommen. Aktuelle Betreiberseiten bestätigen die dort hinterlegten Stammdaten; keine Änderung der Identitäts-/Kontakt-Frontmatter nötig. Impressum/Preis-/Standortseiten sind mehrere Dokumente desselben Betreibers, keine unabhängigen Registerquellen.

## Originalfeedback versus heutige Beobachtung

`internal.db.feedback`, ausschließlich read-only: #5557, medium, erstellt `2026-10-08T19:16:08Z`. Titel „nur 11 von 57 Sorten“, Details „12 current_prices“, Seite zuletzt geändert 07.10.2026, 7 Gruppen; V4A-Beispiel 2,15 EUR/kg.

- Die 7 im Feedback aufgezählten Gruppen summieren sich tatsächlich auf **57** Sorten. Heute kommt „Schrott“ mit 3 Anfrage-Sorten hinzu: **60 / 8**. Ohne historischen HTML-Snapshot lässt sich nicht behaupten, dass die achte Gruppe schon beim Feedbackabruf sichtbar war.
- „46 Sorten der übrigen 6 Gruppen“ ist rechnerisch falsch: **57 − 14 = 43 Sorten**, davon 2 „auf Anfrage“, also **41 Zahlenpreise außerhalb Kupfer**. **46** wäre 57 − 11, schließt aber die 3 Kupfer-Mapping-Skips ein und enthält Anfrage-Sorten.
- V4A steht aktuell bei **1,77**, „Geändert: heute“, statt der gemeldeten 2,15. Deshalb kein Fix auf den historischen Feedbackbetrag. Kupferwerte des Livehandlers stimmen mit der heutigen Übersicht überein.

## Vollständiger Gruppenzähler

Quelle für sämtliche Tabellenzahlen: P1, direktes HTML, HTTP 200; Livehandler `status=200 bytes=252882`. Snapshot außerhalb Repo: `/tmp/opencode/mkm-metallpreise-20261008.html`, SHA-256 `5821afef76711d99972614c1a016e0d149e892f500344f980fec79a40232406b`; daraus extrahierte Produktzeilen `/tmp/opencode/mkm-inventory-20261008.json`. Temporäre Artefakte sind keine versionierten Fixtures.

| HTML-Gruppe | Sorten | Zahlenpreise | Anfrage | Handler-Zahlenpreise |
|---|---:|---:|---:|---:|
| kupfer | 14 | 14 | 0 | 11 |
| messing | 8 | 8 | 0 | 0 |
| aluminium | 13 | 13 | 0 | 0 |
| edelstahl | 3 | 3 | 0 | 0 |
| ne-metalle | 10 | 8 | 2 | 0 |
| blei-zink-e-motoren | 4 | 4 | 0 | 0 |
| hartmetall | 5 | 5 | 0 | 0 |
| schrott | 3 | 0 | 3 | 0 |
| **Gesamt** | **60** | **55** | **5** | **11** |

## Reproduzierbare Lücken: die 44 fehlenden Zahlenpreise

Alle Werte EUR/kg und **Höchstpreise** aus P1, nicht unbedingte Fixpreise. Artikelnummer/Link liefern bessere Identität als veränderlicher Labeltext; aktuelle Gruppenmitgliedschaft aus tatsächlicher Tabelle nehmen. P1 hat selbst einen veralteten Sortimenttext, der Alu-Kupfer-Kabel noch bei Blei/Zink nennt, obwohl SW10035 in der Kupfertabelle steht.

| Gruppe | Artikel | Sorte | EUR/kg | Höchstpreis ab kg |
|---|---|---|---:|---:|
| Kupfer | MKM-CU-02 | Kupfergranulat | 10,50 | 250 |
| Kupfer | SW10017 | Kupfer/Messing Kühler ohne Eisen | 5,10 | 250 |
| Kupfer | SW10035 | Alu Kupfer Kabel | 1,04 | 250 |
| Messing | SW10024 | Rotguss-Stücke | 9,69 | 250 |
| Messing | SW10025 | Rotguss-Späne | 9,17 | 250 |
| Messing | SW10020 | Messing Hülsen | 7,35 | 250 |
| Messing | SW10019 | Messing 63 Abfälle | 7,09 | 250 |
| Messing | SW10018 | Messing 58 Abfälle | 6,71 | 500 |
| Messing | SW10021 | Messing schwer | 6,57 | 500 |
| Messing | SW10023 | Messing Späne | 6,38 | 500 |
| Messing | SW10022 | Messing leicht | 4,52 | 500 |
| Aluminium | SW10011 | Alu Kupfer Kühler Fe frei | 5,16 | 500 |
| Aluminium | SW10001 | Alu Profile blank/max. 10%eloxiert | 2,61 | 500 |
| Aluminium | SW10002 | Alu Felgen sauber | 2,43 | 500 |
| Aluminium | MKM-AL-06 | Alu Profile Neu | 2,34 | 250 |
| Aluminium | SW10000 | Alu Iso Profile Ausbau | 1,89 | 500 |
| Aluminium | MKM-AL-01 | Alu Profile mit Farbe | 1,89 | 500 |
| Aluminium | SW10003 | Alu Felgen unsauber | 2,03 | 10000 |
| Aluminium | SW10004 | Alu Geschirr max. 10% Anhaftungen | 1,47 | 500 |
| Aluminium | SW10012 | Alu Späne Basis 5% | 1,44 | 500 |
| Aluminium | MKM-AL-02 | Alu Bleche neu blank | 1,37 | 500 |
| Aluminium | MKM-AL-03 | Aluminium gemischt | 1,37 | 500 |
| Aluminium | MKM-AL-04 | Alu Guss sauber | 1,30 | 10 |
| Aluminium | MKM-AL-05 | Aluminiumkabel | 0,90 | 10 |
| Edelstahl | SW10028 | V4A Abfälle bis 1,5m | 1,77 | 250 |
| Edelstahl | SW10026 | V2A Abfälle bis 1,5m | 1,04 | 500 |
| Edelstahl | SW10029 | V2A-Späne | 0,88 | 500 |
| NE-Metalle | MKM-NE-06 | Wolfram ab 99 % | 49,35 | 50 |
| NE-Metalle | SW10040 | Zinn 99% | 31,50 | 100 |
| NE-Metalle | SW10039 | Zinn 90% | 25,73 | 100 |
| NE-Metalle | SW10042 | Zinn 60% | 15,75 | 100 |
| NE-Metalle | MKM-NE-05 | Nickel rein ab 99 % | 11,90 | 10 |
| NE-Metalle | MKM-NE-03 | Lötzinn | 8,50 | 10 |
| NE-Metalle | SW10041 | Zinn 30% | 7,35 | 250 |
| NE-Metalle | MKM-NE-04 | Nickel 60-99 % | 6,90 | 10 |
| Blei/Zink | SW10031 | Zink | 2,38 | 500 |
| Blei/Zink | SW10030 | Blei | 1,26 | 500 |
| Blei/Zink | SW10033 | E-Motoren ohne Anbauteile max. 500KG | 1,10 | 500 |
| Blei/Zink | MKM-BL-01 | Geschossblei | 0,42 | 500 |
| Hartmetall | MKM-HM-01 | Hartmetall gemischt | 54,20 | 50 |
| Hartmetall | SW10037 | VHM Bohrer | 54,20 | 30 |
| Hartmetall | SW10036 | Widia Wendeschneidplättchen | 54,20 | 30 |
| Hartmetall | MKM-HM-02 | Hartmetall Stückschrott | 49,20 | 30 |
| Hartmetall | SW10038 | HSS Schnellarbeitsstahl | 1,70 | 10 |

**Anfrage-Sorten:** MKM-NE-01 Leiterplatten, MKM-NE-02 Neodym-Magnete, MKM-SCH-01 Eisenschrott, MKM-SCH-03 Metallschrott, MKM-SCH-02 Mischschrott. Keine Nullpreise erfinden; gegebenenfalls als Annahme erfassen, nach geklärter Katalogzuordnung.

## Tatsächlicher Handleroutput / Ursachen

DB-freier realer Handlerlauf, erfolgreich gebaut und ausgeführt:

```sh
/root/.cargo/bin/cargo run -p schrott-mcp-ingestion --example live_handlers -- rp-ludwigshafen-mkm-metals
```

Signal: `status=200`, `published_at=None`, `acceptances=0`, `alive=true`, Kontakt exakt wie P2. 11 Preise mit `conf=Some(0.5)`:

```text
kupfer-millberry                     11.13  Kupfer Millberry
kupfer-berry / Kerze                 10.92  Kupfer Kerze
kupfer-gemischt / Sammelschienen     10.92  Kupfer Sammelschienen
kupfer-gemischt / schwer             10.40  Kupfer (Cu) schwer Basis 95% Kupfer-Inhalt
kupfer-berry / Berry 98%             10.29  Cu Berry / 98%
kupfer-gemischt / Raff               10.29  Kupfer Raff
kupfer-gemischt / leicht              6.93  Kupfer leicht
kabel-kupfer / 70%                    5.99  Kupferkabel ca. 70%
kabel-kupfer / 60%                    4.94  Kupferkabel ca. 60%
kabel-kupfer / 38-40%                 3.93  Cu Kabel gemischt (38-40%Cu)
kabel-mit-stecker / Stecker 30%       2.29  Cu Steckerkabel (30%Cu)
SKIP: Kupfergranulat
SKIP: Kupfer/Messing Kühler ohne Eisen
SKIP: Alu Kupfer Kabel
```

Gerankte Diagnose nach rotem Coverage-Signal: (1) explizit begrenztes Tabellenfenster, (2) zusätzliche Katalog-/Mapping-Lücken, (3) historische Current-Duplikate. Alle bestätigt, keine Netzwerk-/Betreiberverwechslung als Erklärung nötig:

1. `crates/ingestion/src/traders/handlers/mkm_metals.rs:292–385`: `parse()` nimmt nur Fenster `id="kupfer"` bis `id="messing"`. Die übrigen Gruppen werden **gar nicht besucht** und erscheinen daher auch nicht in `skipped_labels`. Scope ist in Modulkommentar explizit als Kupfer-only beschrieben, keine kurzfristige CSS-Störung.
2. `grade_for():91–129`: Granulat und Kühler bewusst `None`; Alu-Kupfer-Kabel enthält keine der erlaubten 70/60/38/Stecker-Kombinationen und fällt ebenfalls heraus. Öffnen des Fensters allein wäre gefährlich: ungesicherte generische Arme wie `schwer`/`leicht` könnten etwa Messing falsch als Kupfer klassifizieren.
3. `stand_date():275–289` sucht nur „Stand “. P1 lautet inzwischen „zuletzt geändert 08.10.2026“; echter Output `published_at=None`. Außerdem besitzen Produktzeilen konkrete `<time datetime>`-Änderungsdaten. Page-last-change und sortenspezifisches Änderungsdatum nicht gleichsetzen. Die gemeinsame Ingestion (`traders/mod.rs:344–374`) kann alternativ Beobachtungs-Änderungsdaten ableiten; das ist kein Ersatz für falsch übersehene publizierte Quelldaten.

## Reproduktionssignal (keine Produktionsschreibzugriffe)

Tatsächlich ausgeführt: `python3 /tmp/opencode/mkm-coverage-check.py`; sammelt aktuelle Tabellenzeilen und ruft `target/debug/examples/live_handlers` auf. Ergebnis **Exit 1**, `Coverage: 11/55 numeric rows; 44 missing`, `AssertionError: Coverage regression: 11 != 55`. Snapshot und Rohoutput unter `/tmp/opencode/mkm-*`. Das ist bewusst rot, kein bestandener Test. Live-Netzwerk nicht deterministisch; für Regression gespeichertes HTML als Fixture verwenden. Nach gebautem Example beträgt die Schleife Sekunden.

Ohne temporäres Script reproduzierbar mit Standard-Python, aus Repo-Wurzel nach obigem Build:

```sh
python3 - <<'PY'
import re, subprocess, urllib.request
html = urllib.request.urlopen('https://mkm-metals.de/metallpreise', timeout=30).read().decode()
sections = [b for b in re.findall(r'<section\b[^>]*>(.*?)</section>', html, re.S)
            if 'Ankaufpreise für' in b]
rows = [r for b in sections for r in re.findall(r'<tr\b[^>]*>(.*?)</tr>', b, re.S)
        if '/metallankauf/produkt/' in r]
numeric = sum('/kg' in r for r in rows)
out = subprocess.check_output(['target/debug/examples/live_handlers', 'rp-ludwigshafen-mkm-metals'], text=True)
actual = sum(' conf=' in line for line in out.splitlines())
print(f'groups={len(sections)} rows={len(rows)} numeric={numeric} handler={actual}')
assert actual == numeric, (actual, numeric)
PY
```

Der Count-Check lokalisiert den vorhandenen Fehler, reicht später allein aber nicht: gleiche Anzahl könnte falsche/duplizierte Sorten verdecken. Regression soll anhand Artikel-ID die gesamte Inputmenge in Preise, Anfrage-Annahmen und explizite Mapping-Gaps partitionieren.

## Produktion: 12 Zeilen sind nicht 12 aktuelle Sorten

Beide DBs **ausschließlich `sqlite3 -readonly`** geöffnet; kein Scheduler/Seeder gestartet. Reproduktionsabfragen:

```sh
sqlite3 -readonly -header -column /var/lib/schrott-mcp/internal.db "SELECT id,severity,feedback,details,created_at FROM feedback WHERE id=5557;"
sqlite3 -readonly -header -column /var/lib/schrott-mcp/public.db "SELECT material_slug,variant,price,price_kind,observed_at,published_at FROM v_current_prices WHERE trader_slug='rp-ludwigshafen-mkm-metals' ORDER BY material_slug,variant;"
```

11 heute vom Handler gelieferte Material/Variant-Schlüssel haben Produktionsbeobachtung `2026-10-08T17:20:43.338670865+00:00`. Zusätzlich steht `kabel-kupfer / Stecker 30%` mit 2,29 EUR/kg seit `2026-09-29T19:45:47.963507650+00:00` neben `kabel-mit-stecker / Stecker 30%` desselben Produkts. Das ist ein alter Mapping-Schlüssel, keine zwölfte Quelle-Sorte. Für fast alle anderen Zeilen ist `published_at=2026-09-28T00:00:00+00:00`, für neue Steckerkabelzuordnung NULL. Aus dieser Ansicht allein folgt nicht, welcher historische Fallback/Versionsstand den alten Datumswert verursacht hat.

Current wird pro Händler + Material + Variant geführt (README/`v_current_prices`), nicht als vollständiger Snapshot der aktuellen Website. Frühere Schlüssel verschwinden nicht allein deshalb, weil ein neuer Scrape sie nicht mehr liefert. Fehlende Website-Artikel und Mappingänderungen brauchen daher ein ausdrückliches, historieerhaltendes Gültigkeits-/Retirementkonzept; keine append-only Preisgeschichte löschen.

## Ankauf, Verkauf und Gebühren strikt trennen

- **Ankauf:** alle 55 Zahlen aus P1 sind Kunden-Erlöse, Höchst-/Staffelrichtwerte. Vorhandenes `price_kind="upto"`, `price_max=price`, confidence 0,5 ist sinnvoller als ungeprüft „fixed“. Die Mengenbedingung ist nicht strukturiert im aktuellen `ScrapedPrice`; keine pauschale „ab 250 kg“-Übernahme für andere Gruppen. P4 belegt explizite Staffel, während allgemeine Prosa in P1/P5 teils abweichende Staffelbeschreibungen enthält; produktspezifische Tabelle ist für den konkreten Preis maßgeblich.
- **Händler-Verkauf / Beschaffung durch Kunden:** P6 bietet Produktionsmaterial auf Angebot, keine bezifferten Verkaufs-Kilopreise gefunden. P5 heißt „Verkauf“, beschreibt jedoch Verkauf **an MKM**. Nicht mit Händler-Verkaufspreisen vermengen.
- **Gebühren laut P5:** Selbstanlieferung ohne Transportkosten; Abholung ab 500 kg deutschlandweit **40–350 EUR nach Entfernung**, Versand **6 EUR/Paket** bis 31,5 kg/Paket und 1000 kg/Vorgang, PayPal **2 % Einbehalt**. Mindestauszahlung **25 EUR** ist eine Transaktionsbedingung, kein EUR/kg-Preis. P1 nennt zusätzlich Containerkosten ohne hier bezifferten Betrag; P4 legt Rücksendekosten beim Kunden, ohne Betrag. Diese Positionen getrennt von Ankaufkursen behandeln, nicht als negative Metallpreise importieren und nicht pauschal vom veröffentlichten Höchstkurs abziehen. Betreiberangaben sind keine eigenständige Prüfung der erwähnten Rechtsnormen.

## Sinnvolle nächste Schritte für den Owner

1. **Fixture-first:** heutiges HTML sichern, Test über echten Parser/Mappings mit sämtlichen Gruppen schreiben. Neue/wegfallende/umbenannte Artikel und gruppenübergreifende Umordnung testen; keine feste „57“-Sollzahl. Eingang = Preis + Anfrage + expliziter Skip, ohne stillen Rest.
2. **Alle echten Ankauf-Tabellen besuchen:** Caption/Section semantisch begrenzen, nicht blind sämtliche Tabellen oder Detailseiten-Doppelungen einlesen. Mapping gruppen-/artikelbewusst, Varianten erhalten (Späne/Stücke/Gehalte/sauber/unsauber).
3. **Catalog-Entscheidungen separat:** vorhandene DB-Materialien u.a. `messing`, `bronze-rotguss`, `aluminium-profile`, `aluminium-felgen`, `alu-cu-kuehler`, `edelstahl-v2a/v4a`, `zinn`, `loetzinn`, `blei`, `zink`, `elektromotoren`, `hartmetall`, `hss-werkzeuge` erlauben viele Ergänzungen ohne neue Katalogslugs. Granulat, Cu/Ms-Kühler, Wolfram, Nickel, Neodym und Alu-Cu-Kabel besitzen keinen offensichtlich passenden spezifischen Katalogeintrag; fachlich entscheiden, nicht ersatzweise generisches Kupfer/Hartmetall nehmen. Alu-Späne etc. benötigen ebenfalls bewusste Katalog-/Variantwahl.
4. **Anfrage-Sorten:** Annahmen mit Herkunft erfassen, sofern Katalogbezug verifiziert; keine Phantompreise. Für unbekannte Sorten sichtbaren Coverage-Alarm mit Artikel-ID/Label/Gruppe statt stiller Drop.
5. **Datums-/Staffelmodell klären:** neue Seitenform „zuletzt geändert“ erkennen; pro-Zeile `<time datetime>` von Seitenänderungsdatum unterscheiden. Datenmodell hat derzeit nur `HandlerOutcome.published_at` für alle Preise. Staffelbedingungen explizit erhalten; nicht Kleinmengenhöchstpreis suggerieren.
6. **Historieerhaltende Bereinigung planen:** alten `kabel-kupfer / Stecker 30%` Current-Schlüssel und spätere Entfernungen behandeln. Dazu gezielte Migration/Retirement nach Owner-Freigabe, nicht DB-Handarbeit in dieser Recherche.
7. **Danach Gates und Veröffentlichung:** Parser-/Catalog-/Seedtests, DB-freier Livecheck, read-only Produktionsabgleich nach regulärem Deployment; Feedback erst mit überprüfter Coverage schließen. Diese Diagnose bestätigt den offenen Defekt und behauptet keine Behebung.

## Durchgeführte Validierung

- `/root/.cargo/bin/cargo test -p schrott-mcp-ingestion mkm_metals`: **5 bestanden**, 505 ausgefiltert; Dossier-Codegen erfolgreich. Das sind vorhandene Kupfer-/Kontakt-/Datumseinheitstests mit alter „Stand“-Fixture, kein Test vollständiger Live-Coverage.
- `git diff --check -- dossiers/rp/rp-ludwigshafen-mkm-metals.md research/checkin-20261008-mkm-price-coverage.md`: erfolgreich, keine Whitespace-Fehler. Kein vollständiger Workspace-/Seed-Test behauptet.
- Coverage-Reproduktion bleibt bewusst **rot (11/55)**. Keine Handler-/Schemaänderung vorgenommen.
