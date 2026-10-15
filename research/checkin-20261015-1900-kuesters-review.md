# Küsters Selfkant – unabhängiger Quellen- und Identitätsreview

Recherchetag: **15.10.2026**. Gegenstand: `dossiers/nw/nw-selfkant-kusters.md`; Prüfmaßstab: README, Abschnitte „Feedback triage“ und „Operator rename/merge chains“. Der Dateisuffix `1900` bezeichnet den Check-in, keine behauptete sekundengenaue Abrufzeit.

## Integrationsentscheidung

**Kein Stammdaten-Fill, keine Aktivierung, keine Preisintegration. `status: pruefung` beibehalten.** Hochstr. 36 in Selfkant-Schalbruch ist ein konkreter und inzwischen besser verknüpfter Recherchelead, aber weiterhin keine nach README belegte Betriebsadresse. Arndt Küsters ist ein plausibler Kandidat, nicht eine abschließend identifizierte Betreiberperson des Seeds. Die Werkstatt in Höngen darf nicht übernommen werden.

Für den Owner sind ausschließlich eine differenziertere Recherche-Timeline und die Korrektur der Datumsinterpretation des 11880-Eintrags integrierbar. Alle strukturierten Kontakt-, Adress-, Website-, GEO-, Anliefer-/Abhol- und Preisfelder bleiben unverändert leer; `website_status: unbekannt` bleibt sachgerecht. Kein Schließungsnachweis, daher auch kein `geschlossen`. Slug unverändert lassen, keine Zusammenführung und keinen neuen Händler allein aus diesen Leads erzeugen.

## Maßstab und Belegzählung

- Aggregatoren zählen nach README **nur als Leads**, auch bei übereinstimmenden Adressen und Telefonnummern. Zehn Verzeichniseinträge ergeben keine zwei zulässigen Belege.
- Betreiberwebsite, Register/Northdata/Creditreform, kommunales Gewerberegister und verifiziert zuordenbare Betreiber-Social-Profile sind mögliche Belegklassen. Normalfall: zwei unabhängige Quellen. Mehrere Unterseiten desselben Betreibers sind nicht unabhängig.
- Die Ausnahme für eine autoritative Betreiberwebsite setzt u. a. Registeridentität, lebenden Registerstatus und aktuelle standortbezogene Inhalte voraus. Für den gesuchten Schrotthändler ist sie nicht erfüllt. Auch die Werkstattwebsite erfüllt hier keinen belegten Registerabgleich und darf schon wegen der anderen Identität nicht für den Seed benutzt werden.
- Für eine Umbenennungs-, Nachfolge-, Umzugs- oder Verschmelzungskette ist eine eigenständige Identitätsprüfung erforderlich. **Keine solche Kette gefunden oder bewiesen.** Insbesondere kein Nachweis „Arndt Küsters → Rene Ringel“ oder „Hochstraße → Heerstraße“.

## Tatsächlich abgerufene Quellen und Befunde

Alle folgenden Abrufe erfolgten in diesem Review am 15.10.2026. `webfetch` lieferte die Inhalte; wo die Markdown-Ausgabe leer blieb, wurde die tatsächlich empfangene HTML-Antwort lokal **nur lesend** auf relevante Felder geprüft. Suchtreffer sind unten ausdrücklich von Seitenbefunden getrennt. HTTP-Statuscodes wurden vom Werkzeug nicht ausgewiesen; daraus werden keine erfundenen 200-/403-Befunde abgeleitet.

### 1. Hochstraßen-Lead: konkreter, aber kein zulässiger Betriebsbeleg

