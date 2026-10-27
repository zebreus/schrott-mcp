---
slug: rp-trier-54294-dres-georg
clear_fields: 'geo'
name: Dres Georg
trader_type: schrotthaendler
state: RP
city: Trier
street: Andreas-Hoevel-Str. 6
postcode: '54294'
phone: 0651 84609
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Dres Georg

## Überblick

Der Seed ist erstmals einem konkreten Trierer Schrotthandelsbetrieb zugeordnet: Drei voneinander unabhängige Branchenverzeichnisse führen denselben Betrieb unter dem Namen „Dres Georg Schrott- und Metallhandel" bzw. „Georg Dres Schrott- und Metallhandel" an **Andreas-Hoevel-Str. 6, 54294 Trier-West**, Telefon **0651 84609**. Der Seed-Name „Dres Georg" und die Seed-Postleitzahl 54294 stimmen damit überein. Diese Belege sind Verzeichniskonsens und nach der Quellenhierarchie Leads, keine Betreiber-Primärquelle: Es gibt keine eigene Website, kein Impressum und keinen Registereintrag, die Statusentscheidung bleibt daher `pruefung`. Der in der Produktionszeile gespeicherte Punkt lag im Trierer Altstadt-Postleitzahl 54290 und widersprach der belegten Anschrift; `clear_fields: 'geo'` zieht ihn zurück.

**ANKAUF:** keine Kurse, keine Materialangaben belegt; die Verzeichnisseinträge nennen nur die Branche „Schrott". **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife.

## Timeline

### Recherche 27.10.2026: Verzeichniskonsenz entschlüsselt Seed-Name und Anschrift

- [Recherche 27.10.2026: Der bislang leere Seed wird durch drei voneinander unabhängige Verzeichniseinträge aufgelöst, die alle denselben Betrieb an derselben Anschrift führen: Gelbe Seiten (Detailseite direkt gelesen) „Dres Georg Schrott- und Metallhandel, Andreas-Hoevel-Str. 6, 54294 Trier-West, 0651 8 46 09, Branche Schrott"; Cylex (Suchindex) „Dres Georg Schrott U. Metallhandel, Andreas-Hoevel-Str. 6, 54294 Trier, Trier-West, 0651 84609, 49.7551698, 6.6220534"; werkenntdenbesten (Suchindex) „Georg Dres Schrott- und Metallhandel, Altmetallrecycling/Altmaterialhandel/Schrotthandel, Andreas-Hoevel-Str. 6, 54294 Trier (Trier-West)". Der Seed-Name „Dres Georg" entspricht der Namensform des Gelbe-Seiten-Eintrags, die Seed-Postleitzahl 54294 der belegten PLZ. Frontmatter-Füllung street „Andreas-Hoevel-Str. 6", postcode „54294", phone „0651 84609"; city von „Trier 54294" auf „Trier" normalisiert (PLZ steht jetzt im Feld postcode). Das sind Verzeichnisbelege, kein Betreiber- oder Registerbeleg; status bleibt `pruefung`. Quellen: https://www.gelbeseiten.de/gsbiz/9a872267-a9c2-4989-809c-1b822a5b03e4 (direkt abgerufen) ; https://web2.cylex.de/firma-home/dres-georg-schrott-u--metallhandel-3976689.html (Suchindex; Direktabruf HTTP 403) ; https://www.werkenntdenbesten.de/schrotthandel/trier .]
- [Recherche 27.10.2026: GEO-Korrektur: Der gespeicherte Punkt 49.7534166666667, 6.63708333333333 löst per Nominatim-Reverse auf „Viehmarktplatz 20, Altstadt, Trier, 54290" auf, also in einen anderen Trierer Postleitzahl als die belegte Anschrift in 54294 Trier-West (Nominatim: 49.7550690, 6.6222452 für Andreas-Hoevel-Straße 6). `clear_fields: 'geo'` entzieht das Alt-Paar beim regulären Seed; das bedeutet nur „keine kuratierte aktuelle Koordinate", keine Schließung und keine Annahmeverweigerung. Eine Ersatzkoordinate wurde nicht geraten, weil die Beleglage nur aus Verzeichnissen besteht und kein Nachweis einer öffentlichen Annahmestelle vorliegt. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.7534166666667&lon=6.63708333333333&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Andreas-Hoevel-Stra%C3%9Fe%206%2C%2054294%20Trier&format=jsonv2&addressdetails=1&limit=3 ; read-only-Abgleich `public.db` (27.10.2026) ; `docs/trader-seed.md`.]
- [Recherche 27.10.2026: **ANKAUF:** keine Kurse und keine Materialangaben; die Verzeichnisseinträge nennen ausschließlich die Branche „Schrott", die Cylex-Fassung ergänzt „Metall, Schrotthändler, Metall Großhandel, Altmetall". **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Tarife. Offen bleibt eine Betreiber-Primärquelle (Impressum, Register- oder Behördenbeleg) sowie die Frage, ob an Andreas-Hoevel-Str. 6 überhaupt eine öffentliche Kundenannahme stattfindet. Quellen: siehe Vorbullet.]
- [Recherche 27.10.2026: Abgrenzung zu anderen Trierer Metallbetrieben: Die ebenfalls in Trier geführten „Schrotthandel & Altmaterialhandel Hutmacher" (Karl-Grün-Str. 8, 54292), „Schrott & Metallhandel Hutmacher", „H.Block Schrotthandel" (Granastraße 8, 54294), „Schrotthandel Bonaventura" (Herresthal 3, 54294), „Schrotthändler MW" (Eifelstr. 4, 54294), „Schrotthandel & Demontagen Anton Dick", „Kochann Uwe Schrotthandel und Metallhandel" (Hornstr. 24) sowie „Georg Dres" sind eigenständige Betriebe; es wurde nichts davon übernommen und nichts zusammengeführt. Namensvetter mit gleichem Ort aber abweichender Nummer sind keine Belege für diesen Seed. Quellen: https://www.gelbeseiten.de/branchenbuch/staedte/rheinland-pfalz/kreisfrei/trier ; https://www.11880.com/suche/schrotthandel/dillingen (Suchindex).]

### Importiert (Seed-Stand 2026-09-30)

- Schrott
- Adresse: Trier 54294
