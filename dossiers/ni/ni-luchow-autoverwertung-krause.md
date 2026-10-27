---
slug: ni-luchow-autoverwertung-krause
name: Autoverwertung Krause
trader_type: autoverwertung
state: NI
city: Lüchow
street: 'Albrecht-Thaer-Str. 16'
postcode: ''
lat: '52.9838646'
lon: '11.1672339'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: aktiv
description: 'Anerkannter Demontagebetrieb für Altfahrzeuge nach AltfahrzeugV am Registerstandort Albrecht-Thaer-Str. 16 in Lüchow (Zertifikat ZAFS00100011). Eine Betreiber-Website oder ein lesbares Impressum existiert nicht; Fahrzeugvergütung, Annahmebedingungen und Gebühren sind nicht veröffentlicht.'
certifications: '["Demontagebetrieb nach AltfahrzeugV (Fachbetrieberegister GESA, Zertifikat ZAFS00100011, Registerstand 11.08.2026)"]'
dropoff_json: ''
pickup_json: ''
provenance_section: Weitere Verzeichnis-Funde (Nienburg/Diepholz/Verden, Osnabrück-Nord,
  Vechta/Cloppenburg, Peine, Stade, Gifhorn, Lüneburg
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# Autoverwertung Krause

## Überblick

Der Betrieb ist im amtlichen Fachbetrieberegister (GESA-Auszug, Stand 11.08.2026) als **anerkannter Demontagebetrieb** „Autoverwertung Krause, Albrecht-Thaer-Str. 16" geführt (Zertifikat ZAFS00100011). Damit ist die zuvor als „nicht exklusiv zuordenbar" notierte Hausnummer jetzt amtlich dem Betrieb Krause zugeordnet; die gleichlautenden Adress-Leads für Autoverwertung Bleck und Boick bleiben unberührt. Offen bleiben die Postleitzahl (Register 29432 gegen Verzeichnis und Adressdaten 29439), die Betreiberidentität ohne Impressum sowie die öffentliche Fahrzeugannahme.

- **Ankauf:** keine bestätigten Fahrzeugvergütungen oder Schrottkurse; das Register führt ausdrücklich keine Preise.
- **Verkauf:** Kleinanzeigen-Ersatzteilangebot nicht als Betreiberseite verifiziert; keine belegte Preisliste.
- **Gebühren:** keine belegten Verwertungs-/Abholtarife.
- **GEO:** adressgenauer OSM-Gebäudepunkt für „16, Albrecht-Thaer-Straße" gesetzt; der bisherige Produktionspunkt lag rund 44 km Luftlinie entfernt in Mecklenburg-Vorpommern.

## Timeline

### Owner-Korrektur 27.10.2026: Distanzangabe der GEO-Korrektur berichtigt

- [Owner-Korrektur 27.10.2026: Die Distanzangabe im GEO-Eintrag vom selben Tag war überhöht: Dort stand „rund 150 km“, gemessen sind es **43,7 km Luftlinie** zwischen dem alten Produktionspunkt (53.3257/10.84365, Butterberg bei Teldau in Mecklenburg-Vorpommern) und dem belegten Standort (52.9838646/11.1672339, Albrecht-Thaer-Straße 16, Lüchow); die Zahl 150 km entstammte der Entfernung zum Ortsmittelpunkt des Zielorts, nicht zum belegten Standort. Die Aussage „anderes Bundesland“ und die Grundsatzentscheidung bleiben davon unberührt, die Strecke ist trotzdem landschaftsfern und der alte Punkt eindeutig falsch. Die Nachmessung ist in `research/wave-20261027-1200-geo-evidence.json` (Abfrage und Abrufdatum 27.10.2026) abgelegt. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&zoom=18&lat=53.3257&lon=10.84365&addressdetails=1 ; research/wave-20261027-1200-geo-evidence.json.]

### Owner-GEO-Nachprüfung 27.10.2026: Kartendatum und Abweichung gemessen

- [Owner-GEO-Nachprüfung 27.10.2026: Der eingetragene Punkt 52.9838646/11.1672339 (OSM-Way 213905139) wurde am 27.10.2026 mit einer Nominatim-Vorwärtssuche der belegten Adresse „Albrecht-Thaer-Straße 16, 29432 Lüchow“ unabhängig nachgemessen: Der Treffer liegt auf 52.9838646/11.1672339, also **0 m Abweichung** zum Dossierwert; der Treffer trägt dieselbe Hausnummer und dieselbe Ortsangabe. Die Abfrage, das Abrufdatum (27.10.2026) und die vollständige Antwort sind in `research/wave-20261027-1200-geo-evidence.json` im Repo abgelegt, damit die Nachmessung aus dem Bestand reproduzierbar ist. Punktart: Gebäude- bzw. Adresspunkt, **keine bestätigte Anlieferzufahrt** und keine Eingangspräzision; das bleibt als offene Restlücke dokumentiert. Der Punkt ersetzt einen nachweislich falsch gesetzten Produktionswert; er ist damit kein kuratierter Standort im Sinne einer Annahmestelle. Quellen: https://nominatim.openstreetmap.org/search (Abfrage in der Evidenzdatei dokumentiert) ; https://www.openstreetmap.org/ ; research/wave-20261027-1200-geo-evidence.json.]

### Recherche 27.10.2026: amtlicher Demontagebetriebs-Nachweis und GEO-Korrektur

- [Recherche 27.10.2026: Amtlicher Fachbetrieberegister neu herangezogen (GESA-Auszug, Stand 11.08.2026): „Autoverwertung Krause, Albrecht-Thaer-Str. 16, 29432 Lüchow" ist dort als anerkannter Demontagebetrieb nach AltfahrzeugV geführt (darf Altfahrzeuge trockenlegen und zerlegen), Zertifikatsnummer ZAFS00100011. Damit ist die bisher nur als Verzeichnislead geführte Anschrift amtlich dem Betrieb Krause zugeordnet und die Demontagezulassung belegt; die gleichlautenden Adress-Leads für Autoverwertung Bleck (05841 709300) und Boick werden nicht übertragen und bleiben getrennte Identitäten. Das Register nennt weder Telefon noch E-Mail und keine Preise. Status daher von `pruefung` auf `aktiv` entsprechend dem Registerbefund korrigiert; das ist keine Aussage über Fahrzeugvergütung oder Annahmezusagen; Quelle(n): https://schrottregister.pages.dev/ort-luchow ; https://schrottregister.pages.dev/betrieb-autoverwertung-krause-luchow]
- [Recherche 27.10.2026: PLZ-Widerspruch bleibt offen und ist deshalb nicht ins Frontmatter übernommen: Das Register nennt 29432, die 11880-Verzeichnisangabe und die adressgenaue OSM-/Nominatim-Adresseingabe zu „16, Albrecht-Thaer-Straße" nennen 29439 — beide Werte liegen in Lüchow. Die Straße gilt als Registerstandort, die Postleitzahl bleibt bis zur Klärung leer. Ferner ist die Mobilnummer 0162 8340276 weiterhin nur ein Verzeichnislead; das Register führt keine Telefonnummer, das Telefonfeld bleibt daher leer; Quelle(n): https://www.11880.com/branchenbuch/luechow-wendland/060440092B113982255/autoverwertung-krause.html ; https://nominatim.openstreetmap.org/search?format=jsonv2&street=Albrecht-Thaer-Stra%C3%9Fe%2016&city=L%C3%BCchow&addressdetails=1]
- [Recherche 27.10.2026: GEO-Korrektur. Die read-only Produktionszeile enthielt 53.3257/10.84365; dieser Punkt löst nach OSM/Nominatim auf „Grabenau, Teldau, Boizenburg-Land, Mecklenburg-Vorpommern (19273)" auf, rund 44 km Luftlinie vom belegten Lüchower Standort entfernt. Ersetzt durch das adressgenaue OSM-Gebäude-Way 213905139 zu „16, Albrecht-Thaer-Straße, 29439 Lüchow": 52.9838646/11.1672339. Es ist ein Gebäude-/Adresspunkt, keine bestätigte Annahmezufahrt; die PLZ-Divergenz zum Register (29432) bleibt dokumentiert; Quelle(n): https://nominatim.openstreetmap.org/search?format=jsonv2&street=Albrecht-Thaer-Stra%C3%9Fe%2016&city=L%C3%BCchow&addressdetails=1 ; https://www.openstreetmap.org/way/213905139 ; read-only `public.db` (27.10.2026).]
- [Recherche 27.10.2026: **Ankauf:** keine Fahrzeugvergütung und keine Schrottkurse belegt. **Verkauf:** keine verifizierte Ersatzteilpreisliste; das Kleinanzeigen-Angebot „VW-T5-Schlachtfest" bleibt ein Verzeichnislead ohne gelesenes Anbieter-Impressum. **Gebühren:** keine Verwertungs-/Abholtarife. Der Feuerwehr-Eigenbericht vom 11.04.2026 nennt Krause weiterhin als Mitgeber eines Übungsgeländes (gemeinsam mit Firma Piskorski, die im Register als „Piskorski GmbH & Co. KG, Albrecht-Thaer-Straße 13" geführt ist) — das ist ein lokaler Existenzhinweis ohne Inhaber- oder Adressbezug und kein Preisfund; Quelle(n): https://www.ffluechow.de/2026/04/11/verkehrsunfall-mit-eingeklemmter-person-nach-kleinbrand/ ; https://schrottregister.pages.dev/betrieb-piskorski-gmbh-co-kg-luchow]

### Recherche 27.10.2026: Verzeichnisangabe gegen Übungsbericht abgeglichen

- [Recherche 27.10.2026: Der 11880-Eintrag nennt weiterhin Albrecht-Thaer-Str. 16, 29439 Lüchow und 0162 8340276 (Eintragsdatum 16.04.2026, Portalaktualisierung 04.10.2026); diese Metadaten sind kein heutiger Betreiberbeleg. Der Feuerwehrbericht vom 11.04.2026 erwähnt Krause ausdrücklich als Mitgeber eines Betriebsgeländes für eine Übung, jedoch ohne Adresse oder Inhaber. Die Portalanschrift bleibt wegen gleichlautender Adress-Leads für Bleck/Boick nicht exklusiv zuordenbar. Kein Fill oder GEO. **Ankauf:** keine Fahrzeugvergütung/Schrottkurse; **Verkauf:** keine verifizierte Ersatzteilpreisliste; **Gebühren:** keine Verwertungs-/Abholtarife. Quellen: https://www.11880.com/branchenbuch/luechow-wendland/060440092B113982255/autoverwertung-krause.html ; https://www.ffluechow.de/2026/04/11/verkehrsunfall-mit-eingeklemmter-person-nach-kleinbrand/ .]

### Recherche 20.10.2026 — Eigenrecherche

- [Recherche 20.10.2026: Neue Suchtreffer zu „Autoverwertung Krause“ wiederholen Albrecht-Thaer-Str. 16/29439 und 0162 8340276 als Verzeichnisangaben. Dieselbe Hausnummer erscheint in Suchtreffern zugleich für „Autoverwertung Bleck“ (05841 709300) und „Boick Autoverwertung“; keine der gefundenen Seiten belegt eine gemeinsame Rechtsperson, Umbenennung oder Nachfolge. Dies ist ein konkreter Adress-/Identitätskonflikt, kein Beweis gegen die Feuerwehrnennung der Krause bei der Übung. Keine Telefon-/Adressübernahme, kein GEO. Ankauf/Verkauf/Gebühren ohne bestätigte Tarife. Quelle(n): https://www.werkenntdenbesten.de/e/113982255/altmetallrecycling/luechow-wendland/autoverwertung-krause-bewertungen.html ; https://autoverwerter.de/autoverwertungen/Niedersachsen/L%C3%BCchow-Dannenberg-3354.html ; https://www.yelp.com/biz/boick-autoverwertung-u-recycling-abschlepp-u-bergungsdienst-l%C3%BCchow.]

### Recherche 18.10.2026 (Welle 0200 B)

- [Recherche 18.10.2026: Feuerwehr-Eigenbericht vom 11.04.2026 direkt gelesen; ausdrücklicher Dank an Autoverwertung Krause und Firma Piskorski für ihre Betriebsgelände, Szenario als Übung gekennzeichnet. Dies ist ein zeitlich konkreter lokaler Hinweis auf einen so benannten Betrieb, aber kein Gewerbe-/Zertifizierungsnachweis und keine Bestätigung der Portalanschrift. Suchleads 11880/Infobel nennen Albrecht-Thaer-Str. 16/29439 Lüchow; Kleinanzeigen zeigt VW-T5-Schlachtfest und Anbieter Autoverwertung Krause, aber kein Anbieter-Impressum gelesen, daher keine Zuordnung oder Preise übernommen. Frontmatter unverändert, pruefung bleibt; keine Unfall-/Brandbehauptung. Ankauf: keine Fahrzeugvergütung; Verkauf: Ersatzteilverkaufslead ohne geprüften Preis; Gebühren: keine belegte Verwertungs-/Abholgebühr. Offen: Betreiber-/Gewerbebeleg, Annahmeadresse und aktuelle Zulassung. Quellen: https://www.ffluechow.de/2026/04/11/verkehrsunfall-mit-eingeklemmter-person-nach-kleinbrand/ ; https://www.11880.com/branchenbuch/luechow-wendland/060440092B113982255/autoverwertung-krause.html ; https://www.kleinanzeigen.de/s-anzeige/vw-t5-1-9-tdi-schlachtfest-viele-ersatzteile-vorhanden/3542208136-223-2351 (letztere Suchleads).]

### Importiert (Seed-Stand 2026-09-30)

- Autoverwertung (Flag)
- urspr. Website-Angabe: keine (nur Verzeichnisbeleg)
