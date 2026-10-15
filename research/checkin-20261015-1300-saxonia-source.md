# SAXONIA: Produktionswarnung Tageskurse, 15.10.2026 13:00 UTC

## Ergebnis

**Offiziell verlinkte Ersatzseite gefunden: <https://saxonia.de/tageskursanzeiger/>.** Die bisherige URL `/edelmetallhandel/tageskurse/` liefert tatsächlich HTTP 404, während die neue Seite HTTP 200 und unmittelbar im HTML Tageskurse mit Datum **15.10.2026** enthält. Navigation und Sitemap der unveränderten Domain verweisen auf die neue Seite. Das spricht für eine Pfad-/Websiteumstellung, nicht für einen allgemeinen Websiteausfall. Ein Betreiber- oder Domainwechsel ist durch die untersuchten offiziellen Quellen **nicht belegt**. Der genaue Umstellungszeitpunkt und die Dauerhaftigkeit der neuen URL bleiben offen. [S1–S5]

Erhebung: öffentliche HTTP-GETs am **15.10.2026, 13:01–13:02 UTC**, einschließlich Roh-HTML und eingebundener öffentlicher JavaScript-Dateien. Keine Anmeldung, Formularübermittlung, POSTs, Datenbankzugriffe oder lokalen Handler-/Teständerungen. Nur diese Recherchedatei geschrieben.

## HTTPbefunde und Auffindbarkeit

| Offizielle URL | Direkter Befund | Zeitpunkt UTC |
| --- | --- | --- |
| <https://saxonia.de/edelmetallhandel/tageskurse/> | **404**, finale URL unverändert; HTML-Titel `Seite nicht gefunden – SAXONIA`; kein Redirect zur Ersatzseite | 13:01:32 |
| <https://saxonia.de/edelmetallhandel/tageskurse> | **404**, auch ohne abschließenden Slash | 13:01:44 |
| <https://www.saxonia.de/edelmetallhandel/tageskurse/> | finale URL auf `saxonia.de`, dort **404** | 13:01:45 |
| <https://saxonia.de/tageskurse/> | **404**; kein geeigneter Alternativpfad | 13:01:44 |
| <https://saxonia.de/> | **200**, reguläre Unternehmenswebsite | 13:01:32 / 13:01:45 |
| <https://saxonia.de/edelmetallhandel/> | **200**, reguläre Leistungsseite | 13:01:44 |
| <https://saxonia.de/tageskursanzeiger/> | **200**, finale URL unverändert, `Content-Type: text/html; charset=UTF-8`; Antwortheader `Date: Thu, 15 Oct 2026 13:01:56 GMT` | 13:01:56 |
| <https://saxonia.de/impressum/> | **200**, Betreiberangaben vorhanden | 13:01:57 |
| <https://saxonia.de/robots.txt> | **200**, nennt `Sitemap: https://saxonia.de/wp-sitemap.xml` | 13:01:32 |
| <https://saxonia.de/sitemap.xml> | finale URL `https://saxonia.de/wp-sitemap.xml`, **200** | 13:01:33 |
| <https://saxonia.de/wp-sitemap-posts-page-1.xml> | **200**, enthält `/tageskursanzeiger/` | 13:01:57 |

Die Hauptnavigation im Roh-HTML der Startseite **und** der Edelmetallhandelsseite enthält ausdrücklich:

```html
<a href="https://saxonia.de/tageskursanzeiger/" class="elementor-sub-item" tabindex="-1">Tageskursanzeiger</a>
```

Der Startseiten-Button „Zum Tageskursanzeiger“ hat dagegen lediglich `href="#"`; ein reiner Markdown-/Textabruf kann deshalb den funktionierenden Navigationslink übersehen. Die Sitemap bestätigt die neue URL mit `lastmod` **2026-10-13T14:43:29+02:00**. Das ist Seitenmetadatum, **kein** Beweis für den tatsächlichen Umstellungszeitpunkt und **kein** Kursdatum. Für die Startseite nennt sie `2026-10-15T11:25:17+02:00`, für die Edelmetallhandelsseite `2026-10-15T13:53:54+02:00`. [S2–S5]

## Identität / Domain / Betreiber

Das aktuelle Impressum auf `saxonia.de` nennt: [S6]

- **SAXONIA Edelmetalle GmbH**, Erzstraße 9, 09633 Halsbrücke/Sachsen.
- Registergericht Chemnitz, **HRB 31481**.
- USt-ID **DE 316 975 889**.
- Vertreten durch Dirk Hadlich und Mirko Niepel.

