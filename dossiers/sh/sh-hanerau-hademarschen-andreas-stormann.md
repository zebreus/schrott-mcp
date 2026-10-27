---
slug: sh-hanerau-hademarschen-andreas-stormann
name: Andreas Störmann
trader_type: schrotthaendler
state: SH
city: Hanerau-Hademarschen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Schrotthandel/Altmetallhandel in Hanerau-Hademarschen, in Branchenverzeichnissen unter Andreas Störmann mit Olden-Hop 12 geführt. Für den Betrieb existiert weder eine Betreiber-Website noch ein Registereintrag; das amtliche Register der Entsorgungsfachbetriebe (Stand 11.08.2026) führt für Hanerau-Hademarschen keinen anerkannten Betrieb. Die Annahme von Stahl, Eisen, Alu, Kupfer, Blei und Messing stammt aus dem Seed und ist nicht bestätigt; Preise, Gebühren und Öffnungszeiten sind nicht belegt.'
dropoff_json: ''
clear_fields: 'dropoff_json'
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Andreas Störmann

## Überblick

Andreas Störmann wird in lokalen Verzeichnissen mit Olden-Hop 12 und den Telefonen 04872 2880 / 0173 9108832 geführt; es wurde keine Betreiberseite oder verifizierte aktuelle Preisangabe gefunden, und auch im amtlichen Register der Entsorgungsfachbetriebe (Stand 11.08.2026) gibt es für Hanerau-Hademarschen keinen anerkannten Betrieb. Das Seed nennt Schrottannahme von Stahl/Eisen/Alu/Kupfer/Blei/Messing, aber die Zuordnung und Aktualität dieser Bedingungen ist nicht primär bestätigt; der Status ist deshalb von `aktiv` auf `pruefung` zurückgenommen. **Ankauf:** Arten laut Seed/Verzeichnissen, keine Preise; **Verkauf:** keine Liste; **Gebühren:** keine Angaben.

## Timeline

### Recherche 27.10.2026

