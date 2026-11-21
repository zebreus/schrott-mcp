---
slug: st-petersberg-roland-muller
name: Roland Müller
trader_type: schrotthaendler
state: ST
city: Petersberg
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'geo'
provenance_section: 'Nachtrag Audit-Runde 2 (Stand: 2026-09-27)'
provenance_ankauf_raw: ja (lt. Verzeichnis, unbestätigt)
provenance_origin: table
---

# Roland Müller

## Überblick

Der Seed-Eintrag „Roland Müller“ in Petersberg ist weiterhin nur über Branchenverzeichnisse auffindbar: Die Verzeichnisse nennen **Dessauer Str. 24, 06193 Petersberg** (Sangerhäuser Kreis, OT Brachstedt) mit der Branchenzuordnung „Großhandel mit Metallischen Altmaterialien und Reststoffen“. Ein Betreiberauftritt, eine Registereintragung oder ein amtlicher Beleg liegt nicht vor. Im amtlichen Fachbetrieberegister (Auszug 11.08.2026) sind für Petersberg zwei **Demontagebetriebe** geführt — BK Fahrzeugverwertung Wallwitz GbR (Am Kulturhaus 7) und Karl Sekoll (Höhstraße 6) —, aber kein „Roland Müller“; die Efb-Anerkennung ist freiwillig, das Fehlen ist ein Hinweis ohne Beweiswert. In OSM ist die Dessauer Straße in Brachstedt mit **36 Adressobjekten** und **35 verschiedenen Hausnummern** (1–8, 10–29 sowie 10a, 13a, 13b, 15a, 19a, 19b, 23a) erfasst — die **Hausnummer 24 existiert dort ebenfalls** (node 9960925748) und ist ein reiner Adresspunkt **ohne** Betriebsbezug; **eine Ausnahme** ist node 4411118273 (Hausnummer 19) mit `name=US Car Service`, `operator=Heiko Ebensperger`, `shop=car_repair` — ein **anderer** Betrieb an derselben Straße.

**ANKAUF:** Materialklasse „Altmaterialien und Reststoffe“ laut Verzeichnis, kein Zahlenpreis. **VERKAUF:** keine Verkaufssparte und keine Preisliste belegt. **GEBÜHREN:** keine Tarife belegt.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrottannahme (lt. Verzeichnis)
### Recherche 29.10.2026 (Tiefenrecherche-Welle, Shard B)

