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
