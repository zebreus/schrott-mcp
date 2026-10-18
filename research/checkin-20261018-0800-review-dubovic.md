# Review Dubovic / Annweiler — 18.10.2026

## Entscheidung für den Owner

**Bedingt freigeben:** Der gegen HEAD vorhandene Frontmatter-Diff ist exakt zulässig:

```diff
-street: Dubovic Annweiler Annweiler Hauptstr. 51
+street: ''
```

Das ist der Rückzug eines erkennbar zusammengesetzten, unbelegten Cluster-/Parsewerts, **keine** Bestätigung, dass Hauptstraße 51 falsch oder der Betrieb nicht existent sei. Deshalb braucht es für diese Rücknahme keine erfundene Ersatzadresse. Keine Normalisierung auf `Hauptstr. 51`, kein Name-Fill `Ramadan Dubovic`, kein Telefon-Fill. Alle anderen Frontmatter-Werte unverändert lassen, insbesondere `status: pruefung`, `website_status: unbekannt`, leere Website und `postcode: '76855'` (übernommener geographischer Ortsbezug, nicht neu bestätigte Händleranschrift). Slug unverändert. Keine `clear_fields`-Ergänzung: street ist dafür nicht zugelassen.

**Text-Diff noch nicht unverändert integrieren:** Datierung und Quellenzugriff präzisieren, Sarnstall-Zuordnung relativieren und vermeintliche Negativaussage zur Verkaufspreisliste vermeiden. Die Betreiberidentität bleibt offen; die Adressgegenseite wurde untersucht, aber kein aktueller Betreiber an dieser Hausnummer qualifiziert bestätigt. Keine Schließung, kein Split, kein Zusammenführen mit anderen Annweiler Betrieben ableitbar.

## Grundlage und Scope

Gelesen: `README.md` (insbesondere Feedback-Triage, Zwei-Seiten-Prüfung und Quellenhierarchie), `docs/trader-seed.md`, das vollständige Dossier und `git diff HEAD -- dossiers/rp/rp-annweiler-dubovic-annweiler.md`. Das Dossier wurde nicht verändert. Keine Produktionsdatenbank benutzt, keine DB-Mutation, kein Geocoder-/OSM-/Nominatim-Aufruf, keine Koordinaten erhoben oder geändert. Nur diese Reviewdatei geschrieben.

README: Aggregatoren sind Leads, auch mehrere ergeben keine zwei qualifizierten Betreiberbelege. Ein negatives Suchergebnis ist keine negative Registerauskunft. Identitäts-/Adressbehauptungen sind deshalb von einer bloßen Fehlerwert-Rücknahme zu trennen.

## Eigenständige zweiseitige Prüfung

### 1. Name/Betreiber → Adresse

- Websuche `"Dubovic" "Annweiler"`: Cylex-Snippet nennt „Ramadan dubovic“, „Haupt str. 51“, 76855 Annweiler am Trifels und 0162 6739221. **Cylex-Detailabruf scheiterte mit HTTP 403.** Nicht als gelesene Detailseite oder aktuelle Betreiberbestätigung darstellen.
- Tatsächlich gelesene 11880-Detailseite führt **„Ramadan dobovic“** (abweichende Schreibweise), Hauptstr. 51, 76855 Annweiler, dieselbe Telefonnummer und Kategorie Altmetallrecycling. Sie bietet „Website hinzufügen“/„E-Mail-Adresse hinzufügen“ und enthält keine verifizierte Betreiberwebsite. Seitenangaben: „aktualisiert am 03.10.2026“, „Eintragsdaten vom 28.03.2025“. Beides sind Aggregatormetadaten, weder Abrufdatum dieses Reviews noch Nachweis aktuellen Betriebs.
- Infobel-Suchsnippet nennt „Ramadan dubovic“ an Hauptstraße 51. Der direkte Abruf lieferte jedoch nur eine Zugriffs-/Captcha-/AGB-Sperrseite, **nicht** die Händlerdetails. Nicht als gelesener Firmenbeleg zählen.
- Suchen `"Ramadan Dubovic" Schrott Impressum`, `"Dubovic" "Annweiler" register OR northdata OR creditreform` und `"Ramadan dobovic"` ergaben keinen belastbaren Betreiber-/Registertreffer. Keine direkte Registerprüfung und keine Aussage „nicht eingetragen“ daraus ableiten.

