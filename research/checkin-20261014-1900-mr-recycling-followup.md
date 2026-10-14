# Feedback 7384 — M.R. Recycling / Michael Reinhardt: Originalbelege und Annahmeplatz

> **Nachtrag nach Owner-Hinweis, ebenfalls 14.10.2026:** Die anfängliche technische Efb-Suchgrenze ist überwunden. **Abschnitt 6 unten ersetzt die Aussagen zur nicht abgeschlossenen Registersuche und den browserabhängigen nächsten Schritt aus dem Erstbericht.** Tatsächlich ausgeführte Namens-/Hauptsitz-/Standortsuchen liefern keinen Michael-Reinhardt-/Wankstraßenbeleg. Eine Originalurkunde wurde nun gelesen, jedoch ausschließlich für die **andere MRA Recycling GmbH, HRB 23388, Firnhaberstraße 112**. Sie belegt nicht M.R. Recycling. Der Erstbericht darunter bleibt als nachvollziehbarer Stand des ersten Versuchs erhalten.

Recherche am **14.10.2026**; Dateiname gemäß Auftrag, **„1900“ ist keine behauptete Abrufzeit**. Ausgangspunkt: [Owner-Lead](feedback-20261014-mr-recycling-owner-lead.md). Maßstab: README, Abschnitt Feedback triage, insbesondere unabhängige Identität und Zwei-Quellen-Standard. Nur diese Datei geschrieben; keine Kontakte angeschrieben, keine Gewerbeauskunft bestellt, keine Dossiers oder Produktion geändert, kein Commit/Push/Deploy. Produktionsdaten wurden in diesem Follow-up überhaupt nicht geöffnet; der Bestandsabgleich im Ausgangsbericht wurde nicht als neuer eigener Check ausgegeben.

## Ergebnis für den Owner

**Aktualisierter Abschlussstand:** Der Nachtrag in Abschnitt 6 ersetzt die anfängliche technische Register-Suchgrenze: erfolgreiche Originalregisterabfragen einschließlich sämtlicher 79 lokalen und 112 MR-Ergebniszeilen ergaben keinen passenden Michael-Reinhardt-/Wankstraßenbeleg. Die gelesene MRA-Urkunde gehört ausdrücklich einem anderen Rechtsträger. Der unabhängige Identitäts-/Annahmeplatzbeleg fehlt weiterhin; Lead offen lassen. Frühere Abrufgrenzen unten sind Recherchehistorie, nicht der abschließende Registerstand.

**Lead weiter offen halten. Kein neuer qualifizierter unabhängiger Identitäts-/Standortbeleg und keine Original-Efb-Urkunde gewonnen.** Das ist ein Nichtnachweis mit konkreten Zugangsgrenzen, **keine Feststellung, dass der Betrieb nicht existiert oder nicht zertifiziert ist**.

Neu gegenüber dem Ausgangsbericht:

1. Den **aktuellen amtlich verlinkten Efb-Registerzugang** über Bayerisches Landesamt für Umwelt → GADSYS → `fachbetrieberegister.gadsys.de` gefunden und tatsächlich geöffnet. Alte ZKS-Hosts scheitern hier an DNS. Der aktuelle Zugang funktioniert, seine JavaScript-/Vaadin-Suche konnte jedoch nicht bis zu einer gültigen Betreiberabfrage ausgeführt werden. **Keine „0 Efb-Treffer“-Behauptung.**
2. Die **zuständige kommunale Gewerbeauskunft Augsburg** tatsächlich gelesen: Die Gewerbedatei ist ausdrücklich nicht öffentlich; Auskunft enthält grundsätzlich Name/Inhaber, betriebliche Anschrift und angezeigte Tätigkeit. Antrag ist gebührenpflichtig. Das benennt einen belastbaren nächsten Belegweg, ersetzt aber keinen Auszug für Michael Reinhardt.
3. **Wankstraße 7 ist auch in den direkt gelesenen M.R.-Metall-AGB ausdrücklich Annahmeplatz.** Der Konflikt liegt damit innerhalb beider Betreiberauftritte, nicht bloß zwischen zwei Domains: M.R.-Metall-Ankaufseite sagt 11b, M.R.-Metall-AGB sagt 7. Keine Mehrheitsentscheidung zugunsten 11b.
4. Die Zertifizierungswerbung ist erneut direkt nachgewiesen und zusätzlich in mehreren öffentlich lesbaren Betreiberartikeln vorhanden. Die begrenzte Suche in öffentlichen WordPress-Medien lieferte keine Urkunde; in den geprüften einschlägigen Seiten-/Artikelinhalten kein entsprechender PDF-/Zertifikatslink. **Werbung nicht in `certifications` übernehmen.**

## 1. Original-Efb: zuständige Quelle gefunden, Betreiberprüfung nicht abgeschlossen

### Amtliche Herkunft des Zugangs

