# Owner-Check-in: Neckartilda im Bestandsdossier, 14.10.2026

## Geprüfte Änderung

Bestehendes Dossier
`dossiers/sl/sl-st-ingbert-66386-rohstoffhandel-neckartilda-nl-st-ingbert.md`
erhält Überblick und zwei datierte Timeline-Notizen zum Betreiberkonflikt.
Der bereits geprüfte Aufnahmebericht
`research/feedback-20261014-neckartilda-admission.md` bleibt erhalten.

Owner hat erneut das [Betreiber-Impressum](https://wk0s76uib.hier-im-netz.de/Impressum)
und das [NorthData-Originalprofil](https://www.northdata.de/NECKARTILDA%20Rohstoffhandel%20GmbH,%20Berlin/Amtsgericht%20Charlottenburg%20(Berlin)%20HRB%20217698%20B)
selbst gelesen. Impressum: HRB 31272 Zweibrücken, Pascal Keßler, beide getrennten
Plätze. Profil-HTML: `title="erloschen"`, historische HRB 31272 und eingebettetes
Löschungsereignis **08.01.2025** für die Berliner Gesellschaft. Kein aktueller
amtlicher Tagesauszug oder heutiger Nachfolgebetreiber daraus behauptet.

**Keine Gleichsetzung von Gesellschaftsende und Platzschließung.** St. Ingbert
bleibt `pruefung`; stabiler Slug, Standortgrenzen und sämtliche Frontmatterfelder
unverändert. Kein RP-Platz aufgenommen, keine aktiven Kontakte oder Bedingungen
aus dem alten Impressum importiert. Feedback 7380 und historische IDs 1415/1423
bleiben mit korrekter Zuordnung erhalten.

## Owner-Gate und Tests

- Import-Timeline vollständig erhalten, nur belegten Recherchekontext ergänzt.
- Frontmatter bytegleich zum bisherigen Commit geprüft: keine Überschreibung
  strukturierter Fakten, kein neuer Enum, keine neuen Websitefelder/Domainwurzeln
  oder Em-dash-Füllwerte, keine GEO-Werte.
- `cargo test -p schrott-mcp-ingestion seed`: **14 bestanden, 0 fehlgeschlagen**;
  `/tmp/opencode/checkin-neckartilda-dossier-seed.log`.
- Alle **155** geschützten fremden Dateien SHA-256-geprüft: 0 Abweichungen.
- Nur dieses Dossier und dieser Owner-Bericht gehören zur lokalen Integration.

## Check-in und offene Arbeit

Service aktiv, HTTPS-`/health` erfolgreich. Keine systemd-Warnings in sechs
Stunden; separate JSON-WARNs: AMR HTTP 502, Huth alte Produktionsversion ohne
Preistabelle, Münster erneut 04:55 UTC mit verdächtiger Preisleiter korrekt
abgelehnt. Produktionsstatistik read-only: **3881 Händler / 54 Materialien /
2426 aktuelle Preise**. Neueste Feedback-IDs 7381–7390 unverändert; bestehende
Triageberichte maßgeblich. Keine Geo-Logs, keine Produktionswrites.

Disjunkte Research-Aufträge zu RHR Recklinghausen (7379) und Hermann Korn
(7377) wurden gestartet, aber beide vom Subagenten-Nutzungslimit beendet.
Keine Ergebnisdateien vorhanden; **keine abgeschlossene Prüfung oder Aufnahme**
behauptet. Die zusätzlich eingegangene 6h-Recherchewelle ist ebenfalls noch
offen und nicht durch diese einzelne Dossierkorrektur ersetzt oder abgeschlossen.

`gh auth status`: weiterhin keine GitHub-Anmeldung. Push, Redeploy und
Produktionsverifikation der neuen Seednotizen bleiben offen. Der bestehende
Produktionsservice ist nicht als Veröffentlichung dieses lokalen Stands gewertet.
