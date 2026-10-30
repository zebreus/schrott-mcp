---
slug: rp-simmern-schneider-recycling
name: Schneider Recycling GmbH & Co. KG
trader_type: containerdienst
state: RP
city: Simmern
street: Fuhrweg 1-3
postcode: '56337'
phone: '0 26 20 / 95 69-0'
email: info@schneider-recycling.com
opening_hours: 'Mo-Fr 07:30-12:30 und 13:30-16:30'
website: https://www.schneider-recycling.com
website_status: aktiv
status: pruefung
description: 'Schneider Recycling GmbH & Co. KG, Fuhrweg 1-3, 56337 Simmern/Ww., Familienunternehmen: 1975 von Winfried Schneider mit der Entsorgung von Papier und Folie gegründet, 1986 kam Thomas Schneider hinzu. Schwerpunkt seit über 20 Jahren auf Erfassung, Sortierung und Vermarktung von Altpapier; Leistungen Altpapierentsorgung (Verpackungspapiere, Karton, Prospekte), Kunststoffentsorgung (Folien, Hartkunststoffe, PET) und Containerdienst für gemischte Wertstoffe, Baustellenmischabfälle und Grünabfälle mit Containern von 10 bis 32 cbm im Behälter-Mietservice. Eine zweite Gesellschaft am Standort Kreuzwiese 6 betreibt die Akten- und Datenträgervernichtung. Laut amtlichem Fachbetrieberegister (Auszug 11.08.2026) Tätigkeit Lagern und Behandeln. Ein Ankauf von Schrott, Altmetall oder Eisenmetallen ist nicht belegt.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Schneider Recycling GmbH & Co. KG

## Überblick

Die **Schneider Recycling GmbH & Co. KG** in **Fuhrweg 1-3, 56337 Simmern/Ww.** ist ein Familienbetrieb, den Winfried Schneider 1975 mit der "Entsorgung von Papier und Folie" im Ein-Mann-Betrieb gründete und in den Thomas Schneider 1986 eintrat. Der Schwerpunkt liegt seit über 20 Jahren "auf Erfassung, Sortierung und Vermarktung von Altpapier"; das Leistungsspektrum umfasst Altpapierentsorgung (Verpackungspapiere, Karton, Prospekte), Kunststoffentsorgung (Folien, Hartkunststoffe, PET) und einen Containerdienst für gemischte Wertstoffe, Baustellenmischabfälle und Grünabfälle. Das amtliche Fachbetrieberegister führt den Standort entsprechend als "Papier- und Kunststoffentsorgung (Lagerplatz und manuelle Sortieranlage)" mit "Lagern, Behandeln". **Ein Ankauf von Schrott, Altmetall oder Eisenmetallen ist weder auf der Betreiberseite noch im Register belegt**; das Seed-Fragment "Schrott/Metall/Container" ist damit nur teilweise zutreffend, weshalb `trader_type` von `schrotthaendler` auf `containerdienst` korrigiert wurde.

- **Ankauf:** kein Ankauf von Schrott oder Metall belegt; vermarktet wird Altpapier, Ankaufspreise werden nicht genannt.
- **Verkauf:** Vermarktung von Altpapier und Kunststoff, keine Preisliste und keine Beträge.
- **Gebühren:** Container im Behälter-Mietservice von 10 cbm bis 32 cbm, auch als flüssigkeitsdichte Spezialcontainer; kein bezifferter Tarif veröffentlicht.
- **Geokodierung:** Auf dem Fuhrweg in OSM ist nur die Hausnummer 1 erfasst (node/683450269, Café); die Nummern 2, 3 und die Bereichsnotation "1-3" fehlen im OSM-Bestand. Eine belegte Geokodierung steht aus.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrott/Metall/Container
- Adresse: Simmern, Fuhrweg 1-3, 56337
- Adressbeleg: https://www.schneider-recycling.de

### Recherche 30.10.2026

