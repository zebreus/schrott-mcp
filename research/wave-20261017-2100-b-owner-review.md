# Owner-Endprüfung — Welle 17.10.2026 21:00, ausschließlich B

## Entscheidung

**Die vorliegenden Änderungen aller 20 B-Dossiers und der Shard-Bericht sind inhaltlich zur gemeinsamen Integration freigegeben. Kein neuer dossierseitiger Blocker gefunden.** Broicher und SRG einschließlich der neuen Frontmatterwerte wurden unabhängig durch tatsächliche Betreiber- und Northdata-Abrufe nachgeprüft. Die übrigen 18 erhalten eine Freigabe ihrer vorsichtigen Recherche-/Klärfallnotizen, ausdrücklich keine neue Bestätigung sämtlicher dort referenzierter Quellen oder historischer Felder.

Keine Freigabe für Produktion, GEO, numerische Preisübernahme oder Handlerentwicklung. Die gemeinsame Build-/Seedprüfung ist außerhalb dieses Auftrags noch erforderlich; hier wurden keine Tests ausgeführt. Die nachfolgend genannten offenen Betreiberfragen blockieren weitergehende Datenfills, nicht die Integration der als offen gekennzeichneten Notizen.

## Prüfgrundlage und echte Diff-Abgrenzung

- Gelesen: `README.md`, insbesondere Zeilen 183–202 und 204–240; ergänzend `docs/trader-seed.md`, Owner-Gate in `prompts/tiefenrecherche-welle.md` und die Enumdefinitionen in `crates/ingestion/src/seed_traders.rs`.
- Auswahl ausschließlich `shards.b` aus `/tmp/opencode/wave-20261017-2100-selection.json`: exakt 20 verschiedene Pfade. Kein anderer Shard geprüft.
- Auswahlbasis: `deca75f9e11d2446934632a96d99d2bb4f61f155`. Tatsächlich angetroffener HEAD: `dbb61a659a9a2f62506685cd7b2b6dd161f4c29a`. **Alle 20 HEAD-Dateiinhalte passen trotzdem zu den in der Auswahl gespeicherten SHA-256.** Damit wurde nicht irrtümlich gegen einen fremden Zwischenstand geprüft.
- Für jeden der 20 Pfade den vollständigen echten `git diff HEAD -- <Pfad>` gelesen, nicht nur den Agentenbericht oder eine Dateistatistik.
- In allen 20 ist der komplette bisherige Inhalt nach `## Timeline` bytegleich als Suffix erhalten. Genau ein neuer Abschnitt `### Recherche 17.10.2026 — Welle 21:00 B` je Dossier; keine Löschung alter Gegenbelege. Zehn bisherige Überblick-Platzhalter ersetzt, keiner bleibt in diesen 20 übrig.
- Frontmatter bei 18 bytegleich; Änderungen ausschließlich Broicher (sechs Felder) und SRG (vier Felder). Slugs, Namen, Adressen, Provenienz, Annahme-JSON und Koordinaten nicht verändert. Keine neuen Clear-Direktiven.
- Status/Website-Status entsprechen den erlaubten Werten. Bereits vorhandener leerer `website_status` ist nach dem gelesenen Validator ausdrücklich zulässig, kein neu entstandener Enumfehler. Händlertyp und Bundesland bleiben unverändert und zulässig.
- Em-Dash-Prüfung: kein `—` in den 20 Frontmattern; neu jeweils nur als typografischer Trenner der Abschnittsüberschrift. Kein Em-Dash als Feld-Platzhalter oder vermeintliche unbekannte Zahl.
- Bericht `research/wave-20261017-2100-b.md` vollständig gelesen; sämtliche 22 relativen Dossierlinks lösen auf vorhandene Dateien auf (20 verschiedene Dossiers, Broicher/SRG zusätzlich im Fließtext). Externe Beleglinks sind konkrete Firmen-/Profil-/Impressumsseiten; die unten erläuterten Übersichts-/Rootlinks sind keine heimlich als Detailbelege ausgegebenen Ersatzquellen.
- Bericht-SHA-256 zum Prüfstand: `96985250604cf462f7f86ddbdc725bf761be43f0affbcb1d0ba2c4c4c8c09f7f`.

## Unabhängige tatsächliche Abrufe: Broicher

Am Prüflauf 17.10.2026 erfolgreich textlich abgerufen und gelesen:

