# Review B — begrenzte lokale Integration

Stand: 18.10.2026. Geprüft wurden **genau die sechs Dossieränderungen aus `shards.b`** in `/tmp/opencode/checkin-20261018-0230-review-scope.json`, jeweils vollständige Datei und vollständiger Arbeitsbaum-Diff gegen HEAD `8ac7c1d3b7a6d71cc964db74a691e892383bec8a`. Alle sechs SHA-256-Werte stimmen mit dem Scope überein. Keine Dossieränderung, Gitmutation, Produktion, GEO-Abfrage oder Builds durchgeführt; geschrieben wurde nur dieser Bericht.

## Maßstab und gemeinsame Befunde

- README §§ „Trader seed“, „Feedback triage“, „Operator rename/merge chains“: Verzeichnisse sind Leads; zwei unabhängige qualifizierte Quellen oder die eng definierte Betreiber-Ausnahme sind für positive Stammdaten nötig. Ein Impressum identifiziert zunächst nur den Websitebetreiber. Such-Nulltreffer und Abruffehler beweisen keine Nichtexistenz oder Schließung.
- Frontmatter ist in **allen sechs Dateien bytegleich** zu HEAD; alle bisherigen Timeline-Zeilen bleiben in ihrer bisherigen Reihenfolge erhalten (zusätzlich maschinell verglichen). Historische Fehler werden nicht wegredigiert. Keine neue Stammdaten-, Identitäts-, Nachfolge- oder Preisfreigabe ableitbar.
- Wichtig: Neue Timeline-Bullets sind laut README Buildinput für `notes`, nicht bloß unveröffentlichte Arbeitsnotizen. Unbelegte historische Abrufbehauptungen dürfen daher nicht als frisch verifizierte Betriebsdaten ausgegeben werden.
- Die neue Recherche ist auf den 12.10. datiert. Eigene Abrufe am 18.10. können diesen historischen Vorgang **nicht** nachträglich beweisen. Die Dispositionen betreffen die vertretbare lokale Übernahme vorhandener Notizen, keine Beglaubigung jeder damaligen Suche.
- Wiederkehrende Preisformulierungen („keine Liste“, „keine Angaben“) ausschließlich als **im ausgewerteten Material nicht belegt** verstehen, nicht als Aussage, der Betrieb veröffentliche überhaupt keine Preise. Diese Begrenzung bei späterer Weiterverwendung ausdrücklich mitführen.

## 1. Meringer — lokal freigeben, nur als offener Identitätsfall

Datei: `dossiers/by/by-moosburg-meringer.md`.

Der Diff ersetzt den Überblick-Platzhalter und ergänzt ausschließlich den 12.10.-Bullet. Die Recherche vom 04.10. mit Anton/Sudetenlandstraße gegenüber Johann Meringer & Sohn/Lausitzstraße bleibt vollständig erhalten. Die letzte einschlägige Historie (`b9d7ca6a`, 04.10.) stützt die Herkunft dieser Konfliktnotiz, nicht die Richtigkeit der Verzeichnisdaten.

Der neue Text übernimmt weder die Kandidatenadresse noch HRA 55433 als Registerbeleg und behauptet keine Zusammenführung. Eigener Direktabruf `https://www.meringer-schrott.de/` am 18.10.: **Transportfehler**, kein lesbarer Inhalt/Impressum. Das bestätigt nur die jetzige Abrufgrenze, nicht Domaininhaber, Schließung oder den historischen Abruf.

**Disposition: begrenzte lokale Integration freigegeben.** Keine zwingende sachliche Korrektur im Diff. Bei nächster Textbearbeitung „Verkauf: keine Preisliste“ zu „keine Preisliste im ausgewerteten Material belegt“ präzisieren. Personen, Anschriften und Registerzeichen bleiben ausdrücklich unaufgelöste Aggregator-Leads; kein Stammdaten-Fill.

## 2. K.S. Dienstleistungen K. Seelhoff — lokal freigeben, keine Betriebsbestätigung

Datei: `dossiers/hb/hb-bassum-k-s-dienstleistungen-k-seelhoff.md`.

Überblick neu, 12.10.-Bullet vorangestellt; bestehende Recherche 03./04.10. und Import-/01.10.-Notizen bleiben erhalten. Die dokumentierte Korrektur HB → NI (`3fa26a2b`, 03.10.) wird nicht zurückgedreht; hb-Slug/Pfad bleiben stabil. Die Hausadresse ist bereits in HEAD vorhanden und wird durch diesen Review **nicht** nachverifiziert oder freigegeben.

