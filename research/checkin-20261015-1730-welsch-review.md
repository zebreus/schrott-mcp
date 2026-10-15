# Welsch Bornheim – Review der vorbestehenden Änderung

Stand: 15.10.2026. Gegenstand: `git diff HEAD -- dossiers/nw/nw-bornheim-welsch.md`, HEAD bei Prüfung `db31467a81185019ceeddf804547b6ccdb7e5a4b`. Nur dieser Bericht wurde erstellt; keine Dossieränderung, kein Commit, kein DB-Zugriff.

## Begrenzte Integrationsentscheidung

**Als dokumentarische Unsicherheitskennzeichnung integrierbar; ausdrücklich keine Freigabe als verifizierter Händler-/Standortdatensatz.** Der Diff ersetzt den leeren Überblick und ergänzt einen Rechercheabschnitt vom 12.10.; er verändert kein Frontmatterfeld und entfernt keine Historie. Seine Kernaussage (nur Verzeichnis-Leads, Identität/Adresse offen, keine Übernahme von Zeiten oder Preisen) ist durch die unten direkt geprüften Seiten nachvollziehbar.

Die Integration löst weder die Betreiberidentität noch Straufsberg 2 versus Kerpengasse 2. Insbesondere sind die bereits nichtleeren Felder **nicht durch ihr Fortbestehen bestätigt**. `status: pruefung` und `website_status: unbekannt` müssen bleiben; kein Adresswechsel, kein neuer Betreiber, keine Zusammenführung, keine Schließungsfeststellung. Auch eine vollständige Neuerfassung desselben Händlers wäre ohne zusätzliche Evidenz nicht gerechtfertigt.

Für eine spätere Dossierergänzung empfohlen, hier nicht ausgeführt: datierter Review-Nachtrag vom 15.10. mit dem gleichlautenden Telefon auf beiden Adressleads, dem fremden Anbieter an Kerpengasse 2 und der ausdrücklichen Entwertung des historischen Wortes „Doppelbeleg“. Nicht den neuen Abruf auf den 12.10. rückdatieren: Ob an diesem Tag tatsächlich recherchiert wurde, belegt dieser Review nicht.

## Maßstab und Historie

README, Abschnitte „Feedback triage“ und „Operator rename/merge chains“: Aggregatoren sind Leads, keine Evidenz; grundsätzlich zwei unabhängige zulässige Quellen. Die Ausnahme einer autoritativen Betreiberquelle verlangt u. a. registerkongruente Betreiberidentität, aktuelle standortspezifische Seiten und Registerangabe. Nichts davon ist für Welsch erfüllt. Beide Adressen müssen eigenständig geprüft werden; ein fremdes Impressum darf keine falsche Betreiberzuordnung erzeugen.

`git log` und `git show 71f45681 -- dossiers/nw/nw-bornheim-welsch.md` geprüft: Der 03.10.-Commit setzte lediglich `website_status` von leer auf `unbekannt` und ergänzte den bereits vorhandenen Quellenvorbehalt. Die Timeline vom 30.09. nennt Adresse/Telefon noch „je Doppelbeleg“. Das ist nach README **kein gültiger Doppelbeleg**, sondern zweimal Aggregator; 11880 verlinkt sogar direkt auf schrottplatz-info. Die Unabhängigkeit der Datengrundlagen ist nicht nachgewiesen. Der Vorbehalt vom 03.10. bleibt durch den aktuellen Diff erhalten und wird verstärkt. Historische Importzeilen und der abweichende Adresslead bleiben vollständig erhalten.

## Direkt geprüfte Links und zweiseitiger Adressbefund

Alle folgenden Abrufe am 15.10.2026 mit lesbarem Seiteninhalt, soweit nicht anders vermerkt. Sie beweisen bei Verzeichnissen nur, was diese anzeigen.

