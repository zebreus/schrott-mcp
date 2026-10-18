# Bachmann / Collecta — Kontakt- und Website-Quellengates

**Fortgesetzt und frisch geprüft am 18.10.2026, 02:01–02:03 UTC (04:01–04:03 MESZ).** Der bestellte Dateiname bleibt unverändert; er ist **kein Abrufzeitpunkt**. Die vor dem Nutzungslimit gelesenen Seiten wurden für die entscheidenden Befunde erneut direkt abgerufen. Sekundenangaben unten sind protokollierte **Request-Startzeiten UTC**, nicht Veröffentlichungsdaten. Ausgangspunkt: README, insbesondere Zeilen 183–202, und `research/checkin-20261017-2130-wave13-b-owner-final.md`.

## Ergebnis / Integrationsentscheidung

**Unter dem unveränderten README-Standard keine zusätzlichen strukturierten Kontakt-/Websitefelder freigeben.** Bachmann bleibt Einzelbeleg. Bei Collecta ist ein echter, vollständig gelesener unabhängiger **feldbezogener Verbandsabgleich** neu gelungen: bvse nennt exakt die Zieltelefonnummer und Domain. Das ist deutlich mehr als ein Suchindex. Allerdings gehört ein Verbands-Mitgliederverzeichnis nicht zu den in README ausdrücklich zugelassenen Zweitquellenklassen. Diesen Unterschied nicht stillschweigend durch eine angeblich abgeschlossene README-Freigabe überspielen.

| Ziel / Feld | Tatsächlich gelesene Evidenz | Disposition unter geltender README |
|---|---|---|
| Bachmann `phone: '0561 523934'` | Betreiber-Impressum und Kontakt kongruent; Creditreform 403; NorthData ohne Telefon | **Weiter gehalten.** Kein lesbarer qualifizierender feldbezogener Zweitbeleg. |
| Bachmann `website: https://www.bachmann-ks.de/` | Root/Impressum HTTP 200, Firma und HRA passen zu NorthData; keine Domain im gelesenen NorthData-HTML | **Weiter gehalten.** Erreichbarkeit und Betreiberzuordnung im Einzelbeleg sind bewiesen, zweite qualifizierende Feldquelle fehlt. |
| Bachmann `website_status: aktiv` | Root HTTP 200 mit echtem Betreiberinhalt | Technischer Befund **erreichbar**, aber strukturierter Fill bleibt an die gehaltene Domainfreigabe gebunden. |
| Collecta `phone: '06122 5337-0'` | Betreiber-Impressum **und bvse-Mitgliedsdetail Nr. 201** nennen dieselbe Nummer | **Inhaltlicher unabhängiger Feldabgleich gelungen; formales README-Klassengate offen.** Keine automatische Integration. |
| Collecta `website: https://www.collecta-rohstoff.de/` | Betreiber-Impressum/Root **und bvse-Mitgliedsdetail Nr. 201**, zusätzlich Entsorgergemeinschaft Nr. 2102 | **Inhaltlicher unabhängiger Domainabgleich gelungen; formales README-Klassengate offen.** HTTPS-Root selbst HTTP 200. Die beiden verwandten bvse-Auftritte nicht als zwei unabhängige Herausgeber zählen. |
| Collecta `website_status: aktiv` | HTTPS-Root HTTP 200, Impressum und Ansprechpartnerseite lesbar | Technischer Befund **erreichbar**, strukturierter Fill weiter an endgültige Domainfreigabe gebunden. |

**Freigabefähig ohne Quellenstandardänderung:** die präzisen zugeschriebenen Recherchebefunde dieses Berichts, nicht neue positive Frontmatterwerte. Die zuvor freigegebenen Identitäts-/Adresskorrekturen werden durch diese Recherche nicht widerrufen, aber auch nicht pauschal neu abgenommen. Beide gelesenen Dossiers enthalten für `phone`, `website`, `website_status` bereits leere Werte; hier nichts daran geändert. Leerhaltung ist kein autorisierter Datenbank-Clear.

