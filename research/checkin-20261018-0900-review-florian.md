# Review Schrott Florian – 18.10.2026

## Urteil und Umfang

**Feldrücknahmen freigabefähig; den Diff als Ganzes erst nach Text-/Datierungskorrektur übernehmen. Kein Rollout-Go.** Geprüft wurde der uncommittete Diff von `dossiers/by/by-bamberg-schrott-florian.md` gegen HEAD `e136327c12adfb7d49bcd9700d3abfa2c3d77ee4`, dazu README und `docs/trader-seed.md`. Dossier nur gelesen. Keine Produktionsabfrage oder -änderung, keine Geokodierung, keine Tests, kein Commit. Ausschließlich diese Reviewdatei geschrieben. Integration, Tests und Produktionsnachweis bleiben beim Owner.

Die Rücknahme sagt **„kein hinreichend kuratierter aktueller Kontakt-/Standortfakt“**, nicht „Adresse falsch“, „Telefon abgeschaltet“, „Betrieb geschlossen“ oder „Person existiert nicht“. Das gilt getrennt für Adresse und Telefon. Fehlender Registertreffer widerlegt insbesondere keinen Kleinstbetrieb ohne Handelsregistereintrag.

## Exakte freigabefähige Felder

```yaml
street: ''
postcode: ''
phone: ''
clear_fields: 'phone'
status: pruefung
```

- **Straße/PLZ zurücknehmen:** ja. Der bisherige Wert ist zwar aktuell direkt in zwei Verzeichnissen lesbar, aber keine zulässige Betreiber-/Standortbestätigung nach README, Abschnitt „Feedback triage“, Regeln 6–7. Auch die importierte Straßenangabe ist keine unabhängige Primärquelle. Keine neue Adresse einsetzen. Die Rücknahme ist eine Qualitätsentscheidung, kein positiver Gegenbeweis gegen Heumannstr. 1.
- **Telefon explizit löschen:** ja. Derselbe unzureichend belegte Lead; `phone: ''` allein würde gespeicherte Anreicherung behalten. `clear_fields: 'phone'` ist laut Seed-Doku die passende und eng begrenzte Rücknahme. Nicht um `street`, `postcode` oder GEO erweitern: diese Tokens sind verboten.
- **Status `pruefung` behalten:** ja. Keine Aktivsetzung, Schließung, Umbenennung oder Verschmelzung freigegeben. Name, Typ, Stadt und Bundesland allenfalls als bisheriger Kandidatenrahmen belassen, nicht als neu verifizierte Händleridentität ausgeben; Slug unverändert.
- Übrige leere Kontakt-/Leistungsfelder nicht füllen, keine weiteren Clear-Direktiven ohne eigene Prüfung. Insbesondere CUTMETALL-Adresse/-Telefon/-Website nicht übernehmen. Keine `lat`/`lon` ergänzen.

## Live-Recherche 18.10.2026: Direktinhalte und Beleggrenzen

„Direkt gelesen“ beschreibt den Abruf, **nicht** automatisch die Belegqualität. Auch direkt gelesene Aggregatorseiten bleiben Leads.

