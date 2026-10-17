# Unabhängige Owner-Endprüfung: alte Wave 13-A, 17.10.2026 / 21:00

## Entscheidung

**Redaktionelle scoped Integrationserlaubnis für genau die unten aufgeführten 20 alten Dossiers und ihre zwei Begleitberichte. Kein verbleibender redaktioneller Blocker innerhalb dieses Korrekturauftrags.** Die vier zwingenden Korrekturen aus dem 17:00-Review sind tatsächlich umgesetzt; die weiteren Einschränkungen stehen ausdrücklich im aktuellen Überblick und im jeweils vorgeschalteten Korrekturhinweis. Die erhaltenen alten Aussagen werden dadurch als historische, teils korrigierte Claims eingeordnet, nicht nachträglich bestätigt.

Diese Erlaubnis ist **keine bestandene technische Integrations-/Releasefreigabe**: Der README-Seedtest ist hier ausdrücklich nicht ausgeführt und bleibt vor einer späteren Integration beim Owner offen. Keine Freigabe neuer Händlerfakten, Aktivierungen, Schließungen, öffentlicher Annahmestellen, Identitätsketten oder alter Aggregator-Fills. Die redaktionelle Korrektur benötigt keine erneute Recherche zur vollständigen Auflösung dieser Fälle; deren offene Identitäten sind Teil des zulässigen Ergebnisses.

## Prüfgrundlage und tatsächliche Integrität

- Vollständig gelesen: `research/checkin-20261017-1700-wave13-a-review.md`, `research/checkin-20261017-2100-wave13-a-corrections.md`, sämtliche 20 tatsächlichen `git diff HEAD -- <Dossier>` sowie der korrigierte alte Wellenbericht. Die Prüfung beruht nicht allein auf dem Korrekturhandoff.
- Geprüftes HEAD: `dbb61a659a9a2f62506685cd7b2b6dd161f4c29a`. Backup-HEAD laut Manifest: `deca75f9e11d2446934632a96d99d2bb4f61f155`. Der explizit auf diese 20 Pfade begrenzte Vergleich zwischen beiden Commits ist leer; die zwischenzeitliche HEAD-Bewegung verändert diese Prüfgrundlage nicht.
- Backup: `/root/Documents/schrott-wave13-a-20261017-2100-backup-zd5r07yh/`; `manifest.json` und `after-SHA256SUMS` unabhängig gegen die tatsächlichen Bytes abgeglichen.
- **20/20 Backupdateien** stimmen mit ihren Vorher-SHA-256 im Manifest überein.
- **20/20 vollständige Dateipräfixe vor `## Überblick` sind bytegleich zum Backup**, einschließlich des vollständigen Frontmatters, seiner Reihenfolge und Quotierung. Seit der Sicherung keine skalare Änderung.
- **20/20 bisherige Timelineinhalte sind als vollständiger bytegleicher Suffix erhalten**, einschließlich der 13.10.-Abschnitte und sämtlicher älterer Historie. Vor dem jeweiligen Altblock steht genau ein neuer Abschnitt `### Ownerkorrektur 17.10.2026`. Auch die jeweilige HEAD-Timeline ist unverändert als Suffix erhalten.
- **22/22 Nachherhashes** aus dem externen `after-SHA256SUMS` stimmen mit den aktuellen Repositorydateien überein: 20 Dossiers, alter Wellenbericht und Korrekturhandoff. Damit gilt diese Entscheidung genau für den dort festgehaltenen Stand; bei späteren Änderungen erneut prüfen.
- Alter Wellenbericht: ursprünglicher Titel erhalten; der ursprüngliche Inhalt nach der ersten Leerzeile ist vollständig bytegleich als Suffix erhalten. Dazwischen wurden Ownerhinweis und historische Abschnittsüberschrift eingefügt. Der komplette Originalbericht ist deshalb nicht mehr ein zusammenhängender Byteblock; es wurde aber kein ursprünglicher Text umgeschrieben oder entfernt. Der im Manifest gespeicherte Originaltext hat den erwarteten Vorherhash `9d25fd01f3c256f47c3abed40d104c00b6446d759151965687489b0cb158d9da`, die aktuelle Datei `aef04fd06f63b9c686796819df0fc62341719f6d2e1db0ec6d11da0e2d9a6584`.
- Rein textueller `git diff --check HEAD -- <exakte 20 Pfade>` ohne Befund. Das ist kein Code-, Build- oder Seedtest.

