# Review B – begrenzte lokale Textintegration (18.10.2026)

## Prüfgrundlage und Ergebnis

Geprüft wurden genau die sieben vollständigen Dossiers aus `shards.b` in `/tmp/opencode/checkin-20261018-0330-review-scope.json`, jeweils einschließlich vollständigem Diff gegen `b052b2ba248bbeb8511023c52af6a12cae0c6643`. HEAD entspricht diesem Commit; alle sieben SHA-256 stimmen mit dem Scope überein. Sämtliche Diffs betreffen ausschließlich Überblick/Timeline; Frontmatter und vorhandene Timeline-Einträge sind unverändert erhalten. Keine Slugänderung, Löschung, Statushochsetzung oder Kontaktübertragung im geprüften Diff.

**Disposition: 7 × nur nach den unten genannten Textkorrekturen lokal integrierbar; keine unveränderte Freigabe der historischen Recherchebehauptungen.** Die Identitätsgrenzen sind grundsätzlich richtig. Eine Quellen-URL allein belegt aber weder den behaupteten Abruf am 12.10. noch eine damalige negative Suche. Ein tragfähiges historisches Abruf-/Such-/Produktionsprüfprotokoll wurde in dieser Prüfung nicht vorgelegt; die gezielte Suche in `research/` nach `ZASS00300004`, `Böhm Wertstoffzentrum`, `6290`, `Eugen Elias` und `Stiefel Norbert` ergab keine Treffer. Das behauptet nicht die Nichtexistenz sämtlicher Protokolle außerhalb dieses Suchbereichs.

Maßstab: README 168–202 (zweiseitige Identitätsprüfung, Aggregatoren nur Leads, zwei unabhängige zulässige Quellen bzw. vollständig erfüllte autoritative Betreiber-Ausnahme), 220–240 (Betreiberketten, Historienerhalt, Overwrite-Gate). Die unveränderten Altadressen, Telefonnummern, Beschreibungen und Klassifikationen werden **nicht** neu bestätigt. Alle sieben Datensätze bleiben `pruefung`. Insbesondere sind `trader_type: autoverwertung` bei Kaiser und die historischen Material-/Annahmeangaben bei Feix keine durch diesen Review freigegebenen Fakten.

Nur dieser Bericht wurde geschrieben. Keine Dossieränderungen, Gitmutation, Produktionsabfragen, GEO-Prüfungen oder Builds. Die Ersatztexte sind Vorschläge für den Owner, nicht bereits angewendete Änderungen. Neue und noch nicht integrierte Abschnitte vom 12.10. sollen durch korrekt datierte heutige Einträge ersetzt werden; bestehende HEAD-Historie bleibt unangetastet.

## Direkt geprüfte Primärquellen am 18.10.2026

