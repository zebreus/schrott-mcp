# Unabhängiges Review — Welle 20261018-1202, Nordost

Review/Abrufe: **18.10.2026**, während dieser Review-Sitzung; lokale Kontrollzeit **12:14:49 UTC**. Scope: die 20 Pfade aus `/tmp/opencode/wave-20261018-1202-scope.json`, Gruppe `nordost`; Ausgangsbericht `research/wave-20261018-1202-nordost.md`. Maßstab: `README.md:183–202`.

## Urteil

**Keine unveränderte Gesamtfreigabe: Die Quellenbegründung muss korrigiert werden. Die neuen Standortfelder von Scholz Marienberg und TSR Chemnitz sind nach der hier zusätzlich tatsächlich gelesenen Registergegenprüfung jedoch haltbar; eine Rücknahme wegen fehlender Standortdaten ist nicht erforderlich. Behnckes Abstufung auf `pruefung` ist angemessen.**

Die Standortfüllungen lassen sich über die **autoritative Betreiber-Ausnahme**, nicht über „Betreiber plus EFB = zwei zulässige Quellen“ freigeben. Hierfür sind die unten dokumentierten neuen Registerabrufe ausdrücklich als **Review-Nachtrag** zu übernehmen. Sie waren kein nachgewiesener Bestandteil der ursprünglichen Welle. Keine pauschale Bestätigung sämtlicher unveränderter Legacy-Felder der 20 Dossiers.

Nur diese Reviewdatei im Repository geschrieben. Keine Dossiers oder README geändert, keine DB-Verbindung/DB-Schreibzugriffe, keine Koordinaten gesetzt oder publiziert, kein Commit/Deploy. Die zwei erlaubten PDFs wurden als temporäre Dateien unter `/tmp/opencode/` gelesen; TSR-Seiten 1/2 zusätzlich gerendert.

## 1. Befunde mit Handlungsbedarf

### F1 — Freigabeblocker der Dokumentation: EFB fälschlich als zweiter zulässiger Quellentyp

- `dossiers/sn/sn-marienberg-scholz-recycling-betrieb-marienberg.md:35`: „Zwei zulässige Quellen: Betreiber und externe ESN-Zertifizierung“.
- `dossiers/sn/sn-chemnitz-tsr-deutschland-nl-chemnitz.md:35`: „Betreiber plus externe EdDE-Zertifizierung als zwei zulässige Quellen“.
- Der Nordost-Bericht, insbesondere Zeilen 16 sowie 22–48, verwendet dies als Begründung der beiden belastbaren Verbesserungen, ohne einen tatsächlich erfolgten unabhängigen Registerabgleich dieser Betreiber zu dokumentieren.

**Das ist nach README falsch.** EFB ist kein zusätzlicher Quellentyp der abschließenden Liste in Z.186–188. Ein unabhängiger Aussteller wird durch Betreiberhosting weder zum zweiten Betreiber noch zum Handelsregister. Zertifikate sind hier sehr gute Aktualitäts-/Standortstützen im Rahmen Z.192–202, ersetzen aber nicht „entity is live in the register“. Gleiche HR-Nummer in Impressum und Zertifikat allein beweist keine heute lebende Registerentität.

**Empfehlung:** Die beiden „zwei zulässige Quellen“-Aussagen durch eine datierte Korrektur ersetzen bzw. ausdrücklich berichtigen: Betreiber-Ausnahme mit Impressum, konkret gelesener lokaler Seite/Standortdatensatz, aktuellen Zertifikatsinhalten und den nachfolgenden, neu gelesenen NorthData-Profilen. GLEIF nur als zusätzliche Gegenprüfung, nicht als stillschweigend neuer README-Quellentyp. Historische Rechercheeinträge nicht rückwirkend umetikettieren.

### F2 — Kein flächiger Legacy-Freibrief

