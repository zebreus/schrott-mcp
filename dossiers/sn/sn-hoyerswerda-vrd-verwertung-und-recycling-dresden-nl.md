---
slug: sn-hoyerswerda-vrd-verwertung-und-recycling-dresden-nl
clear_fields: 'geo'
name: VRD Verwertung und Recycling Dresden GmbH, NL Hoyerswerda (VEZ)
trader_type: sonstige
state: SN
city: Hoyerswerda
street: Dresdner Straße 151/152
postcode: '02977'
phone: ''
email: ''
opening_hours: ''
website: http://www.vrd-gmbh.de
website_status: aktiv
status: pruefung
description: Anlage zur zeitweiligen Lagerung gefährlicher Abfälle; öffentlicher Kundenzugang und Schrottankauf nicht belegt.
dropoff_json: ''
pickup_json: ''
provenance_section: 6) Oberlausitz (Bautzen, Görlitz, Kamenz, Hoyerswerda, Weißwasser)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# VRD Verwertung und Recycling Dresden GmbH, NL Hoyerswerda (VEZ)

## Überblick

Ein aktuellerer amtlicher Überwachungsbericht des Landkreises Bautzen (Routinekontrolle 14.04.2025) benennt VRD Verwertung u. Recycling Dresden GmbH als Betreiber der Hoyerswerdaer Anlage zur zeitweiligen Lagerung gefährlicher Abfälle und gibt die Betriebsstätte mit Dresdner Straße 151/152, 02977 Hoyerswerda an; die Kontrolle stellte keine Beanstandungen oder Umweltauswirkungen fest. Straße und PLZ sind damit amtlich belegt. Das Dokument bestätigt aber weder öffentlichen Kundenzugang noch Schrottankauf oder Verkauf. Die Betreiberwebsite beschreibt weiterhin vor allem Leistungen des Kompostwerks Brischko; diese nicht ohne Standortbezug übertragen. `status: pruefung` bleibt.

**Preise:** **Ankauf** – keine Hoyerswerdaer Ankaufskurse belegt. **Verkauf** – keine Standort-Preisliste. **Gebühren** – keine Hoyerswerdaer Annahme-/Containergebühren veröffentlicht. **GEO:** genaue Anlagenadresse nun belegt; Koordinaten erst nach Entscheidung, ob der Datensatz eine regulierte Anlage oder einen öffentlich zugänglichen Händler-/Annahmepunkt darstellen soll.

## Timeline

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 51.4379, 14.2355. Nominatim-Reverse löst den Punkt an der Steinbrückstraße 21 in der Hoyerswerdaer Altstadt auf, nicht an der durch den Landkreis-Bericht belegten Betriebsstätte Dresdner Straße 151/152. `clear_fields: 'geo'` entzieht den adressfremden Altpunkt beim nächsten regulären Seed. Die Anlage ist zudem kein bestätigter öffentlicher Annahmepunkt; keine Ersatzkoordinate. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.4379&lon=14.2355&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026: Adressnachweis

