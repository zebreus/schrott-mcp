# Review Wiedemann – 18.10.2026

## Urteil

**Feldrücknahmen und Stadtnormalisierung fachlich befürwortet; Diff wegen Datierung/Belegtext noch nicht unverändert integrationsreif. Keine Veröffentlichungsfreigabe.**

Geprüft: uncommitteter Diff gegen HEAD von `dossiers/sn/sn-leipzig-04288-wiedemann-wertstoffhandel-gartnerei.md`, README (insbesondere Quellenhierarchie/zweiseitige Prüfung) und `docs/trader-seed.md`. Dossier ausschließlich gelesen. Dieser Bericht ist die einzige geschriebene Datei. Keine Produktionsabfrage, kein Seed, keine Tests, keine Produktionsänderung, kein Geocoding und keine GEO-Verifikation durch diesen Reviewer; Integration/Tests liegen beim Owner.

## Exakte Feldentscheidung

| Feld | HEAD → vorgeschlagen | Entscheidung |
|---|---|---|
| `street` | `Liebertwolkwitzer Str. 49` → `''` | Rücknahme übernehmen. Nur Portalzuordnung, kein tragfähiger Betreiber-/Gewerbenachweis. Keine Behauptung, die Straße sei falsch oder der Betrieb geschlossen. |
| `postcode` | `'04288'` → `''` | Rücknahme übernehmen. Orts-/PLZ-Plausibilität ist kein Beleg einer aktuellen Betriebsanschrift. |
| `phone` | `034297 43296` → `''` | Rücknahme übernehmen. Kein unabhängiger Betreiberbeleg/keine geprüfte aktuelle Nummernzuordnung gefunden. Nicht als erwiesen falsche oder abgeschaltete Nummer darstellen. |
| `clear_fields` | fehlt → `'phone'` | Übernehmen und bei leerem Telefon beibehalten. Leer allein würde gespeichertes Telefon erhalten. Keine Erweiterung um Straße/PLZ/GEO: diese Tokens sind unzulässig. |
| `city` | `Leipzig 04288` → `Leipzig` | Übernehmen als syntaktische Normalisierung des bisherigen Ortsleads, **nicht** als neu verifizierter Betriebssitz. Keinen alternativen Ort/Ortsteil zum neuen Betriebssitz erklären. |
| `status` | `pruefung` unverändert | Beibehalten. Existenz/Identität und heutige Leistungen offen; kein Schließungsbeweis. |
| Name, Slug, Typ, übrige Leerfelder | unverändert | Keine neue Namens-/Betreiberzuordnung oder Leistungsanreicherung; Slug stabil lassen. Name/Typ bleiben Arbeitsidentität, keine Bestätigung eines aktuellen Wertstoffhofs. |
| `lat`/`lon` | fehlen weiterhin | Nichts ergänzen. Ein bestehender DB-Punkt wird dadurch nicht gelöscht oder verifiziert. |

Seed-Vertrag: Auf geändertem Seed ersetzen leere Straße/PLZ die gespeicherten Kernadresswerte; Stadt wird normalisiert. Telefon benötigt explizites Clear. Eine vorhandene Clear-Direktive wirkt auch bei später erneut angereichertem Telefon und unverändertem Seed-Hash; sie ist nicht nur eine einmalige Löschanweisung. Rebuild erforderlich. Quelle: `docs/trader-seed.md`, Abschnitte „Explicit enrichment clears“ und „GEO contract“; README „Trader seed“.

## Eigenständige zweiseitige Recherche

### A. Gesuchte Leipziger Identität

Am **18.10.2026 direkt abgerufen**:

1. <https://www.gelbeseiten.de/gsbiz/19137f15-0bf0-48b1-9ec7-8c6360b16607>
   - Seite nennt „Wiedemann Wertstoffhandel & Gärtnerei“, Liebertwolkwitzer Str. 49, 04288 Leipzig-Holzhausen, `034297 4 32 96` (Telefonlink `+493429743296`). Kategorie Gartenbaubetriebe; Leistungsumfang Altkleider, Altpapier, Buntmetall, Schrott, Wertstoffhandel.
   - **Tatsächlich direkt gelesener Aggregator, weiterhin ausschließlich Lead.** Kein ausgewiesener Inhaber/Registerbezug und im abgerufenen Text keine nutzbare Betreiberwebsite. „Website“ als Portalbedienelement ist keine verifizierte Betreiber-URL. Leistungsprosa bestätigt weder Ankauf noch Kundenannahme, Öffnungszeiten, Gebühren oder aktiven Hof.
