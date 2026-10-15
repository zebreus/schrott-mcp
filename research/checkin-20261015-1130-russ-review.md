# RUSS Entsorgung – Review der vorbestehenden Änderungen

Stand: 15.10.2026. Prüfobjekt: `dossiers/bw/bw-neu-ulm-russ-entsorgung.md`, Original `HEAD` (`4609df44dd9b753894ebf7cfcb5e947c5cbf083b`) gegen Arbeitsfassung. Nur dieser Bericht wurde bearbeitet; keine Dossieränderung, kein Commit/Push/Deploy, keine Produktions-DB-Abfrage oder -Änderung. Tests und Integrationsgate bleiben beim Owner.

## Entscheidung

**Die bestehende Änderung ist als konservative Rechercheergänzung sachlich weitgehend freigabefähig, aber NICHT als abgeschlossene Landes-/Betreiberbereinigung.** Die inzwischen direkt geprüften Quellen erlauben eine konkretere Entscheidung:

- RUSS ist ein realer Entsorgungsbetrieb an **Otto-Hahn-Straße 26, 89231 Neu-Ulm, Bayern**. Keine Ortsverlagerung aus einem BW-Seed ableiten: Hier ist die falsche Landesklassifikation zu berichtigen, nicht ein nachgewiesener Umzug zu dokumentieren.
- Aktueller Betreiber: **Russ Entsorgung GmbH & Co. KG, AG Memmingen HRA 13628**. Die bestehende Website gehört zu diesem Betrieb; Name, Ort, Geschäftsfeld und bisherige Timeline passen zusammen. Kein nachgewiesener zweiter RUSS-Betrieb in Ulm/BW, dem diese Zeile stattdessen zugeordnet werden müsste.
- Owner-Empfehlung für eine separate, belegte Korrektur: **`state: BY`, Slug unverändert**; Adresse/Kontakte und aktueller rechtlicher Name sind jetzt ausreichend belegt. Kein neues BY-Duplikat und kein Verschieben/Umbenennen allein wegen des Präfixes `bw-`. Der bestehende Pfad kann zur Wahrung des Umfangs zunächst bleiben; die Ablageentscheidung ist kein Beleg für den tatsächlichen Standort.
- **`status: pruefung` beibehalten**, solange unklar ist, ob Schrott gegen Vergütung angekauft wird. Entsorgung/Verwertung von FE-/NE-Metall ist hingegen ausdrücklich nachgewiesen. Keine Heraufstufung zum bestätigten Ankaufbetrieb aus Zertifikat oder Unternehmensgegenstand.
- Für eine inhaltlich abgeschlossene Integration sollte der Owner die neu verifizierten Belege in einer datierten Timeline-Ergänzung aufnehmen und die unten genannten absoluten Preisformulierungen begrenzen. Historische Recherche 01.10.2026 erhalten, aber deren damaligen Einzelbeleg-Blocker nicht als heute unverändert darstellen.

## Original HEAD versus Arbeitsfassung

`git show HEAD:…` und `git diff HEAD -- …` direkt gelesen:

| Bereich | HEAD | Arbeitsfassung | Bewertung |
|---|---|---|---|
| Frontmatter | BW, Neu-Ulm, leere Adresse/Kontakte, `schrotthaendler`, `pruefung` | unverändert | Keine neuen strukturierten Behauptungen; der bekannte Landesfehler bleibt aber wirksam. |
| Überblick | Platzhalter | Betreiber/Adresse, BW-Widerspruch, Preisabgrenzung | Im Kern richtig; Betreiber-Einzelquelle ausdrücklich benannt. |
| Timeline | Import + Recherche 01.10.2026 | zusätzlicher Eintrag 13.10.2026 | Alte Historie nicht überschrieben; nachvollziehbare Quellen und Abrufdatum. |
| Preise | keine neuen Angaben | Ankauf/Verkauf/Gebühren getrennt; Shop nicht abrufbar | Richtige Trennung; keine Zahlen aus Suchsnippets übernommen. |

Kein Beleg für einen durch die Änderung verursachten Umzug oder eine Zusammenführung. Die Aussage „GEO erst nach Bundesland-/Entitätsklärung“ ist ein vorsichtiger Arbeitsvorbehalt, kein aktueller Beweisnotstand mehr bezüglich der Adresse. **Ein Koordinatenpaar wurde trotzdem nicht recherchiert/verifiziert.**

