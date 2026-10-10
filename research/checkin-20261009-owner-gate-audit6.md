# Owner-Gate Audit 6 — neun vorhandene Dossiers

**Prüfdatum: 09.10.2026. Ergebnis: bedingte Freigabe, keine pauschale Käufer-/Frontmatter-Freigabe.** Die neuen Material-/Servicehinweise sind überwiegend durch direkt abgerufene Betreiberseiten bestätigt. Vor Integration sind die unten genannten Textkorrekturen nötig; Ramm hat außerdem einen aktuellen Frontmatter-Widerspruch. Alle vorhandenen Dossieränderungen bleiben unverändert für die Owner-Integration erhalten. In diesem Arbeitsgang wurde ausschließlich dieser Bericht angelegt; keine Datenbankänderungen, Builds, Tests oder Deployments.

## Maßstab und Umfang

- Geprüft: Working-tree-Diff gegenüber HEAD und vollständiges Frontmatter der neun beauftragten Dossiers; Kontext aus `checkin-20261009-audit6-{metallnebenprodukte,logistik,rest}.md`.
- README, insbesondere Zeilen 173–192 und 222–230: Verzeichnisse sind Leads; zwei unabhängige zulässige Quellen oder vollständig belegte authoritative-primary-Ausnahme. Mehrere Unterseiten derselben Domain sind **keine unabhängigen Quellen**. Identitäts-/Registerabgleich ist kein unabhängiger Beleg dafür, dass jede Materialcharge gekauft wird.
- Betreiber-Leistungsseiten wurden live abgerufen. Ergänzend Impressum/Kontakt und für TAE, Wittmann, Brito, Stienemann und Feinhütte NorthData gegengeprüft (nach README zulässiger Registerbeleg). NorthData bestätigt Identität/Adresse, nicht aktuelle sortenbezogene Ankaufkonditionen. Kein Telefoncheck, kein amtlicher Registerauszug, keine erneute Prüfung aller Altzertifikate oder Koordinaten. Ein Copyright/Javascript-Tagesdatum beweist keine Aktualisierung des Angebots.
- Sieben Änderungen betreffen nur Timeline. Nur **Brito und Stienemann** füllen bisher leere `description`-Felder. Slugs, Geschichte und übrige Frontmatter bleiben im vorliegenden Diff erhalten. Auch unveränderte Frontmatter-Felder sind nicht automatisch sachlich freigegeben; konkrete Widersprüche stehen unten.

## Einzelprüfung und Integrationskorrekturen

### 1. TAE Neidenstein — Materialhinweis freigeben

Datei: `dossiers/bw/bw-neidenstein-74933-tae-wertstoffe.md`