**Falls der Owner die Zulässigkeit einer unmittelbar vom Fachverband geführten Mitgliedsakte ausdrücklich entscheidet:** Für Collecta ist dann das konkrete Telefon-/Domainpaket inhaltlich entscheidungsreif, nicht bloß eine weitere Suchspur. Dafür genau Betreiber-Impressum + bvse-Detail Nr. 201 + Root-Abruf anführen. Das ist eine **bedingte Empfehlung**, keine hier erteilte Ausnahme oder Freigabe. Mail, Händleraktivstatus, Öffnungszeiten, Leistungen oder Zertifizierung daraus nicht mitfreigeben.

## 1. Bachmann: Was der Vollabruf trägt — und was nicht

Ziel: `he-kassel-bettenhausen-bachmann-matthias-autoverwertung`.

### Betreiberquelle B1

- <https://www.bachmann-ks.de/> — HTTP 200, 18.10.2026 02:02:05Z. Inhalt ist die Autoverwertung, nicht Parking/Fehlerseite. Footer nennt Firma, Heiligenröder Straße 45, 34123 Kassel, Telefon und Mail.
- <https://www.bachmann-ks.de/impressum/> — HTTP 200, 02:02:05Z. Gelesen: „Matthias Bachmann Autoverwertung GmbH & Co. KG“, AG Kassel **HRA 7347**; Komplementärin Matthias Bachmann Autoverwertung Verwaltungs GmbH **HRB 16002**, Geschäftsführer Matthias und Michael Müller. Kontakt: **05 61 / 52 39 34**, `info@bachmann-ks.de`.
- <https://www.bachmann-ks.de/kontakt/> — HTTP 200, 02:02:26Z. Individuelle Standortadresse und dieselben Kontakte. Keine Öffnungszeiten im gelesenen Hauptinhalt.
- <https://www.bachmann-ks.de/unternehmen/> — HTTP 200, 02:02:26Z. Verlinkt weiterhin nur das unten untersuchte Zertifikat 2023/2024. „75 Jahre“, Copyright 2023 oder eine historische Anerkennung seit 1998 sind kein belastbarer Aktualitätsnachweis für Oktober 2026.

Diese Unterseiten sind **eine** Betreiberquelle; URL-Mehrzahl ist keine unabhängige Zweitquelle.

### Unabhängige Identitätsquelle B2 — kein Kontaktfeldbeleg

<https://www.northdata.de/Matthias+Bachmann+Autoverwertung+GmbH+%26+Co.+KG,+Kassel/Amtsgericht+Kassel+HRA+7347> — im öffentlichen Seiteninhalt direkt gelesen, HTTP 200 erneut 02:02:25Z und 02:02:35Z. Firma, HRA 7347, Heiligenröder Str. 45/34123 und Unternehmensgegenstand sind kongruent. Die öffentliche Publikationsliste zeigt den Übergang 2013; kein kostenpflichtiger Originalauszug behauptet.

**Feldgrenze:** Im gelesenen Inhalt keine Telefon-/Websiteangabe des Zielbetriebs; zusätzliche Volltextkontrolle des zurückgelieferten öffentlichen HTML: weder Ziffernfolge `523934` noch Domainliteral `bachmann-ks.de`. Das stützt die Identität, **nicht** den gehaltenen Telefon-/Domainfill. Kein separates aktuelles Registerstatusattest abgerufen.

### Weitere tatsächlich geprüfte Wege

| Quelle / Abruf | Konkretes Ergebnis |
|---|---|
| <https://firmeneintrag.creditreform.de/34123/6150052355/MATTHIAS_BACHMANN_AUTOVERWERTUNG_GMBH_CO_KG>, 02:02:25Z | **HTTP 403.** Kein Datensatz gelesen; keine Sperrumgehung, kein anderer Zugang/Proxy eingesetzt. Der vorbekannte Suchindex wird nicht aufgewertet. |
| <https://www.facebook.com/people/Matthias-Bachmann-Autoverwertung-GmbH-Co-KG/100057306075660/>, 02:02:27Z | Vom Betreiber verlinkt. HTTP 200 liefert HTML mit Titel „Facebook“, aber keinen verifizierbaren lesbaren Unternehmens-Kontaktblock; `523934` fehlt. Kein Profil-/Post-/Telefonbeleg behauptet. |
| <https://www.instagram.com/autoverwertung_m_bachmann/>, 02:02:27Z | Vom Betreiber verlinkt; Weiterleitung auf `https://www.instagram.com/accounts/login/?next=https%3A%2F%2Fwww.instagram.com%2Fautoverwertung_m_bachmann%2F&is_from_rle`, dann HTTP 200 Login. Keine Profilinhalte verifiziert; nicht eingeloggt. |
| <https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung>, 02:02:27Z | HTTP 200, **2.417 Byte Anwendungshülle**, leeres `outlet`; kein Bachmann-Einzelnachweis. Öffentliche JS-Datei `https://fachbetrieberegister.gadsys.de/fachbetrieberegister/VAADIN/build/indexhtml-BSsZwGIF.js` zusätzlich gelesen, nur Anwendungsbootstrap, kein abschließender Betriebs-/Kontaktabruf. Keine Registerabwesenheit behauptet. |
| <https://www.bvse.de/ueber-uns/mitglieder.html?fs%5Bname1%5D%5Blike%5D=Bachmann> | Öffentliches Suchformular verwendet; leere Ergebnistabelle. Kein Existenz-/Schließungsbeweis und keine Aussage über andere Fachverbände/Register. |