Ergebnis: konkreter, plausibler Recherchekandidat und ein Namensvarianten-Klärfall; keine qualifizierte Identitätsbestätigung. Dubovic/Dobovic nicht stillschweigend gleichsetzen oder umbenennen. Telefonnummer nicht in operative Felder übernehmen.

### 2. Adresse → möglicher anderer Betreiber / Ortslage

- Eigenständige Adresssuchen nach `"Hauptstr. 51" "Annweiler"`, `"Hauptstraße 51" "Annweiler" Schrott` und zusätzlich ohne große Aggregatoren liefern denselben Ramadan-Lead sowie Gebäude-/Denkmalhinweise. Kein qualifizierter anderer aktueller Schrottbetreiber an genau dieser Adresse gefunden. Das ist **kein** Nachweis, dass dort kein anderer Betreiber existiert.
- **Primärquelle neu gelesen:** GDKE Rheinland-Pfalz, „Nachrichtliches Verzeichnis der Kulturdenkmäler, Kreis Südliche Weinstraße“, Dokumentstand **07.12.2023**, gedruckte Seite **6**, Abschnitt **Annweiler am Trifels**: „Hauptstraße 51 — spätbarocker Krüppelwalmdachbau, bez. 1781; rückwärtig Gerbhaus, Sandsteinquaderbau, um 1900“. Damit existiert ein amtlicher Gebäudeadressbezug im Annweiler-Hauptabschnitt; das ist weder Betreiber-/Gewerbenachweis noch Beleg gegen einen dort ansässigen Kleinbetrieb. Der separate Abschnitt „Annweiler am Trifels - Sarnstall“ steht auf Seite 8. Er erlaubt keine Gleichsetzung des Ramadan-Leads mit Sarnstall. Weitere Hauptstraße-51-Treffer im Kreisdokument gehören anderen Ortsabschnitten; nicht quer zuordnen.
- Tatsächlich gelesene VG-Primärseite bestätigt lediglich: Sarnstall ist einer der Stadtteile Annweilers; Verwaltungsadresse 76855 Annweiler. Sie weist Dubovic weder einer Adresse noch einem Stadtteil zu.
- Die alte Timeline-Behauptung vom 04.10. „OSM führt Hauptstraße 51 in Sarnstall“ wurde **nicht** neu verifiziert. Sie darf nicht zur aktuellen Schlussfolgerung „Hauptstraße 51 liegt für diesen Betrieb in Sarnstall“ werden. Der neue amtliche Fund macht diese Verknüpfung zusätzlich klärungsbedürftig. Kein GEO nötig oder zulässig.
- Suchtreffer zu MF Schrotthandel & Entrümpelung (Altenstr., Lead) und CRONIMET CREMETAL (In den Bruchwiesen 17, 11880-Lead) betreffen andere Anschriften. Keine Identitäts-, Umzugs- oder Nachfolgerkette zu Dubovic belegt; nicht vermischen.

## Nötige Text- und Datierungskorrekturen

