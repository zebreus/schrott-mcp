# Feedback 7390 — Audit 7, Identitäts-/Standort-Dubletten B

Stand: 14.10.2026. Umfang ausschließlich **Index 20–39 (nullbasiert)** aus `/tmp/opencode/audit7-identity-duplicate-candidates.json`: 20 Gruppen, 40 Feedback-IDs. Vollständige `feedback`-/`details`-Felder wurden ausgewertet; unten genannte Betreiberseiten wurden am 14.10.2026 erneut read-only abgerufen, soweit nicht ausdrücklich anders gekennzeichnet.

## Maßstab und Ergebnisgrenze

Maßstab: `README.md`, insbesondere „Feedback triage“, Punkte 1–7, und „Operator rename/merge chains“. Name/PLZ, Familienname, gemeinsamer Konzern und aggregierte Verzeichniseinträge reichen **nicht** als Identitätsbeweis. Entscheidend ist die Verbindung Betreiber → konkreter Standort → standortbezogene Kontakte/Leistungen. Verschiedene Standorte bleiben getrennt. Betreiberwechsel benötigen eine belegte Kette.

**Dies ist eine Empfehlung zum Bündeln von Triagearbeit, keine Händlerfreigabe und keine Löschfreigabe.** „Bündeln“ heißt: zwei Meldungen behandeln denselben beschriebenen Standortfall; beide IDs, ihre zusätzlichen Angaben und Gegenbelege bleiben erhalten. Es wurde kein Register-Livestatus unabhängig überprüft. Mehrere Unterseiten derselben Betreiberwebsite sind nur **eine** Quelle. Die README-Ausnahme für eine vollständig verifizierte autoritative Betreiberquelle wird daher nicht pauschal beansprucht; insbesondere für historische Betreiberketten/Umzüge bleibt die unabhängige zweite Identitätsquelle offen.

Ergebnis: **17 Gruppen als primärquellenverankerte Doppelhinweise bündeln** (20, 22, 23, 25–29, 31–39); **2 Gruppen nur als gemeinsame ungeklärte Leads bündeln** (21, 24); **1 Gruppe nur für den ausdrücklich aktuellen Standortanteil bündeln, Altadresse/Altbetreiber separat offen halten** (30). Keine Gruppe beweist zwei verschiedene aktuelle Betreiber/Standorte innerhalb der beiden ausdrücklich gemeldeten Hauptfälle. Das ist kein Beweis, dass sämtliche in den Details erwähnten Altadressen identisch sind.

Die in Meldungen behaupteten Produktions-DB-Abgleiche vom 12.10.2026 wurden nicht neu verifiziert und sind kein Identitätsbeweis. Kein Produktionszugriff war erforderlich. Keine Dossiers, geschützten Fremddateien, Feedbacks oder Produktionsdaten wurden verändert; keine Kontaktaufnahme, TLS-Ausnahme, Commits oder Deployments.

## Einzelprüfung

### Index 20 — Metallhandel Konik, Bochum — IDs 6283 / 7283

