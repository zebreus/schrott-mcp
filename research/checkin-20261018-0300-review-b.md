# Check-in 18.10.2026 03:00 – Review B

## Umfang und Ergebnis

Genau die vier Änderungen aus `shards.b` in `/tmp/opencode/checkin-20261018-0300-review-scope.json` geprüft: vollständige Arbeitsdateien, vollständige HEAD-Fassungen und vollständige Diffs gegen `bea932f501637e0c67d5da6e9175b8d13a8f6480`. HEAD und alle vier SHA-256 stimmen mit dem Scope überein. Maßstab: README, insbesondere Quellenhierarchie, Identitätsketten, Historienerhalt und Owner-Gate (Zeilen 160–240).

**Vier bedingte Dispositionen: lokal integrierbar erst nach den unten genannten Textkorrekturen; keine unveränderte Freigabe der behaupteten historischen Rechercheereignisse.** Keine Frontmatter-Änderung in diesen Diffs; alle vier Statuswerte bleiben `pruefung`, Slugs und frühere Timeline-Einträge bleiben vollständig erhalten. Nur SMS ersetzt zusätzlich den bisherigen Überblick-Platzhalter. Es wird weder ein Altfeld noch die Existenz/Aktivität des jeweiligen Seed-Betriebs neu bestätigt. Keine Dossieränderungen, Gitmutationen, Produktionseingriffe, GEO-Abfragen oder Builds vorgenommen.

## Direktprüfung am 18.10.2026 und Beleggrenzen

- `https://www.smsrecycling.de/`: Selbstdarstellung „SMS Metall Recycling in Scheeßel“, Schrotthandel/Container/Demontage, allgemeine „faire Preise“; sichtbare Kontaktangaben 017668232814, e.schmelzer@smsrecycling.de und In den Wiesenhöfen 34, Scheeßel, 27383. Keine numerischen Ankauf-/Verkaufspreise oder Gebühren im gelesenen Inhalt. Auffällig: Startseitenlinks `tel:0000000` und `mailto:email@example.com` widersprechen den sichtbaren Texten; deshalb keine Kontaktfunktionsprüfung behaupten.
- `https://www.smsrecycling.de/impressum/`: SMS Metall Recycling, dieselbe Anschrift, Telefonnummer und E-Mail, USt-ID DE301515073. **Kein ausdrücklich genannter Personenname „E. Schmelzer“**, sondern nur der E-Mail-Lokalteil `e.schmelzer`; keine HR-Nummer oder Boizenburg-Kette. Zwei Seiten derselben Domain sind keine zwei unabhängigen Quellen. Die README-Ausnahme für verifizierte autoritative Betreiberquellen ist damit nicht nachgewiesen; die Seite darf hier lediglich als Scheeßeler Identitätslead dienen.
- `http://wegner-schrotthandel.regionale-onlinepräsenz.de/`: nennt Stefan Wegner, Müggenhof 1, 17449 Peenemünde, 038371-20757, Mobilnummer und Termine nach Vereinbarung; bietet im Text Abholung/Anlieferung an. Im gelesenen Inhalt nur Portal-Kontakt/Urheberhinweis Regionale-Onlinepräsenz, kein Wegner zugeordnetes Impressum. Keine numerische Preis-/Gebührenliste auf dieser Seite. Das ist ein Portal-Lead, kein unabhängig verifizierter Betreiberbeleg.
- `https://www.handelsregister.de/rp_web/welcome.xhtml`: generische Portalstartseite mit Suchnavigation, Verfügbarkeits- und Sitzungs-/Fehlerhinweisen. Kein unternehmensbezogener Auszug oder dokumentiertes Suchergebnis. Der Abruf bestätigt **keinen negativen Registersuchlauf** zu Holtzmann oder Friehold und keine Nichtexistenz. In diesem Review wurde keine firmenspezifische Registersuche durchgeführt.

Die Direktabrufe vom 18.10. beweisen keine Abrufe oder Suchvorgänge am 12.10. Für diese historischen Recherchebehauptungen lag im begrenzten Prüfmaterial kein zeitnahes Abrufprotokoll vor. Frühere Timeline-Berichte werden als überlieferte Recherchehistorie erhalten, nicht rückwirkend verifiziert. Breite Negativaussagen zu Preisen sind auf tatsächlich gelesene Seiten bzw. den vorliegenden Dossierbestand zu begrenzen.

## 1. Holtzmann – bedingt annehmen, Suchbehauptung präzisieren

Datei: `dossiers/hh/hh-lurup-altona-hans-christian-holtzmann.md`, neuer Block Zeilen 33–35.

