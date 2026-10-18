# Jansen / Roding – unabhängiger Diff- und Quellenreview

Prüfdatum/Abrufe: **18.10.2026**. Vergleich: `git diff HEAD -- dossiers/by/by-roding-jansen.md`, HEAD `e136327c12adfb7d49bcd9700d3abfa2c3d77ee4`.

## Entscheidung

**Die Rücknahme von Straße/PLZ und der explizite Telefon-Clear sind fachlich freigabefähig; der vorliegende Text ist erst nach den unten genannten Korrekturen integrationsfähig. Keine Rolloutfreigabe.** Es ist die Rücknahme unzureichend belegter aktueller Behauptungen, keine Widerlegung der Adresse, kein Schließungsnachweis und keine Identifizierung eines anderen Betreibers.

Es gibt einen ernst zu nehmenden lokalen Gegenhinweis: Ein heute direkt lesbarer Telefonbuch-Aggregator nennt **Romano Jansen, Flurweg 8, 93426 Roding**, allerdings mit **09461 401336**, nicht der zurückzunehmenden Mobilnummer. Das widerlegt die Annahme einer frei erfundenen lokalen Namens-/Adresskombination, beweist aber weder einen Schrotthändler noch einen aktuellen Anlieferort. Deshalb nicht auf einen auswärtigen Jansen umstellen und nicht Romano als gesicherten Betreiber übernehmen. Der lokale Identitätsklärfall bleibt erhalten.

### Exakt freigabefähige Frontmatter-Änderungen gegenüber HEAD

```yaml
street: ''
postcode: ''
phone: ''
clear_fields: 'phone'
```

- `status: pruefung`, `slug: by-roding-jansen`, `name: Jansen`, `city: Roding`, `state: BY` und `trader_type: sonstige` **unverändert lassen**. Das ist Bestandserhaltung eines ungeklärten Rechercheobjekts, keine neue Tatsachenfreigabe seiner gewerblichen Existenz.
- Keine neuen Betreiber-, Telefon-, Website-, Service-, Preis- oder Koordinatenwerte freigegeben. Insbesondere weder `Romano Jansen` noch `09461 401336` in operative Felder übernehmen.
- Keine weiteren `clear_fields`-Tokens ohne eigenen Befund hinzufügen; `street`, `postcode`, `lat` und `lon` sind dort ausdrücklich verboten.

## Maßstab und Seed-Wirkung

Gelesen: [README](../README.md), insbesondere Zeilen 160–202 (zweiseitige Prüfung, Namesakes, Phantomreste, Quellenhierarchie), 109–134 (Buildinput/GEO) und 232–240 (Ownergate); [Seed-Dokumentation](../docs/trader-seed.md), insbesondere Zeilen 22–69 sowie 88–111. Zusätzlich statisch gelesen: `crates/ingestion/src/seed_traders.rs:343–442`.

- Aggregatoren sind nach README **Leads**, auch bei kongruenten Adressen oder mehreren Portalen. Zwei Aggregatoren sind kein Zwei-Quellen-Beleg. Eine direkt gelesene Aggregatorseite bleibt ein Lead.
- NorthData-Suchtreffer ohne zugeordnete Detail-/Registeridentität erfüllen keinen Betreiberbeleg. Eine unscharfe, paginierte Suche ist kein Negativregisterauszug. Fehlende HR-Zuordnung beweist weder Nichtexistenz noch „Kleinstbetrieb“.
- Die Betreiber-Ausnahme des README greift hier nicht: keine identitätsgesicherte Betreiberwebsite mit den geforderten Register-/Standort-/Aktualitätsmerkmalen gefunden.
- Leeres `phone` allein behält gespeichertes Enrichment. `clear_fields: 'phone'` setzt dagegen den leeren Default, ist auch gegen spätere Drift autoritativ und bleibt wirksam, solange die Directive steht. Das macht sie zur passenden, bewusst zu genehmigenden Rücknahme der alten Mobilnummer.
- `street`/`postcode` werden beim geänderten Seed direkt überschrieben, auch mit Leerstrings. Das ist ein separater Adressrücknahmeentscheid, nicht Nebenwirkung des Telefon-Clears.
- Fehlendes Koordinatenpaar übernimmt vorhandenes DB-GEO. Adressrücknahme bereinigt oder bestätigt keinen Pin. Ein Rebuild mit anschließendem Seed ist nötig; eine Markdown-Änderung ist noch keine Produktionsänderung.

## Zweiseitige Recherche: Direktinhalte und Leads

Alle folgenden Abrufe erfolgten in diesem Review am **18.10.2026**. Frühere Dossier-Abrufe wurden weder nachgestellt noch nachträglich zertifiziert.

