# Gemeinsames Owner-Gate: Recherchewelle 26.10.2026

## Auswahl und Umfang

Die 60 Dossiers wurden frisch und read-only aus dem Produktionspool mit der
Abfrage in [`wave-20261026-owner-scope.md`](wave-20261026-owner-scope.md)
ausgewählt. Der Pool enthielt beim Gegencheck 1.933 Händler mit leerer Website
oder Straße. Die drei getrennten Shards HE, NW und RP bearbeiteten je 20
Dossiers. Ergebnisse: [HE](wave-20261026-he-20-dossiers.md),
[NW](wave-20261026-nw-20-dossiers.md) und
[RP](wave-20261026-rp-20-dossiers.md).

48 Dossiers erhielten neue Rechercheeinträge oder kuratierte Übersichten;
12 blieben nach der Gegenprüfung ohne redundante Änderung. Es gab keine
Produktionsdatenbank-Schreibzugriffe.

## Owner-Review

- **Identität und Quellen:** Der Owner hat die geänderten Frontmatter-Felder
  gesondert geprüft. Änderungen an Stammdaten beschränken sich auf DMR
  Rohstoffhandel und Ayan Börrstadt; die übrigen geänderten Dossiers bewahren
  ihre Stammdaten und kennzeichnen neue Treffer als Leads, wenn die
  Seedzuordnung nicht unabhängig bestätigt ist.
- **DMR:** Impressum und NorthData stimmen zu HRB 33481 und
  Eselsfürther Bahnhof 4b, 67657 Kaiserslautern überein; der Betreiberauftritt
  beschreibt Schrott-/NE-Metallhandel. Name, Kategorie, Ort, Straße, PLZ,
  Website und Beschreibung wurden präzisiert. Der Unternehmenssitz ist kein
  bestätigter öffentlicher Annahmehof; `status: pruefung`, keine Kontakte,
  keine Annahmedaten und kein GEO.
- **Ayan:** Betreiberauftritt und zwei aktuelle, visuell geprüfte SMU-Zertifikate
  verbinden Resul Ayan, Schrott-/Autorecycling und Kaiserstraße 5,
  67725 Börrstadt. Die Materialankaufseite lädt ausdrücklich zum Besuch ein und
  nennt die Annahme kleiner und großer Mengen; `dropoff_json.allowed` ist
  eingetragen. Die Zertifikate sind Betreiber-Uploads und nicht separat beim
  Ausstellerregister verifiziert. Kontaktdaten/Zeiten bleiben leer; keine
  Koordinaten oder Produktionsänderung.
- **Historie/Overwrite:** In allen 48 geänderten Dossiers ist die bestehende
  Timeline wortgetreu erhalten. Die Frontmatter-Diffprüfung fand Änderungen
  ausschließlich in den beiden oben begründeten Dossiers. Keine GEO-Felder,
  `clear_fields` oder Slugs wurden geändert. Der frühere falsche Produktions-
  GEO-Punkt für Fritsch Hagen wurde ausdrücklich nicht verändert.
- **Preisdisziplin:** Die drei Shard-Berichte trennen Ankauf, Verkauf und
  Gebühren. Nicht numerische Tagespreiswerbung wurde nicht als Preisliste
  ausgegeben. Fremdpreise und Leistungen von Namensvettern, Kandidaten oder
  Nachbarbetrieben wurden nicht auf Seeds übertragen.

## Checks

- `git diff --check`: bestanden.
- Neue Dossier-/Berichtstexte: kein hinzugefügter Em-Dash-Platzhalter.
- Timeline-Erhalt: 48 von 48 geänderten Dossiers bestanden.
- Lokale Markdown-Links: **54 gestagte Markdown-Dateien geprüft, 0 defekte
  lokale Links**.
- `cargo fmt --all -- --check`: bestanden.
- `cargo test -p schrott-mcp-ingestion seed --quiet`: **14 bestanden, 0
  fehlgeschlagen**.
- Es wurden keine Quellcodeänderungen, Seed-Anwendungen, Produktionsschreib-
  zugriffe oder Deployments vorgenommen.

## Check-in und Produktionsgrenze (26.10.2026, 10:24 UTC)

- `schrott-mcp.service` ist aktiv; `journalctl -p warning` enthält im
  Sechs-Stunden-Fenster keine System-Warnungen. Das App-Journal zeigt sechs
  `WARN`s: Wahl & Co. fehlende Preisliste, A-Zs verdächtige Preisleiter, AMR
  HTTP 502, Heraeus alte Ankauf-URL, MKM Source-Contract und
  WirKaufenDeinGold nicht validierbare Tageskursreferenzen. Letzter App-WARN:
  09:05 UTC.
- Scheduler-Lauf **#2804** (10:20 UTC) war `ok`: 73 Preise, 0 Händlerfehler.
  Read-only Produktionszahlen: **3.893 Händler, 83 Materialien, 2.683 aktuelle
  Preise**. Die letzten zehn Feedbacks sind weiterhin IDs 7390–7381 vom
  13.10.2026.
- **Fritsch Hagen:** der Produktionsdatensatz `nw-hagen-fritsch` hat bei der
  erneuten Read-only-Prüfung weiter leere Straße/PLZ, Status `pruefung` und den
  falschen Punkt **51.9093300/7.8882500**. Die Dossier-/Seed-Korrektur mit
  Oeger Straße 63 und **51.3459700/7.5837325** bleibt die belegte Fassung;
  Produktion wurde nicht direkt geändert.
- Die Pre-Deploy-Freigabe bleibt offen: 294 strukturell geänderte Zeilen sind
  noch nicht vollständig dispositioniert; Source-/Seed-/Release-Binary-Bindung
  sowie der bestehende Harbi-Logbefund sind nicht abgeschlossen. Daher weder
  Deployment noch Neustart, Force-Ingest oder direkte Produktionsänderung.

## Veröffentlichung

Der Branch `main` stand zu Beginn auf `292bcf94`, gleichauf mit `origin/main`.
GitHub-Schreibzugriff wurde mit dem im Check-in-Prompt dokumentierten Dry-Run
getestet. Commit `fe03b706` mit den 48 Dossiers und zugehörigen Scope-, Shard-,
Zwischen- und Owner-Berichten wurde erfolgreich nach `origin/main` gepusht.
Der Push wurde verifiziert; `HEAD` und `origin/main` stimmen überein und der
Worktree ist sauber. Kein Deployment.