- [Recherche 25.10.2026: Landkreis Bautzen-Übersichtsseite und den dort verlinkten Routine-Überwachungsbericht direkt geprüft. Der Bericht vom 14.04.2025 nennt als Betreiber VRD Verwertung u. Recycling Dresden GmbH und als Standort Dresdner Straße 151/152, 02977 Hoyerswerda; Anlage: zeitweilige Lagerung gefährlicher Abfälle. Kontrolle: keine Beanstandungen, keine festgestellten Umweltauswirkungen. Damit sind `street` und `postcode` im Frontmatter ergänzt; dies belegt weder Kundenannahme noch Kauf von Metallschrott, und `status: pruefung` bleibt. ANKAUF: keine Kurse; VERKAUF: keine Liste; GEBÜHREN: kein Tarif. GEO-Bedarf neu konkretisiert, aber zunächst Punktfunktion (Anlage vs. öffentliches Annahmelokal) mit Owner klären. Quellen: https://www.landkreis-bautzen.de/ueberwachung-nach-industrie-emissionsrichtlinie-5355.php ; https://www.landkreis-bautzen.de/download/Bauaufsichtsamt/14.04.2025_Ueberwachungsbericht_zur_Routinekontrolle_VRD_Hoyerswerda.pdf .]

### Recherche 12.10.2026

- [Recherche 12.10.2026: Die aktuelle Betreiberseite beschreibt die VRD und ihre Leistungen am Kompostwerk Brischko: Annahme biologisch abbaubarer Abfälle, Klärschlamm, Speisereste, Altholz AI–AIII und sauberem Oberboden sowie Kompost-/Bodenprodukte, Lieferung und Containerleistungen. Das belegt nicht, dass diese Annahme oder Produkte am Dossierstandort Hoyerswerda gelten. Ein Suchtreffer-Auszug zum Landkreis-Bautzen-Kurzbericht 2023 nennt VRD Verwertung u. Recycling Dresden GmbH, Betriebsstätte Dresdner Straße 151/152, 02977 Hoyerswerda; das PDF ließ sich beim Abruf nicht als Volltext auslesen. Wegen des Alters und der eingeschränkten Einsicht sind heutige Anschrift/Zufahrt nicht bestätigt. Keine Adresse oder GEO geändert. ANKAUF: keine €/Einheit-Preisliste für Hoyerswerda gefunden. VERKAUF: keine veröffentlichte Preisangabe für Kompost/Boden. GEBÜHREN: keine veröffentlichte Annahme-/Containergebühr für diesen Standort. Quellen (Abruf 12.10.2026): http://www.vrd-gmbh.de/ ; http://www.vrd-gmbh.de/leistungen/ ; https://www.landkreis-bautzen.de/download/Bauaufsichtsamt/Kurzbericht_zur_UEW14032023.pdf (PDF-Suchtreffer, nicht als Volltext auslesbar).]

### Recherche 01.10.2026 (Feedback 2679)

- [Recherche 01.10.2026: Feedback 2679 geprüft — Beobachtung ZUTREFFEND (Betreiber-Seite vrd-gmbh.de nennt nur Kompostwerk Brischko, keinen Standort Hoyerswerda), aber KEIN Dossier-Fehler: street/postcode bereits leer, status pruefung, website belegt. Kein Frontmatter-Change (unberechtigt im Sinne kein Handlungsbedarf). Zweiseitig geprüft: Gegenbeleg für Hoyerswerda-Bezug existiert (Landkreis Bautzen, Überwachungsbericht Routinekontrolle VRD Hoyerswerda 14.04.2025), aber keine Adresse → bestehender Klärfall (VEZ-Adresse) bleibt offen, kein Phantom erzeugt. Quelle(n): http://www.vrd-gmbh.de/, Folgewelle 01.10.2026 (Landkreis-Dokument)]

### Importiert (Seed-Stand 2026-09-30)

- Lagern (Register)
- —

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Website live verifiziert (vrd-gmbh.de, abgerufen 01.10.2026): VRD Verwertung und Recycling Dresden GmbH (Kompostwerk Brischko, Bioabfall/Klärschlamm/Kompost); Kontakt Brischko Nr. 42, 02997 Wittichenau, Tel. 035725/71000, vrd-gmbh@t-online.de; Impressum: Andre Höhne, Zur Wetterwarte 17, 01109 Dresden (HR-Felder leer). Hoyerswerda-NL-Adresse auf Betreiber-Seite NICHT genannt (City-Mismatch: kein street/postcode-Fill). Existenz VRD-Anlage Hoyerswerda belegt durch kommunales Dokument (Landkreis Bautzen, Überwachungsbericht Routinekontrolle VRD Hoyerswerda, 14.04.2025, Scan-PDF) → website/website_status aktiv gefüllt (Domain-Root, live verifiziert); street/postcode/phone/email leer. Klärfall: Hoyerswerda-Standortadresse (VEZ-Bezug) requires Betreiber-/Register-/Kommunalbeleg; Quelle(n): http://www.vrd-gmbh.de/kontakt-1/, http://www.vrd-gmbh.de/impressum/, landkreis-bautzen.de (Überwachungsbericht 14.04.2025)]