Die Timeline wird laut README/build.rs in DB-Notizen übernommen; bloßer Überblickstext korrigiert das strukturierte `state` nicht. `crates/ingestion/build.rs:286–293` verwendet einen vorhandenen Frontmatter-Staat statt des Verzeichnis-Fallbacks. Das ist keine ausgeführte Testfreigabe für eine Änderung; vollständige Seed-Validierung bleibt Owner-Aufgabe.

## Direkt geprüfte Quellen (Abruf jeweils 15.10.2026)

### P1 – Betreiber-Impressum

<https://www.russ-entsorgung.de/impressum>

Gelesen: Russ Entsorgung GmbH & Co. KG; Otto-Hahn-Str. 26, 89231 Neu-Ulm; HRA 13628, Amtsgericht Memmingen; vertreten durch NUR Neu-Ulmer Recycling Verwaltungs-GmbH, HRB 7450, AG Memmingen; deren Vertreter Peter Schaible und Thomas Kentischer. Telefon +49 (0)731 97950-0; Header-Mail `info@russ-entsorgung.de`; USt-ID DE345030375. **NUR ist die Vertretungsgesellschaft, nicht als zweiter Entsorgungsstandort zu zählen.**

### P2 – Registerbasierter unabhängiger Gegencheck

<https://www.northdata.de/Russ%20Entsorgung%20GmbH%20%26%20Co.%20KG,%20Neu-Ulm/Amtsgericht%20Memmingen%20HRA%2013628>

Detailseite tatsächlich gelesen, nicht nur Suchtreffer: Name und HRA stimmen mit P1 überein; Adresse Otto-Hahn-Str. 26, D-89231 Neu-Ulm. Vorname der Gesellschaft: **Recycling Burlafingen GmbH & Co. KG**. Publikationsübersicht: Eintragung 01.06.2021 unter diesem Namen; 01.07.2021 Anschrift/Firmensitz Neu-Ulm/Name Russ Entsorgung GmbH & Co. KG; 06.07.2021 Wechsel der persönlich haftenden Gesellschafterin zu NUR. Weitere Prokura-Publikation 19.01.2026; GWV-Gesellschafterspur 26.02.2026.

Grenze: Kein beglaubigter aktueller Registerauszug abgerufen, historische Originalpublikationen nicht einzeln ausgewertet. Die gelesene Seite zeigt keine Erloschen-/Liquidationsmarkierung und aktuelle Publikationen; zusammen mit aktuellem Zertifikat stützt dies den laufenden Betreiber. Keine Rechtsnachfolge eines anderen, vor 2021 bestehenden Russ-Rechtsträgers allein aus „seit 1981“ konstruieren. Insbesondere sind Datum und frühere Adresse einer tatsächlichen Verlagerung damit nicht bewiesen.

### P3 – Aktuelle Standortseite und Zertifizierung

- <https://www.russ-entsorgung.de/kontakt>: gleicher vollständiger Betreiber/Standort, Telefon 0731-979500, standortspezifische Anfahrt und saisonale Öffnungszeiten. Mo–Fr 07:00–12:00 und 13:00–16:30; März–Oktober zusätzlich Sa 09:00–13:00, **nur Selbstanlieferungen**. November–Februar samstags geschlossen, individuelle Öffnung nach Absprache möglich. Selbstanlieferung ist kein Zahlungsversprechen.
- <https://www.russ-entsorgung.de/service/downloads>: aktuelle 2026-Dokumente und Zertifikat verlinkt.
- <https://www.russ-entsorgung.de/images/pdf/Russ-KG-EfB-Zertifikat-bis-25-08-2027.pdf>: PDF direkt über `curl -fsSL | pdftotext -layout - -` gelesen, nachdem webfetch PDF nicht unterstützen konnte. PÜG-Deckblatt nennt Firma, Verwaltungssitz **und Standort** Otto-Hahn-Straße 26, 89231 Neu-Ulm; Sammeln, Befördern, Lagern und Behandeln; Behördenzertifikat **ZZHT004006222006**, Gültigkeit **27.02.2026–25.08.2027**, PÜG K24622. Das PDF bezeichnet sich selbst als Deckblatt, nicht als vollständiges Behördenzertifikat.
- <https://www.russ-entsorgung.de/images/pdf/Russ-KG-EfbV-Zertifikatsanhang-2026.pdf>: ebenfalls direkt als PDF-Text gelesen. Anlagen, u.a. Anlage 13 auf gedruckter Seite 48/53, nennen denselben Betreiber/Standort und ausdrücklich **„Bundesland: BY“**. Wertstoffsortierung, auf Folgeseite u.a. Eisen-/Nichteisenmetalle, Kupfer/Bronze/Messing, Aluminium, Eisen/Stahl, gemischte Metalle, Kabel. Aus der Textliste unmarkierter Formularoptionen keine zusätzlichen zertifizierten Tätigkeiten ableiten.

