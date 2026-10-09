---
slug: mv-duvendiek-18442-dapa
name: DAPA GmbH
trader_type: autoverwertung
state: MV
city: Duvendiek
street: ''
postcode: '18442'
phone: 038321 363
email: ''
opening_hours: ''
website: https://www.dapa-hst.de/
website_status: aktiv
status: aktiv
description: Autoverwertung und Gebrauchtteileverkauf am DAPA-Standort Duvendiek. Betreiber bietet Altfahrzeugannahme mit Verwertungsnachweis, Unfallwagenankauf und Abholung nicht fahrbereiter Fahrzeuge; Transportkosten nach Aufwand.
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: ja (Autoankauf-Rubrik lt. Website)
provenance_origin: table
---

# DAPA GmbH

## Überblick

Eigenständiger DAPA-Verwertungsstandort, **nicht die Werkstatt am Hauptsitz Lüssow**. Die aktuelle Standortseite nennt Kranichblick 32, 18442 Duvendiek, und 038321 363. Der verzeichnisbasierte Gegenlead Hausnummer 30 ist kein gleichrangiger Beleg. Dennoch bleibt die Straße im Frontmatter vorerst leer: Die Betreiberadresse ist eine eindeutige Eigenangabe, aber es fehlt ein unabhängiger Hausnummerbeleg; das Impressum enthält keine HRB und erfüllt die strenge Alleinbeleg-Ausnahme des README nicht vollständig.

Die juristische Betreiberidentität DAPA GmbH ist nun registerseitig mit dem Hauptsitz abgeglichen. Fahrzeuge werden verwertet, Unfallwagen angekauft und geprüfte Gebrauchtteile verkauft. Abholung ist **nicht pauschal kostenlos**, sondern Transport nach Aufwand. Website-Werkstattzeiten und der 24/7-Pannendienst gelten nicht automatisch für die Annahme in Duvendiek.

Preisfunde sind nicht austauschbar: individuelle Fahrzeugbewertung/vereinbarte Auszahlung (Ankauf), Gebrauchtteile nur auf Anfrage (Verkauf) und Abholtransport nach Aufwand mit Vorabangebot (Gebühr); ein fester öffentlicher Tarif ist nicht ausgewiesen. Hausnummer 30/32 bleibt in Konflikt, daher Adresse nicht geokodieren, bis sie geklärt ist.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Abschleppdienst, Autoverwertung, Autoankauf
- Adresse: Duvendiek 18442, Kranichblick 32 (Klärfall entschieden: Duvendiek, nicht Niepars)

### Korrektur 30.09.2026

