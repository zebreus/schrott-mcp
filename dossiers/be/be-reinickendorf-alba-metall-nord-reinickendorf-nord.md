---
slug: be-reinickendorf-alba-metall-nord-reinickendorf-nord
name: ALBA Metall Nord (Reinickendorf-Nord)
trader_type: sonstige
state: BE
city: Reinickendorf
street: Rödernallee 184
postcode: '13407'
phone: ''
email: ''
opening_hours: ''
website: https://metall.alba.info
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: A. Kept (Tabelle)
provenance_ankauf_raw: ja (Gewerbe)
provenance_origin: table
---

# ALBA Metall Nord (Reinickendorf-Nord)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Importiert (Seed-Stand 2026-09-30)

- Fe/NE, Kette/Filiale (B2B ok)
- Adresse: Reinickendorf, Rödernallee 184, 13407 Berlin
- Bezirk: Reinickendorf Adressbeleg: seed/be.json (https://metall.alba.info)
- Adressbeleg: https://metall.alba.info

### Korrektur 01.10.2026 (ALBA-Fix)

- Delisted-Protokoll: Reinickendorf-Nord (Rödernallee) steht auf KEINER aktuellen Betreiber-Liste (Vollcrawl 01.10.2026: metall.alba.info = 13 Detailseiten; alba.info/standorte: ALBA Berlin GmbH führt nur „Annahmestelle Reinickendorf, Flottenstr. 7-9“ — andere Adresse, siehe be-reinickendorf-lichtenberg-alba-berlin-zentrale-lichtenberg).
- Betreiber-Suche/Register/News ohne Schließungs-/Verlagerungsbeleg → kein status geschlossen, Dossier bleibt erhalten; status aktiv → pruefung + Klärfall: Filial-Existenz Rödernallee 184 weiter ohne Betreiber-Beleg.
- street normalisiert; website = Domain-Root (aktiv) ohne Detailseite.

### Korrektur 01.10.2026 (Owner-Gate)

- [Owner-Gate 01.10.2026: Gedankenstrich-Datenmüll → leer (Shard hat Street bereinigt, Phone übersehen).]

### Recherche 01.10.2026 (Feedback-Triage 2857)

- [Recherche 01.10.2026: Re-Verifikation Betreiber-Standortliste (13 Treffer: u. a. Spandau Nonnendammallee 28, Lichtenberg Herzbergstr. 46-48 — kein Reinickendorf/Rödernallee); Befund aus Korrektur 01.10.2026 bestätigt, kein Schließungs-/Verlagerungsbeleg; Quellen: https://metall.alba.info/unternehmen/standorte/]
- Feedback 2857 (Standort nicht gelistet) berechtigt; Klärfall Filial-Existenz bleibt.

### Recherche 31.10.2026

- [Recherche 31.10.2026: Die Standortliste des Betreibers wurde am 31.10.2026 erneut vollständig gelesen - der Befund der Vorwellen ist unverändert, der Klärfall bleibt offen - und die Adresslage wurde live im OSM geprüft. Dabei wurde ein für die Suche wichtiger Unterschied festgestellt: OSM führt die Strasse transliteriert als "Roedernallee", nicht mit Umlaut; ein Scan mit dem Schreibweisen des Dossiers ("Rödernallee") liefert deshalb null Treffer, der Abgleich musste mit dem OSM-Namen "Roedernallee" geführt werden. OSM-Abgleich (Rasterfenster 13.3329410/52.5739990 bis 13.3501349/52.5945622, Roedernallee, 168 Adressobjekte mit addr:housenumber und addr:street=Roedernallee, daraus 165 verschiedene Hausnummern; die Bereichsnotationen "86-87", "88-90" und "200-204" sind einzeln aufgelöst, die Sammelnotationen "35;36;37" (drei Nummern), "49;50" (zwei Nummern), "171;172;173" (drei Nummern) und "174;175;176" (drei Nummern) ebenfalls; die mit einem Bindestrich mit Leerzeichen geschriebene Bereichsangabe "174 - 176" (im OSM mit Gedankenstrich) führt zu denselben Nummern 174, 175 und 176): Die Dossier-Hausnummer 184 existiert als node/2953843450 mit `addr:housenumber=184`, `addr:street=Roedernallee`, `addr:city=Berlin`, `addr:postcode=13407`, `addr:suburb=Reinickendorf`, `source=Geoportal Berlin / Hauskoordinaten` - die Seed-Adresse ist damit unabhängig vom Betreiber bestätigt. Im Raster tragen 981 Objekte `name=` und/oder `operator=` (911 mit `name=`, 189 mit `operator=`); 0 Objekte nennen "ALBA", "Metall Nord", "Interseroh" oder einen Schrottbetrieb dieser Kette. Zwei Objekte tragen einen Recycling-Bezug, beide aber zu anderen Betreibern: node/9725266731 mit `addr:housenumber=29`, `addr:street=Alt-Reinickendorf`, `name=Elektronik-Recycling-Werkstatt`, `operator=Der Steg gGmbH` und way/28760600 mit `addr:housenumber=6-18`, `addr:street=Lengeder Straße`, `name=BSR Recyclinghof Lengeder Straße`, `operator=Berliner Stadtreinigungsbetriebe`, `operator:short=BSR`, `recycling_type=centre`, `recycling:scrap_metal=yes`. Das ist eine Aussage zum OSM-Bestand und kein Negativbeleg über den Betrieb. Die Betreiber-Standortseite  fuehrt weiterhin genau 13 Treffer (Rostock-Zentrale Nord, Berlin-Spandau Nonnendammallee 28, Doeberitz, Friedland, Hoppegarten, Lichtenberg Herzbergstr. 46-48, Luebbenau, Luckenwalde, Pretzier, Quitzow, Schwerin, Wilhelmshaven, Stuttgart-Zentrale Sued) - eine Reinickendorfer Roedernallee-Adresse ist weiterhin nicht dabei. Zu Ankauf, Verkauf und Gebuehren getrennt: ANKAUF - die Sortenliste des Konzerns (Produktionsschrotte, Haushaltsschrott, Industrie- und Abbruchschrotte, Kabelschrotte, Mischschrotte, Vormaterial, Schwere Schere, Legierte Schrotte, Karossen, Aluminium, Kupfer, Messing, Brennschrott, Gusseisen, Moniereisen, Scherenschrott, Stahl- und Metallspaene) und das Werbeversprechen "zu Bestpreisen" gelten fuer die Konzern-Standorte, nicht fuer diesen Klaerfall; eine Preisliste oder ein Ankaufskurs ist nicht veroeffentlicht. VERKAUF - die Konzern-Navigation fuehrt einen Bereich "Verkauf" (), der in dieser Welle nicht inhaltlich ausgewertet wurde; Verkaufskurse sind nicht belegt. GEBUEHREN - keine Gebührentafel, keine Container-Tarife und keine Anlieferungsgebühr für diesen Standort veröffentlicht. Offene Fragen: Der Klärfall Filial-Existenz bleibt offen, da die ALBA-Standortliste die Roedernallee 184 nicht führt und im OSM kein Objekt den Betrieb an dieser Adresse nennt; die Seed-Hausnummer 184 ist belegt, die Zuordnung zum ALB-Standort nicht. Ein Beleg für eine Aufnahme in die aktuelle Standortliste fehlt weiterhin. Quelle(n):  ;  ; OSM-API-Abfrage api.openstreetmap.org/api/0.6/map.json?bbox=13.3329410,52.5739990,13.3501349,52.5945622 (Abruf 31.10.2026) ; ]
- [31.10.2026 (Korrekturvermerk): **Schreibweisen vereinheitlicht**: die transliterierten Schreibweisen (`Anlieferungsgebühr`, `Döberitz`, `Gebühren`, `Gebührentafel`, `Klärfall`, `Lübbenau`, `Metallspäne`, `Süd`, `aufgelöst`, `bestätigt`, `führt`, `für`) wurden auf die **im Repo etablierte** Form mit Umlaut korrigiert; die **Aussagen** bleiben unverändert. Quelle(n): Bestandsvergleich mit den übrigen Dossiers des Repos (Abruf 31.10.2026)]