| Quelle / Abruf | Tatsächlich sichtbarer Inhalt | Beweisgrenze / Verwendung |
|---|---|---|
| [lokaleschrottplatz.de/jansen/](https://lokaleschrottplatz.de/jansen/) | `webfetch`: **HTTP 404**, kein Sachinhalt | Nur Abrufbefund. Kein Beleg für Kontakt, Betreiber, Preise, Schließung oder heutigen HTTP 403. |
| [NorthData: Jansen Roding](https://www.northdata.de/?query=Jansen%20Roding) | Direkt gelesene Suchseite, 15 Treffer von 1.000+; u. a. Jansen RS Holding B.V. Maastricht und zahlreiche unscharfe internationale Personen-/Firmentreffer; im gelesenen Block keine kongruente Roding-Schrottidentität | Keine vollständige Registerprüfung; kein Nichtexistenzbeweis. Suchseite, keine zugeordnete Unternehmensdetailseite. |
| [DuckDuckGo: „Jansen“ „Roding“](https://html.duckduckgo.com/html/?q=%22Jansen%22+%22Roding%22) | Suchausgabe nennt Romano Jansen an Flurweg 8, Telefonbuch-Seiten „Jansen Romano Maier Katrin“ bzw. „Jansen Romano u. Maier Katrin“ sowie sachfremde Jansen-Maschinenanzeigen | **Suchleads**, keine Betreiberprimärbelege. Die Branchenwörter in den Telefonbuch-URLs („Eisen“, „Edelstahl“) beweisen keine Geschäftstätigkeit. Maschinenmarke Jansen ≠ lokaler Betreiber. |
| [telefonbuchsuche.de: Romano Jansen](https://telefonbuchsuche.de/romano-jansen-flurweg-93426-roding) | **Direkt lesbare Aggregatorseite**: „Romano Jansen“, „Flurweg 8, 93426 Roding“, „(0 94 61) 40 13 36“; Personen-Adressen-Finder, keine belegte Schrotttätigkeit/Annahmebedingungen | Stärkster heute lesbarer lokaler **Lead**, aber kein Betreiberbeleg. Stützt Recherche an der alten Adresse; rechtfertigt weder Rückfüllung noch neue Namens-/Telefonzuordnung. Die andere Rufnummer beweist nicht, dass die Mobilnummer falsch ist. |
| [Das Telefonbuch: Eisen-URL](https://adresse.dastelefonbuch.de/Roding/1-Eisen-Romano-Jansen-Katrin-Maier-Roding-Flurweg.html), [Edelstahl-URL](https://adresse.dastelefonbuch.de/Roding/1-Edelstahl-Romano-Jansen-u-Katrin-Maier-Roding-Flurweg.html), [Firmensuche](https://www.dastelefonbuch.de/Firmen/Jansen/Roding) | Direktabrufe jeweils **HTTP 410**; Suchsnippet nennt 09461 401336 und Roding-Mitterdorf | Snippet nicht zu direkt gelesenem Telefonbucheintrag hochstufen. Kein weiterer unabhängiger Beleg, kein Schließungsnachweis. |
| [NorthData: Romano Jansen](https://www.northdata.de/?query=Romano%20Jansen) | Direkt gelesener erster Suchblock: unscharfe Treffer wie Roman Francis Jansen / Roman Jansen-Winkeln, keine passend zugeordnete Roding-Identität | Lokalen Vornamenslead gegengeprüft, aber kein belastbarer Registerentscheid; keine vollständige Ausschöpfung behauptet. |
| [Jansen Recycling Group](https://jansengroup.com/de/) | **Betreiberdirektinhalt**: Metallrecycling; Kontakt/Standorte **Koggehaven 3, 3133 LA Vlaardingen** und **Van Leeuwenhoekweg 21, 3316 AV Dordrecht**, allgemeines Telefon **+31 78 6522633**; News auch aus 2026 | Tatsächlicher namensgleicher Recyclinganbieter, aber im gelesenen Inhalt **keine Roding-Verknüpfung**. Keine dieser Angaben übernehmen. Aus zwei genannten Standorten keine weltweite Negativaussage über alle möglichen Beziehungen ableiten. |
| [jansen.de / Impressum](https://www.jansen.de/impressum/) | **Betreiberdirektinhalt**: P.A. Jansen GmbH u. Co., KG, Hochstadenstraße 22, 53474 Ahrweiler, +49 2641 3897-0, HRA 10166 Koblenz; Website für Lacke/Lasuren/Speziallösungen | Konkreter weiterer Namesake, kein Beleg für Roding und kein Schrotthändlernachweis für dieses Dossier. Registerstatus nicht separat bestätigt; keine authoritative-primary-Ausnahme für die Roding-Zeile. |

### Telefonprüfung und Suchgrenzen

Die alte **0171 1884077** wurde in den heute lesbaren Sachinhalten nicht primär einem Roding-Betreiber zugeordnet. Das ist eine Beleglücke, kein Beweis einer falschen/abgeschalteten Nummer. Versuchte Suchen: `"0171 1884077"` (integrierte Websuche: HTTP 401), `"01711884077"` (Google nur Zugriffshinweis; DuckDuckGo CAPTCHA), `"0171" "1884077"` (Bing nur unspezifische Vorwahlseiten), [DuckDuckGo lite](https://lite.duckduckgo.com/lite/?q=%220171+1884077%22) (CAPTCHA). Keine Anrufe durchgeführt.

Auch integrierte Namenssuche scheiterte mit HTTP 401. Google-Suchen nach Name und Adresse lieferten keinen lesbaren Ergebnisblock; Bing-Suchen nach `"Jansen" "Roding"` bzw. `"Flurweg 8" "Roding"` sachfremde Cricket-/Wikipedia-Treffer. DuckDuckGo-Adresssuche und spätere `"Romano Jansen" Schrott`-Suche zeigten CAPTCHA. Diese Ausgaben sind **keine** fachlichen Negativtreffer. Keine CAPTCHA-Umgehung, keine historischen Webarchive, keine amtliche Gewerbeauskunft beschafft.

## Erforderliche Datierungs- und Textkorrekturen vor Integration

1. **Keine Rückdatierung der heutigen Prüfung auf 12.10.2026.** Die beiden neuen, uncommitteten Abschnitte heißen „Korrektur 12.10.2026“ und „Owner-Korrektur 12.10.2026“. Dieser Review belegt nur den heutigen Arbeitsbaum und heutige Abrufe. Owner muss entweder den tatsächlichen damaligen Vorgang durch ein datiertes Artefakt belegen und als historischen Bericht zuordnen oder die neue Entscheidung auf **18.10.2026** datieren. Ein heute neu formulierter Abruf darf nicht als damaliger Abruf erscheinen. Der Dateiname `0900` ist Auftragskennung, kein Nachweis einer minutengenauen Abrufzeit.
2. **Historische Abschnitte 02.10./08.10. erhalten, nicht beglaubigen.** „kongruente Aggregatoren inkl. Telefonbuch“ ohne genaue damalige URLs/Abrufartefakte lässt sich heute nicht unabhängig verifizieren. Die neue Korrektur muss dies sagen. Im Überblick „der historische Beleg bleibt“ ersetzen durch **„die frühere, heute nicht unabhängig verifizierte Lead-Behauptung bleibt als Historie dokumentiert“**. Den damaligen Geokodierauftrag ausdrücklich als überholt kennzeichnen, ohne die historische Zeile still zu löschen.
3. **Heutigen HTTP-Befund korrekt trennen:** 403 nur als im alten Text berichtete Abrufgrenze, nicht als Ergebnis des 18.10.; heute lieferte der Portalabruf **404**. Weder 403 noch 404 belegt eine Schließung. NorthData-Suchseite als begrenzten Suchbefund beschreiben, nicht als negativen Registerbeweis.
4. **Lokalbefund ergänzen:** Romano Jansen / Flurweg 8 / 93426 / 09461 401336 ausschließlich als heute direkt gelesenen Telefonbuch-Lead nennen; Katrin Maier nur als Suchsnippet-Lead, ohne Betreiberrolle. Sonst wäre die Rücknahmebegründung trotz sachgerechtem Ergebnis zu einseitig. Externe Namesakes konkret abgrenzen, nicht nur pauschal warnen.
5. **Keine unbewiesene DB-Abrufdatierung:** Die Behauptung „read-only-Abfrage 12.10.2026“ und die gespeicherten Werte einschließlich GEO im neuen Owner-Abschnitt sind in diesem Review **nicht** unabhängig durch DB-Zugriff bestätigt. Owner muss seine tatsächliche Abfrage samt Datum/Nachweis referenzieren oder deutlich „laut bisherigem Owner-Vermerk, hier nicht erneut geprüft“ schreiben. Code/Doku beweisen Seed-Verhalten, nicht den damaligen DB-Inhalt. Auch „Keine Produktionsdaten geändert“ nur auf die eigenen nachweisbaren Handlungen beziehen.
6. **Lokaler Entwurf ≠ Produktion:** „aus den operativen Feldern entfernt“ präzisieren zu **„im Dossier-Frontmatter zurückgenommen; Übernahme in die Datenbank erst nach Owner-Integration, Rebuild und Seed“**. Der Telefon-Clear ist die freigegebene Absicht, noch kein erfolgreicher Produktions-Clear.
7. **Preisaussagen epistemisch formulieren:** Statt „Ankauf: keine Kurse / Verkauf: keine Preise / Gebühren: keine Tarife“ schreiben: **„Ankauf, Verkauf, Gebühren: keine dieser Identität belastbar zugeordneten veröffentlichten Zahlen belegt; keine Preisprüfung an identitätsgesicherter Betreiberquelle möglich.“** Keine Aussage, der Händler erhebe nichts oder veröffentliche grundsätzlich nichts.
8. **GEO ausdrücklich offen lassen:** Nicht nur „Adresse erst nach Primärabgleich geokodieren“, sondern zusätzlich **„Ein etwaiger gespeicherter, ungeprüfter GEO-Punkt bleibt vom Seed erhalten und ist weiterhin Rolloutgate; dieser Review bestätigt keinen Standort/Pin.“** Hier keine Koordinatenprüfung oder Neugeokodierung veranlassen.

### Vorschlag für einen neuen Korrekturbullet (Owner integriert)

> [Korrektur 18.10.2026: Die nur als frühere Verzeichnis-Ausnahme dokumentierten Werte Flurweg 8 / 93426 und 0171 1884077 werden im Dossier-Frontmatter zurückgenommen; ihre damaligen Abrufe sind heute nicht unabhängig verifiziert. Der heute direkt lesbare Aggregator telefonbuchsuche.de nennt Romano Jansen an derselben Adresse mit 09461 401336, belegt aber weder Schrottgewerbe noch aktuellen Anlieferort. Der Name ist daher nur ein Recherchelead, keine Betreiberzuordnung. NorthData-Suchen nach Jansen Roding und Romano Jansen ergaben im gelesenen ersten Block keine kongruente Registeridentität; keine vollständige Negativauskunft. Das Portal lokaleschrottplatz.de/jansen/ lieferte heute HTTP 404, keine Sachangaben. Keine Gleichsetzung mit der Jansen Recycling Group in den Niederlanden oder P.A. Jansen in Ahrweiler. status pruefung bleibt; keine Schließungsaussage und keine belastbar zugeordneten veröffentlichten Zahlen für Ankauf, Verkauf oder Gebühren. clear_fields phone zieht den gespeicherten Telefonlead beim Seed ausdrücklich zurück; Straße/PLZ werden beim geänderten Seed leer überschrieben. Keine Produktionsbereinigung aus der Dossieränderung abgeleitet. Vorhandenes ungeprüftes GEO bleibt erhalten und Rolloutgate. Quellen: https://telefonbuchsuche.de/romano-jansen-flurweg-93426-roding ; https://www.northdata.de/?query=Jansen%20Roding ; https://www.northdata.de/?query=Romano%20Jansen ; https://lokaleschrottplatz.de/jansen/ ; https://jansengroup.com/de/ ; https://www.jansen.de/impressum/ ; docs/trader-seed.md.]

## Owner-Handoff und Arbeitsgrenzen

- Owner übernimmt sämtliche Dossierintegration, endgültige Datierung, Tests und Rolloutentscheidungen. Keine neue Untersuchung behauptet die historische Abruf- oder DB-Chronik als bewiesen.
- Nach Textkorrekturen ist genau der oben genannte Vier-Felder-Diff fachlich zur lokalen Integration freigegeben. Kein Anlass für `geschlossen`, Ersatzeintrag, Namensänderung oder Standortverlegung.
- Für spätere Rückfüllung zuerst die lokale Hypothese prüfen: identitätsgesicherter Betreiber-/Gewerbebeleg zu Romano Jansen bzw. anderem tatsächlichen Betreiber an Flurweg 8 sowie Geschäftstätigkeit/Annahme und aktuelle Kontakte. Ein fehlender HR-Treffer allein genügt nicht; eine amtliche Gewerbeauskunft oder zuordenbare Betreiberquelle wäre substanzieller als weitere Portalduplikate.
- **Erhaltener ungeprüfter GEO bleibt Rolloutgate.** Auch grüne Seedtests bestätigen weder Betreiber noch Punktzuordnung. Keine Freigabe durch bloßes Leeren der Adresse.
- Ausgeführt: Git-Diff/HEAD-Lektüre, README-/Seed-/Code-Lektüre und die beschriebenen Webabrufe. **Keine Tests/Builds, keine Produktions- oder sonstigen DB-Abfragen, keine DB-Schreibzugriffe, kein GEO-Abruf/Geocoding.** Ein nach Skill vorgesehener zusätzlicher Rechercheagent konnte wegen Harness-Tiefenlimit nicht starten; alle hier berichteten Recherchen stammen aus diesem Review.
- Dossier ausschließlich gelesen. Einzige geschriebene Datei dieses Reviews: `research/checkin-20261018-0900-review-jansen.md`.
