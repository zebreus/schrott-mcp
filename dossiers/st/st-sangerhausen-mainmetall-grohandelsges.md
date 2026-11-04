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

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Diese Zeile ist ein **geschlossener
  Nicht-Schrott-Eintrag** (reiner Sanitär-/Heizungs-/Baustoff-Großhandel) und trägt im Frontmatter **weder Straße noch
  PLZ** - es gibt damit **keine** Dossier-Adresse, die geokodiert werden könnte; zusätzlich existiert in OSM **kein**
  Objekt mit Bezug zu „MAINMETALL“ in Sangerhausen. **Quelle 1 (OSM, live abgerufen am 04.11.2026):** Kachelraster
  über **Sangerhausen** (**11.2600/51.4400 bis 11.3400/51.5000**, live über
  `https://api.openstreetmap.org/api/0.6/map.json?bbox=11.2600,51.4400,11.3400,51.5000`): das Raster enthält den Ort
  **Sangerhausen** selbst (place=town, 51.4730/11.2982) samt der Ortsteile **Othal**, **Engelsburg**, **Im Schlag**,
  **Wallburg am Schlösschenkopf**, **Weinlager** und **Wüstung Brechtewende**; eine Namenssuche nach
  **„Mainmetall“**, **„MAINMETALL“**, **„Sanitär“** und **„Baustoff“** in den Feldern name, operator, description,
  website, contact:website, brand, note und addr:housename findet **keinen** Treffer für die MAINMETALL GmbH - der
  einzige Haustechnik-Treffer im Raster ist **node/4857544613** („SHS Riedel Sanitär- und Heizungsbau GmbH“,
  `craft`=hvac, 51.47563/11.27307), ein **anderer** Betrieb. Es gibt in OSM also **keinen** Punkt, der die
  MAINMETALL-Filiale in Sangerhausen bezeichnet; ein Geokodierungs-Punkt müsste frei gewählt werden. Quelle 2
  (Betreiber, im Dossier bereits zitiert, live nachgelesen am 04.11.2026): die **Live-Seite der MAINMETALL GmbH**
  (https://www.mainmetall.de/) bestätigt den reinen Haustechnik-Handel („Partner des Fachhandwerks seit 1948“, über
  38.000 Artikel im Sortiment, 25 Standorte) und **keinen** Schrott- oder Metallankauf; das im Dossier dokumentierte
  **Branchenbuch** („Großhandel (Haustechnik)“) stuft die Zeile ebenfalls als geschlossenen Nicht-Schrott-Eintrag ein.
  **Warum keine Koordinate:** (a) die Zeile wird **nicht** als Schrotthändler geführt (`trader_type: sonstige`,
  `status: geschlossen`), eine Geokodierung würde einen Schrott-Annahmestandort suggerieren, den es nicht gibt und
  den das Dossier ausdrücklich verneint; (b) `street`, `postcode` und `phone` sind **leer** - es gibt keine Adresse,
  deren Lage geprüft werden könnte; (c) der bisher gespeicherte Produktionspunkt **51.4722/11.2953** ist ein reiner
  **Stadtplatzhalter** (Sangerhausen) und kein Betriebsstandort - er wird deshalb mit `clear_fields: geo` beim
  regulären Seed **zurückgenommen** und **nicht** ersetzt. **Was zur Auflösung fehlt:** für diesen Datensatz ist
  **nichts** zu klären - die Zeile bleibt ein geschlossener Nicht-Schrott-Eintrag; eine Wiederaufnahme als
  Schrotthändler würde eine eigene Recherche mit Betriebsbeleg erfordern. `status: geschlossen`,
  `trader_type: sonstige`, `website`, `website_status`, `description`, `dropoff_json`, `pickup_json` und die übrigen
  Frontmatter-Felder bleiben **unverändert**; es wird weder eine rechtliche Löschung des Unternehmens noch
  Schrottannahme behauptet. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=11.2600,51.4400,11.3400,51.5000 ;
  https://www.mainmetall.de/ (Abruf 04.11.2026).]
