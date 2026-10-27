---
slug: nw-munster-hr
name: HR
trader_type: schrotthaendler
state: NW
city: Münster
street: Drachterstraße 29a
postcode: '48153'
lat: '51.9240697'
lon: '7.6220598'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: 'HR Transport & Handel GmbH, Amtsgericht Münster HRB 19010, Anschrift Drachterstraße 29a, 48153 Münster-Hiltrup; Unternehmensgegenstand laut Register: Transport und Handel mit Metallen und anderen Schüttgütern. Die Betreiberdomain hr-transport-handel.de ist tot (durchgängig HTTP 404), eine lesbare Betreiber-Primärquelle existiert derzeit nicht; Annahme von Schrott, Preise und Gebühren sind nicht belegt.'
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# HR

## Überblick

Northdata nennt HR Transport & Handel GmbH, AG Münster HRB 19010, Drachterstraße 29a, 48153 Münster und den Gesellschaftszweck Transport/Handel mit Metallen und Schüttgütern; der Handelsregister-Auszug (online-handelsregister.de) bestätigt Rechtsträger, HRB 19010, Anschrift und Status „aktiv“. Die sichtbaren Registerveröffentlichungen reichen bis 04.06.2025; die Betreiberwebsite lieferte beim Abruf am 23.10.2026 und erneut am 27.10.2026 HTTP 404 (durchgängig, auch /impressum, /kontakt, /ueber-uns). Das bestätigt weder aktuelle Annahme noch Schließung. Die Anschrift ist belegt und die Koordinate auf den adressgenauen OSM-Gebäudepunkt korrigiert (der alte Punkt lag rund 38 km entfernt im Kreis Soest); Betreiberkontakt und Annahme bleiben ungeklärt, `status: pruefung` bleibt angemessen.

## Timeline

### Recherche 27.10.2026: Registerabgleich und GEO-Korrektur Münster

