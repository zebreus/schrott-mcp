# Review Born Metall – 18.10.2026

## Entscheidung für den Owner

**Den vorliegenden Diff nicht unverändert übernehmen.** Die Begründung zur Leerung von `street` („nur Branchenbuch-Lead“, „Registerbestätigung fehlt“) wird durch die heute tatsächlich gelesenen Quellen widerlegt. North Data führt exakt Born Metall GmbH, Amtsgericht Chemnitz HRB 20368, an der Chemnitzer Landstr. 53, 09244 Lichtenau. Der aktuelle GLEIF-Datensatz führt dieselbe Gesellschaft, HRB und Adresse als Rechts- und Hauptsitzadresse. Das belegt eine Gesellschaftsadresse, nicht automatisch einen heute zugänglichen Schrottankaufplatz.

**Exakte empfohlene Feldentscheidung:** Im geprüften Arbeitsstand `street: ''` durch `street: Chemnitzer Landstr. 53` ersetzen, also den vorgeschlagenen Feld-Delta gegenüber HEAD nicht integrieren. Sonst **keine Frontmatter-Änderung**: insbesondere `status: pruefung`, leere Telefon-/Website-/PLZ-/Annahmefelder sowie Name, Ort und Slug unverändert. Kein GEO, keine Schließung und keine Aktivierung ableiten. Der Owner integriert; das Dossier wurde von diesem Review nicht bearbeitet.

**README-Grenze ausdrücklich:** North Data zählt laut README als zulässige Quelle. GLEIF ist eine eigenständige Primärquelle für den LEI-Datensatz, steht aber nicht ausdrücklich in der geschlossenen Quellenliste des README. Bloomberg und GLEIF sind für denselben LEI **keine zwei unabhängigen Belege**. Deshalb hier keine behauptete formal erfüllte Zwei-Quellen-Freigabe für neue Felder oder eine aktive Betriebsstätte. Der Owner muss GLEIF als Registervalidierungsbeleg ausdrücklich einordnen oder zusätzlich Creditreform/Handelsregister tatsächlich lesen. Diese Einschränkung rechtfertigt nicht den nachweislich falschen Text „kein Registerbeleg“. Konservative Integration: bestehende Straße behalten, Aussage auf Gesellschaftsadresse begrenzen, operativen Standort weiterhin prüfen.

## Geprüfter lokaler Diff

Gelesen: README vollständig, Dossier und `git diff HEAD -- dossiers/sn/sn-lichtenau-born-metall.md` am 18.10.2026. Keine AGENTS.md im Workspace gefunden.

- Einziger skalarer Delta: `street: Chemnitzer Landstr. 53` → `street: ''`.
- Überblick wurde auf vermeintlich ausschließlich vorhandenen Stadtbranchenbuch-Lead und bereits entfernte Straße umgestellt.
- Neu angehängt: `### Korrektur 12.10.2026` mit derselben Leerungsbegründung.
- `status: pruefung` war bereits in HEAD vorhanden; die aktuelle Änderung ist **keine** neue Statuskorrektur.

## Zweiseitige Prüfung

### Gesellschaft / Namensgleichheit

Die gelesene North-Data-Unternehmensseite nennt Born Metall GmbH, zusätzlich den früheren Namen Born Hofmann Metall GmbH, HRB 20368 und LEI 391200B8EYWAF2DLPV79. Publikationsüberschriften: Eintragung am 25.08.2003 als Born Hofmann Metall GmbH mit Sitz Hirschfeld; am 15.12.2003 „Firmensitz: Lichtenau · Name: Born Metall GmbH“. Gegenstand: Erfassen, Aufbereiten und Vermarkten von Rohstoffen, insbesondere Eisen-, legierten Stahl- und Nichteisen-Metallschrotten. Die verlinkten Originalpublikationen wurden **nicht** geöffnet; daraus keine vollständig geprüfte Betreiberkette behaupten.

GLEIF führt denselben Namen und HRB, `entity.status: ACTIVE`, `registration.status: ISSUED`, `corroborationLevel: FULLY_CORROBORATED`, Validierung gegen RA000224/HRB 20368. Das stützt, dass Name + Lichtenau keine bloße Branchenbuch-Fiktion sind. `ACTIVE` ist ein Gesellschafts-/LEI-Befund, kein Nachweis heutiger Annahme oder Öffnungszeiten.

### Adresse / anderer Betreiber

