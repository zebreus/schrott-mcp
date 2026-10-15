# Owner-Gate: Recherchewelle 15.10.2026, 12:00 UTC

## Auftrag und Scope

Eine neue Welle mit 60 frisch aus dem aktuellen Produktionspool gewählten Slugs, drei bundesland-disjunkte Hintergrundagenten mit je 20 Dossiers. Auswahl, Sessions und Betriebsprüfung im [Startbericht](wave-20261015-1200-owner-start.md). Ergebnisse: [A](wave-20261015-1200-a.md), [B](wave-20261015-1200-b.md), [C](wave-20261015-1200-c.md). Ausgangs-HEAD `99da1af2b4165873b089b4cc960b45f3c5399fdf`; Manifest `/tmp/opencode/wave-20261015-1200-selection.json`.

Alle 60 Dossierdiffs und die drei Berichte ownerseitig gelesen. Kein ungeprüftes Zusammenführen von Namensvettern oder Ersetzen eines historischen Betreibers durch einen heutigen Nachbarn. Portale bleiben Leads, Abrufhürden keine Schließungsnachweise. Gegenwartsbetrieb und Registerexistenz sind getrennte Aussagen. Offene Fragen sind absichtlicher Bestandteil des Ergebnisses, keine Aktivfreigabe durch bloß erhaltene Altwerte.

## Frontmatter-Entscheidungen

Exakt sechs Feldänderungen, keine weiteren Frontmatter-Differenzen:

| Dossier | Änderung | Owner-Entscheidung |
| --- | --- | --- |
| [Schrottagentur Münster](../dossiers/nw/nw-munster-metall-schrottagentur-munster.md) | Leere PLZ zu `48159`, leere Website zu `https://www.metall-schrottagentur.de/`, leerer Webstatus zu `aktiv` | Betreiberimpressum und unabhängiges Northdata-Einzelprofil HRA 8938 mit passender KG, Ort, PLZ und Geschäftsführerbezug direkt gegengeprüft. Webstatus ist Erreichbarkeit, nicht Geschäftstätigkeit. Betrieb bleibt `pruefung`; keine Hausnummer-/Kontaktübernahme bei 143/143c und Footer 2015. Keine autoritative Filialausnahme behauptet. |
| [MSW Freiburg](../dossiers/bw/bw-freiburg-msw-metallhandel-sudwest.md) | `aktiv` zu `pruefung` | Alte Aktivfreigabe ohne aufgelöste MSW/TSR-Kette nicht haltbar. TSR-Standortdetail frisch ownerseitig HTTP 200 gelesen: Engesserstraße 5, 79108, +49 761504410, Mo-Do 07:30-16:00, Fr 07:30-13:00; Breisach eigener Block. Weder Telefonnummer noch heutiger Betreibername ersetzen MSW ohne unabhängige Registerkette. |
| [M.Baum Fischbach](../dossiers/sn/sn-arnsdorf-ot-fischbach-schrotthandel-hausmeisterservice-m-baum.md) | `aktiv` zu `pruefung` | Portal reicht nicht für geprüfte Tätigkeit. Kommunale Seite ownerseitig direkt gelesen: Gewerbegebiet Seeligstädter Straße, ansässiges Gewerbe 2015, keine vollständige Fischbacher Gewerbeauskunft. Keine Schließung, kein Portal-Kontaktfill. |
| [Leipziger Sammeldossier](../dossiers/sn/sn-leipzig-schanko-altmetallhandel-schrotthandel-jo.md) | `aktiv` zu `pruefung` | Mehrere unaufgelöste Gewerbe, OSM plus Portal kein qualifizierter Doppelbeleg. Udo-Schanko-Impressum ownerseitig direkt gelesen: Zum Voßberg 23, 17498 Helmshagen, nicht Leipzig. Keine Übertragung oder automatische Aufteilung. |

Die drei nichtleeren Status-Overwrites sind konservative Prüfmarkierungen gemäß README-Triage, keine Behauptung von Geschäftsaufgaben. Die Quellenlage und Gegenhinweise stehen jeweils neu datiert in den Dossiers. Historische Portalwerte bleiben als Altbestand gekennzeichnet, nicht neu zertifiziert.

## Kritische narrative Funde