1. <https://www.pressbetrieb.de/impressum>: Broicher Pressbetrieb GmbH, Scheibenstraße 159, 50737 Köln, HRB 10153 Amtsgericht Köln; Telefon `00492217122031`, Mail `broicher-koeln@t-online.de`. Weiter unten im Datenschutz-Kontakt auch `info@pressbetrieb.de`.
2. <https://www.pressbetrieb.de>: dieselbe Firma/Adresse, Hauptmail `info@pressbetrieb.de`, sichtbarer Telefontext `0221/7122031`; Öffnungszeiten Mo–Do 07:00–16:00, Fr 07:00–15:00. Ausdrücklich Ankauf FE-/NE-Metalle, Metallpaketproduktion und gewerblicher Containerservice, ausdrücklich kein Autoersatzteilverkauf. Privat-Ausweishinweis tatsächlich vorhanden; im Dossier zutreffend nur als Betreiberbedingung wiedergegeben.
3. <https://www.pressbetrieb.de/containerservice>: Terminvereinbarung, Bereitstellung auf Firmengelände, Austausch/Verwertung und Überweisung des Materialerlöses; Telefon `0221/7122031`. Kein numerischer Tarif oder Gratisversprechen im gelesenen Text.
4. <https://www.northdata.de/Broicher%20Pressbetrieb%20GmbH,%20K%C3%B6ln/HRB%2010153>: Firma, HRB/EUID, Güterbahnhof Niehl/Scheibenstr. 159/50737 kongruent; Publikationsübersicht nennt Jahresabschluss 2025 am 06.08.2026. Kein amtlicher Originalregisterauszug abgerufen.

**Feldfreigabe:** `phone`, `email`, `opening_hours`, `website`, `website_status: aktiv`, `description` wie im Diff. Der bestehende `status: aktiv` ist nun zusätzlich durch den Betreiberauftritt getragen. Keine beleglose Überschreibung einer vorhandenen Telefonnummer oder Adresse.

**Grenzen:** Ein Startseiten-Anfragenlink ist tatsächlich falsch verdrahtet (`tel:02217122301`), während sichtbarer Text, Kontaktblock, Impressum und Containerseite die übernommene Nummer bestätigen. Kein Blocker. Die Website hat Copyright 2021 und Datenschutzstand 29.10.2020; ihre Erreichbarkeit beweist kein Veröffentlichungsdatum 2026. Deshalb Freigabe über kongruente Betreiberquelle plus unabhängigen Registerspiegel, nicht über eine ungeprüft behauptete aktuelle Alleinbeleg-Ausnahme. Die beiden Betreiberseiten und das Impressum zählen zusammen als eine Quelle. Einfahrt „über Holzland Schwan“ ist Websiteauskunft, kein hier bestätigter Zugangspunkt. Keine numerischen Ankauf-, Verkaufs- oder Gebührenpreise aus Paketmaßen/Gewicht herleiten.

## Unabhängige tatsächliche Abrufe: SRG

Am selben Prüflauf erfolgreich textlich abgerufen und gelesen:

1. <https://bgh.de/merchant/SRG-Schrott-Recycling-GmbH>: ausdrücklich SRG Schrott & Recycling GmbH, Industriestraße 1b/57076 Siegen, `+49 271 77292-70`, `info@srg-siegen.de`; Montag–Donnerstag 07:00–16:30, Freitag 07:00–16:00, Wochenende geschlossen. Text: seit mehr als 20 Jahren Teil der BGH-Gruppe, weltweiter Handel mit Schrotten.
2. <https://bgh.de/informationen/impressum/>: Herausgeber BGH Edelstahlwerke GmbH, Freital, Registergericht Dresden HRB 7. **Nicht** als Impressum der eigenständigen SRG GmbH missverstanden.
3. <https://www.northdata.de/SRG%20Schrott%20und%20Recycling%20GmbH,%20Siegen/HRB%204931>: SRG, HRB 4931 AG Siegen, Industriestr. 1 b/57076; Gegenstand Schrott-/Metallhandel und Recycling. Publikationsübersicht: Abschluss 2024 am 10.02.2026 und „Gesellschafter: BGH Edelstahlwerke GmbH“ am 06.03.2026. Dies ist der unabhängige Brückenbeleg zum Websiteherausgeber, kein bloßer Schluss aus gleicher Anschrift.
4. <https://rps-siegen.de/historie>: 1998 Gründung SRG durch RPS und Albert Buchen. Fußzeile ausdrücklich für RPS: Warenannahme/Verladung 07:00–14:30, keine Privatannahme, kein Autoschrott. Stützt historische Gruppenzuordnung, ersetzt aber nicht die unabhängige Registerquelle.

