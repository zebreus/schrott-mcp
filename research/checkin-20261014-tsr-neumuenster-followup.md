# Check-in 14.10.2026 — TSR Neumünster: Betreiber, Zertifikate, Website

## Ergebnis für den Owner

**Der heutige Standort Leinestraße 4, 24539 Neumünster wird in Standortdetail, EFB-Zertifikat und ISO-Anlage übereinstimmend als TSR Deutschland GmbH & Co. KG geführt. TSR Recycling wurde nicht einfach in TSR Deutschland umbenannt:** Der frühere Rechtsträger TSR Recycling GmbH & Co. KG, Dortmund HRA 18511, heißt heute TSR Group GmbH & Co. KG. TSR Deutschland ist ein anderer, 2024 eingetragener Rechtsträger, Dortmund HRA 19743. Eine Ausgliederung wurde am 14.10.2024 wirksam; der konkrete Übertragungsumfang einschließlich einer historischen Zuordnung dieses einzelnen Betriebs wurde nicht aus dem Umwandlungsvertrag nachgewiesen. [S1–S9]

Die drei direkt auf der Neumünster-Seite verlinkten Zertifikate wurden inhaltlich gelesen und sind nach ihren angegebenen Laufzeiten am 14.10.2026 gültig. EFB und ISO nennen **die konkrete Straße**, A|U|F nennt Betreiber, PLZ und Ort. [S10–S12] Die im Dossier gespeicherte Startseite, die Standortseite und das Impressum lieferten heute unmittelbar HTTP 200 und lesbare Inhalte; `website_status: blockiert` ist als aktueller Abrufstatus nicht haltbar. Eine frühere Blockade wird dadurch nicht widerlegt. [S1, S2, S13]

## Umfang und Maßstab

- Arbeitsgrundlage: `dossiers/sh/sh-neumunster-tsr-recycling-nl-neumunster.md` und README, insbesondere „Source hierarchy“, Betreiber-Ausnahme und Betreiber-Ketten. Dossier aktuell: Altname, leere Straße/PLZ/Kontaktdaten/Öffnungszeiten, Website `https://www.tsr-recycling.de/`, `website_status: blockiert`, `status: aktiv`.
- Nur diese Recherchedatei angelegt. Keine Dossier-, Rust-, GEO-, Datenbank-, Commit-, Push- oder Deploy-Arbeit. Vorhandene fremde Änderungen nicht bearbeitet. Kein Abgleich mit einer Live-DB; „aktueller DB-Händler“ ist die vom Auftrag vorgegebene Identität, nicht ein hier ausgelesener DB-Zustand.
- Keine Händleraggregatoren als Belege gezählt. North Data wird ausdrücklich als README-zugelassene **Registeraufbereitung**, nicht als direkt abgerufener amtlicher Registerauszug behandelt. Mehrere North-Data-Seiten sind ein Quellenkanal, nicht mehrere unabhängige Quellen. Unabhängig davon wurden Betreiberquellen und die Primär-LEI-Datensätze bei GLEIF gelesen.
- Alle unten genannten Webquellen am **14.10.2026** abgerufen. HTML per direktem HTTPS-GET, PDFs vollständig im Speicher geladen und mit `pdftotext -layout` ausgewertet. Keine gesonderten Download-/Extraktionsdateien angelegt; keine kryptographische Signatur- oder Widerrufsprüfung der Zertifikate.

## 1. Rechtlich sauber getrennte Betreiberkette

