# Check-in 18.10.2026, 11:00 – Review Borgsdorf-Recycling

## Ergebnis / Integrationsentscheidung

**Bedingt integrationsfähig, nicht unverändert freigeben.** Der Neuentwurf `dossiers/st/st-zerbst-borgsdorf-recycling.md` beschreibt einen heute durch Betreiberseiten, Registerabgleich und tatsächlich abgerufenes EFB-2026-PDF belegten Schrottankäufer am **Altbuchsland 18d**. Kein lokaler Dublettenbeleg. Vor Integration die pauschale Maildarstellung korrigieren, Papier-Annahmebedingungen ergänzen und den angeblichen „Geschäftsführerwechsel 2026“ präzisieren. Bildhinweise auf der Kontaktseite bleiben eine ausdrückliche Abrufgrenze. Keine Zusammenführung mit Bornum/GbR/Kirschallee 5; keine ElektroG-Annahme am Altbuchsland ableiten.

Diese Prüfung beruht auf **neuen Abrufen am 18.10.2026**, nicht auf einer Umdatierung der Recherche vom 12.10. Die unten protokollierten PDF-Abrufe erfolgten um 11:01 UTC. Ausschließlich dieses Review geschrieben; Dossier, DB, GEO und Git-Index/Commits unverändert. Keine DB-Abfrage durchgeführt; frühere DB-Abgleichbehauptungen des Entwurfs sind hier nicht erneut bestätigt.

## Maßstab

Gelesen: `README.md`, insbesondere Zeilen 160–202 (Feedback nur Lead, zweiseitige Identitätsprüfung, Aggregatoren keine Belege, Betreiber-Ausnahme) und 204–240 (Betreiberketten, keine vorschnellen Merges). Die Betreiber-Ausnahme für eigene Standortdaten ist sachlich gut begründet: Impressum nennt Gesellschaft/HRB/Ort; NorthData bestätigt dieselbe Gesellschaft und Anschrift; Kontakt ist standortspezifisch; echte aktuelle Zertifikate sind vorhanden. Mehrere Seiten derselben Domain sind keine unabhängigen Quellen. NorthData ist ein gemäß README zugelassener Registerabgleich, hier aber **kein direkt abgerufener amtlicher aktueller Registerauszug**.

Die heute lesbare NorthData-Ausgabe zeigt zwei aktuelle Geschäftsführer, passende HRB/EUID und Bekanntmachungen bis 2026, aber keinen eindeutig auslesbaren individuellen Statuswert „Aktiv“. Die Wörter Aktiv/Liquidation/Erloschen in der Suchmaske sind **keine** Statusfeststellung für diese GmbH. Betreiberaktivität ist durch das aktuelle Angebot plausibel belegt; eine uneingeschränkt amtliche Aussage „Registerstatus heute geprüft: aktiv“ wäre zu stark. Falls der Owner hierfür ausdrücklich einen aktuellen amtlichen Auszug verlangt, bleibt dieser Gate offen.

**Nachprüfung nach Fortsetzung, 18.10.2026 um 12:02:31 UTC:** [S8] zusätzlich direkt per Python/HTTPS als HTML angefordert (HTTP 200), um einen im Markdown möglicherweise ausgelassenen strukturierten Aktivstatus zu suchen. Kein Treffer für `companyStatus`, `isActive`, `status-active` oder ein JSON-Feld `"status"`; gefunden wurde nur das Suchformularfeld `name="status"`. Die Statusgrenze ist damit nachgeprüft, nicht aufgelöst. Dies ist kein Nachweis einer Inaktivität.

## Feldweise Empfehlungen

