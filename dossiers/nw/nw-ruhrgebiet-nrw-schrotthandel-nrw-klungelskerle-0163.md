---
slug: nw-ruhrgebiet-nrw-schrotthandel-nrw-klungelskerle-0163
clear_fields: 'dropoff_json,geo'
name: Schrotthandel NRW / Klüngelskerle (0163)
trader_type: schrotthaendler
state: NW
city: Ruhrgebiet/NRW
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://schrotthandel.nrw
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Mobile / NRW-weite Händler (ohne geprüften stationären Platz)
provenance_ankauf_raw: mobil/ja
provenance_origin: table
---

# Schrotthandel NRW / Klüngelskerle (0163)

## Überblick

Mobiler NRW-Schrottankauf wird jetzt direkt auf der erreichbaren Zieldomain beworben. Der aktuelle Auftritt nennt 0163 4694319 und info@schrotthandel.nrw; das Impressum nennt „Schrotthandel.nrw / EL-Lahib“ und Vorstadtstraße 65, Bochum, jedoch weder vollständigen Vornamen noch Rechtsform. Die Kontaktseite zeigt 4694319, enthält im HTML aber zusätzlich einen unbeschrifteten `tel:`-Link zu 0163 4630051; auch der frühere 11880-Lead nennt 4630051 und Lohrheidestr. 156. Betreiberbezug und Einsatz-/Betriebsadresse sind ungeklärt. Die separate Domain nrw-schrott.de mit Tarek El-Lahib und Robertstraße 70 wird nicht ohne belegte Betreiberkette zusammengeführt. `website_status: aktiv` beschreibt nur die technische Erreichbarkeit; wegen ungeklärter Betreiber-/Seedidentität bleibt `status: pruefung`. Keine Kontaktdaten oder Adresse in strukturierte Felder übernommen; `clear_fields: 'dropoff_json,geo'` nimmt die nicht belegte frühere Annahmezusage und den Punkt zurück. Für den Zielbetrieb: **Ankauf** ohne verifizierte Kurse; **Verkauf** ohne Preisliste; **Gebühren** ohne überprüfbare Tarifangabe.

## Timeline

### Owner-Audit 26.10.2026: Website wieder direkt erreichbar