2. <https://www.northdata.de/suggest.json?query=Wiedemann%20Wertstoffhandel&countries=DE>
   - Antwort tatsächlich `{"results":[]}`. Das ist nur ein negatives Suggest-Ergebnis dieses Suchstrings, **keine** vollständige Register-/Gewerberecherche und kein Nichtexistenz- oder Schließungsbeweis.

### B. Gegenprüfung Adresse/Telefon und Namesakes

Unabhängige Websuchen mit `"Liebertwolkwitzer Straße 49"`, `"Liebertwolkwitzer Str. 49" -site:gelbeseiten.de`, `"034297" "43296"`, `"03429743296"`, Namen + Leipzig/Holzhausen sowie Namen + Northdata/Creditreform ergaben keinen belastbaren zweiten Nachweis für diesen Betrieb und keinen belastbar identifizierten anderen aktuellen Betreiber an Hausnummer 49. Mehrere Suchantworten waren offensichtlich unspezifisch (z. B. fremde Straßenlisten/CVE-Nummern): nicht verwertet. Das Scheitern dieser begrenzten Suche beweist weder einen leeren Standort noch Exklusivität der Portalzuordnung. Telefon wurde nicht angerufen.

**Direkt abgerufene Namensgegenproben am 18.10.2026:**

- <https://www.schlosserei-wiedemann.de/> und <https://www.schlosserei-wiedemann.de/kontakt/>: Betreiberauftritt „Schlosserei Wiedemann“, Kontakt nennt Inhaber **Ingo Wiedemann**, Mierendorffstraße 15, 04318 Leipzig, Telefon **0341/6891642**, USt-ID DE266152061. Relevanter lokaler Metallbau-Namesake, aber andere Anschrift/Nummer und kein Zuordnungsbeleg zum Wertstoffhandelslead. **Keine Übernahme** von Inhaber, Website oder Kontakten in das Dossier; keine behauptete Rechtsnachfolge oder gesicherte vollständige Identitätstrennung.
- <https://blumen-wiedemann.de/gaertnerei-wiedemann> und <https://blumen-wiedemann.de/impressum/>: eigene Gärtnereiseite nennt Weiherstraße 4, **86899 Landsberg am Lech**, **08191–50505**; Impressum nennt Franz Wiedemann, Blumengeschäft/Gärtnerei und HRB 18783, Amtsgericht Augsburg. Standort- und Telefongegenprobe zum Gärtnerei-Namen, **kein Leipziger Beleg**. Registerangabe nur als Selbstauskunft gelesen, Registerkongruenz nicht geprüft; deshalb keine autoritative Primärquellenausnahme für irgendeinen Leipziger Fill.
- <https://www.wiedemann.de/ueber-uns/unternehmensgruppe/wiedemann-sarstedt>: eigener Auftritt nennt **WIEDEMANN Sarstedt KG**, Wiedemannstraße, 31157 Sarstedt, **05066 997-0**, haustechnischen Fachgroßhandel. Weiterer Namensgleicher, kein Beleg einer Verbindung zum Leipziger Wertstoffhandelslead. Keine Übernahme.
- <https://www.leipzig.de/mein-stadtteil/stadtbezirk-suedost/holzhausen>: kommunale Ortsteilseite direkt abgerufen (umfangreiche Navigation); Suchauszug beschreibt Holzhausen als Leipziger Stadtteil. Allenfalls Ortskontext, **kein Gewerbe-/Betreiberbeleg** für Hausnummer 49. Keine Koordinaten ermittelt oder ausgewertet.

Nur Suchtreffer, nicht als Direktseitenbelege verwendet: Gelbe-Seiten-Branchenliste <https://www.gelbeseiten.de/branchen/schrotth%C3%A4ndler/leipzig> (derselbe Portalbestand, nicht unabhängig); kommunales Straßenabschnittsverzeichnis <https://www.leipzig.de/fileadmin/mediendatenbank/leipzig-de/Stadt/02.1_Dez1_Allgemeine_Verwaltung/12_Statistik_und_Wahlen/Raumbezug/Strassenabschnittsverzeichnis_Leipzig.txt> (nicht direkt gelesen, kein Betriebsnachweis); Treffer zu Richard Wiedemann/Geislingen/Krailling, historischem L. Wiedemann Leipzig und weiteren gleichnamigen Firmen. Auch Betreiber-Kontakt und Impressum derselben Domain sind keine zwei unabhängigen Quellen.

