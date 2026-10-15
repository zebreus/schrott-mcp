# RUSS Entsorgung – bounded Integrationsreview 13:30

Stand/Abruf: 15.10.2026. Prüfobjekt: `dossiers/bw/bw-neu-ulm-russ-entsorgung.md`; Vorbericht `research/checkin-20261015-1130-russ-review.md` vollständig gelesen. Dies ist eine Quellen-/Entscheidungsprüfung, keine Umsetzung oder Rolloutfreigabe. Ausschließlich dieser Bericht wurde geschrieben; keine Dossier-/Codeänderung, kein Commit, Build, Deploy oder DB-Zugriff.

## Entscheidung

**Die geplante begrenzte Integration ist sachlich bestätigt:** `state: BY` statt `BW`, Name **Russ Entsorgung GmbH & Co. KG**, belegte Anschrift/Kontakte und saisonal vollständige Öffnungszeiten. Slug und Pfad können unverändert bleiben; sie sind historische Identifikatoren, keine Standortbelege. `status: pruefung` erhalten; kein GEO-/Preisfill.

- **Identität unabhängig gestützt:** Betreiber-Impressum und die erneut direkt gelesene registerbasierte North-Data-Detailseite stimmen bei exakter Firma, **AG Memmingen HRA 13628** und **Otto-Hahn-Str. 26, 89231 Neu-Ulm** überein. Das ist eine starke Zuordnung des vorhandenen RUSS-Dossiers, kein Anlass für ein neues BY-Duplikat.
- **Bayern bestätigt:** Die amtliche Gemeindezuordnung im BayernPortal sowie der dort verlinkte Eintrag des Hauses der Bayerischen Geschichte ordnen Neu-Ulm dem Landkreis Neu-Ulm und Bezirk Schwaben zu. Zusammen mit der übereinstimmenden Betreiber-/Registeradresse trägt dies BY ohne erneuten PDF-Abruf. Weder Ulmer Telefonvorwahl noch BW-Seed-Pfad beweisen BW.
- **Aktualität ausreichend für Betreiberangaben, nicht rechtlich lückenlos:** Heute abrufbare Kontakt-/Impressumsseiten und 2026-Publikationshinweise im unabhängigen Register-Aggregat stützen den gegenwärtig dargestellten Betrieb. Kein heutiger amtlicher Registerauszug, keine telefonische Betriebsbestätigung, keine Garantie tatsächlicher Öffnung an Feiertagen oder bei Sonderfällen.
- **Keine Auszahlungszusage:** Die direkt gelesene Betreiber-Leistungsseite nennt FE-Schrott und NE-Metall, aber weder Vergütung noch numerische Ankaufpreise. Entsorgung/Annahme ist kein bestätigter Schrottankauf. `pruefung` ist daher weiterhin angemessen.

## Für den Owner belegbare Werte

| Feld | Freigabefähiger Inhalt | Beleg/Grenze |
|---|---|---|
| Name | Russ Entsorgung GmbH & Co. KG | P1 und R1; nicht nur Markenname RUSS Entsorgung |
| Staat/Ort | BY / Neu-Ulm | P1, P2, R1 und A1/A2 |
| Straße/PLZ | Otto-Hahn-Straße 26 / 89231 | P1/P2 und R1; `Str.` bzw. `Strasse` sind Schreibvarianten derselben Anschrift |
| Telefon | +49 731 97950-0 (normalisiert +49731979500) | P1; P2 schreibt 0731-979500, gleiche Ziffernfolge |
| E-Mail | info@russ-entsorgung.de | Sichtbarer Header-Mailto auf P1/P2; der Impressums-Fließtext selbst ist JS-geschützt |
| Öffnungszeiten | Mo–Fr 07:00–12:00 und 13:00–16:30; März–Oktober zusätzlich Sa 09:00–13:00 **nur Selbstanlieferungen**; November–Februar Sa geschlossen; individuelle Öffnung in dieser Zeit nach vorheriger Absprache möglich | P2; keine pauschale ganzjährige Samstagöffnung, keine spontane Öffnungsgarantie |

Website bleibt die vorhandene Betreiber-Domain. Keine persönliche Ansprechpartner-Mail statt allgemeiner Mail übernehmen. Zeiten vollständig mit Saison und Selbstanlieferungsbeschränkung integrieren, nicht zu unbedingtem `Sa 09–13` verkürzen.

