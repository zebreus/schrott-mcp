---
slug: hh-harburg-kern-heinz-terboven
name: Heinz Terboven
trader_type: schrotthaendler
state: HH
city: Harburg-Kern
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: Ankauf anzufragen
provenance_origin: table
---

# Heinz Terboven

## Überblick

Prüffall ohne verifizierten Betreiberbeleg. Zwei 11880-Profile widersprechen sich zur Anschrift (Eißendorfer Str. 41 vs. 43), Telefonnummer und Schließungsangabe; die Einträge stammen aus demselben Aggregator und die NorthData-Suche ergab keinen eindeutig passenden Registertreffer. Weder Adresse/Telefon noch Schließung in die Frontmatter übernehmen; `status: pruefung` bleibt.

**Preise (Ankauf/Verkauf/Gebühren):** kein belastbarer Ankaufskurs, Verkaufspreis oder Gebührenbeleg. **Offen:** Identität des Händlers, richtige Hausnummer, Kontakt und Fortbestand direkt verifizieren. Quellen: die widersprüchlichen, nicht unabhängigen Profile unter https://www.11880.com/suche/heinz-terboven/hamburg und https://www.11880.com/branchenbuch/hamburg/120674719B111438881/heinz-terboven-schrotthandel.html; siehe Recherche 03.–04.10.2026.

## Timeline

### Recherche 04.10.2026

