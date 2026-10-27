---
slug: ni-bad-grund-schmidt-schmidt-rohprodukte-altmetalle
clear_fields: 'geo'
name: Schmidt + Schmidt Rohprodukte Altmetalle
trader_type: schrotthaendler
state: NI
city: Bad Grund
street: Sägemühlenstr. 1
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: 'Verzeichnis-Kandidat mit zwei widersprüchlichen Anschriften (Sägemühlenstr. 1, Teichhütte und Am Steinbühl 2, jeweils 37539 Bad Grund); keine Betreiber- oder Registeridentität, kein belegter Metallankauf. Der Seed-Doppelname ist vermutlich ein Verzeichnisartefakt (Das Örtliche führt an Am Steinbühl 2 den Personeneintrag Schmidt Kerstin u. Egon). Laut Register (Stand 11.08.2026) ist in Bad Grund nur MPM Environment Intelligence GmbH, Bahnhofstraße 3a, geführt.'
dropoff_json: ''
pickup_json: ''
provenance_section: Neue Funde (Gelbe Seiten, je Stadt abgefragt)
provenance_ankauf_raw: Harzrand-Stütze Bestand
provenance_origin: table
---

# Schmidt + Schmidt Rohprodukte Altmetalle

## Überblick

Der direkt gelesene Gelbe-Seiten-Eintrag heißt „Schmidt Rohprodukte Altmetalle“ (ohne Plus) und nennt Sägemühlenstr. 1, Teichhütte, Telefon 05327 8698701. Ein direkt gelesener 11880-Eintrag unter demselben Namen führt dagegen Am Steinbühl 2 in Bad Grund und Telefon 05327 4425/5606; Eintragsdaten 15.06.2023, Portaländerung 01.10.2026. Die beiden Adress-/Telefonvarianten und der Dossiername „Schmidt + Schmidt“ sind nicht über Betreiber/Registerbelege verbunden.

- **Ankauf:** Verzeichnisse nennen nur Branchenrubriken; keine verifizierte Kaufzusage oder numerischen Kurse.
- **Verkauf:** keine Produkt-/Preisliste.
- **Gebühren:** keine Entsorgungs-/Abholtarife.
- **GEO:** weder Sägemühlenstr. 1 noch Am Steinbühl 2 ohne Identitäts-/Adressklärung als Annahmepunkt geokodieren.

## Timeline

### Recherche 27.10.2026 (Welle Shard C, NI)
- [Owner-Korrektur 27.10.2026: Die Entfernungsangabe dieser Notiz war falsch: Der Produktionspunkt 52.180721875/9.673703125 (Nominatim-Rückwärtsauflösung „L 422, Eldagsen, Holtensen, Springe, 31832“) liegt **rund 56 km** (56,4 km, Haversine gegen die Nominatim-Vorwärtssuche „Bad Grund, 37539, Deutschland“ → 51,80977/10,23630) vom Zeilenort Bad Grund (Harz) entfernt, nicht 32 km. Der **Ortsnachweis** bleibt davon unberührt: Der Punkt liegt in einer anderen Gemeinde (Region Hannover) und damit in keinem Bad-Grunder Standort; die Rücknahme des GEO-Paares per `clear_fields: 'geo'` ist damit weiterhin belegt. Quelle(n): https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=52.180721875&lon=9.673703125&zoom=14&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Bad+Grund%2C+37539%2C+Deutschland&format=jsonv2&limit=1 ; read-only `public.db` (27.10.2026).]