**Feldfreigabe:** `website` auf konkrete SRG-Unterseite, `website_status: aktiv`, `status: pruefung → aktiv`; bestehende nichtleere `opening_hours` von 07:30 auf 07:00 korrigiert, Endzeiten erhalten. Gerade dieser echte Overwrite ist direkt durch die konkrete Standortseite getragen und in der neuen Timeline dokumentiert. Telefon/Mail/Adresse bleiben unverändert und wurden im eigenen Abruf nachbestätigt.

**Grenzen:** Die Gruppenwebsite ist mit dem Northdata-Gesellschafterhinweis identitätskongruent, aber kein Nachweis einer Verschmelzung SRG/RPS. Keine RPS-Privatannahme-/Autoschrottverbote auf SRG übertragen. Öffnungszeiten sind keine automatisch bestätigten SRG-Waagen-/Annahmezeiten; JSON bleibt zurecht leer. Keine Original-Gesellschafterliste, Beteiligungsquote oder amtlicher Tagesauszug abgerufen; öffentlich lesbare Spiegelübersicht genügt hier dem README-Zwei-Quellenstandard. Keine numerischen SRG-Preise im gelesenen Seitenumfang.

## Einzelentscheidung aller 20 echten Diffs

„Notiz frei“ bedeutet nur: neue Rechercheaussage und ihre Grenzen widersprechen weder dem Diff noch dem README-Standard; Frontmatter bleibt erhalten. Die verlinkten Quellen der 18 Notizfälle wurden in dieser Endprüfung nicht erneut live abgerufen. Ihre Detaildaten/Abrufcodes bleiben dem Recherchebericht zugeschriebene Ergebnisse, keine hier behauptete eigene Quellenbestätigung.

| Nr. | Dossier (Dateiname unter `dossiers/<state>/`) | Entscheidung / weitergehender Blocker |
|---|---|---|
| 1 | `ni-hildesheim-holtensen-hildesheimer-schrotthandler.md` | Notiz frei. Wennigsen/Miro nur Namens-/Telefonlead, keine Ortskorrektur; Betreiber und Ort offen. |
| 2 | `ni-hannover-30539-anders-schrott.md` | Notiz frei. GS-Kontakt nur Lead; Straße, Rechtsträger und Annahmefunktion fehlen. |
| 3 | `nw-lippstadt-a-r-schrottabholung-lippstadt.md` | Notiz frei. Portaldatum nicht als Betreiberfrische ausgegeben; Autoankauf-/Schrottabholungsalias und Dortmundkette offen. |
| 4 | `hh-stellingen-georg-lensch-sohne.md` | Notiz frei. KG/GmbH sauber getrennt; alter Geschlossenstatus nicht neu als bewiesen erklärt. Seed-GmbH/Lederstr. 33 und Jahr 2018 weiter ungeklärt. |
| 5 | `nw-minden-pold.md` | Notiz frei. Tobias-/Gratis-/Barzahlungswerbung nicht hochgestuft; blockierte Kleinanzeige kein Betreiberbeleg. |
| 6 | `nw-rheda-wiedenbruck-sasse.md` | Notiz frei. Register-KG nicht mit Hermann/Südring verschmolzen; leere HTTP-200-Seite weder Aktiv- noch Schließungsbeweis. |
| 7 | `nw-lennestadt-l-mulaj.md` | Notiz frei. 403 korrekt begrenzt, zifferngleiche Nummern kein neuer Konflikt; Leonard-/Baro-2-Zuordnung und Hofannahme offen. |
| 8 | `nw-essen-cbr-bracke.md` | Notiz frei. Rechtsträgerlöschung nicht als taggenaue Essener Hofschließung gewertet; Standort/Nachfolge offen. |
| 9 | `nw-bergheim-frechen-gartner.md` | Notiz frei. Historische Nummer/Zeiten nicht durch Portalwerte überschrieben; Wochenend-Schließtext kein endgültiger Schließbeleg. |
| 10 | `ni-delmenhorst-deichhorst-27753-grundler-siegfried-autoverwertung.md` | Notiz frei. Konkreter Örtliche-Lead, keine PLZ-/Telefonfills; fremde Empfehlungsadresse abgegrenzt. |
| 11 | `sh-lensahn-milon-sonke-schrott-u-autorecyclinghof.md` | Notiz frei. Landesportal mit Stand 2024 nicht als heutige Hoföffnung verkauft; Lübeck separat, 503 kein Schließungsbeweis. |
| 12 | `nw-koln-nippes-broicher-pressbetrieb.md` | Frontmatter und Notizen frei; eigene Quellenprüfung oben. Künftige Zufahrts-/Punktprüfung bleibt separat. |
| 13 | `nw-koln-mulheim-a-f-altleben.md` | Notiz frei. Bochumer Prosche nicht Köln zugeschrieben; bildbasiertes Zusatzimpressum ausdrücklich ungelesen, Aliasfrage offen. |
| 14 | `ni-wolfsburg-vorsfelde-eisen-metallhandel-ankauf-und-verkauf.md` | Notiz frei. Possiel nicht mit generischem Seed gleichgesetzt; Name ist weder Preis- noch aktueller Leistungsbeweis. |
| 15 | `hh-veddel-veddel-oberwerder-damm-11-21.md` | Korrektur frei. Frühere Zwei-Portal-Fills explizit relativiert, voreilige GEO-Empfehlung zurückgenommen; Oberwerder Damm/Hovestr. und Wagner-Rechtsträger offen. Keine pauschale Verifikation alter Felder. |
| 16 | `nw-detmold-schmidt-peter-tino.md` | Notiz frei. Portalnachprüfung keine Primärbestätigung; H./Heinz-Henry nicht zusammengeführt, aktuelle Gewerbe-/Hofadresse offen. |
| 17 | `nw-monchengladbach-birker.md` | Notiz frei. Karl-Wilhelm/Veronika nicht zusammengeführt; Portal-„geschlossen“ nicht als Statuswechsel übernommen. |
| 18 | `nw-erkelenz-kb.md` | Notiz frei. Nur indexlesbare Kandidatendomain, Zertifikatsfehler nicht globalisiert; Frank/Franz-Halcour und Erkelenz/Mönchengladbach offen. |
| 19 | `nw-siegen-srg-schrott-und-recycling.md` | Frontmatter und Korrektur frei; eigene Quellenprüfung oben. Privatannahme und SRG-Annahmefenster bleiben offen. |
| 20 | `nw-leverkusen-alsleben.md` | Notiz frei. Kein Gratis-/Aliasfill aus Titel oder Bewertung; Küppersteger Straße und Kölner Altleben bleiben getrennte Klärfälle. |

