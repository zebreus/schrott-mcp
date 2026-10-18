# Review A – Check-in 18.10.2026, 03:30

## Ergebnis und Prüfgrenze

**Siebenmal bedingt integrierbar, nicht unverändert:** Die vorsichtige lokale Textintegration ist nach den unten angegebenen Ersatztexten vertretbar. Historische Direktabrufe und erneute Negativsuchen vom 12.10.2026 sind damit ausdrücklich **nicht bestätigt**. Keine Freigabe alter Stammdaten, Geschäftsaktivität, Preise, GEO oder Produktion.

Geprüft wurden genau die sieben vollständigen Dateien aus `shards.a` in `/tmp/opencode/checkin-20261018-0330-review-scope.json` und jeweils der vollständige Diff gegen HEAD `b052b2ba248bbeb8511023c52af6a12cae0c6643`. Alle sieben SHA-256 entsprechen dem Scope. Für jede Datei separat technisch nachgewiesen: Frontmatter bytegleich zu HEAD; sämtliche nichtleeren bisherigen Timeline-Zeilen unverändert in ursprünglicher Reihenfolge erhalten. Die Änderungen bestehen ausschließlich aus Überblickstexten und zusätzlichen Timeline-Abschnitten. Das schützt die Historie, ist aber kein Nachweis der darin behaupteten damaligen Recherche.

Maßstab: `README.md:160–202` (Identität, Gegenprüfung, Verzeichnisse nur Leads, zwei zulässige unabhängige Quellen beziehungsweise eng begrenzte Betreiberprimärquellen-Ausnahme) sowie `README.md:204–240` (Betreiberketten, Slugstabilität, Historien-/Overwrite-Gate). Timeline-Texte werden gemäß `README.md:111–126` zu Seed-Notizen; auch ohne Frontmatteränderung sind falsche Abrufbehauptungen deshalb relevant.

Im bereitgestellten Scope und den geprüften lokalen Dossier-/Research-Unterlagen liegt kein Such- oder Abrufprotokoll vor, das die neuen Recherchebehauptungen vom 12.10. belegt. Die sieben URLs beziehungsweise früheren Timeline-Aussagen allein sind kein solches Protokoll. Das ist keine Aussage darüber, ob außerhalb dieses geprüften Materials noch Protokolle existieren. Historische HEAD-Timeline nicht nachträglich umschreiben; die **noch nicht integrierten neuen** 12.10.-Abschnitte durch korrekt datierte heutige Bewertungen ersetzen.

## Heutige direkte Belegprüfung (18.10.2026)

Die folgenden Ergebnisse stammen aus direkten `webfetch`-Abrufen während dieses Reviews, nicht aus Such-Snippets. Sie beglaubigen keine Abrufe am 12.10. und keine umfassende Suche.

