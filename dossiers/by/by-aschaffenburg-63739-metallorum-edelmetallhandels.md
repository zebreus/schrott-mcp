---
slug: by-aschaffenburg-63739-metallorum-edelmetallhandels
name: Metallorum Edelmetallhandels GmbH
trader_type: metallhaendler
state: BY
city: Aschaffenburg
street: Weißenburger Str. 18
postcode: '63739'
phone: 06021 4542399
email: info@edelmetallshop-aschaffenburg.de
opening_hours: Mo-Fr 9:30-13:00 und 14:00-18:00, Sa (1. und 3. im Monat) 9:30-13:30
website: https://metallorum.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Mömlingen-70km (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Metallorum Edelmetallhandels GmbH

## Überblick

Aschaffenburger Filiale der Metallorum Edelmetallhandels GmbH mit An- und Verkauf von Anlagebarren und -münzen sowie Altgold-/Altsilberankauf. Die Produktpreisliste ist nicht mit dem unverbindlichen Legierungs-Ankaufsrechner gleichzusetzen. Beim Ankauf zeigt die interaktive Preisliste ausschließlich die als „Preis (netto)“ bezeichnete Spalte und das Gewicht; MwSt. und Brutto bleiben im HTML, werden aber ausdrücklich ausgeblendet. Für allgemein veröffentlichte Produktankaufspreise deshalb den angezeigten Betrag übernehmen, nicht die versteckte steuerlich hochgerechnete Bruttospalte. Eine besondere Auszahlung inklusive Umsatzsteuer für Gewerbekunden ist damit nicht zugesagt; deren konkrete Abrechnung muss der Betreiber bestätigen.

## Timeline

### Preisaudit 08.10.2026 (Feedback #4785)

