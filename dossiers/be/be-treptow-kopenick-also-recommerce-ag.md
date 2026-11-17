---
slug: be-treptow-kopenick-also-recommerce-ag
name: ALSO Recommerce AG
trader_type: schrotthaendler
state: BE
city: Treptow-Köpenick
street: Wegedornstr. 36
postcode: '12524'
phone: '+49 30 767670240'
email: info.arc@also.com
opening_hours: ''
website: https://www.webinstore.de/
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Berlin (BE) — Recherche
provenance_ankauf_raw: kein Ankauf (nur Bestand)
provenance_origin: table
---

# ALSO Recommerce AG

## Überblick

Die Existenz der **ALSO Recommerce AG** ist in NorthData als AG Charlottenburg HRB 125589 B belegt; daraus folgen weder die Seed-Adresse Wegedornstr. 36 noch eine Schrott-/Altmetallannahme. Die aktuelle offizielle ALSO-Deutschland-Seite beschreibt die ALSO Deutschland GmbH als ITK-Technologieanbieter und Refurbishment-Anbieter, identifiziert aber nicht die ALSO Recommerce AG oder eine Berliner Filiale. Die Gesellschaften werden nicht gleichgesetzt; Adresse und `status: unbekannt` bleiben ungeklärt.

**Preise:** Ankauf: kein Schrottankauf belegt; Verkauf: keine schrottbezogene Liste; Gebühren: keine Angaben.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „—“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Importiert (Seed-Stand 2026-09-30)

- Elektronik-Refurbishment, kein Schrottankauf
- Adresse: Wegedornstr. 36, 12489 Berlin
- Bezirk: Treptow-Köpenick Adressbeleg: seed/be.json + recherche/be.md (Verzeichnisbeleg)

### Recherche 22.10.2026

