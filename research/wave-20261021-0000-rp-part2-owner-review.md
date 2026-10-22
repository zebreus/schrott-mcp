# Owner-Prüfung — Mitternachtswelle RP, Teil 2

Stand: 21.10.2026, nach Abschluss des zweiten 10er-Pakets.

## Umfang und Struktur

- Geprüft wurden ausschließlich die zehn Dossiers des zweiten RP-Pakets. Alle zehn wurden bearbeitet; Frontmatter-Identität, `status`, Websitefelder und Koordinaten blieben unverändert.
- YAML/JSON, Timeline-Reihenfolge und Notenformat sind gültig. Die zehn Recherchebelege und eine zusätzliche Owner-Korrektur haben Datumspräfix und Quellen-URL. `git diff --check` ist sauber; keine neuen Em-Dashes oder Website-Deep-Links.
- Fünf andere bereits geänderte RP-Dossiers blieben außerhalb dieses Pakets unberührt.

## Quellen- und Identitätsprüfung

- **Proba Roman / Weitersburg:** Der direkt gelesene Gelbe-Seiten-Eintrag bestätigt den genannten Containerdienst-Lead, nicht jedoch Schrottannahme oder einen Betreiber-/Registerbezug. Keine Felder übernommen.
- **Levin / Andernach:** Gelbe Seiten führt Krahnenbergweg 2 und Schrott-/Metallschrottleistungen; die ältere Beckstraße-Spur bleibt widersprüchlich. Das ist keine unabhängige Umzugs-/Identitätsbestätigung. Keine Adresse übernommen.
- **Landau Kat-Ankauf / Cadorin:** 11880 zeigt zwei getrennte Profile mit unterschiedlichen Nummern/Adressen und 705 m Abstand. Der Cadorin-Eintrag wurde ausdrücklich nicht dem Kat-Ankauf-Dossier zugeschrieben.
- **Koblenzer Schrotthandel:** Die Betreiberwebsite und ihr Impressum bestätigen Karin Reinhardt, Am Metternicher Bahnhof 18, sowie beworbene Schrottannahme. Eine Zuordnung zum generischen Seed „Koblenzer Entrümpelungen“ ist nicht belegt; keine Felder übertragen.
- **Torsten Schmidt:** Owner-Nachprüfung fand die aktuelle Homepage erreichbar, aber den verlinkten Impressumspfad HTTP 404. Die Übersicht wurde entsprechend korrigiert; Kontakt-/Adress- und Identitätsfelder bleiben leer. Die Websitewerbung zu aktuellen Marktpreisen ist keine Zahlenpreisliste.
- Die übrigen Verzeichnis- und Betreiberhinweise sind im Dossier als Leads/Selbstauskünfte abgegrenzt. Es wurden weder neue Preise noch Schließungen oder kostenlose Abholung aus unzureichenden Belegen abgeleitet.

Neue ungeklärte Adressspuren (Proba, Levin und der nicht zugeordnete Koblenzer Kandidat) sind noch keine freigegebenen Annahmestellen; keine wurde geokodiert. Der getrennte Landauer Cadorin-Lead ist ebenfalls kein GEO-Punkt für das Kat-Ankauf-Dossier.

## Status

Die Recherche-Agenten griffen nicht auf Produktionsdatenbanken zu. Owner-Abfragen nach der Dossierprüfung waren ausschließlich read-only; es gab keine Produktionsdatenbankänderung. Tests, Commits, Pushes und Deployments fanden nicht statt. Teil 1 wurde anschließend separat owner-geprüft; diese Teilprüfung allein war keine Freigabe für einen Seed-Rollout.
