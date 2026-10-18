# Owner-Check-in 18.10.2026, 10:30 – Tamm: Betreiber-/Feldreview

## Ergebnis

**Die sachliche Aktivierung ist belastbar; kein Beleg für einen vergüteten Schrottankauf.** Die bereits uncommitteten Ergänzungen zu Standort, Kontakt, Anlieferzeiten und Containerabholung sind weitgehend bestätigt. Zu verbessern sind Preisformulierungen, die explizite Grundlage beider Kundenarten und die Trennung von Anliefer- und Abholzeiten. Die bisherige Timeline ist kein Beweis für tatsächliche Abrufe am 12.10.; alle hier dokumentierten Neuabrufe gehören zum **18.10.2026**.

Auftragsscope: ausschließlich diese Reviewdatei. Dossier, Datenbanken, GEO und Commits wurden nicht verändert. Keine Koordinatenrecherche. „10:30“ bezeichnet den Check-in-Auftrag, keine behauptete sekundengenaue Abrufzeit.

## Vergleichsbasis / Quellenstandard

- Geprüft: `dossiers/nw/nw-wuppertal-ronsdorf-tamm.md` im Arbeitsbaum und in HEAD `a61dcc944d2db7094bbcf429f5f794a4fe6eb959`.
- HEAD enthält nur den Import mit `status: pruefung`, leerem Kontakt/Website/Zeiten/Leistungsfeldern, `city: Wuppertal-Ronsdorf`, leerer PLZ und dem verschmutzten Straßenfeld `Tamm GmbH Wuppertal-Ronsdorf Gasstr. 11`. Quelle des Imports: schrottradar, also nur Lead.
- Der Arbeitsbaum enthält bereits Adresse/Kontakt/Zeiten, Website und Aktivierung, dropoff/pickup, Beschreibung sowie zwei auf 12.10. datierte Timelineabschnitte. Diese Änderungen sind fremde Arbeit und wurden nicht überschrieben.
- Maßstab: `README.md:183–202`: Verzeichnisse sind keine Belege; normalerweise zwei unabhängige zugelassene Quellen. Registerkongruente, lebende Betreiber mit konkreter Standortseite und aktuellem Inhalt dürfen eigene Standort-/Leistungsfakten allein belegen. Identitätsfragen benötigen weiterhin eine zweite Quelle. Startseite und Impressum sind **eine** Betreiberquelle, keine zwei unabhängigen Belege.

## Tatsächliche Abrufe am 18.10.2026