North Data nennt Chemnitzer Landstr. 53, D-09244 Lichtenau. GLEIF nennt Chemnitzer Landstraße 53, Lichtenau OT Auerswalde, 09244, sowohl als `legalAddress` als auch `headquartersAddress`. Aktualisierung des Datensatzes: 21.01.2026; nächste Erneuerung: 17.03.2027; Golden-Copy-Veröffentlichung: 17.10.2026. Letzteres ist **kein** Datum einer Vor-Ort- oder Adressprüfung.

Die adressbezogene North-Data-URL lieferte erneut die Born-Unternehmensseite, **keine vollständige Liste aller ansässigen Unternehmen**. Es wurde kein belastbarer anderer aktueller Betreiber an Nr. 53 gefunden; das beweist nicht dessen Abwesenheit. Kein sachlicher Anlass, einen anderen Born-Betrieb zuzuordnen, die Gesellschaft zu splitten oder den Standort als geschlossen darzustellen. Heutige Betriebsstätte/Annahme und mögliche Mitnutzer bleiben offen.

### Telefon / Betreiberwebsite

Die gelesene schrottplatz-info-Seite führt 0371 4 58 46 10 mit Born und derselben Adresse, ist aber nach README nur ein Lead. Ihr Link „zur Homepage“ enthält eine PHP-Fehlermeldung statt einer nutzbaren Betreiber-URL. Keine verifizierte Betreiberwebsite/kein Impressum gelesen.

Creditreform-Suchtreffer nennt eine abweichende Nummer +49 37208 893000; Yelp-Suchtreffer nennt 0371 417847. Beide Nummern **nicht übernehmen**: Creditreform-Inhalt war beim Direktabruf 403, Yelp wurde nicht geöffnet. Die historische Nummer 03714584610 bleibt ausschließlich ein Lead; keine Funktionsprüfung/kein Anruf durchgeführt.

## Nötige Text- und Datierungskorrekturen

1. Überblick nicht auf „nur ein Stadtbranchenbuch-Lead“ reduzieren. Vorschlag:

   > Born Metall GmbH (Amtsgericht Chemnitz HRB 20368) wird bei North Data und im GLEIF-Datensatz an der Chemnitzer Landstr. 53, 09244 Lichtenau OT Auerswalde, geführt. Damit ist die Gesellschaftsadresse gestützt; ein heute betriebener, zugänglicher Schrottankaufplatz und aktuelle Annahmebedingungen sind nicht bestätigt. Eine verifizierte Betreiberwebsite und belastbare aktuelle Telefonnummer fehlen in der Recherche. `status: pruefung` bleibt bestehen; ein Schließungsbefund liegt nicht vor. Sorten und Annahme bleiben historische Seed-/Verzeichnisangaben.