| Feld / Aussage | Empfehlung | Heute gewonnener Beleg / Grenze |
| --- | --- | --- |
| `slug` | Beibehalten, kein neuer Alias-/Merge-Schlüssel. | Bestehender Neuentwurf; Slug-Stabilität laut README. |
| `name`, Betreiber | **Borgsdorf-Recycling GmbH** beibehalten. | Impressum [S1], NorthData [S8], EFB [S7]: HRB 15643, Stendal, Hauptsitz Altbuchsland 18 d. |
| HRB / Geschäftsführung im Überblick | HRB 15643 AG Stendal behalten; Kay und Mario als **laut aktuellem Impressum** nennen. „Geschäftsführerwechsel 2026“ ersetzen durch „personenbezogene Daten Kay Borgsdorfs geändert am 03.03.2026“. | [S1], [S8–S11]. [S9] bestellt keinen neuen Geschäftsführer, sondern ändert Personendaten. Die aktuelle NorthData-Vertreterliste ist teilweise Premium; historische Meldungen nicht allein als vollständigen heutigen Auszug ausgeben. |
| `state`, `city`, `street`, `postcode` | ST; Zerbst/Anhalt; Altbuchsland 18d; 39261 beibehalten. | [S1], [S2], [S7], [S8]. Schreibweise „18 d“ im Register/PDF entspricht derselben Hausnummer. |
| `phone` | `+49 3923 786437` beibehalten. | [S1], [S2], EFB-Anlage 5, beschriftete S. 15 [S7]. Kirschallee-Telefon 03923 487900 aus [S12] **nicht** übertragen. |
| `email` | T-Online-Adresse ist belegbar und kann bleiben, aber nicht als einzig aktuell bestätigte Adresse darstellen. Für kundenbezogenen Hauptkontakt ist `info@borgsdorf-recycling.de` nach [S2] ebenfalls ausdrücklich begründet; eine Umstellung wäre eine Owner-Entscheidung, keine technische „Korrektur“ des Mailto allein. | [S1] Text `info@borgsdorf-recycling.de` → `mailto:borgsdorf-recycling@t-online.de`; **[S2] und Startseite dagegen Text UND Mailto `info@borgsdorf-recycling.de`**. [S7] S. 15 und [S12] nennen T-Online. Kein Mailversand/MX-/Zustelltest. |
| `opening_hours` | Regulär `Mo-Fr 07:00-17:00; Sa 08:00-12:00` übernehmen, Quelle als Textangabe aus [S2] kennzeichnen. | Kontakt-/Startseitentext [S2]. Zwei September-2026-Bildhinweise nicht visuell auswertbar, daher keine Aussage „ohne Sonderausnahmen“. GbR-Samstag „nach Vereinbarung“ gehört nicht zur GmbH. |
| `website`, `website_status` | Domain und `aktiv` behalten. | Betreiber-HTML heute lesbar [S1–S6, S13–S14]; PDFs per HTTP 200 neu geholt. „Aktiv“ beschreibt Website-Erreichbarkeit, nicht garantiert jederzeitige Hoföffnung. |
| `trader_type`, `status` | `schrotthaendler`, `aktiv` für das belegte Betreiberangebot vertretbar. | Konkreter Schrottankauf privat/gewerblich und Anlieferung [S3–S4], Schrottplatz [S7]. Kein Schluss aus bloßem Branchenverzeichnis. Registerstatusgrenze siehe oben. |
| `description` | Inhaltlich behalten; Ankauf, Anlieferung und Containerdienst belegt. „Keine feste Preisliste veröffentlicht“ enger: **„In den heute lesbaren geprüften Seiten keine numerische Ankaufspreisliste gefunden; aktuelle Konditionen auf Anfrage.“** | [S3–S5]; AGB/Bilder nicht vollständig gelesen, kein Vollständigkeitsbeweis über sämtliche Webinhalte. |
| `certifications` | Vorhandene eng gefasste EFB-Zeile mit 801.0112/26, Anlage 4, Altbuchsland und 16.12.2027 beibehalten. | Tatsächlicher PDF-Inhalt [S7], nicht bloß Linktitel/Uploadjahr. Nicht zum unbeschränkten Zertifikat für alle Abfallarten oder alle Standorte erweitern. |
| `dropoff_json.allowed`, Kunden | `true`, privat/gewerbe behalten. | [S4] ausdrücklich „zu uns bringen“, Firma/Privatperson; [S2–S3] ordnen Wertstoffhof/Papier dem Hauptstandort zu. |
| `dropoff_json.days/time_windows` | Bestehende reguläre Zeitfenster plausibel behalten; keine Feiertags-/Sonderöffnungszusagen hinzufügen. | [S2], Bildgrenze siehe Abrufprotokoll. |
| `dropoff_json.conditions` | Metalle und Ausschlüsse behalten, **Papierbedingungen ergänzen**; Ausschlüsse auf Schrott-/Metallanlieferung beziehen statt pauschal auf sämtliche Entsorgungsdienste der GmbH. | [S3–S4]: keine Druckgasflaschen, E-Schrott, Kühl-/Gefriergeräte; keine Öl-/Flüssigkeits-/Chemie-, Asbest- oder radioaktiven Beimengungen im Schrott. Andere Entsorgungsangebote und Zertifikatspositionen sind kein Widerspruch dazu. |
| `pickup_json` | Bestehende bedingte Form behalten: Abholung **über Containerdienst**, Behälter liefern/gefüllt abholen, individuelle Preise. Optional Kundenarten privat/gewerbe mit [S4] explizit setzen; keine Abholtage oder Liefergebietsgrenze erfinden. | [S4–S5]. Keine freie Hausabholung, Mindestmenge oder kostenlose Leistung zugesagt. Big Pack nicht als kostenlose Schrottmitnahme behandeln. |
| Mengenfelder / Preise | Numerische Mindest-/Höchstmengen, Ankaufkurse, Gebühren leer lassen. | [S3] „kleinen und großen Mengen“ ohne Zahlen; [S4–S5] nur Tages-/Individualkonditionen. |
| Provenienz / Timeline | Feedback #6663/#5490 als Lead-Historie erhalten, nicht als Beleg verwenden. Bei späterer Integration **neuen Rechercheeintrag 18.10.2026** mit Quellen und Grenzen ergänzen, den 12.10.-Eintrag nicht umdatieren. | Feedback-IDs hier nur aus Entwurf übernommen, nicht gegen interne DB geprüft. |

