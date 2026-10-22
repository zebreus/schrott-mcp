# Owner-Gate — Recherchewelle 21.10.2026, 00:00 UTC

Abschluss der Owner-Prüfung am 21.10.2026. Die Zwischenaufnahme von 00:12 UTC bleibt als zeitgestempelte Historie erhalten.

## Umfang und Dossier-Gate

- Der Scope enthält 60 eindeutige Händler aus RP, ST und SN, je 20 pro Bundesland. Eine erneute Abfrage mit read-only/`query_only` fand alle 60 Datensätze und alle 60 weiterhin im Auswahlpool `website='' OR street=''`.
- Alle 60 Scope-Dossiers wurden geändert. Der Diff-Audit fand jetzt 69 neu hinzugefügte, datierte Timeline-Belege mit Quellen-URL: 60 Rechercheeinträge und neun Owner-Nachprüfungen.
- YAML ist parsebar; `slug`, `state` und `trader_type` blieben unverändert. Status-Enums, Drop-off-/Pickup-JSON und `clear_fields` sind gültig. Frühere Timeline-Belege blieben erhalten. Kein Dossier-Koordinatenpaar wurde ergänzt oder geändert; keine `website`-Deep-Links oder neu eingefügten Em-Dashes.
- Eine bereits zuvor owner-geprüfte Ausnahme bleibt unverändert: `sn-chemnitz-tsr-deutschland-nl-chemnitz` enthält das belegte Paar 50.860415, 12.941881 als Gebäude-Referenzpunkt. Es ist kein Eingangspunkt. Die Produktionszeile hat noch das alte, rund 4,93 km entfernte Paar; der Seed würde die dokumentierte Korrektur anwenden.
- Der erneute `git diff --check` für RP/ST/SN ist sauber. Die Prüfung änderte keine Produktionsdatenbank.

## Quellen- und Identitätsentscheidungen

- **Weinand / Bad Kreuznach:** Die Betreiberwebsite belegt den gemeinsamen Standort und die Metall-/Elektroschrottannahme, aber löst die Zuordnung der Geschäfte zwischen Weinand Entsorgungs-GmbH und Weinand Recycling GmbH & Co. KG nicht auf. Der Datensatz bleibt ein Standort-/Marken-Dossier, keine behauptete Firmenverschmelzung. Die konkrete Standortseite schließt nichtmetallische Anlieferungen seit 25.04.2025 aus und verweist an REMONDIS; allgemeinere AGB-Erwähnungen von Kunststoff/Papier wurden nicht auf diesen Standort übertragen.
- **Torsten Schmidt / Trier:** Die Homepage ist erreichbar und bewirbt Schrottankauf, Abriss und Demontage. Der verlinkte Impressumspfad liefert aktuell HTTP 404; es fehlen verifizierte Rechtsform, Registerangabe und Kundenannahmeadresse. Daher wurden keine Kontakt-/Adressfelder gefüllt.
- **Kopp / Neustadt:** Verzeichnisse widersprechen sich bei Telefonnummer und Betriebsstatus. Branchweilerhof 17b bleibt ein Lead; kein Betreiber-/Registerbeleg und keine Frontmatter-Übernahme.
- **Landau Kat-Ankauf:** Das Kat-Ankauf-Profil und das 705 m entfernte Cadorin-Profil haben unterschiedliche Adressen/Nummern. Keine Fremdzuordnung.
- **Koblenz:** Die Betreiberwebsite des Koblenzer Schrotthandels belegt Karin Reinhardt und eine Metallannahme, aber keinen Zusammenhang mit dem generischen Seed „Koblenzer Entrümpelungen“; keine Übertragung.
- **Fuhlrott / Schönebeck:** Quellen führen weiterhin abweichende Inhaber-/Namensstände und widersprüchliche Öffnungszeiten. Keine Aktualisierung dieser Felder.
- **Schliwa / Calbe:** Der einzige Verzeichniseintrag widerspricht sich bei Donnerstag-Zeiten. Zeiten wurden zurückgenommen, `status` auf `pruefung` gesetzt und `clear_fields: 'opening_hours'` ergänzt, damit der leere Wert beim Seed das alte Feld tatsächlich entfernt. Das belegt weder Schließung noch fehlende Annahme.
- **Habermann / Dessau:** MHG erklärt die Übernahme des Entsorgungsfachbetriebs und den Standortbetrieb ab 01.08.2026. Das belegt Standortkontinuität laut aktuellem Betreiber, nicht rechtliche Identität mit dem Legacy-Datensatz; MHG-Kontakte wurden nicht in Habermanns Stammdaten übernommen.

Ankauf, Verkauf und Gebühren bleiben getrennt. Allgemeine Marktpreiswerbung, Mindestmengen, Plattformwerte und Unternehmensregister wurden nicht als konkrete lokale Ankaufskurse oder Annahmebelege ausgegeben. Unbestätigte Adressen/Partner bleiben Leads statt erfundener Filialen.

## GEO- und Seed-Rollout-Blocker

