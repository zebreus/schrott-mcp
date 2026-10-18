# Review G&S Bendorf – Check-in 18.10.2026

## Entscheidung für den Owner

**`street: ''` befürwortet; Text/Datierung vor Integration korrigieren.** Geprüft wurde der uncommittete Diff von `dossiers/rp/rp-bendorf-g-s.md` gegen HEAD. Einzige skalare Änderung: `S Bendorf Hauptstr. 77` → leer. Dazu neuer Überblick und ein als 12.10.2026 datierter Korrekturblock. Das Dossier wurde nur gelesen; ausschließlich dieser Bericht geschrieben. Kein Produktionszugriff, keine GEO-Abfrage/-Änderung, keine Veröffentlichung.

Die Leerung ist eine **Rücknahme eines verunreinigten, nach heutiger Prüfung nur aggregatgestützten Seed-Werts**, keine belegte Aussage, dass Hauptstr. 77 falsch sei oder G&S dort nicht existiere. Den alten Clusterwert wieder einzusetzen wäre ebenso wenig quellenhierarchiekonform wie die Normalisierung zu `Hauptstr. 77`. Das positive Gegenargument muss im Text bleiben: Mehrere heute lesbare Verzeichnisse führen genau diesen Bendorfer Kandidaten. Die Adresse ist damit ein konkreter, weiterhin offener Lead – nicht widerlegt.

### Exakte Feldentscheidung

| Feld | Empfehlung | Grundlage / Grenze |
|---|---|---|
| `street` | `''` (Diff übernehmen) | HEAD enthält Orts-/Namensrest im Straßenfeld; kein zulässiger Betreiber-/Registerbeleg für die Zuordnung gefunden. Rohwert und normalisierter Lead in Timeline bewahren. |
| `postcode`, `phone`, `email`, `opening_hours` | jeweils `''` belassen | 56170, 0172 5969230, jasarisamir@gmx.de, Mo–Sa 09:00–17:30 nur Verzeichnisangaben. |
| `website`, `website_status` | jeweils `''` belassen | TikTok nicht inhaltlich prüfbar; Münster-Domain nicht zuweisen. Auch nicht `tot`/`blockiert` aus dem einzelnen Abruf ableiten. |
| `status` | `pruefung` belassen | Identität/aktuelle Existenz/Annahmestelle nicht ausreichend belegt; weder `aktiv` noch `geschlossen` setzen. |
| `name`, `city`, `state`, `slug` | unverändert: G&S / Bendorf / RP / rp-bendorf-g-s | Bezeichnung eines ungeklärten Recherchekandidaten, keine neu bestätigte Rechtseinheit. Keine Münster- oder Ludwigshafen-Zuordnung. |
| `trader_type`, `description`, `dropoff_json`, `pickup_json`, Provenienzfelder | unverändert | Keine aus Werbe-/Reinigungstext abgeleitete GmbH, Annahmeberechtigung, Abholung oder Leistung strukturieren. |

README §§ Feedback triage / Betreiber-Ketten (insb. Zeilen 168–202, 236–239) verlangt zulässige Direktbelege statt Aggregator-Konsens. Die Ausnahme für autoritative Betreiberquellen ist hier **nicht** erfüllt: Für Bendorf fehlt schon ein zuordenbarer Betreiberauftritt; für Münster wurde kein Live-Registerabgleich durchgeführt. Drei Seiten derselben Münster-Domain sind zudem keine unabhängigen Quellen. Die allgemeine Overwrite-Regel ist kein Grund, einen erkennbaren Import-/Clusterrest als sichere Straße fortzuführen; der Owner sollte die Änderung ausdrücklich als Rücknahme eines unbelegten Felds dokumentieren, nicht als bewiesene Adresskorrektur.

## Heute tatsächlich direkt gelesene Quellen (18.10.2026)

### Bendorfer Seite: konkrete Leads, kein Betreiberbeweis