## Frisch direkt gelesene Quellen

### P1 – Betreiber-Impressum

<https://www.russ-entsorgung.de/impressum>

Nennt die oben angegebene Firma, Anschrift, HRA 13628/AG Memmingen und USt-ID DE345030375. Vertretung durch **NUR Neu-Ulmer Recycling Verwaltungs-GmbH, HRB 7450, AG Memmingen**, vertreten durch Peter Schaible und Thomas Kentischer. NUR ist damit die Vertretungsgesellschaft; nicht als alternativer Name oder zweiter Standort der KG übernehmen. Telefon im Impressum, allgemeine E-Mail im Seitenheader direkt lesbar.

### P2 – Betreiber-Kontaktseite

<https://www.russ-entsorgung.de/kontakt>

Gleicher Firmen-/Adress-/Telefonblock, konkrete Anfahrt zum Firmengelände sowie die in der Tabelle vollständig wiedergegebenen saisonalen Zeiten. Dadurch handelt es sich nicht lediglich um eine Verwaltungspostadresse aus einem Impressum. Selbstanlieferung wird erwähnt, eine Vergütung für angelieferten Schrott nicht.

### R1 – Unabhängige registerbasierte Detailseite, kein Registeroriginal

<https://www.northdata.de/Russ%20Entsorgung%20GmbH%20%26%20Co.%20KG,%20Neu-Ulm/Amtsgericht%20Memmingen%20HRA%2013628>

Die tatsächlich gelesene Unternehmenssektion nennt **Russ Entsorgung GmbH & Co. KG**, auch den Namen **Recycling Burlafingen GmbH & Co. KG**, **HRA 13628**, EUID **DED2505V.HRA13628** und Otto-Hahn-Str. 26, D-89231 Neu-Ulm. Sichtbare Publikationsübersicht: Eintragung 01.06.2021 als Recycling Burlafingen; Name/Anschrift/Firmensitz Neu-Ulm 01.07.2021; persönlich haftende Gesellschafterin NUR 06.07.2021; Prokura 19.01.2026; Gesellschafterhinweis über GWV 26.02.2026.

**Evidenzgrenze:** North Data ist eine vom Betreiber unabhängige, automatisierte Sekundärdarstellung öffentlicher Registerdaten und warnt selbst vor möglichen Fehlern. Kein beglaubigter/amtlicher aktueller Auszug und keine Originalbekanntmachung wurden in diesem Review gelesen. Damit ist „unabhängig registerbasiert gegengeprüft“ bestätigt, **„heute im amtlichen Originalregister geprüft“ wäre falsch**. P1/P2 sind untereinander nicht unabhängig. R1 belegt weder Öffnungszeiten noch Email unabhängig. Aus der Publikationsübersicht allein keine lückenlose Rechtsnachfolge seit 1981 oder konkreten Umzug ableiten.

### A1/A2 – Amtliche Ortszuordnung statt neuer PDF-Abhängigkeit

- <https://www.bayernportal.de/dokumente/behoerde/20108987458>: Große Kreisstadt Neu-Ulm, 89231 Neu-Ulm, **Landkreis Neu-Ulm**; herausgegeben vom Bayerischen Staatsministerium für Digitales. Verlinkt den folgenden amtlichen Gemeindeeintrag.
- <https://www.hdbg.eu/gemeinden/index.php/detail?rschl=9775135>: Haus der Bayerischen Geschichte, Große Kreisstadt Neu-Ulm, **Bezirk Schwaben**, **Landkreis Neu-Ulm**, PLZ 89231; Wappentext nennt die bayerischen Landesfarben. Unterstützt die bayerische Gemeindezuordnung, nicht den Betrieb als solchen. Dort ebenfalls angezeigte Gemeinde-Koordinaten sind **keine Betriebskoordinaten** und werden nicht übernommen.

### P3 – Leistungsumfang

<https://www.russ-entsorgung.de/abfaelle-zur-verwertung>

Titel „Schrott / Metalle“, zulässig „FE-Schrott“ und „NE-Metall“. Ausgeschlossen werden Elektronikschrott, Ölradiatoren, Gasflaschen, Druckbehälter, Nachtspeicheröfen, Reifen mit Felgen sowie gefährliche/staubige Stoffe. Containergröße auf Kontaktanfrage. Auf dieser Seite keine numerischen Preise oder Auszahlungszusage. Weder eine generelle Ankaufabsage noch kostenlose Entsorgung folgt daraus.

