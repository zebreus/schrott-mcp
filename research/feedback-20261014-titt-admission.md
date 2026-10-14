# Aufnahmegate · Fritz Titt · Feedback 6276 / 7373

Stand und eigener Abruf: **14.10.2026**. Vorarbeit: [Audit 7, Index 19](feedback-20261014-audit7-duplicates-a.md). Beide vollständigen Originalmeldungen aus `/tmp/opencode/audit7-identity-duplicate-candidates.json`, Index 19, wurden gelesen; ihre historischen DB-Abgleiche sind keine eigenen aktuellen Produktionsabfragen.

## Entscheidung

**Gate nicht bestanden: kein neues Dossier.** Betreiberseite, konkreter Hof und aktueller vergüteter Ankauf sind stark belegt. Es fehlt jedoch der **zweite unabhängige zulässige Firmen-/Gewerbe-/Betreiberbeleg** nach README. Die Ausnahme für eine autoritative Betreiberquelle greift nicht: Das Impressum nennt keine Handelsregisternummer; ein unabhängiger Register-Livestatus wurde nicht nachgewiesen. Das ist eine Beweisgrenze, **kein Schließungsbefund** und keine Behauptung, der Betrieb existiere nicht.

| Gate | Ergebnis |
|---|---|
| Betreiberzuordnung | Eigenauftritt nennt Fritz Titt, Adresse, Mobilnummer, Mail und USt-ID; nur eine Quellenfamilie |
| Unabhängige zulässige Identität/Aktivität | Nicht nachgewiesen; kein verwertbarer Handelsregister-/Northdata-/Creditreform-/kommunaler Gewerberegisterbeleg oder unabhängiges Betreiberprofil gefunden |
| Aktueller Betrieb | Positives Eigenquellensignal: Preisliste 05.10.2026 und bevorstehende Annahme 23./24.10.2026 |
| Konkreter Standort | Siegener Straße 542, 57258 Freudenberg-Lindenberg; eigene Anfahrtsbeschreibung zur Firmeneinfahrt, nicht lediglich Impressumsadresse |
| Vergüteter Ankauf | Ja: ausdrücklich Ankauf, Auszahlung und „ab dem ersten Kilo“, Privat/Gewerbe, frei Lager Lindenberg |
| Dubletten | Kein Kandidat im gesamten lokalen Dossierbestand; aktueller Produktionsbestand nicht selbst abgefragt |

**Keine Rust-Handler-Implementierung, keine Preisimporte, keine Feedbackschließung.** Eine Dossier-Slug wird ohne Aufnahme nicht reserviert. Bei später bestandener Aufnahme wäre `nw-freudenberg-fritz-titt` ein plausibler einmalig abzuleitender stabiler Schlüssel; Owner muss dann nochmals den aktuellen Bestand abgleichen.

## Selbst abgerufene Betreiberbelege

