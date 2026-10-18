---
slug: by-roding-jansen
name: Jansen
trader_type: sonstige
state: BY
city: Roding
street: ''
postcode: ''
phone: ''
clear_fields: 'phone'
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: 'Neu: nur Verzeichnis-Beleg (lokaleschrottplatz.de, Ankauf unklar)'
provenance_ankauf_raw: unklar (Register-Prosa)
provenance_origin: prose
---

# Jansen

## Überblick

Unvollständige Betreiberidentität „Jansen“: Flurweg 8, 93426 Roding und 0171 1884077 als unzureichend belegte aktuelle Fakten im Dossier-Frontmatter zurückgenommen; frühere, heute nicht unabhängig verifizierte Leadbehauptungen bleiben Historie. Ein am 18.10.2026 direkt gelesener Telefonbuch-Aggregator nennt Romano Jansen an derselben Adresse, aber 09461 401336; konkreter lokaler Gegenhinweis, kein Nachweis eines Schrotthändlers oder Anlieferorts. Weder Namen noch neue Nummer übernehmen, die alte Mobilnummer nicht als falsch/abgeschaltet widerlegt. Status `pruefung`, keine Schließung. Keine belastbar zugeordneten veröffentlichten Zahlen für Ankauf, Verkauf oder Gebühren; keine identitätsgesicherte Betreiberquelle für eine Preisprüfung. Datenbankübernahme erst nach Integration/Rebuild/Seed. Kein GEO dieses Reviews; erhaltene ungeprüfte Punkte bleiben Rolloutgate, alte Geokodierankündigung keine aktuelle Freigabe.

## Timeline

### Recherche 08.10.2026

- [Recherche 08.10.2026: NorthData-Suche „Jansen Roding“ zeigt im ersten Ergebnisblock keine kongruente lokale Schrottidentität; unscharfe/paginierte Treffer und nichtregisterpflichtige Gewerbe begrenzen die Aussage. Portal-Kandidat lokaleschrottplatz.de/jansen/ HTTP403, Brave-Namenssuche 429; kein neuer Betreiberbeleg und keine Preisprüfung an identitätsgesicherter Website möglich. Historische Flurweg-/Telefon-Ausnahme unverändert, ausdrücklich nicht als heutiger Zwei-Quellen-Nachweis gewertet. Owner: vollständigen Namen hinter Jansen und Gewerbetätigkeit an Flurweg 8 klären; Quelle(n): https://www.northdata.de/?query=Jansen%20Roding ; https://lokaleschrottplatz.de/jansen/ (Abrufgrenze).]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Adresse/Telefon als begründeter Ausnahmefall gefüllt (kongruente Aggregatoren inkl. Telefonbuch): Flurweg 8, 93426 Roding, Tel. 0171 1884077; kein Betreiber/HR-Beleg (Kleinstbetrieb) — Restunsicherheit bleibt, status pruefung unverändert. Hinweis: Adresse neu gefüllt — Koordinaten neu geocodieren; Quelle(n): kongruente Verzeichnis-/Telefonbuch-Eintraege (Flurweg 8, 93426, 0171 1884077).]

### Importiert (Seed-Stand 2026-09-30)

- Registerfund ohne geprüfte Website (PLZ 93426)

### Korrektur 18.10.2026

- [Korrektur 18.10.2026: Straße/PLZ/Telefon im Frontmatter zurückgenommen, clear_fields phone eng begrenzt freigegeben; Rücknahme unzureichend belegter aktueller Tatsachen, keine Widerlegung der Adresse oder Nummer. Heute direkt gelesener Aggregator telefonbuchsuche.de nennt Romano Jansen, Flurweg 8, 93426 Roding und 09461 401336; kein Betreiber-/Schrottbeleg, keine Namens-/Telefonübernahme. Katrin Maier nur Suchsnippet-Lead, keine Betreiberrolle. NorthData-Suchen Jansen Roding/Romano Jansen im gelesenen ersten Block ohne kongruente Zuordnung, keine vollständige Negativregisterauskunft. lokaleschrottplatz.de/jansen heute HTTP 404, historisches 403 nicht als heutiger Befund fortgeschrieben. Gelesene Jansen-Recycling-Group-Seite nennt niederländische Standorte, P.A.-Jansen-Impressum Ahrweiler; keine Roding-Kette belegt, keine fremden Daten übertragen. Weitere Suchen teilweise 401/CAPTCHA/irrelevant, keine erschöpfenden Negativbefunde. Status pruefung; keine Schließung, Aktivierung oder belastbar zugeordneten Preis-/Gebührenzahlen. Historische 02./08.10.-Abrufe nicht nachträglich verifiziert, uncommittierter 12.10.-Entwurf kein Nachweis damaliger Recherche. Alte Geokodierungsankündigung nicht geltend; ungeprüftes DB-GEO bleibt erhalten und Rolloutgate. Quellen: https://telefonbuchsuche.de/romano-jansen-flurweg-93426-roding ; https://www.northdata.de/?query=Jansen%20Roding ; https://www.northdata.de/?query=Romano%20Jansen ; https://lokaleschrottplatz.de/jansen/ ; https://jansengroup.com/de/ ; https://www.jansen.de/impressum/]

### Owner-Korrektur 18.10.2026 (Seed-Verhalten)

- [Owner-Korrektur 18.10.2026: Heutiger mode=ro/query_only-Abgleich mit public.db zeigt Telefon 0171 1884077 und Flurweg 8/93426 weiterhin gespeichert; nur Bestandsaufnahme, keine Tatsachenverifikation oder Bestätigung der im Entwurf behaupteten 12.10.-Abfrage. Nach Rebuild/Integration entfernt clear_fields phone das gespeicherte Telefon und bleibt bei jedem Seed autoritativ, auch gegen spätere Drift mit gleich gebliebenem Seed-Hash. Leeres Telefon allein würde Enrichment behalten. Straße/PLZ werden beim geänderten Seed leer überschrieben, vorhandenes GEO 49.1943/12.5196 dagegen behalten und nicht neu validiert. Ungeklärte Händler-/Punktzuordnung bleibt Veröffentlichungsblocker, keine GEO- oder Produktionsmutation dieses Check-ins. Grundlage: docs/trader-seed.md; heutige Abfrage dokumentiert in /tmp/opencode/checkin-20261018-0900-production-gate.json.]
