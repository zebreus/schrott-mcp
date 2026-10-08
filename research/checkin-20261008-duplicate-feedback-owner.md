# Owner-Check-in — doppelte Feedback-Leads, 08.10.2026

## Gesundheit und neuer Stand

Service aktiv; sechs Stunden Warning-Journal ohne Einträge. Read-only:
3878 Händler, 52 Materialien, 2280 aktuelle Preise. Abgeschlossene Läufe
1139–1142 ok, null Händlerfehler. NORDKAT weiterhin ohne neuen regulären
Schritt nach Fix; alter 404 in Lauf 1126 nicht als neuer Fehler ausgegeben.
Working Tree zu Beginn sauber; vier vorherige Prüfdossiers bereits live.
Neu seit letzter Sichtung #5508: #5509–5517, neun Meldungen;
letzte zehn Titel geprüft. Schwerpunkt dieses Check-ins: #5517.

## Hinweis #5517 gegen Meldungen und Bestand geprüft

Alle zwölf genannten Paare in `internal.db` gelesen und Namen/Standort-
angaben verglichen. Das ist **Deduplizierung von Recherche-Leads**, keine
vollständige Bestätigung des Geschäftsbetriebs oder seiner Annahmekonditionen.
Pro Paar nur einen Recherchefall führen, beide IDs als Herkunft erhalten:

| Betrieb/Standort | Feedback-Paar | Einordnung |
|---|---|---|
| Oblinger Entsorgungsfachbetrieb, Mittersteigweg 16/Pförring | #4831/#5316 | Ein Leadpaar; nicht der bestehende Michael-Oblinger-Händler in Ingolstadt |
| Böck, Finninger Straße 68/Neu-Ulm | #4832/#5313 | Ein Leadpaar, Betreiber-/Zeitenprüfung offen |
| Tadick, Bahnhofstraße 20/Beckum | #4859/#5274 | Ein Leadpaar, vollständige Betreiberprüfung offen |
| Hoffmann & Ernst, Friedrich-Hoffmann-Straße 1/Gröningen | #4901/#5494 | Ein Standortleadpaar; Blankenburg-Bestand ist anderer Standort |
| Carnuth, Sachsenring 23/Straubing | #4912/#5324 | **Bereits live erfasst**, Fehlend-Behauptung erledigt; Bedingungen bleiben pruefung |
| MAR, Linzer Straße 10/Nürnberg | #4922/#5322 | Ein Standortleadpaar; Lauingen nicht Nürnberg und Bremer Straße 163 separat prüfen |
| Uwe Schero, Martin-Luther-Straße 10/Sprockhövel | #5030/#5275 | Ein Leadpaar, keine belegte Betreiberidentität allein aus Portalen |
| SRT Pößneck, Am Oberen Bahnhof | #5070/#5505 | Ein Leadpaar; Hausnummern 0/31 nicht aus Verzeichnissen auswählen |
| Scholz Apolda, Flurstedter Marktweg 9 | #5075/#5503 | Ein Leadpaar, nicht W.K.W. Am Kalkteich 6 |
| Aperam Minden, Windmühlenstraße 32 | #5107/#5282 | Ein Standortleadpaar, Duisburg/Dresden nicht Minden |
| ERG Aschaffenburg, Limesstraße 20 | #5110/#5258 | Ein Leadpaar; Betreiber-/Rechtsformkette noch prüfen |
| LRP Leipzig, Brahestraße | #5214/#5479 | Ein Standortleadpaar; Krostitz/Leipzig-Sammelname ist kein Beweis für Brahestraße |

Alle Abgleiche ausschließlich SELECT. Keine Feedbackzeile gelöscht oder
verändert, keine direkte Produktionskorrektur, kein Händlerzusammenführen.
Die Aussage von #5517 „jeder fehlt nur einmal“ ist bei Carnuth bereits
überholt. Bei den übrigen elf Paaren ist „Meldungen zusammenfassen“ nicht
gleichbedeutend mit „Bestandslücke abschließend belegt“.

## Carnuth-Fall konkret abgeschlossen

Produktionszeile `by-straubing-carnuth-stahlrecycling`, Sachsenring 23,
Straubing vorhanden. Aktuelle Betreiberkontaktseite live erneut gelesen:
Werk Straubing, Adresse, 09421 9254-0 kongruent; Werk Bogen getrennt.
Quelle: https://www.carnuth.de/kontakt/.
Frühere Register-/Zertifikatsidentität und Gegenbelege im Dossier erhalten.
Neuer Timeline-Eintrag verknüpft beide alten Meldungen und #5517.
Kein Stammdaten-Overwrite, keine neuen Preise/Koordinaten, keine Auflösung
der weiterhin widersprüchlichen Freitag-/Anlieferzeiten behauptet.

## Offene Aufgaben / Limits

Vorherige Rechercheagenten abgeschlossen bzw. Nutzungslimit; ihre vier
Ergebnisse sind owner-geprüft und deployed. Keine Wiederholungsaufrufe
gegen das bekannte Limit. Keine Rust-Geocodierung gestartet; Koordinaten
weiterhin dossierbasiert. Übrige neue Händler-/Handler-Leads bleiben offen.
Git-Push weiterhin ohne verfügbare HTTPS-Zugangsdaten; lokalen Commit
nicht als Remote-Veröffentlichung ausgeben.

## Owner-Gate

Carnuth-Frontmatter vollständig unverändert: keine Enum-/Root-URL-/JSON-
oder Koordinatenänderung; sämtliche alten Timeline-Bullets erhalten.
Diff-Check und Rustfmt grün, acht Seedtests bestanden, null Fehler.
Änderung wird regulär über eingebetteten Dossier-Seed veröffentlicht,
nicht über manuelle Produktions-Notizen oder Feedback-SQL.