- **Urteil:** Doppelhinweis auf denselben Betreiber-/Standortfall; bündeln, Identitätsbeleg weiterhin Einzelquelle.
- **Übereinstimmungen:** Beide melden Elsa-Brändström-Str. 56B, 44795 Bochum, Festnetz 0234 5882472, Mobil 0160 1542376, `info@metallhandel-konik.de`, dieselbe Domain und denselben Metallankauf/Abholservice. Das erneut abgerufene [Impressum](http://www.metallhandel-konik.de/impressum) bindet genau diese Daten an **Alexander Konik**, USt-ID **DE224636663**; [Kontakt](http://www.metallhandel-konik.de/kontakt) weist denselben Standort mit Geschäftszeiten aus. Damit mehr als bloße Namens-/PLZgleichheit.
- **Widersprüche/Abgrenzung:** Kontaktseite nennt im Kontaktblock Faxvorwahl **0233**, im eigenen Footer **0234**; beide Meldungen nutzen 0234. Interner Quellenwiderspruch, kein Beleg für zweiten Betrieb. „König“ in Bochum und A. Stöber, Am Kuhlenkamp 34, sind nur in Meldung 7283/6283 genannte Fremdfälle, nicht mit Konik zusammenführen. 7283 ergänzt Inhaber, USt-ID, Zeiten und Barauszahlung; behalten.

### Index 21 — Karl Dandler Schrotthandel, Kröv-Kövenig — IDs 6287 / 7343

- **Urteil:** Sehr ähnlicher **Doppel-Lead**, gleiche reale Betreiberidentität jedoch **nicht nach README belegt**; gemeinsame Klärarbeit ja, endgültige Identitätsdeduplizierung nein.
- **Übereinstimmungen:** Beide Details beschreiben Karl Dandler, Kirchstr. 12, 54536 Kröv-Kövenig, Festnetz 06541 812526 **und** Mobil 0170 2135352, Schrotthandel. Sie referenzieren denselben [11880-Datensatz](https://www.11880.com/branchenbuch/kroev/120674719B26099839/karl-dandler-schrotthandel.html); 7343 benennt zusätzlich zwei Gelbe-Seiten-Listings mit den unterschiedlichen Nummern. Das belegt die Wiederholung desselben Rechercheleads, nicht die heutige rechtliche Betreiberidentität.
- **Widersprüche/Beleggrenze:** Ausschließlich Aggregatoren, kein Betreiberimpressum/Register-/Kommunalbeleg. Die Behauptung in 7343 „gleiche Anschrift = ein Betrieb“ ist ausdrücklich **kein** ausreichender Beweis. Ergänzende Suche nach „Karl Dandler“/„Kirch“/Schrotthandel erbrachte keine nutzbare standortbezogene Primärquelle; namensgleiche Malta-Profile und Schrott Karl in Bayern sind nicht zuordenbar. Kein belegter Betreiberwechsel, aber auch kein belastbarer Ausschluss. Beide IDs unter offenem Identitäts-/Existenzcheck erhalten.

### Index 22 — Nehlsen E. Heeren, Standort Leer — IDs 6297 / 7356

- **Urteil:** Doppelhinweis für **Leer**; bündeln. Historische Kette Interseroh → Nehlsen nicht hier abschließend verifiziert.
- **Übereinstimmungen:** Beide beschreiben Nehlsen E. Heeren GmbH, Sägemühlenstr. 102, 26789 Leer, 0491 92807-0, `info.e.heeren@nehlsen.com`; identische Kassen-/Annahmezeiten, Personalausweis und standortbezogener Stahl-/NE-Metallankauf. Die erneut gelesene [Standortdetailseite Leer](https://www.nehlsen.com/standorte/detail/nehlsen-e-heeren-gmbh-standort-leer) nennt Betreiber, Straße, Zentrale, Materialliste und Zeiten zusammen.
- **Widersprüche/Abgrenzung:** 7356 ergänzt den Aggregator-Altnamen „Interseroh Evert Heeren GmbH“ und 0491 9280740. Keine unabhängige Registerhistorie abgerufen: Altname nicht als gesicherte Umfirmierung übernehmen. **Emden ist ausdrücklich getrennt**: [Betreiberdetail Emden](https://www.nehlsen.com/standorte/detail/nehlsen-e-heeren-gmbh-standort-emden) bestätigt Zu den Hafenbecken 18, 26723 Emden, 04921 20216. Den in beiden Details genannten DB-Standort 1163 in Emden keinesfalls mit Leer deduplizieren. Gleiche E-Mail/Gesellschaft ersetzt keinen Standortnachweis.

### Index 23 — Autoverwertung Herrmann, Geestland-Lintig — IDs 6300 / 6589

- **Urteil:** Doppelhinweis auf denselben Erwin-Herrmann-Standort; bündeln, Einzelbeleg.
- **Übereinstimmungen:** Erwin Herrmann, Industriestraße 20, 27624 Geestland-Lintig, 04745 6697, Fax 04745 8090, `autoverwertung-herrmann@freenet.de`, Domain und Pkw-Ankauf/Verwertung stimmen in beiden Meldungen überein. Das aktuelle [Impressum](https://autoverwertung-herrmann.de/impressum) nennt ausdrücklich „Fa. Erwin Herrmann, Autoverwertung“, „Inh. Erwin Herrmann“, dieselbe Anschrift und Kontakte, USt-ID **DE116757145**.
- **Widersprüche/Abgrenzung:** Websitefooter nennt zusätzlich Marco Herrmann als fachlichen Ansprechpartner; das widerlegt den im Impressum benannten Inhaber nicht und beweist keinen Wechsel. GESA ZAAS00300052/1 in 6589 wurde laut Meldung nicht erneut geprüft, auch hier nicht als zweite Quelle gezählt. Abholkanal/Letztbesitzer in 6589 ist Ergänzung gegenüber 6300, kein zweiter Standort. Herrmann-Namensvetter in anderen Orten sowie SMA, An der Autobahn 7, bleiben getrennt.

### Index 24 — Jörg Remde, Kyritz-Mechow — IDs 6319 / 7311

- **Urteil:** Doppel-Lead derselben Verzeichnisrecherche; **Betreiberidentität/aktiver Standort ungesichert**, keine endgültige Deduplizierung.
- **Übereinstimmungen:** Beide Details betreffen Jörg Remde KFZ-Handel/Autoverwertung, Chausseestr. 2 A, 16866 Kyritz-Mechow, 033971 53889 und `Mietstation-Mechow.de`; 2 A/2A ist nur Schreibweise. 6319 hat zusätzlich Verzeichnis-E-Mail und Zeiten, 7311 keine nutzbaren zusätzlichen Primärbelege.
- **Aktueller Abruf:** [HTTP-Website](http://www.Mietstation-Mechow.de) zeigt weiterhin nur „Hier ensteht eine neue Homepage“/T-Home-Webhosting, ohne Betreiber/Adresse/Ankauf. HTTPS-Abruf scheitert an **Zertifikats-Hostnameabweichung**; keine TLS-Ausnahme verwendet. Der HTTP-Platzhalter ist kein Betreiberbeweis und kein Schließungsbeweis.
- **Widersprüche/Beleggrenze:** Beide stützen sich letztlich auf Das Örtliche/Gelbe Seiten (z. B. [Lead 6319](https://www.dasoertliche.de/?form_name=detail&id=1106000360630652125858&niceid=Remde-J%C3%B6rg-KFZ-Handel-Autoverwertung-Kyritz-Mechow-Chausseestr)). Branchenbezeichnung „Schrott“ ersetzt keine ausdrückliche Ankaufsaussage. Ergänzende Namens-/Mechow-Suche ergab keine zurechenbare Primärquelle. Ob aktiver Altbetreiber, Nachfolge oder veraltete Listings bleibt offen; beide IDs im gemeinsamen Klärfall erhalten.

### Index 25 — pro aurum, Bad Homburg — IDs 6346 / 6810

- **Urteil:** Doppelhinweis auf dieselbe konkrete Filiale; bündeln.
- **Übereinstimmungen:** Haingasse 22/Ecke Höhestraße, 61348 Bad Homburg, +49 6172 18969-0, `badhomburg@proaurum.de`, identische Filialdomain und Altgoldservice. Die aktuelle [Filialseite](https://www.proaurum.de/unternehmen/standorte/bad-homburg/) nennt **pro aurum GmbH, Filiale Bad Homburg**, genau Anschrift/Telefon und Goldankauf vor Ort, samt aktuellem Newsverweis vom 08.10.2026. [Impressum](https://www.proaurum.de/impressum/) nennt pro aurum GmbH, HRB **178875 München**, und dieselbe Geschäftsstelle separat vom Münchner Sitz.
- **Widersprüche/Abgrenzung:** Einschränkung an verkaufsoffenen Samstagen in 6810 ergänzt 6346, keine getrennte Identität. Navi-Adresse Höhestraße 6 auf der Filialseite ist ausdrücklich Parkdeck-Anfahrt, nicht neuer Handelsstandort. Hamburger Filiale/DB 10149 und Münchner Firmensitz bleiben getrennt. Die zwei Betreiberunterseiten sind keine zwei unabhängigen Quellen.

### Index 26 — Julius Lax GmbH, Minden — IDs 6517 / 7322

- **Urteil:** Doppelhinweis auf denselben Hof; **unterschiedliche Domains sind positiv abgeglichen**, bündeln.
- **Übereinstimmungen:** Beide melden Schwarzer Weg 14, 32423 Minden, 0571 30410, Fax 0571 37516, `info@juliuslax.de`, Metalle/Batterien, Hofwaage und Abrechnung. [Impressum juliuslax.de](https://www.juliuslax.de/impressum.html) **und** [Impressum julius-lax-minden.de](http://julius-lax-minden.de/impressum.html) liefern dieselben Anbieterangaben: Julius Lax GmbH, vertreten durch Heiner Lax, HRB **4793 Bad Oeynhausen**, USt-ID **DE155834126**, identischer Hof/Kontakte. Keine bloß aus dem Domainnamen abgeleitete Identität.
- **Widersprüche/Abgrenzung:** Zwei Domains, aber kein abweichender Betreiber/Standortinhalt im Abruf. Die in Details genannten Minden-Einträge Chow/Möller/Pold und Schwarzer Weg in Perleberg sind nicht Lax. 7322 ergänzt Registerangaben; behalten. Die behauptete BDSV-Mitgliedschaft aus 6517 wurde nicht neu abgerufen und zählt hier nicht als zweiter Beleg.

### Index 27 — AMR Entsorgung GmbH, Rahden — IDs 6524 / 6964

- **Urteil:** Doppelhinweis auf denselben **Schrottplatz Eisenbahnstraße 7**, nicht zwei Schrottstandorte; bündeln, Büroanschrift nicht als Anlieferstandort ersetzen.
- **Übereinstimmungen:** Beide Details nennen **beide** Adressen mit identischen Rollen, 05771 9740-0, `info@amr-entsorgung.de`, Domain, Sorten und Abrechnung am selben Tag. Die aktuelle [Kontaktseite](https://www.amr-entsorgung.de/kontakt) trennt „Postadresse/Büro Weller Straße 15“ von „Schrottplatz/Wertstoffhof Eisenbahnstraße 7“, beide Rahden, samt unterschiedlichen Öffnungszeiten. Das [Impressum](https://www.amr-entsorgung.de/impressum) bindet den Betreiber an HRB **11564 Bad Oeynhausen**, Petra Laurenz, Büro Weller Straße 15 und dieselbe Zentrale.
- **Widersprüche/Abgrenzung:** Unterschiedliche erste Straßenangabe erklärt sich ausdrücklich durch Büro versus Annahme. Das rechtfertigt **nur** Bündelung der vorliegenden identischen kombinierten Fälle, nicht pauschal Zusammenführung beliebiger AMR-Standorte. AMR Metalle und Rohstoffe in Berlin ist laut Details andere Firma; nicht zuordnen. Materialfokus Stanzabfälle/Neuschrott in 6964 erzeugt keine neue Filiale.

### Index 28 — Hasselbacher, Weißenburg — IDs 6545 / 7014

- **Urteil:** Doppelhinweis auf den **aktuell in beiden Meldungen ausdrücklich benannten Standort Nr. 24**; bündeln. Weitere Betriebsflächen nicht blind einziehen.
- **Übereinstimmungen:** Beide aktuellen Details nennen Treuchtlinger Straße **24**, 91781 Weißenburg, 09141 85230, `info@hasselbacher-metalle.de` und dieselbe Domain. [Impressum](https://hasselbacher-metalle.de/impressum/) und [Kontakt](https://hasselbacher-metalle.de/kontakt/) nennen Hasselbacher GmbH Metallrecycling, Nr. 24 und identische Kontakte; Impressum ergänzt HRB **3865 Ansbach**, Werner/Christian Hasselbacher, USt-ID **DE262033793**.
- **Widersprüche/Abgrenzung:** 6545 nennt Betriebsflächen 14/28/39 nach BDSV/ESN; 7014 erklärt ausdrücklich, die Audit-6-Erstmeldung habe 14 genannt, seine jetzige Meldung aber 24. Diese anderen Hausnummern sind **nicht** unabhängig überprüft; weder bloßer Tippfehler noch ein Hofkomplex noch Umzug ist dadurch bewiesen. Die ID-Paar-Bündelung bezieht sich auf Nr. 24; eigenständige Meldungen zu 14/28/39 benötigen Standort-/Zertifikatsprüfung. NE-Späne versus allgemeines Metallrecycling ist nur Leistungsfokus.

### Index 29 — Limekon GmbH, Dörentrup-Bega — IDs 6549 / 7008

- **Urteil:** Doppelhinweis für Limekon an Bahnhofstr. 23; bündeln. Historischer Alias und Personenschreibweise nicht ungeprüft fortschreiben.
- **Übereinstimmungen:** Bahnhofstr. 23, 32694 Dörentrup-Bega, +49 5263 9561906, `info@limekon.de`, Domain, Zinn/Silber/Goldankauf, Paket-/persönlicher Ankauf stimmen überein. Das aktuelle [Impressum](https://limekon.de/pages/impressum1.php) verbindet **Limekon GmbH** mit dieser Anschrift/Telefon, USt-ID **DE300828016**, Entsorgernummer **E766A7005** und Link „Zertifikat Entsorgungsfachbetrieb 2026“.
- **Betreiberdetail:** 7008 nennt zusätzlich **Theodor Brüggemann** als Vertretung; die aktuell gelesene Betreiberseite bestätigt diesen Namen. 6549 nennt keine abweichende Person. Damit kein personeller Gegenbeleg innerhalb des ID-Paars; trotzdem kein unabhängig verifizierter Register-Livestatus.
- **Weitere Abgrenzung:** „früher Lippische Metallkontor“/`zinn-ankauf.de` in 7008 ist keine in dieser Prüfung belegte Rechtsnachfolge. Die [Startseite](https://limekon.de/) nennt inzwischen Preisliste **14.10.2026 08:00**, statt des in beiden Meldungen referenzierten Stands 05.10.; normale Preisaktualisierung, kein Betreiberwechsel. Keine Preisübernahme/Freigabe aus dieser Dublettenprüfung.

### Index 30 — Hellmich, Büddenstedt/Helmstedt — IDs 6606 / 7376

- **Urteil:** **Bedingte Bündelung:** Die aktuellen Betreiberseiten-/Ankaufsteile beider Meldungen betreffen dieselbe Betriebsstätte **Alversdorfer Straße 75**. Den in 7376 enthaltenen Altadress-/Altbetreiberfall **nicht als erwiesene Standortdublette wegdeduplizieren**.
- **Übereinstimmungen:** Beide Details benennen HELLMICH RECYCLING GmbH, Betriebsstätte Helmstedt, Alversdorfer Straße 75, 38372 Büddenstedt, 05352 9068313, dieselbe Domain/E-Mail und Zeiten. [Standorte](https://hellmich-recycling.de/standorte/) nennt genau diese Betriebsstätte; [Helmstedt-Detailseite](https://hellmich-recycling.de/helmstedt/) verbindet Nr. 75 mit Alt-Kfz-/Schrottannahme und Ankauf. [Impressum](https://hellmich-recycling.de/impressum) nennt Mathias Hellmich, HRB **35881 Hildesheim**, HELLMICH RECYCLING GmbH.
- **Konkreter Widerspruch:** 7376 hat als **ursprünglichen Verzeichniskandidaten „Hellmich Rohstoff GmbH“, An der Kreisstr. 22, Reinsdorf, 05352 9698313**. Diese Adresse und Nummer weichen tatsächlich ab; keine einfache Telefonnummernnormalisierung! Websitegleichheit/ähnlicher Firmenname beweist weder Umzug noch Identität der Rechtsträger. Für Rohstoff → Recycling sowie Kreisstr. 22 → Alversdorfer 75 fehlen Registerhistorie und standortbezogene zweite Quelle. Diesen offenen Teil von 7376 bei Bündelung sichtbar erhalten.
- **Andere echte Standorte:** Lehrte, Industriestraße 57, und Herzberg, Am Hattorfer Weg, werden auf der Betreiberseite separat mit eigenen Telefonen aufgeführt. Sie sind nicht mit Büddenstedt zu deduplizieren, auch wenn beide Meldungen deren Daten mitliefern.

### Index 31 — KOOS Edelmetalle, Renningen — IDs 6675 / 7093

- **Urteil:** Doppelhinweis für denselben Betreiber/Standort; bündeln, Einzelquelle.
- **Übereinstimmungen:** KOOS Edelmetalle GmbH, Steinbeisstraße 1, 71272 Renningen, +49 7159 9274-0, `info@koos.de`, Domain, Scheidgut/Gekrätz und Sofort-/Versandservice stimmen überein. Das aktuelle [Impressum](https://www.koos.de/de/impressum) nennt Kevin Koos, HRB **252425 Stuttgart**, USt-ID **DE147867079**, dieselbe Sitzanschrift/Kontakte.
- **Widersprüche/Abgrenzung:** 07159/92740 versus 07159/9274-0 sind identische Ziffern; `scheidgut@koos.de` in 7093 ist zusätzliche Funktionsmail, kein weiterer Standort. Unterschiedliche Arten („Scheidgut“, „Scheideanstalt“) und Dentalspendenhinweis ändern die Identität nicht. Gold.de-Verzeichnis ist keine unabhängige Identitätsevidenz. Gewerblicher Kundenkreis/Sofortservicebedingungen aus 6675 nicht durch die allgemeinere 7093 überschreiben.

### Index 32 — Marco Schmidt Goldankauf, Heidelberg — IDs 6690 / 7089

- **Urteil:** Doppelhinweis für **Heidelberg Bergheimerstraße 97**; bündeln. Altlußheim und ungeklärte Walldorf-Adresse nicht einziehen.
- **Übereinstimmungen:** Beide melden Marco Schmidt, Bergheimerstraße 97, 69115 Heidelberg, 06221 7253664, Mobil 0162 4888769, `info@schmidt-gold.de`, dieselbe Domain und Materialien. Das aktuelle [Impressum](https://www.schmidt-gold.de/impressum.html) nennt Marco Schmidt Goldankauf, Heidelberg und Mobilnummer; [Betreiberstartseite](https://www.schmidt-gold.de/) führt Heidelberg **und separat Altlußheim**, Hauptstraße 55, 68804, mit eigenen Zeiten auf.
- **Widersprüche/Abgrenzung:** 7089 benennt Gold.de-Lead **Hauptstraße 3, 69190 Walldorf**, gleiche Mobilnummer/Domain. Die aktuelle Website überschreibt diese nicht einfach: Seitentitel erwähnt zwar auch Walldorf, die Standortnavigation/Adressblöcke aber nur Heidelberg/Altlußheim. Daher Walldorf weder als geschlossen noch als umgezogen noch als identisch bewiesen. CMS-/Goldverwertungsalias ist keine Registerkette. Der ID-Paar-Abgleich Heidelberg bleibt positiv; Altlußheim ist ein real separat dargestellter Standort.

### Index 33 — Goldankauf-Pro24 GmbH, München — IDs 6704 / 7111

- **Urteil:** Doppelhinweis auf denselben Münchner Goldladen; bündeln.
- **Übereinstimmungen:** Beide melden Schlörstraße 2, 80634 München, +49 89 718918, `info@goldankauf-pro24.de`, Domain, Zahngold-/Altgoldankauf sowie Laden-/Versandkanal. Das aktuelle [Impressum](https://www.goldankauf-pro24.de/impressum) nennt **David Rosendorfer**, HRB **194952 München**, USt-ID **DE279903753**, exakt die Standort-/Kontaktdaten und daneben „Barankauf in Münchner Goldladen“/Zeiten; Aktualisierungsdatum 27.02.2025.
- **Widersprüche/Abgrenzung:** Kategorien/Materiallisten variieren nur im Umfang; kein zweiter Betreiber/Standort genannt. SEO-Städtelinks „Goldankauf in Ihrer Stadt“ sind keine bewiesenen eigenen Filialen; Versand bundesweit ebenfalls kein weiterer Standort. 7111 ergänzt HRB/Geschäftsführer, behalten. Register-Livestatus weiterhin nicht unabhängig abgeglichen.

### Index 34 — Schützen Goldstube, München — IDs 6710 / 7109

- **Urteil:** Doppelhinweis für denselben Sam-Jaizani-Standort; bündeln, Einzelbeleg.
- **Übereinstimmungen:** Beide nennen **Sam Jaizani**, Schützenstraße 3, 80335 München, 089 23051681 und 0157 77777228, `hallo@goldstube-goldankauf-muenchen.de`, dieselbe Domain/Zahngoldankauf. Das aktuelle [Impressum](https://www.goldstube-goldankauf-muenchen.de/impressum/) bestätigt Betreibername, beide Nummern, E-Mail und Anschrift zusammen.
- **Widersprüche/Abgrenzung:** Zusätzliche `kontakt@…`-Mail und Öffnungszeiten in 7109 sowie Terminsystem/gesonderter Beratungsraum in 6710 sind Ergänzungen, keine neue Betreiberidentität. Allgemeiner Werbetitel „Goldankauf München“ und rechtlicher Name „Schützen Goldstube“ beziehen sich in dieser Primärquelle auf dieselbe Anschrift; daraus keinen separaten Händler ableiten. Kein Register-/zweiter unabhängiger Identitätsbeleg erhoben.

### Index 35 — Goldankauf Goebels, Mönchengladbach-Wickrath — IDs 6730 / 7174

- **Urteil:** Doppelhinweis für Roman Goebels am selben Standort; bündeln, Einzelbeleg.
- **Übereinstimmungen:** Beide Details nennen Roman Goebels, Trompeterallee 7, 41189 Mönchengladbach, +49 2166 6396966, `info@goldankauf-goebels.de`, dieselbe Domain und identischen Ankaufs-/Ladenfall. Das aktuelle [Impressum](https://www.goldankauf-goebels.de/impressum) bindet Goldankauf Goebels an „Herrn Roman Goebels“, Straße/Ort/Telefon/E-Mail; Steuer-Nr. **121/5122/6201**.
- **Widersprüche/Abgrenzung:** „Gold, Silber & Antiquitäten“, Erbschaften, Bar-/Überweisung in 7174 erweitern die Angaben. Kein abweichender Inhaber/Standort und kein Betreiberwechselbeleg. Verzeichnisse/Bewertungszahlen aus 7174 sind keine zweite Belegquelle und kein Identitätsargument. Wickrath ist Ortsteil, keine separate Filiale.

### Index 36 — FairPreis Gold GmbH, Bordesholm — IDs 6733 / 7140

- **Urteil:** Doppelhinweis für **Zentrale/Einlieferadresse Holstenstr. 16**, bündeln. Andere behauptete Standorte getrennt prüfen.
- **Übereinstimmungen:** Beide nennen FairPreis Gold GmbH, Christian Schön, HRB **13723 KI**, Holstenstr. 16, 24582 Bordesholm, 04322 8876321, `info@fairpreisgold.de` und dieselbe Domain. Das aktuelle [Impressum](https://www.fairpreisgold.de/impressum.html) bestätigt diese Daten und USt-ID **DE286433635**. [Ablaufseite](https://www.fairpreisgold.de/ablauf-details.html) nennt Versandadresse **FPG-Logistik, Holstenstr. 16** und persönliche Einlieferung mit Barauszahlung gegen Ausweis.
- **Widersprüche/Abgrenzung:** Versandfokus in 6733 gegenüber Zentrale/Ortsterminen in 7140 ist kein Widerspruch der Identität. „FPG-Logistik“ ist auf der Betreiber-Ablaufseite ausdrücklich Versandanschrift ohne Goldhinweis; nicht daraus einen separaten Ankäufer ableiten. Behauptete fünf Standorte nach Gold.de und Wohnorttermine/Mobilnummer in 7140 wurden nicht als eigene Filialen verifiziert; **keine** Deduplizierung dieser Orte mit Bordesholm.

### Index 37 — Bissendorf Recycling GmbH, Wedemark — IDs 6911 / 7369

- **Urteil:** Doppelhinweis für denselben Platz; bündeln. **7369 enthält wesentlichen fehlenden Primärbeleg**, nicht als inhaltsgleiche Wiederholung verwerfen.
- **Übereinstimmungen:** Beide melden Auf der Haube 5, 30900 Wedemark-Bissendorf, 05130 3767666, Fax 05130 5840405, `bissendorfrecycling@web.de`, Domain, **Timo Kohsen**, HRB **215994 Hannover**. Aktuelles [Impressum](https://www.bissendorf-recycling.de/impressum/) bestätigt alle Betreiber-/Standortdaten, USt-ID **DE315785546**; zudem aktueller Betriebshinweis zum Brückentag 15.05.2026.
- **Inhaltlicher Gegenbeleg:** 6911 sagt, der Ankauf stütze sich nur auf ein Verzeichnisprofil, weil die eigene Website ihn nicht ausdrücklich aufführe. Die erneut gelesene [Betreiberseite Angebot](https://www.bissendorf-recycling.de/angebot/) sagt ausdrücklich: **„Wir kaufen Schrott oder ähnliches von jedem an, auch Abholung gehört zum Programm. Containerstellung ist auch möglich.“** Somit ist 7369 eine belegte Ergänzung/Korrektur der Recherche, kein anderer Betrieb. Zeiten 8–16/Sa geschlossen auf Betreiberseiten versus Verzeichnis 8–17/Sa 9–13 sind Quellenkonflikt, kein Identitätswechsel. Fremdzeiten nicht übernehmen.

### Index 38 — Nando Metalle, Bünde — IDs 6933 / 7325

- **Urteil:** Doppelhinweis für denselben Fernando-Broschinski-Standort; bündeln, Einzelbeleg.
- **Übereinstimmungen:** Beide melden **Fernando Broschinski**, Wilhelmstraße 118, 32257 Bünde, 05223 6552162 **und** 0176 70365360, `nb-metalle@gmx.de`, dieselbe Domain, Alt-/Buntmetall, Katalysatoren und Alt-Pkw. Das aktuelle [Impressum](https://www.nandometalle.de/impressum/) nennt „Nando Metalle, Broschinski Fernando“, Anschrift, beide Nummern und Mail gemeinsam; Websitejahr 2026.
- **Widersprüche/Abgrenzung:** „Ennigloh“ in 7325 ist Ortsteil; zusätzliche Materialliste/Container/Bauzaun ergänzt 6933. Der Schreibfehler „Wilhelmstaße“ im Seitenfooter ändert nicht die im Impressum klare Anschrift. Martin/Peter Broschinski in Visselhövede/Bühren sowie Janus, Mindener Str. 74, werden in beiden Details als Fremdfälle unterschieden; Familienname allein rechtfertigt keine Verbindung. Keine belegte Nachfolgekette erforderlich/erhoben.

### Index 39 — HOFFMANN Altmetallhandel, Schönhorst — IDs 6984 / 7323

- **Urteil:** Doppelhinweis für denselben mobilen Betreiber mit Sitz Steendiek 3; bündeln. **Keine Annahmestelle aus der Sitzanschrift erfinden.**
- **Übereinstimmungen:** Beide nennen **Klaus Hoffmann**, Steendiek 3, 24220 Schönhorst, 04347 8363, `info@mobiler-schrottplatz.de`, dieselbe Domain, Materialsorten und mobile Abholung/sofortige Vergütung. Das aktuelle [Impressum](https://mobiler-schrottplatz.de/impressum/) bestätigt HOFFMANN Altmetallhandel, Klaus Hoffmann, Sitz/Kontakte und USt-ID **DE409401208**. Die [Startseite](https://mobiler-schrottplatz.de/) nennt denselben Ankaufskontakt und ausdrücklich Abholung im Betrieb/auf Baustellen, Wiegen und sofortige Bezahlung.
- **Widersprüche/Abgrenzung:** HTTP/HTTPS und www/ohne www sind URLvarianten desselben referenzierten Betreibers; der aktuelle Impressumsabruf ist der Zuordnungsanker, nicht die Ähnlichkeit der URLs. Verzeichnisname „Maurer- und Betonarbeiten sowie Schrotthandel“ in 7323 ist ein unbestätigter Tätigkeitsalias, kein Beweis für zweite Firma. Andere Hoffmann-Orte und „Mobiler Schrottservice Hamburg-Billstedt“ nicht zuordnen. Baustellen/Abholorte bilden keine jeweils eigene Niederlassung.

## Owner-Gate / erhaltene offene Arbeit

1. IDs nur als Triagebündel behandeln; keine automatische Feedbacklöschung. Zusätzliche Quellen/Materialien/Servicebedingungen aus Zweitmeldungen erhalten.
2. **21/24:** Betreiberexistenz und Identität fehlen nach README; gemeinsamer Recherchefall, keine Freigabe. Aggregator-Daten nicht zu Beweisen aufwerten.
3. **30:** aktuellen Standortanteil bündeln; Altbetreiber „Hellmich Rohstoff GmbH“/Kreisstr. 22/abweichende Nummer als separate offene Identitäts-/Umzugskette weiterführen.
4. **22/29:** historische Firmen-/Alias-Ketten nicht durch heutige Betreiberseite allein als bewiesen erklären. **28/32:** Neben-/Altadressen gesondert erhalten; keine übergreifende Standortdeduplizierung.
5. Primärseitenunterseiten sind eine Quelle; Register-Livestatus und gegebenenfalls unabhängige zweite Identitätsquelle bleiben Voraussetzung für spätere Stammdatenentscheidung. Diese Notiz entscheidet ausschließlich über das Bündeln der angeführten Meldungsfälle.