### Zertifikatsprüfung B3 — frisch extrahiert, nicht aus Altnotiz übernommen

<https://www.bachmann-ks.de/wp-content/uploads/2023/12/Zertifikat-2023-2024.pdf> — HTTP 200, 02:02:05Z, **235.324 Byte**, SHA-256 `878bdf6161ee24f97f58bcc54fd9f2b391d3f2005f890ea6c66fccd48b06a049`. PDF direkt im Speicher abgerufen und mit `pdftotext -layout - -` gelesen, keine PDF-Datei im Repository angelegt.

Der Text nennt die GmbH & Co. KG, Heiligenröder Straße 45, Demontagebetrieb, Betriebsnummer **F 75 E 02100**, Bescheinigung nach § 5 Abs. 3 Altfahrzeug-Verordnung, Registriernummer **01 410 080029**, gültig bis **24.12.2024**. OCR-Schreibfehler im extrahierten Datum (`24.',t2.2024`) nicht als neue Gültigkeit interpretieren. Kein Telefon-/Websitebeleg im Zertifikat, keine heute gültige Zertifizierung.

Zusätzlich öffentliche Medien-API <https://www.bachmann-ks.de/wp-json/wp/v2/media?search=Zertifikat&per_page=100>, HTTP 200, 02:02:26Z: **zwei** passende Datensätze, IDs 13555 (PDF 2023/2024, API-Datum 08.12.2023) und 13176 (PDF 2022). Kein neueres Zertifikat in **dieser Suchantwort**. Das ist keine vollständige Inventur aller möglichen Dokumente und kein Beweis, dass keine aktuelle Bescheinigung existiert. Öffentliche API ohne Anmeldung, kein Zugriff auf private Medien.

## 2. Collecta: Neuer vollständiger unabhängiger Kontaktabgleich, aber kein stilles Policy-Upgrade

Ziel: `he-wiesbaden-65183-collecta-rohstoff`.

### Betreiberquelle C1

- <https://www.collecta-rohstoff.de/> — HTTP 200, 02:02:05Z. Echte Unternehmensauswahl **COLLECTA Rohstoff / COLLECTA Europe**. Rohstoff-Unterseite und Impressum separat gelesen; die gemeinsame Startseite macht die beiden Gesellschaften nicht zu einem Rechtsträger.
- <https://www.collecta-rohstoff.de/impressum/> — HTTP 200, 02:02:05Z. Rohstoff-Block: **COLLECTA Rohstoff GmbH & Co. KG**, Senefelderstraße 3A, 65205 Wiesbaden, **HRA 4165**, Verwaltungs-GmbH **HRB 9660**, AG Wiesbaden, Olaf Schäfer, **+49 (0) 6122 / 5337 – 0**, `info@collecta-rohstoff.de`. Europe-Block getrennt: **HRB 9659**, andere Maildomain. Beide nicht zusammenziehen.
- <https://www.collecta-rohstoff.de/ansprechpartnerinnen/> — HTTP 200, 02:02:26Z. Standortbezogene Personenkontakte, nicht nur Formular. Olaf Schäfer hat Durchwahl **126**, Thomas Giesinger und der Rohstoff-Footer die **0**; Lars Machenheimer ist mit einem Bildpfad `/wp-content/uploads/2026/09/D_4340-1000_1-683x1024.jpg` eingebunden. Der Bildpfad ist ein **begrenztes jüngeres Pflegesignal**, kein selbständiger datierter Geschäfts-/Zertifikatsnachweis. Keine Öffnungszeiten im gelesenen Hauptinhalt.
- <https://www.collecta-rohstoff.de/collecta-rohstoff/> und <https://www.collecta-rohstoff.de/jobangebote/> tatsächlich gelesen: B2B-Altpapierhandel bzw. undatierte Stellenangebote am Bürostandort Wiesbaden-Nordenstadt. Kein Beleg für öffentliche Hofannahme, keine ausdrücklich aktuelle datierte Betriebsbestätigung.

