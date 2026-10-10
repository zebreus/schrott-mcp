# Owner-Gate 10.10.2026 – Welle D Mitte/Nordost

## Owner-Abnahme nach Main-Übernahme

Alle 20 Main-Diffs vollständig gelesen. Hofmann-Gegenbeleg, Klawes-/Klinkow-
Rückstufungen und neue Martins-/JK-Leads umgesetzt; kein Hofmann-Ortswechsel,
keine Kontakt-/Zeit-/Preisfüllung. Die Freigabe betrifft diese Notizdeltas und
zwei belegqualitätsbedingte Rückstufungen, nicht die ungeklärten Legacyfelder.
Statischer Gate: vollständige alte Timeline erhalten, Slugs/GEO unverändert,
JSON lesbar, keine Emdash-Platzhalter; Frontmatter ausschließlich die zwei
`aktiv`→`pruefung`-Änderungen. Neun Seed-gefilterte Tests bestanden, darunter
der neue Kernschrott-Seed-/Persistenztest; ausgewählte Dateien vor/nach Lauf
hashgleich. Ein erster Owner-Hilfsbefehl hatte den Test irrtümlich pro Datei
wiederholt und lief ins Zeitlimit; der korrigierte einmalige Lauf endete grün.
Lokaler Main-Commit freigegeben. Push und Redeploy bleiben wegen fehlender
GitHub-Anmeldung offen; keine Produktionsdatenbank beschrieben.

## Main-Vorbereitung nach Ownerentscheidung (10.10.2026)

**Genau die 20 unten aufgeführten Dossier-Deltas sind im Main `/root/Documents/schrott-mcp` per gezielten Patch vorbereitet. Kein Merge-/Test-/Deploymentpass und keine pauschale Legacyfreigabe.** Die nachfolgenden ursprünglichen Prüfentscheidungen beziehen sich auf den unveränderten Quellworktree; dieser Abschnitt dokumentiert den neueren Main-Stand.

- Vor den Patches alle 20 Main-Zieldossiers gegen Baseline `072cf25` gelesen: dort keine Abweichungen seit Baseline. Bestehende Überblickstexte in den sechs unveränderten Überblick-Dossiers erhalten; nur 13 Platzhalter ersetzt und die beanstandete Schneider/Malessa-Aussage gezielt korrigiert. Keine Ganzdatei aus altem Stand kopiert; fremde Änderungen/Tranchen nicht angefasst.
- Sämtliche 09.10.-Timeline-Deltas aus der Quelle übernommen. Hofmann, Klawes, Klinkow, Martins und JK zusätzlich um datierte 10.10.-Gegenprüfung/Ownerentscheidung ergänzt. Die historischen 09.10.-Aussagen „Frontmatter unverändert“/„Aktiv-Seedstatus unverändert“ bleiben als damaliger Befund erhalten und werden durch die neueren Ownerkorrekturen ausdrücklich überholt.
- **Hofmann:** echter Footer-Konflikt (Impressum/Privatkunden Mo–Do bis 16:30 versus Detailseite bis 16:00) im Überblick und neuer Timeline dokumentiert; keinerlei Öffnungszeiten-, State-, Adress-, PLZ- oder Kontakt-Fill. BB→SN und Kontaktdatenzuordnung bleiben konkretes separates Identitätsgate, diesmal ausdrücklich nicht ausgeführt; Slug unverändert.
- **Klawes und Klinkow:** ausschließlich `status: aktiv` → `status: pruefung` als Frontmatter-Korrektur, mit datierter Beleglücke und ausdrücklichem Nicht-Schließungsurteil. Klinkow-Kommune live, Kia ausschließlich Suchauszug und Social-Prüfziele samt Abrufgrenzen dokumentiert; kein Aktivpass.
- **Martins:** Creditreform-ID 3330286979, Suchauszug versus direkter 403 getrennt dokumentiert, Betreiberkette offen; keine Fills.
- **JK:** konkret benannter BA-Arbeitgeber-Suchlead Jennifer Klöpper dokumentiert. Keine erfundene individuelle Stellen-URL und kein behaupteter individueller Vollabruf; Originaldatensatz bleibt Prüfziel. Creditreform erneut 403, keine Fills.

