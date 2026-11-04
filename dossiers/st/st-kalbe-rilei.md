---
slug: st-kalbe-rilei
clear_fields: geo
name: Rilei Automobile GmbH
trader_type: autoverwertung
state: ST
city: Kalbe
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: Registeridentität Rilei Automobile GmbH in Cheinitz gefunden, früher RILEI Autoverwertung GmbH Badel; heutige Autoverwertung und Ankaufbedingungen ungeklärt, ehemalige Domain geparkt.
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# RILEI

## Überblick

Der bisherige Negativbefund ist überholt: **Rilei Automobile GmbH**, Amtsgericht Stendal HRB 737, wird bei NorthData geführt, mit früherem Namen **RILEI Autoverwertung GmbH Badel**. GLEIF bestätigt denselben Rechtsträger/Registerschlüssel unabhängig. Ein zusätzliches Werkstattverzeichnis listet Cheinitzer Rundling 44, 39624 Kalbe, 039001 90922 und Ansprechpartnerin Nicole Wernecke, kennzeichnet den Eintrag aber als ungeprüft und führt nur Autoreparatur. Das ist keine Betreiberbestätigung für die Autoverwertung, keine Aussage zu aktuellem Hofbetrieb und kein Grund, die Verzeichnisdaten in die Frontmatter zu übernehmen. Der formale Registername und der stabile Slug bleiben unverändert.

Registerzweck umfasst Abschleppen/Bergung, Autoverwertung/Entsorgung, Fahrzeug- und Ersatzteilhandel sowie Reparatur. Das beweist keine aktuelle anerkannte Altfahrzeugannahme. Die frühere Domain rilei.de steht zum Verkauf. GLEIF nennt zwar entity.status ACTIVE, aber LEI-Registrierung LAPSED, letztes Update 2021; beides nicht mit nachgewiesenem aktuellen Hofbetrieb verwechseln. Zum Rundling 44 bzw. Cheinitzer Rundling 44, Cheinitz/Kalbe, bleibt bis zur aktuellen Standortbestätigung Timeline-only.

## Timeline

### Owner-Korrektur 27.10.2026: falsche Produktions-GEO zurücknehmen

