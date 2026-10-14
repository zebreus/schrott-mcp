# Mudrow Hameln – Betreiber-/Identitätsreview, 14.10.2026

## Ergebnis und Beleggrenze

Für `dossiers/ni/ni-hameln-mudrow-schrotthandel-altmetall-kabel.md` wurde eine bisher nicht hinterlegte **Betreiberseite** gefunden: <https://www.mvm-hameln.de/>. Ihr Titel lautet „MVM-Hameln - Metallverwertung Mudrow“; das verlinkte Impressum nennt **Klaus und Margit Mudrow** als Inhaber, **Langes Feld 16, 31789 Hameln-Afferde**, Telefon **05151 9924004**, E-Mail **mudrow@web.de**. Die Startgrafik bewirbt ausdrücklich die vergütete Annahme von Schrott und Kabeln. Damit liegt erstmals ein inhaltlich passender Betreiber-Einzelbeleg statt bloßer Telefonbuch-Leads vor. [S1–S4]

Zusätzlich gefunden: ein unabhängig veröffentlichtes kommunales Planungsdokument im Ratsinformationssystem der Stadt Hameln, das „Metallverwertung Mudrow“ nennt und in der Abwägung die zulässige Metallverwertung an dieser Stelle bestätigt. **Historische Plausibilisierung, kein aktueller Gewerberegisterauszug.** [S5]

**README-Gate nicht vollständig erfüllt:** Impressum, Datenschutz und Grafik sind dieselbe Betreiberquelle, nicht drei unabhängige Belege. Ein HR-/Northdata-/Creditreform-Unternehmensdatensatz oder kommunaler Gewerberegisterauszug wurde nicht gelesen/gewonnen. Das kommunale Planungsdokument fällt nicht unter den in `README.md`, Z. 180–199, ausdrücklich genannten kommunalen Gewerberegisterbeleg. Auch die Ausnahme für eine allein ausreichende autoritative Betreiberseite greift nicht: keine Handelsregisternummer, kein geprüfter Register-Live-Status, keine belastbare Datierung der Geschäftsinformationen. Keine Vollverifikation oder pauschale Feldfreigabe behaupten.

Nur diese Forschungsnotiz wurde im Repository neu angelegt. Dossier, fremde Änderungen und Produktionsdaten wurden nicht verändert; keine Produktionszugriffe erforderlich.

## Tatsächlich gelesene Quellen

Alle Abrufe am 14.10.2026. Bei PDF und Grafik wurden die Inhalte selbst gelesen, nicht nur Suchtreffer.

| ID | Quelle / gelesener Umfang | Aussage und Grenze |
| --- | --- | --- |
| S1 | <https://www.mvm-hameln.de/> – HTML und Markdown | HTML-Titel und Metadaten „MVM-Hameln - Metallverwertung Mudrow“, eingebundene `mvm-seite.jpg`, Links auf Impressum und Datenschutz. Der eigentliche Betriebshinweis ist eine Grafik; reines Markdown enthält ihn nicht. Alte HTML-Metadaten mit `2001-02-28` und sogar „Westernreiten“ sind kein belastbares Aktualitätsdatum der Grafik. |
| S2 | <https://www.mvm-hameln.de/Impressum.htm> – vollständiger Text | Inhaber und inhaltliche Verantwortung: Klaus und Margit Mudrow; Langes Feld 16, 31789 Hameln-Afferde; 05151 9924004; mudrow@web.de. Keine HRA/HRB-Angabe. Die dort als „USt-IdNr“ bezeichnete Zahl `22 13008056` nicht als geprüfte USt-ID oder Handelsregisterkennung übernehmen. |
| S3 | <https://www.mvm-hameln.de/Datenschutz.htm> – vollständiger Text | Wiederholt Klaus und Margit Mudrow, Anschrift mit 31789, Telefon und E-Mail als verantwortliche Stelle. Konsistenzprüfung innerhalb derselben Website, kein zweiter unabhängiger Beleg. |
| S4 | <https://www.mvm-hameln.de/mvm-seite.jpg> – vollständig visuell gelesen | „MVM - Metallverwertung Mudrow“, Langes Feld 16, **31785** Hameln; identische Telefonnummer und Mail. Materialannahme, Ausschlüsse und Öffnungszeiten siehe unten. Undatierte Aussage „Ab sofort gelten unsere neuen Öffnungszeiten“. |
| S5 | <https://ris.hameln.de/ris/hameln/file/getfile/69873> – PDF heruntergeladen, mit `pdftotext -layout` erschlossen; Dokumentkopf und relevante Stelle S. 69–70 gelesen | Abwägungs-/Beschlussvorschläge eines Bauleitplanungsverfahrens; Text nennt Bebauungsplan Nr. 605 und „Südlich Langes Feld“, Beteiligungen 2017–2020, darunter Stellungnahmen bis September 2020. Auf **PDF-S. 70, zugleich gedruckter S. 70**, Einwand vom 12.11.2019: „Die Metallverwertung Mudrow liegt innerhalb von 300 Metern zum künftigen Mischgebiet.“ In der rechten Abwägungsspalte: „Die Metallverwertung, ist keine BImSch-Anlage nach der 4 BImSchV […] Die Metallverwertung wird zulässigerweise an dieser Stelle betrieben.“ Enthält an dieser Stelle keine Hausnummer, Inhabernamen oder aktuellen Registerstatus. Veröffentlichungs-/Beschlussdatum nicht separat verifiziert. |
| S6 | <https://www.northdata.de/?id=Mudrow%2C%20Hameln> und <https://www.northdata.de/?id=Klaus+Mudrow> – zurückgelieferte Seiten gelesen | Beide lieferten die allgemeine Northdata-Such-/Startseite, keinen zuordenbaren Unternehmens- oder Personendatensatz. **Keine Registerprüfung mit Negativergebnis**, kein Nachweis einer Nichteintragung. |