**Ergebnis:** Für die konkrete Leipziger Arbeitsidentität weiterhin kein zulässiges Quellenpaar und keine für sie verifizierte autoritative Betreiberprimärquelle. Die zweiseitige Recherche liefert keine tragfähige Ersatzadresse/-nummer und erlaubt keine Verschmelzung mit einem Namesake. Ein Portalrest bleibt als offener Kandidat unter `pruefung`, nicht als verifizierte reale Annahmestelle.

## Notwendige Datierungs- und Textkorrekturen durch Owner

1. Die neu angefügten Überschriften/Bullets `Korrektur 12.10.2026` und `Owner-Korrektur 12.10.2026` sind im aktuellen uncommitteten Diff vorhanden. Dieses Review belegt **keine** Handlung oder DB-Abfrage am 12.10.2026. Falls keine zeitgenössischen Nachweise vorliegen: aktuelle Rücknahme als **18.10.2026** dokumentieren; historischen DB-Abgleich nicht durch bloßes Umdatieren als heutigen eigenen Abruf ausgeben. Bei vorhandener Historie exakt auf ihren tatsächlichen Bericht/Querynachweis verweisen und Urheber/Zeitpunkt trennen.
2. Insbesondere „read-only-Abfrage 12.10.2026“ sowie die konkreten Bestandswerte inklusive Koordinaten sind hier nur **Behauptungen des zu prüfenden Difftexts**, nicht von mir erhobene Produktionsbefunde. Owner muss sie belegen oder als nicht erneut geprüfte historische Angabe kennzeichnen/aus dem neuen Befundtext entfernen. Keine erfundene aktuelle Produktionsprüfung.
3. Historische Timeline vom 02./08.10.2026 unverändert als Historie erhalten, aber aktuelle Korrektur ausdrücklich klarstellen lassen: „Ausnahmefall“/„aus Historie erhalten“ begründen **keine zulässige Ausnahme** vom README-Standard. Der heutige Direktabruf legitimiert frühere Abrufdaten oder frühere Fills nicht rückwirkend.
4. Im neuen Quellenzusatz statt „zuvor direkt geprüft 08.10.2026“ sauber: **„Gelbe-Seiten-Detail am 18.10.2026 erneut direkt abgerufen; Aggregator-Lead, kein Betreiberbeleg. Ältere Abrufbehauptungen nicht Gegenstand einer Nachverifikation.“** Keine alten Recherchehandlungen als eigene ausgeben.
5. „Leipzig als Stadt beibehalten“ präzisieren zu **„Ortslead Leipzig lediglich um den PLZ-Anhang bereinigt; aktuelle Betriebsidentität und Geschäftsanschrift nicht verifiziert“**. Überblick darf keine Phantom-Sicherheit erzeugen.
6. Den Satz zum Telefon präzisieren: **„Leeres Telefon plus `clear_fields: 'phone'` zieht den gespeicherten Telefonlead beim Seed explizit zurück; ein leeres Telefon allein würde ihn erhalten.“** Adresse und GEO getrennt erklären; `pruefung` ist kein technischer Veröffentlichungsfilter.

## Gates / Übergabe

- **Ungeprüfter erhaltener GEO bleibt Veröffentlichungsblocker**, auch nach erfolgreichem Clear und Adressrücknahme. Nicht aus der scheinbar passenden Stadt oder Portaladresse validieren; keine GEO-Freigabe aus diesem Review. Der im Diff genannte Punkt wurde nicht überprüft.
- Betreiber-/Gewerbenachweis, aktuelle Anschrift, aktuelle Nummernzuordnung, öffentlich zugängliche Annahme, Materialannahmebedingungen und Ankauf/Verkauf/Gebühren bleiben offen. Kein neues Preis-/Leistungsstatement.
- Owner korrigiert Datierung/Text, integriert die oben bestätigte Feldentscheidung und testet den Seed-Vertrag. Keine ungeprüfte Veröffentlichung oder produktive Seed-Anwendung allein auf Basis dieses Berichts; verbleibende GEO-/Identitätsblocker müssen separat aufgelöst werden.