## README-/Format-Gate

Maßstab: `README.md` (Dossiers als Build-Input, Quellenhierarchie, autoritative Ausnahme, Betreiberketten, Owner-Gate), ergänzend `docs/trader-seed.md` und `prompts/tiefenrecherche-welle.md` für die ausdrücklich genannten Prüfarten.

### Felder und Overwrites

Der vollständige Frontmatter-Diff gegen HEAD umfasst ausschließlich:

- `status: aktiv → pruefung` bei **Gramet und Wolf**.
- `website_status: '' → unbekannt` bei **Storkow, Wolf, Albert, Bach, Kreutz, Maik, Breyer, Eftekhari, Wenner, Trier-West und RWR** (11).
- Sonst keine Frontmatteränderung; acht Dossiers bleiben gegenüber HEAD dort vollständig unverändert. Keine neuen Schlüssel, Slugs, Namen, Anschriften, Kontakte, Website-URLs, Öffnungszeiten, Leistungen, Zertifikate oder Koordinaten. Insbesondere kein WSH-Telefon-Fill und kein Bassenheim-/Börrstadt-/Rohrbach-Transfer.

Die zwei Herabstufungen sind dokumentierte Evidenzvorsicht, keine Schließung. Die elf Zustandspräzisierungen sind keine Betriebsverifikation. Eine technische Ungültigkeit des früheren leeren Website-Enums wird nicht behauptet. Die gegenwärtigen Statuswerte sind `pruefung` und beim unveränderten Trapp `aktiv`; Website-Zustände `unbekannt`, `tot` und `aktiv` sind zulässige Werte. Typen und Bundeslandcodes sind im geprüften Umfang unverändert und entsprechen den vorhandenen Enumgruppen. Dies ist eine statische Sichtprüfung, keine Behauptung bestandener Korpusvalidierung.

### Deep-Links, Emdash und Noten

- Die neuen Korrekturen enthalten reproduzierbare Detail-/Profilverweise, soweit solche im Review vorlagen: insbesondere WSH-11880, Gramet-Publikationszuordnung, Eftekhari-Registerprofil, Kreutz-Einzelanzeige, Radke-Detaillead sowie Maik/MCR-Profilverweise. Suchleads, indirekt verlinkte Einzelprofile, 403, Transportfehler und textlose Social-Abrufe sind als solche gekennzeichnet, nicht als gelesene Betreiberbelege.
- Radkes alte Portal-Roots und Wenners alter Kleinanzeigen-Root bleiben ausschließlich qualifizierte Historie, keine aktuellen Einzelbelege. Storkows Such-/Straßenseiten dienen ausdrücklich nur der Lead-/Namensgegenprobe. Maiks tatsächlich gelesene Übersichtsseite belegt die Anordnung der Portalkarten, nicht einen Betrieb. Trapps Root ist selbst die zitierte Startseitenquelle; hier ist ein Root-Link kein Ersatz für eine nicht belegte Detailaussage.
- Betreiber-Unterseiten werden nicht zu unabhängigen Zweitquellen hochgezählt. NorthData bleibt der README-zulässige Registerkanal, kein beglaubigter Auszug. Die autoritative Betreiber-Ausnahme wird nirgends ungeprüft zur Feldfreigabe verwendet.
- Kein U+2014-Emdash in neu hinzugefügten Dossier-Diffzeilen. Die bestehenden Emdashes in `provenance_section` bei Demand/Trapp sind unveränderte Herkunftsprosa, keine Platzhalter in sachlichen Kontakt-/Adressfeldern. Keine neuen Emdash-Platzhalter oder Notenformatprobleme festgestellt.
- Neue Hinweise stehen als datierte `- [Korrektur 17.10.2026: … Quelle(n): …]` unter `## Timeline`; der Build kann sie als Noten übernehmen. Deshalb ist ihre ausdrückliche Einschränkung alter Sicherheit relevant. Die historischen Fehlinterpretationen sind nicht gelöscht, aber konkret korrigiert; ihre frühere Aussage darf nicht isoliert als heutiger Nachweis gelesen werden.

