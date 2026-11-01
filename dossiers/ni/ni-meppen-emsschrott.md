---
slug: ni-meppen-emsschrott
name: Emsschrott GmbH & Co. KG
trader_type: schrotthaendler
state: NI
city: Meppen
street: 'Essener Str. 15'
postcode: '49716'
phone: '(05932) 7333-0'
email: 'info@emsschrott.de'
opening_hours: 'Mo-Fr 07:30-17:00, Sa 09:00-12:00'
website: https://emsschrott.de
website_status: aktiv
status: aktiv
description: 'Altmetall- und Schrottspezialist in Meppen-Hüntel (Augustin-Gruppe): Ankauf und Erfassung sowie Sortierung, Aufbereitung und Vermarktung von Schrotten und Metallen.'
dropoff_json: ''
pickup_json: ''
provenance_section: Osnabrück / Emsland / Grafschaft Bentheim
provenance_ankauf_raw: ja
provenance_origin: table
---

# Emsschrott GmbH & Co. KG

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrott (Emsland)

### Recherche 01.10.2026

- [Recherche 01.10.2026: Adresse, Telefon, Mail, Öffnungszeiten, Beschreibung verifiziert und Frontmatter gefüllt (Owner-Ausnahme: Betreiber-Primärquelle HR-kongruent: Emsschrott GmbH & Co. KG, Essener Str. 15, 49716 Meppen, HRA 202873 AG Osnabrück, Gesellschafter Augustin Entsorgung Holding GmbH); website live abgerufen daher website_status aktiv; Quelle(n): https://emsschrott.de/ + https://emsschrott.de/kontakt/ + https://emsschrott.de/impressum/ + https://www.northdata.de/Emsschrott%20GmbH%20%26%20Co%C2%B7%20KG,%20Meppen/Amtsgericht%20Osnabr%C3%BCck%20HRA%20202873]
- Standort Meppen-Hüntel; GF Rolf Augustin, Tim Metz; Vertrieb Gabe van der Zwaag, Andre Holtkamp (Betreiber-Kontaktseite)
- Northdata-Gegenstand (Schiffsrückbau/DA Schiffsverwertung) veraltet, Betreiber-Leistung (Altmetalle/Schrotte) gilt; City-Feld Meppen unverändert (Betreiber: Meppen-Hüntel)

### Recherche 01.11.2026

- [Recherche 01.11.2026: Ankaufpreis live bestätigt: Emsschrott veröffentlicht auf https://emsschrott.de/altmetalle-und-schrott-an-und-verkauf/ (HTTP 200, 118.150 Bytes, abgerufen 01.11.2026 05:37 UTC) eine Tabelle „Schrottangebot der Woche" mit 8 Sorten.
  Wörtliche Preisfragmente (Sorte | Wert | Einheit): „Mischschrott | 105,- €/to", „Messing | 5,70 €/kg", „Alu Geschirr | 1,06 €/kg", „Alu Felgen | 1,77 €/kg", „Blei | 0,78 €/kg", „Kupfer Kabel (ohne Stecker) | 3,24 €/kg", „Kupfer alt (z.B. alte Wasserleitungen, …) | 9,25 €/kg", „Zink (Dachrinnen, …) | 1,90 €/kg".
  Das Screening nannte 7 Sorten und ließ „Zink (Dachrinnen, …) | 1,90 €/kg" aus; die Live-Tabelle hat 8 Datenzeilen, die Sortenzahl des Screenings ist damit zu korrigieren.
  Preisart: ANKAUF (Betreiber zahlt dem Verkäufer), wörtlich belegt durch „Wir kaufen Ihren Schrott an, zu Bestpreisen" und „Wir kaufen Ihren Schrott, sortenrein oder gemischt, in kleinen Mengen oder großen Stückzahlen, zu Bestpreisen an."
  Der Preis ist ausdrücklich marktbasiert abgeleitet: „Wir rechnen hierbei nach tagesaktuellen marktgerechten Preisen ab. Dafür greifen wir auf verschiedene Leitindexe oder Börsen zurück." Das ist eine Herkunftsangabe zum Ankaufpreis des Betreibers, kein eigenständiger Börsenkurs-Eintrag; ein Börsenkurs wird auf der Seite nicht veröffentlicht.
  Mindestmenge und Staffel: nicht veröffentlicht; die Preise gelten laut Betreibertext für „in kleinen Mengen oder großen Stückzahlen". Ausgeschlossen ist „weiße Ware" (elektrische Haushaltsgroßgeräte) sowie „weiße Ware" laut Hinweis.
  Die Tabelle liegt in einem aufklappbaren Accordion-Block „Schrottpreis-Angebote der Woche" und ist serverseitig gerendert; JS ist nicht erforderlich.
  Quelle(n): https://emsschrott.de/altmetalle-und-schrott-an-und-verkauf/ (Live-Abruf 01.11.2026, 05:37-05:47 UTC)]
- [Recherche 01.11.2026: Datumfrage des Screenings geklärt: die Tabelle selbst nennt KEIN Datum, ihr Aktualisierungsrhythmus ist aber im Text ausgewiesen.
  Wörtlich: „Jede Woche Donnerstag veröffentlichen wir aktuelle Preisangebote für bestimmte Schrottsorten." Der Block ist als „Schrottangebot der Woche" überschrieben.
  Der Preisstand lässt sich damit zeitlich eingrenzen, aber nicht auf einen Tag festlegen: die Seite wurde laut WordPress-Metadaten zuletzt am 26.10.2026 geändert (og:updated_time 2026-10-26T15:02:42+00:00, created 2026-08-25T13:37:52, bestätigt über https://emsschrott.de/wp-json/wp/v2/pages?slug=altmetalle-und-schrott-an-und-verkauf).
  Da der 26.10.2026 ein Montag ist und die Veröffentlichung donnerstags erfolgen soll, war der zuletzt veröffentlichte Angebotsstand der Donnerstag 22.10.2026; ob die Tabelle seither unverändert blieb, ist nicht belegbar, weil kein Tagesstempel im Text steht.
  Für die Datenmodellierung folgt daraus: kein verlässliches published_at aus der Quelle, Preisstand als observed_at am Abrufzeitpunkt führen.
  Prüfung der übrigen Seiten: alle übrigen Unterseiten der Domain wurden separat abgerufen und enthalten keinen einzigen Euro-Wert (Startseite, /leistungen/, /erfassung-von-altmetallen/, /aufbereitung-und-verwertung-von-altmetallen/, /sorten-katalog/, /kontakt/ jeweils HTTP 200 ohne Preiszeile; die Pfade /preise, /ankaufpreise, /downloads, /preisliste und /schrottpreise antworten HTTP 404, /sorten liefert den Sortenkatalog ohne Preise).
  Der Sortenkatalog beschreibt 16 Sorten rein sachlich (Aluminium-Felgen, Kupfer-Millberry, Kupfergranulat, Neuschrott, Altschrott u. a.) ohne jede Preisangabe.
  Quelle(n): https://emsschrott.de/altmetalle-und-schrott-an-und-verkauf/ ; https://emsschrott.de/sorten-katalog/ ; https://emsschrott.de/wp-json/wp/v2/pages?slug=altmetalle-und-schrott-an-und-verkauf (Live-Abruf 01.11.2026, 05:37-05:47 UTC)]
- [Recherche 01.11.2026: Dossier- und Standortbezug: die Preistabelle ist eine Gruppenliste des Emsschrott-Standortverbunds und deckt die drei Dossiers dieser Welle (Meppen-Hüntel, Nordhorn, Werlte) gemeinsam ab.
  Die Preisquelle liegt auf der Domain des Betreibers Emsschrott GmbH & Co. KG (Essener Str. 15, 49716 Meppen-Hüntel); die Seite nennt die Anlieferpunkte im Wortlaut: „Bei Emsschrott in Meppen-Hüntel und an vielen weiteren Augustin Entsorgung Standorten können sie ganz einfach ihren Schrott anliefern."
  Die Liste der Schrottannahmestellen auf derselben Seite führt wörtlich: „Meppen – Hüntel, Essener Str. 15, 49716 Meppen", „Meppen, Dieselstraße 49, 49716 Meppen", „Werlte, Am Zirkel 10, 49757 Werlte", „Leer / Ostfriesland, Eisenstr. 7, 26789 Leer", „Herzlake, Boschstr. 12, 49768 Herzlake", „Nordhorn, Alfred-Mozer-Str. 59, 48527 Nordhorn", „Papenburg, Am Nordhafen 13, 26871 Papenburg".
  Damit ist die Preisquelle für alle drei Emsschrott-Dossiers dieselbe Operator-Primärquelle; es gibt keine standortspezifische Preisliste.
  Für die Dossiers Werlte und Nordhorn ist die Standortzuordnung damit neu belegt: die auf der Preisquelle genannten Anschriften decken sich mit den Dossier-Adressen (Am Zirkel 10, 49757 Werlte; Alfred-Mozer-Str. 59, 48527 Nordhorn). Abweichung: das Dossier Nordhorn nennt „Alfred-Mozer-Str. 57-59", die Preisquelle nennt „Alfred-Mozer-Str. 59"; die Dossier-Adresse bleibt unverändert, die Abweichung ist als offener Punkt notiert.
  Ebenfalls neu belegt: die Gruppenseiten des Betreibers https://www.augustin-entsorgung.de/ueber-uns/standorte/nordhorn/ und https://www.augustin-entsorgung.de/ueber-uns/standorte/werlte/ (beide HTTP 200) führen Emsschrott als Aufnahmeeinheit mit derselben Adresse; diese Seiten enthalten selbst keine Preise.
  Offen bleibt: ob die Wochenpreise an allen aufgeführten Annahmestellen identisch gelten, ist nicht ausgewiesen.
  Quelle(n): https://emsschrott.de/altmetalle-und-schrott-an-und-verkauf/ ; https://www.augustin-entsorgung.de/ueber-uns/standorte/nordhorn/ ; https://www.augustin-entsorgung.de/ueber-uns/standorte/werlte/ (Live-Abruf 01.11.2026, 05:37-05:47 UTC)]