| ID | Quelle / Abruf | Tatsächlich lesbarer Befund und Grenze |
| --- | --- | --- |
| S1 | https://www.tamm-gmbh.com/ – `webfetch`, Markdown erfolgreich | Vollständiger Leistungs-, Geschichts- und Kontakttext; Gasstraße 11, 42369 Wuppertal; Telefon/Mail; ausdrücklich **Anlieferung** Mo–Fr 7:30–16:30. Containerdienst und Abholung ausdrücklich genannt. Aktuellstes Zertifikat als `Zertifikat-EfbV-2026-Tamm.pdf` verlinkt. HTML hat kein festgestelltes Veröffentlichungsdatum. |
| S2 | https://www.tamm-gmbh.com/de/impressum – `webfetch`, Markdown erfolgreich | Tamm GmbH; „Vertreten durch: Klaus Tamm, Paul Tamm & Claudia Droste“; gleiche Anschrift/Kontakte; Amtsgericht Wuppertal HRB7490. Keine eigenständige Prüfung aller aktuellen Vertretungsbefugnisse. |
| S3 | https://www.northdata.de/Tamm%20GmbH%20Schrott,%20Metalle,%20Containerdienst,%20Wuppertal/HRB%207490 – `webfetch`, Markdown erfolgreich | Firmenname „Tamm GmbH Schrott, Metalle, Containerdienst“; AG Wuppertal HRB 7490; Gasstr. 11, D-42369 Wuppertal; LEI 529900TD0WMC4RUCFO42. Gegenstand: Handel mit edlen/unedlen Metallen und gewerblicher Containerdienst. Sichtbare Publikationen bis 14.11.2025, Jahresabschluss 2024 vom 10.11.2025. Originaldokumente/Teile der Vertreterdaten hinter Premium; kein amtlicher aktueller Registerauszug abgerufen. Allgemeine Suchfilter „Aktiv/Liquidation/Erloschen“ sind **kein** Status der konkreten Firma. |
| S4 | https://api.gleif.org/api/v1/lei-records/529900TD0WMC4RUCFO42 – `webfetch`, JSON erfolgreich | Primärquelle des LEI-Datensatzes: `legalName` = „Tamm GmbH. Schrott, Metalle, Containerdienst.“; Gasstraße 11 / 42369 Wuppertal; `registeredAs` und `validatedAs` = HRB 7490; `entity.status: ACTIVE`; `registration.status: ISSUED`; `corroborationLevel: FULLY_CORROBORATED`; `lastUpdateDate: 2026-09-07T06:10:35Z`, `nextRenewalDate: 2027-10-29T22:00:00Z`; Golden Copy vom 18.10.2026. Dies ist ein aktueller registerreferenzierter LEI-Befund, **kein** direkt eingesehener Handelsregisterauszug. |
| S5 | https://www.brd.nrw.de/umweltinspektionsberichte/stadt-wuppertal – `webfetch`, Markdown erfolgreich | Amtliche Übersicht führt Tamm GmbH an Gasstraße 11, 42369 Wuppertal mit Anlagen-Detailverweis. Kein heutiger Öffnungs-/Kontaktbeleg. |
| S6 | https://www.brd.nrw.de/umweltinspektionsberichte/Stadt-Wuppertal/W-039-Anlage-zur-zeitweiligen-Lagerung-und-Behandlung-von – `webfetch`, Markdown erfolgreich | Amtlicher Detailbericht: Tamm GmbH, Gasstraße 11, 42369 Wuppertal; Anlage zur zeitweiligen Lagerung und Behandlung von Abfällen. Berichtdatum 20.05.2025, Inspektion 12.12.2024, Ergebnis „Keine Mängel“. Historischer, unabhängiger Betreiber-/Standortbeleg; nicht als Inspektion 2026 oder heutige Vollzulassung ausgeben. |
| S7 | https://www.tamm-gmbh.com/s/Zertifikat-EfbV-2026-Tamm.pdf | `webfetch` meldete **Unsupported fetched file content type: application/pdf**, nicht HTTP 403/404. Anschließender `curl -fL --max-time 60` über Pipe an `pdftotext -layout - -` erfolgreich (Exit 0), Ausgabe nur Seitenumbrüche: kein extrahierbarer Text. OCR-Werkzeug `tesseract` nicht vorhanden (`command -v`, Exit 127). Zertifikatsinhalt, Laufzeit, Zertifikatsnummer und genauer Geltungsbereich **nicht visuell/OCR geprüft**. Der Dateiname ist kein geprüfter Gültigkeitsnachweis. |

Keine Suchsnippets/Branchenportale als Beleg eingesetzt. Keine Telefon-/Mailanfrage, kein Vor-Ort-Check; keine Vollständigkeitsbehauptung über nichtöffentliche Preise. S5/S6 sind zusätzliche amtliche Sachbelege, nicht stillschweigend als Handels-/Gewerberegister umetikettiert.

## Identität und Aktivierung

Die Kette ist konkret, nicht nur namensbasiert: **S2 HRB 7490 + Ort/Adresse ↔ S3 gleiche HRB/Adresse und längerer Firmenname ↔ S4 gleiche HRB/Adresse, aktuelle aktive Entität ↔ S6 tatsächliche Abfallanlage unter Tamm GmbH an Gasstraße 11**. Damit gehört die Website zum Betreiber des importierten Standortes; keine aus diesen Quellen erkennbare Umfirmierungs-/Verschmelzungskette nötig. Der Register-Langname ist ein Namenszusatz derselben Entität, kein Beleg eines anderen Unternehmens. Eine umfassende Negativsuche nach sämtlichen Namensvettern wurde nicht durchgeführt.