### P4/P5 – Aktualität und Gruppe

- <https://www.russ-entsorgung.de/service/downloads>: heute direkt gelesener HTML-Downloadbereich verlinkt „Zertifikat Entsorgungsfachbetrieb“, „Anhang zum Zertifikat“, „Bestätigung Vorbehandlungsanlage 2026“ und Mindestlohnerklärung 2026. Das ist ein aktuelles Pflegeindiz, **kein neuer Inhalts-/Gültigkeitsnachweis der PDF-Dokumente**.
- <https://www.russ-entsorgung.de/ueber-uns/die-firma>: „Teil der GWV-Gruppe seit Juli 2021“, Geschäftstätigkeit seit 1981 und Entsorgungsleistungen. Konzernzugehörigkeit ist kein Nachweis einer Verlagerung nach BW, einer Schließung oder eines anderen rechtlichen Betreibers. Das Gründungsnarrativ nicht ungeprüft zur Existenz der heutigen KG seit 1981 umdeuten.

## PDF-, Quellen- und Umfangsgrenzen

1. **PDF-Sperre respektiert:** Der Auftrag meldet den zuletzt fehlgeschlagenen Python-GET mit HTTP 403. Hier wurde kein PDF erneut angefordert und keinerlei alternative Abrufroute, Header-/Clientwechsel oder anderer Umgehungsversuch eingesetzt. Die im 11:30-Bericht behaupteten PDF-Inhalte einschließlich Zertifikatsnummer, Laufzeit und „Bundesland: BY“ sind ausschließlich Vorberichtsevidenz, nicht in diesem Lauf neu verifiziert. Downloadlinks/Dateinamen allein bestätigen diese Inhalte nicht. Die Integrationsentscheidung bleibt durch HTML-/Registergegencheck und amtliche Gemeindezuordnung tragfähig.
2. Ein zunächst versuchter anderer BayernPortal-Pfad lieferte HTTP 400; der oben zitierte, per Suche gefundene korrekte Behördeneintrag wurde danach vollständig gelesen. Die Ulmer Stadtseite zur Nachbarstadt lieferte HTTP 403; ihr Suchsnippet wird nicht als gelesener Primärbeleg gewertet. Kein Bypass.
3. Der Vorbericht grenzt Thomas Ruß/Knetzgau und GWV Ulm ab. Diese beiden Fremdstandorte wurden hier nicht erneut recherchiert; ihre Details nicht als frisch bestätigt ausgeben. Die heutige exakte Übereinstimmung von Firma/HRA/Adresse reicht für die konkrete RUSS-Zuordnung, beweist aber nicht, dass nirgendwo ein weiterer Namensvetter existiert.
4. Kein vollständiger Preis-/Shop-/AGB-Audit. Aussagen nur als „auf den geprüften Seiten nicht verifiziert“ formulieren, nicht „Preise sind nirgends veröffentlicht“. Keine Preiszahlen oder Gratisleistungen ergänzen.
5. Kein GEO recherchiert oder für den Betrieb verifiziert. Kein tatsächlicher DB-/Produktionszustand geprüft. Slug-/Pfadstabilität und Frontmatter-/Seed-Verarbeitung sind hier keine technisch getestete Garantie; Integrationsgate bleibt beim Owner.

## Konkrete Integrationsempfehlung

Die oben genannten Werte dürfen in einer separaten Owner-Änderung übernommen werden. Historische Timeline erhalten, neuen datierten Korrektureintrag ergänzen und den bisherigen offenen „Bundesland-/Entitätsklärfall“ im aktuellen Überblick nicht unverändert fortschreiben. **Es ist eine Landesklassifikationskorrektur, kein belegter Umzug.** Unsicherheit bleibt beim Ankauf-/Vergütungsmodell und bei nicht erneut gelesenen Zertifikatsinhalten, nicht beim bayerischen Ort der konkret zugeordneten Anschrift. Status `pruefung`, kein GEO-/Preisfill, kein neues Dossier und keine automatische Typ-/Aktiv-Heraufstufung.
