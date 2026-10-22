# Owner-Gate — Recherchewelle 21.10.2026, 06:00 UTC

Abschluss der Owner-Prüfung um 06:26 UTC. Der frühere Zwischenbericht bleibt als zeitgestempelte Zwischenaufnahme erhalten.

## Umfang und Formatprüfung

- Der Scope enthält 60 eindeutige Händler aus dem read-only Pool, je 20 aus MV, BW und TH. Die aktuelle read-only-Abfrage bestätigt, dass alle 60 weiterhin im Pool stehen.
- 57 Scope-Dossiers wurden geändert: MV 20, BW 17, TH 20. Drei BW-Dossiers blieben bewusst unverändert, weil die Quellenlage keinen belastbaren Zusatz rechtfertigt: Rheinstetten/Brandt, Freiburg/Maier und Karlsruhe-Oberreut/Wagner.
- YAML-Frontmatter parsebar; `slug`, `state` und `trader_type` unverändert; Status-Enums gültig; `dropoff_json`/`pickup_json` parsebar. Keine Koordinaten im Dossier ergänzt/geändert, keine Deep-Links in `website`, keine neu eingefügten Em-Dashes.
- Alle 58 neuen Timeline-Belege haben jetzt ein standardisiertes Datumspräfix und eine Quellen-URL. Bei den 20 TH-Dossiers wurden ausschließlich die Präfixe der neuen Notizen vereinheitlicht; Recherchetext und ältere Einträge blieben erhalten. Vorherige datierte Timeline-Einträge sind in ihrer Reihenfolge bewahrt.
- Scoped `git diff --check` ist sauber. Nicht zum Scope gehörende Änderungen und untracked Dossiers im Working Tree blieben unberührt.

## Quellen- und Identitätsentscheidungen

- **Dinkel Recycling GmbH:** Betreiberseiten und Registerprofil stützen Betreiberkette, Adresse Epplestraße 182, Stuttgart-Degerloch, Kontakt und Anlieferbedingungen für Schrott/Metall. Selbstanlieferung ist belegt, aber kein Ankauf gegen Auszahlung oder Ankaufskurs. Der Wechsel von Möhringen auf Degerloch ist im Dossier kenntlich.
- **Auto Schnell OHG:** Betreiberauftritt und Register-/Verbands-Spuren stützen Identität und neue Adresse An der Bundesstraße 2, Bad Kleinen-Niendorf. Die geprüfte Werkstattseite belegt keine aktuelle Altfahrzeugannahme; der externe GESA-Spiegel ist kein direkt gelesener GESA-Nachweis. Deshalb `status: pruefung`, keine erfundenen Preis-/Gebührenangaben.
- **Fallenstein Rolf Schrotthandel:** Owner-Korrektur: Straße, PLZ und Rufnummer waren nur durch Branchenverzeichnisse gestützt und wurden aus dem Frontmatter entfernt. Leads und Begründung bleiben in der Timeline; keine Schließungsbehauptung.
- **Spindler/Albstadt, Schlaich/Balingen, Intlekofer/Offenburg und Reinhardt/Ulm:** Verzeichnisangaben widersprechen einander oder sind nicht durch zulässige Betreiber-/Registerbelege bestätigt. Strukturierte Adresse-/Kontaktwerte wurden entsprechend zurückgenommen; `pruefung` bezeichnet den offenen Identitäts-/Annahmefall, nicht eine Schließung.
- **GEPARD:** Website, Kontaktadresse in Möglingen und Verzeichnislead in Asperg werden nicht zu einer bestätigten Annahmestelle zusammengezogen. Die früher zugeschriebene Kruppstraße 12 ist auf Märker Transportbeton zurückgeführt; keine Adresse oder Koordinate übernommen.
- **TH-Stichprobe:** Wagner, Scholz, G&K und Jäger wurden anhand der jeweiligen Betreiber-/Register-/Zertifikatsquellen geprüft. Unternehmens- bzw. Registeradresse wird nicht automatisch als öffentlicher Annahmeort ausgegeben. Nowatzky und GÖREC bleiben wegen Rechtsträger-/Standortunterschieden ungeklärt.
- Ankauf, Warenverkauf und Entsorgungs-/Containergebühren sind getrennt dokumentiert; nicht veröffentlichte Preise oder Gratisleistungen wurden nicht abgeleitet.

## GEO- und Seed-Rollout-Blocker

Ein read-only-Abgleich fand bei 13 Dossiers bereits gespeicherte GEO-Paare, obwohl Adressen geändert, zurückgenommen oder erstmals konkretisiert wurden:

- BW: `bw-albstadt-spindler-karl-schrotthdl`, `bw-balingen-schlaich-kurt-schrotthdl`, `bw-offenburg-intlekofer-schrott`, `bw-stuttgart-mohringen-dinkel`, `bw-ulm-eselsberg-reinhardt-schrotthandel`.
- MV: `mv-bad-kleinen-23996-auto-schnell`.
- TH: `th-amt-creuzburg-nowatzky-recycling`, `th-friemar-goerec-goelz-recycling`, `th-bad-salzungen-fallenstein-rolf-schrotthandel`, `th-erfurt-wagner-recycling`, `th-groengottern-scholz-recycling-standort-groengottern`, `th-schmalkalden-jager-jurgen`, `th-rudolstadt-g-u-k-recycling-utsch`.

Laut [Seed-Vertrag](../docs/trader-seed.md) erhält ein Seed ohne Dossier-Koordinatenpaar das gespeicherte GEO; Adressänderung oder -rücknahme validiert es nicht. Die betroffenen Punkte müssen vor einem Seed-Rollout einzeln zugeordnet oder anderweitig sicher behandelt werden. Keine Produktionsdatenbank wurde beschrieben, keine Koordinate ergänzt und kein Rust-Geocoding vorgenommen.

## Tests und Veröffentlichung

- Root frei um 06:26 UTC: **1.104.654.336 Bytes (~1.053,5 MiB)**, ca. **146,5 MiB unter** dem dokumentierten Testminimum von 1.200 MiB. `/root/.cargo/bin/cargo` ist vorhanden, liegt aber nicht im PATH; wegen des Speicherdefizits wurden keine Cargo-Tests und kein Build gestartet.
- `main` ist 32 Commits vor `origin/main`; es gibt zahlreiche unabhängige Änderungen im gemeinsamen Working Tree. **Kein Commit, Push oder Redeploy.** Das Seed-Testgate und der ungeklärte GEO-Bestand blockieren die Veröffentlichung.

## Verknüpfter Check-in

Der read-only Owner-Check-in vom selben Zeitpunkt steht in [`checkin-20261021-0626-owner.md`](checkin-20261021-0626-owner.md). Die RP-Hälfte der Mitternachtswelle war noch unberührt, obwohl alle 20 Slugs aktuell im Pool stehen; sie wurde in zwei überschneidungsfreie Recherchepakete zur Nacharbeit gegeben.