| Ebene | Gelesener Befund | Beleg und Grenze |
|---|---|---|
| Aktueller Standortbetreiber | Standortdetail: **TSR Deutschland GmbH & Co. KG**, Leinestraße 4, 24539 Neumünster. Impressum: Sitz Lünen, Dortmund **HRA 19743**, persönlich haftende Gesellschafterin **TSR Deutschland Verwaltungs GmbH**, Dortmund **HRB 36274**. | [S1, S2]; das Impressum allein wäre kein Standortbeleg, hier stimmen Detail und Zertifikatsanlagen überein. |
| Eigenständiger Rechtsträger Deutschland | North Data zeigt Eintragung **19.07.2024**, TSR Recycling als damaligen Kommanditisten und TSR Deutschland Verwaltungs als Komplementärin. GLEIF bestätigt Deutschland/HRA 19743, `entity.status: ACTIVE`, `registration.status: ISSUED`, `FULLY_CORROBORATED`, letzte Aktualisierung 28.05.2026. | [S5, S8]; Gründungs-/Eintragungsinformationen bei North Data, keine vollständig gelesene amtliche Ersteintragung. GLEIF ist LEI-Register, kein Handelsregisterersatz für sämtliche Rechtsverhältnisse. |
| Ausgliederung, keine bloße Namensänderung | Registeraufbereitung für Deutschland nennt am **14.10.2024** Übernahme/Ausgliederung mit TSR Recycling. Der frei lesbare Registertext vom **21.10.2024**, Eintragung Nr. 3, lautet: „Die Ausgliederung ist mit der Eintragung auf dem Registerblatt des übertragenden Rechtsträgers am 14.10.2024 wirksam geworden.“ | [S5, S6]. Volltext des Ausgliederungsvorgangs und Umwandlungsvertrag nicht gelesen; deshalb keine Behauptung, dass Neumünster nachweislich genau an diesem Datum übertragen wurde oder sämtliche Vermögenswerte übergingen. |
| Alt-Rechtsträger Recycling → Group | Betreiber-Mitteilung vom **05.12.2024**: „Die Umfirmierung der bisherigen TSR Recycling GmbH & Co. KG in die TSR Group GmbH & Co. KG erfolgte Anfang Dezember 2024.“ Group-Impressum nennt Dortmund **HRA 18511**, Komplementärin Group Verwaltungs **HRB 29503**. North Data führt Recycling als früheren Namen und eine Namensänderung am **04.12.2024**. | [S3, S4, S7]; Betreiber-Mitteilung und unabhängige Registeraufbereitung erfüllen hier den README-Identitätsstandard. |
| Unabhängiger Primärdaten-Abgleich | GLEIF führt unter LEI **529900C3Y7YXPRLGIK87** Group, HRA 18511, `ACTIVE`; Recycling steht in `otherNames` ausdrücklich als `PREVIOUS_LEGAL_NAME`. Deutschland hat die andere LEI **529900PYWIUX8UMMSD29** und HRA 19743. | [S8, S9]; Primär-LEI-Daten bestätigen die Abgrenzung. Beim Group-Datensatz liegt das gespeicherte Namensänderungsereignis im Juni 2025, nicht Dezember 2024; dies nicht als Handelsregister-Umfirmierungsdatum übernehmen. |

**Wichtiger Website-Widerspruch:** Im Haupttext der Neumünster-Seite steht noch „Tochtergesellschaft der TSR Recycling GmbH & Co. KG“, während Footer/Startseite „Tochtergesellschaft der TSR Group GmbH & Co. KG“ sagen. Das passt zur belegten Umfirmierung der Mutter, ist aber eine redaktionell nicht vollständig aktualisierte Seite. Es ist kein Beleg, dass TSR Deutschland und TSR Recycling identisch wären. Der Standortblock und die 2026er Zertifikate nennen durchgehend Deutschland. [S1, S3, S10–S13]

**Zulässige Formulierung:** „Heute TSR Deutschland GmbH & Co. KG, Niederlassung Neumünster. Historisch im Dossier unter TSR Recycling geführt; TSR Recycling heißt seit Anfang Dezember 2024 TSR Group. TSR Deutschland ist ein eigener Rechtsträger; eine Ausgliederung aus TSR Recycling wurde am 14.10.2024 wirksam. Der genaue historische Übertragungsumfang dieses Standorts ist nicht aus dem Vertrag geprüft.“

**Nicht zulässig:** „TSR Recycling wurde in TSR Deutschland umfirmiert“, „Recycling ist erloschen“, „vollständige Gesamtrechtsnachfolge sämtlicher Geschäfte“ oder „Neumünster wurde sicher am 14.10.2024 übertragen“. Dafür fehlen entsprechende Einzelbelege.

## 2. Zertifikate: branchbezogen statt Konzernpauschale