## Entscheidung je tatsächlichem Dossier-Diff

Pfadpräfix der ersten sieben Zeilen: `dossiers/bb/`, danach `dossiers/rp/`; jeweils Dateiendung `.md`. **Alle 20 Zeilen sind im genannten redaktionellen Umfang freigegeben**, nicht als abgeschlossene Händlerverifikation.

| Exakter Dateistamm | Geprüfte Korrektur und ausdrücklich offene Grenze |
|---|---|
| `bb-eberswalde-seik-automobilrecycling` | Hausnummern 1/1A nur Leads; Herten-Betriebsende ausdrücklich historischer 03.10.-Befund, Reviewabrufe Transportfehler. Eberswalder Betreiber und Annahme offen; keine Schließungsübertragung. |
| `bb-gramzow-gramet-schrott-u-metallhandel` | Profil-Löschung historisch; 108975156 als Eintragung statt Löschung korrigiert, Löschungsverweis 1096675191 und ungeprüfte Einzeltexte benannt. Keine Brücke zu Szymon Mach/Prenzlauer Str. 3; Downgrade wegen fehlender aktueller Absicherung. |
| `bb-lauchhammer-radke-ulrich` | Konkreter Telefonbuchlead ergänzt, Roots und frühere „verifiziert“-Aussagen begrenzt; Status schon am 07.10. integriert. Betreiber, Aktivität, Ankauf und Annahme offen. |
| `bb-lauchhammer-wsh-schrotthandel` | Zwingender Blocker behoben: konkreter 11880-Lokal-Lead mit Portalmetadaten relativiert alte Trefferlosigkeit. Kindsbach nicht verbunden, kein Telefon-Fill, kein Betriebs-/Annahmenachweis. |
| `bb-storkow-utmar-koch-ralf` | Fehlende qualifizierte Belege statt fehlender jeglicher aktueller Quellen; abweichende Namensformen und unbestätigter Social-Kanal ausdrücklich offen. |
| `bb-wandlitz-wolf-abschleppdienst` | Kategorienvergleich als zeitabhängig/historisch eingeordnet, Places nicht betreiberverifiziert; Prüfstatus statt behaupteter Schließung. Betreiber und öffentliche Fahrzeugannahme offen. |
| `bb-zossen-autoverwertung-klucke` | Portalstruktur Klucke Nr. 2 / HEBE Nr. 4 korrekt eingeordnet; nur Portalinhalt bestätigt. HTTP 410 historisch; Betreiber, heutiger Standort und individuelle Anerkennung offen. |
| `rp-bechtolsheim-ayan-bechtolsheim` | Keine belegte Identitätsbrücke statt sicher getrennter Rechtsträger; Börrstadter Impressum mit Platzhaltern nicht auf Bechtolsheim übertragen. Alte Tagespreiswerbung kein neuer Fund. |
| `rp-bendorf-albert` | Jasari-Mail ungeprüfter Snippet; Bewertungsdifferenz kein Abhängigkeitsbeweis. Alte Aggregator-Ausnahme ausdrücklich keine aktuelle Feldverifizierung, Jasari-Zusammenhang offen. |
| `rp-geinsheim-sudpfalz-metall-geinsheim` | Fremder Adressnutzer schließt Mit-/Vornutzung nicht aus, keine sichere Trennung oder Zuordnung zu Rohrbach. Altdomainbefund historisch, keine Betriebsschließung. |
| `rp-kinderbeuern-bach-kinderbeuern` | Domain bereits seit 06.10., Transportfehler keine Eigentums-/Dauerverfügbarkeitsaussage; Bausendorf kein Umzug. Alte Verzeichnisverifizierung relativiert, Mindestmengen keine Kurse. |
| `rp-koblenz-56073-kreutz-schrotthandel` | Neue konkrete Anzeige nur Suchlead mit 403; „geschlossen“ ohne Auflösung kein Aufgabe-Beweis. Adresskonflikt und Urheberschaft offen, kein Kontakt-Fill. |
| `rp-koblenz-karthause-56075-schrotthandel-maik-alten` | Getrennte Maik-/MCR-Portalkarten nachvollziehbar, kein neuer Standort-/Umzugs-/Identitätsbeweis. 24-h- und Mengenangaben unbestätigt. |
| `rp-landau-autoverwertung-demand-recycling` | Bereits bekannte 50–2.000 € abhängig von Fahrzeugmerkmalen, keine Mindestgarantie; 5 €/Müllsack, unbezifferte Abmeldung, kostenlose Abholung Landau/SÜW sauber getrennt. Register-/Nachfolgekette und Zertifikat offen; HRA/HRB allein entscheidet Ausnahme nicht. |
| `rp-limburgerhof-breyer` | Begrenzte erfolglose Suche, kein Nichtexistenz- oder Quellenabschluss. Namensähnliche Domain nicht als identifizierter Limburgerhofer Betreiber übernommen. |
| `rp-mainz-55120-eftekhari` | Zwingender Blocker behoben: aktuelle Wiesbaden HRB 34887 und frühere Mainz HRB 48328 dieselbe Gesellschaft laut Profil. Akazienweg-Seedmatch offen, Liquidation nicht als Seed-Schließung übertragen. |
| `rp-trier-54294-wenner-david` | Social-Folgehinweis aufgenommen, Direktabruf ohne Text; altes „kein Social“ begrenzt. David/Dennis, Inseratsidentität und Annahmeadressen offen. |
| `rp-trier-m-trier-west` | Malformierter Importstring als Datenfehler, kein Existenzbeleg; keine gesicherte Trennung zu Theo Steil behauptet. Identität und strukturierter Straßenwert bleiben ausdrücklich Owner-Klärfall. |
| `rp-weienthurm-rwr-weienthurm` | Zwingender Blocker behoben: echte EUID unter falscher USt-Überschrift, keine verstümmelte Kennung/erfundene USt-ID. Bassenheim-Registerabgleich trägt keine Weißenthurm-Kette; kostenloses Angebot keine Gratislogistik. |
| `rp-westheim-rohstoffe-trapp` | Zwingender Blocker behoben: kein unbelegtes „nur“, kleine Mengen nicht ausgeschlossen, kein pauschales Mengen-/Gratisversprechen. Betreiberanschrift kein bestätigter Abladehof; unabhängige Identität offen, alte Standortfehler nur historisch. |