### Empfohlene Präzisierung der Servicebedingungen

Für `dropoff_json.conditions` mindestens ergänzen: **„Papier nur am Altbuchsland 18d, lose und nicht gebündelt. Keine mit Speiseresten/Farbe/Öl verunreinigten Kartons oder Papiere, keine Tetrapaks, Wachspapiere oder Aktenordner; Durchschreibepapiere nur ohne Kohlepapier.“** [S2–S3] Die jetzige Formulierung „sowie Altpapier“ unterschlägt diese konkreten Bedingungen. Saubere Verpackungskartonagen, Zeitungen/Zeitschriften/Prospekte, Briefe und Umschläge sind Beispiele, kein pauschaler Freibrief für jedes Papier.

Die Materialliste ist nicht erschöpfend: [S4] nennt zusätzlich u. a. Blei, Zink, Edelstahl, Rotguss, Hartmetall und Zinn, Kabel isoliert und geschält. Nicht zwingend alle Beispiele in die JSON-Prosa kopieren; ebenso keine Qualitäts-/Ankaufgarantie aus bloßem AVV-Zertifikatsumfang ableiten.

## Identitäten und Standorte: strikt getrennt halten

1. **Altbuchsland 18d:** Hauptniederlassung der GmbH [S1–S2], EFB-Anlagen 1–7 [S7]. Anlage 4 benennt dort ausdrücklich den Schrottplatz für Eisen-/Nichteisenschrott. Dies ist der geprüfte Dossierstandort.
2. **Kirschallee 3a:** Ebenfalls GmbH, aber anderer Betriebsstandort. EFB-Anlage 8 beschreibt Zwischenlager/Wertstoffhof/Aktenvernichtung; Anlage 9 Erstbehandlung ElektroG. Das separate ElektroG-Beiblatt [S12] nennt diese Adresse und 03923 487900. Keine ElektroG-Annahme am Altbuchsland daraus folgern; keine Übertragung von Hauptstandort-Zeiten auf Kirschallee. Zertifizierungsumfang ist nicht gleich öffentliche Annahmezusage. Auch kein Schluss auf aktuelle Papierannahme in Kirschallee: [S2] sagt ausdrücklich ausschließlich Altbuchsland.
3. **Biaser Str. 21:** [S1–S2] trennen **Borgsdorf GbR**, Vermietung Baumaschinen/Anhänger, Tel. 03923 6209468, eigene USt-ID DE 242555905 von GmbH DE 271779045. Gleiche Namen Kay/Mario oder geteilte Mailto-Ziele begründen keine Rechtsträgeridentität. Kein Schrottankauf der GbR allein daraus belegt; nicht als GmbH-Standort übernehmen.
4. **Jürgen Borgsdorf, Bornum:** Das lokale Dossier ist `status: pruefung`, ohne Straße/PLZ im Frontmatter; Lange Str. 18, 39264 steht lediglich im historischen Aggregator-Lead der Timeline. [S10] belegt Ausscheiden eines Jürgen Borgsdorf als GmbH-Geschäftsführer am 08.07.2024 (dort Ortsangabe OT Bias), **nicht** den Betrieb in Bornum, nicht dessen Rechtsform und nicht eine Rechtsnachfolge. [S11] nennt Bornum bei Mario als persönliche Ortsangabe; auch das beweist keinen Händlerstandort oder Merge. Den Entwurf nicht so lesen, als sei die Bornumer Adresse primär bestätigt. Keine Bornumer Kontaktdaten übertragen.

