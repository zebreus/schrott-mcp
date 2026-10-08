# Recherchewelle 08.10.2026 C — Start und Owner-Check-in

Die doppelt zitierte Wellen-Anforderung einmal ausgeführt. 60 Slugs frisch
aus der Produktionsdatenbank read-only ausgewählt: `website='' OR street=''`,
zufällig jeweils 20 aus drei disjunkten Bundeslandgruppen. Auch erneut
gezogene Dossiers bleiben bewusst im aktuellen Pool; keine Auswahl aus
alten Ergebnisdateien. Konkrete Sluglisten stehen in den Agentenaufträgen.

| Shard | Länder | Session | Bericht |
| --- | --- | --- | --- |
| Süd/Ost | BW, BY, SN, TH | ses_ee3531440ffe4byedoSHQKZd44 | research/wave-20261008c-suedost.md |
| Mitte/Nordost | BE, BB, MV, ST, HE | ses_ee353143bffeXtyILKsXUHTajB | research/wave-20261008c-mittenordost.md |
| Nord/West | NI, NW, HB, HH, SH, RP, SL | ses_ee353143affewTufVNIPpRADYj | research/wave-20261008c-nordwest.md |

Alle drei Hintergrundaufträge angenommen. Kurze ergebnisorientierte
Aufträge mit methodischem Freiraum; gemeinsame Veröffentlichung erst nach
Ergebnissen und Owner-Gate. Noch keine abgeschlossene Welle und keine
Freigabe der Agentenänderungen. Produktionsdatenbanken bleiben read-only.

## Gleichzeitig angeforderter Check-in

Dienst aktiv, öffentliche HTTPS-Health `ok:true`, keine Warnungen im
Sechs-Stunden-Fenster. 3881 Händler / 52 Materialien / 2280 Current-Preise.
Zehn jüngste Feedbacktitel erneut gelesen, weiterhin maximal #5517:
kein neues Feedback gegenüber dem vorigen Check-in. Die offenen
Autoverwertungs-Leads #5508–#5516 sind dadurch nicht als verifiziert erledigt.

Ingestion #1151 ok, null Fehler. #1152 partial, 17 Preise übernommen,
ein Requestfehler bei `hh-hammerbrook-madi-metall-recycling` gegen
`https://www.madi-schrott.de/`. Ein anschließender direkter HTTPS-Abruf
lieferte HTTP 200 ohne TLS-Ausnahme. Das ist nur Erreichbarkeit beim
Nachtest, kein grüner Handler-/regulärer Ingestionnachweis und keine
bewiesene Fehlerursache. Keine spekulative Parseränderung.

Nach Abschluss aller Shards: Historien-/Overwrite-/Enum-/Rootlink-/
Platzhalter-/Notenprüfung, Seedtests und Rustfmt, Commit, Pushversuch,
beide Binaries aus sauberem Stand deployen und Produktion spotprüfen.

## Süd/Ost abgeschlossen — Teilreview

Session `ses_ee3531440ffe4byedoSHQKZd44` regulär abgeschlossen, Bericht
vollständig gelesen. Wolf- und ECOPROEKT-Frontmatter-/Timeline-Diffs
gesichtet: Wolf trennt belegte Entsorgungsannahme von unbewiesener
Vergütung; ECOPROEKT bleibt Handel/Makeln, keine behauptete öffentliche
Annahmestelle. Quellen-Nachprüfung und gemeinsamer vollständiger Gate
stehen noch aus. Keine neuen numerischen Preislisten gemeldet.
Die anderen Shards werden nicht gepollt oder neu gestartet.

## Nord/West abgeschlossen — Bericht gelesen

Session `ses_ee353143affewTufVNIPpRADYj` regulär abgeschlossen, Bericht
vollständig gelesen. Wesentliche Owner-Prüfpunkte: Kurzname Fitz versus
Fritz Eckhardt, bestehender TuS-Aktivstatus ohne Betreiberbelege und
SK-Mayen-Personen-/Adresswiderspruch. Die 15 EUR je Kühlgerät/Bildschirm/
Fernseher sind ein Gebührenfund, kein Ankaufpreis; Steuerbasis und
Gültigkeitsdatum bleiben offen. Biskupek-Liquidation bedeutet keinen
pauschalen Schließungsnachweis für alle Händler im Sammeldossier.
Agent meldet acht grüne Seedtests im gemeinsamen Arbeitsbaum. Eigener
abschließender Test/Gate und Veröffentlichung stehen weiter aus.

## Gemeinsamer Owner-Gate

Alle drei Shards regulär fertig; alle Berichte vollständig gelesen.
60 Dossiers geprüft: YAML lesbar, stabile Slugs, Status-/Website-Enummen,
Root-Websites und wortgetreuer Erhalt jeder ursprünglichen Timeline-Bullet
maschinell geprüft. Frontmatter-Änderungsliste vollständig gesichtet.
Keine Koordinatenänderungen, keine neue Schließungsbehauptung.

Wolf-Impressum, getrennte Öffnungszeiten und Registerprofil erneut gelesen;
aktuelle Efb-Kopfseite bestätigt HRA7265/Röntgenstraße11 und Gültigkeit
04.11.2027. Eckhardt-Impressum/Standortzeiten, Register-Namenshistorie und
Bildzertifikat unabhängig gelesen: HRA4174, Prinzenstraße58, ESN97-080009(26)
bis31.07.2027. Fitz-Suchbrücke bleibt dokumentierter verkürzter Seedalias,
kein zweiter behaupteter Betrieb. ECOPROEKT-Register und Zertifikatsseiten
1/2 unabhängig gelesen: HRB798917, ausschließlich Handel/Makeln in der
geprüften Anlage; keine öffentliche Anlage daraus abgeleitet.

DAR-Kontakt-Fills zurückgenommen: Register bestätigt Identität/Adresse,
nicht Telefon/Mail; mangels nachgewiesener vollständiger Primärquellen-
Ausnahme bleiben diese Einzelangaben in der Timeline. TuS-Aktivstatus auf
pruefung korrigiert, weil erhaltene Recherche nur Verzeichnisbelege trägt.
Wetzel/Hinze-Prüfstatus nachvollziehbar, keine ungeprüfte Schließung.

SK-Gebührenseite unabhängig gelesen: 15 EUR pro genanntem Gerät, inklusive
Entsorgungsnachweis, Steuerbasis/Gültigkeit offen. Schulz-Greifswald-Tabelle
erneut gelesen: Papier0,06/Glanzpapier0,08/Bücher0,03 EUR/kg; Metall nur
Tagespreis, Pappe ausdrücklich keine Annahme. Keine numerischen Metall-
Ankaufpreise und keine neuen Handler/Preisimporte in dieser Dossierwelle.

Abschlussgates: acht Seedtests bestanden, null Fehler; Rustfmt und
`git diff --check` grün. Zusätzlicher 60-Datei-Gate bestätigt YAML,
Status-Enummen, unveränderte Händlertypen, Root-Websites, Slugs,
vollständige alte Timeline-Bullets und entfernte Überblick-Platzhalter.
Ein erster ad-hoc Typcheck nutzte eine unvollständige Liste und schlug
deshalb fehl; kein Dossierfehler. Korrigierter Gate und echte Seedtests grün.
Standortkoordinaten für Wolf/Eckhardt fehlen weiterhin; ECOPROEKT ist nur
Geschäftsadresse. Keine neuen Punkte erfunden und keine Rust-Geokodierung.
