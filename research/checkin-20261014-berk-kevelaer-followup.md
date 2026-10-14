# BERK Kevelaer – Register-/Standort-Follow-up 14.10.2026

## Ergebnis und Owner-Entscheidung

**Das bisher offene Registergate ist heute mit amtlichen AD/CD erfolgreich geschlossen.** HRB 18672 beim Amtsgericht Kleve ist ein aktuelles Registerblatt der **BERK Recycling GmbH**, Sitz Kevelaer; Geschäftsführer ist Marius Kohl. Die frühere Firma **BERK GmbH** ist dieselbe Rechtsperson, keine zusätzlich zu erfassende Niederlassung. Betreiber-Kontakt und echte Kevelaer-Schrotthandelsdetailseite bestätigen Adresse, Telefon, Mail und Zeiten. Aktuelle Betreiber-Sitemapdaten liefern ein datiertes Aktualitätssignal. Ein gültiges Kevelaer-Auditzertifikat oder ein standortspezifischer Genehmigungsbescheid wurde dagegen **nicht** nachgewiesen.

**Wichtiger neuer Gegenbefund:** Der amtliche AD nennt **„Velderdyck 21, 47625 Kevelaer“**, während Impressum/Kontakt/Kevelaer-Detailseite **„Velder Dyck 21, 47624 Kevelaer“** nennen. Die Straßen-Schreibweise und insbesondere PLZ sind nicht wortgleich. Keine Verlegung und keine Korrektur des Registers behaupten. Register-Geschäftsanschrift und veröffentlichte Anlieferadresse getrennt dokumentieren.

Konkrete Empfehlung für einen späteren Owner-Patch, **hier nicht ausgeführt**:

| Feld | Empfohlener Wert / Entscheidung | Grundlage und Grenze |
|---|---|---|
| `name` | BERK Recycling GmbH beibehalten | Amtlicher aktueller AD und Betreiber-Impressum kongruent [R1, P1]. |
| `street` | `Velder Dyck 21` | Als Anliefer-/Standortadresse nach Betreiber übernehmen; abweichende Register-Schreibweise in Timeline erhalten [P2, P3, R1]. |
| `postcode` | `47624` | Betreiber-Standort-PLZ, **nicht registerbestätigte PLZ**. Freigabe nur mit ausdrücklicher Owner-Akzeptanz dieses Gegenbefunds; sonst nur dieses Feld bis postalischer/Betreiber-Klärung offenlassen [P2, P3, R1]. |
| `email` | `info@berk-recycling.com` | Allgemeine Kontaktmail im Kevelaer-Kontaktblock und Impressum; `vertrieb@berk-recycling.com` ist zusätzlich die Material-/Preisanfragenadresse auf P3, kein Widerspruch [P1–P3]. |
| `opening_hours` | `Mo–Fr 08:00–16:30; Sa geschlossen` | Kevelaer-spezifisch in Kontakt und Anlieferdetail; Sonntag nicht ausdrücklich ergänzt [P2, P3]. |
| `phone` | `02832 78597` statt `02832 78597)` | Ziffern unverändert, nur unbelegte Schlussklammer entfernen. Betreiber schreibt die Nummer ohne Klammer und verlinkt `tel:+49283278597` [P1–P3]. Keine zweite unabhängige Telefonquelle behaupten. |
| Zertifizierungen | Keine EFB-/ISO-Zertifizierung ergänzen | Kein Zertifikat samt Geltungszeitraum, Anlage und Kevelaer-Zuordnung gelesen; BImSchG-Selbstangabe ist kein Auditzertifikat. |
| Status/Website | Keine Herabstufung wegen früherem Register-Abrufblocker | Aktuelles Registerblatt und ausführliche lebende Standortseite; heutige Registerbestätigung ersetzt nicht den Nachweis einer einzelnen Umweltgenehmigung. |

README §§ Quellenstandard (Zeilen 180–199) angewandt: Viele Seiten desselben Betreibers bleiben **eine** Quelle. Für die Identität liegen jetzt amtliches Register und Betreiber als unabhängige Belege vor. Die Betreiber-Ausnahme für eigene Standortfakten ist vertretbar: Impressum mit Firma/HRB/Ort; amtlich aktuelles Blatt; echte per-site Adresse/Telefon/Zeiten; datierte Aktualitätssignale. Zertifikate sind eine mögliche, nicht die einzige Form des Aktualitätssignals. Die PLZ-Abweichung trotzdem nicht unter dieser Ausnahme verstecken. Falls der Owner Sitemap-`lastmod` nicht als hinreichendes Aktualitätssignal anerkennt, greift wieder der Zwei-Quellen-Standard; dann Mail/Zeiten/Telefon nicht als doppelt unabhängig belegt ausgeben.