- [Validierung 08.10.2026: Neuer Regressionstest vor Korrektur reproduziert den Fehler als 1,86202 statt 1,56472 EUR/g; nach Korrektur alle 4 Metallorum-Tests und alle 8 Seed-Tests bestanden. Datenbankfreier Live-Lauf des bestehenden Beispiels live_handlers mit dem Filialslug liefert HTTP 200, 44 Ankaufprodukte (33 Gold, 11 Silber), 44 ausdrücklich übersprungene Verkaufzeilen und unveränderten Aschaffenburger Kontaktblock. Quelle(n): lokale Testläufe cargo test -p schrott-mcp-ingestion metallorum und cargo test -p schrott-mcp-ingestion seed; https://metallorum.de/unser-service/preislisten/; https://metallorum.de/verkaufsstellen/aschaffenburg/]
- [Korrektur 08.10.2026: Die Behauptung, Verkauf und Ankauf zeigten jeweils Netto/MwSt./Brutto als relevante Preiswerte, trifft nur auf das statische HTML zu, nicht auf die interaktive Ankaufansicht. Beide Richtungen sind durch data-direction eindeutig getrennt; der Tabellen-Handler filterte Ankauf korrekt, nahm aber cells[4] (Brutto). Die von der echten Seite eingebundene widget.js vermerkt „Bei Ankauf sind MwSt. und Brutto-Preis nicht relevant -> Spalten ausblenden“ und setzt hwt-mp-dir-ankauf. Die zugehörige widget.css setzt für .hwt-mp-col-vat und .hwt-mp-col-price-gross in dieser Ankaufklasse display: none und kommentiert „nur Netto + Gewicht anzeigen“. Die verbleibende Spalte Preis (netto), cells[2], ist daher der vom Betreiber präsentierte Ankaufspreis. Handler eng auf diese Spalte korrigiert, ohne Steuerumrechnung oder Spotpreis-Heuristik. Quelle(n): https://metallorum.de/unser-service/preislisten/; https://metallorum.de/wp-content/plugins/hwt-metallpreise-tabelle-elementor/assets/js/widget.js?ver=1.0.1; https://metallorum.de/wp-content/plugins/hwt-metallpreise-tabelle-elementor/assets/css/widget.css?ver=1.0.1]
- [Recherche 08.10.2026: Live-HTML beim Audit: 1kg Silberbarren Ankauf 1.564,72 EUR netto, 19 % (297,30 EUR), versteckt 1.862,02 EUR brutto bei 1.000,0000 g; Maple Leaf 1 Unze 19 %: 50,78 EUR netto, versteckt 60,43 EUR brutto bei 31,1000 g; Maple Leaf Diff.-besteuert: 50,79 EUR netto, 7 % (3,55 EUR), versteckt 54,34 EUR brutto. Korrekte Übernahme entsprechend 1,56472 EUR/g, 50,78/31,1 EUR/g und 50,79/31,1 EUR/g. Die früher gemeldeten 1.885,60/1.584,54 EUR bzw. 61,21/51,44 EUR sind wegen laufender Kursänderungen keine festen Test-Sollwerte. 19-%- und Diff.-Varianten bleiben getrennt; Regressionstest erfasst beide sowie fehlenden/Null-Netto ohne Brutto-Fallback. Gold mit 0 % bleibt numerisch unverändert. Quelle(n): https://metallorum.de/unser-service/preislisten/]
- [Recherche 08.10.2026: Gegenbeleg ernst genommen: Die Preislistenseite erklärt Ankauf als „welchen Preis wir Ihnen aktuell für ein Produkt zahlen“; die Service-Seite bezeichnet Preise als Orientierung für „Auszahlungsbeträge beim Verkauf“. Das belegt Produktankauf und Auszahlung, aber nicht die im HTML versteckten Steueraufschläge. Die Filialseite spricht Privat- und Geschäftskunden an, enthält jedoch keine unterschiedliche Ankaufabrechnung. AGB 1.1 betrifft Metallorum als Verkäufer seiner Online-Shop-Waren; die Gesamtpreis-/USt.-Aussage in AGB 4.1 darf nicht auf Metallorum als Käufer übertragen werden. Allgemein gilt: Ein echter Privatverkauf außerhalb unternehmerischer Tätigkeit löst nicht allein durch Silberverkauf Umsatzsteuer nach § 1 Abs. 1 Nr. 1 UStG aus; der Händler kann gleichwohl einen beliebigen Gesamtankaufspreis vereinbaren. § 25a Abs. 3/5 betrifft bei zulässiger Differenzbesteuerung die Wiederverkäufermarge, nicht pauschal 7 % auf den vollständigen Ankauf eines Privaten. Gewerbe ist keine einheitliche Steuerkategorie; steuerpflichtige Lieferung, Steuerbefreiung und ggf. § 13b sind fallabhängig. Daraus folgt keine frei erfundene Regel „Privat erhält immer netto, Gewerbe immer brutto“. Gesichert ist allein die veröffentlichte Ankaufansicht; konkrete Gewerbe-USt.-Abrechnung und verbindliche individuelle Auszahlung bleiben beim Betreiber zu klären. Über-Spot-Werte allein waren ausdrücklich kein Fehlerbeweis. Quelle(n): https://metallorum.de/unser-service/preislisten/; https://metallorum.de/unser-service/edelmetallhandel/; https://metallorum.de/verkaufsstellen/aschaffenburg/; https://metallorum.de/agb/; https://www.gesetze-im-internet.de/ustg_1980/__1.html; https://www.gesetze-im-internet.de/ustg_1980/__25a.html; https://www.gesetze-im-internet.de/ustg_1980/__13b.html]
- [Recherche 08.10.2026: Ankaufsbedingungen: Der separate Legierungsrechner dient laut Betreiber nur zur Orientierung; endgültiger Ankaufspreis vor Ort nach aktuellen Preisen, Echtheits- und Feingehaltsprüfung. FAQ im Original-HTML bestätigt kostenlose/unverbindliche Prüfung, Teilnahme des Kunden möglich, Termin empfohlen aber nicht zwingend, Annahme Gold/Silber/Platin/Palladium als Schmuck, Münzen, Barren, Zahn-/Bruchgold. Die statische Rechneransicht zeigt 0,00 EUR und eine Preis-API-Fehlermeldung; daraus keine Materialpreise ableiten. Der Produktlisten-Spaltenfix ist deshalb keine Zusicherung für beliebiges Altsilber oder jede Kundenkonstellation. Adresse/Kontakte/Öffnungszeiten unverändert; keine Produktionsschreibzugriffe, kein Commit oder Deployment. Quelle(n): https://metallorum.de/unser-service/ankaufsrechner/; https://metallorum.de/verkaufsstellen/aschaffenburg/]

### Korrektur 04.10.2026 (Feedback #4762)

- [Korrektur 04.10.2026: Die getrennte PLZ ist bereits korrekt als postcode 63739 eingetragen; die Betreiber-Verkaufsstellenseite bestätigt Weißenburger Str. 18, 63739 Aschaffenburg. city „Aschaffenburg 63739“ ist nicht leer und bleibt nach Fill-only-Regel unangetastet. website_status auf aktiv gesetzt, da die hinterlegte Betreiber-Domain und die Standortseite live erreichbar sind. Keine Adressänderung und keine Neu-Geocodierung; Quelle(n): https://metallorum.de/verkaufsstellen/edelmetallshops; https://metallorum.de/ueber-uns/kontakt/]

### Korrektur 05.10.2026 (Feedback #4762)

- [Korrektur 05.10.2026: city von „Aschaffenburg 63739“ zu „Aschaffenburg“ normalisiert; postcode 63739 bleibt separat. Die frühere Notiz berief sich auf eine Fill-only-Regel, die das README nicht vorgibt: belegte nichtleere Felder dürfen mit dokumentierter Evidenz korrigiert werden. Die aktuelle Betreiber-Verkaufsstellenseite nennt Weißenburger Str. 18, 63739 Aschaffenburg; Impressum und HRB 13597 ordnen die Filiale der Metallorum Edelmetallhandels GmbH zu. Straße, Telefonnummer, E-Mail, Öffnungszeiten und Status bleiben unverändert. Die Adresse selbst änderte sich nicht, daher keine neue Geocodierung. Quelle(n): https://metallorum.de/verkaufsstellen/edelmetallshops; https://metallorum.de/impressum/; https://www.northdata.de/Metallorum%20Edelmetallhandels%20GmbH,%20Unterpleichfeld/HRB%2013597]

### Nachprüfung 06.10.2026 (Feedback #4762)

- [Nachprüfung 06.10.2026: Die aktuelle Betreiber-Standortseite für Aschaffenburg bestätigt separat Weißenburger Str. 18, 63739 Aschaffenburg, die Öffnungszeiten (Mo–Fr 09:30–13:00 und 14:00–18:00; 1. und 3. Samstag 09:30–13:30), Telefon 06021 4542399 und info@edelmetallshop-aschaffenburg.de. Die Shop-Übersicht führt Aschaffenburg als eigenen Standort; die Einträge in Frontmatter (city Aschaffenburg, postcode 63739, Straße, Telefon, E-Mail und Öffnungszeiten) sind damit weiterhin konsistent. Keine Änderung an Produktionsdaten oder Geokoordinaten. Quelle(n): https://metallorum.de/verkaufsstellen/aschaffenburg/; https://metallorum.de/verkaufsstellen/edelmetallshops/]

### Ergänzende Identitätsprüfung 06.10.2026 (Feedback #4762)

- [Nachprüfung 06.10.2026: Die aktuelle Register-Dossieransicht identifiziert die Metallorum Edelmetallhandels GmbH unter HRB 13597, Amtsgericht Würzburg, mit Sitz An der Windmühle 6, 97294 Unterpleichfeld und Edelmetallhandel als Unternehmensgegenstand; sie führt als frühere Firmierung „Metallorum GmbH“. Die Betreiberseite weist Aschaffenburg separat als eigenen Edelmetallshop aus und nennt dort Weißenburger Str. 18, 63739 Aschaffenburg sowie die im Frontmatter geführten Öffnungszeiten, Telefonnummer und E-Mail. Damit ist die Abweichung zwischen Gesellschaftssitz und Aschaffenburger Filiale erklärt; die lokalen Frontmatter-Felder beschreiben die Filiale korrekt und werden nicht durch die Anschrift des Gesellschaftssitzes ersetzt. Keine Änderung an Adresse, Kontakten oder Geokoordinaten. Quelle(n): https://metallorum.de/verkaufsstellen/aschaffenburg/; https://metallorum.de/verkaufsstellen/edelmetallshops/; https://metallorum.de/impressum/; https://www.northdata.de/Metallorum%20Edelmetallhandels%20GmbH,%20Unterpleichfeld/HRB%2013597]

### Importiert (Seed-Stand 2026-09-30)

- Gold/Silber, Münzen/Barren, Altgold — https://metallorum.de/unser-service/preislisten/ + /ankaufsrechner/ → prüfen
- Adresse: Aschaffenburg 63739, Weißenburger Str.
- Adressbeleg: https://metallorum.de, https://metallorum.de/ueber-uns/kontakt/