- [Recherche 04.10.2026: Keine neue nicht-aggregierte Quelle für Heinz Terboven in Hamburg-Harburg gefunden. Die verfügbaren Treffer bleiben widersprüchliche Verzeichnisprofile (Eißendorfer Str. 41 vs. 43, unterschiedliche Telefonnummern und widersprüchliche Schließungsangaben); kein passender Registerbeleg und kein Betreiberauftritt verifiziert. Adresse, Telefon und Schließung daher nicht übernommen, bestehende leere Felder/Prüfstatus unverändert; Quelle(n): https://www.11880.com/suche/heinz-terboven/hamburg ; https://www.11880.com/branchenbuch/hamburg/120674719B111438881/heinz-terboven-schrotthandel.html ; https://www.11880.com/branchenbuch/hamburg/251092496B111438881/heinz-terboven-schrotthandel.html ; https://www.northdata.de/Heinz+Terboven,+Hamburg (ohne passenden Registerbeleg)]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (FE/NE lt. Aggregator)

### Recherche 01.10.2026

- [Recherche 01.10.2026: miss — nur Aggregator-Leads; kein Betreiber-Impressum/Registerbeleg; Statusfeld unveraendert; Quelle(n): Aggregator-Leads]

### Recherche 03.10.2026

- [Recherche 03.10.2026: Zwei 11880-Profile zu Heinz Terboven/Schrotthandel widersprechen sich: eines führt „geschlossen“, Telefon 040 776019 und FAQ-Adresse Eißendorfer Str. 41; das andere ist als Heinz Terboven Schrotthandel mit Mobilnummer 0172 4533204, ohne Adresse, aktualisiert. Im FAQ des zweiten steht stattdessen Eißendorfer Str. 43. Ein Profil verknüpft außerdem klempnerei-gen.de, ohne unabhängigen Beleg für die Zuordnung. Das sind Leads desselben Aggregators; NorthData-Suche lieferte keinen passenden belastbaren Registerbeleg. Deshalb weder Adresse/Telefon/Website noch Schließung übernommen; bisheriges aktiv wegen ungeklärter Identität auf pruefung korrigiert, Website-Status unbekannt. Kein Schließungsbeleg. Quelle(n): https://www.11880.com/suche/heinz-terboven/hamburg; https://www.11880.com/suche/schrotthandel/hamburg-harburg; https://www.11880.com/branchenbuch/hamburg/060440092B27522157/heinz-terboven-schrott-und-altmetalle.html; https://www.11880.com/branchenbuch/hamburg/120674719B111438881/heinz-terboven-schrotthandel.html; https://www.northdata.de/Heinz+Terboven,+Hamburg]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Keine Koordinate gesetzt — Klärfall dokumentiert.** Die beiden vorliegenden
  Verzeichnisprofile zu „Heinz Terboven" **widersprechen sich** in der Hausnummer: das eine führt **Eißendorfer Str.
  41** (Telefon 040 776019, FAQ-Adresse, mit Schließungsvermerk), das andere **Eißendorfer Str. 43** (Mobilnummer 0172
  4533204, ohne Adresse) — beide aus demselben Aggregator (11880), also **keine** zwei unabhängigen Quellen; die
  NorthData-Suche ergab keinen eindeutig passenden Registertreffer, und die verknüpfte Domain klempnerei-gen.de ist ohne
  unabhängigen Beleg. Weder Adresse noch Telefonnummer noch Fortbestand sind damit belegt; das Dossier hat sie bewusst
  nicht in die Frontmatter übernommen. Eine OSM-Stichprobe (api.openstreetmap.org/api/0.6/map.json, 0,01-Grad-Kacheln)
  im Umfeld der Dossier-Adresse **Eißendorfer Straße 41/43** (Kachelraster 53.4550-53.4650 / 9.9950-10.0150) ergab
  **kein** Objekt mit `addr:housenumber`=41 bzw. 43 und `addr:street`=Eißendorfer Straße und **kein** Objekt mit
  Namens-, Betreiber- oder Branchenbezug zu „Terboven" oder zu einem Schrotthandel an dieser Straße — die
  Dossier-Hausnummern sind in OSM **nicht nachgewiesen**; die Aussage ist als Stichprobe qualifiziert, weil die OSM-API
  in dieser Welle stark gedrosselt war (HTTP 429 bei Kachelabrufen). **Konsequenz:** Solange der Adresswiderspruch 41/43
  besteht und keine Betreiber-/Registerquelle vorliegt, ist die Zwei-Quellen-Regel nicht erfüllt; eine Koordinate müsste
  zwischen zwei widersprüchlichen Hausnummern raten. `status: pruefung`, `street`/`postcode` (beide leer) und alle
  übrigen Frontmatter-Felder bleiben unverändert; es wird **keine** Schließung behauptet, obwohl ein Aggregatorprofil
  eine Schließung andeutet. **Nächster Schritt für den Owner:** Direktkontakt/Einwohnermeldung oder Gewerbeauskunft zur
  richtigen Hausnummer am Eißendorfer Straße; danach erneute OSM-Hausnummerensuche. Quelle(n):
  https://www.11880.com/suche/heinz-terboven/hamburg ;
  https://www.11880.com/branchenbuch/hamburg/120674719B111438881/heinz-terboven-schrotthandel.html ;
  https://www.11880.com/branchenbuch/hamburg/251092496B111438881/heinz-terboven-schrotthandel.html (Aggregator-Leads,
  nicht unabhängig) ; https://www.northdata.de/Heinz+Terboven,+Hamburg ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9950,53.4550,10.0150,53.4650 (Abruf 03.11.2026).]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Keine Koordinate gesetzt - die Negativaussage der Vorwelle war ein Fensterfehler; beide Hausnummern existieren in
  OSM, tragen aber fremde Nutzungen.** **Fehler der Vorwelle:** das am 03.11.2026 abgetastete Raster 53.4550-53.4650
  / 9.9950-10.0150 liegt rund **2,3 km östlich** der Eißendorfer Straße (53.4597 N / 9.9730 E); die Aussage „kein
  Objekt mit `addr:housenumber`=41 bzw. 43" war ein **Fensterfehler**, kein Datenbefund. **Korrigiertes Fenster
  (live, vollständig): 9.9600/53.4490 bis 9.9800/53.4690** (Hamburg-Harburg, Eißendorfer Straße), **4 Kacheln von je
  0,01 Grad**, **0 Kachel-Fehler** (api.openstreetmap.org/api/0.6/map.json, Abruf 04.11.2026). Das Fenster enthält
  die gesuchte Straße **nachweislich**: die **Eißendorfer Straße** ist mit **18** Straßenobjekten darin erfasst.
  **Census im Fenster:** **138** Adressobjekte mit `addr:street`=Eißendorfer Straße und `addr:postcode`=21073 mit
  **129** verschiedenen Hausnummern - **41** und **43** sind beide belegt. **Quelle 1 (OSM, live geladen
  04.11.2026):** **node/2518031220** trägt `addr:housenumber`=**41**, `addr:street`=Eißendorfer Straße,
  `addr:postcode`=21073, `craft`=**plumber**, `name`=**„Klempnerei-Genossenschaft"**, `operator`=**Klempnerei-,
  Installations- und Rohrleitungsbau-Genossenschaft eG**, `phone`=+49 40 772531,
  `website`=https://www.klempnerei-gen.de/, `opening_hours`=Mo-Th 07:00-16:00; Fr 07:00-12:30,
  `check_date`=2026-10-29, Version 7 vom 29.10.2026, bei **53.4596896 / 9.9732282**; **way/53011904** trägt
  `addr:housenumber`=**43**, `addr:street`=Eißendorfer Straße, `addr:postcode`=21073, `building`=apartments,
  `building:levels`=4, `roof:levels`=1, Version 6 vom 28.03.2021, Mittel der **6** eindeutigen Knoten **53.4596697 /
  9.9729578**, BBox 53.4595786/9.9727870 bis 53.4597300/9.9731450. **Quelle 2 (Betreiber-/Register, weiterhin nicht
  erfüllt):** Die beiden vorliegenden Verzeichnisprofile zu „Heinz Terboven" **widersprechen sich in der Hausnummer**
  (Eißendorfer Str. 41 mit Telefon 040 776019 und Schließungsvermerk gegen Eißendorfer Str. 43 mit Mobilnummer 0172
  4533204), stammen aber **aus demselben Aggregator** (11880) und sind damit **keine** zwei unabhängigen Quellen; die
  NorthData-Suche ergab keinen eindeutig passenden Registertreffer, und die verknüpfte Domain klempnerei-gen.de ist
  ohne unabhängigen Beleg. **Warum keine Koordinate:** (a) der Adresswiderspruch 41/43 bleibt ungelöst, (b) in OSM
  trägt **41** ein **Klempnerei-Handwerk** (`craft`=plumber, `operator`=Klempnerei-Genossenschaft eG) und **43** ein
  **Wohngebäude** - **keines** der beiden Objekte hat einen Bezug zu „Heinz Terboven" oder zu einem Schrotthandel,
  (c) ohne Betreiber-/Registerquelle ist die Zwei-Quellen-Regel nicht erfüllt, eine Koordinate müsste zwischen zwei
  widersprüchlichen Hausnummern raten und würde zudem eine fremde Nutzung als Dossier-Standort ausgeben.
  **Negativaussage, qualifiziert:** Eine Suche nach „Terboven" und „Schrott" über alle Tag-Werte des Fensters findet
  **kein** Objekt mit Bezug zum Dossier-Händler. **Reichweite / Fehlbelegklausel:** Die in OSM erfasste Nutzung an 41
  und 43 ist eine **Standangabe** und **kein** Beleg gegen einen (heute nicht mehr belegten) Schrotthandel ebendort;
  umgekehrt belegt eine vorhandene Hausnummer **keinen** Betrieb. `status: pruefung`, `street`/`postcode` (beide
  leer) und alle übrigen Frontmatter-Felder bleiben unverändert; es wird **keine** Schließung behauptet, obwohl ein
  Aggregatorprofil eine Schließung andeutet. **Nächster Schritt für den Owner:** Direktkontakt, Einwohnermeldung oder
  Gewerbeauskunft zur richtigen Hausnummer an der Eißendorfer Straße; danach erneute OSM-Hausnummerensuche.
  Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9600,53.4490,9.9800,53.4690 ;
  https://api.openstreetmap.org/api/0.6/node/2518031220.json ;
  https://api.openstreetmap.org/api/0.6/way/53011904/full.json ; https://www.11880.com/suche/heinz-terboven/hamburg ;
  https://www.11880.com/branchenbuch/hamburg/120674719B111438881/heinz-terboven-schrotthandel.html ;
  https://www.11880.com/branchenbuch/hamburg/251092496B111438881/heinz-terboven-schrotthandel.html (Aggregator-Leads,
  nicht unabhängig) ; https://www.northdata.de/Heinz+Terboven,+Hamburg (Abruf 04.11.2026).]