- [Owner-Audit 26.10.2026: Root, Impressum, Kontakt und Schrottankauf von schrotthandel.nrw lieferten jetzt direkt lesbare Betreiberseiten; die frühere Einstufung `website_status: blockiert` war als technische Erreichbarkeitsangabe überholt und wird zu `aktiv` korrigiert. Der Auftritt nennt mobil NRW-weite Leistungen, 0163 4694319, info@schrotthandel.nrw und im Impressum „Schrotthandel.nrw / EL-Lahib“, Vorstadtstraße 65, 44866 Bochum; ein voller Vorname bzw. eine Rechtsform fehlen. Zusätzlich zur sichtbaren 0163 4694319 enthält die aktuelle Kontaktseite einen unbeschrifteten HTML-`tel:`-Link auf 0163 4630051; die Nummer steht auch im bisherigen 11880-Lead, der außerdem Lohrheidestr. 156 nennt. Das ist ein ungelöster Kontakt-/Adresskonflikt, keine Bestätigung der 11880-Adresse. Die separate nrw-schrott.de-Spur mit Tarek El-Lahib/Robertstraße 70 wird mangels Betreiberkette nicht zusammengeführt. `status: pruefung`, leere Adresse/Kontakte und `clear_fields: 'geo'` bleiben angemessen: die Website belegt kein stationäres Annahmegrundstück und keinen Punkt; keine Nummer übernommen. Keine Produktionsdaten abgefragt oder geändert. Quellen: https://schrotthandel.nrw/ ; https://schrotthandel.nrw/impressum ; https://schrotthandel.nrw/contact ; https://schrotthandel.nrw/schrottankauf ; Abgrenzung: https://nrw-schrott.de/impressum.html (Abruf 26.10.2026).]

### Owner-Korrektur 26.10.2026: alte Annahmezusage zurücknehmen

- [Owner-Korrektur 26.10.2026: Der read-only Produktionsdatensatz enthielt neben dem Punkt `51.58945/7.2035` noch `dropoff_json={"allowed":true}`. Die aktuelle Betreiberseite bewirbt einen mobilen NRW-weiten Abholservice und nennt Partner-Wertstoffhöfe als Ziel der abgeholten Materialien; sie belegt keinen eigenen öffentlichen Annahmestandort. Zusammen mit der weiterhin ungeklärten Betreiber-/Seedidentität trägt das die gespeicherte allgemeine Selbstanlieferungszusage nicht. Der Owner ergänzt `clear_fields: 'dropoff_json,geo'`; der nächste reguläre Seed setzt die Zusage auf unbekannt `{}` und entfernt den falschen GEO-Punkt, ohne `allowed:false`, Schließung oder eine Partnerannahme dem Seed-Händler zuzuschreiben. `website_status: aktiv` bleibt allein technische Erreichbarkeit, `status: pruefung` bleibt wegen der offenen Identität. Keine Produktionsdaten geändert. Quellen: Read-only `public.db` (26.10.2026); https://schrotthandel.nrw/ ; https://schrotthandel.nrw/schrottankauf ; https://schrotthandel.nrw/contact ; `docs/trader-seed.md`.]

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 51.58945, 7.2035. Nominatim-Reverse löst den Punkt in der Tiroler Straße in Recklinghausen auf. Der Seed beschreibt einen unbestätigten mobilen NRW-Service, keine dortige Betriebs- oder Annahmestelle; der Straßenpunkt ist irreführend. `clear_fields: 'geo'` entzieht ihn beim nächsten regulären Seed. Keine Ersatzkoordinate für ein regionales Einsatzgebiet geraten. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.58945&lon=7.2035&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026: neue konkrete Anschrift als Lead

- [Recherche 25.10.2026: 11880-Detailseite „Schrotthandel.nrw“ direkt gelesen: Lohrheidestr. 156, 44866 Bochum-Wattenscheid, 0163 4630051, info@schrotthandel.nrw, Mo–Sa 07:00–19:00; Eintragsdaten 21.09.2026, Portalaktualisierung 09.10.2026. Die Seite verlinkt die Zieldomain schrotthandel.nrw. Das ist ein neuer, passender Lead für eine der bisherigen konkurrierenden 0163-Nummern (die 0163 4694319 bleibt unbestätigt), aber kein Betreiber-Impressum oder unabhängiger Zweitbeleg. Die Direktabrufe vom 18.10. hatten Root 403 und /impressum, /schrottankauf, /datenschutz 404; der Portalstatus „lädt normal“ ersetzt keinen Primärquellenabruf. **Ankauf:** keine Zahlenkurse; **Verkauf:** keine Liste; **Gebühren:** keine belastbaren Tarife. Keine Felder geändert. Lohrheidestr. 156 ist erst nach Betreiber-/Annahmebestätigung geocodierbar; nicht mit der separaten Robertstraße-70-Spur zusammenführen. Quelle (Lead): https://www.11880.com/branchenbuch/bochum/120674719B112437141/schrotthandel-nrw.html ; Direktgegenprobe: https://schrotthandel.nrw/ .]

### Importiert (Seed-Stand 2026-09-30)

- Altmetall (Cu/Messing/Alu/Edelstahl), Autoteile

### Recherche 01.10.2026

- [Recherche 01.10.2026: Homepage-Abruf 403, /impressum-Abruf 404 → website_status blockiert gesetzt; Impressum-Inhalt (EL-Lahib, Vorstadtstr. 65, 44866 Bochum) nur als Such-Snippet greifbar (Lead, keine zweite Quelle); Vorgänger-Domain nrw-schrott.de nennt abweichend Robertstr. 70, 44809 Bochum → Identitäts-/Adressklärung offen, Klärfall; Quelle(n): eigene Fetch-Probe 01.10.2026, Websuche (Leads)]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Retry /schrottabholung-koeln → 404 (Suchindex listet die Seite mit Inhalt + Tel. 0163/4694319, Zeiten Mo-Fr 7-21/Sa 9-21 - Bot-Sperre wahrscheinlich, kein Browser-Rendering moeglich). website_status bleibt blockiert; kein Fill; Klaerfall Adressen (Vorstadtstr. 65 vs. Robertstr. 70) weiter offen; Quelle(n): eigene Fetch-Probe 02.10.2026, Websuche (Leads)]

### Recherche 03.10.2026

- [Recherche 03.10.2026: website auf Domain-Root normalisiert. Betreiberseiten /schrottankauf und /schrottabholung-solingen sind im Suchindex mit Schrottankauf/Abholung in NRW, Kontaktangaben und aktuellen Copyright-Jahren auffindbar, konnten einzeln direkt jedoch nicht abgerufen werden (Root 403, relevante Unterseiten 404); daher keine Kontakt-, Öffnungszeit- oder Adressdaten daraus übernommen und website_status bleibt blockiert. Die direkt abrufbare ältere Domain nrw-schrott.de nennt Tarek El-Lahib, Robertstraße 70, 44809 Bochum, Kontakt 01525-2376589 und Inhalte mit RSS-Daten aus 2015; eine belastbare Betreiber-/Identitätsverknüpfung zur aktuellen Domain mit abweichenden indexierten Kontaktdaten fehlt. Frühere Such-Snippets zu Vorstadtstr. 65 und die abweichende Alt-Domain-Adresse bleiben Leads; keine Merge- oder Adressentscheidung. Quelle(n): https://schrotthandel.nrw/, https://schrotthandel.nrw/schrottankauf, https://schrotthandel.nrw/schrottabholung-solingen, https://schrotthandel.nrw/impressum, https://nrw-schrott.de/impressum.html, https://nrw-schrott.de/rss.xml]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Erneute Direktprobe bestätigt: https://schrotthandel.nrw/ weiterhin 403, /schrottankauf und /impressum weiterhin 404. Der Suchindex führt zur Ankaufseite nun Telefon 0163 4630051, während frühere Snippets 0163 4694319 nennen; beide Nummern bleiben unbestätigte Leads und werden nicht ins Frontmatter übernommen. Die ältere, direkt erreichbare Domain https://nrw-schrott.de/impressum.html nennt Tarek El-Lahib, Robertstraße 70, Bochum, aber keine belegte Verbindung zum aktuellen Domainauftritt. ANKAUF: nur Werbeaussage/Indexhinweis, keine numerische Preisliste; VERKAUF: keine Liste gefunden; GEBÜHREN: keine überprüfbare Tarifangabe. Offen: aktuelle Betreiberidentität, korrekter Kontakt, Rechts-/Impressumsseite und ob tatsächlich Schrott angenommen wird. Quelle(n): https://schrotthandel.nrw/; https://schrotthandel.nrw/schrottankauf; https://schrotthandel.nrw/impressum; https://nrw-schrott.de/impressum.html; Suchindex-Abfrage als Lead: https://www.google.com/search?q=site%3Aschrotthandel.nrw+0163+Schrottankauf]

### Korrektur 18.10.2026

- [Korrektur 18.10.2026: Status aktiv → pruefung wegen ungeklärter Betreiber-/Seedidentität und lediglich indexgestützter Ziel-Leistungen, nicht allein wegen HTTP 403. Im heutigen Review Root mit/ohne www jeweils 403, /impressum, /schrottankauf und /datenschutz jeweils 404; kein Betreiberinhalt dieser Zieldomain gelesen. Indexangaben 0163 4630051 versus 0163 4694319 bleiben Konflikt-Leads, keine bestätigten Kontakte. Startseite/Impressum der separaten Domain nrw-schrott.de gelesen: Tarek El-Lahib, Robertstraße 70, Bochum, 01525-2376589; Kopf mit zusätzlichem WhatsApp-Kontakt. Dort beworbene kostenlose Abholung ist termin-/standort-/mengenabhängig; 100–150 kg nur FAQ-Beispiel, keine allgemeine feste Mindestmenge. Ankauf nach Menge/Qualität/Aufwand, Wiederverkauf von Fahrzeugteilen und möglicher Demontagemehraufwand, im gelesenen Text keine numerischen Tarife. Keine dieser Angaben in strukturierte Zielfelder übertragen. Gemeinsamer Nachname beweist keine Betreiberkette; separate Domain nicht als bewiesener Vorgänger oder anderer Betreiber behandeln. Historische RSS-Daten beweisen nicht heutiges Inhaltsalter; der uncommittete 12.10.-Entwurf ist kein belegter damaliger Abruf. Für den Zielbetrieb Preis-/Gebührenbeträge offen. Quellen: https://schrotthandel.nrw/ ; https://www.schrotthandel.nrw/ ; https://schrotthandel.nrw/impressum ; https://schrotthandel.nrw/schrottankauf ; https://schrotthandel.nrw/datenschutz ; https://nrw-schrott.de/impressum.html ; https://nrw-schrott.de/]
