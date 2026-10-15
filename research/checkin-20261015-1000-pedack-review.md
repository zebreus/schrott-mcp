# Integrationsreview: Metall Service Pedack → CRONIMET CREMETAL

Stand: 15.10.2026. Prüfgegenstand ausschließlich die bereits vorhandenen uncommitteten Änderungen an `dossiers/bw/bw-karlsruhe-metall-service-pedack.md` gegenüber HEAD `ebc1d15ce1ec7910a4d5a150a3e9f25f8a0eaa9d` (17 hinzugefügte, 11 entfernte Zeilen). Original mit `git show HEAD:…`, Arbeitsfassung und vollständigen `git diff HEAD -- …` gelesen. Kein Dossier bearbeitet; keine Datenbank geöffnet, kein Commit/Push/Deploy.

## Entscheidung

**Bedingt integrationsreif, keine unveränderte Veröffentlichungsfreigabe.** Die wesentliche Betreiberkorrektur ist belastbar: keine bloße Adressgleichheit, sondern eine dokumentierte Umfirmierung derselben HRB-110265-Gesellschaft. Aktueller Standort, Kundenkreis, Ankauf, Kontaktdaten und reguläre Zeiten sind direkt bestätigt. Kein Identitätsblocker und kein Grund, CREMETAL mit der Holding zusammenzuführen.

Vor Freigabe sind die unten bezeichneten Auszahlungsbedingungen nachzutragen und der vorgeschriebene Seed-Test auf dem tatsächlich ausgewählten Integrationsstand nachzuweisen. Das ist eine begrenzte Nacharbeit, keine Aufforderung, die belegten Betreiberfelder zurückzusetzen. Dieser Review hat selbst keine Integrations- oder Produktionsverifikation vorgenommen.

## Maßstab

- `README.md:180–199`: Verzeichnisse nur Leads; grundsätzlich zwei unabhängige zugelassene Quellen. Die qualifizierte Betreiberseite genügt für eigene Standortfakten, nicht allein für Umfirmierungs-/Identitätsketten.
- `README.md:217–227`: Registerkette belegen, Standortdetailseite einzeln prüfen, früheren Namen erhalten, Slug stabil lassen, Schwester-Dossiers querprüfen statt zusammenführen.
- `README.md:233–237`: Owner-Gate für Identität, Quellen, Historie und nichtleere Overwrites; `cargo test -p schrott-mcp-ingestion seed` vor Integration, Produktionsprüfung erst nach autorisiertem Redeploy.
- `README.md:76–80,108–129`: Annahme-/Abholbedingungen sind strukturierte Fakten; Frontmatter wird importiert, Überblick ist Arbeitsnotiz, Timeline wird zu `notes`; fehlendes Koordinatenpaar erhält bestehende DB-Koordinaten.

## Direkt geprüfte Quellen

Alle folgenden Inhalte am **15.10.2026** per HTTP gelesen, nicht nur aus Suchtreffern übernommen. Northdata ist hier die im README ausdrücklich zugelassene Registeraufbereitung, nicht ein eigener Abruf beim amtlichen Registerportal.

1. **Betreiber-Standortdetail:** https://www.cronimet-cremetal.de/metallrecycling/recyclingzentrum-karlsruhe
   - „CRONIMET Cremetal GmbH“, Rheinhafenstr. 12, 76189 Karlsruhe; +49 721 16131-0; Standortkontakt `karlsruhe@cronimet-cremetal.de` (im HTML/Linktitel trotz sichtbarem `noSpam` erkennbar).
   - Büro Mo–Fr 07:30–16:00; Anlieferung/Kasse Mo–Fr 07:00–15:45.
   - Ausdrücklich Privatpersonen, Gewerbe und Industrie; „Annahme“ und „Ankauf“; Stahlschrott, Edelstahl-Legierungen, NE-Metalle, Kabel, Batterien.
   - „Unsere tagesaktuellem Ankaufpreise im Recyclingzentrum Karlsruhe erhalten Sie auf Anfrage.“ Keine numerische Materialpreisliste in der gelesenen Seite.
   - Sonder-Samstage 19. August, 10./17./24. Oktober, jeweils 09:00–11:45, ohne Jahresangabe direkt an diesen Zeilen; zusätzlich ausdrücklich „Samstags ist keine Barauszahlung möglich.“ Schließhinweis 09.10.2026 belegt zeitbezogene Pflege. Die Samstagstabelle sollte wegen ihrer gemischten Datumsangaben nicht eigenmächtig als dauerhafter Wochenplan oder als vollständig jahresgesicherte Terminliste interpretiert werden.