| Quelle | Tatsächlich gelesener Inhalt | Verwendbarkeit |
|---|---|---|
| [Gelbe-Seiten-Detail](https://www.gelbeseiten.de/gsbiz/aa2e23f3-e1e1-4579-a2e7-84f566a7dafe) | „Schrott Florian“, Branche „Schrott“, Heumannstr. 1, 96050 Bamberg, `0951 30 94 51 95`; keine im ausgegebenen Inhalt zuordenbare Betreiberwebsite oder Betreiberidentität. Generische Schaltflächen „Website“/„E-Mail“ sind kein Betreiberbeleg. | Aktuell vorhandener **Verzeichnislead**, keine Primärbestätigung von Betrieb oder Kontakt. |
| [Das Örtliche, Namenssuche](https://www.dasoertliche.de/?form_name=search_nat&kw=Schrott+Florian&ci=Bamberg) | Ein Treffer „Schrott Florian“, Rubrik Schrott, identische Adresse und Nummer. | Zweite Verzeichnisoberfläche, **kein zweiter zulässiger unabhängiger Beleg**. Datenunabhängigkeit nicht nachgewiesen; nicht als Doppelverifikation zählen. |
| [Das Örtliche, Rückwärtssuche](https://www.dasoertliche.de/?form_name=search_inv&ph=095130945195) | Ein Treffer mit demselben Namen, derselben Adresse und Nummer. | Zweiseitiger Telefon-Abgleich bleibt innerhalb eines Aggregators. Keine fremde Telefonzuordnung belegt; ebenso wenig Erreichbarkeit oder Betreiberinhaberschaft bestätigt. |
| [NorthData Namenssuche](https://www.northdata.de/Florian+Schrott,+Bamberg), [Seite 2](https://www.northdata.de/Florian%20Schrott,%20Bamberg?offset=15), [Seite 3](https://www.northdata.de/Florian%20Schrott,%20Bamberg?offset=30) | 34 angezeigte Resultate insgesamt, darunter Alexander Schrott, Uwe Schrott und Florian Werner Joachim Fischer; kein eindeutig dem Kandidaten zuordenbarer Betreiber in den gelesenen Listen. | Suchresultate/Rechercheleads, **kein negativer Registerauszug** und keine Identitätsbrücke. |
| [NorthData Adresssuchversuch](https://www.northdata.de/Heumannstr.+1,+Bamberg) | Unscharfe Suchliste mit acht Resultaten, u. a. Neumann & Lück GmbH und mehreren Neumann-Einträgen. Keine daraus lesbare bestätigte Belegung von Heumannstr. 1 durch einen Schrotthändler. | Kein amtliches Adressregister, kein Nachweis alternativer Belegung und kein Nachweis eines leeren Standorts. Nicht aus ähnlich klingendem „Neumann“ einen neuen Betreiber ableiten. |
| [NorthData Personendetail Florian Werner Joachim Fischer](https://www.northdata.de/Fischer,%20Florian%20Werner%20Joachim,%20Bamberg/mdm) | Person heißt Fischer, nicht Schrott; zeigt Beziehungen zu CUTMETALL Pelletizing Tools GmbH und CutMetall Recycling Tools Germany GmbH, darunter Geschäftsführerereignisse von 2021/2023. | Direkter Personendetailinhalt zur Abgrenzung eines Suchtreffers. Keine Beziehung zu Heumannstr. 1 oder zur fraglichen Telefonnummer im gelesenen Inhalt. Kein aktueller Betreiberstatus für das Dossier abgeleitet. |
| [CUTMETALL Impressum](https://www.cutmetall.com/de/legal/impressum/) und [Kontakt](https://www.cutmetall.com/en/company/contact/) | Betreiberseite nennt CUTMETALL Sales GmbH, Industriering 24, 96149 Breitengüßbach, Telefon +49 (0)951 96838-0; Impressum: Geschäftsführer Alexander Klär/Volker Neuber, AG Bamberg HRB 9704. | **Betreiber-Direktinhalt für CUTMETALL**, nicht für „Schrott Florian“. Stützt die Nichtübernahme dieser Kontaktdaten; beweist keine vollständige gesellschaftsrechtliche Trennung sämtlicher historischer Einheiten. |

### Zweiseitige Bewertung / Namesakes

1. **Dossierseite:** Der Verzeichniskandidat ist weiterhin auffindbar; deshalb keine Nicht-Existenz behaupten. Ob „Schrott“ hier Familienname oder Geschäfts-/Branchenbezeichnung ist, bleibt ungeklärt.
2. **Adress-/Telefonseite:** Nummer rückwärts geprüft; Adresse separat gesucht. Kein belastbarer Alternativbetreiber an Heumannstr. 1 nachgewiesen. Damit ist diese Seite **offen**, nicht erfolgreich ausgeschlossen. Auch keinen privaten Wohnsitz oder Fremdbelegung erfinden.
3. **Verwechslungskandidaten:** Alexander/Uwe Schrott und Florian Fischer sind Suchtreffer, keine Treffer auf die volle Identität. CUTMETALL wurde zusätzlich an der eigenen Betreiberseite geprüft; deren andere Adresse und Nummer dürfen nicht zum Ersatzdatensatz werden. Der Vorname Florian plus Ortsnähe/Recyclingbezug reicht nicht zur Zuordnung.
4. **Restdatensatz:** Name + Bamberg bleibt ein Verzeichniskandidat unter `pruefung`, kein neu bestätigter Händler. Keine Phantom-Aktivierung nach Entfernen der Kontaktdaten.

### Recherchegrenzen

Die integrierte Websuche scheiterte für Namen, Adresse und Nummer mit HTTP 401. Google-Fallbacks lieferten keine verwertbaren Ergebnisinhalte, DuckDuckGo eine Bot-Challenge; Bing zur exakten Adresse lieferte sachfremde Wikipedia-Treffer und wurde verworfen. Direkte Adresssuchen bei Das Örtliche bzw. Gelbe Seiten scheiterten mit HTTP 410 bzw. 404. Diese Abruffehler sind **keine Negativbelege**. Keine kommunale Gewerbeauskunft und kein amtlicher Registerauszug erhoben. Kein verifizierter Betreiberauftritt für den Kandidaten gefunden; Recherche nicht als vollständigen Ausschluss aller Betreiber-/Namesake-Möglichkeiten ausgeben. Eine weitere Delegation war wegen Subagent-Tiefenlimit technisch nicht möglich.

## Notwendige Text- und Datierungskorrekturen vor Integration

1. **Neue Abschnitte nicht ungestützt auf 12.10.2026 datieren.** Der aktuelle Live-Abruf ist vom **18.10.2026**. Die im uncommitteten Diff neu auftauchenden Abschnitte „Korrektur 12.10.2026“ und „Owner-Korrektur 12.10.2026“ benötigen entweder separat nachgewiesene damalige Abruf-/Abfragebelege oder eine heutige, sauber begrenzte Reviewnotiz. Nicht einfach dieselben unbewiesenen historischen Aussagen unter neuem Datum wiederholen.
2. **Historie 02./05.10. nicht legitimieren.** Die vorhandenen damaligen Abrufbehauptungen sind Altbestand und in diesem Review nicht forensisch nachgewiesen. Der heutige identische Verzeichnisinhalt bestätigt keine damaligen Abrufe. Historische Timeline erhalten; bei heutiger Notiz ausdrücklich nur heutigen Abruf beanspruchen.
3. **„ein einzelner Aggregator-Lead“ aktualisieren:** heute zwei direkt gelesene Verzeichnisoberflächen mit identischen Angaben, aber **kein zulässiger unabhängiger Betreiberbeleg**. Nicht mehr behaupten, nur Gelbe Seiten enthalte die Angaben; ebenso wenig daraus zwei Belege im README-Sinn machen.
4. **Überblick präzisieren:** „aus den operativen Kontaktfeldern entfernt“ klingt nach angewandter DB-Korrektur. Stattdessen: „im Dossier-Frontmatter zurückgenommen; Datenbankübernahme erst nach Owner-Freigabe, Rebuild und Seed“. Ohne aktuellen Produktionsnachweis kein angewandtes Clearing behaupten.
5. **DB-/GEO-Aussagen attribuieren:** Der neue Owner-Absatz behauptet eine `public.db`-Abfrage am 12.10.2026 sowie konkrete Koordinaten. Diese Abfrage habe ich nicht durchgeführt oder verifiziert. Als eigene aktuelle Feststellung nicht freigeben. Entweder Owner liefert den datierten Abfragenachweis oder konkrete Bestandswerte als „laut bisherigem unbestätigtem Entwurf“ kennzeichnen/aus dem neuen Tatsachenbericht herausnehmen. Kein heutiges Prüfdatum für die alte Abfrage erfinden.
6. **Seed-Formulierung berichtigen:** „beim nächsten geänderten Seed“ ist für `clear_fields` zu eng. Nach Rebuild/Integration bleibt die Direktive bei jedem Seed maßgeblich und entfernt später eingetragenes Telefon auch bei gleich gebliebenem Seed-Hash. Straße/PLZ werden dagegen **beim geänderten Seed** als leere Strings übernommen. Keine rückwirkende Anwendung behaupten.
7. Die drei Preis-/Gebührenpunkte sind als **Unkenntnis** vertretbar, nicht als erschöpfende Marktprüfung: besser „keine für diesen Kandidaten bestätigte Preisliste/Tarife“. Kein Schluss „kein Verkauf“, „kein Ankauf“ oder „kostenlos“.

### Vorschlag für eine neue, belegbare Timeline-Notiz

> [Review 18.10.2026: Gelbe-Seiten-Detail sowie Das-Örtliche-Namens- und Telefonrückwärtssuche direkt gelesen: „Schrott Florian“, Heumannstr. 1, 96050 Bamberg, 0951 30945195 bleiben Verzeichnisleads, keine Betreiberbestätigung. NorthData-Namens- und Adresssuchen liefern keine eindeutige Zuordnung; Such-Misses beweisen keinen Nichtbetrieb. Florian Fischer/CUTMETALL nicht gleichsetzen: CUTMETALL nennt im eigenen Impressum andere Betreiber- und Kontaktdaten. Straße/PLZ und Telefon im Frontmatter als unzureichend belegte aktuelle Fakten zurückgenommen; clear_fields phone für explizite Telefonrücknahme vorgesehen. Keine Schließung, Aktivsetzung, neue Adresse oder GEO-Freigabe. Historische Abrufbehauptungen durch diesen heutigen Abruf nicht nachgewiesen. Quelle(n): https://www.gelbeseiten.de/gsbiz/aa2e23f3-e1e1-4579-a2e7-84f566a7dafe ; https://www.dasoertliche.de/?form_name=search_inv&ph=095130945195 ; https://www.northdata.de/Florian+Schrott,+Bamberg ; https://www.northdata.de/Heumannstr.+1,+Bamberg ; https://www.cutmetall.com/de/legal/impressum/]

Der Owner muss diese Notiz an den tatsächlichen Integrationsstand anpassen; sie ist kein Nachweis eines bereits gelaufenen Seeds.

## Seed-Vertrag und Rolloutgate

- [README](../README.md), Zeilen 109–134 und 160–202: Dossiers sind Buildinput; Aggregatoren sind Leads; Namesake-/Adressgegenseite prüfen; keine Felder aus bloßen Aggregatoren neu legitimieren.
- [Seed-Doku](../docs/trader-seed.md), Zeilen 22–69: leeres Telefon erhält sonst DB-Anreicherung; explizites `clear_fields` löscht, bleibt autoritativ und ist keine Betriebsschließung.
- Dieselbe Doku, Zeilen 88–106: leere `street`/`postcode` ersetzen die Adresse auf geändertem Seed; fehlende Dossierkoordinaten behalten gespeichertes GEO. Adressrücknahme verifiziert oder löscht GEO nicht.
- **Erhaltenes ungeprüftes GEO bleibt Rolloutgate.** Diese Feldfreigabe löst es nicht. Kein Geo-Dienst aufgerufen, kein Punkt validiert, keine GEO-/Produktionsmutation autorisiert. Owner übernimmt Integration/Tests und muss das ausstehende Gate separat auflösen; dieser Review ist keine Deployfreigabe.
