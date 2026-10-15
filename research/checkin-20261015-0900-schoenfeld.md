# Check-in 15.10.2026 – Feedback 7385: Schönfeld, Münster

## Ergebnis / Owner-Gate

**Weiter offener Recherchelead, keine Integrationsfreigabe empfohlen.** Für „Schönfeld Metallrecycling Industriedemontage“, Gasselstiege 190, 48159 Münster, ist weiterhin weder die Betreiberidentität noch ein aktueller Metallankauf durch qualifizierte Quellen ausreichend nachgewiesen. Das ist **kein Schließungs- oder Nichtexistenznachweis**. Keine neue Händlerzeile, kein Merge mit SMI und keine Übernahme von Leistungen benachbarter Betriebe empfohlen. Entscheidung liegt beim Owner.

Neu gegenüber der [Vorprüfung vom 14.10.2026](feedback-20261014-nrw-7385-7386.md): kommunalen, kostenpflichtigen Auskunftsweg direkt verifiziert; Creditreform-403 erneut reproduziert; 11880 und SMI-Gegenspur frisch gelesen. **Keine neue positive Identitäts- oder Ankaufsevidenz.**

## Umfang und Belegmaßstab

- README „Feedback triage“ und „Operator rename/merge chains“ angewandt: zwei unabhängige qualifizierte Quellen; Verzeichniskonsens genügt nicht. Die Ausnahme für registerkongruente, aktuelle Betreiber-/Filialseiten ist hier mangels verifizierter Betreiberwebsite und Registerkennung nicht anwendbar.
- Feedback 7385 einschließlich vollständiger `details` frisch aus Produktion gelesen. `internal.db` und `public.db` ausschließlich über SQLite `mode=ro` und zusätzlich `PRAGMA query_only=ON` geöffnet. Keine Nutzerreferenzen in diesen Bericht übernommen.
- Alle unten als frisch gelesen bezeichneten Webabrufe stammen vom 15.10.2026. Suchindex, Vorbericht und Direktabruf sind ausdrücklich getrennt. Keine Kontaktaufnahme, kein Formular abgesendet, keine kostenpflichtige Auskunft bestellt.
- Ausschließlich dieser Bericht wird bearbeitet. Vorbestehende umfangreiche Worktree-Änderungen bleiben unangetastet; keine Dossier-/DB-Änderung, kein Seedlauf, Commit, Push oder Deploy.

## Identität und Adresse: belastbar, plausibel, offen