**[11880: Küsters Schrotthandel](https://www.11880.com/branchenbuch/selfkant/120674719B41808320/kuesters-schrotthandel.html)** – vollständig als Text abgerufen:

- Name „Küsters Schrotthandel“, Branche Schrotthandel, **Hochstr. 36, 52538 Selfkant (Schalbruch)**.
- Festnetz **02456 508323**, zusätzlich unter „Weitere Kontaktdaten“ Mobil **0171 6913136**.
- „E-Mail-Adresse hinzufügen“, „Website hinzufügen“; kein Betreiber-Impressum oder Registerbezug des gelisteten Händlers.
- Angezeigt: „aktualisiert am 03.10.2026“, „Eintragsdaten vom 25.09.2026“. Die beiden angezeigten „top service“-Bewertungen tragen **22. August 2016** und werden von GoLocal übernommen; die Seite weist auf ungeprüfte Bewertungen hin.
- **Korrektur zur bisherigen Dossiernotiz:** Das im Abruf sichtbare Jahr 2016 gehört zu Bewertungen, nicht zu einem nachgewiesenen Gründungs- oder Eintragsdatum. Auch die Aktualisierungszeile 2026 belegt keine aktuelle Betriebsprüfung oder Betreiberbestätigung. Der allgemeine FAQ-Text zum Label „vom Inhaber bestätigt“ ist kein Nachweis, dass gerade dieser Datensatz vom Inhaber bestätigt wurde.

**[branchen-info: Arndt Küsters](https://selfkant.branchen-info.net/arndt-kuesters/2109254)** – als Text abgerufen:

- „Arndt Küsters“, Branche **Schrott**, **Hochstr.36, 52538 Selfkant-Schalbruch**, **0171 6913136**.
- Website, Impressum, E-Mail und Social-Media-Links ausdrücklich nicht hinterlegt; keine Öffnungszeiten und keine Unternehmensbeschreibung.
- Portal weist selbst darauf hin, dass Aktualität und Richtigkeit nicht garantiert werden.

**Auswertung:** Die gleiche Mobilnummer verbindet die beiden Verzeichnisdarstellungen zusätzlich zu Name, Adresse und Branche. Der bisherige Befund „keine Verbindung“ darf deshalb nicht pauschal als „überhaupt kein Verknüpfungsindiz“ verstanden werden. Es gibt nun eine **Aggregatorbrücke Küsters Schrotthandel ↔ Arndt Küsters**, aber weiterhin keine primär belegte Brücke zum Facebook-Konto oder eine belastbare heutige Betreiber-/Standortidentität. Unabhängigkeit der zugrunde liegenden Verzeichnisdaten ist ebenfalls unbekannt.

### 2. Arndt Küsters: echter Social-Abruf mit wichtigen Grenzen

**[Facebook: arndt.kusters](https://www.facebook.com/arndt.kusters)** – Markdown-Ausgabe leer; erneuter HTML-Abruf lieferte Profil-Metadaten und eingebettete Profilfelder:

- Seitentitel: „Arndt Küsters (@arndt.kusters) …“; Profil-ID **100064305488659**.
- Eingebettetes Feld `alternate_name`: **„Selfkant Schrotti“**.
- Arbeitsplatzfeld: **„Works at Schrotthandel Küsters“**, mit Zeitraumtext **„March 1, 2021 - Present“**. Dieser Selbstangabe-Zeitraum ist kein Nachweis einer Gewerbeanmeldung, Betriebsgründung oder aktuellen Ausübung.
- Das Arbeitsplatzfeld verweist auf Facebook-Seite **101718538651692** (siehe Folgeabruf). „Works at“ belegt selbst bei authentischem Profil **nicht automatisch Inhaberschaft**.
- Im untersuchten HTML keine Treffer für `Hochstr`, `508323`, `6913136`, `Schalbruch`, `Alteisen`, `Altauto` oder „Hole alles“. Telefonnummern wurden zusätzlich als vollständige Kandidaten geprüft; bloße Ziffernfragmente in Bild-/Skript-IDs wurden nicht als Kontaktdaten gewertet.
- **[Profil-Unterseite /about](https://www.facebook.com/arndt.kusters/about)** ebenfalls aufgerufen; Werkzeug lieferte keine auswertbare Textausgabe. Daraus weder Nichtvorhandensein des Profils noch Abwesenheit sämtlicher Kontaktdaten behauptet.

**Wichtig: nur Suchindex, nicht im abgerufenen Profiltext reproduziert:** Mehrere `websearch`-Abfragen lieferten für genau die Profil-URL den Text „Hole alles an Schrott oder Alteisen einfach melden auch Altauto Ankauf .Schnell und unkompliziert.“ sowie „Schalbruch · Schrotthandel Küsters“. Der indexierte Text stärkt den Kandidaten und nennt offenbar einen lokalen Bezug, ist aber weder aktuell direkt reproduzierter Angebotsinhalt noch eine verifizierte Betreiber-Adressverknüpfung. Der bisherige Dossiertext sollte nicht suggerieren, ein heutiger vollständiger Profilabruf habe diese Ankaufsaussage erneut bestätigt.

**[Verlinkte Facebook-Seite: Schrotthandel Küsters](https://www.facebook.com/Schrotthandel-K%C3%BCsters-101718538651692/)** – aus dem Arbeitsplatzlink verfolgt, HTML tatsächlich abgerufen und geprüft:

- Titel „Schrotthandel Küsters“, ID **101718538651692**.
- Explizite Datenfelder: **`is_owned: false`**, **`category_name: "Interest"`**, **`verification_status: "NOT_VERIFIED"`**; Routing `CometPageUnownedRoute`/`comet.page.unowned`.
- `address: null`, `formatted_phone_number: null`, `website: null`, `email: null`, `description: null`, `price_range: null` im gelieferten Seiten-Datenobjekt.
- **Keine zweite Betreiberquelle:** Der verlinkte Eintrag ist nach diesen Daten gerade keine nachgewiesen betreibergeführte Unternehmensseite. Der Personenprofil-Verweis auf diese Seite erzeugt keinen unabhängigen Identitätsbeleg und keinen Standortnachweis. Leere Felder gelten nur für die ausgelieferte Antwort, nicht als universelle Negativauskunft.

**Ergebnis Social:** Selbstbeschreibung und Arbeitsplatzlink sind echte neue Primärindizien für einen Schrottbezug im Selfkant. Authentizität, Betreiberrolle, vollständige Adresse und Verbindung zu den Verzeichnisnummern sind nicht verifiziert. Damit weder Zwei-Quellen-Standard noch autoritative Betreiber-Ausnahme erfüllt.

### 3. Werkstatt-Namensvetter: Betreiberwebsite zusätzlich geprüft

**[Eigene Website KFZ-Service Küsters](http://www.kfzservice-kuesters.de)** – abgerufen: freie Werkstatt „in Selfkant-Höngen“, **Inhaber und KFZ-Meister Rene Ringel**, Wartung/Reparatur, HU/AU, Motordiagnose, Unfallinstandsetzung, Inspektionen, Klima-, Bremsen-, Autoglas-, Reifenservice und Achsvermessung.

- **[Eigene Kontaktseite](http://www.kfzservice-kuesters.de/pages/anfahrt.html)**: „KFZ-Service Küsters, Inh. Rene Ringel“, **Heerstr. 4a, 52538 Selfkant**, `info@kfzservice-kuesters.de`.
- **[Eigene Datenschutzseite](http://www.kfzservice-kuesters.de/pages/datenschutz.html)**: Verantwortlicher ebenfalls „KFZ Service Küsters, Inh. Rene Ringel“, Heerstr. 4a. Enthält alte Mustertexte/Generatorversion 2018-06-22; kein belastbarer Nachweis aktueller Registerkonformität.
- **[Verlinktes Impressum](http://www.kfzservice-kuesters.de/pages/impressum.html)**: tatsächlich abgerufen, im gelieferten Text nur Haftungsausschluss/Urheberrechtsbausteine; **keine auswertbare Registeridentität**. Nicht als vollständiges Identitätsimpressum ausgegeben.

Zusätzlich alle drei vorhandenen Garage4Tires-Standortseiten einzeln erneut gelesen:

1. **[Start](https://www.garage4tires.com/en/kfz-service-kuesters)**: Werkstatt, Rene Ringel, Heerstraße 4a, Selfkant-Höngen.
2. **[Kontakt](https://www.garage4tires.com/en/kfz-service-kuesters/kontakt)**: Heerstraße 4a, **02456-2381**, gleiche E-Mail.
3. **[Leistungen](https://www.garage4tires.com/en/kfz-service-kuesters/leistungen)**: Reparatur/Wartung und Reifenbeschaffung/-montage/-einlagerung; kein auswertbares Angebot zum Schrott- oder Altautoankauf, keine Schrottankaufsliste.

Die eigene Kontaktseite und Garage4Tires nennen **abweichende Öffnungszeiten** (eigene Seite Mo.–Do. 8–12/12:30–17:30, Fr. bis 14 Uhr; Garage4Tires Mo.–Fr. 8–12/13–17 Uhr). Nicht auflösen oder übertragen. Es ist nicht geklärt, ob Partnerseite und eigene Website redaktionell unabhängige Quellen sind; sie werden nicht künstlich als zwei unabhängige Register-/Identitätsbelege gezählt.

**Entscheidung:** Als Abgrenzung belastbar: Die gefundene Website beschreibt eine konkret anders adressierte Werkstatt mit anderem benannten Verantwortlichen, nicht den gesuchten Hochstraßen-Schrotthändler. Daraus folgt kein Beweis, dass die Werkstatt niemals Schrott annimmt; ebenso wenig folgt ein Beweis für eine gemeinsame Familie, frühere Identität, Rechtsnachfolge oder Verlagerung. Website, Öffnungszeiten, Telefonnummer, E-Mail, Adresse und Reifenverkauf gehören **nicht** in `nw-selfkant-kusters`.

## Ankauf, Verkauf, Gebühren: strikt getrennt

| Kategorie | Tatsächlicher Befund | Integrationsfolge |
|---|---|---|
| **Ankauf** | Indexierter Facebook-Satz nennt Abholung von Schrott/Alteisen und ausdrücklich „Altauto Ankauf“. Im eigenen HTML-Abruf keine reproduzierte Angebotspassage, keine zugeordnete Betriebsadresse, kein Betrag, keine Material-/Mengen-/Qualitätsstaffel. | Kein bestätigter Ankauf für den Seed; keine Preise, keine Materialannahmen oder Pickup-Regeln übernehmen. „Hole alles“ bedeutet nicht Ankauf jedes Materials gegen Geld. |
| **Verkauf** | Keine zuordenbare Schrott-/Metallverkaufsliste oder Verkaufspreise gefunden. Reifenbeschaffung auf der Werkstattseite betrifft den anderen Betrieb. | Keine Verkaufspreise und kein Reifenangebot übertragen. |
| **Gebühren** | Keine zuordenbaren Abhol-, Anliefer-, Entsorgungs-, Wiege- oder sonstigen Gebühren gefunden. Keine explizite kostenlose Abholung. | Unbekannt bleibt unbekannt; weder 0 EUR noch „kostenlos“ oder Gebührenpflicht setzen. |

Keine numerische Preisbeobachtung, keine Einheit/Währung, kein belegter Veröffentlichungstag einer Preisliste. Daher auch kein Handler-Kandidat aus dem vorliegenden Material.

## Suchläufe und Grenzen

Eigenständige `websearch`-Abfragen: `"Küsters" "Schrotthandel" "Selfkant"`, `"Arndt Küsters" Schrott`, `"Arndt Küsters" "Selfkant" Register`, `"Küsters" "Hochstr" "Schrott"`, `site:northdata.de "Küsters" "Selfkant"`, `"Arndt Küsters" Schrotthandel -site:branchen-info.net -site:geni.com -site:andritz.com`, `"01716913136" Küsters`, `"Arndt Küsters" site:creditreform.de OR site:handelsregister.de OR site:northdata.de`, `"101718538651692"` und `"Schrotthandel Küsters" "Arndt" Facebook`.

- Northdata-bezogene Suche lieferte andere Personen Küsters in Selfkant (u. a. Gerard/Björn), aber keinen zuordenbaren Schrotthändler Arndt. Andere Register-/Telefon-Suchen lieferten irrelevante Ergebnisse. **Das ist nur Suchindex-Nichtfund, keine durchgeführte Handelsregister-Negativauskunft.** Keine aktuelle amtliche Registerakte, Gewerbeanzeige, abfallrechtliche Anzeige oder EFB-Zertifizierung zu Arndt eingesehen. Ein Einzelgewerbe kann ohne Handelsregistereintrag bestehen.
- Weitere Verzeichnisse, lokale Presse und Schrottportale erschienen als Suchtreffer, wurden hier nicht als zusätzliche Seitenbelege ausgegeben. Die bisherige Schrottregister.pages.dev-Behauptung wurde nicht neu abgerufen und ist keine Grundlage der Entscheidung.
- Keine Telefon-/E-Mail-Kontaktaufnahme; keine Gewerberegisterauskunft beantragt; keine authentifizierte Facebook-Sitzung und keine belastbare Aktualitätsprüfung des ganzen Profils. Keine Aussage über Nichtexistenz, Schließung oder Rechtmäßigkeit des Unternehmens.
- Eine zusätzliche unabhängige Agentenrecherche ließ sich wegen der vom Harness gemeldeten Subagent-Tiefengrenze nicht starten. Sämtliche hier dokumentierten tatsächlichen Abrufe wurden deshalb in diesem Review selbst ausgeführt; kein zweiter Agentenbefund wird behauptet.

## Nächster belastbarer Owner-Schritt

1. Die Telefonnummern aus den Verzeichnissen höchstens zur Kontaktaufnahme verwenden, nicht als bereits verifizierte öffentliche Stammdaten. Betreiber um bestätigenden schriftlichen Nachweis bitten: vollständiger Gewerbename/Inhaber, aktuelle Betriebs-/Korrespondenzadresse, eigene Website/Social-Zuordnung, Verbindung zu Arndt und Hochstr. 36 sowie heutiges Ankaufs-/Abholangebot.
2. Unabhängigen zulässigen Beleg beschaffen, etwa kommunale Gewerberegisterauskunft oder passende Register-/Creditreform-Auskunft. Nicht pauschal ein HRB bei möglichem Einzelgewerbe verlangen. Eine bloße telefonische Bestätigung erfüllt die im README aufgezählten Belegklassen nicht automatisch.
3. Erst dann Identität und heutige Standortrolle entscheiden: Hof mit Anlieferung, reiner mobiler Sammler oder andere Betriebsform. Eine Wohn-/Kontaktadresse darf nicht ohne Anlieferbeleg als Schrottplatz beworben werden. Bei tatsächlicher Nachfolge getrennten Kettennachweis führen.
4. Falls Owner die Timeline ergänzt: neue Aggregatorbrücke und echte Social-Felder dokumentieren; 2016 ausdrücklich als Bewertungsdatum präzisieren; historische Recherche nicht überschreiben. Kein Stammdaten-Fill aus diesem Review.

## Arbeitsgrenze

Geschrieben wurde ausschließlich diese Recherchenotiz. Dossier unverändert; keine Git-Befehle, keine Tests, keine Veröffentlichung und keine Datenbankzugriffe (auch keine Produktionsabfragen). Dossierintegration, Tests und Veröffentlichung verbleiben vollständig beim Owner.