**Primärbeleg:** [Materialankauf](https://tae-wertstoffe.de/materialankauf.html): „Wir kaufen, was andere entsorgen“, „Kunststoffe: ab 2 t, sortenrein“, Tabelle „Ankaufsfraktionen · Stand Juli 2026“. PP/PE: Angüsse, Formteile, Mahlgut, Folienreste, „keine Mischung“; ABS natur/gefarbt; PA6/PA66 auch glasfaserverstärkt; POM sortenrein. Weitere Thermoplaste/Metallsorten **nach Rücksprache**. Angebot nach Foto/Menge, €/t verwogenes Netto, Abholung ab entsprechender Menge nach Absprache. Das ist echter Ankauf, nicht bloß Lohnvermahlung oder kostenlose Entsorgung.

**Grenzen:** 2 t gilt für Kunststoff, nicht global für Metall/Hartmetall. [Wertstoffhof](https://tae-wertstoffe.de/wertstoffhof.html) bestätigt Kunststoff ausschließlich Gewerbe und privaten Metallankauf ohne Termin in Neidenstein; Warenanlieferung Mo–Fr 08:15–16:30, allgemeine Öffnung bis 17:00. Kein pauschaler Kunststoffpreis und keine garantierte Abholung ab genau 2 t.

**Frontmatter:** vorhandenes `dropoff_json` und `opening_hours` sind mit beiden Seiten kongruent. [Impressum](https://tae-wertstoffe.de/impressum.html) nennt HRB 746320 Mannheim, Fuchslochweg 2; [NorthData](https://www.northdata.de/TAE%20Wertstoffe%20GmbH,%20Neidenstein/Amtsgericht%20Mannheim%20HRB%20746320) bestätigt Firma/Adresse und Sitzverlegung von Ittlingen nach Neidenstein 2023. Damit ist die ältere Timeline-Notiz „Zweitquelle fehlt“ für Identität/Adresse überholt, nicht rückwirkend zu löschen. `status: pruefung` kann konservativ bleiben.

**Korrektur/Präzisierung:** In neuem Bullet „und weitere Fraktionen ab 2 t“ besser ersetzen durch „die genannten Kunststofffraktionen ab 2 t; weitere Thermoplaste nach Rücksprache“. Juli 2026 ist Tabellenstand, nicht pauschales Publikationsdatum der gesamten Website. Optional neuen Identitäts-Nachtrag mit Impressum + NorthData ergänzen; keine Statushochstufung erforderlich.

### 2. Aulfinger Stuttgart/Möglingen — freigeben, Gefahrstoffgrenze ergänzen

Datei: `dossiers/bw/bw-stuttgart-moglingen-jorg-aulfinger.md`

**Primärbeleg:** [Abfälle, Abschnitt Aluminium](https://www.aulfinger.de/de/leistungen/abfaelle): „jede Form von Aluminiumguss oder Aluminiumkrätze aus Produktion und Handwerk“; „Dabei vergüten wir Ihnen Tagespreise entsprechend der Aluminiumbörse“. Ankauf von Aluminiumkrätze ist ausdrücklich belegt. Fehlguss/Gießereirücklauf wird nicht ausdrücklich als eigene Qualität genannt; vorhandene Abgrenzung ist korrekt. Keine numerischen Preise.

**Grenzen/Frontmatter:** Die Seite bestätigt die zwei bisherigen Adressen und standortspezifischen Stunden. Allgemeiner NE-Metalltext nennt Mengen ab 1 kg, ist aber keine sichere universelle Mindestmenge für jede Krätzecharge. Derselbe Betreiber warnt: Die meisten gefährlichen Abfälle dürfen an den Wertstoffhöfen nicht angenommen werden; Abholung kann direkt zu zugelassenen Anlagen erfolgen. Deshalb aus „jede Form“ keine bedingungslose Annahme gefährlicher Aluminiumkrätze ableiten. Das kombinierte `city`-/Stundenfeld ist ein bestehender Zweistandortdatensatz, keine neue Zusage beliebiger Annahme am HQ.

**Konkreter Zusatz:** „Die allgemeine Einschränkung für gefährliche Abfälle an den Wertstoffhöfen bleibt zu beachten; konkrete Krätzecharge/Abfallschlüssel vor Anlieferung klären.“ Frontmatter im Audit unverändert lassen.

### 3. FMR Frankenberg Emskirchen — freigeben nach Quellenverstärkung

Datei: `dossiers/by/by-emskirchen-frankenberg.md`

**Primärbeleg:** [Zink, Zinn, Blei](https://fmr.ag/de/leistungen/zink-zinn-blei) listet Zn-Hartzinkblöcke, Zn-Leg.-Abschöpfblöcke sowie Krätze aus Sn-Cu, Sn-Pb, Pb-Sn und Sn-Ag; **Pasten stehen konkret bei Sn-Ag**, nicht bei jeder Legierung. Die Seite formuliert Recycling/Entsorgungslösungen, der Keywordblock enthält „Ankauf“; allein daraus folgt keine Vergütungszusage für jede gelistete Charge.

**Wichtiger zusätzlicher Betreiberbeleg:** [Allgemeiner Metallhandel](https://fmr.ag/de/leistungen/metallhandel) fordert ausdrücklich „Ankaufs- und Verkaufsanfragen von Nichteisenmetallen und Eisenschrott jeglicher Form und Art“ an und listet **„Lötzinnkrätzen“**, „Lötzinnstangen“ und „Lotpasten“. Damit ist FMR über den bisherigen Bericht hinaus ein ausdrücklich benannter Lötzinnkrätze-Kandidat im Ankaufsprogramm. Ankauf bleibt anfrageabhängig, keine pauschale Auszahlung/Qualitätsgarantie.

**Grenzen:** Keine Zinkasche ausdrücklich genannt. Metallische/Filterstäube und Spritzverzinkungsrückstände sind keine automatische Zinkasche-Synonymie. Kleine Mengen bis Komplettladungen im allgemeinen Metallhandel, keine spezielle Hartzink-/Krätze-Mindestmenge. Nicht auf unbekannte Legierungen ausweiten.

**Konkrekte Ersatzformulierung im neuen Bullet:** „Die Spezialseite listet Hartzinkblöcke und legierungsbezogene Zinnkrätzen; die Metallhandelsseite nennt Ankaufsanfragen und ausdrücklich Lötzinnkrätzen/Lotpasten. Konkreter anfrageabhängiger Ankaufskandidat, keine Zusage einer Vergütung jeder Charge; Pasten insbesondere bei Sn-Ag genannt.“ Beide URLs zitieren. Kontextbericht Metallnebenprodukte kann FMR zusätzlich unter #6073 aufnehmen.

**Frontmatter:** Audit ändert nichts. `name: Frankenberg` ist gegenüber bereits dokumentierter GmbH unspezifisch, kein neuer Identitätsfehler des Auditdiffs. Optional später auf `Frankenberg-Metallrecycling GmbH` präzisieren und vorhandenen Impressums-/Registerbeleg explizit dokumentieren; **Slug unverändert**. Öffnungszeiten ohne Wochentage weiterhin nicht erfinden. Die alte Provenienz „Ankauf unklar“ bleibt historische Herkunft, nicht aktuelle Gegenbehauptung.

### 4. Wittmann Geisenhausen — Ankauf freigeben, Öffnungszeiten nachtragen

Datei: `dossiers/by/by-geisenhausen-lorenz-wittmann.md`

**Primärbeleg:** [Textil-Recycling](https://www.wittmann-recycling.de/textil-recycling/): „Ankauf von Sammelware“, „Wir übernehmen die gesammelte Ware zu marktgerechten Preisen“; Verladung **beim Sammler vor Ort** in Wechselbrücken/Abrollcontainern. Gemeinnützige, kommunale und gewerbliche Sammelpartner sind belegt. Die Aussage „kein belegter Ankauf privater Einzelmengen“ ist korrekt; nicht zu „Privatkunden werden ausdrücklich ausgeschlossen“ verschärfen. Keine Mindesttonnage genannt.

**Standort:** Firmenzentrale Geisenhausen bestätigt, kein Beleg für beliebige Walk-in-Textilannahme dort oder in Ergolding. Zwei Dossiers desselben Rechtsträgers sind nicht zwei unabhängige Käufer.

**Neue Frontmatter-Erkenntnis:** Die geprüfte Leistungsseite hat jetzt ausdrücklich „Zentrale Geisenhausen“ mit Stunden **Mo–Do 07:30–12:00 / 13:00–17:00; Fr 07:30–13:00**. Die alte Aussage „nur Ergolding, Zentrale nicht publiziert“ ist aktuell überholt. [Impressum](https://www.wittmann-recycling.de/impressum/) und [NorthData](https://www.northdata.de/Lorenz%20Wittmann%20GmbH,%20Geisenhausen/Amtsgericht%20Landshut%20HRB%201526) bestätigen HRB 1526, Vilsbiburger Str. 70–74; NorthData nennt Publikation eines Abschlusses am 31.03.2026. Kein Namesake-Wechsel erkennbar.

**Integrationsvorschlag:** `opening_hours: 'Zentrale: Mo-Do 07:30-12:00, 13:00-17:00; Fr 07:30-13:00; keine belegten Warenannahmezeiten'` plus datierter Nachtrag mit Primärseite/Registerabgleich. Historischen 01.10.-Eintrag erhalten. Optional `website` auf tatsächlich geprüfte HTTPS-URL aktualisieren; `status: unbekannt` nicht allein aufgrund dieses Materialhinweises automatisch ändern.

### 5. A.S. Brito Lampertheim — neue description freigeben als Betreiberangabe

Datei: `dossiers/he/he-lampertheim-a-s-brito.md`

**Primärbeleg:** [Palettenhandel](https://www.asbrito.de/de/palettenhandel.html): Europaletten, Gitterboxen, H1-Paletten; I./II. Wahl und Plastikpaletten; „Bringen Sie sie zu uns, wir zahlen sofort und in Bar“. Klarer bezahlter Ankauf, nicht nur Ladungsträgerlogistik. Keine numerischen Preise, Mindestmengen oder Zusage für defekte Ware. Die Aussage „Qualitäten nicht weiter spezifiziert“ sollte **nicht** die veröffentlichten I./II.-Wahl-Gruppen unterschlagen.

**Identität:** [Impressum](https://www.asbrito.de/de/impressum.html) und [NorthData](https://www.northdata.com/A%C2%B7S%C2%B7%20Brito%20GmbH,%20Lampertheim/Amtsgericht%20Darmstadt%20HRB%2062115) kongruent: GmbH, HRB 62115, Industriestr. 38, Lampertheim; NorthData führt Publikationen 2026 und Unternehmenszweck Fahrzeughandel. Letzteres widerlegt das ausdrücklich veröffentlichte Paletten-/Metallangebot nicht. Der dynamische Kopf „08.10.2026“ ist kein Nachweis aktueller Konditionspflege; alte Seitenstruktur/Impressums-Copyright 2017 bleibt Aktualitätsvorbehalt.

**Frontmatter:** Neue `description` ist eng genug („laut Betreiberseite“). [Recycling](https://www.asbrito.de/de/recycling.html) bestätigt Metallankauf, Barzahlung, Ausweis und Annahme Mo–Fr 08–19, Sa 08–14. Allgemeiner Footer nennt Mo–Fr 08–18: `opening_hours` und `dropoff_json` sind somit **unterschiedlich belegte allgemeine Öffnung versus spezielle Metallannahme**, nicht eigenmächtig auf ein Ende vereinheitlichen. Metall-/kg-Bedingungen nicht auf Paletten übertragen.

**Textkorrektur:** „I./II.-Wahl werden genannt; genaue Ankaufkriterien, Defektannahme, Mengen und aktuelle Preise bleiben offen.“ Keine neue Menge oder Stückpreise eintragen. Älteren Einzelbeleg-/Zweitquellen-Vorbehalt durch einen neuen NorthData-Nachtrag ergänzen, nicht löschen.

### 6. Tammling Hamburg — Nicht-Ankauf-Abgrenzung freigeben, Gratisbehauptung vermeiden

Datei: `dossiers/hh/hh-moorfleet-tammling-rekon-technik.md`

**Primärbeleg:** [Abholung/Logistik](https://www.tammling-rekon.de/abholung-and-logistikservice/) nennt bundesweite Einsammlung, auch einzelner Fässer, Schwerpunkt 200-l-Stahlfässer und 1.000-l-IBC aus Mineralöl-/Schmierstoffindustrie; anschließende Rekonditionierung oder Verwertung. Vorbefüllung, Verschmutzungsgrad und Stoffeigenschaften bestimmen Verwertbarkeit. Keine Vergütung/Ankaufpreise auf dieser Seite.

**Entscheidende Abgrenzung:** „Unentgeltlich zurückzunehmen“ beschreibt dort die **gesetzliche Pflicht von Herstellern/Vertreibern nach §15 VerpackG**. Daraus folgt weder vergüteter Ankauf noch ein kostenloser Tammling-Abholvertrag für jeden Kunden. Kostenlosigkeit des Dienstleisters bleibt offen. Regionale Wechselbrückenangebote gelten in erreichbarer Tagesdistanz und für restentleerte Gebinde; keine allgemeine Walk-in-Zusage in Hamburg.

**Konkreter Zusatz im Dossier und Kontextbericht:** „Die Unentgeltlichkeit bezieht sich auf die zitierte Hersteller-/Vertreiberpflicht; konkrete Tammling-Abhol-/Entsorgungskosten sind nicht veröffentlicht und anzufragen.“ Vorhandener neuer Bullet ist ansonsten korrekt.

**Frontmatter:** Keine Auditänderung. `description: Fass-/IBC-Rekonditionierung, BAM-zugelassen, ISO 9001` enthält Alt-Zertifizierungsbehauptungen, die dieser Gate nicht erneut belegt; keine neue Zertifikatsfreigabe. Vor einer Änderung/Erweiterung aktuelle Zertifikate des heutigen GmbH-Betreibers prüfen, nicht die frühere e.K.-Zulassung automatisch fortschreiben. Betreiberkette bleibt vom bestehenden Dossier dokumentiert, nicht hier neu vollständig geprüft.

### 7. Zinnrecycling Ramm Hambergen — Materialhinweis freigeben, Stundenkorrektur erforderlich

Datei: `dossiers/ni/ni-hambergen-zinnrecycling-und-metallhandel-ramm-zinn.md`

**Primärbeleg:** [Handel](https://www.zinn-metall.de/metallankauf/handel/): „Zinnaschen / Zinnausläufer / Zinnschmelzreste“, „ein Ankaufspreis wird ermittelt“, „Preise nach Analyse“. Echter analyseabhängiger Ankauf; nicht automatisch synonym zu Lötkrätze. Lötzinnkurse nur Paketsendungen, 20–63% Sn, frei von Anhaftungen; die Aschenanalysezeile selbst trägt nicht die pauschale Paket-only-Beschriftung der bezifferten Lötzinnzeile. Keine bewiesene Mindestmenge für Rückstände.

**Aktuelle Sperre:** Bis einschließlich 16.10.2026 geschlossen für Kundenverkehr, Paketkunden ausgenommen, Rückkehr 19.10.2026. Nicht dauerhaft `geschlossen` setzen, aber normale Öffnungszeiten nicht vorbehaltlos als heutige Annahme ausspielen.

**Frontmatter-Widerspruch:** Frontmatter führt Sa 08–13. [Kontakt](https://www.zinn-metall.de/kontakt/) und Footer der Handelsseite führen nur **Mo–Fr 08–12 / 13–17**, getrennte Warenannahme **08–11:30 / 13–16:30**. Samstag ist aktuell nicht belegt. Beide Seiten sind dieselbe Betreiberquelle, nicht zwei unabhängige Identitätsquellen; sie belegen jedoch den konkreten publizierten Stundenwiderspruch eindeutig. Im Bericht keine neue unabhängige Ramm-Identitätsprüfung behauptet.

**Konkreter Integrationswert:** `opening_hours: 'Mo-Fr 08:00-12:00, 13:00-17:00; Warenannahme 08:00-11:30, 13:00-16:30; Kundenverkehr 14.09.-16.10.2026 geschlossen, Paketkunden ausgenommen; Wiederöffnung laut Betreiber 19.10.2026'` mit datierter Korrektur und beiden URLs. Keine Samstagsschließung erfinden; lediglich unbestätigte Samstagsstunden entfernen. Nach 19.10. neu prüfen und zeitlich begrenzten Hinweis bereinigen.

**Weiterer Preisbefund:** Aktuell versilberte Messer ab 90er-Stempel **21 €/kg**, während historischer 06.10.-Bullet **22,50 €/kg** dokumentiert. Historischen Abrufwert **erhalten**; neuen 09.10.-Beobachtungsnachtrag ergänzen, keinen Fehler am 06.10. behaupten. Kein Händler-Publikationsdatum bekannt; Paketbedingung weiterhin zwingend.

### 8. Stienemann Borken — description freigeben, Registerfehler zwingend korrigieren

Datei: `dossiers/nw/nw-borken-bernhard-stienemann.md`

**Primärbeleg:** [Gitterboxen/Paletten](https://www.stienemann-borken.de/gitterboxen-und-paletten/): ausdrücklich An-/Verkauf, Verleih und Reparatur **gebrauchter und neuer** Gitterboxen, Paletten und Behälter. Lieferprogramm nennt Euro-/Einweg-/Kunststoffpaletten, Euro-/Industriegitterboxen, Tanks und **IPC-Behälter**. Lieferprogramm nicht ungeprüft als Liste garantierter Einkaufsqualitäten interpretieren; „IPC“ nicht eigenmächtig in „IBC“ korrigieren. Preise, Ankaufzustand, Mindestmenge und Abholbedingungen bleiben offen.

**Neue description:** [Startseite](https://www.stienemann-borken.de/) belegt Metallhandel/-ankauf, Textilverwertung/Gewebehandel und Handel mit **einigen** Kunststoffabfällen. Damit ist der breite Beschreibungstext gedeckt, aber kein pauschaler bezahlter Textil-/Kunststoffankauf. Keine solche Materialzuordnung daraus ableiten.

**Identität:** [Impressum](https://www.stienemann-borken.de/impressum/) = **KG HRA 3518 Coesfeld**; Komplementärin Stienemann GmbH = **HRB 5413**, nicht HRB 3518. [NorthData](https://www.northdata.de/Bernhard%20Stienemann%20GmbH%20%26%20Co.%20KG,%20Borken/Amtsgericht%20Coesfeld%20HRA%203518) bestätigt KG/Adresse und Abschluss-Publikation am 10.09.2026. Das neue Bullet behauptet fälschlich „HRB 3518“ und einen bereits im 01.10.-Eintrag dokumentierten NorthData-Abgleich; dort steht tatsächlich Impressum + Dutz-Kontakt. Dutz ist Gegenbeleg zur falschen Straße, kein KG-Registerbeleg.

**Zwingende Textkorrektur:** „Betreiberidentität: Bernhard Stienemann GmbH & Co. KG, HRA 3518 AG Coesfeld; Impressum und NorthData am 09.10.2026 kongruent. Komplementärin Stienemann GmbH, HRB 5413.“ Falschen Rückverweis auf 01.10. entfernen. Die beiden tatsächlich geprüften URLs angeben.

**Stundenpräzisierung:** Betreiberseite nennt Pausen Mo–Fr **09:30–09:45 und 12:30–13:00** zusätzlich zu bestehenden Warenannahme-/Bürostunden. Als Zusatz in `opening_hours` oder neuem Timeline-Nachtrag aufnehmen; keine durchgehende Annahme garantieren. Neue `description` selbst kann bleiben.

### 9. Feinhütte Halsbrücke — freigeben, B2B nicht als ausdrücklichen Ausschluss behaupten

Datei: `dossiers/sn/sn-halsbrucke-feinhutte-halsbrucke.md`

**Primärbeleg:** [Ankauf Zinn](https://www.feinhuette.de/recycling/ankauf-von-metallen/ankauf-von-zinn): unter „Das kaufen wir“ zinn-/antimonhaltige Materialien, Lotpasten, Altanoden, Krätzen, Schrotte, Aschen/Schlämme, Skimmings. Bemusterung, Metallanalyse, LME-Fixierung und Auszahlung/Gutschrift; **250 kg pro Einheit**, sowohl Formulartext als auch FAQ. Echter Ankauf, keine Gratisannahme. Kein numerischer Preis, keine frei erfundene Maximalmenge.

**Grenzen:** Metallgruppe entscheidend: kein Beleg allgemeiner Al-/Messing-/Kupferkrätze. LKW-Anlieferung und industrieller Schwerpunkt sind belegt; die Seite nennt auch Geschirrzinn und macht „Firma/Organisation“ im Formular nicht zum Pflichtfeld. Daher **„Industriekunden-Angebot/industrieller Schwerpunkt“ ist zulässig, „ausschließlich B2B/keine Privatkunden“ nicht ausdrücklich belegt**. 250 kg bleibt auch bei Geschirrzinn die publizierte Mindestmenge.

**Zusatzquelle für Altlot:** [Rücknahme](https://www.feinhuette.de/recycling/ruecknahme) erklärt Rückkauf gebrauchter Zinn-/Bleirückstände, Lötzinnabfälle/Altlot und freiwillige Rücknahme. Kostenlose Nachweisabwicklung/Behälter nicht mit kostenlosem Material oder pauschal kostenlosem Transport verwechseln. Der besondere §26-KrWG-Weg für gefährliche Abfälle bezieht sich auf Rückstände **aus eigenen Produkten**; das schränkt nicht automatisch das gesamte allgemeine Ankaufsprogramm auf eigene Produkte ein.

**Konkrekte Präzisierung:** Rücknahme-URL zusätzlich für Altlot zitieren; „industrielles, analyseabhängiges Ankaufsangebot ab 250 kg je Einheit; keine ausdrückliche allgemeine Privatkundenausschlussklausel auf der geprüften Seite“. Die neue Timeline ist ansonsten materialseitig korrekt.

**Frontmatter/Identität:** [NorthData](https://www.northdata.de/Feinh%C3%BCtte%20Halsbr%C3%BCcke%20GmbH,%20Halsbr%C3%BCcke/Amtsgericht%20Chemnitz%20HRB%206408) bestätigt Name, HRB 6408 und Krummenhennersdorfer Str. 2; Abschluss-Publikation 23.07.2026. Damit ist der ältere ausdrücklich offene Zweitbeleg für Identität/Adresse jetzt nachholbar. Datierter Nachtrag statt Umschreiben der 01.10.-Historie; keine neuen Öffnungszeiten ableiten, `dropoff_json` bleibt ohne unbeschränkte Walk-in-Zusage.

## Integrationsliste nach Priorität

1. **Pflicht:** Stienemann HRB 3518 → HRA 3518; falschen historischen NorthData-Rückverweis korrigieren. Ramm aktuelle Mo–Fr-/Warenannahmezeiten und Kundenverkehrspause im skalaren Datenstand sichtbar machen; Samstag nicht als belegt stehenlassen.
2. **Sachliche Absicherung:** FMR Metallhandelsquelle mit ausdrücklicher Lötzinnkrätze ergänzen; keine bedingungslose Hartzink-/Krätze-Vergütung behaupten. Tammling Rechtszitat nicht als kostenlose Dienstleisterabholung ausgeben. Aulfinger gefährliche-Abfälle-Vorbehalt ergänzen.
3. **Präzisierung:** TAE weitere Kunststoffe nur nach Rücksprache; Brito veröffentlichte I./II.-Wahl nennen; Feinhütte Privatkundenausschluss nicht aus industriellem Schwerpunkt ableiten und Altlot-Rücknahme zusätzlich belegen.
4. **Aktualisierung:** Wittmann publizierte Zentralstunden nachtragen, Stienemann Pausen hinzufügen; Ramm Messerpreis 21 €/kg als neue Beobachtung neben historischem 22,50-€-Wert festhalten. Neue Identitäts-Nachträge für TAE/Wittmann/Feinhütte/Brito/Stienemann mit den hier geprüften Registerquellen möglich, keine automatische Status-/Typänderung.
5. **Bewusst nicht freigegeben:** Neue Material-/Preiszeilen, generische kg-Preise für Paletten/Textilien, bedingungslose Kundenannahme an HQ-/Schwesterstandorten, pauschale Vergütung sämtlicher Recyclingrückstände und heutige Zertifikatsgültigkeit aufgrund alter Betreiber-/e.K.-Belege.

**Technischer Gate bleibt beim Owner:** Der README-Seed-Test/Build-/Produktionsgate wurde auftragsgemäß nicht ausgeführt. Dieser Bericht ist ein Quellen-/Inhaltsgate, kein Nachweis technischen oder produktiven Rollouts.
