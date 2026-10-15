# Hilgenberg Bad Salzuflen – Integrationsreview 15.10.2026

## Entscheidung

**Nur als begrenzte Recherche-/Unsicherheitsnotiz integrierbar, nicht unverändert als abgeschlossene Identitätsprüfung freigeben.** Der Diff gegen HEAD fügt ausschließlich den Timeline-Abschnitt „Recherche 13.10.2026“ in `dossiers/nw/nw-bad-salzuflen-hilgenberg.md` hinzu; Frontmatter und bestehende Historie bleiben unverändert. Die vorsichtige Datenentscheidung ist richtig: `status: pruefung`, `website_status: unbekannt`, keine Kontakt-, Website-, Preis- oder GEO-Fills. Keine Freigabe als aktuell nachgewiesener Händler, keine Schließungsentscheidung und keine Produktionsaussage.

**Wichtige neue Spur:** Die unabhängige Suche ergab ein direkt lesbares firminform-Profil „Johannes Hilgenberg e.K.“ mit derselben Anschrift Kiliansweg 6, 32108 Bad Salzuflen und dem Registerhinweis **AG Lemgo HRA 53**, dort als **inaktiv** bezeichnet (Profilaktualisierung 04.10.2017). Das ist ein relevanter Register-Rechercheansatz, aber noch kein abgerufener Registerauszug und nach README keine automatisch anzuerkennende zweite Evidenzquelle. Der pauschale Eindruck „kein Register(ansatz)“ wäre deshalb irreführend. Auch „inaktiv“ darf nicht als bewiesene heutige Betriebsschließung übernommen werden.

## Maßstab und Umfang

- Gelesen: `README.md`, insbesondere Zeilen 160–202 (zweiseitige Identitätsprüfung, Portale nur Leads, zwei unabhängige zulässige Quellen bzw. streng begrenzte Betreiberprimärquellen-Ausnahme) und 204–240 (Betreiberketten, Owner-Gate).
- Geprüft: `git diff HEAD -- dossiers/nw/nw-bad-salzuflen-hilgenberg.md` sowie gesamtes aktuelles Dossier. Der Diff umfasst vier zusätzliche Zeilen, darunter eine neue Recherche-Aufzählung. Dies ist kein vollständiger Neuaudit sämtlicher historischer Aussagen.
- Alle nachfolgenden eigenen Webabrufe erfolgten am **15.10.2026**. Ein heutiger Abruf bestätigt weder den behaupteten Abruf am 13.10.2026 noch damalige Seitenstände. Keine historische Archivkopie geprüft.
- Nur dieser Reviewbericht wurde geschrieben; Dossier, Datenbanken und Commits wurden nicht geändert. Keine Seedtests ausgeführt: keine Dossierimplementierung in diesem Review; der Owner muss vor einer späteren Integration weiterhin `cargo test -p schrott-mcp-ingestion seed` ausführen.

## Direkt geprüfte Quellen

### 1. 11880: adressgenauer Lead, kein Betriebsnachweis

URL: https://www.11880.com/branchenbuch/bad-salzuflen/060440090B26054969/johannes-hilgenberg-rohstoffe.html

Markdown und HTML erfolgreich gelesen. Tatsächlich enthalten:

- „Johannes Hilgenberg Rohstoffe“, Kiliansweg 6, 32108 Bad Salzuflen (Schötmar), (05222) 81651;
- Kategorien Altmaterialhandel, Rohstoff Großhandel, Schrotthandel;
- „Website hinzufügen“, „E-Mail-Adresse hinzufügen“, „Öffnungszeiten hinzufügen“ statt entsprechender Betreiberangaben;
- Aktualisierungs-/Eintragsdatenanzeige **15.10.2026**. Die alten Angaben im Dossier zu 29.06.2021 bzw. 01.10.2026 sind damit nicht historisch nachverifiziert;
- allgemeiner FAQ-Text über „vom Inhaber bestätigt“ und eine allgemeine Eurozeichen-Preisspannen-Erklärung. Beides ist weder eine konkrete Inhaberbestätigung dieses Eintrags noch eine Materialpreisliste;
- HTML-JSON-LD mit Portal-Koordinaten. Nicht übernommen: die Koordinaten lösen weder Identität noch aktuelle Standortexistenz.

Das Portal beschreibt selbst einen Datenmix aus Telekommunikationsanbietern, weiteren Quellen, Eigenrecherche und Nutzerfeedback. Damit weder eigene Betreiberquelle noch unabhängiger Registerbeleg; Tagesdatum und technische Erreichbarkeit werten es nicht auf.

### 2. Hilgenberg GmbH: belegter anderer Websitebetreiber