2. **Betreiber-Impressum:** https://www.cronimet-cremetal.de/impressum
   - CRONIMET CREMETAL GmbH, Rheinhafenstraße 12, 76189 Karlsruhe, AG Mannheim **HRB 110265**, vertreten durch Patrick Manias; Telefon kongruent. Allgemeine Mail `info@cronimet-cremetal.de` ersetzt nicht die spezifische Standortmail.
3. **Registeridentität und heutiger Stand:** https://www.northdata.de/CRONIMET%20CREMETAL%20GmbH,%20Karlsruhe/Amtsgericht%20Mannheim%20HRB%20110265
   - Beide Namen unter derselben HRB 110265, gleiche Adresse; aktuelle Unternehmensdarstellung und Publikationsverknüpfungen bis 2026. Kein Erlöschens-/Liquidationshinweis im gelesenen Inhalt. Das ist kein selbst eingeholter amtlicher aktueller Registerauszug.
4. **Konkrete Registerbekanntmachung über Northdata:** https://www.northdata.de/?id=5663977365307392
   - Registerbekanntmachung vom **10.01.2022**: „HRB 110265: Metall Service Pedack GmbH, Karlsruhe, Rheinhafenstr. 12, 76189 Karlsruhe“; Gesellschafterversammlung **17.12.2021**; „Firma geändert, nun: CRONIMET CREMETAL GmbH.“ Damit ist „Umfirmierung 2022“ sachgerecht als Bekanntmachungsjahr; Beschluss und Veröffentlichung sind nicht dasselbe Datum.
5. **Verlinkte Betreiber-Leistungsdetails:** https://www.cronimet-cremetal.de/metallrecycling
   - „Wir nehmen Metalle in unterschiedlichen Mengen an und bieten eine Vergütung ab einem Materialwert von **5 Euro**.“ Das ist eine Auszahlungsschwelle, kein EUR/kg-Preis, keine pauschale Gebühr und kein Mindestgewicht.
   - Containerservice für Gewerbe, Industrie und private Projekte, 3–40 m³; keine numerischen Container-/Entsorgungstarife im gelesenen Inhalt. Diese allgemeine Leistungsseite allein begründet noch keine uneingeschränkte, standortspezifische kostenlose Abholung.
6. **Aktualitäts-/Zertifizierungsindiz:** https://www.cronimet-cremetal.de/kontakt/zertifizierungen
   - Standortbezug Karlsruhe/Annweiler; verlinktes EFB-Dokument `Zertifikat2026_EFB_alt.pdf`. PDF-Inhalt und Gültigkeitsende nicht geprüft; keine Zertifikatsgültigkeit daraus behauptet. Zusammen mit dem datierten Standort-Schließhinweis ein Pflegeindiz, kein selbständiger Identitätsbeleg.
7. **Abgrenzung Holding:** https://www.cronimet.de/de/
   - Im eingebetteten Impressum CRONIMET Holding GmbH, Südbeckenstraße 22, 76189 Karlsruhe, **HRB 110997**, +49 721 95225-0. Dieselbe Seite führt CREMETAL getrennt an Rheinhafenstr. 12 mit +49 721 16131-0. `/de/impressum` lieferte 404; der Beleg stammt aus dem tatsächlich gelesenen Hauptseiten-Impressum.

Die mehreren CREMETAL-Unterseiten zählen nicht als mehrere unabhängige Quellen. Für die Kette stehen die konkrete Registerbekanntmachung samt Namenshistorie und das HRB-kongruente aktuelle Betreiber-Impressum nebeneinander; für Standortfakten greift die Betreiber-Ausnahme. Es wurde nicht jede weltweit verlinkte Konzernniederlassung geprüft: relevant sind Karlsruhe und die belegte Abgrenzung zum lokalen Holding-Dossier.