**README-Einstufung:** P1 + P2 sind zwei voneinander unabhängige, zugelassene Quellen. Registernummer/-kongruenz, laufender Betrieb, standortspezifische Adresse/Telefon/Zeiten und aktuelle Zertifikate erfüllen zudem die Voraussetzungen für die autoritative Betreiberquelle hinsichtlich eigener Standortfakten. Mehrere Betreiber-Unterseiten zählen nicht als unabhängige Quellen; ein auf der Betreiberseite gehostetes Zertifikat wurde hier nicht als zusätzlich unabhängig abgerufene Registerquelle gezählt.

### P4 – Leistungs- und Preisgrenzen

- <https://www.russ-entsorgung.de/abfallentsorgung> verlinkt den eigenen Detailpunkt „Schrott / Metalle“.
- <https://www.russ-entsorgung.de/abfaelle-zur-verwertung> direkt gelesen: zulässig **FE-Schrott und NE-Metall**; ausgeschlossen Elektronikschrott, Ölradiatoren, Gasflaschen, Druckbehälter, Nachtspeicheröfen, Reifen mit Felgen sowie gefährliche/staubige Stoffe. Containergröße auf Kontaktanfrage. Keine Vergütungszusage oder numerischen Preise.
- <https://www.russ-entsorgung.de/service/containerbestellung>: kostenlos ist ausdrücklich das **Angebot**, nicht die Entsorgung. Absetz 3,5–10 m³, Abroll 10–40 m³, Kurz-/Langzeitmiete; BauBag „1 cbm oder 1000kg“. Metall im Abfallartenformular. Keine Tariftabelle.
- <https://www.russ-entsorgung.de/?Itemid=854>: eigener Shoplink aus dem Formular; direkter Abruf jetzt **404**. Bestätigt Nichterreichbarkeit dieser Route, nicht des gesamten Betreiberauftritts und nicht das Fehlen sämtlicher Tarife im Internet.
- <https://www.russ-entsorgung.de/component/content/article/210-testen-sie-unseren-bau-bag-der-big-bag-von-russ-entsorgung?Itemid=858&catid=86>: BauBag-Versand/Abholung; Abholung im Umkreis 20 km um Otto-Hahn-Str. 26, außerhalb individuelle Abstimmung; Bereitstellung im Radius von 7 m zur Straße. Kein Preis. Diese BauBag-Bedingungen nicht auf allgemeinen Metallschrott-Abholservice übertragen.
- <https://www.russ-entsorgung.de/service/entsorgungsanlage>: Betreiber beschreibt Lagerung, Behandlung und Umschlag gemäß BImSchG; kein Ankaufpreis oder Vergütungsmodell.

| Kategorie | Entscheidung | Nicht ableiten |
|---|---|---|
| Ankauf | Keine öffentliche numerische Schrott-Ankaufsrate und keine eindeutige Auszahlungszusage auf den geprüften Seiten verifiziert. | FE-/NE-Annahme, Registerzweck „Handel“ und Zertifizierung beweisen keine Auszahlung. Ebenso wenig ist Ankauf ausgeschlossen. |
| Verkauf | Kein Metallverkaufspreisblatt auf den geprüften Seiten gefunden. BauBag ist ein eigenes Produkt-/Entsorgungsangebot. | Kein Nachweis, dass keinerlei Verkauf stattfindet. |
| Gebühren | Containerangebot auf Anfrage/kostenfrei erstellt; keine numerischen Miet-, Transport- oder Entsorgungsgebühren auf den geprüften HTML-Seiten. Shoproute 404. | Keine kostenlose Containerstellung/Abholung/Entsorgung; keine Suchsnippets als Tarifnachweis. |

Die Arbeitsfassung sollte „Miet-/Entsorgungsgebühren sind nicht veröffentlicht“ durch **„auf den geprüften Seiten keine numerischen Miet-/Entsorgungsgebühren verifiziert“** ersetzen. Auch „keine Verkaufspreisliste“ ist als Such-/Abrufbefund, nicht als umfassend bewiesene Nichtexistenz zu lesen. AGB und sämtliche Annahmekriterien-PDFs wurden nicht vollständig als Tarifquellen geprüft.

## Geschwister, Namensvetter und Betreiberkette

### Interner Dossier-Abgleich