- [Recherche 27.10.2026: Handelsregister-Auszug erneut live gelesen (online-handelsregister.de, Spiegelservice): **HR Transport & Handel GmbH**, HRB 19010, Amtsgericht Münster, Status **aktiv**, Anschrift **Drachterstraße 29a, 48153 Münster**, Eintragung 25.11.2020, **2 Registerbekanntmachungen**, letzte Änderung 10.12.2021, Gesellschafterliste mit Fassung 11.07.2025, Jahresabschluss für das Geschäftsjahr 01.09.2020–31.12.2020 offengelegt am 11.02.2025. Der bereits dokumentierte NorthData-Befund (Gegenstand: Transport und Handel mit Metallen und anderen Schüttgütern) bleibt die inhaltliche Registerangabe; die Adresse in `street`/`postcode` ist damit bestätigt.]
- [Recherche 27.10.2026: Amtliches Fachbetrieberegister (Fachbetrieberegister der Gemeinsamen Stelle Altfahrzeuge der Länder nach § 7 Abs. 2a AltfahrzeugV und Register der Entsorgungsfachbetriebe nach § 56 KrWG, Auszug mit Stand 11.08.2026, gespiegelt über SchrottRegister) führt für Münster 16 anerkannte Betriebe (PLZ 48155, 48157, 48163, 48165) — darunter SRM Schrott & Metallrecycling Münster GmbH (Kesslerweg 37), Zimmermann Entsorgungs GmbH (Hessenbusch 183), Reiling PV-Recycling GmbH & Co. KG (Hessenbusch 180). Ein **„HR Transport & Handel“ fehlt**, jedoch deckt der Münster-Auszug die PLZ **48153** (Drachterstraße) nicht ab; der Negativbefund trägt deshalb nichts zur Existenz- oder Schließungsfrage bei.]
- [Recherche 27.10.2026: Betreiberdomain weiterhin tot: `hr-transport-handel.de` leitet per 301 auf `https://www.hr-transport-handel.de` und liefert dort durchgängig **HTTP 404** („Not found: https://www.hr-transport-handel.de“) — geprüft für `/`, `/impressum`, `/kontakt`, `/ueber-uns`, `/robots.txt`, `/sitemap.xml` und `/schrott`. Die Domain löst auf (217.160.0.65), hat aber keinen Inhalt mehr; eine Wayback-Snapshots gibt es für die Domain nicht. Suchindex-Anzeigen (Brave-Wissenspanel: Drachter Str. 29 a, 48153 Münster, info@hr-transport-handel.de, 0151 / 55460407) stammen aus derselben Operator-Website und sind **kein zweiter Beleg**; deshalb **kein** Telefon-, E-Mail- oder Website-Fill. `website`/`website_status` bleiben unverändert.]
- [Recherche 27.10.2026: GEO-Korrektur. Die read-only-Produktionszeile enthielt den Punkt **51.6992941176471/8.03888235294118**; die OSM/Nominatim-Rückabfrage löst ihn auf **„Frölichweg, Lippetal, 59510 (Kreis Soest, NRW)“** auf — das liegt **rund 38 km Luftlinie** von der registerbelegten Anschrift entfernt und in einem anderen Kreis. Ersetzt durch den adressgenauen OSM-Gebäudepunkt zu „Drachterstraße 29a, 48153 Münster-Hiltrup“ (OSM-Objekt 303120014): **51.9240697 / 7.6220598**. Der Punkt wurde unabhängig über zwei Geokodierdienste bestimmt (Nominatim-Vorwärtssuche und Photon/Komoot) und liegt **0 m aequivalent** auseinander. Punktart: **Gebäude-/Adresspunkt**, keine bestätigte Anlieferzufahrt; das bleibt als offene Restlücke dokumentiert. Quellen: https://www.online-handelsregister.de/handelsregisterauszug/nw/Muenster/HRB/19010/HR-Transport-Handel-GmbH ; https://schrottregister.pages.dev/ort-munster ; https://hr-transport-handel.de/ (404) ; http://archive.org/wayback/available?url=hr-transport-handel.de ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=Drachterstra%C3%9Fe+29a%2C+48153+M%C3%BCnster&addressdetails=1 ; https://photon.komoot.io/api/?q=Drachterstra%C3%9Fe+29a%2C+48153+M%C3%BCnster ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&zoom=18&lat=51.6992941176471&lon=8.03888235294118&addressdetails=1 ; read-only `public.db` (27.10.2026).]
- [Preise/Ankauf/Verkauf/Gebühren 27.10.2026: **Ankauf:** der Registergegenstand „Transport und Handel mit Metallen und anderen Schüttgütern“ ist **kein** konkreter Ankaufbeleg; es gibt keine Preisliste. **Verkauf:** keine Liste. **Gebühren:** unbekannt. Status bleibt `pruefung`, weil die Betreiberpräsenz (tote Domain, keine lesbare Primärquelle) ungeklärt ist. Quellen: https://www.northdata.de/HR%20Transport%20%26%20Handel%20GmbH,%20M%C3%BCnster/HRB%2019010 ; https://schrottregister.pages.dev/ort-munster .]

### Recherche 23.10.2026

- [Recherche 23.10.2026: Northdata-Direktansicht bestätigt HR Transport & Handel GmbH, AG Münster HRB 19010, Drachterstr. 29 a, 48153 Münster und den Gesellschaftszweck Transport/Handel mit Metallen und sonstigen Schüttgütern. Sichtbare Veröffentlichungen reichen bis zur Gesellschafterliste vom 04.06.2025; daraus folgt weder aktuelle operative Tätigkeit noch Annahmebetrieb. hr-transport-handel.de lieferte beim Direktabruf HTTP 404. Keine Kontakt-/Website-Fills oder GEO, Status pruefung. **ANKAUF:** Gesellschaftszweck ist kein konkreter Ankaufbeleg; keine Preisliste. **VERKAUF:** keine Liste. **GEBÜHREN:** unbekannt. Quelle: https://www.northdata.com/HR%20Transport%20%26%20Handel%20GmbH,%20M%C3%BCnster/HRB%2019010 ; Abrufversuch: https://hr-transport-handel.de/ .]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Teil-Fill (Straße/PLZ) aus starker Register-Einzelquelle: Northdata nennt HR Transport & Handel GmbH, AG Münster HRB 19010, Drachterstr. 29a, 48153 Münster, Gegenstand Transport/Handel mit Metallen und Schüttgütern; kongruent: handelsregister.ai/webvalid (HRB 19010, Drachterstraße 29a) + Creditreform-Firmeneintrag. Betreiber-Domain hr-transport-handel.de existiert(e) (Über-uns-Snippet: gleiche Adresse, info@hr-transport-handel.de), Abruf 02.10.2026 jedoch HTTP 404 — daher KEIN Website-/E-Mail-/Telefon-Fill, website_status bleibt leer. Adresse neu → Koordinaten neu zu geocodieren. Status bleibt pruefung (Betreiber-Präsenz ungeklärt). Quelle(n): Northdata-Register + handelsregister.ai/Creditreform (Register-Zweitbelege)]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Münster
