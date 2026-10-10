---
slug: ni-osnabruck-reinhard-peters-schrottplatz
name: Reinhard Peters Schrottplatz
trader_type: schrottplatz
state: NI
city: Osnabrück
street: ''
postcode: ''
phone: ''
clear_fields: 'phone'
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Osnabrück / Emsland / Grafschaft Bentheim
provenance_ankauf_raw: ja
provenance_origin: table
---

# Reinhard Peters Schrottplatz

## Überblick

Reinhard Peters bleibt ein ungeklärter Osnabrücker Schrott-Verzeichnislead. GS nennt Kleiner Muskamp 2, ältere Quellen 2 A; kein Betreiber-/Gewerbenachweis oder aktuelle Hofannahme. Aktivstatus zurückgenommen, alte Standort-/Kontaktwerte nur historisch geführt, Typ nicht durch Vermutung ersetzt. Keine verifizierten Preis-/Gebührenlisten.

## Timeline

- [Owner-Korrektur 10.10.2026: Explizite Seed-Direktive clear_fields für phone ergänzt. Die nicht ausreichend belegte aktuelle Kontaktzuordnung soll nach geprüfter Veröffentlichung durch normalen Seedupsert entfernt werden; leeres Markdown allein würde den gespeicherten Wert erhalten. Alte Nummer und Beleglücke unten erhalten. Keine direkte Produktionsmutation, keine GEO-Löschung oder Schließungsbehauptung. Grundlage: docs/trader-seed.md; research/checkin-20261010-waveD-nordwest-owner-gate.md.]

### Recherche 09.10.2026 / Korrektur 10.10.2026

- [Recherche 10.10.2026: GS-Detail aus Wave D direkt gegengeprüft: Peters Reinhard, Kleiner Muskamp 2 (ohne A), 49078 Osnabrück-Hellern, 0541 442168, keine Betreiberwebsite/Zeiten. Portal-Materiallisten sind keine Betreiberzusagen. Northdata-Suche nur unspezifische Namensvetter, kein amtlicher Negativnachweis. Kein Ankauf-/Verkaufspreis oder Gebührentarif belegt; Quelle(n): https://www.gelbeseiten.de/gsbiz/74fd9935-652e-4380-986e-dde08c09040c ; https://www.northdata.de/?query=Reinhard+Peters+Osnabr%C3%BCck]
- [Korrektur 10.10.2026: Frühere „unabhängig bestätigt“/aktiv-Ableitung aus 11880/lokaleschrottplatz/firmania erfüllt README nicht. status aktiv auf pruefung; street „Kleiner Muskamp 2 A“, postcode 49078 und phone 0541 442168 aus strukturierter Frontmatter zurückgenommen, hier vollständig erhalten. Keine Hausnummer gewählt, keine Schließung/Ortsverschiebung. Legacy trader_type schrottplatz bleibt unverändert als ungeklärte Seed-Kategorie, nicht bestätigter Hof; Slug/GEO erhalten. Quelle: README.md, Source hierarchy; obige Leads und Recherche 02.10.2026]

### Recherche 02.10.2026

- Beleglage: 1) 11880-Eintrag Reinhard Peters Osnabrück, aktualisiert 05.09.2026: Kleiner Muskamp 2 A, 49078 Osnabrück (Hellern), Tel. (0541) 442168, Branche Altmetallrecycling. 2) Unabhängig bestätigt: lokaleschrottplatz.de und firmania.de führen gleiche Adresse Kleiner Muskamp 2A, 49078 Osnabrück + Tel. 0541 442168 (Ankauf Stahl/Eisen, Kupfer, Alu, Messing, Kabel).
- `status: aktiv` bestätigt (September-2026-Aktualisierung + aktive Lead-Seite). Kein `## Überblick` angelegt.

### Importiert (Seed-Stand 2026-09-30)

- Schrott