| Quelle | Tatsächlich lesbares Ergebnis und Grenze |
| --- | --- |
| https://www.mr-braun.de/ | „Tel.: 017646157693“; Kontaktblock „Am Bellensee 6“, „76865 Rohrbach“, `info@mr-braun.de`. Werbung für Privat/Gewerbe, Anlieferung/Abholung und Container; „faire Tagespreise“, keine bezifferten Metallkurse im gelesenen Text. |
| https://www.mr-braun.de/impressum/ | „Martin Braun / Metall Recycling Braun / Wollmesheimer Hauptstraße 24 / 76829 Landau in der Pfalz“; „Telefon: +49 176 4615769“; USt-ID `DE461381403`. Kein Register-Lebendnachweis und keine ausdrückliche Erklärung einer Hof-/Sitztrennung. |
| https://www.mr-braun.de/kontakt-schrottankauf-rohrbach/ | „76865 Rohrbach (Pfalz)“, „Telefon: +49 123 456789“, `info@mr-braun.de`. Die Nummer wirkt wie ein Platzhalter; keine bestätigte erreichbare Rufnummer. |
| https://www.mr-braun.de/schrottankauf-haeufige-fragen/ | Eisen, Stahl, Kupfer, Aluminium, Edelstahl, Messing und Kabelreste; „Sie können Ihr Altmetall direkt anliefern oder unseren Abholservice nutzen“; Abholung größerer Mengen, Container und Privatpersonen ausdrücklich genannt; „Jetzt Preis anfragen“. Keine numerischen Ankaufskurse, Verkaufspreise oder Gebühren im gelesenen Text. Nicht gleichbedeutend mit kostenloser Leistung. |
| https://brh-recycling.de/ | Im Markdown nur ein externes Trackingbild und „Do Not Sell or Share My Personal Information“. Keine lesbaren BRH-Kundeninformationen, kein Impressum und keine Domain-Betreiberzuordnung aus dieser Antwort. Heutiges Ergebnis nicht als erneute JS-Challenge oder bloßer technischer Platzhalter spezifizieren. |
| https://www.northdata.de/BRH%20Bochumer%20Rohstoffhandel%20GmbH,%20Bochum/HRB%2019036 | Direkt gelesene Unternehmensdatenseite: „BRH Bochumer Rohstoffhandel GmbH“, „Amtsgericht Bochum HRB 19036“, „Dieselstr. 45-47, D-44805 Bochum“, Gegenstand „Der Schrotthandel“. Kein heute gelesener Telefonbeleg, keine Kundenannahme. Northdata ist eine nach README zulässige Unternehmensdatenquelle, kein hier selbst eingeholter amtlicher Registerauszug. |
| https://firmeneintrag.creditreform.de/44805/4050638012/BRH_BOCHUMER_ROHSTOFFHANDEL_GMBH | HTTP 403. Inhalt und Telefonnummer heute nicht bestätigt. Kein zweiter heute lesbarer Unternehmensbeleg. |
| https://tierheilpraxis-henke.com/schrottabholungcelle | HTTP 404. Kein heute lesbarer Schrotttext und kein Betreiber-Impressum geprüft. Weder ursprünglichen Seiteninhalt noch dessen behauptete Übereinstimmung bestätigt; kein Schließungsnachweis. |

Die vier Braun-Seiten sind **eine Betreiberquellenfamilie**, nicht vier unabhängige Quellen. Impressum mit USt-ID statt Register-/Lebendnachweis und widersprüchliche Kontakte erfüllen die umfassende autoritative Ausnahme des README nicht. Die Belege tragen attributierte Websiteaussagen und die offene Widerspruchsfrage, keine neue Standort- oder Identitätsfreigabe. Für Dingel, Budnik, Wiese und Gerolstein ist keine neue Vollsuche erforderlich, um den vorsichtigen Aktenstatus zu beschreiben; es wird hier auch keine behauptet.

## 1. Dingel – bedingt integrieren nach Ersatz

Datei: `dossiers/ni/ni-bad-salzdetfurth-dingel-schrott.md`

- Identität weiterhin offen: Verzeichniskonsens ist kein Doppelbeleg. ZAH weder Alias noch Nachfolger; keine Schließung ableiten.
- Neu hinzugefügt werden eine „erneute Primärquellensuche“ und deren negatives Ergebnis ohne Protokoll. Die angegebenen zwei Portale belegen keinen solchen Suchlauf. Auch „Seed-Adresse“ ist irreführend: `street`/`postcode` sind leer; Hinter dem Dorfe 25 ist nur ein historisch genannter Lead.
- Frontmatter und ältere Historie bleiben vollständig erhalten, aber deren Abrufbehauptungen werden nicht neu bestätigt.

**Ersatz des neu angefügten Überblicksabsatzes** (der mit „Die erneute Primärquellensuche“ beginnt):