`status: aktiv` ist gegenüber HEAD `pruefung` zu empfehlen: aktiver LEI-Entitätsstatus mit Aktualisierung September 2026, registerkongruentes Impressum, konkrete betrieblich nutzbare Website mit Standortkontakt/Leistungen und 2026-Zertifikatslink, unabhängige amtliche Anlagenzuordnung. „Aktiv“ meint hier den belegten Betreiber-/Standortbetrieb, **nicht** bestätigte Auszahlung für Schrott. Die Quelle S3 erfüllt die README-zugelassene Registerdatenbankgegenprüfung; S4 macht die aktuelle Statusaussage zusätzlich explizit. Der unmittelbare amtliche Register-Livestatus und die EFB-Gültigkeit bleiben als oben beschriebene Abrufgrenzen kenntlich.

Die Betreiber-Sonderregel ist für eigene Standortfakten sachgerecht: eindeutiges Registerimpressum, konkrete Standortangaben statt generischem Formular, lebende Entität, aktueller 2026-Zertifikatsverweis. Die Aktualität des Links darf nicht zur Behauptung eines inhaltlich validierten Zertifikats ausgeweitet werden. Wer „aktueller Inhalt“ nur nach Einsicht in die Zertifikatslaufzeit akzeptiert, muss diese Einsicht nachholen; S1/S2 allein wären dann nach strengem Fallback ein Einzelbeleg insbesondere für Telefon/Mail/Zeiten. Diese Grenze rechtfertigt keine Behauptung, jene Felder seien zweimal unabhängig bestätigt.

## Feldweise Empfehlung