| Quelle / Abruf | Tatsächlicher Befund | Beleggrenze |
|---|---|---|
| [11880: Metallrecycling](https://www.11880.com/branchenbuch/muenster-westfalen/060440092B53178019/metallrecycling.html), frisch vollständig gelesen | Gasselstiege 190, 48159 Münster (Uppenberg); 0251 22817, zusätzlich 0172 5746298; Mail-Linktitel `juergen-schoenfeld1968@web.de`; Branche Altmetallrecycling; „Website hinzufügen“, „Öffnungszeiten hinzufügen“. | Aggregatorlead, keine Betreiberquelle. Im gelesenen Eintrag keine individuelle Inhaberbestätigung nachgewiesen. |
| [Gelbe Seiten: Schönfeld](https://www.gelbeseiten.de/gsbiz/e32270b2-1dbd-4998-8676-67851462a554), frischer Direktabruf **403** | Kein heute gelesener Profilinhalt. Frischer Suchindex der [Münster-Branchenliste](https://www.gelbeseiten.de/branchen/schrotth%C3%A4ndler/m%C3%BCnster) nennt Schönfeld Metallrecycling Industriedemontage, Gasselstiege 190, 0172 5746298. | Die detaillierte Leistungsliste ist heute nur über Feedback/Vorbericht bekannt, nicht frisch direkt verifiziert. Auch bei Lesbarkeit kein qualifizierter Zweitbeleg. |
| [Creditreform: Jürgen Schönfeld Industriemontagen, 4250600230](https://firmeneintrag.creditreform.de/48159/4250600230/JUERGEN_SCHOENFELD_INDUSTRIEMONTAGEN), frischer Direktabruf **403** | Kein gelesener Firmenauszug; keine frisch bestätigte Rechtsform, Registerkennung, Betriebsaktivität oder Adresse aus diesem Profil. | Die Vorprüfung führt einen Suchindexhinweis auf Gasselstiege 190, „Gewerbebetrieb“, „Reparatur von Maschinen“. Das bleibt ausdrücklich unbestätigte Vorrecherchespur. Maschinenreparatur wäre ohnehin kein Metallankaufsnachweis. |
| [Stadt Münster: Gewerberegisterauskunft](https://www.stadt-muenster.de/ordnungsamt/gewerbe/gewerberegisterauskunft), frisch direkt gelesen | Zuständige Stadt beschreibt Auskunft nach § 14 GewO bei berechtigtem Interesse, Ermessensentscheidung ohne Rechtsanspruch und 20 EUR Gebühr; verlinkt Online-Anfrage. | Qualifizierte Primärquelle **für das Auskunftsverfahren**, nicht für die Existenz dieses Gewerbes. Kein tatsächlicher Registerauszug erhalten. |

**Zeitstempelwarnung:** 11880 nennt im Direktabruf „aktualisiert am 03.10.2026“, „Eintragsdaten vom 12.11.2025“. Die Suchmaschine lieferte für dieselbe URL dagegen einen älteren Juli-2024-Text. Maßgeblich für die Abrufbeschreibung ist der gelesene Seiteninhalt, aber auch sein Aktualisierungsdatum beweist keinen aktiven Betrieb.

Die Mail macht „Jürgen Schönfeld“ als Personenkandidaten plausibel; sie beweist nicht, dass die Geschäftsbezeichnungen „Industriemontagen“ und „Metallrecycling Industriedemontage“ rechtlich demselben gegenwärtigen Betreiber gehören. Keine belegte Umbenennungs-, Übernahme- oder Verlegungskette. Kein Impressum, kein nachweisbar betreibergeführtes Social-Profil und kein direkt gelesener Gewerbe-/Registereintrag zum Ziel gefunden.

Zusätzliche Suchläufe über Name/Ort, Industriemontagen, Nummer `01725746298`, Mail und Facebook erbrachten keinen zurechenbaren Betreiberbeleg. Einige exakte Abfragen lieferten sachfremde Ergebnisse; die Northdata-Site-Abfrage schlug technisch fehl. Daher ausdrücklich **keine vollständige Register-Negativprüfung** behauptet. Creditreform-Empfehlungslisten bei anderen Firmen erschienen im Suchindex mit „Jürgen Schönfeld Industriemontagen, 48159 Münster“; solche Nachbarschafts-/Empfehlungslisten belegen weder Zieladresse noch Tätigkeit und wurden nicht als Auszug gezählt.

## Annahme, Ankauf und getrennte Preisrichtungen

| Gegenstand | Ergebnis für Schönfeld | Fehlender konkreter Beleg |
|---|---|---|
| Metallannahme / Ankauf | Nur Branchen-/Verzeichnisbehauptung Altmetallrecycling bzw. Schrotthandel. Aktuelle vergütete Annahme nicht primär belegt. | Aktuelle Betreiberangabe mit zurechenbarer Identität und tatsächlich angenommenen Materialien; Unterscheidung Ankauf, kostenlose Übernahme und kostenpflichtige Entsorgung. |
| Anlieferung / Abholung | Weder öffentlich zugänglicher Hof an Gasselstiege 190 noch Abholbetrieb verifiziert. Container/Demontage allein beweisen keine Hofannahme. | Bestätigte Betriebs-/Annahmestelle oder Abholbedingungen, Privat-/Gewerbekunden, Mengen, Terminpflicht, Erreichbarkeit und Öffnungszeiten. |
| **Ankaufspreisliste** | Keine verifizierte numerische, Schönfeld zurechenbare Liste gefunden. | Sorten/Qualitäten, EUR je Einheit, Gültigkeit/Datum, Mengenstaffeln und Abzüge. Keine allgemeinen Marktpreise ersatzweise zuordnen. |
| **Verkaufspreisliste** | Keine verifizierte Liste für ausgehende Materialien/Waren gefunden. | Eigenständige Betreiberverkaufsliste mit Einheit, Steuer-/Mengenbedingungen. Nicht aus „Schrotthandel“ ableiten. |
| **Gebühren / Dienstleistungen** | Keine verifizierte Container-, Transport-, Demontage- oder Entsorgungsgebührenordnung gefunden. Keine Kostenfreiheit belegt. | Zurechenbare Gebührentabelle oder Leistungsangebot mit Bedingungen; negative Entsorgungspreise nicht als Ankaufskurse ausgeben. |

Die 11880-FAQ „Wo sehe ich, welche Preise …“ erklärt lediglich generische Euro-Symbol-Preisranges und enthält keine Materialkurse. **Kein Preisdatensatz und kein Handler vorgeschlagen.** Nichtfund bedeutet nicht, dass keine individuell vereinbarte Vergütung existiert. Die oben genannten **20 EUR betreffen ausschließlich die städtische Registerauskunft**, nicht eine Händlergebühr.

## Doppelanlagen- und Namensvetterprüfung

Frischer Produktionsscan über sämtliche `traders`-Spalten (inkl. `notes`, `description`, `extra_json`, Unicode-normalisiert) nach `schonfeld`, `schoenfeld`, `gasselstiege`, `5746298`, `22817` sowie SMI-Slug. Zusätzlich bundesweite Dossier-Volltextsuche nach Umlaut-/ASCII-Namen, Straße und Nummernfragmenten. **Keine Zielanschrift-/Kontakt-Dublette gefunden; keine absolute Dublettenfreiheit behauptet.**

- **SMI Metallrecycling**, `nw-munster-smi-metallrecycling`, Produktion ID **1456**, bleibt ein gesonderter ungeklärter Bestandskandidat (`pruefung`). Straße, PLZ und Telefon sind in den aktuellen Produktionsfeldern leer; Am Schild 11 steht in den Notizen. [WLW](https://www.wlw.de/de/firma/smi-metallrecycling-1786787), frisch direkt gelesen, nennt Am Schild 11, 48159 Münster, Geschäftsführung **Can Ikram**, Gründung 2012. Das bleibt ein Verzeichnislead, kein gesicherter Betreiberbeleg. [SMI-Domain](https://smi-metallrecycling.de/), frisch direkt gelesen: nur **„BALD VERFÜGBAR“**, kein Identitätsinhalt. Keine Brücke zu Jürgen Schönfeld/Gasselstiege und keine bewiesene Verlegung. Sprakeler Str. 34/0176 32738749 aus dem Feedback nicht als aktuell bestätigt übernehmen. SMI-Koordinaten nicht als Zieladressbeleg benutzen.
- **Kollmann GmbH Metallrecycling**, `by-lappersdorf-kollmann`, Produktion ID **362**: Namensfund ausschließlich in der historischen Notiz zu Geschäftsführer **Christoph Schönfeld**, Am Kuffholz 16, 93138 Lappersdorf. Kein Treffer auf Zieladresse oder Zielkontakte. Keine Münster-Filiale und keine Personenidentität mit Jürgen daraus ableiten. Einziger Schönfeld-Treffer der Dossier-Volltextsuche ist diese Kollmann-Timeline; hier keine frische Registerverifikation der Kollmann-Gesellschaft behauptet.
- Suchergebnisse zu **SRM Schrott & Metallrecycling Münster GmbH**, Kesslerweg 37, 48155 Münster, gehören ebenfalls nicht zum Lead. Deren sichtbare Betreiberwerbung für Vergütung/Anlieferung darf nicht Schönfeld zugeschrieben werden; generische Suche „Metallrecycling Münster“ liefert überwiegend SRM statt Schönfeld.

## Konkreter nächster Evidenzbedarf für den Owner

1. **Identität/Gewerbestatus:** Nach Owner-Freigabe kommunale Auskunft gezielt zu „Jürgen Schönfeld / Schönfeld Metallrecycling Industriedemontage / Industriemontagen, Gasselstiege 190, 48159 Münster“ anfragen. Erbeten: tatsächliche Geschäftsbezeichnung, Inhaber, aktuelle Betriebsanschrift, angemeldete Tätigkeiten und ggf. Abmelde-/Verlegungsstatus, soweit zulässig. Stadt stellt [Online-Anfrage](https://formulare.stadt-muenster.de:443/metaform/Form-Solutions/sid/assistant/6476ea1cacb27b60607c0479) bereit; Formular hier nicht abgeschickt oder auf Funktionsfähigkeit geprüft. 20 EUR Kosten und Zulässigkeit bleiben Owner-Entscheidung.
2. **Unabhängige aktuelle Betreiberquelle:** Nachweisbar betreibergeführte Website/Social-Seite oder anderweitig nach README qualifizierte zweite Quelle finden, die explizit die Namenskette und Metallannahme/Ankauf trägt. Eine Auskunft nur zu Maschinenreparatur beseitigt den Ankaufblocker nicht. Weitere Verzeichnisse oder weitere Seiten derselben Quelle erfüllen das Gate nicht.
3. **Annahmebedingungen / Preise:** Erst nach Identitätszuordnung materialbezogene Annahme, Kundenarten, Hof versus Abholung, Zeiten und Mengen klären; Preislisten getrennt nach Ankauf/Verkauf/Gebühren anfordern bzw. verifizieren. Weder vermeintliche Tagespreise noch pauschal kostenlose Abholung ergänzen.
4. **Vor Integration:** Zielkontakte und Aliasnamen erneut gegen Dossiers und aktuelle Produktion prüfen; keine SMI-/Kollmann-Zusammenführung ohne nachgewiesene Betreiberbrücke. Fehlende Evidenz nicht durch ein neues `pruefung`-Dossier umgehen.

**Handoff:** Bericht abgeschlossen, Händlerprüfung weiterhin evidenzbedingt offen. Kein Status in `feedback` geändert; Integration und etwaige kostenpflichtige Nachrecherche ausschließlich durch Owner.
