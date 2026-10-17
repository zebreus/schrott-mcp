# Mairitsch — Nachweisprüfung H1

Prüfung am 17.10.2026 zu `research/wave-20261016-1300-bc-review.md`, H1. Keine erneute Website-Recherche, kein CAPTCHA/Login-Versuch, keine Dossieränderung.

## Ergebnis

**Die ursprüngliche lesbare Werkzeugantwort existiert im Gesprächskontext dieser Recherche-Sitzung. Ein separat gesichertes HTTP-Rohdokument mit genauem Abrufzeitpunkt und Originaldatei-Hash liegt mir dagegen nicht vor. Der aktuelle Livebefund ist durch mich nicht erneut reproduziert; der vom Reviewer/Owner gemeldete Identitätsprüfungsbefund wird nicht widerlegt. H1 deshalb nicht als unabhängig reproduziert oder bestanden behandeln.**

## Konkret vorhandener Originalbeleg

In der ursprünglichen Gruppen-B-Recherche steht ein `multi_tool_use.parallel`-Aufruf mit diesen sechs Direktabrufen, in dieser Reihenfolge:

1. `https://koenig-wilhelm.com/Versandinformationen`
2. `https://schrottplatzmetall.de/gebruder-dubovic-gladbeck/`
3. **`https://www.ebay.de/str/autoverwertungmairitsch?_tab=about`** — Tool `functions.webfetch`, Argument `url` genau wie angegeben.
4. `https://www.gelbeseiten.de/gsbiz/46856cf5-a20d-4b72-8165-2093a16333ad`
5. `https://www.gelbeseiten.de/gsbiz/2b6fa38a-0efb-47e4-8cc2-c126f9d14327`
6. `https://www.gelbeseiten.de/gsbiz/cb6485ca-50dc-4f92-8d0e-4df27797e377`

Die Antwort des dritten Abrufs ist im Gespräch als eigene `functions.webfetch`-Werkzeugantwort vorhanden, **nicht bloß als von mir verfasste Recherchebehauptung**. Sie enthält Shopnavigation, Abschnitt „Über uns“, Abschnitt „Angaben zum Unternehmen“ und eBay-Footer. Der entscheidende Abschnitt lautet dort wörtlich in der vom Werkzeug gerenderten Markdown-/Textdarstellung:

> ## Angaben zum Unternehmen
>
> Unternehmensname: Andreas Mairitsch \- Autoverwertung MairitschVorname: AndreasNachname: MairitschAdresse: Dortmunder Str. 88 \- 90, 59427, Unna, GermanyTelefonnummer: \+49 2303255080E-Mail: autoverwertung@mairitsch.deUSt-IdNr. DE 340018138EPR: DE1492726284790Ergänzende Angaben: Geschäftsführer: Andreas Mairitsch

Der Abschnitt läuft anschließend mit dem Hinweis auf die EU-Streitbeilegungsplattform und Kundeninformation zur Vertragstextspeicherung weiter. Unmittelbar davor nennt die Werkzeugantwort `Standort: Deutschland`, Anmeldung seit `16. Apr. 2018` und Verkäufer `autoverwertung-mairitsch`.

**Dieses Zitat ist eine jetzt angefertigte Übertragung aus der ursprünglichen Werkzeugantwort, keine damals gespeicherte Rohdatei.** Die Markdown-Formatierung ist nicht identisch mit HTTP-Responsebytes; daraus wird kein Originaldatei-Hash konstruiert. Die damals geschriebene Dossiernotiz und der Gruppenbericht sind abgeleitete Aussagen und keine zusätzlichen unabhängigen Belege.

## Zeit-, Datei- und Bindungsgrenzen

- Der Abruf gehört zur am **16.10.2026** geführten ursprünglichen Recherche vor dem großen Patch aller20 B-Dossiers. Das Datum ist aus dem Gesprächsablauf/Recherchedatum zugeordnet, **nicht aus einem HTTP-Zeitstempel**.
- Im hier sichtbaren Werkzeugprotokoll fehlen genaue Uhrzeit/Zeitzone, Responseheader, HTTP-Rohbytes, Cacheherkunft und ein exportierbarer Toolcall-/Message-Identifier. „13:00 B“ ist der Wellenname, **keine nachgewiesene Abrufuhrzeit**.
- Gezielte Inhaltssuche nach `DE340018138`, `DE 340018138` und `DE1492726284790` in `/root/.local/share/opencode/tool-output` sowie `/root/.local/share/opencode/shell` ergab **keine Treffer**. Eine pfadbasierte Suche nach `*mairitsch*` unter `/tmp/opencode` fand nur Dossierkopien in anderen Arbeitsverzeichnissen, keine erkennbare Abrufsicherung.
- Diese Suchgrenzen sind kein Nachweis, dass nirgendwo ein persistiertes Sitzungsereignis existiert. Die lokale OpenCode-Sitzungsdatenbank wurde **nicht geöffnet oder exportiert**; ebenso keine Produktionsdatenbank. Eine gesicherte Originaldatei mit URL/Abrufzeit kann ich daher hier nicht referenzieren und **keinen Originalbeleg-SHA256 angeben**.
- Die obige Reihenfolge und der exakte URL-Parameter machen die Werkzeugantwort im ursprünglichen Gespräch auffindbar. Sie ersetzen aber keine zeitgestempelte, unabhängig zugängliche Archivierung. Ein Hash dieses neu geschriebenen Berichts würde nur die jetzige Transkription binden, nicht die damalige Websiteantwort.

## Konsequenz für Owner

Historischer Befund: tatsächlich lesbarer Inhalt in der ursprünglichen Webfetch-Antwort dieser Sitzung; keine bloße nachträgliche Wiederholung aus dem Dossier. **Unabhängige aktuelle Reproduktion und belastbar zeitgestempelte Rohabrufbindung fehlen.** Soweit das Gate einen solchen extern prüfbaren Originalbeleg verlangt, bleibt die präzise neue Betreiberzuordnung **nicht reproduziert / H1 offen**. Keine Ableitung aktueller Annahme oder UG-/KG-Rechtsnachfolge. Über Zurückstellung/Kennzeichnung der Dossierbehauptung entscheidet der Owner; hier unverändert gelassen.
