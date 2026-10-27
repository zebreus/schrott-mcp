---
slug: nw-essen-schrotthandel-missal-metzler
clear_fields: geo
name: Schrotthandel Missal / Metzler
trader_type: schrotthaendler
state: NW
city: Essen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://schrotthandel-metzler-essen.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: '{"allowed": true, "conditions": "Kostenlose Schrott-/Altmetallabholung im Ruhrgebiet/NRW laut Betreiber; Demontage, Brennarbeiten, Sortierung und mögliche Entsorgungskosten vorab vereinbaren, nicht pauschal kostenlos. Ankaufpreis abhängig von Art, Menge und Aufwand."}'
provenance_section: Essen / Mülheim / Oberhausen / Bottrop
provenance_ankauf_raw: ja (mobil)
provenance_origin: table
---

# Schrotthandel Missal / Metzler

## Überblick

Mobile Betreiber-Sphäre Metzler/Jürgen Missal mit belegter kostenloser Altmetallabholung im Ruhrgebiet/NRW. Impressum nennt Jürgen Missal, Kalthofweg 4, zusätzlich „Vertretungsberechtigt: Markus Metzler“. Historische Zum-Oberhof-Adresse und rechtliche Rollen bleiben ungeklärt, daher keine Kontakt-/Adressfills. **Ankauf:** ausdrücklich nur „je nach Art und Umfang“; telefonische Preisvereinbarung, keine EUR/kg-Liste. **Verkauf:** keine separate Liste. **Gebühren:** Abholung wird kostenlos beworben, Preisberechnung berücksichtigt aber Demontage, Sortierung und mögliche Entsorgungskosten; keine pauschale kostenlose Demontage/Entrümpelung zusagen.

**GEO:** Kalthofweg 4 ist Impressums-/Kontaktadresse, Zum Oberhof 32 ein widersprüchlicher Portal-Lead; weder ist als Annahmestelle oder Abholstützpunkt bestätigt. Kein Betriebs-Pin bis Betreiberrollen, Adresse und Kundenannahme geklärt sind.

## Timeline

### Owner-Korrektur 27.10.2026: falsches GEO zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`; der Reverse-Lookup führt auf die Fernbus-Spur „Freiheit“ am Essener Hauptbahnhof und damit auf keinen Betriebsort. Betreiber-Impressum und 11880 belegen zwei Adressen derselben Betreiber-Sphäre: Kalthofweg 4, 45277 Essen (Jürgen Missal, Vertretungsberechtigt Markus Metzler) und Zum Oberhof 32, 45307 Essen (11880-Eintrag mit der im Dossier gespeicherten Domain, während die Betreiberseiten „Zum Oberhof“ nicht erwähnen). Beide liegen 4,07 bis 5,17 km vom gespeicherten Punkt entfernt und sind Wohnobjekte ohne belegten Annahmehof; Rollen- und Standortkonflikt zwischen Missal und Metzler bleiben offen, ebenso Registerstatus und Preislisten. Das gespeicherte Paar ist damit für genau diese Zeile falsch und wird mit `clear_fields: geo` zurückgenommen, ohne Ersatzkoordinate und ohne Annahmestellen-Behauptung; `status: aktiv` bleibt unverändert. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://schrotthandel-metzler-essen.de/impressum/ ; https://www.11880.com/branchenbuch/essen-ruhr/060440090B101936723/schrotthandel-juergen-missal.html ; https://www.11880.com/branchenbuch/essen-ruhr/120672194B37573047/markus-metzler-schrotthandel.html ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Vertiefung Nordwest 08.10.2026