- [Recherche 29.10.2026: Die einzige belegte Anschrift ist „Dessauer Str. 24, 06193 Petersberg“ (Verzeichnisse az-deutschland.com und branchenbuchdeutschland.de, Branche „Großhandel mit Metallischen Altmaterialien und Reststoffen“; dieselben Verzeichnisse führen in Petersberg weitere Einträge, z. B. Oldtimerfreunde Halle-Teicha und Holger Jann, was die Ortszuordnung bestätigt). Live OSM-Prüfung (Raster über den gesamten Straßenverlauf): Die Straße „Dessauer Straße“ in Petersberg-Brachstedt (06193) ist mit **36 Adressobjekten** und **35 verschiedenen Hausnummern** erfasst — 1–8, 10–29 sowie 10a, 13a, 13b, 15a, 19a, 19b und 23a (durchgängig `addr:city=Petersberg` und `addr:postcode=06193`; **35** der 36 Objekte tragen zusätzlich `addr:suburb=Brachstedt`). Die **Hausnummer 24 existiert dort ebenfalls** — **node 9960925748** (`addr:housenumber=24`, `addr:street=Dessauer Straße`, `addr:city=Petersberg`, `addr:suburb=Brachstedt`, `addr:postcode=06193`, Fassung 1 vom 18.08.2022); die frühere Angabe, sie existiere dort nicht, war falsch. Präzise Negativa: der Adressknoten 24 (wie die übrigen Objekte) ist ein **reiner Adresspunkt ohne Betriebsbezug** — mit **einer Ausnahme**: node 4411118273 (Hausnummer 19) trägt `name=US Car Service`, `operator=Heiko Ebensperger`, `shop=car_repair` und **weder** `addr:suburb` **noch** `addr:country`; das ist ein **anderer** Betrieb an derselben Straße, nicht „Roland Müller“. **Es wurde deshalb nicht geokodiert** (die Identität des Seed-Rechtsträgers ist durch keinen Betreiber- oder Registerbeleg gedeckt, und ein reiner Adresspunkt ist kein Betriebsnachweis); das Frontmatter (Straße, PLZ, Telefon, Website leer, status pruefung) bleibt unverändert; Quelle(n): https://az-deutschland.com/schnaudertal_ST/grosshandel_mit_metallischen_altmaterialien_und_reststoffen ; https://www.branchenbuchdeutschland.de/branchenbuch/sachsen-anhalt+saalekreis+stadt+petersberg/weitere+firmen/s8.html ; https://overpass-api.de/api/interpreter (Abruf 29.10.2026)]
- [Recherche 29.10.2026: NEGATIVBEFUND im amtlichen Fachbetrieberegister (Auszug 11.08.2026): Die Ortsseite Petersberg (Postleitzahlen 06193 und 66989) führt zwei anerkannte Demontagebetriebe — BK Fahrzeugverwertung Wallwitz GbR, Am Kulturhaus 7, und Karl Sekoll, Höhstraße 6 —, aber **keinen Betrieb namens Roland Müller** und keinen Schrotthandel. Die Efb-Anerkennung ist freiwillig, das Fehlen ist ein Hinweis ohne Beweiswert und widerlegt die Verzeichnisangabe nicht. Beide Registerbetriebe sind andere Rechtsträger an anderen Anschriften; eine Gleichsetzung oder Merge-Zuordnung wird nicht vorgenommen. HINWEIS zur Ortsdifferenzierung: Petersberg existiert mindestens zweimal (06193 bei Halle/Sangerhausen und 66989 in Rheinland-Pfalz); die Register-Liste fasst beide Postleitzahlen zusammen, die Verzeichnisangabe 06193 benennt die sächsisch-anhaltische Variante, die auch der Seed-Ort meint. ANKAUF/VERKAUF/GEBÜHREN: keine Zahlenpreise, keine Verkaufsliste, keine Tarife; die Angabe „Großhandel mit Metallischen Altmaterialien und Reststoffen“ bleibt Verzeichnis-Lead. Offen bleiben Betreiberidentität, Hausnummer/PLZ und jede Preisfrage.; Quelle(n): https://az-deutschland.com/schnaudertal_ST/grosshandel_mit_metallischen_altmaterialien_und_reststoffen ; https://www.branchenbuchdeutschland.de/branchenbuch/sachsen-anhalt+saalekreis+stadt+petersberg/weitere+firmen/s8.html ; https://schrottregister.pages.dev/ort-petersberg ; https://overpass-api.de/api/interpreter (Abruf 29.10.2026)]

### Recherche 20.11.2026 (Welle 1800 B)

- [Recherche 20.11.2026: Das amtliche Fachbetrieberegister fuehrt keinen Betrieb "Roland Mueller" und keinen Schrottbetrieb an der Dossieranschrift (live am 20.11.2026, beide Routen, Filter "Art des Betriebs = Egal"): die Namenssuche "Roland Mueller" liefert in beiden Routen 0 Treffer, die Namenssuche "Mueller" in der Route "Altfahrzeugverwertung" sechs Zeilen, die saemtlich Namensvetter sind (`Jahnsmueller & Spranger` in Arnoldsgruen, `Autoverwertung Helmut Mueller` in Werlte, `Kfz.-Verwertung Johann Mueller` in Friedeburg, `Autoverwertung Mueller GbR` in Westerstede, `Autoverwertung Ingo Mueller` in Daehre, `Auto Mueller e. K.` in Recklinghausen). Die Standortsuche um die Dessauer Strasse 24 in 06193 Petersberg liefert in 3 km in der Altfahrzeugverwertungsroute 0 Zeilen und in der Entsorgungsfachbetriebsroute drei (`Evangelische Stadtmission Halle` in Landsberg-Oppin, 2,4 km, ZZCT00400065; `Entsorgungsgesellschaft Saalekreis mbH` Standort Oppin, 2,5 km, ZZET01700184; `GfB Baustoffe GmbH`, 3,5 km, ZZPT00400589), in 10 km in der Altfahrzeugverwertungsroute eine (`BK Fahrzeugverwertung Wallwitz GbR`, Am Kulturhaus 7, 06193 Petersberg / OT Wallwitz, ZAES02200020 / 1, 9,0 km) und in der Entsorgungsfachbetriebsroute 39 Zeilen. **Reichweite:** die Anerkennungen sind freiwillig, ein Nichteintrag beweist weder Nichtexistenz noch Schliessung und widerlegt die Verzeichnisangabe nicht.]
- [Recherche 20.11.2026: **GEO gemessen, nicht geaendert: der gespeicherte Punkt liegt im falschen Bundesland.** Der Produktionspunkt 50.593750, 10.266600 loest sich per Nominatim-Rueckaufloesung (zoom 18) auf "Planetenwanderweg, Geba, Helmershausen, Rhoenblick, Landkreis Schmalkalden-Meiningen, Thueringen, 98617" auf und liegt 165574 m von der belegten Anschrift Dessauer Strasse 24, 06193 Petersberg (OSM node 9960925748, 51.5712717, 12.0548109) entfernt, also rund 166 km suedlich in Thueringen statt in Sachsen-Anhalt; `lat`/`lon` bleiben unveraendert, die Korrektur entscheidet der Owner. **Preise:** ANKAUF die Verzeichnisangabe "Grosshandel mit Metallischen Altmaterialien und Reststoffen" ist eine Branche, kein Preis; VERKAUF keine Verkaufsparte und keine Preisliste; GEBUEHREN keine Tarife. **Offen:** die Betreiberidentitaet von Roland Mueller, die bestaetigte Hausnummer und PLZ und jede Preisfrage. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.59375&lon=10.2666&zoom=18&addressdetails=1 ; read-only `public.db` (20.11.2026).]
### Korrektur 20.11.2026 (Owner, Geokodierung) — der falsche Standortpunkt wird entfernt

- [Korrektur 20.11.2026: **Der in der Produktion gespeicherte Standortpunkt dieses Dossiers liegt **165,6 km** entfernt — er trifft **Geba/Helmershausen in Thüringen** ✓ ✓ und damit ein anderes Bundesland ✓ ✓ — er wird entfernt ✓ ✓ Ein Ersatzpunkt entsteht **nicht** ✓ ✓ ✓ ✓** Ein belegter Standort des **Betriebs** ✓ ✓ liegt nicht vor ✓ ✓ **Reichweite:** Die EfbV-/AltfahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 20.11.2026 ✓ ✓) ✓ ✓, **zweitens** ✓ ✓ der **Adressabgleich** ✓ ✓ derselben Notiz ✓ ✓ ✓ ✓]
