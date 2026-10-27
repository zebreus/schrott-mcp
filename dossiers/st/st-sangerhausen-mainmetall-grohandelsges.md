---
slug: st-sangerhausen-mainmetall-grohandelsges
clear_fields: geo
name: Mainmetall Großhandelsges.
trader_type: sonstige
state: ST
city: Sangerhausen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://www.mainmetall.de/
website_status: aktiv
status: geschlossen
description: Sanitär-, Heizungs- und Baustoff-Großhandel (B2B, MAINMETALL GmbH, 25 Standorte);
  reiner Haustechnik-Handel ohne Schrott- oder Metallankauf, kein Schrotthandel am Standort
  Sangerhausen belegt.
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: nein
provenance_origin: table
---

# Mainmetall Großhandelsges.

## Überblick

Die MAINMETALL GmbH ist ein Sanitär-, Heizungs-, Bad- und Baustoff-Großhändler („Partner des
Fachhandwerks seit 1948“, über 38.000 Artikel im Sortiment, 25 Standorte von Bürgstadt bis
Potsdam). Ein Schrott- oder Metallankauf ist weder auf der Startseite noch im Sortiment
(Sanitär, Heizung, Installation, Bedachung, Klima und Lüftung, Werkzeug, Fliesen) erkennbar;
die Live-Seite bestätigt das am 27.10.2026 erneut. Die Zeile war nie ein Schrotthändler und
wird deshalb als **geschlossen** geführt: Der Eintrag wird nicht mehr als Schrottannahme
veröffentlicht. Eine rechtliche Löschung des Unternehmens ist damit nicht behauptet, und
Ankauf-, Verkaufs- oder Gebührenpreise werden für diese Zeile nicht geführt.

**Preise:** Ankauf — keine Mainmetall-Ankaufskurse, das Unternehmen handelt Sanitär- und
Heizungsartikel; Verkauf — keine Schrott-Verkaufspreise; Gebühren — keine Tarife belegt.

## Timeline

### Owner-Korrektur 27.10.2026: verwaiste Zeile ohne Dossier als geschlossen wiederhergestellt

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile existiert, hat aber seit dem
  03.10.2026 kein Dossier mehr: Der Commit `599520de` („Mainmetall-Dossier entfernen:
  Haustechnikware allein ist keine Schrottrelevanz“) hat `dossiers/st/st-sangerhausen-mainmetall-grohandelsges.md`
  gelöscht, gestützt auf das Feedback 4299 vom 03.10.2026 („Website ist Sanitär-/Heizungs-Großhandel,
  kein Metallhändler“, Severity critical). Der Seed kann keine Zeile löschen, die
  Produktionszeile blieb deshalb als offener Händler mit `status: pruefung`, Website und dem
  Stadtpunkt `51.4722, 11.2953` (Sangerhausen) stehen und wurde seit dem 02.10.2026 nicht mehr
  angefasst. Das widerspricht der Seed-Regel „The seed never deletes: closures arrive as
  `status: geschlossen`“. Das Dossier wird deshalb als Minimaldatensatz wiederhergestellt:
  `status: geschlossen`, `trader_type: sonstige`, `clear_fields: geo` für den
  Stadt-Platzhalter, korrigierte Beschreibung. Die Live-Seite vom 27.10.2026 bestätigt den
  reinen Haustechnik-Handel („Willkommen in unserer Welt der Haustechnik!“, 38.000 Artikel,
  25 Standorte, kein Schrott- oder Metallsortiment). Die Schließung betrifft den
  Schrott-Eintrag, nicht das Unternehmen; eine Behauptung über eine rechtliche Löschung,
  über Annahme von Materialien oder über Preise wird nicht erhoben. Quellen:
  https://www.mainmetall.de/ ; Feedback 4299 (internal.db, 03.10.2026); Commit `599520de`;
  read-only `public.db` (27.10.2026); README.md, Seed-Statusregel; docs/trader-seed.md.]

### Recherche 02.10.2026 (übernommen aus der Produktions-Notiz)

- [Recherche 02.10.2026: Betreiber verifiziert, aber kein Schrott- oder Metallankauf:
  MAINMETALL GmbH (Sanitär/Heizung/Bad-Großhandel, 25 Standorte unter anderem Sangerhausen laut
  Standortliste), reiner Haustechnik-Großhandel B2B. Die Seed-Adresse „An der Stollenmühle 17“
  war auf der Betreiberseite nicht verifiziert, die Adresszeile bleibt deshalb leer; Website,
  Website-Status und Beschreibung waren gefüllt, der Status blieb zu diesem Zeitpunkt `pruefung`,
  weil das Ankauf-Flag offen war. Quellen: https://www.mainmetall.de/ ;
  https://www.mainmetall.de/kontakt/]
