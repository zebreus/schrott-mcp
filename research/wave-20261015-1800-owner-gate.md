# Welle18: Owner-Gate 15.10.2026, 18:00 UTC

## Gemeinsame lokale Integrationsfreigabe nach allen drei Shards

Start-/Auswahlbeleg: [Owner-Start](wave-20261015-1800-owner-start.md), Manifest `/tmp/opencode/wave-20261015-1800-selection.json`, Ausgangs-HEAD `1f91f55c`. Drei disjunkte Shards, 60 frisch aus dem aktuellen read-only Produktionspool ausgewählte und zuvor saubere Dossiers. 149 fremde offene Pfade geschützt. Ein gemeinsamer Commit erst nach allen Ergebnissen und technischen Gates.

## A und B tatsächlich geprüft

Beide Agenten erfolgreich abgeschlossen; [Bericht A](wave-20261015-1800-a.md) und [Bericht B](wave-20261015-1800-b.md) vollständig gelesen. Alle 40 vollständigen Dossierdiffs gegen Ausgangs-HEAD gelesen, alte nichtleere Timelinezeilen enthalten, 20/20 Frontmatter B unverändert. Shard A genau ein Dossier mit fünf Feldänderungen, unten einzeln bewertet. Zu diesem Prüfzeitpunkt alle 149 Fremdhashes unverändert. Keine Ergebnisse von C oder finale Tests vorweggenommen.

### L'honneux: begrenzte konkrete Stammdatenfreigabe

