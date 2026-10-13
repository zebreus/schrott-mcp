# Frische 60-Händler-Welle 13.10.2026 — Owner-Gate

## Auftrag und Auswahl

Status: **A/B/C abgeschlossen; Owner-Gate und lokale Prüfungen bestanden. Ein gezielter lokaler Wave-Commit ist freigegeben; Veröffentlichung offen.**
Die doppelte Wellen-Nachricht wurde als ein Auftrag ausgeführt, nicht als
zwei überlappende Wellen. Der anschließende Stunden-Check ergänzt diese Arbeit.

60 Slugs wurden frisch aus der Produktionsdatenbank ausgewählt:
`website='' OR street=''`, zufällige Reihenfolge je bundesland-disjunkter
Gruppe, jeweils 20 vorhandene Dossiers. Bereits veränderte oder unversionierte
Dateien wurden ausgeschlossen, statt fremde Recherche zu überschreiben.
Verbindung ausdrücklich `mode=ro`, zusätzlich `PRAGMA query_only=ON`.

Exakte Auswahl, Main-Ausgangspunkt `ddc66004`, Auswahlzeit, 60 Ausgangshashes
und Hashes aller 155 vorher offenen Dateien:
`/root/Documents/worktree-archive-20261013/wave-20261013-fresh-selection.json`.
Die Auswahl stammt nicht aus älteren Wave-Berichten.

| Shard | Bundesländer | Session | Ergebnisbericht |
|---|---|---|---|
| A | NW, NI, SH, HH, HB | `ses_eca1a3297ffeiud02Tv6Cs9jFm` | `research/wave-20261013-fresh-a.md` |
| B | BB, BE, MV, SN, ST, TH | `ses_eca1a3291ffeXbG28kTbTdcMjg` | `research/wave-20261013-fresh-b.md` |
| C | BW, BY, HE, RP, SL | `ses_eca1a31e8ffenpy620UTeHzaFY` | `research/wave-20261013-fresh-c.md` |

Jeder Agent darf ausschließlich seine 20 Dossiers und seinen Ergebnisbericht
bearbeiten. Rechercheweg frei, README-Quellenhierarchie und erhaltene Historie
verbindlich; Ankauf, Verkauf und Gebühren getrennt. Kein eigener Commit,
Branch/Worktree, Deployment, Produktionswrite oder Rust-GEO.

Die erste Runde mit anderen Sessions endete jeweils mit „usage limit has been
reached“ und lieferte keine der drei `fresh-*`-Berichtsdateien. Danach wurden
die drei Shards neu gestartet und abgeschlossen. Owner-Abgleich mit der
Produktionsdatenbank und dem Manifest: **60/60 eindeutige Slugs** erfüllen
weiterhin `website='' OR street=''`; A änderte 20/20 Dossiers, B 20/20 und C
14/20. C hat 10 neue Timeline-Notizen, vier reine Überblickskurationen und
sechs unveränderte Dateien. Ein Tabellenfehler im C-Bericht zu Bayreuth und
Wiesbaden wurde korrigiert. Die älteren `research/wave-20261013-{a,b,c}.md`
gehören nicht zu dieser frischen Auswahl.

## Owner-Gate und Validierung

- Die tatsächlichen Deltas stimmen mit den drei Sluglisten überein: **54/60**
  Dossiers geändert; sechs C-Dossiers unverändert. Die 155 geschützten
  Arbeitsdateien stimmen weiterhin **155/155** mit ihren Ausgangs-SHA256s
  überein; außerhalb von Auswahl, diesen drei Ergebnisberichten und diesem
  Owner-Bericht sind keine neuen Änderungen aufgetaucht.
- Historienprüfung: alle **60/60** bisherigen Timeline-Blöcke byte-identisch
  erhalten, nachdem die neuen Abschnitte vom 13.10.2026 ausgeklammert wurden.
  Keine Slug- oder Koordinatenänderung. Arends zuvor gemeldete PLZ/Telefon/
  E-Mail wurden nach der Gegenprüfung nicht strukturiert übernommen, weil die
  öffentliche Profilansicht diese Werte nicht zeigte und Creditreform nicht
  auswertbar war. Der C-Bericht behauptet keine Frontmatter-Änderungen.