1. **11880 – direkt gelesen, ausschließlich Aggregator-Lead:**
   https://www.11880.com/branchenbuch/bendorf-rhein/120674719B107321307/schrotthandel-g-s.html
   - Titel „Schrotthandel G&S“, Hauptstr. 77, 56170 Bendorf, 0172 5969230; Mailadresse im Linktitel `jasarisamir@gmx.de`; Öffnungszeiten Mo–Sa 09:00–17:30.
   - Text verbindet Schrottabholung/„Höchstpreise“ mit Entrümpelung, Reinigung, Containerdiensten; am Ende „G&S GebäudereinigungKraft Schrott& Metall gmbh“, „J.Samir“, „J.selver“. Das ist keine überprüfte juristische Firmierung. Keine eigene Website angegeben („Website hinzufügen“).
   - Seite zeigt „aktualisiert am 03.10.2026“ und „Eintragsdaten vom 13.04.2026“. **Das sind Portalangaben, keine eigenen Abrufdaten und kein Aktualitätsbeweis des Betriebs.** „Geschlossen bis Mo., 09:00 Uhr“ ist keine dauerhafte Schließung.
2. **tellows – direkt gelesen, ausschließlich Aggregator-Lead:**
   https://www.tellows.de/num/01725969230
   - Trotz vorgeschaltetem Consent-Text lesbarer Eintrag „Altautoannahmestellen Schrotthandel G&S“, Hauptstr 77, 56170 Bendorf, genannte Nummer. Keine Betreiberidentifizierung/kein Handelsregisterbeleg; „Alle Angaben erfolgen ohne Gewähr“. Nicht als Bestätigung einer zugelassenen Altautoannahmestelle verwenden.
3. **Koblenzer Tagblatt – direkt gelesen, Branchenverzeichnis-Lead, kein redaktioneller Betriebsnachweis:**
   https://koblenzer-tagblatt.de/lokales/bendorf/schrotthandel/schrotthandel-g-s-2313/
   - Eintragsseite nennt Hauptstr. 77, 56170 Bendorf, dieselbe Mail und dieselben Öffnungszeiten. Kein lesbares Betreiber-Impressum für G&S, keine belegte Unabhängigkeit von den übrigen Verzeichnisdaten. Der Nachrichten-/Zeitungstitel macht einen Branchenlisteneintrag nicht zur zulässigen zweiten Quelle.
4. **TikTok – Direktabruf versucht, kein lesbarer Inhalt:**
   https://www.tiktok.com/@schrotthandelgs
   - `webfetch` lieferte keinen Tool-Inhalt. Kein Profiltext gelesen, kein Betreiber, Ort oder Telefon aus diesem Profil bestätigt. Nicht sagen, ein existierendes und vom Bendorfer Betreiber geführtes Profil sei nachgewiesen. Ebenso keine Löschung/Nichtexistenz behaupten.

### Namensvetter-/Gegenseite: Betreiber-Selbstauskunft statt fremde Kontaktdaten übernehmen

5. **G&S Entsorgung, Münster – drei Betreiberseiten direkt gelesen:**
   - https://www.g-s-entsorgung.de/
   - https://www.g-s-entsorgung.de/IMPRESSUM/
   - https://www.g-s-entsorgung.de/UeBER-UNS
   - Impressum nennt G&S Entsorgung UG (haftungsbeschränkt), Brüningheide 73, 48159 Münster, Sarah Gattung, HRB 22957 Amtsgericht Münster, 0163 3379051, g.s-entsorgung@web.de. Start-/Über-uns-Seite nennt Münster/Münsterland, Schrottankauf und Entsorgungsleistungen.
   - In **diesen gelesenen Seiten** keine Zuordnung zu Hauptstr. 77, Bendorf, 0172 5969230 oder den Bendorfer Namensfragmenten. Damit kein Grund zur Übertragung; **kein umfassender Beweis, dass eine Verbindung ausgeschlossen ist**. HRB und Geschäftsführerin hier nur als Impressumsinhalt, nicht registerkongruent verifiziert.
6. **Adressähnlicher Metallbetrieb – Betreiberstartseite direkt gelesen:**
   https://www.blecheundeisen.de/
   - „Bleche & Eisen GmbH“, **Rheinstr. 77**, 56170 Bendorf, 02622 92680, info@blecheundeisen.de; Autogen-/Plasmazuschnitte. Nicht Hauptstr. 77, nicht G&S; keine Ersatzadresse oder bewiesene Betreiberidentität. Kein Registerabgleich.
