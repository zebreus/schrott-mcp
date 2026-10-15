# Welle 15.10.2026 00:00: gemeinsames Owner-Gate

## Ergebnis und Grenzen

**60von60Dossiers derselben frischen Auswahl recherchiert und bearbeitet.** Dies bedeutet dokumentierten Rechercheabschluss, nicht60abschließend identifizierte aktive Händler. Zahlreiche Fälle bleiben bewusst Prüfung mit konkreten Beleglücken. Kein Ersatz durch neue Auswahl und keine zusätzliche06:00Welle behauptet.

Die ursprünglichen drei Sessions scheiterten am Nutzungslimit;40A/B-Diffs blieben erhalten, C damals leer. Owner rekonstruierte A/B-Berichte anhand vollständiger Diffs und vervollständigte C selbst sowie mit fünf erfolgreichen, disjunkten Folgeaufträgen. Alle60Dossier-Diffs vollständig gelesen. Quellen selbst nachgeprüft, wo neue Stammdaten oder riskante Identitätsfragen entscheidend waren; nicht jeder Agentenabruf als eigener Abruf ausgegeben.

## Prompt-zu-Artefakt-Prüfung

| Anforderung | Konkreter Nachweis |
| --- | --- |
| Frische60aus Produktionspool, Zustandsgruppen disjunkt | Manifest `/tmp/opencode/wave-20261015-0000-selection.json`; Auswahl und ursprüngliche Sessions im [Startbericht](checkin-20261015-0000-wave-start.md); ursprüngliche Auswahl nicht ersetzt. |
| Drei Shards20 | [A-Ownerübersicht](wave-20261015-0000-a-owner.md), [B-Ownerübersicht](wave-20261015-0000-b-owner.md), [C-Verlauf](wave-20261015-0000-c.md). A/B ausdrücklich keine erfundenen Agentenabschlussberichte. |
| Letzte elf C-Dossiers | [Berlin/BB4](wave-20261015-0000-rest-berlin-bb.md), [MV3](wave-20261015-0000-rest-mv.md), [Nordwest4](wave-20261015-0000-rest-nordwest.md); zuvor [Rohrmeyer](wave-20261015-0000-rohrmeyer.md), [P&M](wave-20261015-0000-pm.md) sowie fünf Owner-Recherchen im C-Bericht. |
| Identität, zwei Quellen, Overwrites | Siehe Einzelentscheidungen unten und neue Dossier-Timelines; alle Frontmatterdifferenzen aus HEAD vollständig ausgegeben/geprüft. |
| Historie und Slugs erhalten | Python gegen `git show HEAD:<dossier>`: bei allen60vollständige alte Timeline als zusammenhängender bytegleicher Block enthalten, Slugs identisch, alle60tatsächlich geändert. |
| Enums und Notenformat | Dossier-Codegen und14Seedtests erfolgreich; neu eingefügte Rechercheblöcke vollständig im Diff gelesen, Quellen-/Abrufgrenzen kenntlich, keine unvollständigen Notenfragmente. |
| Deep-Links | Konkrete Betreiber-/Register-/Portal-Unterseiten in Dossiers; Suchlinks ausdrücklich nur Suchspuren. Relative21Links je A/B-Bericht lokal vorhanden. Neue entscheidende Primärquellen durch Owner direkt abgerufen. Kein pauschaler HTTP-Erfolg für blockierte oder ungelesene Quellen behauptet. |
| Emdash/Whitespace | Alle60Added-Line-Diffs ohne neue Emdashs; historische unverändert. `git diff --check` erfolgreich. |
| Fremdarbeit geschützt |155vorhandene geschützte Pfade SHA256-identisch zum Auswahlmanifest. Keine fremden Dateien mitstagen. |
| Seedtests | `/root/.cargo/bin/cargo test -p schrott-mcp-ingestion seed`:14bestanden,0Fehler,647gefiltert nach sämtlichen Dossieränderungen und Zimmermann-Korrektur. |
| Ein gemeinsamer Commit | Exakte60Manifest-Dossiers plus zugehörige Owner-/Rechercheberichte vorgesehen; Index vor Staging leer. Commit-/Pushstatus anhand tatsächlicher Git-Ausgabe, nicht aus diesem Plan ableiten. |
| Push, beide Binaries, Prod-Spots | Noch nicht erreicht. GitHub-Authentifizierung war zuletzt blockiert. Kein Redeploy vor erfolgreichem Push und geprüfter Releasebasis; kein Teilrollout behauptet. |