Startseite, Handelsseite und Kursseite führen dieselbe Unternehmensbezeichnung und Anschrift im Footer. Die Startseite bezeichnet SAXONIA Edelmetalle als Unternehmen der SAXONIA-Gruppe und verlinkt die **SAXONIA Holding GmbH** separat auf `saxonia-holding.de`. Das ist kein Beleg, die Holding oder deren Domain als Ersatz-Kursquelle einzusetzen. Die neue Kursseite bleibt auf derselben Domain. Ein historischer Register-/WHOIS-Vergleich wurde nicht durchgeführt; eine juristische Unverändertheit über die Zeit wird daher nicht behauptet. [S2, S3, S5, S6]

## Kurse: Datum, Richtung, Einheit

Die neue Seite schreibt **„Aktueller Tageskurs vom 15.10.2026“**. Sämtliche nachstehenden Werte und Kategorien sind direkt in ihren HTML-Buttons enthalten; keine JavaScript-Ausführung oder API-Abfrage ist für diesen Nachweis erforderlich. Zahlen hier mit dem Dezimalpunkt der Quelle wiedergegeben. [S5]

| Metall | ausdrücklich „Ankauf“, €/kg | „verarbeitet“, €/kg | „unverarbeitet“, €/kg |
| --- | ---: | ---: | ---: |
| Gold | 117611.5 | 131568.09 | 128488.20 |
| Silber | 1595.63 | 1881.55 | 1751.79 |
| Platin | 44731.94 | 53512.13 | nicht ausgewiesen |
| Palladium | 29786.96 | 34656.51 | nicht als eigener Button ausgewiesen |

- **Richtung:** Nur die ausdrücklich mit **„Ankauf“** beschrifteten Werte sind hier als Ankaufskurse nachgewiesen. Die anderen Kategorien dürfen nicht als zusätzliche Ankaufskurse übernommen werden. Die Handelsseite beschreibt sowohl An- als auch Verkäufe; die Kursseite benennt die beiden übrigen Kategorien lediglich „verarbeitet“/„unverarbeitet“. Eine weitergehende verbindliche Verkaufs-/Produktkondition wird daraus nicht abgeleitet. [S3, S5]
- **Einheit:** Die Seite nennt **€/kg**, auch für Gold, Platin und Palladium; nicht €/g und nicht €/Feinunze. Bei einer Darstellung in €/g wäre rechnerisch durch 1000 zu teilen, aber keine solche Umrechnung wurde hier vorgenommen. Eine Spezifikation des Feingehalts oder eine separate Umsatzsteuerangabe ist den untersuchten Kursbeschriftungen nicht zu entnehmen. [S5]
- **Datum:** Kursdatum 15.10.2026, getrennt von Abrufdatum, HTTP-`Date` und Sitemap-`lastmod`. Das eingebettete Chart enthält `labels: ["13.10.2026","14.10.2026","15.10.2026"]`; z. B. `GOLD Ankauf` mit `["119531.30","117941.80","117611.50"]`. Damit ist die Zuordnung des aktuellen Goldwerts zusätzlich innerhalb derselben Quelle nachvollziehbar. [S5]
- **Preischarakter:** Die Seite erklärt, die Werte basierten auf **Spot-Preisen gegen 9:00 Uhr** und seien **„lediglich … unverbindliche Richtwerte“**; aktuelle Preise seien bei Mitarbeitenden anzufragen. Eine Zeitzone für 9:00 Uhr wird dort nicht angegeben. Keine verbindliche aktuelle Abrechnungszusage und kein allgemeiner Schrottankaufpreis. [S5]

### Wichtige Quellengrenzen / widersprüchliche Seitenelemente