- [LfU Bayern: Entsorgungsfachbetriebe in Bayern](https://www.lfu.bayern.de/abfall/entsorgungsfachbetriebe/index.htm), direkt HTTPS/HTTP 200 gelesen: Nach § 28 EfbV müssen Zertifizierungsorganisationen Zertifikate elektronisch übermitteln. Diese Daten bilden die Grundlage des öffentlich zugänglichen bundesweiten Registers. Die Seite verlinkt auf GADSYS.
- [GADSYS: Fachbetrieberegister – Entsorgungsfachbetriebe](https://www.gadsys.de/entsorgungsfachbetriebeverordnung-eefbv/fachbetrieberegister-entsorgungsfachbetriebe), direkt gelesen: Länder führen das bundesweite Register; Aktualisierung aus Zertifikaten und Mitteilungen der technischen Überwachungsorganisationen bzw. Entsorgergemeinschaften. Online-Angebot verweist ausdrücklich auf **[fachbetrieberegister.gadsys.de/fachbetrieberegister/](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/)**.
- [§ 28 EfbV](https://www.gesetze-im-internet.de/efbv_2017/__28.html) ist dort verlinkt, wurde in diesem Follow-up **nicht separat gelesen**; rechtliche Beschreibung oben ist der tatsächlich gelesene LfU-/GADSYS-Inhalt.

Diese institutionellen Quellen belegen den **Prüfweg**, nicht die Identität oder Zertifizierung von M.R. Recycling.

### Was am Register wirklich geprüft wurde

Der aktuelle Register-Root antwortet mit HTTP 200, App-Metadatum `eEFBV FBR`, Vaadin-/JavaScript-Bootstrap und zunächst leerem `outlet`. Über den öffentlich verwendeten Initialisierungsaufruf (`?v-r=init&location=&query=`) und eine UI-Navigation wurden serverseitige UI-Inhalte gelesen, unter anderem:

- Seitentitel **„eEFBV Fachbetrieberegister v1.15.0“**;
- „Entsorgungsfachbetriebe“, „Suche“, „Zurücksetzen“, „Erweiterte Suche“;
- Abfallart-/Tätigkeitsfelder und ein Ergebnis-Grid;
- Google-Maps-Zustimmung für die Karte (nicht erteilt).

Das ist **noch keine Suchantwort** zu Michael Reinhardt, M.R. Recycling, Augsburg oder einer Wankstraßen-Adresse. Die weitere manuelle HTTP-Abbildung der dynamischen UI scheiterte; ein Versuch, Komponenten-Anbindung zu melden, lieferte die UI-Antwort **„Internal error“**. Danach nicht als funktionierenden Suchlauf fortgesetzt. Kein Browser/Playwright/Selenium/Chromium war in der Umgebung verfügbar. Es wurde weder eine Trefferliste noch ein Zertifikat zu diesem Betreiber gelesen. Insbesondere keine Zertifikatsnummer, Zertifizierungsorganisation, Gültigkeit, Standortanlage, Tätigkeit oder AVV-Liste behaupten.

Die zuvor versuchten Hosts `https://www.zks-abfall.de/` und `https://fachbetrieberegister.zks-abfall.de/fachbetrieberegister/` lieferten Transportfehler; ein zweiter Abruf mit Python `requests` konkretisierte dies zu **NameResolutionError**. Diese alten Zugangspfade sind **kein** Nachweis einer fehlenden Efb-Zertifizierung und kein Beleg dafür, dass das neue Register unerreichbar wäre.

**Nächster Originalbeleg-Schritt:** Im funktionierenden Browser den amtlich verlinkten GADSYS-Zugang verwenden. Zunächst breit Michael/Reinhardt bzw. Augsburg suchen, dann Betreiberidentität und einzelne Standortanlagen 7/11b vergleichen; MRA Recycling GmbH und andere M&R-Unternehmen nicht übertragen. Bei Treffer Originalurkunde samt Anlagen lesen, Gültigkeit und genau zertifizierte Tätigkeit/Abfallarten dokumentieren. Eine bloße Beförderer-Zertifizierung wäre noch kein Nachweis eines stationären Annahmehofs.

## 2. Zertifizierungswerbung und begrenzte Originaldokument-Suche beim Betreiber

### Tatsächlich gelesene Behauptungen

| Betreiberquelle | Direkt gelesener Inhalt | Tragweite |
| --- | --- | --- |
| [Container-Homepage](https://www.abfallentsorgung-augsburg.de/) | Badge „Zertifizierter Entsorgungsfachbetrieb“; Body „Als zertifizierter Entsorgungsfachbetrieb mit über 10 Jahren Erfahrung …“ | Aktuelle Eigenbehauptung, keine Urkunde. Im geprüften HTML kein entsprechender PDF-/Zertifikatslink gefunden. |
| [Öffentliche WordPress-Seitensuche `zertifiz`](https://www.abfallentsorgung-augsburg.de/wp-json/wp/v2/pages?search=zertifiz&per_page=100) | HTTP 200, `X-WP-Total: 1`: Homepage, ID 6482, mit obigen Aussagen. | Nur im öffentlichen API-Suchindex erfasste Seiten; keine vollständige Domain-/Dateisystemsuche. |
| [Öffentliche WordPress-Artikelsuche `zertifiz`](https://www.abfallentsorgung-augsburg.de/wp-json/wp/v2/posts?search=zertifiz&per_page=100) | HTTP 200, `X-WP-Total: 3`; die drei unten genannten Artikelinhalte wurden aus der API gelesen. | Wiederholte Eigenbehauptungen, nicht drei unabhängige Belege. |

Die drei aus der API gelesenen Artikel:

- [Container mieten in Augsburg](https://www.abfallentsorgung-augsburg.de/2026/05/14/container-mieten-in-augsburg/), ID 5962: spricht von „über 20 Jahren Erfahrung als zertifizierter Entsorgungsfachbetrieb“ und behauptet auf Wunsch einen „offiziellen Entsorgungsnachweis“. Kein Originalzertifikat oder entsprechende Urkundennummer im geprüften Inhalt. Die Homepage nennt dagegen über zehn Jahre: **Erfahrungstexte nicht als belegte Unternehmensgeschichte verwenden**.
- [Mikroplastik in den Ozeanen](https://www.abfallentsorgung-augsburg.de/2024/01/23/mikroplastik-in-den-ozeanen/), ID 4832: „Als zertifizierter Entsorgungsfachbetrieb …“ und „Wir sind zertifizierter Entsorgungsfachbetrieb …“.
- [Union fragt Regierung nach EU-Plastikabgabe](https://www.abfallentsorgung-augsburg.de/2024/01/22/union-fragt-regierung-nach-eu-plastikabgabe/), ID 4825: „Für Entsorgungsfachbetriebe wie M.R. Recycling Augsburg …“ und weitere ausdrückliche Zertifizierungsbehauptungen. Enthält im gelesenen Text Verweise auf April 2026 trotz 2024-URL. **URL-Datum ist kein Zertifizierungsdatum und keine unveränderte Veröffentlichungshistorie**.

Die Artikel wurden als veröffentlichte `content.rendered`-Inhalte der Betreiber-API gelesen; die einzelnen Artikel-URLs oben wurden nicht zusätzlich separat abgerufen. In den vier einschlägigen API-Inhalten wurde kein Link mit PDF-/Zertifikats-/GADSYS-Motiv gefunden. Das schließt Dateien unter anderen Namen, Bilder ohne passende Beschriftung, externe Zertifikatsportale oder unverlinkte Dokumente nicht aus.

### Medien-Suchgrenzen explizit

Folgende **öffentliche GET-Abrufe** antworteten jeweils HTTP 200 mit `[]`:

- Containerdomain: `/wp-json/wp/v2/media?search=zert&per_page=100`;
- Containerdomain: `/wp-json/wp/v2/media?search=pdf&per_page=100`;
- Containerdomain: `/wp-json/wp/v2/media?search=efb&per_page=100`;
- Containerdomain: `/wp-json/wp/v2/media?search=urkunde&per_page=100`;
- [Containerdomain: application-Medien](https://www.abfallentsorgung-augsburg.de/wp-json/wp/v2/media?media_type=application&per_page=100);
- [M.R.-Metall: application-Medien](https://mrmetall.de/wp-json/wp/v2/media?media_type=application&per_page=100).

Für die `efb`-/`urkunde`-Abfragen wurde außerdem `X-WP-Total: 0` gelesen. **Das sind echte Nichttreffer dieser API-Abfragen, keine Nichttreffer im amtlichen Efb-Register.** Keine vollständige Inventur aller Betreiberbilder oder Upload-Verzeichnisse durchgeführt. Keine Zertifizierungsstelle identifiziert, deren Originalurkunde man bereits gezielt abrufen könnte.

## 3. Standortkonflikt: 7 versus 11b erneut aus Originalseiten

| Direkt erneut gelesene URL | Wortlaut / Identitätsmerkmale | Schluss |
| --- | --- | --- |
| [Container-Impressum](https://www.abfallentsorgung-augsburg.de/impressum) | Michael Reinhardt, M.R. Recycling – Abfallentsorgung Augsburg; Wankstraße **11b**, 86165 Augsburg; Anschrift zusätzlich ausdrücklich „Postadresse“; DE340523253; Mobil +49 1511 766 11 13. | Betreiber-/Postadresse, nicht unabhängiger Nachweis des Hofzugangs. Keine HR-Nummer im Impressum. |
| [Container-AGB](https://www.abfallentsorgung-augsburg.de/agb) | „Die Metalle können am Platz der M.R. Recycling **Wankstraße 7 86165 Augsburg**, angeliefert werden.“ | Explizite Annahmeplatzbehauptung, nicht auf 11b umschreiben. |
| [M.R.-Metall-Impressum](https://mrmetall.de/impressum/) | Michael Reinhardt, M.R. Recycling; **11b**, gleiche DE340523253, Mobilnummer und Containerdomain-Mail; ebenfalls „Postadresse“. | Gleiche Eigenidentität; weiterhin **keine zweite unabhängige Quelle**. |
| [M.R.-Metall-AGB](https://mrmetall.de/agb/) | Derselbe ausdrückliche Satz zur Selbstanlieferung am Platz **Wankstraße 7**. | Neuer direkt gelesener Gegenbeleg zur 11b-Ankaufseite, aber kein unabhängiger zweiter Hofbeleg. |
| [M.R.-Metall-Ankauf](https://mrmetall.de/metallankauf/) | „Kommen Sie einfach bei uns vorbei: **Wankstraße 11b**, 86165 Augsburg.“ CTA ebenfalls 11b. | Mehr als Postadresse behauptet, jedoch im eigenen Auftritt widersprüchlich. |

In der Containerhomepage wurde zusätzlich im selbst publizierten JSON-LD `legalName: Michael Reinhardt`, `streetAddress: Wankstraße 11b`, GEO **48.386 / 10.935** und Mo–Fr 07–17 gelesen. **Diese Betreiber-Metadaten lösen den Adresskonflikt nicht; keine Hofkoordinate, kein Eingang und keine verifizierte Öffnungszeit daraus ableiten.** JSON-LD ist kein Kataster-/Genehmigungs-/Registerbeleg.

Unverändert offen: Sind 7 und 11b separate Betriebs-/Postadressen, alte Texte, ein Umzug oder ein Fehler? Keine dieser Erklärungen konnte unabhängig bestätigt werden. Kein qualifiziert unabhängiger Nachweis eines aktuellen Annahmeplatzes an 7 oder 11b und kein unabhängig verifizierter anderer Hofbetreiber an einer der beiden Adressen gewonnen. **Nicht zwei Filialen anlegen, nicht Robert Reinhardt zuschlagen, keine Umfirmierung/Nachfolge behaupten.**

## 4. Gewerbe-/Firmenregister: echte Zugangsbefunde, kein Auszug

### Zuständige kommunale Gewerbeauskunft gefunden

Über [Dienste G der Stadt Augsburg](https://www.augsburg.de/buergerservice-rathaus/buergerservice/dienste-a-z/dienste/g) wurde die aktuelle Leistungsseite **[Gewerbeauskunft – Ordnungsamt](https://www.augsburg.de/buergerservice-rathaus/buergerservice/dienste-a-z/aemterweise/leistungen-ordnungsamt/gewerbeauskunft)** gefunden und vollständig in den relevanten Abschnitten gelesen. Sie erklärt:

- **„KEINE Auskunft aus dem Gewerbezentralregister!“**
- Antrag persönlich, schriftlich, per E-Mail oder Fax; telefonisch keine Auskünfte.
- Bekannte Firma/Gewerbetreibender und zuletzt bekannte Anschrift angeben.
- Einfache Auskunft beschränkt auf **Name des Betriebes oder Inhabers, betriebliche Anschrift und angezeigte Tätigkeit**.
- **„Die Gewerbedatei ist keine öffentliche Datei“**; kein Rechtsanspruch auf Datenmitteilung, Ermessensentscheidung.
- Inhalte entsprechen dem der Behörde bis zur Auskunft angezeigten Stand; keine Gewähr für Richtigkeit/Vollständigkeit.
- Gebühr **15 EUR auch bei Nichtermittlung**, jede gleichzeitig beantragte weitere Auskunft zusätzlich 7,50 EUR.
- Zuständig Ordnungsamt, Grottenau 1, 86150 Augsburg; `gewerberecht@augsburg.de`; ein Online-Antragsformular wird verlinkt.

**Nicht bestellt, nicht abgeschickt, kein Formular ausgefüllt.** Die Seite ist ein unabhängiger amtlicher Verfahrensbeleg, aber **kein Firmen-/Standortbeleg für Michael Reinhardt**. Für eine spätere autorisierte Auskunft genau „Michael Reinhardt, M.R. Recycling / Abfallentsorgung Augsburg / M.R. Metall“, beide Wankstraßen-Adressen und die Frage nach angezeigter betrieblicher Anschrift/Tätigkeit angeben. Auch ein Gewerbeauszug würde nicht automatisch aktuelle Publikumsanlieferung oder Hoföffnungszeiten beweisen.

Zwei zuvor nur vermutete Bürgeramt-URLs für `gewerberegisterauskunft` antworteten 404. Das wurde durch die tatsächlich verlinkte Ordnungsamt-Leistungsseite ersetzt, nicht als „Augsburg bietet keine Auskunft“ missverstanden.

### Creditreform und Handelsregister

- [Creditreform 8030463596](https://firmeneintrag.creditreform.de/86165/8030463596/MICHAEL_REINHARDT_M_R_RECYCLING): erneuter HTTPS-Abruf **403**, nur Abwehr-/JavaScript-Antwort. Keine Anschrift, Rechtsform, Wirtschaftaktivität oder aktuelle Hofleistung aus dem Vollprofil gelesen. Den im Ausgangsbericht dokumentierten Suchauszug nicht zu einem hier verifizierten Vollprofil aufgewertet.
- [Registerportal Startseite](https://www.handelsregister.de/rp_web/welcome.xhtml): HTTP 200, Portalrahmen gelesen; direkter Aufruf von `/rp_web/erweitertesuche.xhtml` HTTP **400**. **Keine erfolgreich ausgeführte Register-Namenssuche und kein Registerauszug.** Nicht als „nicht im Handelsregister“ oder als Beweis einer bestimmten Rechtsform werten. Die fehlende Registerangabe im Betreiber-Impressum bleibt lediglich ein Befund dieses Impressums.

## 5. Freie Websuche: technische Grenze, keine belastbaren Standort-Nichttreffer

Die Websearch-Funktion scheiterte mit `Unable to search the web`. Ergänzende Suchmaschinenabrufe wurden versucht:

- Google: gezielte Suchbegriffe `"M.R. Recycling" "Zertifikat"`, `"Michael Reinhardt" "Wank"`, `"Wankstraße 7" Augsburg`, `"Wankstraße 11b" Augsburg`, `site:augsburg.de Gewerberegisterauskunft`, `Entsorgungsfachbetrieberegister ZKS öffentlich`. HTTP-Antworten enthielten nur Consent-/Weiterleitungshinweise, **keine verwertbaren Trefferlisten**.
- Bing für `"Wankstraße" "Reinhardt"`: zwar lesbare Ergebnisliste, aber fachfremde Outlook-/E-Mail-Treffer. Nicht als Identitäts-/Standortbeleg oder glaubwürdigen Nullbefund verwendet.
- DuckDuckGo HTML für Michael Reinhardt/Recycling: Verbindungsabbruch (`Connection reset by peer`).

Folglich hier **keine erfolgreiche flächige Recherche unabhängiger Nachbarn, kommunaler Genehmigungen oder Pressestandortbelege behaupten**. Die oben belegten Betreibertexte bleiben Einzelbelege derselben Eigenidentität.

## Owner-Gate / nächste Entscheidung

- Aktueller unabhängiger Identitäts-/Standortbeleg fehlt weiterhin. Der Zwei-Quellen-Standard ist durch LfU-/GADSYS-Verfahrensseiten und kommunale Auskunftsinformation **nicht** erfüllt, da sie diesen Betrieb nicht nennen. Die README-Ausnahme der registerkongruenten autoritativen Betreiberquelle greift weiterhin nicht.
- Efb-Claim **unverifiziert**, nicht widerlegt. Vor Zertifizierungsfeld entweder Originalurkunde über den jetzt gefundenen aktuellen Registerzugang lesen oder später ausdrücklich autorisiert vom Betreiber beschaffen; Betreibername, Standortanlagen und zertifizierte Tätigkeit abgleichen.
- Annahmeplatz **7/11b ungelöst**; Gewerbeauskunft könnte betriebliche Anschrift/Tätigkeit unabhängig belegen. Aktuelle Anlieferung müsste zusätzlich mit konfliktauflösender Originalquelle gesichert werden.
- M.R. Metall bleibt derselben Eigenidentität zugeordnet, nicht als zweiter Händler oder zweite unabhängige Quelle. Robert-Reinhardt-/MRA-Abgrenzung wurde hier nicht durch einen neuen Rechtskettenbeleg entschieden; Ausgangsbericht bleibt maßgeblich.
- **Keine neue Preisprüfung/-tabelle und keine harmonisierten Preise** in diesem Follow-up. Konditionen-/Preiswidersprüche des Ausgangsberichts bleiben offen; weder Qualitäts-/Lieferweg-Erklärungen noch universelle Abholschwellen hinzuerfinden.
- Nur diesen Recherche-Check-in integrieren. Keine neue aktive Händleranlage aus dem Nichtnachweis ableiten; Owner entscheidet über weiteren Originalregisterabruf oder eine ausdrücklich autorisierte gebührenpflichtige Gewerbeauskunft. Keine Produktionsfreigabe aus diesem Follow-up.

## 6. Nachtrag: öffentliche Efb-Betreibersuche tatsächlich ausgeführt

### Verfahren und erfolgreiche Abfrage, nicht bloß Portalaufruf

Nach Owner-Hinweis [Habersack-Registerbericht](checkin-20261014-1830-habersack-certificate.md) und die vorhandenen Antwortartefakte `habersack-fbr-navigate.json`, `habersack-efb-results.json`, `habersack-fbr-results.json`, `habersack-fbr-local-results.json` und `habersack-fbr-location.json` unter `/tmp/opencode/` **nur gelesen**. Keine vorhandenen Artefakte, Sitzungen oder Korn-Dateien geändert oder wiederverwendet. Die nachfolgenden Abrufe erfolgten in **eigenen neuen HTTP-Sitzungen** ausschließlich am öffentlichen Register; keine Anmeldung, keine Registerbearbeitung.

Der funktionierende Weg ist die **explizite Efb-Route mit Suchparametern**, nicht die anfänglich verwendete leere Root-Route:

1. GET auf den öffentlichen Register-Root und dessen Initialisierung mit `v-r=init`, `location=Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Reinhardt`, leerem `query`.
2. POST auf `?v-r=uidl&v-uiId=0` mit dem vom Server initialisierten Sitzungszustand und Ereignis `ui-navigate` auf genau diese Route. Die Antwort enthält gleichzeitig den gesetzten Suchfeldwert, die gewählte Suchart, **„7 Treffer gefunden“** und die sieben tatsächlich zurückgegebenen Grid-Datensätze.
3. Für weitere UI-Abfragen Suchfelder mittels `mSync`/`value-changed`, anschließend Suchbutton `click`; Hauptsitzmodus über den tatsächlichen UI-Wert `2` des Felds „Suche nach Angaben zum Hauptsitz“.
4. Nachladen größerer Ergebnislisten über die vom Server exportierte Grid-Methode `setViewportRange`: Bereich ab 50, Länge 50; bei MR zusätzlich ab 100. Damit wurden **alle 112 MR- und alle 79 lokalen Datensätze** gelesen, nicht nur eine Bildschirmseite.
5. Standortsuche: Namensfeld leer, Standortfeld „Augsburg“, Vorschlag **„Augsburg, Deutschland“** ausgewählt und bestätigt, Umkreis **10 km**. Das ist eine Ortsmittelpunkt-Abfrage, kein geocodierter M.R.-Hof.

Die erfolgreich ausgeführten Aufrufe antworteten HTTP 200 ohne `appError`; die Serverantworten enthalten Daten und echte Trefferzahlen. Der im ersten Versuch beobachtete `Internal error` gehört zum dortigen fehlgeschlagenen Komponenten-Anbindungsversuch und ist **kein Ergebnis dieser jetzt erfolgreichen Suchen**. Browsermangel ist somit **keine verbleibende Register-Suchgrenze**. CSRF-Werte und Cookies werden nicht ausgegeben. Eigene Antworten/PDF wurden im Arbeitsspeicher verarbeitet, nicht in fremden Artefakten gespeichert; ausschließlich diese Berichtsdatei wurde ergänzt.

### Tatsächlich gelesene Suchergebnisse

Voreinstellung der folgenden Namensabfragen: **Angaben zu zertifizierten Standorten**, **alle Zertifikatsanlagen zu einem Standort anzeigen**, kein Orts-/Bundesland-/Tätigkeitsfilter. Trefferzahlen zählen daher **Anlagenzeilen**, nicht notwendigerweise Unternehmen.

| Name im Register-Suchfeld / Standortkriterium | Serverergebnis | Auswertung / öffentliche Quelle |
| --- | --- | --- |
| `Michael Reinhardt` | **0 Treffer gefunden** | [Öffentliche Efb-Namensroute](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Michael%20Reinhardt). Tatsächlicher Nullbefund dieser Suche, kein Abruffehler. |
| `Reinhardt` | **7 Treffer gefunden**, alle 7 gelesen | [Route](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Reinhardt). Drei andersörtige Rechtsträger, siehe Tabelle unten. Kein Michael, Augsburg oder Wankstraße. |
| `Reinhard` | **15 Treffer gefunden**, alle 15 gelesen | [Route](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Reinhard). Zusätzlich Keim/Felsberg, Juraschek/Celle, Roselieb/Celle und Schwier/Petershagen; kein Michael-/Wankstraßenfund. |
| `M.R.` | **4 Treffer gefunden**, alle 4 gelesen | [Route](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=M.R.). Ausschließlich **C.M.R. Recycling Vertriebs GmbH**, Ripshorster Straße 386, 45357 Essen, ZZEE00300027 / Anlagen 1–4. Nicht der Augsburger Einzelbetreiber. |
| `M.R. Recycling` | **4 Treffer gefunden**, alle 4 gelesen | [Route](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=M.R.%20Recycling). Ebenfalls nur derselbe Essener C.M.R.-Betrieb. Dies zeigt zugleich, dass die Namenssuche keine strikte Gleichheit des gesamten Namens verlangt. |
| `MR` | **112 Treffer gefunden**, alle 112 Zeilen über drei Bereiche gelesen | [Route](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=MR). In den 112 Name-/Hauptsitz-/Adresszeilen kein Michael Reinhardt/M.R. Recycling oder Wankstraße; die vier Augsburger Treffer sind **MRA**, nicht M.R. |
| `M R` | **500 Treffer gefunden**, nur erste 50 Zeilen gelesen | [Route](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=M%20R). Breite, unspezifische Ergebnisse. **Kein vollständiger Negativbefund aus dieser Abfrage**; 500 ist die Anzeige, keine hier geprüfte unbeschränkte Gesamtmenge. |
| `Augsburg` **im Namensfeld** | **19 Treffer gefunden**, alle 19 gelesen | [Route](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Augsburg). REMONDIS, AVA, DB Bahnbau, Gigler und Kühl; kein M.R./Wank. **Keine vollständige Ortsabfrage:** MRA erscheint hier nicht, obwohl in Augsburg. |
| Name leer; Standort **Augsburg, Deutschland**, Umkreis **10 km** | **79 Treffer gefunden**, alle 79 Zeilen über zwei Bereiche gelesen | [Öffentliche Efb-Suche](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe), UI-Schritte wie oben. Kein Reinhardt/M.R.-Name und **keine Wankstraßen-Adresse**, weder 7 noch 11b. MRA und APP Logistik an Firnhaberstraße 112 sind getrennt vorhanden. |
| `MRA` | **6 Treffer gefunden**, alle 6 gelesen | [Route](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=MRA). Vier Anlagen **MRA Recycling GmbH** in Augsburg; außerdem TOMRA/Mühlheim-Kärlich und Mechanische Restabfall-Aufbereitungsanlage/Münster. Keine Michael-Zuordnung. |
| `Wank` **im Namensfeld** | **0 Treffer gefunden** | [Route](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Wank). **Keine Straßenfeldsuche**; nicht als eigenständige Vollprüfung der Adresse verwenden. Die Straßenprüfung erfolgte an den tatsächlich gelesenen 79 lokalen Ergebnisadressen. |

Zusätzlich den Modus **„Suche nach Angaben zum Hauptsitz“** tatsächlich über die UI gewählt, ohne Ortsfilter: `Reinhardt` **3**, `Michael Reinhardt` **0**, `M.R. Recycling` **1**, `MRA` **2**, `Augsburg` **1 Treffer gefunden**. Die zurückgegebenen Hauptsitze wurden gelesen: bei Reinhardt dieselben drei unten genannten Betriebe; bei M.R. Recycling nur C.M.R./Essen; bei MRA nur TOMRA und MRA/Firnhaberstraße; bei Augsburg nur AVA. Die folgende Hauptsitz-Abfrage `M.R.` lieferte ebenfalls einen Essener C.M.R.-Datensatz, aber **keine neue Treffertext-Änderung** in dieser Deltaantwort; daraus keine zusätzlich abgelesene Trefferzahl behauptet.

### Vollständige Abgrenzung der sieben Reinhardt-Anlagenzeilen

| Originalregister-Bezeichnung | Adresse | Vorgang / gelesene Anlagen und Tätigkeiten |
| --- | --- | --- |
| R+S Reinhardt & Sela GmbH u.Co.KG | Am Bahndamm 2, 63683 Ortenberg-Bleichenbach | ZZFT01300094: Anlage 1 Sammeln/Befördern, 2 Lagern, 3 Behandeln |
| VEREVE Inh. Gina Reinhardt | Otto-Hahn-Straße 6, 71254 Ditzingen; Standort „Anlage“ Robert-Bosch-Strasse 1, 71729 Erdmannhausen | ZZFT00300031: Anlage 1 Lagern/Erdmannhausen, 2 Sammeln/Befördern/Ditzingen |
| Reinhardt Rohstoffe GmbH | Lichterfelder Weg 1–3, 14167 Berlin | ZZLT00400087: Anlage 1 Handeln, 2 „Schrottplatz“, Lagern/Beseitigen |

Diese Originalregister-Treffer sind **Namensvetterabgrenzung**, keine Zuordnung zu Michael oder Robert Reinhardt in Augsburg. Ihre Original-PDFs wurden nicht verfolgt, da Namen/Orte schon nicht kongruent sind; keine Gültigkeit dieser anderen Zertifikate behauptet. MRA wurde wegen des ausdrücklich benannten Augsburger Verwechslungsrisikos zusätzlich bis zur Originalurkunde geprüft.

### MRA: Originalurkunde und sämtliche Standortanlagen tatsächlich gelesen

Aus den vier Augsburger MRA-Treffern den Datensatz ZZHT00301309 / Anlage 1 gewählt, „Details“ geöffnet und die vom Server ausgegebene öffentliche Detail-/Downloadfunktion benutzt. Die UI fordert dabei die aktuelle Seiten-URL an; die reguläre Vaadin-Return-Channel-Antwort lieferte den Detaildialog mit:

- **MRA Recycling GmbH**, Firnhaberstraße 112, 86159 Augsburg;
- Vorgangsnummer ZZHT00301309, Zertifikatsanlage 1, Sammeln/Befördern;
- [öffentlichem Zertifikat-Permalink](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=MRA&p=ZZHT00301309&fd=1);
- tatsächlichem dynamischen PDF-Link `ZZHT003013095.pdf`.

PDF über dieselbe eigene Sitzung abgerufen: **HTTP 200, `application/pdf`, 517.383 Bytes, `%PDF-`**. Alle zehn Seiten mit `pdftotext -layout` aus dem Arbeitsspeicher gelesen. Der dynamische Sitzungs-Download ist kein dauerhafter Quellenlink; oben steht stattdessen der tatsächlich vom Register ausgegebene Permalink. Konkreter Originalinhalt:

| Original-PDF-Feld | Tatsächlich gelesener Inhalt |
| --- | --- |
| Zertifizierungsorganisation | **DEKRA Certification GmbH**, Handwerkstraße 15, 70565 Stuttgart |
| Zertifikatsnummer der Organisation | **290123002** |
| Vollständige Vorgangsnummer | **ZZHT003013094005**; nicht mit dem zwölfstelligen UI-Grundvorgang oder dem Download-Dateinamen gleichsetzen |
| Rechtsträger / Hauptsitz | **MRA Recycling GmbH**, Firnhaberstraße 112, 86159 Augsburg |
| Registeridentität | **HRB 23388**, Registergericht Augsburg |
| Prüfungsdatum / Ausstellungsdatum | **21.01.2026 / 02.06.2026** |
| Gültig bis | **21.07.2027** — liegt nach dem Recherchetag 14.10.2026; PDF-Frist, keine eigene Widerrufs-/Signaturprüfung |
| Anlagen | **4**, alle nennen denselben Rechtsträger und **Firnhaberstraße 112** |
| Anlage 1, S. 2–3 | UI Sammeln/Befördern; Kennnummer I761Q01424; Beschreibung „Containerdienstleistungen, allg. Transportdienstleistungen“ |
| Anlage 2, S. 4–5 | UI Handeln; I761H00304; „Handel von Wert-, Rest- und Abfallstoffen“ |
| Anlage 3, S. 6–8 | UI Lagern/Verwerten; I761S00120; Lagerung Eisen-/Nichteisenschrotte einschließlich Autowracks, Nr. 8.12.3.2; weiteres Zwischenlager; AVV-Liste u. a. 170401 Kupfer/Bronze/Messing, 170402 Aluminium, 170405 Eisen/Stahl, 170411 Kabel, 200140 Metalle sowie 160601* Bleibatterien |
| Anlage 4, S. 9–10 | UI Behandeln/Verwerten; I761S00120; Anlage zur sonstigen Behandlung nicht gefährlicher Abfälle nach Nr. 8.11.2.2; materialspezifische AVV-Liste, u. a. 170401–170407, 170411 und 200140 |

**Textauslese-Grenze:** Die PDF enthält Formular-/Ankreuzfelder; ihre grafischen Markierungen sind im `pdftotext`-Text nicht zuverlässig erhalten. Deshalb Tätigkeitsauswahl oben zusätzlich mit den tatsächlich gelesenen Register-Grid-Zeilen abgeglichen. Aus den bloß gedruckten Auswahlmöglichkeiten „alle Abfallarten / alle nicht gefährlichen / alle gefährlichen“ insbesondere für Anlagen 1/2 **keine pauschale Auswahl konstruiert**. Die benannten AVV-Codes in Anlagen 3/4 stehen hingegen als konkrete Tabellenzeilen im Originaltext. Keine Kunden-/Ankauf-/Preiskonditionen aus dieser Urkunde abgeleitet.

Im vollständig gelesenen Originaltext sind **weder Michael Reinhardt noch Wankstraße 7 oder 11b** enthalten. Damit ist das konkrete Augsburger MRA-Zertifikat **positiv einem anderen Rechtsträger und einem anderen Standort zugeordnet**; nicht auf die Zertifizierungswerbung von Michael/M.R. übertragen. Die Register-/PDF-Kongruenz HRB 23388/Firnhaberstraße 112 passt zum separat beschriebenen MRA-Kandidaten im Ausgangsbericht, beweist aber keine Betreiberkette zwischen MRA und Michael.

### Aktualisiertes Owner-Ergebnis / Reichweite der Nichttreffer

**Jetzt echter Originalregister-Nichtnachweis statt bloßer Browser-/Abrufgrenze:** In den erfolgreichen Efb-Namens-/Hauptsitzabfragen und in sämtlichen 79 gelesenen Anlagenzeilen der 10-km-Standortsuche **kein Datensatz zu Michael Reinhardt/M.R. Recycling an Wankstraße 7 oder 11b**. Für Michael wurden daher weiterhin **keine Originalurkunde, Anlagen, Tätigkeiten oder Gültigkeitsfrist** gewonnen. Die bekannte Werbung „zertifizierter Entsorgungsfachbetrieb“ ist nicht positiv verifiziert und erhält mit diesem Registercheck eine konkrete zusätzliche Prüflücke.

Das ist **keine rechtsverbindliche Negativbescheinigung**: alternative eingetragene Bezeichnung, anderer Rechtsträger, abweichende Standorte oder Melde-/Aktualisierungslücken bleiben möglich. Die unspezifische `M R`-Abfrage wurde nicht vollständig ausgewertet; nicht als globaler Ausschluss darstellen. Das Register liefert zertifizierte Standorte/Tätigkeiten, nicht automatisch Publikumsanlieferung, Gewerbeaktivität, Schrottvergütung oder Hoftore. **Keine Schließung, Rechtswidrigkeit oder definitive „nicht zertifiziert“-Feststellung** aus den Nichttreffern ableiten.

Die Wankstraße-7-/11b-Frage bleibt ungelöst; keine der beiden Adressen ist durch diesen Registercheck positiv als Michael-Reinhardt-Annahmehof belegt. Das nächste sachliche Gate ist eine konkret diesem Betreiber zuordenbare Urkunde/Vorgangsnummer oder eine ausdrücklich autorisierte kommunale Gewerbeauskunft mit aktueller betrieblicher Anschrift, ergänzt um eine konfliktauflösende Annahmeplatzquelle. **Nicht mehr bloß einen Browser beschaffen oder MRA als Ersatzbeleg verwenden.** Keine Kontakte oder Auskunftsbestellungen ausgeführt. Weiterhin nur diese Datei ergänzt; keine Dossier-/Produktions-/Preiswrites und keine Integration/Commit/Deploy.

## Abschließendes Owner-Gate

Validierung vor Integration: `cargo test -p schrott-mcp-ingestion seed_traders::tests` — **11 erfolgreich, 0 Fehler**.

Der Owner hat den vollständigen Bericht und den Originalregister-Nachtrag gelesen. Registerabfragen und PDF-Auswertung sind dokumentierte Agentenbefunde; die UI-Sitzung und Originalurkunde wurden vom Owner nicht unabhängig wiederholt. Sie rechtfertigen keine Freigabe oder Zertifizierungsübernahme für Michael Reinhardt. Ausschließlich Forschungsbericht integrieren, keine neue Händleranlage und keine Adress-/Preis-/Produktionsänderung. Gebührenpflichtige Gewerbeauskunft nicht bestellt. Reine Dokumentation erfordert keinen Redeploy; Push bleibt von funktionierender GitHub-Authentifizierung abhängig.