> Die Aktenprüfung vom 18.10.2026 stellt keinen neuen Betreiber-/Registerbeleg fest: Die im Dossier genannten Kontakt- und Mindestmengenangaben bleiben Verzeichnisleads. Status `pruefung`; die nur als Lead genannte Adresse ist nicht als Händlerstandort bestätigt. In den vorliegenden Unterlagen sind keine belegten numerischen Ankaufskurse, Verkaufspreise oder Entsorgungsgebühren dokumentiert. Eine heutige erneute Web- oder Registersuche wird damit nicht behauptet.

**Ersatz des neuen Abschnitts `### Recherche 12.10.2026`**, ohne ältere Abschnitte zu verändern:

```markdown
### Aktenprüfung 18.10.2026

- [Recherche 18.10.2026: Heutige Prüfung der vorhandenen Dossierunterlagen, kein protokollierter erneuter Suchlauf. Die in der Recherchehistorie genannten Angaben Hinter dem Dorfe 25, Telefon 0173 5664616 und Mindestmengen 100 kg Eisen/1 kg NE bleiben unbestätigte Verzeichnisleads; die früheren Abrufe werden nicht neu beglaubigt. Kein Betreiber-/Registerbeleg in den vorliegenden Unterlagen, keine Zuordnung zu ZAH und kein Schließungsnachweis. Frontmatter unverändert, Status pruefung; keine Standort- oder Preisfreigabe. Quellenbasis: vorhandene Timeline, insbesondere 06.10.2026; dort genannte Leads https://lokaleschrottplatz.de/dingel-schrott/ und https://www.11880.com/branchenbuch/bad-salzdetfurth/060440092B112739424/dingel-schrott.html, heute nicht erneut als Sachbelege verifiziert.]
```

## 2. Schrott-Abholung-celle – bedingt integrieren nach Ersatz

Datei: `dossiers/ni/ni-celle-29225-schrott-abholung-celle.md`

- Denickestraße ohne Hausnummer und 0162 8769770 bleiben portalgestützte Altfelder. Die frühere „Owner-Ausnahme Telefon“ in HEAD ist keine heute erneuerte Freigabe nach README.
- Der neue Text erklärt Inhalt und fehlendes Impressum einer fremden Domain zu einem historischen Direktbefund. Das lässt sich aus der heutigen 404-Antwort nicht prüfen. Fremdes Domain-Thema allein wäre außerdem kein Beweis für fehlende Betreiberidentität; erforderlich wäre der konkrete Inhalt/Impressumsbezug.
- Keine Fills, Slugänderung oder Historienverluste; weder 24/7 noch Mindestgewichte/Abholung als verifiziert behandeln.

**Ersatz des neuen Überblicks:**

> Der Datensatz bleibt ein ungeklärter Portal-/SEO-Lead. Denickestraße, 29225 Celle und Telefon 0162 8769770 sind unverifizierte Altfelder; 24/7-Erreichbarkeit und Mindestmengen sind lediglich in der bisherigen Recherchehistorie genannte Portalangaben. Die zusätzlich angeführte URL unter tierheilpraxis-henke.com lieferte im Review am 18.10.2026 HTTP 404; ihr früherer Inhalt und eine Betreiberzuordnung sind damit nicht bestätigt. Kein Betreiber-/Registerbeleg in den vorliegenden Unterlagen; Status `pruefung`, Annahme und Abholung nicht freigegeben. 100 kg Eisen/1 kg NE wären Mindestmengen, keine Preise. Numerische Ankaufskurse, Verkaufspreise und Gebühren sind in den vorliegenden Unterlagen nicht belegt.

**Ersatz des neuen Timeline-Abschnitts:**