**Disposition: ANNEHMEN NACH TEXTKORREKTUR.** Die Grenze ist sachlich richtig: Portal bleibt Lead; keine PLZ-/Telefonfüllung, kein Aktiv- oder Schließungsurteil. Lüdersring 39 und der Name bleiben unbestätigte Seed-Angaben. Die allgemeine Registerstartseite trägt aber nicht „Erneute Suche … ergab …“ als firmenspezifischen Befund. Der Portalabruf vom 07.10. wird hier nicht erneut bestätigt.

Falls kein belastbares Protokoll vom 12.10. beigebracht wird, den ausschließlich neu hinzugefügten Block durch diese dokumentenbezogene Notiz ersetzen:

```markdown
### Belegprüfung 18.10.2026

- [Recherche 18.10.2026: Im vorliegenden Dossierbestand kein qualifizierter Betreiber-/Registerbeleg für Hans-Christian Holtzmann am Lüdersring 39 dokumentiert. Die historisch notierte Portalangabe 22547/040 843444 bleibt ein unbestätigter Lead; keine Kontakt- oder Aktivitätsbestätigung. Keine zuordenbare Betreiberliste für Ankauf, Verkauf oder Gebühren im Dossier belegt. Frontmatter und status pruefung unverändert. Der Direktabruf der allgemeinen Registerstartseite ist kein firmenspezifischer Suchbefund; ein erneuter Suchlauf am 12.10. wurde in dieser Prüfung nicht verifiziert. Quelle(n): Dossierhistorie 07.10.2026; https://www.handelsregister.de/rp_web/welcome.xhtml (nur Startseite, Abruf 18.10.2026)]
```

## 2. SMS Boizenburg – bedingt annehmen, Personenname und Chronologie korrigieren

Datei: `dossiers/mv/mv-boizenburg-19258-sms-metall-recycling-e-schmelzer.md`, neuer Überblick Zeile 27 und neuer Block Zeilen 31–33.

**Disposition: ANNEHMEN NACH TEXTKORREKTUR.** Standort-/Domaintrennung und ausbleibende Feldübernahme sind richtig. Insbesondere keine Verlagerung nach Scheeßel und keine Gleichsetzung mit `sms-recycling.de`. Zu stark ist dagegen „Website nennt E. Schmelzer“: Der direkt gelesene Auftritt nennt nur die E-Mail-Adresse. Auch „derselbe … Initial“ ist daher kein ausdrücklich bestätigter Personenvergleich. Die ältere Domain ist nur als historisch ausgeschlossener Lead dokumentiert, nicht hier neu als eigenständiger Betreiber validiert.

Vorgeschlagener Ersatz für den neuen Überblick:

```markdown
Ein am 18.10.2026 direkt gelesener Auftritt unter smsrecycling.de bezeichnet sich als SMS Metall Recycling in Scheeßel und nennt In den Wiesenhöfen 34, 27383 Scheeßel, 0176 68232814 sowie e.schmelzer@smsrecycling.de. Der Betriebsname und der E-Mail-Lokalteil sind ein Identitätslead zur unbestätigten Boizenburger Seed-Spur, aber kein ausdrücklich genannter Personenname und kein Nachweis für Umzug, Filiale oder Rechtsnachfolge. In Startseite und Impressum ist keine Boizenburg-Verbindung dokumentiert. Keine Website-/Kontakt-/Adressübernahme; status pruefung bleibt. Die dort genannten Leistungen und allgemeinen „fairen Preise“ werden Boizenburg nicht zugerechnet; auf den beiden gelesenen Seiten keine numerischen Ankauf-/Verkaufspreise oder Gebühren. Die andere Domain sms-recycling.de bleibt entsprechend der historischen Notiz getrennt; deren damalige Zuordnung wurde hier nicht nachgeprüft.
```

Falls kein Abrufprotokoll vom 12.10. vorliegt, Ersatz des ausschließlich neuen Timeline-Blocks:

```markdown
### Recherche 18.10.2026 (Scheeßeler Identitätslead)

- [Recherche 18.10.2026: Startseite und Impressum von smsrecycling.de direkt gelesen: Selbstauskunft SMS Metall Recycling, In den Wiesenhöfen 34, 27383 Scheeßel, 0176 68232814, e.schmelzer@smsrecycling.de, USt-ID DE301515073. Ein Personenname E. Schmelzer wird nicht ausdrücklich genannt; der E-Mail-Lokalteil ist nur ein Lead. Keine Boizenburg-Verbindung oder Register-/Umzugskette auf diesen Seiten belegt. Keine numerischen Ankauf-/Verkaufspreise oder Gebühren auf den beiden gelesenen Seiten. Die sichtbaren Startseitenkontaktdaten haben abweichende technische Linkziele; Erreichbarkeit nicht geprüft. Status pruefung und alle Frontmatter-Werte unverändert; keine Standortzuordnung oder Geocodierung. Historischer Abruf am 12.10. in dieser Prüfung nicht verifiziert. Quelle(n): https://www.smsrecycling.de/ ; https://www.smsrecycling.de/impressum/ ; Abruf 18.10.2026]
```

