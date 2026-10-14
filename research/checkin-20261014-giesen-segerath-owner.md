# Owner-Gate: Giesen-Wekos / Segerath, 14.10.2026

## Entscheidungen

- **Giesen-Wekos:** genau ein neuer lokaler Standortdatensatz
  [`nw-neuss-giesen-wekos`](../dossiers/nw/nw-neuss-giesen-wekos.md).
  [Aufnahmeprüfung](feedback-20261014-giesen-wekos-admission.md).
  Betreiber plus unabhängiges Registerprofil kongruent; explizite
  Fortbestandsprüfung und gültiges Betriebszertifikat ergänzen das Gate.
- **Segerath:** [fortgeschrittener offener Aufnahmefall](feedback-20261014-segerath-admission.md),
  kein Dossier. Identität/Ankauf stark gestützt; expliziter aktueller
  Registerstatus und datierte Standortaktualität nicht ausreichend bestätigt.
  Nicht als geschlossen oder Meldung als widerlegt behandeln.

Beide Paare behalten alle Original-Feedback-IDs: 6261/7350 und 6240/7265.
Kein automatisches Erledigen oder Löschen der Meldungen.

## Separate Owner-Quellenprüfung

GLEIF-Original-API selbst regulär gelesen:
https://api.gleif.org/api/v1/lei-records/391200VKPY0ZYQGOBJ19 .
Voller KG-Name, Duisburger Straße 15c/41460 Neuss, `registeredAs` und
`validatedAs` HRA 6020, Entity `ACTIVE`, Registrierung `ISSUED`,
`FULLY_CORROBORATED`, Aktualisierung 11.07.2026, nächste Erneuerung 04.09.2027.
Kein amtlicher Handelsregister-Tagesauszug und kein eigenständiger
Ankaufbeleg daraus behauptet.

Leistungsseite selbst direkt gelesen:
https://www.giesen-wekos.de/produkte-und-dienstleistungen .
Expliziter Schrott-/Metallankauf und eigenes Lager/Aufbereitung, Container
und Abholung für produzierendes Gewerbe. Privatkunden und kostenlose
Abholung werden nicht ergänzt; keine Preise importiert.

ESN-PDF selbst regulär heruntergeladen und Text geprüft:
https://www.giesen-wekos.de/download/esn_zertifikat.pdf .
ESN 98-120179(25), ZZEE003001804009, Prüfung 25.11.2025, Ausstellung
11.12.2025, gültig bis 30.04.2027; alle fünf Anlagen für Duisburger Straße 15c.
Falsches Registerfeld HRB 11146 bleibt dokumentiert: das ist die
Komplementär-GmbH, nicht KG/HRA 6020. Identität deshalb nicht aus diesem Feld
abgeleitet. Zertifizierung ist keine universelle Kundenannahme; publizierte
Ausschlüsse und Emulsions-/Öl-Konflikt bleiben im Dossier.

## Gemeinsames Abschlussgate

- Beide Berichte und neues Dossier vollständig gesichtet; Quellenfamilien,
  Betreiber-/Standortgrenzen, Registerabweichung und Flächenkonflikt erhalten.
- Leistungs-Deep-Link im neuen Websitefeld, keine Domainwurzel. Kein
  Kontakt-Emdash-Fill, keine Default-Privatkunden-/Kostenfreiheitszusage.
- Kein bestehendes Dossier überschrieben oder stabiler Slug geändert;
  Komplementär-GmbH nicht als zusätzlicher Händler angelegt.
- Keine GEO-Werte. Neue Neusser Adresse benötigt noch eigenständige GEO-
  Prüfung; Segerath-Koordinatenkonflikt nicht übernommen.
- Gemeinsamer `cargo test -p schrott-mcp-ingestion seed`: 14 bestanden,
  0 fehlgeschlagen. Log `/tmp/opencode/checkin-20261014-giesen-segerath-seed.log`.
- Alle 155 fremden Datei-Hashes unverändert zum bestehenden Auswahlmanifest.
- Nur ein neues Dossier, zwei Aufnahmeberichte und dieser Owner-Bericht
  gehören zur lokalen Integration; keine Produktionsdatenbank beschrieben.

## Veröffentlichung

Push, Redeploy beider Binaries und Produktions-Spot-Verifikation bleiben
bei fehlender GitHub-Anmeldung offen. Kein erzwungener Ingest und keine
Feedbackquittierung. Lokaler Aufnahmeentscheid ist keine produktive Aufnahme.
Vor Veröffentlichung aus sauberem geprüftem Commit bauen, nicht aus dem
Gesamtbaum mit den 155 fremden Änderungen.