## Amtliche Registerlage: heute tatsächlich abgerufen

### R1 – Handelsregister, aktueller Abdruck (AD)

- Einstieg: <https://www.handelsregister.de/rp_web/welcome.xhtml>
- Formularnavigation „Erweiterte Suche“: <https://www.handelsregister.de/rp_web/erweitertesuche/welcome.xhtml>
- Suche: Registergericht **Kleve** (`R1304`), Registerart **HRB**, Nummer **18672**, einschließlich geschlossener Blätter.
- Ergebnis: **1 Treffer**, Nordrhein-Westfalen, Amtsgericht Kleve HRB 18672, BERK Recycling GmbH, Kevelaer, Status **„aktuell“**; Historie BERK GmbH.
- Ergebnisroute: <https://www.handelsregister.de/rp_web/sucheErgebnisse/welcome.xhtml?cid=1> (sessionabhängig, **kein stabiler Dokument-Permalink**).
- AD-Download: HTTP 200, `APPLICATION/OCTET-STREAM`, Dateiname **`NW-Kleve_HRB_18672+AD-20261014150409.pdf`**, 35.231 Byte. Dokumentkopf: **„Abruf vom 14.10.2026 15:04“**, zwei Seiten. PDF im Arbeitsspeicher mit `pdftotext -layout - -` gelesen; keine zusätzliche Datei angelegt.

Relevanter gelesener Inhalt:

> Anzahl der bisherigen Eintragungen: 3. Firma: BERK Recycling GmbH. Sitz: Kevelaer. Geschäftsanschrift: Velderdyck 21, 47625 Kevelaer. Gegenstand: Handel, Transport und Bearbeitung von Abfällen und Metallen, Containerdienst, Transportdienstleistungen, Entrümpelungen und Demontage. Stammkapital: 28.500,00 EUR. Geschäftsführer: Kohl, Marius, Kevelaer; einzelvertretungsberechtigt und mit Befreiung im Sinne der ausgewiesenen §-181-Regelung. Prokura: ---. GmbH; Gesellschaftsvertrag vom 23.12.2022, zuletzt geändert durch Beschluss vom 05.12.2024. Sonstige Rechtsverhältnisse: ---. Tag der letzten Eintragung: 27.01.2025.

Der heutige AD enthält keinen Auflösungs-/Löschungsvermerk. Das ist ein tatsächlicher aktueller Registerabruf, **nicht** bloß die HRB-Selbstangabe im Impressum oder ein Suchmaschinen-Snippet. Der letzte Eintragungstag 2025 ist nicht der Abruftag und kein Hinweis auf ein geschlossenes Blatt.

### R2 – Handelsregister, chronologischer Abdruck (CD)

- Gleiche Suche, danach CD-Link des einzigen Treffers.
- HTTP 200, Dateiname **`NW-Kleve_HRB_18672+CD-20261014150447.pdf`**, 33.597 Byte; Kopf **„Abruf vom 14.10.2026 15:04“**, eine Seite; ebenfalls ausschließlich im Speicher gelesen.
- Eintragung 1, **08.03.2023**: BERK GmbH, Kevelaer, Geschäftsanschrift Velderdyck 21, 47625 Kevelaer; 25.000 EUR; Marius Kohl als Geschäftsführer.
- Eintragung 2, **29.03.2023**: Firma BERK Recycling GmbH; Gesellschafterversammlung vom 07.02.2023 änderte § 1 Ziffer 1 und die Firma.
- Eintragung 3, **27.01.2025**: Stammkapital 28.500 EUR; Beschluss vom 05.12.2024 erhöht um 3.500 EUR.

**Betreiberkette:** BERK GmbH → BERK Recycling GmbH ist amtlich belegt. Die Unternehmensseite erzählt außerdem die berufliche Vorgeschichte bei Kohl Containerdienst/EGN [P4]. Daraus folgt **keine** Rechtsnachfolge, Fusion oder Umbenennung der Kohl Containerdienst GmbH in BERK; eine solche Kette nicht ergänzen.

### Abrufhindernisse korrekt eingeordnet

Ein direkter GET auf die zunächst vermutete Route `https://www.handelsregister.de/rp_web/erweitertesuche.xhtml` ergab HTTP 400. Die reguläre Navigation mittels Cookie-Session und JSF-Formular-POST von der Startseite führte dagegen erfolgreich zur tatsächlichen Suchroute und AD/CD. Deshalb **nicht mehr pauschal „Register live blockiert“** für diesen Follow-up festhalten. Die allgemeinen Dialogtexte „Abgelaufene Session“/„Fehler“ liegen auch im gelieferten Seitentemplate; entscheidend waren der sichtbare Treffer und tatsächlich erhaltene PDFs.