1. **Neue Korrektur auf 18.10.2026 datieren.** Der uncommittete Diff ergänzt `### Korrektur 12.10.2026` und `[Korrektur 12.10.2026: ...]`. Für eine tatsächliche damalige Korrektur liegt in diesem Review kein datierter Arbeitsbeleg vor. Nicht rückdatieren. Falls der Owner einen solchen Beleg besitzt, alte Beobachtung separat erhalten und heutige Nachprüfung zusätzlich datieren; ansonsten beide neuen Daten auf 18.10.2026.
2. Cylex ausdrücklich **Suchtreffer/Snippet** nennen, Direktabruf 403; neuen gelesenen 11880-Lead einschließlich Schreibweise „dobovic“ aufnehmen. Mehrere Aggregatoren bleiben Leads, nicht „zwei Quellen bestätigt“.
3. `**Verkauf:** keine Preisliste.` ändern in `**Verkauf:** keine Preisliste verifiziert.` Für Website/Identität gibt es keinen vollständigen Zugriff, der eine Existenznegation trägt. Analog Gebühren nur als „keine verifizierten Angaben“ bezeichnen.
4. Sarnstall nicht als belegte Lage der Kandidatenanschrift darstellen. Die Historie vom 04.10. bewahren, aber mit heutiger Einordnung überholen. Die alte Ankündigung automatischer Neu-Geocodierung ausdrücklich als **nicht geltend** kennzeichnen: README/Seed-Dokumentation sehen kein Rust-Geocoding vor; street-Leerung korrigiert oder löscht bestehendes DB-GEO nicht.
5. Nicht behaupten, die Street-Entfernung sei schon produktiv. Es handelt sich bislang um einen uncommitteten Dossier-Diff. `docs/trader-seed.md` Z. 88–105: leerer street-Wert ersetzt bei geändertem Seed die DB-Straße; fehlendes Koordinatenpaar erhält vorhandenes GEO. Rebuild/Rollout und gegebenenfalls read-only Produktionskontrolle liegen beim Owner, nicht in diesem Review.

### Vorschlag Überblick

> Der Seed-/Clustername „Dubovic Annweiler“ ist nicht als vollständige Betreiberidentität bestätigt. Ein Cylex-Suchtreffer nennt „Ramadan dubovic“, Hauptstr. 51, 76855 Annweiler, Tel. 0162 6739221; der Direktabruf war am 18.10.2026 mit HTTP 403 blockiert. Die gelesene 11880-Seite führt dieselbe Anschrift und Telefonnummer unter „Ramadan dobovic“. Beide sind Aggregator-Leads, keine qualifizierten Betreiberbelege. Der zusammengesetzte street-Clusterwert wurde im Dossier zurückgezogen, ohne eine Ersatzadresse oder Nichtexistenz zu behaupten. Postcode 76855 bleibt geographischer Ortsbezug, keine bestätigte Händleranschrift. Status `pruefung`.

Die vorhandenen Stichpunkte mit unbekannten Leistungen beibehalten, aber Verkauf/Gebühren wie oben präzisieren. GEO-Stichpunkt: „Keine Betreiberadresse oder Händlerkoordinaten bestätigt; keine Neu-Geocodierung aus diesem Klärfall.“

### Vorschlag neue Timeline (statt ungesicherter Rückdatierung)