Der Diff nennt Telefonnummer/Adresse korrekt nur als Verzeichnis-Leads. Die Nominatim-Aussage ist aus der bestehenden Timeline übernommen; sie wurde wegen des GEO-Ausschlusses nicht erneut abgefragt. Ein Hauspunkt wäre ohnehin kein Unternehmensnachweis. Die Quellen des neuen Bullets sind Portal und Geocoding, keine Betreiber-/Registerquelle; die Formulierung „keinen neuen ... Beleg“ trägt daher nur eine begrenzte Recherche-Notiz.

**Disposition: begrenzte lokale Integration freigegeben.** Keine zwingende Korrektur; keinesfalls als aktuelle Bestätigung der bestehenden Frontmatter-Adresse behandeln. Falls eine erneute umfassende Betreiber-/Registerprüfung behauptet werden soll, müssen tatsächlich geprüfte Quellen und Abrufresultate ergänzt werden; die vorhandenen Links leisten das nicht.

## 3. Goldhaus Bruchköbel — nur nach kleiner Textkorrektur freigeben

Datei: `dossiers/he/he-bruchkobel-63486-goldhaus-bruchkobel-goldankauf.md`.

Der neue Überblick respektiert die Entfernung von Straße/PLZ am 05.10. (`d8a405cb`) und die bloße Leadqualität von H. Joseph, Telefon und Zeiten. Kein Wieder-Fill. Die alten DNS-/Wayback-Angaben vom 02.10. bleiben unverändert dokumentiert, enthalten aber keine reproduzierbare Snapshot-URL oder DNS-Rohbelege.

Eigener Direktabruf `https://goldhaus-bruchkoebel.de` am 18.10.: **Transportfehler**. Daraus folgt weder NXDOMAIN noch eine historische Betreiberzuordnung. Insbesondere ist die neue Wendung **„Die früher genutzte Domain“** zu stark: HEAD dokumentiert eine untersuchte Domain, nicht ihre nachgewiesene frühere Nutzung durch dieses Goldhaus.

**Disposition: bedingt freigeben; vorher neue Überblick-Aussage korrigieren.** Ersatz: „Die am 02.10.2026 untersuchte Kandidatendomain `goldhaus-bruchkoebel.de` wurde damals laut Timeline mit NXDOMAIN protokolliert; der dort erwähnte Wayback-Treffer wurde als Redirect-Stub beschrieben. Eine Betreiberzuordnung ist damit nicht belegt.“ Keine historische Notiz löschen oder einen jetzigen Transportfehler als neue DNS-Messung ausgeben. Die ungenaue Leadreferenz „Gelbe-Seiten-Profil“ ist kein neuer Primärbeleg; eine konkrete URL wäre für spätere Nachprüfung hilfreich.

## 4. Loew Alfred — lokal als Recherche-Nullstand freigeben

Datei: `dossiers/he/he-frankfurt-60388-loew-alfred.md`.

Nur ein neuer Timeline-Bullet; der gesamte Überblick stammt bereits aus HEAD (`bbfe6676`, 07.10.) und ist keine Neuerung dieser Integration. Die ältere Vermutung „Klein-AV?“ wird ausdrücklich nicht zur Tatsache erhoben. Die Northdata-Such-URLs und 11880-Suchen vom 03.10. belegen allenfalls den damaligen Rechercheansatz, keinen Registerauszug und keine erschöpfende Firmenprüfung.

Der neue Bullet verweist nur auf frühere Suchquellen; ein eigenständiges Abrufprotokoll für 12.10. liegt im Diff nicht vor. Das ist eine Nachvollziehbarkeitsgrenze, kein Gegenbeweis zur Notiz. Eine ergänzende eigene Websuche „Alfred Löw Frankfurt Schrotthandel“ lieferte im verwendeten Suchwerkzeug keine Ergebnisse; auch dies ist kein belastbarer Nichtexistenznachweis und kein vollständiger Registerabgleich.

**Disposition: begrenzte lokale Integration freigegeben, ausdrücklich nicht als Identitäts-/Registerfreigabe.** Keine zwingende Korrektur. Bei stärkerer Weiterverwendung „nochmals ... geprüft“ durch konkrete Such-/Abrufbelege untersetzen oder vorsichtiger als fortbestehenden Dokumentationsstand formulieren. Keine Ableitung von Schließung, Branchenzugehörigkeit oder Nullpreisen.

## 5. Schrotthandel Münstermann — nur nach Identitätspräzisierung freigeben

Datei: `dossiers/he/he-gieen-schrotthandel-munstermann.md`.