Technische Lesekopien von S4/S5 liegen außerhalb des Repositorys in `/tmp/opencode/mudrow-review-20261014/`; sie sind keine zusätzlichen Quellen. `webfetch` konnte S5 wegen PDF-Inhaltstyp nicht darstellen; der anschließende HTTP-Download und die Textextraktion waren erfolgreich.

## Betreiberangaben: Kandidaten für spätere Integration, nicht jetzt freigegeben

| Sachverhalt | Gelesene Angabe | Bewertung |
| --- | --- | --- |
| Betriebs-/Anzeigename | MVM – Metallverwertung Mudrow | S1/S4; passt zu Dossiername und Schrott/Kabel, aber kein Register-Firmenname nachgewiesen. |
| Websitebetreiber / Inhaber | Klaus und Margit Mudrow | S2; S3 wiederholt. Rechtsform nicht ableiten, insbesondere weder GbR noch Einzelunternehmen erfinden. |
| Straße / Ort | Langes Feld 16, Hameln-Afferde | S2/S3/S4 konsistent. S5 stützt historisch die Metallverwertung im Planungsumfeld, nicht die exakte Hausnummer. |
| PLZ | Impressum/Datenschutz **31789**, Startgrafik **31785** | Echter Widerspruch innerhalb der Betreiberquelle. Vermutlich Grafikfehler ist nur eine Hypothese. Nicht stillschweigend glätten; amtliche Adress-/Gewerbeauskunft oder Betreiberklärung benötigt. |
| Telefon | 05151 9924004 | S2/S3/S4 konsistent, keine zweite unabhängige aktuelle Bestätigung. |
| E-Mail | mudrow@web.de | S2/S3/S4 konsistent, gleiche Beleggrenze. |
| Website-Erreichbarkeit | HTTPS erfolgreich abgerufen | Technisch erreichbar am Prüftag; nicht gleichbedeutend mit geprüft aktivem Gewerbe. |
| Öffnungszeiten | Mo–Mi 08:00–11:30 und 13:00–15:00; Do 08:00–11:30; Fr/Sa geschlossen | Visuell S4. Kein Kalenderdatum; keine Sonntag-Angabe und keine Sondertage ableiten. Vor Übernahme Aktualität klären. |

### Ankauf / Annahme laut Betreiber-Grafik [S4]

**„Annahme mit Vergütung“:** Schrott, Handelsguss, Kupfer, Kabel, Messing, Zink, Blei, Aluminium und Bleiakkus. Dies ist ein ausdrücklicher Ankaufhinweis, nicht bloß Entsorgungswerbung. **Keine numerischen Ankaufspreise** genannt; keine Preisquelle oder „Tageshöchstpreise“ erfinden.

**„Annahme ohne Vergütung“:** Gasflaschen, Elektroschrott, Elektrogeräte, Platinen. Diese nicht als vergütete Ankaufsmaterialien kennzeichnen. Keine Detailbedingungen zur Gasflaschenannahme angegeben.

**Ausschlüsse:** „Kein Materialverkauf“; keine Annahme von Feuerlöschern, Haushaltsmüll, Kühlschränken oder Kühlaggregaten. Keine belegten Aussagen zu Abholung/Containerdienst, Privat-/Gewerbekunden, Mindestmengen, Ausweispflichten, Zertifizierungen oder GEO.

## Identität und Verwechslungsprüfung