- **P1:** http://www.schrotthandel-kaiser-trier.de/impressum.html – vollständig lesbar; Sascha Kaiser, Pfalzeler Straße 11, 54293 Trier, +49 1520 6623504, info@schrotthandel-kaiser-trier.de, USt-ID DE 242 6429 64. Footer `(c) 2014`. Das ist kein Veröffentlichungs-/Gültigkeitsdatum der einzelnen Aussagen und kein heutiger Registerabgleich.
- **P2:** http://www.schrotthandel-kaiser-trier.de/leistungen.html – vollständig lesbar; Leistungen für Firmen/Privatpersonen, Metall-/Schrottentsorgung, Fahrzeugverschrottung, Demontagen; wörtlich: „Für unsere festen Firmenkunden stellen wir kostenfreie Container zur Verfügung.“ Keine numerische Preisliste im gelesenen Text. Keine allgemeine Gratisabholung ableitbar.
- **P3:** https://fachbetrieberegister.gadsys.de/fbrwiki/gesa-info – direkter Versuch lieferte 503. Über https://www.altfahrzeugstelle.de/ wurde anschließend die GESA-Onlinehilfe vollständig lesbar geliefert, einschließlich Änderungsvermerk `2026/06/24 09:03`. Wörtlich: Daten zu „allen anerkannten Demontagebetrieben und Schredderanlagen“, hingegen nur einzelnen freiwillig gemeldeten Annahme-/Rücknahmestellen. Allgemeine Registerabdeckung, **kein** Osman-Einzelfallbeleg. Kein eigener Abruf der dynamischen Osman-Einzelakte durchgeführt.
- **P4:** https://www.veolia.de/standorte/gera-hainstr – Markdown und HTML direkt gelesen. Standortkörper: Hainstraße 17, 07545 Gera, Veolia Umweltservice Ost GmbH, +49 (0)365 82487 150, Mo–Fr 9–17, Sa 9–14; Schrott/Metalle und Elektroschrott. HTML-Titel: „Recyclinghof Gera, Hainstraße (ehemals GUD)“. Metabeschreibung bewirbt kostenlose Annahme verschiedener Abfälle einschließlich Restschrott; das ist eine Veolia-Aussage, kein ERV-Tarif und keine pauschale Kostenfreiheitsgarantie. Keine ERV-Rechtsnachfolgekette aus dieser Seite bewiesen.
- **P5:** https://stiefel-abbruch.de/ und https://stiefel-abbruch.de/impressum/ – direkt gelesen. Impressum: Stiefel Abbruch GmbH, Salweidenbecke 18, 44894 Bochum, AG Bochum HRB 11890, Geschäftsführer Thomas/Jann/Kay Stiefel. Das identifiziert den Bochumer Websiteinhaber, nicht den Sachsenbrunner Seed; keine bewiesene Verbindung und auch kein absoluter Beweis, dass eine Verbindung ausgeschlossen ist.

## 1. Sascha Kaiser / Trier-Pfalzel

**Datei:** `dossiers/rp/rp-trier-pfalzel-sascha-kaiser-schrott-metallhandel.md`

**Disposition: nach Datierungs- und Reichweitenkorrektur integrierbar.** P1/P2 bestätigen heute den wesentlichen Inhalt des neuen Überblicks. Kein Registerzweitbeleg; die autoritative Ausnahme ist nicht nachgewiesen. Trierer Websiteangaben dürfen nicht als unabhängig bestätigte Dossieridentität oder heutige Tätigkeit ausgegeben werden. Namensvetter-Trennung im Altbestand bleibt historische Aussage, keine hier erneut bewiesene Trennung. Historie vollständig erhalten.

**Nötige Ersatztexte:** Im neuen Überblick den ersten Absatz ersetzen durch:

> Die am 18.10.2026 gelesenen Website-Seiten nennen Sascha Kaiser, Schrott- und Metallhandel, Pfalzeler Straße 11, 54293 Trier, 01520 6623504 und info@schrotthandel-kaiser-trier.de sowie Entsorgungs- und Demontageleistungen für Privatpersonen/Firmen. Der Footer lautet © 2014; daraus folgt kein gesicherter Aktualitätsstand der Angaben. Ein unabhängiger aktueller Identitäts-/Registerabgleich liegt dieser Prüfung nicht vor. Leere Adress-/Kontaktfelder und `status: pruefung` bleiben unverändert; die Websiteangaben sind ein Einzelbeleg.

Die neuen Preisbullets ersetzen durch:

> - **Ankauf/Verkauf:** In den am 18.10.2026 gelesenen Impressums-/Leistungsseiten keine numerische Ankauf- oder Verkaufspreisliste dokumentiert; keine Aussage über sämtliche anderen Veröffentlichungen.
> - **Gebühren:** Die Leistungsseite bewirbt kostenfreie Container für feste Firmenkunden. Umfang, Neben-/Transportkosten und heutige Gültigkeit sind nicht unabhängig bestätigt; keine allgemeine Gratisabholung ableiten.

Den neuen Abschnitt `### Recherche 12.10.2026` einschließlich Bullet ersetzen durch:

> ### Quellenprüfung 18.10.2026
>
> - [Recherche 18.10.2026: Impressum und Leistungsseite heute direkt gelesen; dort Sascha Kaiser, Pfalzeler Straße 11, 54293 Trier, 01520 6623504, info@schrotthandel-kaiser-trier.de sowie Entsorgungs-/Demontageleistungen für Privatpersonen/Firmen. Footer © 2014 ist kein gesichertes Aktualitätsdatum. Kostenfreie Container werden nur für feste Firmenkunden beworben; keine allgemeine Gratisabholung oder numerische Ankauf-/Verkaufspreisliste aus diesen Seiten abgeleitet. Einzelbeleg, keine Status-/Kontakt-/Adressfreigabe. Ein Abruf vom 12.10.2026 wird mangels vorgelegtem Protokoll nicht bestätigt. Quellen: http://www.schrotthandel-kaiser-trier.de/impressum.html ; http://www.schrotthandel-kaiser-trier.de/leistungen.html.]

## 2. Elias Eugen / Owschlag

**Datei:** `dossiers/sh/sh-owschlag-elias-eugen.md`

**Disposition: nur als heutige Aktenprüfung integrierbar.** Der neue 11880-Link ist höchstens ein weiterer Aggregator-Lead, keine unabhängige Bestätigung des alten Telefons. Keine Betreiberidentität und keine Zuordnung zu MW Containerdienst bewiesen. Die konkrete historische Namenssuche und Portalwiedergabe wurden nicht erneut verifiziert; keinen Primärbeleg durch Portalzählung vortäuschen. Vollständige Althistorie und Frontmatter unverändert.

**Nötiger Ersatz:** Den gesamten neuen Abschnitt vom 12.10. ersetzen durch:

> ### Aktenprüfung 18.10.2026
>
> - [Recherche 18.10.2026: Der vorliegende Rechercheentwurf nennt als weiteren, hier nicht erneut inhaltlich verifizierten Verzeichnis-Lead https://www.11880.com/branchenbuch/owschlag/060440092B102285883/eugen-elias.html. Weder die behauptete Namenssuche noch ein Abruf am 12.10.2026 sind durch ein vorgelegtes Protokoll bestätigt. Portalangaben wären auch bei Übereinstimmung kein Betreiber-/Registerbeleg und keine unabhängige Bestätigung der vorhandenen Telefonnummer. Betreiberidentität, heutige Anschrift/Tätigkeit, Annahme-/Abholbedingungen sowie Ankauf-/Verkaufspreise und Gebühren bleiben in der Aktenlage ungeklärt. Keine Gleichsetzung mit MW Containerdienst, keine Frontmatter-Änderung, keine Schließungsaussage.]

## 3. Feix / Homburg-Beeden–Bexbach

**Datei:** `dossiers/sl/sl-homburg-beeden-bexbach-feix-charli-siegfried-wilhelm.md`

**Disposition: nur mit reduziertem Aktenprüftext integrierbar.** Richtige Grenze: Wilhelm/Charli/Siegfried/Ludwig und Bexbacher Buntmetallverwertung nicht zusammenführen. Die Differenz von Altadressen ist ein Warnsignal, kein beweiskräftiger „Gegenbeleg“ für rechtliche Personenverschiedenheit. Portalangaben und Vereinsheft sind weder zwei zulässige Identitätsquellen noch Register-/Betreiberbeleg. Die Behauptung einer damaligen Produktionsdatenbankprüfung darf ohne Protokoll nicht stehen bleiben. Der lokale separate Ludwig-Datensatz ist anhand `dossiers/sl/sl-bexbach-oberbexbach-ludwig-feix-sohn-prazisiert-nr-33.md` feststellbar, aber nicht dadurch sachlich verifiziert. Bestehende Aggregator-Ausnahme von 02.10. bleibt als durch 08.10. relativierte Historie erhalten; keine Altfeldfreigabe.

**Nötiger Ersatz:** Den gesamten neuen Abschnitt `Feedback-Triage 12.10.2026` ersetzen durch:

> ### Aktenprüfung 18.10.2026 (Feedback-Referenzen 6290/6291)
>
> - [Recherche 18.10.2026: Der vorliegende Entwurf ordnet Feedback 6290 Wilhelm und 6291 Charli Feix zu; Feedbackinhalte und eine behauptete Produktionsprüfung vom 12.10.2026 wurden in diesem Review nicht unabhängig geprüft. Die angeführten Verzeichnis-/Vereinsheft-Leads sind keine hinreichenden Betreiber-/Registerbelege. Abweichende Namen, Anschriften und Telefonnummern sind Konflikthinweise, kein gesicherter Beweis getrennter oder identischer Rechtsträger. Im lokalen Dossierbestand existiert zusätzlich `sl-bexbach-oberbexbach-ludwig-feix-sohn-prazisiert-nr-33`; daraus folgt keine nachgewiesene Betreiberkette. Keine Neuaufnahme, Zusammenführung oder Übertragung von Angaben zwischen Wilhelm, Charli, Siegfried und den Bexbacher Datensätzen. Vorhandene Adresse, Telefon und Materialbeschreibung bleiben ausdrücklich unbestätigter Altbestand; `status: pruefung` unverändert. Offen: unabhängige Betreiber-/Register-/Kommunalbelege für Personen und Standorte.]

Die sechs bereits im Entwurf genannten URLs können als `Unverifizierte Recherche-Leads des Entwurfs` angehängt werden, dürfen aber nicht als am 12.10. oder heute vollständig gelesene Belege bezeichnet werden. Für die begrenzte Integration ist ihre Übernahme nicht erforderlich.

## 4. Autoservice Osman Mamdouh / Chemnitz

**Datei:** `dossiers/sn/sn-chemnitz-09120-autoservice-osman-mamdouh.md`

**Disposition: nach Ersatz der historischen Zugriffserzählung integrierbar.** Die allgemeine GESA-Abdeckungsgrenze ist heute über P3 belegt. Sie beweist weder einen Chemnitzer Datensatz noch die Gültigkeit von ZASS00300004. Vorsicht: Die freiwillige/unvollständige Abdeckung betrifft Annahme-/Rücknahmestellen, nicht pauschal Demontagebetriebe. Der Entwurf beachtet diese Grenze weitgehend. Nicht bestätigen lässt sich der behauptete fehlgeschlagene Einzelaktenabruf am 12.10. Die bereits korrigierte Fehlklassifikationshistorie vom 01.10. bleibt erhalten; keinerlei Transfer aus Böhlen. Bestehende Seed-Adresse und `trader_type` nicht freigegeben.

**Nötiger Ersatz:** Den neuen Abschnitt vom 12.10. ersetzen durch:

> ### Quellen- und Aktenprüfung 18.10.2026
>
> - [Recherche 18.10.2026: Die über https://www.altfahrzeugstelle.de/ heute gelesene GESA-Onlinehilfe (Änderungsvermerk 24.06.2026) beschreibt Daten zu allen anerkannten Demontagebetrieben/Schredderanlagen, aber nur einzelnen freiwillig gemeldeten Annahme-/Rücknahmestellen. Diese allgemeine Abdeckungsgrenze ist kein Osman-Einzelfallbeleg. Der direkte Hilfe-Link https://fachbetrieberegister.gadsys.de/fbrwiki/gesa-info lieferte zunächst 503; eine heutige amtliche Osman-Einzelakte wurde nicht geprüft. Der im Entwurf behauptete Einzelaktenabruf am 12.10.2026 ist ohne vorgelegtes Protokoll nicht bestätigt. Der bisher dokumentierte Drittanbietermirror und ZASS00300004 bleiben Prüfansätze, kein amtlicher Ersatz. Keine nachgewiesene Betreiberkette nach Böhlen, kein Status-/Kontakt-/Adress-Fill; für Chemnitz in der Aktenlage keine verifizierten Ankauf-, Teileverkaufs- oder Gebührenzahlen.]

Den GEO-Satz des Entwurfs nicht als Geokodierungsauftrag oder Standortfreigabe übernehmen; diese Prüfung führt keine GEO-Arbeiten durch.

## 5. Böhm Wertstoffzentrum / Lengenfeld

**Datei:** `dossiers/sn/sn-lengenfeld-bohm-wertstoffzentrum.md`

**Disposition: nur als heutige Aktenprüfung integrierbar.** Eine Bing-Abfrage-URL ist kein Suchprotokoll: Ergebnisinhalt, Zeitpunkt und Prüfumfang fehlen. Die Aussage „Treffer waren sachfremd“ und die behauptete exakte Suche am 12.10. sind nicht bestätigt. Die vorsichtige Behandlung als Seed-Kandidat ist richtig; keine Nichtexistenz oder Betriebsschließung ableiten. Historie/Frontmatter unverändert.

**Nötiger Ersatz für den gesamten neuen Überblick:**

> „Böhm Wertstoffzentrum“ ist im vorliegenden Dossier ein unbestätigter Seed-Kandidat in Lengenfeld mit den Stichworten Wertstoffhof/Container. Die Aktenprüfung vom 18.10.2026 liefert daraus keinen belastbaren Betreiber-, Register- oder Kommunalbeleg für Identität, genaue Anschrift, öffentliche Annahme oder aktuelle Tätigkeit. Das ist keine erneute Websuche und kein Beweis für Nichtexistenz. `status: pruefung` und leere Felder bleiben unverändert. Ankauf, Verkauf, Gebühren und Annahmebedingungen sind in der Aktenlage nicht verifiziert; keine Standortfreigabe.

**Nötiger Ersatz für den neuen Timeline-Abschnitt:**

> ### Aktenprüfung 18.10.2026
>
> - [Recherche 18.10.2026: Vollständiges Dossier und HEAD-Diff geprüft. Die im Rechercheentwurf genannte Bing-Abfrage-URL dokumentiert ohne Ergebnisprotokoll keine tatsächlich ausgeführte Suche vom 12.10.2026 und keinen belastbaren Negativbefund. Seed-Stichworte Wertstoffhof/Container und Lengenfeld bleiben unbestätigt. Offen: vollständige Firmierung, Betreiberbeleg, Hausanschrift, Kundenannahme sowie Ankauf/Verkauf und Gebühren. Keine Fills und keine Schlussfolgerung auf Nichtexistenz oder Schließung.]

## 6. ERV / Gera

**Datei:** `dossiers/th/th-gera-07545-erv-elektronik-recycling-und-verwertung.md`

**Disposition: nach Neudatierung und enger Preisformulierung integrierbar.** P4 bestätigt heute den Veolia-Standorttext einschließlich „ehemals GUD“ im HTML-Titel. Keine Identität/Rechtsnachfolge zur ERV belegt; der Entwurf vermeidet korrekt Kontakttransfer. Der behauptete historische Abruf ist dennoch nicht bestätigt. Die bereits in HEAD enthaltene Ortsnormalisierung ist kein neuer Frontmatter-Diff. Adresse/Telefon der ERV bleiben aggregatorbasierter Altbestand. Veolia-Metatext enthält Kostenfreiheitswerbung; deshalb nicht pauschal behaupten, es gebe keinerlei Gebühren-/Kostenangaben auf der Seite, und diese nicht ERV zuschreiben.

**Nötiger Ersatz:** Den neuen Abschnitt vom 12.10. ersetzen durch:

> ### Quellenprüfung 18.10.2026 (Veolia, keine ERV-Betreiberkette)
>
> - [Recherche 18.10.2026: https://www.veolia.de/standorte/gera-hainstr heute direkt als Markdown und HTML gelesen. Standorttext nennt Veolia Umweltservice Ost GmbH, Hainstraße 17, 07545 Gera, 0365 82487 150, Mo–Fr 09:00–17:00, Sa 09:00–14:00 sowie Schrott/Metalle und Elektroschrott; HTML-Titel bezeichnet den Standort als Recyclinghof Gera, Hainstraße (ehemals GUD). Das belegt eine Veolia-Selbstauskunft zum Standort, nicht die rechtliche Identität oder Nachfolge der Seed-ERV. Die Metabeschreibung bewirbt kostenlose Annahme ausgewählter Abfälle; weder diese Aussage noch Kontakte/Öffnungszeiten werden auf ERV übertragen. Keine ERV zuordenbare Ankauf-, Verkaufs- oder Gebührenzahl durch diese Quelle bestätigt. Historischer Abruf am 12.10.2026 ohne vorgelegtes Protokoll nicht bestätigt; Frontmatter unverändert, Standortzuordnung des ERV-Dossiers offen.]

## 7. Norbert Stiefel / Sachsenbrunn

**Datei:** `dossiers/th/th-sachsenbrunn-stiefel-norbert-containerdienst-metall-u.md`

**Disposition: nach Aktenprüf-/Primärquellentrennung integrierbar.** P5 bestätigt einen Bochumer Websitebetreiber, nicht den Sachsenbrunner Seed. Der Hinweis auf widersprüchliche Verzeichnisadressen darf nur als vorhandene Aktenangabe, nicht als frisch bestätigter Widerspruch erscheinen. Die im Entwurf behauptete negative neue Suche vom 12.10. ist unbelegt; „Details und Quellenliste siehe 01.10.“ ist zu stark, weil dieser Alteintrag Portalnamen, aber keine konkreten Beleg-URLs enthält. Identitätsgrenze und Verzicht auf Fills sind richtig; weder Verbindung noch absolute Beziehungslosigkeit bewiesen. Althistorie unverändert.

**Nötiger Ersatz für den neuen Überblick:**

> Für den Sachsenbrunner Seed „Stiefel Norbert Containerdienst Metall- u. Schrott“ liegt in der geprüften Aktenlage kein belastbarer Identitäts-/Standortbeleg vor. Die vorhandene Timeline dokumentiert widersprüchliche Verzeichnis-Leads zu Schulstraße 8, Alte Schulstraße 9 und PLZ 98678/98673; diese wurden hier nicht neu bestätigt. Das am 18.10.2026 gelesene Impressum von stiefel-abbruch.de identifiziert dagegen Stiefel Abbruch GmbH in Bochum, AG Bochum HRB 11890. Eine Verbindung zum Sachsenbrunner Seed ist damit nicht belegt; keine Übertragung von Kontakten, Leistungen oder Preisen. Anschrift-/Kontaktfelder bleiben leer, `status: pruefung`; Annahmebedingungen und Ankauf-/Verkaufspreise bzw. Gebühren sind für den Seed ungeklärt.

**Nötiger Ersatz für den neuen Timeline-Abschnitt:**

> ### Quellen- und Aktenprüfung 18.10.2026
>
> - [Recherche 18.10.2026: Vollständiges Dossier und HEAD-Diff geprüft. Die frühere Timeline nennt widersprüchliche Verzeichnisadressen, enthält aber kein vollständiges URL-/Abrufprotokoll; ein erneuter Suchlauf oder Negativbefund vom 12.10.2026 wird nicht bestätigt. Heute direkt gelesenes Impressum von Stiefel Abbruch GmbH nennt Salweidenbecke 18, 44894 Bochum, AG Bochum HRB 11890 und Geschäftsführer Thomas, Jann und Kay Stiefel. Das identifiziert den Websiteinhaber, belegt aber keine Identität oder Betreiberkette zu Norbert Stiefel/Sachsenbrunn. Keine Datenübertragung, keine Fills, keine Schließungsaussage. Für den Sachsenbrunner Seed bleiben Betreiber, Anschrift, Annahmebedingungen und Preis-/Gebührenangaben ungeklärt. Quellen: https://stiefel-abbruch.de/ ; https://stiefel-abbruch.de/impressum/.]

## Owner-Grenze

Nach Übernahme der Ersatztexte ist ausschließlich die lokale textliche Dokumentation enger Beleg-/Aktenstände vertretbar. Das ersetzt weder das README-Gate für Altfeldkorrekturen noch Seed-Tests, Merge-/Deployentscheidung oder Produktionsverifikation. Falls später belastbare historische Protokolle vorgelegt werden, können die originalen 12.10.-Behauptungen separat dagegen geprüft werden; heutige Abrufe bestätigen niemals rückwirkend deren Durchführung.