| Zertifikat | Dokumentdaten | Tatsächlicher Bezug zu Neumünster |
|---|---|---|
| **Entsorgungsfachbetrieb**, EdDE e.V. | Nr. **0679**, 8. jährliche Überprüfung; Vorgang **ZZEE002000199013**. Prüfung **19.02.2026**, Ausstellung **27.04.2026**, gültig bis **18.08.2027**. Titelblatt nennt TSR Deutschland, Region Nordost/Regionalbereich Nord, **HRA 19743 Dortmund**. | Anlagen **17–20**, dokumentinterne Seiten **42–50**, ausdrücklich Niederlassung Neumünster, **Leinestraße 4**. Tätigkeitsbeschreibungen: Lagerung/Behandlung FE-/NE-Metalle (BE 1), Lagerung von Abfällen (BE 2), Behandlung von Abfällen (BE 2), Annahmestelle für Kleinanlieferer (BE 3). [S10] |
| **DIN EN ISO 9001:2015**, GZQ GmbH | **Z-001422-3**, ausgestellt **22.05.2026**, gültig **01.04.2026–31.03.2029**, ersetzt Z-001422-2. | Anlage, PDF-Seite **2**, **Standort 7**, Leinestraße 4, DE-24539 Neumünster, TSR Deutschland. Lokaler Geltungsbereich: **„Handel mit und Aufbereitung von Stahl- und NE-Metallschrott“**. Nicht den umfangreicheren Gesamtgeltungsbereich vom Titelblatt pauschal der Filiale zuschreiben. [S11] |
| **Autorisierter Sammelpartner A|U|F e.V.** | Ausgestellt Frankfurt **01.01.2026**, gültig bis **31.01.2027**; Neu-Ausstellung bei weiterer Teilnahme vorgesehen. | PDF-Seite **33**: TSR Deutschland GmbH & Co. KG, **24539 Neumünster**; Aluminium-/Umweltengagement im Fenster- und Fassadenbau. Keine Straße auf dieser Seite; Bezug zusätzlich durch direkte Verlinkung im Neumünster-Detail. Dies ist eine Sammelpartner-Bescheinigung, keine weitere allgemeine EFB- oder ISO-Zertifizierung. [S12] |

EFB-Präzisierung: Anlage 17 führt u. a. AVV 170401 Kupfer/Bronze/Messing, 170402 Aluminium, 170405 Eisen/Stahl und 200140 Metalle. Anlage 18 nennt u. a. **160601\* Bleibatterien**; Anlage 20 ebenfalls. Die Annahme von Bleibatterien im Standorttext hat damit zusätzlichen Dokumentbezug. [S1, S10] Das Zertifikat begrenzt seinen Umfang ausdrücklich auf Anlagen/Abfallarten/Tätigkeiten/Standorte. Seine Auflistung von Altfahrzeugen **160106** (ohne Flüssigkeiten/gefährliche Bestandteile) ist **kein** Nachweis einer örtlichen Altfahrzeug-Demontagezulassung. Die Anerkennung nach AltfahrzeugV verweist im Titelblatt auf Anlage 10, die ElektroG-Erstbehandlung auf Anlage 14 — nicht auf die Neumünster-Anlagen 17–20. Kein pauschaler ElektroG-/Demontagebetriebs-Claim für Neumünster.

Grenze der Auswertung: Checkbox-Markierungen sind in der Textausgabe nicht zuverlässig abgebildet. Obige Tätigkeiten stammen aus den ausgeschriebenen Tätigkeitsbeschreibungen; keine weiteren angekreuzten Welt-/Deutschland- oder Verwertungsoptionen daraus geraten. Zertifikatsumfang ist nicht gleich öffentliches Ankaufangebot: Nicht alle im EFB aufgeführten Abfälle als vergütet oder bedingungslos angenommen eintragen.

## 3. Website-Status und operative Tatsachen

### Abrufstatus

| GET-URL | Befund am 14.10.2026 |
|---|---|
| `https://www.tsr-recycling.de/` | **HTTP 200**, lesbare aktuelle Startseite, TSR Deutschland als Anbieter; keine Umleitung auf einen Fremdbetreiber. |
| `https://www.tsr-recycling.de/standorte/schrotthaendler-neumuenster` | **HTTP 200**, konkreter Standortblock, Ankaufbedingungen und Zertifikatslinks lesbar. |
| `https://www.tsr-recycling.de/impressum` | **HTTP 200**, Firma, HRA und Komplementärin lesbar. |
| Alle drei Zertifikats-URLs [S10–S12] | **HTTP 200**, PDFs tatsächlich gelesen. |