URLs, beide erfolgreich gelesen:

- https://www.hilgenberg-gmbh.de/impressum
- https://www.hilgenberg-gmbh.de/en/company/locations

Impressum: **Hilgenberg GmbH, Strauchgraben 2, 34323 Malsfeld**, Sitz Malsfeld, AG Fritzlar **HRB 11038**, Geschäftsführer Ingo Hilgenberg und Florian Salewski. Produktnavigation nennt unter anderem Glaskapillaren, Glasrohre und Glasstäbe. Standortseite nennt **Malsfeld, Körle, Waldkappel, Münnerstadt, Niederlauer**. Keine dort gelesene Zuordnung zu Johannes Hilgenberg/Kiliansweg/Bad Salzuflen.

Diese Seiten tragen die Abgrenzung des Glasunternehmens vom ungeklärten Rohstoff-Lead. Sie beweisen nicht die Identität, Gegenwartsexistenz oder Schließung des Rohstoffhändlers; zwei Seiten derselben Firma sind auch nicht zwei unabhängige Quellen für den Bad-Salzufler Kandidaten. Präziser als „keine Verbindung“ ist **„in den geprüften Seiten keine Verbindung belegt“**. Eine umfassende gesellschaftliche oder historische Nichtverbindung wurde nicht nachgewiesen. Die Register-Live-Eigenschaft der Malsfelder GmbH wurde nicht separat bestätigt; die README-Einzelquellen-Ausnahme wird hier nicht beansprucht.

### 3. Neu gefundener registerbezogener Lead: Johannes Hilgenberg e.K.

URL: https://www.firminform.de/unternehmen/Johannes-Hilgenberg-e-K-Bad-Salzuflen

Direkt erfolgreich gelesen: Name, **Kiliansweg 6 / 32108 Bad Salzuflen**, Register **AG Lemgo HRA 53**, Rechtsform e.K., Status „inaktiv“, „Profil wurde zuletzt am 04.10.2017 aktualisiert“. Keine Tätigkeitsbeschreibung, keine aktuelle Meldung und kein Registerdokument im gelesenen Profil. Anbieter laut Seitenfuß Validatis GmbH.

Bewertung: Übereinstimmung von Name und genauer Anschrift macht die Spur sachlich relevant und unterscheidet sie vom Malsfelder Glas-Namensvetter. Noch offen sind die juristisch belegte Identitätsbrücke zu „Johannes Hilgenberg Rohstoffe“, Registerhistorie/Löschungsgrund, mögliche Nachfolge und heutiger Betrieb. firminform wird nicht stillschweigend zu Handelsregister/Northdata/Creditreform oder Betreiberquelle im Sinne der README erklärt. Ein veraltetes Profilmerkmal „inaktiv“ reicht weder für `geschlossen` noch für eine Betreiberkette.

Zusätzliche Suchspur: https://www.webvalid.de/company/Johannes+Hilgenberg+e.K.,+Bad+Salzuflen/HRA+53 wurde in Suchergebnissen mit gleichem Namen/Registerhinweis angezeigt. Direkter Abruf sowie URL-kodierte Variante scheiterten mit **Transport error**. Daher kein daraus gelesener Inhalt, keine unabhängige Doppelbestätigung; Suchsnippet nur Wegweiser.

## Eigenständige Suche und ehrliche Grenzen