- MCA Röbel: Bekanntmachung `https://www.northdata.de/?id=5480743164444672` ownerseitig direkt lesbar: Eintragung 20.08.2025, Beschluss 06.08.2025, Glienholzweg 2, neuer Geschäftsführer Jan Paul Nordried Wilken. Keine automatische Seed-Identität oder Schrottannahme daraus. Historische Liquidation ist keine aktuelle Schließung. Frontmatter unverändert.
- HERO: Agentenbericht dokumentiert öffentlich ausgelieferte, undatierte Zahlen und weiterhin fehlenden Rechtsträger-/Standortbeleg. Nicht als Gardelegen-Preise, heutige Veröffentlichung oder neue Händlerdaten freigegeben. Kein Login oder Schutzumgehung. Nicht sämtliche Agentenabrufe unabhängig wiederholt.
- Eddie 23a/25a, Münster 143/143c, Lenhardt ohne primärbestätigte Hausnummer, Albers/Cohrs und Metalports/BREWELO bleiben ausdrücklich Klärfälle. PMS-Kostenfreiheit nur bedingt; Ankauf, Verkauf und Gebühren nicht vertauscht. Keine neue zugeordnete numerische Preisliste produktiv freigegeben.
- Suche teilweise HTTP 401, Register-/Portalseiten teilweise 403/429, GESA-App ohne auswertbare Betriebsergebnisse. Weder erschöpfende amtliche Negativsuche noch Nichtbestehen behauptet. Neue GEO-Werte und Rust-GEO-Code: keine.

## Tatsächlich ausgeführte Gates

1. Alle 60 Manifest-Ausgangsdateien per SHA256 gegen festgehaltenen HEAD verifiziert; genau 60 unterschiedliche Pfade/Slugs. Jeder komplette alte Timeline-Text weiterhin unverändert und zusammenhängend enthalten, nicht nur einzelne Stichwörter.
2. Alle sechs Frontmatter-Differenzen enumeriert und auf obige Entscheidungen begrenzt. Keine weiteren nichtleeren Felder überschrieben. Formatnachlauf ersetzt ausschließlich neu hinzugefügte Emdashes; historische bleiben erhalten. In hinzugefügten Dossierzeilen keine Emdashes; relative Markdown-Deep-Links auf bestehende Ziele geprüft; `git diff --check` erfolgreich.
3. 154 vorbestehende Fremdpfade vor und nach Recherche/Tests hash-identisch. Index vor dem Staging leer. Wellencommit darf ausschließlich 60 Dossiers, drei Agentenberichte und diese beiden Owner-Berichte enthalten: 65 Pfade. Keine Rustdatei oder fremde offene Recherche darin.
4. `/root/.cargo/bin/cargo test -p schrott-mcp-ingestion seed`: 14 bestanden, 0 Fehler. Validiert u. a. Enum-/URL-/Slugkorpus, Idempotenz und Erhalt von Anreicherungen mit lokalen Testdatenbanken, nicht Produktionswrites.
5. `/root/.cargo/bin/cargo test --workspace`: 724 bestanden, 0 Fehler, 7 ignorierte optionale Live-/Evidence-Tests. Aufteilung 4 Auth, 3 Core, 654 Ingestion, 4 Worker, 19 Server, 40 Store; alle Doctests erfolgreich. Vollständiger tatsächlicher Log: `/root/.local/share/opencode/shell/42aa29071d6c1a29b482e0269f73393202254764/sh_13f7bbf3e001ZKAPOAKGAANp8B.out`. Zusammenfassungen gesondert gelesen. `/root/.cargo/bin/cargo fmt --all -- --check`: Exit 0.
6. Zusätzlich isolierter Zero-Dependency-Codegen: Git-Archiv des Ausgangs-HEAD plus ausschließlich 60 Wellen-Dateien unter `/root/Documents/wave-20261015-1200-isolated-5omi3toe`. Unverändertes `crates/ingestion/build.rs` separat mit `rustc --edition=2021` kompiliert und ausgeführt: 16 Bundesland-JSONs, 3892 Datensätze, alle 60 mit neuer datierter Notiz; Münster-Fills und drei Prüfstatus gezielt aus generiertem JSON bestätigt. Keine Fremddossiers überlagert. Das ist isolierter Codegen, kein behaupteter isolierter Workspace-/Store-Test; Cargo-Tests liefen im gemeinsamen Arbeitsbaum.

## Erneuter Check-in und Veröffentlichungsgrenze

15.10.2026 gegen 12:13 UTC: Service `active`, HTTPS `/health` `ok:true`. Beide Datenbanken ausschließlich `mode=ro` und `PRAGMA query_only=ON` geöffnet: Feedback 7381-7390 unverändert, 3881 Händler, 54 Materialien, 2426 aktuelle Preise. Systemd-Warnprioritäten sechs Stunden leer. App-WARNs unverändert Hofmann 09:39 (alter Produktionsstand ohne lokale Zeilendatenkorrektur), Münster 11:10 (Preisleiter absichtlich blockiert), AMR 11:25 HTTP 502. Keine erzwungene Ingestion oder SQL-Korrektur.

Gemeinsamer lokaler Wellencommit nach Gates. Push anschließend tatsächlich versuchen; bekannte fehlende GitHub-HTTPS-Anmeldung nicht umgehen. Ohne erfolgreichen Push und Release-Build kein Redeploy. Produktions-Spotverifikation der neuen Welle steht dann weiterhin aus; funktionierende alte Healthchecks beweisen keine Veröffentlichung dieser 60 Änderungen. Die offenen RUSS-/Fix-/ASR-Nacharbeiten und übrigen Fremddossiers bleiben getrennt erhalten.