## Exakter Integrationsumfang und Rest-Gates

Erlaubter bestehender Änderungsumfang: **die 20 obigen Dossiers**, `research/wave-20261013-a.md` und `research/checkin-20261017-2100-wave13-a-corrections.md` (22 Dateien, identisch zum externen Nachhermanifest). Diese neue Datei `research/checkin-20261017-2100-wave13-a-owner-final.md` ist als **23. Datei ausschließlich Prüfbeleg**. Das unveränderte 17:00-Review ist Referenz, keine neue Änderung dieser Prüfung. Keine pauschale Freigabe des breiten Arbeitsbaums, kein `git add -A`; neue 60er-Welle vollständig ausgeschlossen.

Weiterhin offen, ohne redaktionellen Rücksprung: unabhängige Identitäts-/Annahmebelege; historische Aggregator-Felder insbesondere Gramet, Radke, Albert und Bach; Trier-West-Straßenfehler; unveränderter Trapp-Aktivstatus und alte Standortfehlerhinweise. Sie werden durch diesen Text weder neu bestätigt noch stillschweigend repariert. Nach README ist dies keine abgeschlossene Neuverifikation dieser Bestandsfelder. Ebenso beweisen leere Servicefelder und Prüfstatus nicht, dass alte gespeicherte Servicebehauptungen entfernt wären (`docs/trader-seed.md`); ein solches Clear ist nicht Gegenstand der Erlaubnis.

**Vor technischer Integration bleibt der ausdrücklich separate Owner-Seedtest erforderlich.** Ein bestandener Test würde nur Technik, nicht offene Identität oder Quellenlage klären. Kein Release-/Prod-Gate hier als bestanden ausgeben. Bei geändertem Stand gegenüber den geprüften Nachherhashes scoped Diff erneut prüfen.

In dieser unabhängigen Prüfung wurde ausschließlich diese Ergebnisdatei angelegt. Keine Tests, keine Webrecherche, keine Produktionszugriffe, keine GEO-Arbeit, keine Dossieränderungen, keine Commits/Pushes oder Deployments. Die Auswahl und Inhalte der neuen 60er-Welle wurden nicht bearbeitet.
