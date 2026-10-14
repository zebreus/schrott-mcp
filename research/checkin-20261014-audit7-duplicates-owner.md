# Owner-Gate: Audit-7-Dublettenhinweis 7390

Stand: 14.10.2026. Reine organisatorische Triageentscheidung, keine
Händlerfreigabe, Feedbackquittierung, Löschung oder Standortzusammenführung.

## Umfang und Korrektur der bisherigen Suchgrenze

Die frühere exakte Textprobe erfasste nur die einfachen Betreffmarker und
fand 316 verschiedene Texte. Sie konnte die als „erneute Meldung“ markierten
Händler nicht ausreichend untersuchen. Erweiterte read-only Auswahl erfasst
1238 Audit-7-Meldungen mit „Fehlender Händler“ oder „Fehlender Ankäufer“
einschließlich Zusätzen vor dem Doppelpunkt. Normalisierter Namensanfang plus
PLZ liefert **40 Kandidatengruppen mit 81 Meldungen** (eine Dreiergruppe).

Diese Heuristik dient nur der Auswahl, nicht als Identitätsnachweis. Sie ist
nicht vollständig: abweichende Namen, Schreibweisen oder PLZ können weitere
Paare verbergen. Der Hinweis „genau 30 doppelte Händler“ wird deshalb weder
als bewiesen übernommen noch durch die 40 Kandidaten widerlegt.
Lokales Auswahlartefakt: `/tmp/opencode/audit7-identity-duplicate-candidates.json`.
Die beiden Berichte sichern sämtliche Gruppen/Original-IDs dauerhaft:

- [A, Index 0–19: 20 Gruppen / 41 Meldungen](feedback-20261014-audit7-duplicates-a.md).
- [B, Index 20–39: 20 Gruppen / 40 Meldungen](feedback-20261014-audit7-duplicates-b.md).

## Owner-Entscheidung

- **26 primärquellenverankerte gemeinsame Triagefälle, 53 Originalmeldungen:**
  Index 0, 1, 3, 7, 12, 14, 15, 16, 19, 20, 22, 23, 25–29 und 31–39.
  Gemeinsame Recherche sinnvoll; zusätzliche Detailangaben und Konflikte
  beider bzw. aller drei Meldungen bleiben erhalten.
- **13 offene Identitätsgruppen, 26 Meldungen:** Index 2, 4–6, 8–11, 13,
  17, 18, 21 und 24. Wiederholung eines Leads ist plausibel, echte heutige
  Betreiber-/Standortidentität nicht ausreichend belegt. Keine endgültige
  Identitätsdeduplizierung.
- **Eine bedingte Gruppe, zwei Meldungen:** Index 30, Hellmich 6606/7376.
  Aktuelle Alversdorfer-Straße-75-Teile gemeinsam bearbeiten; Kreisstraße 22,
  abweichende Rufnummer und „Hellmich Rohstoff“ als offene Kette erhalten.

Kein Bericht gilt als unabhängiger Register-Livestatus oder bestandener
Zwei-Quellen-Händlergate. Bündelung bedeutet keine automatische Erledigung.
Andere Standorte derselben Firma sowie Familien-/Namensvetter bleiben
getrennt. Abweichende Öffnungszeiten, GEO und Altadressen nicht glätten.

## Separate Quellenkontrollen

Owner hat folgende Seiten selbst regulär direkt gelesen:

- https://www.besta-metallhandel.at/kontakt-deutschland/ bestätigt deutsche
  Adresse Hauptstraße 29 und ausdrücklich **keine Anlieferung vor Ort**;
  Österreich hat separat benannte Anlieferbedingungen. Nicht zusammenführen.
- https://www.schrotthandel-schneck.de/impressum/ gehört Kevin Schneck,
  Oberrot/Mangenhofweg 6; kein Betreiberbeleg für Markus in Möckmühl.
- https://www.bissendorf-recycling.de/angebot/ bestätigt ausdrücklich
  Schrottankauf, Abholung und Containerstellung an Auf der Haube 5. Die
  zweite Meldung enthält daher einen relevanten neuen Primärbeleg und ist
  nicht als inhaltslose Wiederholung zu verwerfen.
- https://hellmich-recycling.de/standorte/ bestätigt Alversdorfer Straße 75
  separat von Lehrte und Herzberg; die Altbetreiber-/Kreisstraßenkette wird
  damit nicht bewiesen.

## Abschluss und Grenzen

- Alle 81 Original-IDs gegen Kandidatenmanifest auf Berichtabdeckung geprüft:
  keine fehlende ID; alle 40 Gruppen vertreten. Detailurteile und
  Quellenfamilien-/Standortgrenzen beider Berichte geprüft.
- Alle 155 fremden Dateien per SHA-256 unverändert zum Auswahlmanifest.
- Keine bestehende Dossierhistorie oder Seedfelder geändert. Overwrite-/
  Enum-/Deep-Link-/Emdash-Gate auf drei eigene neue Forschungsdateien begrenzt.
- `cargo test -p schrott-mcp-ingestion seed`: 14 bestanden, 0 fehlgeschlagen;
  Log `/tmp/opencode/checkin-20261014-audit7-duplicates-seed.log`.
- Keine Produktionswrites, TLS-Ausnahmen, Kontaktaufnahme oder Preisimports.
- Nur zwei Shardberichte plus dieser Owner-Bericht im Commit. Veröffentlichung
  und Redeploy bleiben bei fehlender GitHub-Anmeldung offen; reine Triage-
  Dokumentation ist keine produktive Händlerkorrektur.