## Identität, Overwrites und Historie

| Änderung gegenüber HEAD | Bewertung |
| --- | --- |
| `name`: Metall Service Pedack → CRONIMET CREMETAL GmbH | Zulässiger nichtleerer Overwrite: gleiche HRB, expliziter Alt-/Neunamenbeleg [3–4], heutiges Betreiber-Impressum [2]. Keine Übertragung allein wegen gemeinsamer Anschrift. |
| `trader_type`: sonstige → schrotthaendler | Zulässige Präzisierung; örtlicher Schrott-/Metallhandel und Ankauf explizit [1]. |
| `status`: pruefung → aktiv | Inhaltlich tragfähig durch geklärte Kette, aktuelle Kundenannahme und gepflegte Standortseite [1–4]. Kein Beleg für eine weiterhin eigenständige aktive „Pedack“-Marke nötig, da der Name ausdrücklich ersetzt wird. |
| Leere Telefon-/Mail-/Zeit-/Website-/Beschreibungsfelder gefüllt | Direkt belegt [1–2]; `website_status: aktiv` durch erreichbaren Betreiberauftritt bestätigt. Telefonnummer nur national formatiert, inhaltlich gleich. |
| Adresse/PLZ/Stadt/Slug | Unverändert; Adresse durch Betreiber und Register erneut bestätigt. Slug-Stabilität korrekt. |
| `dropoff_json`, `pickup_json` weiterhin leer | Kein erfundenes Abholversprechen. Annahme ist inzwischen bekannt, aber noch nicht strukturiert abgebildet; Datenvollständigkeitslücke, kein Identitätsfehler. |
| `provenance_ankauf_raw: unklar` / historische Provenienz | Unveränderte Seed-Herkunft, nicht als aktuelle Widerlegung des belegten Ankaufs lesen. Kein Bedarf, Rohprovenienz rückwirkend umzuschreiben. |

**Historie tatsächlich erhalten:** Der komplette Abschnitt ab `### Importiert (Seed-Stand 2026-09-30)` bis Dateiende ist gegenüber HEAD exakt gleich geblieben (programmatischer Textvergleich: `True`). Damit bleiben Seedname, Klärfälle vom 01.10., Registerbefund und GEO-Hinweis vom 04.10. erhalten. Der neue Befund vom 13.10. steht davor und erklärt die Statusänderung. Früherer Name bleibt zusätzlich in Kopf/Überblick und Slug sichtbar; kein stilles Umschreiben alter Recherchebehauptungen.

Arbeitsfassungs-SHA256 bei Prüfung: `cd60c51794fffb72a473259523fd474b50835dfbc3945e5e96622b38bbc97f1e`. Nachträgliche Änderungen sind nicht durch dieses Urteil abgedeckt.

**Holding-Dublettenfrage erledigt für diesen Scope:** `dossiers/bw/bw-karlsruhe-cronimet-holding.md` bezeichnet HRB 110997 an Südbeckenstr. 22; CREMETAL ist HRB 110265 an Rheinhafenstr. 12. Nicht verschmelzen, keine Holding-Kontakte/-Leistungen hierher kopieren. Der neue generische Hinweis „bitte Zuordnung/Dublettenprüfung“ kann durch diesen konkreten Prüfbefund präzisiert werden; die Holding-Datei muss hierfür nicht geändert werden. Deren eigene Handels-/Annahmequalifikation ist kein Ergebnis dieses Reviews.

## Ankauf, Verkauf, Gebühren: Grenzen

