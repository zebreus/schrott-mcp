---
slug: rp-koblenz-56073-r-a
clear_fields: 'geo'
name: Schrotthandel R&A
trader_type: schrotthaendler
state: RP
city: Koblenz 56073
street: Moselweißer Str. 20
postcode: 56073
phone: 0176 65073477
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# R&A

## Überblick

Der Seed „Schrotthandel R&A" ist durch zwei unabhängige Verzeichnisse gedeckt: Gelbe Seiten (Detailseite direkt gelesen) führt „Schrotthandel R&A, Moselweißer Str. 20, 56073 Koblenz-Rauental, 0176 65 07 34 77, Branche Schrott", schrottplatz.org führt „Schrott Schrotthandel. R&A" an derselben Anschrift. Beides sind Aggregator-Leads; eine Betreiber-Website, ein Impressum oder ein Registereintrag existiert nicht, der Status bleibt `pruefung`. Der in der Produktionszeile gespeicherte Koordinatenpunkt gehört nachweislich zu einem **anderen** Koblenzer Betrieb: Er löst auf „Devorastraße 2, Goldgrube, 56073 Koblenz" auf, und schrottplatz.org führt unter dieser Anschrift „Schrottplatz 1A Schrotthandel & Entrümpelungen". `clear_fields: 'geo'` zieht den fremden Punkt zurück.

**ANKAUF:** keine Kurse und keine Materialangaben. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife.

## Timeline

### Recherche 27.10.2026: Zweites Verzeichnis bestätigt Anschrift, gespeicherter Punkt gehört zu anderem Betrieb

- [Recherche 27.10.2026: Die Gelbe-Seiten-Detailseite wurde direkt gelesen und nennt „Schrotthandel R&A", „Moselweißer Str. 20, 56073 Koblenz-Rauental", „0176 65 07 34 77", Branche „Schrott", ohne Website und ohne E-Mail. Das Schrottplatz-Portal führt unabhängig „Schrott Schrotthandel. R&A" mit derselben Anschrift „Moselweißer Str. 20, 56073 Koblenz". Damit liegen die bereits eingetragenen Werte street, postcode und phone auf zwei unabhängigen Verzeichnisquellen; es gibt weiterhin keine Betreiber-Primärquelle und kein Register, der Status bleibt `pruefung`, E-Mail, Öffnungszeiten und Website bleiben leer. Quellen: https://www.gelbeseiten.de/gsbiz/f3e15f7e-7279-4692-86b1-1504b98cf884 (direkt abgerufen) ; https://www.schrottplatz.org/koblenz/schrott-schrotthandel-ra-aYEPwO.html .]
- [Recherche 27.10.2026: GEO-Korrektur mit Namensvetter-Bezug: Der gespeicherte Punkt 50.3540142857143, 7.5774 löst per Nominatim-Reverse auf „2, Devorastraße, Goldgrube, Koblenz, 56073" auf. Dieselbe Anschrift führt im Schrottplatz-Portal der Eintrag „Schrottplatz 1A Schrotthandel Koblenz, Devorastraße 2, 56073 Koblenz", der als „Schrottplatz 1A Schrotthandel & Entrümpelungen" geführt wird. Der gespeicherte Punkt gehört damit zu einem anderen Koblenzer Betrieb und nicht zu „Schrotthandel R&A" (Moselweißer Str. 20). `clear_fields: 'geo'` entzieht das Paar beim regulären Seed; das bedeutet nur „keine kuratierte aktuelle Koordinate", keine Schließung und keine Annahmeverweigerung. Eine Ersatzkoordinate wurde nicht geraten, weil für Moselweißer Straße 20 nur eine Verzeichnisangabe vorliegt und kein Nachweis einer öffentlichen Annahmestelle. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.3540142857143&lon=7.5774&zoom=18&addressdetails=1 ; https://www.schrottplatz.org/koblenz/schrottplatz-1a-schrotthandel-koblenz-aYEOYK.html ; https://nominatim.openstreetmap.org/search?q=Moselwei%C3%9Fer%20Stra%C3%9Fe%2020%2C%2056073%20Koblenz&format=jsonv2&addressdetails=1&limit=3 ; read-only-Abgleich `public.db` (27.10.2026).]
- [Recherche 27.10.2026: **ANKAUF:** keine Kurse, keine Materialangaben; die Verzeichniseinträge nennen nur die Branche „Schrott". **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife; die genannten Leistungen sind Schrott/Schrottankauf/Schrottentsorgung, ohne Bezifferung. Abgrenzung: Die übrigen im Gelbe-Seiten-Umfeld geführten Koblenzer Betriebe („Koblenzer Schrotthandel", Am Metternicher Bahnhof 18, 56072; „SCHROTTHANDEL 56", Kastorstr. 12, 56068; „Kreutz Schrotthandel", Dominicusstr. 43, 56073; „Schrottplatz 1A", Devorastraße 2, 56073; „Schrott Alten Angelo", Peter-Friedhofen-Str. 9, 56073) sowie die Betreiberauftritte Koblenzer Schrotthandel (koblenzer-schrotthandel.de), PMS Schrotthandel, TSR Deutschland und Schrotthandel NCM sind eigenständige Betriebe und wurden nicht übernommen. Quellen: https://www.gelbeseiten.de/branchen/schrotth%C3%A4ndler/koblenz%20am%20rhein ; https://www.koblenzer-schrotthandel.de/ ; https://www.tsr-recycling.de/standorte/schrotthaendler-koblenz .]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Adresse Moselweißer Str. 20, 56073 Koblenz-Rauental + Tel. 0176 65073477 + Leistungen Schrott/Schrottankauf/-entsorgung aus Gelbe-Seiten-Filialseite (Branchenliste Koblenz + Detailseite gsbiz). Einzige Quelle ein Aggregator (zwei Seiten derselben Website = EINE Quelle): Ausnahme-Fill (offen dokumentiert): name präzisiert zu Schrotthandel R&A, street, postcode, phone; website_status unbekannt (kein Betreiber-Webauftritt, kein HR-Beleg). Kein Bezug zu R und S Laroche GbR (Mülheim-Kärlich) oder anderen Koblenzer Betrieben hergestellt, kein Merge. Status bleibt pruefung; Quelle(n): gelbeseiten.de (Branchenseite + Detailseite, nur Lead)]

### Importiert (Seed-Stand 2026-09-30)

- Schrott
- Adresse: Koblenz 56073