`website_status` bezeichnet laut README die Website-Erreichbarkeit, nicht die juristische Identitätsklarheit. **Empfehlung: `aktiv` mit Prüftag 14.10.2026**, historische `blockiert`-Beobachtung nicht rückwirkend leugnen. CAPTCHA ist nur beim Kontaktformular, Consent-Overlays betreffen Karte/Social-Inhalte; der Standorttext benötigt kein JavaScript/CAPTCHA zur Lektüre. [S1, S2, S13]

### Übernahmefähige Standortangaben und Preisgrenzen

- **Adresse/Telefon/Zeiten:** Leinestraße 4, 24539 Neumünster; **+49 4321 6905980**; **Mo–Fr 07:00–16:00**. [S1]
- **E-Mail:** Der Containeranfrage-Link aus dem Standortdetail enthält den Empfänger **neumuenster@tsr.eu** (`Empfaenger=neumuenster%40tsr.eu`). Das ist ein veröffentlichter technischer Linkbeleg, kein getestetes Postfach; nach Owner-Prüfung verwendbar. [S1]
- **Ankauf:** Privat- und Geschäftskunden ausdrücklich; Fe-/NE-Metalle jeder Art und Menge, Selbstanlieferung oder Containerdienst; Verwiegen/Bewertung vor Ort; tagesaktuelle Vergütung, Barauszahlung auf Wunsch. Bleibatterien werden angenommen, aber **kein eigener Batterie-Ankaufkurs oder garantierte Batterie-Vergütung** genannt. [S1]
- **Verkauf:** Startseite/Footer beschreiben An- und Verkauf als allgemeines Portfolio. Auf der gelesenen Standortseite keine numerische Verkaufsliste und keine spezifischen örtlichen Verkaufsbedingungen. Nicht mit Ankaufspreisen verwechseln. [S1, S13]
- **Gebühren/Abholung:** Container werden bei größeren Mengen angeboten, einschließlich Anlieferung und Abholung; verschiedene Größen, THE METAL BOX für kleine regelmäßige NE-Mengen im Handwerk. **„Inklusive“ ist kein Beleg für kostenlos.** Keine bezifferten Container-, Transport-, Entsorgungs- oder Abzugsgebühren in den gelesenen Seiten. Keine kostenlose oder bedingungslose Haushaltsabholung ableiten. [S1]
- **Preis-Ingestion:** Kein Zahlenkurs gefunden. „Tagesaktuelle Preise“ und Preisanfrage sind keine scraperfähige Preisveröffentlichung. Keine erfundenen EUR/kg-Werte und keine Preiszeilen aus Zertifikaten.

## 4. Konkrete Owner-Empfehlung

1. **Slug stabil lassen.** Name künftig **„TSR Deutschland GmbH & Co. KG, NL Neumünster“**, nicht TSR Group als örtlichen Betreiber. Altbezeichnung und bisherige Recherche in der Timeline erhalten; als historische Betreiberbezeichnung, nicht als früherer Firmenname derselben HRA-19743-Gesellschaft erklären.
2. Nach Quellenprüfung Adresse, PLZ, Telefon und Zeiten wie oben befüllen; Standortdetail als branchbezogene Quellen-URL ergänzen. Die bestehende Startseiten-Website darf bleiben, da sie ebenfalls aktiv ist. E-Mail mit Linkbeleg übernehmen oder separat verifizieren.
3. **`website_status: aktiv`** ist unabhängig von der verbleibenden historischen Vertragsfrage belegt. `status: aktiv` kann bleiben; kein Anhaltspunkt für Schließung oder Verlegung. Aktueller Rechtsträger ist durch Betreiber-Impressum, registerkongruente North-Data-Seite, aktive GLEIF-Identität und aktuelle branchbezogene Zertifikate ausreichend abgesichert; die README-Betreiber-Ausnahme ist für heutige eigene Standortfakten damit nachvollziehbar geprüft.
4. EFB, ISO 9001:2015 und A|U|F **mit ihren Laufzeiten und standortbezogenen Grenzen** dokumentieren. A|U|F spätestens vor **31.01.2027**, EFB vor **18.08.2027**, ISO vor **31.03.2029** erneut prüfen; Laufzeiten sind keine Zusage unveränderter Gültigkeit bis zum Enddatum.
5. Für eine **vollständig amtlich dokumentierte historische Standortübertragung** noch chronologische Auszüge zu HRA 18511 und HRA 19743 sowie Ausgliederungs-/Übernahmevertrag vom 2024er Vorgang beschaffen und Neumünster/Leinestraße 4 in den Anlagen prüfen. Das ist eine offene Vertiefung, kein Grund, die heute ausdrücklich belegte Betreiberbezeichnung falsch zu belassen. Das Datum 14.10.2024 nur auf die Ausgliederung insgesamt beziehen, nicht ungeprüft als branchbezogenen Betreiberwechsel speichern.
6. Keine Zahlenpreis-Ingestion freigeben. Kundenbedingungen strukturiert nur im belegten Umfang: Selbstanlieferung privat/gewerblich; Containerabholung als angebotene Lösung bei größeren Mengen, Gebühren unbekannt.