- [Recherche 30.10.2026: **Die Seed-Domain ist nur eine Auswahlseite; der Inhaltsauftritt liegt auf einer anderen Domain.** https://www.schneider-recycling.de (Abruf 30.10.2026) zeigt nur "Willkommen bei Schneider Recycling / Bitte wählen Sie die gewünschte Firma:" mit zwei Eintraegen: "Schneider Recycling GmbH & Co. KG / Fuhrweg 1-3 / D-56337 Simmern/Ww. / Telefon: 0 26 20 / 95 69-0 / Fax: 0 26 20 / 95 69-20" und "Schneider Recycling GmbH / Kreuzwiese 6 / D-56337 Simmern/Ww. / Tel. 0 26 20 / 95 47 74-0 / Fax 0 26 20 / 95 47 74-22"; verlinkt sind https://www.schneider-recycling.com/home und https://www.schneider-recycling.com/akten. `website` wurde deshalb auf die Inhaltsdomain geändert und `website_status` auf aktiv gesetzt. Das Dossier-Adressfeld "Fuhrweg 1-3" ist damit durch die Betreiberseite bestätigt. Quelle(n): https://www.schneider-recycling.de (Abruf 30.10.2026); https://www.schneider-recycling.com/home (Abruf 30.10.2026)]
- [Recherche 30.10.2026: **Kernbefund: das Geschäft ist Papier- und Kunststoff-Recycling, kein Schrott- oder Metallhandel.** Die Seite "Recycling" (Abruf 30.10.2026) schreibt wörtlich: "Seit mehr als 20 Jahren liegt der Schwerpunkt unserer Aktivitäten auf Erfassung, Sortierung und Vermarktung von **Altpapier**." Die Startseite nennt als "Unsere Leistungsschwerpunkte" die "Altpapierentsorgung" ("Verpackungspapiere, Karton, Prospekte") und die "Kunststoffentsorgung" ("Folien, Hartkunststoffe, PET") sowie den "Containerdienst" für "gemischte Wertstoffe, Baustellenmischabfälle, Grünabfälle". Die Seite "Über uns" nennt wörtlich: "Den Grundstein für die Schneider Recycling GmbH & Co. KG sowie die Schneider Recycling GmbH legte Winfried Schneider im Jahre 1975 mit der **Entsorgung von Papier und Folie** im Rahmen eines Ein-Mann-Betriebes" und "Im Jahre 1986 trat die zweite Generation, Thomas Schneider, mit in das Unternehmen ein." ANKAUF: **kein Ankauf von Schrott, Altmetall oder Eisenmetallen** auf den geprüften Seiten; vermarktet wird Altpapier. VERKAUF: Vermarktung von Altpapier, keine Preisliste und keine Beträge. GEBÜHREN: "Mit unserem **Behälter-Mietservice** erhalten Sie eine flexible und günstige Möglichkeit" sowie "Wir stellen Ihnen Container in Größen von **10 cbm bis 32 cbm**, auch als flüssigkeitsdichte Spezialcontainer"; kein bezifferter Tarif. Quelle(n): https://www.schneider-recycling.com/home/recycling/ ; https://www.schneider-recycling.com/home/wir/ ; https://www.schneider-recycling.com/home/container/ (Abruf 30.10.2026)]
- [Recherche 30.10.2026: **Kontaktdaten und Öffnungszeiten live belegt.** Die Fußzeile der Inhaltsseite (Abruf 30.10.2026) lautet wörtlich: "Unsere Anschrift: Schneider Recycling GmbH & Co. KG / Fuhrweg 1-3 / D-56337 Simmern/Ww. / Telefon: 0 26 20 / 95 69-0 / Fax: 0 26 20 / 95 69-20 / EMail: info@schneider-recycling.com"; die in derselben Fusszeile genannte Internet-Adresse verweist auf die Inhaltsdomain schneider-recycling.com und "Unsere Öffnungszeiten: Montag bis Freitag / 07:30 Uhr bis 12:30 Uhr und / 13:30 Uhr bis 16:30 Uhr" mit dem Hinweis "Zutritt zum Firmengelände nur mit Warnweste!" und dem Kontaktkopf "Call us : 02620 9569-0 / Mail us : info@schneider-recycling.com". Telefon, Fax und E-Mail wurden in die Frontmatter übernommen; die Öffnungszeiten als "Mo-Fr 07:30-12:30 und 13:30-16:30". Quelle(n): https://www.schneider-recycling.com/home (Abruf 30.10.2026)]
- [Recherche 30.10.2026: **Amtsregister bestätigt die Zweiggesellschaft und die Tätigkeit als Papier- und Kunststoffentsorgung.** Die Ortsseite Simmern des amtlichen Fachbetrieberegisters (Auszug 11.08.2026, Spiegel schrottregister.pages.dev) fuehrt "Schneider Recycling GmbH & Co. KG **Papier- und Kunststoffentsorgung (Lagerplatz und manuelle Sortieranlage)** | Fuhrweg 1-3 | Lagern, Behandeln" sowie "Schneider Recycling GmbH (Akten- und Datenträger- Vernichtungsanlage) | Kreuzwiese 6 | Sammeln, Befördern, Lagern, Behandeln". Die Registerangabe deckt sich mit dem Betreibertext; **kein** Schrott- oder Metallankauf wird weder vom Register noch vom Betreiber ausgewiesen. Quelle(n): https://schrottregister.pages.dev/ort-simmern (Abruf 30.10.2026)]
- [Recherche 30.10.2026: **OSM-Abgleich: auf dem Fuhrweg in Simmern ist nur die Hausnummer 1 erfasst.** Die Straße "Fuhrweg" besteht in OSM aus den zwei Ways way/233137833 und way/1526112580. Im Rasterfenster **7.6716988/49.3897662 bis 7.6770371/49.3939214** (Way-Union-BBox plus 0,002 Grad Rand) liegt dort **1 Adressobjekt / 1 verschiedene Hausnummer**: **node/683450269** mit den Tags `addr:street=Fuhrweg`, `addr:housenumber=1`, `addr:city=Simmern`, `addr:country=DE`, `addr:postcode=56337`, `amenity=cafe`, `name=Parkcafe`, `phone=+49 26 20 9 56 90`, `smoking=no`. Präzise Negativaussage: **die Hausnummern 2 und 3 sowie die Bereichsnotation "1-3" sind auf dem Fuhrweg in OSM nicht erfasst**, und es existiert dort kein Objekt mit Bezug zur Schneider Recycling; das einzige Adressobjekt ist ein Cafe. Das ist eine Aussage zum OSM-Bestand und kein Gegenbeleg gegen die Betreiberanschrift. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (bbox-Raster, Abruf 30.10.2026); https://api.openstreetmap.org/api/0.6/node/683450269.json]
