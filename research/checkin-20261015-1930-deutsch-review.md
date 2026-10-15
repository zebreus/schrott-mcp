# Unabhängige Nachprüfung: Mario Deutsch Metallrecycling, Kreuztal

Recherche-/Abrufdatum: 15.10.2026. Ziel: Integrationsentscheidung nach README, insbesondere Quellenhierarchie und Zwei-Seiten-/Namensvetterprüfung. Ausgangspunkt: `dossiers/nw/nw-kreuztal-mario-deutsch-metallrecycling.md` (gelesen, nicht verändert). Der Dateiname bezeichnet den Check-in, keine sekundengenau protokollierte Abrufzeit.

## Entscheidung für den Owner

**Keine Freigabe zur Aufwertung oder Übernahme von Kontaktdaten/Betriebsbedingungen. `status: pruefung` beibehalten.** Das Portalprofil ist heute direkt erreichbar; eine gegenwärtig tätige, eindeutig identifizierte Betreiberperson bzw. ein eigener Auftritt ist damit weiterhin nicht belegt. Kein Schließungsbeleg: nicht auf `geschlossen` setzen und nicht löschen. Auch ein mobiler Betrieb ist nicht nachgewiesen; Mobilfunknummer plus fehlende Anschrift sind kein Beweis dafür.

Die bestehende Vorsicht im Dossier wird bestätigt. Neu bzw. präziser dokumentierbar sind **portalinterne Öffnungszeiten-Widersprüche**, die zusätzliche Ortsangabe **Kredenbach nur in der Übersicht**, der dortige **unbelegte Badge „Zertifiziert“** und die Abgrenzung **regionaler Portalpreisanalysen von Händlerpreisen**. Keines dieser Felder gehört als bestätigte Betriebsangabe ins Frontmatter.

| Merkmal | Entscheidung | Begründung |
|---|---|---|
| Name/Ort und stabiler Slug | Bestehenden Recherchefall behalten | Nur Portal-Lead; keine belastbare alternative Identität oder Betreiberkette gefunden |
| Straße/PLZ/GEO | Leer lassen | Detailprofil nennt nur Kreuztal; Kredenbach ist ebenfalls nur Portalangabe, keine Betriebsanschrift |
| Telefon | Frontmatter leer lassen | `+49 162 1529764` erneut im Portal gesehen, nicht unabhängig einem Betreiber zugeordnet |
| Website/Website-Status | Leer lassen | Kein zurechenbarer eigener Auftritt gefunden; Portal nicht als Betreiberwebsite eintragen, fehlenden Fund nicht als tote Website werten |
| Öffnungszeiten/Materialannahme/Mindestmengen | Nicht übernehmen | Widersprüchliche Stunden und wiederholte Material-/Mengentexte; keine Betreiberbestätigung |
| Anlieferung/Abholung/Kundentypen | Leer lassen | Keine verifizierte Annahmestelle oder Abholregelung |
| Zertifizierung | Nicht übernehmen | Übersicht zeigt Badge, aber kein EFB-Nachweis, Zertifikat, Aussteller oder Geltungsbereich |
| Preise/Handler | Keine Preisübernahme, kein Handler | Weder eigene Ankaufsliste noch Verkaufs- oder Gebührenliste belegt |

Nach README ist ein Aggregator **kein** Identitätsbeleg, auch nicht dann, wenn Übersicht und Detailprofil übereinstimmen. Hier fehlen zwei unabhängige zugelassene Belege; auch die Kriterien für eine allein ausreichende autoritative Betreiberquelle sind nicht erfüllt. Die amtliche Kontaktseite unten bestätigt nur die Zuständigkeit der Behörde, nicht dieses Gewerbe.

## Tatsächlich direkt abgerufene Seiten und Befunde

Alle folgenden Seiten wurden am 15.10.2026 mit `webfetch` als lesbarer Seiteninhalt erfolgreich abgerufen. Das Werkzeug lieferte dabei keinen gesonderten numerischen HTTP-Status; daher wird kein „HTTP 200“ behauptet. Keine Anrufe, Formulareinsendungen, Behördenanfragen, Käufe oder Logins.

### 1. Portal-Detailprofil

https://lokaleschrottplatz.de/mario-deutsch-metallrecycling-schrotthandel-dienstleistungen/