- [Recherche 08.10.2026: Schrottpreise-Seite separat erneut direkt gelesen: ausdrücklich telefonische Angebote, Kalkulation abhängig von Menge/Zusammensetzung, Demontage/Sortierung und möglichen Entsorgungskosten. Keine EUR/kg-Zahlen, keine datierte Liste. Bereits heute präzisiertes pickup_json sachgerecht und unverändert; keine kostenlose Zusatzarbeit aus Werbebanner ableiten. Drei Betreiberdomains weiterhin nicht drei unabhängige Belege, Rollen-/Adressklärung offen. Verkaufspreisliste/Gebührentarife auf der geprüften Seite nicht veröffentlicht. Quelle: https://schrotthandel-metzler-essen.de/schrottpreise/]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Bestaetigung + neue Zweitspur, Klaerfall bleibt: alle drei Betreiber-Domains live re-verifiziert (schrotthandel-metzler-essen.de HTTP 200; schrotthandel-missal-essen.de HTTP 301; schrott-in-essen.de live) + schrottradar-Portal „Schrott und Metallhandel J. Missal" Essen. NEU: cylex-Eintrag „Schrotthandel Metzler, Zum Oberhof, Essen-Kray, 0175 9254..." stuetzt die Seed-Adresse Zum Oberhof 32 gegen das Impressum (Kalthofweg 4, Juergen Missal) — Widerspruch damit weiter unaufgeloest (Aggregator vs. Impressum); Feedback-Triage-Entscheid 01.10.2026 (Felder leer lassen) bleibt gueltig, kein Fill. Quelle(n): Live-Abrufe + cylex (02.10.2026).]

### Recherche 01.10.2026

- [Feedback-Triage 01.10.2026 (ID 2453): Adress-/Kontakt-Felder BLEIBEN LEER (Klaerfall bestaetigt) — Betreiber-Impressum (Einzelunternehmen Juergen Missal, Kalthofweg 4, 45277 Essen, Tel. 0170 3547895, schrotthandel-missal@web.de, ohne HRB) widerspricht Seed-Adresse 'Zum Oberhof 32'; Zweit-Domains schrotthandel-missal-essen.de + schrott-in-essen.de bestaetigen Kalthofweg 4 (alle eine Betreiber-Sphaere = formal EIN Beleg; keine Owner-Ausnahme ohne HRB). Firmania-Lead nennt Zum Oberhof 32, 45307 Essen als Alternativadresse — Widerspruch unaufgeloest, daher kein Adress-/Kontakt-Fill (Owner-Gate-Praezedenz). Schwerpunkt Abholung zweifelsfrei belegt ('Wir holen Schrott bei Ihnen kostenlos ab, Ruhrgebiet/NRW', Demontage/Brennarbeiten) → pickup_json gesetzt. Quelle(n): https://schrotthandel-metzler-essen.de/, https://schrotthandel-metzler-essen.de/impressum/ (Abruf 01.10.2026).]

- [Recherche 01.10.2026: Betreiber-Website live und tiefgecrawlt (Start, Schrottabholung, Kabelschrott, Metallhandel, Altmetall-Ankauf, Impressum einzeln abgerufen). Angebot: kostenlose mobile Schrott-/Altmetallabholung im Ruhrgebiet/NRW, Demontage/Brennarbeiten. Website per Regel auf Domain-Root normalisiert; website_status aktiv (eigener Live-Abruf 01.10.2026). Impressum (Einzelbeleg, unsicher — Einzelunternehmen Jürgen Missal, keine HRB, Owner-Direktive nicht anwendbar): Kalthofweg 4, 45277 Essen, Tel. 0170 3547895, schrotthandel-missal@web.de — weicht von Seed-Adresse „Zum Oberhof 32" ab; Straße/PLZ NICHT überschrieben (Klärfall Adresswiderspruch für Folgewelle). Kontaktfelder daher leer gelassen. Quelle(n): https://schrotthandel-metzler-essen.de/schrottabholung/, https://schrotthandel-metzler-essen.de/impressum/.]

### Importiert (Seed-Stand 2026-09-30)

- Abholung, Demontage, Brennarbeiten (Zum Oberhof 32)

### Recherche 08.10.2026

- [Korrektur 08.10.2026: Owner hat Schrottpreise-Seite erneut gelesen und die bisherige pauschale inkl.-Formulierung in pickup_json präzisiert. Kostenlose Abholung bleibt Betreiberwerbung; Demontage-/Sortieraufwand und mögliche Entsorgungskosten fließen laut derselben Betreiberseite in die Preiskalkulation ein. Keine Zusage kostenloser Zusatzarbeiten. Quelle: https://schrotthandel-metzler-essen.de/schrottpreise/]

- [Recherche 08.10.2026: Impressum, tatsächliche Schrottpreis-Seite und Ankaufseite einzeln live gelesen. Impressum enthält neben Jürgen Missal auch Vertretungsberechtigt Markus Metzler; keine HRB, USt-ID derzeit nicht erteilt. Beide Preis-/Ankaufseiten nennen 0170 3547895 UND 0175 9254651; letztere passt zur früheren Metzler-Verzeichnisspur, beweist aber keine Umzugs-/Rechtsträgerkette. Owner-Gate von 01.10. bleibt: keine Kontakt-/Adressfills, alle Domains derselben Sphäre kein unabhängiger Zweitbeleg. Quelle(n): https://schrotthandel-metzler-essen.de/impressum/ ; https://schrotthandel-metzler-essen.de/schrottpreise/ ; https://schrotthandel-metzler-essen.de/altmetall-alteisen-ankauf-essen-ruhrgebiet-nrw-fa-metzler/]
- [Recherche 08.10.2026: PREISPRÄZISIERUNG ANKAUF: je nach Art und Umfang möglich, tagesaktuelle telefonische Angebote, keine numerische öffentliche Liste. Schrottpreise-Seite nennt Zusammensetzung/Menge, Demontage-/Sortieraufwand und mögliche Entsorgungskosten als Kalkulationsfaktoren. VERKAUF: keine separate Preisliste in den geprüften Seiten. GEBÜHREN: kostenlose Schrottabholung beworben, nicht automatisch kostenlose Zusatzarbeiten; keine Tariftabelle. Vorhandenes pickup_json bleibt erhalten, aber dessen Formulierung inkl. Demontage/Brennarbeiten darf nicht als Zusage kostenloser Zusatzarbeiten ausgelegt werden. Quelle(n): https://schrotthandel-metzler-essen.de/schrottpreise/ ; https://schrotthandel-metzler-essen.de/altmetall-alteisen-ankauf-essen-ruhrgebiet-nrw-fa-metzler/]

### Recherche 20.10.2026

- [Recherche 20.10.2026: Schrottpreise-Seite direkt erneut abgerufen. Sie verlangt für ein konkretes Angebot den Anruf und nennt keine EUR/kg-Werte oder datierte Tabelle; Zusammensetzung/Menge, Demontage-/Sortieraufwand und mögliche Entsorgungskosten wirken laut Text auf die Preisberechnung. Seite wirbt zugleich mit kostenloser Abholung im Ruhrgebiet/NRW. ANKAUF: telefonische Einzelfallpreise, keine numerische Preisliste. VERKAUF: keine separate Produkt-/Verkaufspreisliste. GEBÜHREN: kostenlose Abholung behauptet; Zusatzarbeiten/Entsorgungskosten können anfallen und sind nicht pauschal kostenfrei, aber keine Tarife ausgewiesen. Identitäts-/Adresskonflikt aus bisherigen Einträgen bleibt offen; Frontmatter nicht geändert. GEO offen: keine Zuordnung von Kalthofweg 4 oder Zum Oberhof 32 als Schrottannahme; mobiler Dienst ist kein bestätigter Hof. Quelle: https://schrotthandel-metzler-essen.de/schrottpreise/]