Der neue Überblick und 12.10.-Bullet bewahren die seit 01./02.10. dokumentierte Trennung zwischen Münstermann-Lead und Michael Schulz. Eigener vollständiger Abruf von `https://schrotthandel-giessen.de/impressum/` am 18.10. erfolgreich: **Michael Schulz, Schrotthandel, Mittelmark 18, 35435 Wettenberg; 0641-8778567; im Seitenfuß 0172-1596177**. Das ist ein Primärbeleg für den dort genannten Websitebetreiber, nicht für die Identität oder Nichtexistenz Münstermanns. Kein Registerzeichen und keine belegte Betreiberkette gefunden/behauptet.

Der neue Satz **„keine Verbindung zu Münstermann oder Gießen belegt“** ist zu weit: Die Seite tritt selbst als „Schrotthandel Giessen“ auf. Es fehlt die Identitätsverbindung zum konkreten Münstermann-Dossier, nicht jeglicher Gießen-Bezug. Auch „gehört ... nicht Münstermann“ sollte keine ungeprüfte frühere Betreiber-/Nachfolgeverbindung ausschließen.

**Disposition: bedingt freigeben; vor Integration zwei Formulierungen präzisieren.** Im neuen Überblick: „Das gelesene Impressum nennt Michael Schulz in Wettenberg; eine Identitätsverbindung zum Münstermann-Lead ist nicht belegt.“ Im neuen Timeline-Bullet: „keine Identitätsverbindung zum als Münstermann geführten Gießener Seed-Eintrag belegt“. Fremde Telefonnummern weiterhin nicht übernehmen. Die vorhandene Münstermann-Verzeichnisnummer bleibt unbestätigter Lead, kein Telefon-Fill.

## 6. Harald Blum — lokal freigeben, Blumer bleibt unzugeordnet

Datei: `dossiers/he/he-heuchelheim-blum-harald-schrotthandel.md`.

Nur 12.10.-Bullet hinzugefügt. Die wichtige Korrektur vom 07.10. (Adresse/PLZ entfernt), die Nachprüfung vom 08.10. (`877ba545`) und der widersprechende ältere Ausnahme-Fill vom 02.10. bleiben erhalten und als Historie unterscheidbar. Marco Blum wird nicht als Nachfolger oder gemeinsamer Betrieb ausgegeben.

Eigener Abruf `https://schrotthandel-blumer.jimdosite.com/` am 18.10.: **HTTP 403**. Damit ist kein Betreiberinhalt gelesen. Der im neuen Bullet erwähnte NRW-Bezug ist nur als Beschreibung eines damaligen Suchtreffers dokumentiert; weder der jetzige Abruf noch der angegebene Link allein beweisen ihn. Entscheidend: Der Diff ordnet Blumer gerade **nicht** Harald Blum zu und übernimmt keine fremden Daten.

**Disposition: begrenzte lokale Integration freigegeben.** Keine zwingende Korrektur, solange NRW nur Suchhinweis und keine festgestellte Betreiberidentität bleibt. Für spätere fachliche Nutzung Suchtrefferbeleg ergänzen oder den NRW-Halbsatz weglassen; HTTP 403 niemals als Nichtexistenz- oder Schließungsbeleg verwenden.

## Eigene Abruf- und Aussagegrenzen / Ergebnis

Die vier oben protokollierten Direktabrufe erfolgten am 18.10.2026; nur Schulz' Impressum war lesbar. Ergänzende Websuchen zu Seelhoff/Bassum, Alfred Löw/Frankfurt, Meringer/Moosburg und Goldhaus Bruchköbel/Joseph ergaben im Suchwerkzeug jeweils keine Ergebnisse. Diese generischen Nullresultate werden **nicht** als unabhängige Gegenbelege oder gründlicher Registerabgleich gewertet. Kein Gewerberegister, kein Handelsregisterauszug, keine DNS-/Wayback-Neuprüfung, keine OSM-/GEO-Abfrage durchgeführt.

**Gesamtergebnis: vier begrenzte lokale Freigaben, zwei bedingte Freigaben (Goldhaus und Münstermann) mit den oben konkret benannten Korrekturen.** Keine pauschale Datenbestätigung, keine neue Händleridentität, keine aktuelle Adresse/Annahmebedingung/Preisliste verifiziert. Ohne die zwei Textkorrekturen keine uneingeschränkte Freigabe des gesamten Sechserpakets. Dossierkorrekturen sind absichtlich nur vorgeschlagen, nicht ausgeführt; Tests, Commit und Produktionsprüfung sind nicht Bestandteil dieses Reviews.