- Überschrift: „Mario Deutsch Metallrecycling Schrotthandel & Dienstleistungen“; Adresse ausschließlich „Kreuztal“; Telefon `+49 162 1529764`; Bewertung 3,3 aus 3 Bewertungen.
- Stundenliste Mo–Sa `00:00–24:00`, So `12:00–18:00`. Gleichzeitig Statuszeile „Geschlossen, öffnet morgen um 04:00“. Schon diese Kombination passt nicht ohne Weiteres zusammen; keine tatsächlichen Betriebszeiten daraus ableiten.
- Unter „Annahme von Schrott“ 15 Gruppen von Stahl/Eisen bis Elektromotoren, darunter Altfahrzeuge sowie Hart-/Sondermetalle. Mindestgewicht wörtlich: „Annahme von Eisenmetallen ab 100 kg, Nichteisenmetallen ab 1 kg.“
- Im abgerufenen Kontaktteil keine Straßenanschrift, E-Mail, Betreiberwebsite, Registerkennung oder personenbezogenes Betreiberimpressum. „Route berechnen“ führt im aufbereiteten Inhalt nur auf `#` und bestätigt keinen Betriebspunkt.
- Keine numerische Händlerpreisliste und keine nachvollziehbare Bestätigung, dass Mario Deutsch selbst diesen Eintrag betreut.

### 2. Kreuztaler Portalübersicht: zusätzliche Widersprüche und Preisfalle

https://lokaleschrottplatz.de/nordrhein-westfalen/kreuztal/

- Mario Deutsch dort ebenfalls mit derselben Nummer und Bewertung, aber Ortslabel **„Kredenbach“** statt einer Straße und Badge **„Zertifiziert“**. Auch die beiden anderen auf dieser Übersicht gelisteten Betriebe tragen dieses Badge. Das ist kein Zertifikatsnachweis.
- Beim Deutsch-Eintrag Mo–Sa **`22:00–22:00`**, So **`10:00–16:00`**, Status **„Geöffnet bis 22:00“**. Das weicht von der Detailseite ab. Ein Zeitzonen-/Darstellungsfehler ist denkbar, aber nicht untersucht oder bewiesen; Stunden nicht stillschweigend „korrigieren“.
- Die Übersicht hat eine ausdrücklich regionale „Kurze Preisanalyse für Schrott in Nordrhein-Westfalen“ mit u.a. Messing Schwer `5,70–6,70 €/kg` und Kupfer Millberry `10,60–11,70 €/kg`, Trendangaben sowie Deutschland-Vergleichen. **Diese tatsächlich gesehenen Zahlen sind weder Angebot noch Auszahlung von Mario Deutsch** und dürfen nicht als dessen `published`-Ankaufskurse übernommen werden.
- Der „Wohin verkaufen?“-Block ist Portalnavigation, nicht gleichbedeutend mit einer Händler-Ankaufserklärung oder einem Verkaufslager.

### 3. Direktvergleich eines anderen Portalprofils

https://lokaleschrottplatz.de/gluck-schrotthandel-recycling/

- „Glück Schrotthandel + Recycling“, ebenfalls nur Adresse Kreuztal, andere Nummer `+49 1520 9471757`.
- Identische 15 Materialgruppen und identischer Satz „Annahme von Eisenmetallen ab 100 kg, Nichteisenmetallen ab 1 kg.“
- Das bestätigt die Wiederholung **dieser Material-/Mengenblöcke** in mindestens einem anderen Profil. Es beweist nicht allein, dass die Bedingungen falsch sind oder wer sie erstellt hat. Dieses Vergleichsprofil hat keine Stundenliste; die alte pauschale Behauptung, auch Deutschs Öffnungszeiten seien bei anderen Händlern identisch, wird durch diesen konkreten Vergleich **nicht** belegt. Belastbarer sind die heute selbst beobachteten Widersprüche zwischen Übersicht und Detail.

### 4. Selbstauskunft des Portals über seine Quellen

https://lokaleschrottplatz.de/nutzungsbedingungen/