1. **Beide Seiten geprüft:** Nicht nur Impressum gelesen, sondern die Betreiber-Grafik mit expliziter Metall-/Schrott-/Kabelannahme und die kommunale Nennung der Metallverwertung in Hameln. Der gefundene Websitebetreiber ist also nicht lediglich ein namensgleicher fachfremder Anbieter. [S1–S5]
2. **Dossier-Zuordnung bleibt ohne eindeutigen Seed-Identifikator:** Das alte Dossier hat keine Anschrift/Telefon/URL. Name, Ort und Kabel-/Schrottangebot passen sehr gut zur gefundenen MVM. Mangels alter eindeutiger Kennung ist die Zuordnung eine stark gestützte Arbeitshypothese, keine registergestützte Identitätskette.
3. **Aggregatorvarianten sind Leads derselben möglichen Entität:** Erste Suchtreffer auf Gelbe Seiten, GoLocal, Das Örtliche, Stadtbranchenbuch und Schrottplatz-Info nennen „Mudrow Schrotthandel Verwertung, Altmetall, Kabel, Schrott“ bzw. „Schrotthandel Mudrow“ an Langes Feld 16 und teilweise 05151 9924004. Diese wurden nur als Suchresultate gesehen, nicht als Belegseiten ausgewertet, und zählen ausdrücklich nicht zum Nachweis. Übereinstimmung vieler Telefonbuchkopien schafft keinen unabhängigen Doppelbeleg.
4. **Kommunale Gegenprobe auf andere Betriebe:** S5 unterscheidet die Metallverwertung von Enertec und nennt unmittelbar danach gesondert „Autoverwertung Zeaiter“. Dessen BImSchV-Genehmigung darf nicht Mudrow zugerechnet werden. Auch keine Autoverwertung für Mudrow aus dieser Nachbarschaft ableiten.
5. **Namensgleiche Kandidaten:** Suchläufe zu Mudrow/Schrotthandel ohne Hameln, zu Klaus/Margit Mudrow, Register und Betreiber-Social-Profilen brachten keinen zweiten belastbar zuordenbaren Metallhändler. Ein Northdata-Suchlauf warf u. a. „DoMaFloor GmbH, Gehrden, HRB 216789“ als Treffer aus; diese Seite wurde nicht gelesen, es gab keine sichtbare Schrotthandel-/Langes-Feld-Verbindung. Deshalb ausdrücklich **kein** Mudrow-Registerbeleg, keine HRB-Übernahme. Aus dem fehlenden zweiten Treffer folgt nicht, dass Namensvettern ausgeschlossen sind.

## Offene Fragen und Owner-Empfehlung

- **Aktueller rechtlicher Betreiber / Gewerbestatus:** Gewerberegisterauskunft für Klaus und/oder Margit Mudrow, Tätigkeit Metallverwertung/Schrotthandel, Langes Feld 16 in Hameln einholen; prüfen, ob beide weiterhin Inhaber sind und unter welcher Rechtsform. Hier keine behördliche Anfrage gestellt, keine personenbezogene Registerauskunft bestellt.
- **PLZ:** 31789 im Impressum/Datenschutz versus 31785 in der Geschäftsgrafik amtlich oder beim Betreiber klären. Bis dahin beide Aussagen mitsamt Quellen erhalten.
- **Aktualität der Grafik/Zeiten:** „Ab sofort“ ohne Datum genügt nicht für ein gesichertes Änderungsdatum. Die abgerufene Grafik dokumentieren, aber aktuelle Zeiten erst nach Bestätigung als belastbar markieren.
- **Zweite README-zulässige Quelle:** Aktueller Gewerberegisterauszug, konkret passender HR-/Creditreform-/Northdata-Datensatz oder authentifiziertes betreibereigenes Social-Profil fehlt. Die Suchen nach Register/Unternehmensregister/Creditreform/Firmenwissen sowie Facebook/Instagram lieferten keinen gelesenen passenden Datensatz. Das ist eine Recherchegrenze, keine Feststellung „nicht eingetragen“ oder „geschlossen“.
- **Status:** Der bisherige `aktiv`-Wert ist durch diese Runde nicht vollverifiziert. Owner-seitig weiter als **Klärfall / Einzelbeleg mit historischer kommunaler Stütze** behandeln; bei einer späteren Dossierbearbeitung `pruefung` erwägen, solange das README-Gate offen bleibt. Kein Schließungsbeleg gefunden.
- **Nächste Timeline-Aussage:** Fund der Betreiberseite und vergüteten Schrott-/Kabelannahme festhalten, kommunale historische Stütze separat zitieren, PLZ-Widerspruch und fehlenden aktuellen Zweitbeleg ausdrücklich nennen. Die alte Recherche vom 01.10.2026 nicht löschen; sie dokumentiert den damaligen Quellenstand.

Kein Seed-Test oder Build erforderlich: Es wurden weder Dossier noch Code geändert. Aus dieser Notiz folgt keine Produktionsveröffentlichung.