| Feld | HEAD → Arbeitsbaum | Empfehlung / Beleg |
| --- | --- | --- |
| `slug` | unverändert | Unverändert lassen, auch bei Ortsnormalisierung. |
| `name` | Tamm GmbH → gleich | Beibehalten: exakte Betreiber-Selbstdarstellung S1/S2 und Behördenname S6. Register-/LEI-Langnamen in die neue Timeline aufnehmen, nicht als unbelegte Umbenennung behandeln (S3/S4). |
| `trader_type` | schrotthaendler → gleich | Beibehalten: Schrott-/NE-Metallannahme S1 und Metallhandel S3. Kategorie ist kein Ankaufversprechen. |
| `state` | NW → gleich | Beibehalten; Wuppertaler Standort S1–S4. |
| `city` | Wuppertal-Ronsdorf → Wuppertal | Arbeitsbaum übernehmen: amtliche/postalische Ortsangabe in S1–S6 ist Wuppertal; Ronsdorf allenfalls historischer Ortsteilhinweis, keine Slugänderung. |
| `street` | verschmutzter Gesamtstring → Gasstraße 11 | Arbeitsbaum übernehmen; S1/S2/S4/S6, S3 abgekürzt Gasstr. 11. |
| `postcode` | leer → 42369 | Arbeitsbaum übernehmen; S1–S6. |
| `phone` | leer → 0202 4698372 | Arbeitsbaum übernehmen, führende Null erhalten; S1/S2, normalisiert aus 0202 – 4 69 83 72. Nicht unabhängig durch S3/S4 bestätigt. |
| `email` | leer → info@tamm-gmbh.com | Arbeitsbaum übernehmen; S1/S2. Keine Zustellprüfung durchgeführt. |
| `opening_hours` | leer → Anlieferung Mo–Fr 07:30–16:30 | Genau so beibehalten; S1. Weder allgemeine Bürozeiten noch Container-Liefer-/Abholfenster daraus machen. In bestehender Timeline steht fälschlich „Lieferung“; bei heutiger Einordnung klarstellen. |
| `website` | leer → https://www.tamm-gmbh.com | Arbeitsbaum übernehmen; S1/S2 sind abrufbar und registerkongruent. |
| `website_status` | leer → aktiv | Übernehmen: erfolgreicher Inhaltsabruf, keine geparkte oder fremde Domain S1/S2. Kein Telefonstatus. |
| `status` | pruefung → aktiv | Übernehmen mit heutiger Quellen-/Grenzennotiz wie oben; Ankauf bleibt offen. |
| `description` | leer → Containerdienst/Schrott-/Abfallentsorgung, kein Ankaufskurs | Inhaltlich übernehmen; Vorschlag: „Containerdienst sowie Schrott- und Abfallentsorgung für Privat- und Firmenkunden; Anlieferung und Containerabholung, Materialtrennung vor Ort. Vergüteter Schrottankauf und öffentliche Ankaufspreise nicht belegt.“ S1. |
| `dropoff_json.allowed` | leer → true | Beibehalten: S1 „Abfälle können bei uns angeliefert werden“ und Liste Schrott/NE-Metalle. |
| `dropoff_json.customer_types` | leer → privat/gewerbe | Beibehalten, aber Beleg explizit benennen: Unternehmensgeschichte S1, Abschnitt **1982**, nennt „Komplettentsorgung … von Privat- und Firmenkunden“. Historisch formulierte explizite Kundengruppenaussage im heutigen Betreiberauftritt; keine eigens datierte Zusage jeder Material-/Mengenkombination. |
| `dropoff_json.days/time_windows` | leer → Mo–Fr 07:30–16:30 | Beibehalten, S1 „Anlieferung“. Keine Samstagsöffnung/Feiertagsregel erfinden. |
| `dropoff_json.conditions` | leer → Annahme, Trennung, Konditionen anfragen, Vergütung offen | Trenngebot und Bar-/EC-/Girokartenzahlung ergänzen. „Konditionen anfragen“ als redaktionelle Handlungsanweisung, nicht als wörtliche Preiszusage des Betreibers. Keine Mindest-/Höchstmenge belegt. |
| `pickup_json.allowed` | leer → true | Beibehalten: S1 **ausdrücklich** „Wahlweise holen wir Ihre Abfälle auch mit unseren Containern ab.“ Stärker als bloße Ableitung aus „Containerdienst“. |
| `pickup_json.customer_types` | leer → privat/gewerbe | Beibehalten mit gleicher S1-Kundengruppengrundlage wie dropoff; Abholung als Containerdienst, nicht genereller mobiler Schrottankauf. |
| `pickup_json.conditions` | leer → Containergrößen, Preise auf Anfrage | Größen beibehalten, aber als Beispiele: S1 schreibt **„bspw.“** 5/7,5/10 bzw. 20/40 m³, keine abschließende Größenliste. Gebiet S1 ergänzbar. Keine kostenlose Abholung, kein Pauschalpreis, keine Mengenuntergrenze oder Abholzeit belegt. |
| `provenance_*` | unveränderte Importklassifikation | Als historische Importprovenienz beibehalten; „nur Verzeichnis-/Profilbeleg“ beschreibt den Seed, nicht den heutigen Recherchebeleg. `provenance_ankauf_raw: unklar` bleibt sachgerecht. |
| GEO-Felder/-Text | separat geändert | Hier ausdrücklich nicht bewertet; keine Empfehlung zu Koordinaten oder Geocoding. |

### Empfohlene strukturierte Bedingungen (nur Vorschlag, nicht angewandt)

```json
{
  "allowed": true,
  "customer_types": ["privat", "gewerbe"],
  "days": ["Mo", "Di", "Mi", "Do", "Fr"],
  "time_windows": [{"start": "07:30", "end": "16:30"}],
  "conditions": "Anlieferung von Schrott und Abfällen zur Entsorgung/Verwertung; Materialarten sauber trennen, nicht vermischen. Lager- und Sortierboxen sowie Separation vor Ort. Zahlung bar oder mit EC-/Girokarte. Materialbezogene Annahmebedingungen vorab klären; keine veröffentlichten Mengenlimits oder Preisbeträge auf den geprüften Seiten. Vergüteter Ankauf nicht ausdrücklich bestätigt."
}
```

```json
{
  "allowed": true,
  "customer_types": ["privat", "gewerbe"],
  "conditions": "Abfallabholung mit Containern. Betreiber nennt beispielhaft Absetzmulden 5, 7.5 oder 10 m³ und Abrollmulden 20 oder 40 m³; Ausführungen mit Klappe, Deckel oder offen. Tätigkeitsgebiet: Wuppertal, Remscheid, Solingen, Velbert, Heiligenhaus, Mettmann, Wülfrath, Radevormwald und darüber hinaus; konkreten Einsatzort und Kosten vorab klären. Keine veröffentlichten Abholzeiten, Preisbeträge oder kostenlose Schrottabholung belegt."
}
```