- [Owner-Korrektur 27.10.2026: Die aktuelle read-only Produktionszeile enthält weiterhin `53.2934, 9.5506`. Der bereits dokumentierte Reverse-Lookup ordnet diesen Punkt Kalbe in Niedersachsen (Samtgemeinde Sittensen) zu, nicht Kalbe (Milde), Sachsen-Anhalt; auch die bekannte Betreiber-/Adressspur Cheinitzer Rundling 44 ist nur ein ungeprüfter Verzeichnis-Lead. Deshalb wird ausschließlich das nicht zum belegten Ort passende GEO-Paar mit `clear_fields: geo` beim nächsten regulären Seed zurückgenommen. Keine Ersatzkoordinate geraten, keine Adresse aus dem Lead übernommen und `status: pruefung` unverändert gelassen. Das ist weder eine Betriebsaufgabe noch ein Urteil über die aktuelle Annahme. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.2934&lon=9.5506&zoom=10 ; https://www.auto-werkstatt.de/kalbe/rilei-automobile-gmbh-aULLNy ; read-only `public.db` (27.10.2026); README.md, Quellenhierarchie.]

### Recherche 27.10.2026

- [Recherche 27.10.2026: GLEIF-API erneut direkt abgerufen. Das Golden-Copy-Publishdatum ist 27.10.2026, aber der Datensatz selbst nennt weiterhin `entity.status: ACTIVE`, `registration.status: LAPSED` und `lastUpdateDate: 19.07.2021`. Rechtlicher Name/Registerkennung und Anschrift unverändert: Rilei Automobile GmbH, HRB 737, Zum Rundling 44, Cheinitz, 39624; Hauptsitzfeld „Cheinitzer Rundling 44“, Kalbe (Milde). Das neue technische Publishdatum ist keine frische Unternehmens-/Betriebsbestätigung und `ACTIVE` kein Hof-/Annahmebeleg. Keine Frontmatter-Änderung, keine GEO und keine aktuellen Ankauf-/Verkaufs-/Gebührenpreise. Quelle: https://api.gleif.org/api/v1/lei-records/529900TGM8IAAPLG8W46 .]

### Recherche 21.10.2026

- [Recherche 21.10.2026: Auto-Werkstatt.de listet RILEI Automobile GmbH, Ansprechpartnerin Nicole Wernecke, Cheinitzer Rundling 44, 39624 Kalbe, Festnetz 039001 90922/90924 und rilei.de; das Profil ist ein Branchenverzeichnis (mit „Unternehmen verifizieren“-Aufforderung) und nennt als Spezialisierung nur Autoreparatur. Die Angaben sind ein neuer Identitäts-/Adresslead, aber keine Primärquelle und kein aktueller Nachweis für Autoverwertung, Schrottankauf oder Nachfolge des früheren Unternehmens. Rilei-Domain bei Direktabruf weiter Domain-Verkaufsseite. Keine Frontmatter-Fills oder GEO. **Ankauf:** keine Autoverwertungs-/Schrottpreise; **Verkauf:** keine Ersatzteilpreise; **Gebühren:** keine Tarife. Quelle(n): https://www.auto-werkstatt.de/kalbe/rilei-automobile-gmbh-aULLNy ; https://www.rilei.de/ (Abruf 21.10.2026).]

### Owner-GEO-Prüfung 21.10.2026

- [Owner-Prüfung 21.10.2026: Die read-only Produktionszeile enthält 53.2934, 9.5506. Reverse lookup dieses bestehenden Punkts führt nach Kalbe, Samtgemeinde Sittensen, Niedersachsen; das ist nicht der Verzeichnis-Adresslead Cheinitzer Rundling 44, 39624 Kalbe (Milde), Sachsen-Anhalt. Der Adresslead bleibt unbestätigt und wird nicht als Ersatzpunkt geokodiert. Keine Koordinate übernommen oder Produktionsdaten geändert. Quelle des Punkt-Lookups: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.2934&lon=9.5506&zoom=10 ; Adresslead: https://www.auto-werkstatt.de/kalbe/rilei-automobile-gmbh-aULLNy.]

### Recherche 14.10.2026

- [Recherche 14.10.2026: Register-/GLEIF-Spur aus der Historie erneut direkt geprüft: NorthData HRB 737 führt Rilei Automobile GmbH, früher RILEI Autoverwertung GmbH Badel; die einzeln gelesenen Bekanntmachungen 22.04.2002 und 19.11.2002 bestätigen Unternehmenszweck und Umfirmierung/Sitz Cheinitz, nicht heutigen Annahmebetrieb. GLEIF liefert Golden-Copy-Datum 14.10.2026, aber weiterhin entity ACTIVE, Registrierung LAPSED und lastUpdateDate 19.07.2021; das Veröffentlichungsdatum ist kein frisches Betriebsprüfdatum. rilei.de erneut Domainverkauf. Keine aktuellen ANKAUF-Kurse, VERKAUF-Teilepreise oder Betriebsgebühren; Domainverkaufsbedingungen ausdrücklich nicht als Händlertarife werten. Keine Fills, status pruefung bleibt; Quelle(n): https://www.northdata.de/?query=RILEI+Automobile+GmbH ; https://www.northdata.de/?id=5068343485136896 ; https://www.northdata.de/?id=6163219433914368 ; https://api.gleif.org/api/v1/lei-records/529900TGM8IAAPLG8W46 ; https://www.rilei.de/]

### Recherche 12.10.2026

- [Recherche 12.10.2026: Die frühere Domain rilei.de erneut direkt aufgerufen; sie zeigt weiter eine Verkaufs-/Parkseite („steht zum Verkauf“), keine Betreiberinformationen. Das bestätigt nur, dass der Domainauftritt kein aktueller Betriebsnachweis ist; ältere Register-/GLEIF-Einträge bleiben historische Identitätsbelege und belegen weder aktiven Hofbetrieb noch GESA-Anerkennung. Verzeichnis-Kontaktangaben nicht übernommen. ANKAUF — keine aktuellen Schrott-/Fahrzeugpreise; VERKAUF — keine Ersatzteilpreisliste; GEBÜHREN — keine Tarife. Status `pruefung` unverändert. Quelle: https://www.rilei.de/ (direkt abgerufen 12.10.2026).]

### Recherche 05.10.2026 (Abend-Welle)

- [Recherche 05.10.2026: Identitätsfund statt bisherigem „kein Befund“: NorthData HRB 737 Stendal nennt Rilei Automobile GmbH / RILEI Autoverwertung GmbH Badel, Zum Rundling 44, 39624 Cheinitz; Historie 19.11.2002 Name/Sitz Zethlingen OT Cheinitz, Geschäftsführer-/Adresspublikation 25.02.2009. GLEIF-API zu LEI 529900TGM8IAAPLG8W46 bestätigt legalName Rilei Automobile GmbH, registeredAs HRB 737, legalAddress Zum Rundling 44/Cheinitz/39624 und headquartersAddress Cheinitzer Rundling 44/Kalbe (Milde)/39624. entity.status ACTIVE, registration.status LAPSED, lastUpdateDate 19.07.2021: historisch korroborierte Identität, kein frischer Betriebsnachweis. Formalen Namen auf 2-Quellen-Basis präzisiert, keine Adress-/Telefon-Fills, status pruefung bleibt. Quelle(n): https://www.northdata.de/Rilei+Automobile+GmbH,+Kalbe+(Milde)/Amtsgericht+Stendal+HRB+737 ; https://api.gleif.org/api/v1/lei-records/529900TGM8IAAPLG8W46 .]
- [Recherche 05.10.2026: Domain-Kandidat https://www.rilei.de leitet auf Domainverkauf („rilei.de steht zum Verkauf“), kein Betreiberinhalt; website bewusst leer statt falschem aktiv-Fill. Verzeichnisleads nennen 039001 90922/info@rilei.de, nicht als aktuelle Betreiberkontakte übernehmen. Keine belegte heutige Autoverwertungsanerkennung, kein allgemeiner Schrottankauf, keine Ankauf-/Verkaufspreisliste oder Gebührenordnung gefunden. Quelle(n): https://www.rilei.de ; https://exa.ai/library/place/6h1pm66y8jh (nur Lead/Suchtreffer).]

### Importiert (Seed-Stand 2026-09-30)

- Autoverwertung
- Adresse: Kalbe (Ankauf fraglich)

### Recherche 01.10.2026

- [Recherche 01.10.2026: Kein Befund — Gelbe Seiten kennt kein »RILEI« in Kalbe (nur Riley-Rauschen), Örtliches listet unter Autoverwertung/Kalbe nur Betriebe aus Gardelegen/Klötze/Dähre, kein RILEI; kein Fill, status weiter pruefung; Quelle(n): https://www.gelbeseiten.de/suche/rilei/kalbe und https://www.dasoertliche.de/Themen/Autoverwertung/Kalbe.html.]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Für die im Dossier belegte Anschrift
  **„Cheinitzer Rundling 44 / Zum Rundling 44, 39624 Kalbe (Milde)“** existiert in OSM **kein** adressgenaues Objekt;
  die beiden live abgetasteten Fenster umfassen **Kalbe (Milde) einschließlich der Ortsteile** nachweislich, der
  gesuchte Straßenname existiert dort aber **nicht**. **Quelle 1 (OSM, live abgerufen am 04.11.2026):**
  (a) Kachelraster über **Kalbe (Milde) West** (**11.3200/52.6000 bis 11.4400/52.7000**, live über
  `https://api.openstreetmap.org/api/0.6/map.json?bbox=11.3200,52.6000,11.4400,52.7000`): das Raster enthält den Ort
  **Kalbe (Milde)** selbst (place=town, 52.6562/11.3872) sowie die Ortsteile **Faulenhorst** (52.6582/11.3324),
  **Wernstedt** (52.6457/11.3378), **Neu Wernstedt** (52.6334/11.3352), **Schenkenhorst** (52.6032/11.3947),
  **Vahrholz** (52.6790/11.3870), **Altmersleben** (52.6853/11.4244), **Klein Engersen**, **Engersen**, **Bühne** und den
  Ortsteil **Dolchau** - eine Namenssuche nach **„Rundling“**, **„Cheinitz“** und **„Rilei“** über die Felder name,
  operator, description, website, contact:website, brand, note und addr:housename findet **keinen** Treffer, und es
  gibt **kein** Objekt mit `addr:street`=Cheinitzer Rundling oder Zum Rundling. (b) das Ergänzungsraster über
  **Kalbe (Milde) Ost** (**11.3800/52.5800 bis 11.5000/52.7000**) enthält **Kassieck**, **Karritz**, **Kremkau**,
  **Neuendorf am Damm**, **Berkau**, **Eichenhof**, **Butterhorst**, **Algenstedt** und **Kalbe (Milde)** - ebenfalls
  **kein** „Rundling“ und **kein** „Cheinitz“. Ein Kontrollscan des **gesamten** Ortsbereichs Kalbe (Milde) in zwei
  0,10°-Schritten (11.32–11.50 / 52.58–52.70) findet damit **weder** die Straße **noch** die Hausnummer **44**.
  Quelle 2 (Betreiber/Register, im Dossier bereits zitiert): der **GLEIF-Datensatz** zu LEI
  **529900TGM8IAAPLG8W46** (live abgerufen 27.10.2026) nennt Rechtsträger **Rilei Automobile GmbH**, `registeredAs`
  **HRB 737**, `legalAddress` **Zum Rundling 44, Cheinitz, 39624** und `headquartersAddress` **Cheinitzer Rundling 44,
  Kalbe (Milde), 39624** - der Datensatz weist aber selbst `entity.status: ACTIVE` bei `registration.status: LAPSED`
  und `lastUpdateDate: 19.07.2021` aus; das **Golden-Copy-Publishdatum** ist kein Betriebsprüfdatum. Der im Dossier
  zitierte **NorthData-Registerabdruck** (HRB 737 Stendal, ehemaliger Name RILEI Autoverwertung GmbH Badel) und das
  **Verzeichnisprofil auto-werkstatt.de** („Rilei Automobile GmbH, Cheinitzer Rundling 44, 39624 Kalbe, 039001 90922,
  Ansprechpartnerin Nicole Wernecke“) sind **kein** Standortbeleg; das Verzeichnis ist ausdrücklich **ungeprüft** und
  nennt als Spezialisierung nur **Autoreparatur**. **Warum keine Koordinate:** OSM kennt die Straße nicht; eine
  Position auf die Ortsteil-Fläche „Cheinitz“ oder auf ein Nachbarobjekt wäre **geraten**; der zurückgenommene
  Produktionspunkt **53.2934/9.5506** (Kalbe in **Niedersachsen**, Kreis Rotenburg - also ein **anderer** Ort) bleibt
  mit `clear_fields: geo` zurückgenommen. **Was zur Auflösung fehlt:** ein OSM-Objekt mit `addr:street`=Cheinitzer
  Rundling bzw. `addr:housenumber`=44 und `addr:postcode`=39624 (oder ein benannter Betriebspunkt „Rilei“) - vor allem
  aber ein **aktueller Gewerbe-/Betriebsbeleg**, der die heutige Tätigkeit als Autoverwertung an dieser Anschrift
  bestätigt (LEI-Registrierung ist LAPSED, die Domain rilei.de steht zum Verkauf, die Autoverwertung ist nicht
  belegt); `street`, `postcode`, `phone`, `email`, `opening_hours`, `website`, `website_status`, `description` und
  `status: pruefung` bleiben unverändert; es wird weder Fortbestand noch Schließung behauptet. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=11.3200,52.6000,11.4400,52.7000 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=11.3800,52.5800,11.5000,52.7000 ;
  https://api.gleif.org/api/v1/lei-records/529900TGM8IAAPLG8W46 ;
  https://www.northdata.de/Rilei+Automobile+GmbH,+Kalbe+(Milde)/Amtsgericht+Stendal+HRB+737 ;
  https://www.auto-werkstatt.de/kalbe/rilei-automobile-gmbh-aULLNy (Abruf 04.11.2026).]