```markdown
### Belegprüfung 18.10.2026

- [Recherche 18.10.2026: Die angeführte URL https://tierheilpraxis-henke.com/schrottabholungcelle lieferte beim heutigen Direktabruf HTTP 404. Früheren Seiteninhalt, Impressum und behauptete Wiederholung von Portalangaben damit nicht bestätigt; insbesondere keinen Abruf am 12.10.2026 beglaubigt. Die vorhandene Dossierhistorie liefert weiterhin keinen qualifizierten Betreiber-/Registerbeleg. Adresse, Telefon, 24/7-Angabe und Mindestmengen bleiben unverifizierte Altfelder beziehungsweise Portalbehauptungen; keine bestätigte Annahme, Abholung oder Preisquelle. Frontmatter und status pruefung unverändert, keine Schließung abgeleitet. Quelle: heutiger Abruf der genannten URL; bisheriger Lead laut Akte https://lokaleschrottplatz.de/schrott-abholung-celle/, heute nicht neu verifiziert.]
```

## 3. Budnik – bedingt integrieren nach Ersatz

Datei: `dossiers/nw/nw-arnsberg-budnik.md`

- Kontaktdaten und Beschreibung sind alte Verzeichnisfelder, kein aktueller Betreiberbeweis. Der Frontmattername lautet nur `Budnik`, nicht `Rolf Budnik`; dieser Personenname ist die Zuordnung der älteren Timeline.
- Der neue 12.10.-Eintrag behauptet eine nicht protokollierte Namens-/Adresssuche. Durch Aktenbewertung ersetzen; auch pauschale „keine Preise“-Sätze auf die vorhandenen Unterlagen begrenzen.
- Die alte Aktiv-/Prüfungskorrektur und der alte unzureichende Hausnummern-Geocoding-Befund sind bewahrt. Kein neues GEO und keine Bestätigung des alten Suchlaufs.

**Ersatz des neuen Überblicks:**

> Der Frontmattername lautet Budnik; die bisherige Recherchehistorie ordnet die vorhandenen Kontaktfelder dem Verzeichnislead „Rolf Budnik Schrott-RohstoffHdl.“, Sternhelle 18, 59757 Arnsberg-Herdringen, Telefon 02932 36942 zu. Eine aktuelle Betreiberidentität ist damit nicht bestätigt. Das in der Historie genannte Portal-Aktualisierungsdatum ersetzt keine Betreiberprüfung. Ein Betreiber-/Registerbeleg und betriebsspezifische numerische Ankaufskurse, Verkaufspreise oder Gebühren sind in den vorliegenden Unterlagen nicht dokumentiert. Status `pruefung`; alte Kontakt- und Beschreibungsfelder bleiben unverifiziert. Auch der frühere Geocoding-Befund bestätigt die Hausnummer nicht und wird heute nicht neu geprüft.

**Ersatz des neuen Timeline-Abschnitts:**

```markdown
### Aktenprüfung 18.10.2026

- [Recherche 18.10.2026: Heutige Bewertung der vorhandenen Dossierhistorie, keine erneute Namens-/Adress- oder Registersuche behauptet. Sternhelle 18 und 02932 36942 bleiben Verzeichnisleads; ein passender Betreiber-/Registerbeleg sowie numerische Ankaufskurse, Verkaufspreise oder Gebühren sind in den vorliegenden Unterlagen nicht dokumentiert. Frühere Abrufe und Geocoding-Ergebnisse nicht neu bestätigt. Frontmatter unverändert, status pruefung; keine Kontakt-, Standort- oder Aktivitätsfreigabe und keine Schließung abgeleitet. Quellenbasis: vorhandene Timeline 02./03.10.2026; dort genannter Lead https://www.11880.com/branchenbuch/arnsberg-westfalen/120674719B28964503/rolf-budnik-schrott-rohstoffhdl.html, heute nicht neu verifiziert.]
```

## 4. BRH Bochum – bedingt integrieren nach Ersatz

Datei: `dossiers/nw/nw-bochum-brh.md`