2. Neue Korrektur auf den **tatsächlichen Recherche-/Integrationszeitpunkt 18.10.2026** datieren. Das im uncommitteten Diff verwendete 12.10.2026 ist durch diesen Review nicht belegt; nicht rückdatieren. Falls der Owner für den 12.10. eigene Evidenz besitzt, historische Handlung und heutige Gegenprüfung getrennt dokumentieren.
3. Historische Notizen 01.10. und 07.10. als damaligen Kenntnisstand erhalten, aber am 18.10. ausdrücklich überholen. Nicht die damalige Recherche nachträglich als aktuell ausgeben.
4. Den neu hinzugefügten, unbelegten Korrekturabsatz zur Leerung durch einen ehrlich datierten Gegenprüfungsabsatz ersetzen, etwa:

   > [Korrektur 18.10.2026: Der bisherige Recherchebefund „nur Branchenbuch-Lead, kein Registerbeleg“ ist überholt. North Data nennt Born Metall GmbH, HRB 20368, an Chemnitzer Landstr. 53; GLEIF führt dieselbe Gesellschaft mit dieser Rechts-/Hauptsitzadresse (Datensatz aktualisiert 21.01.2026). Die vorgeschlagene Leerung von street wird nicht übernommen. Gesellschaftsadresse ist nicht gleich aktuell bestätigter Annahmestandort; Status pruefung und leere Kontakt-/Annahmefelder bleiben unverändert. Historische Telefonnummer 0371 4584610 nur Verzeichnislead, keine Schließung und keine Preise abgeleitet. Quellen: https://www.northdata.de/Born%20Metall%20GmbH,%20Lichtenau/Amtsgericht%20Chemnitz%20HRB%2020368 ; https://api.gleif.org/api/v1/lei-records/391200B8EYWAF2DLPV79 ; historische Telefonspur: https://www.schrottplatz-info.de/schrottplatz/Lichtenau/BORN-METALL-GmbH (Aggregator).]

5. Preistext ohne Vollständigkeitsbehauptung: „In den gelesenen Quellen keine aktuellen numerischen Ankaufspreise, Verkaufspreise oder Gebühren gefunden; aktuelle Annahme unbestätigt.“ Der bisherige Satz „keine Preisliste“ darf nicht als Nachweis verstanden werden, es gebe nirgends eine.
6. GEO bleibt gesperrt; ehrlicher Text: „Keine Geokodierung im Rahmen dieses Reviews; Gesellschaftsadresse nicht als bestätigten Annahmepunkt behandeln.“

## Quellenprotokoll – tatsächlich gelesen versus Leads

### Erfolgreich vollständig als Text/JSON abgerufen, 18.10.2026

- https://www.northdata.de/Born%20Metall%20GmbH,%20Lichtenau/Amtsgericht%20Chemnitz%20HRB%2020368 — zulässige Registerauswertung, Name/HRB/Adresse/Gegenstand und Publikationsüberschriften gelesen; keine Originalregisterauszüge gelesen.
- https://api.gleif.org/api/v1/lei-records/391200B8EYWAF2DLPV79 — Primärdatensatz des LEI-Systems; genaue Felder/Datierungen oben. Keine Betreiberwebsite, keine Bestätigung von Publikumsverkehr.
- https://lei.bloomberg.com/leis/view/391200B8EYWAF2DLPV79 — dieselben LEI-Adress-/Validierungsdaten; redundant zum GLEIF-Datensatz, nicht unabhängig zählen.
- https://www.northdata.de/Chemnitzer%20Landstr%C2%B7%2053,%20D-09244%20Lichtenau — lieferte Born-Unternehmensinhalt; keine zusätzliche unabhängige Quelle, keine vollständige Adressbelegung.
- https://www.schrottplatz-info.de/schrottplatz/Lichtenau/BORN-METALL-GmbH — **Aggregator/Lead**, trotz erfolgreichem Lesen kein zulässiger Standort-/Telefonbeleg; Leistungen und Öffnungszeiten dort ausdrücklich nicht erfasst.

### Direktabruf gescheitert – nicht als gelesen ausgeben

- https://firmeneintrag.creditreform.de/09244/3030197671/BORN_METALL_GMBH — 403. Suchtreffer zeigte Name/HRB bzw. Adresse und +49 37208 893000; kein gelesener Creditreform-Beleg.
- https://lichtenau-bei-chemnitz-sachsen.stadtbranchenbuch.com/F/firmenverzeichnis.html — 403. Suchtreffer zeigte Born, Nr. 53, 0371 4584610; nur Aggregator-Lead.

### Nur Suchtreffer, nicht geöffnet – Leads, keine Belege

- https://m.yelp.com/biz/born-metall-lichtenau — Adresse, abweichende Nummer 0371 417847.
- https://lichtenau.branchen-info.net/born-metall-gmbh/4330762/ — Adresse und 0371 4584610.
- https://www.tellows.de/num/03714584610 — Label Schrott BORN METALL GmbH.
- https://lei.report/LEI/391200B8EYWAF2DLPV79/ — weiterer LEI-Spiegel; nicht unabhängig zählen.
- https://www.schrottplatz.org/lichtenau-sachsen/schrott-born-metall-gmbh-aYEPz6.html — Verzeichnislead.

Suchläufe: `"Born Metall" "Lichtenau"`, `"Chemnitzer Landstr" "53" "Metall"`, `"03714584610"`, `"Born Metall GmbH" website Impressum`. Zwei spätere Vertiefungssuchen zu ausgeschriebener Straße bzw. 037208 scheiterten mit Websearch HTTP 401; daraus keinen Negativbefund ableiten. Recherche war nicht erschöpfend.

## Ausführungsgrenze

Nur diese Reviewdatei geschrieben. Dossier ausschließlich gelesen, Produktionsdatenbanken weder geöffnet noch verändert, keine GEO-Abfrage/-Berechnung, kein Build/Seed/Deployment/Commit. Integrations- und Testgate beim Owner.