Der read-only-Abgleich fand in 56 von 60 Produktionszeilen ein vollständiges GEO-Paar; vier Zeilen haben kein Paar. Nur das bereits owner-geprüfte TSR-Dossier führt ein explizites Paar. Bei den übrigen Dossiers ohne Paar erhält der Seed nach Vertrag das bestehende Produktions-GEO. Daher bedeutet „keine Koordinate im Dossier geändert“ nicht, dass ein vorhandener Produktionspunkt korrekt ist.

1. **Weinand:** Die Lageplan-Sektion von `weinand.de` nennt 49.883796, 7.851105. Das ist ein Betreiberpunkt, aber nicht unabhängig als Eingang verifiziert. Die Produktionszeile enthält weiterhin kein gespeichertes `street`/`postcode`, aber das alte GEO 49.8426, 7.85125, rund 4,6 km entfernt. Das Dossier lässt `lat`/`lon` leer; ohne Klärung würde der Seed den Altpunkt erhalten.
2. **Habermann/MHG:** Die Produktionszeile enthält 51.8333, 12.25. Die Postleitzahl 06847 wurde für die vom aktuellen Betreiber bestätigte Adresse Taubenstraße 37 ergänzt, jedoch kein neues Koordinatenpaar. Der Altpunkt ist nicht unabhängig als heutiger MHG-Annahmepunkt bestätigt.
3. **Weitere Alt-GEOs, read-only reverse-geprüft am 21.10.2026:** Die vorhandenen Punkte führen bei Nominatim (`zoom=10`) jeweils in einen anderen Ort/Verwaltungsbereich als die dokumentierte Händleradresse: Kopp/Neustadt 51.4672, 10.4673 → Am Ohmberg, Thüringen, gegenüber Branchweilerhof 17b/67433 Neustadt, RP ([Reverse](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.4672&lon=10.4673&zoom=10)); Rilei/Kalbe 53.2934, 9.5506 → Kalbe, Niedersachsen, gegenüber dem Adress-Lead Cheinitzer Rundling 44/39624 Kalbe (Milde), ST ([Reverse](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.2934&lon=9.5506&zoom=10)); Fuhlrott/Schönebeck 53.0595, 12.2331 → Gumtow, Brandenburg, gegenüber Calbesche Str. 51/39218 Schönebeck ([Reverse](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.0595&lon=12.2331&zoom=10)); Stiegler/Lichtenstein 48.4396, 9.2528 → Lichtenstein, Landkreis Reutlingen, Baden-Württemberg, nicht Lichtenstein/Sachsen ([Reverse](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=48.4396&lon=9.2528&zoom=10)); Köhn/Glashütte 52.21815, 14.0135 → Wendisch Rietz, Brandenburg, gegenüber Glashütte/01768 ([Reverse](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=52.21815&lon=14.0135&zoom=10)); Knobloch/Neustadt i. Sa. 50.74755, 13.2756 → Großhartmannsdorf, Sachsen, gegenüber Am Güterbahnhof/01844 Neustadt ([Reverse](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.74755&lon=13.2756&zoom=10)); Schrott24/Halle 51.60848125, 11.27634375 → Mansfeld/06343, nicht Halle; die Plattform benennt den lokalen Partner nicht öffentlich ([Reverse](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.60848125&lon=11.27634375&zoom=10)). Diese Rückwärtssuche identifiziert nur den kartierten Verwaltungsbereich des bestehenden Punkts, nicht den Händler, die Zufahrt oder die richtige Ersatzkoordinate. Kein Punkt wurde übernommen oder verändert.

Die 13 GEO-Fälle der unmittelbar vorherigen 06:00-Welle bleiben ebenfalls offen. Damit sind mindestens 15 konkrete Adress-/GEO-Fälle aus den zwei Wellen vor einem Seed-Rollout zu prüfen; die zusätzlichen Verdachtsfälle oben erhöhen den nötigen GEO-Abgleich. Das bestehende TSR-Paar ist gesondert als bereits dokumentierte, beabsichtigte Korrektur zu behandeln. **Keines der neu erkannten GEO-Probleme wurde durch Koordinatenänderung oder Produktionsschreibzugriff „gelöst“.** Keine Rust-Geokodierung.

## Tests und Veröffentlichung

- Root frei bei Abschluss: rund **1.041,8 MiB**, etwa **158,2 MiB unter** dem dokumentierten Seed-Testminimum von 1.200 MiB. `/root/.cargo/bin/cargo` ist vorhanden, aber nicht im PATH. Es liefen keine Cargo-/Rust-Prozesse; deshalb kein Seed-Test und kein Build.
- `main` liegt 32 Commits vor `origin/main`. Im gemeinsamen Arbeitsbaum bestehen zahlreiche unabhängige Änderungen. Kein Commit, Push oder Redeploy. Seed-Testgate, bestehende GEO-Risiken und nicht isolierte Änderungen blockieren die Veröffentlichung.

## Verknüpfter Check-in

Der zeitnahe read-only Owner-Check-in steht in [`checkin-20261021-0640-owner.md`](checkin-20261021-0640-owner.md).
