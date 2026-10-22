# Zwischenprüfung — Recherchewelle 21.10.2026, 06:15 UTC

**Kein finales Owner-Gate:** MV und TH sind fertig und vorläufig geprüft; BW läuft noch. Diese Notiz hält die bisherige Prüfung der beiden abgeschlossenen Shards fest.

## Struktur und Arbeitsbaumscope

- Die 20 MV-Dossiers des Scopes wurden gegen `HEAD` geprüft: alle 20 Slugs eindeutig; `slug`, `state` und `trader_type` unverändert; Statuswerte gültig; keine Koordinaten ergänzt oder geändert. `git diff --check -- dossiers/mv` ist sauber.
- Eine weitere geänderte MV-Datei (`mv-wismar-nord-rwe-umwelt-mecklenburg-vorpommern-contai.md`) gehört nicht zum Scope und hat Dateizeit 20.10.2026, 09:09 UTC; sie ist vorhandene Arbeit und wird dieser Welle nicht zugerechnet.
- Die 20 TH-Dossiers stimmen exakt mit dem TH-Scope überein. YAML-Frontmatter wurde geparst; `slug`, `state` und `trader_type` blieben unverändert; Statuswerte und Koordinaten-Diffs sind sauber. Kein Deep-Link in geänderten `website`-Feldern, kein Em-Dash in den TH-Diffs; `git diff --check -- dossiers/th` ist sauber.
- Scope und Produktionsdatenbank blieben getrennt: die Produktionsdatenbank wurde ausschließlich read-only abgefragt.

## Konkreter Owner-Fund: Auto Schnell — neue Adresse, altes GEO

Die Betreiberwebsite und das Impressum bestätigen Auto Schnell OHG, An der Bundesstraße 2, 23996 Bad Kleinen-Niendorf, Kontakt und Registerangabe. Eine unabhängige Verbands-/Fachverzeichnis-Spur stützt den Standort; die GESA-Angabe liegt nur als Fachverzeichnis-Spiegel vor, nicht als direkt bestätigter GESA-Eintrag. Da die Betreiberwebsite Werkstatt und Pannenhilfe beschreibt, aber keine aktuelle Altfahrzeugannahme belegt, ist `status: pruefung` beibehalten.

Die read-only-Abfrage in `public.db` zeigt für denselben Slug bislang leere Straße und PLZ, `city=Bad Kleinen 23996`, aber bereits `lat=53.7713`, `lon=11.4716`. Das Dossier dokumentiert nun die belegte Straße/PLZ, enthält jedoch bewusst kein Koordinatenpaar. Laut [Seed-Vertrag](../docs/trader-seed.md) bleibt das alte GEO ohne neues Paar erhalten; die Adressänderung validiert es nicht. **Vor einem Seed-Rollout muss die Zuordnung dieses gespeicherten Punkts geklärt werden.** Es wurden weder Koordinaten ergänzt noch DB-Daten verändert; kein Rust-Geocoding.

## TH-Quellen- und Feldprüfung (vorläufig)

- In der eigenen Quellenstichprobe bestätigen die Wagner-Betreiberseite Identität/Adresse/HRB/Kontakt/Leistungen, Scholz' Betreiberauftritt plus die visuell geprüfte Altfahrzeug-Bescheinigung die Betriebsadresse Großengottern (gültig bis 23.10.2027), G&K die konkreten Annahme-/Gebührenbedingungen und Jäger-Impressum plus Gelbe Seiten die Anschrift/Kontaktdaten. Kein pauschaler Ankaufspreis wurde daraus abgeleitet.
- Die im Agentenbericht genannten Geocoding-Kandidaten sind Geschäftsstandorte, nicht automatisch öffentliche Annahmestellen: Wagner Recycling (Salinenstraße 91, Erfurt), Scholz Großengottern (Mülverstedter Landstraße 2), Jäger (Burgweg 1a, Schmalkalden-Näherstille) und G&K Utsch (Gewerbepark 4, Rudolstadt). Für keinen wurde ein Koordinatenpaar ergänzt.
- Nowatzky (Rechtsträger-/Standortabweichung), GÖREC (andere Gesellschaft an derselben Adresse) und Fallenstein (nur Verzeichnis-Leads) bleiben ausdrücklich ungeklärt und sind keine bestätigten Intake-GEO-Punkte. Owner-Gate korrigierte Fallenstein: Die neue PLZ/Rufnummer sowie die zuvor vorhandene, nur durch Verzeichnis-Leads gestützte Straße sind aus dem Frontmatter entfernt; Details und Begründung bleiben im Timeline-Nachtrag.

## Ergebnis und offene Gates

- Vorläufige Prüfung ist kein vollständiges Quellen-/Identitätsgate für die 60 Dossiers. BW steht noch aus; der Gesamt-Owner-Gate folgt erst nach dessen Abschluss.
- Root frei: **1.106.538.496 Bytes (~1.055,1 MiB)**, rund **144,9 MiB unter** dem Testminimum von 1.200 MiB. `cargo` fehlt im PATH; daher kein Seed-Test oder Build.
- Keine Produktionsdatenbankänderung, kein Commit, Push oder Redeploy. Die vorhandenen Dossieränderungen außerhalb dieses Shards blieben unangetastet.
