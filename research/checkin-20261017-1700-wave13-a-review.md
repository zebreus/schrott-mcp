# Owner-Review Wave 13-A – 17.10.2026, Check-in 17:00

## Ergebnis und Prüfrahmen

**Keine pauschale Integrationsfreigabe.** Alle 20 tatsächlichen Dossier-Diffs wurden gegen HEAD und die jeweilige Historie geprüft. Die meisten Änderungen sind vorsichtige Klärfall-Dokumentation, keine neu belegten Händlerdaten. Besonders wichtig vor einer späteren Übernahme: WSH-Negativbefund korrigieren, RWR-EUID richtig einordnen und Eftekhari-Registerwechsel präzisieren. Bereits integrierte Preis- und Identitätsabgrenzungen nicht als neue Funde ausgeben.

- Referenz: `research/wave-20261013-a.md`; Git-Basis `4b5a0f3b9a1e83561d9800c48726effb28dd4719`.
- Gelesen: `README.md`, alle 20 Arbeitsbaum-Diffs (`git diff HEAD -- <Pfad>`), jeweiliger HEAD-Inhalt (`git show HEAD:<Pfad>`) und Dateihistorie (`git log -- <Pfad>`).
- Maßstab: README, insbesondere Zeilen 183–202 und 220–240. Aggregatoren sind Leads; zwei solche Seiten ergeben keinen Doppelbeleg. Mehrere Unterseiten derselben Betreiberwebsite sind eine Quelle. Die Ausnahme für autoritative Betreiberseiten setzt alle Anerkennungskriterien voraus; ein Impressum oder eine USt-ID allein genügt nicht. Identitäts-/Umzugsketten brauchen weiterhin unabhängige Absicherung.
- Eigene Webabrufe und Suchgegenproben am **17.10.2026**; historische Abrufbehauptungen vom 13.10. sind damit nicht nachträglich bewiesen. Transportfehler, leere Social-Abrufe und 403 werden ausdrücklich nicht als Nichtexistenz gewertet.
- Ausschließlich diese Review-Datei geschrieben. Keine Dossieränderungen, kein Commit/Push, kein Produktionszugriff, keine GEO-Recherche oder Koordinatenbearbeitung. Historische GEO-Aussagen werden hier weder neu vermessen noch als aktuell verifiziert bestätigt.
- Ein durch die Research-Skill angeregter zusätzlicher Read-only-Rechercheagent konnte wegen der Subagent-Tiefengrenze nicht starten; sämtliche folgenden Befunde stammen aus eigener Prüfung.

### Tatsächlicher Frontmatter-Umfang

Es gibt **keine neuen Adress-, Telefon-, Website-URL-, Preis- oder Leistungs-Fills** in diesen 20 Diffs. Änderungen:

- `status: aktiv → pruefung`: **Gramet und Wolf**.
- `website_status: '' → unbekannt`: **Storkow, Wolf, Albert, Bach, Kreutz, Maik, Breyer, Eftekhari, Wenner, Trier-West und RWR** (11).
- Die übrigen acht Frontmatter bleiben unverändert (Wolf hat beide Arten von Änderung). Die Normalisierungen sind redaktionelle Zustandspräzisierungen, kein neuer Existenzbeleg; eine technische Ungültigkeit des alten Leerwerts wird hier nicht behauptet.
- Alle übrigen Änderungen betreffen Überblick/Timeline. Auch Timeline-Text ist laut README Build-Input für `notes`; unbelegte Sicherheit im Fließtext ist deshalb nicht folgenlos.

## Entscheidungen je Dossier

Pfadkonvention unten: `dossiers/<erste zwei Slugzeichen>/<slug>.md`. „Freigabe“ meint nur eine begründete Empfehlung für spätere Integration, keine hier ausgeführte Übernahme und keine Produktionsfreigabe.

### 1. `bb-eberswalde-seik-automobilrecycling`

**Entscheidung: bedingt freigeben, nur als Klärfall-Überblick; keinen neuen Herten-Fakt behaupten.**

