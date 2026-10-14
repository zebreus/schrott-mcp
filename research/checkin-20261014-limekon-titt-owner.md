# Owner-Gate: Limekon / Fritz Titt, 14.10.2026

## Entscheidungen

- **Limekon:** genau ein neues lokales Standortdossier
  [`nw-doerentrup-limekon`](../dossiers/nw/nw-doerentrup-limekon.md).
  [Aufnahme- und Preisquellenprüfung](feedback-20261014-limekon-admission.md).
- **Fritz Titt:** [offener Aufnahmefall mit geprüfter Preisquelle](feedback-20261014-titt-admission.md),
  kein neues Dossier/Handler. Aktueller Betreiberankauf ist stark gestützt;
  unabhängiger zulässiger Firmenbeleg und Adresskette fehlen weiter.

## Separate Owner-Prüfung und Korrekturen

Limekon-GLEIF-Original-API selbst regulär gelesen: voller Name, Bahnhofstraße
23/32694 Dörentrup, HRB 7953, ACTIVE, ISSUED, FULLY_CORROBORATED, Aktualisierung
19.08.2026. Zusatz zur unabhängigen NorthData-Identitätsquelle, kein amtlicher
heutiger Handelsregisterauszug. Betreiber-Ankaufseite selbst gelesen:
persönlicher Ankauf, RFA-Sortierung und umgehende Auszahlung am Adressstandort;
Paket- und Abholmengen nicht zu universellen Hoflimits gemacht.

Dedizierte Limekon-Preisliste selbst gelesen: Stand 14.10.2026, ausdrücklich
Richtpreise. Silber fein 1000 EUR/kg ist nicht 1000 EUR/g; versilberte
Objekte/Feinbasis und gemeinsame Blei/Zink-Zeile bleiben getrennte
Interpretationsgrenzen. Keine Preise übernommen und noch kein Rust-Handler.
Owner präzisiert Beschreibung auf **Zinnlegierungen** statt unbewiesener
Lötverwendung. Versandgebühren-/Abholschwellenkonflikte bleiben sichtbar.
Zertifikatslink nicht verifiziert, deshalb keine Zertifizierung ergänzt.

Bei beiden Recherchen fehlte ein eigener aktueller Produktionsabgleich.
Owner hat ihn separat read-only nachgeholt: jeweils sämtliche 3881
Händlerfelder normalisiert nach Identitäts-/Kontakt-/Adressankern geprüft,
**0 Kandidaten**. Titt zusätzlich Freudenberg/57258, Altfestnetz und
Siegener Straße 542/554. Kein absoluter Ausschluss anonymer Identitäten.
Die leere lokale `internal.db` ist keine Produktionsquelle; deren Nichtfund
wird nicht als Händlerbestandsbeleg verwendet. Artefakte:
`/tmp/opencode/limekon-owner-production-check-20261014.json` und
`/tmp/opencode/titt-owner-production-check-20261014.json`.

## Abschlussgate

- Neues Dossier und beide Berichte vollständig gesichtet; Quellenfamilien,
  Aliase, historischen Rechtsereignisse und Adresskonflikte nicht geglättet.
- Konkreter Ankauf-Deep-Link; kein Em-dash-Kontaktfill, keine erfundenen
  Kundentypen oder kostenlose Abholung. Keine GEO-/Materialpreisänderung.
- Beide Originalpaare 6549/7008 und 6276/7373 erhalten; keine Quittierung.
- Alle 155 fremden Datei-Hashes unverändert zum bestehenden Auswahlmanifest.
- Gemeinsamer `cargo test -p schrott-mcp-ingestion seed` nach Owner-Korrektur:
  14 bestanden, 0 fehlgeschlagen. Log
  `/tmp/opencode/checkin-20261014-limekon-titt-seed.log`.
- Nur ein neues Dossier, zwei Rechercheberichte und dieser Gatebericht im
  Commit. Kein bestehendes Dossier überschrieben oder stabiler Slug verändert.

## Veröffentlichung

Lokale Main-Integration, keine produktive Aufnahme. Push/Redeploy/produktive
Spot-Verifikation bleiben bei fehlender GitHub-Anmeldung offen. Keine
Produktionswrites oder erzwungene Ingestion. Release später aus sauberem
geprüftem Commit statt schmutzigem Gesamtbaum bauen. Limekon-Adresse benötigt
noch unabhängige GEO-Prüfung; Rust geocodiert nicht.