- [Recherche 27.10.2026: Amtlicher Registerabgleich neu (Stand des Registerabrufs 11.08.2026): Für Bad Grund (37539) führt das Verzeichnis genau einen Betrieb — „MPM Environment Intelligence GmbH“, Bahnhofstraße 3a, 37539 Bad Grund Gittelde, Tätigkeiten Lagern/Behandeln/Verwerten/Handeln/Makeln, 62 Abfallschlüssel im Zertifikat (u. a. 020110 Metallabfälle, 160104* Altfahrzeuge). „Schmidt Rohprodukte Altmetalle“ wird weder für Sägemühlenstr. 1 noch für Am Steinbühl 2 verzeichnet: Das Register nennt den Betrieb nicht — bei der Reichweite dieser Register (Fachbetrieberegister Altfahrzeuge, Register der Entsorgungsfachbetriebe) ist das **weder** ein Nichtexistenz- **noch** ein Schließungsbeleg; der in Bad Grund registrierte Metall-/Fahrzeugbetrieb trägt einen anderen Namen und wird nicht mit diesem Dossier zusammengeführt. NorthData live am 27.10.2026: weder „Schmidt + Schmidt“ noch „Schmidt Rohprodukte“ in Bad Grund auffindbar; die Namensvetter-Liste führt „Schmidt & Schmidt GmbH“, Olsberg, „Schmidt & Schmidt eGbR“ in Geestland/Hamburg/Itzehoe/Hamminkeln/Schwerte sowie weitere Schmidt-Gesellschaften — keine Übernahme. Beide Verzeichnis-Leads sind unverändert live und widersprechen einander weiterhin: Gelbe Seiten nennen Sägemühlenstr. 1, 37539 Bad Grund (Harz)-Teichhütte, 05327 8698701; 11880 nennt Am Steinbühl 2, 37539 Bad Grund, 05327 4425 und 05327 5606 (Eintragsdaten 15.06.2023, Portaländerung 01.10.2026). Beide stammen aus derselben Quellenklasse (Telefonbuch-/Verzeichnisdaten), sind also **keine** zwei unabhängigen Belege; der Namenszusatz „+ Schmidt“ bleibt ungeklärt. Ein amtlicher Gewerberegister-Zugriff für Bad Grund war nicht erreichbar (Stadtseite badgrund.de leitet auf kurmittelhaus-harz.de um, keine öffentliche eAuskunft gefunden) — Identität bleibt offen. street-Feld unverändert (Altwert). GEO: Die read-only Produktionszeile enthält den Punkt 52.180721875/9.673703125; die Nominatim-Rückwärtsauflösung ordnet ihn „L 422, Eldagsen, Holtensen, Springe, 31832“ zu — rund 32 km vom Zeilenort Bad Grund (Harz) entfernt und in einer anderen Gemeinde (Region Hannover). Der Punkt kann also kein Bad-Grunder Standort sein und wird mit `clear_fields: 'geo'` zurückgenommen; das behauptet weder, dass eine der beiden Verzeichnisanschriften stimmt, noch dass der Betrieb geschlossen ist, und es wird **keine** Ersatzkoordinate gesetzt (Identität ungeklärt, nur eine Quelle). ANKAUF/VERKAUF/GEBÜHREN: keine Kurse, keine Verkaufsliste, keine Tarife. Quellen: https://schrottregister.pages.dev/ort-bad-grund ; https://schrottregister.pages.dev/betrieb-mpm-environment-intelligence-gmbh-bad-grund ; https://www.northdata.de/suggest.json?query=Schmidt+Schmidt&countries=DE ; https://www.gelbeseiten.de/gsbiz/05d791dc-41da-43a5-86ff-a676f51beabd ; https://www.11880.com/branchenbuch/bad-grund-harz/120674719B377410/schmidt-rohprodukte-altmetalle.html ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=52.180721875&lon=9.673703125&zoom=14&addressdetails=1 ; read-only `public.db` (27.10.2026)]

- [Recherche 27.10.2026: Namensvetter mit Hausnummernbezug neu: Das Örtliche führt an **derselben Hausnummer** Am Steinbühl 2, 37539 Bad Grund (Harz)-Windhausen den Personeneintrag „Schmidt Kerstin u. Egon“ und daneben (Rückwärtssuche zur Nummer 05327 4425) den Brancheneintrag „Schmidt Rohprodukte Altmetalle, Recycling“ ohne weitere Anschrift. Damit gibt es eine belegte Erklärungshypothese für den Seed-Namen „Schmidt + Schmidt“ (zwei Personen namens Schmidt an einer Adresse), ohne dass daraus folgt, dass ein Betrieb mit diesem Namen besteht oder dass die beiden Adressvarianten (Sägemühlenstr. 1/05327 8698701 gegen Am Steinbühl 2/05327 4425) denselben Betrieb beschreiben. Für 05327 8698701 und 05327 5606 liefert die Rückwärtssuche von Das Örtliche keinen Treffer (nur den Ortsbezug Bad Grund (Harz)/Seesen). Beide Verzeichnisse bleiben Aggregator-Leads derselben Quellenklasse; der Widerspruch zwischen den beiden Anschriften ist damit **nicht** aufgelöst, der Seed-Doppelname „+ Schmidt“ ist als Verzeichnisartefakt sehr wahrscheinlich, aber nicht bewiesen. Kein Fill, kein Merge, keine Preise, keine Tarife. Quellen: https://www.dasoertliche.de/Themen/Schmidt-Kerstin-u-Egon-Bad-Grund-Harz-Windhausen-Am-Steinb%C3%BChl ; https://www.dasoertliche.de/?form_name=search_inv&ph=053274425 ; https://www.dasoertliche.de/?form_name=search_inv&ph=053278698701 ; https://www.dasoertliche.de/?form_name=search_inv&ph=053275606]

### Recherche 20.10.2026 — Eigenrecherche

- [Recherche 20.10.2026: Den 11880-Detailtreffer „Schmidt Rohprodukte Altmetalle“ direkt gelesen. Er führt Am Steinbühl 2, 37539 Bad Grund, Telefon 05327 4425 und weitere Rufnummer 05327 5606; Eintragsdaten laut Seite 15.06.2023, aktualisiert 01.10.2026. Das widerspricht dem direkt gelesenen Gelbe-Seiten-Eintrag Sägemühlenstr. 1/05327 8698701. Beide sind Telefonbuch-/Verzeichnisbelege, kein Nachweis einer Verlegung, Filiale oder Betreiberkette. Namensvariante „Schmidt“ löst den Seed-Zusatz „+ Schmidt“ nicht. Kein Fill/kein GEO; keine Ankauf-, Verkaufs- oder Gebührenpreise. Quelle(n): https://www.11880.com/branchenbuch/bad-grund-harz/120674719B377410/schmidt-rohprodukte-altmetalle.html ; https://www.gelbeseiten.de/gsbiz/05d791dc-41da-43a5-86ff-a676f51beabd.]

### Importiert (Seed-Stand 2026-09-30)

- GS-Einträge

### Recherche 05.10.2026

- [Recherche 05.10.2026: Seed-Name UND Seed-Anschrift Sägemühlenstr. 1 gegengeprüft, keine belastbar zugeordnete Betreiberwebsite/Registergesellschaft gefunden. Die einzeln live abgerufene NorthData-Abfrage Schmidt Schmidt/Bad Grund lieferte eine Personenseite Beatrice Schmidt, Bad Grund (Harz), mit Schul-/Kita-Fördervereinsfunktionen, keinen Rohprodukte-/Altmetallhandel. Diese automatische Namensauflösung nicht als Firmenbeleg verwendet. Websuchvarianten zu Rohprodukten/Altmetallen und Sägemühlenstraße lieferten unscharfe andere Schmidt-Betriebe; Suchtreffer sind keine verifizierten Seitenabrufe. Keine Aussage, dass es einen nicht registerpflichtigen örtlichen Händler nicht gibt. Quelle(n): https://www.northdata.de/Schmidt+Schmidt,+Bad+Grund (live abgerufene, unpassende Personenseite)]
- [Recherche 05.10.2026: Vorhandene Straße als ungeklärten Altwert erhalten, keine PLZ aus dem Ort abgeleitet und keine fremden Schmidt-Kontaktdaten übertragen. Ohne Betreiberidentität keine einzeln abrufbaren Händler-Unterseiten für Impressum/Kontakt/Leistungen/Preise/Zeiten/Zertifikate; Zwei-Quellen-Standard und Betreiber-Ausnahme nicht erfüllt. Owner: vollständige Inhabernamen, Grundstücks-/Gewerbebeleg und heutige Annahme an Sägemühlenstr. 1 beschaffen. Kein Fill, status pruefung unverändert, keine Schließung behauptet, keine verifizierte Ankauf-/Verkaufs-/Gebührenliste. Quelle(n): https://www.northdata.de/Schmidt+Schmidt,+Bad+Grund (kein Händlerbeleg)]

### Recherche 08.10.2026

- [Recherche 08.10.2026: Original-Verzeichnislead einzeln live HTTP 200 gelesen: Gelbe Seiten nennt Schmidt Rohprodukte Altmetalle (kein Plus-/Doppelname), Sägemühlenstr. 1, 37539 Bad Grund (Harz)-Teichhütte, 05327 8698701; Leistungskategorien Altmetall/Altschrott/Schrottentsorgung, ohne Betreiberwebsite/Kaufzusage/Preise. Orts-/Straßensuche löst den Seed-Namenszweifel nicht mit qualifizierter Primärquelle, daher Name/PLZ/Telefon unverändert. Keine Beatrice-Schmidt- oder Haustechnik-Zuordnung. Owner: Gewerbeidentität und Entstehung des Doppelnamens prüfen, statt ausschließlich nach Schmidt + Schmidt zu suchen. Ankauf-, Verkauf-, Gebührenliste weiterhin nicht verifiziert. Quelle: https://www.gelbeseiten.de/gsbiz/05d791dc-41da-43a5-86ff-a676f51beabd (Lead, kein Beleg nach README)]