- Heutige Northdata-Seite trägt Firmenname, HRB, Anschriftbereich und Unternehmensgegenstand. Das ist keine Bestätigung laufender Kundenannahme oder Eigentümerschaft der Domain.
- Creditreform heute 403: Die alte Telefonnummer darf nicht durch „bleiben durch … Unternehmensdatenquellen gestützt“ unbemerkt als heute zweifach geprüft erscheinen. Bestehende Felder werden nicht gelöscht, aber auch nicht neu freigegeben.
- Neue historische Direktabrufbehauptung vom 12.10. ersetzen. Heutige Root-Antwort zeigt keinen Betreiberinhalt; ihre technische Gestalt präzise beschreiben. Das alte `website_status: blockiert` wird hier nicht aktualisiert oder neu bestätigt.
- Keine Überblicksänderung im Diff; bestehender Überblick bleibt historische Aktenlage, nicht Ergebnis einer heutigen vollständigen Zweiquellenverifikation. Keine Betreiberkette, Umbenennung oder Standortverlagerung nachgewiesen.

**Ersatz ausschließlich des neuen Timeline-Abschnitts:**

```markdown
### Belegprüfung 18.10.2026

- [Recherche 18.10.2026: Die heute direkt gelesene Northdata-Unternehmensseite nennt BRH Bochumer Rohstoffhandel GmbH, Amtsgericht Bochum HRB 19036, Dieselstr. 45-47, 44805 Bochum und den Gegenstand Schrotthandel. Der angeführte Creditreform-Link lieferte HTTP 403; die dort früher dokumentierte Telefonnummer 0234 90413152 wurde heute nicht neu bestätigt. brh-recycling.de lieferte im lesbaren Abruf nur ein externes Trackingbild und einen Datenschutzlink, keine BRH-Kundeninformationen oder Betreiberzuordnung. Keine historischen Direktabrufe vom 12.10.2026 bestätigt. Keine heutige Annahme-, Aktivitäts- oder Preisfreigabe, keine Schließung abgeleitet. Frontmatter einschließlich status pruefung und des historischen website_status blockiert unverändert; daraus keine erneute Freigabe der Altfelder ableiten. Quellen: https://www.northdata.de/BRH%20Bochumer%20Rohstoffhandel%20GmbH,%20Bochum/HRB%2019036 ; https://firmeneintrag.creditreform.de/44805/4050638012/BRH_BOCHUMER_ROHSTOFFHANDEL_GMBH ; https://brh-recycling.de/ .]
```

## 5. Wiese – bedingt integrieren nach Ersatz

Datei: `dossiers/nw/nw-porta-westfalica-wiese.md`

- Name im Frontmatter nur `Wiese`; „Gerhard Wiese Alt- und Abfallstoffe“ ist die frühere Verzeichniszuordnung. Die alte Einzelfill-Ausnahme ist keine heutige Identitätsprüfung.
- Neue „Exakte Suche“ ohne Suchprotokoll und „einziger konkreter Nachweis“ zu weitgehend: nur als in der vorliegenden Akte dokumentierter Lead formulieren. Kein Negativbeweis zur Existenz oder zu sämtlichen Quellen.
- Kein belegter Betreiber-/Register-Doppelbeleg; ältere widersprüchliche Suchbefunde bleiben als Historie erhalten. Kein neuer Geocoding-Auftrag aus altem Adressfill.

**Ersatz des neuen Überblicks:**

> Der Frontmattername lautet Wiese. Die bisherige Recherchehistorie führt den 11880-Lead „Gerhard Wiese Alt- und Abfallstoffe“, Zur Porta 19, 32457 Porta Westfalica (Lerbeck), Telefon 0571 74313. Dieser Einzelfund bestätigt weder die heutige Betreiberidentität noch die Annahme vor Ort. Ein Betreiber-/Registerbeleg und numerische Ankaufskurse, Verkaufspreise oder Gebühren sind in den vorliegenden Unterlagen nicht dokumentiert; eine heutige erneute Suche wird nicht behauptet. Kontaktwerte bleiben unverifizierte Altfelder, nicht als bestätigte Anlieferadresse verwenden. Status `pruefung`; keine Schließung abgeleitet.