## Deep-Links, Bericht und Abrufgrenzen

- Die neue SRG-Website ist eine tatsächlich gelesene Standortdetailseite, keine generische BGH-Startseite. Broicher-Root ist die tatsächliche Betreiberseite mit Kontakt-/Leistungsinhalt; Impressum und Containerdetail sind zusätzlich konkret verlinkt und gelesen.
- Milons Landesportal-Link ist eine gefilterte Anlagenübersicht, kein behaupteter einzelner Bescheid. Sasse-/Wagner-Roots sind explizite Leer-/Timeoutversuche, keine Inhaltsbelege. Veddels Verweis auf alte lokale Timeline ist als historische Referenz transparent; die neue Korrektur behauptet keinen dort fehlenden neuen Deep-Link. Kein Blocker daraus.
- Den in `wave-20261017-2100-b.md` dokumentierten 403/503/Timeout-/TLS-/Bildextraktionsfällen wird hier nicht nachträglich ein erfolgreicher eigener Abruf unterstellt. Eigene Liveabrufe dieser Endprüfung beschränken sich auf die acht oben einzeln aufgeführten Broicher-/SRG-Seiten. Sie lieferten lesbaren Text; eingebettete Karten/Bilder, Premium-Dokumente und amtliche Registeroriginale nicht geprüft.
- Die Behauptung „keine aktuelle numerische Betreiberpreisliste neu erschlossen“ ist mit den Dossieränderungen und den eigenen acht Abrufen vereinbar, aber keine Vollständigkeitsbehauptung über das gesamte Web. Keine Preisquelle nur aus Portalwerbung konstruiert.
- Alte nichtleer gespeicherte Felder bleiben bei den 18 erhalten; die neuen Warnnotizen sind keine technische Rücknahme möglicherweise gespeicherter Daten. Nach `docs/trader-seed.md` würden leere Enrichmentfelder bestehende DB-Werte ebenfalls nicht automatisch löschen. Kein Clear-/Produktionsauftrag daraus abgeleitet.
- Ausschließlich diese Reviewdatei geschrieben. Keine Dossieränderungen, Tests, Commits, Produktionszugriffe oder GEO-Requests/-Writes durchgeführt. Freigabe gilt dem gelesenen B-Prüfstand; spätere Änderungen benötigen erneuten Diff-Abgleich.