## EFB 2026: tatsächlich gelesener Inhalt

[S7] ist ein vollständiges PDF mit vorgeschaltetem Überwachungszeichen und **beschrifteten Zertifikatsseiten 1–24**, neun Anlagen. Deckblatt: Zertifikat **801.0112/26**, Vorgang **ZZST002000170011**, Zertifizierer **Ingenieurbüro Ulbricht GmbH, Mittweida**; Prüfung **17.06.2026**, Ausstellung **18.06.2026**, gültig bis **16.12.2027**. Hauptsitz und Registerangabe stimmen mit [S1, S8] überein. PDF enthält digitale Unterschriftsdarstellungen vom 19.06.2026; ihre kryptografische Gültigkeit wurde nicht geprüft.

- **Anlage 4, beschriftete S. 10–12:** Altbuchsland 18 d, Anlagenkennung **NA82000736**; Beiblatt wörtlich „Anlage zur zeitweiligen Lagerung von Eisen- und Nichteisenschrotten – Schrottplatz – Lagern und Behandeln“. Genannt: befestigtes Freilager, Lagerboxen/Container, Halle, manuelle und Baggersortierung, Alligatorschere, Mobilbagger/Gabelstapler.
- Zugehörige AVV-Liste: 120101, 120103, 150104, 160106, 160117, 160118, 160216, **160601\*** (Bleibatterien), 170401–170407, 170411, 191202, 191203. Kein allgemeiner Beleg „alle Abfälle erlaubt“, kein Zahlungsversprechen für jede Position. Bleibatterien sind im Zertifikatsumfang, aber daraus hier kein zusätzlicher ungeprüfter öffentlicher Ankauf ergänzt.
- Weitere Anlagen betreffen Container-/Transportdienst (1), Bauschuttrecycling (2–3), Altautodemontage (5), Papier/Kunststoff (6–7) und Kirschallee (8–9). Anlage 5, S. 15, nennt T-Online-Mail/786437 und „Für keinen Hersteller“ als Bestimmung zur unentgeltlichen Altfahrzeugrücknahme. Daraus keine kostenlose allgemeine Rücknahme-/Abholleistung ableiten.
- Formularkästchen gehen bei `pdftotext` teilweise verloren. Die hier verwendete Standort-/Schrottplatz-Zuordnung ist aus explizitem Fließtext gewonnen; keine unlesbaren Häkchen als geprüft ausgeben.