<https://www.unternehmensregister.de/> war lesbar, aber dort wurde kein firmenspezifischer Auszug abgerufen; die Startseite ist kein zusätzlicher Identitätsbeleg.

### R3 – Northdata: heutiger Gegencheck, nicht erforderlich für das amtliche Ergebnis

<https://www.northdata.de/BERK+Recycling+GmbH,+Kevelaer/Amtsgericht+Kleve+HRB+18672> war bei diesem Abruf textlich lesbar (anders als die im Dossier protokollierte frühere 429-Antwort). Gelesen: Firma BERK Recycling GmbH, früher BERK GmbH, HRB 18672, EUID `DER1304.HRB18672`, Adresse Velderdyck 21, D-47625 Kevelaer; Publikationen 29.03.2023 Namensänderung und 27.01.2025 Kapital 28.500 EUR sowie Jahresabschluss-Veröffentlichung vom 14.01.2026 zum 31.12.2024. Das bestätigt die Kongruenz zur amtlichen Registeradresse, **nicht** die Betreiber-PLZ 47624. Keine amtliche Primärquelle und kein eigener zusätzlicher Telefon-/Zeitenbeleg; Entscheidung stützt sich auf R1/R2.

## Aktuelle Kevelaer-Primärquellen

Alle folgenden HTML-Seiten wurden am 14.10.2026 direkt gelesen; P1–P7 zusätzlich per HTTP-Abruf auf **200** geprüft. Keine Aggregatorangaben gezählt.

| ID | Quelle | Aussage |
|---|---|---|
| P1 | <https://berk-recycling.com/impressum/> | BERK Recycling GmbH; Marius Kohl Geschäftsführer; Amtsgericht Kleve HRB 18672; Velder Dyck 21, 47624 Kevelaer; 02832 78597; info@berk-recycling.com. |
| P2 | <https://berk-recycling.com/kontakt/> | Eigener Hauptblock „Niederlassung Kevelaer“ mit genau dieser Adresse, Telefon, Mail und Mo–Fr 08:00–16:30/Sa geschlossen. Separater Grefrath-Block mit anderen Zeiten/Telefon; diese nicht nach Kevelaer kopieren. |
| P3 | <https://berk-recycling.com/services/schrotthandel-kevelaer/> | Echter Standort-/Anlieferdetailtext, nicht nur Footer: Anlieferadresse und Zeiten; Privat, Gewerbe, Industrie; Personalausweis; größere Mengen telefonisch ankündigen; Sortierung/Verwiegung; sofort bar oder Überweisung, Gewerbe auf Wunsch Gutschrift. Vertriebs-Mail `vertrieb@berk-recycling.com`. |
| P4 | <https://berk-recycling.com/unternehmen/> | Betreiber-Selbstdarstellung, Gründung 2023 durch Dirk Bergerfurth und Marius Kohl; zwei Niederlassungen; BImSchG-genehmigter Recyclingbetrieb als Selbstangabe. Keine Register-Gesellschafterliste gelesen. |
| P5 | <https://berk-recycling.com/containerdienst-kevelaer/> | Eigener Hof am Velder Dyck 21, Selbstanlieferung kleinerer Mengen Bauschutt/Grünschnitt/Altholz/Schrott Mo–Fr 08:00–16:30; Container-Festpreis/Shop und Mietbedingungen, siehe unten. |
| P6 | <https://berk-recycling.com/schrottcontainer/> | Konditionen explizit: Stellung/Transport können berechnet oder mit Schrottwert verrechnet werden; abhängig von Menge, Sorte, Standzeit. Schrotterlös zum Tagespreis der Abholung. |
| P7 | <https://berk-recycling.com/services/entsorgung/> | Anlieferung/Abholung/Container für Abfälle und Metallankauf; keine belastbare Aussage, dass jede Abfallannahme vergütet oder gratis ist. |
| P8 | <https://berk-recycling.com/page-sitemap.xml> | Betreiber-generierte `lastmod`: P3 **2026-09-27T11:44:49+00:00**, P5 **2026-10-05T10:42:26+00:00**, P6 **2026-09-27T11:45:18+00:00**, P2 **2026-08-03T15:50:38+00:00**. Aktuelle Bearbeitungssignale, keine Garantie einer vor Ort kontrollierten Öffnungszeit. |
| P9 | <https://berk-recycling.com/wp-sitemap.xml> | Sitemapindex mit jüngsten Seiten-/Beitragsänderungen im September/Oktober 2026. Ergänzendes Aktualitätssignal, keine weitere unabhängige Quelle. |