## 3. Friehold – bedingt annehmen, Register-Negativbefund zurücknehmen

Datei: `dossiers/mv/mv-carinerland-18236-friehold-recycling.md`, neuer Block Zeilen 31–33.

**Disposition: ANNEHMEN NACH TEXTKORREKTUR.** Kein falscher Betreiber aus ähnlich klingenden Treffern übernommen; Kastanienallee 12 bleibt ungeprüft. Nichtexistenz wird ausdrücklich nicht behauptet. Die allgemeine Register-URL und die überlieferte Northdata-Abfrage belegen allerdings weder einen aktuellen exakten Suchlauf noch den behaupteten erneuten Primärregisterversuch. Northdata-Fehlgriff und frühere Rechercheberichte unverändert stehen lassen; damit keine Bestätigung dieser historischen Abrufe verbinden.

Ohne Protokoll vom 12.10. ausschließlich neuen Block ersetzen:

```markdown
### Belegprüfung 18.10.2026

- [Recherche 18.10.2026: Im vorliegenden Dossierbestand kein eindeutig zuordenbarer Betreiber-/Registerbeleg für Friehold Recycling GmbH dokumentiert. Seed-Adresse Kastanienallee 12 bleibt unbestätigt; keine neue Standort- oder GEO-Zuordnung. Für Ankaufstätigkeit, Ankaufskurse, Verkaufspreise oder Gebühren ist im Dossier kein qualifizierter Beleg dokumentiert. Frühere negative Suchberichte sind kein Nichtexistenzbeweis und wurden hier nicht erneut ausgeführt. Die direkt gelesene allgemeine Registerstartseite enthält keinen firmenspezifischen Suchbefund; ein erneuter Suchlauf am 12.10. ist durch dieses Review nicht bestätigt. Status pruefung und Frontmatter unverändert. Quelle(n): Dossierhistorie 02./07.10.2026; https://www.handelsregister.de/rp_web/welcome.xhtml (nur Startseite, Abruf 18.10.2026)]
```

## 4. Wegner – bedingt annehmen, Abrufdatum und Suchumfang begrenzen

Datei: `dossiers/mv/mv-peenemunde-17449-wegner-stefan-schrotthandel-abriss-entke.md`, neuer Block Zeilen 31–33.

**Disposition: ANNEHMEN NACH TEXTKORREKTUR.** Portal-/Betreibergrenze stimmt mit dem heutigen Direktabruf überein. Die früher als Ausnahme übernommenen Adresse/Telefon bleiben historische Werte mit ausdrücklich offener Beleglage, nicht neue Primärbestätigung. Die alte automatische GEO-Aussage wird nicht als tatsächlich vollzogener Vorgang bestätigt. Unbelegt bleibt ein historischer Abruf vom 12.10.; die Portalprüfung ist zudem keine allgemeine Registerrecherche. Das Örtliche wurde in diesem Review nicht erneut geöffnet.

Ohne Protokoll vom 12.10. ausschließlich neuen Block ersetzen:

```markdown
### Recherche 18.10.2026

- [Recherche 18.10.2026: Regionale-Onlinepräsenz direkt gelesen: Portal-Mikroseite nennt Stefan Wegner, Müggenhof 1, 17449 Peenemünde, 038371-20757 sowie Termine und Abholung/Anlieferung; im gelesenen Inhalt nur Portal-Kontakt/Urheberhinweis, kein Stefan Wegner zugeordnetes Impressum. Die Angaben bleiben unbestätigte Leads, keine verbindlichen Servicebedingungen. Keine numerischen Ankaufspreise, Materialverkaufspreise oder Gebühren auf dieser Seite. Kein firmenspezifischer Registerbefund erhoben und kein historischer Abruf am 12.10. verifiziert. Vorhandene Frontmatter-Werte und status pruefung unverändert; keine GEO-Aktion, frühere automatische Geocodierung nicht geprüft. Quelle: http://wegner-schrotthandel.regionale-onlinepräsenz.de/ ; Abruf 18.10.2026]
```

## Integrationsgrenze

Die Ersatztexte sind Vorschläge für die Owner-Integration, hier **nicht angewandt**. Sie betreffen nur die neu hinzugefügten, noch nicht integrierten Absätze; alle bereits in HEAD vorhandenen historischen Einträge bleiben unverändert. Alternativ kann der Owner zeitnahe Abrufbelege vom 12.10. vorlegen und die damaligen Notizen quellengetreu belassen; die SMS-Personennamenaussage muss unabhängig davon korrigiert werden. Für die vier Dossiers werden damit nur Beleggrenzen und ein Scheeßeler Lead lokal dokumentiert, keine Identitätskette, Betriebsbestätigung, Altfeldvalidierung, Preisquelle, Produktion oder Seed-/Build-Freigabe.