- [Korrektur 30.09.2026: city von 'Duvendiek 18442' auf 'Duvendiek' bereinigt (PLZ gehört nicht in den Ort, Feedback 1651). Impressum dapa-hst.de nennt nur Sitz Lüssow/Stralsund (Am Langendorfer Berg 8) — kein Frontmatter-Fill daraus (Stadt-Mismatch); Website bleibt (Name + PLZ-Raum matchen). Quelle: https://www.dapa-hst.de/impressum/.]

### Korrektur 30.09.2026 (Feedback-Triage)

- [Korrektur 30.09.2026 (Feedback-Triage): Re-Report fb1766 (»Ort fehlerhaft, Strasse/PLZ/Telefon fehlen«) — Tiefencrawl: Duvendiek-Zweigstelle ist real (eigene Standortseite /locations/duvendiek: »DAPA Autoverwertung Duvendiek, Kranichblick 32, 18442 Duvendiek, Tel. 038321-363« + Footer/Standorte-Seite). Impressum weiter nur HQ Lüssow/Stralsund → Stadt-Mismatch-Regel, kein Fill daraus. PLZ 18442 (Betreiber + Gelbe Seiten + Das Oertliche) → gefuellt; Telefon 038321 363 (Betreiber + GS + DO) → gefuellt. Strasse NICHT gefuellt: Konflikt Kranichblick 32 (Betreiber, 1 Quellfamilie) vs. Kranichblick 30 (Gelbe Seiten + Das Oertliche + Stadtbranchenbuch) — Klaerfall, Vermerk statt Fiktion. website_status aktiv (Impressum DAPA GmbH + Luessow/Stralsund, Name+Ort ✓; Duvendiek als Zweigstelle am selben Betreiber belegt). Oeffnungszeiten Duvendiek nicht ausgewiesen (nur Werkstatt-Zeiten HQ) → leer. Quelle: dapa-hst.de (Startseite, /locations/duvendiek, /standorte, /impressum), gelbeseiten.de, dasoertliche.de.]

### Recherche 01.10.2026 (Feedback-Triage 2928)

- [Recherche 01.10.2026: Feedback 2928 („Straße fehlt") berechtigt, aber fortbestehender Klärfall — keine Änderung: Straßenkonflikt Kranichblick 32 (Betreiber-Standortseite) vs 30 (Gelbe Seiten + Örtliches + Stadtbranchenbuch) besteht fort (Re-Verifizierung 01.10.2026); PLZ/Telefon/website_status bereits gesetzt und korrekt. Koordinaten: kein Frontmatter-Feld, Neu-Geocodierung erst nach Klärung der Hausnummer sinnvoll. Quelle(n): s. Recherche-Einträge unten, Re-Verifizierung 01.10.2026]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Bestand re-verifiziert – Betreiber-Standortseite nennt weiter "DAPA Autoverwertung Duvendiek, Kranichblick 32, 18442 Duvendiek, Telefon 038321-363"; Impressum weiter nur HQ Am Langendorfer Berg 8, 18442 Lüssow/Stralsund; Straßenkonflikt 32 (Betreiber) vs. 30 (Verzeichnisse) besteht fort → street weiter leer; Quelle(n): dapa-hst.de/locations/duvendiek + /impressum (Abruf 01.10.2026)]
- [Recherche 01.10.2026: Keine neuen Fills (PLZ/Telefon/website_status bereits gesetzt); Statusfeld unverändert; Quelle(n): siehe Vorbullet]

### Recherche 05.10.2026

- [Recherche 05.10.2026: Zweitseitige Identitätsprüfung — Impressum benennt DAPA GmbH, Am Langendorfer Berg 8, 18442 Lüssow/Stralsund, Detlef Salomon als Inhaltsverantwortlichen; Northdata bestätigt DAPA GmbH, HRB 21162 Stralsund, dieselbe Hauptsitzanschrift und u.a. Autoverwertung/Fahrzeug-/Ersatzteilhandel. Die Duvendiek-Detailseite ist ausdrücklich Teil dieses Betreiberauftritts, kein Hauptsitz-Stadt-Mismatch. Straße weiterhin nicht gefüllt: Kranichblick 32 ist Betreiber-Einzelangabe; kein unabhängiger Hausnummerbeleg, Impressum ohne HRB erfüllt nicht sämtliche README-Ausnahmekriterien. Alte Verzeichnis-Hausnummer 30 nicht als bewiesene Alternativadresse behandeln. Quelle: https://www.dapa-hst.de/standorte/impressum/ ; https://www.northdata.de/DAPA+GmbH,+L%C3%BCssow ; https://www.dapa-hst.de/locations/duvendiek/]
- [Recherche 05.10.2026: Vollcrawl aller sechs Standorte — Duvendiek, Stralsund/Lüssow, Grimmen, Ribnitz-Damgarten, Jarmen und Rügen einzeln geöffnet, dazu Standortübersicht, Kontakt, Autoverwertung und Autoankauf. Duvendiek spezialisiert auf Verwertung/Ersatzteile, Werkstatt/Abschleppdienst laut Detailseite an anderen Standorten. Für Altfahrzeugabgabe Fahrzeugschein/Fahrzeugbrief mitbringen; bei nicht fahrbereitem Fahrzeug Abholung möglich, Transport nach Aufwand. Deshalb keine Übernahme der globalen Werkstattzeit Mo-Fr 8–18 oder Notrufbereitschaft 24/7 in opening_hours; keine generische Schrottannahme aller Metalle daraus ableiten. Beschreibung präzisiert; übrige fehlende Felder nicht aus HQ-Facts aufgefüllt. Quelle: https://www.dapa-hst.de/standorte/ ; https://www.dapa-hst.de/locations/duvendiek/ ; https://www.dapa-hst.de/locations/stralsund/ ; https://www.dapa-hst.de/locations/grimmen/ ; https://www.dapa-hst.de/locations/ribnitz-damgarten/ ; https://www.dapa-hst.de/locations/jarmen/ ; https://www.dapa-hst.de/locations/samtens-auf-ruegen/ ; https://www.dapa-hst.de/standorte/kontakt/ ; https://www.dapa-hst.de/autoverwertung/ ; https://www.dapa-hst.de/autoankauf/]
- [Recherche 05.10.2026: Preiswege getrennt — ANKAUF: individuelle Fahrzeugbewertung und vereinbarte Auszahlung, keine numerische Schrott-/Autoankaufsliste. VERKAUF: Gebrauchtteile nach Anfrage; der verlinkte Onlineshop zeigt beim Abruf überwiegend Merchandising (Bekleidung, Modell-LKW, Sticker usw.), keine belastbare Gebrauchtteile- oder Metallpreisliste. GEBÜHREN: Autoverwertungsseite berechnet Abholtransport nach Aufwand und nennt Preis vorab telefonisch, keine Pauschale veröffentlicht. Shopbeträge sind Waren-Verkaufspreise (AGB inkl. MwSt. zzgl. ggf. Versand), keinesfalls Schrottvergütung. Quelle: https://www.dapa-hst.de/autoverwertung/ ; https://www.dapa-hst.de/autoankauf/ ; https://www.dapa-hst.de/shop/ ; https://www.dapa-hst.de/agb/]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Kranichblick 32 stammt von der Betreiber-Standortseite, während ältere Verzeichnisse Hausnummer 30 nennen. Hausnummer weiterhin nicht ins Frontmatter übernommen; Geokodierung bis zum unabhängigen Abgleich ausgesetzt. In dieser Bearbeitung erfolgten weder Geocoding noch Datenbankzugriffe; Quelle: https://www.dapa-hst.de/locations/duvendiek/ ; https://www.dapa-hst.de/impressum/ (Abrufe 05.10.2026; die Quellen belegen nicht unabhängig die Hausnummer).]