## 5. Quellen und Zugriffslücken

### Gelesene Primärquellen

- **[S1]** Betreiber-Standortdetail: https://www.tsr-recycling.de/standorte/schrotthaendler-neumuenster — konkreter Betreiber/Adresse/Kontakt/Zeiten/Ankauf/Container und drei Zertifikatslinks; Haupttext/Footer-Muttername unterschiedlich.
- **[S2]** Deutschland-Impressum: https://www.tsr-recycling.de/impressum — HRA 19743, HRB 36274, Sitz Lünen.
- **[S3]** Betreiber-Mitteilung, 05.12.2024: https://www.tsr.eu/news/detail/tsr-group-buendelt-nationale-und-internationale-aktivitaeten/ — ausdrücklich Recycling → Group, Anfang Dezember 2024.
- **[S4]** Group-Impressum: https://www.tsr.eu/impressum/ — HRA 18511, HRB 29503, Sitz Lünen.
- **[S8]** GLEIF Deutschland, LEI-Primärdatensatz: https://api.gleif.org/api/v1/lei-records/529900PYWIUX8UMMSD29 — aktuelle Identität, ACTIVE, HRA 19743; Golden Copy 14.10.2026.
- **[S9]** GLEIF Group, LEI-Primärdatensatz: https://api.gleif.org/api/v1/lei-records/529900C3Y7YXPRLGIK87 — HRA 18511, Recycling als vorheriger Rechtsname, ACTIVE; Namensereignis-Datum in LEI-Daten abweichend; `conformityFlag: NON_CONFORMING` trotz aktiver Entity/ISSUED-Registrierung, deshalb keine pauschale Qualitätsgarantie aus diesem Datensatz ableiten.
- **[S10]** EdDE-EFB, direkt vom Betreiber verlinktes Ausstellerdokument: https://www.tsr-recycling.de/uploads/tx_3slocations/Efb-Zertifikat-2026_TSR-NordOst1.pdf — Titelblatt und Neumünster-Anlagen 17–20, dokumentinterne Seiten 42–50.
- **[S11]** GZQ-ISO-Zertifikat und Standortanlage: https://www.tsr-recycling.de/uploads/tx_3slocations/Z-001422-3_D_QM_TSR_Deutschland_20290331.pdf — Titelblatt und PDF-Seite 2, Standort 7.
- **[S12]** A|U|F-Zertifikatsbündel: https://www.tsr-recycling.de/uploads/tx_3slocations/Zert_AUF_2026_SPs_TSR_DE.pdf — PDF-Seite 33, Neumünster.
- **[S13]** Gespeicherte Website direkt: https://www.tsr-recycling.de/ — HTTP 200, aktueller Deutschland-/Group-Kontext.

### Gelesene Registeraufbereitung, nicht als amtlicher Direktabruf ausgeben