**ElektroG ist enger und hat eine Datumsabweichung:** [S12] ist ein separates Beiblatt zum selben EFB-Vorgang, Kirschallee 3a. Es nennt **EBA SW (Schadstoffentfrachtung/Wertstoffseparierung)** und als konkrete Tätigkeitsbeschreibung „Aktenvernichtung von Datenträgern wie CD, DVD, Disketten, Magnetbänder, Festplatten“. Keine pauschale Zertifizierung beliebiger Elektroaltgeräte aus dieser Tabelle behaupten. Das Beiblatt nennt Bindung an EFB-Gültigkeit **18.06.2026 bis 15.12.2027**, während das EFB-Deckblatt **16.12.2027** nennt; jährliche Überwachung spätestens 06/2027. Diese eintägige Abweichung dokumentieren, nicht glattziehen. Für die EFB-Zeile des Altbuchsland-Dossiers ist 16.12.2027 korrekt; ElektroG nicht als gleichlaufende Zusatz-Zertifizierung dort eintragen.

Gültigkeit bedeutet hier **im heute heruntergeladenen Dokument angegebene Frist**, nicht unabhängig bestätigte Nichtwiderrufenheit. Kein Live-Abgleich mit einem Zertifikats-/Widerrufsregister erfolgt. Die Entsorgungsseite [S13] verlinkt noch ein 2024er Zertifikat; für Aktualität ist die explizite 2026-Zertifikatsseite [S6] mit neu gelesenem PDF maßgeblich, nicht der alte Link.

## Ankauf / Verkauf / Gebühren

- **Ankauf:** Bargeld für Schrott/Metalle und ausgewählte Papierwertstoffe ausdrücklich für Privat- und Firmenkunden [S3–S4]. Keine heute lesbare numerische Ankaufquote, Einheit oder Preisgültigkeitsliste; Preise täglich schwankend, Auskunft telefonisch/persönlich. Keine Zahlen aus Verkauf oder Entsorgung als Ankauf erfassen.
- **Verkauf:** [S3–S4] bezeichnen das Angebot als An-/Verkauf, liefern aber keine numerischen Verkaufspreise im gelesenen Text. Daraus keine Ankaufpreisliste und keine konkrete Verfügbarkeit einzelner Verkaufsprodukte ableiten.
- **Gebühren / Service:** [S5] individuelle Preise für Big Pack/Container, [S13] fordert Kontakt wegen „anfallenden Kosten“ der Entsorgung. Weder kostenlos noch bestimmter Betrag belegt. Keine Gebührenfreiheit für Selbstanlieferung, Abholung oder Problemstoffe versprechen. Mengen, Entfernung, Standzeit und Nebenkosten nicht erfinden.
- **AGB-Grenze:** [S14] verlinkt ein fünfseitiges bildbasiertes PDF, heute erfolgreich heruntergeladen, aber `pdftotext` liefert keinen Text. Kein OCR-Programm verfügbar. Deshalb AGB-Konditionen/Gebühren nicht inhaltlich verifiziert; „keine veröffentlichten Gebühren auf der gesamten Website“ wäre nicht gedeckt.

## Lokaler Dublettenabgleich (18.10., nur Dossiers)

Repo-weite Textsuche nach `Borgsdorf`, Domain, `786437`, `Altbuchsland`, `Biaser` und `Kirschallee 3` sowie Dateisuche nach Zerbst durchgeführt. Neben dem Neuentwurf nur folgende sachlich relevanten vorhandenen Dossiers:

| Dossier | Lokaler Befund | Merge-Empfehlung |
| --- | --- | --- |
| `dossiers/st/st-bornum-b-zerbst-jurgen-borgsdorf.md` | Jürgen Borgsdorf; Bornum b. Zerbst; 039248/424; Website unbekannt; `pruefung`; Straße/PLZ leer. Timeline nennt Aggregator-Lead Lange Str. 18 / 39264. | **Nicht mergen**, keine über die Organhistorie hinaus belegte Identitätskette. |
| `dossiers/st/st-zerbst-fachgrohandel-metall-zerbst.md` | Kirschallee **5**, 03923 71900, metall-gmbh-zerbst.de; lokale Recherche nennt eigenständige Fachgroßhandel Metall GmbH, HRB **11482**. | **Nicht mergen**, andere Identität und Anschrift; Kirschallee 5 ist nicht 3a. Lokale Alt-Recherche nicht als heute neu abgerufener Registerbeleg deklarieren. |
| `dossiers/st/st-zerbst-albert-b.md` | `pruefung`; leere Stammdaten, aktuelle lokale Timeline führt nur Lead Bernd Albert / Lusoer Str. 21 / 03923 785391. | **Nicht mergen**; keine passende Identitätskette. Die Entwurfsbehauptung „andere Anschrift“ gilt hier nur für den Lead, nicht gefülltes Frontmatter. |

Kein weiteres lokales Dossier mit der GmbH-Domain, Altbuchsland-Adresse oder Telefonnummer gefunden. Das ist eine **lokale Korpusprüfung**, keine Aussage zur heutigen Produktions-DB oder Vollständigkeit aller Namenvarianten. Der im Entwurf behauptete DB-Befund vom 12.10. und dortige IDs wurden nicht erneuert.

## Quellen und tatsächliche Abrufgrenzen

Alle folgenden URLs wurden **18.10.2026 neu angefordert**. HTML über `webfetch` lesbar; PDFs wegen `webfetch`-Formatgrenze anschließend mit Python/HTTPS neu geladen und in-memory über `pdftotext -layout - -` extrahiert, keine Dateien dafür angelegt.

| ID | URL | Ergebnis / Verwendung |
| --- | --- | --- |
| S1 | https://www.borgsdorf-recycling.de/wp/impressum/ | Lesbar; GmbH/HRB/GF, separate GbR, unterschiedliche USt-IDs und Mailto-Konflikt. |
| S2 | https://www.borgsdorf-recycling.de/wp/kontakt/ und https://www.borgsdorf-recycling.de/ | Beide lesbar; gleicher Kontaktinhalt mit korrektem info-Mailto, Hauptstandort, Zeiten, Papier lose/ungebündelt, GbR getrennt. |
| S3 | https://www.borgsdorf-recycling.de/altstoffe/ | Lesbar; Privat/Firma, Material-/Papierbedingungen, Tageskonditionen. |
| S4 | https://www.borgsdorf-recycling.de/das-neue-museum-oeffnet-seine-tueren-2/ | Lesbar; trotz ungewöhnlichem Pfad konkrete Schrottseite, Materialgruppen, Ausschlüsse, Containerabholung. |
| S5 | https://www.borgsdorf-recycling.de/containerdienst/ | Lesbar; Anlieferung/Abholung gefüllter Behälter, individuelle Preise. Behältergrafiken nicht vermessen/visuell ausgewertet. |
| S6 | https://www.borgsdorf-recycling.de/ueber-uns/ | Lesbar; ausdrücklich beide Zertifikate „aus 2026“, Verknüpfung S7/S12. |
| S7 | https://www.borgsdorf-recycling.de/wp-content/uploads/2026/07/Zertifikat-Entsorgungsfachbetrieb.pdf | HTTP 200, 11:01:37 UTC, 868717 Bytes; vollständiger Text extrahiert. SHA-256 `a6e2fcc069f51f3330d0cf1da04c3675de2eeb83722939531923cbd41659773d`. Last-Modified 03.07.2026 ist Servermetadatum, kein Prüfdatum. |
| S8 | https://www.northdata.de/Borgsdorf-Recycling+GmbH,+Zerbst/Anhalt/Amtsgericht+Stendal+HRB+15643 | Lesbar; Name, HRB, EUID DEW1215.HRB15643, Altbuchsland 18 d, Unternehmensgegenstand, zwei aktuelle Geschäftsführer, Meldungshistorie. Premiumgrenzen und automatisierte Auswertung beachten. |
| S9 | https://www.northdata.de/?id=5250384583065600 | Lesbar; Registeränderung 03.03.2026 Nr. 9: **Personenbezogene Daten geändert**, Kay, nicht Neubestellung. |
| S10 | https://www.northdata.de/?id=5679226386907136 | Lesbar; 08.07.2024 Nr. 7: Jürgen ausgeschieden; Personendaten Kay geändert. Keine Bornumer Betriebsadresse. |
| S11 | https://www.northdata.de/?id=84185086 | Lesbar; historische Bestellung Mario, Registerbekanntmachung 17.02.2014 / Veränderung 12.02.2014; OT Bornum als Personenort, nicht Betriebsstätte. |
| S12 | https://www.borgsdorf-recycling.de/wp-content/uploads/2026/07/Zertifikat-ElektroG-1.pdf | HTTP 200, 11:01:37 UTC, 323196 Bytes; Text extrahiert. SHA-256 `545d4064cefa8071caff1b9ccae64407f25cb4c0a2976c00acd4563d38952468`. Kirschallee/Datenträger und abweichendes Enddatum wie oben. |
| S13 | https://www.borgsdorf-recycling.de/entsorgung/ | Lesbar; anfallende Kosten auf Anfrage, materialbezogene Entsorgungsbedingungen; noch alter 2024-Zertifikatslink, dessen PDF nicht neu abgerufen wurde. |
| S14 | https://www.borgsdorf-recycling.de/wp/agb/ und https://www.borgsdorf-recycling.de/wp-content/uploads/2024/08/AGB-1.pdf | HTML lesbar; PDF HTTP 200, 11:01:37 UTC, 1938276 Bytes, SHA-256 `aaba25fb03f78dd6f9550cc19fd6298ef67b2221a03247641ce2c8bd06c56965`; fünf leere Textseiten, **Inhalt nicht gelesen**. |