- `websearch` für `"Johannes Hilgenberg" "Rohstoffe"` und `"Hilgenberg" "Bad Salzuflen" Handelsregister Kiliansweg`: **HTTP 401 / authentication failed**, keine verwertbaren Suchergebnisse dieses Werkzeugs.
- Google-HTTP-Suchen nach Name/Rohstoffe, Name/Ort/Register und kommunaler Domain: nur Zugriffshinweis, keine auswertbaren Trefferlisten.
- https://www.bing.com/search?q=%22Johannes+Hilgenberg%22+Rohstoffe lieferte überwiegend sachfremde Johannes-Treffer, keinen belastbaren Betreiberbeleg.
- https://html.duckduckgo.com/html/?q=%22Johannes+Hilgenberg%22+Rohstoffe lieferte 11880, Öffnungszeitenbuch, ein weiteres Firmenportal und einen personenbezogenen Namensvetter. Keine davon als Betreiberbeweis übernommen.
- https://html.duckduckgo.com/html/?q=%22Hilgenberg%22+%22Bad+Salzuflen%22+Handelsregister lieferte die oben geprüften firminform-/webvalid-Spuren.
- https://html.duckduckgo.com/html/?q=%22Hilgenberg%22+%22Kiliansweg%22 zeigte „No results found“. Das ist nur dieser Suchlauf, ausdrücklich keine Widerlegung der direkt lesbaren Portal-/Profilanschrift.
- Weitere DuckDuckGo-Abfragen zur Telefonnummer und zur kommunalen Domain lieferten eine Bot-Challenge; keine negative Sachauskunft.
- https://www.northdata.de/?name=Johannes+Hilgenberg+Bad+Salzuflen lieferte nur eine allgemeine Suchoberfläche. Ein gezielter Pfadversuch https://www.northdata.de/Johannes%20Hilgenberg%20e.K.,%20Bad%20Salzuflen/Amtsgericht%20Lemgo%20HRA%2053 führte zu einer Suche nach dem Firmennamen mit „Keine Resultate zu dieser Suchanfrage“, **nicht** zu einem Registerprofil. Daraus kein Beweis, HRA 53 existiere nicht.
- https://www.handelsregister.de/ erfolgreich als Startseite mit Suchnavigation und im extrahierten Inhalt auch Session-/Fehlerhinweisen gelesen. **Keine abgeschlossene Namens-/HRA-Suche und kein aktueller oder chronologischer Auszug abgerufen.** Der neue HRA-Hinweis ist dort noch gezielt zu prüfen; kein amtlicher Negativbefund.
- https://www.stadt-bad-salzuflen.de/ und https://www.stadt-bad-salzuflen.de/serviceportal erfolgreich als allgemeine Stadt-/Serviceoberflächen gelesen, aber kein kandidatenbezogener Gewerberegisterauszug. Kein Kontakt zur Verwaltung, keine kostenpflichtige Auskunft veranlasst. Die kommunale Prüflücke bleibt offen.

**Gesamtergebnis der Suche:** Kein eigener Betreiberbeleg und kein erfolgreich abgerufener amtlicher Registerauszug für den Bad-Salzufler Kandidaten in diesem Review; zugleich ein konkreter neuer Registeransatz. Weder Nichtexistenz noch Schließung oder aktueller Ankauf lassen sich aus den Such-/Abrufgrenzen ableiten.

## Ankauf, Verkauf und Gebühren getrennt

| Bereich | Belastbarer Stand dieses Reviews |
| --- | --- |
| Ankauf | Heutiger Ankauf/Annahme nicht durch Betreiber belegt; keine zuordenbare veröffentlichte Ankaufspreisliste in den geprüften Kandidatenquellen. Portal-Branchenkategorie ist keine Annahmebestätigung. |
| Verkauf | Keine dem Kandidaten belastbar zuordenbare Verkaufspreisliste geprüft/gefunden. Glasprodukte des Malsfelder Namensvetters nicht übertragen. |
| Gebühren | Keine dem Kandidaten zuordenbare Gebühren-/Entgeltliste in den geprüften Quellen. Unbekannt bedeutet weder kostenlos noch gebührenpflichtig. |

## Vor Integration notwendige Präzisierungen

1. „ohne … Register- oder kommunalen Zweitbeleg“ als **„bislang kein erfolgreich abgerufener zulässiger Register-/kommunaler Beleg; neu gefundener Prüfhinweis AG Lemgo HRA 53 (firminform, Profilstand 04.10.2017, dort inaktiv), amtliche Identitäts-/Statusprüfung offen“** dokumentieren. Kein Schema-Fill und keine Schließung aus diesem Profil.
2. „keine Verbindung zum Bad-Salzufler Schrotthandel“ auf **„keine Verbindung in den geprüften Betreiberseiten belegt“** begrenzen.
3. „keine Ankaufspreise; Verkauf und Gebühren: keine Liste“ ausdrücklich auf **die geprüften, zuordenbaren Quellen** beziehen; keine universelle Aussage über das Vorhandensein von Listen.
4. Den historischen Abschnitt vom 13.10. nicht so umschreiben, als seien neue Erkenntnisse damals erhoben worden. Eine datierte Ergänzung/Korrektur vom **15.10.2026** ist der saubere Ort für diesen Review, das dynamische Portaldatum und die Abrufgrenzen. Der ursprüngliche Abruf am 13.10. bleibt übernommene, nicht unabhängig historisch verifizierte Rechercheangabe.

Damit ist die **konservative Zurückhaltung in den Stammdaten freigabefähig**, die aktuelle Quellenbeschreibung aber ergänzungsbedürftig. Eine spätere Aufwertung erfordert die README-Evidenzbasis und die Klärung der neuen Registerspur; ein erneutes Suchfehlen genügt nicht. Diese Präzisierungen sind Empfehlungen an den integrierenden Owner und wurden hier nicht am Dossier vorgenommen.