- Gelesener Seitenstand mit „Letzte Aktualisierung: 15.10.2026“ (Seitenangabe, kein unabhängig geprüftes Änderungsdatum).
- Ziff. 1.2 bezeichnet den Dienst als Aggregator. Ziff. 2.1 nennt öffentlich zugängliche Daten **sowie** Einsendungen von Unternehmensvertretern; 3.2 beschreibt die nachträgliche Übernahme eines bereits aus öffentlichen Quellen angelegten Profils.
- Deshalb ist die bloße Existenz des Deutsch-Profils gerade kein Beweis für Betreiberautorisierung. Aus diesen allgemeinen Bedingungen lässt sich nicht feststellen, aus welcher Quelle dieser einzelne Eintrag stammt oder ob er übernommen wurde.
- Ziff. 2.2 beschreibt Preisaggregation aus Websites/anderen öffentlichen Quellen und schließt eine Garantie für Richtigkeit/Aktualität aus. Keine konkrete Herkunftskette für die auf der Stadtseite gesehenen Preisintervalle.

### 5. Regionaler eigener Auftritt als Gegenprüfung, nicht als Identitätsfund

https://www.schrottsiegen.de/impressum.html

- Die Suche nach dem Zielbetrieb im Raum Siegen/Kreuztal lieferte auch `schrottsiegen.de`. Das direkt gelesene Impressum nennt aber **Mohammed El-lahib, Ahlmannshof 42, 45889 Gelsenkirchen**, `0152 / 13 65 38 54`, `info@schrottsiegen.de`.
- Damit **kein positiver Nachweis für Mario Deutsch**: weder Name noch Nummer stimmen mit dem Portal-Lead überein. Nicht aufgrund des Einsatzorts Kreuztal verknüpfen oder übernehmen. Eine gesellschaftsrechtliche Beziehung wird weder behauptet noch allein durch diese Abweichungen ausgeschlossen.

### 6. Amtlicher Klärweg

https://www.kreuztal.de/verwaltung-und-politik/rathaus-online/dienstleistungen/212

- Direkt gelesene städtische Seite zur Gewerbean-/um-/abmeldung: zuständig für das Stadtgebiet ist das Gewerbeamt beim Ordnungsamt Kreuztal. Genannte Ansprechpartnerinnen: Pia Klein, `+49 2732 51 350`, und Louisa Pötz, `+49 2732 51 217`.
- Das ist ein **Kontaktweg zur weiteren Klärung**, kein öffentlicher Registerauszug für Mario Deutsch. Kein bestimmtes Auskunftsverfahren, keine Auskunftsgebühr und keine erfolgte Anfrage aus dieser Seite ableiten. Dort genannte Anmeldegebühren sind selbstverständlich auch keine Entsorgungsgebühren dieses Händlers.

## Eigene Suchläufe und Identitätsgrenzen

Zusätzlich zu den Direktabrufen wurden folgende Suchabfragen mit `websearch` ausgeführt; die Suchergebnisse sind nur Suchraum/Leads, nicht vollständig direkt geprüfte Dokumente:

- `"Mario Deutsch" "Metallrecycling"`
- `"Mario Deutsch" Kreuztal Schrott`
- `"Mario Deutsch" Metallrecycling Kreuztal Kontakt`
- `Mario Deutsch Schrotthandel Dienstleistungen Kreuztal Siegen`
- `"Deutsch" "Metallrecycling" "Kreuztal"`
- `"Mario Deutsch" "Kredenbach"` — Werkzeug meldete keine Treffer.
- `"0162 1529764"`, `"01621529764"`, `"491621529764"`, `"1529764" Schrott`, `"Deutsch" "Schrotthandel" "1529764"` — keine einschlägige unabhängige Betreiberzuordnung in den zurückgegebenen Ergebnissen; vielfach themenfremde Treffer. Das begrenzt die Aussagekraft der negativen Suche.
- `"Mario Deutsch" site:northdata.de` — keine passende Identitätsseite in den gelieferten Treffern (stattdessen andere Personen/Firmen). **Keine direkte Registerabfrage und kein Beweis eines fehlenden Registereintrags.**
- `"Mario Deutsch" site:firmenwissen.de` — Werkzeug meldete keine Treffer. Keine direkt gelesene Creditreform-Firmenauskunft.
- `"Mario Deutsch" Schrotthandel Facebook` — kein zurechenbares Betreiberprofil in den gelieferten Treffern. Keine vollständige Facebook-interne Suche.
- `site:kreuztal.de "Gewerberegisterauskunft"` — führte zur oben direkt geprüften Gewerbeamtseite, nicht zu einem Zielbetriebs-Auszug.

