# Owner-Check-in: Feedback 7381–7384, 14.10.2026

## Ergebnisprüfung

Vier ältere offene Leads in zwei überschneidungsfreien Recherchen geprüft:

- [Voth/Schwartze, 7381–7383](feedback-20261014-voth-owner-leads.md):
  Gesellschafts-/Standortkette und historische unabhängige TÜV-Belege
  dokumentiert. Drei Standortkandidaten, nicht drei unabhängige GmbHs.
  Mindener Registerblatt-Schließung 2007 ist administrativ; kein
  Betriebsschließungsbeweis. Aktuelle vergütete Annahme pro Standort und
  gültige Zertifizierung bleiben offen.
- [M.R. Recycling Augsburg, 7384](feedback-20261014-mr-recycling-owner-lead.md):
  ausdrücklich vergüteter Ankauf angeboten; M.R. Metall ist derselbe
  Betreiber, kein zusätzlicher Händler. Unabhängiger Vollbeleg und aktuelle
  Annahmeadresse fehlen. Wankstraße 7/11b, Bedingungen und Preisangaben
  widersprechen sich; keine harmonisierte Preis-/Adressfreigabe.

Owner hat die Voth-Standortseiten Lingen und Minden selbst regulär gelesen:
https://www.g-voth.de/unternehmen_li.html und
https://www.g-voth.de/unternehmen_mi.html . Sie bestätigen die dokumentierten
Standortangaben, aber enthalten keine ausdrücklichen vergüteten
Ankaufbedingungen. Die pauschale Zertifizierungswerbung ersetzt keine gültige
Urkunde. Windmühlenstraße und Karlstraße nicht als Umzug zusammengeführt.

Owner hat außerdem https://mrmetall.de/impressum und
https://www.abfallentsorgung-augsburg.de/agb selbst regulär gelesen:
Michael Reinhardt/M.R. Recycling an 11b im Impressum; AGB nennen 7 als Platz
und Vergütung sowie Containergrenze 1000 kg. Gleiche Betreiberangaben sind
kein unabhängiger Zweitbeleg. Robert Reinhardt nicht umbenannt oder gemergt.

**Entscheidung:** Beide Forschungsberichte übernehmen; keine neuen
seedwirksamen Dossiers, aktuellen Preise, GEO oder Kontaktbedingungen
freigeben. Die Feedbacks bleiben offen. Es wurden keine Nachrichten an
Betreiber versandt und keine Erledigungsflags in Produktion geschrieben.

## Health, Schutz und Abschlussgate

- Dienst aktiv; Produktionsbestand read-only: 3881 Händler, 54 Materialien,
  2426 aktuelle Preise. Neueste Feedback-ID weiterhin 7390.
- Systemd-Warnungen der letzten sechs Stunden leer; App-JSON-WARNs separat
  gelesen: bekannte Münster-Preisleiter, AMR HTTP 502, Huth-Tabelle im noch
  alten Deployment. Keine erneute Sicherheitsumgehung oder erzwungene
  Produktions-Ingestion.
- Alle 155 bereits bestehenden fremden Dossier-/Recherchedateien per
  SHA-256 unverändert gegenüber `/tmp/opencode/wave-20261014-selection.json`.
- Keine vorhandene Dossierhistorie verändert; Overwrite-/Enum-/Deep-Link-/
  Emdash-Gate für drei neue Forschungsberichte ohne Seedänderung geprüft.
- Gemeinsamer `cargo test -p schrott-mcp-ingestion seed`: 14 bestanden,
  0 fehlgeschlagen; Log `/tmp/opencode/checkin-20261014-feedback-seed.log`.
- Nur die zwei Leadberichte und dieser Owner-Bericht gehören zum Commit.
  Veröffentlichung/Redeploy und produktive Änderungsverifikation bleiben
  wegen der zuletzt bestätigten fehlenden GitHub-Anmeldung offen; lokale
  Rechercheintegration ist kein Produktionsnachweis.
