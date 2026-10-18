# Bremen: Review des uncommitteten Dossier-Diffs gegen HEAD

Review/Abrufe: **18.10.2026**. Dossier ausschließlich gelesen; keine Produktionsabfrage/-änderung, kein GEO und keine Dossieränderung. Owner integriert.

## Entscheidung

**Feldrücknahme freigeben, neuen Timeline-Text vor Integration korrigieren.**

| Feld | Exakte Entscheidung |
| --- | --- |
| `street` | `''` beibehalten statt HEAD `Auf den Roden 2` |
| `postcode` | `''` beibehalten statt HEAD `'28309'`; insbesondere nicht auf `'28307'` wechseln |
| `city`, `state` | `Bremen`, `HB` als Kandidatenkontext unverändert lassen, keine neue Standortverifikation behaupten |
| `status` | `pruefung` beibehalten; keine Schließung und keinen aktiven Betrieb bestätigen |
| Name, Slug | unverändert als historischer Kandidat; die `28309` im stabilen Slug ist kein aktueller Adressbeleg |
| Telefon, E-Mail, Website, Öffnungszeiten, Beschreibung, Dropoff/Pickup | leer lassen; keine Kontaktdaten oder Dienstleistungen von Namesakes übernehmen |

Maßgeblich README.md, insbesondere Quellenhierarchie und zweiseitige Prüfung (Zeilen 160–202): Verzeichnisse sind Leads, nicht Betriebsbelege. Der neue 11880-Direktabruf stärkt die Nachvollziehbarkeit des Kandidaten, **nicht** seine betriebliche Verifikation. Der Entzug einer unbelegten Adresse benötigt keinen positiven Nachweis einer anderen PLZ und ist keine Aussage, dass die Straße falsch oder der Betrieb geschlossen sei. Selbst eine unabhängig bestätigte postalische Zuordnung der Straße würde die fehlende Verknüpfung zum Betreiber nicht ersetzen.

## Tatsächlich am 18.10.2026 gelesene Quellen

### A. Adress-/Kandidatenseite — nur Leads

1. https://www.11880.com/suche/altmetallrecycling/bremen
   - Im aktuellen Volltext: „Schrottabholung Bremen Kostenlos Recycling“, **Auf den Roden 2, 28307 Bremen (Arbergen), (0176) 22544746**.
2. https://www.11880.com/branchenbuch/bremen/060440092B113127592/schrottabholung-bremen-kostenlos-recycling.html
   - Direktprofil selbst gelesen, gleiche Adresse/Nummer; „E-Mail-Adresse hinzufügen“, „Website hinzufügen“, keine Öffnungszeiten und keine natürliche/juristische Betreiberidentität genannt.
   - Zeigt „aktualisiert am 15.10.2026“ und „Eintragsdaten vom 17.07.2024“. Das sind **Portalangaben**, keine am 15.10. erfolgte eigene Verifikation und kein Lebenszeichen des Betriebs.
   - Verweist auf Cylex-Profil und übernimmt eine Cylex-Bewertung. Das beweist nicht die Herkunft sämtlicher Stammdaten, warnt aber zusätzlich davor, Portale als unabhängige Bestätigungen zu zählen. Rezensionen und Google-Verweise nicht als Betreiber-/Adressbeleg verwendet.
   - Gegenargument zur kompletten Verwerfung berücksichtigt: Der Kandidat ist aktuell als konkreter Adresse/Telefon-Lead nachvollziehbar, also nicht allein wegen fehlenden Impressums als erfunden deklarieren. Es reicht aber nicht für operative Felder oder verifizierte Existenz.
3. https://web2.cylex.de/suche/schrotth%C3%A4ndler/Bremen und https://web2.cylex.de/firma-home/schrottabholung-bremen-kostenlos-recycling-16433002.html
   - Beide heute **HTTP 403**, kein Inhalt gelesen. Die Angabe **28309 bei Cylex** ist in diesem Review ausschließlich historische Dossieraussage vom 05.10., **nicht heute erneut bestätigt**. Nicht formulieren: „Cylex nennt weiterhin 28309“.

### B. Namesake-/Betreiberseite — gelesene Betreiber-Selbstauskünfte, keine Identitätskette

4. https://bremerschrotthandel.com/impressum/
   - Direkt gelesen: **Kadir Sayli, Syker Straße 6, 28307 Bremen**, im Impressum angegebene Telefonnummer **+49 (0) 1522 6349848**, office@bremerschrotthandel.com.
   - Sichtbarer Footer hat einen abweichenden `tel:`-Link (+4917680820377) trotz ähnlich dargestellter Nummer. Daher präzise „Impressum nennt …“, nicht pauschal eine vollständig konsistente oder unabhängig verifizierte Kontaktlage behaupten.
   - In der gelesenen Impressumsseite keine Verknüpfung zu Auf den Roden 2 oder 0176 22544746 gefunden. Anderer Name/Adresse/Telefon sind Anlass zur Nichtzuordnung, **kein Beweis**, dass eine historische Beziehung ausgeschlossen ist.
5. https://www.bremer-recycling-service.de/ — HTML gelesen (Markdown lieferte keinen Inhalt).
   - HTML-JSON-LD nennt **Bremer Recycling Service / BRS**, Gründer **Adil Altun**, **Flughafenallee 26, 28199 Bremen**, **+4915737775391**, info@bremer-recycling-service.de. Die Seite verlinkt tatsächlich das nachfolgende Bundle.