- **[S5]** https://www.northdata.de/TSR+Deutschland+GmbH+%26+Co.+KG,+L%C3%BCnen/Amtsgericht+Dortmund+HRA+19743 — frei lesbare Identifikation und Publikationsübersicht mit Eintragung 19.07.2024, Ausgliederung 14.10.2024 und Wirksamkeitsvermerk 21.10.2024.
- **[S6]** https://www.northdata.de/?id=5001342105157632 — vollständig lesbarer Registertext des Wirksamkeitsvermerks vom 21.10.2024. Dagegen sind die Detailseiten https://www.northdata.de/?id=4973297914871808 und https://www.northdata.de/?id=4686764238438400 (Ausgliederung) sowie https://www.northdata.de/?id=5134190775631872 (Ersteintragung) Premium-gesperrt; nur Übersichten/Titel gelesen, keine hinter der Schranke liegenden Volltexte.
- **[S7]** https://www.northdata.de/TSR+Group+GmbH+%26+Co.+KG,+L%C3%BCnen/Amtsgericht+Dortmund+HRA+18511 — aktuelle/frühere Namen, gleiches Registerblatt HRA 18511, Namensänderungsereignis 04.12.2024, Ausgliederungsverknüpfung zu Deutschland.

**Amtliches Portal:** https://www.handelsregister.de/rp_web/welcome.xhtml lieferte HTTP 200 und die Startseite. Kein firmenspezifischer aktueller/chronologischer Abdruck oder Umwandlungsvertrag dort extrahiert; **keinen erfolgreichen amtlichen Registervolltextabruf behaupten**. Portal-Erreichbarkeit allein zählt nicht als Identitätsbeleg. Websearch-Tool scheiterte mit HTTP 401; Webfetch lieferte keine verwertbare Ausgabe, direkte HTTPS-Abrufe funktionierten. Diese Werkzeuglücken ändern weder den erfolgreichen Website-Abruf noch die oben offengelegten Register-Volltextgrenzen.

## Nachtrag: reproduzierbarer Neuabruf und Owner-Zugriffsgrenze (14.10.2026)

**Owner-Gegenbefund:** Laut Owner ergab ein unabhängiger frischer urllib-GET mit User-Agent `schrott-mcp-ingestion/0.1` bei Standort, Impressum und News HTTP 200, aber nur „Challenge Validation“ (ca. 1.949 Bytes); die drei PDFs ergaben HTTP 403. Dieser Befund ist ernst zu nehmen: **HTTP 200 allein beweist weder lesbaren Fachinhalt noch Ingestion-Erreichbarkeit.** Die Zugänglichkeit ist derzeit nicht für jeden Client reproduziert. Die Ursache des Unterschieds (User-Agent, Zeitpunkt oder andere Server-/Clientmerkmale) ist nicht durch einen kontrollierten Vergleich nachgewiesen.

### Gewöhnlicher Abrufweg und Sicherung

Am **14.10.2026, 13:33:43–13:33:47 UTC (15:33:43–15:33:47 MESZ)** wurden die URLs erneut frisch über das Netz abgerufen: Linux, **Python 3.14.4**, jeweils `urllib.request.urlopen(url, timeout=45)` mit unverändertem Standard-Opener, Standard-User-Agent **`Python-urllib/3.14`**, ohne explizite Header, Cookie-Jar, vorgeschalteten Browser oder Challenge-Ausführung. Dies ist derselbe gewöhnliche Abrufweg wie bei den erfolgreichen ursprünglichen Python-Abrufen; lediglich Timeout und Evidenzspeicherung sind jetzt explizit. Standard-TLS-Prüfung blieb aktiv. Keine User-Agent-Rotation, kein Proxy-/IP-Wechsel, keine Challenge-, TLS- oder Sperrenumgehung und kein Rückgriff auf lokal gespeicherte ältere Dokumente.

Die tatsächlich neu empfangenen Antwortbytes liegen in **`/tmp/opencode/tsr-owner-evidence-20261014/`**. Für jede Antwort enthält `<Dateiname>.metadata.json` den UTC-Abrufbeginn, Original-/Final-URL, HTTP-Status, sämtliche ausgelesenen Response-Header, Bytezahl und SHA-256 sowie Abrufweg/User-Agent. Die drei PDFs beginnen mit `%PDF-` und wurden aus genau diesen gespeicherten Bytes mit `pdftotext -layout <Datei> -` extrahiert; Exitcode jeweils **0**, stderr leer. Keine HTML-Fehlerseite als PDF abgelegt oder als Zertifikat gewertet.