Die Öffnungszeiten gehören **nicht** zusätzlich ins pickup-Objekt. Containergrößen sind keine Mindest-/Höchstmengen des angenommenen Materials. S1 nennt bei Dämmwolle vorgeschriebene KMF-Säcke und bei Asbestzement geschlossene geeignete Big-Bags sowie getrennte Erfassung; diese materialbezogenen Sonderbedingungen nicht als pauschale Annahme aller Abfälle ohne Rückfrage darstellen. Kein Anlass, unbekannte Limits als Null zu modellieren.

## Preise: Ankauf ≠ Entsorgung

- **Ankauf:** S1 listet leichten/schweren Mischschrott, NE-Metalle (Aluminium, Kupfer, Zink, Blei) und Eisenspäne als Stoffe zur Entsorgung/Verwertung. Kein eindeutiges Vergütungsversprechen, keine Ankaufskurse auf S1/S2. Auch S3-Unternehmensgegenstand „Handel“ beweist weder aktuelle Ankaufskonditionen noch Auszahlung bei jeder Anlieferung. Nicht als „kauft keinen Schrott“ umkehren; unbekannt bleibt unbekannt.
- **Entsorgung:** „Die Bezahlung kann hierbei in bar sowie mit der EC-/Girokarte erfolgen“ steht im Zusammenhang der Kundenanlieferung/„preiswerten Schrott- und Abfallentsorgung“ (S1). Daher als Bezahlmöglichkeit für Entsorgungsleistungen verstehen, **nicht** als Barauszahlung des Händlers an Anlieferer. Keine veröffentlichten Gebührenbeträge auf den geprüften Seiten; keine kostenlose Annahme oder Abholung ableitbar. Auch nicht behaupten, jede Schrottanlieferung sei zwingend kostenpflichtig.
- **Verkauf:** S1 sagt ausdrücklich, geeignete Asbest-Big-Bags könnten dort **erworben** oder mit bestelltem Container angeliefert werden. „Keine Preisliste gefunden“ bleibt für S1/S2 zutreffend; „kein Verkauf“ wäre dagegen falsch. Kein Beleg eines Schrott-/Metallverkaufs an Endkunden.
- **Sprachliche Korrektur:** Arbeitsbaum „Containerpreise und Entsorgungskonditionen auf Anfrage“ ist eine sinnvolle Empfehlung, aber kein vorgefundenes ausdrückliches Preiszitat. Belastbarer: **„Auf den geprüften Betreiberseiten keine Preisbeträge veröffentlicht; konkrete Entsorgungs-/Containerkosten beim Betreiber erfragen. Ankauf/Vergütung nicht ausdrücklich belegt.“** Keine Preisdatensätze/Null-Euro-Werte erzeugen, kein Handler aus dieser Recherche ableitbar.

## Datierungs- und Integrationshinweis an den Owner

Die schon vorhandene Überschrift „Recherche 12.10.2026“ samt Behauptung damaliger Direktprüfungen wurde heute nicht historisch verifiziert. Die heutigen Abrufe beweisen nur den heutigen Inhalt. Neue S4-/S6-Befunde, die explizite Kundenartenstelle, Bezahlmöglichkeiten, Preisgrenzen und PDF-Abrufprobleme dürfen **nicht** nachträglich unter 12.10. als damalige Beobachtungen ergänzt werden. Historie erhalten; heute einen separaten Abschnitt `Recherche 18.10.2026 / Owner-Check-in` ergänzen und dort auch die Richtigstellung „Anlieferung, nicht Lieferzeiten“ sowie die PDF-Grenze dokumentieren.

Nicht aus diesem Review ableitbar: Produktionsveröffentlichung, DB-Update, heutiger amtlicher Registerauszug, geprüfte EFB-Laufzeit, telefonische Erreichbarkeit, tatsächlich verfügbare Container am Auftragstag oder konkrete Preise. Kein Commit und keine Daten-/GEO-Änderung vorgenommen; diese Datei enthält ausschließlich Empfehlungen.