> [Korrektur 18.10.2026: Der zusammengesetzte street-Wert „Dubovic Annweiler Annweiler Hauptstr. 51“ wurde als unbelegte Clusterzeichenfolge zurückgezogen, nicht durch eine bestätigte Anschrift ersetzt. Cylex nennt im Suchtreffer „Ramadan dubovic“; Direktabruf HTTP 403. Die gelesene 11880-Detailseite führt „Ramadan dobovic“, Hauptstr. 51, 76855 Annweiler und 0162 6739221. Diese Angaben bleiben Aggregator-Leads; Name und Telefonnummer werden nicht übernommen. Die amtliche GDKE-Denkmalliste (Stand 07.12.2023, S. 6) führt Hauptstraße 51 im Abschnitt Annweiler am Trifels als Gebäude, ohne Dubovic-/Gewerbezuordnung. Die frühere Sarnstall-Ortsannahme bestätigt daher keine Betreiberanschrift. Die VG-Seite bestätigt nur Sarnstalls Zugehörigkeit zu Annweiler. Betreiberidentität und Adressgegenseite bleiben ungeklärt; kein Ankauf, Verkauf oder Tarif verifiziert, status pruefung. Der frühere Neu-Geocodierungsvermerk ist keine aktuelle Freigabe: keine GEO-Aktion, vorhandenes DB-GEO durch diese Dossierkorrektur nicht bereinigt. Quelle(n): https://www.11880.com/branchenbuch/annweiler-am-trifels/060440092B106626886/ramadan-dobovic.html (gelesener Aggregator-Lead); https://web2.cylex.de/firma-home/ramadan-dubovic-13844658.html (nur Suchtreffer, Direktabruf 403); https://gdke.rlp.de/fileadmin/gdke/Wer_wir_sind/Landesdenkmalpflege/Denkmalliste/Kreis_Suedliche_Weinstrasse.pdf (Gebäude-/Adresskontext, kein Betreiberbeleg); https://www.vg-annweiler.de/Ortsgemeinden/Annweiler-am-Trifels/ (nur kommunaler Ortskontext)]

## Abrufprotokoll — gelesene Inhalte versus Leads

Alle nachfolgenden Abruf-/Suchbeobachtungen **18.10.2026**:

| URL | Tatsächlicher Zugriff und Beleggrenze |
|---|---|
| https://www.vg-annweiler.de/Ortsgemeinden/Annweiler-am-Trifels/ | Seiteninhalt gelesen; kanonische Links klein geschrieben `/ortsgemeinden/annweiler-am-trifels/`; Sarnstall als Stadtteil. Kein Dubovic-Beleg. |
| https://www.11880.com/suche/altmetallrecycling/annweiler | Ergebnislisteninhalt gelesen; Ramadan **dobovic**, Adresse/Telefon/Kategorie. Nur Lead. |
| https://www.11880.com/branchenbuch/annweiler-am-trifels/060440092B106626886/ramadan-dobovic.html | Detailinhalt gelesen; gleiche Daten, Datumsmetadaten, keine Betreiberwebsite. Nur Lead. |
| https://gdke.rlp.de/fileadmin/gdke/Wer_wir_sind/Landesdenkmalpflege/Denkmalliste/Kreis_Suedliche_Weinstrasse.pdf | webfetch konnte PDF nicht darstellen; anschließend Original-PDF via Python urllib im Speicher heruntergeladen und mit `pdftotext -layout - -` gelesen. S. 4–6 Abschnitt Annweiler sowie S. 8 Sarnstall geprüft. Kein Datei-Download ins Repo, kein GEO. Amtlicher Gebäude-/Adresskontext. |
| https://web2.cylex.de/firma-home/ramadan-dubovic-13844658.html | Suchsnippet gelesen; direkter Abruf HTTP 403. Nur Lead. |
| https://www.infobel.com/de/germany/ramadan_dubovic/annweiler_am_trifels/DE108985193/businessdetails.aspx | Suchsnippet gelesen; direkter Abruf nur Sperr-/AGB-Seite, keine Händlerdetails. Nur Lead. |
| https://commons.wikimedia.org/wiki/File:Annweiler_Hauptstr_51.jpg | Nur Suchtreffer gesehen, nicht aufgerufen; kein Betreiberbeleg. |
| https://lokaleschrottplatz.de/rheinland-pfalz/annweiler-am-trifels/ | Nur Suchsnippet gesehen, nicht aufgerufen; keine Übernahme generischer „Zertifiziert“-/Materialbehauptungen. |
| https://web2.cylex.de/firma-home/mf-schrotthandel-_-entruempelung-15403392.html | Nur Suchsnippet gesehen, nicht aufgerufen; anderer Adresslead, keine Verbindung belegt. |

Zusätzliche Websuchen nach vollständiger Telefonnummer außerhalb der großen Aggregatoren sowie Facebook wurden wegen HTTP-401-Authentifizierungsfehlern nicht ausgeführt. Aus diesen Fehlversuchen keine Nulltrefferbehauptung ableiten. Die alte OSM-Way-/Nominatim-URL und Northdata-Suchseite aus der Dossierhistorie wurden nicht neu aufgerufen. Keine qualifizierte Register-/Betreiberquelle für Ramadan Dubovic/Dobovic gefunden oder gelesen; die Recherche bleibt in dieser Kernfrage offen.

## Owner-Gate

- Nur die eine oben genannte Scalar-Änderung freigegeben; ehrliche, auf 18.10. datierte Textkorrektur dazu.
- Kein Nachweis eines aktuellen Händlers durch bloßen Orts-/Gebäudebeleg; vorhandene Identität nicht auf `aktiv` hochstufen.
- Historische Timeline nicht löschen; heutige Evidenzgrenzen hinzufügen.
- Seed-Tests/Integration/Rollout nicht durch dieses reine Review behauptet. Produktionsdaten und GEO bleiben hier unangetastet.