6. https://www.bremer-recycling-service.de/static/js/main.3eadf2b3.js
   - Ausgelieferten JavaScript-Quelltext direkt gelesen und relevante Zeichenfolgen extrahiert: Konfiguration `name:"Bremer Recycling Service"`, `phone:"+49 1573 7775391"`; `impressum`-Objekt mit **Adil Altun, c/o HQ Bremen Airport, Flughafenallee 26, 28199 Bremen**, USt-ID DE450657364. Keine interaktive Impressumsansicht gerendert; Befund stammt aus Anwendungs-Code und ergänzend HTML-Strukturdaten.
   - Keine Zeichenfolge „Auf den Roden“ oder „22544746“ im gelesenen Bundle gefunden. Kein Beleg einer Umbenennung/Übernahme/Verbindung zum Dossierkandidaten.

Die beiden Betreiber-Websites sind je eigene Selbstauskünfte über **ihre** Betreiber, keine zwei unabhängigen Belege für den Kandidaten Auf den Roden 2. HR-/Northdata-/Gewerberegisterbeleg wurde nicht gefunden bzw. nicht gelesen. Die README-Ausnahme für allein ausreichende autoritative Betreiberseiten (u.a. Registerkongruenz) ist hier nicht nachgewiesen. Deshalb keine positive Neuzuordnung.

### Recherchegrenze

Eigenständige Suchversuche für exakte Adresse und Leadtelefonnummer: `websearch` scheiterte mit HTTP 401; Google-Direktabrufe lieferten nur einen Zugriffshinweis, Bing irrelevante Ergebnisse. Daraus folgt **kein belastbarer Negativbefund** zur Existenz weiterer Betreiber an der Adresse. Die zweiseitige Prüfung hat den konkreten Adresslead direkt sowie beide im Dossier genannten möglichen Verwechslungen selbst untersucht, aber die Identität bleibt offen. Keine amtliche PLZ-Zuordnung und keine Registeridentität verifiziert; kein Geocoding ausgeführt.

## Nötige Datierungs-/Textkorrekturen im Dossier (Owner)

1. Der Diff fügt `### Korrektur 12.10.2026` und eine auf 12.10. datierte Feldentfernung hinzu. Aus dem Diff lässt sich der tatsächliche Bearbeitungs-/Abrufzeitpunkt nicht beweisen. **Ohne gesonderten zeitgenössischen Nachweis auf 18.10.2026 datieren**, sowohl Überschrift als auch Klammermarker. Heutige Abrufe nicht als am 12.10. oder 05.10. durchgeführt ausgeben.
2. Den historischen Abschnitt `Recherche 05.10.2026` nicht mit heutigen Quellenbefunden umschreiben. Sein „bestehendes postcode: 28309 bleibt ungeklärt“ beschreibt den damaligen Stand; neuer Korrekturpunkt muss explizit sagen, dass der Wert jetzt zurückgenommen wird.
3. Im Überblick „Mobiler Kandidat“ zu **„Historischer Verzeichnis-Kandidat mit ungeklärtem Betreiber; die mobile Tätigkeit ist nicht verifiziert“** abschwächen. „PLZ widersprüchlich“ genauer als historisch dokumentierten Leadkonflikt kennzeichnen; 11880 28307 heute gelesen, Cylex 28309 heute nicht nachlesbar.
4. „keine Preisliste“ / „keine Tarife“ nicht als bewiesene Nichtexistenz schreiben, sondern **„keine diesem Kandidaten belastbar zuordenbare Preis-/Tarifquelle im Review verifiziert“**. Auch kostenlose Abholung oder Ankauf bleiben unbekannt; Namesake-Werbung ist kein Kandidatenbeleg.
5. Nicht nur die beiden Suchseiten als Quellen nennen: Direktprofil sowie selbst gelesene Betreiberseiten/Bundledaten für die Abgrenzung mit tatsächlichem Abrufdatum aufführen; Cylex explizit als heute blockierten historischen Lead markieren.

### Vorschlag für den neuen Korrekturpunkt

> [Korrektur 18.10.2026: `street` und `postcode` aus den operativen Feldern zurückgenommen, weil für den historischen Kandidaten keine zulässige Betreiber-/Adressverknüpfung verifiziert ist. Das heute gelesene 11880-Direktprofil nennt Auf den Roden 2, 28307 Bremen und 0176 22544746, bleibt aber ein Aggregator-Lead ohne benannten Betreiber oder Website. 28309 bei Cylex ist eine historische Dossierangabe vom 05.10.; Cylex-Suche und -Detailprofil waren heute mit HTTP 403 nicht nachlesbar. Der Leadkonflikt ist nicht postalisch aufgelöst. Das heute gelesene Impressum von bremerschrotthandel.com nennt Kadir Sayli, Syker Straße 6; HTML/Anwendungs-Code von bremer-recycling-service.de nennen Adil Altun, Flughafenallee 26. In diesen gelesenen Betreiberquellen wurde keine Verknüpfung zur Leadadresse/-nummer gefunden; weder Identität noch Übernahme oder Umzug sind belegt. Status bleibt `pruefung`, kein Kontakt-/Service-Fill und kein GEO. Ankauf/Vergütung, Verkaufspreise und Gebühren: keine diesem Kandidaten belastbar zuordenbare Preis-/Tarifquelle im Review verifiziert; kostenlose Abholung nicht bestätigt. Quellen: 11880-Direktprofil; bremerschrotthandel.com/impressum/; bremer-recycling-service.de/ und dort verlinktes main.3eadf2b3.js; Cylex nur historischer, heute blockierter Lead.]

Die im Quellenabschnitt ausgeschriebenen Direkt-URLs sollten im endgültigen Timeline-Punkt übernommen werden. Bericht belegt nur die oben beschriebenen aktuellen Abrufe, nicht die tatsächliche Durchführung historischer Abrufe. Keine Produktionsveröffentlichung behauptet.