Rein lesende Abschlussinventur gegen `072cf25` (keine Tests/Builds): **20/20 Slugs stabil, 20/20 alte Timeline-Abschnitte als exakter unveränderter Dateisuffix erhalten, 20/20 vollständige 09.10.-Quell-Timeline-Deltas enthalten.** Frontmatter bei 18/20 bytegleich zur Baseline; bei 2/20 exakt nur der autorisierte Statuswechsel. Zieldossier-Diff: 20 Dateien, 145 hinzugefügte/16 entfernte Zeilen; entfernter Inhalt sind Überblick-Platzhalter, die gezielt korrigierte Überblicksaussage und die zwei Aktivwerte, keine historischen Timeline-Zeilen.

In dieser Vorbereitung ausschließlich die 20 Zieldossiers und diesen Main-Bericht gepatcht. Quelle `/root/Documents/schrott-mcp-deploy` ausschließlich gelesen. **Keine Tests, Builds, Commits, Deployments oder DB-Zugriffe.** Owner prüft Diff und Seedtests danach; verbleibende Identitäts-/Quellenlücken unten bleiben offen.

## Ursprüngliches Urteil zum Quellworktree (vor Main-Vorbereitung)

**Keine pauschale Integrations-/Legacyfreigabe.** Die 20 Dossier-Deltas sind überwiegend brauchbare, ausdrücklich vorsichtige Recherche-/Einordnungsnotizen. 19 Deltas können als solche integriert werden, **ohne damit bestehende Frontmatter-Fakten oder Aktivstatus zu bestätigen**. Hofmann benötigt vor der Freigabe eine ergänzende Gegenprüfung/Korrektur zum weiterhin vorhandenen Zeitenwiderspruch. Die bestehenden Aktivwerte bei Klawes und Klinkow bleiben separate Owner-Blocker einer vollständigen Seed-Evidenzfreigabe. Keine neue Preis-/Handlerfreigabe.

Geprüft: `/root/Documents/schrott-mcp-deploy/research/wave-20261009d-mittenordost.md`, dessen genau 20 Zieldossiers und deren Arbeitsbaum-Diffs gegen dortiges `HEAD`; Maßstab: dortige `README.md`, besonders Zeilen 148–182, 200–219. Alle Quell-Dateien nur gelesen. Ausschließlich dieses Check-in geschrieben; keine Builds, Tests, Commits, Deployments oder DB-Zugriffe.

## Ursprünglicher Diff-/Seed-Audit im Quellworktree

- 20/20 Zieldossiers besitzen ein Delta vom 09.10.; die Frontmatter-Blöcke sind gegenüber `HEAD` unverändert (Python-Lesevergleich).
- 20/20 enthalten sämtliche früheren Timeline-Zeilen weiterhin (zeilenweiser Inhaltsvergleich des alten Timeline-Abschnitts). Kein Historyverlust festgestellt.
- 13 Überblick-Platzhalter ersetzt; Schneider/Malessa-Überblick zurecht von vermeintlich bewiesener Nachfolge/Anerkennung auf Quellenkritik zurückgenommen.
- **Frontmatter unverändert ist kein Evidenzpass.** Nichtleere Altadressen, Telefone, Namen und Typisierungen dürfen nicht als neu geprüft ausgegeben werden. Besonders `status: aktiv` bei Klawes/Klinkow ist nicht durch diese Welle neu bestätigt.
- README Zeilen 98–109: Timeline-Bullets werden als `notes` kompiliert. Deshalb sind reine Notizänderungen ebenfalls Seed-Änderungen, nicht bloß interne Kommentare. Alte Fehlaussagen müssen durch eindeutig datierte Gegenbelege eingeordnet bleiben; der spätere Owner-Test ist weiterhin nötig und hier ausdrücklich nicht durchgeführt.

