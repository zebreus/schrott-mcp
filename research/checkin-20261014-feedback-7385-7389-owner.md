# Owner-Gate: Feedback 7385–7389, 14.10.2026

## Entscheidung

Die fünf Leads sind recherchiert, bleiben aber inhaltlich offen. Übernommen
werden nur die zwei Forschungsberichte und dieses Gate; keine neuen
Dossiers, Preise, Annahmebedingungen oder GEO-Felder.

- [7385–7386 Münster/Bochum](feedback-20261014-nrw-7385-7386.md): Schönfelds
  aktueller vergüteter Ankauf nicht primär belegt. Bei A. al Lahib sind
  Betreiberangebot und Autoankauf-Aliasse kongruent, aber kein unabhängiger
  Identitätsbeleg und keine Klärung der Hordeler-Gegenadresse vorhanden.
- [7387–7389 Borna/Nauen/Giengen](feedback-20261014-7387-7389.md): BeGoSi und
  vanBOXTEL bieten Ankauf an; qualifizierter unabhängiger Zweitbeleg fehlt.
  Hammerschmidt bleibt Verzeichnislead mit Wallerstein-/Geißäckerweg-Konflikt.

## Separate Owner-Quellenprüfung

- https://www.schrott-metall-ankauf.de/kontakt/ direkt gelesen: Dieselstraße
  12, Abholbetrieb, Selbstanlieferung nur nach Absprache; telefonische
  Erreichbarkeit ist keine allgemeine Hoföffnung.
- https://www.11880.com/branchenbuch/bochum/120674719B102336132/schrott-metall-ankauf.html
  direkt gelesen: Hordeler Straße, andere Mobilnummer, gleicher Domainlink.
  Kein belegter Umzug oder zweiter Platz; Verzeichnis bleibt Lead.
- https://begosi.de/impressum direkt gelesen: Philipp Eiker/BeGoSi,
  Ziegeleiweg 5 Borna-Neukirchen, USt-ID und LUCID-Selbstauskunft, keine
  Handelsregisternummer. Kein Zweitbeleg aus mehreren eigenen Unterseiten.
- https://havelland.city-map.de/de/schrotthandel-brandenburg-buntmetallhandel-van-boxtel-nauen-havelland
  direkt gelesen: individualisierte Betreiber-Selbstauskunft mit eigenem
  Inhaberimpressum, Johannes van Boxtel, Hamburger Allee 47, Nauen-Berge.
  Owner akzeptiert dies als gehosteten Eigenauftritt, nicht als unabhängigen
  Registerbeleg und nicht als Zwei-Quellen-Gate. Liste „Gültig ab 03.06.2026“
  separat vorhanden; Materialsorten, Mengenbedingungen und Tagespreiswarnung
  erhalten. Keine bestätigte unveränderte Oktober-Auszahlung abgeleitet.

## Feedback 7390: keine automatische Dublettenbereinigung

Die Meldung behauptet 30 doppelt gemeldete Händler. Read-only Gegenprobe:
1310 Audit-7-Meldungen; 316 unterschiedliche exakt formulierte
Fehlender-Händler-/Ankäufer-Texte, keine identischen Textgruppen. Das widerlegt
keine unterschiedlich formulierten Doppelmeldungen, bestätigt aber auch
nicht die behaupteten 30 Paare. Identitätsabgleich statt Textgleichheit ist
nötig; weder Feedback gelöscht noch Händler zusammengeführt. Lokales
Prüfartefakt: `/tmp/opencode/audit7-feedback-duplicates.json`.

## Abschlussgate und Veröffentlichungsgrenze

- Quellen- und Namensvetter-/Aliasgrenzen beider Berichte geprüft; fehlende
  Evidenz nicht durch `pruefung`-Phantomzeilen umgangen.
- Alle 155 fremden Dateien per SHA-256 unverändert zum Auswahlmanifest.
- Keine bestehenden Dossiers/Historien geändert; keine Enum-/Deep-Link-/
  Emdash-Seedänderung. Nur drei neue eigene Forschungsdateien im Commit.
- `cargo test -p schrott-mcp-ingestion seed`: 14 bestanden, 0 fehlgeschlagen.
  Log: `/tmp/opencode/checkin-20261014-feedback-7385-7389-seed.log`.
- Produktionsdatenbanken nur gelesen. Keine TLS-Ausnahmen, keine Kontakte
  angeschrieben, keine Feedbackquittierung und kein erzwungener Ingest.
- Push/Redeploy bleiben wegen der zuletzt bestätigten fehlenden GitHub-
  Anmeldung offen. Lokaler Bericht ist keine produktive Händleraufnahme.