| Quelle | Direktbefund | Tragweite |
| --- | --- | --- |
| [11880-Suche Bornheim](https://www.11880.com/suche/schrotthandel/bornheim) | „Schrott und Metallhandel P. Welsch“, Straufsberg 2, 53332 Bornheim (Waldorf), (02227) 908232. | Aggregator-Lead für die erste Adresse. „Geschlossen“ erscheint im Öffnungszeitenkontext, kein Schließungsbeleg. |
| [11880-Detail](https://www.11880.com/branchenbuch/bornheim-rheinland/060440090B52609796/schrott-und-metallhandel-p-welsch.html) | Gleicher Name/Adresse/Telefon; Mo–Fr 9–17 Uhr; Websiteziel tatsächlich `https://schrottplatz-info.de`; Eintragsdaten 02.06.2026, Aktualisierung 10.10.2026. | Die Zeitangabe ist direkt auf der Detailseite sichtbar, nicht allein durch die bisher zitierte Suchseite gedeckt. Aktualisierungsdatum ist kein Betreiberbeleg; generische Erläuterung „vom Inhaber bestätigt“ bestätigt diesen Eintrag nicht. |
| [schrottplatz-info: Welsch P.](https://www.schrottplatz-info.de/schrottplatz/Bornheim/Welsch-P-) | Straufsberg 2, 53332 Bornheim, Tel. (02227) 90 82 32; Beschreibung/Leistungsangebot fehlen, Zeiten nicht erfasst. „zur Homepage“ führt auf `/`, also das Verzeichnis selbst. | Kein Betreiberlink, keine unabhängige Identitätsbestätigung. |
| [schrottplatz-info Startseite](https://schrottplatz-info.de) und [Impressum](https://www.schrottplatz-info.de/impressum) | Deutschlandweites Portal; Verantwortlicher Colin Schulz, Wiesbaden. Disclaimer: weder Betreiber der Schrottplätze noch Anbieter ihrer Dienstleistungen. | Direkt geprüfte Widerlegung der Interpretation des 11880-Websitefelds als Welsch-Website. Keine Übernahme der Domain ins Frontmatter. |
| [GoLocal Welsch](https://www.golocal.de/bornheim/schrotthandel/schrott-metallhandel-p-welsch-47MaY) | „Schrott und Metallhandel P. Welsch“, **Kerpengasse 2**, 53332 Waldorf Stadt Bornheim im Rheinland; **dieselbe Nummer (02227) 90 82 32**; keine Zeiten. Ein Nutzerbericht vom 17.07.2015 erwähnt kostenlose Demontage/Abholung eines 600-Liter-Öltanks. | Zweite Adresse eigenständig direkt geprüft. Namens- und Telefonübereinstimmung sind nur ein Verknüpfungslead, kein Beweis derselben Rechtsperson, eines Umzugs oder aktueller Betriebsanschrift. „Offizieller Telefonbucheintrag“ ist keine Registerauskunft. Rezension kein Betreiberbeleg und kein aktueller Gratistarif. |
| [MWS Design: Impressum](https://mwsdesign.de/impressum) | MWS Design, Einzelunternehmen, Inhaber Maurice Schlebusch, **Kerpengasse 2, 53332 Bornheim**; andere Telefonnummer; Impressumsstand August 2024. Website nennt Webdesign/Marketing/Social Media/Digitalisierung. | Konkreter fremder Anbieter an der zweiten Adresse nach eigener Anbieterkennzeichnung. **Kein Nachweis für Welsch, keinen Schrotthändler belegt.** Beweist weder Verdrängung/Schließung noch ausschließliche Belegung oder eine Beziehung zu Welsch. Keine Datenübernahme. |

**Straufsberg-Seite:** Zusätzlich nach exakter Adresse und nach Straße + Welsch gesucht. Kein passendes Betreiber-Impressum, Betreiberprofil oder Registerbeleg ermittelt; damit auch kein belegter anderer Schrotthändler an Straufsberg 2. Zwei lesbare Verzeichnisse reichen nicht zur Bestätigung einer realen aktuellen Annahmestelle.

**Kerpengasse-Seite:** Zusätzlich nach exakter Adresse und Straße + Welsch gesucht. Neben dem direkt geprüften GoLocal-Lead wurde das oben direkt geprüfte fremde Impressum gefunden. Keine Betreiber-/Registerquelle bestätigt dort Welsch oder einen anderen Schrotthandel. Mehrfachnutzung, Altadresse, Datierungs-/Zuordnungsfehler bleiben unaufgelöste Möglichkeiten, nicht Feststellungen. Die beiden Anschriften werden weder vereinigt noch gegeneinander als „richtig/falsch“ entschieden.

## Betreiber-/Registersuche und Grenzen

Gezielte Websuchen nach `"Welsch" "Bornheim" "Schrott" Register`, `"P. Welsch" Schrott Impressum`, `"Welsch" "Bornheim" site:northdata.de`, entsprechend `site:firmeneintrag.creditreform.de`, den beiden exakten Anschriften sowie `"02227908232"` lieferten **keinen passenden Betreiber-/Registerbeleg**. Ergebnisse zu anderen Welsch-Personen/Firmen oder anderen Orten wurden nicht zugeordnet. Aus dem Initial „P.“ wurde kein Vorname erschlossen.

Das [gemeinsame Registerportal](https://www.handelsregister.de/rp_web/welcome.xhtml) wurde direkt aufgerufen. Die Antwort enthielt Portalnavigation, aber auch „Expired session“/Anwendungsfehler; **keine belastbare unternehmensbezogene Suchergebnisliste und kein Registerauszug** wurden gewonnen. Deshalb ausdrücklich kein negativer Handelsregisterbescheid, keine Aussage „nicht eingetragen“ und keine behauptete Registerprüfung mit Ergebnis. Auch fehlende Suchmaschinenfunde bei Northdata/Creditreform sind keine Nichtexistenzbelege. Ein kommunaler Gewerberegisterauszug und eine Betreiberantwort liegen nicht vor; es wurden keine kostenpflichtigen Anträge oder Kontaktaufnahmen ausgelöst.

Die Suchausgabe war teilweise unpräzise und lieferte sachfremde Treffer. Deshalb tragen allein die genannten direkt abgerufenen Seiten die positiven Inhaltsbefunde. Eine erschöpfende Register-/Existenzprüfung oder Standortbesichtigung wurde nicht geleistet. Die unbekannte aktuelle Existenz bleibt als möglicher Phantom-/Altbestand unter `pruefung` offen, statt aus dem bloßen Entfernen eines falschen Links einen bestätigten Restdatensatz zu konstruieren.

## Feldprüfung und Leistungs-/Preisgrenzen

| Feldgruppe | Bestand und Bewertung |
| --- | --- |
| `slug` | `nw-bornheim-welsch`, unverändert und stabil zu halten. |
| `name`, `trader_type` | `Welsch`, `schrotthaendler`: historische/aggregatorgestützte Zuordnung, keine verifizierte Rechtsform, kein gesicherter vollständiger Betreibername. Nicht aufgrund fremder Namensähnlichkeit ergänzen. |
| `state`, `city`, `postcode`, `street` | NW / Bornheim / 53332 / Straufsberg 2 unverändert. Orts-/PLZ-Plausibilität ersetzt keinen Identitäts- oder Standortbeleg. Straße bleibt widersprochener Altwert, nicht geprüftes Betriebsziel. Kein GEO ableiten. |
| `phone` | 02227 908232 unverändert; auf beiden Adressleads gleich, aber nicht als aktuell erreichbar oder Betreiberkontakt verifiziert. |
| `website`, `email`, `opening_hours`, `description` | Weiter leer. Keine Portal- oder MWS-Domain, keine fremde E-Mail, keine 11880-Zeiten übernehmen. Überblick bleibt Arbeitsnotiz, keine verifizierte Leistungsbeschreibung. |
| `status`, `website_status` | `pruefung` / `unbekannt`, unverändert sachgerecht. Weder `aktiv`, `tot` noch `geschlossen` nachgewiesen. |
| `dropoff_json`, `pickup_json` | Weiter leer: keine Anlieferungs-/Abholfreigabe, Kundengruppe, Menge oder Bedingung belegt. |
| Provenienz | `provenance_section`, `provenance_ankauf_raw: unklar`, `provenance_origin: table` unverändert; Herkunftsangaben sind keine Validierung. |

- **ANKAUF:** Verzeichniskategorie „Schrotthandel“ ist keine aktuelle Betreiberbestätigung für Ankauf, Auszahlung, bestimmte Materialien oder Privatannahme. Keine belastbare Ankaufspreisliste gefunden; nichts als Preisbeobachtung/Handlerquelle freigeben.
- **VERKAUF:** Keine Betreiber-Verkaufsliste oder Verkaufspreise in den geprüften Quellen gefunden. Das ist kein Beleg, dass Verkauf nicht angeboten wird.
- **GEBÜHREN:** Keine aktuelle Betreiber-Tarifliste gefunden. Die GoLocal-Erfahrung von 2015 begründet weder generelle kostenlose Abholung/Demontage noch einen heutigen Nullpreis. Auch 11880-Euro-/Preisspannen-FAQ ist kein Tarif.

## Offener Nachweis vor einer fachlichen Aufwertung

Benötigt wird eine zurechenbare aktuelle Betreiber-/Gewerberegisterauskunft mit vollständiger Identität, Tätigkeit und Betriebsanschrift sowie eine unabhängige zweite zulässige Quelle entsprechend README; für einen behaupteten Umzug zusätzlich die nachvollziehbare Verbindung beider Anschriften. Dabei ausdrücklich nach früherer versus aktueller Adresse und nach Annahmestelle versus reiner Kontaktanschrift unterscheiden. Bis dahin nur die vorhandene vorsichtige Dokumentation integrieren, keine Sachfelder als verifiziert ausgeben und keine Produktionsverifikation behaupten. Seed-/Buildtests wurden für diesen reinen Berichtsauftrag nicht ausgeführt; ein späterer Integrator verantwortet das README-Seed-Gate.