- [Recherche 27.10.2026: Amtliches Fachbetrieberegister neu ausgewertet (Fachbetrieberegister der Gemeinsamen Stelle Altfahrzeuge der Länder nach § 7 Abs. 2a AltfahrzeugV und Register der Entsorgungsfachbetriebe nach § 56 KrWG, Auszug mit Stand 11.08.2026, gespiegelt über SchrottRegister): Für **Hanerau-Hademarschen existiert keine Ortsseite** und damit **kein anerkannter Betrieb**; ein „Störmann“-Treffer existiert dort nicht. Efb-Anerkennung ist freiwillig, der Negativbefund beweist also keine Nichtexistenz. Ergänzend: die Branchenbuchsuche „Schrotthandel in Hanerau-Hademarschen“ (11880, 27.10.2026) führt **keinen** Störmann und überhaupt keinen Betrieb in Hanerau-Hademarschen, sondern nur regionale Treffer (Schenefeld, Hohenaspe, Buchholz, Hohn, Windbergen, Rendsburg, Itzehoe, Gnutz).]
- [Recherche 27.10.2026: Beleglage unverändert: Die einzigen Angaben zu „Andreas Störmann“, **Olden-Hop 12, 25557 Hanerau-Hademarschen, 04872 2880, 0173 9108832** stammen von **Cylex** (zuletzt aktualisiert 11.05.2026) und **Stadtbranchenbuch** — zwei Aggregatoren, also **ein** Beleg. OSM/Nominatim kennt **kein** Objekt „Störmann“ in Hanerau-Hademarschen; „12, Olden-Hop“ ist dort ein **unbenanntes** Gebäude (OSM-Way 1040019469, 54.1172949/9.4162934). Es gibt damit weder eine Betreiber-Primärquelle noch einen Registertreffer.]
- [Recherche 27.10.2026: **Status-Korrektur `aktiv` → `pruefung`.** Der bisherige Status `aktiv` stützte sich allein auf den Verzeichnisimport (`provenance_ankauf_raw: ja`, heuristischer Register-Cluster-Pars); nach der Recherche vom 08.10. und der jetzigen Prüfung liegt für den Betrieb **keine** belastbare Betreiber- oder Registerquelle vor, die eine aktive Annahme belegt — das entspricht dem bereits im Bestand gehandhabten Muster „nur ein Verzeichnislead ⇒ `pruefung`“. Anschrift, Telefon, E-Mail und Website bleiben **leer**, weil die Zwei-Quellen-Regel nicht erfüllt ist.]
- [Recherche 27.10.2026: GEO-Abgleich (read-only, 27.10.2026): Der gespeicherte Punkt **54.1333/9.4167** löst per OSM/Nominatim-Rückabfrage auf **„24, Tanneck, Heisern, 25557 Hanerau-Hademarschen“** auf und liegt rund **1,8 km Luftlinie** vom Verzeichnis-Adresslead „Olden-Hop 12“ (54.1172949/9.4162934) entfernt. Da die Zeile keine eigene belegte Ortsangabe trägt, wird die Koordinate **nicht** zurückgenommen und **nicht** umgehängt; die Unstimmigkeit bleibt als Klärfall notiert.]
- [Preise/Ankauf/Verkauf/Gebühren 27.10.2026: **ANKAUF:** die Annahme von Stahlschrott, Eisen, Alu, Kupfer, Blei und Messing stammt aus dem Seed bzw. aus Verzeichnisangaben und ist nicht bestätigt; **keine Zahlenpreise**. **VERKAUF:** keine Verkaufspreisliste. **GEBÜHREN:** keine Gebührenangaben. Quelle(n): https://schrottregister.pages.dev/ort-hanerau-hademarschen (keine Ortsseite) ; https://www.11880.com/suche/schrotthandel/hanerau-hademarschen ; https://web2.cylex.de/firma-home/stoermann-andreas-11434259.html ; https://hanerau-hademarschen.stadtbranchenbuch.com/3262792.html ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=St%C3%B6rmann%2C+Hanerau-Hademarschen ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&zoom=18&lat=54.1333&lon=9.4167&addressdetails=1 ; read-only `public.db` (27.10.2026).]
- [Owner-Hinweis 27.10.2026: Die Produktionszeile trägt zusätzlich `dropoff_json={"allowed":true}` — eine **Annahmezusage**, die durch die Recherche nicht gedeckt ist (nur Aggregator-Leads). Ein Statuswechsel auf `pruefung` zieht diese Zusage nach `docs/trader-seed.md` **nicht** automatisch zurück; eine Rücknahme wäre ein eigener, reviewpflichtiger `clear_fields`-Direktive mit Token `dropoff_json` und damit außerhalb des hier vergebenen Änderungsrahmens. Der Punkt wird dem Owner zur Entscheidung vorgelegt.]
- [Owner-Korrektur 27.10.2026: Der oben offengelegte Fall ist entschieden: Die Produktionszeile trägt `dropoff_json={"allowed":true}` ohne jeden Betreiber- oder Registerbeleg (nur Aggregator-Leads, Negativbefund im amtlichen Register). Eine unbestätigte **Annahmezusage** darf nicht stehen bleiben, solange der Status auf `pruefung` steht — dasselbe Muster ist in `sn-rochlitz-db-recycling-denis-blum`, `sn-mucka-werner-bogner` und `sn-frankenberg-rainer-reinwarth-bde` bereits mit `clear_fields: 'dropoff_json'` zurückgenommen worden. Der Owner gibt deshalb das eng begrenzte Token `dropoff_json` frei; der nächste reguläre Seed leert die Zusage auf `{}` (unbekannt, **nicht** `allowed:false`, keine Schließung). Das GEO-Paar bleibt **unangetastet**: die Zeile trägt keine eigene belegte Ortsangabe, die dem Punkt widerspricht — der Punkt bleibt ungeprüfter Veröffentlichungsblocker. Quelle(n): read-only `public.db` (27.10.2026); `docs/trader-seed.md`; Recherche 27.10.2026 oben.]

### Recherche 08.10.2026

- [Recherche 08.10.2026: Erneute Suche fand keine Betreiberwebsite, amtliche Register-/Kommunalquelle oder belastbares Betreiberprofil. Cylex (zuletzt aktualisiert 11.05.2026) nennt Andreas Störmann, Olden-Hop 12, 25557 Hanerau-Hademarschen, Festnetz 04872 2880 und Mobil 0173 9108832; Stadtbranchenbuch wiederholt Adresse/Festnetz. Das sind Aggregator-Leads und nicht zwei unabhängige Primärquellen; Anschrift/Kontakte bleiben ungefüllt. **Ankauf:** Seed-/Verzeichnisangaben zu Stahlschrott, Eisen, Alu, Kupfer, Blei und Messing ohne Zahlenpreise; **Verkauf:** keine Verkaufspreisliste gefunden; **Gebühren:** keine Gebührenangaben gefunden. Quelle(n): https://web2.cylex.de/firma-home/stoermann-andreas-11434259.html ; https://hanerau-hademarschen.stadtbranchenbuch.com/3262792.html]

### Importiert (Seed-Stand 2026-09-30)

- Schrottannahme Stahl/Eisen/Alu/Kupfer/Blei/Messing