Corpus-Suche nach Russ/Ruß sowie exakten Betreiber-/Domain-/Adressspuren und Recycling Burlafingen/NUR/GWV: kein zweites RUSS-Neu-Ulm-Dossier gefunden. Der relevante Namensvetter ist **`dossiers/by/by-knetzgau-russ.md`**. Dessen Betreiber-Impressum wurde zusätzlich direkt gelesen:

<http://www.schrotthandel-hammerschmidt.de/impressum/>

Es nennt **Thomas Ruß**, Schrott und Altmetall, Ringstr. 33, 97478 Knetzgau, Telefon 09527 4149401, Mobil 0160 94671365, `polo_vw@web.de`. Das ist ein anderer Name/Rechtsträger, anderer Ort und andere Kontakte. **Nicht zusammenführen, nicht als Neu-Ulmer Filiale behandeln, keine Hammerschmidt→RUSS-Nachfolge konstruieren.** Die Inhaltsaussage zählt, nicht der Domainname. Kein Nachweis einer Verbindung zur Neu-Ulmer KG.

### GWV-Verbindung ist keine Verlagerung nach BW

<https://www.russ-entsorgung.de/ueber-uns/die-firma> nennt „Teil der GWV-Gruppe seit Juli 2021“ und seit 1981 bestehende Geschäftstätigkeit; verlinkt AEU, ADU und GWV als Partnernetzwerk. Die GWV-Beteiligung wird von P2 unabhängig gestützt. Partnerlinks allein beweisen keine jeweiligen gesellschaftsrechtlichen Verhältnisse.

Die konkrete BW-Verwechslungsspur wurde zweiseitig geprüft: Die auf <https://www.gwv-remseck.de/> verlinkte Standortdetailseite <https://www.gwv-remseck.de/recycling-ludwigsburg-uber-uns/standorte/ulm> nennt **GWV Ulm, Ernst-Abbe-Straße 20, 89079 Ulm**, Telefon 07141 2953-23, Verwaltung Remseck. Damit existiert eine separate BW-Betriebsstätte im Gruppenumfeld; sie ist **nicht** Russ Neu-Ulm an Otto-Hahn-Straße 26. Die GWV-Seite zeigt sechs Standortdetail-Links; nur der für die konkrete Ulm-Verwechslung relevante Detail-Link wurde hier geprüft. Keine vollständige GWV-Filialinventur oder Standortbereinigung wird behauptet.

Empfehlung: Im RUSS-Dossier bei Bedarf als Abgrenzung vermerken, keine fremde Standortadresse übernehmen. Keine Schließung, Verschmelzung oder Umbenennung des laufenden RUSS-Betriebs zur GWV GmbH aus der Gruppenzugehörigkeit ableiten. Recycling Burlafingen ist laut P2 ein früherer Name derselben HRA-13628-KG, kein neu anzulegendes Geschwisterdossier.

## Handoff an Owner

1. Bestehenden Text nicht als Beleg einer Landesbereinigung präsentieren; er belässt `state: BW` unverändert. Für reine Rechercheintegration ist das kein neu eingeführter Fehler, für einen abgeschlossenen Bereinigungsauftrag jedoch offen.
2. Neue Timeline-Korrektur mit P1/P2/P3/P4 und Preisgrenzen ergänzen; Slug/alte Historie bewahren. Belegte Landeskorrektur **BY** und ggf. strukturierte Adresse/Kontakte freigeben, nicht aus Seed-Land einen Umzug oder neue Identität ableiten.
3. Kein automatischer `aktiv`-Ankaufstatus, keine Zahlenpreise/Gratis-Dienste, kein GEO ohne eigenen Koordinatenbeleg. Die Typfrage (`schrotthaendler` versus Entsorger) getrennt von der sicheren Standortkorrektur behandeln.
4. Vor Redeploy den wirklichen DB-Zustand read-only prüfen: Leere Adressfelder können laut `docs/trader-seed.md` bei geändertem Seed gespeicherte Adressen leeren; fehlendes GEO erhält vorhandene DB-Koordinaten und bestätigt sie nicht. Hier wurde kein Produktionszustand behauptet.
5. Owner führt Seed-/Integrationsgate und spätere Produktionskontrolle aus. Dieses Review ist Quellenprüfung und Entscheidungsvorlage, keine Test-/Rolloutfreigabe.

Recherchegrenzen: allgemeine Websuche scheiterte mit HTTP 401; dies ist kein Negativbeweis zur Existenz weiterer Firmen. Die oben aufgeführten Betreiberseiten und Northdata-Detailseite wurden dennoch direkt erfolgreich geprüft. Keine Aggregatoren als Evidenz verwendet; keine Verifikationsbehauptung für nicht gelesene Registeroriginale oder alle GWV-Filialen.