7. **Suchtreffer zu J.S./Schrott – Betreiberstartseite direkt gelesen:**
   https://js-schrotthandel.de/
   - JS Schrotthandel, Herxheimer Straße 47, 67065 Ludwigshafen am Rhein, 01799313066, info@js-schrotthandel.de. Keine auf der gelesenen Seite belegte Beziehung zu G&S Bendorf. Initialen sind kein Identitätsbeweis; nichts übernehmen. Keine Register-/Impressumsprüfung dieses Betriebs.

## Eigenständiger zweiseitiger Suchumfang und offene Gegenfrage

Gesucht nach G&S/G S mit Bendorf/Schrott/Hauptstr. 77, ausgeschriebener Hauptstraße 77 und PLZ, Telefonnummer, Mail, Jasari, J.Samir/J.selver sowie gezielt Northdata und Creditreform. Viele exakten Abfragen lieferten nur unpassende Treffer; die Trefferqualität begrenzt die Negativaussage. **Kein direkt gelesener passender Bendorfer Handelsregister-, Northdata-, Creditreform- oder kommunaler Gewerbenachweis liegt vor.** Eine erfolglose Websuche beweist weder fehlenden Registereintrag noch Nichtexistenz eines Einzelgewerbes.

Die Adressseite wurde separat nach einem anderen realen Betreiber an Hauptstr. 77 abgesucht. Ergebnis: weiterhin die G&S-Verzeichnisleads, aber kein belastbar identifizierter anderer Betreiber genau dort. Der Treffer Bleche & Eisen betrifft ausdrücklich eine andere Straße. Deshalb **weder Adressbelegung durch einen anderen Betrieb noch Phantomauflösung behaupten**. Der Restdatensatz bleibt bewusst ein `pruefung`-Kandidat.

Nur Suchtreffer, **nicht direkt gelesen/keine Belege**: Implisense zu G&S Münster; Facebook `schrottankaufahlen`; `g-s-entsorgung.de.tl` (Suchsnippet mit Daimlerstr. 12, Ahlen und Münsteraner Telefonnummer); weitere G&S-Namensvetter bei Northdata. Keine Umzugs-/Übernahmekette aus diesen Leads konstruieren. Der Vimeo-Film der 11880-Seite wurde nicht angesehen und nicht als unabhängige Quelle gezählt.

## Notwendige Datierungs- und Textkorrekturen

1. **Neuen Korrekturblock nicht auf 12.10.2026 rückdatieren.** Der vorliegende Review belegt eigene Abrufe erst am **18.10.2026**. Ohne separat vorhandene Original-Artefakte des Owners sowohl Überschrift als auch Bullet-Präfix auf `Korrektur 18.10.2026` ändern. Ein heutiger erfolgloser TikTok-Abruf bestätigt keinen historischen Abruf am 12.10.
2. **Historische Timeline 04.10.2026 bewahren, nicht rückwirkend attestieren.** Die Aussagen „tatsächlich abgerufen“ und „separat live abgerufen“ stammen aus HEAD; dieser Review hat keine damaligen Fetch-Logs geprüft. Heutige Inhalte sind damit vereinbar, belegen aber nicht den damaligen Inhalt/Abruf. Kein „04.10. erneut bestätigt“ ergänzen. In neuem Text ausdrücklich heutige Beobachtung von historischer Dokumentation trennen.
3. **„Verzeichnis-/Social-Media-Leads“ präzisieren:** Adresse/Telefon wurden heute aus Verzeichnissen gelesen; aus TikTok gerade nicht. Besser: „Verzeichnis-Leads; das als weiterer Lead notierte TikTok-Ziel konnte inhaltlich nicht geprüft werden.“ Einen Suchtreffer des Vorbearbeiters nicht als selbst gefunden/inhaltlich bestätigt darstellen: heutige Handle-Suche lieferte keinen passenden Profilbeleg.
4. **„Keine Primärquelle zur Firma/Annahme“ einschränken:** „Kein dem Bendorfer Kandidaten zuordenbarer Betreiber-/Registerbeleg in dieser Prüfung“; schließlich wurden Betreiberseiten eines anderen G&S gelesen. „Keine öffentliche Annahmestelle bestätigt“ ist richtig, „keine Annahmestelle vorhanden“ wäre unbelegt.
5. **Preise/Verkauf/Gebühren nicht absolut verneinen:** „Keine dem Bendorfer Kandidaten verifiziert zuordenbare öffentliche Preis-/Gebührenliste in den geprüften Quellen.“ Die Münsteraner Ankaufwerbung betrifft nicht diesen Datensatz; „Höchstpreise“ ist kein bezifferter Kurs.
6. **GEO-Passage neutralisieren:** Statt einer geokodierenden Folgeanweisung: „Standort ungeklärt; keine GEO-Prüfung oder -Änderung in diesem Check-in.“ Historische Geokodierungsnotiz nicht als hier ausgeführte Arbeit lesen.

