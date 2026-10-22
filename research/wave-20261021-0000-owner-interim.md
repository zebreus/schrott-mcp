# Zwischenprüfung — Recherchewelle 21.10.2026, 00:12 UTC

**Kein finales Owner-Gate:** RP läuft noch. Diese Notiz hält die bisherige Prüfung der fertiggestellten, bundesland-disjunkten ST- und SN-Shards fest.

## Struktur und Arbeitsbaumscope

- Die 40 abgeschlossenen ST-/SN-Scope-Dossiers wurden gegen `HEAD` geprüft: `slug`, `state` und `trader_type` unverändert; Statuswerte gültig; keine Koordinatenpaare neu ergänzt/geändert. `git diff --check -- dossiers/st dossiers/sn` ist sauber.
- Zusätzliche, außerhalb der aktuellen Scope-Liste geänderte Dateien in ST/SN tragen Dateizeit 20.10.2026, 09:03 UTC und sind vor dieser Welle vorhandene Arbeit; sie werden nicht dieser Welle zugerechnet.
- Der Scope selbst enthält 60 eindeutige Slugs aus dem aktuellen read-only Pool, je 20 passend zu RP, ST, SN.

## Konkreter Owner-Fund: Schliwa-Feldsemantik

`st-calbe-wertstoffhandel-mike-schliwa.md` dokumentiert einen Widerspruch der einzigen Verzeichnisquelle bei Donnerstag-Öffnungszeiten und setzt den Status auf `pruefung`. Ein leeres `opening_hours` allein entfernt jedoch laut `docs/trader-seed.md` keinen gespeicherten Wert. Owner-seitig wurde deshalb die dokumentierte Direktive `clear_fields: 'opening_hours'` ergänzt. Das ist kein Produktionsschreibzugriff; der Seed-Test/Rollout bleibt ausstehend.

## Adresse und GEO bei Habermann/MHG

Der ST-Dossierdiff ergänzt für Horst Habermann die PLZ **06847**; die `website`- und `name`-Zuordnung bleiben bewusst unverändert. Die offizielle MHG-Startseite und das Impressum wurden direkt gegengeprüft: MHG erklärt die Übernahme des Habermann-Entsorgungsfachbetriebs zum 01.08.2026 und nennt Taubenstraße 37, 06847 Dessau-Roßlau; die MHG-Preisliste trägt den Stand 20.10.2026. Das belegt den Betreiber-/Standortwechsel laut MHG, nicht die rechtliche Identität der Legacy-Dossierentität.

Read-only `public.db` enthält für den Habermann-Datensatz bereits **51.8333, 12.25**. Der Dossier-Eintrag enthält kein neues Koordinatenpaar; laut Seed-Vertrag bleibt der gespeicherte Punkt erhalten. Keine Koordinate wurde ergänzt. Dieser vierte, bislang in der vorherigen Welle nicht erfasste GEO-Punkt ist vor jedem Seed-Rollout zu klären.

## Ergebnisbelege und offene Grenzen

- SN-Recherchebericht meldet Identitäts-/Adressfragen weiterhin offen, nennt die Preisfunde Schrott24/Schrottjungs als bedingt bzw. teilweise nicht lokal zurechenbar und hat keine Koordinaten geschrieben. Vollständige fachliche Quellenprüfung ist Teil des späteren Gesamt-Gates.
- Der dritte Shard RP ist noch nicht abgeschlossen; keine Vollfreigabe der 60 Dossiers.
- Root frei bei dieser Zwischenprüfung: **1.180.667.904 Bytes (~1.126,0 MiB)**, etwa **74,0 MiB unter** dem Testminimum 1.200 MiB. `cargo` ist im PATH nicht vorhanden. Kein Cargo-Test/Build, Commit, Push oder Redeploy.
- Produktionsdatenbanken wurden nicht beschrieben.
