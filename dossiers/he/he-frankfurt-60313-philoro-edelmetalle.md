---
slug: he-frankfurt-60313-philoro-edelmetalle
name: philoro EDELMETALLE GmbH
trader_type: sonstige
state: HE
city: Frankfurt 60313
street: Schillerstraße 15-17
postcode: '60313'
phone: +49 69 153257182
email: frankfurt@philoro.de
opening_hours: Mo-Fr 10:00-18:00, Sa 10:00-15:00
website: https://www.philoro.de
website_status: ''
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4b – Kandidaten-Sweep (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# philoro EDELMETALLE GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026 (Feedback-Triage 2918)

- [Recherche 01.10.2026: Feedback 2918 berechtigt — Betreiber-Filialdetailseite (philoro.de/filialen/frankfurt, Abruf 01.10.2026): „philoro EDELMETALLE GmbH, Schillerstraße 15-17, 60313 Frankfurt, Tel. +49 69 153257182, frankfurt@philoro.de, Mo-Fr 10:00-18:00, Sa 10:00-15:00". Owner-Direktive greift (verifiziertes Betreiber-Impressum: philoro EDELMETALLE GmbH, Barfußgässchen 11, 04109 Leipzig, HRB 27556 AG Leipzig, GF Steffen Horn/Tobias Kascha — HR-kongruent, Filialseite per-site). street/postcode/phone/email/opening_hours gefüllt; city-Feld unverändert (historisch mit PLZ). Hinweis: Koordinaten neu zu geocodieren. Quelle(n): philoro.de/filialen/frankfurt + /filialen + /impressum]

### Importiert (Seed-Stand 2026-09-30)

- Edelmetall: Zahngold-/Altgold-Ankauf, Filiale FFM. PREISLISTE: https://www.philoro.de/preisliste/alle
- Adresse: Frankfurt 60313

### Recherche 01.11.2026

- [Recherche 01.11.2026: **Preisliste live bestätigt, aber ohne Preis-Datum — nach Repo-Standard nicht als datierter Ankaufpreis verwendbar.** `https://philoro.de/preisliste/alle` liefert am 01.11.2026 07:09 UTC HTTP 200 (2.582.010 Bytes, serverseitig gerendert) mit **sechs** Preistabellen — die Spalten lauten durchgehend „Artikel | Name | Feingewicht | Verkaufspreis | Kaufpreis", die Abschnitte heißen „Goldmünzen" (54 Datenzeilen), „Goldbarren" (32), „Silbermünzen" (26), „Silberbarren" (15), „Platin & Palladium" (12) und „Numismatik" (60). Wörtliche Preisfragmente: „1991 | Gold Philharmoniker 1 oz - 2026 | 31.10 g | 3.454,94 € | 3.625,81 €", „2513 | Goldbarren 250 g gegossen - philoro | 250.00 g | 27.329,26 € | 28.544,89 €", „5081 | Platinbarren 1 oz diverse Hersteller | 31.10 g | 1.258,63 € | 1.737,14 €", „5087 | Palladiumbarren 1 oz diverse Hersteller | 31.10 g | 846,45 € | 1.240,50 €". PREISART: die Spalte „Kaufpreis" ist der Ankauf von **Edelmetall-Handelsware** (Münzen, Barren) durch philoro — **kein** Ankauf von Schrott; die Werte sind ausdrücklich kursgetrieben: „Die angegebenen Preise umfassen sowohl den Kauf- als auch den Verkaufspreis. Die Werte werden basierend auf dem Gewicht in Unzen oder Gramm und dem aktuellen Edelmetallkurs berechnet." — damit **Spot-Referenz**, kein Händler-Ankaufpreis für Schrott; Einheit je Zeile ist **€/Stück** (Stückpreis, nicht €/g). DATUM: **im Dokument keines** — keine Zeitstempelzeile, kein `<time>`-Element, kein `datetime`-Attribut; der einzige datenbezogene Satz lautet wörtlich „Die Preise werden alle 3 Minuten aktualisiert, damit Sie stets den tatsächlichen Kauf- und Verkaufspreis sehen können." Der HTTP-Header `Last-Modified: Sat, 31 Oct 2026 14:22:19 GMT` (bei `age: 60472`) ist der **Erzeugungszeitpunkt des ausgelieferten HTML-Dokuments** und damit eine Build-/Cache-Angabe — **kein** Preisdatum und **kein** Gültigkeitszeitraum; die 3-Minuten-Aktualisierung erfolgt clientseitig über den Kursrechner, nicht im Dokument. Eine datierte Preisquelle ist damit nicht auffindbar: die Preis-API `https://api.prod-eu.philoro.ch/api/{prices,price,kurs,rates,quote,metallpreise}` lieferte durchgängig HTTP 404, und `philoro.de/kursrechner` sowie `philoro.de/goldankauf` HTTP 404. STANDORT: die Preisliste ist eine **Konzernliste ohne Standortbezug**; die Frankfurter Filiale `https://philoro.de/filialen/frankfurt` (HTTP 200, 547.820 Bytes) nennt „Schillerstraße 15-17, 60313 Frankfurt" und verweist auf die Preisliste, führt aber keine eigenen Werte. Quelle(n): https://philoro.de/preisliste/alle ; https://philoro.de/filialen/frankfurt (Abruf 01.11.2026 07:09 UTC)]
