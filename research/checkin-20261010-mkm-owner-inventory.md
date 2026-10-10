# MKM: erneuter Owner-Coverage-Check

Quelle https://mkm-metals.de/metallpreise erneut direkt abgerufen; realen unveränderten Main-Handler mit `cargo run --locked -q -p schrott-mcp-ingestion --example live_handlers -- rp-ludwigshafen-mkm-metals` ausgeführt. HTTP 200. Keine Produktionswrites, kein Codefix oder Abschluss von Feedback #5557 behauptet.

## Reproduzierbare Evidenz

- Snapshot `/tmp/opencode/mkm-overview-owner-20261010.html`, SHA256 `dd69560a9f34b226995869ca47eca09d7ffb9d3aa00383d12d337a6ea9c7de1d`.
- Liveoutput `/root/Documents/mkm-owner-live-20261010.log`.
- Inventur `/root/Documents/mkm-owner-inventory-20261010.json`, sämtliche 60 Produktzeilen mit Artikel, Deep-Link, Tabellen-Gruppe, Preistext/Mindestmenge, eigenem Produktänderungsdatum und eingebetteten Staffelwerten.
- Neuer read-only Diagnosebefehl: `python3 scripts/audit-mkm-overview.py /tmp/opencode/mkm-overview-owner-20261010.html --handler-output /root/Documents/mkm-owner-live-20261010.log`.
- Tatsächlich **Exit 1**: 8 Ankauftabellen, 60 Produkte, 55 Zahlenpreise, 5 Anfrageprodukte; Handler 11 Zahlenpreise, **44 fehlen**. Kein grüner Vollständigkeitsnachweis.
- Script nimmt tatsächliche Tabellen-Captions, nicht bloße Vorkommen von „Ankaufpreise“ in einer Section. Verkaufs-/Navigationstabellen ausgeschlossen. Es ist eine Diagnose mit bewusst enger HTML-Form, kein produktiver Ingestionparser. Gleiche Zeilenanzahl beweist insbesondere keine richtigen Materialmappings.
- Fünf lokale Script-Regressionen bestanden: Quote/Menge/Datum/Staffeltrennung, Anfrage ohne Nullpreis, Duplikat-/Leer-/Mehrdeutigkeitsfehler, Verkaufsausschluss, CLI rot bei Count-Gap.

## Zusätzlicher umsetzungsrelevanter Befund

Die aktuelle Übersicht enthält bereits produktspezifische `data-staffel`-JSONs. Millberry MET-KU-MILL: ab 0 / 40 / 100 / 250 kg → 10,60 / 10,81 / 10,97 / 11,13 EUR/kg. Tabellen-Höchstpreis 11,13 mit ab 250 kg; das `<time datetime="2026-09-28">` ist ein eigenes Produktänderungsdatum, nicht pauschal der heutige Preisstand. Die Daten wurden gelesen, noch nicht unabhängig gegen alle Detailseiten gegengeprüft oder zur Ingestion freigegeben. Sichtbare Produktzeile, versteckte Formwerte und allgemeine Page-last-change dürfen nicht stillschweigend gleichgesetzt werden.

Vollständiger Fix muss alle Gruppen artikelbewusst mappen, statt generische Kupfer-`schwer`/`leicht`-Arme auf Messing anzuwenden; Anfrageprodukte und echte Kataloglücken ausdrücklich partitionieren. Höchstpreis-/Mengenbedingungen bewahren, keine Fixpreise oder Veröffentlichungsdaten erfinden. Die alte Steckerkabel-Current-Altzuordnung braucht historieerhaltende, quellen-/händlerbegrenzte Reconciliation. Die Diagnose vom 08.10. im Deploy-Worktree bleibt unberührt, ihr 44-Zahlenpreis-Gap ist heute erneut bestätigt.

## Parallelfortsetzung / Check-in

Gescheiterten MSG-Agenten `ses_ed973ec41ffe4nqjV0RpksfHkW` per neuer Nachricht fortgesetzt. Tool nahm den Auftrag an und meldete Hintergrundlauf; **keine fertige Arbeit oder erfolgreiche Ausführung daraus abgeleitet**. Auftrag: separate Worktree-Migration dynamischer Preisvarianten mit unveränderten Legacy-Identitäten und realem TempDB-Test; noch keine Staffelaktivierung ohne Reconciliation. Owner berührt dessen Codebereich nicht parallel.

Service aktiv, keine Warnungen sechs Stunden, Feedback weiterhin maximal 6080. Read-only Produktionszahlen 3881 Händler / 54 Materialien / 2426 Current-Preise. Keine Geocoding-Logs unter `/tmp/opencode` gefunden. GitHub-Auth weiterhin fehlt (`gh auth status`), Veröffentlichung/Redeploy offen. Zu Beginn Main sauber, elf Commits voraus.