### Vorschlag für den neuen Überblick

> „G&S“ in Bendorf bleibt ein ungeklärter Recherchekandidat (`pruefung`). Am 18.10.2026 gelesene Verzeichniseinträge führen Hauptstr. 77, 56170 Bendorf und 0172 5969230; nach der README-Quellenhierarchie sind dies Leads, keine bestätigten Betreiber-/Standortangaben. Das verunreinigte Straßenfeld „S Bendorf Hauptstr. 77“ wird zurückgenommen, nicht zu einer bestätigten Adresse normalisiert. Der Direktabruf des notierten TikTok-Ziels lieferte heute keinen lesbaren Inhalt. Die ebenfalls geprüften Betreiberseiten von G&S Entsorgung UG nennen Münster und abweichende Kontaktdaten; eine Verbindung zum Bendorfer Kandidaten ist damit nicht belegt. Aktuelle Betreiberidentität, öffentlich zugängliche Annahmestelle und zuordenbare Preis-/Gebührenlisten bleiben unbestätigt. Keine GEO-Prüfung oder -Änderung.

### Vorschlag für die neue Timeline-Notiz

> [Korrektur 18.10.2026: Gegenprüfung des unbelegten Seed-Clusterwerts: 11880, tellows und der Branchenverzeichniseintrag beim Koblenzer Tagblatt nennen weiterhin Schrotthandel G&S, Hauptstr. 77, 56170 Bendorf; sie bleiben Aggregator-Leads. Kein dem Bendorfer Kandidaten zuordenbarer zulässiger Betreiber-/Registerbeleg in dieser Prüfung gefunden. `street` von „S Bendorf Hauptstr. 77“ auf leer zurückgenommen; weder Adresse widerlegt noch Ersatzadresse bestätigt. Rohwert/Adress- und Kontaktleads bleiben in der Timeline. TikTok-Direktabruf heute ohne lesbaren Inhalt. Startseite, Impressum und Über-uns-Seite von g-s-entsorgung.de nennen Münster mit abweichenden Kontaktdaten; keine auf diesen Seiten belegte Bendorf-Verbindung, kein Registerabgleich. `status: pruefung`, keine dauerhafte Schließung abgeleitet, keine GEO-/Produktionsänderung. Die historischen Abrufangaben vom 04.10.2026 wurden nicht rückwirkend verifiziert; Quellen: https://www.11880.com/branchenbuch/bendorf-rhein/120674719B107321307/schrotthandel-g-s.html ; https://www.tellows.de/num/01725969230 ; https://koblenzer-tagblatt.de/lokales/bendorf/schrotthandel/schrotthandel-g-s-2313/ ; https://www.tiktok.com/@schrotthandelgs (kein lesbarer Inhalt) ; https://www.g-s-entsorgung.de/ ; https://www.g-s-entsorgung.de/IMPRESSUM/ ; https://www.g-s-entsorgung.de/UeBER-UNS]

**Verbleibendes Owner-Gate:** Die Feldrücknahme mit obiger Einschränkung integrieren; erst ein zuordenbarer Betreiber-/kommunaler Gewerbe-/Registerbeleg und die nach README erforderliche unabhängige Bestätigung könnten die Bendorfer Identität/Adresse positiv freigeben. Keine vorhandene Quelle in diesem Review erfüllt das. Keine Tests/Seed-Builds ausgeführt, da Dossier und Produktion read-only blieben.