Zusätzliche erfolglose Darstellungsversuche: Die auf [S2] eingebundenen Bilder https://www.borgsdorf-recycling.de/wp-content/uploads/2026/09/Screenshot-2026-09-01-095655.png und https://www.borgsdorf-recycling.de/wp-content/uploads/2026/09/Screenshot-2026-09-01-095854.png wurden am 18.10. per `webfetch` angefordert; beide scheiterten an „Unsupported fetched image content type: image/png“. Bildinhalt, mögliche Schließtage/Sonderzeiten oder Preisankündigungen daher **nicht geprüft**. Keine erfundene Bildinterpretation. Vor endgültiger Veröffentlichung reguläre Zeiten gegebenenfalls visuell gegen diese Hinweise abgleichen.

Keine Georecherche, keine Koordinaten, kein Telefonat, kein E-Mail-Test, keine DB-Änderung/-Abfrage, kein Build/Import/Commit. Der lokale Wurzel-PDF-Bestand `UeZEFB_SZB.pdf` wurde bewusst nicht als heutiger Abruf verwendet. Die versuchte Research-Delegation scheiterte am Subagent-Tiefenlimit; alle hier behaupteten Abrufe wurden direkt ausgeführt.

## Abschluss

Review nach Fortsetzung am **18.10.2026** fertiggestellt. Kein Nutzungslimit hat diese Fertigstellung blockiert. Die offenen Punkte sind ausdrücklich dokumentierte Beleg-/Darstellungsgrenzen (Bildhinweise, gescannte AGB, amtlicher aktueller Registerstatus und unabhängige Nichtwiderrufenheitsprüfung), keine als erledigt ausgegebenen Prüfungen. Integrationsfreigabe bleibt bedingt; insbesondere kein Merge und keine ungeprüfte Übernahme sämtlicher ursprünglicher Timeline-Behauptungen. Die Fertigstellung dieses Reviews ist weder Dossierintegration noch Produktionsveröffentlichung.