### Unabhängiger Feldabgleich C2: bvse-Bundesverband, Mitgliedsakte Nr. 201

**Fundweg:** Auf <https://www.bvse.de/> verlinkte Mitgliedersuche → öffentliches GET-Suchformular <https://www.bvse.de/ueber-uns/mitglieder.html?fs%5Bname1%5D%5Blike%5D=Collecta> → zwei getrennte Treffer (Europe Nr. 3076 und **Rohstoff Nr. 201**) → **Rohstoff-Detail vollständig gelesen**:

<https://www.bvse.de/ueber-uns/mitglieder/201-collecta-rohstoff-gmbh-co-kg.html?ml=1>

Frischer Direktabruf **18.10.2026 02:02:06Z, HTTP 200**, 8.227 Byte. SHA-256 der zu diesem Zeitpunkt gelieferten HTML-Antwort: `b41abfa515fab81437699f8f4200ee782be6cfbd159a319f9ddc8a52a47479dc` (dynamische Tokens können spätere HTML-Hashes ändern).

Tatsächlich sichtbarer Inhalt:

> Collecta Rohstoff GmbH & Co. KG
>
> Senefelderstraße 3a, 65205 Wiesbaden, Hessen, D
>
> Telefon: **0049 6122 5337-0**
>
> Fax: 0049 6122 5337-134
>
> Internet: **www.collecta-rohstoff.de**

Der Internetlink zeigt auf `//www.collecta-rohstoff.de`. Telefonnummer normalisiert zu `06122 5337-0`, Domain exakt kongruent. Rechtsträger und Straße/PLZ trennen den Treffer vom Schwesterunternehmen. E-Mail bleibt in der sichtbaren Ausgabe JS-geschützt und wird hier **nicht** als zweifach bestätigt ausgegeben.

**Qualität / Unabhängigkeit:** Direktes Mitgliedsdetail auf dem eigenen Server des Fachverbands, kein allgemeines Branchenaggregatorportal, kein Suchmaschinenfragment. Herausgeber ist unabhängig vom Händlerauftritt; Herkunft und Aktualisierungsdatum der einzelnen Mitgliedsfelder sind jedoch nicht ausgewiesen, keine unabhängige telefonische Funktionsprüfung. Eine vollständig gelesene öffentliche Mitgliedsakte, kein beglaubigter Registerauszug. README 183–202 nennt Verbandsverzeichnisse nicht als zugelassene Zweitquellenklasse; deshalb keine eigenmächtige Quellenklassen-Erweiterung.

### Ergänzung C3: bvse-Entsorgergemeinschaft, Eintrag Nr. 2102

Öffentliche Suche <https://www.bvse-entsorgergemeinschaft.de/bvse-entsorgungsfachbetriebe.html?fs%5Bfirmenname%5D%5Blike%5D=Collecta> liefert Europe und Rohstoff getrennt. Wichtig: Das Formular verwendet hier **`firmenname`**, nicht `name1`. Der zuvor tatsächlich probierte Parameter `fs[name1][like]=Collecta` lieferte die ungefilterte Liste und ist **kein Negativbefund**.

<https://www.bvse-entsorgergemeinschaft.de/bvse-entsorgungsfachbetriebe/2102-collecta-rohstoff-gmbh-co-kg.html?ml=1> — HTTP 200, 02:02:06Z, 9.578 Byte, SHA-256 `1cbd19ececf3a1d8b1ccecf4fc305b55f1039eb08e4da4afa1e2f1fdfab622d2`.