1. Der Textabruf liefert zusätzlich die Meldung **„Die Edelmetallpreise können derzeit wegen technischer Schwierigkeiten leider nicht angezeigt werden.“** Das Roh-HTML ordnet diese jedoch einem Container mit **`elementor-hidden-desktop elementor-hidden-tablet elementor-hidden-mobile`** zu. Gleichzeitig sind datierte Kurse vorhanden. Dies ist somit **kein belastbarer Nachweis einer aktuell sichtbaren Vollstörung**. Kein gerenderter Browser-/CSS-Test vorgenommen; die Einordnung stützt sich auf die expliziten HTML-Klassen. [S5]
2. **Palladium-Kategorien widersprechen sich intern:** Button, Tabellenkopf und `localizedLabels` nennen den Wert 34656.51 **„PALLADIUM verarbeitet“**; das eingebettete Chart-Dataset nennt dieselbe Wertfolge dagegen **„PALLADIUM unverarbeitet“**. Die vier expliziten Ankaufskategorien sind davon in den untersuchten Beschriftungen nicht betroffen. Keine automatische Zusammenführung/Umdeutung der Palladium-Nichtankaufskategorie empfehlen. [S5, S7]
3. Im statischen HTML ist die historische Tabelle zunächst leer; JavaScript baut sie dynamisch. Die eingebundene offizielle Datei `kursanzeiger.js` beschreibt POSTs an `/wp-content/themes/saxonia26/kursabfrage.php`. **Dieser Endpunkt wurde nicht aufgerufen**; historische Abfrage-, Export- und Browserfunktion sind daher nicht verifiziert. Für die aktuellen Ankaufskurse existiert bereits der oben beschriebene serverseitige HTML-Beleg. [S5, S7, S8]
4. Die über die deutsche Seite verlinkte englische Fassung zeigt dieselben aktuellen Zahlen, lässt die Kurskategorien aber ebenfalls auf Deutsch; sie klärt daher die Kategorien nicht zusätzlich. [S9]
5. Ein Suchmaschinenfund führte noch die alte URL mit älteren Zahlen (z. B. Gold 119531.3) auf. Solche Snippets sind **kein Live- oder Aktualitätsbeleg** und wurden nicht als Preisquelle verwendet; die direkte alte URL lieferte 404. Keine Archivkopie zur historischen Identitäts-/Pfadrekonstruktion untersucht.

## Empfehlung für die parallele Handlerprüfung

1. **Neue offizielle Quelladresse als konkreten Migrationskandidaten prüfen:** <https://saxonia.de/tageskursanzeiger/>. Ein bloßer Retry der alten URL behebt den hier reproduzierten Pfadfehler nicht. Keine fremde Gruppengesellschaft als Ersatzquelle verwenden.
2. Für einen späteren, separat autorisierten Fix nur die vier ausdrücklich beschrifteten **Ankauf**-Werte übernehmen, mit Kursdatum und **€/kg**; Richtwertcharakter bewahren. Numerische CSS-Klassennamen nicht als Metallidentität verwenden: z. B. der Gold-Ankaufbutton hat `class="metal-button gold"`, zahlreiche andere Buttons teilen jedoch `palladium` unabhängig vom Metall. Text-/Metallblock und Datenzuordnung sind maßgeblich. [S5]
3. Nicht allein auf HTTP 200 vertrauen: Kursdatum, vier Metall-/Ankaufzuordnungen, numerische Werte und Einheit validieren. Den versteckten Störungstext nicht blind als globale Sperre auswerten, aber bei tatsächlich fehlenden/undatierten Kursen keine alten Preise als frisch ausgeben.
4. **Pfadumstellung plausibel und Ersatzseite direkt belegt; Dauerhaftigkeit, Betreiberhistorie und volle dynamische Funktion nicht bewiesen.** Wiederholungsabruf/Monitoring und ggf. Rückfrage beim auf der Handelsseite genannten offiziellen Kontakt **Sebastian Meyer, +49 3731 2089-300, spmm@saxonia.de** sind angemessen. Keine Kontaktaufnahme im Rahmen dieser Recherche erfolgt. [S3]

## Direktquellen

- **S1:** <https://saxonia.de/edelmetallhandel/tageskurse/> — direkter 404-GET, Fehlerseiten-HTML und Header.
- **S2:** <https://saxonia.de/> — Roh-HTML-Navigation, Tageskurs-Hinweis, Gruppenstruktur und Footer.
- **S3:** <https://saxonia.de/edelmetallhandel/> — Roh-HTML-Navigation, An-/Verkaufsgeschäft und offizieller Kontakt.
- **S4:** <https://saxonia.de/robots.txt>, <https://saxonia.de/wp-sitemap.xml>, <https://saxonia.de/wp-sitemap-posts-page-1.xml> — offizielle Sitemap-Kette und neuer Pfad.
- **S5:** <https://saxonia.de/tageskursanzeiger/> — direktes HTML: Kurse, Datum, Einheiten, Kategorien, Chart-Datensätze, Richtwert-Hinweis und ausgeblendeter Störungstext.
- **S6:** <https://saxonia.de/impressum/> — aktuelle rechtliche Anbieteridentität.
- **S7:** <https://saxonia.de/wp-content/themes/saxonia26/js/kursanzeiger.js?ver=1.0> — eingebundener öffentlicher Clientcode, Labels und Formularabfrage.
- **S8:** <https://saxonia.de/wp-content/themes/saxonia26/wsb-kursanzeiger.js?ver=1.0> — eingebundener öffentlicher Clientcode für Export; nur gelesen, nicht ausgeführt.
- **S9:** <https://saxonia.de/en/daily-exchange-rate-display/> — offizielle, gegenseitig sprachverlinkte Kursseite; direkte Zahlenbestätigung, keine zusätzliche Richtungsklärung.