**Ersatz des neuen Timeline-Abschnitts:**

```markdown
### Aktenprüfung 18.10.2026

- [Recherche 18.10.2026: Heutige Aktenbewertung, keine erneute Exakt-, Betreiber- oder Registersuche behauptet. Die Zuordnung zu Gerhard Wiese, Zur Porta 19 und 0571 74313 beruht in der vorliegenden Historie auf einem 11880-Verzeichnislead; ein qualifizierter Betreiber-/Registerbeleg ist darin nicht dokumentiert. Alte Abrufe nicht neu bestätigt. Frontmatter unverändert, status pruefung; Kontakt- und Standortfelder bleiben unverifiziert. Keine belegten numerischen Ankaufskurse, Verkaufspreise oder Gebühren in den geprüften Unterlagen; keine Schließung abgeleitet. Quellenbasis: vorhandene Timeline, insbesondere Recherche 02.10.2026; dortiger 11880-Einzelfund ohne konkrete Detail-URL, kein heute neu gelesener Sachbeleg.]
```

## 6. Gerolstein – bedingt integrieren nach enger Aktenformulierung

Datei: `dossiers/rp/rp-gerolstein-gerolstein.md`

- Orts-/Suchcluster statt identifizierter Händler: zutreffende Vorsichtsrichtung. Keine reale Firma aus „Gerolstein“ oder Sarresdorfer Str. 8 konstruieren. Portalsuchen in der alten Historie sind kein amtlicher Negativauszug und kein Lösch-/Schließungsgrund.
- Der neue Timeline-Text ist bereits überwiegend eine Bewertung vorhandener Historie, kein klar behaupteter Direktabruf. Ohne Nachweis des Bewertungszeitpunkts nicht auf 12.10. datieren. Den Überblick auf dokumentierte Aktenlage statt unbeschränkten Negativbefund begrenzen.
- Kein Feldfill und kein Historienverlust. Ältere Formulierung „Löschkandidat“ bleibt historische Notiz, wird nicht als heutige Handlungsfreigabe übernommen.

**Ersatz des neuen Überblicks:**

> „Gerolstein“ ist hier ein Orts-/Suchcluster, kein bestätigter Betreibername. Sarresdorfer Str. 8 ist nur ein importierter Adresslead; in den vorliegenden Dossierunterlagen ist keine Verbindung zu einer konkreten Person/Firma oder bestätigten Annahmestelle belegt. Ein Betreiber-/Registerbeleg sowie numerische Ankaufskurse, Verkaufspreise oder Gebühren sind darin nicht dokumentiert. Status `pruefung`; keine Schließung oder Nichtexistenz behauptet. Keine heutige erneute Web- oder Registersuche und keine Standort-/Geocodingfreigabe.

**Ersatz des neuen Timeline-Abschnitts:**

```markdown
### Aktenprüfung 18.10.2026

- [Recherche 18.10.2026: Heutige Bewertung des importierten Leads Sarresdorfer Str. 8 und der vorhandenen Recherchehistorie. Darin ist kein zugeordneter Betreibername oder Betreiber-/Registerbeleg dokumentiert. Die frühere GS-/11880-Trefferprüfung ist kein Sachbeleg für Nichtexistenz; ihr damaliger Suchlauf wird heute nicht neu bestätigt. Keine neue externe Suche behauptet. Frontmatter unverändert, status pruefung; Ortsname bleibt ungeklärter Suchcluster statt Firmenidentität. Keine bestätigte Annahmestelle, keine belegten numerischen Ankaufskurse, Verkaufspreise oder Gebühren in den geprüften Unterlagen. Quellenbasis: Timeline Recherche 02.10.2026 und Importiert, Seed-Stand 2026-09-30.]
```

## 7. Metall Recycling Braun – bedingt integrieren mit heutigem Primärbeleg

Datei: `dossiers/rp/rp-rohrbach-metall-recycling-braun.md`