## Eigenständige Live-Gegenprüfung: entscheidende Befunde

### 1. Hofmann: Zeitenkonflikt weiterhin real, Ortskorrektur substanziell belegbar

Am 10.10. direkt gelesen:

- [Standortdetail Hirschfeld](https://www.hofmann-metall.de/standort/hirschfeld/): Bahnhofstraße 5, 08144 Hirschfeld; 037607 5531, 0172 3761211, `hirschfeld@hofmann-metall.de`; Mo–Fr 07–16, Samstag/länger nach Absprache, Pausen 09–09:30 und 12–12:30; Wiedereröffnung für Privat-/Kleingewerbekunden 2004.
- [Impressum](https://www.hofmann-metall.de/impressum/): Hofmann Metall GmbH, Äußere Dresdner Straße 80, Zwickau, HRB 5475 Chemnitz. **Dortiger Hirschfeld-Footer nennt weiterhin Mo–Do 07–16:30, Fr 07–16 sowie `info@hofmann-metall.de`.** Derselbe Footer steht auf der ebenfalls live gelesenen [Privat-/Kleingewerbeseite](https://www.hofmann-metall.de/privat-kleingewerbekunden/).
- [Northdata HRB 5475](https://www.northdata.de/Hofmann%20Metall%20GmbH,%20Zwickau/Amtsgericht%20Chemnitz%20HRB%205475): identischer Name, Register, Hauptsitz, Schrott-/Metallhandel; Publikationen 17.03.2026 und 03.08.2026. Betreiber-Auftritt aktuell: der Abruf der alten Feiertags-News-URL lieferte Startseiteninhalt mit News vom 06.10.2026 und Marktwerten vom 08.10.; **nicht als erneuten Abruf der alten Feiertagsmeldung zählen**.

**Korrektur vor Deltafreigabe:** Die Aussage „im aktuellen extrahierten Footer keine abweichenden Zeiten mehr“ ist nur für den einzelnen Detailseitenabruf richtig. Daraus keine Entwarnung für den gesamten Auftritt machen. Datierte Ergänzung: „10.10. Impressum/Privatkunden-Footer reproduziert weiterhin 16:30 statt 16:00 Mo–Do; Öffnungszeiten-Fill gesperrt.“ Alte Zeile nicht löschen.

**Konkrete Owner-Option:** Nach dokumentierter Entscheidung, dass der schon vorhandene Name + Bahnhofstraße 5 den sächsischen Betriebsteil meint (kein neuer Rechtsträger/keine Nachfolgekette), ist `state: SN`, `postcode: '08144'`, normalisierte Straße sowie Betreiberwebsite/Bürotelefon/Standort-Mail sachlich freigabefähig. Impressum, Northdata-Kongruenz, per-site Details und aktuelle Inhalte erfüllen die Betreiber-Ausnahme für eigene Filialfakten. Slug `bb-hirschfeld-hofmann-metall` unverändert lassen; Datei-/State-Handhabung anhand Seedsemantik durch Owner prüfen. Keine stillschweigende automatische BB→SN-Integration in dieser Prüfung. Zeiten bleiben leer; keine Geokoordinaten aus Adresskenntnis, kein standortspezifischer Zertifizierungs-Fill ohne Anlage.

Privatkunden-Seite schließt Autokarossen, Elektronikschrott, Weiße Ware und Katalysatoren ausdrücklich aus. Keine Autoverwertung ergänzen. BDSV-/Börsenwerte sind keine Händler-Ankaufpreise; keine Preisübernahme.

### 2. Martins: neuer qualifizierter Quellenkandidat, aber kein fertiger Zweitbeleg

[Impressum](https://www.schrott-martins.de/impressum/) live: Stefanie Martins, Am Hafen 10a, 19386 Lübz, 038731-22406, `info@schrott-martins.de`; HRB **XYZ**, USt-ID **DE XYZ**. Saisonzeiten wie im Delta. [Schrott-/Metallseite](https://www.schrott-martins.de/portfolio/schrott-metallhandel-luebz/) live bestätigt Materialliste, Ausschlüsse und Preisbildung auf Anfrage/nach Gutbefund, keine Zahlen.

Eigene Suche `"Martins" "Lübz" "Stefanie"` findet zusätzlich [Creditreform: Stefanie Martins Containerdienst und Recycling, ID 3330286979](https://firmeneintrag.creditreform.de/19386/3330286979/STEFANIE_MARTINS_CONTAINERDIENST_UND_RECYCLING); Suchauszug nennt Lübz, 038731 22406 und `schrott-martins.de`. Direktabruf **403**. Damit ist die Wellen-Discovery ergänzbar: Es gibt einen vielversprechenden erlaubten zweiten Quellenkandidaten. **Keine vollständige Register-/Adressbestätigung durch einen gesperrten Detailabruf behaupten.** Ein vollständig gelesener Creditreform-/Gewerbenachweis könnte die Betreiberwebsite zum Zweiquellenbeleg ergänzen; Heiko→Stefanie-Nachfolge ist davon getrennt zu beweisen. Fehlende echte HRB ist bei einem Einzelunternehmen nicht automatisch Gewerbenegativbeleg, verhindert hier aber die spezielle README-Ausnahme. Kontakt-/Adress-Fills derzeit nicht freigegeben; Notizen ja.

### 3. JK: Arbeitgebername stärker belegt als nur Initialen, Annahme weiter offen

Creditreform-Ziel [ID 3170285894](https://firmeneintrag.creditreform.de/39317/3170285894/JENNIFER_KLOEPPER_SCHROTTHANDEL_CONTAINERDIENST) direkt erneut **403**. Eigene Suche `"Jennifer Klöpper" "Schrotthandel"` findet [BA-Jobsuche Stendal](https://www.arbeitsagentur.de/jobsuche/suche?angebotsart=1&berufsfeld=B%C3%BCro%20und%20Sekretariat&suchbereich=jobs&wo=Stendal): Suchauszug nennt „JK Schrotthandel Ferchland Jennifer Klöpper“, Bürokraft und Arbeitsort Ferchland. Das ist ein zusätzlicher Arbeitgeber-Identitätslead, keine bloße Initialendeutung. **Nur Suchauszug gelesen; kein individueller Arbeitgeber-/Stellen-Originaldatensatz.** BA-Portal außerdem nicht automatisch zweite Quelle aus der engen README-Liste; Arbeitgeberangaben nicht mit amtlichem Gewerbenachweis verwechseln. Keine Mühlenweg-Adresse/Telefon/Aktivität/öffentliche Annahme freigeben.

### 4. Klinkow: kommunales Unternehmensverzeichnis, Social-/Herstellerleads nachholen

[Stadtprofil](https://www.prenzlau.eu/cms/detail.php/brandenburg_06.c.835762.de) live stimmt mit Delta überein: Autoverwertung Klinkow GbR, Cornell Ramm, Am Quillow 38, 03984 5701, Mo–Fr 07–12/12:30–16, Altfahrzeugannahme/Teileverkauf. Seite ist ausdrücklich **Unternehmensverzeichnis**, kein ausgewiesener Gewerberegisterauszug. Führt zudem Facebook „Autoverwertung Klinkow GbR“ und Instagram `av_klinkow` als konkrete Betreiber-Social-Prüfziele. [Instagram](https://www.instagram.com/av_klinkow/) direkt lieferte nur Bild-/Login-Inhalt ohne verwertbare Profilidentität; kein zweiter Betreiberbeleg. `https://www.autoverwertung-klinkow.de/` (aus der E-Mail-Domain abgeleitetes Prüfziel) Transportfehler, kein Betriebsaufgabebeleg.

Eigene Suche findet [Kia-Rücknahmestellen März 2026](https://www.kia.com/content/dam/kwcms/kme/de/de/assets/contents/service/recycling/Kia_Uebersicht-Annahmestellen_Maerz-2026.pdf): **Suchauszug** nennt GbR, Am Quillow 38, 17291 Klinkow, 03984/5701, Herr & Frau Ramm, `autoverwertung-klinkow@t-online.de`. Dazu historische Ford-/Honda-/Mazda-Verzeichnisse. Diese sind neue historische/vertragliche Standortindizien, **nicht individuell gelesene aktuelle GESA-Anerkennung und nicht automatisch README-Zweitbelege**. Kein `aktiv`-Pass daraus. Owner sollte Betreiber-Social/Gewerberegister/GESA gezielt nachholen, nicht aus der GmbH Nachfolge konstruieren.

[Ramm-GmbH-Downloads](https://www.mcr-ramm.de/unsere-zertifikate) zusätzlich **HTML** live geprüft: beide Preislisten stehen in bloßen `<p>`-Texten ohne `<a href>`; nur das Zertifikat hat einen PDF-Link. Damit nicht nur Markdown-Extraktionsgrenze: im ausgelieferten HTML keine Preisdatei an den Überschriften. Stettiner Straße 79/03984 801942 im GmbH-Footer sind keine GbR-Fakten.

### 5. Dahm/Bohnsack und Schneider/Malessa: Zurückhaltung bestätigt

[Northdata Dahm/Bohnsack HRB 11108](https://www.northdata.de/Dahm%20%26%20Bohnsack%20Handels-%20und%20Recycling%20GmbH,%20Neustadt%20(Dosse)/Amtsgericht%20Neuruppin%20HRB%2011108) live bestätigt GmbH, vorheriges HRB 26488 Potsdam, Gewerbegebiet Ost 8, Ortsbezug Heiligengrabe/Falkensee, Abschluss 2024 publiziert 12.03.2026. Keine sichtbare GbR→GmbH-Kette und keine aktuelle Annahmestelle Wittstocker Chaussee 1. Eigene Suche findet weiterhin ausschließlich Verzeichnisleads zum Zielhof, teils Hausnummer 2 statt 1. Nicht umziehen, umbenennen, schließen oder dieses Adressfragment normalisieren.

[Schneider/Malessa-Registerspiegel](https://handelsregister.live/hra/hessen/ag-darmstadt/83123/schneider-malessa-e-k-inhaberin-bettina-ringhof) live: Titel e.K./Bettina Ringhof, sichtbarer chronologischer Inhalt aber OHG 2006 und Austritt Andrea Neubert/Adresse 2018, Abrufdatum 04.05.2024. Widerspruch tatsächlich reproduziert. **Nicht titelbasierte Nachfolge freigeben.** Originalauszug und individueller GESA-Datensatz fehlen weiterhin. Aktueller Frontmatter-Name ist ein ungelöster Alt-Fill, nicht mit dieser Welle neu bestätigt.

## Einzelentscheidungen über alle 20 Deltas

„Notizen frei“ bedeutet nur Integration des kritischen Deltas; **keine** automatische Freigabe der Seed-Legacyfelder. „Suche ohne Zielbeleg“ bezeichnet die begrenzte eigene Websuche am 10.10., nie einen Register-/Existenznegativbeleg.

| Dossier-Slug | Entscheidung / konkrete Restlücke |
| --- | --- |
| he-reichelsheim-odw-walther-andreas | Notizen frei. Eigene Suche Andreas/Walther/Reichelsheim liefert keinen Zielbeleg. Nicht eingetragener Einzelunternehmer bleibt möglich; Ortsteil und Odenwald-Gewerbeidentität fehlen. Keine Kontaktdaten. |
| he-zwingenberg-64673-gerhardt-august | Notizen frei. Eigene Suche Gerhardt August/Zwingenberg kein Betreiberbeleg; kommunaler Wertstoffhof Wiesenstraße ist ein anderer Betrieb. Hohl 27a und Schreibweise offen. |
| bb-hirschfeld-hofmann-metall | **Delta erst mit datiertem Footer-Gegenbeleg frei.** BB/SN-Ownerkorrektur konkret möglich wie oben; Zeiten gesperrt. |
| he-neu-isenburg-metall-recycling-winterstein | Notizen frei; zusätzliche Discovery notieren: [FirmenZone](https://firmenzone.com/stadt/neu-isenburg) Suchauszug nennt An den Grundwiesen 65/06102 39093. Nur Aggregatorlead, kein Fill. Nicht Alsfeld/Wiesbaden oder Knettenbrech-Gurdulic zuordnen. |
| st-dessau-rolau-decker | Notizen frei. Eigene Decker/Dessau/Neekener-Suche kein Zielbeleg. 9B/31, Inhaber, Verhältnis Autoservice/Demontage und GESA offen. |
| he-schaafheim-64850-he-containerdienst-sell | Notizen frei. Eigene Sell/Schaafheim/Ziegelei-Suche liefert fremde Nachbarbetriebe, keinen Sell-Betreiberbeleg. 11/11a und Rudolf/Charlie/Sascha nicht vermischen; Legacy-Telefon nicht freigegeben. |
| be-pankow-markopol | Notizen frei. Eigene Suche findet fach-/ortsfremde Markopol/Monopol, keinen Zielbetreiber. Schönhauser Straße/Allee sauber getrennt halten; Inhaber/Hofbetrieb offen. |
| he-dillenburg-herborn-35683-schafer-und-scheffel-sds | Notizen frei. Eigene Schäfer/Scheffel/Schrott-Suche kein Zielbeleg. Original-Seedname/SDS und eindeutiger Gewerbeort fehlen. |
| bb-prenzlau-ot-klinkow-ramm-klinkow | Notizen frei, **Aktiv-Legacy nicht frei**. Kommune live, Social/Herstellerleads oben; aktuelle Anerkennung und README-konformer Identitätsbeleg fehlen. Keine GmbH-Preise übertragen. |
| he-viernheim-schneider-malessa | Notizen-/Überblickskorrektur frei. Registerspiegel-Widerspruch live bestätigt; bestehender Name/Betreiberkette sowie GESA separat gesperrt. Kein Schließungsurteil aus Parking. |
| bb-potsdam-klawes-rainer | Notizen frei, **Aktiv-Legacy nicht frei**. Eigene Klawes/Potsdam/Schrott-Suche kein Betreiberbeleg. Owner: `pruefung` mit datierter Evidenzlücke statt unbestätigtem Aktivitätsversprechen erwägen; keine Schließung, Portal-Root bleibt keine Website. |
| bb-heiligengrabe-b-wittstock-dahm-bohnsack-schrotthandel | Notizen frei. Northdata-GmbH live bestätigt, Seed-GbR-Kette/Zielhof offen. Keine Adress-/Telefon- oder Nachfolgefreigabe. |
| he-frankfurt-60386-autoverschrottung-cassellastr | Notizen frei. Eigene Telefonnummernsuche kein Zielbetreiberbeleg. Positiver alter Portalbefund widerlegt nur die damalige Trefferlosigkeit, nicht Identitätslücke. 31/Telefon keine aktuelle Betriebsstättenbestätigung. |
| mv-lubz-19386-martins-schrotthandel-containerdienst | Notizen frei, keine Frontmatter-Fills. Betreiber live; neuer Creditreform-ID-Lead 3330286979 ergänzbar, 403 und Heiko-Kette ausdrücklich offen. |
| he-frankfurt-dornbusch-metallwelt | Notizen frei. Eigene Suche liefert metallwelt.eu-Dachprofilanbieter mit Sauerland-Bezug, keinen Friedleben-Zielbeleg. Legacy-Straße/Mobilnummer nicht neu bestätigt. |
| he-eschborn-65760-kuhn-heinrich | Notizen frei. Eigene Heinrich-Kuhn/Eschborn-Suche kein Zielbeleg. Niddastraße/Telefon Legacy; `sonstige` nicht spekulativ in Schrotthandel ändern. |
| st-ferchland-jk-schrotthandel-ferchland | Notizen frei. Creditreform 403; BA-Arbeitgeber-Suchlead ergänzbar, aber Adresse/öffentliche Annahme/Preise nicht freigegeben. |
| st-genthin-knitter-tobias | Notizen frei. Eigene Tobias-Knitter/Genthin/Schrott-Suche kein Zielbeleg. Ortsteil Dorfstraße 15 und Gewerbeidentität offen; Privatpersonen nicht als Unternehmensquellen verwenden. |
| he-babenhausen-64832-ries-peter | Notizen frei. Eigene Suche reproduziert [Das Örtliche](https://www.dasoertliche.de/Themen/Schrotth%C3%A4ndler/Babenhausen-Hess.html): Feldbergstraße 10/06073 61957, **Hergershausen** als zusätzlicher Ortsteil-Lead. Zwei Portale bleiben null qualifizierte Belege, kein Fill/Aktivpass. |
| be-friedrichshain-kreuzberg-schrott-und-altmetall-abholung | Notizen frei. Eigene Suche reproduziert [Gelbe-Seiten-Liste](https://www.gelbeseiten.de/branchen/schrotth%C3%A4ndler/berlin) mit Möckernstraße 139/0155 10713536, aber keine Betreiberidentität. Keine Hof-/kostenlose Abholungsfreigabe. |

## Grenzen und ursprüngliche Owner-To-dos

Die Suchintegration liefert bei vielen exakt formulierten deutschen Zielabfragen überwiegend sachfremde Namensartikel/SEO-Seiten. Das begrenzt Discovery stark. Kein geschlossener Betrieb, keine Nichtexistenz und kein vollständig durchsuchtes Gewerberegister daraus ableitbar. Vollständige Primärprüfung aller 20 Identitäten wurde **nicht** erreicht. Erfolgreiche Betreiber-/Register-Liveabrufe, Suchauszüge, blockierte Abrufe und nur aus alten Dossiers übernommene Quellen sind oben getrennt.

**Aktualisierung nach Ownerentscheidung:** Punkt 1 (Footer-Ergänzung) und die `pruefung`-Option aus Punkt 2 sind jetzt ausschließlich im Main vorbereitet, wie im ersten Abschnitt belegt. Hofmanns geografische/Stammdatenentscheidung ist ausdrücklich vertagt. Die folgenden ursprünglichen Punkte sind als Prüf-/Entscheidungshistorie zu lesen; Punkt 3–5 bleiben offen.

1. Hofmann-Notiz um reproduzierten Footer-Gegenbeleg ergänzen; geografische Ownerentscheidung mit stabiler Slug dokumentieren. Keine Zeitenübernahme.
2. Klawes/Klinkow-Aktivwerte ausdrücklich entscheiden: qualifizierten aktuellen Beleg beschaffen oder datiert auf `pruefung` zurückstufen; **nicht** `geschlossen` setzen. Das wäre eine zukünftige Ownerkorrektur, hier nicht ausgeführt.
3. Martins-Creditreform und JK-individuellen Arbeitgeber-/Gewerbenachweis nachholen; Klinkow-Social/GESA gezielt prüfen; Schneider- und Dahm-Nachfolgekette mit Originalereignissen beweisen.
4. Keine numerischen Zielpreise in dieser Prüfung verifiziert: ANKAUF, VERKAUF und GEBÜHREN bleiben getrennte offene Kategorien. Kein Preis-Fill, kein Handler, keine Behauptung „kostenlos“ aus bloßer Annahme.
5. Vor späterer Integration/Deployment Owner-Gate gemäß README einschließlich `cargo test -p schrott-mcp-ingestion seed` durchführen. Diese Prüfung enthält **keinen** technischen Build-/Test-/Produktionspass.