P1 enthält noch „§ 5 TMG“ und einen OS-Plattform-Verweis. Nicht allein aus dem Impressumsboilerplate bzw. automatisch gesetztem Copyright 2026 auf gepflegte Inhalte schließen; das Aktualitätssignal kommt hier aus datierten spezifischen Leistungsseiten/Sitemap, nicht aus diesen Standardtexten.

## Zertifikate / Umweltgenehmigung: offen, nicht widerlegt

- P4 und Betreiber-Footer sprechen von BImSchG-genehmigtem Recyclingbetrieb; Startseite von zwei genehmigten Standorten und „zertifiziert“. Das sind **Selbstangaben**, kein gelesener behördlicher Genehmigungsbescheid und keine EFB-Zertifikatsanlage für Kevelaer.
- In den gelesenen Roh-HTML-Verweisen von P1–P7 fanden sich **keine** Links mit `.pdf`, `zertifikat`, `zertifiz` oder `download`. In der Seiten-Sitemap wurde keine entsprechende Download-/Zertifikatsseite gefunden. Das ist ein begrenzter Suchbefund, keine Aussage, dass der Betreiber überhaupt kein Zertifikat besitzt.
- Versuch <https://fachbetrieberegister.zks-abfall.de/fachbetrieberegister/>: Webabruf meldete **Transportfehler**, kein auswertbarer Registerinhalt. Keine Negativsuche/kein „nicht zertifiziert“ daraus ableiten.
- Gezielte Websuchen zu BERK/Zertifikat und BERK/Kreis Kleve/Genehmigung lieferten keinen gelesenen firmenspezifischen amtlichen Beleg. Treffer anderer Firmen nicht übertragen.
- Owner kann bei Bedarf aktuelles Zertifikat **mit Kevelaer-Anlage**, Zertifizierer, Nummer, Tätigkeiten/AVV und Gültigkeitsende bzw. behördlichen Bescheid mit Betreiber und Standort anfordern. BImSchG-Genehmigung, EFB-Zertifizierung und ISO-Zertifizierung sind verschiedene Aussagen. Das offene Zertifikatsgate blockiert die Aufnahme einer Zertifizierung, nicht automatisch die bestätigten Kontaktfakten.

## Ankauf, Verkauf, Gebühren getrennt

1. **Ankauf durch BERK:** P3 nennt Kupfer, Kabel, Aluminium, Messing/Rotguss, Edelstahl, Eisen/Stahl/Mischschrott sowie Blei, Bleibatterien, Zink und Zinn. Vergütung nach Sorte/Gewicht/Reinheit/Menge, bar oder Überweisung. Tagespreis am Telefon, bei Selbstanlieferanfrage gültig für den Anruftag; bei Schrottcontainer laut P6 Tagespreis der Abholung. **Keine numerische veröffentlichte Ankaufsliste** aus diesen Seiten erfasst. Werbeformulierung „Tageshöchstpreise“ nicht als verifizierten Marktvergleich übernehmen; Kundenbewertungs-Zahlen nicht als Preisliste erfassen.
2. **Verkauf durch BERK:** Keine numerische Schrott-/Metall-Verkaufspreisliste in den gelesenen Quellen. Seitentitel/CTA „Schrott verkaufen“ richtet sich an den Kunden und bezeichnet BERKs **Ankauf**, nicht Händler-Verkaufspreise.
3. **Gebühren/Dienstleistungsverkäufe:** P5 verweist Privatkunden auf den Shop für Festpreise einschließlich Lieferung, Abholung und Entsorgung; sieben Tage mietfrei, danach wöchentliche Miete (keine Zahl auf P5). Der verlinkte Shop wurde hier nicht ausgelesen: daher **nicht behaupten, Containerpreise seien nirgends öffentlich**, sondern „keine Zahlen aus den geprüften Seiten erhoben“. P6 sagt ausdrücklich, dass Stellung und Transport berechnet oder verrechnet werden können. Container-/Abholung nicht generell kostenlos modellieren.
4. **Kostenfreie Annahme:** Der Betreiberbanner gilt ausdrücklich nur für **Pappe und Papier** in Kevelaer/Grefrath. Kein Metallpreis, keine allgemeine Gebührenfreiheit und keine Auszahlungsgarantie für sämtliche Abfallarten.

## Übergabegrenze

Gelesen: `dossiers/nw/nw-kevelaer-berk-recycling.md` und README-Quellenstandard. Geschrieben wurde ausschließlich diese Research-Datei. Keine Dossier-, Rust- oder GEO-Änderungen, keine Datenbankzugriffe, Commits, Pushes oder Deploys. Das Ergebnis ist ein Owner-Handoff, keine Behauptung einer bereits erfolgten DB-/Produktionskorrektur.