1. [Ankaufspreisliste / aktueller Betrieb](http://www.schrott-titt.de/), zusätzlich als Roh-HTML gelesen: „Schrott- und Metallpreise Stand: 05.10.2026“, „Ankaufspreise frei Lager Lindenberg. Preisänderungen vorbehalten“, „Wir zahlen ab dem ersten Kilo, auch an Privatpersonen“. Kopfbereich verbindet Fritz Titt und Schrott-/Metallhandel mit Siegener Str. 542, 57258 Freudenberg-Lindenberg und 0171 771 85 50.
2. [Impressum](http://www.schrott-titt.de/impressum/): Fritz Titt, Siegener Str. 542, 57258 Freudenberg, Mobil 0171 7718550, `fa.titt@gmx.de`, USt-ID **DE012661307**, Gerichtsstand Siegen. Formulierung „vertretungsberechtigter Geschäftsführer“ beweist keine GmbH/KG; keine Rechtsform oder HR-Nummer daraus erfinden. USt-ID ist selbst veröffentlicht, nicht amtlich validiert.
3. **Standort-Deep-Link:** [Anfahrt](http://www.schrott-titt.de/anfahrt/) beschreibt die Zufahrt von Freudenberg bzw. Siegen/Kirchen/Niederfischbach über die parallele Straße zur L562 bis zur eigenen Firmeneinfahrt. Adresse/Kontakte kongruent. Kein GEO übernommen.
4. **Leistungs-Deep-Link:** [Leistungen](http://www.schrott-titt.de/leistungen/): Schrottankauf, Metallankauf, Recycling/Entsorgung, Demontage, NE-/FE-Metalle, Schrottabholung, Geschäfts-/Firmenauflösungen, Kabelschrott, Edelstahl und Rohstoffhandel. Abholung genannt, aber weder kostenlose Abholung noch Radius/Mindestmenge/Abholvergütung belegt.

Alle vier Seiten sind **eine Betreiberquelle**, nicht vier unabhängige Belege. HTTP wurde regulär gelesen; keine TLS-Ausnahme.

### Annahmebedingungen

- Angekündigte einzelne Annahmetermine: **Fr 23.10.2026 10–16 Uhr**, **Sa 24.10.2026 10–14 Uhr**. Nicht in dauerhaft wöchentliche Öffnungszeiten umdeuten.
- Privat und Gewerbe; Auszahlung ab dem ersten Kilo. Keine belegte Maximalmenge.
- Anlieferung: Auszahlung nur gegen gültigen Personalausweis mit inländischer Adresse; bei ausländischem Ausweis Meldebestätigung des Einwohnermeldeamtes nötig. Nicht bloß „gegen Ausweis“ verkürzen.
- Nur noch mobil erreichbar. Im HTML ist der `tel:`-Link fehlerhaft (`01712077120852050`); sichtbarer Kopf-/Impressumstext nennt konsistent **0171 7718550**. Einen späteren Parser nicht auf diesen Link stützen.

## Unabhängige Recherche und Gegenleads

Websuche u. a. nach `"Fritz Titt" Freudenberg Handelsregister Gewerbe`, `"Titt" "Freudenberg" "Northdata"`, `"Titt" "Freudenberg" "Creditreform"`, `"Titt" "Freudenberg" site:firmeneintrag.creditreform.de`, `"Titt" "Freudenberg" site:firmenwissen.de`, `"Fritz Titt" "Gewerberegister"`, `"Fritz Titt" "companyhouse"`, `"Titt" site:freudenberg-stadt.de` und Betreiberprofil-Suchen Facebook/Instagram. **Kein einschlägiger zulässiger unabhängiger Firmenbeleg aus diesen Suchen.** Such-Nichttreffer sind keine amtliche Register-Negativauskunft; keine vollständige Registerabdeckung behauptet. Gemeindeseiten/Ratsinformationssystem lieferten keinen konkreten Firmenbeleg.

Die Suchintegration lieferte stattdessen folgende **Verzeichnis-/Aggregatoren-Leads (hier Suchtreffer, keine selbst geprüften Firmenregisterauszüge)**:

- [Gelbe Seiten](https://www.gelbeseiten.de/gsbiz/d524ec40-c781-478e-bd96-3b8b76ee3282), [ENF](https://de.enfmetal.com/fritz-titt-schrott-und-metallhandel), [Scraprice](https://de.scraprice.com/profile/fritz-titt/910): 542 und kongruente Mobilnummer; erfüllen den zweiten Beleg nicht. Scraprice wiederholt sogar Betreibertexte, kein unabhängiger Preisbeleg.
- [11880](https://www.11880.com/branchenbuch/freudenberg-westfalen/120674719B110112695/fritz-titt.html), [Stadtbranchenbuch](https://freudenberg.stadtbranchenbuch.com/3577729.html), [Hausmeister.biz](https://www.hausmeister.biz/stadt/freudenberg-westfalen/profil/fritz-titt/), [Recyclingpoint](https://recyclingpoint.de/listing/fritz-titt/): **Siegener Str. 554**, alte Festnetznummern 02734 433942 bzw. 02734 1327, teils breiter Leistungs-/Öffnungszeitenmix. [Misterwhat](https://www.misterwhat.de/company/1569070-titt-fritz-freudenberg) nennt **542–554**. Keine datierte Umzugs-/Adress-/Betreiberkette belegt. Nicht als bloße Tippfehler bereinigen, 554 nicht als zweite Betriebsstätte aufnehmen und keine regelmäßigen Verzeichniszeiten übernehmen.
- [Stadtbranche-Kranlead](https://stadtbranche.de/w-titt-krane.de) nennt „Fritz Titt GbR“, `titt-krane.de`, 542. Abrufe von `http://www.titt-krane.de/` und `https://www.titt-krane.de/` scheiterten mit Transportfehlern. Daher kein verwertbarer Betreiberbeleg und keine beweisbare Gleichsetzung der GbR/Kranaktivität mit dem Schrotthandel. Selbst ein kongruenter weiterer Eigenauftritt wäre auf Quellenunabhängigkeit zu prüfen.
- Suchtreffer zu **FritzT GmbH, Markkleeberg, HRB 36587**, und historischen Archivpersonen sind andere Identitäten, keine Firmenquelle für Freudenberg.

## Breiter read-only Dublettencheck

Alle lokalen `dossiers/**/*.md`, einschließlich vorhandener fremder Änderungen/unversionierter Dossiers, wurden vollständig gelesen und nach Namen/Alias, Domain, Mail, normalisierten Rufnummern, USt-ID, Anschrift sowie Ort/PLZ durchsucht. Kein Treffer für eigenständiges `Titt`, `schrott-titt`, `fa.titt`, Mobil-Ziffern 7718550, DE012661307, Freudenberg, 57258, Siegener oder 433942. Regex `771\D*85\D*50` erfasst die verschiedene Leer-/Trennzeichenschreibung. Der breite Teilstring `titt` trifft nur die irrelevante **Tittmoning**-Timeline in `by-bischberg-schrott-michael.md`.

Vorwahl 02734 trifft ausschließlich `rp-niederfischbach-57572-weyel.md` und dessen Erwähnung in `mv-boizenburg-19258-metal-union-logistics.md`: **Weyel**, andere Adresse/PLZ und Kontakt 02734 439690; kein Titt-Kandidat. Gesamte Repo-Suche in Markdown/JSON/Rust findet die eigentlichen Titt-Bezüge nur in der Vorarbeitsnotiz. Kein vorhandenes Titt-Dossier editiert.

Die lokale `internal.db` wurde mit SQLite `mode=ro` geöffnet und enthält keine Tabellen, also keinen aktuellen Händlerbestand. Kein Public-Query-MCP verfügbar gefunden; kein eigener Produktionsabgleich. Die beiden Meldungen berichten DB-Nichttreffer vom **12.10.2026**, einschließlich Namen/Domain/Kontakt/Adresse und eines damaligen Umfeldchecks; das bleibt ausdrücklich historische Meldungsevidenz. Hier keine GEO-Abfrage oder Koordinatenbearbeitung. Vor späterer Aufnahme ist ein aktueller read-only DB-Abgleich durch Owner nötig, auch für 554/542–554 und Alt-Festnetzvarianten.

## Preise: Ankauf, Einheiten, Sorten, Handler-Eignung

**Numerische Ankaufsliste**, nicht Verkaufspreise oder Entsorgungsgebühren: Seitentitel „Ankauf und Verkauf“ ist allgemeine Metadatenbeschreibung; die konkrete Liste heißt „Ankaufspreise“. Auszahlungsaussage bestätigt Geldrichtung Händler → Anlieferer. Keine separate numerische Verkaufsliste oder Gebührenliste in den geprüften Seiten. Fehlende Gebührentabelle bedeutet nicht, dass alle Dienstleistungen kostenfrei sind.

Alle folgenden Originalwerte stehen in derselben Liste, **Stand 05.10.2026**, beobachtet 14.10.2026; keine Beobachtung auf 05.10. rückdatieren. Preisänderung vorbehalten, kein belegtes `valid_to`, keine belegte Preisgarantie oder gesonderte MwSt.-Aussage. Keine Ankaufswerte aus Metallbörsen/Verzeichnissen ergänzt.

| Originalsortenlabel | Preis laut Seite | Mapping-/Interpretationsgrenze für späteren Handler |
|---|---:|---|
| Shredder-Vormaterial | 70,00 EUR/Tonne | `stahlschrott-shredder`, EUR/t |
| Mischschrott | 90,00 EUR/Tonne | `mischschrott`, EUR/t; nicht mit Shredder zusammenlegen |
| Kupfer Millberry | 9,60 EUR/kg | `kupfer-millberry` |
| Kupfer Raff | 8,60 EUR/kg | `kupfer-gemischt`, Variante Raff |
| Kupfer Kabel (ohne Stecker) | 3,00 EUR/kg | Kupferkabel, Variante ohne Stecker; keinen unbelegten Cu-Prozentsatz erfinden |
| Messing | 4,00 EUR/kg | `messing`; keine zusätzliche Güte behaupten |
| Edelstahl V2A | 0,50 EUR/kg | `edelstahl-v2a`, nicht V4A |
| Blei | 0,90 EUR/kg | `blei`, getrennt von Batterien |
| Zink | 0,90 EUR/kg | `zink` |
| Elektro-Motoren | 0,15 EUR/kg | `elektromotoren`, keine losen Kupferwicklungen |
| Alu-Geschirr | 0,75 EUR/kg | Allgemeines Aluminium mit erhaltener Variante Geschirr; nicht unbelegt als Guss klassifizieren |
| Alu-Profile blank | 0,00 EUR/kg | Explizite Null, kein positiver vergüteter Preis; Bedeutung Nullquote vs. Platzhalter ungeklärt |
| Alu-Profile bunt | 0,00 EUR/kg | Separate Güte, nicht blank überschreiben; Bedeutung Nullquote vs. Platzhalter ungeklärt |
| Alu-Felgen | 1,00 EUR/kg | `aluminium-felgen` |
| Wendeplatten / Hartmetall | 15,00 EUR/kg | `hartmetall`, Originalvariante erhalten; kein HSS |
| Bremsscheiben | 180,00 EUR/Tonne | Gussschrott-Kandidat mit Variante Bremsscheiben; keine allgemeine Gussschrottpreis-Gleichsetzung |
| Altbatterien (mit PB-Kennung) | 0,20 EUR/kg | `batterien-blei`, PB-Kennung erhalten; keine Lithium-/Allbatterienpreise |

**17 Quotes, davon 15 positiv und 2 explizite Nullanzeigen.** Tonnenpreise als EUR/t erhalten (Katalog in `crates/ingestion/src/pipeline.rs`); nur bei bewusst nötiger Umrechnung /1000 → Shredder 0,07, Mischschrott 0,09, Bremsscheiben 0,18 EUR/kg. Keinesfalls 70/90/180 als EUR/kg importieren. Nicht alle Werkstofflabel sind detaillierte Qualitätsdefinitionen; Bilder ersetzen keine textlich belegten Materialgrade.

**Technisch gut geeignet, fachlich noch gesperrt:** Roh-HTML enthält jede Quote serverseitig in `.grid_3 > .box`, mit eigener `h2`-Sortenüberschrift und `td.currency`; kein Browser/PDF/OCR nötig. Ein späterer Händler-spezifischer Parser kann Label und Preis innerhalb desselben Kartencontainers paaren, `Tonne`/`kg` explizit erkennen und das Listenstanddatum als `published_at_basis=page_stated` verwenden. Globale Tabellen-ID `tables` ist mehrfach verwendet und darf nicht der eindeutige Selektor sein. Annahmetermine sind keine Preisdatumsquelle.

Vor Implementierung: Aufnahmebeleg nachholen; dann Tests für 17 Karten, deutsche Dezimalkommas, drei EUR/t-Zeilen, zwei getrennte Aluminiumprofil-Nullwerte, Raff/ohne-Stecker/PB-/V2A-Güten und entkoppelte Annahmetermine. Nullwerte nicht still zu positiven Preisen machen oder als Nichtannahme ausgeben; sie zunächst separat dokumentieren und für automatische aktuelle Vergütungsvergleiche zurückhalten, bis die Semantik geklärt ist. Gemeinsame Katalogslugs/Varianten endgültig am bestehenden Schema prüfen. Keine neue Rust-Datei und keine Registry-Änderung in diesem Auftrag.

## Owner-Handoff / Schutz

Owner-Ergänzung 14.10.2026: Fehlenden aktuellen Produktionsabgleich separat nachgeholt, ausschließlich `/var/lib/schrott-mcp/public.db` mit `mode=ro` und `query_only`. Alle 3881 Zeilen/Felder normalisiert nach Domain/Mail, USt-ID, Mobil-/Altfestnetznummern sowie Siegener-Str.-542/554 gesucht; zusätzlich Name Titt, Stadt Freudenberg und PLZ 57258. **0 Kandidaten**. Artefakt `/tmp/opencode/titt-owner-production-check-20261014.json`. Die lokale leere `internal.db` ist keine Produktionsquelle. Der Nichtfund schließt anonyme/ungenügend identifizierte Dubletten nicht absolut aus und ersetzt den weiterhin fehlenden unabhängigen Aufnahmebeleg nicht. Alle 155 geschützten Datei-Hashes erneut unverändert.

Benötigt wird ein unabhängiger zulässiger Nachweis, der **Fritz Titt, den aktuellen Schrott-/Metallhandel und den Betrieb in Freudenberg** tatsächlich verbindet; bei 554 oder GbR-Abweichung zusätzlich eine belegte Betreiber-/Standortkette. Mit bloßer Verzeichniskongruenz oder vier Seiten derselben Domain darf das Gate nicht geschlossen werden. Bis dahin bleiben beide Original-Feedback-IDs und die Adressgegenleads offen.

Einzige eigene Repo-Änderung: **dieser Bericht**. Keine bestehenden Dossiers oder fremden Dateien bearbeitet, kein neues Dossier angelegt. Auswahl `/tmp/opencode/wave-20261014-selection.json` nur gelesen; SHA-256 beim Check: `b29e92e659701bff200e41e820e26c2e6b29419fe6930c25699b6623bd734916`. Keine Kontaktaufnahme, TLS-Ausnahme, Produktionswrites, GEO, Commits, Pushes oder Deploys. Kein Cargo-Test nötig für reine Research-Datei; keine Seed-/Handleränderung zu validieren.