Gelesen: COLLECTA Rohstoff GmbH & Co. KG, Senefelder Straße 3a, 65205 Wiesbaden; Internetlink `http://www.collecta-rohstoff.de`; Telefon **06122/5337-126**. **Das ist nicht die Zielzentrale `-0`**, sondern kongruent zur Betreiber-Durchwahl von Olaf Schäfer. C3 daher nur zusätzlicher Domain-/Personenkontaktabgleich, **kein zweiter Beleg für die exakte `-0`** und kein unaufgelöster Zentralnummern-Override. Nicht als vom Bundesverband unabhängige dritte Quellenfamilie zählen. Undatiertes Listing beweist kein aktuell gültiges EFB-Zertifikat.

### Registerspur C4 / gesperrter Kontaktweg C5

- <https://www.northdata.de/COLLECTA%20Rohstoff%20GmbH%20%26%20Co.%20KG,%20Wiesbaden/HRA%204165> — öffentliche Seite vollständig gelesen; HTTP 200 erneut 02:02:25Z und 02:02:35Z. HRA 4165 und Senefelderstr. 3 a/65205 kongruent, Publikationsliste u. a. Jahresabschlussveröffentlichung 04.11.2025, Prokura-Meldung 30.12.2024 und Erwähnung 23.09.2026. **Keine Zieltelefon-/Domainangabe**: `5337` und `collecta-rohstoff.de` im zurückgelieferten HTML nicht enthalten. Nicht aus bloßer Existenz eines Profils einen separat bestätigten heutigen Registeraktivstatus machen; keine Originalpublikation hinter Premium freigeschaltet.
- <https://firmeneintrag.creditreform.de/65205/6250080496/COLLECTA_ROHSTOFF_GMBH_CO_KG> — **HTTP 403**, 02:02:25Z. Kein Vollabruf. Kein Umgehen der Sperre, keine Aufwertung alter Suchindexdaten.

### Zertifikatsprüfung C6 — Ablauf bleibt Ablauf

<https://www.collecta-rohstoff.de/zertifikate/> — HTTP 200, 02:02:26Z; beide als „Rohstoff“ und „Europe“ beschrifteten Downloadlinks zeigen weiterhin auf **dasselbe Rohstoff-PDF**. Nicht als zwei Zertifikate zählen.

<https://www.collecta-rohstoff.de/wp-content/uploads/2025/05/EfB-Zertifikat2025_COLLECTA-Rohstoff-GmbHCoKG.pdf> — HTTP 200, 02:02:06Z, **305.475 Byte**, SHA-256 `fc41aa41feae4eb3c740da9d0068c1289af0c01101b90317cab7bb6c726a9ff9`. Frisch per Speicherabruf/`pdftotext` gelesen: **Nr. 10515**, COLLECTA Rohstoff GmbH & Co. KG, Senefelderstr. 3 a, Überprüfung **19.11.2024**, Ausstellung **23.01.2025**, gültig bis **31.05.2026**, Verweis auf rechtlich verbindliches Zertifikat mit Vorgangsnummer **ZZEE001002504007**. Das unten aufgedruckte Telefon **0228 98849-28** gehört dem **Aussteller bvse**, nicht Collecta. Kein Feldbeleg für `06122 5337-0` oder die Collecta-Domain; kein aktueller EFB-Nachweis.

Öffentliche Medien-API <https://www.collecta-rohstoff.de/wp-json/wp/v2/media?search=Zertifikat&per_page=100>, HTTP 200, 02:02:26Z: zehn passende Anhänge; jüngste API-Datumsangaben 26.05.2025, Rohstoff-PDF ID 1266. Separates Europe-PDF ID 1270 vorhanden, aber hier nicht als Rohstoff-Zertifizierung gelesen/übertragen. **Kein 2026-Zertifikat in dieser Suchantwort**, keine Behauptung einer vollständigen Dokumentinventur oder eines Zertifikatsentzugs.

## 3. README-Betreiber-Ausnahme — alle Kriterien ausdrücklich geprüft

