# Owner-Follow-up — Seed-Audit-Reconciliation, 27.10.2026, 01:45 UTC

## Ergebnis der 324-Zeilen-Prüfung

Drei nicht überlappende, bundesland-disjunkte Reviews prüften **324 Händlerzeilen**
und **1.035 strukturelle Feldänderungen** des Seedstands `8157c8c3` gegen
Dossiers und README-Quellenstandards. Aggregat: **535 Felder gestützt**, **492
begründete Rücknahmen**, **8 beanstandet**. Die Reviewer dokumentierten
Spotchecks, Snapshot-Grenzen und nicht erneut abgerufene Quellen; es wird keine
vollständige Live-Wiederholung jeder historischen Quellprüfung behauptet.

## Owner-Disposition und Datenkorrekturen

- **Status auf `pruefung` korrigiert:** Rainer Ludwig (Registerlöschung der KG
  belegt, aber Datensatz-/Standortzuordnung nicht unabhängig), Altfeld, Kühn,
  Bellersheim, Hochscheider und Wagner (aktive Firmen-/Entsorgungsleistung ist
  nicht automatisch belegter aktueller Schrottankauf). `website_status: aktiv`
  bleibt unabhängig davon bestehen, wo die Website erreichbar ist.
- **AK Containerdienst:** `city: Aschaffenburg` bleibt als beworbener
  Container-Serviceort erhalten, nicht als physischer Standort. Die
  strukturierte Beschreibung nennt jetzt ausdrücklich Sitz und
  Metall-Anlieferplatz in Rödermark (Hessen); kein Ersatzpin.
- **Habermann:** Straße und PLZ aus dem Legacy-Dossier entfernt. MHG behauptet
  die Betriebsübernahme und nennt die aktuelle Standortadresse, aber eine
  unabhängige zweite Bestätigung der Betreiberkette fehlt. Status `pruefung`,
  Historie und GEO-Clear bleiben erhalten.
- Detailbegründungen mit Quellen und Abrufdatum wurden in den betroffenen
  Dossier-Timelines ergänzt; frühere Rechercheeinträge blieben erhalten.

## Private Übergangsprobe der korrigierten Seedfassung

- Read-only SQLite-Backup aus Produktion um **01:43:44 UTC**, SHA-256
  `f6801b537fdff9b45cd078138f40733abe7eb5d654c8c2c85189f326cad52671`, Schema
  v6; 3.893 Händler, 83 Materialien, 31.043 Preise, 2.715 aktuelle Zeiger,
  1.402 `trader_materials`.
- Auf einer separaten privaten Kopie liefen `PublicDb::open` (Migration bis
  v9) und `seed_metadata` aus der korrigierten Worktree-Fassung. Resultat:
  3.936 Händler, 83 Materialien, 31.043 Preise, 2.659 aktuelle Zeiger,
  1.402 `trader_materials`; **323** bestehende strukturelle Zeilen mit **1.030**
  Feldänderungen. Zweiter Trader-Seed schrieb **0** Zeilen.
- `PRAGMA integrity_check=ok`; die 24 bereits im Baseline-Snapshot vorhandenen
  FK-Verletzungen blieben 24; keine aktuellen Preiszeiger ohne Preiszeile.
  Private Assertions bestätigten Fritsch Hagen (**51.34597/7.5837325**), Elma
  und RoVa ohne Telefon-/GEO-Werte und RVE ohne GEO/Annahmezusage, jeweils im
  konservativen Prüfstatus. Temporärer Harness wurde entfernt.
- Probe-Artefakte und Feldvergleich: `/tmp/opencode/checkin-20261027-audit-afterfix/`.
  Die Probe lief vor dem Commit auf exakt den hier dokumentierten Dossier-
  Änderungen; sie ist weder Produktionsübernahme noch vollständiges
  Binary-/Pre-Deploy-Gate.

## Aktuelle Produktion und Freigabegrenze

- Read-only Produktionsabgleich um **01:45 UTC**: Fritsch weiterhin
  **51.90933/7.88825**, Elma **50.0167/10.4667** samt altem Telefon, RoVa
  **50.86395/12.08895** samt altem Telefon, RVE **49.2516/6.8587** samt
  `dropoff_json={"allowed":true}`. Kein Produktions-Write und kein Deploy.
- `schrott-mcp.service` aktiv; keine systemd-Warnungen. Lauf **#2865** war
  `ok` (24 Preise, 0 Händlerfehler); der vorherige Lauf **#2864** blieb wegen
  MKM `partial`. Read-only Bestand: 3.893 Händler, 83 Materialien, 31.056
  Preise, 2.715 aktuelle Zeiger. Feedback 7390–7381 bleibt das Neueste.
- Die 324er-Dossierprüfung und private Datenübergangsprobe sind Fortschritt,
  aber keine grüne Pre-Deploy-Freigabe: einzelne Quellen wurden nicht live
  erneut gelesen und der korrigierte Stand ist noch nicht an exakt geprüfte
  Release-Binaries gebunden. Die falschen Produktionsdaten bleiben offene
  Korrekturaufträge bis zum regulären freigegebenen Rollout und read-only
  Nachweis.