Namensvettertreffer auf LinkedIn betrafen IT/Krypto bzw. Applikationstechnik; sie wurden nicht direkt aufgerufen und mangels Verknüpfung durch Ort/Nummer/Branche ausdrücklich **nicht** dem Händler zugerechnet. Auch weitere ähnlich benannte Recycling-Websites aus unscharfen Suchtreffern sind ohne Identitätsbrücke kein eigener Auftritt von Mario Deutsch.

**Nicht geleistet:** vollständige Handelsregister-/Gewerberegisterprüfung, behördliche Negativauskunft, telefonische Identitätsbestätigung, interne Suche sozialer Netzwerke, Vor-Ort-Prüfung oder historische Domain-/Archivrekonstruktion. Ergebnis daher „eigener Auftritt nicht gefunden/Identität offen“, ausdrücklich nicht „existiert nicht“, „geschlossen“ oder „Portal erfindet den Betrieb“.

## Ankauf, Verkauf und Gebühren getrennt

| Richtung | Tatsächlich beobachtet | Zulässige Integration |
|---|---|---|
| **Ankauf**: Händler bezahlt Anlieferer | Portal-Annahmebeschreibung, Portalnavigation „Wohin verkaufen?“; keine individuelle Vergütungszusage und keine Händlerkurse | Ankauf weiter ungeklärt; keine Preise, keine Materialannahme als bestätigt importieren |
| **Verkauf**: Händler verkauft Material an Kunden | Keine zurechenbare Verkaufsangebots-/Lager-/Preisliste auf den geprüften Zielseiten | Nicht mit Kundensicht „Schrott verkaufen“ im Portal verwechseln; unbekannt, nicht ausgeschlossen |
| **Gebühren**: Kunde bezahlt Händler für Annahme/Abholung/Dienstleistung | Keine zurechenbare Gebühren-/Transport-/Demontageliste | Weder kostenlos noch gebührenpflichtig behaupten; keine Nullpreise |

## Vorschlag für eine spätere Owner-Timeline, nicht angewandt

> [Recherche 15.10.2026: Portalprofil erneut direkt gelesen: nur Kreuztal und +49 162 1529764, keine Betreiberwebsite/Anschrift oder Registerkennung. Portalübersicht nennt zusätzlich Kredenbach und „Zertifiziert“, aber ohne Zertifikat; Öffnungszeiten dort Mo–Sa 22–22/So 10–16 statt Detail Mo–Sa 0–24/So 12–18, zusätzlich widersprüchliche Statuszeilen. Materialliste/Mindestmengen 100 kg Eisen/1 kg NE wortgleich auf Glück-Profil; deshalb keine bestätigten Händlerbedingungen. Eigener Auftritt und unabhängiger Betreiberbeleg in dokumentierten Suchläufen nicht gefunden, nicht ausgeschlossen. Regionale Portalpreisintervalle sind keine Deutsch-Ankaufskurse; Verkauf/Gebühren weiter unbelegt. Status pruefung und leere Kontakt-/Bedingungsfelder beibehalten. Quellen: https://lokaleschrottplatz.de/mario-deutsch-metallrecycling-schrotthandel-dienstleistungen/ ; https://lokaleschrottplatz.de/nordrhein-westfalen/kreuztal/ ; https://lokaleschrottplatz.de/gluck-schrotthandel-recycling/ ; Abruf 15.10.2026.]

Nächster belastbarer Schritt: Betreiberidentität/Gewerbeanschrift über eine konkrete amtliche Auskunft klären, anschließend mit unabhängig zurechenbarer Betreiberquelle abgleichen. Die Portalnummer bleibt bis dahin ein unbestätigter Kontakt-Lead. Keine Annahmestelle oder GEO aus Ortsmitte/Kredenbach konstruieren. Kein Änderungsbedarf an Produktionsdaten aus dieser Recherche ableitbar.

## Arbeitsgrenzen

Nur diese Research-Datei geschrieben. Keine Dossier-/Codeänderungen, keine Git-Änderungsoperationen, keine Datenbankzugriffe oder -schreibvorgänge und keine Produktionseingriffe. Integration, Seed-Tests und gegebenenfalls Deployment liegen beim Owner; nicht als durchgeführt behauptet.