- Der materielle Widerspruch ist heute direkt belegbar: Rohrbacher Kontaktadresse auf Root versus Landauer Impressumsadresse, unterschiedliche Telefonnummern und mutmaßlicher Kontaktseiten-Platzhalter. Keine ausdrückliche Hof-/Sitzrollenklärung. Betreibername Martin Braun ist eine Websiteangabe, keine unabhängige Identitätskette.
- Vier Betreiberseiten bleiben eine Quellenfamilie. Kein Nachweis der Gleichheit oder Rechtsnachfolge mit Südpfalz Metall; bestehende Warnung vor Fusion beibehalten. Keine aktuelle Registerprüfung behaupten.
- Neuer Text behauptet direkte Wiederholungsabrufe am 12.10.; heutige Belege rechtfertigen nur einen Eintrag vom 18.10. Die Preisnegativaussage auf genau die vier gelesenen Seiten beschränken; „Tagespreise“/„Preis anfragen“ sind keine Zahlenliste und keine kostenlosen Dienstleistungen.
- Frontmatter unverändert, aber `status: aktiv` ist ein **nicht neu freigegebenes Altfeld**. Daraus keine Bestätigung des Hofs oder Standortes ableiten. Auch die alte Koordinatenwarnung ist keine heutige GEO-Prüfung.

**Ersatz ausschließlich des neuen Timeline-Abschnitts:**

```markdown
### Belegprüfung 18.10.2026

- [Recherche 18.10.2026: Root, Impressum, Kontakt und FAQ heute direkt gelesen; kein historischer Direktabruf vom 12.10.2026 bestätigt. Root nennt Am Bellensee 6, 76865 Rohrbach und 017646157693; Impressum nennt Martin Braun, Metall Recycling Braun, Wollmesheimer Hauptstraße 24, 76829 Landau in der Pfalz und +49 176 4615769. Kontaktseite nennt Rohrbach ohne Straße und die wie ein Platzhalter wirkende Nummer +49 123 456789. Keine ausdrückliche Hof-/Geschäftssitzrollenklärung; vier Seiten derselben Website sind kein unabhängiger Zweitbeleg. FAQ nennt Eisen, Stahl, Kupfer, Aluminium, Edelstahl, Messing und Kabelreste, direkte Anlieferung, Abholung größerer Mengen, Container und Privatkunden. Dies sind attributierte Betreiberangaben, keine neue Standort- oder Servicefeldfreigabe. Root wirbt mit Tagespreisen, FAQ mit Preis anfragen; in diesen vier gelesenen Seiten keine numerische Ankaufskursliste, Verkaufspreisliste oder bezifferten Abhol-/Container-/Annahmegebühren. Daraus keine Kostenfreiheit ableiten. Keine bewiesene gemeinsame Betreiberidentität oder Nachfolgekette mit Südpfalz Metall. Frontmatter unverändert; insbesondere vorhandenes status aktiv nicht neu geprüft oder freigegeben. Keine Adress-/Telefon-Fills und keine Standort-/GEO-Freigabe. Quellen: https://www.mr-braun.de/ ; https://www.mr-braun.de/impressum/ ; https://www.mr-braun.de/kontakt-schrottankauf-rohrbach/ ; https://www.mr-braun.de/schrottankauf-haeufige-fragen/ .]
```

## Abschlussgrenze

Die Ersatztexte sind Vorschläge für die verantwortliche lokale Integration; dieser Review ändert **keines der sieben Dossiers**. Bereits in HEAD vorhandene historische Texte und alle Frontmatterfelder sind nicht Gegenstand einer pauschalen Neufreigabe. Offene Identitäts-/Kontaktfelder benötigen vor späterer Übernahme weiterhin das README-Quellengate. Keine Gitmutation, keine Produktionsabfrage/-mutation, kein GEO-Lauf und kein Build/Test ausgeführt. Einziger geschriebener Pfad ist dieser Bericht.