17 der 20 Frontmatter sind gegenüber dem gesicherten Vorwellenstand unverändert. In Behncke wurde nur `status` geändert. Das ist ein enger Änderungsumfang, aber nach README Z.190–191 kein Beweis, dass verbliebene Aggregatorfelder zulässig oder aktuell sind. Insbesondere bleiben Behnckes Telefon und die zahlreichen Portal-/Seed-Adressen **ungeprüfte Bestandswerte**. Die Welle benennt diese Grenze überwiegend sauber; ein Gesamtlabel „alle 20 verifiziert“ wäre trotzdem falsch.

**Empfehlung:** Dokumentationsfortschritt und konservative Prüffälle freigeben, nicht deren Legacy-Kontaktwerte als frisch verifiziert ausgeben. Für bestätigte Veröffentlichung dieser Altwerte weiterhin einzelne zulässige Nachweise verlangen; keine automatische Wiederaktivierung aus Verzeichniskonsistenz oder Zugriffssperren.

## 2. Neue Registergegenprüfung — keine rückwirkende Legitimierung

### Scholz Recycling GmbH, Essingen

Tatsächlich mittels `webfetch` vollständig im relevanten Profilbereich gelesen:

- [NorthData-Unternehmensprofil, Ulm HRB 733963](https://www.northdata.com/Scholz%20Recycling%20GmbH%2C%20Essingen/Amtsgericht%20Ulm%20HRB%20733963): aktueller Profilname **Scholz Recycling GmbH**, Berndt-Ulrich-Scholz-Str.1, 73457 Essingen, **HRB 733963 Ulm**, EUID DEB8537.HRB733963. Profil führt aktuelle gesetzliche Vertreter, keine firmenspezifische Erloschen-Kennzeichnung im gelesenen Bereich. Publikationsübersicht nennt **20.08.2026 Geschäftsführer A. El Aoufir / Ausscheiden dreier Geschäftsführer**, 31.07.2026 Gesellschafterliste, 23.06.2026 Prokura. Keine Einzelurkunde oder vollständige Umwandlungshistorie gelesen.
- [Betreiber-Impressum](https://www.scholz-recycling.com/impressum/) direkt gelesen: dieselbe GmbH, Sitz/Adresse Essingen, HRB 733963 Ulm, Geschäftsführer **Abderrahmane El Aoufir**. Damit konkrete Namens-/Sitz-/Register-/Geschäftsführerkongruenz, nicht nur eine passende Suchüberschrift.
- [GLEIF-API LEI 529900IPVWLJXD5SPC93](https://api.gleif.org/api/v1/lei-records/529900IPVWLJXD5SPC93), HTTP200: gleiche Firma/Adresse, `registeredAs=HRB 733963`, `entity.status=ACTIVE`, `registration.status=ISSUED`, `corroborationLevel=FULLY_CORROBORATED`, keine Nachfolge-/Ablaufentität. **Datensatz-lastUpdateDate 29.11.2025**, nächste Erneuerung 07.01.2027. Der heutige API-Abruf macht daraus ausdrücklich keinen Registerauszug vom 18.10.2026.

Zusammengenommen belastbare aktuelle Betreiberqualifikation nach README; kein eigener amtlicher Tagesauszug. Ein Creditreform-Suchergebnis nannte „wirtschaftsaktiv“ und letzte Änderung 29.09.2026; **Direktabruf HTTP403**, deshalb diesen Änderungsstand nicht als selbst gelesene Registertatsache übernommen.

### TSR Deutschland GmbH & Co. KG, Lünen

- [NorthData-Unternehmensprofil, Dortmund HRA 19743](https://www.northdata.de/TSR%20Deutschland%20GmbH%20&%20Co%C2%B7%20KG,%20L%C3%BCnen/Amtsgericht%20Dortmund%20HRA%2019743), mittels `webfetch` tatsächlich gelesen: **TSR Deutschland GmbH & Co. KG**, **Brunnenstr.138, 44536 Lünen**, **HRA 19743 Dortmund**, EUID DER2402.HRA19743, LEI 529900PYWIUX8UMMSD29. Aktuelle gesetzliche Vertreter/Komplementärstruktur vorhanden, keine firmenspezifische Erloschen-Kennzeichnung im gelesenen Profil. Publikationsübersicht: Eintragung **19.07.2024**, Ausgliederungs-/Übernahmeereignisse 14./21.10.2024, Gesellschaftererwähnung Rethmann 01.12.2025 und weitere Erwähnung 19.05.2026. **Keine Original-Ausgliederungsurkunde gelesen und keine Chemnitzer Betriebskette aus diesen Überschriften bewiesen.**
- [Impressum](https://www.tsr-recycling.de/impressum/), direkter HTTP200-HTML-Abruf mit Python `requests`: gleicher Betreiber, Lünen, **HRA 19743**; persönlich haftende **TSR Deutschland Verwaltungs GmbH, HRB 36274** ist ausdrücklich ein anderer Registereintrag. Nicht HRB der Komplementärin als HRA des Betreibers ausgeben.
- [GLEIF-API LEI 529900PYWIUX8UMMSD29](https://api.gleif.org/api/v1/lei-records/529900PYWIUX8UMMSD29), HTTP200: gleicher Name/Sitz/Adresse, `registeredAs=HRA 19743`, `entity.status=ACTIVE`, `registration.status=ISSUED`, `FULLY_CORROBORATED`, keine Nachfolge-/Ablaufentität. **lastUpdateDate 28.05.2026**, nächste Erneuerung 26.07.2027. Zusatzprüfung, kein neuer Handelsregister-Tagesauszug.

Damit ist die konkrete lebende, registerkongruente Betreibergesellschaft hinreichend gegengeprüft. Das bestehende Dossier heißt bereits TSR Deutschland: keine neue Rename-/Merge-Entscheidung nötig. Der zusätzlich gefundene Creditreform-Suchtext („wirtschaftsaktiv“, HRA19743) bleibt **Indexinhalt**, da direkter Abruf HTTP403. Keine historische Betreiberkette dadurch frisch beglaubigt.

**Wichtige Abrufgrenze:** Die obigen NorthData-Profilabrufe per `webfetch` lieferten tatsächlich Firmeninhalte. Spätere unabhängige Kontrollversuche derselben Profile mit Python `requests` lieferten **HTTP429**. Beides gehört in die Evidenzbeschreibung; nicht „Register generell unzugänglich“, aber auch nicht „jeder Abruf erfolgreich“. Nach den 429 keine weiteren NorthData-Versuche. Die pauschalen Statusfilter „Aktiv/Liquidation/Erloschen“ im Seitenkopf sind **keine** firmenspezifischen Statusbelege; sie wurden nicht so verwendet.

## 3. Standort- und Frontmatterprüfung

### Scholz Marienberg — neue Felder sachlich bestätigt

[Öffentlicher Betreiber-JSON-Endpunkt](https://www.scholz-recycling.com/standorte-json-nicht-loeschen/), HTTP200, als Liste eingelesen und Marienberger Datensatz isoliert:

| Feld | Tatsächlich gelesener Inhalt / Bewertung |
|---|---|
| Betreiber/Ort | Scholz Recycling GmbH, Marienberg; kein anderer Scholz-Betrieb übertragen |
| `street`, `postcode` | Reitzenhainer Straße **35 c**, **09496** — kongruent zum Zertifikat |
| `phone`, `email` | **+49 3735 22495**, **marienberg@scholz-recycling.de** — lokale Betreiberwerte |
| `opening_hours` | Mo–Do 07–09 / 09:15–12 / 12:30–16; Fr letzter Block bis 13:45 — korrekt übernommen |
| Samstag | Standard „geschlossen“, Sondertext „1. Samstag im Monat 08–12“ — vorhandene bedingte Formulierung vertretbar, keine vorbehaltlose Samstagsgarantie |
| Aktualität | Feiertage 2026 vorhanden; Betriebsruhe **22.12.2025–03.01.2026** ist Vergangenheit, kein bevorstehender Schließungszeitraum |

[ESN-PDF](https://www.scholz-recycling.com/wp-content/uploads/260623_UeZEFB_MAR.pdf), direkter HTTP200, alle acht Seiten textlich ausgewertet: **ESN 04-110142 (26)** / **ZZEE003000304010**; Prüfung **23.06.2026**, Ausstellung **03.08.2026**, gültig bis **30.11.2027**. Seite1 gleiche GmbH/HRB; Seiten2/4/7 gleiche Marienberger Adresse. Seite4/7 beschreibt ausdrücklich **„Anlage zur zeitweiligen Lagerung und Behandlung von Eisen- und Nichteisen-Schrott (Schrottplatz)“**. Seiten5/8 AVV160106 ausdrücklich nur Fahrzeuge **außerhalb des Anwendungsbereichs der AltfahrzeugV**.

Damit `trader_type: schrottplatz` sachlich präziser als die unbelegte Seed-Autoverwertung. Keine pauschale PKW-Demontage-/Annahmeberechtigung, Preiszusage oder Privatkundenregel aus dem PDF ableiten. Checkboxzustände wurden bei diesem rein textlichen Review nicht zusätzlich visuell bestimmt; die Tätigkeitsbewertung stützt sich auf die ausdrückliche Anlagenbeschreibung.

**Empfehlung:** Adresse, PLZ, Telefon, Mail, bedingte Zeiten, Kategorie und `status: aktiv` halten — **unter korrigierter Betreiber-Ausnahme mit dem neuen Review-Registerbeleg**. Nicht das EFB selbst als zweite README-Quelle deklarieren. Fehlende konkrete Privatbedingungen rechtfertigen einen Vorbehalt, widerlegen aber nicht den jetzt belegten Schrottplatz.

### TSR Chemnitz — neue Felder sachlich bestätigt

[Detailseite](https://www.tsr-recycling.de/schrotthaendler-chemnitz/) leitet auf [kanonische Standortseite](https://www.tsr-recycling.de/standorte/schrotthaendler-chemnitz/) weiter. Python-HTTP200 mit auswertbarem vollständigem HTML, nicht bloß Übersichts-/Suchsnippet:

- TSR Deutschland GmbH & Co. KG, **Frankenberger Straße155a, 09131 Chemnitz**, **+49 371 3563998-0**.
- Zeiten **Mo/Mi/Fr07–16, Di/Do07–17:30** unverändert korrekt.
- Lokaler TYPO3-Token `nbjmup+difnojuaAuts/fv`, Vektor1, ergibt lokale Mail **chemnitz@tsr.eu**; nicht zentrale info@tsr.eu übernommen.
- Standorttext spricht ausdrücklich von Chemnitzer Gewerbe-/Industrie-/Privatkunden, vergüteter Annahme, Fe-/NE-Ankauf auch für Kleinkunden, möglicher Auszahlung vor Ort, Rückbau, Abholung/Container. Individuelles Altautoangebot ist Werbung, kein garantierter Restwert und kein örtliches Demontagezertifikat.

[EdDE-PDF](https://www.tsr-recycling.de/uploads/tx_3slocations/EfB_Zertifikat_SuedOstI_Halle_Torgau_25052027.pdf), HTTP200. Textausgabe hat defekte Zeichenzuordnung; deshalb **Seiten1/2 gerendert und selbst visuell gelesen**: **0689 / ZZEE002002296009**, **Prüfung26.11.2025**, **Ausstellung20.02.2026**, **gültig25.05.2027**. Das Ausstellungsdatum darf nicht zum Prüfdatum werden. Seite1 HRA19743 Dortmund; Seite2 **Niederlassung Chemnitz, Frankenberger Straße155a, 09131**. Sammeln/Befördern angekreuzt, Beschreibung „Containerdienst“. Nicht alle übrigen 20 Seiten visuell geprüft, keine weitergehende Checkbox-/Demontagebehauptung.

**Empfehlung:** Alle neuen Kontakt-/Adress-/Zeitfelder und `website_status: aktiv` halten; bestehendes `status: aktiv` durch die jetzt vollständige Betreiberprüfung stützbar. Leeres `webfetch`-Ergebnis bei den TSR-Webseiten war eine Toolgrenze, kein Nachweis einer toten/blockierten Website; direkte HTML-Abrufe funktionierten.

### Walter Behncke — Abstufung richtig, Telefon weiter unbelegt

[Ortsseite](https://schrottregister.pages.dev/ort-hamburg) und zusätzlich [konkretes Behncke-Profil](https://schrottregister.pages.dev/betrieb-walter-behncke-e-k-inh-bernd-behncke-hamburg) direkt gelesen: **Walter Behncke e.K., Inh.Bernd Behncke**, **Kulemannstieg32, 22457 Hamburg**, Lagern/Behandeln; Detail nennt Zertifikatskennung **ZZBT00500176**, aber keine gelesene Gültigkeit. Der Spiegel sagt selbst **„kein amtliches Angebot“**, zugrunde liegender Abruf **11.08.2026**. Er erklärt ausdrücklich, dass das Register **weder Telefon noch E-Mail** nennt und der Spiegel keine veröffentlicht.

Das ist ein konkreter Anlagenlead und ein neuer präziser Suchschlüssel für amtliche Folgeprüfung, aber **kein zweiter zulässiger Quellentyp**, kein heute selbst gelesener HR-/EFB-Einzelnachweis und kein Beleg heutiger Publikumsannahme. Eigene Behncke-/Creditreform-Suchen lieferten keinen verwendbaren exakten Firmeneintrag, teils völlig sachfremde Treffer; kein amtlicher Negativbefund. Den Creditreform-Altvermerk vom02.10. ohne konkrete URL habe ich nicht erneut bestätigt.

**Empfehlung:** `aktiv → pruefung` freigeben. Keine Schließung feststellen. Name/Adresse sind plausible, noch nicht nach README vollständig bestätigte Bestandswerte; **040/5508707 bleibt unbelegt**, nicht aus dem Schrottregister bestätigt. Wenn das Veröffentlichungsgate ausschließlich verifizierte Kontaktfelder zulässt, Telefon bis zulässigem Beleg leeren; keinesfalls den alten Verzeichnis-Ausnahmefall neu legitimieren. Neue Kundenzeiten/Website zu Recht nicht gefüllt.

## 4. Gegenbelege, Geschwister und übriger Scope

- **Scholz Chemnitz ≠ TSR Chemnitz:** derselbe heute gelesene Scholz-JSON führt **Uhlestraße20, 09120 Chemnitz**, lokale Scholz-Kontakte. Das existierende Geschwister `dossiers/sn/sn-chemnitz-scholz-recycling-betrieb-chemnitz.md` trägt bereits Uhlestr.20. Keine Verwechslung, Umadressierung oder Zusammenführung mit TSR/Frankenberger Straße155a.
- **Mittweida:** im vollständig eingelesenen Scholz-JSON kein Mittweida-Datensatz. `sn-mittweida-scholz-recycling-betrieb-mittweida` zu Recht nicht mit Marienberg oder Chemnitz gefüllt. Fehlende Listung ist kein Betriebsaufgabebeweis.
- **Marienberg B & R:** separates vorhandenes Dossier `dossiers/sn/sn-marienberg-b-r-recycling.md`, Altangaben Äußere Annaberger Straße12 und GbR Brunner/Reichel. Nur lokaler Dossierabgleich, **keine neue externe Verifikation** dieses Geschwisters. Kein Hinweis auf Identität mit Scholz, kein Merge empfohlen.
- **Behncke/Schrieber:** die heute gelesene Ortsliste führt **Gebrüder Schrieber G.m.b.H. separat am Kulemannstieg33**, Behncke an32. Geschwister `dossiers/hh/hh-schnelsen-gebruder-schrieber.md` ist ebenfalls Prüffall; dessen historische HRB15508 wurde hier nicht frisch registerverifiziert. Keine Nachbarzeiten/-telefone und keine Schrieber-Identität übertragen.

Alle 20 Scope-Dateien mit ihrem gesicherten Vorwellenpendant verglichen; **20/20 Backup-SHA256 stimmen mit Scope überein**. Genau drei Frontmatter betroffen: Marienberg sieben Schlüssel, TSR sechs, Behncke einer. Bei allen übrigen 17 nur Text-/Timelineänderungen. Historische Timelines blieben im gelesenen Diff erhalten.

| Übrige Fälle | Reviewentscheidung / Evidenzgrenze |
|---|---|
| Wedrich, Rupy, Eichler, R u.S, Reinharz | Konservativer Prüffall und Lead-Kennzeichnung vertretbar; keine neue unabhängige Vollverifikation durch diesen Reviewer. CAPTCHA/DNS/429 und Portalnummern bleiben keine Existenz-/Schließungsnachweise. |
| Dahua, Herbig, Lüpcke, Otto Wiese | Keine neue Betreiber-/Nachfolgezuordnung im Diff: Frankfurt nicht nach Hamburg, NIEMET/Bremen nicht nach Berlin, Wardenburg nicht allein wegen Registergericht nach Oldenburg, Bohmann nicht ohne Wiese-Kette. Historische Registerketten hier ausdrücklich nicht neu geprüft. |
| Mittweida | Fehlende JSON-Listung selbst bestätigt, keine Schließung oder Ersatzfiliale ableiten. |
| Hildesheim, Wagner Erfurt, Neumann, ahab | Kein neuer Frontmatter-Fill; offene mobile-/Gewerbe-/Register-/GESA-Fragen bleiben offen. Die Einzelabrufberichte der Welle hier nicht vollständig nachgestellt. |
| Schott, Lück, Kujat | Keine neue Aktivierung/Identitätskonstruktion. Schotts Arztgegenlead historisch, Lücks Verzeichnisse keine Mehrquellenbestätigung, Kujats Fehlerlink keine Betreiberwebsite. Keine frischen Register-Negativauskünfte behaupten. |

## 5. Freigabegate und präzise Rückfalloption

1. **Vor Veröffentlichung Quellenbegründung in beiden Dossiers und Wellenbericht berichtigen**, neue Review-Registerprüfung als solche ergänzen. Dann sind Marienbergs neue sieben und TSRs neue sechs Schlüssel nach Betreiber-Ausnahme sachlich freigabefähig; keine neue Rechtsnachfolgekette behaupten.
2. Behnckes `pruefung` halten. Sein Alttelefon nicht frisch bestätigen; gegebenenfalls im verifizierten Ausgabebestand zurücknehmen. Kein Behncke-Schließungsurteil und kein Merge.
3. Falls der Owner die ergänzende Registerprüfung nicht als hinreichende Betreiberqualifikation akzeptiert, ist die enge Alternative **Marienberg: `street/postcode/phone/email/opening_hours` wieder leer, `status` wieder `pruefung`; TSR: dieselben fünf neuen lokalen Felder wieder leer**. Websiteerreichbarkeit darf getrennt als `aktiv` bleiben. Marienbergs Kategorieänderung ist im expliziten Schrottplatz-Anlagentext gut gestützt, aber ebenfalls nicht mit einer erfundenen zweiten README-Quelle begründen. Keine pauschale Rücksetzung fremder Dossieränderungen.
4. Diese Rückfalloption ist **keine Empfehlung, die hier bestätigten Standortdaten zu verwerfen**, sondern die präzise Trennung zwischen belegten Betreiberangaben und noch nicht akzeptiertem Freigabegate. Tests/Whitespace-Prüfungen ersetzen dieses Sachquellengate nicht. Tests der Ursprungswelle wurden in diesem Review nicht erneut ausgeführt.

**Abschließende Empfehlung:** bedingte fachliche Freigabe der konkreten Standortfüllungen und der Behncke-Abstufung **nach Quellenkorrektur und dokumentiertem Review-Nachtrag**; keine unveränderte Freigabe der falschen Zwei-Quellen-Erzählung und keine frische Legitimation der ungeprüften Legacy-/Registerhistorie.