## Abschließende Stammdatenentscheidungen

- **Scholz Torgau:** Adresse, Telefon, Mail, Zeiten und Betreiber-Standortmarker akzeptiert. JSON-Filialobjekt, Impressum/HRB733963 und kongruentes Registerprofil selbst gegengeprüft; Details im [06:30-Bericht](checkin-20261015-0630-partial-wave-owner.md). Keine Torvermessung oder Altautozulassung abgeleitet.
- **M-Cycle Cuxhaven:** Werbealleinstellung aus Name entfernt, Gruppenmarke statt falscher GmbH-Zuordnung, Baudirektor-Hahn-Straße38/27472aus konkreten Betreiberseiten und Herbert-Machulez-Originalurkunde/Register. Containerdienst-GmbH und Herbert-Machulez-GmbH ausdrücklich getrennt. Keine Koordinaten gesetzt, Geocoding vor Kartenübernahme nötig.
- **Zimmermann Wietze:** Impressum und vollständige unabhängig ausgestellte TÜV-NORD-Urkunde225ZEB018 selbst gelesen; Name/DEA-Str.17/29323kongruent. Urkunde seit11.08.2026abgelaufen, nicht aktuelle Efb-Zertifizierung. Adresse/Website akzeptiert, **Telefon/Mail/Zeiten aus Agenten-Fills zurückgestellt**, da kein entsprechender Zweitbeleg/keine HR-Ausnahme. Einzelangaben mit Freitagskonflikt im Text erhalten. Neues Adress-GEO noch nötig, keine Koordinaten geraten.
- **Milon Lensahn:** Landesportal führt exakten e.K.-Namen und Ottostr.11, NorthData-HRA2053OL/2019Publikation nennt23738Lensahn. Canonical-Name/PLZ akzeptiert; keine Betreiberübernahme, Lübeck-Merge oder heutige Annahme behauptet. Domain503/Wartungsseite ohne Impressum begründet keine Kontakt-/Zeitenfüllung.
- **ELMH:** tot→unbekannt nach tatsächlich erfolgreichem Domain-DNS und differenzierter TLS-/Weiterleitungsgrenze. Keine authentifizierte Betreiberseite oder bestätigte Social-Kontaktanschrift daraus gemacht.
- **S.L.A., Bexbacher, Lenhart sen., Körner, Sönksen und Rohrmeyer:** nur portal-/unklar identitätsgestützten Aktivstatus auf Prüfung korrigiert; **P&M** unbekannt→Prüfung nach unabhängig belegtem Neubauadresskonflikt. Keine geschlossene Firma ohne Beleg. Vorsichtige Beschreibungen nur P&M/Rohrmeyer; keine fremden Betreiber übernommen.
- **Alle übrigen48Dossiers:** keine Frontmatteränderung, aber nachvollziehbare Recherche-/Quellengrenzen ergänzt. Alte Adresse/Telefon dadurch nicht neu verifiziert. Kein Rustcode verändert und keine Handler-/Preisfreigabe aus bloßen Werbeobergrenzen.

## Betrieb und Veröffentlichung

Letzter Check-in08:30UTC: Service active, HTTPS-Health ok,3881Händler/54Materialien/2426Currentpreise, Feedback unverändert7390als neuester. Nur bekannte App-WARNs Hofmann/Münster/AMR. Produktionsdatenbanken strikt read-only; keine Seed-/Ingestion-Erzwingung auf Prod. Dieser Betriebsbefund ist kein Spotcheck einer noch nicht veröffentlichten Welle. Die bereits lokal geprüfte Hofmann-Korrektur wird nicht als produktiv behauptet.

Veröffentlichung, Neubau beider Binaries und produktive Stichproben bleiben offen, bis eine verifizierte Releasebasis erreichbar ist. Offene Gewerbe-/Registerauskünfte sind weiterführende Identitätsaufgaben, keine pauschale Behauptung fehlender Gewerbe. Keine Telefonkontakte oder kostenpflichtigen Auskünfte vorgenommen.