- Quellen, Identitäten, Namensvetter und Preisrichtung wurden anhand der drei
  Berichte und geänderten Dossiers geprüft. Owner hat Schlüsselseiten von
  Graf, Fernau, Trabandt, Port, Fischl, Klein, Südpfalz Metall, Bertin und
  Sontra erneut direkt gelesen. Nicht auflösbare Identitäten bleiben offen;
  keine Händlerpreise wurden aus unpassenden oder unbezifferten Angaben
  importiert. Drei Notizen ohne qualifizierten Neufund nennen transparent den
  alten Lead und die erfolglose gezielte Suche.
- Validierung: `/root/.cargo/bin/cargo fmt --all -- --check` sauber;
  `/root/.cargo/bin/cargo test -p schrott-mcp-ingestion seed` **14 bestanden**;
  `/root/.cargo/bin/cargo test --workspace` **706 bestanden, 7 ignoriert,
  0 fehlgeschlagen**. Seed-Codegen und Tests validieren Dossierformat/Enums;
  Whitespace- und Historienprüfungen wurden zusätzlich separat ausgeführt.
- ELNO, AGH, Saxonia, Hofmann Rastatt, reGOLD und Trapper sind im Handler-Modul
  registriert. `live_handlers` lieferte jeweils HTTP 200 und `alive=true`,
  ohne Datenbankzugriff. Hofmanns Quellstand 30.09.2026 und Trappers 04.08.2026
  sind älter und werden nicht als tagesaktuelle Kurse dargestellt. Im
  Handler-Verzeichnis gab es keine TODO/FIXME/unimplemented-Treffer; kein
  aktiver Geocoding-Log in `/tmp/opencode`. Keine Rust- oder GEO-Datei geändert.
- Der geprüfte Commit-Scope umfasst genau 58 Dateien: 54 geänderte ausgewählte
  Dossiers und die drei Shard-Berichte samt Owner-Bericht. Unveränderte
  Dossiers, die 155 geschützten Dateien und sonstige parallele Recherche
  bleiben ausgeschlossen. Vorbestehende Main-Commits bleiben unangetastet.
- `gh auth status` meldet keine GitHub-Anmeldung; `GIT_TERMINAL_PROMPT=0 git
  push --dry-run origin main` scheiterte mit „could not read Username“. Push
  und Deployment bleiben offen. Produktionsdatenbank blieb strikt read-only;
  kein Rollout oder Seed-Schreibvorgang wurde ausgeführt.

## Aktueller Check-in und Publikationsgrenze

Service aktiv, keine systemd-Priority-Warnungen im Sechsstundenfenster.
App-WARNs: Münster verdächtige Preisleiter bewusst zurückgewiesen,
AMR HTTP 502. Produktion read-only: 3881 Händler, 54 Materialien,
2426 Current-Preise. Feedback IDs 7381–7390 gehören zu Audit 7; wegen der
parallelen Feedback-Triage wurden sie nicht bearbeitet.

`gh auth status` war nicht angemeldet. Vor dem Wave-Commit war lokaler Main
`ddc66004`, zuletzt direkt bestätigter Remote-Main `a024a7b6` (lokal 2 Commits
voraus). Vor einer Publikation sind Push-Berechtigung, Build-/Rollout- und
Produktions-Spotchecks erneut zu prüfen.

Aktueller Branch-Check: lokal gibt es nur `main` und nur den Main-Worktree.
GitHub führt zusätzlich `copilot/security-audit` und
`owner/dossier-quality-2026-10-07`; beide Branch-Spitzen sind bereits Vorfahren
des Remote-`main` (jeweils 0 Commits exklusiv auf dem Nebenbranch). Remote-
Löschung ist mit dem derzeit fehlenden GitHub-Login nicht möglich; nichts
davon wurde verändert.
