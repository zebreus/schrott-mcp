# Owner-Gate — Recherchewelle 20.10.2026, 18:01 UTC

## Umfang und Struktur

Die Scope-Datei enthält 60 eindeutige Dossiers: je 20 aus SL, BB und BY. Alle 60 ausgewählten Dateien wurden geändert; es gibt keine Überschneidung zwischen Shards. Der strukturelle Owner-Scan bestätigte, dass `slug`, `state` und `trader_type` unverändert blieben, Statuswerte zum vorhandenen Enum gehören, frühere Timeline-Überschriften erhalten sind und keine Koordinaten neu eingetragen wurden. Zehn Saarland-Notizen wurden nur im Präfix auf das Repoformat `[Recherche 20.10.2026: …]` vereinheitlicht.

`git diff --check` ist sauber. Zusätzlich bestehen weiterhin zahlreiche nicht zum 18:01-Scope gehörende Änderungen und neue Dateien im Working Tree; sie wurden nicht verändert oder in diese Welle eingerechnet.

## Quellen- und Identitätsprüfung

- **ArcelorMittal Eisenhüttenstadt Recycling:** Die aktuellen Betreiber-Standort- und Kontaktseiten bestätigen Recyclinggesellschaft, GmbH-Namen, Adresse, Telefon und E-Mail. Das separate NorthData-Profil bestätigt HRB 3613 FF, Adresse und Geschäftszweck. Ein archivierter Stand des alten eigenen Impressums vom 05.03.2026 wurde gelesen; dessen Live-URL leitet inzwischen auf die allgemeine Standortseite um. Keine dieser Quellen belegt öffentliche Privatkundenannahme; `status: pruefung` und ungeklärtes GEO bleiben.
- **Benzin Paul / Prenzlau:** Schrottplatz-Info listet den Lead mit zwei verschiedenen Telefonnummern. Das aktuelle PSR-Impressum weist dieselbe Hausnummer der Prenzlauer Schrott-Recycling GmbH (HRB 10921) zu; die PSR-Preisseite nennt nur telefonische Tagespreisauskunft. Die Dossierkorrektur entfernt die ungeklärte Prenzlauer Adresse, überträgt weder PSR-Identität noch Preise und behauptet keine Schließung.
- **Krasulsky / Goldkontor:** Das aktuelle Goldkontor-Impressum bestätigt Sandower Straße 52 und HRB 18621. Daraus wird keine Verbindung zu Krasulsky an der abweichenden Adresse abgeleitet.
- **Kroher:** Der Betreiber-Impressumabruf bestätigt Christian Kroher und Rupert-Bodner-Straße 19; ältere Verzeichnisdaten nennen 23. Die Gebühren stehen auf einer mit ©2018 markierten Leistungsseite. Mangels aktueller Betriebs-/Annahmebestätigung wurde `status` von `aktiv` auf `pruefung` gesetzt; Gebühren und Kundenannahme sind vor Nutzung erneut zu prüfen.
- **Florian-Metalle:** Der aktuelle Betreibertext bestätigt individuell angefragte Buntmetallpreise, unbezahlte kostenlose Mitnahme von Eisenschrott und kostenlose Abholung mit mengen-/entfernungsabhängiger Machbarkeit. Eine zuvor zu konkrete „ca. 50 km“-Angabe wurde entfernt; die Website nennt keinen festen Radius.
- Die übrigen geprüften Einträge bewahren offene Namens-, Adress- und Annahmefragen; Aggregator- und Suchindexangaben wurden nicht als Betreiberbelege oder Preislisten ausgegeben. Ankauf, Verkauf und Gebühren sind in den Dossiers getrennt behandelt.

## GEO- und Veröffentlichungsblocker

Ein read-only-Abgleich mit `public.db` fand für alle drei geänderten Adressfälle bereits gespeicherte GEO-Paare:

| Dossier | Aktueller DB-Stand vor Seed | Vorhandenes GEO |
|---|---|---:|
| `bb-eisenhuttenstadt-arcelormittal-eisenhuttenstadt-recycling` | Adresse war leer; Dossier ergänzt Straße 14 Nr. 3, 15890 | 52.1601, 14.555 |
| `bb-templin-benzin-paul-schrotthandel` | Brüssower Allee 90, Prenzlau; Dossier nimmt Straße/PLZ zurück | 53.3131636363636, 13.9321636363636 |
| `by-munchen-autoverwertung-kroher` | Rupert-Bodner-Str. 23; Dossier ändert auf Nr. 19 | 48.143697333333, 11.559808 |

Nach `docs/trader-seed.md` erhalten Seeds ohne Koordinatenpaar das gespeicherte GEO; eine Adressänderung oder -rücknahme validiert oder löscht es nicht. Es gibt keine zulässige GEO-Clear-Option im Seed-Vertrag. Diese drei Bestands-Pins sind deshalb ungeklärt bzw. durch die Adressänderung besonders prüfbedürftig. Keine Produktionsdatenbank wurde geschrieben, keine Koordinate geraten und keine Rust-Geokodierung hinzugefügt. **Kein Seed-Rollout**, bis ein sicherer, owner-geprüfter Umgang mit diesen Pins feststeht.

## Tests und Veröffentlichungsstatus

- Root frei: **1.196.187.648 Bytes (ca. 1.140,8 MiB)**, rund **59,2 MiB unter** dem dokumentierten Minimum von 1.200 MiB. Deshalb wurden keine Cargo-Seed-Tests und kein Build gestartet.
- `git diff --check`: bestanden; strukturelle Auswertung der 60 Scope-Dateien: bestanden.
- Arbeitsbranch `main`, 32 Commits vor `origin/main`; 120 weitere dossierbezogene Dateien außerhalb dieses Scopes sowie untracked Arbeit bleiben unberührt.
- **Nicht committet, nicht gepusht, nicht deployed.** Neben dem fehlenden Test-/Build-Gate blockiert der ungeklärte GEO-Bestand den sicheren Seed-Rollout.

## Check-in-Verweis

Der zugehörige read-only Check-in steht in [`checkin-20261020-1801-owner.md`](checkin-20261020-1801-owner.md): Dienst aktiv, neueste Feedback-ID 7390, Bestand 3.893 Händler / 83 Materialien / 2.683 aktuelle Preise; vier App-WARNs im Sechs-Stunden-Fenster, darunter der fail-closed MKM-Quellfehler in Lauf #2264 (`partial`).