| Gesicherte Antwort | Quelle | HTTP | Empfangene Bytes | Inhalt erneut geprüft |
|---|---|---:|---:|---|
| `standort.html` | [S1] | 200 | 66.705 | TSR Deutschland, Leinestraße 4, Telefonnummer 6905980 enthalten; keine „Challenge Validation“. |
| `impressum.html` | [S2] | 200 | 44.000 | HRA 19743 und HRB 36274 enthalten; keine „Challenge Validation“. |
| `news.html` | [S3] | 200 | 40.684 | Umfirmierung und „Anfang Dezember 2024“ enthalten; keine „Challenge Validation“. |
| `startseite.html` | [S13] | 200 | 49.994 | TSR Deutschland enthalten; keine „Challenge Validation“. |
| `efb.pdf` | [S10] | 200 | 640.472 | Echter PDF-Body; `efb.pdf.txt` (110.116 Bytes) enthält Neumünster/Leinestraße 4 und 18.08.2027. |
| `iso.pdf` | [S11] | 200 | 550.610 | Echter PDF-Body; `iso.pdf.txt` (12.912 Bytes) enthält Leinestraße 4 und 31.03.2029. |
| `auf.pdf` | [S12] | 200 | 566.504 | Echter PDF-Body; `auf.pdf.txt` (93.059 Bytes) enthält 24539 Neumünster und 31.01.2027. |
| `gleif-deutschland.json` | [S8] | 200 | 2.418 | Neu empfangener LEI-JSON-Body gesichert. |
| `gleif-group.json` | [S9] | 200 | 2.707 | Neu empfangener LEI-JSON-Body gesichert. |

**Cache-Transparenz:** Es wurden keine alten lokalen Dateien als neuer Abruf ausgegeben. Die frischen Netzantworten sind aber nicht mit einer nachgewiesenen Neugenerierung am Origin gleichzusetzen: News/Startseite tragen serverseitige `X-TYPO3-Debug-Cache`-Hinweise auf zuvor generierte Seiten; das EFB-PDF trägt `MPULSE_CDN_CACHE: HIT`. Alle PDFs haben ältere `Last-Modified`-Daten und Cache-Laufzeiten. Diese unveränderten Header sind in den Metadaten erhalten. Der Beleg lautet daher **„frischer HTTPS-GET mit tatsächlich geliefertem Fachinhalt, teils aus serverseitigem CMS-/CDN-Cache“**, nicht „frisch am Ursprung erzeugtes Dokument“. Kein Cache-Busting oder Versuch, Cache-/Schutzmechanismen zu umgehen.

Minimal reproduzierbar für den Owner (nur gewöhnlicher GET; beim eigenen Ergebnis Inhalt und Status gemeinsam prüfen):

```python
import urllib.request
url = "https://www.tsr-recycling.de/standorte/schrotthaendler-neumuenster"
with urllib.request.urlopen(url, timeout=45) as response:
    body = response.read()
    print(response.status, response.url, len(body))
    print(body[:300])
```

Der Standard-User-Agent hängt von der Python-Version ab; hier wurde **nicht** der Ingestion-UA benutzt. Deshalb belegt dieser Nachtrag **nicht**, dass der Produktions-/Ingestion-Client derzeit lesen kann. Für Owner-Prüfung können die gesicherten Originalbytes und ihre Metadaten jetzt unabhängig geprüft werden; sie ersetzen keinen erfolgreichen erneuten Produktionsabruf und keinen amtlichen Ausgliederungsvertrag.

### Präzisierte Empfehlung zum Website-Status

Die ursprüngliche Empfehlung `website_status: aktiv` ist nur für den erfolgreich getesteten gewöhnlichen Rechercheclient uneingeschränkt bestätigt. **Nicht als universelle Erreichbarkeit oder als Freigabe für den Ingestion-UA übernehmen.** Im Owner-Gate beide Befunde mit Client und Zeit dokumentieren. Falls `website_status` operativ den maßgeblichen Ingestion-Client beschreibt, `blockiert` bis zu dessen erfolgreichem Fachinhaltabruf beibehalten und den gelungenen Rechercheabruf separat notieren; bei allgemeiner Website-Erreichbarkeit kann `aktiv` mit explizitem Hinweis auf Challenge/403 beim Ingestion-UA sachgerecht sein. Die Enum allein bildet diese Clientabhängigkeit nicht ab. Keine Dossieränderung durch diesen Nachtrag.