- **Diff/Historie:** Nur Überblick und 13.10.-Timeline neu. `website_status: unbekannt`, Nichtzuordnung von seik.de und Herten-Betriebsende stehen bereits in HEAD, zuletzt `ae397aa3` (03.10.).
- **Eigene Gegenprobe:** Suche `"SEIK" "Eberswalde"` findet ausschließlich Verzeichnisleads: [Onlinestreet](https://onlinestreet.de/poi/ZGVBZE9PbnViaC9lN05jSVZuVW0rUT09) nennt Nr. 1, [Stadtbranchenbuch](https://eberswalde.stadtbranchenbuch.com/1738216.html) Nr. 1A. Das bestätigt einen Lead-Konflikt, keine richtige Hausnummer.
- **Beleggrenze/Korrektur:** Eigene Direktabrufe von [Unternehmen](https://seik.de/unternehmen), [Impressum](https://seik.de/impressum) und [News](https://seik.de/news) scheitern mit Transportfehler. Herten-Schließung zum 30.06.2026 deshalb ausdrücklich als **bereits am 03.10. dokumentierten historischen Betreiberbefund**, nicht als hier aktuell bestätigte Tatsache formulieren. Eberswalde weder schließen noch mit Herten verknüpfen. Lokale Identität bleibt ungeprüft.

### 2. `bb-gramzow-gramet-schrott-u-metallhandel`

**Entscheidung: Status-Downgrade und historische Registerergänzung freigeben; kein Schließungs-/Identitätstransfer.**

- **Diff/Historie:** `aktiv → pruefung` ist tatsächlich neu. Würzburg-Abgrenzung und unverifizierte Prenzlauer Straße/Telefon sind bereits in `6d713da6` (06.10.) enthalten.
- **Eigener belastbarer Abruf:** [NorthData HRB 10759](https://www.northdata.de/Gramet%20GmbH,%20Gramzow/Amtsgericht%20Neuruppin%20HRB%2010759) nennt Gramet GmbH, Gramzow, Neuruppin HRB 10759, Löschungsankündigung 22.06.2015 und Löschung 15.07.2015. Damit ist der historische Registerbefund im zulässigen README-Quellenkanal nachvollzogen.
- **Beleggrenze/Korrektur:** Keine Verbindung dieses Rechtsträgers zu Szymon Mach/Prenzlauer Str. 3 bewiesen. Die verlinkte [Publikation 108975156](https://www.northdata.de/?id=108975156) ist laut Übersichtsseite eine **Eintragung vom 07.05.2014**, nicht die Löschung; für die Löschung die dort verlinkte [Publikation 1096675191](https://www.northdata.de/?id=1096675191) referenzieren. Aussage über maskierte Straße nicht selbst anhand der Einzelpublikation nachgeprüft. Downgrade wegen fehlender aktueller Belege, nicht wegen einer angeblich identischen gelöschten Firma.

### 3. `bb-lauchhammer-radke-ulrich`

**Entscheidung: keine neue Sachintegration nötig; optional datierter Recheck mit besseren Quellenlinks.**

- **Diff/Historie:** Ausschließlich zusätzliche Timeline. Die entscheidende Herabstufung auf `pruefung` wurde bereits in `0da8d4fa` (07.10.) integriert. Nicht erneut als neue Statusänderung zählen.
- **Eigene Gegenprobe:** Suche `"Radke" "Lauchhammer" Autoverwertung` führt zu [Das Örtliche](https://www.dasoertliche.de/Themen/Radke-Ulrich-Autoverwertung-Lauchhammer-Windm%C3%BChlenstr) und weiteren Telefonbuch-/Portalprofilen mit Windmühlenstr. 21, aber zu keinem belastbar zugeordneten Betreiber-/Registerbeleg.
- **Beleggrenze/Korrektur:** Die im neuen Absatz genannten Root-URLs `gelbeseiten.de` und `11880.com` sind keine reproduzierbaren Detailbelege für Adresse/Telefon. Bei Übernahme auf konkrete Profile oder den ausdrücklich historischen 02.10.-Befund verweisen. Bestehende aggregierte Frontmatter nicht als neu verifiziert deklarieren. Kein aktiver Ankauf oder Gebührenstand belegt.

### 4. `bb-lauchhammer-wsh-schrotthandel`

**Entscheidung: in der vorliegenden Fassung zurückstellen; Negativbefund vor Integration korrigieren.**

- **Diff/Historie:** Nur Überblick/Timeline neu; `pruefung` und Website-Normalisierung bereits `f6185814` (02.10.). Der neue Text stützt sich ausdrücklich auf frühere lokale Suchen ohne Treffer.
- **Eigene neue Gegen-Evidenz:** [11880-Detailprofil](https://www.11880.com/branchenbuch/lauchhammer/060440459B113497545/wsh-schrotthandel.html) ist lesbar: WSH Schrotthandel, Bockwitzer Str. 97, 01979 Lauchhammer, **01511 5603024**, Autoverschrottung/Autoverwertung. Seite nennt Aktualisierung **04.10.2026**, Eintragsdaten 11.04.2026. Damit ist „11880 ohne WSH“ kein geeigneter aktueller Negativbefund; es existiert jetzt ein konkreter lokaler Lead.
- **Primär-Gegenprobe:** [wsh-recycling.de](https://wsh-recycling.de/) nennt Westpfälzische Schrotthandels GmbH, Kaiserstraße 168, Kindsbach. Kein Beleg einer Lauchhammer-Verknüpfung.
- **Korrektur:** Historische erfolglose Suche stehen lassen, aber mit datiertem neuen Portalbefund relativieren. **Keine Telefonnummer übernehmen**, keine Aktivierung: 11880 bleibt Aggregator, Betreiberidentität offen. „Kein Primärbeleg“ bleibt richtig; „kein lokaler Treffer“ nicht als Gegenwartsaussage wiederholen.

### 5. `bb-storkow-utmar-koch-ralf`

**Entscheidung: Enum-Normalisierung und ausdrücklich vorläufigen Überblick freigeben; kein Identitäts-Fill.**

- **Diff/Historie:** Website-Normalisierung und erster Überblick neu; zuvor nur Seed/Importhistorie, zuletzt `d9d551f0` (01.10.).
- **Eigene Gegenprobe:** Suche `"Utmar" "Storkow"` liefert [11880-Suche](https://www.11880.com/suche/altmaterialhandel/storkow) mit „Ralf Utmar-Koch“, Robert-Koch-Str. 24, 033678 60698; [Straßenverzeichnis Das Örtliche](https://www.dasoertliche.de/Themen/Robert--Koch--Str/Storkow-Mark.htm) nennt „Koch Utmar“ an derselben Adresse. Abweichende Namensformen lösen die Identität gerade nicht.
- **Beleggrenze/Korrektur:** [Facebook „Utmar Koch“](https://www.facebook.com/p/Utmar-Koch-100007363059780/) erscheint im Suchindex als selbstständig/in Storkow, ist aber nicht als Betreiberprofil dieses Schrotthandels verifiziert. Kein zulässiger Doppelbeleg, kein öffentlicher Annahmenachweis. Die Aussage „keine aktuelle unabhängige Quelle“ auf **keinen qualifizierten Betreiber-/Registerbeleg** begrenzen; es gibt aktuelle Leads.

### 6. `bb-wandlitz-wolf-abschleppdienst`

**Entscheidung: `aktiv → pruefung`, Website-Normalisierung und vorsichtige Beschreibung freigeben.**

- **Diff/Historie:** Beide Zustandsänderungen tatsächlich neu; HEAD zuletzt `d9d551f0` (01.10.), bis dahin nur dünne Seed-Spur.
- **Eigene Gegenprobe:** [Gelbe Seiten](https://www.gelbeseiten.de/gsbiz/f711816b-b99a-4e05-8707-98a4f93ae581) und [Das Örtliche](https://www.dasoertliche.de/Themen/Autoverwertung-Wolf-Abschleppdienst-Wandlitz-Basdorf-Z%C3%BChlsdorfer-Str) erscheinen mit Zühlsdorfer Str. 5 und der bekannten Mobilnummer. Die aktuelle Suchdarstellung von Das Örtliche nennt ebenfalls Abschleppdienste; der behauptete Kategorienunterschied ist damit kein stabiler Sachbeleg.
- **Beleggrenze/Korrektur:** Gefundenes [Facebook-Places-Profil](https://m.facebook.com/pages/Autoverwertung-Wolf-Abschleppdienst/133204130551519) ist nicht als betreibergeführt nachgewiesen. Branchenbegriffe/Kategorien belegen weder aktuellen Fahrzeugankauf noch öffentliche Abgabe. Herabstufung als Evidenzvorsicht, ausdrücklich keine Schließungsentscheidung. Kategorienvergleich bei Übernahme als datierten Portalbefund kennzeichnen.

### 7. `bb-zossen-autoverwertung-klucke`

**Entscheidung: Klarstellung des Portal-Missverständnisses freigeben; kein neuer Betriebsnachweis.**

- **Diff/Historie:** Überblick präzisiert, Timeline ergänzt; keine Frontmatteränderung. Domain-/GESA-Klärfall und Nichtübernahme bereits `7465638d` (08.10.).
- **Eigener Direktabruf:** [Schrottplatz.org Klucke](https://www.schrottplatz.org/zossen/autoverwertung-autoverwertung-klucke-aYEPN2.html) führt **Klucke, Nr. 2, 03377 300471** im eigentlichen Profil und **HEBE, Nr. 4** separat im Abschnitt „Schrottplätze in der Nähe“. Der Bericht liest die Seitenstruktur insoweit richtig.
- **Beleggrenze/Korrektur:** Der Abruf belegt ausschließlich, was dieses Portal veröffentlicht. Kein Betreiber-/Registerbeleg und keine Anerkennung als Altfahrzeugbetrieb. Die neue Gegenprüfung darf eine frühere Suchsnippet-Vermutung korrigieren, aber nicht rückwirkend eine gültige Betreiberadresse behaupten. Den alten HTTP-410-Befund nur historisch referenzieren.

### 8. `rp-bechtolsheim-ayan-bechtolsheim`

**Entscheidung: keine neue Sachintegration erforderlich; Recheck optional.**

- **Diff/Historie:** Nur zusätzliche Timeline. Börrstadt-/Bechtolsheim-Abgrenzung einschließlich Preise bereits ausführlich integriert, zuletzt `bbfe6676` (07.10.), mit Identitätsprüfung vom 03.10.
- **Eigene Primärprobe:** [Impressum](https://www.schrotthandel-ayan.de/impressum) nennt Resul Ayan, Kaiserstraße 5, 67725 Börrstadt, ohne Bechtolsheim-Niederlassung; Steuerfelder sind Platzhalter. Das ist kein qualifizierter Nachweis für den Bechtolsheimer Seed.
- **Beleggrenze/Korrektur:** Fehlende Filialnennung beweist keine Nichtverbindung. „Keine belegte Identitätsbrücke“ ist zulässig, „sicher getrennte Rechtsträger“ wäre zu stark. Keine Adresse/Website/Tagespreiswerbung übertragen. Keine neuen numerischen Preise nachgewiesen; Wiederholung nicht als neuen Preislistenfund zählen.

### 9. `rp-bendorf-albert`

**Entscheidung: Enum und Lead-Überblick bedingt freigeben; Jasari-Hinweis nicht aufwerten.**

- **Diff/Historie:** Website-Normalisierung neu. Abhängigkeit von RäumungsFinder/11880 und historische Aggregator-Ausnahme für Kontakt stehen schon in `71f45681` (03.10.).
- **Eigene Gegenprobe:** Suche `"Albert Schrotthandel" Bendorf Jasari` findet [Albert/Cylex](https://web2.cylex.de/firma-home/albert-schrotthandel-15157667.html) an Bahnhofstr. 109 und [Jasari/Cylex](https://web2.cylex.de/firma-home/schrotthandel-jasari-15460171.html) an Welfenweg 4 mit anderer Telefonnummer. Beide sind nur Leads, weder Beweis einer Trennung noch einer Zusammenführung.
- **Beleggrenze/Korrektur:** [Vorstadt-Post-Profil](https://vorstadt-post.de/branchenbuch/bendorf/schrotthandel/albert-schrotthandel-32522/) und behauptete Mail bleiben ein nicht von mir direkt bestätigter Snippet-Hinweis. Unterschiedliche Bewertungen beweisen nicht automatisch Datenabhängigkeit; Nichtqualifikation folgt bereits aus der Aggregatornatur. Datums-/Stundenangaben nicht als aktuell verifiziert behandeln; keine Stundenübernahme. Historische Ausnahme ist keine heutige README-Freigabe der Adresse.

### 10. `rp-geinsheim-sudpfalz-metall-geinsheim`

**Entscheidung: Überblick als Abgrenzungsnotiz freigeben; keine neue Identitätsentscheidung.**

- **Diff/Historie:** Keine Frontmatteränderung; Domainbefund, AH-Gegenprobe und Rohrbach-Abgrenzung bereits `71f45681` (03.10.).
- **Eigene Primärabrufe:** [AH-Drucktechnik-Impressum](https://www.ah-drucktechnik.de/impressum.html) nennt Böbinger Straße 9, Geinsheim, ah-drucktechnik GmbH, HRB 64280. [Rohrbach-Impressum](https://suedpfalzmetall-gmbh.de/impressum) nennt Janneck Knecht, Am Bellensee 6, Rohrbach, HRB 33727; Kontaktfelder im Impressumsblock enthalten teils Platzhalter.
- **Beleggrenze/Korrektur:** Ein anderer Nutzer derselben Anschrift schließt Mitnutzung oder frühere Nutzung nicht aus. Rohrbach ist kein Nachweis für Geinsheim, aber ohne Identitätskette ist auch die endgültige Aussage „anderer Rechtsträger als der Seed“ nicht bewiesen. Sachfremde Altdomain hier nicht erneut geprüft; `tot` als historischen Websitebefund, nicht als Schließung formulieren. Kein autoritativer Geinsheim-Nachweis.

### 11. `rp-kinderbeuern-bach-kinderbeuern`

**Entscheidung: Enum freigeben; Recheck nur mit korrekter Neuheitskennzeichnung.**

- **Diff/Historie:** Nur Website-Normalisierung plus Timeline. Domainlead, Transportfehler, Material-/Mindestmengenabgrenzung stehen bereits in `6d713da6` (06.10.). „Neu gefundene Domain“ wäre sachlich falsch; neu sind allenfalls weitere erfolglose Unterseitenversuche.
- **Eigene Abrufe:** [Root](https://www.metallankauf-bach.de/) und [Impressum](https://www.metallankauf-bach.de/impressum) scheitern erneut mit Transportfehler. Suche nach Metallankauf Bach findet zusätzlich einen [Bausendorf-Portal-Lead](https://www.klick-deinen-schrottplatz.de/schrottplatz-schrotthaendler-bausendorf-rheinland-pfalz-metallankauf-bach-ID232); das ist kein belegter Standortwechsel.
- **Beleggrenze/Korrektur:** Eigentümer/Zuordnung und aktuelle Anschrift nicht gesichert. 100 kg Fe/1 kg NE bleiben unbestätigte Portal-Mindestmengen, keine Preisstaffel. Alte Timeline vom 02.10. mit „verifiziert“ durch zwei Portale ist durch spätere Einschränkungen relativiert, nicht als bestandener heutiger Standard nutzbar. Keine Aktivierung oder URL-/Kontaktfüllung.

### 12. `rp-koblenz-56073-kreutz-schrotthandel`

**Entscheidung: Enum und dokumentierten Adresskonflikt freigeben; aktuellen Betreiberbeleg weiterhin offen halten.**

- **Diff/Historie:** Enum neu; Dominicusstraße/Peter-Friedhofen-Straße-Konflikt und Nicht-Fill bereits `39c75397` (04.10.).
- **Eigene Gegenprobe:** Suche `"René Kreutz" Schrotthandel Koblenz` findet weitere Portale und eine konkrete neue [Kleinanzeigen-Anzeige](https://www.kleinanzeigen.de/s-anzeige/schrotthandel-schrotthaendler-kostenlose-abholung-metall-ankauf/3517253616-298-5424) mit Anbieter René Kreutz. **Direktabruf 403**: keine Rechtlichen Angaben, Urheberschaft, Anschrift oder tatsächliche Bedingungen verifiziert.
- **Beleggrenze/Korrektur:** Neue Anzeige als Folgeprüfspur dokumentierbar, nicht als Primärbestätigung. Portalwort „geschlossen“ muss auf Betriebsaufgabe versus momentanen Öffnungszustand geprüft werden; kein Schließungsstatus daraus. Mindestmengen und Bewertungen nicht zu Kursen/Garantien hochstufen. Keine Adress-/Telefonübernahme.

### 13. `rp-koblenz-karthause-56075-schrotthandel-maik-alten`

**Entscheidung: Enum und Korrektur der Portal-Zuordnung freigeben, weiterhin nur Leadniveau.**

- **Diff/Historie:** Enum/Überblick/Timeline neu. Cottbuser-Str.-Lead ist bereits `ab24d3e3` (02.10.); im aktuellen HEAD steht keine bestätigte Maik-Adresse Schönbornsluster Straße, die jetzt in Frontmatter zu korrigieren wäre.
- **Eigener Direktabruf:** [Werkenntdenbesten Koblenz](https://www.werkenntdenbesten.de/schrotthandel/koblenz) zeigt Cottbuser Str. 2 unmittelbar vor **Maik Alten**, Schönbornsluster Str. 12 unmittelbar vor **MCR Metallgroßhandel und Containerdienst Riebe GmbH**. Ein Suchsnippet schiebt diese Blöcke leicht zusammen; der Volltext bestätigt die Warnung des Berichts.
- **Beleggrenze/Korrektur:** Portal nicht Betreiberquelle; keine Identitäts- oder Umzugskette. 24-h- und Mindestmengenangaben nicht übernehmen. Die Klärung betrifft eine Lead-Lesart, keinen neuen branchentauglichen Standortbeweis.

### 14. `rp-landau-autoverwertung-demand-recycling`

**Entscheidung: Preis-/Gebühren-Recheck sachlich freigeben, aber nicht als neuen Fund integrieren.**

- **Diff/Historie:** Nur Timeline neu. Sämtliche wesentlichen Beträge/Bedingungen stehen bereits in `877ba545` (08.10.); kein Frontmatter-Fill und keine neue Preisdatenstruktur.
- **Eigene Primärabrufe:** [Fahrzeugabgabe](https://www.autoverwertung-demand.de/fahrzeugabgabe) bestätigt Prämie **je nach Typ, Alter, Zustand 50–2.000 €**, Abholung im 50-km-Umkreis nach Termin, ausdrücklich kostenlos für Landau/SÜW; Abmeldung gegen unbezifferte geringe Gebühr; benötigte Müllsäcke **5 €/Stück**, größere Müllmengen mit Annahmeverweigerung oder tatsächlichen Entsorgungskosten. [Impressum](https://www.autoverwertung-demand.de/impressum) nennt Demand-Recycling e.K., Roswitha Wisniewski, HRA 2369 AG Landau.
- **Beleggrenze/Korrektur:** Keine garantierte Mindestprämie für jedes Fahrzeug und kein €/kg-Kurs. Betreiber-Selbstbeschreibung „zertifiziert“ ersetzt kein geprüftes gültiges Zertifikat. Zwei Unterseiten ergeben keinen Register-Zweitbeleg; aktuelle Nachfolgekette/autoritative Ausnahme hier nicht vollständig geprüft. Aussage „keine Ersatzteilpreisliste“ nur auf die geprüften Seiten begrenzen, nicht auf jeden Vertriebskanal.

### 15. `rp-limburgerhof-breyer`

**Entscheidung: Enum und expliziten Klärfall-Überblick freigeben; kein neuer Rechercheerfolg.**

- **Diff/Historie:** Enum/Überblick neu; erfolglose Namenszuordnung bereits in HEAD (letzte Änderung `d9d551f0`, 01.10.).
- **Eigene Gegenprobe:** Suche `"Breyer" "Limburgerhof" Schrott` führt vor allem zu regionalen Containerdienst-Verzeichnissen und Namensähnlichkeiten, u. a. zur Domain [container-breyer.de](https://www.container-breyer.de/) als Verzeichnislink. Kein hier verifizierter Betreiber-/Registerbeleg für den Limburgerhofer Seed.
- **Beleggrenze/Korrektur:** Suchergebnisabwesenheit ist kein Existenz-/Schließungsbeweis. Root-/Namensähnlichkeiten rechtfertigen weder Standort- noch Kontaktdaten. Die neue Timeline hat keine konkrete neue Einzelfundstelle; sie ist als begrenzter Negativ-Recheck, nicht als Quellenabschluss akzeptabel. Gewerbe-/Betreiberidentifizierung bleibt nötig.

### 16. `rp-mainz-55120-eftekhari`

**Entscheidung: Enum freigeben; Registerbeschreibung vor neuer Sachintegration präzisieren.**

- **Diff/Historie:** Nur Enum plus Timeline. Standortkonflikt und Liquidation bereits in `bbfe6676` (07.10.) dokumentiert; nicht erneut als neue Identitätsklärung zählen.
- **Eigener zulässiger Registerkanal:** [NorthData aktuelles Registerprofil](https://www.northdata.de/Eftekhari%20GmbH,%20Mainz-Kastel/Amtsgericht%20Wiesbaden%20HRB%2034887) nennt **Wiesbaden HRB 34887**, zusätzlich frühere **Mainz HRB 48328**, Am Fort Biehler 12, Mainz-Kastel; Sitzverlegung März 2024 und Liquidation **27.09.2024**. Der neue Absatz verkürzt dies unnötig auf die alte HRB.
- **Korrektur/Beleggrenze:** Nicht einen „anderen Rechtsträger“ aus dem Registerwechsel ableiten: Die Registerseite führt beide Kennungen derselben Gesellschaft. Unbewiesen bleibt die **Zuordnung des Akazienweg-1-Leads zu dieser Gesellschaft bzw. Niederlassung**. Aktuelle und alte Registernummer sauber unterscheiden; Liquidation ist nicht automatisch Betriebsaufgabe der ungeklärten Seed-Zeile. Alter Domain-404 bleibt historisch, kein Schließungsbeleg.

### 17. `rp-trier-54294-wenner-david`

**Entscheidung: Enum/Überblick freigeben, aber Social-Folgehinweis aufnehmen, falls neu recherchierter Stand beansprucht wird.**

- **Diff/Historie:** Enum/Überblick neu; David/Dennis- und Adresskonflikt bereits in HEAD vom 01.10. (`d9d551f0` als letzte Dateihistorie).
- **Eigene Gegenprobe:** Suche `"Wenner David" Schrott` findet [Facebook David Wenner](https://www.facebook.com/david.wenner.7503/) mit Suchindex-Selbstdarstellung „Schrotthandel D.Wenner“, Wohnort Trier. **Direktabruf ohne lesbaren Text**. Daneben weiterhin [Das Örtliche](https://www.dasoertliche.de/Themen/Wenner-David-Schrott-Metallrecycling-Trier-S%C3%BCd-Im-Schammat) als Lead Im Schammat 60.
- **Beleggrenze/Korrektur:** Social-Treffer ist ein möglicher Betreiberkanal, aber weder Authentizität/Aktualität noch Anschrift sind bestätigt. Kein Doppelbeleg und keine Brücke zu Dennis. Pauschales „kein Social“ aus der Alt-Timeline nicht unkommentiert als aktuellen Suchabschluss wiederholen. Kleinanzeigen-Root ist keine reproduzierbare Quelle des konkreten Dennis-Inserats; dessen Angaben ausdrücklich historisch/unbestätigt belassen.

### 18. `rp-trier-m-trier-west`

**Entscheidung: Enum und Warn-Überblick freigeben; keine Adressbereinigung freigeben.**

- **Diff/Historie:** Enum/Überblick neu; fehlerhafter Importstring und bewusster Nicht-Fill bereits `df6be8fb` (02.10.).
- **Eigene Gegenprobe:** Exakte Suche `"M Trier-West"` liefert keinen qualifizierten Händlernachweis. Der unmittelbar lesbare Primärbeleg für die Datenqualität ist hier **das Git-Objekt und Arbeitsbaum-Frontmatter** mit `M Trier-West Trier Hornstr. 23`; das beweist ein Importproblem, keinen realen Händler oder eine korrekte Anschrift.
- **Beleggrenze/Korrektur:** Keine belastbare Identifizierung. Ein separat bekannter Theo-Steil-Betrieb ist kein Ersatz für die fehlende Identität; dessen aktuelle Anschrift wurde in diesem Review nicht als Beleg neu geprüft. „Sicher anderer Betrieb“ besser als „keine belegte Zuordnung, nicht übernehmen“ formulieren. Warntext heilt den fehlerhaften strukturierten Straßenwert nicht; der bleibt ein gesonderter Owner-Klärfall, hier unangetastet.

### 19. `rp-weienthurm-rwr-weienthurm`

**Entscheidung: Enum sowie neue Registerabsicherung für Bassenheim bedingt freigeben; EUID-Aussage zwingend korrigieren.**

- **Diff/Historie:** Enum neu; Bassenheim-Impressum, fehlende Weißenthurm-Kette und Preisgrenzen bereits `91a7b3a1` (06.10.). Tatsächlich neu ist der explizite Abgleich HRB 32106/Eintragung.
- **Eigene Direktabrufe:** [Impressum](https://www.rwr-recycling.de/impressum/) nennt RWR Recycling GmbH, Am Bahnhof 9, Bassenheim, Rebecca Reinhardt und `USt-IdNr.: DET2210V.HRB32106`. [NorthData](https://www.northdata.de/RWR%20Recycling%20GmbH,%20Bassenheim/Amtsgericht%20Koblenz%20HRB%2032106) bestätigt Koblenz HRB 32106 und führt **exakt `DET2210V.HRB32106` als EUID**, Eintragung 22.07.2026 mit Rebecca Reinhardt.
- **Zwingende Korrektur:** Die Kennung ist **nicht erkennbar verstümmelt/fehlformatiert**, sondern eine nachvollziehbare **EUID unter falscher Überschrift „USt-IdNr.“**. So dokumentieren; keine USt-ID erfinden. Der Registerabgleich trägt Bassenheim, nicht die Identitätskette zum alten Weißenthurm-Einzelunternehmenslead.
- **Preise:** [Metallhandel](https://www.rwr-recycling.de/metallhandel/) nennt qualitäts-/marktbezogene Tageshöchstpreise und kostenloses Angebot, keine Zahlen. Kostenlose Angebotserstellung nicht zu kostenloser Logistik machen. Keine Weißenthurm-Website-/Adress-/Statusübernahme.

### 20. `rp-westheim-rohstoffe-trapp`

**Entscheidung: qualifizierte Servicekorrektur und Betreiber-Selbstauskunft freigeben; Mengenbedingung nicht verabsolutieren.**

- **Diff/Historie:** Kein Frontmatter geändert. Anschrift/Kontakt und historischer Fehlerhinweis bereits `16cf4db8` (01.10.). Neu sind vor allem detailliertere Bedingungen und Inhaberin/USt-ID im Überblick.
- **Eigene Primärabrufe:** [Impressum](https://rohstoffe-trapp.de/impressum.html) nennt Susanne Trapp, Hauptstrasse 142, 67368 Westheim, DE223302816. [Startseite](https://rohstoffe-trapp.de/) nennt Westheim (Pfalz), 80-km-Umkreis, „Kostenlose Abholung bei verwertbarem Material und Vergütung bei größeren Mengen“; FAQ: kostenlos in vielen Fällen abhängig vom Material, auch kleine Mengen je nach Materialart. [Ankaufseite](https://rohstoffe-trapp.de/schrott-metallankauf.html) wirbt ausdrücklich sowohl mit kleinen Mengen als auch größeren Posten, ohne Zahlenkurse.
- **Korrektur/Beleggrenze:** Überblick „Vergütung **nur** bei größeren/verwertbaren Mengen“ ist stärker als der belegte Text. Besser wörtlich/attributiert „Startseite bewirbt Vergütung bei größeren Mengen; Einzelfallangebot nötig“. Keine generelle Mindestmenge ableiten. Ein Betreiberauftritt ohne unabhängigen Identitätsbeleg genügt hier nicht für einen Frontmatter-Vollfill; Impressumsanschrift ist kein bestätigter Kundenhof. Historischen Fehlerhinweis nicht als neu überprüfte Geodatenlage ausgeben.

## Integrationsauftrag an den Owner, nicht hier ausgeführt

1. **Vor Übernahme korrigieren:** WSH aktueller lokaler 11880-Lead statt fortgeschriebener Trefferlosigkeit; RWR richtige EUID-Einordnung; Eftekhari aktuelle/alte Registernummer und Identitätsgrenze; Trapp kein unbelegtes „nur“ bei Vergütung.
2. **Teiländerungen separat behandeln:** Zwei neue Prüfstatus-Herabstufungen und elf Website-Zustandspräzisierungen sind der gesamte skalare Änderungsumfang. Keine nachträgliche Quellenfreigabe alter Aggregator-Fills (insbesondere Gramet, Radke, Albert, Bach).
3. **Redundanz vermeiden:** Radke-Status, Demand-Beträge, Ayan-Ortsabgrenzung, Eftekhari-Liquidations-/Standortkonflikt, Klucke-Domainbefund und RWR-Servicegrenzen stehen schon in HEAD. Nur neue belegte Nuance bzw. klar datierten Recheck integrieren; alte Timeline erhalten, nicht durch neue Gewissheit überschreiben.
4. **Negativbefunde begrenzen:** „Auf geprüften Seiten keine Zahlenliste“ statt „keine Preise vorhanden“; „kein qualifizierter Nachweis gefunden“ statt „Betrieb existiert nicht“. Social-/Anzeigenfunde bei Storkow, Wolf, Wenner und Kreutz sind Folgeprüfspuren, keine übernahmefähigen Primärbelege.
5. Kein Dossier erhält durch diesen Review eine neue `aktiv`-/`geschlossen`-Freigabe oder einen geprüften öffentlichen Annahmestandort. Tests/Seed-/Release-/Produktionsgate bleiben Aufgabe eines später ausdrücklich autorisierten Integrationsschritts; hier wurden keine Cargo-/Buildtests ausgeführt.

## Prüfgrenzen und Nachweisqualität

- NorthData ist nach README ein zulässiger Registerkanal; die hier lesbaren Profile sind keine amtlich beglaubigten Registerauszüge. Nicht gelesene Einzelpublikationen werden nicht als selbst geprüft ausgegeben.
- Belegstärkste frische Ergebnisse: Gramet-Historie, RWR HRB/EUID und Eftekhari-Registerwechsel (zulässiger Registerkanal), Demand-Konditionen und Trapp-Betreiber-/Serviceaussagen (direkte Betreibertexte). Das löst keine der offenen lokalen Identitätsketten vollständig.
- Verzeichnis-Direktabrufe bei WSH/Klucke/Maik dienen ausschließlich dazu, **Aussagen über die jeweiligen Portaltexte** zu überprüfen. Sie sind ausdrücklich keine Primärbelege für den Betrieb.
- Kein Schluss aus bloßer Suchindex-Abwesenheit, kein fingierter unabhängiger Doppelbeleg aus zwei Betreiberunterseiten, kein Transfer fremder Standorte. Die Entscheidungssicherheit liegt bei der jeweiligen Begrenzung/Übernahmeempfehlung, nicht bei einer behaupteten Vollverifizierung aller Händler.
