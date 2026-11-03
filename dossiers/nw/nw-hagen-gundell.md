---
slug: nw-hagen-gundell
name: Gündell
trader_type: schrotthaendler
state: NW
city: Hagen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
clear_fields: 'geo'
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Gündell

## Überblick

„Schrotthandel Gündell“ ist bislang nur über einen 11880-Lead lokalisierbar: Obernahmerstr. 2, 58119 Hagen, Telefon 0171 5443575. Ein weiterer Suchtreffer in einem Verzeichnis bestätigt lediglich die Listung, nicht Betreiberidentität oder aktuellen Betrieb. Kein eigener Webauftritt/Registerbeleg und keine verlässliche Angabe zu Annahme, Abholung oder Öffnungszeiten gefunden; die Kontaktdaten sind nicht in die Stammdaten übernommen. Keine belastbare Ankaufspreisliste, Verkaufspreisliste oder Gebührenordnung gefunden.

## Timeline

### Korrektur 24.10.2026: Gemeinsamen falschen Hagen-Punkt zurückziehen

- [Korrektur 24.10.2026: Die Produktionszeile `nw-hagen-gundell` enthielt den mit mehreren anderen Hagen-Datensätzen geteilten Punkt `51.90933, 7.88825`. Die read-only-Rückwärtssuche löst ihn in Mehringen, Everswinkel, Kreis Warendorf, PLZ 48351 auf, nicht in Hagen. `clear_fields: 'geo'` zieht den falschen Punkt zurück; mangels belegter Händleradresse wird keine Ersatzkoordinate gesetzt. Die Produktionsdatenbank blieb unverändert. Quelle: https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; https://www.openstreetmap.org/way/81516036.]

### Recherche 18.10.2026 (Welle 1202 West)

- [Recherche 18.10.2026: 11880-Detail erstmals in dieser Welle direkt vollständig gelesen (HTTP 200): Gündell/Obernahmerstr. 2/58119 Hagen-Hohenlimburg/0171 5443575, Altmaterialhandel/Schrotthandel. Keine Betreiberwebsite/Mail/Zeiten; Eintragsdaten 29.06.2021, Aktualisierungsanzeige 02.10.2026 kein Nachweis heutigen Betriebs. Allgemeine FAQ zur Kennzeichnung vom Inhaber bestätigt ist keine konkrete Inhaberbestätigung dieses Profils. Keine zusätzliche qualifizierte Identitäts-/Standortquelle; keine Stammdatenfüllung. Ankauf: keine Zahlen/Annahmebedingungen; Verkauf: keine Liste; Gebühren: keine Tarife, Euro-Symbole kein Materialpreis. Betreiber-/Gewerbeabgleich vor GEO; Quelle: https://www.11880.com/branchenbuch/hagen-westfalen/120674719B29367375/schrotthandel-guendell.html .]

### Recherche 03.10.2026

- [Recherche 03.10.2026: Neuer Verzeichnis-Lead Schrotthandel Gündell, Obernahmerstr. 2, 58119 Hagen, 0171 5443575 aus vollständig gelesener 11880-Ergebnisliste. Kein unabhängig verifizierter Betreiber, keine passende Website mit Impressum; Detail-Link als Folgeprüfpunkt, kein zusätzlicher Beleg. Adresse und Telefon nicht gefüllt, status pruefung bleibt. Blocker: alleiniger Aggregatorbeleg, Aktualität/Ankauf offen; Quelle(n): https://www.11880.com/suche/schrotthandel/hagen-westfalen + https://www.11880.com/branchenbuch/hagen-westfalen/120674719B29367375/schrotthandel-guendell.html (Detail-Link zum Lead)]

### Recherche 06.10.2026

- [Recherche 06.10.2026: Der Suchlauf brachte keine zulässige Betreiber- oder Registerquelle für Gündell hervor; der Treffer von „Wer kennt den BESTEN“ ist ebenfalls nur ein Portal-Lead. Keine Aussage über Schließung/Nichtexistenz abgeleitet. Aktuelle Tätigkeit, Adresse und Telefonnummer bleiben zu bestätigen; zu Ankauf, Verkauf oder Gebühren wurden keine belastbaren Preise gefunden. Quellen/Leads: https://www.11880.com/branchenbuch/hagen-westfalen/120674719B29367375/schrotthandel-guendell.html ; https://www.werkenntdenbesten.de/schrotthandel/hagen]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Hagen

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Lead-Adresse **Obernahmerstr. 2, 58119
  Hagen-Hohenlimburg** ist im live abgerufenen OSM-Bestand **nicht** erfasst, und die Adresse stützt sich allein auf
  **einen** Verzeichniseintrag. **Was live geprüft wurde (03.11.2026):** (a) Eine Suche über alle Tag-Werte nach
  `addr:street`=**Obernahmerstraße** im Kachelraster **7.4200/51.3400 – 7.5600/51.4200** (112 Kacheln à 0,01 Grad,
  live abgerufen über `https://api.openstreetmap.org/api/0.6/map.json?bbox=…`, **0 Kachel-Fehler**) liefert **0**
  Adressobjekte und **0** Hausnummern; auch eine Suche nach dem Straßennamen selbst (als `name=` eines
  Straßen-Ways oder eines Stoppunkts) liefert im Raster **keinen** Treffer — vorhanden sind dort nur die
  Buslinien-Relationen „HA-Obernahmer“ (Linien 538) sowie die benachbarten Straßen Rehstraße, Obere Rehstraße,
  Eugen-Richter-Straße, Wehringhauser Straße und Berliner Straße in Hagen-Hohenlimburg. Die **Dossier-Straße
  „Obernahmerstr.“ existiert in OSM nicht**. (b) Eine Suche über `name`, `operator`, `brand`, `website`,
  `contact:website`, `description`, `note` und `short_name` nach **„Gündell“**, **„Gündell“/„Gundell“** und
  **„Schrotthandel Gündell“** im selben Raster liefert **0** Treffer — Aussage zum OSM-Bestand, **kein**
  Negativbeleg über den Betrieb. (c) Für die in der Straße üblichen Nachbarnummern (Rehstraße, Obere Rehstraße)
  wurde kein Objekt mit Betriebsbezug gefunden. Quelle 1 (Verzeichnis, **keine** Betreiber-/Registerquelle): der
  11880-Eintrag „Schrotthandel Gündell“, **Obernahmerstr. 2, 58119 Hagen-Hohenlimburg**, **0171 5443575**, Branchen
  Altmaterialhandel/Schrotthandel (Eintragsdaten 29.06.2021, Aktualisierungsanzeige 02.10.2026 — Portalmetadaten, **kein**
  Nachweis heutiger Tätigkeit); keine Betreiberwebsite, keine E-Mail, keine Öffnungszeiten; der Treffer bei „Wer
  kennt den BESTEN“ ist ebenfalls nur ein Portal-Lead. Quelle 2 (OSM): die Dossier-Straße und ihre Hausnummer
  existieren im live abgerufenen Raster **nicht**, ein Objekt mit Betriebsbezug ebenfalls nicht. **Was fehlt:** ein
  adressgenauer OSM-Punkt **und** eine zweite unabhängige Belegquelle für die Anschrift (Betreiberkontakt oder
  Gewerbeauskunft); erst damit wäre die Zeile geokodierbar. `clear_fields: 'geo'` bleibt bestehen,
  `status: pruefung` bleibt unverändert, es wird weder Fortbestand noch Schließung behauptet.
  Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=7.4200,51.3400,7.5600,51.4200 ;
  https://www.11880.com/branchenbuch/hagen-westfalen/120674719B29367375/schrotthandel-guendell.html ;
  https://www.werkenntdenbesten.de/schrotthandel/hagen (Abruf 03.11.2026).]
