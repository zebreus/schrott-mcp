---
slug: sn-rochlitz-db-recycling-denis-blum
clear_fields: geo
name: DB Recycling (Denis Blum)
trader_type: schrotthaendler
state: SN
city: Rochlitz
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://www.dein-schrottplatz.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: 4) Chemnitz und Mittelsachsen
provenance_ankauf_raw: ja
provenance_origin: table
---

# DB Recycling (Denis Blum)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Fe/Bunt, Autoankauf, Container, Altpapier/-kleider
- Einzug Mittweida/Geithain/Penig/Lunzenau

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2657 bestaetigt — Betreiber-Selbstbezeichnung Metall und Schrotthandel / Schrottplatz Rochlitz (Ankauf Buntmetall, Autoverwertung, Containerdienst) → trader_type sonstige nach schrotthaendler korrigiert; Betreiberseite nennt Schuetzenstr. 6, 09306 Rochlitz, (03737) 7864315, 0162 8938850, info@dein-schrottplatz.de, Zeiten Mo-Fr 9-12/13-18, Sa 9-12 — Einzelunternehmen ohne HRB, Einzelbeleg ohne Zweitquelle, daher Timeline-only, website live aktiv; Quelle(n): https://www.dein-schrottplatz.de/, https://www.dein-schrottplatz.de/pages/kontakt.php]

### Abrufprüfung 22.10.2026

- Ingestion-Lauf #2402 (04:56 UTC) scheiterte für diesen Händler mit einem Request-Fehler beim Abruf von `https://www.dein-schrottplatz.de/pages/preise.php`. Owner-Nachprüfung um 05:00 UTC: DNS löst `www.dein-schrottplatz.de` zu `85.215.148.73` auf; sowohl Preis-URL (20 s) als auch Homepage (10 s) liefen vom Projektserver in einen Timeout (HTTP 000, kein Inhalt). Ein ähnlicher Timeout vom Projektserver war bereits am 21.10. dokumentiert. Das belegt nur die fehlende Erreichbarkeit aus dieser Umgebung, keinen globalen Website-Ausfall. Ohne Quellinhalt keine Parseränderung und kein Preis-Fallback; bei wiederhergestelltem Zugriff erneut gegen die offizielle Preistabelle prüfen.

### Owner-Korrektur 27.10.2026: Einzelbeleg nicht als Adressfill

- [Korrektur 27.10.2026: Betreiber-Homepage, Kontaktseite und Preisliste erneut direkt gelesen. Der Betreiber nennt Schützenstr. 6, 09306 Rochlitz und eine Preisliste mit Gültigkeit ab 03.10.2026; das bestätigt die Betreiberangabe als aktuellen Einzelbeleg, aber es wurde keine unabhängige zweite Standortquelle gefunden. Straße daher wie die bereits geleerte PLZ nicht strukturiert übernehmen. `status: aktiv` bleibt durch die aktuelle Betreiber-Preisliste zum Metallankauf gestützt; `website_status: aktiv` beschreibt die erreichbare Seite. Kein Schließungsurteil. Quelle(n): https://www.dein-schrottplatz.de/ ; https://www.dein-schrottplatz.de/pages/kontakt.php ; https://www.dein-schrottplatz.de/pages/preise.php ; README.md, Quellenhierarchie.]

### Owner-Laufzeitkorrektur 27.10.2026: ungeprüfte Adresse und GEO zurücknehmen

- [Owner-Laufzeitkorrektur 27.10.2026: Im Produktionslauf #2873 setzte der Preis-Handler Straße und PLZ erneut auf `Schützenstr. 6`/`09306`; das Dienstjournal bestätigt das Kontaktupdate um 03:43:55 UTC. Die vorhandene GEO 51.0476634/12.8084764 löst per Nominatim exakt auf das Gebäude Schützenstraße 6 auf, beweist aber nicht die Zuordnung des Betriebs zu diesem Gebäude. Die Recherche zu `SMS Sterns` findet dieselbe Anschrift und denselben Inhaber Denis Blum; die beiden vom selben Inhaber betriebenen Websites sind keine unabhängigen Standortquellen und belegen weder einen getrennten Parallelbetrieb noch die konkrete Annahmestelle dieses Slugs. Nach README-Quellenhierarchie bleibt die Zuordnung ein Einzelbeleg. Straße/PLZ werden daher nicht als Händlerstandort geführt und `clear_fields: geo` zieht den dafür ungestützten Punkt beim regulären Seed zurück; das behauptet nicht, die Gebäudeadresse selbst sei falsch. Der Preis-Handler validiert den Impressum-Block weiter, veröffentlicht daraus aber keine Straße/PLZ/Stadt mehr. Der neue Timeline-Eintrag ändert den Seed-Payload-Hash, damit der nächste freigegebene Seed-Lauf die bereits zurückgenommenen Kernadressfelder tatsächlich leert. Kein direkter Produktions-Write. Quellen: read-only `public.db`; `journalctl`/Lauf #2873; https://www.dein-schrottplatz.de/pages/kontakt/impressum.php ; http://www.sms-rochlitz.de/pages/kontakt/impressum.php ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.0476634&lon=12.8084764&zoom=18&addressdetails=1 ; README.md, Quellenhierarchie.]