- [Recherche 22.10.2026: NorthData-Profil zur ALSO Recommerce AG (HRB 125589 B) und aktuelle ALSO-Deutschland-Startseite erneut direkt gelesen. Die Konzernseite beschreibt ITK-Handel und Refurbishment, nennt aber weder diese Berliner AG noch Wegedornstr. 36 oder Schrottannahme. Das belegt keine Konzernzugehörigkeit/Filiale und ist kein Negativbeweis für den Registereintrag. Seed-Adresse und Status nicht verändert. ANKAUF/VERKAUF/GEBÜHREN: keine schrottspezifischen Bedingungen oder Zahlen. Quellen: https://www.northdata.de/ALSO+Recommerce+AG,+Berlin/HRB+125589 ; https://www.also.com/ec/cms5/de_1010/1010/index.jsp]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Registerbeleg JA, Schrottbezug NEIN — northdata.de: „ALSO Recommerce AG, Berlin, AG Charlottenburg HRB 125589 B“ (Existenz als AG registerbelegt). Kein Schrottankauf belegbar: Gelbe Seiten (Suche recommerce/berlin), 11880.com (Suche recommerce/berlin: nur reBuy reCommerce GmbH — anderes Unternehmen) und ALSO Deutschland (also.de/also.com: kein Recommerce-Treffer) kennen keine „ALSO Recommerce“ mit Schrottbezug; keine Betreiber-Website, keine Rufnummer auffindbar. Seed-Aussage (Elektronik-Refurbishment, kein Schrottankauf, Wegedornstr. 36, 12489 Berlin) unwidersprochen. NICHTS übernommen (Adresse bleibt Seed-Stand), status bleibt unbekannt (kein Ankauf → Kandidat für De-Listung, aber nichts gelöscht). Fix: phone („—“ verboten) → leer, website_status → unbekannt (Enum-Validierung). Beleglage offen: Register ja, Branche/Adresse/Betreiber nein — Folgewelle: HR-Auszug/Adressabgleich. Quelle(n): northdata.de.]

### Recherche 17.11.2026

- [Recherche 17.11.2026: Die Gesellschaft ist jetzt über eine amtliche Anerkennung am Seed-Standort belegt: Das amtliche Fachbetrieberegister führt in der Route Entsorgungsfachbetriebe den Eintrag "ALSO Recommerce AG, Wegedornstraße 36, 12524 Berlin", Vorgang ZzHT00300293 / 1, anerkannte Tätigkeit "Lagern". Damit sind Rechtsträger und Straßenname identisch mit dem Seed-Datensatz, und der Dossier-Eintrag ist ein amtlich anerkannter Entsorgungsfachbetrieb. Die anerkannte Tätigkeit ist ausdrücklich nur das Lagern; ein "Handeln" und damit eine öffentliche Ankauf- oder Annahmeberechtigung für Schrott ist nicht ausgewiesen, was zur bisherigen Einstufung "kein Ankauf (nur Bestand)" passt. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Recommerce (Abruf 17.11.2026).]
- [Recherche 17.11.2026: Die Postleitzahl des Seed-Datensatzes ist falsch: Das amtliche Fachbetrieberegister nennt Wegedornstraße 36, 12524 Berlin, und OpenStreetMap löst die Hausnummer 36 in 12524 Berlin (Ortsteil Altglienicke, Bezirk Treptow-Köpenick) auf; dort trägt das Objekt way/220317432 den Namen "Webinstore" und den Betreiber "Webinstore AG", also den früheren Namen dieser Gesellschaft. `postcode` wurde deshalb von 12489 auf 12524 berichtigt; die Postleitzahl 12489 gehört zu einem anderen Berliner Ortsteil und ist durch keine Quelle für diesen Betrieb gedeckt. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Recommerce ; https://api.openstreetmap.org/api/0.6/way/220317432/full.json ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=Wegedornstra%C3%9Fe+36%2C+Berlin&addressdetails=1&extratags=1 (Abruf 17.11.2026).]
- [Recherche 17.11.2026: Der Betreiberauftritt ist gefunden und bestätigt die Namenskette: Die Domain webinstore.de trägt seit der Umfirmierung die Wortmarke "ALSO Recommerce | ALSO Group" und nennt als Gründungsjahr 1998 sowie die Eingliederung in die Services-Sparte der ALSO Holding AG im Jahr 2013; North Data führt für dasselbe Registerblatt HRB 125589 B die Kette Webinstore GmbH (2005) über Webinstore AG (Formwechsel 22.03.2010, Firmensitz Berlin, Kapital 500.000 EUR) zu "Name: ALSO Recommerce AG" (28.01.2026) sowie die Beteiligung der ALSO Enterprise Services GmbH (Gesellschafterin CORA - IT GmbH) seit 01.06.2011. `website` wurde auf die Betreiberdomain gesetzt; das Feld `website_status` bleibt unbekannt, weil die Domain technisch erreichbar, aber als Auftritt der Gesellschaft nicht eindeutig ist. Quelle(n): https://www.webinstore.de/ ; https://www.northdata.de/ALSO+Recommerce+AG,+Berlin/HRB+125589 (Abruf 17.11.2026).]
- [Recherche 17.11.2026: Das Leistungsprofil des Betreibers schließt den Schrottankauf aus: Die Website beschreibt IT-Asset-Disposition, zertifizierte Datenlöschung, Refurbishment, Remarketing und die Entsorgung gebrauchter IT-Hardware mit einer Refurbishing-Rate von 85 Prozent, und das Trade-In-Angebot erlaubt die Erfassung von Menge, Art und Zustand der zu verwertenden IT-Hardware mit einem freibleibenden Ankaufsangebot in Echtzeit. ANKAUF: dieses Angebot betrifft gebrauchte IT-Hardware und ist kein Schrottankauf, für Schrott oder Metall ist kein Ankaufskurs und keine Preisliste belegt. VERKAUF: Refurbishment-Geräte werden angeboten, eine Preisliste mit Zahlen ist auf der Startseite nicht ausgewiesen. GEBÜHREN: keine Tarife veröffentlicht. Quelle(n): https://www.webinstore.de/ (Abruf 17.11.2026).]
- [Recherche 17.11.2026: Ein Namensvetter bleibt ausgeschlossen: Die Namenssuche im amtlichen Fachbetrieberegister liefert für "Recommerce" in der Route Altfahrzeugverwertung 0 Treffer, und die ebenfalls in Berlin ansässige reBuy reCommerce GmbH wird nicht mit der ALSO Recommerce AG gleichgesetzt; auch die ebenfalls in Berlin eingetragene Webinstore AG ist nach der Umfirmierung nur die historische Firmierung desselben Blattes. Die Route Entsorgungsfachbetriebe enthält genau einen "Recommerce"-Treffer, den der ALSO Recommerce AG. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Recommerce ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Recommerce (Abruf 17.11.2026).]
- [Recherche 17.11.2026: Die Betreiberkontaktdaten stehen im serverseitig gerenderten Fußbereich jeder Seite der Betreiberdomain: "ALSO Recommerce AG, Wegedornstraße 36, DE-12524 Berlin, +49 30 767670240, info.arc@also.com". `phone` und `email` wurden daraus übernommen und die Clear-Direktive `clear_fields: 'phone'` zurückgenommen, weil sie 2026 nur den Import-Platzhalter "—" zurücknehmen sollte und jetzt eine belegte Nummer vorliegt; die Postleitzahl 12524 ist damit durch eine dritte, betreiberseitige Quelle bestätigt. `website_status` wurde auf aktiv gesetzt, weil die Domain technisch erreichbar ist und die Wortmarke "ALSO Recommerce | ALSO Group" trägt. `status` wurde von unbekannt auf aktiv gesetzt, weil Registerbestand, amtliche EfbV-Anerkennung und Betreiberauftritt den Eintrag als existierenden Betrieb ausweisen. Quelle(n): https://www.webinstore.de/ ; https://www.webinstore.de/ec/cms5/de_4112/4112/legal/impressum/index.jsp (Abruf 17.11.2026).]