- **Ankauf:** Für die genannten Materialgruppen und Kundenarten belegt, Preis nur auf Anfrage. Keine Zahlen erfinden oder aus Annweiler/anderen Konzernseiten übertragen. Die **5-Euro-Materialwertschwelle** ist ergänzend bekannt und im Diff nicht enthalten.
- **Verkauf:** Vermittlung an Verarbeiter ist beschrieben; daraus folgt weder ein öffentlicher Endkundenverkauf noch eine Verkaufspreisliste. „Keine öffentlichen Verkaufspreise gefunden“ ist als Suchbefund zu lesen, nicht als Beweis, dass kein Verkauf stattfindet. Auf den hier gelesenen Standort-/Leistungsseiten keine numerischen Verkaufspreise.
- **Gebühren:** Keine veröffentlichten numerischen Standort-/Container-/Entsorgungstarife auf [1,5] gefunden. Nicht gleichbedeutend mit kostenloser Entsorgung/Abholung. Die 5-Euro-Schwelle und fehlende Samstags-Barauszahlung nicht als Gebühr modellieren.
- **Zeiten:** Reguläre Frontmatter-Zeiten stimmen. Sonder-Samstage nicht zu regelmäßigen Samstagsöffnungen verallgemeinern. Der datierte 09.10.-Schließtag ist am Reviewtag vergangen und kein Grund für `geschlossen`.

## Konkrete Restpunkte vor Freigabe

1. **Inhaltliche Nacharbeit (mittlere Relevanz): Auszahlungsbedingungen fehlen.** Dossier Zeilen 15, 27, 29, 35 formuliert Annahme/Ankauf und Preise, lässt aber die auf der direkt verlinkten Leistungsseite veröffentlichte „Vergütung ab einem Materialwert von 5 Euro“ vollständig weg. Auch die bei den ausdrücklich erwähnten Sonder-Samstagen geltende fehlende Barauszahlung fehlt. Beides im neuen Recherchekontext mit Quelle ergänzen; Schwelle als Betreiber-Leistungsbedingung kennzeichnen, nicht als pauschale Annahmeverweigerung unter 5 Euro auslegen. Annahme nach Möglichkeit zugleich in `dropoff_json` mit Kundenarten und belegtem Wochenzeitfenster abbilden; nicht aus Kalender-Sondertagen einen Dauerplan konstruieren. Die konkrete JSON-Vervollständigung ist eine Qualitätsverbesserung, nicht allein ein harter Freigabeblocker; die fehlenden finanziellen Bedingungen sind der Grund für die inhaltlich bedingte Freigabeempfehlung.
2. **Nachweis im Dossier vervollständigen:** Der neue Bullet nennt bislang weder das HRB-Impressum noch die konkrete Umfirmierungsbekanntmachung. Diese Reviewquellen machen die Entscheidung nachvollziehbar; für die dauerhafte dossiernahe Beweiskette sollten [2,4,5] in einen ergänzenden aktuellen Timeline-Befund aufgenommen werden. Nicht rückwirkend behaupten, diese Seiten seien bereits am 13.10. neu geprüft worden. Kein neuer Identitätsblocker, sondern Dokumentationsnacharbeit.
3. **Technisches Owner-Gate offen:** `cargo test -p schrott-mcp-ingestion seed` hier bewusst nicht ausgeführt; kein Test-Pass behauptet. Der Workspace enthält zahlreiche fremde Änderungen. Vor Integration muss der Owner den beabsichtigten Änderungssatz auswählen und darauf den README-Test erfolgreich ausführen. Ein späterer Gesamtlauf im ungefilterten Workspace wäre kein isolierter Pedack-Nachweis.

**Keine zusätzlichen Blocker erfinden:** Die in der Timeline genannten Koordinaten wurden weder neu vermessen noch in Produktion abgefragt. Der Diff ändert Adresse und GEO nicht und enthält kein `lat`/`lon`-Paar; daraus folgt kein Bedarf für ein ungefragtes Datenbankupdate. Kunden-Einfahrt/Kartenpin bleibt eine offen deklarierte Genauigkeitsfrage, kein Gegenbeweis zum belegten Standort. Eine vollständige Produktionsabnahme ist aus diesem Dateireview nicht ableitbar.

## Ausführungsgrenze

Nur dieser Bericht wurde geschrieben. Dossier, Holding-Dossier und sonstige vorhandene Änderungen blieben unangetastet; kein Commit, Push, Deploy und keinerlei Produktionsdatenbankzugriff. Auftrag als Quellen-/Integrationsreview abgeschlossen, erforderliche Nacharbeit und Release-Gates ausdrücklich offen.
