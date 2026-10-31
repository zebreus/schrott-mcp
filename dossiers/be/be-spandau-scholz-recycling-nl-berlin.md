---
slug: be-spandau-scholz-recycling-nl-berlin
name: Scholz Recycling NL Berlin
trader_type: sonstige
state: BE
city: Spandau
street: 'Spandau, Tiefwerderweg 13,  (gleiche Adresse ELNO-Seed: räumliche Koinzidenz,
  getrennte Firmen)'
postcode: '13597'
phone: ''
email: ''
opening_hours: ''
website: https://www.scholz-recycling.com/
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: A. Kept (Tabelle)
provenance_ankauf_raw: ja (Gewerbe)
provenance_origin: table
---

# Scholz Recycling NL Berlin

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 03.10.2026

- [Recherche 03.10.2026: Partieller Fill ausschließlich der Konzern-Website https://www.scholz-recycling.com/; Root und Impressum erreichbar, website_status tot auf aktiv korrigiert (Website-Zustand, KEIN Nachweis einer aktiven Berlin-Niederlassung). Impressum Scholz Recycling GmbH, Essingen, HRB 733963 AG Ulm, Geschäftsführer Abderrahmane El Aoufir. Starke Betreiber-Einzelquelle, konkrete Zuordnung der historischen Berlin-NL zum heutigen Betreiber bleibt offen. phone von unzulässigem Gedankenstrich auf leeren String korrigiert; keine belegte lokale Nummer. Straße und übrige Seed-Fakten nicht überschrieben; Quelle(n): https://www.scholz-recycling.com/ + https://www.scholz-recycling.com/impressum/]
- [Recherche 03.10.2026: Vollständiger Standortfinder-Datensatz abgerufen und read-only ausgewertet: 184 Standorte, kein Treffer Berlin oder Tiefwerder. Fehlender Eintrag beweist keine Schließung. Konzern-Services, Geschäftsbereiche, Recycling, Zertifikate und Kontakt einzeln vollständig gelesen; keine lokalen Berlin-Fakten daraus abgeleitet. Tiefwerderweg 13 bleibt unbestätigter historischer Verzeichnis-Lead; räumliche Koinzidenz mit ELNO ist kein Identitätsbeleg. Blocker für Filial-Fills: aktueller Standort-/Betreiberbeleg fehlt; Quelle(n): https://www.scholz-recycling.com/standorte/ + https://www.scholz-recycling.com/standorte-json-nicht-loeschen/ + https://www.scholz-recycling.com/kontakt/ + https://www.scholz-recycling.com/services/ + https://www.scholz-recycling.com/geschaeftsbereiche/ + https://www.scholz-recycling.com/recycling/ + https://www.scholz-recycling.com/zertifikate/]

### Importiert (Seed-Stand 2026-09-30)

- Stahl/NE, Kette/Filiale (B2B ok)
- Adresse: Spandau, Tiefwerderweg 13, 13597 Berlin (gleiche Adresse ELNO-Seed: räumliche Koinzidenz, getrennte Firmen)
- Bezirk: Spandau Adressbeleg: seed/be.json + recherche/be.md (Verzeichnisbeleg)

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Dachseite scholz-recycling.com verifiziert (Impressum: Scholz Recycling GmbH, Berndt-Ulrich-Scholz-Straße 1, 73457 Essingen, HRB 733963 AG Ulm) — kein statisch verifizierbarer Berlin-NL-Eintrag (Standortfinder JS-gesteuert). Adresse Tiefwerderweg 13, 13597 Berlin nur Verzeichnis-Leads (Gelbe Seiten, Das Örtliche). website_status tot und Status aktiv unverändert; räumliche Koinzidenz mit ELNO (eigenständige GmbH, HRB 141261 B) bestätigt getrennte Firmen. Kein Zweitbeleg für NL-Adresse — Felder unverändert; Quelle(n): scholz-recycling.com (/impressum, /standorte), gelbeseiten.de (nur Lead)]

### Recherche 31.10.2026

- [Recherche 31.10.2026: Der Standortfinder des Betreibers wurde am 31.10.2026 erneut vollstaendig ausgewertet - das Ergebnis ist unveraendert -, die Seed-Adresse wurde unabhaengig im OSM geprueft, und die Strasse selbst traegt in OSM heute keine Entsorgungsnutzung. Der Standortfinder-Datensatz https://www.scholz-recycling.com/standorte-json-nicht-loeschen/ enthaelt 184 Standort-Datensaetze; eine Suche ueber die Felder ort, plz und strasse nach "Berlin" liefert 0 Treffer - weder ein Berliner Standort noch Tiefwerderweg oder 13597 kommen vor. Ein fehlender Eintrag beweist keine Schliessung, die Zuordnung der historischen Berliner Niederlassung zum heutigen Betreiber bleibt offen; website_status aktiv und status aktiv bleiben unveraendert, weil beide nur den Zustand der Konzern-Website bzw. den Bestand der Zeile betreffen. OSM-Abgleich (Rasterfenster 13.2066394/52.5233669 bis 13.2107512/52.5289213, Tiefwerderweg, 18 Adressobjekte mit addr:housenumber und addr:street=Tiefwerderweg, daraus 16 verschiedene Hausnummern 2, 5, 8, 13, 14, 15, 17, 18, 19, 19A, 28, 31, 32, 33, 34 und 34A; die Schreibweisen "19A" und "34A" sind eigene Adressangaben und keine Bereichsnotationen): Die Dossier-Hausnummer 13 existiert als node/2875570034 mit `addr:housenumber=13`, `addr:street=Tiefwerderweg`, `addr:city=Berlin`, `addr:postcode=13597`, `addr:suburb=Spandau`, `source=Geoportal Berlin / Hauskoordinaten` - die Seed-Adresse ist damit als Adresse bestaetigt. Im Raster tragen 117 Objekte `name=` und/oder `operator=` (108 mit `name=`, 30 mit `operator=`); 0 Objekte nennen "Scholz", "ELNO" oder einen Recycling- oder Schrottbetrieb dieser Anschrift. Die Strasse traegt in OSM ausschliesslich Sport- und Gemeinwesen-Nutzungen: node/2875569756 mit `addr:housenumber=15`, `name=Kajak-Club Albatros 1926 e. V.`, node/2875570228 mit `addr:housenumber=17`, `name=Turngemeinde in Berlin 1848 e.V. Rudern Tiefwerder`, `sport=rowing`, `contact:website=http://www.tib1848ev.de/sportarten/rudern-tiefwerder/`, `contact:phone=+49 30 6110100`, way/228214997 mit `addr:housenumber=28`, `name=Spandauer Hockey- und Tennis-Club 1910`, `operator=Spandauer Hockey- und Tennis-Club 1910 e.V.`, `sport=hockey;tennis`, `community_centre=club_home` sowie node/3667655313 mit `addr:housenumber=5`, `name=Adventgemeinde Spandau`, `amenity=place_of_worship`, `contact:website=http://www.adventgemeinde-spandau.de/`. Das ist eine Aussage zum OSM-Bestand und kein Negativbeleg ueber den Betrieb; die raeumliche Koinzidenz mit dem ELNO-Seed ist damit weiterhin kein Identitaetsbeleg, und die Seed-Adresse ist derzeit nur ueber die Hauskoordinaten des Geoportals belegt, nicht ueber eine Entsorgungsnutzung. Zu Ankauf, Verkauf und Gebuehren getrennt: ANKAUF - der Konzern beschreibt auf https://www.scholz-recycling.com/ das Leistungsspektrum als "ANKAUF / VERKAUF" mit "marktgerechten Preisen" fuer Stahl- und NE-Metallschrott; eine Preisliste, ein Ankaufskurs oder eine standortspezifische Sortenliste fuer Berlin ist nicht veroeffentlicht, und die Sortenangaben der Konzern-Seiten beziehen sich auf den Konzern, nicht auf die historische Niederlassung. VERKAUF - der Konzern tritt als Verkaeufer von Recyclingrohstoffen auf ("ANKAUF / VERKAUF"), nennt aber keine Verkaufskurse. GEBUEHREN - keine Gebuehrentafel, keine Container-Tarife und keine Anlieferungsgebuehr fuer Berlin veroeffentlicht. Offene Fragen: Die Seed-Adresse Tiefwerderweg 13 stuetzt sich weiterhin auf die Geoportal-Hauskoordinaten und auf Verzeichnis-Leads; ein Betriebsbeleg an dieser Anschrift existiert weder in OSM noch im Standortfinder des Betreibers; ob der Betrieb ueberhaupt noch besteht, ist aus den verfuegbaren Quellen nicht entscheidbar. Quelle(n): https://www.scholz-recycling.com/standorte-json-nicht-loeschen/ ; https://www.scholz-recycling.com/standorte/ ; OSM-API-Abfrage api.openstreetmap.org/api/0.6/map.json?bbox=13.2066394,52.5233669,13.2107512,52.5289213 (Abruf 31.10.2026) ; https://www.openstreetmap.org/node/2875570034 ; https://www.openstreetmap.org/way/228214997]