Owner liest selbst direkt das Betreiber-[Impressum](https://www.container-kerpen.de/impressum/), die [Metallseite](https://www.container-kerpen.de/metall/) und das vollständige frei zugängliche [NorthData-Profil HRB 120852](https://www.northdata.de/L'honneux%20Ressourcen%20%26%20Logistik%20GmbH,%20Kerpen/Amtsgericht%20K%C3%B6ln%20HRB%20120852). Registerauswertung nennt Karl-Ferdinand-Braun-Str. 12-14/50170 Kerpen, aktuelle GmbH und Geschäftsführer Daniel Kannenberg/Hansjörg L'Honneux, Eintragung 18.10.2024. Kein eigener amtlicher Originalauszug behauptet. Betreiber nennt gleiche Firma/Geschäftsführer, Kontaktnummer 12 und Footergrundstück 12-14. Owner extrahiert außerdem das vom Agenten heute heruntergeladene PDF `/tmp/opencode/w18a-lhonneux.pdf` erneut vollständig mit `pdftotext -layout`: ZER-QMS 2212/Z3580/Efb, ZZET017005051001, HRB 120852, Standort 12-14, Prüfdatum 19.03.2026, Ausstellung 21.04.2026, gültig bis 18.09.2027 mit drei Standortanlagen. Das ist eine aktuelle Betreiber-gehostete Urkunde des Zertifizierers, keine zusätzlich abgefragte unabhängige Zertifikatsdatenbank. Keine Checkboxdetails oder pauschale Abfall-/Annahmefreigabe allein aus Textausgabe abgeleitet.

Strukturierte Änderungen ausschließlich:

- `name`: Lhonneux Containerdienste → L'honneux Ressourcen & Logistik GmbH (Containerdienste).
- `street`: leer → Karl-Ferdinand-Braun-Straße 12-14.
- `postcode`: leer → 50170.
- `website`: leer → https://www.container-kerpen.de/.
- `website_status`: leer → aktiv.

Begründung: knapper Seed-/Markenname, Ort und historischer Import mit Karl-Ferdinand-Braun-Str. 12 kongruent zum heutigen Betreiberauftritt; unabhängiges Registerprofil und aktuelle Standorturkunde bestätigen die GmbH am Grundstück. Keine aus Namensähnlichkeit behauptete Rechtsnachfolge zum früheren Einzelunternehmer und keine erfundene Umwandlungskette. Korpussuche nach Lhonneux/Domain ergab kein weiteres Geschwisterdossier; kein automatischer Merge. Alter Titel/Importname/Timeline und Slug stabil. Die Grenze Hausnummer 12/Grundstück 12-14 steht sichtbar im Dossier; kein Nachweis einer vermessenen Hofzufahrt.

Impressum ohne HRB: deshalb **keine pauschale README-Autoritätsausnahme für alle Betreiberfakten**. Kontakte, Zeiten, Service-JSON und Betriebsstatus nicht gefüllt/aufgewertet, `pruefung` bleibt. Metallseite bestätigt Auszahlungsschwellen 100 kg Schrott/10 kg sortenreine Metalle, darunter kostenneutral ohne Vergütung; keine Annahmemindestmenge, kein Kurs. Samstags keine NE-Annahme über Kleinmengenwaage, keine allgemeine Samstags-Schließung. Kein kostenloser Transport oder Material-Verkaufskurs daraus ableitbar.

Read-only Produktionszeile: bisher alter Name, Adresse/PLZ/Website leer, Website-Status unbekannt, pruefung. Vorhandenes GEO 50.883433333333336/6.706766666666667 wird durch diese Adresskorrektur **nicht verifiziert oder verändert**; keine Dossierkoordinaten ergänzt, bestehende DB-Paare bleiben laut Seedsemantik erhalten. Keine Produktionswrites.

### B: wertvolle Gegenbelege, keine Feldübernahme

Owner liest [Herforder Betreiber-Impressum](https://www.autoverschrottung-herford.de/impressum) selbst: A. Lahib, Hansastraße 14/32049 Herford, 0152 04045656, ausdrücklich zentraler mobiler Dienst ohne feste Niederlassungen in anderen Städten. Passt zur Frankfurter Seednummer, beweist aber keine vollständige Identität der Cassellastraßen-Zeile oder dortige aktuelle Nutzung. Keine Frankfurter Annahmestelle/Preisquelle freigeben, keine Umbenennung oder Umzug behaupten; zweiseitige Adressprüfung als Folgeschritt offen.

Owner liest [FAIR-METALL-Preisrubrik](https://www.fair-metall.de/schrottpreise/) selbst: Tagespreise telefonisch ab 8 Uhr; auf derselben Seite Sidebar Mo-Fr 7-17 und Footer 7-16, Sa jeweils 9-13. Kein numerisches Ankaufblatt, keine strukturierte Zeitenfüllung aus widersprüchlichem Text. Creditreform-403/Suchauszüge in B bleiben klar als unvollständige Quellenkandidaten, keine zwei Vollbelege behauptet. Keine numerischen Herforder Werbepreise als Frankfurter Materialkurse übernehmen; GESA-/Register-Negativbeweise nicht aus gesperrten oder ungelesehenen Seiten konstruieren.

## C und abschließende Owner-Entscheidungen

Auch C erfolgreich abgeschlossen; [Bericht C](wave-20261015-1800-c.md) vollständig gelesen und sämtliche 20 Dossierdiffs gegen Ausgangs-HEAD geprüft, einschließlich zunächst in der Shellanzeige abgeschnittener Jesewski-/Dreßler-Abschnitte. Im Agentenergebnis alle Frontmatter unverändert, keine stillen Betreiberwechsel.

- Goldankauf Mülheim-Kärlich: Betreiber-/Registerkandidat mit vier visuell vom Agenten gelesenen undatierte-Beispiel-Gesamtauszahlungen. Kein Gewicht/Feingehalt/Gültigkeitsdatum, daher keine EUR/g-Umrechnung, aktuelle Materialkurse oder garantierte Auszahlung. Bilder wurden durch den Agenten gelesen, keine zusätzliche visuelle Owner-Prüfung behauptet. Der generische adresslose Seed ist nicht automatisch diese GmbH; **keine weiteren Felder übernommen**, trotz positiv recherchierter Betreiberidentität.
- Jörg Schmitz: GmbH-Sitzkette als solche, nicht unbelegte Einzelbetrieb-Übernahme/Stilllegung; Premium-gesperrter Beteiligungshinweis kein bewiesener Merge. Kein Fill.
- Sterz: gemischte Portal-Orts-/Branchen-/Zeitenzuordnung konkret benannt, kommunale Breitscheid/Hessen-Fakten nicht zum Sterz-Betrieb erklärt. Kein Umzug/Schließungsurteil.
- S. Alten: Owner liest zusätzlich das fremde [Angelo-Alten-Impressum](https://entruempelungenkoblenz.de/impressum) selbst, Naumburgerstr. 14/56075 Koblenz, 0261 3947719. Belegt Website-Inhaber, nicht die S.-Alten-/St.-Maternus-Identität. Keine Übernahme der fremden Kontakte oder Preise; Standort-/Betreiberkettenklärung offen.
- **Weerts/Peters: zusätzlicher Owner-Statusgate.** Kombinierter Name/Norden hat seit 01.10. keine zugeordnete Betreiber-, Register- oder Gewerbeevidenz; der historische `aktiv`-Wert ist kein belastbarer heutiger Annahmebeleg. Owner liest Peters-Gelbe-Seiten-Detail selbst: Kienholzstr. 49, Rechtsupweg, anderer Ort, Portal mit angebotener Profilübernahme. Keine Bestätigung einer Weerts-Verbindung und kein individueller Betreiberbeleg. Daher `aktiv` → `pruefung`, nicht `geschlossen`. Datierte Owner-Korrektur und aktueller Überblick erläutern die Entscheidung; sämtliche vorherigen Notizen einschließlich der ursprünglichen Bitte um Owner-Entscheidung erhalten. Keine Adresse oder Nummer aus Rechtsupweg übertragen und keine Nichtexistenz aus Suchfehlen behauptet.

## Finale technische Gates tatsächlich erfolgreich

Alle 60 Dossiers mit genau einer jeweiligen Welle18A/B/C-Überschrift. Slugs stabil, Frontmatter als YAML geprüft; **genau zwei Dossiers mit strukturierten Änderungen**: L'honneux fünf oben einzeln freigegebene Felder und Weerts/Peters ausschließlich Status. Alle anderen 58 Frontmatter unverändert. Alte nichtleere Timelinezeilen aller 60 Dateien vollständig und in ursprünglicher Reihenfolge erhalten. Neue Timeline-Bullets im Dossierformat, Quellenarten und Abrufgrenzen sichtbar, keine hinzugefügten Dossier-Emdashes. Relative Markdownlinks und `git diff --check` sauber. Keine Preisbeobachtungen, Koordinaten oder Rustdateien geändert.

Alle **149** im Startmanifest geschützten offenen Pfade SHA256-identisch; keine fremde Arbeit integriert. Index vor scoped Staging leer. Nur die 60 ausgewählten Dossiers und fünf Berichte (drei Shards plus Owner-Start/-Gate) für einen gemeinsamen Commit freigegeben.

Isolierter Endstand tatsächlich aus Ausgangs-HEAD-Archiv plus ausschließlich 60 finalen Dossiers generiert: `/root/Documents/wave-20261015-1800-isolated-yz9mzfmx`, Ergebnis `owner-gate-result.json`. Vorher/nachher **3892 Zeilen in 16 JSON-Dateien**. Vollständiger Payloadvergleich: exakt die 60 ausgewählten Slugs verändert; 58 ausschließlich `notes`, L'honneux `notes` und die fünf freigegebenen Felder, Weerts/Peters `notes` und `status`. Keine weiteren Zeilen- oder Feldänderungen. Diese isolierte Seedgenerierung ist kein isolierter Rust-Workspace-Test.

Final tatsächlich `/root/.cargo/bin/cargo test -p schrott-mcp-ingestion seed`: **14 bestanden, 0 Fehler**. Deckt Enum-/URL-/Slugkorpus, lokale Store-/Idempotenz-/Clear-/GEO-Erhaltungssemantik ab, beweist keine reale Firmenidentität. Tests auf gemeinsamem Arbeitsbaum mit fremden offenen Dossiers; deren Unverändertheit separat gehasht. Kein Rustcode geändert und keine neue volle Workspace-/Release-Suite für diese Welle behauptet; die 726 bestandenen Workspace-Tests aus dem 17:30-Check-in werden nicht als nach dieser Welle wiederholt ausgegeben.

## Veröffentlichung getrennt vom lokalen Erfolg

Ein gemeinsamer scoped Commit und tatsächlicher Push nach obigem Gate. Bekannten fehlenden GitHub-HTTPS-Login nicht umgehen. **Ohne erfolgreiche Veröffentlichung und geprüften Release-Build kein Redeploy beider Binaries und kein produktiver Seed-/Spotnachweis dieser Welle.** Der 18:01-Healthcheck belegt nur den weiterhin laufenden älteren Stand. Produktionsdatenbanken während der gesamten Welle read-only; keine erzwungene Ingestion und kein Rust-Geocoding. Die fachlichen Klärfälle sind transparent dokumentiert, nicht pauschal „60 Händler verifiziert“.