| README-Kriterium | Bachmann | Collecta |
|---|---|---|
| Impressum benennt Betriebsgesellschaft samt Registerkennung | **Erfüllt**, HRA 7347 + Komplementärin HRB 16002. | **Erfüllt**, HRA 4165 + Komplementärin HRB 9660; Europe getrennt. |
| Registerkongruenz / Gesellschaft live im Register | Firma/Ort/Anschrift im gelesenen NorthData-Profil kongruent. **Kein separat bestätigter aktueller Registerstatusauszug**. | Firma/Ort/Anschrift im gelesenen NorthData-Profil kongruent; jüngere Publikationen sind positive Spuren, **kein separat bestätigter aktueller Registerstatusauszug**. |
| Konkrete Standortseite statt bloß allgemeinem Kontaktformular | Kontaktseite mit Adresse, Telefon und Mail vorhanden. Öffnungszeiten nicht gefunden. | Ansprechpartnerseite mit individuellen Durchwahlen und Standortadresse vorhanden; Büro statt belegter öffentlicher Annahmestelle. Öffnungszeiten nicht gefunden. |
| Belastbare Inhaltsaktualität (Zertifikate, Daten, Nachrichten) | **Nicht geschlossen**: verlinktes Zertifikat seit 24.12.2024 abgelaufen; kein aktueller datierter Betreiberbeleg gelesen. | **Nicht vollständig geschlossen**: Foto-Uploadpfad 2026/09 ist ein Pflegesignal, undatierte Stellenanzeigen kein hinreichender Datumsbeleg; Zertifikat seit 31.05.2026 abgelaufen. |
| Gesamtergebnis | **Keine vollständige Ausnahme.** | **Keine vollständige Ausnahme.** |

Kein einzelnes fehlendes Gate durch Erreichbarkeit, WordPress-Pfad, Copyright-Jahr oder die bloße Selbstbehauptung „jährliche Auditierung“ ersetzen. Aus abgelaufenen PDFs folgen weder Betriebsstilllegung noch heute fehlende Zertifizierung; lediglich **dieser konkrete Beleg ist nicht aktuell gültig**.

## 4. Grenzen und nächster entscheidender Beleg

- **Bachmann:** Lesbarer aktueller Creditreform-/Register-/kommunaler Gewerberegisterdatensatz oder tatsächlich lesbares betreibergeführtes Socialprofil mit **genau Telefon und Domain** wäre entscheidend. Alternativ alle Ausnahmegates tatsächlich schließen, insbesondere aktuelle datierte Betreiberunterlagen und belastbarer Live-Registerstatus. Mehr Betreiber-Unterseiten allein helfen dem Zwei-Quellen-Gate nicht.
- **Collecta:** Telefon-/Domainabgleich liegt jetzt jenseits eines Suchindex vor. Unter unveränderter README fehlt weiterhin ein Kontaktfeldbeleg aus der ausdrücklich erlaubten Quellenklasse. Der konkret vorliegende bvse-Befund ermöglicht eine enge Owner-Entscheidung zur Quellenzulässigkeit; eine solche Entscheidung ist hier **nicht fingiert**. Alternative bleibt ein lesbarer zulässiger Feldbeleg oder eine vollständig geschlossene Betreiber-Ausnahme.
- **Nicht freigegeben:** neue E-Mail-Fills, `status: aktiv`, Öffnungszeiten, aktuelle Zertifizierungen, öffentliche Anlieferung/Abholung, Annahme loser Metalle, Preise oder Leistungs-/Description-Erweiterungen. Keine GmbH→KG-Umwandlung bei Collecta und kein Europe-Merge aus diesen Kontaktquellen ableiten.
- Die vor der Unterbrechung verwendeten Websuchen `"Bachmann" "523934" Zertifikat` und `"COLLECTA Rohstoff" "5337"` lieferten sachfremde Modellbahn-/Spielzeugtreffer. Daraus weder Vollständigkeit der Suche noch Nichtvorhandensein anderer Belege ableiten. Keine Suchindexinformation ist Grundlage einer positiven Feldfreigabe dieses Berichts.

## Scope-Nachweis

Nur diese Recherchedatei geschrieben. README, Ausgangsreview und beide Dossiers ausschließlich gelesen. Keine Dossieredits, Datenbank-/Produktivzugriffe, GEO-Änderungen, Tests, Builds, Seeds oder Gitoperationen. Öffentliche HTTP-Abrufe, lokale Uhrzeitabfrage und speicherbasierte HTML-/PDF-Auswertung; keine Anmeldung, kein Kontaktformular abgesendet, keine Sperrumgehung. HTTP 200 bei Login-/JS-Hüllen ausdrücklich nicht als erfolgreich gelesener Geschäftsnachweis gewertet.
